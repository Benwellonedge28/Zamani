//! Zamani Quantum Resilience — Mitigation Strategy Contract
//!
//! Path:
//!     src/quantum/resilience/mitigation/strategy.rs
//!
//! Purpose:
//!     Defines the stable, provider-independent contract implemented by every
//!     quantum error-mitigation strategy in Zamani.
//!
//! Architectural position:
//!
//! ```text
//!                    Canonical Zamani Program / IR
//!                               |
//!                               v
//!                       Resilience Controller
//!                               |
//!                +--------------+--------------+
//!                |                             |
//!                v                             v
//!          Mitigation Selection          Mitigation Policy
//!                |                             |
//!                +--------------+--------------+
//!                               |
//!                               v
//!                     MitigationStrategy
//!                               |
//!                  +------------+------------+
//!                  |            |            |
//!                  v            v            v
//!                Readout       ZNE        Twirling
//!             mitigation               ... future ...
//!                  |
//!                  v
//!                    Mitigation Executor
//!                  |
//!                  v
//!              Quantum Execution
//!                  |
//!                  v
//!                Verification
//! ```
//!
//! This file is a DOMAIN/CONTRACT module.
//!
//! It MUST NOT:
//!
//! - execute a quantum circuit;
//! - communicate with a backend/provider;
//! - access hardware directly;
//! - perform routing;
//! - perform scheduling;
//! - compile circuits;
//! - optimize circuits;
//! - implement QEC;
//! - inspect provider credentials;
//! - perform filesystem/network I/O;
//! - contain retry loops;
//! - contain fixed machine-size limits;
//! - contain provider-specific branching;
//! - contain global mutable state;
//! - silently select a mitigation strategy;
//! - silently mutate execution state.
//!
//! Those responsibilities belong to other resilience and quantum subsystems.
//!
//! -----------------------------------------------------------------------------
//! Design goals
//! -----------------------------------------------------------------------------
//!
//! 1. Write once, scale everywhere.
//!
//!    A mitigation strategy must operate on workload/capability contracts,
//!    never on assumptions such as "127 qubits", "device 7", or a particular
//!    provider.
//!
//! 2. Strategy selection is separate from strategy execution.
//!
//!    `selection.rs` chooses a strategy.
//!    `executor.rs` executes a selected strategy.
//!
//! 3. Strategies are declarative where possible.
//!
//!    The contract exposes requirements, expected overhead, applicability and
//!    deterministic identity without requiring a concrete backend.
//!
//! 4. Semantic correctness remains authoritative.
//!
//!    Mitigation may improve an estimate, but it must never silently redefine
//!    the program's semantics.
//!
//! 5. No artificial scalability ceiling.
//!
//!    This module contains no maximum qubit count, maximum circuit size,
//!    maximum shot count, or maximum number of resources.
//!
//! 6. Deterministic identity.
//!
//!    Every strategy has a stable identifier and semantic version.
//!
//! 7. Extensibility.
//!
//!    New strategies can implement the trait without changing this file.
//!
//! 8. Safe Rust.
//!
//!    Rust 1.97 / 1.97.1, Rust 2021, no `unsafe`.
//!
//! -----------------------------------------------------------------------------
//! Integration contract
//! -----------------------------------------------------------------------------
//!
//! `mitigation/selection.rs`
//!     Consumes `MitigationStrategy`, requirements, applicability and overhead
//!     information to select a suitable strategy.
//!
//! `mitigation/executor.rs`
//!     Executes the strategy selected by the selection layer.
//!
//! `mitigation/readout.rs`
//!     Implements readout/measurement mitigation using this contract.
//!
//! `mitigation/zero_noise.rs`
//!     Implements zero-noise extrapolation using this contract.
//!
//! `mitigation/probabilistic.rs`
//!     Implements probabilistic error cancellation or related methods using
//!     this contract.
//!
//! `mitigation/twirling.rs`
//!     Implements gate/randomized twirling using this contract.
//!
//! `mitigation/dynamical_decoupling.rs`
//!     Implements dynamical-decoupling strategies using this contract.
//!
//! `mitigation/custom.rs`
//!     Provides extension mechanisms for application/domain-specific methods.
//!
//! `registry/strategy.rs`
//!     Registers concrete strategy implementations.
//!
//! `policy/*`
//!     Supplies policy constraints and budgets.
//!
//! `planning/*`
//!     Treats mitigation as a candidate resilience action.
//!
//! `verification/*`
//!     Verifies that mitigation did not invalidate required semantic guarantees.
//!
//! `telemetry/*`
//!     Records strategy identity, version, configuration identity, expected
//!     overhead and execution outcome.
//!
//! `history/*`
//!     Records strategy outcomes for later statistical evaluation.
//!
//! `serialization/*`
//!     Serializes strategy descriptors and immutable strategy metadata.
//!
//! `quantum::ir`
//!     Remains authoritative for program and quantum-operation semantics.
//!
//! `quantum::hardware`
//!     Remains authoritative for hardware capabilities and execution state.
//!
//! `quantum::zqn`
//!     Remains authoritative for quantum fault/noise semantics.
//!
//! -----------------------------------------------------------------------------
//! Important boundary
//! -----------------------------------------------------------------------------
//!
//! A mitigation strategy does NOT mean:
//!
//!     "run this algorithm on hardware"
//!
//! It means:
//!
//!     "this is the formally identified mitigation mechanism, these are the
//!      conditions under which it is applicable, and this is the expected
//!      resource/statistical overhead."
//!
//! Execution belongs to `mitigation/executor.rs`.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

