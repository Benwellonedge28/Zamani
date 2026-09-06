//! Zamani Quantum Resilience — Durable Planner State
//!
//! Path:
//!     src/quantum/resilience/planning/planner_state.rs
//!
//! Purpose:
//!     Provides the immutable/safely-mutable state boundary required to make
//!     resilience planning deterministic, replayable, auditable and scalable.
//!
//! Architectural position:
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
//!         +--> planner_state.rs  <-- this file
//!         +--> planner.rs
//!         |
//!         v
//!     RecoveryPlan
//!         |
//!         v
//!     adaptation / recovery
//!         |
//!         v
//!     verification
//!
//! -----------------------------------------------------------------------------
//! Responsibility
//! -----------------------------------------------------------------------------
//!
//! This file owns:
//!
//! - durable planner state;
//! - deterministic planning epochs;
//! - input/snapshot identity;
//! - plan generation tracking;
//! - active/superseded plan identity;
//! - bounded planning history;
//! - recovery-attempt accounting;
//! - deterministic replay metadata;
//! - state freshness;
//! - state transitions;
//! - explicit caller-supplied capacity;
//! - state integrity metadata;
//! - provider-neutral state references.
//!
//! This file does NOT:
//!
//! - execute recovery;
//! - execute quantum programs;
//! - mutate Quantum IR;
//! - perform routing;
//! - perform scheduling;
//! - perform compilation;
//! - perform optimization;
//! - implement QEC;
//! - implement mitigation;
//! - communicate with hardware;
//! - discover hardware;
//! - contain provider-specific logic;
//! - define QubitId;
//! - define PhysicalQubitId;
//! - persist data to a particular storage backend;
//! - use randomness;
//! - inspect global mutable state.
//!
//! -----------------------------------------------------------------------------
//! Write once, scale everywhere
//! -----------------------------------------------------------------------------
//!
//! The state model contains no quantum-machine-size assumptions.
//!
//! It MUST NOT encode:
//!
//!     MAX_QUBITS = 127
//!     MAX_QUBITS = 1000
//!     MAX_DEVICES = 10
//!     RETRIES = 3
//!
//! Any operational capacity is supplied by the caller.
//!
//! The only bounds represented by this file are caller-selected storage
//! capacities and integer representation limits. These are not quantum
//! architecture limits.
//!
//! A one-qubit system and a very large distributed quantum system therefore
//! use the same state representation.
//!
//! -----------------------------------------------------------------------------
//! Determinism
//! -----------------------------------------------------------------------------
//!
//! Deterministic planning requires explicit state rather than implicit
//! process/global state.
//!
//! This module therefore:
//!
//! - never calls SystemTime::now();
//! - never generates random identifiers;
//! - never reads environment variables;
//! - never reads process-global mutable state;
//! - never depends on HashMap/BTreeMap iteration for semantic ordering;
//! - never uses thread completion order;
//! - never silently wraps integer arithmetic.
//!
//! A deterministic caller supplies:
//!
//! - execution identity;
//! - planning epoch;
//! - input snapshot identity;
//! - policy identity;
//! - capability snapshot identity;
//! - resource snapshot identity;
//! - explicit replay seed/identity where required.
//!
//! -----------------------------------------------------------------------------
//! State versus plan
//! -----------------------------------------------------------------------------
//!
//! A RecoveryPlan is an immutable decision artifact.
//!
//! PlannerState is the durable context in which plans are generated and
//! evaluated.
//!
//! Therefore:
//!
//!     PlannerState != RecoveryPlan
//!
//! State records facts about planning.
//!
//! Plans record proposed actions.
//!
//! Executing a plan MUST NOT mutate a RecoveryPlan.
//!
//! Instead, execution produces a new state observation/event which is applied
//! to PlannerState by the orchestration layer.
//!
//! -----------------------------------------------------------------------------
//! Quantum identity
//! -----------------------------------------------------------------------------
//!
//! This module deliberately does not define quantum identifiers.
//!
//! Whenever a caller needs to associate planner state with quantum resources,
//! it MUST use the canonical repository types:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! where those types are appropriate.
//!
//! Planner state itself stores opaque, stable resource references so that it
//! does not become coupled to the physical representation of a machine.
//!
//! -----------------------------------------------------------------------------
//! Persistence
//! -----------------------------------------------------------------------------
//!
//! Persistence belongs to:
//!
//!     resilience::state::persistence
//!     resilience::checkpoint
//!     resilience::serialization
//!
//! This file therefore provides deterministic state snapshots and restoration
//! contracts without choosing a database, file system, cloud service or
//! serialization format.
//!
//! -----------------------------------------------------------------------------
//! Security
//! -----------------------------------------------------------------------------
//!
//! PlannerState MUST NOT contain:
//!
//! - credentials;
//! - API keys;
//! - passwords;
//! - private keys;
//! - bearer tokens;
//! - authorization headers;
//! - raw device pointers;
//! - memory addresses;
//! - secret provider configuration.
//!
//! External references are opaque stable identifiers.
//!
//! -----------------------------------------------------------------------------
//! Rust compatibility
//! -----------------------------------------------------------------------------
//!
//! - Rust 2021
//! - Rust 1.97
//! - Rust 1.97.1
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::VecDeque;
use std::fmt;
use std::sync::Arc;

// ============================================================================
// Stable schema
// ============================================================================

/// Stable schema identifier for durable planner state.
pub const PLANNER_STATE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.planning.planner-state";

/// Semantic schema version.
///
/// Increment this when externally observable state representation or semantics
/// change in a way that requires compatibility handling.
pub const PLANNER_STATE_SCHEMA_VERSION: u16 = 1;

/// Implementation version.
///
/// This identifies implementation behavior and is independent of the
/// serialization schema version.
pub const PLANNER_STATE_VERSION: u32 = 1;

// ============================================================================
// Opaque stable identifiers
// ============================================================================

/// Stable execution identity.
///
/// The value is supplied by the execution/runtime layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionId(Arc<str>);

impl ExecutionId {
    /// Creates an execution identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, PlannerStateError> {
        let value = value.into();

        validate_identifier("execution identity", value.as_ref())?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ExecutionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable planning-session identity.
///
/// A session may cover multiple planning epochs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlanningSessionId(Arc<str>);

impl PlanningSessionId {
    /// Creates a planning-session identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, PlannerStateError> {
        let value = value.into();

        validate_identifier("planning session identity", value.as_ref())?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for PlanningSessionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable identity for an input/state snapshot.
///
/// The actual snapshot contents are owned by the subsystem that created it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SnapshotId(Arc<str>);

impl SnapshotId {
    /// Creates a snapshot identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, PlannerStateError> {
        let value = value.into();

        validate_identifier("snapshot identity", value.as_ref())?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for SnapshotId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable policy identity.
///
/// Policy contents remain owned by the policy subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PolicyId(Arc<str>);

impl PolicyId {
    /// Creates a policy identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, PlannerStateError> {
        let value = value.into();

        validate_identifier("policy identity", value.as_ref())?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for PolicyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable plan identity.
///
/// The planner owns generation of plan identities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PlanId(Arc<str>);

impl PlanId {
    /// Creates a plan identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, PlannerStateError> {
        let value = value.into();

