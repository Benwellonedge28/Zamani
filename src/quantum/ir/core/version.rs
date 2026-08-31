//! Zamani Quantum IR — Version Contract
//!
//! Canonical versioning primitives for the hardware-independent Zamani
//! Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! This module owns the version contract of the canonical Zamani Quantum IR.
//!
//! It answers:
//!
//! - Which IR contract is this?
//! - Is this version newer, older, or equal to another IR version?
//! - Can this IR version be consumed by this implementation?
//! - Does the version require migration?
//! - Is a compatibility decision exact, backward-compatible, or unsupported?
//!
//! This module deliberately does NOT own:
//!
//! - Zamani language versions;
//! - compiler versions;
//! - frontend versions;
//! - backend versions;
//! - hardware versions;
//! - calibration versions;
//! - target/device versions;
//! - logical qubit identity;
//! - physical qubit identity;
//! - quantum operations;
//! - serialization implementation;
//! - hashing implementation;
//! - migration implementation.
//!
//! Logical and physical qubit identity remain owned by:
//!
//! `quantum::ir::qubit`
//!
//! In particular, this module intentionally does not import
//! `quantum::ir::qubit`. Versioning and qubit identity are independent
//! contracts and must remain independently reusable.
//!
//! # Universal-program principle
//!
//! The Quantum IR has no fixed quantum-machine size.
//!
//! IR versioning therefore MUST NOT encode:
//!
//! - a maximum qubit count;
//! - a maximum register size;
//! - a maximum operation count;
//! - a hardware topology;
//! - a hardware architecture;
//! - a vendor;
//! - a backend;
//! - a simulator.
//!
//! A version identifies the meaning and structural contract of the IR.
//!
//! It does not identify the size of a quantum machine.
//!
//! # Version semantics
//!
//! The canonical version follows a semantic three-component model:
//!
//! ```text
//! MAJOR.MINOR.PATCH
//! ```
//!
//! ## MAJOR
//!
//! A major version change may introduce incompatible semantic or structural
//! changes that cannot safely be interpreted by an implementation of the
//! previous major contract.
//!
//! Example:
//!
//! ```text
//! 1.x.x -> 2.x.x
//! ```
//!
//! requires explicit compatibility handling or migration.
//!
//! ## MINOR
//!
//! A minor version adds compatible IR capabilities within the same major
//! semantic contract.
//!
//! Example:
//!
//! ```text
//! 1.0.0 -> 1.1.0
//! ```
//!
//! A consumer implementing 1.1.0 may consume an explicitly supported older
//! 1.0.x representation, subject to the serialization/extension rules.
//!
//! A 1.0.0 implementation MUST NOT automatically claim support for 1.1.0,
//! because it cannot assume that it understands the newly introduced
//! semantics.
//!
//! ## PATCH
//!
//! A patch version represents a contract-preserving correction.
//!
//! Example:
//!
//! ```text
//! 1.0.0 -> 1.0.1
//! ```
//!
//! A consumer may accept an older patch version within the same known minor
//! contract.
//!
//! A future patch version is conservatively rejected by this module because
//! the implementation cannot assume that every future patch-level correction
//! is recognizable without an explicit compatibility policy.
//!
//! # Compatibility philosophy
//!
//! Compatibility is deliberately conservative.
//!
//! Unknown future semantics MUST NOT be silently interpreted as known
//! semantics.
//!
//! The safe default is:
//!
//! ```text
//! known older contract     -> potentially supported
//! exact known contract     -> supported
//! known future contract    -> reject / negotiate / migrate
//! unknown major            -> reject / migrate
//! ```
//!
//! The serialization layer remains responsible for deciding whether unknown
//! fields/extensions can be preserved losslessly.
//!
//! This module only answers version-contract questions.
//!
//! # Version versus migration
//!
//! A version being older does not necessarily mean that migration is required.
//!
//! For example:
//!
//! ```text
//! 1.0.0 -> 1.0.0
//! ```
//!
//! requires no migration.
//!
//! ```text
//! 1.0.0 -> 1.0.1
//! ```
//!
//! may be consumed without semantic migration if the implementation explicitly
//! supports that older version.
//!
//! A major-version transition normally requires migration or explicit
//! incompatibility handling.
//!
//! Migration algorithms belong outside this file.
//!
//! # Determinism
//!
//! `IrVersion` is:
//!
//! - immutable;
//! - `Copy`;
//! - `Eq`;
//! - `Ord`;
//! - `Hash`;
//! - deterministic;
//! - architecture-independent;
//! - serialization-friendly.
//!
//! It contains no pointers, addresses, process-local state, timestamps,
//! randomness, hardware information, or global mutable state.
//!
//! # Integer representation
//!
//! Version components use `u16`.
//!
//! This is intentionally independent of machine word size and therefore does
//! not depend on `usize`.
//!
//! Version components are not resource counters.
//!
//! A version component is limited only by the representable version schema,
//! not by quantum-machine size.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! This module contains no unsafe code and explicitly forbids it.
//!
//! # Integration contract
//!
//! The canonical dependency direction is:
//!
//! ```text
//! core::version
//!       │
//!       ├── identity
//!       ├── serialization
//!       ├── hashing
//!       ├── validation
//!       ├── provenance
//!       └── compatibility
//! ```
//!
//! No arrow points back into `core::version` from those modules in a way that
//! creates a circular dependency.
//!
//! `identity.rs` must eventually consume or re-export `IrVersion` from this
//! module rather than defining a second `IrVersion` type.
//!
//! `serialization.rs` must use this type for the IR version field.
//!
//! `hash.rs` must include the canonical IR version when version participates
//! in semantic content identity.
//!
//! `validation.rs` must use this module for version compatibility checks.
//!
//! `compatibility/migration.rs` owns actual migration transformations.
//!
//! # Public API stability
//!
//! The following items form the stable version API:
//!
//! - `IrVersion`;
//! - `IrVersionError`;
//! - `VersionCompatibility`;
//! - `IrVersion::CURRENT`;
//! - `IrVersion::new`;
//! - `IrVersion::major`;
//! - `IrVersion::minor`;
//! - `IrVersion::patch`;
//! - `IrVersion::current`;
//! - `IrVersion::same_major`;
//! - `IrVersion::is_exact`;
//! - `IrVersion::is_current`;
//! - `IrVersion::is_older_than`;
//! - `IrVersion::is_newer_than`;
//! - `IrVersion::is_supported_by`;
//! - `IrVersion::is_supported_by_current`;
//! - `IrVersion::compatibility_with`;
//! - `IrVersion::requires_migration_to`;
//! - `IrVersion::to_string` / `Display`;
//! - `FromStr` parsing.
//!
//! Implementations elsewhere in the IR should use this contract rather than
//! duplicating version comparison logic.
//!
//! # Important ownership rule
//!
//! This file owns VERSION SEMANTICS.
//!
//! It does not own VERSION MIGRATION.
//!
//! It does not own SERIALIZATION.
//!
//! It does not own HASHING.
//!
//! It does not own QUANTUM RESOURCE LIMITS.
//!
//! It does not own QUANTUM MACHINE CAPACITY.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::str::FromStr;

