//! Zamani Quantum Resilience — Checkpoint Compatibility
//!
//! Path:
//!     src/quantum/resilience/checkpoint/compatibility.rs
//!
//! Purpose:
//!     Provider-neutral compatibility negotiation for restoring a checkpoint
//!     against a target execution environment.
//!
//! Architectural ownership:
//!
//!     checkpoint.rs
//!         Owns:
//!             - checkpoint identity;
//!             - checkpoint lifecycle;
//!             - checkpoint semantic boundary;
//!             - checkpoint state classification;
//!             - payload descriptors;
//!             - resource/qubit scope.
//!
//!     snapshot.rs
//!         Owns:
//!             - snapshot semantics;
//!             - snapshot restoration requirements;
//!             - provider/runtime snapshot descriptors.
//!
//!     manifest.rs
//!         Owns:
//!             - artifact enumeration;
//!             - artifact ordering;
//!             - manifest composition.
//!
//!     storage.rs
//!         Owns:
//!             - physical persistence;
//!             - storage capabilities;
//!             - object addressing;
//!             - streaming I/O.
//!
//!     integrity.rs
//!         Owns:
//!             - cryptographic integrity;
//!             - digest verification;
//!             - authenticity verification.
//!
//!     compatibility.rs
//!         Owns:
//!             - compatibility requirements;
//!             - target compatibility descriptions;
//!             - schema compatibility;
//!             - capability compatibility;
//!             - restoration-mode compatibility;
//!             - compatibility assessment;
//!             - deterministic compatibility negotiation.
//!
//!     recovery::checkpoint
//!         Owns:
//!             - recovery orchestration;
//!             - ordering of integrity/compatibility/policy checks;
//!             - restore execution.
//!
//!     verification::*
//!         Owns:
//!             - semantic verification;
//!             - post-restore validation;
//!             - result acceptance.
//!
//! Important quantum correctness rule:
//!
//!     Compatibility MUST NOT imply that an arbitrary unknown quantum state
//!     can be restored.
//!
//!     A checkpoint is compatible only when the checkpoint's declared
//!     restoration semantics are explicitly supported by the target or can be
//!     reconstructed using a declared portable mechanism.
//!
//! Write once, scale everywhere:
//!
//!     This module contains no:
//!
//!         MAX_QUBITS
//!         MAX_PHYSICAL_QUBITS
//!         MAX_CHECKPOINTS
//!         MAX_ARTIFACTS
//!         MAX_RETRIES
//!         provider names
//!         fixed topology assumptions
//!         fixed device IDs
//!
//!     Resource quantities are represented as requirements and observations.
//!     Actual limits come from target capabilities, policy, runtime resources,
//!     storage, memory, and the selected execution environment.
//!
//! Logical/physical identity:
//!
//!     The canonical IR identity types are authoritative:
//!
//!         crate::quantum::ir::qubit::QubitId
//!         crate::quantum::ir::qubit::PhysicalQubitId
//!
//!     This file MUST NOT define another qubit identity type.
//!
//! Compatibility is not equality:
//!
//!     A target does not have to be identical to the checkpoint's original
//!     target. It only has to satisfy the checkpoint's semantic and restoration
//!     requirements.
//!
//!     Example:
//!
//!         checkpoint created on target A
//!                 |
//!                 v
//!         target B
//!                 |
//!                 v
//!         compatible after adaptation
//!
//!     may be valid when the checkpoint is replayable/reconstructible and the
//!     new target satisfies all required capabilities.
//!
//! Unknown compatibility:
//!
//!     Unknown MUST NOT silently become Compatible.
//!
//!     Unknown may be converted to a usable outcome only after another trusted
//!     capability/metadata source establishes the missing fact.
//!
//! Determinism:
//!
//!     Assessment ordering is deterministic.
//!
//!     Collections used for capability/property requirements are BTreeMap/
//!     BTreeSet rather than HashMap/HashSet.
//!
//!     No wall-clock time, randomness, provider ordering, or pointer identity
//!     participates in the compatibility decision.
//!
//! Security:
//!
//!     Compatibility metadata is input data and must be treated as untrusted.
//!
//!     This module does not authenticate metadata. Authenticity/integrity is
//!     supplied by checkpoint/integrity.rs and the security boundary.
//!
//!     A compatible result is not authorization to restore. Recovery policy
//!     and security authorization remain separate gates.
//!
//! Rust:
//!
//!     - Rust 1.97 / 1.97.1
//!     - Rust 2021
//!     - stable Rust
//!     - no nightly features
//!     - no unsafe code

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

use super::checkpoint::{
    Checkpoint,
    CheckpointBoundary,
    CheckpointId,
    CheckpointStateKind,
};

// ============================================================================
// Schema identity
// ============================================================================

/// Stable namespace for checkpoint compatibility.
pub const CHECKPOINT_COMPATIBILITY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.checkpoint.compatibility";

/// Current compatibility contract major version.
///
/// Breaking semantic changes require a new major version.
pub const CHECKPOINT_COMPATIBILITY_SCHEMA_MAJOR: u16 = 1;

/// Current compatibility contract minor version.
///
/// Backwards-compatible additions may advance this value.
pub const CHECKPOINT_COMPATIBILITY_SCHEMA_MINOR: u16 = 0;

/// Current compatibility contract patch version.
pub const CHECKPOINT_COMPATIBILITY_SCHEMA_PATCH: u16 = 0;

/// Complete current compatibility schema version.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct CompatibilitySchemaVersion {
    /// Breaking version.
    pub major: u16,

    /// Backwards-compatible version.
    pub minor: u16,

    /// Patch version.
    pub patch: u16,
}

impl CompatibilitySchemaVersion {
    /// Current version.
    pub const CURRENT: Self = Self {
        major: CHECKPOINT_COMPATIBILITY_SCHEMA_MAJOR,
        minor: CHECKPOINT_COMPATIBILITY_SCHEMA_MINOR,
        patch: CHECKPOINT_COMPATIBILITY_SCHEMA_PATCH,
    };

    /// Creates a version.
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

    /// Returns whether two versions share the same semantic major version.
    #[must_use]
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }
}

// ============================================================================
// Compatibility outcome
// ============================================================================

/// Result of compatibility negotiation.
///
/// These are decisions, not recovery actions.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum CompatibilityLevel {
    /// Existing checkpoint representation can be restored directly.
    Compatible,

    /// Restoration is possible after a valid adaptation.
    CompatibleWithAdaptation,

    /// Restoration is possible while accepting an explicitly declared
    /// degradation.
    CompatibleWithDegradation,

    /// Restoration is possible only after moving to another target.
    CompatibleWithMigration,

    /// Compatibility depends on a condition that must be verified before
    /// restoration.
    ConditionallyCompatible,

    /// The target cannot satisfy the checkpoint requirements.
    Incompatible,

    /// Trusted information is insufficient to establish compatibility.
    Unknown,
}

impl CompatibilityLevel {
    /// Returns whether this outcome is potentially usable.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::Compatible
                | Self::CompatibleWithAdaptation
                | Self::CompatibleWithDegradation
                | Self::CompatibleWithMigration
                | Self::ConditionallyCompatible
        )
    }

    /// Returns whether this outcome is definitively incompatible.
    #[must_use]
    pub const fn is_incompatible(self) -> bool {
        matches!(self, Self::Incompatible)
    }

    /// Returns whether more information is required.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns whether another execution target may be required.
    #[must_use]
    pub const fn requires_migration(self) -> bool {
        matches!(self, Self::CompatibleWithMigration)
    }

    /// Returns whether an adaptation is required.
    #[must_use]
    pub const fn requires_adaptation(self) -> bool {
        matches!(
            self,
            Self::CompatibleWithAdaptation
                | Self::CompatibleWithDegradation
                | Self::CompatibleWithMigration
        )
    }
}

// ============================================================================
// Compatibility dimension
// ============================================================================

/// Independent dimension used in compatibility assessment.
///
/// Keeping dimensions explicit prevents one successful check from masking a
/// failure in another dimension.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum CompatibilityDimension {
    /// Checkpoint schema.
    CheckpointSchema,

    /// Canonical IR schema.
    IrSchema,

    /// Program semantic identity.
    Program,

    /// Checkpoint state semantics.
    StateSemantics,

    /// Restoration mechanism.
    Restoration,

    /// Logical resource requirements.
    LogicalResources,

    /// Physical resource requirements.
    PhysicalResources,

    /// Required quantum operations.
    Operations,

    /// Connectivity/topology.
    Topology,

    /// Timing.
    Timing,

    /// Measurement.
    Measurement,

    /// Reset.
    Reset,

    /// Mid-circuit measurement/classical control.
    ClassicalControl,

    /// QEC/logical protection.
    Qec,

    /// Noise/fault constraints.
    Noise,

    /// Execution/result format.
    Execution,

    /// Security requirements.
    Security,

    /// Storage/checkpoint artifact requirements.
    Storage,

    /// Integrity/authenticity requirements.
    Integrity,

    /// Recovery requirements.
    Recovery,

    /// Distributed execution requirements.
    Distributed,
}

