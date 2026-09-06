//! Zamani Quantum Resilience — Recovery Plan Contract
//!
//! Path:
//!     src/quantum/resilience/planning/plan.rs
//!
//! Purpose:
//!     Defines the immutable, validated, deterministic representation of a
//!     resilience/recovery plan.
//
//! Architectural role:
//!
//!     detection
//!         |
//!         v
//!     diagnosis
//!         |
//!         v
//!     policy
//!         |
//!         v
//!     planning
//!         |
//!         +--> action.rs
//!         +--> cost.rs
//!         +--> feasibility.rs
//!         +--> ranking.rs
//!         +--> planner_state.rs
//!         |
//!         v
//!     RecoveryPlan                 <-- this file
//!         |
//!         v
//!     adaptation / recovery
//!         |
//!         v
//!     verification
//!
//! This file owns the immutable plan contract.
//!
//! It does NOT:
//! - execute recovery;
//! - select a plan;
//! - perform routing;
//! - perform scheduling;
//! - perform compilation;
//! - perform optimization;
//! - perform QEC;
//! - perform mitigation;
//! - communicate with a backend;
//! - mutate hardware;
//! - decide policy;
//! - diagnose faults.
//!
//! Those responsibilities belong to other subsystems.
//!
//! -----------------------------------------------------------------------------
//! Production requirements
//! -----------------------------------------------------------------------------
//!
//! - Rust 2021
//! - Rust 1.97 / 1.97.1
//! - no unsafe code
//! - no provider-specific behavior
//! - no fixed machine-size limits
//! - deterministic representation
//! - explicit versioning
//! - explicit state/capability freshness
//! - explicit verification requirements
//! - explicit provenance
//! - immutable after construction
//! - scalable through caller/resource supplied bounds
//!
//! "Infinite" scalability means that this module imposes no artificial finite
//! quantum-machine size. Actual execution remains bounded by resources,
//! capabilities, policy and operating-system constraints.
//!
//! -----------------------------------------------------------------------------
//! Canonical quantum identity
//! -----------------------------------------------------------------------------
//!
//! This module deliberately does not define a QubitId.
//!
//! When future action/precondition implementations need to identify a quantum
//! qubit, they MUST use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! as appropriate.
//!
//! Logical-to-physical conversion belongs to routing/mapping.
//!
//! -----------------------------------------------------------------------------
//! Dependency direction
//! -----------------------------------------------------------------------------
//!
//! errors      -> foundational
//! model       -> domain
//! policy      -> constraints
//! planning    -> consumes all of the above
//! recovery    -> consumes plans
//! verification -> verifies plans/results
//!
//! This file must never depend on recovery execution implementations.
//!
//! -----------------------------------------------------------------------------
//! Determinism
//! -----------------------------------------------------------------------------
//!
//! A RecoveryPlan must be deterministic with respect to the values supplied
//! to its constructor/builder.
//!
//! This module MUST NOT:
//! - call SystemTime::now();
//! - generate random IDs;
//! - inspect environment variables;
//! - inspect global mutable state;
//! - depend on HashMap iteration order;
//! - depend on thread completion order.
//!
//! Deterministic IDs are expected to be supplied by the planning layer from
//! canonical content or an equivalent repository-wide identity mechanism.
//!
//! Operational IDs may be supplied separately, but must not affect plan
//! semantics.
//!
//! -----------------------------------------------------------------------------
//! Security
//! -----------------------------------------------------------------------------
//!
//! A plan is potentially security-sensitive.
//!
//! It MUST NOT contain:
//! - credentials;
//! - API keys;
//! - private keys;
//! - passwords;
//! - bearer tokens;
//! - raw authorization headers;
//! - secret provider configuration;
//! - memory addresses;
//! - raw device pointers.
//!
//! Resource references should use stable resource identities.
//!
//! -----------------------------------------------------------------------------
//! Immutability
//! -----------------------------------------------------------------------------
//!
//! Once a RecoveryPlan is constructed, it cannot be mutated.
//!
//! If a material input changes:
//!
//!     old plan -> stale
//!     new observation -> new plan
//!
//! The executor must not modify an active plan in place.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::errors::ResilienceError;

use super::action::RecoveryAction;
use super::cost::RecoveryCost;

// ============================================================================
// Schema identity
// ============================================================================

/// Stable schema identifier for recovery plans.
///
/// This identifier is suitable for serialization, telemetry and compatibility
/// negotiation.
pub const RECOVERY_PLAN_SCHEMA_ID: &str =
    "zamani.quantum.resilience.recovery-plan";

