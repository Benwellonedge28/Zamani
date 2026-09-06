//! Zamani Quantum Resilience — Production Response Contract
//!
//! Path:
//!     src/quantum/resilience/api/response.rs
//!
//! Purpose:
//!     Defines the immutable, provider-independent result returned by the
//!     quantum resilience orchestration boundary.
//!
//! Architectural role:
//!     `ResilienceResponse` is the stable public result envelope produced
//!     after a resilience lifecycle has completed its current orchestration
//!     cycle.
//!
//! The response records:
//!
//!     request identity
//!     cycle identity
//!     lifecycle outcome
//!     execution status
//!     verification status
//!     degradation state
//!     adaptation/recovery/mitigation summary
//!     affected canonical logical/physical resources
//!     execution artifact
//!     verification artifact
//!     provenance artifact
//!
//! The response does NOT implement:
//!
//!     detection
//!     diagnosis
//!     policy
//!     planning
//!     routing
//!     scheduling
//!     optimization
//!     QEC
//!     recovery
//!     mitigation
//!     verification
//!     hardware execution
//!
//! Those responsibilities remain in their authoritative subsystems.
//!
//! # Integration boundary
//!
//! ```text
//! Zamani Program
//!       |
//!       v
//! ResilienceRequest
//!       |
//!       v
//! ResilienceController
//!       |
//!       +--> detection
//!       +--> diagnosis
//!       +--> policy
//!       +--> planning
//!       +--> adaptation
//!       +--> recovery
//!       +--> mitigation
//!       +--> verification
//!       |
//!       v
//! ResilienceResponse
//!       |
//!       +--> runtime
//!       +--> history
//!       +--> telemetry
//!       +--> serialization
//!       +--> caller
//! ```
//!
//! # Write once, scale everywhere
//!
//! This module intentionally contains no:
//!
//!     MAX_QUBITS
//!     MAX_PHYSICAL_QUBITS
//!     MAX_BACKENDS
//!     MAX_INCIDENTS
//!     MAX_RECOVERY_ATTEMPTS
//!     DEFAULT_RETRY_COUNT
//!     DEFAULT_FIDELITY_THRESHOLD
//!     provider-specific device IDs
//!
//! Resource cardinalities are observations supplied by the execution and
//! resilience subsystems. The response merely records them.
//!
//! "Infinite" scalability therefore means that this API imposes no artificial
//! finite quantum-machine ceiling. Actual finite limits remain those of the
//! executing environment and its available resources.
//!
//! # Canonical quantum identity
//!
//! Logical and physical qubit identities MUST use the canonical IR types:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! This module never converts either identity into an untyped integer.
//!
//! A logical-to-physical mapping is reported only when an authoritative
//! routing/placement subsystem supplies it. The response never invents one.
//!
//! # Verification invariant
//!
//! An execution completing successfully is not equivalent to a verified
//! quantum result.
//!
//! ```text
//! execution_success != semantic_acceptance
//! ```
//!
//! The response therefore distinguishes:
//!
//!     execution status
//!     verification status
//!     final resilience decision
//!
//! A caller MUST inspect the final decision and verification state before
//! treating an execution result as semantically accepted.
//!
//! # Determinism
//!
//! This module does not:
//!
//!     read the clock
//!     generate randomness
//!     access environment variables
//!     access global mutable state
//!     perform I/O
//!
//! Timestamps, trace identifiers, hashes and other provenance values must be
//! supplied by the orchestration context.
//!
//! # Ownership and scalability
//!
//! Large artifacts are stored behind `Arc` so that constructing and passing a
//! response does not require copying potentially large execution,
//! verification or provenance objects.
//!
//! Collections are represented using `Arc<[T]>` where practical. This gives
//! callers immutable, shareable snapshots without imposing a fixed capacity.
//!
//! # Rust contract
//!
//! - Rust 1.97 / 1.97.1
//! - Rust 2021
//! - stable Rust only
//! - no `unsafe`
//! - no hidden I/O
//! - no hidden concurrency
//! - no hard-coded hardware limits
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::resilience::api::controller::{
    ResilienceCycleId,
    ResilienceDecision,
};
use crate::quantum::resilience::api::request::ResilienceRequestId;

// =============================================================================
// Public schema
// =============================================================================

/// Stable schema identifier for the resilience response contract.
pub const RESILIENCE_RESPONSE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.api.response";

/// Semantic version of the response contract.
///
/// This version is independent of the quantum IR version.
pub const RESILIENCE_RESPONSE_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Execution status
// =============================================================================

