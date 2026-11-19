
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Agent Compliance
//!
//! Path: src/toolchain/agents/compliance.rs
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Coordinates compliance verification for human and AI-agent
//! workflows using the existing canonical toolchain authorities.
//!
//! Compliance checks cover:
//!
//! - Structural validity of agent requests.
//! - Binding between authorization and the requested command.
//! - Request correlation.
//! - Explicit authorization outcomes.
//! - Structural validity of agent results.
//! - Correlation between requests and results.
//! - Additional registered agent compliance controls.
//!
//! ## OWNERSHIP
//!
//! This module owns:
//!
//! - Agent compliance check contracts.
//! - Agent compliance evidence representation.
//! - Compliance control extension contracts.
//! - Compliance report aggregation.
//! - Fail-closed compliance acceptance.
//! - Agent workflow compliance orchestration.
//!
//! ## DOES NOT OWN
//!
//! This module does not own:
//!
//! - Authentication or identity verification.
//! - Authorization or permission evaluation.
//! - Security policy definitions.
//! - Trust evaluation.
//! - Cryptographic verification.
//! - Audit persistence.
//! - Audit retention.
//! - Semantic analysis.
//! - Capability discovery.
//! - Target feasibility.
//! - Resource allocation.
//! - Compiler execution.
//! - Backend selection.
//! - Hardware discovery.
//! - Runtime enforcement.
//! - Agent reasoning or planning.
//!
//! Those responsibilities remain with their authoritative
//! repository subsystems.
//!
//! ## AUTHORITATIVE INTEGRATION
//!
//! Agent context:
//!     crate::toolchain::agents::context
//!
//! Agent requests:
//!     crate::toolchain::agents::requests
//!
//! Agent results:
//!     crate::toolchain::agents::results
//!
//! Authorization:
//!     crate::toolchain::agents::permissions
//!
//! Command identity:
//!     crate::toolchain::commands
//!
//! Audit:
//!     crate::toolchain::agents::audit
//!
//! Security policy:
//!     crate::toolchain::security::policy
//!
//! Security validation:
//!     crate::toolchain::security::validation
//!
//! Toolchain errors:
//!     crate::toolchain::errors
//!
//! Source-level security intent:
//!     grammar/security/permissions.g4
//!     grammar/security/authorization.g4
//!     grammar/security/audit.g4
//!
//! Normative semantics:
//!     grammar/spec/security.md
//!
//! ## SECURITY INVARIANTS
//!
//! 1. Compliance never authenticates a client.
//! 2. Compliance never grants authorization.
//! 3. Only the canonical authorization authority establishes
//!    authorization.
//! 4. Only an explicit Permit establishes authorization success.
//! 5. Missing authorization is NotEvaluated.
//! 6. Denied authorization is Rejected.
//! 7. Indeterminate authorization never grants permission.
//! 8. Agent-supplied identity is never authenticated identity.
//! 9. A mismatched command authorization is rejected.
//! 10. A mismatched request correlation is rejected.
//! 11. Invalid requests cannot pass compliance.
//! 12. Invalid results cannot pass compliance.
//! 13. Failed compliance controls cannot become success.
//! 14. Registered controls are not automatically trusted evidence.
//! 15. Advisory controls cannot override mandatory failures.
//! 16. An empty compliance report cannot authorize an operation.
//! 17. Suggested agent actions remain advisory.
//! 18. Compliance does not replace command-level audit enforcement.
//! 19. No secrets or credentials are included in compliance reports.
//! 20. Unknown or unresolved outcomes never become verified.
//!
//! ## POCO-REAF
//!
//! This module uses open-ended compliance-control identities.
//!
//! It does not enumerate:
//!
//! - Computing domains.
//! - Processor architectures.
//! - Hardware vendors.
//! - Quantum devices.
//! - Accelerator types.
//! - Distributed participants.
//! - Execution environments.
//! - Agent implementations.
//!
//! The same compliance architecture applies to classical,
//! quantum, HDL, AI, hybrid, distributed and future domains.
//!
//! ## SCALABILITY
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Compliance controls.
//! - Compliance reports.
//! - Agent workflows.
//! - Computing domains.
//! - Targets.
//! - Providers.
//! - Hardware resources.
//!
//! Collections grow according to available implementation resources.
//!
//! Allocation failures are reported explicitly where fallible
//! collection reservation is available.
//!
//! No arbitrary hardware-capacity limits are introduced.
//!
//! Literal infinite computation or storage is not promised.
//!
//! ## DETERMINISM
//!
//! Built-in checks execute in a defined order.
//!
//! Extension controls execute in their configured order.
//!
//! Report ordering is independent of hash-table iteration.
//!
//! Time, randomness, network access and hardware discovery are
//! not implicitly introduced.
//!
//! ## TRUST BOUNDARY
//!
//! Agent compliance controls are trusted in-process integration code.
//!
//! Their results are evidence supplied by their respective
//! authorities, not cryptographic proof generated by this module.
//!
//! Untrusted remote results must be authenticated and validated
//! by their owning transport and security authorities before use.

