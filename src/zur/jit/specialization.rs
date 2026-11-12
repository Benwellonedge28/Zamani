
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — JIT Specialization
//!
//! Path: src/zur/jit/specialization.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides the validation and orchestration boundary for JIT
//! specialization requests.
//!
//! This module accepts:
//!
//! - A canonical compiler-owned compilation unit.
//! - Its existing baseline JIT cache identity.
//! - A compiler-supplied specialization cache identity.
//! - A hotness decision from the canonical JIT hotness subsystem.
//! - Opaque specialization evidence.
//! - An explicitly injected specialization authority.
//!
//! It produces a validated specialization request for the existing
//! JIT engine.
//!
//! This module does not implement compiler transformations.
//!
//! # OWNERSHIP
//!
//! This module owns:
//!
//! - Specialization candidate validation.
//! - Specialization evidence contracts.
//! - Specialization authority contracts.
//! - Specialization certificate contracts.
//! - Certificate-to-request binding validation.
//! - Prepared specialization requests.
//! - Baseline identity preservation.
//!
//! It does not own:
//!
//! - Source parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Requirements or constraints.
//! - Compiler IR.
//! - Quantum IR.
//! - HDL IR.
//! - ZUIR.
//! - Compiler optimization.
//! - Compiler lowering.
//! - Backend registration.
//! - Backend selection.
//! - Hardware discovery.
//! - Physical resource reservation.
//! - Code generation.
//! - Executable memory.
//! - Code-cache management.
//! - Profiling.
//! - Hotness detection.
//! - Deoptimization.
//! - Invalidation.
//! - Physical execution.
//!
//! # CANONICAL INTEGRATION
//!
//! JIT engine:
//!     crate::zur::jit::engine
//!
//! Canonical compilation unit:
//!     JitCompilationUnit
//!
//! Canonical cache identity:
//!     JitCacheKey
//!
//! Canonical compilation request:
//!     JitCompilationRequest
//!
//! Canonical compilation reason:
//!     JitCompilationReason
//!
//! Runtime profiling:
//!     crate::zur::jit::profiling
//!
//! Hotness analysis:
//!     crate::zur::jit::hotness
//!
//! Canonical compiler:
//!     crate::compiler
//!
//! Compiler optimization:
//!     crate::compiler::optimization
//!
//! Compiler IR and passes:
//!     crate::compiler::ir
//!     crate::compiler::passes
//!
//! Backend:
//!     crate::backend
//!
//! # GRAMMAR BOUNDARY
//!
//! Relevant grammar:
//!
//!     grammar/compile/optimization.g4
//!     grammar/core/requirements.g4
//!     grammar/core/constraints.g4
//!     grammar/core/hints.g4
//!     grammar/resources/
//!     grammar/execution/
//!
//! Relevant specifications:
//!
//!     grammar/spec/portability.md
//!     grammar/specification/semantic-model.md
//!     grammar/specification/execution-model.md
//!
//! Source-level optimization intent is resolved by the frontend
//! and compiler semantic authorities.
//!
//! This module does not parse grammar or interpret AST nodes.
//!
//! A source-level preference is not a specialization requirement.
//!
//! A capability declaration is not proof of current capability.
//!
//! A resource requirement is not proof of resource availability.
//!
//! A successful validation is not proof that physical execution
//! has succeeded.
//!
//! # SCALABILITY
//!
//! No arbitrary limits are imposed on:
//!
//! - Computational domains.
//! - Compilation units.
//! - Specialization assumptions.
//! - Providers.
//! - Hardware architectures.
//! - Capabilities.
//! - Resource dimensions.
//! - Specialization candidates.
//!
//! Cache identities and evidence fingerprints are supplied by
//! their canonical authorities.
//!
//! Optional resource budgets belong to the relevant runtime,
//! compiler and resource-management policies.
//!
//! Physical memory and processing capacity remain real constraints.
//!
//! Literal infinite computation is not promised.
//!
//! # SAFETY
//!
//! - No unsafe Rust.
//! - No raw pointers.
//! - No global mutable state.
//! - No implicit hardware discovery.
//! - No implicit backend selection.
//! - No fabricated capability evidence.
//! - No fabricated semantic-preservation claims.
//! - No automatic resource reservation.
//! - No automatic compilation.
//! - No automatic execution.
//!
//! # INTEGRATION INVARIANTS
//!
//! 1. Compiler-owned representations remain compiler-owned.
//! 2. Specialization keys are supplied by the compiler integration.
//! 3. A specialization candidate must have sufficient hotness evidence.
//! 4. Capability evidence must be validated by an injected authority.
//! 5. Target compatibility must be validated by an injected authority.
//! 6. Assumptions must have an applicable runtime guard strategy.
//! 7. Semantic-preservation obligations must be verified.
//! 8. The baseline identity must remain available for fallback integration.
//! 9. The certificate must bind to the exact candidate being prepared.
//! 10. Only the existing JIT engine performs compilation.
//!
//! ============================================================================

