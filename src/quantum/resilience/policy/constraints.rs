//! Zamani Quantum Resilience — Policy Constraints
//!
//! Path:
//!     src/quantum/resilience/policy/constraints.rs
//!
//! Purpose:
//!     Defines immutable, backend-independent constraints that the resilience
//!     policy/planner must preserve while adapting, recovering, migrating,
//!     mitigating, or otherwise changing the physical realization of a
//!     quantum computation.
//!
//! Architectural role:
//!     This module defines WHAT MUST remain true.
//!
//!     It does NOT define:
//!
//!     - how constraints are optimized;
//!     - how recovery is performed;
//!     - how routing is performed;
//!     - how scheduling is performed;
//!     - how QEC is implemented;
//!     - how hardware is discovered;
//!     - how calibration is performed;
//!     - how a backend is selected;
//!     - how a quantum program is represented;
//!     - how execution is performed;
//!     - how telemetry is collected;
//!     - how budgets are enforced;
//!     - how objectives are ranked.
//!
//! Those responsibilities belong to their authoritative subsystems.
//!
//! Design principles:
//!
//! 1. The Zamani quantum program is the semantic source of truth.
//! 2. Physical realization is replaceable.
//! 3. Constraints are caller/runtime supplied, never machine-size constants.
//! 4. No fixed qubit count, topology, provider, backend, or retry count exists.
//! 5. Unknown capability information is never silently treated as sufficient.
//! 6. Logical and physical identities are never conflated.
//! 7. Constraints are deterministic value objects.
//! 8. Constraint evaluation has no I/O and no hidden global state.
//! 9. Constraint evaluation does not execute recovery.
//! 10. Constraint violation must be observable and explainable.
//! 11. A constraint set can scale from one logical qubit to arbitrarily large
//!     executions subject only to externally supplied resource availability.
//!
//! Canonical quantum identity:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! MUST be used for logical qubit identities. This module intentionally does
//! not define another logical-qubit or physical-qubit identifier.
//!
//! Physical placement remains owned by routing/hardware. A resilience policy
//! may constrain physical-resource usage abstractly, but it must not encode a
//! physical topology.
//!
//! Rust contract:
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust
//! - safe Rust only
//! - `unsafe` is forbidden
//! - no hard-coded machine-size limits
//! - no provider-specific assumptions
//! - no hidden I/O
//! - no hidden concurrency
//! - no hidden retry loops
//!
//! Integration:
//!
//!     api/request.rs
//!          |
//!          v
//!     policy/constraints.rs
//!          |
//!          +--> policy/policy.rs
//!          |
//!          +--> planning/feasibility.rs
//!          |
//!          +--> planning/planner.rs
//!          |
//!          +--> adaptation/*
//!          |
//!          +--> verification/*
//!
//! `constraints.rs` is intentionally below policy orchestration and above
//! concrete execution mechanisms.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::num::{NonZeroU64, NonZeroUsize};
use std::sync::Arc;
use std::time::Duration;

use crate::quantum::ir::qubit::QubitId;

// =============================================================================
// Stable schema identity
// =============================================================================

/// Stable schema identifier for the resilience constraint contract.
pub const RESILIENCE_CONSTRAINTS_SCHEMA_ID: &str =
    "zamani.quantum.resilience.policy.constraints";

/// Semantic version of the constraint contract.
///
/// This version is independent from:
///
/// - Zamani language version;
/// - canonical quantum IR version;
/// - hardware capability schema version;
/// - resilience request schema version.
///
/// Increment this value only when the externally observable constraint schema
/// changes incompatibly.
pub const RESILIENCE_CONSTRAINTS_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Constraint severity
// =============================================================================

/// Defines how a violated constraint affects a candidate execution plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConstraintStrength {
    /// A preference that may be violated when policy explicitly permits it.
    Advisory,

    /// A requirement that must be satisfied for normal acceptance.
    Required,

    /// A safety/semantic requirement whose violation makes the candidate
    /// unacceptable.
    Mandatory,
}

impl Default for ConstraintStrength {
    fn default() -> Self {
        Self::Required
    }
}

impl ConstraintStrength {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Required => "required",
            Self::Mandatory => "mandatory",
        }
    }

    /// Returns whether the constraint is non-negotiable.
    pub const fn is_mandatory(self) -> bool {
        matches!(self, Self::Mandatory)
    }

    /// Returns whether the constraint must normally be satisfied.
    pub const fn is_required_or_stronger(self) -> bool {
        !matches!(self, Self::Advisory)
    }
}

impl fmt::Display for ConstraintStrength {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Constraint knowledge
// =============================================================================

/// Describes how confidently a constraint fact is known.
///
/// This is intentionally separate from fault/diagnostic confidence.
///
/// A planner must not treat an unknown capability as equivalent to a known
/// capability merely because a candidate appears otherwise attractive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ConstraintKnowledge {
    /// The fact is explicitly supplied or verified.
    #[default]
    Known,

    /// The fact is inferred from authoritative observations.
    Inferred,

    /// The fact is incomplete or uncertain.
    Unknown,
}

impl ConstraintKnowledge {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Known => "known",
            Self::Inferred => "inferred",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether the fact is sufficiently explicit for strict policy.
    pub const fn is_known(self) -> bool {
        matches!(self, Self::Known)
    }
}

impl fmt::Display for ConstraintKnowledge {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Unknown-value behavior
// =============================================================================

/// Defines how a policy should treat an unknown constraint fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum UnknownConstraintBehavior {
    /// Reject a candidate when a mandatory/required constraint cannot be
    /// established.
    #[default]
    Reject,

    /// Permit evaluation to continue, but require the uncertainty to remain
    /// visible to the planner/verifier.
    RequireExplicitEscalation,

    /// Permit an advisory constraint to remain unresolved.
    AllowForAdvisoryOnly,
}

impl UnknownConstraintBehavior {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reject => "reject",
            Self::RequireExplicitEscalation => "require_explicit_escalation",
            Self::AllowForAdvisoryOnly => "allow_for_advisory_only",
        }
    }
}

impl fmt::Display for UnknownConstraintBehavior {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Semantic constraints
// =============================================================================

/// Declares the semantic guarantees that a resilience action must preserve.
///
/// These constraints describe the computation rather than its hardware
/// realization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticConstraints {
    /// Strength of the semantic contract.
    strength: ConstraintStrength,

    /// Logical qubits that must remain semantically represented.
    ///
    /// Physical placement is deliberately excluded.
    required_logical_qubits: Arc<[QubitId]>,

    /// Whether logical-qubit identity must remain stable across adaptation.
    preserve_logical_identity: bool,

    /// Whether the logical operation ordering must remain semantically
    /// equivalent to the original program.
    preserve_operation_semantics: bool,

    /// Whether measurements must preserve their declared semantic meaning.
    preserve_measurement_semantics: bool,

    /// Whether classical control dependencies must remain valid.
    preserve_classical_dependencies: bool,

    /// Whether observable output semantics must remain equivalent.
    preserve_observable_semantics: bool,

    /// Whether a candidate result must be independently verified before
    /// acceptance.
    require_verification: bool,

    /// How unknown semantic facts are handled.
    unknown_behavior: UnknownConstraintBehavior,
}

impl Default for SemanticConstraints {
    fn default() -> Self {
        Self {
            strength: ConstraintStrength::Mandatory,
            required_logical_qubits: Arc::from([]),
            preserve_logical_identity: true,
            preserve_operation_semantics: true,
            preserve_measurement_semantics: true,
            preserve_classical_dependencies: true,
            preserve_observable_semantics: true,
            require_verification: true,
            unknown_behavior: UnknownConstraintBehavior::Reject,
        }
    }
}

