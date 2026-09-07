//! Zamani Quantum Resilience — Distributed Coordination Backend
//!
//! Path:
//!     src/quantum/resilience/coordination/distributed.rs
//!
//! Purpose:
//!     Provider-neutral distributed coordination backend for the resilience
//!     subsystem.
//!
//! Architectural position:
//!
//!     Zamani Program
//!          |
//!          v
//!     Canonical Quantum IR
//!          |
//!          v
//!     Resilience
//!          |
//!          v
//!     coordination::coordinator
//!          |
//!          v
//!     coordination::distributed
//!       /       |        \
//! ownership   lease    consensus
//!                    \
//!                     participant transport
//!
//! This module is the integration layer between the high-level
//! `ResilienceCoordinator` and independently replaceable distributed
//! coordination services.
//!
//! It does NOT implement:
//!
//! - quantum routing;
//! - scheduling;
//! - QEC;
//! - decoding;
//! - hardware discovery;
//! - calibration;
//! - optimization;
//! - quantum circuit execution;
//! - error mitigation;
//! - semantic/result verification;
//! - a specific consensus algorithm;
//! - a specific network protocol;
//! - a specific quantum provider.
//!
//! Those responsibilities remain in their authoritative subsystems.
//!
//! -----------------------------------------------------------------------------
//! PRODUCTION REQUIREMENTS
//! -----------------------------------------------------------------------------
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021 edition
//! - stable Rust
//! - no unsafe code
//! - no hard-coded machine size
//! - no hard-coded qubit count
//! - no hard-coded node count
//! - no hard-coded quorum
//! - no hard-coded retry count
//! - no hard-coded timeout
//! - no hard-coded lease duration
//! - no hard-coded topology
//! - no provider-specific branches
//! - no hidden global mutable state
//! - no implicit threads
//! - no implicit I/O
//! - explicit fencing
//! - explicit ownership
//! - explicit lease lifecycle
//! - explicit participant preparation
//! - explicit durable coordination decision
//! - idempotent commit semantics
//! - deterministic behavior when dependencies are deterministic
//! - scalable participant collections
//! - graceful handling of partial failures
//! - no assumption that quantum state can be rolled back
//!
//! -----------------------------------------------------------------------------
//! SCALABILITY
//! -----------------------------------------------------------------------------
//!
//! This file imposes no artificial finite resource limit.
//!
//! The number of:
//!
//! - participants;
//! - QPUs;
//! - nodes;
//! - resources;
//! - logical qubits;
//! - physical qubits;
//! - distributed domains
//!
//! is determined by the supplied implementation and available resources.
//!
//! The implementation deliberately avoids fixed-size arrays and constants such
//! as MAX_NODES, MAX_QUBITS, MAX_PARTICIPANTS, or fixed quorum sizes.
//!
//! Actual scalability is bounded by the resources of the deployment:
//!
//!     CPU + memory + communication + storage + quantum resources
//!
//! -----------------------------------------------------------------------------
//! QUANTUM IDENTITY
//! -----------------------------------------------------------------------------
//!
//! This module deliberately does not define a QubitId.
//!
//! If a concrete distributed implementation needs logical or physical qubit
//! identity, it must use the canonical definitions:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! The distributed coordination layer itself works with the opaque
//! `CoordinationResourceId` defined by `coordinator.rs`.
//!
//! This prevents accidental mixing of logical and physical qubit identities.
//!
//! -----------------------------------------------------------------------------
//! COMMIT SEMANTICS
//! -----------------------------------------------------------------------------
//!
//! Distributed quantum execution requires an important distinction:
//!
//!     distributed decision != quantum state rollback
//!
//! A committed coordination decision means that the distributed control plane
//! has durably accepted an action according to its coordination protocol.
//!
//! It does NOT imply that an arbitrary quantum state can be serialized,
//! restored, or undone.
//!
//! Quantum execution/recovery/checkpoint subsystems must establish whether
//! the actual operation is reversible, restartable, resumable, or migratable.
//!
//! -----------------------------------------------------------------------------
//! FENCING
//! -----------------------------------------------------------------------------
//!
//! Ownership and leases produce a fencing token.
//!
//! Every externally visible distributed mutation must carry the authoritative
//! fence through the lower-level service boundary.
//!
//! A stale owner must never be able to commit after another owner has acquired
//! a newer generation.
//!
//! -----------------------------------------------------------------------------
//! FAILURE MODEL
//! -----------------------------------------------------------------------------
//!
//! The backend distinguishes:
//!
//! 1. validation failure;
//! 2. ownership failure;
//! 3. lease failure;
//! 4. participant preparation failure;
//! 5. consensus failure;
//! 6. participant commit failure;
//! 7. finalization failure;
//! 8. cleanup failure.
//!
//! A failure is never silently converted into success.
//!
//! If durable consensus has already accepted a commit but participant
//! application subsequently fails, the operation remains recoverable through
//! the durable decision rather than pretending that rollback is possible.
//!
//! -----------------------------------------------------------------------------
//! DEPENDENCY DIRECTION
//! -----------------------------------------------------------------------------
//!
//! coordinator.rs
//!       |
//!       v
//! distributed.rs
//!       |
//!       +--> ownership.rs
//!       +--> lease.rs
//!       +--> consensus.rs
//!       +--> transport/participant implementation
//!
//! Lower-level services must not depend on this orchestration implementation.
//!
//! -----------------------------------------------------------------------------
//! OBSERVABILITY
//! -----------------------------------------------------------------------------
//!
//! Concrete services should emit the repository's canonical telemetry events
//! for:
//!
//! - ownership acquisition/release;
//! - lease acquisition/release/loss;
//! - participant preparation;
//! - coordination decision;
//! - durable commit;
//! - participant commit;
//! - finalization;
//! - coordination conflict;
//! - stale fencing;
//! - cleanup failure.
//!
//! This module does not directly select an observability framework.
//!
//! -----------------------------------------------------------------------------
//! SECURITY
//! -----------------------------------------------------------------------------
//!
//! The backend never treats participant success as sufficient evidence of
//! correctness.
//!
//! Authentication, authorization, telemetry trust, checkpoint integrity and
//! result verification remain separate security boundaries.
//!
//! No credentials, tokens or secrets are stored in this object.
//!
//! -----------------------------------------------------------------------------
//! RUST SAFETY
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::quantum::resilience::coordination::coordinator::{
    CoordinationBackend,
    CoordinationDecision,
    CoordinationGeneration,
    CoordinationMode,
    CoordinationPolicy,
    CoordinationRequest,
    CoordinationResult,
    CoordinationResourceId,
    CoordinationFence,
    LeaseGrant,
    OwnershipGrant,
    Participant,
    ParticipantId,
    ParticipantState,
    PreparedParticipants,
    ResilienceCoordinator,
    ValidatedCoordinationBackend,
    RequiredLeaseError,
};