// =============================================================================
// Stable schema
// =============================================================================

/// Stable schema identifier for mitigation strategy contracts.
pub const MITIGATION_STRATEGY_SCHEMA_ID: &str =
    "zamani.quantum.resilience.mitigation.strategy";

/// Semantic version of the mitigation strategy contract.
///
/// This changes only when the externally observable contract changes.
pub const MITIGATION_STRATEGY_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Strategy identity
// =============================================================================

/// Stable identity of a mitigation strategy.
///
/// The identifier is provider-independent and must remain stable once released.
///
/// Examples:
///
/// ```text
/// readout
/// zero_noise_extrapolation
/// probabilistic_error_cancellation
/// randomized_twirling
/// dynamical_decoupling
/// custom.example.strategy
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StrategyId(String);

impl StrategyId {
    /// Creates a strategy identifier.
    ///
    /// The identifier must not be empty or contain ASCII whitespace.
    pub fn new(value: impl Into<String>) -> Result<Self, StrategyContractError> {
        let value = value.into();

        if value.is_empty() {
            return Err(StrategyContractError::EmptyStrategyId);
        }

        if value.chars().any(char::is_whitespace) {
            return Err(StrategyContractError::InvalidStrategyId);
        }

        Ok(Self(value))
    }

    /// Returns the stable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for StrategyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Strategy version
// =============================================================================

/// Semantic version of a mitigation strategy.
///
/// This is deliberately represented numerically so the contract does not
/// depend on a particular external version parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StrategyVersion {
    /// Major semantic version.
    pub major: u16,

    /// Minor semantic version.
    pub minor: u16,

    /// Patch semantic version.
    pub patch: u16,
}

impl StrategyVersion {
    /// Creates a strategy version.
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl Default for StrategyVersion {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for StrategyVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
// Strategy family
// =============================================================================

/// Broad family of quantum error-mitigation techniques.
///
/// This classification is informational and is intentionally independent of
/// provider implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StrategyFamily {
    /// Mitigation applied to classical measurement/readout results.
    Readout,

    /// Noise-scaling and extrapolation techniques.
    ZeroNoiseExtrapolation,

    /// Probabilistic cancellation/reconstruction techniques.
    Probabilistic,

    /// Randomized compiling/twirling techniques.
    Twirling,

    /// Pulse/scheduling-oriented coherence-preservation techniques.
    DynamicalDecoupling,

    /// Strategy supplied by an external/custom implementation.
    Custom,
}

impl StrategyFamily {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Readout => "readout",
            Self::ZeroNoiseExtrapolation => "zero_noise_extrapolation",
            Self::Probabilistic => "probabilistic",
            Self::Twirling => "twirling",
            Self::DynamicalDecoupling => "dynamical_decoupling",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for StrategyFamily {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Strategy phase
// =============================================================================

/// Phase at which a mitigation strategy primarily operates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StrategyPhase {
    /// Operates on a circuit/program before execution.
    PreExecution,

    /// Operates while execution is being prepared or transformed.
    ExecutionPreparation,

    /// Requires multiple executions/shots and combines their results.
    DuringExecution,

    /// Operates on returned classical results.
    PostExecution,

    /// Can span more than one phase.
    CrossPhase,
}

impl StrategyPhase {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreExecution => "pre_execution",
            Self::ExecutionPreparation => "execution_preparation",
            Self::DuringExecution => "during_execution",
            Self::PostExecution => "post_execution",
            Self::CrossPhase => "cross_phase",
        }
    }
}

