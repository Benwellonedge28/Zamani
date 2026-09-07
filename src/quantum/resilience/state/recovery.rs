//! Zamani Quantum Resilience — Recovery Operation State
//!
//! Path:
//!     src/quantum/resilience/state/recovery.rs
//!
//! # Purpose
//!
//! This module owns the state of an individual resilience recovery operation.
//!
//! It records:
//!
//! - recovery-operation identity;
//! - recovery lifecycle state;
//! - operation generation;
//! - attempt generation;
//! - affected logical qubits;
//! - recovery-plan identity/version;
//! - verification requirements and status;
//! - terminal outcome;
//! - deterministic transition history;
//! - optimistic-concurrency expectations;
//! - transactional multi-operation state transitions.
//!
//! It does NOT execute recovery.
//!
//! Actual recovery mechanisms belong to:
//!
//!     crate::quantum::resilience::recovery
//!
//! Planning belongs to:
//!
//!     crate::quantum::resilience::planning
//!
//! Verification belongs to:
//!
//!     crate::quantum::resilience::verification
//!
//! Logical-qubit identity comes from:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! Physical placement remains outside this module.
//!
//! # Architectural position
//!
//! ```text
//!                     Detection / Diagnosis
//!                              |
//!                              v
//!                         Recovery Plan
//!                              |
//!                              v
//!                    +---------------------+
//!                    | state::recovery     |
//!                    |                     |
//!                    | operation lifecycle |
//!                    | attempt generation  |
//!                    | plan generation     |
//!                    | affected Qubits     |
//!                    | verification state  |
//!                    +----------+----------+
//!                               |
//!                +--------------+--------------+
//!                |                             |
//!                v                             v
//!           recovery/*                    verification/*
//!           executes                     validates
//!                |                             |
//!                +--------------+--------------+
//!                               |
//!                               v
//!                         state::machine
//! ```
//!
//! `state::machine` owns the global resilience lifecycle.
//!
//! This module owns recovery-operation-local state.
//!
//! Those are deliberately separate.
//!
//! # Canonical identity
//!
//! Logical resources MUST use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!
//! This file MUST NOT define a second logical-qubit identifier.
//!
//! Physical qubit identifiers are intentionally not stored here. A logical
//! resource may be remapped to a different physical resource during recovery.
//! Placement is therefore owned by routing/adaptation/hardware layers.
//!
//! # Recovery is not quantum state
//!
//! `RecoveryState` is operational bookkeeping.
//!
//! It does not represent:
//!
//! - amplitudes;
//! - wavefunctions;
//! - density matrices;
//! - stabilizer state;
//! - syndrome state;
//! - decoder state;
//! - physical calibration;
//! - hardware health.
//!
//! Quantum state belongs to the appropriate quantum execution/simulation/QEC
//! subsystem.
//!
//! # Write once / scale everywhere
//!
//! This module contains no artificial machine-size limits.
//!
//! It does NOT define:
//!
//! - maximum qubits;
//! - maximum recovery operations;
//! - maximum attempts;
//! - maximum incidents;
//! - maximum plan size;
//! - maximum resources;
//! - provider-specific limits;
//! - fixed-size arrays;
//! - fixed retry counts.
//!
//! Collections grow according to workload and available resources.
//!
//! "Infinite" therefore means that the architecture imposes no artificial
//! finite quantum-machine ceiling. A concrete process remains limited by the
//! memory, address space, execution policy, hardware and budgets actually
//! available.
//!
//! # Determinism
//!
//! Ordered collections are used wherever ordering affects observable behavior.
//!
//! No clock, randomness source, thread scheduler or provider-specific behavior
//! is consulted here.
//!
//! The state machine is therefore deterministic for an identical initial state
//! and identical ordered sequence of transitions.
//!
//! # Concurrency
//!
//! This module deliberately contains no mutex, RwLock, async runtime primitive,
//! actor runtime, or global synchronization mechanism.
//!
//! Synchronization belongs to the controller/runtime/coordination layer.
//!
//! The `generation` and `revision` values provide optimistic-concurrency
//! protection for callers that need to detect stale state.
//!
//! # Transactionality
//!
//! Batch transitions are atomic from the perspective of this state object:
//!
//! ```text
//! all requested transitions succeed
//!             OR
//! none of them become observable
//! ```
//!
//! Rollback uses a compact journal instead of cloning the entire state map.
//!
//! This keeps transaction overhead proportional to the number of affected
//! recovery operations rather than the total number of recovery operations.
//!
//! # Stale-plan protection
//!
//! A recovery plan is expected to carry the recovery generation it was created
//! against.
//!
//! Callers can compare:
//!
//!     expected_plan_generation()
//!
//! with:
//!
//!     plan_generation()
//!
//! before executing a plan.
//!
//! If state changed after planning, the recovery implementation can reject the
//! stale plan and request replanning instead of executing against obsolete
//! assumptions.
//!
//! # Verification barrier
//!
//! A recovery operation cannot transition directly from `Recovering` to
//! `Succeeded`.
//!
//! It must pass through `Verifying`.
//!
//! This enforces the architectural rule:
//!
//! > Availability is never sufficient evidence that recovery succeeded.
//!
//! The verification subsystem remains authoritative for semantic acceptance.
//!
//! # Integration contract
//!
//! Consumers:
//!
//! - `resilience::recovery`;
//! - `resilience::planning`;
//! - `resilience::adaptation`;
//! - `resilience::verification`;
//! - `resilience::checkpoint`;
//! - `resilience::telemetry`;
//! - `resilience::history`;
//! - `resilience::coordination`;
//! - `resilience::api`.
//!
//! Authorities:
//!
//! - `quantum::ir::qubit::QubitId` for logical identity;
//! - planning for recovery-plan semantics;
//! - verification for result acceptance;
//! - routing/adaptation for physical placement;
//! - hardware for physical resource state;
//! - state::machine for global resilience lifecycle.
//!
//! This module intentionally depends only on the canonical logical-qubit ID
//! type and the Rust standard library. That makes it implementable before the
//! higher-level recovery modules exist and prevents circular dependencies.
//!
//! # Rust compatibility
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::quantum::ir::qubit::QubitId;

// ============================================================================
// Recovery operation identity
// ============================================================================

/// Stable identity of one recovery operation.
///
/// The state layer does not assign semantic meaning to the identifier.
/// Identity generation belongs to the controller/coordination layer.
///
/// `u128` provides a large identifier space without imposing a machine-size
/// limit. The value may be generated from a deterministic execution identifier,
/// UUID-like scheme, or another repository-level identity provider.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecoveryOperationId(pub u128);

impl RecoveryOperationId {
    /// Creates an operation identifier from a raw value.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the underlying identifier value.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }
}

