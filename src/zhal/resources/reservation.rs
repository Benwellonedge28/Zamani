
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Physical Hardware Resource Reservation
//!
//! Path: `src/zhal/resources/reservation.rs`
//!
//! Rust edition: 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines the provider-facing physical hardware reservation contract.
//!
//! This module allows the Zamani runtime to request, renew and release
//! physical resource reservations through the authoritative hardware
//! provider responsible for the requested resource.
//!
//! Supported resource domains include:
//!
//! - Classical processors and memory.
//! - Quantum processors, qubits and control resources.
//! - FPGA and ASIC resources.
//! - GPUs, TPUs, NPUs and other accelerators.
//! - Distributed computing resources.
//! - Embedded hardware.
//! - Networking and interconnect resources.
//! - Neuromorphic hardware.
//! - Photonic and optical resources.
//! - Analog and future computational technologies.
//!
//! Resource categories and provider implementations are open-world.
//!
//! ## ARCHITECTURAL OWNERSHIP
//!
//! This module owns:
//!
//! - HardwareReservationRequest.
//! - HardwareReservationGrant.
//! - HardwareReservationReference.
//! - HardwareReservationProvider.
//! - Hardware reservation request validation.
//! - Hardware reservation grant validation.
//! - Provider-facing reservation, renewal and release contracts.
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
//! - Hardware device inventory.
//! - Runtime resource allocation policy.
//! - Runtime resource lease identities.
//! - Runtime resource lease lifecycle.
//! - Resource scheduling.
//! - Resource placement.
//! - Quantum routing.
//! - HDL realization.
//! - Distributed execution.
//! - Hardware execution.
//!
//! ## CANONICAL REPOSITORY AUTHORITIES
//!
//! Physical resource identity:
//!     crate::zhal::identity::HardwareResourceIdentity
//!
//! Physical resource quantities:
//!     crate::zhal::model::resource::ResourceAmount
//!
//! Provider identity:
//!     crate::stdlib::provider::ProviderId
//!
//! Hardware provider operations:
//!     crate::zhal::providers::contract
//!
//! Hardware operation context:
//!     crate::zhal::context::ZhalContext
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Runtime resource acquisition and lease lifecycle:
//!     crate::zur::resources::lease
//!
//! Runtime reservation and acquisition lifecycle:
//!     crate::zur::resources::lifecycle
//!
//! Runtime resource accounting:
//!     crate::zur::resources::accounting
//!
//! Source-level resource requirements:
//!     crate::frontend::semantic::resources
//!
//! ## CRITICAL ARCHITECTURAL DISTINCTIONS
//!
//! Requirement != Feasibility.
//! Feasibility != Reservation.
//! Reservation != Acquisition.
//! Acquisition != Execution.
//! Release request != Confirmed release.
//! Cancellation != Confirmed remote termination.
//! Reservation grant != Permanent availability.
//!
//! ZHAL communicates with the physical resource authority.
//!
//! ZUR remains responsible for the complete runtime resource lease
//! lifecycle and orchestration.
//!
//! ## RESERVATION MODEL
//!
//! A reservation request identifies:
//!
//! - The physical resource.
//! - The requested quantity.
//! - The explicit owner.
//! - An idempotency reference.
//! - An optional requested lifetime.
//!
//! The provider validates actual physical availability and returns
//! a confirmed grant or a structured error.
//!
//! A successful grant must never be fabricated from an availability
//! observation.
//!
//! ## IDEMPOTENCY
//!
//! Every request contains a caller-generated idempotency reference.
//!
//! Providers must use this reference to reconcile retries and ambiguous
//! network outcomes.
//!
//! Repeating a request with the same reference and identical parameters
//! must not create duplicate reservations.
//!
//! Reusing the same reference with different parameters must be rejected.
//!
//! ## RENEWAL
//!
//! Renewal must be explicitly confirmed by the provider.
//!
//! A failed or ambiguous renewal must not be interpreted as a confirmed
//! extension.
//!
//! The runtime must reconcile uncertain remote outcomes.
//!
//! ## RELEASE
//!
//! Release must be idempotent.
//!
//! A provider must not report successful physical release until the
//! authoritative resource manager confirms the release.
//!
//! An ambiguous release remains an uncertain outcome.
//!
//! ## SCALABILITY
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Resource quantities.
//! - Reservation requests.
//! - Hardware resources.
//! - Resource categories.
//! - Reservation references.
//! - Hardware providers.
//! - Devices.
//! - Computational domains.
//! - Distributed participants.
//!
//! Quantities reuse ResourceAmount, which represents exact decimal
//! values without fixed-width integer hardware ceilings.
//!
//! Operational limits may be supplied by the runtime or provider.
//! Such limits are operational budgets, not language-level limitations.
//!
//! Literal infinite physical resources are not promised.
//!
//! Actual operations are bounded by available physical resources,
//! address-space representation, available memory, provider capabilities
//! and explicitly configured operational budgets.
//!
//! ## CONCURRENCY
//!
//! This module owns no global mutable state.
//!
//! Provider implementations own synchronization of their reservation
//! authorities.
//!
//! Provider operations must not hold unrelated ZHAL registry or inventory
//! locks while waiting for remote operations.
//!
//! ## SECURITY
//!
//! Owner references and reservation references are descriptive identifiers.
//!
//! They are not credentials or authorization tokens.
//!
//! Authorization must be performed by the existing security authority.
//!
//! Never store credentials, passwords, private keys, bearer tokens or
//! authentication headers in reservation requests or grants.
//!
//! Debug output must not expose authentication secrets.
//!
//! ## GRAMMAR BOUNDARY
//!
//! Relevant grammar:
//!
//!     grammar/resources/resources.g4
//!     grammar/resources/requirements.g4
//!     grammar/statements/resource.g4
//!
//! Relevant normative specification:
//!
//!     grammar/specification/poco-reaf.md
//!     grammar/specification/execution-model.md
//!
//! The grammar expresses resource intent.
//!
//! Semantic analysis establishes requirement meaning.
//!
//! Runtime planning evaluates feasibility.
//!
//! ZUR coordinates acquisition.
//!
//! ZHAL exposes physical reservation operations.
//!
//! None of these responsibilities should be duplicated here.

