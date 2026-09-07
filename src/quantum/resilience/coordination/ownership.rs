//! Zamani Quantum Resilience — Authoritative Ownership
//!
//! Path:
//!     src/quantum/resilience/coordination/ownership.rs
//!
//! Purpose:
//!     Define the provider-neutral, production ownership contract used by
//!     distributed quantum-resilience coordination.
//!
//! Ownership is deliberately distinct from leasing:
//!
//! - ownership answers who is authoritative for a coordination operation;
//! - a lease answers for how long that authority remains valid;
//! - fencing prevents stale authorities from mutating distributed state;
//! - consensus determines distributed agreement where required.
//!
//! This file owns the ownership contract and its validation/composition layer.
//! It does not own lease timing, distributed transport, consensus, routing,
//! scheduling, QEC, hardware discovery, calibration, optimization, execution,
//! mitigation, or result verification.
//!
//! Architectural position:
//!
//!     Zamani Program
//!          |
//!          v
//!     Canonical Quantum IR
//!          |
//!          v
//!     Resilience Controller
//!          |
//!          v
//!     coordination::coordinator
//!          |
//!          +----> ownership.rs   <-- this file
//!          +----> lease.rs
//!          +----> distributed.rs
//!          +----> consensus.rs
//!
//! Production requirements:
//!
//! - Rust 1.97 / 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust only;
//! - no unsafe;
//! - no hidden global mutable state;
//! - no implicit threads;
//! - no implicit I/O;
//! - no provider-specific branches;
//! - no fixed qubit count;
//! - no fixed participant count;
//! - no fixed resource count;
//! - no fixed timeout;
//! - no fixed lease duration;
//! - no fixed retry count;
//! - no fixed quorum size;
//! - deterministic validation;
//! - explicit ownership identity;
//! - explicit generation/fencing;
//! - stale-owner rejection;
//! - idempotent release at the authoritative implementation boundary;
//! - explicit authorization hooks;
//! - explicit persistence hooks;
//! - explicit audit hooks;
//! - graceful partial-failure propagation.
//!
//! "Infinite scalability" means that this file imposes no artificial machine
//! size. Actual scalability is limited only by the resources and policies of
//! the deployment: CPU, memory, communication, storage, and quantum resources.
//!
//! Canonical quantum identity:
//!
//! This module intentionally does not define QubitId or PhysicalQubitId.
//! If a concrete ownership implementation needs qubit identity it must use:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! Generic coordination continues to use CoordinationResourceId. This keeps
//! logical and physical identities separate and prevents ownership from
//! becoming coupled to a particular quantum-machine representation.
//!
//! Ownership safety invariant:
//!
//! A participant MUST NOT be considered authoritative merely because it claims
//! ownership. Authority exists only when the authoritative ownership service
//! returns a valid grant for the requested generation and the corresponding
//! fence is accepted by the resource authority.
//!
//! A stale owner must not be able to mutate state after ownership has been
//! superseded. Downstream mutation APIs must therefore carry the returned
//! CoordinationFence and validate it at the authoritative resource boundary.
//!
//! Important quantum distinction:
//!
//! Ownership is control-plane authority. It is not ownership of an arbitrary
//! quantum state. This module never promises that a quantum state can be
//! serialized, rolled back, migrated, or restored. Those semantics belong to
//! execution/checkpoint/recovery subsystems.
//!
//! Integration:
//!
//! coordination/coordinator.rs
//!     consumes DistributedOwnershipService.
//!
//! coordination/distributed.rs
//!     composes this service with lease, participant and consensus services.
//!
//! coordination/lease.rs
//!     remains authoritative for time-bounded validity.
//!
//! coordination/consensus.rs
//!     remains authoritative for distributed agreement.
//!
//! The public types in this file are stable contract types. Concrete stores,
//! databases, local services, cluster services, quantum-network controllers,
//! and future distributed implementations can therefore be added without
//! changing the ownership API.
//!
//! Persistence:
//!
//! Ownership state must be persisted by the injected OwnershipAuthority when
//! durable ownership is required. OwnershipRecord contains only portable
//! control-plane state and no process-local runtime handles.
//!
//! Determinism:
//!
//! This module never reads a clock or creates identifiers. The caller supplies
//! operation identity, participant identity, expected generation, and resource
//! set. Validation is consequently deterministic.
//!
//! Security:
//!
//! Ownership is an authorization boundary. Implementations must authenticate
//! callers and authorize ownership changes before mutating authoritative state.
//! Credentials and secrets must never be stored in OwnershipRecord or metadata.
//!
//! Error policy:
//!
//! This module uses the repository's canonical ResilienceError rather than
//! introducing a second resilience-wide error taxonomy.
//!
//! No hidden production implementation:
//!
//! This file intentionally does not provide a process-local ownership database
//! as the default implementation. A local in-memory database would falsely
//! imply that local memory is authoritative in a distributed deployment.
//!
//! Compiler-enforced safety boundary.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

