
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — JIT Artifact Verification
//!
//! Path: `src/zur/jit/verification.rs`
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! # Purpose
//!
//! Provides the verification boundary for artifacts produced by the
//! canonical Zamani JIT compiler integration.
//!
//! Verification is required for:
//!
//! - Newly compiled JIT artifacts.
//! - Existing JIT cache entries.
//! - Concurrently published cache artifacts.
//! - Specialized artifacts.
//! - Recompiled artifacts.
//! - Artifacts presented for subsequent execution.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - JIT artifact identity consistency checks.
//! - Verification authority contracts.
//! - Verification evidence contracts.
//! - Evidence-to-request binding.
//! - Verification receipts.
//! - The `JitArtifactVerifier` implementation.
//!
//! It does not own:
//!
//! - Source parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Requirements, capabilities, preferences or constraints.
//! - Classical IR.
//! - Quantum IR.
//! - HDL IR.
//! - ZUIR.
//! - Compiler optimization.
//! - Compiler lowering.
//! - Backend registration.
//! - Backend selection.
//! - Code generation.
//! - Artifact serialization.
//! - Artifact manifest construction.
//! - Executable memory.
//! - Runtime execution.
//! - Hardware discovery.
//! - Physical resource reservation.
//!
//! # Canonical integration
//!
//! `crate::zur::jit::engine`:
//!     JitArtifactVerifier
//!     JitCompilationRequest
//!     CompiledJitArtifact
//!     JitCacheKey
//!
//! `crate::zur::error`:
//!     ZurError
//!     ZurResult
//!
//! `crate::compiler::artifacts`:
//!     Canonical compiler artifact and provenance authorities.
//!
//! `crate::backend`:
//!     Backend-produced artifact and target compatibility authorities.
//!
//! `crate::zur::security`:
//!     Artifact integrity and runtime security policy adapters.
//!
//! `crate::zur::execution`:
//!     Consumes artifacts after successful verification.
//!
//! # Grammar boundary
//!
//! Relevant grammar authorities:
//!
//! - `grammar/execution/runtime.g4`
//! - `grammar/execution/runtime-capabilities.g4`
//! - `grammar/compile/optimization.g4`
//! - `grammar/compile/specialization.g4`
//! - `grammar/resources/portability.g4`
//! - `grammar/core/requirements.g4`
//! - `grammar/core/constraints.g4`
//! - `grammar/core/hints.g4`
//!
//! Relevant specifications:
//!
//! - `grammar/spec/portability.md`
//! - `grammar/specification/execution-model.md`
//! - `grammar/specification/semantic-model.md`
//!
//! These authorities establish source intent and program meaning.
//! This module consumes validated compiler/runtime inputs.
//! It never parses grammar or interprets AST nodes.
//!
//! # Verification invariants
//!
//! 1. An artifact must match its original compilation request.
//! 2. An artifact must be checked before cache acceptance or publication.
//! 3. A cache hit is not proof of artifact validity.
//! 4. A provider identity is not proof of provider trust.
//! 5. A capability declaration is not proof of actual capability.
//! 6. A successful verification is not proof of successful execution.
//! 7. An injected authority must perform the checks appropriate to the
//!    artifact format and its security policy.
//! 8. Missing verification evidence is a failure.
//! 9. Evidence for another artifact or request must be rejected.
//! 10. Verification failures must not be converted into success.
//!
//! # Scalability
//!
//! No fixed hardware, provider, domain, artifact-count, or evidence-count
//! ceiling is imposed.
//!
//! Any operational budgets belong to the injected verification authority
//! and its canonical resource/security policies.
//!
//! Literal infinite physical computation is not promised.

use std::fmt;
use std::sync::Arc;

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::jit::engine::{
    CompiledJitArtifact,
    JitArtifactVerifier,
    JitCompilationRequest,
    JitCacheKey,
};

// ============================================================================
// COMPONENT IDENTITY
// ============================================================================

const COMPONENT: &str = "zur.jit.verification";

