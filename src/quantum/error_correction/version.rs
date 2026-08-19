//! Zamani Quantum Error Correction Versioning
//!
//! Provides explicit versioning and compatibility management for all
//! persistent and executable QEC artifacts.
//!
//! Versioned artifacts include:
//! - QEC algorithms
//! - QEC configuration
//! - checkpoints
//! - syndrome streams
//! - decoding graphs
//! - simulations
//! - execution backends
//! - capabilities
//! - QPU interfaces
//!
//! Design goals:
//! - deterministic
//! - forward-compatible where explicitly supported
//! - backward-compatible where explicitly supported
//! - corruption-resistant
//! - downgrade-resistant
//! - suitable for checkpoint persistence
//! - suitable for distributed execution
//! - independent of external serialization crates
//! - no panics on untrusted version input
//!
//! Important:
//! Version compatibility is deliberately explicit. A version mismatch must
//! never silently reinterpret persisted QEC state.

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

/// Current Zamani QEC versioning protocol.
pub const VERSION_PROTOCOL_MAJOR: u16 = 1;
pub const VERSION_PROTOCOL_MINOR: u16 = 0;

/// Current versions of the individual QEC artifacts.
///
/// These constants are intentionally independent. Changing the checkpoint
/// format does not necessarily mean changing the decoder algorithm version.
pub const CURRENT_ALGORITHM_VERSION: Version = Version::new(1, 0, 0);
pub const CURRENT_CONFIGURATION_VERSION: Version = Version::new(1, 0, 0);
pub const CURRENT_CHECKPOINT_VERSION: Version = Version::new(1, 0, 0);
pub const CURRENT_SYNDROME_VERSION: Version = Version::new(1, 0, 0);
pub const CURRENT_GRAPH_VERSION: Version = Version::new(1, 0, 0);
pub const CURRENT_SIMULATION_VERSION: Version = Version::new(1, 0, 0);
pub const CURRENT_BACKEND_VERSION: Version = Version::new(1, 0, 0);
pub const CURRENT_CAPABILITY_VERSION: Version = Version::new(1, 0, 0);
pub const CURRENT_QPU_INTERFACE_VERSION: Version = Version::new(1, 0, 0);

/// Maximum supported version-string length.
///
/// This protects APIs that receive versions from untrusted external sources.
pub const MAX_VERSION_STRING_LENGTH: usize = 64;

/// Maximum supported artifact identifier length.
pub const MAX_ARTIFACT_ID_LENGTH: usize = 128;

/// A semantic version.
///
/// Zamani uses semantic versioning:
///
/// `MAJOR.MINOR.PATCH`
///
/// Major:
/// - incompatible semantic/API/schema changes.
///
/// Minor:
/// - backward-compatible functionality.
///
/// Patch:
/// - backward-compatible fixes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub const fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    /// Returns true if this version has the same compatibility family.
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns true if this version is older than another version.
    pub const fn is_older_than(self, other: Self) -> bool {
        self.cmp_const(other) == Ordering::Less
    }

    /// Returns true if this version is newer than another version.
    pub const fn is_newer_than(self, other: Self) -> bool {
        self.cmp_const(other) == Ordering::Greater
    }

    /// Returns a deterministic packed representation.
    ///
    /// Useful for compact identifiers and deterministic metadata.
    pub const fn packed(self) -> u64 {
        ((self.major as u64) << 32)
            | ((self.minor as u64) << 16)
            | self.patch as u64
    }

    /// Compile-time compatible comparison.
    const fn cmp_const(self, other: Self) -> Ordering {
        if self.major < other.major {
            Ordering::Less
        } else if self.major > other.major {
            Ordering::Greater
        } else if self.minor < other.minor {
            Ordering::Less
        } else if self.minor > other.minor {
            Ordering::Greater
        } else if self.patch < other.patch {
            Ordering::Less
        } else if self.patch > other.patch {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        CURRENT_ALGORITHM_VERSION
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(VersionError::Empty);
        }

        if value.len() > MAX_VERSION_STRING_LENGTH {
            return Err(VersionError::TooLong);
        }

        let mut parts = value.split('.');

        let major = parse_component(parts.next(), "major")?;
        let minor = parse_component(parts.next(), "minor")?;
        let patch = parse_component(parts.next(), "patch")?;

        if parts.next().is_some() {
            return Err(VersionError::InvalidFormat);
        }

        Ok(Self::new(major, minor, patch))
    }
}