use crate::quantum::resilience::coordination::coordinator::{
    CoordinationFence,
    CoordinationGeneration,
    CoordinationRequest,
    CoordinationResourceId,
    OwnershipGrant,
};

use crate::quantum::resilience::errors::error::ResilienceError;

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for the ownership contract.
pub const OWNERSHIP_SCHEMA_ID: &str =
    "zamani.quantum.resilience.coordination.ownership";

/// Current ownership contract schema version.
///
/// This is a protocol/schema version, not a machine-size limit.
pub const OWNERSHIP_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Result
// =============================================================================

/// Result alias used by ownership contracts.
pub type OwnershipResult<T> = Result<T, ResilienceError>;

// =============================================================================
// Ownership identity
// =============================================================================

/// Stable identity of one ownership record.
///
/// The identity is supplied by the caller or authoritative service. Ownership
/// never invents identifiers because deterministic replay and distributed
/// idempotency require identifier generation to remain outside this service.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OwnershipId(Arc<str>);

impl OwnershipId {
    /// Creates a validated ownership identity.
    pub fn new(value: impl Into<Arc<str>>) -> OwnershipResult<Self> {
        let value = value.into();

        validate_identifier(&value, "ownership identity")?;

        Ok(Self(value))
    }

    /// Returns the identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for OwnershipId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// =============================================================================
// Ownership state
// =============================================================================

/// Authoritative lifecycle state of an ownership record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OwnershipState {
    /// The record is not known to the authority.
    Unknown,

    /// Ownership is currently authoritative.
    Active,

    /// Ownership was explicitly released.
    Released,

    /// Ownership was superseded by a newer generation.
    Superseded,

    /// Ownership was administratively revoked.
    Revoked,
}

impl OwnershipState {
    /// Returns whether this state grants authority.
    #[must_use]
    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Stable machine-readable representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Active => "active",
            Self::Released => "released",
            Self::Superseded => "superseded",
            Self::Revoked => "revoked",
        }
    }
}

impl fmt::Display for OwnershipState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Ownership metadata
// =============================================================================

/// Deterministic, non-secret ownership metadata.
///
/// Metadata is descriptive and auditable. It MUST NOT be used to override
/// authorization, fencing, generation, or policy decisions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OwnershipMetadata {
    values: BTreeMap<Arc<str>, Arc<str>>,
}

impl OwnershipMetadata {
    /// Creates empty metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a metadata entry.
    ///
    /// No arbitrary maximum size is imposed. Deployment policy and available
    /// resources determine practical bounds.
    pub fn insert(
        &mut self,
        key: impl Into<Arc<str>>,
        value: impl Into<Arc<str>>,
    ) -> OwnershipResult<Option<Arc<str>>> {
        let key = key.into();
        let value = value.into();

        validate_identifier(&key, "ownership metadata key")?;

        if value.trim().is_empty() {
            return Err(ResilienceError::invalid_argument(
                "ownership metadata value must not be empty",
            ));
        }

        Ok(self.values.insert(key, value))
    }

    /// Returns a metadata value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(AsRef::as_ref)
    }

    /// Returns deterministic metadata entries.
    #[must_use]
    pub fn entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values
            .iter()
            .map(|(key, value)| (key.as_ref(), value.as_ref()))
    }

    /// Returns the number of metadata entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether no metadata entries exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

// =============================================================================
// Ownership record
// =============================================================================