// ============================================================================
// ERROR CONSTRUCTION
// ============================================================================

fn verification_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::invalid_data(message)
        .with_component(COMPONENT)
        .with_operation("verify_artifact")
}

fn authority_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::configuration(message)
        .with_component(COMPONENT)
        .with_operation("configure_verification_authority")
}

fn compatibility_error(
    message: impl Into<String>,
) -> ZurError {
    ZurError::compatibility(message)
        .with_component(COMPONENT)
        .with_operation("verify_artifact")
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

/// Validates a nonempty, canonical verification identifier.
///
/// Identifiers are intentionally open-ended. No maximum length or
/// namespace-depth limit is imposed.
///
/// Surrounding whitespace is rejected rather than silently normalized.
fn validate_identifier(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.is_empty() {
        return Err(verification_error(
            format!("{description} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(verification_error(
            format!(
                "{description} contains surrounding whitespace"
            ),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(verification_error(
            format!(
                "{description} contains control characters"
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// VERIFICATION SUBJECT
// ============================================================================

/// Identifies the artifact and compilation request being verified.
///
/// This is a borrowed view over canonical engine-owned objects.
/// It does not create another artifact representation.
pub struct VerificationSubject<'a> {
    request: &'a JitCompilationRequest,
    artifact: &'a dyn CompiledJitArtifact,
}

impl<'a> VerificationSubject<'a> {
    /// Creates a verification subject.
    ///
    /// The constructor checks identity consistency before the subject
    /// can be passed to an authority.
    pub fn new(
        request: &'a JitCompilationRequest,
        artifact: &'a dyn CompiledJitArtifact,
    ) -> ZurResult<Self> {
        validate_artifact_identity(request, artifact)?;

        Ok(Self {
            request,
            artifact,
        })
    }

    /// Returns the original JIT compilation request.
    #[must_use]
    pub fn request(&self) -> &'a JitCompilationRequest {
        self.request
    }

    /// Returns the artifact being verified.
    #[must_use]
    pub fn artifact(&self) -> &'a dyn CompiledJitArtifact {
        self.artifact
    }

    /// Returns the canonical cache identity.
    #[must_use]
    pub fn cache_key(&self) -> &'a JitCacheKey {
        self.request.cache_key()
    }

    /// Returns the artifact's domain identity.
    #[must_use]
    pub fn domain_identity(&self) -> &'a str {
        self.artifact.domain_identity()
    }

    /// Returns the artifact's provider identity.
    #[must_use]
    pub fn provider_identity(&self) -> &'a str {
        self.artifact.provider_identity()
    }

    /// Returns the artifact-format identity.
    #[must_use]
    pub fn format_identity(&self) -> &'a str {
        self.artifact.format_identity()
    }

    /// Returns the compiler contract version.
    #[must_use]
    pub fn compiler_contract_version(&self) -> &'a str {
        self.artifact.compiler_contract_version()
    }
}

impl fmt::Debug for VerificationSubject<'_> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("VerificationSubject")
            .field("cache_key", &self.cache_key())
            .field("domain_identity", &self.domain_identity())
            .field("provider_identity", &self.provider_identity())
            .field("format_identity", &self.format_identity())
            .field(
                "compiler_contract_version",
                &self.compiler_contract_version(),
            )
            .finish()
    }
}

// ============================================================================
// ARTIFACT IDENTITY VALIDATION
// ============================================================================

