
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Universal Runtime — JIT Deoptimization
//!
//! Path: src/zur/jit/deoptimization.rs
//! Edition: Rust 2021
//! Minimum Rust: 1.97+
//! Safety: Safe Rust only.
//!
//! # PURPOSE
//!
//! Provides the validated coordination boundary for transferring an
//! executing specialized JIT computation to a semantically compatible
//! baseline execution path.
//!
//! Deoptimization may be requested when:
//!
//! - A specialization guard fails.
//! - A specialization assumption becomes invalid.
//! - Specialized code is invalidated.
//! - A target or capability assumption is no longer valid.
//! - A runtime execution constraint requires another realization.
//! - An explicit execution policy requests deoptimization.
//! - A debugger or authorized execution controller requests it.
//!
//! # OWNERSHIP
//!
//! This module owns:
//!
//! - Deoptimization request contracts.
//! - Deoptimization evidence contracts.
//! - Execution-state snapshot contracts.
//! - Deoptimization validation certificates.
//! - Baseline artifact resolution contracts.
//! - State reconstruction validation contracts.
//! - Atomic baseline handoff contracts.
//! - Deoptimization coordination.
//! - Structured deoptimization outcomes.
//!
//! This module does NOT own:
//!
//! - Source parsing.
//! - AST construction.
//! - Semantic analysis.
//! - Source-level optimization intent.
//! - Requirements, capabilities or constraints.
//! - Classical IR.
//! - Quantum IR.
//! - HDL IR.
//! - ZUIR.
//! - Compiler optimization.
//! - Compiler lowering.
//! - Specialization.
//! - JIT compilation.
//! - Code generation.
//! - Executable memory.
//! - Code-cache management.
//! - Code invalidation.
//! - Runtime scheduling.
//! - Physical execution.
//! - Domain-specific execution.
//!
//! Those responsibilities remain with their existing authorities.
//!
//! # CANONICAL INTEGRATION
//!
//! JIT engine:
//!     crate::zur::jit::engine
//!
//! Specialization:
//!     crate::zur::jit::specialization
//!
//! Compilation identity:
//!     crate::zur::jit::engine::JitCacheKey
//!
//! Compiled artifacts:
//!     crate::zur::jit::engine::CompiledJitArtifact
//!
//! Cancellation:
//!     crate::zur::jit::engine::JitCancellation
//!
//! Compiler:
//!     crate::compiler
//!
//! Compiler optimization:
//!     crate::compiler::optimization
//!
//! Execution:
//!     crate::zur::execution
//!
//! Invalidation:
//!     crate::zur::jit::invalidation
//!
//! Security:
//!     crate::zur::security
//!
//! Observability:
//!     crate::zur::observability
//!
//! # GRAMMAR BOUNDARY
//!
//! Relevant grammar:
//!
//!     grammar/compile/optimization.g4
//!     grammar/execution/runtime.g4
//!     grammar/resources/portability.g4
//!
//! Relevant specifications:
//!
//!     grammar/spec/portability.md
//!     grammar/specification/semantic-model.md
//!     grammar/specification/execution-model.md
//!
//! Grammar and semantic authorities establish program meaning,
//! optimization obligations, execution requirements and portability.
//!
//! This module receives validated execution contracts.
//!
//! It never parses grammar or interprets AST nodes.
//!
//! # SCALABILITY
//!
//! No arbitrary limits are imposed on:
//!
//! - Computational domains.
//! - Compilation units.
//! - Execution sessions.
//! - Specialization assumptions.
//! - State snapshots.
//! - Providers.
//! - Devices.
//! - Hardware architectures.
//! - Resource dimensions.
//!
//! Actual storage and processing are constrained by available resources.
//!
//! Literal infinite computation is not promised.
//!
//! # SAFETY
//!
//! - No unsafe Rust.
//! - No raw pointers.
//! - No global mutable state.
//! - No fabricated execution success.
//! - No fabricated baseline compatibility.
//! - No unverified state transfer.
//! - No implicit compilation.
//! - No implicit execution.
//! - No automatic code-cache invalidation.
//!
//! # IMPORTANT INVARIANTS
//!
//! 1. The original program semantics remain authoritative.
//! 2. Specialized and baseline identities must be different.
//! 3. The specialization certificate must match the exact request.
//! 4. The baseline artifact must be independently verified.
//! 5. State reconstruction must be validated.
//! 6. Observable effects must remain semantically correct.
//! 7. The handoff must have an explicit atomicity contract.
//! 8. Failed handoffs must never be reported as successful.
//! 9. Deoptimization does not imply baseline execution completed.
//! 10. Code invalidation remains independently owned.
//! 11. Cancellation must be checked before committing the handoff.
//! 12. Domain identities remain open-ended.

use std::fmt;
use std::sync::Arc;

use crate::zur::error::{ZurError, ZurResult};

use crate::zur::jit::engine::{
    CompiledJitArtifact,
    JitCacheKey,
    JitCancellation,
    JitCompilationUnit,
    NeverCancelled,
};

use crate::zur::jit::specialization::{
    SpecializationCertificate,
};

// =============================================================================
// API VERSION
// =============================================================================

/// Public API version for ZUR JIT deoptimization.
pub const JIT_DEOPTIMIZATION_API_VERSION: u16 = 1;

