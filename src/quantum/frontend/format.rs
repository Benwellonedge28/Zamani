//! Quantum frontend format contracts.
//!
//! This module defines the format-independent identity, version, capability,
//! and feature model used by the quantum frontend.
//!
//! # Architectural boundary
//!
//! This module MUST remain independent of:
//!
//! - OpenQASM
//! - QIR
//! - Quil
//! - any other concrete frontend format
//! - `crate::quantum::ir`
//! - parsers
//! - lexers
//! - importers
//! - exporters
//! - diagnostics
//! - frontend error implementations
//!
//! Concrete formats implement the contracts defined here.
//!
//! The intended dependency direction is:
//!
//! ```text
//! Concrete format
//!       │
//!       ▼
//! frontend::format
//!       │
//!       ├── importer contract
//!       └── exporter contract
//!
//! Concrete format
//!       │
//!       ▼
//! Zamani Quantum IR
//! ```
//!
//! A format must never depend on another format.
//!
//! # Design goals
//!
//! This module provides:
//!
//! - stable format identity;
//! - explicit version identity;
//! - deterministic capability declarations;
//! - feature-level capability queries;
//! - safe comparison of format versions;
//! - format-independent API contracts;
//! - no stringly-typed capability checks;
//! - forward-compatible capability extension;
//! - zero knowledge of concrete formats.
//!
//! # Rust compatibility
//!
//! This implementation targets Rust 1.97.1 and Rust 2021.
//!
//! # Stability
//!
//! `FormatId`, `FormatVersion`, `FormatCapability`, and
//! `FormatCapabilities` are intended to form the stable frontend contract.
//!
//! Concrete format implementations should not expose their internal parser
//! or AST types through this module.

use core::fmt;
use core::str::FromStr;
use std::collections::BTreeSet;
use std::convert::Infallible;

/// Result type used by operations that parse or construct format identifiers.
///
/// This alias intentionally uses `Infallible` because `FormatId` is currently
/// a closed set of syntactically valid, owned identifiers represented by a
/// normalized string.
///
/// Future registry-based APIs may introduce a dedicated validation error
/// without changing the `FormatId` representation.
pub type FormatResult<T> = Result<T, FormatError>;

/// Maximum length of a format identifier in bytes.
///
/// This prevents accidentally creating enormous identifiers while keeping the
/// limit generous enough for vendor and implementation-specific formats.
pub const MAX_FORMAT_ID_LENGTH: usize = 128;

/// Maximum number of capabilities that may be stored in one capability set.
///
/// This is primarily a defensive bound for programmatically constructed
/// capability sets.
pub const MAX_FORMAT_CAPABILITIES: usize = 256;

/// Stable identifier for a frontend format.
///
/// A `FormatId` identifies the *format family*, not a particular version.
///
/// Examples:
///
/// ```text
/// openqasm
/// qir
/// quil
/// ```
///
/// Version information belongs in [`FormatVersion`].
///
/// The identifier is intentionally represented as an owned string so future
/// formats can be added without modifying this file. This is important for
/// independent format addition/removal.
///
/// # Normalization
///
/// Format identifiers:
///
/// - are ASCII;
/// - are lowercase;
/// - may contain ASCII letters, digits, `-`, `_`, and `.`;
/// - must begin with an ASCII letter;
/// - must not contain whitespace;
/// - must not exceed [`MAX_FORMAT_ID_LENGTH`] bytes.
///
/// The canonical representation is returned by [`FormatId::as_str`].
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FormatId(String);