/// Checks that the artifact belongs to the supplied compilation request.
///
/// This check is independent of the injected artifact verification
/// authority. It prevents an authority from being asked to verify an
/// artifact that is already inconsistent with the engine request.
pub fn validate_artifact_identity(
    request: &JitCompilationRequest,
    artifact: &dyn CompiledJitArtifact,
) -> ZurResult<()> {
    if artifact.cache_key() != request.cache_key() {
        return Err(compatibility_error(
            "artifact cache identity does not match the compilation request",
        ));
    }

    validate_identifier(
        artifact.domain_identity(),
        "artifact domain identity",
    )?;

    validate_identifier(
        artifact.provider_identity(),
        "artifact provider identity",
    )?;

    validate_identifier(
        artifact.format_identity(),
        "artifact format identity",
    )?;

    validate_identifier(
        artifact.compiler_contract_version(),
        "artifact compiler contract version",
    )?;

    if artifact.domain_identity() != request.domain_identity() {
        return Err(compatibility_error(
            "artifact domain does not match the compilation request",
        ));
    }

    if artifact.compiler_contract_version()
        != request.unit().compiler_contract_version()
    {
        return Err(compatibility_error(
            "artifact compiler contract version does not match the compilation unit",
        ));
    }

    Ok(())
}

// ============================================================================
// VERIFICATION EVIDENCE
// ============================================================================

/// Immutable evidence returned by an injected verification authority.
///
/// Evidence binds a successful verification decision to:
///
/// - The exact JIT cache identity.
/// - The computational domain.
/// - The code-generation provider.
/// - The artifact format.
/// - The compiler contract version.
/// - The authority that performed verification.
/// - The verification policy.
/// - An authority-supplied evidence fingerprint.
///
/// The fingerprint is opaque. This module does not invent a digest
/// algorithm or claim that a string is cryptographic proof.
///
/// The injected authority is responsible for ensuring that its
/// fingerprint identifies the actual verification evidence it produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationEvidence {
    cache_key: JitCacheKey,
    domain_identity: String,
    provider_identity: String,
    format_identity: String,
    compiler_contract_version: String,
    authority_identity: String,
    policy_identity: String,
    evidence_fingerprint: String,
}

impl VerificationEvidence {
    /// Creates evidence bound to a verification subject.
    ///
    /// This constructor validates evidence identity and structure.
    ///
    /// The caller must only construct this value after its verification
    /// authority has genuinely completed all applicable checks.
    ///
    /// Creating an evidence object does not itself verify an artifact.
    pub fn new(
        subject: &VerificationSubject<'_>,
        authority_identity: impl Into<String>,
        policy_identity: impl Into<String>,
        evidence_fingerprint: impl Into<String>,
    ) -> ZurResult<Self> {
        let authority_identity = authority_identity.into();
        let policy_identity = policy_identity.into();
        let evidence_fingerprint = evidence_fingerprint.into();

        validate_identifier(
            &authority_identity,
            "verification authority identity",
        )?;

        validate_identifier(
            &policy_identity,
            "verification policy identity",
        )?;

        validate_identifier(
            &evidence_fingerprint,
            "verification evidence fingerprint",
        )?;

        Ok(Self {
            cache_key: subject.cache_key().clone(),
            domain_identity: subject.domain_identity().to_owned(),
            provider_identity: subject.provider_identity().to_owned(),
            format_identity: subject.format_identity().to_owned(),
            compiler_contract_version: subject
                .compiler_contract_version()
                .to_owned(),
            authority_identity,
            policy_identity,
            evidence_fingerprint,
        })
    }

    /// Returns the verified cache identity.
    #[must_use]
    pub fn cache_key(&self) -> &JitCacheKey {
        &self.cache_key
    }

    /// Returns the verified computational domain.
    #[must_use]
    pub fn domain_identity(&self) -> &str {
        &self.domain_identity
    }

    /// Returns the verified code-generation provider.
    #[must_use]
    pub fn provider_identity(&self) -> &str {
        &self.provider_identity
    }

    /// Returns the verified artifact format.
    #[must_use]
    pub fn format_identity(&self) -> &str {
        &self.format_identity
    }

    /// Returns the verified compiler contract version.
    #[must_use]
    pub fn compiler_contract_version(&self) -> &str {
        &self.compiler_contract_version
    }

    /// Returns the identity of the verification authority.
    #[must_use]
    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    /// Returns the identity of the verification policy.
    #[must_use]
    pub fn policy_identity(&self) -> &str {
        &self.policy_identity
    }