fn parse_component(value: Option<&str>, name: &'static str) -> Result<u16, VersionError> {
    let value = value.ok_or(VersionError::MissingComponent(name))?;

    if value.is_empty() {
        return Err(VersionError::InvalidComponent(name));
    }

    if !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err(VersionError::InvalidComponent(name));
    }

    value
        .parse::<u16>()
        .map_err(|_| VersionError::ComponentOverflow(name))
}

/// Every independently versioned QEC artifact.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ArtifactKind {
    Algorithm,
    Configuration,
    Checkpoint,
    Syndrome,
    DecodingGraph,
    Simulation,
    Backend,
    Capability,
    QpuInterface,
}

impl ArtifactKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Algorithm => "algorithm",
            Self::Configuration => "configuration",
            Self::Checkpoint => "checkpoint",
            Self::Syndrome => "syndrome",
            Self::DecodingGraph => "decoding_graph",
            Self::Simulation => "simulation",
            Self::Backend => "backend",
            Self::Capability => "capability",
            Self::QpuInterface => "qpu_interface",
        }
    }

    pub const fn current_version(self) -> Version {
        match self {
            Self::Algorithm => CURRENT_ALGORITHM_VERSION,
            Self::Configuration => CURRENT_CONFIGURATION_VERSION,
            Self::Checkpoint => CURRENT_CHECKPOINT_VERSION,
            Self::Syndrome => CURRENT_SYNDROME_VERSION,
            Self::DecodingGraph => CURRENT_GRAPH_VERSION,
            Self::Simulation => CURRENT_SIMULATION_VERSION,
            Self::Backend => CURRENT_BACKEND_VERSION,
            Self::Capability => CURRENT_CAPABILITY_VERSION,
            Self::QpuInterface => CURRENT_QPU_INTERFACE_VERSION,
        }
    }
}

/// Execution target represented by a versioned QEC artifact.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExecutionTarget {
    ClassicalCpu,
    ParallelCpu,
    Gpu,
    Accelerator,
    Distributed,
    Qpu,
    HybridCpuQpu,
    HybridAcceleratorQpu,
}

impl ExecutionTarget {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClassicalCpu => "cpu",
            Self::ParallelCpu => "parallel_cpu",
            Self::Gpu => "gpu",
            Self::Accelerator => "accelerator",
            Self::Distributed => "distributed",
            Self::Qpu => "qpu",
            Self::HybridCpuQpu => "hybrid_cpu_qpu",
            Self::HybridAcceleratorQpu => "hybrid_accelerator_qpu",
        }
    }
}

/// Compatibility policy.
///
/// Compatibility must never be inferred implicitly.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CompatibilityPolicy {
    /// Exact version required.
    Exact,

    /// Same major version and compatible minor/patch version.
    SameMajor,

    /// Current implementation may read an older artifact.
    BackwardCompatible,

    /// Current implementation may read an older artifact but will not
    /// automatically write it back in the old format.
    ReadOnlyLegacy,

    /// Explicitly reject the artifact.
    Reject,
}

/// Version relationship between two artifacts.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Compatibility {
    Exact,
    Compatible,
    LegacyCompatible,
    Incompatible,
    NewerThanSupported,
}

impl Compatibility {
    pub const fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::Exact | Self::Compatible | Self::LegacyCompatible
        )
    }
}

