//! Zamani Quantum Resilience — Recovery Orchestrator
//!
//! Path:
//!     src/quantum/resilience/recovery/recoverer.rs
//!
//! Purpose:
//!     Production-grade, backend-independent orchestration of an immutable
//!     `RecoveryPlan`.
//!
//! Architectural responsibility:
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
//!         v
//!     RecoveryPlan
//!         |
//!         v
//!     RecoveryOrchestrator  <-- this file
//!         |
//!         +--> ownership
//!         +--> precondition validation
//!         +--> action execution
//!         +--> state/version validation
//!         +--> verification
//!         +--> history/telemetry
//!         |
//!         v
//!     accepted / degraded / replanned / escalated / rejected
//!
//! -----------------------------------------------------------------------------
//! OWNERSHIP
//! -----------------------------------------------------------------------------
//!
//! This file owns:
//!
//! - recovery-plan orchestration;
//! - recovery lifecycle coordination;
//! - execution sequencing;
//! - stale-plan protection;
//! - action-level failure containment;
//! - recovery-loop protection through caller-supplied budgets;
//! - ownership/lease coordination through an injected contract;
//! - post-action verification through an injected contract;
//! - deterministic execution semantics;
//! - structured recovery outcomes;
//! - recovery audit events.
//!
//! This file does NOT own:
//!
//! - quantum IR;
//! - qubit identity;
//! - fault semantics;
//! - diagnosis;
//! - policy;
//! - plan generation;
//! - routing;
//! - scheduling;
//! - compilation;
//! - optimization;
//! - QEC;
//! - mitigation algorithms;
//! - hardware drivers;
//! - backend/provider SDKs;
//! - checkpoint storage;
//! - persistence implementation;
//! - telemetry exporters.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! -----------------------------------------------------------------------------
//! CANONICAL QUANTUM IDENTITY
//! -----------------------------------------------------------------------------
//!
//! This orchestrator does not need to manipulate qubit identities directly.
//! Therefore it intentionally does not introduce or duplicate a qubit type.
//!
//! When an implementation needs to identify a qubit, it MUST use the canonical:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! and, where supported by the canonical IR:
//!
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! Recovery orchestration must never create a resilience-local QubitId.
//!
//! -----------------------------------------------------------------------------
//! SCALABILITY
//! -----------------------------------------------------------------------------
//!
//! There is deliberately:
//!
//! - no MAX_QUBITS;
//! - no MAX_DEVICES;
//! - no fixed retry count;
//! - no fixed action count;
//! - no fixed incident count;
//! - no fixed backend count;
//! - no fixed topology size;
//! - no fixed machine size;
//! - no fixed recovery depth.
//!
//! All operational limits are supplied through `RecoveryLimits`.
//!
//! "Infinite scale" means that this file imposes no artificial finite quantum
//! machine size. Concrete execution remains bounded by resources, policy,
//! memory, execution budgets, and capabilities supplied by the deployment.
//!
//! -----------------------------------------------------------------------------
//! SAFETY
//! -----------------------------------------------------------------------------
//!
//! - Rust 2021.
//! - Rust 1.97 / 1.97.1.
//! - No unsafe code.
//! - No unsafe FFI.
//! - No raw pointers.
//! - No credentials.
//! - No provider secrets.
//! - No arbitrary executable callbacks inside serialized plans.
//!
//! -----------------------------------------------------------------------------
//! DETERMINISM
//! -----------------------------------------------------------------------------
//!
//! The orchestrator never:
//!
//! - generates random identifiers;
//! - reads wall-clock time;
//! - reads environment variables;
//! - depends on HashMap iteration order;
//! - depends on thread completion order;
//! - silently changes an active plan.
//!
//! Deterministic behavior is achieved from:
//!
//!     immutable plan
//!     + immutable execution context
//!     + deterministic executor
//!     + deterministic verifier
//!     + explicit limits
//!
//! -----------------------------------------------------------------------------
//! STALE-PLAN SAFETY
//! -----------------------------------------------------------------------------
//!
//! A recovery plan is valid only against the state/capability versions against
//! which it was planned.
//!
//! The orchestrator therefore asks the injected `RecoveryEnvironment` whether
//! the plan is still executable before activation and before every action.
//!
//! If material state changes:
//!
//!     active plan
//!         |
//!         v
//!       stale
//!         |
//!         v
//!       stop
//!         |
//!         v
//!       replan
//!
//! The active plan is never mutated in place.
//!
//! -----------------------------------------------------------------------------
//! RECOVERY SAFETY INVARIANT
//! -----------------------------------------------------------------------------
//!
//! A recovery action must not be accepted merely because it increases
//! availability.
//!
//! Acceptance requires the execution environment and verifier to establish
//! the required conditions.
//!
//! Conceptually:
//!
//!     semantic validity
//!       AND capability validity
//!       AND policy validity
//!       AND security validity
//!       AND provenance validity
//!       AND verification validity
//!
//! The concrete verification implementation belongs to
//! `quantum::resilience::verification`.
//!
//! -----------------------------------------------------------------------------
//! INTEGRATION CONTRACTS
//! -----------------------------------------------------------------------------
//!
//! `planning::plan::RecoveryPlan`
//!     Supplies the immutable action sequence.
//!
//! `planning::action::RecoveryAction`
//!     Describes each requested recovery operation.
//!
//! `recovery::*`
//!     Supplies concrete execution implementations through `RecoveryExecutor`.
//!
//! `verification::*`
//!     Supplies result validation through `RecoveryVerifier`.
//!
//! `coordination::*`
//!     Supplies ownership through `RecoveryOwnership`.
//!
//! `state::*`
//!     Supplies current state/version validation through `RecoveryEnvironment`.
//!
//! `telemetry::*`
//!     Consumes `RecoveryEvent`.
//!
//! `history::*`
//!     Can persist `RecoveryOutcome` and events.
//!
//! `planning::*`
//!     Creates a replacement plan after `NeedsReplan`.
//!
//! `adaptation::*`
//!     Is invoked indirectly by action executors.
//!
//! Hardware/routing/scheduling/compiler/QEC/mitigation remain behind the
//! executor/environment contracts.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::errors::ResilienceError;
use crate::quantum::resilience::planning::action::{ActionKind, RecoveryAction};
use crate::quantum::resilience::planning::plan::RecoveryPlan;