    /// Returns the authority-supplied evidence fingerprint.
    #[must_use]
    pub fn evidence_fingerprint(&self) -> &str {
        &self.evidence_fingerprint
    }

    /// Checks whether this evidence is bound to the supplied subject.
    pub fn validate_binding(
        &self,
        subject: &VerificationSubject<'_>,
    ) -> ZurResult<()> {
        if self.cache_key != *subject.cache_key() {
            return Err(verification_error(
                "verification evidence belongs to another cache identity",
            ));
        }

        if self.domain_identity != subject.domain_identity() {
            return Err(verification_error(
                "verification evidence belongs to another computational domain",
            ));
        }

        if self.provider_identity != subject.provider_identity() {
            return Err(verification_error(
                "verification evidence belongs to another code-generation provider",
            ));
        }

        if self.format_identity != subject.format_identity() {
            return Err(verification_error(
                "verification evidence belongs to another artifact format",
            ));
        }

        if self.compiler_contract_version
            != subject.compiler_contract_version()
        {
            return Err(verification_error(
                "verification evidence belongs to another compiler contract version",
            ));
        }

        Ok(())
    }
}

// ============================================================================
// VERIFICATION AUTHORITY
// ============================================================================

/// Performs artifact-specific verification using canonical authorities.
///
/// Implementations are injected by the ZUR composition layer.
///
/// An implementation may adapt:
///
/// - Compiler artifact integrity verification.
/// - Compiler provenance validation.
/// - Backend artifact-format validation.
/// - Runtime ABI compatibility.
/// - Security policy.
/// - Provider trust policy.
/// - Target compatibility evidence.
/// - Specialization certificates and assumptions.
///
/// The implementation must inspect or validate the actual artifact
/// through the concrete artifact integration available to it.
///
/// The common JIT artifact trait intentionally does not expose a
/// universal byte representation.
///
/// A successful return must mean that every check required by the
/// applicable verification policy has completed successfully.
///
/// Returning evidence without performing those checks violates this
/// contract.
pub trait JitArtifactVerificationAuthority: Send + Sync {
    /// Performs artifact-specific verification.
    ///
    /// Returns evidence only after successful verification.
    ///
    /// Errors must be propagated. An unavailable verification service,
    /// unsupported artifact format, missing provenance, failed integrity
    /// check or rejected security policy must not be reported as success.
    fn verify(
        &self,
        subject: &VerificationSubject<'_>,
    ) -> ZurResult<VerificationEvidence>;
}

// ============================================================================
// VERIFICATION RECEIPT
// ============================================================================

/// Receipt for a successfully verified JIT artifact.
///
/// A receipt records the authority and policy that approved the artifact.
/// It does not certify that the artifact has been executed successfully.
///
/// Receipts can be retained by observability or audit integrations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReceipt {
    evidence: VerificationEvidence,
}

impl VerificationReceipt {
    /// Creates a receipt from validated evidence.
    fn from_evidence(
        evidence: VerificationEvidence,
    ) -> Self {
        Self { evidence }
    }

    /// Returns the verified cache identity.
    #[must_use]
    pub fn cache_key(&self) -> &JitCacheKey {
        self.evidence.cache_key()
    }

    /// Returns the authority identity.
    #[must_use]
    pub fn authority_identity(&self) -> &str {
        self.evidence.authority_identity()
    }

    /// Returns the policy identity.
    #[must_use]
    pub fn policy_identity(&self) -> &str {
        self.evidence.policy_identity()
    }

    /// Returns the evidence fingerprint.
    #[must_use]
    pub fn evidence_fingerprint(&self) -> &str {
        self.evidence.evidence_fingerprint()
    }

    /// Returns the full verification evidence.
    #[must_use]
    pub fn evidence(&self) -> &VerificationEvidence {
        &self.evidence
    }
}

// ============================================================================
// JIT VERIFIER
// ============================================================================

