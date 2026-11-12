
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Resource Lease Lifecycle
//!
//! Path: src/zur/resources/lifecycle.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the authoritative operational lifecycle state machine for
//! one ZUR resource lease.
//!
//! This module tracks the lifecycle of a resource reservation and
//! acquisition without performing physical resource operations.
//!
//! The resource manager and its selected provider perform the actual
//! reservation, acquisition and release operations.
//!
//! This module records their explicitly acknowledged outcomes.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Resource lease lifecycle states.
//! - Resource lease lifecycle transitions.
//! - Reservation transition validation.
//! - Acquisition transition validation.
//! - Release transition validation.
//! - Failed-operation handling.
//! - Uncertain-operation reconciliation.
//! - Resource cleanup requirements.
//! - Resource lease lifecycle invariants.
//!
//! This module does NOT own:
//!
//! - Resource-intent semantics.
//! - Requirement evaluation.
//! - Capability discovery.
//! - Preference evaluation.
//! - Constraint evaluation.
//! - Feasibility analysis.
//! - Resource planning.
//! - Hardware discovery.
//! - Hardware selection.
//! - Physical resource allocation.
//! - Resource provider registration.
//! - Resource provider discovery.
//! - Provider implementation.
//! - Resource accounting.
//! - Scheduling.
//! - Placement.
//! - Routing.
//! - Execution.
//! - Distributed membership.
//! - Domain-specific resource semantics.
//!
//! These responsibilities remain with their existing authorities.
//!
//! ## CANONICAL INTEGRATION
//!
//! Resource identity and quantity:
//!
//!     crate::zuir::ir::resources
//!
//! Resource lease identity:
//!
//!     crate::zur::ids::ResourceLeaseId
//!
//! Resource errors:
//!
//!     crate::zur::error
//!
//! Runtime operation context and cancellation:
//!
//!     crate::stdlib::context
//!
//! Resource allocation:
//!
//!     crate::zur::memory
//!
//! Runtime lifecycle:
//!
//!     crate::zur::core::lifecycle
//!
//! Runtime service lifecycle:
//!
//!     crate::zur::core::service_manager
//!
//! ## IMPORTANT DISTINCTIONS
//!
//!     Resource intent != Resource reservation
//!     Resource reservation != Resource acquisition
//!     Resource acquisition != Resource execution
//!     Resource release request != Resource release completion
//!     Operation timeout != Operation failure
//!     Provider registration != Resource availability
//!     Feasibility != Resource reservation
//!
//! A lifecycle transition is recorded only after the responsible
//! resource manager or provider explicitly acknowledges its outcome.
//!
//! ## FAILURE HANDLING
//!
//! External resource operations may have uncertain outcomes.
//!
//! For example, a remote provider may successfully reserve a resource
//! but lose its response before the caller receives confirmation.
//!
//! Such an operation MUST NOT be treated as definitively unsuccessful.
//!
//! OutcomeUnknown preserves the pending operation until an authoritative
//! reconciliation result is supplied.
//!
//! The lease identity serves as a stable operation correlation identity.
//!
//! Providers should make reservation, acquisition and release operations
//! idempotent for a given lease identity.
//!
//! ## CLEANUP
//!
//! Successfully reserved or acquired resources require explicit release.
//!
//! A failed acquisition may still leave a successfully reserved resource.
//!
//! A failed release retains cleanup responsibility.
//!
//! This module never claims release merely because release was requested.
//!
//! Release completion must be explicitly acknowledged.
//!
//! The resource manager must retain failed leases for retry or recovery.
//!
//! ## GRAMMAR INTEGRATION
//!
//! Relevant authorities:
//!
//! - grammar/resources/resources.g4
//! - grammar/resources/requirements.g4
//! - grammar/resources/resource.g4
//! - grammar/resources/resource-expressions.g4
//! - grammar/resources/capabilities.g4
//! - grammar/execution/runtime.g4
//! - grammar/execution/runtime-capabilities.g4
//! - grammar/spec/resources.md
//!
//! Source-level resource operations include:
//!
//!     reserve resource_expression;
//!     acquire resource_expression;
//!     release resource_expression;
//!
//! These declarations express intent.
//!
//! The frontend and semantic authority validate their meaning.
//!
//! The compiler and resource-planning subsystems produce validated
//! resource requests.
//!
//! The resource manager coordinates actual provider operations.
//!
//! This module receives canonical, validated resource identities and
//! quantities. It does not parse grammar or reinterpret source intent.
//!
//! ## SCALABILITY
//!
//! No artificial maximum is imposed on:
//!
//! - Resource categories.
//! - Resource identities.
//! - Resource quantities.
//! - Lease identities.
//! - Providers.
//! - Devices.
//! - CPUs.
//! - GPUs.
//! - QPUs.
//! - FPGAs.
//! - Distributed participants.
//! - Computational domains.
//!
//! Resource names and quantities reuse the canonical ZUIR representation.
//!
//! This module contains no fixed hardware inventory.
//!
//! The lifecycle state count is finite because lifecycle phases are
//! finite concepts, not because computational resources are limited.
//!
//! Actual operations remain bounded by available resources, memory,
//! address space, provider capabilities and operational policies.
//!
//! Literal infinite physical resources are not promised.
//!
//! ## CONCURRENCY
//!
//! This coordinator is intentionally not internally synchronized.
//!
//! Each lifecycle instance requires exclusive mutable access for
//! state transitions.
//!
//! A resource manager sharing lifecycle instances across threads
//! must synchronize access at its ownership boundary.
//!
//! External provider callbacks must never be executed while holding
//! a lifecycle-manager lock.
//!
//! ## FREEZE CRITERIA
//!
//! - Canonical resource identities are reused.
//! - Canonical resource quantities are reused.
//! - Lease identities are reused.
//! - Reservation and acquisition remain distinct.
//! - Release requires explicit acknowledgement.
//! - Uncertain operations remain distinguishable from failures.
//! - Failed cleanup remains retryable.
//! - No hardware discovery is introduced.
//! - No allocation implementation is duplicated.
//! - No unsafe Rust is introduced.
//! - No global mutable state is introduced.
//! - All lifecycle transitions are tested.
//! - The resource manager consumes this stable state machine.

