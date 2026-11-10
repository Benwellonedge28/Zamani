
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Standard Library — Version Infrastructure
//!
//! Path: `src/stdlib/version.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides target-independent version representation, validation,
//! formatting and precedence comparison for Zamani standard-library
//! and integration contracts.
//!
//! This module implements the core representation of Semantic
//! Versioning 2.0.0.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - Semantic version representation.
//! - Strict semantic version parsing.
//! - Version formatting.
//! - Prerelease identifier validation.
//! - Build metadata validation.
//! - Semantic version precedence.
//! - Version parsing errors.
//!
//! This module DOES NOT own:
//!
//! - Zamani language version policy.
//! - Canonical grammar version policy.
//! - Compiler compatibility policy.
//! - IR schema compatibility policy.
//! - Standard-library API stability policy.
//! - Runtime compatibility decisions.
//! - Hardware or platform compatibility.
//! - Package dependency resolution.
//!
//! Those responsibilities belong to their respective authorities.
//!
//! ## VERSION IDENTITIES
//!
//! These identities must remain separate:
//!
//! 1. Cargo package version.
//! 2. Zamani language version.
//! 3. Grammar version.
//! 4. Compiler API version.
//! 5. Standard-library API version.
//! 6. Intermediate representation schema version.
//! 7. Serialization format version.
//!
//! Two components may happen to have the same numeric version.
//! That does not make their compatibility contracts interchangeable.
//!
//! ## SCALABILITY
//!
//! Numeric version components are represented as decimal strings.
//!
//! This avoids introducing an arbitrary u64/u128 maximum into version
//! representation.
//!
//! Actual memory and processing requirements remain subject to the
//! available resources and configured operational budgets.
//!
//! ## DEPENDENCIES
//!
//! This module intentionally depends only on Rust's standard library.
//!
//! It must remain independent of:
//!
//! - Frontend semantic analysis.
//! - AST implementations.
//! - Grammar parsers.
//! - Compiler internals.
//! - ZUIR.
//! - Quantum IR.
//! - Hardware providers.
//! - Runtime.
//! - External versioning crates.

// ============================================================================
// PACKAGE IDENTITY
// ============================================================================

/// Cargo package version supplied by Cargo at compilation time.
///
/// This identifies the Zamani Rust package.
///
/// It MUST NOT be interpreted as the Zamani language version,
/// grammar version, compiler API version, or standard-library
/// API compatibility version.
///
/// The value comes from Cargo.toml rather than being duplicated
/// as an independently maintained literal.
pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the Cargo package version.
///
/// This is a convenience alias for [`PACKAGE_VERSION`].
#[must_use]
pub const fn package_version() -> &'static str {
    PACKAGE_VERSION
}

// ============================================================================
// ERROR CLASSIFICATION
// ============================================================================

/// Classification of semantic-version parsing failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionErrorKind {
    /// The supplied version is empty.
    Empty,

    /// A required numeric component is missing.
    MissingComponent,

    /// A version contains an unexpected component.
    UnexpectedComponent,

    /// A numeric component contains a non-decimal character.
    InvalidNumericComponent,

    /// A numeric component contains an illegal leading zero.
    LeadingZero,

    /// A prerelease or build identifier is empty.
    EmptyIdentifier,

    /// An identifier contains a character not permitted by SemVer.
    InvalidIdentifier,

    /// A prerelease numeric identifier contains a leading zero.
    InvalidPrereleaseNumber,

    /// The version contains multiple build metadata separators.
    MultipleBuildSeparators,

    /// The version contains whitespace or other invalid syntax.
    InvalidSyntax,
}