// ============================================================================
// Requirement severity
// ============================================================================

/// Importance of a compatibility requirement.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum RequirementStrength {
    /// Requirement is mandatory.
    Required,

    /// Requirement is preferred but may be relaxed by policy.
    Preferred,

    /// Requirement is informational.
    Optional,
}

// ============================================================================
// Generic capability value
// ============================================================================

/// Capability value used by extensible compatibility requirements.
///
/// Core compatibility semantics use strongly typed fields below. This
/// extensible map allows future hardware/runtime features without requiring
/// this file to know every future quantum technology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityValue {
    /// Boolean capability.
    Boolean(bool),

    /// Non-negative integer quantity.
    Unsigned(u64),

    /// Textual capability identifier.
    Text(String),

    /// Set of textual capability identifiers.
    TextSet(BTreeSet<String>),

    /// Opaque version string.
    Version(String),
}

impl CapabilityValue {
    /// Validates the value.
    pub fn validate(&self) -> Result<(), CompatibilityError> {
        if let Self::Text(value) | Self::Version(value) = self {
            validate_non_empty(value, "capability_value")?;
        }

        if let Self::TextSet(values) = self {
            if values.iter().any(|value| value.trim().is_empty()) {
                return Err(CompatibilityError::InvalidCapabilityValue);
            }
        }

        Ok(())
    }
}

// ============================================================================
// Generic capability set
// ============================================================================

/// Extensible target capability set.
///
/// This deliberately does not model a particular provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilitySet {
    /// Generic named capabilities.
    pub values: BTreeMap<String, CapabilityValue>,
}

impl CapabilitySet {
    /// Creates an empty capability set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: BTreeMap::new(),
        }
    }

    /// Adds or replaces a capability.
    pub fn insert(
        &mut self,
        name: impl Into<String>,
        value: CapabilityValue,
    ) -> Result<(), CompatibilityError> {
        let name = name.into();

        validate_non_empty(&name, "capability_name")?;
        value.validate()?;

        self.values.insert(name, value);

        Ok(())
    }

    /// Gets a capability.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&CapabilityValue> {
        self.values.get(name)
    }

    /// Returns whether a named capability exists.
    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    /// Returns the number of named capabilities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns true when there are no capabilities.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl Default for CapabilitySet {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Resource requirements
// ============================================================================

/// Resource requirements for restoring a checkpoint.
///
/// Quantities are requirements, not allocations.
///
/// `None` means the checkpoint does not declare a requirement in that
/// dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// Required logical-qubit count.
    pub logical_qubits: Option<u64>,

    /// Required physical-qubit count.
    pub physical_qubits: Option<u64>,

    /// Required classical memory in bytes.
    pub classical_memory_bytes: Option<u64>,

    /// Required execution channels.
    pub execution_channels: Option<u64>,

    /// Required storage bytes.
    pub storage_bytes: Option<u64>,
}

impl ResourceRequirements {
    /// Creates an unconstrained requirement set.
    #[must_use]
    pub const fn unconstrained() -> Self {
        Self {
            logical_qubits: None,
            physical_qubits: None,
            classical_memory_bytes: None,
            execution_channels: None,
            storage_bytes: None,
        }
    }

    /// Validates the requirements.
    pub fn validate(&self) -> Result<(), CompatibilityError> {
        Ok(())
    }
}

// ============================================================================
// Target resources
// ============================================================================

/// Resources currently available on the restoration target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetResources {
    /// Available logical qubits.
    pub logical_qubits: Option<u64>,

    /// Available physical qubits.
    pub physical_qubits: Option<u64>,

    /// Available classical memory in bytes.
    pub classical_memory_bytes: Option<u64>,

    /// Available execution channels.
    pub execution_channels: Option<u64>,

    /// Available storage bytes.
    pub storage_bytes: Option<u64>,
}

impl TargetResources {
    /// Creates an unconstrained resource description.
    ///
    /// This means the target did not advertise a fixed quantity. It does not
    /// allocate resources.
    #[must_use]
    pub const fn unconstrained() -> Self {
        Self {
            logical_qubits: None,
            physical_qubits: None,
            classical_memory_bytes: None,
            execution_channels: None,
            storage_bytes: None,
        }
    }
}

// ============================================================================
// Operation requirements
// ============================================================================

/// Required operation identifiers.
///
/// Operation identifiers are semantic names supplied by the canonical IR or
/// lowering layer. No provider-specific operation enum is embedded here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationRequirements {
    /// Required operations.
    pub required: BTreeSet<String>,

    /// Preferred operations.
    pub preferred: BTreeSet<String>,
}

impl OperationRequirements {
    /// Creates empty operation requirements.
    #[must_use]
    pub fn new() -> Self {
        Self {
            required: BTreeSet::new(),
            preferred: BTreeSet::new(),
        }
    }

    /// Adds a required operation.
    pub fn require(
        &mut self,
        operation: impl Into<String>,
    ) -> Result<(), CompatibilityError> {
        let operation = operation.into();
        validate_non_empty(&operation, "operation")?;
        self.required.insert(operation);
        Ok(())
    }

    /// Adds a preferred operation.
    pub fn prefer(
        &mut self,
        operation: impl Into<String>,
    ) -> Result<(), CompatibilityError> {
        let operation = operation.into();
        validate_non_empty(&operation, "operation")?;
        self.preferred.insert(operation);
        Ok(())
    }

    /// Validates operation requirements.
    pub fn validate(&self) -> Result<(), CompatibilityError> {
        if self
            .required
            .iter()
            .any(|operation| operation.trim().is_empty())
            || self
                .preferred
                .iter()
                .any(|operation| operation.trim().is_empty())
        {
            return Err(CompatibilityError::InvalidOperationRequirement);
        }

        Ok(())
    }
}

impl Default for OperationRequirements {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Target operations
// ============================================================================

/// Operations exposed by the target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetOperations {
    /// Supported operations.
    pub supported: BTreeSet<String>,
}

impl TargetOperations {
    /// Creates an empty set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            supported: BTreeSet::new(),
        }
    }

    /// Adds an operation.
    pub fn insert(
        &mut self,
        operation: impl Into<String>,
    ) -> Result<(), CompatibilityError> {
        let operation = operation.into();
        validate_non_empty(&operation, "operation")?;
        self.supported.insert(operation);
        Ok(())
    }

    /// Returns whether the operation is supported.
    #[must_use]
    pub fn contains(&self, operation: &str) -> bool {
        self.supported.contains(operation)
    }
}

impl Default for TargetOperations {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Topology requirements
// ============================================================================

/// Abstract topology requirements.
///
/// Resilience does not implement routing. It only records whether routing
/// capability is required and whether a target must expose a topology suitable
/// for the checkpoint's restoration mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyRequirements {
    /// Whether topology information is required.
    pub required: bool,

    /// Whether arbitrary routing is acceptable.
    pub routing_supported: bool,

    /// Number of interaction relations required, when known.
    pub required_interactions: Option<u64>,
}

impl TopologyRequirements {
    /// No topology requirement.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            required: false,
            routing_supported: false,
            required_interactions: None,
        }
    }
}

// ============================================================================
// Target topology
// ============================================================================

/// Abstract topology capabilities exposed by the target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetTopology {
    /// Whether topology information is available.
    pub available: bool,

    /// Whether routing can construct alternative placements.
    pub routing_supported: bool,

    /// Number of currently usable interaction relations, if known.
    pub available_interactions: Option<u64>,
}

impl TargetTopology {
    /// Creates an unknown topology description.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            available: false,
            routing_supported: false,
            available_interactions: None,
        }
    }
}

// ============================================================================
// Timing requirements
// ============================================================================

/// Timing requirements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimingRequirements {
    /// Whether target timing compatibility is mandatory.
    pub required: bool,

    /// Maximum allowed execution duration in nanoseconds, if declared.
    pub maximum_execution_duration_ns: Option<u64>,

    /// Whether dynamic timing/scheduling adaptation is acceptable.
    pub rescheduling_allowed: bool,
}

impl TimingRequirements {
    /// No timing requirement.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            required: false,
            maximum_execution_duration_ns: None,
            rescheduling_allowed: false,
        }
    }
}

// ============================================================================
// Target timing
// ============================================================================

/// Timing capabilities supplied by the target/scheduler.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetTiming {
    /// Whether timing information is available.
    pub available: bool,

    /// Maximum execution duration supported by the current target, when known.
    pub maximum_execution_duration_ns: Option<u64>,

    /// Whether rescheduling is supported.
    pub rescheduling_supported: bool,
}

impl TargetTiming {
    /// Unknown timing.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            available: false,
            maximum_execution_duration_ns: None,
            rescheduling_supported: false,
        }
    }
}

// ============================================================================
// QEC requirements
// ============================================================================

/// Logical/QEC restoration requirements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QecRequirements {
    /// Whether QEC support is required.
    pub required: bool,

    /// Optional code-family identifier.
    pub code_family: Option<String>,

    /// Required logical-qubit count.
    pub logical_qubits: Option<u64>,

    /// Required minimum code distance.
    pub minimum_distance: Option<u64>,

    /// Required decoder identifier.
    pub decoder: Option<String>,

    /// Whether QEC adaptation is permitted.
    pub adaptation_allowed: bool,
}

