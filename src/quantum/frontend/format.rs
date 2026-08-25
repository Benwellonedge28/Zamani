//! Quantum frontend format contracts.
//!
//! This module defines the format-independent identity, version, capability,
//! and compatibility model used by the Zamani quantum frontend.
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
//! The dependency direction is:
//!
//! ```text
//! concrete format
//!       │
//!       ▼
//! frontend::format
//!       │
//!       ├──────────────► importer contract
//!       ├──────────────► exporter contract
//!       └──────────────► lowering contract
//!
//! concrete format
//!       │
//!       ▼
//! frontend
//!       │
//!       ▼
//! Zamani Quantum IR
//! ```
//!
//! A format must never depend on another format.
//!
//! # Important semantic distinction
//!
//! A format capability describes what a format can express or transport.
//! It does NOT guarantee that every construct expressible by that format can
//! be represented by the canonical Zamani Quantum IR.
//!
//! Therefore:
//!
//! ```text
//! format capability
//!        ≠
//! IR representability
//!        ≠
//! backend capability
//! ```
//!
//! Those questions belong to separate layers.
//!
//! # Design goals
//!
//! This module provides:
//!
//! - stable format identity;
//! - explicit version identity;
//! - deterministic capability declarations;
//! - feature-level capability queries;
//! - safe format-aware compatibility checks;
//! - format-independent API contracts;
//! - bounded metadata;
//! - deterministic iteration;
//! - no stringly-typed capability checks;
//! - no concrete-format dependencies;
//! - no filesystem/network/process/hardware access.
//!
//! # Rust compatibility
//!
//! This implementation targets:
//!
//! - Rust 1.97.1
//! - Rust 2021
//!
//! # Stability
//!
//! The following types form the stable frontend format contract:
//!
//! - [`FormatId`]
//! - [`FormatVersion`]
//! - [`FormatCapability`]
//! - [`FormatCapabilities`]
//! - [`FrontendFormat`]
//! - [`FormatCompatibility`]
//!
//! Concrete format implementations should not expose their lexer, parser,
//! validator, AST, or other internal implementation details through this
//! module.

use core::fmt;
use core::str::FromStr;
use std::collections::BTreeSet;

/// Maximum number of bytes permitted in a format identifier.
///
/// Format identifiers are protocol identifiers rather than arbitrary user
/// strings. The bound prevents accidental or maliciously oversized metadata.
pub const MAX_FORMAT_ID_LENGTH: usize = 128;

/// Maximum number of capabilities permitted in one capability set.
///
/// The value is deliberately much larger than the number of capabilities
/// currently defined, allowing future extensions without changing the
/// defensive invariant.
pub const MAX_FORMAT_CAPABILITIES: usize = 256;

/// Result used by fallible format-contract constructors.
pub type FormatResult<T> = Result<T, FormatError>;

/// Stable identifier for a frontend format.
///
/// A `FormatId` identifies a format family, not a particular version.
///
/// Examples:
///
/// ```text
/// openqasm
/// qir
/// quil
/// ```
///
/// Version information belongs to [`FormatVersion`].
///
/// # Canonical representation
///
/// Format IDs:
///
/// - are ASCII;
/// - are normalized to lowercase;
/// - begin with an ASCII letter;
/// - may contain ASCII letters;
/// - may contain ASCII digits;
/// - may contain `-`, `_`, and `.` after the first character;
/// - contain no whitespace;
/// - contain no control characters;
/// - do not exceed [`MAX_FORMAT_ID_LENGTH`] bytes.
///
/// The canonical representation is returned by [`FormatId::as_str`].
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FormatId(String);

impl FormatId {
    /// Creates and validates a format identifier.
    ///
    /// ASCII letters are normalized to lowercase.
    ///
    /// # Errors
    ///
    /// Returns [`FormatError`] when the identifier is empty, too long,
    /// non-ASCII, begins with an invalid character, or contains unsupported
    /// characters.
    pub fn new(value: impl AsRef<str>) -> FormatResult<Self> {
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

        if !normalized.bytes().all(|byte| {
            byte.is_ascii_alphanumeric()
                || byte == b'-'
                || byte == b'_'
                || byte == b'.'
        }) {
            return Err(FormatError::InvalidFormatIdCharacters {
                value: normalized,
            });
        }

        Ok(Self(normalized))
    }