        validate_identifier("plan identity", value.as_ref())?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for PlanId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable incident identity.
///
/// Incident semantics remain owned by model/incident.rs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IncidentId(Arc<str>);

impl IncidentId {
    /// Creates an incident identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, PlannerStateError> {
        let value = value.into();

        validate_identifier("incident identity", value.as_ref())?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for IncidentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable recovery-attempt identity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AttemptId(Arc<str>);

impl AttemptId {
    /// Creates an attempt identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, PlannerStateError> {
        let value = value.into();

        validate_identifier("attempt identity", value.as_ref())?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for AttemptId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable strategy identity.
///
/// This can refer to a planner/recovery/mitigation strategy without coupling
/// planner state to a concrete implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StrategyId(Arc<str>);

impl StrategyId {
    /// Creates a strategy identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, PlannerStateError> {
        let value = value.into();

        validate_identifier("strategy identity", value.as_ref())?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for StrategyId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable resource identity.
///
/// This intentionally does not encode a physical qubit number or provider
/// representation.
///
/// If a resource is a canonical quantum qubit, the resource's originating
/// subsystem should use `crate::quantum::ir::qubit::QubitId` or
/// `PhysicalQubitId` before converting it to this stable reference.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ResourceRef(Arc<str>);

impl ResourceRef {
    /// Creates an opaque resource reference.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, PlannerStateError> {
        let value = value.into();

        validate_identifier("resource reference", value.as_ref())?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ResourceRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Planning epoch
// ============================================================================

/// Monotonically increasing planning epoch.
///
/// Epochs are supplied by the caller and therefore deterministic.
///
/// The type intentionally does not use timestamps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct PlanningEpoch(u64);

impl PlanningEpoch {
    /// First epoch.
    pub const INITIAL: Self = Self(0);

    /// Creates an epoch.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric epoch.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Advances by one epoch.
    ///
    /// Returns an error instead of wrapping.
    pub const fn checked_next(self) -> Result<Self, PlannerStateError> {
        match self.0.checked_add(1) {
            Some(value) => Ok(Self(value)),
            None => Err(PlannerStateError::ArithmeticOverflow {
                operation: "planning epoch increment",
            }),
        }
    }
}

// ============================================================================
// Plan generation
// ============================================================================

/// Monotonically increasing generation of planner decisions.
///
/// A generation is scoped to a planning session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct PlanGeneration(u64);

impl PlanGeneration {
    /// Initial generation.
    pub const INITIAL: Self = Self(0);

    /// Creates a generation.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the numeric generation.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Advances by one generation.
    pub const fn checked_next(self) -> Result<Self, PlannerStateError> {
        match self.0.checked_add(1) {
            Some(value) => Ok(Self(value)),
            None => Err(PlannerStateError::ArithmeticOverflow {
                operation: "plan generation increment",
            }),
        }
    }
}

// ============================================================================
// State lifecycle
// ============================================================================

/// Lifecycle state of durable planner state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PlannerStateStatus {
    /// State has been initialized.
    Initialized,

    /// State is ready to accept planning observations.
    Ready,

    /// A planning operation is currently represented by the state.
    Planning,

    /// A plan is active.
    Active,

    /// The current state became stale because a material input changed.
    Stale,

    /// The planning session has completed.
    Completed,

    /// Planning was escalated.
    Escalated,

    /// The state has been terminated and must not be reused.
    Terminated,
}

impl PlannerStateStatus {
    /// Returns a stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initialized => "initialized",
            Self::Ready => "ready",
            Self::Planning => "planning",
            Self::Active => "active",
            Self::Stale => "stale",
            Self::Completed => "completed",
            Self::Escalated => "escalated",
            Self::Terminated => "terminated",
        }
    }
}

// ============================================================================
// Plan lifecycle
// ============================================================================

/// Lifecycle status of a plan reference stored by planner state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StoredPlanStatus {
    /// Candidate has been generated.
    Candidate,

    /// Candidate passed required planning validation.
    Validated,

    /// Plan was selected.
    Selected,

    /// Plan execution started.
    Executing,

    /// Execution finished and verification is pending or complete.
    Completed,

    /// Plan became stale.
    Stale,

    /// Plan failed.
    Failed,

    /// Plan was rejected.
    Rejected,

    /// Plan was superseded.
    Superseded,

    /// Plan was cancelled.
    Cancelled,
}

impl StoredPlanStatus {
    /// Returns a stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::Validated => "validated",
            Self::Selected => "selected",
            Self::Executing => "executing",
            Self::Completed => "completed",
            Self::Stale => "stale",
            Self::Failed => "failed",
            Self::Rejected => "rejected",
            Self::Superseded => "superseded",
            Self::Cancelled => "cancelled",
        }
    }
}

// ============================================================================
// Attempt outcome
// ============================================================================

/// Outcome of a recovery/planning attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AttemptOutcome {
    /// Attempt has not completed.
    Pending,

    /// Attempt completed successfully.
    Succeeded,

    /// Attempt failed but another strategy may remain possible.
    Failed,

    /// Attempt was rejected by policy or feasibility.
    Rejected,

    /// Attempt was cancelled.
    Cancelled,

    /// Attempt became stale because inputs changed.
    Stale,

    /// Attempt was escalated.
    Escalated,
}

impl AttemptOutcome {
    /// Returns a stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Rejected => "rejected",
            Self::Cancelled => "cancelled",
            Self::Stale => "stale",
            Self::Escalated => "escalated",
        }
    }
}

// ============================================================================
// State freshness
// ============================================================================

/// Freshness of planner inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StateFreshness {
    /// No material inputs have been changed.
    Fresh,

    /// Some non-material information changed.
    ObservationallyChanged,

    /// A material planning input changed.
    Stale,

    /// Freshness cannot be established.
    Unknown,
}

impl StateFreshness {
    /// Returns whether the state can safely be used as current planning state.
    #[must_use]
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Fresh | Self::ObservationallyChanged)
    }

    /// Returns a stable identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::ObservationallyChanged => "observationally_changed",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        }
    }
}

// ============================================================================
// Snapshot bundle
// ============================================================================

/// Identities of all material planner inputs.
///
/// The planner does not own the contents represented by these identities.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlannerSnapshot {
    /// Canonical workload/program snapshot.
    workload: SnapshotId,

    /// Canonical policy snapshot.
    policy: PolicyId,

    /// Hardware capability snapshot.
    capabilities: SnapshotId,

    /// Resource availability snapshot.
    resources: SnapshotId,

    /// Execution state snapshot.
    execution: SnapshotId,

    /// Diagnostic/incident snapshot.
    diagnosis: SnapshotId,
}

impl PlannerSnapshot {
    /// Creates a complete snapshot bundle.
    pub fn new(
        workload: SnapshotId,
        policy: PolicyId,
        capabilities: SnapshotId,
        resources: SnapshotId,
        execution: SnapshotId,
        diagnosis: SnapshotId,
    ) -> Self {
        Self {
            workload,
            policy,
            capabilities,
            resources,
            execution,
            diagnosis,
        }
    }

