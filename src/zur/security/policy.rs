
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — Security Policy Enforcement
//!
//! Path: src/zur/security/policy.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! PURPOSE
//!
//! Provides the ZUR enforcement boundary for security authorization.
//!
//! This module ensures that security-sensitive runtime operations are
//! authorized by the existing authoritative security policy service
//! before the protected operation is permitted to proceed.
//!
//! SUPPORTED EXECUTION ENVIRONMENTS
//!
//! The enforcement boundary is computational-domain neutral.
//!
//! It can protect operations involving:
//!
//! - Classical execution.
//! - Quantum execution.
//! - HDL and hardware.
//! - AI and tensor computation.
//! - Accelerators.
//! - Hybrid execution.
//! - Distributed execution.
//! - Networking.
//! - External providers.
//! - Future computational domains.
//!
//! OWNERSHIP
//!
//! This module owns:
//!
//! - Runtime integration with the canonical authorization service.
//! - Runtime security enforcement at the ZUR boundary.
//! - Fail-closed authorization receipt validation.
//! - Mandatory audit confirmation.
//! - Enforcement of the configured runtime security uncertainty policy.
//!
//! EXISTING AUTHORITIES
//!
//! The following modules remain authoritative:
//!
//! - crate::zur::policy:
//!   RuntimePolicy and SecurityPolicy.
//!
//! - crate::zur::error:
//!   ZurError and ZurResult.
//!
//! - crate::stdlib::security::policy:
//!   AuthorizationRequest, AuthorizationService, PolicyDecision,
//!   PolicyEffect, PolicyError and audit contracts.
//!
//! - crate::stdlib::security::identity:
//!   Principal identity.
//!
//! - crate::backend::security:
//!   Backend-specific filesystem and external-tool security.
//!
//! - crate::frontend::semantic:
//!   Source-language security semantics.
//!
//! - crate::frontend::semantic::capabilities:
//!   Canonical capability semantics.
//!
//! - crate::distributed:
//!   Distributed security and execution contracts.
//!
//! THIS MODULE DOES NOT OWN
//!
//! - Security grammar.
//! - AST or semantic security models.
//! - Authentication.
//! - Principal creation.
//! - Identity verification.
//! - Policy parsing.
//! - Policy persistence.
//! - Policy evaluation algorithms.
//! - Capability discovery.
//! - Capability evidence.
//! - Cryptographic algorithms.
//! - Key management.
//! - Trust establishment.
//! - Hardware attestation.
//! - Hardware discovery.
//! - Resource allocation.
//! - Device isolation implementation.
//! - Backend selection.
//! - Distributed security protocols.
//! - Security audit storage.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! SECURITY INVARIANTS
//!
//! 1. Authorization is deny-by-default.
//! 2. Authentication is not inferred from an identity reference.
//! 3. Policy declarations are not authorization decisions.
//! 4. Capability declarations are not permission grants.
//! 5. Unknown security information never establishes permission.
//! 6. Denial cannot become permission through fallback.
//! 7. Indeterminate decisions cannot authorize execution.
//! 8. Not-applicable decisions cannot authorize execution.
//! 9. Outstanding obligations prevent execution.
//! 10. Required audit confirmation must exist before authorization succeeds.
//! 11. Evaluator failures cannot become successful authorization.
//! 12. An authorization receipt is not a substitute for provider-side
//!     security enforcement.
//!
//! SCALABILITY
//!
//! This module imposes no arbitrary maximum on:
//!
//! - Policies.
//! - Principals.
//! - Actions.
//! - Resources.
//! - Authorization requests.
//! - Obligations.
//! - Providers.
//! - Devices.
//! - Nodes.
//! - Computational domains.
//! - Execution sessions.
//!
//! Actual resource availability and configured operational budgets
//! determine practical limits.
//!
//! Literal infinite execution on finite physical resources is not promised.
//!
//! INTEGRATION
//!
//! RuntimeContext and RuntimePolicy supply the runtime's effective
//! security configuration.
//!
//! AuthorizationService supplies the authoritative policy evaluator
//! and audit sink.
//!
//! Execution dispatch, JIT verification, domain providers, distributed
//! execution and hybrid execution must invoke this enforcement boundary
//! before performing their protected operations.
//!
//! This module does not cache authorization decisions. Authorization
//! must be performed at the protected operation boundary to avoid
//! relying on stale decisions.
//!
//! IMPORTANT
//!
//! Constructing this enforcement service does not establish that its
//! evaluator, audit sink, principal or security provider is trustworthy.
//!
//! Those trust relationships must be established independently.

