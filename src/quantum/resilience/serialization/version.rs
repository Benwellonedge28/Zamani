//! Zamani Quantum Resilience — Serialization Version Policy.
//!
//! Path:
//!     src/quantum/resilience/serialization/version.rs
//!
//! # Purpose
//!
//! This module is the single authoritative policy boundary for compatibility
//! between versions of the Zamani quantum-resilience serialization schema.
//!
//! It answers:
//!
//! - Is a serialized schema exactly the version this implementation expects?
//! - Can it be decoded directly?
//! - Can it be decoded with compatible degradation?
//! - Does it require an explicit migration?
//! - Is it incompatible?
//! - Is compatibility conditional on caller policy?
//!
//! It does NOT:
//!
//! - serialize bytes;
//! - deserialize bytes;
//! - migrate domain objects;
//! - modify quantum IR;
//! - define QubitId or PhysicalQubitId;
//! - define ZQN faults;
//! - inspect hardware;
//! - inspect a backend;
//! - perform recovery;
//! - execute a quantum program;
//! - access the filesystem;
//! - access the network;
//! - access a clock;
//! - use randomness;
//! - contain hardware-size limits.
//!
//! Those responsibilities belong to their owning modules.
//!
//! # Architectural position
//!
//! ```text
//! serialization/schema.rs
//!          |
//!          v
//! serialization/version.rs
//!          |
//!      +---+---+
//!      |       |
//!      v       v
//!    encode   decode
//!      |       |
//!      +---+---+
//!          |
//!          v
//!     domain object
//! ```
//!
//! `schema.rs` owns the representation of a schema version.
//!
//! `version.rs` owns the interpretation of compatibility between versions.
//!
//! `encode.rs` owns encoding.
//!
//! `decode.rs` owns decoding.
//!
//! This separation prevents version policy from being duplicated in every
//! codec.
//!
//! # Compatibility model
//!
//! The production compatibility model is intentionally conservative:
//!
//! ```text
//! exact same version
//!     -> Compatible
//!
//! same major + newer reader + older/equal minor
//!     -> Compatible
//!
//! same major + older reader + newer minor
//!     -> CompatibleWithDegradation
//!
//! known compatible historical version with registered migration
//!     -> CompatibleWithMigration
//!
//! different major
//!     -> Incompatible
//!
//! future/unknown compatibility information
//!     -> Unknown / Conditional according to policy
//! ```
//!
//! A newer minor schema is NOT automatically treated as fully compatible.
//! A reader may only directly consume it when its unknown additions are
//! explicitly allowed by the schema's forward-extension contract.
//!
//! # Write once, scale everywhere
//!
//! Version compatibility contains no assumptions about:
//!
//! - number of qubits;
//! - number of logical qubits;
//! - number of physical qubits;
//! - number of operations;
//! - number of devices;
//! - number of backends;
//! - topology size;
//! - checkpoint count;
//! - telemetry volume.
//!
//! Version components are fixed-width schema metadata, while serialized
//! collections remain bounded only by the resource policies of their owning
//! layers.
//!
//! There is therefore no artificial machine-size ceiling here.
//!
//! # Determinism
//!
//! All compatibility decisions are deterministic.
//!
//! Given the same:
//!
//! - reader version;
//! - writer version;
//! - schema metadata;
//! - compatibility policy;
//! - migration availability;
//!
//! this module returns the same result.
//!
//! No time, randomness, process state, thread state, memory address or
//! environment variable participates in compatibility decisions.
//!
//! # Security
//!
//! Version metadata is untrusted input.
//!
//! A compatible schema version does NOT imply:
//!
//! - authorization;
//! - integrity;
//! - authenticity;
//! - trust;
//! - safe execution;
//! - safe recovery;
//! - safe checkpoint restoration.
//!
//! The caller must independently perform structural, integrity, security and
//! domain validation.
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
//! - no unsafe Rust.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use super::schema::{
    SchemaCompatibility,
    SchemaEnvelope,
    SchemaId,
    SchemaVersion,
    CURRENT_SCHEMA_VERSION,
};

