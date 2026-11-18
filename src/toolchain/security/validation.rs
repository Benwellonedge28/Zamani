
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Security Validation Coordinator
//!
//! Path: src/toolchain/security/validation.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Mission
//!
//! Coordinate security validation before sensitive toolchain operations.
//!
//! The coordinator integrates:
//!
//! - Authorization policy evaluation.
//! - Trust assessment.
//! - Digital signature verification through the trust authority.
//! - Content-integrity verification.
//! - Lifecycle provenance verification.
//! - Extensible security validation authorities.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Security validation requests.
//! - Security check identities.
//! - Validation requirements.
//! - Validation outcomes.
//! - Validation reports.
//! - Cross-subsystem security validation coordination.
//! - Security validation extension contracts.
//! - Aggregate fail-closed acceptance decisions.
//!
//! This module does not own:
//!
//! - Authentication.
//! - Authorization policy definitions.
//! - Cryptographic algorithms.
//! - Key management.
//! - Signature implementations.
//! - Hashing implementations.
//! - Trust-store persistence.
//! - Revocation infrastructure.
//! - Provenance generation.
//! - Package archive parsing.
//! - Registry communication.
//! - External process execution.
//! - Hardware discovery.
//! - Resource allocation.
//! - Runtime isolation.
//! - Runtime enforcement.
//! - Compiler semantic analysis.
//!
//! ## Authoritative integration
//!
//! Authorization:
//!     crate::toolchain::security::policy
//!
//! Trust:
//!     crate::toolchain::security::trust
//!
//! Signatures:
//!     crate::toolchain::security::signatures
//!
//! Integrity:
//!     crate::toolchain::security::integrity
//!
//! Provenance:
//!     crate::toolchain::security::provenance
//!
//! Cryptography:
//!     crate::stdlib::security
//!
//! Compiler security:
//!     crate::compiler::security
//!
//! Backend security:
//!     crate::backend::security
//!
//! Hardware security:
//!     crate::zhal::security
//!
//! Runtime security:
//!     crate::zur::security
//!
//! Normative semantics:
//!     grammar/spec/security.md
//!     grammar/spec/provenance.md
//!
//! Security intent:
//!     grammar/security/*.g4
//!
//! ## Security invariants
//!
//! 1. No checks means no authorization to proceed.
//! 2. Unknown evidence never establishes security.
//! 3. Unverified evidence never establishes trust.
//! 4. Missing required checks never establish permission.
//! 5. An explicit security rejection always blocks acceptance.
//! 6. An indeterminate mandatory check blocks acceptance.
//! 7. Policy evaluation must produce an explicit permit.
//! 8. Trust requires an affirmative trust assessment.
//! 9. Integrity requires an actual digest verification.
//! 10. Provenance requires an actual fingerprint comparison.
//! 11. Extension results must identify their authority and evidence.
//! 12. Provider errors never become successful verification.
//! 13. Operational resource exhaustion never becomes permission.
//! 14. Validation does not itself enforce execution isolation.
//! 15. Security decisions remain separate from hardware selection.
//!
//! ## Scalability
//!
//! No arbitrary universal maximum is imposed on:
//!
//! - Security checks.
//! - Security authorities.
//! - Trust evidence.
//! - Providers.
//! - Packages.
//! - Artifacts.
//! - Targets.
//! - Computational domains.
//! - Hardware resources.
//! - Distributed participants.
//!
//! Collections grow according to supplied data and available resources.
//!
//! Operational budgets belong to the individual authoritative checks.
//! An exhausted budget must not produce a successful security result.
//!
//! Literal infinite processing, memory or storage is not promised.
//!
//! ## Determinism
//!
//! Built-in validation checks execute in request order.
//! Their reports preserve that order.
//!
//! Determinism of individual results remains the responsibility of the
//! authoritative security subsystem.
//!
//! Time-sensitive checks receive explicit timestamps.
//! This module does not silently read the system clock.
//!
//! ## Important integration rule
//!
//! A registered extension is trusted executable integration code.
//! Its implementation must delegate to the owning security authority.
//! Merely implementing this trait does not independently establish
//! the authenticity of its output.

use std::fmt;
use std::io::Read;
use std::sync::Arc;

use crate::stdlib::provider::ProviderId;
use crate::stdlib::security::hashing::{
    HashAlgorithm,
    HashDigest,
};

use crate::toolchain::errors::{
    ToolchainError,
    ToolchainErrorKind,
    ToolchainResult,
};

use crate::toolchain::security::integrity::{
    IntegrityBudget,
    IntegrityReport,
    IntegrityStatus,
    IntegrityVerifier,
};

use crate::toolchain::security::policy::{
    DecisionEffect,
    EvaluationBudget,
    PolicyContext,
    PolicyDecision,
    PolicyEngine,
    PolicySet,
};

use crate::toolchain::security::provenance::{
    fingerprint_snapshot,
    ProvenanceSnapshot,
};

