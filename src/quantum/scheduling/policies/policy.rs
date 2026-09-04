//! Zamani Quantum Scheduling — Policy Contracts
//!
//! This module defines the provider-neutral policy layer for
//! `quantum::scheduling`.
//!
//! # Responsibility
//!
//! This file answers:
//!
//! > "What scheduling behaviour and optimization intent should the scheduler
//! > use for this scheduling request?"
//!
//! It does NOT answer:
//!
//! - which physical qubit a logical qubit maps to;
//! - when a particular operation executes;
//! - how a dependency graph is constructed;
//! - how hardware is discovered;
//! - how calibration is acquired;
//! - how pulses are generated;
//! - how QEC is implemented;
//! - how a provider API is called;
//! - how a schedule is executed.
//!
//! Those responsibilities belong to their owning subsystems.
//!
//! # Architectural position
//!
//! ```text
//! Zamani program
//!       |
//!       v
//! quantum::ir
//!       |
//!       v
//! optimization
//!       |
//!       v
//! routing
//!       |
//!       v
//! quantum::scheduling
//!       |
//!       +-------------------------+
//!       |                         |
//!       v                         v
//! dependency/resource/timing      policy
//! analysis                        |
//!       |                         |
//!       +-------------+-----------+
//!                     |
//!                     v
//!                 planner
//!                     |
//!                     v
//!                 schedule
//! ```
//!
//! The policy is therefore an input to scheduling algorithms, not an
//! implementation of an algorithm itself.
//!
//! # Design principles
//!
//! This module is designed for:
//!
//! - Rust 1.97 / 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no `unsafe`;
//! - no global mutable state;
//! - no provider-specific assumptions;
//! - no fixed machine size;
//! - no fixed qubit count;
//! - no fixed resource count;
//! - no fixed gate set;
//! - no fixed topology;
//! - no floating-point scheduling semantics;
//! - deterministic behaviour when requested;
//! - explicit configuration;
//! - extensible policies;
//! - forward-compatible serialization;
//! - bounded arithmetic;
//! - composability;
//! - thread-safe immutable policy values.
//!
//! # Important boundary
//!
//! The canonical qubit identity remains:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! This module deliberately does not redefine `QubitId`, because a scheduling
//! policy is a machine-independent scheduling strategy and should not own
//! qubit identity.
//!
//! Physical resources are supplied by the scheduler/resource layer.
//!
//! # Policy semantics
//!
//! A policy describes scheduling intent. It must never alter quantum
//! semantics.
//!
//! Given the same:
//!
//! - executable quantum workload;
//! - target description;
//! - timing model;
//! - resource model;
//! - constraints;
//! - policy;
//!
//! a deterministic scheduler must produce the same result.
//!
//! # Relationship with existing Zamani policy types
//!
//! Zamani currently contains policy concepts in other subsystems, including
//! `quantum::hardware::scheduling` and `quantum::optimization`.
//!
//! This module intentionally does not reuse their enum names because those
//! policies govern different abstraction layers.
//!
//! Conversion belongs in integration adapters, not in this foundational
//! contract.
//!
//! # Integration contract
//!
//! `policy.rs` is intentionally independent from the other scheduler files.
//!
//! Future files may consume these contracts without modifying this file:
//!
//! - `policies/asap.rs`
//! - `policies/alap.rs`
//! - `policies/priority.rs`
//! - `policies/resource_aware.rs`
//! - `policies/hybrid.rs`
//! - `planners/planner.rs`
//! - `planners/list.rs`
//! - `planners/critical_path.rs`
//! - `planners/resource_constrained.rs`
//! - `algorithms/*`
//! - `config.rs`
//! - `context.rs`
//! - `optimization/*`
//! - `verification/*`
//!
//! None of those modules should need to change the semantic meaning of the
//! types in this file.
//!
//! # Compatibility
//!
//! No dependency on `serde`, external crates, hardware providers, or runtime
//! APIs is required. Serialization adapters can provide serde implementations
//! at the boundary without forcing serialization dependencies into this core
//! contract.
//!
//! # Safety
//!
//! This file contains no unsafe code.
//!
//! `#![deny(unsafe_code)]` is intentionally enabled at module scope.
//!
//! # Scalability
//!
//! A policy must describe intent, not capacity.
//!
//! It must therefore never contain fields such as:
//!
//! - `number_of_qubits`;
//! - `number_of_channels`;
//! - `maximum_qubits`;
//! - `maximum_gates`;
//! - `maximum_depth`;
//! - `hardware_size`.
//!
//! Capacity belongs to the target/resource model.
//!
//! Consequently the same policy can be applied to:
//!
//! - a single-qubit target;
//! - a small QPU;
//! - a large QPU;
//! - a modular quantum computer;
//! - a distributed quantum system;
//! - a future architecture not yet known to Zamani.
//!
//! # Versioning
//!
//! `POLICY_SCHEMA_VERSION` versions the semantic contract of this module.
//! It must only be incremented when serialized or externally observable
//! policy semantics change incompatibly.
//!
//! # Examples
//!
//! ```
//! use crate::quantum::scheduling::policies::policy::{
//!     SchedulingPolicy,
//!     SchedulingPolicyKind,
//! };
//!
//! let policy = SchedulingPolicy::new(SchedulingPolicyKind::AsSoonAsPossible);
//!
//! assert_eq!(
//!     policy.kind(),
//!     SchedulingPolicyKind::AsSoonAsPossible
//! );
//! ```
//!
//! ```
//! use crate::quantum::scheduling::policies::policy::{
//!     SchedulingPolicy,
//!     SchedulingPolicyKind,
//! };
//!
//! let policy = SchedulingPolicy::builder()
//!     .kind(SchedulingPolicyKind::ResourceAware)
//!     .deterministic(true)
//!     .build();
//!
//! assert!(policy.deterministic());
//! ```

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use core::num::NonZeroU16;

