
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Physical Hardware Reservation Reconciliation
//!
//! Path: src/zhal/resources/leases.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Defines provider-facing inspection and reconciliation contracts
//! for physical hardware reservations.
//!
//! This module complements:
//!
//! - crate::zhal::resources::reservation
//! - crate::zur::resources::lease
//! - crate::zur::resources
//!
//! It allows the runtime to establish the authoritative external
//! state of a reservation when an earlier operation has an ambiguous
//! outcome.
//!
//! Examples include:
//!
//! - A reservation request was submitted but its response was lost.
//! - A renewal request timed out after reaching the provider.
//! - A release acknowledgement was lost.
//! - A remote hardware provider became temporarily unreachable.
//! - The runtime restarted while a reservation remained active.
//!
//! # ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - HardwareReservationLookup.
//! - HardwareReservationLeaseState.
//! - HardwareReservationLeaseObservation.
//! - HardwareReservationLeaseProvider.
//! - Reservation inspection validation.
//! - Provider-backed reservation reconciliation.
//!
//! This module does NOT own:
//!
//! - Source-language resource requirements.
//! - Resource AST representations.
//! - Semantic resource models.
//! - Resource feasibility.
//! - Hardware discovery.
//! - Provider registration.
//! - Provider lifecycle.
//! - Physical resource allocation policy.
//! - Runtime resource lease identities.
//! - Runtime lease state.
//! - Runtime lease expiration tracking.
//! - Runtime scheduling.
//! - Resource placement.
//! - Quantum routing.
//! - HDL realization.
//! - Distributed execution.
//! - Hardware execution.
//!
//! ZUR remains the authoritative runtime lease lifecycle owner.
//!
//! # CANONICAL REPOSITORY AUTHORITIES
//!
//! Physical reservation:
//!     crate::zhal::resources::reservation
//!
//! Physical resource identity:
//!     crate::zhal::identity
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Hardware-provider asynchronous operations:
//!     crate::zhal::providers::contract
//!
//! Hardware operation context:
//!     crate::zhal::context::ZhalContext
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Runtime resource lease lifecycle:
//!     crate::zur::resources::lease
//!
//! # FUNDAMENTAL DISTINCTIONS
//!
//! Reservation request != confirmed reservation.
//! Confirmed reservation != runtime lease.
//! Runtime lease != execution.
//! Expiration != confirmed physical release.
//! Missing response != failed physical operation.
//! Unknown outcome != confirmed absence.
//! Cancellation != confirmed remote termination.
//!
//! # RECONCILIATION
//!
//! Providers must expose an authoritative inspection mechanism
//! for reservations they support reconciling.
//!
//! A provider must distinguish:
//!
//! - Active.
//! - Released.
//! - Expired.
//! - Not found.
//! - Pending.
//! - Indeterminate.
//!
//! NotFound must only be returned when the provider can establish
//! that the requested reservation does not exist.
//!
//! If the provider cannot establish that fact, it must return
//! Indeterminate.
//!
//! A provider must never convert a communication failure,
//! expired idempotency record or unavailable database into
//! a fabricated NotFound result.
//!
//! # IDEMPOTENCY
//!
//! A reservation can be looked up by its original idempotency key.
//!
//! Providers must preserve the relationship between an idempotency
//! key and its logical reservation for the retention period defined
//! by their documented operational policy.
//!
//! If the provider can no longer establish that relationship,
//! it must return Indeterminate rather than claiming absence.
//!
//! # CONCURRENCY
//!
//! This module owns no global mutable state.
//!
//! Inspection is observational and must not create, renew or release
//! a physical reservation.
//!
//! Provider implementations own synchronization with their
//! authoritative reservation systems.
//!
//! # SECURITY
//!
//! Lookup references are descriptive identifiers, not credentials.
//!
//! They must not contain passwords, private keys, bearer tokens,
//! authentication headers or other secrets.
//!
//! Providers must independently authenticate and authorize
//! reservation inspection requests.
//!
//! # SCALABILITY
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Reservation references.
//! - Idempotency keys.
//! - Resource quantities.
//! - Hardware providers.
//! - Devices.
//! - Reservations.
//! - Computational domains.
//! - Distributed participants.
//!
//! Actual operations remain bounded by available memory,
//! provider capabilities, address-space representation and
//! explicitly configured operational budgets.
//!
//! Literal infinite physical resources are not promised.
//!
//! # GRAMMAR BOUNDARY
//!
//! Relevant grammar:
//!
//!     grammar/resources/resources.g4
//!     grammar/resources/requirements.g4
//!     grammar/statements/resource.g4
//!
//! Normative specifications:
//!
//!     grammar/spec/resources.md
//!     grammar/specification/execution-model.md
//!     grammar/specification/poco-reaf.md
//!
//! The grammar expresses reservation and acquisition intent.
//!
//! Semantic analysis establishes its meaning.
//!
//! Compiler planning evaluates feasibility.
//!
//! ZUR coordinates runtime acquisition and lease lifecycle.
//!
//! ZHAL communicates with the physical reservation authority.
//!
//! This module does not parse or reinterpret grammar productions.

