
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Interchange Schema
//!
//! Path: `src/hybrid/interchange/schema.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Responsibility
//!
//! Defines the logical, versioned interchange document for hybrid
//! computation. The schema preserves cross-domain composition, references
//! to canonical domain operations, dependencies, cross-domain values,
//! synchronization contracts, requirements, capabilities, constraints,
//! provenance, and compatibility metadata.
//!
//! This module defines an interchange contract, not an execution IR.
//!
//! ## Canonical ownership
//!
//! - Hybrid identifiers: `super::super::ids`.
//! - Hybrid schema and format versions: `super::super::version`.
//! - Domain interface contracts: `super::super::contract`.
//! - Classical semantics and IR: their existing canonical subsystem.
//! - Quantum semantics and IR: `quantum::ir`.
//! - Universal computation IR: the existing ZUIR subsystem.
//! - Target lowering and code generation: compiler/backend.
//! - Execution and device lifecycle: runtime.
//!
//! This module never serializes domain-specific IR by reconstructing it
//! from strings or inventing a parallel representation of its operations.
//! Instead, it carries stable references to canonical domain artifacts.
//!
//! ## Scalability
//!
//! The schema introduces no universal hardware, node, edge, domain,
//! operation, tensor-rank, or nesting-depth ceiling. Collections grow
//! according to available resources and caller policy.
//!
//! The document is finite, and finite computers have finite resources.
//! Serialization and decoding must therefore enforce any configured
//! operational budgets in the encoder/decoder, not as language semantics.
//!
//! ## Validation
//!
//! Serde deserialization is not proof of validity. Call `validate` after
//! deserializing an untrusted document. Validation checks identity scope,
//! duplicate IDs, references, endpoint consistency, required identifiers,
//! and basic document invariants. Registry resolution, semantic type
//! checking, domain-contract satisfaction, and target feasibility remain
//! responsibilities of their canonical owners.

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::hybrid::contract::DomainInterfaceRef;
use crate::hybrid::ids::{
    CrossDomainValueId,
    DomainInstanceId,
    HybridDependencyId,
    HybridEdgeId,
    HybridGraphId,
    HybridNodeId,
    SynchronizationEventId,
};
use crate::hybrid::version::{
    HybridFormatVersion,
    HybridSchemaVersion,
    CURRENT_HYBRID_FORMAT_VERSION,
    CURRENT_HYBRID_SCHEMA_VERSION,
};

/// Stable identifier for this logical document kind.
///
/// This identifies the schema family, not a physical target or domain.
pub const HYBRID_DOCUMENT_KIND: &str = "zamani.hybrid";

/// Current logical schema version.
///
/// The authoritative version declaration lives in `version.rs`.
pub const SCHEMA_VERSION: HybridSchemaVersion =
    CURRENT_HYBRID_SCHEMA_VERSION;

/// Current interchange format version.
///
/// The authoritative version declaration lives in `version.rs`.
pub const FORMAT_VERSION: HybridFormatVersion =
    CURRENT_HYBRID_FORMAT_VERSION;

/// Versioned hybrid interchange document.
///
/// The representation separates format compatibility from logical schema
/// compatibility. A matching format version alone does not establish that
/// the document's domain contracts or semantic references are valid.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HybridDocument {
    /// Stable schema-family identifier.
    pub kind: String,

    /// Serialized encoding contract version.
    pub format_version: HybridFormatVersion,

    /// Logical representation contract version.
    pub schema_version: HybridSchemaVersion,

    /// Document-wide identity and provenance.
    pub metadata: DocumentMetadata,

    /// Canonical hybrid-composition references and relationships.
    pub composition: HybridComposition,

    /// Portable declarations associated with this composition.
    ///
    /// The meaning of each declaration is owned by the canonical semantic,
    /// resource, capability, or policy subsystem.
    #[serde(default)]
    pub declarations: Vec<SemanticDeclaration>,

    /// Extensible, namespaced metadata for tools and integrations.
    ///
    /// These values are descriptive. Unknown metadata must never be
    /// interpreted as a satisfied requirement or a semantic guarantee.
    #[serde(default)]
    pub extensions: Vec<ExtensionRecord>,
}