impl QecRequirements {
    /// No QEC requirement.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            required: false,
            code_family: None,
            logical_qubits: None,
            minimum_distance: None,
            decoder: None,
            adaptation_allowed: false,
        }
    }

    /// Validates QEC requirements.
    pub fn validate(&self) -> Result<(), CompatibilityError> {
        if let Some(value) = &self.code_family {
            validate_non_empty(value, "code_family")?;
        }

        if let Some(value) = &self.decoder {
            validate_non_empty(value, "decoder")?;
        }

        Ok(())
    }
}

// ============================================================================
// Target QEC
// ============================================================================

/// QEC capabilities of a target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetQec {
    /// Whether QEC is supported.
    pub supported: bool,

    /// Supported code families.
    pub code_families: BTreeSet<String>,

    /// Available logical qubits.
    pub logical_qubits: Option<u64>,

    /// Maximum/current code distance, when known.
    pub maximum_distance: Option<u64>,

    /// Supported decoders.
    pub decoders: BTreeSet<String>,

    /// Whether QEC adaptation is supported.
    pub adaptation_supported: bool,
}

impl TargetQec {
    /// Unknown QEC capability.
    #[must_use]
    pub fn unknown() -> Self {
        Self {
            supported: false,
            code_families: BTreeSet::new(),
            logical_qubits: None,
            maximum_distance: None,
            decoders: BTreeSet::new(),
            adaptation_supported: false,
        }
    }
}

// ============================================================================
// Restoration requirements
// ============================================================================

/// Restoration mechanism required by a checkpoint.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum RestorationRequirement {
    /// Classical data can directly reconstruct the required state.
    DirectClassical,

    /// Canonical program/execution information can reconstruct the state.
    DeterministicReplay,

    /// Compiled representation can be replayed/reconstructed.
    CompiledReplay,

    /// Measurement boundary is reconstructible.
    MeasurementReconstruction,

    /// Logical/QEC state must be restored through a supported QEC mechanism.
    LogicalQecRestore,

    /// Runtime-defined reconstruction is required.
    RuntimeReconstruction,

    /// Provider-native quantum state restore is required.
    ProviderNativeRestore,

    /// Opaque provider state restore is required.
    ProviderOpaqueRestore,
}

impl RestorationRequirement {
    /// Returns whether the requirement is portable in principle.
    #[must_use]
    pub const fn is_portable(self) -> bool {
        matches!(
            self,
            Self::DirectClassical
                | Self::DeterministicReplay
                | Self::CompiledReplay
                | Self::MeasurementReconstruction
                | Self::LogicalQecRestore
                | Self::RuntimeReconstruction
        )
    }

    /// Returns whether explicit provider/runtime support is required.
    #[must_use]
    pub const fn requires_native_support(self) -> bool {
        !self.is_portable()
    }
}

// ============================================================================
// Target restoration capabilities
// ============================================================================

/// Restoration capabilities of the target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetRestorationCapabilities {
    /// Whether any checkpoint restoration is supported.
    pub restoration_supported: bool,

    /// Classical restoration.
    pub direct_classical_supported: bool,

    /// Deterministic replay.
    pub deterministic_replay_supported: bool,

    /// Compiled replay.
    pub compiled_replay_supported: bool,

    /// Measurement reconstruction.
    pub measurement_reconstruction_supported: bool,

    /// Logical/QEC restore.
    pub logical_qec_restore_supported: bool,

    /// Runtime reconstruction.
    pub runtime_reconstruction_supported: bool,

    /// Provider-native state restoration.
    pub provider_native_restore_supported: bool,

    /// Provider-opaque state restoration.
    pub provider_opaque_restore_supported: bool,

    /// Whether the target can reconstruct the checkpoint on a different
    /// physical mapping.
    pub remapping_supported: bool,

    /// Whether the target can reschedule the restored workload.
    pub rescheduling_supported: bool,

    /// Whether recompilation is supported.
    pub recompilation_supported: bool,

    /// Whether backend/device migration is supported by the surrounding
    /// execution system.
    pub migration_supported: bool,
}

impl TargetRestorationCapabilities {
    /// Creates a completely unsupported target description.
    #[must_use]
    pub const fn unsupported() -> Self {
        Self {
            restoration_supported: false,
            direct_classical_supported: false,
            deterministic_replay_supported: false,
            compiled_replay_supported: false,
            measurement_reconstruction_supported: false,
            logical_qec_restore_supported: false,
            runtime_reconstruction_supported: false,
            provider_native_restore_supported: false,
            provider_opaque_restore_supported: false,
            remapping_supported: false,
            rescheduling_supported: false,
            recompilation_supported: false,
            migration_supported: false,
        }
    }

    /// Returns whether the requested restoration mechanism is supported.
    #[must_use]
    pub const fn supports(
        &self,
        requirement: RestorationRequirement,
    ) -> bool {
        match requirement {
            RestorationRequirement::DirectClassical => {
                self.direct_classical_supported
            }
            RestorationRequirement::DeterministicReplay => {
                self.deterministic_replay_supported
            }
            RestorationRequirement::CompiledReplay => {
                self.compiled_replay_supported
            }
            RestorationRequirement::MeasurementReconstruction => {
                self.measurement_reconstruction_supported
            }
            RestorationRequirement::LogicalQecRestore => {
                self.logical_qec_restore_supported
            }
            RestorationRequirement::RuntimeReconstruction => {
                self.runtime_reconstruction_supported
            }
            RestorationRequirement::ProviderNativeRestore => {
                self.provider_native_restore_supported
            }
            RestorationRequirement::ProviderOpaqueRestore => {
                self.provider_opaque_restore_supported
            }
        }
    }
}

// ============================================================================
// Security requirements
// ============================================================================

/// Security requirements relevant to restoration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityRequirements {
    /// Whether authenticated checkpoint metadata is mandatory.
    pub authenticated_metadata_required: bool,

    /// Whether encrypted payloads are mandatory.
    pub encrypted_payload_required: bool,

    /// Whether trusted execution is mandatory.
    pub trusted_execution_required: bool,

    /// Security profile identifier.
    pub profile: Option<String>,
}

impl SecurityRequirements {
    /// No additional security requirement.
    #[must_use]
    pub const fn none() -> Self {
        Self {
            authenticated_metadata_required: false,
            encrypted_payload_required: false,
            trusted_execution_required: false,
            profile: None,
        }
    }

    /// Validates the requirements.
    pub fn validate(&self) -> Result<(), CompatibilityError> {
        if let Some(profile) = &self.profile {
            validate_non_empty(profile, "security_profile")?;
        }

        Ok(())
    }
}

// ============================================================================
// Target security
// ============================================================================

/// Security capabilities exposed by the target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetSecurity {
    /// Target can authenticate checkpoint metadata.
    pub authenticated_metadata_supported: bool,

    /// Target can restore encrypted checkpoint payloads.
    pub encrypted_payload_supported: bool,

    /// Target provides the required trusted execution boundary.
    pub trusted_execution_supported: bool,

    /// Supported security profiles.
    pub profiles: BTreeSet<String>,
}

impl TargetSecurity {
    /// Unknown security capabilities.
    #[must_use]
    pub fn unknown() -> Self {
        Self {
            authenticated_metadata_supported: false,
            encrypted_payload_supported: false,
            trusted_execution_supported: false,
            profiles: BTreeSet::new(),
        }
    }
}

// ============================================================================
// Checkpoint compatibility requirements
// ============================================================================

/// Complete compatibility requirements extracted from a checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointCompatibilityRequirements {
    /// Compatibility schema version.
    pub schema_version: CompatibilitySchemaVersion,

    /// Checkpoint identity.
    pub checkpoint_id: CheckpointId,

    /// Checkpoint schema/version.
    pub checkpoint_schema_version: String,

    /// Canonical IR schema/version.
    pub ir_schema_version: String,

    /// Program identity.
    pub program_identity: String,

    /// Required checkpoint boundary.
    pub boundary: CheckpointBoundary,

    /// State kind.
    pub state_kind: CheckpointStateKind,

    /// Required restoration mechanism.
    pub restoration: RestorationRequirement,

    /// Resource requirements.
    pub resources: ResourceRequirements,

    /// Operation requirements.
    pub operations: OperationRequirements,

    /// Topology requirements.
    pub topology: TopologyRequirements,

    /// Timing requirements.
    pub timing: TimingRequirements,

    /// QEC requirements.
    pub qec: QecRequirements,

    /// Security requirements.
    pub security: SecurityRequirements,

    /// Generic extensible capabilities.
    pub capabilities: CapabilitySet,

    /// Whether physical mapping is semantically fixed.
    ///
    /// Normally this is false for portable/replayable checkpoints.
    pub physical_mapping_required: bool,

    /// Whether migration is allowed by the checkpoint contract.
    pub migration_allowed: bool,

    /// Whether adaptation is allowed by the checkpoint contract.
    pub adaptation_allowed: bool,
}