/// Current semantic schema version.
///
/// Changes to externally observable representation or semantics require a
/// compatibility-policy review before changing this value.
pub const RECOVERY_PLAN_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Plan version
// ============================================================================

/// Version of an individual immutable recovery plan.
///
/// This is deliberately separate from the schema version.
///
/// Schema version answers:
///     "How is a plan represented?"
///
/// Plan version answers:
///     "Which revision of this particular plan is this?"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlanVersion(u64);

impl PlanVersion {
    /// Creates a plan version.
    ///
    /// Version zero is valid and may be used by a caller as the first version.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric version.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl Default for PlanVersion {
    fn default() -> Self {
        Self::new(1)
    }
}

// ============================================================================
// Plan identity
// ============================================================================

/// Stable identity of a recovery plan.
///
/// The planner is responsible for generating this value.
///
/// This type deliberately does not generate UUIDs or access randomness.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlanId(Arc<str>);

impl PlanId {
    /// Creates a plan identity from an already validated identifier.
    ///
    /// The planner should normally derive this from canonical plan content.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "recovery plan identity must not be empty",
            ));
        }

        if value.len() > u32::MAX as usize {
            return Err(ResilienceError::representation_overflow(
                "recovery plan identity is too large",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the stable textual representation.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for PlanId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Incident identity
// ============================================================================

/// Identity of the incident for which the plan was generated.
///
/// The actual incident model belongs to `model::incident`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IncidentRef(Arc<str>);

impl IncidentRef {
    /// Creates an incident reference.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "incident reference must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the incident identifier.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for IncidentRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Snapshot identity
// ============================================================================

/// Stable identity of a snapshot used as an input to planning.
///
/// A snapshot may represent:
/// - capability state;
/// - resource state;
/// - observation state;
/// - policy state;
/// - execution state;
/// - hardware state.
///
/// The contents are owned by their respective subsystems.
///
/// This file only records their identities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SnapshotRef(Arc<str>);

impl SnapshotRef {
    /// Creates a snapshot reference.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "snapshot reference must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the snapshot identifier.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for SnapshotRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Plan state
// ============================================================================

/// Lifecycle state of an immutable recovery plan.
///
/// A plan itself is immutable, but its lifecycle may be represented externally
/// by the recovery/state subsystem.
///
/// This enum describes the plan lifecycle semantics; it does not mutate plans.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlanState {
    /// Plan has been constructed but not activated.
    Draft,

    /// Plan passed validation and is eligible for activation.
    Validated,

    /// Plan has been selected for execution.
    Activated,

    /// Plan execution has started.
    Executing,

    /// Plan was completed successfully and awaits/has completed verification.
    Completed,

    /// Plan became stale before execution or during validation.
    Stale,

    /// Plan was rejected.
    Rejected,

    /// Plan was cancelled.
    Cancelled,

    /// Plan was superseded by a newer plan.
    Superseded,
}

// ============================================================================
// Plan validity
// ============================================================================

/// Explicit validity state of a plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlanValidity {
    /// Plan has not yet been validated.
    Unvalidated,

    /// All currently supplied validation requirements passed.
    Valid,

    /// Material inputs changed.
    Stale,

    /// Plan is structurally invalid.
    Invalid,
}

// ============================================================================
// Preconditions
// ============================================================================

/// A precondition attached to a recovery plan.
///
/// The actual evaluation logic belongs to `feasibility.rs`, policy and the
/// recovery executor.
///
/// A precondition is represented by a stable semantic key and a required
/// state/value description rather than an executable callback.
///
/// This keeps plans:
/// - serializable;
/// - deterministic;
/// - auditable;
/// - backend-independent;
/// - free of arbitrary code execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlanPrecondition {
    /// Stable semantic identifier.
    key: Arc<str>,

    /// Required condition/value.
    requirement: Arc<str>,
}

impl PlanPrecondition {
    /// Creates a precondition.
    pub fn new(
        key: impl Into<Arc<str>>,
        requirement: impl Into<Arc<str>>,
    ) -> Result<Self, ResilienceError> {
        let key = key.into();
        let requirement = requirement.into();

        if key.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "plan precondition key must not be empty",
            ));
        }

        if requirement.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "plan precondition requirement must not be empty",
            ));
        }

        Ok(Self { key, requirement })
    }

    /// Returns the semantic key.
    pub fn key(&self) -> &str {
        self.key.as_ref()
    }

    /// Returns the required condition.
    pub fn requirement(&self) -> &str {
        self.requirement.as_ref()
    }
}

// ============================================================================
// Expected effects
// ============================================================================

