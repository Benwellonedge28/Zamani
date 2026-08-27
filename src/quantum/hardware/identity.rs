//! Zamani Quantum Hardware — Stable Identity Primitives
//!
//! This module defines the canonical identity types used by the quantum
//! hardware abstraction layer.
//!
//! # Responsibility
//!
//! This module owns:
//!
//! - provider identities;
//! - hardware/device identities;
//! - backend identities;
//! - architecture identities;
//! - firmware versions;
//! - hardware revisions;
//! - canonical identity formatting;
//! - identity validation;
//! - deterministic ordering, equality, and hashing;
//! - lossless Serde serialization;
//! - parsing of externally supplied identity strings.
//!
//! # Non-responsibilities
//!
//! This module deliberately does NOT own:
//!
//! - backend capabilities;
//! - hardware technologies;
//! - topology;
//! - calibration;
//! - instruction sets;
//! - execution;
//! - jobs;
//! - queues;
//! - authentication;
//! - credentials;
//! - networking;
//! - provider-specific APIs;
//! - routing;
//! - scheduling;
//! - benchmarking;
//! - quantum IR;
//! - source-language parsing.
//!
//! Those responsibilities belong to their respective hardware or quantum
//! subsystems.
//!
//! # Architectural contract
//!
//! Identity is a foundational hardware concept. Higher-level hardware
//! modules may depend on this module, but this module must not depend on
//! higher-level hardware modules.
//!
//! Intended dependency direction:
//!
//! ```text
//! identity
//!    │
//!    ├── technology
//!    ├── capabilities
//!    ├── timing
//!    ├── instruction_set
//!    ├── topology
//!    ├── calibration
//!    ├── backend
//!    ├── provider
//!    ├── registry
//!    └── execution
//! ```
//!
//! The identity module must never reverse that dependency.
//!
//! # Canonical representation
//!
//! Identity values are opaque, validated, immutable newtypes around
//! canonical UTF-8 strings. The canonical representation is intentionally
//! deterministic and suitable for:
//!
//! - maps and sets;
//! - cache keys;
//! - persistence;
//! - reproducibility metadata;
//! - audit records;
//! - execution provenance;
//! - serialization;
//! - cross-process communication.
//!
//! # Namespace model
//!
//! Hardware identities may optionally use a namespace:
//!
//! ```text
//! provider://ibm/ibm_torino
//! provider://ionq/forte
//! local://simulator/statevector
//! ```
//!
//! A namespace is part of the canonical identity and therefore participates
//! in equality and hashing.
//!
//! Provider-specific identifiers must never contain credentials, access
//! tokens, URLs containing secrets, or other sensitive material.
//!
//! # Stability
//!
//! The public types in this module are intended to be stable foundational
//! APIs. Future hardware modules should consume these types rather than
//! creating replacement `String` aliases.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Dependencies
//!
//! Only the standard library and the repository's existing Serde dependency
//! are required.

use std::error::Error;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

// =============================================================================
// Constants
// =============================================================================

/// Maximum number of Unicode scalar values permitted in an identity.
///
/// This deliberately keeps identifiers small enough to be practical as
/// database keys, cache keys, log fields, and provider metadata while leaving
/// ample room for real-world provider identifiers.
pub const MAX_IDENTITY_LENGTH: usize = 256;

/// Maximum number of Unicode scalar values permitted in a namespace.
pub const MAX_NAMESPACE_LENGTH: usize = 64;

/// Maximum number of Unicode scalar values permitted in a version string.
pub const MAX_VERSION_LENGTH: usize = 128;

/// Maximum number of Unicode scalar values permitted in a hardware revision.
pub const MAX_REVISION_LENGTH: usize = 128;

/// Canonical default namespace for locally owned resources.
pub const LOCAL_NAMESPACE: &str = "local";

/// Canonical namespace used when an external provider is explicitly named.
pub const PROVIDER_NAMESPACE: &str = "provider";

// =============================================================================
// Identity errors
// =============================================================================

