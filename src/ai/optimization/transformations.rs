
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Optimization — Graph Transformations
//!
//! Path: src/ai/optimization/transformations.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides target-independent, transactional transformation contracts
//! for canonical AI computational graphs.
//!
//! This module does not introduce another graph representation.
//!
//! Instead, it operates through graph interfaces implemented by adapters
//! over the repository's canonical model, pipeline, and IR structures.
//!
//! ## Ownership
//!
//! This module owns:
//! - transformation identities;
//! - transformation classifications;
//! - transformation policies;
//! - transformation preconditions;
//! - transformation plans;
//! - transformation edit descriptions;
//! - transformation reporting;
//! - transactional transformation orchestration;
//! - explicitly semantics-preserving generic graph rewrites;
//! - deterministic transformation selection;
//! - transformation failure reporting.
//!
//! This module does not own:
//! - canonical model graphs;
//! - canonical pipeline graphs;
//! - universal IR;
//! - tensor storage;
//! - semantic validation;
//! - transformation legality authority;
//! - numerical precision policy;
//! - quantization policy;
//! - sparsity policy;
//! - cost models;
//! - hardware discovery;
//! - resource allocation;
//! - physical placement;
//! - scheduling;
//! - distributed execution;
//! - runtime execution.
//!
//! ## Canonical integration
//!
//! Model graph:
//!     crate::ai::model::graph
//!
//! Pipeline graph:
//!     crate::ai::pipeline::graph
//!
//! Graph analysis:
//!     crate::ai::optimization::graph
//!
//! Semantic authority:
//!     crate::frontend::semantic::domains::ai
//!
//! Universal computational representation:
//!     crate::zuir
//!
//! Legality:
//!     crate::ai::optimization::legality
//!
//! Cost estimation:
//!     crate::ai::optimization::cost_model
//!
//! ## Scalability
//!
//! No universal maximum is imposed on:
//! - graph nodes;
//! - graph edges;
//! - graph depth;
//! - transformation count;
//! - transformation passes;
//! - model parameters;
//! - tensor dimensions;
//! - tensor rank;
//! - pipeline stages;
//! - distributed participants;
//! - accelerators;
//! - memory capacity.
//!
//! Actual capacity depends on available resources, addressability,
//! allocator capacity, and explicit operational policies.
//!
//! ## Critical correctness rule
//!
//! A structurally possible rewrite is not necessarily semantically legal.
//!
//! Graph adapters must provide explicit transformation facts and must
//! independently validate the semantic legality of every proposed rewrite.
//!
//! No transformation is allowed to silently discard:
//! - observable effects;
//! - numerical requirements;
//! - ordering requirements;
//! - state dependencies;
//! - provenance;
//! - security obligations;
//! - reproducibility requirements;
//! - resource requirements.
//!
//! ## Transactional behavior
//!
//! Transformations are planned before they are committed.
//!
//! A graph implementation must either apply an entire accepted plan
//! atomically or leave the original graph unchanged.
//!
//! A partially applied transformation is never reported as successful.
//!
//! ## POCO-REAF
//!
//! All transformations describe logical computation.
//!
//! No transformation selects a CPU, GPU, FPGA, ASIC, QPU, or other
//! physical target.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

// ============================================================================
// TRANSFORMATION IDENTITY
// ============================================================================

/// Stable identity of an AI transformation.
///
/// The identity is independent of implementation provider and hardware.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TransformationId(Arc<str>);

impl TransformationId {
    /// Creates a transformation identity.
    pub fn new(
        value: impl AsRef<str>,
    ) -> Result<Self, TransformationError> {
        let value = value.as_ref();

        validate_identity(value)?;

        Ok(Self(Arc::from(value)))
    }

    /// Returns the identity string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TransformationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

// ============================================================================
// VERSION
// ============================================================================

/// Version of a transformation contract.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub struct TransformationVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl TransformationVersion {
    #[must_use]
    pub const fn new(
        major: u64,
        minor: u64,
        patch: u64,
    ) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    #[must_use]
    pub const fn major(self) -> u64 {
        self.major
    }

