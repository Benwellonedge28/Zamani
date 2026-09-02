//! Zamani Quantum Noise (ZQN) — Target Requirements
//!
//! # Purpose
//!
//! This module defines the target-independent requirements of a ZQN-aware
//! quantum computation.
//!
//! A `TargetRequirements` value answers:
//
//! > "What capabilities, resource semantics, numerical guarantees,
//! > execution guarantees, and noise semantics must the eventual target
//! > provide for this computation to be realized faithfully under the
//! > selected policy?"
//!
//! It deliberately does NOT select a target.
//!
//! The intended pipeline is:
//
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::ir
//!      │
//!      ▼
//! ZQN semantic requirements
//!      │
//!      ▼
//! TargetRequirements
//!      │
//!      ▼
//! target capabilities
//!      │
//!      ▼
//! compatibility evaluation
//!      │
//!      ├── compatible
//!      │
//!      ├── compatible with declared approximation
//!      │
//!      └── incompatible
//!      │
//!      ▼
//! target lowering / execution
//! ```
//!
//! # Architectural ownership
//!
//! This file owns:
//!
//! - target-independent ZQN execution requirements;
//! - required capability declarations;
//! - resource requirements;
//! - modality requirements;
//! - representation requirements;
//! - numerical requirements;
//! - approximation policy;
//! - determinism requirements;
//! - correlation requirements;
//! - temporal requirements;
//! - calibration requirements;
//! - characterization requirements;
//! - provenance requirements;
//! - execution requirements;
//! - requirement composition;
//! - requirement validation;
//! - requirement identity;
//! - deterministic ordering;
//! - target-independent requirement diagnostics.
//!
//! This file does NOT own:
//!
//! - target discovery;
//! - hardware inventory;
//! - vendor APIs;
//! - QPU credentials;
//! - hardware topology;
//! - target capabilities;
//! - routing;
//! - scheduling;
//! - channel implementation;
//! - noise-model implementation;
//! - simulation implementation;
//! - QEC implementation;
//! - benchmarking implementation;
//! - frontend syntax;
//! - source-language ASTs;
//! - target lowering;
//! - execution.
//!
//! Those responsibilities belong to other layers.
//!
//! # Canonical quantum identity
//!
//! This module does not define another `QubitId` or `PhysicalQubitId`.
//!
//! When a requirement is scoped to a logical or physical quantum resource,
//! it uses the canonical identities owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Specifically:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This follows the established ZQN identity contract.
//!
//! A numeric identifier must never be interpreted as a target capacity.
//!
//! # Write once, scale everywhere
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_PHYSICAL_QUBITS
//! MAX_OPERATIONS
//! MAX_RESOURCES
//! MAX_CAPABILITIES
//! MAX_ARITY
//! MAX_CORRELATION_SIZE
//! ```
//!
//! in this file.
//!
//! `TargetRequirements` describes semantic requirements for any finite
//! computation representable by the host environment. The actual execution
//! limit is supplied independently by the target, runtime, memory system,
//! simulator, distributed environment, or security policy.
//!
//! "Infinity" therefore means:
//!
//! > ZQN does not impose an artificial finite machine-size ceiling.
//!
//! It does not mean that physical machines or hosts have infinite resources.
//!
//! # Capability extensibility
//!
//! Capability identifiers are represented by the existing extensible
//! `crate::quantum::zqn::core::capabilities::CapabilityId` type.
//!
//! The existing capability layer intentionally uses namespaced identifiers
//! rather than a closed Rust enum, allowing future quantum technologies and
//! providers to introduce capabilities without modifying this file.
//!
//! # Approximation rule
//!
//! A target requirement can be:
//!
//! - exact;
//! - approximate within an explicit tolerance;
//! - bounded by an explicit error bound;
//! - statistically satisfied with an explicit confidence;
//! - unsupported.
//!
//! Approximation MUST NEVER be silently treated as exact support.
//!
//! # Requirement versus capability
//!
//! A requirement says:
//!
//! ```text
//! "I need X."
//! ```
//!
//! A capability says:
//!
//! ```text
//! "I can provide X."
//! ```
//!
//! Compatibility is evaluated only when both are available.
//!
//! This module therefore does not contain a target capability implementation.
//!
//! # Determinism
//!
//! Requirements are immutable value objects.
//!
//! This module performs no random generation and does not inspect clocks,
//! environment variables, process IDs, memory addresses, or global state.
//!
//! Requirement equality, hashing and ordering are deterministic.
//!
//! # Security
//!
//! Requirements are data, not authority.
//!
//! A requirement must never grant:
//!
//! - QPU access;
//! - credentials;
//! - filesystem access;
//! - network access;
//! - hardware control;
//! - calibration authority.
//!
//! A caller may submit arbitrary requirements, but the target/runtime must
//! independently authorize execution.
//!
//! # Rust compatibility
//!
//! This file targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes accidental unsafe additions a compile-time
//! error.
//!
//! # Integration contract
//!
//! Producers:
//!
//! - quantum IR analysis;
//! - ZQN noise models;
//! - ZQN channel models;
//! - calibration-aware compilation;
//! - routing analysis;
//! - scheduling analysis;
//! - QEC integration;
//! - simulation planning;
//! - benchmarking planning;
//! - user/API configuration.
//!
//! Consumers:
//!
//! - `target/capabilities.rs`;
//! - `target/compatibility.rs`;
//! - `target/lowering.rs`;
//! - `target/validation.rs`;
//! - hardware integration;
//! - runtime integration;
//! - routing;
//! - scheduling;
//! - QEC;
//! - simulation;
//! - benchmarking.
//!
//! The dependency direction is:
//!
//! ```text
//! semantic computation
//!       │
//!       ▼
//! TargetRequirements
//!       │
//!       ▼
//! TargetCapabilities
//!       │
//!       ▼
//! compatibility
//!       │
//!       ▼
//! lowering / execution
//! ```
//!
//! `requirements.rs` never calls those downstream modules.
//!
//! # Future-file stability
//!
//! This API intentionally does not depend on types from future files such as:
//!
//! ```text
//! target/capabilities.rs
//! target/compatibility.rs
//! target/lowering.rs
//! target/validation.rs
//! ```
//!
//! Those files consume this contract.
//!
//! Therefore this file can be completed and stabilized independently.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::capabilities::CapabilityId;

