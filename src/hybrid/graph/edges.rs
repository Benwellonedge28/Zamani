
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Hybrid Computing — Typed Graph Edges
//!
//! Path: `src/hybrid/graph/edges.rs`
//! Edition: Rust 2021
//! Minimum supported Rust: 1.97
//! Safety: Unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Define the canonical, typed relationships between nodes in a
//! [`HybridGraph`](super::model::HybridGraph).
//!
//! The edge layer represents dependencies and coordination relationships
//! across classical, quantum, tensor, AI/ML, accelerator, HDL, distributed,
//! networking, and future computational domains.
//!
//! An edge describes a relationship between canonical operations. It does
//! not copy those operations or define their domain-specific semantics.
//!
//! ## Ownership
//!
//! This module owns:
//! - Hybrid edge identity and dependency identity.
//! - Directed source and target node references.
//! - The standard hybrid dependency categories.
//! - Explicit, namespaced extension edge kinds.
//! - Edge-local descriptive metadata.
//! - Edge-set structural validation and graph-reference validation.
//! - Reporting of optional extension relationships whose semantics are
//!   not understood by this implementation.
//!
//! This module does not own:
//! - Source syntax, ASTs, type checking, or semantic analysis.
//! - Classical instructions, quantum operations, or any domain-specific IR.
//! - Data-value definitions or ownership checking.
//! - Control-flow construction or synchronization execution.
//! - Physical device selection, allocation, scheduling, routing, QEC,
//!   calibration, pulse generation, or backend code generation.
//! - Runtime handles, vendor SDKs, or network transports.
//!
//! ## Semantic contract
//!
//! All edges are directed. `source` is the prerequisite, producer,
//! controller, or causally preceding participant; `target` is the
//! dependent, consumer, controlled participant, or causally following
//! participant. The exact interpretation is determined by `kind`.
//!
//! The standard kinds are:
//! - `Data`: a value or result dependency.
//! - `Control`: a control-decision dependency.
//! - `Completion`: a completion dependency.
//! - `Ordering`: an observable ordering requirement.
//! - `Feedback`: a causal feedback relationship.
//! - `DomainBoundary`: a relationship crossing distinct semantic domains.
//!
//! A graph may contain multiple edges between the same pair of nodes when
//! they represent different relationships. Self-edges are not universally
//! prohibited: a loop-carried or feedback relationship may legitimately
//! refer to the same graph node. Whether a cycle is legal depends on the
//! declared computation semantics and belongs to graph verification.
//!
//! Unknown extension kinds are never treated as understood semantics.
//! Required extension semantics cause validation to fail unless a higher
//! layer explicitly resolves them. Optional extensions may be preserved,
//! but validation reports their unresolved status so callers cannot mistake
//! structural validity for full semantic verification.
//!
//! ## Scalability
//!
//! This module defines no fixed maximum for graph edges, domains, nodes,
//! dependencies, or extension kinds. Collections grow according to available
//! resources. Callers may supply `HybridConfig` to enforce explicit
//! operational budgets.
//!
//! No configured budget may silently alter program semantics. Budget
//! exhaustion is an explicit error. Finite identifier representations and
//! finite machine resources remain real constraints; neither is a language
//! semantics ceiling.
//!
//! ## Determinism
//!
//! Edge declaration order is preserved. `edges_canonical_order()` supplies
//! an identifier-sorted view without mutating that order. Descriptive
//! metadata uses `BTreeMap` for stable key ordering.
//!
//! IDs are identities, not content hashes. Canonical fingerprints must be
//! defined by the repository's canonical fingerprint or interchange layer.
//!
//! ## Integration
//!
//! - `super::model` supplies `HybridGraph`, `HybridNode`, and node lookup.
//! - `crate::hybrid::ids` supplies graph-scoped typed IDs.
//! - `crate::hybrid::errors` supplies structured errors.
//! - `crate::hybrid::config` supplies optional operational budgets.
//! - `crate::hybrid::contract::DependencyKind` is the corresponding
//!   domain-contract vocabulary; adapters must map explicitly between the
//!   two rather than relying on string parsing.
//! - `control/`, `sync/`, `graph/builder.rs`, `graph/verify.rs`, and
//!   `graph/analysis.rs` consume these definitions.
//! - Compiler and runtime integrations must preserve these relationships.
//! - The quantum semantic boundary remains `quantum::ir`.
//!
//! `HybridEdgeSet` is deliberately separate from `HybridGraph` because the
//! current `HybridGraph` model does not contain an edge collection. The
//! builder, verifier, and interchange layer must carry both objects together
//! until a separately reviewed graph-model change integrates them directly.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::hybrid::config::{BudgetKind, HybridConfig};
use crate::hybrid::errors::{HybridError, HybridErrorCode};
use crate::hybrid::ids::{
    CrossDomainValueId,
    HybridDependencyId,
    HybridEdgeId,
    HybridGraphId,
    HybridNodeId,
    SynchronizationEventId,
};

