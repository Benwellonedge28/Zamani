//! Zamani Quantum Error Correction — Union-Find Decoder.
//!
//! Deterministic, resource-bounded Union-Find style decoding over the
//! canonical [`super::decoding_graph::DecodingGraph`].
//!
//! Architectural boundary:
//!
//! ```text
//! Syndrome / Detection Events
//!            │
//!            ▼
//!     DecodingGraph
//!            │
//!            ▼
//!      Union-Find
//!       ├─ clusters
//!       ├─ parity
//!       ├─ boundary attachment
//!       ├─ deterministic growth
//!       └─ correction extraction
//!            │
//!            ▼
//!       DecodeResult
//!            │
//!            ▼
//!    Pauli Frame / Logical Layer
//! ```
//!
//! This module deliberately does NOT:
//!
//! - define another decoding graph;
//! - define another QEC resource policy;
//! - define another cancellation token;
//! - generate physical noise;
//! - extract syndromes;
//! - perform stabilizer algebra;
//! - modify a quantum state;
//! - classify logical equivalence.
//!
//! Those responsibilities belong to the corresponding QEC infrastructure
//! modules.
//!
//! Resource architecture:
//!
//! ```text
//! QecLimits
//!     │
//!     ├── graph preflight
//!     │
//!     └── decoder budget
//!             │
//!             ▼
//!       Union-Find execution
//!             │
//!             ├── CancellationToken
//!             └── ResourceManager
//! ```
//!
//! The implementation is deterministic:
//!
//! - graph iteration order is canonical;
//! - edge ordering is `(weight, endpoint ordering)`;
//! - union tie-breaking is deterministic;
//! - correction ordering is deterministic;
//! - no floating-point arithmetic is used by the decoder.

use std::collections::BTreeMap;
use std::fmt;
use std::time::Instant;

use super::cancellation::CancellationToken;
use super::decoding_graph::{
    BoundaryId,
    DecodingGraph,
    GraphEdge,
    GraphEndpoint,
    NodeId,
};
use super::errors::{
    DecoderKind,
    QecError,
    QecResult,
    ResourceKind as QecResourceKind,
};
use super::limits::QecLimits;
use super::resources::ResourceManager;

// ============================================================================
// Public compatibility aliases
// ============================================================================

/// Canonical Union-Find vertex identifier.
///
/// This is an alias for the canonical decoding-graph node identifier.
/// Union-Find no longer owns a separate vertex-ID type.
pub type VertexId = NodeId;

// ============================================================================
// Decoder limits
// ============================================================================

/// Decoder-specific operation budget.
///
/// These are algorithmic budgets, not a replacement for [`QecLimits`].
///
/// `QecLimits` remains authoritative for workload resources such as graph
/// nodes, graph edges, memory and decoder iterations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecoderBudget {
    /// Maximum graph-edge evaluations performed by the decoder.
    pub max_growth_operations: usize,

    /// Maximum cluster union operations.
    pub max_union_operations: usize,

    /// Maximum correction extraction operations.
    pub max_peel_operations: usize,
}

impl DecoderBudget {
    /// Creates an explicit decoder budget.
    #[must_use]
    pub const fn new(
        max_growth_operations: usize,
        max_union_operations: usize,
        max_peel_operations: usize,
    ) -> Self {
        Self {
            max_growth_operations,
            max_union_operations,
            max_peel_operations,
        }
    }

    /// Creates a budget derived from the configured QEC iteration limit.
    ///
    /// The multiplication is saturating so configuration itself cannot
    /// overflow.
    #[must_use]
    pub const fn from_limits(limits: QecLimits) -> Self {
        let iterations = limits.max_decoder_iterations;

        Self {
            max_growth_operations: iterations,
            max_union_operations: iterations,
            max_peel_operations: iterations,
        }
    }

    /// Conservative production budget.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_growth_operations: 20_000_000,
            max_union_operations: 4_000_000,
            max_peel_operations: 20_000_000,
        }
    }
}

impl Default for DecoderBudget {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Decoder configuration
// ============================================================================

/// Union-Find execution configuration.
///
/// `limits` is the central QEC resource policy. No independent graph-size
/// policy is maintained here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnionFindConfig {
    limits: QecLimits,
    budget: DecoderBudget,
}

impl UnionFindConfig {
    /// Creates a configuration.
    pub const fn new(
        limits: QecLimits,
        budget: DecoderBudget,
    ) -> Self {
        Self { limits, budget }
    }

    /// Creates a configuration using the central production limits.
    #[must_use]
    pub const fn production() -> Self {
        let limits = QecLimits::new();

        Self {
            limits,
            budget: DecoderBudget::from_limits(limits),
        }
    }

    /// Returns the authoritative QEC limits.
    #[must_use]
    pub const fn limits(self) -> QecLimits {
        self.limits
    }

    /// Returns the decoder operation budget.
    #[must_use]
    pub const fn budget(self) -> DecoderBudget {
        self.budget
    }

    /// Replaces the central QEC resource policy.
    #[must_use]
    pub const fn with_limits(
        self,
        limits: QecLimits,
    ) -> Self {
        Self {
            limits,
            budget: DecoderBudget::from_limits(limits),
        }
    }

    /// Replaces only the algorithmic decoder budget.
    #[must_use]
    pub const fn with_budget(
        self,
        budget: DecoderBudget,
    ) -> Self {
        Self { budget, ..self }
    }
}

impl Default for UnionFindConfig {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Correction
// ============================================================================

/// Correction produced by Union-Find.
///
/// The correction consists of canonical graph edges rather than a private
/// edge-ID namespace. This prevents the decoder from maintaining a duplicate
/// graph representation.
///
/// The edges are always returned in deterministic order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Correction {
    edges: Vec<GraphEdge>,
}