// =============================================================================
// Schema
// =============================================================================

/// Stable identifier for the scheduling-policy schema.
pub const POLICY_SCHEMA_ID: &str = "zamani.quantum.scheduling.policy";

/// Semantic version of the scheduling-policy contract.
///
/// This value is intentionally independent from the crate version.
pub const POLICY_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Policy kind
// =============================================================================

/// High-level scheduling strategy.
///
/// This enum describes *intent*. The actual scheduling algorithm is owned by
/// the corresponding algorithm/planner module.
///
/// A policy implementation MUST NOT assume a particular hardware technology,
/// qubit count, topology, instruction set, or resource count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingPolicyKind {
    /// Schedule operations as early as their dependencies, resources, timing
    /// constraints, and target constraints permit.
    AsSoonAsPossible,

    /// Schedule operations as late as possible while preserving legality and
    /// the requested schedule boundary.
    AsLateAsPossible,

    /// Use deterministic ready-list scheduling with policy-defined priority.
    List,

    /// Prioritize operations according to critical-path urgency.
    CriticalPath,

    /// Treat scarce resources as first-class scheduling constraints and
    /// prioritize resource-feasible execution.
    ResourceAware,

    /// Combine dependency criticality with resource pressure.
    CriticalPathResourceAware,

    /// Adapt strategy according to workload/target characteristics while
    /// preserving deterministic behaviour when requested.
    Adaptive,

    /// Delegate to a registered custom scheduling implementation.
    Custom,
}

impl Default for SchedulingPolicyKind {
    fn default() -> Self {
        Self::AsSoonAsPossible
    }
}

impl SchedulingPolicyKind {
    /// Returns a stable machine-readable name.
    ///
    /// These strings are part of the diagnostics/serialization contract and
    /// therefore must not be casually renamed.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AsSoonAsPossible => "as_soon_as_possible",
            Self::AsLateAsPossible => "as_late_as_possible",
            Self::List => "list",
            Self::CriticalPath => "critical_path",
            Self::ResourceAware => "resource_aware",
            Self::CriticalPathResourceAware => "critical_path_resource_aware",
            Self::Adaptive => "adaptive",
            Self::Custom => "custom",
        }
    }

    /// Returns whether the policy requires backward scheduling information.
    pub const fn requires_backward_analysis(self) -> bool {
        matches!(
            self,
            Self::AsLateAsPossible
                | Self::CriticalPath
                | Self::CriticalPathResourceAware
        )
    }

    /// Returns whether the policy requires resource availability analysis.
    pub const fn requires_resource_analysis(self) -> bool {
        matches!(
            self,
            Self::ResourceAware
                | Self::CriticalPathResourceAware
                | Self::Adaptive
        )
    }

    /// Returns whether the policy is inherently dependency-oriented.
    pub const fn requires_dependency_analysis(self) -> bool {
        true
    }

    /// Returns whether the policy may adapt its strategy to the input.
    pub const fn is_adaptive(self) -> bool {
        matches!(self, Self::Adaptive)
    }

    /// Returns whether this policy represents an external/custom strategy.
    pub const fn is_custom(self) -> bool {
        matches!(self, Self::Custom)
    }
}

impl fmt::Display for SchedulingPolicyKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Optimization objective
// =============================================================================

/// Primary objective used by a scheduling policy.
///
/// Objectives describe what should be improved; they do not contain hardware
/// assumptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SchedulingObjective {
    /// Minimize total schedule duration.
    MinimizeMakespan,

    /// Minimize scheduled circuit depth.
    MinimizeDepth,

    /// Minimize total resource idle time.
    MinimizeIdleTime,

    /// Prefer schedules with lower estimated physical error.
    MaximizeEstimatedFidelity,

    /// Minimize an externally supplied energy/cost estimate.
    MinimizeEnergy,

    /// Preserve the input order whenever legal and otherwise make the
    /// smallest scheduling change necessary.
    PreserveOrder,

    /// Use a caller-supplied composite objective.
    Composite,
}

impl Default for SchedulingObjective {
    fn default() -> Self {
        Self::MinimizeMakespan
    }
}

impl SchedulingObjective {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MinimizeMakespan => "minimize_makespan",
            Self::MinimizeDepth => "minimize_depth",
            Self::MinimizeIdleTime => "minimize_idle_time",
            Self::MaximizeEstimatedFidelity => "maximize_estimated_fidelity",
            Self::MinimizeEnergy => "minimize_energy",
            Self::PreserveOrder => "preserve_order",
            Self::Composite => "composite",
        }
    }

    /// Returns whether the objective needs timing information.
    pub const fn requires_timing(self) -> bool {
        true
    }

    /// Returns whether the objective may need a target/resource model.
    pub const fn requires_target_information(self) -> bool {
        matches!(
            self,
            Self::MinimizeIdleTime
                | Self::MaximizeEstimatedFidelity
                | Self::MinimizeEnergy
                | Self::Composite
        )
    }

    /// Returns whether this objective can be evaluated without modifying
    /// quantum semantics.
    pub const fn is_semantics_preserving(self) -> bool {
        true
    }
}

impl fmt::Display for SchedulingObjective {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Tie-breaking
// =============================================================================

/// Deterministic tie-breaking rule.
///
/// Tie-breaking affects which legal operation is selected when multiple
/// candidates have equivalent primary scheduling scores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TieBreakRule {
    /// Earlier legal start time first.
    EarliestStart,

    /// Higher explicit operation priority first.
    Priority,

    /// Greater critical-path urgency first.
    Criticality,

    /// Stable operation identifier first.
    OperationId,

    /// Preserve source/program order where available.
    SourceOrder,

    /// Prefer the operation using fewer scarce resources.
    ResourceFootprint,

    /// Apply a deterministic lexicographic combination supplied by the
    /// policy's built-in precedence.
    DeterministicDefault,
}