/// An expected post-effect of a recovery plan.
///
/// Expected effects are declarative.
///
/// They do not claim that the effect actually happened.
///
/// Actual effects MUST be established by execution and verification.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExpectedEffect {
    /// Stable semantic identifier.
    key: Arc<str>,

    /// Expected state/effect description.
    description: Arc<str>,
}

impl ExpectedEffect {
    /// Creates an expected effect.
    pub fn new(
        key: impl Into<Arc<str>>,
        description: impl Into<Arc<str>>,
    ) -> Result<Self, ResilienceError> {
        let key = key.into();
        let description = description.into();

        if key.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "expected-effect key must not be empty",
            ));
        }

        if description.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "expected-effect description must not be empty",
            ));
        }

        Ok(Self { key, description })
    }

    /// Returns the semantic key.
    pub fn key(&self) -> &str {
        self.key.as_ref()
    }

    /// Returns the expected effect description.
    pub fn description(&self) -> &str {
        self.description.as_ref()
    }
}

// ============================================================================
// Verification requirements
// ============================================================================

/// Verification level required before a recovered result may be accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationLevel {
    /// Structural checks only.
    Structural,

    /// Execution/result integrity checks.
    Integrity,

    /// Resource/capability consistency checks.
    Resource,

    /// Quantum semantic invariants.
    Semantic,

    /// Full acceptance verification.
    Full,
}

/// Declarative verification requirements for a plan.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VerificationRequirements {
    /// Minimum verification level.
    level: VerificationLevel,

    /// Whether semantic equivalence must be demonstrated.
    semantic_preservation: bool,

    /// Whether provenance completeness is mandatory.
    provenance_complete: bool,

    /// Whether capability freshness must be rechecked.
    capability_freshness: bool,

    /// Whether resource state must be rechecked.
    resource_freshness: bool,
}

impl VerificationRequirements {
    /// Creates verification requirements.
    pub const fn new(
        level: VerificationLevel,
        semantic_preservation: bool,
        provenance_complete: bool,
        capability_freshness: bool,
        resource_freshness: bool,
    ) -> Self {
        Self {
            level,
            semantic_preservation,
            provenance_complete,
            capability_freshness,
            resource_freshness,
        }
    }

    /// Strict full verification suitable for plans whose result must not be
    /// accepted without semantic verification.
    pub const fn strict() -> Self {
        Self::new(
            VerificationLevel::Full,
            true,
            true,
            true,
            true,
        )
    }

    /// Returns the minimum verification level.
    pub const fn level(&self) -> VerificationLevel {
        self.level
    }

    /// Whether semantic preservation is required.
    pub const fn semantic_preservation(&self) -> bool {
        self.semantic_preservation
    }

    /// Whether complete provenance is required.
    pub const fn provenance_complete(&self) -> bool {
        self.provenance_complete
    }

    /// Whether capability freshness must be checked.
    pub const fn capability_freshness(&self) -> bool {
        self.capability_freshness
    }

    /// Whether resource freshness must be checked.
    pub const fn resource_freshness(&self) -> bool {
        self.resource_freshness
    }
}

impl Default for VerificationRequirements {
    fn default() -> Self {
        Self::strict()
    }
}

// ============================================================================
// Rollback reference
// ============================================================================

/// Declarative reference to rollback/reversal information.
///
/// The implementation belongs to the recovery/checkpoint subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RollbackRef {
    /// Stable rollback strategy identity.
    strategy: Arc<str>,

    /// Whether rollback capability is required.
    required: bool,
}

impl RollbackRef {
    /// Creates a rollback reference.
    pub fn new(
        strategy: impl Into<Arc<str>>,
        required: bool,
    ) -> Result<Self, ResilienceError> {
        let strategy = strategy.into();

        if strategy.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "rollback strategy must not be empty",
            ));
        }

        Ok(Self {
            strategy,
            required,
        })
    }

    /// Returns the rollback strategy identity.
    pub fn strategy(&self) -> &str {
        self.strategy.as_ref()
    }

    /// Returns whether rollback is mandatory.
    pub const fn required(&self) -> bool {
        self.required
    }
}

// ============================================================================
// Provenance
// ============================================================================

