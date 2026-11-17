
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Package Model
//!
//! Path: `src/toolchain/packages/model.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the canonical, domain-neutral model of a Zamani package.
//!
//! A package is a logical distribution and compatibility unit. It is not
//! a machine, target, process, runtime instance, deployment, hardware
//! allocation, compiler IR, or physical resource.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Package identity and package naming.
//! - Package release-version representation.
//! - Package type identifiers.
//! - Structured package metadata.
//! - Portable package-content paths.
//! - Package content inventory descriptors.
//! - Package artifact descriptors.
//! - Extensible artifact compatibility declarations.
//! - Package capability declarations.
//! - Integrity descriptor representation.
//! - Signature and provenance reference representation.
//! - Structural validation of the package model.
//!
//! This module does not own:
//!
//! - Package ID allocation.
//! - Canonical source-language package parsing.
//! - Canonical project-manifest parsing.
//! - Dependency declarations or dependency resolution.
//! - Package archive serialization.
//! - Package extraction or installation.
//! - Filesystem or network access.
//! - Package-content hashing.
//! - Signature creation or verification.
//! - Publisher authentication or trust decisions.
//! - Compatibility evaluation.
//! - Target discovery or target selection.
//! - Compiler or backend execution.
//! - Hardware discovery or physical resource allocation.
//! - Runtime scheduling or execution.
//!
//! ## Canonical authorities
//!
//! Package IDs:
//! `crate::toolchain::ids::PackageId`
//!
//! Package names:
//! `crate::toolchain::dependencies::model::DependencyName`
//!
//! Version-scheme identifiers:
//! `crate::toolchain::dependencies::constraints::VersionSchemeId`
//!
//! Dependency declarations and coordinates:
//! `crate::toolchain::dependencies::model`
//!
//! Canonical project manifest:
//! `crate::zamani_project_config`
//!
//! Package declaration syntax:
//! `grammar/modules/packages.g4`
//!
//! Dependency declaration syntax:
//! `grammar/modules/dependencies.g4`
//!
//! ## Metadata
//!
//! Package metadata is static structured data. It is not executable code.
//!
//! Numeric values are retained as lexical text so this model does not
//! impose a particular integer width, floating-point representation,
//! precision, or numeric interpretation.
//!
//! ## Artifacts
//!
//! An artifact describes a package-contained realization or deliverable.
//! Its format, compatibility requirements and capabilities are extensible.
//!
//! The presence of a target, ABI, compiler, IR, capability, digest,
//! signature or provenance declaration does not prove compatibility,
//! integrity, authenticity, authorization or feasibility.
//!
//! Those decisions belong to their respective authorities.
//!
//! ## Portability
//!
//! The model does not enumerate classical, quantum, HDL, AI, tensor,
//! hybrid, distributed, networking, cryptographic, embedded, accelerator,
//! or future computational domains.
//!
//! New artifact formats, compatibility namespaces and capabilities are
//! represented using extensible identifiers.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Packages.
//! - Metadata entries.
//! - Metadata nesting.
//! - Content entries.
//! - Artifacts.
//! - Compatibility declarations.
//! - Capabilities.
//! - Signatures.
//! - Provenance references.
//! - Package-name qualification.
//! - Version-string length.
//! - Computing domains.
//! - Hardware resources.
//!
//! Collections grow according to available memory and allocator capacity.
//! Processing remains subject to available resources and explicit
//! operational budgets owned by the calling subsystem.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Determinism
//!
//! Observable maps and sets use BTreeMap and BTreeSet.
//!
//! Equivalent package models therefore expose deterministic iteration
//! order for their ordered collections.
//!
//! This does not by itself guarantee reproducible archives. The archive
//! subsystem owns canonical serialization and archive reproducibility.
//!
//! ## Security
//!
//! This module performs no filesystem, network, process, environment,
//! credential, hardware or cryptographic operations.
//!
//! Package paths are validated as portable relative paths. The archive
//! extraction subsystem must independently enforce containment and
//! extraction-safety rules at the filesystem boundary.
//!
//! ## Integration
//!
//! Consumed by:
//!
//! - `packages::archive`
//! - `packages::creation`
//! - `packages::extraction`
//! - `packages::installation`
//! - `packages::verification`
//! - `packages::metadata`
//! - `packages::validation`
//! - `registries`
//! - `dependencies`
//!
//! The package model does not own the lifecycle operations performed by
//! those consumers.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::toolchain::dependencies::constraints::VersionSchemeId;
use crate::toolchain::dependencies::model::DependencyName;
use crate::toolchain::errors::ToolchainError;
use crate::toolchain::ids::{PackageId, SourceId};
use crate::toolchain::result::Result;

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_MODEL: &str = "package.model";
const OPERATION_VALIDATION: &str = "package.model.validate";

// ============================================================================
// COMMON VALIDATION
// ============================================================================

/// Validate required textual data without silently normalizing it.
///
/// This function deliberately imposes no arbitrary string-length limit.
///
/// Leading or trailing whitespace is rejected because silently trimming
/// externally supplied identifiers can change their meaning.
fn validate_text(value: &str, field: &'static str) -> Result<()> {
    if value.is_empty() {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} must not be empty"),
            )
            .with_operation(OPERATION_VALIDATION),
        );
    }

    if value.trim() != value {
        return Err(
            ToolchainError::invalid_input(
                format!(
                    "{field} must not contain leading or trailing whitespace"
                ),
            )
            .with_operation(OPERATION_VALIDATION),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} must not contain control characters"),
            )
            .with_operation(OPERATION_VALIDATION),
        );
    }

    Ok(())
}

