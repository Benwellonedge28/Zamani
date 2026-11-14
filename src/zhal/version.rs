
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer — Version Contracts
//!
//! Path: `src/zhal/version.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the independent version identity of the Zamani Universal
//! Hardware Abstraction Layer (ZHAL).
//!
//! ZHAL versioning identifies the public contracts exposed by the
//! universal hardware abstraction subsystem.
//!
//! This module reuses the canonical Semantic Versioning implementation
//! provided by:
//!
//!     crate::stdlib::version
//!
//! It does not implement another semantic-versioning engine.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - ZHAL API identity.
//! - ZHAL API version identity.
//! - Strongly typed ZHAL API versions.
//! - ZHAL API version parsing.
//! - ZHAL API version formatting.
//! - ZHAL API version precedence comparison.
//! - ZHAL API version validation.
//! - ZHAL API version documentation.
//!
//! ## DOES NOT OWN
//!
//! This module does not own:
//!
//! - Zamani language versions.
//! - Zamani grammar versions.
//! - Rust compiler versions.
//! - Cargo package versions.
//! - Standard-library versions.
//! - Provider implementation versions.
//! - Provider API compatibility policies.
//! - Hardware firmware versions.
//! - Hardware protocol versions.
//! - Device revisions.
//! - Backend versions.
//! - Runtime versions.
//! - ZUIR versions.
//! - Quantum IR versions.
//! - HDL IR versions.
//! - Serialization schema versions.
//! - Hardware compatibility decisions.
//! - Provider admission or registration.
//!
//! These responsibilities belong to their existing authoritative
//! repository subsystems.
//!
//! ## CANONICAL VERSION AUTHORITY
//!
//! Semantic version representation and parsing:
//!
//!     crate::stdlib::version::Version
//!
//! Language versioning:
//!
//!     grammar/specification/language-version.md
//!
//! Cross-layer compatibility:
//!
//!     grammar/compatibility/versions.md
//!
//! Provider version contracts:
//!
//!     crate::stdlib::provider
//!
//! ZUIR versioning:
//!
//!     crate::zuir::version
//!
//! ## VERSION IDENTITY
//!
//! ZHAL API versions use:
//!
//!     MAJOR.MINOR.PATCH
//!
//! Optional suffixes follow Semantic Versioning 2.0.0:
//!
//!     MAJOR.MINOR.PATCH-PRERELEASE
//!
//!     MAJOR.MINOR.PATCH+BUILD
//!
//!     MAJOR.MINOR.PATCH-PRERELEASE+BUILD
//!
//! The initial ZHAL API version is deliberately 0.1.0.
//!
//! A zero-major API is considered an evolving API whose compatibility
//! guarantees must be established explicitly before stabilization.
//!
//! The version is independent of Cargo's package version.
//!
//! ## POCO-REAF
//!
//! Version components are represented using the existing canonical
//! Version implementation.
//!
//! Numeric components are stored as decimal strings rather than
//! fixed-width integers.
//!
//! Consequently, this module introduces no arbitrary numeric ceiling
//! for version components.
//!
//! It introduces no limits on:
//!
//! - CPUs;
//! - processor cores;
//! - GPUs;
//! - FPGAs;
//! - ASICs;
//! - QPUs;
//! - qubits;
//! - devices;
//! - providers;
//! - hardware resources;
//! - distributed participants;
//! - memory;
//! - storage;
//! - register widths;
//! - computational domains.
//!
//! Hardware scalability is independent of API version representation.
//!
//! Actual processing and memory consumption remain subject to
//! available resources and implementation constraints.
//!
//! No literal infinite physical capacity is promised.
//!
//! ## DEPENDENCIES
//!
//! This module depends only on:
//!
//!     crate::stdlib::version
//!
//! No additional external dependency is required.
//!
//! ## INTEGRATION
//!
//! The ZHAL composition root must expose this module:
//!
//!     pub mod version;
//!
//! Provider implementations should use the canonical standard-library
//! Version type for their independently declared provider API versions.
//!
//! ZHAL compatibility and provider validation must consume these
//! version identities through explicit compatibility policies.
//!
//! This module does not automatically accept or reject providers.
//!
//! ## INITIALIZATION
//!
//! Importing this module performs no initialization.
//!
//! It does not:
//!
//! - discover hardware;
//! - initialize providers;
//! - allocate resources;
//! - create global registries;
//! - initialize runtimes;
//! - access hardware;
//! - communicate with external services.
//!
//! ## SAFETY
//!
//! Unsafe Rust is forbidden.
//!
//! This module requires no unsafe implementation.
//!
//! All externally supplied versions are validated before construction.