use crate::quantum::resilience::errors::error::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceErrorCategory,
    ResilienceSeverity,
    Retryability,
    Recoverability,
};

use std::fmt;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the distributed coordination backend.
pub const DISTRIBUTED_COORDINATION_SCHEMA_ID: &str =
    "zamani.quantum.resilience.coordination.distributed";

/// Current distributed coordination contract version.
pub const DISTRIBUTED_COORDINATION_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Distributed service contracts
// =============================================================================

/// Authoritative ownership service.
///
/// The implementation belongs in `ownership.rs`.
///
/// This trait intentionally contains no machine-size assumptions.
pub trait DistributedOwnershipService {
    /// Validates whether ownership can participate in the request.
    fn validate(
        &self,
        request: &CoordinationRequest,
    ) -> Result<(), ResilienceError>;

    /// Acquires authoritative ownership.
    ///
    /// The returned generation and fence are authoritative.
    fn acquire(
        &self,
        request: &CoordinationRequest,
    ) -> Result<Option<OwnershipGrant>, ResilienceError>;

    /// Releases ownership.
    ///
    /// The implementation must verify fencing where appropriate.
    fn release(
        &self,
        request: &CoordinationRequest,
        ownership: &OwnershipGrant,
    ) -> Result<(), ResilienceError>;
}

/// Authoritative lease service.
///
/// The implementation belongs in `lease.rs`.
pub trait DistributedLeaseService {
    /// Validates lease requirements.
    fn validate(
        &self,
        request: &CoordinationRequest,
    ) -> Result<(), ResilienceError>;

    /// Acquires a lease associated with the ownership grant.
    fn acquire(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
    ) -> Result<Option<LeaseGrant>, ResilienceError>;

    /// Releases the lease.
    fn release(
        &self,
        request: &CoordinationRequest,
        lease: &LeaseGrant,
    ) -> Result<(), ResilienceError>;
}