// =============================================================================
// Capability requirements
// =============================================================================

/// Capability required by a mitigation strategy.
///
/// This enum describes requirements, not a particular implementation.
///
/// The actual capability model remains owned by the hardware/runtime
/// subsystems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StrategyRequirement {
    /// Classical post-processing capability.
    ClassicalPostProcessing,

    /// Access to measurement/readout results.
    MeasurementResults,

    /// Ability to execute a circuit more than once.
    RepeatedExecution,

    /// Ability to execute with configurable noise scaling.
    NoiseScaling,

    /// Ability to vary execution parameters deterministically.
    ParameterVariation,

    /// Ability to randomize eligible circuit operations.
    RandomizedCompilation,

    /// Ability to preserve and record randomness provenance.
    RandomnessProvenance,

    /// Ability to modify scheduling/timing.
    ScheduleControl,

    /// Ability to access timing constraints.
    TimingInformation,

    /// Ability to access pulse/control-level abstractions.
    PulseControl,

    /// Ability to perform sufficiently precise classical statistical analysis.
    StatisticalAnalysis,

    /// Ability to preserve execution provenance.
    Provenance,

    /// Ability to compare multiple executions under a common semantic program.
    CrossExecutionCorrelation,

    /// Ability to execute the same logical workload under compatible variants.
    VariantExecution,

    /// Ability to apply a strategy to the selected logical scope.
    ScopedExecution,

    /// Strategy requires explicit user/policy authorization.
    ExplicitPolicyAuthorization,
}

impl StrategyRequirement {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClassicalPostProcessing => "classical_post_processing",
            Self::MeasurementResults => "measurement_results",
            Self::RepeatedExecution => "repeated_execution",
            Self::NoiseScaling => "noise_scaling",
            Self::ParameterVariation => "parameter_variation",
            Self::RandomizedCompilation => "randomized_compilation",
            Self::RandomnessProvenance => "randomness_provenance",
            Self::ScheduleControl => "schedule_control",
            Self::TimingInformation => "timing_information",
            Self::PulseControl => "pulse_control",
            Self::StatisticalAnalysis => "statistical_analysis",
            Self::Provenance => "provenance",
            Self::CrossExecutionCorrelation => "cross_execution_correlation",
            Self::VariantExecution => "variant_execution",
            Self::ScopedExecution => "scoped_execution",
            Self::ExplicitPolicyAuthorization => "explicit_policy_authorization",
        }
    }
}

// =============================================================================
// Expected overhead
// =============================================================================

/// Resource dimensions affected by a mitigation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OverheadDimension {
    /// Additional quantum/classical execution time.
    Time,

    /// Additional number of executions or shots.
    Executions,

    /// Additional classical computation.
    ClassicalComputation,

    /// Additional quantum operations.
    QuantumOperations,

    /// Additional schedule duration.
    ScheduleDuration,

    /// Additional circuit variants.
    Variants,

    /// Additional memory/storage.
    Memory,

    /// Additional calibration/characterization work.
    Characterization,

    /// Additional statistical uncertainty or sampling burden.
    StatisticalSampling,
}

impl OverheadDimension {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Time => "time",
            Self::Executions => "executions",
            Self::ClassicalComputation => "classical_computation",
            Self::QuantumOperations => "quantum_operations",
            Self::ScheduleDuration => "schedule_duration",
            Self::Variants => "variants",
            Self::Memory => "memory",
            Self::Characterization => "characterization",
            Self::StatisticalSampling => "statistical_sampling",
        }
    }
}