/// Validate a metadata string value.
///
/// Newlines, carriage returns and tabs are permitted because they may be
/// meaningful decoded string data. NUL and other control characters are
/// rejected.
fn validate_string_value(value: &str, field: &'static str) -> Result<()> {
    if value.contains('\0')
        || value.chars().any(|character| {
            character.is_control()
                && !matches!(character, '\n' | '\r' | '\t')
        })
    {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} contains prohibited control characters"),
            )
            .with_operation(OPERATION_VALIDATION),
        );
    }

    Ok(())
}

/// Validate a metadata or extension key.
///
/// The canonical grammar owns the actual source-language identifier
/// syntax. This boundary validates structural text only.
fn validate_key(value: &str, field: &'static str) -> Result<()> {
    validate_text(value, field)?;

    if value.contains('\0') {
        return Err(
            ToolchainError::invalid_input(
                format!("{field} contains a NUL character"),
            )
            .with_operation(OPERATION_VALIDATION),
        );
    }

    Ok(())
}

// ============================================================================
// PACKAGE IDENTITY
// ============================================================================

/// Stable logical identity of a package.
///
/// A package ID is independent of:
///
/// - Package display name.
/// - Package version.
/// - Package source.
/// - Package archive location.
/// - Package content digest.
/// - Artifact target.
/// - Physical hardware.
///
/// The ID is allocated by the canonical toolchain ID authority.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageIdentity {
    id: PackageId,
    name: DependencyName,
}

impl PackageIdentity {
    /// Construct a package identity from an allocated ID and canonical name.
    ///
    /// This function does not allocate IDs or establish global uniqueness.
    pub fn new(id: PackageId, name: DependencyName) -> Self {
        Self { id, name }
    }

    /// Return the stable package ID.
    #[must_use]
    pub fn id(&self) -> &PackageId {
        &self.id
    }

    /// Return the canonical package name.
    #[must_use]
    pub fn name(&self) -> &DependencyName {
        &self.name
    }
}

// ============================================================================
// PACKAGE RELEASE VERSION
// ============================================================================

/// Exact release version of a package.
///
/// A package release version is distinct from:
///
/// - A dependency version requirement.
/// - The toolchain API version.
/// - The package archive schema version.
/// - The Zamani language version.
/// - The compiler version.
/// - An IR schema version.
/// - A backend executable version.
///
/// The version scheme is explicit. This type does not assume SemVer or
/// interpret the version text.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageVersion {
    scheme: VersionSchemeId,
    value: String,
}

impl PackageVersion {
    /// Construct a package release version using an explicit scheme.
    ///
    /// The scheme determines how another authoritative subsystem may
    /// interpret the version. This constructor does not compare versions.
    pub fn new(
        scheme: VersionSchemeId,
        value: impl Into<String>,
    ) -> Result<Self> {
        let value = value.into();

        validate_text(&value, "package release version")?;

        Ok(Self { scheme, value })
    }

    /// Return the version-scheme identifier.
    #[must_use]
    pub fn scheme(&self) -> &VersionSchemeId {
        &self.scheme
    }

    /// Return the exact release-version text.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for PackageVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.value)
    }
}

// ============================================================================
// PACKAGE TYPE
// ============================================================================

/// Extensible package classification.
///
/// Examples may include application, library, source, tool, model,
/// dataset, hardware-design, firmware, or future ecosystem-defined types.
///
/// This is intentionally not a closed enum.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageType(String);

impl PackageType {
    /// Construct a package-type identifier.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_key(&value, "package type")?;

        Ok(Self(value))
    }

    /// Return the package-type identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PackageType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// STRUCTURED PACKAGE METADATA VALUES
// ============================================================================

/// Static package metadata value.
///
/// This representation follows the structural intent of
/// `grammar/modules/packages.g4`, which composes the canonical
/// `attributeValue` grammar.
///
/// Numeric values remain lexical text. This avoids imposing fixed numeric
/// widths or precision at the package-model boundary.
///
/// This type is data only. It does not evaluate expressions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum PackageMetadataValue {
    /// A decoded string value.
    String(String),

    /// An integer literal preserved in its original lexical representation.
    Integer(String),

    /// A floating-point or decimal literal preserved as lexical text.
    Decimal(String),

    /// A character value containing one Unicode scalar value.
    Character(char),

    /// A boolean value.
    Boolean(bool),

    /// A symbolic or qualified name.
    ///
    /// The canonical name grammar remains authoritative for source syntax.
    Name(String),

    /// An ordered sequence of metadata values.
    List(Vec<PackageMetadataValue>),

    /// A deterministically ordered map of metadata values.
    Map(BTreeMap<String, PackageMetadataValue>),
}

impl PackageMetadataValue {
    /// Construct a string metadata value.
    pub fn string(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_string_value(&value, "package metadata string")?;

        Ok(Self::String(value))
    }

    /// Construct an integer metadata value without imposing a numeric width.
    pub fn integer(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_text(&value, "package metadata integer literal")?;

        Ok(Self::Integer(value))
    }

    /// Construct a decimal metadata value without imposing a precision.
    pub fn decimal(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_text(&value, "package metadata decimal literal")?;

        Ok(Self::Decimal(value))
    }

    /// Construct a character metadata value.
    #[must_use]
    pub const fn character(value: char) -> Self {
        Self::Character(value)
    }