    /// Workload snapshot.
    #[must_use]
    pub fn workload(&self) -> &SnapshotId {
        &self.workload
    }

    /// Policy snapshot.
    #[must_use]
    pub fn policy(&self) -> &PolicyId {
        &self.policy
    }

    /// Capability snapshot.
    #[must_use]
    pub fn capabilities(&self) -> &SnapshotId {
        &self.capabilities
    }

    /// Resource snapshot.
    #[must_use]
    pub fn resources(&self) -> &SnapshotId {
        &self.resources
    }

    /// Execution snapshot.
    #[must_use]
    pub fn execution(&self) -> &SnapshotId {
        &self.execution
    }

    /// Diagnosis snapshot.
    #[must_use]
    pub fn diagnosis(&self) -> &SnapshotId {
        &self.diagnosis
    }
}

// ============================================================================
// Plan record
// ============================================================================

/// Durable reference to a plan generated during planning.
///
/// This is deliberately smaller than the complete RecoveryPlan.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoredPlan {
    /// Plan identity.
    id: PlanId,

    /// Generation at which it was produced.
    generation: PlanGeneration,

    /// Planning epoch at which it was produced.
    epoch: PlanningEpoch,

    /// Incident that caused the planning operation, if any.
    incident: Option<IncidentId>,

    /// Strategy selected for the plan, if known.
    strategy: Option<StrategyId>,

    /// Input snapshot used to generate the plan.
    snapshot: PlannerSnapshot,

    /// Current stored lifecycle status.
    status: StoredPlanStatus,
}

impl StoredPlan {
    /// Creates a stored plan reference.
    pub fn new(
        id: PlanId,
        generation: PlanGeneration,
        epoch: PlanningEpoch,
        incident: Option<IncidentId>,
        strategy: Option<StrategyId>,
        snapshot: PlannerSnapshot,
        status: StoredPlanStatus,
    ) -> Self {
        Self {
            id,
            generation,
            epoch,
            incident,
            strategy,
            snapshot,
            status,
        }
    }

    /// Returns the plan identity.
    #[must_use]
    pub fn id(&self) -> &PlanId {
        &self.id
    }

    /// Returns the generation.
    #[must_use]
    pub const fn generation(&self) -> PlanGeneration {
        self.generation
    }

    /// Returns the planning epoch.
    #[must_use]
    pub const fn epoch(&self) -> PlanningEpoch {
        self.epoch
    }

    /// Returns the incident identity.
    #[must_use]
    pub fn incident(&self) -> Option<&IncidentId> {
        self.incident.as_ref()
    }

    /// Returns the strategy identity.
    #[must_use]
    pub fn strategy(&self) -> Option<&StrategyId> {
        self.strategy.as_ref()
    }

    /// Returns the input snapshot.
    #[must_use]
    pub fn snapshot(&self) -> &PlannerSnapshot {
        &self.snapshot
    }

    /// Returns the stored lifecycle status.
    #[must_use]
    pub const fn status(&self) -> StoredPlanStatus {
        self.status
    }

    /// Creates a copy with another lifecycle status.
    #[must_use]
    pub fn with_status(&self, status: StoredPlanStatus) -> Self {
        Self {
            id: self.id.clone(),
            generation: self.generation,
            epoch: self.epoch,
            incident: self.incident.clone(),
            strategy: self.strategy.clone(),
            snapshot: self.snapshot.clone(),
            status,
        }
    }
}

// ============================================================================
// Attempt record
// ============================================================================

/// Durable record of one recovery/planning attempt.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttemptRecord {
    /// Attempt identity.
    id: AttemptId,

    /// Plan associated with the attempt.
    plan: PlanId,

    /// Strategy used.
    strategy: Option<StrategyId>,

    /// Planning epoch.
    epoch: PlanningEpoch,

    /// Outcome.
    outcome: AttemptOutcome,

    /// Optional deterministic reason code.
    reason: Option<Arc<str>>,
}

impl AttemptRecord {
    /// Creates a pending attempt record.
    pub fn new(
        id: AttemptId,
        plan: PlanId,
        strategy: Option<StrategyId>,
        epoch: PlanningEpoch,
    ) -> Self {
        Self {
            id,
            plan,
            strategy,
            epoch,
            outcome: AttemptOutcome::Pending,
            reason: None,
        }
    }

    /// Returns the attempt identity.
    #[must_use]
    pub fn id(&self) -> &AttemptId {
        &self.id
    }

    /// Returns the associated plan.
    #[must_use]
    pub fn plan(&self) -> &PlanId {
        &self.plan
    }

    /// Returns the strategy.
    #[must_use]
    pub fn strategy(&self) -> Option<&StrategyId> {
        self.strategy.as_ref()
    }

    /// Returns the epoch.
    #[must_use]
    pub const fn epoch(&self) -> PlanningEpoch {
        self.epoch
    }

    /// Returns the outcome.
    #[must_use]
    pub const fn outcome(&self) -> AttemptOutcome {
        self.outcome
    }

    /// Returns the optional reason.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }

    /// Produces an updated record with a new outcome.
    pub fn with_outcome(
        &self,
        outcome: AttemptOutcome,
        reason: Option<impl Into<Arc<str>>>,
    ) -> Result<Self, PlannerStateError> {
        let reason = reason.map(Into::into);

        if let Some(value) = &reason {
            validate_identifier("attempt reason", value.as_ref())?;
        }

        Ok(Self {
            id: self.id.clone(),
            plan: self.plan.clone(),
            strategy: self.strategy.clone(),
            epoch: self.epoch,
            outcome,
            reason,
        })
    }
}

// ============================================================================
// State capacity
// ============================================================================

/// Caller-selected storage capacity for durable planner state.
///
/// These values bound stored history, not quantum-machine size.
///
/// Zero means "retain no entries of that category".
///
/// There is intentionally no default finite machine-specific value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlannerStateCapacity {
    /// Maximum stored plan references.
    plans: usize,

    /// Maximum stored attempts.
    attempts: usize,

    /// Maximum stored incidents.
    incidents: usize,
}

impl PlannerStateCapacity {
    /// Creates an explicit state capacity.
    ///
    /// The caller is responsible for choosing values appropriate to its
    /// resource environment.
    #[must_use]
    pub const fn new(plans: usize, attempts: usize, incidents: usize) -> Self {
        Self {
            plans,
            attempts,
            incidents,
        }
    }

    /// Returns the plan-history capacity.
    #[must_use]
    pub const fn plans(self) -> usize {
        self.plans
    }

    /// Returns the attempt-history capacity.
    #[must_use]
    pub const fn attempts(self) -> usize {
        self.attempts
    }

    /// Returns the incident-history capacity.
    #[must_use]
    pub const fn incidents(self) -> usize {
        self.incidents
    }
}

impl Default for PlannerStateCapacity {
    /// Creates an intentionally empty retention policy.
    ///
    /// Production callers should explicitly select a capacity based on their
    /// persistence/retention architecture rather than inheriting an arbitrary
    /// machine-size constant.
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

// ============================================================================
// State integrity
// ============================================================================

/// Deterministic integrity metadata for a planner-state snapshot.
///
/// The digest is opaque here. Hash implementation belongs to the repository's
/// canonical serialization/integrity layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateIntegrity(Arc<str>);

impl StateIntegrity {
    /// Creates integrity metadata from an externally computed digest.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, PlannerStateError> {
        let value = value.into();