/// Describes the outcome of the underlying execution attempt.
///
/// This is deliberately separate from `ResilienceDecision`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionStatus {
    /// No execution outcome is available yet.
    NotExecuted,

    /// Execution is currently represented as in progress.
    InProgress,

    /// The execution completed at the execution layer.
    Completed,

    /// Execution failed.
    Failed,

    /// Execution was intentionally cancelled.
    Cancelled,

    /// Execution was intentionally aborted by resilience policy.
    Aborted,

    /// Execution was interrupted and may be resumable.
    Interrupted,

    /// Execution state is unavailable or cannot currently be classified.
    Unknown,
}

impl Default for ExecutionStatus {
    fn default() -> Self {
        Self::NotExecuted
    }
}

impl ExecutionStatus {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotExecuted => "not_executed",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Aborted => "aborted",
            Self::Interrupted => "interrupted",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether execution completed at the execution layer.
    pub const fn is_completed(self) -> bool {
        matches!(self, Self::Completed)
    }

    /// Returns whether execution failed or was interrupted.
    pub const fn is_failed_or_interrupted(self) -> bool {
        matches!(
            self,
            Self::Failed | Self::Interrupted | Self::Aborted
        )
    }
}

impl fmt::Display for ExecutionStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Verification status
// =============================================================================

/// Describes the result of semantic/result verification.
///
/// Verification remains authoritative to the verification subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationStatus {
    /// Verification has not been performed.
    NotPerformed,

    /// Verification is currently in progress.
    InProgress,

    /// Verification established that the applicable contract was satisfied.
    Verified,

    /// Verification established that the result is valid but degraded within
    /// an explicitly permitted degradation contract.
    VerifiedDegraded,

    /// Verification found that the result does not satisfy the contract.
    Failed,

    /// Verification could not establish correctness.
    Inconclusive,

    /// Verification could not be completed because required evidence was
    /// unavailable.
    Unavailable,
}

impl Default for VerificationStatus {
    fn default() -> Self {
        Self::NotPerformed
    }
}

impl VerificationStatus {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotPerformed => "not_performed",
            Self::InProgress => "in_progress",
            Self::Verified => "verified",
            Self::VerifiedDegraded => "verified_degraded",
            Self::Failed => "failed",
            Self::Inconclusive => "inconclusive",
            Self::Unavailable => "unavailable",
        }
    }

    /// Returns whether verification establishes full acceptance evidence.
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }

    /// Returns whether verification establishes permitted degraded validity.
    pub const fn is_verified_degraded(self) -> bool {
        matches!(self, Self::VerifiedDegraded)
    }

    /// Returns whether verification failed.
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Returns whether verification did not establish a conclusion.
    pub const fn is_inconclusive(self) -> bool {
        matches!(
            self,
            Self::Inconclusive | Self::Unavailable | Self::NotPerformed
        )
    }
}

impl fmt::Display for VerificationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Degradation state
// =============================================================================

/// Describes the degree of degradation observed during execution.
///
/// This is a state classification, not a numerical threshold. Thresholds
/// remain policy/verification responsibilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DegradationStatus {
    /// No degradation was reported.
    None,

    /// Degradation exists but remains within the declared contract.
    Degraded,

    /// Degradation exceeded the currently acceptable operating envelope.
    Severe,

    /// The degradation state could not be established.
    Unknown,
}

impl Default for DegradationStatus {
    fn default() -> Self {
        Self::None
    }
}

impl DegradationStatus {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Degraded => "degraded",
            Self::Severe => "severe",
            Self::Unknown => "unknown",
        }
    }

    /// Returns whether any degradation was reported.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::None)
    }
}

impl fmt::Display for DegradationStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Lifecycle activity summary
// =============================================================================

/// Immutable summary of resilience actions performed during the cycle.
///
/// This does not replace detailed history or telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResilienceActivitySummary {
    adaptations: u64,
    recoveries: u64,
    mitigations: u64,
    verification_attempts: u64,
}

impl ResilienceActivitySummary {
    /// Creates an empty activity summary.
    pub const fn new() -> Self {
        Self {
            adaptations: 0,
            recoveries: 0,
            mitigations: 0,
            verification_attempts: 0,
        }
    }

    /// Creates a summary with explicit values.
    pub const fn from_counts(
        adaptations: u64,
        recoveries: u64,
        mitigations: u64,
        verification_attempts: u64,
    ) -> Self {
        Self {
            adaptations,
            recoveries,
            mitigations,
            verification_attempts,
        }
    }

