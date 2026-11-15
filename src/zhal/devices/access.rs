
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Device Access
//!
//! Path: src/zhal/devices/access.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the production hardware-device access boundary
//! for the Zamani Universal Hardware Abstraction Layer.
//!
//! This module coordinates:
//!
//! - Explicit access requests.
//! - Principal identification.
//! - Existing standard-library authorization.
//! - Provider identity verification.
//! - Provider readiness verification.
//! - Fresh hardware-device inspection.
//! - Device availability verification.
//! - Provider-managed session creation.
//! - Session identity verification.
//! - Access provenance.
//! - Fail-closed error handling.
//!
//! Supported hardware domains are open-ended:
//!
//! - Classical computing.
//! - Quantum computing.
//! - HDL and programmable hardware.
//! - GPUs and accelerators.
//! - AI and tensor acceleration.
//! - Hybrid computing.
//! - Distributed computing.
//! - Embedded computing.
//! - Neuromorphic computing.
//! - Photonic computing.
//! - Analog computing.
//! - Future computational technologies.
//!
//! # Architectural ownership
//!
//! THIS MODULE OWNS:
//!
//! - HardwareAccessRequest.
//! - HardwareAccessService.
//! - HardwareAccessGrant.
//! - Access validation and coordination.
//! - Access provenance.
//!
//! THIS MODULE DOES NOT OWN:
//!
//! - Provider identity.
//! - Provider registration.
//! - Provider lifecycle.
//! - Provider authentication.
//! - Provider trust verification.
//! - Authorization policy evaluation.
//! - Principal authentication.
//! - Device discovery.
//! - Device inventory.
//! - Capability evaluation.
//! - Semantic requirements.
//! - Resource allocation.
//! - Resource reservation.
//! - Hardware scheduling.
//! - Hardware routing.
//! - Hardware execution semantics.
//! - Quantum execution.
//! - HDL realization.
//! - Distributed execution.
//! - Runtime orchestration.
//!
//! # Canonical repository authorities
//!
//! Provider identity:
//!     crate::stdlib::provider
//!
//! Hardware provider:
//!     crate::zhal::providers::contract
//!
//! Hardware session:
//!     crate::zhal::providers::contract
//!
//! Provider registration:
//!     crate::zhal::providers::registry
//!
//! Hardware identity:
//!     crate::zhal::identity
//!
//! Device description:
//!     crate::zhal::model::device
//!
//! Operation context:
//!     crate::zhal::context
//!
//! Structured errors:
//!     crate::zhal::errors
//!
//! Authorization:
//!     crate::stdlib::security::policy
//!
//! Principal identity:
//!     crate::stdlib::security::identity
//!
//! Resource allocation:
//!     Canonical ZUR resource-management authority.
//!
//! Quantum hardware:
//!     crate::quantum::hardware
//!
//! HDL realization:
//!     crate::hdl::realization
//!
//! Generic device I/O:
//!     crate::stdlib::io::devices
//!
//! # Access security
//!
//! Hardware access is deny-by-default.
//!
//! A hardware device identifier does not establish authorization.
//!
//! A discovery result does not establish authorization.
//!
//! A provider registration does not establish trust.
//!
//! A caller-supplied principal reference does not authenticate
//! the principal.
//!
//! Authentication and identity resolution must occur through
//! their existing security authorities before access is requested.
//!
//! Authorization is delegated to the canonical
//! stdlib::security::policy::AuthorizationService.
//!
//! Only an unconditional policy permit authorizes this module
//! to proceed to provider session establishment.
//!
//! Outstanding policy obligations are not automatically fulfilled.
//!
//! The provider must independently enforce its hardware access
//! and device-specific security requirements.
//!
//! # Access consistency
//!
//! Device availability is revalidated immediately before
//! authorization and session creation.
//!
//! The provider must perform its own final availability,
//! access-control and session-establishment checks.
//!
//! This avoids treating an earlier discovery or inspection
//! result as a permanent guarantee.
//!
//! Device availability observations can change concurrently.
//!
//! The provider's open_session_validated contract is the
//! final provider-facing validation boundary.
//!
//! # Resource ownership
//!
//! Opening a hardware session does not automatically reserve
//! computational resources.
//!
//! It does not allocate memory, processors, accelerators,
//! qubits, FPGA resources or distributed execution capacity.
//!
//! Resource allocation and reservation remain owned by ZUR
//! and the applicable specialized provider contracts.
//!
//! # Domain independence
//!
//! This module does not introduce a universal instruction API.
//!
//! A successful session can be used by the corresponding
//! specialized domain interface.
//!
//! Examples:
//!
//! - Classical execution through classical/backend interfaces.
//! - Quantum execution through quantum::hardware.
//! - HDL realization through hdl::realization.
//! - Accelerator execution through accelerator providers.
//! - Distributed hardware through distributed providers.
//!
//! # POCO-REAF
//!
//! No arbitrary hardware ceilings are introduced.
//!
//! There are no fixed limits on:
//!
//! - Hardware devices.
//! - Providers.
//! - Sessions.
//! - Processors.
//! - Accelerators.
//! - QPUs.
//! - Qubits.
//! - Memory.
//! - Hardware categories.
//! - Computational domains.
//! - Device identifier lengths.
//! - Provider identifier lengths.
//!
//! Actual operation is governed by available memory,
//! platform constraints, provider capabilities and
//! explicitly configured operational policies.
//!
//! Literal infinite physical resources are not promised.
//!
//! # Grammar boundary
//!
//! Relevant grammar:
//!
//! - grammar/hardware/devices.g4
//! - grammar/hardware/hardware.g4
//! - grammar/effects/hardware.g4
//! - grammar/effects/security.g4
//! - grammar/execution/execution-context.g4
//! - grammar/execution/distributed-execution.g4
//!
//! Relevant normative specifications:
//!
//! - grammar/specification/poco-reaf.md
//! - grammar/specification/semantic-model.md
//! - grammar/specification/scalability-model.md
//! - grammar/specification/execution-model.md
//! - grammar/validation/semantic-boundaries.md
//!
//! Source grammar describes portable computational intent.
//!
//! This module consumes already validated identity and
//! authorization information.
//!
//! It does not parse source syntax or perform semantic analysis.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.