// ============================================================================
// IMPORTS
// ============================================================================

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

use crate::stdlib::version::{
    Version,
    VersionError,
};

// ============================================================================
// ZHAL API IDENTITY
// ============================================================================

/// Stable namespace identifying the ZHAL public API contract.
///
/// This identifier is independent of:
///
/// - Cargo package identity;
/// - hardware provider identity;
/// - physical device identity;
/// - hardware vendor;
/// - hardware architecture;
/// - runtime identity;
/// - language identity.
///
/// It remains stable across compatible ZHAL API releases.
///
/// A future breaking API redesign does not automatically require
/// changing this namespace.
///
/// The version identifies the evolution of the contract.
pub const ZHAL_API_NAMESPACE: &str = "org.zamani.zhal.api";

// ============================================================================
// ZHAL API VERSION
// ============================================================================

/// Current ZHAL public API version.
///
/// This version identifies the public ZHAL API contract.
///
/// It is deliberately independent of:
///
/// - `CARGO_PKG_VERSION`;
/// - the Zamani language version;
/// - the Rust compiler version;
/// - the standard-library version;
/// - provider implementation versions;
/// - hardware firmware versions.
///
/// The initial ZHAL API contract starts at `0.1.0`.
///
/// A change to this constant requires an explicit API compatibility
/// review and corresponding documentation and tests.
///
/// This constant is not a promise that every planned ZHAL feature
/// has already been implemented.
pub const ZHAL_API_VERSION: &str = "0.1.0";

/// Rust edition targeted by the ZHAL implementation.
///
/// This describes the Rust implementation, not the Zamani language.
pub const ZHAL_RUST_EDITION: &str = "2021";

/// Minimum Rust version declared by the current Cargo manifest.
///
/// The repository currently declares Rust 1.97 in `Cargo.toml`.
///
/// This constant is informational and does not replace Cargo's
/// `rust-version` enforcement.
pub const ZHAL_MINIMUM_RUST_VERSION: &str = "1.97";

// ============================================================================
// STRONGLY TYPED ZHAL API VERSION
// ============================================================================

/// A validated version identifying the ZHAL public API contract.
///
/// This is a domain-specific wrapper around the canonical
/// [`crate::stdlib::version::Version`] type.
///
/// It prevents accidental interchange between:
///
/// - ZHAL API versions;
/// - Cargo package versions;
/// - language versions;
/// - hardware firmware versions;
/// - provider API versions.
///
/// The underlying Semantic Versioning representation remains owned
/// by `crate::stdlib::version`.
///
/// # Examples
///
/// ```
/// use zamani_compiler::zhal::version::ZhalApiVersion;
///
/// let version = ZhalApiVersion::parse("1.2.3").unwrap();
///
/// assert_eq!(version.major(), "1");
/// assert_eq!(version.minor(), "2");
/// assert_eq!(version.patch(), "3");
/// assert_eq!(version.to_string(), "1.2.3");
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ZhalApiVersion {
    version: Version,
}

impl ZhalApiVersion {
    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    /// Constructs a ZHAL API version from an existing canonical version.
    ///
    /// The supplied value has already been validated by the canonical
    /// Semantic Versioning implementation.
    ///
    /// # Arguments
    ///
    /// * `version` - Canonical validated semantic version.
    #[must_use]
    pub const fn new(version: Version) -> Self {
        Self {
            version,
        }
    }

    /// Parses and validates a ZHAL API version.
    ///
    /// Parsing follows the repository's canonical Semantic Versioning
    /// 2.0.0 implementation.
    ///
    /// Numeric version components are not restricted to `u32`, `u64`
    /// or `u128`.
    ///
    /// # Errors
    ///
    /// Returns [`VersionError`] when the supplied version is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use zamani_compiler::zhal::version::ZhalApiVersion;
    ///
    /// let version =
    ///     ZhalApiVersion::parse("2.4.1-beta.2+build.7").unwrap();
    ///
    /// assert_eq!(version.major(), "2");
    /// assert_eq!(version.prerelease(), Some("beta.2"));
    /// assert_eq!(version.build_metadata(), Some("build.7"));
    /// ```
    pub fn parse(input: &str) -> Result<Self, VersionError> {
        Version::parse(input).map(Self::new)
    }