// ============================================================================
// IMPORTS
// ============================================================================

use core::fmt;

use crate::stdlib::provider::ProviderId;

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::providers::contract::HardwareProviderFuture;

use crate::zhal::resources::reservation::{
    HardwareReservationGrant,
    HardwareReservationProviderIdentity,
    HardwareReservationReference,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable component identity.
pub const COMPONENT: &str = "zhal.resources.leases";

/// Reservation reconciliation schema version.
///
/// This is independent of the Zamani package version,
/// language version, provider API version and firmware version.
pub const LEASE_SCHEMA_VERSION: u16 = 1;

/// Returns the reconciliation schema version.
#[must_use]
pub const fn schema_version() -> u16 {
    LEASE_SCHEMA_VERSION
}

// ============================================================================
// VALIDATION
// ============================================================================

fn validate_reference(
    value: &str,
    field: &'static str,
) -> ZhalResult<()> {
    if value.is_empty() {
        return Err(
            ZhalError::new(
                ZhalErrorKind::InvalidInput,
                format!("{field} must not be empty"),
            )
            .in_component(COMPONENT),
        );
    }

    if value.trim() != value {
        return Err(
            ZhalError::new(
                ZhalErrorKind::InvalidInput,
                format!(
                    "{field} must not contain surrounding whitespace"
                ),
            )
            .in_component(COMPONENT),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ZhalError::new(
                ZhalErrorKind::InvalidInput,
                format!(
                    "{field} must not contain control characters"
                ),
            )
            .in_component(COMPONENT),
        );
    }

    Ok(())
}

fn contract_violation(
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(
        ZhalErrorKind::ContractViolation,
        message,
    )
    .in_component(COMPONENT)
}

/// Checks whether an inspection may begin.
///
/// This delegates cancellation and deadline ownership to the
/// existing ZHAL context.
///
/// It does not create another cancellation mechanism.
fn check_active(
    context: &ZhalContext,
) -> ZhalResult<()> {
    if context.is_cancelled() {
        return Err(
            ZhalError::new(
                ZhalErrorKind::Cancelled,
                "reservation inspection was cancelled before dispatch",
            )
            .in_component(COMPONENT),
        );
    }

    if context.is_deadline_expired() {
        return Err(
            ZhalError::new(
                ZhalErrorKind::OperationTimedOut,
                "reservation inspection deadline expired before dispatch",
            )
            .in_component(COMPONENT),
        );
    }

    Ok(())
}

// ============================================================================
// RESERVATION LOOKUP
// ============================================================================

/// Identifies an existing physical reservation for inspection.
///
/// A lookup reference is not a reservation and does not authorize
/// physical resource access.
///
/// Two lookup forms are supported:
///
/// - The original caller-generated idempotency key.
/// - The provider-issued reservation reference.
///
/// The idempotency key is particularly important when a reservation
/// response was lost before the caller received its grant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HardwareReservationLookup {
    /// Looks up the logical reservation request by its original key.
    IdempotencyKey(String),

    /// Looks up the reservation using its provider-issued reference.
    ProviderReference(HardwareReservationReference),
}

impl HardwareReservationLookup {
    /// Constructs a lookup using the original idempotency key.
    ///
    /// The key must be identical to the key used for the original
    /// reservation request.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty, whitespace-padded or
    /// control-character-containing key.
    pub fn by_idempotency_key(
        key: impl Into<String>,
    ) -> ZhalResult<Self> {
        let key = key.into();

        validate_reference(
            &key,
            "reservation idempotency key",
        )?;

        Ok(Self::IdempotencyKey(key))
    }

