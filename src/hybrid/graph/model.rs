
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Canonical Graph Model
//!
//! Path: `src/hybrid/graph/model.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Represent the domain-neutral structure of a hybrid computation spanning
//! classical, quantum, tensor, AI/ML, accelerator, HDL, distributed,
//! networking, and future computational domains.
//!
//! This is a composition model, not a competing universal IR.
//!
//! A graph node refers to an operation or other entity owned by a canonical
//! domain representation. The referenced representation remains authoritative
//! for the meaning of that entity.
//!
//! ## Ownership
//!
//! This module owns:
//! - The canonical hybrid graph container.
//! - Graph identity and representation-version declarations.
//! - Hybrid node identity and node-to-domain-instance association.
//! - References to entities in canonical domain representations.
//! - Graph-level descriptive metadata.
//! - Structural graph-model validation.
//!
//! It does not own:
//! - Source syntax, ASTs, name resolution, or type checking.
//! - Classical instructions or classical IR.
//! - Quantum gates, measurement semantics, or quantum IR.
//! - Tensor, HDL, accelerator, or other domain-specific operations.
//! - Execution-dependency edge semantics.
//! - Physical device allocation, placement, routing, or scheduling.
//! - Runtime handles, vendor SDKs, QEC, calibration, or pulse generation.
//!
//! Execution dependencies and typed graph edges belong to
//! `graph/edges.rs`. Graph construction, graph-wide verification, and
//! read-only analysis belong to their corresponding modules.
//!
//! ## Canonical integration
//!
//! - `super::super::ids` owns hybrid graph and node identifiers.
//! - `super::super::version` owns hybrid schema and format versions.
//! - `super::super::errors` owns structured hybrid errors.
//! - `super::super::config` owns explicit operational budgets.
//! - `super::super::domain::composition` owns domain instances and their
//!   structural containment.
//! - `super::super::contract` owns domain interface contracts.
//! - `quantum::ir` remains the canonical quantum semantic boundary.
//! - Existing canonical classical and other domain representations remain
//!   authoritative for their respective operations.
//!
//! No domain IR is copied into this graph merely to make it appear uniform.
//!
//! ## Scalability
//!
//! Collections grow according to available resources. No architectural
//! maximum is imposed on graph nodes, domains, metadata entries, or the
//! number of supported computational domains.
//!
//! Optional operational budgets are enforced only when supplied through
//! `HybridConfig`.
//!
//! Every concrete process remains subject to available memory, address space,
//! allocation behavior, and execution time. Exhaustion must not be treated as
//! proof that a source program is semantically invalid.
//!
//! ## Determinism
//!
//! Metadata uses `BTreeMap` to provide deterministic key ordering.
//! Node declaration order is preserved. Callers needing canonical node order
//! can use `nodes_canonical_order()`.
//!
//! Node identities are identities, not content hashes. Reproducible graph
//! fingerprints, if needed, must be implemented by the canonical fingerprint
//! subsystem using an explicitly defined canonical encoding.
//!
//! ## Trust boundary
//!
//! Deserialization validates local graph-model invariants and domain
//! composition structure. It does not prove that referenced domain entities
//! exist, that an implementation fulfils its contract, or that a target can
//! execute the computation.
//!
//! Those properties require downstream verification, registry resolution,
//! semantic validation, and target feasibility analysis.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::super::config::{BudgetKind, HybridConfig};
use super::super::domain::composition::DomainComposition;
use super::super::errors::{HybridError, HybridErrorCode};
use super::super::ids::{
    DomainInstanceId,
    HybridGraphId,
    HybridNodeId,
};
use super::super::version::{
    HybridFormatVersion,
    HybridSchemaVersion,
    CURRENT_HYBRID_FORMAT_VERSION,
    CURRENT_HYBRID_SCHEMA_VERSION,
};

/// A namespaced reference to an entity owned by a canonical domain
/// representation.
///
/// `representation` identifies the owning representation or dialect.
/// `entity` identifies the referenced entity within that representation.
///
/// These are opaque, extensible identifiers. This type deliberately does
/// not import, copy, or interpret classical, quantum, tensor, HDL, or other
/// domain-specific IR objects.
///
/// Examples of possible namespaces include `quantum.ir`, `classical.ir`,
/// `tensor.ir`, and `hdl.ir`. These are examples, not a closed registry.
///
/// Constructing a reference does not establish that the referenced entity
/// exists. Resolution is the responsibility of the relevant integration
/// adapter or canonical IR owner.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "CanonicalIrRefWire")]
pub struct CanonicalIrRef {
    representation: String,
    entity: String,
}