    /// Construct a boolean metadata value.
    #[must_use]
    pub const fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }

    /// Construct a symbolic or qualified-name metadata value.
    pub fn name(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_key(&value, "package metadata name")?;

        Ok(Self::Name(value))
    }

    /// Construct a list metadata value.
    ///
    /// Nested values are validated recursively when the containing package
    /// is validated.
    #[must_use]
    pub fn list(values: Vec<Self>) -> Self {
        Self::List(values)
    }

    /// Construct a metadata map from entries.
    ///
    /// Duplicate keys are rejected instead of silently overwriting values.
    pub fn map<I, K>(entries: I) -> Result<Self>
    where
        I: IntoIterator<Item = (K, Self)>,
        K: Into<String>,
    {
        let mut values = BTreeMap::new();

        for (key, value) in entries {
            let key = key.into();

            validate_key(&key, "package metadata map key")?;

            if values.insert(key.clone(), value).is_some() {
                return Err(
                    ToolchainError::validation_failed(
                        format!(
                            "duplicate package metadata map key `{key}`"
                        ),
                    )
                    .with_operation(OPERATION_MODEL),
                );
            }
        }

        Ok(Self::Map(values))
    }

    /// Validate this metadata value and all nested values.
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::String(value) => {
                validate_string_value(
                    value,
                    "package metadata string",
                )
            }

            Self::Integer(value) => {
                validate_text(
                    value,
                    "package metadata integer literal",
                )
            }

            Self::Decimal(value) => {
                validate_text(
                    value,
                    "package metadata decimal literal",
                )
            }

            Self::Character(_) | Self::Boolean(_) => Ok(()),

            Self::Name(value) => {
                validate_key(value, "package metadata name")
            }

            Self::List(values) => {
                for value in values {
                    value.validate()?;
                }

                Ok(())
            }

            Self::Map(values) => {
                for (key, value) in values {
                    validate_key(key, "package metadata map key")?;
                    value.validate()?;
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// PACKAGE METADATA
// ============================================================================

/// Deterministically ordered package metadata.
///
/// Metadata keys remain extensible. This model does not reserve a fixed
/// set of metadata names or impose a package-specific metadata schema.
///
/// Schema interpretation belongs to `packages::metadata` and
/// `packages::validation`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackageMetadata {
    entries: BTreeMap<String, PackageMetadataValue>,
}

impl PackageMetadata {
    /// Construct empty package metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct metadata from entries.
    ///
    /// Duplicate keys are rejected.
    pub fn from_entries<I, K>(entries: I) -> Result<Self>
    where
        I: IntoIterator<Item = (K, PackageMetadataValue)>,
        K: Into<String>,
    {
        let mut metadata = Self::new();

        for (key, value) in entries {
            metadata.insert(key, value)?;
        }

        Ok(metadata)
    }

    /// Insert one metadata entry.
    ///
    /// Existing entries are never silently overwritten.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: PackageMetadataValue,
    ) -> Result<()> {
        let key = key.into();

        validate_key(&key, "package metadata key")?;
        value.validate()?;

        if self.entries.contains_key(&key) {
            return Err(
                ToolchainError::validation_failed(
                    format!("duplicate package metadata key `{key}`"),
                )
                .with_operation(OPERATION_MODEL),
            );
        }

        self.entries.insert(key, value);

        Ok(())
    }

    /// Return a metadata value by key.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&PackageMetadataValue> {
        self.entries.get(key)
    }

    /// Return all metadata entries in deterministic key order.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<String, PackageMetadataValue> {
        &self.entries
    }

    /// Return the number of metadata entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Return whether metadata is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Validate every metadata entry.
    pub fn validate(&self) -> Result<()> {
        for (key, value) in &self.entries {
            validate_key(key, "package metadata key")?;
            value.validate()?;
        }

        Ok(())
    }
}

// ============================================================================
// PORTABLE PACKAGE CONTENT PATH
// ============================================================================

/// Validated portable relative path inside a package.
///
/// The representation uses `/` separators independently of the host
/// operating system.
///
/// This prevents package identity from depending on Windows, Unix or
/// another host path representation.
///
/// This is a logical package path, not an already-authorized filesystem
/// destination. Extraction must still enforce destination containment,
/// symlink policy and archive-entry safety.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackagePath(String);

impl PackagePath {
    /// Construct a portable package-relative path.
    ///
    /// Rejected forms include:
    ///
    /// - Empty paths.
    /// - Absolute paths.
    /// - Empty path segments.
    /// - `.` and `..` segments.
    /// - Backslash separators.
    /// - Drive/alternate-stream colon syntax.
    /// - Control characters.
    ///
    /// No arbitrary path-length ceiling is imposed.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_text(&value, "package content path")?;

        if value.starts_with('/')
            || value.contains('\\')
            || value.contains(':')
            || value.contains('\0')
        {
            return Err(
                ToolchainError::invalid_input(
                    "package content path must be a portable relative path",
                )
                .with_operation(OPERATION_MODEL),
            );
        }

        let segments: Vec<&str> = value.split('/').collect();

        if segments.iter().any(|segment| {
            segment.is_empty() || *segment == "." || *segment == ".."
        }) {
            return Err(
                ToolchainError::invalid_input(
                    "package content path contains an invalid path segment",
                )
                .with_operation(OPERATION_MODEL),
            );
        }

        Ok(Self(value))
    }

    /// Return the canonical portable path text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PackagePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// CONTENT KIND
// ============================================================================

/// Extensible classification of a package content entry.
///
/// Examples may include source, artifact, resource, documentation,
/// license, metadata, dataset, model, firmware, or future content kinds.
///
/// This is not a closed list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageContentKind(String);

impl PackageContentKind {
    /// Construct a content-kind identifier.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();

        validate_key(&value, "package content kind")?;

