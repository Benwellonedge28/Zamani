
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Toolchain — Lifecycle Security Provenance
//!
//! Path: src/toolchain/security/provenance.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! ## Mission
//!
//! Preserve verifiable lineage across the Zamani development lifecycle:
//!
//! Source -> Compilation -> Artifact -> Package -> Registry
//!        -> Installation -> Deployment -> Execution handoff
//!
//! This module provides toolchain lifecycle provenance records, immutable
//! snapshots, causal relationship validation, evidence references and
//! deterministic provenance fingerprints.
//!
//! ## Architectural ownership
//!
//! This module owns:
//! - Toolchain lifecycle provenance record contracts.
//! - Lifecycle provenance references.
//! - Source, artifact, package and deployment lineage references.
//! - Evidence references and reported evidence status.
//! - Lifecycle provenance snapshot construction.
//! - Causal relationship validation.
//! - Deterministic lifecycle provenance fingerprinting.
//!
//! This module does not own:
//! - The compiler provenance ledger.
//! - Compiler provenance event creation.
//! - Source parsing or semantic interpretation.
//! - Dependency resolution.
//! - Artifact payload storage.
//! - Package archive implementation.
//! - Registry implementation.
//! - Cryptographic algorithms.
//! - Signature generation or verification.
//! - Trust decisions.
//! - Authorization.
//! - Hardware discovery or allocation.
//! - Deployment execution.
//! - Runtime scheduling.
//! - Physical execution.
//!
//! ## Canonical authorities
//!
//! Compilation provenance:
//!     crate::compiler::context::provenance_context
//!
//! Compilation provenance fingerprinting:
//!     crate::compiler::reproducibility::provenance
//!
//! Artifact identity and integrity:
//!     crate::compiler::artifacts
//!
//! Canonical fingerprint implementation:
//!     crate::zuir::fingerprint::stable_hash
//!
//! Backend reproducibility:
//!     crate::backend::reproducibility
//!
//! Security verification:
//!     crate::compiler::security
//!
//! Normative provenance semantics:
//!     grammar/spec/provenance.md
//!
//! Compilation provenance syntax:
//!     grammar/compile/provenance.g4
//!
//! Security provenance syntax:
//!     grammar/security/provenance.g4
//!
//! ## Security invariants
//!
//! A fingerprint is not a digital signature.
//! A digest is not proof of authenticity.
//! A record is not proof that an event occurred.
//! A reported verification result is not independent verification.
//! A successful compilation is not proof of successful deployment.
//! An artifact identity is not proof of artifact integrity.
//!
//! All security-sensitive decisions remain with their authoritative
//! security, cryptography, integrity and authorization providers.
//!
//! ## Scalability
//!
//! No arbitrary universal maximum is imposed on:
//! - Provenance records.
//! - Parent relationships.
//! - Sources.
//! - Artifacts.
//! - Packages.
//! - Deployments.
//! - Evidence references.
//! - Properties.
//! - Providers.
//! - Targets.
//! - Computing domains.
//! - Distributed participants.
//!
//! Collections grow according to supplied data and available resources.
//!
//! Representation overflow and invalid relationships are reported.
//! Nothing is silently truncated.
//!
//! Literal infinite memory, processing or storage is not promised.
//!
//! ## Determinism
//!
//! - Records are indexed using ordered maps.
//! - Relationships use ordered sets.
//! - Fingerprints use the canonical ZUIR stable hashing implementation.
//! - Causal ordering is independent of insertion order.
//! - Wall-clock timestamps and process-local identities are not introduced.
//! - Fingerprint schema changes require explicit versioning.
//!
//! ## Persistence
//!
//! This module does not write files or contact registries.
//! Persistence and immutable storage are supplied by external providers.
//! A persisted snapshot must retain its schema and fingerprint identity.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use crate::zuir::fingerprint::stable_hash::{
    Fingerprint,
    FingerprintError,
    StableHashBuilder,
};

/// Public API version of lifecycle provenance.
pub const PROVENANCE_API_VERSION: u16 = 1;

/// Version of the canonical lifecycle provenance fingerprint schema.
pub const PROVENANCE_FINGERPRINT_SCHEMA_VERSION: u32 = 1;

/// Canonical fingerprint domain.
pub const PROVENANCE_FINGERPRINT_DOMAIN: &str =
    "zamani.toolchain.security.provenance";

/// Canonical provenance module identity.
pub const PROVENANCE_MODULE_ID: &str =
    "zamani.toolchain.security.provenance";

/// Returns the lifecycle provenance API version.
#[must_use]
pub const fn api_version() -> u16 {
    PROVENANCE_API_VERSION
}

// ============================================================================
// IDENTIFIER VALIDATION
// ============================================================================

