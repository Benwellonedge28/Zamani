
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR format versioning
//!
//! Path: `src/hdl/hdlir/version.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the version of the serialized HDL intermediate-representation
//! format and the rules for determining whether a reader can read an artifact.
//!
//! ## Authority
//!
//! This module must conform to:
//! - `grammar/spec/hdl.md`
//! - `src/zuir/ZUIR-SPEC.md`
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, once established
//! - `src/hdl/hdlir/compatibility.md`, once established
//!
//! The HDLIR format version is independent of the Zamani language version,
//! crate version, HDL syntax version, ZUIR version, and dialect versions.
//!
//! ## Ownership
//!
//! This module owns the representation and comparison of HDLIR format
//! versions. It does not own artifact encoding, schema registration,
//! migrations, or dialect-feature discovery.
//!
//! ## Compatibility policy
//!
//! A format version is written as `major.minor.patch`.
//!
//! - A major-version change may be incompatible.
//! - A minor-version change within one major version must be backward
//!   readable according to the HDLIR compatibility specification.
//! - A patch-version change must not change the format's interpretation.
//!
//! An artifact can be read by a reader when both versions have the same
//! major version and the artifact's minor version is not newer than the
//! reader's minor version. Patch versions do not affect this check.
//!
//! This version comparison is necessary but not sufficient for reading an
//! artifact. The decoder and schema registry must also validate required
//! features, operation definitions, dialect versions, and structural rules.
//!
//! ## Scalability
//!
//! Version numbers use `u32` components. This is a version-encoding choice,
//! not a hardware-capacity limit. Parsing rejects overflow instead of
//! wrapping. No hardware counts, widths, or resource quantities are stored
//! in this module.
//!
//! ## Freeze criteria
//!
//! Freeze this API only after the compatibility specification ratifies the
//! initial format version and the artifact reader's compatibility policy.

use core::fmt;
use core::str::FromStr;

/// The initial HDLIR artifact-format version.
///
/// This value must be ratified in `HDLIR-SPEC.md` and `compatibility.md`
/// before a released artifact format relies on it.
pub const HDLIR_FORMAT_VERSION: HdlirVersion = HdlirVersion::new(1, 0, 0);

/// A validated HDLIR artifact-format version.
///
/// The components are numeric and cannot contain invalid states.
/// This type is deliberately independent of the crate's package version.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HdlirVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl HdlirVersion {
    /// Construct a version from its three numeric components.
    ///
    /// All `u32` component values are valid. This constructor does not
    /// imply that the resulting version is supported by a particular reader.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Return the major version component.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Return the minor version component.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Return the patch version component.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }

    /// Check whether an artifact using `self` can be read by `reader`.
    ///
    /// This checks only the version-level compatibility policy. A successful
    /// result does not establish that the artifact's operations, types,
    /// dialects, required features, or contents are supported or valid.
    ///
    /// # Errors
    ///
    /// Returns [`VersionCompatibilityError::MajorVersionMismatch`] if the
    /// major versions differ.
    ///
    /// Returns [`VersionCompatibilityError::ArtifactMinorTooNew`] if the
    /// artifact's minor version exceeds the reader's minor version.
    pub fn ensure_readable_by(
        self,
        reader: HdlirVersion,
    ) -> Result<(), VersionCompatibilityError> {
        if self.major != reader.major {
            return Err(VersionCompatibilityError::MajorVersionMismatch {
                artifact_major: self.major,
                reader_major: reader.major,
            });
        }

        if self.minor > reader.minor {
            return Err(VersionCompatibilityError::ArtifactMinorTooNew {
                artifact_minor: self.minor,
                reader_minor: reader.minor,
            });
        }

        Ok(())
    }

    /// Return whether this artifact version is readable by `reader`
    /// under the version-level compatibility policy.
    ///
    /// This is a convenience check. Use [`Self::ensure_readable_by`] when
    /// the caller needs a reason for incompatibility.
    #[must_use]
    pub fn is_readable_by(self, reader: HdlirVersion) -> bool {
        self.ensure_readable_by(reader).is_ok()
    }
}