/// Errors produced while constructing or parsing hardware identities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityError {
    /// The supplied identity is empty or contains only whitespace.
    Empty,

    /// The supplied value exceeds the permitted length.
    TooLong {
        /// The logical field being validated.
        field: &'static str,

        /// Number of Unicode scalar values supplied.
        length: usize,

        /// Maximum permitted number of Unicode scalar values.
        maximum: usize,
    },

    /// The supplied value contains leading or trailing whitespace.
    SurroundingWhitespace {
        /// The logical field being validated.
        field: &'static str,
    },

    /// The supplied value contains an invalid character.
    InvalidCharacter {
        /// The logical field being validated.
        field: &'static str,

        /// The offending Unicode scalar value.
        character: char,
    },

    /// A namespace is syntactically invalid.
    InvalidNamespace {
        /// The supplied namespace.
        namespace: String,
    },

    /// A qualified identity contains an invalid number of separators.
    InvalidQualifiedIdentity {
        /// The supplied identity.
        value: String,
    },

    /// A qualified identity contains an empty component.
    EmptyQualifiedComponent {
        /// The supplied identity.
        value: String,
    },

    /// A version contains an invalid component.
    InvalidVersion {
        /// The supplied version.
        value: String,
    },

    /// A hardware revision contains an invalid component.
    InvalidRevision {
        /// The supplied revision.
        value: String,
    },
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                write!(f, "hardware identity cannot be empty")
            }

            Self::TooLong {
                field,
                length,
                maximum,
            } => {
                write!(
                    f,
                    "{} is {} characters long; maximum is {}",
                    field, length, maximum
                )
            }

            Self::SurroundingWhitespace { field } => {
                write!(
                    f,
                    "{} cannot contain leading or trailing whitespace",
                    field
                )
            }

            Self::InvalidCharacter { field, character } => {
                write!(
                    f,
                    "{} contains invalid character {:?}",
                    field, character
                )
            }

            Self::InvalidNamespace { namespace } => {
                write!(f, "invalid hardware identity namespace '{}'", namespace)
            }

            Self::InvalidQualifiedIdentity { value } => {
                write!(
                    f,
                    "invalid qualified hardware identity '{}'",
                    value
                )
            }

            Self::EmptyQualifiedComponent { value } => {
                write!(
                    f,
                    "qualified hardware identity '{}' contains an empty component",
                    value
                )
            }

            Self::InvalidVersion { value } => {
                write!(f, "invalid version '{}'", value)
            }

            Self::InvalidRevision { value } => {
                write!(f, "invalid hardware revision '{}'", value)
            }
        }
    }
}

impl Error for IdentityError {}

// =============================================================================
// Validation helpers
// =============================================================================

/// Returns whether a character is valid inside a canonical identity
/// component.
///
/// Identity components intentionally use a conservative character set:
///
/// - ASCII letters;
/// - ASCII digits;
/// - `_`;
/// - `-`;
/// - `.`.
///
/// This avoids ambiguity across URLs, filesystems, shells, configuration
/// formats, databases, and provider APIs.
fn is_valid_identity_character(character: char) -> bool {
    character.is_ascii_alphanumeric()
        || matches!(character, '_' | '-' | '.')
}

/// Validate a plain identity component.
fn validate_component(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), IdentityError> {
    if value.is_empty() {
        return Err(IdentityError::Empty);
    }

    let length = value.chars().count();

    if length > maximum {
        return Err(IdentityError::TooLong {
            field,
            length,
            maximum,
        });
    }

    if value.trim() != value {
        return Err(IdentityError::SurroundingWhitespace { field });
    }

    for character in value.chars() {
        if !is_valid_identity_character(character) {
            return Err(IdentityError::InvalidCharacter { field, character });
        }
    }

    Ok(())
}

/// Validate a namespace.
fn validate_namespace(value: &str) -> Result<(), IdentityError> {
    if value.is_empty() {
        return Err(IdentityError::InvalidNamespace {
            namespace: value.to_owned(),
        });
    }

    if value.chars().count() > MAX_NAMESPACE_LENGTH {
        return Err(IdentityError::TooLong {
            field: "namespace",
            length: value.chars().count(),
            maximum: MAX_NAMESPACE_LENGTH,
        });
    }

    if value.trim() != value {
        return Err(IdentityError::SurroundingWhitespace {
            field: "namespace",
        });
    }

    for character in value.chars() {
        if !character.is_ascii_lowercase()
            && !character.is_ascii_digit()
            && !matches!(character, '_' | '-')
        {
            return Err(IdentityError::InvalidCharacter {
                field: "namespace",
                character,
            });
        }
    }

    Ok(())
}

/// Validate a version-like string.
///
/// This deliberately accepts both ordinary semantic versions and provider
/// version strings such as:
///
/// - `1.0.0`
/// - `1.97.1`
/// - `v1.2.3`
/// - `2026.08`
/// - `1.0.0-rc1`
/// - `2026.08.27-build7`
///
/// The identity layer validates syntax and canonical representation; it does
/// not attempt to impose semantic-versioning policy on providers.
fn validate_version(value: &str) -> Result<(), IdentityError> {
    if value.is_empty() {
        return Err(IdentityError::InvalidVersion {
            value: value.to_owned(),
        });
    }

    if value.chars().count() > MAX_VERSION_LENGTH {
        return Err(IdentityError::TooLong {
            field: "version",
            length: value.chars().count(),
            maximum: MAX_VERSION_LENGTH,
        });
    }

    if value.trim() != value {
        return Err(IdentityError::SurroundingWhitespace {
            field: "version",
        });
    }

    for character in value.chars() {
        if !character.is_ascii_alphanumeric()
            && !matches!(character, '.' | '-' | '+' | '_')
        {
            return Err(IdentityError::InvalidCharacter {
                field: "version",
                character,
            });
        }
    }

    Ok(())
}

/// Validate a hardware revision.
///
/// Hardware revisions are intentionally not forced into semantic versioning.
/// Real devices commonly use values such as:
///
/// - `A0`
/// - `B1`
/// - `rev-2`
/// - `v3`
/// - `gen4`
/// - `chip-2026-01`.
fn validate_revision(value: &str) -> Result<(), IdentityError> {
    if value.is_empty() {
        return Err(IdentityError::InvalidRevision {
            value: value.to_owned(),
        });
    }

    if value.chars().count() > MAX_REVISION_LENGTH {
        return Err(IdentityError::TooLong {
            field: "hardware revision",
            length: value.chars().count(),
            maximum: MAX_REVISION_LENGTH,
        });
    }

    if value.trim() != value {
        return Err(IdentityError::SurroundingWhitespace {
            field: "hardware revision",
        });
    }

    for character in value.chars() {
        if !is_valid_identity_character(character) {
            return Err(IdentityError::InvalidCharacter {
                field: "hardware revision",
                character,
            });
        }
    }

    Ok(())
}