impl fmt::Display for RecoveryOperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:032x}", self.0)
    }
}

// ============================================================================
// Recovery state
// ============================================================================

/// Lifecycle state of one recovery operation.
///
/// This is intentionally different from `state::machine::ResilienceState`.
///
/// `ResilienceState` answers:
///
///     "Which global resilience phase is active?"
///
/// `RecoveryState` answers:
///
///     "What is this particular recovery operation doing?"
///
/// # State graph
///
/// ```text
/// Planned
///    |
///    v
/// Authorized
///    |
///    v
/// Adapting
///    |
///    +------------------+
///    |                  |
///    v                  v
/// Recovering         Escalated
///    |                  |
///    v                  |
/// Verifying             |
///    |                  |
///    +------+-----------+
///    |      |
///    v      v
///Succeeded Failed
///
/// Recovering may also transition to:
///
///     Planned       (replan required)
///     Authorized    (new execution authorization)
///     Escalated
///     Failed
///
/// Verifying may transition to:
///
///     Succeeded
///     Planned        (verification requires recovery/replanning)
///     Escalated
///     Failed
///
/// Terminal states:
///
///     Succeeded
///     Failed
///     Escalated
///     Cancelled
/// ```
///
/// A terminal state cannot be silently reopened.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecoveryState {
    /// A recovery plan exists but execution has not been authorized.
    Planned,

    /// Policy/authorization boundaries have accepted execution of the plan.
    Authorized,

    /// The recovery implementation is changing execution resources, routing,
    /// scheduling, compilation, QEC configuration, or another implementation
    /// detail before actual recovery execution.
    Adapting,

    /// Recovery actions are actively executing.
    Recovering,

    /// The recovery result is being checked by the verification subsystem.
    Verifying,

    /// Recovery completed and passed the required state-level verification
    /// barrier.
    Succeeded,

    /// Recovery failed and cannot currently continue through this operation.
    Failed,

    /// Automatic recovery stopped and the incident requires escalation.
    Escalated,

    /// Recovery was explicitly cancelled before successful completion.
    Cancelled,
}

impl RecoveryState {
    /// Returns a stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Authorized => "authorized",
            Self::Adapting => "adapting",
            Self::Recovering => "recovering",
            Self::Verifying => "verifying",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Escalated => "escalated",
            Self::Cancelled => "cancelled",
        }
    }

    /// Returns a stable ordinal representation for deterministic serialization.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        match self {
            Self::Planned => 0,
            Self::Authorized => 1,
            Self::Adapting => 2,
            Self::Recovering => 3,
            Self::Verifying => 4,
            Self::Succeeded => 5,
            Self::Failed => 6,
            Self::Escalated => 7,
            Self::Cancelled => 8,
        }
    }

    /// Decodes the stable ordinal representation.
    #[must_use]
    pub const fn from_ordinal(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Planned),
            1 => Some(Self::Authorized),
            2 => Some(Self::Adapting),
            3 => Some(Self::Recovering),
            4 => Some(Self::Verifying),
            5 => Some(Self::Succeeded),
            6 => Some(Self::Failed),
            7 => Some(Self::Escalated),
            8 => Some(Self::Cancelled),
            _ => None,
        }
    }

    /// Returns whether the state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded | Self::Failed | Self::Escalated | Self::Cancelled
        )
    }

    /// Returns whether recovery execution is actively running.
    #[must_use]
    pub const fn is_executing(self) -> bool {
        matches!(self, Self::Adapting | Self::Recovering)
    }

    /// Returns whether verification is required before success can be claimed.
    #[must_use]
    pub const fn requires_verification(self) -> bool {
        matches!(self, Self::Recovering | Self::Verifying)
    }

    /// Returns whether the operation can be replanned.
    #[must_use]
    pub const fn can_replan(self) -> bool {
        matches!(
            self,
            Self::Planned
                | Self::Authorized
                | Self::Adapting
                | Self::Recovering
                | Self::Verifying
        )
    }
}

impl fmt::Display for RecoveryState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Default for RecoveryState {
    fn default() -> Self {
        Self::Planned
    }
}

// ============================================================================
// Verification status
// ============================================================================

/// State-level verification status for a recovery operation.
///
/// This does not replace `resilience::verification`.
///
/// It records only the result needed by the recovery state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecoveryVerification {
    /// No verification has been requested or started.
    NotRequired,

    /// Verification is required but has not started.
    Required,

    /// Verification is currently running.
    InProgress,

    /// Verification accepted the recovered execution.
    Accepted,

    /// Verification determined that recovery was insufficient and another
    /// recovery/replan cycle is required.
    RequiresRecovery,

    /// Verification rejected the result.
    Rejected,
}

impl RecoveryVerification {
    /// Returns a stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Required => "required",
            Self::InProgress => "in_progress",
            Self::Accepted => "accepted",
            Self::RequiresRecovery => "requires_recovery",
            Self::Rejected => "rejected",
        }
    }

    /// Returns a stable ordinal representation.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        match self {
            Self::NotRequired => 0,
            Self::Required => 1,
            Self::InProgress => 2,
            Self::Accepted => 3,
            Self::RequiresRecovery => 4,
            Self::Rejected => 5,
        }
    }

    /// Decodes a stable ordinal.
    #[must_use]
    pub const fn from_ordinal(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::NotRequired),
            1 => Some(Self::Required),
            2 => Some(Self::InProgress),
            3 => Some(Self::Accepted),
            4 => Some(Self::RequiresRecovery),
            5 => Some(Self::Rejected),
            _ => None,
        }
    }

    /// Returns whether verification has positively accepted the result.
    #[must_use]
    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accepted)
    }

    /// Returns whether verification requires another recovery/replan.
    #[must_use]
    pub const fn requires_recovery(self) -> bool {
        matches!(self, Self::RequiresRecovery)
    }
}

impl fmt::Display for RecoveryVerification {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Default for RecoveryVerification {
    fn default() -> Self {
        Self::Required
    }
}

// ============================================================================
// Recovery outcome
// ============================================================================

/// Terminal outcome of a recovery operation.
///
/// This is deliberately separate from `RecoveryState` because future
/// verification layers may need to distinguish operational terminal state from
/// semantic outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RecoveryOutcome {
    /// Recovery succeeded and verification accepted it.
    Succeeded,

    /// Recovery failed.
    Failed,

    /// Automatic recovery stopped and escalation is required.
    Escalated,

    /// Recovery was explicitly cancelled.
    Cancelled,
}

