//! Zamani Quantum Error Correction — Minimum-Weight Perfect Matching.
//!
//! This module provides a deterministic, resource-bounded MWPM decoder over
//! [`DecodingGraph`].
//!
//! The decoder operates on the metric closure of the decoding graph:
//!
//! ```text
//!                 DecodingGraph
//!                       │
//!              ┌────────┴────────┐
//!              │                 │
//!        detection events     boundaries
//!              │                 │
//!              └────────┬────────┘
//!                       ▼
//!                shortest paths
//!                       │
//!                       ▼
//!                 metric closure
//!                       │
//!                       ▼
//!                 exact MWPM
//!                       │
//!             ┌─────────┴─────────┐
//!             ▼                   ▼
//!       event ↔ event       event ↔ boundary
//!             │                   │
//!             └─────────┬─────────┘
//!                       ▼
//!                correction paths
//! ```
//!
//! # Design principles
//!
//! * `QecLimits` is the authoritative resource policy.
//! * Cancellation is cooperative and checked throughout expensive work.
//! * Matching is exact for the supported bounded problem.
//! * Event-to-boundary matching is supported.
//! * Boundary nodes are virtual decoder endpoints; they are not ordinary
//!   detection events.
//! * All ordering and tie-breaking is deterministic.
//! * No floating-point values participate in matching decisions.
//! * Arithmetic is checked.
//! * Malformed graphs produce structured errors.
//! * The decoder never mutates quantum state.
//! * The decoder never silently falls back to a greedy approximation.
//!
//! The exact solver is intentionally bounded. For large workloads, callers
//! should use a scalable decoder implementation rather than allowing an
//! exponential algorithm to consume unbounded resources.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::fmt;

use super::cancellation::CancellationToken;
use super::decoding_graph::{
    DecodingGraph,
    GraphEdge,
    GraphEndpoint,
    NodeId,
    BoundaryId,
};
use super::errors::{
    DecoderKind,
    QecError,
    QecResult,
    ResourceKind,
};
use super::limits::QecLimits;

// ============================================================================
// Compatibility / representation constants
// ============================================================================

/// Maximum number of active detection events supported by the exact solver.
///
/// This is an algorithmic safety bound, not the global QEC graph limit.
pub const MAX_MWPM_EVENTS: usize = 24;

/// Maximum graph nodes inspected by the exact decoder when using the default
/// decoder policy.
///
/// The authoritative graph limit remains [`QecLimits`].
pub const MAX_MWPM_GRAPH_NODES: usize = 4_096;

/// Maximum graph edges inspected by the exact decoder when using the default
/// decoder policy.
///
/// The authoritative graph limit remains [`QecLimits`].
pub const MAX_MWPM_GRAPH_EDGES: usize = 32_768;

/// Maximum shortest-path relaxations performed by the default exact decoder.
pub const MAX_SHORTEST_PATH_RELAXATIONS: usize = 10_000_000;

/// Maximum correction-path edges materialized by the default decoder.
pub const MAX_CORRECTION_PATH_EDGES: usize = 100_000;

/// Internal representation of infinity.
const INF: u64 = u64::MAX;

/// Conservative memory estimate for one metric-closure distance.
const ESTIMATED_DISTANCE_BYTES: u64 = 8;

/// Conservative memory estimate for one predecessor entry.
const ESTIMATED_PREDECESSOR_BYTES: u64 = 16;

// ============================================================================
// Matching endpoint
// ============================================================================

/// Endpoint participating in an MWPM match.
///
/// Detection events are physical syndrome defects.
///
/// Boundaries are virtual endpoints representing a correction terminating at a
/// physical or logical boundary.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum MatchingEndpoint {
    Detection(MatchingEvent),
    Boundary(BoundaryId),
}

impl MatchingEndpoint {
    /// Returns true when this is a detection event.
    pub const fn is_detection(self) -> bool {
        matches!(self, Self::Detection(_))
    }

    /// Returns true when this is a boundary.
    pub const fn is_boundary(self) -> bool {
        matches!(self, Self::Boundary(_))
    }

    /// Returns the detection event when applicable.
    pub const fn detection(self) -> Option<MatchingEvent> {
        match self {
            Self::Detection(event) => Some(event),
            Self::Boundary(_) => None,
        }
    }

    /// Returns the boundary identifier when applicable.
    pub const fn boundary(self) -> Option<BoundaryId> {
        match self {
            Self::Detection(_) => None,
            Self::Boundary(boundary) => Some(boundary),
        }
    }
}

// ============================================================================
// Matching event
// ============================================================================

/// Stable identifier for an active detection event.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct MatchingEvent {
    node: NodeId,
}

impl MatchingEvent {
    /// Creates an event from a graph node.
    pub const fn new(node: NodeId) -> Self {
        Self { node }
    }

    /// Returns the graph node.
    pub const fn node(self) -> NodeId {
        self.node
    }
}

impl fmt::Display for MatchingEvent {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "event({})", self.node)
    }
}

// ============================================================================
// Event ↔ event matching pair
// ============================================================================

/// One detection-event ↔ detection-event match.
///
/// This type is retained for compatibility with callers that previously used
/// `MatchingPair`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct MatchingPair {
    first: MatchingEvent,
    second: MatchingEvent,
    weight: u64,
}

impl MatchingPair {
    /// Creates a canonical event/event pair.
    pub fn new(
        first: MatchingEvent,
        second: MatchingEvent,
        weight: u64,
    ) -> Result<Self, MwpmError> {
        if first == second {
            return Err(MwpmError::SelfMatch { event: first });
        }

        if first > second {
            return Err(MwpmError::NonCanonicalPair);
        }

        Ok(Self {
            first,
            second,
            weight,
        })
    }

    /// Returns the first event.
    pub const fn first(&self) -> MatchingEvent {
        self.first
    }

    /// Returns the second event.
    pub const fn second(&self) -> MatchingEvent {
        self.second
    }

    /// Returns the metric weight.
    pub const fn weight(&self) -> u64 {
        self.weight
    }
}

// ============================================================================
// Boundary match
// ============================================================================

/// One detection-event ↔ boundary match.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct BoundaryMatching {
    event: MatchingEvent,
    boundary: BoundaryId,
    weight: u64,
}

impl BoundaryMatching {
    /// Creates a boundary match.
    pub const fn new(
        event: MatchingEvent,
        boundary: BoundaryId,
        weight: u64,
    ) -> Self {
        Self {
            event,
            boundary,
            weight,
        }
    }

    /// Returns the detection event.
    pub const fn event(&self) -> MatchingEvent {
        self.event
    }

    /// Returns the boundary.
    pub const fn boundary(&self) -> BoundaryId {
        self.boundary
    }

    /// Returns the metric weight.
    pub const fn weight(&self) -> u64 {
        self.weight
    }
}

// ============================================================================
// Unified matching
// ============================================================================

/// One complete matching decision.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum Matching {
    EventPair(MatchingPair),
    BoundaryPair(BoundaryMatching),
}

impl Matching {
    /// Returns the matching weight.
    pub const fn weight(self) -> u64 {
        match self {
            Self::EventPair(pair) => pair.weight(),
            Self::BoundaryPair(pair) => pair.weight(),
        }
    }

    /// Returns true when this match terminates on a boundary.
    pub const fn touches_boundary(self) -> bool {
        matches!(self, Self::BoundaryPair(_))
    }
}

// ============================================================================
// Correction path
// ============================================================================

/// Physical graph path implementing one matching decision.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct CorrectionPath {
    matching: Matching,
    edges: Vec<GraphEdge>,
}

impl CorrectionPath {
    fn new(
        matching: Matching,
        edges: Vec<GraphEdge>,
        max_edges: usize,
    ) -> Result<Self, MwpmError> {
        if edges.len() > max_edges {
            return Err(
                MwpmError::CorrectionPathTooLong {
                    requested: edges.len(),
                    limit: max_edges,
                },
            );
        }

        Ok(Self {
            matching,
            edges,
        })
    }

    /// Returns the matching represented by this path.
    pub const fn matching(&self) -> Matching {
        self.matching
    }

    /// Returns the path edges.
    pub fn edges(&self) -> &[GraphEdge] {
        &self.edges
    }

    /// Returns the number of graph edges.
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Returns true when the path contains no graph edges.
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

// ============================================================================
// Decoder termination
// ============================================================================

/// Why an MWPM operation terminated.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum MwpmTermination {
    Completed,
    EmptyInput,
}

// ============================================================================
// Decoder result
// ============================================================================

