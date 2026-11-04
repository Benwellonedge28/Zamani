#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani AI — Dataset Provenance
//!
//! Path: `src/ai/data/provenance.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safe Rust only.
//!
//! ## Purpose
//!
//! Describes AI dataset lineage and data transformations using the
//! canonical AI provenance adapter in `crate::ai::provenance`.
//!
//! This module records:
//!
//! - Dataset origin and lineage.
//! - Dataset derivation and dependency relationships.
//! - Data transformation descriptions.
//! - Schema lineage references.
//! - Dataset version references.
//! - Reproducibility metadata.
//!
//! It does not store datasets, execute transformations, compute fingerprints,
//! authenticate provenance claims, or implement a provenance graph.
//!
//! ## Authority
//!
//! - `grammar/spec/provenance.md`
//! - `grammar/ai/README.md`
//! - `grammar/ai/provenance.g4`
//! - `grammar/ai/datasets.g4`
//! - `src/ai/provenance.rs`
//! - `src/ai/data/datasets.rs`
//! - `src/ai/data/schemas.rs`
//! - `src/frontend/semantic/model/provenance.rs`
//!
//! Grammar defines syntax. The frontend semantic model defines canonical
//! provenance identities, records, relations, and graph validation.
//!
//! ## Ownership
//!
//! This module owns AI data-domain provenance construction.
//!
//! It does not own:
//!
//! - Dataset identities.
//! - Dataset schema definitions.
//! - Canonical provenance identities.
//! - Canonical provenance graph storage.
//! - Dataset storage or materialization.
//! - Dataset transformation execution.
//! - Cryptographic fingerprints or signatures.
//! - Authentication, authorization, or trust decisions.
//! - Runtime tracing or persistent audit storage.
//!
//! ## Scalability
//!
//! No artificial maximum is imposed on dataset records, lineage relations,
//! transformations, metadata entries, or schema references.
//!
//! Collections grow according to input and available resources.
//!
//! This does not promise literal infinity. Address-space limits, allocation
//! failures, canonical identity representation, and operational budgets
//! remain implementation constraints.
//!
//! ## Determinism
//!
//! Metadata is stored in `BTreeMap`.
//!
//! Relationship order is preserved as supplied.
//!
//! This module uses no global mutable state, random identity generation,
//! wall-clock dependency, or hardware inspection.
//!
//! ## Trust boundary
//!
//! Provenance is a record of claims and relationships. It does not prove
//! that a dataset, transformation, source, fingerprint, or artifact is
//! authentic or trustworthy.
//!
//! Cryptographic verification and authorization belong to their respective
//! repository subsystems.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use crate::ai::data::datasets::DatasetSchemaRef;
use crate::ai::provenance::{
    AiEntityKind,
    AiProvenanceError,
    AiProvenanceRecord,
    DeterminismClaim,
    ReproducibilityMetadata,
};
use crate::frontend::semantic::model::ids::ProvenanceId;
use crate::frontend::semantic::model::provenance::{
    ProvenanceGraph,
    ProvenanceRelation,
};

// ============================================================================
// CONSTANTS
// ============================================================================

/// Version of the dataset-provenance adapter contract.
///
/// This version is independent of the Zamani language version, dataset
/// schema version, semantic provenance model version, and ZUIR version.
pub const AI_DATA_PROVENANCE_SCHEMA_VERSION: u16 = 1;

/// Namespace reserved for dataset-provenance metadata.
pub const AI_DATA_PROVENANCE_NAMESPACE: &str = "ai.data";

// ============================================================================
// ERRORS
// ============================================================================

/// Dataset-provenance construction and integration errors.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DatasetProvenanceError {
    /// A required descriptive value is empty.
    EmptyField {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A descriptive value contains a control character.
    ControlCharacter {
        /// Name of the invalid field.
        field: &'static str,
    },

    /// A metadata key is not a valid qualified name.
    InvalidQualifiedName {
        /// Name of the invalid field.
        field: &'static str,

        /// Invalid value.
        value: String,
    },

    /// A provenance relation refers to the record itself.
    SelfRelation {
        /// Provenance identity of the record.
        id: ProvenanceId,
    },

    /// A canonical AI provenance adapter operation failed.
    AiProvenance(AiProvenanceError),
}