impl RecoveryOutcome {
    /// Returns a stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Escalated => "escalated",
            Self::Cancelled => "cancelled",
        }
    }

    /// Returns the corresponding terminal recovery state.
    #[must_use]
    pub const fn state(self) -> RecoveryState {
        match self {
            Self::Succeeded => RecoveryState::Succeeded,
            Self::Failed => RecoveryState::Failed,
            Self::Escalated => RecoveryState::Escalated,
            Self::Cancelled => RecoveryState::Cancelled,
        }
    }
}

impl fmt::Display for RecoveryOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// Recovery transition
// ============================================================================

/// A validated recovery-state transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RecoveryTransition {
    from: RecoveryState,
    to: RecoveryState,
}

impl RecoveryTransition {
    /// Creates a validated transition.
    pub const fn new(
        from: RecoveryState,
        to: RecoveryState,
    ) -> Result<Self, RecoveryStateError> {
        if !is_valid_transition(from, to) {
            return Err(RecoveryStateError::InvalidTransition { from, to });
        }

        Ok(Self { from, to })
    }

    /// Returns the source state.
    #[must_use]
    pub const fn from(self) -> RecoveryState {
        self.from
    }

    /// Returns the destination state.
    #[must_use]
    pub const fn to(self) -> RecoveryState {
        self.to
    }
}

// ============================================================================
// Recovery operation entry
// ============================================================================

/// Complete operational state for one recovery operation.
///
/// This structure intentionally contains no provider-specific execution
/// objects. It is a state record and therefore remains serializable by the
/// separate resilience serialization layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveryOperation {
    id: RecoveryOperationId,
    state: RecoveryState,
    generation: u64,
    attempt: u64,
    plan_generation: u64,
    verification: RecoveryVerification,
    affected_logical_qubits: Vec<QubitId>,
    outcome: Option<RecoveryOutcome>,
}

impl RecoveryOperation {
    /// Creates a new recovery operation.
    ///
    /// The operation begins in `Planned`.
    ///
    /// No logical resources are assumed. The caller explicitly supplies the
    /// affected logical qubits later.
    #[must_use]
    pub fn new(id: RecoveryOperationId) -> Self {
        Self {
            id,
            state: RecoveryState::Planned,
            generation: 0,
            attempt: 0,
            plan_generation: 0,
            verification: RecoveryVerification::Required,
            affected_logical_qubits: Vec::new(),
            outcome: None,
        }
    }

    /// Creates an operation from explicit state components.
    ///
    /// Intended for checkpoint restoration and deterministic deserialization.
    ///
    /// The supplied state is validated for semantic consistency.
    pub fn from_parts(
        id: RecoveryOperationId,
        state: RecoveryState,
        generation: u64,
        attempt: u64,
        plan_generation: u64,
        verification: RecoveryVerification,
        affected_logical_qubits: Vec<QubitId>,
        outcome: Option<RecoveryOutcome>,
    ) -> Result<Self, RecoveryStateError> {
        validate_operation_parts(state, generation, verification, outcome)?;

        let mut qubits = affected_logical_qubits;
        qubits.sort();
        qubits.dedup();

        Ok(Self {
            id,
            state,
            generation,
            attempt,
            plan_generation,
            verification,
            affected_logical_qubits: qubits,
            outcome,
        })
    }

    /// Returns the operation identifier.
    #[must_use]
    pub const fn id(&self) -> RecoveryOperationId {
        self.id
    }

    /// Returns the current recovery state.
    #[must_use]
    pub const fn state(&self) -> RecoveryState {
        self.state
    }

    /// Returns the operation generation.
    ///
    /// The generation increases after each successful state mutation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the number of recovery attempts started.
    #[must_use]
    pub const fn attempt(&self) -> u64 {
        self.attempt
    }

    /// Returns the generation of the recovery plan currently associated with
    /// this operation.
    #[must_use]
    pub const fn plan_generation(&self) -> u64 {
        self.plan_generation
    }

    /// Returns the verification status.
    #[must_use]
    pub const fn verification(&self) -> RecoveryVerification {
        self.verification
    }

    /// Returns the terminal outcome, if one exists.
    #[must_use]
    pub const fn outcome(&self) -> Option<RecoveryOutcome> {
        self.outcome
    }

    /// Returns the logical qubits affected by this recovery operation.
    ///
    /// The returned slice is sorted by canonical `QubitId` order.
    #[must_use]
    pub fn affected_logical_qubits(&self) -> &[QubitId] {
        &self.affected_logical_qubits
    }

    /// Returns whether the operation is terminal.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    /// Returns whether the operation may be executed.
    #[must_use]
    pub const fn is_executable(&self) -> bool {
        matches!(
            self.state,
            RecoveryState::Authorized
                | RecoveryState::Adapting
                | RecoveryState::Recovering
        )
    }

    /// Returns whether the current plan generation is stale relative to the
    /// supplied expected generation.
    #[must_use]
    pub const fn is_plan_stale(&self, expected_generation: u64) -> bool {
        self.plan_generation != expected_generation
    }

    /// Returns whether a logical qubit is affected by this operation.
    #[must_use]
    pub fn affects(&self, qubit: QubitId) -> bool {
        self.affected_logical_qubits.binary_search(&qubit).is_ok()
    }
}

// ============================================================================
// State store
// ============================================================================

/// Deterministic collection of recovery operations.
///
/// `BTreeMap` provides stable iteration order and avoids requiring a hash
/// function for operation identities.
///
/// The store itself is deliberately not synchronized.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RecoveryStateStore {
    operations: BTreeMap<RecoveryOperationId, RecoveryOperation>,
    revision: u64,
}

impl RecoveryStateStore {
    /// Creates an empty recovery state store.
    #[must_use]
    pub fn new() -> Self {
        Self {
            operations: BTreeMap::new(),
            revision: 0,
        }
    }

    /// Returns the current store revision.
    ///
    /// The revision changes after every successful observable mutation.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns the number of recovery operations currently tracked.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether no recovery operations are tracked.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Registers a new recovery operation.
    ///
    /// New operations always begin in `Planned`.
    pub fn register(
        &mut self,
        id: RecoveryOperationId,
    ) -> Result<&RecoveryOperation, RecoveryStateError> {
        if self.operations.contains_key(&id) {
            return Err(RecoveryStateError::AlreadyExists { id });
        }

        let operation = RecoveryOperation::new(id);

        self.ensure_revision_available()?;

        self.operations.insert(id, operation);
        self.revision += 1;

        Ok(self
            .operations
            .get(&id)
            .expect("inserted recovery operation must exist"))
    }

    /// Registers a fully specified recovery operation.
    ///
    /// Intended for checkpoint restoration and state replication.
    pub fn register_restored(
        &mut self,
        operation: RecoveryOperation,
    ) -> Result<(), RecoveryStateError> {
        let id = operation.id;

        if self.operations.contains_key(&id) {
            return Err(RecoveryStateError::AlreadyExists { id });
        }

        self.ensure_revision_available()?;

        self.operations.insert(id, operation);
        self.revision += 1;

        Ok(())
    }

