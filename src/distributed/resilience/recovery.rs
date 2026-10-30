
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # Zamani Distributed Resilience — Recovery
//!
//! Path: src/distributed/resilience/recovery.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provides provider-independent recovery orchestration for distributed
//! Zamani computations.
//!
//! Recovery coordinates the transition from a failed or uncertain
//! execution to a verified, semantically valid execution state.
//!
//! It does not implement hardware recovery, quantum error correction,
//! routing, scheduling, checkpoint serialization, or execution itself.
//!
//! # Architectural ownership
//!
//! This module owns:
//! - Recovery incidents.
//! - Recovery plans.
//! - Recovery action identities.
//! - Recovery lifecycle states.
//! - Recovery journal contracts.
//! - Recovery execution contracts.
//! - Recovery verification contracts.
//! - Recovery orchestration.
//! - Optimistic concurrency and stale-plan protection.
//!
//! This module does not own:
//! - Distributed semantic analysis.
//! - Source AST or compiler IR.
//! - ZUIR, ZCIR, quantum::ir, or HDLIR.
//! - ZQN fault classification.
//! - Quantum error correction.
//! - Physical hardware discovery.
//! - Device calibration.
//! - Physical qubit allocation.
//! - Routing or scheduling.
//! - Transport implementations.
//! - Checkpoint encoding or storage engines.
//! - Authentication implementations.
//! - Authorization policy definitions.
//! - Backend compilation.
//! - Domain-specific execution.
//!
//! # Fundamental recovery invariant
//!
//! Recovery is not complete merely because a recovery action executed.
//!
//! The successful lifecycle is:
//!
//! Incident
//!    -> Planning
//!    -> PlanValidated
//!    -> Recovering
//!    -> Verifying
//!    -> Succeeded
//!
//! A recovery operation MUST NOT be accepted before verification.
//!
//! # Crash consistency
//!
//! Every action is recorded as Started before being submitted to an
//! execution provider.
//!
//! The action identity remains stable across recovery of the recovery
//! process itself.
//!
//! An execution provider MUST deduplicate repeated submissions of the
//! same recovery action identity, or provide an equivalent authoritative
//! reconciliation mechanism.
//!
//! A journal MUST provide atomic compare-and-swap updates.
//!
//! This prevents concurrent recovery coordinators from silently
//! overwriting each other's state.
//!
//! # Scalability
//!
//! No universal maximum is imposed on:
//! - incidents;
//! - tasks;
//! - participants;
//! - recovery actions;
//! - execution contexts;
//! - devices;
//! - checkpoints;
//! - computational domains.
//!
//! Actual storage, execution, numeric representation and resource
//! constraints remain explicit operational limitations.
//!
//! # Integration
//!
//! distributed/error.rs:
//!     Canonical DistributedError and DistributedResult.
//!
//! distributed/config.rs:
//!     Canonical RetryPolicy and operational configuration.
//!
//! distributed/resilience/retry.rs:
//!     Retry decisions and retry safety.
//!
//! distributed/resilience/checkpoint.rs:
//!     Checkpoint ownership and checkpoint validity.
//!
//! distributed/resilience/idempotency.rs:
//!     Durable operation deduplication.
//!
//! distributed/membership/:
//!     Participant availability and membership observations.
//!
//! distributed/planning/:
//!     Replanning, feasibility and plan validation.
//!
//! distributed/execution/:
//!     Actual execution and lifecycle integration.
//!
//! quantum::zqn:
//!     Canonical quantum fault information.
//!
//! quantum::ir:
//!     Canonical logical quantum identities.
//!
//! hardware/HAL:
//!     Physical realization and device operations.
//!
//! No competing recovery, checkpoint, scheduling, routing,
//! hardware, or quantum representation is introduced.

use core::fmt;

use crate::distributed::error::{
    DistributedError,
    DistributedResult,
};

use crate::distributed::ids::{
    OperationId,
    PlanId,
};

use crate::distributed::ids::CheckpointId;