impl Correction {
    /// Creates an empty correction.
    #[must_use]
    pub const fn empty() -> Self {
        Self { edges: Vec::new() }
    }

    /// Creates a correction from canonical graph edges.
    fn new(edges: Vec<GraphEdge>) -> Self {
        Self { edges }
    }

    /// Returns selected correction edges.
    #[must_use]
    pub fn edges(&self) -> &[GraphEdge] {
        &self.edges
    }

    /// Returns the number of selected edges.
    #[must_use]
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Returns whether the correction is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Returns the correction as an iterator.
    pub fn iter(&self) -> impl Iterator<Item = &GraphEdge> {
        self.edges.iter()
    }
}

// ============================================================================
// Decode termination
// ============================================================================

/// Why Union-Find terminated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminationReason {
    /// All active detection-event parity was resolved.
    Completed,

    /// The graph contained no detection nodes.
    EmptyGraph,
}

// ============================================================================
// Decode statistics
// ============================================================================

/// Deterministic execution statistics produced by Union-Find.
///
/// These are decoder-local metrics. Higher-level metrics infrastructure may
/// additionally record wall time, memory and backend information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeStatistics {
    /// Number of graph edges considered.
    pub growth_operations: usize,

    /// Number of successful cluster unions.
    pub union_operations: usize,

    /// Number of correction edges emitted.
    pub peel_operations: usize,

    /// Number of final Union-Find clusters.
    pub clusters: usize,

    /// Number of graph detection nodes.
    pub detection_nodes: usize,

    /// Number of graph boundaries.
    pub boundary_nodes: usize,

    /// Number of graph edges.
    pub graph_edges: usize,

    /// Decoder wall-clock duration.
    ///
    /// This value is informational and is intentionally not part of
    /// deterministic equality semantics for the algorithmic result.
    pub elapsed_nanos: u64,
}

// ============================================================================
// Decode result
// ============================================================================

/// Complete Union-Find result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeResult {
    correction: Correction,
    statistics: DecodeStatistics,
    termination: TerminationReason,
}

impl DecodeResult {
    /// Returns the correction.
    #[must_use]
    pub fn correction(&self) -> &Correction {
        &self.correction
    }

    /// Returns decoder statistics.
    #[must_use]
    pub const fn statistics(&self) -> DecodeStatistics {
        self.statistics
    }

    /// Returns the termination reason.
    #[must_use]
    pub const fn termination(&self) -> TerminationReason {
        self.termination
    }

    /// Returns the number of clusters.
    #[must_use]
    pub const fn clusters(&self) -> usize {
        self.statistics.clusters
    }

    /// Returns the number of growth operations.
    #[must_use]
    pub const fn growth_operations(&self) -> usize {
        self.statistics.growth_operations
    }

    /// Returns the number of union operations.
    #[must_use]
    pub const fn union_operations(&self) -> usize {
        self.statistics.union_operations
    }

    /// Returns the number of peeling operations.
    #[must_use]
    pub const fn peel_operations(&self) -> usize {
        self.statistics.peel_operations
    }

    /// Returns true if decoding completed normally.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        matches!(
            self.termination,
            TerminationReason::Completed
                | TerminationReason::EmptyGraph
        )
    }
}

// ============================================================================
// Union-Find state
// ============================================================================

/// One Union-Find cluster.
#[derive(Debug, Clone)]
struct Cluster {
    parent: usize,
    rank: usize,

    /// XOR parity of active detection events in this cluster.
    parity: bool,

    /// Whether the cluster has access to a graph boundary.
    has_boundary: bool,
}

impl Cluster {
    fn new(
        index: usize,
        active: bool,
    ) -> Self {
        Self {
            parent: index,
            rank: 0,
            parity: active,
            has_boundary: false,
        }
    }
}

/// Internal Union-Find state.
///
/// The vector is indexed only after a deterministic graph-node-to-index map
/// has been constructed. All indexing is checked.
#[derive(Debug, Clone)]
struct UnionFindState {
    nodes: Vec<NodeId>,
    indices: BTreeMap<NodeId, usize>,
    clusters: Vec<Cluster>,
}

impl UnionFindState {
    fn from_graph(
        graph: &DecodingGraph,
    ) -> Result<Self, UnionFindError> {
        let nodes: Vec<NodeId> =
            graph
                .nodes()
                .map(|node| node.id())
                .collect();

        let mut indices = BTreeMap::new();

        for (index, node) in nodes.iter().copied().enumerate() {
            indices.insert(node, index);
        }

        let clusters = nodes
            .iter()
            .enumerate()
            .map(|(index, _)| {
                // Every node in the canonical decoding graph represents an
                // actual detection event. Inactive events are rejected by the
                // graph at insertion time.
                Cluster::new(index, true)
            })
            .collect();

        Ok(Self {
            nodes,
            indices,
            clusters,
        })
    }

    fn index_of(
        &self,
        node: NodeId,
    ) -> Result<usize, UnionFindError> {
        self.indices
            .get(&node)
            .copied()
            .ok_or(
                UnionFindError::UnknownNode { node },
            )
    }