// =============================================================================
// Namespace
// =============================================================================

/// Namespace for a hardware identity.
///
/// Namespaces prevent otherwise identical provider/device names from
/// colliding.
///
/// Examples:
///
/// ```text
/// provider
/// local
/// emulator
/// simulator
/// custom
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IdentityNamespace(String);

impl IdentityNamespace {
    /// Construct a validated namespace.
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();

        validate_namespace(&value)?;

        Ok(Self(value))
    }

    /// Return the local namespace.
    pub fn local() -> Self {
        Self(LOCAL_NAMESPACE.to_owned())
    }

    /// Return the provider namespace.
    pub fn provider() -> Self {
        Self(PROVIDER_NAMESPACE.to_owned())
    }

    /// Return the canonical namespace string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the namespace and return its canonical string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl Default for IdentityNamespace {
    fn default() -> Self {
        Self::local()
    }
}

impl fmt::Display for IdentityNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for IdentityNamespace {
    type Err = IdentityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for IdentityNamespace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for IdentityNamespace {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// Generic qualified identity
// =============================================================================

/// Canonical namespace-qualified identity.
///
/// The canonical serialized representation is:
///
/// ```text
/// namespace://value
/// ```
///
/// A value may itself contain `/`, allowing hierarchical provider/device
/// identifiers:
///
/// ```text
/// provider://ibm/ibm_torino
/// local://simulator/statevector
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QualifiedIdentity {
    namespace: IdentityNamespace,
    value: String,
}

impl QualifiedIdentity {
    /// Construct a namespace-qualified identity.
    pub fn new(
        namespace: IdentityNamespace,
        value: impl Into<String>,
    ) -> Result<Self, IdentityError> {
        let value = value.into();

        validate_qualified_value(&value)?;

        Ok(Self { namespace, value })
    }

    /// Construct an identity using a namespace string.
    pub fn with_namespace(
        namespace: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, IdentityError> {
        Self::new(IdentityNamespace::new(namespace)?, value)
    }

    /// Return the namespace.
    pub fn namespace(&self) -> &IdentityNamespace {
        &self.namespace
    }

    /// Return the unqualified value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Return the canonical representation.
    pub fn as_str(&self) -> String {
        format!("{}://{}", self.namespace, self.value)
    }

    /// Consume the identity and return its canonical representation.
    pub fn into_string(self) -> String {
        format!("{}://{}", self.namespace, self.value)
    }

    /// Parse a canonical qualified identity.
    ///
    /// Exactly one `://` separator is required.
    pub fn parse(value: &str) -> Result<Self, IdentityError> {
        let (namespace, identity) =
            value.split_once("://").ok_or_else(|| {
                IdentityError::InvalidQualifiedIdentity {
                    value: value.to_owned(),
                }
            })?;

        if namespace.is_empty() || identity.is_empty() {
            return Err(IdentityError::EmptyQualifiedComponent {
                value: value.to_owned(),
            });
        }

        Self::with_namespace(namespace, identity)
    }
}

/// Validate a value which may contain hierarchy separators.
///
/// `/` is permitted here because qualified hardware identities commonly use
/// hierarchical paths. Empty path components and `.`/`..` are rejected to
/// prevent ambiguous canonicalization.
fn validate_qualified_value(value: &str) -> Result<(), IdentityError> {
    if value.is_empty() {
        return Err(IdentityError::Empty);
    }

    if value.chars().count() > MAX_IDENTITY_LENGTH {
        return Err(IdentityError::TooLong {
            field: "identity",
            length: value.chars().count(),
            maximum: MAX_IDENTITY_LENGTH,
        });
    }

    if value.trim() != value {
        return Err(IdentityError::SurroundingWhitespace {
            field: "identity",
        });
    }

    for component in value.split('/') {
        if component.is_empty() {
            return Err(IdentityError::EmptyQualifiedComponent {
                value: value.to_owned(),
            });
        }

        if component == "." || component == ".." {
            return Err(IdentityError::InvalidQualifiedIdentity {
                value: value.to_owned(),
            });
        }

        validate_component("identity component", component, MAX_IDENTITY_LENGTH)?;
    }

    Ok(())
}

impl fmt::Display for QualifiedIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}", self.namespace, self.value)
    }
}

impl FromStr for QualifiedIdentity {
    type Err = IdentityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl Serialize for QualifiedIdentity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.as_str())
    }
}

impl<'de> Deserialize<'de> for QualifiedIdentity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// Macro for strongly typed simple identities
// =============================================================================