impl HybridDocument {
    /// Creates a document using the current schema and format versions.
    #[must_use]
    pub fn new(
        metadata: DocumentMetadata,
        composition: HybridComposition,
    ) -> Self {
        Self {
            kind: HYBRID_DOCUMENT_KIND.to_owned(),
            format_version: CURRENT_HYBRID_FORMAT_VERSION,
            schema_version: CURRENT_HYBRID_SCHEMA_VERSION,
            metadata,
            composition,
            declarations: Vec::new(),
            extensions: Vec::new(),
        }
    }

    /// Validates the document's schema-level invariants.
    ///
    /// This is structural validation, not proof of semantic equivalence,
    /// domain availability, capability satisfaction, or executability.
    pub fn validate(&self) -> Result<(), SchemaError> {
        if self.kind != HYBRID_DOCUMENT_KIND {
            return Err(SchemaError::InvalidDocumentKind);
        }

        if self.format_version != CURRENT_HYBRID_FORMAT_VERSION {
            return Err(SchemaError::UnsupportedFormatVersion {
                found: self.format_version,
                expected: CURRENT_HYBRID_FORMAT_VERSION,
            });
        }

        if self.schema_version != CURRENT_HYBRID_SCHEMA_VERSION {
            return Err(SchemaError::UnsupportedSchemaVersion {
                found: self.schema_version,
                expected: CURRENT_HYBRID_SCHEMA_VERSION,
            });
        }

        self.metadata.validate()?;
        self.composition.validate()?;

        for declaration in &self.declarations {
            declaration.validate()?;
        }

        for extension in &self.extensions {
            extension.validate()?;
        }

        validate_unique_by(
            self.declarations.iter(),
            |item| (item.namespace.as_str(), item.name.as_str()),
            SchemaError::DuplicateDeclaration,
        )?;

        validate_unique_by(
            self.extensions.iter(),
            |item| item.namespace.as_str(),
            SchemaError::DuplicateExtensionNamespace,
        )?;

        Ok(())
    }
}

/// Document identity and provenance.
///
/// Provenance identifies the producer and source context. It is not a
/// cryptographic signature or proof that the producer is trusted.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DocumentMetadata {
    /// Identity of the composition described by this document.
    pub graph_id: HybridGraphId,

    /// Stable logical document identifier, if supplied by the producer.
    ///
    /// This identifier is opaque and does not replace `graph_id`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,

    /// Producer name or tool identifier.
    pub producer: String,

    /// Producer version, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer_version: Option<String>,

    /// Optional source artifact digest, including its algorithm identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_digest: Option<ArtifactDigest>,

    /// Optional language-version reference.
    ///
    /// The language-version authority remains outside this schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language_version: Option<String>,

    /// Additional non-semantic provenance attributes.
    #[serde(default)]
    pub attributes: Vec<MetadataEntry>,
}

impl DocumentMetadata {
    /// Checks the structural validity of document metadata.
    pub fn validate(&self) -> Result<(), SchemaError> {
        require_non_empty("producer", &self.producer)?;

        validate_optional_string("document_id", self.document_id.as_deref())?;
        validate_optional_string(
            "producer_version",
            self.producer_version.as_deref(),
        )?;
        validate_optional_string(
            "language_version",
            self.language_version.as_deref(),
        )?;

        if let Some(digest) = &self.source_digest {
            digest.validate()?;
        }

        for entry in &self.attributes {
            entry.validate()?;
        }

        validate_unique_by(
            self.attributes.iter(),
            |entry| entry.key.as_str(),
            SchemaError::DuplicateMetadataKey,
        )
    }
}

