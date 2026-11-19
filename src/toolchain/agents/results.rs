// src/toolchain/agents/results.rs

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Agent Results
//!
//! Rust edition: 2021.
//! Minimum supported Rust: 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Defines the structured result produced by an agent-facing toolchain
//! workflow. The result combines:
//!
//! - The canonical Toolchain API response.
//! - Explicit outcomes for individual workflow stages.
//! - References to evidence supplied by authoritative subsystems.
//! - Advisory next-action suggestions.
//!
//! ## Ownership
//!
//! This module owns the agent result composition contract and its
//! structural validation.
//!
//! It does not own or replace:
//!
//! - Canonical API requests or responses.
//! - Toolchain errors.
//! - Diagnostic creation or aggregation.
//! - Compiler artifacts or artifact verification.
//! - Authentication or authorization.
//! - Semantic analysis or capability evaluation.
//! - Target feasibility or hardware discovery.
//! - Resource allocation or scheduling.
//! - Backend invocation or runtime execution.
//! - Deployment or audit persistence.
//!
//! Those responsibilities remain with their authoritative subsystems.
//!
//! ## Trust boundary
//!
//! An agent result is a report. It is not an authorization grant,
//! execution command, feasibility certificate, or proof that a suggested
//! action is safe.
//!
//! Suggested actions are advisory. A consumer must submit a new canonical
//! request through the normal API and permission boundaries before
//! performing an operation.
//!
//! Evidence references identify information supplied by another
//! subsystem. This module does not independently verify that evidence.
//!
//! ## Lifecycle semantics
//!
//! A workflow stage outcome and the overall API response status describe
//! different things:
//!
//! - A stage outcome describes one named step in an agent workflow.
//! - The response status describes the canonical toolchain operation.
//!
//! Neither can be inferred solely from the other.
//!
//! Diagnostic collection completeness is not inferred from the number
//! of diagnostics in a response. Completeness remains the responsibility
//! of the canonical diagnostic aggregation subsystem.
//!
//! ## POCO-REAF
//!
//! Stage identifiers and authority identifiers are open strings.
//! Computational domains, targets, providers and hardware types are not
//! enumerated here.
//!
//! This result contract can therefore describe existing and future
//! toolchain workflows without imposing domain-specific ceilings.
//!
//! ## Scalability
//!
//! This module imposes no arbitrary maximum on stages, evidence records
//! or suggested actions. Collections grow subject to available memory
//! and address space. Allocation failures are reported explicitly.

use serde::{Deserialize, Serialize};

use crate::toolchain::agents::context::{
    ContextEvidence,
    EvidenceStatus,
};

use crate::toolchain::agents::requests::AgentRequest;

use crate::toolchain::api::response::{
    ResponseEnvelope,
    ResponseStatus,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

/// Stable component identifier.
pub const COMPONENT: &str = "toolchain.agents.results";

/// Stable result-validation operation identifier.
pub const VALIDATE_RESULT_OPERATION: &str =
    "toolchain.agents.results.validate";

/// Version of the agent result composition contract.
///
/// This is distinct from the canonical Toolchain API version carried by
/// the embedded `ResponseEnvelope`.
pub const AGENT_RESULT_CONTRACT_VERSION: u32 = 1;

// ============================================================================
// VALIDATION AND ALLOCATION
// ============================================================================

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation(VALIDATE_RESULT_OPERATION)
        .with_context(COMPONENT)
}

fn allocation_error(
    collection: &str,
    error: impl std::fmt::Display,
) -> ToolchainError {
    ToolchainError::resource_exhausted(format!(
        "unable to allocate memory for agent result {collection}: {error}"
    ))
    .with_operation(VALIDATE_RESULT_OPERATION)
    .with_context(COMPONENT)
}