/// Component identity used in structured errors.
const COMPONENT: &str = "zur.jit.deoptimization";

// =============================================================================
// ERROR CONSTRUCTION
// =============================================================================

/// Constructs a deoptimization validation error.
fn validation_error(message: impl Into<String>) -> ZurError {
    ZurError::invalid_input(message)
        .with_component(COMPONENT)
        .with_operation("validate_deoptimization")
}

/// Constructs a deoptimization compatibility error.
fn compatibility_error(message: impl Into<String>) -> ZurError {
    ZurError::compatibility(message)
        .with_component(COMPONENT)
        .with_operation("validate_deoptimization")
}

/// Constructs a deoptimization contract error.
fn contract_error(message: impl Into<String>) -> ZurError {
    ZurError::internal(message)
        .with_component(COMPONENT)
        .with_operation("validate_deoptimization_contract")
}

/// Constructs a handoff error.
fn handoff_error(message: impl Into<String>) -> ZurError {
    ZurError::service_unavailable(message)
        .with_component(COMPONENT)
        .with_operation("baseline_handoff")
}

/// Validates an identity without imposing an arbitrary length limit.
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
// DEOPTIMIZATION REASON
// =============================================================================

/// Explains why specialized execution must be left.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeoptimizationReason {
    /// A runtime specialization guard failed.
    GuardFailure {
        /// Identity of the failed guard.
        guard_identity: String,
    },

    /// A specialization assumption is no longer valid.
    AssumptionInvalidated {
        /// Identity of the invalidated assumption.
        assumption_identity: String,
    },

    /// The specialized artifact was invalidated.
    CodeInvalidated {
        /// Identity of the invalidation event.
        invalidation_identity: String,
    },

    /// Target compatibility is no longer established.
    TargetIncompatibility {
        /// Identity of the compatibility failure.
        evidence_identity: String,
    },

    /// A runtime resource or execution constraint requires a transition.
    ExecutionConstraint {
        /// Identity of the constraint evidence.
        constraint_identity: String,
    },

    /// An authorized controller explicitly requested deoptimization.
    ExplicitRequest {
        /// Identity of the requesting authority.
        authority_identity: String,
    },

    /// A debugger requested a baseline execution transition.
    DebuggerRequest {
        /// Identity of the debugger request.
        request_identity: String,
    },
}

impl DeoptimizationReason {
    /// Returns the identity associated with the reason.
    #[must_use]
    pub fn evidence_identity(&self) -> &str {
        match self {
            Self::GuardFailure { guard_identity } => guard_identity,

            Self::AssumptionInvalidated {
                assumption_identity,
            } => assumption_identity,

            Self::CodeInvalidated {
                invalidation_identity,
            } => invalidation_identity,

            Self::TargetIncompatibility {
                evidence_identity,
            } => evidence_identity,

            Self::ExecutionConstraint {
                constraint_identity,
            } => constraint_identity,

            Self::ExplicitRequest {
                authority_identity,
            } => authority_identity,

            Self::DebuggerRequest {
                request_identity,
            } => request_identity,
        }
    }

    /// Validates the evidence identity.
    pub fn validate(&self) -> ZurResult<()> {
        validate_identity(
            self.evidence_identity(),
            "deoptimization reason evidence identity",
        )
    }
}

// =============================================================================
// EXECUTION STATE SNAPSHOT
// =============================================================================

/// Immutable execution-state snapshot supplied by the execution authority.
///
/// Implementations must capture all state required by the applicable
/// baseline reconstruction contract.
///
/// The snapshot may represent:
///
/// - Classical execution state.
/// - Register and stack state.
/// - Runtime-managed values.
/// - Domain-specific execution state.
/// - Quantum execution session state where legally transferable.
/// - Hybrid execution state.
/// - Provider-owned state represented through a safe opaque handle.
///
/// A snapshot is not automatically portable merely because it implements
/// this trait.
///
/// The deoptimization authority must validate its compatibility with
/// the baseline execution contract.
///
/// Implementations must not expose raw pointers or unsafe operations.
pub trait DeoptimizationState: fmt::Debug + Send + Sync {
    /// Returns the stable identity of this snapshot.
    fn snapshot_identity(&self) -> &str;

    /// Returns the identity of the state representation contract.
    fn schema_identity(&self) -> &str;

    /// Returns a canonical fingerprint of the captured state.
    fn state_fingerprint(&self) -> &str;

    /// Returns whether the snapshot is complete according to its
    /// producing execution authority.
    fn is_complete(&self) -> bool;
}

// =============================================================================
// DEOPTIMIZATION EVIDENCE
// =============================================================================

/// Opaque evidence associated with a deoptimization event.
///
/// The evidence producer must be the authoritative execution,
/// invalidation, compatibility or debugging subsystem.
///
/// A human-readable message is not sufficient evidence.
///
/// The evidence must identify the actual event and the assumptions
/// affected by it.
pub trait DeoptimizationEvidence: fmt::Debug + Send + Sync {
    /// Returns the canonical evidence identity.
    fn evidence_identity(&self) -> &str;

    /// Returns the identity of the affected assumption set.
    fn affected_assumptions_identity(&self) -> &str;

    /// Returns the identity of the execution observation.
    fn execution_observation_identity(&self) -> &str;