// =============================================================================
// Constants
// =============================================================================

/// Current stable Zamani Quantum IR major version.
pub const CURRENT_IR_MAJOR: u16 = 1;

/// Current stable Zamani Quantum IR minor version.
pub const CURRENT_IR_MINOR: u16 = 0;

/// Current stable Zamani Quantum IR patch version.
pub const CURRENT_IR_PATCH: u16 = 0;

// =============================================================================
// IR version
// =============================================================================

/// Version of the Zamani Quantum IR semantic and structural contract.
///
/// `IrVersion` is independent of:
///
/// - the Zamani language version;
/// - the Zamani compiler version;
/// - frontend versions;
/// - backend versions;
/// - hardware versions;
/// - calibration versions.
///
/// It identifies the IR contract only.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
///
/// use zamani_compiler::quantum::ir::core::version::IrVersion;
///
/// let version = IrVersion::new(1, 0, 0);
///
/// assert_eq!(version.major(), 1);
/// assert_eq!(version.minor(), 0);
/// assert_eq!(version.patch(), 0);
/// assert!(version.is_current());
///
/// let parsed = IrVersion::from_str("1.0.0").expect("valid IR version");
/// assert_eq!(parsed, version);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IrVersion {
    major: u16,
    minor: u16,
    patch: u16,
}

