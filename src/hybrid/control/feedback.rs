#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Feedback Contracts
//!
//! Path: `src/hybrid/control/feedback.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Represent and validate causal feedback relationships in a hybrid
//! computation.
//!
//! A common feedback path is:
//!
//!     measurement/result producer
//!             |
//!             | data dependency
//!             v
//!     classical decision
//!             |
//!             | control dependency
//!             v
//!     subsequent operation
//!
//! The participating domains are not restricted to quantum and classical
//! computation. Other domains may participate when their canonical
//! representations and semantic adapters establish the relevant meaning.
//!
//! This module describes relationships between canonical graph entities.
//! It does not execute operations, evaluate predicates, perform measurement,
//! transfer payloads, schedule work, or select physical resources.
//!
//! ## Canonical ownership
//!
//! - `graph::model::HybridGraph` owns the hybrid graph and its node references.
//! - `graph::edges::HybridEdgeSet` owns the canonical dependency edges.
//! - `ids` owns graph-scoped identities.
//! - `errors` owns structured hybrid errors.
//! - The canonical semantic layer owns types, effects, predicate meaning,
//!   value provenance, and operation semantics.
//! - `quantum::ir` remains authoritative for quantum semantics.
//! - `control::flow` owns structural control-region descriptions.
//! - Synchronization and runtime modules own their respective contracts.
//!
//! This module does not create a competing edge model, value model, quantum
//! IR, classical IR, expression evaluator, or runtime state machine.
//!
//! ## Validation boundary
//!
//! Structural validation proves that:
//!
//! - all referenced identities belong to the declared graph;
//! - each referenced dependency exists;
//! - the data dependency connects the result producer to the decision;
//! - the data dependency carries the declared cross-domain value identity;
//! - the control dependency connects the decision to the controlled operation;
//! - an optional loop-carried feedback dependency has the declared direction
//!   and canonical `Feedback` edge kind.
//!
//! Structural validation does not prove that a node really performs a
//! measurement, that a decision is Boolean-compatible, or that a target
//! supports dynamic feedback. Those require canonical semantic resolution
//! and downstream feasibility analysis.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on feedback flows, nodes, edges, domains,
//! or loop iterations. Collections grow according to available resources.
//! Optional operational budgets can be checked through `HybridConfig`.
//!
//! A budget is an implementation policy, not a restriction on the meaning
//! of a valid Zamani program. Exhaustion is reported explicitly.
//!
//! ## Cycles
//!
//! Feedback and loop-carried dependencies may form valid cycles. This module
//! does not impose a universal directed-acyclic-graph rule. Cycle legality
//! is determined by the canonical control-flow and semantic verification
//! layers.
//!
//! ## Serialization
//!
//! Deserialization validates local identity invariants. It does not establish
//! graph membership or dependency existence. Call `validate_against` before
//! using deserialized feedback descriptions as structurally validated data.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::graph::edges::{EdgeKind, HybridEdge, HybridEdgeSet};
use crate::hybrid::graph::model::HybridGraph;
use crate::hybrid::ids::{
    CrossDomainValueId,
    HybridEdgeId,
    HybridGraphId,
    HybridNodeId,
};

/// A single, explicitly described feedback relationship.
///
/// The fields refer to existing canonical graph nodes and dependency edges.
/// They do not duplicate those nodes or edges.
///
/// `result_producer` is the operation that produces the value consumed by
/// the decision. For quantum feedback, this is commonly a measurement
/// operation, but the type deliberately does not hard-code a domain.
///
/// `decision` identifies the canonical operation that makes or represents
/// the decision. Its predicate semantics and effects must be established by
/// the canonical semantic layer.
///
/// `controlled_operation` identifies the operation whose behavior depends
/// on the decision.
///
/// `result_dependency` must be a `Data` edge carrying `result_value` from
/// `result_producer` to `decision`.
///
/// `control_dependency` must be a `Control` or `Feedback` edge from
/// `decision` to `controlled_operation`.
///
/// `loop_back_dependency`, when present, must be a `Feedback` edge from
/// `controlled_operation` to `result_producer`. It expresses a graph-level
/// loop-carried relationship; it does not prescribe a physical schedule or
/// a number of iterations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "FeedbackFlowWire")]
pub struct HybridFeedbackFlow {
    graph_id: HybridGraphId,
    result_producer: HybridNodeId,
    result_value: CrossDomainValueId,
    decision: HybridNodeId,
    controlled_operation: HybridNodeId,
    result_dependency: HybridEdgeId,
    control_dependency: HybridEdgeId,
    loop_back_dependency: Option<HybridEdgeId>,
}