impl std::fmt::Display for VersionErrorKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>)
        -> std::fmt::Result
    {
        let message = match self {
            Self::Empty => "version is empty",
            Self::MissingComponent => "required version component is missing",
            Self::UnexpectedComponent => "unexpected version component",
            Self::InvalidNumericComponent => {
                "numeric version component contains an invalid character"
            }
            Self::LeadingZero => {
                "numeric version component contains a leading zero"
            }
            Self::EmptyIdentifier => "version identifier is empty",
            Self::InvalidIdentifier => {
                "version identifier contains an invalid character"
            }
            Self::InvalidPrereleaseNumber => {
                "numeric prerelease identifier contains a leading zero"
            }
            Self::MultipleBuildSeparators => {
                "version contains multiple build metadata separators"
            }
            Self::InvalidSyntax => "version contains invalid syntax",
        };

        formatter.write_str(message)
    }
}

/// Structured semantic-version parsing error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionError {
    kind: VersionErrorKind,
    component: &'static str,
    position: usize,
}

impl VersionError {
    /// Creates a structured version error.
    fn new(
        kind: VersionErrorKind,
        component: &'static str,
        position: usize,
    ) -> Self {
        Self {
            kind,
            component,
            position,
        }
    }

    /// Returns the error classification.
    #[must_use]
    pub const fn kind(&self) -> VersionErrorKind {
        self.kind
    }

    /// Returns the component associated with the error.
    #[must_use]
    pub const fn component(&self) -> &'static str {
        self.component
    }

    /// Returns the byte position associated with the error.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }
}

impl std::fmt::Display for VersionError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            formatter,
            "Invalid semantic version: {} in {} at byte position {}",
            self.kind,
            self.component,
            self.position
        )
    }
}

impl std::error::Error for VersionError {}

// ============================================================================
// SEMANTIC VERSION
// ============================================================================

/// A validated Semantic Versioning 2.0.0 version.
///
/// Representation:
///
/// MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]
///
/// Numeric components are stored as decimal strings instead of fixed-width
/// integers, avoiding an unnecessary numeric ceiling.
///
/// Build metadata is retained for identity and formatting but does not
/// participate in semantic precedence.
///
/// Equality includes build metadata because the complete version identity
/// includes every supplied component.
///
/// Use [`Version::precedence_cmp`] when comparing semantic precedence.
///
/// # Examples
///
/// ```
/// use zamani_compiler::stdlib::version::Version;
///
/// let version = Version::parse("1.4.2-beta.1+build.7").unwrap();
///
/// assert_eq!(version.major(), "1");
/// assert_eq!(version.minor(), "4");
/// assert_eq!(version.patch(), "2");
/// assert_eq!(version.prerelease(), Some("beta.1"));
/// assert_eq!(version.build_metadata(), Some("build.7"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    major: String,
    minor: String,
    patch: String,
    prerelease: Option<String>,
    build_metadata: Option<String>,
}

