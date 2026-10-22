
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Abstract Realization Plan
//!
//! Path: `src/hybrid/planning/plan.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Represent an abstract, target-independent realization plan for an
//! already constructed hybrid computation graph.
//!
//! A plan preserves references to canonical domain operations and the
//! dependency relationships that a downstream implementation must respect.
//! It is an immutable description of planning inputs and obligations, not
//! an executable schedule and not a replacement for any canonical IR.
//!
//! ## Ownership
//!
//! This module owns:
//! - The abstract hybrid plan envelope.
//! - Plan-local references to canonical hybrid nodes.
//! - A loss-preserving projection of hybrid dependency relationships.
//! - Structural plan validation.
//! - Consistency checks against the graph and edge set from which a plan
//!   was constructed.
//!
//! This module does not own:
//! - Source syntax, ASTs, type checking, or semantic analysis.
//! - The canonical classical, quantum, tensor, AI, or HDL IR.
//! - Resource requirements, capability discovery, or device allocation.
//! - Target selection, instruction selection, routing, or scheduling.
//! - Runtime execution, synchronization engines, or vendor SDKs.
//! - Serialization format migration or target-specific lowering.
//!
//! ## Canonical integration
//!
//! - `crate::hybrid::graph::model::HybridGraph` is the canonical composition
//!   graph.
//! - `crate::hybrid::graph::model::CanonicalIrRef` identifies an operation
//!   owned by a canonical domain representation.
//! - `crate::hybrid::graph::edges::HybridEdgeSet` is the canonical source
//!   of dependency relationships.
//! - `crate::hybrid::graph::edges::EdgeKind` preserves dependency semantics.
//! - `crate::hybrid::config::HybridConfig` supplies explicit operational
//!   budgets.
//! - `crate::hybrid::errors::HybridError` is the canonical error vocabulary.
//! - `crate::hybrid::ids` supplies graph-scoped identities.
//! - `quantum::ir` remains the canonical quantum semantic boundary.
//! - The existing compiler pipeline and runtime remain responsible for
//!   realization and execution.
//!
//! The plan stores references and dependency projections, not copies of
//! domain operations or domain-specific IR structures.
//!
//! ## Semantics
//!
//! Node and dependency collections are inventories, not execution order.
//! Their canonical ordering is for reproducibility only.
//!
//! A dependency edge preserves its declared direction and kind. This module
//! never assumes that every graph is a DAG: valid feedback and iterative
//! computations can contain cycles.
//!
//! Constructing a plan does not prove that a target can execute it.
//! Canonical IR resolution, semantic verification, capability checking,
//! feasibility, lowering, scheduling, and runtime validation remain
//! downstream responsibilities.
//!
//! ## Scalability
//!
//! This module imposes no universal maximum on nodes, dependencies, domains,
//! or hardware resources. Collections grow according to available resources.
//!
//! The caller may supply `HybridConfig` to enforce an explicit planning-work
//! budget. Exhaustion is reported as an error and never changes program
//! semantics.
//!
//! Finite memory, address space, allocation behavior, and execution time
//! remain real implementation constraints.
//!
//! ## Compatibility
//!
//! The plan is versioned indirectly by the canonical graph's schema and
//! format versions, which remain authoritative. This module does not invent
//! a competing schema-version registry.
//!
//! Persisted plans must be validated against their canonical graph and edge
//! set before they are trusted or passed to a realization backend.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::graph::edges::{EdgeKind, HybridEdgeSet};
use crate::hybrid::graph::model::{CanonicalIrRef, HybridGraph};
use crate::hybrid::ids::{
    DomainInstanceId,
    HybridDependencyId,
    HybridEdgeId,
    HybridGraphId,
    HybridNodeId,
};

/// A plan-local reference to a node in the canonical hybrid graph.
///
/// This type deliberately does not contain the domain operation's
/// implementation, type system, or executable representation.
///
/// `operation` remains an opaque reference to the canonical owner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanNode {
    node_id: HybridNodeId,
    domain_instance: DomainInstanceId,
    operation: CanonicalIrRef,
}