impl Default for TieBreakRule {
    fn default() -> Self {
        Self::DeterministicDefault
    }
}

impl TieBreakRule {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EarliestStart => "earliest_start",
            Self::Priority => "priority",
            Self::Criticality => "criticality",
            Self::OperationId => "operation_id",
            Self::SourceOrder => "source_order",
            Self::ResourceFootprint => "resource_footprint",
            Self::DeterministicDefault => "deterministic_default",
        }
    }
}

impl fmt::Display for TieBreakRule {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Determinism
// =============================================================================

/// Determinism mode for policy evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Determinism {
    /// Produce deterministic choices whenever the input model itself is
    /// deterministic.
    Deterministic,

    /// Permit implementation-level parallelism and nondeterministic
    /// tie-breaking where the selected scheduler supports it.
    ///
    /// This still MUST preserve quantum semantics.
    Permissive,
}

impl Default for Determinism {
    fn default() -> Self {
        Self::Deterministic
    }
}

impl Determinism {
    /// Returns whether deterministic scheduling is required.
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Deterministic)
    }

    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deterministic => "deterministic",
            Self::Permissive => "permissive",
        }
    }
}

impl fmt::Display for Determinism {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Custom policy identifier
// =============================================================================

/// Stable identifier for a custom scheduler implementation.
///
/// A custom policy is intentionally represented by an identifier rather than
/// a function pointer or trait object. This keeps the policy value:
///
/// - immutable;
/// - serializable at the boundary;
/// - thread-safe;
/// - independent of executable code;
/// - suitable for distributed scheduling requests.
///
/// Registry lookup is performed by `plugins::registry` or the appropriate
/// integration layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CustomPolicyId(String);

impl CustomPolicyId {
    /// Creates a custom policy identifier.
    ///
    /// Empty identifiers are rejected.
    pub fn new(value: impl Into<String>) -> Result<Self, PolicyError> {
        let value = value.into();

        if value.is_empty() {
            return Err(PolicyError::EmptyCustomPolicyId);
        }

        if value.len() > POLICY_ID_MAX_BYTES {
            return Err(PolicyError::CustomPolicyIdTooLong {
                length: value.len(),
                maximum: POLICY_ID_MAX_BYTES,
            });
        }

        if !value.is_ascii() {
            return Err(PolicyError::CustomPolicyIdNotAscii);
        }

        Ok(Self(value))
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for CustomPolicyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Maximum serialized custom-policy identifier length.
///
/// This is a protocol-safety bound, not a machine-capacity bound.
pub const POLICY_ID_MAX_BYTES: usize = 256;

// =============================================================================
// Policy capability
// =============================================================================

/// Capability requested by a policy.
///
/// The planner can use this information to determine which analyses are
/// required before invoking an algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PolicyRequirement {
    /// Dependency DAG.
    Dependencies,

    /// Forward timing analysis.
    ForwardTiming,

    /// Backward timing analysis.
    BackwardTiming,

    /// Resource availability model.
    Resources,

    /// Operation priority metadata.
    Priorities,

    /// Critical-path metadata.
    CriticalPath,

    /// Target-specific objective information.
    TargetMetrics,

    /// Dynamic/runtime scheduling information.
    DynamicExecution,

    /// Distributed communication information.
    DistributedResources,
}

impl PolicyRequirement {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dependencies => "dependencies",
            Self::ForwardTiming => "forward_timing",
            Self::BackwardTiming => "backward_timing",
            Self::Resources => "resources",
            Self::Priorities => "priorities",
            Self::CriticalPath => "critical_path",
            Self::TargetMetrics => "target_metrics",
            Self::DynamicExecution => "dynamic_execution",
            Self::DistributedResources => "distributed_resources",
        }
    }
}

// =============================================================================
// Policy specification
// =============================================================================

/// Immutable scheduling-policy specification.
///
/// This is the primary value consumed by the scheduling context/configuration
/// layer.
///
/// It contains no hardware-specific capacity and no operation-specific state.
///
/// # Invariants
///
/// A valid `SchedulingPolicy` satisfies:
///
/// - the policy kind is defined;
/// - the objective is defined;
/// - the tie-break rule is defined;
/// - custom policies have a custom identifier;
/// - non-custom policies do not require one;
/// - the priority weight is representable;
/// - the policy contains no machine-size assumptions.
///
/// The scheduler is responsible for validating whether the requested policy
/// is compatible with a particular target.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchedulingPolicy {
    kind: SchedulingPolicyKind,
    objective: SchedulingObjective,
    tie_break: TieBreakRule,
    determinism: Determinism,
    custom_id: Option<CustomPolicyId>,
    priority_weight: u16,
    criticality_weight: u16,
    resource_weight: u16,
    fidelity_weight: u16,
}

impl SchedulingPolicy {
    /// Creates a policy using the production default settings.
    pub const fn new(kind: SchedulingPolicyKind) -> Self {
        Self {
            kind,
            objective: SchedulingObjective::default(),
            tie_break: TieBreakRule::default(),
            determinism: Determinism::Deterministic,
            custom_id: None,
            priority_weight: DEFAULT_PRIORITY_WEIGHT,
            criticality_weight: DEFAULT_CRITICALITY_WEIGHT,
            resource_weight: DEFAULT_RESOURCE_WEIGHT,
            fidelity_weight: DEFAULT_FIDELITY_WEIGHT,
        }
    }

    /// Returns a policy builder.
    pub const fn builder() -> SchedulingPolicyBuilder {
        SchedulingPolicyBuilder::new()
    }

    /// Returns the selected policy kind.
    pub const fn kind(&self) -> SchedulingPolicyKind {
        self.kind
    }

    /// Returns the primary objective.
    pub const fn objective(&self) -> SchedulingObjective {
        self.objective
    }