    /// Returns a recovery operation by identifier.
    #[must_use]
    pub fn get(&self, id: RecoveryOperationId) -> Option<&RecoveryOperation> {
        self.operations.get(&id)
    }

    /// Returns the state of a recovery operation.
    #[must_use]
    pub fn state(&self, id: RecoveryOperationId) -> Option<RecoveryState> {
        self.operations.get(&id).map(RecoveryOperation::state)
    }

    /// Returns the operation generation.
    #[must_use]
    pub fn generation(&self, id: RecoveryOperationId) -> Option<u64> {
        self.operations.get(&id).map(RecoveryOperation::generation)
    }

    /// Returns an ordered iterator over all recovery operations.
    ///
    /// Ordering is by `RecoveryOperationId`.
    pub fn iter(&self) -> impl Iterator<Item = (&RecoveryOperationId, &RecoveryOperation)> {
        self.operations.iter()
    }

    /// Returns an iterator over operations in a particular state.
    pub fn iter_state(
        &self,
        state: RecoveryState,
    ) -> impl Iterator<Item = (&RecoveryOperationId, &RecoveryOperation)> {
        self.operations
            .iter()
            .filter(move |(_, operation)| operation.state == state)
    }

    /// Returns the IDs of all operations currently in a state.
    ///
    /// The caller controls the size of the resulting allocation through the
    /// state population.
    pub fn operation_ids_in_state(&self, state: RecoveryState) -> Vec<RecoveryOperationId> {
        self.iter_state(state)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Associates a recovery plan generation with an operation.
    ///
    /// The generation must change whenever the caller establishes a new plan
    /// against a new state snapshot.
    pub fn set_plan_generation(
        &mut self,
        id: RecoveryOperationId,
        plan_generation: u64,
    ) -> Result<(), RecoveryStateError> {
        let operation = self.operation_mut(id)?;

        if operation.is_terminal() {
            return Err(RecoveryStateError::TerminalMutation { id });
        }

        if operation.plan_generation == plan_generation {
            return Ok(());
        }

        self.ensure_revision_available()?;

        operation.plan_generation = plan_generation;
        increment_generation(operation)?;
        self.revision += 1;

        Ok(())
    }

    /// Adds one logical qubit to the operation's affected-resource set.
    ///
    /// The collection remains sorted and deduplicated.
    pub fn add_affected_qubit(
        &mut self,
        id: RecoveryOperationId,
        qubit: QubitId,
    ) -> Result<bool, RecoveryStateError> {
        let operation = self.operation_mut(id)?;

        if operation.is_terminal() {
            return Err(RecoveryStateError::TerminalMutation { id });
        }

        match operation.affected_logical_qubits.binary_search(&qubit) {
            Ok(_) => Ok(false),
            Err(index) => {
                self.ensure_revision_available()?;

                operation.affected_logical_qubits.insert(index, qubit);
                increment_generation(operation)?;
                self.revision += 1;

                Ok(true)
            }
        }
    }

    /// Adds multiple logical qubits to an operation.
    ///
    /// Input order does not affect resulting state.
    ///
    /// Returns the number of newly inserted qubits.
    pub fn add_affected_qubits<I>(
        &mut self,
        id: RecoveryOperationId,
        qubits: I,
    ) -> Result<usize, RecoveryStateError>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let mut incoming: Vec<QubitId> = qubits.into_iter().collect();
        incoming.sort();
        incoming.dedup();

        if incoming.is_empty() {
            return Ok(0);
        }

        let operation = self.operation_mut(id)?;

        if operation.is_terminal() {
            return Err(RecoveryStateError::TerminalMutation { id });
        }

        let mut merged = Vec::with_capacity(
            operation
                .affected_logical_qubits
                .len()
                .saturating_add(incoming.len()),
        );

        let mut existing_index = 0usize;
        let mut incoming_index = 0usize;
        let mut added = 0usize;

        while existing_index < operation.affected_logical_qubits.len()
            && incoming_index < incoming.len()
        {
            let existing = operation.affected_logical_qubits[existing_index];
            let candidate = incoming[incoming_index];

            if existing < candidate {
                merged.push(existing);
                existing_index += 1;
            } else if candidate < existing {
                merged.push(candidate);
                incoming_index += 1;
                added += 1;
            } else {
                merged.push(existing);
                existing_index += 1;
                incoming_index += 1;
            }
        }

        while existing_index < operation.affected_logical_qubits.len() {
            merged.push(operation.affected_logical_qubits[existing_index]);
            existing_index += 1;
        }

        while incoming_index < incoming.len() {
            merged.push(incoming[incoming_index]);
            incoming_index += 1;
            added += 1;
        }

        if added == 0 {
            return Ok(0);
        }

        self.ensure_revision_available()?;

        operation.affected_logical_qubits = merged;
        increment_generation(operation)?;
        self.revision += 1;

        Ok(added)
    }

    /// Records the start of a new recovery attempt.
    ///
    /// Attempts are monotonically increasing and are never reset.
    pub fn begin_attempt(
        &mut self,
        id: RecoveryOperationId,
    ) -> Result<u64, RecoveryStateError> {
        let operation = self.operation_mut(id)?;

        if operation.is_terminal() {
            return Err(RecoveryStateError::TerminalMutation { id });
        }

        if !matches!(
            operation.state,
            RecoveryState::Authorized
                | RecoveryState::Adapting
                | RecoveryState::Recovering
        ) {
            return Err(RecoveryStateError::AttemptNotAllowed {
                id,
                state: operation.state,
            });
        }

        let next_attempt = operation
            .attempt
            .checked_add(1)
            .ok_or(RecoveryStateError::AttemptOverflow { id })?;

        self.ensure_revision_available()?;

        operation.attempt = next_attempt;
        increment_generation(operation)?;
        self.revision += 1;

        Ok(next_attempt)
    }

    /// Applies a validated state transition.
    ///
    /// Success is only permitted from `Verifying` when verification has been
    /// accepted.
    pub fn transition(
        &mut self,
        id: RecoveryOperationId,
        target: RecoveryState,
    ) -> Result<RecoveryTransition, RecoveryStateError> {
        let operation = self.operation_mut(id)?;
        let transition = RecoveryTransition::new(operation.state, target)?;

        if target == RecoveryState::Succeeded
            && operation.verification != RecoveryVerification::Accepted
        {
            return Err(RecoveryStateError::VerificationRequired { id });
        }

        if target == RecoveryState::Verifying {
            if operation.attempt == 0 {
                return Err(RecoveryStateError::AttemptRequired { id });
            }

            if operation.verification == RecoveryVerification::NotRequired {
                return Err(RecoveryStateError::VerificationRequired { id });
            }
        }

        self.ensure_revision_available()?;

        operation.state = target;

        if target == RecoveryState::Verifying {
            operation.verification = RecoveryVerification::InProgress;
        }

        if let Some(outcome) = terminal_outcome_for_state(target) {
            operation.outcome = Some(outcome);
        }

        increment_generation(operation)?;
        self.revision += 1;

        Ok(transition)
    }