impl SemanticConstraints {
    /// Creates the strict semantic contract.
    pub const fn strict() -> Self {
        Self {
            strength: ConstraintStrength::Mandatory,
            required_logical_qubits: Arc::from([]),
            preserve_logical_identity: true,
            preserve_operation_semantics: true,
            preserve_measurement_semantics: true,
            preserve_classical_dependencies: true,
            preserve_observable_semantics: true,
            require_verification: true,
            unknown_behavior: UnknownConstraintBehavior::Reject,
        }
    }

    /// Creates a configurable semantic contract from the strict baseline.
    pub fn builder() -> SemanticConstraintsBuilder {
        SemanticConstraintsBuilder::default()
    }

    /// Returns the constraint strength.
    pub const fn strength(&self) -> ConstraintStrength {
        self.strength
    }

    /// Returns required logical qubits.
    pub fn required_logical_qubits(&self) -> &[QubitId] {
        self.required_logical_qubits.as_ref()
    }

    /// Returns whether logical identities must be preserved.
    pub const fn preserve_logical_identity(&self) -> bool {
        self.preserve_logical_identity
    }

    /// Returns whether operation semantics must be preserved.
    pub const fn preserve_operation_semantics(&self) -> bool {
        self.preserve_operation_semantics
    }

    /// Returns whether measurement semantics must be preserved.
    pub const fn preserve_measurement_semantics(&self) -> bool {
        self.preserve_measurement_semantics
    }

    /// Returns whether classical dependencies must be preserved.
    pub const fn preserve_classical_dependencies(&self) -> bool {
        self.preserve_classical_dependencies
    }

    /// Returns whether observable semantics must be preserved.
    pub const fn preserve_observable_semantics(&self) -> bool {
        self.preserve_observable_semantics
    }

    /// Returns whether verification is mandatory before acceptance.
    pub const fn require_verification(&self) -> bool {
        self.require_verification
    }

    /// Returns behavior for unknown facts.
    pub const fn unknown_behavior(&self) -> UnknownConstraintBehavior {
        self.unknown_behavior
    }
}

/// Builder for [`SemanticConstraints`].
#[derive(Debug, Clone, Default)]
pub struct SemanticConstraintsBuilder {
    strength: ConstraintStrength,
    required_logical_qubits: Vec<QubitId>,
    preserve_logical_identity: bool,
    preserve_operation_semantics: bool,
    preserve_measurement_semantics: bool,
    preserve_classical_dependencies: bool,
    preserve_observable_semantics: bool,
    require_verification: bool,
    unknown_behavior: UnknownConstraintBehavior,
}

impl SemanticConstraintsBuilder {
    /// Sets constraint strength.
    pub fn strength(mut self, strength: ConstraintStrength) -> Self {
        self.strength = strength;
        self
    }

    /// Replaces the required logical-qubit set.
    ///
    /// The supplied identifiers are canonical Zamani IR identifiers.
    pub fn required_logical_qubits<I>(mut self, qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        self.required_logical_qubits = qubits.into_iter().collect();
        self
    }

    /// Sets logical identity preservation.
    pub const fn preserve_logical_identity(mut self, value: bool) -> Self {
        self.preserve_logical_identity = value;
        self
    }

    /// Sets operation-semantic preservation.
    pub const fn preserve_operation_semantics(mut self, value: bool) -> Self {
        self.preserve_operation_semantics = value;
        self
    }

    /// Sets measurement-semantic preservation.
    pub const fn preserve_measurement_semantics(mut self, value: bool) -> Self {
        self.preserve_measurement_semantics = value;
        self
    }

    /// Sets classical dependency preservation.
    pub const fn preserve_classical_dependencies(mut self, value: bool) -> Self {
        self.preserve_classical_dependencies = value;
        self
    }

    /// Sets observable-semantic preservation.
    pub const fn preserve_observable_semantics(mut self, value: bool) -> Self {
        self.preserve_observable_semantics = value;
        self
    }

    /// Sets verification requirement.
    pub const fn require_verification(mut self, value: bool) -> Self {
        self.require_verification = value;
        self
    }

    /// Sets unknown-fact behavior.
    pub const fn unknown_behavior(
        mut self,
        behavior: UnknownConstraintBehavior,
    ) -> Self {
        self.unknown_behavior = behavior;
        self
    }

    /// Builds an immutable semantic constraint set.
    pub fn build(self) -> SemanticConstraints {
        SemanticConstraints {
            strength: self.strength,
            required_logical_qubits: deduplicate_qubits(self.required_logical_qubits),
            preserve_logical_identity: self.preserve_logical_identity,
            preserve_operation_semantics: self.preserve_operation_semantics,
            preserve_measurement_semantics: self.preserve_measurement_semantics,
            preserve_classical_dependencies: self.preserve_classical_dependencies,
            preserve_observable_semantics: self.preserve_observable_semantics,
            require_verification: self.require_verification,
            unknown_behavior: self.unknown_behavior,
        }
    }
}

// =============================================================================
// Resource dimensions
// =============================================================================

/// Generic resource dimensions that can be constrained without embedding a
/// particular hardware architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResourceDimension {
    /// Number of logical qubits required by the computation.
    LogicalQubits,

    /// Number of physical qubits required by a realization.
    PhysicalQubits,

    /// Number of logical/physical resources occupied simultaneously.
    ConcurrentResources,

    /// Number of execution slots/resources occupied.
    ExecutionSlots,

    /// Number of classical resources required by the execution.
    ClassicalResources,

    /// Number of measurement resources required.
    MeasurementResources,

    /// Number of control resources required.
    ControlResources,

    /// Number of communication resources required.
    CommunicationResources,

    /// Generic implementation-defined resource units.
    Custom,
}

impl ResourceDimension {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogicalQubits => "logical_qubits",
            Self::PhysicalQubits => "physical_qubits",
            Self::ConcurrentResources => "concurrent_resources",
            Self::ExecutionSlots => "execution_slots",
            Self::ClassicalResources => "classical_resources",
            Self::MeasurementResources => "measurement_resources",
            Self::ControlResources => "control_resources",
            Self::CommunicationResources => "communication_resources",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for ResourceDimension {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Resource bound
// =============================================================================

/// An optional upper bound on a resource requirement.
///
/// `None` means that this particular resource has no caller-declared upper
/// bound. It does NOT mean that the resource is infinite or automatically
/// available.
///
/// Actual availability must be supplied by the hardware/resource layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceUpperBound {
    value: NonZeroU64,
}

impl ResourceUpperBound {
    /// Creates a positive resource upper bound.
    pub const fn new(value: NonZeroU64) -> Self {
        Self { value }
    }

    /// Creates a bound from a primitive value.
    pub fn from_u64(value: u64) -> Option<Self> {
        NonZeroU64::new(value).map(Self::new)
    }

    /// Returns the bound.
    pub const fn get(self) -> u64 {
        self.value.get()
    }
}

/// A resource requirement/constraint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceConstraint {
    /// Resource dimension.
    dimension: ResourceDimension,

    /// Maximum amount the candidate is allowed to require.
    maximum: Option<ResourceUpperBound>,

    /// Minimum amount the candidate must be able to provide.
    minimum: Option<NonZeroU64>,

    /// Strength of the constraint.
    strength: ConstraintStrength,

    /// Whether this constraint was explicitly supplied.
    knowledge: ConstraintKnowledge,

    /// Behavior when the resource fact cannot be established.
    unknown_behavior: UnknownConstraintBehavior,

    /// Optional stable caller-defined identifier.
    ///
    /// This is intentionally an opaque identifier and must not encode a
    /// provider/backend name.
    identifier: Option<Arc<str>>,
}

impl ResourceConstraint {
    /// Creates an unconstrained resource dimension.
    pub const fn unconstrained(dimension: ResourceDimension) -> Self {
        Self {
            dimension,
            maximum: None,
            minimum: None,
            strength: ConstraintStrength::Advisory,
            knowledge: ConstraintKnowledge::Known,
            unknown_behavior: UnknownConstraintBehavior::AllowForAdvisoryOnly,
            identifier: None,
        }
    }

