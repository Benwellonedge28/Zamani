
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani Compiler Security Audit
//!
//! Path: src/compiler/security/audit.rs
//! Edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Provides the compiler-side security audit model and audit orchestration
//! boundary for Zamani.
//!
//! This module collects explicit observations from compiler subsystems,
//! correlates them into deterministic audit reports, evaluates whether
//! required audit evidence is complete, and delegates persistence to an
//! explicitly configured audit sink.
//!
//! It does not invent security findings or independently claim that an
//! artifact, compilation, backend, dependency, or program is safe.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - compiler audit event descriptions;
//! - audit event identifiers and correlation references;
//! - audit observations and findings;
//! - evidence completeness evaluation;
//! - audit report construction;
//! - audit persistence interfaces;
//! - persistence acknowledgement validation;
//! - explicit audit failure propagation.
//!
//! This module does NOT own:
//!
//! - source grammar or parsing;
//! - AST construction;
//! - semantic security analysis;
//! - authentication or identity establishment;
//! - authorization or policy enforcement;
//! - cryptographic verification;
//! - supply-chain verification;
//! - canonical provenance graph storage;
//! - IR representation or IR verification;
//! - backend implementation;
//! - runtime enforcement;
//! - hardware discovery;
//! - persistent storage implementation;
//! - general-purpose application logging.
//!
//! # Canonical integration boundaries
//!
//! - grammar/security/audit.g4:
//!   source-level audit intent.
//!
//! - grammar/security/security.g4:
//!   security grammar composition.
//!
//! - compiler/security/policy.rs:
//!   security policy evaluation.
//!
//! - compiler/security/supply_chain.rs:
//!   dependency and package evidence.
//!
//! - compiler/safety_guard.rs:
//!   instruction-stream security inspection.
//!
//! - frontend/semantic/model/provenance.rs:
//!   canonical semantic provenance.
//!
//! - distributed/security:
//!   authentication, authorization and runtime policy enforcement.
//!
//! - backend/security:
//!   backend security enforcement.
//!
//! - ai/security/audit.rs:
//!   AI-specific audit events and sink integration.
//!
//! The adapters connecting these authorities to this module must preserve
//! their original evidence and must not manufacture successful outcomes.
//!
//! # Scalability
//!
//! No universal maximum is imposed on:
//!
//! - audit events;
//! - findings;
//! - compiler stages;
//! - security domains;
//! - targets;
//! - dependencies;
//! - principals;
//! - artifacts;
//! - provenance references;
//! - computing domains.
//!
//! Optional operational budgets are supplied by the caller.
//! Actual capacity depends on available memory, processing capacity,
//! storage capacity and configured operational policies.
//!
//! # Security
//!
//! - Missing required evidence is not successful verification.
//! - Indeterminate evidence is not successful verification.
//! - A failed audit append is not successful persistence.
//! - An Accepted receipt is not durable persistence.
//! - An audit record is not proof that the recorded claim is true.
//! - A provenance reference is not cryptographic verification.
//! - A signature observation is not signature verification.
//! - An audit report does not replace runtime enforcement.
//!
//! # Concurrency
//!
//! The service owns no global mutable state and creates no implicit
//! background workers or unbounded queues.
//!
//! Audit sinks are responsible for documenting their concurrency,
//! atomicity, persistence, integrity and recovery guarantees.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

// ============================================================================
// 1. VERSIONING
// ============================================================================

/// Version of the compiler audit API.
pub const COMPILER_AUDIT_API_VERSION: u16 = 1;

/// Version of the serialized audit report contract.
pub const COMPILER_AUDIT_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// 2. RESULT AND ERRORS
// ============================================================================

/// Result returned by compiler audit operations.
pub type AuditResult<T> = Result<T, AuditError>;

/// Errors raised by the compiler audit subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuditError {
    /// A required identifier or descriptive field was empty.
    EmptyField {
        field: &'static str,
    },

    /// A field contained an invalid control character.
    InvalidField {
        field: &'static str,
    },

    /// An event identifier was duplicated in a report.
    DuplicateEvent {
        id: String,
    },

    /// An observation identifier was duplicated.
    DuplicateObservation {
        id: String,
    },

    /// A finding identifier was duplicated.
    DuplicateFinding {
        id: String,
    },

    /// A report contains an event with an incompatible schema version.
    UnsupportedSchemaVersion {
        version: u16,
    },

    /// The caller exceeded an explicitly configured operational budget.
    BudgetExceeded {
        resource: &'static str,
        limit: usize,
    },

    /// A required evidence item was not supplied.
    MissingRequiredEvidence {
        subject: String,
        evidence: String,
    },

    /// Evidence was indeterminate.
    IndeterminateEvidence {
        subject: String,
        evidence: String,
    },

    /// A storage provider returned a receipt for another event.
    ReceiptMismatch {
        expected: String,
        actual: String,
    },

    /// The configured sink did not meet the required durability.
    DurabilityNotSatisfied {
        event: String,
    },

    /// The audit sink failed.
    SinkFailure {
        message: String,
    },

    /// A report could not be constructed consistently.
    InvalidReport {
        message: String,
    },
}