    fn find(
        &mut self,
        index: usize,
    ) -> Result<usize, UnionFindError> {
        let parent = self
            .clusters
            .get(index)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index,
                },
            )?
            .parent;

        if parent == index {
            return Ok(index);
        }

        let root = self.find(parent)?;

        let cluster = self
            .clusters
            .get_mut(index)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index,
                },
            )?;

        cluster.parent = root;

        Ok(root)
    }

    fn cluster_state(
        &mut self,
        index: usize,
    ) -> Result<(bool, bool), UnionFindError> {
        let root = self.find(index)?;

        let cluster = self
            .clusters
            .get(root)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index: root,
                },
            )?;

        Ok((
            cluster.parity,
            cluster.has_boundary,
        ))
    }

    fn mark_boundary(
        &mut self,
        index: usize,
    ) -> Result<(), UnionFindError> {
        let root = self.find(index)?;

        let cluster = self
            .clusters
            .get_mut(root)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index: root,
                },
            )?;

        cluster.has_boundary = true;

        Ok(())
    }

    fn union(
        &mut self,
        left: usize,
        right: usize,
    ) -> Result<bool, UnionFindError> {
        let mut left_root = self.find(left)?;
        let mut right_root = self.find(right)?;

        if left_root == right_root {
            return Ok(false);
        }

        let left_rank = self
            .clusters
            .get(left_root)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index: left_root,
                },
            )?
            .rank;

        let right_rank = self
            .clusters
            .get(right_root)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index: right_root,
                },
            )?
            .rank;

        // Deterministic union-by-rank with lower root ID as tie breaker.
        if left_rank < right_rank
            || (left_rank == right_rank
                && left_root > right_root)
        {
            std::mem::swap(
                &mut left_root,
                &mut right_root,
            );
        }

        let right_cluster = self
            .clusters
            .get(right_root)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index: right_root,
                })?
            .clone();

        {
            let left_cluster = self
                .clusters
                .get_mut(left_root)
                .ok_or(
                    UnionFindError::InternalIndexOutOfRange {
                        index: left_root,
                    },
                )?;

            left_cluster.parity ^= right_cluster.parity;
            left_cluster.has_boundary |=
                right_cluster.has_boundary;
        }

        let right_parent = self
            .clusters
            .get_mut(right_root)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index: right_root,
                },
            )?;

        right_parent.parent = left_root;

        if left_rank == right_rank {
            let root = self
                .clusters
                .get_mut(left_root)
                .ok_or(
                    UnionFindError::InternalIndexOutOfRange {
                        index: left_root,
                    },
                )?;

            root.rank = root
                .rank
                .checked_add(1)
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;
        }

        Ok(true)
    }

    fn root_count(
        &mut self,
    ) -> Result<usize, UnionFindError> {
        let mut roots = std::collections::BTreeSet::new();

        for index in 0..self.clusters.len() {
            roots.insert(self.find(index)?);
        }

        Ok(roots.len())
    }

    fn unresolved_nodes(
        &mut self,
    ) -> Result<Vec<NodeId>, UnionFindError> {
        let mut unresolved = Vec::new();

        for (index, node) in self.nodes.iter().copied().enumerate() {
            let (parity, boundary) =
                self.cluster_state(index)?;

            if parity && !boundary {
                unresolved.push(node);
            }
        }

        Ok(unresolved)
    }
}

// ============================================================================
// Decoder
// ============================================================================

/// Deterministic Union-Find decoder.
///
/// The decoder consumes the canonical `DecodingGraph`. Resource safety is
/// enforced before internal state allocation.
#[derive(Debug, Clone, Copy)]
pub struct UnionFindDecoder {
    config: UnionFindConfig,
}

