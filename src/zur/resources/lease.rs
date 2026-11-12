
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Resource Leases
//!
//! Path: src/zur/resources/lease.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//!
//! ## Purpose
//!
//! Provides safely owned, explicitly acquired resource leases.
//!
//! A resource lease represents temporary authorization or reservation
//! of an externally managed computational resource.
//!
//! It supports:
//!
//! - Explicit acquisition.
//! - Strongly typed lease identities.
//! - Open-ended resource references.
//! - Explicit ownership scopes.
//! - Optional expiration.
//! - Lease renewal.
//! - Explicit release.
//! - Failure-aware lifecycle tracking.
//! - Shared lease handles.
//! - Execution-session integration.
//!
//! ## Ownership
//!
//! This module owns resource lease lifecycle and lease handles.
//!
//! It does NOT own:
//!
//! - Resource requirement semantics.
//! - Resource capability definitions.
//! - Hardware discovery.
//! - Provider registration.
//! - Physical allocation algorithms.
//! - Resource feasibility.
//! - Resource scheduling.
//! - Backend selection.
//! - Domain-specific resource representations.
//! - Distributed membership.
//!
//! Resource acquisition is delegated to an explicitly injected
//! ResourceLeaseAuthority.
//!
//! ## Existing canonical contracts
//!
//! - zur::ids::ResourceLeaseId
//! - zur::ids::IdAllocator
//! - zur::contracts::provider::ResourceRequirementRef
//! - zur::error::ZurError
//! - zur::error::ZurResult
//! - zur::execution::session::ResourceAccountingReference
//!
//! No competing identity or requirement model is introduced.
//!
//! ## Important distinctions
//!
//! Requirement != Reservation.
//! Capability != Availability.
//! Feasibility != Acquisition.
//! Acquisition != Execution.
//! Expiration != Confirmed physical release.
//! Cancellation != Confirmed remote termination.
//!
//! ## Scalability
//!
//! No arbitrary maximum is imposed on:
//!
//! - Resource leases.
//! - Resource types.
//! - Computational domains.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - FPGAs.
//! - QPUs.
//! - Distributed participants.
//! - Resource quantities.
//!
//! Storage grows according to actual requirements and available memory.
//!
//! Literal infinite physical resources are not promised.
//!
//! ## Safety
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No raw pointer ownership.
//! No global mutable state.
//! No fabricated reservations.
//! No fabricated availability.
//! No automatic hardware discovery.
//! No silent release failures.
//!
//! ## Concurrency
//!
//! Lease handles may be shared between threads.
//!
//! Lifecycle transitions are synchronized.
//!
//! External authority callbacks execute without holding lease-state locks.
//!
//! A failed or ambiguous remote operation is represented explicitly.
//!
//! ## Execution-session integration
//!
//! A ResourceLease can produce the existing:
//!
//! ResourceAccountingReference
//!
//! ExecutionSession retains that reference without taking ownership
//! of the physical allocation authority.

// ============================================================================
// IMPORTS
// ============================================================================

use core::fmt;

use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use crate::zur::contracts::provider::ResourceRequirementRef;

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::ids::{
    ExecutionId,
    IdAllocator,
    ResourceLeaseId,
    SessionId,
};

use crate::zur::execution::session::ResourceAccountingReference;

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.resources.lease";

// ============================================================================
// RESOURCE LEASE AUTHORITY
// ============================================================================