/// Deserialization representation for a canonical IR reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CanonicalIrRefWire {
    representation: String,
    entity: String,
}

impl TryFrom<CanonicalIrRefWire> for CanonicalIrRef {
    type Error = HybridError;

    fn try_from(value: CanonicalIrRefWire) -> Result<Self, Self::Error> {
        Self::new(value.representation, value.entity)
    }
}

impl CanonicalIrRef {
    /// Construct a reference to an entity in a canonical representation.
    ///
    /// The identifiers must be non-empty and must not contain control
    /// characters. No fixed identifier length or domain inventory is imposed.
    pub fn new(
        representation: impl Into<String>,
        entity: impl Into<String>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            representation: representation.into(),
            entity: entity.into(),
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the representation namespace.
    #[must_use]
    pub fn representation(&self) -> &str {
        &self.representation
    }

    /// Return the opaque entity identifier.
    #[must_use]
    pub fn entity(&self) -> &str {
        &self.entity
    }

    /// Validate the reference's local structural invariants.
    ///
    /// This does not resolve the reference against a canonical IR.
    pub fn validate(&self) -> Result<(), HybridError> {
        validate_identifier("canonical representation", &self.representation)?;
        validate_identifier("canonical entity", &self.entity)?;
        Ok(())
    }

    fn estimated_metadata_bytes(&self) -> Result<u128, HybridError> {
        checked_byte_sum([
            self.representation.len(),
            self.entity.len(),
        ])
    }
}

impl fmt::Display for CanonicalIrRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.representation, self.entity)
    }
}

/// One node in the hybrid composition graph.
///
/// A node identifies a participating canonical entity and the domain
/// instance through which that entity participates in this composition.
///
/// It does not duplicate the referenced operation's implementation or
/// domain-specific semantics.
///
/// The node's `label` and `metadata` are descriptive only. They must not
/// override the referenced entity's semantic meaning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "HybridNodeWire")]
pub struct HybridNode {
    id: HybridNodeId,
    domain_instance: DomainInstanceId,
    operation: CanonicalIrRef,
    label: Option<String>,
    metadata: BTreeMap<String, String>,
}

/// Deserialization representation for a hybrid node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct HybridNodeWire {
    id: HybridNodeId,
    domain_instance: DomainInstanceId,
    operation: CanonicalIrRef,
    label: Option<String>,
    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl TryFrom<HybridNodeWire> for HybridNode {
    type Error = HybridError;

    fn try_from(value: HybridNodeWire) -> Result<Self, Self::Error> {
        Self::new(
            value.id,
            value.domain_instance,
            value.operation,
            value.label,
            value.metadata,
        )
    }
}

impl HybridNode {
    /// Construct and locally validate a hybrid node.
    ///
    /// The supplied identifiers must refer to the same graph. Existence of
    /// the domain instance and canonical IR entity is checked when the node
    /// is integrated into a complete graph and its references are resolved.
    pub fn new(
        id: HybridNodeId,
        domain_instance: DomainInstanceId,
        operation: CanonicalIrRef,
        label: Option<String>,
        metadata: BTreeMap<String, String>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            id,
            domain_instance,
            operation,
            label,
            metadata,
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the node identity.
    #[must_use]
    pub const fn id(&self) -> HybridNodeId {
        self.id
    }

    /// Return the participating domain instance.
    #[must_use]
    pub const fn domain_instance(&self) -> DomainInstanceId {
        self.domain_instance
    }

    /// Return the canonical domain entity reference.
    #[must_use]
    pub fn operation(&self) -> &CanonicalIrRef {
        &self.operation
    }

    /// Return the optional descriptive label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Return deterministic descriptive metadata.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validate local node invariants.
    ///
    /// Graph membership and reference existence require whole-graph
    /// validation and canonical IR resolution.
    pub fn validate(&self) -> Result<(), HybridError> {
        if self.id.graph() != self.domain_instance.graph() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidReference,
                format!(
                    "hybrid node {} and domain instance {} belong to different graphs",
                    self.id, self.domain_instance
                ),
            ));
        }

        self.operation
            .validate()
            .map_err(|error| error.in_component("hybrid graph node"))?;

        if let Some(label) = &self.label {
            validate_text("hybrid node label", label)?;
        }

        validate_metadata(&self.metadata, "hybrid node metadata")?;

        Ok(())
    }

    fn estimated_metadata_bytes(&self) -> Result<u128, HybridError> {
        let mut total = self.operation.estimated_metadata_bytes()?;

        if let Some(label) = &self.label {
            total = checked_add(total, label.len() as u128)?;
        }

        total = checked_add(
            total,
            estimated_map_bytes(&self.metadata)?,
        )?;

        Ok(total)
    }
}