impl IrVersion {
    // =========================================================================
    // Current version
    // =========================================================================

    /// Current stable Quantum IR contract.
    ///
    /// This is the single canonical current version for the IR.
    ///
    /// If the IR contract changes:
    ///
    /// - compatible capability addition -> increment minor;
    /// - contract-preserving correction -> increment patch;
    /// - breaking semantic/structural change -> increment major.
    pub const CURRENT: Self = Self::new(
        CURRENT_IR_MAJOR,
        CURRENT_IR_MINOR,
        CURRENT_IR_PATCH,
    );

    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates an IR version.
    ///
    /// Version components are structurally valid as unsigned integers.
    ///
    /// This constructor does not claim that the resulting version is known,
    /// supported, current, or compatible.
    ///
    /// Compatibility must be checked explicitly.
    #[must_use]
    pub const fn new(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the current stable IR version.
    #[must_use]
    pub const fn current() -> Self {
        Self::CURRENT
    }

    // =========================================================================
    // Accessors
    // =========================================================================

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u16 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u16 {
        self.minor
    }

    /// Returns the patch version.
    #[must_use]
    pub const fn patch(self) -> u16 {
        self.patch
    }

    // =========================================================================
    // Exact comparison
    // =========================================================================

    /// Returns `true` when both versions are exactly equal.
    #[must_use]
    pub const fn is_exact(self, other: Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Alias for [`Self::is_exact`].
    #[must_use]
    pub const fn is_exactly(self, other: Self) -> bool {
        self.is_exact(other)
    }

    /// Returns `true` when this version is the current stable version.
    #[must_use]
    pub const fn is_current(self) -> bool {
        self.is_exact(Self::CURRENT)
    }

    /// Returns `true` when both versions have the same major version.
    #[must_use]
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns `true` when this version is older than `other`.
    #[must_use]
    pub const fn is_older_than(self, other: Self) -> bool {
        self < other
    }

    /// Returns `true` when this version is newer than `other`.
    #[must_use]
    pub const fn is_newer_than(self, other: Self) -> bool {
        self > other
    }

    // =========================================================================
    // Current-version comparison
    // =========================================================================

    /// Returns whether this version is older than the current stable version.
    #[must_use]
    pub const fn is_older_than_current(self) -> bool {
        self < Self::CURRENT
    }

    /// Returns whether this version is newer than the current stable version.
    #[must_use]
    pub const fn is_newer_than_current(self) -> bool {
        self > Self::CURRENT
    }

    // =========================================================================
    // Compatibility
    // =========================================================================

    /// Determines whether `self` can be consumed by `consumer`.
    ///
    /// Compatibility is deliberately conservative:
    ///
    /// - same known version -> supported;
    /// - older minor within the same major -> supported;
    /// - older patch within the same minor -> supported;
    /// - future minor -> unsupported;
    /// - future patch within the current minor -> unsupported;
    /// - different major -> unsupported.
    ///
    /// The function does not perform migration.
    ///
    /// # Arguments
    ///
    /// `consumer` is the version implemented by the consuming component.
    #[must_use]
    pub const fn is_supported_by(
        self,
        consumer: Self,
    ) -> bool {
        self.major == consumer.major
            && (
                self.minor < consumer.minor
                || (
                    self.minor == consumer.minor
                    && self.patch <= consumer.patch
                )
            )
    }

    /// Determines whether this version is supported by the current
    /// implementation.
    #[must_use]
    pub const fn is_supported_by_current(self) -> bool {
        self.is_supported_by(Self::CURRENT)
    }

    /// Determines the compatibility relationship between this version and a
    /// consumer version.
    #[must_use]
    pub const fn compatibility_with(
        self,
        consumer: Self,
    ) -> VersionCompatibility {
        if self.is_exact(consumer) {
            VersionCompatibility::Exact
        } else if self.major != consumer.major {
            VersionCompatibility::MajorMismatch
        } else if self > consumer {
            if self.minor > consumer.minor {
                VersionCompatibility::FutureMinor
            } else {
                VersionCompatibility::FuturePatch
            }
        } else {
            VersionCompatibility::OlderCompatible
        }
    }

    /// Determines the compatibility relationship with the current IR
    /// implementation.
    #[must_use]
    pub const fn compatibility_with_current(
        self,
    ) -> VersionCompatibility {
        self.compatibility_with(Self::CURRENT)
    }

    /// Returns whether migration is required for a consumer using `consumer`.
    ///
    /// This function reports whether the version relationship crosses a
    /// compatibility boundary. It does not perform migration.
    #[must_use]
    pub const fn requires_migration_to(
        self,
        consumer: Self,
    ) -> bool {
        !self.is_supported_by(consumer)
    }

    /// Returns whether migration is required for the current IR implementation.
    #[must_use]
    pub const fn requires_migration_to_current(self) -> bool {
        self.requires_migration_to(Self::CURRENT)
    }

    // =========================================================================
    // Version construction helpers
    // =========================================================================

    /// Returns a new version with the specified major component.
    ///
    /// This is useful for tests and compatibility tooling.
    #[must_use]
    pub const fn with_major(
        self,
        major: u16,
    ) -> Self {
        Self::new(major, self.minor, self.patch)
    }

    /// Returns a new version with the specified minor component.
    #[must_use]
    pub const fn with_minor(
        self,
        minor: u16,
    ) -> Self {
        Self::new(self.major, minor, self.patch)
    }

    /// Returns a new version with the specified patch component.
    #[must_use]
    pub const fn with_patch(
        self,
        patch: u16,
    ) -> Self {
        Self::new(self.major, self.minor, patch)
    }
}

impl Default for IrVersion {
    fn default() -> Self {
        Self::CURRENT
    }
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for IrVersion {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

// =============================================================================
// Parsing errors
// =============================================================================

/// Error returned when parsing an IR version string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IrVersionError {
    /// The input does not contain exactly three components.
    InvalidComponentCount {
        /// Number of components encountered.
        count: usize,
    },

    /// A version component is empty.
    EmptyComponent {
        /// Zero-based component position.
        index: usize,
    },

    /// A component is not a valid unsigned 16-bit integer.
    InvalidComponent {
        /// Zero-based component position.
        index: usize,

        /// The invalid component text.
        value: String,
    },

    /// The input contains surrounding whitespace.
    ///
    /// Version parsing is intentionally strict so that canonical version
    /// strings remain deterministic.
    SurroundingWhitespace,
}

impl fmt::Display for IrVersionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidComponentCount { count } => {
                write!(
                    formatter,
                    "invalid IR version: expected exactly 3 components, found {count}"
                )
            }

            Self::EmptyComponent { index } => {
                write!(
                    formatter,
                    "invalid IR version: component {index} is empty"
                )
            }

            Self::InvalidComponent { index, value } => {
                write!(
                    formatter,
                    "invalid IR version: component {index} \
                     is not a valid u16: {value:?}"
                )
            }

            Self::SurroundingWhitespace => {
                write!(
                    formatter,
                    "invalid IR version: surrounding whitespace is not permitted"
                )
            }
        }
    }
}