// =============================================================================
// Requirement identifiers
// =============================================================================

/// Stable identifier for a target requirement set.
///
/// This is an application-level identity, not a hardware identity.
///
/// A value does not imply that a target exists or is available.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TargetRequirementsId(u64);

impl TargetRequirementsId {
    /// Creates an explicit requirement-set identifier.
    ///
    /// The value is opaque to consumers.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for TargetRequirementsId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "zqn-target-requirements:{}", self.0)
    }
}

// =============================================================================
// Requirement strength
// =============================================================================

/// Specifies how strictly a requirement must be satisfied.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum RequirementStrength {
    /// The target must satisfy the semantic requirement exactly.
    Exact,

    /// An approximation is allowed only when the declared tolerance is met.
    Approximate,

    /// The target must provide a formally declared bound.
    Bounded,

    /// The target may satisfy the requirement statistically.
    Statistical,
}

impl Default for RequirementStrength {
    fn default() -> Self {
        Self::Exact
    }
}

// =============================================================================
// Approximation policy
// =============================================================================

/// Defines whether and how semantic approximation may be used.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ApproximationPolicy {
    /// No approximation is permitted.
    Exact,

    /// Approximation is allowed up to an absolute tolerance.
    AbsoluteTolerance(f64),

    /// Approximation is allowed up to a relative tolerance.
    RelativeTolerance(f64),

    /// Approximation is allowed when the target declares an error bound no
    /// greater than this value.
    ErrorBound(f64),

    /// Statistical realization is accepted at the specified confidence.
    ///
    /// `confidence` is expressed in `[0, 1]`.
    StatisticalConfidence {
        confidence: f64,
    },
}

impl Default for ApproximationPolicy {
    fn default() -> Self {
        Self::Exact
    }
}

impl ApproximationPolicy {
    /// Validates the policy.
    pub fn validate(self) -> Result<(), TargetRequirementsError> {
        match self {
            Self::Exact => Ok(()),

            Self::AbsoluteTolerance(value)
            | Self::RelativeTolerance(value)
            | Self::ErrorBound(value) => {
                if value.is_finite() && value >= 0.0 {
                    Ok(())
                } else {
                    Err(TargetRequirementsError::InvalidTolerance(value))
                }
            }

            Self::StatisticalConfidence { confidence } => {
                if confidence.is_finite() && (0.0..=1.0).contains(&confidence) {
                    Ok(())
                } else {
                    Err(TargetRequirementsError::InvalidConfidence(confidence))
                }
            }
        }
    }

    /// Returns whether exact realization is required.
    #[must_use]
    pub const fn requires_exact(self) -> bool {
        matches!(self, Self::Exact)
    }
}

// =============================================================================
// Numerical requirements
// =============================================================================

/// Numerical guarantees required by a computation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NumericalRequirements {
    /// Required numerical strength.
    pub strength: RequirementStrength,

    /// Optional absolute numerical tolerance.
    pub absolute_tolerance: Option<f64>,

    /// Optional relative numerical tolerance.
    pub relative_tolerance: Option<f64>,

    /// Whether non-finite numerical values must be rejected.
    pub reject_non_finite: bool,
}

impl Default for NumericalRequirements {
    fn default() -> Self {
        Self {
            strength: RequirementStrength::Exact,
            absolute_tolerance: None,
            relative_tolerance: None,
            reject_non_finite: true,
        }
    }
}

impl NumericalRequirements {
    /// Creates exact finite numerical requirements.
    #[must_use]
    pub const fn exact() -> Self {
        Self {
            strength: RequirementStrength::Exact,
            absolute_tolerance: None,
            relative_tolerance: None,
            reject_non_finite: true,
        }
    }

    /// Validates all numerical constraints.
    pub fn validate(self) -> Result<(), TargetRequirementsError> {
        validate_optional_non_negative(
            self.absolute_tolerance,
            "absolute numerical tolerance",
        )?;

        validate_optional_non_negative(
            self.relative_tolerance,
            "relative numerical tolerance",
        )?;

        Ok(())
    }
}

// =============================================================================
// Resource requirements
// =============================================================================

/// Resource requirements that may be needed by a target.
///
/// These are requirements, not machine-size limits.
///
/// `None` means that this particular resource quantity is not constrained by
/// this requirement set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResourceRequirements {
    /// Required number of logical quantum resources, when known.
    pub logical_resources: Option<u64>,

    /// Required number of physical quantum resources, when known.
    pub physical_resources: Option<u64>,

    /// Required classical memory in bytes, when known.
    pub classical_memory_bytes: Option<u64>,

    /// Required device/storage memory in bytes, when known.
    pub device_memory_bytes: Option<u64>,

    /// Required execution operations, when known.
    pub operation_count: Option<u64>,

    /// Required execution depth, when known.
    pub depth: Option<u64>,

    /// Required sampling shots, when known.
    pub shots: Option<u64>,
}