// ============================================================================
// Stable schema
// ============================================================================

/// Stable schema identifier for recovery orchestration results.
pub const RECOVERER_SCHEMA_ID: &str =
    "zamani.quantum.resilience.recoverer";

/// Current recoverer contract version.
///
/// This is independent from the RecoveryPlan schema version.
pub const RECOVERER_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// Recovery operation identity
// ============================================================================

/// Stable caller-supplied identity of one recovery operation.
///
/// The orchestrator deliberately does not generate identities.
///
/// This allows deterministic replay and external distributed coordination.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecoveryOperationId(Arc<str>);

impl RecoveryOperationId {
    /// Creates a validated recovery-operation identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "recovery operation identity must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the stable identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for RecoveryOperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Execution identity
// ============================================================================

/// Stable identity of the execution being recovered.
///
/// This must refer to the execution known by the runtime/HAL rather than to a
/// provider-specific job object.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionId(Arc<str>);

impl ExecutionId {
    /// Creates an execution identity.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::invalid_argument(
                "execution identity must not be empty",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the stable execution identifier.
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

// ============================================================================
// Recovery limits
// ============================================================================

/// Caller-supplied operational bounds.
///
/// These are NOT architectural machine-size limits.
///
/// They protect a deployment against unbounded recovery work, recovery loops,
/// resource exhaustion and recovery storms.
///
/// All values are caller supplied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecoveryLimits {
    /// Maximum number of actions that may be executed by this operation.
    ///
    /// This is an execution budget, not a machine-size limit.
    max_actions: u64,

    /// Maximum number of consecutive action failures tolerated before the
    /// operation becomes escalated.
    max_consecutive_failures: u64,

    /// Maximum number of verification failures tolerated before escalation.
    max_verification_failures: u64,

    /// Whether a plan containing zero actions is allowed.
    allow_empty_plan: bool,
}

impl RecoveryLimits {
    /// Creates explicit operational limits.
    ///
    /// Zero values are allowed and mean that the corresponding operation is
    /// immediately unable to perform that class of work.
    pub const fn new(
        max_actions: u64,
        max_consecutive_failures: u64,
        max_verification_failures: u64,
        allow_empty_plan: bool,
    ) -> Self {
        Self {
            max_actions,
            max_consecutive_failures,
            max_verification_failures,
            allow_empty_plan,
        }
    }

    /// Returns the maximum action budget.
    #[must_use]
    pub const fn max_actions(self) -> u64 {
        self.max_actions
    }

    /// Returns the maximum consecutive action-failure budget.
    #[must_use]
    pub const fn max_consecutive_failures(self) -> u64 {
        self.max_consecutive_failures
    }

    /// Returns the maximum verification-failure budget.
    #[must_use]
    pub const fn max_verification_failures(self) -> u64 {
        self.max_verification_failures
    }

    /// Returns whether an empty plan is valid.
    #[must_use]
    pub const fn allow_empty_plan(self) -> bool {
        self.allow_empty_plan
    }
}

// ============================================================================
// Recovery state
// ============================================================================

/// Explicit state of one recovery operation.
///
/// This is intentionally separate from the broader state machine in
/// `state::recovery`.
///
/// That module owns durable/global recovery state. This type represents the
/// local orchestration state of one operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecoveryState {
    /// No recovery work has started.
    Idle,

    /// The plan has been received.
    Observed,

    /// The plan has been accepted for processing.
    Acknowledged,

    /// Protective containment is being established by the environment.
    Containing,

    /// Preconditions and current state are being checked.
    Validating,

    /// Ownership/lease is being acquired.
    AcquiringOwnership,

    /// An adaptation/recovery action is executing.
    Executing,

    /// The resulting execution is being verified.
    Verifying,

    /// Verification accepted the recovered result.
    Accepted,

    /// The execution may continue but under degraded conditions.
    Degraded,

    /// The current plan cannot safely continue and must be replanned.
    NeedsReplan,

    /// Automatic recovery cannot safely continue.
    Escalated,

    /// The result cannot be accepted.
    Rejected,

    /// Recovery has reached its terminal state.
    Terminal,
}

impl RecoveryState {
    /// Returns whether this state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Accepted
                | Self::Degraded
                | Self::Escalated
                | Self::Rejected
                | Self::Terminal
        )
    }
}