/// Expected mitigation overhead.
///
/// Values are deliberately expressed as policy-neutral qualitative classes
/// instead of hardware-specific constants.
///
/// Concrete quantities belong to the selected strategy configuration and
/// execution planner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OverheadLevel {
    /// No meaningful additional overhead is expected.
    None,

    /// Small additional overhead relative to the baseline.
    Low,

    /// Moderate overhead.
    Medium,

    /// High overhead.
    High,

    /// Potentially very high overhead requiring explicit policy evaluation.
    VeryHigh,

    /// Cannot be estimated before target-specific analysis.
    Unknown,
}

impl OverheadLevel {
    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::VeryHigh => "very_high",
            Self::Unknown => "unknown",
        }
    }
}

/// One expected overhead dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExpectedOverhead {
    /// Resource dimension.
    pub dimension: OverheadDimension,

    /// Qualitative level.
    pub level: OverheadLevel,
}

impl ExpectedOverhead {
    /// Creates an overhead descriptor.
    pub const fn new(dimension: OverheadDimension, level: OverheadLevel) -> Self {
        Self { dimension, level }
    }
}

// =============================================================================
// Applicability
// =============================================================================

/// Result of asking a mitigation strategy whether it can be considered.
///
/// This is intentionally not a boolean: a strategy may be applicable but
/// require additional target-specific validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Applicability {
    /// Strategy is known to be applicable under the supplied abstract context.
    Applicable,

    /// Strategy cannot apply to the supplied abstract context.
    NotApplicable,

    /// Strategy may apply, but target-specific capability validation is needed.
    RequiresCapabilityValidation,

    /// Strategy may apply, but policy validation is required first.
    RequiresPolicyValidation,

    /// Strategy cannot be evaluated because required information is unavailable.
    InsufficientInformation,
}

impl Applicability {
    /// Returns whether the strategy is a candidate for further evaluation.
    #[must_use]
    pub const fn is_candidate(self) -> bool {
        matches!(
            self,
            Self::Applicable
                | Self::RequiresCapabilityValidation
                | Self::RequiresPolicyValidation
        )
    }

    /// Stable machine-readable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Applicable => "applicable",
            Self::NotApplicable => "not_applicable",
            Self::RequiresCapabilityValidation => "requires_capability_validation",
            Self::RequiresPolicyValidation => "requires_policy_validation",
            Self::InsufficientInformation => "insufficient_information",
        }
    }
}

// =============================================================================
// Mitigation scope
// =============================================================================

/// Scope to which a strategy may be applied.
///
/// Physical placement remains owned by routing/hardware. Logical identity is
/// represented using the canonical Zamani IR `QubitId` rather than defining a
/// second identifier type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MitigationScope {
    /// Entire logical program.
    Program,

    /// Current execution.
    Execution,

    /// Explicit logical qubits.
    LogicalQubits(Arc<[crate::quantum::ir::qubit::QubitId]>),

    /// A provider-neutral externally identified resource region.
    ResourceRegion(StrategyResourceId),
}

impl MitigationScope {
    /// Creates a whole-program scope.
    pub const fn program() -> Self {
        Self::Program
    }

    /// Creates an execution scope.
    pub const fn execution() -> Self {
        Self::Execution
    }

    /// Creates a logical-qubit scope using the canonical IR qubit identity.
    pub fn logical_qubits<I>(qubits: I) -> Self
    where
        I: IntoIterator<Item = crate::quantum::ir::qubit::QubitId>,
    {
        Self::LogicalQubits(qubits.into_iter().collect())
    }

    /// Creates an opaque resource-region scope.
    pub fn resource_region(
        id: impl Into<String>,
    ) -> Result<Self, StrategyContractError> {
        Ok(Self::ResourceRegion(StrategyResourceId::new(id)?))
    }

    /// Returns the explicitly scoped logical qubits.
    #[must_use]
    pub fn logical_qubits_ref(
        &self,
    ) -> Option<&[crate::quantum::ir::qubit::QubitId]> {
        match self {
            Self::LogicalQubits(qubits) => Some(qubits.as_ref()),
            _ => None,
        }
    }
}

// =============================================================================
// Opaque resource identity
// =============================================================================