    /// Constructs a lookup using the provider reservation reference.
    #[must_use]
    pub fn by_provider_reference(
        reference: HardwareReservationReference,
    ) -> Self {
        Self::ProviderReference(reference)
    }

    /// Returns the idempotency key when this is an idempotency lookup.
    #[must_use]
    pub fn idempotency_key(&self) -> Option<&str> {
        match self {
            Self::IdempotencyKey(value) => Some(value),
            Self::ProviderReference(_) => None,
        }
    }

    /// Returns the provider reference when this is a reference lookup.
    #[must_use]
    pub fn provider_reference(
        &self,
    ) -> Option<&HardwareReservationReference> {
        match self {
            Self::IdempotencyKey(_) => None,
            Self::ProviderReference(value) => Some(value),
        }
    }
}

// ============================================================================
// AUTHORITATIVE RESERVATION STATE
// ============================================================================

/// Authoritative state reported by a physical reservation provider.
///
/// This describes the state established by the provider's
/// reservation authority at inspection time.
///
/// It is not a locally maintained runtime lease state.
///
/// # Important
///
/// `NotFound` is a definitive provider assertion.
///
/// `Indeterminate` means that the provider cannot currently
/// establish the reservation's state.
///
/// These states must never be conflated.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HardwareReservationLeaseState {
    /// The physical reservation is currently confirmed active.
    ///
    /// The grant must identify the provider, resource, quantity
    /// and reservation reference.
    Active(HardwareReservationGrant),

    /// The provider confirms that the reservation was released.
    Released,

    /// The provider confirms that the reservation expired.
    ///
    /// Expiration does not necessarily prove that every external
    /// cleanup action has completed.
    Expired,

    /// The provider authoritatively confirms that no reservation
    /// exists for the requested lookup.
    NotFound,

    /// The provider confirms that the reservation operation is
    /// still being processed.
    Pending,

    /// The provider cannot currently establish the reservation state.
    ///
    /// The runtime must not treat this state as active, released
    /// or absent.
    Indeterminate,
}

// ============================================================================
// RESERVATION OBSERVATION
// ============================================================================

/// Immutable observation returned by a reservation provider.
///
/// The lookup is echoed in the result so that callers can verify
/// that the provider answered the intended query.
///
/// Observations are point-in-time provider responses. They must
/// not be cached indefinitely or treated as proof of future state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HardwareReservationLeaseObservation {
    lookup: HardwareReservationLookup,
    state: HardwareReservationLeaseState,
}

impl HardwareReservationLeaseObservation {
    /// Constructs an observation from an authoritative provider response.
    ///
    /// Provider implementations should use this constructor when
    /// translating their native reservation status.
    ///
    /// When looking up by provider reference, an active grant must
    /// contain that exact reference.
    ///
    /// # Errors
    ///
    /// Returns a contract violation if the active grant does not
    /// correspond to the requested provider reference.
    pub fn new(
        lookup: HardwareReservationLookup,
        state: HardwareReservationLeaseState,
    ) -> ZhalResult<Self> {
        if let (
            HardwareReservationLookup::ProviderReference(expected),
            HardwareReservationLeaseState::Active(grant),
        ) = (&lookup, &state)
        {
            if grant.reference() != expected {
                return Err(
                    contract_violation(
                        "provider returned an active grant for a different reservation reference",
                    ),
                );
            }
        }

        Ok(Self {
            lookup,
            state,
        })
    }

    /// Returns the exact lookup answered by the provider.
    #[must_use]
    pub fn lookup(&self) -> &HardwareReservationLookup {
        &self.lookup
    }

    /// Returns the provider-reported reservation state.
    #[must_use]
    pub fn state(&self) -> &HardwareReservationLeaseState {
        &self.state
    }

    /// Returns the active grant, if the reservation is confirmed active.
    #[must_use]
    pub fn active_grant(&self) -> Option<&HardwareReservationGrant> {
        match &self.state {
            HardwareReservationLeaseState::Active(grant) => Some(grant),
            _ => None,
        }
    }