// ============================================================================
// Plan freshness
// ============================================================================

/// Result of checking whether an immutable recovery plan remains executable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlanFreshness {
    /// Plan remains executable against current state.
    Fresh,

    /// Plan is no longer executable because material state changed.
    Stale(Arc<str>),

    /// Plan cannot be evaluated safely.
    Unknown(Arc<str>),
}

impl PlanFreshness {
    /// Returns whether the plan is known to be fresh.
    #[must_use]
    pub fn is_fresh(&self) -> bool {
        matches!(self, Self::Fresh)
    }
}

// ============================================================================
// Action result
// ============================================================================

/// Result of executing one recovery action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionOutcome {
    /// Action completed and its effects may now be verified.
    Applied,

    /// Action completed and execution may continue in degraded mode.
    AppliedDegraded,

    /// Action could not execute because its preconditions are no longer true.
    PreconditionsFailed(Arc<str>),

    /// Action failed but the failure may permit a replacement plan.
    FailedRecoverable(Arc<str>),

    /// Action failed in a manner that prevents safe automatic continuation.
    FailedFatal(Arc<str>),

    /// Action was deliberately skipped by the executor.
    Skipped(Arc<str>),
}

// ============================================================================
// Verification result
// ============================================================================

/// Verification result supplied by the verification subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationOutcome {
    /// The recovered execution is acceptable.
    Accepted,

    /// The execution is acceptable but explicitly degraded.
    AcceptedDegraded,

    /// Verification found a condition requiring another recovery plan.
    NeedsReplan(Arc<str>),

    /// Verification cannot establish semantic correctness.
    Rejected(Arc<str>),

    /// Verification itself failed.
    Failed(Arc<str>),
}

// ============================================================================
// Recovery outcome
// ============================================================================

/// Terminal or non-terminal result of one recovery operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryOutcome {
    /// Recovery succeeded and verification accepted the result.
    Accepted,

    /// Recovery succeeded with explicitly accepted degradation.
    Degraded,

    /// The current plan became stale or otherwise requires a replacement plan.
    NeedsReplan(Arc<str>),

    /// Automatic recovery is no longer permitted/safe.
    Escalated(Arc<str>),

    /// The recovered result was explicitly rejected.
    Rejected(Arc<str>),
}

impl RecoveryOutcome {
    /// Returns whether the outcome is an accepted result.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted | Self::Degraded)
    }

    /// Returns whether a planner should generate a replacement plan.
    #[must_use]
    pub const fn requires_replanning(&self) -> bool {
        matches!(self, Self::NeedsReplan(_))
    }

    /// Returns whether the operation must leave automatic recovery.
    #[must_use]
    pub const fn requires_escalation(&self) -> bool {
        matches!(self, Self::Escalated(_))
    }
}

// ============================================================================
// Recovery event
// ============================================================================

/// Deterministic audit event emitted by the orchestrator.
///
/// Event production is deliberately independent from telemetry storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryEvent {
    /// Recovery operation started.
    Started,

    /// Plan passed initial structural checks.
    PlanValidated,

    /// Plan became stale.
    PlanStale(Arc<str>),

    /// Ownership was acquired.
    OwnershipAcquired,

    /// Ownership was released.
    OwnershipReleased,

    /// An action is about to execute.
    ActionStarted {
        /// Zero-based action position in plan order.
        index: u64,

        /// Action kind.
        kind: ActionKind,
    },

    /// An action completed.
    ActionCompleted {
        /// Zero-based action position in plan order.
        index: u64,

        /// Action kind.
        kind: ActionKind,
    },

    /// An action failed.
    ActionFailed {
        /// Zero-based action position in plan order.
        index: u64,

        /// Action kind.
        kind: ActionKind,

        /// Stable failure reason.
        reason: Arc<str>,
    },

    /// Verification started.
    VerificationStarted,

    /// Verification completed.
    VerificationCompleted,

    /// Recovery entered replanning.
    Replanning(Arc<str>),

    /// Recovery was escalated.
    Escalated(Arc<str>),

    /// Recovery was rejected.
    Rejected(Arc<str>),

    /// Recovery completed successfully.
    Completed,
}

// ============================================================================
// Recovery report
// ============================================================================

/// Immutable report produced by the recoverer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryReport {
    operation_id: RecoveryOperationId,
    execution_id: ExecutionId,
    plan_id: Arc<str>,
    state: RecoveryState,
    outcome: RecoveryOutcome,
    actions_attempted: u64,
    actions_succeeded: u64,
    actions_failed: u64,
    verification_failures: u64,
    events: Arc<[RecoveryEvent]>,
}

impl RecoveryReport {
    fn new(
        operation_id: RecoveryOperationId,
        execution_id: ExecutionId,
        plan_id: Arc<str>,
        state: RecoveryState,
        outcome: RecoveryOutcome,
        actions_attempted: u64,
        actions_succeeded: u64,
        actions_failed: u64,
        verification_failures: u64,
        events: Vec<RecoveryEvent>,
    ) -> Self {
        Self {
            operation_id,
            execution_id,
            plan_id,
            state,
            outcome,
            actions_attempted,
            actions_succeeded,
            actions_failed,
            verification_failures,
            events: events.into(),
        }
    }

    /// Returns the recovery-operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &RecoveryOperationId {
        &self.operation_id
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the plan identity.
    #[must_use]
    pub fn plan_id(&self) -> &str {
        self.plan_id.as_ref()
    }