    #[must_use]
    pub const fn minor(self) -> u64 {
        self.minor
    }

    #[must_use]
    pub const fn patch(self) -> u64 {
        self.patch
    }
}

// ============================================================================
// CLASSIFICATION
// ============================================================================

/// Classification of an AI transformation.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[non_exhaustive]
pub enum TransformationKind {
    /// Changes graph structure without intentionally changing semantics.
    Structural,

    /// Combines compatible computational operations.
    Composition,

    /// Eliminates explicitly redundant computation.
    RedundancyElimination,

    /// Rewrites equivalent operation structures.
    Canonicalization,

    /// Changes data-flow relationships.
    DataFlow,

    /// Changes control-flow relationships.
    ControlFlow,

    /// Changes representation while preserving declared semantics.
    Representation,

    /// Extension-defined transformation.
    Extension,
}

// ============================================================================
// TRANSFORMATION PHASE
// ============================================================================

/// Logical phase in which a transformation may be considered.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
#[non_exhaustive]
pub enum TransformationPhase {
    Initial,
    GraphSimplification,
    Canonicalization,
    Composition,
    DataFlow,
    Numerical,
    Representation,
    Finalization,
    Extension,
}

// ============================================================================
// SEMANTIC PRESERVATION
// ============================================================================

/// Semantic preservation requirements for a transformation.
///
/// These are declarations, not proof of equivalence.
///
/// Actual legality must be checked by the canonical semantic and
/// optimization legality infrastructure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreservationRequirements {
    /// Preserve observable behavior.
    pub observable_behavior: bool,

    /// Preserve required numerical semantics.
    pub numerical_semantics: bool,

    /// Preserve effect ordering.
    pub effect_ordering: bool,

    /// Preserve state dependencies.
    pub state_dependencies: bool,

    /// Preserve provenance.
    pub provenance: bool,

    /// Preserve security obligations.
    pub security: bool,

    /// Preserve determinism requirements.
    pub determinism: bool,

    /// Preserve resource requirements.
    pub resource_requirements: bool,
}

impl Default for PreservationRequirements {
    fn default() -> Self {
        Self {
            observable_behavior: true,
            numerical_semantics: true,
            effect_ordering: true,
            state_dependencies: true,
            provenance: true,
            security: true,
            determinism: true,
            resource_requirements: true,
        }
    }
}

// ============================================================================
// TRANSFORMATION POLICY
// ============================================================================

/// Controls transformation behavior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformationPolicy {
    /// Whether the transformation must preserve semantics.
    pub preservation: PreservationRequirements,

    /// Whether transformations must be deterministic.
    pub deterministic: bool,

    /// Whether an independent legality check is mandatory.
    pub require_legality_validation: bool,

    /// Whether the transformation must provide a complete plan.
    pub require_complete_plan: bool,

    /// Whether a failed transformation must leave the graph unchanged.
    pub require_atomic_commit: bool,
}

impl Default for TransformationPolicy {
    fn default() -> Self {
        Self {
            preservation: PreservationRequirements::default(),
            deterministic: true,
            require_legality_validation: true,
            require_complete_plan: true,
            require_atomic_commit: true,
        }
    }
}

// ============================================================================
// GRAPH REVISION
// ============================================================================

/// Opaque graph revision used to prevent stale transformation commits.
///
/// The revision is supplied by the canonical graph adapter.
///
/// It is not a hardware or execution identifier.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub struct GraphRevision(u128);

impl GraphRevision {
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }
}

// ============================================================================
// GRAPH INTERFACE
// ============================================================================

/// Read-only interface to an existing canonical graph.
///
/// Implementations must expose canonical identities and relationships.
///
/// This trait does not create or own a second graph.
///
/// The returned collections must be deterministic.
pub trait TransformationGraph {
    /// Canonical node identity.
    type Node: Clone + Ord + fmt::Debug;

    /// Canonical edge representation.
    type Edge: Clone + Ord + fmt::Debug;

    /// Returns the current graph revision.
    fn revision(&self) -> GraphRevision;

    /// Returns canonical node identities.
    fn nodes(&self) -> Vec<Self::Node>;

