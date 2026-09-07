//! Zamani Quantum Resilience — Distributed Lease Contract
//!
//! This module defines the provider-neutral lease abstraction used by
//! `quantum::resilience::coordination`.
//!
//! # Purpose
//!
//! A lease gives one coordination participant temporary authority over a
//! resource or coordination scope while providing a monotonically advancing
//! fencing token.
//!
//! The fencing token is the critical safety mechanism:
//!
//! ```text
//! coordinator A
//!      |
//!      | acquire lease
//!      v
//! token = 41
//!
//! coordinator A stalls
//!
//! coordinator B
//!      |
//!      | acquire/recover lease
//!      v
//! token = 42
//!
//! coordinator A resumes
//!      |
//!      | attempts mutation with token 41
//!      v
//! REJECT
//! ```
//!
//! A lease is therefore NOT merely a timeout. It is an authorization epoch
//! combined with a fencing value.
//!
//! # Architectural responsibility
//!
//! This file owns:
//!
//! - lease identity;
//! - lease owner identity;
//! - lease scope;
//! - lease epoch;
//! - fencing token;
//! - lease duration semantics;
//! - acquisition requests;
//! - renewal requests;
//! - release requests;
//! - lease state;
//! - lease validation contracts;
//! - lease service trait;
//! - lease error classification helpers;
//! - deterministic lease-domain representations.
//!
//! This file does NOT own:
//!
//! - distributed consensus;
//! - leader election;
//! - quorum calculation;
//! - network transport;
//! - resource discovery;
//! - hardware allocation;
//! - routing;
//! - scheduling;
//! - quantum execution;
//! - QEC;
//! - recovery planning;
//! - recovery execution;
//! - checkpoint storage;
//! - persistence implementation;
//! - a particular distributed database;
//! - a particular cloud/provider;
//! - a particular clock synchronization protocol.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! coordination/distributed.rs
//! coordination/ownership.rs
//! coordination/consensus.rs
//! coordination/coordinator.rs
//! ```
//!
//! # Architectural position
//!
//! ```text
//!                     Resilience Coordinator
//!                              |
//!                              v
//!                    +-------------------+
//!                    |   Coordination    |
//!                    +---------+---------+
//!                              |
//!                +-------------+-------------+
//!                |                           |
//!                v                           v
//!          OwnershipService             LeaseService
//!                                                |
//!                                                v
//!                                      fencing token
//!                                                |
//!                                                v
//!                                      distributed resource
//! ```
//!
//! # Write once, scale everywhere
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_RESOURCES
//! MAX_QUBITS
//! MAX_LEASES
//! DEFAULT_TTL
//! MAX_RENEWALS
//! RETRY_COUNT
//! ```
//!
//! A lease may refer to:
//!
//! - a single logical qubit;
//! - a physical qubit;
//! - a QPU;
//! - a backend;
//! - an execution slot;
//! - a control channel;
//! - a distributed quantum resource;
//! - a cluster;
//! - a region;
//! - an arbitrary resource represented by the canonical resilience resource
//!   model.
//!
//! Resource capacity is determined by the resource provider and policy rather
//! than this module.
//!
//! # Quantum identity
//!
//! This module does not create a new qubit identifier.
//!
//! When a lease is scoped to a quantum resource, callers should use the
//! canonical resource identity from:
//!
//! ```text
//! quantum::resilience::model::resource::ResourceIdentity
//! ```
//!
//! which in turn uses the canonical IR identities:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Therefore this file does not duplicate those types.
//!
//! # Safety invariant
//!
//! A caller MUST NOT treat possession of an old lease object as sufficient
//! authorization for a mutating operation.
//!
//! Every mutating distributed operation must carry the current fencing token.
//!
//! The resource owner/service is responsible for rejecting stale fencing
//! tokens.
//!
//! # Expiration invariant
//!
//! A lease expiry is not itself proof that an old process has stopped.
//!
//! Therefore lease expiration MUST be combined with fencing.
//!
//! ```text
//! expiration + fencing
//! ```
//!
//! rather than:
//!
//! ```text
//! expiration only
//! ```
//!
//! # Renewal invariant
//!
//! Renewal MUST be conditional on:
//!
//! - the lease identity;
//! - the current owner;
//! - the current epoch;
//! - the current fencing token;
//! - service-side validity.
//!
//! A stale owner must never be able to renew a lease after authority has
//! moved elsewhere.
//!
//! # Persistence
//!
//! This file intentionally does not serialize `Instant` or assume a global
//! wall clock.
//!
//! A distributed lease implementation must use an authoritative clock or a
//! clock contract appropriate to its coordination system.
//!
//! # Error boundary
//!
//! Public service operations return the repository's canonical:
//!
//! ```text
//! quantum::resilience::errors::ResilienceError
//! ```
//!
//! Lease-specific failures use the existing resilience error taxonomy,
//! especially:
//!
//! ```text
//! ResourceOwnershipConflict
//! LeaseExpired
//! ConcurrencyConflict
//! SynchronizationTimeout
//! CoordinationFailed
//! InvalidArgument
//! InvalidIdentifier
//! InvalidState
//! DeadlineExceeded
//! ```
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! -----------------------------------------------------------------------------
//! Compiler-enforced safety boundary
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

// =============================================================================
// Imports
// =============================================================================

use std::fmt;
use std::num::NonZeroU64;
use std::sync::Arc;
use std::time::Duration;

use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
};
use crate::quantum::resilience::model::resource::ResourceIdentity;

// =============================================================================
// Schema constants
// =============================================================================

/// Stable schema identifier for the lease contract.
pub const LEASE_SCHEMA_ID: &str = "zamani.quantum.resilience.coordination.lease";

/// Semantic version of the lease contract.
pub const LEASE_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Lease ID
// =============================================================================

/// Globally meaningful identifier of a lease.
///
/// The identifier is opaque to this module. A distributed implementation may
/// derive it from an operation ID, resource identity, coordinator identity,
/// random entropy, or another scheme appropriate to its persistence model.
///
/// The lease ID must never be interpreted as a hardware identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LeaseId(Arc<str>);

