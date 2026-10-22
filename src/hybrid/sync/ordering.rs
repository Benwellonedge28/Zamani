#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Ordering Semantics
//!
//! Path: `src/hybrid/sync/ordering.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provide a target-independent interpretation of ordering-bearing hybrid
//! dependency edges and answer ordering queries without creating a second
//! authoritative dependency graph.
//!
//! The module distinguishes a known required ordering path from the absence
//! of a known path and from an inconclusive result caused by unresolved
//! optional extension semantics.
//!
//! ## Canonical ownership
//!
//! This module owns:
//! - The built-in ordering-guarantee classification used by hybrid analysis.
//! - A read-only, derived view of ordering-bearing graph edges.
//! - Strict reachability queries over known ordering-bearing relationships.
//! - Validation that synchronization declarations refer to nodes in the
//!   analyzed graph.
//! - Explicitly inconclusive results where unknown extension semantics
//!   prevent a sound negative conclusion.
//!
//! This module does not own:
//! - The canonical hybrid dependency graph or its edges.
//! - General language memory-model or atomic-ordering semantics.
//! - Source syntax, ASTs, types, effects, or semantic analysis.
//! - Domain-specific operation semantics or commutativity proofs.
//! - Scheduling, placement, hardware barriers, locks, or runtime execution.
//! - Classical IR, quantum IR, ZUIR, ZCIR, HDLIR, or backend behavior.
//!
//! ## Canonical integration
//!
//! - `super::model` owns synchronization declarations and their vocabulary.
//! - `crate::hybrid::graph::edges` owns dependency edges and their kinds.
//! - `crate::hybrid::graph::analysis` owns graph validation, adjacency
//!   indexes, and the canonical graph-analysis view.
//! - `crate::hybrid::config` owns explicit operational budgets.
//! - `crate::hybrid::errors` owns structured hybrid errors.
//! - `crate::compiler::hybrid_pipeline` retains its existing compatibility
//!   API and adapts to canonical hybrid semantics at the integration boundary.
//! - `crate::runtime::hybrid_kernel` realizes the ordering obligations
//!   supplied by the validated compilation/runtime contracts.
//! - The language-wide memory model remains authoritative for memory ordering.
//!
//! ## Semantics
//!
//! The built-in graph relationships have these ordering interpretations:
//!
//! - Data: the producer must make the required value available to its consumer.
//! - Control: the controlling dependency must be respected.
//! - Completion: the declared completion prerequisite must be respected.
//! - Ordering: the explicit source-to-target ordering must be preserved.
//! - Feedback: the represented causal dependency must be preserved.
//! - DomainBoundary: crossing a domain boundary alone does not establish
//!   an ordering relationship.
//!
//! Unknown extension relationships are never silently assigned built-in
//! ordering semantics.
//!
//! A missing ordering path does not prove that two operations may overlap.
//! Proving overlap or legal reordering requires the authoritative semantic
//! model, effects, memory model, and relevant domain-specific guarantees.
//!
//! ## Synchronization integration
//!
//! A synchronization event's participant list is not an execution schedule.
//! Its order must not be inferred from vector position. This module validates
//! that event participants belong to the analyzed graph, but does not infer
//! a directed ordering between participants when the event does not encode
//! one.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on graph nodes, edges, participants, or
//! ordering queries. Traversal uses growable collections and the existing
//! graph-analysis indexes.
//!
//! Callers may supply explicit work budgets through `HybridConfig`. Budget
//! exhaustion is an operational failure, not proof that the represented
//! program is semantically invalid.
//!
//! Finite memory, finite identifier representations, allocator failures, and
//! available execution time remain real resource constraints.
//!
//! ## Determinism
//!
//! Ordering constraints are derived from the canonical edge set. Graph
//! adjacency iteration uses the existing deterministic graph-analysis view.
//! This module does not sort operations into a new execution schedule.
//!
//! ## Completion criteria
//!
//! - No second dependency graph is stored.
//! - Ordering-bearing edge kinds have explicit, documented interpretations.
//! - Unknown optional extension semantics cannot produce a false negative.
//! - No path result is misrepresented as proof that operations may overlap.
//! - Synchronization participants are validated against the canonical graph.
//! - Operational work budgets are honored.
//! - No unsafe Rust or universal hardware-size ceiling is introduced.