/// Provider-neutral resource identity used only where a strategy must refer to
/// an externally selected region.
///
/// The strategy layer does not interpret this identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StrategyResourceId(String);

impl StrategyResourceId {
    /// Creates an opaque resource identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, StrategyContractError> {
        let value = value.into();

        if value.is_empty() {
            return Err(StrategyContractError::EmptyResourceId);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for StrategyResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Abstract evaluation context
// =============================================================================

/// Immutable abstract information supplied to a strategy when evaluating
/// applicability.
///
/// This deliberately does not contain a concrete hardware/backend object.
/// `selection.rs` can construct this from the authoritative hardware,
/// policy, ZQN and execution models.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrategyContext {
    /// Requested mitigation scope.
    pub scope: MitigationScope,

    /// Whether measurement results are available.
    pub measurement_results_available: bool,

    /// Whether repeated execution is permitted.
    pub repeated_execution_allowed: bool,

    /// Whether noise scaling is available.
    pub noise_scaling_available: bool,

    /// Whether parameter variation is available.
    pub parameter_variation_available: bool,

    /// Whether randomized compilation is available.
    pub randomized_compilation_available: bool,

    /// Whether randomness provenance is available.
    pub randomness_provenance_available: bool,

    /// Whether schedule modification is available.
    pub schedule_control_available: bool,

    /// Whether timing information is available.
    pub timing_information_available: bool,

    /// Whether pulse/control abstraction is available.
    pub pulse_control_available: bool,

    /// Whether statistical analysis is available.
    pub statistical_analysis_available: bool,

    /// Whether provenance recording is available.
    pub provenance_available: bool,

    /// Whether cross-execution correlation is available.
    pub cross_execution_correlation_available: bool,

    /// Whether explicit policy authorization has been granted.
    pub policy_authorized: bool,
}

impl Default for StrategyContext {
    fn default() -> Self {
        Self {
            scope: MitigationScope::Program,
            measurement_results_available: false,
            repeated_execution_allowed: false,
            noise_scaling_available: false,
            parameter_variation_available: false,
            randomized_compilation_available: false,
            randomness_provenance_available: false,
            schedule_control_available: false,
            timing_information_available: false,
            pulse_control_available: false,
            statistical_analysis_available: false,
            provenance_available: false,
            cross_execution_correlation_available: false,
            policy_authorized: false,
        }
    }
}

// =============================================================================
// Strategy descriptor
// =============================================================================

/// Immutable metadata describing a mitigation strategy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrategyDescriptor {
    /// Stable strategy identity.
    pub id: StrategyId,

    /// Strategy semantic version.
    pub version: StrategyVersion,

    /// Strategy family.
    pub family: StrategyFamily,

    /// Primary operating phase.
    pub phase: StrategyPhase,

    /// Human-readable description.
    pub description: Arc<str>,

    /// Required capabilities.
    pub requirements: Arc<[StrategyRequirement]>,

    /// Expected resource overhead.
    pub expected_overhead: Arc<[ExpectedOverhead]>,

    /// Whether deterministic execution is supported.
    pub deterministic: bool,

    /// Whether explicit policy authorization is required.
    pub requires_explicit_authorization: bool,
}

impl StrategyDescriptor {
    /// Returns whether a particular requirement is declared.
    #[must_use]
    pub fn requires(&self, requirement: StrategyRequirement) -> bool {
        self.requirements.iter().any(|item| *item == requirement)
    }

    /// Returns whether a particular overhead dimension is declared.
    #[must_use]
    pub fn overhead_for(
        &self,
        dimension: OverheadDimension,
    ) -> Option<OverheadLevel> {
        self.expected_overhead
            .iter()
            .find(|item| item.dimension == dimension)
            .map(|item| item.level)
    }
}

// =============================================================================
// Evaluation result
// =============================================================================

/// Immutable result of strategy applicability evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrategyEvaluation {
    /// Strategy identity.
    pub strategy_id: StrategyId,

    /// Strategy version.
    pub strategy_version: StrategyVersion,

    /// Applicability classification.
    pub applicability: Applicability,

    /// Requirements that are currently unavailable.
    pub missing_requirements: Arc<[StrategyRequirement]>,
}