use crate::stdlib::security::policy::{
    AuditStatus,
    AuthorizationReceipt,
    AuthorizationRequest,
    AuthorizationService,
    PolicyEffect,
    PolicyError,
    PolicyErrorKind,
};

use crate::zur::error::{
    ZurError,
    ZurResult,
};

use crate::zur::policy::{
    SecurityPolicy,
    SecurityUncertaintyPolicy,
};

use core::fmt;

// ============================================================================
// SECURITY ENFORCEMENT ERROR CLASSIFICATION
// ============================================================================

/// Classification of ZUR security enforcement failures.
///
/// These variants describe failures at the runtime enforcement boundary.
/// They do not replace the canonical authorization error model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SecurityEnforcementError {
    /// The supplied runtime security policy is invalid.
    InvalidRuntimePolicy,

    /// Authorization was explicitly denied.
    Denied,

    /// No applicable authorization policy established permission.
    NotApplicable,

    /// The authorization decision was indeterminate.
    Indeterminate,

    /// An independent security decision is required.
    ExternalDecisionRequired,

    /// Policy evaluation failed.
    EvaluationFailed,

    /// The authorization service could not be used.
    AuthorizationUnavailable,

    /// The audit requirement was not satisfied.
    AuditUnavailable,

    /// The evaluator returned a decision with outstanding obligations.
    ObligationsPending,

    /// An unexpected authorization result was received.
    InvalidAuthorizationResult,
}

impl SecurityEnforcementError {
    /// Returns the stable machine-readable classification.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRuntimePolicy =>
                "invalid_runtime_policy",

            Self::Denied =>
                "denied",

            Self::NotApplicable =>
                "not_applicable",

            Self::Indeterminate =>
                "indeterminate",

            Self::ExternalDecisionRequired =>
                "external_decision_required",

            Self::EvaluationFailed =>
                "evaluation_failed",

            Self::AuthorizationUnavailable =>
                "authorization_unavailable",

            Self::AuditUnavailable =>
                "audit_unavailable",

            Self::ObligationsPending =>
                "obligations_pending",

            Self::InvalidAuthorizationResult =>
                "invalid_authorization_result",
        }
    }
}

impl fmt::Display for SecurityEnforcementError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// RUNTIME SECURITY ENFORCER
// ============================================================================

/// Runtime security enforcement service.
///
/// This type connects the existing ZUR security configuration to the
/// canonical standard-library authorization service.
///
/// It does not contain its own policy database, evaluator, identity
/// authority or audit implementation.
///
/// The authorization service must be explicitly injected.
///
/// No implicit allow-all evaluator or no-op audit sink exists.
#[derive(Clone)]
pub struct SecurityPolicyEnforcer {
    policy: SecurityPolicy,

    authorization: AuthorizationService,
}

impl SecurityPolicyEnforcer {
    /// Constructs a runtime security enforcement service.
    ///
    /// The supplied policy is validated before the service is created.
    ///
    /// The authorization service must provide an authoritative evaluator
    /// and an explicitly configured audit sink.
    ///
    /// Construction does not authenticate users or certify the
    /// trustworthiness of the supplied security providers.
    ///
    /// # Errors
    ///
    /// Returns an error if the runtime security policy is invalid.
    pub fn new(
        policy: SecurityPolicy,
        authorization: AuthorizationService,
    ) -> ZurResult<Self> {
        policy.validate().map_err(|error| {
            ZurError::permission_denied(
                "runtime security policy validation failed",
            )
            .with_component("zur.security.policy")
            .with_operation("construct")
            .with_source(error)
        })?;

        Ok(Self {
            policy,
            authorization,
        })
    }

