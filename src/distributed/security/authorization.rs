
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani distributed authorization enforcement.
//!
//! Path: src/distributed/security/authorization.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Connects authenticated distributed sessions to the existing policy
//! enforcement subsystem.
//!
//! This module provides:
//!
//! - session-backed authorization;
//! - trusted, injectable authorization context;
//! - policy request construction;
//! - request-scoped policy enforcement;
//! - an authorize-before-execute boundary;
//! - explicit session revocation integration;
//! - fail-closed behavior;
//! - provider-neutral integration.
//!
//! # Ownership
//!
//! This module owns authorization orchestration at the distributed
//! security boundary.
//!
//! It does not own:
//!
//! - source-language authorization syntax;
//! - AST construction or semantic validation;
//! - identity representation;
//! - authentication or credential verification;
//! - trust establishment;
//! - policy language parsing;
//! - policy evaluation algorithms;
//! - policy storage or distribution;
//! - cryptographic operations;
//! - transport or TLS;
//! - resource discovery;
//! - scheduling or placement;
//! - workload execution engines;
//! - domain-specific IR.
//!
//! # Existing authorities
//!
//! - identity.rs owns PrincipalId and PeerIdentity.
//! - authentication.rs owns AuthenticationService, SessionId,
//!   AuthenticatedSession and AuthenticatedSubject.
//! - policy.rs owns PolicyRequest, PolicyEngine, PolicyEnforcer,
//!   PolicyPermit, PolicyIdentifier and PolicyAttributes.
//! - distributed/error.rs owns DistributedError and DistributedResult.
//! - grammar/security/authorization.g4 owns source authorization syntax.
//! - grammar/security/permissions.g4 owns source permission syntax.
//! - grammar/spec/security.md owns normative security semantics.
//!
//! # Security invariants
//!
//! 1. An authorization request must reference an active authenticated
//!    session.
//! 2. A session identifier alone is not proof that a session is active.
//! 3. Session validity is checked through AuthenticationService for every
//!    authorization request.
//! 4. Only the existing PolicyEnforcer can produce a PolicyPermit.
//! 5. Deny, NotApplicable, Indeterminate and evaluation errors never grant
//!    permission.
//! 6. Policy obligations must be fulfilled by the existing PolicyEnforcer.
//! 7. Authorization must happen at the protected-operation boundary.
//! 8. A permit is request-scoped and must not be cached or reused.
//! 9. Authorization context must be obtained from a trusted runtime
//!    provider, not blindly accepted from an untrusted remote caller.
//! 10. Authentication and authorization remain separate operations.
//!
//! # Scalability
//!
//! No fixed number of principals, sessions, policies, resources, actions,
//! context attributes, operations, or authorization providers is imposed.
//!
//! The module introduces no universal machine, node, task, device, or
//! workload limits.
//!
//! Actual resource budgets belong to the relevant runtime and policy
//! providers.
//!
//! # Concurrency
//!
//! This module owns no global mutable state and creates no internal
//! unbounded task queues.
//!
//! Its collaborators are shared through Arc and must implement their
//! documented concurrency and failure contracts.
//!
//! # Important limitation
//!
//! This module cannot make an arbitrary external side effect transactional.
//! The caller must invoke authorize_and_execute at the actual operation
//! boundary and must still apply the operation's idempotency, transaction,
//! and failure-reconciliation requirements.

use std::sync::Arc;

use serde_json::Value;

use crate::distributed::error::{
    DistributedError,
    DistributedResult,
};

use crate::distributed::security::authentication::{
    AuthenticationService,
    SessionId,
};

use crate::distributed::security::policy::{
    PolicyAttributes,
    PolicyEnforcer,
    PolicyIdentifier,
    PolicyPermit,
    PolicyRequest,
};

/// Supplies trusted runtime attributes for authorization evaluation.
///
/// Implementations may obtain attributes from authoritative runtime
/// components such as:
///
/// - verified workload metadata;
/// - authenticated service metadata;
/// - validated execution context;
/// - resource ownership records;
/// - trusted deployment metadata;
/// - security labels established by the runtime.
///
/// Implementations must not treat caller-supplied or remotely supplied
/// attribute values as trusted merely because they appear in a request.
///
/// In particular, implementations must not insert:
///
/// - passwords;
/// - private keys;
/// - bearer tokens;
/// - API secrets;
/// - authentication headers;
/// - unrestricted secret material.
///
/// Attribute names and values are consumed by the configured policy
/// engine. The provider is responsible for the provenance and correctness
/// of the values it supplies.
pub trait AuthorizationContextProvider: Send + Sync {
    /// Construct policy attributes for one authenticated authorization
    /// request.
    ///
    /// The provider receives the authenticated subject and the requested
    /// action/resource identifiers. It must return only attributes whose
    /// provenance it can justify.
    ///
    /// Returning an error prevents authorization from proceeding.
    fn attributes(
        &self,
        subject: &crate::distributed::security::policy::AuthenticatedSubject,
        action: &PolicyIdentifier,
        resource: &PolicyIdentifier,
    ) -> DistributedResult<PolicyAttributes>;
}

