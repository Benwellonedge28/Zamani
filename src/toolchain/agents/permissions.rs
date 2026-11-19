
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Agent and Client Permissions
//!
//! Provides operation-specific authorization for toolchain clients,
//! including AI agents, IDEs, automation and human developers.
//!
//! Authorization is delegated to the canonical toolchain security
//! policy engine. This module does not implement a second policy
//! evaluator or authenticate identities.
//!
//! Every client must use the same command authorization boundary.
//!
//! ## Ownership
//!
//! This module owns:
//! - Integration of authenticated principals with toolchain commands.
//! - Command-to-action identity mapping.
//! - Canonical resource resolution.
//! - Authorization attribute acquisition.
//! - Policy evaluation orchestration.
//! - Authorization decision adaptation.
//! - Immutable authorization receipts.
//!
//! It does not own:
//! - Authentication or identity verification.
//! - Credential management.
//! - Trust establishment.
//! - Policy definitions or policy evaluation algorithms.
//! - Capability discovery or feasibility.
//! - Command execution.
//! - Runtime enforcement.
//! - Hardware discovery or allocation.
//! - Audit persistence.
//! - Cryptographic operations.
//!
//! ## Authoritative integrations
//!
//! Source-level security intent:
//!     grammar/security/permissions.g4
//!     grammar/security/authorization.g4
//!     grammar/spec/security.md
//!
//! Policy evaluation:
//!     crate::toolchain::security::policy
//!
//! Authentication:
//!     An injected authoritative identity provider.
//!
//! Trust:
//!     crate::toolchain::security::trust
//!
//! Aggregate security validation:
//!     crate::toolchain::security::validation
//!
//! Command dispatch:
//!     crate::toolchain::commands
//!
//! Errors:
//!     crate::toolchain::errors
//!
//! Audit:
//!     crate::toolchain::agents::audit
//!
//! ## Security invariants
//!
//! - Authorization is default-deny.
//! - Agent-supplied arguments cannot establish identity.
//! - Unknown evidence never grants permission.
//! - Indeterminate decisions never grant permission.
//! - Registration does not grant permission.
//! - Capabilities are not permissions.
//! - Authorization does not establish target feasibility.
//! - Policy evaluation precedes command execution.
//! - Human and AI callers share the same authorization contract.
//! - Evaluation budgets are operational safeguards.
//! - No arbitrary hardware or identity ceilings are imposed.
//!
//! ## Scalability
//!
//! Identifiers and policy collections are open-world.
//! Operational evaluation budgets are configurable.
//! Actual scalability remains subject to available resources.
//!
//! Literal infinite computation or storage is not promised.

use std::fmt;
use std::sync::Arc;

use crate::toolchain::commands::{
    CommandAuthorizer,
    CommandDescriptor,
    CommandError,
    CommandErrorKind,
    CommandRequest,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

use crate::toolchain::security::policy::{
    Action,
    CombiningAlgorithm,
    DecisionEffect,
    EvaluationBudget,
    PolicyAttributes,
    PolicyContext,
    PolicyDecision,
    PolicyEngine,
    PolicySet,
    Principal,
    ProtectedResource,
};

/// Stable identity of this authorization integration.
pub const COMPONENT: &str = "toolchain.agents.permissions";

/// Stable authorization operation identity.
pub const AUTHORIZE_COMMAND_OPERATION: &str =
    "toolchain.agents.permissions.authorize_command";

/// Stable authorization API version.
pub const PERMISSIONS_API_VERSION: u32 = 1;

// ============================================================================
// AUTHENTICATED PRINCIPAL PROVIDER
// ============================================================================

/// Supplies the principal established by an authoritative
/// authentication boundary.
///
/// Implementations must obtain identity from a trusted authenticated
/// session or identity provider.
///
/// They must never derive authenticated identity from command arguments,
/// an agent-provided identity string, or an unverified request field.
///
/// This interface does not itself authenticate the principal.
pub trait AuthenticatedPrincipalProvider: Send + Sync + 'static {
    /// Return the principal established for this client session.
    fn authenticated_principal(&self) -> ToolchainResult<Principal>;

    /// Return an opaque, non-secret authentication evidence reference.
    ///
    /// The reference is suitable for audit correlation.
    /// It must not contain credentials or authentication tokens.
    fn authentication_reference(&self) -> Option<String> {
        None
    }
}

// ============================================================================
// CANONICAL RESOURCE RESOLUTION
// ============================================================================