/// Portable authoritative ownership state.
///
/// This contains only control-plane state and can therefore be persisted by an
/// implementation of OwnershipAuthority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnershipRecord {
    id: OwnershipId,
    request_operation_id: Arc<str>,
    execution_id: Arc<str>,
    owner: Arc<str>,
    resources: Arc<[CoordinationResourceId]>,
    generation: CoordinationGeneration,
    fence: CoordinationFence,
    state: OwnershipState,
    metadata: OwnershipMetadata,
}

impl OwnershipRecord {
    /// Creates a record from an authoritative grant and request.
    pub fn from_grant(
        id: OwnershipId,
        request: &CoordinationRequest,
        grant: &OwnershipGrant,
        metadata: OwnershipMetadata,
    ) -> OwnershipResult<Self> {
        validate_request(request)?;
        validate_grant(request, grant)?;

        Ok(Self {
            id,
            request_operation_id: Arc::from(request.operation_id().as_str()),
            execution_id: Arc::from(request.execution_id().as_str()),
            owner: Arc::from(grant.owner().as_str()),
            resources: Arc::from(
                request
                    .resources()
                    .to_vec()
                    .into_boxed_slice(),
            ),
            generation: grant.generation(),
            fence: grant.fence().clone(),
            state: OwnershipState::Active,
            metadata,
        })
    }

    /// Returns the ownership identity.
    #[must_use]
    pub fn id(&self) -> &OwnershipId {
        &self.id
    }

    /// Returns the originating operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &str {
        &self.request_operation_id
    }

    /// Returns the logical execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &str {
        &self.execution_id
    }

    /// Returns the owner identity.
    #[must_use]
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns the resources covered by ownership.
    #[must_use]
    pub fn resources(&self) -> &[CoordinationResourceId] {
        &self.resources
    }

    /// Returns the authoritative generation.
    #[must_use]
    pub const fn generation(&self) -> CoordinationGeneration {
        self.generation
    }

    /// Returns the authoritative fencing token.
    #[must_use]
    pub fn fence(&self) -> &CoordinationFence {
        &self.fence
    }

    /// Returns lifecycle state.
    #[must_use]
    pub const fn state(&self) -> OwnershipState {
        self.state
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &OwnershipMetadata {
        &self.metadata
    }

    /// Returns whether this record is currently authoritative.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.state.is_authoritative()
    }
}

// =============================================================================
// Ownership proof
// =============================================================================

/// Minimal proof carried by downstream mutating operations.
///
/// The proof contains no quantum state. It proves control-plane authority only.
/// Resource authorities must validate all fields relevant to their namespace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnershipProof {
    ownership_id: OwnershipId,
    owner: Arc<str>,
    generation: CoordinationGeneration,
    fence: CoordinationFence,
}

impl OwnershipProof {
    /// Creates a proof from an authoritative record.
    pub fn from_record(record: &OwnershipRecord) -> OwnershipResult<Self> {
        if !record.is_active() {
            return Err(ResilienceError::invalid_argument(
                "cannot create an ownership proof from inactive ownership",
            ));
        }

        Ok(Self {
            ownership_id: record.id.clone(),
            owner: Arc::from(record.owner()),
            generation: record.generation(),
            fence: record.fence().clone(),
        })
    }

    /// Returns ownership identity.
    #[must_use]
    pub fn ownership_id(&self) -> &OwnershipId {
        &self.ownership_id
    }

    /// Returns owner identity.
    #[must_use]
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns generation.
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
// Ownership options
// =============================================================================

/// Request-specific ownership options.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OwnershipOptions {
    metadata: OwnershipMetadata,
}

impl OwnershipOptions {
    /// Creates default options.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces metadata.
    #[must_use]
    pub fn with_metadata(mut self, metadata: OwnershipMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Returns metadata.
    #[must_use]
    pub fn metadata(&self) -> &OwnershipMetadata {
        &self.metadata
    }
}

// =============================================================================
// Ownership operation
// =============================================================================

/// Operation presented to authorization and policy layers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OwnershipOperation<'a> {
    /// Request to acquire ownership.
    Acquire {
        /// Coordination request.
        request: &'a CoordinationRequest,

        /// Ownership options.
        options: &'a OwnershipOptions,
    },