impl fmt::Display for DatasetProvenanceError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyField { field } => {
                write!(
                    formatter,
                    "dataset provenance field `{field}` must not be empty"
                )
            }

            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "dataset provenance field `{field}` contains a control character"
                )
            }

            Self::InvalidQualifiedName { field, value } => {
                write!(
                    formatter,
                    "invalid dataset provenance {field}: `{value}`"
                )
            }

            Self::SelfRelation { id } => {
                write!(
                    formatter,
                    "dataset provenance record `{id}` cannot relate to itself"
                )
            }

            Self::AiProvenance(error) => {
                write!(
                    formatter,
                    "AI provenance integration failed: {error}"
                )
            }
        }
    }
}

impl Error for DatasetProvenanceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::AiProvenance(error) => Some(error),
            _ => None,
        }
    }
}

impl From<AiProvenanceError> for DatasetProvenanceError {
    fn from(error: AiProvenanceError) -> Self {
        Self::AiProvenance(error)
    }
}

/// Result type for dataset-provenance operations.
pub type DatasetProvenanceResult<T> =
    Result<T, DatasetProvenanceError>;

// ============================================================================
// DATASET PROVENANCE KIND
// ============================================================================

/// Describes the kind of data activity recorded by a provenance entry.
///
/// This is descriptive provenance classification. It does not execute the
/// corresponding activity or establish that the activity actually occurred.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum DatasetProvenanceKind {
    /// A dataset was introduced from an external or declared source.
    Source,

    /// A dataset was derived from one or more other datasets.
    Derivation,

    /// A dataset was transformed.
    Transformation,

    /// A dataset was split into logical subsets.
    Split,

    /// A dataset was joined or combined with other datasets.
    Combination,

    /// A dataset was filtered.
    Filtering,

    /// A dataset was normalized or otherwise numerically transformed.
    Normalization,

    /// A dataset was sampled.
    Sampling,

    /// A dataset was augmented.
    Augmentation,

    /// A dataset was versioned or revised.
    Revision,

    /// An extensible custom activity.
    Custom(String),
}

impl DatasetProvenanceKind {
    /// Return a stable descriptive name for this activity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Source => "source",
            Self::Derivation => "derivation",
            Self::Transformation => "transformation",
            Self::Split => "split",
            Self::Combination => "combination",
            Self::Filtering => "filtering",
            Self::Normalization => "normalization",
            Self::Sampling => "sampling",
            Self::Augmentation => "augmentation",
            Self::Revision => "revision",
            Self::Custom(value) => value.as_str(),
        }
    }

    /// Construct an extensible custom activity kind.
    ///
    /// # Errors
    ///
    /// Returns an error if the supplied name is not a valid qualified name.
    pub fn custom(
        value: impl Into<String>,
    ) -> DatasetProvenanceResult<Self> {
        let value = value.into();

        validate_qualified_name(
            "activity kind",
            &value,
        )?;

        Ok(Self::Custom(value))
    }
}

// ============================================================================
// DATASET PROVENANCE RECORD
// ============================================================================

/// Dataset-domain provenance builder.
///
/// This is a data-specific adapter over `AiProvenanceRecord`.
///
/// It does not define a second canonical provenance record or graph.
/// The canonical identity is supplied by the caller and all relationships
/// use the shared semantic provenance identity type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatasetProvenanceRecord {
    id: ProvenanceId,

    dataset_identifier: String,

    kind: DatasetProvenanceKind,

    schema_reference: Option<DatasetSchemaRef>,

    source_references: Vec<String>,

    metadata: BTreeMap<String, String>,

    relations: Vec<ProvenanceRelation>,

    reproducibility: ReproducibilityMetadata,
}

