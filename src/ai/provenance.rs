
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani AI Provenance
//!
//! Path: `src/ai/provenance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide AI-domain provenance construction and integration using the
//! canonical frontend semantic provenance model.
//!
//! AI provenance describes the origin, lineage, transformation history,
//! training context, dataset dependencies, operation relationships,
//! artifact history, and reproducibility information associated with
//! AI computational entities.
//!
//! ## AUTHORITY
//!
//! - grammar/spec/provenance.md
//! - grammar/spec/ai.md
//! - grammar/ai/README.md
//! - grammar/ai/provenance.g4
//! - grammar/expressions/provenance.g4
//! - src/frontend/semantic/model/provenance.rs
//! - src/frontend/semantic/model/ids.rs
//! - src/frontend/semantic/model/program.rs
//! - src/zuir/source_map.rs
//! - src/zuir/schema/metadata.rs
//!
//! ## OWNS
//!
//! - AI-specific provenance entity classification.
//! - AI provenance record construction.
//! - AI provenance metadata conventions.
//! - AI reproducibility metadata.
//! - AI-to-semantic-provenance adaptation.
//! - AI-specific metadata validation.
//!
//! ## DOES NOT OWN
//!
//! - Universal provenance identities.
//! - Universal provenance records.
//! - Universal provenance graph storage.
//! - AST node identities.
//! - Source spans.
//! - Semantic identity allocation.
//! - Cryptographic hashing or signatures.
//! - Authentication or authorization.
//! - Persistent provenance databases.
//! - Runtime tracing.
//! - Model execution.
//! - Training execution.
//! - Dataset storage.
//! - Hardware discovery.
//! - Resource allocation.
//! - Scheduling.
//! - AI intermediate representations.
//! - ZUIR or domain-specific IR implementations.
//!
//! ## INTEGRATION
//!
//! AI records are converted into canonical
//! `frontend::semantic::model::provenance::ProvenanceRecord` values.
//!
//! Canonical identities use `ProvenanceId`.
//!
//! Canonical artifact references use `ArtifactReference`.
//!
//! Canonical relationships use `ProvenanceRelation`.
//!
//! Canonical graph storage uses `ProvenanceGraph`.
//!
//! Source origins use `ProvenanceOrigin` and the canonical AST Span.
//!
//! ZUIR and downstream domain adapters consume the established semantic
//! provenance rather than creating another AI provenance representation.
//!
//! ## SCALABILITY
//!
//! No artificial maximum is imposed on:
//!
//! - AI models;
//! - model parameters;
//! - datasets;
//! - training runs;
//! - operations;
//! - transformations;
//! - agents;
//! - artifacts;
//! - lineage relationships;
//! - metadata entries.
//!
//! Dynamic collections grow according to actual input and available
//! resources.
//!
//! Rust allocation failures, address-space limits, and the canonical
//! semantic identity representation remain implementation constraints.
//!
//! This module does not promise literal infinity.
//!
//! Operational budgets belong to the caller and must remain distinct
//! from language semantics.
//!
//! ## DETERMINISM
//!
//! AI metadata uses BTreeMap.
//!
//! The canonical provenance graph owns deterministic record ordering.
//!
//! No global mutable state, random identity generation, wall-clock
//! dependency, or hardware inspection is introduced.
//!
//! ## TRUST
//!
//! Provenance metadata records claims and relationships.
//!
//! It does not establish that an external model, dataset, signature,
//! training record, or artifact is authentic or trustworthy.
//!
//! Cryptographic verification and security enforcement remain with
//! their respective repository subsystems.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::frontend::semantic::model::ids::ProvenanceId;

use crate::frontend::semantic::model::provenance::{
    ArtifactReference,
    GeneratedOrigin,
    ProvenanceError,
    ProvenanceGraph,
    ProvenanceOrigin,
    ProvenanceRecord,
    ProvenanceRelation,
    ProvenanceRelationKind,
    ProvenanceSubject,
};

/// AI provenance adapter schema version.
///
/// Independent of the Zamani language version, AI API version,
/// semantic provenance schema version, and ZUIR schema version.
pub const AI_PROVENANCE_SCHEMA_VERSION: u16 = 1;