use std::collections::HashSet;
use std::fmt;
use std::sync::Arc;

use crate::toolchain::agents::permissions::{
    AuthorizationReceipt,
};

use crate::toolchain::agents::requests::{
    AgentRequest,
};

use crate::toolchain::agents::results::{
    AgentResult,
};

use crate::toolchain::commands::{
    CommandDescriptor,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainResult,
};

use crate::toolchain::security::policy::{
    DecisionEffect,
};

// ============================================================================
// 1. MODULE IDENTITY
// ============================================================================

/// Stable component identity.
pub const COMPONENT: &str =
    "toolchain.agents.compliance";

/// Stable compliance evaluation operation.
pub const EVALUATE_OPERATION: &str =
    "toolchain.agents.compliance.evaluate";

/// Version of the agent compliance contract.
///
/// Independent of the language, grammar, compiler, API,
/// authorization policy and package-format versions.
pub const COMPLIANCE_API_VERSION: u32 = 1;

// ============================================================================
// 2. BUILT-IN CHECK IDENTITIES
// ============================================================================

/// Compliance check identifiers.
///
/// These are stable identities for built-in checks.
/// Additional controls use open-ended identifiers.
pub mod check_ids {
    /// Agent request structural validation.
    pub const REQUEST_VALIDATION: &str =
        "agent.request.validation";

    /// Authorization and command-binding validation.
    pub const AUTHORIZATION: &str =
        "agent.authorization.validation";

    /// Agent result structural and correlation validation.
    pub const RESULT_VALIDATION: &str =
        "agent.result.validation";
}

// ============================================================================
// 3. VALIDATION HELPERS
// ============================================================================

fn validation_error(
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::validation_failed(message)
        .with_operation(EVALUATE_OPERATION)
        .with_context(COMPONENT)
}

fn allocation_error(
    collection: &str,
    error: impl fmt::Display,
) -> ToolchainError {
    ToolchainError::resource_exhausted(format!(
        "unable to allocate memory for agent compliance {collection}: {error}"
    ))
    .with_operation(EVALUATE_OPERATION)
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
            "{field} contains prohibited control characters"
        )));
    }

    Ok(())
}

// ============================================================================
// 4. COMPLIANCE REQUIREMENTS
// ============================================================================

/// Importance of an agent compliance check.
///
/// Required checks must be affirmatively verified.
///
/// Advisory checks provide additional information but cannot
/// override an explicit rejection.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[non_exhaustive]
pub enum ComplianceRequirement {
    /// Mandatory compliance condition.
    Required,

    /// Advisory compliance condition.
    Advisory,
}

// ============================================================================
// 5. COMPLIANCE STATUS
// ============================================================================

/// Outcome of an individual compliance check.
///
/// Unknown or inconclusive information must never be interpreted
/// as successful verification.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[non_exhaustive]
pub enum ComplianceStatus {
    /// The authoritative check established compliance.
    Verified,