impl fmt::Display for AuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(f, "audit field cannot be empty: {field}")
            }

            Self::InvalidField { field } => {
                write!(f, "invalid audit field: {field}")
            }

            Self::DuplicateEvent { id } => {
                write!(f, "duplicate audit event: {id}")
            }

            Self::DuplicateObservation { id } => {
                write!(f, "duplicate audit observation: {id}")
            }

            Self::DuplicateFinding { id } => {
                write!(f, "duplicate audit finding: {id}")
            }

            Self::UnsupportedSchemaVersion { version } => {
                write!(f, "unsupported audit schema version: {version}")
            }

            Self::BudgetExceeded { resource, limit } => {
                write!(
                    f,
                    "audit operational budget exceeded for {resource}: {limit}"
                )
            }

            Self::MissingRequiredEvidence {
                subject,
                evidence,
            } => {
                write!(
                    f,
                    "required audit evidence is missing for {subject}: {evidence}"
                )
            }

            Self::IndeterminateEvidence {
                subject,
                evidence,
            } => {
                write!(
                    f,
                    "audit evidence is indeterminate for {subject}: {evidence}"
                )
            }

            Self::ReceiptMismatch { expected, actual } => {
                write!(
                    f,
                    "audit receipt mismatch: expected {expected}, received {actual}"
                )
            }

            Self::DurabilityNotSatisfied { event } => {
                write!(
                    f,
                    "required audit durability was not established for {event}"
                )
            }

            Self::SinkFailure { message } => {
                write!(f, "audit sink failure: {message}")
            }

            Self::InvalidReport { message } => {
                write!(f, "invalid audit report: {message}")
            }
        }
    }
}

impl Error for AuditError {}

// ============================================================================
// 3. VALIDATION
// ============================================================================

fn validate_text(
    field: &'static str,
    value: &str,
) -> AuditResult<()> {
    if value.trim().is_empty() {
        return Err(AuditError::EmptyField { field });
    }

    if value.chars().any(char::is_control) {
        return Err(AuditError::InvalidField { field });
    }

    Ok(())
}

// ============================================================================
// 4. IDENTIFIERS
// ============================================================================

/// Caller-assigned audit event identity.
///
/// IDs must be stable within the audit storage and correlation domain.
/// This module deliberately does not invent a global identity scheme.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuditEventId(String);

impl AuditEventId {
    /// Construct an event identifier.
    pub fn new(value: impl Into<String>) -> AuditResult<Self> {
        let value = value.into();
        validate_text("event identifier", &value)?;
        Ok(Self(value))
    }

    /// Return the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AuditEventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Caller-assigned observation identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuditObservationId(String);

impl AuditObservationId {
    /// Construct an observation identifier.
    pub fn new(value: impl Into<String>) -> AuditResult<Self> {
        let value = value.into();
        validate_text("observation identifier", &value)?;
        Ok(Self(value))
    }

    /// Return the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AuditObservationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Caller-assigned finding identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuditFindingId(String);

impl AuditFindingId {
    /// Construct a finding identifier.
    pub fn new(value: impl Into<String>) -> AuditResult<Self> {
        let value = value.into();
        validate_text("finding identifier", &value)?;
        Ok(Self(value))
    }

    /// Return the identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AuditFindingId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Open-world identifier for a compiler stage.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuditStageId(String);

impl AuditStageId {
    /// Construct a stage identifier.
    pub fn new(value: impl Into<String>) -> AuditResult<Self> {
        let value = value.into();
        validate_text("stage identifier", &value)?;
        Ok(Self(value))
    }

    /// Return the stage identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Open-world identifier for a security evidence type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidenceType(String);

impl EvidenceType {
    /// Construct an evidence type.
    pub fn new(value: impl Into<String>) -> AuditResult<Self> {
        let value = value.into();
        validate_text("evidence type", &value)?;
        Ok(Self(value))
    }

    /// Return the evidence type.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Open-world identifier for the audited subject.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuditSubject(String);

impl AuditSubject {
    /// Construct an audit subject.
    pub fn new(value: impl Into<String>) -> AuditResult<Self> {
        let value = value.into();
        validate_text("audit subject", &value)?;
        Ok(Self(value))
    }

    /// Return the subject.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// 5. TIME
// ============================================================================

/// Explicit timestamp supplied by a trusted clock or caller.
///
/// Unix nanoseconds are represented as signed 128-bit values to avoid
/// imposing an arbitrary date range on the audit model.
///
/// This type does not obtain wall-clock time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuditTimestamp {
    unix_nanoseconds: i128,
}

impl AuditTimestamp {
    /// Construct a timestamp from Unix nanoseconds.
    pub const fn from_unix_nanoseconds(value: i128) -> Self {
        Self {
            unix_nanoseconds: value,
        }
    }

    /// Return Unix nanoseconds.
    pub const fn unix_nanoseconds(self) -> i128 {
        self.unix_nanoseconds
    }
}

// ============================================================================
// 6. AUDIT CLASSIFICATION
// ============================================================================

/// Open-world audit event classification.
///
/// The symbolic value permits future security domains without changing
/// a closed compiler enum.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuditCategory(String);

impl AuditCategory {
    /// Construct a category.
    pub fn new(value: impl Into<String>) -> AuditResult<Self> {
        let value = value.into();
        validate_text("audit category", &value)?;
        Ok(Self(value))
    }

    /// Return the category.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Severity of an audit finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditSeverity {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

/// Result of an audited operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditOutcome {
    Succeeded,
    Failed,
    Denied,
    Rejected,
    Cancelled,
    NotApplicable,
    Indeterminate,
}

/// Security decision recorded by an authoritative subsystem.
///
/// Recording a decision does not create or grant that decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditDecision {
    Permit,
    Deny,
    NotApplicable,
    Indeterminate,
    NotEvaluated,
}

/// State of a particular evidence item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceStatus {
    Verified,
    Rejected,
    Missing,
    Indeterminate,
    NotApplicable,
}

// ============================================================================
// 7. REFERENCES
// ============================================================================

/// Reference to an external provenance record.
///
/// This is an opaque reference, not a locally created provenance record
/// and not proof that the referenced record is authentic.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceReference(String);

impl ProvenanceReference {
    /// Construct a provenance reference.
    pub fn new(value: impl Into<String>) -> AuditResult<Self> {
        let value = value.into();
        validate_text("provenance reference", &value)?;
        Ok(Self(value))
    }