impl PlanNode {
    /// Create a plan node from a node in a validated hybrid graph.
    ///
    /// Prefer [`HybridPlan::from_graph`] when constructing a complete plan.
    /// This constructor is useful to adapters that need to build and inspect
    /// individual plan references.
    pub fn new(
        node_id: HybridNodeId,
        domain_instance: DomainInstanceId,
        operation: CanonicalIrRef,
    ) -> Result<Self, HybridError> {
        if node_id.graph() != domain_instance.graph() {
            return Err(HybridError::invalid_reference(format!(
                "plan node {node_id} and domain instance {domain_instance} \
                 belong to different graphs"
            )));
        }

        operation
            .validate()
            .map_err(|error| error.in_component("hybrid plan node"))?;

        Ok(Self {
            node_id,
            domain_instance,
            operation,
        })
    }

    /// Return the canonical graph node identity.
    #[must_use]
    pub const fn node_id(&self) -> HybridNodeId {
        self.node_id
    }

    /// Return the participating domain instance.
    #[must_use]
    pub const fn domain_instance(&self) -> DomainInstanceId {
        self.domain_instance
    }

    /// Return the opaque reference to the canonical domain operation.
    #[must_use]
    pub const fn operation(&self) -> &CanonicalIrRef {
        &self.operation
    }

    /// Validate the local identity and reference invariants.
    pub fn validate(&self) -> Result<(), HybridError> {
        if self.node_id.graph() != self.domain_instance.graph() {
            return Err(HybridError::invalid_reference(format!(
                "plan node {} and domain instance {} belong to different graphs",
                self.node_id, self.domain_instance
            )));
        }

        self.operation
            .validate()
            .map_err(|error| error.in_component("hybrid plan node"))
    }
}

/// A loss-preserving projection of one canonical graph dependency.
///
/// The referenced edge and dependency identities remain owned by the
/// canonical graph-edge subsystem. This projection allows a consumer to
/// inspect the plan's obligations without copying domain operations.
///
/// `kind` is retained as its canonical typed representation. It must not be
/// reduced to a string or interpreted as a target-specific scheduling
/// directive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanDependency {
    edge_id: HybridEdgeId,
    dependency_id: HybridDependencyId,
    source: HybridNodeId,
    target: HybridNodeId,
    kind: EdgeKind,
}

impl PlanDependency {
    /// Return the canonical edge identity.
    #[must_use]
    pub const fn edge_id(&self) -> HybridEdgeId {
        self.edge_id
    }

    /// Return the canonical dependency identity.
    #[must_use]
    pub const fn dependency_id(&self) -> HybridDependencyId {
        self.dependency_id
    }

    /// Return the prerequisite or causal source node.
    #[must_use]
    pub const fn source(&self) -> HybridNodeId {
        self.source
    }

    /// Return the dependent or causal target node.
    #[must_use]
    pub const fn target(&self) -> HybridNodeId {
        self.target
    }

    /// Return the canonical dependency kind.
    #[must_use]
    pub const fn kind(&self) -> &EdgeKind {
        &self.kind
    }

    fn from_edge(edge: &crate::hybrid::graph::edges::HybridEdge) -> Self {
        Self {
            edge_id: edge.id(),
            dependency_id: edge.dependency_id(),
            source: edge.source(),
            target: edge.target(),
            kind: edge.kind().clone(),
        }
    }

    fn validate(&self, graph_id: HybridGraphId) -> Result<(), HybridError> {
        if self.edge_id.graph() != graph_id
            || self.dependency_id.graph() != graph_id
            || self.source.graph() != graph_id
            || self.target.graph() != graph_id
        {
            return Err(HybridError::invalid_reference(format!(
                "plan dependency {} contains an identity from another graph",
                self.dependency_id
            )));
        }

        if self.source == self.target {
            // Self-dependencies are not rejected here. Feedback and
            // loop-carried relationships can legitimately form cycles.
            // Their legality is determined by canonical graph verification.
        }

        Ok(())
    }
}

