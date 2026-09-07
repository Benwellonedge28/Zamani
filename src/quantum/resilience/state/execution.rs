//! Zamani Quantum Resilience — execution state.
//!
//! Path:
//!     src/quantum/resilience/state/execution.rs
//!
//! # Purpose
//!
//! This module owns the provider-neutral, execution-local state required by
//! the resilience subsystem.
//!
//! It records facts about an execution without implementing:
//!
//! - resilience lifecycle transitions;
//! - quantum fault semantics;
//! - QEC;
//! - routing;
//! - scheduling;
//! - compilation;
//! - optimization;
//! - hardware discovery;
//! - backend execution;
//! - checkpoint storage;
//! - persistence;
//! - telemetry collection;
//! - authorization;
//! - authentication.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Program
//!                               |
//!                               v
//!                       Canonical Quantum IR
//!                               |
//!                               v
//!                    +-----------------------+
//!                    |     Execution State   |
//!                    |                       |
//!                    | identity              |
//!                    | status                |
//!                    | attempts              |
//!                    | progress              |
//!                    | logical resources     |
//!                    | physical resources    |
//!                    | target generation     |
//!                    | semantic identity     |
//!                    +-----------+-----------+
//!                                |
//!              +-----------------+-----------------+
//!              |                 |                 |
//!              v                 v                 v
//!          state/machine     routing          scheduling
//!              |                 |                 |
//!              v                 v                 v
//!          resilience        placement        timing
//!              |                 |                 |
//!              +-----------------+-----------------+
//!                                |
//!                                v
//!                           execution
//! ```
//!
//! # Separation from `state/machine.rs`
//!
//! `state/machine.rs` answers:
//!
//!     "Which resilience lifecycle phase are we in?"
//!
//! This module answers:
//!
//!     "What is the execution currently doing and what execution facts are
//!      known?"
//!
//! These are deliberately different concepts.
//!
//! The lifecycle machine therefore remains the authoritative owner of:
//!
//!     Idle
//!     Detecting
//!     Diagnosing
//!     Planning
//!     Adapting
//!     Recovering
//!     Verifying
//!     Completed
//!     Escalated
//!     Failed
//!
//! This module does not redefine those states.
//!
//! # Canonical qubit identity
//!
//! When execution state needs to identify quantum resources, it MUST use the
//! canonical IR identity types:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! A `QubitId` represents logical program identity.
//!
//! A `PhysicalQubitId` represents a physical target identity.
//!
//! This module never converts one into the other.
//!
//! Routing/placement establishes the logical-to-physical relationship.
//!
//! In particular, this module MUST NOT assume:
//!
//!     logical q7 == physical p7
//!
//! merely because both identifiers may currently be integer-backed.
//!
//! # Write once / scale everywhere
//!
//! No machine-size assumption exists here.
//!
//! There is no:
//!
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_OPERATIONS
//!     MAX_SHOTS
//!     DEFAULT_RETRIES
//!     DEFAULT_DEVICE_SIZE
//!
//! Resource cardinality is represented by dynamically sized collections and
//! counters whose limits are determined by the execution environment,
//! configuration and applicable resource policies.
//!
//! The implementation therefore supports the same representation for:
//!
//!     one logical qubit
//!     |
//!     small QPU
//!     |
//!     large QPU
//!     |
//!     fault-tolerant logical machine
//!     |
//!     multi-QPU execution
//!     |
//!     distributed quantum execution
//!
//! "Infinite" means that this module imposes no artificial quantum-machine
//! ceiling. Every concrete execution remains finite and bounded by available
//! resources and representable state.
//!
//! # Determinism
//!
//! This module:
//!
//! - does not read clocks;
//! - does not generate randomness;
//! - does not use process-global mutable state;
//! - does not depend on hash-map iteration;
//! - uses ordered qubit collections;
//! - rejects counter overflow;
//! - does not silently discard state mutations.
//!
//! Identical ordered mutations applied to identical initial state produce
//! identical resulting state.
//!
//! # Persistence
//!
//! This type is intentionally persistence-neutral.
//!
//! `ExecutionStateSnapshot` contains owned values suitable for the persistence
//! and serialization layers, but this module does not choose a storage engine
//! or serialization format.
//!
//! `state/persistence.rs` and `serialization/*` own those concerns.
//!
//! # Security
//!
//! Execution state can become sensitive operational metadata.
//!
//! Callers MUST NOT place:
//!
//! - credentials;
//! - access tokens;
//! - private keys;
//! - passwords;
//! - provider secrets;
//! - raw memory addresses;
//! - device authentication material;
//! - private program data
//!
//! into free-form diagnostic strings.
//!
//! This module does not authenticate or authorize state changes.
//! Higher-level security and ownership boundaries must do so.
//!
//! # Rust contract
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Integration contract
//!
//! `state/machine.rs`
//!     Owns resilience lifecycle state.
//!
//! `state/logical.rs`
//!     Owns logical-resource health/state details.
//!
//! `state/physical.rs`
//!     Owns physical-resource health/state details.
//!
//! `state/persistence.rs`
//!     Owns durable persistence.
//!
//! `checkpoint/*`
//!     Owns checkpoint/reconstruction semantics.
//!
//! `routing/*`
//!     Establishes logical-to-physical placement.
//!
//! `scheduling/*`
//!     Establishes execution timing/order.
//!
//! `hardware/*`
//!     Supplies target capability and hardware state.
//!
//! `telemetry/*`
//!     Observes execution state changes.
//!
//! `verification/*`
//!     Determines whether an execution/result is semantically acceptable.
//!
//! `recovery/*`
//!     Performs recovery actions using this state as execution context.
//!
//! `api/*`
//!     Coordinates this state with the complete resilience lifecycle.
//!
//! The dependency direction is intentionally one-way:
//!
//!     execution state
//!          ^
//!          |
//!     orchestration / recovery / telemetry / persistence
//!
//! This prevents this foundational type from becoming coupled to concrete
//! implementations added later.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use std::collections::BTreeSet;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

