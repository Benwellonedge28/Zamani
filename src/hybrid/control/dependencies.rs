
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Dependency Analysis
//!
//! Path: `src/hybrid/control/dependencies.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//!
//! ## Purpose
//!
//! Provide a validated, read-only index over the dependency relationships
//! owned by `crate::hybrid::graph::edges`.
//!
//! This module distinguishes data, control, completion, ordering, feedback,
//! domain-boundary, and explicitly namespaced extension relationships.
//!
//! It does not define a second dependency graph. `HybridEdgeSet` remains the
//! authoritative representation; this module stores borrowed references to
//! its edges in indexes that are valid only while that edge set is borrowed.
//!
//! ## Ownership
//!
//! This module owns:
//! - Dependency-semantic classification for consumers of the hybrid layer.
//! - A read-only, graph-validated dependency index.
//! - Queries for incoming and outgoing dependencies.
//! - Queries by typed dependency identity and dependency semantics.
//! - Reporting of unresolved optional extension semantics.
//!
//! This module does not own:
//! - Graph nodes or dependency-edge definitions.
//! - Domain-specific operations or IR.
//! - Source syntax, predicates, type checking, or semantic analysis.
//! - Runtime readiness, completion, synchronization, or cancellation state.
//! - Scheduling, routing, target selection, or resource allocation.
//!
//! ## Canonical integration
//!
//! - `crate::hybrid::graph::model::HybridGraph` owns graph nodes.
//! - `crate::hybrid::graph::edges::HybridEdgeSet` owns dependency edges.
//! - `crate::hybrid::graph::edges::EdgeKind` defines edge semantics.
//! - `crate::hybrid::ids` owns typed graph-scoped identities.
//! - `crate::hybrid::config::HybridConfig` owns operational budgets.
//! - `crate::hybrid::contract::DependencyKind` remains the domain-contract
//!   declaration vocabulary. Adapters must map between it and `EdgeKind`
//!   explicitly; this module does not infer dependencies from strings.
//! - Canonical classical and quantum representations remain authoritative.
//! - `quantum::ir` remains the canonical quantum semantic boundary.
//! - Compiler and runtime integrations consume validated dependencies but
//!   retain ownership of planning and execution.
//!
//! ## Correctness
//!
//! Constructing an index validates the edge set against the graph before
//! exposing queries. A dependency is not evidence that its source has
//! executed, that its result is available, or that its target is feasible.
//!
//! A dependency edge describes a semantic relationship, not a physical
//! schedule. In particular, asynchronous execution does not remove a required
//! dependency, and completion dependencies must not be treated as satisfied
//! merely because an operation was submitted.
//!
//! Cycles are not rejected here. Feedback, loop-carried dependencies, and
//! repeated computation can legitimately form cycles. Legality is determined
//! by semantic-aware graph verification.
//!
//! Unknown optional extension semantics are preserved and reported.
//! Required unknown extension semantics must be rejected by the canonical
//! edge validator; this index does not reinterpret them as standard edges.
//!
//! ## Scalability
//!
//! No fixed maximum is imposed on nodes, edges, domains, or dependencies.
//! Index collections grow according to available resources.
//!
//! `with_config` enforces only budgets explicitly supplied through
//! `HybridConfig`. An absent budget does not mean infinite physical memory;
//! allocation and operating-system failures remain possible.
//!
//! Index construction is O(E log V) in the worst case for ordered-map
//! insertion, where E is the number of edges and V is the number of nodes
//! appearing as edge endpoints. A node query is O(1) for map lookup plus
//! O(k) to return its k matching edges. No repeated linear search of the
//! canonical edge set is required for indexed dependency queries.
//!
//! The index borrows edges instead of cloning them. It must not outlive the
//! graph's corresponding `HybridEdgeSet`.
//!
//! ## Determinism
//!
//! Each per-node index preserves the declaration order of the authoritative
//! edge set. Canonical query methods sort their results by typed dependency
//! identity without modifying the original representation.
//!
//! ## Safety
//!
//! Unsafe Rust is prohibited. No hardware capacity or universal workload
//! ceiling is embedded in this module.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::errors::HybridError;
use crate::hybrid::graph::edges::{
    EdgeKind,
    EdgeValidationReport,
    HybridEdge,
    HybridEdgeSet,
};
use crate::hybrid::graph::model::HybridGraph;
use crate::hybrid::ids::{
    HybridDependencyId,
    HybridEdgeId,
    HybridGraphId,
    HybridNodeId,
};

