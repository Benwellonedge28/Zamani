//! # Zamani AST Serialization Versioning
//!
//! Authoritative version and compatibility policy for the native Zamani AST
//! serialization subsystem.
//!
//! ## Architectural position
//!
//! This module belongs to:
//!
//! ```text
//! src/frontend/ast/node/serialization/
//! ```
//!
//! It owns the version identity and compatibility rules for the serialized
//! native AST representation.
//!
//! It deliberately does NOT own:
//!
//! - AST node definitions;
//! - source spans;
//! - `NodeId` allocation;
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - OpenQASM;
//! - QIR;
//! - MLIR;
//! - LLVM;
//! - hardware targets;
//! - backend formats;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - execution.
//!
//! Those systems may consume a deserialized AST, but they must not define the
//! AST serialization version.
//!
//! ## Version layers
//!
//! Zamani intentionally distinguishes several independent version concepts:
//!
//! ```text
//! Zamani language version
//!         │
//!         ├── AST schema version
//!         │
//!         ├── AST serialization format version
//!         │
//!         ├── node-local schema versions
//!         │
//!         ├── extension versions
//!         │
//!         └── compiler version
//! ```
//!
//! These versions MUST NOT be conflated.
//!
//! In particular, changing the compiler implementation does not automatically
//! change the AST serialization format.
//!
//! ## Compatibility model
//!
//! The serialization format uses semantic-version-style compatibility:
//!
//! - MAJOR changes may be incompatible;
//! - MINOR changes may add backward-compatible representation capabilities;
//! - PATCH changes are corrections that do not intentionally change the
//!   representation contract.
//!
//! Compatibility is always evaluated explicitly through [`Compatibility`].
//!
//! No caller should infer compatibility merely by comparing a version number.
//!
//! ## Scalability
//!
//! This module contains no machine-size limits.
//!
//! It does not impose limits on:
//!
//! - AST nodes;
//! - qubits;
//! - registers;
//! - resources;
//! - modules;
//! - operations;
//! - nesting;
//! - serialized payload size.
//!
//! Resource limits for untrusted serialization input belong to the decoding
//! layer and must be explicitly configured there.
//!
//! ## Determinism
//!
//! Version values are plain value types. They contain no:
//!
//! - timestamps;
//! - process IDs;
//! - memory addresses;
//! - random values;
//! - global mutable state;
//! - target information.
//!
//! Therefore version identity is deterministic and suitable for reproducible
//! serialization and compatibility checks.
//!
//! ## Rust compatibility
//!
//! Designed for Rust 1.97 / Rust 1.97.1.
//!
//! This file uses only stable Rust facilities, `serde`, and the standard
//! library. It contains no `unsafe` code.
//!
//! ## Integration contract
//!
//! The intended dependency direction is:
//!
//! ```text
//! version.rs
//!     │
//!     ├── schema.rs
//!     │      │
//!     │      ├── json.rs
//!     │      └── binary.rs
//!     │
//!     └── serialization/mod.rs
//!              │
//!              └── AST nodes
//! ```
//!
//! `version.rs` must remain below concrete serialization implementations.
//!
//! A serializer may consume the constants/types in this file, but this file
//! must never import a serializer in return.
//!
//! ## Important compatibility rule
//!
//! A node-local schema version, such as a version exposed by an individual AST
//! node, is NOT the same thing as [`CURRENT_AST_SCHEMA_VERSION`] or
//! [`CURRENT_SERIALIZATION_FORMAT_VERSION`].
//!
//! Node-local versions identify the contract of one node representation.
//!
//! The AST schema version identifies the aggregate native AST schema.
//!
//! The serialization format version identifies how that schema is encoded.
//!
//! Keeping those identities separate prevents an unrelated node change from
//! silently becoming a serialization-protocol change.
//!

use core::cmp::Ordering;
use core::fmt;

use serde::{Deserialize, Serialize};

/// Current native Zamani AST schema version.
///
/// This identifies the logical aggregate structure of the native AST.
///
/// It is independent from:
///
/// - the Zamani language version;
/// - the compiler version;
/// - the serialization encoding version;
/// - individual node schema versions;
/// - extension versions.
///
/// Increment this when the aggregate AST schema changes in a way that affects
/// the serialized structural contract.
pub const CURRENT_AST_SCHEMA_VERSION: AstSchemaVersion = AstSchemaVersion::new(1, 0, 0);