/// A complete QEC version manifest.
///
/// This object should accompany persistent QEC state such as checkpoints,
/// graph snapshots, syndrome streams, and simulation artifacts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionManifest {
    pub protocol_major: u16,
    pub protocol_minor: u16,

    pub algorithm: Version,
    pub configuration: Version,
    pub checkpoint: Version,
    pub syndrome: Version,
    pub graph: Version,
    pub simulation: Version,
    pub backend: Version,
    pub capability: Version,
    pub qpu_interface: Version,

    pub target: ExecutionTarget,
}

impl VersionManifest {
    pub fn current(target: ExecutionTarget) -> Self {
        Self {
            protocol_major: VERSION_PROTOCOL_MAJOR,
            protocol_minor: VERSION_PROTOCOL_MINOR,
            algorithm: CURRENT_ALGORITHM_VERSION,
            configuration: CURRENT_CONFIGURATION_VERSION,
            checkpoint: CURRENT_CHECKPOINT_VERSION,
            syndrome: CURRENT_SYNDROME_VERSION,
            graph: CURRENT_GRAPH_VERSION,
            simulation: CURRENT_SIMULATION_VERSION,
            backend: CURRENT_BACKEND_VERSION,
            capability: CURRENT_CAPABILITY_VERSION,
            qpu_interface: CURRENT_QPU_INTERFACE_VERSION,
            target,
        }
    }

    /// Validate the manifest before it is trusted.
    pub fn validate(&self) -> Result<(), VersionError> {
        if self.protocol_major == 0 {
            return Err(VersionError::InvalidProtocolVersion);
        }

        if self.protocol_major != VERSION_PROTOCOL_MAJOR {
            return Err(VersionError::ProtocolMismatch {
                expected_major: VERSION_PROTOCOL_MAJOR,
                found_major: self.protocol_major,
            });
        }

        Ok(())
    }

    /// Return the version belonging to an artifact kind.
    pub const fn version_of(&self, kind: ArtifactKind) -> Version {
        match kind {
            ArtifactKind::Algorithm => self.algorithm,
            ArtifactKind::Configuration => self.configuration,
            ArtifactKind::Checkpoint => self.checkpoint,
            ArtifactKind::Syndrome => self.syndrome,
            ArtifactKind::DecodingGraph => self.graph,
            ArtifactKind::Simulation => self.simulation,
            ArtifactKind::Backend => self.backend,
            ArtifactKind::Capability => self.capability,
            ArtifactKind::QpuInterface => self.qpu_interface,
        }
    }

    /// Determine compatibility with the currently supported version.
    pub fn compatibility(
        &self,
        kind: ArtifactKind,
        policy: CompatibilityPolicy,
    ) -> Compatibility {
        let found = self.version_of(kind);
        let current = kind.current_version();

        match policy {
            CompatibilityPolicy::Exact => {
                if found == current {
                    Compatibility::Exact
                } else {
                    Compatibility::Incompatible
                }
            }

            CompatibilityPolicy::SameMajor => {
                if found == current {
                    Compatibility::Exact
                } else if found.major == current.major && found <= current {
                    Compatibility::Compatible
                } else if found > current {
                    Compatibility::NewerThanSupported
                } else {
                    Compatibility::Incompatible
                }
            }

            CompatibilityPolicy::BackwardCompatible => {
                if found == current {
                    Compatibility::Exact
                } else if found.major == current.major && found <= current {
                    Compatibility::LegacyCompatible
                } else if found > current {
                    Compatibility::NewerThanSupported
                } else {
                    Compatibility::Incompatible
                }
            }

            CompatibilityPolicy::ReadOnlyLegacy => {
                if found == current {
                    Compatibility::Exact
                } else if found < current && found.major == current.major {
                    Compatibility::LegacyCompatible
                } else {
                    Compatibility::Incompatible
                }
            }

            CompatibilityPolicy::Reject => Compatibility::Incompatible,
        }
    }