impl ResourceRequirements {
    /// Creates unconstrained resource requirements.
    #[must_use]
    pub const fn unconstrained() -> Self {
        Self {
            logical_resources: None,
            physical_resources: None,
            classical_memory_bytes: None,
            device_memory_bytes: None,
            operation_count: None,
            depth: None,
            shots: None,
        }
    }

    /// Validates internal consistency.
    pub fn validate(self) -> Result<(), TargetRequirementsError> {
        if let (Some(logical), Some(physical)) =
            (self.logical_resources, self.physical_resources)
        {
            if physical < logical {
                return Err(TargetRequirementsError::InconsistentResources {
                    logical,
                    physical,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Modality requirements
// =============================================================================

/// Quantum computational modality required by a workload.
///
/// This is intentionally extensible and does not assume that future quantum
/// computing is limited to qubits and gate circuits.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ModalityRequirement {
    name: String,
}

impl ModalityRequirement {
    /// Creates a modality requirement.
    pub fn new<S>(name: S) -> Result<Self, TargetRequirementsError>
    where
        S: Into<String>,
    {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(TargetRequirementsError::EmptyModality);
        }

        if name.chars().any(|character| character.is_control()) {
            return Err(TargetRequirementsError::InvalidModality);
        }

        Ok(Self { name })
    }

    /// Returns the modality identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }
}

// =============================================================================
// Resource-scoped requirements
// =============================================================================

/// Scope to which a target requirement applies.
///
/// Canonical IR qubit identifiers are used directly.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum RequirementScope {
    /// Applies to the complete target.
    Global,

    /// Applies to a logical qubit.
    LogicalQubit(QubitId),

    /// Applies to a physical qubit.
    PhysicalQubit(PhysicalQubitId),
}

impl Default for RequirementScope {
    fn default() -> Self {
        Self::Global
    }
}

// =============================================================================
// Capability requirement
// =============================================================================

/// A single required target capability.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct RequiredCapability {
    capability: CapabilityId,
    scope: RequirementScope,
    strength: RequirementStrength,
    approximation: ApproximationPolicy,
}

impl RequiredCapability {
    /// Creates an exact global capability requirement.
    #[must_use]
    pub fn exact(capability: CapabilityId) -> Self {
        Self {
            capability,
            scope: RequirementScope::Global,
            strength: RequirementStrength::Exact,
            approximation: ApproximationPolicy::Exact,
        }
    }

    /// Creates a scoped capability requirement.
    #[must_use]
    pub fn scoped(
        capability: CapabilityId,
        scope: RequirementScope,
        strength: RequirementStrength,
        approximation: ApproximationPolicy,
    ) -> Self {
        Self {
            capability,
            scope,
            strength,
            approximation,
        }
    }

    /// Returns the capability.
    #[must_use]
    pub fn capability(&self) -> &CapabilityId {
        &self.capability
    }

    /// Returns the requirement scope.
    #[must_use]
    pub const fn scope(&self) -> RequirementScope {
        self.scope
    }

    /// Returns the required support strength.
    #[must_use]
    pub const fn strength(&self) -> RequirementStrength {
        self.strength
    }

    /// Returns the approximation policy.
    #[must_use]
    pub const fn approximation(&self) -> ApproximationPolicy {
        self.approximation
    }

    /// Validates this requirement.
    pub fn validate(&self) -> Result<(), TargetRequirementsError> {
        self.approximation.validate()
    }
}

// =============================================================================
// Representation requirements
// =============================================================================

/// Required representation of quantum noise/channel semantics.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct RepresentationRequirement {
    /// Namespaced representation capability.
    pub capability: CapabilityId,

    /// Required strength.
    pub strength: RequirementStrength,

    /// Approximation policy.
    pub approximation: ApproximationPolicy,
}

impl RepresentationRequirement {
    /// Creates an exact representation requirement.
    #[must_use]
    pub fn exact(capability: CapabilityId) -> Self {
        Self {
            capability,
            strength: RequirementStrength::Exact,
            approximation: ApproximationPolicy::Exact,
        }
    }

    /// Validates this requirement.
    pub fn validate(&self) -> Result<(), TargetRequirementsError> {
        self.approximation.validate()
    }
}

// =============================================================================
// Correlation requirements
// =============================================================================

/// Required correlation semantics.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CorrelationRequirements {
    /// Spatial correlation must be representable.
    pub spatial: bool,

    /// Temporal correlation must be representable.
    pub temporal: bool,

    /// Crosstalk must be representable.
    pub crosstalk: bool,

    /// Non-Markovian memory must be representable.
    pub non_markovian: bool,

    /// Conditional/dynamic correlation must be representable.
    pub conditional: bool,
}

// =============================================================================
// Temporal requirements
// =============================================================================

/// Required temporal behavior of the target/noise realization.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct TemporalRequirements {
    /// Time-dependent noise must be supported.
    pub time_dependent: bool,

    /// Explicit duration-aware noise must be supported.
    pub duration_aware: bool,

    /// Calibration validity over time must be observable.
    pub calibration_time_awareness: bool,

    /// History-dependent behavior must be supported.
    pub history_dependent: bool,
}

// =============================================================================
// Calibration requirements
// =============================================================================