impl LeaseId {
    /// Creates a lease identifier.
    ///
    /// Empty and whitespace-only identifiers are rejected.
    pub fn new(value: impl Into<String>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(invalid_identifier());
        }

        Ok(Self(Arc::<str>::from(value)))
    }

    /// Returns the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LeaseId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Lease owner
// =============================================================================

/// Identity of the process, runtime, coordinator, or service currently
/// holding lease authority.
///
/// This is deliberately opaque.
///
/// It must not be interpreted as:
///
/// - a user identity;
/// - a backend identity;
/// - a hardware identity;
/// - a network address.
///
/// Authentication and authorization are separate concerns.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LeaseOwnerId(Arc<str>);

impl LeaseOwnerId {
    /// Creates an owner identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(invalid_identifier());
        }

        Ok(Self(Arc::<str>::from(value)))
    }

    /// Returns the owner identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LeaseOwnerId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Coordination epoch
// =============================================================================

/// Monotonically increasing coordination epoch.
///
/// An epoch separates different generations of coordination authority.
///
/// Example:
///
/// ```text
/// epoch 10
///   coordinator A
///
/// coordinator A fails
///
/// epoch 11
///   coordinator B
/// ```
///
/// Old epoch operations must be rejected by implementations that use epoch
/// fencing.
///
/// The value is a protocol counter, not a resource-size limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LeaseEpoch(NonZeroU64);

impl LeaseEpoch {
    /// Creates an epoch from a positive integer.
    pub const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    /// Creates an epoch from a raw integer.
    ///
    /// Zero is rejected because epoch zero represents "no established
    /// coordination generation".
    pub fn try_from_u64(value: u64) -> Result<Self, ResilienceError> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or_else(|| invalid_argument())
    }

    /// Returns the numeric epoch.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }

    /// Returns the next epoch, if representable.
    pub fn next(self) -> Result<Self, ResilienceError> {
        let next = self
            .get()
            .checked_add(1)
            .ok_or_else(|| arithmetic_overflow())?;

        Self::try_from_u64(next)
    }
}

impl fmt::Display for LeaseEpoch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

// =============================================================================
// Fencing token
// =============================================================================

/// Monotonically increasing fencing token.
///
/// A fencing token is stronger than a lease expiry check.
///
/// Every mutation that can affect coordinated state should carry the fencing
/// token that was issued with the current lease.
///
/// A resource service must reject a token that is older than its current
/// accepted token.
///
/// Example:
///
/// ```text
/// lease A -> token 100
/// lease A expires
///
/// lease B -> token 101
///
/// mutation(token=100) -> reject
/// mutation(token=101) -> accept
/// ```
///
/// The token is deliberately independent of:
///
/// - qubit count;
/// - machine size;
/// - retry count;
/// - resource count.
///
/// It is a coordination protocol value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FencingToken(NonZeroU64);

impl FencingToken {
    /// Creates a fencing token from a known positive value.
    pub const fn new(value: NonZeroU64) -> Self {
        Self(value)
    }

    /// Creates a fencing token from a raw value.
    pub fn try_from_u64(value: u64) -> Result<Self, ResilienceError> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or_else(|| invalid_argument())
    }

    /// Returns the numeric token.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }

    /// Returns the next fencing token.
    pub fn next(self) -> Result<Self, ResilienceError> {
        let next = self
            .get()
            .checked_add(1)
            .ok_or_else(|| arithmetic_overflow())?;

        Self::try_from_u64(next)
    }
}

impl fmt::Display for FencingToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

// =============================================================================
// Lease sequence
// =============================================================================

/// Monotonically increasing lease operation sequence.
///
/// This allows implementations to distinguish:
///
/// ```text
/// acquire
/// renew #1
/// renew #2
/// release
/// ```
///
/// without requiring the lease implementation to expose internal database
/// versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LeaseSequence(NonZeroU64);

impl LeaseSequence {
    /// Creates the initial sequence.
    pub const fn initial() -> Self {
        Self(NonZeroU64::MIN)
    }

    /// Creates a sequence from a positive integer.
    pub fn try_from_u64(value: u64) -> Result<Self, ResilienceError> {
        NonZeroU64::new(value)
            .map(Self)
            .ok_or_else(|| invalid_argument())
    }

    /// Returns the numeric sequence.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }

    /// Returns the next sequence.
    pub fn next(self) -> Result<Self, ResilienceError> {
        let next = self
            .get()
            .checked_add(1)
            .ok_or_else(|| arithmetic_overflow())?;

        Self::try_from_u64(next)
    }
}

impl Default for LeaseSequence {
    fn default() -> Self {
        Self::initial()
    }
}

// =============================================================================
// Lease scope
// =============================================================================

/// Scope of a lease.
///
/// The resource identity remains owned by the canonical resilience resource
/// model. This enum describes the semantic scope at which coordination is
/// taking place.
///
/// This allows the same lease contract to operate from a single physical
/// qubit to an arbitrarily large execution fabric.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LeaseScope {
    /// Lease a single canonical resource.
    Resource(ResourceIdentity),

    /// Lease a logical coordination domain.
    ///
    /// The identifier is intentionally opaque because a logical domain can
    /// represent a circuit, logical block, execution graph, or another
    /// application-defined coordination domain.
    LogicalDomain(Arc<str>),

    /// Lease a physical execution domain.
    ///
    /// This can represent a device, QPU, control domain, or another
    /// implementation-defined physical scope.
    PhysicalDomain(Arc<str>),

    /// Lease a generic distributed execution domain.
    DistributedDomain(Arc<str>),
}

impl LeaseScope {
    /// Creates a logical-domain scope.
    pub fn logical_domain(
        value: impl Into<String>,
    ) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(invalid_identifier());
        }

        Ok(Self::LogicalDomain(Arc::<str>::from(value)))
    }

    /// Creates a physical-domain scope.
    pub fn physical_domain(
        value: impl Into<String>,
    ) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(invalid_identifier());
        }

        Ok(Self::PhysicalDomain(Arc::<str>::from(value)))
    }

    /// Creates a distributed-domain scope.
    pub fn distributed_domain(
        value: impl Into<String>,
    ) -> Result<Self, ResilienceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(invalid_identifier());
        }

        Ok(Self::DistributedDomain(Arc::<str>::from(value)))
    }

    /// Returns the semantic category of the scope.
    #[must_use]
    pub const fn kind(&self) -> LeaseScopeKind {
        match self {
            Self::Resource(_) => LeaseScopeKind::Resource,
            Self::LogicalDomain(_) => LeaseScopeKind::LogicalDomain,
            Self::PhysicalDomain(_) => LeaseScopeKind::PhysicalDomain,
            Self::DistributedDomain(_) => LeaseScopeKind::DistributedDomain,
        }
    }
}