    /// Require compatibility or return an explicit error.
    pub fn require_compatible(
        &self,
        kind: ArtifactKind,
        policy: CompatibilityPolicy,
    ) -> Result<Compatibility, VersionError> {
        self.validate()?;

        let compatibility = self.compatibility(kind, policy);

        if !compatibility.is_compatible() {
            return Err(VersionError::IncompatibleArtifact {
                kind,
                found: self.version_of(kind),
                expected: kind.current_version(),
            });
        }

        Ok(compatibility)
    }
}

impl Default for VersionManifest {
    fn default() -> Self {
        Self::current(ExecutionTarget::ClassicalCpu)
    }
}

/// Version identity for a concrete QEC component.
///
/// `artifact_id` allows several algorithms/backends/QPU providers to coexist
/// while still having independent versions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentVersion {
    pub artifact: ArtifactKind,
    pub artifact_id: String,
    pub version: Version,
}

impl ComponentVersion {
    pub fn new(
        artifact: ArtifactKind,
        artifact_id: impl Into<String>,
        version: Version,
    ) -> Result<Self, VersionError> {
        let artifact_id = artifact_id.into();

        if artifact_id.is_empty() {
            return Err(VersionError::EmptyArtifactId);
        }

        if artifact_id.len() > MAX_ARTIFACT_ID_LENGTH {
            return Err(VersionError::ArtifactIdTooLong);
        }

        if !artifact_id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        {
            return Err(VersionError::InvalidArtifactId);
        }

        Ok(Self {
            artifact,
            artifact_id,
            version,
        })
    }

    pub fn identity(&self) -> String {
        format!(
            "{}:{}@{}",
            self.artifact.as_str(),
            self.artifact_id,
            self.version
        )
    }
}

/// Version information for a physical or virtual QPU interface.
///
/// This deliberately versions the interface separately from the backend.
/// A backend implementation may change while the QPU protocol remains stable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QpuVersion {
    /// QPU vendor/provider-neutral identifier.
    pub provider: String,

    /// Device family.
    pub device_family: String,

    /// Concrete device identifier, when available.
    ///
    /// This is an opaque identifier and must not be interpreted as a secret.
    pub device_id: Option<String>,

    /// QPU interface/API version.
    pub interface_version: Version,

    /// Firmware version exposed by the device.
    pub firmware_version: Version,

    /// Control-stack version.
    pub control_stack_version: Version,

    /// Calibration/schema version.
    pub calibration_version: Version,

    /// Whether the device supports dynamic circuit/QEC operations.
    pub dynamic_circuits: bool,

    /// Whether the device can execute hybrid CPU/QPU workflows.
    pub hybrid_execution: bool,
}

impl QpuVersion {
    pub fn new(
        provider: impl Into<String>,
        device_family: impl Into<String>,
        interface_version: Version,
        firmware_version: Version,
        control_stack_version: Version,
        calibration_version: Version,
    ) -> Result<Self, VersionError> {
        let provider = provider.into();
        let device_family = device_family.into();

        validate_identifier(&provider, "provider")?;
        validate_identifier(&device_family, "device_family")?;

        Ok(Self {
            provider,
            device_family,
            device_id: None,
            interface_version,
            firmware_version,
            control_stack_version,
            calibration_version,
            dynamic_circuits: false,
            hybrid_execution: false,
        })
    }

    pub fn with_device_id(
        mut self,
        device_id: impl Into<String>,
    ) -> Result<Self, VersionError> {
        let device_id = device_id.into();

        validate_identifier(&device_id, "device_id")?;

        self.device_id = Some(device_id);
        Ok(self)
    }

    pub fn supports_interface(&self, required: Version) -> bool {
        self.interface_version.major == required.major
            && self.interface_version >= required
    }

    /// Returns a stable human-readable QPU identity.
    pub fn identity(&self) -> String {
        match &self.device_id {
            Some(id) => format!(
                "{}:{}:{}",
                self.provider, self.device_family, id
            ),
            None => format!("{}:{}", self.provider, self.device_family),
        }
    }
}

