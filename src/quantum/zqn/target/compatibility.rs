//! Zamani Quantum Noise (ZQN) — Target Compatibility
//!
//! This module is the deterministic compatibility boundary between:
//!
//!     `zqn::target::requirements::TargetRequirements`
//!
//! and:
//!
//!     `zqn::target::capabilities::TargetCapabilities`.
//!
//! # Architectural position
//!
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
//!      │        TargetCapabilities
//!      │                 ▲
//!      │                 │
//!      └───────┬─────────┘
//!              ▼
//!       THIS MODULE
//!       compatibility
//!              │
//!       ┌──────┼─────────────┐
//!       ▼      ▼             ▼
//!   exact   approximate   incompatible
//!       │      │             │
//!       └──────┼─────────────┘
//!              ▼
//!          validation
//!              │
//!              ▼
//!           lowering
//!              │
//!              ▼
//!       runtime / hardware
//! ```
//!
//! # Ownership
//!
//! This file owns:
//!
//! - compatibility policy;
//! - compatibility decisions;
//! - deterministic compatibility reports;
//! - exact versus approximate acceptance;
//! - capability requirement evaluation;
//! - representation requirement evaluation;
//! - target-policy enforcement;
//! - compatibility diagnostics;
//! - aggregation of compatibility results;
//! - explicit handling of requirements that require information outside the
//!   capability set;
//! - compatibility validation before lowering/execution.
//!
//! # This file does NOT own
//!
//! This module does not own:
//!
//! - quantum IR;
//! - qubit identity;
//! - target discovery;
//! - hardware discovery;
//! - hardware topology;
//! - credentials;
//! - provider APIs;
//! - network access;
//! - resource allocation;
//! - routing;
//! - scheduling;
//! - noise models;
//! - channels;
//! - faults;
//! - calibration values;
//! - simulation;
//! - QEC;
//! - benchmarking;
//! - lowering;
//! - execution.
//!
//! Those responsibilities remain in their respective modules.
//!
//! # Canonical quantum identity
//!
//! This file does not create another `QubitId` or `PhysicalQubitId`.
//!
//! Resource identities are already owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! through:
//!
//!     QubitId
//!     PhysicalQubitId
//!
//! Capability scopes are consequently compared exactly as supplied by the
//! canonical ZQN capability layer.
//!
//! # Write once / scale everywhere
//!
//! This file imposes no semantic limit on:
//!
//! - number of logical qubits;
//! - number of physical qubits;
//! - number of resources;
//! - operation count;
//! - capability count;
//! - capability arity;
//! - correlation size;
//! - machine size;
//! - target size.
//!
//! There are deliberately no:
//!
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_CAPABILITIES
//!     MAX_REQUIREMENTS
//!
//! constants.
//!
//! Compatibility complexity is proportional to the requirements being
//! evaluated and the target capability representation. It does not depend on
//! a hard-coded machine-size ceiling.
//!
//! # Critical semantic rule
//!
//! Compatibility is NOT the same thing as target selection.
//!
//! This module answers:
//!
//!     "Can these requirements be satisfied by this target description?"
//!
//! It does not answer:
//!
//!     "Which target should be selected?"
//!
//! Target selection belongs to a higher-level orchestration layer.
//!
//! # Exactness rule
//!
//! Approximation is never silently accepted.
//!
//! In particular:
//!
//!     exact requirement
//!         + approximate capability
//!         = incompatible
//!
//!     native requirement
//!         + emulated capability
//!         = incompatible
//!
//!     exact requirement
//!         + exact emulated capability
//!         = compatible
//!
//!     approximate requirement
//!         + approximate capability
//!         = compatible
//!
//! The approximation decision is represented explicitly in the resulting
//! report.
//!
//! # Scope rule
//!
//! Scope matching is exact.
//!
//! A capability for:
//!
//!     PhysicalQubit(5)
//!
//! does not automatically satisfy:
//!
//!     PhysicalQubit(6)
//!
//! and a global capability does not silently satisfy a resource-specific
//! requirement.
//!
//! Any future scope inheritance or capability propagation policy must be made
//! explicit in a separate policy layer.
//!
//! # Determinism
//!
//! Compatibility evaluation is deterministic.
//!
//! It does not:
//!
//! - access clocks;
//! - access environment variables;
//! - access global mutable state;
//! - use randomness;
//! - call hardware;
//! - call networks;
//! - inspect process state.
//!
//! Requirement ordering supplied by `TargetRequirements` is preserved.
//!
//! # Resource safety
//!
//! This module never materializes quantum states, tensors, circuits, topology,
//! channels, or fault sets.
//!
//! The compatibility engine only evaluates declarative metadata.
//!
//! It therefore remains lightweight for very large quantum machines.
//!
//! Actual target capacity is evaluated through explicit target resource facts,
//! not inferred from a fixed machine-size constant.
//!
//! # Security
//!
//! Compatibility is a validation boundary, not an authorization boundary.
//!
//! A compatible target must still be independently authorized before execution.
//!
//! Compatibility must never grant:
//!
//! - QPU access;
//! - credentials;
//! - filesystem access;
//! - network access;
//! - hardware control.
//!
//! # Numerical safety
//!
//! This file does not perform floating-point approximation calculations itself.
//!
//! Approximation values are therefore treated as declared semantic data and
//! must be validated by the requirements/capability layers that create them.
//!
//! No NaN/∞ value is converted into a valid value.
//!
//! # Serialization
//!
//! No wire format is defined here.
//!
//! Serialization belongs to:
//!
//!     zqn::io
//!
//! Compatibility reports are ordinary value objects and can therefore be
//! serialized by the future canonical schema layer.
//!
//! # Integration contract
//!
//! ## Producer
//!
//!     target::requirements
//!     target::capabilities
//!
//! ## Consumer
//!
//!     target::validation
//!     target::lowering
//!     integration::hardware
//!     integration::runtime
//!     integration::routing
//!     integration::scheduling
//!     integration::qec
//!     integration::benchmarking
//!
//! ## Required dependency direction
//!
//! ```text
//! TargetRequirements ───────┐
//!                           │
//! TargetCapabilities ───────┼──► compatibility.rs
//!                           │
//!                           ▼
//!                     CompatibilityReport
//!                           │
//!                           ▼
//!                    validation/lowering
//! ```
//!
//! `compatibility.rs` must never import routing, scheduling, hardware,
//! runtime, QEC, or lowering implementations.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]

use std::fmt;

use super::capabilities::{
    TargetCapabilities,
    TargetCapabilityMatch,
    TargetCapabilityPolicy,
};
use super::requirements::{
    ApproximationPolicy,
    CalibrationRequirements,
    CharacterizationRequirements,
    CorrelationRequirements,
    DeterminismRequirements,
    ExecutionRequirements,
    ModalityRequirement,
    NumericalRequirements,
    RepresentationRequirement,
    RequiredCapability,
    ResourceRequirements,
    TargetRequirements,
    TemporalRequirements,
};

