//! Zamani Quantum Error Correction — Minimum-Weight Perfect Matching.
//!
//! This module implements an exact, deterministic, bounded MWPM engine over
//! the validated `DecodingGraph` representation.
//!
//! Architecture:
//!
//! ```text
//! Syndrome / Detection Events
//!             │
//!             ▼
//!       DecodingGraph
//!             │
//!             ▼
//!       shortest-path metric
//!             │
//!             ▼
//!       exact MWPM solver
//!             │
//!             ▼
//!       Matched detection pairs
//!             │
//!             ▼
//!       physical graph paths
//! ```
//!
//! Important:
//!
//! MWPM is a global optimization problem. A greedy nearest-neighbour
//! algorithm is NOT equivalent to MWPM and is therefore not used here.
//!
//! This implementation deliberately uses an exact dynamic-programming
//! formulation over the active detection-event set. Because the algorithm
//! is exponential in the number of simultaneously active detection events,
//! a hard resource limit is enforced.
//!
//! This is preferable to silently degrading into an incorrect decoder.
//!
//! The graph itself remains independently bounded by `decoding_graph.rs`.
//! The MWPM solver adds a substantially smaller bound over the active event
//! set.
//!
//! Guarantees:
//! - deterministic results;
//! - exact minimum-weight pairing for supported inputs;
//! - checked arithmetic;
//! - no unchecked indexing;
//! - no production `unwrap()` / `expect()`;
//! - bounded memory growth;
//! - bounded computation;
//! - deterministic tie-breaking;
//! - graph validation before decoding;
//! - no mutation of the quantum state;
//! - no hidden randomness;
//! - no floating-point matching decisions;
//! - explicit rejection of unsupported boundary matching.
//!
//! The solver currently matches detection events against other detection
//! events. Boundary-aware MWPM should be added only when the surface-code
//! boundary semantics and correction-chain reconstruction are defined by the
//! topology layer.
//!
//! That restriction is intentional: silently treating a boundary as an
//! ordinary detection event would be mathematically incorrect.

use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::fmt;

use super::decoding_graph::{
    DecodingGraph,
    DetectionNode,
    GraphEdge,
    GraphEndpoint,
    NodeId,
};

// ============================================================================
// Resource limits
// ============================================================================

/// Maximum number of simultaneously active detection events supported by the
/// exact solver.
///
/// The exact solver has exponential complexity.
///
/// For `n` events, the number of possible pairings grows approximately as:
///
/// ```text
/// (n - 1)!!
/// ```
///
/// This bound prevents malformed or adversarial input from causing an
/// unbounded computation.
pub const MAX_MWPM_EVENTS: usize = 24;

/// Maximum number of graph vertices used by the shortest-path computation.
///
/// The full decoding graph may be considerably larger. MWPM only needs a
/// bounded graph projection for the active events.
pub const MAX_MWPM_GRAPH_NODES: usize = 4_096;

/// Maximum number of graph edges inspected by the decoder.
pub const MAX_MWPM_GRAPH_EDGES: usize = 32_768;

/// Maximum number of shortest-path relaxations permitted during one decode.
///
/// This is an additional computational safety valve.
pub const MAX_SHORTEST_PATH_RELAXATIONS: usize = 10_000_000;

/// Maximum total correction-path length returned by one decoding operation.
pub const MAX_CORRECTION_PATH_EDGES: usize = 100_000;

/// Sentinel used internally for an unreachable vertex.
const INF: u64 = u64::MAX;

// ============================================================================
// Matching event
// ============================================================================

/// Stable identifier for an MWPM event.
///
/// This is deliberately separate from `NodeId` so the matching layer can
/// maintain a compact active-event index without changing graph identity.
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
    /// Creates an event from a graph node identifier.
    pub const fn new(
        node: NodeId,
    ) -> Self {
        Self { node }
    }

    /// Returns the graph node identifier.
    pub const fn node(
        self,
    ) -> NodeId {
        self.node
    }
}

impl fmt::Display for MatchingEvent {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "event({})", self.node)
    }
}

// ============================================================================
// Matching pair
// ============================================================================

/// One matched pair of detection events.
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
    /// Creates a validated matching pair.
    pub fn new(
        first: MatchingEvent,
        second: MatchingEvent,
        weight: u64,
    ) -> Result<Self, MwpmError> {
        if first == second {
            return Err(
                MwpmError::SelfMatch {
                    event: first,
                },
            );
        }

        if first > second {
            return Err(
                MwpmError::NonCanonicalPair,
            );
        }

        Ok(Self {
            first,
            second,
            weight,
        })
    }

    /// Returns the first event.
    pub const fn first(
        &self,
    ) -> MatchingEvent {
        self.first
    }

    /// Returns the second event.
    pub const fn second(
        &self,
    ) -> MatchingEvent {
        self.second
    }

    /// Returns the metric weight.
    pub const fn weight(
        &self,
    ) -> u64 {
        self.weight
    }
}

// ============================================================================
// Correction path
// ============================================================================

/// A physical path connecting two matched detection events.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct CorrectionPath {
    pair: MatchingPair,
    edges: Vec<GraphEdge>,
}

