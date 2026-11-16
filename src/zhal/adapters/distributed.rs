
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Distributed Hardware Adapter
//!
//! Path: src/zhal/adapters/distributed.rs
//!
//! Rust edition: 2021
//! Minimum supported Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! Provides the integration boundary between ZHAL hardware providers and
//! Zamani's canonical distributed membership and execution architecture.
//!
//! The adapter exposes an existing remote HardwareProvider through a
//! participant-bound ZHAL integration.
//!
//! It preserves:
//!
//! - Canonical hardware provider identity.
//! - Canonical hardware device identity.
//! - Distributed participant identity.
//! - Remote endpoint identity.
//! - Membership state.
//! - Membership lease validity.
//! - Hardware capability evidence.
//! - Hardware observations.
//! - Provider errors.
//! - Device session lifecycle.
//! - Cancellation and deadline semantics.
//!
//! This adapter does not implement another distributed execution engine.
//!
//! ============================================================================
//! ARCHITECTURAL AUTHORITIES
//! ============================================================================
//!
//! ZHAL:
//!     crate::zhal
//!
//! Hardware provider:
//!     crate::zhal::providers::contract
//!
//! Hardware device identity:
//!     crate::zhal::identity
//!
//! Hardware device descriptors:
//!     crate::zhal::model::device
//!
//! Hardware interface descriptors:
//!     crate::zhal::model::interface
//!
//! Hardware context:
//!     crate::zhal::context
//!
//! Hardware errors:
//!     crate::zhal::errors
//!
//! Provider identity and capability observations:
//!     crate::stdlib::provider
//!     crate::stdlib::capability
//!
//! Operation context:
//!     crate::stdlib::context
//!
//! Distributed participant identity:
//!     crate::distributed::ids
//!
//! Distributed membership:
//!     crate::distributed::membership
//!
//! Distributed transport:
//!     crate::distributed::transport
//!
//! Distributed execution:
//!     crate::distributed::execution
//!
//! Distributed planning:
//!     crate::distributed::planning
//!
//! Distributed errors:
//!     crate::distributed::error
//!
//! Runtime integration:
//!     crate::zur::distributed
//!
//! ============================================================================
//! OWNERSHIP
//! ============================================================================
//!
//! This adapter owns:
//!
//! - The participant-bound hardware-provider adapter.
//! - The remote participant/endpoint binding.
//! - Membership validation before hardware operations.
//! - Membership validation after asynchronous observations.
//! - Preservation of remote provider identity.
//! - Validation of returned device identities.
//! - Safe hardware-session cleanup after membership loss.
//!
//! This adapter DOES NOT own:
//!
//! - Provider identity.
//! - Provider lifecycle.
//! - Hardware device identity.
//! - Hardware discovery implementation.
//! - Participant identity.
//! - Membership registration.
//! - Participant discovery.
//! - Authentication.
//! - Authorization.
//! - Network endpoint resolution.
//! - Transport connections.
//! - Message serialization.
//! - Network routing.
//! - Distributed placement.
//! - Distributed scheduling.
//! - Distributed execution.
//! - Distributed resource allocation.
//! - Resource reservation.
//! - Checkpointing.
//! - Recovery.
//! - Hardware drivers.
//! - Classical IR.
//! - Quantum IR.
//! - HDLIR.
//! - ZUIR.
//! - Source-language semantics.
//!
//! These responsibilities remain with their existing authorities.
//!
//! ============================================================================
//! REMOTE PROVIDER MODEL
//! ============================================================================
//!
//! A DistributedHardwareAdapter wraps an existing HardwareProvider.
//!
//! The wrapped provider is the remote hardware communication implementation.
//!
//! It must use the canonical distributed/networking transport and the
//! applicable distributed security and execution contracts.
//!
//! This adapter does not invent a second remote hardware wire protocol.
//!
//! The adapter associates that provider with an explicitly registered
//! distributed participant and endpoint.
//!
//! The endpoint is descriptive membership information. It is not proof
//! of authentication, authorization or connectivity.
//!
//! ============================================================================
//! MEMBERSHIP INVARIANTS
//! ============================================================================
//!
//! Every hardware operation requires:
//!
//! 1. The participant to exist in the canonical membership registry.
//! 2. Its effective membership state to be Active.
//! 3. Its membership lease to remain valid.
//! 4. Its registered endpoints to contain the bound endpoint.
//! 5. The operation context to remain active.
//!
//! Membership is checked before and after asynchronous operations.
//!
//! Membership observations do not guarantee that a remote participant
//! remains reachable after the observation.
//!
//! The underlying provider must independently authenticate the remote
//! peer and enforce authorization at the protected execution boundary.
//!
//! ============================================================================
//! IDENTITY INVARIANTS
//! ============================================================================
//!
//! The wrapped provider remains the owner of its ProviderId.
//!
//! The adapter never manufactures a replacement provider identity.
//!
//! Every discovered or inspected device must belong to that provider.
//!
//! Remote participant identity and hardware device identity remain
//! separate identity authorities.
//!
//! A participant may expose multiple hardware devices.
//!
//! A device identity must not be confused with a participant identity.
//!
//! ============================================================================
//! SCALABILITY AND POCO-REAF
//! ============================================================================
//!
//! This adapter introduces no arbitrary universal ceilings on:
//!
//! - Distributed participants.
//! - Hardware providers.
//! - Hardware devices.
//! - Hardware interfaces.
//! - Hardware sessions.
//! - Hardware resources.
//! - Computational domains.
//! - Remote operations.
//! - Network endpoints.
//! - Hardware categories.
//! - Hardware architectures.
//!
//! It does not assume:
//!
//! - One participant equals one physical machine.
//! - One participant has one endpoint.
//! - One endpoint represents one device.
//! - One device represents one processor.
//! - One hardware architecture.
//! - One transport protocol.
//! - One computational domain.
//! - One fixed machine size.
//!
//! Actual capacity is determined by available resources, provider
//! implementations, configured operational budgets and platform limits.
//!
//! Literal infinite physical computation is not promised.
//!
//! ============================================================================
//! FAILURE SEMANTICS
//! ============================================================================
//!
//! Membership loss is not proof that a remote operation failed.
//!
//! A transport failure after dispatch may leave an operation outcome
//! unknown.
//!
//! This adapter never automatically retries an operation.
//!
//! It never converts an unknown remote outcome into success.
//!
//! It never claims that cancellation stopped physical execution.
//!
//! Provider and transport failures remain distinguishable from
//! membership failures.
//!
//! ============================================================================
//! SECURITY
//! ============================================================================
//!
//! Membership is not authentication.
//!
//! Authentication is not authorization.
//!
//! Endpoint possession is not authorization.
//!
//! This adapter stores no credentials or authentication secrets.
//!
//! Security decisions remain with distributed security and ZUR.
//!
//! ============================================================================
//! THREAD SAFETY
//! ============================================================================
//!
//! The wrapped provider is retained through Arc.
//!
//! MembershipRegistry is the existing synchronized registry.
//!
//! The adapter introduces no global mutable state.
//!
//! Hardware sessions retain their existing provider-defined lifecycle.
//!
//! ============================================================================
//! INTEGRATION
//! ============================================================================
//!
//! This adapter is intended to be exposed through:
//!
//!     crate::zhal::adapters::distributed
//!
//! The ZHAL composition root must expose the adapters module.
//!
//! Distributed execution remains owned by:
//!
//!     crate::distributed::execution
//!
//! Runtime orchestration remains owned by:
//!
//!     crate::zur::distributed
//!
//! ============================================================================