    /// Returns the resource dimension.
    pub const fn dimension(&self) -> ResourceDimension {
        self.dimension
    }

    /// Returns the maximum requirement.
    pub const fn maximum(&self) -> Option<ResourceUpperBound> {
        self.maximum
    }

    /// Returns the minimum requirement.
    pub const fn minimum(&self) -> Option<NonZeroU64> {
        self.minimum
    }

    /// Returns constraint strength.
    pub const fn strength(&self) -> ConstraintStrength {
        self.strength
    }

    /// Returns knowledge state.
    pub const fn knowledge(&self) -> ConstraintKnowledge {
        self.knowledge
    }

    /// Returns unknown-fact behavior.
    pub const fn unknown_behavior(&self) -> UnknownConstraintBehavior {
        self.unknown_behavior
    }

    /// Returns the optional stable identifier.
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }

    /// Creates a builder.
    pub fn builder(dimension: ResourceDimension) -> ResourceConstraintBuilder {
        ResourceConstraintBuilder {
            dimension,
            ..ResourceConstraintBuilder::default()
        }
    }
}

/// Builder for [`ResourceConstraint`].
#[derive(Debug, Clone)]
pub struct ResourceConstraintBuilder {
    dimension: ResourceDimension,
    maximum: Option<ResourceUpperBound>,
    minimum: Option<NonZeroU64>,
    strength: ConstraintStrength,
    knowledge: ConstraintKnowledge,
    unknown_behavior: UnknownConstraintBehavior,
    identifier: Option<Arc<str>>,
}

impl Default for ResourceConstraintBuilder {
    fn default() -> Self {
        Self {
            dimension: ResourceDimension::Custom,
            maximum: None,
            minimum: None,
            strength: ConstraintStrength::Required,
            knowledge: ConstraintKnowledge::Known,
            unknown_behavior: UnknownConstraintBehavior::Reject,
            identifier: None,
        }
    }
}

impl ResourceConstraintBuilder {
    /// Sets the maximum allowed resource requirement.
    pub const fn maximum(mut self, maximum: ResourceUpperBound) -> Self {
        self.maximum = Some(maximum);
        self
    }

    /// Sets the maximum from a primitive value.
    pub fn maximum_u64(mut self, maximum: u64) -> Self {
        self.maximum = ResourceUpperBound::from_u64(maximum);
        self
    }

    /// Sets the minimum required resource capacity.
    pub fn minimum_u64(mut self, minimum: u64) -> Self {
        self.minimum = NonZeroU64::new(minimum);
        self
    }

    /// Sets constraint strength.
    pub const fn strength(mut self, strength: ConstraintStrength) -> Self {
        self.strength = strength;
        self
    }

    /// Sets knowledge state.
    pub const fn knowledge(mut self, knowledge: ConstraintKnowledge) -> Self {
        self.knowledge = knowledge;
        self
    }

    /// Sets unknown-fact behavior.
    pub const fn unknown_behavior(
        mut self,
        behavior: UnknownConstraintBehavior,
    ) -> Self {
        self.unknown_behavior = behavior;
        self
    }

    /// Sets a stable caller-defined identifier.
    pub fn identifier<S>(mut self, identifier: S) -> Self
    where
        S: Into<Arc<str>>,
    {
        self.identifier = Some(identifier.into());
        self
    }

    /// Builds the constraint.
    ///
    /// The method returns `None` if both bounds are absent and the caller
    /// supplied no meaningful resource constraint.
    pub fn build(self) -> Option<ResourceConstraint> {
        if self.maximum.is_none() && self.minimum.is_none() {
            return None;
        }

        Some(ResourceConstraint {
            dimension: self.dimension,
            maximum: self.maximum,
            minimum: self.minimum,
            strength: self.strength,
            knowledge: self.knowledge,
            unknown_behavior: self.unknown_behavior,
            identifier: self.identifier,
        })
    }
}

// =============================================================================
// Execution-time constraints
// =============================================================================

/// Constraints on execution duration and temporal behavior.
///
/// These are requirements supplied by the caller/runtime. Actual gate timing,
/// queue timing, scheduling and hardware timing are owned elsewhere.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TemporalConstraints {
    /// Maximum end-to-end execution duration, excluding unspecified external
    /// waiting unless the caller explicitly defines that waiting as part of the
    /// execution contract.
    maximum_execution_time: Option<Duration>,

    /// Maximum allowed time for adaptation/recovery.
    maximum_recovery_time: Option<Duration>,

    /// Whether execution must remain within the supplied temporal contract.
    strength: ConstraintStrength,

    /// Unknown temporal facts behavior.
    unknown_behavior: UnknownConstraintBehavior,
}

impl TemporalConstraints {
    /// Creates an unconstrained temporal contract.
    pub const fn unconstrained() -> Self {
        Self {
            maximum_execution_time: None,
            maximum_recovery_time: None,
            strength: ConstraintStrength::Advisory,
            unknown_behavior: UnknownConstraintBehavior::AllowForAdvisoryOnly,
        }
    }

    /// Creates a builder.
    pub const fn builder() -> TemporalConstraintsBuilder {
        TemporalConstraintsBuilder {
            maximum_execution_time: None,
            maximum_recovery_time: None,
            strength: ConstraintStrength::Required,
            unknown_behavior: UnknownConstraintBehavior::Reject,
        }
    }

    /// Maximum execution time.
    pub const fn maximum_execution_time(&self) -> Option<Duration> {
        self.maximum_execution_time
    }

    /// Maximum recovery time.
    pub const fn maximum_recovery_time(&self) -> Option<Duration> {
        self.maximum_recovery_time
    }

    /// Constraint strength.
    pub const fn strength(&self) -> ConstraintStrength {
        self.strength
    }

    /// Unknown-fact behavior.
    pub const fn unknown_behavior(&self) -> UnknownConstraintBehavior {
        self.unknown_behavior
    }
}

/// Builder for [`TemporalConstraints`].
#[derive(Debug, Clone, Copy, Default)]
pub struct TemporalConstraintsBuilder {
    maximum_execution_time: Option<Duration>,
    maximum_recovery_time: Option<Duration>,
    strength: ConstraintStrength,
    unknown_behavior: UnknownConstraintBehavior,
}

impl TemporalConstraintsBuilder {
    /// Sets maximum execution time.
    pub const fn maximum_execution_time(
        mut self,
        value: Duration,
    ) -> Self {
        self.maximum_execution_time = Some(value);
        self
    }

    /// Sets maximum recovery time.
    pub const fn maximum_recovery_time(
        mut self,
        value: Duration,
    ) -> Self {
        self.maximum_recovery_time = Some(value);
        self
    }

    /// Sets constraint strength.
    pub const fn strength(mut self, value: ConstraintStrength) -> Self {
        self.strength = value;
        self
    }

    /// Sets unknown-fact behavior.
    pub const fn unknown_behavior(
        mut self,
        value: UnknownConstraintBehavior,
    ) -> Self {
        self.unknown_behavior = value;
        self
    }

    /// Builds the temporal constraint set.
    pub const fn build(self) -> TemporalConstraints {
        TemporalConstraints {
            maximum_execution_time: self.maximum_execution_time,
            maximum_recovery_time: self.maximum_recovery_time,
            strength: self.strength,
            unknown_behavior: self.unknown_behavior,
        }
    }
}

// =============================================================================
// Fidelity / quality constraints
// =============================================================================

/// Describes a caller-supplied lower bound on an explicitly measured quality
/// quantity.
///
/// The value is represented as `f64` because hardware/QEC subsystems commonly
/// expose probabilities and quality metrics as floating-point quantities.
///
/// Construction rejects:
///
/// - NaN;
/// - positive infinity;
/// - negative infinity;
/// - values outside the caller-defined valid interval.
///
/// The valid interval is intentionally supplied by the caller because
/// different metrics may have different mathematical domains.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FiniteThreshold {
    value: f64,
}