// =============================================================================
// Version policy constants
// =============================================================================

/// The version policy revision.
///
/// This identifies the compatibility-policy implementation, not the persisted
/// schema version.
pub const VERSION_POLICY_REVISION: u32 = 1;

/// The oldest schema major version for which this implementation has a
/// first-class compatibility contract.
///
/// A value of `1` means that schema major 1 is the initial production family.
pub const MIN_SUPPORTED_SCHEMA_MAJOR: u16 = 1;

/// The newest schema major version understood by this implementation.
pub const MAX_SUPPORTED_SCHEMA_MAJOR: u16 = CURRENT_SCHEMA_VERSION.major();

/// The current reader schema version.
pub const READER_SCHEMA_VERSION: SchemaVersion = CURRENT_SCHEMA_VERSION;

// =============================================================================
// Compatibility policy
// =============================================================================

/// Controls how aggressively a caller permits forward compatibility.
///
/// This policy does not change the schema itself. It controls whether the
/// caller is willing to consume a representation containing newer compatible
/// metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityPolicy {
    /// Accept only schemas that this implementation can consume without
    /// migration or degradation.
    Strict,

    /// Permit compatible minor-version additions when the schema explicitly
    /// declares forward extensibility.
    ForwardCompatible,

    /// Permit compatible minor-version additions and compatible degradation.
    ///
    /// This is appropriate for telemetry/history consumers that can safely
    /// ignore optional data while preserving known semantics.
    Degraded,

    /// Require an explicit migration whenever the writer version differs from
    /// the reader version.
    MigrationRequired,
}

impl Default for CompatibilityPolicy {
    fn default() -> Self {
        Self::Strict
    }
}

// =============================================================================
// Compatibility decision
// =============================================================================

/// Detailed reason for a compatibility decision.
///
/// This is deliberately more precise than `SchemaCompatibility`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompatibilityReason {
    /// Reader and writer versions are identical.
    ExactMatch,

    /// Writer uses an older compatible patch version.
    OlderPatch,

    /// Writer uses an older compatible minor version.
    OlderMinor,

    /// Writer uses a newer minor version and the schema is forward-extensible.
    NewerMinorForwardCompatible,

    /// Writer uses a newer minor version and the caller permits degraded
    /// interpretation.
    NewerMinorDegraded,

    /// An explicit migration is required.
    MigrationRequired,

    /// The writer major version is not understood.
    UnsupportedMajor,

    /// The schema version is outside the supported compatibility range.
    UnsupportedVersion,

    /// Compatibility requires additional runtime/domain information.
    Conditional,

    /// The supplied compatibility information is insufficient.
    Unknown,
}

impl fmt::Display for CompatibilityReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::ExactMatch => "exact schema version match",
            Self::OlderPatch => "older compatible patch version",
            Self::OlderMinor => "older compatible minor version",
            Self::NewerMinorForwardCompatible => {
                "newer minor version permitted by forward-compatibility policy"
            }
            Self::NewerMinorDegraded => {
                "newer minor version accepted with compatible degradation"
            }
            Self::MigrationRequired => "explicit schema migration is required",
            Self::UnsupportedMajor => "schema major version is unsupported",
            Self::UnsupportedVersion => "schema version is outside the supported range",
            Self::Conditional => "compatibility requires additional validation",
            Self::Unknown => "compatibility cannot be determined",
        };

        formatter.write_str(text)
    }
}

// =============================================================================
// Compatibility decision object
// =============================================================================

/// Complete result of evaluating one schema version.
///
/// This type is immutable and contains no execution instructions.
///
/// The caller must interpret the result before deciding whether to decode,
/// migrate, degrade, reject or escalate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompatibilityDecision {
    /// High-level schema compatibility.
    compatibility: SchemaCompatibility,

    /// Deterministic reason for the result.
    reason: CompatibilityReason,

    /// Reader version.
    reader: SchemaVersion,

    /// Writer version.
    writer: SchemaVersion,
}