/// Wire representation used to validate local invariants during
/// deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FeedbackFlowWire {
    graph_id: HybridGraphId,
    result_producer: HybridNodeId,
    result_value: CrossDomainValueId,
    decision: HybridNodeId,
    controlled_operation: HybridNodeId,
    result_dependency: HybridEdgeId,
    control_dependency: HybridEdgeId,
    loop_back_dependency: Option<HybridEdgeId>,
}

impl TryFrom<FeedbackFlowWire> for HybridFeedbackFlow {
    type Error = HybridError;

    fn try_from(value: FeedbackFlowWire) -> Result<Self, Self::Error> {
        Self::new(
            value.graph_id,
            value.result_producer,
            value.result_value,
            value.decision,
            value.controlled_operation,
            value.result_dependency,
            value.control_dependency,
            value.loop_back_dependency,
        )
    }
}

impl HybridFeedbackFlow {
    /// Construct a feedback description and validate its local invariants.
    ///
    /// This constructor does not establish that the referenced graph nodes
    /// or dependency edges exist. Use `validate_against` for that purpose.
    pub fn new(
        graph_id: HybridGraphId,
        result_producer: HybridNodeId,
        result_value: CrossDomainValueId,
        decision: HybridNodeId,
        controlled_operation: HybridNodeId,
        result_dependency: HybridEdgeId,
        control_dependency: HybridEdgeId,
        loop_back_dependency: Option<HybridEdgeId>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            graph_id,
            result_producer,
            result_value,
            decision,
            controlled_operation,
            result_dependency,
            control_dependency,
            loop_back_dependency,
        };