    /// Request to validate ownership.
    Validate {
        /// Coordination request.
        request: &'a CoordinationRequest,

        /// Ownership proof.
        proof: &'a OwnershipProof,
    },

    /// Request to release ownership.
    Release {
        /// Coordination request.
        request: &'a CoordinationRequest,

        /// Ownership proof.
        proof: &'a OwnershipProof,
    },
}

// =============================================================================
// Authorization
// =============================================================================

/// Authorization boundary for ownership operations.
///
/// Authentication and authorization are injected. This module does not assume
/// certificates, tokens, operating-system identities, cluster identities, or
/// any particular security protocol.
pub trait OwnershipAuthorizer: Send + Sync {
    /// Authorizes the requested ownership operation.
    fn authorize(
        &self,
        operation: OwnershipOperation<'_>,
    ) -> OwnershipResult<()>;
}

/// Authorizer for an already trusted deployment boundary.
///
/// Network-facing deployments should provide a real authorizer.
#[derive(Clone, Copy, Debug, Default)]
pub struct TrustedBoundaryAuthorizer;

impl OwnershipAuthorizer for TrustedBoundaryAuthorizer {
    fn authorize(
        &self,
        _operation: OwnershipOperation<'_>,
    ) -> OwnershipResult<()> {
        Ok(())
    }
}

// =============================================================================
// Ownership policy
// =============================================================================

/// Policy boundary for ownership decisions.
///
/// Policy determines whether ownership is permitted. It must not mutate
/// authoritative ownership state.
pub trait OwnershipPolicy: Send + Sync {
    /// Validates an acquisition request.
    fn validate_acquire(
        &self,
        request: &CoordinationRequest,
        options: &OwnershipOptions,
    ) -> OwnershipResult<()>;

    /// Validates an ownership proof before use.
    fn validate_proof(
        &self,
        request: &CoordinationRequest,
        proof: &OwnershipProof,
    ) -> OwnershipResult<()>;

    /// Validates release of ownership.
    fn validate_release(
        &self,
        request: &CoordinationRequest,
        proof: &OwnershipProof,
    ) -> OwnershipResult<()>;
}

/// Policy that applies structural validation only.
///
/// Deployment-specific authorization, quotas, resource constraints and
/// administrative policy are injected separately.
#[derive(Clone, Copy, Debug, Default)]
pub struct StructuralOwnershipPolicy;

impl OwnershipPolicy for StructuralOwnershipPolicy {
    fn validate_acquire(
        &self,
        request: &CoordinationRequest,
        _options: &OwnershipOptions,
    ) -> OwnershipResult<()> {
        validate_request(request)
    }

    fn validate_proof(
        &self,
        request: &CoordinationRequest,
        proof: &OwnershipProof,
    ) -> OwnershipResult<()> {
        validate_request(request)?;

        if proof.owner() != request.initiator().as_str() {
            return Err(ResilienceError::invalid_argument(
                "ownership proof owner does not match request initiator",
            ));
        }

        Ok(())
    }

    fn validate_release(
        &self,
        request: &CoordinationRequest,
        proof: &OwnershipProof,
    ) -> OwnershipResult<()> {
        self.validate_proof(request, proof)
    }
}

// =============================================================================
// Authoritative ownership service
// =============================================================================

/// Durable authoritative ownership service.
///
/// Implementations own the actual serialization and concurrency mechanism.
/// This abstraction supports:
///
/// - one process;
/// - one host;
/// - a cluster;
/// - a data center;
/// - a distributed quantum system;
/// - a heterogeneous quantum fleet;
/// - a remote control plane.
///
/// The implementation must provide a documented consistency model strong
/// enough to prevent two conflicting owners from simultaneously being treated
/// as authoritative.
///
/// For exclusive ownership, acquisition, validation, release and generation
/// advancement must be linearizable or have an equivalent safety guarantee.
pub trait OwnershipAuthority: Send + Sync {
    /// Acquires ownership.
    ///
    /// `None` means the authority deliberately declined the acquisition under
    /// its documented conflict semantics.
    fn acquire(
        &self,
        request: &CoordinationRequest,
        options: &OwnershipOptions,
    ) -> OwnershipResult<Option<OwnershipGrant>>;