    /// The authoritative check established a violation.
    Rejected,

    /// Available information could not establish compliance.
    Indeterminate,

    /// The required check has not been performed.
    NotEvaluated,
}

impl ComplianceStatus {
    /// Whether the check affirmatively establishes compliance.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }

    /// Whether the check explicitly rejects compliance.
    #[must_use]
    pub const fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected)
    }

    /// Whether the check remains unresolved.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(
            self,
            Self::Indeterminate | Self::NotEvaluated
        )
    }
}

// ============================================================================
// 6. COMPLIANCE EVIDENCE
// ============================================================================

/// Evidence supplied by an agent compliance authority.
///
/// The authority and reference identify the source of the evidence.
///
/// This module does not independently verify external evidence.
/// The supplying integration must use its authoritative subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentComplianceEvidence {
    authority: Arc<str>,
    reference: Arc<str>,
    status: ComplianceStatus,
    message: Arc<str>,
}

impl AgentComplianceEvidence {
    /// Construct compliance evidence.
    pub fn new(
        authority: impl Into<Arc<str>>,
        reference: impl Into<Arc<str>>,
        status: ComplianceStatus,
        message: impl Into<Arc<str>>,
    ) -> ToolchainResult<Self> {
        let authority = authority.into();
        let reference = reference.into();
        let message = message.into();

        validate_identifier(
            "compliance evidence authority",
            authority.as_ref(),
        )?;

        validate_identifier(
            "compliance evidence reference",
            reference.as_ref(),
        )?;

        validate_identifier(
            "compliance evidence message",
            message.as_ref(),
        )?;

        Ok(Self {
            authority,
            reference,
            status,
            message,
        })
    }

    /// Return the evidence authority.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Return the non-secret evidence reference.
    #[must_use]
    pub fn reference(&self) -> &str {
        &self.reference
    }

    /// Return the compliance status.
    #[must_use]
    pub const fn status(&self) -> ComplianceStatus {
        self.status
    }

    /// Return the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// 7. EXTENSIBLE COMPLIANCE CONTROLS
// ============================================================================

/// Integration contract for an additional agent compliance authority.
///
/// Controls can integrate organization-specific, project-specific,
/// regulatory, operational or domain-specific compliance requirements.
///
/// The control must delegate verification to its actual authoritative
/// subsystem.
///
/// A control must not:
///
/// - Authenticate an agent by inspecting descriptive request fields.
/// - Grant authorization.
/// - Override an authorization denial.
/// - Claim target feasibility.
/// - Treat registration as evidence.
/// - Invent successful verification.
/// - Execute the requested toolchain operation.
///
/// Control implementations must not expose credentials, secrets,
/// private keys or sensitive source content in evidence messages.
pub trait AgentComplianceControl: Send + Sync + 'static {
    /// Return the stable, open-ended identity of this control.
    fn id(&self) -> &str;

    /// Return the importance of this compliance requirement.
    fn requirement(&self) -> ComplianceRequirement;

    /// Evaluate the supplied agent workflow.
    ///
    /// This operation must perform compliance inspection only.
    ///
    /// It must not execute the requested command or produce
    /// external side effects.
    ///
    /// # Errors
    ///
    /// Return an error if the authoritative compliance check
    /// cannot complete.
    ///
    /// Such an error must never be converted into Verified.
    fn evaluate(
        &self,
        request: &AgentRequest,
        result: Option<&AgentResult>,
    ) -> ToolchainResult<AgentComplianceEvidence>;
}

// ============================================================================
// 8. INDIVIDUAL COMPLIANCE CHECK
// ============================================================================

/// Immutable outcome of one compliance check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentComplianceCheck {
    id: Arc<str>,
    requirement: ComplianceRequirement,
    status: ComplianceStatus,
    authority: Arc<str>,
    evidence_reference: Option<Arc<str>>,
    message: Arc<str>,
}