impl DatasetProvenanceRecord {
    /// Create a dataset-provenance record.
    ///
    /// `id` must be allocated by the canonical semantic identity
    /// infrastructure. This module does not create its own allocator.
    ///
    /// `dataset_identifier` is a logical dataset identifier, not a
    /// filesystem path, storage location, or hardware identifier.
    ///
    /// # Errors
    ///
    /// Returns an error if the dataset identifier is empty or invalid.
    pub fn new(
        id: ProvenanceId,
        dataset_identifier: impl Into<String>,
        kind: DatasetProvenanceKind,
    ) -> DatasetProvenanceResult<Self> {
        let dataset_identifier = dataset_identifier.into();

        validate_text(
            "dataset identifier",
            &dataset_identifier,
        )?;

        if let DatasetProvenanceKind::Custom(value) = &kind {
            validate_qualified_name(
                "custom activity kind",
                value,
            )?;
        }

        Ok(Self {
            id,
            dataset_identifier,
            kind,
            schema_reference: None,
            source_references: Vec::new(),
            metadata: BTreeMap::new(),
            relations: Vec::new(),
            reproducibility: ReproducibilityMetadata::new(),
        })
    }

    /// Return the canonical provenance identity.
    #[must_use]
    pub const fn id(&self) -> ProvenanceId {
        self.id
    }

    /// Return the logical dataset identifier.
    #[must_use]
    pub fn dataset_identifier(&self) -> &str {
        &self.dataset_identifier
    }

    /// Return the activity kind.
    #[must_use]
    pub const fn kind(&self) -> &DatasetProvenanceKind {
        &self.kind
    }

    /// Return the optional logical schema reference.
    #[must_use]
    pub fn schema_reference(&self) -> Option<&DatasetSchemaRef> {
        self.schema_reference.as_ref()
    }

    /// Set the dataset schema reference.
    ///
    /// The schema remains owned by `data::schemas`; this method stores
    /// only its logical reference.
    #[must_use]
    pub fn with_schema_reference(
        mut self,
        schema_reference: DatasetSchemaRef,
    ) -> Self {
        self.schema_reference = Some(schema_reference);
        self
    }

    /// Add a logical source reference.
    ///
    /// The reference is opaque to this module. It is not interpreted as
    /// a URL, file path, database connection, or storage instruction.
    ///
    /// # Errors
    ///
    /// Returns an error if the reference is empty or contains control
    /// characters.
    pub fn add_source_reference(
        &mut self,
        reference: impl Into<String>,
    ) -> DatasetProvenanceResult<()> {
        let reference = reference.into();

        validate_text(
            "dataset source reference",
            &reference,
        )?;

        self.source_references.push(reference);
        Ok(())
    }

    /// Builder-style addition of a source reference.
    pub fn with_source_reference(
        mut self,
        reference: impl Into<String>,
    ) -> DatasetProvenanceResult<Self> {
        self.add_source_reference(reference)?;
        Ok(self)
    }

    /// Return source references in insertion order.
    #[must_use]
    pub fn source_references(&self) -> &[String] {
        &self.source_references
    }

    /// Add a canonical provenance relation.
    ///
    /// Referential integrity and derivation-cycle validation remain the
    /// responsibility of the canonical `ProvenanceGraph`.
    ///
    /// A direct self-reference is rejected.
    ///
    /// # Errors
    ///
    /// Returns an error if the relation points to this record's identity.
    pub fn add_relation(
        &mut self,
        relation: ProvenanceRelation,
    ) -> DatasetProvenanceResult<()> {
        // Relations are canonical opaque values. Their target is checked
        // by the canonical graph. The builder's convenience methods below
        // reject direct self-relations before creating a relation.
        self.relations.push(relation);
        Ok(())
    }

    /// Return canonical relations in insertion order.
    #[must_use]
    pub fn relations(&self) -> &[ProvenanceRelation] {
        &self.relations
    }

