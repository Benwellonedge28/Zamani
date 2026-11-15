
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Isolation Integration
//!
//! Path: `src/zhal/security/isolation.rs`
//!
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the ZHAL-facing integration boundary for execution
//! isolation across physical and virtual computational hardware.
//!
//! ZHAL delegates actual isolation establishment, verification,
//! session management and cleanup to the canonical ZUR isolation
//! subsystem.
//!
//! This module does not implement a competing isolation engine.
//!
//! # Supported computational domains
//!
//! Isolation can protect execution involving:
//!
//! - Classical processors.
//! - Memory and storage.
//! - Quantum processors and simulators.
//! - FPGA and ASIC realization.
//! - GPUs and other accelerators.
//! - AI and tensor computation.
//! - Embedded hardware.
//! - Hybrid execution.
//! - Distributed execution.
//! - Networking hardware.
//! - Neuromorphic hardware.
//! - Photonic and analog computing.
//! - Future computational technologies.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - The ZHAL-to-ZUR isolation adapter.
//! - Hardware-facing isolation establishment.
//! - Isolation error translation.
//! - Hardware isolation lease access.
//! - Isolation session inspection.
//! - Explicit isolation cleanup integration.
//!
//! This module does NOT own:
//!
//! - Source-language sandbox syntax.
//! - AST construction.
//! - Semantic security analysis.
//! - Isolation requirement interpretation.
//! - Authentication.
//! - Authorization.
//! - Trust establishment.
//! - Credential management.
//! - Cryptographic implementations.
//! - Provider registration.
//! - Provider discovery.
//! - Hardware selection.
//! - Resource allocation.
//! - Process sandbox implementations.
//! - Container implementations.
//! - Virtual-machine implementations.
//! - Enclave implementations.
//! - Operating-system isolation.
//! - Security policy evaluation.
//! - Runtime scheduling.
//! - Hardware execution.
//!
//! # Canonical authorities
//!
//! Actual isolation enforcement:
//!
//!     crate::zur::security::isolation
//!
//! Provider trust:
//!
//!     crate::zhal::security::trust
//!
//! Authorization:
//!
//!     crate::zhal::security::authorization
//!
//! Runtime permission enforcement:
//!
//!     crate::zur::security::permissions
//!
//! Runtime security policy:
//!
//!     crate::zur::security::policy
//!
//! Distributed authentication:
//!
//!     crate::distributed::security
//!
//! Source-level sandbox intent:
//!
//!     grammar/security/sandbox.g4
//!
//! Security constraints:
//!
//!     grammar/security/security-constraints.g4
//!
//! ZHAL error authority:
//!
//!     crate::zhal::errors
//!
//! # Security invariants
//!
//! 1. ZHAL does not manufacture isolation evidence.
//! 2. ZHAL does not establish isolation merely by registering a provider.
//! 3. Actual isolation must be established by an isolation provider.
//! 4. Mandatory isolation controls require affirmative evidence.
//! 5. Unknown isolation evidence never satisfies a mandatory control.
//! 6. Prohibited isolation controls must be affirmatively disabled.
//! 7. An inactive session is never accepted as established isolation.
//! 8. Provider identity mismatches are rejected.
//! 9. Failed isolation establishment never becomes success.
//! 10. Isolation does not imply authorization.
//! 11. Provider trust does not imply isolation.
//! 12. Authorization does not imply isolation.
//! 13. Isolation does not imply hardware availability.
//! 14. Explicit cleanup failures remain observable.
//! 15. Dropping a lease delegates best-effort cleanup to ZUR.
//! 16. No isolation provider is implicitly trusted.
//! 17. No fallback silently weakens mandatory isolation.
//!
//! # POCO-REAF and scalability
//!
//! This module introduces no arbitrary maximum on:
//!
//! - Isolation providers.
//! - Isolation sessions.
//! - Isolation controls.
//! - Isolation directives.
//! - Hardware devices.
//! - Computational domains.
//! - Distributed participants.
//! - Machines.
//! - Processors.
//! - Accelerators.
//! - Quantum processors.
//! - Hardware technologies.
//!
//! Isolation capacity depends on actual platform capabilities,
//! available resources and explicitly configured operational
//! policies.
//!
//! Literal infinite physical capacity is not promised.
//!
//! # Concurrency
//!
//! The canonical ZUR IsolationManager is shared through Arc.
//!
//! This module introduces:
//!
//! - No global mutable state.
//! - No hidden singleton.
//! - No competing provider registry.
//! - No implicit background threads.
//! - No local isolation-session cache.
//!
//! Provider and session thread-safety guarantees remain those
//! established by the canonical ZUR isolation contracts.
//!
//! # Cancellation
//!
//! Cancellation remains owned by the existing runtime and
//! standard-library context authorities.
//!
//! This adapter does not invent a second cancellation token
//! or cancellation state machine.
//!
//! Closing an isolation lease is not equivalent to cancelling
//! the protected hardware operation.
//!
//! # Grammar boundary
//!
//! The frontend semantic layer must translate validated
//! source-level sandbox intent into:
//!
//!     crate::zur::security::isolation::IsolationRequest
//!
//! This module does not parse grammar productions or interpret
//! source-language expressions.
//!
//! # Important execution rule
//!
//! Protected execution must not begin merely because this
//! adapter exists.
//!
//! The runtime composition must establish all applicable
//! security controls, including:
//!
//! - Provider trust.
//! - Principal authentication.
//! - Authorization.
//! - Required isolation.
//! - Artifact integrity where applicable.
//!
//! These controls remain independent and complementary.

