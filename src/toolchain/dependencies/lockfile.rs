
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Toolchain — Dependency Lockfile
//!
//! Path: src/toolchain/dependencies/lockfile.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Persist, load, validate and verify deterministic dependency-resolution
//! snapshots produced by dependencies::resolver.
//!
//! ## Ownership
//!
//! This module owns:
//! - The serialized dependency lockfile schema.
//! - Lockfile format versioning.
//! - Canonical lockfile serialization.
//! - Lockfile deserialization and structural validation.
//! - Lockfile snapshot fingerprints.
//! - Comparison against an authoritative ResolutionGraph.
//! - Safe lockfile persistence.
//!
//! This module does not own:
//! - Dependency declarations or canonical package identities.
//! - Version interpretation or version comparison.
//! - Candidate discovery.
//! - Dependency resolution or candidate selection.
//! - Feature unification.
//! - Dependency graph construction.
//! - Source resolution.
//! - Package downloading or extraction.
//! - Package integrity verification.
//! - Cryptographic signing or trust.
//! - Compiler dependency graphs.
//! - Hardware discovery or resource allocation.
//!
//! ## Authoritative integration
//!
//! dependencies::model:
//!     DependencyName
//!
//! dependencies::constraints:
//!     CandidateVersion
//!
//! dependencies::resolver:
//!     CandidateRef
//!     ResolvedPackage
//!     DependencyEdge
//!     ResolutionGraph
//!     ResolutionLock
//!
//! dependencies::sources:
//!     Source identity and resolution remain authoritative there.
//!
//! dependencies::compatibility:
//!     Compatibility validation remains authoritative there.
//!
//! packages::verification:
//!     Package-content integrity remains authoritative there.
//!
//! security:
//!     Trust and signature verification remain authoritative there.
//!
//! ## Determinism
//!
//! The serialized representation uses:
//! - Explicitly ordered fields.
//! - BTreeMap and BTreeSet-backed resolver data.
//! - Canonically ordered package entries.
//! - Canonically ordered dependency edges.
//! - Canonically ordered root dependencies.
//! - Canonically ordered activated features.
//!
//! Identical resolution snapshots produce identical serialized bytes.
//!
//! ## Security
//!
//! The checksum protects against accidental modification and detects
//! corruption. It is NOT a cryptographic signature, authenticity proof,
//! authorization mechanism or package-content verification.
//!
//! Lockfile paths are explicitly supplied by callers.
//! This module never evaluates paths supplied by dependency declarations.
//!
//! ## Scalability
//!
//! No arbitrary package, dependency, feature, edge or domain count
//! is imposed.
//!
//! Memory and processing requirements grow with the supplied graph.
//! Configurable operational budgets belong to the calling toolchain.
//!
//! Literal infinite storage or computation is not promised.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::toolchain::errors::ToolchainError;
use crate::toolchain::result::Result;

use super::constraints::CandidateVersion;

use super::model::DependencyName;

use super::resolver::{
    CandidateRef,
    DependencyEdge,
    LockedCandidate,
    ResolutionGraph,
    ResolutionLock,
};

// ============================================================================
// FORMAT IDENTITY
// ============================================================================

/// Canonical lockfile format identifier.
pub const LOCKFILE_FORMAT: &str = "zamani-dependency-lock";

/// Current lockfile schema version.
///
/// This is independent of:
/// - Zamani language version.
/// - Grammar version.
/// - Rust version.
/// - Compiler version.
/// - Backend version.
/// - Package version.
/// - Registry protocol version.
pub const LOCKFILE_SCHEMA_VERSION: u64 = 1;

/// Digest algorithm identifier.
pub const LOCKFILE_DIGEST_ALGORITHM: &str = "sha256";

const HASH_DOMAIN: &[u8] =
    b"zamani.dependency-lock.snapshot.v1";

const OPERATION: &str = "dependency.lockfile";

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

// ============================================================================
// ERROR TYPES
// ============================================================================

/// Detailed lockfile-specific failure.
#[derive(Debug)]
pub enum LockfileError {
    /// An I/O operation failed.
    Io(io::Error),

    /// Serialization failed.
    Serialization(String),

    /// Deserialization failed.
    Deserialization(String),

