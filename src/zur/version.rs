
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Version Contracts
//!
//! Path: `src/zur/version.rs`
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Defines version identities and explicit compatibility policies for
//! ZUR-owned runtime contracts.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Runtime API versions.
//! - Runtime protocol versions.
//! - Runtime ABI versions.
//! - Runtime service-contract versions.
//! - The numeric representation used by those version identities.
//! - Inclusive version ranges.
//! - Explicit compatibility policies and decisions.
//! - Parsing and validation of normalized runtime contract versions.
//!
//! This module does not own:
//!
//! - Cargo package versions.
//! - Zamani language versions.
//! - Grammar or lexer versions.
//! - AST or semantic-model versions.
//! - ZUIR schema or serialization versions.
//! - Classical, quantum, HDL, or other domain IR versions.
//! - Compiler release versions.
//! - Backend API versions.
//! - Backend artifact-format versions.
//! - Target compatibility versions.
//! - Physical hardware or firmware versions.
//! - Artifact manifests or artifact migration.
//! - Runtime execution or service discovery.
//!
//! Those responsibilities remain with their existing repository authorities.
//!
//! ## Compatibility principle
//!
//! Version equality does not prove semantic compatibility.
//!
//! A version is accepted only when an explicitly supplied compatibility
//! policy declares it directly compatible or migratable.
//!
//! Unknown, future, and out-of-range versions are rejected by default.
//!
//! ## Scalability
//!
//! Numeric version components are represented as canonical decimal strings.
//! No arbitrary numeric component ceiling is imposed by this module.
//!
//! Version ranges are inclusive and have no fixed number of entries.
//!
//! Actual storage and processing remain subject to available resources.
//!
//! ## Safety
//!
//! Safe Rust only. No global mutable state, unsafe code, automatic migration,
//! implicit fallback, or fabricated compatibility claims.

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

// ============================================================================
// ERRORS
// ============================================================================

/// Errors encountered while parsing or validating a runtime version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VersionParseError {
    /// The input is empty.
    Empty,

    /// The input does not contain exactly three numeric components.
    InvalidComponentCount,

    /// A component is empty.
    EmptyComponent,

    /// A component contains a non-decimal character.
    InvalidCharacter,

    /// A component contains a non-canonical leading zero.
    LeadingZero,
}

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Empty => "version must not be empty",
            Self::InvalidComponentCount => {
                "runtime contract version must contain exactly three components"
            }
            Self::EmptyComponent => "version component must not be empty",
            Self::InvalidCharacter => {
                "version components must contain decimal digits only"
            }
            Self::LeadingZero => {
                "version components must not contain leading zeroes"
            }
        })
    }
}

impl std::error::Error for VersionParseError {}

/// Errors encountered while constructing a version range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VersionRangeError {
    /// The minimum version is greater than the maximum version.
    MinimumExceedsMaximum,
}

impl fmt::Display for VersionRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::MinimumExceedsMaximum => {
                "version range minimum exceeds its maximum"
            }
        })
    }
}

impl std::error::Error for VersionRangeError {}

/// Errors encountered while constructing a compatibility policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CompatibilityPolicyError {
    /// The current version is outside the direct-compatibility range.
    CurrentVersionOutsideDirectRange,
}

impl fmt::Display for CompatibilityPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::CurrentVersionOutsideDirectRange => {
                "current version must be inside the direct-compatibility range"
            }
        })
    }
}

impl std::error::Error for CompatibilityPolicyError {}

// ============================================================================
// NUMERIC VERSION COMPONENT
// ============================================================================

/// A canonical, non-negative decimal version component.
///
/// Components are stored as strings rather than fixed-width integers.
/// This avoids introducing an arbitrary numeric version ceiling.
///
/// Examples:
///
/// - `0`
/// - `1`
/// - `42`
/// - `184467440737095516160000000000000000000`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionComponent(String);