impl FormatId {
    /// Creates a validated format identifier.
    ///
    /// The input is normalized to ASCII lowercase before validation.
    ///
    /// This function is deliberately strict. Format identifiers are protocol
    /// identifiers, not user-facing display names.
    pub fn new(value: impl AsRef<str>) -> Result<Self, FormatError> {
        let value = value.as_ref();

        if value.is_empty() {
            return Err(FormatError::EmptyFormatId);
        }

        if value.len() > MAX_FORMAT_ID_LENGTH {
            return Err(FormatError::FormatIdTooLong {
                max: MAX_FORMAT_ID_LENGTH,
                actual: value.len(),
            });
        }

        if !value.is_ascii() {
            return Err(FormatError::NonAsciiFormatId);
        }

        let normalized = value.to_ascii_lowercase();

        let first = normalized
            .as_bytes()
            .first()
            .copied()
            .ok_or(FormatError::EmptyFormatId)?;

        if !first.is_ascii_alphabetic() {
            return Err(FormatError::InvalidFormatIdStart {
                value: normalized,
            });
        }

        if !normalized
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte == b'.')
        {
            return Err(FormatError::InvalidFormatIdCharacters {
                value: normalized,
            });
        }

        Ok(Self(normalized))
    }

    /// Returns the canonical format identifier.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    #[inline]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Debug for FormatId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("FormatId")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for FormatId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for FormatId {
    type Err = FormatError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Version of a frontend format.
///
/// Versions are intentionally represented numerically rather than as arbitrary
/// strings so callers can perform deterministic compatibility checks.
///
/// For example:
///
/// ```text
/// 3.0
/// 3.1
/// ```
///
/// `patch` is optional in the semantic sense but always stored explicitly as
/// zero when absent.
///
/// Pre-release/build metadata is deliberately not part of this primitive
/// contract. Concrete formats that need richer version semantics should own
/// that representation while exposing a normalized [`FormatVersion`] here.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FormatVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl FormatVersion {
    /// Creates a new format version.
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Creates a version with a zero patch component.
    pub const fn major_minor(major: u32, minor: u32) -> Self {
        Self::new(major, minor, 0)
    }

    /// Returns the major version.
    #[inline]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor version.
    #[inline]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Returns the patch version.
    #[inline]
    pub const fn patch(self) -> u32 {
        self.patch
    }

    /// Returns `true` when this version has the same major component as
    /// `other`.
    ///
    /// Major-version compatibility is only a coarse classification. Concrete
    /// format implementations must still perform feature/capability checks.
    #[inline]
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns `true` when this version is older than `other`.
    #[inline]
    pub const fn is_older_than(self, other: Self) -> bool {
        self.major < other.major
            || (self.major == other.major && self.minor < other.minor)
            || (self.major == other.major
                && self.minor == other.minor
                && self.patch < other.patch)
    }

    /// Returns `true` when this version is newer than `other`.
    #[inline]
    pub const fn is_newer_than(self, other: Self) -> bool {
        self.major > other.major
            || (self.major == other.major && self.minor > other.minor)
            || (self.major == other.major
                && self.minor == other.minor
                && self.patch > other.patch)
    }
}

impl fmt::Display for FormatVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A single capability that a frontend format may advertise.
///
/// Capabilities describe what a format implementation can represent, import,
/// or export. They do **not** assert that every program using the feature can
/// necessarily be lowered to the canonical Zamani IR.
///
/// This distinction is important:
///
/// ```text
/// Format capability
///        ≠
/// IR representability
/// ```
///
/// Lowering remains the responsibility of the importer/lowering boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FormatCapability {
    /// The format can be imported into Zamani.
    Import,

    /// The format can be exported from Zamani.
    Export,

    /// The format supports parameterized quantum operations.
    Parameters,

    /// The format supports measurement operations.
    Measurements,

    /// The format supports reset operations.
    Reset,

    /// The format supports barriers or equivalent ordering constructs.
    Barriers,

    /// The format supports user-defined gates.
    GateDefinitions,

    /// The format supports classical computation.
    ClassicalComputation,

    /// The format supports classical control flow.
    ClassicalControl,

    /// The format supports conditional execution.
    Conditionals,

    /// The format supports loops.
    Loops,

    /// The format supports subroutines/functions.
    Subroutines,

    /// The format supports named includes/imports.
    Includes,

    /// The format supports explicit timing constructs.
    Timing,

    /// The format supports delay constructs.
    Delays,

    /// The format supports calibration constructs.
    Calibration,

    /// The format supports pulse-level constructs.
    Pulse,

    /// The format supports annotations or directives.
    Annotations,

    /// The format can represent classical integer values.
    ClassicalIntegers,

    /// The format can represent classical floating-point values.
    ClassicalFloats,

    /// The format can represent boolean values.
    ClassicalBooleans,

    /// The format can represent arrays.
    Arrays,

    /// The format can represent arbitrary classical expressions.
    Expressions,

    /// The format can preserve source-level symbolic names.
    SymbolicNames,

    /// The format supports explicit qubit/register declarations.
    RegisterDeclarations,

    /// The format supports dynamic resource allocation.
    DynamicResources,

    /// The format supports physical-qubit references.
    PhysicalQubits,
}

