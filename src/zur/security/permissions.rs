
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani Universal Runtime — Permission Enforcement
//!
//! Path: `src/zur/security/permissions.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97.1
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provides the ZUR integration boundary for enforcing permissions
//! on protected Zamani operations.
//!
//! This module connects ZUR execution with the canonical distributed
//! authentication, authorization, and policy-enforcement authorities.
//!
//! It supports:
//!
//! - Open-ended permission identifiers.
//! - Resource-scoped permissions.
//! - Authenticated-session authorization.
//! - Policy-backed permission enforcement.
//! - Authorize-before-execute operation boundaries.
//! - Explicit policy-denial propagation.
//! - Policy-obligation enforcement through the canonical enforcer.
//! - Extensible permission namespaces.
//! - Classical, quantum, HDL, AI, hybrid, and distributed workloads.
//! - Future computational domains without modifying this module.
//!
//! ## ARCHITECTURAL AUTHORITIES
//!
//! This implementation integrates with:
//!
//! - `grammar/security/permissions.g4`
//! - `grammar/security/authorization.g4`
//! - `grammar/security/policies.g4`
//! - `grammar/security/README.md`
//! - `grammar/DESIGN.md`
//! - `grammar/specification/semantic-model.md`
//! - `grammar/validation/semantic-boundaries.md`
//! - `src/frontend/semantic/`
//! - `src/frontend/semantic/capabilities/`
//! - `src/distributed/security/identity.rs`
//! - `src/distributed/security/authentication.rs`
//! - `src/distributed/security/policy.rs`
//! - `src/distributed/security/authorization.rs`
//! - `src/distributed/error.rs`
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - The ZUR permission-enforcement integration.
//! - Validated runtime permission identifiers.
//! - Permission-to-policy-action adaptation.
//! - Resource-scoped authorization requests.
//! - Authorize-before-execute integration.
//!
//! This module does NOT own:
//!
//! - Permission grammar.
//! - AST construction.
//! - Semantic permission declarations.
//! - Permission name resolution.
//! - Authentication.
//! - Principal identity.
//! - Trust establishment.
//! - Credential storage.
//! - Cryptographic operations.
//! - Policy definition or storage.
//! - Policy evaluation algorithms.
//! - Grant and revocation persistence.
//! - Capability discovery.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling.
//! - Classical, quantum, or HDL IR.
//! - Physical execution.
//!
//! These responsibilities remain with their existing authorities.
//!
//! ## SECURITY INVARIANTS
//!
//! 1. An unauthenticated identity cannot authorize an operation.
//! 2. A permission declaration does not grant permission.
//! 3. A capability does not grant permission.
//! 4. Authentication does not imply authorization.
//! 5. Authorization requires an active authenticated session.
//! 6. Only the canonical PolicyEnforcer can produce a policy permit.
//! 7. Deny, NotApplicable, and Indeterminate never authorize execution.
//! 8. Policy evaluation errors never authorize execution.
//! 9. Policy obligations must be fulfilled before execution.
//! 10. The protected operation must execute only after authorization.
//! 11. Authorization context must originate from trusted authorities.
//! 12. Permission identifiers never imply physical device selection.
//! 13. No permission is implicitly granted by this module.
//! 14. No global mutable authorization state is maintained.
//! 15. No authorization result is cached or reused by this module.
//!
//! ## SCALABILITY
//!
//! Permission identifiers use an open-ended namespace.
//!
//! This module introduces no universal maximum for:
//!
//! - Permissions.
//! - Principals.
//! - Resources.
//! - Actions.
//! - Policies.
//! - Providers.
//! - Devices.
//! - Nodes.
//! - Tasks.
//! - Computational domains.
//! - Hardware resources.
//!
//! Practical resource consumption is determined by the actual allocator,
//! policy implementation, authentication provider, and configured
//! operational budgets.
//!
//! Literal infinite computation is not possible on finite resources.
//! No arbitrary universal capacity ceiling is introduced here.
//!
//! ## POCO-REAF
//!
//! Permission names are independent of the physical execution target.
//!
//! The same permission identifier can be enforced against compatible
//! operations on different machines and computational domains.
//!
//! The runtime does not interpret a permission as a hardware capability
//! or select a hardware provider based on its name.

use core::fmt;
use std::sync::Arc;

use crate::distributed::error::DistributedResult;

use crate::distributed::security::authentication::SessionId;

use crate::distributed::security::authorization::AuthorizationService;

use crate::distributed::security::policy::{
    PolicyIdentifier,
    PolicyPermit,
};