/// Kind of lease scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LeaseScopeKind {
    /// Canonical resource scope.
    Resource,

    /// Logical computation scope.
    LogicalDomain,

    /// Physical execution scope.
    PhysicalDomain,

    /// Distributed execution scope.
    DistributedDomain,
}

// =============================================================================
// Lease duration
// =============================================================================

/// Requested lease duration.
///
/// The value is intentionally supplied by policy or the lease implementation.
///
/// This module does not define a default TTL.
///
/// A zero duration is rejected because a lease with no validity interval
/// cannot provide useful authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LeaseDuration(Duration);

impl LeaseDuration {
    /// Creates a lease duration.
    pub fn new(duration: Duration) -> Result<Self, ResilienceError> {
        if duration.is_zero() {
            return Err(invalid_argument());
        }

        Ok(Self(duration))
    }

    /// Returns the duration.
    #[must_use]
    pub const fn get(self) -> Duration {
        self.0
    }

    /// Returns the duration in nanoseconds.
    ///
    /// This method is provided for serialization/adapters and does not imply
    /// that a distributed implementation must use nanoseconds internally.
    #[must_use]
    pub fn as_nanos(self) -> u128 {
        self.0.as_nanos()
    }
}

// =============================================================================
// Lease state
// =============================================================================

/// State of a lease.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LeaseState {
    /// Lease has not yet been acquired.
    Pending,

    /// Lease is currently valid.
    Active,

    /// Lease is currently being renewed.
    Renewing,

    /// Lease expired.
    Expired,

    /// Lease was explicitly released.
    Released,

    /// Lease was revoked by the authority.
    Revoked,

    /// Lease was superseded by newer coordination authority.
    Superseded,
}

impl LeaseState {
    /// Returns whether this state represents currently usable authority.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active | Self::Renewing)
    }

    /// Returns whether this is a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Expired
                | Self::Released
                | Self::Revoked
                | Self::Superseded
        )
    }
}

// =============================================================================
// Lease acquisition mode
// =============================================================================

/// How an acquisition request should behave when a conflicting lease exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LeaseAcquisitionMode {
    /// Acquire only when no conflicting valid lease exists.
    Exclusive,

    /// Reuse the caller's existing lease when the lease identity and owner
    /// match.
    Idempotent,

    /// Request transfer only when the implementation's ownership policy
    /// permits it.
    TransferIfAuthorized,
}

// =============================================================================
// Lease grant
// =============================================================================

/// Authoritative result of a successful lease acquisition or renewal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseGrant {
    /// Stable lease identity.
    lease_id: LeaseId,

    /// Resource/scope controlled by the lease.
    scope: LeaseScope,

    /// Current lease owner.
    owner: LeaseOwnerId,

    /// Current coordination epoch.
    epoch: LeaseEpoch,

    /// Current fencing token.
    fencing_token: FencingToken,

    /// Monotonic operation sequence.
    sequence: LeaseSequence,

    /// Duration granted by the lease service.
    duration: LeaseDuration,

    /// Current lease state.
    state: LeaseState,
}

impl LeaseGrant {
    /// Creates a lease grant.
    ///
    /// This constructor is public so an external lease implementation can
    /// construct the canonical result after authoritative acquisition.
    pub fn new(
        lease_id: LeaseId,
        scope: LeaseScope,
        owner: LeaseOwnerId,
        epoch: LeaseEpoch,
        fencing_token: FencingToken,
        sequence: LeaseSequence,
        duration: LeaseDuration,
        state: LeaseState,
    ) -> Result<Self, ResilienceError> {
        if !state.is_active() {
            return Err(invalid_state());
        }

        Ok(Self {
            lease_id,
            scope,
            owner,
            epoch,
            fencing_token,
            sequence,
            duration,
            state,
        })
    }

    /// Returns the lease identity.
    #[must_use]
    pub fn lease_id(&self) -> &LeaseId {
        &self.lease_id
    }

    /// Returns the scope.
    #[must_use]
    pub fn scope(&self) -> &LeaseScope {
        &self.scope
    }

    /// Returns the owner.
    #[must_use]
    pub fn owner(&self) -> &LeaseOwnerId {
        &self.owner
    }

    /// Returns the coordination epoch.
    #[must_use]
    pub const fn epoch(&self) -> LeaseEpoch {
        self.epoch
    }

    /// Returns the fencing token.
    #[must_use]
    pub const fn fencing_token(&self) -> FencingToken {
        self.fencing_token
    }

    /// Returns the operation sequence.
    #[must_use]
    pub const fn sequence(&self) -> LeaseSequence {
        self.sequence
    }

    /// Returns the granted duration.
    #[must_use]
    pub const fn duration(&self) -> LeaseDuration {
        self.duration
    }

    /// Returns the state.
    #[must_use]
    pub const fn state(&self) -> LeaseState {
        self.state
    }

    /// Returns whether the grant represents active authority.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// Returns a compact authorization proof for a mutating operation.
    ///
    /// Callers should pass this proof through to the actual resource service.
    #[must_use]
    pub fn proof(&self) -> LeaseProof {
        LeaseProof {
            lease_id: self.lease_id.clone(),
            owner: self.owner.clone(),
            epoch: self.epoch,
            fencing_token: self.fencing_token,
            sequence: self.sequence,
        }
    }
}

// =============================================================================
// Lease proof
// =============================================================================

/// Authorization/fencing proof attached to a coordinated mutation.
///
/// This is intentionally separate from `LeaseGrant`.
///
/// A caller should not need to pass the entire lease object through every
/// resource operation.
///
/// The proof contains exactly the coordination information required to reject
/// stale authority.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LeaseProof {
    lease_id: LeaseId,
    owner: LeaseOwnerId,
    epoch: LeaseEpoch,
    fencing_token: FencingToken,
    sequence: LeaseSequence,
}