impl CompatibilityDecision {
    /// Creates a compatibility decision.
    #[must_use]
    pub const fn new(
        compatibility: SchemaCompatibility,
        reason: CompatibilityReason,
        reader: SchemaVersion,
        writer: SchemaVersion,
    ) -> Self {
        Self {
            compatibility,
            reason,
            reader,
            writer,
        }
    }

    /// Returns the high-level compatibility result.
    #[must_use]
    pub const fn compatibility(self) -> SchemaCompatibility {
        self.compatibility
    }

    /// Returns the detailed reason.
    #[must_use]
    pub const fn reason(self) -> CompatibilityReason {
        self.reason
    }

    /// Returns the reader version.
    #[must_use]
    pub const fn reader(self) -> SchemaVersion {
        self.reader
    }

    /// Returns the writer version.
    #[must_use]
    pub const fn writer(self) -> SchemaVersion {
        self.writer
    }

    /// Returns true when direct decoding is permitted by the decision.
    #[must_use]
    pub const fn allows_direct_decode(self) -> bool {
        self.compatibility.allows_direct_decode()
    }

    /// Returns true when migration is required.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        self.compatibility.requires_migration()
    }

    /// Returns true when additional validation is required before acceptance.
    #[must_use]
    pub const fn requires_additional_validation(self) -> bool {
        self.compatibility.requires_additional_validation()
    }

    /// Returns true when the schema is definitively incompatible.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        self.compatibility.is_incompatible()
    }
}

// =============================================================================
// Version comparison
// =============================================================================

/// Relationship between two schema versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionRelation {
    /// Versions are exactly equal.
    Equal,

    /// Writer is older than reader.
    Older,

    /// Writer is newer than reader.
    Newer,

    /// Versions belong to different major families.
    DifferentMajor,
}

impl VersionRelation {
    /// Returns true when the versions are exactly equal.
    #[must_use]
    pub const fn is_equal(self) -> bool {
        matches!(self, Self::Equal)
    }

    /// Returns true when the writer is older.
    #[must_use]
    pub const fn is_older(self) -> bool {
        matches!(self, Self::Older)
    }

    /// Returns true when the writer is newer.
    #[must_use]
    pub const fn is_newer(self) -> bool {
        matches!(self, Self::Newer)
    }

    /// Returns true when major versions differ.
    #[must_use]
    pub const fn is_different_major(self) -> bool {
        matches!(self, Self::DifferentMajor)
    }
}

/// Compares a serialized writer version with a reader version.
///
/// The returned relationship is from the perspective of the serialized
/// document:
///
/// - `Older` means writer < reader;
/// - `Newer` means writer > reader.
#[must_use]
pub const fn compare_versions(
    writer: SchemaVersion,
    reader: SchemaVersion,
) -> VersionRelation {
    if writer.major() != reader.major() {
        return VersionRelation::DifferentMajor;
    }

    if writer == reader {
        VersionRelation::Equal
    } else if writer < reader {
        VersionRelation::Older
    } else {
        VersionRelation::Newer
    }
}

// =============================================================================
// Compatibility evaluation
// =============================================================================

/// Evaluates schema compatibility using the default strict policy.
///
/// This is the safest entry point for persisted or security-sensitive data.
#[must_use]
pub const fn evaluate(
    writer: SchemaVersion,
) -> CompatibilityDecision {
    evaluate_with_policy(
        writer,
        READER_SCHEMA_VERSION,
        CompatibilityPolicy::Strict,
        false,
    )
}