/// Participant/control-plane service.
///
/// This service represents the distributed participants without prescribing
/// whether communication occurs through:
///
/// - classical IP;
/// - quantum-network control channels;
/// - RPC;
/// - local processes;
/// - shared memory;
/// - a simulator;
/// - another transport.
///
/// The transport itself belongs to the implementation.
pub trait DistributedParticipantService {
    /// Validates the request against the participant/control plane.
    fn validate(
        &self,
        request: &CoordinationRequest,
    ) -> Result<(), ResilienceError>;

    /// Prepares the participants.
    ///
    /// Preparation must be safe to repeat when the concrete implementation
    /// supports idempotency.
    fn prepare(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
    ) -> Result<PreparedParticipants, ResilienceError>;

    /// Applies a durable coordination decision to the participants.
    ///
    /// The participant implementation must validate the supplied ownership
    /// and lease fences before performing externally visible mutation.
    fn commit(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
        participants: &PreparedParticipants,
        decision: &CoordinationDecision,
    ) -> Result<(), ResilienceError>;
}

/// Consensus/decision service.
///
/// This abstraction deliberately does not prescribe Raft, Paxos, PBFT,
/// Byzantine agreement, a centralized authority, a quorum service, or any
/// other algorithm.
///
/// The appropriate mechanism is determined by the deployment.
pub trait DistributedConsensusService {
    /// Validates the consensus requirements.
    fn validate(
        &self,
        request: &CoordinationRequest,
    ) -> Result<(), ResilienceError>;

    /// Produces the authoritative coordination decision.
    ///
    /// This method may perform proposal, voting, quorum collection, leader
    /// coordination, or another protocol.
    fn decide(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
        participants: &PreparedParticipants,
    ) -> Result<CoordinationDecision, ResilienceError>;

    /// Durably records the decision before participant mutation.
    ///
    /// This is critical for crash recovery: if participant application fails
    /// after durable decision, the system can recover from the decision rather
    /// than incorrectly attempting to invent a rollback.
    fn record_decision(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
        decision: &CoordinationDecision,
    ) -> Result<(), ResilienceError>;

    /// Finalizes a successfully applied decision.
    fn finalize(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
        participants: &PreparedParticipants,
        decision: &CoordinationDecision,
    ) -> Result<(), ResilienceError>;
}

// =============================================================================
// Optional distributed lifecycle observer
// =============================================================================

/// Lifecycle phase emitted by a distributed backend.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DistributedPhase {
    /// Request validation.
    Validate,

    /// Ownership acquisition.
    Ownership,

    /// Lease acquisition.
    Lease,

    /// Participant preparation.
    Prepare,

    /// Decision formation.
    Decide,

    /// Durable decision record.
    RecordDecision,

    /// Participant commit.
    Commit,

    /// Finalization.
    Finalize,

    /// Cleanup.
    Release,
}

impl DistributedPhase {
    /// Stable machine-readable phase name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validate => "validate",
            Self::Ownership => "ownership",
            Self::Lease => "lease",
            Self::Prepare => "prepare",
            Self::Decide => "decide",
            Self::RecordDecision => "record_decision",
            Self::Commit => "commit",
            Self::Finalize => "finalize",
            Self::Release => "release",
        }
    }
}

/// Observer for distributed coordination lifecycle.
///
/// This is intentionally best-effort from the backend's perspective unless
/// the concrete observer itself is used as a mandatory audit/security
/// boundary elsewhere.
pub trait DistributedObserver {
    /// Observes a phase transition.
    fn observe(
        &self,
        operation: &CoordinationRequest,
        phase: DistributedPhase,
    );
}

/// No-op lifecycle observer.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoopDistributedObserver;

impl DistributedObserver for NoopDistributedObserver {
    fn observe(
        &self,
        _operation: &CoordinationRequest,
        _phase: DistributedPhase,
    ) {
    }
}

// =============================================================================
// Backend
// =============================================================================

/// Production distributed coordination backend.
///
/// This type composes the independently replaceable:
///
/// - ownership service;
/// - lease service;
/// - participant/control-plane service;
/// - consensus service.
///
/// The backend itself is stateless.
///
/// Any persistent state belongs to the underlying services.
///
/// This is intentional: a resilience process may crash and restart without
/// losing authoritative distributed state.
pub struct DistributedCoordinationBackend<O, L, P, C, E = NoopDistributedObserver> {
    ownership: O,
    lease: L,
    participants: P,
    consensus: C,
    observer: E,
}