impl VersionComponent {
    /// Creates a validated decimal component.
    pub fn new(value: impl Into<String>) -> Result<Self, VersionParseError> {
        let value = value.into();

        if value.is_empty() {
            return Err(VersionParseError::EmptyComponent);
        }

        if !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(VersionParseError::InvalidCharacter);
        }

        if value.len() > 1 && value.starts_with('0') {
            return Err(VersionParseError::LeadingZero);
        }

        Ok(Self(value))
    }

    /// Returns the canonical decimal representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for VersionComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialOrd for VersionComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.cmp(&other.0))
    }
}

// ============================================================================
// COMMON NUMERIC VERSION
// ============================================================================

/// A normalized three-component version.
///
/// The canonical representation is:
///
/// `MAJOR.MINOR.PATCH`
///
/// This type represents a version identity. It does not itself establish
/// compatibility.
///
/// Runtime contract versions intentionally use this normalized numeric
/// representation. Source-level version expressions, prerelease channels,
/// and version constraints are interpreted by the grammar and semantic
/// authorities before being converted into an appropriate runtime contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    major: VersionComponent,
    minor: VersionComponent,
    patch: VersionComponent,
}

impl Version {
    /// Constructs a version from three decimal components.
    pub fn new(
        major: impl Into<String>,
        minor: impl Into<String>,
        patch: impl Into<String>,
    ) -> Result<Self, VersionParseError> {
        Ok(Self {
            major: VersionComponent::new(major)?,
            minor: VersionComponent::new(minor)?,
            patch: VersionComponent::new(patch)?,
        })
    }

    /// Constructs a version from three validated components.
    #[must_use]
    pub fn from_components(
        major: VersionComponent,
        minor: VersionComponent,
        patch: VersionComponent,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major component.
    #[must_use]
    pub fn major(&self) -> &VersionComponent {
        &self.major
    }

    /// Returns the minor component.
    #[must_use]
    pub fn minor(&self) -> &VersionComponent {
        &self.minor
    }

    /// Returns the patch component.
    #[must_use]
    pub fn patch(&self) -> &VersionComponent {
        &self.patch
    }

    /// Returns the next patch version.
    ///
    /// Returns an error if the component cannot be represented because
    /// allocation or string capacity is unavailable.
    pub fn checked_next_patch(&self) -> Result<Self, VersionParseError> {
        let next = increment_decimal(self.patch.as_str())?;

        Self::new(
            self.major.as_str(),
            self.minor.as_str(),
            next,
        )
    }

    /// Returns the next minor version, resetting patch to zero.
    pub fn checked_next_minor(&self) -> Result<Self, VersionParseError> {
        let next = increment_decimal(self.minor.as_str())?;

        Self::new(
            self.major.as_str(),
            next,
            "0",
        )
    }

    /// Returns the next major version, resetting minor and patch to zero.
    pub fn checked_next_major(&self) -> Result<Self, VersionParseError> {
        let next = increment_decimal(self.major.as_str())?;

        Self::new(next, "0", "0")
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(VersionParseError::Empty);
        }

        let mut components = input.split('.');

        let major = components
            .next()
            .ok_or(VersionParseError::InvalidComponentCount)?;

        let minor = components
            .next()
            .ok_or(VersionParseError::InvalidComponentCount)?;

        let patch = components
            .next()
            .ok_or(VersionParseError::InvalidComponentCount)?;

        if components.next().is_some() {
            return Err(VersionParseError::InvalidComponentCount);
        }

        Self::new(major, minor, patch)
    }
}

/// Increments a canonical non-negative decimal string without converting it
/// into a fixed-width integer.
fn increment_decimal(value: &str) -> Result<String, VersionParseError> {
    let mut digits = value.as_bytes().to_vec();

    let mut index = digits.len();

    while index > 0 {
        index -= 1;

        if digits[index] < b'9' {
            digits[index] += 1;

            return String::from_utf8(digits)
                .map_err(|_| VersionParseError::InvalidCharacter);
        }

        digits[index] = b'0';
    }

    let mut result = String::with_capacity(digits.len() + 1);
    result.push('1');

    for digit in digits {
        result.push(char::from(digit));
    }

    Ok(result)
}