        Ok(Self(value))
    }

    /// Return the content-kind identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// CONTENT DIGEST DESCRIPTOR
// ============================================================================

/// Declared content digest.
///
/// This structure records an algorithm identifier and its digest text.
/// It does not calculate or verify the digest.
///
/// The security and package-verification authorities own those operations.
///
/// The algorithm and representation remain extensible.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentDigest {
    algorithm: String,
    value: String,
}

impl ContentDigest {
    /// Construct a digest descriptor.
    ///
    /// No fixed digest algorithm or encoded digest length is assumed.
    pub fn new(
        algorithm: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self> {
        let algorithm = algorithm.into();
        let value = value.into();

        validate_key(&algorithm, "digest algorithm")?;
        validate_text(&value, "digest value")?;

        Ok(Self { algorithm, value })
    }

    /// Return the digest algorithm identifier.
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    /// Return the digest representation.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// PACKAGE CONTENT ENTRY
// ============================================================================

/// Descriptor for one logical item contained in a package.
///
/// This records package contents without reading files or storing archive
/// bytes. The archive and extraction modules own physical content handling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageContent {
    path: PackagePath,
    kind: PackageContentKind,
    media_type: Option<String>,
    declared_size: Option<u128>,
    digest: Option<ContentDigest>,
}

impl PackageContent {
    /// Construct a content descriptor.
    pub fn new(path: PackagePath, kind: PackageContentKind) -> Self {
        Self {
            path,
            kind,
            media_type: None,
            declared_size: None,
            digest: None,
        }
    }

    /// Set an optional media type.
    pub fn with_media_type(
        mut self,
        media_type: impl Into<String>,
    ) -> Result<Self> {
        let media_type = media_type.into();

        validate_key(&media_type, "package content media type")?;

        self.media_type = Some(media_type);

        Ok(self)
    }

    /// Set the declared content size in bytes.
    ///
    /// This is descriptive metadata, not a resource allocation or
    /// extraction budget.
    #[must_use]
    pub fn with_declared_size(mut self, size: u128) -> Self {
        self.declared_size = Some(size);
        self
    }

    /// Set the declared content digest.
    ///
    /// This does not verify the digest.
    #[must_use]
    pub fn with_digest(mut self, digest: ContentDigest) -> Self {
        self.digest = Some(digest);
        self
    }

    /// Return the portable content path.
    #[must_use]
    pub fn path(&self) -> &PackagePath {
        &self.path
    }

    /// Return the content-kind identifier.
    #[must_use]
    pub fn kind(&self) -> &PackageContentKind {
        &self.kind
    }

    /// Return the optional media type.
    #[must_use]
    pub fn media_type(&self) -> Option<&str> {
        self.media_type.as_deref()
    }

    /// Return the declared size, if present.
    #[must_use]
    pub const fn declared_size(&self) -> Option<u128> {
        self.declared_size
    }

    /// Return the declared digest, if present.
    #[must_use]
    pub fn digest(&self) -> Option<&ContentDigest> {
        self.digest.as_ref()
    }

    /// Validate the content descriptor.
    pub fn validate(&self) -> Result<()> {
        PackagePath::new(self.path.as_str())?;

        validate_key(self.kind.as_str(), "package content kind")?;

        if let Some(media_type) = &self.media_type {
            validate_key(media_type, "package content media type")?;
        }

        Ok(())
    }
}

// ============================================================================
// ARTIFACT COMPATIBILITY DECLARATION
// ============================================================================

/// Extensible compatibility requirement attached to an artifact.
///
/// The namespace identifies the compatibility contract. The expression
/// is interpreted by the appropriate compatibility authority.
///
/// Examples of possible namespaces include compiler, ABI, IR, runtime,
/// platform, domain, serialization, and provider contracts.
///
/// This type does not evaluate compatibility.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageCompatibilityRequirement {
    namespace: String,
    expression: String,
}

impl PackageCompatibilityRequirement {
    /// Construct a compatibility declaration.
    pub fn new(
        namespace: impl Into<String>,
        expression: impl Into<String>,
    ) -> Result<Self> {
        let namespace = namespace.into();
        let expression = expression.into();

        validate_key(&namespace, "compatibility namespace")?;
        validate_text(&expression, "compatibility expression")?;

        Ok(Self {
            namespace,
            expression,
        })
    }

    /// Return the compatibility namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the opaque compatibility expression.
    #[must_use]
    pub fn expression(&self) -> &str {
        &self.expression
    }
}

// ============================================================================
// ARTIFACT SIGNATURE REFERENCE
// ============================================================================

/// Reference to a signature associated with an artifact.
///
/// The model preserves signature metadata. It does not verify the
/// signature, establish publisher identity, or make a trust decision.
///
/// Verification belongs to the security subsystem.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactSignatureReference {
    algorithm: String,
    signer: String,
    signature: String,
}

impl ArtifactSignatureReference {
    /// Construct a signature reference.
    pub fn new(
        algorithm: impl Into<String>,
        signer: impl Into<String>,
        signature: impl Into<String>,
    ) -> Result<Self> {
        let algorithm = algorithm.into();
        let signer = signer.into();
        let signature = signature.into();

        validate_key(&algorithm, "signature algorithm")?;
        validate_text(&signer, "signature signer")?;
        validate_text(&signature, "signature value")?;

        Ok(Self {
            algorithm,
            signer,
            signature,
        })
    }

    /// Return the signature algorithm identifier.
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    /// Return the signer reference.
    #[must_use]
    pub fn signer(&self) -> &str {
        &self.signer
    }

    /// Return the opaque signature representation.
    #[must_use]
    pub fn signature(&self) -> &str {
        &self.signature
    }
}