fn validate_identifier(
    value: &str,
    description: &'static str,
) -> Result<(), ProvenanceError> {
    if value.trim().is_empty() {
        return Err(ProvenanceError::EmptyIdentifier {
            description,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(ProvenanceError::InvalidIdentifier {
            description,
        });
    }

    Ok(())
}

// ============================================================================
// OPEN IDENTIFIERS
// ============================================================================

/// Stable, extensible provenance identifier.
///
/// Identifiers are logical references, not filesystem paths, memory
/// addresses, physical device handles or authorization credentials.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceId(Arc<str>);

impl ProvenanceId {
    /// Creates a validated provenance identifier.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "provenance identifier")?;

        Ok(Self(value))
    }

    /// Returns the identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProvenanceId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Open-world identifier for a provenance lifecycle stage.
///
/// Examples include `compilation`, `artifact-generation`, `packaging`,
/// `registry-publication`, `installation` and `deployment`.
///
/// New stages do not require changes to this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceStageId(Arc<str>);

impl ProvenanceStageId {
    /// Creates a stage identifier.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "provenance stage")?;

        Ok(Self(value))
    }

    /// Returns the stage identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Open-world identity of a provenance subject.
///
/// A subject can represent a source, compilation, artifact, package,
/// registry publication, installation or deployment.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceSubjectId(Arc<str>);

impl ProvenanceSubjectId {
    /// Creates a subject identifier.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "provenance subject")?;

        Ok(Self(value))
    }

    /// Returns the subject identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Identity of the component or provider responsible for an activity.
///
/// This records a declared identity. Independent authentication is
/// performed by the appropriate trust provider.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceActorId(Arc<str>);

impl ProvenanceActorId {
    /// Creates an actor identity.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "provenance actor")?;

        Ok(Self(value))
    }

    /// Returns the actor identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Reference to an external provenance record.
///
/// The referenced record can belong to another authoritative subsystem.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceReference(Arc<str>);

impl ProvenanceReference {
    /// Creates an external provenance reference.
    pub fn new(
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let value = value.into();

        validate_identifier(value.as_ref(), "provenance reference")?;

        Ok(Self(value))
    }

    /// Returns the referenced identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// CONTENT IDENTITY
// ============================================================================

/// Externally calculated content digest.
///
/// This type stores an existing digest reference. It never calculates
/// or verifies a digest.
///
/// Supported algorithms are determined by the canonical cryptographic
/// and artifact-integrity authorities, not by a closed enum here.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentDigest {
    algorithm: Arc<str>,
    value: Arc<str>,
}

impl ContentDigest {
    /// Creates a digest reference.
    ///
    /// The digest must be supplied by an authoritative hashing provider.
    pub fn new(
        algorithm: impl Into<Arc<str>>,
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let algorithm = algorithm.into();
        let value = value.into();

        validate_identifier(algorithm.as_ref(), "digest algorithm")?;
        validate_identifier(value.as_ref(), "digest value")?;

        Ok(Self {
            algorithm,
            value,
        })
    }

    /// Returns the algorithm identifier.
    #[must_use]
    pub fn algorithm(&self) -> &str {
        &self.algorithm
    }

    /// Returns the digest value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// ARTIFACT REFERENCES
// ============================================================================

/// Reference to an artifact participating in lifecycle provenance.
///
/// The actual artifact and its payload remain owned by
/// `crate::compiler::artifacts`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceArtifact {
    identifier: ProvenanceSubjectId,
    kind: Arc<str>,
    format: Option<Arc<str>>,
    digest: Option<ContentDigest>,
    target: Option<Arc<str>>,
}

impl ProvenanceArtifact {
    /// Creates an artifact reference.
    pub fn new(
        identifier: ProvenanceSubjectId,
        kind: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let kind = kind.into();

        validate_identifier(kind.as_ref(), "artifact kind")?;

        Ok(Self {
            identifier,
            kind,
            format: None,
            digest: None,
            target: None,
        })
    }

    /// Associates an artifact format.
    pub fn with_format(
        mut self,
        format: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let format = format.into();

        validate_identifier(format.as_ref(), "artifact format")?;

        self.format = Some(format);

        Ok(self)
    }

    /// Associates an externally calculated content digest.
    pub fn with_digest(
        mut self,
        digest: ContentDigest,
    ) -> Self {
        self.digest = Some(digest);
        self
    }

    /// Associates an explicit target realization.
    ///
    /// This is descriptive metadata, not proof of target compatibility.
    pub fn with_target(
        mut self,
        target: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let target = target.into();

        validate_identifier(target.as_ref(), "artifact target")?;

        self.target = Some(target);

        Ok(self)
    }

    /// Returns the artifact identity.
    #[must_use]
    pub fn identifier(&self) -> &ProvenanceSubjectId {
        &self.identifier
    }

    /// Returns the artifact kind.
    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns the optional artifact format.
    #[must_use]
    pub fn format(&self) -> Option<&str> {
        self.format.as_deref()
    }

    /// Returns the optional content digest.
    #[must_use]
    pub fn digest(&self) -> Option<&ContentDigest> {
        self.digest.as_ref()
    }

    /// Returns the optional target realization.
    #[must_use]
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }
}

// ============================================================================
// EVIDENCE REFERENCES
// ============================================================================

/// Reported evidence state.
///
/// These variants describe the status reported by an evidence producer.
/// They do not independently authenticate or verify that evidence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum EvidenceStatus {
    /// Evidence has been declared but not independently inspected.
    Claimed,

    /// Evidence has been observed by a named source.
    Observed,

    /// An external verification authority reports successful verification.
    Verified,

    /// An external verification authority reports rejection.
    Rejected,

    /// Verification has not established a conclusion.
    Indeterminate,

    /// An extension-defined status.
    Custom(Arc<str>),
}

/// Reference to evidence maintained by an authoritative provider.
///
/// Verification implementation, evidence storage and trust decisions
/// remain outside this module.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceEvidence {
    identifier: ProvenanceReference,
    provider: ProvenanceActorId,
    status: EvidenceStatus,
    verifier: Option<ProvenanceActorId>,
    digest: Option<ContentDigest>,
}