/// Production JIT artifact verifier.
///
/// This type implements the exact `JitArtifactVerifier` contract consumed
/// by the existing `JitEngine`.
///
/// It performs common identity checks and delegates artifact-specific
/// verification to an injected authority.
///
/// It is immutable after construction and can be shared across concurrent
/// JIT compilation requests.
pub struct JitVerifier {
    authority: Arc<dyn JitArtifactVerificationAuthority>,
}

impl fmt::Debug for JitVerifier {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("JitVerifier")
            .finish_non_exhaustive()
    }
}

impl JitVerifier {
    /// Constructs a verifier with an explicit verification authority.
    ///
    /// No default or permissive verification authority is installed.
    pub fn new(
        authority: Arc<dyn JitArtifactVerificationAuthority>,
    ) -> Self {
        Self { authority }
    }

    /// Performs verification and returns an auditable receipt.
    ///
    /// The receipt is produced only after:
    ///
    /// 1. Request/artifact identity validation.
    /// 2. Successful authority verification.
    /// 3. Evidence structure validation.
    /// 4. Evidence-to-request binding validation.
    pub fn verify_with_receipt(
        &self,
        request: &JitCompilationRequest,
        artifact: &dyn CompiledJitArtifact,
    ) -> ZurResult<VerificationReceipt> {
        let subject = VerificationSubject::new(
            request,
            artifact,
        )?;

        let evidence = self.authority.verify(&subject)?;

        evidence.validate_binding(&subject)?;

        Ok(VerificationReceipt::from_evidence(evidence))
    }
}

