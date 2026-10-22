
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Graph Builder
//!
//! Path: `src/hybrid/graph/builder.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Incrementally construct a structurally valid hybrid computation graph
//! from an existing domain composition, canonical domain-IR references,
//! and explicit dependency edges.
//!
//! The builder is a composition facility, not another IR. It does not copy
//! classical instructions, quantum operations, tensor operations, HDL
//! structures, or other domain-specific computation into a new representation.
//!
//! ## Ownership
//!
//! This module owns:
//! - Incremental construction of `HybridGraph` and `HybridEdgeSet`.
//! - Builder-local uniqueness indexes for node, edge, and dependency IDs.
//! - Early validation of graph ownership and referenced domain instances.
//! - Enforcement of explicitly configured graph, edge, metadata, and work
//!   budgets where the underlying canonical interfaces expose those checks.
//! - Final structural validation of the graph and edge set.
//!
//! It does not own:
//! - Source syntax, ASTs, semantic analysis, or type checking.
//! - Domain contracts or the domain registry.
//! - Classical or quantum IR.
//! - Dataflow semantics beyond the declared `EdgeKind`.
//! - General graph-wide cycle legality or complete semantic verification.
//! - Hardware discovery, placement, scheduling, routing, QEC, or codegen.
//! - Runtime sessions or execution.
//!
//! ## Integration
//!
//! - `super::model` owns `HybridGraph`, `HybridNode`, and `CanonicalIrRef`.
//! - `super::edges` owns `HybridEdge`, `EdgeKind`, and `HybridEdgeSet`.
//! - `crate::hybrid::domain::composition` owns `DomainComposition`.
//! - `crate::hybrid::ids` owns all hybrid identities and their allocator.
//! - `crate::hybrid::config` owns operational budgets.
//! - `crate::hybrid::errors` owns structured hybrid errors.
//! - `graph/verify.rs`, when implemented and wired, must add semantic
//!   verification without duplicating this builder's structural ownership.
//! - Compiler and runtime adapters consume the built graph and edge set.
//!
//! ## Scalability
//!
//! No fixed node, edge, domain, metadata-entry, or nesting ceiling is
//! introduced here. Caller-configured budgets are operational policies,
//! not language-semantic limits. Actual memory, allocation, address-space,
//! and identifier-space exhaustion remains possible and is reported.
//!
//! ## Determinism
//!
//! Node and edge declaration order is preserved. ID allocation order is
//! determined by the supplied shared allocator and the sequence of calls.
//! Canonical sorted views remain the responsibility of the model and edge
//! collection APIs.
//!
//! ## Construction guarantees
//!
//! `build` establishes local graph-model and edge-set structural validity.
//! It does not claim that referenced domain IR entities resolve, that domain
//! contracts are registered, that all extension semantics are understood,
//! or that every cycle is semantically legal.

use std::collections::BTreeSet;

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::domain::composition::DomainComposition;
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::ids::{
    DomainInstanceId,
    HybridDependencyId,
    HybridEdgeId,
    HybridGraphId,
    HybridNodeId,
    IdAllocator,
    IdError,
};

use super::edges::{
    EdgeKind,
    EdgeValidationReport,
    HybridEdge,
    HybridEdgeSet,
};
use super::model::{CanonicalIrRef, HybridGraph, HybridNode};

/// A validated graph and its separately owned dependency edges.
///
/// The repository's current `HybridGraph` stores nodes and domain
/// composition, while `HybridEdgeSet` stores dependency relationships.
/// This result intentionally carries both objects together rather than
/// creating a second graph representation or changing either type's owner.
///
/// Construction through [`HybridGraphBuilder::build`] validates structural
/// graph references and edge relationships. It does not imply complete
/// semantic verification or target feasibility.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltHybridGraph {
    graph: HybridGraph,
    edges: HybridEdgeSet,
    edge_validation: EdgeValidationReport,
}

impl BuiltHybridGraph {
    /// Return the canonical graph model.
    #[must_use]
    pub const fn graph(&self) -> &HybridGraph {
        &self.graph
    }

    /// Return the canonical edge set associated with this graph.
    #[must_use]
    pub const fn edges(&self) -> &HybridEdgeSet {
        &self.edges
    }

    /// Return the result of structural edge validation.
    ///
    /// Optional extension edges whose semantics are not understood remain
    /// explicitly listed in this report.
    #[must_use]
    pub const fn edge_validation(&self) -> &EdgeValidationReport {
        &self.edge_validation
    }