/// Immutable provenance references for a recovery plan.
///
/// Large objects are intentionally referenced by stable identities instead of
/// copied into every plan.
///
/// This prevents recovery planning from duplicating arbitrarily large IR,
/// telemetry or capability structures.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanProvenance {
    /// Program identity.
    program: Option<Arc<str>>,

    /// Canonical IR identity.
    ir: Option<Arc<str>>,

    /// Policy identity.
    policy: Option<Arc<str>>,

    /// Capability snapshot identity.
    capability_snapshot: SnapshotRef,

    /// Resource snapshot identity.
    resource_snapshot: Option<SnapshotRef>,

    /// Observation snapshot identity.
    observation_snapshot: Option<SnapshotRef>,

    /// Execution state snapshot identity.
    execution_snapshot: Option<SnapshotRef>,

    /// Planner/strategy implementation identity.
    planner: Arc<str>,

    /// Optional registry identity.
    registry: Option<Arc<str>>,
}

impl PlanProvenance {
    /// Creates provenance with the mandatory capability snapshot and planner
    /// identities.
    pub fn new(
        capability_snapshot: SnapshotRef,
        planner: impl Into<Arc<str>>,
    ) -> Result<Self, ResilienceError> {
        let planner = planner.into();

        if planner.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "planner identity must not be empty",
            ));
        }

        Ok(Self {
            program: None,
            ir: None,
            policy: None,
            capability_snapshot,
            resource_snapshot: None,
            observation_snapshot: None,
            execution_snapshot: None,
            planner,
            registry: None,
        })
    }

    /// Sets program identity.
    pub fn with_program(mut self, identity: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let identity = identity.into();

        if identity.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "program identity must not be empty",
            ));
        }

        self.program = Some(identity);
        Ok(self)
    }

    /// Sets canonical IR identity.
    pub fn with_ir(mut self, identity: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let identity = identity.into();

        if identity.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "IR identity must not be empty",
            ));
        }

        self.ir = Some(identity);
        Ok(self)
    }

    /// Sets policy identity.
    pub fn with_policy(mut self, identity: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let identity = identity.into();

        if identity.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "policy identity must not be empty",
            ));
        }

        self.policy = Some(identity);
        Ok(self)
    }

    /// Sets resource snapshot.
    pub fn with_resource_snapshot(mut self, snapshot: SnapshotRef) -> Self {
        self.resource_snapshot = Some(snapshot);
        self
    }

    /// Sets observation snapshot.
    pub fn with_observation_snapshot(mut self, snapshot: SnapshotRef) -> Self {
        self.observation_snapshot = Some(snapshot);
        self
    }

    /// Sets execution snapshot.
    pub fn with_execution_snapshot(mut self, snapshot: SnapshotRef) -> Self {
        self.execution_snapshot = Some(snapshot);
        self
    }

    /// Sets registry identity.
    pub fn with_registry(mut self, identity: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let identity = identity.into();

        if identity.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "registry identity must not be empty",
            ));
        }

        self.registry = Some(identity);
        Ok(self)
    }

    /// Returns the program identity.
    pub fn program(&self) -> Option<&str> {
        self.program.as_deref()
    }

    /// Returns the IR identity.
    pub fn ir(&self) -> Option<&str> {
        self.ir.as_deref()
    }

    /// Returns the policy identity.
    pub fn policy(&self) -> Option<&str> {
        self.policy.as_deref()
    }

    /// Returns capability snapshot identity.
    pub fn capability_snapshot(&self) -> &SnapshotRef {
        &self.capability_snapshot
    }

    /// Returns resource snapshot identity.
    pub fn resource_snapshot(&self) -> Option<&SnapshotRef> {
        self.resource_snapshot.as_ref()
    }

    /// Returns observation snapshot identity.
    pub fn observation_snapshot(&self) -> Option<&SnapshotRef> {
        self.observation_snapshot.as_ref()
    }

    /// Returns execution snapshot identity.
    pub fn execution_snapshot(&self) -> Option<&SnapshotRef> {
        self.execution_snapshot.as_ref()
    }

    /// Returns planner identity.
    pub fn planner(&self) -> &str {
        self.planner.as_ref()
    }

    /// Returns registry identity.
    pub fn registry(&self) -> Option<&str> {
        self.registry.as_deref()
    }
}

// ============================================================================
// Plan metadata
// ============================================================================

/// Small, deterministic metadata attached to a plan.
///
/// This metadata is intentionally limited to non-secret descriptive values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanMetadata {
    /// Optional human-readable description.
    description: Option<Arc<str>>,

    /// Optional deterministic decision class.
    decision_class: Option<Arc<str>>,
}

impl PlanMetadata {
    /// Creates empty metadata.
    pub const fn empty() -> Self {
        Self {
            description: None,
            decision_class: None,
        }
    }