/// Current AST serialization format version.
///
/// This identifies the representation protocol used by serializers such as the
/// JSON and binary serializers.
///
/// A change to this value means that the serialization protocol itself has
/// changed, even when the logical AST schema has not.
///
/// This separation is important because a serializer can evolve independently
/// from the source-level AST model.
pub const CURRENT_SERIALIZATION_FORMAT_VERSION: SerializationFormatVersion =
    SerializationFormatVersion::new(1, 0, 0);

/// First supported AST schema major version.
///
/// This is intentionally a compatibility-policy value rather than a resource
/// limit.
pub const MIN_SUPPORTED_AST_SCHEMA_MAJOR: u16 = 1;

/// First supported serialization format major version.
pub const MIN_SUPPORTED_SERIALIZATION_FORMAT_MAJOR: u16 = 1;

/// Version of the logical native Zamani AST schema.
///
/// The three components have the following meanings:
///
/// ```text
/// major
///   incompatible structural/schema change
///
/// minor
///   backward-compatible schema extension
///
/// patch
///   compatible correction/documentation-level schema change
/// ```
///
/// The type is intentionally independent from the language's public version.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct AstSchemaVersion {
    major: u16,
    minor: u16,
    patch: u32,
}

impl AstSchemaVersion {
    /// Creates an AST schema version.
    pub const fn new(major: u16, minor: u16, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    #[inline]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor version.
    #[inline]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch version.
    #[inline]
    pub const fn patch(self) -> u32 {
        self.patch
    }

    /// Returns the version as `(major, minor, patch)`.
    #[inline]
    pub const fn components(self) -> (u16, u16, u32) {
        (self.major, self.minor, self.patch)
    }

    /// Returns true when this version has the same major version as `other`.
    ///
    /// Same-major versions are candidates for compatibility, but this method
    /// does not by itself guarantee compatibility.
    #[inline]
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns true when this version is exactly equal to `other`.
    #[inline]
    pub const fn is_exact(self, other: Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }
}

impl fmt::Display for AstSchemaVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Version of the AST serialization representation.
///
/// This is deliberately separate from [`AstSchemaVersion`].
///
/// For example, the logical AST could remain unchanged while the binary
/// representation gains a new framing mechanism. That is a serialization
/// format change, not necessarily an AST schema change.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct SerializationFormatVersion {
    major: u16,
    minor: u16,
    patch: u32,
}

impl SerializationFormatVersion {
    /// Creates a serialization-format version.
    pub const fn new(major: u16, minor: u16, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    #[inline]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor version.
    #[inline]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch version.
    #[inline]
    pub const fn patch(self) -> u32 {
        self.patch
    }

    /// Returns the version as `(major, minor, patch)`.
    #[inline]
    pub const fn components(self) -> (u16, u16, u32) {
        (self.major, self.minor, self.patch)
    }

    /// Returns true when the versions have the same major version.
    #[inline]
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns true when the versions are exactly equal.
    #[inline]
    pub const fn is_exact(self, other: Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }
}

impl fmt::Display for SerializationFormatVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Compatibility classification between two versioned contracts.
///
/// This enum deliberately distinguishes exact compatibility from merely
/// same-major compatibility.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Compatibility {
    /// Versions are identical.
    Exact,

    /// The consumer is older and the producer version is a compatible
    /// same-major extension.
    ///
    /// Whether this can actually be decoded depends on whether the serializer
    /// preserves unknown fields/extensions. Therefore serializers must still
    /// apply their own explicit unknown-data policy.
    CompatibleNewer,

    /// The consumer is newer and the serialized representation is an older
    /// compatible same-major version.
    ///
    /// Forward compatibility may require defaults or explicit migration rules.
    CompatibleOlder,

    /// The major versions differ and compatibility cannot be assumed.
    IncompatibleMajor,

    /// The versions are from the same major line but the requested direction
    /// is not supported by the current policy.
    UnsupportedMinor,
}

impl Compatibility {
    /// Returns true if the compatibility result permits use without a schema
    /// migration.
    ///
    /// `CompatibleNewer` and `CompatibleOlder` mean the version relationship
    /// is structurally eligible for compatibility, but callers must still
    /// obey their unknown-field/extension policy.
    #[inline]
    pub const fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::Exact | Self::CompatibleNewer | Self::CompatibleOlder
        )
    }

    /// Returns true only when the versions are exactly equal.
    #[inline]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns true when explicit migration is required.
    #[inline]
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::IncompatibleMajor | Self::UnsupportedMinor)
    }
}