/// The semantic category of a hybrid dependency.
///
/// These variants correspond to the standard edge kinds in `graph/edges.rs`.
/// They describe relationships between computations; they do not define
/// execution mechanisms or physical scheduling decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum DependencySemantics {
    /// The target requires a value or result associated with the source.
    Data,

    /// The target depends on a control decision associated with the source.
    Control,

    /// The target requires a declared completion condition from the source.
    Completion,

    /// The source must precede the target under observable ordering rules.
    Ordering,

    /// The source and target participate in a causal feedback relationship.
    Feedback,

    /// The relationship crosses distinct semantic computation domains.
    DomainBoundary,

    /// The relationship's semantics are defined by an extension.
    Extension,
}

/// The classification of one authoritative dependency edge.
///
/// Extension metadata is copied into this small classification value so
/// callers can inspect extension identity without downcasting or parsing
/// display strings. The canonical edge remains the source of truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
#[non_exhaustive]
pub enum DependencyClassification {
    /// Data/result dependency.
    Data,

    /// Control-decision dependency.
    Control,

    /// Completion dependency.
    Completion,

    /// Observable ordering dependency.
    Ordering,

    /// Causal feedback dependency.
    Feedback,

    /// Cross-domain dependency.
    DomainBoundary,

    /// Extension-defined dependency whose meaning must be resolved by an
    /// appropriate extension-aware verifier or consumer.
    Extension {
        namespace: String,
        name: String,
        version: Option<String>,
        required: bool,
    },
}

impl DependencyClassification {
    /// Return the broad semantic category of this dependency.
    #[must_use]
    pub const fn semantics(&self) -> DependencySemantics {
        match self {
            Self::Data => DependencySemantics::Data,
            Self::Control => DependencySemantics::Control,
            Self::Completion => DependencySemantics::Completion,
            Self::Ordering => DependencySemantics::Ordering,
            Self::Feedback => DependencySemantics::Feedback,
            Self::DomainBoundary => DependencySemantics::DomainBoundary,
            Self::Extension { .. } => DependencySemantics::Extension,
        }
    }

    /// Return whether this classification is defined by an extension.
    #[must_use]
    pub const fn is_extension(&self) -> bool {
        matches!(self, Self::Extension { .. })
    }

    /// Return whether an extension dependency declares its semantics
    /// mandatory.
    ///
    /// `None` means that this is a standard dependency kind.
    #[must_use]
    pub const fn extension_is_required(&self) -> Option<bool> {
        match self {
            Self::Extension { required, .. } => Some(*required),
            _ => None,
        }
    }
}

/// Classify an existing canonical edge kind.
///
/// This function does not resolve extension semantics, validate endpoint
/// existence, or establish that the dependency can execute.
#[must_use]
pub fn classify_edge_kind(kind: &EdgeKind) -> DependencyClassification {
    match kind {
        EdgeKind::Data { .. } => DependencyClassification::Data,
        EdgeKind::Control => DependencyClassification::Control,
        EdgeKind::Completion { .. } => DependencyClassification::Completion,
        EdgeKind::Ordering => DependencyClassification::Ordering,
        EdgeKind::Feedback => DependencyClassification::Feedback,
        EdgeKind::DomainBoundary => DependencyClassification::DomainBoundary,
        EdgeKind::Extension(extension) => DependencyClassification::Extension {
            namespace: extension.namespace().to_owned(),
            name: extension.name().to_owned(),
            version: extension.version().map(str::to_owned),
            required: extension.is_required(),
        },
    }
}