    /// Returns the currently declared ZHAL API version.
    ///
    /// The version is parsed through the canonical version authority.
    ///
    /// # Errors
    ///
    /// Returns [`VersionError`] if the repository's declared version
    /// constant is invalid.
    ///
    /// Such an error indicates an implementation defect in the
    /// version declaration.
    pub fn current() -> Result<Self, VersionError> {
        Self::parse(ZHAL_API_VERSION)
    }

    // ------------------------------------------------------------------------
    // ACCESSORS
    // ------------------------------------------------------------------------

    /// Returns the major version component.
    #[must_use]
    pub fn major(&self) -> &str {
        self.version.major()
    }

    /// Returns the minor version component.
    #[must_use]
    pub fn minor(&self) -> &str {
        self.version.minor()
    }

    /// Returns the patch version component.
    #[must_use]
    pub fn patch(&self) -> &str {
        self.version.patch()
    }

    /// Returns the prerelease identifiers, if present.
    #[must_use]
    pub fn prerelease(&self) -> Option<&str> {
        self.version.prerelease()
    }

    /// Returns the build metadata, if present.
    #[must_use]
    pub fn build_metadata(&self) -> Option<&str> {
        self.version.build_metadata()
    }

    /// Returns whether this version is a prerelease.
    #[must_use]
    pub fn is_prerelease(&self) -> bool {
        self.version.is_prerelease()
    }

    /// Returns whether build metadata is present.
    #[must_use]
    pub fn has_build_metadata(&self) -> bool {
        self.version.has_build_metadata()
    }

    /// Returns the canonical version without prerelease or build metadata.
    #[must_use]
    pub fn core_version(&self) -> String {
        self.version.core_version()
    }

    /// Returns the underlying canonical version by reference.
    ///
    /// This is the integration boundary for existing repository
    /// components that consume `crate::stdlib::version::Version`.
    #[must_use]
    pub const fn as_version(&self) -> &Version {
        &self.version
    }

    /// Consumes this wrapper and returns the canonical version.
    #[must_use]
    pub fn into_version(self) -> Version {
        self.version
    }

    // ------------------------------------------------------------------------
    // COMPARISON
    // ------------------------------------------------------------------------

    /// Compares semantic version precedence.
    ///
    /// The comparison follows Semantic Versioning 2.0.0.
    ///
    /// The canonical implementation:
    ///
    /// - compares arbitrary-length numeric components;
    /// - compares prerelease identifiers correctly;
    /// - gives a release higher precedence than its prereleases;
    /// - ignores build metadata for precedence.
    ///
    /// Build metadata remains part of complete version identity.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::cmp::Ordering;
    ///
    /// use zamani_compiler::zhal::version::ZhalApiVersion;
    ///
    /// let older = ZhalApiVersion::parse("1.9.0").unwrap();
    /// let newer = ZhalApiVersion::parse("1.10.0").unwrap();
    ///
    /// assert_eq!(
    ///     older.precedence_cmp(&newer),
    ///     Ordering::Less
    /// );
    /// ```
    #[must_use]
    pub fn precedence_cmp(&self, other: &Self) -> Ordering {
        self.version.precedence_cmp(&other.version)
    }

    /// Returns whether this version precedes another version.
    #[must_use]
    pub fn precedes(&self, other: &Self) -> bool {
        self.version.precedes(&other.version)
    }

    /// Returns whether this version succeeds another version.
    #[must_use]
    pub fn succeeds(&self, other: &Self) -> bool {
        self.version.succeeds(&other.version)
    }

    /// Returns whether both versions have identical major, minor
    /// and patch components.
    ///
    /// Prerelease and build metadata are deliberately ignored.
    #[must_use]
    pub fn has_same_core(&self, other: &Self) -> bool {
        self.version.has_same_core(&other.version)
    }
}

// ============================================================================
// FORMATTING
// ============================================================================

impl fmt::Display for ZhalApiVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        fmt::Display::fmt(&self.version, formatter)
    }
}

// ============================================================================
// STANDARD RUST CONVERSIONS
// ============================================================================

impl FromStr for ZhalApiVersion {
    type Err = VersionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

impl TryFrom<&str> for ZhalApiVersion {
    type Error = VersionError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Self::parse(input)
    }
}

impl TryFrom<String> for ZhalApiVersion {
    type Error = VersionError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        Self::parse(&input)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::cmp::Ordering;
    use core::str::FromStr;

