//! Zamani Quantum Error Correction — Canonical Versioning
//!
//! This module is the foundational version/compatibility contract for the
//! entire QEC subsystem.
//!
//! # Architectural position
//!
//! `version.rs` is intentionally independent of all higher-level QEC modules.
//! It may therefore be used by:
//!
//! - errors.rs
//! - configuration.rs
//! - checkpoint.rs
//! - cache.rs
//! - replay.rs
//! - syndrome.rs
//! - decoding_graph.rs
//! - decoder_result.rs
//! - backend.rs
//! - capabilities.rs
//! - distributed.rs
//! - QPU integration
//!
//! It MUST NOT depend on those modules.
//!
//! # Compatibility rule
//!
//! Version metadata is security-sensitive input. Never interpret an external
//! artifact before validating its version metadata.
//!
//! ```text
//! untrusted artifact
//!        |
//!        v
//! ArtifactHeader
//!        |
//!        v
//! protocol validation
//!        |
//!        v
//! artifact-kind validation
//!        |
//!        v
//! version compatibility
//!        |
//!        v
//! feature compatibility
//!        |
//!        v
//! execution-target compatibility
//!        |
//!        v
//! trusted artifact
//! ```
//!
//! A newer artifact is never silently interpreted as an older artifact.
//! A major-version mismatch is incompatible unless an explicit migration
//! mechanism exists outside this module.
//!
//! # Integration contract
//!
//! Higher-level modules should use:
//!
//! - [`Version::current`] for the subsystem version.
//! - [`ArtifactKind::current_version`] for artifact schemas.
//! - [`VersionManifest::current`] when creating a complete manifest.
//! - [`ArtifactHeader`] for persisted/transmitted artifact metadata.
//! - [`check_compatibility`] before interpreting an artifact.
//! - [`require_compatible`] when incompatibility must fail immediately.
//!
//! `errors.rs` should convert [`VersionError`] into the canonical `QecError`.
//!
//! # Rust compatibility
//!
//! This implementation intentionally uses no external dependencies and is
//! suitable for Rust 1.97.1.

use core::cmp::Ordering;
use core::fmt;
use core::str::FromStr;

// ============================================================================
// Protocol identity
// ============================================================================

/// Versioning protocol major version.
///
/// A change here means the interpretation rules of version metadata changed.
pub const VERSION_PROTOCOL_MAJOR: u16 = 3;

/// Versioning protocol minor version.
///
/// Minor protocol changes must remain backwards-readable.
pub const VERSION_PROTOCOL_MINOR: u16 = 0;

/// Maximum accepted textual version length.
pub const MAX_VERSION_STRING_LENGTH: usize = 64;

/// Maximum artifact identifier length.
pub const MAX_ARTIFACT_ID_LENGTH: usize = 128;

/// Maximum provider/device identifier length.
pub const MAX_IDENTIFIER_LENGTH: usize = 128;

/// Maximum feature-name length.
pub const MAX_FEATURE_NAME_LENGTH: usize = 128;

/// Maximum number of feature bits represented by [`FeatureFlags`].
pub const FEATURE_FLAG_BITS: usize = 64;

// ============================================================================
// Current QEC versions
// ============================================================================

/// Current QEC algorithm contract.
pub const CURRENT_ALGORITHM_VERSION: Version = Version::new(3, 0, 0);

/// Current QEC configuration schema.
pub const CURRENT_CONFIGURATION_VERSION: Version = Version::new(3, 0, 0);

/// Current checkpoint schema.
pub const CURRENT_CHECKPOINT_VERSION: Version = Version::new(3, 0, 0);

/// Current syndrome representation.
pub const CURRENT_SYNDROME_VERSION: Version = Version::new(3, 0, 0);

/// Current decoding-graph representation.
pub const CURRENT_GRAPH_VERSION: Version = Version::new(3, 0, 0);

/// Current simulation artifact representation.
pub const CURRENT_SIMULATION_VERSION: Version = Version::new(3, 0, 0);

/// Current noise-model representation.
pub const CURRENT_NOISE_MODEL_VERSION: Version = Version::new(2, 0, 0);

/// Current decoder-result representation.
pub const CURRENT_DECODER_RESULT_VERSION: Version = Version::new(2, 0, 0);

/// Current execution-backend representation.
pub const CURRENT_BACKEND_VERSION: Version = Version::new(3, 0, 0);

/// Current capability representation.
pub const CURRENT_CAPABILITY_VERSION: Version = Version::new(3, 0, 0);

/// Current QPU interface contract.
pub const CURRENT_QPU_INTERFACE_VERSION: Version = Version::new(2, 0, 0);

/// Current QPU execution schema.
pub const CURRENT_QPU_EXECUTION_VERSION: Version = Version::new(2, 0, 0);

/// Current decoder output contract.
///
/// Kept as an explicit alias for callers that distinguish decoder output from
/// the persisted decoder-result schema.
pub const CURRENT_DECODER_OUTPUT_VERSION: Version =
    CURRENT_DECODER_RESULT_VERSION;