/// Direction in which compatibility is being evaluated.
///
/// Compatibility is directional because a newer schema may know how to
/// interpret an older representation while an older schema cannot necessarily
/// interpret newly introduced fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompatibilityDirection {
    /// Determine whether `candidate` can be consumed by `reference`.
    ///
    /// This is the normal decoder compatibility check.
    ConsumeCandidate,

    /// Determine whether `candidate` can be used as a producer version for
    /// the `reference` consumer.
    ProduceForConsumer,
}

/// Determines AST schema compatibility.
///
/// The policy is intentionally conservative:
///
/// - exact versions are compatible;
/// - same-major newer versions are potentially readable by an older consumer;
/// - same-major older versions are potentially readable by a newer consumer;
/// - major-version changes are incompatible without explicit migration.
///
/// The final serializer/deserializer remains responsible for validating
/// unknown fields, required fields, extensions, and node-local schemas.
pub const fn ast_schema_compatibility(
    serialized: AstSchemaVersion,
    consumer: AstSchemaVersion,
) -> Compatibility {
    if serialized.is_exact(consumer) {
        return Compatibility::Exact;
    }

    if serialized.major() != consumer.major() {
        return Compatibility::IncompatibleMajor;
    }

    match compare_minor_patch(serialized, consumer) {
        Ordering::Greater => Compatibility::CompatibleNewer,
        Ordering::Less => Compatibility::CompatibleOlder,
        Ordering::Equal => Compatibility::Exact,
    }
}

/// Determines serialization-format compatibility.
///
/// Serialization formats are treated more conservatively than logical AST
/// schemas. A same-major format version is structurally eligible for
/// compatibility, but the concrete serializer must still validate its framing,
/// encoding, flags, and required fields.
pub const fn serialization_format_compatibility(
    serialized: SerializationFormatVersion,
    consumer: SerializationFormatVersion,
) -> Compatibility {
    if serialized.is_exact(consumer) {
        return Compatibility::Exact;
    }

    if serialized.major() != consumer.major() {
        return Compatibility::IncompatibleMajor;
    }

    match compare_minor_patch(serialized, consumer) {
        Ordering::Greater => Compatibility::CompatibleNewer,
        Ordering::Less => Compatibility::CompatibleOlder,
        Ordering::Equal => Compatibility::Exact,
    }
}

/// Compares minor and patch components without allocating.
const fn compare_minor_patch<A, B>(left: A, right: B) -> Ordering
where
    A: VersionComponents,
    B: VersionComponents,
{
    match left.minor().cmp(&right.minor()) {
        Ordering::Equal => left.patch().cmp(&right.patch()),
        ordering => ordering,
    }
}

/// Internal abstraction used only to share comparison logic between the two
/// public version types.
trait VersionComponents {
    fn minor(self) -> u16;
    fn patch(self) -> u32;
}

impl VersionComponents for AstSchemaVersion {
    #[inline]
    fn minor(self) -> u16 {
        self.minor
    }

    #[inline]
    fn patch(self) -> u32 {
        self.patch
    }
}

impl VersionComponents for SerializationFormatVersion {
    #[inline]
    fn minor(self) -> u16 {
        self.minor
    }

    #[inline]
    fn patch(self) -> u32 {
        self.patch
    }
}

/// A complete version descriptor for a serialized AST.
///
/// This is the version envelope metadata that `schema.rs`, `json.rs`, and
/// `binary.rs` can share.
///
/// It deliberately does not contain:
///
/// - compiler version;
/// - language version;
/// - backend version;
/// - hardware information;
/// - target information.
///
/// Those belong to other contracts.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    PartialEq,
    Serialize,
)]
pub struct AstSerializationVersion {
    /// Logical AST schema version.
    pub ast_schema: AstSchemaVersion,

    /// Encoding/serialization protocol version.
    pub serialization_format: SerializationFormatVersion,
}

impl AstSerializationVersion {
    /// Creates a version descriptor.
    pub const fn new(
        ast_schema: AstSchemaVersion,
        serialization_format: SerializationFormatVersion,
    ) -> Self {
        Self {
            ast_schema,
            serialization_format,
        }
    }

