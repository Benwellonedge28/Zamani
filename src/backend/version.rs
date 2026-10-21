
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Backend Version Contracts
//!
//! Path: `src/backend/version.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines explicit versions and compatibility ranges for the backend API
//! and backend artifact formats.
//!
//! ## Ownership
//!
//! This module owns:
//! - The common three-component version value.
//! - Backend API version identities.
//! - Backend artifact-format version identities.
//! - Validated inclusive compatibility ranges.
//! - Version parsing, formatting, and compatibility errors.
//!
//! It does not own:
//! - The Zamani source-language version.
//! - Source-language semantics or grammar compatibility.
//! - ZUIR, classical IR, quantum IR, or HDL IR version policy.
//! - Compiler release versions.
//! - Target architecture, ABI, hardware, or toolchain versions.
//! - Artifact contents, artifact hashes, or cache invalidation.
//! - Automatic migration of incompatible artifacts.
//!
//! ## Compatibility contract
//!
//! A version is compatible only when it falls within an explicitly declared
//! supported range. This module does not assume that equal major versions,
//! newer minor versions, or matching version numbers alone guarantee
//! compatibility.
//!
//! Compatibility ranges are inclusive at both ends.
//!
//! ## Version representation
//!
//! The canonical textual representation is `major.minor.patch`.
//! Each component is an unsigned 32-bit integer. Leading zeroes are rejected
//! except for the component `0`.
//!
//! This representation is a three-component version contract; it does not
//! claim to implement the complete Semantic Versioning specification.
//!
//! ## Scalability
//!
//! No hardware capacities, device counts, collection sizes, or target counts
//! are defined here. The numeric components are version identifiers, not
//! resource limits.
//!
//! ## Integration
//!
//! - `registry.rs` uses `BackendApiVersion` to describe backend API support.
//! - `manifest.rs` uses `ArtifactFormatVersion` to identify serialized formats.
//! - `cache.rs` includes relevant version identities in cache keys.
//! - `api.rs` exposes the approved version types.
//! - `errors.rs` may wrap `VersionError` without duplicating its definitions.
//!
//! This module intentionally has no dependency on those future modules.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// A common, ordered three-component version value.
///
/// This type represents version identity only. It does not decide whether
/// two versions are compatible; compatibility is defined by an explicit
/// [`VersionRange`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    /// Construct a version from its major, minor, and patch components.
    ///
    /// All `u32` component values are valid. This constructor imposes no
    /// hardware-related or deployment-related limits.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Return the major component.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Return the minor component.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Return the patch component.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Errors produced when parsing a version string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionParseError {
    /// The input does not contain exactly three components.
    InvalidComponentCount,

    /// A component is empty, signed, or contains non-decimal characters.
    InvalidComponent,

    /// A component contains a leading zero.
    NonCanonicalComponent,

    /// A component exceeds the `u32` range.
    ComponentOverflow,
}

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidComponentCount => {
                f.write_str("version must contain exactly three components")
            }
            Self::InvalidComponent => {
                f.write_str("version components must contain decimal digits only")
            }
            Self::NonCanonicalComponent => {
                f.write_str("version components must not contain leading zeroes")
            }
            Self::ComponentOverflow => {
                f.write_str("version component exceeds the u32 range")
            }
        }
    }
}

impl std::error::Error for VersionParseError {}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split('.');

        let major = parse_component(parts.next().ok_or(
            VersionParseError::InvalidComponentCount,
        )?)?;

        let minor = parse_component(parts.next().ok_or(
            VersionParseError::InvalidComponentCount,
        )?)?;

        let patch = parse_component(parts.next().ok_or(
            VersionParseError::InvalidComponentCount,
        )?)?;

        if parts.next().is_some() {
            return Err(VersionParseError::InvalidComponentCount);
        }

        Ok(Self::new(major, minor, patch))
    }
}

fn parse_component(input: &str) -> Result<u32, VersionParseError> {
    if input.is_empty() || !input.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(VersionParseError::InvalidComponent);
    }

    if input.len() > 1 && input.starts_with('0') {
        return Err(VersionParseError::NonCanonicalComponent);
    }

    input
        .parse::<u32>()
        .map_err(|_| VersionParseError::ComponentOverflow)
}

/// A version of the backend API contract.
///
/// This is distinct from the version of any artifact emitted by that API.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct BackendApiVersion(Version);

impl BackendApiVersion {
    /// Construct a backend API version.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self(Version::new(major, minor, patch))
    }

    /// Construct a backend API version from a common version value.
    #[must_use]
    pub const fn from_version(version: Version) -> Self {
        Self(version)
    }

    /// Return the underlying version value.
    #[must_use]
    pub const fn version(self) -> Version {
        self.0
    }

    /// Return whether this version is supported by the supplied range.
    #[must_use]
    pub fn is_supported_by(self, range: &BackendApiVersionRange) -> bool {
        range.contains(self)
    }
}

