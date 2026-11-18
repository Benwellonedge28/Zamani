
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Deploy Command
//!
//! Path: src/toolchain/commands/deploy.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! PURPOSE
//!
//! Provides the public deployment-command orchestration boundary.
//!
//! The command:
//!
//! 1. Validates the public deployment request.
//! 2. Requests a deployment plan from an explicitly injected provider.
//! 3. Verifies that the plan belongs to the exact request.
//! 4. Preserves unresolved and infeasible deployment outcomes.
//! 5. Supports side-effect-free planning.
//! 6. Applies a ready plan only when explicitly requested.
//! 7. Validates the returned deployment receipt.
//! 8. Distinguishes acceptance, progress, completion, failure,
//!    cancellation and unknown outcomes.
//!
//! OWNERSHIP
//!
//! This module owns command orchestration and its command-level
//! plan, evidence, receipt and result contracts.
//!
//! It does not own:
//!
//! - Grammar, parsing or semantic analysis.
//! - Project or workspace discovery.
//! - Manifest parsing.
//! - Dependency resolution.
//! - Compilation or code generation.
//! - Artifact construction or storage.
//! - Target registration or capability discovery.
//! - Feasibility algorithms.
//! - Physical hardware discovery.
//! - Physical resource allocation.
//! - Infrastructure provisioning.
//! - Distributed placement or scheduling.
//! - Runtime execution.
//! - Provider authentication implementations.
//! - Cryptographic signing or verification.
//! - External process execution.
//!
//! These remain with their canonical repository authorities.
//!
//! INTEGRATION AUTHORITIES
//!
//! Public deployment request:
//!     crate::toolchain::api::request::DeployRequest
//!
//! Build orchestration:
//!     crate::toolchain::commands::build
//!
//! Compiler and artifact contracts:
//!     crate::compiler
//!
//! Backend and target realization:
//!     crate::backend
//!
//! Hardware and resource authorities:
//!     crate::zhal
//!
//! Runtime execution and handoff:
//!     crate::zur
//!
//! Distributed planning and execution:
//!     crate::distributed
//!
//! Deployment policy, security and provenance:
//!     Their respective canonical toolchain and repository authorities.
//!
//! PROVIDER CONTRACT
//!
//! A production DeploymentProvider must:
//!
//! - Use the canonical BuildCommand/build services.
//! - Verify actual compiler artifacts and their provenance.
//! - Resolve requested targets through canonical target authorities.
//! - Obtain feasibility evidence from the relevant compiler,
//!   backend, domain and hardware authorities.
//! - Validate the destination environment.
//! - Enforce deployment policy and authorization.
//! - Verify integrity and trust requirements.
//! - Plan without reserving or allocating physical resources.
//! - Revalidate mutable conditions before applying a plan.
//! - Use ZHAL and infrastructure providers for physical resources.
//! - Use ZUR for application runtime handoff and execution.
//! - Use distributed authorities for distributed placement,
//!   scheduling and lifecycle.
//! - Return evidence tied to the exact plan and deployment.
//! - Preserve indeterminate external side effects as indeterminate.
//!
//! Provider registration alone is not proof of provider readiness.
//! Provider acceptance is not proof of deployment completion.
//!
//! POCO-REAF
//!
//! One logical project and deployment request may produce multiple
//! target-specific realizations.
//!
//! The request's target selectors are intent, not feasibility evidence.
//! Different realizations may have different artifacts and execution
//! environments while preserving the applicable source semantics.
//!
//! No closed computational-domain or hardware inventory is defined here.
//!
//! SCALABILITY
//!
//! No arbitrary universal ceilings are introduced for:
//!
//! - Projects or workspace members.
//! - Target realizations.
//! - Providers.
//! - Artifacts.
//! - Hardware devices.
//! - CPUs, GPUs, FPGAs or QPUs.
//! - Quantum resources.
//! - Memory or storage.
//! - Distributed participants.
//! - Deployment steps.
//! - Computational domains.
//!
//! Collections grow subject to available memory, address space,
//! provider capabilities and configured operational budgets.
//!
//! Literal infinite deployment, storage or execution is not promised.
//!
//! SECURITY
//!
//! - A plan is not an authorization token.
//! - Apply-time authorization must be checked again.
//! - Provider references must not contain credentials.
//! - Evidence must not expose secrets.
//! - No shell command is constructed or executed here.
//! - A provider must use the canonical process and security policies.
//! - A plan must be bound to the exact request it assesses.
//! - A receipt must be bound to the exact plan it applies.
//!
//! FAILURE SEMANTICS
//!
//! - Invalid requests are rejected.
//! - Provider errors are propagated.
//! - Invalid provider plans are rejected.
//! - Blocked or unresolved plans are never applied.
//! - Plan-only requests never invoke the apply operation.
//! - An accepted deployment is not reported as completed.
//! - Partial, failed, cancelled and unknown outcomes remain distinct.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::toolchain::api::request::{
    DeployRequest,
    TargetSelection,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
};

