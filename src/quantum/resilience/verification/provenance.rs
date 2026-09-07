//! Zamani Quantum Resilience — Verification Provenance
//!
//! Path:
//!     src/quantum/resilience/verification/provenance.rs
//!
//! # Purpose
//!
//! This module defines the production provenance contract for resilience
//! verification.
//!
//! Provenance answers:
//!
//! - Which execution is being verified?
//! - Which canonical program/semantic identity was expected?
//! - Which candidate semantic identity was observed?
//! - Which compiler/IR/runtime/backend identities participated?
//! - Which logical and physical quantum resources were involved?
//! - Which adaptations occurred?
//! - Which recovery operations occurred?
//! - Which mitigation/QEC actions were applied?
//! - Which verification components produced evidence?
//! - Which integrity identities were supplied?
//! - Which provenance assertions remain unverifiable?
//!
//! Provenance is an evidence and audit boundary.
//!
//! It is NOT:
//!
//! - semantic verification;
//! - QEC;
//! - error mitigation;
//! - hardware validation;
//! - cryptographic hashing;
//! - authorization;
//! - recovery execution;
//! - result acceptance.
//!
//! Those responsibilities belong to their authoritative subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                    Zamani quantum::ir
//!                           │
//!                           ▼
//!                    logical program
//!                           │
//!                           ▼
//!                    resilience execution
//!                           │
//!       ┌───────────────────┼────────────────────┐
//!       │                   │                    │
//!       ▼                   ▼                    ▼
//!    compiler            routing              QEC
//!       │                   │                    │
//!       └───────────────────┼────────────────────┘
//!                           ▼
//!                      adaptation
//!                           │
//!                           ▼
//!                       recovery
//!                           │
//!                           ▼
//!                       execution
//!                           │
//!                           ▼
//!                  ProvenanceSnapshot
//!                           │
//!                           ▼
//!                   ProvenanceVerifier
//!                           │
//!                           ▼
//!                 VerificationEvidence
//!                           │
//!                           ▼
//!                  ResilienceVerifier
//! ```
//!
//! # Critical semantic rule
//!
//! Provenance is not proof.
//!
//! A provenance record saying:
//!
//!     semantic verification = requested
//!
//! does not prove semantic equivalence.
//!
//! A provenance record saying:
//!
//!     recovery = completed
//!
//! does not prove that the recovered result is correct.
//!
//! A digest saying:
//!
//!     SHA-256 = <digest>
//!
//! does not prove that the producer of the digest was trustworthy.
//!
//! Provenance records claims, identities, relationships and evidence supplied
//! by authoritative components. Those components remain responsible for the
//! validity of their claims.
//!
//! # Write once, scale everywhere
//!
//! This module has no fixed:
//!
//! - qubit count;
//! - physical-qubit count;
//! - operation count;
//! - backend count;
//! - machine count;
//! - recovery count;
//! - adaptation count;
//! - verification-stage count;
//! - provenance-event count.
//!
//! Collections are dynamically sized and represented using owned slices.
//!
//! Memory consumption is therefore proportional to the provenance retained by
//! the caller rather than to an artificial machine-size constant.
//!
//! Large executions can use:
//!
//! - summary provenance;
//! - bounded provenance;
//! - externally persisted provenance;
//! - streaming provenance collection.
//!
//! This file does not impose a global retention policy.
//!
//! # Canonical quantum identity
//!
//! Logical and physical quantum resources use the canonical IR identities:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! No resilience-local qubit identifier is introduced.
//!
//! A logical qubit MUST NOT be silently interpreted as a physical qubit.
//!
//! # Determinism
//!
//! The data model is deterministic for deterministic input.
//!
//! This module:
//!
//! - does not read the clock;
//! - does not generate UUIDs;
//! - does not access environment variables;
//! - does not access global mutable state;
//! - does not access the filesystem;
//! - does not access the network;
//! - does not hash data itself;
//! - does not use random numbers.
//!
//! Timestamps, random seeds and content digests are accepted only as explicit
//! supplied evidence.
//!
//! # Integrity model
//!
//! Provenance supports externally supplied content identities through
//! [`ContentIdentity`].
//!
//! The actual hashing implementation belongs to the repository's canonical
//! hashing/serialization subsystem.
//!
//! This module therefore never:
//!
//! - invents a hash;
//! - computes a competing hash;
//! - treats an arbitrary string as cryptographically trustworthy;
//! - silently normalizes an invalid digest.
//!
//! # Verification integration
//!
//! The existing resilience verifier defines:
//!
//!     ProvenanceVerifier
//!
//! with:
//!
//!     verify_provenance(&VerificationRequest)
//!
//! This file implements that contract through [`DefaultProvenanceVerifier`].
//!
//! Because the current `VerificationRequest` intentionally contains execution
//! identity and semantic identity but does not embed a mutable provenance
//! object, the verifier owns an immutable [`ProvenanceSnapshot`] supplied at
//! construction time.
//!
//! This keeps verification deterministic and avoids:
//!
//! - global provenance registries;
//! - hidden filesystem reads;
//! - hidden telemetry reads;
//! - execution-ID based global lookups;
//! - mutable cross-execution state.
//!
//! # Security
//!
//! Provenance may contain sensitive operational metadata.
//!
//! This module therefore does not expose credentials, tokens, private keys,
//! authorization headers or backend secrets as dedicated fields.
//!
//! Free-form metadata is deliberately represented as explicit safe metadata
//! entries and should only receive non-secret information from callers.
//!
//! # Compatibility
//!
//! The provenance schema is explicitly versioned.
//!
//! New fields should be added in backward-compatible fashion where possible.
//! Existing semantic meanings must not silently change.
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
#![deny(missing_debug_implementations)]
#![deny(rust_2018_idioms)]

use std::fmt;
use std::sync::Arc;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::resilience::errors::{
    ResilienceError,
    ResilienceErrorCode,
    ResilienceResult,
};

use super::verifier::{
    ProvenanceVerifier,
    VerificationConfidence,
    VerificationEvidence,
    VerificationRequest,
    VerificationStage,
    VerificationState,
    VerificationViolation,
    VerificationViolationKind,
};

// =============================================================================
// Schema identity
// =============================================================================

/// Stable schema identifier for resilience verification provenance.
pub const RESILIENCE_VERIFICATION_PROVENANCE_SCHEMA_ID: &str =
    "zamani.quantum.resilience.verification.provenance";

/// Current provenance semantic schema version.
pub const RESILIENCE_VERIFICATION_PROVENANCE_SCHEMA_VERSION: u32 = 1;