/// Required calibration semantics.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CalibrationRequirements {
    /// Calibration data is required.
    pub required: bool,

    /// Calibration validity must be exposed.
    pub validity_interval: bool,

    /// Calibration uncertainty must be exposed.
    pub uncertainty: bool,

    /// Calibration provenance must be exposed.
    pub provenance: bool,

    /// Calibration drift must be representable.
    pub drift: bool,
}

// =============================================================================
// Characterization requirements
// =============================================================================

/// Required characterization functionality.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CharacterizationRequirements {
    /// Characterization data must be available.
    pub required: bool,

    /// Process characterization is required.
    pub process_characterization: bool,

    /// Randomized benchmarking is required.
    pub randomized_benchmarking: bool,

    /// Uncertainty estimates are required.
    pub uncertainty: bool,

    /// Raw observations are required.
    pub raw_observations: bool,
}

// =============================================================================
// Determinism requirements
// =============================================================================

/// Required stochastic reproducibility guarantees.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct DeterminismRequirements {
    /// Stochastic execution must be reproducible.
    pub reproducible_sampling: bool,

    /// Parallel and serial execution must agree under the deterministic policy.
    pub deterministic_parallelism: bool,

    /// The random seed must be explicitly controllable.
    pub explicit_seed: bool,
}

// =============================================================================
// Provenance requirements
// =============================================================================

/// Required scientific/reproducibility provenance.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ProvenanceRequirements {
    /// Provenance is required.
    pub required: bool,

    /// Model identity must be retained.
    pub model_identity: bool,

    /// Calibration identity must be retained.
    pub calibration_identity: bool,

    /// Target identity must be retained.
    pub target_identity: bool,

    /// Execution identity must be retained.
    pub execution_identity: bool,
}

// =============================================================================
// Execution requirements
// =============================================================================

/// Runtime-level requirements that are relevant to ZQN realization.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ExecutionRequirements {
    /// Cancellation must be supported.
    pub cancellation: bool,

    /// Streaming/lazy realization must be supported where needed.
    pub streaming: bool,

    /// Distributed execution must be supported.
    pub distributed: bool,

    /// Remote execution must preserve requirement semantics.
    pub remote_execution: bool,
}

// =============================================================================
// TargetRequirements
// =============================================================================

/// Complete target-independent requirements for a ZQN-aware computation.
///
/// This is the primary type exported by this file.
///
/// It is deliberately declarative: it describes what is needed but does not
/// select, discover, authenticate, or control a target.
#[derive(Clone, Debug, PartialEq)]
pub struct TargetRequirements {
    /// Optional stable application-level identity.
    id: Option<TargetRequirementsId>,

    /// Required capabilities.
    capabilities: BTreeSet<RequiredCapability>,

    /// Required channel representations.
    representations: BTreeSet<RepresentationRequirement>,

    /// Required computational modalities.
    modalities: BTreeSet<ModalityRequirement>,

    /// Resource requirements.
    resources: ResourceRequirements,

    /// Correlation requirements.
    correlations: CorrelationRequirements,

    /// Temporal requirements.
    temporal: TemporalRequirements,

    /// Calibration requirements.
    calibration: CalibrationRequirements,

    /// Characterization requirements.
    characterization: CharacterizationRequirements,

    /// Determinism requirements.
    determinism: DeterminismRequirements,

    /// Provenance requirements.
    provenance: ProvenanceRequirements,

    /// Runtime/execution requirements.
    execution: ExecutionRequirements,

    /// Numerical requirements.
    numerical: NumericalRequirements,

    /// Global approximation policy.
    approximation: ApproximationPolicy,
}

impl Default for TargetRequirements {
    fn default() -> Self {
        Self::new()
    }
}