impl UnionFindDecoder {
    /// Creates a production decoder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: UnionFindConfig::production(),
        }
    }

    /// Creates a decoder from explicit configuration.
    #[must_use]
    pub const fn with_config(
        config: UnionFindConfig,
    ) -> Self {
        Self { config }
    }

    /// Returns the decoder configuration.
    #[must_use]
    pub const fn config(&self) -> UnionFindConfig {
        self.config
    }

    /// Decodes without an external resource manager.
    ///
    /// Central `QecLimits` are still enforced.
    pub fn decode(
        &self,
        graph: &DecodingGraph,
    ) -> Result<DecodeResult, UnionFindError> {
        let cancellation = CancellationToken::new();

        self.decode_with_context(
            graph,
            &cancellation,
            None,
        )
    }

    /// Decodes with the canonical cancellation token.
    pub fn decode_with_cancellation(
        &self,
        graph: &DecodingGraph,
        cancellation: &CancellationToken,
    ) -> Result<DecodeResult, UnionFindError> {
        self.decode_with_context(
            graph,
            cancellation,
            None,
        )
    }

    /// Decodes with canonical cancellation and resource accounting.
    ///
    /// `ResourceManager` is used for accounting and runtime resource
    /// enforcement; `QecLimits` remains the decoder's authoritative
    /// configuration policy.
    pub fn decode_with_resources(
        &self,
        graph: &DecodingGraph,
        cancellation: &CancellationToken,
        resources: &ResourceManager,
    ) -> Result<DecodeResult, UnionFindError> {
        self.decode_with_context(
            graph,
            cancellation,
            Some(resources),
        )
    }

    fn decode_with_context(
        &self,
        graph: &DecodingGraph,
        cancellation: &CancellationToken,
        resources: Option<&ResourceManager>,
    ) -> Result<DecodeResult, UnionFindError> {
        let started = Instant::now();

        cancellation
            .check()
            .map_err(UnionFindError::from)?;

        // ------------------------------------------------------------------
        // Configuration validation
        // ------------------------------------------------------------------

        self.config
            .limits
            .validate()
            .map_err(UnionFindError::from)?;

        // ------------------------------------------------------------------
        // Graph validation and allocation-free preflight
        // ------------------------------------------------------------------

        graph
            .validate()
            .map_err(UnionFindError::from)?;

        let estimate =
            DecodingGraph::preflight(
                &self.config.limits,
                graph.node_count(),
                graph.boundary_count(),
                graph.edge_count(),
            )
            .map_err(UnionFindError::from)?;

        if let Some(manager) = resources {
            manager
                .check()
                .map_err(UnionFindError::from)?;

            manager
                .record_graph_nodes(
                    u64::try_from(
                        graph.total_node_count(),
                    )
                    .map_err(
                        |_| UnionFindError::ArithmeticOverflow,
                    )?,
                )
                .map_err(UnionFindError::from)?;

            manager
                .record_graph_edges(
                    u64::try_from(
                        graph.edge_count(),
                    )
                    .map_err(
                        |_| UnionFindError::ArithmeticOverflow,
                    )?,
                )
                .map_err(UnionFindError::from)?;

            // Reservation exists only for the duration of decoding.
            // The RAII guard releases it when the operation exits.
            let _memory = manager
                .reserve_memory(
                    estimate.estimated_memory_bytes(),
                )
                .map_err(UnionFindError::from)?;

            return self
                .decode_inner(
                    graph,
                    cancellation,
                    resources,
                    started,
                );
        }

        self.decode_inner(
            graph,
            cancellation,
            None,
            started,
        )
    }

    fn decode_inner(
        &self,
        graph: &DecodingGraph,
        cancellation: &CancellationToken,
        resources: Option<&ResourceManager>,
        started: Instant,
    ) -> Result<DecodeResult, UnionFindError> {
        if graph.is_empty() {
            return Ok(DecodeResult {
                correction: Correction::empty(),
                statistics: DecodeStatistics {
                    growth_operations: 0,
                    union_operations: 0,
                    peel_operations: 0,
                    clusters: 0,
                    detection_nodes: 0,
                    boundary_nodes: graph.boundary_count(),
                    graph_edges: graph.edge_count(),
                    elapsed_nanos: elapsed_nanos(
                        started,
                    )?,
                },
                termination:
                    TerminationReason::EmptyGraph,
            });
        }

        let mut state =
            UnionFindState::from_graph(graph)?;

        let mut edges: Vec<GraphEdge> =
            graph.edges().cloned().collect();

        // Canonical deterministic order:
        //
        // 1. edge weight
        // 2. first endpoint
        // 3. second endpoint
        // 4. edge kind
        edges.sort_by(|left, right| {
            (
                left.weight().value(),
                left.first(),
                left.second(),
                left.kind(),
            )
                .cmp(&(
                    right.weight().value(),
                    right.first(),
                    right.second(),
                    right.kind(),
                ))
        });

        let mut growth_operations = 0usize;
        let mut union_operations = 0usize;

        // ==================================================================
        // Growth / cluster resolution
        // ==================================================================

        for edge in edges.iter() {
            cancellation
                .check()
                .map_err(UnionFindError::from)?;

            record_iteration(resources)?;

            growth_operations = growth_operations
                .checked_add(1)
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;

            if growth_operations
                > self
                    .config
                    .budget
                    .max_growth_operations
            {
                return Err(
                    UnionFindError::GrowthBudgetExceeded {
                        limit: self
                            .config
                            .budget
                            .max_growth_operations,
                    },
                );
            }

            let first = edge.first();
            let second = edge.second();

            match (first, second) {
                (
                    GraphEndpoint::Detection(left),
                    GraphEndpoint::Detection(right),
                ) => {
                    let left_index =
                        state.index_of(left)?;

                    let right_index =
                        state.index_of(right)?;

                    let (
                        left_parity,
                        left_boundary,
                    ) =
                        state.cluster_state(
                            left_index,
                        )?;

                    let (
                        right_parity,
                        right_boundary,
                    ) =
                        state.cluster_state(
                            right_index,
                        )?;

                    // Two already-resolved clusters do not need to be joined.
                    if (!left_parity
                        && left_boundary)
                        && (!right_parity
                            && right_boundary)
                    {
                        continue;
                    }

                    // If both roots are identical, this edge does not add
                    // information to the spanning forest.
                    let left_root =
                        state.find(left_index)?;

                    let right_root =
                        state.find(right_index)?;

                    if left_root == right_root {
                        continue;
                    }

                    if union_operations
                        >= self
                            .config
                            .budget
                            .max_union_operations
                    {
                        return Err(
                            UnionFindError::UnionBudgetExceeded {
                                limit: self
                                    .config
                                    .budget
                                    .max_union_operations,
                            },
                        );
                    }

                    // A deterministic cluster-growth rule:
                    //
                    // - connect if at least one cluster remains unresolved;
                    // - never merge two fully resolved clusters.
                    if left_parity
                        || right_parity
                        || !left_boundary
                        || !right_boundary
                    {
                        let merged =
                            state.union(
                                left_index,
                                right_index,
                            )?;

                        if merged {
                            union_operations =
                                union_operations
                                    .checked_add(1)
                                    .ok_or(
                                        UnionFindError::ArithmeticOverflow,
                                    )?;
                        }
                    }
                }

                (
                    GraphEndpoint::Detection(node),
                    GraphEndpoint::Boundary(_),
                )
                | (
                    GraphEndpoint::Boundary(_),
                    GraphEndpoint::Detection(node),
                ) => {
                    let index =
                        state.index_of(node)?;

                    let (
                        parity,
                        has_boundary,
                    ) =
                        state.cluster_state(
                            index,
                        )?;

                    // A boundary is useful only for a cluster that still has
                    // odd detection parity and is not already attached.
                    if parity && !has_boundary {
                        state.mark_boundary(index)?;
                    }
                }

                (
                    GraphEndpoint::Boundary(_),
                    GraphEndpoint::Boundary(_),
                ) => {
                    // Boundary-to-boundary edges do not alter detection
                    // parity. They are retained in the graph for other
                    // decoders but are not required by this Union-Find pass.
                }
            }
        }

        // ==================================================================
        // Boundary resolution pass
        // ==================================================================

        for edge in edges.iter() {
            cancellation
                .check()
                .map_err(UnionFindError::from)?;

            record_iteration(resources)?;

            let node = match (
                edge.first(),
                edge.second(),
            ) {
                (
                    GraphEndpoint::Detection(node),
                    GraphEndpoint::Boundary(_),
                )
                | (
                    GraphEndpoint::Boundary(_),
                    GraphEndpoint::Detection(node),
                ) => node,

                _ => continue,
            };

            let index =
                state.index_of(node)?;

            let (
                parity,
                has_boundary,
            ) =
                state.cluster_state(index)?;

            if parity && !has_boundary {
                state.mark_boundary(index)?;
            }
        }

        // ==================================================================
        // Validate logical decoder state
        // ==================================================================

        let unresolved =
            state.unresolved_nodes()?;

        if !unresolved.is_empty() {
            return Err(
                UnionFindError::UnresolvedSyndrome {
                    vertices:
                        unresolved.len(),
                },
            );
        }

        // ==================================================================
        // Correction extraction
        // ==================================================================

        //
        // A correction is the deterministic spanning forest selected during
        // cluster growth plus the minimum-weight boundary edges that actually
        // resolved odd clusters.
        //
        // We reconstruct the selected forest deterministically from the final
        // cluster relationships rather than maintaining a second graph.
        //

        let correction =
            extract_correction(
                &mut state,
                &edges,
                cancellation,
                resources,
                self.config.budget,
            )?;

        let clusters =
            state.root_count()?;

        let peel_operations =
            correction.len();

        let elapsed =
            elapsed_nanos(started)?;

        Ok(DecodeResult {
            correction,
            statistics: DecodeStatistics {
                growth_operations,
                union_operations,
                peel_operations,
                clusters,
                detection_nodes:
                    graph.node_count(),
                boundary_nodes:
                    graph.boundary_count(),
                graph_edges:
                    graph.edge_count(),
                elapsed_nanos: elapsed,
            },
            termination:
                TerminationReason::Completed,
        })
    }
}