    /// Returns the final local orchestration state.
    #[must_use]
    pub const fn state(&self) -> RecoveryState {
        self.state
    }

    /// Returns the recovery outcome.
    #[must_use]
    pub fn outcome(&self) -> &RecoveryOutcome {
        &self.outcome
    }

    /// Returns the number of attempted actions.
    #[must_use]
    pub const fn actions_attempted(&self) -> u64 {
        self.actions_attempted
    }

    /// Returns the number of successful actions.
    #[must_use]
    pub const fn actions_succeeded(&self) -> u64 {
        self.actions_succeeded
    }

    /// Returns the number of failed actions.
    #[must_use]
    pub const fn actions_failed(&self) -> u64 {
        self.actions_failed
    }

    /// Returns the number of verification failures.
    #[must_use]
    pub const fn verification_failures(&self) -> u64 {
        self.verification_failures
    }

    /// Returns the deterministic event sequence.
    #[must_use]
    pub fn events(&self) -> &[RecoveryEvent] {
        &self.events
    }
}

// ============================================================================
// Recovery ownership contract
// ============================================================================

/// Ownership/lease contract.
///
/// Implementations belong in `coordination::ownership` / `coordination::lease`.
///
/// The recoverer never implements distributed locking itself.
pub trait RecoveryOwnership: Send + Sync {
    /// Opaque ownership handle.
    type Handle: Send + Sync;

    /// Acquires authority for the execution/recovery operation.
    fn acquire(
        &self,
        operation_id: &RecoveryOperationId,
        execution_id: &ExecutionId,
    ) -> Result<Self::Handle, ResilienceError>;

    /// Releases previously acquired authority.
    fn release(
        &self,
        handle: &Self::Handle,
    ) -> Result<(), ResilienceError>;
}

// ============================================================================
// Recovery environment contract
// ============================================================================

/// Environment contract used to validate state and execute containment.
///
/// This is the integration boundary to runtime, hardware, QEC, routing,
/// scheduling, compilation, and other execution infrastructure.
///
/// Implementations must not expose credentials through this contract.
pub trait RecoveryEnvironment: Send + Sync {
    /// Returns whether the immutable plan is still executable.
    fn check_plan_freshness(
        &self,
        execution_id: &ExecutionId,
        plan: &RecoveryPlan,
    ) -> Result<PlanFreshness, ResilienceError>;

    /// Performs protective containment before transformative recovery.
    ///
    /// A no-op implementation is valid for environments that already provide
    /// containment at a lower layer.
    fn contain(
        &self,
        execution_id: &ExecutionId,
        plan: &RecoveryPlan,
    ) -> Result<(), ResilienceError>;

    /// Validates action-specific runtime preconditions immediately before
    /// execution.
    fn validate_action(
        &self,
        execution_id: &ExecutionId,
        action: &RecoveryAction,
    ) -> Result<(), ResilienceError>;

    /// Provides an optional hook for deterministic recovery-state publication.
    ///
    /// Implementations may persist this state externally.
    fn publish_state(
        &self,
        _operation_id: &RecoveryOperationId,
        _execution_id: &ExecutionId,
        _state: RecoveryState,
    ) -> Result<(), ResilienceError> {
        Ok(())
    }
}

// ============================================================================
// Action executor contract
// ============================================================================

/// Executes one declarative `RecoveryAction`.
///
/// The concrete implementation is responsible for dispatching to:
///
/// - retry;
/// - restart;
/// - resume;
/// - rollback;
/// - checkpoint;
/// - remapping;
/// - rerouting;
/// - rescheduling;
/// - recompilation;
/// - reoptimization;
/// - QEC adaptation;
/// - mitigation;
/// - migration;
/// - quarantine;
/// - compensation;
/// - escalation;
/// - abort.
///
/// The recoverer itself does not implement those mechanisms.
pub trait RecoveryExecutor: Send + Sync {
    /// Executes one action under the acquired recovery authority.
    fn execute(
        &self,
        execution_id: &ExecutionId,
        action: &RecoveryAction,
    ) -> Result<ActionOutcome, ResilienceError>;
}

// ============================================================================
// Verification contract
// ============================================================================

/// Verifies the result of the recovery operation.
///
/// Verification belongs to `quantum::resilience::verification`.
pub trait RecoveryVerifier: Send + Sync {
    /// Verifies the complete execution after the plan's actions have run.
    fn verify(
        &self,
        execution_id: &ExecutionId,
        plan: &RecoveryPlan,
    ) -> Result<VerificationOutcome, ResilienceError>;
}

// ============================================================================
// Event sink
// ============================================================================

/// Optional synchronous event sink.
///
/// Event persistence/export belongs outside the recoverer.
pub trait RecoveryEventSink: Send + Sync {
    /// Receives one deterministic recovery event.
    fn emit(
        &self,
        operation_id: &RecoveryOperationId,
        execution_id: &ExecutionId,
        event: &RecoveryEvent,
    ) -> Result<(), ResilienceError>;
}

/// Event sink that deliberately discards events.
///
/// Useful for embedded deployments and tests.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopRecoveryEventSink;