    /// Return the reference.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Reference to a compiler artifact.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArtifactReference(String);

impl ArtifactReference {
    /// Construct an artifact reference.
    pub fn new(value: impl Into<String>) -> AuditResult<Self> {
        let value = value.into();
        validate_text("artifact reference", &value)?;
        Ok(Self(value))
    }

    /// Return the reference.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Reference to a source location, source node or source-level declaration.
///
/// The frontend adapter is responsible for translating its canonical
/// source-location representation into this opaque reference.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceReference(String);

impl SourceReference {
    /// Construct a source reference.
    pub fn new(value: impl Into<String>) -> AuditResult<Self> {
        let value = value.into();
        validate_text("source reference", &value)?;
        Ok(Self(value))
    }

    /// Return the reference.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// 8. EVIDENCE
// ============================================================================

/// A single evidence observation.
///
/// Evidence must originate from an actual subsystem result or an explicitly
/// identified external provider. This module does not infer verification
/// from metadata or from the existence of an artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvidence {
    evidence_type: EvidenceType,
    status: EvidenceStatus,
    provider: String,
    reference: Option<String>,
    explanation: Option<String>,
}

impl AuditEvidence {
    /// Construct evidence from an explicit provider result.
    pub fn new(
        evidence_type: EvidenceType,
        status: EvidenceStatus,
        provider: impl Into<String>,
    ) -> AuditResult<Self> {
        let provider = provider.into();
        validate_text("evidence provider", &provider)?;

        Ok(Self {
            evidence_type,
            status,
            provider,
            reference: None,
            explanation: None,
        })
    }

    /// Attach an opaque evidence reference.
    pub fn with_reference(
        mut self,
        reference: impl Into<String>,
    ) -> AuditResult<Self> {
        let reference = reference.into();
        validate_text("evidence reference", &reference)?;
        self.reference = Some(reference);
        Ok(self)
    }

    /// Attach a non-secret explanation.
    pub fn with_explanation(
        mut self,
        explanation: impl Into<String>,
    ) -> AuditResult<Self> {
        let explanation = explanation.into();
        validate_text("evidence explanation", &explanation)?;
        self.explanation = Some(explanation);
        Ok(self)
    }

    /// Return the evidence type.
    pub fn evidence_type(&self) -> &EvidenceType {
        &self.evidence_type
    }

    /// Return the evidence status.
    pub const fn status(&self) -> EvidenceStatus {
        self.status
    }

    /// Return the provider identifier.
    pub fn provider(&self) -> &str {
        &self.provider
    }

    /// Return the optional reference.
    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }

    /// Return the optional explanation.
    pub fn explanation(&self) -> Option<&str> {
        self.explanation.as_deref()
    }
}

// ============================================================================
// 9. AUDIT EVENT
// ============================================================================

/// Immutable compiler security audit event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    schema_version: u16,
    event_id: AuditEventId,
    timestamp: AuditTimestamp,
    category: AuditCategory,
    stage: AuditStageId,
    subject: AuditSubject,
    action: String,
    decision: AuditDecision,
    outcome: AuditOutcome,
    artifact: Option<ArtifactReference>,
    source: Option<SourceReference>,
    provenance: Vec<ProvenanceReference>,
    correlation_id: Option<String>,
    evidence: Vec<AuditEvidence>,
    metadata: BTreeMap<String, String>,
}

impl AuditEvent {
    /// Construct an audit event.
    ///
    /// Event identity and timestamp are supplied by the caller.
    /// The constructor does not authenticate the subject or authorize
    /// the recorded operation.
    pub fn new(
        event_id: AuditEventId,
        timestamp: AuditTimestamp,
        category: AuditCategory,
        stage: AuditStageId,
        subject: AuditSubject,
        action: impl Into<String>,
        decision: AuditDecision,
        outcome: AuditOutcome,
    ) -> AuditResult<Self> {
        let action = action.into();
        validate_text("audit action", &action)?;

        Ok(Self {
            schema_version: COMPILER_AUDIT_SCHEMA_VERSION,
            event_id,
            timestamp,
            category,
            stage,
            subject,
            action,
            decision,
            outcome,
            artifact: None,
            source: None,
            provenance: Vec::new(),
            correlation_id: None,
            evidence: Vec::new(),
            metadata: BTreeMap::new(),
        })
    }

    /// Attach an artifact reference.
    #[must_use]
    pub fn with_artifact(
        mut self,
        artifact: ArtifactReference,
    ) -> Self {
        self.artifact = Some(artifact);
        self
    }

    /// Attach a source reference.
    #[must_use]
    pub fn with_source(
        mut self,
        source: SourceReference,
    ) -> Self {
        self.source = Some(source);
        self
    }

    /// Attach a correlation identifier.
    pub fn with_correlation_id(
        mut self,
        value: impl Into<String>,
    ) -> AuditResult<Self> {
        let value = value.into();
        validate_text("correlation identifier", &value)?;
        self.correlation_id = Some(value);
        Ok(self)
    }

    /// Attach a canonical provenance reference.
    ///
    /// The referenced record must be managed by the canonical provenance
    /// subsystem. This method does not create or validate that record.
    #[must_use]
    pub fn with_provenance(
        mut self,
        reference: ProvenanceReference,
    ) -> Self {
        if !self.provenance.contains(&reference) {
            self.provenance.push(reference);
        }
        self
    }

    /// Attach evidence supplied by an authoritative provider.
    #[must_use]
    pub fn with_evidence(
        mut self,
        evidence: AuditEvidence,
    ) -> Self {
        self.evidence.push(evidence);
        self
    }

    /// Insert descriptive metadata.
    ///
    /// Callers must not include credentials, private keys, tokens,
    /// passwords, secret material or other sensitive values.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> AuditResult<Option<String>> {
        let key = key.into();
        let value = value.into();

        validate_text("metadata key", &key)?;
        validate_text("metadata value", &value)?;

        Ok(self.metadata.insert(key, value))
    }