/// A borrowed, validated index over the authoritative hybrid dependency edges.
///
/// This is a derived query structure, not a second graph. It cannot modify
/// edge definitions and does not clone the underlying edges.
///
/// Construct it only after the canonical `HybridEdgeSet` and `HybridGraph`
/// have been created. Keep both alive for at least as long as this index.
#[derive(Debug)]
pub struct HybridDependencyIndex<'a> {
    graph_id: HybridGraphId,
    edge_set: &'a HybridEdgeSet,
    by_source: BTreeMap<HybridNodeId, Vec<&'a HybridEdge>>,
    by_target: BTreeMap<HybridNodeId, Vec<&'a HybridEdge>>,
    by_dependency: BTreeMap<HybridDependencyId, &'a HybridEdge>,
    by_edge: BTreeMap<HybridEdgeId, &'a HybridEdge>,
    validation_report: EdgeValidationReport,
}

impl<'a> HybridDependencyIndex<'a> {
    /// Construct a validated dependency index without adding operational
    /// budgets beyond those already represented by the supplied data.
    ///
    /// This is equivalent to `with_config` using the default configuration,
    /// whose operational budgets are unspecified unless configured.
    pub fn new(
        graph: &'a HybridGraph,
        edge_set: &'a HybridEdgeSet,
    ) -> Result<Self, HybridError> {
        Self::with_config(graph, edge_set, &HybridConfig::default())
    }

    /// Construct a validated dependency index with explicit operational
    /// budget policy.
    ///
    /// The graph and edge set must describe the same graph. Canonical graph
    /// validation and edge validation are performed before the index is
    /// returned.
    ///
    /// A budget-exhaustion error indicates that this operation was not
    /// permitted to process the supplied workload under the chosen policy.
    /// It does not mean that the source program is semantically invalid.
    pub fn with_config(
        graph: &'a HybridGraph,
        edge_set: &'a HybridEdgeSet,
        config: &HybridConfig,
    ) -> Result<Self, HybridError> {
        config
            .check_budget(BudgetKind::GraphEdges, edge_set.len() as u128)
            .map_err(|error| error.in_component("hybrid dependency index"))?;

        config
            .check_budget(BudgetKind::ValidationSteps, edge_set.len() as u128)
            .map_err(|error| error.in_component("hybrid dependency index"))?;

        if graph.id() != edge_set.graph_id() {
            return Err(HybridError::invalid_reference(format!(
                "dependency index graph mismatch: graph {} and edge set {}",
                graph.id(),
                edge_set.graph_id()
            ))
            .in_component("hybrid dependency index"));
        }

        // The edge set is the canonical owner of edge relationships.
        // This validator checks local invariants, graph membership, endpoint
        // existence, and domain-boundary claims.
        let validation_report = edge_set
            .validate_against(graph)
            .map_err(|error| error.in_component("hybrid dependency index"))?;

        let mut by_source: BTreeMap<HybridNodeId, Vec<&'a HybridEdge>> =
            BTreeMap::new();
        let mut by_target: BTreeMap<HybridNodeId, Vec<&'a HybridEdge>> =
            BTreeMap::new();
        let mut by_dependency: BTreeMap<
            HybridDependencyId,
            &'a HybridEdge,
        > = BTreeMap::new();
        let mut by_edge: BTreeMap<HybridEdgeId, &'a HybridEdge> =
            BTreeMap::new();

        for edge in edge_set.edges() {
            // `validate_against` has already checked endpoint existence and
            // graph identity. These maps contain references, not cloned
            // dependency definitions.
            by_source.entry(edge.source()).or_default().push(edge);
            by_target.entry(edge.target()).or_default().push(edge);

            if by_dependency.insert(edge.dependency_id(), edge).is_some() {
                return Err(HybridError::invalid_dependency(format!(
                    "duplicate dependency identity {} in graph {}",
                    edge.dependency_id(),
                    graph.id()
                ))
                .in_component("hybrid dependency index"));
            }

            if by_edge.insert(edge.id(), edge).is_some() {
                return Err(HybridError::invalid_dependency(format!(
                    "duplicate edge identity {} in graph {}",
                    edge.id(),
                    graph.id()
                ))
                .in_component("hybrid dependency index"));
            }
        }

        Ok(Self {
            graph_id: graph.id(),
            edge_set,
            by_source,
            by_target,
            by_dependency,
            by_edge,
            validation_report,
        })
    }