use crate::toolchain::result::ToolchainResult;

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    field: &str,
    value: &str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::InvalidInput,
                format!("{field} must not be empty"),
            )
            .with_operation("commands.deploy.validate"),
        );
    }

    if value != value.trim() {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::InvalidInput,
                format!(
                    "{field} must not contain surrounding whitespace"
                ),
            )
            .with_operation("commands.deploy.validate"),
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ToolchainError::new(
                ToolchainErrorKind::InvalidInput,
                format!(
                    "{field} must not contain control characters"
                ),
            )
            .with_operation("commands.deploy.validate"),
        );
    }

    Ok(())
}

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ValidationFailed,
        message,
    )
    .with_operation("commands.deploy.validate")
}

fn provider_contract_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(
        ToolchainErrorKind::ProviderFailed,
        message,
    )
    .with_operation("commands.deploy.provider_contract")
}

// ============================================================================
// ASYNCHRONOUS PROVIDER CONTRACT
// ============================================================================

/// Future returned by deployment-provider operations.
///
/// The future is tied to the provider and the request or plan it
/// operates on. It must not require an arbitrary global executor.
///
/// Provider implementations must propagate cancellation and preserve
/// indeterminate external-operation outcomes.
pub type DeploymentFuture<'a, T> = Pin<
    Box<
        dyn Future<Output = ToolchainResult<T>>
            + Send
            + 'a
    >
>;

// ============================================================================
// DEPLOYMENT READINESS
// ============================================================================

/// Aggregate readiness of a deployment plan.
///
/// Readiness is established by the provider using evidence from the
/// canonical compiler, artifact, target, environment, security and
/// resource authorities.
///
/// `Ready` does not mean that resources have been reserved or that
/// deployment has been applied.
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
pub enum DeploymentReadiness {
    /// All required pre-deployment checks have passed.
    Ready,

    /// At least one required condition is known to be unsatisfied.
    Blocked,

    /// Required feasibility or compatibility information is unavailable.
    Unresolved,
}

// ============================================================================
// DEPLOYMENT EVIDENCE
// ============================================================================

/// Deployment lifecycle stage to which evidence applies.
///
/// These stages describe orchestration boundaries. They do not replace
/// the detailed evidence models owned by compiler, backend, ZHAL, ZUR
/// or distributed execution.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DeploymentStage {
    /// The project was built through the canonical build service.
    BuildVerified,

    /// The exact artifacts intended for deployment were verified.
    ArtifactsVerified,

    /// Target feasibility was established by canonical authorities.
    TargetFeasibilityEstablished,

    /// The requested deployment environment was validated.
    EnvironmentValidated,

    /// Deployment policy and authorization were satisfied.
    PolicyAuthorized,

    /// Required integrity and trust checks passed.
    SecurityVerified,

    /// Source, build and artifact provenance was verified.
    ProvenanceVerified,

    /// Resource requirements were assessed without allocation.
    ResourceRequirementsAssessed,

    /// Resource availability was observed but not reserved.
    ResourceAvailabilityObserved,

    /// Physical or infrastructure resources were actually reserved.
    ResourcesReserved,

    /// A deployment request was submitted to its authority.
    DeploymentSubmitted,

    /// The requested runtime was confirmed as started.
    RuntimeStarted,

    /// The requested runtime was confirmed as completed.
    RuntimeCompleted,

    /// The requested deployment operation was confirmed as completed.
    DeploymentCompleted,

    /// Cancellation was confirmed by the responsible authority.
    DeploymentCancelled,
}

/// Outcome of one deployment evidence observation.
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
pub enum DeploymentEvidenceStatus {
    /// The authority established the stated condition.
    Passed,

    /// The authority established that the condition is not satisfied.
    Failed,

    /// The authority could not establish the condition.
    Unresolved,

    /// The condition does not apply to this deployment.
    NotRequired,
}

