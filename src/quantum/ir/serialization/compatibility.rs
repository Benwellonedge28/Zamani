//! Serialization compatibility policy for the Zamani Quantum IR.
//!
//! # Purpose
//!
//! This module answers one question:
//!
//! > Can a serialized Zamani Quantum IR artifact be interpreted safely by
//! > this serializer/decoder, and if not, what compatibility action is
//! > required?
//!
//! Compatibility is deliberately separated from:
//!
//! - canonical encoding/decoding,
//! - semantic IR validation,
//! - IR construction,
//! - hardware capabilities,
//! - qubit topology,
//! - backend lowering.
//!
//! The compatibility layer therefore contains no hardware-specific limits,
//! no qubit-count limits, no gate-count limits, and no execution logic.
//!
//! # Architectural invariants
//!
//! 1. Semantic IR version and serialization format version are different
//!    concepts.
//! 2. This module does not define either version's meaning.
//! 3. This module only compares versions and applies compatibility policy.
//! 4. Unknown future representations must never be silently interpreted as
//!    known representations.
//! 5. Unknown extension data must be preserved when the surrounding
//!    serialization layer supports preservation.
//! 6. Lossy compatibility must always be explicit.
//! 7. Compatibility decisions are deterministic and side-effect free.
//! 8. Compatibility decisions never depend on machine size.
//! 9. Compatibility decisions never depend on a particular quantum backend.
//! 10. No `unsafe` code is used.
//!
//! # Integration
//!
//! The serialization layer should perform compatibility checking before
//! interpreting version-dependent payload data:
//!
//! ```text
//! serialized bytes
//!       |
//!       v
//! serialization envelope
//!       |
//!       v
//! compatibility check
//!       |
//!       +---- incompatible ------> reject
//!       |
//!       +---- migration required -> migration layer
//!       |
//!       +---- compatible ---------> decoder
//!       |
//!       v
//! canonical IR
//!       |
//!       v
//! semantic validation
//! ```
//!
//! The canonical serializer remains the owner of the wire-format version.
//! The IR identity/version module remains the owner of the semantic IR
//! version. This module must not duplicate those constants.

use core::cmp::Ordering;
use core::fmt;

/// A generic semantic version used by the serialization compatibility layer.
///
/// This type intentionally does not replace Zamani's canonical `IrVersion`.
/// It exists so compatibility policy can remain independent of the concrete
/// version implementation used by another IR module.
///
/// The serializer integration layer should convert its authoritative version
/// type into this representation when invoking compatibility checks.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CompatibilityVersion {
    /// Breaking-change generation.
    pub major: u16,

    /// Backwards-compatible feature generation.
    pub minor: u16,

    /// Patch/fix generation.
    pub patch: u16,
}

impl CompatibilityVersion {
    /// Creates a compatibility version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns `true` when the version is exactly equal to another version.
    pub const fn is_exact(self, other: Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Returns `true` when this version has the same major version as `other`.
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns whether this version is newer than `other`.
    pub const fn is_newer_than(self, other: Self) -> bool {
        matches!(self.cmp(&other), Ordering::Greater)
    }

    /// Returns whether this version is older than `other`.
    pub const fn is_older_than(self, other: Self) -> bool {
        matches!(self.cmp(&other), Ordering::Less)
    }
}

impl Default for CompatibilityVersion {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl fmt::Display for CompatibilityVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

/// The kind of version represented by a compatibility descriptor.
///
/// Semantic IR version and serialization format version must not be confused.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum VersionKind {
    /// Version describing the semantic Zamani Quantum IR.
    Ir,

    /// Version describing the serialized wire/schema representation.
    SerializationFormat,
}

impl fmt::Display for VersionKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ir => formatter.write_str("IR"),
            Self::SerializationFormat => formatter.write_str("serialization format"),
        }
    }
}

/// Policy for handling unknown fields.
///
/// Unknown fields are important for forward compatibility. They must not be
/// silently discarded when doing so could destroy information required by a
/// newer producer.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnknownFieldPolicy {
    /// Preserve unknown fields exactly where the surrounding representation
    /// permits preservation.
    Preserve,

    /// Reject an artifact containing unknown fields.
    Reject,

    /// Explicitly discard unknown fields.
    ///
    /// This is intentionally not the default. Callers should use this only
    /// when they knowingly accept a lossy conversion.
    Discard,
}

impl Default for UnknownFieldPolicy {
    fn default() -> Self {
        Self::Preserve
    }
}

/// Policy for handling unknown extensions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnknownExtensionPolicy {
    /// Preserve extensions without interpreting their semantics.
    Preserve,

    /// Reject the artifact because the extension cannot be interpreted.
    Reject,

    /// Drop the extension explicitly.
    ///
    /// This is a lossy operation and must never be implicit.
    Discard,
}