    /// Returns the identity of the authority producing the evidence.
    fn authority_identity(&self) -> &str;
}

// =============================================================================
// DEOPTIMIZATION REQUEST
// =============================================================================

/// Immutable request to leave one specialized execution path.
///
/// The request binds:
///
/// - The original compiler-owned unit.
/// - The specialized artifact identity.
/// - The baseline artifact identity.
/// - The validated specialization certificate.
/// - The execution-state snapshot.
/// - The deoptimization reason.
/// - The deoptimization evidence.
///
/// Construction performs structural checks only.
///
/// The injected authority must still establish that the requested
/// transition is semantically valid.
#[derive(Clone)]
pub struct DeoptimizationRequest {
    unit: Arc<dyn JitCompilationUnit>,

    specialized_key: JitCacheKey,

    baseline_key: JitCacheKey,

    specialization: Arc<dyn SpecializationCertificate>,

    state: Arc<dyn DeoptimizationState>,

    reason: DeoptimizationReason,

    evidence: Arc<dyn DeoptimizationEvidence>,
}

impl fmt::Debug for DeoptimizationRequest {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("DeoptimizationRequest")
            .field("unit_identity", &self.unit.stable_identity())
            .field("domain_identity", &self.unit.domain_identity())
            .field("specialized_key", &self.specialized_key)
            .field("baseline_key", &self.baseline_key)
            .field("specialization", &self.specialization)
            .field("state", &self.state)
            .field("reason", &self.reason)
            .field("evidence", &self.evidence)
            .finish()
    }
}

impl DeoptimizationRequest {
    /// Constructs a structurally validated deoptimization request.
    ///
    /// The specialized and baseline cache identities must differ.
    ///
    /// The specialization certificate must describe the exact same
    /// baseline, specialized artifact and compilation unit.
    ///
    /// The state snapshot must be complete.
    pub fn new(
        unit: Arc<dyn JitCompilationUnit>,
        specialized_key: JitCacheKey,
        baseline_key: JitCacheKey,
        specialization: Arc<dyn SpecializationCertificate>,
        state: Arc<dyn DeoptimizationState>,
        reason: DeoptimizationReason,
        evidence: Arc<dyn DeoptimizationEvidence>,
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

        validate_identity(
            state.snapshot_identity(),
            "execution-state snapshot identity",
        )?;

        validate_identity(
            state.schema_identity(),
            "execution-state schema identity",
        )?;

        validate_identity(
            state.state_fingerprint(),
            "execution-state fingerprint",
        )?;

        validate_identity(
            evidence.evidence_identity(),
            "deoptimization evidence identity",
        )?;

        validate_identity(
            evidence.affected_assumptions_identity(),
            "affected assumptions identity",
        )?;

        validate_identity(
            evidence.execution_observation_identity(),
            "execution observation identity",
        )?;

        validate_identity(
            evidence.authority_identity(),
            "evidence authority identity",
        )?;

        reason.validate()?;

        if specialized_key == baseline_key {
            return Err(validation_error(
                "specialized and baseline cache identities must differ",
            ));
        }

        if !state.is_complete() {
            return Err(contract_error(
                "execution-state snapshot is incomplete",
            ));
        }

        // Bind the request to the original specialization certificate.

        if specialization.specialized_key() != &specialized_key {
            return Err(compatibility_error(
                "specialization certificate references another specialized artifact",
            ));
        }

        if specialization.baseline_key() != &baseline_key {
            return Err(compatibility_error(
                "specialization certificate references another baseline artifact",
            ));
        }

        if specialization.compilation_unit_identity()
            != unit.stable_identity()
        {
            return Err(compatibility_error(
                "specialization certificate references another compilation unit",
            ));
        }

        if specialization.domain_identity() != unit.domain_identity() {
            return Err(compatibility_error(
                "specialization certificate has a different computational domain",
            ));
        }

        if specialization.compiler_contract_version()
            != unit.compiler_contract_version()
        {
            return Err(compatibility_error(
                "specialization certificate has a different compiler contract",
            ));
        }

        if !specialization.semantic_preservation_verified() {
            return Err(contract_error(
                "original specialization lacks semantic-preservation verification",
            ));
        }

        if !specialization.baseline_fallback_validated() {
            return Err(contract_error(
                "original specialization has no validated baseline fallback",
            ));
        }

        Ok(Self {
            unit,
            specialized_key,
            baseline_key,
            specialization,
            state,
            reason,
            evidence,
        })
    }

    /// Returns the original compilation unit.
    #[must_use]
    pub fn unit(&self) -> &Arc<dyn JitCompilationUnit> {
        &self.unit
    }

    /// Returns the specialized artifact identity.
    #[must_use]
    pub fn specialized_key(&self) -> &JitCacheKey {
        &self.specialized_key
    }

    /// Returns the baseline artifact identity.
    #[must_use]
    pub fn baseline_key(&self) -> &JitCacheKey {
        &self.baseline_key
    }

    /// Returns the original specialization certificate.
    #[must_use]
    pub fn specialization(
        &self,
    ) -> &Arc<dyn SpecializationCertificate> {
        &self.specialization
    }

    /// Returns the captured execution state.
    #[must_use]
    pub fn state(&self) -> &Arc<dyn DeoptimizationState> {
        &self.state
    }