// ============================================================================
// CONTRACT VERSION
// ============================================================================

/// Stable identifier for the recovery contract.
pub const DISTRIBUTED_RECOVERY_SCHEMA_ID: &str =
    "zamani.distributed.resilience.recovery";

/// Recovery contract version.
///
/// This is independent of language, IR and transport versions.
pub const DISTRIBUTED_RECOVERY_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// RECOVERY ACTION IDENTITY
// ============================================================================

/// Stable identity of one recovery action.
///
/// The same identity MUST be reused when reconciling an interrupted
/// submission of that action.
///
/// `generation` distinguishes successive recovery attempts for the
/// same logical operation.
///
/// `ordinal` distinguishes actions within one recovery plan.
///
/// Neither field identifies a physical machine or device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecoveryActionId {
    operation_id: OperationId,
    generation: u64,
    ordinal: u64,
}

impl RecoveryActionId {
    /// Constructs a recovery action identity.
    #[must_use]
    pub const fn new(
        operation_id: OperationId,
        generation: u64,
        ordinal: u64,
    ) -> Self {
        Self {
            operation_id,
            generation,
            ordinal,
        }
    }

    /// Returns the associated logical operation.
    #[must_use]
    pub const fn operation_id(self) -> OperationId {
        self.operation_id
    }

    /// Returns the recovery generation.
    #[must_use]
    pub const fn generation(self) -> u64 {
        self.generation
    }

    /// Returns the action ordinal.
    #[must_use]
    pub const fn ordinal(self) -> u64 {
        self.ordinal
    }
}

impl fmt::Display for RecoveryActionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:recovery:{}:{}",
            self.operation_id,
            self.generation,
            self.ordinal
        )
    }
}

// ============================================================================
// INCIDENT
// ============================================================================

/// Classification of a recovery incident.
///
/// This is an orchestration-level classification, not a replacement
/// for the canonical ZQN or distributed error taxonomies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RecoveryIncidentKind {
    /// An execution operation failed.
    ExecutionFailure,

    /// An execution participant became unavailable.
    ParticipantUnavailable,

    /// Communication failure interrupted execution.
    CommunicationFailure,

    /// Execution completed but its outcome is uncertain.
    OutcomeUnknown,

    /// Required execution state was lost.
    StateUnavailable,

    /// State integrity could not be established.
    StateIntegrityFailure,

    /// Required execution capabilities became unavailable.
    CapabilityLoss,

    /// Required resources became unavailable.
    ResourceExhaustion,

    /// A deployment or migration operation failed.
    DeploymentFailure,

    /// A checkpoint restoration failed.
    CheckpointFailure,

    /// A previous recovery operation was interrupted.
    RecoveryInterrupted,

    /// A domain provider reported a domain-specific fault.
    DomainFailure,

    /// An implementation-defined incident.
    Other,
}

/// Evidence describing one recovery incident.
///
/// Identifiers remain logical. This structure does not contain
/// hardware addresses, physical memory locations or device handles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryIncident {
    operation_id: OperationId,
    kind: RecoveryIncidentKind,
    description: String,
    observed_revision: u64,
    observed_plan_generation: u64,
}

impl RecoveryIncident {
    /// Creates a recovery incident.
    ///
    /// The description is diagnostic information, not executable input.
    #[must_use]
    pub fn new(
        operation_id: OperationId,
        kind: RecoveryIncidentKind,
        description: impl Into<String>,
        observed_revision: u64,
        observed_plan_generation: u64,
    ) -> Self {
        Self {
            operation_id,
            kind,
            description: description.into(),
            observed_revision,
            observed_plan_generation,
        }
    }

    /// Returns the affected logical operation.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the incident classification.
    #[must_use]
    pub const fn kind(&self) -> RecoveryIncidentKind {
        self.kind
    }

    /// Returns the diagnostic description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the revision observed when the incident was created.
    #[must_use]
    pub const fn observed_revision(&self) -> u64 {
        self.observed_revision
    }