// ============================================================================
// ARTIFACT PROVENANCE REFERENCE
// ============================================================================

/// Reference to provenance information associated with an artifact.
///
/// The provenance authority owns provenance generation, validation,
/// authentication and policy evaluation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactProvenanceReference {
    kind: String,
    value: String,
}

impl ArtifactProvenanceReference {
    /// Construct an opaque provenance reference.
    pub fn new(
        kind: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self> {
        let kind = kind.into();
        let value = value.into();

        validate_key(&kind, "provenance reference kind")?;
        validate_text(&value, "provenance reference value")?;

        Ok(Self { kind, value })
    }

    /// Return the provenance reference kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Return the opaque provenance reference value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// PACKAGE ARTIFACT DESCRIPTOR
// ============================================================================

/// Description of one package-contained artifact.
///
/// An artifact may represent a source bundle, executable, library,
/// WebAssembly module, quantum circuit, HDL design input, model, dataset,
/// firmware image, or another registered artifact format.
///
/// The format and compatibility contracts are extensible. This module
/// does not enumerate target architectures or computing domains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageArtifact {
    content_path: PackagePath,
    format: String,
    compatibility: BTreeSet<PackageCompatibilityRequirement>,
    capabilities: BTreeSet<String>,
    digest: Option<ContentDigest>,
    signatures: BTreeSet<ArtifactSignatureReference>,
    provenance: BTreeSet<ArtifactProvenanceReference>,
}

impl PackageArtifact {
    /// Construct an artifact descriptor referencing package content.
    pub fn new(
        content_path: PackagePath,
        format: impl Into<String>,
    ) -> Result<Self> {
        let format = format.into();

        validate_key(&format, "package artifact format")?;

        Ok(Self {
            content_path,
            format,
            compatibility: BTreeSet::new(),
            capabilities: BTreeSet::new(),
            digest: None,
            signatures: BTreeSet::new(),
            provenance: BTreeSet::new(),
        })
    }

    /// Add an extensible compatibility declaration.
    ///
    /// Duplicate declarations are rejected.
    pub fn with_compatibility(
        mut self,
        requirement: PackageCompatibilityRequirement,
    ) -> Result<Self> {
        if !self.compatibility.insert(requirement) {
            return Err(
                ToolchainError::validation_failed(
                    "duplicate artifact compatibility declaration",
                )
                .with_operation(OPERATION_MODEL),
            );
        }

        Ok(self)
    }

    /// Add a capability declaration.
    ///
    /// A declared capability is not proof that a provider or target
    /// actually supplies it.
    pub fn with_capability(
        mut self,
        capability: impl Into<String>,
    ) -> Result<Self> {
        let capability = capability.into();

        validate_key(&capability, "artifact capability")?;

        if !self.capabilities.insert(capability.clone()) {
            return Err(
                ToolchainError::validation_failed(
                    format!("duplicate artifact capability `{capability}`"),
                )
                .with_operation(OPERATION_MODEL),
            );
        }

        Ok(self)
    }

    /// Attach a declared digest.
    ///
    /// This does not calculate or verify the digest.
    #[must_use]
    pub fn with_digest(mut self, digest: ContentDigest) -> Self {
        self.digest = Some(digest);
        self
    }

    /// Attach a signature reference.
    ///
    /// This does not verify the signature.
    pub fn with_signature(
        mut self,
        signature: ArtifactSignatureReference,
    ) -> Result<Self> {
        if !self.signatures.insert(signature) {
            return Err(
                ToolchainError::validation_failed(
                    "duplicate artifact signature reference",
                )
                .with_operation(OPERATION_MODEL),
            );
        }

        Ok(self)
    }

    /// Attach a provenance reference.
    pub fn with_provenance(
        mut self,
        provenance: ArtifactProvenanceReference,
    ) -> Result<Self> {
        if !self.provenance.insert(provenance) {
            return Err(
                ToolchainError::validation_failed(
                    "duplicate artifact provenance reference",
                )
                .with_operation(OPERATION_MODEL),
            );
        }

        Ok(self)
    }

    /// Return the package-relative content path.
    #[must_use]
    pub fn content_path(&self) -> &PackagePath {
        &self.content_path
    }

    /// Return the artifact format identifier.
    #[must_use]
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Return compatibility declarations in deterministic order.
    #[must_use]
    pub fn compatibility(
        &self,
    ) -> &BTreeSet<PackageCompatibilityRequirement> {
        &self.compatibility
    }

    /// Return declared capabilities in deterministic order.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<String> {
        &self.capabilities
    }

    /// Return the optional declared digest.
    #[must_use]
    pub fn digest(&self) -> Option<&ContentDigest> {
        self.digest.as_ref()
    }

    /// Return signature references in deterministic order.
    #[must_use]
    pub fn signatures(&self) -> &BTreeSet<ArtifactSignatureReference> {
        &self.signatures
    }

    /// Return provenance references in deterministic order.
    #[must_use]
    pub fn provenance(&self) -> &BTreeSet<ArtifactProvenanceReference> {
        &self.provenance
    }

    /// Validate the artifact descriptor.
    pub fn validate(&self) -> Result<()> {
        PackagePath::new(self.content_path.as_str())?;

        validate_key(&self.format, "package artifact format")?;

        for requirement in &self.compatibility {
            validate_key(
                requirement.namespace(),
                "compatibility namespace",
            )?;

            validate_text(
                requirement.expression(),
                "compatibility expression",
            )?;
        }

        for capability in &self.capabilities {
            validate_key(capability, "artifact capability")?;
        }

        Ok(())
    }
}