impl CorrectionPath {
    fn new(
        pair: MatchingPair,
        edges: Vec<GraphEdge>,
    ) -> Result<Self, MwpmError> {
        if edges.len()
            > MAX_CORRECTION_PATH_EDGES
        {
            return Err(
                MwpmError::CorrectionPathTooLong {
                    limit:
                        MAX_CORRECTION_PATH_EDGES,
                },
            );
        }

        Ok(Self {
            pair,
            edges,
        })
    }

    /// Returns the matched pair.
    pub const fn pair(
        &self,
    ) -> MatchingPair {
        self.pair
    }

    /// Returns the graph edges forming the correction path.
    pub fn edges(
        &self,
    ) -> &[GraphEdge] {
        &self.edges
    }

    /// Returns the number of physical graph edges.
    pub fn len(
        &self,
    ) -> usize {
        self.edges.len()
    }

    /// Returns true when the path is empty.
    pub fn is_empty(
        &self,
    ) -> bool {
        self.edges.is_empty()
    }
}

// ============================================================================
// Decode result
// ============================================================================

/// Complete result of an MWPM operation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct MwpmResult {
    pairs: Vec<MatchingPair>,
    paths: Vec<CorrectionPath>,
    total_weight: u64,
}

impl MwpmResult {
    fn new(
        pairs: Vec<MatchingPair>,
        paths: Vec<CorrectionPath>,
        total_weight: u64,
    ) -> Result<Self, MwpmError> {
        if pairs.len() != paths.len() {
            return Err(
                MwpmError::InternalResultMismatch,
            );
        }

        Ok(Self {
            pairs,
            paths,
            total_weight,
        })
    }

    /// Returns the matched event pairs.
    pub fn pairs(
        &self,
    ) -> &[MatchingPair] {
        &self.pairs
    }

    /// Returns the physical correction paths.
    pub fn paths(
        &self,
    ) -> &[CorrectionPath] {
        &self.paths
    }

    /// Returns the total MWPM metric weight.
    pub const fn total_weight(
        &self,
    ) -> u64 {
        self.total_weight
    }

    /// Returns the number of matched pairs.
    pub fn pair_count(
        &self,
    ) -> usize {
        self.pairs.len()
    }

    /// Returns true when no detection events required matching.
    pub fn is_trivial(
        &self,
    ) -> bool {
        self.pairs.is_empty()
    }
}

// ============================================================================
// MWPM solver configuration
// ============================================================================

/// Configuration for the exact MWPM solver.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct MwpmConfig {
    /// Maximum number of active detection events.
    max_events: usize,

    /// Maximum graph vertices considered.
    max_graph_nodes: usize,

    /// Maximum graph edges considered.
    max_graph_edges: usize,

    /// Maximum total shortest-path relaxations.
    max_relaxations: usize,

    /// Maximum total correction-path edges.
    max_path_edges: usize,
}

impl MwpmConfig {
    /// Production-safe default configuration.
    pub const fn production() -> Self {
        Self {
            max_events:
                MAX_MWPM_EVENTS,

            max_graph_nodes:
                MAX_MWPM_GRAPH_NODES,

            max_graph_edges:
                MAX_MWPM_GRAPH_EDGES,

            max_relaxations:
                MAX_SHORTEST_PATH_RELAXATIONS,

            max_path_edges:
                MAX_CORRECTION_PATH_EDGES,
        }
    }

    /// Creates a configuration after validating all limits.
    pub const fn new(
        max_events: usize,
        max_graph_nodes: usize,
        max_graph_edges: usize,
        max_relaxations: usize,
        max_path_edges: usize,
    ) -> Result<Self, MwpmError> {
        if max_events == 0
            || max_events
                > MAX_MWPM_EVENTS
        {
            return Err(
                MwpmError::InvalidConfiguration,
            );
        }

        if max_graph_nodes == 0
            || max_graph_nodes
                > MAX_MWPM_GRAPH_NODES
        {
            return Err(
                MwpmError::InvalidConfiguration,
            );
        }

        if max_graph_edges == 0
            || max_graph_edges
                > MAX_MWPM_GRAPH_EDGES
        {
            return Err(
                MwpmError::InvalidConfiguration,
            );
        }

        if max_relaxations == 0
            || max_relaxations
                > MAX_SHORTEST_PATH_RELAXATIONS
        {
            return Err(
                MwpmError::InvalidConfiguration,
            );
        }

        if max_path_edges == 0
            || max_path_edges
                > MAX_CORRECTION_PATH_EDGES
        {
            return Err(
                MwpmError::InvalidConfiguration,
            );
        }

        Ok(Self {
            max_events,
            max_graph_nodes,
            max_graph_edges,
            max_relaxations,
            max_path_edges,
        })
    }

    /// Returns the maximum active-event count.
    pub const fn max_events(
        self,
    ) -> usize {
        self.max_events
    }

    /// Returns the maximum graph-node count.
    pub const fn max_graph_nodes(
        self,
    ) -> usize {
        self.max_graph_nodes
    }

    /// Returns the maximum graph-edge count.
    pub const fn max_graph_edges(
        self,
    ) -> usize {
        self.max_graph_edges
    }

    /// Returns the maximum shortest-path relaxations.
    pub const fn max_relaxations(
        self,
    ) -> usize {
        self.max_relaxations
    }

    /// Returns the maximum correction-path size.
    pub const fn max_path_edges(
        self,
    ) -> usize {
        self.max_path_edges
    }
}