        value.validate_local()?;
        Ok(value)
    }

    /// Return the graph identity.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Return the node that produces the feedback result.
    #[must_use]
    pub const fn result_producer(&self) -> HybridNodeId {
        self.result_producer
    }

    /// Return the identity of the result consumed by the decision.
    #[must_use]
    pub const fn result_value(&self) -> CrossDomainValueId {
        self.result_value
    }

    /// Return the node representing the decision.
    #[must_use]
    pub const fn decision(&self) -> HybridNodeId {
        self.decision
    }

    /// Return the operation controlled by the decision.
    #[must_use]
    pub const fn controlled_operation(&self) -> HybridNodeId {
        self.controlled_operation
    }

    /// Return the canonical data-dependency edge.
    #[must_use]
    pub const fn result_dependency(&self) -> HybridEdgeId {
        self.result_dependency
    }

    /// Return the canonical control-dependency edge.
    #[must_use]
    pub const fn control_dependency(&self) -> HybridEdgeId {
        self.control_dependency
    }

    /// Return the optional loop-carried feedback edge.
    #[must_use]
    pub const fn loop_back_dependency(&self) -> Option<HybridEdgeId> {
        self.loop_back_dependency
    }

    /// Validate identity relationships that do not require a complete graph.
    ///
    /// This method is also used to validate deserialized descriptions.
    pub fn validate_local(&self) -> Result<(), HybridError> {
        let node_ids = [
            self.result_producer,
            self.decision,
            self.controlled_operation,
        ];

        for node in node_ids {
            if node.graph() != self.graph_id {
                return Err(HybridError::invalid_reference(format!(
                    "feedback node {node} does not belong to graph {}",
                    self.graph_id
                )));
            }
        }

        if self.result_value.graph() != self.graph_id {
            return Err(HybridError::invalid_reference(format!(
                "feedback result {} does not belong to graph {}",
                self.result_value, self.graph_id
            )));
        }

        let mut dependency_ids = BTreeSet::new();
        dependency_ids.insert(self.result_dependency);

        if !dependency_ids.insert(self.control_dependency) {
            return Err(HybridError::invalid_dependency(
                "the result and control dependencies must be distinct edges",
            ));
        }

        if let Some(loop_back) = self.loop_back_dependency {
            if !dependency_ids.insert(loop_back) {
                return Err(HybridError::invalid_dependency(
                    "a feedback flow cannot reuse one edge for multiple dependency roles",
                ));
            }
        }

        for dependency in dependency_ids {
            if dependency.graph() != self.graph_id {
                return Err(HybridError::invalid_reference(format!(
                    "feedback dependency {dependency} does not belong to graph {}",
                    self.graph_id
                )));
            }
        }

        Ok(())
    }

    /// Validate this flow against the canonical graph and its canonical edges.
    ///
    /// This validates graph structure and the declared edge relationships.
    /// It does not replace canonical semantic verification.
    pub fn validate_against(
        &self,
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
    ) -> Result<(), HybridError> {
        edges.validate_against(graph)?;

        let edge_index: BTreeMap<HybridEdgeId, &HybridEdge> =
            edges.edges().iter().map(|edge| (edge.id(), edge)).collect();

        self.validate_with_index(graph.id(), &edge_index)
    }

    fn validate_with_index(
        &self,
        graph_id: HybridGraphId,
        edge_index: &BTreeMap<HybridEdgeId, &HybridEdge>,
    ) -> Result<(), HybridError> {
        self.validate_local()?;

        if self.graph_id != graph_id {
            return Err(HybridError::invalid_reference(format!(
                "feedback flow belongs to graph {}, not graph {}",
                self.graph_id, graph_id
            )));
        }

        let result_edge = edge_index
            .get(&self.result_dependency)
            .copied()
            .ok_or_else(|| {
                HybridError::invalid_reference(format!(
                    "feedback result dependency {} does not exist",
                    self.result_dependency
                ))
            })?;

        if result_edge.source() != self.result_producer
            || result_edge.target() != self.decision
        {
            return Err(HybridError::invalid_dependency(format!(
                "result dependency {} must connect producer {} to decision {}",
                self.result_dependency, self.result_producer, self.decision
            )));
        }

        match result_edge.kind() {
            EdgeKind::Data {
                value: Some(value),
            } if *value == self.result_value => {}
            EdgeKind::Data { value: Some(value) } => {
                return Err(HybridError::invalid_dependency(format!(
                    "result dependency {} carries value {}, not declared value {}",
                    self.result_dependency, value, self.result_value
                )));
            }
            _ => {
                return Err(HybridError::invalid_dependency(format!(
                    "result dependency {} must be a data edge carrying an explicit value identity",
                    self.result_dependency
                )));
            }
        }

        let control_edge = edge_index
            .get(&self.control_dependency)
            .copied()
            .ok_or_else(|| {
                HybridError::invalid_reference(format!(
                    "feedback control dependency {} does not exist",
                    self.control_dependency
                ))
            })?;

        if control_edge.source() != self.decision
            || control_edge.target() != self.controlled_operation
        {
            return Err(HybridError::invalid_dependency(format!(
                "control dependency {} must connect decision {} to controlled operation {}",
                self.control_dependency, self.decision, self.controlled_operation
            )));
        }

        if !matches!(
            control_edge.kind(),
            EdgeKind::Control | EdgeKind::Feedback
        ) {
            return Err(HybridError::invalid_dependency(format!(
                "control dependency {} must have Control or Feedback semantics",
                self.control_dependency
            )));
        }

        if let Some(loop_back_id) = self.loop_back_dependency {
            let loop_back_edge = edge_index.get(&loop_back_id).copied().ok_or_else(|| {
                HybridError::invalid_reference(format!(
                    "loop-carried feedback dependency {loop_back_id} does not exist"
                ))
            })?;

            if loop_back_edge.source() != self.controlled_operation
                || loop_back_edge.target() != self.result_producer
            {
                return Err(HybridError::invalid_dependency(format!(
                    "loop-carried feedback dependency {loop_back_id} must connect controlled operation {} back to result producer {}",
                    self.controlled_operation, self.result_producer
                )));
            }

            if !matches!(loop_back_edge.kind(), EdgeKind::Feedback) {
                return Err(HybridError::invalid_dependency(format!(
                    "loop-carried dependency {loop_back_id} must have Feedback semantics"
                )));
            }
        }

        Ok(())
    }
}