/// Permission-specific result type.
///
/// Errors are delegated to the existing distributed security
/// error authority.
pub type PermissionResult<T> = DistributedResult<T>;

// ============================================================================
// Permission identifier
// ============================================================================

/// A validated, open-ended permission identifier.
///
/// The identifier is adapted to the existing PolicyIdentifier
/// representation when an authorization request is submitted.
///
/// No fixed permission vocabulary is defined.
///
/// Examples:
///
/// - `read_data`
/// - `project::read`
/// - `quantum_execution`
/// - `fpga_reconfiguration`
/// - `distributed.execution`
/// - `future.security.operation`
///
/// The identifier represents security intent, not a capability
/// or an authorization grant.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub struct PermissionIdentifier {
    identifier: PolicyIdentifier,
}

impl PermissionIdentifier {
    /// Construct a validated permission identifier.
    ///
    /// Validation is delegated to the canonical PolicyIdentifier
    /// implementation to avoid introducing a competing identifier
    /// validation model.
    pub fn new(
        value: impl Into<String>,
    ) -> PermissionResult<Self> {
        Ok(Self {
            identifier: PolicyIdentifier::new(value)?,
        })
    }

    /// Return the validated identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.identifier.as_str()
    }

    /// Return the canonical policy identifier.
    ///
    /// This is useful for integration with existing security
    /// policy adapters.
    #[must_use]
    pub fn as_policy_identifier(&self) -> &PolicyIdentifier {
        &self.identifier
    }
}

impl fmt::Display for PermissionIdentifier {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl TryFrom<String> for PermissionIdentifier {
    type Error = crate::distributed::error::DistributedError;

    fn try_from(
        value: String,
    ) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

// ============================================================================
// Permission authorization evidence
// ============================================================================

/// Request-scoped evidence that a permission was authorized.
///
/// This type is available only inside the protected operation callback.
///
/// It borrows the canonical PolicyPermit rather than manufacturing
/// an independent authorization token.
///
/// It cannot be serialized, cloned, or retained as an owned permit
/// through the normal API.
pub struct PermissionPermit<'a> {
    permission: &'a PermissionIdentifier,
    resource: &'a PolicyIdentifier,
    policy_permit: &'a PolicyPermit,
}

impl<'a> PermissionPermit<'a> {
    /// Return the permission that was authorized.
    #[must_use]
    pub fn permission(&self) -> &PermissionIdentifier {
        self.permission
    }

    /// Return the protected resource.
    #[must_use]
    pub fn resource(&self) -> &PolicyIdentifier {
        self.resource
    }

    /// Return the canonical policy identifier.
    #[must_use]
    pub fn policy_id(&self) -> &PolicyIdentifier {
        self.policy_permit.policy_id()
    }

    /// Return the canonical policy rule identifier.
    #[must_use]
    pub fn rule_id(&self) -> &PolicyIdentifier {
        self.policy_permit.rule_id()
    }

    /// Return the principal authorized by the canonical policy.
    #[must_use]
    pub fn principal(
        &self,
    ) -> &crate::distributed::security::identity::PrincipalId {
        self.policy_permit.principal()
    }
}

impl fmt::Debug for PermissionPermit<'_> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("PermissionPermit")
            .field("permission", &self.permission)
            .field("resource", &self.resource)
            .field("policy_id", &self.policy_permit.policy_id())
            .field("rule_id", &self.policy_permit.rule_id())
            .finish()
    }
}

// ============================================================================
// Permission authorization service
// ============================================================================

/// ZUR permission authorization integration.
///
/// This service delegates all actual authorization to the existing
/// distributed AuthorizationService.
///
/// It does not create a second policy engine, authentication
/// subsystem, authorization registry, or grant database.
///
/// The AuthorizationService is mandatory.
///
/// There is no implicit allow-all configuration.
pub struct PermissionAuthorizer {
    authorization: Arc<AuthorizationService>,
}

impl PermissionAuthorizer {
    /// Construct a permission authorizer using the canonical
    /// distributed authorization service.
    ///
    /// The caller must provide an already configured authorization
    /// service containing:
    ///
    /// - An authoritative authentication service.
    /// - An authenticated-session store.
    /// - A configured policy enforcer.
    /// - A trusted authorization-context provider.
    ///
    /// No authentication or policy implementation is silently selected.
    #[must_use]
    pub fn new(
        authorization: Arc<AuthorizationService>,
    ) -> Self {
        Self {
            authorization,
        }
    }