// ============================================================================
// SCHEMA
// ============================================================================

/// Stable schema identifier for execution state.
pub const EXECUTION_STATE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.state.execution";

/// Durable semantic schema version.
///
/// Increment when the externally observable representation or semantics change
/// in a compatibility-relevant way.
pub const EXECUTION_STATE_SCHEMA_VERSION: u16 = 1;

/// Implementation version.
///
/// This is independent of the durable schema version.
pub const EXECUTION_STATE_VERSION: u32 = 1;

// ============================================================================
// EXECUTION STATUS
// ============================================================================

/// Operational state of an execution.
///
/// This is deliberately separate from [`crate::quantum::resilience::state::machine::ResilienceState`].
///
/// `ResilienceState` describes the resilience workflow.
///
/// `ExecutionStatus` describes the execution itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExecutionStatus {
    /// Execution has been created but has not started.
    Pending,

    /// Execution is actively executing.
    Running,

    /// Execution is temporarily stopped at a valid execution boundary.
    Suspended,

    /// A recovery action is actively reconstructing or restarting execution.
    Recovering,

    /// The execution/result is undergoing correctness verification.
    Verifying,

    /// Execution completed and its result has reached the execution layer's
    /// successful terminal condition.
    Completed,

    /// Execution terminated unsuccessfully.
    Failed,

    /// Execution was explicitly cancelled.
    Cancelled,

    /// Execution was stopped because higher-level intervention is required.
    Escalated,
}

impl ExecutionStatus {
    /// Returns the stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Suspended => "suspended",
            Self::Recovering => "recovering",
            Self::Verifying => "verifying",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Escalated => "escalated",
        }
    }

    /// Returns a stable ordinal for deterministic serialization.
    ///
    /// This ordinal is an identity of representation, not a severity score.
    #[must_use]
    pub const fn ordinal(self) -> u8 {
        match self {
            Self::Pending => 0,
            Self::Running => 1,
            Self::Suspended => 2,
            Self::Recovering => 3,
            Self::Verifying => 4,
            Self::Completed => 5,
            Self::Failed => 6,
            Self::Cancelled => 7,
            Self::Escalated => 8,
        }
    }

    /// Converts a stable ordinal into a status.
    ///
    /// Unknown values are rejected rather than silently mapped.
    #[must_use]
    pub const fn from_ordinal(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Pending),
            1 => Some(Self::Running),
            2 => Some(Self::Suspended),
            3 => Some(Self::Recovering),
            4 => Some(Self::Verifying),
            5 => Some(Self::Completed),
            6 => Some(Self::Failed),
            7 => Some(Self::Cancelled),
            8 => Some(Self::Escalated),
            _ => None,
        }
    }

    /// Returns whether the execution has reached a terminal status.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Failed
                | Self::Cancelled
                | Self::Escalated
        )
    }

    /// Returns whether execution may still progress.
    #[must_use]
    pub const fn is_active(self) -> bool {
        !self.is_terminal()
    }

    /// Returns whether the execution can legitimately be resumed.
    #[must_use]
    pub const fn is_resumable(self) -> bool {
        matches!(
            self,
            Self::Suspended | Self::Recovering
        )
    }
}

impl fmt::Display for ExecutionStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// EXECUTION ERROR
// ============================================================================

/// Error produced by execution-state manipulation.
///
/// This local error is intentionally small and structural.
///
/// Higher layers can map it into the repository-wide `ResilienceError`
/// contract without coupling this foundational state object to the concrete
/// error implementation.
///
/// This follows the repository's architecture in which foundational modules
/// may expose local validation errors while the common resilience error layer
/// provides the system-wide taxonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionStateError {
    /// Execution identity is empty.
    EmptyExecutionId,

    /// A semantic hash or identifier is empty where a value was required.
    EmptyIdentifier {
        field: &'static str,
    },

    /// A status transition is not legal.
    InvalidStatusTransition {
        from: ExecutionStatus,
        to: ExecutionStatus,
    },

    /// A monotonic counter cannot advance without wrapping.
    CounterOverflow {
        field: &'static str,
    },

    /// A revision cannot advance without wrapping.
    RevisionOverflow,

    /// An execution generation cannot advance without wrapping.
    GenerationOverflow,

    /// An attempt number cannot advance without wrapping.
    AttemptOverflow,

    /// A resource epoch cannot advance without wrapping.
    ResourceEpochOverflow,

    /// A checkpoint epoch cannot advance without wrapping.
    CheckpointEpochOverflow,
}

impl fmt::Display for ExecutionStateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyExecutionId => {
                formatter.write_str("execution identifier must not be empty")
            }

            Self::EmptyIdentifier { field } => {
                write!(formatter, "{field} must not be empty")
            }

            Self::InvalidStatusTransition { from, to } => {
                write!(
                    formatter,
                    "invalid execution status transition: {from} -> {to}"
                )
            }

            Self::CounterOverflow { field } => {
                write!(formatter, "execution counter overflow: {field}")
            }

            Self::RevisionOverflow => {
                formatter.write_str("execution state revision overflow")
            }

            Self::GenerationOverflow => {
                formatter.write_str("execution generation overflow")
            }

            Self::AttemptOverflow => {
                formatter.write_str("execution attempt overflow")
            }

            Self::ResourceEpochOverflow => {
                formatter.write_str("execution resource epoch overflow")
            }

            Self::CheckpointEpochOverflow => {
                formatter.write_str("execution checkpoint epoch overflow")
            }
        }
    }
}

impl std::error::Error for ExecutionStateError {}

// ============================================================================
// EXECUTION SNAPSHOT
// ============================================================================

