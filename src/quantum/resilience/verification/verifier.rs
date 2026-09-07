//! Production-grade verification boundary for Zamani quantum resilience.
//!
//! Path:
//!     src/quantum/resilience/verification/verifier.rs
//!
//! # Purpose
//!
//! This module is the authoritative verification composition boundary for the
//! resilience subsystem.
//!
//! It answers one question:
//!
//!     "May the outcome of this resilient execution be accepted?"
//!
//! It does NOT assume that:
//!
//! - successful execution means semantic correctness;
//! - a backend reporting success means the result is trustworthy;
//! - a recovery action is valid merely because it completed;
//! - a mitigation action preserved program semantics;
//! - a physical qubit identity is a logical qubit identity;
//! - a result with high statistical confidence is semantically correct;
//! - a matching execution identifier means two executions are equivalent.
//!
//! Acceptance requires all mandatory verification dimensions to pass.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Quantum Program
//!                                  │
//!                                  ▼
//!                         canonical quantum::ir
//!                                  │
//!                                  ▼
//!                         resilience execution
//!                                  │
//!              ┌───────────────────┼───────────────────┐
//!              │                   │                   │
//!              ▼                   ▼                   ▼
//!          execution          adaptations          recovery
//!              │                   │                   │
//!              └───────────────────┼───────────────────┘
//!                                  ▼
//!                         ResilienceVerifier
//!                                  │
//!       ┌──────────────┬───────────┼───────────┬──────────────┐
//!       ▼              ▼           ▼           ▼              ▼
//!   invariants      semantic     result     provenance     acceptance
//!       │              │           │           │              │
//!       └──────────────┴───────────┴───────────┴──────────────┘
//!                                  │
//!                    ┌─────────────┴─────────────┐
//!                    ▼                           ▼
//!                 ACCEPT                  NOT ACCEPTED
//! ```
//!
//! # Critical architectural rule
//!
//! Verification is authoritative for acceptance.
//!
//! No controller, recovery engine, mitigation engine, backend, provider,
//! detector, planner, or learning component may convert an unverified result
//! into an accepted result.
//!
//! # Canonical quantum identity
//!
//! Logical and physical quantum resources use the canonical IR identities:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This module deliberately defines no resilience-local qubit identifier.
//!
//! # Write once, scale everywhere
//!
//! There is no architectural maximum for:
//!
//! - qubits;
//! - logical qubits;
//! - physical qubits;
//! - operations;
//! - execution generations;
//! - verification observations;
//! - violations;
//! - resources;
//! - machines;
//! - backends;
//! - verification stages.
//!
//! Concrete memory/time/resource limits are external execution-policy
//! constraints. This module never embeds a machine-size constant.
//!
//! "Infinity" therefore means:
//!
//! > no artificial finite quantum-machine ceiling is encoded here.
//!
//! Every concrete execution remains finite because the supplied computation
//! and execution environment are finite.
//!
//! # Determinism
//!
//! Verification is deterministic for identical inputs and deterministic
//! verifier implementations.
//!
//! The verifier does not:
//!
//! - read the system clock;
//! - access environment variables;
//! - access process-global state;
//! - generate random values;
//! - inspect memory addresses;
//! - use hash-map iteration order for semantic decisions;
//! - silently consult external services.
//!
//! If probabilistic verification is required, the randomness and seed belong
//! to the explicit verification evidence supplied by the caller.
//!
//! # Integration
//!
//! This file intentionally defines stable contracts consumed by:
//!
//! - `verification/invariant.rs`
//! - `verification/semantic.rs`
//! - `verification/result.rs`
//! - `verification/confidence.rs`
//! - `verification/provenance.rs`
//! - `verification/acceptance.rs`
//!
//! Those modules can implement the traits defined below without changing this
//! verifier's orchestration contract.
//!
//! The public resilience controller can depend on [`ResilienceVerifier`].
//!
//! # Rust contract
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust
//! - no nightly features
//! - no unsafe code
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

// =============================================================================
// Schema identity
// =============================================================================

/// Stable schema identifier for the resilience verifier contract.
///
/// This identifier is intentionally independent of the Rust type name.
pub const RESILIENCE_VERIFIER_SCHEMA_ID: &str =
    "zamani.quantum.resilience.verification.verifier";

/// Current verifier contract version.
///
/// Increment only when the serialized/semantic contract changes.
pub const RESILIENCE_VERIFIER_SCHEMA_VERSION: u32 = 1;

// =============================================================================
// Verification policy
// =============================================================================

/// Controls which verification dimensions are mandatory.
///
/// No policy value contains a hardware-specific assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VerificationPolicy {
    /// Semantic equivalence must be established.
    semantic: bool,

    /// Required invariants must hold.
    invariants: bool,

    /// The execution result must pass result validation.
    result: bool,

    /// Provenance/integrity information must be complete.
    provenance: bool,

    /// Evidence must meet the configured confidence requirement.
    confidence: bool,

    /// Physical/logical resource identity consistency must be checked.
    resource_identity: bool,

    /// Execution itself must report a successful terminal state.
    execution: bool,

    /// Recovery/adaptation actions must themselves be authorized/verified.
    recovery_integrity: bool,
}

impl VerificationPolicy {
    /// Strict production policy.
    ///
    /// Every verification dimension is mandatory.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            semantic: true,
            invariants: true,
            result: true,
            provenance: true,
            confidence: true,
            resource_identity: true,
            execution: true,
            recovery_integrity: true,
        }
    }

    /// Creates a policy in which every verification dimension is enabled.
    ///
    /// This is equivalent to [`Self::strict`].
    #[must_use]
    pub const fn all() -> Self {
        Self::strict()
    }

    /// Creates a permissive policy suitable for development or diagnostics.
    ///
    /// This policy is still safe in one important respect: the verifier never
    /// reports acceptance unless the enabled mandatory checks pass.
    ///
    /// Production callers should normally use [`Self::strict`].
    #[must_use]
    pub const fn permissive() -> Self {
        Self {
            semantic: true,
            invariants: true,
            result: true,
            provenance: false,
            confidence: false,
            resource_identity: true,
            execution: true,
            recovery_integrity: false,
        }
    }

    #[must_use]
    pub const fn semantic(self) -> bool {
        self.semantic
    }

    #[must_use]
    pub const fn invariants(self) -> bool {
        self.invariants
    }

    #[must_use]
    pub const fn result(self) -> bool {
        self.result
    }

    #[must_use]
    pub const fn provenance(self) -> bool {
        self.provenance
    }

    #[must_use]
    pub const fn confidence(self) -> bool {
        self.confidence
    }

    #[must_use]
    pub const fn resource_identity(self) -> bool {
        self.resource_identity
    }

    #[must_use]
    pub const fn execution(self) -> bool {
        self.execution
    }

    #[must_use]
    pub const fn recovery_integrity(self) -> bool {
        self.recovery_integrity
    }
}

impl Default for VerificationPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

// =============================================================================
// Verification stage
// =============================================================================

