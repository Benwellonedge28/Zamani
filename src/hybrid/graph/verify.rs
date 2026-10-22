
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Graph Verification
//!
//! Path: `src/hybrid/graph/verify.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: Rust 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Verify a hybrid graph and its separately stored edge set before those
//! representations are consumed by compiler planning, interchange, or
//! runtime integration.
//!
//! The verifier separates:
//!
//! - structural verification, which checks the graph and edge relationships;
//! - semantic verification, which delegates authoritative questions to
//!   existing domain and canonical-IR implementations.
//!
//! This module is not a second semantic analyzer, universal IR, quantum IR,
//! compiler, scheduler, or runtime.
//!
//! ## Important limitations
//!
//! `HybridGraph` stores nodes and composition; `HybridEdgeSet` stores edges.
//! Both objects must be supplied to the verifier.
//!
//! Structural verification cannot independently establish that a referenced
//! canonical IR entity exists, that a domain implementation satisfies its
//! interface contract, or that every dependency required by the source
//! program has been represented. Those checks require a semantic resolver.
//!
//! A cycle is not inherently invalid. Feedback, iteration, and other
//! semantically valid cyclic relationships must not be rejected by a generic
//! DAG check. The resolver is responsible for checking cycle legality against
//! the authoritative semantics.
//!
//! ## Scalability
//!
//! This module introduces no fixed graph-size, domain-count, node-count,
//! edge-count, or nesting ceiling. It uses growable standard-library
//! collections and checked accounting. Explicit operational limits are
//! enforced through `HybridConfig`.
//!
//! Finite address spaces, allocation failure, identifier exhaustion, and
//! explicitly configured operational budgets remain real constraints.
//!
//! ## Ownership
//!
//! - `graph/model.rs` owns the graph and node representation.
//! - `graph/edges.rs` owns edge identities, kinds, and local edge invariants.
//! - `domain/composition.rs` owns domain-instance composition.
//! - Canonical domain IRs own operation meaning and entity resolution.
//! - This module owns whole-graph verification orchestration and its report.
//!
//! ## Integration
//!
//! Compiler and planning entry points that require semantic confidence should
//! call `verify_with_resolver`, not treat a structural-only report as proof
//! that a computation is executable or semantically complete.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::domain::composition::DomainInstance;
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::ids::{
    DomainInstanceId,
    HybridEdgeId,
    HybridGraphId,
    HybridNodeId,
};

use super::edges::{
    EdgeKind,
    HybridEdge,
    HybridEdgeSet,
};
use super::model::{
    CanonicalIrRef,
    HybridGraph,
    HybridNode,
};

/// A resolver backed by the authoritative domain registries and canonical
/// representations.
///
/// Implementations belong in integration adapters. They must not create
/// substitute domain IRs or claim successful resolution merely because an
/// identifier is syntactically valid.
///
/// Every method must either establish the stated property or return a
/// structured error. A successful result is an assertion by the resolver
/// that it performed the requested check against the relevant authority.
///
/// `validate_required_dependencies` must verify that all dependencies
/// required by the authoritative semantic representation are present and
/// represented with relationships that preserve their meaning.
///
/// `validate_edge_semantics` must check any edge-specific semantic rules,
/// including the legality of cycles when the relevant language construct
/// imposes restrictions.
///
/// This trait deliberately does not select hardware or perform lowering.
pub trait HybridSemanticResolver {
    /// Resolve and validate the interface contract referenced by a domain
    /// instance against the authoritative domain registry.
    fn validate_domain_instance(
        &self,
        graph: &HybridGraph,
        instance: &DomainInstance,
    ) -> Result<(), HybridError>;

    /// Resolve a node's canonical IR reference in the context of its
    /// participating domain instance.
    ///
    /// The resolver must establish that the referenced entity exists and
    /// that the reference is valid for the declared domain instance.
    fn validate_canonical_reference(
        &self,
        graph: &HybridGraph,
        node: &HybridNode,
        instance: &DomainInstance,
        reference: &CanonicalIrRef,
    ) -> Result<(), HybridError>;

