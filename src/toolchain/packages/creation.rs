#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Package Creation
//!
//! Path: `src/toolchain/packages/creation.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Construct deterministic, validated package-creation snapshots from
//! explicitly selected project files.
//!
//! This module is the boundary between:
//!
//! - A validated project and manifest.
//! - Resolved dependency snapshots.
//! - Compiler-produced artifact descriptions.
//! - The canonical package model.
//! - The versioned package archive writer.
//!
//! It does not parse manifests, resolve dependencies, compile programs,
//! create signatures, or serialize package archives.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Package-creation request validation.
//! - Explicit package-file selection.
//! - Project-root containment checks.
//! - Symbolic-link rejection.
//! - Package destination-path validation.
//! - Duplicate and path-collision detection.
//! - Streaming content-digest calculation.
//! - File-size and total-size accounting.
//! - Creation-time file snapshots.
//! - Deterministic package-entry ordering.
//! - Creation provenance input.
//! - Resolved dependency snapshot input.
//! - Precompiled-artifact compatibility metadata input.
//!
//! It does not own:
//!
//! - Canonical package identity storage.
//! - Package archive encoding or decoding.
//! - Package extraction.
//! - Dependency declaration parsing or resolution.
//! - Dependency graph construction.
//! - Compiler artifact generation.
//! - Target selection or hardware feasibility.
//! - Cryptographic signing or trust decisions.
//! - Registry access or publication.
//! - Installation.
//! - Runtime execution.
//!
//! ## Integration
//!
//! Upstream:
//!
//! - `crate::zamani_project_config` supplies canonical project metadata.
//! - `crate::toolchain::manifest` supplies validated manifest projections.
//! - `crate::toolchain::dependencies` supplies resolved dependency snapshots.
//! - `crate::compiler::artifacts` supplies compiler artifact descriptions.
//! - `crate::backend` supplies target-specific artifact production.
//!
//! Downstream:
//!
//! - `packages::model` converts the creation manifest into the canonical
//!   package model and assigns or validates package identity.
//! - `packages::metadata` applies package metadata conventions.
//! - `packages::validation` performs package completeness and compatibility
//!   validation.
//! - `packages::archive` consumes the creation manifest and open payload
//!   handles to write the versioned `.zpkg` format.
//! - `packages::verification` verifies content integrity and coordinates
//!   signature and provenance verification.
//! - `security` owns signing, trust and authorization.
//! - `registries` owns upload and publication.
//!
//! ## Important package-selection rule
//!
//! A project directory is not a package inclusion list.
//!
//! This module never recursively walks a project directory and never
//! packages every file it finds. The caller must provide each file through
//! `PackageCreationRequest::include_file`.
//!
//! The manifest adapter or a separately validated packaging policy should
//! produce that explicit list.
//!
//! ## Content integrity
//!
//! Files are hashed incrementally. Their complete contents are not loaded
//! into memory.
//!
//! Each payload remains backed by an open file handle. The archive writer
//! must hash the bytes again while writing and compare the result with the
//! expected digest. This detects content changes between package creation
//! and archive writing.
//!
//! The project tree should not be concurrently modified by an untrusted
//! process during package creation. Portable standard-library filesystem
//! APIs cannot provide every operating system's directory-handle-based,
//! race-free `openat`/no-follow guarantee.
//!
//! ## Determinism
//!
//! - Package metadata uses `BTreeMap`.
//! - Dependency snapshots use `BTreeSet`.
//! - Package file records are sorted by package-relative path.
//! - No wall-clock timestamps or random identifiers are introduced.
//! - Host absolute paths are not written into the package manifest.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on:
//!
//! - Package files.
//! - Package metadata entries.
//! - Dependency snapshots.
//! - Artifact compatibility declarations.
//! - Domain capability declarations.
//! - File sizes.
//! - Total package size.
//! - Target descriptions.
//! - Computing domains.
//!
//! Optional limits are caller-supplied operational budgets. They do not
//! define language or hardware capabilities.
//!
//! Actual operation remains subject to available memory, storage, file
//! system, operating system, processing time and configured budgets.
//!
//! Literal infinite storage or computation is not promised.
//!
//! ## Freeze criteria
//!
//! This file is ready to be frozen when:
//!
//! - The package model consumes `PackageDraftManifest` without redefining
//!   its creation semantics.
//! - The archive writer consumes `PackagePayload` without reopening
//!   untrusted source paths.
//! - The archive writer rechecks each content digest.
//! - The manifest adapter supplies validated package metadata.
//! - The dependency adapter supplies resolved dependency snapshots.
//! - Artifact metadata preserves actual compatibility declarations.
//! - Unit and integration tests pass.
//! - The package subsystem is wired into the toolchain composition root.

use sha2::{Digest, Sha256};

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::fs::{self, File, Metadata};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

// ============================================================================
// OPERATION IDENTIFIERS
// ============================================================================

const OPERATION_CREATE: &str = "packages.creation.create";
const OPERATION_VALIDATE: &str = "packages.creation.validate";
const SHA256_ALGORITHM_ID: &str = "sha-256";

// ============================================================================
// ERRORS
// ============================================================================

/// Errors produced while preparing a package for canonical model creation
/// and archive serialization.
#[derive(Debug)]
#[non_exhaustive]
pub enum PackageCreationError {
    /// A required input field is invalid.
    InvalidField {
        /// Field or input category.
        field: String,
        /// Explanation of the invalid value.
        reason: String,
    },

    /// A filesystem operation failed.
    Io {
        /// Operation that failed.
        operation: &'static str,
        /// Relevant path, when available.
        path: PathBuf,
        /// Original operating-system error.
        source: io::Error,
    },

    /// A symbolic link was encountered in the selected path.
    SymbolicLink {
        /// Path containing the symbolic link.
        path: PathBuf,
    },

    /// A selected entry is not a regular file.
    NotRegularFile {
        /// Selected path.
        path: PathBuf,
    },

    /// The resolved file is outside the declared project root.
    OutsideProjectRoot {
        /// Selected path.
        path: PathBuf,
    },

    /// Two entries have the same package destination.
    DuplicatePackagePath {
        /// Duplicate destination path.
        path: String,
    },

    /// Two entries select the same source file.
    DuplicateSourcePath {
        /// Duplicate source path.
        path: String,
    },

    /// One file destination conflicts with another destination's directory.
    PackagePathCollision {
        /// First conflicting destination.
        first: String,
        /// Second conflicting destination.
        second: String,
    },

    /// The package has no selected content.
    EmptyPackage,

    /// A caller-supplied operational budget was exceeded.
    BudgetExceeded {
        /// Budget name.
        budget: &'static str,
        /// Observed value.
        actual: u128,
        /// Configured maximum.
        maximum: u128,
    },

    /// A checked size calculation overflowed.
    SizeOverflow,

    /// A file changed while its creation-time digest was being calculated.
    FileChanged {
        /// Package-relative destination.
        package_path: String,
    },

    /// A digest provider failed.
    DigestFailure {
        /// Digest algorithm identifier.
        algorithm: String,
        /// Original I/O error.
        source: io::Error,
    },

    /// Creation was cancelled by the caller.
    Cancelled,

    /// A duplicate resolved dependency was supplied.
    DuplicateDependency {
        /// Dependency identity.
        identity: String,
    },
}