    /// Returns the reason for deoptimization.
    #[must_use]
    pub fn reason(&self) -> &DeoptimizationReason {
        &self.reason
    }

    /// Returns the event evidence.
    #[must_use]
    pub fn evidence(&self) -> &Arc<dyn DeoptimizationEvidence> {
        &self.evidence
    }
}

// =============================================================================
// DEOPTIMIZATION CERTIFICATE
// =============================================================================

/// Authority-issued certificate proving that a deoptimization transition
/// has passed the required semantic and state-transfer validations.
///
/// This certificate must be backed by the actual semantic,
/// compiler, execution, state reconstruction and compatibility
/// authorities.
///
/// It is not itself proof of cryptographic authentication.
///
/// Implementations must bind the certificate to the exact request.
pub trait DeoptimizationCertificate: fmt::Debug + Send + Sync {
    /// Returns the identity of the validated specialized artifact.
    fn specialized_key(&self) -> &JitCacheKey;

    /// Returns the identity of the validated baseline artifact.
    fn baseline_key(&self) -> &JitCacheKey;

    /// Returns the compilation-unit identity.
    fn compilation_unit_identity(&self) -> &str;

    /// Returns the execution-state snapshot identity.
    fn snapshot_identity(&self) -> &str;

    /// Returns the execution-state fingerprint.
    fn state_fingerprint(&self) -> &str;

    /// Returns the deoptimization evidence identity.
    fn evidence_identity(&self) -> &str;

    /// Returns the identity of the reconstruction mapping.
    fn reconstruction_identity(&self) -> &str;

    /// Returns the identity of the semantic contract.
    fn semantic_contract_identity(&self) -> &str;

    /// Returns the issuing authority identity.
    fn authority_identity(&self) -> &str;

    /// Whether baseline compatibility has been verified.
    fn baseline_compatibility_verified(&self) -> bool;

    /// Whether state reconstruction has been verified.
    fn state_reconstruction_verified(&self) -> bool;

    /// Whether observable effects are preserved.
    fn observable_effects_preserved(&self) -> bool;

    /// Whether execution ownership can be transferred safely.
    fn execution_ownership_transfer_verified(&self) -> bool;

    /// Whether the specialized assumptions have been accounted for.
    fn assumptions_accounted_for(&self) -> bool;
}

// =============================================================================
// DEOPTIMIZATION AUTHORITY
// =============================================================================

/// Validates a proposed deoptimization using canonical repository
/// authorities.
///
/// Implementations must consult applicable systems, including:
///
/// - Semantic preservation contracts.
/// - Compiler verification.
/// - Specialization certificates.
/// - Runtime guard evidence.
/// - Execution-state reconstruction.
/// - Baseline compatibility.
/// - Effect and side-effect ordering.
/// - Security and isolation policies.
/// - Domain-specific execution contracts.
///
/// Returning a certificate means the validations it represents
/// have genuinely completed.
///
/// No default permissive implementation is provided.
pub trait DeoptimizationAuthority: fmt::Debug + Send + Sync {
    /// Validates a request and returns its authority-issued certificate.
    fn validate(
        &self,
        request: &DeoptimizationRequest,
    ) -> ZurResult<Arc<dyn DeoptimizationCertificate>>;
}

// =============================================================================
// BASELINE ARTIFACT RESOLUTION
// =============================================================================

/// Resolves and verifies the baseline artifact for a deoptimization request.
///
/// Implementations must use the canonical JIT code cache, compiler
/// artifact authority and relevant execution compatibility checks.
///
/// A cache hit alone is insufficient.
///
/// The returned artifact must be verified for:
///
/// - Baseline cache identity.
/// - Domain identity.
/// - Compiler contract compatibility.
/// - Artifact integrity.
/// - Execution ABI compatibility.
/// - Security policy.
/// - Applicable provider compatibility.
///
/// This interface does not compile an artifact.
///
/// If the baseline is unavailable, the implementation must return
/// an error. It must not fabricate an artifact.
pub trait BaselineArtifactResolver: fmt::Debug + Send + Sync {
    /// Resolves and verifies a compatible baseline artifact.
    fn resolve_verified_baseline(
        &self,
        request: &DeoptimizationRequest,
        certificate: &dyn DeoptimizationCertificate,
    ) -> ZurResult<Arc<dyn CompiledJitArtifact>>;
}

// =============================================================================
// BASELINE HANDOFF
// =============================================================================

/// Validated information required for transferring execution to baseline.
///
/// This value is created only after the request, certificate and
/// baseline artifact have passed structural validation.
pub struct ValidatedDeoptimization {
    request: DeoptimizationRequest,

    certificate: Arc<dyn DeoptimizationCertificate>,

    baseline_artifact: Arc<dyn CompiledJitArtifact>,
}

impl fmt::Debug for ValidatedDeoptimization {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("ValidatedDeoptimization")
            .field("request", &self.request)
            .field("certificate", &self.certificate)
            .field("baseline_artifact", &self.baseline_artifact)
            .finish()
    }
}

impl ValidatedDeoptimization {
    /// Returns the original request.
    #[must_use]
    pub fn request(&self) -> &DeoptimizationRequest {
        &self.request
    }

    /// Returns the validated certificate.
    #[must_use]
    pub fn certificate(&self) -> &Arc<dyn DeoptimizationCertificate> {
        &self.certificate
    }