/// Verification dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationStage {
    /// Execution lifecycle status.
    Execution,

    /// Logical/physical resource identity consistency.
    ResourceIdentity,

    /// Canonical program invariants.
    Invariants,

    /// Canonical semantic equivalence.
    Semantic,

    /// Execution-result structural/statistical validation.
    Result,

    /// Provenance and integrity validation.
    Provenance,

    /// Evidence confidence validation.
    Confidence,

    /// Recovery/adaptation integrity.
    RecoveryIntegrity,

    /// Final acceptance decision.
    Acceptance,
}

impl VerificationStage {
    /// Stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Execution => "execution",
            Self::ResourceIdentity => "resource_identity",
            Self::Invariants => "invariants",
            Self::Semantic => "semantic",
            Self::Result => "result",
            Self::Provenance => "provenance",
            Self::Confidence => "confidence",
            Self::RecoveryIntegrity => "recovery_integrity",
            Self::Acceptance => "acceptance",
        }
    }
}

// =============================================================================
// Verification state
// =============================================================================

/// Result of one verification dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationState {
    /// The dimension passed.
    Passed,

    /// The dimension failed.
    Failed,

    /// The dimension was not required by policy.
    NotRequired,

    /// The dimension could not be established.
    Indeterminate,
}

impl VerificationState {
    /// Whether this state is sufficient for acceptance.
    #[must_use]
    pub const fn is_successful(self) -> bool {
        matches!(self, Self::Passed | Self::NotRequired)
    }

    /// Whether this state explicitly blocks acceptance.
    #[must_use]
    pub const fn blocks_acceptance(self) -> bool {
        matches!(self, Self::Failed | Self::Indeterminate)
    }

    /// Stable machine-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::NotRequired => "not_required",
            Self::Indeterminate => "indeterminate",
        }
    }
}

// =============================================================================
// Verification confidence
// =============================================================================

/// Exact confidence representation in basis points.
///
/// Range:
///
///     0     = 0%
///     10000 = 100%
///
/// Integer representation avoids floating-point NaN/ordering problems in the
/// authoritative acceptance path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VerificationConfidence(u16);

impl VerificationConfidence {
    pub const MIN: u16 = 0;
    pub const MAX: u16 = 10_000;

    /// Constructs confidence from basis points.
    pub const fn from_basis_points(value: u16) -> Option<Self> {
        if value <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Constructs 100% confidence.
    #[must_use]
    pub const fn certain() -> Self {
        Self(Self::MAX)
    }

    /// Constructs 0% confidence.
    #[must_use]
    pub const fn none() -> Self {
        Self(Self::MIN)
    }

    #[must_use]
    pub const fn basis_points(self) -> u16 {
        self.0
    }

    /// Returns the confidence as a normalized value in `[0, 1]`.
    #[must_use]
    pub fn as_ratio(self) -> f64 {
        f64::from(self.0) / f64::from(Self::MAX)
    }

    /// Returns whether this confidence meets the required level.
    #[must_use]
    pub const fn meets(self, required: Self) -> bool {
        self.0 >= required.0
    }
}

impl Default for VerificationConfidence {
    fn default() -> Self {
        Self::certain()
    }
}

// =============================================================================
// Canonical execution identity
// =============================================================================

/// Stable identifier for one logical resilience execution.
///
/// This is intentionally opaque to the verifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExecutionIdentity(Arc<str>);

impl ExecutionIdentity {
    /// Creates an execution identity.
    ///
    /// Empty identifiers are rejected because an empty value cannot provide
    /// meaningful provenance.
    pub fn new(value: impl Into<Arc<str>>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidConfiguration,
                "execution identity must not be empty",
            ));
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

// =============================================================================
// Semantic fingerprint
// =============================================================================

/// Canonical semantic identity of a quantum computation.
///
/// This is intentionally an opaque fingerprint. The verifier does not assume
/// how the fingerprint was produced.
///
/// The canonical IR hashing subsystem remains responsible for canonical
/// serialization/hashing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SemanticFingerprint(Arc<str>);

impl SemanticFingerprint {
    /// Creates a semantic fingerprint.
    pub fn new(value: impl Into<Arc<str>>) -> ResilienceResult<Self> {
        let value = value.into();

        if value.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidConfiguration,
                "semantic fingerprint must not be empty",
            ));
        }

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

// =============================================================================
// Resource scope
// =============================================================================

/// Canonical resource scope associated with a verification observation.
///
/// Logical and physical resources are intentionally represented separately.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VerificationResourceScope {
    logical_qubits: Arc<[QubitId]>,
    physical_qubits: Arc<[PhysicalQubitId]>,
}

impl VerificationResourceScope {
    /// Creates a resource scope.
    ///
    /// The caller owns the semantic relationship between logical and physical
    /// resources. This structure merely preserves the two domains separately.
    pub fn new(
        logical_qubits: impl IntoIterator<Item = QubitId>,
        physical_qubits: impl IntoIterator<Item = PhysicalQubitId>,
    ) -> Self {
        let mut logical: Vec<QubitId> = logical_qubits.into_iter().collect();
        let mut physical: Vec<PhysicalQubitId> = physical_qubits.into_iter().collect();

        logical.sort();
        logical.dedup();

        physical.sort();
        physical.dedup();

        Self {
            logical_qubits: logical.into(),
            physical_qubits: physical.into(),
        }
    }

    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }
}

// =============================================================================
// Verification violation
// =============================================================================

/// Machine-readable verification failure category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VerificationViolationKind {
    /// Execution did not reach a valid terminal success state.
    ExecutionFailure,

    /// A logical/physical resource relationship is invalid.
    ResourceIdentityMismatch,

    /// Required program invariant failed.
    InvariantFailure,

    /// Candidate computation is not semantically equivalent.
    SemanticMismatch,

    /// Result structure/content is invalid.
    ResultInvalid,

    /// Provenance is absent, incomplete, inconsistent or tampered with.
    ProvenanceInvalid,

    /// Evidence confidence is insufficient.
    ConfidenceInsufficient,

    /// Recovery/adaptation integrity cannot be established.
    RecoveryIntegrityFailure,

    /// Required evidence was not supplied.
    MissingEvidence,

    /// A verifier implementation could not establish a conclusion.
    Indeterminate,

    /// A verifier implementation violated its own contract.
    VerifierContractViolation,
}

impl VerificationViolationKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionFailure => "execution_failure",
            Self::ResourceIdentityMismatch => "resource_identity_mismatch",
            Self::InvariantFailure => "invariant_failure",
            Self::SemanticMismatch => "semantic_mismatch",
            Self::ResultInvalid => "result_invalid",
            Self::ProvenanceInvalid => "provenance_invalid",
            Self::ConfidenceInsufficient => "confidence_insufficient",
            Self::RecoveryIntegrityFailure => "recovery_integrity_failure",
            Self::MissingEvidence => "missing_evidence",
            Self::Indeterminate => "indeterminate",
            Self::VerifierContractViolation => "verifier_contract_violation",
        }
    }
}

