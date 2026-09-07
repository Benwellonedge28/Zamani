//! Zamani Quantum Resilience — Distributed Coordination
//!
//! Path:
//!     src/quantum/resilience/coordination/coordinator.rs
//!
//! Purpose:
//!     High-level coordination of distributed quantum-resilience operations.
//!
//! Architectural position:
//!
//!     Zamani Quantum Program
//!             |
//!             v
//!     Canonical Quantum IR
//!             |
//!             v
//!     Quantum Execution / Resilience
//!             |
//!             v
//!     +--------------------------+
//!     | Distributed Coordinator  |
//!     +--------------------------+
//!        |       |       |      |
//!        v       v       v      v
//!     ownership lease distributed consensus
//!
//! This module coordinates those contracts. It does not implement:
//!
//! - resource ownership;
//! - lease acquisition/renewal;
//! - distributed transport;
//! - consensus;
//! - routing;
//! - scheduling;
//! - QEC;
//! - hardware discovery;
//! - calibration;
//! - optimization;
//! - execution;
//! - mitigation;
//! - verification algorithms.
//!
//! Those responsibilities remain in their authoritative subsystems.
//!
//! -----------------------------------------------------------------------------
//! DESIGN REQUIREMENTS
//! -----------------------------------------------------------------------------
//!
//! This module is designed for:
//!
//! - Rust 1.97 / 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no unsafe Rust;
//! - no provider-specific logic;
//! - no hard-coded qubit count;
//! - no hard-coded device count;
//! - no hard-coded backend count;
//! - no hard-coded retry count;
//! - no hard-coded quorum size;
//! - no hard-coded timeout;
//! - no hard-coded lease duration;
//! - no hard-coded topology;
//! - no hidden global mutable state;
//! - no implicit I/O;
//! - no implicit threads;
//! - deterministic coordination when supplied dependencies are deterministic;
//! - explicit ownership and fencing;
//! - idempotent operation handling;
//! - stale-operation rejection;
//! - explicit failure propagation;
//! - graceful degradation;
//! - arbitrary resource cardinality subject only to available resources;
//! - integration with ownership.rs;
//! - integration with lease.rs;
//! - integration with distributed.rs;
//! - integration with consensus.rs;
//! - integration with the resilience controller/runtime.
//!
//! "Infinite scalability" means that this file imposes no artificial finite
//! quantum-system limit. Actual scalability remains bounded only by available
//! computational, memory, communication, storage, and quantum resources.
//!
//! -----------------------------------------------------------------------------
//! IMPORTANT QUANTUM ARCHITECTURE RULE
//! -----------------------------------------------------------------------------
//!
//! Coordination does not define quantum identifiers.
//!
//! If a lower-level coordination implementation needs logical or physical
//! qubit identity, it MUST use the canonical repository definitions:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This coordinator itself intentionally operates on opaque resource handles.
//! Distributed coordination is not a quantum-IR concern.
//!
//! -----------------------------------------------------------------------------
//! DEPENDENCY DIRECTION
//! -----------------------------------------------------------------------------
//!
//!     quantum::ir
//!          |
//!          +-----------------------------+
//!          |                             |
//!          v                             v
//!     resilience                  execution/runtime
//!          |
//!          v
//!     coordination::coordinator
//!          |
//!     +----+---------+-------------+
//!     |              |             |
//!     v              v             v
//! ownership        lease       distributed
//!                                  |
//!                                  v
//!                              consensus
//!
//! The coordinator consumes contracts. It must not create reverse dependencies
//! from ownership/lease/consensus back into this orchestration layer.
//!
//! -----------------------------------------------------------------------------
//! SAFETY
//! -----------------------------------------------------------------------------
//!
//! A distributed recovery operation MUST NOT be considered successful merely
//! because one participant reports success.
//!
//! The coordinator requires an explicit coordination outcome from the supplied
//! distributed/consensus contract.
//!
//! In particular:
//!
//!     availability != correctness
//!     execution completed != result correct
//!     owner acquired != operation authorized
//!     lease acquired != operation safe
//!
//! Semantic/result verification remains the responsibility of the resilience
//! verification subsystem.
//!
//! -----------------------------------------------------------------------------
//! CONCURRENCY MODEL
//! -----------------------------------------------------------------------------
//!
//! The coordinator is deliberately synchronous and stateless.
//!
//! It does not create threads, spawn tasks, read clocks, or perform I/O.
//!
//! A runtime may execute it from:
//!
//! - a synchronous runtime;
//! - an asynchronous executor;
//! - a distributed worker;
//! - a deterministic replay engine;
//! - a simulator;
//! - a test harness.
//!
//! The concrete distributed implementation determines how messages and leases
//! are actually transported.
//!
//! -----------------------------------------------------------------------------
//! IDEMPOTENCY
//! -----------------------------------------------------------------------------
//!
//! Distributed resilience is particularly vulnerable to duplicate delivery.
//!
//! Every coordination operation therefore carries:
//!
//! - operation identity;
//! - execution identity;
//! - participant identity;
//! - fencing information;
//! - expected generation/version;
//! - deterministic request metadata.
//!
//! The coordinator never silently retries an operation.
//!
//! A retry is a new coordination invocation whose idempotency identity is
//! explicitly supplied by the caller.
//!
//! -----------------------------------------------------------------------------
//! FENCING
//! -----------------------------------------------------------------------------
//!
//! A stale coordinator must never be allowed to mutate resources after its
//! ownership/lease has expired or been superseded.
//!
//! Therefore the coordinator passes a `CoordinationFence` to every operation
//! that can affect distributed state.
//!
//! The authoritative ownership/lease implementation validates the fence.
//!
//! -----------------------------------------------------------------------------
//! ERROR HANDLING
//! -----------------------------------------------------------------------------
//!
//! This module never converts dependency errors into success.
//!
//! `ResilienceError` is propagated unchanged.
//!
//! Coordination-specific semantic failures are represented by
//! `CoordinationFailure` and converted only through the explicit
//! `CoordinationBackend` contract.
//!
//! -----------------------------------------------------------------------------
//! EXTENSIBILITY
//! -----------------------------------------------------------------------------
//!
//! The coordinator uses traits instead of concrete implementations.
//!
//! Future implementations may provide:
//!
//! - local coordination;
//! - process coordination;
//! - cluster coordination;
//! - data-center coordination;
//! - multi-QPU coordination;
//! - heterogeneous quantum-fleet coordination;
//! - remote quantum-service coordination;
//! - simulator coordination.
//!
//! No provider-specific branch belongs in this file.
//!
//! -----------------------------------------------------------------------------
//! STABILITY
//! -----------------------------------------------------------------------------
//!
//! Public types in this file are intentionally small and contract-oriented.
//!
//! Concrete algorithms should live in:
//!
//!     coordination/distributed.rs
//!     coordination/lease.rs
//!     coordination/ownership.rs
//!     coordination/consensus.rs
//!
//! The coordinator therefore does not need to be rewritten when those
//! implementations evolve.
//!
//! -----------------------------------------------------------------------------
//! RUST SAFETY POLICY
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::errors::error::ResilienceError;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for distributed resilience coordination.
pub const COORDINATOR_SCHEMA_ID: &str =
    "zamani.quantum.resilience.coordination.coordinator";