/// Namespace for AI-specific provenance metadata.
pub const AI_PROVENANCE_NAMESPACE: &str = "ai";

/// AI provenance entity classification.
///
/// The namespace is open-ended. New AI computational entities can
/// be introduced without modifying a closed enumeration.
///
/// Examples:
///
/// - ai.model
/// - ai.dataset
/// - ai.training-run
/// - ai.operation
/// - ai.pipeline
/// - ai.agent
/// - ai.checkpoint
/// - ai.deployment
/// - ai.custom.vendor.extension
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AiEntityKind {
    namespace: String,
}

impl AiEntityKind {
    /// Construct an AI entity kind.
    ///
    /// The namespace must be qualified and begin with `ai.`.
    pub fn new(namespace: impl Into<String>) -> Result<Self, AiProvenanceError> {
        let namespace = namespace.into();

        validate_qualified_name(
            "AI entity namespace",
            &namespace,
        )?;

        if !namespace.starts_with("ai.") {
            return Err(AiProvenanceError::InvalidEntityNamespace {
                value: namespace,
            });
        }

        Ok(Self { namespace })
    }

    /// AI model entity.
    #[must_use]
    pub fn model() -> Self {
        Self {
            namespace: "ai.model".to_owned(),
        }
    }

    /// AI dataset entity.
    #[must_use]
    pub fn dataset() -> Self {
        Self {
            namespace: "ai.dataset".to_owned(),
        }
    }

    /// AI training-run entity.
    #[must_use]
    pub fn training_run() -> Self {
        Self {
            namespace: "ai.training-run".to_owned(),
        }
    }

    /// AI operation entity.
    #[must_use]
    pub fn operation() -> Self {
        Self {
            namespace: "ai.operation".to_owned(),
        }
    }

    /// AI pipeline entity.
    #[must_use]
    pub fn pipeline() -> Self {
        Self {
            namespace: "ai.pipeline".to_owned(),
        }
    }

    /// AI agent entity.
    #[must_use]
    pub fn agent() -> Self {
        Self {
            namespace: "ai.agent".to_owned(),
        }
    }

    /// AI checkpoint entity.
    #[must_use]
    pub fn checkpoint() -> Self {
        Self {
            namespace: "ai.checkpoint".to_owned(),
        }
    }

    /// AI deployment artifact entity.
    #[must_use]
    pub fn deployment() -> Self {
        Self {
            namespace: "ai.deployment".to_owned(),
        }
    }

    /// Return the canonical entity namespace.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.namespace
    }
}

/// Classification of a reproducibility claim.
///
/// These are descriptive declarations, not cryptographic proof that
/// a computation is reproducible.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum DeterminismClaim {
    /// Determinism has not been established.
    #[default]
    Unknown,

    /// The implementation declares deterministic behavior.
    Deterministic,

    /// Determinism is conditional on declared inputs and execution
    /// conditions.
    Conditional,

    /// Reproducibility is best-effort.
    BestEffort,

    /// The computation is explicitly nondeterministic.
    Nondeterministic,
}

impl DeterminismClaim {
    /// Return the stable metadata representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Deterministic => "deterministic",
            Self::Conditional => "conditional",
            Self::BestEffort => "best-effort",
            Self::Nondeterministic => "nondeterministic",
        }
    }
}

/// Reproducibility information associated with an AI computation.
///
/// Values are supplied by the authoritative subsystem responsible
/// for the corresponding information.
///
/// This module does not calculate fingerprints, inspect hardware,
/// generate random seeds, or claim reproducibility without evidence.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReproducibilityMetadata {
    determinism: DeterminismClaim,

    details: BTreeMap<String, String>,
}

impl ReproducibilityMetadata {
    /// Create empty reproducibility metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Declare the determinism classification.
    pub fn set_determinism(&mut self, claim: DeterminismClaim) {
        self.determinism = claim;
    }

    /// Return the determinism classification.
    #[must_use]
    pub const fn determinism(&self) -> DeterminismClaim {
        self.determinism
    }