/// One verification violation.
///
/// The verifier stores a bounded-size textual description only as supplied by
/// the verifier implementation. It does not generate enormous diagnostic
/// strings from arbitrary quantum objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationViolation {
    stage: VerificationStage,
    kind: VerificationViolationKind,
    code: Arc<str>,
    message: Arc<str>,
}

impl VerificationViolation {
    pub fn new(
        stage: VerificationStage,
        kind: VerificationViolationKind,
        code: impl Into<Arc<str>>,
        message: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            stage,
            kind,
            code: code.into(),
            message: message.into(),
        }
    }

    #[must_use]
    pub const fn stage(&self) -> VerificationStage {
        self.stage
    }

    #[must_use]
    pub const fn kind(&self) -> VerificationViolationKind {
        self.kind
    }

    #[must_use]
    pub fn code(&self) -> &str {
        self.code.as_ref()
    }

    #[must_use]
    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}

// =============================================================================
// Verification evidence
// =============================================================================

/// Generic evidence returned by a verification component.
///
/// Specialized verification modules can attach richer evidence through their
/// own result structures and translate it into this stable composition type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationEvidence {
    state: VerificationState,
    confidence: VerificationConfidence,
    violations: Arc<[VerificationViolation]>,
}

impl VerificationEvidence {
    /// Creates evidence.
    ///
    /// A `Passed` result with violations is normalized to `Failed`.
    pub fn new(
        state: VerificationState,
        confidence: VerificationConfidence,
        violations: impl IntoIterator<Item = VerificationViolation>,
    ) -> Self {
        let violations: Vec<VerificationViolation> = violations.into_iter().collect();

        let state = if state == VerificationState::Passed && !violations.is_empty() {
            VerificationState::Failed
        } else {
            state
        };

        Self {
            state,
            confidence,
            violations: violations.into(),
        }
    }

    #[must_use]
    pub const fn passed(confidence: VerificationConfidence) -> Self {
        Self {
            state: VerificationState::Passed,
            confidence,
            violations: Arc::new([]),
        }
    }

    #[must_use]
    pub const fn failed(
        confidence: VerificationConfidence,
        violations: Arc<[VerificationViolation]>,
    ) -> Self {
        Self {
            state: VerificationState::Failed,
            confidence,
            violations,
        }
    }

    #[must_use]
    pub const fn indeterminate(
        confidence: VerificationConfidence,
        violations: Arc<[VerificationViolation]>,
    ) -> Self {
        Self {
            state: VerificationState::Indeterminate,
            confidence,
            violations,
        }
    }

    #[must_use]
    pub const fn not_required() -> Self {
        Self {
            state: VerificationState::NotRequired,
            confidence: VerificationConfidence::certain(),
            violations: Arc::new([]),
        }
    }

    #[must_use]
    pub const fn state(&self) -> VerificationState {
        self.state
    }

    #[must_use]
    pub const fn confidence(&self) -> VerificationConfidence {
        self.confidence
    }

    #[must_use]
    pub fn violations(&self) -> &[VerificationViolation] {
        &self.violations
    }

    #[must_use]
    pub fn is_successful(&self) -> bool {
        self.state.is_successful()
    }
}

// =============================================================================
// Verification request
// =============================================================================

/// Immutable input to the authoritative verifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationRequest {
    execution_id: ExecutionIdentity,
    expected_semantic_fingerprint: SemanticFingerprint,
    candidate_semantic_fingerprint: Option<SemanticFingerprint>,
    resources: VerificationResourceScope,
    execution_completed_successfully: bool,
    recovery_or_adaptation_performed: bool,
    policy: VerificationPolicy,
    required_confidence: VerificationConfidence,
}

impl VerificationRequest {
    /// Creates a strict verification request.
    pub fn new(
        execution_id: ExecutionIdentity,
        expected_semantic_fingerprint: SemanticFingerprint,
        resources: VerificationResourceScope,
    ) -> Self {
        Self {
            execution_id,
            expected_semantic_fingerprint,
            candidate_semantic_fingerprint: None,
            resources,
            execution_completed_successfully: false,
            recovery_or_adaptation_performed: false,
            policy: VerificationPolicy::strict(),
            required_confidence: VerificationConfidence::certain(),
        }
    }

    #[must_use]
    pub fn execution_id(&self) -> &ExecutionIdentity {
        &self.execution_id
    }

    #[must_use]
    pub fn expected_semantic_fingerprint(&self) -> &SemanticFingerprint {
        &self.expected_semantic_fingerprint
    }

    #[must_use]
    pub fn candidate_semantic_fingerprint(&self) -> Option<&SemanticFingerprint> {
        self.candidate_semantic_fingerprint.as_ref()
    }

    #[must_use]
    pub fn resources(&self) -> &VerificationResourceScope {
        &self.resources
    }

    #[must_use]
    pub const fn execution_completed_successfully(&self) -> bool {
        self.execution_completed_successfully
    }

    #[must_use]
    pub const fn recovery_or_adaptation_performed(&self) -> bool {
        self.recovery_or_adaptation_performed
    }

    #[must_use]
    pub const fn policy(&self) -> VerificationPolicy {
        self.policy
    }

    #[must_use]
    pub const fn required_confidence(&self) -> VerificationConfidence {
        self.required_confidence
    }

    /// Sets the candidate semantic fingerprint.
    #[must_use]
    pub fn with_candidate_semantic_fingerprint(
        mut self,
        fingerprint: SemanticFingerprint,
    ) -> Self {
        self.candidate_semantic_fingerprint = Some(fingerprint);
        self
    }

    /// Marks execution as successfully completed.
    #[must_use]
    pub const fn with_execution_success(mut self) -> Self {
        self.execution_completed_successfully = true;
        self
    }

    /// Marks that recovery or adaptation occurred.
    #[must_use]
    pub const fn with_recovery_or_adaptation(mut self) -> Self {
        self.recovery_or_adaptation_performed = true;
        self
    }

    /// Replaces the verification policy.
    #[must_use]
    pub const fn with_policy(mut self, policy: VerificationPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// Sets the minimum required verification confidence.
    #[must_use]
    pub const fn with_required_confidence(
        mut self,
        confidence: VerificationConfidence,
    ) -> Self {
        self.required_confidence = confidence;
        self
    }
}

// =============================================================================
// Verification component traits
// =============================================================================

/// Common contract for a verification component.
///
/// Implementations must be deterministic for deterministic input.
///
/// They must not perform hidden I/O or mutate global state.
pub trait VerificationComponent: Send + Sync + fmt::Debug {
    /// Stable component identifier.
    fn id(&self) -> &str;

    /// Verification stage implemented by the component.
    fn stage(&self) -> VerificationStage;

    /// Verifies the supplied execution.
    fn verify(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence>;
}

/// Semantic verification contract.
///
/// `verification/semantic.rs` should implement this trait using the canonical
/// quantum IR semantic/equivalence machinery.
///
/// The resilience verifier itself does not implement quantum equivalence.
pub trait SemanticVerifier: Send + Sync + fmt::Debug {
    fn id(&self) -> &str;