impl TargetRequirements {
    /// Creates an empty target requirement set.
    ///
    /// Empty means that this object imposes no ZQN-specific target
    /// requirements. It does not mean that every target is necessarily
    /// executable; canonical IR and other subsystems may impose additional
    /// requirements.
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: None,
            capabilities: BTreeSet::new(),
            representations: BTreeSet::new(),
            modalities: BTreeSet::new(),
            resources: ResourceRequirements::unconstrained(),
            correlations: CorrelationRequirements::default(),
            temporal: TemporalRequirements::default(),
            calibration: CalibrationRequirements::default(),
            characterization: CharacterizationRequirements::default(),
            determinism: DeterminismRequirements::default(),
            provenance: ProvenanceRequirements::default(),
            execution: ExecutionRequirements::default(),
            numerical: NumericalRequirements::exact(),
            approximation: ApproximationPolicy::Exact,
        }
    }

    /// Assigns an application-level requirement identifier.
    #[must_use]
    pub fn with_id(mut self, id: TargetRequirementsId) -> Self {
        self.id = Some(id);
        self
    }

    /// Returns the optional requirement identifier.
    #[must_use]
    pub const fn id(&self) -> Option<TargetRequirementsId> {
        self.id
    }

    /// Adds a required capability.
    pub fn require_capability(
        &mut self,
        requirement: RequiredCapability,
    ) -> Result<(), TargetRequirementsError> {
        requirement.validate()?;
        self.capabilities.insert(requirement);
        Ok(())
    }

    /// Builder-style capability requirement.
    pub fn requiring_capability(
        mut self,
        requirement: RequiredCapability,
    ) -> Result<Self, TargetRequirementsError> {
        self.require_capability(requirement)?;
        Ok(self)
    }

    /// Adds an exact global capability.
    pub fn require_exact_capability(
        &mut self,
        capability: CapabilityId,
    ) -> Result<(), TargetRequirementsError> {
        self.require_capability(RequiredCapability::exact(capability))
    }

    /// Returns required capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &BTreeSet<RequiredCapability> {
        &self.capabilities
    }

    /// Adds a required channel representation.
    pub fn require_representation(
        &mut self,
        requirement: RepresentationRequirement,
    ) -> Result<(), TargetRequirementsError> {
        requirement.validate()?;
        self.representations.insert(requirement);
        Ok(())
    }

    /// Returns representation requirements.
    #[must_use]
    pub fn representations(&self) -> &BTreeSet<RepresentationRequirement> {
        &self.representations
    }

    /// Adds a required modality.
    pub fn require_modality(
        &mut self,
        modality: ModalityRequirement,
    ) -> Result<(), TargetRequirementsError> {
        self.modalities.insert(modality);
        Ok(())
    }

    /// Returns required modalities.
    #[must_use]
    pub fn modalities(&self) -> &BTreeSet<ModalityRequirement> {
        &self.modalities
    }

    /// Sets resource requirements.
    pub fn set_resources(
        &mut self,
        resources: ResourceRequirements,
    ) -> Result<(), TargetRequirementsError> {
        resources.validate()?;
        self.resources = resources;
        Ok(())
    }

    /// Returns resource requirements.
    #[must_use]
    pub const fn resources(&self) -> ResourceRequirements {
        self.resources
    }

    /// Sets correlation requirements.
    pub fn set_correlations(&mut self, correlations: CorrelationRequirements) {
        self.correlations = correlations;
    }

    /// Returns correlation requirements.
    #[must_use]
    pub const fn correlations(&self) -> CorrelationRequirements {
        self.correlations
    }

    /// Sets temporal requirements.
    pub fn set_temporal(&mut self, temporal: TemporalRequirements) {
        self.temporal = temporal;
    }

    /// Returns temporal requirements.
    #[must_use]
    pub const fn temporal(&self) -> TemporalRequirements {
        self.temporal
    }

    /// Sets calibration requirements.
    pub fn set_calibration(&mut self, calibration: CalibrationRequirements) {
        self.calibration = calibration;
    }

    /// Returns calibration requirements.
    #[must_use]
    pub const fn calibration(&self) -> CalibrationRequirements {
        self.calibration
    }

    /// Sets characterization requirements.
    pub fn set_characterization(
        &mut self,
        characterization: CharacterizationRequirements,
    ) {
        self.characterization = characterization;
    }

    /// Returns characterization requirements.
    #[must_use]
    pub const fn characterization(&self) -> CharacterizationRequirements {
        self.characterization
    }

    /// Sets determinism requirements.
    pub fn set_determinism(&mut self, determinism: DeterminismRequirements) {
        self.determinism = determinism;
    }

    /// Returns determinism requirements.
    #[must_use]
    pub const fn determinism(&self) -> DeterminismRequirements {
        self.determinism
    }

    /// Sets provenance requirements.
    pub fn set_provenance(&mut self, provenance: ProvenanceRequirements) {
        self.provenance = provenance;
    }

    /// Returns provenance requirements.
    #[must_use]
    pub const fn provenance(&self) -> ProvenanceRequirements {
        self.provenance
    }

    /// Sets execution requirements.
    pub fn set_execution(&mut self, execution: ExecutionRequirements) {
        self.execution = execution;
    }

    /// Returns execution requirements.
    #[must_use]
    pub const fn execution(&self) -> ExecutionRequirements {
        self.execution
    }

    /// Sets numerical requirements.
    pub fn set_numerical(
        &mut self,
        numerical: NumericalRequirements,
    ) -> Result<(), TargetRequirementsError> {
        numerical.validate()?;
        self.numerical = numerical;
        Ok(())
    }

    /// Returns numerical requirements.
    #[must_use]
    pub const fn numerical(&self) -> NumericalRequirements {
        self.numerical
    }

    /// Sets the global approximation policy.
    pub fn set_approximation(
        &mut self,
        approximation: ApproximationPolicy,
    ) -> Result<(), TargetRequirementsError> {
        approximation.validate()?;
        self.approximation = approximation;
        Ok(())
    }

    /// Returns the global approximation policy.
    #[must_use]
    pub const fn approximation(&self) -> ApproximationPolicy {
        self.approximation
    }

    /// Returns whether no explicit ZQN target requirement is present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
            && self.representations.is_empty()
            && self.modalities.is_empty()
            && self.resources == ResourceRequirements::unconstrained()
            && self.correlations == CorrelationRequirements::default()
            && self.temporal == TemporalRequirements::default()
            && self.calibration == CalibrationRequirements::default()
            && self.characterization == CharacterizationRequirements::default()
            && self.determinism == DeterminismRequirements::default()
            && self.provenance == ProvenanceRequirements::default()
            && self.execution == ExecutionRequirements::default()
            && self.numerical == NumericalRequirements::exact()
            && self.approximation == ApproximationPolicy::Exact
    }

    /// Validates the complete requirement set.
    ///
    /// This validates only target-requirement invariants. It does not discover
    /// or inspect any target.
    pub fn validate(&self) -> Result<(), TargetRequirementsError> {
        self.approximation.validate()?;
        self.numerical.validate()?;
        self.resources.validate()?;

        for capability in &self.capabilities {
            capability.validate()?;
        }

        for representation in &self.representations {
            representation.validate()?;
        }

        for modality in &self.modalities {
            if modality.name.trim().is_empty() {
                return Err(TargetRequirementsError::EmptyModality);
            }
        }

        Ok(())
    }

    /// Returns a deterministic iterator over required capabilities.
    pub fn iter_capabilities(
        &self,
    ) -> impl Iterator<Item = &RequiredCapability> {
        self.capabilities.iter()
    }

    /// Returns a deterministic iterator over required representations.
    pub fn iter_representations(
        &self,
    ) -> impl Iterator<Item = &RepresentationRequirement> {
        self.representations.iter()
    }

    /// Returns a deterministic iterator over required modalities.
    pub fn iter_modalities(
        &self,
    ) -> impl Iterator<Item = &ModalityRequirement> {
        self.modalities.iter()
    }

    /// Merges another requirement set into this one.
    ///
    /// The operation is monotonic:
    ///
    /// ```text
    /// existing requirements
    ///          +
    /// additional requirements
    ///          =
    /// stricter/equivalent requirements
    /// ```
    ///
    /// Existing requirements are never removed.
    pub fn merge(
        &mut self,
        other: &Self,
    ) -> Result<(), TargetRequirementsError> {
        other.validate()?;

        if self.id.is_none() {
            self.id = other.id;
        }

        for capability in &other.capabilities {
            self.capabilities.insert(capability.clone());
        }

        for representation in &other.representations {
            self.representations.insert(representation.clone());
        }

        for modality in &other.modalities {
            self.modalities.insert(modality.clone());
        }

        self.resources = merge_resource_requirements(
            self.resources,
            other.resources,
        )?;

        self.correlations.spatial |= other.correlations.spatial;
        self.correlations.temporal |= other.correlations.temporal;
        self.correlations.crosstalk |= other.correlations.crosstalk;
        self.correlations.non_markovian |= other.correlations.non_markovian;
        self.correlations.conditional |= other.correlations.conditional;

        self.temporal.time_dependent |= other.temporal.time_dependent;
        self.temporal.duration_aware |= other.temporal.duration_aware;
        self.temporal.calibration_time_awareness |=
            other.temporal.calibration_time_awareness;
        self.temporal.history_dependent |= other.temporal.history_dependent;

        self.calibration.required |= other.calibration.required;
        self.calibration.validity_interval |=
            other.calibration.validity_interval;
        self.calibration.uncertainty |= other.calibration.uncertainty;
        self.calibration.provenance |= other.calibration.provenance;
        self.calibration.drift |= other.calibration.drift;

        self.characterization.required |= other.characterization.required;
        self.characterization.process_characterization |=
            other.characterization.process_characterization;
        self.characterization.randomized_benchmarking |=
            other.characterization.randomized_benchmarking;
        self.characterization.uncertainty |= other.characterization.uncertainty;
        self.characterization.raw_observations |=
            other.characterization.raw_observations;

        self.determinism.reproducible_sampling |=
            other.determinism.reproducible_sampling;
        self.determinism.deterministic_parallelism |=
            other.determinism.deterministic_parallelism;
        self.determinism.explicit_seed |= other.determinism.explicit_seed;

        self.provenance.required |= other.provenance.required;
        self.provenance.model_identity |= other.provenance.model_identity;
        self.provenance.calibration_identity |=
            other.provenance.calibration_identity;
        self.provenance.target_identity |= other.provenance.target_identity;
        self.provenance.execution_identity |=
            other.provenance.execution_identity;

        self.execution.cancellation |= other.execution.cancellation;
        self.execution.streaming |= other.execution.streaming;
        self.execution.distributed |= other.execution.distributed;
        self.execution.remote_execution |= other.execution.remote_execution;

        self.numerical = merge_numerical_requirements(
            self.numerical,
            other.numerical,
        )?;

        self.approximation =
            merge_approximation_policy(self.approximation, other.approximation)?;

        Ok(())
    }

    /// Returns a merged copy without mutating either input.
    pub fn merged(
        &self,
        other: &Self,
    ) -> Result<Self, TargetRequirementsError> {
        let mut result = self.clone();
        result.merge(other)?;
        Ok(result)
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating target requirements.
#[derive(Clone, Debug, PartialEq)]
pub enum TargetRequirementsError {
    /// A numerical value is not finite.
    NonFiniteValue {
        /// Semantic field name.
        field: &'static str,

        /// Invalid value.
        value: f64,
    },

    /// A tolerance is negative or non-finite.
    InvalidTolerance(f64),

    /// A statistical confidence value is outside `[0, 1]`.
    InvalidConfidence(f64),

    /// A modality name is empty.
    EmptyModality,

    /// A modality contains an invalid control character.
    InvalidModality,

    /// Logical and physical resource requirements contradict each other.
    InconsistentResources {
        /// Required logical resources.
        logical: u64,

        /// Required physical resources.
        physical: u64,
    },

    /// Two exact approximation policies cannot be merged without losing a
    /// stronger guarantee.
    IncompatibleApproximationPolicies,

    /// Numerical requirements cannot be merged safely.
    IncompatibleNumericalRequirements,
}

impl fmt::Display for TargetRequirementsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteValue { field, value } => {
                write!(formatter, "{field} must be finite, got {value}")
            }

            Self::InvalidTolerance(value) => {
                write!(formatter, "invalid tolerance: {value}")
            }

            Self::InvalidConfidence(value) => {
                write!(formatter, "invalid confidence: {value}")
            }

            Self::EmptyModality => {
                write!(formatter, "modality requirement cannot be empty")
            }

            Self::InvalidModality => {
                write!(formatter, "modality requirement contains a control character")
            }

            Self::InconsistentResources { logical, physical } => write!(
                formatter,
                "physical resource requirement ({physical}) cannot be \
                 smaller than logical resource requirement ({logical})"
            ),

            Self::IncompatibleApproximationPolicies => {
                write!(formatter, "approximation policies cannot be safely merged")
            }

            Self::IncompatibleNumericalRequirements => {
                write!(formatter, "numerical requirements cannot be safely merged")
            }
        }
    }
}