    /// Returns whether the provider has established a definitive state.
    ///
    /// Pending and Indeterminate are not definitive.
    #[must_use]
    pub fn is_definitive(&self) -> bool {
        !matches!(
            self.state,
            HardwareReservationLeaseState::Pending
                | HardwareReservationLeaseState::Indeterminate
        )
    }

    /// Validates this observation against the requested provider and lookup.
    ///
    /// # Errors
    ///
    /// Returns a contract violation if the provider identity,
    /// lookup or active grant is inconsistent.
    pub fn validate_for(
        &self,
        provider_id: &ProviderId,
        expected_lookup: &HardwareReservationLookup,
    ) -> ZhalResult<()> {
        if &self.lookup != expected_lookup {
            return Err(
                contract_violation(
                    "provider returned an observation for a different reservation lookup",
                ),
            );
        }

        if let HardwareReservationLeaseState::Active(grant) = &self.state {
            if grant.provider_id() != provider_id {
                return Err(
                    contract_violation(
                        "provider returned an active reservation belonging to another provider",
                    ),
                );
            }

            if let HardwareReservationLookup::ProviderReference(expected) =
                expected_lookup
            {
                if grant.reference() != expected {
                    return Err(
                        contract_violation(
                            "active reservation reference does not match the requested lookup",
                        ),
                    );
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// PROVIDER RECONCILIATION EXTENSION
// ============================================================================

/// Optional physical reservation inspection extension.
///
/// This is deliberately separate from the base hardware-provider
/// contract and from the reservation mutation contract.
///
/// Providers that support authoritative reservation inspection
/// implement this extension.
///
/// It does not require every hardware provider to support reservations.
///
/// # Provider requirements
///
/// Implementations must:
///
/// - Inspect the authoritative reservation system.
/// - Preserve the exact lookup identity.
/// - Return the actual provider-reported state.
/// - Distinguish NotFound from Indeterminate.
/// - Avoid changing reservation state during inspection.
/// - Verify that active grants belong to the provider.
/// - Enforce applicable authentication and authorization.
/// - Avoid exposing credentials in returned data.
///
/// # Idempotency reconciliation
///
/// When inspecting by idempotency key, the provider must resolve
/// the original logical request, not create a new reservation.
///
/// If the provider cannot prove whether the original request
/// committed, it must return Indeterminate.
///
/// # Concurrency
///
/// Inspection must be safe when concurrent with renewal or release.
///
/// Providers should document the consistency guarantees of their
/// reservation-status endpoint.
///
/// A result is an observation, not a lock on future state.
pub trait HardwareReservationLeaseProvider:
    HardwareReservationProviderIdentity
{
    /// Inspects the authoritative state of an existing reservation.
    ///
    /// This operation is observational. It must not reserve,
    /// renew or release physical resources.
    ///
    /// # Errors
    ///
    /// Returns a structured error for communication failure,
    /// access denial, provider failure or invalid requests.
    ///
    /// A provider that successfully responds but cannot establish
    /// the reservation state should return an observation whose
    /// state is `Indeterminate`.
    fn inspect_reservation<'a>(
        &'a self,
        lookup: &'a HardwareReservationLookup,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareReservationLeaseObservation>;
}

// ============================================================================
// VALIDATED INSPECTION
// ============================================================================

/// Inspects a reservation and validates the provider's response.
///
/// This function:
///
/// - Checks cancellation and deadline before dispatch.
/// - Verifies the provider identity in the supplied context, if present.
/// - Calls the provider's authoritative inspection operation.
/// - Verifies that the response answers the requested lookup.
/// - Validates the provider identity of an active grant.
///
/// It does not:
///
/// - Retry a reservation.
/// - Create a reservation.
/// - Renew a reservation.
/// - Release a reservation.
/// - Modify ZUR lease state.
/// - Decide whether a workload should continue.
///
/// # Important
///
/// The response is not discarded merely because cancellation occurs
/// after the read-only inspection has been dispatched. Returning
/// reconciliation evidence is preferable to losing useful recovery
/// information.
///
/// # Errors
///
/// Returns a structured error for cancellation, deadline expiration,
/// provider mismatch, provider failure or contract violation.
pub async fn inspect_validated(
    provider: &dyn HardwareReservationLeaseProvider,
    lookup: &HardwareReservationLookup,
    context: &ZhalContext,
) -> ZhalResult<HardwareReservationLeaseObservation> {
    check_active(context)?;

    let provider_id = provider.provider_id();

    if let Some(context_provider) = context.provider_id() {
        if context_provider != provider_id {
            return Err(
                ZhalError::new(
                    ZhalErrorKind::InvalidInput,
                    "reservation inspection context belongs to another provider",
                )
                .in_component(COMPONENT),
            );
        }
    }

    let observation = provider
        .inspect_reservation(lookup, context)
        .await?;

    observation.validate_for(
        provider_id,
        lookup,
    )?;

    Ok(observation)
}

// ============================================================================
// RECONCILIATION CLASSIFICATION
// ============================================================================

/// Returns whether a provider observation permits treating a reservation
/// as currently confirmed active.
///
/// This deliberately returns true only for Active.
///
/// Pending, Indeterminate, NotFound, Expired and Released must not
/// be treated as active reservations.
#[must_use]
pub fn is_confirmed_active(
    observation: &HardwareReservationLeaseObservation,
) -> bool {
    matches!(
        observation.state(),
        HardwareReservationLeaseState::Active(_)
    )
}

/// Returns whether a provider observation confirms that a reservation
/// has been released.
///
/// Expired is not treated as confirmed release because the provider
/// may still need to perform external cleanup.
#[must_use]
pub fn is_confirmed_released(
    observation: &HardwareReservationLeaseObservation,
) -> bool {
    matches!(
        observation.state(),
        HardwareReservationLeaseState::Released
    )
}

/// Returns whether the reservation outcome requires reconciliation
/// or a later inspection.
#[must_use]
pub fn requires_reconciliation(
    observation: &HardwareReservationLeaseObservation,
) -> bool {
    matches!(
        observation.state(),
        HardwareReservationLeaseState::Pending
            | HardwareReservationLeaseState::Indeterminate
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use core::time::Duration;

    use crate::stdlib::io::devices::DeviceId;

    use crate::zhal::identity::{
        HardwareDeviceIdentity,
        HardwareResourceId,
        HardwareResourceIdentity,
    };

    use crate::zhal::resources::reservation::{
        HardwareReservationReference,
    };

    use crate::zhal::model::resource::ResourceAmount;

    fn provider_id() -> ProviderId {
        ProviderId::new("test.hardware.provider")
            .expect("valid provider identity")
    }

    fn resource_identity() -> HardwareResourceIdentity {
        let device = HardwareDeviceIdentity::new(
            provider_id(),
            DeviceId::new("test-device")
                .expect("valid device identity"),
        );

        HardwareResourceIdentity::new(
            device,
            HardwareResourceId::new("compute-unit")
                .expect("valid resource identity"),
        )
    }

    fn grant() -> HardwareReservationGrant {
        HardwareReservationGrant::confirmed(
            provider_id(),
            resource_identity(),
            ResourceAmount::new("10")
                .expect("valid resource amount"),
            HardwareReservationReference::new(
                "provider.reservation.001",
            )
            .expect("valid provider reference"),
            Some(Duration::from_secs(60)),
        )
        .expect("valid reservation grant")
    }

    fn key_lookup() -> HardwareReservationLookup {
        HardwareReservationLookup::by_idempotency_key(
            "reservation.request.001",
        )
        .expect("valid idempotency key")
    }

    // ------------------------------------------------------------------------
    // Schema
    // ------------------------------------------------------------------------

    #[test]
    fn schema_version_is_nonzero() {
        assert_ne!(schema_version(), 0);
    }

    // ------------------------------------------------------------------------
    // Lookup validation
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_idempotency_lookup() {
        let lookup = key_lookup();

        assert_eq!(
            lookup.idempotency_key(),
            Some("reservation.request.001"),
        );

        assert!(lookup.provider_reference().is_none());
    }

    #[test]
    fn rejects_empty_idempotency_key() {
        assert!(
            HardwareReservationLookup::by_idempotency_key("")
                .is_err()
        );
    }

    #[test]
    fn rejects_whitespace_padded_idempotency_key() {
        assert!(
            HardwareReservationLookup::by_idempotency_key(
                " request ",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_control_characters_in_idempotency_key() {
        assert!(
            HardwareReservationLookup::by_idempotency_key(
                "request\ninvalid",
            )
            .is_err()
        );
    }

    #[test]
    fn accepts_open_world_provider_reference() {
        let reference = HardwareReservationReference::new(
            "future.provider/region/partition/reservation/001",
        )
        .expect("valid provider reference");

        let lookup =
            HardwareReservationLookup::by_provider_reference(
                reference,
            );

        assert!(
            lookup.provider_reference().is_some()
        );
    }

    // ------------------------------------------------------------------------
    // Observation validation
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_active_observation_by_idempotency_key() {
        let observation =
            HardwareReservationLeaseObservation::new(
                key_lookup(),
                HardwareReservationLeaseState::Active(
                    grant(),
                ),
            )
            .expect("valid active observation");

        assert!(
            observation.active_grant().is_some()
        );

        assert!(
            observation.is_definitive()
        );

        assert!(
            is_confirmed_active(&observation)
        );
    }

    #[test]
    fn rejects_active_observation_with_wrong_reference() {
        let expected = HardwareReservationReference::new(
            "provider.reservation.expected",
        )
        .expect("valid reference");

        let lookup =
            HardwareReservationLookup::by_provider_reference(
                expected,
            );

        assert!(
            HardwareReservationLeaseObservation::new(
                lookup,
                HardwareReservationLeaseState::Active(
                    grant(),
                ),
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_observation_for_another_lookup() {
        let observation =
            HardwareReservationLeaseObservation::new(
                key_lookup(),
                HardwareReservationLeaseState::NotFound,
            )
            .expect("valid observation");

        let other_lookup =
            HardwareReservationLookup::by_idempotency_key(
                "another.request",
            )
            .expect("valid lookup");

        assert!(
            observation
                .validate_for(
                    &provider_id(),
                    &other_lookup,
                )
                .is_err()
        );
    }

    #[test]
    fn rejects_active_grant_from_another_provider() {
        let other_provider = ProviderId::new(
            "another.hardware.provider",
        )
        .expect("valid provider");

        let observation =
            HardwareReservationLeaseObservation::new(
                key_lookup(),
                HardwareReservationLeaseState::Active(
                    grant(),
                ),
            )
            .expect("valid observation");

        assert!(
            observation
                .validate_for(
                    &other_provider,
                    &key_lookup(),
                )
                .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // State distinctions
    // ------------------------------------------------------------------------

    #[test]
    fn not_found_is_definitive() {
        let observation =
            HardwareReservationLeaseObservation::new(
                key_lookup(),
                HardwareReservationLeaseState::NotFound,
            )
            .expect("valid observation");

        assert!(observation.is_definitive());

        assert!(!is_confirmed_active(&observation));

        assert!(!is_confirmed_released(&observation));

        assert!(!requires_reconciliation(&observation));
    }

    #[test]
    fn released_is_not_active() {
        let observation =
            HardwareReservationLeaseObservation::new(
                key_lookup(),
                HardwareReservationLeaseState::Released,
            )
            .expect("valid observation");

        assert!(is_confirmed_released(&observation));

        assert!(!is_confirmed_active(&observation));
    }

    #[test]
    fn expired_is_not_confirmed_released() {
        let observation =
            HardwareReservationLeaseObservation::new(
                key_lookup(),
                HardwareReservationLeaseState::Expired,
            )
            .expect("valid observation");

        assert!(!is_confirmed_active(&observation));

        assert!(!is_confirmed_released(&observation));
    }

    #[test]
    fn pending_requires_reconciliation() {
        let observation =
            HardwareReservationLeaseObservation::new(
                key_lookup(),
                HardwareReservationLeaseState::Pending,
            )
            .expect("valid observation");

        assert!(!observation.is_definitive());

        assert!(requires_reconciliation(&observation));
    }

    #[test]
    fn indeterminate_requires_reconciliation() {
        let observation =
            HardwareReservationLeaseObservation::new(
                key_lookup(),
                HardwareReservationLeaseState::Indeterminate,
            )
            .expect("valid observation");

        assert!(!observation.is_definitive());

        assert!(requires_reconciliation(&observation));
    }
}