// ============================================================================
// IMPORTS
// ============================================================================

use core::fmt;
use core::time::Duration;

use crate::stdlib::provider::ProviderId;

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::HardwareResourceIdentity;

use crate::zhal::model::resource::ResourceAmount;

use crate::zhal::providers::contract::HardwareProviderFuture;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable component identity.
pub const COMPONENT: &str = "zhal.resources.reservation";

/// Reservation contract schema version.
///
/// This is independent of the Zamani package version, language version,
/// hardware-provider API version and firmware version.
pub const RESERVATION_SCHEMA_VERSION: u16 = 1;

/// Returns the reservation contract schema version.
#[must_use]
pub const fn schema_version() -> u16 {
    RESERVATION_SCHEMA_VERSION
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
                format!("{field} must not contain control characters"),
            )
            .in_component(COMPONENT),
        );
    }

    Ok(())
}

fn invalid_grant(
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(
        ZhalErrorKind::ContractViolation,
        message,
    )
    .in_component(COMPONENT)
}

fn check_active(
    context: &ZhalContext,
) -> ZhalResult<()> {
    ZhalError::check_cancelled(
        context.operation_context(),
    )?;

    if context.is_deadline_expired() {
        return Err(
            ZhalError::new(
                ZhalErrorKind::OperationTimedOut,
                "hardware reservation operation deadline expired",
            )
            .in_component(COMPONENT),
        );
    }

    Ok(())
}

// ============================================================================
// RESERVATION REQUEST
// ============================================================================