        validate_identifier("state integrity value", value.as_ref())?;

        Ok(Self(value))
    }

    /// Returns the integrity representation.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

// ============================================================================
// Planner state
// ============================================================================

/// Durable planner state.
///
/// The structure is deliberately generic and provider-neutral.
///
/// Mutation is performed through checked methods rather than exposing mutable
/// collections. This prevents callers from bypassing invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannerState {
    /// Schema version of this state.
    schema_version: u16,

    /// Implementation version.
    implementation_version: u32,

    /// Execution identity.
    execution_id: ExecutionId,

    /// Planning-session identity.
    session_id: PlanningSessionId,

    /// Current planning epoch.
    epoch: PlanningEpoch,

    /// Current plan generation.
    generation: PlanGeneration,

    /// Current state lifecycle.
    status: PlannerStateStatus,

    /// Current freshness.
    freshness: StateFreshness,

    /// Current material input snapshot.
    snapshot: PlannerSnapshot,

    /// Currently active plan, if one exists.
    active_plan: Option<PlanId>,

    /// Previously active plan, if one was superseded.
    previous_plan: Option<PlanId>,

    /// Bounded plan history.
    plans: VecDeque<StoredPlan>,

    /// Bounded attempt history.
    attempts: VecDeque<AttemptRecord>,

    /// Bounded incident history.
    incidents: VecDeque<IncidentId>,

    /// Caller-selected retention capacities.
    capacity: PlannerStateCapacity,

    /// Optional integrity metadata.
    integrity: Option<StateIntegrity>,
}

impl PlannerState {
    /// Creates a new planner state.
    ///
    /// The state begins at epoch zero and generation zero.
    ///
    /// No machine-size assumption is made.
    pub fn new(
        execution_id: ExecutionId,
        session_id: PlanningSessionId,
        snapshot: PlannerSnapshot,
        capacity: PlannerStateCapacity,
    ) -> Self {
        Self {
            schema_version: PLANNER_STATE_SCHEMA_VERSION,
            implementation_version: PLANNER_STATE_VERSION,
            execution_id,
            session_id,
            epoch: PlanningEpoch::INITIAL,
            generation: PlanGeneration::INITIAL,
            status: PlannerStateStatus::Initialized,
            freshness: StateFreshness::Fresh,
            snapshot,
            active_plan: None,
            previous_plan: None,
            plans: VecDeque::new(),
            attempts: VecDeque::new(),
            incidents: VecDeque::new(),
            capacity,
            integrity: None,
        }
    }

    /// Returns the schema identifier.
    #[must_use]
    pub const fn schema_id() -> &'static str {
        PLANNER_STATE_SCHEMA_ID
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the implementation version.
    #[must_use]
    pub const fn implementation_version(&self) -> u32 {
        self.implementation_version
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the planning-session identity.
    #[must_use]
    pub fn session_id(&self) -> &PlanningSessionId {
        &self.session_id
    }

    /// Returns the current planning epoch.
    #[must_use]
    pub const fn epoch(&self) -> PlanningEpoch {
        self.epoch
    }

    /// Returns the current plan generation.
    #[must_use]
    pub const fn generation(&self) -> PlanGeneration {
        self.generation
    }

    /// Returns the lifecycle status.
    #[must_use]
    pub const fn status(&self) -> PlannerStateStatus {
        self.status
    }

    /// Returns state freshness.
    #[must_use]
    pub const fn freshness(&self) -> StateFreshness {
        self.freshness
    }

    /// Returns whether the current snapshot is usable for planning.
    #[must_use]
    pub const fn is_current(&self) -> bool {
        self.freshness.is_current()
    }

    /// Returns the current input snapshot.
    #[must_use]
    pub fn snapshot(&self) -> &PlannerSnapshot {
        &self.snapshot
    }

    /// Returns the active plan identity.
    #[must_use]
    pub fn active_plan(&self) -> Option<&PlanId> {
        self.active_plan.as_ref()
    }

    /// Returns the previous plan identity.
    #[must_use]
    pub fn previous_plan(&self) -> Option<&PlanId> {
        self.previous_plan.as_ref()
    }

    /// Returns the configured capacity.
    #[must_use]
    pub const fn capacity(&self) -> PlannerStateCapacity {
        self.capacity
    }

    /// Returns the optional integrity metadata.
    #[must_use]
    pub fn integrity(&self) -> Option<&StateIntegrity> {
        self.integrity.as_ref()
    }

    /// Returns the number of retained plans.
    #[must_use]
    pub fn plan_count(&self) -> usize {
        self.plans.len()
    }

    /// Returns the number of retained attempts.
    #[must_use]
    pub fn attempt_count(&self) -> usize {
        self.attempts.len()
    }

    /// Returns the number of retained incidents.
    #[must_use]
    pub fn incident_count(&self) -> usize {
        self.incidents.len()
    }

    /// Returns plans in deterministic oldest-to-newest order.
    #[must_use]
    pub fn plans(&self) -> impl Iterator<Item = &StoredPlan> {
        self.plans.iter()
    }

    /// Returns attempts in deterministic oldest-to-newest order.
    #[must_use]
    pub fn attempts(&self) -> impl Iterator<Item = &AttemptRecord> {
        self.attempts.iter()
    }

    /// Returns incident references in deterministic oldest-to-newest order.
    #[must_use]
    pub fn incidents(&self) -> impl Iterator<Item = &IncidentId> {
        self.incidents.iter()
    }

    /// Moves initialized state to ready.
    pub fn mark_ready(&mut self) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        match self.status {
            PlannerStateStatus::Initialized
            | PlannerStateStatus::Stale
            | PlannerStateStatus::Completed
            | PlannerStateStatus::Escalated
            | PlannerStateStatus::Ready => {
                self.status = PlannerStateStatus::Ready;
                Ok(())
            }

            PlannerStateStatus::Planning | PlannerStateStatus::Active => {
                Err(PlannerStateError::InvalidTransition {
                    from: self.status,
                    to: PlannerStateStatus::Ready,
                })
            }

            PlannerStateStatus::Terminated => {
                Err(PlannerStateError::Terminated)
            }
        }
    }

    /// Begins a planning epoch.
    ///
    /// This advances the epoch exactly once and marks the state as planning.
    pub fn begin_planning(&mut self) -> PlannerStateResult<PlanningEpoch> {
        self.ensure_not_terminated()?;

        if !self.freshness.is_current() {
            return Err(PlannerStateError::StaleState);
        }

        match self.status {
            PlannerStateStatus::Initialized
            | PlannerStateStatus::Ready
            | PlannerStateStatus::Completed
            | PlannerStateStatus::Escalated
            | PlannerStateStatus::Active => {}

            PlannerStateStatus::Planning => {
                return Err(PlannerStateError::AlreadyPlanning);
            }

            PlannerStateStatus::Stale => {
                return Err(PlannerStateError::StaleState);
            }

            PlannerStateStatus::Terminated => {
                return Err(PlannerStateError::Terminated);
            }
        }

        self.epoch = self.epoch.checked_next()?;
        self.generation = self.generation.checked_next()?;
        self.status = PlannerStateStatus::Planning;

        Ok(self.epoch)
    }