/// Current replay artifact schema.
pub const CURRENT_REPLAY_VERSION: Version = Version::new(1, 0, 0);

/// Current cache artifact schema.
pub const CURRENT_CACHE_VERSION: Version = Version::new(1, 0, 0);

/// Current partition artifact schema.
pub const CURRENT_PARTITION_VERSION: Version = Version::new(1, 0, 0);

/// Current distributed execution schema.
pub const CURRENT_DISTRIBUTED_VERSION: Version = Version::new(1, 0, 0);

/// Current streaming schema.
pub const CURRENT_STREAMING_VERSION: Version = Version::new(1, 0, 0);

/// Current version of the complete QEC subsystem contract.
pub const CURRENT_QEC_VERSION: Version = CURRENT_ALGORITHM_VERSION;

// ============================================================================
// Semantic version
// ============================================================================

/// Semantic version used by QEC contracts and schemas.
///
/// Major:
/// incompatible API, schema, or semantic change.
///
/// Minor:
/// backwards-compatible functionality.
///
/// Patch:
/// backwards-compatible correction.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    /// Creates a version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Zero/uninitialized version.
    pub const fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    /// Returns the current QEC subsystem version.
    pub const fn current() -> Self {
        CURRENT_QEC_VERSION
    }

    /// Returns true when the major versions match.
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }

    /// Returns true when the version is exactly equal.
    pub const fn is_exact(self, other: Self) -> bool {
        self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }

    /// Returns true when this version is older.
    pub fn is_older_than(self, other: Self) -> bool {
        self < other
    }

    /// Returns true when this version is newer.
    pub fn is_newer_than(self, other: Self) -> bool {
        self > other
    }

    /// Deterministic packed representation.
    ///
    /// This is intended for comparison/hash material, not textual encoding.
    pub const fn packed(self) -> u64 {
        ((self.major as u64) << 32)
            | ((self.minor as u64) << 16)
            | self.patch as u64
    }

    /// Determines whether `self` may be used by an implementation supporting
    /// `supported`, under the default same-major policy.
    pub fn is_compatible_with(self, supported: Self) -> bool {
        check_version_pair(self, supported, CompatibilityPolicy::SameMajor)
            .is_compatible()
    }

    /// Requires compatibility under the default same-major policy.
    pub fn require_compatible(
        self,
        supported: Self,
    ) -> Result<(), VersionError> {
        require_version_compatible(
            self,
            supported,
            CompatibilityPolicy::SameMajor,
        )
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::current()
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
        parse_version(value)
    }
}

/// Parses a strict `MAJOR.MINOR.PATCH` version.
pub fn parse_version(value: &str) -> Result<Version, VersionError> {
    if value.is_empty() {
        return Err(VersionError::Empty);
    }

    if value.len() > MAX_VERSION_STRING_LENGTH {
        return Err(VersionError::TooLong {
            max: MAX_VERSION_STRING_LENGTH,
        });
    }

    let mut parts = value.split('.');

    let major = parse_component(parts.next(), "major")?;
    let minor = parse_component(parts.next(), "minor")?;
    let patch = parse_component(parts.next(), "patch")?;

    if parts.next().is_some() {
        return Err(VersionError::InvalidFormat);
    }

    Ok(Version::new(major, minor, patch))
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
    Replay,
    Cache,
    Partition,
    Streaming,
    Distributed,
}

impl ArtifactKind {
    /// Stable wire/storage identifier.
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
            Self::Replay => "replay",
            Self::Cache => "cache",
            Self::Partition => "partition",
            Self::Streaming => "streaming",
            Self::Distributed => "distributed",
        }
    }

    /// Current schema/contract version for this artifact.
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
            Self::Replay => CURRENT_REPLAY_VERSION,
            Self::Cache => CURRENT_CACHE_VERSION,
            Self::Partition => CURRENT_PARTITION_VERSION,
            Self::Streaming => CURRENT_STREAMING_VERSION,
            Self::Distributed => CURRENT_DISTRIBUTED_VERSION,
        }
    }

    /// Returns true when this artifact is persistent or externally
    /// transferable and therefore requires explicit version validation.
    pub const fn is_persistent(self) -> bool {
        matches!(
            self,
            Self::Checkpoint
                | Self::Syndrome
                | Self::DecodingGraph
                | Self::Simulation
                | Self::NoiseModel
                | Self::DecoderResult
                | Self::Replay
                | Self::Cache
                | Self::Partition
                | Self::Streaming
                | Self::Distributed
        )
    }

    /// Returns true when the artifact can cross a trust boundary.
    pub const fn is_security_boundary(self) -> bool {
        matches!(
            self,
            Self::Checkpoint
                | Self::Capability
                | Self::QpuInterface
                | Self::QpuExecution
                | Self::Distributed
        )
    }
}