/// An abstract hybrid realization plan.
///
/// `nodes` and `dependencies` are canonical inventories. Their stored order
/// is deterministic, but must not be interpreted as execution order.
///
/// A plan is tied to one graph identity. It is not evidence that a target
/// exists, that canonical IR references have been resolved, or that a
/// particular backend can realize the computation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "HybridPlanWire")]
pub struct HybridPlan {
    graph_id: HybridGraphId,
    nodes: Vec<PlanNode>,
    dependencies: Vec<PlanDependency>,
}

/// Private wire representation ensures deserialization cannot bypass the
/// plan's local structural validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HybridPlanWire {
    graph_id: HybridGraphId,
    #[serde(default)]
    nodes: Vec<PlanNode>,
    #[serde(default)]
    dependencies: Vec<PlanDependency>,
}

impl TryFrom<HybridPlanWire> for HybridPlan {
    type Error = HybridError;

    fn try_from(wire: HybridPlanWire) -> Result<Self, Self::Error> {
        Self::new(wire.graph_id, wire.nodes, wire.dependencies)
    }
}

impl HybridPlan {
    /// Construct a plan from its graph identity and explicit inventories.
    ///
    /// This constructor validates local invariants only. To establish
    /// consistency with canonical inputs, call [`Self::validate_against`].
    pub fn new(
        graph_id: HybridGraphId,
        nodes: Vec<PlanNode>,
        dependencies: Vec<PlanDependency>,
    ) -> Result<Self, HybridError> {
        let plan = Self {
            graph_id,
            nodes,
            dependencies,
        };

        plan.validate_local()?;
        Ok(plan)
    }