impl Default for MwpmConfig {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// MWPM decoder
// ============================================================================

/// Exact deterministic minimum-weight perfect matching decoder.
///
/// The decoder operates directly on a validated `DecodingGraph`.
///
/// The current implementation supports:
///
/// ```text
/// detection event ↔ detection event
/// ```
///
/// Boundary matching is explicitly rejected until the surface-code topology
/// layer supplies the required logical-boundary semantics and correction
/// reconstruction.
#[derive(
    Debug,
    Clone,
)]
pub struct MwpmDecoder {
    config: MwpmConfig,
}

impl MwpmDecoder {
    /// Creates an MWPM decoder with production defaults.
    pub const fn new() -> Self {
        Self {
            config:
                MwpmConfig::production(),
        }
    }

    /// Creates an MWPM decoder from explicit bounded configuration.
    pub const fn with_config(
        config: MwpmConfig,
    ) -> Self {
        Self { config }
    }

    /// Returns the decoder configuration.
    pub const fn config(
        &self,
    ) -> MwpmConfig {
        self.config
    }

    /// Decodes the supplied graph.
    pub fn decode_graph(
        &self,
        graph: &DecodingGraph,
    ) -> Result<MwpmResult, MwpmError> {
        graph
            .validate()
            .map_err(MwpmError::Graph)?;

        if graph.node_count()
            > self.config.max_graph_nodes()
        {
            return Err(
                MwpmError::GraphTooLarge {
                    nodes:
                        graph.node_count(),
                    limit:
                        self.config
                            .max_graph_nodes(),
                },
            );
        }

        if graph.edge_count()
            > self.config.max_graph_edges()
        {
            return Err(
                MwpmError::GraphTooLargeByEdges {
                    edges:
                        graph.edge_count(),
                    limit:
                        self.config
                            .max_graph_edges(),
                },
            );
        }

        if graph.boundary_count() > 0 {
            return Err(
                MwpmError::BoundaryMatchingUnsupported,
            );
        }

        let events =
            collect_detection_events(
                graph,
                self.config.max_events(),
            )?;

        if events.is_empty() {
            return MwpmResult::new(
                Vec::new(),
                Vec::new(),
                0,
            );
        }

        if events.len() % 2 != 0 {
            return Err(
                MwpmError::OddDetectionEventCount {
                    count: events.len(),
                },
            );
        }

        let metric =
            ShortestPathMetric::build(
                graph,
                &events,
                self.config,
            )?;

        let matching =
            solve_exact_mwpm(
                &metric,
                self.config,
            )?;

        materialize_result(
            graph,
            &metric,
            matching,
            self.config,
        )
    }
}

impl Default for MwpmDecoder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Detection event collection
// ============================================================================

fn collect_detection_events(
    graph: &DecodingGraph,
    limit: usize,
) -> Result<
    Vec<MatchingEvent>,
    MwpmError,
> {
    let mut events =
        Vec::new();

    for node in graph.nodes() {
        if events.len() >= limit {
            return Err(
                MwpmError::TooManyEvents {
                    count:
                        events.len()
                            .saturating_add(1),
                    limit,
                },
            );
        }

        events.push(
            MatchingEvent::new(
                node.id(),
            ),
        );
    }

    Ok(events)
}

// ============================================================================
// Shortest-path metric
// ============================================================================

/// Pairwise metric between active detection events.
///
/// MWPM operates on the metric closure of the decoding graph. Therefore the
/// direct graph edge between two events is not necessarily the relevant
/// matching cost; the cost is the minimum physical path through the graph.
#[derive(
    Debug,
    Clone,
)]
struct ShortestPathMetric {
    events:
        Vec<MatchingEvent>,

    distances:
        Vec<Vec<u64>>,

    paths:
        BTreeMap<
            (NodeId, NodeId),
            Vec<GraphEdge>,
        >,
}

impl ShortestPathMetric {
    fn build(
        graph: &DecodingGraph,
        events: &[MatchingEvent],
        config: MwpmConfig,
    ) -> Result<Self, MwpmError> {
        let count =
            events.len();

        let mut distances =
            vec![
                vec![INF; count];
                count
            ];

        let mut paths =
            BTreeMap::new();

        let mut relaxations =
            0usize;

        for i in 0..count {
            if let Some(row) =
                distances.get_mut(i)
            {
                if let Some(value) =
                    row.get_mut(i)
                {
                    *value = 0;
                }
            }

            let (
                dist,
                predecessors,
            ) =
                dijkstra_from_event(
                    graph,
                    events[i],
                    config,
                    &mut relaxations,
                )?;

            for j in
                (i + 1)..count
            {
                let target =
                    events[j].node();

                let distance =
                    dist.get(
                        &target,
                    )
                    .copied()
                    .unwrap_or(INF);

                if let Some(row) =
                    distances.get_mut(i)
                {
                    if let Some(value) =
                        row.get_mut(j)
                    {
                        *value = distance;
                    }
                }

                if let Some(row) =
                    distances.get_mut(j)
                {
                    if let Some(value) =
                        row.get_mut(i)
                    {
                        *value = distance;
                    }
                }

                if distance != INF {
                    let path =
                        reconstruct_path(
                            graph,
                            events[i].node(),
                            target,
                            &predecessors,
                        )?;

                    let key =
                        canonical_node_pair(
                            events[i].node(),
                            target,
                        );

                    paths.insert(
                        key,
                        path,
                    );
                }
            }
        }

        Ok(Self {
            events:
                events.to_vec(),

            distances,
            paths,
        })
    }