impl<O, L, P, C> DistributedCoordinationBackend<O, L, P, C, NoopDistributedObserver> {
    /// Creates a backend using the supplied distributed services.
    ///
    /// No service is created implicitly.
    pub const fn new(
        ownership: O,
        lease: L,
        participants: P,
        consensus: C,
    ) -> Self {
        Self {
            ownership,
            lease,
            participants,
            consensus,
            observer: NoopDistributedObserver,
        }
    }
}

impl<O, L, P, C, E> DistributedCoordinationBackend<O, L, P, C, E> {
    /// Creates a backend with an explicit lifecycle observer.
    pub const fn with_observer(
        ownership: O,
        lease: L,
        participants: P,
        consensus: C,
        observer: E,
    ) -> Self {
        Self {
            ownership,
            lease,
            participants,
            consensus,
            observer,
        }
    }

    /// Returns the ownership service.
    #[must_use]
    pub const fn ownership(&self) -> &O {
        &self.ownership
    }

    /// Returns the lease service.
    #[must_use]
    pub const fn lease(&self) -> &L {
        &self.lease
    }

    /// Returns the participant service.
    #[must_use]
    pub const fn participants(&self) -> &P {
        &self.participants
    }

    /// Returns the consensus service.
    #[must_use]
    pub const fn consensus(&self) -> &C {
        &self.consensus
    }

    /// Returns the lifecycle observer.
    #[must_use]
    pub const fn observer(&self) -> &E {
        &self.observer
    }

    /// Releases every acquired distributed resource.
    ///
    /// All applicable release operations are attempted.
    ///
    /// If more than one cleanup operation fails, the first error is returned.
    /// Cleanup is never silently ignored.
    fn release_all(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
    ) -> Result<(), ResilienceError>
    where
        O: DistributedOwnershipService,
        L: DistributedLeaseService,
    {
        self.observer.observe(request, DistributedPhase::Release);

        let mut first_error: Option<ResilienceError> = None;

        // Release lease before ownership because the lease may be scoped to
        // the ownership generation.
        if let Some(lease) = lease {
            if let Err(error) = self.lease.release(request, lease) {
                first_error = Some(error);
            }
        }

        if let Some(ownership) = ownership {
            if let Err(error) = self.ownership.release(request, ownership) {
                if first_error.is_none() {
                    first_error = Some(error);
                }
            }
        }

        match first_error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    /// Produces the canonical error for a required ownership contract
    /// violation.
    fn missing_ownership_error(
        request: &CoordinationRequest,
    ) -> ResilienceError {
        ResilienceError::new(
            ResilienceErrorCode::ResourceOwnershipConflict,
            "required distributed ownership was not granted",
        )
        .with_operation(request.operation_id().as_str().to_owned())
    }

    /// Produces the canonical error for a required lease contract violation.
    fn missing_lease_error(
        request: &CoordinationRequest,
    ) -> ResilienceError {
        ResilienceError::new(
            ResilienceErrorCode::LeaseExpired,
            "required distributed lease was not granted",
        )
        .with_operation(request.operation_id().as_str().to_owned())
    }

    /// Produces a canonical invalid coordination-mode error.
    fn invalid_mode_error(
        request: &CoordinationRequest,
        mode: CoordinationMode,
    ) -> ResilienceError {
        ResilienceError::new(
            ResilienceErrorCode::InvalidConfiguration,
            format!(
                "distributed coordination mode '{}' is not supported by the selected backend",
                mode_name(mode)
            ),
        )
        .with_operation(request.operation_id().as_str().to_owned())
    }

    /// Validates the generation and fence relationship.
    ///
    /// A lease and ownership grant must represent the same authoritative
    /// generation/fence when both are present.
    fn validate_fence_consistency(
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
    ) -> Result<(), ResilienceError> {
        if let (Some(ownership), Some(lease)) = (ownership, lease) {
            if ownership.generation() != lease.generation() {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::ConcurrencyConflict,
                        "ownership and lease generations do not match",
                    )
                    .with_operation(
                        request.operation_id().as_str().to_owned(),
                    ),
                );
            }

            if ownership.fence() != lease.fence() {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::ConcurrencyConflict,
                        "ownership and lease fencing tokens do not match",
                    )
                    .with_operation(
                        request.operation_id().as_str().to_owned(),
                    ),
                );
            }
        }

        if let Some(expected) = request.expected_generation() {
            let actual = ownership
                .map(OwnershipGrant::generation)
                .or_else(|| lease.map(LeaseGrant::generation));

            if let Some(actual) = actual {
                if actual != expected {
                    return Err(
                        ResilienceError::new(
                            ResilienceErrorCode::ResourceStateChanged,
                            "distributed coordination generation does not match the request",
                        )
                        .with_operation(
                            request.operation_id().as_str().to_owned(),
                        ),
                    );
                }
            }
        }

        Ok(())
    }
}