impl LeaseProof {
    /// Creates a proof.
    pub fn new(
        lease_id: LeaseId,
        owner: LeaseOwnerId,
        epoch: LeaseEpoch,
        fencing_token: FencingToken,
        sequence: LeaseSequence,
    ) -> Self {
        Self {
            lease_id,
            owner,
            epoch,
            fencing_token,
            sequence,
        }
    }

    /// Returns the lease identity.
    #[must_use]
    pub fn lease_id(&self) -> &LeaseId {
        &self.lease_id
    }

    /// Returns the owner.
    #[must_use]
    pub fn owner(&self) -> &LeaseOwnerId {
        &self.owner
    }

    /// Returns the epoch.
    #[must_use]
    pub const fn epoch(&self) -> LeaseEpoch {
        self.epoch
    }

    /// Returns the fencing token.
    #[must_use]
    pub const fn fencing_token(&self) -> FencingToken {
        self.fencing_token
    }

    /// Returns the sequence.
    #[must_use]
    pub const fn sequence(&self) -> LeaseSequence {
        self.sequence
    }
}

// =============================================================================
// Lease acquisition request
// =============================================================================

/// Request to acquire a lease.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseAcquireRequest {
    /// Caller-selected or coordinator-generated lease identity.
    lease_id: LeaseId,

    /// Requested resource/scope.
    scope: LeaseScope,

    /// Desired owner.
    owner: LeaseOwnerId,

    /// Coordination epoch under which acquisition is requested.
    epoch: LeaseEpoch,

    /// Requested duration.
    duration: LeaseDuration,

    /// Acquisition behavior.
    mode: LeaseAcquisitionMode,
}

impl LeaseAcquireRequest {
    /// Creates an acquisition request.
    pub fn new(
        lease_id: LeaseId,
        scope: LeaseScope,
        owner: LeaseOwnerId,
        epoch: LeaseEpoch,
        duration: LeaseDuration,
        mode: LeaseAcquisitionMode,
    ) -> Result<Self, ResilienceError> {
        Ok(Self {
            lease_id,
            scope,
            owner,
            epoch,
            duration,
            mode,
        })
    }

    /// Returns the lease identity.
    #[must_use]
    pub fn lease_id(&self) -> &LeaseId {
        &self.lease_id
    }

    /// Returns the requested scope.
    #[must_use]
    pub fn scope(&self) -> &LeaseScope {
        &self.scope
    }

    /// Returns the owner.
    #[must_use]
    pub fn owner(&self) -> &LeaseOwnerId {
        &self.owner
    }

    /// Returns the requested epoch.
    #[must_use]
    pub const fn epoch(&self) -> LeaseEpoch {
        self.epoch
    }

    /// Returns the requested duration.
    #[must_use]
    pub const fn duration(&self) -> LeaseDuration {
        self.duration
    }

    /// Returns the acquisition mode.
    #[must_use]
    pub const fn mode(&self) -> LeaseAcquisitionMode {
        self.mode
    }
}

// =============================================================================
// Lease renewal request
// =============================================================================

/// Request to renew an existing lease.
///
/// Renewal is explicitly fenced by the current proof.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseRenewRequest {
    /// Existing lease identity.
    lease_id: LeaseId,

    /// Current authorization proof.
    proof: LeaseProof,

    /// Requested additional duration.
    duration: LeaseDuration,
}

impl LeaseRenewRequest {
    /// Creates a renewal request.
    pub fn new(
        lease_id: LeaseId,
        proof: LeaseProof,
        duration: LeaseDuration,
    ) -> Result<Self, ResilienceError> {
        if lease_id != *proof.lease_id() {
            return Err(invalid_argument());
        }

        Ok(Self {
            lease_id,
            proof,
            duration,
        })
    }

    /// Returns the lease identity.
    #[must_use]
    pub fn lease_id(&self) -> &LeaseId {
        &self.lease_id
    }

    /// Returns the authorization proof.
    #[must_use]
    pub fn proof(&self) -> &LeaseProof {
        &self.proof
    }

    /// Returns the requested duration.
    #[must_use]
    pub const fn duration(&self) -> LeaseDuration {
        self.duration
    }
}

// =============================================================================
// Lease release request
// =============================================================================

/// Request to release a lease.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseReleaseRequest {
    /// Lease identity.
    lease_id: LeaseId,

    /// Current authorization proof.
    proof: LeaseProof,
}

impl LeaseReleaseRequest {
    /// Creates a release request.
    pub fn new(
        lease_id: LeaseId,
        proof: LeaseProof,
    ) -> Result<Self, ResilienceError> {
        if lease_id != *proof.lease_id() {
            return Err(invalid_argument());
        }

        Ok(Self { lease_id, proof })
    }

    /// Returns the lease identity.
    #[must_use]
    pub fn lease_id(&self) -> &LeaseId {
        &self.lease_id
    }

    /// Returns the authorization proof.
    #[must_use]
    pub fn proof(&self) -> &LeaseProof {
        &self.proof
    }
}

// =============================================================================
// Lease validation request
// =============================================================================

/// Request to validate a lease proof before a mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseValidationRequest {
    /// Lease identity.
    lease_id: LeaseId,

    /// Fencing proof.
    proof: LeaseProof,

    /// Expected scope.
    scope: LeaseScope,

    /// Expected coordination epoch.
    epoch: LeaseEpoch,
}

impl LeaseValidationRequest {
    /// Creates a validation request.
    pub fn new(
        lease_id: LeaseId,
        proof: LeaseProof,
        scope: LeaseScope,
        epoch: LeaseEpoch,
    ) -> Result<Self, ResilienceError> {
        if lease_id != *proof.lease_id() {
            return Err(invalid_argument());
        }

        if proof.epoch() != epoch {
            return Err(stale_epoch());
        }

        Ok(Self {
            lease_id,
            proof,
            scope,
            epoch,
        })
    }

    /// Returns the lease ID.
    #[must_use]
    pub fn lease_id(&self) -> &LeaseId {
        &self.lease_id
    }