    /// Marks the current inputs as observationally changed but still usable.
    pub fn mark_observational_change(&mut self) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        self.freshness = StateFreshness::ObservationallyChanged;

        Ok(())
    }

    /// Marks the state stale.
    ///
    /// Any active plan is also marked stale in the retained plan history.
    pub fn mark_stale(&mut self) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        self.freshness = StateFreshness::Stale;
        self.status = PlannerStateStatus::Stale;

        if let Some(active) = self.active_plan.clone() {
            self.replace_plan_status(&active, StoredPlanStatus::Stale);
        }

        Ok(())
    }

    /// Replaces the material input snapshot.
    ///
    /// A snapshot replacement invalidates the current planning state until
    /// explicitly acknowledged by the caller.
    pub fn replace_snapshot(
        &mut self,
        snapshot: PlannerSnapshot,
    ) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        self.snapshot = snapshot;
        self.freshness = StateFreshness::Stale;
        self.status = PlannerStateStatus::Stale;

        if let Some(active) = self.active_plan.clone() {
            self.replace_plan_status(&active, StoredPlanStatus::Stale);
        }

        Ok(())
    }

    /// Acknowledges that the supplied snapshot is the new current state.
    ///
    /// This operation does not claim that the snapshot is correct; it records
    /// that the upstream orchestration layer has completed its validation.
    pub fn acknowledge_snapshot(
        &mut self,
        snapshot: PlannerSnapshot,
    ) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        self.snapshot = snapshot;
        self.freshness = StateFreshness::Fresh;

        if self.status == PlannerStateStatus::Stale {
            self.status = PlannerStateStatus::Ready;
        }

        Ok(())
    }

    /// Records an incident identity.
    ///
    /// Incidents are retained according to caller-selected capacity.
    pub fn record_incident(
        &mut self,
        incident: IncidentId,
    ) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        if self.capacity.incidents == 0 {
            return Ok(());
        }

        if self.incidents.iter().any(|existing| existing == &incident) {
            return Ok(());
        }

        self.incidents.push_back(incident);

        while self.incidents.len() > self.capacity.incidents {
            self.incidents.pop_front();
        }

        Ok(())
    }

    /// Records a generated plan reference.
    ///
    /// This does not activate the plan.
    pub fn record_plan(
        &mut self,
        plan: StoredPlan,
    ) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        if plan.epoch != self.epoch {
            return Err(PlannerStateError::EpochMismatch {
                expected: self.epoch,
                actual: plan.epoch,
            });
        }

        if plan.generation != self.generation {
            return Err(PlannerStateError::GenerationMismatch {
                expected: self.generation,
                actual: plan.generation,
            });
        }

        if self.capacity.plans == 0 {
            return Ok(());
        }

        if self
            .plans
            .iter()
            .any(|existing| existing.id() == plan.id())
        {
            return Err(PlannerStateError::DuplicatePlan {
                plan: plan.id().clone(),
            });
        }

        self.plans.push_back(plan);

        while self.plans.len() > self.capacity.plans {
            let removed = self.plans.pop_front();

            if let Some(removed) = removed {
                if self
                    .active_plan
                    .as_ref()
                    .is_some_and(|active| active == removed.id())
                {
                    self.active_plan = None;
                }

                if self
                    .previous_plan
                    .as_ref()
                    .is_some_and(|previous| previous == removed.id())
                {
                    self.previous_plan = None;
                }
            }
        }

        Ok(())
    }

    /// Activates an existing stored plan.
    pub fn activate_plan(
        &mut self,
        plan_id: &PlanId,
    ) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        let index = self
            .find_plan_index(plan_id)
            .ok_or_else(|| PlannerStateError::UnknownPlan {
                plan: plan_id.clone(),
            })?;

        let current_status = self.plans[index].status();

        match current_status {
            StoredPlanStatus::Candidate | StoredPlanStatus::Validated => {}

            StoredPlanStatus::Selected | StoredPlanStatus::Executing => {
                return Err(PlannerStateError::PlanAlreadyActive {
                    plan: plan_id.clone(),
                });
            }

            StoredPlanStatus::Completed
            | StoredPlanStatus::Stale
            | StoredPlanStatus::Failed
            | StoredPlanStatus::Rejected
            | StoredPlanStatus::Superseded
            | StoredPlanStatus::Cancelled => {
                return Err(PlannerStateError::InvalidPlanActivation {
                    plan: plan_id.clone(),
                    status: current_status,
                });
            }
        }

        if let Some(previous) = self.active_plan.take() {
            self.previous_plan = Some(previous.clone());
            self.replace_plan_status(&previous, StoredPlanStatus::Superseded);
        }

        self.replace_plan_status(plan_id, StoredPlanStatus::Selected);
        self.active_plan = Some(plan_id.clone());
        self.status = PlannerStateStatus::Active;

        Ok(())
    }

    /// Marks the active plan as executing.
    pub fn mark_plan_executing(&mut self) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        let plan = self
            .active_plan
            .clone()
            .ok_or(PlannerStateError::NoActivePlan)?;

        self.replace_plan_status(&plan, StoredPlanStatus::Executing);

        Ok(())
    }

    /// Completes the active plan.
    pub fn complete_active_plan(&mut self) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        let plan = self
            .active_plan
            .clone()
            .ok_or(PlannerStateError::NoActivePlan)?;

        self.replace_plan_status(&plan, StoredPlanStatus::Completed);
        self.status = PlannerStateStatus::Completed;

        Ok(())
    }

    /// Fails the active plan.
    pub fn fail_active_plan(
        &mut self,
    ) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        let plan = self
            .active_plan
            .clone()
            .ok_or(PlannerStateError::NoActivePlan)?;

        self.replace_plan_status(&plan, StoredPlanStatus::Failed);
        self.status = PlannerStateStatus::Ready;

        Ok(())
    }

    /// Rejects the active plan.
    pub fn reject_active_plan(&mut self) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        let plan = self
            .active_plan
            .clone()
            .ok_or(PlannerStateError::NoActivePlan)?;

        self.replace_plan_status(&plan, StoredPlanStatus::Rejected);
        self.status = PlannerStateStatus::Ready;

        Ok(())
    }

    /// Cancels the active plan.
    pub fn cancel_active_plan(&mut self) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        let plan = self
            .active_plan
            .clone()
            .ok_or(PlannerStateError::NoActivePlan)?;

        self.replace_plan_status(&plan, StoredPlanStatus::Cancelled);
        self.status = PlannerStateStatus::Ready;

        Ok(())
    }

    /// Records a recovery/planning attempt.
    pub fn record_attempt(
        &mut self,
        attempt: AttemptRecord,
    ) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        if attempt.epoch() != self.epoch {
            return Err(PlannerStateError::EpochMismatch {
                expected: self.epoch,
                actual: attempt.epoch(),
            });
        }

        if self.capacity.attempts == 0 {
            return Ok(());
        }

        if self
            .attempts
            .iter()
            .any(|existing| existing.id() == attempt.id())
        {
            return Err(PlannerStateError::DuplicateAttempt {
                attempt: attempt.id().clone(),
            });
        }

        self.attempts.push_back(attempt);

        while self.attempts.len() > self.capacity.attempts {
            self.attempts.pop_front();
        }

        Ok(())
    }

    /// Updates a previously retained attempt.
    pub fn update_attempt(
        &mut self,
        attempt_id: &AttemptId,
        outcome: AttemptOutcome,
        reason: Option<impl Into<Arc<str>>>,
    ) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        let index = self
            .attempts
            .iter()
            .position(|attempt| attempt.id() == attempt_id)
            .ok_or_else(|| PlannerStateError::UnknownAttempt {
                attempt: attempt_id.clone(),
            })?;

        let updated = self.attempts[index]
            .with_outcome(outcome, reason)
            .map_err(PlannerStateError::from)?;

        self.attempts[index] = updated;

        Ok(())
    }

    /// Marks the planner state as escalated.
    pub fn escalate(&mut self) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        self.status = PlannerStateStatus::Escalated;

        Ok(())
    }

    /// Terminates this planner state.
    ///
    /// A terminated state cannot be reused.
    pub fn terminate(&mut self) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        self.status = PlannerStateStatus::Terminated;
        self.active_plan = None;

        Ok(())
    }

    /// Attaches externally calculated integrity metadata.
    ///
    /// The hash/digest calculation belongs to the serialization/integrity
    /// subsystem.
    pub fn set_integrity(
        &mut self,
        integrity: StateIntegrity,
    ) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        self.integrity = Some(integrity);

        Ok(())
    }

    /// Clears integrity metadata after a material state mutation.
    ///
    /// Persistence/integrity code should calculate a new value before durable
    /// storage.
    pub fn clear_integrity(&mut self) -> PlannerStateResult<()> {
        self.ensure_not_terminated()?;

        self.integrity = None;

        Ok(())
    }

    /// Creates a deterministic immutable state snapshot.
    ///
    /// The returned value contains no mutable collection references.
    #[must_use]
    pub fn snapshot_state(&self) -> PlannerStateSnapshot {
        PlannerStateSnapshot {
            schema_version: self.schema_version,
            implementation_version: self.implementation_version,
            execution_id: self.execution_id.clone(),
            session_id: self.session_id.clone(),
            epoch: self.epoch,
            generation: self.generation,
            status: self.status,
            freshness: self.freshness,
            snapshot: self.snapshot.clone(),
            active_plan: self.active_plan.clone(),
            previous_plan: self.previous_plan.clone(),
            plans: self.plans.iter().cloned().collect(),
            attempts: self.attempts.iter().cloned().collect(),
            incidents: self.incidents.iter().cloned().collect(),
            capacity: self.capacity,
            integrity: self.integrity.clone(),
        }
    }

    /// Finds a retained plan by identity.
    #[must_use]
    pub fn find_plan(&self, plan_id: &PlanId) -> Option<&StoredPlan> {
        self.plans.iter().find(|plan| plan.id() == plan_id)
    }

    /// Finds a retained attempt by identity.
    #[must_use]
    pub fn find_attempt(
        &self,
        attempt_id: &AttemptId,
    ) -> Option<&AttemptRecord> {
        self.attempts
            .iter()
            .find(|attempt| attempt.id() == attempt_id)
    }

    /// Returns the number of failed attempts for a specific plan.
    #[must_use]
    pub fn failed_attempts_for(
        &self,
        plan_id: &PlanId,
    ) -> usize {
        self.attempts
            .iter()
            .filter(|attempt| {
                attempt.plan() == plan_id
                    && attempt.outcome() == AttemptOutcome::Failed
            })
            .count()
    }

    /// Returns whether a strategy has already failed for a plan.
    #[must_use]
    pub fn strategy_failed_for(
        &self,
        plan_id: &PlanId,
        strategy: &StrategyId,
    ) -> bool {
        self.attempts.iter().any(|attempt| {
            attempt.plan() == plan_id
                && attempt
                    .strategy()
                    .is_some_and(|candidate| candidate == strategy)
                && attempt.outcome() == AttemptOutcome::Failed
        })
    }

    fn find_plan_index(&self, plan_id: &PlanId) -> Option<usize> {
        self.plans
            .iter()
            .position(|plan| plan.id() == plan_id)
    }

    fn replace_plan_status(
        &mut self,
        plan_id: &PlanId,
        status: StoredPlanStatus,
    ) {
        if let Some(index) = self.find_plan_index(plan_id) {
            let updated = self.plans[index].with_status(status);
            self.plans[index] = updated;
        }
    }

    fn ensure_not_terminated(&self) -> PlannerStateResult<()> {
        if self.status == PlannerStateStatus::Terminated {
            return Err(PlannerStateError::Terminated);
        }

        Ok(())
    }
}

