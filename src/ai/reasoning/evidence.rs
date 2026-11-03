
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI Evidence
//!
//! Path: `src/ai/reasoning/evidence.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Define AI-domain evidence records and evidence relationships.
//!
//! Evidence describes observations, source material, provenance, and
//! relationships to claims or other evidence. It does not establish
//! that a claim is true.
//!
//! ## Ownership
//!
//! This module owns:
//! - Evidence record descriptions.
//! - Evidence source references.
//! - Evidence locators.
//! - Evidence relationships.
//! - Evidence access and verification states.
//! - Evidence collection validation.
//! - Evidence-specific structural diagnostics.
//!
//! This module does not own:
//! - Source syntax or AST construction.
//! - Canonical semantic validation.
//! - Universal provenance storage.
//! - Persistent knowledge storage.
//! - Inference execution.
//! - Statistical confidence computation.
//! - Authentication or authorization.
//! - Cryptographic signature verification.
//! - Universal IR.
//! - Hardware allocation or scheduling.
//!
//! ## Integration
//!
//! - `src/ai/ids.rs` supplies `AiEvidenceId`.
//! - `src/ai/provenance.rs` owns AI provenance integration.
//! - `src/ai/reasoning/knowledge.rs` owns knowledge representation.
//! - `src/frontend/semantic/domains/ai.rs` owns AI semantic validation.
//! - `grammar/ai/evidence.g4` composes the universal evidence grammar.
//! - `grammar/expressions/evidence.g4` owns evidence expression syntax.
//!
//! No competing AST, semantic model, provenance graph, or knowledge store
//! is introduced here.
//!
//! ## Scalability
//!
//! No evidence-count, source-count, claim-count, or graph-depth ceiling is
//! imposed by this module. Collections grow according to available memory
//! and caller-supplied operational budgets.
//!
//! Rust collection allocation can fail or abort depending on the allocator.
//! This module does not claim that finite hardware can store infinite data.
//!
//! ## Validation
//!
//! Structural validation is deterministic and does not establish:
//! - Source authenticity.
//! - Scientific reliability.
//! - Truth of a claim.
//! - Correctness of an inference.
//! - Authorization to access a source.
//!
//! Those properties require their authoritative services.

use crate::ai::ids::AiEvidenceId;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// ============================================================================
// SOURCE REFERENCES
// ============================================================================

/// Identifies the source from which an evidence observation originated.
///
/// The value is an opaque source identity. Source resolution, network access,
/// filesystem access, and authentication belong to other subsystems.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidenceSourceId(String);

impl EvidenceSourceId {
    /// Construct a source identity.
    pub fn new(value: impl Into<String>) -> Result<Self, EvidenceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(EvidenceError::EmptySourceIdentity);
        }

        Ok(Self(value))
    }

    /// Return the source identity.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EvidenceSourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Source material version.
///
/// This is descriptive metadata, not proof of authenticity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidenceSourceVersion(String);

impl EvidenceSourceVersion {
    pub fn new(value: impl Into<String>) -> Result<Self, EvidenceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(EvidenceError::EmptySourceVersion);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A location within source material.
///
/// The locator is intentionally format-neutral. It can describe a page,
/// paragraph, section, table, figure, equation, data row, source span,
/// or another source-defined location.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidenceLocator {
    value: String,
}

impl EvidenceLocator {
    pub fn new(value: impl Into<String>) -> Result<Self, EvidenceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(EvidenceError::EmptyLocator);
        }

        Ok(Self { value })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// EVIDENCE CLASSIFICATION
// ============================================================================

/// Describes the epistemic relationship between evidence and a claim.
///
/// These values are not numerical confidence scores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceRelation {
    /// The observation supports the claim within its stated scope.
    Supports,

    /// The observation conflicts with the claim.
    Contradicts,

    /// The observation limits the scope or applicability of the claim.
    Limits,

    /// The observation provides relevant context without directly
    /// establishing or contradicting the claim.
    Context,
}

/// Describes what kind of evidence is represented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceKind {
    Observation,
    ExperimentalResult,
    Measurement,
    Dataset,
    Simulation,
    Derivation,
    FormalProof,
    SourceStatement,
    ImplementationResult,
    VerificationResult,
    TestResult,
    Other,
}