    fn verify_semantics(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence>;
}

/// Invariant verification contract.
///
/// `verification/invariant.rs` should implement this trait.
pub trait InvariantVerifier: Send + Sync + fmt::Debug {
    fn id(&self) -> &str;

    fn verify_invariants(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence>;
}

/// Result verification contract.
///
/// `verification/result.rs` should implement this trait.
pub trait ResultVerifier: Send + Sync + fmt::Debug {
    fn id(&self) -> &str;

    fn verify_result(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence>;
}

/// Provenance verification contract.
///
/// `verification/provenance.rs` should implement this trait.
pub trait ProvenanceVerifier: Send + Sync + fmt::Debug {
    fn id(&self) -> &str;

    fn verify_provenance(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence>;
}

/// Confidence verification contract.
///
/// `verification/confidence.rs` should implement this trait.
pub trait ConfidenceVerifier: Send + Sync + fmt::Debug {
    fn id(&self) -> &str;

    fn verify_confidence(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence>;
}

/// Recovery/adaptation integrity contract.
///
/// This is intentionally separate from semantic verification.
///
/// A recovery action may be semantically equivalent while still being
/// unauthorized, incomplete or inconsistent with the recorded recovery plan.
pub trait RecoveryIntegrityVerifier: Send + Sync + fmt::Debug {
    fn id(&self) -> &str;

    fn verify_recovery_integrity(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence>;
}

/// Resource identity verification contract.
///
/// This component verifies that logical and physical identity information is
/// structurally consistent without conflating the two identity domains.
pub trait ResourceIdentityVerifier: Send + Sync + fmt::Debug {
    fn id(&self) -> &str;

    fn verify_resource_identity(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence>;
}

// =============================================================================
// Verifier outcome
// =============================================================================

/// Final authoritative acceptance decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationDecision {
    /// The execution has satisfied all mandatory verification requirements.
    Accept,

    /// The execution may be used only under an explicitly degraded policy.
    DegradedAccept,

    /// Verification failed and another execution/recovery cycle is required.
    Repeat,

    /// Verification cannot establish safety/correctness automatically.
    Escalate,

    /// The result must not be accepted.
    Reject,
}

impl VerificationDecision {
    #[must_use]
    pub const fn is_accepted(self) -> bool {
        matches!(self, Self::Accept | Self::DegradedAccept)
    }

    #[must_use]
    pub const fn blocks_acceptance(self) -> bool {
        !self.is_accepted()
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::DegradedAccept => "degraded_accept",
            Self::Repeat => "repeat",
            Self::Escalate => "escalate",
            Self::Reject => "reject",
        }
    }
}

/// One completed verification-stage outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationStageResult {
    stage: VerificationStage,
    state: VerificationState,
    confidence: VerificationConfidence,
    component_id: Arc<str>,
    violations: Arc<[VerificationViolation]>,
}

impl VerificationStageResult {
    fn from_evidence(
        stage: VerificationStage,
        component_id: impl Into<Arc<str>>,
        evidence: VerificationEvidence,
    ) -> Self {
        Self {
            stage,
            state: evidence.state,
            confidence: evidence.confidence,
            component_id: component_id.into(),
            violations: evidence.violations,
        }
    }

    #[must_use]
    pub const fn stage(&self) -> VerificationStage {
        self.stage
    }

    #[must_use]
    pub const fn state(&self) -> VerificationState {
        self.state
    }

    #[must_use]
    pub const fn confidence(&self) -> VerificationConfidence {
        self.confidence
    }

    #[must_use]
    pub fn component_id(&self) -> &str {
        self.component_id.as_ref()
    }

    #[must_use]
    pub fn violations(&self) -> &[VerificationViolation] {
        &self.violations
    }

    #[must_use]
    pub fn passed(&self) -> bool {
        self.state.is_successful()
    }
}

/// Complete verification result.
///
/// This object is immutable after construction and can safely be shared with
/// the resilience response/provenance layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReport {
    schema_id: &'static str,
    schema_version: u32,
    execution_id: ExecutionIdentity,
    decision: VerificationDecision,
    overall_state: VerificationState,
    overall_confidence: VerificationConfidence,
    stages: Arc<[VerificationStageResult]>,
    violations: Arc<[VerificationViolation]>,
}

impl VerificationReport {
    fn new(
        execution_id: ExecutionIdentity,
        decision: VerificationDecision,
        overall_state: VerificationState,
        overall_confidence: VerificationConfidence,
        stages: Vec<VerificationStageResult>,
        violations: Vec<VerificationViolation>,
    ) -> Self {
        Self {
            schema_id: RESILIENCE_VERIFIER_SCHEMA_ID,
            schema_version: RESILIENCE_VERIFIER_SCHEMA_VERSION,
            execution_id,
            decision,
            overall_state,
            overall_confidence,
            stages: stages.into(),
            violations: violations.into(),
        }
    }

    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    #[must_use]
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    #[must_use]
    pub fn execution_id(&self) -> &ExecutionIdentity {
        &self.execution_id
    }

    #[must_use]
    pub const fn decision(&self) -> VerificationDecision {
        self.decision
    }

    #[must_use]
    pub const fn overall_state(&self) -> VerificationState {
        self.overall_state
    }

    #[must_use]
    pub const fn overall_confidence(&self) -> VerificationConfidence {
        self.overall_confidence
    }

    #[must_use]
    pub fn stages(&self) -> &[VerificationStageResult] {
        &self.stages
    }

    #[must_use]
    pub fn violations(&self) -> &[VerificationViolation] {
        &self.violations
    }

    #[must_use]
    pub const fn accepted(&self) -> bool {
        self.decision.is_accepted()
    }
}

// =============================================================================
// Resource identity verifier
// =============================================================================

/// Built-in structural resource identity verifier.
///
/// This does not attempt to determine whether a physical placement is
/// executable. Routing/hardware own that question.
///
/// It only ensures that identity domains are not accidentally collapsed.
#[derive(Debug, Default)]
pub struct StructuralResourceIdentityVerifier;

impl StructuralResourceIdentityVerifier {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl ResourceIdentityVerifier for StructuralResourceIdentityVerifier {
    fn id(&self) -> &str {
        "zamani.resilience.verification.resource_identity.structural"
    }

