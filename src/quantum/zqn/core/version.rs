//! ZQN versioning and schema compatibility.
//!
//! # Ownership
//!
//! This file is the single authoritative owner of ZQN version semantics.
//!
//! It owns:
//!
//! - the ZQN public semantic version;
//! - the ZQN schema version;
//! - the ZQN compatibility version;
//! - version parsing and formatting;
//! - compatibility classification;
//! - explicit version requirements;
//! - forward/backward compatibility decisions;
//! - stable version constants;
//! - machine-readable version metadata.
//!
//! It does NOT own:
//!
//! - quantum IR semantics;
//! - qubit identities;
//! - noise models;
//! - quantum channels;
//! - hardware capabilities;
//! - calibration;
//! - serialization formats themselves;
//! - migration implementations;
//! - runtime policy;
//! - vendor APIs.
//!
//! # Architectural role
//!
//! ZQN is a downstream quantum subsystem. Its version contract must therefore
//! remain independent from `quantum::ir` implementation details.
//!
//! The dependency direction is:
//!
//! ```text
//! ZQN core/version
//!       │
//!       ├──► ZQN probability
//!       ├──► ZQN channels
//!       ├──► ZQN faults
//!       ├──► ZQN noise
//!       ├──► ZQN calibration
//!       ├──► ZQN characterization
//!       ├──► ZQN simulation
//!       ├──► ZQN propagation
//!       ├──► ZQN target integration
//!       └──► ZQN I/O
//! ```
//!
//! This file intentionally has no dependency on those modules.
//!
//! # Canonical quantum identity
//!
//! Version objects do not identify quantum resources.
//!
//! Consequently this file deliberately does not import:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Those remain owned by the canonical Quantum IR as required by the quantum
//! architecture. Version identity and resource identity are separate domains.
//!
//! # Write once, scale everywhere
//!
//! Versioning has no relationship to:
//!
//! - number of qubits;
//! - number of operations;
//! - circuit depth;
//! - target size;
//! - number of noise locations;
//! - number of channels;
//! - machine topology;
//! - vendor;
//! - execution technology.
//!
//! There is therefore no machine-size constant in this file.
//!
//! A ZQN version is finite metadata describing software/schema semantics, not a
//! limit on the computation represented by ZQN.
//!
//! # Version layers
//!
//! ZQN deliberately distinguishes three version dimensions:
//!
//! ```text
//! semantic version
//!     │
//!     ├── major
//!     ├── minor
//!     └── patch
//!
//! schema version
//!     │
//!     └── identifies serialized structural schema
//!
//! compatibility version
//!     │
//!     └── identifies the compatibility contract
//! ```
//!
//! These must not be conflated.
//!
//! A patch release may change implementation details without changing semantic
//! meaning. A schema change may require migration even where semantic concepts
//! remain equivalent. Compatibility is therefore represented explicitly.
//!
//! # Semantic-version policy
//!
//! ZQN follows this contract:
//!
//! - `major` changes may break public semantic/API compatibility;
//! - `minor` changes add backward-compatible functionality;
//! - `patch` changes are backward-compatible corrections;
//! - schema versions identify persisted representation structure;
//! - compatibility versions identify the contract expected by consumers.
//!
//! This is an internal ZQN implementation of semantic-version concepts and does
//! not require an additional semver dependency.
//!
//! # Compatibility policy
//!
//! Compatibility is NEVER inferred merely from a version number being greater
//! or smaller.
//!
//! Consumers must ask the version contract explicitly.
//!
//! ```text
//! exact
//! backward compatible
//! forward compatible
//! incompatible
//! ```
//!
//! The result is deterministic and contains no target-specific behavior.
//!
//! # Serialization contract
//!
//! The types in this file derive Serde serialization where appropriate because
//! the workspace already provides Serde.
//!
//! This file does not define a wire format. `zqn::io` owns the external schema.
//!
//! The serialized representation of these types must therefore be treated as a
//! schema input to the ZQN I/O layer rather than as an accidental consequence
//! of Rust memory layout.
//!
//! # Error contract
//!
//! Parsing and validation never panic.
//!
//! Invalid version strings return `VersionError`.
//!
//! No malformed input is silently normalized into a different version.
//!
//! # Resource contract
//!
//! Version values contain only bounded integer fields and therefore require
//! constant-sized storage independent of quantum-system size.
//!
//! There are no allocations in the core version representation.
//!
//! # Determinism contract
//!
//! Version parsing, comparison, compatibility classification and formatting are
//! deterministic.
//!
//! They do not use:
//!
//! - randomness;
//! - clocks;
//! - environment variables;
//! - filesystem state;
//! - network state;
//! - global mutable state.
//!
//! # Thread-safety
//!
//! The contained primitive integer fields make the public version types safe to
//! share between threads. No mutable global state is used.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. it is the only authoritative owner of ZQN version semantics;
//! 2. no ZQN child module defines another ZQN version type;
//! 3. schema and semantic versions remain distinct;
//! 4. compatibility decisions are explicit;
//! 5. malformed versions return errors instead of panicking;
//! 6. no machine-size limit is encoded;
//! 7. no vendor information is encoded;
//! 8. no quantum-resource identity is encoded;
//! 9. no global mutable state exists;
//! 10. no unsafe Rust exists;
//! 11. the public types remain usable by future ZQN modules without redesign;
//! 12. serialization remains owned by the ZQN I/O subsystem;
//! 13. later ZQN modules can depend on this file without requiring changes here.
//!
//! # Integration contract
//!
//! Future ZQN modules should use:
//!
//! ```text
//! crate::quantum::zqn::core::version::ZqnVersion
//! crate::quantum::zqn::core::version::ZqnSchemaVersion
//! crate::quantum::zqn::core::version::ZqnCompatibilityVersion
//! crate::quantum::zqn::core::version::Compatibility
//! crate::quantum::zqn::core::version::VersionRequirement
//! ```
//!
//! The ZQN root may re-export these types through its stable public API.
//!
//! The I/O subsystem should use `ZqnVersionMetadata` when constructing or
//! validating persisted ZQN documents.
//!
//! Calibration, characterization, simulation, benchmarking and hardware
//! integration should record the ZQN semantic version through this module
//! rather than defining independent version constants.
//!
//! =============================================================================
//! Compiler-enforced safety boundary
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};