use crate::toolchain::security::trust::{
    evaluate_trust,
    RevocationChecker,
    TrustAssessment,
    TrustDecision,
    TrustEvidence,
    TrustPolicy,
    TrustTimestamp,
    SignatureVerifier,
};

use crate::zuir::fingerprint::stable_hash::Fingerprint;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Stable component identity.
pub const COMPONENT: &str =
    "toolchain.security.validation";

/// Stable security-validation operation identity.
pub const VALIDATE_OPERATION: &str =
    "toolchain.security.validate";

/// Security validation API version.
pub const API_VERSION: u16 = 1;

// ============================================================================
// ERROR HANDLING
// ============================================================================

fn validation_error(
    kind: ToolchainErrorKind,
    message: impl Into<String>,
) -> ToolchainError {
    ToolchainError::new(kind, message)
        .with_operation(VALIDATE_OPERATION)
        .with_context(COMPONENT)
}

fn invalid_input(
    message: impl Into<String>,
) -> ToolchainError {
    validation_error(
        ToolchainErrorKind::InvalidInput,
        message,
    )
}

fn invalid_configuration(
    message: impl Into<String>,
) -> ToolchainError {
    validation_error(
        ToolchainErrorKind::InvalidConfiguration,
        message,
    )
}

fn validation_failure(
    message: impl Into<String>,
) -> ToolchainError {
    validation_error(
        ToolchainErrorKind::ValidationFailed,
        message,
    )
}

/// Validate an open-world identifier.
///
/// No arbitrary identifier length, namespace depth or domain count
/// is imposed by this module.
fn validate_identifier(
    value: &str,
    description: &'static str,
) -> ToolchainResult<()> {
    if value.trim().is_empty() {
        return Err(invalid_input(
            format!("{description} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_input(
            format!("{description} contains control characters"),
        ));
    }

    Ok(())
}

// ============================================================================
// IDENTIFIERS
// ============================================================================

/// Stable identity of one security validation check.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct SecurityCheckId(Arc<str>);

impl SecurityCheckId {
    /// Construct a validated security check identity.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> ToolchainResult<Self> {
        let value = value.into();

        validate_identifier(
            value.as_ref(),
            "security check identity",
        )?;

        Ok(Self(value))
    }

    /// Return the exact identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SecurityCheckId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// ============================================================================
// CHECK REQUIREMENTS
// ============================================================================

/// Importance of a security validation check.
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
pub enum CheckRequirement {
    /// The check must be affirmatively verified before proceeding.
    Required,

    /// The check is advisory but its result remains visible.
    ///
    /// An explicit rejection still blocks the complete validation.
    Advisory,
}

// ============================================================================
// VALIDATION OUTCOMES
// ============================================================================

/// Result of an individual security validation.
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
pub enum ValidationStatus {
    /// The authoritative authority verified the requirement.
    Verified,

    /// The authoritative authority established a violation.
    Rejected,

    /// Available evidence could not establish a conclusion.
    Indeterminate,

    /// The check was not performed.
    NotEvaluated,
}

impl ValidationStatus {
    /// Return whether this status establishes affirmative verification.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }

    /// Return whether this status explicitly rejects the requirement.
    #[must_use]
    pub const fn is_rejected(self) -> bool {
        matches!(self, Self::Rejected)
    }

    /// Return whether the status requires additional evidence.
    #[must_use]
    pub const fn is_unresolved(self) -> bool {
        matches!(
            self,
            Self::Indeterminate | Self::NotEvaluated
        )
    }
}

// ============================================================================
// CHECK CATEGORIES
// ============================================================================

/// Classification of a security validation check.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SecurityCheckKind {
    /// Authorization policy validation.
    Authorization,

    /// Trust evidence validation.
    Trust,

    /// Digital signature validation through the trust authority.
    Signature,

    /// Content-integrity validation.
    Integrity,

    /// Lifecycle provenance validation.
    Provenance,

    /// Extension-defined validation.
    Extension(Arc<str>),
}

// ============================================================================
// INDIVIDUAL CHECK REPORT
// ============================================================================

/// Immutable result of one security validation check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityCheckReport {
    id: SecurityCheckId,
    kind: SecurityCheckKind,
    requirement: CheckRequirement,
    status: ValidationStatus,
    authority: Arc<str>,
    evidence_reference: Option<Arc<str>>,
    message: Arc<str>,
}

impl SecurityCheckReport {
    fn new(
        id: SecurityCheckId,
        kind: SecurityCheckKind,
        requirement: CheckRequirement,
        status: ValidationStatus,
        authority: impl Into<Arc<str>>,
        evidence_reference: Option<Arc<str>>,
        message: impl Into<Arc<str>>,
    ) -> ToolchainResult<Self> {
        let authority = authority.into();
        let message = message.into();

        validate_identifier(
            authority.as_ref(),
            "security authority identity",
        )?;

        validate_identifier(
            message.as_ref(),
            "security validation message",
        )?;

        if let Some(reference) = &evidence_reference {
            validate_identifier(
                reference.as_ref(),
                "security evidence reference",
            )?;
        }

        Ok(Self {
            id,
            kind,
            requirement,
            status,
            authority,
            evidence_reference,
            message,
        })
    }