use super::model::HybridGraph;

/// A standard hybrid dependency relationship or an explicitly namespaced
/// extension relationship.
///
/// The standard variants have stable, built-in meanings. An extension
/// variant identifies semantics owned by another registered component;
/// its name alone does not make those semantics understood by this crate.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum EdgeKind {
    /// The target depends on a value or result associated with the source.
    ///
    /// When `value` is present, it identifies a graph-scoped cross-domain
    /// value. Absence does not mean that a data dependency can be ignored:
    /// the canonical IR adapter must still establish the dependency's
    /// actual value semantics.
    Data {
        value: Option<CrossDomainValueId>,
    },

    /// The target's execution or behavior depends on a control decision
    /// associated with the source.
    Control,

    /// The target requires the source to reach the completion condition
    /// specified by the canonical semantic or runtime contract.
    ///
    /// An optional event ID correlates the relationship with a declared
    /// synchronization event. It does not itself prove that the event exists
    /// or that a runtime has completed it.
    Completion {
        event: Option<SynchronizationEventId>,
    },

    /// An observable ordering relationship from source to target.
    ///
    /// This is not a physical scheduling decision. The compiler and runtime
    /// must preserve the language-required ordering guarantees.
    Ordering,

    /// A causal feedback relationship, including measurement-to-classical
    /// decision and classical-decision-to-subsequent-quantum-operation flows.
    ///
    /// Feedback can participate in a valid cycle. A universal DAG rule must
    /// not be inferred from this edge kind.
    Feedback,

    /// A relationship that crosses distinct semantic domain identifiers.
    ///
    /// Structural containment or a mere change of domain instance does not,
    /// by itself, establish a semantic domain boundary.
    DomainBoundary,

    /// A relationship whose semantics are defined by an extension.
    ///
    /// The extension is preserved as data, but this module does not claim
    /// to understand or execute it.
    Extension(ExtensionEdgeKind),
}

impl EdgeKind {
    /// Return a stable name for a built-in kind.
    ///
    /// Extension kinds are represented by their namespaced identity through
    /// `ExtensionEdgeKind`; this method does not flatten them into a
    /// potentially ambiguous string.
    #[must_use]
    pub const fn builtin_name(&self) -> Option<&'static str> {
        match self {
            Self::Data { .. } => Some("data"),
            Self::Control => Some("control"),
            Self::Completion { .. } => Some("completion"),
            Self::Ordering => Some("ordering"),
            Self::Feedback => Some("feedback"),
            Self::DomainBoundary => Some("domain_boundary"),
            Self::Extension(_) => None,
        }
    }

    /// Return whether this kind requires extension-specific interpretation.
    #[must_use]
    pub const fn is_extension(&self) -> bool {
        matches!(self, Self::Extension(_))
    }

    /// Return whether this kind is fully understood by the built-in model.
    ///
    /// Built-in kind recognition is not proof that the referenced domain
    /// operations satisfy the dependency. That is checked by higher layers.
    #[must_use]
    pub const fn has_builtin_semantics(&self) -> bool {
        !self.is_extension()
    }

    fn validate(&self, graph_id: HybridGraphId) -> Result<(), HybridError> {
        match self {
            Self::Data { value: Some(value) } if value.graph() != graph_id => {
                Err(HybridError::invalid_reference(format!(
                    "cross-domain value {value} does not belong to graph {graph_id}"
                )))
            }
            Self::Completion { event: Some(event) }
                if event.graph() != graph_id =>
            {
                Err(HybridError::invalid_reference(format!(
                    "synchronization event {event} does not belong to graph {graph_id}"
                )))
            }
            Self::Extension(extension) => extension.validate(),
            _ => Ok(()),
        }
    }

    fn estimated_metadata_bytes(&self) -> Result<u128, HybridError> {
        match self {
            Self::Extension(extension) => extension.estimated_bytes(),
            _ => Ok(0),
        }
    }
}