/// Immutable owned snapshot of execution state.
///
/// The snapshot intentionally contains no implementation-specific handles,
/// locks, pointers, clocks, credentials or backend objects.
///
/// It is suitable for:
///
/// - persistence adapters;
/// - serialization;
/// - deterministic testing;
/// - checkpoint metadata;
/// - telemetry projection;
/// - state comparison.
///
/// The logical and physical qubit collections are copied because a snapshot
/// must remain independent from the live mutable execution state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionStateSnapshot {
    execution_id: String,
    status: ExecutionStatus,
    attempt: u64,
    generation: u64,
    revision: u64,

    submitted_operations: u64,
    completed_operations: u64,

    requested_shots: Option<u64>,
    completed_shots: u64,

    logical_qubits: BTreeSet<QubitId>,
    physical_qubits: BTreeSet<PhysicalQubitId>,

    target_epoch: u64,
    checkpoint_epoch: u64,

    semantic_hash: Option<String>,
    implementation_hash: Option<String>,
    result_hash: Option<String>,
}

impl ExecutionStateSnapshot {
    /// Returns the immutable execution identifier.
    #[must_use]
    pub fn execution_id(&self) -> &str {
        &self.execution_id
    }

    /// Returns the execution status.
    #[must_use]
    pub const fn status(&self) -> ExecutionStatus {
        self.status
    }

    /// Returns the current attempt number.
    ///
    /// The initial attempt is `0`.
    #[must_use]
    pub const fn attempt(&self) -> u64 {
        self.attempt
    }

    /// Returns the current execution generation.
    ///
    /// A generation changes when the execution representation is replaced or
    /// reconstructed by a higher-level recovery mechanism.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the state revision.
    ///
    /// The revision changes after every successful mutation of the state.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns the number of submitted operations represented by this state.
    #[must_use]
    pub const fn submitted_operations(&self) -> u64 {
        self.submitted_operations
    }

    /// Returns the number of completed operations represented by this state.
    #[must_use]
    pub const fn completed_operations(&self) -> u64 {
        self.completed_operations
    }

    /// Returns the requested shot count when known.
    #[must_use]
    pub const fn requested_shots(&self) -> Option<u64> {
        self.requested_shots
    }

    /// Returns the number of completed shots.
    #[must_use]
    pub const fn completed_shots(&self) -> u64 {
        self.completed_shots
    }

    /// Returns logical qubits associated with this execution.
    ///
    /// Ordering is canonical because the underlying collection is a
    /// `BTreeSet`.
    #[must_use]
    pub fn logical_qubits(&self) -> impl Iterator<Item = &QubitId> {
        self.logical_qubits.iter()
    }

    /// Returns the number of logical qubits associated with this execution.
    #[must_use]
    pub fn logical_qubit_count(&self) -> usize {
        self.logical_qubits.len()
    }

    /// Returns physical qubits associated with this execution.
    ///
    /// Physical identity remains distinct from logical identity.
    #[must_use]
    pub fn physical_qubits(&self) -> impl Iterator<Item = &PhysicalQubitId> {
        self.physical_qubits.iter()
    }

    /// Returns the number of physical qubits associated with this execution.
    #[must_use]
    pub fn physical_qubit_count(&self) -> usize {
        self.physical_qubits.len()
    }

    /// Returns the target/resource epoch.
    ///
    /// A higher-level hardware/resource integration layer should advance this
    /// when the target capability snapshot changes.
    #[must_use]
    pub const fn target_epoch(&self) -> u64 {
        self.target_epoch
    }

    /// Returns the latest checkpoint epoch known to the execution state.
    #[must_use]
    pub const fn checkpoint_epoch(&self) -> u64 {
        self.checkpoint_epoch
    }

    /// Returns the semantic program identity hash when available.
    #[must_use]
    pub fn semantic_hash(&self) -> Option<&str> {
        self.semantic_hash.as_deref()
    }

    /// Returns the implementation/IR identity hash when available.
    #[must_use]
    pub fn implementation_hash(&self) -> Option<&str> {
        self.implementation_hash.as_deref()
    }

    /// Returns the result identity hash when available.
    #[must_use]
    pub fn result_hash(&self) -> Option<&str> {
        self.result_hash.as_deref()
    }
}

// ============================================================================
// EXECUTION STATE
// ============================================================================

/// Mutable provider-neutral state for one quantum execution.
///
/// This is an execution facts container, not an execution engine.
///
/// It is intentionally independent of:
///
/// - backend SDKs;
/// - QPU objects;
/// - network clients;
/// - compiler objects;
/// - scheduler objects;
/// - router objects;
/// - QEC decoders;
/// - checkpoint stores.
///
/// This makes it safe to use as the common state object across hardware,
/// simulator and distributed execution environments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionState {
    execution_id: String,
    status: ExecutionStatus,

    /// Current retry/recovery attempt.
    attempt: u64,

    /// Current execution representation generation.
    generation: u64,

    /// Monotonic state revision.
    revision: u64,

    /// Number of operations submitted to the execution representation.
    submitted_operations: u64,

    /// Number of operations known to have completed.
    completed_operations: u64,

    /// Requested number of shots, if the execution model specifies shots.
    ///
    `None` deliberately means "not applicable/unknown", rather than zero.
    requested_shots: Option<u64>,

    /// Number of shots known to have completed.
    completed_shots: u64,

    /// Logical qubits participating in this execution.
    logical_qubits: BTreeSet<QubitId>,

    /// Physical resources currently associated with this execution.
    ///
    /// The logical-to-physical mapping itself belongs to routing/placement.
    /// This collection only records the physical identities known to be
    /// associated with the execution.
    physical_qubits: BTreeSet<PhysicalQubitId>,

    /// Resource/capability snapshot generation.
    target_epoch: u64,

    /// Checkpoint generation known to the execution.
    checkpoint_epoch: u64,

    /// Hash identifying the semantic program/input.
    semantic_hash: Option<String>,

    /// Hash identifying the concrete implementation representation.
    ///
    /// This can be an IR/compiled-representation hash supplied by the compiler
    /// or execution layer. This module does not calculate it.
    implementation_hash: Option<String>,

    /// Hash identifying the accepted/observed result representation.
    ///
    /// This module does not calculate or verify the hash.
    result_hash: Option<String>,
}