impl Default for UnknownExtensionPolicy {
    fn default() -> Self {
        Self::Preserve
    }
}

/// Policy for a compatibility operation that might lose semantic information.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LossPolicy {
    /// Loss of information is forbidden.
    Forbid,

    /// Loss is permitted only when the migration explicitly declares the
    /// affected information as non-semantic.
    AllowDeclared,

    /// Loss is permitted.
    ///
    /// This mode should normally only be exposed by tools explicitly intended
    /// for lossy export.
    Allow,
}

impl Default for LossPolicy {
    fn default() -> Self {
        Self::Forbid
    }
}

/// Compatibility options.
///
/// These options describe what the caller is willing to accept. They do not
/// alter the semantic meaning of the IR.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CompatibilityOptions {
    /// How unknown serialized fields are handled.
    pub unknown_fields: UnknownFieldPolicy,

    /// How unknown extensions are handled.
    pub unknown_extensions: UnknownExtensionPolicy,

    /// Whether an explicitly declared lossy migration is allowed.
    pub loss_policy: LossPolicy,

    /// Whether migration may be required.
    ///
    /// If false, any migration-required artifact is rejected.
    pub allow_migration: bool,
}

impl Default for CompatibilityOptions {
    fn default() -> Self {
        Self {
            unknown_fields: UnknownFieldPolicy::Preserve,
            unknown_extensions: UnknownExtensionPolicy::Preserve,
            loss_policy: LossPolicy::Forbid,
            allow_migration: true,
        }
    }
}

/// The broad compatibility classification.
///
/// This classification is deliberately more expressive than a boolean
/// compatible/incompatible result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CompatibilityClass {
    /// Producer and consumer understand exactly the same representation.
    Exact,

    /// The artifact can be read without a migration.
    BackwardCompatible,

    /// The artifact can be read only after an explicit migration.
    MigrationRequired,

    /// The artifact cannot safely be interpreted by this compatibility
    /// contract.
    Incompatible,
}

impl CompatibilityClass {
    /// Returns whether the class is directly readable without migration.
    pub const fn is_readable_without_migration(self) -> bool {
        matches!(self, Self::Exact | Self::BackwardCompatible)
    }

    /// Returns whether the class requires migration.
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::MigrationRequired)
    }

    /// Returns whether the class is incompatible.
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }
}

impl fmt::Display for CompatibilityClass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => formatter.write_str("exact"),
            Self::BackwardCompatible => formatter.write_str("backward-compatible"),
            Self::MigrationRequired => formatter.write_str("migration-required"),
            Self::Incompatible => formatter.write_str("incompatible"),
        }
    }
}

/// A specific reason for a compatibility decision.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CompatibilityReason {
    /// Versions are identical.
    ExactVersion,

    /// Producer is older within a compatible major generation.
    OlderCompatibleVersion,

    /// Producer uses a newer patch version within the same compatible minor
    /// generation.
    NewerCompatiblePatch,

    /// A schema/semantic migration is required.
    ExplicitMigration,

    /// Major versions differ.
    MajorVersionMismatch,

    /// A future minor version is not known to the consumer.
    FutureMinorVersion,

    /// A future patch version is not known to the consumer.
    FuturePatchVersion,

    /// Required fields are missing.
    MissingRequiredField,

    /// An unknown field was encountered under a rejecting policy.
    UnknownField,

    /// An unknown extension was encountered under a rejecting policy.
    UnknownExtension,

    /// A lossy migration was requested but forbidden.
    LossNotAllowed,

    /// A migration is required but the caller disallowed migrations.
    MigrationNotAllowed,
}

impl fmt::Display for CompatibilityReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::ExactVersion => "exact version",
            Self::OlderCompatibleVersion => "older compatible version",
            Self::NewerCompatiblePatch => "newer compatible patch version",
            Self::ExplicitMigration => "explicit migration required",
            Self::MajorVersionMismatch => "major version mismatch",
            Self::FutureMinorVersion => "future minor version",
            Self::FuturePatchVersion => "future patch version",
            Self::MissingRequiredField => "missing required field",
            Self::UnknownField => "unknown field",
            Self::UnknownExtension => "unknown extension",
            Self::LossNotAllowed => "loss is not allowed",
            Self::MigrationNotAllowed => "migration is not allowed",
        };

        formatter.write_str(text)
    }
}