use std::fmt;
use std::sync::Arc;
use std::time::Instant;

use crate::stdlib::capability::CapabilitySnapshot;

use crate::stdlib::context::OperationContext;

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result as StdResult,
};

use crate::stdlib::provider::{
    Provider,
    ProviderDescriptor,
    ProviderFuture,
    ProviderStatus,
};

use crate::distributed::ids::ParticipantId;

use crate::distributed::membership::{
    MembershipRegistry,
    MembershipState,
    ParticipantEndpoint,
};

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::{
    HardwareDeviceIdentity,
    HardwareInterfaceIdentity,
};

use crate::zhal::model::device::{
    HardwareDeviceDescriptor,
};

use crate::zhal::model::interface::{
    HardwareInterfaceDescriptor,
};

use crate::zhal::providers::contract::{
    HardwareProvider,
    HardwareProviderFuture,
    HardwareSession,
    DeviceDiscoveryRequest,
    DeviceDiscoveryPage,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Canonical component identifier.
pub const COMPONENT: &str = "zhal.adapters.distributed";

/// Distributed hardware adapter contract identifier.
pub const ADAPTER_CONTRACT_ID: &str =
    "zamani.zhal.adapters.distributed";

/// Distributed hardware adapter contract major version.
pub const ADAPTER_CONTRACT_MAJOR: u16 = 1;

/// Distributed hardware adapter contract minor version.
pub const ADAPTER_CONTRACT_MINOR: u16 = 0;

/// Distributed hardware adapter contract patch version.
pub const ADAPTER_CONTRACT_PATCH: u16 = 0;

// ============================================================================
// DISTRIBUTED HARDWARE ADAPTER
// ============================================================================

/// A participant-bound adapter for an existing remote hardware provider.
///
/// The wrapped provider remains responsible for actual remote hardware
/// communication through its existing provider and distributed transport
/// integration.
///
/// The adapter adds canonical distributed membership validation.
///
/// # Type parameters
///
/// `P` is an existing implementation of the canonical ZHAL
/// [`HardwareProvider`] contract.
///
/// The adapter supports both concrete providers and dynamically dispatched
/// providers.
///
/// # Provider identity
///
/// The wrapped provider retains its original identity.
///
/// The adapter does not register a second provider identity.
///
/// # Lifecycle
///
/// Provider initialization and shutdown remain with the canonical
/// provider lifecycle manager.
///
/// # Security
///
/// Membership validation does not authenticate a remote participant.
///
/// The wrapped provider must enforce authentication and authorization.
///
/// # Examples
///
/// ```ignore
/// let adapter = DistributedHardwareAdapter::new(
///     Arc::clone(&remote_provider),
///     membership_registry.clone(),
///     participant_id,
///     remote_endpoint,
/// )?;
/// ```
pub struct DistributedHardwareAdapter<
    P: HardwareProvider + ?Sized,
> {
    provider: Arc<P>,

    membership: MembershipRegistry,

    participant_id: ParticipantId,

    endpoint: ParticipantEndpoint,
}

// ============================================================================
// DEBUG
// ============================================================================

impl<P> fmt::Debug for DistributedHardwareAdapter<P>
where
    P: HardwareProvider + ?Sized,
{
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("DistributedHardwareAdapter")
            .field(
                "provider_id",
                self.provider.descriptor().id(),
            )
            .field(
                "participant_id",
                &self.participant_id,
            )
            .field(
                "endpoint",
                &self.endpoint,
            )
            .finish_non_exhaustive()
    }
}