/// A graph-scoped collection of feedback-flow descriptions.
///
/// The collection is a derived semantic view over canonical graph nodes and
/// edges. It does not own or replace those graph objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "FeedbackFlowSetWire")]
pub struct HybridFeedbackFlowSet {
    graph_id: HybridGraphId,
    flows: Vec<HybridFeedbackFlow>,
}

/// Wire representation used to validate feedback-flow collections on
/// deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct FeedbackFlowSetWire {
    graph_id: HybridGraphId,
    #[serde(default)]
    flows: Vec<HybridFeedbackFlow>,
}

impl TryFrom<FeedbackFlowSetWire> for HybridFeedbackFlowSet {
    type Error = HybridError;

    fn try_from(value: FeedbackFlowSetWire) -> Result<Self, Self::Error> {
        Self::new(value.graph_id, value.flows)
    }
}

impl HybridFeedbackFlowSet {
    /// Construct a feedback-flow collection.
    ///
    /// The flows remain in declaration order. No semantic ordering is
    /// inferred from their position in this collection.
    pub fn new(
        graph_id: HybridGraphId,
        flows: Vec<HybridFeedbackFlow>,
    ) -> Result<Self, HybridError> {
        let value = Self { graph_id, flows };
        value.validate_local()?;
        Ok(value)
    }

    /// Return the graph identity.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Return the feedback flows in declaration order.
    #[must_use]
    pub fn flows(&self) -> &[HybridFeedbackFlow] {
        &self.flows
    }

    /// Return the number of described feedback flows.
    #[must_use]
    pub fn len(&self) -> usize {
        self.flows.len()
    }