impl StrategyEvaluation {
    /// Creates an evaluation result.
    pub fn new(
        descriptor: &StrategyDescriptor,
        applicability: Applicability,
        missing_requirements: Vec<StrategyRequirement>,
    ) -> Self {
        Self {
            strategy_id: descriptor.id.clone(),
            strategy_version: descriptor.version,
            applicability,
            missing_requirements: missing_requirements.into(),
        }
    }

    /// Returns whether this strategy can continue through selection.
    #[must_use]
    pub const fn is_candidate(&self) -> bool {
        self.applicability.is_candidate()
    }
}

// =============================================================================
// Strategy trait
// =============================================================================

/// Stable contract implemented by every mitigation strategy.
///
/// The trait is deliberately free of backend-specific execution objects.
///
/// A concrete strategy should:
///
/// 1. expose a stable descriptor;
/// 2. declare its requirements;
/// 3. report expected overhead;
/// 4. determine abstract applicability;
/// 5. never execute hardware itself through this contract.
///
/// Actual execution belongs to `mitigation/executor.rs`.
pub trait MitigationStrategy: Send + Sync {
    /// Returns immutable strategy metadata.
    fn descriptor(&self) -> &StrategyDescriptor;

    /// Evaluates whether the strategy can be considered for the supplied
    /// abstract context.
    ///
    /// This method must be:
    ///
    /// - deterministic for identical inputs;
    /// - side-effect free;
    /// - free of I/O;
    /// - free of hidden global state.
    fn evaluate(&self, context: &StrategyContext) -> StrategyEvaluation {
        let descriptor = self.descriptor();

        if descriptor.requires_explicit_authorization
            && !context.policy_authorized
        {
            return StrategyEvaluation::new(
                descriptor,
                Applicability::RequiresPolicyValidation,
                vec![StrategyRequirement::ExplicitPolicyAuthorization],
            );
        }

        let mut missing = Vec::new();

        for requirement in descriptor.requirements.iter() {
            if !requirement_satisfied(*requirement, context) {
                missing.push(*requirement);
            }
        }

        if !missing.is_empty() {
            return StrategyEvaluation::new(
                descriptor,
                Applicability::RequiresCapabilityValidation,
                missing,
            );
        }

        StrategyEvaluation::new(descriptor, Applicability::Applicable, Vec::new())
    }
}

// =============================================================================
// Requirement evaluation
// =============================================================================

/// Evaluates a declared requirement against an abstract strategy context.
#[must_use]
pub fn requirement_satisfied(
    requirement: StrategyRequirement,
    context: &StrategyContext,
) -> bool {
    match requirement {
        StrategyRequirement::ClassicalPostProcessing => {
            context.statistical_analysis_available
        }

        StrategyRequirement::MeasurementResults => {
            context.measurement_results_available
        }

        StrategyRequirement::RepeatedExecution => {
            context.repeated_execution_allowed
        }

        StrategyRequirement::NoiseScaling => {
            context.noise_scaling_available
        }

        StrategyRequirement::ParameterVariation => {
            context.parameter_variation_available
        }

        StrategyRequirement::RandomizedCompilation => {
            context.randomized_compilation_available
        }

        StrategyRequirement::RandomnessProvenance => {
            context.randomness_provenance_available
        }

        StrategyRequirement::ScheduleControl => {
            context.schedule_control_available
        }

        StrategyRequirement::TimingInformation => {
            context.timing_information_available
        }

        StrategyRequirement::PulseControl => {
            context.pulse_control_available
        }

        StrategyRequirement::StatisticalAnalysis => {
            context.statistical_analysis_available
        }

        StrategyRequirement::Provenance => {
            context.provenance_available
        }

        StrategyRequirement::CrossExecutionCorrelation => {
            context.cross_execution_correlation_available
        }

        StrategyRequirement::VariantExecution => {
            context.repeated_execution_allowed
        }

        StrategyRequirement::ScopedExecution => {
            true
        }

        StrategyRequirement::ExplicitPolicyAuthorization => {
            context.policy_authorized
        }
    }
}

// =============================================================================
// Strategy collection
// =============================================================================