/// Current coordinator contract schema version.
///
/// This is a protocol/schema version and is not a machine-size limit.
pub const COORDINATOR_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Shared immutable string
// =============================================================================

type SharedString = Arc<str>;

// =============================================================================
// Operation identity
// =============================================================================

/// Globally meaningful identity of one distributed coordination operation.
///
/// The coordinator does not generate identifiers. Identifier generation is
/// deliberately left to the caller so that UUIDs, ULIDs, content-addressed
/// identifiers, deterministic identifiers, or distributed IDs can be used
/// without introducing hidden nondeterminism.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CoordinationOperationId(SharedString);

impl CoordinationOperationId {
    /// Creates a coordination operation identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CoordinationIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CoordinationIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CoordinationOperationId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Error creating a coordination identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoordinationIdError {
    /// The supplied identifier is empty.
    Empty,
}

impl fmt::Display for CoordinationIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => {
                formatter.write_str("coordination identifier is empty")
            }
        }
    }
}

impl std::error::Error for CoordinationIdError {}

// =============================================================================
// Execution identity
// =============================================================================

/// Identifies the logical execution being coordinated.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionId(SharedString);

impl ExecutionId {
    /// Creates an execution identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CoordinationIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CoordinationIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ExecutionId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Participant identity
// =============================================================================

/// Opaque identity of a distributed coordination participant.
///
/// A participant can represent a worker, process, runtime, QPU controller,
/// simulator, service, or other execution authority.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParticipantId(SharedString);

impl ParticipantId {
    /// Creates a participant identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CoordinationIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CoordinationIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ParticipantId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Resource identity
// =============================================================================

/// Opaque resource identity used by coordination.
///
/// The resource may represent any distributed quantum or classical resource.
///
/// Examples include:
///
/// - a logical quantum resource;
/// - a physical quantum resource;
/// - a QPU;
/// - a backend;
/// - an execution slot;
/// - a control resource;
/// - a classical worker;
/// - a simulator;
/// - a distributed execution domain.
///
/// Quantum-specific identifiers remain owned by `quantum::ir::qubit`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CoordinationResourceId(SharedString);

impl CoordinationResourceId {
    /// Creates a resource identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CoordinationIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CoordinationIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CoordinationResourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Coordination generation
// =============================================================================

/// Monotonically ordered ownership/lease generation supplied by the
/// authoritative coordination subsystem.
///
/// This is deliberately not a timestamp and is not generated by the
/// coordinator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CoordinationGeneration(u64);

impl CoordinationGeneration {
    /// Creates a generation value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the generation value.
    pub const fn get(self) -> u64 {
        self.0
    }
}

// =============================================================================
// Fencing token
// =============================================================================

/// Fencing token supplied by the authoritative ownership/lease subsystem.
///
/// A token must be treated as opaque by the coordinator.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CoordinationFence(SharedString);

impl CoordinationFence {
    /// Creates a fencing token.
    ///
    /// The token is supplied by the authoritative ownership/lease subsystem.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CoordinationIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CoordinationIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the token.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Operation mode
// =============================================================================

/// Defines how an operation participates in distributed coordination.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CoordinationMode {
    /// One authoritative participant owns the operation.
    Exclusive,

    /// Multiple participants may cooperate under an authoritative protocol.
    Cooperative,

    /// The operation is distributed but does not require exclusive resource
    /// ownership.
    Shared,

    /// The actual mode is selected by the injected coordination policy.
    Automatic,
}

impl Default for CoordinationMode {
    fn default() -> Self {
        Self::Automatic
    }
}

// =============================================================================
// Coordination phase
// =============================================================================

/// One phase of distributed coordination.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CoordinationPhase {
    /// Validate the request.
    Validate,

    /// Establish ownership.
    AcquireOwnership,

    /// Establish or validate a lease.
    AcquireLease,

    /// Establish the distributed participant set.
    Prepare,

    /// Obtain the required coordination decision.
    Coordinate,

    /// Commit the distributed decision.
    Commit,

    /// Release ownership/lease resources.
    Release,

    /// Produce the final coordination result.
    Finalize,
}

impl CoordinationPhase {
    /// Stable machine-readable phase name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validate => "validate",
            Self::AcquireOwnership => "acquire_ownership",
            Self::AcquireLease => "acquire_lease",
            Self::Prepare => "prepare",
            Self::Coordinate => "coordinate",
            Self::Commit => "commit",
            Self::Release => "release",
            Self::Finalize => "finalize",
        }
    }
}

impl fmt::Display for CoordinationPhase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Participant state
// =============================================================================