// ============================================================================
// CONSTRUCTION
// ============================================================================

impl<P> DistributedHardwareAdapter<P>
where
    P: HardwareProvider + ?Sized,
{
    /// Constructs a distributed hardware adapter.
    ///
    /// The provider must already be associated with the supplied remote
    /// participant and endpoint.
    ///
    /// Construction performs structural and current-membership validation.
    ///
    /// It does not:
    ///
    /// - Initialize the provider.
    /// - Register the participant.
    /// - Establish a transport connection.
    /// - Authenticate the remote participant.
    /// - Authorize hardware access.
    /// - Discover hardware.
    /// - Allocate resources.
    /// - Reserve devices.
    /// - Start remote execution.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The participant is not registered.
    /// - The participant is not Active.
    /// - Its membership lease has expired.
    /// - The endpoint is not registered for the participant.
    ///
    /// Provider identity remains owned by the wrapped provider.
    pub fn new(
        provider: Arc<P>,
        membership: MembershipRegistry,
        participant_id: ParticipantId,
        endpoint: ParticipantEndpoint,
    ) -> ZhalResult<Self> {
        let adapter = Self {
            provider,
            membership,
            participant_id,
            endpoint,
        };

        adapter.ensure_membership()?;

        Ok(adapter)
    }

    /// Returns the canonical remote hardware provider.
    ///
    /// This does not bypass the requirement that callers use the adapter's
    /// membership-aware operations when performing remote hardware access.
    #[must_use]
    pub fn hardware_provider(&self) -> &P {
        self.provider.as_ref()
    }

    /// Returns a shared handle to the canonical hardware provider.
    #[must_use]
    pub fn hardware_provider_handle(&self) -> Arc<P> {
        Arc::clone(&self.provider)
    }

    /// Returns the canonical distributed membership registry.
    ///
    /// The returned registry is a shared handle to the existing authority.
    #[must_use]
    pub fn membership_registry(&self) -> &MembershipRegistry {
        &self.membership
    }

    /// Returns the logical remote participant identity.
    #[must_use]
    pub const fn participant_id(&self) -> ParticipantId {
        self.participant_id
    }

    /// Returns the explicitly bound remote endpoint.
    #[must_use]
    pub fn endpoint(&self) -> &ParticipantEndpoint {
        &self.endpoint
    }

    /// Returns the canonical provider descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ProviderDescriptor {
        self.provider.descriptor()
    }

    /// Returns the underlying provider's current observed status.
    ///
    /// Provider status and distributed membership status are distinct.
    #[must_use]
    pub fn provider_status(&self) -> ProviderStatus {
        self.provider.status()
    }

    /// Returns whether the remote participant currently satisfies
    /// the adapter's membership requirements.
    ///
    /// This is an observation, not a guarantee of future reachability.
    #[must_use]
    pub fn is_participant_active(&self) -> bool {
        self.ensure_membership().is_ok()
    }

    /// Returns the stable adapter contract identifier.
    #[must_use]
    pub const fn adapter_contract_id(&self) -> &'static str {
        ADAPTER_CONTRACT_ID
    }

    /// Returns the adapter contract version.
    #[must_use]
    pub const fn adapter_contract_version(&self) -> (u16, u16, u16) {
        (
            ADAPTER_CONTRACT_MAJOR,
            ADAPTER_CONTRACT_MINOR,
            ADAPTER_CONTRACT_PATCH,
        )
    }
}