/// A content digest whose algorithm is explicitly named.
///
/// The digest is opaque to the schema. Verification belongs to the
/// artifact-integrity layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactDigest {
    /// Algorithm identifier, for example `sha256`.
    pub algorithm: String,

    /// Canonical textual digest encoding.
    pub value: String,
}

impl ArtifactDigest {
    /// Validates the presence of an algorithm and digest value.
    pub fn validate(&self) -> Result<(), SchemaError> {
        require_non_empty("digest algorithm", &self.algorithm)?;
        require_non_empty("digest value", &self.value)?;

        if self.algorithm.bytes().any(|byte| byte.is_ascii_whitespace()) {
            return Err(SchemaError::InvalidField("digest algorithm"));
        }

        if self.value.bytes().any(|byte| byte.is_ascii_whitespace()) {
            return Err(SchemaError::InvalidField("digest value"));
        }

        Ok(())
    }
}

/// A single key/value provenance or metadata attribute.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetadataEntry {
    /// Attribute key.
    pub key: String,

    /// Attribute value.
    pub value: String,
}

impl MetadataEntry {
    /// Validates a metadata entry.
    pub fn validate(&self) -> Result<(), SchemaError> {
        require_non_empty("metadata key", &self.key)?;
        require_non_empty("metadata value", &self.value)
    }
}

/// Domain-neutral hybrid composition.
///
/// Nodes refer to operations owned by existing domain IRs. Edges express
/// relationships between those references without copying their operations
/// into another IR.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HybridComposition {
    /// Identity of this composition.
    pub graph_id: HybridGraphId,

    /// Participating domain instances.
    pub domains: Vec<DomainInstanceRecord>,

    /// Hybrid invocation and composition nodes.
    pub nodes: Vec<HybridNodeRecord>,

    /// Typed cross-node relationships.
    pub edges: Vec<HybridEdgeRecord>,

    /// Cross-domain values and their canonical type references.
    #[serde(default)]
    pub values: Vec<CrossDomainValueRecord>,

    /// Explicit synchronization contracts.
    #[serde(default)]
    pub synchronization: Vec<SynchronizationRecord>,
}

impl HybridComposition {
    /// Checks structural graph invariants and all local references.
    ///
    /// Cycles are not globally forbidden. Whether a cycle is valid depends
    /// on the canonical control-flow and feedback semantics.
    pub fn validate(&self) -> Result<(), SchemaError> {
        let graph_id = self.graph_id;

        let mut domain_ids = HashSet::new();
        for domain in &self.domains {
            domain.validate(graph_id)?;

            if !domain_ids.insert(domain.id) {
                return Err(SchemaError::DuplicateDomainId);
            }
        }

        let mut node_ids = HashSet::new();
        for node in &self.nodes {
            node.validate(graph_id)?;

            if !node_ids.insert(node.id) {
                return Err(SchemaError::DuplicateNodeId);
            }

            if !domain_ids.contains(&node.domain_instance) {
                return Err(SchemaError::UnknownDomainInstance);
            }
        }

        let mut edge_ids = HashSet::new();
        let mut dependency_ids = HashSet::new();

        for edge in &self.edges {
            edge.validate(graph_id)?;

            if !edge_ids.insert(edge.id) {
                return Err(SchemaError::DuplicateEdgeId);
            }

            if !dependency_ids.insert(edge.dependency_id) {
                return Err(SchemaError::DuplicateDependencyId);
            }

            if !node_ids.contains(&edge.source) || !node_ids.contains(&edge.target) {
                return Err(SchemaError::UnknownEdgeEndpoint);
            }
        }

        let mut value_ids = HashSet::new();
        for value in &self.values {
            value.validate(graph_id)?;

            if !value_ids.insert(value.id) {
                return Err(SchemaError::DuplicateValueId);
            }

            if !node_ids.contains(&value.producer) {
                return Err(SchemaError::UnknownValueProducer);
            }

            for consumer in &value.consumers {
                if !node_ids.contains(consumer) {
                    return Err(SchemaError::UnknownValueConsumer);
                }
            }
        }

        let mut synchronization_ids = HashSet::new();
        for synchronization in &self.synchronization {
            synchronization.validate(graph_id)?;

            if !synchronization_ids.insert(synchronization.id) {
                return Err(SchemaError::DuplicateSynchronizationId);
            }

            for participant in &synchronization.participants {
                if !node_ids.contains(participant) {
                    return Err(SchemaError::UnknownSynchronizationParticipant);
                }
            }
        }

        Ok(())
    }
}