use core::fmt;
use std::any::Any;
use std::sync::Arc;

use crate::zur::security::isolation::{
    IsolationControlState,
    IsolationDisposition,
    IsolationDirective,
    IsolationError,
    IsolationErrorKind,
    IsolationEvidence,
    IsolationIdentifier,
    IsolationLease,
    IsolationManager,
    IsolationProvider,
    IsolationReport,
    IsolationRequest,
    IsolationResult,
    IsolationSession,
};

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable ZHAL isolation component identifier.
pub const COMPONENT: &str = "zhal.security.isolation";

/// Version of the ZHAL isolation integration contract.
pub const ISOLATION_SCHEMA_VERSION: &str = "1";

// ============================================================================
// CANONICAL TYPE EXPOSURE
// ============================================================================

/// Open-world isolation identifier.
pub use crate::zur::security::isolation::IsolationIdentifier
    as HardwareIsolationIdentifier;

/// Required, preferred or prohibited isolation disposition.
pub use crate::zur::security::isolation::IsolationDisposition
    as HardwareIsolationDisposition;

/// Isolation directive.
pub use crate::zur::security::isolation::IsolationDirective
    as HardwareIsolationDirective;

/// Validated isolation request.
pub use crate::zur::security::isolation::IsolationRequest
    as HardwareIsolationRequest;

/// Provider-reported isolation state.
pub use crate::zur::security::isolation::IsolationControlState
    as HardwareIsolationControlState;

/// Provider-reported isolation evidence.
pub use crate::zur::security::isolation::IsolationEvidence
    as HardwareIsolationEvidence;

/// Provider-managed isolation session.
pub use crate::zur::security::isolation::IsolationSession
    as HardwareIsolationSession;

/// Provider isolation contract.
pub use crate::zur::security::isolation::IsolationProvider
    as HardwareIsolationProvider;

/// Verified isolation report.
pub use crate::zur::security::isolation::IsolationReport
    as HardwareIsolationReport;

// ============================================================================
// ERROR TRANSLATION
// ============================================================================