    /// Returns the canonical identifier.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
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
/// Versions are represented numerically so callers can perform deterministic
/// comparisons without parsing version strings.
///
/// For example:
///
/// ```text
/// 3.0.0
/// 3.1.0
/// 4.0.0
/// ```
///
/// A two-component version such as `3.1` is represented as `3.1.0`.
///
/// Pre-release and build metadata are intentionally outside this primitive
/// contract. Concrete formats that require richer version semantics may keep
/// their richer representation internally and expose a normalized
/// [`FormatVersion`] here.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FormatVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl FormatVersion {
    /// Creates a complete format version.
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

    /// Returns the major component.
    #[inline]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor component.
    #[inline]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Returns the patch component.
    #[inline]
    pub const fn patch(self) -> u32 {
        self.patch
    }

    /// Returns whether both versions have the same major component.
    ///
    /// This is only a coarse compatibility signal. It does not mean that the
    /// two versions are semantically interchangeable.
    #[inline]
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns whether this version is older than `other`.
    #[inline]
    pub const fn is_older_than(self, other: Self) -> bool {
        self < other
    }

    /// Returns whether this version is newer than `other`.
    #[inline]
    pub const fn is_newer_than(self, other: Self) -> bool {
        self > other
    }
}

impl fmt::Display for FormatVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

/// A capability advertised by a frontend format.
///
/// Capabilities describe format-level expressive or interchange features.
///
/// They do not guarantee that:
///
/// - the Zamani IR can represent every instance;
/// - a backend can execute the construct;
/// - lowering is lossless;
/// - optimization preserves the construct;
/// - export is possible for every IR operation.
///
/// Those concerns belong to their respective layers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FormatCapability {
    /// The format can be imported into Zamani.
    Import,

    /// The format can be exported from Zamani.
    Export,

    /// The format supports parameterized operations.
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

    /// The format supports include/import declarations.
    Includes,

    /// The format supports timing constructs.
    Timing,

    /// The format supports delay constructs.
    Delays,

    /// The format supports calibration constructs.
    Calibration,

    /// The format supports pulse-level constructs.
    Pulse,

    /// The format supports annotations/directives/pragmas.
    Annotations,

    /// The format supports classical integer values.
    ClassicalIntegers,

    /// The format supports classical floating-point values.
    ClassicalFloats,

    /// The format supports boolean values.
    ClassicalBooleans,

    /// The format supports arrays.
    Arrays,

    /// The format supports classical expressions.
    Expressions,

    /// The format preserves symbolic identifiers.
    SymbolicNames,

    /// The format supports explicit qubit/bit/register declarations.
    RegisterDeclarations,

    /// The format supports dynamic resource allocation.
    DynamicResources,

    /// The format supports explicit physical-qubit references.
    PhysicalQubits,
}

impl FormatCapability {
    /// Returns the stable machine-readable name of the capability.
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

/// Deterministic set of capabilities advertised by a format.
///
/// A `BTreeSet` is deliberately used instead of `HashSet` so externally
/// observable iteration and formatting remain deterministic between runs.
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

    /// Creates a capability set from an iterator.
    ///
    /// Duplicate capabilities are harmless and are collapsed.
    pub fn from_iter<I>(capabilities: I) -> FormatResult<Self>
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
    ///
    /// Duplicate insertion is idempotent.
    pub fn insert(&mut self, capability: FormatCapability) -> FormatResult<()> {
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

    /// Removes a capability.
    ///
    /// Returns `true` when the capability existed.
    pub fn remove(&mut self, capability: FormatCapability) -> bool {
        self.capabilities.remove(&capability)
    }

    /// Returns whether the capability exists.
    #[inline]
    pub fn supports(&self, capability: FormatCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Returns the number of capabilities.
    #[inline]
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    /// Returns whether the capability set is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }

    /// Returns capabilities in deterministic enum order.
    pub fn iter(&self) -> impl Iterator<Item = FormatCapability> + '_ {
        self.capabilities.iter().copied()
    }

    /// Returns the capabilities as a deterministic vector.
    pub fn to_vec(&self) -> Vec<FormatCapability> {
        self.iter().collect()
    }

    /// Returns a new capability set containing the union of both sets.
    pub fn union(&self, other: &Self) -> FormatResult<Self> {
        let mut result = self.clone();

        for capability in other.iter() {
            result.insert(capability)?;
        }

        Ok(result)
    }

    /// Returns whether every capability in `required` is available.
    pub fn contains_all(&self, required: &Self) -> bool {
        required
            .capabilities
            .iter()
            .all(|capability| self.capabilities.contains(capability))
    }

    /// Returns the capabilities that are present in `required` but absent from
    /// this set.
    pub fn missing_from(&self, required: &Self) -> Vec<FormatCapability> {
        required
            .capabilities
            .iter()
            .copied()
            .filter(|capability| !self.capabilities.contains(capability))
            .collect()
    }
}

impl Default for FormatCapabilities {
    fn default() -> Self {
        Self::new()
    }
}

/// Stable description of one frontend format revision.
///
/// `FrontendFormat` is descriptive only.
///
/// It does not contain:
///
/// - a parser;
/// - a lexer;
/// - an importer;
/// - an exporter;
/// - an AST;
/// - a validator;
/// - a Quantum IR object.
///
/// Those belong to other layers.
///
/// This separation prevents the format contract from becoming coupled to
/// concrete frontend implementations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontendFormat {
    id: FormatId,
    version: FormatVersion,
    capabilities: FormatCapabilities,
}