    fn verify_resource_identity(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        let logical = request.resources.logical_qubits();
        let physical = request.resources.physical_qubits();

        // The two domains are intentionally independent. An empty physical
        // scope is valid for logical/simulated execution. An empty logical
        // scope can also be valid for certain infrastructure operations.
        //
        // Therefore this verifier must NOT impose:
        //
        //     logical_count == physical_count
        //
        // because that would incorrectly reject encoded, distributed,
        // simulator, measurement-only and dynamically mapped executions.

        if logical.iter().any(|qubit| {
            logical
                .iter()
                .filter(|candidate| *candidate == qubit)
                .count()
                > 1
        }) {
            return Ok(VerificationEvidence::new(
                VerificationState::Failed,
                VerificationConfidence::certain(),
                [VerificationViolation::new(
                    VerificationStage::ResourceIdentity,
                    VerificationViolationKind::ResourceIdentityMismatch,
                    "QR-VER-RESOURCE-001",
                    "logical qubit identity is duplicated",
                )],
            ));
        }

        if physical.iter().any(|qubit| {
            physical
                .iter()
                .filter(|candidate| *candidate == qubit)
                .count()
                > 1
        }) {
            return Ok(VerificationEvidence::new(
                VerificationState::Failed,
                VerificationConfidence::certain(),
                [VerificationViolation::new(
                    VerificationStage::ResourceIdentity,
                    VerificationViolationKind::ResourceIdentityMismatch,
                    "QR-VER-RESOURCE-002",
                    "physical qubit identity is duplicated",
                )],
            ));
        }

        Ok(VerificationEvidence::passed(
            VerificationConfidence::certain(),
        ))
    }
}

// =============================================================================
// Resilience verifier
// =============================================================================

/// Authoritative resilience verification orchestrator.
///
/// The verifier owns composition and acceptance semantics, not the underlying
/// quantum verification algorithms.
#[derive(Debug)]
pub struct ResilienceVerifier {
    policy: VerificationPolicy,
    required_confidence: VerificationConfidence,
    semantic: Option<Arc<dyn SemanticVerifier>>,
    invariants: Option<Arc<dyn InvariantVerifier>>,
    result: Option<Arc<dyn ResultVerifier>>,
    provenance: Option<Arc<dyn ProvenanceVerifier>>,
    confidence: Option<Arc<dyn ConfidenceVerifier>>,
    recovery_integrity: Option<Arc<dyn RecoveryIntegrityVerifier>>,
    resource_identity: Option<Arc<dyn ResourceIdentityVerifier>>,
}

impl ResilienceVerifier {
    /// Creates a strict verifier with the built-in structural resource check.
    #[must_use]
    pub fn strict() -> Self {
        Self {
            policy: VerificationPolicy::strict(),
            required_confidence: VerificationConfidence::certain(),
            semantic: None,
            invariants: None,
            result: None,
            provenance: None,
            confidence: None,
            recovery_integrity: None,
            resource_identity: Some(Arc::new(StructuralResourceIdentityVerifier::new())),
        }
    }

    /// Creates a verifier from explicit policy.
    #[must_use]
    pub fn with_policy(policy: VerificationPolicy) -> Self {
        Self {
            policy,
            required_confidence: VerificationConfidence::certain(),
            semantic: None,
            invariants: None,
            result: None,
            provenance: None,
            confidence: None,
            recovery_integrity: None,
            resource_identity: Some(Arc::new(StructuralResourceIdentityVerifier::new())),
        }
    }

    /// Sets the minimum required aggregate confidence.
    #[must_use]
    pub const fn with_required_confidence(
        mut self,
        confidence: VerificationConfidence,
    ) -> Self {
        self.required_confidence = confidence;
        self
    }

    /// Installs the semantic verifier.
    #[must_use]
    pub fn with_semantic_verifier(
        mut self,
        verifier: Arc<dyn SemanticVerifier>,
    ) -> Self {
        self.semantic = Some(verifier);
        self
    }

    /// Installs the invariant verifier.
    #[must_use]
    pub fn with_invariant_verifier(
        mut self,
        verifier: Arc<dyn InvariantVerifier>,
    ) -> Self {
        self.invariants = Some(verifier);
        self
    }

    /// Installs the result verifier.
    #[must_use]
    pub fn with_result_verifier(
        mut self,
        verifier: Arc<dyn ResultVerifier>,
    ) -> Self {
        self.result = Some(verifier);
        self
    }

    /// Installs the provenance verifier.
    #[must_use]
    pub fn with_provenance_verifier(
        mut self,
        verifier: Arc<dyn ProvenanceVerifier>,
    ) -> Self {
        self.provenance = Some(verifier);
        self
    }

    /// Installs the confidence verifier.
    #[must_use]
    pub fn with_confidence_verifier(
        mut self,
        verifier: Arc<dyn ConfidenceVerifier>,
    ) -> Self {
        self.confidence = Some(verifier);
        self
    }

    /// Installs the recovery-integrity verifier.
    #[must_use]
    pub fn with_recovery_integrity_verifier(
        mut self,
        verifier: Arc<dyn RecoveryIntegrityVerifier>,
    ) -> Self {
        self.recovery_integrity = Some(verifier);
        self
    }

    /// Installs a resource identity verifier.
    #[must_use]
    pub fn with_resource_identity_verifier(
        mut self,
        verifier: Arc<dyn ResourceIdentityVerifier>,
    ) -> Self {
        self.resource_identity = Some(verifier);
        self
    }

    /// Returns the policy.
    #[must_use]
    pub const fn policy(&self) -> VerificationPolicy {
        self.policy
    }

    /// Returns the configured aggregate confidence requirement.
    #[must_use]
    pub const fn required_confidence(&self) -> VerificationConfidence {
        self.required_confidence
    }