/// Complete result of one MWPM operation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct MwpmResult {
    matchings: Vec<Matching>,
    paths: Vec<CorrectionPath>,
    total_weight: u64,
    termination: MwpmTermination,
    shortest_path_relaxations: usize,
}

impl MwpmResult {
    fn new(
        matchings: Vec<Matching>,
        paths: Vec<CorrectionPath>,
        total_weight: u64,
        termination: MwpmTermination,
        shortest_path_relaxations: usize,
    ) -> Result<Self, MwpmError> {
        if matchings.len() != paths.len() {
            return Err(
                MwpmError::InternalResultMismatch,
            );
        }

        Ok(Self {
            matchings,
            paths,
            total_weight,
            termination,
            shortest_path_relaxations,
        })
    }

    /// Returns all matching decisions.
    pub fn matchings(&self) -> &[Matching] {
        &self.matchings
    }

    /// Returns event/event pairs only.
    ///
    /// Boundary matches are available through [`Self::boundary_matches`].
    pub fn pairs(&self) -> Vec<MatchingPair> {
        self.matchings
            .iter()
            .filter_map(|matching| match matching {
                Matching::EventPair(pair) => Some(*pair),
                Matching::BoundaryPair(_) => None,
            })
            .collect()
    }

    /// Returns all event/event pairs as a slice when possible.
    ///
    /// This compatibility helper returns a newly allocated vector because
    /// event/event and boundary matches share one result representation.
    pub fn event_pairs(&self) -> Vec<MatchingPair> {
        self.pairs()
    }

    /// Returns all event/boundary matches.
    pub fn boundary_matches(&self) -> Vec<BoundaryMatching> {
        self.matchings
            .iter()
            .filter_map(|matching| match matching {
                Matching::EventPair(_) => None,
                Matching::BoundaryPair(pair) => Some(*pair),
            })
            .collect()
    }

    /// Returns correction paths.
    pub fn paths(&self) -> &[CorrectionPath] {
        &self.paths
    }

    /// Returns total metric weight.
    pub const fn total_weight(&self) -> u64 {
        self.total_weight
    }

    /// Returns number of matching decisions.
    pub fn pair_count(&self) -> usize {
        self.matchings.len()
    }

    /// Returns number of boundary matches.
    pub fn boundary_pair_count(&self) -> usize {
        self.matchings
            .iter()
            .filter(|matching| matching.touches_boundary())
            .count()
    }

    /// Returns whether the operation matched nothing.
    pub fn is_trivial(&self) -> bool {
        self.matchings.is_empty()
    }

    /// Returns termination state.
    pub const fn termination(&self) -> MwpmTermination {
        self.termination
    }

    /// Returns shortest-path relaxation count.
    pub const fn shortest_path_relaxations(&self) -> usize {
        self.shortest_path_relaxations
    }
}

// ============================================================================
// Configuration
// ============================================================================

/// Exact MWPM algorithm configuration.
///
/// Global QEC resource limits remain authoritative. These fields are
/// algorithmic safety ceilings and must not exceed the central policy.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct MwpmConfig {
    max_events: usize,
    max_graph_nodes: usize,
    max_graph_edges: usize,
    max_relaxations: usize,
    max_path_edges: usize,
}

impl MwpmConfig {
    /// Production defaults.
    pub const fn production() -> Self {
        Self {
            max_events: MAX_MWPM_EVENTS,
            max_graph_nodes: MAX_MWPM_GRAPH_NODES,
            max_graph_edges: MAX_MWPM_GRAPH_EDGES,
            max_relaxations: MAX_SHORTEST_PATH_RELAXATIONS,
            max_path_edges: MAX_CORRECTION_PATH_EDGES,
        }
    }

    /// Creates a bounded configuration.
    pub const fn new(
        max_events: usize,
        max_graph_nodes: usize,
        max_graph_edges: usize,
        max_relaxations: usize,
        max_path_edges: usize,
    ) -> Result<Self, MwpmError> {
        if max_events == 0
            || max_events > MAX_MWPM_EVENTS
            || max_graph_nodes == 0
            || max_graph_nodes > MAX_MWPM_GRAPH_NODES
            || max_graph_edges == 0
            || max_graph_edges > MAX_MWPM_GRAPH_EDGES
            || max_relaxations == 0
            || max_relaxations > MAX_SHORTEST_PATH_RELAXATIONS
            || max_path_edges == 0
            || max_path_edges > MAX_CORRECTION_PATH_EDGES
        {
            return Err(MwpmError::InvalidConfiguration);
        }

        Ok(Self {
            max_events,
            max_graph_nodes,
            max_graph_edges,
            max_relaxations,
            max_path_edges,
        })
    }

    pub const fn max_events(self) -> usize {
        self.max_events
    }

    pub const fn max_graph_nodes(self) -> usize {
        self.max_graph_nodes
    }

    pub const fn max_graph_edges(self) -> usize {
        self.max_graph_edges
    }

    pub const fn max_relaxations(self) -> usize {
        self.max_relaxations
    }

    pub const fn max_path_edges(self) -> usize {
        self.max_path_edges
    }

    fn clamp_to_limits(
        self,
        limits: QecLimits,
    ) -> Self {
        Self {
            max_events: self
                .max_events
                .min(limits.max_syndrome_events)
                .min(MAX_MWPM_EVENTS),

            max_graph_nodes: self
                .max_graph_nodes
                .min(limits.max_graph_nodes)
                .min(MAX_MWPM_GRAPH_NODES),

            max_graph_edges: self
                .max_graph_edges
                .min(limits.max_graph_edges)
                .min(MAX_MWPM_GRAPH_EDGES),

            max_relaxations: self
                .max_relaxations
                .min(limits.max_decoder_iterations)
                .min(MAX_SHORTEST_PATH_RELAXATIONS),

            max_path_edges: self
                .max_path_edges
                .min(limits.max_graph_edges)
                .min(MAX_CORRECTION_PATH_EDGES),
        }
    }
}

impl Default for MwpmConfig {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Decoder
// ============================================================================

/// Exact deterministic MWPM decoder.
#[derive(
    Debug,
    Clone,
)]
pub struct MwpmDecoder {
    config: MwpmConfig,
}

impl MwpmDecoder {
    /// Creates the default exact decoder.
    pub const fn new() -> Self {
        Self {
            config: MwpmConfig::production(),
        }
    }

    /// Creates an exact decoder with an algorithm configuration.
    pub const fn with_config(
        config: MwpmConfig,
    ) -> Self {
        Self { config }
    }

    /// Returns configured algorithm limits.
    pub const fn config(&self) -> MwpmConfig {
        self.config
    }

    /// Decodes using the graph's centrally configured `QecLimits`.
    pub fn decode_graph(
        &self,
        graph: &DecodingGraph,
    ) -> Result<MwpmResult, MwpmError> {
        let limits = graph.limits();
        let cancellation = CancellationToken::new();

        self.decode_graph_with_context(
            graph,
            &limits,
            &cancellation,
        )
    }

    /// Decodes with explicit central resource limits and cancellation.
    ///
    /// The supplied limits are intersected with the graph's own policy.
    /// No caller can enlarge the graph's resource policy through this API.
    pub fn decode_graph_with_context(
        &self,
        graph: &DecodingGraph,
        limits: &QecLimits,
        cancellation: &CancellationToken,
    ) -> Result<MwpmResult, MwpmError> {
        limits
            .validate()
            .map_err(MwpmError::Limit)?;

        cancellation
            .check()
            .map_err(MwpmError::Cancellation)?;

        graph
            .validate()
            .map_err(MwpmError::Graph)?;

        let effective_limits = intersect_limits(
            graph.limits(),
            *limits,
        );

        let config = self
            .config
            .clamp_to_limits(effective_limits);

        validate_effective_configuration(
            config,
            effective_limits,
        )?;

        preflight_graph(
            graph,
            effective_limits,
            config,
        )?;

        let events = collect_detection_events(
            graph,
            effective_limits,
            config.max_events(),
            cancellation,
        )?;

        if events.is_empty() {
            return MwpmResult::new(
                Vec::new(),
                Vec::new(),
                0,
                MwpmTermination::EmptyInput,
                0,
            );
        }

        /*
         * Unlike event/event-only MWPM, boundary-aware decoding does not
         * require an even number of detection events.
         *
         * Each event may either:
         *
         *   event ↔ event
         *   event ↔ boundary
         *
         * Therefore an odd event count is valid whenever at least one
         * reachable boundary exists.
         */

        let metric = ShortestPathMetric::build(
            graph,
            &events,
            effective_limits,
            config,
            cancellation,
        )?;

        let matching = solve_exact_mwpm(
            &metric,
            config,
            cancellation,
        )?;

        materialize_result(
            graph,
            &metric,
            matching,
            config,
            cancellation,
        )
    }
}

