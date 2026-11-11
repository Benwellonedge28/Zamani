
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Data Provenance
//!
//! Path: `src/stdlib/data/provenance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Provide the standard-library data interface for provenance,
//! data lineage, origin tracking, derivation relationships,
//! transformation attribution, and evidence associations.
//!
//! This module reuses the canonical semantic provenance model.
//! It does not define another provenance identity system,
//! record representation, or graph implementation.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! Canonical provenance:
//!
//! `crate::frontend::semantic::model::provenance`
//!
//! Canonical semantic identities:
//!
//! `crate::frontend::semantic::model::ids`
//!
//! Data buffers:
//!
//! `crate::stdlib::data::buffers`
//!
//! Data storage:
//!
//! `crate::stdlib::data::storage`
//!
//! Data serialization:
//!
//! `crate::stdlib::data::serialization`
//!
//! Distributed ledger compatibility:
//!
//! `crate::stdlib::distributed_ledger::data_provenance`
//!
//! Grammar authority:
//!
//! `grammar/data/provenance.g4`
//!
//! Data grammar composition:
//!
//! `grammar/data/data.g4`
//!
//! ## OWNS
//!
//! - The standard-library data provenance facade.
//! - Data-oriented provenance graph management.
//! - Standard-library error adaptation.
//! - Validated provenance insertion operations.
//! - Provenance graph access and interoperability.
//!
//! ## DOES NOT OWN
//!
//! - Provenance identity allocation.
//! - Semantic provenance record definitions.
//! - Semantic relationship definitions.
//! - AST identities or source spans.
//! - Semantic validation.
//! - Cryptographic verification.
//! - Authentication or authorization.
//! - Trust decisions.
//! - Persistent storage.
//! - Distributed consensus.
//! - Runtime tracing.
//! - Hardware selection or allocation.
//! - Scheduling or execution.
//! - Classical, quantum, HDL, or other IR.
//!
//! ## SCALABILITY
//!
//! No arbitrary universal provenance-record, lineage, evidence,
//! relationship, artifact, or domain-count ceiling is introduced.
//!
//! Storage grows according to the actual provenance information
//! and available resources.
//!
//! The implementation uses the canonical semantic graph's
//! deterministic ordered representation.
//!
//! The graph's validation algorithms are iterative.
//!
//! Atomic validated insertion requires a temporary graph snapshot,
//! so its additional memory consumption is proportional to the
//! existing graph. Callers handling very large graphs can use
//! staged insertion followed by explicit validation instead.
//!
//! Rust address-space limits and allocation failures remain
//! implementation constraints, not language-level limits.
//!
//! ## SECURITY
//!
//! Provenance metadata is descriptive.
//!
//! Recording an evidence reference does not establish that the
//! evidence is authentic, trustworthy, or independently verified.
//!
//! Credentials, private keys, access tokens, and other secrets
//! must never be stored in provenance attributes.
//!
//! ## DETERMINISM
//!
//! Identity allocation belongs to the semantic identity authority.
//!
//! This module introduces no global mutable state, randomness,
//! automatic timestamps, machine inspection, or hidden I/O.
//!
//! Graph iteration follows canonical provenance-ID ordering.
//!
//! ## INTEGRATION
//!
//! All public provenance types are re-exported from the semantic
//! authority. The standard-library facade does not redefine them.
//!
//! Storage and serialization adapters may consume the canonical
//! graph through `as_semantic_graph()` or
//! `into_semantic_graph()`.
//!
//! Distributed-ledger adapters must migrate to the same canonical
//! record and identity types.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No implicit provider selection.
//! No fabricated success.
//! No implicit persistence.
//! No printing or logging.

use core::fmt;

use crate::frontend::semantic::model::provenance::{
    ArtifactReference,
    ExternalOrigin,
    GeneratedOrigin,
    ProvenanceError,
    ProvenanceGraph,
    ProvenanceOrigin,
    ProvenanceRecord,
    ProvenanceRelation,
    ProvenanceRelationKind,
    ProvenanceSubject,
    SourceOrigin,
    PROVENANCE_MODEL_SCHEMA_VERSION,
};

use crate::frontend::semantic::model::ids::ProvenanceId;

use crate::stdlib::error::{
    Error,
    ErrorKind,
    Result,
};

// ============================================================================
// PUBLIC CANONICAL RE-EXPORTS
// ============================================================================

/// Canonical semantic provenance record identity.
pub use crate::frontend::semantic::model::ids::ProvenanceId as DataProvenanceId;