macro_rules! define_identity_type {
    (
        $(#[$meta:meta])*
        $name:ident,
        $field_name:literal
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            /// Construct a validated identity.
            pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
                let value = value.into();

                validate_component(
                    $field_name,
                    &value,
                    MAX_IDENTITY_LENGTH,
                )?;

                Ok(Self(value))
            }

            /// Return the canonical identity string.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consume the identity and return its canonical string.
            pub fn into_string(self) -> String {
                self.0
            }

            /// Return whether the identity is empty.
            ///
            /// This always returns `false` for successfully constructed
            /// values and exists as a convenient generic API for callers
            /// working with identity types.
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Return the number of Unicode scalar values in the identity.
            pub fn len(&self) -> usize {
                self.0.chars().count()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl FromStr for $name {
            type Err = IdentityError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;

                Self::new(value).map_err(serde::de::Error::custom)
            }
        }
    };
}

// =============================================================================
// Provider identity
// =============================================================================

define_identity_type!(
    /// Stable identity of a quantum hardware provider.
    ///
    /// Examples:
    ///
    /// ```text
    /// ibm
    /// ionq
    /// quantinuum
    /// rigetti
    /// iqm
    /// aws-braket
    /// quera
    /// local
    /// ```
    ///
    /// Provider IDs are intentionally provider-level identifiers and do not
    /// include a device/backend path.
    ProviderId,
    "provider ID"
);

impl ProviderId {
    /// Construct the canonical local provider identity.
    pub fn local() -> Self {
        Self("local".to_owned())
    }
}

// =============================================================================
// Hardware identity
// =============================================================================

define_identity_type!(
    /// Stable identity of a physical or logical quantum hardware resource.
    ///
    /// This identifies the hardware resource itself rather than an execution
    /// API endpoint.
    ///
    /// Examples:
    ///
    /// ```text
    /// ibm_torino
    /// ionq_forte
    /// chip_a
    /// logical_qpu_01
    /// ```
    HardwareId,
    "hardware ID"
);

// =============================================================================
// Device identity
// =============================================================================

define_identity_type!(
    /// Stable identity of a discoverable quantum device.
    ///
    /// A provider may expose several device identities while one physical
    /// hardware resource may have multiple backend/execution interfaces.
    DeviceId,
    "device ID"
);

// =============================================================================
// Backend identity
// =============================================================================

define_identity_type!(
    /// Stable identity of an executable quantum backend.
    ///
    /// A backend is an execution target, not necessarily synonymous with the
    /// physical device.
    ///
    /// Examples:
    ///
    /// ```text
    /// ibm_torino
    /// ibm_torino-runtime
    /// local-statevector
    /// ```
    BackendId,
    "backend ID"
);

// =============================================================================
// Architecture identity
// =============================================================================

define_identity_type!(
    /// Stable identity of a quantum hardware architecture.
    ///
    /// Architecture identity is deliberately separate from device identity.
    /// Multiple devices may implement the same architecture.
    ///
    /// Examples:
    ///
    /// ```text
    /// ibm-heron-r2
    /// ionq-forte
    /// neutral-atom-a
    /// zamani-statevector-v1
    /// ```
    ArchitectureId,
    "architecture ID"
);

// =============================================================================
// Firmware version
// =============================================================================

/// Validated firmware version.
///
/// Firmware versions are represented as opaque provider-defined version
/// strings rather than forcing every vendor into one versioning scheme.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FirmwareVersion(String);

impl FirmwareVersion {
    /// Construct a validated firmware version.
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();

        validate_version(&value)?;

        Ok(Self(value))
    }

    /// Return the version string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the version and return its string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Parse a conventional semantic version if the caller specifically
    /// requires three numeric components.
    ///
    /// This helper does not change the underlying opaque representation.
    pub fn semver_components(&self) -> Option<(u64, u64, u64)> {
        let value = self.0.strip_prefix('v').unwrap_or(&self.0);

        let mut components = value.split('.');

        let major = components.next()?.parse().ok()?;
        let minor = components.next()?.parse().ok()?;
        let patch_part = components.next()?;

        let patch = patch_part
            .split(|character: char| !character.is_ascii_digit())
            .next()?
            .parse()
            .ok()?;

        if components.next().is_some() {
            return None;
        }

        Some((major, minor, patch))
    }
}

impl fmt::Display for FirmwareVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for FirmwareVersion {
    type Err = IdentityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for FirmwareVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for FirmwareVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// Hardware revision
// =============================================================================

/// Validated physical hardware revision.
///
/// This is intentionally separate from firmware version.
///
/// Examples:
///
/// ```text
/// A0
/// B1
/// rev-2
/// gen4
/// chip-2026-01
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HardwareRevision(String);

impl HardwareRevision {
    /// Construct a validated hardware revision.
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();

        validate_revision(&value)?;

        Ok(Self(value))
    }

    /// Return the revision string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the revision and return its string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for HardwareRevision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for HardwareRevision {
    type Err = IdentityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for HardwareRevision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for HardwareRevision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        Self::new(value).map_err(serde::de::Error::custom)
    }
}

// =============================================================================
// Hardware identity descriptor
// =============================================================================