    /// Returns the tie-breaking rule.
    pub const fn tie_break(&self) -> TieBreakRule {
        self.tie_break
    }

    /// Returns the determinism mode.
    pub const fn determinism(&self) -> Determinism {
        self.determinism
    }

    /// Returns whether deterministic behaviour is required.
    pub const fn deterministic(&self) -> bool {
        self.determinism.is_required()
    }

    /// Returns the custom policy identifier.
    pub fn custom_id(&self) -> Option<&CustomPolicyId> {
        self.custom_id.as_ref()
    }

    /// Returns the explicit-operation-priority weight.
    pub const fn priority_weight(&self) -> u16 {
        self.priority_weight
    }

    /// Returns the criticality weight.
    pub const fn criticality_weight(&self) -> u16 {
        self.criticality_weight
    }

    /// Returns the resource-pressure weight.
    pub const fn resource_weight(&self) -> u16 {
        self.resource_weight
    }

    /// Returns the fidelity weight.
    pub const fn fidelity_weight(&self) -> u16 {
        self.fidelity_weight
    }

    /// Returns whether this is a custom policy.
    pub const fn is_custom(&self) -> bool {
        self.kind.is_custom()
    }

    /// Returns the requirements implied by the policy.
    ///
    /// The returned list is fixed and deterministic. It contains no target
    /// information.
    pub fn requirements(&self) -> PolicyRequirements {
        let mut requirements = PolicyRequirements::new();

        requirements.insert(PolicyRequirement::Dependencies);
        requirements.insert(PolicyRequirement::ForwardTiming);

        if self.kind.requires_backward_analysis() {
            requirements.insert(PolicyRequirement::BackwardTiming);
        }

        if self.kind.requires_resource_analysis() {
            requirements.insert(PolicyRequirement::Resources);
        }

        if self.priority_weight > 0 {
            requirements.insert(PolicyRequirement::Priorities);
        }

        if self.criticality_weight > 0
            || matches!(
                self.kind,
                SchedulingPolicyKind::CriticalPath
                    | SchedulingPolicyKind::CriticalPathResourceAware
            )
        {
            requirements.insert(PolicyRequirement::CriticalPath);
        }

        if self.objective.requires_target_information() {
            requirements.insert(PolicyRequirement::TargetMetrics);
        }

        if matches!(self.kind, SchedulingPolicyKind::Adaptive) {
            requirements.insert(PolicyRequirement::Resources);
            requirements.insert(PolicyRequirement::TargetMetrics);
        }

        requirements
    }

    /// Validates policy-local invariants.
    ///
    /// Target compatibility is intentionally not checked here because this
    /// file does not own hardware/resource descriptions.
    pub fn validate(&self) -> Result<(), PolicyError> {
        match (self.kind, self.custom_id.is_some()) {
            (SchedulingPolicyKind::Custom, false) => {
                Err(PolicyError::CustomPolicyIdRequired)
            }
            (SchedulingPolicyKind::Custom, true) => Ok(()),
            (_, true) => Err(PolicyError::UnexpectedCustomPolicyId),
            (_, false) => Ok(()),
        }
    }

    /// Returns a stable schema identifier.
    pub const fn schema_id(&self) -> &'static str {
        POLICY_SCHEMA_ID
    }

    /// Returns the semantic schema version.
    pub const fn schema_version(&self) -> u16 {
        POLICY_SCHEMA_VERSION
    }
}

impl Default for SchedulingPolicy {
    fn default() -> Self {
        Self::new(SchedulingPolicyKind::AsSoonAsPossible)
    }
}

// =============================================================================
// Default weights
// =============================================================================

/// Default explicit operation-priority weight.
///
/// Weight values are relative scoring coefficients, not physical units.
pub const DEFAULT_PRIORITY_WEIGHT: u16 = 1;

/// Default criticality weight.
pub const DEFAULT_CRITICALITY_WEIGHT: u16 = 1;

/// Default resource-pressure weight.
pub const DEFAULT_RESOURCE_WEIGHT: u16 = 1;

/// Default fidelity weight.
pub const DEFAULT_FIDELITY_WEIGHT: u16 = 1;

// =============================================================================
// Policy builder
// =============================================================================

/// Builder for [`SchedulingPolicy`].
///
/// The builder exists to keep policy construction explicit and future-proof.
/// It performs no hardware discovery.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchedulingPolicyBuilder {
    kind: SchedulingPolicyKind,
    objective: SchedulingObjective,
    tie_break: TieBreakRule,
    determinism: Determinism,
    custom_id: Option<CustomPolicyId>,
    priority_weight: u16,
    criticality_weight: u16,
    resource_weight: u16,
    fidelity_weight: u16,
}

impl SchedulingPolicyBuilder {
    /// Creates a builder with production defaults.
    pub const fn new() -> Self {
        Self {
            kind: SchedulingPolicyKind::AsSoonAsPossible,
            objective: SchedulingObjective::MinimizeMakespan,
            tie_break: TieBreakRule::DeterministicDefault,
            determinism: Determinism::Deterministic,
            custom_id: None,
            priority_weight: DEFAULT_PRIORITY_WEIGHT,
            criticality_weight: DEFAULT_CRITICALITY_WEIGHT,
            resource_weight: DEFAULT_RESOURCE_WEIGHT,
            fidelity_weight: DEFAULT_FIDELITY_WEIGHT,
        }
    }