/// A complete compatibility decision.
///
/// The decision is immutable and contains enough information for the caller
/// to make a deterministic next-step decision without repeating compatibility
/// logic.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CompatibilityDecision {
    /// Overall compatibility class.
    pub class: CompatibilityClass,

    /// Primary reason for the decision.
    pub reason: CompatibilityReason,

    /// Producer version.
    pub producer: CompatibilityVersion,

    /// Consumer version.
    pub consumer: CompatibilityVersion,

    /// Version kind being compared.
    pub kind: VersionKind,
}

impl CompatibilityDecision {
    /// Constructs a compatibility decision.
    pub const fn new(
        class: CompatibilityClass,
        reason: CompatibilityReason,
        producer: CompatibilityVersion,
        consumer: CompatibilityVersion,
        kind: VersionKind,
    ) -> Self {
        Self {
            class,
            reason,
            producer,
            consumer,
            kind,
        }
    }

    /// Returns whether the artifact can be decoded directly.
    pub const fn is_directly_readable(self) -> bool {
        self.class.is_readable_without_migration()
    }

    /// Returns whether migration is required.
    pub const fn requires_migration(self) -> bool {
        self.class.requires_migration()
    }

    /// Returns whether decoding must be rejected.
    pub const fn must_reject(self) -> bool {
        self.class.is_incompatible()
    }
}

/// A migration direction.
///
/// Migrations are directional. An old representation can often be upgraded,
/// while arbitrary downgrade is not necessarily lossless.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MigrationDirection {
    /// Upgrade an older representation to a newer supported representation.
    Upgrade,

    /// Downgrade a newer representation to an older representation.
    ///
    /// Downgrades must be explicitly declared and are potentially lossy.
    Downgrade,
}

impl fmt::Display for MigrationDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Upgrade => formatter.write_str("upgrade"),
            Self::Downgrade => formatter.write_str("downgrade"),
        }
    }
}

/// A migration requirement.
///
/// This is descriptive metadata, not the migration implementation itself.
/// Keeping the migration executor separate prevents compatibility checking
/// from silently mutating IR.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MigrationRequirement {
    /// Version kind that requires migration.
    pub kind: VersionKind,

    /// Source representation.
    pub from: CompatibilityVersion,

    /// Target representation.
    pub to: CompatibilityVersion,

    /// Direction of migration.
    pub direction: MigrationDirection,

    /// Whether the migration is potentially lossy.
    pub potentially_lossy: bool,
}

impl MigrationRequirement {
    /// Creates an upgrade requirement.
    pub const fn upgrade(
        kind: VersionKind,
        from: CompatibilityVersion,
        to: CompatibilityVersion,
        potentially_lossy: bool,
    ) -> Self {
        Self {
            kind,
            from,
            to,
            direction: MigrationDirection::Upgrade,
            potentially_lossy,
        }
    }

    /// Creates a downgrade requirement.
    pub const fn downgrade(
        kind: VersionKind,
        from: CompatibilityVersion,
        to: CompatibilityVersion,
        potentially_lossy: bool,
    ) -> Self {
        Self {
            kind,
            from,
            to,
            direction: MigrationDirection::Downgrade,
            potentially_lossy,
        }
    }
}

/// A complete compatibility assessment.
///
/// Unlike `CompatibilityDecision`, this also records unknown-field and
/// extension policy outcomes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CompatibilityAssessment {
    /// Version compatibility decision.
    pub version: CompatibilityDecision,

    /// Optional migration requirement.
    pub migration: Option<MigrationRequirement>,

    /// Whether unknown fields can be retained.
    pub preserves_unknown_fields: bool,

    /// Whether unknown extensions can be retained.
    pub preserves_unknown_extensions: bool,

    /// Whether the resulting operation can be lossless.
    pub lossless: bool,
}

impl CompatibilityAssessment {
    /// Returns true when the assessment permits direct decoding.
    pub const fn can_decode_directly(self) -> bool {
        self.version.is_directly_readable()
            && self.migration.is_none()
            && self.lossless
    }

    /// Returns true when an explicit migration step is required.
    pub const fn requires_migration(self) -> bool {
        self.migration.is_some()
    }

    /// Returns true when the assessment is safe under its declared policies.
    pub const fn is_acceptable(self) -> bool {
        !self.version.must_reject()
    }
}