/// Resolves the protected resource associated with a command.
///
/// Implementations must resolve resources through the appropriate
/// workspace, project, package, registry or deployment authority.
///
/// Raw paths and other caller-controlled values must not be treated
/// as canonical resource identities without validation.
///
/// A missing or ambiguous resource must produce an error rather
/// than an unrestricted resource identity.
pub trait AuthorizationResourceResolver: Send + Sync + 'static {
    /// Resolve the canonical resource for one command request.
    fn resolve_resource(
        &self,
        descriptor: &CommandDescriptor,
        request: &CommandRequest,
    ) -> ToolchainResult<ProtectedResource>;
}

// ============================================================================
// AUTHORIZATION ATTRIBUTE PROVIDER
// ============================================================================

/// Supplies policy attributes obtained from authoritative sources.
///
/// Attributes can represent authenticated identity properties,
/// workspace classifications, package ownership, operation context,
/// security validation results and other policy-relevant evidence.
///
/// Implementations must only mark evidence as verified when the
/// owning authority has actually established that evidence.
///
/// Missing or unverified attributes cannot satisfy policy conditions.
pub trait AuthorizationAttributeProvider: Send + Sync + 'static {
    /// Resolve policy attributes for an authorization request.
    fn resolve_attributes(
        &self,
        principal: &Principal,
        descriptor: &CommandDescriptor,
        request: &CommandRequest,
    ) -> ToolchainResult<PolicyAttributes>;
}

// ============================================================================
// AUTHORIZATION RECEIPT
// ============================================================================

/// Immutable record of an evaluated command authorization.
///
/// This is an authorization result, not an audit-storage record.
/// The audit subsystem owns persistence and retention.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationReceipt {
    command: String,
    principal: String,
    resource: String,
    correlation_id: Option<String>,
    authentication_reference: Option<String>,
    decision: PolicyDecision,
}

impl AuthorizationReceipt {
    /// Return the authorized command identity.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Return the authenticated principal identity.
    #[must_use]
    pub fn principal(&self) -> &str {
        &self.principal
    }

    /// Return the canonical protected resource identity.
    #[must_use]
    pub fn resource(&self) -> &str {
        &self.resource
    }

    /// Return the optional caller correlation identifier.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    /// Return the optional authentication evidence reference.
    #[must_use]
    pub fn authentication_reference(&self) -> Option<&str> {
        self.authentication_reference.as_deref()
    }

    /// Return the complete policy decision.
    #[must_use]
    pub fn decision(&self) -> &PolicyDecision {
        &self.decision
    }

    /// Return whether the policy engine explicitly permitted the request.
    #[must_use]
    pub fn is_permitted(&self) -> bool {
        self.decision.is_permitted()
    }

    /// Return the policy evaluation effect.
    #[must_use]
    pub fn effect(&self) -> DecisionEffect {
        self.decision.effect()
    }
}

// ============================================================================
// AUTHORITATIVE PERMISSION SERVICE
// ============================================================================

/// Thread-safe command permission service.
///
/// This service implements the existing `CommandAuthorizer` contract.
///
/// It is deliberately independent of whether the requesting client
/// is a human, an AI agent, an IDE or an automation system.
///
/// Each instance is associated with one authenticated client identity
/// provider and one immutable policy-set snapshot.
///
/// Policy updates should create a new service instance and replace
/// the previous instance through the owning configuration lifecycle.
#[derive(Clone)]
pub struct AgentPermissionAuthorizer {
    identity: Arc<dyn AuthenticatedPrincipalProvider>,
    resources: Arc<dyn AuthorizationResourceResolver>,
    attributes: Arc<dyn AuthorizationAttributeProvider>,
    policies: Arc<PolicySet>,
    engine: PolicyEngine,
    budget: EvaluationBudget,
}

impl fmt::Debug for AgentPermissionAuthorizer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AgentPermissionAuthorizer")
            .field("policy_count", &self.policies.len())
            .field("combining_algorithm", &self.policies.algorithm())
            .field("evaluation_budget", &self.budget)
            .finish_non_exhaustive()
    }
}