    /// Returns canonical edges.
    fn edges(&self) -> Vec<Self::Edge>;

    /// Returns the source node of an edge.
    fn source(&self, edge: &Self::Edge) -> Self::Node;

    /// Returns the destination node of an edge.
    fn destination(&self, edge: &Self::Edge) -> Self::Node;

    /// Returns whether the node is explicitly declared transparent.
    ///
    /// Transparency means that removing the node is permitted by its
    /// semantic contract, subject to independent legality validation.
    ///
    /// An adapter must return false when this fact is unknown.
    fn is_transparent(&self, node: &Self::Node) -> bool;

    /// Returns whether the edge can be composed through the node.
    ///
    /// The implementation must preserve all edge semantics, including
    /// port identity, data-flow meaning, control dependencies and
    /// relevant metadata.
    ///
    /// Returning None means that composition is not supported.
    fn compose_edges(
        &self,
        incoming: &Self::Edge,
        outgoing: &Self::Edge,
        removed_node: &Self::Node,
    ) -> Option<Self::Edge>;
}

// ============================================================================
// GRAPH EDITS
// ============================================================================

/// An individual graph edit.
///
/// Edits are declarative. They are not applied directly by this module.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum GraphEdit<N, E> {
    /// Remove an existing edge.
    RemoveEdge(E),

    /// Add a validated edge.
    AddEdge(E),

    /// Remove an existing node.
    RemoveNode(N),
}

// ============================================================================
// TRANSFORMATION PLAN
// ============================================================================

/// A complete proposed graph transformation.
///
/// A plan is tied to the graph revision from which it was produced.
///
/// It must be validated before being committed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformationPlan<N, E> {
    transformation: TransformationId,
    version: TransformationVersion,
    source_revision: GraphRevision,
    phase: TransformationPhase,
    edits: Vec<GraphEdit<N, E>>,
    affected_nodes: BTreeSet<N>,
    description: String,
}

impl<N, E> TransformationPlan<N, E> {
    /// Returns the transformation identity.
    #[must_use]
    pub fn transformation(&self) -> &TransformationId {
        &self.transformation
    }

    /// Returns the transformation version.
    #[must_use]
    pub const fn version(&self) -> TransformationVersion {
        self.version
    }

    /// Returns the graph revision used during planning.
    #[must_use]
    pub const fn source_revision(&self) -> GraphRevision {
        self.source_revision
    }

    /// Returns the transformation phase.
    #[must_use]
    pub const fn phase(&self) -> TransformationPhase {
        self.phase
    }

    /// Returns proposed edits.
    #[must_use]
    pub fn edits(&self) -> &[GraphEdit<N, E>] {
        &self.edits
    }

    /// Returns affected nodes.
    #[must_use]
    pub fn affected_nodes(&self) -> &BTreeSet<N> {
        &self.affected_nodes
    }

    /// Returns the plan description.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns whether the plan contains edits.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }

    /// Returns the number of proposed edits.
    #[must_use]
    pub fn edit_count(&self) -> usize {
        self.edits.len()
    }
}

// ============================================================================
// PLAN BUILDER
// ============================================================================

/// Constructs transformation plans without modifying a graph.
pub struct TransformationPlanBuilder<N, E> {
    transformation: TransformationId,
    version: TransformationVersion,
    source_revision: GraphRevision,
    phase: TransformationPhase,
    edits: Vec<GraphEdit<N, E>>,
    affected_nodes: BTreeSet<N>,
    description: String,
}