    /// Number of adaptations.
    pub const fn adaptations(self) -> u64 {
        self.adaptations
    }

    /// Number of recoveries.
    pub const fn recoveries(self) -> u64 {
        self.recoveries
    }

    /// Number of mitigation operations.
    pub const fn mitigations(self) -> u64 {
        self.mitigations
    }

    /// Number of verification attempts.
    pub const fn verification_attempts(self) -> u64 {
        self.verification_attempts
    }
}

// =============================================================================
// Resource impact
// =============================================================================

/// Immutable description of quantum resources directly affected by the
/// resilience cycle.
///
/// Both logical and physical identifiers use the canonical IR vocabulary.
///
/// The response does not infer mappings. A physical identifier is included
/// only when an authoritative subsystem supplied it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResourceImpact {
    logical_qubits: Arc<[QubitId]>,
    physical_qubits: Arc<[PhysicalQubitId]>,
}

impl ResourceImpact {
    /// Creates an empty resource-impact record.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Creates a resource-impact record from canonical logical identities.
    pub fn logical<I>(logical_qubits: I) -> Self
    where
        I: IntoIterator<Item = QubitId>,
    {
        Self {
            logical_qubits: logical_qubits.into_iter().collect(),
            physical_qubits: Arc::from([]),
        }
    }

    /// Creates a resource-impact record from canonical logical and physical
    /// identities.
    ///
    /// The caller must obtain both collections from authoritative IR/routing/
    /// hardware contracts.
    pub fn from_parts<L, P>(
        logical_qubits: L,
        physical_qubits: P,
    ) -> Self
    where
        L: IntoIterator<Item = QubitId>,
        P: IntoIterator<Item = PhysicalQubitId>,
    {
        Self {
            logical_qubits: logical_qubits.into_iter().collect(),
            physical_qubits: physical_qubits.into_iter().collect(),
        }
    }

    /// Returns affected logical qubits.
    pub fn logical_qubits(&self) -> &[QubitId] {
        self.logical_qubits.as_ref()
    }

    /// Returns affected physical qubits.
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        self.physical_qubits.as_ref()
    }

    /// Returns the number of affected logical qubits.
    pub fn logical_qubit_count(&self) -> usize {
        self.logical_qubits.len()
    }

    /// Returns the number of affected physical qubits.
    pub fn physical_qubit_count(&self) -> usize {
        self.physical_qubits.len()
    }

    /// Returns whether no resource identities were reported.
    pub fn is_empty(&self) -> bool {
        self.logical_qubits.is_empty()
            && self.physical_qubits.is_empty()
    }
}

// =============================================================================
// Artifact identifiers
// =============================================================================

/// Stable identifier for a response artifact.
///
/// The response never interprets artifact contents. Interpretation belongs to
/// the subsystem that owns the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ArtifactId(Arc<str>);

impl ArtifactId {
    /// Creates an artifact identifier.
    ///
    /// Empty identifiers are rejected because an artifact reference without
    /// an identity cannot participate reliably in provenance.
    pub fn new(value: impl Into<Arc<str>>) -> Result<Self, ArtifactIdError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(ArtifactIdError::Empty);
        }

        Ok(Self(value))
    }

    /// Returns the identifier.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ArtifactId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Validation error for `ArtifactId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactIdError {
    /// The identifier was empty or whitespace-only.
    Empty,
}

impl fmt::Display for ArtifactIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str(
                "resilience artifact ID must not be empty",
            ),
        }
    }
}

impl std::error::Error for ArtifactIdError {}

// =============================================================================
// Artifact references
// =============================================================================

/// Reference to the execution artifact.
///
/// The actual execution-result type belongs to the execution subsystem and is
/// therefore represented generically here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionArtifact<T> {
    id: ArtifactId,
    value: Arc<T>,
}

impl<T> ExecutionArtifact<T> {
    /// Creates an execution artifact reference.
    pub fn new(id: ArtifactId, value: Arc<T>) -> Self {
        Self { id, value }
    }

    /// Returns the artifact identity.
    pub fn id(&self) -> &ArtifactId {
        &self.id
    }

    /// Borrows the artifact.
    pub fn value(&self) -> &T {
        self.value.as_ref()
    }

    /// Returns a shared artifact handle.
    pub fn value_arc(&self) -> Arc<T> {
        Arc::clone(&self.value)
    }