/// Immutable request to reserve a physical hardware resource.
///
/// The request is provider-independent.
///
/// It does not imply that the requested resource is available,
/// compatible or successfully reserved.
///
/// Resource feasibility must be established by the appropriate
/// compiler/runtime planning authorities.
///
/// The physical provider must independently validate the current
/// resource state before committing the reservation.
///
/// # Idempotency
///
/// `idempotency_key` identifies one logical reservation request.
///
/// Providers must preserve its meaning across retries.
///
/// An identical retry must not create another reservation.
///
/// A retry using the same key with different request parameters
/// must be rejected.
///
/// # Lifetime
///
/// `requested_ttl` is optional.
///
/// When absent, the provider's explicitly configured reservation
/// policy determines the applicable lifetime.
///
/// An absent lifetime does not mean an infinite reservation.
///
/// # Scalability
///
/// Resource quantities are represented using ResourceAmount.
///
/// No fixed-width machine integer is used to represent physical
/// resource capacity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HardwareReservationRequest {
    resource: HardwareResourceIdentity,
    quantity: ResourceAmount,
    owner_reference: String,
    idempotency_key: String,
    requested_ttl: Option<Duration>,
}

impl HardwareReservationRequest {
    /// Creates a validated physical resource reservation request.
    ///
    /// # Arguments
    ///
    /// * `resource` - Canonical physical resource identity.
    /// * `quantity` - Exact non-negative physical quantity.
    /// * `owner_reference` - Explicit external owner identity.
    /// * `idempotency_key` - Stable caller-generated request identity.
    /// * `requested_ttl` - Optional requested reservation lifetime.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The quantity is zero.
    /// - The owner reference is invalid.
    /// - The idempotency reference is invalid.
    /// - The requested lifetime is zero.
    pub fn new(
        resource: HardwareResourceIdentity,
        quantity: ResourceAmount,
        owner_reference: impl Into<String>,
        idempotency_key: impl Into<String>,
        requested_ttl: Option<Duration>,
    ) -> ZhalResult<Self> {
        if quantity.is_zero() {
            return Err(
                ZhalError::new(
                    ZhalErrorKind::InvalidInput,
                    "a physical resource reservation quantity must be greater than zero",
                )
                .in_component(COMPONENT),
            );
        }

        let owner_reference = owner_reference.into();
        let idempotency_key = idempotency_key.into();

        validate_reference(
            &owner_reference,
            "owner reference",
        )?;

        validate_reference(
            &idempotency_key,
            "idempotency key",
        )?;

        if requested_ttl.is_some_and(|ttl| ttl.is_zero()) {
            return Err(
                ZhalError::new(
                    ZhalErrorKind::InvalidInput,
                    "requested reservation lifetime must be greater than zero",
                )
                .in_component(COMPONENT),
            );
        }

        Ok(Self {
            resource,
            quantity,
            owner_reference,
            idempotency_key,
            requested_ttl,
        })
    }

    /// Returns the physical resource identity.
    #[must_use]
    pub fn resource(&self) -> &HardwareResourceIdentity {
        &self.resource
    }

    /// Returns the exact requested resource quantity.
    #[must_use]
    pub fn quantity(&self) -> &ResourceAmount {
        &self.quantity
    }

    /// Returns the external owner reference.
    #[must_use]
    pub fn owner_reference(&self) -> &str {
        &self.owner_reference
    }

    /// Returns the stable idempotency reference.
    #[must_use]
    pub fn idempotency_key(&self) -> &str {
        &self.idempotency_key
    }

    /// Returns the optional requested reservation lifetime.
    #[must_use]
    pub const fn requested_ttl(&self) -> Option<Duration> {
        self.requested_ttl
    }

    /// Returns the provider responsible for this resource.
    ///
    /// Provider identity is derived from the canonical resource identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        self.resource.device().provider_id()
    }
}

// ============================================================================
// PROVIDER RESERVATION REFERENCE
// ============================================================================

/// Opaque reservation reference assigned by an external provider.
///
/// This identifies an externally managed physical reservation.
///
/// It is not:
///
/// - A ZUR resource lease identity.
/// - A hardware resource identity.
/// - An authentication credential.
/// - An authorization token.
/// - A reservation request idempotency key.
///
/// Its interpretation belongs exclusively to the provider that
/// created the reservation.
///
/// References are open-ended and have no arbitrary length ceiling.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HardwareReservationReference(String);

impl HardwareReservationReference {
    /// Constructs a validated provider reservation reference.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty reference, surrounding whitespace
    /// or control characters.
    pub fn new(
        value: impl Into<String>,
    ) -> ZhalResult<Self> {
        let value = value.into();

        validate_reference(
            &value,
            "provider reservation reference",
        )?;

        Ok(Self(value))
    }