/// One evidence item returned by a deployment provider.
///
/// `authority` identifies the subsystem or provider responsible for
/// the observation. `subject` identifies the project, artifact,
/// target, environment or other assessed subject.
///
/// A target-specific item should set `target` to the exact selector
/// or resolved target identity to which it applies.
///
/// Evidence descriptions and references must not contain credentials,
/// private keys, tokens or other secrets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeploymentEvidence {
    stage: DeploymentStage,
    status: DeploymentEvidenceStatus,
    authority: String,
    subject: String,
    target: Option<String>,
    reference: Option<String>,
    message: String,
}

impl DeploymentEvidence {
    /// Construct one evidence item.
    pub fn new(
        stage: DeploymentStage,
        status: DeploymentEvidenceStatus,
        authority: impl Into<String>,
        subject: impl Into<String>,
        message: impl Into<String>,
    ) -> ToolchainResult<Self> {
        let evidence = Self {
            stage,
            status,
            authority: authority.into(),
            subject: subject.into(),
            target: None,
            reference: None,
            message: message.into(),
        };

        evidence.validate()?;
        Ok(evidence)
    }

    /// Associate the evidence with a target selector or resolved target.
    #[must_use]
    pub fn for_target(
        mut self,
        target: impl Into<String>,
    ) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Attach a non-secret reference to authoritative evidence.
    #[must_use]
    pub fn with_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Return the evidence stage.
    #[must_use]
    pub const fn stage(&self) -> DeploymentStage {
        self.stage
    }

    /// Return the evidence status.
    #[must_use]
    pub const fn status(&self) -> DeploymentEvidenceStatus {
        self.status
    }

    /// Return the responsible authority identifier.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Return the assessed subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Return the associated target, if any.
    #[must_use]
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// Return the evidence reference, if any.
    #[must_use]
    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }

    /// Return the human-readable explanation.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Validate the evidence representation.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier("evidence.authority", &self.authority)?;
        validate_identifier("evidence.subject", &self.subject)?;

        if self.message.trim().is_empty() {
            return Err(validation_error(
                "deployment evidence message must not be empty",
            ));
        }

        if self.message.chars().any(char::is_control) {
            return Err(validation_error(
                "deployment evidence message contains control characters",
            ));
        }

        if let Some(target) = &self.target {
            validate_identifier("evidence.target", target)?;
        }

        if let Some(reference) = &self.reference {
            validate_identifier("evidence.reference", reference)?;
        }

        Ok(())
    }
}

// ============================================================================
// DEPLOYMENT PLAN
// ============================================================================

/// Immutable deployment plan returned by an authoritative provider.
///
/// The plan is bound to the exact request it assesses.
///
/// A plan contains no physical resource lease and does not prove that
/// resources remain available after planning. Resource availability
/// must be revalidated at apply time.
///
/// The provider must build and verify the required artifacts before
/// reporting `Ready`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeploymentPlan {
    plan_id: String,
    provider: String,
    request: DeployRequest,
    readiness: DeploymentReadiness,
    evidence: Vec<DeploymentEvidence>,
}

impl DeploymentPlan {
    /// Construct a provider-backed deployment plan.
    ///
    /// Construction validates structural invariants. It does not
    /// independently authenticate the provider's evidence.
    ///
    /// A production provider must obtain evidence from the canonical
    /// authorities described in this module's integration contract.
    pub fn new(
        plan_id: impl Into<String>,
        provider: impl Into<String>,
        request: DeployRequest,
        readiness: DeploymentReadiness,
        evidence: Vec<DeploymentEvidence>,
    ) -> ToolchainResult<Self> {
        let plan = Self {
            plan_id: plan_id.into(),
            provider: provider.into(),
            request,
            readiness,
            evidence,
        };

        plan.validate()?;
        Ok(plan)
    }

    /// Return the provider's stable plan identity.
    #[must_use]
    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    /// Return the provider responsible for planning.
    #[must_use]
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Return the exact request assessed by this plan.
    #[must_use]
    pub fn request(&self) -> &DeployRequest {
        &self.request
    }

    /// Return aggregate deployment readiness.
    #[must_use]
    pub const fn readiness(&self) -> DeploymentReadiness {
        self.readiness
    }

    /// Return the plan's evidence.
    #[must_use]
    pub fn evidence(&self) -> &[DeploymentEvidence] {
        &self.evidence
    }

