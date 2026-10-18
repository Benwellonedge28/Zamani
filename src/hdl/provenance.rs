
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Provenance
//!
//! Path: `src/hdl/provenance.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//! Status: Integration-ready contract; workspace verification is required.
//!
//! ## Purpose
//!
//! Provide HDL-specific constructors and conventions for provenance while
//! reusing the canonical AST source infrastructure and semantic provenance
//! model. This module records origin and lineage metadata; it does not prove
//! that a claim is true, authenticate an external artifact, or execute a
//! compiler transformation.
//!
//! ## Authority
//!
//! - `grammar/specification/`
//! - `grammar/spec/provenance.md`
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `src/frontend/ast/node/source/`
//! - `src/frontend/semantic/model/ids.rs`
//! - `src/frontend/semantic/model/provenance.rs`
//! - `src/frontend/parser/domains/hdl.rs`
//! - `src/frontend/semantic/domains/hdl.rs`
//!
//! Grammar owns source syntax. The canonical AST owns node identities and
//! source locations. The semantic provenance model owns provenance records,
//! origins, relationship kinds, graph storage, and graph validation.
//! This module owns only HDL-specific provenance conventions and adapters.
//!
//! ## Owns
//!
//! - Stable HDL provenance attribute keys.
//! - HDL-specific record constructors.
//! - Explicit portable versus physical-realization provenance scope.
//! - Import metadata and source-location association.
//! - Helpers for expressing derivation through canonical relationships.
//!
//! ## Does not own
//!
//! - A second AST, source map, span type, provenance ID allocator, or
//!   provenance graph.
//! - General semantic provenance semantics.
//! - Parsing or interpreting HDL source syntax.
//! - Name resolution, type checking, or semantic validation.
//! - HDLIR, synthesis, scheduling, placement, routing, or physical allocation.
//! - Artifact retrieval, hashing, signatures, attestation, or trust decisions.
//!
//! All HDLIR implementation remains under `src/hdl/hdlir/` and is out of
//! scope for this module.
//!
//! ## Scalability and determinism
//!
//! This module defines no maximum number of records, origins, source files,
//! modules, signals, or hardware resources. It uses dynamically sized values
//! and delegates graph storage to the canonical semantic model.
//!
//! IDs are supplied by the owning caller. No global allocator, pointer
//! identity, timestamp, random identity, host-dependent index, or physical
//! device number is used as portable semantic identity.
//!
//! The finite ID representation, address space, available memory, and
//! configured operational budgets remain implementation constraints.
//! Such constraints must be reported explicitly and must not be mistaken
//! for source-language restrictions.
//!
//! ## Integration
//!
//! Consumers may include these records in the canonical
//! `ProvenanceGraph`. Source origins use canonical `NodeId` and `Span`.
//! Generated and external origins use the canonical semantic provenance
//! types. Lineage uses canonical `ProvenanceId` references.
//!
//! The caller must ensure that IDs are unique in the owning graph, that
//! referenced records exist when graph validation is performed, and that
//! source spans actually belong to their referenced AST nodes.
//!
//! The semantic model's `provenance` module must be publicly registered
//! before this module can import it.
//!
//! ## Freeze criteria
//!
//! - The canonical semantic provenance module is registered and public.
//! - The HDL composition root registers this module.
//! - The canonical frontend API is available through the crate's module tree.
//! - Tests and workspace formatting, checking, linting, and conformance checks
//!   pass after integration.

use core::fmt;

use crate::frontend::ast::node::node_id::NodeId;
use crate::frontend::ast::node::source::Span;
use crate::frontend::semantic::model::ids::ProvenanceId;
use crate::frontend::semantic::model::provenance::{
    ArtifactReference,
    ExternalOrigin,
    GeneratedOrigin,
    ProvenanceError,
    ProvenanceOrigin,
    ProvenanceRecord,
    ProvenanceRelation,
    ProvenanceRelationKind,
    ProvenanceSubject,
    SourceOrigin,
};