/// The current ZQN semantic version.
///
/// This is the version of the ZQN semantic/API contract, not the Rust crate
/// version and not the serialized schema version.
///
/// The initial production ZQN contract is version `1.0.0`.
pub const ZQN_VERSION: ZqnVersion = ZqnVersion::new(1, 0, 0);

/// The current ZQN persisted-schema version.
///
/// Schema evolution is deliberately independent from semantic versioning.
pub const ZQN_SCHEMA_VERSION: ZqnSchemaVersion = ZqnSchemaVersion::new(1, 0);

/// The current ZQN compatibility contract.
///
/// A compatibility version is deliberately separate from both package and
/// semantic versions. It identifies the compatibility guarantees that an
/// artifact consumer relies upon.
pub const ZQN_COMPATIBILITY_VERSION: ZqnCompatibilityVersion =
    ZqnCompatibilityVersion::new(1, 0);

/// Machine-readable metadata describing the currently implemented ZQN contract.
pub const ZQN_VERSION_METADATA: ZqnVersionMetadata = ZqnVersionMetadata::new(
    ZQN_VERSION,
    ZQN_SCHEMA_VERSION,
    ZQN_COMPATIBILITY_VERSION,
);

/// A ZQN semantic version.
///
/// The three components have the conventional meanings:
///
/// - major: potentially breaking semantic/API change;
/// - minor: backward-compatible feature addition;
/// - patch: backward-compatible correction.
///
/// This type intentionally contains no machine/resource information.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct ZqnVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl ZqnVersion {
    /// Creates a semantic ZQN version.
    ///
    /// Integer components are already validated by their representation, so
    /// construction cannot fail.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Returns the patch component.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }

    /// Returns whether this is the zero/uninitialized version.
    ///
    /// `0.0.0` is not used as the production ZQN version. This helper is useful
    /// when validating externally constructed metadata.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.major == 0 && self.minor == 0 && self.patch == 0
    }

    /// Determines whether `self` is compatible with the supplied requirement.
    #[must_use]
    pub const fn satisfies(self, requirement: VersionRequirement) -> bool {
        requirement.matches(self)
    }
}

impl fmt::Display for ZqnVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

impl FromStr for ZqnVersion {
    type Err = VersionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parse_semantic_version(value)
    }
}