/// Describes the acquisition state of the source material.
///
/// Access state is not a quality ranking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceAccessState {
    /// Only a discovery result or snippet was obtained.
    SearchHit,

    /// Bibliographic or source metadata was obtained.
    MetadataOnly,

    /// An abstract or summary was read.
    AbstractRead,

    /// A full-text asset is available but has not necessarily been read.
    FullTextFileAvailable,

    /// Relevant body text was actually read.
    FullTextTextRead,

    /// A claim-relevant location was located and read.
    FullTextLocated,
}

/// Describes verification state.
///
/// Verification is separate from source access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceVerification {
    /// No independent verification has been performed.
    Unverified,

    /// The source identity was checked.
    SourceIdentityVerified,

    /// A relevant source location was checked.
    LocationVerified,

    /// A relevant source detail was explicitly checked by a human.
    HumanVerified,

    /// Verification was attempted but remains unresolved.
    VerificationUnresolved,
}

/// Describes source identity status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceIdentityStatus {
    Unknown,
    Candidate,
    Matched,
    Conflicting,
}

// ============================================================================
// EVIDENCE OBSERVATION
// ============================================================================

/// An observation recorded from evidence.
///
/// This value does not assert that the observation is correct. Its
/// source, location, and verification status remain independently
/// inspectable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceObservation {
    kind: EvidenceKind,
    content: String,
    source: EvidenceSourceId,
    source_version: Option<EvidenceSourceVersion>,
    locator: Option<EvidenceLocator>,
    access: EvidenceAccessState,
    verification: EvidenceVerification,
    identity_status: EvidenceIdentityStatus,
}

impl EvidenceObservation {
    /// Construct an observation.
    pub fn new(
        kind: EvidenceKind,
        content: impl Into<String>,
        source: EvidenceSourceId,
    ) -> Result<Self, EvidenceError> {
        let content = content.into();

        if content.trim().is_empty() {
            return Err(EvidenceError::EmptyObservation);
        }

        Ok(Self {
            kind,
            content,
            source,
            source_version: None,
            locator: None,
            access: EvidenceAccessState::MetadataOnly,
            verification: EvidenceVerification::Unverified,
            identity_status: EvidenceIdentityStatus::Unknown,
        })
    }

    pub fn with_source_version(
        mut self,
        version: EvidenceSourceVersion,
    ) -> Self {
        self.source_version = Some(version);
        self
    }

    pub fn with_locator(mut self, locator: EvidenceLocator) -> Self {
        self.locator = Some(locator);
        self
    }

    pub fn with_access_state(
        mut self,
        access: EvidenceAccessState,
    ) -> Self {
        self.access = access;
        self
    }

    pub fn with_verification(
        mut self,
        verification: EvidenceVerification,
    ) -> Self {
        self.verification = verification;
        self
    }

    pub fn with_identity_status(
        mut self,
        status: EvidenceIdentityStatus,
    ) -> Self {
        self.identity_status = status;
        self
    }

    pub fn kind(&self) -> EvidenceKind {
        self.kind
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn source(&self) -> &EvidenceSourceId {
        &self.source
    }

    pub fn source_version(&self) -> Option<&EvidenceSourceVersion> {
        self.source_version.as_ref()
    }

    pub fn locator(&self) -> Option<&EvidenceLocator> {
        self.locator.as_ref()
    }

    pub fn access_state(&self) -> EvidenceAccessState {
        self.access
    }

    pub fn verification(&self) -> EvidenceVerification {
        self.verification
    }

    pub fn identity_status(&self) -> EvidenceIdentityStatus {
        self.identity_status
    }
}

// ============================================================================
// EVIDENCE RELATIONSHIPS
// ============================================================================

/// A relationship between evidence and a claim.
///
/// The claim identifier is opaque because claim identity belongs to the
/// canonical semantic or knowledge representation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidenceClaimReference {
    claim_id: String,
}

impl EvidenceClaimReference {
    pub fn new(value: impl Into<String>) -> Result<Self, EvidenceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(EvidenceError::EmptyClaimReference);
        }

        Ok(Self { claim_id: value })
    }

    pub fn as_str(&self) -> &str {
        &self.claim_id
    }
}

