
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Agent Request Boundary
//!
//! Path: src/toolchain/agents/requests.rs
//! Rust edition: 2021.
//! Minimum supported Rust: 1.97.
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Defines the validated request boundary used by AI agents and other
//! context-aware toolchain clients.
//!
//! This module deliberately reuses the canonical public toolchain
//! RequestEnvelope and AgentContext. It does not introduce a second
//! build, test, package, deployment, target-selection, or project-request
//! model.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - The agent-facing request envelope.
//! - Composition of canonical API requests and validated agent context.
//! - Cross-validation of request and context identity.
//! - Cross-validation of project scope.
//! - Construction of context-free agent request envelopes.
//! - Access to the canonical operation and request metadata.
//!
//! This module does not own:
//!
//! - Authentication or identity verification.
//! - Authorization or permission evaluation.
//! - Agent reasoning or planning.
//! - Project or workspace discovery.
//! - Manifest parsing.
//! - Semantic analysis.
//! - Requirement or capability evaluation.
//! - Target registration or feasibility assessment.
//! - Dependency resolution.
//! - Build planning or compilation.
//! - Backend selection or code generation.
//! - Package creation or registry access.
//! - Hardware discovery or allocation.
//! - Runtime execution or deployment.
//! - Audit persistence.
//! - Response or result construction.
//!
//! Those responsibilities remain with their canonical subsystems.
//!
//! ## Security boundary
//!
//! A request's ClientIdentity is descriptive metadata. It is not proof
//! of authentication and must never be used to establish permissions.
//!
//! AgentContext is descriptive, provenance-aware context. It is not
//! authorization and does not establish that a fact is true merely
//! because an agent supplied it.
//!
//! Every request must pass the same API validation, version compatibility,
//! authentication, authorization, and service validation as a human
//! developer's request.
//!
//! Deserialization does not replace validation. Call validate() before
//! dispatch, including for requests reconstructed from serialized data.
//!
//! ## POCO-REAF
//!
//! A single canonical ToolchainOperation can express intent for any
//! computational domain supported by the toolchain API.
//!
//! This module does not enumerate processors, accelerators, quantum
//! devices, HDL targets, vendors, providers, or computing domains.
//!
//! Target feasibility is determined by the compiler, capability,
//! backend, hardware, and runtime authorities.
//!
//! ## Scalability
//!
//! This module introduces no arbitrary maximum on:
//!
//! - Agent requests.
//! - Request metadata.
//! - Context facts.
//! - Evidence records.
//! - Projects.
//! - Workspaces.
//! - Targets.
//! - Providers.
//! - Computing domains.
//!
//! Context collection budgets, when configured, remain the responsibility
//! of AgentContextBuilder and its ContextLimits contract.
//!
//! Finite memory, address space, transport capacity, and execution time
//! remain physical and operational constraints.

use serde::{Deserialize, Serialize};

use crate::toolchain::agents::context::{
    AgentContext,
    AgentContextBuilder,
};

use crate::toolchain::api::request::{
    BuildRequest,
    CheckRequest,
    DeployRequest,
    PackageRequest,
    ProjectScope,
    PublishRequest,
    RequestEnvelope,
    RunRequest,
    TestRequest,
    ToolchainOperation,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

/// Stable component identifier.
pub const COMPONENT: &str = "toolchain.agents.requests";

/// Stable validation operation identifier.
pub const VALIDATE_REQUEST_OPERATION: &str =
    "toolchain.agents.requests.validate";

/// Agent request contract version.
///
/// This identifies this module's composition contract. It is deliberately
/// distinct from the canonical toolchain API version in RequestContext.
pub const AGENT_REQUEST_CONTRACT_VERSION: u32 = 1;

// ============================================================================
// AGENT REQUEST
// ============================================================================

/// A validated agent-facing wrapper around a canonical toolchain request.
///
/// The contained RequestEnvelope is the authoritative operation request.
/// The contained AgentContext supplies descriptive, provenance-linked
/// context for that same request.
///
/// This type does not create a separate operation model. Human developers,
/// CLI clients, IDEs, automation and agents continue to use the same
/// ToolchainOperation and toolchain services.
///
/// Fields are private so callers must use the validated constructor or
/// deserialize and explicitly validate the resulting value.
///
/// A successful construction proves only structural consistency between
/// the request and context. It does not authenticate the caller, authorize
/// the operation, or establish that the operation is feasible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentRequest {
    /// Canonical public toolchain request.
    envelope: RequestEnvelope,

    /// Validated context associated with the canonical request.
    context: AgentContext,
}