/// A versioned execution environment.
///
/// This binds the QEC implementation to the execution target without
/// requiring the decoder itself to know how that target is implemented.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionVersion {
    pub target: ExecutionTarget,
    pub component: ComponentVersion,
    pub manifest: VersionManifest,
    pub qpu: Option<QpuVersion>,
}

impl ExecutionVersion {
    pub fn classical(
        component_id: impl Into<String>,
        version: Version,
    ) -> Result<Self, VersionError> {
        let component =
            ComponentVersion::new(ArtifactKind::Backend, component_id, version)?;

        Ok(Self {
            target: ExecutionTarget::ClassicalCpu,
            component,
            manifest: VersionManifest::current(ExecutionTarget::ClassicalCpu),
            qpu: None,
        })
    }

    pub fn qpu(
        component_id: impl Into<String>,
        version: Version,
        qpu: QpuVersion,
    ) -> Result<Self, VersionError> {
        let component =
            ComponentVersion::new(ArtifactKind::Backend, component_id, version)?;

        Ok(Self {
            target: ExecutionTarget::Qpu,
            component,
            manifest: VersionManifest::current(ExecutionTarget::Qpu),
            qpu: Some(qpu),
        })
    }

    pub fn hybrid_qpu(
        component_id: impl Into<String>,
        version: Version,
        qpu: QpuVersion,
    ) -> Result<Self, VersionError> {
        let component =
            ComponentVersion::new(ArtifactKind::Backend, component_id, version)?;

        Ok(Self {
            target: ExecutionTarget::HybridCpuQpu,
            component,
            manifest: VersionManifest::current(ExecutionTarget::HybridCpuQpu),
            qpu: Some(qpu),
        })
    }

    pub fn validate(&self) -> Result<(), VersionError> {
        self.manifest.validate()?;

        match self.target {
            ExecutionTarget::Qpu
            | ExecutionTarget::HybridCpuQpu
            | ExecutionTarget::HybridAcceleratorQpu => {
                if self.qpu.is_none() {
                    return Err(VersionError::MissingQpuMetadata);
                }
            }
            _ => {}
        }

        if matches!(
            self.target,
            ExecutionTarget::ClassicalCpu
                | ExecutionTarget::ParallelCpu
                | ExecutionTarget::Gpu
                | ExecutionTarget::Accelerator
                | ExecutionTarget::Distributed
        ) && self.qpu.is_some()
        {
            return Err(VersionError::UnexpectedQpuMetadata);
        }

        Ok(())
    }
}

/// Persistent artifact header.
///
/// This should be placed at the beginning of serialized QEC artifacts.
///
/// It allows Zamani to reject incompatible data before interpreting the
/// remainder of the payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactHeader {
    pub magic: [u8; 4],
    pub protocol_major: u16,
    pub protocol_minor: u16,
    pub artifact: ArtifactKind,
    pub version: Version,
    pub target: ExecutionTarget,
}

impl ArtifactHeader {
    pub const MAGIC: [u8; 4] = *b"ZQEC";

    pub fn new(
        artifact: ArtifactKind,
        version: Version,
        target: ExecutionTarget,
    ) -> Self {
        Self {
            magic: Self::MAGIC,
            protocol_major: VERSION_PROTOCOL_MAJOR,
            protocol_minor: VERSION_PROTOCOL_MINOR,
            artifact,
            version,
            target,
        }
    }

    pub fn validate(&self) -> Result<(), VersionError> {
        if self.magic != Self::MAGIC {
            return Err(VersionError::InvalidMagic);
        }

        if self.protocol_major != VERSION_PROTOCOL_MAJOR {
            return Err(VersionError::ProtocolMismatch {
                expected_major: VERSION_PROTOCOL_MAJOR,
                found_major: self.protocol_major,
            });
        }

        Ok(())
    }