// =============================================================================
// Stable violation codes
// =============================================================================

const CODE_EMPTY_EXECUTION_ID: &str = "QR-PROV-001";
const CODE_EXECUTION_ID_MISMATCH: &str = "QR-PROV-002";
const CODE_EMPTY_EXPECTED_FINGERPRINT: &str = "QR-PROV-003";
const CODE_EXPECTED_FINGERPRINT_MISMATCH: &str = "QR-PROV-004";
const CODE_CANDIDATE_FINGERPRINT_MISMATCH: &str = "QR-PROV-005";
const CODE_MISSING_PROGRAM_IDENTITY: &str = "QR-PROV-006";
const CODE_MISSING_EXECUTION_IDENTITY: &str = "QR-PROV-007";
const CODE_MISSING_INTEGRITY_IDENTITY: &str = "QR-PROV-008";
const CODE_INVALID_RESOURCE_IDENTITY: &str = "QR-PROV-009";
const CODE_DUPLICATE_LOGICAL_QUBIT: &str = "QR-PROV-010";
const CODE_DUPLICATE_PHYSICAL_QUBIT: &str = "QR-PROV-011";
const CODE_INVALID_EVENT_SEQUENCE: &str = "QR-PROV-012";
const CODE_UNVERIFIED_CLAIM: &str = "QR-PROV-013";
const CODE_INVALID_METADATA: &str = "QR-PROV-014";
const CODE_SCHEMA_MISMATCH: &str = "QR-PROV-015";

// =============================================================================
// Provenance completeness
// =============================================================================

/// Describes how completely provenance was collected.
///
/// This is deliberately separate from verification confidence.
///
/// Completeness answers:
///
/// > "How much provenance was retained?"
///
/// Confidence answers:
///
/// > "How strongly does the evidence support a claim?"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProvenanceCompleteness {
    /// No provenance was collected.
    None,

    /// Only essential identity information was retained.
    Minimal,

    /// Required execution history was retained, but optional detail may be
    /// absent.
    Partial,

    /// Required and optional provenance supplied by the producer was retained.
    Complete,
}

impl ProvenanceCompleteness {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Minimal => "minimal",
            Self::Partial => "partial",
            Self::Complete => "complete",
        }
    }

    #[must_use]
    pub const fn is_sufficient_for_strict_verification(self) -> bool {
        matches!(self, Self::Complete)
    }
}

impl fmt::Display for ProvenanceCompleteness {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Provenance source
// =============================================================================

/// Identifies the subsystem that supplied a provenance record.
///
/// This is an opaque provider-neutral identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProvenanceSource(Arc<str>);

impl ProvenanceSource {
    /// Creates a provenance source.
    pub fn new(value: impl Into<Arc<str>>) -> ResilienceResult<Self> {
        let value = value.into();

        validate_non_empty("provenance source", &value)?;

        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for ProvenanceSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Content identity
// =============================================================================

/// An externally supplied content identity.
///
/// This type intentionally does not implement hashing.
///
/// The actual hashing algorithm belongs to the canonical serialization/hash
/// subsystem.
///
/// `algorithm` and `digest` are recorded exactly as supplied after basic
/// structural validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContentIdentity {
    algorithm: Arc<str>,
    digest: Arc<str>,
}

impl ContentIdentity {
    /// Creates a content identity.
    ///
    /// The digest must be non-empty.
    ///
    /// This method does not attempt to determine whether the supplied digest
    /// is cryptographically valid for the named algorithm. That responsibility
    /// belongs to the authoritative hashing implementation.
    pub fn new(
        algorithm: impl Into<Arc<str>>,
        digest: impl Into<Arc<str>>,
    ) -> ResilienceResult<Self> {
        let algorithm = algorithm.into();
        let digest = digest.into();

        validate_non_empty("content identity algorithm", &algorithm)?;
        validate_non_empty("content identity digest", &digest)?;

        Ok(Self { algorithm, digest })
    }

    #[must_use]
    pub fn algorithm(&self) -> &str {
        self.algorithm.as_ref()
    }

    #[must_use]
    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

// =============================================================================
// Resource scope
// =============================================================================

/// Canonical logical/physical resource scope associated with provenance.
///
/// Logical and physical qubits remain separate types.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProvenanceResourceScope {
    logical_qubits: Arc<[QubitId]>,
    physical_qubits: Arc<[PhysicalQubitId]>,
}

impl ProvenanceResourceScope {
    /// Creates a resource scope.
    ///
    /// The two collections are sorted and deduplicated so deterministic
    /// provenance comparison does not depend on caller iteration order.
    #[must_use]
    pub fn new<L, P>(
        logical_qubits: L,
        physical_qubits: P,
    ) -> Self
    where
        L: IntoIterator<Item = QubitId>,
        P: IntoIterator<Item = PhysicalQubitId>,
    {
        let mut logical: Vec<QubitId> = logical_qubits.into_iter().collect();
        let mut physical: Vec<PhysicalQubitId> =
            physical_qubits.into_iter().collect();

        logical.sort_unstable();
        logical.dedup();

        physical.sort_unstable();
        physical.dedup();

        Self {
            logical_qubits: logical.into(),
            physical_qubits: physical.into(),
        }
    }

    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        self.logical_qubits.as_ref()
    }

    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        self.physical_qubits.as_ref()
    }

    #[must_use]
    pub fn logical_count(&self) -> usize {
        self.logical_qubits.len()
    }

    #[must_use]
    pub fn physical_count(&self) -> usize {
        self.physical_qubits.len()
    }
}

// =============================================================================
// Provenance event kind
// =============================================================================

/// Classification of a provenance event.
///
/// The enum is intentionally generic so future quantum technologies do not
/// require modification of the provenance core merely because a new event
/// type appears.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProvenanceEventKind {
    /// Program identity established.
    ProgramIdentity,

    /// IR identity established.
    IrIdentity,

    /// Compilation event.
    Compilation,

    /// Routing event.
    Routing,

    /// Scheduling event.
    Scheduling,

    /// Optimization event.
    Optimization,

    /// QEC event.
    Qec,

    /// Mitigation event.
    Mitigation,

    /// Adaptation event.
    Adaptation,

    /// Recovery event.
    Recovery,

    /// Backend/device selection event.
    BackendSelection,

    /// Execution event.
    Execution,

    /// Verification event.
    Verification,

    /// Resource state event.
    ResourceState,

    /// Generic execution metadata.
    Metadata,
}