    /// Validate the plan and all attached evidence.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier("plan.plan_id", &self.plan_id)?;
        validate_identifier("plan.provider", &self.provider)?;

        self.request.validate().map_err(|error| {
            validation_error(
                format!("deployment plan contains an invalid request: {error}"),
            )
            .with_source(error)
        })?;

        for evidence in &self.evidence {
            evidence.validate()?;

            if is_apply_or_runtime_stage(evidence.stage) {
                return Err(validation_error(
                    "a deployment plan must not claim resource reservation, \
                     deployment submission, runtime launch or completion",
                ));
            }
        }

        if self.readiness == DeploymentReadiness::Ready {
            self.validate_ready_evidence()?;
        }

        Ok(())
    }

    /// Ensure the plan assesses the exact request submitted to the command.
    pub fn validate_for(
        &self,
        request: &DeployRequest,
    ) -> ToolchainResult<()> {
        self.validate()?;

        if &self.request != request {
            return Err(provider_contract_error(
                "deployment provider returned a plan for a different request",
            ));
        }

        Ok(())
    }

    fn validate_ready_evidence(&self) -> ToolchainResult<()> {
        let required_global_stages = [
            DeploymentStage::BuildVerified,
            DeploymentStage::EnvironmentValidated,
            DeploymentStage::PolicyAuthorized,
            DeploymentStage::SecurityVerified,
            DeploymentStage::ProvenanceVerified,
        ];

        for stage in required_global_stages {
            let passed = self.evidence.iter().any(|item| {
                item.stage == stage
                    && item.status == DeploymentEvidenceStatus::Passed
                    && item.target.is_none()
            });

            if !passed {
                return Err(provider_contract_error(
                    format!(
                        "ready deployment plan is missing passed global evidence \
                         for stage {stage:?}",
                    ),
                ));
            }
        }

        self.validate_target_evidence()
    }

    fn validate_target_evidence(&self) -> ToolchainResult<()> {
        let has_passed_target_evidence = |stage: DeploymentStage,
                                          target: Option<&str>| {
            self.evidence.iter().any(|item| {
                item.stage == stage
                    && item.status == DeploymentEvidenceStatus::Passed
                    && match target {
                        Some(expected) => {
                            item.target.as_deref() == Some(expected)
                        }
                        None => item.target.is_some(),
                    }
            })
        };

        match &self.request.targets {
            TargetSelection::Explicit {
                targets,
                require_all,
            } => {
                let passed = targets
                    .iter()
                    .filter(|target| {
                        has_passed_target_evidence(
                            DeploymentStage::TargetFeasibilityEstablished,
                            Some(target.as_str()),
                        ) && has_passed_target_evidence(
                            DeploymentStage::ArtifactsVerified,
                            Some(target.as_str()),
                        )
                    })
                    .count();

                if *require_all && passed != targets.len() {
                    return Err(provider_contract_error(
                        "ready deployment plan does not establish feasibility \
                         and verified artifacts for every required target",
                    ));
                }

                if passed == 0 {
                    return Err(provider_contract_error(
                        "ready deployment plan has no target with verified \
                         feasibility and artifacts",
                    ));
                }
            }

            TargetSelection::ResolveFromCapabilities { .. } => {
                let resolved = self.evidence.iter().any(|item| {
                    item.stage
                        == DeploymentStage::TargetFeasibilityEstablished
                        && item.status == DeploymentEvidenceStatus::Passed
                        && item.target.is_some()
                });

                let artifacts = self.evidence.iter().any(|item| {
                    item.stage == DeploymentStage::ArtifactsVerified
                        && item.status == DeploymentEvidenceStatus::Passed
                        && item.target.is_some()
                });

                if !resolved || !artifacts {
                    return Err(provider_contract_error(
                        "capability-based deployment plan must identify at least \
                         one feasible target and its verified artifact",
                    ));
                }
            }
        }

        Ok(())
    }
}

fn is_apply_or_runtime_stage(stage: DeploymentStage) -> bool {
    matches!(
        stage,
        DeploymentStage::ResourcesReserved
            | DeploymentStage::DeploymentSubmitted
            | DeploymentStage::RuntimeStarted
            | DeploymentStage::RuntimeCompleted
            | DeploymentStage::DeploymentCompleted
            | DeploymentStage::DeploymentCancelled
    )
}

// ============================================================================
// DEPLOYMENT RECEIPT
// ============================================================================

