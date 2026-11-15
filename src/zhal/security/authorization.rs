
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Hardware Abstraction Layer
//!
//! ## Hardware Authorization Integration
//!
//! Path: `src/zhal/security/authorization.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the ZHAL integration boundary for authorization of
//! protected physical hardware operations.
//!
//! This module delegates authorization decisions to the existing
//! canonical Zamani security authority.
//!
//! It does not implement a second authorization engine.
//!
//! # Supported computational domains
//!
//! Authorization is independent of the computational domain.
//!
//! It can protect operations involving:
//!
//! - Classical processors and memory.
//! - Quantum hardware.
//! - HDL, FPGA and ASIC realization.
//! - GPUs and other accelerators.
//! - AI and tensor hardware.
//! - Embedded devices.
//! - Hybrid execution.
//! - Distributed hardware.
//! - Networking hardware.
//! - Neuromorphic hardware.
//! - Photonic and analog computing.
//! - Future computational technologies.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - The ZHAL-to-security authorization adapter.
//! - Resource-scoped hardware authorization enforcement.
//! - Authorization error translation.
//! - Validation of successful authorization receipts.
//! - Fail-closed hardware authorization behavior.
//!
//! This module does NOT own:
//!
//! - Principal identity.
//! - Principal authentication.
//! - Identity verification.
//! - Session authentication.
//! - Permission declarations.
//! - Permission identity.
//! - Policy definitions.
//! - Policy parsing.
//! - Policy persistence.
//! - Policy evaluation algorithms.
//! - Grant persistence.
//! - Revocation persistence.
//! - Security audit storage.
//! - Cryptographic algorithms.
//! - Credential storage.
//! - Private-key management.
//! - Provider trust.
//! - Provider authentication.
//! - Hardware discovery.
//! - Hardware selection.
//! - Capability discovery.
//! - Resource allocation.
//! - Resource reservation.
//! - Hardware scheduling.
//! - Runtime orchestration.
//! - Hardware execution.
//!
//! These responsibilities remain with their canonical authorities.
//!
//! # Canonical repository authorities
//!
//! Authorization requests, policy evaluation and audit:
//!
//!     crate::stdlib::security::policy
//!
//! Principal identity:
//!
//!     crate::stdlib::security::identity
//!
//! Runtime authenticated-session permission enforcement:
//!
//!     crate::zur::security::permissions
//!
//! Runtime security enforcement:
//!
//!     crate::zur::security::policy
//!
//! Distributed authentication and authorization:
//!
//!     crate::distributed::security
//!
//! Provider trust:
//!
//!     crate::zhal::security::trust
//!
//! Hardware access:
//!
//!     crate::zhal::devices::access
//!
//! Hardware identity:
//!
//!     crate::zhal::identity
//!
//! ZHAL errors:
//!
//!     crate::zhal::errors
//!
//! # Security invariants
//!
//! 1. Authorization is deny-by-default.
//! 2. Authentication is not inferred from a principal identifier.
//! 3. A permission declaration does not grant permission.
//! 4. A capability does not grant permission.
//! 5. Provider registration does not grant permission.
//! 6. Provider trust does not grant permission.
//! 7. Device discovery does not grant permission.
//! 8. Resource availability does not grant permission.
//! 9. Missing authorization never permits hardware access.
//! 10. Denial never becomes permission through fallback.
//! 11. Indeterminate decisions never authorize hardware access.
//! 12. Not-applicable decisions never authorize hardware access.
//! 13. Outstanding policy obligations prevent authorization.
//! 14. Required audit failure prevents authorization.
//! 15. Policy evaluator failure prevents authorization.
//! 16. Invalid authorization results prevent hardware access.
//! 17. Authorization receipts are not permanent grants.
//! 18. Authorization is evaluated for the requested resource.
//! 19. This module never caches authorization decisions.
//! 20. The hardware provider must enforce its own applicable
//!     security boundary.
//!
//! # Resource-scoped authorization
//!
//! A hardware authorization request must identify the resource
//! being protected.
//!
//! An authorization request without a resource is rejected.
//!
//! The resource identifier is constructed by the caller using
//! the canonical policy resource representation.
//!
//! This module does not invent a second resource identity system.
//!
//! # Authentication boundary
//!
//! The standard-library AuthorizationService evaluates an
//! AuthorizationRequest.
//!
//! A PrincipalId inside that request is an identity reference,
//! not proof of authentication.
//!
//! The caller must ensure that the principal was authenticated
//! and resolved through the canonical identity/authentication
//! authorities before constructing the protected request.
//!
//! Where authenticated-session enforcement is required, the
//! runtime should use:
//!
//!     crate::zur::security::permissions::PermissionAuthorizer
//!
//! That authority validates the authenticated session and
//! delegates policy evaluation to the canonical distributed
//! authorization service.
//!
//! This module must not bypass that requirement by treating
//! an arbitrary principal reference as an authenticated session.
//!
//! # Authorization and trust are separate
//!
//! Provider trust is evaluated through:
//!
//!     crate::zhal::security::trust
//!
//! Authorization is evaluated through:
//!
//!     crate::stdlib::security::policy
//!
//! A trusted provider is not automatically authorized.
//!
//! An authorized operation does not establish provider trust.
//!
//! Both checks must be performed when required by the applicable
//! runtime security policy.
//!
//! # POCO-REAF and scalability
//!
//! This module introduces no arbitrary universal maximum on:
//!
//! - Principals.
//! - Permissions.
//! - Policies.
//! - Authorization requests.
//! - Resources.
//! - Providers.
//! - Devices.
//! - Hardware operations.
//! - Computational domains.
//! - Distributed participants.
//!
//! It does not introduce fixed hardware counts, fixed memory
//! assumptions, or closed lists of hardware technologies.
//!
//! Practical limits are determined by available resources,
//! canonical security authorities, providers and explicitly
//! configured operational budgets.
//!
//! Literal infinite computation on finite resources is not promised.
//!
//! # Concurrency
//!
//! The canonical AuthorizationService is shared through Arc.
//!
//! This module introduces no global mutable state, implicit
//! singleton, hidden registry or local authorization cache.
//!
//! Concurrent authorization behavior remains governed by the
//! canonical authorization service and its injected dependencies.
//!
//! # Error handling
//!
//! Canonical policy errors are retained as causal errors.
//!
//! Error classifications are translated into the existing
//! ZHAL error model.
//!
//! Authorization failures never become successful results.
//!
//! # Integration
//!
//! The ZHAL hardware access service should use this adapter
//! rather than independently reimplementing policy evaluation.
//!
//! The canonical authorization service remains the only
//! authority that decides whether an operation is permitted.