impl AgentPermissionAuthorizer {
    /// Construct an authorization service.
    ///
    /// All three providers must be supplied explicitly.
    ///
    /// The policy set must be nonempty and use the currently supported
    /// `FirstApplicable` combining algorithm.
    ///
    /// An absent or incomplete policy configuration must never create
    /// an implicit allow-all authorization service.
    pub fn new(
        identity: Arc<dyn AuthenticatedPrincipalProvider>,
        resources: Arc<dyn AuthorizationResourceResolver>,
        attributes: Arc<dyn AuthorizationAttributeProvider>,
        policies: PolicySet,
        budget: EvaluationBudget,
    ) -> ToolchainResult<Self> {
        if policies.is_empty() {
            return Err(
                ToolchainError::invalid_configuration(
                    "authorization policy set must not be empty",
                )
                .with_operation(AUTHORIZE_COMMAND_OPERATION)
                .with_context(COMPONENT),
            );
        }

        if policies.algorithm() != CombiningAlgorithm::FirstApplicable {
            return Err(
                ToolchainError::invalid_configuration(
                    "the current policy evaluator requires FirstApplicable",
                )
                .with_operation(AUTHORIZE_COMMAND_OPERATION)
                .with_context(COMPONENT),
            );
        }

        Ok(Self {
            identity,
            resources,
            attributes,
            policies: Arc::new(policies),
            engine: PolicyEngine::new(),
            budget,
        })
    }

    /// Return the configured policy evaluation budget.
    #[must_use]
    pub fn evaluation_budget(&self) -> &EvaluationBudget {
        &self.budget
    }

    /// Return the number of configured policies.
    #[must_use]
    pub fn policy_count(&self) -> usize {
        self.policies.len()
    }

    /// Evaluate authorization and return the complete decision receipt.
    ///
    /// A receipt is returned for both permitted and rejected decisions.
    ///
    /// Provider failures are returned as errors. They must never
    /// be converted into successful authorization.
    pub fn authorize_with_receipt(
        &self,
        descriptor: &CommandDescriptor,
        request: &CommandRequest,
    ) -> ToolchainResult<AuthorizationReceipt> {
        self.validate_command_binding(descriptor, request)?;

        // Identity must originate from the injected authentication
        // authority, never from caller-controlled command arguments.
        let principal = self
            .identity
            .authenticated_principal()
            .map_err(|error| {
                ToolchainError::authentication_failed(
                    "authenticated principal could not be established",
                )
                .with_operation(AUTHORIZE_COMMAND_OPERATION)
                .with_source(error)
                .with_context(COMPONENT)
            })?;

        let authentication_reference =
            self.identity.authentication_reference();

        // Resolve the protected resource through its authoritative
        // project/workspace/package/provider integration.
        let resource = self
            .resources
            .resolve_resource(descriptor, request)
            .map_err(|error| {
                ToolchainError::validation_failed(
                    "canonical authorization resource could not be resolved",
                )
                .with_operation(AUTHORIZE_COMMAND_OPERATION)
                .with_source(error)
                .with_context(COMPONENT)
            })?;

        // Every command has its own action identity.
        //
        // The identifier remains open-world so new commands and
        // domain-specific operations need no closed permission enum.
        let action = Action::new(format!(
            "toolchain.command.{}",
            descriptor.id.as_str()
        ))?;

        // Only authoritative attribute providers may establish
        // verified policy evidence.
        let attributes = self
            .attributes
            .resolve_attributes(
                &principal,
                descriptor,
                request,
            )
            .map_err(|error| {
                ToolchainError::validation_failed(
                    "authorization attributes could not be established",
                )
                .with_operation(AUTHORIZE_COMMAND_OPERATION)
                .with_source(error)
                .with_context(COMPONENT)
            })?;

        let mut context = PolicyContext::new(
            principal.clone(),
            action,
            resource.clone(),
        );

        for (path, fact) in attributes {
            context = context.with_attribute(path, fact);
        }

        // The existing canonical security policy engine performs
        // the actual policy evaluation.
        let decision = self
            .engine
            .evaluate(
                self.policies.as_ref(),
                &context,
                &self.budget,
            )
            .map_err(|error| {
                ToolchainError::authentication_failed(
                    "authorization policy evaluation failed",
                )
                .with_operation(AUTHORIZE_COMMAND_OPERATION)
                .with_source(error)
                .with_context(COMPONENT)
            })?;

        Ok(AuthorizationReceipt {
            command: descriptor.id.as_str().to_owned(),
            principal: principal.identity().to_owned(),
            resource: resource.as_str().to_owned(),
            correlation_id: request.correlation_id.clone(),
            authentication_reference,
            decision,
        })
    }