/// Converts a canonical ZUR isolation error into the ZHAL error model.
///
/// The original isolation error is retained as the causal source.
///
/// Error translation never converts an isolation failure into
/// successful isolation.
///
/// The mapping deliberately distinguishes unsupported requirements
/// from indeterminate evidence and provider contract violations.
fn translate_isolation_error(
    error: IsolationError,
) -> ZhalError {
    let kind = match error.kind() {
        IsolationErrorKind::InvalidRequest => {
            ZhalErrorKind::InvalidInput
        }

        IsolationErrorKind::ProviderFailure => {
            ZhalErrorKind::ProviderFailure
        }

        IsolationErrorKind::ProviderIdentityMismatch => {
            ZhalErrorKind::ContractViolation
        }

        IsolationErrorKind::SessionInactive => {
            ZhalErrorKind::ContractViolation
        }

        IsolationErrorKind::RequiredControlUnsatisfied
        | IsolationErrorKind::ProhibitedControlEnabled => {
            ZhalErrorKind::IncompatibleWorkload
        }

        IsolationErrorKind::ControlStateIndeterminate => {
            ZhalErrorKind::CapabilityUnknown
        }

        IsolationErrorKind::CleanupFailed => {
            ZhalErrorKind::ProviderFailure
        }
    };

    let message = match error.kind() {
        IsolationErrorKind::InvalidRequest => {
            "the isolation request is invalid"
        }

        IsolationErrorKind::ProviderFailure => {
            "the isolation provider failed"
        }

        IsolationErrorKind::ProviderIdentityMismatch => {
            "the isolation provider identity does not match its evidence"
        }

        IsolationErrorKind::SessionInactive => {
            "the isolation provider returned an inactive session"
        }

        IsolationErrorKind::RequiredControlUnsatisfied => {
            "a mandatory hardware isolation control is unsatisfied"
        }

        IsolationErrorKind::ProhibitedControlEnabled => {
            "a prohibited hardware isolation control is enabled"
        }

        IsolationErrorKind::ControlStateIndeterminate => {
            "the required hardware isolation state cannot be established"
        }

        IsolationErrorKind::CleanupFailed => {
            "hardware isolation session cleanup failed"
        }
    };

    let mut translated = ZhalError::new(kind, message)
        .in_component(COMPONENT)
        .with_source(error);

    // Preserve the relevant control identifier as diagnostic context.
    //
    // The control name is descriptive metadata, not secret material.
    if let Some(source) = translated.source_error() {
        if let Some(isolation_error) =
            source.downcast_ref::<IsolationError>()
        {
            if let Some(control) = isolation_error.control() {
                translated = translated.with_context(
                    "isolation_control",
                    control.as_str(),
                ).unwrap_or(translated);
            }
        }
    }

    translated
}

// ============================================================================
// HARDWARE ISOLATION SERVICE
// ============================================================================

/// ZHAL integration service for canonical runtime isolation.
///
/// This service does not implement isolation itself.
///
/// It delegates isolation establishment and verification to
/// the explicitly injected ZUR IsolationManager.
///
/// The supplied manager must already have been constructed with
/// an appropriately admitted and trusted isolation provider.
///
/// Provider discovery, admission and selection remain outside
/// this adapter.
///
/// # Security
///
/// This service does not authenticate principals, authorize
/// hardware access or establish provider trust.
///
/// Those checks must be performed by their respective authorities.
///
/// # Concurrency
///
/// Clones share the same canonical isolation manager.
#[derive(Clone)]
pub struct HardwareIsolationService {
    manager: Arc<IsolationManager>,
}

impl HardwareIsolationService {
    /// Constructs the ZHAL isolation integration service.
    ///
    /// The manager must have an explicitly selected isolation provider.
    ///
    /// No default provider or implicit isolation implementation exists.
    #[must_use]
    pub fn new(
        manager: Arc<IsolationManager>,
    ) -> Self {
        Self { manager }
    }

    /// Returns the identity of the selected isolation provider.
    ///
    /// This is descriptive identity, not proof of provider trust.
    #[must_use]
    pub fn provider_id(
        &self,
    ) -> &IsolationIdentifier {
        self.manager.provider_id()
    }