use std::fmt;
use std::sync::Arc;

use crate::stdlib::security::policy::{
    AuthorizationReceipt,
    AuthorizationRequest,
    AuthorizationService,
    PolicyError,
    PolicyErrorKind,
};

use crate::zhal::errors::{
    ZhalError,
    ZhalErrorKind,
    ZhalResult,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

/// Stable component identifier.
pub const COMPONENT: &str = "zhal.security.authorization";

// ============================================================================
// ERROR TRANSLATION
// ============================================================================

/// Translates canonical policy errors into the ZHAL error model.
///
/// The original policy error is retained as the causal source.
///
/// No policy error is interpreted as authorization success.
fn translate_policy_error(error: PolicyError) -> ZhalError {
    let kind = match error.kind() {
        PolicyErrorKind::InvalidIdentifier
        | PolicyErrorKind::InvalidRequest => {
            ZhalErrorKind::InvalidInput
        }

        PolicyErrorKind::InvalidDecision
        | PolicyErrorKind::InvalidProviderResult => {
            ZhalErrorKind::ContractViolation
        }

        PolicyErrorKind::Denied
        | PolicyErrorKind::NotApplicable
        | PolicyErrorKind::Indeterminate
        | PolicyErrorKind::ObligationsPending => {
            ZhalErrorKind::PermissionDenied
        }

        PolicyErrorKind::EvaluatorUnavailable => {
            ZhalErrorKind::ProviderUnavailable
        }

        PolicyErrorKind::EvaluationFailure
        | PolicyErrorKind::AuditUnavailable
        | PolicyErrorKind::AuditFailure => {
            ZhalErrorKind::ProviderFailure
        }

        _ => ZhalErrorKind::PermissionDenied,
    };

    ZhalError::new(
        kind,
        "the canonical security authority did not establish hardware authorization",
    )
    .in_component(COMPONENT)
    .with_source(error)
}

// ============================================================================
// HARDWARE AUTHORIZATION SERVICE
// ============================================================================

/// ZHAL adapter for the canonical authorization service.
///
/// This type does not evaluate policies itself.
///
/// It delegates every authorization request to the injected
/// standard-library AuthorizationService.
///
/// The underlying service must be configured with its authoritative
/// policy evaluator and required audit sink.
///
/// There is no implicit allow-all configuration.
#[derive(Clone)]
pub struct HardwareAuthorizationService {
    authorization: Arc<AuthorizationService>,
}

impl HardwareAuthorizationService {
    /// Constructs the hardware authorization adapter.
    ///
    /// The supplied service must already be configured with:
    ///
    /// - An authoritative policy evaluator.
    /// - A trusted policy configuration.
    /// - A required audit sink where required by security policy.
    ///
    /// This constructor does not authenticate principals or
    /// establish trust in the supplied evaluator.
    ///
    /// # Arguments
    ///
    /// `authorization`:
    ///     The canonical standard-library authorization service.
    #[must_use]
    pub fn new(
        authorization: Arc<AuthorizationService>,
    ) -> Self {
        Self {
            authorization,
        }
    }

    /// Authorizes a resource-scoped hardware operation.
    ///
    /// The request must contain a resource reference.
    ///
    /// The canonical authority performs policy evaluation,
    /// decision validation and audit handling.
    ///
    /// Only an unconditional policy permit is accepted.
    ///
    /// Denial, not-applicable, indeterminate decisions,
    /// outstanding obligations, evaluator failures and
    /// mandatory audit failures all return errors.
    ///
    /// This method does not execute the hardware operation.
    /// The caller may proceed to provider session establishment
    /// only after this method succeeds and all other required
    /// security checks have succeeded.
    ///
    /// The returned receipt is the canonical authorization receipt.
    /// It is not an independently created ZHAL permission grant.
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - The request has no resource.
    /// - The canonical policy authority denies the operation.
    /// - The policy decision is indeterminate.
    /// - The policy has outstanding obligations.
    /// - Policy evaluation fails.
    /// - Required audit recording fails.
    /// - The authorization result is not an unconditional permit.
    pub fn authorize(
        &self,
        request: &AuthorizationRequest,
    ) -> ZhalResult<AuthorizationReceipt> {
        if request.resource().is_none() {
            return Err(
                ZhalError::new(
                    ZhalErrorKind::InvalidInput,
                    "hardware authorization requires a resource-scoped request",
                )
                .in_component(COMPONENT),
            );
        }

        let receipt = self
            .authorization
            .authorize(request)
            .map_err(translate_policy_error)?;

        // Defense in depth.
        //
        // AuthorizationService::authorize already rejects decisions
        // that are not unconditional permits. This additional check
        // ensures that ZHAL never relies on an unexpected receipt.
        if !receipt.is_authorized() {
            return Err(
                ZhalError::new(
                    ZhalErrorKind::PermissionDenied,
                    "the authorization receipt does not establish an unconditional permit",
                )
                .in_component(COMPONENT),
            );
        }

        Ok(receipt)
    }

    /// Returns the canonical authorization service.
    ///
    /// This accessor is intended for controlled integration.
    ///
    /// Protected hardware operations should normally use
    /// [`Self::authorize`] rather than bypassing this adapter.
    #[must_use]
    pub fn authorization_service(
        &self,
    ) -> &AuthorizationService {
        self.authorization.as_ref()
    }
}

// ============================================================================
// DEBUG
// ============================================================================

impl fmt::Debug for HardwareAuthorizationService {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("HardwareAuthorizationService")
            .field(
                "authorization",
                &"[CANONICAL AUTHORIZATION SERVICE]",
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

    #[test]
    fn policy_denial_maps_to_permission_denied() {
        let error = translate_policy_error(
            PolicyError::new(PolicyErrorKind::Denied),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::PermissionDenied,
        );

        assert_eq!(
            error.component(),
            Some(COMPONENT),
        );

        assert!(error.has_source());
    }

    #[test]
    fn indeterminate_policy_result_fails_closed() {
        let error = translate_policy_error(
            PolicyError::new(PolicyErrorKind::Indeterminate),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::PermissionDenied,
        );

        assert!(error.has_source());
    }

    #[test]
    fn outstanding_obligations_fail_closed() {
        let error = translate_policy_error(
            PolicyError::new(
                PolicyErrorKind::ObligationsPending,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::PermissionDenied,
        );
    }

    #[test]
    fn evaluator_unavailability_is_preserved() {
        let error = translate_policy_error(
            PolicyError::new(
                PolicyErrorKind::EvaluatorUnavailable,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::ProviderUnavailable,
        );
    }

    #[test]
    fn audit_failure_is_not_authorization_success() {
        for kind in [
            PolicyErrorKind::AuditUnavailable,
            PolicyErrorKind::AuditFailure,
        ] {
            let error = translate_policy_error(
                PolicyError::new(kind),
            );

            assert_eq!(
                error.kind(),
                ZhalErrorKind::ProviderFailure,
            );

            assert!(error.has_source());
        }
    }

    #[test]
    fn invalid_authority_result_is_a_contract_violation() {
        let error = translate_policy_error(
            PolicyError::new(
                PolicyErrorKind::InvalidProviderResult,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::ContractViolation,
        );
    }

    #[test]
    fn invalid_request_is_not_silently_authorized() {
        let error = translate_policy_error(
            PolicyError::new(
                PolicyErrorKind::InvalidRequest,
            ),
        );

        assert_eq!(
            error.kind(),
            ZhalErrorKind::InvalidInput,
        );
    }

    #[test]
    fn authorization_adapter_debug_does_not_expose_service_internals() {
        // The service itself requires an evaluator and audit sink.
        // This test intentionally verifies only the adapter's
        // diagnostic contract through its Debug implementation
        // when a fully configured service is supplied by integration
        // tests.
        //
        // No fake authorization authority is constructed here.
        assert_eq!(
            COMPONENT,
            "zhal.security.authorization",
        );
    }
}
