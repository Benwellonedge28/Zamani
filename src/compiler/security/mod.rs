
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani Compiler Security Subsystem
//!
//! Path: `src/compiler/security/mod.rs`
//!
//! Rust: 1.97+
//! Edition: 2021
//!
//! ## Purpose
//!
//! This module is the canonical public composition root for compiler-side
//! security facilities.
//!
//! It integrates the existing:
//!
//! - security policy evaluator;
//! - security verification coordinator;
//! - software supply-chain verifier;
//! - security audit subsystem.
//!
//! It does not implement competing versions of those systems.
//!
//! ## Architectural ownership
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Lexer / Parser
//!      |
//!      v
//! Security AST
//!      |
//!      v
//! Semantic Security Model
//!      |
//!      v
//! Compiler Security Subsystem
//!      |
//!      +----------------------+
//!      |          |           |
//!      v          v           v
//!   Policy   Verification  Supply Chain
//! Evaluation Coordinator   Assessment
//!      |          |           |
//!      +----------+-----------+
//!                 |
//!                 v
//!             Audit
//!                 |
//!                 v
//!       Verified Compilation Evidence
//!                 |
//!                 v
//!          Compiler Pipeline
//!                 |
//!                 v
//!       Backend / Artifact Validation
//!                 |
//!                 v
//!             Runtime
//! ```
//!
//! ## Single-authority rules
//!
//! This module MUST NOT:
//!
//! - redefine semantic types;
//! - introduce another IR;
//! - implement cryptographic primitives;
//! - implement package resolution;
//! - implement runtime authorization;
//! - implement physical hardware discovery;
//! - allocate hardware resources;
//! - execute untrusted source code;
//! - replace authoritative domain-specific verifiers;
//! - replace the existing compiler safety guard.
//!
//! Those responsibilities remain with their existing repository owners.
//!
//! ## Grammar integration
//!
//! The authoritative source-level security grammar is:
//!
//! - `grammar/security/security.g4`
//! - `grammar/security/authorization.g4`
//! - `grammar/security/capabilities.g4`
//! - `grammar/security/permissions.g4`
//! - `grammar/security/policies.g4`
//! - `grammar/security/provenance.g4`
//! - `grammar/security/security-constraints.g4`
//! - `grammar/security/identity.g4`
//! - `grammar/security/identifiers.g4`
//! - `grammar/security/trust.g4`
//! - `grammar/security/privacy.g4`
//! - `grammar/security/cryptography.g4`
//! - `grammar/security/key-management.g4`
//! - `grammar/security/secrets.g4`
//! - `grammar/security/signatures.g4`
//! - `grammar/security/hashes.g4`
//! - `grammar/security/secure-computation.g4`
//! - `grammar/security/zero-knowledge.g4`
//! - `grammar/security/audit.g4`
//!
//! General policy syntax remains owned by:
//!
//! - `grammar/policies/policy.g4`
//! - `grammar/policies/security.g4`
//!
//! These grammars describe source-level intent.
//!
//! This Rust module consumes the validated representations supplied by
//! the frontend and semantic subsystem. It does not parse grammar files
//! or establish semantic validity.
//!
//! ## Repository integration
//!
//! Existing integration points:
//!
//! - `src/compiler/safety_guard.rs`
//! - `src/compiler/ir/verification.rs`
//! - `src/compiler/backends/`
//! - `src/compiler/artifacts/`
//! - `src/compiler/analysis/`
//! - `src/backend/`
//! - `src/semantic.rs`
//! - `src/ir_verify.rs`
//! - `src/runtime/`
//! - `src/source_map/`
//!
//! ## Security invariants
//!
//! 1. Missing mandatory verification evidence cannot establish acceptance.
//! 2. Indeterminate mandatory verification cannot establish acceptance.
//! 3. A provider failure cannot become successful verification.
//! 4. An explicit policy denial cannot become permission.
//! 5. Missing policy information cannot silently become permission.
//! 6. An unverified dependency cannot be represented as verified.
//! 7. An audit report cannot be represented as cryptographic proof.
//! 8. A successful compilation security assessment cannot be represented
//!    as proof of runtime safety.
//! 9. Security evidence must retain its originating provider and reference.
//! 10. Runtime enforcement remains the runtime's responsibility.
//!
//! ## Scalability
//!
//! No universal hardware, device, instruction, policy, package, or
//! verification-count ceiling is imposed by this module.
//!
//! Operational limits belong to the individual subsystem's configurable
//! budgets.
//!
//! Actual scalability depends on available memory, processing capacity,
//! provider capacity, and explicitly configured operational budgets.
//!
//! ## Compatibility
//!
//! The existing subsystem files and their public APIs remain authoritative.
//! This facade exposes them without changing their internal contracts.

// ============================================================================
// 1. SUBSYSTEM MODULES
// ============================================================================

/// Deterministic compiler security policy evaluation.
pub mod policy;

/// Security verification coordination and evidence management.
pub mod verification;