    /// Consumes the reference and returns the shared artifact.
    pub fn into_value_arc(self) -> Arc<T> {
        self.value
    }
}

/// Reference to a verification artifact.
///
/// The concrete verification type belongs to `verification/`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationArtifact<T> {
    id: ArtifactId,
    value: Arc<T>,
}

impl<T> VerificationArtifact<T> {
    /// Creates a verification artifact reference.
    pub fn new(id: ArtifactId, value: Arc<T>) -> Self {
        Self { id, value }
    }

    /// Returns the artifact identity.
    pub fn id(&self) -> &ArtifactId {
        &self.id
    }

    /// Borrows the verification artifact.
    pub fn value(&self) -> &T {
        self.value.as_ref()
    }

    /// Returns a shared artifact handle.
    pub fn value_arc(&self) -> Arc<T> {
        Arc::clone(&self.value)
    }
}

/// Reference to a provenance artifact.
///
/// The concrete provenance representation belongs to `verification/provenance`
/// and/or the serialization/history layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceArtifact<T> {
    id: ArtifactId,
    value: Arc<T>,
}

impl<T> ProvenanceArtifact<T> {
    /// Creates a provenance artifact reference.
    pub fn new(id: ArtifactId, value: Arc<T>) -> Self {
        Self { id, value }
    }

    /// Returns the artifact identity.
    pub fn id(&self) -> &ArtifactId {
        &self.id
    }

    /// Borrows the provenance artifact.
    pub fn value(&self) -> &T {
        self.value.as_ref()
    }

    /// Returns a shared provenance artifact handle.
    pub fn value_arc(&self) -> Arc<T> {
        Arc::clone(&self.value)
    }
}

// =============================================================================
// Response metadata
// =============================================================================

/// Immutable metadata associated with one response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResilienceResponseMetadata {
    schema_id: &'static str,
    schema_version: u16,
}

impl Default for ResilienceResponseMetadata {
    fn default() -> Self {
        Self {
            schema_id: RESILIENCE_RESPONSE_SCHEMA_ID,
            schema_version: RESILIENCE_RESPONSE_SCHEMA_VERSION,
        }
    }
}

impl ResilienceResponseMetadata {
    /// Creates metadata for the current response schema.
    pub const fn current() -> Self {
        Self {
            schema_id: RESILIENCE_RESPONSE_SCHEMA_ID,
            schema_version: RESILIENCE_RESPONSE_SCHEMA_VERSION,
        }
    }

    /// Returns the schema identifier.
    pub const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    /// Returns the schema version.
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }
}

// =============================================================================
// Response
// =============================================================================

/// Immutable production response returned by the resilience API.
///
/// `E`, `V` and `P` deliberately remain generic:
///
/// - `E` = execution artifact
/// - `V` = verification artifact
/// - `P` = provenance artifact
///
/// This prevents the resilience API from defining duplicate execution,
/// verification or provenance models.
///
/// Typical integration:
///
/// ```text
/// ResilienceResponse<
///     RuntimeExecutionResult,
///     VerificationResult,
///     ProvenanceRecord,
/// >
/// ```
///
/// The actual repository implementations can evolve independently while the
/// resilience response envelope remains stable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResilienceResponse<E, V, P> {
    metadata: ResilienceResponseMetadata,

    request_id: ResilienceRequestId,

    cycle_id: ResilienceCycleId,

    decision: ResilienceDecision,

    execution_status: ExecutionStatus,

    verification_status: VerificationStatus,

    degradation: DegradationStatus,

    activity: ResilienceActivitySummary,

    resources: ResourceImpact,

    execution: Option<ExecutionArtifact<E>>,

    verification: Option<VerificationArtifact<V>>,

    provenance: Option<ProvenanceArtifact<P>>,
}

impl<E, V, P> ResilienceResponse<E, V, P> {
    /// Creates a response from its complete immutable components.
    ///
    /// This constructor performs structural validation only. Semantic
    /// acceptance remains the responsibility of the verification subsystem.
    pub fn new(
        request_id: ResilienceRequestId,
        cycle_id: ResilienceCycleId,
        decision: ResilienceDecision,
        execution_status: ExecutionStatus,
        verification_status: VerificationStatus,
        degradation: DegradationStatus,
        activity: ResilienceActivitySummary,
        resources: ResourceImpact,
        execution: Option<ExecutionArtifact<E>>,
        verification: Option<VerificationArtifact<V>>,
        provenance: Option<ProvenanceArtifact<P>>,
    ) -> Result<Self, ResponseValidationError> {
        let response = Self {
            metadata: ResilienceResponseMetadata::current(),
            request_id,
            cycle_id,
            decision,
            execution_status,
            verification_status,
            degradation,
            activity,
            resources,
            execution,
            verification,
            provenance,
        };

        response.validate()?;

        Ok(response)
    }