impl<O, L, P, C, E> CoordinationBackend
    for DistributedCoordinationBackend<O, L, P, C, E>
where
    O: DistributedOwnershipService,
    L: DistributedLeaseService,
    P: DistributedParticipantService,
    C: DistributedConsensusService,
    E: DistributedObserver,
{
    fn validate(
        &self,
        request: &CoordinationRequest,
    ) -> Result<(), ResilienceError> {
        self.observer
            .observe(request, DistributedPhase::Validate);

        self.ownership.validate(request)?;
        self.lease.validate(request)?;
        self.participants.validate(request)?;
        self.consensus.validate(request)?;

        match request.mode() {
            CoordinationMode::Exclusive
            | CoordinationMode::Cooperative
            | CoordinationMode::Shared
            | CoordinationMode::Automatic => {}
        }

        Ok(())
    }

    fn acquire_ownership(
        &self,
        request: &CoordinationRequest,
    ) -> Result<Option<OwnershipGrant>, ResilienceError> {
        self.observer
            .observe(request, DistributedPhase::Ownership);

        self.ownership.acquire(request)
    }

    fn acquire_lease(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
    ) -> Result<Option<LeaseGrant>, ResilienceError> {
        self.observer.observe(request, DistributedPhase::Lease);

        let lease = self.lease.acquire(request, ownership)?;

        Self::validate_fence_consistency(
            request,
            ownership,
            lease.as_ref(),
        )?;

        Ok(lease)
    }

    fn prepare(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
    ) -> Result<PreparedParticipants, ResilienceError> {
        self.observer
            .observe(request, DistributedPhase::Prepare);

        Self::validate_fence_consistency(
            request,
            ownership,
            lease,
        )?;

        self.participants.prepare(
            request,
            ownership,
            lease,
        )
    }

    fn coordinate(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
        participants: &PreparedParticipants,
    ) -> Result<CoordinationDecision, ResilienceError> {
        self.observer
            .observe(request, DistributedPhase::Decide);

        Self::validate_fence_consistency(
            request,
            ownership,
            lease,
        )?;

        let decision = self.consensus.decide(
            request,
            ownership,
            lease,
            participants,
        )?;

        // A consensus service must not return a decision from a stale
        // generation.
        if let Some(expected) = request.expected_generation() {
            if decision.generation() != expected {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::ResourceStateChanged,
                        "consensus decision generation does not match the requested generation",
                    )
                    .with_operation(
                        request.operation_id().as_str().to_owned(),
                    ),
                );
            }
        }

        if let Some(ownership) = ownership {
            if decision.generation() != ownership.generation() {
                return Err(
                    ResilienceError::new(
                        ResilienceErrorCode::ConcurrencyConflict,
                        "consensus decision generation does not match ownership generation",
                    )
                    .with_operation(
                        request.operation_id().as_str().to_owned(),
                    ),
                );
            }
        }

        Ok(decision)
    }

    fn commit(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
        participants: &PreparedParticipants,
        decision: &CoordinationDecision,
    ) -> Result<(), ResilienceError> {
        Self::validate_fence_consistency(
            request,
            ownership,
            lease,
        )?;

        //
        // A non-committing decision is still durable coordination state.
        //
        // This matters for:
        //
        // Retryable
        // Escalated
        // Rejected
        //
        // Those decisions must not accidentally become participant mutations.
        //
        self.observer
            .observe(request, DistributedPhase::RecordDecision);

        self.consensus.record_decision(
            request,
            ownership,
            lease,
            decision,
        )?;

        if !decision.outcome().is_committed() {
            //
            // The decision is authoritative, but there is no participant
            // mutation to apply.
            //
            self.observer
                .observe(request, DistributedPhase::Finalize);

            return self.consensus.finalize(
                request,
                ownership,
                lease,
                participants,
                decision,
            );
        }

        //
        // The decision is durable before participant mutation.
        //
        // This creates an explicit recovery point:
        //
        //     durable decision
        //          |
        //          v
        //     participant application
        //
        // If the process crashes after the durable decision but before all
        // participants apply it, the consensus layer can recover/replay the
        // decision without inventing a quantum-state rollback.
        //
        self.observer
            .observe(request, DistributedPhase::Commit);

        self.participants.commit(
            request,
            ownership,
            lease,
            participants,
            decision,
        )?;

        self.observer
            .observe(request, DistributedPhase::Finalize);

        self.consensus.finalize(
            request,
            ownership,
            lease,
            participants,
            decision,
        )
    }

    fn release(
        &self,
        request: &CoordinationRequest,
        ownership: Option<&OwnershipGrant>,
        lease: Option<&LeaseGrant>,
    ) -> Result<(), ResilienceError> {
        self.release_all(
            request,
            ownership,
            lease,
        )
    }
}