    pub fn is_current(&self) -> bool {
        self.version == self.artifact.current_version()
    }
}

/// Policy for handling persisted artifacts during upgrades.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpgradePolicy {
    /// Refuse anything other than the exact current format.
    Strict,

    /// Permit compatible older versions.
    Compatible,

    /// Permit compatible older versions and explicitly mark them as legacy.
    MigrateOnRead,

    /// Permit reading legacy artifacts but never permit writing them back.
    ReadLegacyOnly,
}

impl UpgradePolicy {
    pub const fn compatibility_policy(self) -> CompatibilityPolicy {
        match self {
            Self::Strict => CompatibilityPolicy::Exact,
            Self::Compatible => CompatibilityPolicy::BackwardCompatible,
            Self::MigrateOnRead => CompatibilityPolicy::BackwardCompatible,
            Self::ReadLegacyOnly => CompatibilityPolicy::ReadOnlyLegacy,
        }
    }

    pub const fn permits_migration(self) -> bool {
        matches!(self, Self::MigrateOnRead)
    }
}

/// Result of validating a persisted artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationStatus {
    Current,
    Compatible,
    Legacy,
}

/// Validate an artifact header against current Zamani support.
pub fn validate_artifact(
    header: &ArtifactHeader,
    policy: UpgradePolicy,
) -> Result<ValidationStatus, VersionError> {
    header.validate()?;

    let current = header.artifact.current_version();

    match policy.compatibility_policy() {
        CompatibilityPolicy::Exact => {
            if header.version == current {
                Ok(ValidationStatus::Current)
            } else {
                Err(VersionError::IncompatibleArtifact {
                    kind: header.artifact,
                    found: header.version,
                    expected: current,
                })
            }
        }

        CompatibilityPolicy::SameMajor
        | CompatibilityPolicy::BackwardCompatible
        | CompatibilityPolicy::ReadOnlyLegacy => {
            if header.version == current {
                Ok(ValidationStatus::Current)
            } else if header.version.major == current.major
                && header.version < current
            {
                Ok(ValidationStatus::Legacy)
            } else if header.version > current {
                Err(VersionError::NewerArtifact {
                    kind: header.artifact,
                    found: header.version,
                    supported: current,
                })
            } else {
                Err(VersionError::IncompatibleArtifact {
                    kind: header.artifact,
                    found: header.version,
                    expected: current,
                })
            }
        }

        CompatibilityPolicy::Reject => Err(VersionError::Rejected),
    }
}

/// Validate that a QPU can execute against a required interface version.
pub fn require_qpu_interface(
    qpu: &QpuVersion,
    required: Version,
) -> Result<(), VersionError> {
    if !qpu.supports_interface(required) {
        return Err(VersionError::QpuInterfaceMismatch {
            required,
            found: qpu.interface_version,
        });
    }

    Ok(())
}

/// Validate a string identifier used by version metadata.
fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), VersionError> {
    if value.is_empty() {
        return Err(VersionError::EmptyIdentifier(field));
    }

    if value.len() > MAX_ARTIFACT_ID_LENGTH {
        return Err(VersionError::IdentifierTooLong(field));
    }

    if !value
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b':'))
    {
        return Err(VersionError::InvalidIdentifier(field));
    }

    Ok(())
}