    /// Return the schema version.
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Return the event identifier.
    pub fn event_id(&self) -> &AuditEventId {
        &self.event_id
    }

    /// Return the timestamp.
    pub const fn timestamp(&self) -> AuditTimestamp {
        self.timestamp
    }

    /// Return the category.
    pub fn category(&self) -> &AuditCategory {
        &self.category
    }

    /// Return the stage.
    pub fn stage(&self) -> &AuditStageId {
        &self.stage
    }

    /// Return the subject.
    pub fn subject(&self) -> &AuditSubject {
        &self.subject
    }

    /// Return the action.
    pub fn action(&self) -> &str {
        &self.action
    }

    /// Return the recorded decision.
    pub const fn decision(&self) -> AuditDecision {
        self.decision
    }

    /// Return the operation outcome.
    pub const fn outcome(&self) -> AuditOutcome {
        self.outcome
    }

    /// Return the optional artifact reference.
    pub fn artifact(&self) -> Option<&ArtifactReference> {
        self.artifact.as_ref()
    }

    /// Return the optional source reference.
    pub fn source(&self) -> Option<&SourceReference> {
        self.source.as_ref()
    }

    /// Return provenance references.
    pub fn provenance(&self) -> &[ProvenanceReference] {
        &self.provenance
    }

    /// Return the correlation identifier.
    pub fn correlation_id(&self) -> Option<&str> {
        self.correlation_id.as_deref()
    }

    /// Return evidence.
    pub fn evidence(&self) -> &[AuditEvidence] {
        &self.evidence
    }

    /// Return metadata.
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }
}

// ============================================================================
// 10. OBSERVATIONS
// ============================================================================

/// One observation submitted by a compiler subsystem.
///
/// Observations are facts reported by their originating subsystem.
/// They are not independently re-verified by this module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditObservation {
    id: AuditObservationId,
    stage: AuditStageId,
    subject: AuditSubject,
    category: AuditCategory,
    evidence: AuditEvidence,
    related_events: BTreeSet<AuditEventId>,
}

impl AuditObservation {
    /// Construct an observation.
    pub fn new(
        id: AuditObservationId,
        stage: AuditStageId,
        subject: AuditSubject,
        category: AuditCategory,
        evidence: AuditEvidence,
    ) -> Self {
        Self {
            id,
            stage,
            subject,
            category,
            evidence,
            related_events: BTreeSet::new(),
        }
    }

    /// Associate the observation with an event.
    #[must_use]
    pub fn with_event(
        mut self,
        event_id: AuditEventId,
    ) -> Self {
        self.related_events.insert(event_id);
        self
    }

    /// Return the observation identifier.
    pub fn id(&self) -> &AuditObservationId {
        &self.id
    }

    /// Return the stage.
    pub fn stage(&self) -> &AuditStageId {
        &self.stage
    }

    /// Return the subject.
    pub fn subject(&self) -> &AuditSubject {
        &self.subject
    }

    /// Return the category.
    pub fn category(&self) -> &AuditCategory {
        &self.category
    }

    /// Return the evidence.
    pub fn evidence(&self) -> &AuditEvidence {
        &self.evidence
    }

    /// Return related event identifiers.
    pub fn related_events(&self) -> &BTreeSet<AuditEventId> {
        &self.related_events
    }
}

// ============================================================================
// 11. FINDINGS
// ============================================================================

/// Origin of a finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindingOrigin {
    /// Reported by a compiler subsystem.
    Subsystem {
        stage: AuditStageId,
        provider: String,
    },

    /// Derived from missing required evidence.
    EvidenceCompleteness,

    /// Derived from conflicting audit data.
    AuditConsistency,
}

/// A structured audit finding.
///
/// Findings are created from supplied observations and explicit audit
/// requirements. The audit engine does not contain simulated checks,
/// fabricated vulnerabilities or hard-coded threat inventories.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditFinding {
    id: AuditFindingId,
    subject: AuditSubject,
    severity: AuditSeverity,
    code: String,
    message: String,
    origin: FindingOrigin,
    evidence: Option<AuditObservationId>,
}

impl AuditFinding {
    /// Construct a finding from an explicit subsystem result.
    pub fn new(
        id: AuditFindingId,
        subject: AuditSubject,
        severity: AuditSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
        origin: FindingOrigin,
    ) -> AuditResult<Self> {
        let code = code.into();
        let message = message.into();

        validate_text("finding code", &code)?;
        validate_text("finding message", &message)?;

        Ok(Self {
            id,
            subject,
            severity,
            code,
            message,
            origin,
            evidence: None,
        })
    }

    /// Attach the observation supporting this finding.
    #[must_use]
    pub fn with_observation(
        mut self,
        observation: AuditObservationId,
    ) -> Self {
        self.evidence = Some(observation);
        self
    }

    /// Return the finding identifier.
    pub fn id(&self) -> &AuditFindingId {
        &self.id
    }

    /// Return the subject.
    pub fn subject(&self) -> &AuditSubject {
        &self.subject
    }

    /// Return the severity.
    pub const fn severity(&self) -> AuditSeverity {
        self.severity
    }

    /// Return the finding code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Return the message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return the origin.
    pub fn origin(&self) -> &FindingOrigin {
        &self.origin
    }

    /// Return the supporting observation.
    pub fn observation(&self) -> Option<&AuditObservationId> {
        self.evidence.as_ref()
    }
}

// ============================================================================
// 12. REQUIREMENTS AND BUDGETS
// ============================================================================

/// An evidence requirement declared by the audit request.
///
/// Requirements originate from validated semantic policy or compiler
/// configuration. They are not inferred from a target name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiredEvidence {
    subject: AuditSubject,
    evidence_type: EvidenceType,
}

impl RequiredEvidence {
    /// Construct an evidence requirement.
    pub fn new(
        subject: AuditSubject,
        evidence_type: EvidenceType,
    ) -> Self {
        Self {
            subject,
            evidence_type,
        }
    }