    /// Return the check identity.
    #[must_use]
    pub fn id(&self) -> &SecurityCheckId {
        &self.id
    }

    /// Return the check category.
    #[must_use]
    pub fn kind(&self) -> &SecurityCheckKind {
        &self.kind
    }

    /// Return the check requirement.
    #[must_use]
    pub const fn requirement(&self) -> CheckRequirement {
        self.requirement
    }

    /// Return the validation status.
    #[must_use]
    pub const fn status(&self) -> ValidationStatus {
        self.status
    }

    /// Return the authoritative implementation identity.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Return the external evidence reference, if available.
    #[must_use]
    pub fn evidence_reference(&self) -> Option<&str> {
        self.evidence_reference.as_deref()
    }

    /// Return a safe explanatory message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

// ============================================================================
// SPECIALIZED VALIDATION INPUTS
// ============================================================================

/// One authorization-policy validation request.
pub struct AuthorizationCheck<'a> {
    id: SecurityCheckId,
    requirement: CheckRequirement,
    policies: &'a PolicySet,
    context: &'a PolicyContext,
    budget: EvaluationBudget,
}

impl<'a> AuthorizationCheck<'a> {
    /// Construct an authorization check.
    pub fn new(
        id: SecurityCheckId,
        requirement: CheckRequirement,
        policies: &'a PolicySet,
        context: &'a PolicyContext,
        budget: EvaluationBudget,
    ) -> Self {
        Self {
            id,
            requirement,
            policies,
            context,
            budget,
        }
    }
}

/// One trust-evidence validation request.
pub struct TrustCheck<'a> {
    id: SecurityCheckId,
    requirement: CheckRequirement,
    policy: &'a TrustPolicy,
    evidence: &'a TrustEvidence,
    expected_provider: &'a ProviderId,
    evaluated_at: TrustTimestamp,
    signature_verifier: Option<&'a dyn SignatureVerifier>,
    revocation_checker: Option<&'a dyn RevocationChecker>,
}

impl<'a> TrustCheck<'a> {
    /// Construct a trust validation request.
    ///
    /// The timestamp is explicit to preserve deterministic evaluation.
    pub fn new(
        id: SecurityCheckId,
        requirement: CheckRequirement,
        policy: &'a TrustPolicy,
        evidence: &'a TrustEvidence,
        expected_provider: &'a ProviderId,
        evaluated_at: TrustTimestamp,
        signature_verifier: Option<&'a dyn SignatureVerifier>,
        revocation_checker: Option<&'a dyn RevocationChecker>,
    ) -> Self {
        Self {
            id,
            requirement,
            policy,
            evidence,
            expected_provider,
            evaluated_at,
            signature_verifier,
            revocation_checker,
        }
    }
}

/// One streaming content-integrity validation request.
pub struct IntegrityCheck<'a> {
    id: SecurityCheckId,
    requirement: CheckRequirement,
    algorithm: HashAlgorithm,
    input: &'a mut dyn Read,
    expected: HashDigest,
    budget: IntegrityBudget,
}

impl<'a> IntegrityCheck<'a> {
    /// Construct an integrity validation request.
    ///
    /// The input is consumed incrementally by the canonical integrity
    /// verifier. Its complete contents are not accumulated here.
    pub fn new(
        id: SecurityCheckId,
        requirement: CheckRequirement,
        algorithm: HashAlgorithm,
        input: &'a mut dyn Read,
        expected: HashDigest,
        budget: IntegrityBudget,
    ) -> Self {
        Self {
            id,
            requirement,
            algorithm,
            input,
            expected,
            budget,
        }
    }
}

/// One lifecycle provenance validation request.
pub struct ProvenanceCheck<'a> {
    id: SecurityCheckId,
    requirement: CheckRequirement,
    snapshot: &'a ProvenanceSnapshot,
    expected_fingerprint: &'a Fingerprint,
}

impl<'a> ProvenanceCheck<'a> {
    /// Construct a provenance validation request.
    ///
    /// The expected fingerprint must originate from an independently
    /// established, trusted reference.
    pub fn new(
        id: SecurityCheckId,
        requirement: CheckRequirement,
        snapshot: &'a ProvenanceSnapshot,
        expected_fingerprint: &'a Fingerprint,
    ) -> Self {
        Self {
            id,
            requirement,
            snapshot,
            expected_fingerprint,
        }
    }
}

// ============================================================================
// EXTENSION AUTHORITIES
// ============================================================================

