//! Zamani Quantum Error Correction Versioning
//!
//! Canonical version and compatibility layer for the QEC subsystem.
//!
//! Versioned artifacts include:
//!
//! - algorithms
//! - configuration
//! - checkpoints
//! - syndrome streams
//! - decoding graphs
//! - simulation artifacts
//! - noise models
//! - decoder results
//! - execution backends
//! - capabilities
//! - QPU interfaces
//! - QPU execution schemas
//!
//! Architectural contract:
//!
//! ```text
//! UNTRUSTED ARTIFACT
//!       │
//!       ▼
//! ArtifactHeader
//!       │
//!       ▼
//! Protocol validation
//!       │
//!       ▼
//! Artifact/version validation
//!       │
//!       ▼
//! Component compatibility
//!       │
//!       ▼
//! Configuration compatibility
//!       │
//!       ▼
//! Backend/QPU compatibility
//!       │
//!       ▼
//! EXECUTION
//! ```
//!
//! Version compatibility is NEVER inferred from a version number alone.
//!
//! A major-version mismatch is incompatible unless an explicit migration
//! layer is introduced in the future.
//!
//! Newer artifacts must never be silently interpreted as older artifacts.
//!
//! This module intentionally does not depend on external serialization
//! libraries. It provides metadata and compatibility primitives which can be
//! consumed by checkpointing, caching, replay, distributed execution,
//! simulation, backend and QPU layers.
//!
//! Version errors remain locally typed because version metadata must be
//! validated before an artifact is trusted. High-level callers may convert
//! them to `QecError` at the public API boundary.

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

// ============================================================================
// Protocol versions
// ============================================================================

/// Current Zamani QEC versioning protocol major version.
pub const VERSION_PROTOCOL_MAJOR: u16 = 2;

/// Current Zamani QEC versioning protocol minor version.
pub const VERSION_PROTOCOL_MINOR: u16 = 0;

/// Maximum accepted external version-string length.
pub const MAX_VERSION_STRING_LENGTH: usize = 64;

/// Maximum accepted artifact/component identifier length.
pub const MAX_ARTIFACT_ID_LENGTH: usize = 128;

/// Maximum accepted provider/device identifier length.
pub const MAX_IDENTIFIER_LENGTH: usize = 128;

// ============================================================================
// Current artifact versions
// ============================================================================

/// Current algorithm schema/version.
pub const CURRENT_ALGORITHM_VERSION: Version = Version::new(2, 0, 0);

/// Current configuration schema/version.
pub const CURRENT_CONFIGURATION_VERSION: Version = Version::new(2, 0, 0);

/// Current checkpoint schema/version.
pub const CURRENT_CHECKPOINT_VERSION: Version = Version::new(2, 0, 0);

/// Current syndrome representation/version.
pub const CURRENT_SYNDROME_VERSION: Version = Version::new(2, 0, 0);

/// Current decoding-graph representation/version.
pub const CURRENT_GRAPH_VERSION: Version = Version::new(2, 0, 0);

/// Current simulation artifact/version.
pub const CURRENT_SIMULATION_VERSION: Version = Version::new(2, 0, 0);

/// Current noise-model representation/version.
pub const CURRENT_NOISE_MODEL_VERSION: Version = Version::new(1, 0, 0);

/// Current decoder-result representation/version.
pub const CURRENT_DECODER_RESULT_VERSION: Version = Version::new(1, 0, 0);

/// Current execution-backend representation/version.
pub const CURRENT_BACKEND_VERSION: Version = Version::new(2, 0, 0);

/// Current capability representation/version.
pub const CURRENT_CAPABILITY_VERSION: Version = Version::new(2, 0, 0);

/// Current QPU interface/API version.
pub const CURRENT_QPU_INTERFACE_VERSION: Version = Version::new(2, 0, 0);

/// Current QPU execution schema.
pub const CURRENT_QPU_EXECUTION_VERSION: Version = Version::new(1, 0, 0);

/// Current decoder-output schema.
pub const CURRENT_DECODER_OUTPUT_VERSION: Version = CURRENT_DECODER_RESULT_VERSION;

// ============================================================================
// Semantic version
// ============================================================================

/// Semantic version used by the QEC subsystem.
///
/// ```text
/// MAJOR.MINOR.PATCH
/// ```
///
/// Major:
/// incompatible API/schema/semantic changes.
///
/// Minor:
/// backwards-compatible functionality.
///
/// Patch:
/// backwards-compatible corrections.
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

    /// Whether two versions belong to the same compatibility family.
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Whether this version is older than another version.
    pub const fn is_older_than(self, other: Self) -> bool {
        self.cmp_const(other) == Ordering::Less
    }

    /// Whether this version is newer than another version.
    pub const fn is_newer_than(self, other: Self) -> bool {
        self.cmp_const(other) == Ordering::Greater
    }

    /// Deterministic compact representation.
    pub const fn packed(self) -> u64 {
        ((self.major as u64) << 32)
            | ((self.minor as u64) << 16)
            | self.patch as u64
    }

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

fn parse_component(
    value: Option<&str>,
    name: &'static str,
) -> Result<u16, VersionError> {
    let value = value.ok_or(VersionError::MissingComponent(name))?;

    if value.is_empty() {
        return Err(VersionError::InvalidComponent(name));
    }

    if !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(VersionError::InvalidComponent(name));
    }

    value
        .parse::<u16>()
        .map_err(|_| VersionError::ComponentOverflow(name))
}

// ============================================================================
// Artifact kinds
// ============================================================================