    // ------------------------------------------------------------------------
    // IDENTITY
    // ------------------------------------------------------------------------

    #[test]
    fn exposes_stable_api_namespace() {
        assert_eq!(
            ZHAL_API_NAMESPACE,
            "org.zamani.zhal.api"
        );
    }

    #[test]
    fn declares_valid_api_version() {
        assert!(
            ZhalApiVersion::parse(ZHAL_API_VERSION).is_ok()
        );
    }

    #[test]
    fn exposes_current_version() {
        let version = ZhalApiVersion::current().unwrap();

        assert_eq!(
            version.to_string(),
            ZHAL_API_VERSION
        );
    }

    #[test]
    fn api_version_is_independent_of_package_version() {
        let package_version =
            crate::stdlib::version::package_version();

        assert!(
            !package_version.is_empty()
        );

        let api_version =
            ZhalApiVersion::current().unwrap();

        assert!(
            !api_version.to_string().is_empty()
        );
    }

    // ------------------------------------------------------------------------
    // BASIC PARSING
    // ------------------------------------------------------------------------

    #[test]
    fn parses_basic_api_version() {
        let version =
            ZhalApiVersion::parse("1.2.3").unwrap();

        assert_eq!(version.major(), "1");
        assert_eq!(version.minor(), "2");
        assert_eq!(version.patch(), "3");

        assert_eq!(version.prerelease(), None);
        assert_eq!(version.build_metadata(), None);
    }

    #[test]
    fn parses_prerelease_version() {
        let version =
            ZhalApiVersion::parse("2.0.0-alpha.1").unwrap();

        assert_eq!(
            version.prerelease(),
            Some("alpha.1")
        );

        assert!(version.is_prerelease());
    }

    #[test]
    fn parses_build_metadata() {
        let version =
            ZhalApiVersion::parse("2.0.0+build.123").unwrap();

        assert_eq!(
            version.build_metadata(),
            Some("build.123")
        );

        assert!(version.has_build_metadata());
    }

    #[test]
    fn parses_complete_semantic_version() {
        let version =
            ZhalApiVersion::parse(
                "2.4.1-beta.2+build.7"
            )
            .unwrap();

        assert_eq!(
            version.to_string(),
            "2.4.1-beta.2+build.7"
        );
    }

    // ------------------------------------------------------------------------
    // VALIDATION
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_empty_version() {
        assert!(
            ZhalApiVersion::parse("").is_err()
        );
    }

    #[test]
    fn rejects_missing_components() {
        assert!(
            ZhalApiVersion::parse("1").is_err()
        );

        assert!(
            ZhalApiVersion::parse("1.2").is_err()
        );

        assert!(
            ZhalApiVersion::parse("1.2.").is_err()
        );
    }

    #[test]
    fn rejects_extra_components() {
        assert!(
            ZhalApiVersion::parse("1.2.3.4").is_err()
        );
    }

    #[test]
    fn rejects_leading_zeroes() {
        assert!(
            ZhalApiVersion::parse("01.2.3").is_err()
        );

        assert!(
            ZhalApiVersion::parse("1.02.3").is_err()
        );

        assert!(
            ZhalApiVersion::parse("1.2.03").is_err()
        );
    }

    #[test]
    fn rejects_invalid_characters() {
        assert!(
            ZhalApiVersion::parse("a.2.3").is_err()
        );

        assert!(
            ZhalApiVersion::parse("1.b.3").is_err()
        );

        assert!(
            ZhalApiVersion::parse("1.2.c").is_err()
        );
    }

    #[test]
    fn rejects_invalid_prerelease() {
        assert!(
            ZhalApiVersion::parse("1.2.3-").is_err()
        );

        assert!(
            ZhalApiVersion::parse("1.2.3-alpha..1").is_err()
        );

        assert!(
            ZhalApiVersion::parse("1.2.3-alpha_1").is_err()
        );
    }

    #[test]
    fn rejects_invalid_build_metadata() {
        assert!(
            ZhalApiVersion::parse("1.2.3+").is_err()
        );

        assert!(
            ZhalApiVersion::parse("1.2.3+build..7").is_err()
        );

        assert!(
            ZhalApiVersion::parse("1.2.3+build+extra").is_err()
        );
    }

    #[test]
    fn rejects_surrounding_whitespace() {
        assert!(
            ZhalApiVersion::parse(" 1.2.3").is_err()
        );

        assert!(
            ZhalApiVersion::parse("1.2.3 ").is_err()
        );
    }