impl FrontendFormat {
    /// Creates a format descriptor from already validated components.
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

    /// Returns the format-family identifier.
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

    /// Returns whether this format supports a capability.
    #[inline]
    pub fn supports(&self, capability: FormatCapability) -> bool {
        self.capabilities.supports(capability)
    }

    /// Returns whether two descriptors belong to the same format family.
    ///
    /// Version differences do not affect this result.
    #[inline]
    pub fn same_format(&self, other: &Self) -> bool {
        self.id == other.id
    }

    /// Returns whether two descriptors identify the exact same format revision.
    #[inline]
    pub fn same_revision(&self, other: &Self) -> bool {
        self.id == other.id && self.version == other.version
    }

    /// Compares this descriptor with another complete format descriptor.
    ///
    /// This is the preferred compatibility API because it verifies the
    /// format identity as well as version and capabilities.
    pub fn compatibility_with_format(
        &self,
        requested: &FrontendFormat,
        required_capabilities: &FormatCapabilities,
    ) -> FormatCompatibility {
        if self.id != requested.id {
            return FormatCompatibility::DifferentFormat;
        }

        self.compatibility_with(
            requested.version,
            required_capabilities,
        )
    }

    /// Compares this descriptor with a requested version and required
    /// capabilities.
    ///
    /// This method assumes the caller has already established that both
    /// descriptors refer to the same format family.
    ///
    /// For a comparison where format identity must be enforced, use
    /// [`FrontendFormat::compatibility_with_format`].
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

    /// Returns the capabilities required by the caller but unavailable in this
    /// descriptor.
    pub fn missing_capabilities(
        &self,
        required: &FormatCapabilities,
    ) -> Vec<FormatCapability> {
        self.capabilities.missing_from(required)
    }
}

/// Result of comparing a format descriptor with another format/version and a
/// required capability set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormatCompatibility {
    /// The requested format family and exact version are available and all
    /// required capabilities are present.
    Exact,

    /// The exact requested version exists but one or more required
    /// capabilities are unavailable.
    ExactVersionMissingCapabilities,

    /// The format family matches and a different version with the same major
    /// version is available with all required capabilities.
    SameMajorVersion,

    /// The format family matches and a same-major version is available, but
    /// one or more required capabilities are unavailable.
    SameMajorVersionMissingCapabilities,

    /// The format family is different.
    ///
    /// This state is intentionally distinct from an incompatible version.
    /// Version numbers have meaning only within a format family.
    DifferentFormat,

    /// The same format family was requested but the major versions differ.
    IncompatibleVersion,
}

impl FormatCompatibility {
    /// Returns `true` only when the requested format and version are exact and
    /// all required capabilities are present.
    #[inline]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns whether the format family is the same and the version relationship
    /// is exact or same-major.
    ///
    /// Capability deficiencies do not make this return `false`.
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

    /// Returns whether one or more required capabilities are unavailable.
    #[inline]
    pub const fn missing_capabilities(self) -> bool {
        matches!(
            self,
            Self::ExactVersionMissingCapabilities
                | Self::SameMajorVersionMissingCapabilities
        )
    }