/// Canonical semantic provenance subject.
pub use crate::frontend::semantic::model::provenance::ProvenanceSubject
    as DataProvenanceSubject;

/// Canonical semantic provenance origin.
pub use crate::frontend::semantic::model::provenance::ProvenanceOrigin
    as DataProvenanceOrigin;

/// Canonical semantic provenance record.
pub use crate::frontend::semantic::model::provenance::ProvenanceRecord
    as DataProvenanceRecord;

/// Canonical semantic provenance relationship.
pub use crate::frontend::semantic::model::provenance::ProvenanceRelation
    as DataProvenanceRelation;

/// Canonical semantic provenance relationship kind.
pub use crate::frontend::semantic::model::provenance::ProvenanceRelationKind
    as DataProvenanceRelationKind;

/// Canonical semantic provenance graph.
pub use crate::frontend::semantic::model::provenance::ProvenanceGraph
    as DataProvenanceGraph;

/// Canonical semantic provenance error.
pub use crate::frontend::semantic::model::provenance::ProvenanceError
    as DataProvenanceError;

/// Canonical artifact reference.
pub use crate::frontend::semantic::model::provenance::ArtifactReference
    as DataArtifactReference;

/// Canonical source-origin information.
pub use crate::frontend::semantic::model::provenance::SourceOrigin
    as DataSourceOrigin;

/// Canonical generated-origin information.
pub use crate::frontend::semantic::model::provenance::GeneratedOrigin
    as DataGeneratedOrigin;

/// Canonical external-origin information.
pub use crate::frontend::semantic::model::provenance::ExternalOrigin
    as DataExternalOrigin;

// ============================================================================
// MODULE IDENTITY
// ============================================================================

/// Stable identity of the data provenance module.
pub const DATA_PROVENANCE_MODULE_NAME: &str =
    "zamani.stdlib.data.provenance";

/// Version of the data provenance facade contract.
///
/// This is independent of the semantic provenance model schema,
/// language version, compiler version, and serialized format version.
pub const DATA_PROVENANCE_API_VERSION: u16 = 1;

/// Returns the module's stable identity.
#[must_use]
pub const fn module_name() -> &'static str {
    DATA_PROVENANCE_MODULE_NAME
}

/// Returns the data provenance API version.
#[must_use]
pub const fn api_version() -> u16 {
    DATA_PROVENANCE_API_VERSION
}

/// Returns the canonical semantic provenance schema version.
#[must_use]
pub const fn semantic_schema_version() -> u16 {
    PROVENANCE_MODEL_SCHEMA_VERSION
}

// ============================================================================
// ERROR ADAPTATION
// ============================================================================

/// Converts a canonical semantic provenance error into a structured
/// standard-library error.
///
/// The original semantic error is preserved as the causal source.
///
/// This function does not discard the original error classification.
fn adapt_error(
    error: ProvenanceError,
    operation: &'static str,
) -> Error {
    let message = error.to_string();

    Error::new(ErrorKind::InvalidData, message)
        .in_component(DATA_PROVENANCE_MODULE_NAME)
        .during_operation(operation)
        .with_source(error)
}

// ============================================================================
// DATA PROVENANCE FACADE
// ============================================================================

/// Standard-library interface to the canonical semantic provenance graph.
///
/// This type is an adapter around `ProvenanceGraph`.
///
/// It does not maintain an independent record collection or relationship
/// representation.
///
/// Records and identities remain owned by the semantic provenance model.
///
/// # Construction
///
/// ```
/// use zamani::frontend::semantic::model::ids::ProvenanceId;
/// use zamani::frontend::semantic::model::provenance::{
///     ProvenanceSubject,
/// };
/// use zamani::stdlib::data::provenance::DataProvenance;
///
/// let mut provenance = DataProvenance::new();
///
/// let id = ProvenanceId::new(1).expect("non-zero identity");
///
/// let subject = ProvenanceSubject::artifact(
///     "example.dataset",
///     "source",
/// ).expect("valid artifact reference");
///
/// let record =
///     zamani::frontend::semantic::model::provenance::ProvenanceRecord::new(
///         id,
///         subject,
///     );
///
/// provenance.insert(record).expect("record insertion");
/// provenance.validate().expect("valid graph");
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DataProvenance {
    graph: ProvenanceGraph,
}

impl DataProvenance {
    /// Creates an empty data provenance graph.
    ///
    /// No identity is allocated and no external service is initialized.
    #[must_use]
    pub fn new() -> Self {
        Self {
            graph: ProvenanceGraph::new(),
        }
    }