    /// Returns the plan generation observed when the incident was created.
    #[must_use]
    pub const fn observed_plan_generation(&self) -> u64 {
        self.observed_plan_generation
    }
}

// ============================================================================
// RECOVERY ACTIONS
// ============================================================================

/// Domain-independent recovery action category.
///
/// The selected execution provider determines whether an action
/// can actually be performed.
///
/// The variants express orchestration intent, not implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RecoveryActionKind {
    /// Establish the authoritative outcome of an uncertain operation.
    Reconcile,

    /// Continue from a validated checkpoint.
    Resume,

    /// Start a fresh execution under a new valid execution context.
    Restart,

    /// Restore a previously valid state.
    Rollback,

    /// Move a logical computation to another compatible realization.
    Migration,

    /// Recompute work whose result can be safely reconstructed.
    Recompute,

    /// Compensate for an already-observed external effect.
    Compensation,

    /// Use an existing verified replica.
    ReplicaPromotion,

    /// Terminate the affected computation.
    Abort,
}

/// Immutable recovery action description.
///
/// Checkpoint references identify existing checkpoint records.
/// This type does not serialize or reconstruct checkpoint contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryAction {
    kind: RecoveryActionKind,
    checkpoint: Option<CheckpointId>,
    reason: String,
}

impl RecoveryAction {
    /// Creates a recovery action.
    #[must_use]
    pub fn new(
        kind: RecoveryActionKind,
        checkpoint: Option<CheckpointId>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            checkpoint,
            reason: reason.into(),
        }
    }

    /// Returns the action category.
    #[must_use]
    pub const fn kind(&self) -> RecoveryActionKind {
        self.kind
    }

    /// Returns the checkpoint reference, when applicable.
    #[must_use]
    pub const fn checkpoint(&self) -> Option<CheckpointId> {
        self.checkpoint
    }

    /// Returns the action rationale.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

// ============================================================================
// RECOVERY PLAN
// ============================================================================

/// An immutable recovery plan.
///
/// A plan is associated with:
/// - one logical operation;
/// - one recovery generation;
/// - one original plan identity;
/// - one observed capability generation;
/// - an ordered collection of recovery actions.
///
/// Capability generation is supplied by the authoritative capability
/// and planning systems.
///
/// A changed generation invalidates the assumptions of this plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryPlan {
    operation_id: OperationId,
    plan_id: PlanId,
    generation: u64,
    expected_revision: u64,
    source_plan_generation: u64,
    capability_generation: u64,
    actions: Vec<RecoveryAction>,
}

impl RecoveryPlan {
    /// Creates a recovery plan.
    ///
    /// An empty action sequence is rejected.
    pub fn new(
        operation_id: OperationId,
        plan_id: PlanId,
        generation: u64,
        expected_revision: u64,
        source_plan_generation: u64,
        capability_generation: u64,
        actions: Vec<RecoveryAction>,
    ) -> DistributedResult<Self> {
        if actions.is_empty() {
            return Err(DistributedError::InvalidInput {
                message: "recovery plan must contain at least one action"
                    .to_owned(),
            });
        }

        Ok(Self {
            operation_id,
            plan_id,
            generation,
            expected_revision,
            source_plan_generation,
            capability_generation,
            actions,
        })
    }

    /// Returns the logical operation.
    #[must_use]
    pub const fn operation_id(&self) -> OperationId {
        self.operation_id
    }

    /// Returns the source plan identity.
    #[must_use]
    pub const fn plan_id(&self) -> PlanId {
        self.plan_id
    }

    /// Returns the recovery generation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the expected journal revision.
    #[must_use]
    pub const fn expected_revision(&self) -> u64 {
        self.expected_revision
    }

    /// Returns the source execution plan generation.
    #[must_use]
    pub const fn source_plan_generation(&self) -> u64 {
        self.source_plan_generation
    }

    /// Returns the capability generation used during planning.
    #[must_use]
    pub const fn capability_generation(&self) -> u64 {
        self.capability_generation
    }