use std::fmt;
use std::sync::Arc;

use crate::stdlib::provider::{
    ProviderStatus,
};

use crate::stdlib::security::identity::{
    PrincipalId,
};

use crate::stdlib::security::policy::{
    AuthorizationReceipt,
    AuthorizationRequest,
    AuthorizationService,
    PolicyAction,
    PolicyError,
    PolicyResource,
};

use crate::zhal::context::ZhalContext;

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

use crate::zhal::identity::{
    HardwareDeviceIdentity,
};

use crate::zhal::model::device::{
    HardwareDeviceDescriptor,
};

use crate::zhal::providers::contract::{
    HardwareProvider,
    HardwareSession,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable hardware-access component identifier.
pub const COMPONENT: &str = "zhal.devices.access";

/// Authorization action used for hardware-device access.
pub const HARDWARE_DEVICE_ACCESS_ACTION: &str =
    "hardware.device.access";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn access_error(
    kind: ZhalErrorKind,
    message: impl Into<String>,
) -> ZhalError {
    ZhalError::new(kind, message)
        .in_component(COMPONENT)
}

fn invalid_input(
    message: impl Into<String>,
) -> ZhalError {
    access_error(
        ZhalErrorKind::InvalidInput,
        message,
    )
}

fn contract_violation(
    message: impl Into<String>,
) -> ZhalError {
    access_error(
        ZhalErrorKind::ContractViolation,
        message,
    )
}

// ============================================================================
// REQUEST VALIDATION
// ============================================================================

/// Immutable request to access a provider-owned hardware device.
///
/// The request contains:
///
/// - The canonical provider-scoped device identity.
/// - An authenticated principal reference.
/// - The authorization action.
///
/// The principal must already have been authenticated and resolved
/// through the canonical identity authority.
///
/// Merely constructing this request does not authenticate anyone
/// or grant access.
///
/// The authorization service evaluates the request using its
/// configured policy evaluator and audit sink.
///
/// No credentials or secrets are stored in this structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HardwareAccessRequest {
    device: HardwareDeviceIdentity,
    principal: PrincipalId,
    action: PolicyAction,
}