/// Actual state reported by the deployment authority.
///
/// `Accepted` and `InProgress` are not successful completion.
/// `Unknown` is deliberately distinct from failure and success.
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
pub enum DeploymentReceiptStatus {
    /// The deployment authority accepted the request.
    Accepted,

    /// The deployment is still in progress.
    InProgress,

    /// The requested deployment operation completed successfully.
    Completed,

    /// Some deployment targets or operations completed and others did not.
    PartiallyCompleted,

    /// The deployment authority confirmed failure.
    Failed,

    /// Cancellation was confirmed.
    Cancelled,

    /// The authority could not establish the current outcome.
    Unknown,
}

/// Receipt returned by the authority that applied a deployment plan.
///
/// A receipt is bound to the plan identity. Its status describes the
/// deployment operation, not necessarily the lifetime of the deployed
/// application.
///
/// Runtime start and runtime completion, when applicable, are separate
/// evidence stages owned by ZUR.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeploymentReceipt {
    deployment_id: String,
    plan_id: String,
    provider: String,
    status: DeploymentReceiptStatus,
    provider_reference: Option<String>,
    evidence: Vec<DeploymentEvidence>,
}

impl DeploymentReceipt {
    /// Construct and validate a deployment receipt.
    pub fn new(
        deployment_id: impl Into<String>,
        plan_id: impl Into<String>,
        provider: impl Into<String>,
        status: DeploymentReceiptStatus,
        evidence: Vec<DeploymentEvidence>,
    ) -> ToolchainResult<Self> {
        let receipt = Self {
            deployment_id: deployment_id.into(),
            plan_id: plan_id.into(),
            provider: provider.into(),
            status,
            provider_reference: None,
            evidence,
        };

        receipt.validate()?;
        Ok(receipt)
    }

    /// Attach a non-secret provider-side correlation reference.
    #[must_use]
    pub fn with_provider_reference(
        mut self,
        reference: impl Into<String>,
    ) -> Self {
        self.provider_reference = Some(reference.into());
        self
    }

    /// Return the deployment identity.
    #[must_use]
    pub fn deployment_id(&self) -> &str {
        &self.deployment_id
    }

    /// Return the associated plan identity.
    #[must_use]
    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }

    /// Return the applying provider identity.
    #[must_use]
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Return the actual deployment status.
    #[must_use]
    pub const fn status(&self) -> DeploymentReceiptStatus {
        self.status
    }

    /// Return the provider-side reference, if available.
    #[must_use]
    pub fn provider_reference(&self) -> Option<&str> {
        self.provider_reference.as_deref()
    }

    /// Return deployment evidence.
    #[must_use]
    pub fn evidence(&self) -> &[DeploymentEvidence] {
        &self.evidence
    }

    /// Validate the receipt representation.
    pub fn validate(&self) -> ToolchainResult<()> {
        validate_identifier("receipt.deployment_id", &self.deployment_id)?;
        validate_identifier("receipt.plan_id", &self.plan_id)?;
        validate_identifier("receipt.provider", &self.provider)?;

        if let Some(reference) = &self.provider_reference {
            validate_identifier("receipt.provider_reference", reference)?;
        }

        for evidence in &self.evidence {
            evidence.validate()?;
        }

        match self.status {
            DeploymentReceiptStatus::Completed => {
                let completed = self.evidence.iter().any(|item| {
                    item.stage == DeploymentStage::DeploymentCompleted
                        && item.status == DeploymentEvidenceStatus::Passed
                });

                if !completed {
                    return Err(provider_contract_error(
                        "completed deployment receipt lacks confirmed \
                         deployment-completion evidence",
                    ));
                }
            }

            DeploymentReceiptStatus::Cancelled => {
                let cancelled = self.evidence.iter().any(|item| {
                    item.stage == DeploymentStage::DeploymentCancelled
                        && item.status == DeploymentEvidenceStatus::Passed
                });

                if !cancelled {
                    return Err(provider_contract_error(
                        "cancelled deployment receipt lacks confirmed \
                         cancellation evidence",
                    ));
                }
            }

            _ => {}
        }

        Ok(())
    }

    /// Ensure the receipt applies to the exact plan and provider.
    pub fn validate_for(
        &self,
        plan: &DeploymentPlan,
    ) -> ToolchainResult<()> {
        self.validate()?;

        if self.plan_id != plan.plan_id {
            return Err(provider_contract_error(
                "deployment receipt references a different plan",
            ));
        }

        if self.provider != plan.provider {
            return Err(provider_contract_error(
                "deployment receipt identifies a different provider \
                 from the plan",
            ));
        }

        Ok(())
    }

    /// Whether the provider confirmed successful deployment completion.
    #[must_use]
    pub const fn is_completed_successfully(&self) -> bool {
        matches!(
            self.status,
            DeploymentReceiptStatus::Completed
        )
    }
}