impl fmt::Display for BackendApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for BackendApiVersion {
    type Err = VersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.parse::<Version>().map(Self)
    }
}

/// A version of a backend-generated artifact format.
///
/// An artifact format version is not a backend API version, language version,
/// or target toolchain version.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct ArtifactFormatVersion(Version);

impl ArtifactFormatVersion {
    /// Construct an artifact-format version.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self(Version::new(major, minor, patch))
    }

    /// Construct an artifact-format version from a common version value.
    #[must_use]
    pub const fn from_version(version: Version) -> Self {
        Self(version)
    }

    /// Return the underlying version value.
    #[must_use]
    pub const fn version(self) -> Version {
        self.0
    }

    /// Return whether this version is supported by the supplied range.
    #[must_use]
    pub fn is_supported_by(self, range: &ArtifactFormatVersionRange) -> bool {
        range.contains(self)
    }
}

impl fmt::Display for ArtifactFormatVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for ArtifactFormatVersion {
    type Err = VersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.parse::<Version>().map(Self)
    }
}

/// An invalid version range, such as one whose minimum exceeds its maximum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidVersionRange;

impl fmt::Display for InvalidVersionRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("version range minimum must not exceed its maximum")
    }
}

impl std::error::Error for InvalidVersionRange {}

/// An inclusive range of supported versions.
///
/// The range cannot be constructed with its minimum greater than its maximum.
/// It is intentionally not serializable by default: callers loading ranges
/// from configuration must validate them through [`VersionRange::new`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VersionRange<V> {
    minimum: V,
    maximum: V,
}

impl<V: Copy + Ord> VersionRange<V> {
    /// Construct a validated inclusive range.
    pub fn new(minimum: V, maximum: V) -> Result<Self, InvalidVersionRange> {
        if minimum > maximum {
            return Err(InvalidVersionRange);
        }

        Ok(Self { minimum, maximum })
    }

    /// Return the inclusive minimum.
    #[must_use]
    pub const fn minimum(&self) -> V {
        self.minimum
    }

    /// Return the inclusive maximum.
    #[must_use]
    pub const fn maximum(&self) -> V {
        self.maximum
    }

    /// Test whether a version falls within the inclusive range.
    #[must_use]
    pub fn contains(&self, version: V) -> bool {
        self.minimum <= version && version <= self.maximum
    }
}

/// Supported versions of the backend API.
///
/// Use this type instead of a generic range in registry and plugin contracts
/// to prevent accidental mixing with artifact-format versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BackendApiVersionRange(VersionRange<BackendApiVersion>);

impl BackendApiVersionRange {
    /// Construct a validated inclusive backend API version range.
    pub fn new(
        minimum: BackendApiVersion,
        maximum: BackendApiVersion,
    ) -> Result<Self, InvalidVersionRange> {
        VersionRange::new(minimum, maximum).map(Self)
    }

    /// Return the inclusive minimum supported backend API version.
    #[must_use]
    pub const fn minimum(&self) -> BackendApiVersion {
        self.0.minimum()
    }

    /// Return the inclusive maximum supported backend API version.
    #[must_use]
    pub const fn maximum(&self) -> BackendApiVersion {
        self.0.maximum()
    }

    /// Test whether the specified backend API version is supported.
    #[must_use]
    pub fn contains(&self, version: BackendApiVersion) -> bool {
        self.0.contains(version)
    }
}

/// Supported versions of a backend artifact format.
///
/// This type is distinct from [`BackendApiVersionRange`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArtifactFormatVersionRange(VersionRange<ArtifactFormatVersion>);

impl ArtifactFormatVersionRange {
    /// Construct a validated inclusive artifact-format version range.
    pub fn new(
        minimum: ArtifactFormatVersion,
        maximum: ArtifactFormatVersion,
    ) -> Result<Self, InvalidVersionRange> {
        VersionRange::new(minimum, maximum).map(Self)
    }

    /// Return the inclusive minimum supported artifact-format version.
    #[must_use]
    pub const fn minimum(&self) -> ArtifactFormatVersion {
        self.0.minimum()
    }

    /// Return the inclusive maximum supported artifact-format version.
    #[must_use]
    pub const fn maximum(&self) -> ArtifactFormatVersion {
        self.0.maximum()
    }

    /// Test whether the specified artifact-format version is supported.
    #[must_use]
    pub fn contains(&self, version: ArtifactFormatVersion) -> bool {
        self.0.contains(version)
    }
}

/// A version compatibility failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionError {
    /// The requested backend API version is not supported.
    UnsupportedBackendApi {
        /// Version requested by the caller or plugin.
        requested: BackendApiVersion,
    },

    /// The requested artifact-format version is not supported.
    UnsupportedArtifactFormat {
        /// Version identified by the artifact.
        requested: ArtifactFormatVersion,
    },
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedBackendApi { requested } => {
                write!(f, "unsupported backend API version {requested}")
            }
            Self::UnsupportedArtifactFormat { requested } => {
                write!(f, "unsupported artifact format version {requested}")
            }
        }
    }
}