/// A ZQN persisted-schema version.
///
/// Schema versions are independent from semantic API versions.
///
/// A schema change does not automatically imply that the underlying semantic
/// model changed; it means the persisted representation contract changed.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct ZqnSchemaVersion {
    major: u32,
    minor: u32,
}

impl ZqnSchemaVersion {
    /// Creates a schema version.
    #[must_use]
    pub const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    /// Returns the schema major version.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the schema minor version.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Determines whether this schema version can be read by the supplied
    /// schema compatibility policy.
    #[must_use]
    pub const fn compatible_with(self, supported: Self) -> bool {
        self.major == supported.major && self.minor <= supported.minor
    }
}

impl fmt::Display for ZqnSchemaVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.major, self.minor)
    }
}

impl FromStr for ZqnSchemaVersion {
    type Err = VersionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parse_schema_version(value)
    }
}

/// A ZQN compatibility-contract version.
///
/// This identifies the compatibility guarantees rather than the complete
/// semantic/API version.
///
/// Compatibility versions intentionally have only major/minor components.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
)]
pub struct ZqnCompatibilityVersion {
    major: u32,
    minor: u32,
}

impl ZqnCompatibilityVersion {
    /// Creates a compatibility-contract version.
    #[must_use]
    pub const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    /// Returns the compatibility major version.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the compatibility minor version.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Determines whether `self` can satisfy a consumer expecting `required`.
    ///
    /// Compatibility follows:
    ///
    /// - equal major versions;
    /// - provider minor version must be at least the requested minor version.
    #[must_use]
    pub const fn satisfies(self, required: Self) -> bool {
        self.major == required.major && self.minor >= required.minor
    }
}

impl fmt::Display for ZqnCompatibilityVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}", self.major, self.minor)
    }
}

impl FromStr for ZqnCompatibilityVersion {
    type Err = VersionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parse_schema_version(value).map(|version| {
            Self::new(version.major(), version.minor())
        })
    }
}

/// Complete version metadata for a ZQN artifact.
///
/// This structure is the integration point between the core version contract
/// and the future ZQN I/O/provenance systems.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ZqnVersionMetadata {
    /// Semantic ZQN version.
    pub semantic: ZqnVersion,

    /// Persisted schema version.
    pub schema: ZqnSchemaVersion,

    /// Compatibility-contract version.
    pub compatibility: ZqnCompatibilityVersion,
}

impl ZqnVersionMetadata {
    /// Creates complete ZQN version metadata.
    #[must_use]
    pub const fn new(
        semantic: ZqnVersion,
        schema: ZqnSchemaVersion,
        compatibility: ZqnCompatibilityVersion,
    ) -> Self {
        Self {
            semantic,
            schema,
            compatibility,
        }
    }

    /// Returns the metadata for this implementation of ZQN.
    #[must_use]
    pub const fn current() -> Self {
        ZQN_VERSION_METADATA
    }

    /// Determines whether the supplied metadata can be consumed by this
    /// implementation.
    ///
    /// The semantic version itself is not sufficient to establish serialized
    /// compatibility. Schema and compatibility contracts are checked
    /// independently.
    #[must_use]
    pub const fn accepts(self, candidate: Self) -> bool {
        candidate.schema.compatible_with(self.schema)
            && self.compatibility.satisfies(candidate.compatibility)
    }
}

/// A requirement imposed by a ZQN consumer.
///
/// Requirements are explicit rather than hidden in ad-hoc comparisons.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VersionRequirement {
    /// Any version with the specified major version and at least the specified
    /// minor/patch level.
    CompatibleWith(ZqnVersion),

    /// An exact semantic version.
    Exact(ZqnVersion),

    /// At least the specified semantic version.
    AtLeast(ZqnVersion),

    /// No version is accepted.
    Never,
}

impl VersionRequirement {
    /// Returns whether the supplied version satisfies this requirement.
    #[must_use]
    pub const fn matches(self, version: ZqnVersion) -> bool {
        match self {
            Self::CompatibleWith(required) => {
                version.major == required.major
                    && version >= required
            }
            Self::Exact(required) => version == required,
            Self::AtLeast(required) => version >= required,
            Self::Never => false,
        }
    }
}

/// The compatibility relationship between two ZQN semantic versions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Compatibility {
    /// Versions are exactly equal.
    Exact,

    /// The candidate is a compatible newer minor/patch release.
    BackwardCompatible,

    /// The candidate is an older compatible release.
    ForwardCompatible,

    /// The versions cannot be safely treated as compatible.
    Incompatible,
}