    /// Returns the ordered recovery actions.
    #[must_use]
    pub fn actions(&self) -> &[RecoveryAction] {
        &self.actions
    }
}

// ============================================================================
// LIFECYCLE
// ============================================================================

/// Durable recovery lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RecoveryStatus {
    /// An incident has been recorded.
    Incident,

    /// A recovery plan is being prepared.
    Planning,

    /// The recovery plan passed pre-execution validation.
    PlanValidated,

    /// Recovery actions are being performed.
    Recovering,

    /// The resulting execution state is being verified.
    Verifying,

    /// Recovery completed and was verified.
    Succeeded,

    /// Recovery failed.
    Failed,

    /// Recovery was explicitly cancelled.
    Cancelled,

    /// The recovery process requires external reconciliation.
    ReconciliationRequired,
}

/// Status of an individual recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RecoveryActionStatus {
    /// Action has not been submitted.
    Pending,

    /// Action submission was durably recorded.
    Started,

    /// Provider confirmed successful completion.
    Completed,

    /// Provider confirmed failure.
    Failed,

    /// Provider cannot establish the action outcome.
    OutcomeUnknown,

    /// Action was cancelled.
    Cancelled,
}

/// Persisted recovery state.
///
/// This is the authoritative logical lifecycle record.
/// It does not contain serialized computation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryRecord {
    incident: RecoveryIncident,
    status: RecoveryStatus,
    generation: u64,
    revision: u64,
    plan: Option<RecoveryPlan>,
    action_statuses: Vec<RecoveryActionStatus>,
    failure: Option<String>,
}

impl RecoveryRecord {
    /// Creates the initial record for a new incident.
    #[must_use]
    pub fn new(incident: RecoveryIncident) -> Self {
        Self {
            incident,
            status: RecoveryStatus::Incident,
            generation: 0,
            revision: 0,
            plan: None,
            action_statuses: Vec::new(),
            failure: None,
        }
    }

    /// Returns the original incident.
    #[must_use]
    pub fn incident(&self) -> &RecoveryIncident {
        &self.incident
    }

    /// Returns the current recovery status.
    #[must_use]
    pub const fn status(&self) -> RecoveryStatus {
        self.status
    }

    /// Returns the recovery generation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the journal revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns the recovery plan, when available.
    #[must_use]
    pub fn plan(&self) -> Option<&RecoveryPlan> {
        self.plan.as_ref()
    }

    /// Returns all action statuses.
    #[must_use]
    pub fn action_statuses(&self) -> &[RecoveryActionStatus] {
        &self.action_statuses
    }

    /// Returns the failure description, when present.
    #[must_use]
    pub fn failure(&self) -> Option<&str> {
        self.failure.as_deref()
    }
}

// ============================================================================
// DURABLE JOURNAL
// ============================================================================

/// Result of an atomic journal compare-and-swap operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalWrite {
    /// The new record was durably committed.
    Committed,

    /// Another writer changed the record.
    ///
    /// The caller must reload and reconcile.
    RevisionConflict,
}

/// Durable recovery journal contract.
///
/// Implementations MUST:
///
/// - Persist records across process restarts.
/// - Provide atomic compare-and-swap updates.
/// - Never acknowledge a write before its configured durability
///   requirement is satisfied.
/// - Preserve the complete record.
/// - Prevent stale writers from replacing newer records.
/// - Return the current record when loading.
/// - Handle concurrent writers correctly.
///
/// A plain in-memory HashMap does not satisfy this production contract.
///
/// The journal implementation owns its persistence mechanism.
pub trait RecoveryJournal: Send + Sync + fmt::Debug {
    /// Loads a recovery record.
    ///
    /// Returns `None` if no record exists.
    fn load(
        &self,
        operation_id: OperationId,
    ) -> DistributedResult<Option<RecoveryRecord>>;