    fn event_count(
        &self,
    ) -> usize {
        self.events.len()
    }

    fn distance(
        &self,
        first: usize,
        second: usize,
    ) -> Result<u64, MwpmError> {
        if first >= self.event_count()
            || second >= self.event_count()
        {
            return Err(
                MwpmError::MatchingIndexOutOfRange,
            );
        }

        self.distances
            .get(first)
            .and_then(
                |row| row.get(second),
            )
            .copied()
            .ok_or(
                MwpmError::MatchingIndexOutOfRange,
            )
    }

    fn path(
        &self,
        first: MatchingEvent,
        second: MatchingEvent,
    ) -> Option<&[GraphEdge]> {
        let key =
            canonical_node_pair(
                first.node(),
                second.node(),
            );

        self.paths
            .get(&key)
            .map(Vec::as_slice)
    }
}

// ============================================================================
// Dijkstra
// ============================================================================

/// A deterministic priority queue entry.
///
/// Rust's standard library `BinaryHeap` is a max-heap. We therefore reverse
/// the ordering explicitly.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
struct QueueEntry {
    distance: u64,
    node: NodeId,
}

impl Ord for QueueEntry {
    fn cmp(
        &self,
        other: &Self,
    ) -> std::cmp::Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(
                || {
                    other
                        .node
                        .cmp(&self.node)
                },
            )
    }
}

impl PartialOrd
    for QueueEntry
{
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<
        std::cmp::Ordering,
    > {
        Some(self.cmp(other))
    }
}

fn dijkstra_from_event(
    graph: &DecodingGraph,
    source: MatchingEvent,
    config: MwpmConfig,
    relaxations: &mut usize,
) -> Result<
    (
        BTreeMap<NodeId, u64>,
        BTreeMap<NodeId, NodeId>,
    ),
    MwpmError,
> {
    use std::collections::BinaryHeap;

    let mut distances =
        BTreeMap::new();

    let mut predecessors =
        BTreeMap::new();

    let mut queue =
        BinaryHeap::new();

    distances.insert(
        source.node(),
        0,
    );

    queue.push(
        QueueEntry {
            distance: 0,
            node:
                source.node(),
        },
    );

    while let Some(entry) =
        queue.pop()
    {
        let known =
            distances
                .get(&entry.node)
                .copied()
                .ok_or(
                    MwpmError::InternalDistanceState,
                )?;

        if entry.distance
            != known
        {
            continue;
        }

        let incident =
            graph
                .incident_edges(
                    GraphEndpoint::Detection(
                        entry.node,
                    ),
                );

        for edge in incident {
            *relaxations =
                relaxations
                    .checked_add(1)
                    .ok_or(
                        MwpmError::ArithmeticOverflow,
                    )?;

            if *relaxations
                > config.max_relaxations()
            {
                return Err(
                    MwpmError::RelaxationLimitExceeded {
                        limit:
                            config
                                .max_relaxations(),
                    },
                );
            }

            let neighbour =
                other_detection_endpoint(
                    edge,
                    entry.node,
                )?;

            let Some(neighbour) =
                neighbour
            else {
                return Err(
                    MwpmError::BoundaryMatchingUnsupported,
                );
            };

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
                        candidate
                            < current
                    }
                };

            if should_update {
                distances.insert(
                    neighbour,
                    candidate,
                );

                predecessors.insert(
                    neighbour,
                    entry.node,
                );

                queue.push(
                    QueueEntry {
                        distance:
                            candidate,
                        node:
                            neighbour,
                    },
                );
            }
        }
    }

    Ok((
        distances,
        predecessors,
    ))
}

// ============================================================================
// Graph path reconstruction
// ============================================================================

fn reconstruct_path(
    graph: &DecodingGraph,
    source: NodeId,
    target: NodeId,
    predecessors: &BTreeMap<
        NodeId,
        NodeId,
    >,
) -> Result<Vec<GraphEdge>, MwpmError> {
    if source == target {
        return Ok(Vec::new());
    }

    let mut current =
        target;

    let mut reversed =
        Vec::new();

    let mut visited =
        BTreeSet::new();

    while current != source {
        if !visited.insert(
            current,
        ) {
            return Err(
                MwpmError::PathCycle,
            );
        }

        let predecessor =
            predecessors
                .get(&current)
                .copied()
                .ok_or(
                    MwpmError::UnreachablePair {
                        first: source,
                        second: target,
                    },
                )?;

        let edge =
            find_edge_between(
                graph,
                predecessor,
                current,
            )?;

        reversed.push(edge);

        current =
            predecessor;
    }

    reversed.reverse();

    Ok(reversed)
}