    /// Creates a facade around an existing canonical graph.
    ///
    /// This constructor does not certify that the supplied graph
    /// is referentially complete.
    ///
    /// Call `validate()` before treating it as complete.
    #[must_use]
    pub fn from_semantic_graph(graph: ProvenanceGraph) -> Self {
        Self { graph }
    }

    /// Creates a facade from an existing graph after validation.
    ///
    /// # Errors
    ///
    /// Returns a structured standard-library error if the graph
    /// contains missing references or derivation cycles.
    pub fn try_from_semantic_graph(
        graph: ProvenanceGraph,
    ) -> Result<Self> {
        graph
            .validate()
            .map_err(|error| adapt_error(error, "graph.import"))?;

        Ok(Self { graph })
    }

    /// Returns the number of provenance records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.graph.len()
    }

    /// Returns whether the graph contains no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    /// Returns whether a provenance identity exists.
    #[must_use]
    pub fn contains(&self, id: ProvenanceId) -> bool {
        self.graph.contains(id)
    }

    /// Retrieves a canonical provenance record.
    #[must_use]
    pub fn get(
        &self,
        id: ProvenanceId,
    ) -> Option<&ProvenanceRecord> {
        self.graph.get(id)
    }

    /// Iterates over provenance records in canonical ID order.
    ///
    /// This does not clone records or allocate a snapshot.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<
        Item = (&ProvenanceId, &ProvenanceRecord),
    > {
        self.graph.iter()
    }

    /// Inserts a record into the graph.
    ///
    /// This operation rejects duplicate identities.
    ///
    /// Forward references are permitted during graph construction.
    /// Consequently, successful insertion does not certify that the
    /// complete graph is valid.
    ///
    /// Call `validate()` after constructing the complete graph.
    ///
    /// # Errors
    ///
    /// Returns an error when the identity already exists.
    pub fn insert(
        &mut self,
        record: ProvenanceRecord,
    ) -> Result<()> {
        self.graph
            .insert(record)
            .map_err(|error| adapt_error(error, "record.insert"))
    }

    /// Inserts a record and validates the complete resulting graph
    /// atomically.
    ///
    /// The existing graph is unchanged if insertion or validation fails.
    ///
    /// This method clones the current graph before attempting the
    /// operation. It is intended for callers who need all-or-nothing
    /// validated updates.
    ///
    /// For very large graphs, staged `insert()` followed by `validate()`
    /// avoids the additional full-graph snapshot.
    ///
    /// # Errors
    ///
    /// Returns an error for duplicate identities, missing references,
    /// or derivation cycles.
    pub fn insert_validated(
        &mut self,
        record: ProvenanceRecord,
    ) -> Result<()> {
        let mut candidate = self.graph.clone();

        candidate
            .insert(record)
            .map_err(|error| adapt_error(error, "record.insert_validated"))?;

        candidate
            .validate()
            .map_err(|error| adapt_error(error, "graph.validate"))?;

        self.graph = candidate;

        Ok(())
    }

    /// Removes a provenance record.
    ///
    /// Removing a record may leave dangling references.
    ///
    /// The graph must be validated again before being used as a
    /// referentially complete graph.
    ///
    /// Returns the removed canonical record, if present.
    pub fn remove(
        &mut self,
        id: ProvenanceId,
    ) -> Option<ProvenanceRecord> {
        self.graph.remove(id)
    }

    /// Validates all provenance relationships and derivation lineage.
    ///
    /// Validation is delegated to the canonical semantic graph.
    ///
    /// # Errors
    ///
    /// Returns an error for missing references or derivation cycles.
    pub fn validate(&self) -> Result<()> {
        self.graph
            .validate()
            .map_err(|error| adapt_error(error, "graph.validate"))
    }

    /// Validates provenance relationship references.
    ///
    /// Every referenced provenance record must exist.
    ///
    /// # Errors
    ///
    /// Returns an error when a relationship references a missing record.
    pub fn validate_references(&self) -> Result<()> {
        self.graph
            .validate_references()
            .map_err(|error| adapt_error(error, "graph.validate_references"))
    }

    /// Validates derivation acyclicity.
    ///
    /// Only relationships classified by the canonical semantic model
    /// as derivation relationships are subject to this check.
    ///
    /// Other relationships are not incorrectly forced to be acyclic.
    ///
    /// # Errors
    ///
    /// Returns an error when a derivation cycle exists or a referenced
    /// record is missing.
    pub fn validate_derivation_acyclic(&self) -> Result<()> {
        self.graph
            .validate_derivation_acyclic()
            .map_err(|error| {
                adapt_error(error, "graph.validate_derivation_acyclic")
            })
    }

    /// Returns an immutable reference to the canonical semantic graph.
    ///
    /// No copying or conversion takes place.
    #[must_use]
    pub fn as_semantic_graph(&self) -> &ProvenanceGraph {
        &self.graph
    }

    /// Consumes the facade and returns the canonical semantic graph.
    ///
    /// No copying or conversion takes place.
    #[must_use]
    pub fn into_semantic_graph(self) -> ProvenanceGraph {
        self.graph
    }

    /// Returns a deterministic owned snapshot of the records.
    ///
    /// Records are ordered by canonical provenance identity.
    ///
    /// The returned vector requires memory proportional to the
    /// number and size of records.
    #[must_use]
    pub fn to_sorted_records(&self) -> Vec<ProvenanceRecord> {
        self.graph.to_sorted_vec()
    }

    /// Removes every record.
    ///
    /// This operation does not delete external artifacts, persisted
    /// evidence, ledger transactions, or source files.
    pub fn clear(&mut self) {
        self.graph = ProvenanceGraph::new();
    }
}