impl Version {
    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    /// Parses a semantic version.
    ///
    /// Parsing follows the Semantic Versioning 2.0.0 syntax.
    ///
    /// The parser:
    ///
    /// - Requires exactly three numeric core components.
    /// - Rejects empty components.
    /// - Rejects leading zeros in numeric core components.
    /// - Validates prerelease identifiers.
    /// - Validates build metadata identifiers.
    /// - Rejects invalid characters.
    /// - Preserves the complete version identity.
    ///
    /// # Errors
    ///
    /// Returns [`VersionError`] when the input violates SemVer syntax.
    pub fn parse(input: &str) -> Result<Self, VersionError> {
        if input.is_empty() {
            return Err(VersionError::new(
                VersionErrorKind::Empty,
                "version",
                0,
            ));
        }

        if input.bytes().any(|byte| byte.is_ascii_whitespace()) {
            return Err(VersionError::new(
                VersionErrorKind::InvalidSyntax,
                "version",
                input
                    .bytes()
                    .position(|byte| byte.is_ascii_whitespace())
                    .unwrap_or(0),
            ));
        }

        // Separate build metadata.
        let (without_build, build_metadata) =
            match input.split_once('+') {
                Some((version, metadata)) => {
                    if metadata.contains('+') {
                        return Err(VersionError::new(
                            VersionErrorKind::MultipleBuildSeparators,
                            "build metadata",
                            input.find('+').unwrap_or(0) + 1
                                + metadata.find('+').unwrap_or(0),
                        ));
                    }

                    validate_identifiers(
                        metadata,
                        false,
                        "build metadata",
                        input.find('+').unwrap_or(0) + 1,
                    )?;

                    (version, Some(metadata.to_owned()))
                }
                None => (input, None),
            };

        // Separate prerelease identifiers.
        let (core, prerelease) =
            match without_build.split_once('-') {
                Some((version, prerelease)) => {
                    let position = without_build.find('-').unwrap_or(0) + 1;

                    validate_identifiers(
                        prerelease,
                        true,
                        "prerelease",
                        position,
                    )?;

                    (version, Some(prerelease.to_owned()))
                }
                None => (without_build, None),
            };

        // Validate the three numeric core components.
        let mut components = core.split('.');

        let major = components.next().ok_or_else(|| {
            VersionError::new(
                VersionErrorKind::MissingComponent,
                "major",
                0,
            )
        })?;

        let minor = components.next().ok_or_else(|| {
            VersionError::new(
                VersionErrorKind::MissingComponent,
                "minor",
                major.len() + 1,
            )
        })?;

        let patch = components.next().ok_or_else(|| {
            VersionError::new(
                VersionErrorKind::MissingComponent,
                "patch",
                major.len() + minor.len() + 2,
            )
        })?;

        if components.next().is_some() {
            return Err(VersionError::new(
                VersionErrorKind::UnexpectedComponent,
                "core version",
                0,
            ));
        }

        validate_numeric_component(major, "major", 0)?;

        validate_numeric_component(
            minor,
            "minor",
            major.len() + 1,
        )?;

        validate_numeric_component(
            patch,
            "patch",
            major.len() + minor.len() + 2,
        )?;

        Ok(Self {
            major: major.to_owned(),
            minor: minor.to_owned(),
            patch: patch.to_owned(),
            prerelease,
            build_metadata,
        })
    }

    // ------------------------------------------------------------------------
    // ACCESSORS
    // ------------------------------------------------------------------------

    /// Returns the major numeric component.
    #[must_use]
    pub fn major(&self) -> &str {
        &self.major
    }

    /// Returns the minor numeric component.
    #[must_use]
    pub fn minor(&self) -> &str {
        &self.minor
    }

    /// Returns the patch numeric component.
    #[must_use]
    pub fn patch(&self) -> &str {
        &self.patch
    }

    /// Returns the prerelease metadata, if present.
    #[must_use]
    pub fn prerelease(&self) -> Option<&str> {
        self.prerelease.as_deref()
    }

    /// Returns the build metadata, if present.
    #[must_use]
    pub fn build_metadata(&self) -> Option<&str> {
        self.build_metadata.as_deref()
    }

    /// Returns whether the version represents a prerelease.
    #[must_use]
    pub fn is_prerelease(&self) -> bool {
        self.prerelease.is_some()
    }

    /// Returns whether the version has build metadata.
    #[must_use]
    pub fn has_build_metadata(&self) -> bool {
        self.build_metadata.is_some()
    }