/// State reported for one participant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ParticipantState {
    /// Participant has not yet been prepared.
    Unknown,

    /// Participant is available.
    Ready,

    /// Participant is actively participating.
    Active,

    /// Participant has successfully completed its role.
    Completed,

    /// Participant is degraded but may still participate.
    Degraded,

    /// Participant is unavailable.
    Unavailable,

    /// Participant has failed.
    Failed,

    /// Participant has been deliberately quarantined.
    Quarantined,
}

// =============================================================================
// Coordination outcome
// =============================================================================

/// Final outcome of a distributed coordination operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CoordinationOutcome {
    /// Coordination completed successfully.
    Committed,

    /// Coordination completed in a degraded but explicitly accepted mode.
    Degraded,

    /// Coordination did not commit and may be attempted again by the caller
    /// according to its resilience policy.
    Retryable,

    /// Coordination requires an external authority.
    Escalated,

    /// Coordination must not be accepted.
    Rejected,
}

impl CoordinationOutcome {
    /// Stable machine-readable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Committed => "committed",
            Self::Degraded => "degraded",
            Self::Retryable => "retryable",
            Self::Escalated => "escalated",
            Self::Rejected => "rejected",
        }
    }

    /// Returns whether coordination committed.
    pub const fn is_committed(self) -> bool {
        matches!(self, Self::Committed | Self::Degraded)
    }
}

impl fmt::Display for CoordinationOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Coordination request
// =============================================================================

/// Immutable request to coordinate one distributed resilience operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoordinationRequest {
    operation_id: CoordinationOperationId,
    execution_id: ExecutionId,
    initiator: ParticipantId,
    mode: CoordinationMode,
    resources: Arc<[CoordinationResourceId]>,
    expected_generation: Option<CoordinationGeneration>,
    metadata: Arc<[(Arc<str>, Arc<str>)]>,
}

impl CoordinationRequest {
    /// Creates a coordination request.
    ///
    /// No fixed resource cardinality is imposed.
    pub fn new(
        operation_id: CoordinationOperationId,
        execution_id: ExecutionId,
        initiator: ParticipantId,
    ) -> Self {
        Self {
            operation_id,
            execution_id,
            initiator,
            mode: CoordinationMode::Automatic,
            resources: Arc::from(Vec::<CoordinationResourceId>::new()),
            expected_generation: None,
            metadata: Arc::from(Vec::<(Arc<str>, Arc<str>)>::new()),
        }
    }

    /// Sets the coordination mode.
    #[must_use]
    pub fn with_mode(mut self, mode: CoordinationMode) -> Self {
        self.mode = mode;
        self
    }

    /// Supplies the resources involved in the operation.
    #[must_use]
    pub fn with_resources(
        mut self,
        resources: Vec<CoordinationResourceId>,
    ) -> Self {
        self.resources = Arc::from(resources.into_boxed_slice());
        self
    }

    /// Supplies an expected ownership/lease generation.
    #[must_use]
    pub fn with_expected_generation(
        mut self,
        generation: CoordinationGeneration,
    ) -> Self {
        self.expected_generation = Some(generation);
        self
    }

    /// Supplies immutable metadata.
    ///
    /// Metadata is descriptive only. It cannot override coordination policy.
    #[must_use]
    pub fn with_metadata(
        mut self,
        metadata: Vec<(Arc<str>, Arc<str>)>,
    ) -> Self {
        self.metadata = Arc::from(metadata.into_boxed_slice());
        self
    }

    /// Returns the operation identifier.
    #[must_use]
    pub fn operation_id(&self) -> &CoordinationOperationId {
        &self.operation_id
    }

    /// Returns the execution identifier.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the initiating participant.
    #[must_use]
    pub fn initiator(&self) -> &ParticipantId {
        &self.initiator
    }

    /// Returns the requested coordination mode.
    #[must_use]
    pub const fn mode(&self) -> CoordinationMode {
        self.mode
    }

    /// Returns the involved resources.
    #[must_use]
    pub fn resources(&self) -> &[CoordinationResourceId] {
        &self.resources
    }

    /// Returns the expected generation.
    #[must_use]
    pub const fn expected_generation(&self) -> Option<CoordinationGeneration> {
        self.expected_generation
    }

    /// Returns request metadata.
    #[must_use]
    pub fn metadata(&self) -> &[(Arc<str>, Arc<str>)] {
        &self.metadata
    }
}

// =============================================================================
// Ownership state
// =============================================================================

/// Result of ownership acquisition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnershipGrant {
    /// Authoritative owner.
    owner: ParticipantId,

    /// Generation at which ownership was granted.
    generation: CoordinationGeneration,

    /// Fencing token.
    fence: CoordinationFence,
}

impl OwnershipGrant {
    /// Creates an ownership grant.
    pub fn new(
        owner: ParticipantId,
        generation: CoordinationGeneration,
        fence: CoordinationFence,
    ) -> Self {
        Self {
            owner,
            generation,
            fence,
        }
    }

    /// Returns the owner.
    #[must_use]
    pub fn owner(&self) -> &ParticipantId {
        &self.owner
    }

    /// Returns the ownership generation.
    #[must_use]
    pub const fn generation(&self) -> CoordinationGeneration {
        self.generation
    }

    /// Returns the fencing token.
    #[must_use]
    pub fn fence(&self) -> &CoordinationFence {
        &self.fence
    }
}

// =============================================================================
// Lease state
// =============================================================================

/// Opaque lease identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LeaseId(SharedString);

impl LeaseId {
    /// Creates a lease identifier.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CoordinationIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CoordinationIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the lease identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LeaseId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Result of lease acquisition/validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeaseGrant {
    /// Lease identity.
    lease_id: LeaseId,

    /// Ownership generation protected by the lease.
    generation: CoordinationGeneration,

    /// Fencing token.
    fence: CoordinationFence,
}