// =============================================================================
// Compatibility strictness
// =============================================================================

/// Controls how compatibility is evaluated.
///
/// The policy describes what the caller is willing to accept. It does not
/// modify the target capability declarations.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CompatibilityPolicy {
    /// Only native support is accepted.
    NativeOnly,

    /// Exact native or exact emulated support is accepted.
    #[default]
    ExactOnly,

    /// Explicit approximate support is accepted when the requirement allows
    /// approximation.
    AllowApproximate,

    /// Accept every support realization allowed by the requirement.
    AnyAllowed,
}

impl CompatibilityPolicy {
    /// Returns whether approximate support can be accepted.
    #[must_use]
    pub const fn allows_approximate(self) -> bool {
        matches!(
            self,
            Self::AllowApproximate | Self::AnyAllowed
        )
    }

    /// Returns whether exact emulation can be accepted.
    #[must_use]
    pub const fn allows_emulation(self) -> bool {
        !matches!(self, Self::NativeOnly)
    }

    /// Converts this compatibility policy into the target-capability policy.
    #[must_use]
    pub const fn target_policy(self) -> TargetCapabilityPolicy {
        match self {
            Self::NativeOnly => TargetCapabilityPolicy::NativeOnly,
            Self::ExactOnly => TargetCapabilityPolicy::ExactOnly,
            Self::AllowApproximate => TargetCapabilityPolicy::AllowApproximate,
            Self::AnyAllowed => TargetCapabilityPolicy::AnyAllowed,
        }
    }
}

impl fmt::Display for CompatibilityPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NativeOnly => formatter.write_str("native-only"),
            Self::ExactOnly => formatter.write_str("exact-only"),
            Self::AllowApproximate => {
                formatter.write_str("allow-approximate")
            }
            Self::AnyAllowed => formatter.write_str("any-allowed"),
        }
    }
}

// =============================================================================
// Evaluation status
// =============================================================================

/// Overall status of compatibility evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CompatibilityStatus {
    /// All evaluated requirements are exactly satisfied.
    Compatible,

    /// All evaluated requirements are satisfied, but at least one uses an
    /// explicitly accepted approximation.
    CompatibleWithApproximation,

    /// At least one requirement is not satisfied.
    Incompatible,

    /// One or more required facts were not available for evaluation.
    ///
    /// `Undetermined` is deliberately not treated as compatible.
    Undetermined,
}

impl CompatibilityStatus {
    /// Returns whether the status permits execution without further
    /// compatibility evaluation.
    #[must_use]
    pub const fn is_compatible(self) -> bool {
        matches!(
            self,
            Self::Compatible | Self::CompatibleWithApproximation
        )
    }

    /// Returns whether every requirement is satisfied exactly.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Compatible)
    }

    /// Returns whether explicit approximation is involved.
    #[must_use]
    pub const fn uses_approximation(self) -> bool {
        matches!(self, Self::CompatibleWithApproximation)
    }

    /// Returns whether execution must be rejected.
    #[must_use]
    pub const fn is_rejected(self) -> bool {
        matches!(
            self,
            Self::Incompatible | Self::Undetermined
        )
    }
}

impl fmt::Display for CompatibilityStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compatible => formatter.write_str("compatible"),
            Self::CompatibleWithApproximation => {
                formatter.write_str("compatible-with-approximation")
            }
            Self::Incompatible => formatter.write_str("incompatible"),
            Self::Undetermined => formatter.write_str("undetermined"),
        }
    }
}

// =============================================================================
// Requirement category
// =============================================================================

/// Category of a compatibility evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum RequirementCategory {
    /// Explicit capability declaration.
    Capability,

    /// Channel/noise representation.
    Representation,

    /// Quantum computational modality.
    Modality,

    /// Resource/capacity requirement.
    Resource,

    /// Correlation semantics.
    Correlation,

    /// Temporal semantics.
    Temporal,

    /// Calibration semantics.
    Calibration,

    /// Characterization semantics.
    Characterization,

    /// Deterministic execution semantics.
    Determinism,

    /// Provenance semantics.
    Provenance,

    /// Runtime/execution semantics.
    Execution,

    /// Numerical semantics.
    Numerical,

    /// Global approximation policy.
    Approximation,
}

impl fmt::Display for RequirementCategory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Capability => formatter.write_str("capability"),
            Self::Representation => formatter.write_str("representation"),
            Self::Modality => formatter.write_str("modality"),
            Self::Resource => formatter.write_str("resource"),
            Self::Correlation => formatter.write_str("correlation"),
            Self::Temporal => formatter.write_str("temporal"),
            Self::Calibration => formatter.write_str("calibration"),
            Self::Characterization => formatter.write_str("characterization"),
            Self::Determinism => formatter.write_str("determinism"),
            Self::Provenance => formatter.write_str("provenance"),
            Self::Execution => formatter.write_str("execution"),
            Self::Numerical => formatter.write_str("numerical"),
            Self::Approximation => formatter.write_str("approximation"),
        }
    }
}

// =============================================================================
// Generic evaluation result
// =============================================================================

/// Result of evaluating one compatibility condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompatibilityFinding {
    /// Requirement was satisfied exactly.
    Satisfied {
        /// Category of the requirement.
        category: RequirementCategory,

        /// Human-readable stable description.
        description: String,
    },

    /// Requirement was satisfied approximately.
    Approximate {
        /// Category of the requirement.
        category: RequirementCategory,

        /// Human-readable stable description.
        description: String,
    },

    /// Requirement cannot be satisfied.
    Incompatible {
        /// Category of the requirement.
        category: RequirementCategory,

        /// Human-readable stable description.
        description: String,
    },

    /// Required information was not supplied to the compatibility engine.
    Undetermined {
        /// Category of the requirement.
        category: RequirementCategory,

        /// Human-readable stable description.
        description: String,
    },
}

impl CompatibilityFinding {
    /// Returns the finding category.
    #[must_use]
    pub const fn category(&self) -> RequirementCategory {
        match self {
            Self::Satisfied { category, .. }
            | Self::Approximate { category, .. }
            | Self::Incompatible { category, .. }
            | Self::Undetermined { category, .. } => *category,
        }
    }

    /// Returns whether the finding is compatible.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        matches!(
            self,
            Self::Satisfied { .. } | Self::Approximate { .. }
        )
    }

    /// Returns whether the finding is exact.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        matches!(self, Self::Satisfied { .. })
    }

    /// Returns whether the finding uses approximation.
    #[must_use]
    pub const fn is_approximate(&self) -> bool {
        matches!(self, Self::Approximate { .. })
    }

    /// Returns whether evaluation was undetermined.
    #[must_use]
    pub const fn is_undetermined(&self) -> bool {
        matches!(self, Self::Undetermined { .. })
    }

    /// Returns the diagnostic description.
    #[must_use]
    pub fn description(&self) -> &str {
        match self {
            Self::Satisfied { description, .. }
            | Self::Approximate { description, .. }
            | Self::Incompatible { description, .. }
            | Self::Undetermined { description, .. } => description,
        }
    }
}