impl ProvenanceEventKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProgramIdentity => "program_identity",
            Self::IrIdentity => "ir_identity",
            Self::Compilation => "compilation",
            Self::Routing => "routing",
            Self::Scheduling => "scheduling",
            Self::Optimization => "optimization",
            Self::Qec => "qec",
            Self::Mitigation => "mitigation",
            Self::Adaptation => "adaptation",
            Self::Recovery => "recovery",
            Self::BackendSelection => "backend_selection",
            Self::Execution => "execution",
            Self::Verification => "verification",
            Self::ResourceState => "resource_state",
            Self::Metadata => "metadata",
        }
    }
}

impl fmt::Display for ProvenanceEventKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Provenance event
// =============================================================================

/// One immutable provenance event.
///
/// Events are append-oriented and carry an explicit sequence number.
///
/// The sequence number is assigned by the provenance builder and is therefore
/// deterministic when the builder is fed deterministic events in deterministic
/// order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceEvent {
    sequence: u64,
    kind: ProvenanceEventKind,
    source: ProvenanceSource,
    operation: Arc<str>,
    detail: Option<Arc<str>>,
    content_identity: Option<ContentIdentity>,
}

impl ProvenanceEvent {
    /// Creates a provenance event without assigning a sequence number.
    pub fn new(
        kind: ProvenanceEventKind,
        source: ProvenanceSource,
        operation: impl Into<Arc<str>>,
    ) -> ResilienceResult<Self> {
        let operation = operation.into();

        validate_non_empty("provenance event operation", &operation)?;

        Ok(Self {
            sequence: 0,
            kind,
            source,
            operation,
            detail: None,
            content_identity: None,
        })
    }

    /// Adds non-secret diagnostic detail.
    #[must_use]
    pub fn with_detail(mut self, detail: impl Into<Arc<str>>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Attaches an externally supplied content identity.
    #[must_use]
    pub fn with_content_identity(
        mut self,
        identity: ContentIdentity,
    ) -> Self {
        self.content_identity = Some(identity);
        self
    }

    fn with_sequence(mut self, sequence: u64) -> Self {
        self.sequence = sequence;
        self
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn kind(&self) -> ProvenanceEventKind {
        self.kind
    }

    #[must_use]
    pub fn source(&self) -> &ProvenanceSource {
        &self.source
    }

    #[must_use]
    pub fn operation(&self) -> &str {
        self.operation.as_ref()
    }

    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    #[must_use]
    pub fn content_identity(&self) -> Option<&ContentIdentity> {
        self.content_identity.as_ref()
    }
}

// =============================================================================
// Verification provenance record
// =============================================================================

/// Records what a verification component reported.
///
/// This is observational evidence, not a second verification engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationProvenanceRecord {
    sequence: u64,
    verifier_id: Arc<str>,
    stage: VerificationStage,
    state: VerificationState,
    confidence: VerificationConfidence,
    source: ProvenanceSource,
}

impl VerificationProvenanceRecord {
    pub fn new(
        verifier_id: impl Into<Arc<str>>,
        stage: VerificationStage,
        state: VerificationState,
        confidence: VerificationConfidence,
        source: ProvenanceSource,
    ) -> ResilienceResult<Self> {
        let verifier_id = verifier_id.into();

        validate_non_empty(
            "verification provenance verifier id",
            &verifier_id,
        )?;

        Ok(Self {
            sequence: 0,
            verifier_id,
            stage,
            state,
            confidence,
            source,
        })
    }

    fn with_sequence(mut self, sequence: u64) -> Self {
        self.sequence = sequence;
        self
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub fn verifier_id(&self) -> &str {
        self.verifier_id.as_ref()
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
    pub fn source(&self) -> &ProvenanceSource {
        &self.source
    }
}

// =============================================================================
// Adaptation/recovery record
// =============================================================================

/// Records an adaptation or recovery operation.
///
/// The actual operation is owned by the adaptation/recovery subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryProvenanceRecord {
    sequence: u64,
    action: Arc<str>,
    source: ProvenanceSource,
    completed: bool,
    result_identity: Option<ContentIdentity>,
}

impl RecoveryProvenanceRecord {
    pub fn new(
        action: impl Into<Arc<str>>,
        source: ProvenanceSource,
        completed: bool,
    ) -> ResilienceResult<Self> {
        let action = action.into();

        validate_non_empty(
            "recovery provenance action",
            &action,
        )?;

        Ok(Self {
            sequence: 0,
            action,
            source,
            completed,
            result_identity: None,
        })
    }

    #[must_use]
    pub fn with_result_identity(
        mut self,
        identity: ContentIdentity,
    ) -> Self {
        self.result_identity = Some(identity);
        self
    }

    fn with_sequence(mut self, sequence: u64) -> Self {
        self.sequence = sequence;
        self
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub fn action(&self) -> &str {
        self.action.as_ref()
    }

    #[must_use]
    pub fn source(&self) -> &ProvenanceSource {
        &self.source
    }

    #[must_use]
    pub const fn completed(&self) -> bool {
        self.completed
    }

    #[must_use]
    pub fn result_identity(&self) -> Option<&ContentIdentity> {
        self.result_identity.as_ref()
    }
}

// =============================================================================
// Provenance snapshot
// =============================================================================

/// Immutable provenance snapshot supplied to the provenance verifier.
///
/// The snapshot is deliberately self-contained.
///
/// It does not hold references to live hardware, runtime objects, mutable
/// compiler state, telemetry collectors or backend clients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceSnapshot {
    schema_id: &'static str,
    schema_version: u32,

    execution_id: Arc<str>,

    program_identity: Option<ContentIdentity>,
    expected_semantic_identity: Option<ContentIdentity>,
    candidate_semantic_identity: Option<ContentIdentity>,

    ir_identity: Option<ContentIdentity>,
    compiled_identity: Option<ContentIdentity>,
    scheduled_identity: Option<ContentIdentity>,
    routed_identity: Option<ContentIdentity>,
    result_identity: Option<ContentIdentity>,

    resources: ProvenanceResourceScope,

    completeness: ProvenanceCompleteness,

    events: Arc<[ProvenanceEvent]>,
    verification_records: Arc<[VerificationProvenanceRecord]>,
    recovery_records: Arc<[RecoveryProvenanceRecord]>,