use std::fmt;
use std::sync::Arc;

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::jit::engine::{
    JitCacheKey,
    JitCompilationReason,
    JitCompilationRequest,
    JitCompilationUnit,
};

use crate::zur::jit::hotness::{
    HotnessDecision,
    HotnessStatus,
};

// =============================================================================
// API VERSION
// =============================================================================

/// Public API version for ZUR JIT specialization.
pub const JIT_SPECIALIZATION_API_VERSION: u16 = 1;

/// Component identity used in structured errors.
const COMPONENT: &str = "zur.jit.specialization";

// =============================================================================
// ERROR CONSTRUCTION
// =============================================================================

/// Constructs a specialization validation error.
fn validation_error(message: impl Into<String>) -> ZurError {
    ZurError::configuration(message)
        .with_component(COMPONENT)
        .with_operation("validate_specialization")
}

/// Constructs a specialization compatibility error.
fn compatibility_error(message: impl Into<String>) -> ZurError {
    ZurError::compatibility(message)
        .with_component(COMPONENT)
        .with_operation("validate_specialization")
}

/// Constructs a specialization contract error.
fn contract_error(message: impl Into<String>) -> ZurError {
    ZurError::internal(message)
        .with_component(COMPONENT)
        .with_operation("validate_certificate")
}

/// Validates a nonempty, unpadded identity.
///
/// The identity's semantics and authority are established by
/// the subsystem that supplies it.
///
/// This function validates representation only.
fn validate_identity(
    value: &str,
    description: &str,
) -> ZurResult<()> {
    if value.is_empty() {
        return Err(validation_error(
            format!("{description} must not be empty"),
        ));
    }

    if value.trim() != value {
        return Err(validation_error(
            format!("{description} contains surrounding whitespace"),
        ));
    }

    Ok(())
}

// =============================================================================
// SPECIALIZATION EVIDENCE
// =============================================================================

/// Opaque evidence supplied by canonical compiler, semantic,
/// capability and compatibility authorities.
///
/// This type deliberately does not reinterpret evidence.
///
/// The injected specialization authority determines whether
/// the evidence is genuine, current, applicable and sufficient.
///
/// Fingerprints must be stable and must identify all relevant
/// assumptions and evidence used by the proposed specialization.
///
/// Implementations must not use a display string or a non-unique
/// human-readable name as a substitute for a canonical fingerprint.
pub trait SpecializationEvidence: fmt::Debug + Send + Sync {
    /// Returns the stable identity of this evidence bundle.
    fn evidence_identity(&self) -> &str;

    /// Returns the fingerprint of the proposed specialization assumptions.
    fn assumption_fingerprint(&self) -> &str;

    /// Returns the identity of the capability evidence.
    fn capability_evidence_identity(&self) -> &str;

    /// Returns the identity of the target-compatibility evidence.
    fn target_compatibility_identity(&self) -> &str;

    /// Returns the semantic contract fingerprint.
    fn semantic_contract_identity(&self) -> &str;

    /// Returns the canonical compiler-pipeline identity.
    fn compiler_pipeline_identity(&self) -> &str;
}

// =============================================================================
// SPECIALIZATION CANDIDATE
// =============================================================================

/// Immutable input describing one proposed specialization.
///
/// A candidate is not yet an approved specialization.
///
/// Its evidence must be checked by the injected
/// [`SpecializationAuthority`].
///
/// The specialized cache key must be generated by the canonical
/// compiler integration and must include every input that can
/// affect the resulting artifact.
///
/// It must account for applicable items such as:
///
/// - Original compilation-unit identity.
/// - Semantic contract.
/// - Compiler contract version.
/// - Compiler pipeline.
/// - Optimization configuration.
/// - Specialization assumptions.
/// - Guard strategy.
/// - Capability evidence.
/// - Target compatibility.
/// - Backend/provider identity.
/// - Runtime ABI compatibility.
///
/// This module does not construct or hash that identity.
#[derive(Clone)]
pub struct SpecializationCandidate {
    unit: Arc<dyn JitCompilationUnit>,