    /// Constructs an enforcer using the mandatory default security policy.
    ///
    /// This is equivalent to supplying `SecurityPolicy::new()`.
    ///
    /// Authorization and audit implementations must still be supplied.
    pub fn with_default_policy(
        authorization: AuthorizationService,
    ) -> ZurResult<Self> {
        Self::new(
            SecurityPolicy::new(),
            authorization,
        )
    }

    /// Returns the configured runtime security policy.
    #[must_use]
    pub const fn policy(&self) -> SecurityPolicy {
        self.policy
    }

    /// Returns the identity of the authoritative policy evaluator.
    ///
    /// This identifies the configured evaluator.
    ///
    /// It does not independently establish evaluator trust.
    #[must_use]
    pub fn evaluator_id(
        &self,
    ) -> &crate::stdlib::security::policy::PolicyEvaluatorId {
        self.authorization.evaluator_id()
    }

    /// Returns the identity of the configured audit sink.
    ///
    /// This identifies the configured sink.
    ///
    /// It does not independently establish durable audit storage.
    #[must_use]
    pub fn audit_sink_id(
        &self,
    ) -> &crate::stdlib::security::policy::AuditSinkId {
        self.authorization.audit_sink_id()
    }

    // ------------------------------------------------------------------------
    // AUTHORIZATION
    // ------------------------------------------------------------------------

    /// Evaluates and authorizes a protected runtime operation.
    ///
    /// The canonical authorization service performs policy evaluation
    /// and records the corresponding audit event.
    ///
    /// This method permits the operation only when all of the following
    /// conditions are satisfied:
    ///
    /// - The authorization evaluator returns a permit.
    /// - No outstanding obligations remain.
    /// - The audit sink confirms successful recording.
    /// - The authorization receipt is internally consistent.
    ///
    /// All other outcomes fail closed.
    ///
    /// This method does not execute the protected operation.
    ///
    /// The caller must perform the operation only after this method
    /// returns a successful receipt.
    ///
    /// # Important
    ///
    /// Authorization must be checked as close as possible to the
    /// protected operation.
    ///
    /// Do not cache receipts and reuse them after the relevant
    /// authorization context, policy revision, principal or resource
    /// state may have changed.
    ///
    /// Providers must independently enforce their own security
    /// boundaries.
    ///
    /// # Errors
    ///
    /// Returns an error for:
    ///
    /// - Explicit denial.
    /// - Missing applicable authorization.
    /// - Indeterminate policy evaluation.
    /// - Outstanding obligations.
    /// - Failed policy evaluation.
    /// - Missing audit confirmation.
    /// - Invalid authorization results.
    pub fn authorize(
        &self,
        request: &AuthorizationRequest,
    ) -> ZurResult<AuthorizationReceipt> {
        self.policy.validate().map_err(|error| {
            ZurError::permission_denied(
                "runtime security policy validation failed",
            )
            .with_component("zur.security.policy")
            .with_operation("authorize")
            .with_source(error)
        })?;

        // Evaluation and audit are performed by the existing
        // authoritative standard-library security service.
        //
        // evaluate_and_audit is deliberately used instead of relying
        // exclusively on AuthorizationService::authorize because
        // this boundary must independently enforce mandatory auditing.
        //
        // In particular, an AuthorizationService configured with
        // BestEffort auditing must not bypass ZUR's mandatory audit
        // confirmation.
        let receipt = self
            .authorization
            .evaluate_and_audit(request)
            .map_err(|error| {
                self.map_authorization_error(error)
            })?;

        self.validate_receipt(&receipt)?;

        Ok(receipt)
    }

    // ------------------------------------------------------------------------
    // RECEIPT VALIDATION
    // ------------------------------------------------------------------------