    /// Returns the proof.
    #[must_use]
    pub fn proof(&self) -> &LeaseProof {
        &self.proof
    }

    /// Returns the expected scope.
    #[must_use]
    pub fn scope(&self) -> &LeaseScope {
        &self.scope
    }

    /// Returns the expected epoch.
    #[must_use]
    pub const fn epoch(&self) -> LeaseEpoch {
        self.epoch
    }
}

// =============================================================================
// Lease validation result
// =============================================================================

/// Result of lease-proof validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseValidation {
    /// Whether the proof is currently valid.
    valid: bool,

    /// Current authoritative fencing token.
    current_fencing_token: Option<FencingToken>,

    /// Current authoritative epoch.
    current_epoch: Option<LeaseEpoch>,

    /// Current authoritative lease state.
    state: Option<LeaseState>,
}

impl LeaseValidation {
    /// Creates a successful validation result.
    #[must_use]
    pub fn valid(
        fencing_token: FencingToken,
        epoch: LeaseEpoch,
        state: LeaseState,
    ) -> Self {
        Self {
            valid: true,
            current_fencing_token: Some(fencing_token),
            current_epoch: Some(epoch),
            state: Some(state),
        }
    }

    /// Creates an invalid validation result.
    #[must_use]
    pub const fn invalid() -> Self {
        Self {
            valid: false,
            current_fencing_token: None,
            current_epoch: None,
            state: None,
        }
    }

    /// Returns whether the proof is valid.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.valid
    }

    /// Returns the current fencing token when known.
    #[must_use]
    pub const fn current_fencing_token(&self) -> Option<FencingToken> {
        self.current_fencing_token
    }

    /// Returns the current epoch when known.
    #[must_use]
    pub const fn current_epoch(&self) -> Option<LeaseEpoch> {
        self.current_epoch
    }

    /// Returns the current lease state when known.
    #[must_use]
    pub const fn state(&self) -> Option<LeaseState> {
        self.state
    }
}

// =============================================================================
// Lease service trait
// =============================================================================

/// Provider-neutral lease service.
///
/// `distributed.rs` should compose this contract with transport and consensus
/// mechanisms.
///
/// An implementation may be:
///
/// - local/in-process;
/// - database-backed;
/// - cluster-backed;
/// - consensus-backed;
/// - embedded in a quantum runtime;
/// - remote.
///
/// The resilience coordinator must not need to know which.
///
/// # Atomicity requirements
///
/// Implementations MUST make acquisition, renewal, release, and fencing
/// validation atomic with respect to competing lease operations.
///
/// A service that cannot provide the required atomicity must reject the
/// operation rather than silently weakening the safety model.
pub trait LeaseService: Send + Sync {
    /// Acquires a lease.
    ///
    /// A successful result MUST contain a fencing token that is newer than
    /// any token previously accepted for the same coordination authority.
    fn acquire(
        &self,
        request: &LeaseAcquireRequest,
    ) -> Result<LeaseGrant, ResilienceError>;

    /// Renews a lease.
    ///
    /// Renewal MUST reject stale owner/epoch/token combinations.
    fn renew(
        &self,
        request: &LeaseRenewRequest,
    ) -> Result<LeaseGrant, ResilienceError>;

    /// Releases a lease.
    ///
    /// Release must be idempotent when the request refers to the same
    /// already-released lease and valid proof, unless policy explicitly
    /// requires another behavior.
    fn release(
        &self,
        request: &LeaseReleaseRequest,
    ) -> Result<(), ResilienceError>;

    /// Validates a proof before a mutating operation.
    ///
    /// This operation is advisory unless the actual mutation is atomically
    /// coupled to the validation by the underlying resource service.
    fn validate(
        &self,
        request: &LeaseValidationRequest,
    ) -> Result<LeaseValidation, ResilienceError>;

    /// Returns the current lease state when known.
    fn state(
        &self,
        lease_id: &LeaseId,
    ) -> Result<Option<LeaseState>, ResilienceError>;
}

// =============================================================================
// Lease mutation contract
// =============================================================================

/// Generic contract for operations that mutate a fenced resource.
///
/// Resource-specific systems may implement this trait or use `LeaseProof` as
/// part of their own command structure.
///
/// The important invariant is that the fencing proof accompanies the mutation.
pub trait FencedOperation {
    /// Error type used by the resource operation.
    type Error;

    /// Returns the lease proof attached to the operation.
    fn lease_proof(&self) -> &LeaseProof;

    /// Executes the mutation under the fencing proof.
    fn execute(self) -> Result<(), Self::Error>;
}

// =============================================================================
// Lease policy
// =============================================================================

/// Policy contract for lease-related decisions.
///
/// This deliberately does not contain fixed defaults.
///
/// A policy can derive duration, renewal behavior, transfer permission, and
/// release behavior from:
///
/// - resilience policy;
/// - execution deadline;
/// - resource characteristics;
/// - distributed topology;
/// - security requirements;
/// - workload requirements.
pub trait LeasePolicy: Send + Sync {
    /// Returns the requested lease duration.
    fn duration(
        &self,
        scope: &LeaseScope,
    ) -> Result<LeaseDuration, ResilienceError>;

    /// Returns whether a lease may be transferred.
    fn allow_transfer(
        &self,
        scope: &LeaseScope,
    ) -> Result<bool, ResilienceError>;

    /// Returns whether an expired lease may be reacquired.
    fn allow_reacquisition(
        &self,
        scope: &LeaseScope,
    ) -> Result<bool, ResilienceError>;

    /// Returns whether a renewal should be attempted.
    fn should_renew(
        &self,
        lease: &LeaseGrant,
    ) -> Result<bool, ResilienceError>;
}

// =============================================================================
// Lease authority
// =============================================================================

/// Authoritative lease authority contract.
///
/// This is useful for `distributed.rs` implementations where the distinction
/// between a local lease service and the actual authority needs to be explicit.
///
/// The authority is responsible for:
///
/// - assigning epochs;
/// - issuing fencing tokens;
/// - resolving conflicting ownership;
/// - ensuring stale tokens cannot mutate protected state.
pub trait LeaseAuthority: Send + Sync {
    /// Returns the current epoch for the coordination domain.
    fn current_epoch(&self) -> Result<LeaseEpoch, ResilienceError>;