impl ProvenanceEvidence {
    /// Creates an evidence reference.
    pub fn new(
        identifier: ProvenanceReference,
        provider: ProvenanceActorId,
        status: EvidenceStatus,
    ) -> Self {
        Self {
            identifier,
            provider,
            status,
            verifier: None,
            digest: None,
        }
    }

    /// Associates the external verifier identity.
    ///
    /// This records who reportedly performed verification. It does not
    /// authenticate that verifier.
    pub fn with_verifier(
        mut self,
        verifier: ProvenanceActorId,
    ) -> Self {
        self.verifier = Some(verifier);
        self
    }

    /// Associates an externally calculated evidence digest.
    pub fn with_digest(
        mut self,
        digest: ContentDigest,
    ) -> Self {
        self.digest = Some(digest);
        self
    }

    /// Returns the evidence identifier.
    #[must_use]
    pub fn identifier(&self) -> &ProvenanceReference {
        &self.identifier
    }

    /// Returns the evidence provider.
    #[must_use]
    pub fn provider(&self) -> &ProvenanceActorId {
        &self.provider
    }

    /// Returns the reported evidence status.
    #[must_use]
    pub fn status(&self) -> &EvidenceStatus {
        &self.status
    }

    /// Returns the reported verifier, if present.
    #[must_use]
    pub fn verifier(&self) -> Option<&ProvenanceActorId> {
        self.verifier.as_ref()
    }

    /// Returns the optional evidence digest.
    #[must_use]
    pub fn digest(&self) -> Option<&ContentDigest> {
        self.digest.as_ref()
    }
}

// ============================================================================
// ACTIVITY OUTCOME
// ============================================================================

/// Reported outcome of a lifecycle activity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ProvenanceOutcome {
    /// The activity reports successful completion.
    Succeeded,

    /// The activity was explicitly rejected.
    Rejected,

    /// The activity failed.
    Failed,

    /// The activity was intentionally skipped.
    Skipped,

    /// The activity remains incomplete.
    Incomplete,

    /// An extension-defined outcome.
    Custom(Arc<str>),
}

impl Default for ProvenanceOutcome {
    fn default() -> Self {
        Self::Incomplete
    }
}

// ============================================================================
// EXTENSIBLE PROPERTIES
// ============================================================================

/// Namespaced descriptive provenance metadata.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProvenanceProperty {
    key: Arc<str>,
    value: Arc<str>,
}

impl ProvenanceProperty {
    /// Creates a validated property.
    pub fn new(
        key: impl Into<Arc<str>>,
        value: impl Into<Arc<str>>,
    ) -> Result<Self, ProvenanceError> {
        let key = key.into();
        let value = value.into();

        validate_identifier(key.as_ref(), "property key")?;

        if value.chars().any(char::is_control) {
            return Err(ProvenanceError::InvalidPropertyValue);
        }

        Ok(Self {
            key,
            value,
        })
    }

    /// Returns the property key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the property value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// ============================================================================
// PROVENANCE RECORD
// ============================================================================

/// Immutable description of one toolchain lifecycle activity.
///
/// A record preserves lineage and evidence references. It does not
/// execute, authorize or verify the described activity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceRecord {
    identifier: ProvenanceId,
    stage: ProvenanceStageId,
    subject: ProvenanceSubjectId,
    actor: ProvenanceActorId,
    parents: BTreeSet<ProvenanceReference>,
    sources: BTreeSet<ProvenanceReference>,
    inputs: BTreeSet<ProvenanceArtifact>,
    outputs: BTreeSet<ProvenanceArtifact>,
    evidence: BTreeSet<ProvenanceEvidence>,
    properties: BTreeMap<Arc<str>, Arc<str>>,
    external_compiler_fingerprint: Option<Fingerprint>,
    outcome: ProvenanceOutcome,
}

impl ProvenanceRecord {
    /// Returns the record identity.
    #[must_use]
    pub fn identifier(&self) -> &ProvenanceId {
        &self.identifier
    }

    /// Returns the lifecycle stage.
    #[must_use]
    pub fn stage(&self) -> &ProvenanceStageId {
        &self.stage
    }

    /// Returns the subject.
    #[must_use]
    pub fn subject(&self) -> &ProvenanceSubjectId {
        &self.subject
    }

    /// Returns the actor.
    #[must_use]
    pub fn actor(&self) -> &ProvenanceActorId {
        &self.actor
    }

    /// Returns causal parent references.
    #[must_use]
    pub fn parents(&self) -> &BTreeSet<ProvenanceReference> {
        &self.parents
    }

    /// Returns source references.
    #[must_use]
    pub fn sources(&self) -> &BTreeSet<ProvenanceReference> {
        &self.sources
    }

    /// Returns input artifacts.
    #[must_use]
    pub fn inputs(&self) -> &BTreeSet<ProvenanceArtifact> {
        &self.inputs
    }

    /// Returns output artifacts.
    #[must_use]
    pub fn outputs(&self) -> &BTreeSet<ProvenanceArtifact> {
        &self.outputs
    }

    /// Returns associated evidence references.
    #[must_use]
    pub fn evidence(&self) -> &BTreeSet<ProvenanceEvidence> {
        &self.evidence
    }

    /// Returns descriptive properties.
    #[must_use]
    pub fn properties(&self) -> &BTreeMap<Arc<str>, Arc<str>> {
        &self.properties
    }