/// One participating domain instance.
///
/// `interface` refers to a contract owned by the corresponding domain.
/// The schema does not imply that the interface is registered or available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainInstanceRecord {
    /// Graph-scoped identity.
    pub id: DomainInstanceId,

    /// Canonical domain interface contract reference.
    pub interface: DomainInterfaceRef,

    /// Optional stable identifier for the logical instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logical_name: Option<String>,

    /// Optional domain-owned artifact reference.
    ///
    /// This can identify an operation or region in a canonical IR.
    /// It must not contain fabricated instructions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<CanonicalArtifactRef>,

    /// Domain-specific metadata that is explicitly non-semantic.
    #[serde(default)]
    pub metadata: Vec<MetadataEntry>,
}

impl DomainInstanceRecord {
    /// Validates identity scope and local metadata.
    pub fn validate(&self, graph: HybridGraphId) -> Result<(), SchemaError> {
        if self.id.graph() != graph {
            return Err(SchemaError::ForeignGraphIdentity);
        }

        self.interface
            .validate()
            .map_err(|_| SchemaError::InvalidDomainInterface)?;

        validate_optional_string("logical_name", self.logical_name.as_deref())?;

        if let Some(artifact) = &self.artifact {
            artifact.validate()?;
        }

        for entry in &self.metadata {
            entry.validate()?;
        }

        validate_unique_by(
            self.metadata.iter(),
            |entry| entry.key.as_str(),
            SchemaError::DuplicateMetadataKey,
        )
    }
}

/// A reference to an artifact owned by a canonical subsystem.
///
/// `owner` identifies the subsystem and `reference` is interpreted only
/// by that subsystem. This schema deliberately does not define operation
/// payloads for ZUIR, ZCIR, quantum IR, HDLIR, or backend instructions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalArtifactRef {
    /// Stable subsystem identifier, such as `quantum.ir`.
    pub owner: String,

    /// Opaque, owner-defined artifact identity.
    pub reference: String,

    /// Optional owner-defined artifact version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl CanonicalArtifactRef {
    /// Checks only the reference's structure.
    pub fn validate(&self) -> Result<(), SchemaError> {
        require_non_empty("artifact owner", &self.owner)?;
        require_non_empty("artifact reference", &self.reference)?;
        validate_optional_string("artifact version", self.version.as_deref())
    }
}

/// A node in the hybrid composition graph.
///
/// This record references a canonical domain artifact instead of duplicating
/// the operation's semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HybridNodeRecord {
    /// Graph-scoped node identity.
    pub id: HybridNodeId,

    /// Domain instance that owns this node's operation.
    pub domain_instance: DomainInstanceId,

    /// Reference to the canonical operation or region.
    pub operation: CanonicalArtifactRef,

    /// Optional stable source-location reference.
    ///
    /// Source spans remain owned by the frontend/source-map subsystem.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_location: Option<SourceLocationRef>,

    /// Non-semantic annotations.
    #[serde(default)]
    pub metadata: Vec<MetadataEntry>,
}

impl HybridNodeRecord {
    /// Validates node identity and canonical references.
    pub fn validate(&self, graph: HybridGraphId) -> Result<(), SchemaError> {
        if self.id.graph() != graph || self.domain_instance.graph() != graph {
            return Err(SchemaError::ForeignGraphIdentity);
        }

        self.operation.validate()?;

        if let Some(location) = &self.source_location {
            location.validate()?;
        }

        for entry in &self.metadata {
            entry.validate()?;
        }

        validate_unique_by(
            self.metadata.iter(),
            |entry| entry.key.as_str(),
            SchemaError::DuplicateMetadataKey,
        )
    }
}