use std::collections::{BTreeSet, VecDeque};

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::graph::analysis::HybridGraphAnalysis;
use crate::hybrid::graph::edges::{EdgeKind, HybridEdge};
use crate::hybrid::graph::model::HybridGraph;
use crate::hybrid::graph::edges::HybridEdgeSet;
use crate::hybrid::ids::{HybridEdgeId, HybridNodeId};

use super::model::SynchronizationModel;

/// The built-in ordering guarantee represented by a canonical graph edge.
///
/// These classifications describe semantic prerequisites, not physical
/// scheduling decisions. Domain-specific and extension-defined semantics
/// remain owned by their authoritative implementations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum OrderingGuarantee {
    /// The source must make a required value available before the target
    /// consumes that value.
    DataAvailability,

    /// The source's control decision must be respected by the target.
    ControlDependency,

    /// The source must satisfy its declared completion prerequisite before
    /// the target proceeds past the dependency.
    CompletionBeforeProceeding,

    /// The explicit source-to-target ordering must be preserved.
    ExplicitOrdering,

    /// The source-to-target causal feedback relationship must be preserved.
    CausalFeedback,
}

impl OrderingGuarantee {
    /// Classify a canonical edge kind when its built-in meaning establishes
    /// a directed ordering prerequisite.
    ///
    /// `None` means that this module cannot infer a built-in ordering
    /// prerequisite from the edge kind alone. It does not mean that the
    /// edge is semantically irrelevant.
    #[must_use]
    pub fn from_edge_kind(kind: &EdgeKind) -> Option<Self> {
        match kind {
            EdgeKind::Data { .. } => Some(Self::DataAvailability),
            EdgeKind::Control => Some(Self::ControlDependency),
            EdgeKind::Completion { .. } => {
                Some(Self::CompletionBeforeProceeding)
            }
            EdgeKind::Ordering => Some(Self::ExplicitOrdering),
            EdgeKind::Feedback => Some(Self::CausalFeedback),

            // A domain boundary identifies a cross-domain relationship;
            // it does not independently establish a directed prerequisite.
            EdgeKind::DomainBoundary => None,

            // Extension semantics must be interpreted by their registered
            // authority. Never guess that an extension is ordering-bearing.
            EdgeKind::Extension(_) => None,

            // Keep this wildcard so adding future EdgeKind variants cannot
            // silently acquire ordering semantics in this module.
            _ => None,
        }
    }

    /// Return a stable identifier for diagnostics and integrations.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DataAvailability => "data_availability",
            Self::ControlDependency => "control_dependency",
            Self::CompletionBeforeProceeding => {
                "completion_before_proceeding"
            }
            Self::ExplicitOrdering => "explicit_ordering",
            Self::CausalFeedback => "causal_feedback",
        }
    }
}

/// A read-only derived view of one ordering-bearing canonical graph edge.
///
/// This value is not a new dependency declaration. The `edge_id` refers back
/// to the canonical edge owned by `graph::edges`. If the graph changes, this
/// derived value must not be treated as an independently authoritative fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderingConstraint {
    /// Identity of the canonical edge that establishes this prerequisite.
    pub edge_id: HybridEdgeId,

    /// The prerequisite node.
    pub source: HybridNodeId,

    /// The dependent node.
    pub target: HybridNodeId,

    /// The built-in guarantee associated with this edge.
    pub guarantee: OrderingGuarantee,
}

impl OrderingConstraint {
    /// Derive an ordering constraint from a canonical edge.
    ///
    /// Edges without a built-in directed ordering interpretation return
    /// `None`. This function does not resolve extension semantics.
    #[must_use]
    pub fn from_edge(edge: &HybridEdge) -> Option<Self> {
        Some(Self {
            edge_id: edge.id(),
            source: edge.source(),
            target: edge.target(),
            guarantee: OrderingGuarantee::from_edge_kind(edge.kind())?,
        })
    }
}

/// Result of asking whether a directed ordering prerequisite is established.
///
/// `NoKnownOrderingPath` is deliberately weaker than "may overlap": an
/// optimizer must not use it as proof that reordering is legal.
///
/// `Indeterminate` means that unresolved optional extension relationships
/// prevent this analysis from safely concluding that no ordering path exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OrderingAssessment {
    /// A path consisting of recognized ordering-bearing edges exists.
    MustPrecede,

    /// No path was found among the recognized built-in ordering relationships.
    ///
    /// This is not proof of independence, commutativity, or legal overlap.
    NoKnownOrderingPath,

    /// No recognized path was found, but unresolved optional extension
    /// semantics mean that a sound negative conclusion cannot be made.
    Indeterminate,
}