impl fmt::Display for PackageCreationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidField { field, reason } => {
                write!(formatter, "invalid {field}: {reason}")
            }

            Self::Io {
                operation,
                path,
                source,
            } => {
                write!(
                    formatter,
                    "{operation} failed for '{}': {source}",
                    path.display()
                )
            }

            Self::SymbolicLink { path } => {
                write!(
                    formatter,
                    "symbolic links are not accepted in package inputs: '{}'",
                    path.display()
                )
            }

            Self::NotRegularFile { path } => {
                write!(
                    formatter,
                    "package input is not a regular file: '{}'",
                    path.display()
                )
            }

            Self::OutsideProjectRoot { path } => {
                write!(
                    formatter,
                    "package input resolves outside the project root: '{}'",
                    path.display()
                )
            }

            Self::DuplicatePackagePath { path } => {
                write!(formatter, "duplicate package destination '{path}'")
            }

            Self::DuplicateSourcePath { path } => {
                write!(formatter, "duplicate package source '{path}'")
            }

            Self::PackagePathCollision { first, second } => {
                write!(
                    formatter,
                    "package destinations conflict: '{first}' and '{second}'"
                )
            }

            Self::EmptyPackage => {
                formatter.write_str("a package must contain at least one file")
            }

            Self::BudgetExceeded {
                budget,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "package creation budget '{budget}' exceeded: \
                     observed {actual}, maximum {maximum}"
                )
            }

            Self::SizeOverflow => {
                formatter.write_str("package content size accounting overflowed")
            }

            Self::FileChanged { package_path } => {
                write!(
                    formatter,
                    "file changed while packaging '{package_path}'"
                )
            }

            Self::DigestFailure { algorithm, source } => {
                write!(
                    formatter,
                    "content digest provider '{algorithm}' failed: {source}"
                )
            }

            Self::Cancelled => {
                formatter.write_str("package creation was cancelled")
            }

            Self::DuplicateDependency { identity } => {
                write!(
                    formatter,
                    "duplicate resolved dependency '{identity}'"
                )
            }
        }
    }
}

impl Error for PackageCreationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::DigestFailure { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Result type used by package creation.
pub type PackageCreationResult<T> =
    std::result::Result<T, PackageCreationError>;

// ============================================================================
// CANCELLATION
// ============================================================================

/// Cloneable cancellation handle for package-creation operations.
///
/// Cancellation is cooperative. It is checked between files and during
/// content hashing. It does not forcibly terminate operating-system I/O.
#[derive(Debug, Clone, Default)]
pub struct PackageCancellation {
    cancelled: Arc<AtomicBool>,
}

impl PackageCancellation {
    /// Create a new, non-cancelled handle.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Request cancellation.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Return whether cancellation has been requested.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    fn check(&self) -> PackageCreationResult<()> {
        if self.is_cancelled() {
            Err(PackageCreationError::Cancelled)
        } else {
            Ok(())
        }
    }
}

// ============================================================================
// CONTENT DIGESTS
// ============================================================================

/// An algorithm-qualified content digest.
///
/// The algorithm identifier is open-ended. Package creation does not
/// interpret arbitrary algorithm names or assume that all future package
/// formats use the same digest algorithm.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageContentDigest {
    algorithm: String,
    value: String,
}

impl PackageContentDigest {
    /// Construct an algorithm-qualified digest.
    pub fn new(
        algorithm: impl Into<String>,
        value: impl Into<String>,
    ) -> PackageCreationResult<Self> {
        let algorithm = algorithm.into();
        let value = value.into();

        validate_identifier(&algorithm, "digest algorithm")?;
        validate_nonempty_text(&value, "digest value")?;

        Ok(Self { algorithm, value })
    }

    /// Return the digest algorithm identifier.
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    /// Return the encoded digest value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// Provider interface for streaming content digests.
///
/// Implementations must consume the reader from its current position to EOF
/// and return a stable, canonical encoding of the resulting digest.
///
/// Providers must not silently truncate input.
pub trait ContentDigestProvider: Send + Sync {
    /// Return the stable algorithm identifier.
    fn algorithm_id(&self) -> &str;

    /// Calculate a digest by streaming the supplied reader.
    fn digest(
        &self,
        reader: &mut dyn Read,
    ) -> io::Result<String>;
}

/// SHA-256 content-digest provider.
///
/// SHA-256 is the initial default integrity algorithm. The provider interface
/// allows a future package-format contract to register other algorithms
/// without changing package-creation orchestration.
#[derive(Debug, Clone, Copy, Default)]
pub struct Sha256DigestProvider;

impl ContentDigestProvider for Sha256DigestProvider {
    fn algorithm_id(&self) -> &str {
        SHA256_ALGORITHM_ID
    }

    fn digest(
        &self,
        reader: &mut dyn Read,
    ) -> io::Result<String> {
        let mut sink = Sha256Sink(Sha256::new());

        io::copy(reader, &mut sink)?;

        Ok(format!("{:x}", sink.0.finalize()))
    }
}

struct Sha256Sink(Sha256);

impl Write for Sha256Sink {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.0.update(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// ============================================================================
// STREAM ACCOUNTING
// ============================================================================

struct CountingReader<'a> {
    inner: &'a mut dyn Read,
    bytes_read: u128,
    cancellation: &'a PackageCancellation,
}

impl<'a> CountingReader<'a> {
    fn new(
        inner: &'a mut dyn Read,
        cancellation: &'a PackageCancellation,
    ) -> Self {
        Self {
            inner,
            bytes_read: 0,
            cancellation,
        }
    }

    fn bytes_read(&self) -> u128 {
        self.bytes_read
    }
}

impl Read for CountingReader<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if self.cancellation.is_cancelled() {
            return Err(io::Error::new(
                io::ErrorKind::Interrupted,
                "package creation cancelled",
            ));
        }

        let read = self.inner.read(buffer)?;

        self.bytes_read = self
            .bytes_read
            .checked_add(read as u128)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "content byte count overflow",
                )
            })?;

        Ok(read)
    }
}

// ============================================================================
// PACKAGE IDENTITY INPUT
// ============================================================================

/// Source identity supplied by the source-resolution subsystem.
///
/// `locator` and `revision` are opaque identifiers. This module does not
/// canonicalize URLs, resolve Git references, access registries or verify
/// source authenticity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageSourceReference {
    kind: String,
    locator: String,
    revision: Option<String>,
}

impl PackageSourceReference {
    /// Construct a source reference from already-normalized source data.
    pub fn new(
        kind: impl Into<String>,
        locator: impl Into<String>,
        revision: Option<String>,
    ) -> PackageCreationResult<Self> {
        let kind = kind.into();
        let locator = locator.into();

        validate_identifier(&kind, "package source kind")?;
        validate_nonempty_text(&locator, "package source locator")?;

        if let Some(revision) = revision.as_deref() {
            validate_nonempty_text(revision, "package source revision")?;
        }

        Ok(Self {
            kind,
            locator,
            revision,
        })
    }

    /// Return the source kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Return the opaque source locator.
    #[must_use]
    pub fn locator(&self) -> &str {
        &self.locator
    }

    /// Return the immutable source revision, when supplied.
    #[must_use]
    pub fn revision(&self) -> Option<&str> {
        self.revision.as_deref()
    }
}

/// Stable logical package identity input.
///
/// A package name is a qualified source-level name, not a directory path.
/// The version scheme is extensible; this type does not assume SemVer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PackageIdentityInput {
    name: String,
    version_scheme: String,
    version: String,
    source: Option<PackageSourceReference>,
}

impl PackageIdentityInput {
    /// Construct a package identity from canonical name and version data.
    pub fn new(
        name: impl Into<String>,
        version_scheme: impl Into<String>,
        version: impl Into<String>,
        source: Option<PackageSourceReference>,
    ) -> PackageCreationResult<Self> {
        let name = name.into();
        let version_scheme = version_scheme.into();
        let version = version.into();

        validate_qualified_package_name(&name)?;
        validate_identifier(&version_scheme, "package version scheme")?;
        validate_nonempty_text(&version, "package version")?;

        Ok(Self {
            name,
            version_scheme,
            version,
            source,
        })
    }

    /// Return the canonical qualified package name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the version scheme identifier.
    #[must_use]
    pub fn version_scheme(&self) -> &str {
        &self.version_scheme
    }

    /// Return the exact version representation.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Return the source identity, when one is part of the coordinate.
    #[must_use]
    pub fn source(&self) -> Option<&PackageSourceReference> {
        self.source.as_ref()
    }
}

// ============================================================================
// STRUCTURED PACKAGE METADATA
// ============================================================================