    /// Atomically commits a new record if its expected revision matches.
    ///
    /// The implementation MUST durably commit the record before
    /// returning `Committed`.
    fn compare_and_swap(
        &self,
        operation_id: OperationId,
        expected_revision: u64,
        record: RecoveryRecord,
    ) -> DistributedResult<JournalWrite>;
}

// ============================================================================
// EXECUTION PROVIDER
// ============================================================================

/// Authoritative recovery execution provider.
///
/// The provider connects recovery orchestration to existing execution,
/// runtime, checkpoint, domain and hardware authorities.
///
/// Implementations MUST preserve the same action identity across
/// repeated submissions.
///
/// They MUST either:
///
/// - deduplicate repeated action submissions durably; or
/// - reconcile an interrupted action before permitting resubmission.
///
/// A provider MUST NOT report success for an operation it did not perform.
///
/// The provider MUST NOT silently weaken semantic requirements.
pub trait RecoveryExecutor: Send + Sync + fmt::Debug {
    /// Validates that the recovery plan is still executable.
    ///
    /// Implementations check the relevant:
    /// - semantic validity;
    /// - security authorization;
    /// - resource feasibility;
    /// - capabilities;
    /// - membership;
    /// - checkpoint validity;
    /// - execution-plan assumptions.
    ///
    /// Returns an error when any required condition is not satisfied.
    fn validate_plan(
        &self,
        incident: &RecoveryIncident,
        plan: &RecoveryPlan,
    ) -> DistributedResult<()>;

    /// Establishes the authoritative outcome of an interrupted action.
    ///
    /// This must be consulted before resubmitting an action whose
    /// previous outcome is uncertain.
    fn reconcile_action(
        &self,
        action_id: RecoveryActionId,
        action: &RecoveryAction,
    ) -> DistributedResult<RecoveryActionStatus>;

    /// Executes or safely deduplicates one recovery action.
    ///
    /// The action identity is stable across process restarts.
    fn execute_action(
        &self,
        action_id: RecoveryActionId,
        action: &RecoveryAction,
    ) -> DistributedResult<RecoveryActionStatus>;

    /// Verifies the resulting logical computation state.
    ///
    /// A successful return means the provider has established that
    /// the required semantic invariants hold.
    ///
    /// It MUST NOT merely confirm that an action was submitted.
    fn verify_recovered_state(
        &self,
        incident: &RecoveryIncident,
        plan: &RecoveryPlan,
    ) -> DistributedResult<()>;
}

// ============================================================================
// RECOVERY ENGINE
// ============================================================================

/// Distributed recovery orchestrator.
///
/// This type is stateless. Durable lifecycle state belongs to the journal.
/// Execution belongs to the injected recovery provider.
///
/// Multiple engine instances may safely operate against the same journal
/// when the journal implements the required atomic compare-and-swap
/// semantics.
#[derive(Debug, Default, Clone, Copy)]
pub struct RecoveryEngine;

impl RecoveryEngine {
    /// Creates a recovery engine.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Records a new recovery incident.
    ///
    /// Duplicate incident registration is rejected rather than silently
    /// replacing an existing recovery lifecycle.
    pub fn register_incident(
        &self,
        journal: &dyn RecoveryJournal,
        incident: RecoveryIncident,
    ) -> DistributedResult<RecoveryRecord> {
        let operation_id = incident.operation_id();

        if journal.load(operation_id)?.is_some() {
            return Err(DistributedError::Conflict {
                subject: operation_id.to_string(),
                message: "a recovery record already exists for this operation"
                    .to_owned(),
            });
        }

        let record = RecoveryRecord::new(incident);

        match journal.compare_and_swap(
            operation_id,
            0,
            record.clone(),
        )? {
            JournalWrite::Committed => Ok(record),

            JournalWrite::RevisionConflict => {
                Err(DistributedError::Conflict {
                    subject: operation_id.to_string(),
                    message: "concurrent recovery incident registration"
                        .to_owned(),
                })
            }
        }
    }