/// Reference to a source location owned by the frontend.
///
/// The interchange schema does not invent a second source-span type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceLocationRef {
    /// Stable source artifact identifier.
    pub source: String,

    /// Owner-defined span or location reference.
    pub reference: String,
}

impl SourceLocationRef {
    /// Validates the location reference.
    pub fn validate(&self) -> Result<(), SchemaError> {
        require_non_empty("source identifier", &self.source)?;
        require_non_empty("source location reference", &self.reference)
    }
}

/// A typed hybrid dependency relationship.
///
/// The relationship identifier is extensible. Unknown relationship
/// semantics must be rejected by consumers that need to interpret them;
/// this schema never silently assigns meaning to an unknown kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HybridEdgeRecord {
    /// Graph-scoped edge identity.
    pub id: HybridEdgeId,

    /// Graph-scoped dependency identity.
    pub dependency_id: HybridDependencyId,

    /// Upstream node.
    pub source: HybridNodeId,

    /// Downstream node.
    pub target: HybridNodeId,

    /// Namespaced dependency relationship.
    ///
    /// Examples: `hybrid.data`, `hybrid.control`, `hybrid.completion`.
    pub kind: String,

    /// Whether this dependency is mandatory for semantic correctness.
    pub required: bool,

    /// Optional owner-defined attributes.
    #[serde(default)]
    pub metadata: Vec<MetadataEntry>,
}

impl HybridEdgeRecord {
    /// Validates identity scope and edge structure.
    pub fn validate(&self, graph: HybridGraphId) -> Result<(), SchemaError> {
        if self.id.graph() != graph
            || self.dependency_id.graph() != graph
            || self.source.graph() != graph
            || self.target.graph() != graph
        {
            return Err(SchemaError::ForeignGraphIdentity);
        }

        require_non_empty("dependency kind", &self.kind)?;

        if self.source == self.target {
            return Err(SchemaError::SelfReferentialEdge);
        }

        for entry in &self.metadata {
            entry.validate()?;
        }

        validate_unique_by(
            self.metadata.iter(),
            |entry| entry.key.as_str(),
            SchemaError::DuplicateMetadataKey,
        )
    }
}

/// A value crossing a domain boundary.
///
/// The canonical semantic type is referenced, not reconstructed or
/// redefined by this interchange schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrossDomainValueRecord {
    /// Graph-scoped value identity.
    pub id: CrossDomainValueId,

    /// Node that produces the value.
    pub producer: HybridNodeId,

    /// Nodes that consume the value.
    pub consumers: Vec<HybridNodeId>,

    /// Canonical semantic type reference.
    pub semantic_type: String,

    /// Canonical ownership/lifetime contract reference.
    pub ownership_contract: String,

    /// Optional canonical data-layout contract reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout_contract: Option<String>,

    /// Optional transfer semantics reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_contract: Option<String>,
}

impl CrossDomainValueRecord {
    /// Validates identity and semantic-reference structure.
    pub fn validate(&self, graph: HybridGraphId) -> Result<(), SchemaError> {
        if self.id.graph() != graph || self.producer.graph() != graph {
            return Err(SchemaError::ForeignGraphIdentity);
        }

        require_non_empty("semantic type reference", &self.semantic_type)?;
        require_non_empty("ownership contract", &self.ownership_contract)?;
        validate_optional_string(
            "layout contract",
            self.layout_contract.as_deref(),
        )?;
        validate_optional_string(
            "transfer contract",
            self.transfer_contract.as_deref(),
        )?;

        let mut unique_consumers = HashSet::new();

        for consumer in &self.consumers {
            if consumer.graph() != graph {
                return Err(SchemaError::ForeignGraphIdentity);
            }

            if !unique_consumers.insert(*consumer) {
                return Err(SchemaError::DuplicateValueConsumer);
            }
        }

        Ok(())
    }
}