    /// Returns the exact provider reservation reference.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the reference and returns its owned representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Debug for HardwareReservationReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_tuple("HardwareReservationReference")
            .field(&self.0)
            .finish()
    }
}

impl fmt::Display for HardwareReservationReference {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// CONFIRMED RESERVATION GRANT
// ============================================================================

/// Confirmed grant returned by a physical resource authority.
///
/// A grant is evidence that the provider has accepted the reservation.
///
/// It does not guarantee:
///
/// - That the resource will remain healthy.
/// - That hardware execution will succeed.
/// - That a remote connection will remain available.
/// - That the reservation cannot expire.
/// - That the resource has already been acquired by a workload.
///
/// The provider must construct a grant only after its authoritative
/// reservation mechanism confirms the physical reservation.
///
/// # Lifetime
///
/// `granted_ttl` describes the provider-confirmed lifetime from
/// the time the reservation was accepted.
///
/// The provider and runtime must use the appropriate local
/// monotonic-clock authority to track expiration.
///
/// A duration is not a portable absolute timestamp.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HardwareReservationGrant {
    provider_id: ProviderId,
    resource: HardwareResourceIdentity,
    quantity: ResourceAmount,
    reference: HardwareReservationReference,
    granted_ttl: Option<Duration>,
}

impl HardwareReservationGrant {
    /// Constructs a provider-confirmed reservation grant.
    ///
    /// This constructor is intended for provider implementations.
    ///
    /// Callers must not use it to fabricate physical reservations.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// - The quantity is zero.
    /// - The reservation reference is invalid.
    /// - The provider does not own the physical resource.
    /// - The confirmed lifetime is zero.
    pub fn confirmed(
        provider_id: ProviderId,
        resource: HardwareResourceIdentity,
        quantity: ResourceAmount,
        reference: HardwareReservationReference,
        granted_ttl: Option<Duration>,
    ) -> ZhalResult<Self> {
        if quantity.is_zero() {
            return Err(
                invalid_grant(
                    "a confirmed reservation cannot contain a zero quantity",
                ),
            );
        }

        if resource.device().provider_id() != &provider_id {
            return Err(
                invalid_grant(
                    "reservation provider does not own the requested hardware resource",
                ),
            );
        }

        if granted_ttl.is_some_and(|ttl| ttl.is_zero()) {
            return Err(
                invalid_grant(
                    "a confirmed reservation lifetime must be greater than zero",
                ),
            );
        }

        Ok(Self {
            provider_id,
            resource,
            quantity,
            reference,
            granted_ttl,
        })
    }

    /// Returns the authoritative provider identity.
    #[must_use]
    pub fn provider_id(&self) -> &ProviderId {
        &self.provider_id
    }

    /// Returns the physical resource identity.
    #[must_use]
    pub fn resource(&self) -> &HardwareResourceIdentity {
        &self.resource
    }

    /// Returns the confirmed resource quantity.
    #[must_use]
    pub fn quantity(&self) -> &ResourceAmount {
        &self.quantity
    }

    /// Returns the external reservation reference.
    #[must_use]
    pub fn reference(&self) -> &HardwareReservationReference {
        &self.reference
    }

    /// Returns the provider-confirmed optional reservation lifetime.
    #[must_use]
    pub const fn granted_ttl(&self) -> Option<Duration> {
        self.granted_ttl
    }
}

// ============================================================================
// RESERVATION GRANT VALIDATION
// ============================================================================