/// Errors emitted by compatibility policy.
///
/// These errors describe compatibility failures. They intentionally do not
/// contain backend or execution errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CompatibilityError {
    /// The semantic IR major generation is incompatible.
    IrMajorVersionMismatch {
        producer: CompatibilityVersion,
        consumer: CompatibilityVersion,
    },

    /// The serialization format major generation is incompatible.
    SerializationMajorVersionMismatch {
        producer: CompatibilityVersion,
        consumer: CompatibilityVersion,
    },

    /// The producer uses a future minor representation.
    FutureMinorVersion {
        kind: VersionKind,
        producer: CompatibilityVersion,
        consumer: CompatibilityVersion,
    },

    /// The producer uses a future patch representation that cannot be safely
    /// interpreted under the active compatibility contract.
    FuturePatchVersion {
        kind: VersionKind,
        producer: CompatibilityVersion,
        consumer: CompatibilityVersion,
    },

    /// An explicit migration is required but the caller disallowed it.
    MigrationNotAllowed(MigrationRequirement),

    /// The required migration could be lossy but the caller forbids loss.
    LossNotAllowed(MigrationRequirement),

    /// An unknown serialized field was encountered and preservation/rejection
    /// policy prevents safe processing.
    UnknownField,

    /// An unknown extension was encountered and preservation/rejection policy
    /// prevents safe processing.
    UnknownExtension,

    /// A required field is absent.
    MissingRequiredField,

    /// The compatibility contract was asked to compare an unsupported version
    /// kind.
    UnsupportedVersionKind,
}

impl fmt::Display for CompatibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IrMajorVersionMismatch { producer, consumer } => write!(
                formatter,
                "incompatible IR major versions: producer={}, consumer={}",
                producer, consumer
            ),

            Self::SerializationMajorVersionMismatch { producer, consumer } => write!(
                formatter,
                "incompatible serialization format major versions: \
                 producer={}, consumer={}",
                producer, consumer
            ),

            Self::FutureMinorVersion {
                kind,
                producer,
                consumer,
            } => write!(
                formatter,
                "future {} minor version is not safely readable: \
                 producer={}, consumer={}",
                kind, producer, consumer
            ),

            Self::FuturePatchVersion {
                kind,
                producer,
                consumer,
            } => write!(
                formatter,
                "future {} patch version is not safely readable: \
                 producer={}, consumer={}",
                kind, producer, consumer
            ),

            Self::MigrationNotAllowed(requirement) => write!(
                formatter,
                "{} migration from {} to {} is required but migration \
                 is disabled",
                requirement.direction,
                requirement.from,
                requirement.to
            ),

            Self::LossNotAllowed(requirement) => write!(
                formatter,
                "{} migration from {} to {} may be lossy and loss is \
                 forbidden",
                requirement.direction,
                requirement.from,
                requirement.to
            ),

            Self::UnknownField => {
                formatter.write_str("unknown serialized field cannot be handled safely")
            }

            Self::UnknownExtension => {
                formatter.write_str("unknown serialization extension cannot be handled safely")
            }

            Self::MissingRequiredField => {
                formatter.write_str("required serialized field is missing")
            }

            Self::UnsupportedVersionKind => {
                formatter.write_str("unsupported version kind")
            }
        }
    }
}

impl std::error::Error for CompatibilityError {}

/// Compatibility rules.
///
/// This type is deliberately immutable. Construct one at the serialization
/// boundary and pass it to compatibility checks.
///
/// The rules are policy, not mutable global state.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CompatibilityRules {
    /// Version at which the consumer understands the representation.
    pub supported: CompatibilityVersion,

    /// Compatibility options.
    pub options: CompatibilityOptions,

    /// Version kind governed by these rules.
    pub kind: VersionKind,
}

impl CompatibilityRules {
    /// Creates a compatibility rule set.
    pub const fn new(
        kind: VersionKind,
        supported: CompatibilityVersion,
        options: CompatibilityOptions,
    ) -> Self {
        Self {
            supported,
            options,
            kind,
        }
    }

    /// Checks producer/consumer version compatibility.
    ///
    /// Policy:
    ///
    /// * identical versions are exact;
    /// * older versions in the same major generation are candidates for
    ///   backward-compatible reading;
    /// * a newer major version is incompatible;
    /// * a newer minor version is not automatically interpreted;
    /// * a newer patch version is accepted only when the compatibility
    ///   contract explicitly permits patch-level forward compatibility;
    ///
    /// This conservative behavior is intentional. A serializer must never
    /// infer that an unknown future schema is safe merely because its major
    /// number happens to match.
    pub const fn check_version(
        &self,
        producer: CompatibilityVersion,
    ) -> CompatibilityDecision {
        if producer.is_exact(self.supported) {
            return CompatibilityDecision::new(
                CompatibilityClass::Exact,
                CompatibilityReason::ExactVersion,
                producer,
                self.supported,
                self.kind,
            );
        }

        if producer.major != self.supported.major {
            return CompatibilityDecision::new(
                CompatibilityClass::Incompatible,
                CompatibilityReason::MajorVersionMismatch,
                producer,
                self.supported,
                self.kind,
            );
        }

        if producer.minor < self.supported.minor {
            return CompatibilityDecision::new(
                CompatibilityClass::BackwardCompatible,
                CompatibilityReason::OlderCompatibleVersion,
                producer,
                self.supported,
                self.kind,
            );
        }

        if producer.minor > self.supported.minor {
            return CompatibilityDecision::new(
                CompatibilityClass::Incompatible,
                CompatibilityReason::FutureMinorVersion,
                producer,
                self.supported,
                self.kind,
            );
        }

        // At this point major and minor are identical.
        //
        // A future patch may be compatible in a semver-style schema, but this
        // module deliberately uses a conservative default. The caller can
        // explicitly declare the patch as readable through
        // `check_patch_compatible`.
        CompatibilityDecision::new(
            CompatibilityClass::Incompatible,
            CompatibilityReason::FuturePatchVersion,
            producer,
            self.supported,
            self.kind,
        )
    }