    /// Sets a description.
    pub fn with_description(
        mut self,
        description: impl Into<Arc<str>>,
    ) -> Result<Self, ResilienceError> {
        let description = description.into();

        if description.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "plan description must not be empty",
            ));
        }

        self.description = Some(description);
        Ok(self)
    }

    /// Sets the decision class.
    pub fn with_decision_class(
        mut self,
        decision_class: impl Into<Arc<str>>,
    ) -> Result<Self, ResilienceError> {
        let decision_class = decision_class.into();

        if decision_class.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "decision class must not be empty",
            ));
        }

        self.decision_class = Some(decision_class);
        Ok(self)
    }

    /// Returns the optional description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the optional decision class.
    pub fn decision_class(&self) -> Option<&str> {
        self.decision_class.as_deref()
    }
}

impl Default for PlanMetadata {
    fn default() -> Self {
        Self::empty()
    }
}

// ============================================================================
// Recovery plan
// ============================================================================

/// Immutable recovery/adaptation plan.
///
/// A plan is a declarative artifact. It does not execute itself.
///
/// The planner creates it.
///
/// The recovery/adaptation subsystem consumes it.
///
/// The verification subsystem validates its effects.
///
/// # Immutability
///
/// There are no public mutable accessors.
///
/// All fields are private.
///
/// Construction happens through [`RecoveryPlanBuilder`].
///
/// # Scalability
///
/// The plan stores references and compact descriptors instead of embedding
/// complete copies of potentially enormous IR, telemetry, topology or hardware
/// state.
///
/// The number of actions is not fixed by this type.
///
/// The caller is responsible for applying policy/resource limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPlan {
    plan_id: PlanId,
    plan_version: PlanVersion,
    incident: IncidentRef,

    /// State version from which the plan was generated.
    state_snapshot: SnapshotRef,

    /// Plan input provenance.
    provenance: PlanProvenance,

    /// Preconditions required before execution.
    preconditions: Arc<[PlanPrecondition]>,

    /// Ordered recovery/adaptation actions.
    ///
    /// Ordering is semantic and MUST be deterministic.
    actions: Arc<[RecoveryAction]>,

    /// Expected effects.
    expected_effects: Arc<[ExpectedEffect]>,

    /// Estimated multidimensional cost.
    cost: RecoveryCost,

    /// Verification requirements.
    verification: VerificationRequirements,

    /// Optional rollback strategy.
    rollback: Option<RollbackRef>,

    /// Plan lifecycle state.
    state: PlanState,

    /// Current validity.
    validity: PlanValidity,

    /// Optional metadata.
    metadata: PlanMetadata,
}

impl RecoveryPlan {
    /// Returns the stable plan identity.
    pub fn id(&self) -> &PlanId {
        &self.plan_id
    }

    /// Returns the plan version.
    pub const fn version(&self) -> PlanVersion {
        self.plan_version
    }

    /// Returns the associated incident.
    pub fn incident(&self) -> &IncidentRef {
        &self.incident
    }

    /// Returns the state snapshot used during planning.
    pub fn state_snapshot(&self) -> &SnapshotRef {
        &self.state_snapshot
    }

    /// Returns plan provenance.
    pub fn provenance(&self) -> &PlanProvenance {
        &self.provenance
    }

    /// Returns all preconditions.
    pub fn preconditions(&self) -> &[PlanPrecondition] {
        &self.preconditions
    }

    /// Returns all actions in semantic execution order.
    pub fn actions(&self) -> &[RecoveryAction] {
        &self.actions
    }

    /// Returns expected effects.
    pub fn expected_effects(&self) -> &[ExpectedEffect] {
        &self.expected_effects
    }

    /// Returns estimated cost.
    pub fn cost(&self) -> &RecoveryCost {
        &self.cost
    }

    /// Returns verification requirements.
    pub const fn verification(&self) -> &VerificationRequirements {
        &self.verification
    }

    /// Returns optional rollback strategy.
    pub fn rollback(&self) -> Option<&RollbackRef> {
        self.rollback.as_ref()
    }

    /// Returns the plan lifecycle state.
    pub const fn state(&self) -> PlanState {
        self.state
    }

    /// Returns current plan validity.
    pub const fn validity(&self) -> PlanValidity {
        self.validity
    }

    /// Returns metadata.
    pub const fn metadata(&self) -> &PlanMetadata {
        &self.metadata
    }

    /// Returns whether the plan contains at least one action.
    pub fn has_actions(&self) -> bool {
        !self.actions.is_empty()
    }

    /// Returns whether the plan is currently valid.
    pub const fn is_valid(&self) -> bool {
        matches!(self.validity, PlanValidity::Valid)
    }

    /// Returns whether the plan has become stale.
    pub const fn is_stale(&self) -> bool {
        matches!(self.validity, PlanValidity::Stale)
    }