    /// Construct an abstract plan from a canonical graph and edge set.
    ///
    /// This operation:
    /// - validates the graph;
    /// - validates all dependency endpoints against that graph;
    /// - checks explicit graph and planning-work budgets;
    /// - preserves canonical operation references;
    /// - preserves every edge's identity, direction, and typed kind;
    /// - produces deterministic node and dependency inventories.
    ///
    /// It does not choose devices, allocate resources, lower operations,
    /// construct an execution schedule, or claim target feasibility.
    pub fn from_graph(
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
        config: &HybridConfig,
    ) -> Result<Self, HybridError> {
        graph
            .validate()
            .map_err(|error| error.in_component("hybrid plan input graph"))?;

        let edge_report = edges
            .validate_against(graph)
            .map_err(|error| error.in_component("hybrid plan input dependencies"))?;

        // Budget checks are operational safeguards, not semantic limits.
        config.check_budget(
            BudgetKind::GraphNodes,
            graph.node_count() as u128,
        )?;

        config.check_budget(
            BudgetKind::GraphEdges,
            edges.len() as u128,
        )?;

        // Each node and each edge contributes at least one planning work
        // unit. Use checked arithmetic so overflow never silently wraps.
        let planning_work = (graph.node_count() as u128)
            .checked_add(edges.len() as u128)
            .ok_or_else(|| {
                HybridError::invalid_representation(
                    "hybrid planning-work accounting overflowed",
                )
            })?;

        config.check_budget(BudgetKind::PlanningSteps, planning_work)?;

        // Keep the edge validation report alive as validation evidence.
        // Unknown extension relationships must still be interpreted by the
        // extension-aware verifier before a backend claims full semantics.
        let _edge_validation = edge_report;

        let mut nodes = graph
            .nodes()
            .iter()
            .map(|node| {
                PlanNode::new(
                    node.id(),
                    node.domain_instance(),
                    node.operation().clone(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        nodes.sort_by_key(PlanNode::node_id);

        let mut dependencies: Vec<PlanDependency> =
            edges.edges().iter().map(PlanDependency::from_edge).collect();

        dependencies.sort_by_key(PlanDependency::edge_id);

        Self::new(graph.id(), nodes, dependencies)
    }

    /// Return the graph identity to which this plan belongs.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Return the plan's node inventory in canonical identity order.
    #[must_use]
    pub fn nodes(&self) -> &[PlanNode] {
        &self.nodes
    }

    /// Return the plan's dependency inventory in canonical identity order.
    #[must_use]
    pub fn dependencies(&self) -> &[PlanDependency] {
        &self.dependencies
    }

    /// Return the number of referenced graph nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of preserved dependencies.
    #[must_use]
    pub fn dependency_count(&self) -> usize {
        self.dependencies.len()
    }

    /// Return whether the plan contains no graph nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Look up a node reference by canonical graph-node identity.
    #[must_use]
    pub fn node(&self, id: HybridNodeId) -> Option<&PlanNode> {
        self.nodes
            .binary_search_by_key(&id, PlanNode::node_id)
            .ok()
            .map(|index| &self.nodes[index])
    }

    /// Validate plan-local invariants.
    ///
    /// This method does not prove that referenced nodes and edges exist in
    /// the current graph. Use `validate_against` for that stronger check.
    pub fn validate_local(&self) -> Result<(), HybridError> {
        let mut node_ids = BTreeSet::new();

        for node in &self.nodes {
            node.validate()
                .map_err(|error| error.in_component("hybrid plan"))?;

            if node.node_id().graph() != self.graph_id {
                return Err(HybridError::invalid_reference(format!(
                    "plan node {} does not belong to plan graph {}",
                    node.node_id(),
                    self.graph_id
                )));
            }

            if !node_ids.insert(node.node_id()) {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!(
                        "plan contains duplicate node identity {}",
                        node.node_id()
                    ),
                ));
            }
        }

        let mut edge_ids = BTreeSet::new();
        let mut dependency_ids = BTreeSet::new();

        for dependency in &self.dependencies {
            dependency
                .validate(self.graph_id)
                .map_err(|error| error.in_component("hybrid plan"))?;

            if !node_ids.contains(&dependency.source())
                || !node_ids.contains(&dependency.target())
            {
                return Err(HybridError::invalid_reference(format!(
                    "dependency {} references a node absent from the plan",
                    dependency.dependency_id()
                )));
            }

            if !edge_ids.insert(dependency.edge_id()) {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!(
                        "plan contains duplicate edge identity {}",
                        dependency.edge_id()
                    ),
                ));
            }

            if !dependency_ids.insert(dependency.dependency_id()) {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!(
                        "plan contains duplicate dependency identity {}",
                        dependency.dependency_id()
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Validate that this plan still corresponds exactly to the supplied
    /// canonical graph and edge set.
    ///
    /// This detects stale plans, mismatched graph identities, missing
    /// dependencies, altered endpoints, altered dependency kinds, and
    /// operation-reference changes.
    ///
    /// A successful result establishes structural consistency only. It does
    /// not establish target feasibility or resolve external canonical IR
    /// references.
    pub fn validate_against(
        &self,
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
    ) -> Result<(), HybridError> {
        self.validate_local()?;

        graph
            .validate()
            .map_err(|error| error.in_component("hybrid plan validation"))?;

        edges
            .validate_against(graph)
            .map_err(|error| error.in_component("hybrid plan validation"))?;

        if graph.id() != self.graph_id {
            return Err(HybridError::invalid_reference(format!(
                "plan belongs to graph {}, but supplied graph is {}",
                self.graph_id,
                graph.id()
            )));
        }

        if edges.graph_id() != self.graph_id {
            return Err(HybridError::invalid_reference(format!(
                "plan belongs to graph {}, but supplied edge set is {}",
                self.graph_id,
                edges.graph_id()
            )));
        }

        if graph.node_count() != self.nodes.len() {
            return Err(HybridError::invalid_representation(format!(
                "plan contains {} nodes but canonical graph contains {}",
                self.nodes.len(),
                graph.node_count()
            )));
        }

        if edges.len() != self.dependencies.len() {
            return Err(HybridError::invalid_representation(format!(
                "plan contains {} dependencies but canonical edge set contains {}",
                self.dependencies.len(),
                edges.len()
            )));
        }

        for graph_node in graph.nodes() {
            let planned = self.node(graph_node.id()).ok_or_else(|| {
                HybridError::invalid_reference(format!(
                    "canonical graph node {} is missing from the plan",
                    graph_node.id()
                ))
            })?;

            if planned.domain_instance() != graph_node.domain_instance()
                || planned.operation() != graph_node.operation()
            {
                return Err(HybridError::invalid_representation(format!(
                    "plan reference for node {} differs from the canonical graph",
                    graph_node.id()
                )));
            }
        }

        for edge in edges.edges() {
            let planned = self
                .dependencies
                .binary_search_by_key(&edge.id(), |dependency| dependency.edge_id())
                .ok()
                .map(|index| &self.dependencies[index])
                .ok_or_else(|| {
                    HybridError::invalid_reference(format!(
                        "canonical edge {} is missing from the plan",
                        edge.id()
                    ))
                })?;

            if planned.dependency_id() != edge.dependency_id()
                || planned.source() != edge.source()
                || planned.target() != edge.target()
                || planned.kind() != edge.kind()
            {
                return Err(HybridError::invalid_representation(format!(
                    "plan dependency for edge {} differs from the canonical edge",
                    edge.id()
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_node_rejects_cross_graph_identity() {
        // This test uses the public typed identity constructors rather than
        // assuming that numeric components alone establish graph ownership.
        use crate::hybrid::ids::{GraphScopedId, IdComponent};

        let graph_a = HybridGraphId::new(
            IdComponent::new(1).expect("non-zero component"),
        );
        let graph_b = HybridGraphId::new(
            IdComponent::new(2).expect("non-zero component"),
        );

        let node_id = HybridNodeId::new(GraphScopedId::new(
            graph_a,
            IdComponent::new(3).expect("non-zero component"),
        ));
        let domain_id = DomainInstanceId::new(GraphScopedId::new(
            graph_b,
            IdComponent::new(4).expect("non-zero component"),
        ));

        let operation = CanonicalIrRef::new("classical.ir", "operation")
            .expect("valid canonical IR reference");

        assert!(PlanNode::new(node_id, domain_id, operation).is_err());
    }

    #[test]
    fn plan_dependency_preserves_feedback_as_a_typed_relationship() {
        use crate::hybrid::graph::edges::EdgeKind;
        use crate::hybrid::ids::{GraphScopedId, IdComponent};

        let graph_id = HybridGraphId::new(
            IdComponent::new(1).expect("non-zero component"),
        );

        let source = HybridNodeId::new(GraphScopedId::new(
            graph_id,
            IdComponent::new(2).expect("non-zero component"),
        ));

        let target = HybridNodeId::new(GraphScopedId::new(
            graph_id,
            IdComponent::new(3).expect("non-zero component"),
        ));

        let dependency = PlanDependency {
            edge_id: HybridEdgeId::new(GraphScopedId::new(
                graph_id,
                IdComponent::new(4).expect("non-zero component"),
            )),
            dependency_id: HybridDependencyId::new(GraphScopedId::new(
                graph_id,
                IdComponent::new(5).expect("non-zero component"),
            )),
            source,
            target,
            kind: EdgeKind::Feedback,
        };

        assert_eq!(dependency.source(), source);
        assert_eq!(dependency.target(), target);
        assert_eq!(dependency.kind(), &EdgeKind::Feedback);
        assert!(dependency.validate(graph_id).is_ok());
    }

    #[test]
    fn empty_plan_is_a_valid_intermediate_representation() {
        use crate::hybrid::ids::IdComponent;

        let graph_id = HybridGraphId::new(
            IdComponent::new(1).expect("non-zero component"),
        );

        let plan = HybridPlan::new(graph_id, Vec::new(), Vec::new())
            .expect("empty plans are permitted as intermediate representations");

        assert!(plan.is_empty());
        assert_eq!(plan.node_count(), 0);
        assert_eq!(plan.dependency_count(), 0);
        assert!(plan.validate_local().is_ok());
    }
}