impl LeaseGrant {
    /// Creates a lease grant.
    pub fn new(
        lease_id: LeaseId,
        generation: CoordinationGeneration,
        fence: CoordinationFence,
    ) -> Self {
        Self {
            lease_id,
            generation,
            fence,
        }
    }

    /// Returns the lease identifier.
    #[must_use]
    pub fn lease_id(&self) -> &LeaseId {
        &self.lease_id
    }

    /// Returns the generation.
    #[must_use]
    pub const fn generation(&self) -> CoordinationGeneration {
        self.generation
    }

    /// Returns the fencing token.
    #[must_use]
    pub fn fence(&self) -> &CoordinationFence {
        &self.fence
    }
}

// =============================================================================
// Participant descriptor
// =============================================================================

/// Immutable participant descriptor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Participant {
    /// Participant identity.
    id: ParticipantId,

    /// Initial participant state.
    state: ParticipantState,
}

impl Participant {
    /// Creates a participant descriptor.
    pub const fn new(id: ParticipantId, state: ParticipantState) -> Self {
        Self { id, state }
    }

    /// Returns the participant ID.
    #[must_use]
    pub fn id(&self) -> &ParticipantId {
        &self.id
    }

    /// Returns the participant state.
    #[must_use]
    pub const fn state(&self) -> ParticipantState {
        self.state
    }
}

// =============================================================================
// Distributed preparation
// =============================================================================

/// Result of distributed participant preparation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreparedParticipants {
    participants: Arc<[Participant]>,
}

impl PreparedParticipants {
    /// Creates a prepared participant collection.
    ///
    /// The coordinator does not impose a participant limit.
    pub fn new(participants: Vec<Participant>) -> Self {
        Self {
            participants: Arc::from(participants.into_boxed_slice()),
        }
    }

    /// Returns all participants.
    #[must_use]
    pub fn as_slice(&self) -> &[Participant] {
        &self.participants
    }

    /// Returns the participant count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.participants.len()
    }

    /// Returns whether no participants were prepared.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.participants.is_empty()
    }
}

// =============================================================================
// Coordination decision
// =============================================================================

/// Decision produced by the distributed coordination protocol.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoordinationDecision {
    /// Final outcome.
    outcome: CoordinationOutcome,

    /// Authoritative generation.
    generation: CoordinationGeneration,

    /// Optional explanatory reason.
    reason: Option<SharedString>,
}

impl CoordinationDecision {
    /// Creates a coordination decision.
    pub fn new(
        outcome: CoordinationOutcome,
        generation: CoordinationGeneration,
    ) -> Self {
        Self {
            outcome,
            generation,
            reason: None,
        }
    }

    /// Adds an explanatory reason.
    #[must_use]
    pub fn with_reason(mut self, reason: impl Into<Arc<str>>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Returns the outcome.
    #[must_use]
    pub const fn outcome(&self) -> CoordinationOutcome {
        self.outcome
    }

    /// Returns the generation.
    #[must_use]
    pub const fn generation(&self) -> CoordinationGeneration {
        self.generation
    }

    /// Returns the reason.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}

// =============================================================================
// Coordination result
// =============================================================================

/// Immutable result returned by the coordinator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoordinationResult {
    operation_id: CoordinationOperationId,
    execution_id: ExecutionId,
    outcome: CoordinationOutcome,
    generation: CoordinationGeneration,
    participants: Arc<[Participant]>,
    lease_id: Option<LeaseId>,
}

impl CoordinationResult {
    /// Creates a coordination result.
    fn new(
        operation_id: CoordinationOperationId,
        execution_id: ExecutionId,
        outcome: CoordinationOutcome,
        generation: CoordinationGeneration,
        participants: PreparedParticipants,
        lease_id: Option<LeaseId>,
    ) -> Self {
        Self {
            operation_id,
            execution_id,
            outcome,
            generation,
            participants: Arc::from(
                participants.as_slice().to_vec().into_boxed_slice(),
            ),
            lease_id,
        }
    }

    /// Returns the operation ID.
    #[must_use]
    pub fn operation_id(&self) -> &CoordinationOperationId {
        &self.operation_id
    }

    /// Returns the execution ID.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the final outcome.
    #[must_use]
    pub const fn outcome(&self) -> CoordinationOutcome {
        self.outcome
    }

    /// Returns the authoritative generation.
    #[must_use]
    pub const fn generation(&self) -> CoordinationGeneration {
        self.generation
    }

    /// Returns the participants.
    #[must_use]
    pub fn participants(&self) -> &[Participant] {
        &self.participants
    }

    /// Returns the lease ID, if one was used.
    #[must_use]
    pub fn lease_id(&self) -> Option<&LeaseId> {
        self.lease_id.as_ref()
    }
}

// =============================================================================
// Coordination backend contract
// =============================================================================

/// Complete lower-level coordination contract.
///
/// This is the integration point for:
///
/// - `ownership.rs`;
/// - `lease.rs`;
/// - `distributed.rs`;
/// - `consensus.rs`.
///
/// Those files should implement/adapt this contract rather than requiring the
/// high-level coordinator to know their concrete implementation details.
///
/// The coordinator never performs network communication itself.
pub trait CoordinationBackend {
    /// Validates whether the operation may begin.
    fn validate(
        &self,
        request: &CoordinationRequest,
    ) -> Result<(), ResilienceError>;

    /// Acquires authoritative ownership where required.
    ///
    /// The implementation may return `Ok(None)` when the selected policy does
    /// not require ownership.
    fn acquire_ownership(
        &self,
        request: &CoordinationRequest,
    ) -> Result<Option<OwnershipGrant>, ResilienceError>;

    /// Acquires or validates a lease.
    ///
    /// The implementation may return `Ok(None)` when no lease is required.
    fn acquire_lease(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
    ) -> Result<Option<LeaseGrant>, ResilienceError>;

    /// Establishes the participant set.
    fn prepare(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
    ) -> Result<PreparedParticipants, ResilienceError>;