// ============================================================================
// MEMBERSHIP VALIDATION
// ============================================================================

impl<P> DistributedHardwareAdapter<P>
where
    P: HardwareProvider + ?Sized,
{
    /// Validates the current distributed membership binding.
    ///
    /// This method consults the canonical MembershipRegistry.
    ///
    /// It does not perform network communication or authentication.
    ///
    /// Membership leases use the registry's monotonic-clock semantics.
    ///
    /// # Errors
    ///
    /// Returns ProviderUnavailable when the participant is missing,
    /// leaving, suspected or unavailable.
    ///
    /// Returns ProviderUnavailable when the membership lease has expired.
    ///
    /// Returns ContractViolation when the bound endpoint is no longer
    /// registered for the participant.
    pub fn ensure_membership(&self) -> ZhalResult<()> {
        let record = self
            .membership
            .get(self.participant_id)
            .map_err(|error| {
                ZhalError::new(
                    ZhalErrorKind::ProviderUnavailable,
                    format!(
                        "distributed membership lookup failed: {error}"
                    ),
                )
                .in_component(COMPONENT)
            })?
            .ok_or_else(|| {
                ZhalError::new(
                    ZhalErrorKind::ProviderUnavailable,
                    "remote participant is not registered",
                )
                .in_component(COMPONENT)
            })?;

        let now = Instant::now();

        if record.effective_state_at(now) != MembershipState::Active {
            return Err(
                ZhalError::new(
                    ZhalErrorKind::ProviderUnavailable,
                    format!(
                        "remote participant {} is not active",
                        self.participant_id
                    ),
                )
                .in_component(COMPONENT),
            );
        }

        if !record.lease_is_valid_at(now) {
            return Err(
                ZhalError::new(
                    ZhalErrorKind::ProviderUnavailable,
                    "remote participant membership lease has expired",
                )
                .in_component(COMPONENT),
            );
        }

        if !record
            .endpoints()
            .iter()
            .any(|registered| registered == &self.endpoint)
        {
            return Err(
                ZhalError::new(
                    ZhalErrorKind::ContractViolation,
                    "bound remote endpoint is no longer registered for the participant",
                )
                .in_component(COMPONENT),
            );
        }

        Ok(())
    }

    /// Validates that a hardware device belongs to the wrapped provider.
    ///
    /// Distributed participant identity does not replace hardware
    /// provider-scoped device identity.
    ///
    /// # Errors
    ///
    /// Returns InvalidInput when the requested device belongs to
    /// another hardware provider.
    fn validate_device_identity(
        &self,
        identity: &HardwareDeviceIdentity,
    ) -> ZhalResult<()> {
        if identity.provider_id() != self.provider.descriptor().id() {
            return Err(
                ZhalError::new(
                    ZhalErrorKind::InvalidInput,
                    "hardware device belongs to another provider",
                )
                .in_component(COMPONENT),
            );
        }

        Ok(())
    }

    /// Converts a membership failure into the standard-library error
    /// contract required by the canonical Provider trait.
    fn membership_provider_error(
        &self,
        error: ZhalError,
    ) -> Error {
        Error::new(
            ErrorKind::Dependency,
            error.to_string(),
        )
        .in_component(COMPONENT)
        .during_operation("validate_remote_membership")
    }
}