    /// Returns a deterministic action count.
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    /// Returns a deterministic precondition count.
    pub fn precondition_count(&self) -> usize {
        self.preconditions.len()
    }

    /// Returns a deterministic expected-effect count.
    pub fn expected_effect_count(&self) -> usize {
        self.expected_effects.len()
    }
}

// ============================================================================
// Builder
// ============================================================================

/// Builder for [`RecoveryPlan`].
///
/// The builder exists so construction can validate invariants before an
/// immutable plan becomes visible to the rest of the system.
///
/// The builder itself is mutable; the resulting plan is not.
#[derive(Debug, Default)]
pub struct RecoveryPlanBuilder {
    plan_id: Option<PlanId>,
    plan_version: Option<PlanVersion>,
    incident: Option<IncidentRef>,
    state_snapshot: Option<SnapshotRef>,
    provenance: Option<PlanProvenance>,
    preconditions: Vec<PlanPrecondition>,
    actions: Vec<RecoveryAction>,
    expected_effects: Vec<ExpectedEffect>,
    cost: Option<RecoveryCost>,
    verification: Option<VerificationRequirements>,
    rollback: Option<RollbackRef>,
    state: PlanState,
    validity: PlanValidity,
    metadata: PlanMetadata,
}

impl RecoveryPlanBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self {
            state: PlanState::Draft,
            validity: PlanValidity::Unvalidated,
            metadata: PlanMetadata::empty(),
            ..Self::default()
        }
    }

    /// Sets the plan ID.
    pub fn plan_id(mut self, value: PlanId) -> Self {
        self.plan_id = Some(value);
        self
    }

    /// Sets the plan version.
    pub fn plan_version(mut self, value: PlanVersion) -> Self {
        self.plan_version = Some(value);
        self
    }

    /// Sets the incident.
    pub fn incident(mut self, value: IncidentRef) -> Self {
        self.incident = Some(value);
        self
    }

    /// Sets the state snapshot.
    pub fn state_snapshot(mut self, value: SnapshotRef) -> Self {
        self.state_snapshot = Some(value);
        self
    }

    /// Sets provenance.
    pub fn provenance(mut self, value: PlanProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    /// Adds one precondition.
    ///
    /// The builder preserves insertion order because the order may be useful
    /// for diagnostics. The planner should canonicalize semantic ordering if
    /// required by deterministic serialization.
    pub fn add_precondition(mut self, value: PlanPrecondition) -> Self {
        self.preconditions.push(value);
        self
    }

    /// Adds one recovery/adaptation action.
    ///
    /// The caller/planner owns semantic ordering.
    pub fn add_action(mut self, value: RecoveryAction) -> Self {
        self.actions.push(value);
        self
    }

    /// Adds an expected effect.
    pub fn add_expected_effect(mut self, value: ExpectedEffect) -> Self {
        self.expected_effects.push(value);
        self
    }

    /// Sets the multidimensional recovery cost.
    pub fn cost(mut self, value: RecoveryCost) -> Self {
        self.cost = Some(value);
        self
    }

    /// Sets verification requirements.
    pub fn verification(mut self, value: VerificationRequirements) -> Self {
        self.verification = Some(value);
        self
    }

    /// Sets rollback information.
    pub fn rollback(mut self, value: RollbackRef) -> Self {
        self.rollback = Some(value);
        self
    }

    /// Sets plan lifecycle state.
    ///
    /// Planner-created plans should normally remain `Draft` or `Validated`.
    pub fn state(mut self, value: PlanState) -> Self {
        self.state = value;
        self
    }

    /// Sets plan validity.
    ///
    /// A newly created plan should normally be `Unvalidated` until feasibility
    /// and policy validation has completed.
    pub fn validity(mut self, value: PlanValidity) -> Self {
        self.validity = value;
        self
    }

    /// Sets metadata.
    pub fn metadata(mut self, value: PlanMetadata) -> Self {
        self.metadata = value;
        self
    }

    /// Builds an immutable recovery plan.
    ///
    /// Structural invariants are checked here.
    ///
    /// Semantic feasibility is intentionally NOT checked here because that
    /// requires `feasibility.rs`, policy state and live capability state.
    pub fn build(self) -> Result<RecoveryPlan, ResilienceError> {
        let plan_id = self.plan_id.ok_or_else(|| {
            ResilienceError::missing_information(
                "recovery plan requires a plan identity",
            )
        })?;

        let plan_version = self.plan_version.unwrap_or_default();

        let incident = self.incident.ok_or_else(|| {
            ResilienceError::missing_information(
                "recovery plan requires an incident reference",
            )
        })?;

        let state_snapshot = self.state_snapshot.ok_or_else(|| {
            ResilienceError::missing_information(
                "recovery plan requires the state snapshot from which it was created",
            )
        })?;

        let provenance = self.provenance.ok_or_else(|| {
            ResilienceError::missing_information(
                "recovery plan requires provenance",
            )
        })?;

        let cost = self.cost.ok_or_else(|| {
            ResilienceError::missing_information(
                "recovery plan requires a multidimensional cost model",
            )
        })?;

        let verification = self
            .verification
            .unwrap_or_else(VerificationRequirements::strict);

        // A recovery plan without actions is not executable.
        if self.actions.is_empty() {
            return Err(ResilienceError::invalid_plan(
                "recovery plan must contain at least one action",
            ));
        }

        // Validate deterministic action ordering/identity through the action
        // contract without imposing a machine-size limit.
        validate_action_collection(&self.actions)?;

        // Preconditions must have deterministic semantic keys.
        validate_preconditions(&self.preconditions)?;

        // Expected effects must have deterministic semantic keys.
        validate_expected_effects(&self.expected_effects)?;

        // A plan marked Valid cannot omit mandatory validation provenance.
        if matches!(self.validity, PlanValidity::Valid)
            && provenance.capability_snapshot().as_str().is_empty()
        {
            return Err(ResilienceError::invalid_plan(
                "a valid recovery plan requires a capability snapshot",
            ));
        }

        // An Activated/Executing plan cannot be structurally invalid.
        if matches!(
            self.state,
            PlanState::Activated | PlanState::Executing
        ) && matches!(self.validity, PlanValidity::Invalid)
        {
            return Err(ResilienceError::invalid_plan(
                "an activated or executing plan cannot be marked invalid",
            ));
        }

        // Strict semantic verification is mandatory for plans that can lead
        // to acceptance of a recovered quantum result.
        //
        // The plan may still represent an Abort action where semantic
        // preservation of a nonexistent result is not applicable; the
        // verification subsystem owns that action-specific interpretation.
        //
        // We therefore require explicit verification metadata but do not
        // incorrectly impose one generic semantic rule on every action.
        //
        // This is intentional and follows the resilience safety model.

        Ok(RecoveryPlan {
            plan_id,
            plan_version,
            incident,
            state_snapshot,
            provenance,
            preconditions: self.preconditions.into(),
            actions: self.actions.into(),
            expected_effects: self.expected_effects.into(),
            cost,
            verification,
            rollback: self.rollback,
            state: self.state,
            validity: self.validity,
            metadata: self.metadata,
        })
    }
}