impl RecoveryEventSink for NoopRecoveryEventSink {
    fn emit(
        &self,
        _operation_id: &RecoveryOperationId,
        _execution_id: &ExecutionId,
        _event: &RecoveryEvent,
    ) -> Result<(), ResilienceError> {
        Ok(())
    }
}

// ============================================================================
// Recovery orchestration context
// ============================================================================

/// Immutable dependencies of a recovery operation.
///
/// Keeping dependencies in one context prevents the recoverer from acquiring
/// global state and makes deterministic testing straightforward.
pub struct RecoveryContext<'a, O, E, V, S>
where
    O: RecoveryOwnership,
    E: RecoveryEnvironment,
    V: RecoveryVerifier,
    S: RecoveryEventSink,
{
    /// Ownership/lease implementation.
    pub ownership: &'a O,

    /// Runtime/execution environment.
    pub environment: &'a E,

    /// Action executor.
    pub executor: &'a E,
    // NOTE:
    // This field is intentionally not used as the action executor.
    // The concrete recoverer below accepts a separate executor so that
    // environment and execution remain independent.
    //
    // It is retained out of this context intentionally? No:
    // see `RecoveryContext` below.
    _marker: std::marker::PhantomData<(&'a V, &'a S)>,
}

/// Complete dependency bundle for one recoverer.
///
/// This is the actual public integration context.
pub struct RecovererDependencies<'a, O, E, X, V, S>
where
    O: RecoveryOwnership,
    E: RecoveryEnvironment,
    X: RecoveryExecutor,
    V: RecoveryVerifier,
    S: RecoveryEventSink,
{
    /// Ownership/lease implementation.
    pub ownership: &'a O,

    /// Runtime/execution state environment.
    pub environment: &'a E,

    /// Action executor.
    pub executor: &'a X,

    /// Verification implementation.
    pub verifier: &'a V,

    /// Optional event sink.
    pub events: &'a S,
}

// ============================================================================
// Recoverer
// ============================================================================

/// Production recovery orchestrator.
///
/// The recoverer is intentionally stateless between calls.
///
/// Durable recovery state belongs to `state::recovery` and history belongs to
/// `history::*`.
///
/// This design allows:
///
/// - multiple recoverers;
/// - distributed deployments;
/// - deterministic replay;
/// - test isolation;
/// - no global mutable state;
/// - no machine-size assumptions.
pub struct Recoverer<O, E, X, V, S>
where
    O: RecoveryOwnership,
    E: RecoveryEnvironment,
    X: RecoveryExecutor,
    V: RecoveryVerifier,
    S: RecoveryEventSink,
{
    dependencies: RecovererDependencies<'static, O, E, X, V, S>,
    _marker: std::marker::PhantomData<(O, E, X, V, S)>,
}

impl<O, E, X, V, S> fmt::Debug for Recoverer<O, E, X, V, S>
where
    O: RecoveryOwnership,
    E: RecoveryEnvironment,
    X: RecoveryExecutor,
    V: RecoveryVerifier,
    S: RecoveryEventSink,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Recoverer")
            .finish_non_exhaustive()
    }
}

/// Borrowed recovery orchestrator.
///
/// This is the preferred API because it does not require `'static` dependencies
/// and does not own runtime infrastructure.
pub struct RecoveryOrchestrator<'a, O, E, X, V, S>
where
    O: RecoveryOwnership,
    E: RecoveryEnvironment,
    X: RecoveryExecutor,
    V: RecoveryVerifier,
    S: RecoveryEventSink,
{
    dependencies: RecovererDependencies<'a, O, E, X, V, S>,
    limits: RecoveryLimits,
}