impl Compatibility {
    /// Returns whether this relationship permits consumption without migration.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::Exact | Self::BackwardCompatible | Self::ForwardCompatible
        )
    }

    /// Returns whether an explicit migration may be required.
    ///
    /// This deliberately does not claim that every incompatible version has a
    /// migration path. It only identifies the case where the compatibility
    /// contract cannot guarantee direct consumption.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::Incompatible)
    }
}

/// Compares two ZQN semantic versions according to the public compatibility
/// contract.
///
/// `candidate` is the version of an artifact being consumed.
///
/// `supported` is the version understood by the consumer.
#[must_use]
pub const fn compatibility(
    candidate: ZqnVersion,
    supported: ZqnVersion,
) -> Compatibility {
    if candidate == supported {
        return Compatibility::Exact;
    }

    if candidate.major == supported.major {
        if candidate >= supported {
            Compatibility::BackwardCompatible
        } else {
            Compatibility::ForwardCompatible
        }
    } else {
        Compatibility::Incompatible
    }
}

/// Errors produced while parsing or validating ZQN versions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VersionError {
    /// The input was empty.
    Empty,

    /// The input contained leading or trailing whitespace.
    Whitespace,

    /// A semantic version did not contain exactly three components.
    InvalidComponentCount {
        /// Number of components encountered.
        found: usize,
    },

    /// A schema/compatibility version did not contain exactly two components.
    InvalidSchemaComponentCount {
        /// Number of components encountered.
        found: usize,
    },

    /// One component was empty.
    EmptyComponent {
        /// Zero-based component index.
        index: usize,
    },

    /// A component was not an unsigned decimal integer.
    InvalidComponent {
        /// Zero-based component index.
        index: usize,
    },

    /// A numeric component could not be represented by `u32`.
    ComponentOverflow {
        /// Zero-based component index.
        index: usize,
    },

    /// A semantic version contained unsupported prerelease/build syntax.
    UnsupportedQualifier,

    /// A version violated an explicitly requested requirement.
    RequirementNotSatisfied {
        /// Version received.
        actual: ZqnVersion,
    },
}

impl fmt::Display for VersionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("ZQN version is empty"),

            Self::Whitespace => {
                formatter.write_str(
                    "ZQN version must not contain leading or trailing whitespace",
                )
            }

            Self::InvalidComponentCount { found } => write!(
                formatter,
                "ZQN semantic version requires exactly 3 components, found {found}"
            ),

            Self::InvalidSchemaComponentCount { found } => write!(
                formatter,
                "ZQN schema version requires exactly 2 components, found {found}"
            ),

            Self::EmptyComponent { index } => write!(
                formatter,
                "ZQN version component {index} is empty"
            ),

            Self::InvalidComponent { index } => write!(
                formatter,
                "ZQN version component {index} is not an unsigned decimal integer"
            ),

            Self::ComponentOverflow { index } => write!(
                formatter,
                "ZQN version component {index} exceeds the supported u32 range"
            ),

            Self::UnsupportedQualifier => formatter.write_str(
                "ZQN versions do not accept prerelease or build qualifiers",
            ),

            Self::RequirementNotSatisfied { actual } => write!(
                formatter,
                "ZQN version {actual} does not satisfy the requested version requirement"
            ),
        }
    }
}

impl std::error::Error for VersionError {}

/// Parses a strict three-component semantic version.
///
/// Accepted:
///
/// ```text
/// 1.0.0
/// 12.34.567
/// ```
///
/// Rejected:
///
/// ```text
/// 1
/// 1.0
/// 1.0.0-alpha
/// 1.0.0+build
///  1.0.0
/// 1.0.0
/// ```
fn parse_semantic_version(value: &str) -> Result<ZqnVersion, VersionError> {
    if value.is_empty() {
        return Err(VersionError::Empty);
    }

    if value.trim() != value {
        return Err(VersionError::Whitespace);
    }

    if value.contains('-') || value.contains('+') {
        return Err(VersionError::UnsupportedQualifier);
    }

    let mut components = value.split('.');

    let first = components.next();
    let second = components.next();
    let third = components.next();

    let remaining = components.next();

    if remaining.is_some() || first.is_none() || second.is_none() || third.is_none() {
        return Err(VersionError::InvalidComponentCount {
            found: value.split('.').count(),
        });
    }

    let major = parse_component(first.expect("checked above"), 0)?;
    let minor = parse_component(second.expect("checked above"), 1)?;
    let patch = parse_component(third.expect("checked above"), 2)?;

    Ok(ZqnVersion::new(major, minor, patch))
}