    unverified_claims: Arc<[Arc<str>]>,
}

impl ProvenanceSnapshot {
    /// Creates a new immutable snapshot builder.
    pub fn builder(
        execution_id: impl Into<Arc<str>>,
    ) -> ResilienceResult<ProvenanceSnapshotBuilder> {
        let execution_id = execution_id.into();

        validate_non_empty(
            "provenance execution id",
            &execution_id,
        )?;

        Ok(ProvenanceSnapshotBuilder {
            execution_id,
            program_identity: None,
            expected_semantic_identity: None,
            candidate_semantic_identity: None,
            ir_identity: None,
            compiled_identity: None,
            scheduled_identity: None,
            routed_identity: None,
            result_identity: None,
            resources: ProvenanceResourceScope::default(),
            completeness: ProvenanceCompleteness::None,
            events: Vec::new(),
            verification_records: Vec::new(),
            recovery_records: Vec::new(),
            unverified_claims: Vec::new(),
            next_sequence: 0,
        })
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
    pub fn execution_id(&self) -> &str {
        self.execution_id.as_ref()
    }

    #[must_use]
    pub fn program_identity(&self) -> Option<&ContentIdentity> {
        self.program_identity.as_ref()
    }

    #[must_use]
    pub fn expected_semantic_identity(&self) -> Option<&ContentIdentity> {
        self.expected_semantic_identity.as_ref()
    }

    #[must_use]
    pub fn candidate_semantic_identity(
        &self,
    ) -> Option<&ContentIdentity> {
        self.candidate_semantic_identity.as_ref()
    }

    #[must_use]
    pub fn ir_identity(&self) -> Option<&ContentIdentity> {
        self.ir_identity.as_ref()
    }

    #[must_use]
    pub fn compiled_identity(&self) -> Option<&ContentIdentity> {
        self.compiled_identity.as_ref()
    }

    #[must_use]
    pub fn scheduled_identity(&self) -> Option<&ContentIdentity> {
        self.scheduled_identity.as_ref()
    }

    #[must_use]
    pub fn routed_identity(&self) -> Option<&ContentIdentity> {
        self.routed_identity.as_ref()
    }

    #[must_use]
    pub fn result_identity(&self) -> Option<&ContentIdentity> {
        self.result_identity.as_ref()
    }

    #[must_use]
    pub fn resources(&self) -> &ProvenanceResourceScope {
        &self.resources
    }

    #[must_use]
    pub const fn completeness(&self) -> ProvenanceCompleteness {
        self.completeness
    }

    #[must_use]
    pub fn events(&self) -> &[ProvenanceEvent] {
        self.events.as_ref()
    }

    #[must_use]
    pub fn verification_records(
        &self,
    ) -> &[VerificationProvenanceRecord] {
        self.verification_records.as_ref()
    }

    #[must_use]
    pub fn recovery_records(
        &self,
    ) -> &[RecoveryProvenanceRecord] {
        self.recovery_records.as_ref()
    }

    #[must_use]
    pub fn unverified_claims(&self) -> &[Arc<str>] {
        self.unverified_claims.as_ref()
    }

    #[must_use]
    pub fn has_unverified_claims(&self) -> bool {
        !self.unverified_claims.is_empty()
    }

    /// Performs structural validation of the snapshot itself.
    ///
    /// This does not prove semantic correctness.
    pub fn validate(&self) -> ResilienceResult<()> {
        validate_non_empty(
            "provenance execution id",
            &self.execution_id,
        )?;

        if self.schema_id != RESILIENCE_VERIFICATION_PROVENANCE_SCHEMA_ID
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::CompatibilityFailure,
                "unsupported provenance schema identity",
            ));
        }

        if self.schema_version
            != RESILIENCE_VERIFICATION_PROVENANCE_SCHEMA_VERSION
        {
            return Err(ResilienceError::new(
                ResilienceErrorCode::UnsupportedSchemaVersion,
                "unsupported provenance schema version",
            ));
        }

        validate_event_sequences(&self.events)?;
        validate_verification_sequences(&self.verification_records)?;
        validate_recovery_sequences(&self.recovery_records)?;

        Ok(())
    }
}

// =============================================================================
// Provenance snapshot builder
// =============================================================================

/// Builder for [`ProvenanceSnapshot`].
///
/// The builder is mutable only during construction. Once `build()` returns,
/// the resulting snapshot is immutable.
#[derive(Debug)]
pub struct ProvenanceSnapshotBuilder {
    execution_id: Arc<str>,

    program_identity: Option<ContentIdentity>,
    expected_semantic_identity: Option<ContentIdentity>,
    candidate_semantic_identity: Option<ContentIdentity>,

    ir_identity: Option<ContentIdentity>,
    compiled_identity: Option<ContentIdentity>,
    scheduled_identity: Option<ContentIdentity>,
    routed_identity: Option<ContentIdentity>,
    result_identity: Option<ContentIdentity>,

    resources: ProvenanceResourceScope,

    completeness: ProvenanceCompleteness,

    events: Vec<ProvenanceEvent>,
    verification_records: Vec<VerificationProvenanceRecord>,
    recovery_records: Vec<RecoveryProvenanceRecord>,

    unverified_claims: Vec<Arc<str>>,

    next_sequence: u64,
}

impl ProvenanceSnapshotBuilder {
    #[must_use]
    pub fn with_program_identity(
        mut self,
        identity: ContentIdentity,
    ) -> Self {
        self.program_identity = Some(identity);
        self
    }

    #[must_use]
    pub fn with_expected_semantic_identity(
        mut self,
        identity: ContentIdentity,
    ) -> Self {
        self.expected_semantic_identity = Some(identity);
        self
    }

    #[must_use]
    pub fn with_candidate_semantic_identity(
        mut self,
        identity: ContentIdentity,
    ) -> Self {
        self.candidate_semantic_identity = Some(identity);
        self
    }

    #[must_use]
    pub fn with_ir_identity(
        mut self,
        identity: ContentIdentity,
    ) -> Self {
        self.ir_identity = Some(identity);
        self
    }

    #[must_use]
    pub fn with_compiled_identity(
        mut self,
        identity: ContentIdentity,
    ) -> Self {
        self.compiled_identity = Some(identity);
        self
    }

    #[must_use]
    pub fn with_scheduled_identity(
        mut self,
        identity: ContentIdentity,
    ) -> Self {
        self.scheduled_identity = Some(identity);
        self
    }

    #[must_use]
    pub fn with_routed_identity(
        mut self,
        identity: ContentIdentity,
    ) -> Self {
        self.routed_identity = Some(identity);
        self
    }

    #[must_use]
    pub fn with_result_identity(
        mut self,
        identity: ContentIdentity,
    ) -> Self {
        self.result_identity = Some(identity);
        self
    }