/// Structured package metadata value.
///
/// This mirrors the structural nature of the canonical attribute grammar.
/// Numeric source spellings are retained as strings so this layer does not
/// impose numeric precision, width or representation assumptions.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PackageMetadataValue {
    /// String literal.
    String(String),

    /// Integer literal, preserving its source representation.
    Integer(String),

    /// Floating-point or decimal literal, preserving its source representation.
    Decimal(String),

    /// Character literal.
    Character(char),

    /// Boolean literal.
    Boolean(bool),

    /// Qualified symbolic name.
    Name(String),

    /// Ordered sequence of metadata values.
    List(Vec<PackageMetadataValue>),

    /// Deterministically ordered metadata object.
    Map(BTreeMap<String, PackageMetadataValue>),
}

impl PackageMetadataValue {
    /// Construct a metadata map while rejecting duplicate keys.
    pub fn map<I, K>(
        entries: I,
    ) -> PackageCreationResult<Self>
    where
        I: IntoIterator<Item = (K, PackageMetadataValue)>,
        K: Into<String>,
    {
        let mut values = BTreeMap::new();

        for (key, value) in entries {
            let key = key.into();

            validate_identifier(&key, "metadata map key")?;

            if values.insert(key.clone(), value).is_some() {
                return Err(PackageCreationError::InvalidField {
                    field: "metadata map".to_owned(),
                    reason: format!("duplicate metadata key '{key}'"),
                });
            }
        }

        Ok(Self::Map(values))
    }

    fn validate(&self, field: &str) -> PackageCreationResult<()> {
        match self {
            Self::String(value) => {
                if value.contains('\0') {
                    return Err(invalid_field(
                        field,
                        "string metadata contains a NUL character",
                    ));
                }
            }

            Self::Integer(value) | Self::Decimal(value) => {
                validate_nonempty_text(value, field)?;
            }

            Self::Character(value) => {
                if *value == '\0' {
                    return Err(invalid_field(
                        field,
                        "character metadata cannot be NUL",
                    ));
                }
            }

            Self::Boolean(_) => {}

            Self::Name(value) => {
                validate_nonempty_text(value, field)?;
            }

            Self::List(values) => {
                for (index, value) in values.iter().enumerate() {
                    value.validate(&format!("{field}[{index}]"))?;
                }
            }

            Self::Map(values) => {
                for (key, value) in values {
                    validate_identifier(key, "metadata map key")?;
                    value.validate(&format!("{field}.{key}"))?;
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// RESOLVED DEPENDENCY SNAPSHOTS
// ============================================================================

/// Immutable snapshot of a dependency already selected by the dependency
/// resolver.
///
/// This is not a dependency declaration and does not contain an unresolved
/// version constraint.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResolvedDependencySnapshot {
    name: String,
    version_scheme: String,
    version: String,
    source: Option<PackageSourceReference>,
    content_digest: PackageContentDigest,
    features: BTreeSet<String>,
}

impl ResolvedDependencySnapshot {
    /// Construct a resolved dependency snapshot.
    pub fn new(
        name: impl Into<String>,
        version_scheme: impl Into<String>,
        version: impl Into<String>,
        source: Option<PackageSourceReference>,
        content_digest: PackageContentDigest,
        features: BTreeSet<String>,
    ) -> PackageCreationResult<Self> {
        let name = name.into();
        let version_scheme = version_scheme.into();
        let version = version.into();

        validate_qualified_package_name(&name)?;
        validate_identifier(&version_scheme, "dependency version scheme")?;
        validate_nonempty_text(&version, "resolved dependency version")?;

        for feature in &features {
            validate_identifier(feature, "resolved dependency feature")?;
        }

        Ok(Self {
            name,
            version_scheme,
            version,
            source,
            content_digest,
            features,
        })
    }

    /// Return the resolved dependency name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the exact version scheme.
    #[must_use]
    pub fn version_scheme(&self) -> &str {
        &self.version_scheme
    }

    /// Return the resolved version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Return the selected source.
    #[must_use]
    pub fn source(&self) -> Option<&PackageSourceReference> {
        self.source.as_ref()
    }

    /// Return the verified or expected content digest supplied by the
    /// dependency subsystem.
    #[must_use]
    pub fn content_digest(&self) -> &PackageContentDigest {
        &self.content_digest
    }

    /// Return the selected feature set.
    #[must_use]
    pub fn features(&self) -> &BTreeSet<String> {
        &self.features
    }

    /// Return a stable textual identity for diagnostics.
    #[must_use]
    pub fn identity(&self) -> String {
        format!(
            "{}@{}:{}",
            self.name, self.version_scheme, self.version
        )
    }
}

// ============================================================================
// ARTIFACT COMPATIBILITY INPUT
// ============================================================================

/// Compatibility claim attached to a precompiled artifact.
///
/// The namespace and requirement are open-ended. Examples include target,
/// ABI, compiler, IR schema, runtime, operating environment, dependency ABI,
/// accelerator interface or a future compatibility contract.
///
/// This module does not interpret the claim or decide feasibility.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtifactCompatibilityClaim {
    namespace: String,
    requirement: String,
}

impl ArtifactCompatibilityClaim {
    /// Construct an opaque compatibility claim.
    pub fn new(
        namespace: impl Into<String>,
        requirement: impl Into<String>,
    ) -> PackageCreationResult<Self> {
        let namespace = namespace.into();
        let requirement = requirement.into();

        validate_identifier(&namespace, "artifact compatibility namespace")?;
        validate_nonempty_text(
            &requirement,
            "artifact compatibility requirement",
        )?;

        Ok(Self {
            namespace,
            requirement,
        })
    }

    /// Return the compatibility namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the exact compatibility requirement.
    #[must_use]
    pub fn requirement(&self) -> &str {
        &self.requirement
    }
}

/// Description of a precompiled artifact included in a package.
///
/// `format` and `portability_status` are open identifiers. No target list,
/// architecture list, ABI list, compiler list or domain list is hard-coded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageArtifactDescriptor {
    format: String,
    portability_status: String,
    compatibility: BTreeSet<ArtifactCompatibilityClaim>,
    domain_capabilities: BTreeSet<String>,
}

impl PackageArtifactDescriptor {
    /// Construct an artifact description.
    ///
    /// Portability is deliberately initialized to `unclassified`. An
    /// artifact without a target reference is not automatically portable.
    pub fn new(
        format: impl Into<String>,
    ) -> PackageCreationResult<Self> {
        let format = format.into();

        validate_identifier(&format, "artifact format")?;

        Ok(Self {
            format,
            portability_status: "unclassified".to_owned(),
            compatibility: BTreeSet::new(),
            domain_capabilities: BTreeSet::new(),
        })
    }

    /// Set an explicit portability classification supplied by the
    /// authoritative artifact or portability subsystem.
    pub fn with_portability_status(
        mut self,
        status: impl Into<String>,
    ) -> PackageCreationResult<Self> {
        let status = status.into();
        validate_identifier(&status, "artifact portability status")?;
        self.portability_status = status;
        Ok(self)
    }

    /// Add an artifact compatibility claim.
    pub fn with_compatibility(
        mut self,
        claim: ArtifactCompatibilityClaim,
    ) -> Self {
        self.compatibility.insert(claim);
        self
    }

    /// Add an open-ended domain capability identifier.
    pub fn with_domain_capability(
        mut self,
        capability: impl Into<String>,
    ) -> PackageCreationResult<Self> {
        let capability = capability.into();
        validate_identifier(&capability, "artifact domain capability")?;
        self.domain_capabilities.insert(capability);
        Ok(self)
    }

    /// Return the artifact format.
    #[must_use]
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Return the portability classification.
    #[must_use]
    pub fn portability_status(&self) -> &str {
        &self.portability_status
    }

    /// Return compatibility claims in deterministic order.
    #[must_use]
    pub fn compatibility(&self) -> &BTreeSet<ArtifactCompatibilityClaim> {
        &self.compatibility
    }

    /// Return domain capability identifiers.
    #[must_use]
    pub fn domain_capabilities(&self) -> &BTreeSet<String> {
        &self.domain_capabilities
    }
}

// ============================================================================
// PACKAGE PROVENANCE INPUT
// ============================================================================

/// Provenance information supplied to package creation.
///
/// The manifest digest is required. Source revisions and build fingerprints
/// are optional because source packages and precompiled artifacts may have
/// different provenance authorities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageProvenanceInput {
    manifest_digest: PackageContentDigest,
    source_revision: Option<String>,
    build_fingerprint: Option<String>,
}

impl PackageProvenanceInput {
    /// Construct package provenance from a canonical manifest digest.
    pub fn new(
        manifest_digest: PackageContentDigest,
        source_revision: Option<String>,
        build_fingerprint: Option<String>,
    ) -> PackageCreationResult<Self> {
        if let Some(revision) = source_revision.as_deref() {
            validate_nonempty_text(revision, "source revision")?;
        }

        if let Some(fingerprint) = build_fingerprint.as_deref() {
            validate_nonempty_text(fingerprint, "build fingerprint")?;
        }

        Ok(Self {
            manifest_digest,
            source_revision,
            build_fingerprint,
        })
    }