impl fmt::Display for EdgeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Data { .. } => formatter.write_str("data"),
            Self::Control => formatter.write_str("control"),
            Self::Completion { .. } => formatter.write_str("completion"),
            Self::Ordering => formatter.write_str("ordering"),
            Self::Feedback => formatter.write_str("feedback"),
            Self::DomainBoundary => formatter.write_str("domain_boundary"),
            Self::Extension(extension) => write!(formatter, "{extension}"),
        }
    }
}

/// An extensible edge-kind identity.
///
/// `namespace` identifies the component that owns the extension semantics;
/// `name` identifies the relationship within that namespace. `version` is
/// an optional extension-contract version string, not a language, graph, or
/// serialization version.
///
/// `required` means that the consumer must understand the extension's
/// semantics before claiming full validation. It is not a hint to ignore an
/// unknown relationship.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "ExtensionEdgeKindWire")]
pub struct ExtensionEdgeKind {
    namespace: String,
    name: String,
    version: Option<String>,
    required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ExtensionEdgeKindWire {
    namespace: String,
    name: String,
    version: Option<String>,
    required: bool,
}

impl TryFrom<ExtensionEdgeKindWire> for ExtensionEdgeKind {
    type Error = HybridError;

    fn try_from(value: ExtensionEdgeKindWire) -> Result<Self, Self::Error> {
        Self::new(
            value.namespace,
            value.name,
            value.version,
            value.required,
        )
    }
}

impl ExtensionEdgeKind {
    /// Construct a namespaced extension kind.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        version: Option<String>,
        required: bool,
    ) -> Result<Self, HybridError> {
        let value = Self {
            namespace: namespace.into(),
            name: name.into(),
            version,
            required,
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the extension namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Return the extension name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the optional extension-contract version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Return whether this extension's semantics are required.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    /// Validate the extension identity.
    pub fn validate(&self) -> Result<(), HybridError> {
        validate_identifier("edge extension namespace", &self.namespace)?;
        validate_identifier("edge extension name", &self.name)?;

        if let Some(version) = &self.version {
            validate_identifier("edge extension version", version)?;
        }

        Ok(())
    }

    fn estimated_bytes(&self) -> Result<u128, HybridError> {
        let mut total = checked_add(0, self.namespace.len() as u128)?;
        total = checked_add(total, self.name.len() as u128)?;

        if let Some(version) = &self.version {
            total = checked_add(total, version.len() as u128)?;
        }

        Ok(total)
    }
}

impl fmt::Display for ExtensionEdgeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}::{}", self.namespace, self.name)?;

        if let Some(version) = &self.version {
            write!(formatter, "@{version}")?;
        }

        Ok(())
    }
}

/// One directed dependency edge in a hybrid graph.
///
/// Both IDs and endpoints are graph-scoped. The separate edge and dependency
/// identities allow diagnostics and dependency declarations to refer to
/// stable, distinct concepts without confusing identity with relationship
/// semantics.
///
/// `metadata` is descriptive only. It must never override `kind`, alter
/// program semantics, or serve as an implicit target-selection mechanism.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "HybridEdgeWire")]
pub struct HybridEdge {
    id: HybridEdgeId,
    dependency_id: HybridDependencyId,
    source: HybridNodeId,
    target: HybridNodeId,
    kind: EdgeKind,
    metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct HybridEdgeWire {
    id: HybridEdgeId,
    dependency_id: HybridDependencyId,
    source: HybridNodeId,
    target: HybridNodeId,
    kind: EdgeKind,
    #[serde(default)]
    metadata: BTreeMap<String, String>,
}

impl TryFrom<HybridEdgeWire> for HybridEdge {
    type Error = HybridError;