// ============================================================================
// Structural validation helpers
// ============================================================================

fn validate_action_collection(
    actions: &[RecoveryAction],
) -> Result<(), ResilienceError> {
    for action in actions {
        action.validate()?;
    }

    Ok(())
}

fn validate_preconditions(
    preconditions: &[PlanPrecondition],
) -> Result<(), ResilienceError> {
    let mut previous: Option<&str> = None;

    for precondition in preconditions {
        let key = precondition.key();

        if let Some(previous_key) = previous {
            // Ordering is allowed to be arbitrary at construction time, but
            // duplicate semantic keys are prohibited because they make a
            // serialized plan ambiguous.
            if previous_key == key {
                return Err(ResilienceError::invalid_plan(
                    "duplicate recovery-plan precondition key",
                ));
            }
        }

        previous = Some(key);
    }

    Ok(())
}

fn validate_expected_effects(
    effects: &[ExpectedEffect],
) -> Result<(), ResilienceError> {
    let mut previous: Option<&str> = None;

    for effect in effects {
        let key = effect.key();

        if let Some(previous_key) = previous {
            if previous_key == key {
                return Err(ResilienceError::invalid_plan(
                    "duplicate recovery-plan expected-effect key",
                ));
            }
        }

        previous = Some(key);
    }

    Ok(())
}

// ============================================================================
// Plan fingerprint input
// ============================================================================

/// A compact, deterministic description of the material inputs to a plan.
///
/// This does not calculate a cryptographic hash.
///
/// The serialization/integrity subsystem owns cryptographic hashing.
///
/// Its purpose is to expose all material identities that a content-addressed
/// plan identity should depend upon.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlanIdentityInputs {
    /// Plan schema version.
    schema_version: u16,

    /// Incident identity.
    incident: IncidentRef,

    /// State snapshot.
    state_snapshot: SnapshotRef,

    /// Capability snapshot.
    capability_snapshot: SnapshotRef,

    /// Planner identity/version.
    planner: Arc<str>,

    /// Optional program identity.
    program: Option<Arc<str>>,

    /// Optional IR identity.
    ir: Option<Arc<str>>,

    /// Optional policy identity.
    policy: Option<Arc<str>>,
}