impl FormatCapability {
    /// Returns a stable machine-readable capability name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Import => "import",
            Self::Export => "export",
            Self::Parameters => "parameters",
            Self::Measurements => "measurements",
            Self::Reset => "reset",
            Self::Barriers => "barriers",
            Self::GateDefinitions => "gate-definitions",
            Self::ClassicalComputation => "classical-computation",
            Self::ClassicalControl => "classical-control",
            Self::Conditionals => "conditionals",
            Self::Loops => "loops",
            Self::Subroutines => "subroutines",
            Self::Includes => "includes",
            Self::Timing => "timing",
            Self::Delays => "delays",
            Self::Calibration => "calibration",
            Self::Pulse => "pulse",
            Self::Annotations => "annotations",
            Self::ClassicalIntegers => "classical-integers",
            Self::ClassicalFloats => "classical-floats",
            Self::ClassicalBooleans => "classical-booleans",
            Self::Arrays => "arrays",
            Self::Expressions => "expressions",
            Self::SymbolicNames => "symbolic-names",
            Self::RegisterDeclarations => "register-declarations",
            Self::DynamicResources => "dynamic-resources",
            Self::PhysicalQubits => "physical-qubits",
        }
    }
}

impl fmt::Display for FormatCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A deterministic set of capabilities advertised by a format implementation.
///
/// Internally this uses `BTreeSet`, not `HashSet`, so iteration and formatting
/// are deterministic across executions.
///
/// This is intentionally a value type rather than a global registry. A format
/// implementation can construct its capabilities without modifying central
/// frontend code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatCapabilities {
    capabilities: BTreeSet<FormatCapability>,
}

impl FormatCapabilities {
    /// Creates an empty capability set.
    pub const fn new() -> Self {
        Self {
            capabilities: BTreeSet::new(),
        }
    }

    /// Creates a capability set containing the supplied capabilities.
    pub fn from_iter<I>(capabilities: I) -> Result<Self, FormatError>
    where
        I: IntoIterator<Item = FormatCapability>,
    {
        let mut result = Self::new();

        for capability in capabilities {
            result.insert(capability)?;
        }

        Ok(result)
    }

    /// Adds one capability.
    pub fn insert(&mut self, capability: FormatCapability) -> Result<(), FormatError> {
        if !self.capabilities.contains(&capability)
            && self.capabilities.len() >= MAX_FORMAT_CAPABILITIES
        {
            return Err(FormatError::TooManyCapabilities {
                max: MAX_FORMAT_CAPABILITIES,
            });
        }

        self.capabilities.insert(capability);
        Ok(())
    }

    /// Removes one capability.
    pub fn remove(&mut self, capability: FormatCapability) -> bool {
        self.capabilities.remove(&capability)
    }

    /// Returns whether a capability is present.
    #[inline]
    pub fn supports(&self, capability: FormatCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Returns the number of capabilities.
    #[inline]
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    /// Returns whether no capabilities are advertised.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }

    /// Returns capabilities in deterministic order.
    pub fn iter(&self) -> impl Iterator<Item = FormatCapability> + '_ {
        self.capabilities.iter().copied()
    }

    /// Returns the underlying capabilities as a slice-like deterministic
    /// vector.
    pub fn to_vec(&self) -> Vec<FormatCapability> {
        self.iter().collect()
    }

    /// Returns a new set containing capabilities from both sets.
    pub fn union(&self, other: &Self) -> Result<Self, FormatError> {
        let mut result = self.clone();

        for capability in other.iter() {
            result.insert(capability)?;
        }

        Ok(result)
    }