/// Authoritative external resource acquisition and release interface.
///
/// Implementations connect ZUR to the actual resource-management
/// authority, provider infrastructure, or platform resource manager.
///
/// This trait does not define provider discovery or provider registration.
///
/// Those responsibilities remain with their existing subsystems.
///
/// # Acquisition guarantee
///
/// `acquire` MUST be transactional from the caller's perspective.
///
/// If it returns an error, the authority MUST ensure that no
/// reservation remains committed for the requested lease identity.
///
/// Implementations that use remote protocols must reconcile ambiguous
/// responses before returning an ordinary acquisition failure.
///
/// # Renewal guarantee
///
/// Renewal failures may have ambiguous remote outcomes.
///
/// ZUR therefore marks a lease uncertain when renewal fails.
///
/// # Release guarantee
///
/// Release MUST be idempotent.
///
/// Repeating a release request for an already released lease must not
/// create another reservation or corrupt resource accounting.
pub trait ResourceLeaseAuthority: Send + Sync + 'static {

    /// Acquires the requested resource reservation.
    ///
    /// The implementation must return a confirmed grant only after
    /// the underlying authority has actually accepted the reservation.
    ///
    /// # Arguments
    ///
    /// * `lease_id` - ZUR-generated identity for this reservation.
    /// * `requirement` - Reference to the canonical external requirement.
    /// * `owner` - Explicit owner and lifetime scope.
    /// * `requested_ttl` - Optional requested lease duration.
    ///
    /// # Errors
    ///
    /// Returns an error if acquisition cannot be confirmed.
    fn acquire(
        &self,
        lease_id: &ResourceLeaseId,
        requirement: &ResourceRequirementRef,
        owner: &LeaseOwner,
        requested_ttl: Option<Duration>,
    ) -> ZurResult<LeaseGrant>;

    /// Renews an existing reservation.
    ///
    /// Returns the newly confirmed monotonic expiration instant.
    ///
    /// The returned instant must be in the future.
    ///
    /// # Errors
    ///
    /// Returns an error if renewal cannot be confirmed.
    fn renew(
        &self,
        lease_id: &ResourceLeaseId,
        owner: &LeaseOwner,
        requested_ttl: Duration,
    ) -> ZurResult<Instant>;

    /// Releases an existing reservation.
    ///
    /// This operation must be idempotent.
    ///
    /// # Errors
    ///
    /// Returns an error if release cannot be confirmed.
    fn release(
        &self,
        lease_id: &ResourceLeaseId,
    ) -> ZurResult<()>;

    /// Returns the stable, namespaced identity of this authority.
    ///
    /// This identity is descriptive metadata, not proof of
    /// authorization or availability.
    fn authority_id(&self) -> &str;
}

// ============================================================================
// LEASE OWNER
// ============================================================================

/// Explicit owner and lifetime scope of a resource lease.
///
/// Scope names remain open-ended.
///
/// Examples include:
///
/// - expression
/// - operation
/// - block
/// - function
/// - task
/// - actor
/// - module
/// - program
/// - execution
/// - deployment
///
/// These are descriptive lifetime boundaries, not a closed list.
///
/// Physical resource lifetime remains managed by the external authority.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LeaseOwner {

    scope: String,

    identity: String,
}

impl LeaseOwner {

    /// Creates a validated lease owner.
    ///
    /// Both values must be nonempty and contain no control characters.
    pub fn new(
        scope: impl Into<String>,
        identity: impl Into<String>,
    ) -> ZurResult<Self> {

        let scope = scope.into();
        let identity = identity.into();

        validate_label(&scope, "lease owner scope")?;
        validate_label(&identity, "lease owner identity")?;

        Ok(Self {
            scope,
            identity,
        })
    }

    /// Creates an owner associated with an execution session.
    pub fn session(
        identity: SessionId,
    ) -> Self {

        Self {
            scope: "session".to_owned(),
            identity: identity.to_string(),
        }
    }

    /// Creates an owner associated with an execution.
    pub fn execution(
        identity: ExecutionId,
    ) -> Self {

        Self {
            scope: "execution".to_owned(),
            identity: identity.to_string(),
        }
    }

    /// Returns the owner lifetime scope.
    #[must_use]
    pub fn scope(&self) -> &str {
        &self.scope
    }

    /// Returns the canonical owner identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

// ============================================================================
// CONFIRMED LEASE GRANT
// ============================================================================

/// Confirmed reservation returned by a resource authority.
///
/// A grant must only be constructed after the external resource
/// authority has actually confirmed acquisition.
///
/// Creating this value does not itself allocate or reserve hardware.
///
/// The constructor is public because independently implemented
/// authorities must be able to return grants.
#[derive(Debug, Clone)]
pub struct LeaseGrant {

    lease_id: ResourceLeaseId,

    resource_reference: String,

    expires_at: Option<Instant>,
}

impl LeaseGrant {

    /// Constructs a grant representing an already-confirmed reservation.
    ///
    /// The resource reference is an opaque, authority-owned identifier.
    ///
    /// It does not introduce a second resource identity model.
    ///
    /// An optional expiration is expressed using a monotonic clock.
    pub fn confirmed(
        lease_id: ResourceLeaseId,
        resource_reference: impl Into<String>,
        expires_at: Option<Instant>,
    ) -> ZurResult<Self> {

        let resource_reference = resource_reference.into();

        validate_label(
            &resource_reference,
            "resource reference",
        )?;

        Ok(Self {
            lease_id,
            resource_reference,
            expires_at,
        })
    }