    /// Sets the scheduling strategy.
    pub const fn kind(mut self, kind: SchedulingPolicyKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the primary objective.
    pub const fn objective(mut self, objective: SchedulingObjective) -> Self {
        self.objective = objective;
        self
    }

    /// Sets the tie-breaking rule.
    pub const fn tie_break(mut self, tie_break: TieBreakRule) -> Self {
        self.tie_break = tie_break;
        self
    }

    /// Sets determinism.
    pub const fn determinism(mut self, determinism: Determinism) -> Self {
        self.determinism = determinism;
        self
    }

    /// Enables or disables deterministic scheduling.
    pub const fn deterministic(mut self, deterministic: bool) -> Self {
        self.determinism = if deterministic {
            Determinism::Deterministic
        } else {
            Determinism::Permissive
        };
        self
    }

    /// Sets a custom policy identifier.
    ///
    /// This does not load or execute a plugin. Registry resolution belongs to
    /// the plugin layer.
    pub fn custom_id(
        mut self,
        custom_id: impl Into<String>,
    ) -> Result<Self, PolicyError> {
        self.custom_id = Some(CustomPolicyId::new(custom_id)?);
        Ok(self)
    }

    /// Removes a custom policy identifier.
    pub const fn without_custom_id(mut self) -> Self {
        self.custom_id = None;
        self
    }

    /// Sets explicit operation priority weight.
    pub const fn priority_weight(mut self, weight: u16) -> Self {
        self.priority_weight = weight;
        self
    }

    /// Sets critical-path weight.
    pub const fn criticality_weight(mut self, weight: u16) -> Self {
        self.criticality_weight = weight;
        self
    }

    /// Sets resource-pressure weight.
    pub const fn resource_weight(mut self, weight: u16) -> Self {
        self.resource_weight = weight;
        self
    }

    /// Sets estimated-fidelity weight.
    pub const fn fidelity_weight(mut self, weight: u16) -> Self {
        self.fidelity_weight = weight;
        self
    }

    /// Builds and validates the policy.
    pub fn build(self) -> Result<SchedulingPolicy, PolicyError> {
        let policy = SchedulingPolicy {
            kind: self.kind,
            objective: self.objective,
            tie_break: self.tie_break,
            determinism: self.determinism,
            custom_id: self.custom_id,
            priority_weight: self.priority_weight,
            criticality_weight: self.criticality_weight,
            resource_weight: self.resource_weight,
            fidelity_weight: self.fidelity_weight,
        };

        policy.validate()?;
        Ok(policy)
    }
}

impl Default for SchedulingPolicyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Policy requirements
// =============================================================================

/// Set of analyses required by a policy.
///
/// A fixed-size bit representation is used so requirement construction does
/// not allocate memory proportional to the number of scheduler resources or
/// operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PolicyRequirements(u16);

impl PolicyRequirements {
    const DEPENDENCIES: u16 = 1 << 0;
    const FORWARD_TIMING: u16 = 1 << 1;
    const BACKWARD_TIMING: u16 = 1 << 2;
    const RESOURCES: u16 = 1 << 3;
    const PRIORITIES: u16 = 1 << 4;
    const CRITICAL_PATH: u16 = 1 << 5;
    const TARGET_METRICS: u16 = 1 << 6;
    const DYNAMIC_EXECUTION: u16 = 1 << 7;
    const DISTRIBUTED_RESOURCES: u16 = 1 << 8;

    /// Creates an empty requirement set.
    pub const fn new() -> Self {
        Self(0)
    }

    /// Inserts a requirement.
    pub const fn insert(&mut self, requirement: PolicyRequirement) {
        self.0 |= Self::mask(requirement);
    }

    /// Returns whether a requirement is present.
    pub const fn contains(self, requirement: PolicyRequirement) -> bool {
        self.0 & Self::mask(requirement) != 0
    }

    /// Returns whether no requirements are present.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns the raw representation for diagnostics/serialization adapters.
    pub const fn bits(self) -> u16 {
        self.0
    }

    const fn mask(requirement: PolicyRequirement) -> u16 {
        match requirement {
            PolicyRequirement::Dependencies => Self::DEPENDENCIES,
            PolicyRequirement::ForwardTiming => Self::FORWARD_TIMING,
            PolicyRequirement::BackwardTiming => Self::BACKWARD_TIMING,
            PolicyRequirement::Resources => Self::RESOURCES,
            PolicyRequirement::Priorities => Self::PRIORITIES,
            PolicyRequirement::CriticalPath => Self::CRITICAL_PATH,
            PolicyRequirement::TargetMetrics => Self::TARGET_METRICS,
            PolicyRequirement::DynamicExecution => Self::DYNAMIC_EXECUTION,
            PolicyRequirement::DistributedResources => Self::DISTRIBUTED_RESOURCES,
        }
    }
}

// =============================================================================
// Policy scoring
// =============================================================================

/// Normalized scheduler score components.
///
/// These values are intentionally represented as signed integers rather than
/// floating-point numbers. The scheduler/planner determines the units and
/// normalization of the individual components.
///
/// This type only combines already-normalized values.
///
/// A score is not itself a physical measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PolicyScore {
    /// Explicit operation priority contribution.
    priority: i128,

    /// Critical-path contribution.
    criticality: i128,

    /// Resource-pressure contribution.
    resource: i128,

    /// Fidelity contribution.
    fidelity: i128,
}

impl PolicyScore {
    /// Creates a score from normalized components.
    pub const fn new(
        priority: i128,
        criticality: i128,
        resource: i128,
        fidelity: i128,
    ) -> Self {
        Self {
            priority,
            criticality,
            resource,
            fidelity,
        }
    }

    /// Returns explicit-priority contribution.
    pub const fn priority(self) -> i128 {
        self.priority
    }

    /// Returns criticality contribution.
    pub const fn criticality(self) -> i128 {
        self.criticality
    }

    /// Returns resource contribution.
    pub const fn resource(self) -> i128 {
        self.resource
    }

    /// Returns fidelity contribution.
    pub const fn fidelity(self) -> i128 {
        self.fidelity
    }