    /// Returns whether this set contains every capability in `required`.
    pub fn contains_all(&self, required: &Self) -> bool {
        required
            .capabilities
            .iter()
            .all(|capability| self.capabilities.contains(capability))
    }
}

impl Default for FormatCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

/// Stable description of a frontend format.
///
/// `FrontendFormat` is deliberately descriptive rather than executable.
///
/// It tells the frontend system:
///
/// - which format this is;
/// - which version is represented;
/// - what the implementation claims to support.
///
/// It does **not** contain an importer or exporter object. Those belong to
/// `importer.rs` and `exporter.rs`.
///
/// This separation prevents the format identity layer from becoming coupled
/// to concrete implementations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontendFormat {
    id: FormatId,
    version: FormatVersion,
    capabilities: FormatCapabilities,
}

impl FrontendFormat {
    /// Creates a format descriptor.
    pub fn new(
        id: FormatId,
        version: FormatVersion,
        capabilities: FormatCapabilities,
    ) -> Self {
        Self {
            id,
            version,
            capabilities,
        }
    }

    /// Returns the stable format identifier.
    #[inline]
    pub fn id(&self) -> &FormatId {
        &self.id
    }

    /// Returns the format version.
    #[inline]
    pub const fn version(&self) -> FormatVersion {
        self.version
    }

    /// Returns the advertised capabilities.
    #[inline]
    pub fn capabilities(&self) -> &FormatCapabilities {
        &self.capabilities
    }

    /// Returns whether this format supports the requested capability.
    #[inline]
    pub fn supports(&self, capability: FormatCapability) -> bool {
        self.capabilities.supports(capability)
    }

    /// Returns whether this descriptor represents the same format family as
    /// another descriptor.
    #[inline]
    pub fn same_format(&self, other: &Self) -> bool {
        self.id == other.id
    }

    /// Returns whether the descriptor represents exactly the same format
    /// family and version.
    #[inline]
    pub fn same_revision(&self, other: &Self) -> bool {
        self.id == other.id && self.version == other.version
    }

    /// Returns a deterministic compatibility decision based on the format
    /// identity and requested capabilities.
    ///
    /// This deliberately does not make assumptions about semantic
    /// compatibility between arbitrary versions. Concrete formats may impose
    /// stricter rules.
    pub fn compatibility_with(
        &self,
        requested_version: FormatVersion,
        required_capabilities: &FormatCapabilities,
    ) -> FormatCompatibility {
        if self.version == requested_version {
            if self.capabilities.contains_all(required_capabilities) {
                FormatCompatibility::Exact
            } else {
                FormatCompatibility::ExactVersionMissingCapabilities
            }
        } else if self.version.same_major(requested_version) {
            if self.capabilities.contains_all(required_capabilities) {
                FormatCompatibility::SameMajorVersion
            } else {
                FormatCompatibility::SameMajorVersionMissingCapabilities
            }
        } else {
            FormatCompatibility::IncompatibleVersion
        }
    }
}

/// Result of comparing a format descriptor with a requested version/capability
/// set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormatCompatibility {
    /// Exact version and all required capabilities are available.
    Exact,

    /// Exact version exists, but one or more requested capabilities are
    /// unavailable.
    ExactVersionMissingCapabilities,

    /// A different version with the same major version is available and all
    /// requested capabilities are present.
    SameMajorVersion,

    /// Same major version is available, but requested capabilities are missing.
    SameMajorVersionMissingCapabilities,

    /// The available and requested major versions differ.
    IncompatibleVersion,
}

impl FormatCompatibility {
    /// Returns `true` only for an exact match.
    #[inline]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns `true` when the versions are considered potentially compatible.
    ///
    /// Capability deficiencies still need to be handled separately.
    #[inline]
    pub const fn same_major(self) -> bool {
        matches!(
            self,
            Self::Exact
                | Self::ExactVersionMissingCapabilities
                | Self::SameMajorVersion
                | Self::SameMajorVersionMissingCapabilities
        )
    }