    /// Begins a recovery generation using a validated recovery plan.
    ///
    /// The caller must supply a plan produced by the authoritative
    /// recovery/planning integration.
    ///
    /// This method rejects stale incident revisions and invalid lifecycle
    /// transitions.
    pub fn begin_recovery(
        &self,
        journal: &dyn RecoveryJournal,
        executor: &dyn RecoveryExecutor,
        plan: RecoveryPlan,
    ) -> DistributedResult<RecoveryRecord> {
        let operation_id = plan.operation_id();

        let mut record = self.load_required(journal, operation_id)?;

        if record.status != RecoveryStatus::Incident
            && record.status != RecoveryStatus::Failed
            && record.status != RecoveryStatus::ReconciliationRequired
        {
            return Err(DistributedError::Conflict {
                subject: operation_id.to_string(),
                message: "recovery cannot begin from the current lifecycle state"
                    .to_owned(),
            });
        }

        if plan.expected_revision() != record.revision {
            return Err(DistributedError::Conflict {
                subject: operation_id.to_string(),
                message: "recovery plan was created against a stale journal revision"
                    .to_owned(),
            });
        }

        if plan.source_plan_generation()
            != record.incident.observed_plan_generation()
        {
            return Err(DistributedError::Conflict {
                subject: operation_id.to_string(),
                message: "recovery plan references a stale execution-plan generation"
                    .to_owned(),
            });
        }

        let next_generation = record
            .generation
            .checked_add(1)
            .ok_or_else(|| recovery_error(
                operation_id,
                "generation",
                "recovery generation representation exhausted",
            ))?;

        if plan.generation() != next_generation {
            return Err(DistributedError::Conflict {
                subject: operation_id.to_string(),
                message: "recovery plan generation does not match the next generation"
                    .to_owned(),
            });
        }

        executor.validate_plan(&record.incident, &plan)?;

        record.generation = next_generation;
        record.status = RecoveryStatus::PlanValidated;
        record.action_statuses = vec![
            RecoveryActionStatus::Pending;
            plan.actions().len()
        ];
        record.plan = Some(plan);
        record.failure = None;

        self.commit_next(journal, record)
    }