// ============================================================================
// PACKAGE ORIGIN
// ============================================================================

/// Declared origin of a package release.
///
/// The source ID belongs to the dependency-source identity authority.
/// The revision remains opaque and is not fetched or verified here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageOrigin {
    source: SourceId,
    revision: Option<String>,
}

impl PackageOrigin {
    /// Construct a package origin descriptor.
    pub fn new(
        source: SourceId,
        revision: Option<String>,
    ) -> Result<Self> {
        if let Some(value) = revision.as_deref() {
            validate_text(value, "package source revision")?;
        }

        Ok(Self { source, revision })
    }

    /// Return the canonical source identity.
    #[must_use]
    pub fn source(&self) -> &SourceId {
        &self.source
    }

    /// Return the optional immutable source revision.
    #[must_use]
    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }
}

// ============================================================================
// PACKAGE
// ============================================================================

/// Complete domain-neutral package model.
///
/// A package contains:
///
/// - One stable package identity.
/// - One exact release version.
/// - One extensible package type.
/// - Structured package metadata.
/// - A deterministic inventory of package content.
/// - A deterministic inventory of artifact descriptors.
/// - Optional package capability declarations.
/// - Optional source-origin information.
///
/// Package dependency declarations are deliberately not stored here.
/// They remain owned by `dependencies::model` and the dependency subsystem.
///
/// Construct a package with [`Package::new`], add its descriptors through
/// the builder methods, and call [`Package::validate`] before passing it
/// to archive, publication, installation or verification workflows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    identity: PackageIdentity,
    version: PackageVersion,
    package_type: PackageType,
    metadata: PackageMetadata,
    contents: BTreeMap<PackagePath, PackageContent>,
    artifacts: BTreeMap<PackagePath, PackageArtifact>,
    capabilities: BTreeSet<String>,
    origin: Option<PackageOrigin>,
}

impl Package {
    /// Construct a package with its required identity, release and type.
    pub fn new(
        identity: PackageIdentity,
        version: PackageVersion,
        package_type: PackageType,
    ) -> Self {
        Self {
            identity,
            version,
            package_type,
            metadata: PackageMetadata::new(),
            contents: BTreeMap::new(),
            artifacts: BTreeMap::new(),
            capabilities: BTreeSet::new(),
            origin: None,
        }
    }

    /// Set the package origin.
    #[must_use]
    pub fn with_origin(mut self, origin: PackageOrigin) -> Self {
        self.origin = Some(origin);
        self
    }