    /// Computes the weighted score.
    ///
    /// Checked arithmetic is used because scheduler metadata may originate
    /// from very large workloads.
    pub fn weighted(self, policy: &SchedulingPolicy) -> Result<i128, PolicyError> {
        let priority = self
            .priority
            .checked_mul(i128::from(policy.priority_weight()))
            .ok_or(PolicyError::ScoreOverflow)?;

        let criticality = self
            .criticality
            .checked_mul(i128::from(policy.criticality_weight()))
            .ok_or(PolicyError::ScoreOverflow)?;

        let resource = self
            .resource
            .checked_mul(i128::from(policy.resource_weight()))
            .ok_or(PolicyError::ScoreOverflow)?;

        let fidelity = self
            .fidelity
            .checked_mul(i128::from(policy.fidelity_weight()))
            .ok_or(PolicyError::ScoreOverflow)?;

        priority
            .checked_add(criticality)
            .and_then(|value| value.checked_add(resource))
            .and_then(|value| value.checked_add(fidelity))
            .ok_or(PolicyError::ScoreOverflow)
    }
}

// =============================================================================
// Policy errors
// =============================================================================

/// Errors produced by policy construction or policy-local validation.
///
/// These errors deliberately do not contain hardware errors. Hardware
/// compatibility belongs to the scheduling context/validation layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyError {
    /// A custom policy kind requires an identifier.
    CustomPolicyIdRequired,

    /// A custom identifier was supplied to a built-in policy.
    UnexpectedCustomPolicyId,

    /// Custom identifier is empty.
    EmptyCustomPolicyId,

    /// Custom identifier exceeds the protocol safety bound.
    CustomPolicyIdTooLong {
        /// Actual byte length.
        length: usize,

        /// Maximum permitted byte length.
        maximum: usize,
    },

    /// Custom identifiers are restricted to stable ASCII identifiers.
    CustomPolicyIdNotAscii,

    /// Weighted score arithmetic overflowed.
    ScoreOverflow,
}

impl fmt::Display for PolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CustomPolicyIdRequired => {
                formatter.write_str("custom scheduling policy requires a custom policy identifier")
            }

            Self::UnexpectedCustomPolicyId => {
                formatter.write_str(
                    "a custom scheduling policy identifier was supplied for a built-in policy",
                )
            }

            Self::EmptyCustomPolicyId => {
                formatter.write_str("custom scheduling policy identifier must not be empty")
            }

            Self::CustomPolicyIdTooLong { length, maximum } => {
                write!(
                    formatter,
                    "custom scheduling policy identifier is too long: {} bytes; maximum is {}",
                    length, maximum
                )
            }

            Self::CustomPolicyIdNotAscii => {
                formatter.write_str(
                    "custom scheduling policy identifier must contain only ASCII characters",
                )
            }

            Self::ScoreOverflow => {
                formatter.write_str("scheduling policy score arithmetic overflowed")
            }
        }
    }
}

impl std::error::Error for PolicyError {}

// =============================================================================
// Policy contract
// =============================================================================

/// Behavioural contract implemented by scheduling-policy adapters.
///
/// This trait deliberately contains only policy semantics. It does not
/// receive mutable scheduler state and cannot directly mutate a schedule.
///
/// Concrete algorithms such as ASAP, ALAP, list scheduling, critical-path
/// scheduling, and resource-constrained scheduling should consume the
/// immutable [`SchedulingPolicy`] value and implement their own execution
/// logic.
///
/// This trait exists primarily for integration layers that need to inspect
/// policy behaviour without coupling to a concrete algorithm.
pub trait SchedulingPolicyContract {
    /// Returns the selected scheduling policy.
    fn kind(&self) -> SchedulingPolicyKind;

    /// Returns the optimization objective.
    fn objective(&self) -> SchedulingObjective;

    /// Returns tie-breaking behaviour.
    fn tie_break(&self) -> TieBreakRule;

    /// Returns determinism mode.
    fn determinism(&self) -> Determinism;

    /// Returns policy requirements.
    fn requirements(&self) -> PolicyRequirements;

    /// Validates policy-local invariants.
    fn validate(&self) -> Result<(), PolicyError>;
}

impl SchedulingPolicyContract for SchedulingPolicy {
    fn kind(&self) -> SchedulingPolicyKind {
        self.kind()
    }

    fn objective(&self) -> SchedulingObjective {
        self.objective()
    }

    fn tie_break(&self) -> TieBreakRule {
        self.tie_break()
    }

    fn determinism(&self) -> Determinism {
        self.determinism()
    }

    fn requirements(&self) -> PolicyRequirements {
        self.requirements()
    }

    fn validate(&self) -> Result<(), PolicyError> {
        self.validate()
    }
}

// =============================================================================
// Policy identity
// =============================================================================

/// Stable policy identity.
///
/// This is useful to diagnostics, provenance, benchmarking, and distributed
/// scheduling without requiring executable scheduler objects to be serialized.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PolicyIdentity {
    kind: SchedulingPolicyKind,
    custom_id: Option<CustomPolicyId>,
}

impl PolicyIdentity {
    /// Creates an identity from a policy.
    pub fn from_policy(policy: &SchedulingPolicy) -> Self {
        Self {
            kind: policy.kind(),
            custom_id: policy.custom_id().cloned(),
        }
    }

    /// Returns the policy kind.
    pub const fn kind(&self) -> SchedulingPolicyKind {
        self.kind
    }

    /// Returns the custom identifier, if present.
    pub fn custom_id(&self) -> Option<&CustomPolicyId> {
        self.custom_id.as_ref()
    }
}

impl fmt::Display for PolicyIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.custom_id {
            Some(custom_id) => write!(formatter, "{}:{}", self.kind, custom_id),
            None => formatter.write_str(self.kind.as_str()),
        }
    }
}

// =============================================================================
// Policy compatibility
// =============================================================================