impl fmt::Display for ArtifactKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Execution target
// ============================================================================

/// Execution environment associated with an artifact.
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

    /// Returns true when QPU metadata/authorization is required.
    pub const fn requires_qpu(self) -> bool {
        matches!(
            self,
            Self::Qpu
                | Self::HybridCpuQpu
                | Self::HybridAcceleratorQpu
        )
    }

    /// Determines whether two execution targets can share an artifact
    /// without target-specific migration.
    pub const fn compatible_with(self, other: Self) -> bool {
        if self as u8 == other as u8 {
            return true;
        }

        matches!(
            (self, other),
            (Self::ClassicalCpu, Self::ParallelCpu)
                | (Self::ParallelCpu, Self::ClassicalCpu)
                | (Self::Simulator, Self::Emulator)
                | (Self::Emulator, Self::Simulator)
        )
    }
}

impl fmt::Display for ExecutionTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// Feature flags
// ============================================================================

/// Compact deterministic feature bitmap.
///
/// Feature flags are part of compatibility. A version match alone is not
/// sufficient when an artifact requires an unsupported feature.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FeatureFlags(u64);

impl FeatureFlags {
    pub const NONE: Self = Self(0);

    pub const STREAMING: u8 = 0;
    pub const PARTITIONING: u8 = 1;
    pub const DISTRIBUTED: u8 = 2;
    pub const GPU: u8 = 3;
    pub const ACCELERATOR: u8 = 4;
    pub const QPU: u8 = 5;
    pub const CHECKPOINTING: u8 = 6;
    pub const REPLAY: u8 = 7;
    pub const DETERMINISTIC: u8 = 8;
    pub const RESOURCE_ACCOUNTING: u8 = 9;
    pub const CAPABILITY_SECURITY: u8 = 10;
    pub const STATISTICAL_VERIFICATION: u8 = 11;
    pub const LOGICAL_EQUIVALENCE: u8 = 12;

    /// Creates flags from a raw bitmap.
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns the raw bitmap.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Returns true when a feature bit is enabled.
    pub const fn contains(self, bit: u8) -> bool {
        if bit >= FEATURE_FLAG_BITS as u8 {
            return false;
        }

        self.0 & (1u64 << bit) != 0
    }

    /// Enables a feature.
    pub const fn with(self, bit: u8) -> Self {
        if bit >= FEATURE_FLAG_BITS as u8 {
            return self;
        }

        Self(self.0 | (1u64 << bit))
    }

    /// Removes a feature.
    pub const fn without(self, bit: u8) -> Self {
        if bit >= FEATURE_FLAG_BITS as u8 {
            return self;
        }

        Self(self.0 & !(1u64 << bit))
    }

    /// Returns true when all required features are available.
    pub const fn supports(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }
}

// ============================================================================
// Compatibility policy
// ============================================================================

/// Explicit version compatibility policy.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CompatibilityPolicy {
    /// Exact major/minor/patch match.
    Exact,

    /// Same major version; artifact must not be newer than the supported
    /// implementation.
    SameMajor,

    /// Same major family; older versions may be read.
    BackwardCompatible,

    /// Same-major legacy artifacts may be read but must not be written back
    /// without migration.
    ReadOnlyLegacy,

    /// Explicit rejection.
    Reject,
}

impl Default for CompatibilityPolicy {
    fn default() -> Self {
        Self::SameMajor
    }
}

/// Result of comparing artifact and supported versions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Compatibility {
    Exact,
    Compatible,
    LegacyCompatible,
    Incompatible,
    NewerThanSupported,
    TargetMismatch,
    MissingFeature,
    ProtocolMismatch,
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

    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::LegacyCompatible)
    }
}

// ============================================================================
// Version manifest
// ============================================================================

/// Complete version manifest for a QEC execution environment.
///
/// This structure is the cross-module version contract.
///
/// `configuration`, `checkpoint`, `syndrome`, `graph`, decoder-result,
/// backend, capability, QPU, streaming, partition and distributed modules
/// may embed or reference this manifest.
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

    pub replay: Version,
    pub cache: Version,
    pub partition: Version,
    pub streaming: Version,
    pub distributed: Version,

    pub target: ExecutionTarget,
    pub features: FeatureFlags,
}