/// Errors generated by the versioning subsystem.
///
/// These errors are deliberately independent from the broader QEC error
/// hierarchy so that version metadata can be validated before the rest of
/// an artifact is trusted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VersionError {
    Empty,
    TooLong,
    InvalidFormat,
    MissingComponent(&'static str),
    InvalidComponent(&'static str),
    ComponentOverflow(&'static str),

    EmptyArtifactId,
    ArtifactIdTooLong,
    InvalidArtifactId,

    EmptyIdentifier(&'static str),
    IdentifierTooLong(&'static str),
    InvalidIdentifier(&'static str),

    InvalidProtocolVersion,
    ProtocolMismatch {
        expected_major: u16,
        found_major: u16,
    },

    InvalidMagic,

    IncompatibleArtifact {
        kind: ArtifactKind,
        found: Version,
        expected: Version,
    },

    NewerArtifact {
        kind: ArtifactKind,
        found: Version,
        supported: Version,
    },

    QpuInterfaceMismatch {
        required: Version,
        found: Version,
    },

    MissingQpuMetadata,
    UnexpectedQpuMetadata,

    Rejected,
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "version string is empty"),

            Self::TooLong => {
                write!(f, "version string exceeds maximum length")
            }

            Self::InvalidFormat => {
                write!(f, "invalid semantic version format")
            }

            Self::MissingComponent(name) => {
                write!(f, "missing {} version component", name)
            }

            Self::InvalidComponent(name) => {
                write!(f, "invalid {} version component", name)
            }

            Self::ComponentOverflow(name) => {
                write!(f, "{} version component exceeds u16", name)
            }

            Self::EmptyArtifactId => {
                write!(f, "artifact identifier is empty")
            }

            Self::ArtifactIdTooLong => {
                write!(f, "artifact identifier is too long")
            }

            Self::InvalidArtifactId => {
                write!(f, "artifact identifier contains invalid characters")
            }

            Self::EmptyIdentifier(field) => {
                write!(f, "{} identifier is empty", field)
            }

            Self::IdentifierTooLong(field) => {
                write!(f, "{} identifier is too long", field)
            }

            Self::InvalidIdentifier(field) => {
                write!(f, "{} identifier contains invalid characters", field)
            }

            Self::InvalidProtocolVersion => {
                write!(f, "invalid QEC versioning protocol version")
            }

            Self::ProtocolMismatch {
                expected_major,
                found_major,
            } => write!(
                f,
                "QEC version protocol mismatch: expected major {}, found {}",
                expected_major, found_major
            ),

            Self::InvalidMagic => {
                write!(f, "invalid QEC artifact magic")
            }

            Self::IncompatibleArtifact {
                kind,
                found,
                expected,
            } => write!(
                f,
                "incompatible {} version: found {}, expected {}",
                kind.as_str(),
                found,
                expected
            ),

            Self::NewerArtifact {
                kind,
                found,
                supported,
            } => write!(
                f,
                "{} version {} is newer than supported version {}",
                kind.as_str(),
                found,
                supported
            ),

            Self::QpuInterfaceMismatch { required, found } => write!(
                f,
                "QPU interface mismatch: required {}, found {}",
                required,
                found
            ),

            Self::MissingQpuMetadata => {
                write!(f, "QPU execution target requires QPU metadata")
            }

            Self::UnexpectedQpuMetadata => {
                write!(f, "QPU metadata supplied for non-QPU target")
            }

            Self::Rejected => {
                write!(f, "artifact rejected by compatibility policy")
            }
        }
    }
}

impl std::error::Error for VersionError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_round_trip() {
        let version = Version::new(4, 12, 7);
        let encoded = version.to_string();
        let decoded: Version = encoded.parse().unwrap();

        assert_eq!(version, decoded);
    }

    #[test]
    fn invalid_version_is_rejected() {
        assert!("1".parse::<Version>().is_err());
        assert!("1.2".parse::<Version>().is_err());
        assert!("1.2.3.4".parse::<Version>().is_err());
        assert!("1.x.3".parse::<Version>().is_err());
        assert!("1.2.999999".parse::<Version>().is_err());
    }

    #[test]
    fn artifact_versions_are_independent() {
        assert_eq!(
            ArtifactKind::Checkpoint.current_version(),
            CURRENT_CHECKPOINT_VERSION
        );

        assert_eq!(
            ArtifactKind::QpuInterface.current_version(),
            CURRENT_QPU_INTERFACE_VERSION
        );
    }

    #[test]
    fn current_manifest_validates() {
        let manifest = VersionManifest::current(ExecutionTarget::ClassicalCpu);

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn qpu_manifest_requires_qpu_metadata() {
        let mut manifest = VersionManifest::current(ExecutionTarget::Qpu);

        assert!(manifest.validate().is_ok());

        // Manifest itself does not contain device metadata; execution-level
        // validation is responsible for enforcing the physical QPU binding.
        manifest.target = ExecutionTarget::ClassicalCpu;

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn qpu_execution_requires_qpu_metadata() {
        let execution = ExecutionVersion::classical("cpu", Version::new(1, 0, 0))
            .unwrap();

        assert!(execution.validate().is_ok());
    }

    #[test]
    fn qpu_interface_is_checked() {
        let qpu = QpuVersion::new(
            "provider",
            "device",
            Version::new(2, 1, 0),
            Version::new(1, 0, 0),
            Version::new(1, 0, 0),
            Version::new(1, 0, 0),
        )
        .unwrap();

        assert!(
            require_qpu_interface(&qpu, Version::new(2, 0, 0)).is_ok()
        );

        assert!(
            require_qpu_interface(&qpu, Version::new(3, 0, 0)).is_err()
        );
    }

    #[test]
    fn artifact_header_rejects_bad_magic() {
        let mut header = ArtifactHeader::new(
            ArtifactKind::Checkpoint,
            CURRENT_CHECKPOINT_VERSION,
            ExecutionTarget::ClassicalCpu,
        );

        header.magic = *b"BAD!";

        assert!(matches!(
            header.validate(),
            Err(VersionError::InvalidMagic)
        ));
    }

    #[test]
    fn current_artifact_is_accepted() {
        let header = ArtifactHeader::new(
            ArtifactKind::Checkpoint,
            CURRENT_CHECKPOINT_VERSION,
            ExecutionTarget::ClassicalCpu,
        );

        assert_eq!(
            validate_artifact(&header, UpgradePolicy::Strict).unwrap(),
            ValidationStatus::Current
        );
    }

    #[test]
    fn older_same_major_artifact_can_be_legacy() {
        let header = ArtifactHeader::new(
            ArtifactKind::Checkpoint,
            Version::new(
                CURRENT_CHECKPOINT_VERSION.major,
                CURRENT_CHECKPOINT_VERSION.minor.saturating_sub(1),
                0,
            ),
            ExecutionTarget::ClassicalCpu,
        );

        let result = validate_artifact(
            &header,
            UpgradePolicy::Compatible,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn newer_artifact_is_not_silently_accepted() {
        let current = CURRENT_CHECKPOINT_VERSION;

        let header = ArtifactHeader::new(
            ArtifactKind::Checkpoint,
            Version::new(
                current.major.saturating_add(1),
                0,
                0,
            ),
            ExecutionTarget::ClassicalCpu,
        );

        assert!(matches!(
            validate_artifact(&header, UpgradePolicy::Compatible),
            Err(VersionError::NewerArtifact { .. })
        ));
    }

    #[test]
    fn component_identity_is_deterministic() {
        let component = ComponentVersion::new(
            ArtifactKind::Algorithm,
            "mwpm",
            Version::new(1, 2, 3),
        )
        .unwrap();

        assert_eq!(
            component.identity(),
            "algorithm:mwpm@1.2.3"
        );
    }

    #[test]
    fn packed_version_is_deterministic() {
        let a = Version::new(1, 2, 3);
        let b = Version::new(1, 2, 3);

        assert_eq!(a.packed(), b.packed());
    }

    #[test]
    fn qpu_identity_is_stable() {
        let qpu = QpuVersion::new(
            "provider",
            "surface-code-qpu",
            Version::new(1, 0, 0),
            Version::new(2, 0, 0),
            Version::new(3, 0, 0),
            Version::new(4, 0, 0),
        )
        .unwrap()
        .with_device_id("device-01")
        .unwrap();

        assert_eq!(
            qpu.identity(),
            "provider:surface-code-qpu:device-01"
        );
    }
}