    /// Record an extensible reproducibility property.
    ///
    /// Examples:
    ///
    /// - language.version
    /// - compiler.version
    /// - model.fingerprint
    /// - dataset.fingerprint
    /// - source.revision
    /// - numerical.mode
    /// - random.seed-reference
    /// - operation-set.version
    ///
    /// Fingerprints and seed references are opaque caller-supplied
    /// values. No hashing or random-number generation occurs here.
    pub fn set_detail(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, AiProvenanceError> {
        let key = key.into();
        let value = value.into();

        validate_qualified_name(
            "reproducibility metadata key",
            &key,
        )?;

        validate_text(
            "reproducibility metadata value",
            &value,
        )?;

        Ok(self.details.insert(key, value))
    }

    /// Return reproducibility details in deterministic key order.
    #[must_use]
    pub fn details(&self) -> &BTreeMap<String, String> {
        &self.details
    }

    /// Whether no reproducibility information has been supplied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.determinism == DeterminismClaim::Unknown
            && self.details.is_empty()
    }
}

/// AI-specific provenance record builder.
///
/// This is an adapter around the canonical semantic provenance
/// record, not a competing record or graph implementation.
///
/// Every completed instance can be converted into the canonical
/// semantic provenance representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AiProvenanceRecord {
    id: ProvenanceId,

    entity_kind: AiEntityKind,

    artifact: ArtifactReference,

    origin: ProvenanceOrigin,

    relations: Vec<ProvenanceRelation>,

    metadata: BTreeMap<String, String>,

    reproducibility: ReproducibilityMetadata,
}

impl AiProvenanceRecord {
    /// Create an AI provenance record.
    ///
    /// The identity is allocated by the canonical semantic identity
    /// infrastructure and supplied by the caller.
    ///
    /// No global allocator or duplicate AI identity system is created.
    pub fn new(
        id: ProvenanceId,
        entity_kind: AiEntityKind,
        artifact_identifier: impl Into<String>,
    ) -> Result<Self, AiProvenanceError> {
        let artifact = ArtifactReference::new(
            entity_kind.as_str(),
            artifact_identifier,
        )?;

        Ok(Self {
            id,
            entity_kind,
            artifact,
            origin: ProvenanceOrigin::Unknown,
            relations: Vec::new(),
            metadata: BTreeMap::new(),
            reproducibility: ReproducibilityMetadata::new(),
        })
    }

    /// Return the canonical provenance identity.
    #[must_use]
    pub const fn id(&self) -> ProvenanceId {
        self.id
    }

    /// Return the AI entity classification.
    #[must_use]
    pub fn entity_kind(&self) -> &AiEntityKind {
        &self.entity_kind
    }

    /// Return the canonical artifact reference.
    #[must_use]
    pub fn artifact(&self) -> &ArtifactReference {
        &self.artifact
    }

    /// Set the canonical provenance origin.
    ///
    /// This accepts the universal source, generated, and external
    /// origin representations.
    #[must_use]
    pub fn with_origin(mut self, origin: ProvenanceOrigin) -> Self {
        self.origin = origin;
        self
    }

    /// Set a source origin.
    ///
    /// The supplied origin must use the canonical AST NodeId and Span.
    #[must_use]
    pub fn with_source_origin(
        self,
        node_id: crate::frontend::ast::node::node_id::NodeId,
        span: crate::frontend::ast::node::source::Span,
    ) -> Self {
        use crate::frontend::semantic::model::provenance::SourceOrigin;

        self.with_origin(ProvenanceOrigin::Source(
            SourceOrigin::new(node_id, span),
        ))
    }

    /// Set a generated origin.
    pub fn with_generated_origin(
        self,
        activity: impl Into<String>,
        activity_version: Option<String>,
    ) -> Result<Self, AiProvenanceError> {
        let origin = GeneratedOrigin::new(
            activity,
            activity_version,
        )?;

        Ok(self.with_origin(
            ProvenanceOrigin::Generated(origin),
        ))
    }

    /// Append a canonical provenance relationship.
    ///
    /// The target is another canonical ProvenanceId.
    ///
    /// Referential integrity is checked by the universal graph.
    #[must_use]
    pub fn with_relation(
        mut self,
        relation: ProvenanceRelation,
    ) -> Self {
        self.relations.push(relation);
        self
    }