    /// Return the graph identity associated with this index.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Return the canonical edge set from which this index was derived.
    #[must_use]
    pub const fn edge_set(&self) -> &'a HybridEdgeSet {
        self.edge_set
    }

    /// Return the number of indexed dependencies.
    #[must_use]
    pub fn len(&self) -> usize {
        self.by_edge.len()
    }

    /// Return whether this graph contains no dependencies.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_edge.is_empty()
    }

    /// Return the canonical edge-validation report.
    ///
    /// An unresolved optional extension is not equivalent to a fully
    /// understood semantic relationship. Consumers requiring complete
    /// semantic understanding must check the report before proceeding.
    #[must_use]
    pub const fn validation_report(&self) -> &EdgeValidationReport {
        &self.validation_report
    }

    /// Return whether every edge kind was understood by the canonical
    /// edge validator.
    #[must_use]
    pub fn has_unresolved_optional_extensions(&self) -> bool {
        !self
            .validation_report
            .unresolved_optional_extensions
            .is_empty()
    }

    /// Return the edge identified by a graph-scoped edge identity.
    #[must_use]
    pub fn edge(&self, id: HybridEdgeId) -> Option<&'a HybridEdge> {
        if id.graph() != self.graph_id {
            return None;
        }

        self.by_edge.get(&id).copied()
    }

    /// Return the edge associated with a graph-scoped dependency identity.
    #[must_use]
    pub fn dependency(
        &self,
        id: HybridDependencyId,
    ) -> Option<&'a HybridEdge> {
        if id.graph() != self.graph_id {
            return None;
        }

        self.by_dependency.get(&id).copied()
    }

    /// Return dependencies originating at a node, in declaration order.
    ///
    /// A source edge identifies a prerequisite/controller/causal predecessor;
    /// the precise meaning is determined by its edge kind.
    #[must_use]
    pub fn outgoing(
        &self,
        node: HybridNodeId,
    ) -> Vec<&'a HybridEdge> {
        if node.graph() != self.graph_id {
            return Vec::new();
        }

        self.by_source
            .get(&node)
            .map_or_else(Vec::new, |edges| edges.clone())
    }

    /// Return dependencies targeting a node, in declaration order.
    ///
    /// These edges identify relationships that the target must preserve.
    /// They do not prove that the prerequisites have executed or completed.
    #[must_use]
    pub fn incoming(
        &self,
        node: HybridNodeId,
    ) -> Vec<&'a HybridEdge> {
        if node.graph() != self.graph_id {
            return Vec::new();
        }

        self.by_target
            .get(&node)
            .map_or_else(Vec::new, |edges| edges.clone())
    }

    /// Return incoming edges of one semantic category, in declaration order.
    #[must_use]
    pub fn incoming_with_semantics(
        &self,
        node: HybridNodeId,
        semantics: DependencySemantics,
    ) -> Vec<&'a HybridEdge> {
        self.incoming(node)
            .into_iter()
            .filter(|edge| {
                classify_edge_kind(edge.kind()).semantics() == semantics
            })
            .collect()
    }

    /// Return outgoing edges of one semantic category, in declaration order.
    #[must_use]
    pub fn outgoing_with_semantics(
        &self,
        node: HybridNodeId,
        semantics: DependencySemantics,
    ) -> Vec<&'a HybridEdge> {
        self.outgoing(node)
            .into_iter()
            .filter(|edge| {
                classify_edge_kind(edge.kind()).semantics() == semantics
            })
            .collect()
    }

    /// Return dependencies in canonical identity order.
    ///
    /// The authoritative edge-set declaration order is not modified.
    #[must_use]
    pub fn dependencies_canonical_order(&self) -> Vec<&'a HybridEdge> {
        self.by_dependency.values().copied().collect()
    }

    /// Return a canonical-order list of dependency IDs entering a node.
    #[must_use]
    pub fn incoming_ids_canonical_order(
        &self,
        node: HybridNodeId,
    ) -> Vec<HybridDependencyId> {
        let mut ids: Vec<_> = self
            .incoming(node)
            .into_iter()
            .map(HybridEdge::dependency_id)
            .collect();

        ids.sort_unstable();
        ids
    }

    /// Return a canonical-order list of dependency IDs leaving a node.
    #[must_use]
    pub fn outgoing_ids_canonical_order(
        &self,
        node: HybridNodeId,
    ) -> Vec<HybridDependencyId> {
        let mut ids: Vec<_> = self
            .outgoing(node)
            .into_iter()
            .map(HybridEdge::dependency_id)
            .collect();

        ids.sort_unstable();
        ids
    }

    /// Return dependency identities with unresolved optional extension
    /// semantics, in canonical identity order.
    ///
    /// Required unknown extension semantics are expected to have been
    /// rejected by `HybridEdgeSet::validate_against`.
    #[must_use]
    pub fn unresolved_optional_dependency_ids(
        &self,
    ) -> Vec<HybridDependencyId> {
        let unresolved_edges: BTreeSet<HybridEdgeId> = self
            .validation_report
            .unresolved_optional_extensions
            .iter()
            .copied()
            .collect();

        let mut ids: Vec<_> = self
            .by_dependency
            .iter()
            .filter_map(|(dependency_id, edge)| {
                unresolved_edges
                    .contains(&edge.id())
                    .then_some(*dependency_id)
            })
            .collect();

        ids.sort_unstable();
        ids
    }

    /// Revalidate the canonical edge set against the supplied graph.
    ///
    /// This is useful at integration boundaries where a caller wants to
    /// confirm that the index's borrowed data still describes the expected
    /// graph. Rust borrowing prevents safe mutation of the borrowed edge set
    /// while this index exists, but it does not establish the validity of
    /// externally supplied semantic facts or target capabilities.
    pub fn validate_against(
        &self,
        graph: &HybridGraph,
    ) -> Result<EdgeValidationReport, HybridError> {
        if graph.id() != self.graph_id {
            return Err(HybridError::invalid_reference(format!(
                "dependency index belongs to graph {}, not graph {}",
                self.graph_id,
                graph.id()
            ))
            .in_component("hybrid dependency index"));
        }

        self.edge_set
            .validate_against(graph)
            .map_err(|error| error.in_component("hybrid dependency index"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::graph::edges::ExtensionEdgeKind;

    #[test]
    fn classifies_standard_dependency_kinds() {
        assert_eq!(
            classify_edge_kind(&EdgeKind::Data { value: None }).semantics(),
            DependencySemantics::Data
        );

        assert_eq!(
            classify_edge_kind(&EdgeKind::Control).semantics(),
            DependencySemantics::Control
        );

        assert_eq!(
            classify_edge_kind(&EdgeKind::Completion { event: None }).semantics(),
            DependencySemantics::Completion
        );

        assert_eq!(
            classify_edge_kind(&EdgeKind::Ordering).semantics(),
            DependencySemantics::Ordering
        );

        assert_eq!(
            classify_edge_kind(&EdgeKind::Feedback).semantics(),
            DependencySemantics::Feedback
        );

        assert_eq!(
            classify_edge_kind(&EdgeKind::DomainBoundary).semantics(),
            DependencySemantics::DomainBoundary
        );
    }

    #[test]
    fn preserves_extension_identity_and_requiredness() {
        let extension = ExtensionEdgeKind::new(
            "example.org.hybrid",
            "causal_dependency",
            Some("1".to_owned()),
            true,
        )
        .expect("valid extension identity");

        let classification =
            classify_edge_kind(&EdgeKind::Extension(extension));

        assert_eq!(
            classification.semantics(),
            DependencySemantics::Extension
        );
        assert!(classification.is_extension());
        assert_eq!(classification.extension_is_required(), Some(true));

        match classification {
            DependencyClassification::Extension {
                namespace,
                name,
                version,
                required,
            } => {
                assert_eq!(namespace, "example.org.hybrid");
                assert_eq!(name, "causal_dependency");
                assert_eq!(version.as_deref(), Some("1"));
                assert!(required);
            }
            _ => panic!("expected extension classification"),
        }
    }

    #[test]
    fn standard_kinds_have_no_extension_requiredness() {
        let classification = classify_edge_kind(&EdgeKind::Feedback);

        assert_eq!(classification.extension_is_required(), None);
        assert!(!classification.is_extension());
    }
}