    #[must_use]
    pub fn with_resources(
        mut self,
        resources: ProvenanceResourceScope,
    ) -> Self {
        self.resources = resources;
        self
    }

    #[must_use]
    pub fn with_completeness(
        mut self,
        completeness: ProvenanceCompleteness,
    ) -> Self {
        self.completeness = completeness;
        self
    }

    /// Adds one provenance event.
    ///
    /// Sequence numbers are allocated here rather than by event producers.
    pub fn push_event(
        &mut self,
        event: ProvenanceEvent,
    ) -> ResilienceResult<()> {
        let sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or_else(|| {
                ResilienceError::new(
                    ResilienceErrorCode::ArithmeticOverflow,
                    "provenance event sequence overflow",
                )
            })?;

        self.next_sequence = sequence;

        self.events.push(event.with_sequence(sequence));

        Ok(())
    }

    /// Adds a verification record.
    pub fn push_verification_record(
        &mut self,
        record: VerificationProvenanceRecord,
    ) -> ResilienceResult<()> {
        let sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or_else(|| {
                ResilienceError::new(
                    ResilienceErrorCode::ArithmeticOverflow,
                    "provenance sequence overflow",
                )
            })?;

        self.next_sequence = sequence;

        self.verification_records
            .push(record.with_sequence(sequence));

        Ok(())
    }

    /// Adds a recovery/adaptation record.
    pub fn push_recovery_record(
        &mut self,
        record: RecoveryProvenanceRecord,
    ) -> ResilienceResult<()> {
        let sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or_else(|| {
                ResilienceError::new(
                    ResilienceErrorCode::ArithmeticOverflow,
                    "provenance sequence overflow",
                )
            })?;

        self.next_sequence = sequence;

        self.recovery_records
            .push(record.with_sequence(sequence));

        Ok(())
    }

    /// Records a claim that has not independently been verified.
    pub fn push_unverified_claim(
        &mut self,
        claim: impl Into<Arc<str>>,
    ) -> ResilienceResult<()> {
        let claim = claim.into();

        validate_non_empty(
            "unverified provenance claim",
            &claim,
        )?;

        self.unverified_claims.push(claim);

        Ok(())
    }

    /// Builds an immutable snapshot.
    pub fn build(self) -> ResilienceResult<ProvenanceSnapshot> {
        let snapshot = ProvenanceSnapshot {
            schema_id: RESILIENCE_VERIFICATION_PROVENANCE_SCHEMA_ID,
            schema_version:
                RESILIENCE_VERIFICATION_PROVENANCE_SCHEMA_VERSION,

            execution_id: self.execution_id,

            program_identity: self.program_identity,
            expected_semantic_identity: self.expected_semantic_identity,
            candidate_semantic_identity: self.candidate_semantic_identity,

            ir_identity: self.ir_identity,
            compiled_identity: self.compiled_identity,
            scheduled_identity: self.scheduled_identity,
            routed_identity: self.routed_identity,
            result_identity: self.result_identity,

            resources: self.resources,

            completeness: self.completeness,

            events: self.events.into(),
            verification_records: self.verification_records.into(),
            recovery_records: self.recovery_records.into(),

            unverified_claims: self.unverified_claims.into(),
        };

        snapshot.validate()?;

        Ok(snapshot)
    }
}

// =============================================================================
// Provenance verifier
// =============================================================================

/// Production provenance verifier.
///
/// The verifier is immutable after construction and contains exactly one
/// provenance snapshot.
///
/// This avoids mutable global provenance state and makes verification
/// deterministic and replayable.
#[derive(Debug, Clone)]
pub struct DefaultProvenanceVerifier {
    snapshot: Arc<ProvenanceSnapshot>,
    minimum_completeness: ProvenanceCompleteness,
}

impl DefaultProvenanceVerifier {
    /// Creates a strict provenance verifier.
    ///
    /// Strict mode requires complete provenance.
    #[must_use]
    pub fn new(snapshot: ProvenanceSnapshot) -> Self {
        Self {
            snapshot: Arc::new(snapshot),
            minimum_completeness: ProvenanceCompleteness::Complete,
        }
    }

    /// Creates a verifier with an explicit completeness requirement.
    ///
    /// This is useful for development, simulation, partial execution and
    /// resource-constrained environments where policy deliberately permits
    /// reduced provenance.
    #[must_use]
    pub fn with_minimum_completeness(
        snapshot: ProvenanceSnapshot,
        minimum_completeness: ProvenanceCompleteness,
    ) -> Self {
        Self {
            snapshot: Arc::new(snapshot),
            minimum_completeness,
        }
    }

    #[must_use]
    pub fn snapshot(&self) -> &ProvenanceSnapshot {
        self.snapshot.as_ref()
    }

    #[must_use]
    pub const fn minimum_completeness(
        &self,
    ) -> ProvenanceCompleteness {
        self.minimum_completeness
    }