    baseline_key: JitCacheKey,

    specialized_key: JitCacheKey,

    hotness: HotnessDecision,

    evidence: Arc<dyn SpecializationEvidence>,
}

impl fmt::Debug for SpecializationCandidate {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("SpecializationCandidate")
            .field(
                "unit_identity",
                &self.unit.stable_identity(),
            )
            .field(
                "domain_identity",
                &self.unit.domain_identity(),
            )
            .field(
                "compiler_contract_version",
                &self.unit.compiler_contract_version(),
            )
            .field("baseline_key", &self.baseline_key)
            .field("specialized_key", &self.specialized_key)
            .field("hotness", &self.hotness)
            .field("evidence", &self.evidence)
            .finish()
    }
}

impl SpecializationCandidate {
    /// Creates a specialization candidate.
    ///
    /// The candidate is rejected unless:
    ///
    /// - The compilation-unit identity is valid.
    /// - Its domain identity matches the hotness observation.
    /// - Its baseline key matches the hotness observation.
    /// - Hotness has been established.
    /// - The specialized cache key differs from the baseline key.
    /// - All required evidence identities are present.
    ///
    /// These checks establish structural consistency only.
    ///
    /// They do not replace compiler, semantic, capability or
    /// target-compatibility validation.
    pub fn new(
        unit: Arc<dyn JitCompilationUnit>,
        baseline_key: JitCacheKey,
        specialized_key: JitCacheKey,
        hotness: HotnessDecision,
        evidence: Arc<dyn SpecializationEvidence>,
    ) -> ZurResult<Self> {
        validate_identity(
            unit.stable_identity(),
            "compilation-unit identity",
        )?;

        validate_identity(
            unit.domain_identity(),
            "computational-domain identity",
        )?;

        validate_identity(
            unit.compiler_contract_version(),
            "compiler contract version",
        )?;

        if hotness.status() != HotnessStatus::Hot {
            return Err(validation_error(
                "specialization requires an established hotness decision",
            ));
        }

        if hotness.key() != &baseline_key {
            return Err(compatibility_error(
                "hotness evidence does not belong to the supplied baseline key",
            ));
        }

        if hotness.domain_identity() != unit.domain_identity() {
            return Err(compatibility_error(
                "hotness domain does not match the compilation unit",
            ));
        }

        if baseline_key == specialized_key {
            return Err(validation_error(
                "specialized cache identity must differ from the baseline identity",
            ));
        }

        validate_identity(
            evidence.evidence_identity(),
            "specialization evidence identity",
        )?;

        validate_identity(
            evidence.assumption_fingerprint(),
            "specialization assumption fingerprint",
        )?;

        validate_identity(
            evidence.capability_evidence_identity(),
            "capability evidence identity",
        )?;

        validate_identity(
            evidence.target_compatibility_identity(),
            "target compatibility identity",
        )?;

        validate_identity(
            evidence.semantic_contract_identity(),
            "semantic contract identity",
        )?;

        validate_identity(
            evidence.compiler_pipeline_identity(),
            "compiler pipeline identity",
        )?;

        Ok(Self {
            unit,
            baseline_key,
            specialized_key,
            hotness,
            evidence,
        })
    }

    /// Returns the compiler-owned compilation unit.
    #[must_use]
    pub fn unit(&self) -> &Arc<dyn JitCompilationUnit> {
        &self.unit
    }

    /// Returns the original baseline cache identity.
    #[must_use]
    pub fn baseline_key(&self) -> &JitCacheKey {
        &self.baseline_key
    }

    /// Returns the proposed specialized cache identity.
    #[must_use]
    pub fn specialized_key(&self) -> &JitCacheKey {
        &self.specialized_key
    }

    /// Returns the hotness evidence used to propose specialization.
    #[must_use]
    pub fn hotness(&self) -> &HotnessDecision {
        &self.hotness
    }

    /// Returns the opaque canonical specialization evidence.
    #[must_use]
    pub fn evidence(&self) -> &Arc<dyn SpecializationEvidence> {
        &self.evidence
    }
}

// =============================================================================
// SPECIALIZATION CERTIFICATE
// =============================================================================