// =============================================================================
// Validated backend integration
// =============================================================================

impl<O, L, P, C, E> ValidatedCoordinationBackend
    for DistributedCoordinationBackend<O, L, P, C, E>
where
    O: DistributedOwnershipService,
    L: DistributedLeaseService,
    P: DistributedParticipantService,
    C: DistributedConsensusService,
    E: DistributedObserver,
{
    fn validate_policy<Policy>(
        &self,
        request: &CoordinationRequest,
        policy: &Policy,
    ) -> Result<(), ResilienceError>
    where
        Policy: CoordinationPolicy,
    {
        let ownership_required =
            policy.requires_ownership(request)?;

        let lease_required =
            policy.requires_lease(request)?;

        let commit_required =
            policy.requires_commit(request)?;

        //
        // Validate the combination before any mutation occurs.
        //
        // We cannot determine whether the lower-level services will actually
        // grant ownership/lease without executing them, but we can ensure that
        // the protocol is structurally capable of providing them.
        //
        if !ownership_required && lease_required {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                    "a distributed policy cannot require a lease without allowing ownership",
                )
                .with_operation(
                    request.operation_id().as_str().to_owned(),
                ),
            );
        }

        //
        // Shared coordination can intentionally operate without an exclusive
        // commit boundary, but the decision service remains authoritative.
        //
        if !commit_required
            && matches!(request.mode(), CoordinationMode::Exclusive)
        {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::InvalidConfiguration,
                    "exclusive distributed coordination requires an explicit commit boundary",
                )
                .with_operation(
                    request.operation_id().as_str().to_owned(),
                ),
            );
        }

        Ok(())
    }
}

impl<O, L, P, C, E> RequiredLeaseError
    for DistributedCoordinationBackend<O, L, P, C, E>
where
    O: DistributedOwnershipService,
    L: DistributedLeaseService,
    P: DistributedParticipantService,
    C: DistributedConsensusService,
    E: DistributedObserver,
{
    fn missing_required_lease_error(&self) -> ResilienceError {
        ResilienceError::new(
            ResilienceErrorCode::LeaseExpired,
            "required distributed lease was not granted",
        )
    }
}

// =============================================================================
// Generic distributed protocol façade
// =============================================================================

/// Describes the immutable protocol capabilities of a distributed backend.
///
/// This is descriptive metadata only. It does not replace hardware or network
/// capability discovery.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DistributedProtocolCapabilities {
    /// Whether ownership fencing is supported.
    fencing: bool,

    /// Whether leases are supported.
    leases: bool,

    /// Whether durable decisions are supported.
    durable_decisions: bool,

    /// Whether participant commit is idempotent.
    idempotent_commit: bool,

    /// Whether deterministic participant ordering can be guaranteed.
    deterministic_ordering: bool,
}

impl DistributedProtocolCapabilities {
    /// Creates a capability description.
    #[must_use]
    pub const fn new(
        fencing: bool,
        leases: bool,
        durable_decisions: bool,
        idempotent_commit: bool,
        deterministic_ordering: bool,
    ) -> Self {
        Self {
            fencing,
            leases,
            durable_decisions,
            idempotent_commit,
            deterministic_ordering,
        }
    }

    /// Whether fencing is supported.
    #[must_use]
    pub const fn fencing(&self) -> bool {
        self.fencing
    }

    /// Whether leases are supported.
    #[must_use]
    pub const fn leases(&self) -> bool {
        self.leases
    }

    /// Whether durable decisions are supported.
    #[must_use]
    pub const fn durable_decisions(&self) -> bool {
        self.durable_decisions
    }

    /// Whether participant commit is idempotent.
    #[must_use]
    pub const fn idempotent_commit(&self) -> bool {
        self.idempotent_commit
    }

    /// Whether deterministic ordering is supported.
    #[must_use]
    pub const fn deterministic_ordering(&self) -> bool {
        self.deterministic_ordering
    }
}

// =============================================================================
// Protocol validation helpers
// =============================================================================