    fn try_from(value: HybridEdgeWire) -> Result<Self, Self::Error> {
        Self::new(
            value.id,
            value.dependency_id,
            value.source,
            value.target,
            value.kind,
            value.metadata,
        )
    }
}

impl HybridEdge {
    /// Construct an edge and validate all locally checkable invariants.
    ///
    /// Node existence, domain membership, and target-specific meaning require
    /// validation against the complete graph and the relevant domain adapters.
    pub fn new(
        id: HybridEdgeId,
        dependency_id: HybridDependencyId,
        source: HybridNodeId,
        target: HybridNodeId,
        kind: EdgeKind,
        metadata: BTreeMap<String, String>,
    ) -> Result<Self, HybridError> {
        let value = Self {
            id,
            dependency_id,
            source,
            target,
            kind,
            metadata,
        };

        value.validate()?;
        Ok(value)
    }

    /// Return the edge identity.
    #[must_use]
    pub const fn id(&self) -> HybridEdgeId {
        self.id
    }

    /// Return the dependency identity.
    #[must_use]
    pub const fn dependency_id(&self) -> HybridDependencyId {
        self.dependency_id
    }

    /// Return the source node.
    #[must_use]
    pub const fn source(&self) -> HybridNodeId {
        self.source
    }

    /// Return the target node.
    #[must_use]
    pub const fn target(&self) -> HybridNodeId {
        self.target
    }

    /// Return the edge kind.
    #[must_use]
    pub const fn kind(&self) -> &EdgeKind {
        &self.kind
    }

    /// Return descriptive metadata in deterministic key order.
    #[must_use]
    pub const fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Validate invariants that do not require a complete graph.
    pub fn validate(&self) -> Result<(), HybridError> {
        let graph_id = self.id.graph();

        if self.dependency_id.graph() != graph_id {
            return Err(HybridError::invalid_reference(format!(
                "edge {} and dependency {} belong to different graphs",
                self.id, self.dependency_id
            )));
        }

        if self.source.graph() != graph_id || self.target.graph() != graph_id {
            return Err(HybridError::invalid_reference(format!(
                "edge {} endpoints must belong to its graph {graph_id}",
                self.id
            )));
        }

        self.kind
            .validate(graph_id)
            .map_err(|error| error.in_component("hybrid graph edge"))?;

        validate_metadata(&self.metadata)
            .map_err(|error| error.in_component("hybrid graph edge"))?;

        Ok(())
    }

    fn estimated_metadata_bytes(&self) -> Result<u128, HybridError> {
        let mut total = estimated_map_bytes(&self.metadata)?;
        total = checked_add(total, self.kind.estimated_metadata_bytes()?)?;
        Ok(total)
    }
}

/// A collection of hybrid edges belonging to one graph.
///
/// The current `HybridGraph` representation stores nodes and domain
/// composition but does not embed edges. This collection therefore records
/// the graph identity explicitly and is validated against the graph before
/// compiler, planner, or runtime use.
///
/// Construction validates edge-local invariants and duplicate identities.
/// `validate_against` additionally validates endpoint existence and
/// domain-boundary claims.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "HybridEdgeSetWire")]
pub struct HybridEdgeSet {
    graph_id: HybridGraphId,
    edges: Vec<HybridEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct HybridEdgeSetWire {
    graph_id: HybridGraphId,
    #[serde(default)]
    edges: Vec<HybridEdge>,
}

impl TryFrom<HybridEdgeSetWire> for HybridEdgeSet {
    type Error = HybridError;

    fn try_from(value: HybridEdgeSetWire) -> Result<Self, Self::Error> {
        Self::new(value.graph_id, value.edges)
    }
}

impl HybridEdgeSet {
    /// Construct an edge set and validate graph-scoped identities.
    ///
    /// This method does not prove that endpoint nodes exist. Call
    /// `validate_against` before treating the set as a valid graph edge set.
    pub fn new(
        graph_id: HybridGraphId,
        edges: Vec<HybridEdge>,
    ) -> Result<Self, HybridError> {
        let value = Self { graph_id, edges };
        value.validate_local()?;
        Ok(value)
    }