// =============================================================================
// Capability result
// =============================================================================

/// Compatibility result for one capability requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityFinding {
    /// The original target requirement.
    pub requirement: RequiredCapability,

    /// Result returned by the target capability layer.
    pub result: TargetCapabilityMatch,
}

impl CapabilityFinding {
    /// Returns whether this capability is compatible.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.result.is_compatible()
    }

    /// Returns whether this capability is exact.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        self.result.is_satisfied()
    }

    /// Returns whether this capability is approximate.
    #[must_use]
    pub const fn is_approximate(&self) -> bool {
        self.result.is_approximate()
    }
}

// =============================================================================
// Additional target facts
// =============================================================================

/// Target facts that cannot safely be inferred from a capability declaration.
///
/// This structure intentionally lives at the compatibility boundary rather
/// than inside `TargetCapabilities`.
///
/// `TargetCapabilities` answers semantic capability questions.
///
/// `TargetFacts` supplies concrete target-state facts needed for requirements
/// such as capacity and modality.
///
/// A hardware adapter, simulator, emulator, or distributed runtime may create
/// this value.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TargetFacts {
    /// Supported computational modalities.
    modalities: Vec<String>,

    /// Available logical quantum resources.
    logical_resources: Option<u64>,

    /// Available physical quantum resources.
    physical_resources: Option<u64>,

    /// Available classical memory in bytes.
    classical_memory_bytes: Option<u64>,

    /// Available device memory in bytes.
    device_memory_bytes: Option<u64>,

    /// Maximum supported operation count, when the target exposes one.
    operation_count: Option<u64>,

    /// Maximum supported depth, when the target exposes one.
    depth: Option<u64>,

    /// Maximum available sampling shots, when the target exposes one.
    shots: Option<u64>,
}

impl TargetFacts {
    /// Creates an empty target-facts object.
    ///
    /// Unknown facts remain unknown. Unknown is never interpreted as
    /// unlimited.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            modalities: Vec::new(),
            logical_resources: None,
            physical_resources: None,
            classical_memory_bytes: None,
            device_memory_bytes: None,
            operation_count: None,
            depth: None,
            shots: None,
        }
    }

    /// Sets the supported modality collection.
    pub fn with_modalities<I, S>(mut self, modalities: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.modalities = modalities.into_iter().map(Into::into).collect();
        self
    }

    /// Sets available logical resources.
    #[must_use]
    pub const fn with_logical_resources(mut self, value: u64) -> Self {
        self.logical_resources = Some(value);
        self
    }

    /// Sets available physical resources.
    #[must_use]
    pub const fn with_physical_resources(mut self, value: u64) -> Self {
        self.physical_resources = Some(value);
        self
    }

    /// Sets available classical memory.
    #[must_use]
    pub const fn with_classical_memory_bytes(mut self, value: u64) -> Self {
        self.classical_memory_bytes = Some(value);
        self
    }

    /// Sets available device memory.
    #[must_use]
    pub const fn with_device_memory_bytes(mut self, value: u64) -> Self {
        self.device_memory_bytes = Some(value);
        self
    }

    /// Sets available operation capacity.
    #[must_use]
    pub const fn with_operation_count(mut self, value: u64) -> Self {
        self.operation_count = Some(value);
        self
    }

    /// Sets available depth.
    #[must_use]
    pub const fn with_depth(mut self, value: u64) -> Self {
        self.depth = Some(value);
        self
    }

    /// Sets available shots.
    #[must_use]
    pub const fn with_shots(mut self, value: u64) -> Self {
        self.shots = Some(value);
        self
    }

    /// Returns supported modalities.
    #[must_use]
    pub fn modalities(&self) -> &[String] {
        &self.modalities
    }

    /// Returns logical resource capacity.
    #[must_use]
    pub const fn logical_resources(&self) -> Option<u64> {
        self.logical_resources
    }

    /// Returns physical resource capacity.
    #[must_use]
    pub const fn physical_resources(&self) -> Option<u64> {
        self.physical_resources
    }

    /// Returns classical memory capacity.
    #[must_use]
    pub const fn classical_memory_bytes(&self) -> Option<u64> {
        self.classical_memory_bytes
    }

    /// Returns device memory capacity.
    #[must_use]
    pub const fn device_memory_bytes(&self) -> Option<u64> {
        self.device_memory_bytes
    }

    /// Returns operation capacity.
    #[must_use]
    pub const fn operation_count(&self) -> Option<u64> {
        self.operation_count
    }

    /// Returns depth capacity.
    #[must_use]
    pub const fn depth(&self) -> Option<u64> {
        self.depth
    }

    /// Returns shot capacity.
    #[must_use]
    pub const fn shots(&self) -> Option<u64> {
        self.shots
    }

    fn supports_modality(&self, requirement: &ModalityRequirement) -> bool {
        self.modalities
            .iter()
            .any(|modality| modality == requirement.as_str())
    }
}

// =============================================================================
// Compatibility context
// =============================================================================

/// Complete immutable input to compatibility evaluation.
///
/// This is deliberately a value object. It does not contain provider handles,
/// credentials, clocks, sockets, or mutable execution state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompatibilityContext {
    /// Additional target facts.
    facts: TargetFacts,

    /// Whether calibration requirements are known to be supported.
    calibration: CapabilityAvailability,

    /// Whether characterization requirements are known to be supported.
    characterization: CapabilityAvailability,

    /// Whether correlation requirements are known to be supported.
    correlations: CapabilityAvailability,

    /// Whether temporal requirements are known to be supported.
    temporal: CapabilityAvailability,

    /// Whether determinism requirements are known to be supported.
    determinism: CapabilityAvailability,

    /// Whether provenance requirements are known to be supported.
    provenance: CapabilityAvailability,

    /// Whether execution requirements are known to be supported.
    execution: CapabilityAvailability,

    /// Whether numerical requirements are known to be supported.
    numerical: CapabilityAvailability,
}