    /// Consume the result and return the canonical graph and edge set.
    #[must_use]
    pub fn into_parts(self) -> (HybridGraph, HybridEdgeSet) {
        (self.graph, self.edges)
    }

    /// Consume the result and return all constructed parts.
    #[must_use]
    pub fn into_parts_with_report(
        self,
    ) -> (HybridGraph, HybridEdgeSet, EdgeValidationReport) {
        (self.graph, self.edges, self.edge_validation)
    }
}

/// Incremental builder for a canonical hybrid graph and its edges.
///
/// The builder receives a pre-existing, structurally validated
/// `DomainComposition`. Domain instances are not silently created or inferred
/// from operation names. Every node must explicitly reference an instance
/// in that composition.
///
/// Clone the supplied `IdAllocator` when other construction components
/// must share the same allocation sequence. The allocator does not promise
/// content-derived or schedule-independent IDs.
#[derive(Debug, Clone)]
pub struct HybridGraphBuilder {
    graph_id: HybridGraphId,
    composition: DomainComposition,
    allocator: IdAllocator,
    config: HybridConfig,
    nodes: Vec<HybridNode>,
    edges: Vec<HybridEdge>,
    node_ids: BTreeSet<HybridNodeId>,
    edge_ids: BTreeSet<HybridEdgeId>,
    dependency_ids: BTreeSet<HybridDependencyId>,
    domain_instance_ids: BTreeSet<DomainInstanceId>,
    graph_metadata: std::collections::BTreeMap<String, String>,
    work_units: u128,
}

impl HybridGraphBuilder {
    /// Create a builder from a validated domain composition.
    ///
    /// The graph identity is taken from `composition`; callers cannot supply
    /// a second graph ID that might disagree with it.
    ///
    /// The composition is validated against the supplied configuration before
    /// the builder is created. The builder does not resolve interface
    /// references against a domain registry.
    pub fn new(
        composition: DomainComposition,
        allocator: IdAllocator,
        config: HybridConfig,
    ) -> Result<Self, HybridError> {
        composition
            .validate_with_config(&config)
            .map_err(|error| error.in_component("hybrid graph builder initialization"))?;

        let graph_id = composition.graph_id();
        let domain_instance_ids = composition
            .instances()
            .iter()
            .map(|instance| instance.id())
            .collect();

        Ok(Self {
            graph_id,
            composition,
            allocator,
            config,
            nodes: Vec::new(),
            edges: Vec::new(),
            node_ids: BTreeSet::new(),
            edge_ids: BTreeSet::new(),
            dependency_ids: BTreeSet::new(),
            domain_instance_ids,
            graph_metadata: std::collections::BTreeMap::new(),
            work_units: 0,
        })
    }

    /// Return the graph identity shared by all constructed entities.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Return the explicit operational configuration used by this builder.
    #[must_use]
    pub const fn config(&self) -> &HybridConfig {
        &self.config
    }

    /// Return the number of nodes currently staged.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of edges currently staged.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Return whether no nodes have been staged.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Return the domain composition supplied at initialization.
    #[must_use]
    pub const fn composition(&self) -> &DomainComposition {
        &self.composition
    }