    /// Checks a patch-level compatibility relationship explicitly.
    ///
    /// This function is intentionally opt-in because patch versions can still
    /// introduce serialization behavior that a decoder does not understand.
    pub const fn check_patch_compatible(
        &self,
        producer: CompatibilityVersion,
    ) -> CompatibilityDecision {
        if producer.is_exact(self.supported) {
            return CompatibilityDecision::new(
                CompatibilityClass::Exact,
                CompatibilityReason::ExactVersion,
                producer,
                self.supported,
                self.kind,
            );
        }

        if producer.major != self.supported.major {
            return CompatibilityDecision::new(
                CompatibilityClass::Incompatible,
                CompatibilityReason::MajorVersionMismatch,
                producer,
                self.supported,
                self.kind,
            );
        }

        if producer.minor > self.supported.minor {
            return CompatibilityDecision::new(
                CompatibilityClass::Incompatible,
                CompatibilityReason::FutureMinorVersion,
                producer,
                self.supported,
                self.kind,
            );
        }

        if producer.minor < self.supported.minor {
            return CompatibilityDecision::new(
                CompatibilityClass::BackwardCompatible,
                CompatibilityReason::OlderCompatibleVersion,
                producer,
                self.supported,
                self.kind,
            );
        }

        // Same major/minor; patch differs.
        if producer.patch <= self.supported.patch {
            CompatibilityDecision::new(
                CompatibilityClass::BackwardCompatible,
                CompatibilityReason::OlderCompatibleVersion,
                producer,
                self.supported,
                self.kind,
            )
        } else {
            CompatibilityDecision::new(
                CompatibilityClass::BackwardCompatible,
                CompatibilityReason::NewerCompatiblePatch,
                producer,
                self.supported,
                self.kind,
            )
        }
    }

    /// Checks whether an unknown field can be handled according to policy.
    pub const fn check_unknown_field(
        &self,
    ) -> Result<bool, CompatibilityError> {
        match self.options.unknown_fields {
            UnknownFieldPolicy::Preserve => Ok(true),
            UnknownFieldPolicy::Reject => Err(CompatibilityError::UnknownField),
            UnknownFieldPolicy::Discard => Ok(false),
        }
    }

    /// Checks whether an unknown extension can be handled according to policy.
    pub const fn check_unknown_extension(
        &self,
    ) -> Result<bool, CompatibilityError> {
        match self.options.unknown_extensions {
            UnknownExtensionPolicy::Preserve => Ok(true),
            UnknownExtensionPolicy::Reject => Err(CompatibilityError::UnknownExtension),
            UnknownExtensionPolicy::Discard => Ok(false),
        }
    }

    /// Checks whether a migration is permitted.
    pub const fn check_migration(
        &self,
        requirement: MigrationRequirement,
    ) -> Result<(), CompatibilityError> {
        if !self.options.allow_migration {
            return Err(CompatibilityError::MigrationNotAllowed(requirement));
        }

        if requirement.potentially_lossy {
            match self.options.loss_policy {
                LossPolicy::Forbid => {
                    return Err(CompatibilityError::LossNotAllowed(requirement));
                }
                LossPolicy::AllowDeclared | LossPolicy::Allow => {}
            }
        }

        Ok(())
    }
}

/// Performs a compatibility check for a version.
///
/// This is the preferred simple API when the caller has already established
/// the authoritative producer and consumer versions.
pub const fn check_version(
    kind: VersionKind,
    producer: CompatibilityVersion,
    consumer: CompatibilityVersion,
) -> CompatibilityDecision {
    CompatibilityRules::new(
        kind,
        consumer,
        CompatibilityOptions::default(),
    )
    .check_version(producer)
}