impl CheckpointCompatibilityRequirements {
    /// Creates a requirements object.
    pub fn new(
        checkpoint_id: CheckpointId,
        checkpoint_schema_version: impl Into<String>,
        ir_schema_version: impl Into<String>,
        program_identity: impl Into<String>,
        boundary: CheckpointBoundary,
        state_kind: CheckpointStateKind,
        restoration: RestorationRequirement,
    ) -> Result<Self, CompatibilityError> {
        let checkpoint_schema_version = checkpoint_schema_version.into();
        let ir_schema_version = ir_schema_version.into();
        let program_identity = program_identity.into();

        validate_non_empty(
            &checkpoint_schema_version,
            "checkpoint_schema_version",
        )?;

        validate_non_empty(&ir_schema_version, "ir_schema_version")?;
        validate_non_empty(&program_identity, "program_identity")?;

        if !state_kind.is_restorable() {
            return Err(CompatibilityError::StateNotRestorable);
        }

        Ok(Self {
            schema_version: CompatibilitySchemaVersion::CURRENT,
            checkpoint_id,
            checkpoint_schema_version,
            ir_schema_version,
            program_identity,
            boundary,
            state_kind,
            restoration,
            resources: ResourceRequirements::unconstrained(),
            operations: OperationRequirements::new(),
            topology: TopologyRequirements::none(),
            timing: TimingRequirements::none(),
            qec: QecRequirements::none(),
            security: SecurityRequirements::none(),
            capabilities: CapabilitySet::new(),
            physical_mapping_required: false,
            migration_allowed: true,
            adaptation_allowed: true,
        })
    }

    /// Validates all requirements.
    pub fn validate(&self) -> Result<(), CompatibilityError> {
        validate_non_empty(
            &self.checkpoint_schema_version,
            "checkpoint_schema_version",
        )?;

        validate_non_empty(&self.ir_schema_version, "ir_schema_version")?;
        validate_non_empty(&self.program_identity, "program_identity")?;

        if !self.state_kind.is_restorable() {
            return Err(CompatibilityError::StateNotRestorable);
        }

        self.resources.validate()?;
        self.operations.validate()?;
        self.qec.validate()?;
        self.security.validate()?;

        for value in self.capabilities.values.values() {
            value.validate()?;
        }

        if self.boundary == CheckpointBoundary::ProviderSupportedSnapshot
            && !self.restoration.requires_native_support()
        {
            return Err(
                CompatibilityError::InvalidRestorationRequirement,
            );
        }

        Ok(())
    }
}

// ============================================================================
// Target compatibility description
// ============================================================================

/// Complete capability description supplied by the target environment.
///
/// This structure deliberately does not contain a backend/provider enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityTarget {
    /// Compatibility schema version.
    pub schema_version: CompatibilitySchemaVersion,

    /// Stable target identity.
    pub target_id: String,

    /// Checkpoint schema versions supported by the target.
    pub checkpoint_schema_versions: BTreeSet<String>,

    /// IR schema versions supported by the target.
    pub ir_schema_versions: BTreeSet<String>,

    /// Program identities are not normally restricted. When a target
    /// explicitly restricts them, this set may be populated.
    pub program_identities: Option<BTreeSet<String>>,

    /// Restoration capabilities.
    pub restoration: TargetRestorationCapabilities,

    /// Resource availability.
    pub resources: TargetResources,

    /// Supported operations.
    pub operations: TargetOperations,

    /// Topology capability.
    pub topology: TargetTopology,

    /// Timing capability.
    pub timing: TargetTiming,

    /// QEC capability.
    pub qec: TargetQec,

    /// Security capability.
    pub security: TargetSecurity,

    /// Generic capabilities.
    pub capabilities: CapabilitySet,

    /// Whether the target supports physical remapping.
    pub remapping_supported: bool,

    /// Whether the target supports execution migration.
    pub migration_supported: bool,

    /// Whether this target can accept adapted checkpoints.
    pub adaptation_supported: bool,
}

impl CompatibilityTarget {
    /// Creates an empty target description.
    pub fn new(
        target_id: impl Into<String>,
    ) -> Result<Self, CompatibilityError> {
        let target_id = target_id.into();

        validate_non_empty(&target_id, "target_id")?;

        Ok(Self {
            schema_version: CompatibilitySchemaVersion::CURRENT,
            target_id,
            checkpoint_schema_versions: BTreeSet::new(),
            ir_schema_versions: BTreeSet::new(),
            program_identities: None,
            restoration: TargetRestorationCapabilities::unsupported(),
            resources: TargetResources::unconstrained(),
            operations: TargetOperations::new(),
            topology: TargetTopology::unknown(),
            timing: TargetTiming::unknown(),
            qec: TargetQec::unknown(),
            security: TargetSecurity::unknown(),
            capabilities: CapabilitySet::new(),
            remapping_supported: false,
            migration_supported: false,
            adaptation_supported: false,
        })
    }

    /// Validates structural target metadata.
    pub fn validate(&self) -> Result<(), CompatibilityError> {
        validate_non_empty(&self.target_id, "target_id")?;

        if self.schema_version.major == 0 {
            return Err(CompatibilityError::InvalidSchemaVersion);
        }

        if self
            .checkpoint_schema_versions
            .iter()
            .any(|value| value.trim().is_empty())
        {
            return Err(CompatibilityError::InvalidSchemaVersion);
        }

        if self
            .ir_schema_versions
            .iter()
            .any(|value| value.trim().is_empty())
        {
            return Err(CompatibilityError::InvalidSchemaVersion);
        }

        if let Some(programs) = &self.program_identities {
            if programs.iter().any(|value| value.trim().is_empty()) {
                return Err(CompatibilityError::InvalidProgramIdentity);
            }
        }

        Ok(())
    }
}

// ============================================================================
// Compatibility finding
// ============================================================================

/// Result of one compatibility-dimension check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityFinding {
    /// Dimension checked.
    pub dimension: CompatibilityDimension,

    /// Requirement strength.
    pub strength: RequirementStrength,

    /// Whether the check succeeded.
    pub satisfied: bool,

    /// Whether the result is unknown due to insufficient information.
    pub unknown: bool,

    /// Stable machine-readable reason code.
    pub code: CompatibilityReasonCode,

    /// Human-readable explanation.
    ///
    /// This is diagnostic information and must not be treated as a machine
    /// protocol.
    pub message: String,
}

impl CompatibilityFinding {
    /// Creates a satisfied finding.
    fn satisfied(
        dimension: CompatibilityDimension,
        strength: RequirementStrength,
        code: CompatibilityReasonCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            dimension,
            strength,
            satisfied: true,
            unknown: false,
            code,
            message: message.into(),
        }
    }

    /// Creates a failed finding.
    fn failed(
        dimension: CompatibilityDimension,
        strength: RequirementStrength,
        code: CompatibilityReasonCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            dimension,
            strength,
            satisfied: false,
            unknown: false,
            code,
            message: message.into(),
        }
    }

    /// Creates an unknown finding.
    fn unknown(
        dimension: CompatibilityDimension,
        strength: RequirementStrength,
        code: CompatibilityReasonCode,
        message: impl Into<String>,
    ) -> Self {
        Self {
            dimension,
            strength,
            satisfied: false,
            unknown: true,
            code,
            message: message.into(),
        }
    }
}

// ============================================================================
// Stable reason codes
// ============================================================================

/// Stable machine-readable compatibility reason.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub enum CompatibilityReasonCode {
    /// Requirement is satisfied.
    Satisfied,

    /// Compatibility schema is incompatible.
    SchemaIncompatible,

    /// Required checkpoint schema is unsupported.
    CheckpointSchemaUnsupported,

    /// Required IR schema is unsupported.
    IrSchemaUnsupported,

    /// Program identity is incompatible.
    ProgramIdentityMismatch,

    /// Restoration is unsupported.
    RestorationUnsupported,

    /// Required state is not restorable.
    StateNotRestorable,

    /// Logical resources are insufficient.
    LogicalResourcesInsufficient,

    /// Physical resources are insufficient.
    PhysicalResourcesInsufficient,

    /// Classical memory is insufficient.
    ClassicalMemoryInsufficient,

    /// Execution channels are insufficient.
    ExecutionChannelsInsufficient,

    /// Storage capacity is insufficient.
    StorageInsufficient,

    /// Required operation is unsupported.
    OperationUnsupported,

    /// Topology information is unavailable.
    TopologyUnknown,

    /// Topology cannot satisfy the requirement.
    TopologyIncompatible,

    /// Timing information is unavailable.
    TimingUnknown,

    /// Timing constraints cannot be satisfied.
    TimingIncompatible,

    /// QEC is unsupported.
    QecUnsupported,

    /// QEC configuration is incompatible.
    QecIncompatible,

    /// Security requirement is unsupported.
    SecurityUnsupported,

    /// Required generic capability is missing.
    CapabilityMissing,

    /// Required generic capability has an incompatible value.
    CapabilityIncompatible,

    /// Physical mapping must be preserved.
    PhysicalMappingRequired,

    /// Remapping is available and can resolve the mapping difference.
    RemappingRequired,

    /// Migration is required.
    MigrationRequired,

    /// Adaptation is required.
    AdaptationRequired,

    /// Compatibility depends on runtime verification.
    RuntimeVerificationRequired,

    /// Invalid compatibility metadata.
    InvalidMetadata,
}