    fn validate_request_identity(
        &self,
        request: &VerificationRequest,
        violations: &mut Vec<VerificationViolation>,
    ) {
        let request_execution_id = request.execution_id().as_str();
        let provenance_execution_id = self.snapshot.execution_id();

        if request_execution_id.is_empty() {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_EMPTY_EXECUTION_ID,
                "verification request execution identity is empty",
            ));
        }

        if provenance_execution_id.is_empty() {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_MISSING_EXECUTION_IDENTITY,
                "provenance execution identity is missing",
            ));
        }

        if request_execution_id != provenance_execution_id {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_EXECUTION_ID_MISMATCH,
                "verification request and provenance refer to different executions",
            ));
        }
    }

    fn validate_semantic_identity(
        &self,
        request: &VerificationRequest,
        violations: &mut Vec<VerificationViolation>,
    ) {
        let expected = request.expected_semantic_fingerprint().as_str();

        if expected.is_empty() {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_EMPTY_EXPECTED_FINGERPRINT,
                "expected semantic fingerprint is empty",
            ));
            return;
        }

        let Some(provenance_expected) =
            self.snapshot.expected_semantic_identity()
        else {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_MISSING_PROGRAM_IDENTITY,
                "provenance does not contain the expected semantic identity",
            ));
            return;
        };

        /*
         * The provenance identity is an opaque externally supplied digest.
         *
         * The request's semantic fingerprint and provenance's semantic
         * identity are intentionally compared as strings because this module
         * does not own the canonical hashing algorithm.
         *
         * Producers must use the repository's canonical semantic fingerprint
         * representation consistently.
         */
        if provenance_expected.digest() != expected {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_EXPECTED_FINGERPRINT_MISMATCH,
                "provenance expected semantic identity does not match the verification request",
            ));
        }

        match (
            request.candidate_semantic_fingerprint(),
            self.snapshot.candidate_semantic_identity(),
        ) {
            (Some(candidate), Some(provenance_candidate)) => {
                if provenance_candidate.digest() != candidate.as_str() {
                    violations.push(VerificationViolation::new(
                        VerificationStage::Provenance,
                        VerificationViolationKind::ProvenanceInvalid,
                        CODE_CANDIDATE_FINGERPRINT_MISMATCH,
                        "provenance candidate semantic identity does not match the verification request",
                    ));
                }
            }

            (Some(_), None) => {
                violations.push(VerificationViolation::new(
                    VerificationStage::Provenance,
                    VerificationViolationKind::ProvenanceInvalid,
                    CODE_CANDIDATE_FINGERPRINT_MISMATCH,
                    "verification request contains a candidate semantic identity that is absent from provenance",
                ));
            }

            (None, Some(_)) => {
                /*
                 * A candidate may be present in provenance even when the
                 * request does not expose one. This is not automatically a
                 * violation because the request contract allows the candidate
                 * to be optional.
                 *
                 * The semantic verifier remains authoritative for whether
                 * equivalence has actually been established.
                 */
            }

            (None, None) => {}
        }
    }

    fn validate_resources(
        &self,
        request: &VerificationRequest,
        violations: &mut Vec<VerificationViolation>,
    ) {
        let provenance = self.snapshot.resources();
        let request_resources = request.resources();

        /*
         * Logical and physical domains are independent.
         *
         * We deliberately do NOT require:
         *
         *     logical_count == physical_count
         *
         * because encoded, distributed, simulator, ancilla-heavy and
         * dynamically mapped executions legitimately violate that equality.
         */

        if has_duplicates(provenance.logical_qubits()) {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ResourceIdentityMismatch,
                CODE_DUPLICATE_LOGICAL_QUBIT,
                "provenance contains duplicate logical qubit identities",
            ));
        }

        if has_duplicates(provenance.physical_qubits()) {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ResourceIdentityMismatch,
                CODE_DUPLICATE_PHYSICAL_QUBIT,
                "provenance contains duplicate physical qubit identities",
            ));
        }

        if has_duplicates(request_resources.logical_qubits()) {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ResourceIdentityMismatch,
                CODE_DUPLICATE_LOGICAL_QUBIT,
                "verification request contains duplicate logical qubit identities",
            ));
        }

        if has_duplicates(request_resources.physical_qubits()) {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ResourceIdentityMismatch,
                CODE_DUPLICATE_PHYSICAL_QUBIT,
                "verification request contains duplicate physical qubit identities",
            ));
        }

        if provenance.logical_qubits()
            != request_resources.logical_qubits()
        {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ResourceIdentityMismatch,
                CODE_INVALID_RESOURCE_IDENTITY,
                "provenance logical resource scope differs from the verification request",
            ));
        }

        if provenance.physical_qubits()
            != request_resources.physical_qubits()
        {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ResourceIdentityMismatch,
                CODE_INVALID_RESOURCE_IDENTITY,
                "provenance physical resource scope differs from the verification request",
            ));
        }
    }

    fn validate_integrity_identity(
        &self,
        violations: &mut Vec<VerificationViolation>,
    ) {
        /*
         * The result identity is required because provenance verification
         * must be able to associate the verified execution with an explicit
         * result artifact.
         *
         * The digest itself is not cryptographically validated here.
         */
        if self.snapshot.result_identity().is_none() {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::MissingEvidence,
                CODE_MISSING_INTEGRITY_IDENTITY,
                "provenance does not contain a result content identity",
            ));
        }
    }

    fn validate_sequences(
        &self,
        violations: &mut Vec<VerificationViolation>,
    ) {
        if !strictly_increasing(
            self.snapshot.events().iter().map(
                ProvenanceEvent::sequence,
            ),
        ) {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_INVALID_EVENT_SEQUENCE,
                "provenance event sequence is not strictly increasing",
            ));
        }

        if !strictly_increasing(
            self.snapshot
                .verification_records()
                .iter()
                .map(VerificationProvenanceRecord::sequence),
        ) {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_INVALID_EVENT_SEQUENCE,
                "verification provenance sequence is not strictly increasing",
            ));
        }

        if !strictly_increasing(
            self.snapshot
                .recovery_records()
                .iter()
                .map(RecoveryProvenanceRecord::sequence),
        ) {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_INVALID_EVENT_SEQUENCE,
                "recovery provenance sequence is not strictly increasing",
            ));
        }
    }

    fn validate_schema(
        &self,
        violations: &mut Vec<VerificationViolation>,
    ) {
        if self.snapshot.schema_id()
            != RESILIENCE_VERIFICATION_PROVENANCE_SCHEMA_ID
        {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_SCHEMA_MISMATCH,
                "provenance schema identity is incompatible with the resilience verification contract",
            ));
        }

        if self.snapshot.schema_version()
            != RESILIENCE_VERIFICATION_PROVENANCE_SCHEMA_VERSION
        {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_SCHEMA_MISMATCH,
                "provenance schema version is incompatible with the resilience verification contract",
            ));
        }
    }

    fn validate_claims(
        &self,
        request: &VerificationRequest,
        violations: &mut Vec<VerificationViolation>,
    ) {
        if !self.snapshot.has_unverified_claims() {
            return;
        }

        /*
         * An explicit claim that has not been independently verified is not
         * necessarily an error in a diagnostic/development environment.
         *
         * Strict production verification, however, must not silently treat
         * unverifiable provenance as complete evidence.
         */
        if request.policy().provenance() {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::ProvenanceInvalid,
                CODE_UNVERIFIED_CLAIM,
                "provenance contains claims that have not been independently verified",
            ));
        }
    }

    fn validate_completeness(
        &self,
        violations: &mut Vec<VerificationViolation>,
    ) {
        if !completeness_at_least(
            self.snapshot.completeness(),
            self.minimum_completeness,
        ) {
            violations.push(VerificationViolation::new(
                VerificationStage::Provenance,
                VerificationViolationKind::MissingEvidence,
                CODE_MISSING_PROGRAM_IDENTITY,
                "provenance completeness is below the configured verification requirement",
            ));
        }
    }
}

impl ProvenanceVerifier for DefaultProvenanceVerifier {
    fn id(&self) -> &str {
        "zamani.resilience.verification.provenance.default"
    }