    /// Establishes and verifies hardware execution isolation.
    ///
    /// The request must already have been constructed from validated
    /// frontend semantic information.
    ///
    /// The canonical ZUR manager:
    ///
    /// - Establishes the provider-managed isolation session.
    /// - Checks provider identity.
    /// - Checks session activity.
    /// - Verifies mandatory controls.
    /// - Verifies prohibited controls.
    /// - Records unmet preferences.
    /// - Attempts cleanup when verification fails.
    ///
    /// ZHAL accepts the lease only after the canonical manager
    /// has successfully completed those checks.
    ///
    /// The returned lease must remain alive for the duration
    /// of the protected execution that depends on isolation.
    ///
    /// # Errors
    ///
    /// Returns a translated ZHAL error if isolation cannot
    /// be established or verified.
    pub fn establish(
        &self,
        request: &IsolationRequest,
    ) -> ZhalResult<HardwareIsolationLease> {
        let lease = self.manager
            .establish(request)
            .map_err(translate_isolation_error)?;

        Ok(HardwareIsolationLease {
            inner: lease,
        })
    }

    /// Returns the canonical runtime isolation manager.
    ///
    /// This accessor is intended for controlled runtime composition.
    ///
    /// Protected execution should normally use [`Self::establish`].
    #[must_use]
    pub fn runtime_manager(
        &self,
    ) -> &IsolationManager {
        self.manager.as_ref()
    }
}

impl fmt::Debug for HardwareIsolationService {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("HardwareIsolationService")
            .field("manager", &"[CANONICAL ZUR ISOLATION MANAGER]")
            .finish()
    }
}

// ============================================================================
// HARDWARE ISOLATION LEASE
// ============================================================================

/// An accepted, verified hardware execution-isolation lease.
///
/// This wrapper retains the canonical ZUR IsolationLease.
///
/// It does not duplicate its lifecycle, evidence verification,
/// provider session ownership or cleanup behavior.
///
/// The lease must remain alive for as long as the protected
/// execution depends on the established isolation boundary.
///
/// Explicit cleanup is preferred because it returns observable errors.
///
/// Dropping the lease delegates best-effort cleanup to the
/// underlying canonical ZUR lease.
pub struct HardwareIsolationLease {
    inner: IsolationLease,
}

impl HardwareIsolationLease {
    /// Returns the verified isolation report.
    #[must_use]
    pub fn report(
        &self,
    ) -> &IsolationReport {
        self.inner.report()
    }

    /// Returns the active provider-managed isolation session.
    ///
    /// Returns None after successful explicit closure.
    #[must_use]
    pub fn session(
        &self,
    ) -> Option<&dyn IsolationSession> {
        self.inner.session()
    }

    /// Returns the provider-specific safe execution context.
    ///
    /// The context remains valid only while its owning session
    /// remains alive.
    ///
    /// This is an opaque safe Rust context. It is not a credential,
    /// authorization receipt or isolation proof.
    #[must_use]
    pub fn provider_context(
        &self,
    ) -> Option<&(dyn Any + Send + Sync)> {
        self.session()
            .map(IsolationSession::provider_context)
    }

    /// Returns whether the underlying isolation session is active.
    ///
    /// A closed lease returns Ok(false).
    ///
    /// Provider inspection errors are preserved as ZHAL errors.
    pub fn is_active(
        &self,
    ) -> ZhalResult<bool> {
        self.inner
            .is_active()
            .map_err(translate_isolation_error)
    }

    /// Explicitly closes the isolation session.
    ///
    /// On successful closure, the underlying lease releases ownership
    /// of the provider session.
    ///
    /// If cleanup fails, the canonical ZUR lease retains ownership
    /// so that cleanup can be attempted again.
    ///
    /// # Errors
    ///
    /// Returns a translated ZHAL error if provider cleanup fails.
    pub fn close(
        &mut self,
    ) -> ZhalResult<()> {
        self.inner
            .close()
            .map_err(translate_isolation_error)
    }

    /// Returns whether explicit closure has released the session.
    ///
    /// This describes lease ownership, not an independent hardware
    /// observation. Use `is_active()` for provider inspection.
    #[must_use]
    pub fn is_closed(
        &self,
    ) -> bool {
        self.inner.session().is_none()
    }
}