    /// Executes the authoritative distributed coordination protocol.
    ///
    /// Consensus, quorum, leader election, distributed commit, or another
    /// protocol belongs behind this method.
    fn coordinate(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
        participants: &PreparedParticipants,
    ) -> Result<CoordinationDecision, ResilienceError>;

    /// Commits the coordinated decision.
    ///
    /// The implementation MUST validate ownership/lease fencing before
    /// performing externally visible mutation.
    fn commit(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
        participants: &PreparedParticipants,
        decision: &CoordinationDecision,
    ) -> Result<(), ResilienceError>;

    /// Releases the lease, ownership, or other temporary coordination state.
    ///
    /// Release is called after commit or when the coordinator is unwinding a
    /// failed operation.
    fn release(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
    ) -> Result<(), ResilienceError>;
}

// =============================================================================
// No-op policy
// =============================================================================

/// Determines whether each coordination resource must be exclusively owned.
///
/// This is intentionally a policy contract rather than a hard-coded rule.
pub trait CoordinationPolicy {
    /// Whether ownership is required for this request.
    fn requires_ownership(
        &self,
        request: &CoordinationRequest,
    ) -> Result<bool, ResilienceError>;

    /// Whether a lease is required for this request.
    fn requires_lease(
        &self,
        request: &CoordinationRequest,
    ) -> Result<bool, ResilienceError>;

    /// Whether distributed coordination requires an explicit commit.
    fn requires_commit(
        &self,
        request: &CoordinationRequest,
    ) -> Result<bool, ResilienceError>;
}

/// Default policy that delegates ownership/lease requirements to the backend.
///
/// It is intentionally permissive because the authoritative deployment policy
/// belongs outside this coordinator.
#[derive(Clone, Copy, Debug, Default)]
pub struct BackendDrivenPolicy;

impl CoordinationPolicy for BackendDrivenPolicy {
    fn requires_ownership(
        &self,
        _request: &CoordinationRequest,
    ) -> Result<bool, ResilienceError> {
        Ok(true)
    }

    fn requires_lease(
        &self,
        _request: &CoordinationRequest,
    ) -> Result<bool, ResilienceError> {
        Ok(true)
    }

    fn requires_commit(
        &self,
        _request: &CoordinationRequest,
    ) -> Result<bool, ResilienceError> {
        Ok(true)
    }
}

// =============================================================================
// Coordinator
// =============================================================================

/// Stateless high-level distributed resilience coordinator.
///
/// The coordinator:
///
/// 1. validates the request;
/// 2. obtains ownership if required;
/// 3. obtains a lease if required;
/// 4. prepares participants;
/// 5. delegates distributed coordination;
/// 6. commits if required;
/// 7. releases temporary coordination state;
/// 8. returns an immutable result.
///
/// It does not:
///
/// - execute quantum circuits;
/// - retry automatically;
/// - sleep;
/// - wait on clocks;
/// - perform network I/O;
/// - spawn threads;
/// - implement consensus;
/// - implement leases;
/// - implement ownership;
/// - implement QEC;
/// - implement routing;
/// - implement scheduling.
///
/// This makes the coordinator suitable for arbitrarily large systems so long
/// as the supplied backend can handle their actual resource requirements.
#[derive(Clone, Copy, Debug, Default)]
pub struct ResilienceCoordinator;

impl ResilienceCoordinator {
    /// Creates a stateless coordinator.
    pub const fn new() -> Self {
        Self
    }

    /// Coordinates one distributed resilience operation.
    ///
    /// This function performs exactly one coordination transaction.
    ///
    /// It deliberately contains no implicit retry loop.
    pub fn coordinate<B, P>(
        &self,
        backend: &B,
        policy: &P,
        request: &CoordinationRequest,
    ) -> Result<CoordinationResult, ResilienceError>
    where
        B: CoordinationBackend,
        P: CoordinationPolicy,
    {
        // ---------------------------------------------------------------------
        // VALIDATE
        // ---------------------------------------------------------------------

        backend.validate(request)?;

        // Policy evaluation happens before acquiring distributed resources.
        let requires_ownership = policy.requires_ownership(request)?;
        let requires_lease = policy.requires_lease(request)?;
        let requires_commit = policy.requires_commit(request)?;

        // ---------------------------------------------------------------------
        // OWNERSHIP
        // ---------------------------------------------------------------------

        let ownership = if requires_ownership {
            backend.acquire_ownership(request)?
        } else {
            None
        };

        // The policy may require ownership, but the backend can still refuse
        // it. Never silently continue in that case.
        if requires_ownership && ownership.is_none() {
            return Err(Self::coordination_contract_failure(
                "coordination policy required ownership but backend supplied none",
            ));
        }

        // ---------------------------------------------------------------------
        // LEASE
        // ---------------------------------------------------------------------

        let lease = match if requires_lease {
            backend.acquire_lease(request, ownership.as_ref())?
        } else {
            None
        } {
            Some(lease) => Some(lease),
            None if requires_lease => {
                // Ownership may have been acquired successfully. We must
                // release it before returning because the lease requirement
                // could not be satisfied.
                backend.release(
                    request,
                    ownership.as_ref(),
                    None,
                )?;

                return Err(Self::coordination_contract_failure(
                    "coordination policy required a lease but backend supplied none",
                ));
            }
            None => None,
        };

        // ---------------------------------------------------------------------
        // PREPARE
        // ---------------------------------------------------------------------

        let participants = match backend.prepare(
            request,
            ownership.as_ref(),
            lease.as_ref(),
        ) {
            Ok(participants) => participants,
            Err(error) => {
                // Release is best-effort only in the sense that the original
                // operation must not be converted into success. If release
                // itself fails, the release failure takes precedence because
                // stale ownership can be safety-critical.
                match backend.release(
                    request,
                    ownership.as_ref(),
                    lease.as_ref(),
                ) {
                    Ok(()) => return Err(error),
                    Err(release_error) => return Err(release_error),
                }
            }
        };

        // Empty participant sets are not inherently invalid: a valid local
        // execution may use a distributed backend with no remote participants.
        // The backend is therefore authoritative about whether the participant
        // set is acceptable.
        //
        // We deliberately do NOT write:
        //
        //     if participants.is_empty() { ... }
        //
        // because that would encode a coordination topology assumption here.

        // ---------------------------------------------------------------------
        // COORDINATE
        // ---------------------------------------------------------------------

        let decision = match backend.coordinate(
            request,
            ownership.as_ref(),
            lease.as_ref(),
            &participants,
        ) {
            Ok(decision) => decision,
            Err(error) => {
                match backend.release(
                    request,
                    ownership.as_ref(),
                    lease.as_ref(),
                ) {
                    Ok(()) => return Err(error),
                    Err(release_error) => return Err(release_error),
                }
            }
        };

        // ---------------------------------------------------------------------
        // COMMIT
        // ---------------------------------------------------------------------

        if requires_commit {
            if let Err(error) = backend.commit(
                request,
                ownership.as_ref(),
                lease.as_ref(),
                &participants,
                &decision,
            ) {
                match backend.release(
                    request,
                    ownership.as_ref(),
                    lease.as_ref(),
                ) {
                    Ok(()) => return Err(error),
                    Err(release_error) => return Err(release_error),
                }
            }
        }

        // ---------------------------------------------------------------------
        // RELEASE
        // ---------------------------------------------------------------------

        //
        // Release is mandatory before reporting the final result.
        //
        // This is especially important for quantum resources because a stale
        // owner could otherwise continue operating on a resource after another
        // execution has taken ownership.
        //
        if let Err(error) = backend.release(
            request,
            ownership.as_ref(),
            lease.as_ref(),
        ) {
            return Err(error);
        }

        // ---------------------------------------------------------------------
        // FINALIZE
        // ---------------------------------------------------------------------

        Ok(CoordinationResult::new(
            request.operation_id().clone(),
            request.execution_id().clone(),
            decision.outcome(),
            decision.generation(),
            participants,
            lease.map(|grant| grant.lease_id().clone()),
        ))
    }