    /// Return whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.flows.is_empty()
    }

    /// Iterate over flows whose result producer matches `node`.
    pub fn produced_by(
        &self,
        node: HybridNodeId,
    ) -> impl Iterator<Item = &HybridFeedbackFlow> {
        self.flows
            .iter()
            .filter(move |flow| flow.result_producer() == node)
    }

    /// Iterate over flows whose decision node matches `node`.
    pub fn decided_by(
        &self,
        node: HybridNodeId,
    ) -> impl Iterator<Item = &HybridFeedbackFlow> {
        self.flows
            .iter()
            .filter(move |flow| flow.decision() == node)
    }

    /// Iterate over flows that control `node`.
    pub fn controlling(
        &self,
        node: HybridNodeId,
    ) -> impl Iterator<Item = &HybridFeedbackFlow> {
        self.flows
            .iter()
            .filter(move |flow| flow.controlled_operation() == node)
    }

    /// Validate collection-local invariants.
    pub fn validate_local(&self) -> Result<(), HybridError> {
        let mut seen = BTreeSet::new();

        for flow in &self.flows {
            flow.validate_local()?;

            if flow.graph_id() != self.graph_id {
                return Err(HybridError::invalid_reference(format!(
                    "feedback flow belongs to graph {}, not collection graph {}",
                    flow.graph_id(),
                    self.graph_id
                )));
            }

            if !seen.insert(flow) {
                return Err(HybridError::invalid_dependency(
                    "the feedback-flow collection contains a duplicate flow description",
                ));
            }
        }

        Ok(())
    }

    /// Validate all feedback descriptions against the canonical graph and
    /// edge set.
    ///
    /// The edge set is validated once, and a borrowed index is built for
    /// repeated dependency lookups. This avoids repeatedly scanning all
    /// edges for each feedback flow.
    pub fn validate_against(
        &self,
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
    ) -> Result<(), HybridError> {
        self.validate_local()?;
        edges.validate_against(graph)?;

        if self.graph_id != graph.id() || self.graph_id != edges.graph_id() {
            return Err(HybridError::invalid_reference(format!(
                "feedback collection graph {}, graph model {}, and edge set {} must agree",
                self.graph_id,
                graph.id(),
                edges.graph_id()
            )));
        }

        let edge_index: BTreeMap<HybridEdgeId, &HybridEdge> =
            edges.edges().iter().map(|edge| (edge.id(), edge)).collect();

        for flow in &self.flows {
            flow.validate_with_index(graph.id(), &edge_index)?;
        }

        Ok(())
    }

    /// Validate the collection after applying explicitly configured
    /// feedback-validation budgets.
    ///
    /// `GraphEdges` checks the supplied edge count. `ValidationSteps` uses
    /// one accounting unit per edge and one per feedback-flow description
    /// for this module's preflight policy. Whole-graph verification remains
    /// responsible for its own complete work accounting.
    pub fn validate_against_with_config(
        &self,
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
        config: &HybridConfig,
    ) -> Result<(), HybridError> {
        config.check_budget(BudgetKind::GraphEdges, edges.len() as u128)?;

        let work_units = (edges.len() as u128)
            .checked_add(self.flows.len() as u128)
            .ok_or_else(|| {
                HybridError::new(
                    HybridErrorCode::BudgetExhausted,
                    "feedback validation work accounting overflowed",
                )
            })?;

        config.check_budget(BudgetKind::ValidationSteps, work_units)?;

        self.validate_against(graph, edges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::ids::IdAllocator;

    #[test]
    fn empty_feedback_set_is_valid_for_its_graph() {
        let allocator = IdAllocator::new();
        let graph_id = allocator
            .allocate_graph_id()
            .expect("a fresh allocator can allocate a graph ID");

        let flows = HybridFeedbackFlowSet::new(graph_id, Vec::new())
            .expect("an empty feedback set is valid");

        assert_eq!(flows.graph_id(), graph_id);
        assert!(flows.is_empty());
        assert_eq!(flows.len(), 0);
    }

    #[test]
    fn feedback_flow_rejects_an_identity_from_another_graph() {
        let allocator = IdAllocator::new();
        let first_graph = allocator
            .allocate_graph_id()
            .expect("first graph ID");
        let second_graph = allocator
            .allocate_graph_id()
            .expect("second graph ID");

        let producer = allocator
            .allocate_node_id(first_graph)
            .expect("producer node ID");
        let decision = allocator
            .allocate_node_id(first_graph)
            .expect("decision node ID");
        let controlled = allocator
            .allocate_node_id(first_graph)
            .expect("controlled node ID");
        let value = allocator
            .allocate_cross_domain_value_id(second_graph)
            .expect("cross-graph value ID");
        let result_edge = allocator
            .allocate_edge_id(first_graph)
            .expect("result edge ID");
        let control_edge = allocator
            .allocate_edge_id(first_graph)
            .expect("control edge ID");

        let result = HybridFeedbackFlow::new(
            first_graph,
            producer,
            value,
            decision,
            controlled,
            result_edge,
            control_edge,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn feedback_flow_rejects_reusing_one_edge_for_two_roles() {
        let allocator = IdAllocator::new();
        let graph_id = allocator
            .allocate_graph_id()
            .expect("graph ID");
        let producer = allocator
            .allocate_node_id(graph_id)
            .expect("producer node ID");
        let decision = allocator
            .allocate_node_id(graph_id)
            .expect("decision node ID");
        let controlled = allocator
            .allocate_node_id(graph_id)
            .expect("controlled node ID");
        let value = allocator
            .allocate_cross_domain_value_id(graph_id)
            .expect("value ID");
        let edge = allocator
            .allocate_edge_id(graph_id)
            .expect("edge ID");

        let result = HybridFeedbackFlow::new(
            graph_id,
            producer,
            value,
            decision,
            controlled,
            edge,
            edge,
            None,
        );

        assert!(result.is_err());
    }
}