/// Validates a provider grant against its original request.
///
/// This prevents providers from returning a grant for:
///
/// - Another provider.
/// - Another physical resource.
/// - An unrequested quantity.
/// - An invalid lifetime.
///
/// A grant that does not satisfy the original request must never
/// be treated as a successful reservation.
///
/// # Errors
///
/// Returns a contract violation when the grant does not match.
pub fn validate_grant(
    request: &HardwareReservationRequest,
    grant: &HardwareReservationGrant,
) -> ZhalResult<()> {
    if grant.provider_id() != request.provider_id() {
        return Err(
            invalid_grant(
                "provider returned a reservation grant belonging to another provider",
            ),
        );
    }

    if grant.resource() != request.resource() {
        return Err(
            invalid_grant(
                "provider returned a reservation grant for another physical resource",
            ),
        );
    }

    if grant.quantity() != request.quantity() {
        return Err(
            invalid_grant(
                "provider returned a reservation quantity different from the requested quantity",
            ),
        );
    }

    if grant.granted_ttl().is_some_and(|ttl| ttl.is_zero()) {
        return Err(
            invalid_grant(
                "provider returned an invalid reservation lifetime",
            ),
        );
    }

    Ok(())
}

// ============================================================================
// HARDWARE RESERVATION PROVIDER
// ============================================================================

/// Physical hardware reservation extension contract.
///
/// Implementations are supplied by the provider that owns the
/// physical resource.
///
/// This is deliberately separate from the universal HardwareProvider
/// trait because not every provider supports physical reservation.
///
/// A provider that does not implement this extension must not pretend
/// that a successful reservation has occurred.
///
/// The runtime may discover reservation support through its
/// provider integration and capability authorities.
///
/// # Concurrency
///
/// Implementations must safely coordinate concurrent requests.
///
/// Idempotency must be enforced by the authoritative reservation system,
/// not merely by a local cache.
///
/// # Distributed operation
///
/// Implementations must reconcile ambiguous remote outcomes.
///
/// A communication failure does not prove that a reservation failed.
///
/// An unknown outcome must be returned using the canonical
/// OutcomeUnknown error classification.
///
/// # Renewal
///
/// Renewal must extend the actual external reservation.
///
/// Returning a locally calculated expiration without provider
/// confirmation violates this contract.
///
/// # Release
///
/// Release must be idempotent.
///
/// A provider must not claim physical release until confirmed by
/// the authoritative resource-management system.
pub trait HardwareReservationProvider: Send + Sync + 'static {

    /// Requests a physical resource reservation.
    ///
    /// The provider must:
    ///
    /// 1. Validate that the requested resource belongs to itself.
    /// 2. Revalidate current physical availability.
    /// 3. Enforce applicable authorization.
    /// 4. Validate the requested quantity and unit compatibility.
    /// 5. Enforce idempotency.
    /// 6. Commit the physical reservation.
    /// 7. Return a confirmed grant only after successful commitment.
    ///
    /// The provider must not infer reservation success from a
    /// previously observed availability snapshot.
    ///
    /// # Errors
    ///
    /// Returns a structured error for:
    ///
    /// - Invalid requests.
    /// - Foreign resources.
    /// - Unavailable resources.
    /// - Insufficient capacity.
    /// - Access denial.
    /// - Provider failure.
    /// - Communication failure.
    /// - Unknown remote outcome.
    /// - Cancellation.
    /// - Deadline expiration.
    fn reserve<'a>(
        &'a self,
        request: &'a HardwareReservationRequest,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareReservationGrant>;

    /// Renews an existing physical reservation.
    ///
    /// The provider must verify that:
    ///
    /// - The reservation belongs to itself.
    /// - The reservation still exists.
    /// - The reservation has not been definitively released.
    /// - The owner remains authorized.
    ///
    /// A successful result must contain the newly confirmed
    /// provider reservation grant.
    ///
    /// The resource identity and quantity must remain unchanged.
    ///
    /// # Errors
    ///
    /// Returns a structured error when renewal cannot be confirmed.
    ///
    /// An ambiguous remote outcome must not be represented as
    /// a successful renewal.
    fn renew<'a>(
        &'a self,
        grant: &'a HardwareReservationGrant,
        requested_ttl: Duration,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareReservationGrant>;

    /// Releases an existing physical reservation.
    ///
    /// The operation must be idempotent.
    ///
    /// The provider must verify reservation ownership and execute
    /// the release through the authoritative resource manager.
    ///
    /// Successful completion means physical release has been confirmed.
    ///
    /// An ambiguous outcome must remain distinguishable from
    /// confirmed release.
    ///
    /// # Errors
    ///
    /// Returns a structured error when release cannot be confirmed.
    fn release<'a>(
        &'a self,
        grant: &'a HardwareReservationGrant,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, ()>;
}