/// Evaluates schema compatibility using an explicit policy.
///
/// `writer` is the schema version contained in the serialized document.
///
/// `reader` is the schema version understood by the consuming implementation.
///
/// `forward_extensible` must be obtained from the serialized schema metadata,
/// not guessed from the version number.
#[must_use]
pub const fn evaluate_with_policy(
    writer: SchemaVersion,
    reader: SchemaVersion,
    policy: CompatibilityPolicy,
    forward_extensible: bool,
) -> CompatibilityDecision {
    if writer.major() < MIN_SUPPORTED_SCHEMA_MAJOR
        || writer.major() > MAX_SUPPORTED_SCHEMA_MAJOR
    {
        return CompatibilityDecision::new(
            SchemaCompatibility::Incompatible,
            CompatibilityReason::UnsupportedMajor,
            reader,
            writer,
        );
    }

    if reader.major() < MIN_SUPPORTED_SCHEMA_MAJOR
        || reader.major() > MAX_SUPPORTED_SCHEMA_MAJOR
    {
        return CompatibilityDecision::new(
            SchemaCompatibility::Unknown,
            CompatibilityReason::UnsupportedVersion,
            reader,
            writer,
        );
    }

    if writer.major() != reader.major() {
        return CompatibilityDecision::new(
            SchemaCompatibility::Incompatible,
            CompatibilityReason::UnsupportedMajor,
            reader,
            writer,
        );
    }

    if writer == reader {
        return CompatibilityDecision::new(
            SchemaCompatibility::Compatible,
            CompatibilityReason::ExactMatch,
            reader,
            writer,
        );
    }

    match compare_versions(writer, reader) {
        VersionRelation::Older => {
            evaluate_older(writer, reader, policy)
        }

        VersionRelation::Newer => {
            evaluate_newer(
                writer,
                reader,
                policy,
                forward_extensible,
            )
        }

        VersionRelation::Equal => CompatibilityDecision::new(
            SchemaCompatibility::Compatible,
            CompatibilityReason::ExactMatch,
            reader,
            writer,
        ),

        VersionRelation::DifferentMajor => {
            CompatibilityDecision::new(
                SchemaCompatibility::Incompatible,
                CompatibilityReason::UnsupportedMajor,
                reader,
                writer,
            )
        }
    }
}

const fn evaluate_older(
    writer: SchemaVersion,
    reader: SchemaVersion,
    policy: CompatibilityPolicy,
) -> CompatibilityDecision {
    match policy {
        CompatibilityPolicy::Strict => {
            if writer.major() == reader.major()
                && writer.minor() == reader.minor()
            {
                CompatibilityDecision::new(
                    SchemaCompatibility::Compatible,
                    CompatibilityReason::OlderPatch,
                    reader,
                    writer,
                )
            } else {
                CompatibilityDecision::new(
                    SchemaCompatibility::CompatibleWithMigration,
                    CompatibilityReason::MigrationRequired,
                    reader,
                    writer,
                )
            }
        }

        CompatibilityPolicy::ForwardCompatible
        | CompatibilityPolicy::Degraded => {
            CompatibilityDecision::new(
                SchemaCompatibility::Compatible,
                if writer.minor() == reader.minor() {
                    CompatibilityReason::OlderPatch
                } else {
                    CompatibilityReason::OlderMinor
                },
                reader,
                writer,
            )
        }

        CompatibilityPolicy::MigrationRequired => {
            CompatibilityDecision::new(
                SchemaCompatibility::CompatibleWithMigration,
                CompatibilityReason::MigrationRequired,
                reader,
                writer,
            )
        }
    }
}