    /// Performs complete authoritative verification.
    ///
    /// The function never returns an accepted result when a mandatory
    /// verification stage is missing or failed.
    pub fn verify(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationReport> {
        self.validate_request(request)?;

        let mut stages = Vec::new();
        let mut violations = Vec::new();

        // ---------------------------------------------------------------------
        // 1. Execution
        // ---------------------------------------------------------------------

        let execution_evidence = if self.policy.execution() {
            if request.execution_completed_successfully() {
                VerificationEvidence::passed(VerificationConfidence::certain())
            } else {
                VerificationEvidence::new(
                    VerificationState::Failed,
                    VerificationConfidence::certain(),
                    [VerificationViolation::new(
                        VerificationStage::Execution,
                        VerificationViolationKind::ExecutionFailure,
                        "QR-VER-EXECUTION-001",
                        "execution did not report successful completion",
                    )],
                )
            }
        } else {
            VerificationEvidence::not_required()
        };

        self.record(
            VerificationStage::Execution,
            "builtin.execution",
            execution_evidence,
            &mut stages,
            &mut violations,
        );

        // ---------------------------------------------------------------------
        // 2. Resource identity
        // ---------------------------------------------------------------------

        if self.policy.resource_identity() {
            let evidence = match &self.resource_identity {
                Some(verifier) => verifier.verify_resource_identity(request)?,
                None => VerificationEvidence::new(
                    VerificationState::Indeterminate,
                    VerificationConfidence::none(),
                    [VerificationViolation::new(
                        VerificationStage::ResourceIdentity,
                        VerificationViolationKind::MissingEvidence,
                        "QR-VER-RESOURCE-003",
                        "resource identity verifier is not configured",
                    )],
                ),
            };

            let component_id = self
                .resource_identity
                .as_ref()
                .map(|verifier| verifier.id())
                .unwrap_or("missing.resource_identity.verifier");

            self.record(
                VerificationStage::ResourceIdentity,
                component_id,
                evidence,
                &mut stages,
                &mut violations,
            );
        } else {
            stages.push(VerificationStageResult::from_evidence(
                VerificationStage::ResourceIdentity,
                "policy.disabled.resource_identity",
                VerificationEvidence::not_required(),
            ));
        }

        // ---------------------------------------------------------------------
        // 3. Invariants
        // ---------------------------------------------------------------------

        if self.policy.invariants() {
            let evidence = match &self.invariants {
                Some(verifier) => verifier.verify_invariants(request)?,
                None => VerificationEvidence::new(
                    VerificationState::Indeterminate,
                    VerificationConfidence::none(),
                    [VerificationViolation::new(
                        VerificationStage::Invariants,
                        VerificationViolationKind::MissingEvidence,
                        "QR-VER-INVARIANT-001",
                        "invariant verifier is not configured",
                    )],
                ),
            };

            let component_id = self
                .invariants
                .as_ref()
                .map(|verifier| verifier.id())
                .unwrap_or("missing.invariant.verifier");

            self.record(
                VerificationStage::Invariants,
                component_id,
                evidence,
                &mut stages,
                &mut violations,
            );
        } else {
            stages.push(VerificationStageResult::from_evidence(
                VerificationStage::Invariants,
                "policy.disabled.invariants",
                VerificationEvidence::not_required(),
            ));
        }

        // ---------------------------------------------------------------------
        // 4. Semantic equivalence
        // ---------------------------------------------------------------------

        if self.policy.semantic() {
            let evidence = self.verify_semantic_stage(request)?;

            let component_id = self
                .semantic
                .as_ref()
                .map(|verifier| verifier.id())
                .unwrap_or("missing.semantic.verifier");

            self.record(
                VerificationStage::Semantic,
                component_id,
                evidence,
                &mut stages,
                &mut violations,
            );
        } else {
            stages.push(VerificationStageResult::from_evidence(
                VerificationStage::Semantic,
                "policy.disabled.semantic",
                VerificationEvidence::not_required(),
            ));
        }

        // ---------------------------------------------------------------------
        // 5. Result
        // ---------------------------------------------------------------------

        if self.policy.result() {
            let evidence = match &self.result {
                Some(verifier) => verifier.verify_result(request)?,
                None => VerificationEvidence::new(
                    VerificationState::Indeterminate,
                    VerificationConfidence::none(),
                    [VerificationViolation::new(
                        VerificationStage::Result,
                        VerificationViolationKind::MissingEvidence,
                        "QR-VER-RESULT-001",
                        "result verifier is not configured",
                    )],
                ),
            };

            let component_id = self
                .result
                .as_ref()
                .map(|verifier| verifier.id())
                .unwrap_or("missing.result.verifier");

            self.record(
                VerificationStage::Result,
                component_id,
                evidence,
                &mut stages,
                &mut violations,
            );
        } else {
            stages.push(VerificationStageResult::from_evidence(
                VerificationStage::Result,
                "policy.disabled.result",
                VerificationEvidence::not_required(),
            ));
        }

        // ---------------------------------------------------------------------
        // 6. Provenance
        // ---------------------------------------------------------------------

        if self.policy.provenance() {
            let evidence = match &self.provenance {
                Some(verifier) => verifier.verify_provenance(request)?,
                None => VerificationEvidence::new(
                    VerificationState::Indeterminate,
                    VerificationConfidence::none(),
                    [VerificationViolation::new(
                        VerificationStage::Provenance,
                        VerificationViolationKind::MissingEvidence,
                        "QR-VER-PROVENANCE-001",
                        "provenance verifier is not configured",
                    )],
                ),
            };

            let component_id = self
                .provenance
                .as_ref()
                .map(|verifier| verifier.id())
                .unwrap_or("missing.provenance.verifier");

            self.record(
                VerificationStage::Provenance,
                component_id,
                evidence,
                &mut stages,
                &mut violations,
            );
        } else {
            stages.push(VerificationStageResult::from_evidence(
                VerificationStage::Provenance,
                "policy.disabled.provenance",
                VerificationEvidence::not_required(),
            ));
        }

        // ---------------------------------------------------------------------
        // 7. Confidence
        // ---------------------------------------------------------------------

        if self.policy.confidence() {
            let evidence = match &self.confidence {
                Some(verifier) => verifier.verify_confidence(request)?,
                None => VerificationEvidence::new(
                    VerificationState::Indeterminate,
                    VerificationConfidence::none(),
                    [VerificationViolation::new(
                        VerificationStage::Confidence,
                        VerificationViolationKind::MissingEvidence,
                        "QR-VER-CONFIDENCE-001",
                        "confidence verifier is not configured",
                    )],
                ),
            };

            let component_id = self
                .confidence
                .as_ref()
                .map(|verifier| verifier.id())
                .unwrap_or("missing.confidence.verifier");

            self.record(
                VerificationStage::Confidence,
                component_id,
                evidence,
                &mut stages,
                &mut violations,
            );
        } else {
            stages.push(VerificationStageResult::from_evidence(
                VerificationStage::Confidence,
                "policy.disabled.confidence",
                VerificationEvidence::not_required(),
            ));
        }

        // ---------------------------------------------------------------------
        // 8. Recovery/adaptation integrity
        // ---------------------------------------------------------------------

        if self.policy.recovery_integrity() && request.recovery_or_adaptation_performed() {
            let evidence = match &self.recovery_integrity {
                Some(verifier) => verifier.verify_recovery_integrity(request)?,
                None => VerificationEvidence::new(
                    VerificationState::Indeterminate,
                    VerificationConfidence::none(),
                    [VerificationViolation::new(
                        VerificationStage::RecoveryIntegrity,
                        VerificationViolationKind::MissingEvidence,
                        "QR-VER-RECOVERY-001",
                        "recovery-integrity verifier is required because recovery or adaptation occurred",
                    )],
                ),
            };

            let component_id = self
                .recovery_integrity
                .as_ref()
                .map(|verifier| verifier.id())
                .unwrap_or("missing.recovery_integrity.verifier");

            self.record(
                VerificationStage::RecoveryIntegrity,
                component_id,
                evidence,
                &mut stages,
                &mut violations,
            );
        } else {
            stages.push(VerificationStageResult::from_evidence(
                VerificationStage::RecoveryIntegrity,
                "policy.disabled.or.not_applicable.recovery_integrity",
                VerificationEvidence::not_required(),
            ));
        }

        // ---------------------------------------------------------------------
        // 9. Aggregate
        // ---------------------------------------------------------------------

        let overall_confidence = self.aggregate_confidence(&stages);

        let overall_state = if stages
            .iter()
            .any(|stage| stage.state() == VerificationState::Failed)
        {
            VerificationState::Failed
        } else if stages
            .iter()
            .any(|stage| stage.state() == VerificationState::Indeterminate)
        {
            VerificationState::Indeterminate
        } else {
            VerificationState::Passed
        };

        if overall_confidence < self.required_confidence {
            violations.push(VerificationViolation::new(
                VerificationStage::Confidence,
                VerificationViolationKind::ConfidenceInsufficient,
                "QR-VER-CONFIDENCE-002",
                "aggregate verification confidence is below the required level",
            ));
        }

        let decision = self.decide(
            request,
            overall_state,
            overall_confidence,
            &violations,
        );

        if decision.is_accepted() && !violations.is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::SemanticVerificationFailed,
                "verification attempted to accept a result while violations remained",
            ));
        }

