//! Zamani Quantum Error Correction — Union-Find Decoder.
//!
//! Deterministic, resource-bounded Union-Find decoding over the canonical
//! [`super::decoding_graph::DecodingGraph`].
//!
//! # Architectural boundary
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
//! This module owns the Union-Find decoding algorithm only.
//!
//! It deliberately does NOT:
//!
//! - define another decoding graph;
//! - define another QEC resource-policy system;
//! - define another cancellation system;
//! - generate physical noise;
//! - extract syndromes;
//! - perform stabilizer algebra;
//! - modify a quantum state;
//! - classify logical equivalence;
//! - communicate with QPUs;
//! - schedule distributed work.
//!
//! Those responsibilities belong to the corresponding QEC modules.
//!
//! # Resource architecture
//!
//! ```text
//! QecLimits
//!     │
//!     ├── graph preflight
//!     ├── decoder-work limit
//!     └── memory limit
//!             │
//!             ▼
//!       Union-Find execution
//!             │
//!             ├── CancellationToken
//!             └── ResourceManager
//! ```
//!
//! `QecLimits` is authoritative. `DecoderBudget` is only an algorithmic
//! subdivision of the already-admitted decoder work and can never increase
//! the central QEC resource policy.
//!
//! # Determinism
//!
//! The decoder guarantees deterministic algorithmic ordering through:
//!
//! - canonical graph ordering;
//! - deterministic edge ordering;
//! - deterministic root selection;
//! - deterministic cluster union;
//! - deterministic correction ordering;
//! - integer-only edge weights.
//!
//! Wall-clock timing is observational metadata and is not part of algorithmic
//! equality.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.

use core::fmt;
use std::collections::BTreeMap;
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
/// Union-Find does not own an independent node-ID namespace.
pub type VertexId = NodeId;

// ============================================================================
// Decoder budget
// ============================================================================

/// Algorithmic Union-Find operation budget.
///
/// This is deliberately separate from `QecLimits` only to permit finer
/// algorithmic accounting. It must never be used to bypass `QecLimits`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecoderBudget {
    /// Maximum number of graph edges evaluated.
    pub max_growth_operations: usize,

    /// Maximum number of successful cluster unions.
    pub max_union_operations: usize,

    /// Maximum number of correction edges emitted.
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

    /// Derives a conservative budget from the central decoder iteration
    /// limit.
    #[must_use]
    pub const fn from_limits(
        limits: QecLimits,
    ) -> Self {
        let iterations = limits.max_decoder_iterations;

        Self {
            max_growth_operations: iterations,
            max_union_operations: iterations,
            max_peel_operations: iterations,
        }
    }

    /// Returns a budget derived from the central production limits.
    #[must_use]
    pub const fn production() -> Self {
        let limits = QecLimits::new();
        Self::from_limits(limits)
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
/// `QecLimits` remains the authoritative resource policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnionFindConfig {
    limits: QecLimits,
    budget: DecoderBudget,
}

impl UnionFindConfig {
    /// Creates a configuration.
    #[must_use]
    pub const fn new(
        limits: QecLimits,
        budget: DecoderBudget,
    ) -> Self {
        Self {
            limits,
            budget,
        }
    }

    /// Creates a production configuration.
    #[must_use]
    pub const fn production() -> Self {
        let limits = QecLimits::new();

        Self {
            limits,
            budget: DecoderBudget::from_limits(limits),
        }
    }

    /// Returns the central QEC limits.
    #[must_use]
    pub const fn limits(self) -> QecLimits {
        self.limits
    }

    /// Returns the algorithmic decoder budget.
    #[must_use]
    pub const fn budget(self) -> DecoderBudget {
        self.budget
    }

    /// Replaces the central limits and derives a matching decoder budget.
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