impl fmt::Debug for HardwareIsolationLease {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("HardwareIsolationLease")
            .field("report", self.inner.report())
            .field(
                "session_owned",
                &self.inner.session().is_some(),
            )
            .finish()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------
    // Error translation
    // ------------------------------------------------------------------------

    #[test]
    fn invalid_request_maps_to_invalid_input() {
        let error = translate_isolation_error(
            IsolationError::new(
                IsolationErrorKind::InvalidRequest,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::InvalidInput,
        );
    }

    #[test]
    fn provider_failure_is_preserved() {
        let error = translate_isolation_error(
            IsolationError::new(
                IsolationErrorKind::ProviderFailure,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::ProviderFailure,
        );

        assert!(error.has_source());
    }

    #[test]
    fn provider_identity_mismatch_is_a_contract_violation() {
        let error = translate_isolation_error(
            IsolationError::new(
                IsolationErrorKind::ProviderIdentityMismatch,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::ContractViolation,
        );
    }

    #[test]
    fn inactive_session_is_a_contract_violation() {
        let error = translate_isolation_error(
            IsolationError::new(
                IsolationErrorKind::SessionInactive,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::ContractViolation,
        );
    }

    #[test]
    fn unsatisfied_required_control_is_incompatible() {
        let error = translate_isolation_error(
            IsolationError::new(
                IsolationErrorKind::RequiredControlUnsatisfied,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::IncompatibleWorkload,
        );
    }

    #[test]
    fn enabled_prohibited_control_is_incompatible() {
        let error = translate_isolation_error(
            IsolationError::new(
                IsolationErrorKind::ProhibitedControlEnabled,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::IncompatibleWorkload,
        );
    }

    #[test]
    fn indeterminate_control_maps_to_unknown_capability() {
        let error = translate_isolation_error(
            IsolationError::new(
                IsolationErrorKind::ControlStateIndeterminate,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::CapabilityUnknown,
        );
    }

    #[test]
    fn cleanup_failure_remains_a_provider_failure() {
        let error = translate_isolation_error(
            IsolationError::new(
                IsolationErrorKind::CleanupFailed,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::ProviderFailure,
        );
    }

    #[test]
    fn isolation_errors_preserve_their_original_cause() {
        let error = translate_isolation_error(
            IsolationError::new(
                IsolationErrorKind::ProviderFailure,
            ),
        );

        assert!(error.has_source());
        assert_eq!(
            error.component(),
            Some(COMPONENT),
        );
    }

    #[test]
    fn isolation_error_component_is_stable() {
        assert_eq!(
            COMPONENT,
            "zhal.security.isolation",
        );
    }

    // ------------------------------------------------------------------------
    // Canonical contract reuse
    // ------------------------------------------------------------------------

    #[test]
    fn future_isolation_controls_remain_supported() {
        let control = IsolationIdentifier::new(
            "future.security.confidential_hybrid_execution",
        )
        .expect("open-world control identifier");

        let directive = IsolationDirective::new(
            control.clone(),
            IsolationDisposition::Required,
        );

        let request = IsolationRequest::new(
            IsolationIdentifier::new(
                "zamani.execution.subject",
            )
            .expect("valid subject"),
            vec![directive],
        )
        .expect("valid isolation request");

        assert_eq!(
            request.directives()[0].control(),
            &control,
        );
    }

    #[test]
    fn canonical_isolation_evidence_preserves_unknown_state() {
        let provider = IsolationIdentifier::new(
            "provider.example",
        )
        .unwrap();

        let evidence = IsolationEvidence::new(
            provider,
            Vec::new(),
        )
        .unwrap();

        let control = IsolationIdentifier::new(
            "security.future_isolation_control",
        )
        .unwrap();

        assert_eq!(
            evidence.state(&control),
            IsolationControlState::Unknown,
        );
    }

    #[test]
    fn canonical_provider_contract_remains_the_authority() {
        fn accepts_provider(
            _provider: Arc<dyn IsolationProvider>,
        ) {
        }

        // Compile-time contract check.
        //
        // The ZHAL adapter deliberately accepts the existing ZUR
        // provider contract rather than defining a competing trait.
        let _ = accepts_provider;
    }
}