/// An explicit synchronization contract.
///
/// Synchronization semantics are expressed as a contract identifier and
/// participant references. Runtime handles and execution states are not
/// serialized as though they were portable program semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SynchronizationRecord {
    /// Graph-scoped synchronization event identity.
    pub id: SynchronizationEventId,

    /// Namespaced synchronization contract kind.
    ///
    /// Examples: `hybrid.sync.barrier`, `hybrid.sync.completion`,
    /// `hybrid.sync.feedback`.
    pub kind: String,

    /// Nodes participating in the synchronization contract.
    pub participants: Vec<HybridNodeId>,

    /// Optional owner-defined ordering guarantee reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordering_contract: Option<String>,

    /// Whether the synchronization contract is semantically required.
    pub required: bool,
}

impl SynchronizationRecord {
    /// Validates identity and synchronization references.
    pub fn validate(&self, graph: HybridGraphId) -> Result<(), SchemaError> {
        if self.id.graph() != graph {
            return Err(SchemaError::ForeignGraphIdentity);
        }

        require_non_empty("synchronization kind", &self.kind)?;
        validate_optional_string(
            "ordering contract",
            self.ordering_contract.as_deref(),
        )?;

        let mut unique_participants = HashSet::new();

        for participant in &self.participants {
            if participant.graph() != graph {
                return Err(SchemaError::ForeignGraphIdentity);
            }

            if !unique_participants.insert(*participant) {
                return Err(SchemaError::DuplicateSynchronizationParticipant);
            }
        }

        if self.required && self.participants.is_empty() {
            return Err(SchemaError::MissingRequiredSynchronizationParticipant);
        }

        Ok(())
    }
}

/// Semantic/resource declaration carried by the document.
///
/// `value` is an interchange payload, not a second expression language.
/// Its meaning is owned by the named canonical declaration namespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticDeclaration {
    /// Canonical declaration namespace.
    ///
    /// Examples: `semantic.requirement`, `resource.capability`,
    /// `resource.constraint`, `resource.preference`, `resource.hint`.
    pub namespace: String,

    /// Declaration identifier within its namespace.
    pub name: String,

    /// Whether failure to satisfy the declaration makes realization invalid.
    pub required: bool,

    /// Declaration payload interpreted by its canonical owner.
    pub value: Value,
}

impl SemanticDeclaration {
    /// Validates declaration identity.
    pub fn validate(&self) -> Result<(), SchemaError> {
        require_non_empty("declaration namespace", &self.namespace)?;
        require_non_empty("declaration name", &self.name)
    }
}

/// Namespaced extension record.
///
/// Extensions are metadata, not permission to alter known semantics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExtensionRecord {
    /// Globally namespaced extension identifier.
    pub namespace: String,

    /// Extension payload owned by the extension provider.
    pub value: Value,
}

impl ExtensionRecord {
    /// Validates the extension namespace.
    pub fn validate(&self) -> Result<(), SchemaError> {
        require_non_empty("extension namespace", &self.namespace)
    }
}