// ============================================================================
// PUBLIC VALIDATION HELPERS
// ============================================================================

/// Validates an existing canonical provenance graph.
///
/// This is useful for consumers that already own a `ProvenanceGraph`
/// and do not need the data-oriented facade.
///
/// # Errors
///
/// Returns a structured standard-library error for invalid graphs.
pub fn validate_graph(
    graph: &ProvenanceGraph,
) -> Result<()> {
    graph
        .validate()
        .map_err(|error| adapt_error(error, "graph.validate"))
}

/// Validates all references in an existing canonical graph.
///
/// # Errors
///
/// Returns an error when a referenced provenance record is absent.
pub fn validate_graph_references(
    graph: &ProvenanceGraph,
) -> Result<()> {
    graph
        .validate_references()
        .map_err(|error| {
            adapt_error(error, "graph.validate_references")
        })
}

// ============================================================================
// STANDARD-LIBRARY INTEROPERABILITY
// ============================================================================

/// Describes the data provenance integration boundary.
///
/// This value is metadata only. It does not initialize storage,
/// serialization, AI, networking, or distributed-ledger providers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProvenanceIntegration {
    /// Stable identity of this integration boundary.
    pub module_name: &'static str,

    /// Data provenance API version.
    pub api_version: u16,

    /// Canonical semantic provenance schema version.
    pub semantic_schema_version: u16,
}

impl ProvenanceIntegration {
    /// Returns the integration descriptor.
    #[must_use]
    pub const fn current() -> Self {
        Self {
            module_name: DATA_PROVENANCE_MODULE_NAME,
            api_version: DATA_PROVENANCE_API_VERSION,
            semantic_schema_version: PROVENANCE_MODEL_SCHEMA_VERSION,
        }
    }
}