    /// Returns the external compiler provenance fingerprint.
    #[must_use]
    pub const fn external_compiler_fingerprint(
        &self,
    ) -> Option<Fingerprint> {
        self.external_compiler_fingerprint
    }

    /// Returns the reported activity outcome.
    #[must_use]
    pub fn outcome(&self) -> &ProvenanceOutcome {
        &self.outcome
    }
}

// ============================================================================
// RECORD BUILDER
// ============================================================================

/// Builder for immutable lifecycle provenance records.
#[derive(Debug, Clone)]
pub struct ProvenanceRecordBuilder {
    identifier: ProvenanceId,
    stage: ProvenanceStageId,
    subject: ProvenanceSubjectId,
    actor: ProvenanceActorId,
    parents: BTreeSet<ProvenanceReference>,
    sources: BTreeSet<ProvenanceReference>,
    inputs: BTreeSet<ProvenanceArtifact>,
    outputs: BTreeSet<ProvenanceArtifact>,
    evidence: BTreeSet<ProvenanceEvidence>,
    properties: BTreeMap<Arc<str>, Arc<str>>,
    external_compiler_fingerprint: Option<Fingerprint>,
    outcome: ProvenanceOutcome,
}

impl ProvenanceRecordBuilder {
    /// Creates a record builder with mandatory identities.
    pub fn new(
        identifier: ProvenanceId,
        stage: ProvenanceStageId,
        subject: ProvenanceSubjectId,
        actor: ProvenanceActorId,
    ) -> Self {
        Self {
            identifier,
            stage,
            subject,
            actor,
            parents: BTreeSet::new(),
            sources: BTreeSet::new(),
            inputs: BTreeSet::new(),
            outputs: BTreeSet::new(),
            evidence: BTreeSet::new(),
            properties: BTreeMap::new(),
            external_compiler_fingerprint: None,
            outcome: ProvenanceOutcome::Incomplete,
        }
    }

    /// Adds a causal parent.
    pub fn with_parent(
        mut self,
        parent: ProvenanceReference,
    ) -> Result<Self, ProvenanceError> {
        if parent.as_str() == self.identifier.as_str() {
            return Err(ProvenanceError::SelfReference);
        }

        self.parents.insert(parent);
        Ok(self)
    }

    /// Adds a source reference.
    pub fn with_source(
        mut self,
        source: ProvenanceReference,
    ) -> Self {
        self.sources.insert(source);
        self
    }

    /// Adds an input artifact.
    pub fn with_input(
        mut self,
        artifact: ProvenanceArtifact,
    ) -> Self {
        self.inputs.insert(artifact);
        self
    }

    /// Adds an output artifact.
    pub fn with_output(
        mut self,
        artifact: ProvenanceArtifact,
    ) -> Self {
        self.outputs.insert(artifact);
        self
    }

    /// Adds an evidence reference.
    pub fn with_evidence(
        mut self,
        evidence: ProvenanceEvidence,
    ) -> Self {
        self.evidence.insert(evidence);
        self
    }

    /// Adds a unique descriptive property.
    pub fn with_property(
        mut self,
        property: ProvenanceProperty,
    ) -> Result<Self, ProvenanceError> {
        if self.properties.contains_key(property.key()) {
            return Err(ProvenanceError::DuplicateProperty {
                key: Arc::from(property.key()),
            });
        }

        self.properties.insert(
            Arc::from(property.key()),
            Arc::from(property.value()),
        );

        Ok(self)
    }

    /// Associates the canonical compiler provenance fingerprint.
    ///
    /// The supplied fingerprint must originate from the compiler's
    /// authoritative provenance fingerprinting subsystem.
    pub fn with_compiler_provenance_fingerprint(
        mut self,
        fingerprint: Fingerprint,
    ) -> Self {
        self.external_compiler_fingerprint = Some(fingerprint);
        self
    }

    /// Sets the reported activity outcome.
    pub fn with_outcome(
        mut self,
        outcome: ProvenanceOutcome,
    ) -> Self {
        self.outcome = outcome;
        self
    }

    /// Builds the immutable record.
    pub fn build(self) -> ProvenanceRecord {
        ProvenanceRecord {
            identifier: self.identifier,
            stage: self.stage,
            subject: self.subject,
            actor: self.actor,
            parents: self.parents,
            sources: self.sources,
            inputs: self.inputs,
            outputs: self.outputs,
            evidence: self.evidence,
            properties: self.properties,
            external_compiler_fingerprint:
                self.external_compiler_fingerprint,
            outcome: self.outcome,
        }
    }
}

// ============================================================================
// PROVENANCE ERRORS
// ============================================================================

/// Errors encountered while constructing or fingerprinting provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvenanceError {
    /// An identifier is empty.
    EmptyIdentifier {
        /// Identifier category.
        description: &'static str,
    },

    /// An identifier contains forbidden control characters.
    InvalidIdentifier {
        /// Identifier category.
        description: &'static str,
    },

    /// A property value contains forbidden control characters.
    InvalidPropertyValue,

    /// A property key was supplied more than once.
    DuplicateProperty {
        /// Duplicated property key.
        key: Arc<str>,
    },

    /// A record references itself.
    SelfReference,

    /// A record identifier was supplied more than once.
    DuplicateRecord {
        /// Duplicated record identifier.
        identifier: Arc<str>,
    },

    /// A causal parent is missing from the snapshot.
    MissingParent {
        /// Referenced parent identifier.
        identifier: Arc<str>,
        /// Record containing the missing reference.
        child: Arc<str>,
    },

    /// A causal cycle exists.
    CausalCycle {
        /// Number of records that could not be ordered.
        remaining_records: usize,
    },

    /// A collection count exceeds its representable fingerprint format.
    CountOverflow {
        /// Collection name.
        collection: &'static str,
    },

    /// Canonical fingerprint construction failed.
    Fingerprinting(FingerprintError),
}