/// Schema version for HDL-specific provenance conventions.
///
/// This version is independent of the language, AST, semantic-program,
/// semantic-provenance, HDLIR, and serialized-artifact schema versions.
pub const HDL_PROVENANCE_SCHEMA_VERSION: u16 = 1;

/// Stable namespace prefix for HDL-specific provenance attributes.
pub const HDL_PROVENANCE_ATTRIBUTE_NAMESPACE: &str = "hdl.provenance";

/// Stable key identifying the kind of HDL provenance origin.
pub const ATTRIBUTE_ORIGIN_KIND: &str = "hdl.provenance.origin_kind";

/// Stable key identifying the provenance scope.
pub const ATTRIBUTE_SCOPE: &str = "hdl.provenance.scope";

/// Stable key identifying an imported artifact's namespace.
pub const ATTRIBUTE_IMPORT_NAMESPACE: &str = "hdl.provenance.import_namespace";

/// Stable key identifying an imported artifact's external identifier.
pub const ATTRIBUTE_IMPORT_IDENTIFIER: &str = "hdl.provenance.import_identifier";

/// Stable key identifying a target namespace for physical provenance.
pub const ATTRIBUTE_TARGET_NAMESPACE: &str = "hdl.provenance.target_namespace";

/// Stable key identifying a target for physical provenance.
pub const ATTRIBUTE_TARGET_IDENTIFIER: &str = "hdl.provenance.target_identifier";

/// Portable or target-specific scope for an HDL provenance record.
///
/// Portable semantic provenance describes source meaning and transformations
/// independently of a physical target. Physical realization provenance
/// describes an implementation associated with an explicitly identified
/// target. The two scopes must not be silently conflated.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlProvenanceScope {
    /// Target-independent source or semantic provenance.
    PortableSemantic,

    /// Provenance associated with a particular realization target.
    ///
    /// The target reference is an opaque, qualified identity. It is not a
    /// device handle, physical address, or proof that the target exists.
    PhysicalRealization {
        /// Qualified identity of the realization target.
        target: ArtifactReference,
    },
}

impl HdlProvenanceScope {
    /// Construct portable semantic scope.
    #[must_use]
    pub const fn portable() -> Self {
        Self::PortableSemantic
    }

    /// Construct physical realization scope for a qualified target.
    pub fn physical(
        namespace: impl Into<String>,
        identifier: impl Into<String>,
    ) -> Result<Self, ProvenanceError> {
        Ok(Self::PhysicalRealization {
            target: ArtifactReference::new(namespace, identifier)?,
        })
    }
}

/// HDL-specific errors when constructing or annotating provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlProvenanceError {
    /// A canonical provenance operation failed.
    Canonical(ProvenanceError),
}

impl fmt::Display for HdlProvenanceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Canonical(error) => {
                write!(formatter, "HDL provenance error: {error}")
            }
        }
    }
}

impl std::error::Error for HdlProvenanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Canonical(error) => Some(error),
        }
    }
}

impl From<ProvenanceError> for HdlProvenanceError {
    fn from(error: ProvenanceError) -> Self {
        Self::Canonical(error)
    }
}

/// Create a provenance record for a canonical source AST node.
///
/// `id` must be unique within the destination provenance graph. `node_id`
/// and `span` must refer to the actual canonical AST node and its source
/// range; this function does not have AST storage with which to verify that
/// association.
///
/// The source span is retained through the canonical semantic provenance
/// type, not copied into an HDL-specific coordinate representation.
pub fn source_record(
    id: ProvenanceId,
    node_id: NodeId,
    span: Span,
    scope: HdlProvenanceScope,
) -> Result<ProvenanceRecord, HdlProvenanceError> {
    let record = ProvenanceRecord::new(
        id,
        ProvenanceSubject::ast_node(node_id),
    )
    .with_origin(ProvenanceOrigin::Source(SourceOrigin::new(
        node_id, span,
    )));

    annotate(record, "source", scope)
}