// ============================================================================
// STRONGLY TYPED VERSION IDENTITIES
// ============================================================================

/// Common interface implemented by runtime version categories.
///
/// The concrete version types intentionally do not convert implicitly
/// between one another.
pub trait VersionIdentity:
    Clone + Eq + Ord + fmt::Debug + fmt::Display + Send + Sync
{
    /// Returns the underlying numeric version.
    fn version(&self) -> &Version;
}

/// Defines a strongly typed runtime version category.
macro_rules! define_runtime_version {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(Version);

        impl $name {
            /// Creates a version from three decimal components.
            pub fn new(
                major: impl Into<String>,
                minor: impl Into<String>,
                patch: impl Into<String>,
            ) -> Result<Self, VersionParseError> {
                Ok(Self(Version::new(major, minor, patch)?))
            }

            /// Creates a version from an existing numeric version.
            #[must_use]
            pub fn from_version(version: Version) -> Self {
                Self(version)
            }

            /// Returns the underlying numeric version.
            #[must_use]
            pub fn version(&self) -> &Version {
                &self.0
            }

            /// Returns the major component.
            #[must_use]
            pub fn major(&self) -> &VersionComponent {
                self.0.major()
            }

            /// Returns the minor component.
            #[must_use]
            pub fn minor(&self) -> &VersionComponent {
                self.0.minor()
            }

            /// Returns the patch component.
            #[must_use]
            pub fn patch(&self) -> &VersionComponent {
                self.0.patch()
            }
        }

        impl VersionIdentity for $name {
            fn version(&self) -> &Version {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl FromStr for $name {
            type Err = VersionParseError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                input.parse::<Version>().map(Self)
            }
        }
    };
}

define_runtime_version!(
    /// Version of the public ZUR API contract.
    ///
    /// This identifies the API contract exposed to runtime consumers.
    RuntimeApiVersion
);

define_runtime_version!(
    /// Version of the ZUR runtime communication protocol.
    ///
    /// This identifies the protocol contract used between independently
    /// communicating runtime components.
    RuntimeProtocolVersion
);

define_runtime_version!(
    /// Version of the ZUR execution ABI contract.
    ///
    /// This identifies the calling, ownership, entry-point, and result
    /// transfer contract expected by runtime-compatible artifacts.
    RuntimeAbiVersion
);

define_runtime_version!(
    /// Version of an individual runtime service contract.
    ///
    /// The service identity is supplied by the service descriptor.
    /// This type does not define a closed inventory of runtime services.
    ServiceContractVersion
);

// ============================================================================
// VERSION RANGES
// ============================================================================

/// An inclusive range of versions.
///
/// The range is generic so runtime API, protocol, ABI, and service-contract
/// versions retain their distinct types.
///
/// Both endpoints are included.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VersionRange<V> {
    minimum: V,
    maximum: V,
}

impl<V> VersionRange<V>
where
    V: VersionIdentity,
{
    /// Constructs an inclusive version range.
    pub fn new(
        minimum: V,
        maximum: V,
    ) -> Result<Self, VersionRangeError> {
        if minimum > maximum {
            return Err(VersionRangeError::MinimumExceedsMaximum);
        }

        Ok(Self { minimum, maximum })
    }

    /// Returns the inclusive minimum version.
    #[must_use]
    pub fn minimum(&self) -> &V {
        &self.minimum
    }

    /// Returns the inclusive maximum version.
    #[must_use]
    pub fn maximum(&self) -> &V {
        &self.maximum
    }

    /// Returns whether a version is inside this inclusive range.
    #[must_use]
    pub fn contains(&self, version: &V) -> bool {
        &self.minimum <= version && version <= &self.maximum
    }
}

// ============================================================================
// COMPATIBILITY CLASSIFICATION
// ============================================================================

/// Result of evaluating a version against an explicit policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Compatibility {
    /// The version is accepted without migration.
    Compatible,

    /// The version is recognized but requires an explicit migration.
    RequiresMigration,

    /// The version is not accepted by the policy.
    Incompatible,
}