    /// Returns the core version without prerelease or build metadata.
    #[must_use]
    pub fn core_version(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    /// Returns whether two versions have identical major, minor and patch
    /// components.
    ///
    /// Prerelease and build metadata are deliberately ignored.
    #[must_use]
    pub fn has_same_core(&self, other: &Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    // ------------------------------------------------------------------------
    // PRECEDENCE
    // ------------------------------------------------------------------------

    /// Compares semantic version precedence.
    ///
    /// Comparison follows Semantic Versioning 2.0.0.
    ///
    /// Rules:
    ///
    /// 1. Compare major versions numerically.
    /// 2. Compare minor versions numerically.
    /// 3. Compare patch versions numerically.
    /// 4. A release outranks its prereleases.
    /// 5. Prerelease identifiers are compared individually.
    /// 6. Numeric prerelease identifiers have lower precedence than
    ///    non-numeric identifiers.
    /// 7. Build metadata is ignored.
    ///
    /// No fixed-width integer conversion is performed.
    #[must_use]
    pub fn precedence_cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        let major = compare_numeric(&self.major, &other.major);

        if major != Ordering::Equal {
            return major;
        }

        let minor = compare_numeric(&self.minor, &other.minor);

        if minor != Ordering::Equal {
            return minor;
        }

        let patch = compare_numeric(&self.patch, &other.patch);

        if patch != Ordering::Equal {
            return patch;
        }

        match (&self.prerelease, &other.prerelease) {
            (None, None) => Ordering::Equal,

            // A normal release has higher precedence.
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,

            (Some(left), Some(right)) => {
                compare_prerelease(left, right)
            }
        }
    }

    /// Returns whether this version has lower semantic precedence
    /// than another version.
    #[must_use]
    pub fn precedes(&self, other: &Self) -> bool {
        self.precedence_cmp(other) == std::cmp::Ordering::Less
    }

    /// Returns whether this version has higher semantic precedence
    /// than another version.
    #[must_use]
    pub fn succeeds(&self, other: &Self) -> bool {
        self.precedence_cmp(other) == std::cmp::Ordering::Greater
    }
}

// ============================================================================
// NUMERIC VALIDATION
// ============================================================================

/// Validates a numeric core component.
fn validate_numeric_component(
    value: &str,
    component: &'static str,
    position: usize,
) -> Result<(), VersionError> {
    if value.is_empty() {
        return Err(VersionError::new(
            VersionErrorKind::MissingComponent,
            component,
            position,
        ));
    }

    if !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(VersionError::new(
            VersionErrorKind::InvalidNumericComponent,
            component,
            position,
        ));
    }

    if value.len() > 1 && value.starts_with('0') {
        return Err(VersionError::new(
            VersionErrorKind::LeadingZero,
            component,
            position,
        ));
    }

    Ok(())
}

/// Validates dot-separated prerelease or build identifiers.
///
/// SemVer identifiers permit ASCII alphanumeric characters and hyphens.
///
/// Numeric prerelease identifiers cannot contain leading zeros.
///
/// Build metadata numeric identifiers may contain leading zeros.
fn validate_identifiers(
    value: &str,
    is_prerelease: bool,
    component: &'static str,
    base_position: usize,
) -> Result<(), VersionError> {
    if value.is_empty() {
        return Err(VersionError::new(
            VersionErrorKind::EmptyIdentifier,
            component,
            base_position,
        ));
    }

    let mut position = base_position;

    for identifier in value.split('.') {
        if identifier.is_empty() {
            return Err(VersionError::new(
                VersionErrorKind::EmptyIdentifier,
                component,
                position,
            ));
        }

        if !identifier.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || byte == b'-'
        }) {
            return Err(VersionError::new(
                VersionErrorKind::InvalidIdentifier,
                component,
                position,
            ));
        }

        if is_prerelease
            && identifier.len() > 1
            && identifier.starts_with('0')
            && identifier.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(VersionError::new(
                VersionErrorKind::InvalidPrereleaseNumber,
                component,
                position,
            ));
        }

        position += identifier.len() + 1;
    }

    Ok(())
}

// ============================================================================
// NUMERIC COMPARISON
// ============================================================================

/// Compares two validated decimal numeric strings.
///
/// Both values must contain only ASCII digits and have no illegal
/// leading zeros.
///
/// Comparing lengths first avoids integer overflow and fixed-width
/// numeric limitations.
fn compare_numeric(
    left: &str,
    right: &str,
) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    match left.len().cmp(&right.len()) {
        Ordering::Equal => left.as_bytes().cmp(right.as_bytes()),
        ordering => ordering,
    }
}

// ============================================================================
// PRERELEASE COMPARISON
// ============================================================================