/// Complete identity information for a quantum hardware resource.
///
/// This structure groups stable identity fields without owning capabilities,
/// topology, calibration, status, or execution state.
///
/// It is therefore safe for higher-level descriptors to embed.
///
/// # Example
///
/// ```
/// use zamani_compiler::quantum::hardware::identity::{
///     ArchitectureId, DeviceId, FirmwareVersion, HardwareId,
///     HardwareIdentity, HardwareRevision, ProviderId,
/// };
///
/// let identity = HardwareIdentity::builder()
///     .provider(ProviderId::new("ibm").unwrap())
///     .hardware(HardwareId::new("ibm_torino").unwrap())
///     .device(DeviceId::new("ibm_torino").unwrap())
///     .architecture(ArchitectureId::new("ibm-heron-r2").unwrap())
///     .firmware(FirmwareVersion::new("1.2.3").unwrap())
///     .revision(HardwareRevision::new("A0").unwrap())
///     .build()
///     .unwrap();
///
/// assert_eq!(identity.provider().as_str(), "ibm");
/// assert_eq!(identity.hardware().as_str(), "ibm_torino");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardwareIdentity {
    /// Provider owning or exposing the hardware.
    provider: ProviderId,

    /// Stable physical/logical hardware resource identity.
    hardware: HardwareId,

    /// Stable execution/discovery device identity.
    device: DeviceId,

    /// Architecture implemented by the hardware.
    architecture: ArchitectureId,

    /// Firmware currently associated with the hardware.
    firmware: FirmwareVersion,

    /// Physical hardware revision.
    revision: HardwareRevision,
}

impl HardwareIdentity {
    /// Create a builder for complete hardware identity information.
    pub fn builder() -> HardwareIdentityBuilder {
        HardwareIdentityBuilder::default()
    }

    /// Return the provider identity.
    pub fn provider(&self) -> &ProviderId {
        &self.provider
    }

    /// Return the hardware identity.
    pub fn hardware(&self) -> &HardwareId {
        &self.hardware
    }

    /// Return the device identity.
    pub fn device(&self) -> &DeviceId {
        &self.device
    }

    /// Return the architecture identity.
    pub fn architecture(&self) -> &ArchitectureId {
        &self.architecture
    }

    /// Return the firmware version.
    pub fn firmware(&self) -> &FirmwareVersion {
        &self.firmware
    }

    /// Return the hardware revision.
    pub fn revision(&self) -> &HardwareRevision {
        &self.revision
    }

    /// Construct the canonical qualified hardware identity.
    ///
    /// Example:
    ///
    /// ```text
    /// provider://ibm/ibm_torino
    /// ```
    pub fn qualified_hardware_id(&self) -> QualifiedIdentity {
        QualifiedIdentity::new(
            IdentityNamespace::provider(),
            format!(
                "{}/{}",
                self.provider.as_str(),
                self.hardware.as_str()
            ),
        )
        .expect("validated hardware identity must remain valid")
    }

    /// Return a deterministic provenance key containing all identity
    /// components.
    ///
    /// This is useful for cache keys and reproducibility metadata.
    pub fn provenance_key(&self) -> String {
        format!(
            "provider={};hardware={};device={};architecture={};firmware={};revision={}",
            self.provider,
            self.hardware,
            self.device,
            self.architecture,
            self.firmware,
            self.revision
        )
    }
}

/// Builder for [`HardwareIdentity`].
#[derive(Debug, Default, Clone)]
pub struct HardwareIdentityBuilder {
    provider: Option<ProviderId>,
    hardware: Option<HardwareId>,
    device: Option<DeviceId>,
    architecture: Option<ArchitectureId>,
    firmware: Option<FirmwareVersion>,
    revision: Option<HardwareRevision>,
}

impl HardwareIdentityBuilder {
    /// Set the provider.
    pub fn provider(mut self, value: ProviderId) -> Self {
        self.provider = Some(value);
        self
    }

    /// Set the hardware identity.
    pub fn hardware(mut self, value: HardwareId) -> Self {
        self.hardware = Some(value);
        self
    }

    /// Set the device identity.
    pub fn device(mut self, value: DeviceId) -> Self {
        self.device = Some(value);
        self
    }

    /// Set the architecture identity.
    pub fn architecture(mut self, value: ArchitectureId) -> Self {
        self.architecture = Some(value);
        self
    }

    /// Set the firmware version.
    pub fn firmware(mut self, value: FirmwareVersion) -> Self {
        self.firmware = Some(value);
        self
    }

    /// Set the hardware revision.
    pub fn revision(mut self, value: HardwareRevision) -> Self {
        self.revision = Some(value);
        self
    }

    /// Build a complete hardware identity.
    ///
    /// All fields are mandatory because a production hardware descriptor
    /// must never silently substitute or invent identity information.
    pub fn build(self) -> Result<HardwareIdentity, IdentityError> {
        Ok(HardwareIdentity {
            provider: self.provider.ok_or(IdentityError::Empty)?,
            hardware: self.hardware.ok_or(IdentityError::Empty)?,
            device: self.device.ok_or(IdentityError::Empty)?,
            architecture: self.architecture.ok_or(IdentityError::Empty)?,
            firmware: self.firmware.ok_or(IdentityError::Empty)?,
            revision: self.revision.ok_or(IdentityError::Empty)?,
        })
    }
}

// =============================================================================
// Backend-qualified identity
// =============================================================================