/// Validates that a distributed backend has the minimum protocol guarantees
/// required for safe exclusive coordination.
pub fn validate_exclusive_capabilities(
    capabilities: &DistributedProtocolCapabilities,
) -> Result<(), ResilienceError> {
    if !capabilities.fencing() {
        return Err(
            ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
                "exclusive distributed coordination requires fencing",
            ),
        );
    }

    if !capabilities.durable_decisions() {
        return Err(
            ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
                "exclusive distributed coordination requires durable decisions",
            ),
        );
    }

    if !capabilities.idempotent_commit() {
        return Err(
            ResilienceError::new(
                ResilienceErrorCode::CapabilityUnavailable,
                "exclusive distributed coordination requires idempotent participant commit",
            ),
        );
    }

    Ok(())
}

// =============================================================================
// Decision validation
// =============================================================================

/// Validates a distributed decision before participant application.
pub fn validate_decision(
    request: &CoordinationRequest,
    decision: &CoordinationDecision,
) -> Result<(), ResilienceError> {
    if let Some(expected) = request.expected_generation() {
        if decision.generation() != expected {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::ResourceStateChanged,
                    "distributed decision has a stale generation",
                )
                .with_operation(
                    request.operation_id().as_str().to_owned(),
                ),
            );
        }
    }

    Ok(())
}

// =============================================================================
// Participant-state validation
// =============================================================================

/// Validates the participant state required before a committing decision.
///
/// This deliberately does not require every participant to be `Ready`.
///
/// Some distributed architectures legitimately have participants initially
/// marked `Active` or `Degraded`.
///
/// The authoritative participant service decides whether those states can
/// participate.
pub fn validate_participant_snapshot(
    _request: &CoordinationRequest,
    participants: &PreparedParticipants,
) -> Result<(), ResilienceError> {
    for participant in participants.as_slice() {
        if matches!(
            participant.state(),
            ParticipantState::Failed
                | ParticipantState::Unavailable
                | ParticipantState::Quarantined
        ) {
            return Err(
                ResilienceError::new(
                    ResilienceErrorCode::ResourceUnavailable,
                    "a prepared distributed participant is unavailable for coordination",
                ),
            );
        }
    }

    Ok(())
}

// =============================================================================
// Mode helper
// =============================================================================

const fn mode_name(mode: CoordinationMode) -> &'static str {
    match mode {
        CoordinationMode::Exclusive => "exclusive",
        CoordinationMode::Cooperative => "cooperative",
        CoordinationMode::Shared => "shared",
        CoordinationMode::Automatic => "automatic",
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Error wrapper for distributed coordination integration.
///
/// This type is intentionally small and mainly useful for implementations
/// that need to preserve protocol-layer classification before converting to
/// `ResilienceError`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DistributedCoordinationError {
    /// A protocol invariant was violated.
    Invariant,

    /// The current generation is stale.
    StaleGeneration,

    /// A fencing token is invalid.
    InvalidFence,

    /// A durable decision could not be established.
    DurableDecisionUnavailable,

    /// Participant state is not suitable for the requested operation.
    ParticipantUnavailable,

    /// The protocol cannot support the requested coordination mode.
    UnsupportedMode,
}

impl DistributedCoordinationError {
    /// Converts this protocol error into the canonical resilience error.
    #[must_use]
    pub fn into_resilience_error(self) -> ResilienceError {
        match self {
            Self::Invariant => ResilienceError::new(
                ResilienceErrorCode::InvariantViolation,
                "distributed coordination protocol invariant violated",
            ),

            Self::StaleGeneration => ResilienceError::new(
                ResilienceErrorCode::ResourceStateChanged,
                "distributed coordination generation is stale",
            ),

            Self::InvalidFence => ResilienceError::new(
                ResilienceErrorCode::ConcurrencyConflict,
                "distributed coordination fence is invalid",
            ),

            Self::DurableDecisionUnavailable => ResilienceError::new(
                ResilienceErrorCode::CoordinationFailed,
                "durable distributed decision is unavailable",
            ),

            Self::ParticipantUnavailable => ResilienceError::new(
                ResilienceErrorCode::ResourceUnavailable,
                "distributed participant is unavailable",
            ),

            Self::UnsupportedMode => ResilienceError::new(
                ResilienceErrorCode::InvalidConfiguration,
                "distributed coordination mode is unsupported",
            ),
        }
    }
}