use core::fmt;

use crate::zur::error::{ZurError, ZurResult};
use crate::zur::ids::ResourceLeaseId;
use crate::zuir::ir::resources::{ResourceKey, ResourceQuantity};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable identity of this implementation component.
const COMPONENT: &str = "zur.resources.lifecycle";

// ============================================================================
// LIFECYCLE STATES
// ============================================================================

/// Observable state of one resource lease.
///
/// States describe the operational lifecycle of a lease.
///
/// They do not represent physical device states, hardware identities,
/// resource feasibility, or execution completion.
///
/// The enum is non-exhaustive to preserve forward compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResourceLeaseState {
    /// A validated resource request has been admitted for management.
    ///
    /// No physical resource has been reserved or acquired.
    Requested,

    /// A reservation operation has been submitted and is awaiting
    /// authoritative confirmation.
    ReservationPending,

    /// A resource reservation has been explicitly confirmed.
    ///
    /// Reservation does not imply acquisition or execution.
    Reserved,

    /// An acquisition operation has been submitted and is awaiting
    /// authoritative confirmation.
    AcquisitionPending,

    /// Resource acquisition has been explicitly confirmed.
    Acquired,

    /// A release operation is awaiting authoritative confirmation.
    ReleasePending,

    /// Resource release has been explicitly confirmed.
    Released,

    /// A lifecycle operation failed definitively.
    ///
    /// Previously confirmed reservations or acquisitions may still
    /// require cleanup.
    Failed,

    /// An external operation has an uncertain outcome.
    ///
    /// The responsible resource manager must reconcile the operation
    /// before attempting a conflicting lifecycle transition.
    OutcomeUnknown,

    /// Release failed definitively.
    ///
    /// Cleanup remains outstanding and release may be retried.
    ReleaseFailed,
}

impl ResourceLeaseState {
    /// Returns the stable textual representation of this state.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::ReservationPending => "reservation_pending",
            Self::Reserved => "reserved",
            Self::AcquisitionPending => "acquisition_pending",
            Self::Acquired => "acquired",
            Self::ReleasePending => "release_pending",
            Self::Released => "released",
            Self::Failed => "failed",
            Self::OutcomeUnknown => "outcome_unknown",
            Self::ReleaseFailed => "release_failed",
        }
    }

    /// Returns whether the resource is confirmed acquired.
    #[must_use]
    pub const fn is_acquired(self) -> bool {
        matches!(self, Self::Acquired)
    }

    /// Returns whether reservation has been confirmed.
    #[must_use]
    pub const fn is_reserved(self) -> bool {
        matches!(self, Self::Reserved)
    }

    /// Returns whether release has completed.
    #[must_use]
    pub const fn is_released(self) -> bool {
        matches!(self, Self::Released)
    }

    /// Returns whether the lifecycle requires reconciliation.
    #[must_use]
    pub const fn requires_reconciliation(self) -> bool {
        matches!(self, Self::OutcomeUnknown)
    }
}

impl fmt::Display for ResourceLeaseState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// LIFECYCLE OPERATIONS
// ============================================================================

/// Identifies an external resource operation awaiting acknowledgement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResourceLifecycleOperation {
    /// Resource reservation.
    Reservation,

    /// Resource acquisition.
    Acquisition,

    /// Resource release.
    Release,
}

impl ResourceLifecycleOperation {
    /// Returns the stable textual representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reservation => "reservation",
            Self::Acquisition => "acquisition",
            Self::Release => "release",
        }
    }
}

impl fmt::Display for ResourceLifecycleOperation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Authoritative result obtained when reconciling an uncertain operation.
///
/// This value must come from the responsible resource manager or
/// provider, not from an assumption based on a timeout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ReconciledOutcome {
    /// The external operation completed successfully.
    Applied,

    /// The external operation definitively did not take effect.
    NotApplied,
}

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