/// Reason for a compatibility decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityReason {
    /// The supplied version equals the current version.
    ExactMatch,

    /// The supplied version is in the declared direct-compatibility range.
    InsideDirectCompatibilityRange,

    /// The supplied version is in the declared migration range.
    InsideMigrationRange,

    /// The supplied version is newer than the current version.
    FutureVersion,

    /// The supplied version is older than the supported migration range.
    BelowSupportedRange,

    /// The supplied version is otherwise outside all declared ranges.
    OutsideSupportedRanges,
}

/// A complete compatibility decision.
///
/// This is evidence from a version policy, not proof of artifact integrity,
/// semantic equivalence, resource feasibility, or successful execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompatibilityDecision<V> {
    supplied: V,
    current: V,
    compatibility: Compatibility,
    reason: CompatibilityReason,
}

impl<V> CompatibilityDecision<V> {
    /// Returns the supplied version.
    #[must_use]
    pub fn supplied(&self) -> &V {
        &self.supplied
    }

    /// Returns the current version.
    #[must_use]
    pub fn current(&self) -> &V {
        &self.current
    }

    /// Returns the compatibility classification.
    #[must_use]
    pub const fn compatibility(&self) -> Compatibility {
        self.compatibility
    }

    /// Returns the reason for the decision.
    #[must_use]
    pub const fn reason(&self) -> CompatibilityReason {
        self.reason
    }

    /// Returns true only when the version is directly compatible.
    #[must_use]
    pub const fn is_directly_compatible(&self) -> bool {
        matches!(self.compatibility, Compatibility::Compatible)
    }

    /// Returns true when an explicit migration is required.
    #[must_use]
    pub const fn requires_migration(&self) -> bool {
        matches!(
            self.compatibility,
            Compatibility::RequiresMigration
        )
    }

    /// Returns true when the version must be rejected by this policy.
    #[must_use]
    pub const fn is_incompatible(&self) -> bool {
        matches!(
            self.compatibility,
            Compatibility::Incompatible
        )
    }
}

// ============================================================================
// EXPLICIT COMPATIBILITY POLICY
// ============================================================================

/// Explicit compatibility policy for one runtime version category.
///
/// The policy declares:
///
/// - The version implemented by the reader or runtime.
/// - Versions accepted without migration.
/// - Versions that are recognized but require migration.
///
/// Versions outside both ranges are incompatible.
///
/// No major-version heuristic is applied.
///
/// The policy does not implement migrations. A separate migration authority
/// must provide and execute an approved migration before interpretation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityPolicy<V> {
    current: V,
    directly_compatible: VersionRange<V>,
    migratable: Option<VersionRange<V>>,
}