    /// Set or replace descriptive graph metadata.
    ///
    /// Metadata is descriptive only. It cannot override operation semantics,
    /// dependencies, capabilities, resource requirements, or target choice.
    /// Identifier validation and the final metadata budget check are enforced
    /// by the canonical graph model when `build` is called.
    ///
    /// A replaced key is not counted as an additional metadata entry.
    pub fn set_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), HybridError> {
        self.reserve_work(1)?;
        self.graph_metadata.insert(key.into(), value.into());
        Ok(())
    }

    /// Add a node by allocating a graph-scoped node identity.
    ///
    /// The `operation` remains a reference to a canonical domain
    /// representation; the builder never copies or interprets that IR.
    ///
    /// The referenced `domain_instance` must already belong to the supplied
    /// composition. A failed operation does not append a node.
    pub fn add_node(
        &mut self,
        domain_instance: DomainInstanceId,
        operation: CanonicalIrRef,
        label: Option<String>,
        metadata: std::collections::BTreeMap<String, String>,
    ) -> Result<HybridNodeId, HybridError> {
        self.check_node_budget(self.nodes.len().saturating_add(1))?;
        self.reserve_work(1)?;

        let id = self.allocate_node_id()?;
        let node = HybridNode::new(
            id,
            domain_instance,
            operation,
            label,
            metadata,
        )
        .map_err(|error| error.in_component("hybrid graph builder add_node"))?;

        self.insert_node_after_checks(node)
    }

    /// Insert an existing node identity, primarily for import or migration.
    ///
    /// The node must belong to this graph, reference a declared domain
    /// instance, and not duplicate an already staged node identity.
    ///
    /// This method preserves the caller's identity. It does not allocate a
    /// replacement ID or silently repair an invalid reference.
    pub fn insert_node(
        &mut self,
        node: HybridNode,
    ) -> Result<HybridNodeId, HybridError> {
        self.check_node_budget(self.nodes.len().saturating_add(1))?;
        self.reserve_work(1)?;
        self.insert_node_after_checks(node)
    }

    /// Add a dependency edge and allocate its edge and dependency identities.
    ///
    /// Both endpoints must already have been inserted. This early check
    /// catches common mistakes without requiring a final graph-wide pass.
    ///
    /// The exact meaning of `kind` is owned by `graph::edges`; this method
    /// does not infer whether a dependency is data, control, ordering,
    /// completion, feedback, or a domain-boundary relationship.
    pub fn add_edge(
        &mut self,
        source: HybridNodeId,
        target: HybridNodeId,
        kind: EdgeKind,
        metadata: std::collections::BTreeMap<String, String>,
    ) -> Result<HybridEdgeId, HybridError> {
        self.check_edge_budget(self.edges.len().saturating_add(1))?;
        self.reserve_work(1)?;
        self.check_edge_endpoints(source, target)?;

        let edge_id = self.allocate_edge_id()?;
        let dependency_id = self.allocate_dependency_id()?;

        let edge = HybridEdge::new(
            edge_id,
            dependency_id,
            source,
            target,
            kind,
            metadata,
        )
        .map_err(|error| error.in_component("hybrid graph builder add_edge"))?;

        self.insert_edge_after_checks(edge)
    }

    /// Insert an existing edge, primarily for artifact import or migration.
    ///
    /// Endpoints must already exist in this builder. Duplicate edge and
    /// dependency identities are rejected.
    pub fn insert_edge(
        &mut self,
        edge: HybridEdge,
    ) -> Result<HybridEdgeId, HybridError> {
        self.check_edge_budget(self.edges.len().saturating_add(1))?;
        self.reserve_work(1)?;
        self.check_edge_endpoints(edge.source(), edge.target())?;
        self.insert_edge_after_checks(edge)
    }

    /// Build the canonical graph and edge set, validating their structure.
    ///
    /// This method:
    /// - creates the canonical `HybridGraph`;
    /// - creates the separate canonical `HybridEdgeSet`;
    /// - enforces graph, edge, domain-instance, metadata and configured work
    ///   budgets through their owning modules;
    /// - validates edge endpoints against the completed graph.
    ///
    /// It does not claim full semantic verification, cycle legality,
    /// registry resolution, or target feasibility. Those checks belong to
    /// their canonical downstream owners.
    pub fn build(self) -> Result<BuiltHybridGraph, HybridError> {
        let graph = HybridGraph::new(
            self.graph_id,
            self.composition,
            self.nodes,
            self.graph_metadata,
        )
        .map_err(|error| error.in_component("hybrid graph builder finalization"))?;

        graph
            .validate_with_config(&self.config)
            .map_err(|error| error.in_component("hybrid graph builder finalization"))?;

        let edges = HybridEdgeSet::new(self.graph_id, self.edges)
            .map_err(|error| error.in_component("hybrid graph builder finalization"))?;

        let edge_validation = edges
            .validate_against_with_config(&graph, &self.config)
            .map_err(|error| error.in_component("hybrid graph builder finalization"))?;

        Ok(BuiltHybridGraph {
            graph,
            edges,
            edge_validation,
        })
    }

    fn insert_node_after_checks(
        &mut self,
        node: HybridNode,
    ) -> Result<HybridNodeId, HybridError> {
        node.validate()
            .map_err(|error| error.in_component("hybrid graph builder node validation"))?;

        if node.id().graph() != self.graph_id {
            return Err(HybridError::invalid_reference(format!(
                "node {} belongs to graph {}, not builder graph {}",
                node.id(),
                node.id().graph(),
                self.graph_id
            )));
        }

        if !self.domain_instance_ids.contains(&node.domain_instance()) {
            return Err(HybridError::invalid_reference(format!(
                "node {} references domain instance {} absent from the builder composition",
                node.id(),
                node.domain_instance()
            )));
        }

        if self.node_ids.contains(&node.id()) {
            return Err(HybridError::new(
                HybridErrorCode::DuplicateIdentity,
                format!("node identity {} has already been inserted", node.id()),
            ));
        }

        let id = node.id();
        self.node_ids.insert(id);
        self.nodes.push(node);
        Ok(id)
    }

    fn insert_edge_after_checks(
        &mut self,
        edge: HybridEdge,
    ) -> Result<HybridEdgeId, HybridError> {
        edge.validate()
            .map_err(|error| error.in_component("hybrid graph builder edge validation"))?;

        if edge.id().graph() != self.graph_id {
            return Err(HybridError::invalid_reference(format!(
                "edge {} belongs to graph {}, not builder graph {}",
                edge.id(),
                edge.id().graph(),
                self.graph_id
            )));
        }

        if edge.dependency_id().graph() != self.graph_id {
            return Err(HybridError::invalid_reference(format!(
                "dependency {} belongs to graph {}, not builder graph {}",
                edge.dependency_id(),
                edge.dependency_id().graph(),
                self.graph_id
            )));
        }

        if self.edge_ids.contains(&edge.id()) {
            return Err(HybridError::new(
                HybridErrorCode::DuplicateIdentity,
                format!("edge identity {} has already been inserted", edge.id()),
            ));
        }

        if self.dependency_ids.contains(&edge.dependency_id()) {
            return Err(HybridError::new(
                HybridErrorCode::DuplicateIdentity,
                format!(
                    "dependency identity {} has already been inserted",
                    edge.dependency_id()
                ),
            ));
        }

        let id = edge.id();
        self.edge_ids.insert(id);
        self.dependency_ids.insert(edge.dependency_id());
        self.edges.push(edge);
        Ok(id)
    }

    fn check_edge_endpoints(
        &self,
        source: HybridNodeId,
        target: HybridNodeId,
    ) -> Result<(), HybridError> {
        if source.graph() != self.graph_id || target.graph() != self.graph_id {
            return Err(HybridError::invalid_reference(format!(
                "edge endpoints must belong to builder graph {}",
                self.graph_id
            )));
        }

        if !self.node_ids.contains(&source) {
            return Err(HybridError::invalid_reference(format!(
                "edge source node {} has not been inserted into this builder",
                source
            )));
        }

        if !self.node_ids.contains(&target) {
            return Err(HybridError::invalid_reference(format!(
                "edge target node {} has not been inserted into this builder",
                target
            )));
        }

        Ok(())
    }

    fn check_node_budget(&self, requested_count: usize) -> Result<(), HybridError> {
        self.config
            .check_budget(BudgetKind::GraphNodes, requested_count as u128)
            .map_err(|error| error.in_component("hybrid graph builder"))
    }

    fn check_edge_budget(&self, requested_count: usize) -> Result<(), HybridError> {
        self.config
            .check_budget(BudgetKind::GraphEdges, requested_count as u128)
            .map_err(|error| error.in_component("hybrid graph builder"))
    }

    fn reserve_work(&mut self, additional: u128) -> Result<(), HybridError> {
        let requested = self.work_units.checked_add(additional).ok_or_else(|| {
            HybridError::new(
                HybridErrorCode::InvalidRepresentation,
                "hybrid graph builder work accounting overflowed",
            )
        })?;

        self.config
            .check_budget(BudgetKind::WorkUnits, requested)
            .map_err(|error| error.in_component("hybrid graph builder"))?;

        self.config
            .check_budget(BudgetKind::ValidationSteps, requested)
            .map_err(|error| error.in_component("hybrid graph builder"))?;

        self.work_units = requested;
        Ok(())
    }

    fn allocate_node_id(&self) -> Result<HybridNodeId, HybridError> {
        self.allocator
            .allocate_node_id(self.graph_id)
            .map_err(|error| map_id_error("node", error))
    }

    fn allocate_edge_id(&self) -> Result<HybridEdgeId, HybridError> {
        self.allocator
            .allocate_edge_id(self.graph_id)
            .map_err(|error| map_id_error("edge", error))
    }

    fn allocate_dependency_id(&self) -> Result<HybridDependencyId, HybridError> {
        self.allocator
            .allocate_dependency_id(self.graph_id)
            .map_err(|error| map_id_error("dependency", error))
    }
}