impl Default for UnionFindDecoder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Correction extraction
// ============================================================================

/// Extracts a deterministic correction forest.
///
/// The extraction pass replays the same deterministic edge ordering over a
/// fresh local Union-Find state. This makes the correction reproducible and
/// avoids storing an additional mutable graph representation during decoding.
fn extract_correction(
    state: &mut UnionFindState,
    edges: &[GraphEdge],
    cancellation: &CancellationToken,
    resources: Option<&ResourceManager>,
    budget: DecoderBudget,
) -> Result<Correction, UnionFindError> {
    let mut replay =
        UnionFindReplay::from_state(state)?;

    let mut selected = Vec::new();

    for edge in edges {
        cancellation
            .check()
            .map_err(UnionFindError::from)?;

        record_iteration(resources)?;

        match (
            edge.first(),
            edge.second(),
        ) {
            (
                GraphEndpoint::Detection(left),
                GraphEndpoint::Detection(right),
            ) => {
                let left_index =
                    replay.index_of(left)?;

                let right_index =
                    replay.index_of(right)?;

                let left_root =
                    replay.find(left_index)?;

                let right_root =
                    replay.find(right_index)?;

                if left_root == right_root {
                    continue;
                }

                let (
                    left_parity,
                    left_boundary,
                ) =
                    replay.cluster_state(
                        left_root,
                    )?;

                let (
                    right_parity,
                    right_boundary,
                ) =
                    replay.cluster_state(
                        right_root,
                    )?;

                if (!left_parity
                    && left_boundary)
                    && (!right_parity
                        && right_boundary)
                {
                    continue;
                }

                if replay.union(
                    left_index,
                    right_index,
                )? {
                    if selected.len()
                        >= budget
                            .max_peel_operations
                    {
                        return Err(
                            UnionFindError::PeelBudgetExceeded {
                                limit: budget
                                    .max_peel_operations,
                            },
                        );
                    }

                    selected.push(
                        edge.clone(),
                    );
                }
            }

            (
                GraphEndpoint::Detection(node),
                GraphEndpoint::Boundary(_),
            )
            | (
                GraphEndpoint::Boundary(_),
                GraphEndpoint::Detection(node),
            ) => {
                let index =
                    replay.index_of(node)?;

                let (
                    parity,
                    boundary,
                ) =
                    replay.cluster_state(
                        index,
                    )?;

                if parity && !boundary {
                    replay.mark_boundary(
                        index,
                    )?;

                    if selected.len()
                        >= budget
                            .max_peel_operations
                    {
                        return Err(
                            UnionFindError::PeelBudgetExceeded {
                                limit: budget
                                    .max_peel_operations,
                            },
                        );
                    }

                    selected.push(
                        edge.clone(),
                    );
                }
            }

            (
                GraphEndpoint::Boundary(_),
                GraphEndpoint::Boundary(_),
            ) => {}
        }
    }

    selected.sort_by(|left, right| {
        (
            left.weight().value(),
            left.first(),
            left.second(),
            left.kind(),
        )
            .cmp(&(
                right.weight().value(),
                right.first(),
                right.second(),
                right.kind(),
            ))
    });

    Ok(Correction::new(selected))
}

// ============================================================================
// Replay state
// ============================================================================

/// Minimal deterministic replay state used for correction extraction.
#[derive(Debug, Clone)]
struct UnionFindReplay {
    nodes: Vec<NodeId>,
    indices: BTreeMap<NodeId, usize>,
    clusters: Vec<Cluster>,
}