    /// Applies a state transition only if the caller's generation still matches
    /// the operation's generation.
    ///
    /// This is the preferred API when a recovery worker is operating from a
    /// previously observed state snapshot.
    pub fn transition_if_generation(
        &mut self,
        id: RecoveryOperationId,
        expected_generation: u64,
        target: RecoveryState,
    ) -> Result<RecoveryTransition, RecoveryStateError> {
        let current_generation = self
            .operations
            .get(&id)
            .ok_or(RecoveryStateError::NotFound { id })?
            .generation;

        if current_generation != expected_generation {
            return Err(RecoveryStateError::GenerationMismatch {
                id,
                expected: expected_generation,
                actual: current_generation,
            });
        }

        self.transition(id, target)
    }

    /// Updates verification state.
    ///
    /// Only the verification boundary should call this API.
    pub fn set_verification(
        &mut self,
        id: RecoveryOperationId,
        verification: RecoveryVerification,
    ) -> Result<(), RecoveryStateError> {
        let operation = self.operation_mut(id)?;

        if operation.is_terminal() {
            return Err(RecoveryStateError::TerminalMutation { id });
        }

        if verification == RecoveryVerification::Accepted
            && operation.state != RecoveryState::Verifying
        {
            return Err(RecoveryStateError::VerificationStateMismatch {
                id,
                state: operation.state,
                verification,
            });
        }

        if verification == RecoveryVerification::InProgress
            && operation.state != RecoveryState::Verifying
        {
            return Err(RecoveryStateError::VerificationStateMismatch {
                id,
                state: operation.state,
                verification,
            });
        }

        self.ensure_revision_available()?;

        operation.verification = verification;
        increment_generation(operation)?;
        self.revision += 1;

        Ok(())
    }

    /// Marks a verification failure that requires another recovery/replan
    /// cycle.
    ///
    /// The operation itself returns to `Planned`; actual replanning belongs to
    /// the planning subsystem.
    pub fn require_recovery(
        &mut self,
        id: RecoveryOperationId,
    ) -> Result<(), RecoveryStateError> {
        let operation = self.operation_mut(id)?;

        if operation.is_terminal() {
            return Err(RecoveryStateError::TerminalMutation { id });
        }

        if operation.state != RecoveryState::Verifying {
            return Err(RecoveryStateError::VerificationStateMismatch {
                id,
                state: operation.state,
                verification: RecoveryVerification::RequiresRecovery,
            });
        }

        self.ensure_revision_available()?;

        operation.verification = RecoveryVerification::RequiresRecovery;
        operation.state = RecoveryState::Planned;
        operation.outcome = None;
        increment_generation(operation)?;
        self.revision += 1;

        Ok(())
    }

    /// Marks verification as rejected and terminates the recovery operation.
    pub fn reject_verification(
        &mut self,
        id: RecoveryOperationId,
    ) -> Result<(), RecoveryStateError> {
        let operation = self.operation_mut(id)?;

        if operation.is_terminal() {
            return Err(RecoveryStateError::TerminalMutation { id });
        }

        if operation.state != RecoveryState::Verifying {
            return Err(RecoveryStateError::VerificationStateMismatch {
                id,
                state: operation.state,
                verification: RecoveryVerification::Rejected,
            });
        }

        self.ensure_revision_available()?;

        operation.verification = RecoveryVerification::Rejected;
        operation.state = RecoveryState::Failed;
        operation.outcome = Some(RecoveryOutcome::Failed);
        increment_generation(operation)?;
        self.revision += 1;

        Ok(())
    }

    /// Applies a terminal outcome.
    ///
    /// Successful completion is forbidden unless verification has accepted the
    /// recovery.
    pub fn complete(
        &mut self,
        id: RecoveryOperationId,
        outcome: RecoveryOutcome,
    ) -> Result<(), RecoveryStateError> {
        let operation = self.operation_mut(id)?;

        if operation.is_terminal() {
            return Err(RecoveryStateError::TerminalMutation { id });
        }

        if outcome == RecoveryOutcome::Succeeded {
            if operation.state != RecoveryState::Verifying {
                return Err(RecoveryStateError::VerificationRequired { id });
            }

            if operation.verification != RecoveryVerification::Accepted {
                return Err(RecoveryStateError::VerificationRequired { id });
            }
        }

        self.ensure_revision_available()?;

        operation.state = outcome.state();
        operation.outcome = Some(outcome);

        increment_generation(operation)?;
        self.revision += 1;

        Ok(())
    }

    /// Escalates a non-terminal recovery operation.
    pub fn escalate(
        &mut self,
        id: RecoveryOperationId,
    ) -> Result<(), RecoveryStateError> {
        self.complete(id, RecoveryOutcome::Escalated)
    }

    /// Cancels a non-terminal recovery operation.
    pub fn cancel(
        &mut self,
        id: RecoveryOperationId,
    ) -> Result<(), RecoveryStateError> {
        self.complete(id, RecoveryOutcome::Cancelled)
    }

    /// Removes a terminal operation from the active store.
    ///
    /// Removal is deliberately allowed only for terminal operations.
    ///
    /// History/checkpoint layers should persist the operation before removing
    /// it if long-term auditability is required.
    pub fn remove(
        &mut self,
        id: RecoveryOperationId,
    ) -> Result<RecoveryOperation, RecoveryStateError> {
        let operation = self
            .operations
            .get(&id)
            .ok_or(RecoveryStateError::NotFound { id })?;

        if !operation.is_terminal() {
            return Err(RecoveryStateError::NonTerminalRemoval { id });
        }

        self.ensure_revision_available()?;

        let removed = self
            .operations
            .remove(&id)
            .expect("operation checked above must exist");

        self.revision += 1;

        Ok(removed)
    }