const fn evaluate_newer(
    writer: SchemaVersion,
    reader: SchemaVersion,
    policy: CompatibilityPolicy,
    forward_extensible: bool,
) -> CompatibilityDecision {
    if writer.minor() == reader.minor() {
        // A newer patch version is allowed only when the patch-level change
        // does not alter semantics. Patch compatibility is therefore direct.
        return match policy {
            CompatibilityPolicy::MigrationRequired => {
                CompatibilityDecision::new(
                    SchemaCompatibility::CompatibleWithMigration,
                    CompatibilityReason::MigrationRequired,
                    reader,
                    writer,
                )
            }

            _ => CompatibilityDecision::new(
                SchemaCompatibility::Compatible,
                CompatibilityReason::OlderPatch,
                reader,
                writer,
            ),
        };
    }

    match policy {
        CompatibilityPolicy::Strict => {
            CompatibilityDecision::new(
                SchemaCompatibility::Incompatible,
                CompatibilityReason::UnsupportedVersion,
                reader,
                writer,
            )
        }

        CompatibilityPolicy::ForwardCompatible => {
            if forward_extensible {
                CompatibilityDecision::new(
                    SchemaCompatibility::Compatible,
                    CompatibilityReason::NewerMinorForwardCompatible,
                    reader,
                    writer,
                )
            } else {
                CompatibilityDecision::new(
                    SchemaCompatibility::ConditionallyCompatible,
                    CompatibilityReason::Conditional,
                    reader,
                    writer,
                )
            }
        }

        CompatibilityPolicy::Degraded => {
            if forward_extensible {
                CompatibilityDecision::new(
                    SchemaCompatibility::CompatibleWithDegradation,
                    CompatibilityReason::NewerMinorDegraded,
                    reader,
                    writer,
                )
            } else {
                CompatibilityDecision::new(
                    SchemaCompatibility::ConditionallyCompatible,
                    CompatibilityReason::Conditional,
                    reader,
                    writer,
                )
            }
        }

        CompatibilityPolicy::MigrationRequired => {
            CompatibilityDecision::new(
                SchemaCompatibility::CompatibleWithMigration,
                CompatibilityReason::MigrationRequired,
                reader,
                writer,
            )
        }
    }
}

// =============================================================================
// Schema-envelope evaluation
// =============================================================================

/// Evaluates the complete schema envelope.
///
/// This is the preferred API for `decode.rs` because it evaluates both the
/// version and the schema's forward-extensibility declaration.
#[must_use]
pub const fn evaluate_envelope(
    envelope: SchemaEnvelope,
    reader: SchemaVersion,
    policy: CompatibilityPolicy,
) -> CompatibilityDecision {
    evaluate_with_policy(
        envelope.version(),
        reader,
        policy,
        envelope
            .flags()
            .contains(
                super::schema::SchemaFlags::FORWARD_EXTENSIBLE,
            ),
    )
}

/// Evaluates an envelope against the current reader schema using strict mode.
#[must_use]
pub const fn evaluate_current(
    envelope: SchemaEnvelope,
) -> CompatibilityDecision {
    evaluate_envelope(
        envelope,
        READER_SCHEMA_VERSION,
        CompatibilityPolicy::Strict,
    )
}

/// Returns whether the supplied envelope can be directly decoded under the
/// strict production policy.
#[must_use]
pub const fn is_directly_compatible(
    envelope: SchemaEnvelope,
) -> bool {
    evaluate_current(envelope).allows_direct_decode()
}

// =============================================================================
// Migration requirement
// =============================================================================

/// Describes whether an explicit migration is needed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MigrationRequirement {
    /// No migration is required.
    None,

    /// A schema migration is required before domain interpretation.
    Required,

    /// Migration is optional because the representation can be consumed
    /// directly under the selected policy.
    Optional,

    /// The schema cannot be migrated by this compatibility layer.
    Unsupported,
}

impl MigrationRequirement {
    /// Returns true when migration is mandatory.
    #[must_use]
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }

    /// Returns true when no migration is necessary.
    #[must_use]
    pub const fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
}

/// Determines migration requirements from a compatibility decision.
#[must_use]
pub const fn migration_requirement(
    decision: CompatibilityDecision,
) -> MigrationRequirement {
    match decision.compatibility() {
        SchemaCompatibility::Compatible => MigrationRequirement::None,

        SchemaCompatibility::CompatibleWithDegradation => {
            MigrationRequirement::Optional
        }

        SchemaCompatibility::CompatibleWithMigration => {
            MigrationRequirement::Required
        }

        SchemaCompatibility::ConditionallyCompatible
        | SchemaCompatibility::Unknown => {
            MigrationRequirement::Unsupported
        }

        SchemaCompatibility::Incompatible => {
            MigrationRequirement::Unsupported
        }
    }
}