    /// Return the graph identity to which these edges belong.
    #[must_use]
    pub const fn graph_id(&self) -> HybridGraphId {
        self.graph_id
    }

    /// Return edges in declaration order.
    #[must_use]
    pub fn edges(&self) -> &[HybridEdge] {
        &self.edges
    }

    /// Return the number of edges.
    #[must_use]
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Return whether the edge set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Look up an edge by ID.
    ///
    /// This is a linear lookup. Repeated graph-scale lookups should use an
    /// index owned by the builder or analysis subsystem, not a second
    /// authoritative edge collection.
    #[must_use]
    pub fn get(&self, id: HybridEdgeId) -> Option<&HybridEdge> {
        self.edges.iter().find(|edge| edge.id() == id)
    }

    /// Return edges sorted by edge ID without changing declaration order.
    #[must_use]
    pub fn edges_canonical_order(&self) -> Vec<&HybridEdge> {
        let mut result: Vec<&HybridEdge> = self.edges.iter().collect();
        result.sort_by_key(|edge| edge.id());
        result
    }

    /// Return outgoing edges from a node in declaration order.
    ///
    /// This convenience method is linear in the number of edges. Graph
    /// analysis should build a reusable adjacency index for repeated queries.
    #[must_use]
    pub fn outgoing(&self, node: HybridNodeId) -> impl Iterator<Item = &HybridEdge> {
        self.edges.iter().filter(move |edge| edge.source() == node)
    }

    /// Return incoming edges to a node in declaration order.
    #[must_use]
    pub fn incoming(&self, node: HybridNodeId) -> impl Iterator<Item = &HybridEdge> {
        self.edges.iter().filter(move |edge| edge.target() == node)
    }

    /// Validate edge-set-local invariants.
    pub fn validate_local(&self) -> Result<(), HybridError> {
        let mut edge_ids = BTreeSet::new();
        let mut dependency_ids = BTreeSet::new();

        for edge in &self.edges {
            edge.validate()
                .map_err(|error| error.in_component("hybrid edge set"))?;

            if edge.id().graph() != self.graph_id {
                return Err(HybridError::invalid_reference(format!(
                    "edge {} does not belong to edge-set graph {}",
                    edge.id(),
                    self.graph_id
                )));
            }

            if !edge_ids.insert(edge.id()) {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!("edge identity {} occurs more than once", edge.id()),
                ));
            }

            if !dependency_ids.insert(edge.dependency_id()) {
                return Err(HybridError::new(
                    HybridErrorCode::DuplicateIdentity,
                    format!(
                        "dependency identity {} occurs more than once in this edge set",
                        edge.dependency_id()
                    ),
                ));
            }
        }

        Ok(())
    }

    /// Validate edge references against a complete hybrid graph.
    ///
    /// Required unknown extension semantics fail closed. Unknown optional
    /// extensions are reported in the returned report rather than silently
    /// treated as understood relationships.
    pub fn validate_against(
        &self,
        graph: &HybridGraph,
    ) -> Result<EdgeValidationReport, HybridError> {
        self.validate_local()?;
        graph.validate()?;

        if self.graph_id != graph.id() {
            return Err(HybridError::invalid_reference(format!(
                "edge set belongs to graph {}, but supplied graph is {}",
                self.graph_id,
                graph.id()
            )));
        }

        let mut report = EdgeValidationReport::default();

        for edge in &self.edges {
            let source = graph.node(edge.source()).ok_or_else(|| {
                HybridError::invalid_reference(format!(
                    "edge {} references missing source node {}",
                    edge.id(),
                    edge.source()
                ))
            })?;

            let target = graph.node(edge.target()).ok_or_else(|| {
                HybridError::invalid_reference(format!(
                    "edge {} references missing target node {}",
                    edge.id(),
                    edge.target()
                ))
            })?;

            match edge.kind() {
                EdgeKind::DomainBoundary => {
                    let source_instance = graph
                        .composition()
                        .get(source.domain_instance())
                        .ok_or_else(|| {
                            HybridError::invalid_reference(format!(
                                "edge {} source node refers to missing domain instance {}",
                                edge.id(),
                                source.domain_instance()
                            ))
                        })?;

                    let target_instance = graph
                        .composition()
                        .get(target.domain_instance())
                        .ok_or_else(|| {
                            HybridError::invalid_reference(format!(
                                "edge {} target node refers to missing domain instance {}",
                                edge.id(),
                                target.domain_instance()
                            ))
                        })?;

                    let source_domain = source_instance
                        .domain_id()
                        .map_err(|error| error.in_component("hybrid edge validation"))?;

                    let target_domain = target_instance
                        .domain_id()
                        .map_err(|error| error.in_component("hybrid edge validation"))?;

                    if source_domain == target_domain {
                        return Err(HybridError::invalid_dependency(format!(
                            "domain-boundary edge {} connects nodes in the same semantic domain",
                            edge.id()
                        )));
                    }

                    report.domain_boundary_edges =
                        report.domain_boundary_edges.saturating_add(1);
                }
                EdgeKind::Extension(extension) if extension.is_required() => {
                    return Err(HybridError::new(
                        HybridErrorCode::UnsupportedOperation,
                        format!(
                            "edge {} requires unsupported extension semantics `{extension}`",
                            edge.id()
                        ),
                    )
                    .at_entity(
                        "hybrid edge validation",
                        edge.id().to_string(),
                        "register an implementation that understands this extension before compilation",
                    ));
                }
                EdgeKind::Extension(_) => {
                    report.unresolved_optional_extensions.push(edge.id());
                }
                _ => {}
            }

            report.validated_edges = report.validated_edges.saturating_add(1);
        }

        Ok(report)
    }