/// Performs a conservative IR-version compatibility check.
///
/// The caller should supply the authoritative version from
/// `quantum::ir::identity`.
pub const fn check_ir_version(
    producer: CompatibilityVersion,
    consumer: CompatibilityVersion,
) -> CompatibilityDecision {
    check_version(VersionKind::Ir, producer, consumer)
}

/// Performs a conservative serialization-format compatibility check.
pub const fn check_serialization_version(
    producer: CompatibilityVersion,
    consumer: CompatibilityVersion,
) -> CompatibilityDecision {
    check_version(
        VersionKind::SerializationFormat,
        producer,
        consumer,
    )
}

/// Creates an upgrade migration requirement.
///
/// The actual transformation belongs in the migration implementation, not
/// in this compatibility module.
pub const fn require_upgrade(
    kind: VersionKind,
    from: CompatibilityVersion,
    to: CompatibilityVersion,
    potentially_lossy: bool,
) -> MigrationRequirement {
    MigrationRequirement::upgrade(kind, from, to, potentially_lossy)
}

/// Creates a downgrade migration requirement.
///
/// Downgrades should normally be considered potentially lossy unless the
/// migration contract explicitly proves otherwise.
pub const fn require_downgrade(
    kind: VersionKind,
    from: CompatibilityVersion,
    to: CompatibilityVersion,
    potentially_lossy: bool,
) -> MigrationRequirement {
    MigrationRequirement::downgrade(kind, from, to, potentially_lossy)
}

/// Performs a complete compatibility assessment.
///
/// This function is useful at the serialization boundary because it combines
/// version compatibility with unknown-field/extension policy.
pub const fn assess(
    rules: &CompatibilityRules,
    producer: CompatibilityVersion,
    migration: Option<MigrationRequirement>,
) -> Result<CompatibilityAssessment, CompatibilityError> {
    let decision = rules.check_version(producer);

    if decision.class.is_incompatible() && migration.is_none() {
        return Err(match decision.reason {
            CompatibilityReason::MajorVersionMismatch => match rules.kind {
                VersionKind::Ir => CompatibilityError::IrMajorVersionMismatch {
                    producer,
                    consumer: rules.supported,
                },
                VersionKind::SerializationFormat => {
                    CompatibilityError::SerializationMajorVersionMismatch {
                        producer,
                        consumer: rules.supported,
                    }
                }
            },

            CompatibilityReason::FutureMinorVersion => {
                CompatibilityError::FutureMinorVersion {
                    kind: rules.kind,
                    producer,
                    consumer: rules.supported,
                }
            }

            CompatibilityReason::FuturePatchVersion => {
                CompatibilityError::FuturePatchVersion {
                    kind: rules.kind,
                    producer,
                    consumer: rules.supported,
                }
            }

            _ => CompatibilityError::UnsupportedVersionKind,
        });
    }

    if let Some(requirement) = migration {
        rules.check_migration(requirement)?;
    }

    let preserves_unknown_fields =
        matches!(rules.options.unknown_fields, UnknownFieldPolicy::Preserve);

    let preserves_unknown_extensions =
        matches!(
            rules.options.unknown_extensions,
            UnknownExtensionPolicy::Preserve
        );

    let lossless = !migration
        .map(|requirement| requirement.potentially_lossy)
        .unwrap_or(false);

    Ok(CompatibilityAssessment {
        version: decision,
        migration,
        preserves_unknown_fields,
        preserves_unknown_extensions,
        lossless,
    })
}

/// Returns whether a version relationship can be read without migration.
///
/// This intentionally uses the conservative compatibility policy.
pub const fn is_backward_compatible(
    kind: VersionKind,
    producer: CompatibilityVersion,
    consumer: CompatibilityVersion,
) -> bool {
    check_version(kind, producer, consumer)
        .is_directly_readable()
}

/// Returns whether the versions are exactly identical.
pub const fn is_exact_version(
    producer: CompatibilityVersion,
    consumer: CompatibilityVersion,
) -> bool {
    producer.is_exact(consumer)
}

/// Returns whether a version has a compatible major generation.
pub const fn has_compatible_major(
    producer: CompatibilityVersion,
    consumer: CompatibilityVersion,
) -> bool {
    producer.same_major(consumer)
}

#[cfg(test)]
mod tests {
    use super::*;

    const V1_0_0: CompatibilityVersion =
        CompatibilityVersion::new(1, 0, 0);

    const V1_0_1: CompatibilityVersion =
        CompatibilityVersion::new(1, 0, 1);

    const V1_1_0: CompatibilityVersion =
        CompatibilityVersion::new(1, 1, 0);

    const V2_0_0: CompatibilityVersion =
        CompatibilityVersion::new(2, 0, 0);