impl std::error::Error for VersionError {}

/// Validate a requested backend API version against an explicit range.
///
/// No compatibility is inferred from version-number similarity.
pub fn require_backend_api(
    requested: BackendApiVersion,
    supported: &BackendApiVersionRange,
) -> Result<(), VersionError> {
    if supported.contains(requested) {
        Ok(())
    } else {
        Err(VersionError::UnsupportedBackendApi { requested })
    }
}

/// Validate an artifact-format version against an explicit reader range.
pub fn require_artifact_format(
    requested: ArtifactFormatVersion,
    supported: &ArtifactFormatVersionRange,
) -> Result<(), VersionError> {
    if supported.contains(requested) {
        Ok(())
    } else {
        Err(VersionError::UnsupportedArtifactFormat { requested })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_round_trips_canonically() {
        let version = Version::new(1, 20, 300);

        assert_eq!(version.to_string(), "1.20.300");
        assert_eq!("1.20.300".parse::<Version>(), Ok(version));
    }

    #[test]
    fn parses_zero_components() {
        assert_eq!("0.0.0".parse::<Version>(), Ok(Version::new(0, 0, 0)));
    }

    #[test]
    fn rejects_noncanonical_and_invalid_versions() {
        assert_eq!(
            "01.2.3".parse::<Version>(),
            Err(VersionParseError::NonCanonicalComponent)
        );
        assert_eq!(
            "1.2".parse::<Version>(),
            Err(VersionParseError::InvalidComponentCount)
        );
        assert_eq!(
            "1.2.3.4".parse::<Version>(),
            Err(VersionParseError::InvalidComponentCount)
        );
        assert_eq!(
            "1.-2.3".parse::<Version>(),
            Err(VersionParseError::InvalidComponent)
        );
        assert_eq!(
            "1.2.4294967296".parse::<Version>(),
            Err(VersionParseError::ComponentOverflow)
        );
    }

    #[test]
    fn api_and_artifact_versions_are_distinct_types() {
        let api = BackendApiVersion::new(1, 2, 3);
        let artifact = ArtifactFormatVersion::new(1, 2, 3);

        assert_eq!(api.to_string(), artifact.to_string());
        assert_eq!(api.version(), artifact.version());

        // The types intentionally cannot be interchanged in typed APIs.
    }

    #[test]
    fn range_includes_both_endpoints() {
        let range = VersionRange::new(
            Version::new(1, 2, 0),
            Version::new(1, 4, 0),
        )
        .expect("ordered bounds should be accepted");

        assert!(range.contains(Version::new(1, 2, 0)));
        assert!(range.contains(Version::new(1, 3, 0)));
        assert!(range.contains(Version::new(1, 4, 0)));
        assert!(!range.contains(Version::new(1, 1, 9)));
        assert!(!range.contains(Version::new(1, 4, 1)));
    }

    #[test]
    fn reversed_range_is_rejected() {
        assert_eq!(
            VersionRange::new(
                Version::new(2, 0, 0),
                Version::new(1, 9, 9),
            ),
            Err(InvalidVersionRange)
        );
    }

    #[test]
    fn api_compatibility_requires_explicit_support() {
        let supported = BackendApiVersionRange::new(
            BackendApiVersion::new(1, 2, 0),
            BackendApiVersion::new(1, 4, 0),
        )
        .expect("ordered bounds should be accepted");

        assert!(require_backend_api(
            BackendApiVersion::new(1, 3, 0),
            &supported
        )
        .is_ok());

        assert_eq!(
            require_backend_api(BackendApiVersion::new(2, 0, 0), &supported),
            Err(VersionError::UnsupportedBackendApi {
                requested: BackendApiVersion::new(2, 0, 0),
            })
        );
    }

    #[test]
    fn artifact_compatibility_is_independent_of_api_compatibility() {
        let supported = ArtifactFormatVersionRange::new(
            ArtifactFormatVersion::new(1, 0, 0),
            ArtifactFormatVersion::new(1, 1, 0),
        )
        .expect("ordered bounds should be accepted");

        assert!(require_artifact_format(
            ArtifactFormatVersion::new(1, 1, 0),
            &supported
        )
        .is_ok());

        assert_eq!(
            require_artifact_format(ArtifactFormatVersion::new(1, 2, 0), &supported),
            Err(VersionError::UnsupportedArtifactFormat {
                requested: ArtifactFormatVersion::new(1, 2, 0),
            })
        );
    }

    #[test]
    fn typed_ranges_reject_reversed_bounds() {
        assert!(BackendApiVersionRange::new(
            BackendApiVersion::new(2, 0, 0),
            BackendApiVersion::new(1, 0, 0),
        )
        .is_err());

        assert!(ArtifactFormatVersionRange::new(
            ArtifactFormatVersion::new(2, 0, 0),
            ArtifactFormatVersion::new(1, 0, 0),
        )
        .is_err());
    }
}