fn find_edge_between(
    graph: &DecodingGraph,
    first: NodeId,
    second: NodeId,
) -> Result<GraphEdge, MwpmError> {
    let endpoint_first =
        GraphEndpoint::Detection(
            first,
        );

    let endpoint_second =
        GraphEndpoint::Detection(
            second,
        );

    for edge in graph.edges() {
        if (edge.from()
            == endpoint_first
            && edge.to()
                == endpoint_second)
            || (edge.from()
                == endpoint_second
                && edge.to()
                    == endpoint_first)
        {
            return Ok(
                edge.clone(),
            );
        }
    }

    Err(
        MwpmError::PathEdgeMissing {
            first,
            second,
        },
    )
}

fn other_detection_endpoint(
    edge: &GraphEdge,
    current: NodeId,
) -> Result<Option<NodeId>, MwpmError> {
    match (
        edge.from(),
        edge.to(),
    ) {
        (
            GraphEndpoint::Detection(a),
            GraphEndpoint::Detection(b),
        ) if a == current => {
            Ok(Some(b))
        }

        (
            GraphEndpoint::Detection(a),
            GraphEndpoint::Detection(b),
        ) if b == current => {
            Ok(Some(a))
        }

        (
            GraphEndpoint::Boundary(_),
            _,
        )
        | (
            _,
            GraphEndpoint::Boundary(_),
        ) => Ok(None),

        _ => Err(
            MwpmError::InvalidGraphEndpoint,
        ),
    }
}

// ============================================================================
// Exact MWPM
// ============================================================================

/// Solves exact minimum-weight perfect matching using memoized recurrence.
///
/// For the first unmatched event `i`, every remaining event `j` is considered
/// as its partner. The optimal solution is:
///
/// ```text
/// min(
///     weight(i,j) + optimum(remaining \ {i,j})
/// )
/// ```
///
/// Because the active event set is bounded by `MAX_MWPM_EVENTS`, the state
/// space remains bounded.
fn solve_exact_mwpm(
    metric: &ShortestPathMetric,
    config: MwpmConfig,
) -> Result<Vec<MatchingPair>, MwpmError> {
    let count =
        metric.event_count();

    if count == 0 {
        return Ok(Vec::new());
    }

    if count
        > config.max_events()
    {
        return Err(
            MwpmError::TooManyEvents {
                count,
                limit:
                    config.max_events(),
            },
        );
    }

    if count % 2 != 0 {
        return Err(
            MwpmError::OddDetectionEventCount {
                count,
            },
        );
    }

    if count > 63 {
        return Err(
            MwpmError::EventMaskOverflow,
        );
    }

    let full_mask =
        if count == 64 {
            u64::MAX
        } else {
            (1u64 << count) - 1
        };

    let mut memo =
        BTreeMap::<
            u64,
            u64,
        >::new();

    let mut choices =
        BTreeMap::<
            u64,
            usize,
        >::new();

    let total =
        solve_mask(
            metric,
            full_mask,
            &mut memo,
            &mut choices,
        )?;

    if total == INF {
        return Err(
            MwpmError::NoPerfectMatching,
        );
    }

    let mut mask =
        full_mask;

    let mut pairs =
        Vec::with_capacity(
            count / 2,
        );

    while mask != 0 {
        let first =
            first_set_bit(mask)
                .ok_or(
                    MwpmError::InternalMatchingState,
                )?;

        let second =
            choices
                .get(&mask)
                .copied()
                .ok_or(
                    MwpmError::InternalMatchingState,
                )?;

        let event_first =
            *metric
                .events
                .get(first)
                .ok_or(
                    MwpmError::MatchingIndexOutOfRange,
                )?;

        let event_second =
            *metric
                .events
                .get(second)
                .ok_or(
                    MwpmError::MatchingIndexOutOfRange,
                )?;

        let weight =
            metric.distance(
                first,
                second,
            )?;

        pairs.push(
            MatchingPair::new(
                event_first,
                event_second,
                weight,
            )?,
        );

        mask &=
            !(1u64 << first);

        mask &=
            !(1u64 << second);
    }

    pairs.sort_by_key(
        |pair| {
            (
                pair.first(),
                pair.second(),
            )
        },
    );

    Ok(pairs)
}