/// Compares prerelease identifiers according to SemVer precedence.
fn compare_prerelease(
    left: &str,
    right: &str,
) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let mut left_parts = left.split('.');
    let mut right_parts = right.split('.');

    loop {
        match (left_parts.next(), right_parts.next()) {
            (None, None) => return Ordering::Equal,

            // A shorter equal prefix has lower precedence.
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,

            (Some(left), Some(right)) => {
                let left_numeric =
                    left.bytes().all(|byte| byte.is_ascii_digit());

                let right_numeric =
                    right.bytes().all(|byte| byte.is_ascii_digit());

                let ordering = match (left_numeric, right_numeric) {
                    (true, true) => compare_numeric(left, right),

                    // Numeric identifiers have lower precedence.
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,

                    // Nonnumeric identifiers use ASCII lexical ordering.
                    (false, false) => left.as_bytes().cmp(right.as_bytes()),
                };

                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
        }
    }
}

// ============================================================================
// FORMATTING
// ============================================================================

impl std::fmt::Display for Version {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )?;

        if let Some(prerelease) = &self.prerelease {
            write!(formatter, "-{prerelease}")?;
        }

        if let Some(metadata) = &self.build_metadata {
            write!(formatter, "+{metadata}")?;
        }

        Ok(())
    }
}

// ============================================================================
// STANDARD RUST CONVERSION
// ============================================================================

impl std::str::FromStr for Version {
    type Err = VersionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

impl TryFrom<&str> for Version {
    type Error = VersionError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;
    use std::str::FromStr;

    #[test]
    fn parses_basic_version() {
        let version = Version::parse("1.2.3").unwrap();

        assert_eq!(version.major(), "1");
        assert_eq!(version.minor(), "2");
        assert_eq!(version.patch(), "3");
        assert!(!version.is_prerelease());
        assert!(!version.has_build_metadata());
    }

    #[test]
    fn parses_prerelease_version() {
        let version = Version::parse("2.0.0-alpha.1").unwrap();

        assert_eq!(version.prerelease(), Some("alpha.1"));
        assert!(version.is_prerelease());
    }

    #[test]
    fn parses_build_metadata() {
        let version = Version::parse("2.0.0+build.123").unwrap();

        assert_eq!(
            version.build_metadata(),
            Some("build.123")
        );
    }

    #[test]
    fn parses_complete_version() {
        let version =
            Version::parse("1.4.2-beta.1+build.7").unwrap();

        assert_eq!(version.to_string(), "1.4.2-beta.1+build.7");
    }

    #[test]
    fn rejects_empty_version() {
        assert_eq!(
            Version::parse("").unwrap_err().kind(),
            VersionErrorKind::Empty
        );
    }

    #[test]
    fn rejects_missing_components() {
        assert!(Version::parse("1.2").is_err());
        assert!(Version::parse("1").is_err());
        assert!(Version::parse("1.2.").is_err());
    }

    #[test]
    fn rejects_extra_components() {
        assert!(Version::parse("1.2.3.4").is_err());
    }

    #[test]
    fn rejects_leading_zeroes_in_core() {
        assert!(Version::parse("01.2.3").is_err());
        assert!(Version::parse("1.02.3").is_err());
        assert!(Version::parse("1.2.03").is_err());
    }

    #[test]
    fn accepts_zero_components() {
        assert!(Version::parse("0.0.0").is_ok());
        assert!(Version::parse("0.1.0").is_ok());
    }

    #[test]
    fn rejects_invalid_numeric_components() {
        assert!(Version::parse("a.2.3").is_err());
        assert!(Version::parse("1.b.3").is_err());
        assert!(Version::parse("1.2.c").is_err());
    }

    #[test]
    fn rejects_invalid_prerelease_identifiers() {
        assert!(Version::parse("1.2.3-").is_err());
        assert!(Version::parse("1.2.3-alpha..1").is_err());
        assert!(Version::parse("1.2.3-01").is_err());
        assert!(Version::parse("1.2.3-alpha_1").is_err());
    }

    #[test]
    fn accepts_leading_zeroes_in_build_metadata() {
        assert!(Version::parse("1.2.3+001").is_ok());
    }