// ============================================================================
// Immutable state snapshot
// ============================================================================

/// Immutable snapshot of PlannerState.
///
/// This is the object that should be passed to deterministic planning/replay
/// code instead of exposing PlannerState's mutation surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannerStateSnapshot {
    schema_version: u16,
    implementation_version: u32,
    execution_id: ExecutionId,
    session_id: PlanningSessionId,
    epoch: PlanningEpoch,
    generation: PlanGeneration,
    status: PlannerStateStatus,
    freshness: StateFreshness,
    snapshot: PlannerSnapshot,
    active_plan: Option<PlanId>,
    previous_plan: Option<PlanId>,
    plans: Vec<StoredPlan>,
    attempts: Vec<AttemptRecord>,
    incidents: Vec<IncidentId>,
    capacity: PlannerStateCapacity,
    integrity: Option<StateIntegrity>,
}

impl PlannerStateSnapshot {
    /// Returns the schema version.
    #[must_use]
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Returns the implementation version.
    #[must_use]
    pub const fn implementation_version(&self) -> u32 {
        self.implementation_version
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the session identity.
    #[must_use]
    pub fn session_id(&self) -> &PlanningSessionId {
        &self.session_id
    }

    /// Returns the current epoch.
    #[must_use]
    pub const fn epoch(&self) -> PlanningEpoch {
        self.epoch
    }

    /// Returns the current generation.
    #[must_use]
    pub const fn generation(&self) -> PlanGeneration {
        self.generation
    }

    /// Returns the lifecycle status.
    #[must_use]
    pub const fn status(&self) -> PlannerStateStatus {
        self.status
    }

    /// Returns the freshness.
    #[must_use]
    pub const fn freshness(&self) -> StateFreshness {
        self.freshness
    }

    /// Returns the current planner snapshot.
    #[must_use]
    pub fn snapshot(&self) -> &PlannerSnapshot {
        &self.snapshot
    }

    /// Returns the active plan.
    #[must_use]
    pub fn active_plan(&self) -> Option<&PlanId> {
        self.active_plan.as_ref()
    }

    /// Returns the previous plan.
    #[must_use]
    pub fn previous_plan(&self) -> Option<&PlanId> {
        self.previous_plan.as_ref()
    }

    /// Returns retained plans.
    #[must_use]
    pub fn plans(&self) -> &[StoredPlan] {
        &self.plans
    }

    /// Returns retained attempts.
    #[must_use]
    pub fn attempts(&self) -> &[AttemptRecord] {
        &self.attempts
    }

    /// Returns retained incidents.
    #[must_use]
    pub fn incidents(&self) -> &[IncidentId] {
        &self.incidents
    }

    /// Returns the configured capacity.
    #[must_use]
    pub const fn capacity(&self) -> PlannerStateCapacity {
        self.capacity
    }

    /// Returns integrity metadata.
    #[must_use]
    pub fn integrity(&self) -> Option<&StateIntegrity> {
        self.integrity.as_ref()
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by PlannerState.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlannerStateError {
    /// An identifier or semantic value is invalid.
    InvalidArgument {
        /// Description of the invalid value.
        message: String,
    },

    /// The state has been terminated.
    Terminated,

    /// A state lifecycle transition is invalid.
    InvalidTransition {
        /// Current state.
        from: PlannerStateStatus,

        /// Requested state.
        to: PlannerStateStatus,
    },

    /// Planning was requested while already planning.
    AlreadyPlanning,

    /// The state is stale and cannot be used for the requested operation.
    StaleState,

    /// A plan refers to a different planning epoch.
    EpochMismatch {
        /// Current state epoch.
        expected: PlanningEpoch,

        /// Supplied epoch.
        actual: PlanningEpoch,
    },

    /// A plan refers to a different plan generation.
    GenerationMismatch {
        /// Current generation.
        expected: PlanGeneration,

        /// Supplied generation.
        actual: PlanGeneration,
    },

    /// The same plan was recorded twice.
    DuplicatePlan {
        /// Duplicate plan.
        plan: PlanId,
    },

    /// The requested plan does not exist in retained state.
    UnknownPlan {
        /// Requested plan.
        plan: PlanId,
    },

    /// A plan cannot be activated from its current status.
    InvalidPlanActivation {
        /// Plan identity.
        plan: PlanId,

        /// Current plan status.
        status: StoredPlanStatus,
    },

    /// Another plan is already active.
    PlanAlreadyActive {
        /// Active plan.
        plan: PlanId,
    },

    /// An operation required an active plan but none exists.
    NoActivePlan,

    /// The same attempt was recorded twice.
    DuplicateAttempt {
        /// Duplicate attempt.
        attempt: AttemptId,
    },

    /// Attempt does not exist.
    UnknownAttempt {
        /// Requested attempt.
        attempt: AttemptId,
    },

    /// Integer arithmetic overflow.
    ArithmeticOverflow {
        /// Operation that overflowed.
        operation: &'static str,
    },
}

impl fmt::Display for PlannerStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument { message } => {
                write!(formatter, "invalid planner-state argument: {message}")
            }

            Self::Terminated => {
                formatter.write_str("planner state is terminated")
            }

            Self::InvalidTransition { from, to } => {
                write!(
                    formatter,
                    "invalid planner-state transition: {} -> {}",
                    from.as_str(),
                    to.as_str()
                )
            }

            Self::AlreadyPlanning => {
                formatter.write_str("planner state is already planning")
            }

            Self::StaleState => {
                formatter.write_str("planner state is stale")
            }

            Self::EpochMismatch { expected, actual } => {
                write!(
                    formatter,
                    "planning epoch mismatch: expected {}, got {}",
                    expected.get(),
                    actual.get()
                )
            }

            Self::GenerationMismatch { expected, actual } => {
                write!(
                    formatter,
                    "plan generation mismatch: expected {}, got {}",
                    expected.get(),
                    actual.get()
                )
            }

            Self::DuplicatePlan { plan } => {
                write!(formatter, "duplicate planner plan `{plan}`")
            }

            Self::UnknownPlan { plan } => {
                write!(formatter, "unknown planner plan `{plan}`")
            }

            Self::InvalidPlanActivation { plan, status } => {
                write!(
                    formatter,
                    "plan `{plan}` cannot be activated from status `{}`",
                    status.as_str()
                )
            }

            Self::PlanAlreadyActive { plan } => {
                write!(formatter, "plan `{plan}` is already active")
            }

            Self::NoActivePlan => {
                formatter.write_str("no active planner plan")
            }

            Self::DuplicateAttempt { attempt } => {
                write!(formatter, "duplicate planner attempt `{attempt}`")
            }

            Self::UnknownAttempt { attempt } => {
                write!(formatter, "unknown planner attempt `{attempt}`")
            }

            Self::ArithmeticOverflow { operation } => {
                write!(
                    formatter,
                    "arithmetic overflow during {operation}"
                )
            }
        }
    }
}