    /// Applies several transitions atomically.
    ///
    /// If any transition fails, every previous mutation is rolled back.
    ///
    /// The returned transitions preserve input order.
    pub fn transition_batch<I>(
        &mut self,
        transitions: I,
    ) -> Result<Vec<RecoveryTransition>, RecoveryStateError>
    where
        I: IntoIterator<Item = (RecoveryOperationId, RecoveryState)>,
    {
        let requests: Vec<(RecoveryOperationId, RecoveryState)> =
            transitions.into_iter().collect();

        if requests.is_empty() {
            return Ok(Vec::new());
        }

        let original_revision = self.revision;

        let mut journal: Vec<(RecoveryOperationId, RecoveryOperation)> =
            Vec::with_capacity(requests.len());

        let mut results = Vec::with_capacity(requests.len());

        for (id, target) in requests {
            let previous = match self.operations.get(&id) {
                Some(operation) => operation.clone(),
                None => {
                    self.rollback_journal(journal, original_revision);
                    return Err(RecoveryStateError::NotFound { id });
                }
            };

            journal.push((id, previous));

            match self.transition(id, target) {
                Ok(transition) => results.push(transition),
                Err(error) => {
                    self.rollback_journal(journal, original_revision);
                    return Err(error);
                }
            }
        }

        Ok(results)
    }

    /// Creates a complete in-memory snapshot.
    ///
    /// The snapshot is intentionally an explicit, caller-requested operation.
    /// Callers processing extremely large state sets should use `iter()` or a
    /// streaming persistence mechanism rather than repeatedly cloning the
    /// complete store.
    #[must_use]
    pub fn snapshot(&self) -> RecoveryStateSnapshot {
        RecoveryStateSnapshot {
            revision: self.revision,
            operations: self.operations.clone(),
        }
    }

    /// Restores a complete snapshot.
    ///
    /// Existing state is replaced only after the snapshot has been validated.
    pub fn restore(
        &mut self,
        snapshot: RecoveryStateSnapshot,
    ) -> Result<(), RecoveryStateError> {
        validate_snapshot(&snapshot)?;

        self.operations = snapshot.operations;
        self.revision = snapshot.revision;

        Ok(())
    }

    /// Restores a snapshot only when the caller's expected current revision
    /// matches.
    pub fn restore_if_revision(
        &mut self,
        expected_revision: u64,
        snapshot: RecoveryStateSnapshot,
    ) -> Result<(), RecoveryStateError> {
        if self.revision != expected_revision {
            return Err(RecoveryStateError::RevisionMismatch {
                expected: expected_revision,
                actual: self.revision,
            });
        }

        self.restore(snapshot)
    }

    fn operation_mut(
        &mut self,
        id: RecoveryOperationId,
    ) -> Result<&mut RecoveryOperation, RecoveryStateError> {
        self.operations
            .get_mut(&id)
            .ok_or(RecoveryStateError::NotFound { id })
    }

    fn ensure_revision_available(&self) -> Result<(), RecoveryStateError> {
        if self.revision == u64::MAX {
            return Err(RecoveryStateError::RevisionOverflow);
        }

        Ok(())
    }

    fn rollback_journal(
        &mut self,
        journal: Vec<(RecoveryOperationId, RecoveryOperation)>,
        revision: u64,
    ) {
        for (id, operation) in journal.into_iter().rev() {
            self.operations.insert(id, operation);
        }

        self.revision = revision;
    }
}

// ============================================================================
// Snapshot
// ============================================================================

/// Immutable snapshot of the recovery-operation state store.
///
/// This type is persistence-neutral. The serialization subsystem decides how
/// it is encoded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecoveryStateSnapshot {
    revision: u64,
    operations: BTreeMap<RecoveryOperationId, RecoveryOperation>,
}

impl RecoveryStateSnapshot {
    /// Returns the snapshot revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns the number of operations in the snapshot.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the snapshot contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns an operation from the snapshot.
    #[must_use]
    pub fn get(&self, id: RecoveryOperationId) -> Option<&RecoveryOperation> {
        self.operations.get(&id)
    }

    /// Returns ordered snapshot entries.
    pub fn iter(&self) -> impl Iterator<Item = (&RecoveryOperationId, &RecoveryOperation)> {
        self.operations.iter()
    }
}

// ============================================================================
// Transition validation
// ============================================================================

/// Determines whether a state transition is structurally legal.
///
/// Semantic checks such as verification acceptance are performed by the state
/// store because they require operation-local data.
#[must_use]
pub const fn is_valid_transition(from: RecoveryState, to: RecoveryState) -> bool {
    match from {
        RecoveryState::Planned => matches!(
            to,
            RecoveryState::Authorized
                | RecoveryState::Adapting
                | RecoveryState::Escalated
                | RecoveryState::Cancelled
        ),

        RecoveryState::Authorized => matches!(
            to,
            RecoveryState::Adapting
                | RecoveryState::Recovering
                | RecoveryState::Planned
                | RecoveryState::Escalated
                | RecoveryState::Cancelled
        ),

        RecoveryState::Adapting => matches!(
            to,
            RecoveryState::Recovering
                | RecoveryState::Planned
                | RecoveryState::Escalated
                | RecoveryState::Failed
                | RecoveryState::Cancelled
        ),

        RecoveryState::Recovering => matches!(
            to,
            RecoveryState::Verifying
                | RecoveryState::Planned
                | RecoveryState::Authorized
                | RecoveryState::Escalated
                | RecoveryState::Failed
                | RecoveryState::Cancelled
        ),

        RecoveryState::Verifying => matches!(
            to,
            RecoveryState::Succeeded
                | RecoveryState::Planned
                | RecoveryState::Escalated
                | RecoveryState::Failed
                | RecoveryState::Cancelled
        ),

        RecoveryState::Succeeded
        | RecoveryState::Failed
        | RecoveryState::Escalated
        | RecoveryState::Cancelled => false,
    }
}

fn terminal_outcome_for_state(state: RecoveryState) -> Option<RecoveryOutcome> {
    match state {
        RecoveryState::Succeeded => Some(RecoveryOutcome::Succeeded),
        RecoveryState::Failed => Some(RecoveryOutcome::Failed),
        RecoveryState::Escalated => Some(RecoveryOutcome::Escalated),
        RecoveryState::Cancelled => Some(RecoveryOutcome::Cancelled),
        RecoveryState::Planned
        | RecoveryState::Authorized
        | RecoveryState::Adapting
        | RecoveryState::Recovering
        | RecoveryState::Verifying => None,
    }
}

fn increment_generation(
    operation: &mut RecoveryOperation,
) -> Result<(), RecoveryStateError> {
    operation.generation = operation
        .generation
        .checked_add(1)
        .ok_or(RecoveryStateError::GenerationOverflow {
            id: operation.id,
        })?;

    Ok(())
}

fn validate_operation_parts(
    state: RecoveryState,
    _generation: u64,
    verification: RecoveryVerification,
    outcome: Option<RecoveryOutcome>,
) -> Result<(), RecoveryStateError> {
    if state.is_terminal() {
        let expected = terminal_outcome_for_state(state);

        if outcome != expected {
            return Err(RecoveryStateError::InconsistentTerminalOutcome {
                state,
                outcome,
            });
        }
    } else if outcome.is_some() {
        return Err(RecoveryStateError::NonTerminalHasOutcome { state });
    }

    if state == RecoveryState::Succeeded && !verification.is_accepted() {
        return Err(RecoveryStateError::InconsistentVerificationState {
            state,
            verification,
        });
    }

    Ok(())
}