impl fmt::Display for ProvenanceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyIdentifier { description } => {
                write!(formatter, "{description} must not be empty")
            }

            Self::InvalidIdentifier { description } => {
                write!(
                    formatter,
                    "{description} contains forbidden control characters"
                )
            }

            Self::InvalidPropertyValue => {
                formatter.write_str(
                    "provenance property contains control characters",
                )
            }

            Self::DuplicateProperty { key } => {
                write!(formatter, "duplicate provenance property `{key}`")
            }

            Self::SelfReference => {
                formatter.write_str(
                    "a provenance record cannot reference itself as parent",
                )
            }

            Self::DuplicateRecord { identifier } => {
                write!(
                    formatter,
                    "duplicate provenance record `{identifier}`",
                )
            }

            Self::MissingParent { identifier, child } => {
                write!(
                    formatter,
                    "record `{child}` references missing parent `{identifier}`",
                )
            }

            Self::CausalCycle { remaining_records } => {
                write!(
                    formatter,
                    "provenance contains a causal cycle involving {remaining_records} records",
                )
            }

            Self::CountOverflow { collection } => {
                write!(
                    formatter,
                    "provenance {collection} count cannot be represented",
                )
            }

            Self::Fingerprinting(error) => {
                write!(
                    formatter,
                    "provenance fingerprinting failed: {error}",
                )
            }
        }
    }
}

impl Error for ProvenanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Fingerprinting(error) => Some(error),
            _ => None,
        }
    }
}

impl From<FingerprintError> for ProvenanceError {
    fn from(error: FingerprintError) -> Self {
        Self::Fingerprinting(error)
    }
}

// ============================================================================
// IMMUTABLE PROVENANCE SNAPSHOT
// ============================================================================

/// Immutable collection of lifecycle provenance records.
///
/// Records are indexed by stable identity. Snapshot construction verifies
/// all internal causal references and rejects missing parents and cycles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceSnapshot {
    records: BTreeMap<ProvenanceId, ProvenanceRecord>,
}

impl ProvenanceSnapshot {
    /// Creates an empty snapshot.
    #[must_use]
    pub fn new() -> Self {
        Self {
            records: BTreeMap::new(),
        }
    }

    /// Returns the number of records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether the snapshot is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Returns a record by identity.
    #[must_use]
    pub fn get(&self, identifier: &ProvenanceId) -> Option<&ProvenanceRecord> {
        self.records.get(identifier)
    }

    /// Returns all records in identifier order.
    #[must_use]
    pub fn records(&self) -> &BTreeMap<ProvenanceId, ProvenanceRecord> {
        &self.records
    }

    /// Produces deterministic causal ordering.
    ///
    /// Independent records are ordered by their stable identifiers.
    /// Every parent precedes its children.
    ///
    /// This is a deterministic topological ordering, not a claim about
    /// wall-clock event ordering.
    pub fn topological_records(
        &self,
    ) -> Result<Vec<&ProvenanceRecord>, ProvenanceError> {
        let mut indegree: BTreeMap<ProvenanceId, usize> =
            BTreeMap::new();

        let mut children: BTreeMap<
            ProvenanceId,
            BTreeSet<ProvenanceId>,
        > = BTreeMap::new();

        for (identifier, record) in &self.records {
            indegree.insert(identifier.clone(), record.parents.len());

            for parent in &record.parents {
                let parent_id = ProvenanceId::new(parent.as_str())?;

                if !self.records.contains_key(&parent_id) {
                    return Err(ProvenanceError::MissingParent {
                        identifier: Arc::from(parent.as_str()),
                        child: Arc::from(identifier.as_str()),
                    });
                }

                children
                    .entry(parent_id)
                    .or_default()
                    .insert(identifier.clone());
            }
        }

        let mut ready: BTreeSet<ProvenanceId> = indegree
            .iter()
            .filter_map(|(identifier, count)| {
                (*count == 0).then_some(identifier.clone())
            })
            .collect();

        let mut ordered = Vec::with_capacity(self.records.len());

        while let Some(identifier) = ready.pop_first() {
            if let Some(record) = self.records.get(&identifier) {
                ordered.push(record);
            }

            if let Some(dependents) = children.get(&identifier) {
                for child in dependents {
                    let count = indegree
                        .get_mut(child)
                        .expect("every child must have an indegree");

                    *count -= 1;

                    if *count == 0 {
                        ready.insert(child.clone());
                    }
                }
            }
        }

        if ordered.len() != self.records.len() {
            return Err(ProvenanceError::CausalCycle {
                remaining_records: self.records.len() - ordered.len(),
            });
        }

        Ok(ordered)
    }
}

impl Default for ProvenanceSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SNAPSHOT BUILDER
// ============================================================================

/// Builder for validated immutable lifecycle provenance snapshots.
#[derive(Debug, Default)]
pub struct ProvenanceSnapshotBuilder {
    records: BTreeMap<ProvenanceId, ProvenanceRecord>,
}