    /// Returns the repository's current version descriptor.
    pub const fn current() -> Self {
        Self {
            ast_schema: CURRENT_AST_SCHEMA_VERSION,
            serialization_format: CURRENT_SERIALIZATION_FORMAT_VERSION,
        }
    }

    /// Determines compatibility with another complete version descriptor.
    ///
    /// Both the logical AST schema and serialization protocol must be
    /// compatible.
    pub const fn compatibility(self, other: Self) -> SerializationCompatibility {
        SerializationCompatibility {
            ast_schema: ast_schema_compatibility(self.ast_schema, other.ast_schema),
            serialization_format: serialization_format_compatibility(
                self.serialization_format,
                other.serialization_format,
            ),
        }
    }
}

impl Default for AstSerializationVersion {
    #[inline]
    fn default() -> Self {
        Self::current()
    }
}

/// Compatibility result for the complete AST serialization contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SerializationCompatibility {
    /// Compatibility of the logical AST schema.
    pub ast_schema: Compatibility,

    /// Compatibility of the serialization format.
    pub serialization_format: Compatibility,
}

impl SerializationCompatibility {
    /// Returns true only when both layers are compatible.
    #[inline]
    pub const fn is_compatible(self) -> bool {
        self.ast_schema.is_compatible()
            && self.serialization_format.is_compatible()
    }

    /// Returns true only when both layers are exact.
    #[inline]
    pub const fn is_exact(self) -> bool {
        self.ast_schema.is_exact() && self.serialization_format.is_exact()
    }

    /// Returns true if either layer requires migration.
    #[inline]
    pub const fn requires_migration(self) -> bool {
        self.ast_schema.requires_migration()
            || self.serialization_format.requires_migration()
    }
}

/// Errors raised when a serialized AST version cannot be accepted.
///
/// This type is deliberately independent of JSON/binary framing errors.
/// Concrete serializers can wrap or translate it into their own error types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VersionError {
    /// The serialized AST schema is from an unsupported major version.
    UnsupportedAstSchemaMajor {
        /// Major version encountered in serialized data.
        found: u16,

        /// Lowest major version accepted by this compiler.
        minimum_supported: u16,

        /// Current native AST major version.
        current: u16,
    },

    /// The serialization protocol is from an unsupported major version.
    UnsupportedSerializationMajor {
        /// Major version encountered in serialized data.
        found: u16,

        /// Lowest major version accepted by this compiler.
        minimum_supported: u16,

        /// Current serialization major version.
        current: u16,
    },

    /// The version relationship is structurally same-major but the current
    /// compatibility policy does not allow the requested operation.
    UnsupportedCompatibility {
        /// Serialized AST schema version.
        serialized_ast_schema: AstSchemaVersion,

        /// Consumer AST schema version.
        consumer_ast_schema: AstSchemaVersion,

        /// Serialized serialization-format version.
        serialized_serialization_format: SerializationFormatVersion,

        /// Consumer serialization-format version.
        consumer_serialization_format: SerializationFormatVersion,
    },
}

impl fmt::Display for VersionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedAstSchemaMajor {
                found,
                minimum_supported,
                current,
            } => write!(
                formatter,
                "unsupported Zamani AST schema major version {found}; \
                 minimum supported major is {minimum_supported}, \
                 current major is {current}"
            ),

            Self::UnsupportedSerializationMajor {
                found,
                minimum_supported,
                current,
            } => write!(
                formatter,
                "unsupported Zamani AST serialization format major version {found}; \
                 minimum supported major is {minimum_supported}, \
                 current major is {current}"
            ),

            Self::UnsupportedCompatibility {
                serialized_ast_schema,
                consumer_ast_schema,
                serialized_serialization_format,
                consumer_serialization_format,
            } => write!(
                formatter,
                "AST serialization versions are not compatible: \
                 serialized AST {serialized_ast_schema}, \
                 consumer AST {consumer_ast_schema}, \
                 serialized format {serialized_serialization_format}, \
                 consumer format {consumer_serialization_format}"
            ),
        }
    }
}

impl std::error::Error for VersionError {}