    /// Return whether the resolver understands the semantics of this edge
    /// kind. Built-in kinds are known to the structural verifier; extension
    /// kinds require explicit support from the resolver for semantic
    /// verification.
    fn understands_edge_kind(&self, kind: &EdgeKind) -> bool {
        !kind.is_extension()
    }

    /// Validate the meaning of an edge against authoritative domain and
    /// language semantics.
    fn validate_edge_semantics(
        &self,
        graph: &HybridGraph,
        edge: &HybridEdge,
    ) -> Result<(), HybridError>;

    /// Check that all semantically required dependencies are represented.
    ///
    /// The graph structure alone cannot prove that an edge has not been
    /// omitted from an otherwise structurally valid graph.
    fn validate_required_dependencies(
        &self,
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
    ) -> Result<(), HybridError>;
}

/// Result of graph verification.
///
/// A report explicitly distinguishes structural success from semantic
/// completeness. Callers must not interpret a structurally valid graph as
/// proof that all canonical references and required dependencies were
/// resolved.
///
/// Counts use `u128` so they are not narrowed to a platform-sized counter
/// merely for reporting. The underlying collections remain bounded by the
/// resources available to the running process.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationReport {
    /// Identity of the verified graph.
    pub graph_id: HybridGraphId,

    /// Number of graph nodes structurally verified.
    pub verified_nodes: u128,

    /// Number of graph edges structurally verified.
    pub verified_edges: u128,

    /// Number of domain instances structurally verified.
    pub verified_domain_instances: u128,

    /// Number of edges explicitly verified to cross semantic domains.
    pub verified_domain_boundary_edges: u128,

    /// Optional extension edges whose semantics were not resolved.
    ///
    /// Such edges may be preserved for inspection or interchange, but the
    /// report is not fully semantically verified while this list is nonempty.
    pub unresolved_optional_extensions: Vec<HybridEdgeId>,

    /// Domain instances whose interface contracts were not resolved.
    ///
    /// This list is populated by structural-only verification.
    pub unresolved_domain_contracts: Vec<DomainInstanceId>,

    /// Nodes whose canonical IR references were not resolved.
    ///
    /// This list is populated by structural-only verification.
    pub unresolved_canonical_references: Vec<HybridNodeId>,

    /// Whether an authoritative semantic resolver completed its checks.
    pub semantic_checks_completed: bool,

    /// Whether required dependencies were checked against authoritative
    /// semantic information.
    pub required_dependencies_checked: bool,
}

impl VerificationReport {
    /// Return whether the result proves structural correctness only.
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        true
    }

    /// Return whether the report represents completed semantic verification.
    ///
    /// This does not establish that any particular physical target can
    /// execute the computation. Target feasibility is a downstream check.
    #[must_use]
    pub fn is_fully_semantically_verified(&self) -> bool {
        self.semantic_checks_completed
            && self.required_dependencies_checked
            && self.unresolved_optional_extensions.is_empty()
            && self.unresolved_domain_contracts.is_empty()
            && self.unresolved_canonical_references.is_empty()
    }

    /// Return whether unresolved optional extension semantics remain.
    #[must_use]
    pub fn has_unresolved_optional_extensions(&self) -> bool {
        !self.unresolved_optional_extensions.is_empty()
    }
}

/// Stateless hybrid graph verifier.
///
/// The verifier keeps no global registry, cache, target state, or mutable
/// process-wide configuration. Callers supply the graph, edge set, policy,
/// and—when semantic verification is required—the authoritative resolver.
#[derive(Debug, Default, Clone, Copy)]
pub struct HybridGraphVerifier;

impl HybridGraphVerifier {
    /// Verify graph structure without claiming canonical IR resolution.
    ///
    /// This operation validates:
    ///
    /// - graph and composition invariants;
    /// - node identity and domain-instance membership;
    /// - edge identity and dependency uniqueness;
    /// - edge endpoint existence;
    /// - graph-scoped identity consistency;
    /// - domain-boundary edge claims;
    /// - required versus optional extension handling;
    /// - explicitly configured graph, edge, metadata, and validation budgets.
    ///
    /// This method does not resolve canonical IR references or establish that
    /// every required semantic dependency has been represented.
    ///
    /// Use `verify_with_resolver` before accepting a graph as semantically
    /// complete for compilation or realization.
    pub fn verify(
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
        config: &HybridConfig,
    ) -> Result<VerificationReport, HybridError> {
        Self::verify_inner(graph, edges, config, None)
    }