impl Default for MwpmDecoder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Resource integration
// ============================================================================

fn intersect_limits(
    graph: QecLimits,
    requested: QecLimits,
) -> QecLimits {
    QecLimits {
        max_code_distance: graph
            .max_code_distance
            .min(requested.max_code_distance),

        max_qubits: graph
            .max_qubits
            .min(requested.max_qubits),

        max_stabilizers: graph
            .max_stabilizers
            .min(requested.max_stabilizers),

        max_syndrome_events: graph
            .max_syndrome_events
            .min(requested.max_syndrome_events),

        max_rounds: graph
            .max_rounds
            .min(requested.max_rounds),

        max_graph_nodes: graph
            .max_graph_nodes
            .min(requested.max_graph_nodes),

        max_graph_edges: graph
            .max_graph_edges
            .min(requested.max_graph_edges),

        max_memory_bytes: graph
            .max_memory_bytes
            .min(requested.max_memory_bytes),

        max_decoder_time_ns: graph
            .max_decoder_time_ns
            .min(requested.max_decoder_time_ns),

        max_parallelism: graph
            .max_parallelism
            .min(requested.max_parallelism),

        max_checkpoint_size_bytes: graph
            .max_checkpoint_size_bytes
            .min(requested.max_checkpoint_size_bytes),

        max_partitions: graph
            .max_partitions
            .min(requested.max_partitions),

        max_stream_buffer_events: graph
            .max_stream_buffer_events
            .min(requested.max_stream_buffer_events),

        max_decoder_iterations: graph
            .max_decoder_iterations
            .min(requested.max_decoder_iterations),

        max_stabilizer_weight: graph
            .max_stabilizer_weight
            .min(requested.max_stabilizer_weight),

        max_logical_operator_weight: graph
            .max_logical_operator_weight
            .min(requested.max_logical_operator_weight),

        max_qubits_per_partition: graph
            .max_qubits_per_partition
            .min(requested.max_qubits_per_partition),
    }
}

fn validate_effective_configuration(
    config: MwpmConfig,
    limits: QecLimits,
) -> Result<(), MwpmError> {
    if config.max_events() == 0
        || config.max_events()
            > limits.max_syndrome_events
    {
        return Err(MwpmError::InvalidConfiguration);
    }

    if config.max_graph_nodes() == 0
        || config.max_graph_nodes()
            > limits.max_graph_nodes
    {
        return Err(MwpmError::InvalidConfiguration);
    }

    if config.max_graph_edges() == 0
        || config.max_graph_edges()
            > limits.max_graph_edges
    {
        return Err(MwpmError::InvalidConfiguration);
    }

    if config.max_relaxations() == 0
        || config.max_relaxations()
            > limits.max_decoder_iterations
    {
        return Err(MwpmError::InvalidConfiguration);
    }

    if config.max_path_edges() == 0
        || config.max_path_edges()
            > limits.max_graph_edges
    {
        return Err(MwpmError::InvalidConfiguration);
    }

    Ok(())
}

fn preflight_graph(
    graph: &DecodingGraph,
    limits: QecLimits,
    config: MwpmConfig,
) -> Result<(), MwpmError> {
    if graph.total_node_count()
        > limits.max_graph_nodes
    {
        return Err(MwpmError::ResourceLimit {
            kind: ResourceKind::GraphNodes,
            requested: graph.total_node_count(),
            limit: limits.max_graph_nodes,
        });
    }

    if graph.edge_count()
        > limits.max_graph_edges
    {
        return Err(MwpmError::ResourceLimit {
            kind: ResourceKind::GraphEdges,
            requested: graph.edge_count(),
            limit: limits.max_graph_edges,
        });
    }

    if graph.node_count()
        > config.max_graph_nodes()
    {
        return Err(MwpmError::GraphTooLarge {
            nodes: graph.node_count(),
            limit: config.max_graph_nodes(),
        });
    }

    if graph.edge_count()
        > config.max_graph_edges()
    {
        return Err(
            MwpmError::GraphTooLargeByEdges {
                edges: graph.edge_count(),
                limit: config.max_graph_edges(),
            },
        );
    }

    let events = graph.node_count();

    let metric_cells = events
        .checked_mul(events)
        .ok_or(MwpmError::ArithmeticOverflow)?;

    let metric_bytes = (metric_cells as u64)
        .checked_mul(ESTIMATED_DISTANCE_BYTES)
        .ok_or(MwpmError::ArithmeticOverflow)?;

    if metric_bytes > limits.max_memory_bytes {
        return Err(MwpmError::MemoryLimit {
            requested: metric_bytes,
            limit: limits.max_memory_bytes,
        });
    }

    Ok(())
}

// ============================================================================
// Detection events
// ============================================================================

fn collect_detection_events(
    graph: &DecodingGraph,
    limits: QecLimits,
    algorithm_limit: usize,
    cancellation: &CancellationToken,
) -> Result<Vec<MatchingEvent>, MwpmError> {
    let limit = algorithm_limit
        .min(limits.max_syndrome_events);

    let mut events = Vec::new();

    for node in graph.nodes() {
        cancellation
            .poll()
            .map_err(MwpmError::Cancellation)?;

        if events.len() >= limit {
            return Err(MwpmError::TooManyEvents {
                count: events
                    .len()
                    .saturating_add(1),
                limit,
            });
        }

        events.push(
            MatchingEvent::new(node.id()),
        );
    }

    Ok(events)
}

// ============================================================================
// Shortest-path metric
// ============================================================================

/// Metric closure over active detection events and graph boundaries.
///
/// For each detection event we calculate shortest paths to:
///
/// * every other detection event;
/// * every reachable boundary.
///
/// Paths are reconstructed from predecessor maps only after matching has been
/// selected.
#[derive(Debug, Clone)]
struct ShortestPathMetric {
    events: Vec<MatchingEvent>,

    boundaries: Vec<BoundaryId>,

    distances: Vec<Vec<u64>>,

    boundary_distances: Vec<BTreeMap<BoundaryId, u64>>,

    paths: BTreeMap<(NodeId, NodeId), Vec<GraphEdge>>,

    boundary_paths:
        BTreeMap<(NodeId, BoundaryId), Vec<GraphEdge>>,

    relaxations: usize,
}

impl ShortestPathMetric {
    fn build(
        graph: &DecodingGraph,
        events: &[MatchingEvent],
        limits: QecLimits,
        config: MwpmConfig,
        cancellation: &CancellationToken,
    ) -> Result<Self, MwpmError> {
        let count = events.len();

        let boundaries: Vec<BoundaryId> =
            graph.boundaries()
                .map(|boundary| boundary.id())
                .collect();

        let mut distances =
            vec![vec![INF; count]; count];

        for index in 0..count {
            let row = distances
                .get_mut(index)
                .ok_or(
                    MwpmError::MatchingIndexOutOfRange,
                )?;

            let cell = row
                .get_mut(index)
                .ok_or(
                    MwpmError::MatchingIndexOutOfRange,
                )?;

            *cell = 0;
        }

        let mut boundary_distances =
            vec![BTreeMap::new(); count];

        let mut paths = BTreeMap::new();

        let mut boundary_paths = BTreeMap::new();

        let mut relaxations = 0usize;

        for (index, event) in events.iter().enumerate() {
            cancellation
                .poll()
                .map_err(MwpmError::Cancellation)?;

            let shortest =
                dijkstra_from_event(
                    graph,
                    *event,
                    limits,
                    config,
                    &mut relaxations,
                    cancellation,
                )?;

            for target_index in
                (index + 1)..count
            {
                cancellation
                    .poll()
                    .map_err(MwpmError::Cancellation)?;

                let target =
                    events
                        .get(target_index)
                        .ok_or(
                            MwpmError::MatchingIndexOutOfRange,
                        )?
                        .node();

                let distance =
                    shortest
                        .distances
                        .get(&target)
                        .copied()
                        .unwrap_or(INF);

                distances
                    .get_mut(index)
                    .and_then(|row| {
                        row.get_mut(target_index)
                    })
                    .ok_or(
                        MwpmError::MatchingIndexOutOfRange,
                    )
                    .map(|cell| {
                        *cell = distance;
                    })?;

                distances
                    .get_mut(target_index)
                    .and_then(|row| {
                        row.get_mut(index)
                    })
                    .ok_or(
                        MwpmError::MatchingIndexOutOfRange,
                    )
                    .map(|cell| {
                        *cell = distance;
                    })?;

                if distance != INF {
                    let path =
                        reconstruct_path(
                            graph,
                            event.node(),
                            target,
                            &shortest.predecessors,
                        )?;

                    paths.insert(
                        canonical_node_pair(
                            event.node(),
                            target,
                        ),
                        path,
                    );
                }
            }

            let boundary_map =
                boundary_distances
                    .get_mut(index)
                    .ok_or(
                        MwpmError::MatchingIndexOutOfRange,
                    )?;

            for boundary in &boundaries {
                cancellation
                    .poll()
                    .map_err(MwpmError::Cancellation)?;

                if let Some(&distance) =
                    shortest
                        .boundary_distances
                        .get(boundary)
                {
                    boundary_map.insert(
                        *boundary,
                        distance,
                    );

                    let path =
                        reconstruct_boundary_path(
                            graph,
                            event.node(),
                            *boundary,
                            &shortest.predecessors,
                        )?;

                    boundary_paths.insert(
                        (
                            event.node(),
                            *boundary,
                        ),
                        path,
                    );
                }
            }
        }

        Ok(Self {
            events: events.to_vec(),
            boundaries,
            distances,
            boundary_distances,
            paths,
            boundary_paths,
            relaxations,
        })
    }