// ============================================================================
// COMMAND RESULT
// ============================================================================

/// Aggregate status of one deployment-command invocation.
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
pub enum DeploymentCommandStatus {
    /// Planning completed and the plan is ready for a later apply operation.
    Planned,

    /// Planning established that required conditions are not satisfied.
    Blocked,

    /// Planning could not establish required feasibility or compatibility.
    Unresolved,

    /// The deployment authority accepted the request.
    Accepted,

    /// The deployment authority reports ongoing work.
    InProgress,

    /// The deployment authority confirmed completion.
    Completed,

    /// The deployment authority reports partial completion.
    PartiallyCompleted,

    /// The deployment authority confirmed failure.
    Failed,

    /// Cancellation was confirmed.
    Cancelled,

    /// The deployment outcome is unknown.
    Unknown,
}

/// Structured result of a deploy-command invocation.
///
/// A plan-only result contains a plan but no deployment receipt.
/// A result with a receipt preserves the authority's actual status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeploymentCommandResult {
    request: DeployRequest,
    plan: DeploymentPlan,
    receipt: Option<DeploymentReceipt>,
    status: DeploymentCommandStatus,
}

impl DeploymentCommandResult {
    fn planned(
        request: DeployRequest,
        plan: DeploymentPlan,
    ) -> Self {
        let status = match plan.readiness {
            DeploymentReadiness::Ready => {
                DeploymentCommandStatus::Planned
            }
            DeploymentReadiness::Blocked => {
                DeploymentCommandStatus::Blocked
            }
            DeploymentReadiness::Unresolved => {
                DeploymentCommandStatus::Unresolved
            }
        };

        Self {
            request,
            plan,
            receipt: None,
            status,
        }
    }

    fn applied(
        request: DeployRequest,
        plan: DeploymentPlan,
        receipt: DeploymentReceipt,
    ) -> Self {
        let status = match receipt.status {
            DeploymentReceiptStatus::Accepted => {
                DeploymentCommandStatus::Accepted
            }
            DeploymentReceiptStatus::InProgress => {
                DeploymentCommandStatus::InProgress
            }
            DeploymentReceiptStatus::Completed => {
                DeploymentCommandStatus::Completed
            }
            DeploymentReceiptStatus::PartiallyCompleted => {
                DeploymentCommandStatus::PartiallyCompleted
            }
            DeploymentReceiptStatus::Failed => {
                DeploymentCommandStatus::Failed
            }
            DeploymentReceiptStatus::Cancelled => {
                DeploymentCommandStatus::Cancelled
            }
            DeploymentReceiptStatus::Unknown => {
                DeploymentCommandStatus::Unknown
            }
        };

        Self {
            request,
            plan,
            receipt: Some(receipt),
            status,
        }
    }

    /// Return the original request.
    #[must_use]
    pub fn request(&self) -> &DeployRequest {
        &self.request
    }

    /// Return the validated deployment plan.
    #[must_use]
    pub fn plan(&self) -> &DeploymentPlan {
        &self.plan
    }

    /// Return the deployment receipt, if the plan was applied.
    #[must_use]
    pub fn receipt(&self) -> Option<&DeploymentReceipt> {
        self.receipt.as_ref()
    }

    /// Return the aggregate command status.
    #[must_use]
    pub const fn status(&self) -> DeploymentCommandStatus {
        self.status
    }

    /// Whether the deployment authority confirmed completion.
    #[must_use]
    pub const fn is_completed_successfully(&self) -> bool {
        matches!(
            self.status,
            DeploymentCommandStatus::Completed
        )
    }
}

// ============================================================================
// DEPLOYMENT PROVIDER
// ============================================================================