impl<V> CompatibilityPolicy<V>
where
    V: VersionIdentity,
{
    /// Constructs a compatibility policy.
    ///
    /// The current version must be included in the direct-compatibility range.
    pub fn new(
        current: V,
        directly_compatible: VersionRange<V>,
        migratable: Option<VersionRange<V>>,
    ) -> Result<Self, CompatibilityPolicyError> {
        if !directly_compatible.contains(&current) {
            return Err(
                CompatibilityPolicyError::CurrentVersionOutsideDirectRange,
            );
        }

        Ok(Self {
            current,
            directly_compatible,
            migratable,
        })
    }

    /// Constructs a conservative exact-version-only policy.
    pub fn exact(current: V) -> Result<Self, VersionRangeError> {
        let range = VersionRange::new(
            current.clone(),
            current.clone(),
        )?;

        // The current version is necessarily inside its own exact range.
        Ok(Self {
            current,
            directly_compatible: range,
            migratable: None,
        })
    }

    /// Returns the current version.
    #[must_use]
    pub fn current(&self) -> &V {
        &self.current
    }

    /// Returns the direct-compatibility range.
    #[must_use]
    pub fn directly_compatible(&self) -> &VersionRange<V> {
        &self.directly_compatible
    }

    /// Returns the optional migration range.
    #[must_use]
    pub fn migratable(&self) -> Option<&VersionRange<V>> {
        self.migratable.as_ref()
    }

    /// Evaluates a supplied version against this policy.
    ///
    /// Direct compatibility takes precedence over migration compatibility.
    /// The method never executes a migration or silently accepts an
    /// unrecognized future version.
    #[must_use]
    pub fn check(&self, supplied: V) -> CompatibilityDecision<V> {
        let (compatibility, reason) =
            if supplied == self.current {
                (
                    Compatibility::Compatible,
                    CompatibilityReason::ExactMatch,
                )
            } else if self.directly_compatible.contains(&supplied) {
                (
                    Compatibility::Compatible,
                    CompatibilityReason::InsideDirectCompatibilityRange,
                )
            } else if self
                .migratable
                .as_ref()
                .is_some_and(|range| range.contains(&supplied))
            {
                (
                    Compatibility::RequiresMigration,
                    CompatibilityReason::InsideMigrationRange,
                )
            } else if supplied > self.current {
                (
                    Compatibility::Incompatible,
                    CompatibilityReason::FutureVersion,
                )
            } else if self
                .migratable
                .as_ref()
                .is_some_and(|range| supplied < *range.minimum())
            {
                (
                    Compatibility::Incompatible,
                    CompatibilityReason::BelowSupportedRange,
                )
            } else {
                (
                    Compatibility::Incompatible,
                    CompatibilityReason::OutsideSupportedRanges,
                )
            };

        CompatibilityDecision {
            supplied,
            current: self.current.clone(),
            compatibility,
            reason,
        }
    }
}

// ============================================================================
// RUNTIME CONTRACT COMPATIBILITY ALIASES
// ============================================================================

/// Compatibility policy for the public runtime API.
pub type RuntimeApiCompatibilityPolicy =
    CompatibilityPolicy<RuntimeApiVersion>;

/// Compatibility policy for the runtime communication protocol.
pub type RuntimeProtocolCompatibilityPolicy =
    CompatibilityPolicy<RuntimeProtocolVersion>;

/// Compatibility policy for the runtime execution ABI.
pub type RuntimeAbiCompatibilityPolicy =
    CompatibilityPolicy<RuntimeAbiVersion>;

