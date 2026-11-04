#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Security — Authorization
//!
//! Path: `src/ai/security/authorization.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the AI-domain authorization boundary.
//!
//! This module connects AI operations and AI resources to Zamani's
//! canonical distributed authentication and policy-enforcement
//! infrastructure.
//!
//! It ensures that protected AI operations are authorized against:
//!
//! - an active authenticated session;
//! - an AI-scoped operation;
//! - a specific AI resource;
//! - trusted runtime authorization context;
//! - the configured canonical policy engine;
//! - all applicable policy obligations.
//!
//! # Architectural ownership
//!
//! This module is an AI-domain integration adapter.
//!
//! It does not introduce another authentication or authorization
//! architecture.
//!
//! Canonical authorities:
//!
//! - `ai::security::identity`
//!   Owns AI identity and principal references.
//!
//! - `ai::ids`
//!   Owns AI-domain typed identifiers.
//!
//! - `ai::capabilities`
//!   Owns AI capability integration.
//!
//! - `ai::provenance`
//!   Owns AI provenance integration.
//!
//! - `distributed::security::authentication`
//!   Owns authenticated sessions and session lifecycle.
//!
//! - `distributed::security::authorization`
//!   Owns distributed authorization orchestration.
//!
//! - `distributed::security::policy`
//!   Owns policy requests, policy evaluation, enforcement,
//!   obligations and enforcement permits.
//!
//! - `frontend::semantic`
//!   Owns source-level security and authorization semantics.
//!
//! - `grammar/security/authorization.g4`
//!   Owns source-level authorization syntax.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - AI-scoped authorization action references;
//! - AI authorization request descriptions;
//! - AI resource-to-policy-reference adaptation;
//! - AI authorization service integration;
//! - authorization-before-AI-execution orchestration.
//!
//! # Does not own
//!
//! This module does not own:
//!
//! - identity allocation;
//! - principal authentication;
//! - credential validation;
//! - cryptographic operations;
//! - trust establishment;
//! - capability issuance;
//! - capability discovery;
//! - permission declaration;
//! - generic policy evaluation;
//! - policy storage;
//! - policy distribution;
//! - delegation authority;
//! - grant storage;
//! - revocation storage;
//! - cryptographic key management;
//! - distributed transport;
//! - resource allocation;
//! - scheduling;
//! - hardware discovery;
//! - model execution;
//! - AI operation implementation;
//! - runtime execution infrastructure.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! # Security invariants
//!
//! 1. Authentication is not authorization.
//! 2. Identity is not authentication.
//! 3. Identity is not trust.
//! 4. Capability is not permission.
//! 5. Intelligence does not increase authority.
//! 6. An AI model cannot authorize itself.
//! 7. An AI agent identity reference is not proof of an authenticated
//!    executing agent.
//! 8. An AI resource reference does not prove ownership or access rights.
//! 9. An active session does not automatically authorize an operation.
//! 10. Every protected operation requires a fresh policy evaluation.
//! 11. Deny, NotApplicable and Indeterminate must not execute work.
//! 12. Policy evaluation errors must not execute work.
//! 13. Failed policy obligations must not execute work.
//! 14. A policy permit must not become a reusable authorization token.
//! 15. Caller-supplied AI identity information must not be treated as
//!     trusted runtime context.
//! 16. Authentication and policy infrastructure failures must fail closed.
//!
//! # POCO-REAF
//!
//! Authorization describes whether a particular authenticated subject
//! may perform a particular AI operation on a particular AI resource.
//!
//! Authorization does not encode:
//!
//! - processor architecture;
//! - accelerator vendor;
//! - CPU count;
//! - GPU count;
//! - memory capacity;
//! - tensor rank;
//! - model size;
//! - worker count;
//! - node count;
//! - cluster topology;
//! - quantum hardware topology.
//!
//! The same AI authorization intent remains meaningful across different
//! execution targets.
//!
//! # Scalability
//!
//! This module imposes no fixed maximum on:
//!
//! - AI principals;
//! - AI models;
//! - AI agents;
//! - AI resources;
//! - AI operations;
//! - authorization actions;
//! - authorization requests;
//! - policies;
//! - execution targets;
//! - distributed participants.
//!
//! Resource capacity is determined by the configured security providers,
//! available memory, storage, and execution environment.
//!
//! No internal global registry, background worker, or unbounded task
//! queue is created.
//!
//! Literal infinite physical capacity is not promised.
//!
//! # Authorization flow
//!
//! ```text
//! AI operation request
//!         |
//!         v
//! AI authorization request
//!         |
//!         v
//! Active authenticated session
//!         |
//!         v
//! Trusted runtime context
//!         |
//!         v
//! Canonical policy request
//!         |
//!         v
//! Existing PolicyEnforcer
//!         |
//!         v
//! Explicit policy Permit
//!         |
//!         v
//! Fulfil policy obligations
//!         |
//!         v
//! Request-scoped PolicyPermit
//!         |
//!         v
//! Protected AI operation
//! ```
//!
//! No operation is executed if any required security stage fails.
//!
//! # Integration
//!
//! `identity.rs`
//!     -> AiIdentityRef is used to identify the protected AI resource.
//!
//! `distributed/security/authentication.rs`
//!     -> SessionId identifies the authentication session.
//!
//! `distributed/security/authorization.rs`
//!     -> AuthorizationService performs authorization.
//!
//! `distributed/security/policy.rs`
//!     -> PolicyIdentifier and PolicyPermit.
//!
//! `frontend/semantic`
//!     -> Resolves source-level authorization intent.
//!
//! `distributed/security`
//!     -> Enforces authorization at distributed operation boundaries.
//!
//! `runtime`
//!     -> Invokes this boundary before protected AI execution.
//!
//! # Important integration rule
//!
//! The authenticated subject comes from the canonical authentication
//! service, never from an AI identity supplied by the caller.
//!
//! The AI resource identity is a resource reference, not proof that
//! the caller owns or controls that resource.
//!
//! The configured trusted authorization context provider and policy
//! engine must validate the actual access relationship.
//!
//! # Concurrency
//!
//! This module owns no mutable global state.
//!
//! The underlying distributed authorization service is shared through
//! `Arc` and must satisfy its own concurrency and lifecycle contracts.
//!
//! # Permit lifecycle
//!
//! A PolicyPermit is obtained and consumed inside
//! `authorize_and_execute`.
//!
//! This module does not expose a method that returns a permit for later
//! reuse.
//!
//! The caller must invoke the authorization boundary at the actual
//! protected-operation boundary.
//!
//! # Freeze criteria
//!
//! - Existing identity references are reused.
//! - Existing authentication is reused.
//! - Existing policy enforcement is reused.
//! - AI actions are explicitly namespaced.
//! - AI resource identities preserve their authority and category.
//! - Caller-supplied identities do not authenticate callers.
//! - Policy permits do not escape the protected-operation boundary.
//! - Failures prevent protected execution.
//! - No arbitrary scalability ceilings are introduced.
//! - No unsafe Rust is used.