/// Independently versioned QEC artifact.
///
/// Keep this enum synchronized with the persistent formats used by
/// checkpointing, caching, replay, simulation and QPU execution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ArtifactKind {
    Algorithm,
    Configuration,
    Checkpoint,
    Syndrome,
    DecodingGraph,
    Simulation,
    NoiseModel,
    DecoderResult,
    Backend,
    Capability,
    QpuInterface,
    QpuExecution,
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
            Self::NoiseModel => "noise_model",
            Self::DecoderResult => "decoder_result",
            Self::Backend => "backend",
            Self::Capability => "capability",
            Self::QpuInterface => "qpu_interface",
            Self::QpuExecution => "qpu_execution",
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
            Self::NoiseModel => CURRENT_NOISE_MODEL_VERSION,
            Self::DecoderResult => CURRENT_DECODER_RESULT_VERSION,
            Self::Backend => CURRENT_BACKEND_VERSION,
            Self::Capability => CURRENT_CAPABILITY_VERSION,
            Self::QpuInterface => CURRENT_QPU_INTERFACE_VERSION,
            Self::QpuExecution => CURRENT_QPU_EXECUTION_VERSION,
        }
    }
}

impl fmt::Display for ArtifactKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Execution targets
// ============================================================================

/// Execution environment associated with a versioned artifact.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ExecutionTarget {
    ClassicalCpu,
    ParallelCpu,
    Gpu,
    Accelerator,
    Distributed,
    Simulator,
    Emulator,
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
            Self::Simulator => "simulator",
            Self::Emulator => "emulator",
            Self::Qpu => "qpu",
            Self::HybridCpuQpu => "hybrid_cpu_qpu",
            Self::HybridAcceleratorQpu => "hybrid_accelerator_qpu",
        }
    }

    /// Whether this target requires QPU metadata.
    pub const fn requires_qpu(self) -> bool {
        matches!(
            self,
            Self::Qpu
                | Self::HybridCpuQpu
                | Self::HybridAcceleratorQpu
        )
    }

    /// Whether two targets can directly share the same executable artifact.
    pub const fn compatible_with(self, other: Self) -> bool {
        if self == other {
            return true;
        }

        matches!(
            (self, other),
            (
                Self::ClassicalCpu,
                Self::ParallelCpu
            )
                | (
                    Self::ParallelCpu,
                    Self::ClassicalCpu
                )
                | (
                    Self::Simulator,
                    Self::Emulator
                )
                | (
                    Self::Emulator,
                    Self::Simulator
                )
        )
    }
}

impl fmt::Display for ExecutionTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Compatibility policy
// ============================================================================

/// Explicit compatibility policy.
///
/// Compatibility is never inferred implicitly.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CompatibilityPolicy {
    /// Exact version required.
    Exact,

    /// Same major version, with an artifact that is not newer than the
    /// supported implementation.
    SameMajor,

    /// Older versions within the same major family may be read.
    BackwardCompatible,

    /// Older compatible artifacts may be read but must not be written back
    /// using their old schema.
    ReadOnlyLegacy,

    /// Artifact is explicitly rejected.
    Reject,
}

/// Relationship between two versions/artifacts.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Compatibility {
    Exact,
    Compatible,
    LegacyCompatible,
    Incompatible,
    NewerThanSupported,
    TargetMismatch,
}

impl Compatibility {
    pub const fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::Exact
                | Self::Compatible
                | Self::LegacyCompatible
        )
    }

    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }
}

// ============================================================================
// Version manifest
// ============================================================================

/// Complete QEC version manifest.
///
/// A manifest should accompany persistent state and execution artifacts.
///
/// This allows a checkpoint, graph, syndrome stream, decoder result or QPU
/// job to be rejected before its payload is interpreted under the wrong
/// schema.
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
    pub noise_model: Version,
    pub decoder_result: Version,
    pub backend: Version,
    pub capability: Version,
    pub qpu_interface: Version,
    pub qpu_execution: Version,

    pub target: ExecutionTarget,
}

impl VersionManifest {
    /// Construct a manifest for the currently supported implementation.
    pub const fn current(target: ExecutionTarget) -> Self {
        Self {
            protocol_major: VERSION_PROTOCOL_MAJOR,
            protocol_minor: VERSION_PROTOCOL_MINOR,
            algorithm: CURRENT_ALGORITHM_VERSION,
            configuration: CURRENT_CONFIGURATION_VERSION,
            checkpoint: CURRENT_CHECKPOINT_VERSION,
            syndrome: CURRENT_SYNDROME_VERSION,
            graph: CURRENT_GRAPH_VERSION,
            simulation: CURRENT_SIMULATION_VERSION,
            noise_model: CURRENT_NOISE_MODEL_VERSION,
            decoder_result: CURRENT_DECODER_RESULT_VERSION,
            backend: CURRENT_BACKEND_VERSION,
            capability: CURRENT_CAPABILITY_VERSION,
            qpu_interface: CURRENT_QPU_INTERFACE_VERSION,
            qpu_execution: CURRENT_QPU_EXECUTION_VERSION,
            target,
        }
    }

    /// Validate protocol metadata.
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

        if self.protocol_minor > VERSION_PROTOCOL_MINOR {
            return Err(VersionError::NewerProtocolMinor {
                supported: VERSION_PROTOCOL_MINOR,
                found: self.protocol_minor,
            });
        }

        if self.target.requires_qpu()
            && self.qpu_interface.major == 0
        {
            return Err(VersionError::InvalidQpuVersion);
        }