impl<N, E> TransformationPlanBuilder<N, E>
where
    N: Ord,
{
    /// Creates a plan builder.
    pub fn new(
        transformation: TransformationId,
        version: TransformationVersion,
        source_revision: GraphRevision,
        phase: TransformationPhase,
    ) -> Self {
        Self {
            transformation,
            version,
            source_revision,
            phase,
            edits: Vec::new(),
            affected_nodes: BTreeSet::new(),
            description: String::new(),
        }
    }

    /// Sets a human-readable explanation.
    #[must_use]
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = description.into();
        self
    }

    /// Adds an edit.
    pub fn push(
        &mut self,
        edit: GraphEdit<N, E>,
    ) -> Result<(), TransformationError> {
        self.edits
            .try_reserve(1)
            .map_err(|_| TransformationError::AllocationFailed)?;

        self.edits.push(edit);
        Ok(())
    }

    /// Marks a node as affected.
    pub fn affect(
        &mut self,
        node: N,
    ) -> Result<(), TransformationError> {
        self.affected_nodes
            .try_reserve(1)
            .map_err(|_| TransformationError::AllocationFailed)?;

        self.affected_nodes.insert(node);
        Ok(())
    }

    /// Builds the immutable plan.
    pub fn build(self) -> Result<TransformationPlan<N, E>, TransformationError> {
        if self.description.trim().is_empty() {
            return Err(TransformationError::MissingDescription);
        }

        Ok(TransformationPlan {
            transformation: self.transformation,
            version: self.version,
            source_revision: self.source_revision,
            phase: self.phase,
            edits: self.edits,
            affected_nodes: self.affected_nodes,
            description: self.description,
        })
    }
}

// ============================================================================
// TRANSFORMATION TRAIT
// ============================================================================

/// A target-independent graph transformation.
pub trait Transformation<G: TransformationGraph> {
    /// Returns the transformation identity.
    fn id(&self) -> &TransformationId;

    /// Returns the transformation version.
    fn version(&self) -> TransformationVersion;

    /// Returns its classification.
    fn kind(&self) -> TransformationKind;

    /// Returns its phase.
    fn phase(&self) -> TransformationPhase;

    /// Returns its preservation policy.
    fn policy(&self) -> &TransformationPolicy;

    /// Produces a plan without modifying the graph.
    fn plan(
        &self,
        graph: &G,
    ) -> Result<
        Vec<TransformationPlan<G::Node, G::Edge>>,
        TransformationError,
    >;
}

// ============================================================================
// LEGALITY VALIDATION
// ============================================================================

/// Result of independent semantic legality validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegalityResult {
    /// Whether the proposed transformation is legal.
    pub legal: bool,

    /// Whether the validation completed.
    pub complete: bool,

    /// Diagnostic messages.
    pub diagnostics: Vec<String>,
}

impl LegalityResult {
    /// Creates a successful complete validation.
    #[must_use]
    pub fn valid() -> Self {
        Self {
            legal: true,
            complete: true,
            diagnostics: Vec::new(),
        }
    }

    /// Creates an incomplete validation.
    #[must_use]
    pub fn incomplete(message: impl Into<String>) -> Self {
        Self {
            legal: false,
            complete: false,
            diagnostics: vec![message.into()],
        }
    }

    /// Creates a rejected validation.
    #[must_use]
    pub fn rejected(message: impl Into<String>) -> Self {
        Self {
            legal: false,
            complete: true,
            diagnostics: vec![message.into()],
        }
    }

    /// Returns whether the result certifies legality.
    #[must_use]
    pub fn is_certified_legal(&self) -> bool {
        self.legal && self.complete
    }
}

/// Independent legality service.
///
/// The canonical semantic and optimization legality infrastructure
/// must implement this interface.
///
/// This module does not substitute structural checks for semantic proof.
pub trait TransformationLegality<G: TransformationGraph> {
    fn validate(
        &self,
        graph: &G,
        plan: &TransformationPlan<G::Node, G::Edge>,
        requirements: &PreservationRequirements,
    ) -> LegalityResult;
}

// ============================================================================
// TRANSACTIONAL GRAPH MUTATION
// ============================================================================

/// Result of an atomic graph update.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitResult {
    /// Revision after a successful commit.
    pub revision: GraphRevision,

    /// Number of applied edits.
    pub applied_edits: usize,
}

/// Mutable integration boundary for canonical graphs.
///
/// Implementations must guarantee:
///
/// - stale revisions are rejected;
/// - every edit is validated before mutation;
/// - all edits are committed atomically;
/// - a failed commit leaves the graph unchanged;
/// - canonical graph invariants remain valid;
/// - revision changes only after a successful commit.
///
/// This trait deliberately leaves actual storage with the canonical
/// model, pipeline, or IR implementation.
pub trait TransactionalTransformationGraph: TransformationGraph {
    fn apply_atomically(
        &mut self,
        plan: &TransformationPlan<Self::Node, Self::Edge>,
    ) -> Result<CommitResult, TransformationError>;
}