    /// Validate that the descriptor and request refer to the same command.
    ///
    /// This prevents a caller from supplying a different command request
    /// after the dispatcher has selected a descriptor.
    fn validate_command_binding(
        &self,
        descriptor: &CommandDescriptor,
        request: &CommandRequest,
    ) -> ToolchainResult<()> {
        if descriptor.id != request.command {
            return Err(
                ToolchainError::invalid_input(
                    "command descriptor and request identities do not match",
                )
                .with_operation(AUTHORIZE_COMMAND_OPERATION)
                .with_context(COMPONENT),
            );
        }

        if descriptor.api_version == 0 {
            return Err(
                ToolchainError::invalid_configuration(
                    "command descriptor has an invalid API version",
                )
                .with_operation(AUTHORIZE_COMMAND_OPERATION)
                .with_context(COMPONENT),
            );
        }

        Ok(())
    }

    /// Convert an authorization receipt into a command-dispatch decision.
    ///
    /// Only an explicit Permit can produce success.
    ///
    /// Deny and NotApplicable are authorization denials.
    /// Indeterminate is an authorization availability failure.
    fn command_result(
        descriptor: &CommandDescriptor,
        receipt: &AuthorizationReceipt,
    ) -> Result<(), CommandError> {
        match receipt.effect() {
            DecisionEffect::Permit => Ok(()),

            DecisionEffect::Deny
            | DecisionEffect::NotApplicable => {
                Err(
                    CommandError::new(
                        CommandErrorKind::AuthorizationDenied,
                        format!(
                            "authorization denied for principal '{}' on resource '{}'",
                            receipt.principal(),
                            receipt.resource(),
                        ),
                    )
                    .for_command(descriptor.id.clone()),
                )
            }

            DecisionEffect::Indeterminate => {
                Err(
                    CommandError::new(
                        CommandErrorKind::AuthorizationUnavailable,
                        format!(
                            "authorization could not establish permission: {:?}",
                            receipt.decision().reason(),
                        ),
                    )
                    .for_command(descriptor.id.clone()),
                )
            }
        }
    }
}

// ============================================================================
// EXISTING COMMAND DISPATCH INTEGRATION
// ============================================================================