impl CompatibilityContext {
    /// Creates an empty context.
    ///
    /// Unknown information remains unknown and therefore cannot establish
    /// compatibility for requirements that depend on it.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            facts: TargetFacts::new(),
            calibration: CapabilityAvailability::Unknown,
            characterization: CapabilityAvailability::Unknown,
            correlations: CapabilityAvailability::Unknown,
            temporal: CapabilityAvailability::Unknown,
            determinism: CapabilityAvailability::Unknown,
            provenance: CapabilityAvailability::Unknown,
            execution: CapabilityAvailability::Unknown,
            numerical: CapabilityAvailability::Unknown,
        }
    }

    /// Supplies concrete target facts.
    #[must_use]
    pub fn with_facts(mut self, facts: TargetFacts) -> Self {
        self.facts = facts;
        self
    }

    /// Declares calibration support.
    #[must_use]
    pub const fn with_calibration(
        mut self,
        availability: CapabilityAvailability,
    ) -> Self {
        self.calibration = availability;
        self
    }

    /// Declares characterization support.
    #[must_use]
    pub const fn with_characterization(
        mut self,
        availability: CapabilityAvailability,
    ) -> Self {
        self.characterization = availability;
        self
    }

    /// Declares correlation support.
    #[must_use]
    pub const fn with_correlations(
        mut self,
        availability: CapabilityAvailability,
    ) -> Self {
        self.correlations = availability;
        self
    }

    /// Declares temporal support.
    #[must_use]
    pub const fn with_temporal(
        mut self,
        availability: CapabilityAvailability,
    ) -> Self {
        self.temporal = availability;
        self
    }

    /// Declares deterministic execution support.
    #[must_use]
    pub const fn with_determinism(
        mut self,
        availability: CapabilityAvailability,
    ) -> Self {
        self.determinism = availability;
        self
    }

    /// Declares provenance support.
    #[must_use]
    pub const fn with_provenance(
        mut self,
        availability: CapabilityAvailability,
    ) -> Self {
        self.provenance = availability;
        self
    }

    /// Declares execution support.
    #[must_use]
    pub const fn with_execution(
        mut self,
        availability: CapabilityAvailability,
    ) -> Self {
        self.execution = availability;
        self
    }

    /// Declares numerical support.
    #[must_use]
    pub const fn with_numerical(
        mut self,
        availability: CapabilityAvailability,
    ) -> Self {
        self.numerical = availability;
        self
    }

    /// Returns target facts.
    #[must_use]
    pub const fn facts(&self) -> &TargetFacts {
        &self.facts
    }

    /// Returns calibration availability.
    #[must_use]
    pub const fn calibration(&self) -> CapabilityAvailability {
        self.calibration
    }

    /// Returns characterization availability.
    #[must_use]
    pub const fn characterization(&self) -> CapabilityAvailability {
        self.characterization
    }

    /// Returns correlation availability.
    #[must_use]
    pub const fn correlations(&self) -> CapabilityAvailability {
        self.correlations
    }

    /// Returns temporal availability.
    #[must_use]
    pub const fn temporal(&self) -> CapabilityAvailability {
        self.temporal
    }

    /// Returns determinism availability.
    #[must_use]
    pub const fn determinism(&self) -> CapabilityAvailability {
        self.determinism
    }

    /// Returns provenance availability.
    #[must_use]
    pub const fn provenance(&self) -> CapabilityAvailability {
        self.provenance
    }

    /// Returns execution availability.
    #[must_use]
    pub const fn execution(&self) -> CapabilityAvailability {
        self.execution
    }

    /// Returns numerical availability.
    #[must_use]
    pub const fn numerical(&self) -> CapabilityAvailability {
        self.numerical
    }
}

/// Availability state for a target fact.
///
/// Unknown is intentionally distinct from unsupported.
///
/// Unknown means the compatibility caller did not supply enough information.
/// It must never be interpreted as support.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CapabilityAvailability {
    /// The requested property is known to be supported.
    Supported,

    /// The requested property is known not to be supported.
    Unsupported,

    /// The compatibility engine has insufficient information.
    #[default]
    Unknown,
}

// =============================================================================
// Compatibility report
// =============================================================================

/// Complete deterministic compatibility report.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompatibilityReport {
    policy: CompatibilityPolicy,
    status: CompatibilityStatus,
    capability_findings: Vec<CapabilityFinding>,
    findings: Vec<CompatibilityFinding>,
}

impl CompatibilityReport {
    fn new(policy: CompatibilityPolicy) -> Self {
        Self {
            policy,
            status: CompatibilityStatus::Compatible,
            capability_findings: Vec::new(),
            findings: Vec::new(),
        }
    }

    /// Returns the policy used for evaluation.
    #[must_use]
    pub const fn policy(&self) -> CompatibilityPolicy {
        self.policy
    }

    /// Returns the final compatibility status.
    #[must_use]
    pub const fn status(&self) -> CompatibilityStatus {
        self.status
    }

    /// Returns whether execution is compatible.
    #[must_use]
    pub const fn is_compatible(&self) -> bool {
        self.status.is_compatible()
    }

    /// Returns whether compatibility is exact.
    #[must_use]
    pub const fn is_exact(&self) -> bool {
        self.status.is_exact()
    }

    /// Returns whether accepted approximation is involved.
    #[must_use]
    pub const fn uses_approximation(&self) -> bool {
        self.status.uses_approximation()
    }

    /// Returns whether execution must be rejected.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        self.status.is_rejected()
    }

    /// Returns capability-specific findings.
    #[must_use]
    pub fn capability_findings(&self) -> &[CapabilityFinding] {
        &self.capability_findings
    }

    /// Returns non-capability findings.
    #[must_use]
    pub fn findings(&self) -> &[CompatibilityFinding] {
        &self.findings
    }

    /// Returns every failed compatibility finding.
    pub fn failures(&self) -> impl Iterator<Item = &CompatibilityFinding> {
        self.findings.iter().filter(|finding| {
            matches!(
                finding,
                CompatibilityFinding::Incompatible { .. }
                    | CompatibilityFinding::Undetermined { .. }
            )
        })
    }

    /// Returns every approximate finding.
    pub fn approximations(
        &self,
    ) -> impl Iterator<Item = &CompatibilityFinding> {
        self.findings
            .iter()
            .filter(|finding| finding.is_approximate())
    }

    fn push_capability(&mut self, finding: CapabilityFinding) {
        if !finding.is_compatible() {
            self.status = CompatibilityStatus::Incompatible;
        } else if finding.is_approximate()
            && self.status == CompatibilityStatus::Compatible
        {
            self.status = CompatibilityStatus::CompatibleWithApproximation;
        }

        self.capability_findings.push(finding);
    }

    fn push(&mut self, finding: CompatibilityFinding) {
        match finding {
            CompatibilityFinding::Satisfied { .. } => {}

            CompatibilityFinding::Approximate { .. } => {
                if self.status == CompatibilityStatus::Compatible {
                    self.status =
                        CompatibilityStatus::CompatibleWithApproximation;
                }
            }

            CompatibilityFinding::Incompatible { .. } => {
                self.status = CompatibilityStatus::Incompatible;
            }

            CompatibilityFinding::Undetermined { .. } => {
                if self.status != CompatibilityStatus::Incompatible {
                    self.status = CompatibilityStatus::Undetermined;
                }
            }
        }

        self.findings.push(finding);
    }
}

// =============================================================================
// Compatibility engine
// =============================================================================

/// Deterministic compatibility evaluator.
#[derive(Clone, Copy, Debug, Default)]
pub struct CompatibilityEngine;

impl CompatibilityEngine {
    /// Creates an evaluator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Evaluates target capabilities against requirements using exact-only
    /// policy.
    ///
    /// This is the safe default.
    #[must_use]
    pub fn evaluate(
        &self,
        requirements: &TargetRequirements,
        target: &TargetCapabilities,
    ) -> CompatibilityReport {
        self.evaluate_with_context(
            requirements,
            target,
            &CompatibilityContext::new(),
            CompatibilityPolicy::ExactOnly,
        )
    }