/// Create a provenance record for a compiler- or tool-generated HDL entity.
///
/// The activity name and optional version are descriptive metadata. They
/// are not commands and are never executed by this module.
pub fn generated_record(
    id: ProvenanceId,
    subject: ProvenanceSubject,
    activity: impl Into<String>,
    activity_version: Option<String>,
    scope: HdlProvenanceScope,
) -> Result<ProvenanceRecord, HdlProvenanceError> {
    let origin = GeneratedOrigin::new(activity, activity_version)?;

    let record = ProvenanceRecord::new(id, subject)
        .with_origin(ProvenanceOrigin::Generated(origin));

    annotate(record, "generated", scope)
}

/// Create a provenance record for an imported HDL artifact.
///
/// `authority` and `reference` identify the external source system and
/// artifact. They are not fetched or authenticated here.
///
/// When `source_location` is present, its canonical node ID and span are
/// retained as the source origin while the import's external identity is
/// preserved in namespaced attributes. Without a source location, the
/// canonical origin is external.
///
/// The subject can be an AST node, semantic entity, or qualified artifact,
/// depending on which entity the caller is recording.
pub fn imported_record(
    id: ProvenanceId,
    subject: ProvenanceSubject,
    authority: impl Into<String>,
    reference: impl Into<String>,
    source_location: Option<(NodeId, Span)>,
    scope: HdlProvenanceScope,
) -> Result<ProvenanceRecord, HdlProvenanceError> {
    let authority = authority.into();
    let reference = reference.into();

    let origin = match source_location {
        Some((node_id, span)) => {
            ProvenanceOrigin::Source(SourceOrigin::new(node_id, span))
        }
        None => ProvenanceOrigin::External(
            ExternalOrigin::new(authority.clone(), reference.clone())?,
        ),
    };

    let record = ProvenanceRecord::new(id, subject)
        .with_origin(origin)
        .with_attribute(ATTRIBUTE_IMPORT_NAMESPACE, authority)?
        .with_attribute(ATTRIBUTE_IMPORT_IDENTIFIER, reference)?;

    annotate(record, "imported", scope)
}

/// Create a provenance record whose origin is intentionally unknown.
///
/// Unknown means that the origin has not been established or recorded. It
/// must not be interpreted as proof that the entity has no origin.
pub fn unknown_record(
    id: ProvenanceId,
    subject: ProvenanceSubject,
    scope: HdlProvenanceScope,
) -> Result<ProvenanceRecord, HdlProvenanceError> {
    annotate(
        ProvenanceRecord::new(id, subject)
            .with_origin(ProvenanceOrigin::Unknown),
        "unknown",
        scope,
    )
}

/// Create a provenance record with explicit derivation relationships.
///
/// Each source ID becomes a canonical `WasDerivedFrom` relationship. This
/// helper does not allocate IDs, insert records, or validate that referenced
/// records exist. The owning `ProvenanceGraph` must be validated after
/// construction.
///
/// Relationship order follows the caller's iterator order.
pub fn derived_record(
    id: ProvenanceId,
    subject: ProvenanceSubject,
    sources: impl IntoIterator<Item = ProvenanceId>,
    activity: impl Into<String>,
    activity_version: Option<String>,
    scope: HdlProvenanceScope,
) -> Result<ProvenanceRecord, HdlProvenanceError> {
    let origin = GeneratedOrigin::new(activity, activity_version)?;

    let relations = sources.into_iter().map(|source_id| {
        ProvenanceRelation::new(
            ProvenanceRelationKind::WasDerivedFrom,
            source_id,
        )
    });

    let record = ProvenanceRecord::new(id, subject)
        .with_origin(ProvenanceOrigin::Generated(origin))
        .with_relations(relations);

    annotate(record, "derived", scope)
}