/// Validates a serialized AST version against the currently supported
/// repository contract.
///
/// This function performs only version-policy validation. It does not inspect
/// AST contents and therefore remains safe to call before deserializing the
/// actual AST.
///
/// Concrete deserializers should call this function before constructing an AST
/// from untrusted serialized data whenever the version envelope has already
/// been decoded.
pub const fn validate_version(
    serialized: AstSerializationVersion,
) -> Result<(), VersionError> {
    if serialized.ast_schema.major() < MIN_SUPPORTED_AST_SCHEMA_MAJOR
        || serialized.ast_schema.major() > CURRENT_AST_SCHEMA_VERSION.major()
    {
        return Err(VersionError::UnsupportedAstSchemaMajor {
            found: serialized.ast_schema.major(),
            minimum_supported: MIN_SUPPORTED_AST_SCHEMA_MAJOR,
            current: CURRENT_AST_SCHEMA_VERSION.major(),
        });
    }

    if serialized.serialization_format.major()
        < MIN_SUPPORTED_SERIALIZATION_FORMAT_MAJOR
        || serialized.serialization_format.major()
            > CURRENT_SERIALIZATION_FORMAT_VERSION.major()
    {
        return Err(VersionError::UnsupportedSerializationMajor {
            found: serialized.serialization_format.major(),
            minimum_supported: MIN_SUPPORTED_SERIALIZATION_FORMAT_MAJOR,
            current: CURRENT_SERIALIZATION_FORMAT_VERSION.major(),
        });
    }

    Ok(())
}

/// Validates whether serialized data may be consumed by the current AST
/// implementation.
///
/// This is intentionally stricter than merely checking major versions.
///
/// A serializer/deserializer can use this function as the final version gate
/// before AST materialization.
pub const fn validate_for_current_consumer(
    serialized: AstSerializationVersion,
) -> Result<(), VersionError> {
    match validate_version(serialized) {
        Err(error) => Err(error),
        Ok(()) => {
            let current = AstSerializationVersion::current();
            let compatibility = serialized.compatibility(current);

            if compatibility.is_compatible() {
                Ok(())
            } else {
                Err(VersionError::UnsupportedCompatibility {
                    serialized_ast_schema: serialized.ast_schema,
                    consumer_ast_schema: current.ast_schema,
                    serialized_serialization_format: serialized.serialization_format,
                    consumer_serialization_format: current.serialization_format,
                })
            }
        }
    }
}