/// Evidence that an injected specialization authority has validated
/// a proposed specialization.
///
/// A certificate is an authority-issued contract, not a substitute
/// for cryptographic authentication.
///
/// Its implementation must be backed by the actual compiler,
/// semantic, capability, compatibility and verification authorities.
///
/// The coordinator checks that the certificate refers to the exact
/// candidate being prepared.
///
/// A certificate must not approve:
///
/// - Unvalidated assumptions.
/// - Stale capability evidence.
/// - Incompatible target evidence.
/// - Unverifiable runtime guards.
/// - Semantic changes forbidden by the source contract.
/// - A baseline fallback that is not semantically compatible.
pub trait SpecializationCertificate: fmt::Debug + Send + Sync {
    /// Returns the baseline cache identity covered by the certificate.
    fn baseline_key(&self) -> &JitCacheKey;

    /// Returns the specialized cache identity covered by the certificate.
    fn specialized_key(&self) -> &JitCacheKey;

    /// Returns the compilation-unit identity covered by the certificate.
    fn compilation_unit_identity(&self) -> &str;

    /// Returns the computational-domain identity covered by the certificate.
    fn domain_identity(&self) -> &str;

    /// Returns the compiler contract version covered by the certificate.
    fn compiler_contract_version(&self) -> &str;

    /// Returns the evidence identity covered by the certificate.
    fn evidence_identity(&self) -> &str;

    /// Returns the assumption fingerprint covered by the certificate.
    fn assumption_fingerprint(&self) -> &str;

    /// Returns the capability evidence identity covered by the certificate.
    fn capability_evidence_identity(&self) -> &str;

    /// Returns the target compatibility identity covered by the certificate.
    fn target_compatibility_identity(&self) -> &str;

    /// Returns the semantic contract identity covered by the certificate.
    fn semantic_contract_identity(&self) -> &str;

    /// Returns the compiler pipeline identity covered by the certificate.
    fn compiler_pipeline_identity(&self) -> &str;

    /// Returns the identity of the runtime guard set.
    ///
    /// This must identify the actual guards that protect the
    /// specialization assumptions.
    fn guard_set_identity(&self) -> &str;

    /// Returns the identity of the authority issuing the certificate.
    fn authority_identity(&self) -> &str;

    /// Returns whether semantic preservation has been verified.
    fn semantic_preservation_verified(&self) -> bool;

    /// Returns whether all applicable specialization assumptions
    /// are protected by valid guards or an equivalent verified mechanism.
    fn assumptions_guarded(&self) -> bool;

    /// Returns whether the supplied capability evidence was validated.
    fn capability_evidence_validated(&self) -> bool;

    /// Returns whether target compatibility was validated.
    fn target_compatibility_validated(&self) -> bool;

    /// Returns whether the baseline fallback is semantically compatible.
    ///
    /// This establishes compatibility, not that the baseline artifact
    /// is currently resident or executable.
    fn baseline_fallback_validated(&self) -> bool;
}

// =============================================================================
// SPECIALIZATION AUTHORITY
// =============================================================================

/// Validates proposed specialization using canonical repository authorities.
///
/// An implementation must integrate with the relevant existing systems,
/// including where applicable:
///
/// - Frontend semantic contracts.
/// - Compiler optimization and preservation obligations.
/// - Compiler IR verification.
/// - Canonical capability models.
/// - Backend compatibility evidence.
/// - Target compatibility analysis.
/// - Runtime security policy.
/// - Deoptimization and guard contracts.
///
/// It must not implement another compiler or infer hardware feasibility.
///
/// It must validate the actual evidence supplied in the candidate.
///
/// Returning a certificate means the implementation has genuinely
/// completed the validations represented by that certificate.
///
/// Implementations are trusted integration components. This trait
/// cannot mechanically prove that a custom implementation is honest.
pub trait SpecializationAuthority: fmt::Debug + Send + Sync {
    /// Validates a candidate and returns an authority-issued certificate.
    ///
    /// Errors must be propagated when evidence is absent, stale,
    /// contradictory, incompatible or insufficient.
    fn validate(
        &self,
        candidate: &SpecializationCandidate,
    ) -> ZurResult<Arc<dyn SpecializationCertificate>>;
}

// =============================================================================
// PREPARED SPECIALIZATION
// =============================================================================

/// A specialization request approved for submission to the existing
/// JIT engine.
///
/// This object does not represent compiled or executable code.
///
/// It contains:
///
/// - The canonical JIT compilation request.
/// - The validated specialization certificate.
/// - The baseline identity needed by downstream fallback integration.
///
/// The caller must still submit the request to the existing JIT engine.
/// The engine remains responsible for compilation, verification,
/// cache publication and compilation outcomes.
pub struct PreparedSpecialization {
    request: JitCompilationRequest,