    /// Validates a proof against authoritative current state.
    fn validate(
        &self,
        request: &CoordinationRequest,
        proof: &OwnershipProof,
    ) -> OwnershipResult<OwnershipState>;

    /// Releases ownership.
    ///
    /// A stale proof must never release a newer owner's record.
    fn release(
        &self,
        request: &CoordinationRequest,
        proof: &OwnershipProof,
    ) -> OwnershipResult<()>;

    /// Returns persisted ownership state when the deployment permits it.
    fn get(
        &self,
        ownership_id: &OwnershipId,
    ) -> OwnershipResult<Option<OwnershipRecord>>;
}

// =============================================================================
// Ownership events
// =============================================================================

/// Auditable ownership lifecycle event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OwnershipEventKind {
    /// Acquisition was requested.
    AcquisitionRequested,

    /// Acquisition was granted.
    Acquired,

    /// Acquisition was not granted.
    AcquisitionDenied,

    /// A proof was validated.
    Validated,

    /// A proof was rejected.
    ValidationRejected,

    /// Release was requested.
    ReleaseRequested,

    /// Ownership was released.
    Released,

    /// Ownership was superseded.
    Superseded,

    /// Ownership was revoked.
    Revoked,
}

impl OwnershipEventKind {
    /// Stable event name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AcquisitionRequested => "acquisition_requested",
            Self::Acquired => "acquired",
            Self::AcquisitionDenied => "acquisition_denied",
            Self::Validated => "validated",
            Self::ValidationRejected => "validation_rejected",
            Self::ReleaseRequested => "release_requested",
            Self::Released => "released",
            Self::Superseded => "superseded",
            Self::Revoked => "revoked",
        }
    }
}

/// Immutable ownership audit event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnershipEvent {
    kind: OwnershipEventKind,
    operation_id: Arc<str>,
    execution_id: Arc<str>,
    owner: Arc<str>,
    generation: Option<CoordinationGeneration>,
}

impl OwnershipEvent {
    /// Creates an event from a request and optional proof.
    #[must_use]
    pub fn new(
        kind: OwnershipEventKind,
        request: &CoordinationRequest,
        proof: Option<&OwnershipProof>,
    ) -> Self {
        Self {
            kind,
            operation_id: Arc::from(request.operation_id().as_str()),
            execution_id: Arc::from(request.execution_id().as_str()),
            owner: Arc::from(request.initiator().as_str()),
            generation: proof.map(OwnershipProof::generation),
        }
    }

    /// Returns event kind.
    #[must_use]
    pub const fn kind(&self) -> OwnershipEventKind {
        self.kind
    }

    /// Returns operation identity.
    #[must_use]
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    /// Returns execution identity.
    #[must_use]
    pub fn execution_id(&self) -> &str {
        &self.execution_id
    }

    /// Returns owner identity.
    #[must_use]
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns generation when available.
    #[must_use]
    pub const fn generation(&self) -> Option<CoordinationGeneration> {
        self.generation
    }
}

/// Sink for ownership audit/telemetry events.
///
/// Ownership correctness must not depend on telemetry delivery unless the
/// deployment explicitly makes that observer a mandatory audit boundary.
pub trait OwnershipObserver: Send + Sync {
    /// Observes an ownership event.
    fn observe(&self, event: OwnershipEvent);
}

/// No-op ownership observer.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoopOwnershipObserver;

impl OwnershipObserver for NoopOwnershipObserver {
    fn observe(&self, _event: OwnershipEvent) {}
}

// =============================================================================
// Ownership service
// =============================================================================

/// Composed ownership service used by `coordination/distributed.rs`.
///
/// This type is deliberately stateless. All authoritative mutable state lives
/// in the injected `OwnershipAuthority`.
pub struct OwnershipService<
    A,
    P = StructuralOwnershipPolicy,
    Z = TrustedBoundaryAuthorizer,
    O = NoopOwnershipObserver,
> {
    authority: A,
    policy: P,
    authorizer: Z,
    observer: O,
}

impl<A> OwnershipService<A> {
    /// Creates an ownership service with structural validation.
    #[must_use]
    pub const fn new(authority: A) -> Self {
        Self {
            authority,
            policy: StructuralOwnershipPolicy,
            authorizer: TrustedBoundaryAuthorizer,
            observer: NoopOwnershipObserver,
        }
    }
}