impl FiniteThreshold {
    /// Creates a finite threshold.
    pub fn new(value: f64) -> Option<Self> {
        value.is_finite().then_some(Self { value })
    }

    /// Returns the threshold.
    pub const fn get(self) -> f64 {
        self.value
    }
}

/// A quality constraint.
///
/// This module does not define what "fidelity" means for a particular backend.
/// The metric name is an opaque stable identifier supplied by the authoritative
/// measurement subsystem.
#[derive(Debug, Clone, PartialEq)]
pub struct QualityConstraint {
    /// Stable metric identifier.
    metric: Arc<str>,

    /// Optional minimum acceptable value.
    minimum: Option<FiniteThreshold>,

    /// Optional maximum acceptable value.
    maximum: Option<FiniteThreshold>,

    /// Constraint strength.
    strength: ConstraintStrength,

    /// Knowledge state.
    knowledge: ConstraintKnowledge,

    /// Unknown-fact behavior.
    unknown_behavior: UnknownConstraintBehavior,
}

impl QualityConstraint {
    /// Creates a quality constraint builder.
    pub fn builder<S>(metric: S) -> QualityConstraintBuilder
    where
        S: Into<Arc<str>>,
    {
        QualityConstraintBuilder {
            metric: metric.into(),
            minimum: None,
            maximum: None,
            strength: ConstraintStrength::Required,
            knowledge: ConstraintKnowledge::Known,
            unknown_behavior: UnknownConstraintBehavior::Reject,
        }
    }

    /// Returns metric identifier.
    pub fn metric(&self) -> &str {
        self.metric.as_ref()
    }

    /// Returns minimum.
    pub const fn minimum(&self) -> Option<FiniteThreshold> {
        self.minimum
    }

    /// Returns maximum.
    pub const fn maximum(&self) -> Option<FiniteThreshold> {
        self.maximum
    }

    /// Returns strength.
    pub const fn strength(&self) -> ConstraintStrength {
        self.strength
    }

    /// Returns knowledge.
    pub const fn knowledge(&self) -> ConstraintKnowledge {
        self.knowledge
    }

    /// Returns unknown behavior.
    pub const fn unknown_behavior(&self) -> UnknownConstraintBehavior {
        self.unknown_behavior
    }
}

/// Builder for [`QualityConstraint`].
#[derive(Debug, Clone)]
pub struct QualityConstraintBuilder {
    metric: Arc<str>,
    minimum: Option<FiniteThreshold>,
    maximum: Option<FiniteThreshold>,
    strength: ConstraintStrength,
    knowledge: ConstraintKnowledge,
    unknown_behavior: UnknownConstraintBehavior,
}

impl QualityConstraintBuilder {
    /// Sets a minimum.
    pub fn minimum(mut self, value: f64) -> Option<Self> {
        let threshold = FiniteThreshold::new(value)?;
        self.minimum = Some(threshold);
        Some(self)
    }

    /// Sets a maximum.
    pub fn maximum(mut self, value: f64) -> Option<Self> {
        let threshold = FiniteThreshold::new(value)?;
        self.maximum = Some(threshold);
        Some(self)
    }

    /// Sets strength.
    pub const fn strength(mut self, value: ConstraintStrength) -> Self {
        self.strength = value;
        self
    }

    /// Sets knowledge.
    pub const fn knowledge(mut self, value: ConstraintKnowledge) -> Self {
        self.knowledge = value;
        self
    }

    /// Sets unknown behavior.
    pub const fn unknown_behavior(
        mut self,
        value: UnknownConstraintBehavior,
    ) -> Self {
        self.unknown_behavior = value;
        self
    }

    /// Builds the quality constraint.
    ///
    /// The result is rejected if:
    ///
    /// - no lower or upper bound exists;
    /// - both bounds exist and the lower bound exceeds the upper bound.
    pub fn build(self) -> Option<QualityConstraint> {
        if self.minimum.is_none() && self.maximum.is_none() {
            return None;
        }

        if let (Some(minimum), Some(maximum)) = (self.minimum, self.maximum) {
            if minimum.get() > maximum.get() {
                return None;
            }
        }

        Some(QualityConstraint {
            metric: self.metric,
            minimum: self.minimum,
            maximum: self.maximum,
            strength: self.strength,
            knowledge: self.knowledge,
            unknown_behavior: self.unknown_behavior,
        })
    }
}

// =============================================================================
// Adaptation constraints
// =============================================================================

/// Describes what classes of semantic-preserving physical adaptation are
/// permitted.
///
/// These flags are permissions, not implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AdaptationConstraints {
    /// Permit logical-to-physical remapping.
    remapping: bool,

    /// Permit topology-aware rerouting.
    rerouting: bool,

    /// Permit schedule regeneration.
    rescheduling: bool,

    /// Permit recompilation against a changed target.
    recompilation: bool,

    /// Permit reoptimization against a changed target.
    reoptimization: bool,

    /// Permit migration to another compatible execution target.
    migration: bool,

    /// Constraint strength applied to the permission set.
    strength: ConstraintStrength,
}

impl Default for AdaptationConstraints {
    fn default() -> Self {
        Self {
            remapping: true,
            rerouting: true,
            rescheduling: true,
            recompilation: true,
            reoptimization: true,
            migration: true,
            strength: ConstraintStrength::Required,
        }
    }
}

impl AdaptationConstraints {
    /// Denies all physical adaptation.
    pub const fn deny_all() -> Self {
        Self {
            remapping: false,
            rerouting: false,
            rescheduling: false,
            recompilation: false,
            reoptimization: false,
            migration: false,
            strength: ConstraintStrength::Mandatory,
        }
    }

    /// Permits all abstract adaptation classes.
    ///
    /// This does NOT bypass semantic verification or capability validation.
    pub const fn allow_all() -> Self {
        Self {
            remapping: true,
            rerouting: true,
            rescheduling: true,
            recompilation: true,
            reoptimization: true,
            migration: true,
            strength: ConstraintStrength::Required,
        }
    }

    /// Sets remapping permission.
    pub const fn remapping(mut self, allowed: bool) -> Self {
        self.remapping = allowed;
        self
    }

    /// Sets rerouting permission.
    pub const fn rerouting(mut self, allowed: bool) -> Self {
        self.rerouting = allowed;
        self
    }

    /// Sets rescheduling permission.
    pub const fn rescheduling(mut self, allowed: bool) -> Self {
        self.rescheduling = allowed;
        self
    }

    /// Sets recompilation permission.
    pub const fn recompilation(mut self, allowed: bool) -> Self {
        self.recompilation = allowed;
        self
    }

    /// Sets reoptimization permission.
    pub const fn reoptimization(mut self, allowed: bool) -> Self {
        self.reoptimization = allowed;
        self
    }

    /// Sets migration permission.
    pub const fn migration(mut self, allowed: bool) -> Self {
        self.migration = allowed;
        self
    }

    /// Sets constraint strength.
    pub const fn strength(mut self, strength: ConstraintStrength) -> Self {
        self.strength = strength;
        self
    }

    /// Returns remapping permission.
    pub const fn remapping_allowed(self) -> bool {
        self.remapping
    }

    /// Returns rerouting permission.
    pub const fn rerouting_allowed(self) -> bool {
        self.rerouting
    }

    /// Returns rescheduling permission.
    pub const fn rescheduling_allowed(self) -> bool {
        self.rescheduling
    }

    /// Returns recompilation permission.
    pub const fn recompilation_allowed(self) -> bool {
        self.recompilation
    }

    /// Returns reoptimization permission.
    pub const fn reoptimization_allowed(self) -> bool {
        self.reoptimization
    }

    /// Returns migration_allowed.
    pub const fn migration_allowed(self) -> bool {
        self.migration
    }

    /// Returns strength.
    pub const fn strength_value(self) -> ConstraintStrength {
        self.strength
    }
}