    /// Returns the ZUR resource lease identity.
    #[must_use]
    pub fn lease_id(&self) -> &ResourceLeaseId {
        &self.lease_id
    }

    /// Returns the authority-owned resource reference.
    #[must_use]
    pub fn resource_reference(&self) -> &str {
        &self.resource_reference
    }

    /// Returns the optional monotonic expiration instant.
    #[must_use]
    pub fn expires_at(&self) -> Option<Instant> {
        self.expires_at
    }
}

// ============================================================================
// LEASE LIFECYCLE
// ============================================================================

/// Observable resource lease lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LeaseStatus {

    /// Reservation is confirmed and has not expired.
    Active,

    /// Reservation's locally recorded expiration has passed.
    ///
    /// This does not prove that the external resource was released.
    Expired,

    /// Release is currently being requested.
    Releasing,

    /// Release has been confirmed by the authority.
    Released,

    /// The external outcome is uncertain.
    ///
    /// The lease must not be treated as usable.
    /// Reconciliation or confirmed release is required.
    Uncertain,

    /// Renewal is currently being requested.
    Renewing,
}

// ============================================================================
// INTERNAL LEASE STATE
// ============================================================================

struct LeaseState {

    status: LeaseStatus,

    expires_at: Option<Instant>,
}

struct LeaseInner {

    lease_id: ResourceLeaseId,

    authority_id: String,

    resource_reference: String,

    owner: LeaseOwner,

    requirement: ResourceRequirementRef,

    authority: Arc<dyn ResourceLeaseAuthority>,

    state: Mutex<LeaseState>,
}

// ============================================================================
// RESOURCE LEASE
// ============================================================================

/// Safely owned handle to one confirmed resource reservation.
///
/// Cloning a ResourceLease shares the same underlying lease.
///
/// It does not create another physical reservation.
///
/// Releasing one shared handle transitions the shared lease for all
/// other holders.
///
/// Physical release is explicit through `release`.
///
/// Dropping a Rust handle is not treated as proof that a remote
/// physical reservation has been released.
#[derive(Clone)]
pub struct ResourceLease {

    inner: Arc<LeaseInner>,
}

impl ResourceLease {

    fn from_grant(
        grant: LeaseGrant,
        authority_id: String,
        owner: LeaseOwner,
        requirement: ResourceRequirementRef,
        authority: Arc<dyn ResourceLeaseAuthority>,
    ) -> Self {

        Self {
            inner: Arc::new(LeaseInner {

                lease_id: grant.lease_id,

                authority_id,

                resource_reference: grant.resource_reference,

                owner,

                requirement,

                authority,

                state: Mutex::new(LeaseState {
                    status: LeaseStatus::Active,
                    expires_at: grant.expires_at,
                }),
            }),
        }
    }

    // ------------------------------------------------------------------------
    // IDENTITY
    // ------------------------------------------------------------------------

    /// Returns the canonical resource lease identity.
    #[must_use]
    pub fn id(&self) -> &ResourceLeaseId {
        &self.inner.lease_id
    }

    /// Returns the external authority identity.
    #[must_use]
    pub fn authority_id(&self) -> &str {
        &self.inner.authority_id
    }

    /// Returns the authority-owned resource reference.
    #[must_use]
    pub fn resource_reference(&self) -> &str {
        &self.inner.resource_reference
    }

    /// Returns the explicit lease owner.
    #[must_use]
    pub fn owner(&self) -> &LeaseOwner {
        &self.inner.owner
    }

    /// Returns the canonical requirement reference.
    #[must_use]
    pub fn requirement(&self) -> &ResourceRequirementRef {
        &self.inner.requirement
    }

    /// Returns the number of local shared Rust handles.
    ///
    /// This is diagnostic information only.
    ///
    /// It does not represent the number of remote reservations.
    #[must_use]
    pub fn local_handle_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    // ------------------------------------------------------------------------
    // STATE
    // ------------------------------------------------------------------------

    /// Returns the current observable lease status.
    ///
    /// Expiration is evaluated against the monotonic clock.
    ///
    /// The stored state is not silently changed to Released.
    pub fn status(&self) -> ZurResult<LeaseStatus> {

        let state = lock_state(
            &self.inner.state,
        );

        if state.status == LeaseStatus::Active {

            if state
                .expires_at
                .is_some_and(|expiry| Instant::now() >= expiry)
            {
                return Ok(LeaseStatus::Expired);
            }
        }

        Ok(state.status)
    }