    fn event_count(&self) -> usize {
        self.events.len()
    }

    fn boundary_count(&self) -> usize {
        self.boundaries.len()
    }

    fn distance(
        &self,
        first: usize,
        second: usize,
    ) -> Result<u64, MwpmError> {
        self.distances
            .get(first)
            .and_then(|row| row.get(second))
            .copied()
            .ok_or(
                MwpmError::MatchingIndexOutOfRange,
            )
    }

    fn boundary_distance(
        &self,
        event_index: usize,
        boundary_index: usize,
    ) -> Result<u64, MwpmError> {
        let boundary =
            self.boundaries
                .get(boundary_index)
                .copied()
                .ok_or(
                    MwpmError::MatchingIndexOutOfRange,
                )?;

        Ok(self
            .boundary_distances
            .get(event_index)
            .and_then(|map| map.get(&boundary))
            .copied()
            .unwrap_or(INF))
    }

    fn path(
        &self,
        first: MatchingEvent,
        second: MatchingEvent,
    ) -> Option<&[GraphEdge]> {
        self.paths
            .get(&canonical_node_pair(
                first.node(),
                second.node(),
            ))
            .map(Vec::as_slice)
    }

    fn boundary_path(
        &self,
        event: MatchingEvent,
        boundary: BoundaryId,
    ) -> Option<&[GraphEdge]> {
        self.boundary_paths
            .get(&(event.node(), boundary))
            .map(Vec::as_slice)
    }
}

// ============================================================================
// Dijkstra state
// ============================================================================

#[derive(
    Debug,
    Clone,
)]
struct ShortestPathState {
    distances: BTreeMap<GraphEndpoint, u64>,

    predecessors:
        BTreeMap<GraphEndpoint, GraphEndpoint>,