impl UnionFindReplay {
    fn from_state(
        state: &UnionFindState,
    ) -> Result<Self, UnionFindError> {
        let mut indices =
            BTreeMap::new();

        for (index, node) in
            state.nodes.iter().copied().enumerate()
        {
            indices.insert(node, index);
        }

        let clusters = state
            .nodes
            .iter()
            .enumerate()
            .map(|(index, _)| {
                Cluster::new(index, true)
            })
            .collect();

        Ok(Self {
            nodes: state.nodes.clone(),
            indices,
            clusters,
        })
    }

    fn index_of(
        &self,
        node: NodeId,
    ) -> Result<usize, UnionFindError> {
        self.indices
            .get(&node)
            .copied()
            .ok_or(
                UnionFindError::UnknownNode {
                    node,
                },
            )
    }

    fn find(
        &mut self,
        index: usize,
    ) -> Result<usize, UnionFindError> {
        let parent = self
            .clusters
            .get(index)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index,
                },
            )?
            .parent;

        if parent == index {
            return Ok(index);
        }

        let root =
            self.find(parent)?;

        self.clusters
            .get_mut(index)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index,
                },
            )?
            .parent = root;

        Ok(root)
    }

    fn cluster_state(
        &mut self,
        index: usize,
    ) -> Result<(bool, bool), UnionFindError> {
        let root = self.find(index)?;

        let cluster =
            self.clusters
                .get(root)
                .ok_or(
                    UnionFindError::InternalIndexOutOfRange {
                        index: root,
                    },
                )?;

        Ok((
            cluster.parity,
            cluster.has_boundary,
        ))
    }

    fn mark_boundary(
        &mut self,
        index: usize,
    ) -> Result<(), UnionFindError> {
        let root =
            self.find(index)?;

        self.clusters
            .get_mut(root)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index: root,
                },
            )?
            .has_boundary = true;

        Ok(())
    }

    fn union(
        &mut self,
        left: usize,
        right: usize,
    ) -> Result<bool, UnionFindError> {
        let mut left_root =
            self.find(left)?;

        let mut right_root =
            self.find(right)?;

        if left_root == right_root {
            return Ok(false);
        }

        let left_rank =
            self.clusters
                .get(left_root)
                .ok_or(
                    UnionFindError::InternalIndexOutOfRange {
                        index: left_root,
                    },
                )?
                .rank;

        let right_rank =
            self.clusters
                .get(right_root)
                .ok_or(
                    UnionFindError::InternalIndexOutOfRange {
                        index: right_root,
                    },
                )?
                .rank;

        if left_rank < right_rank
            || (
                left_rank == right_rank
                    && left_root > right_root
            )
        {
            std::mem::swap(
                &mut left_root,
                &mut right_root,
            );
        }

        let right =
            self.clusters
                .get(right_root)
                .ok_or(
                    UnionFindError::InternalIndexOutOfRange {
                        index: right_root,
                    },
                )?
                .clone();

        {
            let left =
                self.clusters
                    .get_mut(left_root)
                    .ok_or(
                        UnionFindError::InternalIndexOutOfRange {
                            index: left_root,
                        },
                    )?;

            left.parity ^= right.parity;
            left.has_boundary |=
                right.has_boundary;
        }

        self.clusters
            .get_mut(right_root)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index: right_root,
                },
            )?
            .parent = left_root;

        if left_rank == right_rank {
            self.clusters
                .get_mut(left_root)
                .ok_or(
                    UnionFindError::InternalIndexOutOfRange {
                        index: left_root,
                    },
                )?
                .rank = left_rank
                .checked_add(1)
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;
        }

        Ok(true)
    }
}

// ============================================================================
// Resource helpers
// ============================================================================

fn record_iteration(
    resources: Option<&ResourceManager>,
) -> Result<(), UnionFindError> {
    if let Some(manager) = resources {
        manager
            .record_decoder_iterations(1)
            .map_err(UnionFindError::from)?;
    }

    Ok(())
}

fn elapsed_nanos(
    started: Instant,
) -> Result<u64, UnionFindError> {
    u64::try_from(
        started.elapsed().as_nanos(),
    )
    .map_err(|_| {
        UnionFindError::ArithmeticOverflow
    })
}

// ============================================================================
// Convenience APIs
// ============================================================================

/// Decodes using the canonical production configuration.
pub fn decode(
    graph: &DecodingGraph,
) -> Result<DecodeResult, UnionFindError> {
    UnionFindDecoder::new()
        .decode(graph)
}

/// Decodes using explicit Union-Find configuration.
pub fn decode_with_config(
    graph: &DecodingGraph,
    config: UnionFindConfig,
) -> Result<DecodeResult, UnionFindError> {
    UnionFindDecoder::with_config(config)
        .decode(graph)
}

/// Decodes and returns the canonical [`QecError`] type.
pub fn decode_qec(
    graph: &DecodingGraph,
) -> QecResult<DecodeResult> {
    decode(graph).map_err(QecError::from)
}

/// Decodes with canonical cancellation and resource accounting.
pub fn decode_qec_with_context(
    graph: &DecodingGraph,
    cancellation: &CancellationToken,
    resources: &ResourceManager,
) -> QecResult<DecodeResult> {
    UnionFindDecoder::new()
        .decode_with_resources(
            graph,
            cancellation,
            resources,
        )
        .map_err(QecError::from)
}

// ============================================================================
// Errors
// ============================================================================