    fn verify_provenance(
        &self,
        request: &VerificationRequest,
    ) -> ResilienceResult<VerificationEvidence> {
        /*
         * Structural validation errors indicate a broken provenance object or
         * incompatible verifier configuration.
         *
         * They are converted into verification violations rather than
         * panicking. A malformed provenance object must never crash the
         * resilience controller.
         */
        let mut violations = Vec::new();

        self.validate_schema(&mut violations);
        self.validate_request_identity(request, &mut violations);
        self.validate_semantic_identity(request, &mut violations);
        self.validate_resources(request, &mut violations);
        self.validate_integrity_identity(&mut violations);
        self.validate_sequences(&mut violations);
        self.validate_completeness(&mut violations);
        self.validate_claims(request, &mut violations);

        if violations.is_empty() {
            Ok(VerificationEvidence::passed(
                VerificationConfidence::certain(),
            ))
        } else {
            Ok(VerificationEvidence::new(
                VerificationState::Failed,
                VerificationConfidence::certain(),
                violations,
            ))
        }
    }
}

// =============================================================================
// Helpers
// =============================================================================

fn validate_non_empty(
    field: &'static str,
    value: &str,
) -> ResilienceResult<()> {
    if value.trim().is_empty() {
        return Err(ResilienceError::new(
            ResilienceErrorCode::InvalidIdentifier,
            format!("{field} must not be empty"),
        ));
    }

    Ok(())
}

fn has_duplicates<T>(values: &[T]) -> bool
where
    T: Ord + Copy,
{
    if values.len() < 2 {
        return false;
    }

    let mut sorted = values.to_vec();
    sorted.sort_unstable();

    sorted.windows(2).any(|pair| pair[0] == pair[1])
}

fn strictly_increasing<I>(values: I) -> bool
where
    I: IntoIterator<Item = u64>,
{
    let mut previous: Option<u64> = None;

    for value in values {
        if value == 0 {
            return false;
        }

        if let Some(previous_value) = previous {
            if value <= previous_value {
                return false;
            }
        }

        previous = Some(value);
    }

    true
}

fn validate_event_sequences(
    events: &[ProvenanceEvent],
) -> ResilienceResult<()> {
    if strictly_increasing(
        events.iter().map(ProvenanceEvent::sequence),
    ) {
        Ok(())
    } else {
        Err(ResilienceError::new(
            ResilienceErrorCode::InvariantViolation,
            "provenance event sequence is invalid",
        ))
    }
}

fn validate_verification_sequences(
    records: &[VerificationProvenanceRecord],
) -> ResilienceResult<()> {
    if strictly_increasing(
        records
            .iter()
            .map(VerificationProvenanceRecord::sequence),
    ) {
        Ok(())
    } else {
        Err(ResilienceError::new(
            ResilienceErrorCode::InvariantViolation,
            "verification provenance sequence is invalid",
        ))
    }
}

fn validate_recovery_sequences(
    records: &[RecoveryProvenanceRecord],
) -> ResilienceResult<()> {
    if strictly_increasing(
        records
            .iter()
            .map(RecoveryProvenanceRecord::sequence),
    ) {
        Ok(())
    } else {
        Err(ResilienceError::new(
            ResilienceErrorCode::InvariantViolation,
            "recovery provenance sequence is invalid",
        ))
    }
}

fn completeness_rank(
    completeness: ProvenanceCompleteness,
) -> u8 {
    match completeness {
        ProvenanceCompleteness::None => 0,
        ProvenanceCompleteness::Minimal => 1,
        ProvenanceCompleteness::Partial => 2,
        ProvenanceCompleteness::Complete => 3,
    }
}