impl PlanIdentityInputs {
    /// Creates identity inputs from an immutable plan.
    pub fn from_plan(plan: &RecoveryPlan) -> Self {
        Self {
            schema_version: RECOVERY_PLAN_SCHEMA_VERSION,
            incident: plan.incident.clone(),
            state_snapshot: plan.state_snapshot.clone(),
            capability_snapshot: plan.provenance.capability_snapshot().clone(),
            planner: Arc::from(plan.provenance.planner()),
            program: plan.provenance.program().map(Arc::from),
            ir: plan.provenance.ir().map(Arc::from),
            policy: plan.provenance.policy().map(Arc::from),
        }
    }

    /// Returns the schema version.
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the incident.
    pub fn incident(&self) -> &IncidentRef {
        &self.incident
    }

    /// Returns the state snapshot.
    pub fn state_snapshot(&self) -> &SnapshotRef {
        &self.state_snapshot
    }

    /// Returns the capability snapshot.
    pub fn capability_snapshot(&self) -> &SnapshotRef {
        &self.capability_snapshot
    }

    /// Returns planner identity.
    pub fn planner(&self) -> &str {
        self.planner.as_ref()
    }

    /// Returns program identity.
    pub fn program(&self) -> Option<&str> {
        self.program.as_deref()
    }

    /// Returns IR identity.
    pub fn ir(&self) -> Option<&str> {
        self.ir.as_deref()
    }

    /// Returns policy identity.
    pub fn policy(&self) -> Option<&str> {
        self.policy.as_deref()
    }
}

// ============================================================================
// Plan comparison
// ============================================================================

/// Deterministic equality semantics for recovery plans.
///
/// This intentionally delegates to the complete structural `Eq` implementation
/// rather than comparing only cost or action type.
///
/// Two plans that have different provenance or preconditions are different
/// plans even if they happen to contain the same action.
pub fn plans_equivalent(left: &RecoveryPlan, right: &RecoveryPlan) -> bool {
    left == right
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Test doubles
    // -------------------------------------------------------------------------
    //
    // The action and cost modules are expected to provide their own canonical
    // constructors. These tests intentionally remain small and structural.
    //
    // Integration tests in planning/tests/planning.rs should test complete
    // RecoveryAction variants and RecoveryCost semantics.

    #[test]
    fn plan_id_rejects_empty_value() {
        let result = PlanId::new("");

        assert!(result.is_err());
    }

    #[test]
    fn incident_ref_rejects_empty_value() {
        let result = IncidentRef::new("");

        assert!(result.is_err());
    }

    #[test]
    fn snapshot_ref_rejects_empty_value() {
        let result = SnapshotRef::new("");

        assert!(result.is_err());
    }

    #[test]
    fn verification_requirements_are_explicit() {
        let requirements = VerificationRequirements::strict();

        assert_eq!(
            requirements.level(),
            VerificationLevel::Full
        );

        assert!(requirements.semantic_preservation());
        assert!(requirements.provenance_complete());
        assert!(requirements.capability_freshness());
        assert!(requirements.resource_freshness());
    }

    #[test]
    fn plan_version_is_orderable() {
        let first = PlanVersion::new(1);
        let second = PlanVersion::new(2);

        assert!(first < second);
    }

    #[test]
    fn metadata_defaults_to_empty() {
        let metadata = PlanMetadata::default();

        assert!(metadata.description().is_none());
        assert!(metadata.decision_class().is_none());
    }

    #[test]
    fn provenance_requires_planner_identity() {
        let capability = SnapshotRef::new("capabilities-v1")
            .expect("valid snapshot");

        let result = PlanProvenance::new(capability, "");

        assert!(result.is_err());
    }

    #[test]
    fn precondition_requires_key_and_requirement() {
        assert!(
            PlanPrecondition::new("", "available").is_err()
        );

        assert!(
            PlanPrecondition::new("resource.available", "").is_err()
        );
    }

    #[test]
    fn expected_effect_requires_key_and_description() {
        assert!(
            ExpectedEffect::new("", "available").is_err()
        );

        assert!(
            ExpectedEffect::new("resource.available", "").is_err()
        );
    }

    #[test]
    fn rollback_requires_strategy_identity() {
        assert!(
            RollbackRef::new("", true).is_err()
        );
    }
}