/// Parses a strict two-component schema version.
fn parse_schema_version(
    value: &str,
) -> Result<ZqnSchemaVersion, VersionError> {
    if value.is_empty() {
        return Err(VersionError::Empty);
    }

    if value.trim() != value {
        return Err(VersionError::Whitespace);
    }

    if value.contains('-') || value.contains('+') {
        return Err(VersionError::UnsupportedQualifier);
    }

    let mut components = value.split('.');

    let first = components.next();
    let second = components.next();

    if components.next().is_some()
        || first.is_none()
        || second.is_none()
    {
        return Err(VersionError::InvalidSchemaComponentCount {
            found: value.split('.').count(),
        });
    }

    let major = parse_component(first.expect("checked above"), 0)?;
    let minor = parse_component(second.expect("checked above"), 1)?;

    Ok(ZqnSchemaVersion::new(major, minor))
}

/// Parses one unsigned decimal version component.
fn parse_component(
    component: &str,
    index: usize,
) -> Result<u32, VersionError> {
    if component.is_empty() {
        return Err(VersionError::EmptyComponent { index });
    }

    if !component.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(VersionError::InvalidComponent { index });
    }

    component
        .parse::<u32>()
        .map_err(|_| VersionError::ComponentOverflow { index })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_version_is_production_one_zero_zero() {
        assert_eq!(ZQN_VERSION, ZqnVersion::new(1, 0, 0));
    }

    #[test]
    fn current_schema_is_one_zero() {
        assert_eq!(
            ZQN_SCHEMA_VERSION,
            ZqnSchemaVersion::new(1, 0)
        );
    }

    #[test]
    fn current_compatibility_is_one_zero() {
        assert_eq!(
            ZQN_COMPATIBILITY_VERSION,
            ZqnCompatibilityVersion::new(1, 0)
        );
    }

    #[test]
    fn semantic_version_accessors_are_stable() {
        let version = ZqnVersion::new(7, 13, 42);

        assert_eq!(version.major(), 7);
        assert_eq!(version.minor(), 13);
        assert_eq!(version.patch(), 42);
    }

    #[test]
    fn semantic_version_formats_canonically() {
        let version = ZqnVersion::new(7, 13, 42);

        assert_eq!(version.to_string(), "7.13.42");
    }

    #[test]
    fn semantic_version_parses_canonically() {
        let version: ZqnVersion = "7.13.42"
            .parse()
            .expect("valid semantic version");

        assert_eq!(version, ZqnVersion::new(7, 13, 42));
    }

    #[test]
    fn semantic_version_round_trips() {
        let original = ZqnVersion::new(123, 456, 789);
        let encoded = original.to_string();
        let decoded: ZqnVersion = encoded
            .parse()
            .expect("canonical version must parse");

        assert_eq!(original, decoded);
    }

    #[test]
    fn rejects_missing_semantic_components() {
        assert!(matches!(
            "1".parse::<ZqnVersion>(),
            Err(VersionError::InvalidComponentCount { .. })
        ));

        assert!(matches!(
            "1.0".parse::<ZqnVersion>(),
            Err(VersionError::InvalidComponentCount { .. })
        ));
    }

    #[test]
    fn rejects_extra_semantic_components() {
        assert!(matches!(
            "1.0.0.1".parse::<ZqnVersion>(),
            Err(VersionError::InvalidComponentCount { .. })
        ));
    }

    #[test]
    fn rejects_non_numeric_components() {
        assert!(matches!(
            "1.a.0".parse::<ZqnVersion>(),
            Err(VersionError::InvalidComponent { index: 1 })
        ));
    }

    #[test]
    fn rejects_empty_components() {
        assert!(matches!(
            "1..0".parse::<ZqnVersion>(),
            Err(VersionError::EmptyComponent { index: 1 })
        ));
    }

    #[test]
    fn rejects_leading_whitespace() {
        assert_eq!(
            " 1.0.0".parse::<ZqnVersion>(),
            Err(VersionError::Whitespace)
        );
    }

    #[test]
    fn rejects_trailing_whitespace() {
        assert_eq!(
            "1.0.0 ".parse::<ZqnVersion>(),
            Err(VersionError::Whitespace)
        );
    }

    #[test]
    fn rejects_prerelease_versions() {
        assert_eq!(
            "1.0.0-alpha".parse::<ZqnVersion>(),
            Err(VersionError::UnsupportedQualifier)
        );
    }

    #[test]
    fn rejects_build_versions() {
        assert_eq!(
            "1.0.0+build".parse::<ZqnVersion>(),
            Err(VersionError::UnsupportedQualifier)
        );
    }

    #[test]
    fn rejects_component_overflow() {
        assert!(matches!(
            "4294967296.0.0".parse::<ZqnVersion>(),
            Err(VersionError::ComponentOverflow { index: 0 })
        ));
    }

    #[test]
    fn schema_version_formats_canonically() {
        let version = ZqnSchemaVersion::new(3, 7);

        assert_eq!(version.to_string(), "3.7");
    }

    #[test]
    fn schema_version_parses_canonically() {
        let version: ZqnSchemaVersion = "3.7"
            .parse()
            .expect("valid schema version");

        assert_eq!(version, ZqnSchemaVersion::new(3, 7));
    }

    #[test]
    fn schema_version_rejects_three_components() {
        assert!(matches!(
            "1.2.3".parse::<ZqnSchemaVersion>(),
            Err(VersionError::InvalidSchemaComponentCount { .. })
        ));
    }

    #[test]
    fn compatibility_version_formats_canonically() {
        let version = ZqnCompatibilityVersion::new(5, 9);

        assert_eq!(version.to_string(), "5.9");
    }

    #[test]
    fn compatibility_version_parses_canonically() {
        let version: ZqnCompatibilityVersion = "5.9"
            .parse()
            .expect("valid compatibility version");

        assert_eq!(version, ZqnCompatibilityVersion::new(5, 9));
    }

    #[test]
    fn equal_versions_are_exactly_compatible() {
        assert_eq!(
            compatibility(
                ZqnVersion::new(1, 0, 0),
                ZqnVersion::new(1, 0, 0),
            ),
            Compatibility::Exact
        );
    }

    #[test]
    fn newer_minor_version_is_backward_compatible() {
        assert_eq!(
            compatibility(
                ZqnVersion::new(1, 2, 0),
                ZqnVersion::new(1, 1, 0),
            ),
            Compatibility::BackwardCompatible
        );
    }

    #[test]
    fn newer_patch_version_is_backward_compatible() {
        assert_eq!(
            compatibility(
                ZqnVersion::new(1, 1, 4),
                ZqnVersion::new(1, 1, 3),
            ),
            Compatibility::BackwardCompatible
        );
    }

    #[test]
    fn older_same_major_version_is_forward_compatible() {
        assert_eq!(
            compatibility(
                ZqnVersion::new(1, 1, 0),
                ZqnVersion::new(1, 2, 0),
            ),
            Compatibility::ForwardCompatible
        );
    }

    #[test]
    fn different_major_versions_are_incompatible() {
        assert_eq!(
            compatibility(
                ZqnVersion::new(2, 0, 0),
                ZqnVersion::new(1, 9, 9),
            ),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn incompatible_versions_require_explicit_migration_policy() {
        assert!(
            compatibility(
                ZqnVersion::new(2, 0, 0),
                ZqnVersion::new(1, 0, 0),
            )
            .requires_migration()
        );
    }

    #[test]
    fn compatible_relationship_is_reported_correctly() {
        assert!(
            Compatibility::Exact.is_compatible()
        );

        assert!(
            Compatibility::BackwardCompatible.is_compatible()
        );

        assert!(
            Compatibility::ForwardCompatible.is_compatible()
        );

        assert!(
            !Compatibility::Incompatible.is_compatible()
        );
    }

    #[test]
    fn compatible_requirement_accepts_same_major_newer_minor() {
        let requirement =
            VersionRequirement::CompatibleWith(
                ZqnVersion::new(1, 2, 0),
            );

        assert!(
            requirement.matches(ZqnVersion::new(1, 3, 0))
        );
    }

    #[test]
    fn compatible_requirement_rejects_different_major() {
        let requirement =
            VersionRequirement::CompatibleWith(
                ZqnVersion::new(1, 0, 0),
            );

        assert!(
            !requirement.matches(ZqnVersion::new(2, 0, 0))
        );
    }

    #[test]
    fn exact_requirement_is_exact() {
        let requirement =
            VersionRequirement::Exact(
                ZqnVersion::new(1, 2, 3),
            );

        assert!(
            requirement.matches(ZqnVersion::new(1, 2, 3))
        );

        assert!(
            !requirement.matches(ZqnVersion::new(1, 2, 4))
        );
    }

    #[test]
    fn at_least_requirement_is_monotonic() {
        let requirement =
            VersionRequirement::AtLeast(
                ZqnVersion::new(1, 2, 3),
            );

        assert!(
            requirement.matches(ZqnVersion::new(1, 2, 3))
        );

        assert!(
            requirement.matches(ZqnVersion::new(1, 9, 0))
        );

        assert!(
            requirement.matches(ZqnVersion::new(2, 0, 0))
        );

        assert!(
            !requirement.matches(ZqnVersion::new(1, 2, 2))
        );
    }

    #[test]
    fn never_requirement_rejects_everything() {
        let requirement = VersionRequirement::Never;

        assert!(
            !requirement.matches(ZqnVersion::new(1, 0, 0))
        );
    }

    #[test]
    fn schema_compatibility_allows_newer_minor_reader() {
        let artifact = ZqnSchemaVersion::new(1, 1);
        let reader = ZqnSchemaVersion::new(1, 2);

        assert!(artifact.compatible_with(reader));
    }

    #[test]
    fn schema_compatibility_rejects_different_major() {
        let artifact = ZqnSchemaVersion::new(2, 0);
        let reader = ZqnSchemaVersion::new(1, 9);

        assert!(!artifact.compatible_with(reader));
    }

    #[test]
    fn compatibility_contract_requires_same_major() {
        let provider = ZqnCompatibilityVersion::new(1, 4);
        let required = ZqnCompatibilityVersion::new(1, 2);

        assert!(provider.satisfies(required));

        let incompatible =
            ZqnCompatibilityVersion::new(2, 0);

        assert!(!incompatible.satisfies(required));
    }

    #[test]
    fn current_metadata_is_self_consistent() {
        let metadata = ZqnVersionMetadata::current();

        assert_eq!(metadata.semantic, ZQN_VERSION);
        assert_eq!(metadata.schema, ZQN_SCHEMA_VERSION);
        assert_eq!(
            metadata.compatibility,
            ZQN_COMPATIBILITY_VERSION
        );
    }

    #[test]
    fn metadata_accepts_itself() {
        let metadata = ZqnVersionMetadata::current();

        assert!(metadata.accepts(metadata));
    }

    #[test]
    fn metadata_accepts_older_compatible_schema() {
        let current = ZqnVersionMetadata::current();

        let candidate = ZqnVersionMetadata::new(
            ZqnVersion::new(1, 0, 0),
            ZqnSchemaVersion::new(1, 0),
            ZqnCompatibilityVersion::new(1, 0),
        );

        assert!(current.accepts(candidate));
    }

    #[test]
    fn metadata_rejects_incompatible_schema_major() {
        let current = ZqnVersionMetadata::current();

        let candidate = ZqnVersionMetadata::new(
            ZqnVersion::new(1, 0, 0),
            ZqnSchemaVersion::new(2, 0),
            ZqnCompatibilityVersion::new(1, 0),
        );

        assert!(!current.accepts(candidate));
    }

    #[test]
    fn ordering_is_lexicographic() {
        assert!(
            ZqnVersion::new(1, 0, 0)
                < ZqnVersion::new(1, 1, 0)
        );

        assert!(
            ZqnVersion::new(1, 1, 0)
                < ZqnVersion::new(1, 1, 1)
        );

        assert!(
            ZqnVersion::new(1, 9, 9)
                < ZqnVersion::new(2, 0, 0)
        );
    }

    #[test]
    fn no_allocation_is_required_for_version_creation() {
        const VERSION: ZqnVersion =
            ZqnVersion::new(4, 8, 15);

        assert_eq!(VERSION.major(), 4);
        assert_eq!(VERSION.minor(), 8);
        assert_eq!(VERSION.patch(), 15);
    }
}