    boundary_distances:
        BTreeMap<BoundaryId, u64>,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
struct QueueEntry {
    distance: u64,
    endpoint: GraphEndpoint,
}

impl Ord for QueueEntry {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| {
                other
                    .endpoint
                    .cmp(&self.endpoint)
            })
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ============================================================================
// Dijkstra
// ============================================================================

fn dijkstra_from_event(
    graph: &DecodingGraph,
    source: MatchingEvent,
    limits: QecLimits,
    config: MwpmConfig,
    relaxations: &mut usize,
    cancellation: &CancellationToken,
) -> Result<ShortestPathState, MwpmError> {
    let source_endpoint =
        GraphEndpoint::Detection(
            source.node(),
        );

    let mut distances =
        BTreeMap::new();

    let mut predecessors =
        BTreeMap::new();

    let mut queue =
        BinaryHeap::new();

    distances.insert(
        source_endpoint,
        0,
    );

    queue.push(
        QueueEntry {
            distance: 0,
            endpoint: source_endpoint,
        },
    );

    while let Some(entry) =
        queue.pop()
    {
        cancellation
            .poll()
            .map_err(MwpmError::Cancellation)?;

        let known =
            distances
                .get(&entry.endpoint)
                .copied()
                .ok_or(
                    MwpmError::InternalDistanceState,
                )?;

        if entry.distance != known {
            continue;
        }

        for edge in graph.incident_edges(
            entry.endpoint,
        ) {
            cancellation
                .poll()
                .map_err(MwpmError::Cancellation)?;

            *relaxations = relaxations
                .checked_add(1)
                .ok_or(
                    MwpmError::ArithmeticOverflow,
                )?;

            if *relaxations
                > config.max_relaxations()
                || *relaxations
                    > limits.max_decoder_iterations
            {
                return Err(
                    MwpmError::RelaxationLimitExceeded {
                        limit: config
                            .max_relaxations()
                            .min(
                                limits
                                    .max_decoder_iterations,
                            ),
                    },
                );
            }

            let neighbour =
                edge.other(
                    entry.endpoint,
                )
                .ok_or(
                    MwpmError::InvalidGraphEndpoint,
                )?;

            let candidate =
                entry
                    .distance
                    .checked_add(
                        edge.weight()
                            .value(),
                    )
                    .ok_or(
                        MwpmError::ArithmeticOverflow,
                    )?;

            let should_update =
                match distances
                    .get(&neighbour)
                {
                    None => true,
                    Some(&current) => {
                        candidate < current
                    }
                };

            if should_update {
                distances.insert(
                    neighbour,
                    candidate,
                );

                predecessors.insert(
                    neighbour,
                    entry.endpoint,
                );

                queue.push(
                    QueueEntry {
                        distance: candidate,
                        endpoint: neighbour,
                    },
                );
            }
        }
    }

    let mut boundary_distances =
        BTreeMap::new();

    for boundary in graph.boundaries() {
        let endpoint =
            GraphEndpoint::Boundary(
                boundary.id(),
            );

        if let Some(&distance) =
            distances.get(&endpoint)
        {
            boundary_distances.insert(
                boundary.id(),
                distance,
            );
        }
    }

    Ok(ShortestPathState {
        distances,
        predecessors,
        boundary_distances,
    })
}

// ============================================================================
// Path reconstruction
// ============================================================================

fn reconstruct_path(
    graph: &DecodingGraph,
    source: NodeId,
    target: NodeId,
    predecessors: &BTreeMap<
        GraphEndpoint,
        GraphEndpoint,
    >,
) -> Result<Vec<GraphEdge>, MwpmError> {
    let source_endpoint =
        GraphEndpoint::Detection(
            source,
        );

    let target_endpoint =
        GraphEndpoint::Detection(
            target,
        );

    reconstruct_endpoint_path(
        graph,
        source_endpoint,
        target_endpoint,
        predecessors,
    )
}

fn reconstruct_boundary_path(
    graph: &DecodingGraph,
    source: NodeId,
    boundary: BoundaryId,
    predecessors: &BTreeMap<
        GraphEndpoint,
        GraphEndpoint,
    >,
) -> Result<Vec<GraphEdge>, MwpmError> {
    reconstruct_endpoint_path(
        graph,
        GraphEndpoint::Detection(source),
        GraphEndpoint::Boundary(boundary),
        predecessors,
    )
}

fn reconstruct_endpoint_path(
    graph: &DecodingGraph,
    source: GraphEndpoint,
    target: GraphEndpoint,
    predecessors: &BTreeMap<
        GraphEndpoint,
        GraphEndpoint,
    >,
) -> Result<Vec<GraphEdge>, MwpmError> {
    if source == target {
        return Ok(Vec::new());
    }

    let mut current = target;
    let mut reversed = Vec::new();
    let mut visited = BTreeSet::new();

    while current != source {
        if !visited.insert(current) {
            return Err(MwpmError::PathCycle);
        }

        let predecessor =
            predecessors
                .get(&current)
                .copied()
                .ok_or(
                    MwpmError::UnreachableEndpoint {
                        source,
                        target,
                    },
                )?;

        let edge =
            graph
                .edge(
                    predecessor,
                    current,
                )
                .cloned()
                .ok_or(
                    MwpmError::PathEdgeMissing {
                        first: predecessor,
                        second: current,
                    },
                )?;

        reversed.push(edge);
        current = predecessor;
    }

    reversed.reverse();
    Ok(reversed)
}

// ============================================================================
// Exact MWPM
// ============================================================================

/// Internal dynamic-programming state.
///
/// The state represents the set of unmatched detection events.
///
/// Boundary matches are modelled as terminal choices. A boundary may be used
/// by at most one detection event in a matching state. This prevents two
/// defects from being silently collapsed onto the same virtual boundary
/// endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MatchChoice {
    kind: MatchChoiceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatchChoiceKind {
    Event {
        second: usize,
    },

    Boundary {
        boundary: usize,
    },
}

fn solve_exact_mwpm(
    metric: &ShortestPathMetric,
    config: MwpmConfig,
    cancellation: &CancellationToken,
) -> Result<Vec<Matching>, MwpmError> {
    let count = metric.event_count();

    if count == 0 {
        return Ok(Vec::new());
    }

    if count > config.max_events() {
        return Err(
            MwpmError::TooManyEvents {
                count,
                limit: config.max_events(),
            },
        );
    }

    if count > 63 {
        return Err(MwpmError::EventMaskOverflow);
    }

    let full_mask =
        (1u64 << count) - 1;

    /*
     * Boundary usage is represented by another bit mask.
     *
     * Exact boundary-aware matching therefore has a state:
     *
     *     (unmatched_events, used_boundaries)
     *
     * The default surface-code boundary count is normally tiny. The
     * representation is intentionally bounded to 63 boundaries.
     */
    if metric.boundary_count() > 63 {
        return Err(
            MwpmError::BoundaryMaskOverflow,
        );
    }

    let mut memo =
        BTreeMap::<(u64, u64), u64>::new();

    let mut choices =
        BTreeMap::<
            (u64, u64),
            MatchChoice,
        >::new();

    let total = solve_mask(
        metric,
        full_mask,
        0,
        &mut memo,
        &mut choices,
        cancellation,
    )?;

    if total == INF {
        return Err(
            MwpmError::NoPerfectMatching,
        );
    }

    let mut mask = full_mask;
    let mut used_boundaries = 0u64;

    let mut result =
        Vec::with_capacity(
            count,
        );

    while mask != 0 {
        cancellation
            .poll()
            .map_err(MwpmError::Cancellation)?;

        let first =
            first_set_bit(mask)
                .ok_or(
                    MwpmError::InternalMatchingState,
                )?;

        let state =
            (mask, used_boundaries);

        let choice =
            choices
                .get(&state)
                .copied()
                .ok_or(
                    MwpmError::InternalMatchingState,
                )?;

        match choice.kind {
            MatchChoiceKind::Event {
                second,
            } => {
                let event_first =
                    *metric.events
                        .get(first)
                        .ok_or(
                            MwpmError::MatchingIndexOutOfRange,
                        )?;

                let event_second =
                    *metric.events
                        .get(second)
                        .ok_or(
                            MwpmError::MatchingIndexOutOfRange,
                        )?;

                let weight =
                    metric.distance(
                        first,
                        second,
                    )?;

                if weight == INF {
                    return Err(
                        MwpmError::NoPerfectMatching,
                    );
                }

                let pair =
                    MatchingPair::new(
                        event_first,
                        event_second,
                        weight,
                    )?;

                result.push(
                    Matching::EventPair(pair),
                );

                mask &=
                    !(1u64 << first);

                mask &=
                    !(1u64 << second);
            }

            MatchChoiceKind::Boundary {
                boundary,
            } => {
                let event =
                    *metric.events
                        .get(first)
                        .ok_or(
                            MwpmError::MatchingIndexOutOfRange,
                        )?;

                let boundary_id =
                    *metric.boundaries
                        .get(boundary)
                        .ok_or(
                            MwpmError::MatchingIndexOutOfRange,
                        )?;

                let weight =
                    metric.boundary_distance(
                        first,
                        boundary,
                    )?;

                if weight == INF {
                    return Err(
                        MwpmError::NoPerfectMatching,
                    );
                }

                result.push(
                    Matching::BoundaryPair(
                        BoundaryMatching::new(
                            event,
                            boundary_id,
                            weight,
                        ),
                    ),
                );

                mask &=
                    !(1u64 << first);

                used_boundaries |=
                    1u64 << boundary;
            }
        }
    }

    result.sort_by_key(
        matching_sort_key,
    );

    Ok(result)
}

fn solve_mask(
    metric: &ShortestPathMetric,
    mask: u64,
    used_boundaries: u64,
    memo: &mut BTreeMap<
        (u64, u64),
        u64,
    >,
    choices: &mut BTreeMap<
        (u64, u64),
        MatchChoice,
    >,
    cancellation: &CancellationToken,
) -> Result<u64, MwpmError> {
    cancellation
        .poll()
        .map_err(MwpmError::Cancellation)?;

    if mask == 0 {
        return Ok(0);
    }

    let state =
        (mask, used_boundaries);

    if let Some(&value) =
        memo.get(&state)
    {
        return Ok(value);
    }

    let first =
        first_set_bit(mask)
            .ok_or(
                MwpmError::InternalMatchingState,
            )?;

    let without_first =
        mask & !(1u64 << first);

    let mut best = INF;

    let mut best_choice = None;

    /*
     * Option 1:
     *
     * Match the first event to another event.
     */
    let mut remaining =
        without_first;

    while remaining != 0 {
        cancellation
            .poll()
            .map_err(MwpmError::Cancellation)?;

        let second =
            first_set_bit(
                remaining,
            )
            .ok_or(
                MwpmError::InternalMatchingState,
            )?;

        let weight =
            metric.distance(
                first,
                second,
            )?;

        if weight != INF {
            let remainder =
                solve_mask(
                    metric,
                    without_first
                        & !(1u64 << second),
                    used_boundaries,
                    memo,
                    choices,
                    cancellation,
                )?;

            if remainder != INF {
                let total =
                    weight
                        .checked_add(
                            remainder,
                        )
                        .ok_or(
                            MwpmError::ArithmeticOverflow,
                        )?;

                let candidate =
                    MatchChoice {
                        kind:
                            MatchChoiceKind::Event {
                                second,
                            },
                    };

                if is_better_choice(
                    total,
                    candidate,
                    best,
                    best_choice,
                ) {
                    best = total;
                    best_choice =
                        Some(candidate);
                }
            }
        }

        remaining &=
            !(1u64 << second);
    }

    /*
     * Option 2:
     *
     * Match the first event to one unused boundary.
     */
    for boundary_index in
        0..metric.boundary_count()
    {
        cancellation
            .poll()
            .map_err(MwpmError::Cancellation)?;

        let bit =
            1u64 << boundary_index;

        if used_boundaries & bit != 0 {
            continue;
        }

        let weight =
            metric.boundary_distance(
                first,
                boundary_index,
            )?;

        if weight == INF {
            continue;
        }

        let remainder =
            solve_mask(
                metric,
                without_first,
                used_boundaries | bit,
                memo,
                choices,
                cancellation,
            )?;

        if remainder == INF {
            continue;
        }

        let total =
            weight
                .checked_add(
                    remainder,
                )
                .ok_or(
                    MwpmError::ArithmeticOverflow,
                )?;

        let candidate =
            MatchChoice {
                kind:
                    MatchChoiceKind::Boundary {
                        boundary:
                            boundary_index,
                    },
            };

        if is_better_choice(
            total,
            candidate,
            best,
            best_choice,
        ) {
            best = total;
            best_choice =
                Some(candidate);
        }
    }

    if let Some(choice) =
        best_choice
    {
        choices.insert(
            state,
            choice,
        );
    }

    memo.insert(
        state,
        best,
    );

    Ok(best)
}

fn is_better_choice(
    total: u64,
    candidate: MatchChoice,
    current_best: u64,
    current_choice: Option<MatchChoice>,
) -> bool {
    if total < current_best {
        return true;
    }

    if total > current_best {
        return false;
    }

    match (candidate.kind, current_choice) {
        (
            MatchChoiceKind::Event {
                second: candidate_second,
            },
            Some(MatchChoice {
                kind:
                    MatchChoiceKind::Event {
                        second:
                            current_second,
                    },
            }),
        ) => candidate_second < current_second,

        (
            MatchChoiceKind::Boundary {
                boundary:
                    candidate_boundary,
            },
            Some(MatchChoice {
                kind:
                    MatchChoiceKind::Boundary {
                        boundary:
                            current_boundary,
                    },
            }),
        ) => candidate_boundary < current_boundary,

        /*
         * Deterministically prefer an event/event match over a boundary match
         * when both have exactly equal metric weight.
         *
         * This avoids unnecessary boundary consumption.
         */
        (
            MatchChoiceKind::Event { .. },
            Some(MatchChoice {
                kind:
                    MatchChoiceKind::Boundary {
                        ..
                    },
            }),
        ) => true,

        _ => false,
    }
}

fn matching_sort_key(
    matching: &Matching,
) -> (u8, usize, usize) {
    match matching {
        Matching::EventPair(pair) => (
            0,
            pair.first().node().index(),
            pair.second().node().index(),
        ),

        Matching::BoundaryPair(pair) => (
            1,
            pair.event().node().index(),
            pair.boundary().index(),
        ),
    }
}

fn first_set_bit(
    mask: u64,
) -> Option<usize> {
    if mask == 0 {
        None
    } else {
        Some(
            mask.trailing_zeros()
                as usize,
        )
    }
}

// ============================================================================
// Result materialization
// ============================================================================

fn materialize_result(
    graph: &DecodingGraph,
    metric: &ShortestPathMetric,
    matchings: Vec<Matching>,
    config: MwpmConfig,
    cancellation: &CancellationToken,
) -> Result<MwpmResult, MwpmError> {
    let mut total_weight = 0u64;

    let mut paths =
        Vec::with_capacity(
            matchings.len(),
        );

    let mut total_path_edges = 0usize;

    for matching in &matchings {
        cancellation
            .poll()
            .map_err(MwpmError::Cancellation)?;

        total_weight =
            total_weight
                .checked_add(
                    matching.weight(),
                )
                .ok_or(
                    MwpmError::ArithmeticOverflow,
                )?;

        let edges =
            match matching {
                Matching::EventPair(pair) => {
                    metric
                        .path(
                            pair.first(),
                            pair.second(),
                        )
                        .ok_or(
                            MwpmError::UnreachablePair {
                                first:
                                    pair.first()
                                        .node(),
                                second:
                                    pair.second()
                                        .node(),
                            },
                        )?
                }

                Matching::BoundaryPair(pair) => {
                    metric
                        .boundary_path(
                            pair.event(),
                            pair.boundary(),
                        )
                        .ok_or(
                            MwpmError::UnreachableBoundary {
                                event:
                                    pair.event()
                                        .node(),
                                boundary:
                                    pair.boundary(),
                            },
                        )?
                }
            };

        total_path_edges =
            total_path_edges
                .checked_add(
                    edges.len(),
                )
                .ok_or(
                    MwpmError::ArithmeticOverflow,
                )?;

        if total_path_edges
            > config.max_path_edges()
        {
            return Err(
                MwpmError::CorrectionPathTooLong {
                    requested:
                        total_path_edges,
                    limit:
                        config.max_path_edges(),
                },
            );
        }

        paths.push(
            CorrectionPath::new(
                *matching,
                edges.to_vec(),
                config.max_path_edges(),
            )?,
        );
    }

    graph
        .validate()
        .map_err(MwpmError::Graph)?;

    MwpmResult::new(
        matchings,
        paths,
        total_weight,
        MwpmTermination::Completed,
        metric.relaxations,
    )
}

// ============================================================================
// Helpers
// ============================================================================

fn canonical_node_pair(
    first: NodeId,
    second: NodeId,
) -> (NodeId, NodeId) {
    if first <= second {
        (first, second)
    } else {
        (second, first)
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the MWPM subsystem.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum MwpmError {
    Graph(
        super::decoding_graph::DecodingGraphError,
    ),

    Limit(
        super::limits::LimitError,
    ),

    Cancellation(QecError),

    ResourceLimit {
        kind: ResourceKind,
        requested: usize,
        limit: usize,
    },

    MemoryLimit {
        requested: u64,
        limit: u64,
    },

    GraphTooLarge {
        nodes: usize,
        limit: usize,
    },

    GraphTooLargeByEdges {
        edges: usize,
        limit: usize,
    },

    TooManyEvents {
        count: usize,
        limit: usize,
    },

    OddDetectionEventCount {
        count: usize,
    },

    SelfMatch {
        event: MatchingEvent,
    },

    NonCanonicalPair,

    MatchingIndexOutOfRange,

    EventMaskOverflow,

    BoundaryMaskOverflow,

    NoPerfectMatching,

    RelaxationLimitExceeded {
        limit: usize,
    },

    ArithmeticOverflow,

    MemoryEstimateOverflow,

    InternalDistanceState,

    InternalMatchingState,

    InternalResultMismatch,

    PathCycle,

    UnreachablePair {
        first: NodeId,
        second: NodeId,
    },

    UnreachableBoundary {
        event: NodeId,
        boundary: BoundaryId,
    },

    UnreachableEndpoint {
        source: GraphEndpoint,
        target: GraphEndpoint,
    },

    PathEdgeMissing {
        first: GraphEndpoint,
        second: GraphEndpoint,
    },

    InvalidGraphEndpoint,

    CorrectionPathTooLong {
        requested: usize,
        limit: usize,
    },

    InvalidConfiguration,
}

impl MwpmError {
    /// Converts the decoder error to the canonical QEC error.
    pub fn into_qec_error(self) -> QecError {
        match self {
            Self::Graph(error) => {
                QecError::invalid_graph(
                    error.to_string(),
                )
            }

            Self::Limit(error) => {
                QecError::resource_limit(
                    ResourceKind::Custom,
                    0,
                    0,
                    error.to_string(),
                )
            }

            Self::Cancellation(error) => error,

            Self::ResourceLimit {
                kind,
                requested,
                limit,
            } => QecError::resource_limit(
                kind,
                requested as u128,
                limit as u128,
                format!(
                    "MWPM resource limit exceeded: requested {requested}, limit {limit}"
                ),
            ),

            Self::MemoryLimit {
                requested,
                limit,
            } => QecError::memory_limit(
                requested,
                limit,
                format!(
                    "MWPM memory preflight requires {requested} bytes but the limit is {limit} bytes"
                ),
            ),

            Self::GraphTooLarge {
                nodes,
                limit,
            } => QecError::resource_limit(
                ResourceKind::GraphNodes,
                nodes as u128,
                limit as u128,
                format!(
                    "MWPM graph-node budget exceeded: {nodes} > {limit}"
                ),
            ),

            Self::GraphTooLargeByEdges {
                edges,
                limit,
            } => QecError::resource_limit(
                ResourceKind::GraphEdges,
                edges as u128,
                limit as u128,
                format!(
                    "MWPM graph-edge budget exceeded: {edges} > {limit}"
                ),
            ),

            Self::TooManyEvents {
                count,
                limit,
            } => QecError::resource_limit(
                ResourceKind::SyndromeEvents,
                count as u128,
                limit as u128,
                format!(
                    "MWPM active-event budget exceeded: {count} > {limit}"
                ),
            ),

            Self::OddDetectionEventCount {
                count,
            } => QecError::decoder_failure(
                DecoderKind::Mwpm,
                format!(
                    "odd detection-event count {count} cannot be decoded without a reachable boundary"
                ),
            ),

            Self::NoPerfectMatching => {
                QecError::decoder_failure(
                    DecoderKind::Mwpm,
                    "no valid MWPM solution exists for the supplied graph",
                )
            }

            Self::RelaxationLimitExceeded {
                limit,
            } => QecError::resource_limit(
                ResourceKind::DecoderIterations,
                (limit as u128).saturating_add(1),
                limit as u128,
                format!(
                    "MWPM shortest-path work exceeded {limit} relaxations"
                ),
            ),

            Self::ArithmeticOverflow => {
                QecError::numerical_failure(
                    super::errors::NumericalOperation::Accumulation,
                    "MWPM arithmetic overflow",
                )
            }

            Self::MemoryEstimateOverflow => {
                QecError::numerical_failure(
                    super::errors::NumericalOperation::IntegerConversion,
                    "MWPM memory estimate overflow",
                )
            }

            Self::Cancellation(_) => unreachable!(
                "handled above"
            ),

            Self::InvalidConfiguration => {
                QecError::unsupported(
                    "mwpm_configuration",
                    "the requested MWPM configuration is incompatible with the active QEC resource policy",
                )
            }

            Self::BoundaryMaskOverflow => {
                QecError::resource_limit(
                    ResourceKind::Custom,
                    64,
                    63,
                    "too many virtual boundaries for the exact bounded MWPM representation",
                )
            }

            Self::EventMaskOverflow => {
                QecError::resource_limit(
                    ResourceKind::SyndromeEvents,
                    64,
                    63,
                    "too many active events for the exact bounded MWPM representation",
                )
            }

            Self::SelfMatch { event } => {
                QecError::invalid_graph(
                    format!(
                        "MWPM attempted to self-match {event}"
                    ),
                )
            }

            Self::NonCanonicalPair => {
                QecError::invalid_input(
                    "MWPM pair is not canonically ordered",
                )
            }

            Self::MatchingIndexOutOfRange => {
                QecError::invariant(
                    "mwpm_matching_index",
                    "MWPM referenced an event outside its metric closure",
                )
            }

            Self::InternalDistanceState => {
                QecError::invariant(
                    "mwpm_distance_state",
                    "MWPM shortest-path state became inconsistent",
                )
            }

            Self::InternalMatchingState => {
                QecError::invariant(
                    "mwpm_matching_state",
                    "MWPM dynamic-programming state became inconsistent",
                )
            }

            Self::InternalResultMismatch => {
                QecError::invariant(
                    "mwpm_result",
                    "MWPM produced different numbers of matches and paths",
                )
            }

            Self::PathCycle => {
                QecError::invariant(
                    "mwpm_path",
                    "MWPM path reconstruction encountered a cycle",
                )
            }

            Self::UnreachablePair {
                first,
                second,
            } => QecError::decoder_failure(
                DecoderKind::Mwpm,
                format!(
                    "no path exists between {first} and {second}"
                ),
            ),

            Self::UnreachableBoundary {
                event,
                boundary,
            } => QecError::decoder_failure(
                DecoderKind::Mwpm,
                format!(
                    "no correction path exists from {event} to boundary {boundary}"
                ),
            ),

            Self::UnreachableEndpoint {
                source,
                target,
            } => QecError::decoder_failure(
                DecoderKind::Mwpm,
                format!(
                    "no path exists from {source:?} to {target:?}"
                ),
            ),

            Self::PathEdgeMissing {
                first,
                second,
            } => QecError::invariant(
                "mwpm_predecessor_edge",
                format!(
                    "predecessor edge missing between {first:?} and {second:?}"
                ),
            ),

            Self::InvalidGraphEndpoint => {
                QecError::invalid_graph(
                    "MWPM encountered an invalid graph endpoint",
                )
            }

            Self::CorrectionPathTooLong {
                requested,
                limit,
            } => QecError::resource_limit(
                ResourceKind::GraphEdges,
                requested as u128,
                limit as u128,
                format!(
                    "MWPM correction path contains {requested} edges, limit {limit}"
                ),
            ),
        }
    }
}

impl fmt::Display for MwpmError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Graph(error) => {
                write!(
                    formatter,
                    "decoding graph validation failed: {error}"
                )
            }

            Self::Limit(error) => {
                write!(
                    formatter,
                    "QEC resource policy rejected MWPM: {error}"
                )
            }

            Self::Cancellation(error) => {
                write!(
                    formatter,
                    "MWPM cancelled: {error}"
                )
            }

            Self::ResourceLimit {
                kind,
                requested,
                limit,
            } => {
                write!(
                    formatter,
                    "MWPM resource limit exceeded for {}: {} > {}",
                    kind.as_str(),
                    requested,
                    limit
                )
            }

            Self::MemoryLimit {
                requested,
                limit,
            } => {
                write!(
                    formatter,
                    "MWPM memory limit exceeded: {requested} > {limit} bytes"
                )
            }

            Self::GraphTooLarge {
                nodes,
                limit,
            } => {
                write!(
                    formatter,
                    "MWPM graph contains {nodes} nodes, limit {limit}"
                )
            }

            Self::GraphTooLargeByEdges {
                edges,
                limit,
            } => {
                write!(
                    formatter,
                    "MWPM graph contains {edges} edges, limit {limit}"
                )
            }

            Self::TooManyEvents {
                count,
                limit,
            } => {
                write!(
                    formatter,
                    "MWPM received {count} detection events, limit {limit}"
                )
            }

            Self::OddDetectionEventCount {
                count,
            } => {
                write!(
                    formatter,
                    "MWPM received odd detection-event count {count}"
                )
            }

            Self::SelfMatch { event } => {
                write!(
                    formatter,
                    "MWPM cannot self-match {event}"
                )
            }

            Self::NonCanonicalPair => {
                write!(
                    formatter,
                    "MWPM pair is not in canonical order"
                )
            }

            Self::MatchingIndexOutOfRange => {
                write!(
                    formatter,
                    "MWPM matching index is outside the active metric closure"
                )
            }

            Self::EventMaskOverflow => {
                write!(
                    formatter,
                    "MWPM event mask exceeds its bounded representation"
                )
            }

            Self::BoundaryMaskOverflow => {
                write!(
                    formatter,
                    "MWPM boundary mask exceeds its bounded representation"
                )
            }

            Self::NoPerfectMatching => {
                write!(
                    formatter,
                    "no valid MWPM solution exists"
                )
            }

            Self::RelaxationLimitExceeded { limit } => {
                write!(
                    formatter,
                    "MWPM shortest-path relaxation limit {limit} exceeded"
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    formatter,
                    "MWPM arithmetic overflow"
                )
            }

            Self::MemoryEstimateOverflow => {
                write!(
                    formatter,
                    "MWPM memory estimate overflow"
                )
            }

            Self::InternalDistanceState => {
                write!(
                    formatter,
                    "MWPM shortest-path state is inconsistent"
                )
            }

            Self::InternalMatchingState => {
                write!(
                    formatter,
                    "MWPM matching state is inconsistent"
                )
            }

            Self::InternalResultMismatch => {
                write!(
                    formatter,
                    "MWPM result contains inconsistent match/path counts"
                )
            }

            Self::PathCycle => {
                write!(
                    formatter,
                    "MWPM path reconstruction encountered a cycle"
                )
            }

            Self::UnreachablePair {
                first,
                second,
            } => {
                write!(
                    formatter,
                    "no path exists between {first} and {second}"
                )
            }

            Self::UnreachableBoundary {
                event,
                boundary,
            } => {
                write!(
                    formatter,
                    "no path exists from {event} to boundary {boundary}"
                )
            }

            Self::UnreachableEndpoint {
                source,
                target,
            } => {
                write!(
                    formatter,
                    "no path exists from {source:?} to {target:?}"
                )
            }

            Self::PathEdgeMissing {
                first,
                second,
            } => {
                write!(
                    formatter,
                    "predecessor edge missing between {first:?} and {second:?}"
                )
            }

            Self::InvalidGraphEndpoint => {
                write!(
                    formatter,
                    "invalid graph endpoint"
                )
            }

            Self::CorrectionPathTooLong {
                requested,
                limit,
            } => {
                write!(
                    formatter,
                    "correction path has {requested} edges, limit {limit}"
                )
            }

            Self::InvalidConfiguration => {
                write!(
                    formatter,
                    "invalid MWPM configuration"
                )
            }
        }
    }
}