impl VersionManifest {
    /// Creates a manifest for the current implementation.
    pub const fn current(target: ExecutionTarget) -> Self {
        let mut features = FeatureFlags::NONE
            .with(FeatureFlags::DETERMINISTIC)
            .with(FeatureFlags::RESOURCE_ACCOUNTING)
            .with(FeatureFlags::CHECKPOINTING)
            .with(FeatureFlags::REPLAY)
            .with(FeatureFlags::CAPABILITY_SECURITY)
            .with(FeatureFlags::LOGICAL_EQUIVALENCE)
            .with(FeatureFlags::STATISTICAL_VERIFICATION);

        if target.requires_qpu() {
            features = features.with(FeatureFlags::QPU);
        }

        if matches!(
            target,
            ExecutionTarget::ParallelCpu
                | ExecutionTarget::Gpu
                | ExecutionTarget::Accelerator
        ) {
            features = features.with(FeatureFlags::STREAMING);
        }

        if matches!(target, ExecutionTarget::Distributed) {
            features = features
                .with(FeatureFlags::STREAMING)
                .with(FeatureFlags::PARTITIONING)
                .with(FeatureFlags::DISTRIBUTED);
        }

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

            replay: CURRENT_REPLAY_VERSION,
            cache: CURRENT_CACHE_VERSION,
            partition: CURRENT_PARTITION_VERSION,
            streaming: CURRENT_STREAMING_VERSION,
            distributed: CURRENT_DISTRIBUTED_VERSION,

            target,
            features,
        }
    }

    /// Returns the version belonging to an artifact kind.
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
            ArtifactKind::Replay => self.replay,
            ArtifactKind::Cache => self.cache,
            ArtifactKind::Partition => self.partition,
            ArtifactKind::Streaming => self.streaming,
            ArtifactKind::Distributed => self.distributed,
        }
    }

    /// Returns the currently supported version for an artifact kind.
    pub const fn current_version_of(kind: ArtifactKind) -> Version {
        kind.current_version()
    }

    /// Validates protocol-level metadata.
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

        if self.target.requires_qpu() {
            if self.qpu_interface.major == 0 {
                return Err(VersionError::InvalidQpuVersion);
            }

            if self.qpu_execution.major == 0 {
                return Err(VersionError::InvalidQpuVersion);
            }

            if !self.features.contains(FeatureFlags::QPU) {
                return Err(VersionError::MissingRequiredFeature(
                    "qpu",
                ));
            }
        }

        Ok(())
    }

    /// Checks whether this manifest is compatible with a supported manifest.
    pub fn compatibility_with(
        &self,
        supported: &Self,
        policy: CompatibilityPolicy,
    ) -> CompatibilityReport {
        let mut report = CompatibilityReport::new();

        if self.protocol_major != supported.protocol_major {
            report.compatibility = Compatibility::ProtocolMismatch;
            report.reason = CompatibilityReason::ProtocolMajorMismatch;
            return report;
        }

        if self.protocol_minor > supported.protocol_minor {
            report.compatibility = Compatibility::NewerThanSupported;
            report.reason = CompatibilityReason::NewerProtocol;
            return report;
        }

        if !self
            .target
            .compatible_with(supported.target)
        {
            report.compatibility = Compatibility::TargetMismatch;
            report.reason = CompatibilityReason::ExecutionTargetMismatch;
            return report;
        }

        if !supported.features.supports(self.features) {
            report.compatibility = Compatibility::MissingFeature;
            report.reason = CompatibilityReason::RequiredFeatureUnavailable;
            return report;
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
            ArtifactKind::Replay,
            ArtifactKind::Cache,
            ArtifactKind::Partition,
            ArtifactKind::Streaming,
            ArtifactKind::Distributed,
        ];

        let mut saw_legacy = false;

        for kind in kinds {
            let artifact = self.version_of(kind);
            let current = supported.version_of(kind);

            let compatibility =
                check_version_pair(artifact, current, policy);

            if !compatibility.is_compatible() {
                report.compatibility = compatibility;
                report.failed_artifact = Some(kind);
                report.reason =
                    CompatibilityReason::ArtifactVersionMismatch;
                return report;
            }

            if compatibility.requires_migration() {
                saw_legacy = true;
                report.legacy_artifact = Some(kind);
            }
        }

        if saw_legacy {
            report.compatibility = Compatibility::LegacyCompatible;
            report.reason = CompatibilityReason::LegacyArtifact;
        } else if self == supported {
            report.compatibility = Compatibility::Exact;
            report.reason = CompatibilityReason::ExactMatch;
        } else {
            report.compatibility = Compatibility::Compatible;
            report.reason = CompatibilityReason::SameMajorCompatible;
        }

        report
    }

    /// Requires compatibility with another manifest.
    pub fn require_compatible(
        &self,
        supported: &Self,
        policy: CompatibilityPolicy,
    ) -> Result<(), VersionError> {
        let report = self.compatibility_with(supported, policy);

        if report.compatibility.is_compatible() {
            Ok(())
        } else {
            Err(VersionError::ManifestIncompatible {
                artifact: report.failed_artifact,
                compatibility: report.compatibility,
            })
        }
    }
}

impl Default for VersionManifest {
    fn default() -> Self {
        Self::current(ExecutionTarget::ClassicalCpu)
    }
}

// ============================================================================
// Artifact header
// ============================================================================