/// An evidence-to-claim relationship.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceClaimRelation {
    claim: EvidenceClaimReference,
    relation: EvidenceRelation,
    warrant: Option<String>,
}

impl EvidenceClaimRelation {
    pub fn new(
        claim: EvidenceClaimReference,
        relation: EvidenceRelation,
    ) -> Self {
        Self {
            claim,
            relation,
            warrant: None,
        }
    }

    /// Attach a concise explanation of why the observation is relevant.
    ///
    /// This is descriptive metadata, not an inference engine.
    pub fn with_warrant(mut self, warrant: impl Into<String>) -> Self {
        self.warrant = Some(warrant.into());
        self
    }

    pub fn claim(&self) -> &EvidenceClaimReference {
        &self.claim
    }

    pub fn relation(&self) -> EvidenceRelation {
        self.relation
    }

    pub fn warrant(&self) -> Option<&str> {
        self.warrant.as_deref()
    }
}

// ============================================================================
// PROVENANCE REFERENCES
// ============================================================================

/// An opaque reference to canonical provenance.
///
/// This module does not define a second provenance identity system.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidenceProvenanceReference(String);

impl EvidenceProvenanceReference {
    pub fn new(value: impl Into<String>) -> Result<Self, EvidenceError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(EvidenceError::EmptyProvenanceReference);
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// EVIDENCE RECORD
// ============================================================================

/// A complete evidence-domain record.
///
/// Construction validates local representation only. It does not establish
/// source authenticity, truth, relevance, or scientific reliability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRecord {
    identity: AiEvidenceId,
    observation: EvidenceObservation,
    relations: Vec<EvidenceClaimRelation>,
    provenance: Vec<EvidenceProvenanceReference>,
    metadata: BTreeMap<String, String>,
}

impl EvidenceRecord {
    pub fn new(
        identity: AiEvidenceId,
        observation: EvidenceObservation,
    ) -> Self {
        Self {
            identity,
            observation,
            relations: Vec::new(),
            provenance: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn add_relation(&mut self, relation: EvidenceClaimRelation) {
        self.relations.push(relation);
    }

    pub fn add_provenance(
        &mut self,
        provenance: EvidenceProvenanceReference,
    ) {
        self.provenance.push(provenance);
    }

    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, EvidenceError> {
        let key = key.into();

        if key.trim().is_empty() {
            return Err(EvidenceError::EmptyMetadataKey);
        }

        Ok(self.metadata.insert(key, value.into()))
    }

    pub fn identity(&self) -> AiEvidenceId {
        self.identity
    }

    pub fn observation(&self) -> &EvidenceObservation {
        &self.observation
    }

    pub fn relations(&self) -> &[EvidenceClaimRelation] {
        &self.relations
    }

    pub fn provenance(&self) -> &[EvidenceProvenanceReference] {
        &self.provenance
    }

    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }
}

// ============================================================================
// EVIDENCE COLLECTION
// ============================================================================

/// An evidence collection.
///
/// This is an in-memory value container, not a persistent evidence database.
/// The caller controls lifetime, storage, and operational resource budgets.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EvidenceCollection {
    records: BTreeMap<AiEvidenceId, EvidenceRecord>,
}