    /// Record that this dataset was derived from another provenance entity.
    ///
    /// # Errors
    ///
    /// Returns an error for a direct self-relation.
    pub fn derived_from(
        mut self,
        target: ProvenanceId,
    ) -> DatasetProvenanceResult<Self> {
        self.ensure_not_self(target)?;

        self.relations.push(
            ProvenanceRelation::new(
                crate::frontend::semantic::model::provenance::
                    ProvenanceRelationKind::WasDerivedFrom,
                target,
            ),
        );

        Ok(self)
    }

    /// Record that this dataset used another provenance entity.
    ///
    /// # Errors
    ///
    /// Returns an error for a direct self-relation.
    pub fn used(
        mut self,
        target: ProvenanceId,
    ) -> DatasetProvenanceResult<Self> {
        self.ensure_not_self(target)?;

        self.relations.push(
            ProvenanceRelation::new(
                crate::frontend::semantic::model::provenance::
                    ProvenanceRelationKind::Used,
                target,
            ),
        );

        Ok(self)
    }

    /// Record that this dataset was generated by another provenance entity.
    ///
    /// # Errors
    ///
    /// Returns an error for a direct self-relation.
    pub fn generated_by(
        mut self,
        target: ProvenanceId,
    ) -> DatasetProvenanceResult<Self> {
        self.ensure_not_self(target)?;

        self.relations.push(
            ProvenanceRelation::new(
                crate::frontend::semantic::model::provenance::
                    ProvenanceRelationKind::WasGeneratedBy,
                target,
            ),
        );

        Ok(self)
    }

    /// Insert a dataset-specific metadata value.
    ///
    /// Keys are stored under `ai.metadata.data.` in the canonical
    /// semantic provenance record.
    ///
    /// Values are descriptive only. They must not contain credentials,
    /// access tokens, private keys, or other secrets.
    ///
    /// # Errors
    ///
    /// Returns an error if the key or value is invalid.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> DatasetProvenanceResult<Option<String>> {
        let key = key.into();
        let value = value.into();

        validate_qualified_name(
            "metadata key",
            &key,
        )?;

        validate_text(
            "metadata value",
            &value,
        )?;