/// Immutable collection of mitigation strategy implementations.
///
/// This is useful for `selection.rs` and registry integration without requiring
/// a global mutable registry.
#[derive(Default)]
pub struct StrategySet {
    strategies: Vec<Arc<dyn MitigationStrategy>>,
}

impl StrategySet {
    /// Creates an empty strategy set.
    pub const fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }

    /// Creates a strategy set with pre-existing implementations.
    pub fn from_strategies(
        strategies: impl IntoIterator<Item = Arc<dyn MitigationStrategy>>,
    ) -> Self {
        Self {
            strategies: strategies.into_iter().collect(),
        }
    }

    /// Adds a strategy.
    ///
    /// Duplicate strategy IDs are rejected.
    pub fn insert(
        &mut self,
        strategy: Arc<dyn MitigationStrategy>,
    ) -> Result<(), StrategyContractError> {
        let id = strategy.descriptor().id.as_str();

        if self
            .strategies
            .iter()
            .any(|existing| existing.descriptor().id.as_str() == id)
        {
            return Err(StrategyContractError::DuplicateStrategyId);
        }

        self.strategies.push(strategy);
        Ok(())
    }

    /// Returns the number of registered strategies.
    #[must_use]
    pub fn len(&self) -> usize {
        self.strategies.len()
    }

    /// Returns whether no strategies are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.strategies.is_empty()
    }

    /// Iterates over strategies in registration order.
    pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn MitigationStrategy>> {
        self.strategies.iter()
    }

    /// Finds a strategy by stable ID.
    #[must_use]
    pub fn get(&self, id: &StrategyId) -> Option<&Arc<dyn MitigationStrategy>> {
        self.strategies
            .iter()
            .find(|strategy| strategy.descriptor().id == *id)
    }

    /// Evaluates all strategies without executing any of them.
    pub fn evaluate_all(
        &self,
        context: &StrategyContext,
    ) -> Vec<StrategyEvaluation> {
        self.strategies
            .iter()
            .map(|strategy| strategy.evaluate(context))
            .collect()
    }
}

// =============================================================================
// Contract errors
// =============================================================================

/// Errors concerning the structural mitigation strategy contract.
///
/// Runtime mitigation failures belong to the central resilience error model
/// (`quantum::resilience::errors`).
///
/// This small local error type exists only for construction/registration of
/// strategy-contract values and therefore does not duplicate runtime error
/// taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StrategyContractError {
    /// Strategy identifier was empty.
    EmptyStrategyId,

    /// Strategy identifier contained forbidden whitespace.
    InvalidStrategyId,

    /// Resource identifier was empty.
    EmptyResourceId,

    /// A strategy with the same stable identifier already exists.
    DuplicateStrategyId,
}

impl fmt::Display for StrategyContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyStrategyId => {
                formatter.write_str("mitigation strategy identifier is empty")
            }
            Self::InvalidStrategyId => {
                formatter.write_str(
                    "mitigation strategy identifier contains whitespace",
                )
            }
            Self::EmptyResourceId => {
                formatter.write_str("mitigation resource identifier is empty")
            }
            Self::DuplicateStrategyId => {
                formatter.write_str(
                    "mitigation strategy identifier is already registered",
                )
            }
        }
    }
}