    /// The lockfile format is unsupported.
    UnsupportedFormat(String),

    /// The lockfile schema is incompatible.
    UnsupportedSchema(u64),

    /// The checksum does not match the contents.
    IntegrityMismatch {
        /// Expected checksum.
        expected: String,

        /// Calculated checksum.
        actual: String,
    },

    /// The lockfile contains invalid structural data.
    InvalidStructure(String),

    /// The resolution graph does not match the lockfile.
    ResolutionMismatch(String),

    /// The destination path is invalid.
    InvalidPath(String),
}

impl fmt::Display for LockfileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => {
                write!(formatter, "lockfile I/O error: {error}")
            }

            Self::Serialization(message) => {
                write!(formatter, "lockfile serialization failed: {message}")
            }

            Self::Deserialization(message) => {
                write!(formatter, "lockfile deserialization failed: {message}")
            }

            Self::UnsupportedFormat(format) => {
                write!(formatter, "unsupported lockfile format: {format}")
            }

            Self::UnsupportedSchema(version) => {
                write!(
                    formatter,
                    "unsupported lockfile schema version: {version}"
                )
            }

            Self::IntegrityMismatch { expected, actual } => {
                write!(
                    formatter,
                    "lockfile integrity mismatch: expected {expected}, \
                     calculated {actual}"
                )
            }

            Self::InvalidStructure(message) => {
                write!(formatter, "invalid lockfile structure: {message}")
            }

            Self::ResolutionMismatch(message) => {
                write!(
                    formatter,
                    "lockfile resolution mismatch: {message}"
                )
            }

            Self::InvalidPath(message) => {
                write!(formatter, "invalid lockfile path: {message}")
            }
        }
    }
}

impl std::error::Error for LockfileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

fn to_toolchain_error(error: LockfileError) -> ToolchainError {
    ToolchainError::validation_failed(error.to_string())
        .with_operation(OPERATION)
}

// ============================================================================
// SERIALIZED PACKAGE RECORD
// ============================================================================

/// One selected package in a dependency lockfile.
///
/// Package identity is the canonical DependencyName.
///
/// Candidate identity is scoped by its provider identity.
///
/// Version strings are preserved exactly. This module never assumes
/// Semantic Versioning or interprets version ordering.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LockedPackageRecord {
    /// Canonical dependency identity.
    pub name: String,

    /// Candidate provider identity.
    pub provider: String,

    /// Provider-defined candidate identity.
    pub candidate: String,

    /// Explicit version comparison scheme.
    pub version_scheme: String,

    /// Original selected package version.
    pub version: String,

    /// Unified activated features in canonical order.
    pub activated_features: BTreeSet<String>,
}