impl AgentRequest {
    /// Construct an agent request from a canonical API request and context.
    ///
    /// The request and context must identify the same RequestContext.
    ///
    /// When the operation identifies a project scope, any project or build
    /// scope included in the context must agree with that operation.
    ///
    /// This constructor performs structural validation only. Authentication,
    /// authorization, semantic validation, capability evaluation, target
    /// feasibility, and execution remain separate mandatory stages.
    pub fn new(
        envelope: RequestEnvelope,
        context: AgentContext,
    ) -> ToolchainResult<Self> {
        let request = Self {
            envelope,
            context,
        };

        request.validate()?;

        Ok(request)
    }

    /// Construct an agent request with an initially empty context.
    ///
    /// The context is created from the canonical RequestContext contained
    /// in the envelope. Additional context can be collected separately
    /// through AgentContextBuilder and supplied to AgentRequest::new.
    ///
    /// An empty context is not a claim that all information needed for the
    /// requested operation is available.
    pub fn from_envelope(
        envelope: RequestEnvelope,
    ) -> ToolchainResult<Self> {
        envelope.validate().map_err(|error| {
            validation_error(format!(
                "canonical API request is invalid: {error}"
            ))
        })?;

        let context = AgentContextBuilder::new(
            envelope.context.clone(),
        )?
        .finish()?;

        Self::new(envelope, context)
    }

    /// Validate the complete agent request.
    ///
    /// This method must be called before dispatch, including after
    /// deserialization.
    ///
    /// Validation checks:
    ///
    /// - The canonical request envelope.
    /// - The agent context and its evidence references.
    /// - Exact equality of the originating RequestContext.
    /// - Consistency between operation project scope and context scope.
    ///
    /// It does not perform authentication, authorization, filesystem
    /// access, target discovery, or feasibility evaluation.
    pub fn validate(&self) -> ToolchainResult<()> {
        self.envelope.validate().map_err(|error| {
            validation_error(format!(
                "canonical API request validation failed: {error}"
            ))
        })?;

        self.context.validate()?;

        if self.context.request() != &self.envelope.context {
            return Err(validation_error(
                "agent context does not belong to the canonical request context",
            ));
        }

        self.validate_project_scope()?;

        Ok(())
    }

    /// Return the canonical request envelope.
    ///
    /// The returned reference exposes the same request contract used by
    /// non-agent toolchain clients.
    #[must_use]
    pub fn envelope(&self) -> &RequestEnvelope {
        &self.envelope
    }

    /// Return the validated agent context.
    #[must_use]
    pub fn context(&self) -> &AgentContext {
        &self.context
    }

    /// Return the canonical operation.
    #[must_use]
    pub fn operation(&self) -> &ToolchainOperation {
        &self.envelope.operation
    }

    /// Return the stable canonical operation name.
    #[must_use]
    pub fn operation_name(&self) -> &'static str {
        self.envelope.operation_name()
    }

    /// Return the canonical request identifier.
    #[must_use]
    pub fn request_id(&self) -> &str {
        &self.envelope.context.request_id
    }

    /// Return the optional request correlation identifier.
    #[must_use]
    pub fn correlation_id(&self) -> Option<&str> {
        self.envelope.context.correlation_id.as_deref()
    }

    /// Return whether the canonical operation may mutate external state.
    ///
    /// This is descriptive operation metadata, not an authorization
    /// decision. The operation must still pass the canonical permission
    /// and security boundaries.
    #[must_use]
    pub fn may_mutate_external_state(&self) -> bool {
        self.envelope.may_mutate_external_state()
    }

    /// Consume this wrapper and return the canonical API request.
    ///
    /// The context is not transferred because it is agent-facing
    /// descriptive information rather than part of the canonical
    /// operation contract.
    ///
    /// Callers that need the context after conversion should obtain it
    /// through context() before consuming the AgentRequest.
    #[must_use]
    pub fn into_envelope(self) -> RequestEnvelope {
        self.envelope
    }

    /// Validate consistency between operation project scope and context.
    fn validate_project_scope(&self) -> ToolchainResult<()> {
        let Some(request_scope) = operation_project_scope(
            &self.envelope.operation,
        ) else {
            // Operations without a project scope, such as workspace
            // discovery, package installation and package publication,
            // are validated by their own canonical request contracts.
            return Ok(());
        };

        if let Some(project_context) = self.context.project() {
            if project_context.scope() != request_scope {
                return Err(validation_error(
                    "agent project context does not match the operation project scope",
                ));
            }
        }

        if let Some(build_context) = self.context.build() {
            if &build_context.request().scope != request_scope {
                return Err(validation_error(
                    "agent build context does not match the operation project scope",
                ));
            }
        }

        Ok(())
    }
}