impl ExecutionState {
    /// Creates a new execution state.
    ///
    /// The initial status is [`ExecutionStatus::Pending`].
    ///
    /// No machine-size allocation occurs here. Qubit collections remain empty
    /// until the execution integration layer records actual resources.
    pub fn new(execution_id: impl Into<String>) -> Result<Self, ExecutionStateError> {
        let execution_id = execution_id.into();

        if execution_id.is_empty() {
            return Err(ExecutionStateError::EmptyExecutionId);
        }

        Ok(Self {
            execution_id,
            status: ExecutionStatus::Pending,
            attempt: 0,
            generation: 0,
            revision: 0,
            submitted_operations: 0,
            completed_operations: 0,
            requested_shots: None,
            completed_shots: 0,
            logical_qubits: BTreeSet::new(),
            physical_qubits: BTreeSet::new(),
            target_epoch: 0,
            checkpoint_epoch: 0,
            semantic_hash: None,
            implementation_hash: None,
            result_hash: None,
        })
    }

    /// Restores execution state from an owned snapshot.
    ///
    /// This does not perform checkpoint compatibility validation.
    /// That belongs to `checkpoint::compatibility`.
    pub fn from_snapshot(snapshot: ExecutionStateSnapshot) -> Self {
        Self {
            execution_id: snapshot.execution_id,
            status: snapshot.status,
            attempt: snapshot.attempt,
            generation: snapshot.generation,
            revision: snapshot.revision,
            submitted_operations: snapshot.submitted_operations,
            completed_operations: snapshot.completed_operations,
            requested_shots: snapshot.requested_shots,
            completed_shots: snapshot.completed_shots,
            logical_qubits: snapshot.logical_qubits,
            physical_qubits: snapshot.physical_qubits,
            target_epoch: snapshot.target_epoch,
            checkpoint_epoch: snapshot.checkpoint_epoch,
            semantic_hash: snapshot.semantic_hash,
            implementation_hash: snapshot.implementation_hash,
            result_hash: snapshot.result_hash,
        }
    }

    /// Creates an owned immutable snapshot.
    #[must_use]
    pub fn snapshot(&self) -> ExecutionStateSnapshot {
        ExecutionStateSnapshot {
            execution_id: self.execution_id.clone(),
            status: self.status,
            attempt: self.attempt,
            generation: self.generation,
            revision: self.revision,
            submitted_operations: self.submitted_operations,
            completed_operations: self.completed_operations,
            requested_shots: self.requested_shots,
            completed_shots: self.completed_shots,
            logical_qubits: self.logical_qubits.clone(),
            physical_qubits: self.physical_qubits.clone(),
            target_epoch: self.target_epoch,
            checkpoint_epoch: self.checkpoint_epoch,
            semantic_hash: self.semantic_hash.clone(),
            implementation_hash: self.implementation_hash.clone(),
            result_hash: self.result_hash.clone(),
        }
    }

    // ------------------------------------------------------------------------
    // Identity
    // ------------------------------------------------------------------------

    /// Returns the execution identifier.
    #[must_use]
    pub fn execution_id(&self) -> &str {
        &self.execution_id
    }

    /// Returns the current execution status.
    #[must_use]
    pub const fn status(&self) -> ExecutionStatus {
        self.status
    }

    /// Returns the current attempt.
    #[must_use]
    pub const fn attempt(&self) -> u64 {
        self.attempt
    }

    /// Returns the current execution generation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the current state revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    // ------------------------------------------------------------------------
    // Progress
    // ------------------------------------------------------------------------

    /// Returns the number of submitted operations.
    #[must_use]
    pub const fn submitted_operations(&self) -> u64 {
        self.submitted_operations
    }

    /// Returns the number of completed operations.
    #[must_use]
    pub const fn completed_operations(&self) -> u64 {
        self.completed_operations
    }

    /// Returns the requested shot count, when applicable.
    #[must_use]
    pub const fn requested_shots(&self) -> Option<u64> {
        self.requested_shots
    }

    /// Returns completed shots.
    #[must_use]
    pub const fn completed_shots(&self) -> u64 {
        self.completed_shots
    }

    /// Returns the target/resource epoch.
    #[must_use]
    pub const fn target_epoch(&self) -> u64 {
        self.target_epoch
    }

    /// Returns the checkpoint epoch.
    #[must_use]
    pub const fn checkpoint_epoch(&self) -> u64 {
        self.checkpoint_epoch
    }

    // ------------------------------------------------------------------------
    // Quantum resources
    // ------------------------------------------------------------------------

    /// Returns logical qubits in canonical deterministic order.
    #[must_use]
    pub fn logical_qubits(&self) -> impl Iterator<Item = &QubitId> {
        self.logical_qubits.iter()
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub fn logical_qubit_count(&self) -> usize {
        self.logical_qubits.len()
    }

    /// Returns whether a logical qubit is associated with this execution.
    #[must_use]
    pub fn contains_logical_qubit(&self, qubit: &QubitId) -> bool {
        self.logical_qubits.contains(qubit)
    }

    /// Returns physical qubits in canonical deterministic order.
    #[must_use]
    pub fn physical_qubits(&self) -> impl Iterator<Item = &PhysicalQubitId> {
        self.physical_qubits.iter()
    }

    /// Returns the number of physical qubits.
    #[must_use]
    pub fn physical_qubit_count(&self) -> usize {
        self.physical_qubits.len()
    }

    /// Returns whether a physical qubit is associated with this execution.
    #[must_use]
    pub fn contains_physical_qubit(&self, qubit: &PhysicalQubitId) -> bool {
        self.physical_qubits.contains(qubit)
    }

    /// Replaces the complete logical-qubit set.
    ///
    /// The caller is responsible for obtaining the set from canonical IR.
    pub fn replace_logical_qubits<I>(&mut self, qubits: I)
    where
        I: IntoIterator<Item = QubitId>,
    {
        self.logical_qubits.clear();
        self.logical_qubits.extend(qubits);
        self.touch_revision();
    }

    /// Replaces the complete physical-qubit set.
    ///
    /// The caller is responsible for obtaining the physical resources from
    /// routing/hardware integration.
    pub fn replace_physical_qubits<I>(&mut self, qubits: I)
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        self.physical_qubits.clear();
        self.physical_qubits.extend(qubits);
        self.touch_revision();
    }