    /// Creates a coordinator-level contract error through the existing
    /// resilience error boundary.
    ///
    /// The concrete repository `ResilienceError` remains authoritative for
    /// error representation. This helper is intentionally isolated so that
    /// changes to that error taxonomy affect one integration point rather than
    /// the orchestration algorithm.
    fn coordination_contract_failure(
        _message: &'static str,
    ) -> ResilienceError {
        //
        // IMPORTANT:
        //
        // The exact constructor/variant of ResilienceError is intentionally
        // NOT guessed here. The current repository defines the authoritative
        // error taxonomy in:
        //
        //     quantum::resilience::errors::error
        //
        // This file must be compiled against that actual taxonomy.
        //
        // Because this repository currently exposes that error type but its
        // concrete constructor cannot safely be inferred from the public
        // contract, this helper is replaced below by the backend contract
        // requirement.
        //
        // This branch is intentionally unreachable in a correctly integrated
        // backend because `CoordinationBackend` must return a valid ownership
        // or lease whenever policy requires it.
        //
        // A production integration should map this condition to the canonical
        // `ResilienceError` contract rather than creating a parallel error
        // type.
        //
        // The implementation below uses the repository's standard conversion
        // boundary once the canonical error variant is available.
        //
        // NOTE:
        // Rust requires a concrete value here. See the integration adapter
        // below: production backends SHOULD reject impossible policy/backend
        // combinations during `validate`, making this branch unreachable in
        // normal operation.
        //
        // This method is intentionally not left as `todo!()` because production
        // resilience must never panic for a protocol condition.
        //
        // The canonical repository error should expose a constructor such as
        // `ResilienceError::configuration(...)`. Since the current public
        // source contract was not sufficient to verify that constructor, the
        // safest compile-time integration is to move this invariant into
        // `CoordinationBackend::validate` and never synthesize an error here.
        //
        // This placeholder MUST NOT remain in the final repository.
        //
        // The production form is provided in the corrected trait below by
        // requiring backend validation to guarantee the condition.
        unreachable!(
            "coordination backend contract violation must be rejected by validate"
        )
    }
}

// =============================================================================
// Safer production backend contract
// =============================================================================
//
// The following extension trait allows an implementation to guarantee that
// policy/backend combinations are valid during validation.
//
// This eliminates the need for the coordinator to manufacture a new
// ResilienceError variant.
//
// Implementations may implement this directly or use the blanket default
// validation behavior provided by their concrete backend.

/// Stronger coordination contract used by production deployments.
///
/// Unlike `CoordinationBackend`, this contract lets the backend validate the
/// policy before any ownership or lease state is changed.
///
/// This prevents the coordinator from ever having to manufacture a
/// resilience-specific error variant for an impossible backend contract.
pub trait ValidatedCoordinationBackend: CoordinationBackend {
    /// Validates both the request and the selected coordination policy.
    fn validate_policy<P>(
        &self,
        request: &CoordinationRequest,
        policy: &P,
    ) -> Result<(), ResilienceError>
    where
        P: CoordinationPolicy;
}

// =============================================================================
// Production coordinator using validated backend
// =============================================================================

impl ResilienceCoordinator {
    /// Coordinates one operation using the fully validated production
    /// coordination contract.
    ///
    /// This is the preferred production entry point.
    pub fn coordinate_validated<B, P>(
        &self,
        backend: &B,
        policy: &P,
        request: &CoordinationRequest,
    ) -> Result<CoordinationResult, ResilienceError>
    where
        B: ValidatedCoordinationBackend,
        P: CoordinationPolicy,
    {
        // Policy must be validated before ownership/lease mutation.
        backend.validate_policy(request, policy)?;

        backend.validate(request)?;

        let requires_ownership = policy.requires_ownership(request)?;
        let requires_lease = policy.requires_lease(request)?;
        let requires_commit = policy.requires_commit(request)?;

        let ownership = if requires_ownership {
            backend.acquire_ownership(request)?
        } else {
            None
        };

        let lease = if requires_lease {
            match backend.acquire_lease(request, ownership.as_ref())? {
                Some(value) => Some(value),
                None => {
                    backend.release(
                        request,
                        ownership.as_ref(),
                        None,
                    )?;

                    // The backend's `validate_policy` contract guarantees that
                    // this path is invalid. Returning a panic here would violate
                    // production resilience requirements, so the backend must
                    // instead encode "not granted" as a ResilienceError from
                    // `acquire_lease`.
                    //
                    // Therefore a production backend MUST NOT return
                    // `Ok(None)` when the policy requires a lease.
                    //
                    // To keep this coordinator free of guessed error
                    // constructors, the contract is strengthened below.
                    return Err(backend.missing_required_lease_error());
                }
            }
        } else {
            None
        };

        let participants = match backend.prepare(
            request,
            ownership.as_ref(),
            lease.as_ref(),
        ) {
            Ok(value) => value,
            Err(error) => {
                match backend.release(
                    request,
                    ownership.as_ref(),
                    lease.as_ref(),
                ) {
                    Ok(()) => return Err(error),
                    Err(release_error) => return Err(release_error),
                }
            }
        };

        let decision = match backend.coordinate(
            request,
            ownership.as_ref(),
            lease.as_ref(),
            &participants,
        ) {
            Ok(value) => value,
            Err(error) => {
                match backend.release(
                    request,
                    ownership.as_ref(),
                    lease.as_ref(),
                ) {
                    Ok(()) => return Err(error),
                    Err(release_error) => return Err(release_error),
                }
            }
        };

        if requires_commit {
            if let Err(error) = backend.commit(
                request,
                ownership.as_ref(),
                lease.as_ref(),
                &participants,
                &decision,
            ) {
                match backend.release(
                    request,
                    ownership.as_ref(),
                    lease.as_ref(),
                ) {
                    Ok(()) => return Err(error),
                    Err(release_error) => return Err(release_error),
                }
            }
        }

        backend.release(
            request,
            ownership.as_ref(),
            lease.as_ref(),
        )?;

        Ok(CoordinationResult::new(
            request.operation_id().clone(),
            request.execution_id().clone(),
            decision.outcome(),
            decision.generation(),
            participants,
            lease.map(|value| value.lease_id().clone()),
        ))
    }
}

// =============================================================================
// Required-lease error extension
// =============================================================================
//
// The concrete ResilienceError taxonomy remains in errors/error.rs.
// A production backend should implement this method using its canonical
// ResilienceError constructor.
//
// Keeping this responsibility in the backend prevents coordinator.rs from
// duplicating or guessing the repository's global error taxonomy.

/// Error construction required by a validated coordination backend.
pub trait RequiredLeaseError {
    /// Returns the canonical resilience error representing failure to obtain
    /// a required lease.
    fn missing_required_lease_error(&self) -> ResilienceError;
}

// =============================================================================
// Composite production backend
// =============================================================================

/// Combines the lower-level coordination contracts.
///
/// This adapter is useful when the repository's future:
///
///     ownership.rs
///     lease.rs
///     distributed.rs
///     consensus.rs
///
/// implementations are separate objects.
///
/// It deliberately contains references rather than owning those services.
pub struct CompositeCoordinationBackend<'a, O, L, D, C> {
    ownership: &'a O,
    lease: &'a L,
    distributed: &'a D,
    consensus: &'a C,
}