    #[test]
    fn rejects_invalid_build_metadata() {
        assert!(Version::parse("1.2.3+").is_err());
        assert!(Version::parse("1.2.3+build..7").is_err());
        assert!(Version::parse("1.2.3+build+extra").is_err());
    }

    #[test]
    fn compares_core_versions_numerically() {
        let left = Version::parse("1.9.0").unwrap();
        let right = Version::parse("1.10.0").unwrap();

        assert_eq!(
            left.precedence_cmp(&right),
            Ordering::Less
        );
    }

    #[test]
    fn compares_large_numeric_components_without_overflow() {
        let left = Version::parse(
            "999999999999999999999999999999999999999999.0.0"
        ).unwrap();

        let right = Version::parse(
            "1000000000000000000000000000000000000000000.0.0"
        ).unwrap();

        assert_eq!(
            left.precedence_cmp(&right),
            Ordering::Less
        );
    }

    #[test]
    fn compares_prerelease_against_release() {
        let prerelease = Version::parse("1.0.0-rc.1").unwrap();
        let release = Version::parse("1.0.0").unwrap();

        assert!(prerelease.precedes(&release));
    }

    #[test]
    fn compares_numeric_prerelease_identifiers() {
        let left = Version::parse("1.0.0-alpha.2").unwrap();
        let right = Version::parse("1.0.0-alpha.10").unwrap();

        assert_eq!(
            left.precedence_cmp(&right),
            Ordering::Less
        );
    }

    #[test]
    fn compares_numeric_and_nonnumeric_prerelease_identifiers() {
        let numeric = Version::parse("1.0.0-1").unwrap();
        let textual = Version::parse("1.0.0-alpha").unwrap();

        assert_eq!(
            numeric.precedence_cmp(&textual),
            Ordering::Less
        );
    }

    #[test]
    fn build_metadata_does_not_affect_precedence() {
        let left = Version::parse("1.0.0+build.1").unwrap();
        let right = Version::parse("1.0.0+build.2").unwrap();

        assert_eq!(
            left.precedence_cmp(&right),
            Ordering::Equal
        );

        // Full identity remains distinct.
        assert_ne!(left, right);
    }

    #[test]
    fn prerelease_ordering_follows_semver_examples() {
        let versions = [
            "1.0.0-alpha",
            "1.0.0-alpha.1",
            "1.0.0-alpha.beta",
            "1.0.0-beta",
            "1.0.0-beta.2",
            "1.0.0-beta.11",
            "1.0.0-rc.1",
            "1.0.0",
        ];

        for pair in versions.windows(2) {
            let left = Version::parse(pair[0]).unwrap();
            let right = Version::parse(pair[1]).unwrap();

            assert!(left.precedes(&right));
        }
    }

    #[test]
    fn identifies_matching_core_versions() {
        let left = Version::parse("1.2.3-alpha").unwrap();
        let right = Version::parse("1.2.3+build").unwrap();

        assert!(left.has_same_core(&right));
    }

    #[test]
    fn formats_versions_without_metadata() {
        let version = Version::parse("1.2.3").unwrap();

        assert_eq!(version.to_string(), "1.2.3");
        assert_eq!(version.core_version(), "1.2.3");
    }

    #[test]
    fn supports_from_str() {
        let version = Version::from_str("3.4.5").unwrap();

        assert_eq!(version.to_string(), "3.4.5");
    }

    #[test]
    fn supports_try_from() {
        let version = Version::try_from("4.5.6").unwrap();

        assert_eq!(version.to_string(), "4.5.6");
    }

    #[test]
    fn package_version_matches_cargo_manifest() {
        assert_eq!(
            package_version(),
            env!("CARGO_PKG_VERSION")
        );

        assert!(Version::parse(PACKAGE_VERSION).is_ok());
    }

    #[test]
    fn rejects_whitespace() {
        assert!(Version::parse(" 1.2.3").is_err());
        assert!(Version::parse("1.2.3 ").is_err());
        assert!(Version::parse("1.2 .3").is_err());
    }
}