    /// Adds one logical qubit to the execution.
    ///
    /// Returns whether the set changed.
    pub fn add_logical_qubit(&mut self, qubit: QubitId) -> bool {
        let changed = self.logical_qubits.insert(qubit);

        if changed {
            self.touch_revision();
        }

        changed
    }

    /// Adds one physical qubit to the execution.
    ///
    /// Returns whether the set changed.
    pub fn add_physical_qubit(&mut self, qubit: PhysicalQubitId) -> bool {
        let changed = self.physical_qubits.insert(qubit);

        if changed {
            self.touch_revision();
        }

        changed
    }

    /// Removes one logical qubit from the execution.
    ///
    /// Returns whether the set changed.
    pub fn remove_logical_qubit(&mut self, qubit: &QubitId) -> bool {
        let changed = self.logical_qubits.remove(qubit);

        if changed {
            self.touch_revision();
        }

        changed
    }

    /// Removes one physical qubit from the execution.
    ///
    /// Returns whether the set changed.
    pub fn remove_physical_qubit(&mut self, qubit: &PhysicalQubitId) -> bool {
        let changed = self.physical_qubits.remove(qubit);

        if changed {
            self.touch_revision();
        }

        changed
    }

    // ------------------------------------------------------------------------
    // Status
    // ------------------------------------------------------------------------

    /// Returns whether a status transition is legal.
    ///
    /// This method is pure and does not mutate state.
    #[must_use]
    pub const fn can_transition_to(
        from: ExecutionStatus,
        to: ExecutionStatus,
    ) -> bool {
        match from {
            ExecutionStatus::Pending => matches!(
                to,
                ExecutionStatus::Running
                    | ExecutionStatus::Cancelled
                    | ExecutionStatus::Failed
                    | ExecutionStatus::Escalated
            ),

            ExecutionStatus::Running => matches!(
                to,
                ExecutionStatus::Suspended
                    | ExecutionStatus::Recovering
                    | ExecutionStatus::Verifying
                    | ExecutionStatus::Completed
                    | ExecutionStatus::Failed
                    | ExecutionStatus::Cancelled
                    | ExecutionStatus::Escalated
            ),

            ExecutionStatus::Suspended => matches!(
                to,
                ExecutionStatus::Running
                    | ExecutionStatus::Recovering
                    | ExecutionStatus::Cancelled
                    | ExecutionStatus::Failed
                    | ExecutionStatus::Escalated
            ),

            ExecutionStatus::Recovering => matches!(
                to,
                ExecutionStatus::Running
                    | ExecutionStatus::Verifying
                    | ExecutionStatus::Failed
                    | ExecutionStatus::Cancelled
                    | ExecutionStatus::Escalated
            ),

            ExecutionStatus::Verifying => matches!(
                to,
                ExecutionStatus::Completed
                    | ExecutionStatus::Recovering
                    | ExecutionStatus::Failed
                    | ExecutionStatus::Escalated
            ),

            ExecutionStatus::Completed
            | ExecutionStatus::Failed
            | ExecutionStatus::Cancelled
            | ExecutionStatus::Escalated => false,
        }
    }

    /// Attempts a legal status transition.
    pub fn transition_to(
        &mut self,
        next: ExecutionStatus,
    ) -> Result<(), ExecutionStateError> {
        if !Self::can_transition_to(self.status, next) {
            return Err(ExecutionStateError::InvalidStatusTransition {
                from: self.status,
                to: next,
            });
        }

        self.status = next;
        self.bump_revision()?;

        Ok(())
    }

    /// Starts execution.
    pub fn start(&mut self) -> Result<(), ExecutionStateError> {
        self.transition_to(ExecutionStatus::Running)
    }

    /// Suspends execution at a higher-level validated execution boundary.
    pub fn suspend(&mut self) -> Result<(), ExecutionStateError> {
        self.transition_to(ExecutionStatus::Suspended)
    }

    /// Marks execution as recovering.
    pub fn begin_recovery(&mut self) -> Result<(), ExecutionStateError> {
        self.transition_to(ExecutionStatus::Recovering)
    }

    /// Marks execution as being verified.
    pub fn begin_verification(&mut self) -> Result<(), ExecutionStateError> {
        self.transition_to(ExecutionStatus::Verifying)
    }

    /// Marks execution complete.
    ///
    /// This does NOT mean the resilience verification layer has accepted the
    /// quantum result. The caller must ensure the transition is semantically
    /// justified.
    pub fn complete(&mut self) -> Result<(), ExecutionStateError> {
        self.transition_to(ExecutionStatus::Completed)
    }

    /// Marks execution failed.
    pub fn fail(&mut self) -> Result<(), ExecutionStateError> {
        self.transition_to(ExecutionStatus::Failed)
    }

    /// Marks execution cancelled.
    pub fn cancel(&mut self) -> Result<(), ExecutionStateError> {
        self.transition_to(ExecutionStatus::Cancelled)
    }

    /// Marks execution escalated.
    pub fn escalate(&mut self) -> Result<(), ExecutionStateError> {
        self.transition_to(ExecutionStatus::Escalated)
    }

    /// Resumes execution after a valid suspension/recovery boundary.
    pub fn resume(&mut self) -> Result<(), ExecutionStateError> {
        self.transition_to(ExecutionStatus::Running)
    }