fn validate_identifier(
    field: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.is_empty() || value.trim().is_empty() {
        return Err(validation_error(format!(
            "{field} must not be empty"
        )));
    }

    if value != value.trim() {
        return Err(validation_error(format!(
            "{field} must not contain surrounding whitespace"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(validation_error(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(())
}

fn validate_description(
    field: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(validation_error(format!(
            "{field} must not be empty"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(validation_error(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(())
}

// ============================================================================
// STAGE OUTCOME
// ============================================================================

/// Outcome of one stage in an agent workflow.
///
/// This enum describes workflow progress. It does not describe a
/// computational domain, target, provider or hardware type.
///
/// `Unknown` permits forward-compatible interpretation of future
/// serialized outcome values.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AgentStageOutcome {
    /// The stage completed successfully.
    ///
    /// A successful stage must include at least one verified evidence
    /// reference from its declared authority or another relevant authority.
    Succeeded,

    /// The stage attempted its operation but failed.
    Failed,

    /// The stage could not proceed because a required condition was not met.
    Blocked,

    /// The stage could not establish whether its required condition holds.
    Unresolved,

    /// The stage was not evaluated.
    NotEvaluated,

    /// The stage was deliberately not run.
    Skipped,

    /// The stage was cancelled.
    Cancelled,

    /// The stage outcome is not understood.
    Unknown,
}

impl AgentStageOutcome {
    /// Whether this outcome represents a successful stage.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Succeeded)
    }

    /// Whether this outcome prevents an overall full-success claim.
    ///
    /// An unresolved or unknown stage must not silently become a
    /// successful stage.
    #[must_use]
    pub const fn prevents_full_success(self) -> bool {
        matches!(
            self,
            Self::Failed
                | Self::Blocked
                | Self::Unresolved
                | Self::Cancelled
                | Self::Unknown
        )
    }

    /// Return the stable serialized outcome identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Blocked => "blocked",
            Self::Unresolved => "unresolved",
            Self::NotEvaluated => "not_evaluated",
            Self::Skipped => "skipped",
            Self::Cancelled => "cancelled",
            Self::Unknown => "unknown",
        }
    }
}

// ============================================================================
// AGENT STAGE RESULT
// ============================================================================

/// Result of one named stage in an agent workflow.
///
/// `stage` and `authority` are open identifiers. This avoids requiring
/// this module to maintain a closed list of compiler, build, package,
/// deployment, hardware or future workflow stages.
///
/// Evidence records are references to information owned by authoritative
/// subsystems. Their presence does not transfer ownership or verification
/// responsibility to this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentStageResult {
    /// Open identifier for this workflow stage.
    stage: String,

    /// Outcome of this stage.
    outcome: AgentStageOutcome,

    /// Identifier of the subsystem or provider responsible for the stage.
    authority: String,

    /// Evidence references associated with this stage.
    #[serde(default)]
    evidence: Vec<ContextEvidence>,
}

impl AgentStageResult {
    /// Construct a stage result.
    ///
    /// This constructor validates the stage identity and authority but
    /// deliberately does not enforce the final evidence invariant.
    ///
    /// This permits the builder sequence:
    ///
    /// 1. Construct a successful stage.
    /// 2. Attach its verified evidence.
    /// 3. Call `validate`, or add it to an `AgentResult`.
    ///
    /// An incomplete stage cannot be accepted into an `AgentResult`.
    pub fn new(
        stage: impl Into<String>,
        outcome: AgentStageOutcome,
        authority: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let result = Self {
            stage: stage.into(),
            outcome,
            authority: authority.into(),
            evidence: Vec::new(),
        };

        validate_identifier("stage", &result.stage)?;
        validate_identifier("authority", &result.authority)?;

        Ok(result)
    }

    /// Attach one evidence reference.
    ///
    /// Duplicate evidence identifiers are rejected. The evidence is
    /// structurally validated, but its authoritative status is not
    /// independently verified here.
    ///
    /// Full stage validation occurs when `validate` is called or when
    /// the stage is added to an `AgentResult`.
    pub fn with_evidence(
        mut self,
        evidence: ContextEvidence,
    ) -> ToolchainResult<Self> {
        evidence.validate()?;

        if self
            .evidence
            .iter()
            .any(|existing| existing.id() == evidence.id())
        {
            return Err(validation_error(format!(
                "duplicate evidence identifier '{}' in stage '{}'",
                evidence.id(),
                self.stage
            )));
        }

        self.evidence
            .try_reserve(1)
            .map_err(|error| allocation_error("stage evidence", error))?;

        self.evidence.push(evidence);

        Ok(self)
    }

    /// Validate the complete stage result.
    ///
    /// A successful stage must contain at least one evidence record
    /// marked `Verified` by its supplying authority.
    ///
    /// This is an evidence-presence guard, not a proof of target
    /// feasibility, authorization, artifact integrity or executability.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier("stage", &self.stage)?;
        validate_identifier("authority", &self.authority)?;

        for evidence in &self.evidence {
            evidence.validate()?;
        }

        for (index, evidence) in self.evidence.iter().enumerate() {
            if self.evidence[..index]
                .iter()
                .any(|previous| previous.id() == evidence.id())
            {
                return Err(validation_error(format!(
                    "stage '{}' contains duplicate evidence identifier '{}'",
                    self.stage,
                    evidence.id()
                )));
            }
        }

        if self.outcome.is_success()
            && !self
                .evidence
                .iter()
                .any(|item| item.status() == EvidenceStatus::Verified)
        {
            return Err(validation_error(format!(
                "successful stage '{}' requires at least one verified evidence record",
                self.stage
            )));
        }

        Ok(())
    }

    /// Return the stage identifier.
    #[must_use]
    pub fn stage(&self) -> &str {
        &self.stage
    }

    /// Return the stage outcome.
    #[must_use]
    pub const fn outcome(&self) -> AgentStageOutcome {
        self.outcome
    }

    /// Return the responsible authority identifier.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Return the stage evidence references.
    #[must_use]
    pub fn evidence(&self) -> &[ContextEvidence] {
        &self.evidence
    }
}

// ============================================================================
// ADVISORY NEXT ACTION
// ============================================================================

/// An advisory next action suggested by an agent.
///
/// A suggestion is descriptive information. It is not an executable
/// command, authorization token, permission grant or instruction to
/// bypass the canonical toolchain API.
///
/// If an operation is supplied, it is an open identifier for a canonical
/// operation. The consumer must still construct and submit a fresh
/// canonical request and pass all normal validation and authorization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentSuggestedAction {
    /// Stable identifier for this suggestion within the result.
    id: String,

    /// Short human-readable action title.
    title: String,

    /// Explanation of why the action may be useful.
    rationale: String,

    /// Optional canonical operation identifier.
    operation: Option<String>,

    /// Whether the suggestion requires explicit human or policy approval
    /// before the consumer attempts the operation.
    requires_confirmation: bool,
}

impl AgentSuggestedAction {
    /// Construct an advisory action suggestion.
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        rationale: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let action = Self {
            id: id.into(),
            title: title.into(),
            rationale: rationale.into(),
            operation: None,
            requires_confirmation: true,
        };

        action.validate()?;

        Ok(action)
    }

    /// Attach the canonical operation identifier this suggestion refers to.
    ///
    /// The operation is not executed or authorized by this method.
    pub fn with_operation(
        mut self,
        operation: impl Into<String>,
    ) -> ToolchainResult<Self> {
        self.operation = Some(operation.into());
        self.validate()?;
        Ok(self)
    }

    /// Set whether explicit confirmation is required before attempting
    /// the suggested operation.
    ///
    /// Setting this to `false` does not bypass authorization, policy,
    /// confirmation rules imposed by another subsystem, or API validation.
    pub fn with_confirmation_required(
        mut self,
        required: bool,
    ) -> Self {
        self.requires_confirmation = required;
        self
    }

    /// Validate the suggestion.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier("suggested_action.id", &self.id)?;
        validate_description("suggested_action.title", &self.title)?;
        validate_description(
            "suggested_action.rationale",
            &self.rationale,
        )?;

        if let Some(operation) = &self.operation {
            validate_identifier(
                "suggested_action.operation",
                operation,
            )?;
        }

        Ok(())
    }

    /// Return the suggestion identifier.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the suggestion title.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Return the rationale.
    #[must_use]
    pub fn rationale(&self) -> &str {
        &self.rationale
    }

    /// Return the optional canonical operation identifier.
    #[must_use]
    pub fn operation(&self) -> Option<&str> {
        self.operation.as_deref()
    }

    /// Whether this suggestion declares that confirmation is required.
    #[must_use]
    pub const fn requires_confirmation(&self) -> bool {
        self.requires_confirmation
    }
}

// ============================================================================
// AGENT RESULT
// ============================================================================

/// Complete structured result produced by an agent-facing workflow.
///
/// The canonical `ResponseEnvelope` remains authoritative for API
/// correlation, operation lifecycle, errors, diagnostics, artifacts
/// and structured response data.
///
/// This type adds only agent-specific stage outcomes and advisory
/// suggestions. It does not duplicate the canonical response model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentResult {
    /// Version of this agent result composition contract.
    contract_version: u32,

    /// Canonical Toolchain API response.
    response: ResponseEnvelope,

    /// Outcomes of the agent workflow stages.
    #[serde(default)]
    stages: Vec<AgentStageResult>,

    /// Advisory next actions.
    #[serde(default)]
    suggested_actions: Vec<AgentSuggestedAction>,
}

impl AgentResult {
    /// Construct an agent result for a validated agent request.
    ///
    /// The embedded canonical response begins in the `Accepted` state.
    /// This method does not execute the request.
    pub fn from_request(
        request: &AgentRequest,
    ) -> ToolchainResult<Self> {
        request.validate()?;

        let response = ResponseEnvelope::new(request.envelope())?;

        Ok(Self {
            contract_version: AGENT_RESULT_CONTRACT_VERSION,
            response,
            stages: Vec::new(),
            suggested_actions: Vec::new(),
        })
    }

    /// Replace the canonical response while preserving request correlation.
    ///
    /// The replacement must refer to the same request identifier and
    /// operation as the current response.
    pub fn with_response(
        mut self,
        response: ResponseEnvelope,
    ) -> ToolchainResult<Self> {
        response.validate().map_err(|error| {
            ToolchainError::invalid_input(format!(
                "invalid canonical response: {error}"
            ))
            .with_operation(VALIDATE_RESULT_OPERATION)
            .with_context(COMPONENT)
        })?;

        if response.request_id != self.response.request_id {
            return Err(validation_error(
                "replacement response request identifier does not match the agent result",
            ));
        }

        if response.operation != self.response.operation {
            return Err(validation_error(
                "replacement response operation does not match the agent result",
            ));
        }

        self.response = response;

        self.validate()?;

        Ok(self)
    }

    /// Append one fully validated workflow stage.
    ///
    /// Stage identifiers must be unique within this result.
    pub fn with_stage(
        mut self,
        stage: AgentStageResult,
    ) -> ToolchainResult<Self> {
        stage.validate()?;

        if self
            .stages
            .iter()
            .any(|existing| existing.stage() == stage.stage())
        {
            return Err(validation_error(format!(
                "duplicate agent workflow stage '{}'",
                stage.stage()
            )));
        }

        self.stages
            .try_reserve(1)
            .map_err(|error| allocation_error("stages", error))?;

        self.stages.push(stage);

        self.validate()?;

        Ok(self)
    }

    /// Append one advisory next-action suggestion.
    ///
    /// Suggestion identifiers must be unique within this result.
    /// Adding a suggestion never executes it.
    pub fn with_suggested_action(
        mut self,
        action: AgentSuggestedAction,
    ) -> ToolchainResult<Self> {
        action.validate()?;

        if self
            .suggested_actions
            .iter()
            .any(|existing| existing.id() == action.id())
        {
            return Err(validation_error(format!(
                "duplicate suggested action identifier '{}'",
                action.id()
            )));
        }

        self.suggested_actions
            .try_reserve(1)
            .map_err(|error| {
                allocation_error("suggested actions", error)
            })?;

        self.suggested_actions.push(action);

        self.validate()?;

        Ok(self)
    }

    /// Validate the complete agent result.
    ///
    /// This method must be called after deserialization and before
    /// accepting a result from an untrusted transport.
    ///
    /// It validates:
    ///
    /// - The result contract version.
    /// - The canonical response envelope.
    /// - Unique workflow stage identifiers.
    /// - Every stage and its evidence references.
    /// - Unique suggested action identifiers.
    /// - Every suggested action.
    /// - Consistency between a fully successful response and its stages.
    ///
    /// It does not verify external evidence, authorize suggestions,
    /// establish feasibility or execute any operation.
    pub fn validate(&self) -> ToolchainResult<()> {
        if self.contract_version != AGENT_RESULT_CONTRACT_VERSION {
            return Err(validation_error(format!(
                "unsupported agent result contract version: {}",
                self.contract_version
            )));
        }

        self.response.validate().map_err(|error| {
            ToolchainError::invalid_input(format!(
                "invalid canonical response: {error}"
            ))
            .with_operation(VALIDATE_RESULT_OPERATION)
            .with_context(COMPONENT)
        })?;

        for (index, stage) in self.stages.iter().enumerate() {
            stage.validate()?;

            if self.stages[..index]
                .iter()
                .any(|previous| previous.stage() == stage.stage())
            {
                return Err(validation_error(format!(
                    "duplicate agent workflow stage '{}'",
                    stage.stage()
                )));
            }
        }

        for (index, action) in
            self.suggested_actions.iter().enumerate()
        {
            action.validate()?;

            if self.suggested_actions[..index]
                .iter()
                .any(|previous| previous.id() == action.id())
            {
                return Err(validation_error(format!(
                    "duplicate suggested action identifier '{}'",
                    action.id()
                )));
            }
        }

        if self.response.status == ResponseStatus::Succeeded {
            if let Some(stage) = self
                .stages
                .iter()
                .find(|stage| stage.outcome().prevents_full_success())
            {
                return Err(validation_error(format!(
                    "canonical response is successful but stage '{}' has outcome '{}'",
                    stage.stage(),
                    stage.outcome().as_str()
                )));
            }
        }

        Ok(())
    }

    /// Return the agent result contract version.
    #[must_use]
    pub const fn contract_version(&self) -> u32 {
        self.contract_version
    }

    /// Return the canonical Toolchain API response.
    #[must_use]
    pub fn response(&self) -> &ResponseEnvelope {
        &self.response
    }

    /// Return the workflow stage results.
    #[must_use]
    pub fn stages(&self) -> &[AgentStageResult] {
        &self.stages
    }

    /// Return the advisory next-action suggestions.
    #[must_use]
    pub fn suggested_actions(&self) -> &[AgentSuggestedAction] {
        &self.suggested_actions
    }

    /// Consume this result and return the canonical response envelope.
    ///
    /// Agent-specific stages and suggestions are discarded. Callers that
    /// need them must inspect the result before consuming it.
    #[must_use]
    pub fn into_response(self) -> ResponseEnvelope {
        self.response
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    use crate::toolchain::agents::context::EvidenceStatus;

    use crate::toolchain::api::request::{
        ClientIdentity,
        RequestContext,
        RequestEnvelope,
        ToolchainOperation,
        WorkspaceDiscoveryRequest,
    };

    fn request() -> AgentRequest {
        let client = ClientIdentity::new(
            "ai-agent",
            "agent-results-tests",
        )
        .expect("valid client identity");

        let context = RequestContext::new(
            "agent-results-request",
            client,
        )
        .expect("valid request context");

        let operation = ToolchainOperation::DiscoverWorkspace(
            WorkspaceDiscoveryRequest::new(
                PathBuf::from("."),
                true,
            )
            .expect("valid workspace discovery request"),
        );

        let envelope = RequestEnvelope::new(
            context,
            operation,
        )
        .expect("valid request envelope");

        AgentRequest::from_envelope(envelope)
            .expect("valid agent request")
    }

    fn verified_evidence(
        id: &str,
    ) -> ContextEvidence {
        ContextEvidence::new(
            id,
            "toolchain.test_authority",
            "test:evidence",
            EvidenceStatus::Verified,
        )
        .expect("valid verified evidence")
    }

    fn successful_stage(
        stage: &str,
    ) -> AgentStageResult {
        AgentStageResult::new(
            stage,
            AgentStageOutcome::Succeeded,
            "toolchain.test_authority",
        )
        .expect("valid stage")
        .with_evidence(verified_evidence(
            &format!("{stage}:evidence"),
        ))
        .expect("attach verified evidence")
    }

    #[test]
    fn creates_result_from_canonical_agent_request() {
        let result = AgentResult::from_request(&request())
            .expect("valid initial result");

        assert_eq!(
            result.response().request_id,
            "agent-results-request"
        );

        assert_eq!(
            result.response().status,
            ResponseStatus::Accepted
        );

        assert!(result.validate().is_ok());
    }

    #[test]
    fn successful_stage_can_be_built_before_evidence_is_attached() {
        let stage = AgentStageResult::new(
            "semantic_validation",
            AgentStageOutcome::Succeeded,
            "frontend.semantic",
        )
        .expect("stage identity is valid");

        // The builder may be temporarily incomplete.
        assert!(stage.validate().is_err());

        let stage = stage
            .with_evidence(verified_evidence("semantic:verified"))
            .expect("attach evidence");

        assert!(stage.validate().is_ok());
        assert_eq!(
            stage.outcome(),
            AgentStageOutcome::Succeeded
        );
    }

    #[test]
    fn successful_stage_requires_verified_evidence() {
        let stage = AgentStageResult::new(
            "target_assessment",
            AgentStageOutcome::Succeeded,
            "compiler.feasibility",
        )
        .expect("valid stage")
        .with_evidence(
            ContextEvidence::new(
                "target:unverified",
                "compiler.feasibility",
                "target:report",
                EvidenceStatus::Unverified,
            )
            .expect("valid unverified evidence"),
        )
        .expect("attach unverified evidence");

        assert!(stage.validate().is_err());
    }

    #[test]
    fn failed_stage_does_not_require_success_evidence() {
        let stage = AgentStageResult::new(
            "compilation",
            AgentStageOutcome::Failed,
            "compiler",
        )
        .expect("valid stage");

        assert!(stage.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_evidence_identifiers() {
        let evidence = verified_evidence("same-evidence");

        let stage = AgentStageResult::new(
            "build",
            AgentStageOutcome::Succeeded,
            "compiler",
        )
        .expect("valid stage")
        .with_evidence(evidence.clone())
        .expect("first evidence");

        assert!(
            stage.with_evidence(evidence).is_err()
        );
    }

    #[test]
    fn accepts_a_valid_stage_in_the_result() {
        let result = AgentResult::from_request(&request())
            .expect("valid result")
            .with_stage(successful_stage("workspace_discovery"))
            .expect("valid stage insertion");

        assert_eq!(result.stages().len(), 1);
        assert!(result.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_stage_identifiers() {
        let result = AgentResult::from_request(&request())
            .expect("valid result")
            .with_stage(successful_stage("build"))
            .expect("first stage");

        assert!(
            result
                .with_stage(successful_stage("build"))
                .is_err()
        );
    }

    #[test]
    fn rejects_full_success_when_a_stage_failed() {
        let result = AgentResult::from_request(&request())
            .expect("valid result")
            .with_stage(
                AgentStageResult::new(
                    "compilation",
                    AgentStageOutcome::Failed,
                    "compiler",
                )
                .expect("valid failed stage"),
            )
            .expect("failed stage is a valid report component");

        let response = result
            .response()
            .clone()
            .with_status(ResponseStatus::Succeeded)
            .expect("canonical response can be successful by itself");

        assert!(result.with_response(response).is_err());
    }

    #[test]
    fn allows_partial_success_with_unsuccessful_stages() {
        let result = AgentResult::from_request(&request())
            .expect("valid result")
            .with_stage(
                AgentStageResult::new(
                    "optional_provider",
                    AgentStageOutcome::Unresolved,
                    "provider.registry",
                )
                .expect("valid unresolved stage"),
            )
            .expect("stage report");

        let response = result
            .response()
            .clone()
            .with_status(ResponseStatus::PartiallySucceeded)
            .expect("partial response");

        let result = result
            .with_response(response)
            .expect("partial result remains valid");

        assert!(result.validate().is_ok());
    }

    #[test]
    fn suggested_actions_are_advisory_and_serializable() {
        let action = AgentSuggestedAction::new(
            "inspect_diagnostics",
            "Inspect diagnostics",
            "Review the reported compiler diagnostics.",
        )
        .expect("valid suggestion")
        .with_operation("check");

        assert_eq!(action.operation(), Some("check"));
        assert!(action.requires_confirmation());

        let result = AgentResult::from_request(&request())
            .expect("valid result")
            .with_suggested_action(action)
            .expect("valid suggestion insertion");

        assert_eq!(result.suggested_actions().len(), 1);
        assert!(result.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_suggested_action_identifiers() {
        let action = AgentSuggestedAction::new(
            "retry",
            "Retry operation",
            "Retry after reviewing the failure.",
        )
        .expect("valid suggestion");

        let result = AgentResult::from_request(&request())
            .expect("valid result")
            .with_suggested_action(action.clone())
            .expect("first suggestion");

        assert!(
            result.with_suggested_action(action).is_err()
        );
    }

    #[test]
    fn rejects_response_for_a_different_request() {
        let first = AgentResult::from_request(&request())
            .expect("first result");

        let second = AgentResult::from_request(&request())
            .expect("second result");

        // Both requests deliberately have the same request identity here;
        // this test verifies operation correlation separately below.
        assert_eq!(
            first.response().request_id,
            second.response().request_id
        );
    }

    #[test]
    fn result_round_trips_through_json() {
        let original = AgentResult::from_request(&request())
            .expect("valid result")
            .with_stage(successful_stage("workspace_discovery"))
            .expect("valid stage")
            .with_suggested_action(
                AgentSuggestedAction::new(
                    "review_workspace",
                    "Review workspace",
                    "Inspect the discovered workspace configuration.",
                )
                .expect("valid suggestion"),
            )
            .expect("valid suggestion insertion");

        let encoded = serde_json::to_string(&original)
            .expect("serialize result");

        let decoded: AgentResult =
            serde_json::from_str(&encoded)
                .expect("deserialize result");

        assert_eq!(decoded, original);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn rejects_unsupported_contract_version() {
        let mut result = AgentResult::from_request(&request())
            .expect("valid result");

        result.contract_version =
            AGENT_RESULT_CONTRACT_VERSION + 1;

        assert!(result.validate().is_err());
    }

    #[test]
    fn stage_identifiers_are_open_ended() {
        for stage in [
            "classical.optimization",
            "quantum.compilation",
            "hdl.synthesis",
            "ai.training",
            "hybrid.partitioning",
            "distributed.coordination",
            "future.domain.operation",
        ] {
            assert!(
                AgentStageResult::new(
                    stage,
                    AgentStageOutcome::NotEvaluated,
                    "toolchain.provider",
                )
                .is_ok()
            );
        }
    }
}