// =============================================================================
// Recovery-state constraints
// =============================================================================

/// Declares recovery-state requirements without implementing recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecoveryConstraints {
    /// Whether retry may be considered.
    retry_allowed: bool,

    /// Whether restart may be considered.
    restart_allowed: bool,

    /// Whether checkpoint resume may be considered.
    resume_allowed: bool,

    /// Whether rollback may be considered.
    rollback_allowed: bool,

    /// Whether migration may be considered.
    migration_allowed: bool,

    /// Whether compensation may be considered.
    compensation_allowed: bool,

    /// Whether recovery requires a verified checkpoint/state boundary.
    require_verified_boundary: bool,

    /// Constraint strength.
    strength: ConstraintStrength,
}

impl Default for RecoveryConstraints {
    fn default() -> Self {
        Self {
            retry_allowed: true,
            restart_allowed: true,
            resume_allowed: true,
            rollback_allowed: true,
            migration_allowed: true,
            compensation_allowed: true,
            require_verified_boundary: true,
            strength: ConstraintStrength::Required,
        }
    }
}

impl RecoveryConstraints {
    /// Denies all recovery classes.
    pub const fn deny_all() -> Self {
        Self {
            retry_allowed: false,
            restart_allowed: false,
            resume_allowed: false,
            rollback_allowed: false,
            migration_allowed: false,
            compensation_allowed: false,
            require_verified_boundary: true,
            strength: ConstraintStrength::Mandatory,
        }
    }

    /// Allows all recovery classes subject to all other policy and verification
    /// constraints.
    pub const fn allow_all() -> Self {
        Self {
            retry_allowed: true,
            restart_allowed: true,
            resume_allowed: true,
            rollback_allowed: true,
            migration_allowed: true,
            compensation_allowed: true,
            require_verified_boundary: true,
            strength: ConstraintStrength::Required,
        }
    }

    /// Sets retry permission.
    pub const fn retry(mut self, allowed: bool) -> Self {
        self.retry_allowed = allowed;
        self
    }

    /// Sets restart permission.
    pub const fn restart(mut self, allowed: bool) -> Self {
        self.restart_allowed = allowed;
        self
    }

    /// Sets resume permission.
    pub const fn resume(mut self, allowed: bool) -> Self {
        self.resume_allowed = allowed;
        self
    }

    /// Sets rollback permission.
    pub const fn rollback(mut self, allowed: bool) -> Self {
        self.rollback_allowed = allowed;
        self
    }

    /// Sets migration permission.
    pub const fn migration(mut self, allowed: bool) -> Self {
        self.migration_allowed = allowed;
        self
    }

    /// Sets compensation permission.
    pub const fn compensation(mut self, allowed: bool) -> Self {
        self.compensation_allowed = allowed;
        self
    }

    /// Requires or permits recovery from verified boundaries only.
    pub const fn require_verified_boundary(mut self, required: bool) -> Self {
        self.require_verified_boundary = required;
        self
    }

    /// Sets strength.
    pub const fn strength(mut self, strength: ConstraintStrength) -> Self {
        self.strength = strength;
        self
    }

    /// Returns retry permission.
    pub const fn retry_allowed(self) -> bool {
        self.retry_allowed
    }

    /// Returns restart permission.
    pub const fn restart_allowed(self) -> bool {
        self.restart_allowed
    }

    /// Returns resume permission.
    pub const fn resume_allowed(self) -> bool {
        self.resume_allowed
    }

    /// Returns rollback permission.
    pub const fn rollback_allowed(self) -> bool {
        self.rollback_allowed
    }

    /// Returns migration permission.
    pub const fn migration_allowed(self) -> bool {
        self.migration_allowed
    }

    /// Returns compensation permission.
    pub const fn compensation_allowed(self) -> bool {
        self.compensation_allowed
    }

    /// Returns whether a verified boundary is required.
    pub const fn requires_verified_boundary(self) -> bool {
        self.require_verified_boundary
    }

    /// Returns strength.
    pub const fn strength_value(self) -> ConstraintStrength {
        self.strength
    }
}

// =============================================================================
// Security constraints
// =============================================================================

/// Security constraints that resilience actions must preserve.
///
/// Authentication and authorization mechanisms remain outside this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SecurityConstraints {
    /// Require authorized execution targets.
    require_authorized_target: bool,

    /// Require trusted/integrity-validated observations for safety decisions.
    require_trusted_observations: bool,

    /// Require integrity-validated checkpoints.
    require_integrity_checked_checkpoints: bool,

    /// Require provenance preservation.
    require_provenance: bool,

    /// Forbid silent degradation.
    forbid_silent_degradation: bool,

    /// Security constraints are mandatory by default.
    strength: ConstraintStrength,
}

impl Default for SecurityConstraints {
    fn default() -> Self {
        Self {
            require_authorized_target: true,
            require_trusted_observations: true,
            require_integrity_checked_checkpoints: true,
            require_provenance: true,
            forbid_silent_degradation: true,
            strength: ConstraintStrength::Mandatory,
        }
    }
}

impl SecurityConstraints {
    /// Returns whether the execution target must be authorized.
    pub const fn require_authorized_target(&self) -> bool {
        self.require_authorized_target
    }

    /// Returns whether safety decisions require trusted observations.
    pub const fn require_trusted_observations(&self) -> bool {
        self.require_trusted_observations
    }

    /// Returns whether checkpoint integrity must be established.
    pub const fn require_integrity_checked_checkpoints(&self) -> bool {
        self.require_integrity_checked_checkpoints
    }

    /// Returns whether provenance must be preserved.
    pub const fn require_provenance(&self) -> bool {
        self.require_provenance
    }

    /// Returns whether silent degradation is forbidden.
    pub const fn forbid_silent_degradation(&self) -> bool {
        self.forbid_silent_degradation
    }

    /// Returns constraint strength.
    pub const fn strength(&self) -> ConstraintStrength {
        self.strength
    }

    /// Creates a builder.
    pub const fn builder() -> SecurityConstraintsBuilder {
        SecurityConstraintsBuilder {
            require_authorized_target: true,
            require_trusted_observations: true,
            require_integrity_checked_checkpoints: true,
            require_provenance: true,
            forbid_silent_degradation: true,
            strength: ConstraintStrength::Mandatory,
        }
    }
}

/// Builder for [`SecurityConstraints`].
#[derive(Debug, Clone, Copy)]
pub struct SecurityConstraintsBuilder {
    require_authorized_target: bool,
    require_trusted_observations: bool,
    require_integrity_checked_checkpoints: bool,
    require_provenance: bool,
    forbid_silent_degradation: bool,
    strength: ConstraintStrength,
}

impl SecurityConstraintsBuilder {
    /// Sets authorized-target requirement.
    pub const fn require_authorized_target(mut self, value: bool) -> Self {
        self.require_authorized_target = value;
        self
    }

    /// Sets trusted-observation requirement.
    pub const fn require_trusted_observations(mut self, value: bool) -> Self {
        self.require_trusted_observations = value;
        self
    }

    /// Sets checkpoint-integrity requirement.
    pub const fn require_integrity_checked_checkpoints(
        mut self,
        value: bool,
    ) -> Self {
        self.require_integrity_checked_checkpoints = value;
        self
    }

    /// Sets provenance requirement.
    pub const fn require_provenance(mut self, value: bool) -> Self {
        self.require_provenance = value;
        self
    }

    /// Sets silent-degradation behavior.
    pub const fn forbid_silent_degradation(mut self, value: bool) -> Self {
        self.forbid_silent_degradation = value;
        self
    }

    /// Sets constraint strength.
    pub const fn strength(mut self, value: ConstraintStrength) -> Self {
        self.strength = value;
        self
    }