/// Canonical integration boundary for deployment planning and application.
///
/// Implementations must be adapters over existing repository authorities.
/// They must not create a competing compiler, target registry, hardware
/// abstraction, distributed scheduler or runtime.
///
/// ## `plan` requirements
///
/// Planning must be side-effect-free with respect to deployment:
///
/// - Do not reserve or allocate physical resources.
/// - Do not program hardware.
/// - Do not submit runtime execution.
/// - Do not create infrastructure.
/// - Do not start application processes.
///
/// Planning must:
///
/// - Resolve the project and workspace.
/// - Load and validate the canonical manifest.
/// - Resolve dependencies and effective profiles.
/// - Invoke the canonical build service when deployment requires artifacts.
/// - Verify actual compiler artifacts and their provenance.
/// - Resolve requested targets.
/// - Obtain per-target feasibility evidence.
/// - Validate environment, policy and authorization.
/// - Assess resource requirements and availability.
/// - Return `Ready`, `Blocked` or `Unresolved` honestly.
///
/// ## `apply` requirements
///
/// Applying a plan must:
///
/// - Verify that the plan is still valid and has not been superseded.
/// - Recheck authorization and mutable policy conditions.
/// - Recheck environment and resource availability.
/// - Reserve resources through ZHAL or the appropriate infrastructure
///   provider, not through this command.
/// - Use the appropriate hardware or deployment provider.
/// - Use distributed authorities for distributed placement and execution.
/// - Hand off application execution to ZUR when the deployment intent
///   requires the application to start or run.
/// - Preserve actual asynchronous/remote execution identities.
/// - Return `Accepted` or `InProgress` when completion is not confirmed.
/// - Return `Completed` only with confirmed completion evidence.
/// - Return `Unknown` when an external side effect may have happened
///   but its outcome cannot be established.
///
/// Providers must implement idempotency and safe retry semantics using
/// their canonical operation identity and caller context. This command
/// does not invent an idempotency key or blindly retry external effects.
pub trait DeploymentProvider: Send + Sync {
    /// Construct a deployment plan without applying it.
    fn plan<'a>(
        &'a self,
        request: &'a DeployRequest,
    ) -> DeploymentFuture<'a, DeploymentPlan>;

    /// Apply a previously validated deployment plan.
    ///
    /// This method must perform apply-time revalidation. A plan is not
    /// an authorization grant or a resource reservation.
    fn apply<'a>(
        &'a self,
        plan: &'a DeploymentPlan,
    ) -> DeploymentFuture<'a, DeploymentReceipt>;
}

// ============================================================================
// DEPLOY COMMAND
// ============================================================================

/// Deployment-command coordinator.
///
/// The provider is injected explicitly. There is no hidden global
/// registry, implicit cloud provider, native-target fallback or
/// fabricated default implementation.
pub struct DeployCommand {
    provider: std::sync::Arc<dyn DeploymentProvider>,
}

impl DeployCommand {
    /// Construct a deployment command with an explicit provider.
    #[must_use]
    pub fn new(
        provider: std::sync::Arc<dyn DeploymentProvider>,
    ) -> Self {
        Self { provider }
    }

    /// Produce a deployment plan without applying it.
    ///
    /// This method always invokes only the provider's side-effect-free
    /// planning operation.
    ///
    /// # Errors
    ///
    /// Returns request-validation, provider or provider-contract errors.
    pub async fn plan(
        &self,
        request: &DeployRequest,
    ) -> ToolchainResult<DeploymentPlan> {
        request.validate().map_err(|error| {
            validation_error(
                format!("invalid deployment request: {error}"),
            )
            .with_source(error)
        })?;

        let plan = self.provider.plan(request).await?;

        plan.validate_for(request)?;

        Ok(plan)
    }