impl EvidenceCollection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a record.
    ///
    /// Duplicate identities are rejected. Existing records are not silently
    /// replaced.
    pub fn insert(
        &mut self,
        record: EvidenceRecord,
    ) -> Result<(), EvidenceError> {
        let identity = record.identity();

        if self.records.contains_key(&identity) {
            return Err(EvidenceError::DuplicateEvidenceIdentity(identity));
        }

        self.records.insert(identity, record);
        Ok(())
    }

    pub fn get(&self, identity: AiEvidenceId) -> Option<&EvidenceRecord> {
        self.records.get(&identity)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&AiEvidenceId, &EvidenceRecord)> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Validate structural consistency.
    ///
    /// A complete result means only that the requested structural checks
    /// completed. It does not mean that evidence is true or reliable.
    pub fn validate(&self) -> EvidenceValidationReport {
        let mut diagnostics = Vec::new();

        let mut claims: BTreeMap<
            EvidenceClaimReference,
            BTreeMap<EvidenceRelation, BTreeSet<AiEvidenceId>>,
        > = BTreeMap::new();

        for (identity, record) in &self.records {
            let observation = record.observation();

            if observation.identity_status()
                == EvidenceIdentityStatus::Conflicting
            {
                diagnostics.push(EvidenceDiagnostic::new(
                    EvidenceDiagnosticCode::ConflictingSourceIdentity,
                    Some(*identity),
                    "The evidence source identity is marked conflicting.",
                ));
            }

            if observation.access_state()
                == EvidenceAccessState::SearchHit
                && observation.verification()
                    != EvidenceVerification::Unverified
            {
                diagnostics.push(EvidenceDiagnostic::new(
                    EvidenceDiagnosticCode::VerificationExceedsAccess,
                    Some(*identity),
                    "Verification claims exceed the recorded access state.",
                ));
            }

            for relation in record.relations() {
                claims
                    .entry(relation.claim().clone())
                    .or_default()
                    .entry(relation.relation())
                    .or_default()
                    .insert(*identity);
            }
        }

        for (claim, relations) in &claims {
            let supports = relations
                .get(&EvidenceRelation::Supports)
                .is_some_and(|values| !values.is_empty());

            let contradicts = relations
                .get(&EvidenceRelation::Contradicts)
                .is_some_and(|values| !values.is_empty());

            if supports && contradicts {
                diagnostics.push(EvidenceDiagnostic::new(
                    EvidenceDiagnosticCode::ConflictingEvidence,
                    None,
                    format!(
                        "Claim '{}' has both supporting and \
                         contradicting evidence.",
                        claim.as_str()
                    ),
                ));
            }
        }

        diagnostics.sort();

        EvidenceValidationReport {
            completeness: EvidenceValidationCompleteness::Complete,
            diagnostics,
        }
    }
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Indicates whether validation completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceValidationCompleteness {
    Complete,
    Incomplete,
}

/// Stable diagnostic codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceDiagnosticCode {
    ConflictingSourceIdentity,
    VerificationExceedsAccess,
    ConflictingEvidence,
    MissingSourceLocator,
    MissingProvenance,
}

/// A structural evidence diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EvidenceDiagnostic {
    code: EvidenceDiagnosticCode,
    evidence: Option<AiEvidenceId>,
    message: String,
}

impl EvidenceDiagnostic {
    pub fn new(
        code: EvidenceDiagnosticCode,
        evidence: Option<AiEvidenceId>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            evidence,
            message: message.into(),
        }
    }

    pub fn code(&self) -> EvidenceDiagnosticCode {
        self.code
    }

    pub fn evidence(&self) -> Option<AiEvidenceId> {
        self.evidence
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Result of evidence structural validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceValidationReport {
    completeness: EvidenceValidationCompleteness,
    diagnostics: Vec<EvidenceDiagnostic>,
}

impl EvidenceValidationReport {
    pub fn completeness(&self) -> EvidenceValidationCompleteness {
        self.completeness
    }

    pub fn diagnostics(&self) -> &[EvidenceDiagnostic] {
        &self.diagnostics
    }

    pub fn is_structurally_valid(&self) -> bool {
        self.completeness == EvidenceValidationCompleteness::Complete
            && self.diagnostics.is_empty()
    }
}

// ============================================================================
// ERRORS
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceError {
    EmptySourceIdentity,
    EmptySourceVersion,
    EmptyLocator,
    EmptyObservation,
    EmptyClaimReference,
    EmptyProvenanceReference,
    EmptyMetadataKey,
    DuplicateEvidenceIdentity(AiEvidenceId),
}

impl fmt::Display for EvidenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySourceIdentity => {
                f.write_str("evidence source identity must not be empty")
            }
            Self::EmptySourceVersion => {
                f.write_str("evidence source version must not be empty")
            }
            Self::EmptyLocator => {
                f.write_str("evidence locator must not be empty")
            }
            Self::EmptyObservation => {
                f.write_str("evidence observation must not be empty")
            }
            Self::EmptyClaimReference => {
                f.write_str("evidence claim reference must not be empty")
            }
            Self::EmptyProvenanceReference => {
                f.write_str("evidence provenance reference must not be empty")
            }
            Self::EmptyMetadataKey => {
                f.write_str("evidence metadata key must not be empty")
            }
            Self::DuplicateEvidenceIdentity(identity) => {
                write!(f, "duplicate evidence identity: {identity}")
            }
        }
    }
}