/// Minimal version envelope that can be placed in front of an artifact.
///
/// This is deliberately payload-independent so checkpoint, cache, replay,
/// distributed and QPU modules can use the same contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactHeader {
    pub protocol_major: u16,
    pub protocol_minor: u16,
    pub kind: ArtifactKind,
    pub version: Version,
    pub target: ExecutionTarget,
    pub features: FeatureFlags,
}

impl ArtifactHeader {
    /// Creates a header using the current version for an artifact.
    pub const fn current(
        kind: ArtifactKind,
        target: ExecutionTarget,
    ) -> Self {
        let manifest = VersionManifest::current(target);

        Self {
            protocol_major: VERSION_PROTOCOL_MAJOR,
            protocol_minor: VERSION_PROTOCOL_MINOR,
            kind,
            version: kind.current_version(),
            target,
            features: manifest.features,
        }
    }

    /// Validates the header itself.
    pub fn validate(&self) -> Result<(), VersionError> {
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

        if self.version == Version::zero() {
            return Err(VersionError::InvalidArtifactVersion(
                self.kind,
            ));
        }

        if self.target.requires_qpu()
            && !self.features.contains(FeatureFlags::QPU)
        {
            return Err(VersionError::MissingRequiredFeature(
                "qpu",
            ));
        }

        Ok(())
    }

    /// Compares the header against the current implementation.
    pub fn compatibility(
        &self,
        policy: CompatibilityPolicy,
    ) -> Compatibility {
        if self
            .validate()
            .is_err()
        {
            return Compatibility::Incompatible;
        }

        check_version_pair(
            self.version,
            self.kind.current_version(),
            policy,
        )
    }

    /// Requires this header to be compatible with the current implementation.
    pub fn require_compatible(
        &self,
        policy: CompatibilityPolicy,
    ) -> Result<(), VersionError> {
        self.validate()?;

        let compatibility = self.compatibility(policy);

        if compatibility.is_compatible() {
            Ok(())
        } else {
            Err(VersionError::ArtifactIncompatible {
                kind: self.kind,
                found: self.version,
                supported: self.kind.current_version(),
                compatibility,
            })
        }
    }
}

// ============================================================================
// Compatibility report
// ============================================================================

/// Detailed result of a manifest compatibility check.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatibilityReport {
    pub compatibility: Compatibility,
    pub reason: CompatibilityReason,
    pub failed_artifact: Option<ArtifactKind>,
    pub legacy_artifact: Option<ArtifactKind>,
}

impl CompatibilityReport {
    pub const fn new() -> Self {
        Self {
            compatibility: Compatibility::Incompatible,
            reason: CompatibilityReason::NotChecked,
            failed_artifact: None,
            legacy_artifact: None,
        }
    }

    pub const fn is_compatible(&self) -> bool {
        self.compatibility.is_compatible()
    }

    pub const fn requires_migration(&self) -> bool {
        self.compatibility.requires_migration()
    }
}

impl Default for CompatibilityReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Machine-readable explanation for a compatibility result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CompatibilityReason {
    NotChecked,
    ExactMatch,
    SameMajorCompatible,
    LegacyArtifact,
    ProtocolMajorMismatch,
    NewerProtocol,
    ArtifactVersionMismatch,
    ExecutionTargetMismatch,
    RequiredFeatureUnavailable,
}

// ============================================================================
// Generic compatibility helpers
// ============================================================================

/// Compares an artifact version against a supported implementation version.
pub fn check_version_pair(
    artifact: Version,
    supported: Version,
    policy: CompatibilityPolicy,
) -> Compatibility {
    match policy {
        CompatibilityPolicy::Reject => Compatibility::Incompatible,

        CompatibilityPolicy::Exact => {
            if artifact == supported {
                Compatibility::Exact
            } else if artifact > supported {
                Compatibility::NewerThanSupported
            } else {
                Compatibility::Incompatible
            }
        }

        CompatibilityPolicy::SameMajor => {
            if artifact == supported {
                Compatibility::Exact
            } else if artifact.major != supported.major {
                Compatibility::Incompatible
            } else if artifact > supported {
                Compatibility::NewerThanSupported
            } else {
                Compatibility::Compatible
            }
        }

        CompatibilityPolicy::BackwardCompatible => {
            if artifact == supported {
                Compatibility::Exact
            } else if artifact.major != supported.major {
                Compatibility::Incompatible
            } else if artifact > supported {
                Compatibility::NewerThanSupported
            } else {
                Compatibility::Compatible
            }
        }

        CompatibilityPolicy::ReadOnlyLegacy => {
            if artifact == supported {
                Compatibility::Exact
            } else if artifact.major != supported.major {
                Compatibility::Incompatible
            } else if artifact > supported {
                Compatibility::NewerThanSupported
            } else {
                Compatibility::LegacyCompatible
            }
        }
    }
}

/// Checks compatibility using an explicit policy.
pub fn is_compatible_with(
    artifact: Version,
    supported: Version,
    policy: CompatibilityPolicy,
) -> bool {
    check_version_pair(artifact, supported, policy)
        .is_compatible()
}