impl std::error::Error for TargetRequirementsError {}

// =============================================================================
// Internal validation helpers
// =============================================================================

fn validate_optional_non_negative(
    value: Option<f64>,
    field: &'static str,
) -> Result<(), TargetRequirementsError> {
    if let Some(value) = value {
        if !value.is_finite() {
            return Err(TargetRequirementsError::NonFiniteValue { field, value });
        }

        if value < 0.0 {
            return Err(TargetRequirementsError::InvalidTolerance(value));
        }
    }

    Ok(())
}

fn merge_resource_requirements(
    left: ResourceRequirements,
    right: ResourceRequirements,
) -> Result<ResourceRequirements, TargetRequirementsError> {
    let result = ResourceRequirements {
        logical_resources: max_optional(
            left.logical_resources,
            right.logical_resources,
        ),
        physical_resources: max_optional(
            left.physical_resources,
            right.physical_resources,
        ),
        classical_memory_bytes: max_optional(
            left.classical_memory_bytes,
            right.classical_memory_bytes,
        ),
        device_memory_bytes: max_optional(
            left.device_memory_bytes,
            right.device_memory_bytes,
        ),
        operation_count: max_optional(
            left.operation_count,
            right.operation_count,
        ),
        depth: max_optional(left.depth, right.depth),
        shots: max_optional(left.shots, right.shots),
    };

    result.validate()?;
    Ok(result)
}