fn completeness_at_least(
    actual: ProvenanceCompleteness,
    required: ProvenanceCompleteness,
) -> bool {
    completeness_rank(actual) >= completeness_rank(required)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(
        algorithm: &'static str,
        digest: &'static str,
    ) -> ContentIdentity {
        ContentIdentity::new(algorithm, digest)
            .expect("test content identity must be valid")
    }

    fn source() -> ProvenanceSource {
        ProvenanceSource::new("test")
            .expect("test source must be valid")
    }

    fn base_builder() -> ProvenanceSnapshotBuilder {
        let mut builder = ProvenanceSnapshot::builder("execution-1")
            .expect("execution id must be valid");

        builder = builder
            .with_program_identity(identity("sha256", "program"))
            .with_expected_semantic_identity(
                identity("semantic", "expected"),
            )
            .with_candidate_semantic_identity(
                identity("semantic", "candidate"),
            )
            .with_ir_identity(identity("sha256", "ir"))
            .with_compiled_identity(identity("sha256", "compiled"))
            .with_scheduled_identity(identity("sha256", "scheduled"))
            .with_routed_identity(identity("sha256", "routed"))
            .with_result_identity(identity("sha256", "result"))
            .with_resources(ProvenanceResourceScope::new(
                std::iter::empty::<QubitId>(),
                std::iter::empty::<PhysicalQubitId>(),
            ))
            .with_completeness(ProvenanceCompleteness::Complete);

        builder
    }

    #[test]
    fn snapshot_round_trip_builds() {
        let snapshot = base_builder()
            .build()
            .expect("snapshot must build");

        assert_eq!(
            snapshot.schema_id(),
            RESILIENCE_VERIFICATION_PROVENANCE_SCHEMA_ID
        );

        assert_eq!(
            snapshot.schema_version(),
            RESILIENCE_VERIFICATION_PROVENANCE_SCHEMA_VERSION
        );

        assert_eq!(snapshot.execution_id(), "execution-1");
    }

    #[test]
    fn event_sequence_is_deterministic() {
        let mut builder = base_builder();

        builder
            .push_event(
                ProvenanceEvent::new(
                    ProvenanceEventKind::Execution,
                    source(),
                    "execute",
                )
                .expect("event must build"),
            )
            .expect("event must append");

        builder
            .push_event(
                ProvenanceEvent::new(
                    ProvenanceEventKind::Verification,
                    source(),
                    "verify",
                )
                .expect("event must build"),
            )
            .expect("event must append");

        let snapshot = builder
            .build()
            .expect("snapshot must build");

        assert_eq!(snapshot.events().len(), 2);
        assert_eq!(snapshot.events()[0].sequence(), 1);
        assert_eq!(snapshot.events()[1].sequence(), 2);
    }

    #[test]
    fn verification_record_is_preserved() {
        let mut builder = base_builder();

        builder
            .push_verification_record(
                VerificationProvenanceRecord::new(
                    "test-verifier",
                    VerificationStage::Semantic,
                    VerificationState::Passed,
                    VerificationConfidence::certain(),
                    source(),
                )
                .expect("verification record must build"),
            )
            .expect("record must append");

        let snapshot = builder
            .build()
            .expect("snapshot must build");

        assert_eq!(
            snapshot.verification_records().len(),
            1
        );

        assert_eq!(
            snapshot.verification_records()[0].stage(),
            VerificationStage::Semantic
        );
    }

    #[test]
    fn recovery_record_is_preserved() {
        let mut builder = base_builder();

        builder
            .push_recovery_record(
                RecoveryProvenanceRecord::new(
                    "reroute",
                    source(),
                    true,
                )
                .expect("recovery record must build"),
            )
            .expect("record must append");

        let snapshot = builder
            .build()
            .expect("snapshot must build");

        assert_eq!(
            snapshot.recovery_records().len(),
            1
        );

        assert!(
            snapshot.recovery_records()[0].completed()
        );
    }

    #[test]
    fn duplicate_resources_are_normalized_by_builder() {
        let logical = vec![
            QubitId::from(0usize),
            QubitId::from(0usize),
        ];

        let physical = vec![
            PhysicalQubitId::from(1usize),
            PhysicalQubitId::from(1usize),
        ];

        let scope = ProvenanceResourceScope::new(
            logical,
            physical,
        );

        assert_eq!(scope.logical_count(), 1);
        assert_eq!(scope.physical_count(), 1);
    }

    #[test]
    fn verifier_accepts_matching_complete_provenance() {
        let snapshot = base_builder()
            .build()
            .expect("snapshot must build");

        let verifier = DefaultProvenanceVerifier::new(snapshot);

        let request = VerificationRequest::new(
            super::super::verifier::ExecutionIdentity::new(
                "execution-1",
            )
            .expect("execution identity"),
            super::super::verifier::SemanticFingerprint::new(
                "expected",
            )
            .expect("semantic identity"),
            super::super::verifier::VerificationResourceScope::new(
                std::iter::empty::<QubitId>(),
                std::iter::empty::<PhysicalQubitId>(),
            ),
        )
        .with_candidate_semantic_fingerprint(
            super::super::verifier::SemanticFingerprint::new(
                "candidate",
            )
            .expect("candidate identity"),
        )
        .with_execution_success();

        let evidence = verifier
            .verify_provenance(&request)
            .expect("verification must not error");

        assert_eq!(
            evidence.state(),
            VerificationState::Passed
        );
        assert_eq!(
            evidence.confidence(),
            VerificationConfidence::certain()
        );
    }

    #[test]
    fn verifier_rejects_execution_identity_mismatch() {
        let snapshot = base_builder()
            .build()
            .expect("snapshot must build");

        let verifier = DefaultProvenanceVerifier::new(snapshot);

        let request = VerificationRequest::new(
            super::super::verifier::ExecutionIdentity::new(
                "different-execution",
            )
            .expect("execution identity"),
            super::super::verifier::SemanticFingerprint::new(
                "expected",
            )
            .expect("semantic identity"),
            super::super::verifier::VerificationResourceScope::new(
                std::iter::empty::<QubitId>(),
                std::iter::empty::<PhysicalQubitId>(),
            ),
        );

        let evidence = verifier
            .verify_provenance(&request)
            .expect("verification must not error");

        assert_eq!(
            evidence.state(),
            VerificationState::Failed
        );

        assert!(
            evidence
                .violations()
                .iter()
                .any(|violation| {
                    violation.code()
                        == CODE_EXECUTION_ID_MISMATCH
                })
        );
    }

    #[test]
    fn verifier_rejects_semantic_identity_mismatch() {
        let snapshot = base_builder()
            .build()
            .expect("snapshot must build");

        let verifier = DefaultProvenanceVerifier::new(snapshot);

        let request = VerificationRequest::new(
            super::super::verifier::ExecutionIdentity::new(
                "execution-1",
            )
            .expect("execution identity"),
            super::super::verifier::SemanticFingerprint::new(
                "different",
            )
            .expect("semantic identity"),
            super::super::verifier::VerificationResourceScope::new(
                std::iter::empty::<QubitId>(),
                std::iter::empty::<PhysicalQubitId>(),
            ),
        );

        let evidence = verifier
            .verify_provenance(&request)
            .expect("verification must not error");

        assert_eq!(
            evidence.state(),
            VerificationState::Failed
        );

        assert!(
            evidence
                .violations()
                .iter()
                .any(|violation| {
                    violation.code()
                        == CODE_EXPECTED_FINGERPRINT_MISMATCH
                })
        );
    }

    #[test]
    fn strict_verifier_rejects_unverified_claims() {
        let mut builder = base_builder();

        builder
            .push_unverified_claim(
                "backend reported successful execution",
            )
            .expect("claim must be accepted");

        let snapshot = builder
            .build()
            .expect("snapshot must build");

        let verifier = DefaultProvenanceVerifier::new(snapshot);

        let request = VerificationRequest::new(
            super::super::verifier::ExecutionIdentity::new(
                "execution-1",
            )
            .expect("execution identity"),
            super::super::verifier::SemanticFingerprint::new(
                "expected",
            )
            .expect("semantic identity"),
            super::super::verifier::VerificationResourceScope::new(
                std::iter::empty::<QubitId>(),
                std::iter::empty::<PhysicalQubitId>(),
            ),
        )
        .with_execution_success();

        let evidence = verifier
            .verify_provenance(&request)
            .expect("verification must not error");

        assert_eq!(
            evidence.state(),
            VerificationState::Failed
        );
    }

    #[test]
    fn logical_and_physical_domains_remain_distinct() {
        let scope = ProvenanceResourceScope::new(
            [QubitId::from(0usize)],
            [PhysicalQubitId::from(0usize)],
        );

        assert_eq!(scope.logical_count(), 1);
        assert_eq!(scope.physical_count(), 1);

        assert_ne!(
            format!("{:?}", scope.logical_qubits()),
            format!("{:?}", scope.physical_qubits())
        );
    }

    #[test]
    fn empty_event_collection_is_valid() {
        let snapshot = base_builder()
            .build()
            .expect("snapshot must build");

        assert!(snapshot.events().is_empty());
    }

    #[test]
    fn content_identity_is_opaque() {
        let identity = ContentIdentity::new(
            "custom-algorithm",
            "opaque-digest",
        )
        .expect("identity must build");

        assert_eq!(
            identity.algorithm(),
            "custom-algorithm"
        );

        assert_eq!(
            identity.digest(),
            "opaque-digest"
        );
    }
}