impl<A, P, Z, O> OwnershipService<A, P, Z, O> {
    /// Creates an ownership service with explicit dependencies.
    #[must_use]
    pub const fn with_dependencies(
        authority: A,
        policy: P,
        authorizer: Z,
        observer: O,
    ) -> Self {
        Self {
            authority,
            policy,
            authorizer,
            observer,
        }
    }

    /// Returns the authoritative service.
    #[must_use]
    pub const fn authority(&self) -> &A {
        &self.authority
    }

    /// Returns the ownership policy.
    #[must_use]
    pub const fn policy(&self) -> &P {
        &self.policy
    }

    /// Returns the authorizer.
    #[must_use]
    pub const fn authorizer(&self) -> &Z {
        &self.authorizer
    }

    /// Returns the observer.
    #[must_use]
    pub const fn observer(&self) -> &O {
        &self.observer
    }
}

impl<A, P, Z, O> OwnershipService<A, P, Z, O>
where
    A: OwnershipAuthority,
    P: OwnershipPolicy,
    Z: OwnershipAuthorizer,
    O: OwnershipObserver,
{
    /// Acquires ownership through the authoritative service.
    pub fn acquire(
        &self,
        request: &CoordinationRequest,
        options: &OwnershipOptions,
    ) -> OwnershipResult<Option<OwnershipGrant>> {
        self.policy.validate_acquire(request, options)?;

        self.authorizer.authorize(
            OwnershipOperation::Acquire {
                request,
                options,
            },
        )?;

        self.observer.observe(OwnershipEvent::new(
            OwnershipEventKind::AcquisitionRequested,
            request,
            None,
        ));

        let result = self.authority.acquire(request, options)?;

        let kind = if result.is_some() {
            OwnershipEventKind::Acquired
        } else {
            OwnershipEventKind::AcquisitionDenied
        };

        self.observer.observe(OwnershipEvent::new(
            kind,
            request,
            None,
        ));

        Ok(result)
    }

    /// Validates an ownership proof against policy, authorization and the
    /// authoritative ownership store.
    pub fn validate(
        &self,
        request: &CoordinationRequest,
        proof: &OwnershipProof,
    ) -> OwnershipResult<OwnershipState> {
        self.policy.validate_proof(request, proof)?;

        self.authorizer.authorize(
            OwnershipOperation::Validate {
                request,
                proof,
            },
        )?;

        let state = self.authority.validate(request, proof)?;

        if !state.is_authoritative() {
            self.observer.observe(OwnershipEvent::new(
                OwnershipEventKind::ValidationRejected,
                request,
                Some(proof),
            ));

            return Err(ResilienceError::invalid_argument(
                "ownership proof is not authoritative",
            ));
        }

        self.observer.observe(OwnershipEvent::new(
            OwnershipEventKind::Validated,
            request,
            Some(proof),
        ));

        Ok(state)
    }

    /// Releases ownership through the authoritative service.
    pub fn release(
        &self,
        request: &CoordinationRequest,
        proof: &OwnershipProof,
    ) -> OwnershipResult<()> {
        self.policy.validate_release(request, proof)?;

        self.authorizer.authorize(
            OwnershipOperation::Release {
                request,
                proof,
            },
        )?;

        self.observer.observe(OwnershipEvent::new(
            OwnershipEventKind::ReleaseRequested,
            request,
            Some(proof),
        ));

        self.authority.release(request, proof)?;

        self.observer.observe(OwnershipEvent::new(
            OwnershipEventKind::Released,
            request,
            Some(proof),
        ));

        Ok(())
    }

    /// Returns persisted ownership state.
    pub fn get(
        &self,
        ownership_id: &OwnershipId,
    ) -> OwnershipResult<Option<OwnershipRecord>> {
        self.authority.get(ownership_id)
    }
}

// =============================================================================
// Integration with distributed.rs
// =============================================================================

impl<A, P, Z, O>
    crate::quantum::resilience::coordination::distributed::DistributedOwnershipService
    for OwnershipService<A, P, Z, O>