    certificate: Arc<dyn SpecializationCertificate>,

    baseline_key: JitCacheKey,
}

impl fmt::Debug for PreparedSpecialization {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("PreparedSpecialization")
            .field("request", &self.request)
            .field("certificate", &self.certificate)
            .field("baseline_key", &self.baseline_key)
            .finish()
    }
}

impl PreparedSpecialization {
    /// Returns the canonical JIT compilation request.
    #[must_use]
    pub fn request(&self) -> &JitCompilationRequest {
        &self.request
    }

    /// Returns the authority-issued validation certificate.
    #[must_use]
    pub fn certificate(&self) -> &Arc<dyn SpecializationCertificate> {
        &self.certificate
    }

    /// Returns the original baseline cache identity.
    #[must_use]
    pub fn baseline_key(&self) -> &JitCacheKey {
        &self.baseline_key
    }

    /// Consumes the prepared specialization and returns the
    /// canonical JIT compilation request.
    ///
    /// The caller should retain the certificate and baseline identity
    /// through the surrounding execution/deoptimization lifecycle
    /// when those are needed by its integration.
    #[must_use]
    pub fn into_request(self) -> JitCompilationRequest {
        self.request
    }

    /// Consumes the prepared specialization and returns its parts.
    ///
    /// This is useful when the caller must retain the certificate
    /// and baseline identity alongside the JIT request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        JitCompilationRequest,
        Arc<dyn SpecializationCertificate>,
        JitCacheKey,
    ) {
        (
            self.request,
            self.certificate,
            self.baseline_key,
        )
    }
}

// =============================================================================
// SPECIALIZATION COORDINATOR
// =============================================================================

/// Coordinates validation and preparation of JIT specialization requests.
///
/// This component is immutable after construction.
///
/// It creates no worker threads, timers, global state or background tasks.
///
/// It does not compile or execute programs.
#[derive(Clone)]
pub struct SpecializationCoordinator {
    authority: Arc<dyn SpecializationAuthority>,
}

impl fmt::Debug for SpecializationCoordinator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("SpecializationCoordinator")
            .field("authority", &self.authority)
            .finish()
    }
}

impl SpecializationCoordinator {
    /// Creates a coordinator with an explicitly supplied validation authority.
    ///
    /// There is intentionally no default authority.
    ///
    /// Specialization must not proceed merely because a caller
    /// requests it.
    #[must_use]
    pub fn new(
        authority: Arc<dyn SpecializationAuthority>,
    ) -> Self {
        Self { authority }
    }

    /// Returns the configured specialization authority.
    #[must_use]
    pub fn authority(&self) -> &Arc<dyn SpecializationAuthority> {
        &self.authority
    }

    /// Validates and prepares a specialization candidate.
    ///
    /// The authority must validate semantic preservation,
    /// assumption guards, capability evidence, target compatibility
    /// and baseline fallback compatibility.
    ///
    /// The returned request uses the existing engine's
    /// `JitCompilationReason::Specialization`.
    ///
    /// No compilation, cache insertion, resource reservation,
    /// or execution occurs in this method.
    pub fn prepare(
        &self,
        candidate: SpecializationCandidate,
    ) -> ZurResult<PreparedSpecialization> {
        let certificate = self.authority.validate(&candidate)?;

        Self::validate_certificate(
            &candidate,
            certificate.as_ref(),
        )?;

        let request = JitCompilationRequest::new(
            Arc::clone(&candidate.unit),
            candidate.specialized_key.clone(),
            JitCompilationReason::Specialization,
        )?;

        Ok(PreparedSpecialization {
            request,
            certificate,
            baseline_key: candidate.baseline_key.clone(),
        })
    }