    /// Record derivation from another provenance entity.
    #[must_use]
    pub fn derived_from(
        self,
        target: ProvenanceId,
    ) -> Self {
        self.with_relation(ProvenanceRelation::new(
            ProvenanceRelationKind::WasDerivedFrom,
            target,
        ))
    }

    /// Record use of another provenance entity.
    #[must_use]
    pub fn used(
        self,
        target: ProvenanceId,
    ) -> Self {
        self.with_relation(ProvenanceRelation::new(
            ProvenanceRelationKind::Used,
            target,
        ))
    }

    /// Record generation by another provenance activity.
    #[must_use]
    pub fn generated_by(
        self,
        target: ProvenanceId,
    ) -> Self {
        self.with_relation(ProvenanceRelation::new(
            ProvenanceRelationKind::WasGeneratedBy,
            target,
        ))
    }

    /// Record influence by another provenance entity.
    #[must_use]
    pub fn influenced_by(
        self,
        target: ProvenanceId,
    ) -> Self {
        self.with_relation(ProvenanceRelation::new(
            ProvenanceRelationKind::WasInfluencedBy,
            target,
        ))
    }

    /// Record an extensible AI-specific provenance relationship.
    ///
    /// Example: `ai.trained-from`, `ai.fine-tuned-from`,
    /// `ai.adapted-from`, or `ai.transformed-by`.
    ///
    /// Derivation-cycle validation remains the responsibility
    /// of the canonical provenance graph.
    pub fn with_extension_relation(
        self,
        qualified_name: impl Into<String>,
        target: ProvenanceId,
    ) -> Result<Self, AiProvenanceError> {
        let relation = ProvenanceRelationKind::extension(
            qualified_name,
        )?;

        Ok(self.with_relation(
            ProvenanceRelation::new(relation, target),
        ))
    }

    /// Insert AI-specific descriptive metadata.
    ///
    /// Keys are stored under the reserved `ai.metadata.` namespace.
    ///
    /// Example:
    ///
    /// `model.version` becomes `ai.metadata.model.version`.
    ///
    /// Values must not contain credentials, access tokens, private
    /// keys, or other secrets.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Option<String>, AiProvenanceError> {
        let key = key.into();
        let value = value.into();

        validate_qualified_name(
            "AI metadata key",
            &key,
        )?;

        validate_text(
            "AI metadata value",
            &value,
        )?;

        let canonical_key = format!("ai.metadata.{key}");