/// The canonical domain-neutral hybrid computation graph.
///
/// `composition` declares participating domain instances and their
/// structural containment. `nodes` references canonical domain entities.
/// Execution dependencies and their semantics are represented separately
/// by the graph edge subsystem.
///
/// The model intentionally allows an empty graph as an intermediate
/// representation. A compiler or execution entry point must decide whether
/// an empty graph is meaningful for its particular operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "HybridGraphWire")]
pub struct HybridGraph {
    id: HybridGraphId,
    schema_version: HybridSchemaVersion,
    format_version: HybridFormatVersion,
    composition: DomainComposition,
    nodes: Vec<HybridNode>,
    metadata: BTreeMap<String, String>,
}

/// Deserialization representation for a complete hybrid graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct HybridGraphWire {
    id: HybridGraphId,
    schema_version: HybridSchemaVersion,
    format_version: HybridFormatVersion,
    composition: DomainComposition,
    #[serde(default)]
    nodes: Vec<HybridNode>,
    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl TryFrom<HybridGraphWire> for HybridGraph {
    type Error = HybridError;

    fn try_from(value: HybridGraphWire) -> Result<Self, Self::Error> {
        Self::new_with_versions(
            value.id,
            value.schema_version,
            value.format_version,
            value.composition,
            value.nodes,
            value.metadata,
        )
    }
}

impl HybridGraph {
    /// Construct a graph using the currently declared schema and format
    /// versions.
    ///
    /// This constructor validates graph-local invariants. It does not resolve
    /// referenced canonical IR entities or claim target feasibility.
    pub fn new(
        id: HybridGraphId,
        composition: DomainComposition,
        nodes: Vec<HybridNode>,
        metadata: BTreeMap<String, String>,
    ) -> Result<Self, HybridError> {
        Self::new_with_versions(
            id,
            CURRENT_HYBRID_SCHEMA_VERSION,
            CURRENT_HYBRID_FORMAT_VERSION,
            composition,
            nodes,
            metadata,
        )
    }