    /// Returns whether the requested format family differs from the available
    /// format family.
    #[inline]
    pub const fn different_format(self) -> bool {
        matches!(self, Self::DifferentFormat)
    }

    /// Returns whether the same format family was requested but the major
    /// versions are incompatible.
    #[inline]
    pub const fn incompatible_version(self) -> bool {
        matches!(self, Self::IncompatibleVersion)
    }

    /// Returns whether this result can be accepted as an exact supported
    /// request without further negotiation.
    #[inline]
    pub const fn is_acceptable(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns whether this result requires capability negotiation or explicit
    /// unsupported-feature handling.
    #[inline]
    pub const fn requires_negotiation(self) -> bool {
        matches!(
            self,
            Self::ExactVersionMissingCapabilities
                | Self::SameMajorVersion
                | Self::SameMajorVersionMissingCapabilities
        )
    }

    /// Returns whether the request cannot be satisfied by this format
    /// descriptor.
    #[inline]
    pub const fn is_incompatible(self) -> bool {
        matches!(
            self,
            Self::DifferentFormat | Self::IncompatibleVersion
        )
    }
}

/// Errors produced while constructing format-independent descriptors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormatError {
    /// The format identifier is empty.
    EmptyFormatId,

    /// The format identifier is longer than the permitted limit.
    FormatIdTooLong {
        /// Maximum permitted length.
        max: usize,

        /// Actual identifier length.
        actual: usize,
    },

    /// The format identifier contains non-ASCII characters.
    NonAsciiFormatId,

    /// The first identifier character is not an ASCII letter.
    InvalidFormatIdStart {
        /// Invalid normalized identifier.
        value: String,
    },

    /// The identifier contains unsupported characters.
    InvalidFormatIdCharacters {
        /// Invalid normalized identifier.
        value: String,
    },