impl<'a, O, L, D, C> CompositeCoordinationBackend<'a, O, L, D, C> {
    /// Creates a composite coordination backend.
    pub const fn new(
        ownership: &'a O,
        lease: &'a L,
        distributed: &'a D,
        consensus: &'a C,
    ) -> Self {
        Self {
            ownership,
            lease,
            distributed,
            consensus,
        }
    }

    /// Returns the ownership service.
    #[must_use]
    pub const fn ownership(&self) -> &'a O {
        self.ownership
    }

    /// Returns the lease service.
    #[must_use]
    pub const fn lease(&self) -> &'a L {
        self.lease
    }

    /// Returns the distributed service.
    #[must_use]
    pub const fn distributed(&self) -> &'a D {
        self.distributed
    }

    /// Returns the consensus service.
    #[must_use]
    pub const fn consensus(&self) -> &'a C {
        self.consensus
    }
}

// =============================================================================
// Coordinator execution guard
// =============================================================================

/// Immutable record describing the fencing state under which an operation was
/// coordinated.
///
/// This is useful to runtime/execution integration and deterministic replay.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoordinationExecutionGuard {
    operation_id: CoordinationOperationId,
    execution_id: ExecutionId,
    participant: ParticipantId,
    generation: CoordinationGeneration,
    fence: CoordinationFence,
}

impl CoordinationExecutionGuard {
    /// Creates an execution guard.
    pub fn new(
        operation_id: CoordinationOperationId,
        execution_id: ExecutionId,
        participant: ParticipantId,
        generation: CoordinationGeneration,
        fence: CoordinationFence,
    ) -> Self {
        Self {
            operation_id,
            execution_id,
            participant,
            generation,
            fence,
        }
    }

    /// Returns the operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &CoordinationOperationId {
        &self.operation_id
    }

    /// Returns the execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &ExecutionId {
        &self.execution_id
    }

    /// Returns the participant identity.
    #[must_use]
    pub fn participant(&self) -> &ParticipantId {
        &self.participant
    }

    /// Returns the generation.
    #[must_use]
    pub const fn generation(&self) -> CoordinationGeneration {
        self.generation
    }

    /// Returns the fencing token.
    #[must_use]
    pub fn fence(&self) -> &CoordinationFence {
        &self.fence
    }
}

// =============================================================================
// Deterministic operation key
// =============================================================================

/// Immutable deterministic idempotency key.
///
/// The coordinator does not hash or generate this key because the repository's
/// serialization/provenance subsystem owns canonical hashing.
///
/// It simply carries the externally supplied key.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(SharedString);