    /// Returns the verified baseline artifact.
    #[must_use]
    pub fn baseline_artifact(&self) -> &Arc<dyn CompiledJitArtifact> {
        &self.baseline_artifact
    }
}

/// Result of an atomic handoff to the baseline execution authority.
///
/// This identity represents the committed transfer operation.
///
/// It does not represent successful completion of the baseline program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineHandoffReceipt {
    handoff_identity: String,

    baseline_key: JitCacheKey,

    specialized_key: JitCacheKey,
}

impl BaselineHandoffReceipt {
    /// Creates a receipt from an actual committed handoff.
    ///
    /// This constructor is intended for implementations of
    /// [`BaselineHandoffExecutor`].
    pub fn new(
        handoff_identity: impl Into<String>,
        baseline_key: JitCacheKey,
        specialized_key: JitCacheKey,
    ) -> ZurResult<Self> {
        let handoff_identity = handoff_identity.into();

        validate_identity(
            &handoff_identity,
            "baseline handoff identity",
        )?;

        if baseline_key == specialized_key {
            return Err(validation_error(
                "handoff baseline and specialized identities must differ",
            ));
        }

        Ok(Self {
            handoff_identity,
            baseline_key,
            specialized_key,
        })
    }

    /// Returns the committed handoff identity.
    #[must_use]
    pub fn handoff_identity(&self) -> &str {
        &self.handoff_identity
    }

    /// Returns the baseline artifact identity.
    #[must_use]
    pub fn baseline_key(&self) -> &JitCacheKey {
        &self.baseline_key
    }

    /// Returns the specialized artifact identity.
    #[must_use]
    pub fn specialized_key(&self) -> &JitCacheKey {
        &self.specialized_key
    }
}

/// Performs the actual execution handoff.
///
/// This is an integration boundary, not a simulated execution engine.
///
/// The implementation must atomically:
///
/// 1. Establish that the specialized execution can no longer commit
///    additional observable effects.
/// 2. Transfer the validated execution state.
/// 3. Establish baseline execution ownership.
/// 4. Prevent duplicate execution ownership.
/// 5. Return a receipt only after the transfer is committed.
///
/// If the method returns an error, the implementation must guarantee
/// that ownership was not partially transferred in a way that could
/// produce duplicate or lost observable effects.
///
/// The implementation must coordinate with the authoritative execution
/// subsystem, not create a competing executor.
pub trait BaselineHandoffExecutor: fmt::Debug + Send + Sync {
    /// Atomically transfers the validated execution to baseline handling.
    fn handoff(
        &self,
        validated: ValidatedDeoptimization,
    ) -> ZurResult<BaselineHandoffReceipt>;
}

// =============================================================================
// OUTCOME
// =============================================================================

/// Result of a deoptimization coordination operation.
///
/// A committed handoff is not proof that the baseline computation
/// has subsequently executed or completed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeoptimizationOutcome {
    /// The execution ownership transfer has been committed.
    HandoffCommitted {
        /// Receipt from the execution authority.
        receipt: BaselineHandoffReceipt,
    },
}

// =============================================================================
// COORDINATOR
// =============================================================================

/// Immutable, dependency-injected deoptimization coordinator.
///
/// The coordinator does not:
///
/// - Spawn threads.
/// - Create background workers.
/// - Compile baseline code.
/// - Execute program instructions.
/// - Modify the compiler IR.
/// - Invalidate code-cache entries.
/// - Perform hardware discovery.
///
/// Those responsibilities belong to their canonical authorities.
#[derive(Clone)]
pub struct DeoptimizationCoordinator {
    authority: Arc<dyn DeoptimizationAuthority>,

    baseline_resolver: Arc<dyn BaselineArtifactResolver>,

    handoff_executor: Arc<dyn BaselineHandoffExecutor>,
}

impl fmt::Debug for DeoptimizationCoordinator {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter
            .debug_struct("DeoptimizationCoordinator")
            .field("authority", &self.authority)
            .field("baseline_resolver", &self.baseline_resolver)
            .field("handoff_executor", &self.handoff_executor)
            .finish()
    }
}

impl DeoptimizationCoordinator {
    /// Constructs a coordinator with explicit authorities.
    ///
    /// No default authorities are provided because deoptimization
    /// requires genuine semantic, artifact and execution validation.
    #[must_use]
    pub fn new(
        authority: Arc<dyn DeoptimizationAuthority>,
        baseline_resolver: Arc<dyn BaselineArtifactResolver>,
        handoff_executor: Arc<dyn BaselineHandoffExecutor>,
    ) -> Self {
        Self {
            authority,
            baseline_resolver,
            handoff_executor,
        }
    }

    /// Returns the deoptimization validation authority.
    #[must_use]
    pub fn authority(&self) -> &Arc<dyn DeoptimizationAuthority> {
        &self.authority
    }