// ============================================================================
// VALIDATED PROVIDER OPERATIONS
// ============================================================================

/// Requests a reservation and validates the provider's response.
///
/// This wrapper performs common validation before and after
/// the provider operation.
///
/// It does not perform provider selection, physical discovery,
/// feasibility analysis or runtime resource allocation.
///
/// # Errors
///
/// Returns a structured error for invalid requests, cancellation,
/// expired deadlines, provider failures and invalid grants.
pub async fn reserve_validated(
    provider: &dyn HardwareReservationProvider,
    request: &HardwareReservationRequest,
    context: &ZhalContext,
) -> ZhalResult<HardwareReservationGrant> {
    check_active(context)?;

    if provider.provider_id() != request.provider_id() {
        return Err(
            ZhalError::new(
                ZhalErrorKind::InvalidInput,
                "hardware reservation request belongs to another provider",
            )
            .in_component(COMPONENT),
        );
    }

    let grant = provider
        .reserve(request, context)
        .await?;

    check_active(context)?;

    validate_grant(request, &grant)?;

    Ok(grant)
}

/// Renews a reservation and validates the returned grant.
///
/// The provider must confirm the actual extension.
///
/// This function does not modify the canonical ZUR lease state.
///
/// ZUR remains responsible for integrating the renewed grant
/// with its ResourceLease lifecycle.
///
/// # Errors
///
/// Returns an error if the provider is incorrect, the requested
/// lifetime is invalid, the operation fails or the returned grant
/// violates the original reservation contract.
pub async fn renew_validated(
    provider: &dyn HardwareReservationProvider,
    grant: &HardwareReservationGrant,
    requested_ttl: Duration,
    context: &ZhalContext,
) -> ZhalResult<HardwareReservationGrant> {
    check_active(context)?;

    if requested_ttl.is_zero() {
        return Err(
            ZhalError::new(
                ZhalErrorKind::InvalidInput,
                "reservation renewal lifetime must be greater than zero",
            )
            .in_component(COMPONENT),
        );
    }

    if provider.provider_id() != grant.provider_id() {
        return Err(
            ZhalError::new(
                ZhalErrorKind::InvalidInput,
                "reservation grant belongs to another provider",
            )
            .in_component(COMPONENT),
        );
    }

    let renewed = provider
        .renew(
            grant,
            requested_ttl,
            context,
        )
        .await?;

    check_active(context)?;

    if renewed.provider_id() != grant.provider_id()
        || renewed.resource() != grant.resource()
        || renewed.quantity() != grant.quantity()
        || renewed.reference() != grant.reference()
    {
        return Err(
            invalid_grant(
                "renewal changed the identity, resource, quantity or provider reservation reference",
            ),
        );
    }

    if renewed.granted_ttl().is_none() {
        return Err(
            invalid_grant(
                "provider did not confirm the renewed reservation lifetime",
            ),
        );
    }

    Ok(renewed)
}

/// Releases a confirmed physical reservation.
///
/// Successful completion means the provider confirms release.
///
/// This wrapper does not silently retry an ambiguous release.
///
/// The runtime must use its existing lifecycle and reconciliation
/// authorities when the outcome cannot be established.
///
/// # Errors
///
/// Returns a structured error when the provider is incorrect,
/// cancellation occurs, the deadline expires or release fails.
pub async fn release_validated(
    provider: &dyn HardwareReservationProvider,
    grant: &HardwareReservationGrant,
    context: &ZhalContext,
) -> ZhalResult<()> {
    check_active(context)?;

    if provider.provider_id() != grant.provider_id() {
        return Err(
            ZhalError::new(
                ZhalErrorKind::InvalidInput,
                "reservation grant belongs to another provider",
            )
            .in_component(COMPONENT),
        );
    }

    provider
        .release(grant, context)
        .await?;

    check_active(context)?;

    Ok(())
}

// ============================================================================
// PROVIDER IDENTITY EXTENSION
// ============================================================================