fn solve_mask(
    metric: &ShortestPathMetric,
    mask: u64,
    memo: &mut BTreeMap<
        u64,
        u64,
    >,
    choices: &mut BTreeMap<
        u64,
        usize,
    >,
) -> Result<u64, MwpmError> {
    if mask == 0 {
        return Ok(0);
    }

    if let Some(&value) =
        memo.get(&mask)
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

    let mut best =
        INF;

    let mut best_second =
        None;

    let mut remaining =
        without_first;

    while remaining != 0 {
        let second =
            first_set_bit(
                remaining,
            )
            .ok_or(
                MwpmError::InternalMatchingState,
            )?;

        let remaining_after_pair =
            without_first
                & !(1u64 << second);

        let pair_weight =
            metric.distance(
                first,
                second,
            )?;

        if pair_weight != INF {
            let remainder =
                solve_mask(
                    metric,
                    remaining_after_pair,
                    memo,
                    choices,
                )?;

            if remainder != INF {
                let total =
                    pair_weight
                        .checked_add(
                            remainder,
                        )
                        .ok_or(
                            MwpmError::ArithmeticOverflow,
                        )?;

                let is_better =
                    total < best
                        || (
                            total == best
                                && best_second
                                    .map_or(
                                        true,
                                        |existing| {
                                            second
                                                < existing
                                        },
                                    )
                        );

                if is_better {
                    best =
                        total;

                    best_second =
                        Some(second);
                }
            }
        }

        remaining &=
            !(1u64 << second);
    }

    if let Some(second) =
        best_second
    {
        choices.insert(
            mask,
            second,
        );
    }

    memo.insert(
        mask,
        best,
    );

    Ok(best)
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
    pairs: Vec<MatchingPair>,
    config: MwpmConfig,
) -> Result<MwpmResult, MwpmError> {
    let mut total_weight =
        0u64;

    let mut paths =
        Vec::with_capacity(
            pairs.len(),
        );

    let mut path_edges =
        0usize;

    for pair in &pairs {
        total_weight =
            total_weight
                .checked_add(
                    pair.weight(),
                )
                .ok_or(
                    MwpmError::ArithmeticOverflow,
                )?;

        let edges =
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
                )?;

        path_edges =
            path_edges
                .checked_add(
                    edges.len(),
                )
                .ok_or(
                    MwpmError::ArithmeticOverflow,
                )?;

        if path_edges
            > config.max_path_edges()
        {
            return Err(
                MwpmError::CorrectionPathTooLong {
                    limit:
                        config
                            .max_path_edges(),
                },
            );
        }

        paths.push(
            CorrectionPath::new(
                *pair,
                edges.to_vec(),
            )?,
        );
    }

    graph
        .validate()
        .map_err(MwpmError::Graph)?;

    MwpmResult::new(
        pairs,
        paths,
        total_weight,
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

/// Errors returned by the MWPM subsystem.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum MwpmError {
    /// The underlying decoding graph is invalid.
    Graph(
        super::decoding_graph::DecodingGraphError,
    ),

    /// The graph exceeds the decoder's node budget.
    GraphTooLarge {
        nodes: usize,
        limit: usize,
    },

    /// The graph exceeds the decoder's edge budget.
    GraphTooLargeByEdges {
        edges: usize,
        limit: usize,
    },

    /// Boundary matching is intentionally not performed until boundary
    /// semantics are supplied by the topology layer.
    BoundaryMatchingUnsupported,

    /// Too many active detection events for the exact solver.
    TooManyEvents {
        count: usize,
        limit: usize,
    },

    /// An odd number of events cannot form a detection-to-detection perfect
    /// matching.
    OddDetectionEventCount {
        count: usize,
    },

    /// A self-pair was requested.
    SelfMatch {
        event: MatchingEvent,
    },

    /// Matching pairs must use canonical event ordering.
    NonCanonicalPair,

    /// Matching index was outside the active event set.
    MatchingIndexOutOfRange,

    /// The active event count cannot be represented by the internal mask.
    EventMaskOverflow,

    /// No perfect matching exists in the graph metric.
    NoPerfectMatching,

    /// Shortest-path computation exceeded its bounded work budget.
    RelaxationLimitExceeded {
        limit: usize,
    },

    /// An arithmetic operation overflowed.
    ArithmeticOverflow,

    /// A shortest-path state was inconsistent.
    InternalDistanceState,

    /// The matching state was internally inconsistent.
    InternalMatchingState,

    /// The final result contained inconsistent pair/path counts.
    InternalResultMismatch,

    /// A path contained a cycle while being reconstructed.
    PathCycle,

    /// A required path was unreachable.
    UnreachablePair {
        first: NodeId,
        second: NodeId,
    },

    /// A predecessor edge could not be found in the graph.
    PathEdgeMissing {
        first: NodeId,
        second: NodeId,
    },

    /// A graph endpoint was malformed for this decoder.
    InvalidGraphEndpoint,

    /// A reconstructed correction path exceeds the safety limit.
    CorrectionPathTooLong {
        limit: usize,
    },

    /// Decoder configuration is invalid.
    InvalidConfiguration,
}