/// Constructs a structured lifecycle transition error.
fn transition_error(
    operation: &'static str,
    state: ResourceLeaseState,
    explanation: &str,
) -> ZurError {
    ZurError::invalid_input(format!(
        "resource lifecycle operation `{operation}` is invalid \
         in state `{state}`: {explanation}"
    ))
    .with_component(COMPONENT)
    .with_operation(operation)
}

/// Constructs an invalid canonical resource-data error.
fn invalid_resource_data(
    operation: &'static str,
    explanation: impl Into<String>,
) -> ZurError {
    ZurError::invalid_data(explanation)
        .with_component(COMPONENT)
        .with_operation(operation)
}

// ============================================================================
// RESOURCE LEASE LIFECYCLE
// ============================================================================

/// Lifecycle coordinator for one canonical ZUR resource lease.
///
/// This object records lifecycle transitions and externally confirmed
/// operation outcomes.
///
/// It does not perform provider calls, reserve resources, acquire
/// hardware, execute workloads, or release physical resources.
///
/// The resource manager owns those operations.
///
/// # Ownership
///
/// A lifecycle instance is associated with exactly one ResourceLeaseId.
///
/// It owns its lifecycle metadata but does not own the physical
/// resource represented by the lease.
///
/// # Thread safety
///
/// This type deliberately uses exclusive mutable access rather than
/// internal locking.
///
/// The resource manager must synchronize shared access when required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceLeaseLifecycle {
    /// Canonical lease identity.
    lease_id: ResourceLeaseId,

    /// Canonical logical resource identity.
    resource: ResourceKey,

    /// Canonical requested resource quantity.
    quantity: ResourceQuantity,

    /// Current lifecycle state.
    state: ResourceLeaseState,

    /// External operation awaiting acknowledgement or reconciliation.
    pending_operation: Option<ResourceLifecycleOperation>,

    /// Whether reservation has been explicitly confirmed.
    reservation_confirmed: bool,

    /// Whether acquisition has been explicitly confirmed.
    acquisition_confirmed: bool,
}

impl ResourceLeaseLifecycle {
    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    /// Creates a lifecycle coordinator for an existing resource lease.
    ///
    /// The caller must supply the canonical lease identity and a
    /// semantically validated resource identity and quantity.
    ///
    /// This operation performs local representation validation only.
    ///
    /// It does not establish resource availability or feasibility.
    ///
    /// # Errors
    ///
    /// Returns an error when the supplied canonical resource data
    /// fails validation.
    pub fn new(
        lease_id: ResourceLeaseId,
        resource: ResourceKey,
        quantity: ResourceQuantity,
    ) -> ZurResult<Self> {
        resource.validate().map_err(|error| {
            invalid_resource_data(
                "construct",
                format!("invalid resource identity: {error}"),
            )
        })?;

        quantity.validate().map_err(|error| {
            invalid_resource_data(
                "construct",
                format!("invalid resource quantity: {error}"),
            )
        })?;

        Ok(Self {
            lease_id,
            resource,
            quantity,
            state: ResourceLeaseState::Requested,
            pending_operation: None,
            reservation_confirmed: false,
            acquisition_confirmed: false,
        })
    }

    // ------------------------------------------------------------------------
    // IDENTITY AND METADATA
    // ------------------------------------------------------------------------

    /// Returns the canonical resource lease identity.
    #[must_use]
    pub fn lease_id(&self) -> &ResourceLeaseId {
        &self.lease_id
    }

    /// Returns the logical resource identity.
    ///
    /// This is not a physical device identity.
    #[must_use]
    pub fn resource(&self) -> &ResourceKey {
        &self.resource
    }

    /// Returns the canonical requested resource quantity.
    #[must_use]
    pub fn quantity(&self) -> &ResourceQuantity {
        &self.quantity
    }

    /// Returns the current lifecycle state.
    #[must_use]
    pub const fn state(&self) -> ResourceLeaseState {
        self.state
    }

    /// Returns the operation awaiting acknowledgement, if any.
    #[must_use]
    pub const fn pending_operation(
        &self,
    ) -> Option<ResourceLifecycleOperation> {
        self.pending_operation
    }

    /// Returns whether reservation has been explicitly confirmed.
    #[must_use]
    pub const fn reservation_confirmed(&self) -> bool {
        self.reservation_confirmed
    }

    /// Returns whether acquisition has been explicitly confirmed.
    #[must_use]
    pub const fn acquisition_confirmed(&self) -> bool {
        self.acquisition_confirmed
    }

    /// Returns whether the lease has completed release.
    #[must_use]
    pub const fn is_released(&self) -> bool {
        self.state.is_released()
    }