/// Requires compatibility using an explicit policy.
pub fn require_compatible(
    artifact: Version,
    supported: Version,
    policy: CompatibilityPolicy,
) -> Result<(), VersionError> {
    require_version_compatible(artifact, supported, policy)
}

/// Internal/common version compatibility failure boundary.
pub fn require_version_compatible(
    artifact: Version,
    supported: Version,
    policy: CompatibilityPolicy,
) -> Result<(), VersionError> {
    let compatibility =
        check_version_pair(artifact, supported, policy);

    if compatibility.is_compatible() {
        Ok(())
    } else {
        Err(VersionError::Incompatible {
            artifact,
            supported,
            compatibility,
        })
    }
}

// ============================================================================
// Version requirements
// ============================================================================

/// Explicit requirements for a consumer of a versioned artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VersionRequirement {
    pub artifact: ArtifactKind,
    pub minimum: Version,
    pub maximum: Option<Version>,
    pub policy: CompatibilityPolicy,
    pub required_features: FeatureFlags,
    pub target: Option<ExecutionTarget>,
}

impl VersionRequirement {
    /// Creates a requirement for the current artifact version.
    pub const fn current(artifact: ArtifactKind) -> Self {
        Self {
            artifact,
            minimum: artifact.current_version(),
            maximum: Some(artifact.current_version()),
            policy: CompatibilityPolicy::Exact,
            required_features: FeatureFlags::NONE,
            target: None,
        }
    }

    /// Validates an artifact header against this requirement.
    pub fn validate(
        &self,
        header: &ArtifactHeader,
    ) -> Result<(), VersionError> {
        header.validate()?;

        if header.kind != self.artifact {
            return Err(VersionError::ArtifactKindMismatch {
                expected: self.artifact,
                found: header.kind,
            });
        }

        if !header.features.supports(self.required_features) {
            return Err(VersionError::MissingFeatures);
        }

        if let Some(target) = self.target {
            if !header.target.compatible_with(target) {
                return Err(VersionError::ExecutionTargetMismatch {
                    expected: target,
                    found: header.target,
                });
            }
        }

        if header.version < self.minimum {
            return Err(VersionError::BelowMinimumVersion {
                artifact: self.artifact,
                minimum: self.minimum,
                found: header.version,
            });
        }

        if let Some(maximum) = self.maximum {
            if header.version > maximum {
                return Err(VersionError::AboveMaximumVersion {
                    artifact: self.artifact,
                    maximum,
                    found: header.version,
                });
            }
        }

        header.require_compatible(self.policy)
    }
}

// ============================================================================
// Identifier validation
// ============================================================================

/// Validates an externally supplied artifact identifier.
///
/// Identifiers are deliberately conservative because they may appear in
/// checkpoint, cache, distributed, telemetry or QPU metadata.
pub fn validate_artifact_identifier(
    value: &str,
) -> Result<(), VersionError> {
    validate_identifier(
        value,
        MAX_ARTIFACT_ID_LENGTH,
        "artifact identifier",
    )
}

/// Validates an externally supplied provider/device identifier.
pub fn validate_provider_identifier(
    value: &str,
) -> Result<(), VersionError> {
    validate_identifier(
        value,
        MAX_IDENTIFIER_LENGTH,
        "provider/device identifier",
    )
}

fn validate_identifier(
    value: &str,
    max: usize,
    name: &'static str,
) -> Result<(), VersionError> {
    if value.is_empty() {
        return Err(VersionError::EmptyIdentifier(name));
    }

    if value.len() > max {
        return Err(VersionError::IdentifierTooLong {
            name,
            max,
        });
    }

    if !value.is_ascii() {
        return Err(VersionError::InvalidIdentifier(name));
    }

    if !value
        .bytes()
        .all(|byte| {
            byte.is_ascii_alphanumeric()
                || matches!(
                    byte,
                    b'-' | b'_' | b'.' | b':'
                )
        })
    {
        return Err(VersionError::InvalidIdentifier(name));
    }

    Ok(())
}

// ============================================================================
// Version errors
// ============================================================================