impl std::error::Error for EvidenceError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(value: u128) -> AiEvidenceId {
        AiEvidenceId::new(value).expect("valid evidence identity")
    }

    fn observation() -> EvidenceObservation {
        EvidenceObservation::new(
            EvidenceKind::Observation,
            "Observed result",
            EvidenceSourceId::new("source:test").expect("valid source"),
        )
        .expect("valid observation")
    }

    fn record(value: u128) -> EvidenceRecord {
        EvidenceRecord::new(identity(value), observation())
    }

    #[test]
    fn source_identity_rejects_empty_values() {
        assert!(EvidenceSourceId::new("").is_err());
        assert!(EvidenceSourceId::new("source").is_ok());
    }

    #[test]
    fn observation_preserves_source_and_content() {
        let value = observation();

        assert_eq!(value.content(), "Observed result");
        assert_eq!(value.source().as_str(), "source:test");
    }

    #[test]
    fn evidence_collection_rejects_duplicate_identity() {
        let mut collection = EvidenceCollection::new();

        collection.insert(record(1)).expect("first insert");

        assert_eq!(
            collection.insert(record(1)),
            Err(EvidenceError::DuplicateEvidenceIdentity(identity(1)))
        );
    }

    #[test]
    fn evidence_collection_accepts_distinct_records() {
        let mut collection = EvidenceCollection::new();

        collection.insert(record(1)).expect("first record");
        collection.insert(record(2)).expect("second record");

        assert_eq!(collection.len(), 2);
    }

    #[test]
    fn evidence_relations_preserve_claim_relationship() {
        let claim = EvidenceClaimReference::new("claim:1")
            .expect("valid claim");

        let relation = EvidenceClaimRelation::new(
            claim,
            EvidenceRelation::Supports,
        )
        .with_warrant("The observation matches the claim's scope.");

        assert_eq!(relation.relation(), EvidenceRelation::Supports);
        assert!(relation.warrant().is_some());
    }

    #[test]
    fn contradictory_evidence_is_reported_not_silently_resolved() {
        let claim = EvidenceClaimReference::new("claim:1")
            .expect("valid claim");

        let mut supporting = record(1);
        supporting.add_relation(EvidenceClaimRelation::new(
            claim.clone(),
            EvidenceRelation::Supports,
        ));

        let mut contradicting = record(2);
        contradicting.add_relation(EvidenceClaimRelation::new(
            claim,
            EvidenceRelation::Contradicts,
        ));

        let mut collection = EvidenceCollection::new();

        collection.insert(supporting).expect("supporting evidence");
        collection.insert(contradicting).expect("contradicting evidence");

        let report = collection.validate();

        assert!(report.diagnostics().iter().any(|diagnostic| {
            diagnostic.code()
                == EvidenceDiagnosticCode::ConflictingEvidence
        }));
    }

    #[test]
    fn validation_is_deterministic() {
        let mut collection = EvidenceCollection::new();

        collection.insert(record(2)).expect("record");
        collection.insert(record(1)).expect("record");

        assert_eq!(collection.validate(), collection.validate());
    }

    #[test]
    fn empty_collection_is_valid() {
        let collection = EvidenceCollection::new();

        assert!(collection.validate().is_structurally_valid());
    }

    #[test]
    fn provenance_references_are_not_empty() {
        assert!(EvidenceProvenanceReference::new("").is_err());
        assert!(EvidenceProvenanceReference::new("prov:1").is_ok());
    }

    #[test]
    fn metadata_rejects_empty_keys() {
        let mut evidence = record(1);

        assert!(evidence.insert_metadata("", "value").is_err());
        assert!(evidence.insert_metadata("source", "value").is_ok());
    }
}