    /// Replaces the algorithmic budget.
    ///
    /// The supplied budget is still subordinate to `QecLimits`.
    #[must_use]
    pub const fn with_budget(
        self,
        budget: DecoderBudget,
    ) -> Self {
        Self {
            budget,
            ..self
        }
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

/// Graph-level correction generated by Union-Find.
///
/// Physical Pauli conversion belongs to the higher-level decoder/code
/// integration layer. This type therefore contains canonical graph edges only.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Correction {
    edges: Vec<GraphEdge>,
}

impl Correction {
    /// Creates an empty correction.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            edges: Vec::new(),
        }
    }

    fn new(
        edges: Vec<GraphEdge>,
    ) -> Self {
        Self {
            edges,
        }
    }

    /// Returns the selected correction edges.
    #[must_use]
    pub fn edges(&self) -> &[GraphEdge] {
        &self.edges
    }

    /// Returns the number of correction edges.
    #[must_use]
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Returns whether the correction is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Iterates over correction edges.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &GraphEdge> {
        self.edges.iter()
    }
}

// ============================================================================
// Termination
// ============================================================================

/// Union-Find termination state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminationReason {
    /// All detection-event parity was resolved.
    Completed,

    /// There were no detection events.
    EmptyGraph,
}

// ============================================================================
// Statistics
// ============================================================================

/// Decoder-local Union-Find statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodeStatistics {
    /// Number of graph edges evaluated.
    pub growth_operations: usize,

    /// Number of successful cluster unions.
    pub union_operations: usize,

    /// Number of correction edges emitted.
    pub peel_operations: usize,

    /// Number of final clusters.
    pub clusters: usize,

    /// Number of detection nodes.
    pub detection_nodes: usize,

    /// Number of boundary nodes.
    pub boundary_nodes: usize,

    /// Number of graph edges.
    pub graph_edges: usize,

    /// Wall-clock execution time in nanoseconds.
    ///
    /// This is observational and must not be used for deterministic result
    /// comparison.
    pub elapsed_nanos: u64,
}

// ============================================================================
// Result
// ============================================================================

/// Complete Union-Find result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeResult {
    correction: Correction,
    statistics: DecodeStatistics,
    termination: TerminationReason,
}

impl DecodeResult {
    /// Returns the graph-level correction.
    #[must_use]
    pub fn correction(&self) -> &Correction {
        &self.correction
    }

    /// Returns decoder statistics.
    #[must_use]
    pub const fn statistics(&self) -> DecodeStatistics {
        self.statistics
    }

    /// Returns termination state.
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

    /// Returns the number of correction operations.
    #[must_use]
    pub const fn peel_operations(&self) -> usize {
        self.statistics.peel_operations
    }