    /// Returns response metadata.
    pub const fn metadata(&self) -> &ResilienceResponseMetadata {
        &self.metadata
    }

    /// Returns the request identity.
    pub fn request_id(&self) -> &ResilienceRequestId {
        &self.request_id
    }

    /// Returns the resilience cycle identity.
    pub const fn cycle_id(&self) -> ResilienceCycleId {
        self.cycle_id
    }

    /// Returns the final resilience decision.
    pub const fn decision(&self) -> ResilienceDecision {
        self.decision
    }

    /// Returns execution status.
    pub const fn execution_status(&self) -> ExecutionStatus {
        self.execution_status
    }

    /// Returns verification status.
    pub const fn verification_status(&self) -> VerificationStatus {
        self.verification_status
    }

    /// Returns degradation status.
    pub const fn degradation(&self) -> DegradationStatus {
        self.degradation
    }

    /// Returns activity summary.
    pub const fn activity(&self) -> ResilienceActivitySummary {
        self.activity
    }

    /// Returns resource impact.
    pub const fn resources(&self) -> &ResourceImpact {
        &self.resources
    }

    /// Returns the execution artifact, when one exists.
    pub fn execution(&self) -> Option<&ExecutionArtifact<E>> {
        self.execution.as_ref()
    }

    /// Returns the verification artifact, when one exists.
    pub fn verification(&self) -> Option<&VerificationArtifact<V>> {
        self.verification.as_ref()
    }

    /// Returns the provenance artifact, when one exists.
    pub fn provenance(&self) -> Option<&ProvenanceArtifact<P>> {
        self.provenance.as_ref()
    }

    /// Returns whether the response is fully verified.
    pub const fn is_verified(&self) -> bool {
        self.verification_status.is_verified()
    }

    /// Returns whether the response is verified but degraded.
    pub const fn is_verified_degraded(&self) -> bool {
        self.verification_status.is_verified_degraded()
    }

    /// Returns whether the final decision accepts the result.
    pub const fn is_accepted(&self) -> bool {
        self.decision.is_accepted()
    }

    /// Returns whether the final decision requests another cycle.
    pub const fn requires_repeat(&self) -> bool {
        self.decision.requires_repeat()
    }

    /// Returns whether the final decision requires escalation.
    pub const fn requires_escalation(&self) -> bool {
        self.decision.requires_escalation()
    }

    /// Returns whether the response contains an execution artifact.
    pub fn has_execution_artifact(&self) -> bool {
        self.execution.is_some()
    }

    /// Returns whether the response contains verification evidence.
    pub fn has_verification_artifact(&self) -> bool {
        self.verification.is_some()
    }

    /// Returns whether the response contains provenance evidence.
    pub fn has_provenance_artifact(&self) -> bool {
        self.provenance.is_some()
    }

    /// Validates response invariants.
    ///
    /// This validation deliberately checks only invariants that can be
    /// established without inspecting the opaque execution/verification/
    /// provenance artifacts.
    pub fn validate(&self) -> Result<(), ResponseValidationError> {
        match self.decision {
            ResilienceDecision::Accept => {
                if !self.verification_status.is_verified() {
                    return Err(
                        ResponseValidationError::AcceptedWithoutVerification,
                    );
                }
            }

            ResilienceDecision::DegradedAccept => {
                if !self
                    .verification_status
                    .is_verified_degraded()
                {
                    return Err(
                        ResponseValidationError::DegradedAcceptedWithoutVerifiedDegradation,
                    );
                }

                if !self.degradation.is_degraded() {
                    return Err(
                        ResponseValidationError::DegradedAcceptanceWithoutDegradation,
                    );
                }
            }

            ResilienceDecision::Repeat
            | ResilienceDecision::Escalate
            | ResilienceDecision::Reject => {}
        }

        if self.decision.is_accepted()
            && !self.execution_status.is_completed()
        {
            return Err(
                ResponseValidationError::AcceptedWithoutCompletedExecution,
            );
        }

        if self.verification_status.is_verified()
            && self.verification.is_none()
        {
            return Err(
                ResponseValidationError::VerifiedWithoutVerificationArtifact,
            );
        }

        Ok(())
    }
}