    /// Allocates a new fencing token for a scope.
    ///
    /// The allocation MUST be monotonically increasing with respect to the
    /// authority governing that scope.
    fn allocate_fencing_token(
        &self,
        scope: &LeaseScope,
        epoch: LeaseEpoch,
    ) -> Result<FencingToken, ResilienceError>;

    /// Validates that an epoch remains authoritative.
    fn validate_epoch(
        &self,
        epoch: LeaseEpoch,
    ) -> Result<(), ResilienceError>;

    /// Validates a fencing token against the current authority.
    fn validate_fencing_token(
        &self,
        scope: &LeaseScope,
        epoch: LeaseEpoch,
        token: FencingToken,
    ) -> Result<(), ResilienceError>;
}

// =============================================================================
// Lease transition
// =============================================================================

/// Valid lifecycle transition for a lease.
///
/// This is useful to persistence and distributed implementations that need to
/// validate state transitions before committing them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LeaseTransition {
    /// Pending -> Active.
    Acquire,

    /// Active -> Renewing.
    BeginRenewal,

    /// Renewing -> Active.
    CompleteRenewal,

    /// Active -> Released.
    Release,

    /// Active -> Expired.
    Expire,

    /// Active -> Revoked.
    Revoke,

    /// Active -> Superseded.
    Supersede,
}

impl LeaseTransition {
    /// Validates whether a transition is legal.
    #[must_use]
    pub const fn is_valid(
        self,
        from: LeaseState,
        to: LeaseState,
    ) -> bool {
        match self {
            Self::Acquire => {
                matches!(
                    (from, to),
                    (LeaseState::Pending, LeaseState::Active)
                )
            }

            Self::BeginRenewal => {
                matches!(
                    (from, to),
                    (LeaseState::Active, LeaseState::Renewing)
                )
            }

            Self::CompleteRenewal => {
                matches!(
                    (from, to),
                    (LeaseState::Renewing, LeaseState::Active)
                )
            }

            Self::Release => {
                matches!(
                    (from, to),
                    (LeaseState::Active, LeaseState::Released)
                        | (LeaseState::Renewing, LeaseState::Released)
                )
            }

            Self::Expire => {
                matches!(
                    (from, to),
                    (LeaseState::Active, LeaseState::Expired)
                        | (LeaseState::Renewing, LeaseState::Expired)
                )
            }

            Self::Revoke => {
                matches!(
                    (from, to),
                    (LeaseState::Active, LeaseState::Revoked)
                        | (LeaseState::Renewing, LeaseState::Revoked)
                )
            }

            Self::Supersede => {
                matches!(
                    (from, to),
                    (LeaseState::Active, LeaseState::Superseded)
                        | (LeaseState::Renewing, LeaseState::Superseded)
                )
            }
        }
    }

    /// Validates a transition and returns a resilience error when invalid.
    pub fn validate(
        self,
        from: LeaseState,
        to: LeaseState,
    ) -> Result<(), ResilienceError> {
        if Self::is_valid(self, from, to) {
            Ok(())
        } else {
            Err(invalid_state())
        }
    }
}

// =============================================================================
// Lease conflict
// =============================================================================

/// Reason a lease request conflicts with existing authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LeaseConflictReason {
    /// Another owner currently has authority.
    DifferentOwner,

    /// The request belongs to an older coordination epoch.
    StaleEpoch,

    /// The presented fencing token is stale.
    StaleFencingToken,

    /// A newer lease superseded this lease.
    Superseded,

    /// Resource state changed concurrently.
    ResourceChanged,

    /// The lease has expired.
    Expired,
}

// =============================================================================
// Lease inspection
// =============================================================================

/// Read-only information about a lease.
///
/// This type deliberately contains no mutable service handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaseInspection {
    /// Lease identity.
    lease_id: LeaseId,

    /// Scope.
    scope: LeaseScope,

    /// Current owner.
    owner: LeaseOwnerId,

    /// Current epoch.
    epoch: LeaseEpoch,

    /// Current fencing token.
    fencing_token: FencingToken,

    /// Current sequence.
    sequence: LeaseSequence,

    /// Current state.
    state: LeaseState,

    /// Last granted duration.
    duration: LeaseDuration,
}

impl LeaseInspection {
    /// Creates an inspection result.
    #[must_use]
    pub const fn new(
        lease_id: LeaseId,
        scope: LeaseScope,
        owner: LeaseOwnerId,
        epoch: LeaseEpoch,
        fencing_token: FencingToken,
        sequence: LeaseSequence,
        state: LeaseState,
        duration: LeaseDuration,
    ) -> Self {
        Self {
            lease_id,
            scope,
            owner,
            epoch,
            fencing_token,
            sequence,
            state,
            duration,
        }
    }

    /// Returns the lease identity.
    #[must_use]
    pub fn lease_id(&self) -> &LeaseId {
        &self.lease_id
    }

    /// Returns the scope.
    #[must_use]
    pub fn scope(&self) -> &LeaseScope {
        &self.scope
    }

    /// Returns the owner.
    #[must_use]
    pub fn owner(&self) -> &LeaseOwnerId {
        &self.owner
    }

    /// Returns the epoch.
    #[must_use]
    pub const fn epoch(&self) -> LeaseEpoch {
        self.epoch
    }

    /// Returns the fencing token.
    #[must_use]
    pub const fn fencing_token(&self) -> FencingToken {
        self.fencing_token
    }

    /// Returns the sequence.
    #[must_use]
    pub const fn sequence(&self) -> LeaseSequence {
        self.sequence
    }

    /// Returns the state.
    #[must_use]
    pub const fn state(&self) -> LeaseState {
        self.state
    }

    /// Returns the duration(&self) -> LeaseDuration {
        self.duration
    }

    /// Returns whether the lease is currently active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.state.is_active()
    }
}

// =============================================================================
// Lease manager
// =============================================================================

/// High-level lease manager used by the resilience coordinator.
///
/// This is intentionally a thin orchestration layer.
///
/// It does not maintain a local lease database and therefore does not create
/// an unbounded in-memory cache that could become a scalability bottleneck.
///
/// Durable state belongs to the injected `LeaseService`.
pub struct LeaseManager<S> {
    service: Arc<S>,
}