    /// Verify graph structure and delegate semantic checks to an
    /// authoritative resolver.
    ///
    /// This additionally validates:
    ///
    /// - domain-interface contract resolution;
    /// - canonical IR reference resolution;
    /// - edge semantics;
    /// - presence of semantically required dependencies;
    /// - extension-edge semantics supported by the resolver.
    ///
    /// The resolver does not prove target feasibility. The compiler's
    /// capability and feasibility stages must still evaluate the requested
    /// computation against an explicitly described target.
    pub fn verify_with_resolver(
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
        config: &HybridConfig,
        resolver: &dyn HybridSemanticResolver,
    ) -> Result<VerificationReport, HybridError> {
        Self::verify_inner(graph, edges, config, Some(resolver))
    }

    fn verify_inner(
        graph: &HybridGraph,
        edges: &HybridEdgeSet,
        config: &HybridConfig,
        resolver: Option<&dyn HybridSemanticResolver>,
    ) -> Result<VerificationReport, HybridError> {
        // Validate configuration-dependent graph invariants first.
        graph
            .validate_with_config(config)
            .map_err(|error| error.in_component("hybrid graph verification"))?;

        edges
            .validate_local()
            .map_err(|error| error.in_component("hybrid graph verification"))?;

        if edges.graph_id() != graph.id() {
            return Err(HybridError::new(
                HybridErrorCode::InvalidReference,
                format!(
                    "edge set belongs to graph {}, but supplied graph is {}",
                    edges.graph_id(),
                    graph.id()
                ),
            )
            .in_component("hybrid graph verification"));
        }

        // Check explicit budgets before building auxiliary indexes.
        config
            .check_budget(BudgetKind::GraphNodes, graph.node_count() as u128)
            .map_err(|error| error.in_component("hybrid graph verification"))?;

        config
            .check_budget(BudgetKind::GraphEdges, edges.len() as u128)
            .map_err(|error| error.in_component("hybrid graph verification"))?;

        config
            .check_budget(
                BudgetKind::DomainInstances,
                graph.composition().len() as u128,
            )
            .map_err(|error| error.in_component("hybrid graph verification"))?;

        // The work estimate accounts for one visit per domain, node, and
        // edge. It is an explicit, deterministic estimate—not a claim that
        // all internal operations have identical computational cost.
        let validation_steps = (graph.composition().len() as u128)
            .checked_add(graph.node_count() as u128)
            .and_then(|value| value.checked_add(edges.len() as u128))
            .ok_or_else(|| {
                HybridError::new(
                    HybridErrorCode::InvalidRepresentation,
                    "hybrid verification work accounting overflowed",
                )
            })?;

        config
            .check_budget(BudgetKind::ValidationSteps, validation_steps)
            .map_err(|error| error.in_component("hybrid graph verification"))?;

        // Build indexes once. The graph model deliberately preserves
        // declaration order; these indexes are temporary verification
        // structures and do not change canonical representation order.
        let mut instances: BTreeMap<DomainInstanceId, &DomainInstance> =
            BTreeMap::new();

        for instance in graph.composition().instances() {
            if instances.insert(instance.id(), instance).is_some() {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!(
                        "domain-instance identity {} occurs more than once",
                        instance.id()
                    ),
                )
                .in_component("hybrid graph verification"));
            }
        }

        let mut nodes: BTreeMap<HybridNodeId, &HybridNode> = BTreeMap::new();

        for node in graph.nodes() {
            node.validate()
                .map_err(|error| error.in_component("hybrid graph verification"))?;

            if node.id().graph() != graph.id() {
                return Err(HybridError::new(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "node {} belongs to graph {}, not graph {}",
                        node.id(),
                        node.id().graph(),
                        graph.id()
                    ),
                )
                .in_component("hybrid graph verification"));
            }

            if !instances.contains_key(&node.domain_instance()) {
                return Err(HybridError::new(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "node {} references missing domain instance {}",
                        node.id(),
                        node.domain_instance()
                    ),
                )
                .in_component("hybrid graph verification"));
            }

            if nodes.insert(node.id(), node).is_some() {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!("node identity {} occurs more than once", node.id()),
                )
                .in_component("hybrid graph verification"));
            }
        }

        let mut report = VerificationReport {
            graph_id: graph.id(),
            verified_nodes: graph.node_count() as u128,
            verified_edges: 0,
            verified_domain_instances: graph.composition().len() as u128,
            verified_domain_boundary_edges: 0,
            unresolved_optional_extensions: Vec::new(),
            unresolved_domain_contracts: Vec::new(),
            unresolved_canonical_references: Vec::new(),
            semantic_checks_completed: resolver.is_some(),
            required_dependencies_checked: false,
        };

        // Structural-only verification must not imply that contracts or
        // canonical entities have been resolved.
        if resolver.is_none() {
            report
                .unresolved_domain_contracts
                .extend(instances.keys().copied());

            report
                .unresolved_canonical_references
                .extend(nodes.keys().copied());
        }

        // Resolve domain contracts before relying on domain identity to
        // validate cross-domain relationships.
        if let Some(resolver) = resolver {
            for instance in graph.composition().instances() {
                resolver
                    .validate_domain_instance(graph, instance)
                    .map_err(|error| {
                        error
                            .in_component("hybrid semantic verification")
                            .at_entity(
                                "domain instance",
                                instance.id().to_string(),
                                "resolve and validate the canonical domain interface contract",
                            )
                    })?;
            }

            for node in graph.nodes() {
                let instance = instances
                    .get(&node.domain_instance())
                    .copied()
                    .ok_or_else(|| {
                        HybridError::new(
                            HybridErrorCode::InternalInvariantViolation,
                            format!(
                                "validated node {} lost its domain-instance index",
                                node.id()
                            ),
                        )
                    })?;

                resolver
                    .validate_canonical_reference(
                        graph,
                        node,
                        instance,
                        node.operation(),
                    )
                    .map_err(|error| {
                        error
                            .in_component("hybrid semantic verification")
                            .at_entity(
                                "hybrid node",
                                node.id().to_string(),
                                "resolve the node against its canonical domain IR",
                            )
                    })?;
            }
        }

        // Verify each edge using the prebuilt node index. This avoids the
        // repeated linear node lookups in HybridEdgeSet::validate_against.
        for edge in edges.edges() {
            edge.validate()
                .map_err(|error| error.in_component("hybrid graph verification"))?;

            if edge.id().graph() != graph.id() {
                return Err(HybridError::new(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "edge {} belongs to graph {}, not graph {}",
                        edge.id(),
                        edge.id().graph(),
                        graph.id()
                    ),
                )
                .in_component("hybrid graph verification"));
            }

            let source = nodes.get(&edge.source()).copied().ok_or_else(|| {
                HybridError::new(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "edge {} references missing source node {}",
                        edge.id(),
                        edge.source()
                    ),
                )
                .in_component("hybrid graph verification")
            })?;

            let target = nodes.get(&edge.target()).copied().ok_or_else(|| {
                HybridError::new(
                    HybridErrorCode::InvalidReference,
                    format!(
                        "edge {} references missing target node {}",
                        edge.id(),
                        edge.target()
                    ),
                )
                .in_component("hybrid graph verification")
            })?;

            match edge.kind() {
                EdgeKind::DomainBoundary => {
                    let source_instance = instances
                        .get(&source.domain_instance())
                        .copied()
                        .ok_or_else(|| {
                            HybridError::new(
                                HybridErrorCode::InvalidReference,
                                format!(
                                    "edge {} source node references missing domain instance {}",
                                    edge.id(),
                                    source.domain_instance()
                                ),
                            )
                        })?;

                    let target_instance = instances
                        .get(&target.domain_instance())
                        .copied()
                        .ok_or_else(|| {
                            HybridError::new(
                                HybridErrorCode::InvalidReference,
                                format!(
                                    "edge {} target node references missing domain instance {}",
                                    edge.id(),
                                    target.domain_instance()
                                ),
                            )
                        })?;

                    let source_domain = source_instance
                        .domain_id()
                        .map_err(|error| {
                            error.in_component("hybrid graph verification")
                        })?;

                    let target_domain = target_instance
                        .domain_id()
                        .map_err(|error| {
                            error.in_component("hybrid graph verification")
                        })?;

                    if source_domain == target_domain {
                        return Err(HybridError::new(
                            HybridErrorCode::InvalidDependency,
                            format!(
                                "domain-boundary edge {} connects nodes in the same semantic domain",
                                edge.id()
                            ),
                        )
                        .in_component("hybrid graph verification"));
                    }

                    report.verified_domain_boundary_edges = report
                        .verified_domain_boundary_edges
                        .checked_add(1)
                        .ok_or_else(|| {
                            HybridError::new(
                                HybridErrorCode::InvalidRepresentation,
                                "domain-boundary edge accounting overflowed",
                            )
                        })?;
                }

                EdgeKind::Extension(extension) => {
                    let understood = resolver
                        .map(|value| value.understands_edge_kind(edge.kind()))
                        .unwrap_or(false);

                    if !understood && extension.is_required() {
                        return Err(HybridError::new(
                            HybridErrorCode::UnsupportedOperation,
                            format!(
                                "edge {} requires unresolved extension semantics `{extension}`",
                                edge.id()
                            ),
                        )
                        .in_component("hybrid graph verification")
                        .at_entity(
                            "hybrid edge",
                            edge.id().to_string(),
                            "register a resolver that implements this required extension",
                        ));
                    }

                    if !understood {
                        if !config
                            .validation
                            .preserve_unknown_optional_extensions
                        {
                            return Err(HybridError::new(
                                HybridErrorCode::UnsupportedOperation,
                                format!(
                                    "optional extension `{extension}` on edge {} is not permitted by the active validation policy",
                                    edge.id()
                                ),
                            )
                            .in_component("hybrid graph verification"));
                        }

                        report.unresolved_optional_extensions.push(edge.id());
                    }
                }

                _ => {}
            }

            if let Some(resolver) = resolver {
                resolver
                    .validate_edge_semantics(graph, edge)
                    .map_err(|error| {
                        error
                            .in_component("hybrid semantic verification")
                            .at_entity(
                                "hybrid edge",
                                edge.id().to_string(),
                                "validate the edge against canonical dependency and control semantics",
                            )
                    })?;
            }

            report.verified_edges = report
                .verified_edges
                .checked_add(1)
                .ok_or_else(|| {
                    HybridError::new(
                        HybridErrorCode::InvalidRepresentation,
                        "verified-edge accounting overflowed",
                    )
                })?;
        }

        // Canonical IDs and BTree-based temporary indexes make the report
        // deterministic for the same graph, edge set, and resolver behavior.
        report
            .unresolved_optional_extensions
            .sort_unstable();
        report
            .unresolved_optional_extensions
            .dedup();

        if let Some(resolver) = resolver {
            resolver
                .validate_required_dependencies(graph, edges)
                .map_err(|error| {
                    error.in_component("hybrid required-dependency verification")
                })?;

            report.required_dependencies_checked = true;
        }

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structural_report_does_not_claim_semantic_completeness() {
        let report = VerificationReport {
            graph_id: test_graph_id(),
            verified_nodes: 0,
            verified_edges: 0,
            verified_domain_instances: 0,
            verified_domain_boundary_edges: 0,
            unresolved_optional_extensions: Vec::new(),
            unresolved_domain_contracts: Vec::new(),
            unresolved_canonical_references: Vec::new(),
            semantic_checks_completed: false,
            required_dependencies_checked: false,
        };

        assert!(report.is_structurally_valid());
        assert!(!report.is_fully_semantically_verified());
    }

    // This helper tests report behavior without inventing a graph fixture
    // or assuming how domain contracts and canonical IRs are constructed.
    fn test_graph_id() -> HybridGraphId {
        use crate::hybrid::ids::IdComponent;

        HybridGraphId::new(IdComponent::new(1).expect("non-zero test ID").get())
            .expect("non-zero test graph ID")
    }
}