fn validate_snapshot(
    snapshot: &RecoveryStateSnapshot,
) -> Result<(), RecoveryStateError> {
    for (id, operation) in &snapshot.operations {
        if *id != operation.id {
            return Err(RecoveryStateError::SnapshotIdentityMismatch {
                key: *id,
                operation: operation.id,
            });
        }

        validate_operation_parts(
            operation.state,
            operation.generation,
            operation.verification,
            operation.outcome,
        )?;

        if operation
            .affected_logical_qubits
            .windows(2)
            .any(|pair| pair[0] >= pair[1])
        {
            return Err(RecoveryStateError::UnsortedLogicalResources {
                id: operation.id,
            });
        }
    }

    Ok(())
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the recovery state model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecoveryStateError {
    /// An operation with the same identifier already exists.
    AlreadyExists {
        /// Conflicting operation identifier.
        id: RecoveryOperationId,
    },

    /// Requested operation does not exist.
    NotFound {
        /// Missing operation identifier.
        id: RecoveryOperationId,
    },

    /// The requested state transition is not structurally legal.
    InvalidTransition {
        /// Current state.
        from: RecoveryState,

        /// Requested destination state.
        to: RecoveryState,
    },

    /// A terminal operation cannot be mutated.
    TerminalMutation {
        /// Operation identifier.
        id: RecoveryOperationId,
    },

    /// A non-terminal operation cannot be removed.
    NonTerminalRemoval {
        /// Operation identifier.
        id: RecoveryOperationId,
    },

    /// A recovery attempt was requested in an invalid state.
    AttemptNotAllowed {
        /// Operation identifier.
        id: RecoveryOperationId,

        /// Current operation state.
        state: RecoveryState,
    },

    /// Attempt counter overflowed.
    AttemptOverflow {
        /// Operation identifier.
        id: RecoveryOperationId,
    },

    /// Operation generation overflowed.
    GenerationOverflow {
        /// Operation identifier.
        id: RecoveryOperationId,
    },

    /// Store revision overflowed.
    RevisionOverflow,

    /// A transition was requested using a stale operation generation.
    GenerationMismatch {
        /// Operation identifier.
        id: RecoveryOperationId,

        /// Generation observed by the caller.
        expected: u64,

        /// Current generation.
        actual: u64,
    },

    /// Store revision did not match the caller's expectation.
    RevisionMismatch {
        /// Expected store revision.
        expected: u64,

        /// Actual store revision.
        actual: u64,
    },

    /// Verification is mandatory before successful completion.
    VerificationRequired {
        /// Operation identifier.
        id: RecoveryOperationId,
    },

    /// At least one recovery attempt must exist before verification begins.
    AttemptRequired {
        /// Operation identifier.
        id: RecoveryOperationId,
    },

    /// Verification status is incompatible with the operation state.
    VerificationStateMismatch {
        /// Operation identifier.
        id: RecoveryOperationId,

        /// Current operation state.
        state: RecoveryState,

        /// Requested verification state.
        verification: RecoveryVerification,
    },

    /// Snapshot contains a terminal state with an incompatible outcome.
    InconsistentTerminalOutcome {
        /// Snapshot state.
        state: RecoveryState,

        /// Snapshot outcome.
        outcome: Option<RecoveryOutcome>,
    },

    /// Snapshot contains an outcome on a non-terminal operation.
    NonTerminalHasOutcome {
        /// Snapshot state.
        state: RecoveryState,
    },

    /// Snapshot contains an impossible state/verification combination.
    InconsistentVerificationState {
        /// Snapshot state.
        state: RecoveryState,

        /// Snapshot verification state.
        verification: RecoveryVerification,
    },

    /// Snapshot map key and contained operation ID disagree.
    SnapshotIdentityMismatch {
        /// Map key.
        key: RecoveryOperationId,

        /// Operation's own identity.
        operation: RecoveryOperationId,
    },

    /// Logical-qubit IDs in a snapshot are not strictly ordered and unique.
    UnsortedLogicalResources {
        /// Operation containing invalid logical-resource ordering.
        id: RecoveryOperationId,
    },
}

impl fmt::Display for RecoveryStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyExists { id } => {
                write!(formatter, "recovery operation {id} already exists")
            }

            Self::NotFound { id } => {
                write!(formatter, "recovery operation {id} was not found")
            }

            Self::InvalidTransition { from, to } => {
                write!(formatter, "invalid recovery transition: {from} -> {to}")
            }

            Self::TerminalMutation { id } => {
                write!(formatter, "terminal recovery operation {id} cannot be mutated")
            }

            Self::NonTerminalRemoval { id } => {
                write!(formatter, "non-terminal recovery operation {id} cannot be removed")
            }

            Self::AttemptNotAllowed { id, state } => {
                write!(
                    formatter,
                    "recovery attempt cannot start for operation {id} in state {state}"
                )
            }

            Self::AttemptOverflow { id } => {
                write!(formatter, "recovery attempt counter overflowed for operation {id}")
            }

            Self::GenerationOverflow { id } => {
                write!(formatter, "recovery generation overflowed for operation {id}")
            }

            Self::RevisionOverflow => {
                formatter.write_str("recovery state-store revision overflowed")
            }

            Self::GenerationMismatch {
                id,
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "recovery operation {id} generation mismatch: expected {expected}, actual {actual}"
                )
            }

            Self::RevisionMismatch { expected, actual } => {
                write!(
                    formatter,
                    "recovery state revision mismatch: expected {expected}, actual {actual}"
                )
            }

            Self::VerificationRequired { id } => {
                write!(
                    formatter,
                    "recovery operation {id} requires accepted verification before success"
                )
            }

            Self::AttemptRequired { id } => {
                write!(
                    formatter,
                    "recovery operation {id} requires at least one attempt before verification"
                )
            }

            Self::VerificationStateMismatch {
                id,
                state,
                verification,
            } => {
                write!(
                    formatter,
                    "verification state {verification} is invalid for recovery operation {id} in state {state}"
                )
            }

            Self::InconsistentTerminalOutcome { state, outcome } => {
                write!(
                    formatter,
                    "terminal recovery state {state} has incompatible outcome {outcome:?}"
                )
            }

            Self::NonTerminalHasOutcome { state } => {
                write!(
                    formatter,
                    "non-terminal recovery state {state} cannot have a terminal outcome"
                )
            }

            Self::InconsistentVerificationState {
                state,
                verification,
            } => {
                write!(
                    formatter,
                    "recovery state {state} has incompatible verification state {verification}"
                )
            }

            Self::SnapshotIdentityMismatch { key, operation } => {
                write!(
                    formatter,
                    "snapshot identity mismatch: map key {key} != operation {operation}"
                )
            }

            Self::UnsortedLogicalResources { id } => {
                write!(
                    formatter,
                    "recovery operation {id} contains unordered or duplicate logical resources"
                )
            }
        }
    }
}