// ============================================================================
// CANONICAL PROJECT-SCOPE EXTRACTION
// ============================================================================

/// Return the project scope carried by an operation, when applicable.
///
/// This is a read-only adapter over the canonical API request types.
/// It does not resolve project identities or discover project directories.
fn operation_project_scope(
    operation: &ToolchainOperation,
) -> Option<&ProjectScope> {
    match operation {
        ToolchainOperation::Build(request) => {
            Some(&request.scope)
        }

        ToolchainOperation::Check(request) => {
            Some(&request.scope)
        }

        ToolchainOperation::Run(request) => {
            Some(&request.build.scope)
        }

        ToolchainOperation::Test(request) => {
            Some(&request.scope)
        }

        ToolchainOperation::Package(request) => {
            Some(&request.scope)
        }

        ToolchainOperation::Deploy(request) => {
            Some(&request.scope)
        }

        ToolchainOperation::DiscoverWorkspace(_)
        | ToolchainOperation::Install(_)
        | ToolchainOperation::Publish(_) => None,
    }
}

// Keep the canonical operation request types visible to this module's
// compile-time contract checks. These assertions also document which
// canonical request types carry a project scope.
const _: fn(&BuildRequest) -> &ProjectScope =
    |request| &request.scope;

const _: fn(&CheckRequest) -> &ProjectScope =
    |request| &request.scope;

const _: fn(&RunRequest) -> &ProjectScope =
    |request| &request.build.scope;

const _: fn(&TestRequest) -> &ProjectScope =
    |request| &request.scope;

const _: fn(&PackageRequest) -> &ProjectScope =
    |request| &request.scope;

const _: fn(&DeployRequest) -> &ProjectScope =
    |request| &request.scope;

const _: fn(&PublishRequest) -> &crate::toolchain::ids::RegistryId =
    |request| &request.registry;

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