impl LockedPackageRecord {
    fn validate(&self) -> std::result::Result<(), LockfileError> {
        for (field, value) in [
            ("package name", self.name.as_str()),
            ("provider", self.provider.as_str()),
            ("candidate", self.candidate.as_str()),
            ("version scheme", self.version_scheme.as_str()),
            ("version", self.version.as_str()),
        ] {
            if value.is_empty() || value.trim() != value {
                return Err(LockfileError::InvalidStructure(
                    format!("{field} must be nonempty and canonical"),
                ));
            }

            if value.chars().any(char::is_control) {
                return Err(LockfileError::InvalidStructure(
                    format!("{field} contains control characters"),
                ));
            }
        }

        for feature in &self.activated_features {
            if feature.is_empty()
                || feature.trim() != feature
                || feature.chars().any(char::is_control)
            {
                return Err(LockfileError::InvalidStructure(
                    format!(
                        "package '{}' contains an invalid feature",
                        self.name
                    ),
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// SERIALIZED DEPENDENCY EDGE
// ============================================================================

/// One directed dependency relationship.
///
/// A missing parent represents a direct root dependency.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LockedDependencyEdge {
    /// Parent package, absent for a direct root dependency.
    pub from: Option<String>,

    /// Child package.
    pub to: String,
}

impl LockedDependencyEdge {
    fn validate(&self) -> std::result::Result<(), LockfileError> {
        if self.to.is_empty() {
            return Err(LockfileError::InvalidStructure(
                "dependency edge has an empty destination".into(),
            ));
        }

        if let Some(parent) = &self.from {
            if parent.is_empty() {
                return Err(LockfileError::InvalidStructure(
                    "dependency edge has an empty parent".into(),
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// LOCKFILE PAYLOAD
// ============================================================================

/// Canonical dependency-resolution payload.
///
/// All collections are ordered. The payload deliberately contains
/// no timestamps, local absolute paths or machine-specific identifiers.
///
/// This ensures that equivalent resolution snapshots do not acquire
/// different fingerprints merely because they were generated at
/// different times or on different machines.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LockfilePayload {
    /// Selected packages indexed by canonical dependency name.
    pub packages: BTreeMap<String, LockedPackageRecord>,

    /// Dependency graph edges in canonical order.
    pub edges: BTreeSet<LockedDependencyEdge>,

    /// Direct root dependencies.
    pub roots: BTreeSet<String>,
}

impl LockfilePayload {
    /// Return the number of locked packages.
    #[must_use]
    pub fn package_count(&self) -> usize {
        self.packages.len()
    }

    /// Return the number of locked dependency edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Validate all package and graph relationships.
    pub fn validate(&self) -> std::result::Result<(), LockfileError> {
        for (key, package) in &self.packages {
            package.validate()?;

            if key != &package.name {
                return Err(LockfileError::InvalidStructure(
                    format!(
                        "package map key '{}' does not match package identity '{}'",
                        key, package.name
                    ),
                ));
            }

            DependencyName::new(package.name.clone())
                .map_err(|error| {
                    LockfileError::InvalidStructure(error.to_string())
                })?;
        }

        for root in &self.roots {
            if !self.packages.contains_key(root) {
                return Err(LockfileError::InvalidStructure(
                    format!("root package '{root}' is not locked"),
                ));
            }
        }

        for edge in &self.edges {
            edge.validate()?;

            if !self.packages.contains_key(&edge.to) {
                return Err(LockfileError::InvalidStructure(
                    format!(
                        "dependency edge references unknown package '{}'",
                        edge.to
                    ),
                ));
            }

            if let Some(parent) = &edge.from {
                if !self.packages.contains_key(parent) {
                    return Err(LockfileError::InvalidStructure(
                        format!(
                            "dependency edge references unknown parent '{}'",
                            parent
                        ),
                    ));
                }
            } else if !self.roots.contains(&edge.to) {
                return Err(LockfileError::InvalidStructure(
                    format!(
                        "root edge '{}' is absent from the root set",
                        edge.to
                    ),
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// LOCKFILE DOCUMENT
// ============================================================================

/// Complete versioned dependency lockfile.
///
/// The checksum covers the canonical serialized payload and format identity.
///
/// A checksum establishes snapshot consistency, not authenticity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lockfile {
    /// Lockfile format identifier.
    pub format: String,

    /// Lockfile schema version.
    pub schema_version: u64,

    /// Digest algorithm identifier.
    pub digest_algorithm: String,

    /// Canonical resolution payload.
    pub payload: LockfilePayload,

    /// Lowercase hexadecimal SHA-256 digest.
    pub checksum: String,
}

impl Lockfile {
    /// Construct a lockfile from an authoritative resolution graph.
    ///
    /// No dependency selection is performed here.
    pub fn from_graph(
        graph: &ResolutionGraph,
    ) -> std::result::Result<Self, LockfileError> {
        let payload = payload_from_graph(graph)?;

        payload.validate()?;

        let mut lockfile = Self {
            format: LOCKFILE_FORMAT.to_owned(),
            schema_version: LOCKFILE_SCHEMA_VERSION,
            digest_algorithm: LOCKFILE_DIGEST_ALGORITHM.to_owned(),
            payload,
            checksum: String::new(),
        };

        lockfile.checksum = lockfile.calculate_checksum()?;

        Ok(lockfile)
    }

    /// Validate the complete lockfile.
    pub fn validate(&self) -> std::result::Result<(), LockfileError> {
        if self.format != LOCKFILE_FORMAT {
            return Err(LockfileError::UnsupportedFormat(
                self.format.clone(),
            ));
        }

        if self.schema_version != LOCKFILE_SCHEMA_VERSION {
            return Err(LockfileError::UnsupportedSchema(
                self.schema_version,
            ));
        }

        if self.digest_algorithm != LOCKFILE_DIGEST_ALGORITHM {
            return Err(LockfileError::UnsupportedFormat(
                self.digest_algorithm.clone(),
            ));
        }

        self.payload.validate()?;

        let actual = self.calculate_checksum()?;

        if actual != self.checksum {
            return Err(LockfileError::IntegrityMismatch {
                expected: self.checksum.clone(),
                actual,
            });
        }

        Ok(())
    }

    /// Calculate the canonical snapshot checksum.
    ///
    /// The checksum excludes the checksum field itself.
    pub fn calculate_checksum(
        &self,
    ) -> std::result::Result<String, LockfileError> {
        let canonical = CanonicalChecksumPayload {
            format: &self.format,
            schema_version: self.schema_version,
            digest_algorithm: &self.digest_algorithm,
            payload: &self.payload,
        };

        let encoded = serde_json::to_vec(&canonical)
            .map_err(|error| {
                LockfileError::Serialization(error.to_string())
            })?;

        let mut hasher = Sha256::new();

        hasher.update(HASH_DOMAIN);
        hasher.update((encoded.len() as u128).to_be_bytes());
        hasher.update(encoded);

        Ok(hex_digest(&hasher.finalize()))
    }

    /// Serialize a validated lockfile into canonical JSON bytes.
    pub fn to_bytes(
        &self,
    ) -> std::result::Result<Vec<u8>, LockfileError> {
        self.validate()?;

        serde_json::to_vec_pretty(self).map_err(|error| {
            LockfileError::Serialization(error.to_string())
        })
    }

    /// Deserialize and verify a lockfile from bytes.
    ///
    /// Unknown fields are rejected to prevent silent schema drift.
    pub fn from_bytes(
        bytes: &[u8],
    ) -> std::result::Result<Self, LockfileError> {
        let lockfile: Self = serde_json::from_slice(bytes)
            .map_err(|error| {
                LockfileError::Deserialization(error.to_string())
            })?;

        lockfile.validate()?;

        Ok(lockfile)
    }

    /// Borrow the canonical lockfile payload.
    #[must_use]
    pub fn payload(&self) -> &LockfilePayload {
        &self.payload
    }

    /// Borrow the snapshot checksum.
    #[must_use]
    pub fn checksum(&self) -> &str {
        &self.checksum
    }

    /// Return the number of selected packages.
    #[must_use]
    pub fn package_count(&self) -> usize {
        self.payload.package_count()
    }

    /// Return whether the lockfile contains no packages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.payload.packages.is_empty()
    }
}

/// Borrowed checksum representation.
///
/// Field order is part of the canonical checksum contract.
#[derive(Serialize)]
struct CanonicalChecksumPayload<'a> {
    format: &'a str,
    schema_version: u64,
    digest_algorithm: &'a str,
    payload: &'a LockfilePayload,
}

// ============================================================================
// RESOLUTION GRAPH CONVERSION
// ============================================================================

fn payload_from_graph(
    graph: &ResolutionGraph,
) -> std::result::Result<LockfilePayload, LockfileError> {
    let mut packages = BTreeMap::new();

    for (name, resolved) in graph.packages() {
        let candidate = resolved.candidate();

        let version = candidate.version().ok_or_else(|| {
            LockfileError::InvalidStructure(
                format!(
                    "resolved package '{}' has no established version",
                    name
                ),
            )
        })?;

        let record = LockedPackageRecord {
            name: name.to_string(),
            provider: resolved.reference().provider().as_str().to_owned(),
            candidate: resolved.reference().candidate().as_str().to_owned(),
            version_scheme: version.scheme().as_str().to_owned(),
            version: version.value().to_owned(),
            activated_features: resolved
                .activated_features()
                .iter()
                .map(|feature| feature.as_str().to_owned())
                .collect(),
        };

        if packages.insert(name.to_string(), record).is_some() {
            return Err(LockfileError::InvalidStructure(
                format!("duplicate canonical package identity '{name}'"),
            ));
        }
    }

    let edges = graph
        .edges()
        .iter()
        .map(|edge| LockedDependencyEdge {
            from: edge.from().map(ToString::to_string),
            to: edge.to().to_string(),
        })
        .collect();

    let roots = graph
        .roots()
        .iter()
        .map(ToString::to_string)
        .collect();

    Ok(LockfilePayload {
        packages,
        edges,
        roots,
    })
}

// ============================================================================
// LOCKFILE VERIFICATION
// ============================================================================

/// Verify that a lockfile describes an exact resolution graph.
///
/// This does not perform resolution or contact package providers.
///
/// The resolver remains responsible for establishing candidate availability.
pub fn verify_graph(
    lockfile: &Lockfile,
    graph: &ResolutionGraph,
) -> std::result::Result<(), LockfileError> {
    lockfile.validate()?;

    let actual = payload_from_graph(graph)?;

    if lockfile.payload != actual {
        return Err(LockfileError::ResolutionMismatch(
            "locked packages, candidate identities, versions, features, \
             roots or dependency edges differ from the resolver graph"
                .into(),
        ));
    }

    Ok(())
}

/// Verify a lockfile against an existing in-memory resolution lock.
///
/// This verifies candidate references only. It cannot establish package
/// versions, feature inventories or dependency edges because those facts
/// are not contained in ResolutionLock.
pub fn verify_resolution_lock(
    lockfile: &Lockfile,
    resolution: &ResolutionLock,
) -> std::result::Result<(), LockfileError> {
    lockfile.validate()?;

    if lockfile.payload.packages.len() != resolution.len() {
        return Err(LockfileError::ResolutionMismatch(
            "locked package count differs from resolution lock".into(),
        ));
    }

    for (name, locked) in &lockfile.payload.packages {
        let dependency_name =
            DependencyName::new(name.clone()).map_err(|error| {
                LockfileError::InvalidStructure(error.to_string())
            })?;

        let candidate = resolution.get(&dependency_name).ok_or_else(|| {
            LockfileError::ResolutionMismatch(
                format!("package '{name}' is missing from resolution lock"),
            )
        })?;

        let reference = candidate.reference();

        if reference.provider().as_str() != locked.provider
            || reference.candidate().as_str() != locked.candidate
        {
            return Err(LockfileError::ResolutionMismatch(
                format!(
                    "candidate identity for package '{name}' differs"
                ),
            ));
        }
    }

    Ok(())
}

/// Convert a validated lockfile into the resolver's lightweight lock type.
///
/// This intentionally restores candidate references only.
/// It does not fabricate package facts or a ResolutionGraph.
pub fn to_resolution_lock(
    lockfile: &Lockfile,
) -> std::result::Result<ResolutionLock, LockfileError> {
    lockfile.validate()?;

    let mut result = ResolutionLock::new();

    for (name, package) in &lockfile.payload.packages {
        let dependency_name =
            DependencyName::new(name.clone()).map_err(|error| {
                LockfileError::InvalidStructure(error.to_string())
            })?;

        let provider =
            crate::toolchain::dependencies::resolver::CandidateProviderId::new(
                package.provider.clone(),
            )
            .map_err(|error| {
                LockfileError::InvalidStructure(error.to_string())
            })?;

        let candidate =
            crate::toolchain::dependencies::resolver::CandidateId::new(
                package.candidate.clone(),
            )
            .map_err(|error| {
                LockfileError::InvalidStructure(error.to_string())
            })?;

        result
            .insert(
                dependency_name,
                LockedCandidate::new(CandidateRef::new(provider, candidate)),
            )
            .map_err(|error| {
                LockfileError::InvalidStructure(error.to_string())
            })?;
    }

    Ok(result)
}

// ============================================================================
// FILE PERSISTENCE
// ============================================================================

/// Load and validate a lockfile from an explicitly supplied path.
///
/// No implicit current-directory discovery is performed.
pub fn load(
    path: impl AsRef<Path>,
) -> std::result::Result<Lockfile, LockfileError> {
    let path = path.as_ref();

    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(LockfileError::Io)?;

    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes)
        .map_err(LockfileError::Io)?;

    Lockfile::from_bytes(&bytes)
}

/// Save a validated lockfile using a temporary sibling file.
///
/// The temporary file is created exclusively to avoid overwriting
/// another process's temporary file.
///
/// Rename is used to avoid exposing partially written lockfile contents.
///
/// The destination directory must already exist.
pub fn save(
    path: impl AsRef<Path>,
    lockfile: &Lockfile,
) -> std::result::Result<(), LockfileError> {
    let path = path.as_ref();

    if path.as_os_str().is_empty() {
        return Err(LockfileError::InvalidPath(
            "destination path must not be empty".into(),
        ));
    }

    let parent = path.parent().unwrap_or_else(|| Path::new("."));

    let file_name = path.file_name().ok_or_else(|| {
        LockfileError::InvalidPath(
            "destination must identify a file".into(),
        )
    })?;

    let bytes = lockfile.to_bytes()?;

    let mut temporary = None;

    loop {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);

        let mut temporary_name = file_name.to_os_string();

        temporary_name.push(format!(
            ".zamani-tmp-{}-{sequence}",
            std::process::id()
        ));

        let temporary_path: PathBuf = parent.join(temporary_name);

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
        {
            Ok(mut file) => {
                temporary = Some((temporary_path, file));
                break;
            }

            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                continue;
            }

            Err(error) => {
                return Err(LockfileError::Io(error));
            }
        }
    }

    let (temporary_path, mut file) =
        temporary.expect("temporary lockfile was created");

    let write_result = (|| {
        file.write_all(&bytes)?;
        file.sync_all()?;
        drop(file);

        fs::rename(&temporary_path, path)?;

        Ok::<(), io::Error>(())
    })();

    if let Err(error) = write_result {
        let _ = fs::remove_file(&temporary_path);
        return Err(LockfileError::Io(error));
    }

    Ok(())
}

// ============================================================================
// HASH UTILITIES
// ============================================================================

fn hex_digest(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut output = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }

    output
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_encoding_is_lowercase_hex() {
        let digest = hex_digest(&[0x00, 0x0f, 0xab, 0xff]);

        assert_eq!(digest, "000fabff");
    }

    #[test]
    fn empty_payload_is_structurally_valid() {
        let payload = LockfilePayload {
            packages: BTreeMap::new(),
            edges: BTreeSet::new(),
            roots: BTreeSet::new(),
        };

        assert!(payload.validate().is_ok());
        assert_eq!(payload.package_count(), 0);
        assert_eq!(payload.edge_count(), 0);
    }

    #[test]
    fn dangling_root_is_rejected() {
        let payload = LockfilePayload {
            packages: BTreeMap::new(),
            edges: BTreeSet::new(),
            roots: BTreeSet::from(["missing".to_owned()]),
        };

        assert!(payload.validate().is_err());
    }

    #[test]
    fn dangling_dependency_edge_is_rejected() {
        let payload = LockfilePayload {
            packages: BTreeMap::new(),
            edges: BTreeSet::from([
                LockedDependencyEdge {
                    from: Some("parent".into()),
                    to: "child".into(),
                },
            ]),
            roots: BTreeSet::new(),
        };

        assert!(payload.validate().is_err());
    }

    #[test]
    fn root_edge_requires_root_membership() {
        let package = LockedPackageRecord {
            name: "example".into(),
            provider: "provider".into(),
            candidate: "candidate".into(),
            version_scheme: "scheme".into(),
            version: "1".into(),
            activated_features: BTreeSet::new(),
        };

        let payload = LockfilePayload {
            packages: BTreeMap::from([
                ("example".into(), package),
            ]),
            edges: BTreeSet::from([
                LockedDependencyEdge {
                    from: None,
                    to: "example".into(),
                },
            ]),
            roots: BTreeSet::new(),
        };

        assert!(payload.validate().is_err());
    }

    #[test]
    fn package_map_identity_must_match_record_identity() {
        let package = LockedPackageRecord {
            name: "different".into(),
            provider: "provider".into(),
            candidate: "candidate".into(),
            version_scheme: "scheme".into(),
            version: "1".into(),
            activated_features: BTreeSet::new(),
        };

        let payload = LockfilePayload {
            packages: BTreeMap::from([
                ("example".into(), package),
            ]),
            edges: BTreeSet::new(),
            roots: BTreeSet::new(),
        };

        assert!(payload.validate().is_err());
    }

    #[test]
    fn duplicate_features_are_canonicalized_by_set_representation() {
        let features = BTreeSet::from([
            "quantum.measurement".to_owned(),
            "classical.math".to_owned(),
        ]);

        assert_eq!(features.len(), 2);
        assert!(features.contains("quantum.measurement"));
    }

    #[test]
    fn checksum_changes_when_payload_changes() {
        let first = Lockfile {
            format: LOCKFILE_FORMAT.into(),
            schema_version: LOCKFILE_SCHEMA_VERSION,
            digest_algorithm: LOCKFILE_DIGEST_ALGORITHM.into(),
            payload: LockfilePayload {
                packages: BTreeMap::new(),
                edges: BTreeSet::new(),
                roots: BTreeSet::new(),
            },
            checksum: String::new(),
        };

        let mut second = first.clone();

        second.payload.roots.insert("example".into());

        assert_ne!(
            first.calculate_checksum().unwrap(),
            second.calculate_checksum().unwrap()
        );
    }

    #[test]
    fn serialization_is_deterministic() {
        let first = Lockfile {
            format: LOCKFILE_FORMAT.into(),
            schema_version: LOCKFILE_SCHEMA_VERSION,
            digest_algorithm: LOCKFILE_DIGEST_ALGORITHM.into(),
            payload: LockfilePayload {
                packages: BTreeMap::new(),
                edges: BTreeSet::new(),
                roots: BTreeSet::new(),
            },
            checksum: String::new(),
        };

        let mut first = first;
        first.checksum = first.calculate_checksum().unwrap();

        let second = first.clone();

        assert_eq!(
            first.to_bytes().unwrap(),
            second.to_bytes().unwrap()
        );
    }

    #[test]
    fn modified_checksum_is_rejected() {
        let mut lockfile = Lockfile {
            format: LOCKFILE_FORMAT.into(),
            schema_version: LOCKFILE_SCHEMA_VERSION,
            digest_algorithm: LOCKFILE_DIGEST_ALGORITHM.into(),
            payload: LockfilePayload {
                packages: BTreeMap::new(),
                edges: BTreeSet::new(),
                roots: BTreeSet::new(),
            },
            checksum: String::new(),
        };

        lockfile.checksum = lockfile.calculate_checksum().unwrap();
        lockfile.checksum.push('0');

        assert!(lockfile.validate().is_err());
    }

    #[test]
    fn unknown_schema_is_rejected() {
        let lockfile = Lockfile {
            format: LOCKFILE_FORMAT.into(),
            schema_version: LOCKFILE_SCHEMA_VERSION + 1,
            digest_algorithm: LOCKFILE_DIGEST_ALGORITHM.into(),
            payload: LockfilePayload {
                packages: BTreeMap::new(),
                edges: BTreeSet::new(),
                roots: BTreeSet::new(),
            },
            checksum: String::new(),
        };

        assert!(lockfile.validate().is_err());
    }

    #[test]
    fn unknown_json_fields_are_rejected() {
        let json = br#"{
            "format": "zamani-dependency-lock",
            "schema_version": 1,
            "digest_algorithm": "sha256",
            "payload": {
                "packages": {},
                "edges": [],
                "roots": []
            },
            "checksum": "invalid",
            "unexpected": true
        }"#;

        assert!(Lockfile::from_bytes(json).is_err());
    }

    #[test]
    fn round_trip_preserves_empty_lockfile() {
        let mut lockfile = Lockfile {
            format: LOCKFILE_FORMAT.into(),
            schema_version: LOCKFILE_SCHEMA_VERSION,
            digest_algorithm: LOCKFILE_DIGEST_ALGORITHM.into(),
            payload: LockfilePayload {
                packages: BTreeMap::new(),
                edges: BTreeSet::new(),
                roots: BTreeSet::new(),
            },
            checksum: String::new(),
        };

        lockfile.checksum = lockfile.calculate_checksum().unwrap();

        let encoded = lockfile.to_bytes().unwrap();
        let decoded = Lockfile::from_bytes(&encoded).unwrap();

        assert_eq!(lockfile, decoded);
    }
}