    /// Returns `true` when requested capabilities are missing.
    #[inline]
    pub const fn missing_capabilities(self) -> bool {
        matches!(
            self,
            Self::ExactVersionMissingCapabilities
                | Self::SameMajorVersionMissingCapabilities
        )
    }

    /// Returns `true` when the version relationship is incompatible.
    #[inline]
    pub const fn incompatible_version(self) -> bool {
        matches!(self, Self::IncompatibleVersion)
    }
}

/// Errors produced while constructing format-independent descriptors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormatError {
    /// The format identifier was empty.
    EmptyFormatId,

    /// The format identifier exceeded the maximum permitted length.
    FormatIdTooLong {
        /// Maximum permitted length.
        max: usize,

        /// Actual length.
        actual: usize,
    },

    /// Format identifiers must be ASCII.
    NonAsciiFormatId,

    /// The first character was not an ASCII letter.
    InvalidFormatIdStart {
        /// Normalized invalid identifier.
        value: String,
    },

    /// The identifier contains an unsupported character.
    InvalidFormatIdCharacters {
        /// Normalized invalid identifier.
        value: String,
    },

    /// A capability set exceeded the defensive maximum.
    TooManyCapabilities {
        /// Maximum number of capabilities.
        max: usize,
    },
}

impl fmt::Display for FormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFormatId => {
                formatter.write_str("format identifier must not be empty")
            }
            Self::FormatIdTooLong { max, actual } => {
                write!(
                    formatter,
                    "format identifier exceeds maximum length: maximum={max}, actual={actual}"
                )
            }
            Self::NonAsciiFormatId => {
                formatter.write_str("format identifier must contain only ASCII characters")
            }
            Self::InvalidFormatIdStart { value } => {
                write!(
                    formatter,
                    "format identifier must begin with an ASCII letter: {value:?}"
                )
            }
            Self::InvalidFormatIdCharacters { value } => {
                write!(
                    formatter,
                    "format identifier contains unsupported characters: {value:?}"
                )
            }
            Self::TooManyCapabilities { max } => {
                write!(
                    formatter,
                    "format capability set exceeds maximum size: {max}"
                )
            }
        }
    }
}

impl std::error::Error for FormatError {}

/// Convenience constructor for a format descriptor.
///
/// This function intentionally returns a `Result` so future validation rules
/// can be introduced without changing callers that already use the fallible
/// constructor pattern.
pub fn define_format(
    id: impl AsRef<str>,
    version: FormatVersion,
    capabilities: FormatCapabilities,
) -> Result<FrontendFormat, FormatError> {
    Ok(FrontendFormat::new(
        FormatId::new(id)?,
        version,
        capabilities,
    ))
}

/// Creates a capability set from a fixed array.
///
/// This is useful for concrete formats while keeping construction concise.
///
/// # Example
///
/// ```
/// # use crate::quantum::frontend::format::{
/// #     capabilities, FormatCapability,
/// # };
/// let caps = capabilities(&[
///     FormatCapability::Import,
///     FormatCapability::Export,
/// ]);
/// assert!(caps.is_ok());
/// ```
pub fn capabilities(
    values: &[FormatCapability],
) -> Result<FormatCapabilities, FormatError> {
    FormatCapabilities::from_iter(values.iter().copied())
}

/// Compile-time assertion that `FormatVersion` remains cheaply copyable.
///
/// This is intentionally expressed through a function rather than relying on
/// unstable compile-time reflection.
const _: fn(FormatVersion) -> FormatVersion = |version| version;