/// Add HDL origin and scope metadata to a canonical provenance record.
///
/// Existing canonical attributes are retained. If one of the reserved HDL
/// keys is already present, this function replaces that value deliberately.
/// Callers should not use these reserved keys for unrelated metadata.
pub fn annotate(
    mut record: ProvenanceRecord,
    origin_kind: &str,
    scope: HdlProvenanceScope,
) -> Result<ProvenanceRecord, HdlProvenanceError> {
    record.set_attribute(ATTRIBUTE_ORIGIN_KIND, origin_kind)?;

    match scope {
        HdlProvenanceScope::PortableSemantic => {
            record.set_attribute(ATTRIBUTE_SCOPE, "portable_semantic")?;
            record.remove_attribute(ATTRIBUTE_TARGET_NAMESPACE);
            record.remove_attribute(ATTRIBUTE_TARGET_IDENTIFIER);
        }
        HdlProvenanceScope::PhysicalRealization { target } => {
            record.set_attribute(ATTRIBUTE_SCOPE, "physical_realization")?;
            record.set_attribute(
                ATTRIBUTE_TARGET_NAMESPACE,
                target.namespace(),
            )?;
            record.set_attribute(
                ATTRIBUTE_TARGET_IDENTIFIER,
                target.identifier(),
            )?;
        }
    }

    Ok(record)
}