/// Complete identity of an executable backend.
///
/// This is intentionally separate from [`HardwareIdentity`] because one
/// physical device may expose multiple execution backends.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendIdentity {
    /// Provider exposing the backend.
    provider: ProviderId,

    /// Executable backend identity.
    backend: BackendId,

    /// Physical/logical device associated with the backend.
    device: DeviceId,

    /// Architecture associated with the backend.
    architecture: ArchitectureId,
}

impl BackendIdentity {
    /// Construct a backend identity.
    pub fn new(
        provider: ProviderId,
        backend: BackendId,
        device: DeviceId,
        architecture: ArchitectureId,
    ) -> Self {
        Self {
            provider,
            backend,
            device,
            architecture,
        }
    }

    /// Return the provider.
    pub fn provider(&self) -> &ProviderId {
        &self.provider
    }

    /// Return the backend.
    pub fn backend(&self) -> &BackendId {
        &self.backend
    }

    /// Return the device.
    pub fn device(&self) -> &DeviceId {
        &self.device
    }

    /// Return the architecture.
    pub fn architecture(&self) -> &ArchitectureId {
        &self.architecture
    }

    /// Return the canonical qualified backend identity.
    ///
    /// Example:
    ///
    /// ```text
    /// provider://ibm/ibm_torino
    /// ```
    pub fn qualified(&self) -> QualifiedIdentity {
        QualifiedIdentity::new(
            IdentityNamespace::provider(),
            format!(
                "{}/{}",
                self.provider.as_str(),
                self.backend.as_str()
            ),
        )
        .expect("validated backend identity must remain valid")
    }

    /// Return a deterministic provenance key.
    pub fn provenance_key(&self) -> String {
        format!(
            "provider={};backend={};device={};architecture={}",
            self.provider,
            self.backend,
            self.device,
            self.architecture
        )
    }
}

// =============================================================================
// Identity relationships
// =============================================================================

/// Stable relationship between a provider, device, hardware resource and
/// backend.
///
/// This is intentionally a lightweight reference rather than a full backend
/// descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardwareIdentityRef {
    provider: ProviderId,
    hardware: HardwareId,
    device: DeviceId,
    backend: BackendId,
}

impl HardwareIdentityRef {
    /// Construct a hardware identity reference.
    pub fn new(
        provider: ProviderId,
        hardware: HardwareId,
        device: DeviceId,
        backend: BackendId,
    ) -> Self {
        Self {
            provider,
            hardware,
            device,
            backend,
        }
    }

    /// Return the provider.
    pub fn provider(&self) -> &ProviderId {
        &self.provider
    }

    /// Return the hardware.
    pub fn hardware(&self) -> &HardwareId {
        &self.hardware
    }

    /// Return the device.
    pub fn device(&self) -> &DeviceId {
        &self.device
    }

    /// Return the backend.
    pub fn backend(&self) -> &BackendId {
        &self.backend
    }