    /// The capability set exceeded its defensive maximum.
    TooManyCapabilities {
        /// Maximum permitted capability count.
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
                    "format identifier exceeds maximum length: \
                     maximum={max}, actual={actual}"
                )
            }

            Self::NonAsciiFormatId => {
                formatter.write_str(
                    "format identifier must contain only ASCII characters",
                )
            }

            Self::InvalidFormatIdStart { value } => {
                write!(
                    formatter,
                    "format identifier must begin with an ASCII \
                     letter: {value:?}"
                )
            }

            Self::InvalidFormatIdCharacters { value } => {
                write!(
                    formatter,
                    "format identifier contains unsupported \
                     characters: {value:?}"
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

/// Defines a validated frontend format descriptor.
///
/// This is the preferred constructor for concrete format modules because it
/// validates the externally supplied format identifier before constructing the
/// descriptor.
pub fn define_format(
    id: impl AsRef<str>,
    version: FormatVersion,
    capabilities: FormatCapabilities,
) -> FormatResult<FrontendFormat> {
    Ok(FrontendFormat::new(
        FormatId::new(id)?,
        version,
        capabilities,
    ))
}

/// Convenience constructor for deterministic capability sets.
pub fn capabilities(
    values: &[FormatCapability],
) -> FormatResult<FormatCapabilities> {
    FormatCapabilities::from_iter(values.iter().copied())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_id_normalizes_ascii_case() {
        let id = FormatId::new("OpenQASM").expect("valid format ID");

        assert_eq!(id.as_str(), "openqasm");
    }

    #[test]
    fn format_id_accepts_valid_identifiers() {
        assert!(FormatId::new("openqasm").is_ok());
        assert!(FormatId::new("qir").is_ok());
        assert!(FormatId::new("quil").is_ok());
        assert!(FormatId::new("vendor-format").is_ok());
        assert!(FormatId::new("vendor_format").is_ok());
        assert!(FormatId::new("vendor.format").is_ok());
        assert!(FormatId::new("format2").is_ok());
        assert!(FormatId::new("a").is_ok());
    }

    #[test]
    fn format_id_rejects_empty_identifier() {
        assert_eq!(
            FormatId::new("")
                .expect_err("empty ID must fail"),
            FormatError::EmptyFormatId
        );
    }

    #[test]
    fn format_id_rejects_identifier_starting_with_digit() {
        assert!(matches!(
            FormatId::new("1qasm"),
            Err(FormatError::InvalidFormatIdStart { .. })
        ));
    }

    #[test]
    fn format_id_rejects_identifier_starting_with_symbol() {
        assert!(matches!(
            FormatId::new("_qasm"),
            Err(FormatError::InvalidFormatIdStart { .. })
        ));
    }

    #[test]
    fn format_id_rejects_whitespace() {
        assert!(matches!(
            FormatId::new("open qasm"),
            Err(FormatError::InvalidFormatIdCharacters { .. })
        ));
    }

    #[test]
    fn format_id_rejects_non_ascii() {
        assert_eq!(
            FormatId::new("qåsm")
                .expect_err("non-ASCII ID must fail"),
            FormatError::NonAsciiFormatId
        );
    }

    #[test]
    fn format_id_rejects_control_characters() {
        assert!(matches!(
            FormatId::new("qasm\n"),
            Err(FormatError::InvalidFormatIdCharacters { .. })
        ));
    }

    #[test]
    fn format_id_rejects_overlong_identifier() {
        let value = "a".repeat(MAX_FORMAT_ID_LENGTH + 1);

        assert!(matches!(
            FormatId::new(value),
            Err(FormatError::FormatIdTooLong { .. })
        ));
    }

    #[test]
    fn equivalent_identifier_case_has_same_identity() {
        let lower = FormatId::new("openqasm").expect("valid");
        let upper = FormatId::new("OPENQASM").expect("valid");

        assert_eq!(lower, upper);
    }

    #[test]
    fn format_id_display_is_canonical() {
        let id = FormatId::new("OpenQASM").expect("valid");

        assert_eq!(id.to_string(), "openqasm");
    }

    #[test]
    fn format_id_from_str_works() {
        let id: FormatId = "OpenQASM"
            .parse()
            .expect("valid format ID");

        assert_eq!(id.as_str(), "openqasm");
    }

    #[test]
    fn format_version_constructor_is_correct() {
        let version = FormatVersion::new(3, 1, 2);

        assert_eq!(version.major(), 3);
        assert_eq!(version.minor(), 1);
        assert_eq!(version.patch(), 2);
    }

    #[test]
    fn format_version_major_minor_sets_zero_patch() {
        let version = FormatVersion::major_minor(3, 1);

        assert_eq!(version, FormatVersion::new(3, 1, 0));
    }

    #[test]
    fn format_version_ordering_is_numeric() {
        let v300 = FormatVersion::new(3, 0, 0);
        let v301 = FormatVersion::new(3, 0, 1);
        let v310 = FormatVersion::new(3, 1, 0);
        let v400 = FormatVersion::new(4, 0, 0);

        assert!(v300 < v301);
        assert!(v301 < v310);
        assert!(v310 < v400);
    }

    #[test]
    fn format_version_helpers_are_correct() {
        let v30 = FormatVersion::new(3, 0, 0);
        let v31 = FormatVersion::new(3, 1, 0);
        let v40 = FormatVersion::new(4, 0, 0);

        assert!(v30.is_older_than(v31));
        assert!(v40.is_newer_than(v31));

        assert!(v30.same_major(v31));
        assert!(!v30.same_major(v40));
    }

    #[test]
    fn format_version_display_is_deterministic() {
        assert_eq!(
            FormatVersion::new(3, 1, 0).to_string(),
            "3.1.0"
        );
    }

    #[test]
    fn capabilities_are_deterministic() {
        let caps = capabilities(&[
            FormatCapability::Export,
            FormatCapability::Import,
            FormatCapability::Measurements,
        ])
        .expect("valid capability set");

        let names: Vec<&str> = caps
            .iter()
            .map(FormatCapability::as_str)
            .collect();

        assert_eq!(
            names,
            vec![
                "import",
                "export",
                "measurements",
            ]
        );
    }

    #[test]
    fn capabilities_support_membership_queries() {
        let caps = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Measurements,
        ])
        .expect("valid capability set");

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
            FormatCapability::Export,
        ])
        .expect("valid capability set");

        assert_eq!(caps.len(), 2);
    }

    #[test]
    fn removing_capability_is_correct() {
        let mut caps = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
        ])
        .expect("valid capability set");

        assert!(caps.remove(FormatCapability::Import));
        assert!(!caps.supports(FormatCapability::Import));
        assert!(!caps.remove(FormatCapability::Import));
    }

    #[test]
    fn capability_union_is_correct() {
        let first =
            capabilities(&[FormatCapability::Import])
                .expect("valid capability set");

        let second =
            capabilities(&[FormatCapability::Export])
                .expect("valid capability set");

        let combined =
            first.union(&second)
                .expect("union must succeed");

        assert!(combined.supports(FormatCapability::Import));
        assert!(combined.supports(FormatCapability::Export));
        assert_eq!(combined.len(), 2);
    }

    #[test]
    fn contains_all_accepts_complete_requirement() {
        let available = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Measurements,
        ])
        .expect("valid");

        let required = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
        ])
        .expect("valid");

        assert!(available.contains_all(&required));
    }

    #[test]
    fn contains_all_rejects_missing_requirement() {
        let available =
            capabilities(&[FormatCapability::Import])
                .expect("valid");

        let required = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
        ])
        .expect("valid");

        assert!(!available.contains_all(&required));
    }

    #[test]
    fn missing_from_returns_only_missing_capabilities() {
        let available =
            capabilities(&[FormatCapability::Import])
                .expect("valid");

        let required = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Measurements,
        ])
        .expect("valid");

        assert_eq!(
            available.missing_from(&required),
            vec![
                FormatCapability::Export,
                FormatCapability::Measurements,
            ]
        );
    }

    #[test]
    fn frontend_format_preserves_identity() {
        let caps = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
            FormatCapability::Parameters,
        ])
        .expect("valid");

        let format = define_format(
            "OpenQASM",
            FormatVersion::major_minor(3, 1),
            caps,
        )
        .expect("valid format");

        assert_eq!(format.id().as_str(), "openqasm");
        assert_eq!(
            format.version(),
            FormatVersion::new(3, 1, 0)
        );
        assert!(format.supports(FormatCapability::Parameters));
    }

    #[test]
    fn same_format_ignores_version() {
        let caps = FormatCapabilities::new();

        let first = define_format(
            "openqasm",
            FormatVersion::new(3, 0, 0),
            caps.clone(),
        )
        .expect("valid");

        let second = define_format(
            "openqasm",
            FormatVersion::new(3, 1, 0),
            caps,
        )
        .expect("valid");

        assert!(first.same_format(&second));
        assert!(!first.same_revision(&second));
    }

    #[test]
    fn different_format_is_detected() {
        let caps = FormatCapabilities::new();

        let openqasm = define_format(
            "openqasm",
            FormatVersion::new(3, 1, 0),
            caps.clone(),
        )
        .expect("valid");

        let qir = define_format(
            "qir",
            FormatVersion::new(1, 0, 0),
            caps,
        )
        .expect("valid");

        assert!(!openqasm.same_format(&qir));
        assert!(!openqasm.same_revision(&qir));
    }

    #[test]
    fn exact_compatibility_requires_capabilities() {
        let available = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
        ])
        .expect("valid");

        let required = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
        ])
        .expect("valid");

        let format = define_format(
            "openqasm",
            FormatVersion::new(3, 1, 0),
            available,
        )
        .expect("valid");

        assert_eq!(
            format.compatibility_with(
                FormatVersion::new(3, 1, 0),
                &required,
            ),
            FormatCompatibility::Exact
        );
    }

    #[test]
    fn exact_version_with_missing_capability_is_reported() {
        let available =
            capabilities(&[FormatCapability::Import])
                .expect("valid");

        let required = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
        ])
        .expect("valid");

        let format = define_format(
            "openqasm",
            FormatVersion::new(3, 1, 0),
            available,
        )
        .expect("valid");

        assert_eq!(
            format.compatibility_with(
                FormatVersion::new(3, 1, 0),
                &required,
            ),
            FormatCompatibility::ExactVersionMissingCapabilities
        );
    }

    #[test]
    fn same_major_version_is_reported() {
        let caps =
            capabilities(&[FormatCapability::Import])
                .expect("valid");

        let format = define_format(
            "openqasm",
            FormatVersion::new(3, 1, 0),
            caps.clone(),
        )
        .expect("valid");

        assert_eq!(
            format.compatibility_with(
                FormatVersion::new(3, 2, 0),
                &caps,
            ),
            FormatCompatibility::SameMajorVersion
        );
    }

    #[test]
    fn same_major_missing_capability_is_reported() {
        let available =
            capabilities(&[FormatCapability::Import])
                .expect("valid");

        let required = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
        ])
        .expect("valid");

        let format = define_format(
            "openqasm",
            FormatVersion::new(3, 1, 0),
            available,
        )
        .expect("valid");

        assert_eq!(
            format.compatibility_with(
                FormatVersion::new(3, 2, 0),
                &required,
            ),
            FormatCompatibility::SameMajorVersionMissingCapabilities
        );
    }

    #[test]
    fn different_major_version_is_incompatible() {
        let caps =
            capabilities(&[FormatCapability::Import])
                .expect("valid");

        let format = define_format(
            "openqasm",
            FormatVersion::new(3, 1, 0),
            caps.clone(),
        )
        .expect("valid");

        assert_eq!(
            format.compatibility_with(
                FormatVersion::new(4, 0, 0),
                &caps,
            ),
            FormatCompatibility::IncompatibleVersion
        );
    }

    #[test]
    fn compatibility_with_format_rejects_different_format() {
        let caps =
            capabilities(&[FormatCapability::Import])
                .expect("valid");

        let openqasm = define_format(
            "openqasm",
            FormatVersion::new(3, 1, 0),
            caps.clone(),
        )
        .expect("valid");

        let qir = define_format(
            "qir",
            FormatVersion::new(3, 1, 0),
            caps.clone(),
        )
        .expect("valid");

        assert_eq!(
            openqasm.compatibility_with_format(
                &qir,
                &caps,
            ),
            FormatCompatibility::DifferentFormat
        );
    }

    #[test]
    fn compatibility_with_format_accepts_exact_match() {
        let caps = capabilities(&[
            FormatCapability::Import,
            FormatCapability::Export,
        ])
        .expect("valid");

        let first = define_format(
            "openqasm",
            FormatVersion::new(3, 1, 0),
            caps.clone(),
        )
        .expect("valid");

        let second = define_format(
            "openqasm",
            FormatVersion::new(3, 1, 0),
            caps.clone(),
        )
        .expect("valid");

        assert_eq!(
            first.compatibility_with_format(
                &second,
                &caps,
            ),
            FormatCompatibility::Exact
        );
    }

    #[test]
    fn compatibility_helpers_are_consistent() {
        assert!(FormatCompatibility::Exact.is_exact());
        assert!(FormatCompatibility::Exact.is_acceptable());
        assert!(!FormatCompatibility::Exact.missing_capabilities());
        assert!(!FormatCompatibility::Exact.incompatible_version());
        assert!(!FormatCompatibility::Exact.different_format());

        assert!(
            FormatCompatibility::ExactVersionMissingCapabilities
                .missing_capabilities()
        );

        assert!(
            FormatCompatibility::SameMajorVersion.same_major()
        );

        assert!(
            FormatCompatibility::SameMajorVersionMissingCapabilities
                .requires_negotiation()
        );

        assert!(
            FormatCompatibility::DifferentFormat
                .different_format()
        );

        assert!(
            FormatCompatibility::DifferentFormat
                .is_incompatible()
        );

        assert!(
            FormatCompatibility::IncompatibleVersion
                .incompatible_version()
        );

        assert!(
            FormatCompatibility::IncompatibleVersion
                .is_incompatible()
        );
    }

    #[test]
    fn capability_names_are_stable() {
        assert_eq!(
            FormatCapability::Import.as_str(),
            "import"
        );

        assert_eq!(
            FormatCapability::Export.as_str(),
            "export"
        );

        assert_eq!(
            FormatCapability::GateDefinitions.as_str(),
            "gate-definitions"
        );

        assert_eq!(
            FormatCapability::ClassicalControl.as_str(),
            "classical-control"
        );

        assert_eq!(
            FormatCapability::PhysicalQubits.as_str(),
            "physical-qubits"
        );
    }

    #[test]
    fn define_format_normalizes_identifier() {
        let format = define_format(
            "OPENQASM",
            FormatVersion::new(3, 1, 0),
            FormatCapabilities::new(),
        )
        .expect("valid");

        assert_eq!(format.id().as_str(), "openqasm");
    }

    #[test]
    fn empty_capability_set_is_valid() {
        let capabilities = FormatCapabilities::new();

        assert!(capabilities.is_empty());
        assert_eq!(capabilities.len(), 0);
    }

    #[test]
    fn capability_set_default_is_empty() {
        let capabilities = FormatCapabilities::default();

        assert!(capabilities.is_empty());
    }
}