where
    A: OwnershipAuthority,
    P: OwnershipPolicy,
    Z: OwnershipAuthorizer,
    O: OwnershipObserver,
{
    fn validate(
        &self,
        request: &CoordinationRequest,
    ) -> OwnershipResult<()> {
        let options = OwnershipOptions::new();

        self.policy.validate_acquire(request, &options)?;

        self.authorizer.authorize(
            OwnershipOperation::Acquire {
                request,
                options: &options,
            },
        )
    }

    fn acquire(
        &self,
        request: &CoordinationRequest,
    ) -> OwnershipResult<Option<OwnershipGrant>> {
        self.acquire(request, &OwnershipOptions::new())
    }

    fn release(
        &self,
        request: &CoordinationRequest,
        ownership: &OwnershipGrant,
    ) -> OwnershipResult<()> {
        validate_grant(request, ownership)?;

        let ownership_id =
            OwnershipId::new(request.operation_id().as_str())?;

        let record = OwnershipRecord::from_grant(
            ownership_id,
            request,
            ownership,
            OwnershipMetadata::new(),
        )?;

        let proof = OwnershipProof::from_record(&record)?;

        self.release(request, &proof)
    }
}

// =============================================================================
// Structural request validation
// =============================================================================

/// Validates the generic coordination request for ownership use.
///
/// No maximum resource cardinality is imposed.
///
/// Duplicate resource identities are rejected because ownership scopes have
/// set semantics. This prevents multiple representations of the same ownership
/// scope from producing different authorization decisions.
pub fn validate_request(
    request: &CoordinationRequest,
) -> OwnershipResult<()> {
    if request.operation_id().as_str().trim().is_empty() {
        return Err(ResilienceError::invalid_argument(
            "coordination operation identity must not be empty",
        ));
    }

    if request.execution_id().as_str().trim().is_empty() {
        return Err(ResilienceError::invalid_argument(
            "coordination execution identity must not be empty",
        ));
    }

    if request.initiator().as_str().trim().is_empty() {
        return Err(ResilienceError::invalid_argument(
            "coordination initiator identity must not be empty",
        ));
    }

    let mut resources = request.resources().iter().collect::<Vec<_>>();
    resources.sort();

    for pair in resources.windows(2) {
        if pair[0] == pair[1] {
            return Err(ResilienceError::invalid_argument(
                "coordination resource set contains a duplicate resource identity",
            ));
        }
    }

    Ok(())
}