// =============================================================================
// Migration target validation
// =============================================================================

/// Validates that a migration target is a supported reader target.
///
/// This does not execute a migration.
#[must_use]
pub const fn validate_migration_target(
    target: SchemaVersion,
) -> bool {
    target.major() >= MIN_SUPPORTED_SCHEMA_MAJOR
        && target.major() <= MAX_SUPPORTED_SCHEMA_MAJOR
}

/// Returns the current schema version used as the canonical migration target.
#[must_use]
pub const fn current_migration_target() -> SchemaVersion {
    READER_SCHEMA_VERSION
}

// =============================================================================
// Schema-version ordering helpers
// =============================================================================

/// Returns true if `candidate` is no newer than `reader` within the same major
/// schema family.
#[must_use]
pub const fn is_at_most(
    candidate: SchemaVersion,
    reader: SchemaVersion,
) -> bool {
    candidate.major() == reader.major() && candidate <= reader
}

/// Returns true if `candidate` is no older than `reader` within the same major
/// schema family.
#[must_use]
pub const fn is_at_least(
    candidate: SchemaVersion,
    reader: SchemaVersion,
) -> bool {
    candidate.major() == reader.major() && candidate >= reader
}

/// Returns true when two versions are within the same major family.
#[must_use]
pub const fn same_major(
    left: SchemaVersion,
    right: SchemaVersion,
) -> bool {
    left.major() == right.major()
}

// =============================================================================
// Schema identity helpers
// =============================================================================

/// Returns the schema identifier represented by an envelope.
#[must_use]
pub const fn schema_id(envelope: SchemaEnvelope) -> SchemaId {
    envelope.schema()
}

/// Returns the schema version represented by a schema identifier.
#[must_use]
pub const fn version_of(schema: SchemaId) -> SchemaVersion {
    schema.version()
}

// =============================================================================
// Version errors
// =============================================================================

/// Error returned when a caller attempts an invalid version operation.
///
/// This is intentionally small and independent of codec-specific errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VersionError {
    /// The supplied writer major version is unsupported.
    UnsupportedMajor {
        /// Supplied major version.
        major: u16,
    },

    /// The supplied reader version is outside the supported range.
    UnsupportedReaderVersion {
        /// Reader major version.
        major: u16,
    },

    /// The requested migration target is not supported.
    UnsupportedMigrationTarget {
        /// Target major version.
        major: u16,
    },

    /// Compatibility is conditional and cannot be safely resolved by this
    /// version layer alone.
    CompatibilityUnknown,
}

impl fmt::Display for VersionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMajor { major } => {
                write!(
                    formatter,
                    "unsupported resilience serialization schema major version {major}"
                )
            }

            Self::UnsupportedReaderVersion { major } => {
                write!(
                    formatter,
                    "unsupported resilience serialization reader major version {major}"
                )
            }

            Self::UnsupportedMigrationTarget { major } => {
                write!(
                    formatter,
                    "unsupported resilience serialization migration target major version {major}"
                )
            }

            Self::CompatibilityUnknown => {
                formatter.write_str(
                    "resilience serialization compatibility could not be determined",
                )
            }
        }
    }
}

impl std::error::Error for VersionError {}

// =============================================================================
// Checked compatibility API
// =============================================================================