// =============================================================================
// Response validation
// =============================================================================

/// Structural validation failures for `ResilienceResponse`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseValidationError {
    /// A fully accepted result did not have successful verification.
    AcceptedWithoutVerification,

    /// A degraded acceptance did not have verified degraded evidence.
    DegradedAcceptedWithoutVerifiedDegradation,

    /// A degraded acceptance was returned without a reported degradation.
    DegradedAcceptanceWithoutDegradation,

    /// An accepted result was returned without completed execution.
    AcceptedWithoutCompletedExecution,

    /// Verification was marked successful without a verification artifact.
    VerifiedWithoutVerificationArtifact,
}

impl fmt::Display for ResponseValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AcceptedWithoutVerification => formatter.write_str(
                "accepted resilience response requires verified execution",
            ),

            Self::DegradedAcceptedWithoutVerifiedDegradation => {
                formatter.write_str(
                    "degraded acceptance requires verified degraded evidence",
                )
            }

            Self::DegradedAcceptanceWithoutDegradation => formatter.write_str(
                "degraded acceptance requires an explicit degradation state",
            ),

            Self::AcceptedWithoutCompletedExecution => formatter.write_str(
                "accepted resilience response requires completed execution",
            ),

            Self::VerifiedWithoutVerificationArtifact => formatter.write_str(
                "verified response requires a verification artifact",
            ),
        }
    }
}

impl std::error::Error for ResponseValidationError {}

// =============================================================================
// Builder
// =============================================================================

/// Builder for `ResilienceResponse`.
///
/// The builder keeps construction stable while allowing the underlying
/// response schema to evolve without exposing field layout.
#[derive(Debug, Clone)]
pub struct ResilienceResponseBuilder<E, V, P> {
    request_id: ResilienceRequestId,
    cycle_id: ResilienceCycleId,
    decision: ResilienceDecision,
    execution_status: ExecutionStatus,
    verification_status: VerificationStatus,
    degradation: DegradationStatus,
    activity: ResilienceActivitySummary,
    resources: ResourceImpact,
    execution: Option<ExecutionArtifact<E>>,
    verification: Option<VerificationArtifact<V>>,
    provenance: Option<ProvenanceArtifact<P>>,
}

impl<E, V, P> ResilienceResponseBuilder<E, V, P> {
    /// Creates a response builder.
    pub fn new(
        request_id: ResilienceRequestId,
        cycle_id: ResilienceCycleId,
    ) -> Self {
        Self {
            request_id,
            cycle_id,
            decision: ResilienceDecision::Reject,
            execution_status: ExecutionStatus::NotExecuted,
            verification_status: VerificationStatus::NotPerformed,
            degradation: DegradationStatus::None,
            activity: ResilienceActivitySummary::new(),
            resources: ResourceImpact::empty(),
            execution: None,
            verification: None,
            provenance: None,
        }
    }

    /// Sets the final resilience decision.
    pub fn decision(
        mut self,
        decision: ResilienceDecision,
    ) -> Self {
        self.decision = decision;
        self
    }

    /// Sets execution status.
    pub fn execution_status(
        mut self,
        status: ExecutionStatus,
    ) -> Self {
        self.execution_status = status;
        self
    }

    /// Sets verification status.
    pub fn verification_status(
        mut self,
        status: VerificationStatus,
    ) -> Self {
        self.verification_status = status;
        self
    }

    /// Sets degradation status.
    pub fn degradation(
        mut self,
        degradation: DegradationStatus,
    ) -> Self {
        self.degradation = degradation;
        self
    }

    /// Sets activity summary.
    pub fn activity(
        mut self,
        activity: ResilienceActivitySummary,
    ) -> Self {
        self.activity = activity;
        self
    }

    /// Sets resource impact.
    pub fn resources(
        mut self,
        resources: ResourceImpact,
    ) -> Self {
        self.resources = resources;
        self
    }

    /// Sets an execution artifact.
    pub fn execution(
        mut self,
        artifact: ExecutionArtifact<E>,
    ) -> Self {
        self.execution = Some(artifact);
        self
    }

    /// Sets a verification artifact.
    pub fn verification(
        mut self,
        artifact: VerificationArtifact<V>,
    ) -> Self {
        self.verification = Some(artifact);
        self
    }

    /// Sets a provenance artifact.
    pub fn provenance(
        mut self,
        artifact: ProvenanceArtifact<P>,
    ) -> Self {
        self.provenance = Some(artifact);
        self
    }