// ============================================================================
// CANONICAL PROVIDER IMPLEMENTATION
// ============================================================================

impl<P> Provider for DistributedHardwareAdapter<P>
where
    P: HardwareProvider + ?Sized,
{
    /// Returns the wrapped provider's original descriptor.
    ///
    /// No replacement provider identity is created.
    fn descriptor(&self) -> &ProviderDescriptor {
        self.provider.descriptor()
    }

    /// Returns the wrapped provider's current status.
    ///
    /// Distributed membership is evaluated separately because provider
    /// readiness and participant membership are different authorities.
    fn status(&self) -> ProviderStatus {
        self.provider.status()
    }

    /// Obtains a capability snapshot from the original provider.
    ///
    /// The returned capability evidence remains owned by the provider
    /// and canonical capability subsystem.
    ///
    /// Membership is checked before and after the observation.
    ///
    /// No capability is manufactured by this adapter.
    fn capability_snapshot<'a>(
        &'a self,
        context: &'a OperationContext,
    ) -> ProviderFuture<'a, CapabilitySnapshot> {
        Box::pin(async move {
            self.ensure_membership()
                .map_err(|error| {
                    self.membership_provider_error(error)
                })?;

            let snapshot = self
                .provider
                .capability_snapshot(context)
                .await?;

            self.ensure_membership()
                .map_err(|error| {
                    self.membership_provider_error(error)
                })?;

            Ok(snapshot)
        })
    }
}

// ============================================================================
// HARDWARE PROVIDER IMPLEMENTATION
// ============================================================================

impl<P> HardwareProvider for DistributedHardwareAdapter<P>
where
    P: HardwareProvider + ?Sized,
{
    /// Discovers one page of remote hardware devices.
    ///
    /// The canonical provider remains responsible for:
    ///
    /// - Actual remote hardware discovery.
    /// - Transport communication.
    /// - Pagination.
    /// - Provider-specific device observation.
    /// - Capability evidence.
    /// - Resource observations.
    ///
    /// This adapter adds membership validation and delegates
    /// validated discovery to the wrapped provider.
    ///
    /// It does not aggregate all remote devices into memory.
    ///
    /// # Errors
    ///
    /// Preserves the wrapped provider's discovery errors.
    ///
    /// Returns a membership error if the remote participant is no longer
    /// eligible at either membership validation boundary.
    fn discover_devices_page<'a>(
        &'a self,
        request: &'a DeviceDiscoveryRequest,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, DeviceDiscoveryPage> {
        Box::pin(async move {
            context.check_active()?;

            self.ensure_membership()?;

            let page = self
                .provider
                .discover_devices(request, context)
                .await?;

            context.check_active()?;

            self.ensure_membership()?;

            Ok(page)
        })
    }

    /// Inspects one remote hardware device.
    ///
    /// The device identity must belong to the wrapped provider.
    ///
    /// The wrapped provider performs the actual remote inspection.
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - Invalid device ownership.
    /// - Invalid remote membership.
    /// - Invalid provider observations.
    /// - Transport failures.
    /// - Hardware failures.
    /// - Cancellation or deadline expiration.
    fn inspect_device<'a>(
        &'a self,
        identity: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareDeviceDescriptor> {
        Box::pin(async move {
            context.check_active()?;

            self.validate_device_identity(identity)?;

            self.ensure_membership()?;

            let descriptor = self
                .provider
                .inspect_device_validated(identity, context)
                .await?;

            context.check_active()?;

            self.ensure_membership()?;

            if descriptor.identity() != identity {
                return Err(
                    ZhalError::new(
                        ZhalErrorKind::ContractViolation,
                        "remote provider returned a different hardware device",
                    )
                    .in_component(COMPONENT),
                );
            }

            Ok(descriptor)
        })
    }

    /// Describes one remote hardware interface.
    ///
    /// Interface identity remains scoped to its canonical hardware device.
    ///
    /// The underlying provider performs the actual interface inspection.
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - Foreign provider identity.
    /// - Invalid membership.
    /// - Invalid interface observations.
    /// - Communication failure.
    /// - Cancellation or deadline expiration.
    fn describe_interface<'a>(
        &'a self,
        identity: &'a HardwareInterfaceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, HardwareInterfaceDescriptor> {
        Box::pin(async move {
            context.check_active()?;

            self.validate_device_identity(identity.device())?;

            self.ensure_membership()?;

            let descriptor = self
                .provider
                .describe_interface_validated(identity, context)
                .await?;

            context.check_active()?;

            self.ensure_membership()?;

            if descriptor.identity() != identity {
                return Err(
                    ZhalError::new(
                        ZhalErrorKind::ContractViolation,
                        "remote provider returned a different hardware interface",
                    )
                    .in_component(COMPONENT),
                );
            }

            Ok(descriptor)
        })
    }

    /// Opens a provider-managed session for a remote hardware device.
    ///
    /// The underlying provider remains responsible for:
    ///
    /// - Authentication.
    /// - Authorization.
    /// - Remote communication.
    /// - Hardware access.
    /// - Session establishment.
    /// - Provider-specific resource interaction.
    ///
    /// This adapter additionally verifies the participant membership
    /// and returned session/device association.
    ///
    /// If membership becomes invalid after session establishment,
    /// the adapter attempts to close the newly created session.
    ///
    /// If cleanup fails, the cleanup error is preserved.
    ///
    /// # Errors
    ///
    /// Returns an error when membership is invalid, access is denied,
    /// the device is unavailable, or session creation fails.
    fn open_session<'a>(
        &'a self,
        device: &'a HardwareDeviceIdentity,
        context: &'a ZhalContext,
    ) -> HardwareProviderFuture<'a, Box<dyn HardwareSession>> {
        Box::pin(async move {
            context.check_active()?;

            self.validate_device_identity(device)?;

            self.ensure_membership()?;

            let mut session = self
                .provider
                .open_session(device, context)
                .await?;

            context.check_active()?;

            if session.device_identity() != device {
                let close_result = session.close(context).await;

                if let Err(close_error) = close_result {
                    return Err(close_error);
                }

                return Err(
                    ZhalError::new(
                        ZhalErrorKind::ContractViolation,
                        "remote provider returned a session for a different device",
                    )
                    .in_component(COMPONENT),
                );
            }

            if let Err(membership_error) = self.ensure_membership() {
                return match session.close(context).await {
                    Ok(()) => Err(membership_error),
                    Err(close_error) => Err(close_error),
                };
            }

            Ok(session)
        })
    }
}