/// Performs strict compatibility validation and returns an error when the
/// schema cannot be safely consumed directly.
///
/// This is intended for `decode.rs`.
pub fn require_direct_compatibility(
    envelope: SchemaEnvelope,
) -> Result<CompatibilityDecision, VersionError> {
    let writer = envelope.version();
    let reader = READER_SCHEMA_VERSION;

    if writer.major() < MIN_SUPPORTED_SCHEMA_MAJOR
        || writer.major() > MAX_SUPPORTED_SCHEMA_MAJOR
    {
        return Err(VersionError::UnsupportedMajor {
            major: writer.major(),
        });
    }

    if reader.major() < MIN_SUPPORTED_SCHEMA_MAJOR
        || reader.major() > MAX_SUPPORTED_SCHEMA_MAJOR
    {
        return Err(VersionError::UnsupportedReaderVersion {
            major: reader.major(),
        });
    }

    let decision = evaluate_current(envelope);

    if decision.allows_direct_decode() {
        Ok(decision)
    } else {
        match decision.compatibility() {
            SchemaCompatibility::ConditionallyCompatible
            | SchemaCompatibility::Unknown => {
                Err(VersionError::CompatibilityUnknown)
            }

            _ => Err(VersionError::UnsupportedMajor {
                major: writer.major(),
            }),
        }
    }
}

/// Validates a migration target before a migration implementation is invoked.
///
/// This function deliberately does not perform migration itself.
pub fn require_supported_migration_target(
    target: SchemaVersion,
) -> Result<SchemaVersion, VersionError> {
    if !validate_migration_target(target) {
        return Err(
            VersionError::UnsupportedMigrationTarget {
                major: target.major(),
            },
        );
    }

    Ok(target)
}

// =============================================================================
// Human-readable policy information
// =============================================================================