    /// Return the manifest digest.
    #[must_use]
    pub fn manifest_digest(&self) -> &PackageContentDigest {
        &self.manifest_digest
    }

    /// Return the source revision.
    #[must_use]
    pub fn source_revision(&self) -> Option<&str> {
        self.source_revision.as_deref()
    }

    /// Return the build fingerprint.
    #[must_use]
    pub fn build_fingerprint(&self) -> Option<&str> {
        self.build_fingerprint.as_deref()
    }
}

// ============================================================================
// PACKAGE FILE SELECTION
// ============================================================================

/// Explicitly selected file to include in a package.
///
/// Both paths must be relative and use portable forward-slash separators.
/// `source_path` is relative to the project root.
/// `package_path` is the path that will appear inside the package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageFileSpec {
    source_path: PathBuf,
    package_path: String,
    content_kind: String,
    artifact: Option<PackageArtifactDescriptor>,
}

impl PackageFileSpec {
    /// Construct a selected package file.
    pub fn new(
        source_path: impl Into<PathBuf>,
        package_path: impl Into<PathBuf>,
        content_kind: impl Into<String>,
    ) -> PackageCreationResult<Self> {
        let source_path = source_path.into();
        let package_path = package_path.into();
        let content_kind = content_kind.into();

        normalize_relative_path(&source_path, "source path")?;
        let package_path =
            normalize_relative_path(&package_path, "package path")?;

        validate_identifier(&content_kind, "package content kind")?;

        Ok(Self {
            source_path,
            package_path,
            content_kind,
            artifact: None,
        })
    }

    /// Construct a selected precompiled-artifact file.
    pub fn artifact(
        source_path: impl Into<PathBuf>,
        package_path: impl Into<PathBuf>,
        content_kind: impl Into<String>,
        artifact: PackageArtifactDescriptor,
    ) -> PackageCreationResult<Self> {
        let mut spec = Self::new(
            source_path,
            package_path,
            content_kind,
        )?;

        spec.artifact = Some(artifact);

        Ok(spec)
    }

    /// Return the project-relative source path.
    #[must_use]
    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    /// Return the package-relative destination path.
    #[must_use]
    pub fn package_path(&self) -> &str {
        &self.package_path
    }

    /// Return the open-ended content-kind identifier.
    #[must_use]
    pub fn content_kind(&self) -> &str {
        &self.content_kind
    }

    /// Return the optional precompiled-artifact description.
    #[must_use]
    pub fn artifact(&self) -> Option<&PackageArtifactDescriptor> {
        self.artifact.as_ref()
    }
}

// ============================================================================
// OPERATIONAL BUDGETS
// ============================================================================

/// Optional operational budgets for one package-creation operation.
///
/// `None` means that this module does not impose a limit for that budget.
/// Callers should provide limits appropriate to their environment and
/// security policy.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PackageCreationPolicy {
    max_file_count: Option<usize>,
    max_file_bytes: Option<u128>,
    max_total_bytes: Option<u128>,
}

impl PackageCreationPolicy {
    /// Construct an unrestricted policy.
    ///
    /// This means no additional package-creation budget is imposed by this
    /// policy. It does not override operating-system or storage limits.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_file_count: None,
            max_file_bytes: None,
            max_total_bytes: None,
        }
    }

    /// Set the maximum number of explicitly selected files.
    #[must_use]
    pub const fn with_max_file_count(
        mut self,
        maximum: Option<usize>,
    ) -> Self {
        self.max_file_count = maximum;
        self
    }

    /// Set the maximum size of an individual file, in bytes.
    #[must_use]
    pub const fn with_max_file_bytes(
        mut self,
        maximum: Option<u128>,
    ) -> Self {
        self.max_file_bytes = maximum;
        self
    }

    /// Set the maximum aggregate file size, in bytes.
    #[must_use]
    pub const fn with_max_total_bytes(
        mut self,
        maximum: Option<u128>,
    ) -> Self {
        self.max_total_bytes = maximum;
        self
    }

    /// Return the configured file-count budget.
    #[must_use]
    pub const fn max_file_count(&self) -> Option<usize> {
        self.max_file_count
    }

    /// Return the configured per-file byte budget.
    #[must_use]
    pub const fn max_file_bytes(&self) -> Option<u128> {
        self.max_file_bytes
    }

    /// Return the configured aggregate byte budget.
    #[must_use]
    pub const fn max_total_bytes(&self) -> Option<u128> {
        self.max_total_bytes
    }
}

// ============================================================================
// PACKAGE CREATION REQUEST
// ============================================================================

/// Complete input to one package-creation operation.
///
/// The request contains explicit file selections. It does not contain
/// unresolved dependency declarations. The dependency subsystem must resolve
/// those declarations before constructing this request.
#[derive(Debug, Clone)]
pub struct PackageCreationRequest {
    project_root: PathBuf,
    identity: PackageIdentityInput,
    package_kind: String,
    metadata: BTreeMap<String, PackageMetadataValue>,
    files: Vec<PackageFileSpec>,
    dependencies: BTreeSet<ResolvedDependencySnapshot>,
    provenance: PackageProvenanceInput,
    policy: PackageCreationPolicy,
}

impl PackageCreationRequest {
    /// Construct a package-creation request.
    pub fn new(
        project_root: impl Into<PathBuf>,
        identity: PackageIdentityInput,
        package_kind: impl Into<String>,
        provenance: PackageProvenanceInput,
    ) -> PackageCreationResult<Self> {
        let project_root = project_root.into();
        let package_kind = package_kind.into();

        if project_root.as_os_str().is_empty() {
            return Err(invalid_field(
                "project root",
                "project root must not be empty",
            ));
        }

        validate_identifier(&package_kind, "package kind")?;

        Ok(Self {
            project_root,
            identity,
            package_kind,
            metadata: BTreeMap::new(),
            files: Vec::new(),
            dependencies: BTreeSet::new(),
            provenance,
            policy: PackageCreationPolicy::unrestricted(),
        })
    }

    /// Add one structured package metadata field.
    ///
    /// Duplicate keys are rejected.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: PackageMetadataValue,
    ) -> PackageCreationResult<Self> {
        let key = key.into();

        validate_identifier(&key, "package metadata key")?;
        value.validate(&format!("package metadata '{key}'"))?;

        if self.metadata.contains_key(&key) {
            return Err(invalid_field(
                "package metadata",
                format!("duplicate metadata key '{key}'"),
            ));
        }

        self.metadata.insert(key, value);

        Ok(self)
    }

    /// Add one explicitly selected file.
    ///
    /// No wildcard, directory-recursion or implicit inclusion behavior is
    /// provided. Each included file must be named explicitly.
    pub fn include_file(
        mut self,
        file: PackageFileSpec,
    ) -> PackageCreationResult<Self> {
        normalize_relative_path(&file.source_path, "source path")?;
        normalize_relative_path(
            Path::new(&file.package_path),
            "package path",
        )?;

        if self.files.iter().any(|existing| {
            existing.package_path == file.package_path
        }) {
            return Err(PackageCreationError::DuplicatePackagePath {
                path: file.package_path,
            });
        }

        if self.files.iter().any(|existing| {
            existing.source_path == file.source_path
        }) {
            return Err(PackageCreationError::DuplicateSourcePath {
                path: file.source_path.display().to_string(),
            });
        }

        self.files.push(file);

        Ok(self)
    }

    /// Add a dependency already resolved by the dependency subsystem.
    ///
    /// This operation does not perform dependency resolution.
    pub fn with_resolved_dependency(
        mut self,
        dependency: ResolvedDependencySnapshot,
    ) -> PackageCreationResult<Self> {
        let identity = dependency.identity();

        if !self.dependencies.insert(dependency) {
            return Err(PackageCreationError::DuplicateDependency {
                identity,
            });
        }

        Ok(self)
    }

    /// Set caller-supplied operational budgets.
    #[must_use]
    pub fn with_policy(
        mut self,
        policy: PackageCreationPolicy,
    ) -> Self {
        self.policy = policy;
        self
    }

    /// Return the package identity input.
    #[must_use]
    pub fn identity(&self) -> &PackageIdentityInput {
        &self.identity
    }

    /// Return the selected package kind.
    #[must_use]
    pub fn package_kind(&self) -> &str {
        &self.package_kind
    }

    /// Return the explicitly selected files.
    #[must_use]
    pub fn files(&self) -> &[PackageFileSpec] {
        &self.files
    }

    /// Return the resolved dependency snapshots.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeSet<ResolvedDependencySnapshot> {
        &self.dependencies
    }

    /// Return the structured metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, PackageMetadataValue> {
        &self.metadata
    }
}