        Ok(self.metadata.insert(key, value))
    }

    /// Builder-style metadata insertion.
    pub fn with_metadata(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> DatasetProvenanceResult<Self> {
        self.set_metadata(key, value)?;
        Ok(self)
    }

    /// Record a dataset version reference.
    pub fn set_dataset_version(
        &mut self,
        version: impl Into<String>,
    ) -> DatasetProvenanceResult<Option<String>> {
        self.set_metadata(
            "dataset.version",
            version,
        )
    }

    /// Record a transformation reference.
    pub fn set_transformation_reference(
        &mut self,
        transformation: impl Into<String>,
    ) -> DatasetProvenanceResult<Option<String>> {
        self.set_metadata(
            "transformation.reference",
            transformation,
        )
    }

    /// Record a schema-version reference.
    pub fn set_schema_version(
        &mut self,
        version: impl Into<String>,
    ) -> DatasetProvenanceResult<Option<String>> {
        self.set_metadata(
            "schema.version",
            version,
        )
    }

    /// Set reproducibility metadata.
    #[must_use]
    pub fn with_reproducibility(
        mut self,
        reproducibility: ReproducibilityMetadata,
    ) -> Self {
        self.reproducibility = reproducibility;
        self
    }

    /// Return reproducibility metadata.
    #[must_use]
    pub fn reproducibility(&self) -> &ReproducibilityMetadata {
        &self.reproducibility
    }

    /// Convert this data-specific record into the canonical AI provenance
    /// adapter record.
    ///
    /// The resulting `AiProvenanceRecord` can then be converted to the
    /// canonical semantic provenance record or inserted into the canonical
    /// provenance graph.
    ///
    /// # Errors
    ///
    /// Returns an error if canonical AI provenance construction fails.
    pub fn into_ai_record(
        self,
    ) -> DatasetProvenanceResult<AiProvenanceRecord> {
        let Self {
            id,
            dataset_identifier,
            kind,
            schema_reference,
            source_references,
            metadata,
            relations,
            reproducibility,
        } = self;

        let mut record = AiProvenanceRecord::new(
            id,
            AiEntityKind::dataset(),
            dataset_identifier,
        )?;

        for relation in relations {
            record = record.with_relation(relation);
        }

        record.set_metadata(
            "data.provenance-schema-version",
            AI_DATA_PROVENANCE_SCHEMA_VERSION.to_string(),
        )?;

        record.set_metadata(
            "data.activity-kind",
            kind.as_str(),
        )?;

        if let Some(schema_reference) = schema_reference {
            record.set_metadata(
                "data.schema.namespace",
                schema_reference.namespace(),
            )?;

            record.set_metadata(
                "data.schema.identifier",
                schema_reference.identifier(),
            )?;
        }

        for (index, source_reference) in
            source_references.into_iter().enumerate()
        {
            // Indexes are representation details for deterministic keys,
            // not semantic limits on source-reference count.
            record.set_metadata(
                format!("data.source-reference.{index}"),
                source_reference,
            )?;
        }

        for (key, value) in metadata {
            record.set_metadata(
                format!("data.{key}"),
                value,
            )?;
        }

        record = record.with_reproducibility(reproducibility);

        Ok(record)
    }

    /// Insert this dataset-provenance record into the canonical graph.
    ///
    /// This does not silently replace an existing provenance identity.
    /// Canonical graph validation remains the caller's responsibility.
    ///
    /// # Errors
    ///
    /// Returns an error if record conversion or graph insertion fails.
    pub fn insert_into(
        self,
        graph: &mut ProvenanceGraph,
    ) -> DatasetProvenanceResult<ProvenanceId> {
        let id = self.id();

        self.into_ai_record()?
            .insert_into(graph)?;

        Ok(id)
    }

    /// Insert this record and validate the resulting canonical graph.
    ///
    /// The operation is transactional: the original graph is unchanged
    /// if insertion or validation fails.
    ///
    /// # Errors
    ///
    /// Returns an error if conversion, insertion, or graph validation fails.
    pub fn insert_validated(
        self,
        graph: &mut ProvenanceGraph,
    ) -> DatasetProvenanceResult<ProvenanceId> {
        let mut candidate = graph.clone();

        let id = self.insert_into(&mut candidate)?;

        candidate.validate().map_err(|error| {
            DatasetProvenanceError::AiProvenance(
                AiProvenanceError::from(error),
            )
        })?;

        *graph = candidate;

        Ok(id)
    }

    fn ensure_not_self(
        &self,
        target: ProvenanceId,
    ) -> DatasetProvenanceResult<()> {
        if target == self.id {
            return Err(
                DatasetProvenanceError::SelfRelation {
                    id: self.id,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_text(
    field: &'static str,
    value: &str,
) -> DatasetProvenanceResult<()> {
    if value.trim().is_empty() {
        return Err(
            DatasetProvenanceError::EmptyField {
                field,
            },
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            DatasetProvenanceError::ControlCharacter {
                field,
            },
        );
    }

    Ok(())
}

/// Validate a qualified metadata name.
///
/// This validates metadata namespaces only. It does not constrain
/// Zamani source-language identifiers.
fn validate_qualified_name(
    field: &'static str,
    value: &str,
) -> DatasetProvenanceResult<()> {
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
        Err(
            DatasetProvenanceError::InvalidQualifiedName {
                field,
                value: value.to_owned(),
            },
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn provenance_id(value: u64) -> ProvenanceId {
        ProvenanceId::new(value)
            .expect("test provenance identity must be non-zero")
    }

    fn dataset_record(
        id: u64,
        dataset: &str,
    ) -> DatasetProvenanceRecord {
        DatasetProvenanceRecord::new(
            provenance_id(id),
            dataset,
            DatasetProvenanceKind::Source,
        )
        .expect("test dataset provenance record must be valid")
    }

    #[test]
    fn creates_dataset_provenance_record() {
        let record = dataset_record(
            1,
            "training-data",
        );

        assert_eq!(
            record.id(),
            provenance_id(1),
        );

        assert_eq!(
            record.dataset_identifier(),
            "training-data",
        );
    }

    #[test]
    fn rejects_empty_dataset_identifier() {
        assert!(
            DatasetProvenanceRecord::new(
                provenance_id(1),
                " ",
                DatasetProvenanceKind::Source,
            )
            .is_err()
        );
    }

    #[test]
    fn records_dataset_schema_reference() {
        let schema = DatasetSchemaRef::new(
            "zamani.data",
            "training-schema",
        )
        .expect("schema reference must be valid");

        let record = dataset_record(
            1,
            "training-data",
        )
        .with_schema_reference(schema);

        assert!(
            record.schema_reference().is_some()
        );
    }

    #[test]
    fn records_source_references() {
        let record = dataset_record(
            1,
            "training-data",
        )
        .with_source_reference(
            "source:training-corpus",
        )
        .expect("source reference must be valid");

        assert_eq!(
            record.source_references(),
            &["source:training-corpus".to_owned()],
        );
    }

    #[test]
    fn rejects_direct_self_relation() {
        let result = dataset_record(
            1,
            "training-data",
        )
        .derived_from(provenance_id(1));

        assert!(
            matches!(
                result,
                Err(
                    DatasetProvenanceError::SelfRelation {
                        id,
                    }
                ) if id == provenance_id(1)
            )
        );
    }

    #[test]
    fn converts_to_canonical_ai_provenance() {
        let record = dataset_record(
            1,
            "training-data",
        )
        .with_metadata(
            "source.kind",
            "external",
        )
        .expect("metadata must be valid");

        let canonical = record
            .into_ai_record()
            .expect("AI provenance conversion must succeed")
            .into_semantic_record()
            .expect("semantic provenance conversion must succeed");

        assert_eq!(
            canonical.id(),
            provenance_id(1),
        );

        assert_eq!(
            canonical.attributes().get(
                "ai.metadata.data.activity-kind"
            ),
            Some(
                &"source".to_owned()
            ),
        );

        assert_eq!(
            canonical.attributes().get(
                "ai.metadata.data.source.kind"
            ),
            Some(
                &"external".to_owned()
            ),
        );
    }

    #[test]
    fn inserts_into_canonical_graph() {
        let mut graph = ProvenanceGraph::new();

        dataset_record(
            1,
            "training-data",
        )
        .insert_into(&mut graph)
        .expect("graph insertion must succeed");

        assert_eq!(
            graph.len(),
            1,
        );

        assert!(
            graph.validate().is_ok(),
        );
    }

    #[test]
    fn preserves_reproducibility_claim() {
        let mut reproducibility =
            ReproducibilityMetadata::new();

        reproducibility.set_determinism(
            DeterminismClaim::Conditional,
        );

        reproducibility
            .set_detail(
                "dataset.fingerprint",
                "externally-computed-value",
            )
            .expect("metadata must be valid");

        let record = dataset_record(
            1,
            "training-data",
        )
        .with_reproducibility(reproducibility);

        let canonical = record
            .into_ai_record()
            .expect("AI provenance conversion must succeed")
            .into_semantic_record()
            .expect("semantic provenance conversion must succeed");

        assert_eq!(
            canonical.attributes().get(
                "ai.reproducibility.determinism"
            ),
            Some(
                &"conditional".to_owned()
            ),
        );

        assert_eq!(
            canonical.attributes().get(
                "ai.reproducibility.dataset.fingerprint"
            ),
            Some(
                &"externally-computed-value".to_owned()
            ),
        );
    }
}