impl AgentComplianceCheck {
    fn new(
        id: impl Into<Arc<str>>,
        requirement: ComplianceRequirement,
        status: ComplianceStatus,
        authority: impl Into<Arc<str>>,
        evidence_reference: Option<Arc<str>>,
        message: impl Into<Arc<str>>,
    ) -> ToolchainResult<Self> {
        let id = id.into();
        let authority = authority.into();
        let message = message.into();

        validate_identifier(
            "compliance check identity",
            id.as_ref(),
        )?;

        validate_identifier(
            "compliance check authority",
            authority.as_ref(),
        )?;

        validate_identifier(
            "compliance check message",
            message.as_ref(),
        )?;

        if let Some(reference) = &evidence_reference {
            validate_identifier(
                "compliance check evidence reference",
                reference.as_ref(),
            )?;
        }

        Ok(Self {
            id,
            requirement,
            status,
            authority,
            evidence_reference,
            message,
        })
    }

    /// Return the stable check identity.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the check requirement.
    #[must_use]
    pub const fn requirement(&self) -> ComplianceRequirement {
        self.requirement
    }

    /// Return the check status.
    #[must_use]
    pub const fn status(&self) -> ComplianceStatus {
        self.status
    }

    /// Return the supplying authority.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Return the optional non-secret evidence reference.
    #[must_use]
    pub fn evidence_reference(&self) -> Option<&str> {
        self.evidence_reference.as_deref()
    }

    /// Return the explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// 9. AGGREGATED COMPLIANCE REPORT
// ============================================================================

/// Complete immutable agent compliance report.
///
/// A report describes the checks performed against one agent request.
///
/// It is not an authorization token, execution permit, feasibility
/// certificate or runtime enforcement mechanism.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentComplianceReport {
    contract_version: u32,
    request_id: Arc<str>,
    operation: Arc<str>,
    checks: Vec<AgentComplianceCheck>,
}

impl AgentComplianceReport {
    fn new(
        request_id: impl Into<Arc<str>>,
        operation: impl Into<Arc<str>>,
        checks: Vec<AgentComplianceCheck>,
    ) -> ToolchainResult<Self> {
        let request_id = request_id.into();
        let operation = operation.into();

        validate_identifier(
            "compliance request identity",
            request_id.as_ref(),
        )?;

        validate_identifier(
            "compliance operation",
            operation.as_ref(),
        )?;

        let report = Self {
            contract_version: COMPLIANCE_API_VERSION,
            request_id,
            operation,
            checks,
        };

        report.validate()?;

        Ok(report)
    }

    /// Validate the report and its individual checks.
    pub fn validate(&self) -> ToolchainResult<()> {
        if self.contract_version != COMPLIANCE_API_VERSION {
            return Err(validation_error(
                "unsupported agent compliance contract version",
            ));
        }

        validate_identifier(
            "compliance request identity",
            &self.request_id,
        )?;

        validate_identifier(
            "compliance operation",
            &self.operation,
        )?;

        for (index, check) in self.checks.iter().enumerate() {
            validate_identifier(
                "compliance check identity",
                check.id(),
            )?;

            if self.checks[..index]
                .iter()
                .any(|previous| previous.id() == check.id())
            {
                return Err(validation_error(format!(
                    "duplicate compliance check identity '{}'",
                    check.id()
                )));
            }
        }

        Ok(())
    }

    /// Return the compliance contract version.
    #[must_use]
    pub const fn contract_version(&self) -> u32 {
        self.contract_version
    }

    /// Return the associated request identity.
    #[must_use]
    pub fn request_id(&self) -> &str {
        &self.request_id
    }

    /// Return the canonical operation name.
    #[must_use]
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Return every individual compliance check.
    #[must_use]
    pub fn checks(&self) -> &[AgentComplianceCheck] {
        &self.checks
    }

    /// Return the number of compliance checks.
    #[must_use]
    pub fn check_count(&self) -> usize {
        self.checks.len()
    }