    /// Evaluates target capabilities using an explicit policy.
    #[must_use]
    pub fn evaluate_with_policy(
        &self,
        requirements: &TargetRequirements,
        target: &TargetCapabilities,
        policy: CompatibilityPolicy,
    ) -> CompatibilityReport {
        self.evaluate_with_context(
            requirements,
            target,
            &CompatibilityContext::new(),
            policy,
        )
    }

    /// Performs complete compatibility evaluation with explicit target facts.
    #[must_use]
    pub fn evaluate_with_context(
        &self,
        requirements: &TargetRequirements,
        target: &TargetCapabilities,
        context: &CompatibilityContext,
        policy: CompatibilityPolicy,
    ) -> CompatibilityReport {
        let mut report = CompatibilityReport::new(policy);

        if let Err(error) = requirements.validate() {
            report.push(CompatibilityFinding::Incompatible {
                category: RequirementCategory::Approximation,
                description: error.to_string(),
            });

            return report;
        }

        self.evaluate_capabilities(
            requirements,
            target,
            policy,
            &mut report,
        );

        self.evaluate_representations(
            requirements,
            target,
            policy,
            &mut report,
        );

        self.evaluate_modalities(requirements, context, &mut report);
        self.evaluate_resources(requirements, context, &mut report);
        self.evaluate_correlations(requirements, context, &mut report);
        self.evaluate_temporal(requirements, context, &mut report);
        self.evaluate_calibration(requirements, context, &mut report);
        self.evaluate_characterization(requirements, context, &mut report);
        self.evaluate_determinism(requirements, context, &mut report);
        self.evaluate_provenance(requirements, context, &mut report);
        self.evaluate_execution(requirements, context, &mut report);
        self.evaluate_numerical(requirements, context, &mut report);
        self.evaluate_global_approximation(requirements, &mut report);

        report
    }

    fn evaluate_capabilities(
        &self,
        requirements: &TargetRequirements,
        target: &TargetCapabilities,
        policy: CompatibilityPolicy,
        report: &mut CompatibilityReport,
    ) {
        for required in requirements.iter_capabilities() {
            let core_requirement = convert_required_capability(required);

            let result = target.evaluate_with_policy(
                &core_requirement,
                policy.target_policy(),
            );

            report.push_capability(CapabilityFinding {
                requirement: required.clone(),
                result,
            });
        }
    }

    fn evaluate_representations(
        &self,
        requirements: &TargetRequirements,
        target: &TargetCapabilities,
        policy: CompatibilityPolicy,
        report: &mut CompatibilityReport,
    ) {
        for requirement in requirements.iter_representations() {
            let core_requirement = convert_representation_requirement(
                requirement,
            );

            let result = target.evaluate_with_policy(
                &core_requirement,
                policy.target_policy(),
            );

            let description = format!(
                "representation `{}`",
                requirement.capability
            );

            match result {
                TargetCapabilityMatch::Satisfied { .. } => {
                    report.push(CompatibilityFinding::Satisfied {
                        category: RequirementCategory::Representation,
                        description,
                    });
                }

                TargetCapabilityMatch::Approximate { .. } => {
                    report.push(CompatibilityFinding::Approximate {
                        category: RequirementCategory::Representation,
                        description,
                    });
                }

                TargetCapabilityMatch::Missing { .. }
                | TargetCapabilityMatch::Rejected { .. } => {
                    report.push(CompatibilityFinding::Incompatible {
                        category: RequirementCategory::Representation,
                        description,
                    });
                }
            }
        }
    }

    fn evaluate_modalities(
        &self,
        requirements: &TargetRequirements,
        context: &CompatibilityContext,
        report: &mut CompatibilityReport,
    ) {
        for requirement in requirements.iter_modalities() {
            if context.facts().supports_modality(requirement) {
                report.push(CompatibilityFinding::Satisfied {
                    category: RequirementCategory::Modality,
                    description: format!(
                        "modality `{}`",
                        requirement.as_str()
                    ),
                });
            } else {
                report.push(CompatibilityFinding::Undetermined {
                    category: RequirementCategory::Modality,
                    description: format!(
                        "target modality `{}` was not supplied",
                        requirement.as_str()
                    ),
                });
            }
        }
    }

    fn evaluate_resources(
        &self,
        requirements: &TargetRequirements,
        context: &CompatibilityContext,
        report: &mut CompatibilityReport,
    ) {
        let required = requirements.resources();

        evaluate_resource(
            required.logical_resources,
            context.facts().logical_resources(),
            "logical quantum resources",
            report,
        );

        evaluate_resource(
            required.physical_resources,
            context.facts().physical_resources(),
            "physical quantum resources",
            report,
        );

        evaluate_resource(
            required.classical_memory_bytes,
            context.facts().classical_memory_bytes(),
            "classical memory bytes",
            report,
        );

        evaluate_resource(
            required.device_memory_bytes,
            context.facts().device_memory_bytes(),
            "device memory bytes",
            report,
        );

        evaluate_resource(
            required.operation_count,
            context.facts().operation_count(),
            "operation count",
            report,
        );

        evaluate_resource(
            required.depth,
            context.facts().depth(),
            "execution depth",
            report,
        );

        evaluate_resource(
            required.shots,
            context.facts().shots(),
            "sampling shots",
            report,
        );
    }

    fn evaluate_correlations(
        &self,
        requirements: &TargetRequirements,
        context: &CompatibilityContext,
        report: &mut CompatibilityReport,
    ) {
        let required = requirements.correlations();

        evaluate_boolean_requirement(
            required.spatial,
            context.correlations(),
            "spatial correlation",
            RequirementCategory::Correlation,
            report,
        );

        evaluate_boolean_requirement(
            required.temporal,
            context.correlations(),
            "temporal correlation",
            RequirementCategory::Correlation,
            report,
        );

        evaluate_boolean_requirement(
            required.crosstalk,
            context.correlations(),
            "crosstalk",
            RequirementCategory::Correlation,
            report,
        );

        evaluate_boolean_requirement(
            required.non_markovian,
            context.correlations(),
            "non-Markovian correlation",
            RequirementCategory::Correlation,
            report,
        );

        evaluate_boolean_requirement(
            required.conditional,
            context.correlations(),
            "conditional correlation",
            RequirementCategory::Correlation,
            report,
        );
    }

    fn evaluate_temporal(
        &self,
        requirements: &TargetRequirements,
        context: &CompatibilityContext,
        report: &mut CompatibilityReport,
    ) {
        let required = requirements.temporal();

        evaluate_boolean_requirement(
            required.time_dependent,
            context.temporal(),
            "time-dependent noise",
            RequirementCategory::Temporal,
            report,
        );

        evaluate_boolean_requirement(
            required.duration_aware,
            context.temporal(),
            "duration-aware noise",
            RequirementCategory::Temporal,
            report,
        );

        evaluate_boolean_requirement(
            required.calibration_time_awareness,
            context.temporal(),
            "calibration time awareness",
            RequirementCategory::Temporal,
            report,
        );

        evaluate_boolean_requirement(
            required.history_dependent,
            context.temporal(),
            "history-dependent noise",
            RequirementCategory::Temporal,
            report,
        );
    }