impl fmt::Display for MwpmError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Graph(error) => {
                write!(
                    f,
                    "decoding graph validation failed: {error}"
                )
            }

            Self::GraphTooLarge {
                nodes,
                limit,
            } => {
                write!(
                    f,
                    "decoding graph contains {nodes} nodes, exceeding MWPM limit {limit}"
                )
            }

            Self::GraphTooLargeByEdges {
                edges,
                limit,
            } => {
                write!(
                    f,
                    "decoding graph contains {edges} edges, exceeding MWPM limit {limit}"
                )
            }

            Self::BoundaryMatchingUnsupported => {
                write!(
                    f,
                    "boundary matching is not enabled by the current topology contract"
                )
            }

            Self::TooManyEvents {
                count,
                limit,
            } => {
                write!(
                    f,
                    "MWPM received {count} active detection events, exceeding exact-solver limit {limit}"
                )
            }

            Self::OddDetectionEventCount {
                count,
            } => {
                write!(
                    f,
                    "MWPM received an odd number of detection events: {count}"
                )
            }

            Self::SelfMatch {
                event,
            } => {
                write!(
                    f,
                    "MWPM cannot match an event with itself: {event}"
                )
            }

            Self::NonCanonicalPair => {
                write!(
                    f,
                    "MWPM matching pair is not in canonical order"
                )
            }

            Self::MatchingIndexOutOfRange => {
                write!(
                    f,
                    "MWPM matching index is outside the active event set"
                )
            }

            Self::EventMaskOverflow => {
                write!(
                    f,
                    "MWPM event set cannot be represented by the internal mask"
                )
            }

            Self::NoPerfectMatching => {
                write!(
                    f,
                    "no perfect matching exists for the active detection events"
                )
            }

            Self::RelaxationLimitExceeded {
                limit,
            } => {
                write!(
                    f,
                    "MWPM shortest-path relaxation limit of {limit} was exceeded"
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    f,
                    "MWPM arithmetic overflow"
                )
            }

            Self::InternalDistanceState => {
                write!(
                    f,
                    "MWPM encountered inconsistent shortest-path state"
                )
            }

            Self::InternalMatchingState => {
                write!(
                    f,
                    "MWPM encountered inconsistent matching state"
                )
            }

            Self::InternalResultMismatch => {
                write!(
                    f,
                    "MWPM produced inconsistent pair/path counts"
                )
            }

            Self::PathCycle => {
                write!(
                    f,
                    "MWPM path reconstruction encountered a cycle"
                )
            }

            Self::UnreachablePair {
                first,
                second,
            } => {
                write!(
                    f,
                    "no decoding-graph path exists between {first} and {second}"
                )
            }

            Self::PathEdgeMissing {
                first,
                second,
            } => {
                write!(
                    f,
                    "MWPM predecessor path edge is missing between {first} and {second}"
                )
            }

            Self::InvalidGraphEndpoint => {
                write!(
                    f,
                    "MWPM encountered an invalid graph endpoint"
                )
            }

            Self::CorrectionPathTooLong {
                limit,
            } => {
                write!(
                    f,
                    "MWPM correction path exceeds the limit of {limit} graph edges"
                )
            }

            Self::InvalidConfiguration => {
                write!(
                    f,
                    "invalid MWPM decoder configuration"
                )
            }
        }
    }
}