    /// Return the number of verified checks.
    #[must_use]
    pub fn verified_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| check.status().is_verified())
            .count()
    }

    /// Return the number of rejected checks.
    #[must_use]
    pub fn rejected_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| check.status().is_rejected())
            .count()
    }

    /// Return the number of unresolved checks.
    #[must_use]
    pub fn unresolved_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| check.status().is_unresolved())
            .count()
    }

    /// Return whether every mandatory check is verified.
    ///
    /// At least one mandatory check must exist.
    #[must_use]
    pub fn required_checks_verified(&self) -> bool {
        self.checks.iter().any(|check| {
            check.requirement() == ComplianceRequirement::Required
        }) && self.checks.iter().all(|check| {
            check.requirement() != ComplianceRequirement::Required
                || check.status().is_verified()
        })
    }

    /// Return the aggregate compliance status.
    ///
    /// Explicit rejection takes precedence over uncertainty.
    ///
    /// An empty report is NotEvaluated.
    #[must_use]
    pub fn status(&self) -> ComplianceStatus {
        if self.checks.is_empty() {
            return ComplianceStatus::NotEvaluated;
        }

        if self.checks.iter().any(|check| {
            check.status().is_rejected()
        }) {
            return ComplianceStatus::Rejected;
        }

        if !self.required_checks_verified() {
            if self.checks.iter().any(|check| {
                check.requirement() == ComplianceRequirement::Required
                    && check.status() == ComplianceStatus::NotEvaluated
            }) {
                return ComplianceStatus::NotEvaluated;
            }

            return ComplianceStatus::Indeterminate;
        }

        ComplianceStatus::Verified
    }

    /// Return whether the report establishes compliance.
    ///
    /// Acceptance requires:
    ///
    /// - At least one mandatory check.
    /// - Every mandatory check verified.
    /// - No rejected checks, including advisory checks.
    ///
    /// Authorization is one of the mandatory built-in checks.
    #[must_use]
    pub fn can_proceed(&self) -> bool {
        self.required_checks_verified()
            && self.rejected_count() == 0
    }

    /// Return whether every configured check has a definitive outcome.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.checks.iter().all(|check| {
            !check.status().is_unresolved()
        })
    }

    /// Return whether unresolved compliance checks remain.
    #[must_use]
    pub fn has_unresolved_checks(&self) -> bool {
        self.unresolved_count() != 0
    }
}

// ============================================================================
// 10. COMPLIANCE EVALUATOR
// ============================================================================

/// Agent compliance orchestration service.
///
/// The evaluator reuses canonical request, result, command and
/// authorization contracts.
///
/// It does not implement another policy engine.
///
/// Extension controls are supplied explicitly during construction.
/// There is no implicit allow-all or no-op security configuration.
#[derive(Clone)]
pub struct AgentComplianceEvaluator {
    controls: Vec<Arc<dyn AgentComplianceControl>>,
}

impl fmt::Debug for AgentComplianceEvaluator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("AgentComplianceEvaluator")
            .field("control_count", &self.controls.len())
            .finish_non_exhaustive()
    }
}

impl AgentComplianceEvaluator {
    /// Construct an agent compliance evaluator.
    ///
    /// Control identities must be valid and unique.
    ///
    /// Built-in check identities are reserved.
    ///
    /// An empty extension list is permitted because the mandatory
    /// built-in request and authorization checks remain active.
    pub fn new(
        controls: Vec<Arc<dyn AgentComplianceControl>>,
    ) -> ToolchainResult<Self> {
        let mut identities = HashSet::new();

        identities
            .try_reserve(controls.len())
            .map_err(|error| {
                allocation_error("control identities", error)
            })?;

        for control in &controls {
            let id = control.id();

            validate_identifier(
                "agent compliance control identity",
                id,
            )?;

            if [
                check_ids::REQUEST_VALIDATION,
                check_ids::AUTHORIZATION,
                check_ids::RESULT_VALIDATION,
            ]
            .contains(&id)
            {
                return Err(validation_error(format!(
                    "compliance control identity '{}' is reserved",
                    id
                )));
            }

            if !identities.insert(id) {
                return Err(validation_error(format!(
                    "duplicate agent compliance control '{}'",
                    id
                )));
            }
        }

        Ok(Self { controls })
    }