// ============================================================================
// TRANSFORMATION REPORT
// ============================================================================

/// Outcome of considering a transformation plan.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TransformationOutcome {
    Planned,
    Rejected,
    Committed,
    Skipped,
}

/// Report for one transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformationReport {
    pub transformation: TransformationId,
    pub outcome: TransformationOutcome,
    pub edit_count: usize,
    pub diagnostics: Vec<String>,
    pub committed_revision: Option<GraphRevision>,
}

impl TransformationReport {
    fn planned(
        transformation: TransformationId,
        edit_count: usize,
    ) -> Self {
        Self {
            transformation,
            outcome: TransformationOutcome::Planned,
            edit_count,
            diagnostics: Vec::new(),
            committed_revision: None,
        }
    }
}

// ============================================================================
// TRANSFORMATION ENGINE
// ============================================================================

/// Coordinates transformation planning, legality and atomic commitment.
///
/// The engine does not execute transformed computation.
pub struct TransformationEngine;

impl TransformationEngine {
    /// Validates and commits one transformation plan.
    ///
    /// A stale plan is rejected.
    ///
    /// Incomplete legality validation is rejected.
    ///
    /// The canonical graph adapter owns the atomic commit.
    pub fn commit<G, L>(
        graph: &mut G,
        legality: &L,
        plan: &TransformationPlan<G::Node, G::Edge>,
        policy: &TransformationPolicy,
    ) -> Result<TransformationReport, TransformationError>
    where
        G: TransactionalTransformationGraph,
        L: TransformationLegality<G>,
    {
        if graph.revision() != plan.source_revision {
            return Err(TransformationError::StalePlan);
        }

        if policy.require_complete_plan && plan.is_empty() {
            return Err(TransformationError::EmptyPlan);
        }

        let result = if policy.require_legality_validation {
            legality.validate(
                graph,
                plan,
                &policy.preservation,
            )
        } else {
            LegalityResult::incomplete(
                "legality validation was not performed",
            )
        };

        if !result.is_certified_legal() {
            return Err(TransformationError::LegalityRejected {
                diagnostics: result.diagnostics,
            });
        }

        let committed = graph.apply_atomically(plan)?;

        if committed.applied_edits != plan.edit_count() {
            return Err(TransformationError::IncompleteCommit);
        }

        let mut report = TransformationReport::planned(
            plan.transformation.clone(),
            plan.edit_count(),
        );

        report.outcome = TransformationOutcome::Committed;
        report.committed_revision = Some(committed.revision);

        Ok(report)
    }
}

// ============================================================================
// BUILT-IN STRUCTURAL TRANSFORMATION
// ============================================================================

/// Eliminates an explicitly transparent intermediate graph node.
///
/// The transformation is considered only when:
///
/// - the node is explicitly transparent;
/// - it has exactly one incoming edge;
/// - it has exactly one outgoing edge;
/// - it is not involved in a self-loop;
/// - the graph adapter can compose the two edges without losing meaning.
///
/// This is a structural candidate, not a semantic proof.
///
/// The independent legality service must approve the resulting plan.
#[derive(Clone, Debug)]
pub struct BypassTransparentNode {
    id: TransformationId,
    version: TransformationVersion,
    policy: TransformationPolicy,
}

impl BypassTransparentNode {
    /// Creates the built-in transformation.
    pub fn new() -> Result<Self, TransformationError> {
        Ok(Self {
            id: TransformationId::new(
                "zamani.ai.optimization.bypass_transparent_node",
            )?,
            version: TransformationVersion::new(1, 0, 0),
            policy: TransformationPolicy::default(),
        })
    }

    /// Returns its default policy.
    #[must_use]
    pub fn policy(&self) -> &TransformationPolicy {
        &self.policy
    }