impl ProvenanceSnapshotBuilder {
    /// Creates an empty snapshot builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a record.
    ///
    /// Duplicate record identities are rejected. Existing records are
    /// never silently overwritten.
    pub fn insert(
        &mut self,
        record: ProvenanceRecord,
    ) -> Result<(), ProvenanceError> {
        let identifier = record.identifier.clone();

        if self.records.contains_key(&identifier) {
            return Err(ProvenanceError::DuplicateRecord {
                identifier: Arc::from(identifier.as_str()),
            });
        }

        self.records.insert(identifier, record);

        Ok(())
    }

    /// Builds and validates an immutable snapshot.
    pub fn build(self) -> Result<ProvenanceSnapshot, ProvenanceError> {
        let snapshot = ProvenanceSnapshot {
            records: self.records,
        };

        snapshot.topological_records()?;

        Ok(snapshot)
    }
}

// ============================================================================
// CANONICAL FINGERPRINTING
// ============================================================================

fn encode_count(
    builder: &mut StableHashBuilder,
    field: &str,
    count: usize,
) -> Result<(), ProvenanceError> {
    let count = u64::try_from(count).map_err(|_| {
        ProvenanceError::CountOverflow {
            collection: "fingerprint",
        }
    })?;

    builder.field_u64(field, count)?;

    Ok(())
}

fn encode_optional_string(
    builder: &mut StableHashBuilder,
    field: &str,
    value: Option<&str>,
) -> Result<(), ProvenanceError> {
    builder.field_bool(
        &format!("{field}.present"),
        value.is_some(),
    )?;

    if let Some(value) = value {
        builder.field_str(field, value)?;
    }

    Ok(())
}

fn encode_digest(
    builder: &mut StableHashBuilder,
    field: &str,
    digest: Option<&ContentDigest>,
) -> Result<(), ProvenanceError> {
    builder.field_bool(
        &format!("{field}.present"),
        digest.is_some(),
    )?;

    if let Some(digest) = digest {
        builder.field_str(
            &format!("{field}.algorithm"),
            digest.algorithm(),
        )?;

        builder.field_str(
            &format!("{field}.value"),
            digest.value(),
        )?;
    }

    Ok(())
}

fn encode_artifact(
    builder: &mut StableHashBuilder,
    artifact: &ProvenanceArtifact,
) -> Result<(), ProvenanceError> {
    builder.field_str(
        "artifact.identifier",
        artifact.identifier().as_str(),
    )?;

    builder.field_str(
        "artifact.kind",
        artifact.kind(),
    )?;

    encode_optional_string(
        builder,
        "artifact.format",
        artifact.format(),
    )?;

    encode_optional_string(
        builder,
        "artifact.target",
        artifact.target(),
    )?;

    encode_digest(
        builder,
        "artifact.digest",
        artifact.digest(),
    )?;

    Ok(())
}

fn encode_evidence(
    builder: &mut StableHashBuilder,
    evidence: &ProvenanceEvidence,
) -> Result<(), ProvenanceError> {
    builder.field_str(
        "evidence.identifier",
        evidence.identifier().as_str(),
    )?;

    builder.field_str(
        "evidence.provider",
        evidence.provider().as_str(),
    )?;

    match evidence.status() {
        EvidenceStatus::Claimed => {
            builder.field_str("evidence.status", "claimed")?;
        }

        EvidenceStatus::Observed => {
            builder.field_str("evidence.status", "observed")?;
        }

        EvidenceStatus::Verified => {
            builder.field_str("evidence.status", "verified")?;
        }

        EvidenceStatus::Rejected => {
            builder.field_str("evidence.status", "rejected")?;
        }

        EvidenceStatus::Indeterminate => {
            builder.field_str("evidence.status", "indeterminate")?;
        }

        EvidenceStatus::Custom(value) => {
            builder.field_str("evidence.status", "custom")?;
            builder.field_str("evidence.status.custom", value)?;
        }
    }

    encode_optional_string(
        builder,
        "evidence.verifier",
        evidence.verifier().map(ProvenanceActorId::as_str),
    )?;

    encode_digest(
        builder,
        "evidence.digest",
        evidence.digest(),
    )?;

    Ok(())
}

fn encode_outcome(
    builder: &mut StableHashBuilder,
    outcome: &ProvenanceOutcome,
) -> Result<(), ProvenanceError> {
    match outcome {
        ProvenanceOutcome::Succeeded => {
            builder.field_str("record.outcome", "succeeded")?;
        }

        ProvenanceOutcome::Rejected => {
            builder.field_str("record.outcome", "rejected")?;
        }

        ProvenanceOutcome::Failed => {
            builder.field_str("record.outcome", "failed")?;
        }

        ProvenanceOutcome::Skipped => {
            builder.field_str("record.outcome", "skipped")?;
        }

        ProvenanceOutcome::Incomplete => {
            builder.field_str("record.outcome", "incomplete")?;
        }

        ProvenanceOutcome::Custom(value) => {
            builder.field_str("record.outcome", "custom")?;
            builder.field_str("record.outcome.custom", value)?;
        }
    }

    Ok(())
}