/// Compile-time assertion for the `Infallible` import kept intentionally out
/// of the public API surface.
const _: fn(Result<(), Infallible>) -> Result<(), Infallible> = |value| value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_id_normalizes_ascii_case() {
        let id = FormatId::new("OpenQASM").expect("valid format id");

        assert_eq!(id.as_str(), "openqasm");
    }

    #[test]
    fn format_id_accepts_supported_identifier_characters() {
        assert!(FormatId::new("openqasm").is_ok());
        assert!(FormatId::new("qir").is_ok());
        assert!(FormatId::new("vendor-format").is_ok());
        assert!(FormatId::new("vendor_format").is_ok());
        assert!(FormatId::new("vendor.format").is_ok());
        assert!(FormatId::new("format2").is_ok());
    }

    #[test]
    fn format_id_rejects_invalid_values() {
        assert_eq!(
            FormatId::new("").expect_err("empty identifier must fail"),
            FormatError::EmptyFormatId
        );

        assert!(matches!(
            FormatId::new("1qasm"),
            Err(FormatError::InvalidFormatIdStart { .. })
        ));

        assert!(matches!(
            FormatId::new("open qasm"),
            Err(FormatError::InvalidFormatIdCharacters { .. })
        ));

        assert_eq!(
            FormatId::new("qåsm").expect_err("non-ascii identifier must fail"),
            FormatError::NonAsciiFormatId
        );
    }

    #[test]
    fn format_id_is_case_insensitive_at_construction() {
        let lower = FormatId::new("openqasm").expect("valid");
        let upper = FormatId::new("OPENQASM").expect("valid");

        assert_eq!(lower, upper);
    }

    #[test]
    fn format_version_orders_correctly() {
        let v30 = FormatVersion::new(3, 0, 0);
        let v31 = FormatVersion::new(3, 1, 0);
        let v310 = FormatVersion::new(3, 1, 0);
        let v40 = FormatVersion::new(4, 0, 0);

        assert!(v30.is_older_than(v31));
        assert!(v40.is_newer_than(v31));
        assert_eq!(v31, v310);
        assert!(v31.same_major(v30));
        assert!(!v31.same_major(v40));
    }

    #[test]
    fn format_version_formats_deterministically() {
        let version = FormatVersion::new(3, 1, 0);

        assert_eq!(version.to_string(), "3.1.0");
    }

    #[test]
    fn capabilities_are_deterministic() {
        let caps = capabilities(&[
            FormatCapability::Export,
            FormatCapability::Import,
            FormatCapability::Measurements,
        ])
        .expect("valid capabilities");

        let names: Vec<_> = caps.iter().map(FormatCapability::as_str).collect();

        assert_eq!(
            names,
            vec!["import", "export", "measurements"]
        );
    }

    #[test]
    fn capability_sets_support_membership_queries() {
        let caps = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Measurements,
        ])
        .expect("valid capabilities");

        assert!(caps.supports(FormatCapability::Import));
        assert!(caps.supports(FormatCapability::Export));
        assert!(caps.supports(FormatCapability::Measurements));
        assert!(!caps.supports(FormatCapability::Pulse));
    }

    #[test]
    fn duplicate_capabilities_are_idempotent() {
        let caps = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Import,
            FormatCapability::Export,
        ])
        .expect("valid capabilities");

        assert_eq!(caps.len(), 2);
    }

    #[test]
    fn capability_union_is_deterministic() {
        let first =
            capabilities(&[FormatCapability::Import]).expect("valid capabilities");

        let second =
            capabilities(&[FormatCapability::Export]).expect("valid capabilities");

        let combined = first.union(&second).expect("union must succeed");

        assert!(combined.supports(FormatCapability::Import));
        assert!(combined.supports(FormatCapability::Export));
        assert_eq!(combined.len(), 2);
    }

    #[test]
    fn contains_all_checks_required_capabilities() {
        let available = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Measurements,
        ])
        .expect("valid capabilities");

        let required =
            capabilities(&[FormatCapability::Import, FormatCapability::Export])
                .expect("valid capabilities");

        assert!(available.contains_all(&required));
    }

    #[test]
    fn contains_all_rejects_missing_capabilities() {
        let available =
            capabilities(&[FormatCapability::Import]).expect("valid capabilities");

        let required =
            capabilities(&[FormatCapability::Import, FormatCapability::Export])
                .expect("valid capabilities");

        assert!(!available.contains_all(&required));
    }

    #[test]
    fn frontend_format_preserves_identity_version_and_capabilities() {
        let caps = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Parameters,
        ])
        .expect("valid capabilities");

        let format =
            define_format("OpenQASM", FormatVersion::major_minor(3, 1), caps)
                .expect("valid format");

        assert_eq!(format.id().as_str(), "openqasm");
        assert_eq!(format.version(), FormatVersion::new(3, 1, 0));
        assert!(format.supports(FormatCapability::Parameters));
    }

    #[test]
    fn exact_compatibility_is_reported() {
        let caps = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
        ])
        .expect("valid capabilities");

        let format =
            define_format("qir", FormatVersion::new(1, 0, 0), caps.clone())
                .expect("valid format");

        assert_eq!(
            format.compatibility_with(FormatVersion::new(1, 0, 0), &caps),
            FormatCompatibility::Exact
        );
    }

    #[test]
    fn exact_version_with_missing_capability_is_reported() {
        let available =
            capabilities(&[FormatCapability::Import]).expect("valid capabilities");

        let required =
            capabilities(&[FormatCapability::Import, FormatCapability::Export])
                .expect("valid capabilities");

        let format =
            define_format("qir", FormatVersion::new(1, 0, 0), available)
                .expect("valid format");

        assert_eq!(
            format.compatibility_with(FormatVersion::new(1, 0, 0), &required),
            FormatCompatibility::ExactVersionMissingCapabilities
        );
    }

    #[test]
    fn same_major_version_is_potentially_compatible() {
        let caps =
            capabilities(&[FormatCapability::Import]).expect("valid capabilities");

        let format =
            define_format("openqasm", FormatVersion::new(3, 1, 0), caps.clone())
                .expect("valid format");

        assert_eq!(
            format.compatibility_with(FormatVersion::new(3, 2, 0), &caps),
            FormatCompatibility::SameMajorVersion
        );
    }

    #[test]
    fn different_major_versions_are_incompatible() {
        let caps =
            capabilities(&[FormatCapability::Import]).expect("valid capabilities");

        let format =
            define_format("openqasm", FormatVersion::new(3, 1, 0), caps.clone())
                .expect("valid format");

        assert_eq!(
            format.compatibility_with(FormatVersion::new(4, 0, 0), &caps),
            FormatCompatibility::IncompatibleVersion
        );
    }

    #[test]
    fn compatibility_helpers_are_consistent() {
        assert!(FormatCompatibility::Exact.is_exact());
        assert!(!FormatCompatibility::Exact.missing_capabilities());
        assert!(!FormatCompatibility::Exact.incompatible_version());

        assert!(FormatCompatibility::ExactVersionMissingCapabilities.same_major());
        assert!(
            FormatCompatibility::ExactVersionMissingCapabilities
                .missing_capabilities()
        );

        assert!(FormatCompatibility::IncompatibleVersion.incompatible_version());
        assert!(!FormatCompatibility::IncompatibleVersion.same_major());
    }

    #[test]
    fn format_identity_is_independent_of_version() {
        let caps = FormatCapabilities::new();

        let first =
            define_format("openqasm", FormatVersion::new(3, 0, 0), caps.clone())
                .expect("valid format");

        let second =
            define_format("openqasm", FormatVersion::new(3, 1, 0), caps)
                .expect("valid format");

        assert!(first.same_format(&second));
        assert!(!first.same_revision(&second));
    }

    #[test]
    fn different_formats_are_not_the_same_format() {
        let caps = FormatCapabilities::new();

        let openqasm =
            define_format("openqasm", FormatVersion::new(3, 1, 0), caps.clone())
                .expect("valid format");

        let qir =
            define_format("qir", FormatVersion::new(1, 0, 0), caps)
                .expect("valid format");

        assert!(!openqasm.same_format(&qir));
    }

    #[test]
    fn capability_names_are_stable() {
        assert_eq!(FormatCapability::Import.as_str(), "import");
        assert_eq!(FormatCapability::Export.as_str(), "export");
        assert_eq!(
            FormatCapability::GateDefinitions.as_str(),
            "gate-definitions"
        );
        assert_eq!(FormatCapability::ClassicalControl.as_str(), "classical-control");
        assert_eq!(FormatCapability::PhysicalQubits.as_str(), "physical-qubits");
    }
}