/// Returns the current version descriptor used by new serializers.
///
/// Keeping this as a function instead of duplicating the constants throughout
/// `json.rs`, `binary.rs`, or `schema.rs` provides a single integration point.
#[inline]
pub const fn current_version() -> AstSerializationVersion {
    AstSerializationVersion::current()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_versions_are_explicit_and_non_zero() {
        let version = AstSerializationVersion::current();

        assert!(version.ast_schema.major() > 0);
        assert!(version.serialization_format.major() > 0);
    }

    #[test]
    fn exact_ast_schema_versions_are_exactly_compatible() {
        let version = AstSchemaVersion::new(1, 0, 0);

        assert_eq!(
            ast_schema_compatibility(version, version),
            Compatibility::Exact
        );
    }

    #[test]
    fn newer_same_major_ast_schema_is_classified_as_compatible_newer() {
        let serialized = AstSchemaVersion::new(1, 1, 0);
        let consumer = AstSchemaVersion::new(1, 0, 0);

        assert_eq!(
            ast_schema_compatibility(serialized, consumer),
            Compatibility::CompatibleNewer
        );
    }

    #[test]
    fn older_same_major_ast_schema_is_classified_as_compatible_older() {
        let serialized = AstSchemaVersion::new(1, 0, 0);
        let consumer = AstSchemaVersion::new(1, 1, 0);

        assert_eq!(
            ast_schema_compatibility(serialized, consumer),
            Compatibility::CompatibleOlder
        );
    }

    #[test]
    fn different_ast_schema_majors_are_incompatible() {
        let serialized = AstSchemaVersion::new(2, 0, 0);
        let consumer = AstSchemaVersion::new(1, 0, 0);

        assert_eq!(
            ast_schema_compatibility(serialized, consumer),
            Compatibility::IncompatibleMajor
        );
    }

    #[test]
    fn exact_serialization_formats_are_compatible() {
        let version = SerializationFormatVersion::new(1, 0, 0);

        assert_eq!(
            serialization_format_compatibility(version, version),
            Compatibility::Exact
        );
    }

    #[test]
    fn different_serialization_format_majors_are_incompatible() {
        let serialized = SerializationFormatVersion::new(2, 0, 0);
        let consumer = SerializationFormatVersion::new(1, 0, 0);

        assert_eq!(
            serialization_format_compatibility(serialized, consumer),
            Compatibility::IncompatibleMajor
        );
    }

    #[test]
    fn complete_version_requires_both_layers_to_be_compatible() {
        let serialized = AstSerializationVersion::new(
            AstSchemaVersion::new(1, 0, 0),
            SerializationFormatVersion::new(1, 0, 0),
        );

        let consumer = AstSerializationVersion::new(
            AstSchemaVersion::new(1, 1, 0),
            SerializationFormatVersion::new(1, 0, 0),
        );

        let compatibility = serialized.compatibility(consumer);

        assert!(compatibility.is_compatible());
        assert!(!compatibility.is_exact());
        assert!(!compatibility.requires_migration());
    }

    #[test]
    fn version_display_is_stable() {
        assert_eq!(
            AstSchemaVersion::new(7, 12, 34).to_string(),
            "7.12.34"
        );

        assert_eq!(
            SerializationFormatVersion::new(8, 9, 10).to_string(),
            "8.9.10"
        );
    }

    #[test]
    fn current_version_matches_constants() {
        let current = current_version();

        assert_eq!(current.ast_schema, CURRENT_AST_SCHEMA_VERSION);
        assert_eq!(
            current.serialization_format,
            CURRENT_SERIALIZATION_FORMAT_VERSION
        );
    }

    #[test]
    fn current_version_is_accepted() {
        assert!(validate_for_current_consumer(current_version()).is_ok());
    }

    #[test]
    fn unsupported_ast_major_is_rejected() {
        let version = AstSerializationVersion::new(
            AstSchemaVersion::new(CURRENT_AST_SCHEMA_VERSION.major() + 1, 0, 0),
            CURRENT_SERIALIZATION_FORMAT_VERSION,
        );

        assert_eq!(
            validate_version(version),
            Err(VersionError::UnsupportedAstSchemaMajor {
                found: CURRENT_AST_SCHEMA_VERSION.major() + 1,
                minimum_supported: MIN_SUPPORTED_AST_SCHEMA_MAJOR,
                current: CURRENT_AST_SCHEMA_VERSION.major(),
            })
        );
    }

    #[test]
    fn unsupported_serialization_major_is_rejected() {
        let version = AstSerializationVersion::new(
            CURRENT_AST_SCHEMA_VERSION,
            SerializationFormatVersion::new(
                CURRENT_SERIALIZATION_FORMAT_VERSION.major() + 1,
                0,
                0,
            ),
        );

        assert_eq!(
            validate_version(version),
            Err(VersionError::UnsupportedSerializationMajor {
                found: CURRENT_SERIALIZATION_FORMAT_VERSION.major() + 1,
                minimum_supported: MIN_SUPPORTED_SERIALIZATION_FORMAT_MAJOR,
                current: CURRENT_SERIALIZATION_FORMAT_VERSION.major(),
            })
        );
    }

    #[test]
    fn serde_round_trip_preserves_version_identity() {
        let version = AstSerializationVersion::new(
            AstSchemaVersion::new(3, 7, 11),
            SerializationFormatVersion::new(4, 2, 9),
        );

        let encoded = serde_json::to_string(&version).expect("version serializes");
        let decoded: AstSerializationVersion =
            serde_json::from_str(&encoded).expect("version deserializes");

        assert_eq!(version, decoded);
    }

    #[test]
    fn versions_have_deterministic_ordering() {
        let a = AstSchemaVersion::new(1, 0, 0);
        let b = AstSchemaVersion::new(1, 0, 1);
        let c = AstSchemaVersion::new(1, 1, 0);
        let d = AstSchemaVersion::new(2, 0, 0);

        assert!(a < b);
        assert!(b < c);
        assert!(c < d);
    }

    #[test]
    fn compatibility_flags_are_consistent() {
        assert!(Compatibility::Exact.is_compatible());
        assert!(Compatibility::CompatibleNewer.is_compatible());
        assert!(Compatibility::CompatibleOlder.is_compatible());

        assert!(!Compatibility::IncompatibleMajor.is_compatible());
        assert!(!Compatibility::UnsupportedMinor.is_compatible());

        assert!(Compatibility::IncompatibleMajor.requires_migration());
        assert!(Compatibility::UnsupportedMinor.requires_migration());

        assert!(!Compatibility::Exact.requires_migration());
    }
}