/// Describes requirements that must be satisfied by the scheduler context
/// before a policy can be executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PolicyCompatibility {
    /// Whether dependency information is required.
    pub dependencies: bool,

    /// Whether forward timing is required.
    pub forward_timing: bool,

    /// Whether backward timing is required.
    pub backward_timing: bool,

    /// Whether resource information is required.
    pub resources: bool,

    /// Whether target metrics are required.
    pub target_metrics: bool,

    /// Whether dynamic execution information is required.
    pub dynamic_execution: bool,

    /// Whether distributed resource information is required.
    pub distributed_resources: bool,
}

impl PolicyCompatibility {
    /// Creates compatibility requirements from a policy.
    pub fn from_policy(policy: &SchedulingPolicy) -> Self {
        let requirements = policy.requirements();

        Self {
            dependencies: requirements.contains(PolicyRequirement::Dependencies),
            forward_timing: requirements.contains(PolicyRequirement::ForwardTiming),
            backward_timing: requirements.contains(PolicyRequirement::BackwardTiming),
            resources: requirements.contains(PolicyRequirement::Resources),
            target_metrics: requirements.contains(PolicyRequirement::TargetMetrics),
            dynamic_execution: requirements
                .contains(PolicyRequirement::DynamicExecution),
            distributed_resources: requirements
                .contains(PolicyRequirement::DistributedResources),
        }
    }
}

// =============================================================================
// Policy presets
// =============================================================================

/// Production-safe default policy.
///
/// This is a semantic constructor, not a machine-specific configuration.
pub const fn default_policy() -> SchedulingPolicy {
    SchedulingPolicy::new(SchedulingPolicyKind::AsSoonAsPossible)
}

/// Creates a policy optimized for critical-path scheduling.
pub const fn critical_path_policy() -> SchedulingPolicy {
    SchedulingPolicy {
        kind: SchedulingPolicyKind::CriticalPath,
        objective: SchedulingObjective::MinimizeMakespan,
        tie_break: TieBreakRule::Criticality,
        determinism: Determinism::Deterministic,
        custom_id: None,
        priority_weight: DEFAULT_PRIORITY_WEIGHT,
        criticality_weight: DEFAULT_CRITICALITY_WEIGHT,
        resource_weight: DEFAULT_RESOURCE_WEIGHT,
        fidelity_weight: DEFAULT_FIDELITY_WEIGHT,
    }
}