use std::fmt;
use std::sync::Arc;

use crate::ai::security::identity::{
    AiIdentityCategory,
    AiIdentityRef,
};

use crate::distributed::error::{
    DistributedError,
    DistributedResult,
};

use crate::distributed::security::authentication::SessionId;

use crate::distributed::security::authorization::{
    AuthorizationService as DistributedAuthorizationService,
};

use crate::distributed::security::policy::{
    PolicyIdentifier,
    PolicyPermit,
};

// ============================================================================
// RESULT
// ============================================================================

/// Result returned by AI authorization operations.
///
/// The canonical distributed error vocabulary is preserved.
///
/// This prevents AI authorization from introducing a competing
/// security error hierarchy.
pub type AiAuthorizationResult<T> = DistributedResult<T>;

// ============================================================================
// AI AUTHORIZATION ACTION
// ============================================================================

/// An AI-scoped authorization action.
///
/// Actions use an open-ended symbolic namespace.
///
/// Examples:
///
/// - `ai.model.infer`
/// - `ai.model.train`
/// - `ai.model.update`
/// - `ai.agent.execute`
/// - `ai.agent.coordinate`
/// - `ai.dataset.read`
/// - `ai.dataset.transform`
/// - `ai.pipeline.execute`
/// - `ai.artifact.deploy`
/// - `ai.reasoning.infer`
/// - `ai.training.distribute`
///
/// This is a runtime policy identifier, not a source-language AST node.
///
/// Source-level names and references must first be resolved by the
/// canonical frontend and semantic infrastructure.
///
/// An AI action does not grant permission merely by existing.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct AiAuthorizationAction {
    identifier: PolicyIdentifier,
}