/// Verified evidence returned by an integrated security authority.
///
/// An extension must not report Verified merely because it was registered,
/// discovered or declared a capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionEvidence {
    authority: Arc<str>,
    evidence_reference: Arc<str>,
    status: ValidationStatus,
    message: Arc<str>,
}

impl ExtensionEvidence {
    /// Construct an extension evidence result.
    ///
    /// The caller is responsible for obtaining the result from the
    /// authoritative security implementation.
    pub fn new(
        authority: impl Into<Arc<str>>,
        evidence_reference: impl Into<Arc<str>>,
        status: ValidationStatus,
        message: impl Into<Arc<str>>,
    ) -> ToolchainResult<Self> {
        let authority = authority.into();
        let evidence_reference = evidence_reference.into();
        let message = message.into();

        validate_identifier(
            authority.as_ref(),
            "extension authority identity",
        )?;

        validate_identifier(
            evidence_reference.as_ref(),
            "extension evidence reference",
        )?;

        validate_identifier(
            message.as_ref(),
            "extension validation message",
        )?;

        Ok(Self {
            authority,
            evidence_reference,
            status,
            message,
        })
    }
}

/// Integration contract for additional security authorities.
///
/// Implementations should delegate to their existing authoritative
/// subsystem rather than implementing another security mechanism.
///
/// Appropriate integrations include:
///
/// - Package verification.
/// - Registry trust.
/// - External-tool verification.
/// - Environment verification.
/// - Compiler supply-chain verification.
/// - Backend provider verification.
/// - Hardware-provider trust.
/// - Runtime security.
/// - Deployment security.
/// - Agent authorization.
///
/// Implementations must not expose credentials, private keys, secrets
/// or unredacted sensitive content through returned messages.
pub trait SecurityValidationExtension: Send + Sync {
    /// Return the stable identity of this validation extension.
    fn id(&self) -> &str;

    /// Perform the authoritative security check.
    ///
    /// # Errors
    ///
    /// Return an error if the authority cannot complete the operation.
    /// An operational failure must never be represented as verification.
    fn validate(
        &self,
        operation: &str,
    ) -> ToolchainResult<ExtensionEvidence>;
}

/// One extension validation request.
pub struct ExtensionCheck<'a> {
    id: SecurityCheckId,
    requirement: CheckRequirement,
    extension: &'a dyn SecurityValidationExtension,
}

impl<'a> ExtensionCheck<'a> {
    /// Construct an extension check.
    pub fn new(
        id: SecurityCheckId,
        requirement: CheckRequirement,
        extension: &'a dyn SecurityValidationExtension,
    ) -> Self {
        Self {
            id,
            requirement,
            extension,
        }
    }
}

// ============================================================================
// VALIDATION REQUEST
// ============================================================================

/// Complete immutable description of a security validation operation.
///
/// The request references authoritative security inputs. It does not
/// duplicate the underlying policies, evidence or cryptographic services.
pub struct SecurityValidationRequest<'a> {
    operation: Arc<str>,
    authorization_checks: Vec<AuthorizationCheck<'a>>,
    trust_checks: Vec<TrustCheck<'a>>,
    integrity_checks: Vec<IntegrityCheck<'a>>,
    provenance_checks: Vec<ProvenanceCheck<'a>>,
    extension_checks: Vec<ExtensionCheck<'a>>,
}

impl<'a> SecurityValidationRequest<'a> {
    /// Construct an empty validation request.
    ///
    /// An empty request cannot establish security acceptance.
    pub fn new(
        operation: impl Into<Arc<str>>,
    ) -> ToolchainResult<Self> {
        let operation = operation.into();

        validate_identifier(
            operation.as_ref(),
            "security operation",
        )?;

        Ok(Self {
            operation,
            authorization_checks: Vec::new(),
            trust_checks: Vec::new(),
            integrity_checks: Vec::new(),
            provenance_checks: Vec::new(),
            extension_checks: Vec::new(),
        })
    }

    /// Add an authorization check.
    #[must_use]
    pub fn with_authorization(
        mut self,
        check: AuthorizationCheck<'a>,
    ) -> Self {
        self.authorization_checks.push(check);
        self
    }

    /// Add a trust check.
    #[must_use]
    pub fn with_trust(
        mut self,
        check: TrustCheck<'a>,
    ) -> Self {
        self.trust_checks.push(check);
        self
    }

    /// Add a streaming integrity check.
    #[must_use]
    pub fn with_integrity(
        mut self,
        check: IntegrityCheck<'a>,
    ) -> Self {
        self.integrity_checks.push(check);
        self
    }

    /// Add a provenance check.
    #[must_use]
    pub fn with_provenance(
        mut self,
        check: ProvenanceCheck<'a>,
    ) -> Self {
        self.provenance_checks.push(check);
        self
    }

    /// Add an extension-authority check.
    #[must_use]
    pub fn with_extension(
        mut self,
        check: ExtensionCheck<'a>,
    ) -> Self {
        self.extension_checks.push(check);
        self
    }