    // ------------------------------------------------------------------------
    // Attempts / generations
    // ------------------------------------------------------------------------

    /// Advances the recovery/execution attempt number.
    ///
    /// This does not itself authorize a retry. Retry policy and recovery
    /// planning remain higher-level responsibilities.
    pub fn advance_attempt(&mut self) -> Result<u64, ExecutionStateError> {
        let next = self
            .attempt
            .checked_add(1)
            .ok_or(ExecutionStateError::AttemptOverflow)?;

        self.attempt = next;
        self.bump_revision()?;

        Ok(next)
    }

    /// Advances the execution representation generation.
    ///
    /// A generation should be advanced when a higher-level recovery/adaptation
    /// layer replaces the concrete execution representation.
    pub fn advance_generation(&mut self) -> Result<u64, ExecutionStateError> {
        let next = self
            .generation
            .checked_add(1)
            .ok_or(ExecutionStateError::GenerationOverflow)?;

        self.generation = next;
        self.bump_revision()?;

        Ok(next)
    }

    // ------------------------------------------------------------------------
    // Operation progress
    // ------------------------------------------------------------------------

    /// Adds submitted operations.
    ///
    /// The value represents a count, not a particular hardware operation ID.
    pub fn record_submitted_operations(
        &mut self,
        count: u64,
    ) -> Result<(), ExecutionStateError> {
        if count == 0 {
            return Ok(());
        }

        let next = self
            .submitted_operations
            .checked_add(count)
            .ok_or(ExecutionStateError::CounterOverflow {
                field: "submitted_operations",
            })?;

        self.submitted_operations = next;
        self.bump_revision()?;

        Ok(())
    }

    /// Adds completed operations.
    ///
    /// This method intentionally does not require the number of completed
    /// operations to equal the number submitted in this object because
    /// distributed or streamed execution can report partial progress.
    ///
    /// Higher-level execution verification may impose stronger invariants.
    pub fn record_completed_operations(
        &mut self,
        count: u64,
    ) -> Result<(), ExecutionStateError> {
        if count == 0 {
            return Ok(());
        }

        let next = self
            .completed_operations
            .checked_add(count)
            .ok_or(ExecutionStateError::CounterOverflow {
                field: "completed_operations",
            })?;

        self.completed_operations = next;
        self.bump_revision()?;

        Ok(())
    }

    /// Sets the requested shot count.
    ///
    /// `None` means that shots are not applicable or not specified.
    pub fn set_requested_shots(
        &mut self,
        shots: Option<u64>,
    ) -> Result<(), ExecutionStateError> {
        self.requested_shots = shots;
        self.bump_revision()
    }