    /// Builds and validates the response.
    pub fn build(
        self,
    ) -> Result<ResilienceResponse<E, V, P>, ResponseValidationError> {
        ResilienceResponse::new(
            self.request_id,
            self.cycle_id,
            self.decision,
            self.execution_status,
            self.verification_status,
            self.degradation,
            self.activity,
            self.resources,
            self.execution,
            self.verification,
            self.provenance,
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn request_id() -> ResilienceRequestId {
        ResilienceRequestId::new("response-test")
            .expect("test request ID must be valid")
    }

    fn cycle_id() -> ResilienceCycleId {
        ResilienceCycleId::new(1)
    }

    fn execution_artifact() -> ExecutionArtifact<&'static str> {
        ExecutionArtifact::new(
            ArtifactId::new("execution-1")
                .expect("artifact ID must be valid"),
            Arc::new("execution"),
        )
    }

    fn verification_artifact() -> VerificationArtifact<&'static str> {
        VerificationArtifact::new(
            ArtifactId::new("verification-1")
                .expect("artifact ID must be valid"),
            Arc::new("verified"),
        )
    }

    fn provenance_artifact() -> ProvenanceArtifact<&'static str> {
        ProvenanceArtifact::new(
            ArtifactId::new("provenance-1")
                .expect("artifact ID must be valid"),
            Arc::new("provenance"),
        )
    }

    #[test]
    fn schema_metadata_is_stable() {
        let metadata = ResilienceResponseMetadata::current();

        assert_eq!(
            metadata.schema_id(),
            RESILIENCE_RESPONSE_SCHEMA_ID
        );
        assert_eq!(
            metadata.schema_version(),
            RESILIENCE_RESPONSE_SCHEMA_VERSION
        );
    }

    #[test]
    fn execution_status_is_independent_from_acceptance() {
        assert!(ExecutionStatus::Completed.is_completed());
        assert!(!ExecutionStatus::Failed.is_completed());
    }

    #[test]
    fn verification_status_distinguishes_full_and_degraded_verification() {
        assert!(VerificationStatus::Verified.is_verified());
        assert!(
            VerificationStatus::VerifiedDegraded
                .is_verified_degraded()
        );
        assert!(
            !VerificationStatus::VerifiedDegraded.is_verified()
        );
    }

    #[test]
    fn accepted_response_requires_verification() {
        let result = ResilienceResponse::<(), (), ()>::new(
            request_id(),
            cycle_id(),
            ResilienceDecision::Accept,
            ExecutionStatus::Completed,
            VerificationStatus::NotPerformed,
            DegradationStatus::None,
            ResilienceActivitySummary::new(),
            ResourceImpact::empty(),
            None,
            None,
            None,
        );

        assert_eq!(
            result,
            Err(ResponseValidationError::AcceptedWithoutVerification)
        );
    }

    #[test]
    fn accepted_response_requires_completed_execution() {
        let result = ResilienceResponse::<(), (), ()>::new(
            request_id(),
            cycle_id(),
            ResilienceDecision::Accept,
            ExecutionStatus::Failed,
            VerificationStatus::Verified,
            DegradationStatus::None,
            ResilienceActivitySummary::new(),
            ResourceImpact::empty(),
            None,
            Some(VerificationArtifact::new(
                ArtifactId::new("verification")
                    .expect("valid artifact"),
                Arc::new(()),
            )),
            None,
        );

        assert_eq!(
            result,
            Err(
                ResponseValidationError::AcceptedWithoutCompletedExecution
            )
        );
    }

    #[test]
    fn verified_response_requires_verification_artifact() {
        let result = ResilienceResponse::<(), (), ()>::new(
            request_id(),
            cycle_id(),
            ResilienceDecision::Accept,
            ExecutionStatus::Completed,
            VerificationStatus::Verified,
            DegradationStatus::None,
            ResilienceActivitySummary::new(),
            ResourceImpact::empty(),
            Some(execution_artifact()),
            None,
            None,
        );

        assert_eq!(
            result,
            Err(
                ResponseValidationError::VerifiedWithoutVerificationArtifact
            )
        );
    }

    #[test]
    fn accepted_response_can_be_constructed() {
        let response = ResilienceResponse::new(
            request_id(),
            cycle_id(),
            ResilienceDecision::Accept,
            ExecutionStatus::Completed,
            VerificationStatus::Verified,
            DegradationStatus::None,
            ResilienceActivitySummary::new(),
            ResourceImpact::empty(),
            Some(execution_artifact()),
            Some(verification_artifact()),
            Some(provenance_artifact()),
        )
        .expect("valid accepted response");

        assert!(response.is_accepted());
        assert!(response.is_verified());
        assert!(response.has_execution_artifact());
        assert!(response.has_verification_artifact());
        assert!(response.has_provenance_artifact());
    }

    #[test]
    fn degraded_acceptance_requires_verified_degradation() {
        let result = ResilienceResponse::<(), (), ()>::new(
            request_id(),
            cycle_id(),
            ResilienceDecision::DegradedAccept,
            ExecutionStatus::Completed,
            VerificationStatus::Verified,
            DegradationStatus::Degraded,
            ResilienceActivitySummary::new(),
            ResourceImpact::empty(),
            Some(execution_artifact()),
            Some(verification_artifact()),
            None,
        );

        assert_eq!(
            result,
            Err(
                ResponseValidationError::
                    DegradedAcceptedWithoutVerifiedDegradation
            )
        );
    }

    #[test]
    fn degraded_acceptance_requires_degradation() {
        let result = ResilienceResponse::<(), (), ()>::new(
            request_id(),
            cycle_id(),
            ResilienceDecision::DegradedAccept,
            ExecutionStatus::Completed,
            VerificationStatus::VerifiedDegraded,
            DegradationStatus::None,
            ResilienceActivitySummary::new(),
            ResourceImpact::empty(),
            Some(execution_artifact()),
            Some(verification_artifact()),
            None,
        );

        assert_eq!(
            result,
            Err(
                ResponseValidationError::
                    DegradedAcceptanceWithoutDegradation
            )
        );
    }

    #[test]
    fn degraded_acceptance_can_be_constructed() {
        let response = ResilienceResponse::new(
            request_id(),
            cycle_id(),
            ResilienceDecision::DegradedAccept,
            ExecutionStatus::Completed,
            VerificationStatus::VerifiedDegraded,
            DegradationStatus::Degraded,
            ResilienceActivitySummary::new(),
            ResourceImpact::empty(),
            Some(execution_artifact()),
            Some(verification_artifact()),
            Some(provenance_artifact()),
        )
        .expect("valid degraded response");

        assert!(response.is_accepted());
        assert!(response.is_verified_degraded());
        assert!(response.degradation().is_degraded());
    }

    #[test]
    fn repeat_does_not_require_completed_execution() {
        let response = ResilienceResponse::<(), (), ()>::new(
            request_id(),
            cycle_id(),
            ResilienceDecision::Repeat,
            ExecutionStatus::Failed,
            VerificationStatus::Failed,
            DegradationStatus::Severe,
            ResilienceActivitySummary::new(),
            ResourceImpact::empty(),
            None,
            None,
            None,
        )
        .expect("repeat response is valid");

        assert!(response.requires_repeat());
        assert!(!response.is_accepted());
    }

    #[test]
    fn escalation_is_not_acceptance() {
        let response = ResilienceResponse::<(), (), ()>::new(
            request_id(),
            cycle_id(),
            ResilienceDecision::Escalate,
            ExecutionStatus::Failed,
            VerificationStatus::Inconclusive,
            DegradationStatus::Unknown,
            ResilienceActivitySummary::new(),
            ResourceImpact::empty(),
            None,
            None,
            None,
        )
        .expect("escalation response is valid");

        assert!(response.requires_escalation());
        assert!(!response.is_accepted());
    }

    #[test]
    fn resource_impact_uses_canonical_qubit_types() {
        let resources = ResourceImpact::empty();

        assert!(resources.is_empty());
        assert_eq!(resources.logical_qubit_count(), 0);
        assert_eq!(resources.physical_qubit_count(), 0);
    }

    #[test]
    fn artifact_ids_reject_empty_values() {
        let result = ArtifactId::new("   ");

        assert_eq!(result, Err(ArtifactIdError::Empty));
    }

    #[test]
    fn builder_produces_valid_response() {
        let response = ResilienceResponseBuilder::new(
            request_id(),
            cycle_id(),
        )
        .decision(ResilienceDecision::Accept)
        .execution_status(ExecutionStatus::Completed)
        .verification_status(VerificationStatus::Verified)
        .execution(execution_artifact())
        .verification(verification_artifact())
        .provenance(provenance_artifact())
        .build()
        .expect("builder should create a valid response");

        assert!(response.is_accepted());
        assert!(response.is_verified());
    }
}