impl AiAuthorizationAction {
    /// Construct an AI-scoped authorization action.
    ///
    /// The action must:
    ///
    /// - be non-empty;
    /// - contain no whitespace;
    /// - contain no control characters;
    /// - begin with `ai.`;
    /// - contain no empty qualified-name segments.
    ///
    /// No arbitrary maximum length is imposed.
    ///
    /// The canonical PolicyIdentifier performs its own validation.
    pub fn new(
        value: impl Into<String>,
    ) -> DistributedResult<Self> {
        let value = value.into();

        let identifier = PolicyIdentifier::new(value)?;

        let value = identifier.as_str();

        if !value.starts_with("ai.") {
            return Err(DistributedError::invalid_input(
                "AI authorization actions must use the ai. namespace",
            ));
        }

        if value.split('.').any(str::is_empty) {
            return Err(DistributedError::invalid_input(
                "AI authorization action contains an empty namespace segment",
            ));
        }

        Ok(Self {
            identifier,
        })
    }

    /// Return the canonical policy identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.identifier.as_str()
    }

    /// Return the canonical policy identifier.
    #[must_use]
    pub fn identifier(&self) -> &PolicyIdentifier {
        &self.identifier
    }

    /// Consume this action and return its string representation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.identifier.into_string()
    }
}

impl fmt::Display for AiAuthorizationAction {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        self.identifier.fmt(formatter)
    }
}

// ============================================================================
// AI AUTHORIZATION REQUEST
// ============================================================================

/// A request to authorize one AI operation against one AI resource.
///
/// The resource is represented using the canonical AI identity reference.
///
/// The authenticated principal is deliberately not stored in this
/// structure. It must be obtained from the authenticated session by
/// the distributed authentication service.
///
/// This prevents a caller from manufacturing an authenticated AI
/// principal by supplying an identity reference.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub struct AiAuthorizationRequest {
    action: AiAuthorizationAction,
    resource: AiIdentityRef,
}

impl AiAuthorizationRequest {
    /// Construct an AI authorization request.
    ///
    /// The request identifies:
    ///
    /// - the AI operation being requested;
    /// - the AI entity against which the operation is requested.
    ///
    /// It does not establish that the requester is authenticated or
    /// authorized.
    pub fn new(
        action: AiAuthorizationAction,
        resource: AiIdentityRef,
    ) -> Self {
        Self {
            action,
            resource,
        }
    }

    /// Return the requested AI action.
    #[must_use]
    pub fn action(&self) -> &AiAuthorizationAction {
        &self.action
    }

    /// Return the target AI resource identity.
    #[must_use]
    pub fn resource(&self) -> &AiIdentityRef {
        &self.resource
    }