/// Convert identity-allocation failures into the canonical hybrid error type.
///
/// Identity-space exhaustion is reported explicitly. It is not treated as
/// permission to wrap, reuse an ID, or alter graph semantics.
fn map_id_error(entity: &str, error: IdError) -> HybridError {
    match error {
        IdError::Exhausted => HybridError::new(
            HybridErrorCode::InvalidRepresentation,
            format!("hybrid {entity} identity allocation exhausted: {error}"),
        ),
        IdError::Zero => HybridError::new(
            HybridErrorCode::InvalidRepresentation,
            format!("hybrid {entity} identity allocation rejected zero: {error}"),
        ),
        IdError::AllocatorPoisoned => HybridError::new(
            HybridErrorCode::InternalInvariantViolation,
            format!("hybrid {entity} identity allocator is unavailable: {error}"),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::contract::DomainInterfaceRef;
    use crate::hybrid::ids::GraphScopedId;
    use crate::hybrid::version::DomainContractVersion;
    use std::collections::BTreeMap;

    fn graph_id() -> HybridGraphId {
        HybridGraphId::new(17).expect("non-zero graph ID")
    }

    fn composition(
        allocator: &IdAllocator,
    ) -> (HybridGraphId, DomainComposition, DomainInstanceId) {
        let graph_id = graph_id();
        let instance_id = allocator
            .allocate_domain_instance_id(graph_id)
            .expect("domain instance ID");

        let interface = DomainInterfaceRef::new(
            "classical",
            "computation",
            DomainContractVersion::new(1, 0, 0),
        )
        .expect("valid interface reference");

        let instance = crate::hybrid::domain::composition::DomainInstance::new(
            instance_id,
            interface,
            None,
            None,
        )
        .expect("valid domain instance");

        let composition = DomainComposition::new(graph_id, vec![instance])
            .expect("valid composition");

        (graph_id, composition, instance_id)
    }

    fn operation(name: &str) -> CanonicalIrRef {
        CanonicalIrRef::new("classical.ir", name).expect("valid canonical IR ref")
    }

    fn builder() -> (HybridGraphBuilder, DomainInstanceId) {
        let allocator = IdAllocator::new();
        let (_, composition, domain_instance) = composition(&allocator);

        let builder = HybridGraphBuilder::new(
            composition,
            allocator,
            HybridConfig::default(),
        )
        .expect("valid builder");

        (builder, domain_instance)
    }

    #[test]
    fn builds_an_empty_intermediate_graph() {
        let (builder, _) = builder();
        let built = builder.build().expect("empty intermediate graph is valid");

        assert!(built.graph().is_empty());
        assert!(built.edges().is_empty());
        assert_eq!(built.edge_validation().validated_edges, 0);
    }

    #[test]
    fn adds_nodes_and_preserves_declaration_order() {
        let (mut builder, domain_instance) = builder();

        let first = builder
            .add_node(domain_instance, operation("first"), None, BTreeMap::new())
            .expect("first node");
        let second = builder
            .add_node(domain_instance, operation("second"), None, BTreeMap::new())
            .expect("second node");

        let built = builder.build().expect("valid graph");

        assert_eq!(built.graph().node_count(), 2);
        assert_eq!(built.graph().nodes()[0].id(), first);
        assert_eq!(built.graph().nodes()[1].id(), second);
    }

    #[test]
    fn adds_and_validates_a_dependency_edge() {
        let (mut builder, domain_instance) = builder();

        let source = builder
            .add_node(domain_instance, operation("source"), None, BTreeMap::new())
            .expect("source node");
        let target = builder
            .add_node(domain_instance, operation("target"), None, BTreeMap::new())
            .expect("target node");

        builder
            .add_edge(source, target, EdgeKind::Control, BTreeMap::new())
            .expect("dependency edge");

        let built = builder.build().expect("valid graph and edge set");

        assert_eq!(built.edges().len(), 1);
        assert_eq!(built.edge_validation().validated_edges, 1);
    }

    #[test]
    fn rejects_an_edge_to_a_node_not_inserted() {
        let (mut builder, domain_instance) = builder();

        let source = builder
            .add_node(domain_instance, operation("source"), None, BTreeMap::new())
            .expect("source node");

        let unknown = HybridNodeId::new(GraphScopedId::new(
            builder.graph_id(),
            crate::hybrid::ids::IdComponent::new(999).expect("non-zero ID"),
        ));

        let result = builder.add_edge(
            source,
            unknown,
            EdgeKind::Control,
            BTreeMap::new(),
        );

        assert!(result.is_err());
        assert_eq!(builder.edge_count(), 0);
    }

    #[test]
    fn rejects_duplicate_node_identity() {
        let (mut builder, domain_instance) = builder();
        let id = HybridNodeId::new(GraphScopedId::new(
            builder.graph_id(),
            crate::hybrid::ids::IdComponent::new(901).expect("non-zero ID"),
        ));

        let node = HybridNode::new(
            id,
            domain_instance,
            operation("same"),
            None,
            BTreeMap::new(),
        )
        .expect("locally valid node");

        builder.insert_node(node.clone()).expect("first insertion");
        let error = builder
            .insert_node(node)
            .expect_err("duplicate identity must fail");

        assert_eq!(error.code(), HybridErrorCode::DuplicateIdentity);
    }

    #[test]
    fn enforces_explicit_node_and_edge_budgets() {
        let allocator = IdAllocator::new();
        let (_, composition, domain_instance) = composition(&allocator);

        let config = HybridConfig {
            budgets: crate::hybrid::config::OperationalBudgets {
                max_graph_nodes: Some(1),
                max_graph_edges: Some(0),
                ..Default::default()
            },
            ..Default::default()
        };

        let mut builder =
            HybridGraphBuilder::new(composition, allocator, config)
                .expect("valid builder");

        builder
            .add_node(domain_instance, operation("one"), None, BTreeMap::new())
            .expect("first node");

        let node_error = builder
            .add_node(domain_instance, operation("two"), None, BTreeMap::new())
            .expect_err("explicit node budget must be enforced");

        assert_eq!(node_error.code(), HybridErrorCode::BudgetExhausted);

        let edge_error = builder
            .add_edge(
                HybridNodeId::new(GraphScopedId::new(
                    builder.graph_id(),
                    crate::hybrid::ids::IdComponent::new(1).expect("non-zero ID"),
                )),
                HybridNodeId::new(GraphScopedId::new(
                    builder.graph_id(),
                    crate::hybrid::ids::IdComponent::new(2).expect("non-zero ID"),
                )),
                EdgeKind::Control,
                BTreeMap::new(),
            )
            .expect_err("zero-edge budget must be enforced");

        assert_eq!(edge_error.code(), HybridErrorCode::BudgetExhausted);
    }

    #[test]
    fn graph_metadata_is_deterministic() {
        let (mut builder, _) = builder();

        builder
            .set_metadata("z-key", "last")
            .expect("metadata");
        builder
            .set_metadata("a-key", "first")
            .expect("metadata");

        let built = builder.build().expect("valid graph");

        let keys = built.graph().metadata().keys().cloned().collect::<Vec<_>>();
        assert_eq!(keys, vec!["a-key".to_string(), "z-key".to_string()]);
    }

    #[test]
    fn failed_edge_insertion_does_not_change_edge_count() {
        let (mut builder, _) = builder();
        let invalid = HybridNodeId::new(GraphScopedId::new(
            builder.graph_id(),
            crate::hybrid::ids::IdComponent::new(77).expect("non-zero ID"),
        ));

        assert!(
            builder
                .insert_edge(
                    HybridEdge::new(
                        crate::hybrid::ids::HybridEdgeId::new(GraphScopedId::new(
                            builder.graph_id(),
                            crate::hybrid::ids::IdComponent::new(78).expect("non-zero ID"),
                        )),
                        crate::hybrid::ids::HybridDependencyId::new(GraphScopedId::new(
                            builder.graph_id(),
                            crate::hybrid::ids::IdComponent::new(79).expect("non-zero ID"),
                        )),
                        invalid,
                        invalid,
                        EdgeKind::Control,
                        BTreeMap::new(),
                    )
                    .expect("edge-local validation"),
                )
                .is_err()
        );
        assert_eq!(builder.edge_count(), 0);
    }
}