impl std::error::Error for PlannerStateError {}

/// Planner-state result.
pub type PlannerStateResult<T> = Result<T, PlannerStateError>;

// ============================================================================
// Validation helpers
// ============================================================================

fn validate_identifier(
    kind: &'static str,
    value: &str,
) -> PlannerStateResult<()> {
    if value.is_empty() {
        return Err(PlannerStateError::InvalidArgument {
            message: format!("{kind} must not be empty"),
        });
    }

    if value.len() > u32::MAX as usize {
        return Err(PlannerStateError::InvalidArgument {
            message: format!("{kind} is too large"),
        });
    }

    if value.chars().any(char::is_control) {
        return Err(PlannerStateError::InvalidArgument {
            message: format!("{kind} contains control characters"),
        });
    }

    Ok(())
}

impl From<PlannerStateError> for std::io::Error {
    fn from(error: PlannerStateError) -> Self {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            error.to_string(),
        )
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot() -> PlannerSnapshot {
        PlannerSnapshot::new(
            SnapshotId::new("workload-1").expect("valid workload"),
            PolicyId::new("policy-1").expect("valid policy"),
            SnapshotId::new("capabilities-1").expect("valid capabilities"),
            SnapshotId::new("resources-1").expect("valid resources"),
            SnapshotId::new("execution-1").expect("valid execution"),
            SnapshotId::new("diagnosis-1").expect("valid diagnosis"),
        )
    }

    fn state(capacity: PlannerStateCapacity) -> PlannerState {
        PlannerState::new(
            ExecutionId::new("execution-1").expect("valid execution"),
            PlanningSessionId::new("session-1").expect("valid session"),
            snapshot(),
            capacity,
        )
    }

    fn plan(state: &PlannerState, id: &str) -> StoredPlan {
        StoredPlan::new(
            PlanId::new(id).expect("valid plan"),
            state.generation(),
            state.epoch(),
            Some(IncidentId::new("incident-1").expect("valid incident")),
            Some(StrategyId::new("strategy-1").expect("valid strategy")),
            state.snapshot().clone(),
            StoredPlanStatus::Candidate,
        )
    }

    #[test]
    fn identifiers_reject_empty_values() {
        assert!(ExecutionId::new("").is_err());
        assert!(PlanningSessionId::new("").is_err());
        assert!(SnapshotId::new("").is_err());
        assert!(PolicyId::new("").is_err());
        assert!(PlanId::new("").is_err());
        assert!(IncidentId::new("").is_err());
        assert!(AttemptId::new("").is_err());
        assert!(StrategyId::new("").is_err());
        assert!(ResourceRef::new("").is_err());
    }