/// Structural schema validation errors.
///
/// These identify schema invariants only. Higher-level integrations should
/// translate them into the repository's canonical `HybridError` vocabulary
/// at the interchange boundary rather than parsing display strings.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SchemaError {
    InvalidDocumentKind,
    UnsupportedFormatVersion {
        found: HybridFormatVersion,
        expected: HybridFormatVersion,
    },
    UnsupportedSchemaVersion {
        found: HybridSchemaVersion,
        expected: HybridSchemaVersion,
    },
    InvalidField(&'static str),
    EmptyField(&'static str),
    InvalidDomainInterface,
    ForeignGraphIdentity,
    DuplicateDomainId,
    DuplicateNodeId,
    DuplicateEdgeId,
    DuplicateDependencyId,
    DuplicateValueId,
    DuplicateSynchronizationId,
    DuplicateValueConsumer,
    DuplicateSynchronizationParticipant,
    DuplicateDeclaration,
    DuplicateExtensionNamespace,
    DuplicateMetadataKey,
    UnknownDomainInstance,
    UnknownEdgeEndpoint,
    UnknownValueProducer,
    UnknownValueConsumer,
    UnknownSynchronizationParticipant,
    SelfReferentialEdge,
    MissingRequiredSynchronizationParticipant,
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDocumentKind =>
                f.write_str("unrecognized hybrid document kind"),

            Self::UnsupportedFormatVersion { found, expected } =>
                write!(f, "unsupported hybrid format version {found}; expected {expected}"),

            Self::UnsupportedSchemaVersion { found, expected } =>
                write!(f, "unsupported hybrid schema version {found}; expected {expected}"),

            Self::InvalidField(field) =>
                write!(f, "invalid value for field {field}"),

            Self::EmptyField(field) =>
                write!(f, "required field {field} is empty"),

            Self::InvalidDomainInterface =>
                f.write_str("invalid domain interface reference"),

            Self::ForeignGraphIdentity =>
                f.write_str("an entity belongs to a different hybrid graph"),

            Self::DuplicateDomainId =>
                f.write_str("duplicate domain-instance identity"),

            Self::DuplicateNodeId =>
                f.write_str("duplicate hybrid-node identity"),

            Self::DuplicateEdgeId =>
                f.write_str("duplicate hybrid-edge identity"),

            Self::DuplicateDependencyId =>
                f.write_str("duplicate dependency identity"),

            Self::DuplicateValueId =>
                f.write_str("duplicate cross-domain value identity"),

            Self::DuplicateSynchronizationId =>
                f.write_str("duplicate synchronization-event identity"),

            Self::DuplicateValueConsumer =>
                f.write_str("duplicate consumer in a cross-domain value"),

            Self::DuplicateSynchronizationParticipant =>
                f.write_str("duplicate synchronization participant"),

            Self::DuplicateDeclaration =>
                f.write_str("duplicate semantic declaration"),

            Self::DuplicateExtensionNamespace =>
                f.write_str("duplicate extension namespace"),

            Self::DuplicateMetadataKey =>
                f.write_str("duplicate metadata key"),

            Self::UnknownDomainInstance =>
                f.write_str("node references an unknown domain instance"),

            Self::UnknownEdgeEndpoint =>
                f.write_str("edge references an unknown node"),

            Self::UnknownValueProducer =>
                f.write_str("cross-domain value references an unknown producer"),

            Self::UnknownValueConsumer =>
                f.write_str("cross-domain value references an unknown consumer"),

            Self::UnknownSynchronizationParticipant =>
                f.write_str("synchronization references an unknown participant"),

            Self::SelfReferentialEdge =>
                f.write_str("edge directly references itself as source and target"),

            Self::MissingRequiredSynchronizationParticipant =>
                f.write_str("required synchronization has no participants"),
        }
    }
}

impl std::error::Error for SchemaError {}

fn require_non_empty(
    field: &'static str,
    value: &str,
) -> Result<(), SchemaError> {
    if value.trim().is_empty() {
        return Err(SchemaError::EmptyField(field));
    }

    Ok(())
}

fn validate_optional_string(
    field: &'static str,
    value: Option<&str>,
) -> Result<(), SchemaError> {
    if let Some(value) = value {
        require_non_empty(field, value)?;
    }

    Ok(())
}