    /// Return a deterministic fully qualified backend path.
    ///
    /// Example:
    ///
    /// ```text
    /// provider://ibm/ibm_torino
    /// ```
    pub fn qualified_backend(&self) -> QualifiedIdentity {
        QualifiedIdentity::new(
            IdentityNamespace::provider(),
            format!(
                "{}/{}",
                self.provider.as_str(),
                self.backend.as_str()
            ),
        )
        .expect("validated hardware identity reference must remain valid")
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Basic identity validation
    // -------------------------------------------------------------------------

    #[test]
    fn provider_id_accepts_valid_identifier() {
        let id = ProviderId::new("ibm").unwrap();

        assert_eq!(id.as_str(), "ibm");
        assert_eq!(id.to_string(), "ibm");
    }

    #[test]
    fn provider_id_rejects_empty_identifier() {
        assert_eq!(
            ProviderId::new(""),
            Err(IdentityError::Empty)
        );
    }

    #[test]
    fn provider_id_rejects_whitespace() {
        assert_eq!(
            ProviderId::new(" ibm"),
            Err(IdentityError::SurroundingWhitespace {
                field: "provider ID",
            })
        );

        assert_eq!(
            ProviderId::new("ibm "),
            Err(IdentityError::SurroundingWhitespace {
                field: "provider ID",
            })
        );
    }

    #[test]
    fn provider_id_rejects_unsupported_characters() {
        assert_eq!(
            ProviderId::new("ibm/provider"),
            Err(IdentityError::InvalidCharacter {
                field: "provider ID",
                character: '/',
            })
        );
    }

    #[test]
    fn identity_length_is_bounded() {
        let value = "a".repeat(MAX_IDENTITY_LENGTH + 1);

        assert!(matches!(
            BackendId::new(value),
            Err(IdentityError::TooLong {
                field: "backend ID",
                ..
            })
        ));
    }

    // -------------------------------------------------------------------------
    // Qualified identity
    // -------------------------------------------------------------------------

    #[test]
    fn qualified_identity_has_stable_format() {
        let identity =
            QualifiedIdentity::with_namespace("provider", "ibm/ibm_torino")
                .unwrap();

        assert_eq!(
            identity.to_string(),
            "provider://ibm/ibm_torino"
        );

        assert_eq!(
            identity.as_str(),
            "provider://ibm/ibm_torino"
        );
    }

    #[test]
    fn qualified_identity_round_trips() {
        let original =
            QualifiedIdentity::with_namespace("provider", "ibm/ibm_torino")
                .unwrap();

        let encoded = original.to_string();
        let decoded = QualifiedIdentity::parse(&encoded).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn qualified_identity_requires_separator() {
        assert!(matches!(
            QualifiedIdentity::parse("provider/ibm"),
            Err(IdentityError::InvalidQualifiedIdentity { .. })
        ));
    }

    #[test]
    fn qualified_identity_rejects_empty_component() {
        assert!(matches!(
            QualifiedIdentity::parse("provider://"),
            Err(IdentityError::EmptyQualifiedComponent { .. })
        ));

        assert!(matches!(
            QualifiedIdentity::parse("provider:///ibm"),
            Err(IdentityError::EmptyQualifiedComponent { .. })
        ));

        assert!(matches!(
            QualifiedIdentity::parse("provider://ibm//torino"),
            Err(IdentityError::EmptyQualifiedComponent { .. })
        ));
    }

    #[test]
    fn qualified_identity_rejects_path_traversal_components() {
        assert!(matches!(
            QualifiedIdentity::parse("provider://ibm/../torino"),
            Err(IdentityError::InvalidQualifiedIdentity { .. })
        ));

        assert!(matches!(
            QualifiedIdentity::parse("provider://ibm/./torino"),
            Err(IdentityError::InvalidQualifiedIdentity { .. })
        ));
    }

    // -------------------------------------------------------------------------
    // Provider identity
    // -------------------------------------------------------------------------

    #[test]
    fn local_provider_is_stable() {
        assert_eq!(ProviderId::local().as_str(), "local");
    }

    // -------------------------------------------------------------------------
    // Firmware
    // -------------------------------------------------------------------------

    #[test]
    fn firmware_version_accepts_provider_versions() {
        let versions = [
            "1.0.0",
            "v1.2.3",
            "2026.08",
            "1.0.0-rc1",
            "2026.08.27-build7",
        ];

        for version in versions {
            assert!(
                FirmwareVersion::new(version).is_ok(),
                "version should be accepted: {}",
                version
            );
        }
    }

    #[test]
    fn firmware_version_rejects_invalid_characters() {
        assert!(matches!(
            FirmwareVersion::new("1.0.0/secret"),
            Err(IdentityError::InvalidCharacter {
                field: "version",
                ..
            })
        ));
    }

    #[test]
    fn firmware_semver_components_are_available_when_possible() {
        let version = FirmwareVersion::new("v1.2.3").unwrap();

        assert_eq!(
            version.semver_components(),
            Some((1, 2, 3))
        );
    }

    #[test]
    fn firmware_semver_components_are_optional() {
        let version = FirmwareVersion::new("2026.08").unwrap();

        assert_eq!(version.semver_components(), None);
    }

    // -------------------------------------------------------------------------
    // Hardware revision
    // -------------------------------------------------------------------------

    #[test]
    fn hardware_revision_accepts_common_forms() {
        for revision in ["A0", "B1", "rev-2", "gen4", "chip-2026-01"] {
            assert!(
                HardwareRevision::new(revision).is_ok(),
                "revision should be accepted: {}",
                revision
            );
        }
    }

    #[test]
    fn hardware_revision_rejects_whitespace() {
        assert!(matches!(
            HardwareRevision::new(" A0"),
            Err(IdentityError::SurroundingWhitespace {
                field: "hardware revision",
            })
        ));
    }

    // -------------------------------------------------------------------------
    // Complete identity
    // -------------------------------------------------------------------------

    #[test]
    fn complete_hardware_identity_can_be_built() {
        let identity = HardwareIdentity::builder()
            .provider(ProviderId::new("ibm").unwrap())
            .hardware(HardwareId::new("ibm_torino").unwrap())
            .device(DeviceId::new("ibm_torino").unwrap())
            .architecture(ArchitectureId::new("ibm-heron-r2").unwrap())
            .firmware(FirmwareVersion::new("1.2.3").unwrap())
            .revision(HardwareRevision::new("A0").unwrap())
            .build()
            .unwrap();

        assert_eq!(identity.provider().as_str(), "ibm");
        assert_eq!(identity.hardware().as_str(), "ibm_torino");
        assert_eq!(identity.device().as_str(), "ibm_torino");
        assert_eq!(identity.architecture().as_str(), "ibm-heron-r2");
        assert_eq!(identity.firmware().as_str(), "1.2.3");
        assert_eq!(identity.revision().as_str(), "A0");
    }

    #[test]
    fn complete_hardware_identity_requires_all_fields() {
        let result = HardwareIdentity::builder()
            .provider(ProviderId::new("ibm").unwrap())
            .hardware(HardwareId::new("ibm_torino").unwrap())
            .build();

        assert_eq!(result, Err(IdentityError::Empty));
    }

    #[test]
    fn qualified_hardware_id_is_deterministic() {
        let identity = HardwareIdentity::builder()
            .provider(ProviderId::new("ibm").unwrap())
            .hardware(HardwareId::new("ibm_torino").unwrap())
            .device(DeviceId::new("ibm_torino").unwrap())
            .architecture(ArchitectureId::new("ibm-heron-r2").unwrap())
            .firmware(FirmwareVersion::new("1.2.3").unwrap())
            .revision(HardwareRevision::new("A0").unwrap())
            .build()
            .unwrap();

        assert_eq!(
            identity.qualified_hardware_id().to_string(),
            "provider://ibm/ibm_torino"
        );
    }

    #[test]
    fn provenance_key_is_deterministic() {
        let identity = HardwareIdentity::builder()
            .provider(ProviderId::new("ibm").unwrap())
            .hardware(HardwareId::new("ibm_torino").unwrap())
            .device(DeviceId::new("ibm_torino").unwrap())
            .architecture(ArchitectureId::new("ibm-heron-r2").unwrap())
            .firmware(FirmwareVersion::new("1.2.3").unwrap())
            .revision(HardwareRevision::new("A0").unwrap())
            .build()
            .unwrap();

        assert_eq!(
            identity.provenance_key(),
            "provider=ibm;\
             hardware=ibm_torino;\
             device=ibm_torino;\
             architecture=ibm-heron-r2;\
             firmware=1.2.3;\
             revision=A0"
                .replace('\n', "")
                .replace(" ", "")
        );
    }

    // -------------------------------------------------------------------------
    // Backend identity
    // -------------------------------------------------------------------------

    #[test]
    fn backend_identity_is_distinct_from_device_identity() {
        let identity = BackendIdentity::new(
            ProviderId::new("ibm").unwrap(),
            BackendId::new("ibm_torino_runtime").unwrap(),
            DeviceId::new("ibm_torino").unwrap(),
            ArchitectureId::new("ibm-heron-r2").unwrap(),
        );

        assert_eq!(
            identity.qualified().to_string(),
            "provider://ibm/ibm_torino_runtime"
        );

        assert_eq!(
            identity.device().as_str(),
            "ibm_torino"
        );
    }

    // -------------------------------------------------------------------------
    // Hardware identity reference
    // -------------------------------------------------------------------------

    #[test]
    fn hardware_identity_reference_is_stable() {
        let reference = HardwareIdentityRef::new(
            ProviderId::new("ibm").unwrap(),
            HardwareId::new("ibm_torino").unwrap(),
            DeviceId::new("ibm_torino").unwrap(),
            BackendId::new("ibm_torino_runtime").unwrap(),
        );

        assert_eq!(
            reference.qualified_backend().to_string(),
            "provider://ibm/ibm_torino_runtime"
        );
    }

    // -------------------------------------------------------------------------
    // Ordering / hashing
    // -------------------------------------------------------------------------

    #[test]
    fn identities_have_deterministic_ordering() {
        let first = BackendId::new("backend-a").unwrap();
        let second = BackendId::new("backend-b").unwrap();

        assert!(first < second);
    }

    // -------------------------------------------------------------------------
    // Serde
    // -------------------------------------------------------------------------

    #[test]
    fn provider_id_serializes_as_string() {
        let id = ProviderId::new("ibm").unwrap();

        let json = serde_json::to_string(&id).unwrap();

        assert_eq!(json, "\"ibm\"");
    }

    #[test]
    fn provider_id_deserializes_with_validation() {
        let id: ProviderId =
            serde_json::from_str("\"ibm\"").unwrap();

        assert_eq!(id.as_str(), "ibm");
    }

    #[test]
    fn invalid_provider_id_cannot_deserialize() {
        let result =
            serde_json::from_str::<ProviderId>("\"ibm/provider\"");

        assert!(result.is_err());
    }

    #[test]
    fn qualified_identity_serializes_canonically() {
        let id =
            QualifiedIdentity::with_namespace("provider", "ibm/ibm_torino")
                .unwrap();

        let json = serde_json::to_string(&id).unwrap();

        assert_eq!(
            json,
            "\"provider://ibm/ibm_torino\""
        );
    }

    #[test]
    fn complete_hardware_identity_serializes() {
        let identity = HardwareIdentity::builder()
            .provider(ProviderId::new("ibm").unwrap())
            .hardware(HardwareId::new("ibm_torino").unwrap())
            .device(DeviceId::new("ibm_torino").unwrap())
            .architecture(ArchitectureId::new("ibm-heron-r2").unwrap())
            .firmware(FirmwareVersion::new("1.2.3").unwrap())
            .revision(HardwareRevision::new("A0").unwrap())
            .build()
            .unwrap();

        let json = serde_json::to_string(&identity).unwrap();

        let decoded: HardwareIdentity =
            serde_json::from_str(&json).unwrap();

        assert_eq!(identity, decoded);
    }

    // -------------------------------------------------------------------------
    // No secret material
    // -------------------------------------------------------------------------

    #[test]
    fn identity_values_do_not_accept_url_like_secret_material() {
        assert!(ProviderId::new("ibm").is_ok());

        assert!(ProviderId::new("https").is_ok());

        // A slash is intentionally forbidden in simple identity components.
        assert!(ProviderId::new("ibm/api/v1").is_err());

        // Credentials cannot therefore be accidentally embedded in a
        // ProviderId through URL syntax.
        assert!(
            ProviderId::new("ibm:secret").is_err()
        );
    }
}