/// Provides the canonical identity of a hardware reservation provider.
///
/// This is separate from HardwareProvider because reservation
/// support is an optional provider extension.
///
/// Implementations must return a stable identity that matches
/// the physical resources they reserve.
pub trait HardwareReservationProviderIdentity:
    HardwareReservationProvider
{
    /// Returns the canonical identity of the reservation authority.
    fn provider_id(&self) -> &ProviderId;
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::DeviceId;

    use crate::zhal::identity::{
        HardwareDeviceIdentity,
        HardwareResourceId,
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

    fn amount(value: &str) -> ResourceAmount {
        ResourceAmount::new(value)
            .expect("valid exact resource quantity")
    }

    fn request() -> HardwareReservationRequest {
        HardwareReservationRequest::new(
            resource_identity(),
            amount("100"),
            "runtime.execution.001",
            "reservation.request.001",
            Some(Duration::from_secs(60)),
        )
        .expect("valid reservation request")
    }

    fn grant(
        quantity: &str,
    ) -> HardwareReservationGrant {
        HardwareReservationGrant::confirmed(
            provider_id(),
            resource_identity(),
            amount(quantity),
            HardwareReservationReference::new(
                "provider.reservation.001",
            )
            .expect("valid reservation reference"),
            Some(Duration::from_secs(60)),
        )
        .expect("valid confirmed grant")
    }

    // ------------------------------------------------------------------------
    // Schema
    // ------------------------------------------------------------------------

    #[test]
    fn schema_version_is_nonzero() {
        assert_ne!(schema_version(), 0);
    }

    // ------------------------------------------------------------------------
    // Request validation
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_valid_reservation_request() {
        let request = request();

        assert_eq!(
            request.quantity().as_str(),
            "100",
        );

        assert_eq!(
            request.owner_reference(),
            "runtime.execution.001",
        );

        assert_eq!(
            request.idempotency_key(),
            "reservation.request.001",
        );
    }

    #[test]
    fn rejects_zero_quantity() {
        assert!(
            HardwareReservationRequest::new(
                resource_identity(),
                amount("0"),
                "owner",
                "request",
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_empty_owner_reference() {
        assert!(
            HardwareReservationRequest::new(
                resource_identity(),
                amount("1"),
                "",
                "request",
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_empty_idempotency_key() {
        assert!(
            HardwareReservationRequest::new(
                resource_identity(),
                amount("1"),
                "owner",
                "",
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_control_characters() {
        assert!(
            HardwareReservationRequest::new(
                resource_identity(),
                amount("1"),
                "owner\ninvalid",
                "request",
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_zero_reservation_lifetime() {
        assert!(
            HardwareReservationRequest::new(
                resource_identity(),
                amount("1"),
                "owner",
                "request",
                Some(Duration::ZERO),
            )
            .is_err()
        );
    }

    #[test]
    fn accepts_provider_defined_lifetime() {
        let result = HardwareReservationRequest::new(
            resource_identity(),
            amount("1"),
            "owner",
            "request",
            None,
        );

        assert!(result.is_ok());
    }

    // ------------------------------------------------------------------------
    // Exact quantities and scalability
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_quantities_beyond_u128() {
        let quantity = amount(
            "340282366920938463463374607431768211456000000000000000000",
        );

        let request = HardwareReservationRequest::new(
            resource_identity(),
            quantity,
            "owner",
            "request.large",
            None,
        )
        .expect("large exact quantity must be supported");

        assert!(!request.quantity().is_zero());
    }

    #[test]
    fn preserves_fractional_resource_quantities() {
        let request = HardwareReservationRequest::new(
            resource_identity(),
            amount("0.125"),
            "owner",
            "request.fractional",
            None,
        )
        .expect("valid fractional quantity");

        assert_eq!(
            request.quantity().as_str(),
            "0.125",
        );
    }

    // ------------------------------------------------------------------------
    // Provider reservation references
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_open_world_provider_references() {
        let reference =
            HardwareReservationReference::new(
                "future.provider/partition/region/reservation/001",
            )
            .expect("valid opaque provider reference");

        assert_eq!(
            reference.as_str(),
            "future.provider/partition/region/reservation/001",
        );
    }

    #[test]
    fn rejects_empty_provider_references() {
        assert!(
            HardwareReservationReference::new("")
                .is_err()
        );
    }

    #[test]
    fn rejects_invalid_provider_references() {
        assert!(
            HardwareReservationReference::new(
                "reservation\ninvalid",
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Grant validation
    // ------------------------------------------------------------------------

    #[test]
    fn accepts_matching_confirmed_grant() {
        assert!(
            validate_grant(
                &request(),
                &grant("100"),
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_quantity_mismatch() {
        assert!(
            validate_grant(
                &request(),
                &grant("99"),
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_grant_for_another_resource() {
        let original = request();

        let other_resource = HardwareResourceIdentity::new(
            HardwareDeviceIdentity::new(
                provider_id(),
                DeviceId::new("another-device")
                    .expect("valid device identity"),
            ),
            HardwareResourceId::new("compute-unit")
                .expect("valid resource identity"),
        );

        let other_grant =
            HardwareReservationGrant::confirmed(
                provider_id(),
                other_resource,
                amount("100"),
                HardwareReservationReference::new(
                    "provider.reservation.002",
                )
                .expect("valid reservation reference"),
                Some(Duration::from_secs(60)),
            )
            .expect("valid grant");

        assert!(
            validate_grant(
                &original,
                &other_grant,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_grant_with_zero_quantity() {
        assert!(
            HardwareReservationGrant::confirmed(
                provider_id(),
                resource_identity(),
                amount("0"),
                HardwareReservationReference::new(
                    "reservation",
                )
                .expect("valid reference"),
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_grant_owned_by_another_provider() {
        let other_provider = ProviderId::new(
            "another.hardware.provider",
        )
        .expect("valid provider");

        assert!(
            HardwareReservationGrant::confirmed(
                other_provider,
                resource_identity(),
                amount("100"),
                HardwareReservationReference::new(
                    "reservation",
                )
                .expect("valid reference"),
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_zero_confirmed_lifetime() {
        assert!(
            HardwareReservationGrant::confirmed(
                provider_id(),
                resource_identity(),
                amount("100"),
                HardwareReservationReference::new(
                    "reservation",
                )
                .expect("valid reference"),
                Some(Duration::ZERO),
            )
            .is_err()
        );
    }

    // ------------------------------------------------------------------------
    // Identity and ownership
    // ------------------------------------------------------------------------

    #[test]
    fn request_provider_identity_matches_resource_owner() {
        let request = request();

        assert_eq!(
            request.provider_id(),
            &provider_id(),
        );
    }

    #[test]
    fn grant_preserves_provider_identity() {
        let grant = grant("100");

        assert_eq!(
            grant.provider_id(),
            &provider_id(),
        );
    }

    #[test]
    fn grant_preserves_reservation_reference() {
        let grant = grant("100");

        assert_eq!(
            grant.reference().as_str(),
            "provider.reservation.001",
        );
    }

    #[test]
    fn grant_preserves_confirmed_lifetime() {
        let grant = grant("100");

        assert_eq!(
            grant.granted_ttl(),
            Some(Duration::from_secs(60)),
        );
    }

    // ------------------------------------------------------------------------
    // Immutability and idempotency
    // ------------------------------------------------------------------------

    #[test]
    fn identical_requests_preserve_idempotency_identity() {
        let first = request();
        let second = request();

        assert_eq!(
            first.idempotency_key(),
            second.idempotency_key(),
        );

        assert_eq!(first, second);
    }

    #[test]
    fn different_request_parameters_remain_distinguishable() {
        let first = request();

        let second = HardwareReservationRequest::new(
            resource_identity(),
            amount("200"),
            "runtime.execution.001",
            "reservation.request.001",
            Some(Duration::from_secs(60)),
        )
        .expect("valid request");

        assert_ne!(first, second);
    }

    // ------------------------------------------------------------------------
    // Security
    // ------------------------------------------------------------------------

    #[test]
    fn reservation_reference_is_not_a_credential_container() {
        let reference =
            HardwareReservationReference::new(
                "provider/reservation/123",
            )
            .expect("valid reference");

        assert_eq!(
            reference.as_str(),
            "provider/reservation/123",
        );
    }
}