    /// Validate the edge set against a graph and explicit operational budgets.
    ///
    /// Budget exhaustion is an operational failure, not proof that the
    /// represented Zamani program is semantically invalid.
    pub fn validate_against_with_config(
        &self,
        graph: &HybridGraph,
        config: &HybridConfig,
    ) -> Result<EdgeValidationReport, HybridError> {
        self.validate_against(graph)?;

        config
            .check_budget(BudgetKind::GraphEdges, self.edges.len() as u128)
            .map_err(|error| error.in_component("hybrid edge set"))?;

        let mut metadata_bytes = 0_u128;

        for edge in &self.edges {
            metadata_bytes = checked_add(
                metadata_bytes,
                edge.estimated_metadata_bytes()?,
            )?;
        }

        config
            .check_budget(BudgetKind::MetadataBytes, metadata_bytes)
            .map_err(|error| error.in_component("hybrid edge set"))?;

        Ok(self.validate_against(graph)?)
    }
}

/// Results of structural edge validation.
///
/// An unresolved optional extension is retained and reported. Consumers must
/// decide explicitly whether their operation can proceed with such extensions;
/// the report is not proof of complete semantic verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EdgeValidationReport {
    /// Number of edges structurally validated.
    pub validated_edges: u128,

    /// Number of verified domain-boundary edges.
    pub domain_boundary_edges: u128,

    /// IDs of optional extension edges whose semantics were not resolved.
    pub unresolved_optional_extensions: Vec<HybridEdgeId>,
}

impl EdgeValidationReport {
    /// Return whether every edge's kind is understood by this implementation.
    #[must_use]
    pub fn has_unresolved_required_semantics(&self) -> bool {
        // Required unknown semantics are rejected during validation.
        false
    }

    /// Return whether all extension semantics were resolved.
    #[must_use]
    pub fn is_fully_semantically_understood(&self) -> bool {
        self.unresolved_optional_extensions.is_empty()
    }
}

fn validate_metadata(
    metadata: &BTreeMap<String, String>,
) -> Result<(), HybridError> {
    for (key, value) in metadata {
        validate_identifier("edge metadata key", key)?;

        if value.chars().any(char::is_control) {
            return Err(HybridError::invalid_representation(
                "edge metadata values must not contain control characters",
            ));
        }
    }

    Ok(())
}