        Ok(())
    }

    /// Return the version belonging to an artifact.
    pub const fn version_of(&self, kind: ArtifactKind) -> Version {
        match kind {
            ArtifactKind::Algorithm => self.algorithm,
            ArtifactKind::Configuration => self.configuration,
            ArtifactKind::Checkpoint => self.checkpoint,
            ArtifactKind::Syndrome => self.syndrome,
            ArtifactKind::DecodingGraph => self.graph,
            ArtifactKind::Simulation => self.simulation,
            ArtifactKind::NoiseModel => self.noise_model,
            ArtifactKind::DecoderResult => self.decoder_result,
            ArtifactKind::Backend => self.backend,
            ArtifactKind::Capability => self.capability,
            ArtifactKind::QpuInterface => self.qpu_interface,
            ArtifactKind::QpuExecution => self.qpu_execution,
        }
    }

    /// Determine compatibility of an artifact with the currently supported
    /// implementation.
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
                } else if found > current {
                    Compatibility::NewerThanSupported
                } else {
                    Compatibility::Incompatible
                }
            }

            CompatibilityPolicy::SameMajor => {
                if found == current {
                    Compatibility::Exact
                } else if found > current {
                    Compatibility::NewerThanSupported
                } else if found.major == current.major {
                    Compatibility::Compatible
                } else {
                    Compatibility::Incompatible
                }
            }

            CompatibilityPolicy::BackwardCompatible => {
                if found == current {
                    Compatibility::Exact
                } else if found > current {
                    Compatibility::NewerThanSupported
                } else if found.major == current.major {
                    Compatibility::LegacyCompatible
                } else {
                    Compatibility::Incompatible
                }
            }

            CompatibilityPolicy::ReadOnlyLegacy => {
                if found == current {
                    Compatibility::Exact
                } else if found < current
                    && found.major == current.major
                {
                    Compatibility::LegacyCompatible
                } else if found > current {
                    Compatibility::NewerThanSupported
                } else {
                    Compatibility::Incompatible
                }
            }

            CompatibilityPolicy::Reject => Compatibility::Incompatible,
        }
    }

    /// Compare this manifest against another manifest.
    ///
    /// This is important for distributed execution, checkpoints, replay and
    /// cache validation because compatibility is not always a comparison
    /// against the locally installed "current" version.
    pub fn is_compatible_with(
        &self,
        other: &Self,
        policy: CompatibilityPolicy,
    ) -> Result<(), VersionError> {
        self.validate()?;
        other.validate()?;

        if !self.target.compatible_with(other.target) {
            return Err(VersionError::TargetMismatch {
                expected: self.target,
                found: other.target,
            });
        }

        let kinds = [
            ArtifactKind::Algorithm,
            ArtifactKind::Configuration,
            ArtifactKind::Checkpoint,
            ArtifactKind::Syndrome,
            ArtifactKind::DecodingGraph,
            ArtifactKind::Simulation,
            ArtifactKind::NoiseModel,
            ArtifactKind::DecoderResult,
            ArtifactKind::Backend,
            ArtifactKind::Capability,
            ArtifactKind::QpuInterface,
            ArtifactKind::QpuExecution,
        ];

        for kind in kinds {
            let left = self.version_of(kind);
            let right = other.version_of(kind);

            match policy {
                CompatibilityPolicy::Exact => {
                    if left != right {
                        return Err(VersionError::ManifestMismatch {
                            artifact: kind,
                            expected: left,
                            found: right,
                        });
                    }
                }

                CompatibilityPolicy::SameMajor
                | CompatibilityPolicy::BackwardCompatible
                | CompatibilityPolicy::ReadOnlyLegacy => {
                    if left.major != right.major {
                        return Err(VersionError::ManifestMismatch {
                            artifact: kind,
                            expected: left,
                            found: right,
                        });
                    }

                    if right > left {
                        return Err(VersionError::NewerArtifact {
                            kind,
                            found: right,
                            supported: left,
                        });
                    }
                }

                CompatibilityPolicy::Reject => {
                    return Err(VersionError::Rejected);
                }
            }
        }

        Ok(())
    }

    /// Require compatibility with the current implementation.
    pub fn require_compatible(
        &self,
        kind: ArtifactKind,
        policy: CompatibilityPolicy,
    ) -> Result<Compatibility, VersionError> {
        self.validate()?;

        let compatibility = self.compatibility(kind, policy);

        if !compatibility.is_compatible() {
            return Err(match compatibility {
                Compatibility::NewerThanSupported => {
                    VersionError::NewerArtifact {
                        kind,
                        found: self.version_of(kind),
                        supported: kind.current_version(),
                    }
                }

                Compatibility::TargetMismatch => {
                    VersionError::TargetMismatch {
                        expected: self.target,
                        found: self.target,
                    }
                }

                _ => VersionError::IncompatibleArtifact {
                    kind,
                    found: self.version_of(kind),
                    expected: kind.current_version(),
                },
            });
        }

        Ok(compatibility)
    }

    /// Determine whether this manifest is entirely current.
    pub fn is_current(&self) -> bool {
        self.protocol_major == VERSION_PROTOCOL_MAJOR
            && self.protocol_minor == VERSION_PROTOCOL_MINOR
            && self.algorithm == CURRENT_ALGORITHM_VERSION
            && self.configuration == CURRENT_CONFIGURATION_VERSION
            && self.checkpoint == CURRENT_CHECKPOINT_VERSION
            && self.syndrome == CURRENT_SYNDROME_VERSION
            && self.graph == CURRENT_GRAPH_VERSION
            && self.simulation == CURRENT_SIMULATION_VERSION
            && self.noise_model == CURRENT_NOISE_MODEL_VERSION
            && self.decoder_result == CURRENT_DECODER_RESULT_VERSION
            && self.backend == CURRENT_BACKEND_VERSION
            && self.capability == CURRENT_CAPABILITY_VERSION
            && self.qpu_interface == CURRENT_QPU_INTERFACE_VERSION
            && self.qpu_execution == CURRENT_QPU_EXECUTION_VERSION
    }
}