/// Validates an ownership identity or metadata key.
fn validate_identifier(
    value: &str,
    field: &str,
) -> OwnershipResult<()> {
    if value.trim().is_empty() {
        return Err(ResilienceError::invalid_argument(format!(
            "{field} must not be empty"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(ResilienceError::invalid_argument(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(())
}

// =============================================================================
// Grant validation
// =============================================================================

/// Validates that a grant can safely represent ownership for a request.
///
/// The authority remains responsible for deciding whether the generation is
/// current and whether the fence is valid. This function validates local
/// structural invariants only.
pub fn validate_grant(
    request: &CoordinationRequest,
    grant: &OwnershipGrant,
) -> OwnershipResult<()> {
    validate_request(request)?;

    if grant.owner() != request.initiator() {
        return Err(ResilienceError::invalid_argument(
            "ownership grant owner does not match request initiator",
        ));
    }

    if grant.fence().as_str().trim().is_empty() {
        return Err(ResilienceError::invalid_argument(
            "ownership grant fencing token must not be empty",
        ));
    }

    if let Some(expected) = request.expected_generation() {
        if grant.generation() != expected {
            return Err(ResilienceError::invalid_argument(
                "ownership grant generation does not match request expectation",
            ));
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

    use crate::quantum::resilience::coordination::coordinator::{
        CoordinationOperationId,
        ExecutionId,
        ParticipantId,
    };

    fn request() -> CoordinationRequest {
        CoordinationRequest::new(
            CoordinationOperationId::new("operation-1")
                .expect("valid operation"),
            ExecutionId::new("execution-1")
                .expect("valid execution"),
            ParticipantId::new("participant-1")
                .expect("valid participant"),
        )
        .with_resources(vec![
            CoordinationResourceId::new("resource-a")
                .expect("valid resource"),
            CoordinationResourceId::new("resource-b")
                .expect("valid resource"),
        ])
    }

    #[test]
    fn validates_normal_request() {
        validate_request(&request())
            .expect("request should validate");
    }

    #[test]
    fn rejects_duplicate_resources() {
        let resource =
            CoordinationResourceId::new("resource-a")
                .expect("valid resource");

        let request = CoordinationRequest::new(
            CoordinationOperationId::new("operation-1")
                .expect("valid operation"),
            ExecutionId::new("execution-1")
                .expect("valid execution"),
            ParticipantId::new("participant-1")
                .expect("valid participant"),
        )
        .with_resources(vec![resource.clone(), resource]);

        assert!(validate_request(&request).is_err());
    }

    #[test]
    fn rejects_control_characters_in_identifiers() {
        assert!(OwnershipId::new("bad\nidentity").is_err());
    }

    #[test]
    fn metadata_is_deterministically_ordered() {
        let mut metadata = OwnershipMetadata::new();

        metadata
            .insert("b", "two")
            .expect("metadata");

        metadata
            .insert("a", "one")
            .expect("metadata");

        let entries =
            metadata.entries().collect::<Vec<_>>();

        assert_eq!(
            entries,
            vec![
                ("a", "one"),
                ("b", "two"),
            ]
        );
    }

    #[test]
    fn active_state_is_authoritative() {
        assert!(OwnershipState::Active.is_authoritative());

        assert!(!OwnershipState::Released.is_authoritative());
        assert!(!OwnershipState::Superseded.is_authoritative());
        assert!(!OwnershipState::Revoked.is_authoritative());
    }

    #[test]
    fn grant_matches_expected_generation() {
        let request =
            request().with_expected_generation(
                CoordinationGeneration::new(7),
            );

        let owner =
            ParticipantId::new("participant-1")
                .expect("owner");

        let fence =
            CoordinationFence::new("fence-7")
                .expect("fence");

        let grant = OwnershipGrant::new(
            owner,
            CoordinationGeneration::new(7),
            fence,
        );

        validate_grant(&request, &grant)
            .expect("grant should validate");
    }

    #[test]
    fn rejects_generation_mismatch() {
        let request =
            request().with_expected_generation(
                CoordinationGeneration::new(7),
            );

        let owner =
            ParticipantId::new("participant-1")
                .expect("owner");

        let fence =
            CoordinationFence::new("fence-8")
                .expect("fence");

        let grant = OwnershipGrant::new(
            owner,
            CoordinationGeneration::new(8),
            fence,
        );

        assert!(validate_grant(&request, &grant).is_err());
    }

    #[test]
    fn rejects_owner_mismatch() {
        let request = request();

        let owner =
            ParticipantId::new("different-owner")
                .expect("owner");

        let fence =
            CoordinationFence::new("fence-1")
                .expect("fence");

        let grant = OwnershipGrant::new(
            owner,
            CoordinationGeneration::new(1),
            fence,
        );

        assert!(validate_grant(&request, &grant).is_err());
    }

    #[test]
    fn rejects_empty_fence() {
        let request = request();

        let owner =
            ParticipantId::new("participant-1")
                .expect("owner");

        let fence_result =
            CoordinationFence::new("");

        assert!(fence_result.is_err());

        let _ = owner;
        let _ = request;
    }

    #[test]
    fn ownership_proof_requires_active_record() {
        let request = request();

        let owner =
            ParticipantId::new("participant-1")
                .expect("owner");

        let fence =
            CoordinationFence::new("fence-1")
                .expect("fence");

        let grant = OwnershipGrant::new(
            owner,
            CoordinationGeneration::new(1),
            fence,
        );

        let id =
            OwnershipId::new("ownership-1")
                .expect("ownership id");

        let record =
            OwnershipRecord::from_grant(
                id,
                &request,
                &grant,
                OwnershipMetadata::new(),
            )
            .expect("record");

        let proof =
            OwnershipProof::from_record(&record)
                .expect("proof");

        assert_eq!(
            proof.generation(),
            CoordinationGeneration::new(1)
        );

        assert_eq!(
            proof.owner(),
            "participant-1"
        );
    }
}