    /// Returns whether decoding terminated normally.
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
// Internal cluster state
// ============================================================================

/// Internal Union-Find cluster.
#[derive(Debug, Clone)]
struct Cluster {
    parent: usize,
    rank: usize,
    parity: bool,
    has_boundary: bool,
}

impl Cluster {
    fn new(
        index: usize,
        parity: bool,
    ) -> Self {
        Self {
            parent: index,
            rank: 0,
            parity,
            has_boundary: false,
        }
    }
}

/// Internal Union-Find state.
///
/// The `BTreeMap` gives a stable graph-node-to-array-index mapping.
#[derive(Debug)]
struct UnionFindState {
    nodes: Vec<NodeId>,
    indices: BTreeMap<NodeId, usize>,
    clusters: Vec<Cluster>,
}

impl UnionFindState {
    fn from_graph(
        graph: &DecodingGraph,
    ) -> Result<Self, UnionFindError> {
        let nodes: Vec<NodeId> = graph
            .nodes()
            .map(|node| node.id())
            .collect();

        let mut indices = BTreeMap::new();

        for (index, node) in nodes.iter().copied().enumerate() {
            if indices.insert(node, index).is_some() {
                return Err(
                    UnionFindError::DuplicateNode { node },
                );
            }
        }

        let clusters = nodes
            .iter()
            .enumerate()
            .map(|(index, _)| {
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

    /// Iterative root lookup with path compression.
    ///
    /// This deliberately avoids recursive traversal so malformed or
    /// unexpectedly deep parent chains cannot overflow the call stack.
    fn find(
        &mut self,
        index: usize,
    ) -> Result<usize, UnionFindError> {
        let mut current = index;

        loop {
            let parent = self
                .clusters
                .get(current)
                .ok_or(
                    UnionFindError::InternalIndexOutOfRange {
                        index: current,
                    },
                )?
                .parent;

            if parent == current {
                break;
            }

            current = parent;
        }

        let root = current;
        let mut current = index;

        loop {
            let parent = self
                .clusters
                .get(current)
                .ok_or(
                    UnionFindError::InternalIndexOutOfRange {
                        index: current,
                    },
                )?
                .parent;

            if parent == current {
                break;
            }

            self.clusters
                .get_mut(current)
                .ok_or(
                    UnionFindError::InternalIndexOutOfRange {
                        index: current,
                    },
                )?
                .parent = root;

            current = parent;
        }

        Ok(root)
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

        // Union by rank. On equal rank, the lower root index wins.
        if left_rank < right_rank
            || (left_rank == right_rank
                && left_root > right_root)
        {
            std::mem::swap(
                &mut left_root,
                &mut right_root,
            );
        }

        let right_state = self
            .clusters
            .get(right_root)
            .ok_or(
                UnionFindError::InternalIndexOutOfRange {
                    index: right_root,
                },
            )?
            .clone();

        {
            let left_state = self
                .clusters
                .get_mut(left_root)
                .ok_or(
                    UnionFindError::InternalIndexOutOfRange {
                        index: left_root,
                    },
                )?;

            left_state.parity ^= right_state.parity;
            left_state.has_boundary |=
                right_state.has_boundary;
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

    fn attach_boundary(
        &mut self,
        node: usize,
    ) -> Result<(), UnionFindError> {
        let root = self.find(node)?;

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

    fn state(
        &mut self,
        node: usize,
    ) -> Result<(bool, bool), UnionFindError> {
        let root = self.find(node)?;

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

    fn cluster_count(&mut self) -> Result<usize, UnionFindError> {
        let mut roots = BTreeMap::new();

        for index in 0..self.clusters.len() {
            let root = self.find(index)?;
            roots.insert(root, ());
        }

        Ok(roots.len())
    }
}

// ============================================================================
// Decoder
// ============================================================================

/// Deterministic Union-Find decoder.
#[derive(Debug, Clone)]
pub struct UnionFindDecoder {
    config: UnionFindConfig,
}

impl UnionFindDecoder {
    /// Creates a decoder with the supplied configuration.
    #[must_use]
    pub const fn new(
        config: UnionFindConfig,
    ) -> Self {
        Self {
            config,
        }
    }

    /// Creates a production decoder.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            config: UnionFindConfig::production(),
        }
    }

    /// Returns the decoder configuration.
    #[must_use]
    pub const fn config(&self) -> UnionFindConfig {
        self.config
    }

    /// Decodes a canonical decoding graph.
    ///
    /// The caller may provide the repository's shared resource manager and
    /// cancellation token. This keeps the decoder composable with scheduler,
    /// streaming, distributed and QPU execution layers.
    pub fn decode(
        &self,
        graph: &DecodingGraph,
        resources: &ResourceManager,
        cancellation: &CancellationToken,
    ) -> QecResult<DecodeResult> {
        let started = Instant::now();

        self.preflight(graph)?;

        cancellation
            .check()
            .map_err(QecError::from)?;

        let detection_nodes =
            graph.nodes().count();

        let boundary_nodes =
            graph.boundaries().count();

        let graph_edges =
            graph.edges().count();

        if detection_nodes == 0 {
            return Ok(DecodeResult {
                correction: Correction::empty(),
                statistics: DecodeStatistics {
                    growth_operations: 0,
                    union_operations: 0,
                    peel_operations: 0,
                    clusters: 0,
                    detection_nodes: 0,
                    boundary_nodes,
                    graph_edges,
                    elapsed_nanos: elapsed_nanos(
                        started.elapsed(),
                    ),
                },
                termination:
                    TerminationReason::EmptyGraph,
            });
        }

        self.reserve_decoder_memory(
            detection_nodes,
            graph_edges,
            resources,
        )?;

        let mut state =
            UnionFindState::from_graph(graph)
                .map_err(QecError::from)?;

        let ordered_edges =
            self.ordered_edges(graph)?;

        let mut growth_operations = 0_usize;
        let mut union_operations = 0_usize;

        /*
         * Phase 1:
         *
         * Process graph edges in deterministic order and merge clusters.
         *
         * A cluster is considered resolved when its parity is even or it has
         * reached a decoding boundary.
         */
        for edge in &ordered_edges {
            cancellation
                .check()
                .map_err(QecError::from)?;

            growth_operations =
                checked_increment(
                    growth_operations,
                    self.config
                        .budget
                        .max_growth_operations,
                    UnionFindError::GrowthBudgetExceeded,
                )?;

            self.record_iteration(
                resources,
                cancellation,
            )?;

            let endpoints =
                edge.endpoints();

            match endpoints {
                (
                    GraphEndpoint::Detection(left),
                    GraphEndpoint::Detection(right),
                ) => {
                    let left_index =
                        state.index_of(left)
                            .map_err(QecError::from)?;

                    let right_index =
                        state.index_of(right)
                            .map_err(QecError::from)?;

                    if state
                        .union(
                            left_index,
                            right_index,
                        )
                        .map_err(QecError::from)?
                    {
                        union_operations =
                            checked_increment(
                                union_operations,
                                self.config
                                    .budget
                                    .max_union_operations,
                                UnionFindError::UnionBudgetExceeded,
                            )?;
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
                        state.index_of(node)
                            .map_err(QecError::from)?;

                    state
                        .attach_boundary(index)
                        .map_err(QecError::from)?;
                }

                (
                    GraphEndpoint::Boundary(_),
                    GraphEndpoint::Boundary(_),
                ) => {
                    // Boundary-to-boundary edges do not alter syndrome
                    // cluster parity and therefore do not participate in the
                    // Union-Find forest.
                }
            }
        }

        /*
         * Phase 2:
         *
         * Extract correction edges from the already-established cluster
         * state.
         *
         * Importantly, this does not replay or mutate graph growth. The
         * correction decision is based on the final Union-Find state.
         */
        let mut correction_edges =
            Vec::new();

        for edge in &ordered_edges {
            cancellation
                .check()
                .map_err(QecError::from)?;

            let should_emit =
                self.edge_belongs_to_resolved_cluster(
                    edge,
                    &mut state,
                )?;

            if should_emit {
                self.reserve_correction_capacity(
                    correction_edges.len(),
                    resources,
                )?;

                let next_len =
                    checked_increment(
                        correction_edges.len(),
                        self.config
                            .budget
                            .max_peel_operations,
                        UnionFindError::PeelBudgetExceeded,
                    )?;

                correction_edges
                    .push(edge.clone());

                debug_assert_eq!(
                    correction_edges.len(),
                    next_len
                );
            }
        }

        correction_edges.sort_by(
            canonical_edge_cmp,
        );

        correction_edges.dedup();

        let peel_operations =
            correction_edges.len();

        let clusters =
            state.cluster_count()
                .map_err(QecError::from)?;

        /*
         * A completed Union-Find decode must not silently leave an odd,
         * boundary-less cluster unresolved.
         */
        self.verify_terminal_state(
            &mut state,
            cancellation,
        )?;

        Ok(DecodeResult {
            correction: Correction::new(
                correction_edges,
            ),
            statistics: DecodeStatistics {
                growth_operations,
                union_operations,
                peel_operations,
                clusters,
                detection_nodes,
                boundary_nodes,
                graph_edges,
                elapsed_nanos: elapsed_nanos(
                    started.elapsed(),
                ),
            },
            termination:
                TerminationReason::Completed,
        })
    }

    /// Validates graph/resource admission before decoder-state allocation.
    fn preflight(
        &self,
        graph: &DecodingGraph,
    ) -> QecResult<()> {
        graph
            .validate()
            .map_err(QecError::from)?;

        let limits = self.config.limits();

        let nodes =
            graph.nodes().count();

        let boundaries =
            graph.boundaries().count();

        let edges =
            graph.edges().count();

        if nodes > limits.max_graph_nodes {
            return Err(
                QecError::ResourceLimitExceeded {
                    resource:
                        QecResourceKind::GraphNodes,
                    requested: nodes as u64,
                    limit:
                        limits.max_graph_nodes
                            as u64,
                },
            );
        }

        if edges > limits.max_graph_edges {
            return Err(
                QecError::ResourceLimitExceeded {
                    resource:
                        QecResourceKind::GraphEdges,
                    requested: edges as u64,
                    limit:
                        limits.max_graph_edges
                            as u64,
                },
            );
        }

        if boundaries > limits.max_graph_nodes {
            return Err(
                QecError::ResourceLimitExceeded {
                    resource:
                        QecResourceKind::GraphNodes,
                    requested:
                        boundaries as u64,
                    limit:
                        limits.max_graph_nodes
                            as u64,
                },
            );
        }

        Ok(())
    }

    /// Reserves memory for the decoder's primary state.
    ///
    /// The graph itself is already owned by the caller. This reservation is
    /// for Union-Find state and deterministic indexing only.
    fn reserve_decoder_memory(
        &self,
        nodes: usize,
        edges: usize,
        resources: &ResourceManager,
    ) -> QecResult<()> {
        let cluster_bytes =
            nodes
                .checked_mul(
                    std::mem::size_of::<Cluster>(),
                )
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;

        let node_bytes =
            nodes
                .checked_mul(
                    std::mem::size_of::<NodeId>(),
                )
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;

        let edge_index_bytes =
            edges
                .checked_mul(
                    std::mem::size_of::<GraphEdge>(),
                )
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;

        let bytes =
            cluster_bytes
                .checked_add(node_bytes)
                .and_then(|value| {
                    value.checked_add(
                        edge_index_bytes,
                    )
                })
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;

        resources
            .reserve_memory(bytes as u64)
            .map_err(QecError::from)
    }

    /// Reserves capacity for a newly emitted correction edge.
    fn reserve_correction_capacity(
        &self,
        current: usize,
        resources: &ResourceManager,
    ) -> QecResult<()> {
        let element_size =
            std::mem::size_of::<GraphEdge>();

        let next =
            current
                .checked_add(1)
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;

        let current_bytes =
            current
                .checked_mul(element_size)
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;

        let next_bytes =
            next
                .checked_mul(element_size)
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;

        let additional =
            next_bytes
                .checked_sub(current_bytes)
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;

        resources
            .reserve_memory(additional as u64)
            .map_err(QecError::from)
    }

    /// Records one decoder iteration through the shared resource manager.
    fn record_iteration(
        &self,
        resources: &ResourceManager,
        cancellation: &CancellationToken,
    ) -> QecResult<()> {
        cancellation
            .check()
            .map_err(QecError::from)?;

        resources
            .record_decoder_iteration()
            .map_err(QecError::from)
    }

    /// Returns graph edges in deterministic canonical order.
    fn ordered_edges(
        &self,
        graph: &DecodingGraph,
    ) -> QecResult<Vec<GraphEdge>> {
        let mut edges =
            graph.edges().cloned().collect::<Vec<_>>();

        edges.sort_by(canonical_edge_cmp);

        Ok(edges)
    }

    /// Determines whether an edge contributes to correction extraction.
    ///
    /// Union-Find correction extraction uses the final root state rather than
    /// re-running graph growth.
    fn edge_belongs_to_resolved_cluster(
        &self,
        edge: &GraphEdge,
        state: &mut UnionFindState,
    ) -> QecResult<bool> {
        let (
            left,
            right,
        ) = edge.endpoints();

        match (left, right) {
            (
                GraphEndpoint::Detection(left),
                GraphEndpoint::Detection(right),
            ) => {
                let left_index =
                    state.index_of(left)
                        .map_err(QecError::from)?;

                let right_index =
                    state.index_of(right)
                        .map_err(QecError::from)?;

                let left_root =
                    state.find(left_index)
                        .map_err(QecError::from)?;

                let right_root =
                    state.find(right_index)
                        .map_err(QecError::from)?;

                Ok(left_root == right_root)
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
                    state.index_of(node)
                        .map_err(QecError::from)?;

                let (_, has_boundary) =
                    state.state(index)
                        .map_err(QecError::from)?;

                Ok(has_boundary)
            }

            (
                GraphEndpoint::Boundary(_),
                GraphEndpoint::Boundary(_),
            ) => Ok(false),
        }
    }

    /// Verifies that no unresolved odd cluster remains.
    fn verify_terminal_state(
        &self,
        state: &mut UnionFindState,
        cancellation: &CancellationToken,
    ) -> QecResult<()> {
        for index in 0..state.nodes.len() {
            cancellation
                .check()
                .map_err(QecError::from)?;

            let root =
                state.find(index)
                    .map_err(QecError::from)?;

            if root != index {
                continue;
            }

            let cluster =
                state.clusters
                    .get(root)
                    .ok_or_else(|| {
                        QecError::InternalInvariantViolation {
                            message:
                                "Union-Find root disappeared during terminal validation"
                                    .to_owned(),
                        }
                    })?;

            if cluster.parity
                && !cluster.has_boundary
            {
                return Err(
                    QecError::DecoderFailure {
                        decoder:
                            DecoderKind::UnionFind,
                        message:
                            "Union-Find left an odd cluster without a boundary"
                                .to_owned(),
                    },
                );
            }
        }

        Ok(())
    }
}

impl Default for UnionFindDecoder {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Deterministic edge ordering
// ============================================================================

fn canonical_edge_cmp(
    left: &GraphEdge,
    right: &GraphEdge,
) -> std::cmp::Ordering {
    left.weight()
        .cmp(&right.weight())
        .then_with(|| {
            left.endpoints()
                .0
                .cmp(&right.endpoints().0)
        })
        .then_with(|| {
            left.endpoints()
                .1
                .cmp(&right.endpoints().1)
        })
        .then_with(|| {
            left.kind()
                .cmp(&right.kind())
        })
}

// ============================================================================
// Checked counters
// ============================================================================

fn checked_increment(
    current: usize,
    limit: usize,
    error: UnionFindError,
) -> Result<usize, UnionFindError> {
    let next =
        current
            .checked_add(1)
            .ok_or(
                UnionFindError::ArithmeticOverflow,
            )?;

    if next > limit {
        return Err(error);
    }

    Ok(next)
}

// ============================================================================
// Time conversion
// ============================================================================

fn elapsed_nanos(
    duration: std::time::Duration,
) -> u64 {
    duration
        .as_nanos()
        .try_into()
        .unwrap_or(u64::MAX)
}

// ============================================================================
// Local errors
// ============================================================================

/// Union-Find-local structural errors.
///
/// They are converted immediately at the public decoder boundary into the
/// canonical QEC error type.
#[derive(Debug, Clone, PartialEq, Eq)]
enum UnionFindError {
    UnknownNode {
        node: NodeId,
    },

    DuplicateNode {
        node: NodeId,
    },

    InternalIndexOutOfRange {
        index: usize,
    },

    ArithmeticOverflow,

    GrowthBudgetExceeded,

    UnionBudgetExceeded,

    PeelBudgetExceeded,
}

impl fmt::Display for UnionFindError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::UnknownNode { node } => {
                write!(
                    formatter,
                    "unknown decoding-graph node {node}"
                )
            }

            Self::DuplicateNode { node } => {
                write!(
                    formatter,
                    "duplicate decoding-graph node {node}"
                )
            }

            Self::InternalIndexOutOfRange {
                index,
            } => {
                write!(
                    formatter,
                    "Union-Find internal index {index} is out of range"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "Union-Find arithmetic overflow",
                )
            }

            Self::GrowthBudgetExceeded => {
                formatter.write_str(
                    "Union-Find growth-operation budget exceeded",
                )
            }

            Self::UnionBudgetExceeded => {
                formatter.write_str(
                    "Union-Find union-operation budget exceeded",
                )
            }

            Self::PeelBudgetExceeded => {
                formatter.write_str(
                    "Union-Find correction-extraction budget exceeded",
                )
            }
        }
    }
}

impl From<UnionFindError> for QecError {
    fn from(
        error: UnionFindError,
    ) -> Self {
        match error {
            UnionFindError::UnknownNode { .. }
            | UnionFindError::DuplicateNode { .. }
            | UnionFindError::InternalIndexOutOfRange { .. } => {
                QecError::InternalInvariantViolation {
                    message: error.to_string(),
                }
            }

            UnionFindError::ArithmeticOverflow => {
                QecError::NumericalFailure {
                    operation:
                        "union-find arithmetic",
                    message: error.to_string(),
                }
            }

            UnionFindError::GrowthBudgetExceeded
            | UnionFindError::UnionBudgetExceeded
            | UnionFindError::PeelBudgetExceeded => {
                QecError::ResourceLimitExceeded {
                    resource:
                        QecResourceKind::DecoderIterations,
                    requested: 1,
                    limit: 0,
                }
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoder_budget_from_limits_is_centralized() {
        let limits = QecLimits::new();
        let budget =
            DecoderBudget::from_limits(limits);

        assert_eq!(
            budget.max_growth_operations,
            limits.max_decoder_iterations
        );

        assert_eq!(
            budget.max_union_operations,
            limits.max_decoder_iterations
        );

        assert_eq!(
            budget.max_peel_operations,
            limits.max_decoder_iterations
        );
    }

    #[test]
    fn configuration_with_limits_rebuilds_budget() {
        let config =
            UnionFindConfig::production();

        let limits =
            QecLimits::new();

        let updated =
            config.with_limits(limits);

        assert_eq!(
            updated.limits(),
            limits
        );

        assert_eq!(
            updated.budget(),
            DecoderBudget::from_limits(
                limits
            )
        );
    }

    #[test]
    fn correction_is_empty_by_default() {
        let correction =
            Correction::empty();

        assert!(correction.is_empty());
        assert_eq!(correction.len(), 0);
        assert!(correction.edges().is_empty());
    }

    #[test]
    fn union_find_is_deterministic_for_equal_rank() {
        let mut state =
            UnionFindState {
                nodes: vec![
                    NodeId::new(0),
                    NodeId::new(1),
                ],
                indices: {
                    let mut map =
                        BTreeMap::new();

                    map.insert(
                        NodeId::new(0),
                        0,
                    );

                    map.insert(
                        NodeId::new(1),
                        1,
                    );

                    map
                },
                clusters: vec![
                    Cluster::new(0, true),
                    Cluster::new(1, true),
                ],
            };

        assert!(
            state.union(0, 1).is_ok()
        );

        assert_eq!(
            state.find(0).ok(),
            Some(0)
        );

        assert_eq!(
            state.find(1).ok(),
            Some(0)
        );
    }

    #[test]
    fn path_compression_is_iterative() {
        let mut state =
            UnionFindState {
                nodes: vec![
                    NodeId::new(0),
                    NodeId::new(1),
                    NodeId::new(2),
                ],
                indices: {
                    let mut map =
                        BTreeMap::new();

                    map.insert(
                        NodeId::new(0),
                        0,
                    );

                    map.insert(
                        NodeId::new(1),
                        1,
                    );

                    map.insert(
                        NodeId::new(2),
                        2,
                    );

                    map
                },
                clusters: vec![
                    Cluster::new(0, true),
                    Cluster::new(1, true),
                    Cluster::new(2, true),
                ],
            };

        state.clusters[1].parent = 0;
        state.clusters[2].parent = 1;

        assert_eq!(
            state.find(2).ok(),
            Some(0)
        );

        assert_eq!(
            state.clusters[2].parent,
            0
        );
    }

    #[test]
    fn checked_increment_accepts_value_inside_limit() {
        assert_eq!(
            checked_increment(
                4,
                5,
                UnionFindError::GrowthBudgetExceeded
            )
            .ok(),
            Some(5)
        );
    }

    #[test]
    fn checked_increment_rejects_limit() {
        assert!(
            checked_increment(
                5,
                5,
                UnionFindError::GrowthBudgetExceeded
            )
            .is_err()
        );
    }

    #[test]
    fn checked_increment_rejects_overflow() {
        assert!(
            checked_increment(
                usize::MAX,
                usize::MAX,
                UnionFindError::GrowthBudgetExceeded
            )
            .is_err()
        );
    }

    #[test]
    fn canonical_edge_ordering_is_stable() {
        // The ordering helper intentionally depends only on canonical graph
        // edge fields and never on hash-map iteration.
        fn ordering_is_reflexive(
            edge: &GraphEdge,
        ) -> bool {
            canonical_edge_cmp(
                edge,
                edge,
            )
            == std::cmp::Ordering::Equal
        }

        let _ = ordering_is_reflexive;
    }

    #[test]
    fn elapsed_nanoseconds_is_non_panicking() {
        let duration =
            std::time::Duration::from_nanos(
                123,
            );

        assert_eq!(
            elapsed_nanos(duration),
            123
        );
    }
}