impl std::error::Error for IrVersionError {}

// =============================================================================
// FromStr
// =============================================================================

impl FromStr for IrVersion {
    type Err = IrVersionError;

    /// Parses a canonical `MAJOR.MINOR.PATCH` version.
    ///
    /// Parsing is intentionally strict.
    ///
    /// Accepted:
    ///
    /// ```text
    /// 0.0.0
    /// 1.0.0
    /// 1.12.3
    /// 65535.65535.65535
    /// ```
    ///
    /// Rejected:
    ///
    /// ```text
    /// 1
    /// 1.0
    /// 1.0.0.0
    /// v1.0.0
    /// 1.0
    /// 1.0.x
    ///  1.0.0
    /// 1.0.0
    /// ```
    fn from_str(
        input: &str,
    ) -> Result<Self, Self::Err> {
        if input.trim() != input {
            return Err(IrVersionError::SurroundingWhitespace);
        }

        let components: Vec<&str> = input.split('.').collect();

        if components.len() != 3 {
            return Err(IrVersionError::InvalidComponentCount {
                count: components.len(),
            });
        }

        let mut values = [0_u16; 3];

        for (index, component) in components.iter().enumerate() {
            if component.is_empty() {
                return Err(IrVersionError::EmptyComponent { index });
            }

            values[index] = match component.parse::<u16>() {
                Ok(value) => value,
                Err(_) => {
                    return Err(IrVersionError::InvalidComponent {
                        index,
                        value: (*component).to_owned(),
                    });
                }
            };
        }

        Ok(Self::new(
            values[0],
            values[1],
            values[2],
        ))
    }
}