fn encode_record(
    builder: &mut StableHashBuilder,
    record: &ProvenanceRecord,
) -> Result<(), ProvenanceError> {
    builder.field_str(
        "record.identifier",
        record.identifier().as_str(),
    )?;

    builder.field_str(
        "record.stage",
        record.stage().as_str(),
    )?;

    builder.field_str(
        "record.subject",
        record.subject().as_str(),
    )?;

    builder.field_str(
        "record.actor",
        record.actor().as_str(),
    )?;

    // Parent relationships.

    encode_count(
        builder,
        "record.parents.count",
        record.parents.len(),
    )?;

    for parent in &record.parents {
        builder.field_str(
            "record.parent",
            parent.as_str(),
        )?;
    }

    // Source lineage.

    encode_count(
        builder,
        "record.sources.count",
        record.sources.len(),
    )?;

    for source in &record.sources {
        builder.field_str(
            "record.source",
            source.as_str(),
        )?;
    }

    // Input artifacts.

    encode_count(
        builder,
        "record.inputs.count",
        record.inputs.len(),
    )?;

    for artifact in &record.inputs {
        encode_artifact(builder, artifact)?;
    }

    // Output artifacts.

    encode_count(
        builder,
        "record.outputs.count",
        record.outputs.len(),
    )?;

    for artifact in &record.outputs {
        encode_artifact(builder, artifact)?;
    }

    // Evidence references.

    encode_count(
        builder,
        "record.evidence.count",
        record.evidence.len(),
    )?;

    for evidence in &record.evidence {
        encode_evidence(builder, evidence)?;
    }

    // Additional properties.

    encode_count(
        builder,
        "record.properties.count",
        record.properties.len(),
    )?;

    for (key, value) in &record.properties {
        builder.field_str("record.property.key", key)?;
        builder.field_str("record.property.value", value)?;
    }

    // External compiler provenance.

    builder.field_bool(
        "record.compiler-fingerprint.present",
        record.external_compiler_fingerprint.is_some(),
    )?;

    if let Some(fingerprint) = record.external_compiler_fingerprint {
        builder.field_fingerprint(
            "record.compiler-fingerprint",
            fingerprint,
        )?;
    }

    encode_outcome(builder, &record.outcome)?;

    Ok(())
}