/// Software dependency and artifact supply-chain assessment.
pub mod supply_chain;

/// Security audit events, evidence and reporting.
pub mod audit;

// ============================================================================
// 2. API VERSIONING
// ============================================================================

/// Public compiler security API version.
///
/// Increment when an incompatible public security-facade contract changes.
pub const COMPILER_SECURITY_API_VERSION: u16 = 1;

/// Security subsystem name.
pub const COMPILER_SECURITY_MODULE_NAME: &str = "zamani.compiler.security";

/// Security subsystem API version.
pub const fn api_version() -> u16 {
    COMPILER_SECURITY_API_VERSION
}

/// Returns the canonical security subsystem name.
pub const fn module_name() -> &'static str {
    COMPILER_SECURITY_MODULE_NAME
}

// ============================================================================
// 3. POLICY API
// ============================================================================

pub use self::policy::{
    AttributePath,
    AttributeValue,
    CombiningAlgorithm,
    ComparisonOperator,
    Condition,
    EvaluationIssue,
    Policy,
    PolicyAttributes,
    PolicyDecision,
    PolicyDecisionKind,
    PolicyEffect,
    PolicyError,
    PolicyEvaluationLimits,
    PolicyEvaluator,
    PolicyId,
    PolicyRequest,
    PolicyRule,
    PolicySet,
    RuleId,
    Selector,
};

// ============================================================================
// 4. VERIFICATION API
// ============================================================================

pub use self::verification::{
    CheckId,
    CheckOutcome,
    CheckRequirement,
    CheckResult,
    EvidenceId,
    VerificationBudget,
    VerificationCancellation,
    VerificationCheck,
    VerificationCoordinator,
    VerificationDecision,
    VerificationDomain,
    VerificationError,
    VerificationEvidence,
    VerificationObserver,
    VerificationPlan,
    VerificationPolicy,
    VerificationProvider,
    VerificationReference,
    VerificationReport,
    VerificationRequest,
    VerificationResult,
    VerificationRunId,
    VerificationSubject,
    VerificationSummary,
    ProviderId,
    SubjectId,
};

// ============================================================================
// 5. SUPPLY-CHAIN API
// ============================================================================

pub use self::supply_chain::{
    AttestationEnvelope,
    FindingSeverity as SupplyChainFindingSeverity,
    PackageContentVerifier,
    PackageDependency,
    PackageId,
    PackageRecord,
    PackageSignatureVerifier,
    PackageSource,
    ProviderVerdict,
    SignatureEnvelope,
    SupplyChainBudget,
    SupplyChainError,
    SupplyChainFinding,
    SupplyChainPolicy,
    SupplyChainProviders,
    SupplyChainReport,
    SupplyChainResult,
    SupplyChainStatus,
    SupplyChainVerifier,
    VulnerabilityFinding,
    VulnerabilityProvider,
    VulnerabilitySeverity,
};

// ============================================================================
// 6. AUDIT API
// ============================================================================

pub use self::audit::{
    ArtifactReference,
    AuditAnalyzer,
    AuditBudget,
    AuditCategory,
    AuditDecision,
    AuditDurability,
    AuditError,
    AuditEvent,
    AuditEventId,
    AuditEvidence,
    AuditFinding,
    AuditFindingId,
    AuditInput,
    AuditObservation,
    AuditObservationId,
    AuditOutcome,
    AuditReceipt,
    AuditReport,
    AuditReportReceipt,
    AuditReportSink,
    AuditResult,
    AuditService,
    AuditSeverity,
    AuditSink,
    AuditStageId,
    AuditStatus,
    AuditSubject,
    AuditTimestamp,
    EvidenceStatus as AuditEvidenceStatus,
    EvidenceType,
    FindingOrigin,
    ProvenanceReference,
    RequiredEvidence,
    SourceReference,
};

// ============================================================================
// 7. API CONTRACT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn security_api_version_is_stable() {
        assert_eq!(api_version(), 1);
        assert_eq!(
            module_name(),
            "zamani.compiler.security"
        );
    }

    #[test]
    fn policy_api_is_available() {
        let id = PolicyId::new("test.policy")
            .expect("valid policy identifier");

        assert_eq!(id.as_str(), "test.policy");
    }

    #[test]
    fn verification_api_is_available() {
        let id = CheckId::new("security.integrity")
            .expect("valid check identifier");

        assert_eq!(id.as_str(), "security.integrity");
    }

    #[test]
    fn supply_chain_api_is_available() {
        let id = PackageId::new("zamani.core")
            .expect("valid package identifier");

        assert_eq!(id.as_str(), "zamani.core");
    }

    #[test]
    fn audit_api_is_available() {
        let timestamp =
            AuditTimestamp::from_unix_nanoseconds(0);

        assert_eq!(
            timestamp.unix_nanoseconds(),
            0
        );
    }

    #[test]
    fn evidence_status_namespaces_are_distinct() {
        let _: Option<AuditEvidenceStatus> = None;
        let _: Option<EvidenceStatus> = None;
    }
}