/// Read-only ordering analysis over the canonical hybrid graph and edge set.
///
/// The analysis retains the repository's existing indexed graph view instead
/// of storing a second dependency graph. Ordering queries consider only edge
/// kinds whose built-in semantics establish directed prerequisites.
pub struct OrderingAnalysis<'a> {
    graph: HybridGraphAnalysis<'a>,
    config: HybridConfig,
}

impl<'a> OrderingAnalysis<'a> {
    /// Validate and index the graph and its edge set for ordering analysis.
    ///
    /// This reuses `HybridGraphAnalysis::new`, which validates the graph,
    /// validates edge references, constructs deterministic adjacency indexes,
    /// and applies the supplied configuration's graph/work budgets.
    pub fn new(
        graph: &'a HybridGraph,
        edges: &'a HybridEdgeSet,
        config: &HybridConfig,
    ) -> Result<Self, HybridError> {
        let graph = HybridGraphAnalysis::new(graph, edges, config)
            .map_err(|error| error.in_component("hybrid ordering analysis"))?;

        Ok(Self {
            graph,
            config: config.clone(),
        })
    }

    /// Return the identity of the graph being analyzed.
    #[must_use]
    pub const fn graph_id(&self) -> crate::hybrid::ids::HybridGraphId {
        self.graph.graph().id()
    }

    /// Return whether all edge semantics are understood by the built-in
    /// edge validator.
    ///
    /// A `false` result means that at least one optional extension remains
    /// unresolved. Required unsupported extensions are rejected during
    /// graph-analysis construction.
    #[must_use]
    pub fn has_complete_builtin_edge_semantics(&self) -> bool {
        self.graph
            .edge_validation_report()
            .is_fully_semantically_understood()
    }