impl HardwareAccessRequest {
    /// Creates a hardware access request.
    ///
    /// # Arguments
    ///
    /// `device`:
    ///     Provider-scoped hardware device identity.
    ///
    /// `principal`:
    ///     Previously authenticated principal identity.
    ///
    /// `action`:
    ///     Open-world authorization action.
    ///
    /// # Errors
    ///
    /// Returns an error if the device identity is malformed
    /// or the action cannot be represented by the policy authority.
    pub fn new(
        device: HardwareDeviceIdentity,
        principal: PrincipalId,
        action: PolicyAction,
    ) -> ZhalResult<Self> {
        if device.provider_id().as_str().is_empty()
            || device.device_id().as_str().is_empty()
        {
            return Err(invalid_input(
                "hardware access requires a valid provider-scoped device identity",
            ));
        }

        Ok(Self {
            device,
            principal,
            action,
        })
    }

    /// Creates a standard hardware-device access request.
    ///
    /// The action is the canonical ZHAL hardware-device access action.
    ///
    /// # Errors
    ///
    /// Returns an error if the policy action cannot be constructed.
    pub fn device_access(
        device: HardwareDeviceIdentity,
        principal: PrincipalId,
    ) -> ZhalResult<Self> {
        let action = PolicyAction::new(
            HARDWARE_DEVICE_ACCESS_ACTION,
        )
        .map_err(|_| {
            invalid_input(
                "the standard hardware access policy action is invalid",
            )
        })?;

        Self::new(
            device,
            principal,
            action,
        )
    }

    /// Returns the requested hardware device.
    #[must_use]
    pub fn device(&self) -> &HardwareDeviceIdentity {
        &self.device
    }

    /// Returns the authenticated principal reference.
    #[must_use]
    pub fn principal(&self) -> &PrincipalId {
        &self.principal
    }

    /// Returns the authorization action.
    #[must_use]
    pub fn action(&self) -> &PolicyAction {
        &self.action
    }
}

// ============================================================================
// ACCESS GRANT
// ============================================================================

/// Successful, authorized provider-managed hardware access.
///
/// This grant contains:
///
/// - The provider-scoped device identity.
/// - The freshly inspected device descriptor.
/// - The successful authorization receipt.
/// - The provider-managed hardware session.
///
/// The grant is not a resource reservation.
///
/// It does not guarantee that the hardware remains available
/// indefinitely.
///
/// It does not imply that domain-specific operations are supported.
///
/// The underlying provider session remains responsible for
/// actual hardware access and session lifecycle.
pub struct HardwareAccessGrant {
    device: HardwareDeviceDescriptor,
    authorization: AuthorizationReceipt,
    session: Box<dyn HardwareSession>,
}

impl HardwareAccessGrant {
    /// Returns the freshly inspected hardware descriptor.
    #[must_use]
    pub fn device(&self) -> &HardwareDeviceDescriptor {
        &self.device
    }

    /// Returns the provider-scoped device identity.
    #[must_use]
    pub fn device_identity(&self) -> &HardwareDeviceIdentity {
        self.device.identity()
    }

    /// Returns the authorization receipt.
    ///
    /// The receipt contains the policy decision and audit status
    /// established by the canonical security authority.
    #[must_use]
    pub fn authorization(&self) -> &AuthorizationReceipt {
        &self.authorization
    }

    /// Returns the provider-managed hardware session identity.
    #[must_use]
    pub fn session_identity(
        &self,
    ) -> &crate::zhal::identity::HardwareSessionIdentity {
        self.session.identity()
    }

    /// Returns whether the underlying session has been closed.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.session.is_closed()
    }

    /// Returns an immutable reference to the provider-managed session.
    ///
    /// Specialized domain adapters may use this reference to
    /// associate the session with their existing execution contracts.
    #[must_use]
    pub fn session(&self) -> &dyn HardwareSession {
        self.session.as_ref()
    }

    /// Returns a mutable reference to the provider-managed session.
    ///
    /// Mutable access is required for the canonical asynchronous
    /// session-close operation.
    #[must_use]
    pub fn session_mut(&mut self) -> &mut dyn HardwareSession {
        self.session.as_mut()
    }

    /// Consumes the grant and returns its provider-managed session.
    ///
    /// The caller becomes responsible for preserving the session's
    /// lifecycle and closing it through the provider contract.
    #[must_use]
    pub fn into_session(self) -> Box<dyn HardwareSession> {
        self.session
    }

    /// Returns the authorization receipt and device descriptor,
    /// consuming the grant's session.
    ///
    /// This method is useful when a specialized adapter takes
    /// ownership of the session separately.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        HardwareDeviceDescriptor,
        AuthorizationReceipt,
        Box<dyn HardwareSession>,
    ) {
        (
            self.device,
            self.authorization,
            self.session,
        )
    }
}