    /// Authorize a protected operation and execute it only after
    /// successful authorization.
    ///
    /// The authorization service:
    ///
    /// 1. Validates the authenticated session.
    /// 2. Resolves the authenticated subject.
    /// 3. Obtains trusted authorization attributes.
    /// 4. Constructs the canonical policy request.
    /// 5. Evaluates the configured policy.
    /// 6. Enforces policy obligations.
    /// 7. Produces the canonical PolicyPermit.
    ///
    /// Only then is `operation` invoked.
    ///
    /// The operation receives a borrowed PermissionPermit.
    ///
    /// The permit cannot be returned through the callback's result
    /// because its lifetime is restricted to this operation.
    ///
    /// The callback is not invoked when authorization fails.
    ///
    /// This function does not make external side effects transactional.
    /// The caller remains responsible for transactionality,
    /// idempotency, and reconciliation where required.
    pub fn authorize_and_execute<T, F>(
        &self,
        session_id: &SessionId,
        permission: &PermissionIdentifier,
        resource: &PolicyIdentifier,
        operation: F,
    ) -> PermissionResult<T>
    where
        F: FnOnce(&PermissionPermit<'_>) -> PermissionResult<T>,
    {
        self.authorization.authorize_and_execute(
            session_id,
            permission.as_str().to_owned(),
            resource.as_str().to_owned(),
            |policy_permit| {
                let permit = PermissionPermit {
                    permission,
                    resource,
                    policy_permit,
                };

                operation(&permit)
            },
        )
    }

    /// Return the canonical authorization service.
    ///
    /// This accessor is intended for controlled integration.
    ///
    /// Protected operations should normally use
    /// authorize_and_execute rather than independently constructing
    /// policy requests.
    #[must_use]
    pub fn authorization_service(
        &self,
    ) -> &AuthorizationService {
        &self.authorization
    }
}

impl fmt::Debug for PermissionAuthorizer {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("PermissionAuthorizer")
            .field(
                "authorization",
                &"[CANONICAL AUTHORIZATION SERVICE]",
            )
            .finish()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_open_ended_permission_names() {
        let identifiers = [
            "read_data",
            "quantum_execution",
            "fpga_reconfiguration",
            "distributed.execution",
            "project::read",
            "future.security.operation",
        ];

        for value in identifiers {
            let permission =
                PermissionIdentifier::new(value.to_owned())
                    .expect("valid permission identifier");

            assert_eq!(permission.as_str(), value);
        }
    }

    #[test]
    fn rejects_empty_permission_identifier() {
        assert!(
            PermissionIdentifier::new(String::new()).is_err()
        );
    }

    #[test]
    fn rejects_permission_identifiers_with_whitespace() {
        assert!(
            PermissionIdentifier::new("read data".to_owned())
                .is_err()
        );

        assert!(
            PermissionIdentifier::new(" read_data".to_owned())
                .is_err()
        );

        assert!(
            PermissionIdentifier::new("read_data ".to_owned())
                .is_err()
        );
    }

    #[test]
    fn permission_identifier_preserves_namespace() {
        let permission =
            PermissionIdentifier::new(
                "project::quantum::execute".to_owned(),
            )
            .expect("valid permission identifier");

        assert_eq!(
            permission.as_str(),
            "project::quantum::execute"
        );
    }

    #[test]
    fn permission_identifier_uses_canonical_policy_identity() {
        let permission =
            PermissionIdentifier::new(
                "quantum.execution".to_owned(),
            )
            .expect("valid permission identifier");

        assert_eq!(
            permission.as_policy_identifier().as_str(),
            "quantum.execution"
        );
    }

    #[test]
    fn equivalent_permission_identifiers_are_equal() {
        let first =
            PermissionIdentifier::new(
                "distributed.execution".to_owned(),
            )
            .expect("valid identifier");

        let second =
            PermissionIdentifier::new(
                "distributed.execution".to_owned(),
            )
            .expect("valid identifier");

        assert_eq!(first, second);
    }

    #[test]
    fn different_permission_identifiers_are_distinct() {
        let first =
            PermissionIdentifier::new(
                "classical.execution".to_owned(),
            )
            .expect("valid identifier");

        let second =
            PermissionIdentifier::new(
                "quantum.execution".to_owned(),
            )
            .expect("valid identifier");

        assert_ne!(first, second);
    }

    #[test]
    fn permission_identifier_is_orderable() {
        let first =
            PermissionIdentifier::new("a.read".to_owned())
                .expect("valid identifier");

        let second =
            PermissionIdentifier::new("b.write".to_owned())
                .expect("valid identifier");

        assert!(first < second);
    }
}