        Ok(self.metadata.insert(canonical_key, value))
    }

    /// Builder-style metadata insertion.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<Self, AiProvenanceError> {
        self.set_metadata(key, value)?;
        Ok(self)
    }

    /// Record an AI model version.
    pub fn set_model_version(
        &mut self,
        version: impl Into<String>,
    ) -> Result<Option<String>, AiProvenanceError> {
        self.set_metadata("model.version", version)
    }

    /// Record a dataset version.
    pub fn set_dataset_version(
        &mut self,
        version: impl Into<String>,
    ) -> Result<Option<String>, AiProvenanceError> {
        self.set_metadata("dataset.version", version)
    }

    /// Record a training-run reference.
    ///
    /// Actual training-run identity is still represented through
    /// canonical provenance relationships.
    pub fn set_training_run_reference(
        &mut self,
        reference: impl Into<String>,
    ) -> Result<Option<String>, AiProvenanceError> {
        self.set_metadata("training.run-reference", reference)
    }

    /// Record a transformation description or identifier.
    pub fn set_transformation(
        &mut self,
        transformation: impl Into<String>,
    ) -> Result<Option<String>, AiProvenanceError> {
        self.set_metadata("transformation", transformation)
    }

    /// Record an AI operation identifier.
    pub fn set_operation_reference(
        &mut self,
        operation: impl Into<String>,
    ) -> Result<Option<String>, AiProvenanceError> {
        self.set_metadata("operation.reference", operation)
    }

    /// Record an artifact format identifier.
    pub fn set_artifact_format(
        &mut self,
        format: impl Into<String>,
    ) -> Result<Option<String>, AiProvenanceError> {
        self.set_metadata("artifact.format", format)
    }

    /// Set reproducibility information.
    #[must_use]
    pub fn with_reproducibility(
        mut self,
        reproducibility: ReproducibilityMetadata,
    ) -> Self {
        self.reproducibility = reproducibility;
        self
    }

    /// Return reproducibility information.
    #[must_use]
    pub fn reproducibility(&self) -> &ReproducibilityMetadata {
        &self.reproducibility
    }

    /// Convert this AI record into the canonical semantic record.
    ///
    /// No separate AI provenance graph is created.
    pub fn into_semantic_record(
        self,
    ) -> Result<ProvenanceRecord, AiProvenanceError> {
        let Self {
            id,
            entity_kind,
            artifact,
            origin,
            relations,
            metadata,
            reproducibility,
        } = self;

        let subject = ProvenanceSubject::Artifact(artifact);

        let mut record = ProvenanceRecord::new(id, subject)
            .with_origin(origin)
            .with_relations(relations);

        record.set_attribute(
            "ai.schema-version",
            AI_PROVENANCE_SCHEMA_VERSION.to_string(),
        )?;

        record.set_attribute(
            "ai.entity-kind",
            entity_kind.as_str(),
        )?;

        for (key, value) in metadata {
            record.set_attribute(key, value)?;
        }

        if reproducibility.determinism()
            != DeterminismClaim::Unknown
        {
            record.set_attribute(
                "ai.reproducibility.determinism",
                reproducibility.determinism().as_str(),
            )?;
        }

        for (key, value) in reproducibility.details() {
            record.set_attribute(
                format!("ai.reproducibility.{key}"),
                value,
            )?;
        }

        Ok(record)
    }

    /// Insert the record into the universal provenance graph.
    ///
    /// The graph must be validated before being treated as complete.
    ///
    /// This method does not silently replace an existing record.
    pub fn insert_into(
        self,
        graph: &mut ProvenanceGraph,
    ) -> Result<ProvenanceId, AiProvenanceError> {
        let id = self.id();

        graph.insert(self.into_semantic_record()?)?;

        Ok(id)
    }

    /// Insert the record and validate the resulting graph atomically.
    ///
    /// A failed insertion or graph validation leaves the original
    /// graph unchanged.
    ///
    /// This is useful when the caller requires a fully valid graph
    /// after every insertion.
    pub fn insert_validated(
        self,
        graph: &mut ProvenanceGraph,
    ) -> Result<ProvenanceId, AiProvenanceError> {
        let mut candidate = graph.clone();

        let id = self.insert_into(&mut candidate)?;

        candidate.validate()?;

        *graph = candidate;

        Ok(id)
    }
}

/// AI provenance construction and integration failures.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AiProvenanceError {
    /// Invalid AI entity namespace.
    InvalidEntityNamespace {
        value: String,
    },

    /// Empty metadata or identifier field.
    EmptyField {
        field: &'static str,
    },

    /// A field contains a control character.
    ControlCharacter {
        field: &'static str,
    },

    /// Invalid qualified metadata name.
    InvalidQualifiedName {
        field: &'static str,
        value: String,
    },

    /// Failure from the canonical semantic provenance system.
    Semantic(ProvenanceError),
}

impl fmt::Display for AiProvenanceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidEntityNamespace { value } => {
                write!(
                    formatter,
                    "AI entity namespace must begin with `ai.`: `{value}`"
                )
            }

            Self::EmptyField { field } => {
                write!(
                    formatter,
                    "AI provenance field `{field}` must not be empty"
                )
            }

            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "AI provenance field `{field}` contains a control character"
                )
            }

            Self::InvalidQualifiedName { field, value } => {
                write!(
                    formatter,
                    "invalid AI provenance {field}: `{value}`"
                )
            }

            Self::Semantic(error) => {
                write!(
                    formatter,
                    "canonical semantic provenance failure: {error}"
                )
            }
        }
    }
}