    /// Prepares and commits a deoptimization handoff.
    ///
    /// The method:
    ///
    /// 1. Checks cancellation.
    /// 2. Validates the deoptimization request.
    /// 3. Obtains an authority-issued certificate.
    /// 4. Validates the certificate against the exact request.
    /// 5. Resolves and verifies the baseline artifact.
    /// 6. Checks cancellation again.
    /// 7. Commits the handoff through the execution authority.
    ///
    /// It does not claim that baseline execution has completed.
    ///
    /// Cancellation is checked before the atomic handoff. Once the
    /// handoff has committed, its receipt is returned even if a
    /// cancellation request races with the completed transfer.
    pub fn deoptimize(
        &self,
        request: DeoptimizationRequest,
        cancellation: &dyn JitCancellation,
    ) -> ZurResult<DeoptimizationOutcome> {
        Self::check_cancelled(cancellation)?;

        let certificate = self.authority.validate(&request)?;

        Self::validate_certificate(
            &request,
            certificate.as_ref(),
        )?;

        Self::check_cancelled(cancellation)?;

        let baseline_artifact = self.baseline_resolver
            .resolve_verified_baseline(
                &request,
                certificate.as_ref(),
            )?;

        Self::validate_baseline_artifact(
            &request,
            baseline_artifact.as_ref(),
        )?;

        Self::check_cancelled(cancellation)?;

        let validated = ValidatedDeoptimization {
            request,
            certificate,
            baseline_artifact,
        };

        // This call is the ownership-transfer commit boundary.
        // Do not perform a cancellation check after a successful commit.

        let receipt = self.handoff_executor.handoff(validated)?;

        Ok(DeoptimizationOutcome::HandoffCommitted {
            receipt,
        })
    }

    /// Performs structural and semantic-certificate binding checks.
    fn validate_certificate(
        request: &DeoptimizationRequest,
        certificate: &dyn DeoptimizationCertificate,
    ) -> ZurResult<()> {
        if certificate.specialized_key() != request.specialized_key() {
            return Err(compatibility_error(
                "deoptimization certificate references another specialized artifact",
            ));
        }

        if certificate.baseline_key() != request.baseline_key() {
            return Err(compatibility_error(
                "deoptimization certificate references another baseline artifact",
            ));
        }

        if certificate.compilation_unit_identity()
            != request.unit().stable_identity()
        {
            return Err(compatibility_error(
                "deoptimization certificate references another compilation unit",
            ));
        }

        if certificate.snapshot_identity()
            != request.state().snapshot_identity()
        {
            return Err(compatibility_error(
                "deoptimization certificate references another state snapshot",
            ));
        }

        if certificate.state_fingerprint()
            != request.state().state_fingerprint()
        {
            return Err(compatibility_error(
                "deoptimization certificate references different execution state",
            ));
        }

        if certificate.evidence_identity()
            != request.evidence().evidence_identity()
        {
            return Err(compatibility_error(
                "deoptimization certificate references different event evidence",
            ));
        }

        validate_identity(
            certificate.reconstruction_identity(),
            "state reconstruction identity",
        )?;

        validate_identity(
            certificate.semantic_contract_identity(),
            "semantic contract identity",
        )?;

        validate_identity(
            certificate.authority_identity(),
            "deoptimization authority identity",
        )?;

        if !certificate.baseline_compatibility_verified() {
            return Err(contract_error(
                "baseline execution compatibility has not been verified",
            ));
        }

        if !certificate.state_reconstruction_verified() {
            return Err(contract_error(
                "execution-state reconstruction has not been verified",
            ));
        }

        if !certificate.observable_effects_preserved() {
            return Err(contract_error(
                "observable effects are not proven to be preserved",
            ));
        }

        if !certificate.execution_ownership_transfer_verified() {
            return Err(contract_error(
                "execution ownership transfer has not been verified",
            ));
        }

        if !certificate.assumptions_accounted_for() {
            return Err(contract_error(
                "specialization assumptions have not been accounted for",
            ));
        }

        Ok(())
    }

    /// Validates the identity of the resolved baseline artifact.
    fn validate_baseline_artifact(
        request: &DeoptimizationRequest,
        artifact: &dyn CompiledJitArtifact,
    ) -> ZurResult<()> {
        if artifact.cache_key() != request.baseline_key() {
            return Err(compatibility_error(
                "resolved baseline artifact has the wrong cache identity",
            ));
        }

        if artifact.domain_identity()
            != request.unit().domain_identity()
        {
            return Err(compatibility_error(
                "baseline artifact belongs to another computational domain",
            ));
        }

        if artifact.compiler_contract_version()
            != request.unit().compiler_contract_version()
        {
            return Err(compatibility_error(
                "baseline artifact has an incompatible compiler contract",
            ));
        }

        validate_identity(
            artifact.provider_identity(),
            "baseline artifact provider identity",
        )?;

        validate_identity(
            artifact.format_identity(),
            "baseline artifact format identity",
        )?;

        Ok(())
    }

    /// Checks cooperative cancellation.
    fn check_cancelled(
        cancellation: &dyn JitCancellation,
    ) -> ZurResult<()> {
        if cancellation.is_cancelled() {
            return Err(
                ZurError::cancelled(
                    "JIT deoptimization was cancelled before handoff",
                )
                .with_component(COMPONENT)
                .with_operation("deoptimize"),
            );
        }

        Ok(())
    }