    /// Return the subject.
    pub fn subject(&self) -> &AuditSubject {
        &self.subject
    }

    /// Return the required evidence type.
    pub fn evidence_type(&self) -> &EvidenceType {
        &self.evidence_type
    }
}

/// Optional operational budgets for audit analysis.
///
/// `None` means the caller has not configured that budget.
/// These are not language or hardware ceilings.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuditBudget {
    pub max_events: Option<usize>,
    pub max_observations: Option<usize>,
    pub max_findings: Option<usize>,
    pub max_evidence_items: Option<usize>,
}

impl AuditBudget {
    /// Construct an unbudgeted analysis policy.
    pub fn unlimited() -> Self {
        Self::default()
    }
}

/// Input to an audit analysis.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuditInput {
    events: Vec<AuditEvent>,
    observations: Vec<AuditObservation>,
    findings: Vec<AuditFinding>,
    requirements: Vec<RequiredEvidence>,
    budget: AuditBudget,
}

impl AuditInput {
    /// Construct an empty audit input.
    pub fn new(budget: AuditBudget) -> Self {
        Self {
            budget,
            ..Self::default()
        }
    }

    /// Add an event.
    pub fn add_event(&mut self, event: AuditEvent) -> AuditResult<()> {
        if let Some(limit) = self.budget.max_events {
            if self.events.len() >= limit {
                return Err(AuditError::BudgetExceeded {
                    resource: "events",
                    limit,
                });
            }
        }

        self.events.push(event);
        Ok(())
    }

    /// Add an observation.
    pub fn add_observation(
        &mut self,
        observation: AuditObservation,
    ) -> AuditResult<()> {
        if let Some(limit) = self.budget.max_observations {
            if self.observations.len() >= limit {
                return Err(AuditError::BudgetExceeded {
                    resource: "observations",
                    limit,
                });
            }
        }

        self.observations.push(observation);
        Ok(())
    }

    /// Add a finding.
    pub fn add_finding(
        &mut self,
        finding: AuditFinding,
    ) -> AuditResult<()> {
        if let Some(limit) = self.budget.max_findings {
            if self.findings.len() >= limit {
                return Err(AuditError::BudgetExceeded {
                    resource: "findings",
                    limit,
                });
            }
        }

        self.findings.push(finding);
        Ok(())
    }

    /// Add a required evidence declaration.
    pub fn require_evidence(
        &mut self,
        requirement: RequiredEvidence,
    ) {
        self.requirements.push(requirement);
    }

    /// Return the configured budget.
    pub fn budget(&self) -> &AuditBudget {
        &self.budget
    }
}

// ============================================================================
// 13. REPORT STATUS
// ============================================================================

/// Overall audit status.
///
/// `Complete` means the configured evidence requirements were satisfied.
/// It does not mean the audited program is universally safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditStatus {
    Complete,
    FindingsPresent,
    Incomplete,
    Indeterminate,
}

/// Overall audit decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditVerdict {
    /// Required audit evidence was supplied and no rejecting finding exists.
    Accepted,

    /// One or more supplied findings require rejection.
    Rejected,

    /// The audit cannot establish a reliable decision.
    Indeterminate,
}

// ============================================================================
// 14. AUDIT REPORT
// ============================================================================

/// Deterministic audit report.
///
/// Collections are sorted and indexed by stable identifiers before the
/// report is returned.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditReport {
    schema_version: u16,
    status: AuditStatus,
    verdict: AuditVerdict,
    events: Vec<AuditEvent>,
    observations: Vec<AuditObservation>,
    findings: Vec<AuditFinding>,
    missing_evidence: Vec<RequiredEvidence>,
    indeterminate_evidence: Vec<RequiredEvidence>,
}

impl AuditReport {
    /// Return schema version.
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Return overall status.
    pub const fn status(&self) -> AuditStatus {
        self.status
    }

    /// Return overall verdict.
    pub const fn verdict(&self) -> AuditVerdict {
        self.verdict
    }

    /// Return events.
    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Return observations.
    pub fn observations(&self) -> &[AuditObservation] {
        &self.observations
    }

    /// Return findings.
    pub fn findings(&self) -> &[AuditFinding] {
        &self.findings
    }

    /// Return missing required evidence.
    pub fn missing_evidence(&self) -> &[RequiredEvidence] {
        &self.missing_evidence
    }

    /// Return indeterminate required evidence.
    pub fn indeterminate_evidence(&self) -> &[RequiredEvidence] {
        &self.indeterminate_evidence
    }

    /// Whether the report established that all required evidence exists.
    pub fn evidence_complete(&self) -> bool {
        self.missing_evidence.is_empty()
            && self.indeterminate_evidence.is_empty()
    }

    /// Whether this report contains any findings.
    pub fn has_findings(&self) -> bool {
        !self.findings.is_empty()
    }
}

// ============================================================================
// 15. AUDIT ANALYZER
// ============================================================================

/// Deterministic compiler audit analyzer.
///
/// The analyzer only evaluates supplied observations and explicit
/// requirements. It does not scan arbitrary files, inspect arbitrary
/// machine state, or manufacture findings.
#[derive(Debug, Default)]
pub struct AuditAnalyzer;