// ============================================================================
// STANDARD ERROR CONVERSION
// ============================================================================

/// Converts a ZHAL membership failure into a standard-library provider
/// dependency error.
///
/// The original structured ZHAL error remains available through the
/// direct ZHAL operation APIs.
fn membership_error_for_provider(
    error: ZhalError,
) -> Error {
    Error::new(
        ErrorKind::Dependency,
        error.to_string(),
    )
    .in_component(COMPONENT)
    .during_operation("distributed_hardware_membership")
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::distributed::membership::ParticipantEndpoint;

    #[test]
    fn adapter_contract_has_stable_identity() {
        assert_eq!(
            ADAPTER_CONTRACT_ID,
            "zamani.zhal.adapters.distributed",
        );

        assert_eq!(
            ADAPTER_CONTRACT_MAJOR,
            1,
        );
    }

    #[test]
    fn adapter_contract_version_is_independent() {
        let version = (
            ADAPTER_CONTRACT_MAJOR,
            ADAPTER_CONTRACT_MINOR,
            ADAPTER_CONTRACT_PATCH,
        );

        assert_eq!(version, (1, 0, 0));
    }

    #[test]
    fn endpoint_identity_is_preserved() {
        let endpoint = ParticipantEndpoint::new(
            "distributed://example/worker",
            "example.transport",
        )
        .expect("valid endpoint");

        assert_eq!(
            endpoint.uri(),
            "distributed://example/worker",
        );

        assert_eq!(
            endpoint.transport_provider(),
            "example.transport",
        );
    }

    #[test]
    fn endpoint_is_not_confused_with_provider_identity() {
        let endpoint = ParticipantEndpoint::new(
            "distributed://example/worker",
            "example.transport",
        )
        .expect("valid endpoint");

        assert_ne!(
            endpoint.uri(),
            endpoint.transport_provider(),
        );
    }

    #[test]
    fn endpoint_binding_preserves_exact_values() {
        let first = ParticipantEndpoint::new(
            "distributed://example/worker",
            "example.transport",
        )
        .expect("valid endpoint");

        let second = ParticipantEndpoint::new(
            "distributed://example/worker",
            "example.transport",
        )
        .expect("valid endpoint");

        assert_eq!(first, second);
    }
}