    #[test]
    fn state_starts_deterministically() {
        let state = state(PlannerStateCapacity::new(8, 8, 8));

        assert_eq!(state.epoch(), PlanningEpoch::INITIAL);
        assert_eq!(state.generation(), PlanGeneration::INITIAL);
        assert_eq!(state.status(), PlannerStateStatus::Initialized);
        assert_eq!(state.freshness(), StateFreshness::Fresh);
        assert_eq!(state.plan_count(), 0);
        assert_eq!(state.attempt_count(), 0);
        assert_eq!(state.incident_count(), 0);
    }

    #[test]
    fn planning_advances_epoch_and_generation() {
        let mut state = state(PlannerStateCapacity::new(8, 8, 8));

        state.mark_ready().expect("ready");
        let epoch = state.begin_planning().expect("planning");

        assert_eq!(epoch, PlanningEpoch::new(1));
        assert_eq!(state.epoch(), PlanningEpoch::new(1));
        assert_eq!(state.generation(), PlanGeneration::new(1));
        assert_eq!(state.status(), PlannerStateStatus::Planning);
    }

    #[test]
    fn duplicate_plans_are_rejected() {
        let mut state = state(PlannerStateCapacity::new(8, 8, 8));

        state.mark_ready().expect("ready");
        state.begin_planning().expect("planning");

        let first = plan(&state, "plan-1");
        let second = plan(&state, "plan-1");

        state.record_plan(first).expect("first plan");
        assert!(state.record_plan(second).is_err());
    }

    #[test]
    fn capacity_is_caller_selected() {
        let mut state = state(PlannerStateCapacity::new(2, 2, 2));

        state.mark_ready().expect("ready");
        state.begin_planning().expect("planning");

        state
            .record_incident(IncidentId::new("incident-1").expect("incident"))
            .expect("record");

        state
            .record_incident(IncidentId::new("incident-2").expect("incident"))
            .expect("record");

        state
            .record_incident(IncidentId::new("incident-3").expect("incident"))
            .expect("record");

        assert_eq!(state.incident_count(), 2);
    }

    #[test]
    fn zero_capacity_does_not_break_planning() {
        let mut state = state(PlannerStateCapacity::new(0, 0, 0));

        state.mark_ready().expect("ready");
        state.begin_planning().expect("planning");

        state
            .record_incident(IncidentId::new("incident-1").expect("incident"))
            .expect("record");

        state
            .record_plan(StoredPlan::new(
                PlanId::new("plan-1").expect("plan"),
                state.generation(),
                state.epoch(),
                None,
                None,
                state.snapshot().clone(),
                StoredPlanStatus::Candidate,
            ))
            .expect("record plan");

        assert_eq!(state.incident_count(), 0);
        assert_eq!(state.plan_count(), 0);
    }

    #[test]
    fn stale_state_invalidates_active_plan() {
        let mut state = state(PlannerStateCapacity::new(8, 8, 8));

        state.mark_ready().expect("ready");
        state.begin_planning().expect("planning");

        let candidate = plan(&state, "plan-1");

        state.record_plan(candidate).expect("record");
        state
            .activate_plan(&PlanId::new("plan-1").expect("plan"))
            .expect("activate");

        state.mark_stale().expect("stale");

        assert_eq!(state.freshness(), StateFreshness::Stale);
        assert_eq!(state.status(), PlannerStateStatus::Stale);

        let stored = state
            .find_plan(&PlanId::new("plan-1").expect("plan"))
            .expect("stored");

        assert_eq!(stored.status(), StoredPlanStatus::Stale);
    }

    #[test]
    fn snapshot_is_immutable_copy() {
        let state = state(PlannerStateCapacity::new(8, 8, 8));

        let snapshot = state.snapshot_state();

        assert_eq!(snapshot.execution_id(), state.execution_id());
        assert_eq!(snapshot.session_id(), state.session_id());
        assert_eq!(snapshot.epoch(), state.epoch());
        assert_eq!(snapshot.generation(), state.generation());
    }

    #[test]
    fn terminated_state_rejects_mutation() {
        let mut state = state(PlannerStateCapacity::new(8, 8, 8));

        state.terminate().expect("terminate");

        assert_eq!(
            state.mark_ready(),
            Err(PlannerStateError::Terminated)
        );
    }

    #[test]
    fn attempt_lifecycle_is_deterministic() {
        let mut state = state(PlannerStateCapacity::new(8, 8, 8));

        state.mark_ready().expect("ready");
        state.begin_planning().expect("planning");

        let plan_id = PlanId::new("plan-1").expect("plan");

        let attempt = AttemptRecord::new(
            AttemptId::new("attempt-1").expect("attempt"),
            plan_id.clone(),
            Some(StrategyId::new("retry").expect("strategy")),
            state.epoch(),
        );

        state.record_attempt(attempt).expect("record");

        assert_eq!(state.attempt_count(), 1);
        assert_eq!(
            state.failed_attempts_for(&plan_id),
            0
        );

        state
            .update_attempt(
                &AttemptId::new("attempt-1").expect("attempt"),
                AttemptOutcome::Failed,
                Some("transient failure"),
            )
            .expect("update");

        assert_eq!(
            state.failed_attempts_for(&plan_id),
            1
        );

        assert!(
            state.strategy_failed_for(
                &plan_id,
                &StrategyId::new("retry").expect("strategy")
            )
        );
    }

    #[test]
    fn plan_activation_supersedes_previous_plan() {
        let mut state = state(PlannerStateCapacity::new(8, 8, 8));

        state.mark_ready().expect("ready");
        state.begin_planning().expect("planning");

        let first = plan(&state, "plan-1");
        let second = plan(&state, "plan-2");

        state.record_plan(first).expect("first");
        state.record_plan(second).expect("second");

        let first_id = PlanId::new("plan-1").expect("first");
        let second_id = PlanId::new("plan-2").expect("second");

        state.activate_plan(&first_id).expect("activate first");
        state.activate_plan(&second_id).expect("activate second");

        assert_eq!(state.active_plan(), Some(&second_id));
        assert_eq!(state.previous_plan(), Some(&first_id));

        assert_eq!(
            state.find_plan(&first_id).expect("first stored").status(),
            StoredPlanStatus::Superseded
        );

        assert_eq!(
            state.find_plan(&second_id).expect("second stored").status(),
            StoredPlanStatus::Selected
        );
    }

    #[test]
    fn epoch_overflow_is_reported() {
        let epoch = PlanningEpoch::new(u64::MAX);

        assert_eq!(
            epoch.checked_next(),
            Err(PlannerStateError::ArithmeticOverflow {
                operation: "planning epoch increment"
            })
        );
    }

    #[test]
    fn generation_overflow_is_reported() {
        let generation = PlanGeneration::new(u64::MAX);

        assert_eq!(
            generation.checked_next(),
            Err(PlannerStateError::ArithmeticOverflow {
                operation: "plan generation increment"
            })
        );
    }
}