fn validate_unique_by<'a, T, K, F>(
    items: impl IntoIterator<Item = &'a T>,
    key: F,
    duplicate_error: SchemaError,
) -> Result<(), SchemaError>
where
    T: 'a,
    K: Eq + std::hash::Hash,
    F: Fn(&'a T) -> K,
{
    let mut seen = HashSet::new();

    for item in items {
        if !seen.insert(key(item)) {
            return Err(duplicate_error.clone());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::ids::{GraphScopedId, IdComponent};

    fn graph_id() -> HybridGraphId {
        HybridGraphId::new(1).expect("nonzero graph ID")
    }

    fn scoped_id(local: u128) -> GraphScopedId {
        GraphScopedId::new(
            graph_id(),
            IdComponent::new(local).expect("nonzero local ID"),
        )
    }

    fn domain_id(local: u128) -> DomainInstanceId {
        DomainInstanceId::new(scoped_id(local))
    }

    fn node_id(local: u128) -> HybridNodeId {
        HybridNodeId::new(scoped_id(local))
    }

    fn metadata() -> DocumentMetadata {
        DocumentMetadata {
            graph_id: graph_id(),
            document_id: None,
            producer: "zamani-test".to_owned(),
            producer_version: None,
            source_digest: None,
            language_version: None,
            attributes: Vec::new(),
        }
    }

    fn composition() -> HybridComposition {
        HybridComposition {
            graph_id: graph_id(),
            domains: Vec::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            values: Vec::new(),
            synchronization: Vec::new(),
        }
    }

    #[test]
    fn empty_composition_is_valid() {
        let document = HybridDocument::new(metadata(), composition());
        assert!(document.validate().is_ok());
    }

    #[test]
    fn document_kind_is_checked() {
        let mut document = HybridDocument::new(metadata(), composition());
        document.kind = "other".to_owned();

        assert_eq!(
            document.validate(),
            Err(SchemaError::InvalidDocumentKind)
        );
    }

    #[test]
    fn metadata_graph_must_match_composition_graph() {
        let mut document = HybridDocument::new(metadata(), composition());
        document.composition.graph_id =
            HybridGraphId::new(2).expect("nonzero graph ID");

        // Graph identity is validated at the document boundary too.
        assert_eq!(
            document.validate(),
            Err(SchemaError::ForeignGraphIdentity)
        );
    }

    #[test]
    fn node_must_reference_a_registered_domain_instance() {
        let mut graph = composition();

        graph.nodes.push(HybridNodeRecord {
            id: node_id(2),
            domain_instance: domain_id(1),
            operation: CanonicalArtifactRef {
                owner: "classical.ir".to_owned(),
                reference: "function:main".to_owned(),
                version: None,
            },
            source_location: None,
            metadata: Vec::new(),
        });

        assert_eq!(
            graph.validate(),
            Err(SchemaError::UnknownDomainInstance)
        );
    }

    #[test]
    fn graph_scoped_ids_cannot_cross_graph_boundaries() {
        let foreign_graph =
            HybridGraphId::new(9).expect("nonzero graph ID");

        let node = HybridNodeRecord {
            id: HybridNodeId::new(GraphScopedId::new(
                foreign_graph,
                IdComponent::new(2).expect("nonzero local ID"),
            )),
            domain_instance: domain_id(1),
            operation: CanonicalArtifactRef {
                owner: "quantum.ir".to_owned(),
                reference: "region:kernel".to_owned(),
                version: None,
            },
            source_location: None,
            metadata: Vec::new(),
        };

        assert_eq!(
            node.validate(graph_id()),
            Err(SchemaError::ForeignGraphIdentity)
        );
    }

    #[test]
    fn serialization_round_trip_preserves_schema() {
        let document = HybridDocument::new(metadata(), composition());

        let encoded =
            serde_json::to_string(&document).expect("serialize document");

        let decoded: HybridDocument =
            serde_json::from_str(&encoded).expect("deserialize document");

        assert_eq!(decoded, document);
        assert!(decoded.validate().is_ok());
    }

    #[test]
    fn duplicate_metadata_keys_are_rejected() {
        let mut document = HybridDocument::new(metadata(), composition());

        document.metadata.attributes = vec![
            MetadataEntry {
                key: "origin".to_owned(),
                value: "frontend".to_owned(),
            },
            MetadataEntry {
                key: "origin".to_owned(),
                value: "compiler".to_owned(),
            },
        ];

        assert_eq!(
            document.validate(),
            Err(SchemaError::DuplicateMetadataKey)
        );
    }
}