    /// Executes the actions in a validated recovery plan.
    ///
    /// The journal is updated before every action submission.
    ///
    /// Unknown outcomes are reconciled before any resubmission.
    ///
    /// This method does not perform automatic retries.
    pub fn execute_recovery(
        &self,
        journal: &dyn RecoveryJournal,
        executor: &dyn RecoveryExecutor,
        operation_id: OperationId,
    ) -> DistributedResult<RecoveryRecord> {
        let mut record = self.load_required(journal, operation_id)?;

        if record.status != RecoveryStatus::PlanValidated
            && record.status != RecoveryStatus::Recovering
        {
            return Err(DistributedError::Conflict {
                subject: operation_id.to_string(),
                message: "recovery execution requires a validated or interrupted recovery plan"
                    .to_owned(),
            });
        }

        let plan = record.plan.clone().ok_or_else(|| {
            recovery_error(
                operation_id,
                "plan",
                "recovery plan is missing",
            )
        })?;

        executor.validate_plan(&record.incident, &plan)?;

        record.status = RecoveryStatus::Recovering;
        record = self.commit_next(journal, record)?;

        for (index, action) in plan.actions().iter().enumerate() {
            let ordinal = u64::try_from(index).map_err(|_| {
                recovery_error(
                    operation_id,
                    "action_identity",
                    "recovery action index cannot be represented",
                )
            })?;

            let action_id = RecoveryActionId::new(
                operation_id,
                plan.generation(),
                ordinal,
            );

            match record.action_statuses.get(index).copied() {
                Some(RecoveryActionStatus::Completed) => continue,

                Some(RecoveryActionStatus::OutcomeUnknown)
                | Some(RecoveryActionStatus::Started) => {
                    let reconciled =
                        executor.reconcile_action(action_id, action)?;

                    match reconciled {
                        RecoveryActionStatus::Completed => {
                            record.action_statuses[index] =
                                RecoveryActionStatus::Completed;

                            record = self.commit_next(journal, record)?;
                            continue;
                        }

                        RecoveryActionStatus::Pending => {
                            // The provider has authoritatively established
                            // that this action was not executed.
                        }

                        RecoveryActionStatus::Failed => {
                            return self.fail_recovery(
                                journal,
                                record,
                                "interrupted recovery action was confirmed failed",
                            );
                        }

                        RecoveryActionStatus::OutcomeUnknown => {
                            record.status =
                                RecoveryStatus::ReconciliationRequired;

                            record.action_statuses[index] =
                                RecoveryActionStatus::OutcomeUnknown;

                            let _ = self.commit_next(journal, record)?;

                            return Err(recovery_error(
                                operation_id,
                                "reconciliation",
                                "provider could not establish the previous action outcome",
                            ));
                        }

                        RecoveryActionStatus::Started => {
                            return Err(recovery_error(
                                operation_id,
                                "reconciliation",
                                "provider returned an unresolved started action",
                            ));
                        }

                        RecoveryActionStatus::Cancelled => {
                            return self.cancel_recovery(
                                journal,
                                record,
                            );
                        }
                    }
                }

                Some(RecoveryActionStatus::Pending) => {}

                Some(RecoveryActionStatus::Failed)
                | Some(RecoveryActionStatus::Cancelled)
                | None => {
                    return Err(recovery_error(
                        operation_id,
                        "action_state",
                        "recovery action is not eligible for execution",
                    ));
                }
            }

            // Record intent before submitting the action.
            record.action_statuses[index] =
                RecoveryActionStatus::Started;

            record = self.commit_next(journal, record)?;

            let outcome = executor.execute_action(action_id, action)?;

            match outcome {
                RecoveryActionStatus::Completed => {
                    record.action_statuses[index] =
                        RecoveryActionStatus::Completed;

                    record = self.commit_next(journal, record)?;
                }

                RecoveryActionStatus::Pending
                | RecoveryActionStatus::Started
                | RecoveryActionStatus::OutcomeUnknown => {
                    record.action_statuses[index] =
                        RecoveryActionStatus::OutcomeUnknown;

                    record.status =
                        RecoveryStatus::ReconciliationRequired;

                    let _ = self.commit_next(journal, record)?;

                    return Err(recovery_error(
                        operation_id,
                        "action_execution",
                        "recovery action did not establish successful completion",
                    ));
                }

                RecoveryActionStatus::Failed => {
                    return self.fail_recovery(
                        journal,
                        record,
                        "recovery provider confirmed action failure",
                    );
                }

                RecoveryActionStatus::Cancelled => {
                    return self.cancel_recovery(
                        journal,
                        record,
                    );
                }
            }
        }

        record.status = RecoveryStatus::Verifying;
        self.commit_next(journal, record)
    }

    /// Verifies recovery and commits the terminal success state.
    ///
    /// Success is impossible without the provider's explicit verification.
    pub fn verify_recovery(
        &self,
        journal: &dyn RecoveryJournal,
        executor: &dyn RecoveryExecutor,
        operation_id: OperationId,
    ) -> DistributedResult<RecoveryRecord> {
        let mut record = self.load_required(journal, operation_id)?;

        if record.status != RecoveryStatus::Verifying {
            return Err(DistributedError::Conflict {
                subject: operation_id.to_string(),
                message: "recovery must finish all actions before verification"
                    .to_owned(),
            });
        }

        let plan = record.plan.as_ref().ok_or_else(|| {
            recovery_error(
                operation_id,
                "verification",
                "verified recovery requires a persisted plan",
            )
        })?;

        if record.action_statuses.iter().any(|status| {
            *status != RecoveryActionStatus::Completed
        }) {
            return Err(recovery_error(
                operation_id,
                "verification",
                "one or more recovery actions remain incomplete",
            ));
        }

        executor.validate_plan(&record.incident, plan)?;

        executor.verify_recovered_state(&record.incident, plan)?;

        record.status = RecoveryStatus::Succeeded;
        self.commit_next(journal, record)
    }