// ============================================================================
// CREATION MANIFEST
// ============================================================================

/// One package-relative file record.
///
/// This record contains no host-specific absolute source path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageFileRecord {
    package_path: String,
    content_kind: String,
    byte_size: u128,
    digest: PackageContentDigest,
    artifact: Option<PackageArtifactDescriptor>,
}

impl PackageFileRecord {
    /// Return the package-relative path.
    #[must_use]
    pub fn package_path(&self) -> &str {
        &self.package_path
    }

    /// Return the content-kind identifier.
    #[must_use]
    pub fn content_kind(&self) -> &str {
        &self.content_kind
    }

    /// Return the measured file size.
    #[must_use]
    pub const fn byte_size(&self) -> u128 {
        self.byte_size
    }

    /// Return the expected content digest.
    #[must_use]
    pub fn digest(&self) -> &PackageContentDigest {
        &self.digest
    }

    /// Return the optional artifact description.
    #[must_use]
    pub fn artifact(&self) -> Option<&PackageArtifactDescriptor> {
        self.artifact.as_ref()
    }
}

/// Deterministic package-creation manifest.
///
/// This is the hand-off contract to `packages::model` and `packages::archive`.
/// It is not itself a serialized `.zpkg` archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageDraftManifest {
    identity: PackageIdentityInput,
    package_kind: String,
    metadata: BTreeMap<String, PackageMetadataValue>,
    files: Vec<PackageFileRecord>,
    dependencies: BTreeSet<ResolvedDependencySnapshot>,
    provenance: PackageProvenanceInput,
    total_bytes: u128,
}

impl PackageDraftManifest {
    /// Return the package identity.
    #[must_use]
    pub fn identity(&self) -> &PackageIdentityInput {
        &self.identity
    }

    /// Return the package kind.
    #[must_use]
    pub fn package_kind(&self) -> &str {
        &self.package_kind
    }

    /// Return structured package metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, PackageMetadataValue> {
        &self.metadata
    }

    /// Return package file records in deterministic destination-path order.
    #[must_use]
    pub fn files(&self) -> &[PackageFileRecord] {
        &self.files
    }

    /// Return resolved dependency snapshots in deterministic order.
    #[must_use]
    pub fn dependencies(&self) -> &BTreeSet<ResolvedDependencySnapshot> {
        &self.dependencies
    }

    /// Return package provenance input.
    #[must_use]
    pub fn provenance(&self) -> &PackageProvenanceInput {
        &self.provenance
    }

    /// Return the aggregate uncompressed content size.
    #[must_use]
    pub const fn total_bytes(&self) -> u128 {
        self.total_bytes
    }
}

// ============================================================================
// OPEN PAYLOADS
// ============================================================================

/// Open payload for a selected package file.
///
/// The file handle is retained so the archive writer does not have to reopen
/// an untrusted project-relative path.
///
/// The archive writer must rewind the handle before reading and must verify
/// the final digest against the corresponding [`PackageFileRecord`].
pub struct PackagePayload {
    package_path: String,
    source_path: PathBuf,
    file: File,
}

impl fmt::Debug for PackagePayload {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PackagePayload")
            .field("package_path", &self.package_path)
            .field("source_path", &self.source_path)
            .finish_non_exhaustive()
    }
}

impl PackagePayload {
    /// Return the package-relative destination path.
    #[must_use]
    pub fn package_path(&self) -> &str {
        &self.package_path
    }

    /// Return the canonical source path for diagnostics only.
    ///
    /// This host-specific path must not be serialized into the package.
    #[must_use]
    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    /// Rewind the open file handle to its beginning.
    pub fn rewind(&mut self) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(0)).map(|_| ())
    }

    /// Borrow the open file handle as a reader.
    ///
    /// The caller must rewind it before consuming the payload.
    pub fn reader_mut(&mut self) -> &mut File {
        &mut self.file
    }
}

/// Completed package-creation snapshot.
///
/// The archive writer consumes this value to serialize the package. The
/// package model consumes the manifest to construct the canonical package
/// object. Neither consumer should independently rediscover project files.
pub struct PackageDraft {
    manifest: PackageDraftManifest,
    payloads: Vec<PackagePayload>,
}

impl fmt::Debug for PackageDraft {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PackageDraft")
            .field("manifest", &self.manifest)
            .field("payload_count", &self.payloads.len())
            .finish()
    }
}

impl PackageDraft {
    /// Borrow the deterministic creation manifest.
    #[must_use]
    pub fn manifest(&self) -> &PackageDraftManifest {
        &self.manifest
    }

    /// Borrow payloads mutably for archive serialization.
    ///
    /// Payload order matches `manifest().files()` order.
    pub fn payloads_mut(&mut self) -> &mut [PackagePayload] {
        &mut self.payloads
    }

    /// Consume the draft and return its manifest and payload handles.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (PackageDraftManifest, Vec<PackagePayload>) {
        (self.manifest, self.payloads)
    }
}

// ============================================================================
// PACKAGE CREATOR
// ============================================================================

/// Stateless package creator.
///
/// The digest provider is injected so the package subsystem can evolve its
/// integrity algorithms without changing file-selection or project-boundary
/// logic.
pub struct PackageCreator {
    digest_provider: Arc<dyn ContentDigestProvider>,
}

impl fmt::Debug for PackageCreator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PackageCreator")
            .field(
                "digest_algorithm",
                &self.digest_provider.algorithm_id(),
            )
            .finish()
    }
}

impl Default for PackageCreator {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageCreator {
    /// Construct a creator using the initial SHA-256 digest provider.
    #[must_use]
    pub fn new() -> Self {
        Self {
            digest_provider: Arc::new(Sha256DigestProvider),
        }
    }

    /// Construct a creator using an explicitly supplied digest provider.
    #[must_use]
    pub fn with_digest_provider(
        digest_provider: Arc<dyn ContentDigestProvider>,
    ) -> Self {
        Self { digest_provider }
    }