    fn evaluate_calibration(
        &self,
        requirements: &TargetRequirements,
        context: &CompatibilityContext,
        report: &mut CompatibilityReport,
    ) {
        evaluate_calibration_requirements(
            requirements.calibration(),
            context.calibration(),
            report,
        );
    }

    fn evaluate_characterization(
        &self,
        requirements: &TargetRequirements,
        context: &CompatibilityContext,
        report: &mut CompatibilityReport,
    ) {
        evaluate_characterization_requirements(
            requirements.characterization(),
            context.characterization(),
            report,
        );
    }

    fn evaluate_determinism(
        &self,
        requirements: &TargetRequirements,
        context: &CompatibilityContext,
        report: &mut CompatibilityReport,
    ) {
        let required = requirements.determinism();

        evaluate_boolean_requirement(
            required.reproducible_sampling,
            context.determinism(),
            "reproducible sampling",
            RequirementCategory::Determinism,
            report,
        );

        evaluate_boolean_requirement(
            required.deterministic_parallelism,
            context.determinism(),
            "deterministic parallelism",
            RequirementCategory::Determinism,
            report,
        );

        evaluate_boolean_requirement(
            required.explicit_seed,
            context.determinism(),
            "explicit stochastic seed control",
            RequirementCategory::Determinism,
            report,
        );
    }

    fn evaluate_provenance(
        &self,
        requirements: &TargetRequirements,
        context: &CompatibilityContext,
        report: &mut CompatibilityReport,
    ) {
        let required = requirements.provenance();

        evaluate_boolean_requirement(
            required.required,
            context.provenance(),
            "provenance",
            RequirementCategory::Provenance,
            report,
        );

        evaluate_boolean_requirement(
            required.model_identity,
            context.provenance(),
            "model identity provenance",
            RequirementCategory::Provenance,
            report,
        );

        evaluate_boolean_requirement(
            required.calibration_identity,
            context.provenance(),
            "calibration identity provenance",
            RequirementCategory::Provenance,
            report,
        );

        evaluate_boolean_requirement(
            required.target_identity,
            context.provenance(),
            "target identity provenance",
            RequirementCategory::Provenance,
            report,
        );

        evaluate_boolean_requirement(
            required.execution_identity,
            context.provenance(),
            "execution identity provenance",
            RequirementCategory::Provenance,
            report,
        );
    }

    fn evaluate_execution(
        &self,
        requirements: &TargetRequirements,
        context: &CompatibilityContext,
        report: &mut CompatibilityReport,
    ) {
        let required = requirements.execution();

        evaluate_boolean_requirement(
            required.cancellation,
            context.execution(),
            "execution cancellation",
            RequirementCategory::Execution,
            report,
        );

        evaluate_boolean_requirement(
            required.streaming,
            context.execution(),
            "streaming execution",
            RequirementCategory::Execution,
            report,
        );

        evaluate_boolean_requirement(
            required.distributed,
            context.execution(),
            "distributed execution",
            RequirementCategory::Execution,
            report,
        );

        evaluate_boolean_requirement(
            required.remote_execution,
            context.execution(),
            "remote execution",
            RequirementCategory::Execution,
            report,
        );
    }

    fn evaluate_numerical(
        &self,
        requirements: &TargetRequirements,
        context: &CompatibilityContext,
        report: &mut CompatibilityReport,
    ) {
        let numerical = requirements.numerical();

        if let Err(error) = numerical.validate() {
            report.push(CompatibilityFinding::Incompatible {
                category: RequirementCategory::Numerical,
                description: error.to_string(),
            });

            return;
        }

        if numerical != NumericalRequirements::exact()
            && numerical.strength
                != super::requirements::RequirementStrength::Exact
        {
            evaluate_boolean_requirement(
                true,
                context.numerical(),
                "requested numerical guarantee",
                RequirementCategory::Numerical,
                report,
            );
        }
    }

    fn evaluate_global_approximation(
        &self,
        requirements: &TargetRequirements,
        report: &mut CompatibilityReport,
    ) {
        let approximation = requirements.approximation();

        if let Err(error) = approximation.validate() {
            report.push(CompatibilityFinding::Incompatible {
                category: RequirementCategory::Approximation,
                description: error.to_string(),
            });
        }
    }
}

// =============================================================================
// Public convenience functions
// =============================================================================

/// Evaluates requirements against target capabilities using exact-only policy.
///
/// This is the safe default and must be preferred unless the caller has
/// explicitly decided that approximation is acceptable.
#[must_use]
pub fn check_compatibility(
    requirements: &TargetRequirements,
    target: &TargetCapabilities,
) -> CompatibilityReport {
    CompatibilityEngine::new().evaluate(requirements, target)
}

/// Evaluates requirements against target capabilities using an explicit policy.
#[must_use]
pub fn check_compatibility_with_policy(
    requirements: &TargetRequirements,
    target: &TargetCapabilities,
    policy: CompatibilityPolicy,
) -> CompatibilityReport {
    CompatibilityEngine::new()
        .evaluate_with_policy(requirements, target, policy)
}

/// Performs complete compatibility evaluation with explicit target facts.
#[must_use]
pub fn check_compatibility_with_context(
    requirements: &TargetRequirements,
    target: &TargetCapabilities,
    context: &CompatibilityContext,
    policy: CompatibilityPolicy,
) -> CompatibilityReport {
    CompatibilityEngine::new().evaluate_with_context(
        requirements,
        target,
        context,
        policy,
    )
}

// =============================================================================
// Conversion helpers
// =============================================================================

fn convert_required_capability(
    requirement: &RequiredCapability,
) -> super::super::core::capabilities::CapabilityRequirement {
    use super::super::core::capabilities::{
        CapabilityRequirement,
        CapabilityScope,
        SupportRequirement,
    };

    let scope = match requirement.scope() {
        super::requirements::RequirementScope::Global => {
            CapabilityScope::Global
        }

        super::requirements::RequirementScope::LogicalQubit(id) => {
            CapabilityScope::LogicalQubit(id)
        }

        super::requirements::RequirementScope::PhysicalQubit(id) => {
            CapabilityScope::PhysicalQubit(id)
        }
    };

    let support = match requirement.strength() {
        super::requirements::RequirementStrength::Exact
        | super::requirements::RequirementStrength::Bounded
        | super::requirements::RequirementStrength::Statistical => {
            SupportRequirement::Exact
        }

        super::requirements::RequirementStrength::Approximate => {
            SupportRequirement::Approximate
        }
    };

    CapabilityRequirement::new(
        requirement.capability().clone(),
        scope,
        support,
    )
}