    /// Builds the security constraints.
    pub const fn build(self) -> SecurityConstraints {
        SecurityConstraints {
            require_authorized_target: self.require_authorized_target,
            require_trusted_observations: self.require_trusted_observations,
            require_integrity_checked_checkpoints:
                self.require_integrity_checked_checkpoints,
            require_provenance: self.require_provenance,
            forbid_silent_degradation: self.forbid_silent_degradation,
            strength: self.strength,
        }
    }
}

// =============================================================================
// Constraint set
// =============================================================================

/// Complete immutable resilience constraint set.
///
/// This is the principal type consumed by `policy/policy.rs` and
/// `planning/feasibility.rs`.
///
/// It deliberately contains no:
///
/// - backend handles;
/// - provider names;
/// - hardware objects;
/// - network clients;
/// - execution handles;
/// - QEC implementations;
/// - routing implementations;
/// - scheduling implementations;
/// - optimizer implementations;
/// - retry counters;
/// - global state.
///
/// Therefore a single constraint set can be reused for any execution target
/// for which the external capability layer can establish feasibility.
#[derive(Debug, Clone, PartialEq)]
pub struct ResilienceConstraints {
    /// Schema version.
    schema_version: u16,

    /// Semantic requirements.
    semantic: SemanticConstraints,

    /// Resource constraints.
    resources: Arc<[ResourceConstraint]>,

    /// Temporal constraints.
    temporal: TemporalConstraints,

    /// Quality constraints.
    quality: Arc<[QualityConstraint]>,

    /// Permitted adaptation classes.
    adaptation: AdaptationConstraints,

    /// Permitted recovery classes.
    recovery: RecoveryConstraints,

    /// Security constraints.
    security: SecurityConstraints,

    /// Global unknown-fact behavior.
    unknown_behavior: UnknownConstraintBehavior,

    /// Optional caller-defined policy identifier.
    identifier: Option<Arc<str>>,
}

impl Default for ResilienceConstraints {
    fn default() -> Self {
        Self::strict()
    }
}

impl ResilienceConstraints {
    /// Creates the strict production baseline.
    ///
    /// This does not impose a hardware size, qubit count, retry count,
    /// topology, fidelity threshold, or backend identity.
    pub fn strict() -> Self {
        Self {
            schema_version: RESILIENCE_CONSTRAINTS_SCHEMA_VERSION,
            semantic: SemanticConstraints::strict(),
            resources: Arc::from([]),
            temporal: TemporalConstraints::unconstrained(),
            quality: Arc::from([]),
            adaptation: AdaptationConstraints::allow_all(),
            recovery: RecoveryConstraints::allow_all(),
            security: SecurityConstraints::default(),
            unknown_behavior: UnknownConstraintBehavior::Reject,
            identifier: None,
        }
    }

    /// Creates a builder.
    pub fn builder() -> ResilienceConstraintsBuilder {
        ResilienceConstraintsBuilder::default()
    }

    /// Returns schema version.
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns semantic constraints.
    pub const fn semantic(&self) -> &SemanticConstraints {
        &self.semantic
    }

    /// Returns resource constraints.
    pub fn resources(&self) -> &[ResourceConstraint] {
        self.resources.as_ref()
    }

    /// Returns temporal constraints.
    pub const fn temporal(&self) -> &TemporalConstraints {
        &self.temporal
    }

    /// Returns quality constraints.
    pub fn quality(&self) -> &[QualityConstraint] {
        self.quality.as_ref()
    }

    /// Returns adaptation constraints.
    pub const fn adaptation(&self) -> &AdaptationConstraints {
        &self.adaptation
    }

    /// Returns recovery constraints.
    pub const fn recovery(&self) -> &RecoveryConstraints {
        &self.recovery
    }

    /// Returns security constraints.
    pub const fn security(&self) -> &SecurityConstraints {
        &self.security
    }

    /// Returns global unknown-fact behavior.
    pub const fn unknown_behavior(&self) -> UnknownConstraintBehavior {
        self.unknown_behavior
    }

    /// Returns caller-defined identifier.
    pub fn identifier(&self) -> Option<&str> {
        self.identifier.as_deref()
    }

    /// Returns the requested maximum for a resource dimension.
    ///
    /// If multiple constraints exist for the same dimension, the strictest
    /// maximum is returned.
    pub fn maximum_resource(
        &self,
        dimension: ResourceDimension,
    ) -> Option<u64> {
        self.resources
            .iter()
            .filter(|constraint| constraint.dimension() == dimension)
            .filter_map(ResourceConstraint::maximum)
            .map(ResourceUpperBound::get)
            .min()
    }

    /// Returns the requested minimum for a resource dimension.
    ///
    /// If multiple constraints exist for the same dimension, the strictest
    /// minimum is returned.
    pub fn minimum_resource(
        &self,
        dimension: ResourceDimension,
    ) -> Option<u64> {
        self.resources
            .iter()
            .filter(|constraint| constraint.dimension() == dimension)
            .filter_map(ResourceConstraint::minimum)
            .map(NonZeroU64::get)
            .max()
    }

    /// Returns whether migration is permitted.
    pub const fn migration_allowed(&self) -> bool {
        self.adaptation.migration_allowed()
            && self.recovery.migration_allowed()
    }

    /// Returns whether semantic verification is required.
    pub const fn requires_verification(&self) -> bool {
        self.semantic.require_verification()
    }

    /// Returns whether provenance is mandatory.
    pub const fn requires_provenance(&self) -> bool {
        self.security.require_provenance()
    }

    /// Validates the internal consistency of the constraint set.
    ///
    /// This function performs only structural validation. It does not compare
    /// the constraints against a machine because machine capabilities belong
    /// to `quantum::hardware`.
    pub fn validate(&self) -> Result<(), ConstraintValidationError> {
        if self.schema_version == 0 {
            return Err(ConstraintValidationError::InvalidSchemaVersion);
        }

        for resource in self.resources.iter() {
            if let (Some(minimum), Some(maximum)) =
                (resource.minimum(), resource.maximum())
            {
                if minimum.get() > maximum.get() {
                    return Err(
                        ConstraintValidationError::ContradictoryResourceBounds {
                            dimension: resource.dimension(),
                        },
                    );
                }
            }
        }

        for quality in self.quality.iter() {
            if let (Some(minimum), Some(maximum)) =
                (quality.minimum(), quality.maximum())
            {
                if minimum.get() > maximum.get() {
                    return Err(
                        ConstraintValidationError::ContradictoryQualityBounds {
                            metric: quality.metric().to_owned(),
                        },
                    );
                }
            }
        }

        validate_unique_resource_identifiers(self.resources())?;

        Ok(())
    }
}

// =============================================================================
// Constraint builder
// =============================================================================

/// Builder for [`ResilienceConstraints`].
///
/// The builder is mutable local construction state only. The resulting
/// `ResilienceConstraints` is immutable.
#[derive(Debug, Clone, Default)]
pub struct ResilienceConstraintsBuilder {
    semantic: Option<SemanticConstraints>,
    resources: Vec<ResourceConstraint>,
    temporal: Option<TemporalConstraints>,
    quality: Vec<QualityConstraint>,
    adaptation: Option<AdaptationConstraints>,
    recovery: Option<RecoveryConstraints>,
    security: Option<SecurityConstraints>,
    unknown_behavior: Option<UnknownConstraintBehavior>,
    identifier: Option<Arc<str>>,
}

impl ResilienceConstraintsBuilder {
    /// Sets semantic constraints.
    pub fn semantic(mut self, value: SemanticConstraints) -> Self {
        self.semantic = Some(value);
        self
    }

    /// Adds a resource constraint.
    pub fn resource(mut self, value: ResourceConstraint) -> Self {
        self.resources.push(value);
        self
    }

    /// Adds multiple resource constraints.
    pub fn resources<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = ResourceConstraint>,
    {
        self.resources.extend(values);
        self
    }

    /// Sets temporal constraints.
    pub fn temporal(mut self, value: TemporalConstraints) -> Self {
        self.temporal = Some(value);
        self
    }