    /// Performs deoptimization without an external cancellation source.
    ///
    /// This is an explicitly synchronous operation.
    pub fn deoptimize_uncancellable(
        &self,
        request: DeoptimizationRequest,
    ) -> ZurResult<DeoptimizationOutcome> {
        self.deoptimize(
            request,
            &NeverCancelled,
        )
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::zur::jit::engine::JitCacheKey;

    use crate::zur::jit::specialization::SpecializationCertificate;

    #[derive(Debug)]
    struct TestUnit;

    impl JitCompilationUnit for TestUnit {
        fn stable_identity(&self) -> &str {
            "zamani.test.function"
        }

        fn domain_identity(&self) -> &str {
            "zamani.classical"
        }

        fn compiler_contract_version(&self) -> &str {
            "1.0.0"
        }
    }

    #[derive(Debug)]
    struct TestState;

    impl DeoptimizationState for TestState {
        fn snapshot_identity(&self) -> &str {
            "test.snapshot"
        }

        fn schema_identity(&self) -> &str {
            "test.state.schema"
        }

        fn state_fingerprint(&self) -> &str {
            "test.state.fingerprint"
        }

        fn is_complete(&self) -> bool {
            true
        }
    }

    #[derive(Debug)]
    struct TestEvidence;

    impl DeoptimizationEvidence for TestEvidence {
        fn evidence_identity(&self) -> &str {
            "test.deoptimization.evidence"
        }

        fn affected_assumptions_identity(&self) -> &str {
            "test.assumptions"
        }

        fn execution_observation_identity(&self) -> &str {
            "test.observation"
        }

        fn authority_identity(&self) -> &str {
            "test.execution.authority"
        }
    }

    #[derive(Debug)]
    struct TestSpecialization {
        specialized: JitCacheKey,
        baseline: JitCacheKey,
    }

    impl SpecializationCertificate for TestSpecialization {
        fn baseline_key(&self) -> &JitCacheKey {
            &self.baseline
        }

        fn specialized_key(&self) -> &JitCacheKey {
            &self.specialized
        }

        fn compilation_unit_identity(&self) -> &str {
            "zamani.test.function"
        }

        fn domain_identity(&self) -> &str {
            "zamani.classical"
        }

        fn compiler_contract_version(&self) -> &str {
            "1.0.0"
        }

        fn evidence_identity(&self) -> &str {
            "test.specialization.evidence"
        }

        fn assumption_fingerprint(&self) -> &str {
            "test.assumption.fingerprint"
        }

        fn capability_evidence_identity(&self) -> &str {
            "test.capability"
        }

        fn target_compatibility_identity(&self) -> &str {
            "test.target"
        }

        fn semantic_contract_identity(&self) -> &str {
            "test.semantic"
        }

        fn compiler_pipeline_identity(&self) -> &str {
            "test.pipeline"
        }

        fn guard_set_identity(&self) -> &str {
            "test.guards"
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
    struct TestCertificate {
        specialized: JitCacheKey,
        baseline: JitCacheKey,
    }

    impl DeoptimizationCertificate for TestCertificate {
        fn specialized_key(&self) -> &JitCacheKey {
            &self.specialized
        }

        fn baseline_key(&self) -> &JitCacheKey {
            &self.baseline
        }

        fn compilation_unit_identity(&self) -> &str {
            "zamani.test.function"
        }

        fn snapshot_identity(&self) -> &str {
            "test.snapshot"
        }

        fn state_fingerprint(&self) -> &str {
            "test.state.fingerprint"
        }

        fn evidence_identity(&self) -> &str {
            "test.deoptimization.evidence"
        }

        fn reconstruction_identity(&self) -> &str {
            "test.reconstruction"
        }

        fn semantic_contract_identity(&self) -> &str {
            "test.semantic"
        }

        fn authority_identity(&self) -> &str {
            "test.deoptimization.authority"
        }

        fn baseline_compatibility_verified(&self) -> bool {
            true
        }

        fn state_reconstruction_verified(&self) -> bool {
            true
        }

        fn observable_effects_preserved(&self) -> bool {
            true
        }

        fn execution_ownership_transfer_verified(&self) -> bool {
            true
        }

        fn assumptions_accounted_for(&self) -> bool {
            true
        }
    }

    #[derive(Debug)]
    struct TestAuthority;

    impl DeoptimizationAuthority for TestAuthority {
        fn validate(
            &self,
            request: &DeoptimizationRequest,
        ) -> ZurResult<Arc<dyn DeoptimizationCertificate>> {
            Ok(Arc::new(TestCertificate {
                specialized: request.specialized_key().clone(),
                baseline: request.baseline_key().clone(),
            }))
        }
    }

    #[derive(Debug)]
    struct TestArtifact {
        key: JitCacheKey,
    }

    impl CompiledJitArtifact for TestArtifact {
        fn cache_key(&self) -> &JitCacheKey {
            &self.key
        }

        fn domain_identity(&self) -> &str {
            "zamani.classical"
        }

        fn provider_identity(&self) -> &str {
            "zamani.test.provider"
        }

        fn format_identity(&self) -> &str {
            "zamani.test.artifact"
        }

        fn compiler_contract_version(&self) -> &str {
            "1.0.0"
        }
    }

    #[derive(Debug)]
    struct TestBaselineResolver;

    impl BaselineArtifactResolver for TestBaselineResolver {
        fn resolve_verified_baseline(
            &self,
            request: &DeoptimizationRequest,
            _certificate: &dyn DeoptimizationCertificate,
        ) -> ZurResult<Arc<dyn CompiledJitArtifact>> {
            Ok(Arc::new(TestArtifact {
                key: request.baseline_key().clone(),
            }))
        }
    }

    #[derive(Debug)]
    struct TestHandoff;

    impl BaselineHandoffExecutor for TestHandoff {
        fn handoff(
            &self,
            validated: ValidatedDeoptimization,
        ) -> ZurResult<BaselineHandoffReceipt> {
            BaselineHandoffReceipt::new(
                "test.committed.handoff",
                validated.request().baseline_key().clone(),
                validated.request().specialized_key().clone(),
            )
        }
    }

    fn key(value: &str) -> JitCacheKey {
        JitCacheKey::new(value)
            .expect("valid test key")
    }

    fn request() -> DeoptimizationRequest {
        let specialized = key("test.specialized");
        let baseline = key("test.baseline");

        DeoptimizationRequest::new(
            Arc::new(TestUnit),
            specialized.clone(),
            baseline.clone(),
            Arc::new(TestSpecialization {
                specialized,
                baseline,
            }),
            Arc::new(TestState),
            DeoptimizationReason::GuardFailure {
                guard_identity: "test.guard".to_owned(),
            },
            Arc::new(TestEvidence),
        )
        .expect("valid deoptimization request")
    }

    fn coordinator() -> DeoptimizationCoordinator {
        DeoptimizationCoordinator::new(
            Arc::new(TestAuthority),
            Arc::new(TestBaselineResolver),
            Arc::new(TestHandoff),
        )
    }

    #[test]
    fn api_version_is_available() {
        assert_eq!(JIT_DEOPTIMIZATION_API_VERSION, 1);
    }

    #[test]
    fn validates_guard_failure_identity() {
        let reason = DeoptimizationReason::GuardFailure {
            guard_identity: "test.guard".to_owned(),
        };

        assert!(reason.validate().is_ok());
    }

    #[test]
    fn rejects_empty_reason_identity() {
        let reason = DeoptimizationReason::GuardFailure {
            guard_identity: String::new(),
        };

        assert!(reason.validate().is_err());
    }

    #[test]
    fn rejects_identical_artifact_identities() {
        let result = DeoptimizationRequest::new(
            Arc::new(TestUnit),
            key("same"),
            key("same"),
            Arc::new(TestSpecialization {
                specialized: key("same"),
                baseline: key("same"),
            }),
            Arc::new(TestState),
            DeoptimizationReason::ExplicitRequest {
                authority_identity: "test.authority".to_owned(),
            },
            Arc::new(TestEvidence),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_incomplete_state_snapshots() {
        #[derive(Debug)]
        struct IncompleteState;

        impl DeoptimizationState for IncompleteState {
            fn snapshot_identity(&self) -> &str {
                "test.snapshot"
            }

            fn schema_identity(&self) -> &str {
                "test.schema"
            }

            fn state_fingerprint(&self) -> &str {
                "test.fingerprint"
            }

            fn is_complete(&self) -> bool {
                false
            }
        }

        let specialized = key("specialized");
        let baseline = key("baseline");

        let result = DeoptimizationRequest::new(
            Arc::new(TestUnit),
            specialized.clone(),
            baseline.clone(),
            Arc::new(TestSpecialization {
                specialized,
                baseline,
            }),
            Arc::new(IncompleteState),
            DeoptimizationReason::ExplicitRequest {
                authority_identity: "test.authority".to_owned(),
            },
            Arc::new(TestEvidence),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_wrong_specialization_identity() {
        let specialized = key("specialized");
        let baseline = key("baseline");

        let result = DeoptimizationRequest::new(
            Arc::new(TestUnit),
            specialized,
            baseline,
            Arc::new(TestSpecialization {
                specialized: key("another.specialized"),
                baseline: key("baseline"),
            }),
            Arc::new(TestState),
            DeoptimizationReason::ExplicitRequest {
                authority_identity: "test.authority".to_owned(),
            },
            Arc::new(TestEvidence),
        );

        assert!(result.is_err());
    }

    #[test]
    fn commits_a_validated_baseline_handoff() {
        let outcome = coordinator()
            .deoptimize_uncancellable(request())
            .expect("handoff succeeds");

        match outcome {
            DeoptimizationOutcome::HandoffCommitted { receipt } => {
                assert_eq!(
                    receipt.baseline_key().as_str(),
                    "test.baseline",
                );

                assert_eq!(
                    receipt.specialized_key().as_str(),
                    "test.specialized",
                );

                assert_eq!(
                    receipt.handoff_identity(),
                    "test.committed.handoff",
                );
            }
        }
    }

    #[test]
    fn cancellation_prevents_handoff() {
        #[derive(Debug)]
        struct Cancelled;

        impl JitCancellation for Cancelled {
            fn is_cancelled(&self) -> bool {
                true
            }
        }

        let result = coordinator()
            .deoptimize(request(), &Cancelled);

        assert!(result.is_err());
    }

    #[test]
    fn handoff_receipt_rejects_identical_keys() {
        let result = BaselineHandoffReceipt::new(
            "test.handoff",
            key("same"),
            key("same"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn handoff_receipt_rejects_empty_identity() {
        let result = BaselineHandoffReceipt::new(
            "",
            key("baseline"),
            key("specialized"),
        );

        assert!(result.is_err());
    }
}