impl AuditAnalyzer {
    /// Analyze compiler audit input.
    pub fn analyze(input: AuditInput) -> AuditResult<AuditReport> {
        Self::validate_budget(&input)?;

        let mut event_ids = BTreeSet::new();

        for event in &input.events {
            if event.schema_version != COMPILER_AUDIT_SCHEMA_VERSION {
                return Err(AuditError::UnsupportedSchemaVersion {
                    version: event.schema_version,
                });
            }

            if !event_ids.insert(event.event_id.clone()) {
                return Err(AuditError::DuplicateEvent {
                    id: event.event_id.to_string(),
                });
            }
        }

        let mut observation_ids = BTreeSet::new();

        for observation in &input.observations {
            if !observation_ids.insert(observation.id.clone()) {
                return Err(AuditError::DuplicateObservation {
                    id: observation.id.to_string(),
                });
            }

            for event_id in &observation.related_events {
                if !event_ids.contains(event_id) {
                    return Err(AuditError::InvalidReport {
                        message: format!(
                            "observation {} references unknown event {}",
                            observation.id,
                            event_id
                        ),
                    });
                }
            }
        }

        let mut finding_ids = BTreeSet::new();

        for finding in &input.findings {
            if !finding_ids.insert(finding.id.clone()) {
                return Err(AuditError::DuplicateFinding {
                    id: finding.id.to_string(),
                });
            }

            if let Some(observation_id) = &finding.evidence {
                if !observation_ids.contains(observation_id) {
                    return Err(AuditError::InvalidReport {
                        message: format!(
                            "finding {} references unknown observation {}",
                            finding.id,
                            observation_id
                        ),
                    });
                }
            }
        }

        let mut missing_evidence = Vec::new();
        let mut indeterminate_evidence = Vec::new();

        for requirement in &input.requirements {
            let matching = input.observations.iter().filter(|observation| {
                observation.subject == requirement.subject
                    && observation.evidence.evidence_type
                        == requirement.evidence_type
            });

            let mut verified = false;
            let mut rejected = false;
            let mut indeterminate = false;

            for observation in matching {
                match observation.evidence.status {
                    EvidenceStatus::Verified => verified = true,
                    EvidenceStatus::Rejected => rejected = true,
                    EvidenceStatus::Indeterminate => indeterminate = true,
                    EvidenceStatus::Missing
                    | EvidenceStatus::NotApplicable => {}
                }
            }

            if rejected {
                // Rejected evidence is represented as a finding by its
                // authoritative subsystem. It must not be rewritten as
                // missing or successful evidence.
                indeterminate_evidence.push(requirement.clone());
            } else if !verified && indeterminate {
                indeterminate_evidence.push(requirement.clone());
            } else if !verified {
                missing_evidence.push(requirement.clone());
            }
        }

        let has_rejecting_finding = input.findings.iter().any(|finding| {
            finding.severity >= AuditSeverity::High
        });

        let has_indeterminate_event = input.events.iter().any(|event| {
            event.decision == AuditDecision::Indeterminate
                || event.decision == AuditDecision::NotEvaluated
                || event.outcome == AuditOutcome::Indeterminate
        });

        let evidence_incomplete =
            !missing_evidence.is_empty()
                || !indeterminate_evidence.is_empty();

        let verdict = if has_rejecting_finding {
            AuditVerdict::Rejected
        } else if evidence_incomplete || has_indeterminate_event {
            AuditVerdict::Indeterminate
        } else {
            AuditVerdict::Accepted
        };

        let status = match verdict {
            AuditVerdict::Accepted if input.findings.is_empty() => {
                AuditStatus::Complete
            }
            AuditVerdict::Accepted => AuditStatus::FindingsPresent,
            AuditVerdict::Rejected => AuditStatus::FindingsPresent,
            AuditVerdict::Indeterminate if evidence_incomplete => {
                AuditStatus::Incomplete
            }
            AuditVerdict::Indeterminate => AuditStatus::Indeterminate,
        };

        let mut events = input.events;
        let mut observations = input.observations;
        let mut findings = input.findings;

        events.sort_by(|a, b| a.event_id.cmp(&b.event_id));
        observations.sort_by(|a, b| a.id.cmp(&b.id));
        findings.sort_by(|a, b| a.id.cmp(&b.id));

        missing_evidence.sort_by(|a, b| {
            (&a.subject, &a.evidence_type)
                .cmp(&(&b.subject, &b.evidence_type))
        });

        indeterminate_evidence.sort_by(|a, b| {
            (&a.subject, &a.evidence_type)
                .cmp(&(&b.subject, &b.evidence_type))
        });

        Ok(AuditReport {
            schema_version: COMPILER_AUDIT_SCHEMA_VERSION,
            status,
            verdict,
            events,
            observations,
            findings,
            missing_evidence,
            indeterminate_evidence,
        })
    }