    /// Returns the current expiration instant.
    pub fn expires_at(&self) -> ZurResult<Option<Instant>> {

        let state = lock_state(
            &self.inner.state,
        );

        Ok(state.expires_at)
    }

    /// Checks whether the lease can currently be used.
    ///
    /// An expired, uncertain, releasing, or released lease is rejected.
    pub fn ensure_active(&self) -> ZurResult<()> {

        match self.status()? {

            LeaseStatus::Active => Ok(()),

            LeaseStatus::Expired => Err(
                lease_error(
                    crate::zur::error::ErrorKind::TimedOut,
                    "zur.resources.lease.expired",
                    "resource lease has expired",
                    "ensure_active",
                )
            ),

            LeaseStatus::Released => Err(
                lease_error(
                    crate::zur::error::ErrorKind::ResourceUnavailable,
                    "zur.resources.lease.released",
                    "resource lease has already been released",
                    "ensure_active",
                )
            ),

            LeaseStatus::Uncertain => Err(
                lease_error(
                    crate::zur::error::ErrorKind::Recovery,
                    "zur.resources.lease.uncertain",
                    "resource lease has an uncertain external state",
                    "ensure_active",
                )
            ),

            LeaseStatus::Releasing => Err(
                lease_error(
                    crate::zur::error::ErrorKind::Lifecycle,
                    "zur.resources.lease.releasing",
                    "resource lease is being released",
                    "ensure_active",
                )
            ),

            LeaseStatus::Renewing => Err(
                lease_error(
                    crate::zur::error::ErrorKind::Lifecycle,
                    "zur.resources.lease.renewing",
                    "resource lease renewal is in progress",
                    "ensure_active",
                )
            ),
        }
    }

    // ------------------------------------------------------------------------
    // RENEWAL
    // ------------------------------------------------------------------------

    /// Requests renewal of the existing reservation.
    ///
    /// The external authority must confirm the renewal.
    ///
    /// A failed or ambiguous renewal changes the lease state to
    /// Uncertain because remote success cannot safely be excluded.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - Duration is zero.
    /// - Lease is expired.
    /// - Lease is released.
    /// - Lease is already transitioning.
    /// - The external authority rejects renewal.
    /// - The returned expiration is invalid.
    pub fn renew(
        &self,
        requested_ttl: Duration,
    ) -> ZurResult<Instant> {

        if requested_ttl.is_zero() {

            return Err(
                ZurError::invalid_input(
                    "resource lease renewal duration must be positive",
                )
                .with_component(COMPONENT)
                .with_operation("renew"),
            );
        }

        {
            let mut state = lock_state(
                &self.inner.state,
            );

            if state.status != LeaseStatus::Active {

                return Err(
                    lease_error(
                        crate::zur::error::ErrorKind::Lifecycle,
                        "zur.resources.lease.not_renewable",
                        "resource lease is not in an active lifecycle state",
                        "renew",
                    )
                );
            }

            if state
                .expires_at
                .is_some_and(|expiry| Instant::now() >= expiry)
            {
                return Err(
                    lease_error(
                        crate::zur::error::ErrorKind::TimedOut,
                        "zur.resources.lease.expired",
                        "an expired resource lease cannot be renewed",
                        "renew",
                    )
                );
            }

            state.status = LeaseStatus::Renewing;
        }

        // Never invoke an external authority while holding a state lock.
        let result = self.inner.authority.renew(
            &self.inner.lease_id,
            &self.inner.owner,
            requested_ttl,
        );

        let mut state = lock_state(
            &self.inner.state,
        );

        match result {

            Ok(expiration) => {

                if expiration <= Instant::now() {

                    state.status = LeaseStatus::Uncertain;

                    return Err(
                        lease_error(
                            crate::zur::error::ErrorKind::ContractViolation,
                            "zur.resources.lease.invalid_renewal",
                            "authority returned an expired renewal",
                            "renew",
                        )
                    );
                }

                state.expires_at = Some(expiration);

                state.status = LeaseStatus::Active;

                Ok(expiration)
            }

            Err(error) => {

                // A remote error may occur after a successful
                // reservation update. Do not assume the old state
                // still describes the external reservation.

                state.status = LeaseStatus::Uncertain;

                Err(
                    error
                        .with_component(COMPONENT)
                        .with_operation("renew")
                )
            }
        }
    }

    // ------------------------------------------------------------------------
    // RELEASE
    // ------------------------------------------------------------------------