    /// Execute the requested deployment workflow.
    ///
    /// If `request.plan_only` is true, this method never calls `apply`.
    ///
    /// If the plan is blocked or unresolved, this method returns the
    /// structured plan result without applying it.
    ///
    /// Only a ready plan can be passed to the provider's apply operation.
    ///
    /// An accepted or in-progress receipt is not reported as completed.
    ///
    /// # Errors
    ///
    /// Returns request-validation, planning, provider or provider-contract
    /// errors. Provider errors retain their original causal source.
    pub async fn execute(
        &self,
        request: DeployRequest,
    ) -> ToolchainResult<DeploymentCommandResult> {
        let plan = self.plan(&request).await?;

        if request.plan_only
            || plan.readiness != DeploymentReadiness::Ready
        {
            return Ok(DeploymentCommandResult::planned(
                request,
                plan,
            ));
        }

        let receipt = self.provider.apply(&plan).await?;

        receipt.validate_for(&plan)?;

        Ok(DeploymentCommandResult::applied(
            request,
            plan,
            receipt,
        ))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::toolchain::api::request::{
        ProjectLocator,
        ProjectScope,
        TargetSelection,
        TargetSelector,
    };

    fn request() -> DeployRequest {
        let project = ProjectLocator::from_path("fixture")
            .expect("valid project path");

        let scope = ProjectScope::new(project)
            .expect("valid project scope");

        let target = TargetSelector::new("test.target")
            .expect("valid target selector");

        DeployRequest::new(
            scope,
            TargetSelection::explicit(vec![target], true)
                .expect("valid target selection"),
        )
        .expect("valid deployment request")
    }

    fn evidence(
        stage: DeploymentStage,
        target: Option<&str>,
    ) -> DeploymentEvidence {
        let evidence = DeploymentEvidence::new(
            stage,
            DeploymentEvidenceStatus::Passed,
            "test.authority",
            "test.subject",
            "test evidence",
        )
        .expect("valid evidence");

        match target {
            Some(target) => evidence.for_target(target),
            None => evidence,
        }
    }

    fn ready_evidence() -> Vec<DeploymentEvidence> {
        vec![
            evidence(DeploymentStage::BuildVerified, None),
            evidence(DeploymentStage::EnvironmentValidated, None),
            evidence(DeploymentStage::PolicyAuthorized, None),
            evidence(DeploymentStage::SecurityVerified, None),
            evidence(DeploymentStage::ProvenanceVerified, None),
            evidence(
                DeploymentStage::TargetFeasibilityEstablished,
                Some("test.target"),
            ),
            evidence(
                DeploymentStage::ArtifactsVerified,
                Some("test.target"),
            ),
        ]
    }

    #[test]
    fn ready_plan_requires_complete_global_evidence() {
        let result = DeploymentPlan::new(
            "plan-1",
            "test.provider",
            request(),
            DeploymentReadiness::Ready,
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn ready_plan_requires_target_feasibility_and_artifacts() {
        let mut evidence = ready_evidence();
        evidence.retain(|item| {
            item.stage() != DeploymentStage::ArtifactsVerified
        });

        let result = DeploymentPlan::new(
            "plan-1",
            "test.provider",
            request(),
            DeploymentReadiness::Ready,
            evidence,
        );

        assert!(result.is_err());
    }

    #[test]
    fn ready_plan_rejects_resource_reservation_during_planning() {
        let mut evidence = ready_evidence();
        evidence.push(evidence(
            DeploymentStage::ResourcesReserved,
            None,
        ));

        let result = DeploymentPlan::new(
            "plan-1",
            "test.provider",
            request(),
            DeploymentReadiness::Ready,
            evidence,
        );

        assert!(result.is_err());
    }

    #[test]
    fn ready_plan_accepts_complete_preflight_evidence() {
        let plan = DeploymentPlan::new(
            "plan-1",
            "test.provider",
            request(),
            DeploymentReadiness::Ready,
            ready_evidence(),
        );

        assert!(plan.is_ok());
    }

    #[test]
    fn plan_must_belong_to_the_exact_request() {
        let original = request();

        let plan = DeploymentPlan::new(
            "plan-1",
            "test.provider",
            original,
            DeploymentReadiness::Blocked,
            Vec::new(),
        )
        .expect("valid blocked plan");

        let different_request = request();

        assert!(plan.validate_for(&different_request).is_ok());
    }

    #[test]
    fn accepted_receipt_is_not_successful_completion() {
        let receipt = DeploymentReceipt::new(
            "deployment-1",
            "plan-1",
            "test.provider",
            DeploymentReceiptStatus::Accepted,
            vec![],
        )
        .expect("valid accepted receipt");

        assert!(!receipt.is_completed_successfully());
    }

    #[test]
    fn completed_receipt_requires_completion_evidence() {
        let receipt = DeploymentReceipt::new(
            "deployment-1",
            "plan-1",
            "test.provider",
            DeploymentReceiptStatus::Completed,
            vec![],
        );

        assert!(receipt.is_err());
    }

    #[test]
    fn completed_receipt_requires_confirmed_deployment_stage() {
        let receipt = DeploymentReceipt::new(
            "deployment-1",
            "plan-1",
            "test.provider",
            DeploymentReceiptStatus::Completed,
            vec![evidence(
                DeploymentStage::DeploymentCompleted,
                None,
            )],
        )
        .expect("completion evidence is present");

        assert!(receipt.is_completed_successfully());
    }
}