    /// Return the number of registered compliance controls.
    #[must_use]
    pub fn control_count(&self) -> usize {
        self.controls.len()
    }

    /// Evaluate an agent workflow.
    ///
    /// `descriptor` must be obtained from the canonical command
    /// registry, not constructed from untrusted agent metadata.
    ///
    /// `authorization` must be the receipt returned by the canonical
    /// AgentPermissionAuthorizer.
    ///
    /// The optional result must belong to the supplied request.
    ///
    /// The evaluator performs no command execution, audit persistence,
    /// deployment or external side effects.
    ///
    /// Controls execute only after the request has been validated
    /// and explicit authorization has been established.
    pub fn evaluate(
        &self,
        request: &AgentRequest,
        descriptor: &CommandDescriptor,
        authorization: Option<&AuthorizationReceipt>,
        result: Option<&AgentResult>,
    ) -> ToolchainResult<AgentComplianceReport> {
        let capacity = self.controls.len()
            .checked_add(if result.is_some() { 3 } else { 2 })
            .ok_or_else(|| {
                validation_error(
                    "compliance report capacity calculation overflowed",
                )
            })?;

        let mut checks = Vec::new();

        checks
            .try_reserve(capacity)
            .map_err(|error| {
                allocation_error("report checks", error)
            })?;

        // ------------------------------------------------------------
        // REQUEST VALIDATION
        // ------------------------------------------------------------

        let request_valid = request.validate().is_ok();

        checks.push(AgentComplianceCheck::new(
            check_ids::REQUEST_VALIDATION,
            ComplianceRequirement::Required,
            if request_valid {
                ComplianceStatus::Verified
            } else {
                ComplianceStatus::Rejected
            },
            "toolchain.agents.requests",
            None,
            if request_valid {
                "canonical agent request and context are valid"
            } else {
                "agent request or associated context failed validation"
            },
        )?);

        // ------------------------------------------------------------
        // AUTHORIZATION AND COMMAND BINDING
        // ------------------------------------------------------------

        let authorization_status = match authorization {
            None => ComplianceStatus::NotEvaluated,

            Some(receipt) => {
                if descriptor.api_version == 0 {
                    ComplianceStatus::Rejected
                } else if receipt.command() != descriptor.id.as_str() {
                    ComplianceStatus::Rejected
                } else if request.correlation_id().is_some()
                    && receipt.correlation_id()
                        != request.correlation_id()
                {
                    ComplianceStatus::Rejected
                } else {
                    match receipt.effect() {
                        DecisionEffect::Permit => {
                            ComplianceStatus::Verified
                        }

                        DecisionEffect::Deny
                        | DecisionEffect::NotApplicable => {
                            ComplianceStatus::Rejected
                        }

                        DecisionEffect::Indeterminate => {
                            ComplianceStatus::Indeterminate
                        }
                    }
                }
            }
        };

        let authorization_message = match authorization_status {
            ComplianceStatus::Verified => {
                "canonical authorization explicitly permitted the matching command"
            }

            ComplianceStatus::Rejected => {
                "authorization was denied or did not match the requested command"
            }

            ComplianceStatus::Indeterminate => {
                "authorization could not establish permission"
            }

            ComplianceStatus::NotEvaluated => {
                "authorization evidence has not been supplied"
            }
        };

        let authorization_reference = authorization
            .and_then(|receipt| {
                receipt.authentication_reference()
            })
            .map(Arc::<str>::from);

        checks.push(AgentComplianceCheck::new(
            check_ids::AUTHORIZATION,
            ComplianceRequirement::Required,
            authorization_status,
            "toolchain.agents.permissions",
            authorization_reference,
            authorization_message,
        )?);

        // ------------------------------------------------------------
        // RESULT VALIDATION
        // ------------------------------------------------------------

        if let Some(result) = result {
            let result_valid = result.validate().is_ok()
                && result.response().request_id == request.request_id();

            checks.push(AgentComplianceCheck::new(
                check_ids::RESULT_VALIDATION,
                ComplianceRequirement::Required,
                if result_valid {
                    ComplianceStatus::Verified
                } else {
                    ComplianceStatus::Rejected
                },
                "toolchain.agents.results",
                None,
                if result_valid {
                    "agent result is valid and belongs to the requested operation"
                } else {
                    "agent result is invalid or belongs to a different request"
                },
            )?);
        }

        // ------------------------------------------------------------
        // ADDITIONAL AUTHORITATIVE COMPLIANCE CONTROLS
        // ------------------------------------------------------------

        let extension_prerequisites_satisfied =
            request_valid
                && authorization_status == ComplianceStatus::Verified;

        for control in &self.controls {
            let id = control.id();

            if !extension_prerequisites_satisfied {
                checks.push(AgentComplianceCheck::new(
                    id,
                    control.requirement(),
                    ComplianceStatus::NotEvaluated,
                    id,
                    None,
                    "compliance control was not evaluated because mandatory prerequisites were not satisfied",
                )?);

                continue;
            }

            let evidence = control
                .evaluate(request, result)
                .map_err(|error| {
                    ToolchainError::validation_failed(
                        format!(
                            "agent compliance control '{}' could not complete",
                            id
                        ),
                    )
                    .with_operation(EVALUATE_OPERATION)
                    .with_source(error)
                    .with_context(COMPONENT)
                })?;

            checks.push(AgentComplianceCheck::new(
                id,
                control.requirement(),
                evidence.status(),
                evidence.authority(),
                Some(Arc::from(evidence.reference())),
                evidence.message(),
            )?);
        }

        // ------------------------------------------------------------
        // FINAL REPORT
        // ------------------------------------------------------------

        AgentComplianceReport::new(
            request.request_id(),
            request.operation_name(),
            checks,
        )
    }
}