    /// Explicitly releases the external reservation.
    ///
    /// Release is shared across cloned handles.
    ///
    /// A confirmed release changes all handles to Released.
    ///
    /// A failed release changes the lease to Uncertain.
    ///
    /// The operation may be retried because the authority contract
    /// requires idempotent release.
    ///
    /// Returns:
    ///
    /// - `true` when this call confirms release.
    /// - `false` when release was already confirmed.
    pub fn release(&self) -> ZurResult<bool> {

        {
            let mut state = lock_state(
                &self.inner.state,
            );

            match state.status {

                LeaseStatus::Released => {
                    return Ok(false);
                }

                LeaseStatus::Releasing
                | LeaseStatus::Renewing => {

                    return Err(
                        lease_error(
                            crate::zur::error::ErrorKind::Lifecycle,
                            "zur.resources.lease.transition_in_progress",
                            "another lease lifecycle operation is in progress",
                            "release",
                        )
                    );
                }

                LeaseStatus::Active
                | LeaseStatus::Expired
                | LeaseStatus::Uncertain => {

                    state.status = LeaseStatus::Releasing;
                }
            }
        }

        // The authority callback executes outside the state lock.
        let result = self.inner.authority.release(
            &self.inner.lease_id,
        );

        let mut state = lock_state(
            &self.inner.state,
        );

        match result {

            Ok(()) => {

                state.status = LeaseStatus::Released;

                Ok(true)
            }

            Err(error) => {

                // Remote release may have succeeded even when its
                // acknowledgement was lost.

                state.status = LeaseStatus::Uncertain;

                Err(
                    error
                        .with_component(COMPONENT)
                        .with_operation("release")
                )
            }
        }
    }

    // ------------------------------------------------------------------------
    // EXECUTION SESSION INTEGRATION
    // ------------------------------------------------------------------------

    /// Creates the existing execution-session accounting reference.
    ///
    /// The session retains a strong reference to this lease.
    ///
    /// The session does not become the physical allocation authority.
    ///
    /// Example:
    ///
    /// ```ignore
    /// let reference = lease.session_reference();
    ///
    /// session.attach_resource_reference(reference)?;
    /// ```
    #[must_use]
    pub fn session_reference(
        &self,
    ) -> ResourceAccountingReference {

        ResourceAccountingReference::new(
            self.inner.lease_id.clone(),
            Arc::new(self.clone()),
        )
    }
}

// ============================================================================
// DEBUG
// ============================================================================

impl fmt::Debug for ResourceLease {

    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {

        formatter
            .debug_struct("ResourceLease")
            .field("lease_id", &self.inner.lease_id)
            .field("authority", &self.inner.authority_id)
            .field(
                "resource_reference",
                &self.inner.resource_reference,
            )
            .field("owner", &self.inner.owner)
            .field(
                "status",
                &self.status().ok(),
            )
            .finish()
    }
}

// ============================================================================
// RESOURCE LEASE MANAGER
// ============================================================================

/// Dependency-injected resource lease acquisition coordinator.
///
/// The manager:
///
/// - Allocates canonical lease identities.
/// - Calls the injected resource authority.
/// - Validates confirmed grants.
/// - Constructs safely owned lease handles.
///
/// It does not discover resources or choose providers.
///
/// Provider selection and actual physical reservation remain with
/// the appropriate external resource-management authority.
pub struct ResourceLeaseManager {

    identities: Arc<IdAllocator<ResourceLeaseId>>,

    authority: Arc<dyn ResourceLeaseAuthority>,
}

impl ResourceLeaseManager {

    /// Creates a manager with explicit identity and authority dependencies.
    ///
    /// The identity allocator must be shared wherever uniqueness within
    /// the same identity namespace is required.
    pub fn new(
        identities: Arc<IdAllocator<ResourceLeaseId>>,
        authority: Arc<dyn ResourceLeaseAuthority>,
    ) -> ZurResult<Self> {

        validate_label(
            authority.authority_id(),
            "resource authority identity",
        )?;

        Ok(Self {
            identities,
            authority,
        })
    }