impl std::error::Error for StrategyContractError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStrategy {
        descriptor: StrategyDescriptor,
    }

    impl TestStrategy {
        fn new() -> Self {
            Self {
                descriptor: StrategyDescriptor {
                    id: StrategyId::new("test.strategy")
                        .expect("static identifier must be valid"),
                    version: StrategyVersion::new(1, 0, 0),
                    family: StrategyFamily::Custom,
                    phase: StrategyPhase::PostExecution,
                    description: Arc::from("Test mitigation strategy"),
                    requirements: Arc::from([
                        StrategyRequirement::MeasurementResults,
                        StrategyRequirement::StatisticalAnalysis,
                        StrategyRequirement::Provenance,
                    ]),
                    expected_overhead: Arc::from([
                        ExpectedOverhead::new(
                            OverheadDimension::ClassicalComputation,
                            OverheadLevel::Low,
                        ),
                        ExpectedOverhead::new(
                            OverheadDimension::Executions,
                            OverheadLevel::None,
                        ),
                    ]),
                    deterministic: true,
                    requires_explicit_authorization: false,
                },
            }
        }
    }

    impl MitigationStrategy for TestStrategy {
        fn descriptor(&self) -> &StrategyDescriptor {
            &self.descriptor
        }
    }

    #[test]
    fn strategy_id_rejects_empty_values() {
        assert_eq!(
            StrategyId::new(""),
            Err(StrategyContractError::EmptyStrategyId)
        );
    }

    #[test]
    fn strategy_id_rejects_whitespace() {
        assert_eq!(
            StrategyId::new("invalid strategy"),
            Err(StrategyContractError::InvalidStrategyId)
        );
    }

    #[test]
    fn strategy_id_is_stable() {
        let id = StrategyId::new("readout").expect("identifier must be valid");

        assert_eq!(id.as_str(), "readout");
    }

    #[test]
    fn version_is_structurally_comparable() {
        let first = StrategyVersion::new(1, 0, 0);
        let second = StrategyVersion::new(1, 1, 0);

        assert!(first < second);
        assert_eq!(first.to_string(), "1.0.0");
    }

    #[test]
    fn descriptor_reports_requirements() {
        let strategy = TestStrategy::new();

        assert!(
            strategy
                .descriptor()
                .requires(StrategyRequirement::MeasurementResults)
        );

        assert_eq!(
            strategy
                .descriptor()
                .overhead_for(OverheadDimension::Executions),
            Some(OverheadLevel::None)
        );
    }

    #[test]
    fn evaluation_reports_missing_capabilities() {
        let strategy = TestStrategy::new();
        let context = StrategyContext::default();

        let evaluation = strategy.evaluate(&context);

        assert_eq!(
            evaluation.applicability,
            Applicability::RequiresCapabilityValidation
        );

        assert!(
            evaluation
                .missing_requirements
                .contains(&StrategyRequirement::MeasurementResults)
        );
    }

    #[test]
    fn evaluation_is_applicable_when_requirements_exist() {
        let strategy = TestStrategy::new();

        let context = StrategyContext {
            measurement_results_available: true,
            statistical_analysis_available: true,
            provenance_available: true,
            ..StrategyContext::default()
        };

        let evaluation = strategy.evaluate(&context);

        assert_eq!(evaluation.applicability, Applicability::Applicable);
        assert!(evaluation.missing_requirements.is_empty());
        assert!(evaluation.is_candidate());
    }

    #[test]
    fn authorization_is_a_real_requirement() {
        let mut descriptor = TestStrategy::new().descriptor.clone();
        descriptor.requires_explicit_authorization = true;

        let strategy = TestStrategy { descriptor };

        let context = StrategyContext {
            measurement_results_available: true,
            statistical_analysis_available: true,
            provenance_available: true,
            policy_authorized: false,
            ..StrategyContext::default()
        };

        let evaluation = strategy.evaluate(&context);

        assert_eq!(
            evaluation.applicability,
            Applicability::RequiresPolicyValidation
        );
    }

    #[test]
    fn strategy_set_rejects_duplicate_ids() {
        let first: Arc<dyn MitigationStrategy> = Arc::new(TestStrategy::new());
        let second: Arc<dyn MitigationStrategy> = Arc::new(TestStrategy::new());

        let mut set = StrategySet::new();

        assert!(set.insert(first).is_ok());
        assert_eq!(
            set.insert(second),
            Err(StrategyContractError::DuplicateStrategyId)
        );
    }

    #[test]
    fn strategy_set_evaluates_without_execution() {
        let strategy: Arc<dyn MitigationStrategy> = Arc::new(TestStrategy::new());

        let set = StrategySet::from_strategies([strategy]);

        let context = StrategyContext {
            measurement_results_available: true,
            statistical_analysis_available: true,
            provenance_available: true,
            ..StrategyContext::default()
        };

        let results = set.evaluate_all(&context);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].applicability, Applicability::Applicable);
    }

    #[test]
    fn logical_scope_uses_canonical_qubit_identity() {
        let qubits: Vec<crate::quantum::ir::qubit::QubitId> = Vec::new();

        let scope = MitigationScope::logical_qubits(qubits);

        assert!(scope.logical_qubits_ref().is_some());
    }
}