impl Error for RecoveryStateError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u128) -> RecoveryOperationId {
        RecoveryOperationId::new(value)
    }

    #[test]
    fn state_ordinals_are_stable_and_round_trip() {
        let states = [
            RecoveryState::Planned,
            RecoveryState::Authorized,
            RecoveryState::Adapting,
            RecoveryState::Recovering,
            RecoveryState::Verifying,
            RecoveryState::Succeeded,
            RecoveryState::Failed,
            RecoveryState::Escalated,
            RecoveryState::Cancelled,
        ];

        for state in states {
            assert_eq!(RecoveryState::from_ordinal(state.ordinal()), Some(state));
        }
    }

    #[test]
    fn terminal_states_cannot_transition() {
        assert!(!is_valid_transition(
            RecoveryState::Succeeded,
            RecoveryState::Planned
        ));

        assert!(!is_valid_transition(
            RecoveryState::Failed,
            RecoveryState::Recovering
        ));

        assert!(!is_valid_transition(
            RecoveryState::Escalated,
            RecoveryState::Authorized
        ));

        assert!(!is_valid_transition(
            RecoveryState::Cancelled,
            RecoveryState::Planned
        ));
    }

    #[test]
    fn success_requires_verification() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();

        store
            .transition(id(1), RecoveryState::Authorized)
            .unwrap();

        store
            .transition(id(1), RecoveryState::Recovering)
            .unwrap();

        store.begin_attempt(id(1)).unwrap();

        store
            .transition(id(1), RecoveryState::Verifying)
            .unwrap();

        let error = store
            .transition(id(1), RecoveryState::Succeeded)
            .unwrap_err();

        assert_eq!(
            error,
            RecoveryStateError::VerificationRequired { id: id(1) }
        );
    }

    #[test]
    fn accepted_verification_allows_success() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();

        store
            .transition(id(1), RecoveryState::Authorized)
            .unwrap();

        store
            .transition(id(1), RecoveryState::Recovering)
            .unwrap();

        store.begin_attempt(id(1)).unwrap();

        store
            .transition(id(1), RecoveryState::Verifying)
            .unwrap();

        store
            .set_verification(id(1), RecoveryVerification::Accepted)
            .unwrap();

        store
            .transition(id(1), RecoveryState::Succeeded)
            .unwrap();

        assert_eq!(
            store.state(id(1)),
            Some(RecoveryState::Succeeded)
        );
    }

    #[test]
    fn affected_qubits_are_sorted_and_deduplicated() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();

        let q1 = QubitId::from(7_u64);
        let q2 = QubitId::from(2_u64);
        let q3 = QubitId::from(7_u64);

        store
            .add_affected_qubits(id(1), [q1, q2, q3])
            .unwrap();

        let operation = store.get(id(1)).unwrap();

        assert_eq!(
            operation.affected_logical_qubits(),
            &[q2, q1]
        );
    }

    #[test]
    fn generation_changes_after_mutation() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();

        let generation = store.generation(id(1)).unwrap();

        store
            .transition(id(1), RecoveryState::Authorized)
            .unwrap();

        assert!(store.generation(id(1)).unwrap() > generation);
    }

    #[test]
    fn stale_generation_is_rejected() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();

        let generation = store.generation(id(1)).unwrap();

        store
            .transition(id(1), RecoveryState::Authorized)
            .unwrap();

        let error = store
            .transition_if_generation(
                id(1),
                generation,
                RecoveryState::Adapting,
            )
            .unwrap_err();

        assert!(matches!(
            error,
            RecoveryStateError::GenerationMismatch { .. }
        ));
    }

    #[test]
    fn verification_can_require_recovery() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();

        store
            .transition(id(1), RecoveryState::Authorized)
            .unwrap();

        store
            .transition(id(1), RecoveryState::Recovering)
            .unwrap();

        store.begin_attempt(id(1)).unwrap();

        store
            .transition(id(1), RecoveryState::Verifying)
            .unwrap();

        store
            .set_verification(
                id(1),
                RecoveryVerification::RequiresRecovery,
            )
            .unwrap();

        store.require_recovery(id(1)).unwrap();

        assert_eq!(
            store.state(id(1)),
            Some(RecoveryState::Planned)
        );
    }

    #[test]
    fn terminal_operations_can_be_removed() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();

        store
            .complete(id(1), RecoveryOutcome::Cancelled)
            .unwrap();

        let removed = store.remove(id(1)).unwrap();

        assert_eq!(removed.outcome(), Some(RecoveryOutcome::Cancelled));
        assert!(store.get(id(1)).is_none());
    }

    #[test]
    fn non_terminal_operations_cannot_be_removed() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();

        let error = store.remove(id(1)).unwrap_err();

        assert_eq!(
            error,
            RecoveryStateError::NonTerminalRemoval { id: id(1) }
        );
    }

    #[test]
    fn snapshot_round_trip_preserves_state() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();

        store
            .transition(id(1), RecoveryState::Authorized)
            .unwrap();

        let snapshot = store.snapshot();

        let mut restored = RecoveryStateStore::new();

        restored.restore(snapshot).unwrap();

        assert_eq!(restored, store);
    }

    #[test]
    fn batch_transition_rolls_back_on_failure() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();
        store.register(id(2)).unwrap();

        let before = store.clone();

        let error = store
            .transition_batch([
                (id(1), RecoveryState::Authorized),
                (id(2), RecoveryState::Succeeded),
            ])
            .unwrap_err();

        assert!(matches!(
            error,
            RecoveryStateError::VerificationRequired { .. }
        ));

        assert_eq!(store, before);
    }

    #[test]
    fn empty_batch_is_a_no_op() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();

        let before = store.clone();

        let result = store
            .transition_batch(std::iter::empty())
            .unwrap();

        assert!(result.is_empty());
        assert_eq!(store, before);
    }

    #[test]
    fn operation_affects_uses_canonical_qubit_identity() {
        let mut store = RecoveryStateStore::new();

        store.register(id(1)).unwrap();

        let qubit = QubitId::from(42_u64);

        store
            .add_affected_qubit(id(1), qubit)
            .unwrap();

        assert!(store.get(id(1)).unwrap().affects(qubit));
    }
}