// =============================================================================
// Compatibility result
// =============================================================================

/// Compatibility relationship between an IR version and a consumer version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VersionCompatibility {
    /// Both versions are exactly equal.
    Exact,

    /// The supplied IR version is older but remains within the consumer's
    /// supported major contract.
    OlderCompatible,

    /// The supplied IR version has a different major version.
    ///
    /// Explicit migration or rejection is required.
    MajorMismatch,

    /// The supplied IR version contains a future minor version that this
    /// consumer cannot safely interpret.
    FutureMinor,

    /// The supplied IR version contains a future patch version that this
    /// consumer cannot safely claim to understand.
    FuturePatch,
}

impl VersionCompatibility {
    /// Returns whether the compatibility relationship is accepted.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        matches!(
            self,
            Self::Exact | Self::OlderCompatible
        )
    }

    /// Returns whether explicit migration is required.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        matches!(
            self,
            Self::MajorMismatch
                | Self::FutureMinor
                | Self::FuturePatch
        )
    }

    /// Returns whether this relationship represents an exact match.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }

    /// Returns whether the supplied IR is an older compatible version.
    #[must_use]
    pub const fn is_older_compatible(self) -> bool {
        matches!(self, Self::OlderCompatible)
    }

    /// Returns whether the versions have incompatible major contracts.
    #[must_use]
    pub const fn is_major_mismatch(self) -> bool {
        matches!(self, Self::MajorMismatch)
    }

    /// Returns whether the supplied IR is from a future minor version.
    #[must_use]
    pub const fn is_future_minor(self) -> bool {
        matches!(self, Self::FutureMinor)
    }

    /// Returns whether the supplied IR is from a future patch version.
    #[must_use]
    pub const fn is_future_patch(self) -> bool {
        matches!(self, Self::FuturePatch)
    }
}

impl fmt::Display for VersionCompatibility {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Exact => {
                write!(formatter, "exact")
            }

            Self::OlderCompatible => {
                write!(formatter, "older-compatible")
            }

            Self::MajorMismatch => {
                write!(formatter, "major-mismatch")
            }

            Self::FutureMinor => {
                write!(formatter, "future-minor")
            }

            Self::FuturePatch => {
                write!(formatter, "future-patch")
            }
        }
    }
}

// =============================================================================
// Canonical helper functions
// =============================================================================

/// Returns the current canonical Quantum IR version.
#[must_use]
pub const fn current_ir_version() -> IrVersion {
    IrVersion::CURRENT
}

/// Returns whether an IR version is supported by the current implementation.
#[must_use]
pub const fn is_supported_ir_version(
    version: IrVersion,
) -> bool {
    version.is_supported_by_current()
}

/// Returns the compatibility relationship between an IR version and the
/// current implementation.
#[must_use]
pub const fn ir_version_compatibility(
    version: IrVersion,
) -> VersionCompatibility {
    version.compatibility_with_current()
}