impl Default for VersionManifest {
    fn default() -> Self {
        Self::current(ExecutionTarget::ClassicalCpu)
    }
}

// ============================================================================
// Component version
// ============================================================================

/// Version identity for a concrete algorithm, backend, decoder, noise model
/// or QPU provider.
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

        validate_identifier(
            &artifact_id,
            "artifact_id",
        )?;

        Ok(Self {
            artifact,
            artifact_id,
            version,
        })
    }

    /// Stable component identity.
    pub fn identity(&self) -> String {
        format!(
            "{}:{}@{}",
            self.artifact,
            self.artifact_id,
            self.version
        )
    }

    /// Validate this component against another component.
    pub fn compatible_with(
        &self,
        other: &Self,
        policy: CompatibilityPolicy,
    ) -> Result<Compatibility, VersionError> {
        if self.artifact != other.artifact {
            return Err(VersionError::ArtifactKindMismatch {
                expected: self.artifact,
                found: other.artifact,
            });
        }

        if self.artifact_id != other.artifact_id {
            return Err(VersionError::ComponentIdentityMismatch {
                expected: self.artifact_id.clone(),
                found: other.artifact_id.clone(),
            });
        }

        let left = self.version;
        let right = other.version;

        let result = match policy {
            CompatibilityPolicy::Exact => {
                if left == right {
                    Compatibility::Exact
                } else if right > left {
                    Compatibility::NewerThanSupported
                } else {
                    Compatibility::Incompatible
                }
            }

            CompatibilityPolicy::SameMajor => {
                if left == right {
                    Compatibility::Exact
                } else if right > left {
                    Compatibility::NewerThanSupported
                } else if left.major == right.major {
                    Compatibility::Compatible
                } else {
                    Compatibility::Incompatible
                }
            }

            CompatibilityPolicy::BackwardCompatible
            | CompatibilityPolicy::ReadOnlyLegacy => {
                if left == right {
                    Compatibility::Exact
                } else if right > left {
                    Compatibility::NewerThanSupported
                } else if left.major == right.major {
                    Compatibility::LegacyCompatible
                } else {
                    Compatibility::Incompatible
                }
            }

            CompatibilityPolicy::Reject => Compatibility::Incompatible,
        };

        if result.is_compatible() {
            Ok(result)
        } else {
            Err(VersionError::ComponentVersionMismatch {
                artifact: self.artifact,
                expected: left,
                found: right,
            })
        }
    }
}

// ============================================================================
// QPU version
// ============================================================================

/// Version information for a physical or virtual QPU.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QpuVersion {
    pub provider: String,
    pub device_family: String,
    pub device_id: Option<String>,

    /// QEC-facing QPU interface version.
    pub interface_version: Version,

    /// Device firmware version.
    pub firmware_version: Version,

    /// Classical control-stack version.
    pub control_stack_version: Version,

    /// Calibration schema/version.
    pub calibration_version: Version,

    /// Whether dynamic circuits are supported.
    pub dynamic_circuits: bool,

    /// Whether hybrid CPU/QPU execution is supported.
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
        validate_identifier(
            &device_family,
            "device_family",
        )?;

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

        validate_identifier(
            &device_id,
            "device_id",
        )?;

        self.device_id = Some(device_id);

        Ok(self)
    }

    pub fn with_dynamic_circuits(
        mut self,
        supported: bool,
    ) -> Self {
        self.dynamic_circuits = supported;
        self
    }

    pub fn with_hybrid_execution(
        mut self,
        supported: bool,
    ) -> Self {
        self.hybrid_execution = supported;
        self
    }

    /// Check whether this QPU can satisfy an interface requirement.
    pub fn supports_interface(
        &self,
        required: Version,
    ) -> bool {
        self.interface_version.major == required.major
            && self.interface_version >= required
    }

    /// Check whether the QPU can participate in the requested execution
    /// target.
    pub fn supports_target(
        &self,
        target: ExecutionTarget,
    ) -> bool {
        match target {
            ExecutionTarget::Qpu => true,

            ExecutionTarget::HybridCpuQpu
            | ExecutionTarget::HybridAcceleratorQpu => {
                self.hybrid_execution
            }

            _ => true,
        }
    }

    /// Stable non-secret QPU identity.
    pub fn identity(&self) -> String {
        match &self.device_id {
            Some(id) => {
                format!(
                    "{}:{}:{}",
                    self.provider,
                    self.device_family,
                    id
                )
            }

            None => {
                format!(
                    "{}:{}",
                    self.provider,
                    self.device_family
                )
            }
        }
    }

    /// Validate QPU metadata.
    pub fn validate(&self) -> Result<(), VersionError> {
        validate_identifier(
            &self.provider,
            "provider",
        )?;

        validate_identifier(
            &self.device_family,
            "device_family",
        )?;

        if let Some(id) = &self.device_id {
            validate_identifier(id, "device_id")?;
        }

        if self.interface_version.major == 0 {
            return Err(VersionError::InvalidQpuVersion);
        }

        Ok(())
    }
}

// ============================================================================
// Execution version
// ============================================================================