/// Returns a stable textual description of the current compatibility policy.
///
/// This is documentation/diagnostic metadata and must not be used as a parser
/// or machine protocol.
#[must_use]
pub const fn policy_name(
    policy: CompatibilityPolicy,
) -> &'static str {
    match policy {
        CompatibilityPolicy::Strict => "strict",
        CompatibilityPolicy::ForwardCompatible => "forward_compatible",
        CompatibilityPolicy::Degraded => "degraded",
        CompatibilityPolicy::MigrationRequired => "migration_required",
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::resilience::serialization::schema::{
        DocumentKind,
        EncodingKind,
        SchemaFeatures,
        SchemaFlags,
    };

    fn version(
        major: u16,
        minor: u16,
        patch: u16,
    ) -> SchemaVersion {
        SchemaVersion::new(major, minor, patch)
    }

    fn envelope(
        version: SchemaVersion,
        flags: SchemaFlags,
    ) -> SchemaEnvelope {
        SchemaEnvelope::new(
            super::super::schema::SchemaHeader::new(
                SchemaId::new(
                    DocumentKind::ExecutionState,
                    version,
                ),
                SchemaFeatures::NONE,
                flags,
            ),
            EncodingKind::Json,
        )
    }

    #[test]
    fn current_version_is_directly_compatible() {
        let document = envelope(
            READER_SCHEMA_VERSION,
            SchemaFlags::SELF_DESCRIBING
                .union(SchemaFlags::DETERMINISTIC),
        );

        let decision = evaluate_current(document);

        assert_eq!(
            decision.compatibility(),
            SchemaCompatibility::Compatible
        );
        assert_eq!(
            decision.reason(),
            CompatibilityReason::ExactMatch
        );
        assert!(decision.allows_direct_decode());
        assert!(!decision.requires_migration());
    }

    #[test]
    fn older_patch_is_compatible() {
        let writer = version(1, 0, 0);
        let reader = version(1, 0, 1);

        let decision = evaluate_with_policy(
            writer,
            reader,
            CompatibilityPolicy::Strict,
            false,
        );

        assert_eq!(
            decision.compatibility(),
            SchemaCompatibility::Compatible
        );
        assert_eq!(
            decision.reason(),
            CompatibilityReason::OlderPatch
        );
    }

    #[test]
    fn older_minor_requires_migration_in_strict_mode() {
        let writer = version(1, 0, 0);
        let reader = version(1, 1, 0);

        let decision = evaluate_with_policy(
            writer,
            reader,
            CompatibilityPolicy::Strict,
            false,
        );

        assert_eq!(
            decision.compatibility(),
            SchemaCompatibility::CompatibleWithMigration
        );
        assert!(decision.requires_migration());
    }

    #[test]
    fn older_minor_is_compatible_in_forward_mode() {
        let writer = version(1, 0, 0);
        let reader = version(1, 1, 0);

        let decision = evaluate_with_policy(
            writer,
            reader,
            CompatibilityPolicy::ForwardCompatible,
            false,
        );

        assert_eq!(
            decision.compatibility(),
            SchemaCompatibility::Compatible
        );
        assert_eq!(
            decision.reason(),
            CompatibilityReason::OlderMinor
        );
    }

    #[test]
    fn newer_minor_requires_forward_extension() {
        let writer = version(1, 1, 0);
        let reader = version(1, 0, 0);

        let rejected = evaluate_with_policy(
            writer,
            reader,
            CompatibilityPolicy::ForwardCompatible,
            false,
        );

        assert_eq!(
            rejected.compatibility(),
            SchemaCompatibility::ConditionallyCompatible
        );

        let accepted = evaluate_with_policy(
            writer,
            reader,
            CompatibilityPolicy::ForwardCompatible,
            true,
        );

        assert_eq!(
            accepted.compatibility(),
            SchemaCompatibility::Compatible
        );
        assert_eq!(
            accepted.reason(),
            CompatibilityReason::NewerMinorForwardCompatible
        );
    }

    #[test]
    fn newer_minor_can_be_degraded() {
        let writer = version(1, 2, 0);
        let reader = version(1, 0, 0);

        let decision = evaluate_with_policy(
            writer,
            reader,
            CompatibilityPolicy::Degraded,
            true,
        );

        assert_eq!(
            decision.compatibility(),
            SchemaCompatibility::CompatibleWithDegradation
        );
        assert!(decision.requires_additional_validation() == false);
    }

    #[test]
    fn different_major_is_incompatible() {
        let writer = version(2, 0, 0);
        let reader = version(1, 0, 0);

        let decision = evaluate_with_policy(
            writer,
            reader,
            CompatibilityPolicy::ForwardCompatible,
            true,
        );

        assert_eq!(
            decision.compatibility(),
            SchemaCompatibility::Incompatible
        );
        assert!(decision.is_incompatible());
    }

    #[test]
    fn migration_required_policy_is_explicit() {
        let writer = version(1, 0, 0);
        let reader = version(1, 1, 0);

        let decision = evaluate_with_policy(
            writer,
            reader,
            CompatibilityPolicy::MigrationRequired,
            true,
        );

        assert_eq!(
            decision.compatibility(),
            SchemaCompatibility::CompatibleWithMigration
        );
        assert_eq!(
            migration_requirement(decision),
            MigrationRequirement::Required
        );
    }

    #[test]
    fn comparison_is_deterministic() {
        let writer = version(1, 3, 0);
        let reader = version(1, 2, 0);

        assert_eq!(
            compare_versions(writer, reader),
            VersionRelation::Newer
        );

        assert_eq!(
            compare_versions(reader, writer),
            VersionRelation::Older
        );
    }

    #[test]
    fn migration_target_is_checked() {
        assert!(
            require_supported_migration_target(
                READER_SCHEMA_VERSION
            )
            .is_ok()
        );

        assert!(
            require_supported_migration_target(
                version(2, 0, 0)
            )
            .is_err()
        );
    }

    #[test]
    fn schema_id_preserves_document_identity() {
        let document = envelope(
            READER_SCHEMA_VERSION,
            SchemaFlags::SELF_DESCRIBING,
        );

        let id = schema_id(document);

        assert_eq!(
            id.kind(),
            DocumentKind::ExecutionState
        );
        assert_eq!(
            version_of(id),
            READER_SCHEMA_VERSION
        );
    }
}