fn convert_representation_requirement(
    requirement: &RepresentationRequirement,
) -> super::super::core::capabilities::CapabilityRequirement {
    use super::super::core::capabilities::{
        CapabilityRequirement,
        CapabilityScope,
        SupportRequirement,
    };

    let support = match requirement.strength {
        super::requirements::RequirementStrength::Exact
        | super::requirements::RequirementStrength::Bounded
        | super::requirements::RequirementStrength::Statistical => {
            SupportRequirement::Exact
        }

        super::requirements::RequirementStrength::Approximate => {
            SupportRequirement::Approximate
        }
    };

    CapabilityRequirement::new(
        requirement.capability.clone(),
        CapabilityScope::Global,
        support,
    )
}

// =============================================================================
// Generic evaluation helpers
// =============================================================================

fn evaluate_resource(
    required: Option<u64>,
    available: Option<u64>,
    name: &'static str,
    report: &mut CompatibilityReport,
) {
    let Some(required) = required else {
        return;
    };

    match available {
        Some(available) if available >= required => {
            report.push(CompatibilityFinding::Satisfied {
                category: RequirementCategory::Resource,
                description: format!(
                    "{name}: required={required}, available={available}"
                ),
            });
        }

        Some(available) => {
            report.push(CompatibilityFinding::Incompatible {
                category: RequirementCategory::Resource,
                description: format!(
                    "{name}: required={required}, available={available}"
                ),
            });
        }

        None => {
            report.push(CompatibilityFinding::Undetermined {
                category: RequirementCategory::Resource,
                description: format!(
                    "{name}: required={required}, available=unknown"
                ),
            });
        }
    }
}

fn evaluate_boolean_requirement(
    required: bool,
    availability: CapabilityAvailability,
    name: &'static str,
    category: RequirementCategory,
    report: &mut CompatibilityReport,
) {
    if !required {
        return;
    }

    match availability {
        CapabilityAvailability::Supported => {
            report.push(CompatibilityFinding::Satisfied {
                category,
                description: format!("{name}: supported"),
            });
        }

        CapabilityAvailability::Unsupported => {
            report.push(CompatibilityFinding::Incompatible {
                category,
                description: format!("{name}: unsupported"),
            });
        }

        CapabilityAvailability::Unknown => {
            report.push(CompatibilityFinding::Undetermined {
                category,
                description: format!("{name}: support unknown"),
            });
        }
    }
}

fn evaluate_calibration_requirements(
    requirements: CalibrationRequirements,
    availability: CapabilityAvailability,
    report: &mut CompatibilityReport,
) {
    evaluate_boolean_requirement(
        requirements.required,
        availability,
        "calibration",
        RequirementCategory::Calibration,
        report,
    );

    evaluate_boolean_requirement(
        requirements.validity_interval,
        availability,
        "calibration validity interval",
        RequirementCategory::Calibration,
        report,
    );

    evaluate_boolean_requirement(
        requirements.uncertainty,
        availability,
        "calibration uncertainty",
        RequirementCategory::Calibration,
        report,
    );

    evaluate_boolean_requirement(
        requirements.provenance,
        availability,
        "calibration provenance",
        RequirementCategory::Calibration,
        report,
    );

    evaluate_boolean_requirement(
        requirements.drift,
        availability,
        "calibration drift",
        RequirementCategory::Calibration,
        report,
    );
}