        Ok(VerificationReport::new(
            request.execution_id().clone(),
            decision,
            overall_state,
            overall_confidence,
            stages,
            violations,
        ))
    }

    // -------------------------------------------------------------------------
    // Request validation
    // -------------------------------------------------------------------------

    fn validate_request(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<()> {
        if request.execution_id().as_str().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidConfiguration,
                "verification execution identity must not be empty",
            ));
        }

        if request.expected_semantic_fingerprint().as_str().is_empty() {
            return Err(ResilienceError::new(
                ResilienceErrorCode::InvalidConfiguration,
                "expected semantic fingerprint must not be empty",
            ));
        }

        if self.policy.semantic()
            && request.candidate_semantic_fingerprint().is_none()
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::SemanticVerificationFailed,
                "semantic verification requires a candidate semantic fingerprint",
            ));
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Semantic stage
    // -------------------------------------------------------------------------

    fn verify_semantic_stage(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        let verifier = match &self.semantic {
            Some(verifier) => verifier,
            None => {
                return Ok(VerificationEvidence::new(
                    VerificationState::Indeterminate,
                    VerificationConfidence::none(),
                    [VerificationViolation::new(
                        VerificationStage::Semantic,
                        VerificationViolationKind::MissingEvidence,
                        "QR-VER-SEMANTIC-001",
                        "semantic verifier is not configured",
                    )],
                ));
            }
        };

        let candidate = match request.candidate_semantic_fingerprint() {
            Some(candidate) => candidate,
            None => {
                return Ok(VerificationEvidence::new(
                    VerificationState::Indeterminate,
                    VerificationConfidence::none(),
                    [VerificationViolation::new(
                        VerificationStage::Semantic,
                        VerificationViolationKind::MissingEvidence,
                        "QR-VER-SEMANTIC-002",
                        "candidate semantic fingerprint is missing",
                    )],
                ));
            }
        };

        // The fingerprint comparison is a cheap, deterministic precondition.
        //
        // It does NOT replace canonical semantic verification. A canonical
        // semantic verifier may establish equivalence even where fingerprints
        // are unavailable or are generated by a compatible alternate method.
        //
        // When both fingerprints are present and unequal, however, this is an
        // authoritative negative signal and there is no reason to claim
        // equivalence merely because another subsystem says execution
        // completed successfully.
        if request.expected_semantic_fingerprint() != candidate {
            return Ok(VerificationEvidence::new(
                VerificationState::Failed,
                VerificationConfidence::certain(),
                [VerificationViolation::new(
                    VerificationStage::Semantic,
                    VerificationViolationKind::SemanticMismatch,
                    "QR-VER-SEMANTIC-003",
                    "expected and candidate semantic fingerprints differ",
                )],
            ));
        }

        verifier.verify_semantics(request)
    }

    // -------------------------------------------------------------------------
    // Stage recording
    // -------------------------------------------------------------------------

    fn record(
        &self,
        stage: VerificationStage,
        component_id: &str,
        evidence: VerificationEvidence,
        stages: &mut Vec<VerificationStageResult>,
        violations: &mut Vec<VerificationViolation>,
    ) {
        violations.extend(evidence.violations().iter().cloned());

        stages.push(VerificationStageResult::from_evidence(
            stage,
            component_id,
            evidence,
        ));
    }

    // -------------------------------------------------------------------------
    // Confidence aggregation
    // -------------------------------------------------------------------------

    fn aggregate_confidence(
        &self,
        stages: &[VerificationStageResult],
    ) -> VerificationConfidence {
        let mut minimum = VerificationConfidence::certain();

        for stage in stages {
            if stage.state() == VerificationState::NotRequired {
                continue;
            }

            if stage.confidence() < minimum {
                minimum = stage.confidence();
            }
        }

        minimum
    }

    // -------------------------------------------------------------------------
    // Final acceptance
    // -------------------------------------------------------------------------