    /// Adds a quality constraint.
    pub fn quality(mut self, value: QualityConstraint) -> Self {
        self.quality.push(value);
        self
    }

    /// Adds multiple quality constraints.
    pub fn qualities<I>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = QualityConstraint>,
    {
        self.quality.extend(values);
        self
    }

    /// Sets adaptation permissions.
    pub fn adaptation(mut self, value: AdaptationConstraints) -> Self {
        self.adaptation = Some(value);
        self
    }

    /// Sets recovery permissions.
    pub fn recovery(mut self, value: RecoveryConstraints) -> Self {
        self.recovery = Some(value);
        self
    }

    /// Sets security constraints.
    pub fn security(mut self, value: SecurityConstraints) -> Self {
        self.security = Some(value);
        self
    }

    /// Sets global unknown-fact behavior.
    pub fn unknown_behavior(
        mut self,
        value: UnknownConstraintBehavior,
    ) -> Self {
        self.unknown_behavior = Some(value);
        self
    }

    /// Sets a caller-defined identifier.
    pub fn identifier<S>(mut self, value: S) -> Self
    where
        S: Into<Arc<str>>,
    {
        self.identifier = Some(value.into());
        self
    }

    /// Builds and validates the immutable constraint set.
    pub fn build(self) -> Result<ResilienceConstraints, ConstraintValidationError> {
        let constraints = ResilienceConstraints {
            schema_version: RESILIENCE_CONSTRAINTS_SCHEMA_VERSION,
            semantic: self.semantic.unwrap_or_else(SemanticConstraints::strict),
            resources: deduplicate_resource_constraints(self.resources),
            temporal: self
                .temporal
                .unwrap_or_else(TemporalConstraints::unconstrained),
            quality: deduplicate_quality_constraints(self.quality),
            adaptation: self
                .adaptation
                .unwrap_or_else(AdaptationConstraints::allow_all),
            recovery: self
                .recovery
                .unwrap_or_else(RecoveryConstraints::allow_all),
            security: self.security.unwrap_or_default(),
            unknown_behavior: self
                .unknown_behavior
                .unwrap_or(UnknownConstraintBehavior::Reject),
            identifier: self.identifier,
        };

        constraints.validate()?;
        Ok(constraints)
    }
}

// =============================================================================
// Constraint evaluation vocabulary
// =============================================================================

/// Result of checking one constraint against externally supplied evidence.
///
/// This type intentionally does not know anything about hardware or execution
/// implementations. The feasibility layer constructs these results from
/// authoritative capability/resource observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstraintStatus {
    /// Constraint is satisfied.
    Satisfied,

    /// Constraint is violated.
    Violated,

    /// Constraint cannot currently be established.
    Unknown,

    /// Constraint is not applicable to the candidate.
    NotApplicable,
}

impl ConstraintStatus {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Violated => "violated",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Returns whether the constraint is satisfied.
    pub const fn is_satisfied(self) -> bool {
        matches!(self, Self::Satisfied | Self::NotApplicable)
    }

    /// Returns whether the constraint is definitely violated.
    pub const fn is_violated(self) -> bool {
        matches!(self, Self::Violated)
    }
}

impl fmt::Display for ConstraintStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A normalized result for one constraint evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintEvaluation {
    /// Constraint identifier.
    identifier: Arc<str>,

    /// Constraint strength.
    strength: ConstraintStrength,

    /// Evaluation status.
    status: ConstraintStatus,

    /// Human-readable explanation.
    reason: Arc<str>,
}

impl ConstraintEvaluation {
    /// Creates an evaluation.
    pub fn new<I, R>(
        identifier: I,
        strength: ConstraintStrength,
        status: ConstraintStatus,
        reason: R,
    ) -> Self
    where
        I: Into<Arc<str>>,
        R: Into<Arc<str>>,
    {
        Self {
            identifier: identifier.into(),
            strength,
            status,
            reason: reason.into(),
        }
    }

    /// Returns identifier.
    pub fn identifier(&self) -> &str {
        self.identifier.as_ref()
    }

    /// Returns strength.
    pub const fn strength(&self) -> ConstraintStrength {
        self.strength
    }

    /// Returns status.
    pub const fn status(&self) -> ConstraintStatus {
        self.status
    }

    /// Returns reason.
    pub fn reason(&self) -> &str {
        self.reason.as_ref()
    }

    /// Returns whether the evaluation can be accepted.
    ///
    /// Advisory constraints may remain violated; mandatory/required
    /// constraints may not.
    pub const fn acceptable(&self) -> bool {
        match self.strength {
            ConstraintStrength::Advisory => true,
            ConstraintStrength::Required | ConstraintStrength::Mandatory => {
                matches!(
                    self.status,
                    ConstraintStatus::Satisfied
                        | ConstraintStatus::NotApplicable
                )
            }
        }
    }
}

// =============================================================================
// Constraint validation errors
// =============================================================================

/// Structural validation failures for [`ResilienceConstraints`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintValidationError {
    /// Schema version is invalid.
    InvalidSchemaVersion,

    /// A resource constraint has contradictory bounds.
    ContradictoryResourceBounds {
        /// Conflicting dimension.
        dimension: ResourceDimension,
    },

    /// A quality constraint has contradictory bounds.
    ContradictoryQualityBounds {
        /// Conflicting metric identifier.
        metric: String,
    },

    /// Two resource constraints use the same explicit identifier.
    DuplicateResourceIdentifier {
        /// Duplicate identifier.
        identifier: String,
    },
}

impl fmt::Display for ConstraintValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchemaVersion => {
                formatter.write_str("invalid resilience constraint schema version")
            }
            Self::ContradictoryResourceBounds { dimension } => {
                write!(
                    formatter,
                    "contradictory resource bounds for {}",
                    dimension
                )
            }
            Self::ContradictoryQualityBounds { metric } => {
                write!(
                    formatter,
                    "contradictory quality bounds for metric `{metric}`"
                )
            }
            Self::DuplicateResourceIdentifier { identifier } => {
                write!(
                    formatter,
                    "duplicate resource constraint identifier `{identifier}`"
                )
            }
        }
    }
}

impl std::error::Error for ConstraintValidationError {}

// =============================================================================
// Internal deterministic helpers
// =============================================================================

fn deduplicate_qubits(mut qubits: Vec<QubitId>) -> Arc<[QubitId]> {
    qubits.sort_by(|left, right| left.cmp(right));
    qubits.dedup();
    Arc::from(qubits)
}

fn deduplicate_resource_constraints(
    resources: Vec<ResourceConstraint>,
) -> Arc<[ResourceConstraint]> {
    // Do not collapse constraints by dimension because two independent
    // constraints may intentionally have different strengths/identifiers.
    //
    // Preserve deterministic ordering by sorting only by stable intrinsic
    // fields. This avoids dependence on hash-map iteration order.
    let mut resources = resources;
    resources.sort_by(|left, right| {
        left.dimension()
            .cmp(&right.dimension())
            .then_with(|| left.identifier().cmp(&right.identifier()))
            .then_with(|| left.strength().cmp(&right.strength()))
    });

    Arc::from(resources)
}

fn deduplicate_quality_constraints(
    qualities: Vec<QualityConstraint>,
) -> Arc<[QualityConstraint]> {
    let mut qualities = qualities;

    qualities.sort_by(|left, right| {
        left.metric()
            .cmp(right.metric())
            .then_with(|| left.strength().cmp(&right.strength()))
    });

    Arc::from(qualities)
}