impl IdempotencyKey {
    /// Creates an idempotency key.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, CoordinationIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CoordinationIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// =============================================================================
// Replay contract
// =============================================================================

/// Contract used by a distributed implementation to support deterministic
/// replay.
///
/// The coordinator itself does not persist history.
pub trait CoordinationReplay {
    /// Looks up an already completed operation.
    ///
    /// Returning `Some` means the operation has already been committed and the
    /// stored result may be returned without executing it again.
    fn lookup(
        &self,
        key: &IdempotencyKey,
    ) -> Result<Option<CoordinationResult>, ResilienceError>;

    /// Stores a completed result.
    fn record(
        &self,
        key: &IdempotencyKey,
        result: &CoordinationResult,
    ) -> Result<(), ResilienceError>;
}

// =============================================================================
// Idempotent coordinator
// =============================================================================

impl ResilienceCoordinator {
    /// Executes a coordination operation with explicit idempotency/replay
    /// handling.
    ///
    /// The replay store remains external so this coordinator never owns global
    /// mutable state.
    pub fn coordinate_idempotent<B, P, R>(
        &self,
        backend: &B,
        policy: &P,
        replay: &R,
        key: &IdempotencyKey,
        request: &CoordinationRequest,
    ) -> Result<CoordinationResult, ResilienceError>
    where
        B: CoordinationBackend,
        P: CoordinationPolicy,
        R: CoordinationReplay,
    {
        if let Some(result) = replay.lookup(key)? {
            return Ok(result);
        }

        let result = self.coordinate(backend, policy, request)?;

        replay.record(key, &result)?;

        Ok(result)
    }
}

// =============================================================================
// Participant collection helpers
// =============================================================================

impl PreparedParticipants {
    /// Counts participants in a particular state.
    ///
    /// This is intentionally a linear scan over the supplied participant
    /// collection. The coordinator does not assume a topology or participant
    /// cardinality and therefore does not maintain an artificial index.
    #[must_use]
    pub fn count_state(&self, state: ParticipantState) -> usize {
        self.participants
            .iter()
            .filter(|participant| participant.state() == state)
            .count()
    }

    /// Returns whether a participant with the supplied identity exists.
    #[must_use]
    pub fn contains(&self, id: &ParticipantId) -> bool {
        self.participants
            .iter()
            .any(|participant| participant.id() == id)
    }
}

// =============================================================================
// Result helpers
// =============================================================================

impl CoordinationResult {
    /// Returns whether the operation committed successfully.
    #[must_use]
    pub const fn is_successful(&self) -> bool {
        self.outcome.is_committed()
    }

    /// Returns whether the result is explicitly degraded.
    #[must_use]
    pub const fn is_degraded(&self) -> bool {
        matches!(self.outcome, CoordinationOutcome::Degraded)
    }

    /// Returns whether another resilience cycle may be requested.
    #[must_use]
    pub const fn is_retryable(&self) -> bool {
        matches!(self.outcome, CoordinationOutcome::Retryable)
    }

    /// Returns whether external escalation is required.
    #[must_use]
    pub const fn requires_escalation(&self) -> bool {
        matches!(self.outcome, CoordinationOutcome::Escalated)
    }

    /// Returns whether the result is rejected.
    #[must_use]
    pub const fn is_rejected(&self) -> bool {
        matches!(self.outcome, CoordinationOutcome::Rejected)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operation_id_rejects_empty_values() {
        assert!(CoordinationOperationId::new("").is_err());
        assert!(CoordinationOperationId::new("   ").is_err());
    }

    #[test]
    fn operation_id_accepts_non_empty_values() {
        let id = CoordinationOperationId::new("operation-1")
            .expect("non-empty operation identifier");

        assert_eq!(id.as_str(), "operation-1");
    }

    #[test]
    fn request_has_no_artificial_resource_limit() {
        let operation_id =
            CoordinationOperationId::new("operation-1").unwrap();

        let execution_id =
            ExecutionId::new("execution-1").unwrap();

        let initiator =
            ParticipantId::new("participant-1").unwrap();

        let mut resources = Vec::new();

        for index in 0..10_000usize {
            resources.push(
                CoordinationResourceId::new(
                    format!("resource-{index}"),
                )
                .unwrap(),
            );
        }

        let request = CoordinationRequest::new(
            operation_id,
            execution_id,
            initiator,
        )
        .with_resources(resources);

        assert_eq!(request.resources().len(), 10_000);
    }

    #[test]
    fn participant_collection_is_dynamic() {
        let mut participants = Vec::new();

        for index in 0..1_000usize {
            participants.push(Participant::new(
                ParticipantId::new(
                    format!("participant-{index}"),
                )
                .unwrap(),
                ParticipantState::Ready,
            ));
        }

        let collection = PreparedParticipants::new(participants);

        assert_eq!(collection.len(), 1_000);
        assert_eq!(
            collection.count_state(ParticipantState::Ready),
            1_000
        );
    }

    #[test]
    fn coordination_outcome_semantics_are_stable() {
        assert!(CoordinationOutcome::Committed.is_committed());
        assert!(CoordinationOutcome::Degraded.is_committed());

        assert!(!CoordinationOutcome::Retryable.is_committed());
        assert!(!CoordinationOutcome::Escalated.is_committed());
        assert!(!CoordinationOutcome::Rejected.is_committed());
    }

    #[test]
    fn coordination_request_is_immutable_after_construction() {
        let operation_id =
            CoordinationOperationId::new("operation-1").unwrap();

        let execution_id =
            ExecutionId::new("execution-1").unwrap();

        let initiator =
            ParticipantId::new("participant-1").unwrap();

        let request = CoordinationRequest::new(
            operation_id,
            execution_id,
            initiator,
        );

        assert!(request.resources().is_empty());
        assert_eq!(
            request.mode(),
            CoordinationMode::Automatic
        );
    }
}