    #[test]
    fn exact_versions_are_exact() {
        let decision =
            check_ir_version(V1_0_0, V1_0_0);

        assert_eq!(decision.class, CompatibilityClass::Exact);
        assert_eq!(
            decision.reason,
            CompatibilityReason::ExactVersion
        );
        assert!(decision.is_directly_readable());
        assert!(!decision.requires_migration());
        assert!(!decision.must_reject());
    }

    #[test]
    fn older_minor_version_is_backward_compatible() {
        let decision =
            check_ir_version(V1_0_0, V1_1_0);

        assert_eq!(
            decision.class,
            CompatibilityClass::BackwardCompatible
        );
        assert_eq!(
            decision.reason,
            CompatibilityReason::OlderCompatibleVersion
        );
    }

    #[test]
    fn different_major_versions_are_incompatible() {
        let decision =
            check_ir_version(V2_0_0, V1_0_0);

        assert_eq!(
            decision.class,
            CompatibilityClass::Incompatible
        );
        assert_eq!(
            decision.reason,
            CompatibilityReason::MajorVersionMismatch
        );
    }

    #[test]
    fn future_minor_versions_are_not_guessed_as_compatible() {
        let decision =
            check_ir_version(V1_1_0, V1_0_0);

        assert_eq!(
            decision.class,
            CompatibilityClass::Incompatible
        );
        assert_eq!(
            decision.reason,
            CompatibilityReason::FutureMinorVersion
        );
    }

    #[test]
    fn future_patch_versions_are_conservative_by_default() {
        let decision =
            check_ir_version(V1_0_1, V1_0_0);

        assert_eq!(
            decision.class,
            CompatibilityClass::Incompatible
        );
        assert_eq!(
            decision.reason,
            CompatibilityReason::FuturePatchVersion
        );
    }

    #[test]
    fn explicitly_compatible_patch_can_be_accepted() {
        let rules = CompatibilityRules::new(
            VersionKind::SerializationFormat,
            V1_0_0,
            CompatibilityOptions::default(),
        );

        let decision =
            rules.check_patch_compatible(V1_0_1);

        assert_eq!(
            decision.class,
            CompatibilityClass::BackwardCompatible
        );
        assert_eq!(
            decision.reason,
            CompatibilityReason::NewerCompatiblePatch
        );
    }

    #[test]
    fn unknown_fields_are_preserved_by_default() {
        let rules = CompatibilityRules::new(
            VersionKind::SerializationFormat,
            V1_0_0,
            CompatibilityOptions::default(),
        );

        assert_eq!(
            rules.check_unknown_field(),
            Ok(true)
        );
    }

    #[test]
    fn unknown_fields_can_be_rejected() {
        let options = CompatibilityOptions {
            unknown_fields: UnknownFieldPolicy::Reject,
            ..CompatibilityOptions::default()
        };

        let rules = CompatibilityRules::new(
            VersionKind::SerializationFormat,
            V1_0_0,
            options,
        );

        assert_eq!(
            rules.check_unknown_field(),
            Err(CompatibilityError::UnknownField)
        );
    }

    #[test]
    fn unknown_fields_can_be_explicitly_discarded() {
        let options = CompatibilityOptions {
            unknown_fields: UnknownFieldPolicy::Discard,
            ..CompatibilityOptions::default()
        };

        let rules = CompatibilityRules::new(
            VersionKind::SerializationFormat,
            V1_0_0,
            options,
        );

        assert_eq!(
            rules.check_unknown_field(),
            Ok(false)
        );
    }

    #[test]
    fn unknown_extensions_are_preserved_by_default() {
        let rules = CompatibilityRules::new(
            VersionKind::SerializationFormat,
            V1_0_0,
            CompatibilityOptions::default(),
        );

        assert_eq!(
            rules.check_unknown_extension(),
            Ok(true)
        );
    }

    #[test]
    fn unknown_extensions_can_be_rejected() {
        let options = CompatibilityOptions {
            unknown_extensions: UnknownExtensionPolicy::Reject,
            ..CompatibilityOptions::default()
        };

        let rules = CompatibilityRules::new(
            VersionKind::SerializationFormat,
            V1_0_0,
            options,
        );

        assert_eq!(
            rules.check_unknown_extension(),
            Err(CompatibilityError::UnknownExtension)
        );
    }

    #[test]
    fn migration_can_be_required() {
        let requirement = require_upgrade(
            VersionKind::SerializationFormat,
            V1_0_0,
            V1_1_0,
            false,
        );

        let assessment = assess(
            &CompatibilityRules::new(
                VersionKind::SerializationFormat,
                V1_1_0,
                CompatibilityOptions::default(),
            ),
            V1_0_0,
            Some(requirement),
        )
        .expect("migration should be permitted");

        assert!(assessment.requires_migration());
        assert!(assessment.is_acceptable());
        assert!(assessment.lossless);
    }