fn max_optional<T: Ord + Copy>(left: Option<T>, right: Option<T>) -> Option<T> {
    match (left, right) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn merge_numerical_requirements(
    left: NumericalRequirements,
    right: NumericalRequirements,
) -> Result<NumericalRequirements, TargetRequirementsError> {
    let absolute_tolerance =
        stricter_optional_tolerance(
            left.absolute_tolerance,
            right.absolute_tolerance,
        );

    let relative_tolerance =
        stricter_optional_tolerance(
            left.relative_tolerance,
            right.relative_tolerance,
        );

    let strength = if left.strength >= right.strength {
        left.strength
    } else {
        right.strength
    };

    let result = NumericalRequirements {
        strength,
        absolute_tolerance,
        relative_tolerance,
        reject_non_finite: left.reject_non_finite || right.reject_non_finite,
    };

    result.validate()?;
    Ok(result)
}

fn stricter_optional_tolerance(
    left: Option<f64>,
    right: Option<f64>,
) -> Option<f64> {
    match (left, right) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn merge_approximation_policy(
    left: ApproximationPolicy,
    right: ApproximationPolicy,
) -> Result<ApproximationPolicy, TargetRequirementsError> {
    match (left, right) {
        (ApproximationPolicy::Exact, other)
        | (other, ApproximationPolicy::Exact) => Ok(other),

        (
            ApproximationPolicy::AbsoluteTolerance(a),
            ApproximationPolicy::AbsoluteTolerance(b),
        ) => Ok(ApproximationPolicy::AbsoluteTolerance(a.min(b))),

        (
            ApproximationPolicy::RelativeTolerance(a),
            ApproximationPolicy::RelativeTolerance(b),
        ) => Ok(ApproximationPolicy::RelativeTolerance(a.min(b))),

        (
            ApproximationPolicy::ErrorBound(a),
            ApproximationPolicy::ErrorBound(b),
        ) => Ok(ApproximationPolicy::ErrorBound(a.min(b))),

        (
            ApproximationPolicy::StatisticalConfidence { confidence: a },
            ApproximationPolicy::StatisticalConfidence { confidence: b },
        ) => Ok(ApproximationPolicy::StatisticalConfidence {
            confidence: a.max(b),
        }),

        _ => Err(
            TargetRequirementsError::IncompatibleApproximationPolicies
        ),
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn capability(name: &str) -> CapabilityId {
        CapabilityId::new(name).expect("test capability must be valid")
    }

    #[test]
    fn empty_requirements_are_valid() {
        let requirements = TargetRequirements::new();

        assert!(requirements.is_empty());
        assert!(requirements.validate().is_ok());
    }

    #[test]
    fn exact_capability_requirement_is_recorded() {
        let mut requirements = TargetRequirements::new();

        requirements
            .require_exact_capability(capability("zqn.channel.kraus"))
            .expect("capability should be accepted");

        assert_eq!(requirements.capabilities().len(), 1);
    }

    #[test]
    fn canonical_qubit_scope_is_supported() {
        let logical = QubitId::new(7);
        let physical = PhysicalQubitId::new(11);

        let logical_requirement = RequiredCapability::scoped(
            capability("zqn.noise.readout"),
            RequirementScope::LogicalQubit(logical),
            RequirementStrength::Exact,
            ApproximationPolicy::Exact,
        );

        let physical_requirement = RequiredCapability::scoped(
            capability("zqn.noise.readout"),
            RequirementScope::PhysicalQubit(physical),
            RequirementStrength::Exact,
            ApproximationPolicy::Exact,
        );

        assert_eq!(
            logical_requirement.scope(),
            RequirementScope::LogicalQubit(logical)
        );

        assert_eq!(
            physical_requirement.scope(),
            RequirementScope::PhysicalQubit(physical)
        );
    }

    #[test]
    fn invalid_tolerance_is_rejected() {
        let policy = ApproximationPolicy::AbsoluteTolerance(f64::NAN);

        assert!(policy.validate().is_err());
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        let policy =
            ApproximationPolicy::StatisticalConfidence { confidence: 1.1 };

        assert!(policy.validate().is_err());
    }

    #[test]
    fn resource_requirements_are_not_architectural_limits() {
        let resources = ResourceRequirements {
            logical_resources: Some(u64::MAX),
            physical_resources: Some(u64::MAX),
            ..ResourceRequirements::unconstrained()
        };

        assert!(resources.validate().is_ok());
    }

    #[test]
    fn inconsistent_resources_are_rejected() {
        let resources = ResourceRequirements {
            logical_resources: Some(10),
            physical_resources: Some(9),
            ..ResourceRequirements::unconstrained()
        };

        assert!(resources.validate().is_err());
    }

    #[test]
    fn merge_is_monotonic_for_resource_requirements() {
        let left = ResourceRequirements {
            logical_resources: Some(10),
            ..ResourceRequirements::unconstrained()
        };

        let right = ResourceRequirements {
            logical_resources: Some(20),
            ..ResourceRequirements::unconstrained()
        };

        let merged =
            merge_resource_requirements(left, right)
                .expect("requirements should merge");

        assert_eq!(merged.logical_resources, Some(20));
    }

    #[test]
    fn stricter_tolerance_wins() {
        let merged = merge_approximation_policy(
            ApproximationPolicy::AbsoluteTolerance(1e-3),
            ApproximationPolicy::AbsoluteTolerance(1e-6),
        )
        .expect("compatible policies should merge");

        assert_eq!(
            merged,
            ApproximationPolicy::AbsoluteTolerance(1e-6)
        );
    }

    #[test]
    fn higher_statistical_confidence_wins() {
        let merged = merge_approximation_policy(
            ApproximationPolicy::StatisticalConfidence {
                confidence: 0.90,
            },
            ApproximationPolicy::StatisticalConfidence {
                confidence: 0.99,
            },
        )
        .expect("compatible policies should merge");

        assert_eq!(
            merged,
            ApproximationPolicy::StatisticalConfidence {
                confidence: 0.99,
            }
        );
    }

    #[test]
    fn exact_policy_dominates_approximation() {
        let merged = merge_approximation_policy(
            ApproximationPolicy::Exact,
            ApproximationPolicy::AbsoluteTolerance(1e-3),
        )
        .expect("exact policy should be mergeable");

        assert_eq!(
            merged,
            ApproximationPolicy::AbsoluteTolerance(1e-3)
        );
    }

    #[test]
    fn incompatible_approximation_modes_are_rejected() {
        let result = merge_approximation_policy(
            ApproximationPolicy::AbsoluteTolerance(1e-3),
            ApproximationPolicy::RelativeTolerance(1e-3),
        );

        assert!(result.is_err());
    }

    #[test]
    fn merge_preserves_all_required_domains() {
        let mut left = TargetRequirements::new();

        left.require_exact_capability(capability("zqn.channel.kraus"))
            .expect("valid capability");

        left.set_correlations(CorrelationRequirements {
            spatial: true,
            ..CorrelationRequirements::default()
        });

        let mut right = TargetRequirements::new();

        right
            .require_exact_capability(capability("zqn.noise.readout"))
            .expect("valid capability");

        right.set_correlations(CorrelationRequirements {
            temporal: true,
            crosstalk: true,
            ..CorrelationRequirements::default()
        });

        left.merge(&right).expect("requirements should merge");

        assert_eq!(left.capabilities().len(), 2);
        assert!(left.correlations().spatial);
        assert!(left.correlations().temporal);
        assert!(left.correlations().crosstalk);
    }

    #[test]
    fn deterministic_iteration_uses_ordered_sets() {
        let mut requirements = TargetRequirements::new();

        requirements
            .require_exact_capability(capability("zqn.noise.readout"))
            .expect("valid capability");

        requirements
            .require_exact_capability(capability("zqn.channel.kraus"))
            .expect("valid capability");

        let names: Vec<&str> = requirements
            .iter_capabilities()
            .map(|requirement| requirement.capability().as_str())
            .collect();

        assert_eq!(
            names,
            vec![
                "zqn.channel.kraus",
                "zqn.noise.readout"
            ]
        );
    }

    #[test]
    fn u64_resource_domain_has_no_artificial_small_limit() {
        let resources = ResourceRequirements {
            logical_resources: Some(u64::MAX),
            physical_resources: Some(u64::MAX),
            classical_memory_bytes: Some(u64::MAX),
            device_memory_bytes: Some(u64::MAX),
            operation_count: Some(u64::MAX),
            depth: Some(u64::MAX),
            shots: Some(u64::MAX),
        };

        assert!(resources.validate().is_ok());
    }

    #[test]
    fn modality_is_extensible() {
        let modality =
            ModalityRequirement::new("future.quantum.modality")
                .expect("future modalities must be representable");

        assert_eq!(
            modality.as_str(),
            "future.quantum.modality"
        );
    }

    #[test]
    fn validation_does_not_discover_targets() {
        let requirements = TargetRequirements::new();

        // Validation is purely structural and therefore succeeds without
        // any target, hardware, network, or credential context.
        assert!(requirements.validate().is_ok());
    }
}