    /// Acquires a resource reservation.
    ///
    /// This operation is explicit and delegates actual reservation
    /// to the injected authority.
    ///
    /// No availability or feasibility is inferred.
    ///
    /// # Arguments
    ///
    /// * `requirement` - Existing canonical requirement reference.
    /// * `owner` - Explicit lease owner and scope.
    /// * `requested_ttl` - Optional requested reservation duration.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - Identity allocation fails.
    /// - The requested duration is invalid.
    /// - The authority rejects acquisition.
    /// - The grant is inconsistent.
    /// - The grant has already expired.
    pub fn acquire(
        &self,
        requirement: ResourceRequirementRef,
        owner: LeaseOwner,
        requested_ttl: Option<Duration>,
    ) -> ZurResult<ResourceLease> {

        if requested_ttl.is_some_and(Duration::is_zero) {

            return Err(
                ZurError::invalid_input(
                    "requested resource lease duration must be positive",
                )
                .with_component(COMPONENT)
                .with_operation("acquire"),
            );
        }

        let lease_id = self.identities
            .try_next()
            .map_err(|error| {

                ZurError::resource_unavailable(
                    "unable to allocate resource lease identity",
                )
                .with_component(COMPONENT)
                .with_operation("acquire")
                .with_source(error)
            })?;

        // This is the actual acquisition boundary.
        // Identity allocation alone does not reserve resources.

        let grant = self.authority.acquire(
            &lease_id,
            &requirement,
            &owner,
            requested_ttl,
        )?;

        // A provider must not return a grant for another identity.
        if grant.lease_id != lease_id {

            // Attempt to clean up the unexpected grant.
            // The cleanup result is deliberately not represented
            // as successful when the authority cannot confirm it.
            let _cleanup_result = self.authority.release(
                &grant.lease_id,
            );

            return Err(
                lease_error(
                    crate::zur::error::ErrorKind::ContractViolation,
                    "zur.resources.lease.identity_mismatch",
                    "resource authority returned a different lease identity",
                    "acquire",
                )
            );
        }

        if grant
            .expires_at
            .is_some_and(|expiry| expiry <= Instant::now())
        {

            let _cleanup_result = self.authority.release(
                &grant.lease_id,
            );

            return Err(
                lease_error(
                    crate::zur::error::ErrorKind::ContractViolation,
                    "zur.resources.lease.expired_grant",
                    "resource authority returned an already expired grant",
                    "acquire",
                )
            );
        }

        Ok(ResourceLease::from_grant(
            grant,
            self.authority.authority_id().to_owned(),
            owner,
            requirement,
            Arc::clone(&self.authority),
        ))
    }

    /// Returns the canonical identity allocator.
    ///
    /// This is an allocation facility, not a reservation interface.
    #[must_use]
    pub fn identity_allocator(
        &self,
    ) -> &Arc<IdAllocator<ResourceLeaseId>> {
        &self.identities
    }

    /// Returns the injected resource authority.
    ///
    /// This does not imply resource availability.
    #[must_use]
    pub fn authority(
        &self,
    ) -> &Arc<dyn ResourceLeaseAuthority> {
        &self.authority
    }
}

// ============================================================================
// INTERNAL VALIDATION
// ============================================================================

fn validate_label(
    value: &str,
    description: &str,
) -> ZurResult<()> {

    if value.trim().is_empty() {

        return Err(
            ZurError::invalid_input(
                format!("{description} cannot be empty"),
            )
            .with_component(COMPONENT)
        );
    }

    if value.chars().any(char::is_control) {

        return Err(
            ZurError::invalid_input(
                format!("{description} contains control characters"),
            )
            .with_component(COMPONENT)
        );
    }

    Ok(())
}

// ============================================================================
// INTERNAL LOCKING
// ============================================================================