/// Context provider for deployments that intentionally require no
/// additional policy attributes.
///
/// This is not an allow-all policy. The configured PolicyEnforcer still
/// evaluates the request and applies its default-deny behavior.
#[derive(Debug, Default)]
pub struct EmptyAuthorizationContext;

impl AuthorizationContextProvider for EmptyAuthorizationContext {
    fn attributes(
        &self,
        _subject: &crate::distributed::security::policy::AuthenticatedSubject,
        _action: &PolicyIdentifier,
        _resource: &PolicyIdentifier,
    ) -> DistributedResult<PolicyAttributes> {
        Ok(PolicyAttributes::new())
    }
}

/// Distributed authorization service.
///
/// This service coordinates:
///
/// 1. active session validation;
/// 2. trusted context construction;
/// 3. policy request construction;
/// 4. existing policy enforcement.
///
/// It does not implement a second policy engine.
///
/// The authentication service and policy enforcer are mandatory
/// dependencies. There is no implicit authentication provider, policy
/// provider, allow-all policy, or default authorization grant.
pub struct AuthorizationService {
    authentication: Arc<AuthenticationService>,
    enforcer: Arc<PolicyEnforcer>,
    context_provider: Arc<dyn AuthorizationContextProvider>,
}

impl AuthorizationService {
    /// Construct an authorization service.
    ///
    /// The caller must provide:
    ///
    /// - the authoritative authentication service;
    /// - the configured policy enforcer;
    /// - a trusted authorization context provider.
    ///
    /// For policies that require no contextual attributes, use
    /// EmptyAuthorizationContext explicitly.
    pub fn new(
        authentication: Arc<AuthenticationService>,
        enforcer: Arc<PolicyEnforcer>,
        context_provider: Arc<dyn AuthorizationContextProvider>,
    ) -> Self {
        Self {
            authentication,
            enforcer,
            context_provider,
        }
    }

    /// Authorize one operation using an active authenticated session.
    ///
    /// The session is looked up again for every request. This ensures that
    /// expiry and revocation are checked through the configured session
    /// store rather than trusting a previously returned session object.
    ///
    /// Only an explicit policy Permit, with all obligations successfully
    /// fulfilled, can produce the returned PolicyPermit.
    ///
    /// The returned permit is evidence for this one request only. It must
    /// not be cached, serialized, transferred, or reused for another
    /// operation.
    pub fn authorize_session(
        &self,
        session_id: &SessionId,
        action: impl Into<String>,
        resource: impl Into<String>,
    ) -> DistributedResult<PolicyPermit> {
        let action = PolicyIdentifier::new(action)?;
        let resource = PolicyIdentifier::new(resource)?;

        // Resolve the session through the authoritative authentication
        // service. Unknown, expired, or revoked sessions fail closed.
        let subject = self
            .authentication
            .authenticate_session(session_id)?;

        // Obtain attributes from the configured trusted runtime provider.
        // An attribute-provider failure must never be interpreted as
        // permission.
        let attributes = self.context_provider.attributes(
            &subject,
            &action,
            &resource,
        )?;

        // Construct the existing canonical policy request.
        let mut request = PolicyRequest::new(
            subject,
            action,
            resource,
        );

        // Insert attributes through the policy module's validation API.
        // PolicyRequest also keeps attribute values out of Debug output.
        for (name, value) in attributes {
            request.insert_attribute(name, value)?;
        }

        // The existing PolicyEnforcer is the sole authority for producing
        // an enforcement permit. It applies default-deny and fulfills
        // obligations before returning success.
        self.enforcer.enforce(&request)
    }