// ============================================================================
// 11. TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn check(
        id: &str,
        requirement: ComplianceRequirement,
        status: ComplianceStatus,
    ) -> AgentComplianceCheck {
        AgentComplianceCheck::new(
            id,
            requirement,
            status,
            "toolchain.test",
            None,
            "test compliance outcome",
        )
        .expect("test check must be valid")
    }

    fn report(
        checks: Vec<AgentComplianceCheck>,
    ) -> AgentComplianceReport {
        AgentComplianceReport::new(
            "test-request",
            "test.operation",
            checks,
        )
        .expect("test report must be valid")
    }

    #[test]
    fn contract_version_is_stable() {
        assert_eq!(COMPLIANCE_API_VERSION, 1);
    }

    #[test]
    fn empty_report_never_establishes_compliance() {
        let result = report(Vec::new());

        assert_eq!(
            result.status(),
            ComplianceStatus::NotEvaluated,
        );

        assert!(!result.can_proceed());
    }

    #[test]
    fn required_checks_must_be_verified() {
        let result = report(vec![
            check(
                "request",
                ComplianceRequirement::Required,
                ComplianceStatus::Verified,
            ),
            check(
                "authorization",
                ComplianceRequirement::Required,
                ComplianceStatus::Verified,
            ),
        ]);

        assert!(result.can_proceed());
        assert_eq!(
            result.status(),
            ComplianceStatus::Verified,
        );
    }

    #[test]
    fn missing_authorization_blocks_compliance() {
        let result = report(vec![
            check(
                "request",
                ComplianceRequirement::Required,
                ComplianceStatus::Verified,
            ),
            check(
                "authorization",
                ComplianceRequirement::Required,
                ComplianceStatus::NotEvaluated,
            ),
        ]);

        assert!(!result.can_proceed());

        assert_eq!(
            result.status(),
            ComplianceStatus::NotEvaluated,
        );
    }

    #[test]
    fn indeterminate_authorization_blocks_compliance() {
        let result = report(vec![
            check(
                "request",
                ComplianceRequirement::Required,
                ComplianceStatus::Verified,
            ),
            check(
                "authorization",
                ComplianceRequirement::Required,
                ComplianceStatus::Indeterminate,
            ),
        ]);

        assert!(!result.can_proceed());

        assert_eq!(
            result.status(),
            ComplianceStatus::Indeterminate,
        );
    }

    #[test]
    fn rejected_advisory_check_blocks_compliance() {
        let result = report(vec![
            check(
                "request",
                ComplianceRequirement::Required,
                ComplianceStatus::Verified,
            ),
            check(
                "authorization",
                ComplianceRequirement::Required,
                ComplianceStatus::Verified,
            ),
            check(
                "advisory.policy",
                ComplianceRequirement::Advisory,
                ComplianceStatus::Rejected,
            ),
        ]);

        assert!(!result.can_proceed());

        assert_eq!(
            result.status(),
            ComplianceStatus::Rejected,
        );
    }

    #[test]
    fn unresolved_advisory_check_does_not_hide_required_success() {
        let result = report(vec![
            check(
                "request",
                ComplianceRequirement::Required,
                ComplianceStatus::Verified,
            ),
            check(
                "authorization",
                ComplianceRequirement::Required,
                ComplianceStatus::Verified,
            ),
            check(
                "advisory.metadata",
                ComplianceRequirement::Advisory,
                ComplianceStatus::Indeterminate,
            ),
        ]);

        assert!(result.can_proceed());

        assert!(result.has_unresolved_checks());

        assert_eq!(
            result.status(),
            ComplianceStatus::Verified,
        );
    }

    #[test]
    fn duplicate_check_identities_are_rejected() {
        let result = AgentComplianceReport::new(
            "test-request",
            "test.operation",
            vec![
                check(
                    "authorization",
                    ComplianceRequirement::Required,
                    ComplianceStatus::Verified,
                ),
                check(
                    "authorization",
                    ComplianceRequirement::Required,
                    ComplianceStatus::Verified,
                ),
            ],
        );

        assert!(result.is_err());
    }

    #[test]
    fn evidence_requires_a_valid_authority() {
        assert!(
            AgentComplianceEvidence::new(
                "",
                "test-reference",
                ComplianceStatus::Verified,
                "test evidence",
            )
            .is_err()
        );
    }

    #[test]
    fn evidence_requires_a_valid_reference() {
        assert!(
            AgentComplianceEvidence::new(
                "test.authority",
                "",
                ComplianceStatus::Verified,
                "test evidence",
            )
            .is_err()
        );
    }

    #[test]
    fn control_statuses_never_confuse_rejection_with_verification() {
        assert!(
            !ComplianceStatus::Rejected.is_verified()
        );

        assert!(
            !ComplianceStatus::Indeterminate.is_verified()
        );

        assert!(
            !ComplianceStatus::NotEvaluated.is_verified()
        );
    }

    #[test]
    fn report_counts_are_consistent() {
        let result = report(vec![
            check(
                "one",
                ComplianceRequirement::Required,
                ComplianceStatus::Verified,
            ),
            check(
                "two",
                ComplianceRequirement::Required,
                ComplianceStatus::Rejected,
            ),
            check(
                "three",
                ComplianceRequirement::Advisory,
                ComplianceStatus::NotEvaluated,
            ),
        ]);

        assert_eq!(result.check_count(), 3);
        assert_eq!(result.verified_count(), 1);
        assert_eq!(result.rejected_count(), 1);
        assert_eq!(result.unresolved_count(), 1);

        assert!(!result.can_proceed());
    }

    #[test]
    fn compliance_contract_uses_open_world_check_identities() {
        for id in [
            "classical.execution",
            "quantum.execution",
            "hdl.synthesis",
            "ai.training",
            "hybrid.execution",
            "distributed.coordination",
            "future.domain.compliance",
        ] {
            assert!(
                AgentComplianceCheck::new(
                    id,
                    ComplianceRequirement::Required,
                    ComplianceStatus::Verified,
                    "test.authority",
                    None,
                    "test verification",
                )
                .is_ok()
            );
        }
    }
}