fn evaluate_characterization_requirements(
    requirements: CharacterizationRequirements,
    availability: CapabilityAvailability,
    report: &mut CompatibilityReport,
) {
    evaluate_boolean_requirement(
        requirements.required,
        availability,
        "characterization",
        RequirementCategory::Characterization,
        report,
    );

    evaluate_boolean_requirement(
        requirements.process_characterization,
        availability,
        "process characterization",
        RequirementCategory::Characterization,
        report,
    );

    evaluate_boolean_requirement(
        requirements.randomized_benchmarking,
        availability,
        "randomized benchmarking",
        RequirementCategory::Characterization,
        report,
    );

    evaluate_boolean_requirement(
        requirements.uncertainty,
        availability,
        "characterization uncertainty",
        RequirementCategory::Characterization,
        report,
    );

    evaluate_boolean_requirement(
        requirements.raw_observations,
        availability,
        "raw characterization observations",
        RequirementCategory::Characterization,
        report,
    );
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::capabilities::{
        TargetCapabilities,
    };
    use super::super::requirements::{
        ModalityRequirement,
        RequiredCapability,
        RequirementScope,
        TargetRequirements,
    };
    use crate::quantum::zqn::core::capabilities::{
        Capability,
        CapabilityId,
        CapabilityScope,
        SupportLevel,
    };

    #[test]
    fn empty_requirements_are_compatible() {
        let requirements = TargetRequirements::new();
        let target = TargetCapabilities::new();

        let report = check_compatibility(&requirements, &target);

        assert!(report.is_compatible());
        assert!(report.is_exact());
        assert!(!report.uses_approximation());
    }

    #[test]
    fn native_capability_satisfies_native_requirement() {
        let capability_id = CapabilityId::noise_readout();

        let mut target = TargetCapabilities::new();
        target.insert(Capability::native(
            capability_id.clone(),
            CapabilityScope::Global,
        ));

        let mut requirements = TargetRequirements::new();

        requirements
            .require_capability(RequiredCapability::scoped(
                capability_id,
                RequirementScope::Global,
                super::super::requirements::RequirementStrength::Exact,
                ApproximationPolicy::Exact,
            ))
            .expect("valid requirement");

        let report = check_compatibility(&requirements, &target);

        assert!(report.is_compatible());
        assert!(report.is_exact());
    }

    #[test]
    fn missing_capability_is_incompatible() {
        let capability_id = CapabilityId::noise_readout();

        let mut requirements = TargetRequirements::new();

        requirements
            .require_capability(RequiredCapability::scoped(
                capability_id,
                RequirementScope::Global,
                super::super::requirements::RequirementStrength::Exact,
                ApproximationPolicy::Exact,
            ))
            .expect("valid requirement");

        let report =
            check_compatibility(&requirements, &TargetCapabilities::new());

        assert!(!report.is_compatible());
        assert_eq!(
            report.status(),
            CompatibilityStatus::Incompatible
        );
    }

    #[test]
    fn approximate_support_is_rejected_by_default() {
        let capability_id = CapabilityId::noise_readout();

        let mut target = TargetCapabilities::new();
        target.insert(Capability::approximate(
            capability_id.clone(),
            CapabilityScope::Global,
        ));

        let mut requirements = TargetRequirements::new();

        requirements
            .require_capability(RequiredCapability::scoped(
                capability_id,
                RequirementScope::Global,
                super::super::requirements::RequirementStrength::Approximate,
                ApproximationPolicy::AbsoluteTolerance(0.01),
            ))
            .expect("valid requirement");

        let report = check_compatibility(&requirements, &target);

        assert!(!report.is_compatible());
    }

    #[test]
    fn approximate_support_can_be_explicitly_enabled() {
        let capability_id = CapabilityId::noise_readout();

        let mut target = TargetCapabilities::new();
        target.insert(Capability::approximate(
            capability_id.clone(),
            CapabilityScope::Global,
        ));

        let mut requirements = TargetRequirements::new();

        requirements
            .require_capability(RequiredCapability::scoped(
                capability_id,
                RequirementScope::Global,
                super::super::requirements::RequirementStrength::Approximate,
                ApproximationPolicy::AbsoluteTolerance(0.01),
            ))
            .expect("valid requirement");

        let report = check_compatibility_with_policy(
            &requirements,
            &target,
            CompatibilityPolicy::AllowApproximate,
        );

        assert!(report.is_compatible());
        assert!(report.uses_approximation());
    }

    #[test]
    fn native_policy_rejects_emulated_support() {
        let capability_id = CapabilityId::noise_readout();

        let mut target = TargetCapabilities::new();
        target.insert(Capability::emulated(
            capability_id.clone(),
            CapabilityScope::Global,
        ));

        let mut requirements = TargetRequirements::new();

        requirements
            .require_capability(RequiredCapability::scoped(
                capability_id,
                RequirementScope::Global,
                super::super::requirements::RequirementStrength::Exact,
                ApproximationPolicy::Exact,
            ))
            .expect("valid requirement");

        let report = check_compatibility_with_policy(
            &requirements,
            &target,
            CompatibilityPolicy::NativeOnly,
        );

        assert!(!report.is_compatible());
    }

    #[test]
    fn exact_requirement_accepts_exact_emulation() {
        let capability_id = CapabilityId::noise_readout();

        let mut target = TargetCapabilities::new();
        target.insert(Capability::emulated(
            capability_id.clone(),
            CapabilityScope::Global,
        ));

        let mut requirements = TargetRequirements::new();

        requirements
            .require_capability(RequiredCapability::scoped(
                capability_id,
                RequirementScope::Global,
                super::super::requirements::RequirementStrength::Exact,
                ApproximationPolicy::Exact,
            ))
            .expect("valid requirement");

        let report = check_compatibility(&requirements, &target);

        assert!(report.is_compatible());
        assert!(report.is_exact());
    }

    #[test]
    fn resource_requirement_needs_explicit_target_capacity() {
        let requirements = TargetRequirements::new();

        let mut requirements = requirements;

        requirements
            .set_resources(ResourceRequirements {
                logical_resources: Some(8),
                physical_resources: None,
                classical_memory_bytes: None,
                device_memory_bytes: None,
                operation_count: None,
                depth: None,
                shots: None,
            })
            .expect("valid resources");

        let report =
            check_compatibility(&requirements, &TargetCapabilities::new());

        assert!(!report.is_compatible());
        assert_eq!(
            report.status(),
            CompatibilityStatus::Undetermined
        );
    }

    #[test]
    fn sufficient_resource_capacity_is_compatible() {
        let mut requirements = TargetRequirements::new();

        requirements
            .set_resources(ResourceRequirements {
                logical_resources: Some(8),
                physical_resources: Some(12),
                classical_memory_bytes: None,
                device_memory_bytes: None,
                operation_count: None,
                depth: None,
                shots: None,
            })
            .expect("valid resources");

        let facts = TargetFacts::new()
            .with_logical_resources(8)
            .with_physical_resources(12);

        let context = CompatibilityContext::new()
            .with_facts(facts);

        let report = check_compatibility_with_context(
            &requirements,
            &TargetCapabilities::new(),
            &context,
            CompatibilityPolicy::ExactOnly,
        );

        assert!(report.is_compatible());
    }

    #[test]
    fn insufficient_resource_capacity_is_incompatible() {
        let mut requirements = TargetRequirements::new();

        requirements
            .set_resources(ResourceRequirements {
                logical_resources: Some(8),
                physical_resources: None,
                classical_memory_bytes: None,
                device_memory_bytes: None,
                operation_count: None,
                depth: None,
                shots: None,
            })
            .expect("valid resources");

        let context = CompatibilityContext::new()
            .with_facts(TargetFacts::new().with_logical_resources(4));

        let report = check_compatibility_with_context(
            &requirements,
            &TargetCapabilities::new(),
            &context,
            CompatibilityPolicy::ExactOnly,
        );

        assert!(!report.is_compatible());
        assert_eq!(
            report.status(),
            CompatibilityStatus::Incompatible
        );
    }

    #[test]
    fn modality_requires_explicit_target_fact() {
        let modality =
            ModalityRequirement::new("qubit-gate").expect("valid modality");

        let mut requirements = TargetRequirements::new();

        requirements
            .require_modality(modality.clone())
            .expect("valid modality requirement");

        let report =
            check_compatibility(&requirements, &TargetCapabilities::new());

        assert!(!report.is_compatible());
        assert_eq!(
            report.status(),
            CompatibilityStatus::Undetermined
        );
    }

    #[test]
    fn modality_is_compatible_when_explicitly_declared() {
        let modality =
            ModalityRequirement::new("qubit-gate").expect("valid modality");

        let mut requirements = TargetRequirements::new();

        requirements
            .require_modality(modality)
            .expect("valid modality requirement");

        let context = CompatibilityContext::new().with_facts(
            TargetFacts::new().with_modalities(["qubit-gate"]),
        );

        let report = check_compatibility_with_context(
            &requirements,
            &TargetCapabilities::new(),
            &context,
            CompatibilityPolicy::ExactOnly,
        );

        assert!(report.is_compatible());
    }

    #[test]
    fn canonical_qubit_scope_is_not_reinterpreted() {
        use crate::quantum::ir::qubit::PhysicalQubitId;

        let capability_id = CapabilityId::noise_readout();

        let mut target = TargetCapabilities::new();

        target.insert(Capability::native(
            capability_id.clone(),
            CapabilityScope::PhysicalQubit(
                PhysicalQubitId::new(7),
            ),
        ));

        let mut requirements = TargetRequirements::new();

        requirements
            .require_capability(RequiredCapability::scoped(
                capability_id,
                RequirementScope::PhysicalQubit(
                    PhysicalQubitId::new(8),
                ),
                super::super::requirements::RequirementStrength::Exact,
                ApproximationPolicy::Exact,
            ))
            .expect("valid requirement");

        let report = check_compatibility(&requirements, &target);

        assert!(!report.is_compatible());
    }

    #[test]
    fn compatibility_has_no_hidden_randomness() {
        let requirements = TargetRequirements::new();
        let target = TargetCapabilities::new();

        let first = check_compatibility(&requirements, &target);
        let second = check_compatibility(&requirements, &target);

        assert_eq!(first, second);
    }

    #[test]
    fn support_level_is_not_reinterpreted() {
        let capability_id = CapabilityId::noise_readout();

        let native = Capability::native(
            capability_id.clone(),
            CapabilityScope::Global,
        );

        let unsupported = Capability::new(
            capability_id,
            CapabilityScope::Resource(
                "example".to_owned(),
            ),
            SupportLevel::Unsupported,
        );

        assert!(native.is_supported());
        assert!(!unsupported.is_supported());
    }
}