    /// Iterate over recognized ordering-bearing edges in canonical
    /// edge-set declaration order.
    ///
    /// This iterator is a derived view; the edge set remains authoritative.
    #[must_use]
    pub fn constraints(
        &self,
    ) -> impl Iterator<Item = OrderingConstraint> + '_ {
        self.graph
            .edges()
            .edges()
            .iter()
            .filter_map(OrderingConstraint::from_edge)
    }

    /// Iterate over recognized direct ordering constraints between two nodes.
    ///
    /// Unknown nodes simply yield no matching constraints. Use
    /// `must_precede` when node existence must be validated.
    #[must_use]
    pub fn direct_constraints(
        &self,
        source: HybridNodeId,
        target: HybridNodeId,
    ) -> impl Iterator<Item = OrderingConstraint> + '_ {
        self.constraints().filter(move |constraint| {
            constraint.source == source && constraint.target == target
        })
    }

    /// Determine whether a recognized ordering path exists from `before`
    /// to `after`.
    ///
    /// The query follows only built-in ordering-bearing edges; it does not
    /// follow every graph edge indiscriminately.
    ///
    /// The configured `ValidationSteps` budget is checked as the traversal
    /// proceeds. If no known path exists but optional extension semantics
    /// remain unresolved, the result is `Indeterminate`, not a false claim
    /// that the nodes are unordered.
    ///
    /// This method does not prove that the path is executable, that the
    /// program is deadlock-free, or that a proposed target plan preserves
    /// every domain-specific semantic guarantee.
    pub fn must_precede(
        &self,
        before: HybridNodeId,
        after: HybridNodeId,
    ) -> Result<OrderingAssessment, HybridError> {
        self.require_node(before)?;
        self.require_node(after)?;

        let mut visited = BTreeSet::new();
        let mut pending = VecDeque::new();
        let mut work = 0_u128;

        visited.insert(before);
        pending.push_back(before);

        while let Some(current) = pending.pop_front() {
            for edge in self.graph.outgoing_edges(current) {
                if OrderingGuarantee::from_edge_kind(edge.kind()).is_none() {
                    continue;
                }

                work = work.checked_add(1).ok_or_else(|| {
                    HybridError::new(
                        HybridErrorCode::BudgetExhausted,
                        "ordering-query work counter overflowed",
                    )
                    .in_component("hybrid ordering analysis")
                })?;

                self.config
                    .check_budget(BudgetKind::ValidationSteps, work)
                    .map_err(|error| {
                        error.in_component("hybrid ordering query")
                    })?;

                let next = edge.target();

                if next == after {
                    return Ok(OrderingAssessment::MustPrecede);
                }

                if visited.insert(next) {
                    pending.push_back(next);
                }
            }
        }

        if self.has_complete_builtin_edge_semantics() {
            Ok(OrderingAssessment::NoKnownOrderingPath)
        } else {
            Ok(OrderingAssessment::Indeterminate)
        }
    }

    /// Validate synchronization declarations against the analyzed graph.
    ///
    /// `SynchronizationModel::validate` checks event-local structure and
    /// graph ownership. This method additionally verifies that every declared
    /// participant exists in the actual graph.
    ///
    /// Synchronization participant order is not interpreted as execution
    /// order. Directed ordering must be represented by canonical dependencies
    /// or by a richer authoritative synchronization contract.
    pub fn validate_synchronization(
        &self,
        synchronization: &SynchronizationModel,
    ) -> Result<(), HybridError> {
        if synchronization.graph_id != self.graph_id() {
            return Err(HybridError::invalid_reference(format!(
                "synchronization model belongs to graph {}, but ordering analysis uses graph {}",
                synchronization.graph_id,
                self.graph_id()
            ))
            .in_component("hybrid ordering analysis"));
        }

        synchronization
            .validate()
            .map_err(|error| error.in_component("hybrid ordering analysis"))?;

        for event in synchronization.events() {
            for participant in &event.participants {
                if !self.graph.contains_node(*participant) {
                    return Err(HybridError::invalid_reference(format!(
                        "synchronization event {} references missing graph node {}",
                        event.id, participant
                    ))
                    .in_component("hybrid ordering analysis"));
                }
            }
        }

        Ok(())
    }

    fn require_node(&self, node: HybridNodeId) -> Result<(), HybridError> {
        if self.graph.contains_node(node) {
            return Ok(());
        }

        Err(HybridError::invalid_reference(format!(
            "ordering query references node {} that is absent from graph {}",
            node,
            self.graph_id()
        ))
        .in_component("hybrid ordering analysis"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_edges_establish_data_availability_ordering() {
        assert_eq!(
            OrderingGuarantee::from_edge_kind(&EdgeKind::Data { value: None }),
            Some(OrderingGuarantee::DataAvailability)
        );
    }

    #[test]
    fn control_edges_establish_control_dependency_ordering() {
        assert_eq!(
            OrderingGuarantee::from_edge_kind(&EdgeKind::Control),
            Some(OrderingGuarantee::ControlDependency)
        );
    }

    #[test]
    fn completion_edges_establish_completion_ordering() {
        assert_eq!(
            OrderingGuarantee::from_edge_kind(
                &EdgeKind::Completion { event: None }
            ),
            Some(OrderingGuarantee::CompletionBeforeProceeding)
        );
    }

    #[test]
    fn explicit_ordering_edges_are_recognized() {
        assert_eq!(
            OrderingGuarantee::from_edge_kind(&EdgeKind::Ordering),
            Some(OrderingGuarantee::ExplicitOrdering)
        );
    }

    #[test]
    fn feedback_edges_preserve_causal_ordering() {
        assert_eq!(
            OrderingGuarantee::from_edge_kind(&EdgeKind::Feedback),
            Some(OrderingGuarantee::CausalFeedback)
        );
    }

    #[test]
    fn domain_boundary_alone_does_not_establish_ordering() {
        assert_eq!(
            OrderingGuarantee::from_edge_kind(&EdgeKind::DomainBoundary),
            None
        );
    }

    #[test]
    fn guarantee_identifiers_are_stable_and_distinct() {
        let guarantees = [
            OrderingGuarantee::DataAvailability,
            OrderingGuarantee::ControlDependency,
            OrderingGuarantee::CompletionBeforeProceeding,
            OrderingGuarantee::ExplicitOrdering,
            OrderingGuarantee::CausalFeedback,
        ];

        let mut identifiers = BTreeSet::new();

        for guarantee in guarantees {
            assert!(identifiers.insert(guarantee.as_str()));
        }
    }

    #[test]
    fn no_known_path_does_not_mean_may_overlap() {
        assert_ne!(
            OrderingAssessment::NoKnownOrderingPath,
            OrderingAssessment::MustPrecede
        );

        assert_ne!(
            OrderingAssessment::NoKnownOrderingPath,
            OrderingAssessment::Indeterminate
        );
    }
}