    #[test]
    fn migration_can_be_disabled() {
        let options = CompatibilityOptions {
            allow_migration: false,
            ..CompatibilityOptions::default()
        };

        let rules = CompatibilityRules::new(
            VersionKind::SerializationFormat,
            V1_1_0,
            options,
        );

        let requirement = require_upgrade(
            VersionKind::SerializationFormat,
            V1_0_0,
            V1_1_0,
            false,
        );

        assert_eq!(
            rules.check_migration(requirement),
            Err(CompatibilityError::MigrationNotAllowed(
                requirement
            ))
        );
    }

    #[test]
    fn lossy_migration_is_forbidden_by_default() {
        let rules = CompatibilityRules::new(
            VersionKind::SerializationFormat,
            V1_1_0,
            CompatibilityOptions::default(),
        );

        let requirement = require_upgrade(
            VersionKind::SerializationFormat,
            V1_0_0,
            V1_1_0,
            true,
        );

        assert_eq!(
            rules.check_migration(requirement),
            Err(CompatibilityError::LossNotAllowed(
                requirement
            ))
        );
    }

    #[test]
    fn declared_lossy_migration_can_be_allowed() {
        let options = CompatibilityOptions {
            loss_policy: LossPolicy::AllowDeclared,
            ..CompatibilityOptions::default()
        };

        let rules = CompatibilityRules::new(
            VersionKind::SerializationFormat,
            V1_1_0,
            options,
        );

        let requirement = require_upgrade(
            VersionKind::SerializationFormat,
            V1_0_0,
            V1_1_0,
            true,
        );

        assert_eq!(
            rules.check_migration(requirement),
            Ok(())
        );
    }

    #[test]
    fn downgrade_is_directional() {
        let requirement = require_downgrade(
            VersionKind::SerializationFormat,
            V1_1_0,
            V1_0_0,
            true,
        );

        assert_eq!(
            requirement.direction,
            MigrationDirection::Downgrade
        );
        assert!(requirement.potentially_lossy);
    }

    #[test]
    fn version_ordering_is_deterministic() {
        assert!(V1_0_0.is_older_than(V1_0_1));
        assert!(V1_0_1.is_older_than(V1_1_0));
        assert!(V1_1_0.is_older_than(V2_0_0));
        assert!(V2_0_0.is_newer_than(V1_1_0));
    }

    #[test]
    fn semantic_and_wire_versions_are_distinct_kinds() {
        let ir = check_version(
            VersionKind::Ir,
            V1_0_0,
            V1_0_0,
        );

        let wire = check_version(
            VersionKind::SerializationFormat,
            V1_0_0,
            V1_0_0,
        );

        assert_eq!(ir.class, CompatibilityClass::Exact);
        assert_eq!(wire.class, CompatibilityClass::Exact);
        assert_eq!(ir.kind, VersionKind::Ir);
        assert_eq!(
            wire.kind,
            VersionKind::SerializationFormat
        );
    }

    #[test]
    fn default_options_are_non_lossy_and_forward_preserving() {
        let options = CompatibilityOptions::default();

        assert_eq!(
            options.unknown_fields,
            UnknownFieldPolicy::Preserve
        );
        assert_eq!(
            options.unknown_extensions,
            UnknownExtensionPolicy::Preserve
        );
        assert_eq!(
            options.loss_policy,
            LossPolicy::Forbid
        );
        assert!(options.allow_migration);
    }

    #[test]
    fn assessment_can_be_directly_decoded_for_exact_match() {
        let rules = CompatibilityRules::new(
            VersionKind::SerializationFormat,
            V1_0_0,
            CompatibilityOptions::default(),
        );

        let assessment =
            assess(&rules, V1_0_0, None)
                .expect("exact versions should be readable");

        assert!(assessment.can_decode_directly());
        assert!(!assessment.requires_migration());
        assert!(assessment.is_acceptable());
    }

    #[test]
    fn compatibility_helpers_are_consistent() {
        assert!(is_exact_version(V1_0_0, V1_0_0));
        assert!(!is_exact_version(V1_0_0, V1_0_1));

        assert!(has_compatible_major(V1_0_0, V1_0_1));
        assert!(!has_compatible_major(V1_0_0, V2_0_0));

        assert!(is_backward_compatible(
            VersionKind::Ir,
            V1_0_0,
            V1_1_0
        ));

        assert!(!is_backward_compatible(
            VersionKind::Ir,
            V2_0_0,
            V1_0_0
        ));
    }
}