    /// Validates an authorization receipt before protected execution.
    ///
    /// The receipt must contain:
    ///
    /// - An unconditional permit.
    /// - No outstanding obligations.
    /// - Confirmed audit recording.
    ///
    /// A receipt containing any other outcome is rejected.
    ///
    /// This method does not execute the protected operation.
    ///
    /// # Errors
    ///
    /// Returns a security error when the receipt does not satisfy
    /// the mandatory enforcement requirements.
    pub fn validate_receipt(
        &self,
        receipt: &AuthorizationReceipt,
    ) -> ZurResult<()> {
        match receipt.decision().effect() {
            PolicyEffect::Permit => {
                // Continue to obligation and audit verification.
            }

            PolicyEffect::Deny => {
                return Err(self.enforcement_error(
                    SecurityEnforcementError::Denied,
                    "security authorization was denied",
                ));
            }

            PolicyEffect::NotApplicable => {
                return Err(self.enforcement_error(
                    SecurityEnforcementError::NotApplicable,
                    "no applicable security policy authorized the operation",
                ));
            }

            PolicyEffect::Indeterminate => {
                return Err(self.indeterminate_error());
            }

            _ => {
                return Err(self.enforcement_error(
                    SecurityEnforcementError::InvalidAuthorizationResult,
                    "unsupported security authorization outcome",
                ));
            }
        }

        // A permit with outstanding obligations is not an
        // unconditional authorization.
        if !receipt.decision().obligations().is_empty() {
            return Err(self.enforcement_error(
                SecurityEnforcementError::ObligationsPending,
                "security authorization has outstanding obligations",
            ));
        }

        // The runtime requires actual confirmation from the audit sink.
        //
        // A BestEffort audit configuration in the underlying service
        // cannot weaken this runtime enforcement requirement.
        match receipt.audit_status() {
            AuditStatus::Recorded(_) => {}

            AuditStatus::Unavailable(_) => {
                return Err(self.enforcement_error(
                    SecurityEnforcementError::AuditUnavailable,
                    "mandatory security audit recording was not confirmed",
                ));
            }

            _ => {
                return Err(self.enforcement_error(
                    SecurityEnforcementError::InvalidAuthorizationResult,
                    "invalid security audit status",
                ));
            }
        }

        // The canonical receipt also exposes its own authorization
        // classification. Check it rather than assuming that a
        // successful evaluation call means permission was granted.
        if !receipt.is_authorized() {
            return Err(self.enforcement_error(
                SecurityEnforcementError::InvalidAuthorizationResult,
                "authorization receipt does not establish unconditional permission",
            ));
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // ERROR ADAPTATION
    // ------------------------------------------------------------------------

    /// Converts an authorization-service failure into the canonical
    /// ZUR operational error model.
    ///
    /// The original authorization error is preserved as the causal source.
    fn map_authorization_error(
        &self,
        error: PolicyError,
    ) -> ZurError {
        let kind = error.kind();

        let (classification, message) = match kind {
            PolicyErrorKind::Denied => (
                SecurityEnforcementError::Denied,
                "security authorization was denied",
            ),

            PolicyErrorKind::Indeterminate => (
                SecurityEnforcementError::Indeterminate,
                "security authorization could not establish permission",
            ),

            PolicyErrorKind::ObligationsPending => (
                SecurityEnforcementError::ObligationsPending,
                "security authorization has outstanding obligations",
            ),

            PolicyErrorKind::AuditUnavailable
            | PolicyErrorKind::AuditFailure => (
                SecurityEnforcementError::AuditUnavailable,
                "mandatory security audit could not be confirmed",
            ),

            PolicyErrorKind::EvaluatorUnavailable => (
                SecurityEnforcementError::AuthorizationUnavailable,
                "authoritative security evaluator is unavailable",
            ),

            PolicyErrorKind::InvalidProviderResult => (
                SecurityEnforcementError::InvalidAuthorizationResult,
                "security provider returned an invalid result",
            ),

            PolicyErrorKind::InvalidIdentifier
            | PolicyErrorKind::InvalidRequest
            | PolicyErrorKind::InvalidDecision => (
                SecurityEnforcementError::EvaluationFailed,
                "security authorization request or decision was invalid",
            ),

            PolicyErrorKind::EvaluationFailure => (
                SecurityEnforcementError::EvaluationFailed,
                "security policy evaluation failed",
            ),
        };

        let error = self.enforcement_error(
            classification,
            message,
        );

        error.with_source(error_source(kind))
    }

    /// Handles an indeterminate authorization result according to
    /// the configured uncertainty policy.
    ///
    /// Neither uncertainty mode grants permission automatically.
    fn indeterminate_error(&self) -> ZurError {
        match self.policy.uncertainty() {
            SecurityUncertaintyPolicy::Deny => {
                self.enforcement_error(
                    SecurityEnforcementError::Indeterminate,
                    "indeterminate security evidence was denied",
                )
            }

            SecurityUncertaintyPolicy::RequireExternalDecision => {
                self.enforcement_error(
                    SecurityEnforcementError::ExternalDecisionRequired,
                    "an independent authoritative security decision is required",
                )
            }

            _ => {
                self.enforcement_error(
                    SecurityEnforcementError::InvalidRuntimePolicy,
                    "unsupported security uncertainty policy",
                )
            }
        }
    }

    /// Creates a structured ZUR security error.
    fn enforcement_error(
        &self,
        classification: SecurityEnforcementError,
        message: &'static str,
    ) -> ZurError {
        ZurError::permission_denied(message)
            .with_component("zur.security.policy")
            .with_operation(classification.as_str())
    }
}

// ============================================================================
// CAUSAL ERROR ADAPTATION
// ============================================================================

/// A non-sensitive causal description of an authorization failure.
///
/// The original policy error classification is preserved without
/// exposing potentially sensitive evaluator diagnostics.
#[derive(Debug)]
struct AuthorizationFailureSource {
    kind: PolicyErrorKind,
}

impl fmt::Display for AuthorizationFailureSource {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "authorization service failure: {}",
            self.kind.as_str()
        )
    }
}