/// Returns whether an IR version requires migration before being consumed by
/// the current implementation.
#[must_use]
pub const fn ir_version_requires_migration(
    version: IrVersion,
) -> bool {
    version.requires_migration_to_current()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Construction
    // =========================================================================

    #[test]
    fn current_version_is_canonical() {
        let version = IrVersion::CURRENT;

        assert_eq!(version.major(), CURRENT_IR_MAJOR);
        assert_eq!(version.minor(), CURRENT_IR_MINOR);
        assert_eq!(version.patch(), CURRENT_IR_PATCH);
    }

    #[test]
    fn default_is_current() {
        assert_eq!(
            IrVersion::default(),
            IrVersion::CURRENT
        );
    }

    #[test]
    fn constructor_preserves_components() {
        let version = IrVersion::new(7, 12, 42);

        assert_eq!(version.major(), 7);
        assert_eq!(version.minor(), 12);
        assert_eq!(version.patch(), 42);
    }

    // =========================================================================
    // Equality and ordering
    // =========================================================================

    #[test]
    fn exact_versions_are_equal() {
        let a = IrVersion::new(1, 2, 3);
        let b = IrVersion::new(1, 2, 3);

        assert!(a.is_exact(b));
        assert_eq!(a, b);
    }

    #[test]
    fn ordering_is_semantic_version_order() {
        assert!(
            IrVersion::new(1, 0, 0)
                < IrVersion::new(1, 0, 1)
        );

        assert!(
            IrVersion::new(1, 0, 1)
                < IrVersion::new(1, 1, 0)
        );

        assert!(
            IrVersion::new(1, 1, 0)
                < IrVersion::new(2, 0, 0)
        );
    }

    // =========================================================================
    // Compatibility
    // =========================================================================

    #[test]
    fn exact_version_is_supported() {
        let version = IrVersion::CURRENT;

        assert!(version.is_supported_by_current());
        assert!(!version.requires_migration_to_current());
        assert_eq!(
            version.compatibility_with_current(),
            VersionCompatibility::Exact
        );
    }

    #[test]
    fn older_patch_is_supported() {
        let version = IrVersion::new(
            CURRENT_IR_MAJOR,
            CURRENT_IR_MINOR,
            0,
        );

        assert!(version.is_supported_by_current());
    }

    #[test]
    fn older_minor_is_supported() {
        let consumer = IrVersion::new(1, 4, 0);
        let producer = IrVersion::new(1, 2, 7);

        assert!(producer.is_supported_by(consumer));

        assert_eq!(
            producer.compatibility_with(consumer),
            VersionCompatibility::OlderCompatible
        );
    }

    #[test]
    fn future_minor_is_not_supported() {
        let consumer = IrVersion::new(1, 0, 0);
        let producer = IrVersion::new(1, 1, 0);

        assert!(!producer.is_supported_by(consumer));

        assert_eq!(
            producer.compatibility_with(consumer),
            VersionCompatibility::FutureMinor
        );

        assert!(producer.requires_migration_to(consumer));
    }

    #[test]
    fn future_patch_is_not_supported() {
        let consumer = IrVersion::new(1, 0, 0);
        let producer = IrVersion::new(1, 0, 1);

        assert!(!producer.is_supported_by(consumer));

        assert_eq!(
            producer.compatibility_with(consumer),
            VersionCompatibility::FuturePatch
        );

        assert!(producer.requires_migration_to(consumer));
    }

    #[test]
    fn major_mismatch_is_not_supported() {
        let consumer = IrVersion::new(1, 0, 0);
        let producer = IrVersion::new(2, 0, 0);

        assert!(!producer.is_supported_by(consumer));

        assert_eq!(
            producer.compatibility_with(consumer),
            VersionCompatibility::MajorMismatch
        );

        assert!(producer.requires_migration_to(consumer));
    }

    // =========================================================================
    // Parsing
    // =========================================================================

    #[test]
    fn parses_canonical_version() {
        let parsed = "1.2.3"
            .parse::<IrVersion>()
            .expect("valid version");

        assert_eq!(
            parsed,
            IrVersion::new(1, 2, 3)
        );
    }

    #[test]
    fn parses_maximum_component_values() {
        let parsed = "65535.65535.65535"
            .parse::<IrVersion>()
            .expect("valid maximum u16 components");

        assert_eq!(
            parsed,
            IrVersion::new(
                u16::MAX,
                u16::MAX,
                u16::MAX
            )
        );
    }

    #[test]
    fn rejects_missing_components() {
        assert!(
            "1.0"
                .parse::<IrVersion>()
                .is_err()
        );
    }

    #[test]
    fn rejects_extra_components() {
        assert!(
            "1.0.0.1"
                .parse::<IrVersion>()
                .is_err()
        );
    }

    #[test]
    fn rejects_non_numeric_components() {
        assert!(
            "1.x.0"
                .parse::<IrVersion>()
                .is_err()
        );
    }

    #[test]
    fn rejects_empty_components() {
        assert!(
            "1..0"
                .parse::<IrVersion>()
                .is_err()
        );
    }

    #[test]
    fn rejects_surrounding_whitespace() {
        assert!(
            " 1.0.0"
                .parse::<IrVersion>()
                .is_err()
        );

        assert!(
            "1.0.0 "
                .parse::<IrVersion>()
                .is_err()
        );
    }

    #[test]
    fn rejects_overflowing_components() {
        assert!(
            "65536.0.0"
                .parse::<IrVersion>()
                .is_err()
        );
    }

    // =========================================================================
    // Display / parse round trip
    // =========================================================================

    #[test]
    fn display_is_canonical() {
        let version = IrVersion::new(12, 34, 56);

        assert_eq!(
            version.to_string(),
            "12.34.56"
        );
    }

    #[test]
    fn display_round_trips_through_parser() {
        let original = IrVersion::new(12, 34, 56);

        let encoded = original.to_string();

        let decoded = encoded
            .parse::<IrVersion>()
            .expect("display output must parse");

        assert_eq!(decoded, original);
    }

    // =========================================================================
    // Helper functions
    // =========================================================================

    #[test]
    fn current_version_helper_is_correct() {
        assert_eq!(
            current_ir_version(),
            IrVersion::CURRENT
        );
    }

    #[test]
    fn current_support_helper_is_correct() {
        assert!(
            is_supported_ir_version(
                IrVersion::CURRENT
            )
        );
    }

    #[test]
    fn compatibility_helper_is_correct() {
        assert_eq!(
            ir_version_compatibility(
                IrVersion::CURRENT
            ),
            VersionCompatibility::Exact
        );
    }

    // =========================================================================
    // No resource-size coupling
    // =========================================================================

    #[test]
    fn version_has_no_quantum_resource_semantics() {
        let version = IrVersion::new(
            u16::MAX,
            u16::MAX,
            u16::MAX,
        );

        assert_eq!(
            version.major(),
            u16::MAX
        );

        assert_eq!(
            version.minor(),
            u16::MAX
        );

        assert_eq!(
            version.patch(),
            u16::MAX
        );
    }

    // =========================================================================
    // Strong compatibility behavior
    // =========================================================================

    #[test]
    fn compatibility_is_directional() {
        let old = IrVersion::new(1, 0, 0);
        let new = IrVersion::new(1, 1, 0);

        assert_eq!(
            old.compatibility_with(new),
            VersionCompatibility::OlderCompatible
        );

        assert_eq!(
            new.compatibility_with(old),
            VersionCompatibility::FutureMinor
        );
    }

    // =========================================================================
    // Modifier helpers
    // =========================================================================

    #[test]
    fn version_modifiers_preserve_other_components() {
        let version = IrVersion::new(1, 2, 3);

        assert_eq!(
            version.with_major(4),
            IrVersion::new(4, 2, 3)
        );

        assert_eq!(
            version.with_minor(5),
            IrVersion::new(1, 5, 3)
        );

        assert_eq!(
            version.with_patch(6),
            IrVersion::new(1, 2, 6)
        );
    }
}