/// Creates a policy optimized for resource pressure.
pub const fn resource_aware_policy() -> SchedulingPolicy {
    SchedulingPolicy {
        kind: SchedulingPolicyKind::ResourceAware,
        objective: SchedulingObjective::MinimizeMakespan,
        tie_break: TieBreakRule::ResourceFootprint,
        determinism: Determinism::Deterministic,
        custom_id: None,
        priority_weight: DEFAULT_PRIORITY_WEIGHT,
        criticality_weight: DEFAULT_CRITICALITY_WEIGHT,
        resource_weight: DEFAULT_RESOURCE_WEIGHT,
        fidelity_weight: DEFAULT_FIDELITY_WEIGHT,
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a policy without requiring a scheduler context.
pub fn validate_policy(policy: &SchedulingPolicy) -> Result<(), PolicyError> {
    policy.validate()
}

/// Returns the canonical default policy.
pub fn production_default() -> SchedulingPolicy {
    default_policy()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_is_asap() {
        let policy = SchedulingPolicy::default();

        assert_eq!(
            policy.kind(),
            SchedulingPolicyKind::AsSoonAsPossible
        );
        assert_eq!(
            policy.objective(),
            SchedulingObjective::MinimizeMakespan
        );
        assert!(policy.deterministic());
    }

    #[test]
    fn builder_produces_valid_policy() {
        let policy = SchedulingPolicy::builder()
            .kind(SchedulingPolicyKind::ResourceAware)
            .objective(SchedulingObjective::MinimizeIdleTime)
            .tie_break(TieBreakRule::ResourceFootprint)
            .deterministic(true)
            .priority_weight(2)
            .criticality_weight(4)
            .resource_weight(8)
            .fidelity_weight(16)
            .build()
            .expect("policy must be valid");

        assert_eq!(
            policy.kind(),
            SchedulingPolicyKind::ResourceAware
        );
        assert_eq!(
            policy.objective(),
            SchedulingObjective::MinimizeIdleTime
        );
        assert_eq!(policy.priority_weight(), 2);
        assert_eq!(policy.criticality_weight(), 4);
        assert_eq!(policy.resource_weight(), 8);
        assert_eq!(policy.fidelity_weight(), 16);
    }

    #[test]
    fn custom_policy_requires_identifier() {
        let policy = SchedulingPolicy::builder()
            .kind(SchedulingPolicyKind::Custom)
            .build();

        assert_eq!(
            policy,
            Err(PolicyError::CustomPolicyIdRequired)
        );
    }

    #[test]
    fn custom_policy_accepts_identifier() {
        let policy = SchedulingPolicy::builder()
            .kind(SchedulingPolicyKind::Custom)
            .custom_id("zamani.experimental.scheduler")
            .expect("identifier must be valid")
            .build()
            .expect("custom policy must be valid");

        assert!(policy.is_custom());
        assert_eq!(
            policy
                .custom_id()
                .expect("custom id must exist")
                .as_str(),
            "zamani.experimental.scheduler"
        );
    }

    #[test]
    fn built_in_policy_rejects_custom_identifier() {
        let policy = SchedulingPolicy::builder()
            .kind(SchedulingPolicyKind::AsSoonAsPossible)
            .custom_id("not-allowed")
            .expect("identifier itself is valid")
            .build();

        assert_eq!(
            policy,
            Err(PolicyError::UnexpectedCustomPolicyId)
        );
    }

    #[test]
    fn empty_custom_identifier_is_rejected() {
        let result = CustomPolicyId::new("");

        assert_eq!(result, Err(PolicyError::EmptyCustomPolicyId));
    }

    #[test]
    fn non_ascii_custom_identifier_is_rejected() {
        let result = CustomPolicyId::new("zamani.é");

        assert_eq!(result, Err(PolicyError::CustomPolicyIdNotAscii));
    }

    #[test]
    fn requirements_for_asap_are_minimal() {
        let policy = SchedulingPolicy::new(
            SchedulingPolicyKind::AsSoonAsPossible,
        );

        let requirements = policy.requirements();

        assert!(requirements.contains(PolicyRequirement::Dependencies));
        assert!(requirements.contains(PolicyRequirement::ForwardTiming));
        assert!(!requirements.contains(PolicyRequirement::BackwardTiming));
    }

    #[test]
    fn requirements_for_alap_include_backward_timing() {
        let policy =
            SchedulingPolicy::new(SchedulingPolicyKind::AsLateAsPossible);

        let requirements = policy.requirements();

        assert!(requirements.contains(PolicyRequirement::BackwardTiming));
    }

    #[test]
    fn resource_policy_requires_resources() {
        let policy =
            SchedulingPolicy::new(SchedulingPolicyKind::ResourceAware);

        let requirements = policy.requirements();

        assert!(requirements.contains(PolicyRequirement::Resources));
    }

    #[test]
    fn critical_path_policy_requires_critical_path() {
        let policy =
            SchedulingPolicy::new(SchedulingPolicyKind::CriticalPath);

        let requirements = policy.requirements();

        assert!(requirements.contains(PolicyRequirement::CriticalPath));
        assert!(requirements.contains(PolicyRequirement::BackwardTiming));
    }

    #[test]
    fn requirements_have_no_heap_dependency() {
        let policy =
            SchedulingPolicy::new(SchedulingPolicyKind::Adaptive);

        let requirements = policy.requirements();

        assert!(!requirements.is_empty());
        assert!(requirements.contains(PolicyRequirement::Resources));
    }

    #[test]
    fn score_uses_checked_integer_arithmetic() {
        let policy = SchedulingPolicy::builder()
            .priority_weight(2)
            .criticality_weight(3)
            .resource_weight(4)
            .fidelity_weight(5)
            .build()
            .expect("policy must be valid");

        let score = PolicyScore::new(10, 20, 30, 40);

        let weighted = score
            .weighted(&policy)
            .expect("score must fit");

        assert_eq!(
            weighted,
            (10 * 2) + (20 * 3) + (30 * 4) + (40 * 5)
        );
    }

    #[test]
    fn score_overflow_is_rejected() {
        let policy = SchedulingPolicy::builder()
            .priority_weight(u16::MAX)
            .build()
            .expect("policy must be valid");

        let score = PolicyScore::new(i128::MAX, 0, 0, 0);

        assert_eq!(
            score.weighted(&policy),
            Err(PolicyError::ScoreOverflow)
        );
    }

    #[test]
    fn identity_is_stable() {
        let policy =
            SchedulingPolicy::new(SchedulingPolicyKind::ResourceAware);

        let identity = PolicyIdentity::from_policy(&policy);

        assert_eq!(
            identity.kind(),
            SchedulingPolicyKind::ResourceAware
        );
        assert_eq!(identity.to_string(), "resource_aware");
    }

    #[test]
    fn custom_identity_contains_plugin_name() {
        let policy = SchedulingPolicy::builder()
            .kind(SchedulingPolicyKind::Custom)
            .custom_id("research.scheduler.v1")
            .expect("identifier must be valid")
            .build()
            .expect("policy must be valid");

        let identity = PolicyIdentity::from_policy(&policy);

        assert_eq!(
            identity.to_string(),
            "custom:research.scheduler.v1"
        );
    }

    #[test]
    fn default_policy_is_const_constructible() {
        const POLICY: SchedulingPolicy = default_policy();

        assert_eq!(
            POLICY.kind(),
            SchedulingPolicyKind::AsSoonAsPossible
        );
    }

    #[test]
    fn critical_path_preset_is_deterministic() {
        let policy = critical_path_policy();

        assert_eq!(
            policy.kind(),
            SchedulingPolicyKind::CriticalPath
        );
        assert_eq!(
            policy.tie_break(),
            TieBreakRule::Criticality
        );
        assert!(policy.deterministic());
    }

    #[test]
    fn resource_aware_preset_is_resource_oriented() {
        let policy = resource_aware_policy();

        assert_eq!(
            policy.kind(),
            SchedulingPolicyKind::ResourceAware
        );
        assert_eq!(
            policy.tie_break(),
            TieBreakRule::ResourceFootprint
        );
    }

    #[test]
    fn compatibility_is_derived_from_policy() {
        let policy =
            SchedulingPolicy::new(SchedulingPolicyKind::AsLateAsPossible);

        let compatibility = PolicyCompatibility::from_policy(&policy);

        assert!(compatibility.dependencies);
        assert!(compatibility.forward_timing);
        assert!(compatibility.backward_timing);
        assert!(!compatibility.resources);
    }

    #[test]
    fn policy_contract_is_implemented() {
        let policy = SchedulingPolicy::default();

        let contract: &dyn SchedulingPolicyContract = &policy;

        assert_eq!(
            contract.kind(),
            SchedulingPolicyKind::AsSoonAsPossible
        );
        assert!(contract.validate().is_ok());
    }

    #[test]
    fn stable_names_are_not_empty() {
        let kinds = [
            SchedulingPolicyKind::AsSoonAsPossible,
            SchedulingPolicyKind::AsLateAsPossible,
            SchedulingPolicyKind::List,
            SchedulingPolicyKind::CriticalPath,
            SchedulingPolicyKind::ResourceAware,
            SchedulingPolicyKind::CriticalPathResourceAware,
            SchedulingPolicyKind::Adaptive,
            SchedulingPolicyKind::Custom,
        ];

        for kind in kinds {
            assert!(!kind.as_str().is_empty());
        }
    }
}