    fn validate_budget(input: &AuditInput) -> AuditResult<()> {
        if let Some(limit) = input.budget.max_events {
            if input.events.len() > limit {
                return Err(AuditError::BudgetExceeded {
                    resource: "events",
                    limit,
                });
            }
        }

        if let Some(limit) = input.budget.max_observations {
            if input.observations.len() > limit {
                return Err(AuditError::BudgetExceeded {
                    resource: "observations",
                    limit,
                });
            }
        }

        if let Some(limit) = input.budget.max_findings {
            if input.findings.len() > limit {
                return Err(AuditError::BudgetExceeded {
                    resource: "findings",
                    limit,
                });
            }
        }

        if let Some(limit) = input.budget.max_evidence_items {
            let total = input.observations.iter().try_fold(
                0usize,
                |count, observation| count.checked_add(
                    usize::from(
                        observation.evidence.status
                            != EvidenceStatus::NotApplicable
                    )
                ),
            );

            let total = total.ok_or(AuditError::InvalidReport {
                message: "evidence count overflow".to_string(),
            })?;

            if total > limit {
                return Err(AuditError::BudgetExceeded {
                    resource: "evidence items",
                    limit,
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// 16. STORAGE ACKNOWLEDGEMENT
// ============================================================================

/// Durability established by the configured storage provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditDurability {
    /// Accepted by the provider, but durability is not established.
    Accepted,

    /// Persisted according to the provider's durable-storage contract.
    Durable,

    /// Replicated according to the provider's replication contract.
    Replicated,
}

/// Acknowledgement returned by an audit sink.
///
/// A receipt is evidence of the provider's acknowledgement, not
/// independent cryptographic proof of the stored record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditReceipt {
    event_id: AuditEventId,
    storage_reference: String,
    durability: AuditDurability,
}

impl AuditReceipt {
    /// Construct a receipt.
    ///
    /// The storage provider must call this only after meeting its
    /// documented append contract.
    pub fn new(
        event_id: AuditEventId,
        storage_reference: impl Into<String>,
        durability: AuditDurability,
    ) -> AuditResult<Self> {
        let storage_reference = storage_reference.into();
        validate_text("storage reference", &storage_reference)?;

        Ok(Self {
            event_id,
            storage_reference,
            durability,
        })
    }

    /// Return the event identifier.
    pub fn event_id(&self) -> &AuditEventId {
        &self.event_id
    }

    /// Return the storage reference.
    pub fn storage_reference(&self) -> &str {
        &self.storage_reference
    }

    /// Return the durability.
    pub const fn durability(&self) -> AuditDurability {
        self.durability
    }
}

// ============================================================================
// 17. AUDIT SINK
// ============================================================================

/// Provider-neutral audit persistence interface.
///
/// Implementations may use:
///
/// - append-only databases;
/// - durable event stores;
/// - remote audit services;
/// - tamper-evident storage;
/// - replicated distributed storage;
/// - externally managed security audit infrastructure.
///
/// Implementations must document their atomicity, durability, integrity,
/// concurrency, duplicate handling and recovery guarantees.
///
/// They must never silently discard events while returning success.
pub trait AuditSink: Send + Sync {
    /// Append one event and return an explicit acknowledgement.
    fn append(&self, event: &AuditEvent) -> AuditResult<AuditReceipt>;
}

/// Provider-neutral report persistence interface.
///
/// A report may contain multiple events and findings and may be persisted
/// atomically by a provider. Implementations must not claim atomicity unless
/// their storage contract actually provides it.
pub trait AuditReportSink: Send + Sync {
    /// Persist a complete audit report.
    fn append_report(&self, report: &AuditReport) -> AuditResult<AuditReportReceipt>;
}

/// Acknowledgement for a persisted report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditReportReceipt {
    report_reference: String,
    durability: AuditDurability,
}

impl AuditReportReceipt {
    /// Construct a report receipt.
    pub fn new(
        report_reference: impl Into<String>,
        durability: AuditDurability,
    ) -> AuditResult<Self> {
        let report_reference = report_reference.into();
        validate_text("report reference", &report_reference)?;

        Ok(Self {
            report_reference,
            durability,
        })
    }

    /// Return the storage reference.
    pub fn report_reference(&self) -> &str {
        &self.report_reference
    }

    /// Return the durability.
    pub const fn durability(&self) -> AuditDurability {
        self.durability
    }
}

// ============================================================================
// 18. AUDIT SERVICE
// ============================================================================

/// Compiler security audit service.
///
/// This service has no implicit discard sink, no in-memory fallback,
/// no background worker and no unbounded event queue.
pub struct AuditService {
    sink: Arc<dyn AuditSink>,
    report_sink: Option<Arc<dyn AuditReportSink>>,
}

impl AuditService {
    /// Construct an audit service with an explicit event sink.
    pub fn new(sink: Arc<dyn AuditSink>) -> Self {
        Self {
            sink,
            report_sink: None,
        }
    }

    /// Attach an optional report sink.
    #[must_use]
    pub fn with_report_sink(
        mut self,
        sink: Arc<dyn AuditReportSink>,
    ) -> Self {
        self.report_sink = Some(sink);
        self
    }

    /// Persist one event.
    ///
    /// The returned receipt must be checked by the caller against the
    /// durability required by the operation.
    pub fn record(&self, event: &AuditEvent) -> AuditResult<AuditReceipt> {
        let receipt = self.sink.append(event)?;

        if receipt.event_id != event.event_id {
            return Err(AuditError::ReceiptMismatch {
                expected: event.event_id.to_string(),
                actual: receipt.event_id.to_string(),
            });
        }

        Ok(receipt)
    }

    /// Persist one event and require durable or replicated acknowledgement.
    pub fn record_durable(
        &self,
        event: &AuditEvent,
    ) -> AuditResult<AuditReceipt> {
        let receipt = self.record(event)?;

        match receipt.durability {
            AuditDurability::Durable | AuditDurability::Replicated => {
                Ok(receipt)
            }
            AuditDurability::Accepted => {
                Err(AuditError::DurabilityNotSatisfied {
                    event: event.event_id.to_string(),
                })
            }
        }
    }

    /// Persist a complete report through the configured report sink.
    pub fn record_report(
        &self,
        report: &AuditReport,
    ) -> AuditResult<AuditReportReceipt> {
        let sink = self.report_sink.as_ref().ok_or_else(|| {
            AuditError::SinkFailure {
                message: "no report sink is configured".to_string(),
            }
        })?;

        sink.append_report(report)
    }

    /// Persist a complete report and require durable or replicated storage.
    pub fn record_report_durable(
        &self,
        report: &AuditReport,
    ) -> AuditResult<AuditReportReceipt> {
        let receipt = self.record_report(report)?;

        match receipt.durability {
            AuditDurability::Durable | AuditDurability::Replicated => {
                Ok(receipt)
            }
            AuditDurability::Accepted => {
                Err(AuditError::DurabilityNotSatisfied {
                    event: receipt.report_reference,
                })
            }
        }
    }
}

impl fmt::Debug for AuditService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuditService")
            .field("sink", &"[CONFIGURED AUDIT PROVIDER]")
            .field("report_sink_configured", &self.report_sink.is_some())
            .finish()
    }
}

// ============================================================================
// 19. TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn event(id: &str) -> AuditEvent {
        AuditEvent::new(
            AuditEventId::new(id).expect("valid ID"),
            AuditTimestamp::from_unix_nanoseconds(1),
            AuditCategory::new("compiler.security").expect("valid category"),
            AuditStageId::new("ir.verification").expect("valid stage"),
            AuditSubject::new("module.example").expect("valid subject"),
            "verify_ir",
            AuditDecision::Permit,
            AuditOutcome::Succeeded,
        )
        .expect("valid event")
    }

    fn requirement() -> RequiredEvidence {
        RequiredEvidence::new(
            AuditSubject::new("module.example").expect("subject"),
            EvidenceType::new("ir.verification").expect("evidence type"),
        )
    }

    fn verified_observation() -> AuditObservation {
        AuditObservation::new(
            AuditObservationId::new("observation.1").expect("ID"),
            AuditStageId::new("ir.verification").expect("stage"),
            AuditSubject::new("module.example").expect("subject"),
            AuditCategory::new("compiler.security").expect("category"),
            AuditEvidence::new(
                EvidenceType::new("ir.verification").expect("type"),
                EvidenceStatus::Verified,
                "canonical_ir_verifier",
            )
            .expect("evidence"),
        )
    }

    #[test]
    fn verified_required_evidence_produces_complete_report() {
        let mut input = AuditInput::new(AuditBudget::unlimited());
        input.add_event(event("event.1")).expect("event");
        input
            .add_observation(verified_observation())
            .expect("observation");
        input.require_evidence(requirement());

        let report = AuditAnalyzer::analyze(input).expect("report");

        assert_eq!(report.status(), AuditStatus::Complete);
        assert_eq!(report.verdict(), AuditVerdict::Accepted);
        assert!(report.evidence_complete());
    }

    #[test]
    fn missing_required_evidence_is_indeterminate() {
        let mut input = AuditInput::new(AuditBudget::unlimited());
        input.require_evidence(requirement());

        let report = AuditAnalyzer::analyze(input).expect("report");

        assert_eq!(report.verdict(), AuditVerdict::Indeterminate);
        assert_eq!(report.status(), AuditStatus::Incomplete);
        assert!(!report.evidence_complete());
    }

    #[test]
    fn indeterminate_evidence_is_not_accepted() {
        let mut input = AuditInput::new(AuditBudget::unlimited());
        let observation = AuditObservation::new(
            AuditObservationId::new("observation.2").expect("ID"),
            AuditStageId::new("ir.verification").expect("stage"),
            AuditSubject::new("module.example").expect("subject"),
            AuditCategory::new("compiler.security").expect("category"),
            AuditEvidence::new(
                EvidenceType::new("ir.verification").expect("type"),
                EvidenceStatus::Indeterminate,
                "canonical_ir_verifier",
            )
            .expect("evidence"),
        );

        input.add_observation(observation).expect("observation");
        input.require_evidence(requirement());

        let report = AuditAnalyzer::analyze(input).expect("report");

        assert_eq!(report.verdict(), AuditVerdict::Indeterminate);
    }

    #[test]
    fn high_severity_finding_rejects_report() {
        let mut input = AuditInput::new(AuditBudget::unlimited());

        input
            .add_finding(
                AuditFinding::new(
                    AuditFindingId::new("finding.1").expect("ID"),
                    AuditSubject::new("module.example").expect("subject"),
                    AuditSeverity::High,
                    "SECURITY_REJECTION",
                    "The authoritative verifier rejected the module",
                    FindingOrigin::Subsystem {
                        stage: AuditStageId::new("ir.verification")
                            .expect("stage"),
                        provider: "canonical_ir_verifier".to_string(),
                    },
                )
                .expect("finding"),
            )
            .expect("add finding");

        let report = AuditAnalyzer::analyze(input).expect("report");

        assert_eq!(report.verdict(), AuditVerdict::Rejected);
    }

    #[test]
    fn duplicate_events_are_rejected() {
        let mut input = AuditInput::new(AuditBudget::unlimited());
        input.add_event(event("same")).expect("first event");
        input.add_event(event("same")).expect("second event");

        assert!(matches!(
            AuditAnalyzer::analyze(input),
            Err(AuditError::DuplicateEvent { .. })
        ));
    }

    #[test]
    fn event_budget_is_enforced_when_configured() {
        let mut input = AuditInput::new(AuditBudget {
            max_events: Some(1),
            ..AuditBudget::default()
        });

        input.add_event(event("one")).expect("first event");

        assert!(matches!(
            input.add_event(event("two")),
            Err(AuditError::BudgetExceeded { .. })
        ));
    }

    #[test]
    fn event_order_is_deterministic() {
        let mut input = AuditInput::new(AuditBudget::unlimited());
        input.add_event(event("z")).expect("event");
        input.add_event(event("a")).expect("event");

        let report = AuditAnalyzer::analyze(input).expect("report");

        assert_eq!(report.events()[0].event_id().as_str(), "a");
        assert_eq!(report.events()[1].event_id().as_str(), "z");
    }

    #[test]
    fn not_evaluated_decision_is_not_accepted() {
        let mut input = AuditInput::new(AuditBudget::unlimited());

        input
            .add_event(
                AuditEvent::new(
                    AuditEventId::new("event.unknown").expect("ID"),
                    AuditTimestamp::from_unix_nanoseconds(1),
                    AuditCategory::new("compiler.security").expect("category"),
                    AuditStageId::new("security.analysis").expect("stage"),
                    AuditSubject::new("module.example").expect("subject"),
                    "analyze",
                    AuditDecision::NotEvaluated,
                    AuditOutcome::Indeterminate,
                )
                .expect("event"),
            )
            .expect("add event");

        let report = AuditAnalyzer::analyze(input).expect("report");

        assert_eq!(report.verdict(), AuditVerdict::Indeterminate);
    }

    #[test]
    fn metadata_rejects_empty_keys() {
        let mut value = event("metadata");

        assert!(matches!(
            value.insert_metadata("", "value"),
            Err(AuditError::EmptyField { .. })
        ));
    }
}