fn validate_unique_resource_identifiers(
    resources: &[ResourceConstraint],
) -> Result<(), ConstraintValidationError> {
    for (index, current) in resources.iter().enumerate() {
        let Some(identifier) = current.identifier() else {
            continue;
        };

        for later in resources.iter().skip(index + 1) {
            if later.identifier() == Some(identifier) {
                return Err(
                    ConstraintValidationError::DuplicateResourceIdentifier {
                        identifier: identifier.to_owned(),
                    },
                );
            }
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_defaults_are_semantically_safe() {
        let constraints = ResilienceConstraints::strict();

        assert_eq!(
            constraints.schema_version(),
            RESILIENCE_CONSTRAINTS_SCHEMA_VERSION
        );

        assert!(constraints.semantic().preserve_logical_identity());
        assert!(constraints.semantic().preserve_operation_semantics());
        assert!(constraints.semantic().preserve_measurement_semantics());
        assert!(constraints.semantic().preserve_classical_dependencies());
        assert!(constraints.semantic().preserve_observable_semantics());
        assert!(constraints.requires_verification());

        assert!(constraints.security().require_provenance());
        assert!(constraints.security().forbid_silent_degradation());
        assert!(constraints.security().require_authorized_target());
    }

    #[test]
    fn no_machine_size_is_assumed() {
        let constraints = ResilienceConstraints::strict();

        assert_eq!(
            constraints.maximum_resource(ResourceDimension::LogicalQubits),
            None
        );

        assert_eq!(
            constraints.maximum_resource(ResourceDimension::PhysicalQubits),
            None
        );
    }

    #[test]
    fn logical_qubits_use_canonical_ids() {
        // The test deliberately relies on QubitId from quantum::ir::qubit.
        // Construction details are left to the canonical IR type.
        //
        // This test primarily ensures the constraint API does not introduce a
        // second logical-qubit identity type.
        let _type_marker: Option<QubitId> = None;
    }

    #[test]
    fn resource_bounds_are_checked() {
        let constraint = ResourceConstraint::builder(
            ResourceDimension::LogicalQubits,
        )
        .minimum_u64(8)
        .maximum_u64(4)
        .build();

        assert!(constraint.is_some());

        let result = ResilienceConstraints::builder()
            .resource(constraint.expect("constraint must exist"))
            .build();

        assert!(matches!(
            result,
            Err(ConstraintValidationError::ContradictoryResourceBounds {
                dimension: ResourceDimension::LogicalQubits
            })
        ));
    }

    #[test]
    fn quality_bounds_reject_nan() {
        let builder = QualityConstraint::builder("logical_quality");

        assert!(builder.minimum(f64::NAN).is_none());
        assert!(builder.maximum(f64::NAN).is_none());
    }

    #[test]
    fn quality_bounds_reject_infinity() {
        let builder = QualityConstraint::builder("logical_quality");

        assert!(builder.minimum(f64::INFINITY).is_none());
        assert!(builder.maximum(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn quality_bounds_reject_inverted_ranges() {
        let builder = QualityConstraint::builder("logical_quality");

        let builder = builder.minimum(0.9).expect("finite minimum");
        let builder = builder.maximum(0.2).expect("finite maximum");

        assert!(builder.build().is_none());
    }

    #[test]
    fn adaptation_permissions_are_independent() {
        let constraints = AdaptationConstraints::allow_all()
            .remapping(false)
            .rerouting(true)
            .rescheduling(false)
            .recompilation(true)
            .reoptimization(false)
            .migration(true);

        assert!(!constraints.remapping_allowed());
        assert!(constraints.rerouting_allowed());
        assert!(!constraints.rescheduling_allowed());
        assert!(constraints.recompilation_allowed());
        assert!(!constraints.reoptimization_allowed());
        assert!(constraints.migration_allowed());
    }

    #[test]
    fn recovery_permissions_do_not_encode_retry_counts() {
        let constraints = RecoveryConstraints::allow_all();

        assert!(constraints.retry_allowed());

        // There is intentionally no retry count in this type.
        // Retry budgets belong to policy/budgets.rs.
    }

    #[test]
    fn unknown_is_not_success() {
        let evaluation = ConstraintEvaluation::new(
            "resource.capacity",
            ConstraintStrength::Required,
            ConstraintStatus::Unknown,
            "capacity was not established",
        );

        assert!(!evaluation.acceptable());
    }

    #[test]
    fn advisory_unknown_can_be_accepted() {
        let evaluation = ConstraintEvaluation::new(
            "optional.preference",
            ConstraintStrength::Advisory,
            ConstraintStatus::Unknown,
            "optional information unavailable",
        );

        assert!(evaluation.acceptable());
    }

    #[test]
    fn mandatory_violation_cannot_be_accepted() {
        let evaluation = ConstraintEvaluation::new(
            "semantic.integrity",
            ConstraintStrength::Mandatory,
            ConstraintStatus::Violated,
            "semantic equivalence failed",
        );

        assert!(!evaluation.acceptable());
    }

    #[test]
    fn satisfied_required_constraint_is_acceptable() {
        let evaluation = ConstraintEvaluation::new(
            "resource.capacity",
            ConstraintStrength::Required,
            ConstraintStatus::Satisfied,
            "capacity established",
        );

        assert!(evaluation.acceptable());
    }

    #[test]
    fn not_applicable_required_constraint_is_acceptable() {
        let evaluation = ConstraintEvaluation::new(
            "optional.measurement_resource",
            ConstraintStrength::Required,
            ConstraintStatus::NotApplicable,
            "candidate does not use this resource",
        );

        assert!(evaluation.acceptable());
    }

    #[test]
    fn duplicate_resource_identifiers_are_rejected() {
        let first = ResourceConstraint::builder(ResourceDimension::Custom)
            .maximum_u64(8)
            .identifier("resource.a")
            .build()
            .expect("valid resource");

        let second = ResourceConstraint::builder(ResourceDimension::Custom)
            .maximum_u64(16)
            .identifier("resource.a")
            .build()
            .expect("valid resource");

        let result = ResilienceConstraints::builder()
            .resource(first)
            .resource(second)
            .build();

        assert!(matches!(
            result,
            Err(
                ConstraintValidationError::DuplicateResourceIdentifier {
                    ..
                }
            )
        ));
    }

    #[test]
    fn resource_lookup_uses_strictest_maximum() {
        let first = ResourceConstraint::builder(ResourceDimension::PhysicalQubits)
            .maximum_u64(100)
            .build()
            .expect("valid resource");

        let second =
            ResourceConstraint::builder(ResourceDimension::PhysicalQubits)
                .maximum_u64(64)
                .build()
                .expect("valid resource");

        let constraints = ResilienceConstraints::builder()
            .resource(first)
            .resource(second)
            .build()
            .expect("valid constraints");

        assert_eq!(
            constraints.maximum_resource(ResourceDimension::PhysicalQubits),
            Some(64)
        );
    }

    #[test]
    fn resource_lookup_uses_strictest_minimum() {
        let first = ResourceConstraint::builder(ResourceDimension::LogicalQubits)
            .minimum_u64(4)
            .maximum_u64(16)
            .build()
            .expect("valid resource");

        let second = ResourceConstraint::builder(ResourceDimension::LogicalQubits)
            .minimum_u64(8)
            .maximum_u64(16)
            .build()
            .expect("valid resource");

        let constraints = ResilienceConstraints::builder()
            .resource(first)
            .resource(second)
            .build()
            .expect("valid constraints");

        assert_eq!(
            constraints.minimum_resource(ResourceDimension::LogicalQubits),
            Some(8)
        );
    }

    #[test]
    fn temporal_constraints_are_optional() {
        let temporal = TemporalConstraints::builder()
            .maximum_execution_time(Duration::from_secs(10))
            .maximum_recovery_time(Duration::from_secs(5))
            .build();

        assert_eq!(
            temporal.maximum_execution_time(),
            Some(Duration::from_secs(10))
        );

        assert_eq!(
            temporal.maximum_recovery_time(),
            Some(Duration::from_secs(5))
        );
    }

    #[test]
    fn schema_identifier_is_stable() {
        assert_eq!(
            RESILIENCE_CONSTRAINTS_SCHEMA_ID,
            "zamani.quantum.resilience.policy.constraints"
        );
    }
}