    /// Validates that a certificate is bound to the exact candidate.
    ///
    /// A certificate for another compilation unit, another cache key,
    /// another compiler version or another evidence bundle is rejected.
    fn validate_certificate(
        candidate: &SpecializationCandidate,
        certificate: &dyn SpecializationCertificate,
    ) -> ZurResult<()> {
        if certificate.baseline_key() != &candidate.baseline_key {
            return Err(compatibility_error(
                "specialization certificate refers to another baseline",
            ));
        }

        if certificate.specialized_key() != &candidate.specialized_key {
            return Err(compatibility_error(
                "specialization certificate refers to another specialized cache identity",
            ));
        }

        if certificate.compilation_unit_identity()
            != candidate.unit.stable_identity()
        {
            return Err(compatibility_error(
                "specialization certificate refers to another compilation unit",
            ));
        }

        if certificate.domain_identity()
            != candidate.unit.domain_identity()
        {
            return Err(compatibility_error(
                "specialization certificate has a different computational domain",
            ));
        }

        if certificate.compiler_contract_version()
            != candidate.unit.compiler_contract_version()
        {
            return Err(compatibility_error(
                "specialization certificate has a different compiler contract version",
            ));
        }

        let evidence = candidate.evidence.as_ref();

        if certificate.evidence_identity()
            != evidence.evidence_identity()
        {
            return Err(compatibility_error(
                "specialization certificate does not bind the supplied evidence",
            ));
        }

        if certificate.assumption_fingerprint()
            != evidence.assumption_fingerprint()
        {
            return Err(compatibility_error(
                "specialization certificate has different assumptions",
            ));
        }

        if certificate.capability_evidence_identity()
            != evidence.capability_evidence_identity()
        {
            return Err(compatibility_error(
                "specialization certificate has different capability evidence",
            ));
        }

        if certificate.target_compatibility_identity()
            != evidence.target_compatibility_identity()
        {
            return Err(compatibility_error(
                "specialization certificate has different target compatibility evidence",
            ));
        }

        if certificate.semantic_contract_identity()
            != evidence.semantic_contract_identity()
        {
            return Err(compatibility_error(
                "specialization certificate has a different semantic contract",
            ));
        }

        if certificate.compiler_pipeline_identity()
            != evidence.compiler_pipeline_identity()
        {
            return Err(compatibility_error(
                "specialization certificate has a different compiler pipeline",
            ));
        }

        validate_identity(
            certificate.guard_set_identity(),
            "specialization guard-set identity",
        )?;

        validate_identity(
            certificate.authority_identity(),
            "specialization authority identity",
        )?;

        if !certificate.semantic_preservation_verified() {
            return Err(contract_error(
                "semantic preservation was not verified",
            ));
        }

        if !certificate.assumptions_guarded() {
            return Err(contract_error(
                "specialization assumptions are not protected by verified guards",
            ));
        }

        if !certificate.capability_evidence_validated() {
            return Err(contract_error(
                "specialization capability evidence was not validated",
            ));
        }

        if !certificate.target_compatibility_validated() {
            return Err(contract_error(
                "specialization target compatibility was not validated",
            ));
        }

        if !certificate.baseline_fallback_validated() {
            return Err(contract_error(
                "baseline fallback compatibility was not validated",
            ));
        }

        Ok(())
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::num::NonZeroU128;

    use crate::zur::jit::hotness::{
        HotnessPolicy,
        HotnessTracker,
    };

    use crate::zur::jit::profiling::{
        JitInvocationOutcome,
        JitProfileCollector,
    };

    #[derive(Debug)]
    struct TestCompilationUnit;

    impl JitCompilationUnit for TestCompilationUnit {
        fn stable_identity(&self) -> &str {
            "zamani.test.module.function"
        }

        fn domain_identity(&self) -> &str {
            "zamani.classical"
        }

        fn compiler_contract_version(&self) -> &str {
            "1.0.0"
        }
    }

    #[derive(Debug)]
    struct TestEvidence;

    impl SpecializationEvidence for TestEvidence {
        fn evidence_identity(&self) -> &str {
            "test.evidence.bundle"
        }

        fn assumption_fingerprint(&self) -> &str {
            "sha256:test.assumptions"
        }

        fn capability_evidence_identity(&self) -> &str {
            "test.capability.evidence"
        }

        fn target_compatibility_identity(&self) -> &str {
            "test.target.compatibility"
        }

        fn semantic_contract_identity(&self) -> &str {
            "test.semantic.contract"
        }

        fn compiler_pipeline_identity(&self) -> &str {
            "test.compiler.pipeline"
        }
    }

    #[derive(Debug)]
    struct TestCertificate {
        baseline_key: JitCacheKey,
        specialized_key: JitCacheKey,
    }

    impl SpecializationCertificate for TestCertificate {
        fn baseline_key(&self) -> &JitCacheKey {
            &self.baseline_key
        }

        fn specialized_key(&self) -> &JitCacheKey {
            &self.specialized_key
        }

        fn compilation_unit_identity(&self) -> &str {
            "zamani.test.module.function"
        }

        fn domain_identity(&self) -> &str {
            "zamani.classical"
        }

        fn compiler_contract_version(&self) -> &str {
            "1.0.0"
        }

        fn evidence_identity(&self) -> &str {
            "test.evidence.bundle"
        }

        fn assumption_fingerprint(&self) -> &str {
            "sha256:test.assumptions"
        }

        fn capability_evidence_identity(&self) -> &str {
            "test.capability.evidence"
        }

        fn target_compatibility_identity(&self) -> &str {
            "test.target.compatibility"
        }

        fn semantic_contract_identity(&self) -> &str {
            "test.semantic.contract"
        }

        fn compiler_pipeline_identity(&self) -> &str {
            "test.compiler.pipeline"
        }

        fn guard_set_identity(&self) -> &str {
            "test.guard.set"
        }

        fn authority_identity(&self) -> &str {
            "test.specialization.authority"
        }

        fn semantic_preservation_verified(&self) -> bool {
            true
        }

        fn assumptions_guarded(&self) -> bool {
            true
        }

        fn capability_evidence_validated(&self) -> bool {
            true
        }

        fn target_compatibility_validated(&self) -> bool {
            true
        }

        fn baseline_fallback_validated(&self) -> bool {
            true
        }
    }

    #[derive(Debug)]
    struct TestAuthority;

    impl SpecializationAuthority for TestAuthority {
        fn validate(
            &self,
            candidate: &SpecializationCandidate,
        ) -> ZurResult<Arc<dyn SpecializationCertificate>> {
            Ok(Arc::new(TestCertificate {
                baseline_key: candidate.baseline_key().clone(),
                specialized_key: candidate.specialized_key().clone(),
            }))
        }
    }

    fn key(value: &str) -> JitCacheKey {
        JitCacheKey::new(value)
            .expect("test cache key must be valid")
    }

    fn hotness_decision() -> HotnessDecision {
        let collector = JitProfileCollector::default();

        let baseline_key = key("sha256:test.baseline");

        let tracker = HotnessTracker::new(
            HotnessPolicy::new(
                NonZeroU128::new(1)
                    .expect("one is nonzero"),
            ),
        )
        .expect("valid hotness tracker");

        tracker
            .observe(
                &collector.snapshot()
                    .expect("initial snapshot"),
            )
            .expect("initial baseline");

        let token = collector
            .begin(
                &baseline_key,
                "zamani.classical",
            )
            .expect("begin invocation");

        collector
            .finish(
                token,
                JitInvocationOutcome::Succeeded,
            )
            .expect("finish invocation");

        let decisions = tracker
            .observe(
                &collector.snapshot()
                    .expect("updated snapshot"),
            )
            .expect("hotness evaluation");

        decisions
            .into_iter()
            .next()
            .expect("one hotness decision")
    }

    fn candidate() -> SpecializationCandidate {
        SpecializationCandidate::new(
            Arc::new(TestCompilationUnit),
            key("sha256:test.baseline"),
            key("sha256:test.specialized"),
            hotness_decision(),
            Arc::new(TestEvidence),
        )
        .expect("valid specialization candidate")
    }

    #[test]
    fn api_version_is_available() {
        assert_eq!(JIT_SPECIALIZATION_API_VERSION, 1);
    }

    #[test]
    fn rejects_specialization_without_hotness_evidence() {
        let collector = JitProfileCollector::default();

        let tracker = HotnessTracker::new(
            HotnessPolicy::new(
                NonZeroU128::new(1)
                    .expect("one is nonzero"),
            ),
        )
        .expect("tracker");

        let decision = tracker
            .observe(
                &collector.snapshot()
                    .expect("snapshot"),
            )
            .expect("observation");

        assert!(decision.is_empty());
    }

    #[test]
    fn rejects_identical_baseline_and_specialized_keys() {
        let decision = hotness_decision();

        let result = SpecializationCandidate::new(
            Arc::new(TestCompilationUnit),
            key("sha256:same"),
            key("sha256:same"),
            decision,
            Arc::new(TestEvidence),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_hotness_for_another_baseline() {
        let decision = hotness_decision();

        let result = SpecializationCandidate::new(
            Arc::new(TestCompilationUnit),
            key("sha256:another.baseline"),
            key("sha256:test.specialized"),
            decision,
            Arc::new(TestEvidence),
        );

        assert!(result.is_err());
    }

    #[test]
    fn prepares_canonical_jit_specialization_request() {
        let coordinator = SpecializationCoordinator::new(
            Arc::new(TestAuthority),
        );

        let prepared = coordinator
            .prepare(candidate())
            .expect("preparation succeeds");

        assert_eq!(
            prepared.request().reason(),
            JitCompilationReason::Specialization,
        );

        assert_eq!(
            prepared.request().cache_key().as_str(),
            "sha256:test.specialized",
        );

        assert_eq!(
            prepared.baseline_key().as_str(),
            "sha256:test.baseline",
        );
    }

    #[test]
    fn certificate_remains_available_with_request() {
        let coordinator = SpecializationCoordinator::new(
            Arc::new(TestAuthority),
        );

        let prepared = coordinator
            .prepare(candidate())
            .expect("preparation succeeds");

        assert_eq!(
            prepared.certificate().authority_identity(),
            "test.specialization.authority",
        );

        let (request, certificate, baseline_key) =
            prepared.into_parts();

        assert_eq!(
            request.reason(),
            JitCompilationReason::Specialization,
        );

        assert_eq!(
            certificate.guard_set_identity(),
            "test.guard.set",
        );

        assert_eq!(
            baseline_key.as_str(),
            "sha256:test.baseline",
        );
    }

    #[test]
    fn rejects_a_certificate_for_another_specialized_key() {
        #[derive(Debug)]
        struct IncorrectAuthority;

        impl SpecializationAuthority for IncorrectAuthority {
            fn validate(
                &self,
                candidate: &SpecializationCandidate,
            ) -> ZurResult<Arc<dyn SpecializationCertificate>> {
                Ok(Arc::new(TestCertificate {
                    baseline_key: candidate.baseline_key().clone(),
                    specialized_key: key("sha256:wrong.specialization"),
                }))
            }
        }

        let coordinator = SpecializationCoordinator::new(
            Arc::new(IncorrectAuthority),
        );

        assert!(coordinator.prepare(candidate()).is_err());
    }

    #[test]
    fn rejects_unverified_semantic_preservation() {
        #[derive(Debug)]
        struct InvalidCertificate {
            baseline_key: JitCacheKey,
            specialized_key: JitCacheKey,
        }

        impl SpecializationCertificate for InvalidCertificate {
            fn baseline_key(&self) -> &JitCacheKey {
                &self.baseline_key
            }

            fn specialized_key(&self) -> &JitCacheKey {
                &self.specialized_key
            }

            fn compilation_unit_identity(&self) -> &str {
                "zamani.test.module.function"
            }

            fn domain_identity(&self) -> &str {
                "zamani.classical"
            }

            fn compiler_contract_version(&self) -> &str {
                "1.0.0"
            }

            fn evidence_identity(&self) -> &str {
                "test.evidence.bundle"
            }

            fn assumption_fingerprint(&self) -> &str {
                "sha256:test.assumptions"
            }

            fn capability_evidence_identity(&self) -> &str {
                "test.capability.evidence"
            }

            fn target_compatibility_identity(&self) -> &str {
                "test.target.compatibility"
            }

            fn semantic_contract_identity(&self) -> &str {
                "test.semantic.contract"
            }

            fn compiler_pipeline_identity(&self) -> &str {
                "test.compiler.pipeline"
            }

            fn guard_set_identity(&self) -> &str {
                "test.guard.set"
            }

            fn authority_identity(&self) -> &str {
                "test.invalid.authority"
            }

            fn semantic_preservation_verified(&self) -> bool {
                false
            }

            fn assumptions_guarded(&self) -> bool {
                true
            }

            fn capability_evidence_validated(&self) -> bool {
                true
            }

            fn target_compatibility_validated(&self) -> bool {
                true
            }

            fn baseline_fallback_validated(&self) -> bool {
                true
            }
        }

        #[derive(Debug)]
        struct InvalidAuthority;

        impl SpecializationAuthority for InvalidAuthority {
            fn validate(
                &self,
                candidate: &SpecializationCandidate,
            ) -> ZurResult<Arc<dyn SpecializationCertificate>> {
                Ok(Arc::new(InvalidCertificate {
                    baseline_key: candidate.baseline_key().clone(),
                    specialized_key: candidate.specialized_key().clone(),
                }))
            }
        }

        let coordinator = SpecializationCoordinator::new(
            Arc::new(InvalidAuthority),
        );

        assert!(coordinator.prepare(candidate()).is_err());
    }
}