    /// Adds completed shots.
    pub fn record_completed_shots(
        &mut self,
        count: u64,
    ) -> Result<(), ExecutionStateError> {
        if count == 0 {
            return Ok(());
        }

        let next = self
            .completed_shots
            .checked_add(count)
            .ok_or(ExecutionStateError::CounterOverflow {
                field: "completed_shots",
            })?;

        self.completed_shots = next;
        self.bump_revision()?;

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Resource / checkpoint epochs
    // ------------------------------------------------------------------------

    /// Advances the target/resource epoch.
    ///
    /// The caller should use this when a new trusted target capability
    /// snapshot becomes authoritative.
    pub fn advance_target_epoch(&mut self) -> Result<u64, ExecutionStateError> {
        let next = self
            .target_epoch
            .checked_add(1)
            .ok_or(ExecutionStateError::ResourceEpochOverflow)?;

        self.target_epoch = next;
        self.bump_revision()?;

        Ok(next)
    }

    /// Records a checkpoint epoch.
    ///
    /// This does not create a checkpoint. It only records the externally
    /// established checkpoint generation.
    pub fn advance_checkpoint_epoch(
        &mut self,
    ) -> Result<u64, ExecutionStateError> {
        let next = self
            .checkpoint_epoch
            .checked_add(1)
            .ok_or(ExecutionStateError::CheckpointEpochOverflow)?;

        self.checkpoint_epoch = next;
        self.bump_revision()?;

        Ok(next)
    }

    // ------------------------------------------------------------------------
    // Identity hashes
    // ------------------------------------------------------------------------

    /// Returns the semantic program identity hash.
    #[must_use]
    pub fn semantic_hash(&self) -> Option<&str> {
        self.semantic_hash.as_deref()
    }

    /// Sets the semantic program identity hash.
    ///
    /// The hash is treated as opaque data. Hash calculation and cryptographic
    /// verification belong elsewhere.
    pub fn set_semantic_hash(
        &mut self,
        value: Option<String>,
    ) -> Result<(), ExecutionStateError> {
        if value.as_deref() == Some("") {
            return Err(ExecutionStateError::EmptyIdentifier {
                field: "semantic_hash",
            });
        }

        self.semantic_hash = value;
        self.bump_revision()
    }

    /// Returns the concrete implementation/IR identity hash.
    #[must_use]
    pub fn implementation_hash(&self) -> Option<&str> {
        self.implementation_hash.as_deref()
    }

    /// Sets the concrete implementation/IR identity hash.
    pub fn set_implementation_hash(
        &mut self,
        value: Option<String>,
    ) -> Result<(), ExecutionStateError> {
        if value.as_deref() == Some("") {
            return Err(ExecutionStateError::EmptyIdentifier {
                field: "implementation_hash",
            });
        }

        self.implementation_hash = value;
        self.bump_revision()
    }

    /// Returns the result identity hash.
    #[must_use]
    pub fn result_hash(&self) -> Option<&str> {
        self.result_hash.as_deref()
    }

    /// Sets the result identity hash.
    ///
    /// The hash is an opaque result identifier. Verification belongs to the
    /// verification subsystem.
    pub fn set_result_hash(
        &mut self,
        value: Option<String>,
    ) -> Result<(), ExecutionStateError> {
        if value.as_deref() == Some("") {
            return Err(ExecutionStateError::EmptyIdentifier {
                field: "result_hash",
            });
        }

        self.result_hash = value;
        self.bump_revision()
    }

    // ------------------------------------------------------------------------
    // Internal revision management
    // ------------------------------------------------------------------------

    /// Advances the state revision.
    fn bump_revision(&mut self) -> Result<(), ExecutionStateError> {
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or(ExecutionStateError::RevisionOverflow)?;

        Ok(())
    }

    /// Advances the state revision without returning a result.
    ///
    /// Used only by mutation paths where overflow cannot be meaningfully
    /// propagated after the collection itself has already changed.
    ///
    /// Therefore all public mutation paths that can fail due to revision
    /// overflow use `bump_revision` directly. Collection replacement methods
    /// intentionally use this method only after a logically successful
    /// replacement.
    fn touch_revision(&mut self) {
        if let Some(next) = self.revision.checked_add(1) {
            self.revision = next;
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> ExecutionState {
        ExecutionState::new("execution-1")
            .expect("test execution identifier must be valid")
    }

    #[test]
    fn new_state_is_pending() {
        let execution = state();

        assert_eq!(execution.status(), ExecutionStatus::Pending);
        assert_eq!(execution.attempt(), 0);
        assert_eq!(execution.generation(), 0);
        assert_eq!(execution.revision(), 0);
        assert_eq!(execution.logical_qubit_count(), 0);
        assert_eq!(execution.physical_qubit_count(), 0);
    }

    #[test]
    fn empty_execution_identifier_is_rejected() {
        assert_eq!(
            ExecutionState::new(""),
            Err(ExecutionStateError::EmptyExecutionId)
        );
    }

    #[test]
    fn normal_execution_lifecycle_is_supported() {
        let mut execution = state();

        execution.start().expect("pending -> running");
        execution
            .begin_verification()
            .expect("running -> verifying");
        execution.complete().expect("verifying -> completed");

        assert_eq!(execution.status(), ExecutionStatus::Completed);
        assert!(execution.status().is_terminal());
    }

    #[test]
    fn recovery_lifecycle_is_supported() {
        let mut execution = state();

        execution.start().expect("pending -> running");
        execution
            .begin_recovery()
            .expect("running -> recovering");
        execution
            .resume()
            .expect("recovering -> running");
        execution
            .begin_verification()
            .expect("running -> verifying");
        execution.complete().expect("verification -> completed");

        assert_eq!(execution.status(), ExecutionStatus::Completed);
    }

    #[test]
    fn terminal_states_are_absorbing() {
        let terminal_states = [
            ExecutionStatus::Completed,
            ExecutionStatus::Failed,
            ExecutionStatus::Cancelled,
            ExecutionStatus::Escalated,
        ];

        for status in terminal_states {
            for target in [
                ExecutionStatus::Pending,
                ExecutionStatus::Running,
                ExecutionStatus::Suspended,
                ExecutionStatus::Recovering,
                ExecutionStatus::Verifying,
                ExecutionStatus::Completed,
                ExecutionStatus::Failed,
                ExecutionStatus::Cancelled,
                ExecutionStatus::Escalated,
            ] {
                assert!(
                    !ExecutionState::can_transition_to(status, target),
                    "terminal state {status} must not transition to {target}"
                );
            }
        }
    }

    #[test]
    fn invalid_transition_does_not_mutate_state() {
        let mut execution = state();

        let before = execution.snapshot();

        let result = execution.complete();

        assert!(result.is_err());
        assert_eq!(execution.snapshot(), before);
    }

    #[test]
    fn attempts_are_monotonic() {
        let mut execution = state();

        assert_eq!(
            execution
                .advance_attempt()
                .expect("first attempt"),
            1
        );

        assert_eq!(
            execution
                .advance_attempt()
                .expect("second attempt"),
            2
        );

        assert_eq!(execution.attempt(), 2);
    }

    #[test]
    fn generations_are_monotonic() {
        let mut execution = state();

        assert_eq!(
            execution
                .advance_generation()
                .expect("first generation"),
            1
        );

        assert_eq!(
            execution
                .advance_generation()
                .expect("second generation"),
            2
        );

        assert_eq!(execution.generation(), 2);
    }

    #[test]
    fn operation_progress_is_additive() {
        let mut execution = state();

        execution
            .record_submitted_operations(10)
            .expect("submit operations");

        execution
            .record_completed_operations(4)
            .expect("complete operations");

        assert_eq!(execution.submitted_operations(), 10);
        assert_eq!(execution.completed_operations(), 4);
    }

    #[test]
    fn zero_progress_does_not_change_revision() {
        let mut execution = state();

        execution
            .record_submitted_operations(0)
            .expect("zero submission");

        execution
            .record_completed_operations(0)
            .expect("zero completion");

        execution
            .record_completed_shots(0)
            .expect("zero shots");

        assert_eq!(execution.revision(), 0);
    }

    #[test]
    fn logical_and_physical_identity_are_distinct_collections() {
        let mut execution = state();

        // The values are supplied by canonical IR identity types. No local
        // resilience-specific qubit identifiers exist here.
        let logical = QubitId::new(0);
        let physical = PhysicalQubitId::new(7);

        assert!(execution.add_logical_qubit(logical));
        assert!(execution.add_physical_qubit(physical));

        assert_eq!(execution.logical_qubit_count(), 1);
        assert_eq!(execution.physical_qubit_count(), 1);

        assert!(execution.contains_logical_qubit(&logical));
        assert!(execution.contains_physical_qubit(&physical));
    }

    #[test]
    fn duplicate_resource_insertion_is_idempotent() {
        let mut execution = state();

        let logical = QubitId::new(3);
        let physical = PhysicalQubitId::new(11);

        assert!(execution.add_logical_qubit(logical));
        assert!(!execution.add_logical_qubit(logical));

        assert!(execution.add_physical_qubit(physical));
        assert!(!execution.add_physical_qubit(physical));

        assert_eq!(execution.logical_qubit_count(), 1);
        assert_eq!(execution.physical_qubit_count(), 1);
    }

    #[test]
    fn resource_order_is_deterministic() {
        let mut execution = state();

        execution.add_logical_qubit(QubitId::new(9));
        execution.add_logical_qubit(QubitId::new(1));
        execution.add_logical_qubit(QubitId::new(5));

        let values: Vec<_> = execution
            .logical_qubits()
            .copied()
            .collect();

        assert_eq!(
            values,
            vec![
                QubitId::new(1),
                QubitId::new(5),
                QubitId::new(9),
            ]
        );
    }

    #[test]
    fn snapshot_round_trip_preserves_state() {
        let mut execution = state();

        execution.start().expect("start");
        execution.advance_attempt().expect("attempt");
        execution.advance_generation().expect("generation");
        execution
            .record_submitted_operations(100)
            .expect("submitted");
        execution
            .record_completed_operations(40)
            .expect("completed");
        execution
            .set_requested_shots(Some(1000))
            .expect("shots");
        execution
            .record_completed_shots(400)
            .expect("completed shots");

        execution.add_logical_qubit(QubitId::new(0));
        execution.add_logical_qubit(QubitId::new(2));
        execution.add_physical_qubit(PhysicalQubitId::new(4));
        execution.add_physical_qubit(PhysicalQubitId::new(8));

        execution
            .set_semantic_hash(Some("semantic".to_owned()))
            .expect("semantic hash");

        execution
            .set_implementation_hash(Some("implementation".to_owned()))
            .expect("implementation hash");

        execution
            .set_result_hash(Some("result".to_owned()))
            .expect("result hash");

        let snapshot = execution.snapshot();
        let restored = ExecutionState::from_snapshot(snapshot);

        assert_eq!(execution, restored);
    }

    #[test]
    fn hashes_are_optional_but_not_empty() {
        let mut execution = state();

        assert_eq!(execution.semantic_hash(), None);

        assert_eq!(
            execution.set_semantic_hash(Some(String::new())),
            Err(ExecutionStateError::EmptyIdentifier {
                field: "semantic_hash"
            })
        );

        assert_eq!(
            execution.set_implementation_hash(Some(String::new())),
            Err(ExecutionStateError::EmptyIdentifier {
                field: "implementation_hash"
            })
        );

        assert_eq!(
            execution.set_result_hash(Some(String::new())),
            Err(ExecutionStateError::EmptyIdentifier {
                field: "result_hash"
            })
        );
    }

    #[test]
    fn status_ordinals_are_stable() {
        assert_eq!(ExecutionStatus::Pending.ordinal(), 0);
        assert_eq!(ExecutionStatus::Running.ordinal(), 1);
        assert_eq!(ExecutionStatus::Suspended.ordinal(), 2);
        assert_eq!(ExecutionStatus::Recovering.ordinal(), 3);
        assert_eq!(ExecutionStatus::Verifying.ordinal(), 4);
        assert_eq!(ExecutionStatus::Completed.ordinal(), 5);
        assert_eq!(ExecutionStatus::Failed.ordinal(), 6);
        assert_eq!(ExecutionStatus::Cancelled.ordinal(), 7);
        assert_eq!(ExecutionStatus::Escalated.ordinal(), 8);

        for value in 0..=8 {
            let status =
                ExecutionStatus::from_ordinal(value)
                    .expect("stable ordinal must decode");

            assert_eq!(status.ordinal(), value);
        }

        assert_eq!(ExecutionStatus::from_ordinal(255), None);
    }

    #[test]
    fn status_transition_function_is_pure() {
        assert!(ExecutionState::can_transition_to(
            ExecutionStatus::Pending,
            ExecutionStatus::Running
        ));

        assert!(!ExecutionState::can_transition_to(
            ExecutionStatus::Pending,
            ExecutionStatus::Completed
        ));

        assert!(ExecutionState::can_transition_to(
            ExecutionStatus::Running,
            ExecutionStatus::Recovering
        ));

        assert!(ExecutionState::can_transition_to(
            ExecutionStatus::Verifying,
            ExecutionStatus::Recovering
        ));
    }

    #[test]
    fn target_and_checkpoint_epochs_are_independent() {
        let mut execution = state();

        assert_eq!(execution.target_epoch(), 0);
        assert_eq!(execution.checkpoint_epoch(), 0);

        execution
            .advance_target_epoch()
            .expect("target epoch");

        execution
            .advance_checkpoint_epoch()
            .expect("checkpoint epoch");

        assert_eq!(execution.target_epoch(), 1);
        assert_eq!(execution.checkpoint_epoch(), 1);
    }

    #[test]
    fn status_transition_increments_revision() {
        let mut execution = state();

        assert_eq!(execution.revision(), 0);

        execution.start().expect("start");

        assert_eq!(execution.revision(), 1);
    }

    #[test]
    fn snapshot_is_independent_of_live_state() {
        let mut execution = state();

        execution.add_logical_qubit(QubitId::new(1));

        let snapshot = execution.snapshot();

        execution.add_logical_qubit(QubitId::new(2));

        assert_eq!(snapshot.logical_qubit_count(), 1);
        assert_eq!(execution.logical_qubit_count(), 2);
    }

    #[test]
    fn completed_execution_is_not_resumable() {
        assert!(!ExecutionStatus::Completed.is_resumable());
        assert!(!ExecutionStatus::Failed.is_resumable());
        assert!(!ExecutionStatus::Cancelled.is_resumable());
        assert!(!ExecutionStatus::Escalated.is_resumable());
    }

    #[test]
    fn suspended_and_recovering_execution_are_resumable() {
        assert!(ExecutionStatus::Suspended.is_resumable());
        assert!(ExecutionStatus::Recovering.is_resumable());
    }
}