/// Return the schema version for HDL-specific provenance conventions.
#[must_use]
pub const fn schema_version() -> u16 {
    HDL_PROVENANCE_SCHEMA_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::source::{
        SourceId,
        SourceOffset,
        Span,
    };
    use crate::frontend::semantic::model::ids::ProvenanceId;

    fn provenance_id(raw: u64) -> ProvenanceId {
        ProvenanceId::new(raw).expect("test provenance IDs must be non-zero")
    }

    fn test_span() -> Span {
        Span::empty(
            SourceId::from_raw(1),
            SourceOffset::from_raw(0),
        )
        .expect("empty source span is valid")
    }

    #[test]
    fn schema_version_is_explicit_and_nonzero() {
        assert_eq!(schema_version(), 1);
        assert!(schema_version() > 0);
    }

    #[test]
    fn source_record_uses_canonical_node_and_span() {
        let node_id = NodeId::first();
        let span = test_span();

        let record = source_record(
            provenance_id(1),
            node_id,
            span,
            HdlProvenanceScope::portable(),
        )
        .expect("source record");

        assert_eq!(record.id(), provenance_id(1));
        assert_eq!(
            record.origin(),
            &ProvenanceOrigin::Source(SourceOrigin::new(node_id, span))
        );
        assert_eq!(
            record.attributes().get(ATTRIBUTE_ORIGIN_KIND).map(String::as_str),
            Some("source")
        );
        assert_eq!(
            record.attributes().get(ATTRIBUTE_SCOPE).map(String::as_str),
            Some("portable_semantic")
        );
    }

    #[test]
    fn generated_record_retains_activity_metadata() {
        let record = generated_record(
            provenance_id(2),
            ProvenanceSubject::semantic_entity(
                crate::frontend::semantic::model::ids::SemanticId::new(7)
                    .expect("non-zero semantic ID"),
            ),
            "hdl.elaboration",
            Some("1".to_owned()),
            HdlProvenanceScope::portable(),
        )
        .expect("generated record");

        match record.origin() {
            ProvenanceOrigin::Generated(origin) => {
                assert_eq!(origin.activity(), "hdl.elaboration");
                assert_eq!(origin.activity_version(), Some("1"));
            }
            other => panic!("unexpected origin: {other:?}"),
        }
    }

    #[test]
    fn imported_record_retains_external_identity_and_source_location() {
        let node_id = NodeId::first();
        let span = test_span();

        let record = imported_record(
            provenance_id(3),
            ProvenanceSubject::ast_node(node_id),
            "hdl.import.systemverilog",
            "library/component",
            Some((node_id, span)),
            HdlProvenanceScope::portable(),
        )
        .expect("imported record");

        assert_eq!(
            record.attributes()
                .get(ATTRIBUTE_ORIGIN_KIND)
                .map(String::as_str),
            Some("imported")
        );
        assert_eq!(
            record.attributes()
                .get(ATTRIBUTE_IMPORT_NAMESPACE)
                .map(String::as_str),
            Some("hdl.import.systemverilog")
        );
        assert_eq!(
            record.attributes()
                .get(ATTRIBUTE_IMPORT_IDENTIFIER)
                .map(String::as_str),
            Some("library/component")
        );
        assert_eq!(
            record.origin(),
            &ProvenanceOrigin::Source(SourceOrigin::new(node_id, span))
        );
    }

    #[test]
    fn imported_record_without_source_location_uses_external_origin() {
        let record = imported_record(
            provenance_id(4),
            ProvenanceSubject::artifact("hdl.library", "component"),
            "hdl.import.vhdl",
            "component-42",
            None,
            HdlProvenanceScope::portable(),
        )
        .expect("imported record");

        assert!(matches!(
            record.origin(),
            ProvenanceOrigin::External(_)
        ));
    }

    #[test]
    fn derived_record_uses_canonical_lineage_relationships() {
        let record = derived_record(
            provenance_id(5),
            ProvenanceSubject::semantic_entity(
                crate::frontend::semantic::model::ids::SemanticId::new(8)
                    .expect("non-zero semantic ID"),
            ),
            [provenance_id(1), provenance_id(2)],
            "hdl.analysis",
            None,
            HdlProvenanceScope::portable(),
        )
        .expect("derived record");

        assert_eq!(record.relations().len(), 2);
        assert!(record.relations().iter().all(|relation| {
            relation.kind().is_derivation()
        }));
        assert_eq!(record.relations()[0].target(), provenance_id(1));
        assert_eq!(record.relations()[1].target(), provenance_id(2));
    }

    #[test]
    fn physical_realization_scope_is_explicit_and_target_scoped() {
        let scope = HdlProvenanceScope::physical(
            "fpga.vendor.example",
            "target-family/device-profile",
        )
        .expect("valid target reference");

        let record = unknown_record(
            provenance_id(6),
            ProvenanceSubject::artifact("hdl.design", "design-1"),
            scope,
        )
        .expect("physical-scope record");

        assert_eq!(
            record.attributes().get(ATTRIBUTE_SCOPE).map(String::as_str),
            Some("physical_realization")
        );
        assert_eq!(
            record.attributes()
                .get(ATTRIBUTE_TARGET_NAMESPACE)
                .map(String::as_str),
            Some("fpga.vendor.example")
        );
        assert_eq!(
            record.attributes()
                .get(ATTRIBUTE_TARGET_IDENTIFIER)
                .map(String::as_str),
            Some("target-family/device-profile")
        );
    }

    #[test]
    fn portable_scope_removes_stale_physical_target_metadata() {
        let target = ArtifactReference::new(
            "vendor.target",
            "device-profile",
        )
        .expect("target");

        let physical = annotate(
            ProvenanceRecord::new(
                provenance_id(7),
                ProvenanceSubject::artifact("hdl.design", "design-2"),
            ),
            "unknown",
            HdlProvenanceScope::PhysicalRealization { target },
        )
        .expect("physical record");

        let portable = annotate(
            physical,
            "source",
            HdlProvenanceScope::portable(),
        )
        .expect("portable record");

        assert_eq!(
            portable.attributes().get(ATTRIBUTE_SCOPE).map(String::as_str),
            Some("portable_semantic")
        );
        assert!(!portable.attributes().contains_key(ATTRIBUTE_TARGET_NAMESPACE));
        assert!(!portable.attributes().contains_key(ATTRIBUTE_TARGET_IDENTIFIER));
    }

    #[test]
    fn invalid_external_metadata_is_reported_without_panicking() {
        let result = imported_record(
            provenance_id(8),
            ProvenanceSubject::artifact("hdl.library", "component"),
            " ",
            "component-42",
            None,
            HdlProvenanceScope::portable(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn origin_metadata_is_not_used_as_identity() {
        let node_id = NodeId::first();
        let span = test_span();

        let first = source_record(
            provenance_id(9),
            node_id,
            span,
            HdlProvenanceScope::portable(),
        )
        .expect("first record");

        let second = source_record(
            provenance_id(10),
            node_id,
            span,
            HdlProvenanceScope::portable(),
        )
        .expect("second record");

        assert_ne!(first.id(), second.id());
        assert_eq!(first.origin(), second.origin());
    }
}