    fn decide(
        &self,
        request: &VerificationRequest,
        state: VerificationState,
        confidence: VerificationConfidence,
        violations: &[VerificationViolation],
    ) -> VerificationDecision {
        if !violations.is_empty() {
            return match state {
                VerificationState::Failed => {
                    if request.recovery_or_adaptation_performed() {
                        VerificationDecision::Repeat
                    } else {
                        VerificationDecision::Reject
                    }
                }
                VerificationState::Indeterminate => VerificationDecision::Escalate,
                VerificationState::Passed | VerificationState::NotRequired => {
                    VerificationDecision::Reject
                }
            };
        }

        if !confidence.meets(self.required_confidence) {
            return VerificationDecision::Escalate;
        }

        match state {
            VerificationState::Passed => VerificationDecision::Accept,
            VerificationState::NotRequired => VerificationDecision::Reject,
            VerificationState::Failed => VerificationDecision::Reject,
            VerificationState::Indeterminate => VerificationDecision::Escalate,
        }
    }
}

// =============================================================================
// Convenience constructor
// =============================================================================

impl Default for ResilienceVerifier {
    fn default() -> Self {
        Self::strict()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn execution_id() -> ExecutionIdentity {
        ExecutionIdentity::new("execution-1").expect("valid execution identity")
    }

    fn fingerprint(value: &str) -> SemanticFingerprint {
        SemanticFingerprint::new(value).expect("valid fingerprint")
    }

    fn request() -> VerificationRequest {
        VerificationRequest::new(
            execution_id(),
            fingerprint("semantic-a"),
            VerificationResourceScope::default(),
        )
        .with_candidate_semantic_fingerprint(fingerprint("semantic-a"))
        .with_execution_success()
    }

    #[derive(Debug)]
    struct PassingSemanticVerifier;

    impl SemanticVerifier for PassingSemanticVerifier {
        fn id(&self) -> &str {
            "test.semantic"
        }

        fn verify_semantics(
            &self,
            _request: &VerificationRequest,
        ) -> ResilienceResult<VerificationEvidence> {
            Ok(VerificationEvidence::passed(
                VerificationConfidence::certain(),
            ))
        }
    }

    #[derive(Debug)]
    struct PassingInvariantVerifier;

    impl InvariantVerifier for PassingInvariantVerifier {
        fn id(&self) -> &str {
            "test.invariants"
        }

        fn verify_invariants(
            &self,
            _request: &VerificationRequest,
        ) -> ResilienceResult<VerificationEvidence> {
            Ok(VerificationEvidence::passed(
                VerificationConfidence::certain(),
            ))
        }
    }

    #[derive(Debug)]
    struct PassingResultVerifier;

    impl ResultVerifier for PassingResultVerifier {
        fn id(&self) -> &str {
            "test.result"
        }

        fn verify_result(
            &self,
            _request: &VerificationRequest,
        ) -> ResilienceResult<VerificationEvidence> {
            Ok(VerificationEvidence::passed(
                VerificationConfidence::certain(),
            ))
        }
    }

    #[derive(Debug)]
    struct PassingProvenanceVerifier;

    impl ProvenanceVerifier for PassingProvenanceVerifier {
        fn id(&self) -> &str {
            "test.provenance"
        }

        fn verify_provenance(
            &self,
            _request: &VerificationRequest,
        ) -> ResilienceResult<VerificationEvidence> {
            Ok(VerificationEvidence::passed(
                VerificationConfidence::certain(),
            ))
        }
    }

    #[derive(Debug)]
    struct PassingConfidenceVerifier;

    impl ConfidenceVerifier for PassingConfidenceVerifier {
        fn id(&self) -> &str {
            "test.confidence"
        }

        fn verify_confidence(
            &self,
            _request: &VerificationRequest,
        ) -> ResilienceResult<VerificationEvidence> {
            Ok(VerificationEvidence::passed(
                VerificationConfidence::certain(),
            ))
        }
    }

    #[derive(Debug)]
    struct PassingRecoveryVerifier;

    impl RecoveryIntegrityVerifier for PassingRecoveryVerifier {
        fn id(&self) -> &str {
            "test.recovery"
        }

        fn verify_recovery_integrity(
            &self,
            _request: &VerificationRequest,
        ) -> ResilienceResult<VerificationEvidence> {
            Ok(VerificationEvidence::passed(
                VerificationConfidence::certain(),
            ))
        }
    }

    fn production_verifier() -> ResilienceVerifier {
        ResilienceVerifier::strict()
            .with_semantic_verifier(Arc::new(PassingSemanticVerifier))
            .with_invariant_verifier(Arc::new(PassingInvariantVerifier))
            .with_result_verifier(Arc::new(PassingResultVerifier))
            .with_provenance_verifier(Arc::new(PassingProvenanceVerifier))
            .with_confidence_verifier(Arc::new(PassingConfidenceVerifier))
            .with_recovery_integrity_verifier(Arc::new(PassingRecoveryVerifier))
    }

    #[test]
    fn strict_policy_requires_all_dimensions() {
        let policy = VerificationPolicy::strict();

        assert!(policy.semantic());
        assert!(policy.invariants());
        assert!(policy.result());
        assert!(policy.provenance());
        assert!(policy.confidence());
        assert!(policy.resource_identity());
        assert!(policy.execution());
        assert!(policy.recovery_integrity());
    }

    #[test]
    fn equal_semantic_fingerprints_can_pass_precondition() {
        let verifier = production_verifier();
        let report = verifier
            .verify(&request())
            .expect("verification should succeed");

        assert_eq!(report.decision(), VerificationDecision::Accept);
        assert!(report.accepted());
        assert!(report.violations().is_empty());
    }

    #[test]
    fn failed_execution_cannot_be_accepted() {
        let verifier = production_verifier();

        let request = VerificationRequest::new(
            execution_id(),
            fingerprint("semantic-a"),
            VerificationResourceScope::default(),
        )
        .with_candidate_semantic_fingerprint(fingerprint("semantic-a"));

        let report = verifier
            .verify(&request)
            .expect("verification should produce a report");

        assert!(!report.accepted());
        assert!(report.violations().iter().any(|violation| {
            violation.kind() == VerificationViolationKind::ExecutionFailure
        }));
    }

    #[test]
    fn different_semantic_fingerprints_are_rejected() {
        let verifier = production_verifier();

        let request = VerificationRequest::new(
            execution_id(),
            fingerprint("semantic-a"),
            VerificationResourceScope::default(),
        )
        .with_candidate_semantic_fingerprint(fingerprint("semantic-b"))
        .with_execution_success();

        let report = verifier
            .verify(&request)
            .expect("verification should produce a report");

        assert!(!report.accepted());
        assert!(report.violations().iter().any(|violation| {
            violation.kind() == VerificationViolationKind::SemanticMismatch
        }));
    }

    #[test]
    fn missing_mandatory_component_is_not_accepted() {
        let verifier = ResilienceVerifier::strict()
            .with_semantic_verifier(Arc::new(PassingSemanticVerifier));

        let report = verifier
            .verify(&request())
            .expect("verification should produce a report");

        assert!(!report.accepted());
        assert!(report.violations().iter().any(|violation| {
            violation.kind() == VerificationViolationKind::MissingEvidence
        }));
    }

    #[test]
    fn recovery_requires_recovery_integrity_verification() {
        let verifier = ResilienceVerifier::strict()
            .with_semantic_verifier(Arc::new(PassingSemanticVerifier))
            .with_invariant_verifier(Arc::new(PassingInvariantVerifier))
            .with_result_verifier(Arc::new(PassingResultVerifier))
            .with_provenance_verifier(Arc::new(PassingProvenanceVerifier))
            .with_confidence_verifier(Arc::new(PassingConfidenceVerifier));

        let request = request().with_recovery_or_adaptation();

        let report = verifier
            .verify(&request)
            .expect("verification should produce a report");

        assert!(!report.accepted());
        assert!(report.violations().iter().any(|violation| {
            violation.kind() == VerificationViolationKind::RecoveryIntegrityFailure
                || violation.kind() == VerificationViolationKind::MissingEvidence
        }));
    }

    #[test]
    fn recovery_can_be_accepted_after_integrity_verification() {
        let verifier = production_verifier();

        let request = request().with_recovery_or_adaptation();

        let report = verifier
            .verify(&request)
            .expect("verification should succeed");

        assert_eq!(report.decision(), VerificationDecision::Accept);
    }

    #[test]
    fn logical_and_physical_resources_are_independent() {
        let scope = VerificationResourceScope::new(
            [QubitId::new(0).expect("valid qubit")],
            [PhysicalQubitId::new(42).expect("valid physical qubit")],
        );

        assert_eq!(scope.logical_qubits().len(), 1);
        assert_eq!(scope.physical_qubits().len(), 1);
        assert_ne!(
            scope.logical_qubits()[0].index(),
            scope.physical_qubits()[0].index()
        );
    }

    #[test]
    fn confidence_is_exact_and_orderable() {
        let low =
            VerificationConfidence::from_basis_points(9_000).expect("valid confidence");
        let high =
            VerificationConfidence::from_basis_points(9_500).expect("valid confidence");

        assert!(high > low);
        assert!(high.meets(low));
        assert!(!low.meets(high));
    }

    #[test]
    fn verification_decisions_have_safe_acceptance_semantics() {
        assert!(VerificationDecision::Accept.is_accepted());
        assert!(VerificationDecision::DegradedAccept.is_accepted());

        assert!(!VerificationDecision::Repeat.is_accepted());
        assert!(!VerificationDecision::Escalate.is_accepted());
        assert!(!VerificationDecision::Reject.is_accepted());
    }

    #[test]
    fn schema_is_stable_and_non_empty() {
        assert!(!RESILIENCE_VERIFIER_SCHEMA_ID.is_empty());
        assert!(RESILIENCE_VERIFIER_SCHEMA_VERSION > 0);
    }
}