impl Error for AiProvenanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Semantic(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ProvenanceError> for AiProvenanceError {
    fn from(error: ProvenanceError) -> Self {
        Self::Semantic(error)
    }
}

/// Validate a non-empty descriptive text value.
fn validate_text(
    field: &'static str,
    value: &str,
) -> Result<(), AiProvenanceError> {
    if value.trim().is_empty() {
        return Err(AiProvenanceError::EmptyField { field });
    }

    if value.chars().any(char::is_control) {
        return Err(AiProvenanceError::ControlCharacter { field });
    }

    Ok(())
}

/// Validate an open-ended qualified identifier.
///
/// This is a metadata namespace contract, not a restriction on
/// Zamani source-language identifiers.
fn validate_qualified_name(
    field: &'static str,
    value: &str,
) -> Result<(), AiProvenanceError> {
    validate_text(field, value)?;

    let valid = value.split('.').all(|segment| {
        let mut characters = segment.chars();

        match characters.next() {
            Some(first)
                if first.is_ascii_alphabetic() || first == '_' =>
            {
                characters.all(|character| {
                    character.is_ascii_alphanumeric()
                        || character == '_'
                        || character == '-'
                })
            }

            _ => false,
        }
    });

    if valid {
        Ok(())
    } else {
        Err(AiProvenanceError::InvalidQualifiedName {
            field,
            value: value.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provenance_id(value: u64) -> ProvenanceId {
        ProvenanceId::new(value)
            .expect("test provenance ID must be non-zero")
    }

    fn ai_record(
        id: u64,
        kind: AiEntityKind,
        identifier: &str,
    ) -> AiProvenanceRecord {
        AiProvenanceRecord::new(
            provenance_id(id),
            kind,
            identifier,
        )
        .expect("test AI provenance record must be valid")
    }

    #[test]
    fn creates_ai_entity_kinds() {
        assert_eq!(
            AiEntityKind::model().as_str(),
            "ai.model"
        );

        assert_eq!(
            AiEntityKind::dataset().as_str(),
            "ai.dataset"
        );

        assert!(
            AiEntityKind::new(
                "ai.custom.future-computational-domain"
            )
            .is_ok()
        );
    }

    #[test]
    fn rejects_non_ai_entity_namespaces() {
        assert!(
            AiEntityKind::new("classical.tensor").is_err()
        );

        assert!(
            AiEntityKind::new("ai..model").is_err()
        );

        assert!(
            AiEntityKind::new("ai.model.").is_err()
        );
    }

    #[test]
    fn creates_canonical_semantic_records() {
        let record = ai_record(
            1,
            AiEntityKind::model(),
            "model-001",
        );

        let semantic = record
            .into_semantic_record()
            .expect("conversion must succeed");

        assert_eq!(semantic.id(), provenance_id(1));

        assert_eq!(
            semantic.attributes().get("ai.entity-kind"),
            Some(&"ai.model".to_owned())
        );

        assert_eq!(
            semantic.attributes().get("ai.schema-version"),
            Some(&"1".to_owned())
        );
    }

    #[test]
    fn preserves_model_and_dataset_versions() {
        let mut record = ai_record(
            1,
            AiEntityKind::model(),
            "model-001",
        );

        record
            .set_model_version("2.4.0")
            .expect("model version must be accepted");

        record
            .set_dataset_version("dataset-7")
            .expect("dataset version must be accepted");

        let semantic = record
            .into_semantic_record()
            .expect("conversion must succeed");

        assert_eq!(
            semantic.attributes().get(
                "ai.metadata.model.version"
            ),
            Some(&"2.4.0".to_owned())
        );

        assert_eq!(
            semantic.attributes().get(
                "ai.metadata.dataset.version"
            ),
            Some(&"dataset-7".to_owned())
        );
    }

    #[test]
    fn preserves_reproducibility_metadata() {
        let mut reproducibility =
            ReproducibilityMetadata::new();

        reproducibility.set_determinism(
            DeterminismClaim::Conditional,
        );

        reproducibility
            .set_detail(
                "compiler.version",
                "1.97",
            )
            .expect("compiler version must be accepted");

        reproducibility
            .set_detail(
                "model.fingerprint",
                "externally-computed-fingerprint",
            )
            .expect("fingerprint must be accepted");

        let record = ai_record(
            1,
            AiEntityKind::model(),
            "model-001",
        )
        .with_reproducibility(reproducibility);

        let semantic = record
            .into_semantic_record()
            .expect("conversion must succeed");

        assert_eq!(
            semantic.attributes().get(
                "ai.reproducibility.determinism"
            ),
            Some(&"conditional".to_owned())
        );

        assert_eq!(
            semantic.attributes().get(
                "ai.reproducibility.compiler.version"
            ),
            Some(&"1.97".to_owned())
        );
    }

    #[test]
    fn inserts_into_canonical_provenance_graph() {
        let mut graph = ProvenanceGraph::new();

        let dataset = ai_record(
            1,
            AiEntityKind::dataset(),
            "dataset-001",
        );

        dataset
            .insert_into(&mut graph)
            .expect("dataset insertion must succeed");

        let model = ai_record(
            2,
            AiEntityKind::model(),
            "model-001",
        )
        .used(provenance_id(1));

        model
            .insert_into(&mut graph)
            .expect("model insertion must succeed");

        assert_eq!(graph.len(), 2);

        assert!(
            graph.validate().is_ok(),
            "canonical graph must be valid"
        );
    }

    #[test]
    fn preserves_training_and_transformation_lineage() {
        let mut graph = ProvenanceGraph::new();

        ai_record(
            1,
            AiEntityKind::dataset(),
            "training-dataset",
        )
        .insert_into(&mut graph)
        .expect("dataset insertion must succeed");

        ai_record(
            2,
            AiEntityKind::training_run(),
            "training-run-001",
        )
        .used(provenance_id(1))
        .insert_into(&mut graph)
        .expect("training insertion must succeed");

        let mut model = ai_record(
            3,
            AiEntityKind::model(),
            "trained-model",
        )
        .derived_from(provenance_id(2));

        model
            .set_transformation("training")
            .expect("transformation must be accepted");

        model
            .insert_into(&mut graph)
            .expect("model insertion must succeed");

        assert!(graph.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_record_ids() {
        let mut graph = ProvenanceGraph::new();

        ai_record(
            1,
            AiEntityKind::model(),
            "model-a",
        )
        .insert_into(&mut graph)
        .expect("first insertion must succeed");

        let result = ai_record(
            1,
            AiEntityKind::model(),
            "model-b",
        )
        .insert_into(&mut graph);

        assert!(result.is_err());
        assert_eq!(graph.len(), 1);
    }

    #[test]
    fn validated_insertion_is_atomic() {
        let mut graph = ProvenanceGraph::new();

        let result = ai_record(
            2,
            AiEntityKind::model(),
            "model-001",
        )
        .derived_from(provenance_id(99))
        .insert_validated(&mut graph);

        assert!(result.is_err());

        // The failed operation must not mutate the original graph.
        assert!(graph.is_empty());
    }

    #[test]
    fn rejects_invalid_metadata_keys() {
        let mut record = ai_record(
            1,
            AiEntityKind::model(),
            "model-001",
        );

        assert!(
            record.set_metadata(
                "invalid key",
                "value",
            ).is_err()
        );

        assert!(
            record.set_metadata(
                "valid.key",
                "value",
            ).is_ok()
        );
    }

    #[test]
    fn supports_open_ended_extension_relationships() {
        let mut graph = ProvenanceGraph::new();

        ai_record(
            1,
            AiEntityKind::dataset(),
            "dataset-001",
        )
        .insert_into(&mut graph)
        .expect("dataset insertion must succeed");

        ai_record(
            2,
            AiEntityKind::model(),
            "model-001",
        )
        .with_extension_relation(
            "ai.trained-from",
            provenance_id(1),
        )
        .expect("extension relationship must be valid")
        .insert_into(&mut graph)
        .expect("model insertion must succeed");

        assert!(graph.validate().is_ok());
    }

    #[test]
    fn rejects_derivation_cycles() {
        let mut graph = ProvenanceGraph::new();

        ai_record(
            1,
            AiEntityKind::model(),
            "model-a",
        )
        .derived_from(provenance_id(2))
        .insert_into(&mut graph)
        .expect("first insertion must succeed");

        ai_record(
            2,
            AiEntityKind::model(),
            "model-b",
        )
        .derived_from(provenance_id(1))
        .insert_into(&mut graph)
        .expect("second insertion must succeed");

        assert!(graph.validate().is_err());
    }
}