/// Compatibility policy for an individual service contract.
pub type ServiceContractCompatibilityPolicy =
    CompatibilityPolicy<ServiceContractVersion>;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn version(value: &str) -> Version {
        value.parse().expect("test version must be valid")
    }

    fn api(value: &str) -> RuntimeApiVersion {
        value.parse().expect("test API version must be valid")
    }

    #[test]
    fn parses_and_formats_canonical_versions() {
        let parsed = version("1.12.300");

        assert_eq!(parsed.to_string(), "1.12.300");
        assert_eq!(parsed.major().as_str(), "1");
        assert_eq!(parsed.minor().as_str(), "12");
        assert_eq!(parsed.patch().as_str(), "300");
    }

    #[test]
    fn rejects_non_canonical_versions() {
        assert_eq!(
            "01.2.3".parse::<Version>(),
            Err(VersionParseError::LeadingZero)
        );

        assert_eq!(
            "1..3".parse::<Version>(),
            Err(VersionParseError::EmptyComponent)
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
            "1.2.x".parse::<Version>(),
            Err(VersionParseError::InvalidCharacter)
        );

        assert_eq!(
            "".parse::<Version>(),
            Err(VersionParseError::Empty)
        );
    }

    #[test]
    fn compares_large_components_numerically() {
        let lower = version("999999999999999999999999999999.0.0");
        let higher = version("1000000000000000000000000000000.0.0");

        assert!(lower < higher);
    }

    #[test]
    fn version_categories_are_distinct_types() {
        let api_version = api("1.0.0");

        let protocol_version: RuntimeProtocolVersion =
            "1.0.0".parse().expect("valid protocol version");

        assert_eq!(
            api_version.to_string(),
            protocol_version.to_string()
        );

        // The types are intentionally not implicitly interchangeable.
    }

    #[test]
    fn checked_increments_work_without_fixed_width_integer_conversion() {
        let initial = version("9.9.999999999999999999999999999999");

        assert_eq!(
            initial
                .checked_next_patch()
                .expect("increment should succeed")
                .to_string(),
            "9.9.1000000000000000000000000000000"
        );

        assert_eq!(
            initial
                .checked_next_minor()
                .expect("increment should succeed")
                .to_string(),
            "9.10.0"
        );

        assert_eq!(
            initial
                .checked_next_major()
                .expect("increment should succeed")
                .to_string(),
            "10.0.0"
        );
    }

    #[test]
    fn ranges_are_inclusive() {
        let range = VersionRange::new(
            api("1.0.0"),
            api("2.0.0"),
        )
        .expect("valid range");

        assert!(range.contains(&api("1.0.0")));
        assert!(range.contains(&api("1.5.0")));
        assert!(range.contains(&api("2.0.0")));
        assert!(!range.contains(&api("2.0.1")));
        assert!(!range.contains(&api("0.9.9")));
    }

    #[test]
    fn rejects_reversed_ranges() {
        let result = VersionRange::new(
            api("2.0.0"),
            api("1.0.0"),
        );

        assert_eq!(
            result,
            Err(VersionRangeError::MinimumExceedsMaximum)
        );
    }

    #[test]
    fn exact_policy_accepts_only_the_declared_version() {
        let policy = RuntimeApiCompatibilityPolicy::exact(
            api("1.0.0"),
        )
        .expect("exact policy");

        let exact = policy.check(api("1.0.0"));
        let future = policy.check(api("1.0.1"));
        let older = policy.check(api("0.9.0"));

        assert_eq!(
            exact.compatibility(),
            Compatibility::Compatible
        );

        assert_eq!(
            future.compatibility(),
            Compatibility::Incompatible
        );

        assert_eq!(
            future.reason(),
            CompatibilityReason::FutureVersion
        );

        assert_eq!(
            older.compatibility(),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn explicit_ranges_determine_compatibility() {
        let direct = VersionRange::new(
            api("2.0.0"),
            api("2.4.0"),
        )
        .expect("valid direct range");

        let migratable = VersionRange::new(
            api("1.0.0"),
            api("1.9.9"),
        )
        .expect("valid migration range");

        let policy = RuntimeApiCompatibilityPolicy::new(
            api("2.4.0"),
            direct,
            Some(migratable),
        )
        .expect("valid policy");

        assert_eq!(
            policy.check(api("2.2.0")).compatibility(),
            Compatibility::Compatible
        );

        assert_eq!(
            policy.check(api("1.5.0")).compatibility(),
            Compatibility::RequiresMigration
        );

        assert_eq!(
            policy.check(api("3.0.0")).compatibility(),
            Compatibility::Incompatible
        );

        assert_eq!(
            policy.check(api("0.9.0")).compatibility(),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn current_version_must_be_inside_direct_range() {
        let direct = VersionRange::new(
            api("1.0.0"),
            api("1.5.0"),
        )
        .expect("valid range");

        let result = RuntimeApiCompatibilityPolicy::new(
            api("2.0.0"),
            direct,
            None,
        );

        assert_eq!(
            result,
            Err(
                CompatibilityPolicyError::CurrentVersionOutsideDirectRange
            )
        );
    }

    #[test]
    fn direct_compatibility_takes_precedence_over_migration() {
        let direct = VersionRange::new(
            api("1.0.0"),
            api("2.0.0"),
        )
        .expect("valid direct range");

        let migratable = VersionRange::new(
            api("1.5.0"),
            api("2.5.0"),
        )
        .expect("valid migration range");

        let policy = RuntimeApiCompatibilityPolicy::new(
            api("2.0.0"),
            direct,
            Some(migratable),
        )
        .expect("valid policy");

        assert_eq!(
            policy.check(api("1.8.0")).compatibility(),
            Compatibility::Compatible
        );
    }
}