/// Union-Find-specific diagnostic error.
///
/// This remains available for detailed local diagnostics, while public
/// high-level QEC APIs can convert it to [`QecError`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnionFindError {
    /// The central QEC resource policy is invalid.
    InvalidLimits {
        message: String,
    },

    /// Canonical decoding graph rejected the workload.
    InvalidGraph {
        message: String,
    },

    /// A node was referenced that is not present in the graph.
    UnknownNode {
        node: NodeId,
    },

    /// Internal state indexing failed.
    InternalIndexOutOfRange {
        index: usize,
    },

    /// Checked arithmetic failed.
    ArithmeticOverflow,

    /// Decoder growth budget was exhausted.
    GrowthBudgetExceeded {
        limit: usize,
    },

    /// Decoder union budget was exhausted.
    UnionBudgetExceeded {
        limit: usize,
    },

    /// Correction extraction budget was exhausted.
    PeelBudgetExceeded {
        limit: usize,
    },

    /// Canonical QEC cancellation was requested.
    Cancelled {
        message: String,
    },

    /// Detection events remain unresolved.
    UnresolvedSyndrome {
        vertices: usize,
    },

    /// Resource manager rejected an operation.
    ResourceLimit {
        message: String,
    },
}

impl fmt::Display for UnionFindError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidLimits { message } => {
                write!(
                    formatter,
                    "invalid Union-Find QEC limits: {message}"
                )
            }

            Self::InvalidGraph { message } => {
                write!(
                    formatter,
                    "invalid decoding graph: {message}"
                )
            }

            Self::UnknownNode { node } => {
                write!(
                    formatter,
                    "unknown decoding node {node}"
                )
            }

            Self::InternalIndexOutOfRange {
                index,
            } => {
                write!(
                    formatter,
                    "internal Union-Find index {index} is out of range"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "arithmetic overflow during Union-Find decoding",
                )
            }

            Self::GrowthBudgetExceeded {
                limit,
            } => {
                write!(
                    formatter,
                    "Union-Find growth budget of {limit} operations exceeded"
                )
            }

            Self::UnionBudgetExceeded {
                limit,
            } => {
                write!(
                    formatter,
                    "Union-Find union budget of {limit} operations exceeded"
                )
            }

            Self::PeelBudgetExceeded {
                limit,
            } => {
                write!(
                    formatter,
                    "Union-Find correction-extraction budget of {limit} operations exceeded"
                )
            }

            Self::Cancelled { message } => {
                write!(
                    formatter,
                    "Union-Find decoding cancelled: {message}"
                )
            }

            Self::UnresolvedSyndrome {
                vertices,
            } => {
                write!(
                    formatter,
                    "{vertices} detection-event vertices remain unresolved"
                )
            }

            Self::ResourceLimit { message } => {
                write!(
                    formatter,
                    "Union-Find resource limit exceeded: {message}"
                )
            }
        }
    }
}

impl std::error::Error for UnionFindError {}

// ============================================================================
// Error integration
// ============================================================================

impl From<super::limits::LimitError>
    for UnionFindError
{
    fn from(
        error: super::limits::LimitError,
    ) -> Self {
        Self::InvalidLimits {
            message: error.to_string(),
        }
    }
}

impl From<super::decoding_graph::DecodingGraphError>
    for UnionFindError
{
    fn from(
        error: super::decoding_graph::DecodingGraphError,
    ) -> Self {
        Self::InvalidGraph {
            message: error.to_string(),
        }
    }
}

impl From<super::resources::ResourceError>
    for UnionFindError
{
    fn from(
        error: super::resources::ResourceError,
    ) -> Self {
        Self::ResourceLimit {
            message: error.to_string(),
        }
    }
}

impl From<QecError>
    for UnionFindError
{
    fn from(
        error: QecError,
    ) -> Self {
        if error.is_cancellation() {
            return Self::Cancelled {
                message: error.to_string(),
            };
        }

        Self::InvalidGraph {
            message: error.to_string(),
        }
    }
}