    /// Convert the AI resource reference into an opaque policy resource
    /// identifier.
    ///
    /// The representation preserves:
    ///
    /// - the AI resource category;
    /// - the authority namespace;
    /// - the authority's UTF-8 byte length;
    /// - the canonical local AI identifier.
    ///
    /// The authority length makes the representation unambiguous even
    /// when the authority itself contains separators.
    ///
    /// This identifier is a lookup reference, not proof of ownership.
    fn policy_resource(&self) -> DistributedResult<PolicyIdentifier> {
        let authority = self.resource.authority().as_str();

        let category = category_name(
            self.resource.category(),
        );

        let identifier = format!(
            "ai.resource.v1.{category}.{}:{authority}.{}",
            authority.len(),
            self.resource.local_id(),
        );

        PolicyIdentifier::new(identifier)
    }
}

// ============================================================================
// AI AUTHORIZATION SERVICE
// ============================================================================

/// AI authorization integration service.
///
/// This service delegates authentication, trusted context construction,
/// policy evaluation, policy obligations, and enforcement to the existing
/// distributed security infrastructure.
///
/// It does not contain a second policy engine.
///
/// It does not maintain its own session store.
///
/// It does not cache authorization decisions.
///
/// It does not allocate or persist authorization permits.
pub struct AiAuthorizationService {
    distributed: Arc<DistributedAuthorizationService>,
}

impl AiAuthorizationService {
    /// Construct an AI authorization service using the canonical
    /// distributed authorization service.
    ///
    /// The supplied service must already be configured with:
    ///
    /// - the authoritative authentication service;
    /// - an appropriate session store;
    /// - the canonical policy enforcer;
    /// - a trusted authorization context provider;
    /// - the required policy engines and obligation handlers.
    ///
    /// This constructor does not create an allow-all configuration.
    pub fn new(
        distributed: Arc<DistributedAuthorizationService>,
    ) -> Self {
        Self {
            distributed,
        }
    }

    /// Authorize and execute one protected AI operation.
    ///
    /// The operation closure is invoked only after the distributed
    /// authorization service has:
    ///
    /// 1. validated the session against the authoritative session store;
    /// 2. obtained the authenticated subject;
    /// 3. constructed trusted policy context;
    /// 4. constructed the canonical policy request;
    /// 5. obtained an explicit policy Permit;
    /// 6. successfully fulfilled all policy obligations.
    ///
    /// The policy permit is borrowed by the closure and is not returned
    /// by this method.
    ///
    /// The operation must not treat the existence of a session as
    /// sufficient authorization.
    ///
    /// # Failure behavior
    ///
    /// If authentication, context construction, policy evaluation,
    /// policy enforcement, or obligation fulfillment fails, the
    /// operation closure is never invoked.
    ///
    /// # Side effects
    ///
    /// This method does not make an arbitrary operation transactional.
    ///
    /// The caller remains responsible for:
    ///
    /// - operation idempotency;
    /// - durable completion records;
    /// - cancellation;
    /// - partial failure handling;
    /// - outcome reconciliation;
    /// - resource cleanup.
    ///
    /// For distributed operations, the authorization boundary must
    /// also be enforced at the actual remote protected-operation
    /// boundary.
    pub fn authorize_and_execute<T, F>(
        &self,
        session_id: &SessionId,
        request: &AiAuthorizationRequest,
        operation: F,
    ) -> DistributedResult<T>
    where
        F: FnOnce(&PolicyPermit) -> DistributedResult<T>,
    {
        let resource = request.policy_resource()?;

        self.distributed.authorize_and_execute(
            session_id,
            request.action.as_str().to_owned(),
            resource.into_string(),
            operation,
        )
    }

    /// Authorize and execute an AI operation without exposing the
    /// policy permit to the operation closure.
    ///
    /// This convenience method is appropriate when the protected
    /// operation does not require permit metadata.
    ///
    /// The authorization service still performs the complete policy
    /// enforcement process.
    pub fn authorize_and_run<T, F>(
        &self,
        session_id: &SessionId,
        request: &AiAuthorizationRequest,
        operation: F,
    ) -> DistributedResult<T>
    where
        F: FnOnce() -> DistributedResult<T>,
    {
        self.authorize_and_execute(
            session_id,
            request,
            |_permit| operation(),
        )
    }
}