fn lock_state(
    state: &Mutex<LeaseState>,
) -> MutexGuard<'_, LeaseState> {

    // No external callbacks execute under this lock.
    // Poison recovery preserves access to the explicit lifecycle
    // state so that a subsequent operation can report or reconcile it.
    state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn lease_error(
    kind: crate::zur::error::ErrorKind,
    code: &'static str,
    message: impl Into<String>,
    operation: &'static str,
) -> ZurError {

    ZurError::with_code(
        kind,
        code,
        message,
    )
    .unwrap_or_else(|_| {
        ZurError::resource_unavailable(
            "unable to construct resource lease error",
        )
    })
    .with_component(COMPONENT)
    .with_operation(operation)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {

    use super::*;

    use std::collections::BTreeSet;
    use std::sync::Mutex;

    // ------------------------------------------------------------------------
    // TEST AUTHORITY
    // ------------------------------------------------------------------------

    struct TestAuthority {

        active: Mutex<BTreeSet<ResourceLeaseId>>,

        fail_release: Mutex<bool>,
    }

    impl TestAuthority {

        fn new() -> Self {

            Self {
                active: Mutex::new(BTreeSet::new()),
                fail_release: Mutex::new(false),
            }
        }

        fn active_count(&self) -> usize {

            self.active
                .lock()
                .unwrap()
                .len()
        }
    }

    impl ResourceLeaseAuthority for TestAuthority {

        fn acquire(
            &self,
            lease_id: &ResourceLeaseId,
            _requirement: &ResourceRequirementRef,
            _owner: &LeaseOwner,
            requested_ttl: Option<Duration>,
        ) -> ZurResult<LeaseGrant> {

            self.active
                .lock()
                .unwrap()
                .insert(lease_id.clone());

            let expiry = requested_ttl
                .and_then(|duration| Instant::now().checked_add(duration));

            LeaseGrant::confirmed(
                lease_id.clone(),
                "test.resource.instance",
                expiry,
            )
        }

        fn renew(
            &self,
            lease_id: &ResourceLeaseId,
            _owner: &LeaseOwner,
            requested_ttl: Duration,
        ) -> ZurResult<Instant> {

            if !self.active
                .lock()
                .unwrap()
                .contains(lease_id)
            {
                return Err(
                    ZurError::resource_unavailable(
                        "test reservation does not exist",
                    )
                );
            }

            Instant::now()
                .checked_add(requested_ttl)
                .ok_or_else(|| {
                    ZurError::invalid_input(
                        "test expiration cannot be represented",
                    )
                })
        }

        fn release(
            &self,
            lease_id: &ResourceLeaseId,
        ) -> ZurResult<()> {

            if *self.fail_release.lock().unwrap() {

                return Err(
                    ZurError::resource_unavailable(
                        "test release failure",
                    )
                );
            }

            self.active
                .lock()
                .unwrap()
                .remove(lease_id);

            Ok(())
        }

        fn authority_id(&self) -> &str {
            "zamani.test.resource_authority"
        }
    }

    // ------------------------------------------------------------------------
    // FIXTURES
    // ------------------------------------------------------------------------

    fn requirement() -> ResourceRequirementRef {

        ResourceRequirementRef::new(
            "zamani.semantic.resources",
            "requirement.compute",
        )
        .unwrap()
    }

    fn manager(
        authority: Arc<TestAuthority>,
    ) -> ResourceLeaseManager {

        let namespace = crate::zur::ids::IdNamespace::new(
            "zamani.runtime.resource.lease",
        )
        .unwrap();

        let allocator = Arc::new(
            IdAllocator::<ResourceLeaseId>::new(namespace),
        );

        ResourceLeaseManager::new(
            allocator,
            authority,
        )
        .unwrap()
    }

    fn acquire(
        manager: &ResourceLeaseManager,
    ) -> ResourceLease {

        manager.acquire(
            requirement(),
            LeaseOwner::new(
                "execution",
                "test.execution.1",
            )
            .unwrap(),
            Some(Duration::from_secs(60)),
        )
        .unwrap()
    }

    // ------------------------------------------------------------------------
    // OWNER VALIDATION
    // ------------------------------------------------------------------------

    #[test]
    fn rejects_empty_owner_scope() {

        assert!(
            LeaseOwner::new(
                "",
                "execution.1",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_empty_owner_identity() {

        assert!(
            LeaseOwner::new(
                "execution",
                "",
            )
            .is_err()
        );
    }

    #[test]
    fn accepts_extensible_owner_scopes() {

        let owner = LeaseOwner::new(
            "future.execution.scope",
            "external.owner.identity",
        )
        .unwrap();

        assert_eq!(
            owner.scope(),
            "future.execution.scope",
        );
    }

    // ------------------------------------------------------------------------
    // ACQUISITION
    // ------------------------------------------------------------------------

    #[test]
    fn acquires_only_through_the_injected_authority() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let lease = acquire(&manager);

        assert_eq!(
            authority.active_count(),
            1,
        );

        assert_eq!(
            lease.status().unwrap(),
            LeaseStatus::Active,
        );
    }

    #[test]
    fn leases_have_distinct_identities() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let first = acquire(&manager);
        let second = acquire(&manager);

        assert_ne!(
            first.id(),
            second.id(),
        );
    }

    #[test]
    fn rejects_zero_requested_duration() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(authority);

        let result = manager.acquire(
            requirement(),
            LeaseOwner::new(
                "execution",
                "execution.1",
            )
            .unwrap(),
            Some(Duration::ZERO),
        );

        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // SHARED OWNERSHIP
    // ------------------------------------------------------------------------

    #[test]
    fn cloned_handles_share_one_reservation() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let first = acquire(&manager);

        let second = first.clone();

        assert_eq!(
            first.id(),
            second.id(),
        );

        assert_eq!(
            authority.active_count(),
            1,
        );

        assert_eq!(
            first.local_handle_count(),
            2,
        );
    }

    #[test]
    fn release_affects_all_shared_handles() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let first = acquire(&manager);

        let second = first.clone();

        assert!(first.release().unwrap());

        assert_eq!(
            second.status().unwrap(),
            LeaseStatus::Released,
        );

        assert_eq!(
            authority.active_count(),
            0,
        );
    }

    #[test]
    fn repeated_release_is_idempotent() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let lease = acquire(&manager);

        assert!(lease.release().unwrap());

        assert!(!lease.release().unwrap());
    }

    // ------------------------------------------------------------------------
    // RENEWAL
    // ------------------------------------------------------------------------

    #[test]
    fn renews_a_confirmed_reservation() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let lease = acquire(&manager);

        let expiration = lease
            .renew(Duration::from_secs(120))
            .unwrap();

        assert!(expiration > Instant::now());

        assert_eq!(
            lease.status().unwrap(),
            LeaseStatus::Active,
        );
    }

    #[test]
    fn renewal_after_release_is_rejected() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let lease = acquire(&manager);

        lease.release().unwrap();

        assert!(
            lease.renew(Duration::from_secs(60)).is_err()
        );
    }

    // ------------------------------------------------------------------------
    // FAILURE HANDLING
    // ------------------------------------------------------------------------

    #[test]
    fn failed_release_does_not_claim_success() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let lease = acquire(&manager);

        *authority.fail_release.lock().unwrap() = true;

        assert!(lease.release().is_err());

        assert_eq!(
            lease.status().unwrap(),
            LeaseStatus::Uncertain,
        );
    }

    #[test]
    fn uncertain_lease_cannot_be_used() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let lease = acquire(&manager);

        *authority.fail_release.lock().unwrap() = true;

        assert!(lease.release().is_err());

        assert!(lease.ensure_active().is_err());
    }

    #[test]
    fn uncertain_release_can_be_reconciled() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let lease = acquire(&manager);

        *authority.fail_release.lock().unwrap() = true;

        assert!(lease.release().is_err());

        *authority.fail_release.lock().unwrap() = false;

        assert!(lease.release().unwrap());

        assert_eq!(
            lease.status().unwrap(),
            LeaseStatus::Released,
        );
    }

    // ------------------------------------------------------------------------
    // SESSION INTEGRATION
    // ------------------------------------------------------------------------

    #[test]
    fn creates_canonical_session_accounting_reference() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let lease = acquire(&manager);

        let reference = lease.session_reference();

        assert_eq!(
            reference.lease_id(),
            lease.id(),
        );

        let retained = reference
            .lease::<ResourceLease>()
            .unwrap();

        assert_eq!(
            retained.id(),
            lease.id(),
        );
    }

    // ------------------------------------------------------------------------
    // SCALABILITY
    // ------------------------------------------------------------------------

    #[test]
    fn represents_large_resource_references_without_fixed_limits() {

        let large_reference = format!(
            "resource.{}",
            "x".repeat(4096),
        );

        let grant = LeaseGrant::confirmed(
            {
                let allocator = IdAllocator::<ResourceLeaseId>::new(
                    crate::zur::ids::IdNamespace::new(
                        "zamani.runtime.resource.lease",
                    )
                    .unwrap(),
                );

                allocator.try_next().unwrap()
            },
            large_reference.clone(),
            None,
        )
        .unwrap();

        assert_eq!(
            grant.resource_reference(),
            large_reference,
        );
    }

    #[test]
    fn permits_unbounded_lifetime_when_authority_confirms_it() {

        let authority = Arc::new(TestAuthority::new());

        let manager = manager(
            Arc::clone(&authority),
        );

        let lease = manager.acquire(
            requirement(),
            LeaseOwner::new(
                "deployment",
                "deployment.1",
            )
            .unwrap(),
            None,
        )
        .unwrap();

        assert_eq!(
            lease.expires_at().unwrap(),
            None,
        );

        assert_eq!(
            lease.status().unwrap(),
            LeaseStatus::Active,
        );
    }
}