impl From<UnionFindError>
    for QecError
{
    fn from(
        error: UnionFindError,
    ) -> Self {
        match error {
            UnionFindError::InvalidLimits {
                message,
            } => QecError::invalid_input(
                message,
            ),

            UnionFindError::InvalidGraph {
                message,
            } => QecError::invalid_graph(
                message,
            ),

            UnionFindError::UnknownNode {
                node,
            } => QecError::invalid_graph(
                format!(
                    "unknown decoding node {node}"
                ),
            ),

            UnionFindError::InternalIndexOutOfRange {
                index,
            } => QecError::invariant(
                "union_find_internal_index",
                format!(
                    "internal index {index} is out of range"
                ),
            ),

            UnionFindError::ArithmeticOverflow => {
                QecError::numerical_failure(
                    super::errors::NumericalOperation::Accumulation,
                    "Union-Find checked arithmetic overflow",
                )
            }

            UnionFindError::GrowthBudgetExceeded {
                limit,
            } => QecError::resource_limit(
                QecResourceKind::DecoderIterations,
                limit as u128,
                limit as u128,
                format!(
                    "Union-Find growth budget of {limit} operations exceeded"
                ),
            ),

            UnionFindError::UnionBudgetExceeded {
                limit,
            } => QecError::resource_limit(
                QecResourceKind::DecoderIterations,
                limit as u128,
                limit as u128,
                format!(
                    "Union-Find union budget of {limit} operations exceeded"
                ),
            ),

            UnionFindError::PeelBudgetExceeded {
                limit,
            } => QecError::resource_limit(
                QecResourceKind::DecoderIterations,
                limit as u128,
                limit as u128,
                format!(
                    "Union-Find correction extraction budget of {limit} operations exceeded"
                ),
            ),

            UnionFindError::Cancelled {
                message,
            } => QecError::cancelled(
                message,
            ),

            UnionFindError::UnresolvedSyndrome {
                vertices,
            } => QecError::decoder_failure(
                DecoderKind::UnionFind,
                format!(
                    "{vertices} detection-event vertices remain unresolved"
                ),
            ),

            UnionFindError::ResourceLimit {
                message,
            } => QecError::resource_limit(
                QecResourceKind::DecoderIterations,
                1,
                1,
                message,
            ),
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::decoding_graph::{
        EdgeKind,
        EdgeWeight,
        GraphEndpoint,
        SpatialCoordinate,
        SpaceTimeCoordinate,
    };
    use super::syndrome::{
        MeasurementConfidence,
        MeasurementRound,
        StabilizerId,
    };

    fn limits() -> QecLimits {
        let mut limits =
            QecLimits::new();

        limits.max_graph_nodes = 64;
        limits.max_graph_edges = 128;
        limits.max_memory_bytes =
            1024 * 1024;
        limits.max_decoder_iterations =
            10_000;

        limits
    }

    fn graph() -> DecodingGraph {
        DecodingGraph::new_with_limits(
            &limits(),
        )
        .expect("test limits are valid")
    }

    fn coordinate(
        x: i64,
        y: i64,
    ) -> SpaceTimeCoordinate {
        SpaceTimeCoordinate::new(
            SpatialCoordinate::xy(x, y)
                .expect("coordinate valid"),
            MeasurementRound::new(0)
                .expect("round valid"),
        )
        .expect("space-time coordinate valid")
    }

    fn add_node(
        graph: &mut DecodingGraph,
        x: i64,
        y: i64,
        stabilizer: u64,
    ) -> NodeId {
        graph
            .add_detection_node(
                coordinate(x, y),
                StabilizerId::new(
                    stabilizer,
                ),
                MeasurementConfidence::High,
            )
            .expect("node insertion succeeds")
    }

    fn boundary(
        graph: &mut DecodingGraph,
        x: i64,
        y: i64,
    ) -> BoundaryId {
        graph
            .add_boundary_node(
                coordinate(x, y),
            )
            .expect("boundary insertion succeeds")
    }

    fn edge(
        graph: &mut DecodingGraph,
        first: GraphEndpoint,
        second: GraphEndpoint,
        weight: u64,
        kind: EdgeKind,
    ) {
        graph
            .add_edge(
                first,
                second,
                EdgeWeight::new(weight)
                    .expect("weight valid"),
                kind,
            )
            .expect("edge insertion succeeds");
    }

    #[test]
    fn empty_graph_is_safe() {
        let graph = graph();

        let result =
            decode(&graph)
                .expect("empty graph decodes");

        assert!(
            result
                .correction()
                .is_empty()
        );

        assert_eq!(
            result.termination(),
            TerminationReason::EmptyGraph
        );
    }

    #[test]
    fn single_detection_event_resolves_to_boundary() {
        let mut graph =
            graph();

        let node =
            add_node(
                &mut graph,
                0,
                0,
                0,
            );

        let boundary =
            boundary(
                &mut graph,
                1,
                0,
            );

        edge(
            &mut graph,
            GraphEndpoint::Detection(
                node,
            ),
            GraphEndpoint::Boundary(
                boundary,
            ),
            10,
            EdgeKind::Boundary,
        );

        let result =
            decode(&graph)
                .expect(
                    "boundary resolution succeeds",
                );

        assert!(
            !result
                .correction()
                .is_empty()
        );
    }

    #[test]
    fn two_detection_events_can_be_joined() {
        let mut graph =
            graph();

        let left =
            add_node(
                &mut graph,
                0,
                0,
                0,
            );

        let right =
            add_node(
                &mut graph,
                1,
                0,
                1,
            );

        edge(
            &mut graph,
            GraphEndpoint::Detection(
                left,
            ),
            GraphEndpoint::Detection(
                right,
            ),
            10,
            EdgeKind::Spatial,
        );

        let result =
            decode(&graph)
                .expect(
                    "pair resolves",
                );

        assert_eq!(
            result.union_operations(),
            1
        );

        assert_eq!(
            result.correction().len(),
            1
        );
    }

    #[test]
    fn equal_weight_edges_are_deterministic() {
        let mut first =
            graph();

        let a =
            add_node(
                &mut first,
                0,
                0,
                0,
            );

        let b =
            add_node(
                &mut first,
                1,
                0,
                1,
            );

        edge(
            &mut first,
            GraphEndpoint::Detection(a),
            GraphEndpoint::Detection(b),
            10,
            EdgeKind::Spatial,
        );

        let result_a =
            decode(&first)
                .expect("decode succeeds");

        let result_b =
            decode(&first)
                .expect("repeat decode succeeds");

        assert_eq!(
            result_a.correction(),
            result_b.correction()
        );

        assert_eq!(
            result_a.statistics().growth_operations,
            result_b.statistics().growth_operations
        );
    }

    #[test]
    fn cancellation_is_enforced() {
        let mut graph =
            graph();

        let node =
            add_node(
                &mut graph,
                0,
                0,
                0,
            );

        let boundary =
            boundary(
                &mut graph,
                1,
                0,
            );

        edge(
            &mut graph,
            GraphEndpoint::Detection(node),
            GraphEndpoint::Boundary(boundary),
            1,
            EdgeKind::Boundary,
        );

        let token =
            CancellationToken::new();

        token.request();

        let result =
            UnionFindDecoder::new()
                .decode_with_cancellation(
                    &graph,
                    &token,
                );

        assert!(
            matches!(
                result,
                Err(
                    UnionFindError::Cancelled {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn qec_error_boundary_is_available() {
        let graph = graph();

        let result =
            decode_qec(&graph)
                .expect(
                    "empty graph succeeds",
                );

        assert_eq!(
            result.termination(),
            TerminationReason::EmptyGraph
        );
    }
}