fn validate_identifier(
    field: &str,
    value: &str,
) -> Result<(), HybridError> {
    if value.trim().is_empty() {
        return Err(HybridError::invalid_representation(format!(
            "{field} must not be empty"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(HybridError::invalid_representation(format!(
            "{field} must not contain control characters"
        )));
    }

    Ok(())
}

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

fn checked_add(left: u128, right: u128) -> Result<u128, HybridError> {
    left.checked_add(right).ok_or_else(|| {
        HybridError::new(
            HybridErrorCode::InvalidRepresentation,
            "hybrid edge metadata accounting overflowed its numeric representation",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hybrid::ids::{GraphScopedId, IdComponent};

    fn graph_id(value: u128) -> HybridGraphId {
        HybridGraphId::new(value).expect("non-zero graph ID")
    }

    fn node_id(graph: HybridGraphId, value: u128) -> HybridNodeId {
        HybridNodeId::new(GraphScopedId::new(
            graph,
            IdComponent::new(value).expect("non-zero local ID"),
        ))
    }

    fn edge_id(graph: HybridGraphId, value: u128) -> HybridEdgeId {
        HybridEdgeId::new(GraphScopedId::new(
            graph,
            IdComponent::new(value).expect("non-zero local ID"),
        ))
    }

    fn dependency_id(graph: HybridGraphId, value: u128) -> HybridDependencyId {
        HybridDependencyId::new(GraphScopedId::new(
            graph,
            IdComponent::new(value).expect("non-zero local ID"),
        ))
    }

    #[test]
    fn rejects_endpoints_from_another_graph() {
        let graph_a = graph_id(1);
        let graph_b = graph_id(2);

        let result = HybridEdge::new(
            edge_id(graph_a, 1),
            dependency_id(graph_a, 2),
            node_id(graph_a, 3),
            node_id(graph_b, 4),
            EdgeKind::Control,
            BTreeMap::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_dependency_identity_from_another_graph() {
        let graph = graph_id(1);
        let other_graph = graph_id(2);

        let result = HybridEdge::new(
            edge_id(graph, 1),
            dependency_id(other_graph, 2),
            node_id(graph, 3),
            node_id(graph, 4),
            EdgeKind::Ordering,
            BTreeMap::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn extension_kind_requires_a_nonempty_namespace_and_name() {
        assert!(ExtensionEdgeKind::new("", "custom", None, false).is_err());
        assert!(ExtensionEdgeKind::new("example.org", "", None, false).is_err());
        assert!(ExtensionEdgeKind::new("example.org", "feedback", None, false).is_ok());
    }

    #[test]
    fn edge_kind_keeps_extension_identity_namespaced() {
        let extension = ExtensionEdgeKind::new(
            "example.org",
            "stream_dependency",
            Some("1.0".to_owned()),
            false,
        )
        .expect("valid extension");

        assert_eq!(extension.namespace(), "example.org");
        assert_eq!(extension.name(), "stream_dependency");
        assert_eq!(extension.version(), Some("1.0"));
        assert!(!extension.is_required());
    }

    #[test]
    fn self_edges_are_not_rejected_by_local_validation() {
        let graph = graph_id(1);
        let node = node_id(graph, 2);

        let edge = HybridEdge::new(
            edge_id(graph, 3),
            dependency_id(graph, 4),
            node,
            node,
            EdgeKind::Feedback,
            BTreeMap::new(),
        );

        assert!(edge.is_ok());
    }

    #[test]
    fn metadata_keys_must_be_valid_identifiers() {
        let graph = graph_id(1);
        let mut metadata = BTreeMap::new();
        metadata.insert("".to_owned(), "description".to_owned());

        let result = HybridEdge::new(
            edge_id(graph, 2),
            dependency_id(graph, 3),
            node_id(graph, 4),
            node_id(graph, 5),
            EdgeKind::Data { value: None },
            metadata,
        );

        assert!(result.is_err());
    }

    #[test]
    fn edge_kind_builtin_names_are_stable() {
        assert_eq!(
            EdgeKind::Data { value: None }.builtin_name(),
            Some("data")
        );
        assert_eq!(EdgeKind::Control.builtin_name(), Some("control"));
        assert_eq!(
            EdgeKind::Completion { event: None }.builtin_name(),
            Some("completion")
        );
        assert_eq!(EdgeKind::Ordering.builtin_name(), Some("ordering"));
        assert_eq!(EdgeKind::Feedback.builtin_name(), Some("feedback"));
        assert_eq!(
            EdgeKind::DomainBoundary.builtin_name(),
            Some("domain_boundary")
        );
    }
}