/// Version information for a concrete execution environment.
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
        let component = ComponentVersion::new(
            ArtifactKind::Backend,
            component_id,
            version,
        )?;

        Ok(Self {
            target: ExecutionTarget::ClassicalCpu,
            component,
            manifest: VersionManifest::current(
                ExecutionTarget::ClassicalCpu,
            ),
            qpu: None,
        })
    }

    pub fn simulator(
        component_id: impl Into<String>,
        version: Version,
    ) -> Result<Self, VersionError> {
        let component = ComponentVersion::new(
            ArtifactKind::Backend,
            component_id,
            version,
        )?;

        Ok(Self {
            target: ExecutionTarget::Simulator,
            component,
            manifest: VersionManifest::current(
                ExecutionTarget::Simulator,
            ),
            qpu: None,
        })
    }

    pub fn qpu(
        component_id: impl Into<String>,
        version: Version,
        qpu: QpuVersion,
    ) -> Result<Self, VersionError> {
        qpu.validate()?;

        if !qpu.supports_interface(
            CURRENT_QPU_INTERFACE_VERSION,
        ) {
            return Err(
                VersionError::QpuInterfaceMismatch {
                    required: CURRENT_QPU_INTERFACE_VERSION,
                    found: qpu.interface_version,
                },
            );
        }

        let component = ComponentVersion::new(
            ArtifactKind::Backend,
            component_id,
            version,
        )?;

        Ok(Self {
            target: ExecutionTarget::Qpu,
            component,
            manifest: VersionManifest::current(
                ExecutionTarget::Qpu,
            ),
            qpu: Some(qpu),
        })
    }

    pub fn hybrid_qpu(
        component_id: impl Into<String>,
        version: Version,
        qpu: QpuVersion,
    ) -> Result<Self, VersionError> {
        qpu.validate()?;

        if !qpu.hybrid_execution {
            return Err(
                VersionError::QpuTargetUnsupported {
                    target: ExecutionTarget::HybridCpuQpu,
                },
            );
        }

        let component = ComponentVersion::new(
            ArtifactKind::Backend,
            component_id,
            version,
        )?;

        Ok(Self {
            target: ExecutionTarget::HybridCpuQpu,
            component,
            manifest: VersionManifest::current(
                ExecutionTarget::HybridCpuQpu,
            ),
            qpu: Some(qpu),
        })
    }

    pub fn validate(&self) -> Result<(), VersionError> {
        self.manifest.validate()?;

        if self.manifest.target != self.target {
            return Err(
                VersionError::ExecutionTargetMismatch {
                    expected: self.target,
                    found: self.manifest.target,
                },
            );
        }

        if self.target.requires_qpu() {
            let qpu = self
                .qpu
                .as_ref()
                .ok_or(VersionError::MissingQpuMetadata)?;

            qpu.validate()?;

            if !qpu.supports_target(self.target) {
                return Err(
                    VersionError::QpuTargetUnsupported {
                        target: self.target,
                    },
                );
            }
        } else if self.qpu.is_some() {
            return Err(
                VersionError::UnexpectedQpuMetadata,
            );
        }

        Ok(())
    }
}

// ============================================================================
// Artifact header
// ============================================================================

/// Persistent artifact header.
///
/// This must be validated before the payload is interpreted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArtifactHeader {
    pub magic: [u8; 4],
    pub protocol_major: u16,
    pub protocol_minor: u16,
    pub artifact: ArtifactKind,
    pub version: Version,
    pub target: ExecutionTarget,
}

impl ArtifactHeader {
    /// Magic value for Zamani QEC artifacts.
    pub const MAGIC: [u8; 4] = *b"ZQEC";

    pub const fn new(
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

    /// Construct a header using the currently supported version.
    pub const fn current(
        artifact: ArtifactKind,
        target: ExecutionTarget,
    ) -> Self {
        Self::new(
            artifact,
            artifact.current_version(),
            target,
        )
    }

    /// Validate the header itself.
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

        if self.protocol_minor > VERSION_PROTOCOL_MINOR {
            return Err(VersionError::NewerProtocolMinor {
                supported: VERSION_PROTOCOL_MINOR,
                found: self.protocol_minor,
            });
        }

        if self.version.major == 0 {
            return Err(VersionError::InvalidArtifactVersion {
                kind: self.artifact,
            });
        }

        Ok(())
    }

    /// Whether the artifact uses the current schema.
    pub fn is_current(&self) -> bool {
        self.version == self.artifact.current_version()
    }

    /// Convert the header into a one-artifact manifest.
    pub const fn manifest(&self) -> VersionManifest {
        VersionManifest::current(self.target)
    }
}

// ============================================================================
// Upgrade policy
// ============================================================================

/// Policy for persisted-artifact upgrades.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpgradePolicy {
    /// Require exact current versions.
    Strict,

    /// Permit explicitly compatible older versions.
    Compatible,

    /// Permit compatible older versions and require migration before write.
    MigrateOnRead,

    /// Permit legacy reads but prohibit writing legacy format.
    ReadLegacyOnly,
}

impl UpgradePolicy {
    pub const fn compatibility_policy(
        self,
    ) -> CompatibilityPolicy {
        match self {
            Self::Strict => CompatibilityPolicy::Exact,
            Self::Compatible => {
                CompatibilityPolicy::BackwardCompatible
            }
            Self::MigrateOnRead => {
                CompatibilityPolicy::BackwardCompatible
            }
            Self::ReadLegacyOnly => {
                CompatibilityPolicy::ReadOnlyLegacy
            }
        }
    }

    pub const fn permits_migration(self) -> bool {
        matches!(self, Self::MigrateOnRead)
    }

    /// Whether this policy permits writing a legacy artifact.
    pub const fn permits_legacy_write(self) -> bool {
        false
    }
}

// ============================================================================
// Validation status
// ============================================================================

/// Result of validating an artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationStatus {
    Current,
    Compatible,
    Legacy,
}

impl ValidationStatus {
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::Legacy)
    }
}