/// Convert a request-boundary validation failure into the shared toolchain
/// error type.
///
/// The original explanation is retained without introducing a second
/// error taxonomy for agent requests.
fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation(VALIDATE_REQUEST_OPERATION)
        .with_context(COMPONENT)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use crate::toolchain::agents::context::{
        AgentContextBuilder,
    };

    use crate::toolchain::api::request::{
        ClientIdentity,
        RequestContext,
        WorkspaceDiscoveryRequest,
    };

    fn request_context(
        request_id: &str,
        client_kind: &str,
    ) -> RequestContext {
        let client = ClientIdentity::new(
            client_kind,
            "agent-request-tests",
        )
        .expect("valid client identity");

        RequestContext::new(
            request_id,
            client,
        )
        .expect("valid request context")
    }

    fn discovery_envelope(
        context: RequestContext,
    ) -> RequestEnvelope {
        let operation = ToolchainOperation::DiscoverWorkspace(
            WorkspaceDiscoveryRequest::new(
                PathBuf::from("."),
                true,
            )
            .expect("valid workspace discovery request"),
        );

        RequestEnvelope::new(
            context,
            operation,
        )
        .expect("valid request envelope")
    }

    fn empty_context(
        request: RequestContext,
    ) -> AgentContext {
        AgentContextBuilder::new(request)
            .expect("valid context builder")
            .finish()
            .expect("valid empty agent context")
    }

    fn valid_agent_request() -> AgentRequest {
        let request = request_context(
            "agent-request-test-1",
            "ai-agent",
        );

        let envelope = discovery_envelope(
            request.clone(),
        );

        let context = empty_context(request);

        AgentRequest::new(
            envelope,
            context,
        )
        .expect("valid agent request")
    }

    #[test]
    fn constructs_request_from_canonical_envelope() {
        let request = valid_agent_request();

        assert_eq!(
            request.operation_name(),
            "workspace.discover",
        );

        assert_eq!(
            request.request_id(),
            "agent-request-test-1",
        );

        assert!(
            !request.may_mutate_external_state(),
        );

        assert!(request.validate().is_ok());
    }

    #[test]
    fn creates_empty_context_from_canonical_envelope() {
        let context = request_context(
            "agent-request-test-2",
            "ai-agent",
        );

        let envelope = discovery_envelope(context);

        let request = AgentRequest::from_envelope(envelope)
            .expect("valid context-free agent request");

        assert!(request.context().facts().is_empty());
        assert!(request.context().evidence().is_empty());
        assert!(request.validate().is_ok());
    }

    #[test]
    fn rejects_context_from_a_different_request() {
        let envelope = discovery_envelope(
            request_context(
                "request-a",
                "ai-agent",
            ),
        );

        let context = empty_context(
            request_context(
                "request-b",
                "ai-agent",
            ),
        );

        assert!(
            AgentRequest::new(envelope, context).is_err()
        );
    }

    #[test]
    fn human_and_agent_clients_use_the_same_request_contract() {
        let human_envelope = discovery_envelope(
            request_context(
                "human-request",
                "cli",
            ),
        );

        let human_request = AgentRequest::from_envelope(
            human_envelope,
        )
        .expect("human client request uses canonical API");

        let agent_envelope = discovery_envelope(
            request_context(
                "agent-request",
                "ai-agent",
            ),
        );

        let agent_request = AgentRequest::from_envelope(
            agent_envelope,
        )
        .expect("agent client request uses canonical API");

        assert_eq!(
            human_request.operation_name(),
            agent_request.operation_name(),
        );
    }

    #[test]
    fn client_kind_is_not_treated_as_authentication() {
        // This module deliberately accepts the canonical client descriptor
        // without granting any authority based on its kind.
        let envelope = discovery_envelope(
            request_context(
                "untrusted-client-request",
                "third-party-client",
            ),
        );

        let request = AgentRequest::from_envelope(envelope)
            .expect("valid descriptive client identity");

        assert!(request.validate().is_ok());

        // Authentication and authorization are separate mandatory stages.
    }

    #[test]
    fn preserves_canonical_operation_identity() {
        let request = valid_agent_request();

        assert_eq!(
            request.operation().operation_name(),
            request.envelope().operation_name(),
        );
    }

    #[test]
    fn preserves_correlation_metadata() {
        let mut context = request_context(
            "correlated-request",
            "ai-agent",
        );

        context.correlation_id =
            Some("correlation-123".to_owned());

        let envelope = discovery_envelope(context.clone());
        let agent_context = empty_context(context);

        let request = AgentRequest::new(
            envelope,
            agent_context,
        )
        .expect("valid correlated request");

        assert_eq!(
            request.correlation_id(),
            Some("correlation-123"),
        );
    }

    #[test]
    fn request_serialization_round_trips() {
        let original = valid_agent_request();

        let encoded = serde_json::to_string(&original)
            .expect("serialize agent request");

        let decoded: AgentRequest =
            serde_json::from_str(&encoded)
                .expect("deserialize agent request");

        assert_eq!(decoded, original);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn deserialized_request_must_still_be_validated() {
        let original = valid_agent_request();

        let mut value = serde_json::to_value(&original)
            .expect("serialize request");

        value["envelope"]["context"]["request_id"] =
            serde_json::Value::String(String::new());

        let decoded: AgentRequest =
            serde_json::from_value(value)
                .expect("wire representation is structurally deserializable");

        assert!(decoded.validate().is_err());
    }

    #[test]
    fn canonical_envelope_can_be_recovered() {
        let request = valid_agent_request();

        let envelope = request.into_envelope();

        assert_eq!(
            envelope.operation_name(),
            "workspace.discover",
        );

        assert!(
            envelope.validate().is_ok()
        );
    }
}