impl fmt::Debug for HardwareAccessGrant {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("HardwareAccessGrant")
            .field("device", &self.device.identity())
            .field(
                "session",
                &self.session.identity(),
            )
            .field(
                "session_closed",
                &self.session.is_closed(),
            )
            .field(
                "authorized",
                &self.authorization.is_authorized(),
            )
            .finish()
    }
}

// ============================================================================
// HARDWARE ACCESS SERVICE
// ============================================================================

/// Coordinates secure access to universal hardware devices.
///
/// This service is explicitly constructed with the canonical
/// standard-library authorization service.
///
/// No implicit allow-all policy is provided.
///
/// No implicit authentication mechanism is provided.
///
/// Hardware providers are supplied as typed HardwareProvider
/// trait objects.
///
/// This is deliberate: the existing ProviderRegistry stores
/// Arc<dyn Provider>, which cannot safely be downcast into
/// Arc<dyn HardwareProvider>.
///
/// Until the registry offers a typed hardware-provider handle,
/// the application or runtime adapter must retain the original
/// typed provider reference.
///
/// The service does not own the provider registry or lifecycle.
///
/// # Thread safety
///
/// The service is shareable across threads.
///
/// Authorization and provider operations are delegated to
/// their respective thread-safe authorities.
#[derive(Clone)]
pub struct HardwareAccessService {
    authorization: Arc<AuthorizationService>,
}

impl HardwareAccessService {
    /// Creates a hardware access service.
    ///
    /// The canonical authorization service is mandatory.
    ///
    /// The authorization service must be configured with:
    ///
    /// - An authoritative policy evaluator.
    /// - An audit sink.
    ///
    /// Its default audit policy requires successful audit recording.
    ///
    /// # Security
    ///
    /// There is deliberately no constructor without authorization.
    #[must_use]
    pub fn new(
        authorization: Arc<AuthorizationService>,
    ) -> Self {
        Self {
            authorization,
        }
    }

    /// Returns the canonical authorization service.
    #[must_use]
    pub fn authorization_service(
        &self,
    ) -> &AuthorizationService {
        self.authorization.as_ref()
    }