    /// Finds and plans transparent-node bypasses.
    ///
    /// Each candidate is planned independently against the same graph
    /// revision. Callers must re-analyze the graph after each commit.
    pub fn plan_candidates<G>(
        &self,
        graph: &G,
    ) -> Result<
        Vec<TransformationPlan<G::Node, G::Edge>>,
        TransformationError,
    >
    where
        G: TransformationGraph,
    {
        let nodes = graph.nodes();
        let edges = graph.edges();

        let declared: BTreeSet<G::Node> =
            nodes.iter().cloned().collect();

        if declared.len() != nodes.len() {
            return Err(TransformationError::InvalidGraph(
                "canonical graph contains duplicate node identities",
            ));
        }

        let mut incoming: BTreeMap<G::Node, Vec<G::Edge>> =
            BTreeMap::new();

        let mut outgoing: BTreeMap<G::Node, Vec<G::Edge>> =
            BTreeMap::new();

        for edge in &edges {
            let source = graph.source(edge);
            let destination = graph.destination(edge);

            if !declared.contains(&source)
                || !declared.contains(&destination)
            {
                return Err(TransformationError::InvalidGraph(
                    "edge references an undeclared node",
                ));
            }

            incoming
                .entry(destination)
                .or_default()
                .push(edge.clone());

            outgoing
                .entry(source)
                .or_default()
                .push(edge.clone());
        }

        let mut plans = Vec::new();

        for node in nodes {
            if !graph.is_transparent(&node) {
                continue;
            }

            let Some(in_edges) = incoming.get(&node) else {
                continue;
            };

            let Some(out_edges) = outgoing.get(&node) else {
                continue;
            };

            if in_edges.len() != 1 || out_edges.len() != 1 {
                continue;
            }

            let incoming_edge = &in_edges[0];
            let outgoing_edge = &out_edges[0];

            let source = graph.source(incoming_edge);
            let destination = graph.destination(outgoing_edge);

            if source == node || destination == node {
                continue;
            }

            // A self-loop or an edge that changes the graph into an
            // unsupported relationship must not be silently accepted.
            if source == destination {
                continue;
            }

            let Some(composed) = graph.compose_edges(
                incoming_edge,
                outgoing_edge,
                &node,
            ) else {
                continue;
            };

            if graph.source(&composed) != source
                || graph.destination(&composed) != destination
            {
                return Err(TransformationError::InvalidComposition);
            }

            let mut builder = TransformationPlanBuilder::new(
                self.id.clone(),
                self.version,
                graph.revision(),
                TransformationPhase::GraphSimplification,
            )
            .with_description(
                "Bypass an explicitly transparent intermediate node",
            );

            builder.push(GraphEdit::RemoveEdge(
                incoming_edge.clone(),
            ))?;

            builder.push(GraphEdit::RemoveEdge(
                outgoing_edge.clone(),
            ))?;

            builder.push(GraphEdit::AddEdge(composed))?;

            builder.push(GraphEdit::RemoveNode(node.clone()))?;

            builder.affect(source)?;
            builder.affect(destination)?;
            builder.affect(node)?;

            plans
                .try_reserve(1)
                .map_err(|_| TransformationError::AllocationFailed)?;

            plans.push(builder.build()?);
        }

        Ok(plans)
    }
}

impl<G> Transformation<G> for BypassTransparentNode
where
    G: TransformationGraph,
{
    fn id(&self) -> &TransformationId {
        &self.id
    }

    fn version(&self) -> TransformationVersion {
        self.version
    }

    fn kind(&self) -> TransformationKind {
        TransformationKind::Structural
    }

    fn phase(&self) -> TransformationPhase {
        TransformationPhase::GraphSimplification
    }

    fn policy(&self) -> &TransformationPolicy {
        &self.policy
    }

    fn plan(
        &self,
        graph: &G,
    ) -> Result<
        Vec<TransformationPlan<G::Node, G::Edge>>,
        TransformationError,
    > {
        self.plan_candidates(graph)
    }
}

// ============================================================================
// TRANSFORMATION ERRORS
// ============================================================================