    /// Return the requested operation identity.
    #[must_use]
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Return the number of configured checks.
    #[must_use]
    pub fn check_count(&self) -> usize {
        self.authorization_checks.len()
            + self.trust_checks.len()
            + self.integrity_checks.len()
            + self.provenance_checks.len()
            + self.extension_checks.len()
    }

    /// Return whether the request contains no security checks.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.check_count() == 0
    }
}

// ============================================================================
// VALIDATION REPORT
// ============================================================================

/// Aggregate result of security validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityValidationReport {
    operation: Arc<str>,
    checks: Vec<SecurityCheckReport>,
    policy_decisions: Vec<PolicyDecision>,
    trust_assessments: Vec<TrustAssessment>,
    integrity_reports: Vec<IntegrityReport>,
    provenance_fingerprints: Vec<Fingerprint>,
}

impl SecurityValidationReport {
    fn new(
        operation: Arc<str>,
        checks: Vec<SecurityCheckReport>,
        policy_decisions: Vec<PolicyDecision>,
        trust_assessments: Vec<TrustAssessment>,
        integrity_reports: Vec<IntegrityReport>,
        provenance_fingerprints: Vec<Fingerprint>,
    ) -> Self {
        Self {
            operation,
            checks,
            policy_decisions,
            trust_assessments,
            integrity_reports,
            provenance_fingerprints,
        }
    }

    /// Return the validated operation.
    #[must_use]
    pub fn operation(&self) -> &str {
        &self.operation
    }

    /// Return all individual check reports.
    #[must_use]
    pub fn checks(&self) -> &[SecurityCheckReport] {
        &self.checks
    }

    /// Return policy decisions.
    #[must_use]
    pub fn policy_decisions(&self) -> &[PolicyDecision] {
        &self.policy_decisions
    }

    /// Return trust assessments.
    #[must_use]
    pub fn trust_assessments(&self) -> &[TrustAssessment] {
        &self.trust_assessments
    }

    /// Return integrity reports.
    #[must_use]
    pub fn integrity_reports(&self) -> &[IntegrityReport] {
        &self.integrity_reports
    }

    /// Return independently computed provenance fingerprints.
    #[must_use]
    pub fn provenance_fingerprints(&self) -> &[Fingerprint] {
        &self.provenance_fingerprints
    }

    /// Return the number of checks.
    #[must_use]
    pub fn check_count(&self) -> usize {
        self.checks.len()
    }

    /// Return the number of affirmatively verified checks.
    #[must_use]
    pub fn verified_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| {
                check.status.is_verified()
            })
            .count()
    }

    /// Return the number of rejected checks.
    #[must_use]
    pub fn rejected_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| {
                check.status.is_rejected()
            })
            .count()
    }

    /// Return the number of unresolved checks.
    #[must_use]
    pub fn unresolved_count(&self) -> usize {
        self.checks
            .iter()
            .filter(|check| {
                check.status.is_unresolved()
            })
            .count()
    }

    /// Return whether every mandatory check has been verified.
    #[must_use]
    pub fn required_checks_verified(&self) -> bool {
        self.checks.iter().any(|check| {
            check.requirement == CheckRequirement::Required
        }) && self.checks.iter().all(|check| {
            check.requirement != CheckRequirement::Required
                || check.status == ValidationStatus::Verified
        })
    }

    /// Return the aggregate validation status.
    ///
    /// Rejection has precedence over uncertainty.
    /// An empty report is indeterminate, never verified.
    #[must_use]
    pub fn status(&self) -> ValidationStatus {
        if self.checks.is_empty() {
            return ValidationStatus::NotEvaluated;
        }

        if self.checks.iter().any(|check| {
            check.status == ValidationStatus::Rejected
        }) {
            return ValidationStatus::Rejected;
        }

        if !self.required_checks_verified() {
            if self.checks.iter().any(|check| {
                check.requirement == CheckRequirement::Required
                    && check.status == ValidationStatus::NotEvaluated
            }) {
                return ValidationStatus::NotEvaluated;
            }

            return ValidationStatus::Indeterminate;
        }

        ValidationStatus::Verified
    }

    /// Return whether the operation has satisfied its security checks.
    ///
    /// This is a validation decision, not runtime authorization enforcement.
    ///
    /// At least one mandatory check must exist.
    /// Every mandatory check must be verified.
    /// Any explicit rejection blocks acceptance.
    #[must_use]
    pub fn can_proceed(&self) -> bool {
        self.required_checks_verified()
            && self.rejected_count() == 0
    }

    /// Return whether the report contains unresolved checks.
    #[must_use]
    pub fn has_unresolved_checks(&self) -> bool {
        self.unresolved_count() != 0
    }

    /// Return whether all configured checks were completed.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.checks.iter().all(|check| {
            check.status != ValidationStatus::NotEvaluated
                && check.status != ValidationStatus::Indeterminate
        })
    }
}