// ============================================================================
// Artifact validation
// ============================================================================

/// Validate a persistent artifact header.
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

        CompatibilityPolicy::SameMajor
        | CompatibilityPolicy::BackwardCompatible
        | CompatibilityPolicy::ReadOnlyLegacy => {
            if header.version == current {
                Ok(ValidationStatus::Current)
            } else if header.version > current {
                Err(VersionError::NewerArtifact {
                    kind: header.artifact,
                    found: header.version,
                    supported: current,
                })
            } else if header.version.major == current.major {
                Ok(ValidationStatus::Legacy)
            } else {
                Err(VersionError::IncompatibleArtifact {
                    kind: header.artifact,
                    found: header.version,
                    expected: current,
                })
            }
        }

        CompatibilityPolicy::Reject => {
            Err(VersionError::Rejected)
        }
    }
}

// ============================================================================
// QPU compatibility
// ============================================================================

/// Require a specific QPU interface version.
pub fn require_qpu_interface(
    qpu: &QpuVersion,
    required: Version,
) -> Result<(), VersionError> {
    qpu.validate()?;

    if !qpu.supports_interface(required) {
        return Err(
            VersionError::QpuInterfaceMismatch {
                required,
                found: qpu.interface_version,
            },
        );
    }

    Ok(())
}

/// Validate QPU compatibility with an execution target.
pub fn validate_qpu_target(
    qpu: &QpuVersion,
    target: ExecutionTarget,
) -> Result<(), VersionError> {
    qpu.validate()?;

    if !target.requires_qpu() {
        return Err(
            VersionError::QpuTargetUnsupported {
                target,
            },
        );
    }

    if !qpu.supports_target(target) {
        return Err(
            VersionError::QpuTargetUnsupported {
                target,
            },
        );
    }

    Ok(())
}

// ============================================================================
// Identifier validation
// ============================================================================

fn validate_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), VersionError> {
    if value.is_empty() {
        return Err(VersionError::EmptyIdentifier(field));
    }

    if value.len() > MAX_IDENTIFIER_LENGTH {
        return Err(VersionError::IdentifierTooLong(field));
    }

    if !value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'-' | b'_' | b'.' | b':'
            )
    }) {
        return Err(
            VersionError::InvalidIdentifier(field)
        );
    }

    Ok(())
}

// ============================================================================
// Version errors
// ============================================================================