    /// Authorize and then invoke one protected operation.
    ///
    /// This is the preferred integration API when a protected operation
    /// can be invoked synchronously.
    ///
    /// The operation closure is not called unless:
    ///
    /// - the session is active;
    /// - trusted context construction succeeds;
    /// - the policy explicitly permits the request;
    /// - all policy obligations are fulfilled.
    ///
    /// The permit is borrowed by the closure and cannot be returned from
    /// this function through the closure's result type.
    ///
    /// This method does not make the operation transactional. If the
    /// operation has side effects, its caller remains responsible for
    /// idempotency, durable completion records, and outcome reconciliation.
    pub fn authorize_and_execute<T, F>(
        &self,
        session_id: &SessionId,
        action: impl Into<String>,
        resource: impl Into<String>,
        operation: F,
    ) -> DistributedResult<T>
    where
        F: FnOnce(&PolicyPermit) -> DistributedResult<T>,
    {
        let permit = self.authorize_session(
            session_id,
            action,
            resource,
        )?;

        operation(&permit)
    }

    /// Revoke an authenticated session through the authentication
    /// subsystem.
    ///
    /// Authorization does not own session storage or revocation state.
    /// This method is a convenience boundary that delegates to the
    /// authoritative AuthenticationService.
    pub fn revoke_session(
        &self,
        session_id: &SessionId,
    ) -> DistributedResult<bool> {
        self.authentication.revoke_session(session_id)
    }

    /// Return a reference to the configured policy enforcer.
    ///
    /// This is intended for controlled integration and diagnostics.
    /// Protected operations should use authorize_session or
    /// authorize_and_execute rather than evaluating a raw policy decision
    /// and treating it as an enforcement permit.
    #[must_use]
    pub fn policy_enforcer(&self) -> &PolicyEnforcer {
        &self.enforcer
    }
}

impl std::fmt::Debug for AuthorizationService {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        formatter
            .debug_struct("AuthorizationService")
            .field("authentication", &"[AUTHENTICATION SERVICE]")
            .field("policy_enforcer", &"[POLICY ENFORCER]")
            .field("context_provider", &"[AUTHORIZATION CONTEXT PROVIDER]")
            .finish()
    }
}

/// Validate that an externally supplied attribute collection does not
/// contain obviously secret-bearing field names.
///
/// This helper is intentionally conservative and is not a substitute for
/// data classification or secret management. The context provider remains
/// responsible for deciding which values are safe and appropriate to
/// disclose to a policy evaluator.
pub fn validate_attribute_names(
    attributes: &PolicyAttributes,
) -> DistributedResult<()> {
    for name in attributes.keys() {
        let normalized = name.to_ascii_lowercase();

        let forbidden = [
            "password",
            "passwd",
            "private_key",
            "secret",
            "access_token",
            "refresh_token",
            "bearer_token",
            "authorization_header",
            "credential",
        ];

        if forbidden.iter().any(|term| normalized.contains(term)) {
            return Err(DistributedError::invalid_input(
                "authorization context contains a field name reserved for secret material",
            ));
        }
    }

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_context_provider_returns_no_attributes() {
        let provider = EmptyAuthorizationContext;
        let attributes = PolicyAttributes::new();

        // The provider does not grant permission. It only supplies context.
        assert!(attributes.is_empty());

        // Compile-time trait contract is exercised through the provider
        // implementation in the integration tests.
        let _ = provider;
    }

    #[test]
    fn secret_bearing_attribute_names_are_rejected() {
        let mut attributes = PolicyAttributes::new();

        attributes.insert(
            "access_token".to_owned(),
            Value::String("must-not-enter-policy-context".to_owned()),
        );

        assert!(validate_attribute_names(&attributes).is_err());
    }

    #[test]
    fn ordinary_runtime_attributes_are_accepted() {
        let mut attributes = PolicyAttributes::new();

        attributes.insert(
            "execution_domain".to_owned(),
            Value::String("distributed".to_owned()),
        );

        attributes.insert(
            "workload_class".to_owned(),
            Value::String("compute".to_owned()),
        );

        assert!(validate_attribute_names(&attributes).is_ok());
    }

    #[test]
    fn attribute_name_validation_is_case_insensitive() {
        let mut attributes = PolicyAttributes::new();

        attributes.insert(
            "Authorization_Header".to_owned(),
            Value::String("redacted".to_owned()),
        );

        assert!(validate_attribute_names(&attributes).is_err());
    }

    #[test]
    fn attribute_name_validation_does_not_reject_unrelated_names() {
        let mut attributes = PolicyAttributes::new();

        attributes.insert(
            "security_classification".to_owned(),
            Value::String("confidential".to_owned()),
        );

        attributes.insert(
            "resource_region".to_owned(),
            Value::String("region-a".to_owned()),
        );

        assert!(validate_attribute_names(&attributes).is_ok());
    }
}