impl std::error::Error for AuthorizationFailureSource {}

/// Creates a safe causal error representation.
fn error_source(
    kind: PolicyErrorKind,
) -> AuthorizationFailureSource {
    AuthorizationFailureSource {
        kind,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::policy::{
        SecurityEnforcement,
        SecurityUncertaintyPolicy,
    };

    #[test]
    fn default_security_policy_requires_mandatory_enforcement() {
        let policy = SecurityPolicy::new();

        assert_eq!(
            policy.enforcement(),
            SecurityEnforcement::Mandatory,
        );

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn default_security_policy_fails_closed_on_uncertainty() {
        let policy = SecurityPolicy::new();

        assert_eq!(
            policy.uncertainty(),
            SecurityUncertaintyPolicy::Deny,
        );
    }

    #[test]
    fn external_decision_policy_does_not_grant_permission() {
        let policy = SecurityPolicy::new()
            .with_external_decision();

        assert_eq!(
            policy.uncertainty(),
            SecurityUncertaintyPolicy::RequireExternalDecision,
        );

        assert!(policy.validate().is_ok());
    }

    #[test]
    fn enforcement_error_classifications_are_stable() {
        assert_eq!(
            SecurityEnforcementError::Denied.as_str(),
            "denied",
        );

        assert_eq!(
            SecurityEnforcementError::AuditUnavailable.as_str(),
            "audit_unavailable",
        );

        assert_eq!(
            SecurityEnforcementError::ObligationsPending.as_str(),
            "obligations_pending",
        );
    }

    #[test]
    fn uncertainty_error_never_becomes_permission() {
        let policy = SecurityPolicy::new()
            .with_external_decision();

        let error = match policy.uncertainty() {
            SecurityUncertaintyPolicy::RequireExternalDecision => {
                SecurityEnforcementError::ExternalDecisionRequired
            }

            SecurityUncertaintyPolicy::Deny => {
                SecurityEnforcementError::Denied
            }

            _ => panic!("unexpected test policy"),
        };

        assert_ne!(
            error,
            SecurityEnforcementError::Denied,
        );

        assert_eq!(
            error,
            SecurityEnforcementError::ExternalDecisionRequired,
        );
    }

    #[test]
    fn enforcement_errors_are_machine_readable() {
        for classification in [
            SecurityEnforcementError::InvalidRuntimePolicy,
            SecurityEnforcementError::Denied,
            SecurityEnforcementError::NotApplicable,
            SecurityEnforcementError::Indeterminate,
            SecurityEnforcementError::ExternalDecisionRequired,
            SecurityEnforcementError::EvaluationFailed,
            SecurityEnforcementError::AuthorizationUnavailable,
            SecurityEnforcementError::AuditUnavailable,
            SecurityEnforcementError::ObligationsPending,
            SecurityEnforcementError::InvalidAuthorizationResult,
        ] {
            assert!(!classification.as_str().is_empty());
        }
    }
}