    /// Returns whether cleanup is still required.
    ///
    /// An uncertain external operation also requires reconciliation
    /// or an explicitly idempotent cleanup procedure.
    #[must_use]
    pub const fn cleanup_required(&self) -> bool {
        self.reservation_confirmed
            || self.acquisition_confirmed
            || matches!(
                self.state,
                ResourceLeaseState::OutcomeUnknown
                    | ResourceLeaseState::ReleaseFailed
                    | ResourceLeaseState::ReleasePending
            )
    }

    /// Returns whether the lifecycle is terminal without outstanding
    /// cleanup or reconciliation.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(self.state, ResourceLeaseState::Released)
            || (matches!(self.state, ResourceLeaseState::Failed)
                && !self.cleanup_required())
    }

    // ------------------------------------------------------------------------
    // RESERVATION
    // ------------------------------------------------------------------------

    /// Begins a resource reservation operation.
    ///
    /// The resource manager must submit the actual reservation request
    /// using the canonical lease identity.
    ///
    /// A successful return means only that the lifecycle transition
    /// was accepted. It does not mean the resource was reserved.
    ///
    /// # Errors
    ///
    /// Returns an error unless the lease is Requested.
    pub fn begin_reservation(&mut self) -> ZurResult<()> {
        self.require_state(
            "begin_reservation",
            ResourceLeaseState::Requested,
        )?;

        self.state = ResourceLeaseState::ReservationPending;

        self.pending_operation =
            Some(ResourceLifecycleOperation::Reservation);

        Ok(())
    }

    /// Records successful external reservation confirmation.
    ///
    /// Call this only after the resource manager or provider has
    /// authoritatively confirmed that the reservation exists.
    ///
    /// # Errors
    ///
    /// Returns an error unless reservation is pending.
    pub fn confirm_reservation(&mut self) -> ZurResult<()> {
        self.require_state(
            "confirm_reservation",
            ResourceLeaseState::ReservationPending,
        )?;

        self.reservation_confirmed = true;
        self.pending_operation = None;
        self.state = ResourceLeaseState::Reserved;

        Ok(())
    }

    /// Records definitive reservation rejection.
    ///
    /// This operation must not be used for timeouts, lost responses
    /// or other situations where the provider may have reserved the
    /// resource successfully.
    ///
    /// # Errors
    ///
    /// Returns an error unless reservation is pending.
    pub fn reject_reservation(&mut self) -> ZurResult<()> {
        self.require_pending_operation(
            "reject_reservation",
            ResourceLifecycleOperation::Reservation,
        )?;

        self.pending_operation = None;
        self.state = ResourceLeaseState::Failed;

        Ok(())
    }

    // ------------------------------------------------------------------------
    // ACQUISITION
    // ------------------------------------------------------------------------

    /// Begins resource acquisition.
    ///
    /// Acquisition may proceed directly from Requested or after a
    /// confirmed reservation.
    ///
    /// The resource manager performs the actual provider operation.
    ///
    /// # Errors
    ///
    /// Returns an error if the lease is not Requested or Reserved.
    pub fn begin_acquisition(&mut self) -> ZurResult<()> {
        match self.state {
            ResourceLeaseState::Requested
            | ResourceLeaseState::Reserved => {}

            state => {
                return Err(transition_error(
                    "begin_acquisition",
                    state,
                    "acquisition requires Requested or Reserved state",
                ));
            }
        }

        self.state = ResourceLeaseState::AcquisitionPending;

        self.pending_operation =
            Some(ResourceLifecycleOperation::Acquisition);

        Ok(())
    }

    /// Records successful external acquisition confirmation.
    ///
    /// Call this only after the responsible provider has confirmed
    /// acquisition of the requested resource.
    ///
    /// Acquisition does not imply that execution has begun.
    ///
    /// # Errors
    ///
    /// Returns an error unless acquisition is pending.
    pub fn confirm_acquisition(&mut self) -> ZurResult<()> {
        self.require_state(
            "confirm_acquisition",
            ResourceLeaseState::AcquisitionPending,
        )?;

        self.acquisition_confirmed = true;
        self.pending_operation = None;
        self.state = ResourceLeaseState::Acquired;

        Ok(())
    }

    /// Records definitive acquisition rejection.
    ///
    /// If reservation was previously confirmed, that reservation
    /// remains recorded and requires explicit release.
    ///
    /// # Errors
    ///
    /// Returns an error unless acquisition is pending.
    pub fn reject_acquisition(&mut self) -> ZurResult<()> {
        self.require_pending_operation(
            "reject_acquisition",
            ResourceLifecycleOperation::Acquisition,
        )?;

        self.pending_operation = None;

        self.state = if self.reservation_confirmed {
            ResourceLeaseState::Reserved
        } else {
            ResourceLeaseState::Failed
        };

        Ok(())
    }

    // ------------------------------------------------------------------------
    // EXPLICIT FAILURE
    // ------------------------------------------------------------------------

    /// Records a definitive lifecycle failure outside a pending
    /// external operation.
    ///
    /// Existing confirmed reservations and acquisitions remain
    /// recorded so the resource manager can perform cleanup.
    ///
    /// An outstanding operation must first be acknowledged or
    /// reconciled; this method never converts uncertainty into failure.
    ///
    /// # Errors
    ///
    /// Returns an error when an external operation is pending,
    /// its outcome is unknown, or release has already completed.
    pub fn mark_failed(&mut self) -> ZurResult<()> {
        if self.pending_operation.is_some()
            || self.state == ResourceLeaseState::OutcomeUnknown
        {
            return Err(transition_error(
                "mark_failed",
                self.state,
                "pending or uncertain external operations must be reconciled",
            ));
        }

        if self.state == ResourceLeaseState::Released {
            return Err(transition_error(
                "mark_failed",
                self.state,
                "a released lease cannot become failed",
            ));
        }

        if self.state == ResourceLeaseState::ReleaseFailed {
            return Err(transition_error(
                "mark_failed",
                self.state,
                "release failure must remain retryable",
            ));
        }

        self.state = ResourceLeaseState::Failed;

        Ok(())
    }

    // ------------------------------------------------------------------------
    // RELEASE
    // ------------------------------------------------------------------------

    /// Begins resource release or cleanup.
    ///
    /// Release may follow:
    ///
    /// - A confirmed reservation.
    /// - A confirmed acquisition.
    /// - A failed acquisition that retained a reservation.
    /// - Another definitive failure with outstanding confirmed resources.
    /// - A previous definitive release failure.
    ///
    /// Release is performed by the resource manager and provider.
    ///
    /// This transition does not claim that release has completed.
    ///
    /// # Errors
    ///
    /// Returns an error if there is no outstanding resource to release,
    /// an operation remains uncertain, or release has already completed.
    pub fn begin_release(&mut self) -> ZurResult<()> {
        match self.state {
            ResourceLeaseState::Reserved
            | ResourceLeaseState::Acquired
            | ResourceLeaseState::ReleaseFailed => {}

            ResourceLeaseState::Failed
                if self.cleanup_required() => {}

            ResourceLeaseState::OutcomeUnknown => {
                return Err(transition_error(
                    "begin_release",
                    self.state,
                    "reconcile the uncertain external operation first",
                ));
            }

            state => {
                return Err(transition_error(
                    "begin_release",
                    state,
                    "no confirmed or retryable resource is available for release",
                ));
            }
        }

        if !self.cleanup_required() {
            return Err(transition_error(
                "begin_release",
                self.state,
                "there is no confirmed or uncertain resource to release",
            ));
        }

        self.state = ResourceLeaseState::ReleasePending;

        self.pending_operation =
            Some(ResourceLifecycleOperation::Release);

        Ok(())
    }

    /// Records successful external release confirmation.
    ///
    /// This operation clears the confirmed reservation and acquisition
    /// records only after the responsible provider confirms release.
    ///
    /// # Errors
    ///
    /// Returns an error unless release is pending.
    pub fn confirm_release(&mut self) -> ZurResult<()> {
        self.require_state(
            "confirm_release",
            ResourceLeaseState::ReleasePending,
        )?;

        self.reservation_confirmed = false;
        self.acquisition_confirmed = false;

        self.pending_operation = None;
        self.state = ResourceLeaseState::Released;

        Ok(())
    }

    /// Records definitive release failure.
    ///
    /// The lease remains eligible for another release attempt.
    ///
    /// A timeout or lost response must instead use
    /// [`Self::mark_outcome_unknown`].
    ///
    /// # Errors
    ///
    /// Returns an error unless release is pending.
    pub fn reject_release(&mut self) -> ZurResult<()> {
        self.require_pending_operation(
            "reject_release",
            ResourceLifecycleOperation::Release,
        )?;

        self.pending_operation = None;
        self.state = ResourceLeaseState::ReleaseFailed;

        Ok(())
    }

    // ------------------------------------------------------------------------
    // UNCERTAIN EXTERNAL OPERATIONS
    // ------------------------------------------------------------------------

    /// Records that an external operation has an uncertain outcome.
    ///
    /// Examples include:
    ///
    /// - A remote reservation timed out.
    /// - A provider connection failed after submission.
    /// - A release response was lost.
    /// - A device-management acknowledgement was interrupted.
    ///
    /// The pending operation is preserved.
    ///
    /// The resource manager must reconcile the outcome using the
    /// canonical lease identity before proceeding.
    ///
    /// # Errors
    ///
    /// Returns an error unless an external operation is pending.
    pub fn mark_outcome_unknown(&mut self) -> ZurResult<()> {
        if self.pending_operation.is_none() {
            return Err(transition_error(
                "mark_outcome_unknown",
                self.state,
                "no external operation is awaiting acknowledgement",
            ));
        }

        self.state = ResourceLeaseState::OutcomeUnknown;

        Ok(())
    }

    /// Resolves an uncertain external operation using authoritative
    /// provider or resource-manager evidence.
    ///
    /// Applied:
    ///
    /// - Reservation becomes Reserved.
    /// - Acquisition becomes Acquired.
    /// - Release becomes Released.
    ///
    /// NotApplied:
    ///
    /// - Reservation becomes Failed.
    /// - Acquisition returns to Reserved when a reservation existed,
    ///   otherwise Failed.
    /// - Release becomes ReleaseFailed and remains retryable.
    ///
    /// # Errors
    ///
    /// Returns an error unless the lease is OutcomeUnknown and has
    /// a preserved pending operation.
    pub fn resolve_unknown(
        &mut self,
        outcome: ReconciledOutcome,
    ) -> ZurResult<()> {
        if self.state != ResourceLeaseState::OutcomeUnknown {
            return Err(transition_error(
                "resolve_unknown",
                self.state,
                "only uncertain operations can be reconciled",
            ));
        }

        let operation = self.pending_operation.ok_or_else(|| {
            transition_error(
                "resolve_unknown",
                self.state,
                "uncertain state has no pending operation",
            )
        })?;

        match (operation, outcome) {
            (
                ResourceLifecycleOperation::Reservation,
                ReconciledOutcome::Applied,
            ) => {
                self.reservation_confirmed = true;
                self.state = ResourceLeaseState::Reserved;
            }

            (
                ResourceLifecycleOperation::Reservation,
                ReconciledOutcome::NotApplied,
            ) => {
                self.state = ResourceLeaseState::Failed;
            }

            (
                ResourceLifecycleOperation::Acquisition,
                ReconciledOutcome::Applied,
            ) => {
                self.acquisition_confirmed = true;
                self.state = ResourceLeaseState::Acquired;
            }

            (
                ResourceLifecycleOperation::Acquisition,
                ReconciledOutcome::NotApplied,
            ) => {
                self.state = if self.reservation_confirmed {
                    ResourceLeaseState::Reserved
                } else {
                    ResourceLeaseState::Failed
                };
            }

            (
                ResourceLifecycleOperation::Release,
                ReconciledOutcome::Applied,
            ) => {
                self.reservation_confirmed = false;
                self.acquisition_confirmed = false;
                self.state = ResourceLeaseState::Released;
            }

            (
                ResourceLifecycleOperation::Release,
                ReconciledOutcome::NotApplied,
            ) => {
                self.state = ResourceLeaseState::ReleaseFailed;
            }
        }

        self.pending_operation = None;

        Ok(())
    }

    // ------------------------------------------------------------------------
    // INTERNAL VALIDATION
    // ------------------------------------------------------------------------

    /// Requires the specified lifecycle state.
    fn require_state(
        &self,
        operation: &'static str,
        expected: ResourceLeaseState,
    ) -> ZurResult<()> {
        if self.state == expected {
            return Ok(());
        }

        Err(transition_error(
            operation,
            self.state,
            "the required lifecycle state has not been reached",
        ))
    }

    /// Requires the specified external operation to be pending.
    fn require_pending_operation(
        &self,
        operation: &'static str,
        expected: ResourceLifecycleOperation,
    ) -> ZurResult<()> {
        if self.state != ResourceLeaseState::OutcomeUnknown
            && self.pending_operation == Some(expected)
        {
            return Ok(());
        }

        Err(transition_error(
            operation,
            self.state,
            "the expected external operation is not awaiting acknowledgement",
        ))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::ids::IdNamespace;

    fn lifecycle() -> ResourceLeaseLifecycle {
        let namespace = IdNamespace::new(
            "zamani.test.resource.lease",
        )
        .expect("valid identity namespace");

        let lease_id = ResourceLeaseId::new(
            namespace,
            vec![1],
        )
        .expect("valid lease identity");

        let resource = ResourceKey::new(
            "quantum.logical_qubit",
        )
        .expect("valid resource key");

        let quantity = ResourceQuantity::parse(
            "128",
            "count",
        )
        .expect("valid resource quantity");

        ResourceLeaseLifecycle::new(
            lease_id,
            resource,
            quantity,
        )
        .expect("valid resource lifecycle")
    }

    // ------------------------------------------------------------------------
    // CONSTRUCTION
    // ------------------------------------------------------------------------

    #[test]
    fn starts_in_requested_state() {
        let lease = lifecycle();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Requested,
        );

        assert!(!lease.cleanup_required());
        assert!(!lease.is_released());
        assert!(!lease.is_terminal());
    }

    #[test]
    fn preserves_canonical_resource_metadata() {
        let lease = lifecycle();

        assert_eq!(
            lease.resource().as_str(),
            "quantum.logical_qubit",
        );

        assert_eq!(
            lease.quantity().magnitude().as_str(),
            "128",
        );

        assert_eq!(
            lease.quantity().unit().as_str(),
            "count",
        );
    }

    // ------------------------------------------------------------------------
    // RESERVATION
    // ------------------------------------------------------------------------

    #[test]
    fn reservation_requires_explicit_confirmation() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::ReservationPending,
        );

        assert!(!lease.reservation_confirmed());
        assert!(!lease.cleanup_required());

        lease.confirm_reservation().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Reserved,
        );

        assert!(lease.reservation_confirmed());
        assert!(lease.cleanup_required());
    }

    #[test]
    fn rejects_definitively_failed_reservation() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.reject_reservation().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Failed,
        );

        assert!(!lease.cleanup_required());
        assert!(lease.is_terminal());
    }

    #[test]
    fn cannot_reserve_twice() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.confirm_reservation().unwrap();

        assert!(lease.begin_reservation().is_err());
    }

    // ------------------------------------------------------------------------
    // ACQUISITION
    // ------------------------------------------------------------------------

    #[test]
    fn supports_direct_acquisition() {
        let mut lease = lifecycle();

        lease.begin_acquisition().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::AcquisitionPending,
        );

        lease.confirm_acquisition().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Acquired,
        );

        assert!(lease.acquisition_confirmed());
        assert!(lease.cleanup_required());
    }

    #[test]
    fn supports_acquisition_after_reservation() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.confirm_reservation().unwrap();

        lease.begin_acquisition().unwrap();
        lease.confirm_acquisition().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Acquired,
        );

        assert!(lease.reservation_confirmed());
        assert!(lease.acquisition_confirmed());
    }

    #[test]
    fn failed_acquisition_preserves_reservation() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.confirm_reservation().unwrap();

        lease.begin_acquisition().unwrap();
        lease.reject_acquisition().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Reserved,
        );

        assert!(lease.reservation_confirmed());
        assert!(!lease.acquisition_confirmed());
        assert!(lease.cleanup_required());
    }

    #[test]
    fn failed_direct_acquisition_requires_no_false_cleanup() {
        let mut lease = lifecycle();

        lease.begin_acquisition().unwrap();
        lease.reject_acquisition().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Failed,
        );

        assert!(!lease.cleanup_required());
    }

    // ------------------------------------------------------------------------
    // RELEASE
    // ------------------------------------------------------------------------

    #[test]
    fn releases_reserved_resource() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.confirm_reservation().unwrap();

        lease.begin_release().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::ReleasePending,
        );

        assert!(lease.cleanup_required());

        lease.confirm_release().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Released,
        );

        assert!(!lease.reservation_confirmed());
        assert!(!lease.acquisition_confirmed());
        assert!(!lease.cleanup_required());
        assert!(lease.is_terminal());
    }

    #[test]
    fn releases_acquired_resource() {
        let mut lease = lifecycle();

        lease.begin_acquisition().unwrap();
        lease.confirm_acquisition().unwrap();

        lease.begin_release().unwrap();
        lease.confirm_release().unwrap();

        assert!(lease.is_released());
        assert!(lease.is_terminal());
    }

    #[test]
    fn failed_release_remains_retryable() {
        let mut lease = lifecycle();

        lease.begin_acquisition().unwrap();
        lease.confirm_acquisition().unwrap();

        lease.begin_release().unwrap();
        lease.reject_release().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::ReleaseFailed,
        );

        assert!(lease.cleanup_required());
        assert!(!lease.is_terminal());

        lease.begin_release().unwrap();
        lease.confirm_release().unwrap();

        assert!(lease.is_released());
        assert!(!lease.cleanup_required());
    }

    #[test]
    fn cannot_release_unacquired_unreserved_resource() {
        let mut lease = lifecycle();

        assert!(lease.begin_release().is_err());
    }

    #[test]
    fn cannot_release_twice() {
        let mut lease = lifecycle();

        lease.begin_acquisition().unwrap();
        lease.confirm_acquisition().unwrap();

        lease.begin_release().unwrap();
        lease.confirm_release().unwrap();

        assert!(lease.begin_release().is_err());
    }

    // ------------------------------------------------------------------------
    // FAILURE
    // ------------------------------------------------------------------------

    #[test]
    fn explicit_failure_preserves_acquired_resource() {
        let mut lease = lifecycle();

        lease.begin_acquisition().unwrap();
        lease.confirm_acquisition().unwrap();

        lease.mark_failed().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Failed,
        );

        assert!(lease.acquisition_confirmed());
        assert!(lease.cleanup_required());

        lease.begin_release().unwrap();
        lease.confirm_release().unwrap();

        assert!(lease.is_released());
    }

    #[test]
    fn cannot_fail_an_uncertain_operation() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.mark_outcome_unknown().unwrap();

        assert!(lease.mark_failed().is_err());
    }

    #[test]
    fn cannot_fail_a_released_lease() {
        let mut lease = lifecycle();

        lease.begin_acquisition().unwrap();
        lease.confirm_acquisition().unwrap();

        lease.begin_release().unwrap();
        lease.confirm_release().unwrap();

        assert!(lease.mark_failed().is_err());
    }

    // ------------------------------------------------------------------------
    // UNCERTAIN OPERATIONS
    // ------------------------------------------------------------------------

    #[test]
    fn uncertain_reservation_can_be_reconciled_as_applied() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.mark_outcome_unknown().unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::OutcomeUnknown,
        );

        assert_eq!(
            lease.pending_operation(),
            Some(ResourceLifecycleOperation::Reservation),
        );

        lease.resolve_unknown(
            ReconciledOutcome::Applied,
        ).unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Reserved,
        );

        assert!(lease.reservation_confirmed());
        assert!(lease.cleanup_required());
    }

    #[test]
    fn uncertain_reservation_can_be_reconciled_as_not_applied() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.mark_outcome_unknown().unwrap();

        lease.resolve_unknown(
            ReconciledOutcome::NotApplied,
        ).unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Failed,
        );

        assert!(!lease.cleanup_required());
    }

    #[test]
    fn uncertain_acquisition_preserves_existing_reservation() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.confirm_reservation().unwrap();

        lease.begin_acquisition().unwrap();
        lease.mark_outcome_unknown().unwrap();

        lease.resolve_unknown(
            ReconciledOutcome::NotApplied,
        ).unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Reserved,
        );

        assert!(lease.reservation_confirmed());
        assert!(!lease.acquisition_confirmed());
    }

    #[test]
    fn uncertain_acquisition_can_be_reconciled_as_applied() {
        let mut lease = lifecycle();

        lease.begin_acquisition().unwrap();
        lease.mark_outcome_unknown().unwrap();

        lease.resolve_unknown(
            ReconciledOutcome::Applied,
        ).unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::Acquired,
        );

        assert!(lease.acquisition_confirmed());
    }

    #[test]
    fn uncertain_release_can_be_reconciled_as_applied() {
        let mut lease = lifecycle();

        lease.begin_acquisition().unwrap();
        lease.confirm_acquisition().unwrap();

        lease.begin_release().unwrap();
        lease.mark_outcome_unknown().unwrap();

        lease.resolve_unknown(
            ReconciledOutcome::Applied,
        ).unwrap();

        assert!(lease.is_released());
        assert!(!lease.cleanup_required());
    }

    #[test]
    fn uncertain_release_can_be_reconciled_as_not_applied() {
        let mut lease = lifecycle();

        lease.begin_acquisition().unwrap();
        lease.confirm_acquisition().unwrap();

        lease.begin_release().unwrap();
        lease.mark_outcome_unknown().unwrap();

        lease.resolve_unknown(
            ReconciledOutcome::NotApplied,
        ).unwrap();

        assert_eq!(
            lease.state(),
            ResourceLeaseState::ReleaseFailed,
        );

        assert!(lease.cleanup_required());
    }

    #[test]
    fn cannot_resolve_an_operation_twice() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.mark_outcome_unknown().unwrap();

        lease.resolve_unknown(
            ReconciledOutcome::Applied,
        ).unwrap();

        assert!(
            lease.resolve_unknown(
                ReconciledOutcome::Applied,
            ).is_err()
        );
    }

    // ------------------------------------------------------------------------
    // TRANSITION INVARIANTS
    // ------------------------------------------------------------------------

    #[test]
    fn cannot_confirm_reservation_without_submission() {
        let mut lease = lifecycle();

        assert!(lease.confirm_reservation().is_err());
    }

    #[test]
    fn cannot_confirm_acquisition_without_submission() {
        let mut lease = lifecycle();

        assert!(lease.confirm_acquisition().is_err());
    }

    #[test]
    fn cannot_confirm_release_without_submission() {
        let mut lease = lifecycle();

        assert!(lease.confirm_release().is_err());
    }

    #[test]
    fn pending_operations_must_be_reconciled() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.mark_outcome_unknown().unwrap();

        assert!(lease.begin_acquisition().is_err());
        assert!(lease.begin_release().is_err());
        assert!(lease.reject_reservation().is_err());
    }

    #[test]
    fn failed_reservation_does_not_create_fake_resource() {
        let mut lease = lifecycle();

        lease.begin_reservation().unwrap();
        lease.reject_reservation().unwrap();

        assert!(!lease.reservation_confirmed());
        assert!(!lease.acquisition_confirmed());
        assert!(!lease.cleanup_required());
    }

    #[test]
    fn preserves_lease_identity_through_lifecycle() {
        let mut lease = lifecycle();

        let original = lease.lease_id().clone();

        lease.begin_acquisition().unwrap();
        lease.confirm_acquisition().unwrap();

        lease.begin_release().unwrap();
        lease.confirm_release().unwrap();

        assert_eq!(lease.lease_id(), &original);
    }

    #[test]
    fn state_names_are_stable() {
        assert_eq!(
            ResourceLeaseState::Requested.as_str(),
            "requested",
        );

        assert_eq!(
            ResourceLeaseState::Reserved.as_str(),
            "reserved",
        );

        assert_eq!(
            ResourceLeaseState::Acquired.as_str(),
            "acquired",
        );

        assert_eq!(
            ResourceLeaseState::Released.as_str(),
            "released",
        );
    }
}