impl CommandAuthorizer for AgentPermissionAuthorizer {
    /// Authorize a command before its handler is executed.
    ///
    /// The command dispatcher must invoke this method before
    /// executing the command handler.
    fn authorize(
        &self,
        descriptor: &CommandDescriptor,
        request: &CommandRequest,
    ) -> Result<(), CommandError> {
        let receipt = self
            .authorize_with_receipt(descriptor, request)
            .map_err(|error| {
                CommandError::new(
                    CommandErrorKind::AuthorizationUnavailable,
                    "authorization evidence or evaluation is unavailable",
                )
                .for_command(descriptor.id.clone())
                .with_source(error)
            })?;

        Self::command_result(descriptor, &receipt)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::security::policy::{
        CombiningAlgorithm,
        PolicyEffect,
        PolicyId,
        PolicyRevision,
        PolicyRule,
        RuleId,
        RulePriority,
        Selector,
        SecurityPolicy,
    };

    use std::collections::BTreeMap;

    struct TestIdentity;

    impl AuthenticatedPrincipalProvider for TestIdentity {
        fn authenticated_principal(
            &self,
        ) -> ToolchainResult<Principal> {
            Principal::new("user:alice")
        }

        fn authentication_reference(&self) -> Option<String> {
            Some("test-session-reference".to_owned())
        }
    }

    struct TestResource;

    impl AuthorizationResourceResolver for TestResource {
        fn resolve_resource(
            &self,
            _descriptor: &CommandDescriptor,
            _request: &CommandRequest,
        ) -> ToolchainResult<ProtectedResource> {
            ProtectedResource::new("project:example")
        }
    }

    struct EmptyAttributes;

    impl AuthorizationAttributeProvider for EmptyAttributes {
        fn resolve_attributes(
            &self,
            _principal: &Principal,
            _descriptor: &CommandDescriptor,
            _request: &CommandRequest,
        ) -> ToolchainResult<PolicyAttributes> {
            Ok(BTreeMap::new())
        }
    }

    fn policy_set(
        effect: PolicyEffect,
        action: &str,
    ) -> PolicySet {
        let rule = PolicyRule::new(
            RuleId::new("rule:build").expect("valid rule ID"),
            effect,
            RulePriority::new(0),
            Selector::exact("user:alice").expect("valid selector"),
            Selector::exact(action).expect("valid selector"),
            Selector::exact("project:example")
                .expect("valid selector"),
            None,
        );

        let policy = SecurityPolicy::new(
            PolicyId::new("policy:toolchain")
                .expect("valid policy ID"),
            PolicyRevision::new(1).expect("valid revision"),
            vec![rule],
        )
        .expect("valid security policy");

        let mut policies =
            PolicySet::new(CombiningAlgorithm::FirstApplicable);

        policies.insert(policy).expect("policy insertion");

        policies
    }

    fn authorizer(
        effect: PolicyEffect,
        action: &str,
    ) -> AgentPermissionAuthorizer {
        AgentPermissionAuthorizer::new(
            Arc::new(TestIdentity),
            Arc::new(TestResource),
            Arc::new(EmptyAttributes),
            policy_set(effect, action),
            EvaluationBudget::unlimited(),
        )
        .expect("valid authorization configuration")
    }

    fn command() -> (CommandDescriptor, CommandRequest) {
        let id = crate::toolchain::commands::CommandId::new("build")
            .expect("valid command ID");

        let descriptor = CommandDescriptor::new(
            id.clone(),
            "Build a Zamani project",
        )
        .requires_workspace();

        let request = CommandRequest::new(id)
            .correlation_id("request:test-1");

        (descriptor, request)
    }

    #[test]
    fn explicit_permit_authorizes_command() {
        let (descriptor, request) = command();

        let authorizer = authorizer(
            PolicyEffect::Permit,
            "toolchain.command.build",
        );

        let receipt = authorizer
            .authorize_with_receipt(&descriptor, &request)
            .expect("policy evaluation must complete");

        assert!(receipt.is_permitted());
        assert_eq!(receipt.principal(), "user:alice");
        assert_eq!(receipt.resource(), "project:example");
        assert_eq!(
            receipt.authentication_reference(),
            Some("test-session-reference"),
        );

        assert!(
            authorizer.authorize(&descriptor, &request).is_ok()
        );
    }

    #[test]
    fn explicit_deny_rejects_command() {
        let (descriptor, request) = command();

        let authorizer = authorizer(
            PolicyEffect::Deny,
            "toolchain.command.build",
        );

        let receipt = authorizer
            .authorize_with_receipt(&descriptor, &request)
            .expect("policy evaluation must complete");

        assert!(!receipt.is_permitted());

        let error = authorizer
            .authorize(&descriptor, &request)
            .expect_err("explicit denial must block execution");

        assert_eq!(
            error.kind,
            CommandErrorKind::AuthorizationDenied,
        );
    }

    #[test]
    fn missing_matching_rule_uses_default_deny() {
        let (descriptor, request) = command();

        let authorizer = authorizer(
            PolicyEffect::Permit,
            "toolchain.command.publish",
        );

        let receipt = authorizer
            .authorize_with_receipt(&descriptor, &request)
            .expect("policy evaluation must complete");

        assert_eq!(
            receipt.effect(),
            DecisionEffect::NotApplicable,
        );

        assert!(!receipt.is_permitted());
    }

    #[test]
    fn mismatched_command_binding_is_rejected() {
        let (descriptor, mut request) = command();

        request.command =
            crate::toolchain::commands::CommandId::new("publish")
                .expect("valid command ID");

        let authorizer = authorizer(
            PolicyEffect::Permit,
            "toolchain.command.build",
        );

        assert!(
            authorizer
                .authorize_with_receipt(&descriptor, &request)
                .is_err()
        );
    }

    #[test]
    fn empty_policy_set_is_rejected() {
        let result = AgentPermissionAuthorizer::new(
            Arc::new(TestIdentity),
            Arc::new(TestResource),
            Arc::new(EmptyAttributes),
            PolicySet::new(CombiningAlgorithm::FirstApplicable),
            EvaluationBudget::unlimited(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn unsupported_combining_algorithm_is_rejected() {
        let (descriptor, request) = command();

        let rule = PolicyRule::new(
            RuleId::new("rule:build").expect("valid ID"),
            PolicyEffect::Permit,
            RulePriority::new(0),
            Selector::exact("user:alice").expect("valid"),
            Selector::exact("toolchain.command.build")
                .expect("valid"),
            Selector::exact("project:example")
                .expect("valid"),
            None,
        );

        let policy = SecurityPolicy::new(
            PolicyId::new("policy:toolchain").expect("valid"),
            PolicyRevision::new(1).expect("valid"),
            vec![rule],
        )
        .expect("valid policy");

        let mut policies =
            PolicySet::new(CombiningAlgorithm::PermitOverrides);

        policies.insert(policy).expect("insert");

        let result = AgentPermissionAuthorizer::new(
            Arc::new(TestIdentity),
            Arc::new(TestResource),
            Arc::new(EmptyAttributes),
            policies,
            EvaluationBudget::unlimited(),
        );

        assert!(result.is_err());

        let _ = (descriptor, request);
    }
}