// ============================================================================
// Compatibility assessment
// ============================================================================

/// Complete compatibility assessment.
///
/// The assessment is immutable after construction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityAssessment {
    /// Schema version used for the assessment.
    pub schema_version: CompatibilitySchemaVersion,

    /// Checkpoint identity.
    pub checkpoint_id: CheckpointId,

    /// Target identity.
    pub target_id: String,

    /// Overall compatibility level.
    pub level: CompatibilityLevel,

    /// Whether all mandatory dimensions passed.
    pub mandatory_requirements_satisfied: bool,

    /// Whether any mandatory dimension is unknown.
    pub has_unknown_mandatory_requirements: bool,

    /// Whether adaptation is required.
    pub adaptation_required: bool,

    /// Whether migration is required.
    pub migration_required: bool,

    /// Whether runtime verification is required before restoration.
    pub runtime_verification_required: bool,

    /// Deterministically ordered findings.
    pub findings: Arc<[CompatibilityFinding]>,
}

impl CompatibilityAssessment {
    /// Returns true when restoration may proceed to policy/authorization gates.
    ///
    /// This deliberately does NOT mean that restoration is authorized.
    #[must_use]
    pub const fn may_proceed_to_authorization(&self) -> bool {
        self.mandatory_requirements_satisfied
            && !self.has_unknown_mandatory_requirements
            && !matches!(
                self.level,
                CompatibilityLevel::Incompatible
                    | CompatibilityLevel::Unknown
            )
    }

    /// Returns the first mandatory failure, if any.
    #[must_use]
    pub fn first_mandatory_failure(
        &self,
    ) -> Option<&CompatibilityFinding> {
        self.findings.iter().find(|finding| {
            finding.strength == RequirementStrength::Required
                && !finding.satisfied
                && !finding.unknown
        })
    }

    /// Returns whether a particular dimension passed.
    #[must_use]
    pub fn dimension_satisfied(
        &self,
        dimension: CompatibilityDimension,
    ) -> bool {
        self.findings.iter().any(|finding| {
            finding.dimension == dimension && finding.satisfied
        })
    }
}

// ============================================================================
// Compatibility engine
// ============================================================================

/// Stateless deterministic compatibility engine.
///
/// The engine performs no I/O, hardware discovery, routing, scheduling,
/// migration, QEC, or restoration.
///
/// Those operations belong to their owning subsystems.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompatibilityEngine;