impl JitArtifactVerifier for JitVerifier {
    /// Verifies an artifact before it is accepted by the JIT engine.
    fn verify(
        &self,
        request: &JitCompilationRequest,
        artifact: &dyn CompiledJitArtifact,
    ) -> ZurResult<()> {
        self.verify_with_receipt(request, artifact)
            .map(|_| ())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{
        AtomicBool,
        AtomicUsize,
        Ordering,
    };

    use crate::zur::jit::engine::{
        JitCompilationReason,
        JitCompilationUnit,
    };

    #[derive(Debug)]
    struct TestCompilationUnit {
        identity: String,
        domain: String,
        compiler_version: String,
    }

    impl JitCompilationUnit for TestCompilationUnit {
        fn stable_identity(&self) -> &str {
            &self.identity
        }

        fn domain_identity(&self) -> &str {
            &self.domain
        }

        fn compiler_contract_version(&self) -> &str {
            &self.compiler_version
        }
    }

    #[derive(Debug)]
    struct TestArtifact {
        key: JitCacheKey,
        domain: String,
        provider: String,
        format: String,
        compiler_version: String,
    }

    impl CompiledJitArtifact for TestArtifact {
        fn cache_key(&self) -> &JitCacheKey {
            &self.key
        }

        fn domain_identity(&self) -> &str {
            &self.domain
        }

        fn provider_identity(&self) -> &str {
            &self.provider
        }

        fn format_identity(&self) -> &str {
            &self.format
        }

        fn compiler_contract_version(&self) -> &str {
            &self.compiler_version
        }
    }

    fn request() -> JitCompilationRequest {
        let unit = Arc::new(TestCompilationUnit {
            identity: "test.unit".to_owned(),
            domain: "zamani.classical".to_owned(),
            compiler_version: "1.0".to_owned(),
        });

        JitCompilationRequest::new(
            unit,
            JitCacheKey::new("test.cache.key")
                .expect("valid cache key"),
            JitCompilationReason::Baseline,
        )
        .expect("valid compilation request")
    }

    fn artifact() -> TestArtifact {
        TestArtifact {
            key: JitCacheKey::new("test.cache.key")
                .expect("valid cache key"),
            domain: "zamani.classical".to_owned(),
            provider: "zamani.test-provider".to_owned(),
            format: "zamani.test-format".to_owned(),
            compiler_version: "1.0".to_owned(),
        }
    }

    struct AcceptingAuthority {
        called: AtomicUsize,
    }

    impl JitArtifactVerificationAuthority for AcceptingAuthority {
        fn verify(
            &self,
            subject: &VerificationSubject<'_>,
        ) -> ZurResult<VerificationEvidence> {
            self.called.fetch_add(1, Ordering::SeqCst);

            VerificationEvidence::new(
                subject,
                "zamani.test-verifier",
                "zamani.test-policy",
                "test-evidence-fingerprint",
            )
        }
    }

    struct RejectingAuthority;

    impl JitArtifactVerificationAuthority for RejectingAuthority {
        fn verify(
            &self,
            _subject: &VerificationSubject<'_>,
        ) -> ZurResult<VerificationEvidence> {
            Err(ZurError::invalid_data(
                "test artifact integrity verification failed",
            ))
        }
    }

    struct MismatchedEvidenceAuthority;

    impl JitArtifactVerificationAuthority for MismatchedEvidenceAuthority {
        fn verify(
            &self,
            subject: &VerificationSubject<'_>,
        ) -> ZurResult<VerificationEvidence> {
            let mut evidence = VerificationEvidence::new(
                subject,
                "zamani.test-verifier",
                "zamani.test-policy",
                "test-evidence-fingerprint",
            )?;

            evidence.domain_identity =
                "zamani.quantum".to_owned();

            Ok(evidence)
        }
    }

    struct CountingAuthority {
        called: AtomicBool,
    }

    impl JitArtifactVerificationAuthority for CountingAuthority {
        fn verify(
            &self,
            subject: &VerificationSubject<'_>,
        ) -> ZurResult<VerificationEvidence> {
            self.called.store(true, Ordering::SeqCst);

            VerificationEvidence::new(
                subject,
                "zamani.test-verifier",
                "zamani.test-policy",
                "test-evidence-fingerprint",
            )
        }
    }

    #[test]
    fn accepts_matching_artifact_identity() {
        let request = request();
        let artifact = artifact();

        assert!(
            validate_artifact_identity(&request, &artifact)
                .is_ok()
        );
    }

    #[test]
    fn rejects_a_different_cache_identity() {
        let request = request();
        let mut artifact = artifact();

        artifact.key = JitCacheKey::new(
            "another.cache.key",
        )
        .expect("valid cache key");

        assert!(
            validate_artifact_identity(&request, &artifact)
                .is_err()
        );
    }

    #[test]
    fn rejects_a_different_domain() {
        let request = request();
        let mut artifact = artifact();

        artifact.domain = "zamani.quantum".to_owned();

        assert!(
            validate_artifact_identity(&request, &artifact)
                .is_err()
        );
    }

    #[test]
    fn rejects_a_different_compiler_contract_version() {
        let request = request();
        let mut artifact = artifact();

        artifact.compiler_version = "2.0".to_owned();

        assert!(
            validate_artifact_identity(&request, &artifact)
                .is_err()
        );
    }

    #[test]
    fn rejects_empty_provider_identity() {
        let request = request();
        let mut artifact = artifact();

        artifact.provider.clear();

        assert!(
            validate_artifact_identity(&request, &artifact)
                .is_err()
        );
    }

    #[test]
    fn rejects_empty_artifact_format() {
        let request = request();
        let mut artifact = artifact();

        artifact.format.clear();

        assert!(
            validate_artifact_identity(&request, &artifact)
                .is_err()
        );
    }

    #[test]
    fn verifier_delegates_to_authority() {
        let authority = Arc::new(AcceptingAuthority {
            called: AtomicUsize::new(0),
        });

        let verifier = JitVerifier::new(authority.clone());

        let request = request();
        let artifact = artifact();

        assert!(
            verifier.verify(&request, &artifact).is_ok()
        );

        assert_eq!(
            authority.called.load(Ordering::SeqCst),
            1
        );
    }

    #[test]
    fn verifier_returns_bound_receipt() {
        let verifier = JitVerifier::new(
            Arc::new(AcceptingAuthority {
                called: AtomicUsize::new(0),
            }),
        );

        let request = request();
        let artifact = artifact();

        let receipt = verifier
            .verify_with_receipt(&request, &artifact)
            .expect("verification should succeed");

        assert_eq!(
            receipt.cache_key(),
            request.cache_key()
        );

        assert_eq!(
            receipt.authority_identity(),
            "zamani.test-verifier"
        );

        assert_eq!(
            receipt.policy_identity(),
            "zamani.test-policy"
        );

        assert_eq!(
            receipt.evidence_fingerprint(),
            "test-evidence-fingerprint"
        );
    }

    #[test]
    fn verifier_propagates_authority_failure() {
        let verifier = JitVerifier::new(
            Arc::new(RejectingAuthority),
        );

        let request = request();
        let artifact = artifact();

        assert!(
            verifier.verify(&request, &artifact).is_err()
        );
    }

    #[test]
    fn verifier_rejects_mismatched_authority_evidence() {
        let verifier = JitVerifier::new(
            Arc::new(MismatchedEvidenceAuthority),
        );

        let request = request();
        let artifact = artifact();

        assert!(
            verifier.verify(&request, &artifact).is_err()
        );
    }

    #[test]
    fn invalid_artifact_is_rejected_before_authority_invocation() {
        let authority = Arc::new(CountingAuthority {
            called: AtomicBool::new(false),
        });

        let verifier = JitVerifier::new(authority.clone());

        let request = request();
        let mut artifact = artifact();

        artifact.key = JitCacheKey::new(
            "different.cache.key",
        )
        .expect("valid cache key");

        assert!(
            verifier.verify(&request, &artifact).is_err()
        );

        assert!(
            !authority.called.load(Ordering::SeqCst)
        );
    }

    #[test]
    fn evidence_cannot_be_reused_for_another_cache_key() {
        let request = request();
        let artifact = artifact();

        let subject = VerificationSubject::new(
            &request,
            &artifact,
        )
        .expect("valid subject");

        let evidence = VerificationEvidence::new(
            &subject,
            "zamani.test-verifier",
            "zamani.test-policy",
            "test-evidence-fingerprint",
        )
        .expect("valid evidence");

        let other_unit = Arc::new(TestCompilationUnit {
            identity: "test.other-unit".to_owned(),
            domain: "zamani.classical".to_owned(),
            compiler_version: "1.0".to_owned(),
        });

        let other_request = JitCompilationRequest::new(
            other_unit,
            JitCacheKey::new("other.cache.key")
                .expect("valid cache key"),
            JitCompilationReason::Baseline,
        )
        .expect("valid request");

        let other_artifact = TestArtifact {
            key: JitCacheKey::new("other.cache.key")
                .expect("valid cache key"),
            ..artifact
        };

        let other_subject = VerificationSubject::new(
            &other_request,
            &other_artifact,
        )
        .expect("valid other subject");

        assert!(
            evidence.validate_binding(&other_subject).is_err()
        );
    }

    #[test]
    fn namespaced_future_domains_are_not_closed_by_verifier() {
        let unit = Arc::new(TestCompilationUnit {
            identity: "test.photonics-unit".to_owned(),
            domain: "zamani.photonics".to_owned(),
            compiler_version: "1.0".to_owned(),
        });

        let request = JitCompilationRequest::new(
            unit,
            JitCacheKey::new("photonics.cache.key")
                .expect("valid cache key"),
            JitCompilationReason::Baseline,
        )
        .expect("valid request");

        let artifact = TestArtifact {
            key: JitCacheKey::new("photonics.cache.key")
                .expect("valid cache key"),
            domain: "zamani.photonics".to_owned(),
            provider: "zamani.photonics-provider".to_owned(),
            format: "zamani.photonics-format".to_owned(),
            compiler_version: "1.0".to_owned(),
        };

        assert!(
            validate_artifact_identity(&request, &artifact)
                .is_ok()
        );
    }
}