// ============================================================================
// VALIDATION ENGINE
// ============================================================================

/// Stateless security validation coordinator.
#[derive(Debug, Clone, Default)]
pub struct SecurityValidator;

impl SecurityValidator {
    /// Construct a security validator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Return the security validation API version.
    #[must_use]
    pub const fn api_version() -> u16 {
        API_VERSION
    }

    /// Validate every configured security check.
    ///
    /// All built-in authorities are invoked through their canonical APIs.
    ///
    /// Checks are evaluated sequentially and in request order.
    /// No worker count, provider count or hardware count is hard-coded.
    ///
    /// A failed operational check returns an error. It never creates a
    /// successful aggregate report.
    ///
    /// A completed report can still be rejected or indeterminate.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid requests, malformed evidence,
    /// invalid configuration, failed integrity operations,
    /// failed provenance fingerprinting, or extension failures.
    pub fn validate(
        &self,
        request: SecurityValidationRequest<'_>,
    ) -> ToolchainResult<SecurityValidationReport> {
        validate_identifier(
            request.operation.as_ref(),
            "security operation",
        )?;

        let mut checks = Vec::new();
        let mut policy_decisions = Vec::new();
        let mut trust_assessments = Vec::new();
        let mut integrity_reports = Vec::new();
        let mut provenance_fingerprints = Vec::new();

        // ------------------------------------------------------------
        // AUTHORIZATION
        // ------------------------------------------------------------

        let policy_engine = PolicyEngine::new();

        for check in request.authorization_checks {
            let decision = policy_engine
                .evaluate(
                    check.policies,
                    check.context,
                    &check.budget,
                )
                .map_err(|error| {
                    validation_failure(
                        "authorization policy evaluation could not complete",
                    )
                    .with_source(error)
                })?;

            let status = match decision.effect() {
                DecisionEffect::Permit => {
                    ValidationStatus::Verified
                }

                DecisionEffect::Deny
                | DecisionEffect::NotApplicable => {
                    ValidationStatus::Rejected
                }

                DecisionEffect::Indeterminate => {
                    ValidationStatus::Indeterminate
                }
            };

            let message = match status {
                ValidationStatus::Verified => {
                    "authorization policy explicitly permitted the operation"
                }

                ValidationStatus::Rejected => {
                    "authorization policy did not permit the operation"
                }

                ValidationStatus::Indeterminate => {
                    "authorization policy evaluation was indeterminate"
                }

                ValidationStatus::NotEvaluated => {
                    "authorization policy was not evaluated"
                }
            };

            checks.push(SecurityCheckReport::new(
                check.id,
                SecurityCheckKind::Authorization,
                check.requirement,
                status,
                "toolchain.security.policy",
                None,
                message,
            )?);

            policy_decisions.push(decision);
        }

        // ------------------------------------------------------------
        // TRUST AND SIGNATURE VALIDATION
        // ------------------------------------------------------------

        for check in request.trust_checks {
            let assessment = evaluate_trust(
                check.policy,
                check.evidence,
                check.expected_provider,
                check.evaluated_at,
                check.signature_verifier,
                check.revocation_checker,
            )
            .map_err(|error| {
                validation_failure(
                    "trust evidence assessment could not complete",
                )
                .with_source(error)
            })?;

            let status = match assessment.decision() {
                TrustDecision::Trusted => {
                    ValidationStatus::Verified
                }

                TrustDecision::Rejected => {
                    ValidationStatus::Rejected
                }

                TrustDecision::Indeterminate => {
                    ValidationStatus::Indeterminate
                }
            };

            let message = match status {
                ValidationStatus::Verified => {
                    "trust authority established affirmative trust"
                }

                ValidationStatus::Rejected => {
                    "trust authority rejected the evidence"
                }

                ValidationStatus::Indeterminate => {
                    "trust authority could not establish trust"
                }

                ValidationStatus::NotEvaluated => {
                    "trust assessment was not evaluated"
                }
            };

            checks.push(SecurityCheckReport::new(
                check.id,
                SecurityCheckKind::Trust,
                check.requirement,
                status,
                "toolchain.security.trust",
                Some(Arc::from(
                    assessment.evidence().as_str(),
                )),
                message,
            )?);

            trust_assessments.push(assessment);
        }

        // ------------------------------------------------------------
        // CONTENT INTEGRITY
        // ------------------------------------------------------------

        let integrity_verifier = IntegrityVerifier::new();

        for check in request.integrity_checks {
            let report = integrity_verifier
                .verify(
                    &check.algorithm,
                    check.input,
                    &check.expected,
                    check.budget,
                )
                .map_err(|error| {
                    validation_failure(
                        "content-integrity verification could not complete",
                    )
                    .with_source(error)
                })?;

            let status = match report.status() {
                IntegrityStatus::Verified => {
                    ValidationStatus::Verified
                }

                IntegrityStatus::Mismatch => {
                    ValidationStatus::Rejected
                }
            };

            let message = match status {
                ValidationStatus::Verified => {
                    "content digest matches the expected digest"
                }

                ValidationStatus::Rejected => {
                    "content digest does not match the expected digest"
                }

                ValidationStatus::Indeterminate => {
                    "integrity verification was indeterminate"
                }

                ValidationStatus::NotEvaluated => {
                    "integrity verification was not evaluated"
                }
            };

            checks.push(SecurityCheckReport::new(
                check.id,
                SecurityCheckKind::Integrity,
                check.requirement,
                status,
                "toolchain.security.integrity",
                None,
                message,
            )?);

            integrity_reports.push(report);
        }

        // ------------------------------------------------------------
        // LIFECYCLE PROVENANCE
        // ------------------------------------------------------------

        for check in request.provenance_checks {
            let actual = fingerprint_snapshot(check.snapshot)
                .map_err(|error| {
                    validation_failure(
                        "lifecycle provenance fingerprinting failed",
                    )
                    .with_source(error)
                })?;

            let status = if &actual == check.expected_fingerprint {
                ValidationStatus::Verified
            } else {
                ValidationStatus::Rejected
            };

            let message = match status {
                ValidationStatus::Verified => {
                    "lifecycle provenance fingerprint matches the trusted reference"
                }

                ValidationStatus::Rejected => {
                    "lifecycle provenance fingerprint differs from the trusted reference"
                }

                ValidationStatus::Indeterminate => {
                    "provenance verification was indeterminate"
                }

                ValidationStatus::NotEvaluated => {
                    "provenance verification was not evaluated"
                }
            };

            checks.push(SecurityCheckReport::new(
                check.id,
                SecurityCheckKind::Provenance,
                check.requirement,
                status,
                "toolchain.security.provenance",
                None,
                message,
            )?);

            provenance_fingerprints.push(actual);
        }

        // ------------------------------------------------------------
        // EXTENSIBLE AUTHORITIES
        // ------------------------------------------------------------

        for check in request.extension_checks {
            validate_identifier(
                check.extension.id(),
                "security extension identity",
            )?;

            let evidence = check.extension
                .validate(request.operation.as_ref())
                .map_err(|error| {
                    validation_failure(
                        "integrated security authority could not complete validation",
                    )
                    .with_source(error)
                })?;

            let extension_id: Arc<str> =
                Arc::from(check.extension.id());

            checks.push(SecurityCheckReport::new(
                check.id,
                SecurityCheckKind::Extension(extension_id),
                check.requirement,
                evidence.status,
                evidence.authority,
                Some(evidence.evidence_reference),
                evidence.message,
            )?);
        }

        // ------------------------------------------------------------
        // AGGREGATE
        // ------------------------------------------------------------

        Ok(SecurityValidationReport::new(
            request.operation,
            checks,
            policy_decisions,
            trust_assessments,
            integrity_reports,
            provenance_fingerprints,
        ))
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn check_id(value: &str) -> SecurityCheckId {
        SecurityCheckId::new(value)
            .expect("test identity must be valid")
    }

    fn report(
        checks: Vec<SecurityCheckReport>,
    ) -> SecurityValidationReport {
        SecurityValidationReport::new(
            Arc::from("test.operation"),
            checks,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    fn check(
        id: &str,
        requirement: CheckRequirement,
        status: ValidationStatus,
    ) -> SecurityCheckReport {
        SecurityCheckReport::new(
            check_id(id),
            SecurityCheckKind::Authorization,
            requirement,
            status,
            "test.authority",
            None,
            "test validation result",
        )
        .expect("test report must be valid")
    }

    #[test]
    fn api_version_is_stable() {
        assert_eq!(
            SecurityValidator::api_version(),
            API_VERSION,
        );
    }

    #[test]
    fn empty_validation_request_is_not_accepted() {
        let request = SecurityValidationRequest::new(
            "package.publish",
        )
        .expect("operation must be valid");

        assert!(request.is_empty());

        let result = SecurityValidator::new()
            .validate(request)
            .expect("empty validation produces a report");

        assert_eq!(
            result.status(),
            ValidationStatus::NotEvaluated,
        );

        assert!(!result.can_proceed());
    }

    #[test]
    fn empty_operation_is_rejected() {
        assert!(
            SecurityValidationRequest::new("").is_err()
        );
    }

    #[test]
    fn control_characters_are_rejected_in_identifiers() {
        assert!(
            SecurityCheckId::new("invalid\nidentity").is_err()
        );
    }

    #[test]
    fn explicit_rejection_always_blocks_acceptance() {
        let result = report(vec![
            check(
                "required.integrity",
                CheckRequirement::Required,
                ValidationStatus::Verified,
            ),
            check(
                "advisory.trust",
                CheckRequirement::Advisory,
                ValidationStatus::Rejected,
            ),
        ]);

        assert_eq!(
            result.status(),
            ValidationStatus::Rejected,
        );

        assert!(!result.can_proceed());
    }

    #[test]
    fn missing_required_verification_blocks_acceptance() {
        let result = report(vec![
            check(
                "required.policy",
                CheckRequirement::Required,
                ValidationStatus::Verified,
            ),
            check(
                "required.provenance",
                CheckRequirement::Required,
                ValidationStatus::Indeterminate,
            ),
        ]);

        assert_eq!(
            result.status(),
            ValidationStatus::Indeterminate,
        );

        assert!(!result.can_proceed());
    }

    #[test]
    fn not_evaluated_required_check_blocks_acceptance() {
        let result = report(vec![
            check(
                "required.authorization",
                CheckRequirement::Required,
                ValidationStatus::Verified,
            ),
            check(
                "required.registry",
                CheckRequirement::Required,
                ValidationStatus::NotEvaluated,
            ),
        ]);

        assert_eq!(
            result.status(),
            ValidationStatus::NotEvaluated,
        );

        assert!(!result.can_proceed());
    }

    #[test]
    fn all_required_checks_must_be_verified() {
        let result = report(vec![
            check(
                "required.policy",
                CheckRequirement::Required,
                ValidationStatus::Verified,
            ),
            check(
                "required.integrity",
                CheckRequirement::Required,
                ValidationStatus::Verified,
            ),
            check(
                "required.trust",
                CheckRequirement::Required,
                ValidationStatus::Verified,
            ),
        ]);

        assert_eq!(
            result.status(),
            ValidationStatus::Verified,
        );

        assert!(result.can_proceed());
    }

    #[test]
    fn advisory_indeterminacy_does_not_hide_required_success() {
        let result = report(vec![
            check(
                "required.integrity",
                CheckRequirement::Required,
                ValidationStatus::Verified,
            ),
            check(
                "advisory.metadata",
                CheckRequirement::Advisory,
                ValidationStatus::Indeterminate,
            ),
        ]);

        assert_eq!(
            result.status(),
            ValidationStatus::Verified,
        );

        assert!(result.can_proceed());
        assert!(result.has_unresolved_checks());
    }

    #[test]
    fn advisory_only_validation_cannot_authorize_an_operation() {
        let result = report(vec![
            check(
                "advisory.metadata",
                CheckRequirement::Advisory,
                ValidationStatus::Verified,
            ),
        ]);

        assert!(!result.can_proceed());

        assert_eq!(
            result.status(),
            ValidationStatus::Indeterminate,
        );
    }

    #[test]
    fn rejected_count_is_accurate() {
        let result = report(vec![
            check(
                "check.one",
                CheckRequirement::Required,
                ValidationStatus::Rejected,
            ),
            check(
                "check.two",
                CheckRequirement::Advisory,
                ValidationStatus::Rejected,
            ),
            check(
                "check.three",
                CheckRequirement::Required,
                ValidationStatus::Verified,
            ),
        ]);

        assert_eq!(result.rejected_count(), 2);
        assert_eq!(result.verified_count(), 1);
        assert!(!result.can_proceed());
    }

    #[test]
    fn open_world_extension_names_are_supported() {
        let evidence = ExtensionEvidence::new(
            "future.security.authority",
            "future.evidence.reference",
            ValidationStatus::Verified,
            "extension validation completed",
        )
        .expect("extension evidence must be valid");

        assert_eq!(
            evidence.authority.as_ref(),
            "future.security.authority",
        );
    }

    #[test]
    fn extension_evidence_requires_an_authority() {
        assert!(
            ExtensionEvidence::new(
                "",
                "evidence.one",
                ValidationStatus::Verified,
                "validation completed",
            )
            .is_err()
        );
    }

    #[test]
    fn extension_evidence_requires_a_reference() {
        assert!(
            ExtensionEvidence::new(
                "authority.one",
                "",
                ValidationStatus::Verified,
                "validation completed",
            )
            .is_err()
        );
    }

    #[test]
    fn rejected_status_is_never_verified() {
        assert!(
            !ValidationStatus::Rejected.is_verified()
        );

        assert!(
            !ValidationStatus::Indeterminate.is_verified()
        );

        assert!(
            !ValidationStatus::NotEvaluated.is_verified()
        );
    }

    #[test]
    fn incomplete_report_is_not_complete() {
        let result = report(vec![
            check(
                "required.one",
                CheckRequirement::Required,
                ValidationStatus::Indeterminate,
            ),
        ]);

        assert!(!result.is_complete());
    }

    #[test]
    fn completed_verified_report_is_complete() {
        let result = report(vec![
            check(
                "required.one",
                CheckRequirement::Required,
                ValidationStatus::Verified,
            ),
        ]);

        assert!(result.is_complete());
        assert!(result.can_proceed());
    }
}