impl CompatibilityEngine {
    /// Creates a compatibility engine.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Assesses checkpoint requirements against target capabilities.
    ///
    /// The function is deterministic for identical inputs.
    pub fn assess(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> Result<CompatibilityAssessment, CompatibilityError> {
        requirements.validate()?;
        target.validate()?;

        let mut findings = Vec::new();

        findings.push(self.check_schema(
            requirements,
            target,
        ));

        findings.push(self.check_checkpoint_schema(
            requirements,
            target,
        ));

        findings.push(self.check_ir_schema(
            requirements,
            target,
        ));

        findings.push(self.check_program(
            requirements,
            target,
        ));

        findings.push(self.check_state(
            requirements,
            target,
        ));

        findings.push(self.check_restoration(
            requirements,
            target,
        ));

        findings.extend(self.check_resources(
            requirements,
            target,
        ));

        findings.extend(self.check_operations(
            requirements,
            target,
        ));

        findings.push(self.check_topology(
            requirements,
            target,
        ));

        findings.push(self.check_timing(
            requirements,
            target,
        ));

        findings.extend(self.check_qec(
            requirements,
            target,
        ));

        findings.extend(self.check_security(
            requirements,
            target,
        ));

        findings.extend(self.check_generic_capabilities(
            requirements,
            target,
        ));

        findings.push(self.check_mapping(
            requirements,
            target,
        ));

        let mandatory_failed = findings.iter().any(|finding| {
            finding.strength == RequirementStrength::Required
                && !finding.satisfied
                && !finding.unknown
        });

        let mandatory_unknown = findings.iter().any(|finding| {
            finding.strength == RequirementStrength::Required
                && finding.unknown
        });

        let adaptation_required = findings.iter().any(|finding| {
            matches!(
                finding.code,
                CompatibilityReasonCode::AdaptationRequired
                    | CompatibilityReasonCode::RemappingRequired
                    | CompatibilityReasonCode::TimingIncompatible
                    | CompatibilityReasonCode::TopologyIncompatible
            )
        });

        let migration_required = findings.iter().any(|finding| {
            finding.code == CompatibilityReasonCode::MigrationRequired
        });

        let runtime_verification_required = findings.iter().any(|finding| {
            finding.code
                == CompatibilityReasonCode::RuntimeVerificationRequired
        });

        let level = if mandatory_failed {
            CompatibilityLevel::Incompatible
        } else if mandatory_unknown {
            CompatibilityLevel::Unknown
        } else if migration_required {
            CompatibilityLevel::CompatibleWithMigration
        } else if adaptation_required {
            CompatibilityLevel::CompatibleWithAdaptation
        } else if runtime_verification_required {
            CompatibilityLevel::ConditionallyCompatible
        } else {
            CompatibilityLevel::Compatible
        };

        Ok(CompatibilityAssessment {
            schema_version: CompatibilitySchemaVersion::CURRENT,
            checkpoint_id: requirements.checkpoint_id.clone(),
            target_id: target.target_id.clone(),
            level,
            mandatory_requirements_satisfied: !mandatory_failed,
            has_unknown_mandatory_requirements: mandatory_unknown,
            adaptation_required,
            migration_required,
            runtime_verification_required,
            findings: Arc::from(findings),
        })
    }

    fn check_schema(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> CompatibilityFinding {
        if requirements.schema_version.same_major(target.schema_version) {
            CompatibilityFinding::satisfied(
                CompatibilityDimension::CheckpointSchema,
                RequirementStrength::Required,
                CompatibilityReasonCode::Satisfied,
                "Compatibility schemas share a compatible major version.",
            )
        } else {
            CompatibilityFinding::failed(
                CompatibilityDimension::CheckpointSchema,
                RequirementStrength::Required,
                CompatibilityReasonCode::SchemaIncompatible,
                "Compatibility schema major versions are incompatible.",
            )
        }
    }

    fn check_checkpoint_schema(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> CompatibilityFinding {
        if target
            .checkpoint_schema_versions
            .contains(&requirements.checkpoint_schema_version)
        {
            CompatibilityFinding::satisfied(
                CompatibilityDimension::CheckpointSchema,
                RequirementStrength::Required,
                CompatibilityReasonCode::Satisfied,
                "Target supports the checkpoint schema.",
            )
        } else {
            CompatibilityFinding::failed(
                CompatibilityDimension::CheckpointSchema,
                RequirementStrength::Required,
                CompatibilityReasonCode::CheckpointSchemaUnsupported,
                "Target does not advertise support for the checkpoint schema.",
            )
        }
    }

    fn check_ir_schema(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> CompatibilityFinding {
        if target
            .ir_schema_versions
            .contains(&requirements.ir_schema_version)
        {
            CompatibilityFinding::satisfied(
                CompatibilityDimension::IrSchema,
                RequirementStrength::Required,
                CompatibilityReasonCode::Satisfied,
                "Target supports the checkpoint IR schema.",
            )
        } else {
            CompatibilityFinding::failed(
                CompatibilityDimension::IrSchema,
                RequirementStrength::Required,
                CompatibilityReasonCode::IrSchemaUnsupported,
                "Target does not advertise support for the checkpoint IR schema.",
            )
        }
    }

    fn check_program(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> CompatibilityFinding {
        match &target.program_identities {
            None => CompatibilityFinding::satisfied(
                CompatibilityDimension::Program,
                RequirementStrength::Required,
                CompatibilityReasonCode::Satisfied,
                "Target does not restrict program identities.",
            ),
            Some(programs) if programs.contains(
                &requirements.program_identity,
            ) => CompatibilityFinding::satisfied(
                CompatibilityDimension::Program,
                RequirementStrength::Required,
                CompatibilityReasonCode::Satisfied,
                "Program identity is accepted by the target.",
            ),
            Some(_) => CompatibilityFinding::failed(
                CompatibilityDimension::Program,
                RequirementStrength::Required,
                CompatibilityReasonCode::ProgramIdentityMismatch,
                "Target does not accept this program identity.",
            ),
        }
    }

    fn check_state(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        _target: &CompatibilityTarget,
    ) -> CompatibilityFinding {
        if requirements.state_kind.is_restorable() {
            CompatibilityFinding::satisfied(
                CompatibilityDimension::StateSemantics,
                RequirementStrength::Required,
                CompatibilityReasonCode::Satisfied,
                "Checkpoint state has an explicit restoration interpretation.",
            )
        } else {
            CompatibilityFinding::failed(
                CompatibilityDimension::StateSemantics,
                RequirementStrength::Required,
                CompatibilityReasonCode::StateNotRestorable,
                "Checkpoint state is not declared restorable.",
            )
        }
    }

    fn check_restoration(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> CompatibilityFinding {
        if !target.restoration.restoration_supported {
            return CompatibilityFinding::failed(
                CompatibilityDimension::Restoration,
                RequirementStrength::Required,
                CompatibilityReasonCode::RestorationUnsupported,
                "Target does not support checkpoint restoration.",
            );
        }

        if target.restoration.supports(requirements.restoration) {
            return CompatibilityFinding::satisfied(
                CompatibilityDimension::Restoration,
                RequirementStrength::Required,
                CompatibilityReasonCode::Satisfied,
                "Target supports the required restoration mechanism.",
            );
        }

        if requirements.adaptation_allowed
            && target.adaptation_supported
        {
            return CompatibilityFinding::failed(
                CompatibilityDimension::Restoration,
                RequirementStrength::Required,
                CompatibilityReasonCode::AdaptationRequired,
                "Target requires an adaptation before restoration.",
            );
        }

        if requirements.migration_allowed
            && target.migration_supported
        {
            return CompatibilityFinding::failed(
                CompatibilityDimension::Required,
                RequirementStrength::Required,
                CompatibilityReasonCode::MigrationRequired,
                "Restoration requires migration to another compatible target.",
            );
        }

        CompatibilityFinding::failed(
            CompatibilityDimension::Restoration,
            RequirementStrength::Required,
            CompatibilityReasonCode::RestorationUnsupported,
            "Target cannot provide the required restoration mechanism.",
        )
    }

    fn check_resources(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> Vec<CompatibilityFinding> {
        let mut findings = Vec::new();

        check_resource(
            &mut findings,
            CompatibilityDimension::LogicalResources,
            RequirementStrength::Required,
            requirements.resources.logical_qubits,
            target.resources.logical_qubits,
            CompatibilityReasonCode::LogicalResourcesInsufficient,
            "logical qubits",
        );

        check_resource(
            &mut findings,
            CompatibilityDimension::PhysicalResources,
            RequirementStrength::Required,
            requirements.resources.physical_qubits,
            target.resources.physical_qubits,
            CompatibilityReasonCode::PhysicalResourcesInsufficient,
            "physical qubits",
        );

        check_resource(
            &mut findings,
            CompatibilityDimension::Execution,
            RequirementStrength::Required,
            requirements.resources.classical_memory_bytes,
            target.resources.classical_memory_bytes,
            CompatibilityReasonCode::ClassicalMemoryInsufficient,
            "classical memory bytes",
        );

        check_resource(
            &mut findings,
            CompatibilityDimension::Execution,
            RequirementStrength::Required,
            requirements.resources.execution_channels,
            target.resources.execution_channels,
            CompatibilityReasonCode::ExecutionChannelsInsufficient,
            "execution channels",
        );

        check_resource(
            &mut findings,
            CompatibilityDimension::Storage,
            RequirementStrength::Required,
            requirements.resources.storage_bytes,
            target.resources.storage_bytes,
            CompatibilityReasonCode::StorageInsufficient,
            "storage bytes",
        );

        findings
    }

    fn check_operations(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> Vec<CompatibilityFinding> {
        requirements
            .operations
            .required
            .iter()
            .map(|operation| {
                if target.operations.contains(operation) {
                    CompatibilityFinding::satisfied(
                        CompatibilityDimension::Operations,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::Satisfied,
                        format!(
                            "Required operation `{operation}` is supported."
                        ),
                    )
                } else if requirements.adaptation_allowed
                    && target.adaptation_supported
                {
                    CompatibilityFinding::failed(
                        CompatibilityDimension::Operations,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::AdaptationRequired,
                        format!(
                            "Operation `{operation}` requires target adaptation."
                        ),
                    )
                } else {
                    CompatibilityFinding::failed(
                        CompatibilityDimension::Operations,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::OperationUnsupported,
                        format!(
                            "Required operation `{operation}` is unsupported."
                        ),
                    )
                }
            })
            .collect()
    }

    fn check_topology(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> CompatibilityFinding {
        if !requirements.topology.required {
            return CompatibilityFinding::satisfied(
                CompatibilityDimension::Topology,
                RequirementStrength::Optional,
                CompatibilityReasonCode::Satisfied,
                "No topology requirement was declared.",
            );
        }

        if !target.topology.available {
            return CompatibilityFinding::unknown(
                CompatibilityDimension::Topology,
                RequirementStrength::Required,
                CompatibilityReasonCode::TopologyUnknown,
                "Target topology information is unavailable.",
            );
        }

        if let (
            Some(required),
            Some(available),
        ) = (
            requirements.topology.required_interactions,
            target.topology.available_interactions,
        ) {
            if available < required {
                if requirements.topology.routing_supported
                    && target.topology.routing_supported
                    && requirements.adaptation_allowed
                {
                    return CompatibilityFinding::failed(
                        CompatibilityDimension::Topology,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::AdaptationRequired,
                        "Target topology requires routing/adaptation.",
                    );
                }

                return CompatibilityFinding::failed(
                    CompatibilityDimension::Topology,
                    RequirementStrength::Required,
                    CompatibilityReasonCode::TopologyIncompatible,
                    "Target topology cannot satisfy the required interactions.",
                );
            }
        }

        if requirements.topology.routing_supported
            && !target.topology.routing_supported
        {
            return CompatibilityFinding::failed(
                CompatibilityDimension::Topology,
                RequirementStrength::Required,
                CompatibilityReasonCode::TopologyIncompatible,
                "Checkpoint requires routing but target does not advertise routing support.",
            );
        }

        CompatibilityFinding::satisfied(
            CompatibilityDimension::Topology,
            RequirementStrength::Required,
            CompatibilityReasonCode::Satisfied,
            "Target topology satisfies the declared requirement.",
        )
    }

    fn check_timing(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> CompatibilityFinding {
        if !requirements.timing.required {
            return CompatibilityFinding::satisfied(
                CompatibilityDimension::Timing,
                RequirementStrength::Optional,
                CompatibilityReasonCode::Satisfied,
                "No timing requirement was declared.",
            );
        }

        if !target.timing.available {
            return CompatibilityFinding::unknown(
                CompatibilityDimension::Timing,
                RequirementStrength::Required,
                CompatibilityReasonCode::TimingUnknown,
                "Target timing information is unavailable.",
            );
        }

        if let (
            Some(required),
            Some(available),
        ) = (
            requirements.timing.maximum_execution_duration_ns,
            target.timing.maximum_execution_duration_ns,
        ) {
            if available < required {
                if requirements.timing.rescheduling_allowed
                    && target.timing.rescheduling_supported
                {
                    return CompatibilityFinding::failed(
                        CompatibilityDimension::Timing,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::AdaptationRequired,
                        "Target requires schedule adaptation.",
                    );
                }

                return CompatibilityFinding::failed(
                    CompatibilityDimension::Timing,
                    RequirementStrength::Required,
                    CompatibilityReasonCode::TimingIncompatible,
                    "Target timing cannot satisfy the checkpoint requirement.",
                );
            }
        }

        CompatibilityFinding::satisfied(
            CompatibilityDimension::Timing,
            RequirementStrength::Required,
            CompatibilityReasonCode::Satisfied,
            "Target timing satisfies the declared requirement.",
        )
    }

    fn check_qec(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> Vec<CompatibilityFinding> {
        let mut findings = Vec::new();

        if !requirements.qec.required {
            findings.push(CompatibilityFinding::satisfied(
                CompatibilityDimension::Qec,
                RequirementStrength::Optional,
                CompatibilityReasonCode::Satisfied,
                "No QEC restoration requirement was declared.",
            ));

            return findings;
        }

        if !target.qec.supported {
            findings.push(CompatibilityFinding::failed(
                CompatibilityDimension::Qec,
                RequirementStrength::Required,
                CompatibilityReasonCode::QecUnsupported,
                "Target does not support the required QEC restoration.",
            ));

            return findings;
        }

        if let Some(code) = &requirements.qec.code_family {
            if !target.qec.code_families.contains(code) {
                if requirements.qec.adaptation_allowed
                    && target.qec.adaptation_supported
                {
                    findings.push(CompatibilityFinding::failed(
                        CompatibilityDimension::Qec,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::AdaptationRequired,
                        "QEC code adaptation is required.",
                    ));
                } else {
                    findings.push(CompatibilityFinding::failed(
                        CompatibilityDimension::Qec,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::QecIncompatible,
                        "Target does not support the required QEC code.",
                    ));
                }
            }
        }

        if let (
            Some(required),
            Some(available),
        ) = (
            requirements.qec.logical_qubits,
            target.qec.logical_qubits,
        ) {
            if available < required {
                findings.push(CompatibilityFinding::failed(
                    CompatibilityDimension::Qec,
                    RequirementStrength::Required,
                    CompatibilityReasonCode::LogicalResourcesInsufficient,
                    "Target has insufficient logical QEC resources.",
                ));
            }
        }

        if let (
            Some(required),
            Some(available),
        ) = (
            requirements.qec.minimum_distance,
            target.qec.maximum_distance,
        ) {
            if available < required {
                if requirements.qec.adaptation_allowed
                    && target.qec.adaptation_supported
                {
                    findings.push(CompatibilityFinding::failed(
                        CompatibilityDimension::Qec,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::AdaptationRequired,
                        "QEC distance adaptation is required.",
                    ));
                } else {
                    findings.push(CompatibilityFinding::failed(
                        CompatibilityDimension::Qec,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::QecIncompatible,
                        "Target cannot satisfy the required QEC distance.",
                    ));
                }
            }
        }

        if let Some(decoder) = &requirements.qec.decoder {
            if !target.qec.decoders.contains(decoder) {
                if requirements.qec.adaptation_allowed
                    && target.qec.adaptation_supported
                {
                    findings.push(CompatibilityFinding::failed(
                        CompatibilityDimension::Qec,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::AdaptationRequired,
                        "A different compatible QEC decoder is required.",
                    ));
                } else {
                    findings.push(CompatibilityFinding::failed(
                        CompatibilityDimension::Qec,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::QecIncompatible,
                        "Target does not support the required QEC decoder.",
                    ));
                }
            }
        }

        if findings.is_empty() {
            findings.push(CompatibilityFinding::satisfied(
                CompatibilityDimension::Qec,
                RequirementStrength::Required,
                CompatibilityReasonCode::Satisfied,
                "Target satisfies the declared QEC requirements.",
            ));
        }

        findings
    }

    fn check_security(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> Vec<CompatibilityFinding> {
        let mut findings = Vec::new();

        if requirements.security.authenticated_metadata_required {
            if target.security.authenticated_metadata_supported {
                findings.push(CompatibilityFinding::satisfied(
                    CompatibilityDimension::Security,
                    RequirementStrength::Required,
                    CompatibilityReasonCode::Satisfied,
                    "Target supports authenticated checkpoint metadata.",
                ));
            } else {
                findings.push(CompatibilityFinding::failed(
                    CompatibilityDimension::Security,
                    RequirementStrength::Required,
                    CompatibilityReasonCode::SecurityUnsupported,
                    "Target cannot satisfy authenticated checkpoint metadata requirements.",
                ));
            }
        }

        if requirements.security.encrypted_payload_required {
            if target.security.encrypted_payload_supported {
                findings.push(CompatibilityFinding::satisfied(
                    CompatibilityDimension::Security,
                    RequirementStrength::Required,
                    CompatibilityReasonCode::Satisfied,
                    "Target supports encrypted checkpoint payloads.",
                ));
            } else {
                findings.push(CompatibilityFinding::failed(
                    CompatibilityDimension::Security,
                    RequirementStrength::Required,
                    CompatibilityReasonCode::SecurityUnsupported,
                    "Target cannot restore encrypted checkpoint payloads.",
                ));
            }
        }

        if requirements.security.trusted_execution_required {
            if target.security.trusted_execution_supported {
                findings.push(CompatibilityFinding::satisfied(
                    CompatibilityDimension::Security,
                    RequirementStrength::Required,
                    CompatibilityReasonCode::Satisfied,
                    "Target provides the required trusted execution capability.",
                ));
            } else {
                findings.push(CompatibilityFinding::failed(
                    CompatibilityDimension::Security,
                    RequirementStrength::Required,
                    CompatibilityReasonCode::SecurityUnsupported,
                    "Target does not provide the required trusted execution capability.",
                ));
            }
        }

        if let Some(profile) = &requirements.security.profile {
            if target.security.profiles.contains(profile) {
                findings.push(CompatibilityFinding::satisfied(
                    CompatibilityDimension::Security,
                    RequirementStrength::Required,
                    CompatibilityReasonCode::Satisfied,
                    "Target supports the required security profile.",
                ));
            } else {
                findings.push(CompatibilityFinding::failed(
                    CompatibilityDimension::Security,
                    RequirementStrength::Required,
                    CompatibilityReasonCode::SecurityUnsupported,
                    "Target does not advertise the required security profile.",
                ));
            }
        }

        if findings.is_empty() {
            findings.push(CompatibilityFinding::satisfied(
                CompatibilityDimension::Security,
                RequirementStrength::Optional,
                CompatibilityReasonCode::Satisfied,
                "No additional security compatibility requirement was declared.",
            ));
        }

        findings
    }

    fn check_generic_capabilities(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> Vec<CompatibilityFinding> {
        requirements
            .capabilities
            .values
            .iter()
            .map(|(name, required)| {
                match target.capabilities.get(name) {
                    None => CompatibilityFinding::failed(
                        CompatibilityDimension::Execution,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::CapabilityMissing,
                        format!(
                            "Required capability `{name}` is not advertised."
                        ),
                    ),
                    Some(actual) if actual == required => {
                        CompatibilityFinding::satisfied(
                            CompatibilityDimension::Execution,
                            RequirementStrength::Required,
                            CompatibilityReasonCode::Satisfied,
                            format!(
                                "Required capability `{name}` is satisfied."
                            ),
                        )
                    }
                    Some(_) => CompatibilityFinding::failed(
                        CompatibilityDimension::Execution,
                        RequirementStrength::Required,
                        CompatibilityReasonCode::CapabilityIncompatible,
                        format!(
                            "Capability `{name}` has an incompatible value."
                        ),
                    ),
                }
            })
            .collect()
    }

    fn check_mapping(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> CompatibilityFinding {
        if !requirements.physical_mapping_required {
            return CompatibilityFinding::satisfied(
                CompatibilityDimension::PhysicalResources,
                RequirementStrength::Optional,
                CompatibilityReasonCode::Satisfied,
                "Checkpoint does not require the original physical mapping.",
            );
        }

        if target.remapping_supported {
            CompatibilityFinding::failed(
                CompatibilityDimension::PhysicalResources,
                RequirementStrength::Required,
                CompatibilityReasonCode::RemappingRequired,
                "Checkpoint physical mapping differs and remapping support is required.",
            )
        } else {
            CompatibilityFinding::failed(
                CompatibilityDimension::PhysicalResources,
                RequirementStrength::Required,
                CompatibilityReasonCode::PhysicalMappingRequired,
                "Checkpoint requires preservation of physical mapping.",
            )
        }
    }
}

// ============================================================================
// Resource helper
// ============================================================================

fn check_resource(
    findings: &mut Vec<CompatibilityFinding>,
    dimension: CompatibilityDimension,
    strength: RequirementStrength,
    required: Option<u64>,
    available: Option<u64>,
    failure_code: CompatibilityReasonCode,
    name: &str,
) {
    let Some(required) = required else {
        findings.push(CompatibilityFinding::satisfied(
            dimension,
            RequirementStrength::Optional,
            CompatibilityReasonCode::Satisfied,
            format!("No {name} requirement was declared."),
        ));
        return;
    };

    match available {
        Some(available) if available >= required => {
            findings.push(CompatibilityFinding::satisfied(
                dimension,
                strength,
                CompatibilityReasonCode::Satisfied,
                format!(
                    "Target provides sufficient {name}."
                ),
            ));
        }
        Some(_) => {
            findings.push(CompatibilityFinding::failed(
                dimension,
                strength,
                failure_code,
                format!(
                    "Target does not provide sufficient {name}."
                ),
            ));
        }
        None => {
            findings.push(CompatibilityFinding::unknown(
                dimension,
                strength,
                failure_code,
                format!(
                    "Target did not advertise available {name}."
                ),
            ));
        }
    }
}

// ============================================================================
// Checkpoint conversion
// ============================================================================

/// Extracts compatibility requirements from a canonical checkpoint.
///
/// This conversion intentionally performs only structural interpretation.
/// It does not inspect storage, read payloads, calculate hashes, or perform
/// hardware discovery.
pub fn requirements_from_checkpoint(
    checkpoint: &Checkpoint,
) -> Result<CheckpointCompatibilityRequirements, CompatibilityError> {
    let checkpoint_id = checkpoint.id().clone();

    let requirements = CheckpointCompatibilityRequirements::new(
        checkpoint_id,
        checkpoint.schema_version().to_string(),
        checkpoint.ir_schema_version().to_string(),
        checkpoint.program_id().to_string(),
        checkpoint.boundary(),
        checkpoint.state_kind(),
        restoration_requirement_from_checkpoint(
            checkpoint.boundary(),
            checkpoint.state_kind(),
        )?,
    )?;

    Ok(requirements)
}

/// Converts checkpoint semantics to the corresponding restoration requirement.
pub fn restoration_requirement_from_checkpoint(
    boundary: CheckpointBoundary,
    state_kind: CheckpointStateKind,
) -> Result<RestorationRequirement, CompatibilityError> {
    if !state_kind.is_restorable() {
        return Err(CompatibilityError::StateNotRestorable);
    }

    let requirement = match boundary {
        CheckpointBoundary::ProgramStart => {
            RestorationRequirement::DeterministicReplay
        }
        CheckpointBoundary::ClassicalExecution => {
            RestorationRequirement::DirectClassical
        }
        CheckpointBoundary::Measurement => {
            RestorationRequirement::MeasurementReconstruction
        }
        CheckpointBoundary::LogicalQec => {
            RestorationRequirement::LogicalQecRestore
        }
        CheckpointBoundary::ProviderSupportedSnapshot => {
            RestorationRequirement::ProviderNativeRestore
        }
        CheckpointBoundary::ReconstructibleRuntime => {
            RestorationRequirement::RuntimeReconstruction
        }
        CheckpointBoundary::CompiledExecution => {
            RestorationRequirement::CompiledReplay
        }
    };

    Ok(requirement)
}

// ============================================================================
// Trait contract
// ============================================================================

/// Public compatibility-validation contract.
///
/// Recovery orchestration can depend on this trait without depending on the
/// concrete compatibility engine.
///
/// This allows future implementations such as:
///
///     CompatibilityEngine
///     PolicyAwareCompatibilityEngine
///     DistributedCompatibilityEngine
///     ProviderAdapterCompatibilityEngine
///
/// without changing recovery orchestration.
pub trait CheckpointCompatibilityValidator:
    Send + Sync
{
    /// Validates compatibility.
    fn validate(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> Result<CompatibilityAssessment, CompatibilityError>;
}

impl CheckpointCompatibilityValidator for CompatibilityEngine {
    fn validate(
        &self,
        requirements: &CheckpointCompatibilityRequirements,
        target: &CompatibilityTarget,
    ) -> Result<CompatibilityAssessment, CompatibilityError> {
        self.assess(requirements, target)
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Compatibility-specific error taxonomy.
///
/// These errors describe malformed compatibility data or impossible
/// compatibility evaluation. They do not represent recovery failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompatibilityError {
    /// Required identifier or string was empty.
    InvalidIdentifier {
        /// Field that failed validation.
        field: &'static str,
    },

    /// Compatibility metadata was invalid.
    InvalidMetadata,

    /// Schema version was invalid.
    InvalidSchemaVersion,

    /// Program identity was invalid.
    InvalidProgramIdentity,

    /// State cannot be restored.
    StateNotRestorable,

    /// Restoration requirement is inconsistent with checkpoint semantics.
    InvalidRestorationRequirement,

    /// Capability value is malformed.
    InvalidCapabilityValue,

    /// Operation requirement is malformed.
    InvalidOperationRequirement,

    /// Target does not advertise required capability.
    CapabilityMissing,

    /// Target capability has an incompatible value.
    CapabilityIncompatible,
}

impl fmt::Display for CompatibilityError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidIdentifier { field } => {
                write!(formatter, "invalid compatibility identifier: {field}")
            }
            Self::InvalidMetadata => {
                formatter.write_str("invalid compatibility metadata")
            }
            Self::InvalidSchemaVersion => {
                formatter.write_str("invalid compatibility schema version")
            }
            Self::InvalidProgramIdentity => {
                formatter.write_str("invalid program identity")
            }
            Self::StateNotRestorable => {
                formatter.write_str("checkpoint state is not restorable")
            }
            Self::InvalidRestorationRequirement => {
                formatter.write_str(
                    "invalid checkpoint restoration requirement",
                )
            }
            Self::InvalidCapabilityValue => {
                formatter.write_str("invalid capability value")
            }
            Self::InvalidOperationRequirement => {
                formatter.write_str("invalid operation requirement")
            }
            Self::CapabilityMissing => {
                formatter.write_str("required capability is missing")
            }
            Self::CapabilityIncompatible => {
                formatter.write_str("capability value is incompatible")
            }
        }
    }
}

impl std::error::Error for CompatibilityError {}

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_non_empty(
    value: &str,
    field: &'static str,
) -> Result<(), CompatibilityError> {
    if value.trim().is_empty() {
        return Err(CompatibilityError::InvalidIdentifier { field });
    }

    Ok(())
}

// ============================================================================
// Compile-time integration assertions
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn checkpoint_requirements() -> CheckpointCompatibilityRequirements {
        let checkpoint_id =
            CheckpointId::new("checkpoint-test")
                .expect("valid checkpoint id");

        CheckpointCompatibilityRequirements::new(
            checkpoint_id,
            "zamani.quantum.resilience.checkpoint@1",
            "zamani.quantum.ir@1",
            "program-test",
            CheckpointBoundary::ProgramStart,
            CheckpointStateKind::ReplayableProgram,
            RestorationRequirement::DeterministicReplay,
        )
        .expect("valid requirements")
    }

    fn target() -> CompatibilityTarget {
        let mut target =
            CompatibilityTarget::new("target-test")
                .expect("valid target");

        target
            .checkpoint_schema_versions
            .insert(
                "zamani.quantum.resilience.checkpoint@1"
                    .to_owned(),
            );

        target
            .ir_schema_versions
            .insert(
                "zamani.quantum.ir@1".to_owned(),
            );

        target.restoration.restoration_supported = true;
        target.restoration.deterministic_replay_supported = true;
        target.adaptation_supported = true;
        target.remapping_supported = true;
        target.migration_supported = true;

        target
    }

    #[test]
    fn compatible_replay_checkpoint_is_compatible() {
        let engine = CompatibilityEngine::new();
        let requirements = checkpoint_requirements();
        let target = target();

        let assessment = engine
            .assess(&requirements, &target)
            .expect("assessment should succeed");

        assert_eq!(
            assessment.level,
            CompatibilityLevel::Compatible
        );

        assert!(assessment.may_proceed_to_authorization());
    }

    #[test]
    fn unknown_resource_does_not_become_compatible() {
        let engine = CompatibilityEngine::new();

        let mut requirements = checkpoint_requirements();
        requirements.resources.logical_qubits = Some(8);

        let target = target();

        let assessment = engine
            .assess(&requirements, &target)
            .expect("assessment should succeed");

        assert_eq!(
            assessment.level,
            CompatibilityLevel::Unknown
        );

        assert!(!assessment.may_proceed_to_authorization());
        assert!(assessment.has_unknown_mandatory_requirements);
    }

    #[test]
    fn insufficient_resource_is_incompatible() {
        let engine = CompatibilityEngine::new();

        let mut requirements = checkpoint_requirements();
        requirements.resources.logical_qubits = Some(8);

        let mut target = target();
        target.resources.logical_qubits = Some(4);

        let assessment = engine
            .assess(&requirements, &target)
            .expect("assessment should succeed");

        assert_eq!(
            assessment.level,
            CompatibilityLevel::Incompatible
        );

        assert!(!assessment.may_proceed_to_authorization());
    }

    #[test]
    fn provider_native_restore_requires_target_support() {
        let engine = CompatibilityEngine::new();

        let checkpoint_id =
            CheckpointId::new("checkpoint-provider")
                .expect("valid checkpoint id");

        let requirements =
            CheckpointCompatibilityRequirements::new(
                checkpoint_id,
                "zamani.quantum.resilience.checkpoint@1",
                "zamani.quantum.ir@1",
                "program-provider",
                CheckpointBoundary::ProviderSupportedSnapshot,
                CheckpointStateKind::SupportedQuantumSnapshot,
                RestorationRequirement::ProviderNativeRestore,
            )
            .expect("valid requirements");

        let mut target = target();
        target.restoration.deterministic_replay_supported = false;
        target.restoration.provider_native_restore_supported = false;

        let assessment = engine
            .assess(&requirements, &target)
            .expect("assessment should succeed");

        assert_eq!(
            assessment.level,
            CompatibilityLevel::Incompatible
        );
    }

    #[test]
    fn physical_mapping_can_require_adaptation() {
        let engine = CompatibilityEngine::new();

        let mut requirements = checkpoint_requirements();
        requirements.physical_mapping_required = true;

        let mut target = target();
        target.remapping_supported = true;

        let assessment = engine
            .assess(&requirements, &target)
            .expect("assessment should succeed");

        assert_eq!(
            assessment.level,
            CompatibilityLevel::CompatibleWithAdaptation
        );

        assert!(assessment.adaptation_required);
    }

    #[test]
    fn deterministic_assessment_has_stable_findings() {
        let engine = CompatibilityEngine::new();

        let requirements = checkpoint_requirements();
        let target = target();

        let first = engine
            .assess(&requirements, &target)
            .expect("first assessment");

        let second = engine
            .assess(&requirements, &target)
            .expect("second assessment");

        assert_eq!(first, second);
    }

    #[test]
    fn capability_set_is_deterministic() {
        let mut capabilities = CapabilitySet::new();

        capabilities
            .insert(
                "z",
                CapabilityValue::Boolean(true),
            )
            .expect("valid capability");

        capabilities
            .insert(
                "a",
                CapabilityValue::Unsigned(4),
            )
            .expect("valid capability");

        let keys: Vec<&String> =
            capabilities.values.keys().collect();

        assert_eq!(
            keys,
            vec![
                &"a".to_owned(),
                &"z".to_owned()
            ]
        );
    }

    #[test]
    fn canonical_qubit_types_are_the_only_qubit_types() {
        fn assert_logical(_: QubitId) {}
        fn assert_physical(_: PhysicalQubitId) {}

        let _ = (
            assert_logical,
            assert_physical,
        );
    }
}