    /// Add package metadata.
    ///
    /// Duplicate metadata keys are rejected.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: PackageMetadataValue,
    ) -> Result<Self> {
        self.metadata.insert(key, value)?;
        Ok(self)
    }

    /// Add a package content descriptor.
    ///
    /// Content paths must be unique within one package.
    pub fn with_content(
        mut self,
        content: PackageContent,
    ) -> Result<Self> {
        content.validate()?;

        let path = content.path().clone();

        if self.contents.insert(path.clone(), content).is_some() {
            return Err(
                ToolchainError::validation_failed(
                    format!("duplicate package content path `{path}`"),
                )
                .with_operation(OPERATION_MODEL)
                .with_subject(self.identity.id().to_string()),
            );
        }

        Ok(self)
    }

    /// Add an artifact descriptor.
    ///
    /// Each artifact references one package content path. The referenced
    /// content must be present when the complete package is validated.
    ///
    /// Artifact paths must be unique within one package.
    pub fn with_artifact(
        mut self,
        artifact: PackageArtifact,
    ) -> Result<Self> {
        artifact.validate()?;

        let path = artifact.content_path().clone();

        if self.artifacts.insert(path.clone(), artifact).is_some() {
            return Err(
                ToolchainError::validation_failed(
                    format!("duplicate package artifact path `{path}`"),
                )
                .with_operation(OPERATION_MODEL)
                .with_subject(self.identity.id().to_string()),
            );
        }

        Ok(self)
    }

    /// Add a package-level capability declaration.
    ///
    /// Capability declarations are descriptive. They do not prove
    /// physical availability or execution feasibility.
    pub fn with_capability(
        mut self,
        capability: impl Into<String>,
    ) -> Result<Self> {
        let capability = capability.into();

        validate_key(&capability, "package capability")?;

        if !self.capabilities.insert(capability.clone()) {
            return Err(
                ToolchainError::validation_failed(
                    format!("duplicate package capability `{capability}`"),
                )
                .with_operation(OPERATION_MODEL)
                .with_subject(self.identity.id().to_string()),
            );
        }

        Ok(self)
    }

    /// Return the package identity.
    #[must_use]
    pub fn identity(&self) -> &PackageIdentity {
        &self.identity
    }

    /// Return the package release version.
    #[must_use]
    pub fn version(&self) -> &PackageVersion {
        &self.version
    }

    /// Return the package type.
    #[must_use]
    pub fn package_type(&self) -> &PackageType {
        &self.package_type
    }

    /// Return package metadata.
    #[must_use]
    pub fn metadata(&self) -> &PackageMetadata {
        &self.metadata
    }

    /// Return package contents in deterministic path order.
    #[must_use]
    pub fn contents(&self) -> &BTreeMap<PackagePath, PackageContent> {
        &self.contents
    }

    /// Return package artifacts in deterministic path order.
    #[must_use]
    pub fn artifacts(&self) -> &BTreeMap<PackagePath, PackageArtifact> {
        &self.artifacts
    }

    /// Return package capability declarations.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<String> {
        &self.capabilities
    }

    /// Return the optional package origin.
    #[must_use]
    pub fn origin(&self) -> Option<&PackageOrigin> {
        self.origin.as_ref()
    }

    /// Return the number of package content entries.
    #[must_use]
    pub fn content_count(&self) -> usize {
        self.contents.len()
    }

    /// Return the number of artifact descriptors.
    #[must_use]
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }

    /// Validate the complete package model.
    ///
    /// This performs structural validation only. It does not:
    ///
    /// - Read package files.
    /// - Verify content digests.
    /// - Verify signatures.
    /// - Authenticate a publisher.
    /// - Resolve dependencies.
    /// - Evaluate compatibility expressions.
    /// - Verify target capabilities.
    /// - Establish hardware feasibility.
    /// - Serialize or extract an archive.
    ///
    /// Those checks must be performed by the authoritative downstream
    /// subsystems before the corresponding operation is accepted.
    pub fn validate(&self) -> Result<()> {
        validate_key(
            self.identity.name().as_str(),
            "package name",
        )?;

        validate_text(
            self.version.value(),
            "package release version",
        )?;

        validate_key(
            self.package_type.as_str(),
            "package type",
        )?;

        self.metadata.validate()?;

        for (path, content) in &self.contents {
            content.validate()?;

            if path != content.path() {
                return Err(
                    ToolchainError::validation_failed(
                        "package content map key does not match its content path",
                    )
                    .with_operation(OPERATION_VALIDATION)
                    .with_subject(self.identity.id().to_string()),
                );
            }
        }

        for (path, artifact) in &self.artifacts {
            artifact.validate()?;

            if path != artifact.content_path() {
                return Err(
                    ToolchainError::validation_failed(
                        "package artifact map key does not match its content path",
                    )
                    .with_operation(OPERATION_VALIDATION)
                    .with_subject(self.identity.id().to_string()),
                );
            }

            let Some(content) = self.contents.get(path) else {
                return Err(
                    ToolchainError::validation_failed(
                        format!(
                            "artifact `{path}` references content absent from the package"
                        ),
                    )
                    .with_operation(OPERATION_VALIDATION)
                    .with_subject(self.identity.id().to_string()),
                );
            };

            if let (Some(content_digest), Some(artifact_digest)) =
                (content.digest(), artifact.digest())
            {
                if content_digest != artifact_digest {
                    return Err(
                        ToolchainError::validation_failed(
                            format!(
                                "artifact and content digest declarations disagree for `{path}`"
                            ),
                        )
                        .with_operation(OPERATION_VALIDATION)
                        .with_subject(self.identity.id().to_string()),
                    );
                }
            }
        }

        for capability in &self.capabilities {
            validate_key(capability, "package capability")?;
        }

        if let Some(origin) = &self.origin {
            if let Some(revision) = origin.revision() {
                validate_text(revision, "package source revision")?;
            }
        }

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::ids::IdNamespace;

    fn package_id(value: u128) -> PackageId {
        PackageId::new(
            IdNamespace::new("zamani.package")
                .expect("valid namespace"),
            value,
        )
        .expect("valid package ID")
    }

    fn package_name(value: &str) -> DependencyName {
        DependencyName::new(value).expect("valid package name")
    }

    fn version() -> PackageVersion {
        PackageVersion::new(
            VersionSchemeId::new("semver")
                .expect("valid version scheme"),
            "1.0.0",
        )
        .expect("valid package version")
    }

    fn package() -> Package {
        Package::new(
            PackageIdentity::new(
                package_id(1),
                package_name("org::zamani::example"),
            ),
            version(),
            PackageType::new("library").expect("valid package type"),
        )
    }

    fn content(path: &str, kind: &str) -> PackageContent {
        PackageContent::new(
            PackagePath::new(path).expect("valid package path"),
            PackageContentKind::new(kind).expect("valid content kind"),
        )
    }

    #[test]
    fn package_identity_is_distinct_from_package_version() {
        let first = PackageIdentity::new(
            package_id(1),
            package_name("org::zamani::math"),
        );

        let second = PackageIdentity::new(
            package_id(1),
            package_name("org::zamani::math"),
        );

        assert_eq!(first, second);
        assert_eq!(first.id(), second.id());
    }

    #[test]
    fn package_versions_require_an_explicit_scheme() {
        let package_version = version();

        assert_eq!(package_version.scheme().as_str(), "semver");
        assert_eq!(package_version.value(), "1.0.0");
    }

    #[test]
    fn arbitrary_version_schemes_are_preserved() {
        let package_version = PackageVersion::new(
            VersionSchemeId::new("vendor.release.scheme")
                .expect("valid scheme"),
            "release-alpha-2026-r7",
        )
        .expect("valid package version");

        assert_eq!(
            package_version.value(),
            "release-alpha-2026-r7",
        );
    }

    #[test]
    fn package_type_is_extensible() {
        let package_type = PackageType::new(
            "future.domain.package",
        )
        .expect("valid package type");

        assert_eq!(
            package_type.as_str(),
            "future.domain.package",
        );
    }

    #[test]
    fn metadata_maps_are_deterministically_ordered() {
        let metadata = PackageMetadata::from_entries([
            (
                "zeta",
                PackageMetadataValue::boolean(true),
            ),
            (
                "alpha",
                PackageMetadataValue::string("value")
                    .expect("valid string"),
            ),
        ])
        .expect("valid metadata");

        let keys: Vec<&str> = metadata
            .entries()
            .keys()
            .map(String::as_str)
            .collect();

        assert_eq!(keys, vec!["alpha", "zeta"]);
    }

    #[test]
    fn duplicate_metadata_keys_are_rejected() {
        let result = PackageMetadata::from_entries([
            (
                "license",
                PackageMetadataValue::string("MIT")
                    .expect("valid string"),
            ),
            (
                "license",
                PackageMetadataValue::string("Apache-2.0")
                    .expect("valid string"),
            ),
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn nested_metadata_is_supported() {
        let nested = PackageMetadataValue::map([
            (
                "domain",
                PackageMetadataValue::name("quantum")
                    .expect("valid name"),
            ),
            (
                "portable",
                PackageMetadataValue::boolean(true),
            ),
        ])
        .expect("valid nested map");

        let metadata = PackageMetadata::from_entries([(
            "capabilities",
            PackageMetadataValue::list(vec![nested]),
        )])
        .expect("valid metadata");

        metadata.validate().expect("metadata validates");
    }

    #[test]
    fn numeric_metadata_does_not_assume_fixed_integer_width() {
        let large_integer = "999999999999999999999999999999999999999999999999";

        let value = PackageMetadataValue::integer(large_integer)
            .expect("valid lexical integer");

        assert_eq!(
            value,
            PackageMetadataValue::Integer(large_integer.to_owned()),
        );
    }

    #[test]
    fn package_paths_are_portable_and_relative() {
        assert!(PackagePath::new("src/lib.zm").is_ok());
        assert!(PackagePath::new("models/weights.bin").is_ok());
        assert!(PackagePath::new("hardware/design/top.v").is_ok());

        assert!(PackagePath::new("").is_err());
        assert!(PackagePath::new("/absolute/file").is_err());
        assert!(PackagePath::new("../outside").is_err());
        assert!(PackagePath::new("src/../outside").is_err());
        assert!(PackagePath::new("src//file").is_err());
        assert!(PackagePath::new("src\\file").is_err());
        assert!(PackagePath::new("C:/file").is_err());
    }

    #[test]
    fn duplicate_content_paths_are_rejected() {
        let result = package()
            .with_content(content("src/lib.zm", "source"))
            .expect("first content")
            .with_content(content("src/lib.zm", "source"));

        assert!(result.is_err());
    }

    #[test]
    fn artifact_must_reference_existing_package_content() {
        let artifact = PackageArtifact::new(
            PackagePath::new("bin/app").expect("valid path"),
            "native-executable",
        )
        .expect("valid artifact");

        let package = package()
            .with_artifact(artifact)
            .expect("artifact descriptor");

        assert!(package.validate().is_err());
    }

    #[test]
    fn package_with_artifact_and_content_validates() {
        let path = PackagePath::new("artifacts/app.bin")
            .expect("valid path");

        let digest = ContentDigest::new(
            "sha256",
            "declared-digest-value",
        )
        .expect("valid digest descriptor");

        let content = PackageContent::new(
            path.clone(),
            PackageContentKind::new("binary")
                .expect("valid content kind"),
        )
        .with_digest(digest.clone());

        let artifact = PackageArtifact::new(
            path,
            "registered.binary.format",
        )
        .expect("valid artifact")
        .with_digest(digest);

        let package = package()
            .with_content(content)
            .expect("add content")
            .with_artifact(artifact)
            .expect("add artifact");

        package.validate().expect("package validates");
    }

    #[test]
    fn conflicting_content_and_artifact_digests_are_rejected() {
        let path = PackagePath::new("artifacts/app.bin")
            .expect("valid path");

        let content = PackageContent::new(
            path.clone(),
            PackageContentKind::new("binary")
                .expect("valid content kind"),
        )
        .with_digest(
            ContentDigest::new("sha256", "digest-a")
                .expect("valid digest"),
        );

        let artifact = PackageArtifact::new(
            path,
            "binary",
        )
        .expect("valid artifact")
        .with_digest(
            ContentDigest::new("sha256", "digest-b")
                .expect("valid digest"),
        );

        let package = package()
            .with_content(content)
            .expect("add content")
            .with_artifact(artifact)
            .expect("add artifact");

        assert!(package.validate().is_err());
    }

    #[test]
    fn compatibility_requirements_are_extensible() {
        let requirement = PackageCompatibilityRequirement::new(
            "vendor.abi",
            "compatible-with:release-7",
        )
        .expect("valid compatibility requirement");

        assert_eq!(requirement.namespace(), "vendor.abi");
        assert_eq!(
            requirement.expression(),
            "compatible-with:release-7",
        );
    }

    #[test]
    fn signatures_are_descriptive_not_verified() {
        let signature = ArtifactSignatureReference::new(
            "ed25519",
            "publisher:key-reference",
            "opaque-signature",
        )
        .expect("valid signature reference");

        assert_eq!(signature.algorithm(), "ed25519");
        assert_eq!(signature.signer(), "publisher:key-reference");
    }

    #[test]
    fn package_capabilities_are_extensible() {
        let package = package()
            .with_capability("quantum.measurement")
            .expect("quantum capability")
            .with_capability("hdl.synthesis")
            .expect("HDL capability")
            .with_capability("future.domain.operation")
            .expect("future capability");

        assert_eq!(package.capabilities().len(), 3);
        package.validate().expect("valid package");
    }

    #[test]
    fn package_model_does_not_require_artifacts() {
        let package = package();

        package.validate().expect("valid source package");
        assert_eq!(package.artifact_count(), 0);
    }

    #[test]
    fn package_content_count_tracks_insertions() {
        let package = package()
            .with_content(content("src/main.zm", "source"))
            .expect("add source")
            .with_content(content("LICENSE", "license"))
            .expect("add license");

        assert_eq!(package.content_count(), 2);
    }
}