impl Default for HdlirVersion {
    /// Return the current HDLIR artifact-format version.
    fn default() -> Self {
        HDLIR_FORMAT_VERSION
    }
}

impl fmt::Display for HdlirVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

/// Errors encountered while parsing a version string.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VersionParseError {
    /// The input contains no characters.
    Empty,

    /// The input does not contain exactly three dot-separated components.
    WrongComponentCount,

    /// A component is empty, for example in `1..0`.
    EmptyComponent,

    /// A component contains a character other than an ASCII decimal digit.
    InvalidDigit,

    /// A numeric component has a leading zero, such as `01.0.0`.
    LeadingZero,

    /// A numeric component exceeds the representable `u32` range.
    ComponentOverflow,
}

impl fmt::Display for VersionParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Empty => "version string is empty",
            Self::WrongComponentCount => {
                "version must contain exactly three components"
            }
            Self::EmptyComponent => "version contains an empty component",
            Self::InvalidDigit => {
                "version components must contain ASCII decimal digits only"
            }
            Self::LeadingZero => {
                "version components must not contain leading zeros"
            }
            Self::ComponentOverflow => {
                "version component exceeds the u32 range"
            }
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for VersionParseError {}

/// Errors indicating that an artifact version cannot be read by a reader.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VersionCompatibilityError {
    /// The artifact and reader use different major versions.
    MajorVersionMismatch {
        /// Major version declared by the artifact.
        artifact_major: u32,
        /// Major version supported by the reader.
        reader_major: u32,
    },

    /// The artifact's minor version is newer than the reader's minor version.
    ArtifactMinorTooNew {
        /// Minor version declared by the artifact.
        artifact_minor: u32,
        /// Minor version supported by the reader.
        reader_minor: u32,
    },
}

impl fmt::Display for VersionCompatibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MajorVersionMismatch {
                artifact_major,
                reader_major,
            } => write!(
                formatter,
                "incompatible HDLIR major version: artifact uses {}, reader supports {}",
                artifact_major, reader_major
            ),
            Self::ArtifactMinorTooNew {
                artifact_minor,
                reader_minor,
            } => write!(
                formatter,
                "unsupported HDLIR minor version: artifact uses {}, reader supports at most {}",
                artifact_minor, reader_minor
            ),
        }
    }
}

impl std::error::Error for VersionCompatibilityError {}

impl FromStr for HdlirVersion {
    type Err = VersionParseError;

    /// Parse a strict `major.minor.patch` version string.
    ///
    /// Leading/trailing whitespace, signs, leading zeros, non-ASCII digits,
    /// missing components, extra components, and overflowing values are
    /// rejected. Callers handling serialized artifacts should apply their
    /// configured input-size budget before parsing untrusted data.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(VersionParseError::Empty);
        }

        let mut components = input.split('.');

        let major = parse_component(
            components.next().ok_or(VersionParseError::WrongComponentCount)?,
        )?;
        let minor = parse_component(
            components.next().ok_or(VersionParseError::WrongComponentCount)?,
        )?;
        let patch = parse_component(
            components.next().ok_or(VersionParseError::WrongComponentCount)?,
        )?;

        if components.next().is_some() {
            return Err(VersionParseError::WrongComponentCount);
        }

        Ok(Self::new(major, minor, patch))
    }
}