    /// Construct a graph with explicitly supplied schema and format versions.
    ///
    /// This is intended for controlled interchange and migration code.
    /// Merely supplying a version does not establish that the current process
    /// supports that version. Compatibility must be checked separately.
    pub fn new_with_versions(
        id: HybridGraphId,
        schema_version: HybridSchemaVersion,
        format_version: HybridFormatVersion,
        composition: DomainComposition,
        nodes: Vec<HybridNode>,
        metadata: BTreeMap<String, String>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            id,
            schema_version,
            format_version,
            composition,
            nodes,
            metadata,
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the graph identity.
    #[must_use]
    pub const fn id(&self) -> HybridGraphId {
        self.id
    }

    /// Return the logical schema version.
    #[must_use]
    pub const fn schema_version(&self) -> HybridSchemaVersion {
        self.schema_version
    }

    /// Return the interchange format version.
    #[must_use]
    pub const fn format_version(&self) -> HybridFormatVersion {
        self.format_version
    }

    /// Return the validated domain composition.
    #[must_use]
    pub const fn composition(&self) -> &DomainComposition {
        &self.composition
    }

    /// Return nodes in declaration order.
    #[must_use]
    pub fn nodes(&self) -> &[HybridNode] {
        &self.nodes
    }

    /// Return descriptive graph metadata in deterministic key order.
    #[must_use]
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Return the number of nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return whether this graph has no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Return nodes sorted by their typed identifiers.
    ///
    /// The returned vector contains references to the original nodes and
    /// does not mutate or reorder the graph.
    #[must_use]
    pub fn nodes_canonical_order(&self) -> Vec<&HybridNode> {
        let mut nodes: Vec<&HybridNode> = self.nodes.iter().collect();
        nodes.sort_by_key(|node| node.id());
        nodes
    }

    /// Look up a node by identity.
    ///
    /// This is a linear lookup. Callers performing many repeated lookups
    /// should build an index in the graph-analysis or builder subsystem
    /// rather than storing a second authoritative copy in this model.
    #[must_use]
    pub fn node(&self, id: HybridNodeId) -> Option<&HybridNode> {
        self.nodes.iter().find(|node| node.id() == id)
    }

    /// Validate all graph-model structural invariants.
    ///
    /// This checks:
    /// - graph identity consistency;
    /// - domain-composition validity;
    /// - node identity uniqueness;
    /// - domain-instance reference existence;
    /// - node-to-domain graph ownership;
    /// - local metadata and canonical-reference validity.
    ///
    /// It does not resolve canonical IR entities, verify typed execution
    /// edges, or establish that any target can realize this graph.
    pub fn validate(&self) -> Result<(), HybridError> {
        if self.composition.graph_id() != self.id {
            return Err(HybridError::new(
                HybridErrorCode::InvalidReference,
                format!(
                    "hybrid graph identity {} does not match composition identity {}",
                    self.id,
                    self.composition.graph_id()
                ),
            ));
        }

        self.composition
            .validate()
            .map_err(|error| error.in_component("hybrid graph composition"))?;

        validate_metadata(&self.metadata, "hybrid graph metadata")?;

        let mut domain_ids = BTreeSet::new();

        for instance in self.composition.instances() {
            if instance.id().graph() != self.id {
                return Err(HybridError::new(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "domain instance {} does not belong to graph {}",
                        instance.id(),
                        self.id
                    ),
                ));
            }

            if !domain_ids.insert(instance.id()) {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!(
                        "domain instance identity {} occurs more than once",
                        instance.id()
                    ),
                ));
            }
        }

        let mut node_ids = BTreeSet::new();

        for node in &self.nodes {
            node.validate()
                .map_err(|error| error.in_component("hybrid graph"))?;

            if node.id().graph() != self.id {
                return Err(HybridError::new(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "node {} does not belong to graph {}",
                        node.id(),
                        self.id
                    ),
                ));
            }

            if !node_ids.insert(node.id()) {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!("node identity {} occurs more than once", node.id()),
                ));
            }

            if !domain_ids.contains(&node.domain_instance()) {
                return Err(HybridError::new(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "node {} references missing domain instance {}",
                        node.id(),
                        node.domain_instance()
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Validate the graph using caller-supplied operational budgets.
    ///
    /// `None` budgets impose no additional configured limit. The method
    /// checks graph-node count, domain-instance count, and estimated
    /// descriptive metadata bytes. These estimates are not exact heap sizes.
    ///
    /// This method does not change program meaning to fit a budget.
    pub fn validate_with_config(
        &self,
        config: &HybridConfig,
    ) -> Result<(), HybridError> {
        self.validate()?;

        config
            .check_budget(BudgetKind::GraphNodes, self.nodes.len() as u128)
            .map_err(|error| error.in_component("hybrid graph"))?;

        config
            .check_budget(
                BudgetKind::DomainInstances,
                self.composition.len() as u128,
            )
            .map_err(|error| error.in_component("hybrid graph"))?;

        let mut metadata_bytes = estimated_map_bytes(&self.metadata)?;

        for node in &self.nodes {
            metadata_bytes = checked_add(
                metadata_bytes,
                node.estimated_metadata_bytes()?,
            )?;
        }

        config
            .check_budget(BudgetKind::MetadataBytes, metadata_bytes)
            .map_err(|error| error.in_component("hybrid graph"))?;

        self.composition
            .validate_with_config(config)
            .map_err(|error| error.in_component("hybrid graph composition"))?;

        Ok(())
    }
}

/// Validate an extensible identifier.
///
/// No fixed identifier-length ceiling is imposed here. A deployment that
/// needs input-size protection must configure and enforce an explicit
/// operational budget at the appropriate input boundary.
fn validate_identifier(
    field: &str,
    value: &str,
) -> Result<(), HybridError> {
    if value.trim().is_empty() {
        return Err(HybridError::new(
            HybridErrorCode::InvalidRepresentation,
            format!("{field} must not be empty"),
        ));
    }

    if value.chars().any(char::is_control) {
        return Err(HybridError::new(
            HybridErrorCode::InvalidRepresentation,
            format!("{field} must not contain control characters"),
        ));
    }

    Ok(())
}

/// Validate descriptive text without assigning it computational semantics.
fn validate_text(
    field: &str,
    value: &str,
) -> Result<(), HybridError> {
    if value.chars().any(char::is_control) {
        return Err(HybridError::new(
            HybridErrorCode::InvalidRepresentation,
            format!("{field} must not contain control characters"),
        ));
    }

    Ok(())
}

/// Validate graph- or node-level metadata.
fn validate_metadata(
    metadata: &BTreeMap<String, String>,
    field: &str,
) -> Result<(), HybridError> {
    for (key, value) in metadata {
        validate_identifier("metadata key", key)
            .map_err(|error| error.in_component(field))?;

        validate_text("metadata value", value)
            .map_err(|error| error.in_component(field))?;
    }

    Ok(())
}

/// Estimate UTF-8 bytes occupied by metadata key/value contents.
///
/// This deliberately excludes allocator overhead, map nodes, capacities,
/// struct fields, and serialization framing.
fn estimated_map_bytes(
    metadata: &BTreeMap<String, String>,
) -> Result<u128, HybridError> {
    let mut total = 0_u128;

    for (key, value) in metadata {
        total = checked_add(total, key.len() as u128)?;
        total = checked_add(total, value.len() as u128)?;
    }

    Ok(total)
}

fn checked_byte_sum<const N: usize>(
    parts: [usize; N],
) -> Result<u128, HybridError> {
    let mut total = 0_u128;

    for part in parts {
        total = checked_add(total, part as u128)?;
    }

    Ok(total)
}

fn checked_add(
    left: u128,
    right: u128,
) -> Result<u128, HybridError> {
    left.checked_add(right).ok_or_else(|| {
        HybridError::new(
            HybridErrorCode::InvalidRepresentation,
            "hybrid graph metadata accounting overflowed its numeric representation",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hybrid::contract::DomainInterfaceRef;
    use crate::hybrid::domain::composition::{DomainComposition, DomainInstance};
    use crate::hybrid::ids::{GraphScopedId, IdComponent, IdAllocator};
    use crate::hybrid::version::INITIAL_DOMAIN_CONTRACT_VERSION;

    use std::collections::BTreeMap;

    fn graph_id() -> HybridGraphId {
        HybridGraphId::new(1).expect("non-zero graph ID")
    }

    fn scoped_node_id(graph: HybridGraphId, local: u128) -> HybridNodeId {
        HybridNodeId::new(GraphScopedId::new(
            graph,
            IdComponent::new(local).expect("non-zero node ID"),
        ))
    }

    fn scoped_domain_id(
        graph: HybridGraphId,
        local: u128,
    ) -> DomainInstanceId {
        DomainInstanceId::new(GraphScopedId::new(
            graph,
            IdComponent::new(local).expect("non-zero domain ID"),
        ))
    }

    fn sample_composition() -> DomainComposition {
        let graph = graph_id();
        let domain_id = scoped_domain_id(graph, 2);

        let interface = DomainInterfaceRef::new(
            "classical",
            "function",
            INITIAL_DOMAIN_CONTRACT_VERSION,
        )
        .expect("valid interface reference");

        let instance = DomainInstance::new(
            domain_id,
            interface,
            None,
            Some("classical-stage".to_owned()),
        )
        .expect("valid domain instance");

        DomainComposition::new(graph, vec![instance])
            .expect("valid composition")
    }

    fn sample_node() -> HybridNode {
        let graph = graph_id();

        HybridNode::new(
            scoped_node_id(graph, 3),
            scoped_domain_id(graph, 2),
            CanonicalIrRef::new("classical.ir", "function:main")
                .expect("valid canonical reference"),
            Some("main".to_owned()),
            BTreeMap::new(),
        )
        .expect("valid node")
    }

    fn sample_graph() -> HybridGraph {
        HybridGraph::new(
            graph_id(),
            sample_composition(),
            vec![sample_node()],
            BTreeMap::new(),
        )
        .expect("valid graph")
    }

    #[test]
    fn canonical_ir_reference_is_extensible() {
        let reference =
            CanonicalIrRef::new("future.domain.ir", "entity-1")
                .expect("extensible namespace");

        assert_eq!(reference.representation(), "future.domain.ir");
        assert_eq!(reference.entity(), "entity-1");
    }

    #[test]
    fn canonical_ir_reference_rejects_empty_namespace() {
        assert!(CanonicalIrRef::new("", "entity-1").is_err());
    }

    #[test]
    fn node_rejects_cross_graph_domain_reference() {
        let graph_a = HybridGraphId::new(1).unwrap();
        let graph_b = HybridGraphId::new(2).unwrap();

        let result = HybridNode::new(
            scoped_node_id(graph_a, 1),
            scoped_domain_id(graph_b, 1),
            CanonicalIrRef::new("classical.ir", "function:main").unwrap(),
            None,
            BTreeMap::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn graph_validates_domain_and_node_references() {
        let graph = sample_graph();

        assert!(graph.validate().is_ok());
        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.node(scoped_node_id(graph_id(), 3)).unwrap().label(), Some("main"));
    }

    #[test]
    fn graph_rejects_composition_with_different_identity() {
        let other_graph = HybridGraphId::new(99).unwrap();

        let result = HybridGraph::new(
            other_graph,
            sample_composition(),
            Vec::new(),
            BTreeMap::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn graph_rejects_duplicate_node_identity() {
        let node = sample_node();

        let result = HybridGraph::new(
            graph_id(),
            sample_composition(),
            vec![node.clone(), node],
            BTreeMap::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn graph_rejects_nodes_referencing_unknown_domains() {
        let graph = graph_id();

        let node = HybridNode::new(
            scoped_node_id(graph, 3),
            scoped_domain_id(graph, 404),
            CanonicalIrRef::new("classical.ir", "function:main").unwrap(),
            None,
            BTreeMap::new(),
        )
        .unwrap();

        let result = HybridGraph::new(
            graph,
            sample_composition(),
            vec![node],
            BTreeMap::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn canonical_node_order_is_independent_of_declaration_order() {
        let graph = graph_id();
        let composition = sample_composition();

        let first = HybridNode::new(
            scoped_node_id(graph, 20),
            scoped_domain_id(graph, 2),
            CanonicalIrRef::new("classical.ir", "function:b").unwrap(),
            None,
            BTreeMap::new(),
        )
        .unwrap();

        let second = HybridNode::new(
            scoped_node_id(graph, 10),
            scoped_domain_id(graph, 2),
            CanonicalIrRef::new("classical.ir", "function:a").unwrap(),
            None,
            BTreeMap::new(),
        )
        .unwrap();

        let model = HybridGraph::new(
            graph,
            composition,
            vec![first, second],
            BTreeMap::new(),
        )
        .unwrap();

        let ordered = model.nodes_canonical_order();

        assert_eq!(ordered[0].id().local().get(), 10);
        assert_eq!(ordered[1].id().local().get(), 20);
    }

    #[test]
    fn metadata_order_is_canonical() {
        let mut metadata = BTreeMap::new();
        metadata.insert("zeta".to_owned(), "last".to_owned());
        metadata.insert("alpha".to_owned(), "first".to_owned());

        let model = HybridGraph::new(
            graph_id(),
            sample_composition(),
            vec![sample_node()],
            metadata,
        )
        .unwrap();

        let keys: Vec<&str> =
            model.metadata().keys().map(String::as_str).collect();

        assert_eq!(keys, vec!["alpha", "zeta"]);
    }

    #[test]
    fn graph_round_trips_through_serde_json() {
        let original = sample_graph();

        let encoded = serde_json::to_string(&original)
            .expect("serialize graph");

        let decoded: HybridGraph = serde_json::from_str(&encoded)
            .expect("deserialize and validate graph");

        assert_eq!(decoded, original);
    }

    #[test]
    fn invalid_deserialized_graph_is_rejected() {
        let graph = sample_graph();
        let encoded = serde_json::to_value(&graph).unwrap();

        let mut invalid = encoded;
        invalid["nodes"][0]["domain_instance"]["graph"] =
            serde_json::json!(99);

        let decoded: Result<HybridGraph, _> =
            serde_json::from_value(invalid);

        assert!(decoded.is_err());
    }

    #[test]
    fn explicit_graph_node_budget_is_enforced() {
        let mut config = HybridConfig::default();
        config.budgets.max_graph_nodes = Some(0);

        let graph = sample_graph();

        assert!(graph.validate().is_ok());
        assert!(graph.validate_with_config(&config).is_err());
    }

    #[test]
    fn empty_graph_is_a_valid_intermediate_model() {
        let graph = graph_id();

        let empty_composition =
            DomainComposition::new(graph, Vec::new()).unwrap();

        let model = HybridGraph::new(
            graph,
            empty_composition,
            Vec::new(),
            BTreeMap::new(),
        )
        .unwrap();

        assert!(model.is_empty());
        assert_eq!(model.node_count(), 0);
    }

    #[test]
    fn allocator_integrates_with_graph_scoped_node_ids() {
        let allocator = IdAllocator::new();
        let graph = allocator.allocate_graph_id().unwrap();
        let node = allocator.allocate_node_id(graph).unwrap();

        assert_eq!(node.graph(), graph);
    }
}