    /// Create a deterministic package snapshot from an explicit request.
    ///
    /// This operation:
    ///
    /// 1. Validates the request and caller-supplied budgets.
    /// 2. Canonicalizes the project root.
    /// 3. Sorts selected entries by package destination.
    /// 4. Rejects duplicate paths and file/directory path collisions.
    /// 5. Rejects symbolic links in selected paths.
    /// 6. Verifies that each selected file is inside the project root.
    /// 7. Opens each regular file.
    /// 8. Calculates its digest incrementally.
    /// 9. Checks file stability and byte budgets.
    /// 10. Retains the open file handle for the archive writer.
    ///
    /// It does not serialize the package or claim that its dependencies,
    /// artifacts, signatures or target compatibility are valid. Those
    /// checks belong to their authoritative subsystems.
    pub fn create(
        &self,
        mut request: PackageCreationRequest,
        cancellation: &PackageCancellation,
    ) -> PackageCreationResult<PackageDraft> {
        cancellation.check()?;

        validate_request(&request)?;

        let root = fs::canonicalize(&request.project_root).map_err(|source| {
            PackageCreationError::Io {
                operation: OPERATION_CREATE,
                path: request.project_root.clone(),
                source,
            }
        })?;

        let root_metadata = fs::metadata(&root).map_err(|source| {
            PackageCreationError::Io {
                operation: OPERATION_VALIDATE,
                path: root.clone(),
                source,
            }
        })?;

        if !root_metadata.is_dir() {
            return Err(invalid_field(
                "project root",
                "project root must resolve to a directory",
            ));
        }

        enforce_file_count_budget(
            request.files.len(),
            request.policy.max_file_count,
        )?;

        request.files.sort_by(|left, right| {
            left.package_path.cmp(&right.package_path)
        });

        validate_destination_paths(&request.files)?;

        let mut records = Vec::with_capacity(request.files.len());
        let mut payloads = Vec::with_capacity(request.files.len());
        let mut total_bytes = 0_u128;

        for spec in request.files {
            cancellation.check()?;

            let normalized_source =
                normalize_relative_path(&spec.source_path, "source path")?;

            let (canonical_path, mut file) =
                open_selected_file(&root, &normalized_source)?;

            let before = file.metadata().map_err(|source| {
                PackageCreationError::Io {
                    operation: OPERATION_VALIDATE,
                    path: canonical_path.clone(),
                    source,
                }
            })?;

            if !before.is_file() {
                return Err(PackageCreationError::NotRegularFile {
                    path: canonical_path,
                });
            }

            enforce_file_size_budget(
                before.len() as u128,
                request.policy.max_file_bytes,
            )?;

            let digest = self.digest_file(
                &mut file,
                cancellation,
            )?;

            let after = file.metadata().map_err(|source| {
                PackageCreationError::Io {
                    operation: OPERATION_VALIDATE,
                    path: canonical_path.clone(),
                    source,
                }
            })?;

            if !file_metadata_stable(&before, &after) {
                return Err(PackageCreationError::FileChanged {
                    package_path: spec.package_path.clone(),
                });
            }

            let measured_size = digest.1;

            enforce_file_size_budget(
                measured_size,
                request.policy.max_file_bytes,
            )?;

            total_bytes = total_bytes
                .checked_add(measured_size)
                .ok_or(PackageCreationError::SizeOverflow)?;

            if let Some(maximum) = request.policy.max_total_bytes {
                if total_bytes > maximum {
                    return Err(PackageCreationError::BudgetExceeded {
                        budget: "total package bytes",
                        actual: total_bytes,
                        maximum,
                    });
                }
            }

            file.seek(SeekFrom::Start(0)).map_err(|source| {
                PackageCreationError::Io {
                    operation: "rewind package input",
                    path: canonical_path.clone(),
                    source,
                }
            })?;

            let content_digest = PackageContentDigest::new(
                self.digest_provider.algorithm_id(),
                digest.0,
            )?;

            records.push(PackageFileRecord {
                package_path: spec.package_path.clone(),
                content_kind: spec.content_kind.clone(),
                byte_size: measured_size,
                digest: content_digest,
                artifact: spec.artifact.clone(),
            });

            payloads.push(PackagePayload {
                package_path: spec.package_path,
                source_path: canonical_path,
                file,
            });
        }

        cancellation.check()?;

        if records.is_empty() {
            return Err(PackageCreationError::EmptyPackage);
        }

        let manifest = PackageDraftManifest {
            identity: request.identity,
            package_kind: request.package_kind,
            metadata: request.metadata,
            files: records,
            dependencies: request.dependencies,
            provenance: request.provenance,
            total_bytes,
        };

        Ok(PackageDraft {
            manifest,
            payloads,
        })
    }