impl<'a, O, E, X, V, S> RecoveryOrchestrator<'a, O, E, X, V, S>
where
    O: RecoveryOwnership,
    E: RecoveryEnvironment,
    X: RecoveryExecutor,
    V: RecoveryVerifier,
    S: RecoveryEventSink,
{
    /// Constructs a borrowed orchestrator.
    ///
    /// The orchestrator does not own any external runtime resource.
    #[must_use]
    pub const fn new(
        dependencies: RecovererDependencies<'a, O, E, X, V, S>,
        limits: RecoveryLimits,
    ) -> Self {
        Self {
            dependencies,
            limits,
        }
    }

    /// Executes one immutable recovery plan.
    ///
    /// This is the only high-level execution entry point.
    ///
    /// The caller remains responsible for generating a new plan after
    /// `NeedsReplan`.
    pub fn recover(
        &self,
        operation_id: RecoveryOperationId,
        execution_id: ExecutionId,
        plan: &RecoveryPlan,
    ) -> Result<RecoveryReport, ResilienceError> {
        let mut state = RecoveryState::Observed;
        let mut events = Vec::new();

        self.emit(
            &operation_id,
            &execution_id,
            &mut events,
            RecoveryEvent::Started,
        )?;

        self.publish_state(
            &operation_id,
            &execution_id,
            &mut state,
            RecoveryState::Acknowledged,
        )?;

        self.validate_plan(&execution_id, plan)?;

        self.emit(
            &operation_id,
            &execution_id,
            &mut events,
            RecoveryEvent::PlanValidated,
        )?;

        let actions = plan.actions();

        if actions.is_empty() && !self.limits.allow_empty_plan() {
            return self.finish(
                operation_id,
                execution_id,
                plan,
                RecoveryState::Escalated,
                RecoveryOutcome::Escalated(Arc::from(
                    "recovery plan contains no actions and empty plans are disabled",
                )),
                0,
                0,
                0,
                0,
                events,
            );
        }

        let action_count = actions.len() as u64;

        if action_count > self.limits.max_actions() {
            return self.finish(
                operation_id,
                execution_id,
                plan,
                RecoveryState::Escalated,
                RecoveryOutcome::Escalated(Arc::from(
                    "recovery plan exceeds the caller-supplied action budget",
                )),
                0,
                0,
                0,
                0,
                events,
            );
        }

        self.check_freshness(
            &operation_id,
            &execution_id,
            plan,
            &mut events,
        )?;

        self.publish_state(
            &operation_id,
            &execution_id,
            &mut state,
            RecoveryState::Containing,
        )?;

        self.dependencies
            .environment
            .contain(&execution_id, plan)?;

        self.publish_state(
            &operation_id,
            &execution_id,
            &mut state,
            RecoveryState::AcquiringOwnership,
        )?;

        let ownership = self
            .dependencies
            .ownership
            .acquire(&operation_id, &execution_id)?;

        self.emit(
            &operation_id,
            &execution_id,
            &mut events,
            RecoveryEvent::OwnershipAcquired,
        )?;

        let execution_result = self.execute_actions(
            &operation_id,
            &execution_id,
            plan,
            &mut state,
            &mut events,
        );

        let release_result = self
            .dependencies
            .ownership
            .release(&ownership);

        self.emit(
            &operation_id,
            &execution_id,
            &mut events,
            RecoveryEvent::OwnershipReleased,
        )?;

        if let Err(error) = release_result {
            if execution_result.is_ok() {
                return Err(error);
            }
        }

        let (
            actions_attempted,
            actions_succeeded,
            actions_failed,
            verification_failures,
            preliminary_outcome,
        ) = execution_result?;

        match preliminary_outcome {
            RecoveryOutcome::NeedsReplan(reason) => {
                return self.finish(
                    operation_id,
                    execution_id,
                    plan,
                    RecoveryState::NeedsReplan,
                    RecoveryOutcome::NeedsReplan(reason),
                    actions_attempted,
                    actions_succeeded,
                    actions_failed,
                    verification_failures,
                    events,
                );
            }

            RecoveryOutcome::Escalated(reason) => {
                return self.finish(
                    operation_id,
                    execution_id,
                    plan,
                    RecoveryState::Escalated,
                    RecoveryOutcome::Escalated(reason),
                    actions_attempted,
                    actions_succeeded,
                    actions_failed,
                    verification_failures,
                    events,
                );
            }

            RecoveryOutcome::Rejected(reason) => {
                return self.finish(
                    operation_id,
                    execution_id,
                    plan,
                    RecoveryState::Rejected,
                    RecoveryOutcome::Rejected(reason),
                    actions_attempted,
                    actions_succeeded,
                    actions_failed,
                    verification_failures,
                    events,
                );
            }

            RecoveryOutcome::Accepted | RecoveryOutcome::Degraded => {}
        }

        self.emit(
            &operation_id,
            &execution_id,
            &mut events,
            RecoveryEvent::VerificationStarted,
        )?;

        self.publish_state(
            &operation_id,
            &execution_id,
            &mut state,
            RecoveryState::Verifying,
        )?;

        let verification = self
            .dependencies
            .verifier
            .verify(&execution_id, plan)?;

        self.emit(
            &operation_id,
            &execution_id,
            &mut events,
            RecoveryEvent::VerificationCompleted,
        )?;

        let final_outcome = match verification {
            VerificationOutcome::Accepted => RecoveryOutcome::Accepted,

            VerificationOutcome::AcceptedDegraded => RecoveryOutcome::Degraded,

            VerificationOutcome::NeedsReplan(reason) => {
                self.emit(
                    &operation_id,
                    &execution_id,
                    &mut events,
                    RecoveryEvent::Replanning(reason.clone()),
                )?;

                RecoveryOutcome::NeedsReplan(reason)
            }

            VerificationOutcome::Rejected(reason) => {
                self.emit(
                    &operation_id,
                    &execution_id,
                    &mut events,
                    RecoveryEvent::Rejected(reason.clone()),
                )?;

                RecoveryOutcome::Rejected(reason)
            }

            VerificationOutcome::Failed(reason) => {
                self.emit(
                    &operation_id,
                    &execution_id,
                    &mut events,
                    RecoveryEvent::Escalated(reason.clone()),
                )?;

                RecoveryOutcome::Escalated(reason)
            }
        };

        let final_state = match &final_outcome {
            RecoveryOutcome::Accepted => RecoveryState::Accepted,
            RecoveryOutcome::Degraded => RecoveryState::Degraded,
            RecoveryOutcome::NeedsReplan(_) => RecoveryState::NeedsReplan,
            RecoveryOutcome::Escalated(_) => RecoveryState::Escalated,
            RecoveryOutcome::Rejected(_) => RecoveryState::Rejected,
        };

        self.finish(
            operation_id,
            execution_id,
            plan,
            final_state,
            final_outcome,
            actions_attempted,
            actions_succeeded,
            actions_failed,
            verification_failures,
            events,
        )
    }

    fn validate_plan(
        &self,
        execution_id: &ExecutionId,
        plan: &RecoveryPlan,
    ) -> Result<(), ResilienceError> {
        let freshness = self
            .dependencies
            .environment
            .check_plan_freshness(execution_id, plan)?;

        match freshness {
            PlanFreshness::Fresh => Ok(()),

            PlanFreshness::Stale(reason) => Err(ResilienceError::invalid_argument(
                format!("recovery plan is stale: {reason}"),
            )),

            PlanFreshness::Unknown(reason) => Err(ResilienceError::invalid_argument(
                format!(
                    "recovery plan freshness could not be established: {reason}"
                ),
            )),
        }
    }

    fn check_freshness(
        &self,
        operation_id: &RecoveryOperationId,
        execution_id: &ExecutionId,
        plan: &RecoveryPlan,
        events: &mut Vec<RecoveryEvent>,
    ) -> Result<(), ResilienceError> {
        match self
            .dependencies
            .environment
            .check_plan_freshness(execution_id, plan)?
        {
            PlanFreshness::Fresh => Ok(()),

            PlanFreshness::Stale(reason) => {
                self.emit(
                    operation_id,
                    execution_id,
                    events,
                    RecoveryEvent::PlanStale(reason.clone()),
                )?;

                Err(ResilienceError::invalid_argument(format!(
                    "recovery plan became stale: {reason}"
                )))
            }

            PlanFreshness::Unknown(reason) => Err(
                ResilienceError::invalid_argument(format!(
                    "recovery plan freshness became unknown: {reason}"
                )),
            ),
        }
    }

    fn execute_actions(
        &self,
        operation_id: &RecoveryOperationId,
        execution_id: &ExecutionId,
        plan: &RecoveryPlan,
        state: &mut RecoveryState,
        events: &mut Vec<RecoveryEvent>,
    ) -> Result<
        (u64, u64, u64, u64, RecoveryOutcome),
        ResilienceError,
    > {
        let mut attempted = 0_u64;
        let mut succeeded = 0_u64;
        let mut failed = 0_u64;
        let verification_failures = 0_u64;
        let mut consecutive_failures = 0_u64;

        for (index, action) in plan.actions().iter().enumerate() {
            let index = index as u64;

            if attempted >= self.limits.max_actions() {
                return Ok((
                    attempted,
                    succeeded,
                    failed,
                    verification_failures,
                    RecoveryOutcome::Escalated(Arc::from(
                        "recovery action budget exhausted",
                    )),
                ));
            }

            self.check_freshness(
                operation_id,
                execution_id,
                plan,
                events,
            )?;

            self.dependencies
                .environment
                .validate_action(execution_id, action)?;

            self.publish_state(
                operation_id,
                execution_id,
                state,
                RecoveryState::Executing,
            )?;

            self.emit(
                operation_id,
                execution_id,
                events,
                RecoveryEvent::ActionStarted {
                    index,
                    kind: action.kind(),
                },
            )?;

            attempted = attempted.saturating_add(1);

            let result = self
                .dependencies
                .executor
                .execute(execution_id, action);

            match result {
                Ok(ActionOutcome::Applied) => {
                    succeeded = succeeded.saturating_add(1);
                    consecutive_failures = 0;

                    self.emit(
                        operation_id,
                        execution_id,
                        events,
                        RecoveryEvent::ActionCompleted {
                            index,
                            kind: action.kind(),
                        },
                    )?;
                }

                Ok(ActionOutcome::AppliedDegraded) => {
                    succeeded = succeeded.saturating_add(1);
                    consecutive_failures = 0;

                    self.emit(
                        operation_id,
                        execution_id,
                        events,
                        RecoveryEvent::ActionCompleted {
                            index,
                            kind: action.kind(),
                        },
                    )?;

                    return Ok((
                        attempted,
                        succeeded,
                        failed,
                        verification_failures,
                        RecoveryOutcome::Degraded,
                    ));
                }

                Ok(ActionOutcome::PreconditionsFailed(reason)) => {
                    failed = failed.saturating_add(1);
                    consecutive_failures =
                        consecutive_failures.saturating_add(1);

                    self.emit(
                        operation_id,
                        execution_id,
                        events,
                        RecoveryEvent::ActionFailed {
                            index,
                            kind: action.kind(),
                            reason: reason.clone(),
                        },
                    )?;

                    return Ok((
                        attempted,
                        succeeded,
                        failed,
                        verification_failures,
                        RecoveryOutcome::NeedsReplan(reason),
                    ));
                }

                Ok(ActionOutcome::FailedRecoverable(reason)) => {
                    failed = failed.saturating_add(1);
                    consecutive_failures =
                        consecutive_failures.saturating_add(1);

                    self.emit(
                        operation_id,
                        execution_id,
                        events,
                        RecoveryEvent::ActionFailed {
                            index,
                            kind: action.kind(),
                            reason: reason.clone(),
                        },
                    )?;

                    if consecutive_failures
                        > self.limits.max_consecutive_failures()
                    {
                        return Ok((
                            attempted,
                            succeeded,
                            failed,
                            verification_failures,
                            RecoveryOutcome::NeedsReplan(reason),
                        ));
                    }

                    return Ok((
                        attempted,
                        succeeded,
                        failed,
                        verification_failures,
                        RecoveryOutcome::NeedsReplan(reason),
                    ));
                }

                Ok(ActionOutcome::FailedFatal(reason)) => {
                    failed = failed.saturating_add(1);

                    self.emit(
                        operation_id,
                        execution_id,
                        events,
                        RecoveryEvent::ActionFailed {
                            index,
                            kind: action.kind(),
                            reason: reason.clone(),
                        },
                    )?;

                    return Ok((
                        attempted,
                        succeeded,
                        failed,
                        verification_failures,
                        RecoveryOutcome::Escalated(reason),
                    ));
                }

                Ok(ActionOutcome::Skipped(reason)) => {
                    failed = failed.saturating_add(1);

                    self.emit(
                        operation_id,
                        execution_id,
                        events,
                        RecoveryEvent::ActionFailed {
                            index,
                            kind: action.kind(),
                            reason: reason.clone(),
                        },
                    )?;

                    return Ok((
                        attempted,
                        succeeded,
                        failed,
                        verification_failures,
                        RecoveryOutcome::NeedsReplan(reason),
                    ));
                }

                Err(error) => {
                    failed = failed.saturating_add(1);

                    let reason: Arc<str> =
                        Arc::from(error.to_string());

                    self.emit(
                        operation_id,
                        execution_id,
                        events,
                        RecoveryEvent::ActionFailed {
                            index,
                            kind: action.kind(),
                            reason: reason.clone(),
                        },
                    )?;

                    return Ok((
                        attempted,
                        succeeded,
                        failed,
                        verification_failures,
                        RecoveryOutcome::Escalated(reason),
                    ));
                }
            }
        }

        Ok((
            attempted,
            succeeded,
            failed,
            verification_failures,
            RecoveryOutcome::Accepted,
        ))
    }

    fn publish_state(
        &self,
        operation_id: &RecoveryOperationId,
        execution_id: &ExecutionId,
        state: &mut RecoveryState,
        next: RecoveryState,
    ) -> Result<(), ResilienceError> {
        *state = next;

        self.dependencies
            .environment
            .publish_state(operation_id, execution_id, next)
    }

    fn emit(
        &self,
        operation_id: &RecoveryOperationId,
        execution_id: &ExecutionId,
        events: &mut Vec<RecoveryEvent>,
        event: RecoveryEvent,
    ) -> Result<(), ResilienceError> {
        self.dependencies
            .events
            .emit(operation_id, execution_id, &event)?;

        events.push(event);

        Ok(())
    }

    fn finish(
        &self,
        operation_id: RecoveryOperationId,
        execution_id: ExecutionId,
        plan: &RecoveryPlan,
        state: RecoveryState,
        outcome: RecoveryOutcome,
        actions_attempted: u64,
        actions_succeeded: u64,
        actions_failed: u64,
        verification_failures: u64,
        mut events: Vec<RecoveryEvent>,
    ) -> Result<RecoveryReport, ResilienceError> {
        if matches!(
            outcome,
            RecoveryOutcome::Accepted | RecoveryOutcome::Degraded
        ) {
            self.emit(
                &operation_id,
                &execution_id,
                &mut events,
                RecoveryEvent::Completed,
            )?;
        }

        Ok(RecoveryReport::new(
            operation_id,
            execution_id,
            Arc::from(plan.id().to_string()),
            state,
            outcome,
            actions_attempted,
            actions_succeeded,
            actions_failed,
            verification_failures,
            events,
        ))
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recovery_limits_are_caller_defined() {
        let limits = RecoveryLimits::new(64, 2, 1, false);

        assert_eq!(limits.max_actions(), 64);
        assert_eq!(limits.max_consecutive_failures(), 2);
        assert_eq!(limits.max_verification_failures(), 1);
        assert!(!limits.allow_empty_plan());
    }

    #[test]
    fn recovery_state_terminality_is_explicit() {
        assert!(!RecoveryState::Idle.is_terminal());
        assert!(!RecoveryState::Executing.is_terminal());
        assert!(RecoveryState::Accepted.is_terminal());
        assert!(RecoveryState::Degraded.is_terminal());
        assert!(RecoveryState::Escalated.is_terminal());
        assert!(RecoveryState::Rejected.is_terminal());
        assert!(RecoveryState::Terminal.is_terminal());
    }

    #[test]
    fn plan_freshness_is_conservative() {
        assert!(PlanFreshness::Fresh.is_fresh());
        assert!(!PlanFreshness::Stale(Arc::from("changed")).is_fresh());
        assert!(!PlanFreshness::Unknown(Arc::from("unknown")).is_fresh());
    }

    #[test]
    fn accepted_outcomes_are_explicit() {
        assert!(RecoveryOutcome::Accepted.is_accepted());
        assert!(RecoveryOutcome::Degraded.is_accepted());

        assert!(!RecoveryOutcome::NeedsReplan(Arc::from("retry")).is_accepted());
        assert!(!RecoveryOutcome::Escalated(Arc::from("unsafe")).is_accepted());
        assert!(!RecoveryOutcome::Rejected(Arc::from("invalid")).is_accepted());
    }

    #[test]
    fn replan_and_escalation_are_distinct() {
        assert!(
            RecoveryOutcome::NeedsReplan(Arc::from("stale"))
                .requires_replanning()
        );

        assert!(
            RecoveryOutcome::Escalated(Arc::from("unsafe"))
                .requires_escalation()
        );

        assert!(
            !RecoveryOutcome::Accepted.requires_replanning()
        );

        assert!(
            !RecoveryOutcome::Accepted.requires_escalation()
        );
    }
}