/// Local versioning error.
///
/// `errors.rs` should convert this into the canonical `QecError`:
///
/// ```text
/// VersionError
///      ↓
/// QecError::VersionMismatch / InvalidInput
/// ```
///
/// `version.rs` intentionally does not depend on `errors.rs`, preventing a
/// foundation-layer cycle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VersionError {
    Empty,

    TooLong {
        max: usize,
    },

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

    InvalidQpuVersion,

    InvalidArtifactVersion(ArtifactKind),

    MissingRequiredFeature(&'static str),

    MissingFeatures,

    ArtifactKindMismatch {
        expected: ArtifactKind,
        found: ArtifactKind,
    },

    ExecutionTargetMismatch {
        expected: ExecutionTarget,
        found: ExecutionTarget,
    },

    BelowMinimumVersion {
        artifact: ArtifactKind,
        minimum: Version,
        found: Version,
    },

    AboveMaximumVersion {
        artifact: ArtifactKind,
        maximum: Version,
        found: Version,
    },

    Incompatible {
        artifact: Version,
        supported: Version,
        compatibility: Compatibility,
    },

    ArtifactIncompatible {
        kind: ArtifactKind,
        found: Version,
        supported: Version,
        compatibility: Compatibility,
    },

    ManifestIncompatible {
        artifact: Option<ArtifactKind>,
        compatibility: Compatibility,
    },

    EmptyIdentifier(&'static str),

    IdentifierTooLong {
        name: &'static str,
        max: usize,
    },

    InvalidIdentifier(&'static str),
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                f.write_str("version is empty")
            }

            Self::TooLong { max } => {
                write!(
                    f,
                    "version exceeds maximum length of {}",
                    max
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
                    "version is missing {} component",
                    name
                )
            }

            Self::InvalidComponent(name) => {
                write!(
                    f,
                    "version contains invalid {} component",
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
                f.write_str("invalid versioning protocol")
            }

            Self::ProtocolMismatch {
                expected_major,
                found_major,
            } => {
                write!(
                    f,
                    "versioning protocol major mismatch: \
                     expected {}, found {}",
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
                    "versioning protocol minor {} is newer \
                     than supported {}",
                    found,
                    supported
                )
            }

            Self::InvalidQpuVersion => {
                f.write_str("invalid QPU version metadata")
            }

            Self::InvalidArtifactVersion(kind) => {
                write!(
                    f,
                    "invalid version for artifact {}",
                    kind
                )
            }

            Self::MissingRequiredFeature(feature) => {
                write!(
                    f,
                    "required feature '{}' is missing",
                    feature
                )
            }

            Self::MissingFeatures => {
                f.write_str(
                    "artifact requires unsupported feature flags",
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

            Self::ExecutionTargetMismatch {
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

            Self::BelowMinimumVersion {
                artifact,
                minimum,
                found,
            } => {
                write!(
                    f,
                    "{} version {} is below required minimum {}",
                    artifact,
                    found,
                    minimum
                )
            }

            Self::AboveMaximumVersion {
                artifact,
                maximum,
                found,
            } => {
                write!(
                    f,
                    "{} version {} exceeds maximum {}",
                    artifact,
                    found,
                    maximum
                )
            }

            Self::Incompatible {
                artifact,
                supported,
                compatibility,
            } => {
                write!(
                    f,
                    "version {} is incompatible with supported {} ({:?})",
                    artifact,
                    supported,
                    compatibility
                )
            }

            Self::ArtifactIncompatible {
                kind,
                found,
                supported,
                compatibility,
            } => {
                write!(
                    f,
                    "{} artifact version {} is incompatible \
                     with supported {} ({:?})",
                    kind,
                    found,
                    supported,
                    compatibility
                )
            }

            Self::ManifestIncompatible {
                artifact,
                compatibility,
            } => {
                match artifact {
                    Some(kind) => write!(
                        f,
                        "version manifest incompatible at {} ({:?})",
                        kind,
                        compatibility
                    ),

                    None => write!(
                        f,
                        "version manifest incompatible ({:?})",
                        compatibility
                    ),
                }
            }

            Self::EmptyIdentifier(name) => {
                write!(f, "{} is empty", name)
            }

            Self::IdentifierTooLong { name, max } => {
                write!(
                    f,
                    "{} exceeds maximum length {}",
                    name,
                    max
                )
            }

            Self::InvalidIdentifier(name) => {
                write!(
                    f,
                    "{} contains invalid characters",
                    name
                )
            }
        }
    }
}

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

        assert_eq!(text, "12.34.56");
        assert_eq!(
            text.parse::<Version>().unwrap(),
            version
        );
    }

    #[test]
    fn version_rejects_invalid_formats() {
        assert!("".parse::<Version>().is_err());
        assert!("1".parse::<Version>().is_err());
        assert!("1.2".parse::<Version>().is_err());
        assert!("1.2.3.4".parse::<Version>().is_err());
        assert!("a.2.3".parse::<Version>().is_err());
        assert!("1.-2.3".parse::<Version>().is_err());
    }

    #[test]
    fn exact_version_is_compatible() {
        let version = Version::new(2, 0, 0);

        assert_eq!(
            check_version_pair(
                version,
                version,
                CompatibilityPolicy::Exact
            ),
            Compatibility::Exact
        );
    }

    #[test]
    fn newer_artifact_is_rejected() {
        assert_eq!(
            check_version_pair(
                Version::new(3, 0, 0),
                Version::new(2, 0, 0),
                CompatibilityPolicy::SameMajor
            ),
            Compatibility::Incompatible
        );

        assert_eq!(
            check_version_pair(
                Version::new(2, 1, 0),
                Version::new(2, 0, 0),
                CompatibilityPolicy::SameMajor
            ),
            Compatibility::NewerThanSupported
        );
    }

    #[test]
    fn older_same_major_artifact_is_compatible() {
        assert_eq!(
            check_version_pair(
                Version::new(2, 0, 0),
                Version::new(2, 1, 0),
                CompatibilityPolicy::SameMajor
            ),
            Compatibility::Compatible
        );
    }

    #[test]
    fn read_only_legacy_is_explicit() {
        assert_eq!(
            check_version_pair(
                Version::new(2, 0, 0),
                Version::new(2, 1, 0),
                CompatibilityPolicy::ReadOnlyLegacy
            ),
            Compatibility::LegacyCompatible
        );
    }

    #[test]
    fn major_mismatch_is_rejected() {
        assert_eq!(
            check_version_pair(
                Version::new(1, 0, 0),
                Version::new(2, 0, 0),
                CompatibilityPolicy::SameMajor
            ),
            Compatibility::Incompatible
        );
    }

    #[test]
    fn feature_flags_are_subset_checked() {
        let supported = FeatureFlags::NONE
            .with(FeatureFlags::QPU)
            .with(FeatureFlags::DETERMINISTIC);

        let required = FeatureFlags::NONE
            .with(FeatureFlags::QPU);

        assert!(supported.supports(required));

        let missing = FeatureFlags::NONE
            .with(FeatureFlags::GPU);

        assert!(!supported.supports(missing));
    }

    #[test]
    fn current_manifest_validates() {
        let manifest =
            VersionManifest::current(
                ExecutionTarget::ClassicalCpu,
            );

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn current_qpu_manifest_contains_qpu_feature() {
        let manifest =
            VersionManifest::current(ExecutionTarget::Qpu);

        assert!(manifest.validate().is_ok());
        assert!(
            manifest
                .features
                .contains(FeatureFlags::QPU)
        );
    }

    #[test]
    fn qpu_header_requires_qpu_feature() {
        let mut header =
            ArtifactHeader::current(
                ArtifactKind::QpuExecution,
                ExecutionTarget::Qpu,
            );

        header.features =
            FeatureFlags::NONE;

        assert!(header.validate().is_err());
    }

    #[test]
    fn artifact_header_uses_current_version() {
        let header =
            ArtifactHeader::current(
                ArtifactKind::Checkpoint,
                ExecutionTarget::ClassicalCpu,
            );

        assert_eq!(
            header.version,
            CURRENT_CHECKPOINT_VERSION
        );

        assert!(
            header
                .require_compatible(
                    CompatibilityPolicy::SameMajor
                )
                .is_ok()
        );
    }

    #[test]
    fn requirement_rejects_wrong_kind() {
        let requirement =
            VersionRequirement::current(
                ArtifactKind::Checkpoint,
            );

        let header =
            ArtifactHeader::current(
                ArtifactKind::Syndrome,
                ExecutionTarget::ClassicalCpu,
            );

        assert!(requirement.validate(&header).is_err());
    }

    #[test]
    fn identifiers_are_conservative() {
        assert!(
            validate_artifact_identifier(
                "checkpoint-v3"
            )
            .is_ok()
        );

        assert!(
            validate_provider_identifier(
                "provider:device_v1"
            )
            .is_ok()
        );

        assert!(
            validate_artifact_identifier(
                "invalid identifier"
            )
            .is_err()
        );
    }

    #[test]
    fn target_compatibility_is_explicit() {
        assert!(
            ExecutionTarget::ClassicalCpu
                .compatible_with(
                    ExecutionTarget::ParallelCpu
                )
        );

        assert!(
            !ExecutionTarget::Qpu
                .compatible_with(
                    ExecutionTarget::Gpu
                )
        );
    }

    #[test]
    fn manifest_comparison_is_exact_for_current_manifests() {
        let a =
            VersionManifest::current(
                ExecutionTarget::ClassicalCpu,
            );

        let b = a.clone();

        let report = a.compatibility_with(
            &b,
            CompatibilityPolicy::SameMajor,
        );

        assert_eq!(
            report.compatibility,
            Compatibility::Exact
        );

        assert!(report.is_compatible());
    }

    #[test]
    fn manifest_rejects_newer_protocol() {
        let mut artifact =
            VersionManifest::current(
                ExecutionTarget::ClassicalCpu,
            );

        artifact.protocol_minor =
            VERSION_PROTOCOL_MINOR + 1;

        let supported =
            VersionManifest::current(
                ExecutionTarget::ClassicalCpu,
            );

        let report = artifact.compatibility_with(
            &supported,
            CompatibilityPolicy::SameMajor,
        );

        assert_eq!(
            report.compatibility,
            Compatibility::NewerThanSupported
        );
    }

    #[test]
    fn packed_version_is_deterministic() {
        let a = Version::new(1, 2, 3);
        let b = Version::new(1, 2, 3);

        assert_eq!(a.packed(), b.packed());
    }
}