    fn digest_file(
        &self,
        file: &mut File,
        cancellation: &PackageCancellation,
    ) -> PackageCreationResult<(String, u128)> {
        file.seek(SeekFrom::Start(0)).map_err(|source| {
            PackageCreationError::Io {
                operation: "rewind before content hashing",
                path: PathBuf::new(),
                source,
            }
        })?;

        let mut reader = CountingReader::new(file, cancellation);

        let digest = self
            .digest_provider
            .digest(&mut reader)
            .map_err(|source| {
                if source.kind() == io::ErrorKind::Interrupted
                    && cancellation.is_cancelled()
                {
                    PackageCreationError::Cancelled
                } else {
                    PackageCreationError::DigestFailure {
                        algorithm: self.digest_provider.algorithm_id().to_owned(),
                        source,
                    }
                }
            })?;

        Ok((digest, reader.bytes_read()))
    }
}

// ============================================================================
// REQUEST AND PATH VALIDATION
// ============================================================================

fn validate_request(
    request: &PackageCreationRequest,
) -> PackageCreationResult<()> {
    validate_qualified_package_name(request.identity.name())?;
    validate_identifier(
        request.identity.version_scheme(),
        "package version scheme",
    )?;
    validate_nonempty_text(
        request.identity.version(),
        "package version",
    )?;
    validate_identifier(request.package_kind(), "package kind")?;

    for (key, value) in &request.metadata {
        validate_identifier(key, "package metadata key")?;
        value.validate(&format!("package metadata '{key}'"))?;
    }

    for dependency in &request.dependencies {
        validate_qualified_package_name(dependency.name())?;
        validate_identifier(
            dependency.version_scheme(),
            "dependency version scheme",
        )?;
        validate_nonempty_text(
            dependency.version(),
            "dependency version",
        )?;
    }

    for file in &request.files {
        normalize_relative_path(&file.source_path, "source path")?;
        normalize_relative_path(
            Path::new(&file.package_path),
            "package path",
        )?;
        validate_identifier(&file.content_kind, "package content kind")?;
    }

    Ok(())
}

fn normalize_relative_path(
    path: &Path,
    field: &str,
) -> PackageCreationResult<String> {
    if path.as_os_str().is_empty() {
        return Err(invalid_field(field, "path must not be empty"));
    }

    if path.is_absolute() {
        return Err(invalid_field(
            field,
            "absolute paths are not permitted",
        ));
    }

    let raw = path.to_str().ok_or_else(|| {
        invalid_field(field, "path must be valid UTF-8")
    })?;

    if raw.contains('\\') {
        return Err(invalid_field(
            field,
            "portable paths must use forward slashes",
        ));
    }

    if raw.contains(':') {
        return Err(invalid_field(
            field,
            "colon characters are not permitted in portable paths",
        ));
    }

    if raw.contains('\0') || raw.chars().any(char::is_control) {
        return Err(invalid_field(
            field,
            "path contains a control character",
        ));
    }

    let mut normalized = Vec::new();

    for component in path.components() {
        match component {
            Component::Normal(value) => {
                let value = value.to_str().ok_or_else(|| {
                    invalid_field(field, "path component is not valid UTF-8")
                })?;

                if value.is_empty() || value == "." || value == ".." {
                    return Err(invalid_field(
                        field,
                        "path contains an invalid component",
                    ));
                }

                normalized.push(value);
            }

            Component::CurDir
            | Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => {
                return Err(invalid_field(
                    field,
                    "path must contain only normal relative components",
                ));
            }
        }
    }

    if normalized.is_empty() {
        return Err(invalid_field(
            field,
            "path must identify a file",
        ));
    }

    let result = normalized.join("/");

    if result.split('/').any(str::is_empty) {
        return Err(invalid_field(
            field,
            "path contains an empty component",
        ));
    }

    Ok(result)
}

fn open_selected_file(
    root: &Path,
    normalized_source: &str,
) -> PackageCreationResult<(PathBuf, File)> {
    let mut current = root.to_path_buf();
    let components: Vec<&str> = normalized_source.split('/').collect();

    for (index, component) in components.iter().enumerate() {
        current.push(component);

        let metadata =
            fs::symlink_metadata(&current).map_err(|source| {
                PackageCreationError::Io {
                    operation: "inspect selected package input",
                    path: current.clone(),
                    source,
                }
            })?;

        if metadata.file_type().is_symlink() {
            return Err(PackageCreationError::SymbolicLink {
                path: current,
            });
        }

        let is_final = index + 1 == components.len();

        if is_final && !metadata.is_file() {
            return Err(PackageCreationError::NotRegularFile {
                path: current,
            });
        }

        if !is_final && !metadata.is_dir() {
            return Err(PackageCreationError::NotRegularFile {
                path: current,
            });
        }
    }

    let canonical = fs::canonicalize(&current).map_err(|source| {
        PackageCreationError::Io {
            operation: "canonicalize selected package input",
            path: current.clone(),
            source,
        }
    })?;

    if !canonical.starts_with(root) {
        return Err(PackageCreationError::OutsideProjectRoot {
            path: canonical,
        });
    }

    let file = File::open(&canonical).map_err(|source| {
        PackageCreationError::Io {
            operation: "open selected package input",
            path: canonical.clone(),
            source,
        }
    })?;

    let metadata = file.metadata().map_err(|source| {
        PackageCreationError::Io {
            operation: "inspect opened package input",
            path: canonical.clone(),
            source,
        }
    })?;

    if !metadata.is_file() {
        return Err(PackageCreationError::NotRegularFile {
            path: canonical,
        });
    }

    Ok((canonical, file))
}

fn validate_destination_paths(
    files: &[PackageFileSpec],
) -> PackageCreationResult<()> {
    let mut destinations = BTreeSet::new();
    let mut sources = BTreeSet::new();

    for file in files {
        let destination =
            normalize_relative_path(Path::new(&file.package_path), "package path")?;

        if !destinations.insert(destination.clone()) {
            return Err(PackageCreationError::DuplicatePackagePath {
                path: destination,
            });
        }

        let source =
            normalize_relative_path(&file.source_path, "source path")?;

        if !sources.insert(source.clone()) {
            return Err(PackageCreationError::DuplicateSourcePath {
                path: source,
            });
        }
    }

    let ordered: Vec<&str> = destinations.iter().map(String::as_str).collect();

    for pair in ordered.windows(2) {
        let first = pair[0];
        let second = pair[1];

        if second.starts_with(first)
            && second.as_bytes().get(first.len()) == Some(&b'/')
        {
            return Err(PackageCreationError::PackagePathCollision {
                first: first.to_owned(),
                second: second.to_owned(),
            });
        }
    }

    Ok(())
}

fn file_metadata_stable(
    before: &Metadata,
    after: &Metadata,
) -> bool {
    if before.len() != after.len() {
        return false;
    }

    match (before.modified(), after.modified()) {
        (Ok(before), Ok(after)) => before == after,
        _ => true,
    }
}

fn enforce_file_count_budget(
    actual: usize,
    maximum: Option<usize>,
) -> PackageCreationResult<()> {
    if let Some(maximum) = maximum {
        if actual > maximum {
            return Err(PackageCreationError::BudgetExceeded {
                budget: "package file count",
                actual: actual as u128,
                maximum: maximum as u128,
            });
        }
    }

    Ok(())
}

fn enforce_file_size_budget(
    actual: u128,
    maximum: Option<u128>,
) -> PackageCreationResult<()> {
    if let Some(maximum) = maximum {
        if actual > maximum {
            return Err(PackageCreationError::BudgetExceeded {
                budget: "individual file bytes",
                actual,
                maximum,
            });
        }
    }

    Ok(())
}

// ============================================================================
// TEXT VALIDATION
// ============================================================================

fn validate_nonempty_text(
    value: &str,
    field: &str,
) -> PackageCreationResult<()> {
    if value.is_empty() {
        return Err(invalid_field(field, "value must not be empty"));
    }

    if value.trim() != value {
        return Err(invalid_field(
            field,
            "surrounding whitespace is not permitted",
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_field(
            field,
            "control characters are not permitted",
        ));
    }

    Ok(())
}

fn validate_identifier(
    value: &str,
    field: &str,
) -> PackageCreationResult<()> {
    validate_nonempty_text(value, field)?;

    if value.chars().any(char::is_whitespace) {
        return Err(invalid_field(
            field,
            "identifier must not contain whitespace",
        ));
    }

    Ok(())
}

fn validate_qualified_package_name(
    value: &str,
) -> PackageCreationResult<()> {
    validate_identifier(value, "package name")?;

    if value.starts_with("::")
        || value.ends_with("::")
        || value.contains("::::")
    {
        return Err(invalid_field(
            "package name",
            "malformed qualified package name",
        ));
    }

    if value.contains(':') {
        for segment in value.split("::") {
            if segment.is_empty() {
                return Err(invalid_field(
                    "package name",
                    "qualified name contains an empty segment",
                ));
            }
        }

        if value.replace("::", "").contains(':') {
            return Err(invalid_field(
                "package name",
                "single colon is not a qualified-name separator",
            ));
        }
    }

    Ok(())
}

fn invalid_field(
    field: impl Into<String>,
    reason: impl Into<String>,
) -> PackageCreationError {
    PackageCreationError::InvalidField {
        field: field.into(),
        reason: reason.into(),
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEST_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new() -> Self {
            let sequence =
                NEXT_TEST_DIRECTORY.fetch_add(1, Ordering::Relaxed);

            let path = std::env::temp_dir().join(format!(
                "zamani-package-creation-{}-{sequence}",
                std::process::id()
            ));

            fs::create_dir_all(&path).expect("create test directory");

            Self { path }
        }

        fn write(
            &self,
            relative: &str,
            bytes: &[u8],
        ) {
            let path = self.path.join(relative);

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("create parent");
            }

            fs::write(path, bytes).expect("write test file");
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn digest(value: &str) -> PackageContentDigest {
        PackageContentDigest::new("sha-256", value)
            .expect("valid digest")
    }

    fn provenance() -> PackageProvenanceInput {
        PackageProvenanceInput::new(
            digest("manifest-fingerprint"),
            Some("revision-1".to_owned()),
            None,
        )
        .expect("valid provenance")
    }

    fn identity() -> PackageIdentityInput {
        PackageIdentityInput::new(
            "org::zamani::example",
            "semver",
            "1.0.0",
            None,
        )
        .expect("valid identity")
    }

    fn request(root: &Path) -> PackageCreationRequest {
        PackageCreationRequest::new(
            root,
            identity(),
            "source",
            provenance(),
        )
        .expect("valid request")
    }

    #[test]
    fn package_name_is_not_treated_as_a_filesystem_path() {
        let identity = identity();

        assert_eq!(
            identity.name(),
            "org::zamani::example"
        );
    }

    #[test]
    fn arbitrary_version_schemes_are_preserved() {
        let identity = PackageIdentityInput::new(
            "org::zamani::example",
            "vendor.release.channel",
            "2026-r4-preview",
            None,
        )
        .expect("valid identity");

        assert_eq!(
            identity.version_scheme(),
            "vendor.release.channel"
        );

        assert_eq!(
            identity.version(),
            "2026-r4-preview"
        );
    }

    #[test]
    fn metadata_preserves_nested_structure() {
        let nested = PackageMetadataValue::map([
            (
                "domain",
                PackageMetadataValue::Name("quantum".to_owned()),
            ),
            (
                "features",
                PackageMetadataValue::List(vec![
                    PackageMetadataValue::String("simulation".to_owned()),
                    PackageMetadataValue::String("measurement".to_owned()),
                ]),
            ),
        ])
        .expect("valid nested metadata");

        let request = request(Path::new("."))
            .with_metadata("capabilities", nested)
            .expect("metadata accepted");

        assert!(request.metadata().contains_key("capabilities"));
    }

    #[test]
    fn duplicate_metadata_keys_are_rejected() {
        let request = request(Path::new("."))
            .with_metadata(
                "license",
                PackageMetadataValue::String("MIT".to_owned()),
            )
            .expect("first metadata");

        let result = request.with_metadata(
            "license",
            PackageMetadataValue::String("Apache-2.0".to_owned()),
        );

        assert!(result.is_err());
    }

    #[test]
    fn path_traversal_is_rejected() {
        assert!(
            normalize_relative_path(
                Path::new("../outside.txt"),
                "package path"
            )
            .is_err()
        );

        assert!(
            normalize_relative_path(
                Path::new("safe/../../outside.txt"),
                "package path"
            )
            .is_err()
        );
    }

    #[test]
    fn absolute_paths_are_rejected() {
        assert!(
            normalize_relative_path(
                Path::new("/etc/passwd"),
                "package path"
            )
            .is_err()
        );
    }

    #[test]
    fn backslash_paths_are_rejected() {
        assert!(
            normalize_relative_path(
                Path::new("folder\\file.zm"),
                "package path"
            )
            .is_err()
        );
    }

    #[test]
    fn duplicate_package_destinations_are_rejected() {
        let first = PackageFileSpec::new(
            "src/one.zm",
            "src/one.zm",
            "source",
        )
        .expect("first file");

        let second = PackageFileSpec::new(
            "src/two.zm",
            "src/one.zm",
            "source",
        )
        .expect("second file");

        let result = request(Path::new("."))
            .include_file(first)
            .expect("first inclusion")
            .include_file(second);

        assert!(matches!(
            result,
            Err(PackageCreationError::DuplicatePackagePath { .. })
        ));
    }

    #[test]
    fn file_directory_destination_collisions_are_rejected() {
        let first = PackageFileSpec::new(
            "one.zm",
            "assets",
            "source",
        )
        .expect("first file");

        let second = PackageFileSpec::new(
            "two.zm",
            "assets/icon.bin",
            "resource",
        )
        .expect("second file");

        let result = validate_destination_paths(&[first, second]);

        assert!(matches!(
            result,
            Err(PackageCreationError::PackagePathCollision { .. })
        ));
    }

    #[test]
    fn unselected_project_files_are_not_included() {
        let directory = TestDirectory::new();

        directory.write("src/main.zm", b"fn main() {}");
        directory.write("secret/private.key", b"not selected");

        let file = PackageFileSpec::new(
            "src/main.zm",
            "src/main.zm",
            "source",
        )
        .expect("valid file selection");

        let request = request(&directory.path)
            .include_file(file)
            .expect("include selected file");

        let draft = PackageCreator::new()
            .create(&request, &PackageCancellation::new())
            .expect("create package");

        assert_eq!(draft.manifest().files().len(), 1);
        assert_eq!(
            draft.manifest().files()[0].package_path(),
            "src/main.zm"
        );
    }

    #[test]
    fn package_creation_hashes_files_without_loading_them_as_a_single_buffer() {
        let directory = TestDirectory::new();

        directory.write(
            "src/main.zm",
            b"package org::zamani::example;",
        );

        let file = PackageFileSpec::new(
            "src/main.zm",
            "src/main.zm",
            "source",
        )
        .expect("valid file selection");

        let draft = PackageCreator::new()
            .create(
                &request(&directory.path)
                    .include_file(file)
                    .expect("include file"),
                &PackageCancellation::new(),
            )
            .expect("create package");

        let record = &draft.manifest().files()[0];

        assert_eq!(
            record.digest().algorithm(),
            SHA256_ALGORITHM_ID
        );

        assert_eq!(
            record.byte_size(),
            b"package org::zamani::example;".len() as u128
        );
    }

    #[test]
    fn package_entries_are_sorted_deterministically() {
        let directory = TestDirectory::new();

        directory.write("z.zm", b"z");
        directory.write("a.zm", b"a");

        let request = request(&directory.path)
            .include_file(
                PackageFileSpec::new("z.zm", "z.zm", "source")
                    .expect("z file"),
            )
            .expect("include z")
            .include_file(
                PackageFileSpec::new("a.zm", "a.zm", "source")
                    .expect("a file"),
            )
            .expect("include a");

        let draft = PackageCreator::new()
            .create(&request, &PackageCancellation::new())
            .expect("create package");

        let paths: Vec<&str> = draft
            .manifest()
            .files()
            .iter()
            .map(PackageFileRecord::package_path)
            .collect();

        assert_eq!(paths, vec!["a.zm", "z.zm"]);
    }

    #[test]
    fn empty_package_is_rejected() {
        let directory = TestDirectory::new();

        let result = PackageCreator::new()
            .create(
                request(&directory.path),
                &PackageCancellation::new(),
            );

        assert!(matches!(
            result,
            Err(PackageCreationError::EmptyPackage)
        ));
    }

    #[test]
    fn configured_file_count_budget_is_enforced() {
        let directory = TestDirectory::new();

        directory.write("a.zm", b"a");
        directory.write("b.zm", b"b");

        let request = request(&directory.path)
            .include_file(
                PackageFileSpec::new("a.zm", "a.zm", "source")
                    .expect("a file"),
            )
            .expect("include a")
            .include_file(
                PackageFileSpec::new("b.zm", "b.zm", "source")
                    .expect("b file"),
            )
            .expect("include b")
            .with_policy(
                PackageCreationPolicy::unrestricted()
                    .with_max_file_count(Some(1)),
            );

        let result = PackageCreator::new()
            .create(&request, &PackageCancellation::new());

        assert!(matches!(
            result,
            Err(PackageCreationError::BudgetExceeded {
                budget: "package file count",
                ..
            })
        ));
    }

    #[test]
    fn configured_total_size_budget_is_enforced() {
        let directory = TestDirectory::new();

        directory.write("a.zm", b"1234");
        directory.write("b.zm", b"5678");

        let request = request(&directory.path)
            .include_file(
                PackageFileSpec::new("a.zm", "a.zm", "source")
                    .expect("a file"),
            )
            .expect("include a")
            .include_file(
                PackageFileSpec::new("b.zm", "b.zm", "source")
                    .expect("b file"),
            )
            .expect("include b")
            .with_policy(
                PackageCreationPolicy::unrestricted()
                    .with_max_total_bytes(Some(7)),
            );

        let result = PackageCreator::new()
            .create(&request, &PackageCancellation::new());

        assert!(matches!(
            result,
            Err(PackageCreationError::BudgetExceeded {
                budget: "total package bytes",
                ..
            })
        ));
    }

    #[test]
    fn precompiled_artifact_metadata_is_extensible() {
        let artifact = PackageArtifactDescriptor::new(
            "vendor.quantum.package",
        )
        .expect("format")
        .with_compatibility(
            ArtifactCompatibilityClaim::new(
                "target.abi",
                "vendor-abi-2026",
            )
            .expect("ABI claim"),
        )
        .with_compatibility(
            ArtifactCompatibilityClaim::new(
                "compiler",
                "compiler-family>=4",
            )
            .expect("compiler claim"),
        )
        .with_domain_capability("quantum.measurement")
        .expect("capability");

        assert_eq!(
            artifact.portability_status(),
            "unclassified"
        );

        assert_eq!(artifact.compatibility().len(), 2);
        assert!(artifact
            .domain_capabilities()
            .contains("quantum.measurement"));
    }

    #[test]
    fn cancellation_is_observed() {
        let directory = TestDirectory::new();

        directory.write("a.zm", b"source");

        let cancellation = PackageCancellation::new();
        cancellation.cancel();

        let result = PackageCreator::new().create(
            &request(&directory.path)
                .include_file(
                    PackageFileSpec::new("a.zm", "a.zm", "source")
                        .expect("file"),
                )
                .expect("include file"),
            &cancellation,
        );

        assert!(matches!(
            result,
            Err(PackageCreationError::Cancelled)
        ));
    }

    #[test]
    fn payload_handles_remain_available_to_the_archive_writer() {
        let directory = TestDirectory::new();

        directory.write("source.zm", b"package test;");

        let draft = PackageCreator::new()
            .create(
                &request(&directory.path)
                    .include_file(
                        PackageFileSpec::new(
                            "source.zm",
                            "source.zm",
                            "source",
                        )
                        .expect("file"),
                    )
                    .expect("include file"),
                &PackageCancellation::new(),
            )
            .expect("create package");

        assert_eq!(draft.payloads.len(), 1);

        let mut parts = draft.into_parts();
        let payload = &mut parts.1[0];

        payload.rewind().expect("rewind payload");

        let mut bytes = Vec::new();
        payload
            .reader_mut()
            .read_to_end(&mut bytes)
            .expect("read payload");

        assert_eq!(bytes, b"package test;");
    }
}