/// Parse one canonical unsigned decimal component without wrapping.
fn parse_component(component: &str) -> Result<u32, VersionParseError> {
    if component.is_empty() {
        return Err(VersionParseError::EmptyComponent);
    }

    if component.len() > 1 && component.starts_with('0') {
        return Err(VersionParseError::LeadingZero);
    }

    let mut value = 0_u32;

    for byte in component.bytes() {
        if !byte.is_ascii_digit() {
            return Err(VersionParseError::InvalidDigit);
        }

        let digit = u32::from(byte - b'0');

        value = value
            .checked_mul(10)
            .and_then(|current| current.checked_add(digit))
            .ok_or(VersionParseError::ComponentOverflow)?;
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{
        HdlirVersion, VersionCompatibilityError, VersionParseError,
        HDLIR_FORMAT_VERSION,
    };
    use core::str::FromStr;

    #[test]
    fn current_version_has_expected_components() {
        assert_eq!(HDLIR_FORMAT_VERSION.major(), 1);
        assert_eq!(HDLIR_FORMAT_VERSION.minor(), 0);
        assert_eq!(HDLIR_FORMAT_VERSION.patch(), 0);
    }

    #[test]
    fn parses_and_formats_canonical_versions() {
        let version = HdlirVersion::from_str("12.34.56").unwrap();

        assert_eq!(version, HdlirVersion::new(12, 34, 56));
        assert_eq!(version.to_string(), "12.34.56");
    }

    #[test]
    fn accepts_zero_components() {
        assert_eq!(
            HdlirVersion::from_str("0.0.0").unwrap(),
            HdlirVersion::new(0, 0, 0)
        );
    }

    #[test]
    fn accepts_maximum_component_values() {
        let version = HdlirVersion::from_str(
            "4294967295.4294967295.4294967295",
        )
        .unwrap();

        assert_eq!(version.major(), u32::MAX);
        assert_eq!(version.minor(), u32::MAX);
        assert_eq!(version.patch(), u32::MAX);
    }

    #[test]
    fn rejects_empty_or_malformed_versions() {
        assert_eq!(
            HdlirVersion::from_str(""),
            Err(VersionParseError::Empty)
        );
        assert_eq!(
            HdlirVersion::from_str("1.0"),
            Err(VersionParseError::WrongComponentCount)
        );
        assert_eq!(
            HdlirVersion::from_str("1.0.0.0"),
            Err(VersionParseError::WrongComponentCount)
        );
        assert_eq!(
            HdlirVersion::from_str("1..0"),
            Err(VersionParseError::EmptyComponent)
        );
    }

    #[test]
    fn rejects_noncanonical_numeric_components() {
        assert_eq!(
            HdlirVersion::from_str("01.0.0"),
            Err(VersionParseError::LeadingZero)
        );
        assert_eq!(
            HdlirVersion::from_str("+1.0.0"),
            Err(VersionParseError::InvalidDigit)
        );
        assert_eq!(
            HdlirVersion::from_str("1.0.0 "),
            Err(VersionParseError::InvalidDigit)
        );
        assert_eq!(
            HdlirVersion::from_str("１.0.0"),
            Err(VersionParseError::InvalidDigit)
        );
    }

    #[test]
    fn rejects_component_overflow_without_wrapping() {
        assert_eq!(
            HdlirVersion::from_str("4294967296.0.0"),
            Err(VersionParseError::ComponentOverflow)
        );
    }

    #[test]
    fn same_major_and_supported_minor_are_compatible() {
        let artifact = HdlirVersion::new(1, 2, 0);
        let reader = HdlirVersion::new(1, 3, 7);

        assert!(artifact.is_readable_by(reader));
        assert_eq!(artifact.ensure_readable_by(reader), Ok(()));
    }

    #[test]
    fn patch_difference_does_not_break_compatibility() {
        let artifact = HdlirVersion::new(1, 2, 99);
        let reader = HdlirVersion::new(1, 2, 0);

        assert!(artifact.is_readable_by(reader));
    }

    #[test]
    fn rejects_different_major_versions() {
        let artifact = HdlirVersion::new(2, 0, 0);
        let reader = HdlirVersion::new(1, 99, 99);

        assert_eq!(
            artifact.ensure_readable_by(reader),
            Err(VersionCompatibilityError::MajorVersionMismatch {
                artifact_major: 2,
                reader_major: 1,
            })
        );
    }

    #[test]
    fn rejects_artifact_with_newer_minor_version() {
        let artifact = HdlirVersion::new(1, 4, 0);
        let reader = HdlirVersion::new(1, 3, 99);

        assert_eq!(
            artifact.ensure_readable_by(reader),
            Err(VersionCompatibilityError::ArtifactMinorTooNew {
                artifact_minor: 4,
                reader_minor: 3,
            })
        );
    }

    #[test]
    fn version_ordering_is_numeric_not_lexicographic() {
        assert!(HdlirVersion::new(1, 10, 0) > HdlirVersion::new(1, 9, 99));
    }
}