/// Calculate a deterministic fingerprint for an immutable provenance
/// snapshot.
///
/// The authoritative ZUIR stable hashing implementation is used.
/// This function does not implement cryptography independently.
///
/// The fingerprint includes the complete canonical record sequence
/// and causal relationships. It excludes wall-clock time and incidental
/// insertion order.
///
/// A fingerprint is a content identifier, not proof of authenticity,
/// authorization or correctness.
pub fn fingerprint_snapshot(
    snapshot: &ProvenanceSnapshot,
) -> Result<Fingerprint, ProvenanceError> {
    let records = snapshot.topological_records()?;

    let mut builder = StableHashBuilder::new(
        PROVENANCE_FINGERPRINT_DOMAIN,
    )?;

    builder.field_u32(
        "provenance.schema.version",
        PROVENANCE_FINGERPRINT_SCHEMA_VERSION,
    )?;

    encode_count(
        &mut builder,
        "provenance.records.count",
        records.len(),
    )?;

    for record in records {
        encode_record(&mut builder, record)?;
    }

    Ok(builder.finish())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> ProvenanceId {
        ProvenanceId::new(value).expect("valid identifier")
    }

    fn reference(value: &str) -> ProvenanceReference {
        ProvenanceReference::new(value).expect("valid reference")
    }

    fn record(
        identifier: &str,
        parent: Option<&str>,
        stage: &str,
    ) -> ProvenanceRecord {
        let mut builder = ProvenanceRecordBuilder::new(
            id(identifier),
            ProvenanceStageId::new(stage).expect("valid stage"),
            ProvenanceSubjectId::new(
                format!("subject.{identifier}"),
            )
            .expect("valid subject"),
            ProvenanceActorId::new("zamani.toolchain")
                .expect("valid actor"),
        );

        if let Some(parent) = parent {
            builder = builder
                .with_parent(reference(parent))
                .expect("valid parent");
        }

        builder
            .with_outcome(ProvenanceOutcome::Succeeded)
            .build()
    }

    fn snapshot(
        records: Vec<ProvenanceRecord>,
    ) -> ProvenanceSnapshot {
        let mut builder = ProvenanceSnapshotBuilder::new();

        for record in records {
            builder.insert(record).expect("unique record");
        }

        builder.build().expect("valid snapshot")
    }

    #[test]
    fn api_version_is_available() {
        assert_eq!(api_version(), 1);
    }

    #[test]
    fn identifiers_reject_empty_values() {
        assert!(ProvenanceId::new("").is_err());
        assert!(ProvenanceStageId::new(" ").is_err());
        assert!(ProvenanceActorId::new("").is_err());
    }

    #[test]
    fn identifiers_reject_control_characters() {
        assert!(ProvenanceId::new("invalid\nid").is_err());
    }

    #[test]
    fn open_world_stage_identifiers_are_supported() {
        let stage = ProvenanceStageId::new(
            "future.computational.domain.stage",
        )
        .expect("future stage");

        assert_eq!(
            stage.as_str(),
            "future.computational.domain.stage",
        );
    }

    #[test]
    fn self_parent_relationship_is_rejected() {
        let builder = ProvenanceRecordBuilder::new(
            id("record.a"),
            ProvenanceStageId::new("build").unwrap(),
            ProvenanceSubjectId::new("subject.a").unwrap(),
            ProvenanceActorId::new("actor.a").unwrap(),
        );

        assert!(
            builder.with_parent(reference("record.a")).is_err()
        );
    }

    #[test]
    fn duplicate_properties_are_rejected() {
        let builder = ProvenanceRecordBuilder::new(
            id("record.a"),
            ProvenanceStageId::new("build").unwrap(),
            ProvenanceSubjectId::new("subject.a").unwrap(),
            ProvenanceActorId::new("actor.a").unwrap(),
        );

        let builder = builder
            .with_property(
                ProvenanceProperty::new("build.profile", "portable")
                    .unwrap(),
            )
            .unwrap();

        let result = builder.with_property(
            ProvenanceProperty::new("build.profile", "release")
                .unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn duplicate_records_are_rejected() {
        let mut builder = ProvenanceSnapshotBuilder::new();

        builder
            .insert(record("record.a", None, "build"))
            .unwrap();

        assert!(
            builder
                .insert(record("record.a", None, "build"))
                .is_err()
        );
    }

    #[test]
    fn missing_parent_is_rejected() {
        let mut builder = ProvenanceSnapshotBuilder::new();

        builder
            .insert(record("record.child", Some("record.missing"), "build"))
            .unwrap();

        assert!(matches!(
            builder.build(),
            Err(ProvenanceError::MissingParent { .. })
        ));
    }

    #[test]
    fn causal_cycles_are_rejected() {
        let mut builder = ProvenanceSnapshotBuilder::new();

        builder
            .insert(record("record.a", Some("record.b"), "build"))
            .unwrap();

        builder
            .insert(record("record.b", Some("record.a"), "package"))
            .unwrap();

        assert!(matches!(
            builder.build(),
            Err(ProvenanceError::CausalCycle { .. })
        ));
    }

    #[test]
    fn records_are_ordered_independently_of_insertion_order() {
        let first = snapshot(vec![
            record("record.b", Some("record.a"), "package"),
            record("record.a", None, "build"),
        ]);

        let second = snapshot(vec![
            record("record.a", None, "build"),
            record("record.b", Some("record.a"), "package"),
        ]);

        assert_eq!(
            fingerprint_snapshot(&first).unwrap(),
            fingerprint_snapshot(&second).unwrap(),
        );
    }

    #[test]
    fn different_lineage_produces_different_fingerprints() {
        let first = snapshot(vec![
            record("record.a", None, "build"),
            record("record.b", Some("record.a"), "package"),
        ]);

        let second = snapshot(vec![
            record("record.a", None, "build"),
            record("record.b", Some("record.a"), "deployment"),
        ]);

        assert_ne!(
            fingerprint_snapshot(&first).unwrap(),
            fingerprint_snapshot(&second).unwrap(),
        );
    }

    #[test]
    fn empty_snapshot_has_stable_fingerprint() {
        let first = ProvenanceSnapshot::new();
        let second = ProvenanceSnapshot::new();

        assert_eq!(
            fingerprint_snapshot(&first).unwrap(),
            fingerprint_snapshot(&second).unwrap(),
        );
    }

    #[test]
    fn artifact_target_is_preserved() {
        let artifact = ProvenanceArtifact::new(
            ProvenanceSubjectId::new("artifact.output").unwrap(),
            "binary",
        )
        .unwrap()
        .with_target("future.accelerator.target")
        .unwrap();

        assert_eq!(
            artifact.target(),
            Some("future.accelerator.target"),
        );
    }

    #[test]
    fn external_digest_is_preserved_without_recalculation() {
        let digest = ContentDigest::new(
            "sha-256",
            "externally-calculated-digest",
        )
        .unwrap();

        let artifact = ProvenanceArtifact::new(
            ProvenanceSubjectId::new("artifact.output").unwrap(),
            "binary",
        )
        .unwrap()
        .with_digest(digest.clone());

        assert_eq!(artifact.digest(), Some(&digest));
    }

    #[test]
    fn evidence_status_is_not_implicitly_verified() {
        let evidence = ProvenanceEvidence::new(
            reference("verification.report"),
            ProvenanceActorId::new("verification.provider").unwrap(),
            EvidenceStatus::Claimed,
        );

        assert_eq!(
            evidence.status(),
            &EvidenceStatus::Claimed,
        );

        assert!(evidence.verifier().is_none());
    }

    #[test]
    fn custom_outcomes_remain_extensible() {
        let outcome = ProvenanceOutcome::Custom(
            Arc::from("future.provider.outcome"),
        );

        assert!(matches!(
            outcome,
            ProvenanceOutcome::Custom(_)
        ));
    }

    #[test]
    fn large_identifiers_are_not_rejected_by_fixed_limits() {
        let value = "x".repeat(16_384);

        let identifier = ProvenanceId::new(value.clone())
            .expect("identifier should not have arbitrary length limit");

        assert_eq!(identifier.as_str(), value);
    }

    #[test]
    fn causal_order_preserves_parent_before_child() {
        let snapshot = snapshot(vec![
            record("record.c", Some("record.b"), "deployment"),
            record("record.b", Some("record.a"), "package"),
            record("record.a", None, "build"),
        ]);

        let ordered = snapshot.topological_records().unwrap();

        let identifiers: Vec<&str> = ordered
            .iter()
            .map(|record| record.identifier().as_str())
            .collect();

        assert_eq!(
            identifiers,
            vec!["record.a", "record.b", "record.c"],
        );
    }
}