impl fmt::Display for ProvenanceIntegration {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{} (API {}, semantic schema {})",
            self.module_name,
            self.api_version,
            self.semantic_schema_version,
        )
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: u64) -> ProvenanceId {
        ProvenanceId::new(value)
            .expect("test identity must be non-zero")
    }

    fn subject(name: &str) -> ProvenanceSubject {
        ProvenanceSubject::artifact(
            "test.dataset",
            name,
        )
        .expect("test artifact reference must be valid")
    }

    fn record(value: u64) -> ProvenanceRecord {
        ProvenanceRecord::new(
            id(value),
            subject(&format!("artifact-{value}")),
        )
    }

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(
            module_name(),
            "zamani.stdlib.data.provenance",
        );

        assert_eq!(api_version(), 1);
        assert_eq!(semantic_schema_version(), 1);
    }

    #[test]
    fn creates_empty_graph() {
        let provenance = DataProvenance::new();

        assert!(provenance.is_empty());
        assert_eq!(provenance.len(), 0);
    }

    #[test]
    fn inserts_canonical_records() {
        let mut provenance = DataProvenance::new();

        provenance
            .insert(record(1))
            .expect("insertion must succeed");

        assert_eq!(provenance.len(), 1);
        assert!(provenance.contains(id(1)));
    }

    #[test]
    fn rejects_duplicate_identity() {
        let mut provenance = DataProvenance::new();

        provenance.insert(record(1)).unwrap();

        let result = provenance.insert(record(1));

        assert!(result.is_err());
        assert_eq!(provenance.len(), 1);
    }

    #[test]
    fn supports_staged_forward_references() {
        let mut provenance = DataProvenance::new();

        let derived = record(2).with_relation(
            ProvenanceRelation::new(
                ProvenanceRelationKind::WasDerivedFrom,
                id(1),
            ),
        );

        provenance.insert(derived).unwrap();

        assert!(provenance.validate().is_err());

        provenance.insert(record(1)).unwrap();

        assert!(provenance.validate().is_ok());
    }

    #[test]
    fn validates_derivation_relationships() {
        let mut provenance = DataProvenance::new();

        provenance.insert(record(1)).unwrap();

        let derived = record(2).with_relation(
            ProvenanceRelation::new(
                ProvenanceRelationKind::WasDerivedFrom,
                id(1),
            ),
        );

        provenance.insert(derived).unwrap();

        assert!(provenance.validate().is_ok());
    }

    #[test]
    fn rejects_derivation_cycles() {
        let mut provenance = DataProvenance::new();

        let first = record(1).with_relation(
            ProvenanceRelation::new(
                ProvenanceRelationKind::WasDerivedFrom,
                id(2),
            ),
        );

        let second = record(2).with_relation(
            ProvenanceRelation::new(
                ProvenanceRelationKind::WasDerivedFrom,
                id(1),
            ),
        );

        provenance.insert(first).unwrap();
        provenance.insert(second).unwrap();

        assert!(provenance.validate().is_err());
    }

    #[test]
    fn validated_insertion_is_atomic() {
        let mut provenance = DataProvenance::new();

        let invalid = record(2).with_relation(
            ProvenanceRelation::new(
                ProvenanceRelationKind::WasDerivedFrom,
                id(99),
            ),
        );

        let result = provenance.insert_validated(invalid);

        assert!(result.is_err());
        assert!(provenance.is_empty());
    }

    #[test]
    fn validated_insertion_accepts_complete_graph() {
        let mut provenance = DataProvenance::new();

        provenance.insert(record(1)).unwrap();

        let derived = record(2).with_relation(
            ProvenanceRelation::new(
                ProvenanceRelationKind::WasDerivedFrom,
                id(1),
            ),
        );

        provenance
            .insert_validated(derived)
            .unwrap();

        assert_eq!(provenance.len(), 2);
        assert!(provenance.validate().is_ok());
    }

    #[test]
    fn removes_records_explicitly() {
        let mut provenance = DataProvenance::new();

        provenance.insert(record(1)).unwrap();

        assert!(provenance.remove(id(1)).is_some());
        assert!(provenance.is_empty());
    }

    #[test]
    fn exposes_canonical_graph_without_copying() {
        let mut provenance = DataProvenance::new();

        provenance.insert(record(1)).unwrap();

        assert!(provenance
            .as_semantic_graph()
            .contains(id(1)));
    }

    #[test]
    fn round_trips_canonical_graph() {
        let mut provenance = DataProvenance::new();

        provenance.insert(record(1)).unwrap();

        let graph = provenance.into_semantic_graph();

        let restored =
            DataProvenance::try_from_semantic_graph(graph)
                .expect("valid graph must be accepted");

        assert_eq!(restored.len(), 1);
    }

    #[test]
    fn deterministic_iteration_uses_identity_order() {
        let mut provenance = DataProvenance::new();

        provenance.insert(record(3)).unwrap();
        provenance.insert(record(1)).unwrap();
        provenance.insert(record(2)).unwrap();

        let identities: Vec<u64> = provenance
            .iter()
            .map(|(identity, _)| identity.get())
            .collect();

        assert_eq!(identities, vec![1, 2, 3]);
    }

    #[test]
    fn integration_descriptor_matches_module() {
        let descriptor = ProvenanceIntegration::current();

        assert_eq!(
            descriptor.module_name,
            DATA_PROVENANCE_MODULE_NAME,
        );

        assert_eq!(
            descriptor.semantic_schema_version,
            PROVENANCE_MODEL_SCHEMA_VERSION,
        );
    }

    #[test]
    fn clearing_graph_does_not_affect_external_semantics() {
        let mut provenance = DataProvenance::new();

        provenance.insert(record(1)).unwrap();
        provenance.clear();

        assert!(provenance.is_empty());
    }
}