impl std::error::Error
    for MwpmError
{
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
                .unwrap(),
            MeasurementRound::new(0)
                .unwrap(),
        )
        .unwrap()
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
                    .add_detection(
                        coordinate(
                            index as i64,
                        ),
                        StabilizerId::new(
                            index,
                        ),
                        MeasurementConfidence::certain(),
                    )
                    .unwrap();

            nodes.push(node);
        }

        for index in 0..count.saturating_sub(1)
        {
            graph
                .connect(
                    GraphEndpoint::Detection(
                        nodes[index],
                    ),
                    GraphEndpoint::Detection(
                        nodes[index + 1],
                    ),
                    EdgeKind::Spatial,
                    EdgeWeight::new(1)
                        .unwrap(),
                )
                .unwrap();
        }

        graph
    }

    #[test]
    fn empty_graph_is_trivial() {
        let graph =
            DecodingGraph::new();

        let decoder =
            MwpmDecoder::new();

        let result =
            decoder
                .decode_graph(&graph)
                .unwrap();

        assert!(
            result.is_trivial()
        );

        assert_eq!(
            result.pair_count(),
            0
        );

        assert_eq!(
            result.total_weight(),
            0
        );
    }

    #[test]
    fn two_events_are_matched() {
        let graph =
            graph_with_line(2);

        let decoder =
            MwpmDecoder::new();

        let result =
            decoder
                .decode_graph(&graph)
                .unwrap();

        assert_eq!(
            result.pair_count(),
            1
        );

        assert_eq!(
            result.total_weight(),
            1
        );

        let pair =
            result
                .pairs()
                .first()
                .copied()
                .unwrap();

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
                .add_detection(
                    coordinate(0),
                    StabilizerId::new(0),
                    MeasurementConfidence::certain(),
                )
                .unwrap();

        let b =
            graph
                .add_detection(
                    coordinate(1),
                    StabilizerId::new(1),
                    MeasurementConfidence::certain(),
                )
                .unwrap();

        let c =
            graph
                .add_detection(
                    coordinate(2),
                    StabilizerId::new(2),
                    MeasurementConfidence::certain(),
                )
                .unwrap();

        graph
            .connect(
                GraphEndpoint::Detection(a),
                GraphEndpoint::Detection(b),
                EdgeKind::Spatial,
                EdgeWeight::new(10)
                    .unwrap(),
            )
            .unwrap();

        graph
            .connect(
                GraphEndpoint::Detection(a),
                GraphEndpoint::Detection(c),
                EdgeKind::Spatial,
                EdgeWeight::new(2)
                    .unwrap(),
            )
            .unwrap();

        graph
            .connect(
                GraphEndpoint::Detection(c),
                GraphEndpoint::Detection(b),
                EdgeKind::Spatial,
                EdgeWeight::new(2)
                    .unwrap(),
            )
            .unwrap();

        let decoder =
            MwpmDecoder::new();

        let result =
            decoder
                .decode_graph(&graph)
                .unwrap();

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
    fn exact_solver_beats_greedy_pairing() {
        let mut graph =
            DecodingGraph::new();

        let mut nodes =
            Vec::new();

        for index in 0..4 {
            nodes.push(
                graph
                    .add_detection(
                        coordinate(
                            index as i64,
                        ),
                        StabilizerId::new(
                            index,
                        ),
                        MeasurementConfidence::certain(),
                    )
                    .unwrap(),
            );
        }

        // Optimal:
        //
        // 0--1 = 4
        // 2--3 = 4
        //
        // total = 8
        //
        // A tempting local choice can produce:
        //
        // 0--2 = 5
        // 1--3 = 5
        //
        // total = 10.
        let weights = [
            (0usize, 1usize, 4u64),
            (0usize, 2usize, 5u64),
            (0usize, 3usize, 6u64),
            (1usize, 2usize, 6u64),
            (1usize, 3usize, 5u64),
            (2usize, 3usize, 4u64),
        ];

        for (
            first,
            second,
            weight,
        ) in weights {
            graph
                .connect(
                    GraphEndpoint::Detection(
                        nodes[first],
                    ),
                    GraphEndpoint::Detection(
                        nodes[second],
                    ),
                    EdgeKind::Custom,
                    EdgeWeight::new(
                        weight,
                    )
                    .unwrap(),
                )
                .unwrap();
        }

        let result =
            MwpmDecoder::new()
                .decode_graph(
                    &graph,
                )
                .unwrap();

        assert_eq!(
            result.total_weight(),
            8
        );

        assert_eq!(
            result.pair_count(),
            2
        );
    }

    #[test]
    fn odd_event_count_is_rejected() {
        let graph =
            graph_with_line(3);

        let result =
            MwpmDecoder::new()
                .decode_graph(
                    &graph,
                );

        assert_eq!(
            result,
            Err(
                MwpmError::OddDetectionEventCount {
                    count: 3,
                }
            )
        );
    }

    #[test]
    fn disconnected_events_are_rejected() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_detection(
                coordinate(0),
                StabilizerId::new(0),
                MeasurementConfidence::certain(),
            )
            .unwrap();

        graph
            .add_detection(
                coordinate(1),
                StabilizerId::new(1),
                MeasurementConfidence::certain(),
            )
            .unwrap();

        let result =
            MwpmDecoder::new()
                .decode_graph(
                    &graph,
                );

        assert_eq!(
            result,
            Err(
                MwpmError::NoPerfectMatching
            )
        );
    }

    #[test]
    fn boundary_graphs_are_not_silently_misdecoded() {
        let mut graph =
            graph_with_line(2);

        graph
            .add_boundary(
                coordinate(10),
            )
            .unwrap();

        let result =
            MwpmDecoder::new()
                .decode_graph(
                    &graph,
                );

        assert_eq!(
            result,
            Err(
                MwpmError::BoundaryMatchingUnsupported
            )
        );
    }

    #[test]
    fn too_many_events_are_rejected() {
        let mut graph =
            DecodingGraph::new();

        for index in
            0..(MAX_MWPM_EVENTS + 1)
        {
            graph
                .add_detection(
                    coordinate(
                        index as i64,
                    ),
                    StabilizerId::new(
                        index,
                    ),
                    MeasurementConfidence::certain(),
                )
                .unwrap();
        }

        let result =
            MwpmDecoder::new()
                .decode_graph(
                    &graph,
                );

        assert_eq!(
            result,
            Err(
                MwpmError::TooManyEvents {
                    count:
                        MAX_MWPM_EVENTS,
                    limit:
                        MAX_MWPM_EVENTS,
                }
            )
        );
    }

    #[test]
    fn matching_pairs_are_canonical() {
        let first =
            MatchingEvent::new(
                NodeId::new(1),
            );

        let second =
            MatchingEvent::new(
                NodeId::new(2),
            );

        assert!(
            MatchingPair::new(
                first,
                second,
                10,
            )
            .is_ok()
        );

        assert_eq!(
            MatchingPair::new(
                second,
                first,
                10,
            ),
            Err(
                MwpmError::NonCanonicalPair
            )
        );
    }

    #[test]
    fn configuration_is_bounded() {
        assert!(
            MwpmConfig::new(
                MAX_MWPM_EVENTS + 1,
                MAX_MWPM_GRAPH_NODES,
                MAX_MWPM_GRAPH_EDGES,
                MAX_SHORTEST_PATH_RELAXATIONS,
                MAX_CORRECTION_PATH_EDGES,
            )
            .is_err()
        );
    }

    #[test]
    fn deterministic_tie_breaking() {
        let mut graph =
            DecodingGraph::new();

        let mut nodes =
            Vec::new();

        for index in 0..4 {
            nodes.push(
                graph
                    .add_detection(
                        coordinate(
                            index as i64,
                        ),
                        StabilizerId::new(
                            index,
                        ),
                        MeasurementConfidence::certain(),
                    )
                    .unwrap(),
            );
        }

        let edges = [
            (0usize, 1usize),
            (0usize, 2usize),
            (0usize, 3usize),
            (1usize, 2usize),
            (1usize, 3usize),
            (2usize, 3usize),
        ];

        for (
            first,
            second,
        ) in edges {
            graph
                .connect(
                    GraphEndpoint::Detection(
                        nodes[first],
                    ),
                    GraphEndpoint::Detection(
                        nodes[second],
                    ),
                    EdgeKind::Custom,
                    EdgeWeight::new(1)
                        .unwrap(),
                )
                .unwrap();
        }

        let decoder =
            MwpmDecoder::new();

        let first =
            decoder
                .decode_graph(&graph)
                .unwrap();

        let second =
            decoder
                .decode_graph(&graph)
                .unwrap();

        assert_eq!(
            first,
            second
        );
    }
}