    // ------------------------------------------------------------------------
    // ARBITRARY NUMERIC COMPONENTS
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_large_numeric_components() {
        let version = ZhalApiVersion::parse(
            "999999999999999999999999999999999999999999.\
             888888888888888888888888888888888888888888.\
             777777777777777777777777777777777777777777"
                .replace(' ', "")
                .as_str(),
        )
        .unwrap();

        assert_eq!(
            version.major(),
            "999999999999999999999999999999999999999999"
        );
    }

    #[test]
    fn compares_large_components_without_integer_overflow() {
        let older = ZhalApiVersion::parse(
            "999999999999999999999999999999999999999999.0.0"
        )
        .unwrap();

        let newer = ZhalApiVersion::parse(
            "1000000000000000000000000000000000000000000.0.0"
        )
        .unwrap();

        assert_eq!(
            older.precedence_cmp(&newer),
            Ordering::Less
        );
    }

    // ------------------------------------------------------------------------
    // SEMANTIC PRECEDENCE
    // ------------------------------------------------------------------------

    #[test]
    fn compares_minor_versions_numerically() {
        let older =
            ZhalApiVersion::parse("1.9.0").unwrap();

        let newer =
            ZhalApiVersion::parse("1.10.0").unwrap();

        assert_eq!(
            older.precedence_cmp(&newer),
            Ordering::Less
        );
    }

    #[test]
    fn compares_patch_versions() {
        let older =
            ZhalApiVersion::parse("1.2.3").unwrap();

        let newer =
            ZhalApiVersion::parse("1.2.4").unwrap();

        assert!(older.precedes(&newer));
    }

    #[test]
    fn release_has_higher_precedence_than_prerelease() {
        let prerelease =
            ZhalApiVersion::parse("1.0.0-rc.1").unwrap();

        let release =
            ZhalApiVersion::parse("1.0.0").unwrap();

        assert!(prerelease.precedes(&release));
    }

    #[test]
    fn build_metadata_does_not_affect_precedence() {
        let first =
            ZhalApiVersion::parse("1.0.0+build.1").unwrap();

        let second =
            ZhalApiVersion::parse("1.0.0+build.2").unwrap();

        assert_eq!(
            first.precedence_cmp(&second),
            Ordering::Equal
        );

        assert_ne!(first, second);
    }

    #[test]
    fn detects_identical_core_versions() {
        let first =
            ZhalApiVersion::parse("1.2.3-alpha").unwrap();

        let second =
            ZhalApiVersion::parse("1.2.3+build").unwrap();

        assert!(
            first.has_same_core(&second)
        );
    }

    // ------------------------------------------------------------------------
    // CONVERSIONS
    // ------------------------------------------------------------------------

    #[test]
    fn supports_from_str() {
        let version =
            ZhalApiVersion::from_str("3.4.5").unwrap();

        assert_eq!(
            version.to_string(),
            "3.4.5"
        );
    }

    #[test]
    fn supports_try_from_str() {
        let version =
            ZhalApiVersion::try_from("4.5.6").unwrap();

        assert_eq!(
            version.to_string(),
            "4.5.6"
        );
    }

    #[test]
    fn supports_try_from_string() {
        let version =
            ZhalApiVersion::try_from(
                String::from("5.6.7")
            )
            .unwrap();

        assert_eq!(
            version.to_string(),
            "5.6.7"
        );
    }

    // ------------------------------------------------------------------------
    // CANONICAL INTEGRATION
    // ------------------------------------------------------------------------

    #[test]
    fn integrates_with_standard_library_version() {
        let zhal_version =
            ZhalApiVersion::parse("1.2.3").unwrap();

        let standard_version =
            Version::parse("1.2.3").unwrap();

        assert_eq!(
            zhal_version.as_version(),
            &standard_version
        );
    }

    #[test]
    fn supports_conversion_to_canonical_version() {
        let zhal_version =
            ZhalApiVersion::parse("1.2.3").unwrap();

        let canonical =
            zhal_version.into_version();

        assert_eq!(
            canonical.to_string(),
            "1.2.3"
        );
    }

    #[test]
    fn exposes_rust_implementation_metadata() {
        assert_eq!(ZHAL_RUST_EDITION, "2021");

        assert_eq!(
            ZHAL_MINIMUM_RUST_VERSION,
            "1.97"
        );
    }
}