/// Errors generated by the versioning subsystem.
///
/// These errors intentionally remain independent from `QecError` so malformed
/// version metadata can be rejected before the artifact is trusted.
///
/// High-level QEC APIs may convert these into `QecError::InvalidInput` or
/// `QecError::UnsupportedConfiguration` at their public boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VersionError {
    Empty,
    TooLong,

    InvalidFormat,

    MissingComponent(&'static str),

    InvalidComponent(&'static str),

    ComponentOverflow(&'static str),

    InvalidProtocolVersion,

    ProtocolMismatch {
        expected_major: u16,
        found_major: u16,
    },

    NewerProtocolMinor {
        supported: u16,
        found: u16,
    },

    EmptyArtifactId,

    ArtifactIdTooLong,

    InvalidArtifactId,

    EmptyIdentifier(&'static str),

    IdentifierTooLong(&'static str),

    InvalidIdentifier(&'static str),

    InvalidMagic,

    InvalidArtifactVersion {
        kind: ArtifactKind,
    },

    InvalidQpuVersion,

    MissingQpuMetadata,

    UnexpectedQpuMetadata,

    QpuInterfaceMismatch {
        required: Version,
        found: Version,
    },

    QpuTargetUnsupported {
        target: ExecutionTarget,
    },

    TargetMismatch {
        expected: ExecutionTarget,
        found: ExecutionTarget,
    },

    ExecutionTargetMismatch {
        expected: ExecutionTarget,
        found: ExecutionTarget,
    },

    ArtifactKindMismatch {
        expected: ArtifactKind,
        found: ArtifactKind,
    },

    ComponentIdentityMismatch {
        expected: String,
        found: String,
    },

    ComponentVersionMismatch {
        artifact: ArtifactKind,
        expected: Version,
        found: Version,
    },

    ManifestMismatch {
        artifact: ArtifactKind,
        expected: Version,
        found: Version,
    },

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

    Rejected,
}

impl fmt::Display for VersionError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Empty => {
                f.write_str(
                    "version string is empty",
                )
            }

            Self::TooLong => {
                f.write_str(
                    "version string exceeds maximum length",
                )
            }

            Self::InvalidFormat => {
                f.write_str(
                    "version must use MAJOR.MINOR.PATCH format",
                )
            }

            Self::MissingComponent(name) => {
                write!(
                    f,
                    "missing {} version component",
                    name
                )
            }

            Self::InvalidComponent(name) => {
                write!(
                    f,
                    "invalid {} version component",
                    name
                )
            }

            Self::ComponentOverflow(name) => {
                write!(
                    f,
                    "{} version component exceeds u16",
                    name
                )
            }

            Self::InvalidProtocolVersion => {
                f.write_str(
                    "invalid QEC protocol version",
                )
            }

            Self::ProtocolMismatch {
                expected_major,
                found_major,
            } => {
                write!(
                    f,
                    "QEC protocol major mismatch: expected {}, found {}",
                    expected_major,
                    found_major
                )
            }

            Self::NewerProtocolMinor {
                supported,
                found,
            } => {
                write!(
                    f,
                    "QEC protocol minor {} is newer than supported {}",
                    found,
                    supported
                )
            }

            Self::EmptyArtifactId => {
                f.write_str(
                    "artifact identifier is empty",
                )
            }

            Self::ArtifactIdTooLong => {
                f.write_str(
                    "artifact identifier is too long",
                )
            }

            Self::InvalidArtifactId => {
                f.write_str(
                    "artifact identifier contains invalid characters",
                )
            }

            Self::EmptyIdentifier(field) => {
                write!(
                    f,
                    "{} identifier is empty",
                    field
                )
            }

            Self::IdentifierTooLong(field) => {
                write!(
                    f,
                    "{} identifier is too long",
                    field
                )
            }

            Self::InvalidIdentifier(field) => {
                write!(
                    f,
                    "{} identifier contains invalid characters",
                    field
                )
            }

            Self::InvalidMagic => {
                f.write_str(
                    "invalid QEC artifact magic",
                )
            }

            Self::InvalidArtifactVersion {
                kind,
            } => {
                write!(
                    f,
                    "invalid version for {} artifact",
                    kind
                )
            }

            Self::InvalidQpuVersion => {
                f.write_str(
                    "invalid QPU version metadata",
                )
            }

            Self::MissingQpuMetadata => {
                f.write_str(
                    "QPU execution target requires QPU metadata",
                )
            }

            Self::UnexpectedQpuMetadata => {
                f.write_str(
                    "non-QPU execution target contains QPU metadata",
                )
            }

            Self::QpuInterfaceMismatch {
                required,
                found,
            } => {
                write!(
                    f,
                    "QPU interface mismatch: required {}, found {}",
                    required,
                    found
                )
            }

            Self::QpuTargetUnsupported {
                target,
            } => {
                write!(
                    f,
                    "QPU does not support execution target {}",
                    target
                )
            }

            Self::TargetMismatch {
                expected,
                found,
            } => {
                write!(
                    f,
                    "execution target mismatch: expected {}, found {}",
                    expected,
                    found
                )
            }

            Self::ExecutionTargetMismatch {
                expected,
                found,
            } => {
                write!(
                    f,
                    "execution version target mismatch: expected {}, found {}",
                    expected,
                    found
                )
            }

            Self::ArtifactKindMismatch {
                expected,
                found,
            } => {
                write!(
                    f,
                    "artifact kind mismatch: expected {}, found {}",
                    expected,
                    found
                )
            }

            Self::ComponentIdentityMismatch {
                expected,
                found,
            } => {
                write!(
                    f,
                    "component identity mismatch: expected {}, found {}",
                    expected,
                    found
                )
            }

            Self::ComponentVersionMismatch {
                artifact,
                expected,
                found,
            } => {
                write!(
                    f,
                    "{} component version mismatch: expected {}, found {}",
                    artifact,
                    expected,
                    found
                )
            }

            Self::ManifestMismatch {
                artifact,
                expected,
                found,
            } => {
                write!(
                    f,
                    "{} manifest version mismatch: expected {}, found {}",
                    artifact,
                    expected,
                    found
                )
            }

            Self::IncompatibleArtifact {
                kind,
                found,
                expected,
            } => {
                write!(
                    f,
                    "incompatible {} artifact: found {}, expected {}",
                    kind,
                    found,
                    expected
                )
            }

            Self::NewerArtifact {
                kind,
                found,
                supported,
            } => {
                write!(
                    f,
                    "{} artifact version {} is newer than supported {}",
                    kind,
                    found,
                    supported
                )
            }

            Self::Rejected => {
                f.write_str(
                    "artifact rejected by compatibility policy",
                )
            }
        }
    }
}

impl std::error::Error for VersionError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_round_trip() {
        let version = Version::new(12, 34, 56);
        let text = version.to_string();

        assert_eq!(
            text.parse::<Version>().unwrap(),
            version
        );
    }

    #[test]
    fn malformed_version_is_rejected() {
        assert!("1".parse::<Version>().is_err());
        assert!("1.2".parse::<Version>().is_err());
        assert!("1.2.3.4".parse::<Version>().is_err());
        assert!("a.2.3".parse::<Version>().is_err());
        assert!("1.b.3".parse::<Version>().is_err());
        assert!("1.2.c".parse::<Version>().is_err());
    }

    #[test]
    fn version_ordering_is_deterministic() {
        assert!(
            Version::new(1, 0, 0)
                < Version::new(1, 1, 0)
        );

        assert!(
            Version::new(1, 1, 0)
                < Version::new(1, 1, 1)
        );

        assert!(
            Version::new(2, 0, 0)
                > Version::new(1, 99, 99)
        );
    }

    #[test]
    fn artifact_versions_are_registered() {
        assert_eq!(
            ArtifactKind::Algorithm.current_version(),
            CURRENT_ALGORITHM_VERSION
        );

        assert_eq!(
            ArtifactKind::NoiseModel.current_version(),
            CURRENT_NOISE_MODEL_VERSION
        );

        assert_eq!(
            ArtifactKind::DecoderResult.current_version(),
            CURRENT_DECODER_RESULT_VERSION
        );

        assert_eq!(
            ArtifactKind::QpuExecution.current_version(),
            CURRENT_QPU_EXECUTION_VERSION
        );
    }

    #[test]
    fn current_manifest_is_current() {
        let manifest =
            VersionManifest::current(
                ExecutionTarget::ClassicalCpu,
            );

        assert!(manifest.validate().is_ok());
        assert!(manifest.is_current());
    }

    #[test]
    fn future_protocol_is_rejected() {
        let mut manifest =
            VersionManifest::current(
                ExecutionTarget::ClassicalCpu,
            );

        manifest.protocol_major += 1;

        assert!(matches!(
            manifest.validate(),
            Err(VersionError::ProtocolMismatch { .. })
        ));
    }

    #[test]
    fn future_minor_protocol_is_rejected() {
        let mut manifest =
            VersionManifest::current(
                ExecutionTarget::ClassicalCpu,
            );

        manifest.protocol_minor += 1;

        assert!(matches!(
            manifest.validate(),
            Err(VersionError::NewerProtocolMinor { .. })
        ));
    }

    #[test]
    fn artifact_header_is_validated_before_payload() {
        let header = ArtifactHeader::current(
            ArtifactKind::Checkpoint,
            ExecutionTarget::ClassicalCpu,
        );

        assert!(header.validate().is_ok());
        assert!(header.is_current());
    }

    #[test]
    fn invalid_magic_is_rejected() {
        let mut header = ArtifactHeader::current(
            ArtifactKind::Checkpoint,
            ExecutionTarget::ClassicalCpu,
        );

        header.magic = *b"NOPE";

        assert!(matches!(
            header.validate(),
            Err(VersionError::InvalidMagic)
        ));
    }

    #[test]
    fn strict_artifact_validation_requires_exact_version() {
        let mut header = ArtifactHeader::current(
            ArtifactKind::Checkpoint,
            ExecutionTarget::ClassicalCpu,
        );

        header.version = Version::new(1, 0, 0);

        assert!(validate_artifact(
            &header,
            UpgradePolicy::Strict
        )
        .is_err());
    }

    #[test]
    fn compatible_legacy_artifact_is_detected() {
        let mut header = ArtifactHeader::current(
            ArtifactKind::Checkpoint,
            ExecutionTarget::ClassicalCpu,
        );

        header.version = Version::new(
            CURRENT_CHECKPOINT_VERSION.major,
            0,
            1,
        );

        let result = validate_artifact(
            &header,
            UpgradePolicy::Compatible,
        );

        assert_eq!(
            result.unwrap(),
            ValidationStatus::Legacy
        );
    }

    #[test]
    fn newer_artifact_is_never_silently_accepted() {
        let mut header = ArtifactHeader::current(
            ArtifactKind::Checkpoint,
            ExecutionTarget::ClassicalCpu,
        );

        header.version = Version::new(
            CURRENT_CHECKPOINT_VERSION.major + 1,
            0,
            0,
        );

        assert!(matches!(
            validate_artifact(
                &header,
                UpgradePolicy::Compatible
            ),
            Err(VersionError::NewerArtifact { .. })
        ));
    }

    #[test]
    fn component_identity_is_stable() {
        let component = ComponentVersion::new(
            ArtifactKind::Algorithm,
            "mwpm",
            Version::new(2, 1, 0),
        )
        .unwrap();

        assert_eq!(
            component.identity(),
            "algorithm:mwpm@2.1.0"
        );
    }

    #[test]
    fn component_identity_is_validated() {
        assert!(
            ComponentVersion::new(
                ArtifactKind::Algorithm,
                "mwpm decoder",
                Version::new(1, 0, 0)
            )
            .is_err()
        );
    }

    #[test]
    fn manifest_comparison_detects_mismatch() {
        let a = VersionManifest::current(
            ExecutionTarget::ClassicalCpu,
        );

        let mut b = a.clone();
        b.decoder_result =
            Version::new(2, 0, 0);

        assert!(
            a.is_compatible_with(
                &b,
                CompatibilityPolicy::Exact
            )
            .is_err()
        );
    }

    #[test]
    fn incompatible_targets_are_rejected() {
        let cpu = VersionManifest::current(
            ExecutionTarget::ClassicalCpu,
        );

        let qpu = VersionManifest::current(
            ExecutionTarget::Qpu,
        );

        assert!(
            cpu.is_compatible_with(
                &qpu,
                CompatibilityPolicy::Exact
            )
            .is_err()
        );
    }

    #[test]
    fn qpu_metadata_is_required_for_qpu_execution() {
        let execution = ExecutionVersion {
            target: ExecutionTarget::Qpu,
            component: ComponentVersion::new(
                ArtifactKind::Backend,
                "test-qpu",
                Version::new(1, 0, 0),
            )
            .unwrap(),
            manifest: VersionManifest::current(
                ExecutionTarget::Qpu,
            ),
            qpu: None,
        };

        assert!(matches!(
            execution.validate(),
            Err(VersionError::MissingQpuMetadata)
        ));
    }

    #[test]
    fn qpu_interface_mismatch_is_rejected() {
        let qpu = QpuVersion::new(
            "test",
            "simulator",
            Version::new(1, 0, 0),
            Version::new(1, 0, 0),
            Version::new(1, 0, 0),
            Version::new(1, 0, 0),
        )
        .unwrap();

        assert!(
            require_qpu_interface(
                &qpu,
                Version::new(2, 0, 0)
            )
            .is_err()
        );
    }

    #[test]
    fn hybrid_qpu_requires_hybrid_support() {
        let qpu = QpuVersion::new(
            "test",
            "qpu",
            Version::new(2, 0, 0),
            Version::new(1, 0, 0),
            Version::new(1, 0, 0),
            Version::new(1, 0, 0),
        )
        .unwrap();

        assert!(
            ExecutionVersion::hybrid_qpu(
                "backend",
                Version::new(1, 0, 0),
                qpu
            )
            .is_err()
        );
    }

    #[test]
    fn qpu_identity_contains_no_secret_metadata() {
        let qpu = QpuVersion::new(
            "provider",
            "family",
            Version::new(2, 0, 0),
            Version::new(1, 0, 0),
            Version::new(1, 0, 0),
            Version::new(1, 0, 0),
        )
        .unwrap();

        assert_eq!(
            qpu.identity(),
            "provider:family"
        );
    }
}