    /// Opens an authorized hardware-device session.
    ///
    /// Processing order:
    ///
    /// 1. Validate the operation context.
    /// 2. Verify provider identity.
    /// 3. Verify provider operational status.
    /// 4. Verify context/provider association.
    /// 5. Inspect the requested physical device.
    /// 6. Validate the fresh device descriptor.
    /// 7. Verify exact device identity and ownership.
    /// 8. Require an explicit available observation.
    /// 9. Evaluate and audit authorization.
    /// 10. Establish the provider-managed session.
    /// 11. Validate the returned session.
    ///
    /// The provider performs its own final access-control
    /// and availability checks inside open_session_validated.
    ///
    /// A device observation does not constitute a reservation.
    ///
    /// # Errors
    ///
    /// Returns a structured error for:
    ///
    /// - Cancellation.
    /// - Deadline expiration.
    /// - Invalid identity.
    /// - Provider identity mismatch.
    /// - Provider unavailability.
    /// - Device disappearance.
    /// - Device unavailability.
    /// - Unknown device availability.
    /// - Authorization denial.
    /// - Authorization evaluation failure.
    /// - Provider failure.
    /// - Invalid provider session.
    ///
    /// No failure is silently converted into successful access.
    pub async fn open(
        &self,
        provider: Arc<dyn HardwareProvider>,
        request: &HardwareAccessRequest,
        context: &ZhalContext,
    ) -> ZhalResult<HardwareAccessGrant> {
        // --------------------------------------------------------------------
        // 1. Operation context
        // --------------------------------------------------------------------

        context.check_active().map_err(|_| {
            if context.is_cancelled() {
                access_error(
                    ZhalErrorKind::Cancelled,
                    "hardware access was cancelled",
                )
            } else {
                access_error(
                    ZhalErrorKind::OperationTimedOut,
                    "hardware access deadline has expired",
                )
            }
        })?;

        // --------------------------------------------------------------------
        // 2. Provider identity
        // --------------------------------------------------------------------

        let provider_descriptor = provider.descriptor();

        let provider_id = provider_descriptor.id();

        if request.device().provider_id() != provider_id {
            return Err(
                invalid_input(
                    "the requested device does not belong to the supplied hardware provider",
                ),
            );
        }

        // --------------------------------------------------------------------
        // 3. Provider implementation integrity
        // --------------------------------------------------------------------

        if provider.descriptor().id() != provider_id {
            return Err(
                contract_violation(
                    "hardware provider identity changed during access validation",
                ),
            );
        }

        // --------------------------------------------------------------------
        // 4. Provider readiness
        // --------------------------------------------------------------------

        let provider_status = provider.status();

        if !provider_status.may_accept_operations() {
            return Err(
                access_error(
                    ZhalErrorKind::ProviderUnavailable,
                    format!(
                        "hardware provider {} is not accepting operations: {}",
                        provider_id,
                        provider_status.as_str(),
                    ),
                ),
            );
        }

        // --------------------------------------------------------------------
        // 5. Context/provider association
        // --------------------------------------------------------------------

        if let Some(context_provider) = context.provider_id() {
            if context_provider != provider_id {
                return Err(
                    invalid_input(
                        "operation context is associated with a different hardware provider",
                    ),
                );
            }
        }

        context.check_active().map_err(|_| {
            access_error(
                ZhalErrorKind::Cancelled,
                "hardware access was cancelled before device inspection",
            )
        })?;

        // --------------------------------------------------------------------
        // 6. Fresh device inspection
        // --------------------------------------------------------------------

        let descriptor = provider
            .inspect_device_validated(
                request.device(),
                context,
            )
            .await?;

        context.check_active().map_err(|_| {
            access_error(
                ZhalErrorKind::Cancelled,
                "hardware access was cancelled after device inspection",
            )
        })?;

        // --------------------------------------------------------------------
        // 7. Device identity and ownership
        // --------------------------------------------------------------------

        descriptor.validate().map_err(|_| {
            access_error(
                ZhalErrorKind::InvalidData,
                "hardware provider returned an invalid device descriptor",
            )
        })?;

        if descriptor.identity() != request.device() {
            return Err(
                contract_violation(
                    "hardware provider returned a different device than requested",
                ),
            );
        }

        if descriptor.provider_id() != provider_id {
            return Err(
                contract_violation(
                    "hardware device descriptor belongs to a different provider",
                ),
            );
        }

        // --------------------------------------------------------------------
        // 8. Availability
        // --------------------------------------------------------------------

        // Unknown is deliberately NOT treated as available.
        //
        // The provider must supply an explicit positive observation.
        // The provider must still revalidate availability when opening
        // the actual hardware session.

        if !descriptor.availability().is_available() {
            let availability = descriptor.availability();

            let (kind, message) =
                if availability.is_unavailable() {
                    (
                        ZhalErrorKind::DeviceUnavailable,
                        "the requested hardware device is unavailable",
                    )
                } else if availability.is_unknown() {
                    (
                        ZhalErrorKind::DeviceInaccessible,
                        "hardware device availability has not been established",
                    )
                } else {
                    (
                        ZhalErrorKind::DeviceInaccessible,
                        "the hardware device is not explicitly reported as available",
                    )
                };

            return Err(
                access_error(kind, message),
            );
        }

        // --------------------------------------------------------------------
        // 9. Authorization request
        // --------------------------------------------------------------------

        context.check_active().map_err(|_| {
            access_error(
                ZhalErrorKind::Cancelled,
                "hardware access was cancelled before authorization",
            )
        })?;

        let resource_reference = format!(
            "hardware::device::{}::{}",
            request.device().provider_id(),
            request.device().device_id(),
        );

        let resource = PolicyResource::new(
            resource_reference,
        )
        .map_err(|_| {
            invalid_input(
                "hardware device could not be represented as a policy resource",
            )
        })?;

        let authorization_request =
            AuthorizationRequest::new(
                request.principal().clone(),
                request.action().clone(),
            )
            .with_resource(resource);

        // The canonical policy authority performs:
        //
        // - Policy evaluation.
        // - Permit/deny/indeterminate handling.
        // - Outstanding-obligation rejection.
        // - Audit recording.
        //
        // No local allow-all fallback exists.

        let authorization = self
            .authorization
            .authorize(&authorization_request)
            .map_err(|error: PolicyError| {
                access_error(
                    ZhalErrorKind::PermissionDenied,
                    format!(
                        "hardware access authorization was not established: {error}",
                    ),
                )
            })?;

        if !authorization.is_authorized() {
            return Err(
                access_error(
                    ZhalErrorKind::PermissionDenied,
                    "the authorization authority did not establish an unconditional permit",
                ),
            );
        }

        // --------------------------------------------------------------------
        // 10. Final context validation
        // --------------------------------------------------------------------

        context.check_active().map_err(|_| {
            access_error(
                ZhalErrorKind::Cancelled,
                "hardware access was cancelled before session establishment",
            )
        })?;

        // --------------------------------------------------------------------
        // 11. Establish the provider-managed session
        // --------------------------------------------------------------------

        // The provider contract requires a final device-availability
        // check and provider-side access enforcement.
        //
        // Discovery and inspection alone never authorize access.

        let mut session = provider
            .open_session_validated(
                request.device(),
                context,
            )
            .await?;

        // --------------------------------------------------------------------
        // 12. Session identity verification
        // --------------------------------------------------------------------

        let session_provider =
            session.identity().provider_id();

        let session_device =
            session.device_identity();

        if session_provider != provider_id
            || session_device != request.device()
        {
            // An invalid session must not be returned to the caller.
            //
            // Attempt cleanup even if cancellation has occurred.
            // Cleanup is a lifecycle operation, not a new workload.

            let _ = session.close(context).await;

            return Err(
                contract_violation(
                    "hardware provider returned a session with an inconsistent provider or device identity",
                ),
            );
        }

        if session.is_closed() {
            return Err(
                contract_violation(
                    "hardware provider returned an already closed hardware session",
                ),
            );
        }

        // --------------------------------------------------------------------
        // 13. Return the authorized grant
        // --------------------------------------------------------------------

        Ok(HardwareAccessGrant {
            device: descriptor,
            authorization,
            session,
        })
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stdlib::io::devices::DeviceId;

    use crate::stdlib::provider::ProviderId;

    fn device_identity() -> HardwareDeviceIdentity {
        HardwareDeviceIdentity::new(
            ProviderId::new(
                "org.example.hardware",
            )
            .expect("valid provider identity"),
            DeviceId::new(
                "device.processor.001",
            )
            .expect("valid device identity"),
        )
    }

    fn principal() -> PrincipalId {
        PrincipalId::new(
            "org.example.user",
        )
        .expect("valid principal identity")
    }

    #[test]
    fn standard_device_access_request_uses_canonical_action() {
        let request = HardwareAccessRequest::device_access(
            device_identity(),
            principal(),
        )
        .expect("valid access request");

        assert_eq!(
            request.action().as_str(),
            HARDWARE_DEVICE_ACCESS_ACTION,
        );
    }

    #[test]
    fn access_request_preserves_provider_scoped_identity() {
        let identity = device_identity();

        let request = HardwareAccessRequest::device_access(
            identity.clone(),
            principal(),
        )
        .expect("valid access request");

        assert_eq!(
            request.device(),
            &identity,
        );
    }

    #[test]
    fn access_request_preserves_principal() {
        let principal = principal();

        let request = HardwareAccessRequest::device_access(
            device_identity(),
            principal.clone(),
        )
        .expect("valid access request");

        assert_eq!(
            request.principal(),
            &principal,
        );
    }

    #[test]
    fn access_request_supports_open_world_actions() {
        let action = PolicyAction::new(
            "future.photonic.hardware.access",
        )
        .expect("valid future action");

        let request = HardwareAccessRequest::new(
            device_identity(),
            principal(),
            action,
        )
        .expect("valid access request");

        assert_eq!(
            request.action().as_str(),
            "future.photonic.hardware.access",
        );
    }

    #[test]
    fn standard_access_action_is_stable() {
        assert_eq!(
            HARDWARE_DEVICE_ACCESS_ACTION,
            "hardware.device.access",
        );
    }
}