    /// Marks an operation as requiring explicit external reconciliation.
    ///
    /// This is useful when a provider, journal or process failure prevents
    /// the engine from safely establishing the previous recovery outcome.
    pub fn require_reconciliation(
        &self,
        journal: &dyn RecoveryJournal,
        operation_id: OperationId,
        reason: impl Into<String>,
    ) -> DistributedResult<RecoveryRecord> {
        let mut record = self.load_required(journal, operation_id)?;

        if record.status == RecoveryStatus::Succeeded {
            return Err(DistributedError::Conflict {
                subject: operation_id.to_string(),
                message: "successful recovery cannot be reopened implicitly"
                    .to_owned(),
            });
        }

        record.status = RecoveryStatus::ReconciliationRequired;
        record.failure = Some(reason.into());

        self.commit_next(journal, record)
    }

    /// Cancels a recovery lifecycle.
    ///
    /// This records cancellation. Propagating cancellation to running
    /// execution providers belongs to the runtime cancellation authority.
    pub fn cancel(
        &self,
        journal: &dyn RecoveryJournal,
        operation_id: OperationId,
    ) -> DistributedResult<RecoveryRecord> {
        let mut record = self.load_required(journal, operation_id)?;

        if record.status == RecoveryStatus::Succeeded {
            return Err(DistributedError::Conflict {
                subject: operation_id.to_string(),
                message: "completed recovery cannot be cancelled",
            });
        }

        record.status = RecoveryStatus::Cancelled;

        self.commit_next(journal, record)
    }

    // ========================================================================
    // INTERNAL TRANSACTION HELPERS
    // ========================================================================

    fn load_required(
        &self,
        journal: &dyn RecoveryJournal,
        operation_id: OperationId,
    ) -> DistributedResult<RecoveryRecord> {
        journal.load(operation_id)?.ok_or_else(|| {
            recovery_error(
                operation_id,
                "journal",
                "recovery record does not exist",
            )
        })
    }

    fn commit_next(
        &self,
        journal: &dyn RecoveryJournal,
        mut record: RecoveryRecord,
    ) -> DistributedResult<RecoveryRecord> {
        let operation_id = record.incident.operation_id();

        let previous_revision = record.revision;

        record.revision = previous_revision.checked_add(1).ok_or_else(|| {
            recovery_error(
                operation_id,
                "journal_revision",
                "recovery journal revision exhausted",
            )
        })?;

        match journal.compare_and_swap(
            operation_id,
            previous_revision,
            record.clone(),
        )? {
            JournalWrite::Committed => Ok(record),

            JournalWrite::RevisionConflict => {
                Err(DistributedError::Conflict {
                    subject: operation_id.to_string(),
                    message: "recovery journal changed concurrently; reload and reconcile"
                        .to_owned(),
                })
            }
        }
    }

    fn fail_recovery(
        &self,
        journal: &dyn RecoveryJournal,
        mut record: RecoveryRecord,
        reason: &str,
    ) -> DistributedResult<RecoveryRecord> {
        record.status = RecoveryStatus::Failed;
        record.failure = Some(reason.to_owned());

        let operation_id = record.incident.operation_id();

        let _ = self.commit_next(journal, record)?;

        Err(recovery_error(
            operation_id,
            "action_execution",
            reason,
        ))
    }

    fn cancel_recovery(
        &self,
        journal: &dyn RecoveryJournal,
        mut record: RecoveryRecord,
    ) -> DistributedResult<RecoveryRecord> {
        record.status = RecoveryStatus::Cancelled;

        self.commit_next(journal, record)
    }
}

// ============================================================================
// ERROR ADAPTER
// ============================================================================

fn recovery_error(
    operation_id: OperationId,
    stage: &str,
    message: &str,
) -> DistributedError {
    DistributedError::RecoveryFailed {
        subject: format!("{operation_id}:{stage}"),
        message: message.to_owned(),
    }
}