impl<S> LeaseManager<S>
where
    S: LeaseService + 'static,
{
    /// Creates a lease manager around an authoritative service.
    #[must_use]
    pub fn new(service: Arc<S>) -> Self {
        Self { service }
    }

    /// Returns the underlying lease service.
    #[must_use]
    pub fn service(&self) -> &Arc<S> {
        &self.service
    }

    /// Acquires a lease through the authoritative service.
    pub fn acquire(
        &self,
        request: &LeaseAcquireRequest,
    ) -> Result<LeaseGrant, ResilienceError> {
        validate_acquire_request(request)?;

        self.service.acquire(request)
    }

    /// Renews a lease.
    pub fn renew(
        &self,
        request: &LeaseRenewRequest,
    ) -> Result<LeaseGrant, ResilienceError> {
        validate_renew_request(request)?;

        self.service.renew(request)
    }

    /// Releases a lease.
    pub fn release(
        &self,
        request: &LeaseReleaseRequest,
    ) -> Result<(), ResilienceError> {
        validate_release_request(request)?;

        self.service.release(request)
    }

    /// Validates a lease proof.
    pub fn validate(
        &self,
        request: &LeaseValidationRequest,
    ) -> Result<LeaseValidation, ResilienceError> {
        validate_validation_request(request)?;

        self.service.validate(request)
    }

    /// Validates a proof and returns an error if it is not currently usable.
    ///
    /// This helper is useful immediately before a mutation.
    pub fn require_valid(
        &self,
        request: &LeaseValidationRequest,
    ) -> Result<LeaseValidation, ResilienceError> {
        let validation = self.validate(request)?;

        if !validation.is_valid() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::LeaseExpired,
            ));
        }

        Ok(validation)
    }

    /// Reads current lease state.
    pub fn state(
        &self,
        lease_id: &LeaseId,
    ) -> Result<Option<LeaseState>, ResilienceError> {
        self.service.state(lease_id)
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a lease acquisition request.
pub fn validate_acquire_request(
    request: &LeaseAcquireRequest,
) -> Result<(), ResilienceError> {
    if request.lease_id().as_str().trim().is_empty() {
        return Err(invalid_identifier());
    }

    if request.owner().as_str().trim().is_empty() {
        return Err(invalid_identifier());
    }

    if request.duration().get().is_zero() {
        return Err(invalid_argument());
    }

    if request.epoch().get() == 0 {
        return Err(invalid_argument());
    }

    Ok(())
}

/// Validates a renewal request.
pub fn validate_renew_request(
    request: &LeaseRenewRequest,
) -> Result<(), ResilienceError> {
    if request.lease_id().as_str().trim().is_empty() {
        return Err(invalid_identifier());
    }

    if request.proof().lease_id() != request.lease_id() {
        return Err(invalid_argument());
    }

    if request.duration().get().is_zero() {
        return Err(invalid_argument());
    }

    Ok(())
}

/// Validates a release request.
pub fn validate_release_request(
    request: &LeaseReleaseRequest,
) -> Result<(), ResilienceError> {
    if request.lease_id().as_str().trim().is_empty() {
        return Err(invalid_identifier());
    }

    if request.proof().lease_id() != request.lease_id() {
        return Err(invalid_argument());
    }

    Ok(())
}

/// Validates a proof validation request.
pub fn validate_validation_request(
    request: &LeaseValidationRequest,
) -> Result<(), ResilienceError> {
    if request.lease_id().as_str().trim().is_empty() {
        return Err(invalid_identifier());
    }

    if request.proof().lease_id() != request.lease_id() {
        return Err(invalid_argument());
    }

    if request.proof().epoch() != request.epoch() {
        return Err(stale_epoch());
    }

    Ok(())
}

// =============================================================================
// Error helpers
// =============================================================================

fn invalid_argument() -> ResilienceError {
    ResilienceError::new(ResilienceErrorCode::InvalidArgument)
}

fn invalid_identifier() -> ResilienceError {
    ResilienceError::new(ResilienceErrorCode::InvalidIdentifier)
}

fn invalid_state() -> ResilienceError {
    ResilienceError::new(ResilienceErrorCode::InvalidState)
}

fn arithmetic_overflow() -> ResilienceError {
    ResilienceError::new(ResilienceErrorCode::ArithmeticOverflow)
}

fn stale_epoch() -> ResilienceError {
    ResilienceError::new(ResilienceErrorCode::ConcurrencyConflict)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lease_duration_rejects_zero() {
        let result = LeaseDuration::new(Duration::ZERO);

        assert!(result.is_err());
    }

    #[test]
    fn lease_duration_accepts_positive_duration() {
        let duration = LeaseDuration::new(Duration::from_secs(1))
            .expect("positive duration must be accepted");

        assert_eq!(duration.get(), Duration::from_secs(1));
    }

    #[test]
    fn epoch_rejects_zero() {
        assert!(LeaseEpoch::try_from_u64(0).is_err());
    }

    #[test]
    fn fencing_token_rejects_zero() {
        assert!(FencingToken::try_from_u64(0).is_err());
    }

    #[test]
    fn sequence_starts_at_one() {
        assert_eq!(LeaseSequence::initial().get(), 1);
    }

    #[test]
    fn sequence_advances_monotonically() {
        let first = LeaseSequence::initial();

        let second = first
            .next()
            .expect("sequence should advance");

        assert!(second > first);
        assert_eq!(second.get(), first.get() + 1);
    }

    #[test]
    fn epoch_advances_monotonically() {
        let epoch = LeaseEpoch::try_from_u64(1)
            .expect("epoch 1 must be valid");

        let next = epoch
            .next()
            .expect("epoch should advance");

        assert!(next > epoch);
        assert_eq!(next.get(), 2);
    }

    #[test]
    fn fencing_token_advances_monotonically() {
        let token = FencingToken::try_from_u64(41)
            .expect("token must be valid");

        let next = token
            .next()
            .expect("token should advance");

        assert!(next > token);
        assert_eq!(next.get(), 42);
    }

    #[test]
    fn empty_lease_id_is_rejected() {
        assert!(LeaseId::new("").is_err());
        assert!(LeaseId::new("   ").is_err());
    }

    #[test]
    fn empty_owner_id_is_rejected() {
        assert!(LeaseOwnerId::new("").is_err());
        assert!(LeaseOwnerId::new("   ").is_err());
    }

    #[test]
    fn scope_rejects_empty_domain() {
        assert!(LeaseScope::logical_domain("").is_err());
        assert!(LeaseScope::physical_domain(" ").is_err());
        assert!(LeaseScope::distributed_domain("").is_err());
    }

    #[test]
    fn lease_scope_kind_is_correct() {
        let logical = LeaseScope::logical_domain("logical-domain")
            .expect("scope should be valid");

        assert_eq!(
            logical.kind(),
            LeaseScopeKind::LogicalDomain
        );
    }

    #[test]
    fn acquisition_request_is_validated() {
        let lease_id =
            LeaseId::new("lease-1").expect("lease ID must be valid");

        let owner =
            LeaseOwnerId::new("owner-1").expect("owner must be valid");

        let epoch =
            LeaseEpoch::try_from_u64(1).expect("epoch must be valid");

        let duration =
            LeaseDuration::new(Duration::from_secs(10))
                .expect("duration must be valid");

        let scope =
            LeaseScope::logical_domain("domain")
                .expect("scope must be valid");

        let request = LeaseAcquireRequest::new(
            lease_id,
            scope,
            owner,
            epoch,
            duration,
            LeaseAcquisitionMode::Exclusive,
        )
        .expect("request must be constructible");

        assert!(validate_acquire_request(&request).is_ok());
    }

    #[test]
    fn renewal_rejects_mismatched_lease_id() {
        let lease_id =
            LeaseId::new("lease-1").expect("lease ID must be valid");

        let other_id =
            LeaseId::new("lease-2").expect("lease ID must be valid");

        let owner =
            LeaseOwnerId::new("owner-1").expect("owner must be valid");

        let epoch =
            LeaseEpoch::try_from_u64(1).expect("epoch must be valid");

        let token =
            FencingToken::try_from_u64(1).expect("token must be valid");

        let sequence = LeaseSequence::initial();

        let proof = LeaseProof::new(
            other_id,
            owner,
            epoch,
            token,
            sequence,
        );

        let duration =
            LeaseDuration::new(Duration::from_secs(10))
                .expect("duration must be valid");

        let request =
            LeaseRenewRequest::new(lease_id, proof, duration);

        assert!(request.is_err());
    }

    #[test]
    fn release_rejects_mismatched_lease_id() {
        let lease_id =
            LeaseId::new("lease-1").expect("lease ID must be valid");

        let other_id =
            LeaseId::new("lease-2").expect("lease ID must be valid");

        let owner =
            LeaseOwnerId::new("owner-1").expect("owner must be valid");

        let epoch =
            LeaseEpoch::try_from_u64(1).expect("epoch must be valid");

        let token =
            FencingToken::try_from_u64(1).expect("token must be valid");

        let proof = LeaseProof::new(
            other_id,
            owner,
            epoch,
            token,
            LeaseSequence::initial(),
        );

        let request =
            LeaseReleaseRequest::new(lease_id, proof);

        assert!(request.is_err());
    }

    #[test]
    fn transition_acquire_is_valid() {
        assert!(LeaseTransition::is_valid(
            LeaseTransition::Acquire,
            LeaseState::Pending,
            LeaseState::Active,
        ));
    }

    #[test]
    fn transition_release_is_valid() {
        assert!(LeaseTransition::is_valid(
            LeaseTransition::Release,
            LeaseState::Active,
            LeaseState::Released,
        ));
    }

    #[test]
    fn transition_invalid_state_is_rejected() {
        assert!(!LeaseTransition::is_valid(
            LeaseTransition::Acquire,
            LeaseState::Released,
            LeaseState::Active,
        ));
    }

    #[test]
    fn validation_request_rejects_stale_epoch() {
        let lease_id =
            LeaseId::new("lease-1").expect("lease ID must be valid");

        let owner =
            LeaseOwnerId::new("owner-1").expect("owner must be valid");

        let proof_epoch =
            LeaseEpoch::try_from_u64(1).expect("epoch must be valid");

        let requested_epoch =
            LeaseEpoch::try_from_u64(2).expect("epoch must be valid");

        let token =
            FencingToken::try_from_u64(1).expect("token must be valid");

        let proof = LeaseProof::new(
            lease_id.clone(),
            owner,
            proof_epoch,
            token,
            LeaseSequence::initial(),
        );

        let scope =
            LeaseScope::logical_domain("domain")
                .expect("scope must be valid");

        let result = LeaseValidationRequest::new(
            lease_id,
            proof,
            scope,
            requested_epoch,
        );

        assert!(result.is_err());
    }

    #[test]
    fn proof_preserves_fencing_information() {
        let lease_id =
            LeaseId::new("lease-1").expect("lease ID must be valid");

        let owner =
            LeaseOwnerId::new("owner-1").expect("owner must be valid");

        let epoch =
            LeaseEpoch::try_from_u64(7).expect("epoch must be valid");

        let token =
            FencingToken::try_from_u64(41).expect("token must be valid");

        let sequence =
            LeaseSequence::try_from_u64(3)
                .expect("sequence must be valid");

        let proof = LeaseProof::new(
            lease_id.clone(),
            owner.clone(),
            epoch,
            token,
            sequence,
        );

        assert_eq!(proof.lease_id(), &lease_id);
        assert_eq!(proof.owner(), &owner);
        assert_eq!(proof.epoch(), epoch);
        assert_eq!(proof.fencing_token(), token);
        assert_eq!(proof.sequence(), sequence);
    }

    #[test]
    fn active_state_is_active() {
        assert!(LeaseState::Active.is_active());
        assert!(LeaseState::Renewing.is_active());
    }

    #[test]
    fn terminal_states_are_terminal() {
        assert!(LeaseState::Expired.is_terminal());
        assert!(LeaseState::Released.is_terminal());
        assert!(LeaseState::Revoked.is_terminal());
        assert!(LeaseState::Superseded.is_terminal());
    }
}