/// Errors encountered during transformation processing.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TransformationError {
    InvalidIdentity,

    MissingDescription,

    AllocationFailed,

    EmptyPlan,

    StalePlan,

    InvalidComposition,

    IncompleteCommit,

    InvalidGraph(&'static str),

    LegalityRejected {
        diagnostics: Vec<String>,
    },

    GraphMutationFailed {
        reason: String,
    },

    ExtensionFailure {
        transformation: String,
        reason: String,
    },
}

impl fmt::Display for TransformationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentity => {
                f.write_str("invalid transformation identity")
            }

            Self::MissingDescription => {
                f.write_str("transformation description is required")
            }

            Self::AllocationFailed => {
                f.write_str("transformation allocation failed")
            }

            Self::EmptyPlan => {
                f.write_str("transformation plan contains no edits")
            }

            Self::StalePlan => {
                f.write_str("transformation plan references a stale graph revision")
            }

            Self::InvalidComposition => {
                f.write_str("composed edge does not preserve its endpoints")
            }

            Self::IncompleteCommit => {
                f.write_str("graph adapter reported an incomplete commit")
            }

            Self::InvalidGraph(reason) => {
                write!(f, "invalid transformation graph: {reason}")
            }

            Self::LegalityRejected { diagnostics } => {
                write!(
                    f,
                    "transformation legality validation failed: {}",
                    diagnostics.join("; ")
                )
            }

            Self::GraphMutationFailed { reason } => {
                write!(f, "graph mutation failed: {reason}")
            }

            Self::ExtensionFailure {
                transformation,
                reason,
            } => {
                write!(
                    f,
                    "transformation {transformation} failed: {reason}"
                )
            }
        }
    }
}

impl Error for TransformationError {}

// ============================================================================
// INTERNAL VALIDATION
// ============================================================================