impl std::error::Error for MwpmError {}

impl From<MwpmError> for QecError {
    fn from(error: MwpmError) -> Self {
        error.into_qec_error()
    }
}

/// Canonical-result convenience API.
pub fn decode(
    graph: &DecodingGraph,
) -> QecResult<MwpmResult> {
    MwpmDecoder::new()
        .decode_graph(graph)
        .map_err(Into::into)
}

/// Canonical-result API with explicit execution context.
pub fn decode_with_context(
    graph: &DecodingGraph,
    limits: &QecLimits,
    cancellation: &CancellationToken,
) -> QecResult<MwpmResult> {
    MwpmDecoder::new()
        .decode_graph_with_context(
            graph,
            limits,
            cancellation,
        )
        .map_err(Into::into)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::decoding_graph::{
        EdgeKind,
        EdgeWeight,
        SpatialCoordinate,
        SpaceTimeCoordinate,
    };

    use super::super::syndrome::{
        MeasurementConfidence,
        MeasurementRound,
        StabilizerId,
    };

    fn coordinate(
        x: i64,
    ) -> SpaceTimeCoordinate {
        SpaceTimeCoordinate::new(
            SpatialCoordinate::xy(x, 0)
                .expect("test coordinate"),
            MeasurementRound::new(0)
                .expect("test round"),
        )
        .expect("test coordinate")
    }

    fn graph_with_line(
        count: usize,
    ) -> DecodingGraph {
        let mut graph =
            DecodingGraph::new();

        let mut nodes =
            Vec::with_capacity(count);

        for index in 0..count {
            let node =
                graph
                    .add_detection_node(
                        coordinate(index as i64),
                        StabilizerId::new(index),
                        MeasurementConfidence::certain(),
                    )
                    .expect("test graph");

            nodes.push(node);
        }

        for index in
            0..count.saturating_sub(1)
        {
            graph
                .add_edge(
                    GraphEndpoint::Detection(
                        nodes[index],
                    ),
                    GraphEndpoint::Detection(
                        nodes[index + 1],
                    ),
                    EdgeWeight::new(1)
                        .expect("test weight"),
                    EdgeKind::Spatial,
                )
                .expect("test edge");
        }

        graph
    }

    #[test]
    fn empty_graph_is_trivial() {
        let graph =
            DecodingGraph::new();

        let result =
            MwpmDecoder::new()
                .decode_graph(&graph)
                .expect("empty graph");

        assert!(result.is_trivial());
        assert_eq!(
            result.pair_count(),
            0
        );
        assert_eq!(
            result.total_weight(),
            0
        );
        assert_eq!(
            result.termination(),
            MwpmTermination::EmptyInput
        );
    }

    #[test]
    fn two_events_are_matched() {
        let graph =
            graph_with_line(2);

        let result =
            MwpmDecoder::new()
                .decode_graph(&graph)
                .expect("two-event graph");

        assert_eq!(
            result.pair_count(),
            1
        );

        assert_eq!(
            result.total_weight(),
            1
        );

        let pairs =
            result.pairs();

        let pair =
            pairs
                .first()
                .copied()
                .expect("one event pair");

        assert_eq!(
            pair.first().node(),
            NodeId::new(0)
        );

        assert_eq!(
            pair.second().node(),
            NodeId::new(1)
        );
    }

    #[test]
    fn shortest_path_is_used() {
        let mut graph =
            DecodingGraph::new();

        let a =
            graph
                .add_detection_node(
                    coordinate(0),
                    StabilizerId::new(0),
                    MeasurementConfidence::certain(),
                )
                .expect("test node");

        let b =
            graph
                .add_detection_node(
                    coordinate(1),
                    StabilizerId::new(1),
                    MeasurementConfidence::certain(),
                )
                .expect("test node");

        let c =
            graph
                .add_detection_node(
                    coordinate(2),
                    StabilizerId::new(2),
                    MeasurementConfidence::certain(),
                )
                .expect("test node");

        graph
            .add_edge(
                GraphEndpoint::Detection(a),
                GraphEndpoint::Detection(b),
                EdgeWeight::new(10)
                    .expect("test weight"),
                EdgeKind::Spatial,
            )
            .expect("test edge");

        graph
            .add_edge(
                GraphEndpoint::Detection(a),
                GraphEndpoint::Detection(c),
                EdgeWeight::new(2)
                    .expect("test weight"),
                EdgeKind::Spatial,
            )
            .expect("test edge");

        graph
            .add_edge(
                GraphEndpoint::Detection(c),
                GraphEndpoint::Detection(b),
                EdgeWeight::new(2)
                    .expect("test weight"),
                EdgeKind::Spatial,
            )
            .expect("test edge");

        let result =
            MwpmDecoder::new()
                .decode_graph(&graph)
                .expect("shortest path");

        assert_eq!(
            result.total_weight(),
            4
        );

        assert_eq!(
            result.paths()[0].len(),
            2
        );
    }

    #[test]
    fn boundary_matching_supports_odd_event_count() {
        let mut graph =
            graph_with_line(1);

        let boundary =
            graph
                .add_boundary(
                    coordinate(2),
                )
                .expect("boundary");

        graph
            .add_edge(
                GraphEndpoint::Detection(
                    NodeId::new(0),
                ),
                GraphEndpoint::Boundary(
                    boundary,
                ),
                EdgeWeight::new(3)
                    .expect("weight"),
                EdgeKind::Boundary,
            )
            .expect("boundary edge");

        let result =
            MwpmDecoder::new()
                .decode_graph(&graph)
                .expect("boundary matching");

        assert_eq!(
            result.pair_count(),
            1
        );

        assert_eq!(
            result.boundary_pair_count(),
            1
        );

        assert_eq!(
            result.total_weight(),
            3
        );

        assert_eq!(
            result.paths()[0].len(),
            1
        );
    }

    #[test]
    fn boundary_matching_can_beat_event_pairing() {
        let mut graph =
            graph_with_line(2);

        let boundary =
            graph
                .add_boundary(
                    coordinate(10),
                )
                .expect("boundary");

        graph
            .add_edge(
                GraphEndpoint::Detection(
                    NodeId::new(0),
                ),
                GraphEndpoint::Boundary(
                    boundary,
                ),
                EdgeWeight::new(1)
                    .expect("weight"),
                EdgeKind::Boundary,
            )
            .expect("boundary edge");

        graph
            .add_edge(
                GraphEndpoint::Detection(
                    NodeId::new(1),
                ),
                GraphEndpoint::Boundary(
                    boundary,
                ),
                EdgeWeight::new(1)
                    .expect("weight"),
                EdgeKind::Boundary,
            )
            .expect("boundary edge");

        /*
         * The single boundary is deliberately exclusive.
         *
         * Therefore the decoder cannot consume it twice. It must use the
         * event/event solution if that is the only valid complete solution.
         */
        let result =
            MwpmDecoder::new()
                .decode_graph(&graph)
                .expect("matching");

        assert_eq!(
            result.pair_count(),
            1
        );

        assert_eq!(
            result.boundary_pair_count(),
            0
        );
    }

    #[test]
    fn disconnected_events_without_boundary_fail() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_detection_node(
                coordinate(0),
                StabilizerId::new(0),
                MeasurementConfidence::certain(),
            )
            .expect("node");

        graph
            .add_detection_node(
                coordinate(1),
                StabilizerId::new(1),
                MeasurementConfidence::certain(),
            )
            .expect("node");

        let result =
            MwpmDecoder::new()
                .decode_graph(&graph);

        assert_eq!(
            result,
            Err(
                MwpmError::NoPerfectMatching
            )
        );
    }

    #[test]
    fn boundary_edges_are_not_rejected() {
        let mut graph =
            graph_with_line(1);

        let boundary =
            graph
                .add_boundary(
                    coordinate(2),
                )
                .expect("boundary");

        graph
            .add_edge(
                GraphEndpoint::Detection(
                    NodeId::new(0),
                ),
                GraphEndpoint::Boundary(
                    boundary,
                ),
                EdgeWeight::new(2)
                    .expect("weight"),
                EdgeKind::Boundary,
            )
            .expect("boundary edge");

        graph
            .validate()
            .expect("valid boundary graph");
    }

    #[test]
    fn deterministic_tie_breaking_is_stable() {
        let mut graph =
            DecodingGraph::new();

        let mut nodes =
            Vec::new();

        for index in 0..4 {
            nodes.push(
                graph
                    .add_detection_node(
                        coordinate(index as i64),
                        StabilizerId::new(index),
                        MeasurementConfidence::certain(),
                    )
                    .expect("node"),
            );
        }

        for first in 0..4 {
            for second in
                (first + 1)..4
            {
                graph
                    .add_edge(
                        GraphEndpoint::Detection(
                            nodes[first],
                        ),
                        GraphEndpoint::Detection(
                            nodes[second],
                        ),
                        EdgeWeight::new(1)
                            .expect("weight"),
                        EdgeKind::Custom,
                    )
                    .expect("edge");
            }
        }

        let decoder =
            MwpmDecoder::new();

        let first =
            decoder
                .decode_graph(&graph)
                .expect("first decode");

        let second =
            decoder
                .decode_graph(&graph)
                .expect("second decode");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn cancellation_is_honoured() {
        let (
            source,
            token,
        ) = super::super::cancellation::CancellationSource::new_pair();

        source.cancel();

        let graph =
            graph_with_line(2);

        let result =
            MwpmDecoder::new()
                .decode_graph_with_context(
                    &graph,
                    &graph.limits(),
                    &token,
                );

        assert!(matches!(
            result,
            Err(MwpmError::Cancellation(_))
        ));
    }

    #[test]
    fn central_limits_are_used() {
        let graph =
            graph_with_line(4);

        let mut limits =
            graph.limits();

        limits.max_syndrome_events = 2;

        let result =
            MwpmDecoder::new()
                .decode_graph_with_context(
                    &graph,
                    &limits,
                    &CancellationToken::new(),
                );

        assert!(matches!(
            result,
            Err(
                MwpmError::TooManyEvents {
                    ..
                }
            )
        ));
    }

    #[test]
    fn algorithm_configuration_cannot_escape_central_limits() {
        let graph =
            graph_with_line(4);

        let mut limits =
            graph.limits();

        limits.max_decoder_iterations = 1;

        let config =
            MwpmConfig::new(
                MAX_MWPM_EVENTS,
                MAX_MWPM_GRAPH_NODES,
                MAX_MWPM_GRAPH_EDGES,
                MAX_SHORTEST_PATH_RELAXATIONS,
                MAX_CORRECTION_PATH_EDGES,
            )
            .expect("valid configuration");

        let result =
            MwpmDecoder::with_config(config)
                .decode_graph_with_context(
                    &graph,
                    &limits,
                    &CancellationToken::new(),
                );

        assert!(result.is_err());
    }
}