impl fmt::Display for DistributedCoordinationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invariant => {
                formatter.write_str("distributed coordination invariant violated")
            }
            Self::StaleGeneration => {
                formatter.write_str("distributed coordination generation is stale")
            }
            Self::InvalidFence => {
                formatter.write_str("distributed coordination fence is invalid")
            }
            Self::DurableDecisionUnavailable => {
                formatter.write_str("durable distributed decision is unavailable")
            }
            Self::ParticipantUnavailable => {
                formatter.write_str("distributed participant is unavailable")
            }
            Self::UnsupportedMode => {
                formatter.write_str("distributed coordination mode is unsupported")
            }
        }
    }
}

impl std::error::Error for DistributedCoordinationError {}

// =============================================================================
// Compile-time contract assertions
// =============================================================================
//
// These functions intentionally do not execute. Their purpose is to keep the
// integration surface explicit and compiler-checked as the repository evolves.

#[allow(dead_code)]
fn assert_coordination_contract<B, P>()
where
    B: CoordinationBackend,
    P: CoordinationPolicy,
{
}

/// Explicit integration constructor for the standard coordinator.
///
/// This does not hide policy or service construction.
pub fn coordinate_with_backend<B, P>(
    coordinator: &ResilienceCoordinator,
    backend: &B,
    policy: &P,
    request: &CoordinationRequest,
) -> Result<CoordinationResult, ResilienceError>
where
    B: CoordinationBackend,
    P: CoordinationPolicy,
{
    coordinator.coordinate(
        backend,
        policy,
        request,
    )
}

/// Explicit integration constructor for the fully validated coordinator
/// contract.
pub fn coordinate_with_validated_backend<B, P>(
    coordinator: &ResilienceCoordinator,
    backend: &B,
    policy: &P,
    request: &CoordinationRequest,
) -> Result<CoordinationResult, ResilienceError>
where
    B: ValidatedCoordinationBackend,
    P: CoordinationPolicy,
{
    coordinator.coordinate_validated(
        backend,
        policy,
        request,
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_is_stable() {
        assert_eq!(
            DISTRIBUTED_COORDINATION_SCHEMA_ID,
            "zamani.quantum.resilience.coordination.distributed"
        );

        assert_eq!(
            DISTRIBUTED_COORDINATION_SCHEMA_VERSION,
            1
        );
    }

    #[test]
    fn protocol_capabilities_allow_valid_configuration() {
        let capabilities =
            DistributedProtocolCapabilities::new(
                true,
                true,
                true,
                true,
                true,
            );

        assert!(
            validate_exclusive_capabilities(&capabilities).is_ok()
        );
    }

    #[test]
    fn protocol_capabilities_reject_missing_fencing() {
        let capabilities =
            DistributedProtocolCapabilities::new(
                false,
                true,
                true,
                true,
                true,
            );

        assert!(
            validate_exclusive_capabilities(&capabilities).is_err()
        );
    }

    #[test]
    fn protocol_capabilities_reject_non_durable_decisions() {
        let capabilities =
            DistributedProtocolCapabilities::new(
                true,
                true,
                false,
                true,
                true,
            );

        assert!(
            validate_exclusive_capabilities(&capabilities).is_err()
        );
    }

    #[test]
    fn protocol_capabilities_reject_non_idempotent_commit() {
        let capabilities =
            DistributedProtocolCapabilities::new(
                true,
                true,
                true,
                false,
                true,
            );

        assert!(
            validate_exclusive_capabilities(&capabilities).is_err()
        );
    }

    #[test]
    fn mode_names_are_stable() {
        assert_eq!(
            mode_name(CoordinationMode::Exclusive),
            "exclusive"
        );

        assert_eq!(
            mode_name(CoordinationMode::Cooperative),
            "cooperative"
        );

        assert_eq!(
            mode_name(CoordinationMode::Shared),
            "shared"
        );

        assert_eq!(
            mode_name(CoordinationMode::Automatic),
            "automatic"
        );
    }

    #[test]
    fn distributed_phases_are_machine_readable() {
        assert_eq!(
            DistributedPhase::Validate.as_str(),
            "validate"
        );

        assert_eq!(
            DistributedPhase::RecordDecision.as_str(),
            "record_decision"
        );

        assert_eq!(
            DistributedPhase::Commit.as_str(),
            "commit"
        );

        assert_eq!(
            DistributedPhase::Finalize.as_str(),
            "finalize"
        );
    }

    #[test]
    fn protocol_error_conversion_is_structured() {
        let error =
            DistributedCoordinationError::StaleGeneration
                .into_resilience_error();

        assert_eq!(
            error.code(),
            ResilienceErrorCode::ResourceStateChanged
        );
    }
}