fn validate_identity(
    value: &str,
) -> Result<(), TransformationError> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(TransformationError::InvalidIdentity);
    }

    Ok(())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Node(&'static str);

    #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Edge {
        source: Node,
        destination: Node,
    }

    struct TestGraph {
        revision: GraphRevision,
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        transparent: BTreeSet<Node>,
    }

    impl TransformationGraph for TestGraph {
        type Node = Node;
        type Edge = Edge;

        fn revision(&self) -> GraphRevision {
            self.revision
        }

        fn nodes(&self) -> Vec<Node> {
            self.nodes.clone()
        }

        fn edges(&self) -> Vec<Edge> {
            self.edges.clone()
        }

        fn source(&self, edge: &Edge) -> Node {
            edge.source.clone()
        }

        fn destination(&self, edge: &Edge) -> Node {
            edge.destination.clone()
        }

        fn is_transparent(&self, node: &Node) -> bool {
            self.transparent.contains(node)
        }

        fn compose_edges(
            &self,
            incoming: &Edge,
            outgoing: &Edge,
            removed: &Node,
        ) -> Option<Edge> {
            if incoming.destination != *removed
                || outgoing.source != *removed
            {
                return None;
            }

            Some(Edge {
                source: incoming.source.clone(),
                destination: outgoing.destination.clone(),
            })
        }
    }

    struct TestLegality;

    impl TransformationLegality<TestGraph> for TestLegality {
        fn validate(
            &self,
            _: &TestGraph,
            _: &TransformationPlan<Node, Edge>,
            _: &PreservationRequirements,
        ) -> LegalityResult {
            LegalityResult::valid()
        }
    }

    fn node(value: &'static str) -> Node {
        Node(value)
    }

    fn edge(
        source: &'static str,
        destination: &'static str,
    ) -> Edge {
        Edge {
            source: node(source),
            destination: node(destination),
        }
    }

    fn graph() -> TestGraph {
        let mut transparent = BTreeSet::new();
        transparent.insert(node("middle"));

        TestGraph {
            revision: GraphRevision::new(1),
            nodes: vec![
                node("input"),
                node("middle"),
                node("output"),
            ],
            edges: vec![
                edge("input", "middle"),
                edge("middle", "output"),
            ],
            transparent,
        }
    }

    #[test]
    fn transformation_identity_rejects_invalid_text() {
        assert!(TransformationId::new("").is_err());
        assert!(TransformationId::new("valid").is_ok());
        assert!(TransformationId::new("bad\nidentity").is_err());
    }

    #[test]
    fn preservation_requirements_default_to_strict() {
        let requirements = PreservationRequirements::default();

        assert!(requirements.observable_behavior);
        assert!(requirements.numerical_semantics);
        assert!(requirements.effect_ordering);
        assert!(requirements.state_dependencies);
        assert!(requirements.provenance);
        assert!(requirements.security);
        assert!(requirements.determinism);
        assert!(requirements.resource_requirements);
    }

    #[test]
    fn legality_requires_complete_validation() {
        assert!(LegalityResult::valid().is_certified_legal());

        assert!(
            !LegalityResult::incomplete("unknown")
                .is_certified_legal()
        );

        assert!(
            !LegalityResult::rejected("illegal")
                .is_certified_legal()
        );
    }

    #[test]
    fn plans_do_not_mutate_the_graph() {
        let graph = graph();

        let before = graph.edges.clone();

        let transformation =
            BypassTransparentNode::new().unwrap();

        let plans = transformation.plan(&graph).unwrap();

        assert_eq!(plans.len(), 1);
        assert_eq!(graph.edges, before);
    }

    #[test]
    fn bypass_produces_complete_edit_sequence() {
        let graph = graph();

        let transformation =
            BypassTransparentNode::new().unwrap();

        let plans = transformation.plan(&graph).unwrap();

        assert_eq!(plans.len(), 1);

        let plan = &plans[0];

        assert_eq!(plan.edit_count(), 4);

        assert!(plan.affected_nodes().contains(&node("input")));
        assert!(plan.affected_nodes().contains(&node("middle")));
        assert!(plan.affected_nodes().contains(&node("output")));

        assert_eq!(
            plan.source_revision(),
            GraphRevision::new(1)
        );
    }

    #[test]
    fn non_transparent_nodes_are_not_removed() {
        let mut graph = graph();

        graph.transparent.clear();

        let transformation =
            BypassTransparentNode::new().unwrap();

        let plans = transformation.plan(&graph).unwrap();

        assert!(plans.is_empty());
    }

    #[test]
    fn nodes_with_multiple_inputs_are_not_bypassed() {
        let mut graph = graph();

        graph.nodes.push(node("second_input"));

        graph.edges.push(edge("second_input", "middle"));

        let transformation =
            BypassTransparentNode::new().unwrap();

        let plans = transformation.plan(&graph).unwrap();

        assert!(plans.is_empty());
    }

    #[test]
    fn nodes_with_multiple_outputs_are_not_bypassed() {
        let mut graph = graph();

        graph.nodes.push(node("second_output"));

        graph.edges.push(edge("middle", "second_output"));

        let transformation =
            BypassTransparentNode::new().unwrap();

        let plans = transformation.plan(&graph).unwrap();

        assert!(plans.is_empty());
    }

    #[test]
    fn missing_composition_prevents_rewrite() {
        struct NonComposableGraph(TestGraph);

        impl TransformationGraph for NonComposableGraph {
            type Node = Node;
            type Edge = Edge;

            fn revision(&self) -> GraphRevision {
                self.0.revision
            }

            fn nodes(&self) -> Vec<Node> {
                self.0.nodes()
            }

            fn edges(&self) -> Vec<Edge> {
                self.0.edges()
            }

            fn source(&self, edge: &Edge) -> Node {
                self.0.source(edge)
            }

            fn destination(&self, edge: &Edge) -> Node {
                self.0.destination(edge)
            }

            fn is_transparent(&self, node: &Node) -> bool {
                self.0.is_transparent(node)
            }

            fn compose_edges(
                &self,
                _: &Edge,
                _: &Edge,
                _: &Node,
            ) -> Option<Edge> {
                None
            }
        }

        let graph = NonComposableGraph(graph());

        let transformation =
            BypassTransparentNode::new().unwrap();

        assert!(
            transformation
                .plan(&graph)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn graph_revisions_are_opaque_and_stable() {
        let revision = GraphRevision::new(42);

        assert_eq!(revision.value(), 42);
        assert_eq!(revision, GraphRevision::new(42));
    }
}