impl fmt::Debug for AiAuthorizationService {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("AiAuthorizationService")
            .field(
                "distributed_authorization",
                &"[CANONICAL DISTRIBUTED AUTHORIZATION SERVICE]",
            )
            .finish()
    }
}

// ============================================================================
// AI RESOURCE CATEGORY ADAPTATION
// ============================================================================

/// Convert a canonical AI identity category into its stable policy
/// namespace segment.
///
/// This is an adapter for the existing identity categories.
///
/// It does not define or allocate new identity categories.
const fn category_name(
    category: AiIdentityCategory,
) -> &'static str {
    match category {
        AiIdentityCategory::Model => "model",
        AiIdentityCategory::Agent => "agent",
        AiIdentityCategory::Dataset => "dataset",
        AiIdentityCategory::TrainingRun => "training_run",
        AiIdentityCategory::Pipeline => "pipeline",
        AiIdentityCategory::Artifact => "artifact",
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ai::ids::{
        AiAgentId,
        AiArtifactId,
        AiDatasetId,
        AiModelId,
        AiPipelineId,
        AiTrainingRunId,
    };

    use crate::ai::security::identity::{
        AiEntityRef,
        IdentityAuthority,
    };

    fn authority(
        value: &str,
    ) -> IdentityAuthority {
        IdentityAuthority::new(value)
            .expect("test authority must be valid")
    }

    fn model_identity(
        authority_name: &str,
        id: u128,
    ) -> AiIdentityRef {
        AiIdentityRef::new(
            authority(authority_name),
            AiEntityRef::Model(
                AiModelId::new(id)
                    .expect("test model ID must be valid"),
            ),
        )
    }

    #[test]
    fn accepts_ai_scoped_actions() {
        let action = AiAuthorizationAction::new(
            "ai.model.infer",
        )
        .expect("AI action must be valid");

        assert_eq!(
            action.as_str(),
            "ai.model.infer",
        );
    }

    #[test]
    fn accepts_open_ended_ai_action_namespaces() {
        let action = AiAuthorizationAction::new(
            "ai.future.computational_domain.execute",
        )
        .expect("future AI action must be accepted");

        assert_eq!(
            action.as_str(),
            "ai.future.computational_domain.execute",
        );
    }

    #[test]
    fn rejects_actions_outside_the_ai_namespace() {
        assert!(
            AiAuthorizationAction::new(
                "distributed.execute",
            )
            .is_err()
        );

        assert!(
            AiAuthorizationAction::new(
                "classical.memory.read",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_empty_actions() {
        assert!(
            AiAuthorizationAction::new("")
                .is_err()
        );
    }

    #[test]
    fn rejects_empty_action_namespace_segments() {
        assert!(
            AiAuthorizationAction::new(
                "ai..model.infer",
            )
            .is_err()
        );

        assert!(
            AiAuthorizationAction::new(
                "ai.model.",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_action_whitespace() {
        assert!(
            AiAuthorizationAction::new(
                "ai.model infer",
            )
            .is_err()
        );

        assert!(
            AiAuthorizationAction::new(
                " ai.model.infer",
            )
            .is_err()
        );
    }

    #[test]
    fn constructs_model_authorization_requests() {
        let action = AiAuthorizationAction::new(
            "ai.model.infer",
        )
        .expect("valid action");

        let resource = model_identity(
            "zamani::models",
            42,
        );

        let request = AiAuthorizationRequest::new(
            action,
            resource,
        );

        assert_eq!(
            request.action().as_str(),
            "ai.model.infer",
        );

        assert_eq!(
            request.resource().local_id(),
            42,
        );
    }

    #[test]
    fn policy_resource_preserves_authority_and_category() {
        let request = AiAuthorizationRequest::new(
            AiAuthorizationAction::new(
                "ai.model.infer",
            )
            .expect("valid action"),
            model_identity(
                "organization::research",
                42,
            ),
        );

        let resource = request
            .policy_resource()
            .expect("policy resource must be valid");

        assert!(
            resource.as_str().starts_with(
                "ai.resource.v1.model."
            )
        );

        assert!(
            resource.as_str().contains(
                "organization::research"
            )
        );

        assert!(
            resource.as_str().ends_with(".42")
        );
    }

    #[test]
    fn different_authorities_produce_different_policy_resources() {
        let first = AiAuthorizationRequest::new(
            AiAuthorizationAction::new(
                "ai.model.infer",
            )
            .expect("valid action"),
            model_identity(
                "organization::one",
                7,
            ),
        );

        let second = AiAuthorizationRequest::new(
            AiAuthorizationAction::new(
                "ai.model.infer",
            )
            .expect("valid action"),
            model_identity(
                "organization::two",
                7,
            ),
        );

        assert_ne!(
            first.policy_resource().unwrap(),
            second.policy_resource().unwrap(),
        );
    }

    #[test]
    fn different_entity_categories_produce_different_resources() {
        let authority = authority("zamani");

        let model = AiIdentityRef::new(
            authority.clone(),
            AiEntityRef::Model(
                AiModelId::new(9).unwrap(),
            ),
        );

        let dataset = AiIdentityRef::new(
            authority,
            AiEntityRef::Dataset(
                AiDatasetId::new(9).unwrap(),
            ),
        );

        let model_request = AiAuthorizationRequest::new(
            AiAuthorizationAction::new(
                "ai.model.read",
            )
            .unwrap(),
            model,
        );

        let dataset_request = AiAuthorizationRequest::new(
            AiAuthorizationAction::new(
                "ai.dataset.read",
            )
            .unwrap(),
            dataset,
        );

        assert_ne!(
            model_request.policy_resource().unwrap(),
            dataset_request.policy_resource().unwrap(),
        );
    }

    #[test]
    fn authority_with_whitespace_fails_policy_identifier_validation() {
        let request = AiAuthorizationRequest::new(
            AiAuthorizationAction::new(
                "ai.model.read",
            )
            .unwrap(),
            model_identity(
                "organization research",
                10,
            ),
        );

        assert!(
            request.policy_resource().is_err()
        );
    }

    #[test]
    fn all_existing_ai_identity_categories_are_supported() {
        let authority = authority("zamani");

        let resources = [
            AiEntityRef::Model(
                AiModelId::new(1).unwrap(),
            ),
            AiEntityRef::Agent(
                AiAgentId::new(2).unwrap(),
            ),
            AiEntityRef::Dataset(
                AiDatasetId::new(3).unwrap(),
            ),
            AiEntityRef::TrainingRun(
                AiTrainingRunId::new(4).unwrap(),
            ),
            AiEntityRef::Pipeline(
                AiPipelineId::new(5).unwrap(),
            ),
            AiEntityRef::Artifact(
                AiArtifactId::new(6).unwrap(),
            ),
        ];

        for resource in resources {
            let request = AiAuthorizationRequest::new(
                AiAuthorizationAction::new(
                    "ai.resource.read",
                )
                .unwrap(),
                AiIdentityRef::new(
                    authority.clone(),
                    resource,
                ),
            );

            assert!(
                request.policy_resource().is_ok()
            );
        }
    }

    #[test]
    fn request_does_not_contain_an_authenticated_principal() {
        // The request intentionally contains only:
        //
        // - an AI action;
        // - an AI resource identity.
        //
        // Authentication is obtained separately from SessionId through
        // the canonical distributed authentication service.
        //
        // This test documents the architectural boundary.
        let request = AiAuthorizationRequest::new(
            AiAuthorizationAction::new(
                "ai.model.infer",
            )
            .unwrap(),
            model_identity(
                "zamani",
                1,
            ),
        );

        assert_eq!(
            request.action().as_str(),
            "ai.model.infer",
        );
    }
}