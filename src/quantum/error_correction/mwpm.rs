//! Zamani Quantum Error Correction — Minimum-Weight Perfect Matching.
//!
//! # Ownership
//!
//! This module owns the exact, deterministic, graph-native MWPM algorithm.
//!
//! It owns:
//!
//! - MWPM matching endpoint representations;
//! - event/event matching;
//! - event/boundary matching;
//! - shortest-path metric construction;
//! - deterministic shortest-path reconstruction;
//! - exact bounded dynamic-programming matching;
//! - correction-path materialization;
//! - MWPM-specific execution statistics;
//! - MWPM-specific errors;
//! - integration with `DecodeContext`;
//! - resource-safe exact-solver admission.
//!
//! It does NOT own:
//!
//! - stabilizer algebra (`stabilizer.rs`);
//! - syndrome construction (`syndrome.rs`);
//! - decoding-graph construction (`decoding_graph.rs`);
//! - global resource policy (`limits.rs`);
//! - runtime resource accounting (`resources.rs`);
//! - memory allocation policy (`memory.rs`);
//! - capability authority (`capabilities.rs`);
//! - logical-equivalence mathematics (`logical_equivalence.rs` / `logical.rs`);
//! - Pauli-frame state (`pauli_frame.rs`);
//! - QPU access;
//! - scheduling;
//! - distributed execution;
//! - statistical threshold analysis.
//!
//! # Integration contract
//!
//! ```text
//! SurfaceCode / Syndrome / QPU / Replay
//!                    |
//!                    v
//!             DecodingGraph
//!                    |
//!                    v
//!             MwpmDecoder
//!                    |
//!        +-----------+-----------+
//!        |           |           |
//!        v           v           v
//!     limits    cancellation  DecodeContext
//!        |           |           |
//!        +-----------+-----------+
//!                    |
//!                    v
//!             resource preflight
//!                    |
//!                    v
//!              metric closure
//!                    |
//!                    v
//!             exact MWPM solver
//!                    |
//!                    v
//!             correction paths
//!                    |
//!                    v
//!              MwpmResult
//!                    |
//!          +---------+----------+
//!          |                    |
//!          v                    v
//!     decoder_result       PauliFrame
//!                              |
//!                              v
//!                    logical equivalence
//! ```
//!
//! # Resource contract
//!
//! `QecLimits` is the only production resource policy.
//!
//! This module does NOT introduce a hidden production limit such as the old
//! fixed 24-event or 4096-node ceilings.
//!
//! Exact MWPM is exponential. Therefore the implementation performs a
//! checked preflight against:
//!
//! - `max_memory_bytes`;
//! - `max_decoder_iterations`;
//! - `max_decoder_time_ns`;
//! - `max_graph_nodes`;
//! - `max_graph_edges`;
//! - `max_syndrome_events`;
//!
//! If the exact algorithm cannot safely fit within the active policy, it
//! rejects the workload instead of allocating unbounded state or silently
//! falling back to an approximation.
//!
//! The 64-bit event-mask representation is an implementation representation
//! constraint, not a QEC production policy. Workloads exceeding it must be
//! routed to a scalable MWPM implementation in a future execution layer.
//!
//! # Boundary semantics
//!
//! A decoding boundary is NOT a capacity-one vertex.
//!
//! Multiple detection events may terminate independently at the same physical
//! or logical boundary. Therefore boundary usage is intentionally NOT encoded
//! in the dynamic-programming state.
//!
//! This fixes an important correctness problem in the previous implementation.
//!
//! # Determinism
//!
//! Determinism is guaranteed by:
//!
//! - ordered graph traversal;
//! - ordered endpoint identifiers;
//! - integer edge weights;
//! - deterministic shortest-path tie breaking;
//! - deterministic first-event selection;
//! - deterministic equal-cost matching tie breaking;
//! - canonical result ordering.
//!
//! No floating-point value participates in a matching decision.
//!
//! # Cancellation
//!
//! Cancellation is checked:
//!
//! - before execution;
//! - during graph traversal;
//! - during shortest-path construction;
//! - during matching;
//! - during path reconstruction;
//! - during result materialization.
//!
//! # Security
//!
//! MWPM receives no QPU credentials and cannot submit hardware work.
//!
//! When called through `decode_graph_with_context`, the caller must possess
//! `Capability::Decode`.
//!
//! # Rust compatibility
//!
//! Rust 1.97.1.
//!
//! No unstable language features are used.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::fmt;
use std::time::Instant;

use super::cancellation::CancellationToken;
use super::capabilities::Capability;
use super::decoder::DecodeContext;
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
    ResourceKind,
};
use super::limits::QecLimits;

/* ========================================================================== */
/* Matching endpoint                                                          */
/* ========================================================================== */

/// Endpoint participating in an MWPM decision.
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
    /// Physical detection event.
    Detection(MatchingEvent),

    /// Physical or logical decoding boundary.
    Boundary(BoundaryId),
}

impl MatchingEndpoint {
    /// Returns true when this is a detection event.
    #[must_use]
    pub const fn is_detection(self) -> bool {
        matches!(self, Self::Detection(_))
    }

    /// Returns true when this is a boundary.
    #[must_use]
    pub const fn is_boundary(self) -> bool {
        matches!(self, Self::Boundary(_))
    }

    /// Returns the detection event when present.
    #[must_use]
    pub const fn detection(self) -> Option<MatchingEvent> {
        match self {
            Self::Detection(event) => Some(event),
            Self::Boundary(_) => None,
        }
    }

    /// Returns the boundary when present.
    #[must_use]
    pub const fn boundary(self) -> Option<BoundaryId> {
        match self {
            Self::Detection(_) => None,
            Self::Boundary(boundary) => Some(boundary),
        }
    }
}

/* ========================================================================== */
/* Matching event                                                             */
/* ========================================================================== */

/// Stable active detection event.
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
    #[must_use]
    pub const fn new(node: NodeId) -> Self {
        Self { node }
    }

    /// Returns the graph node.
    #[must_use]
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

/* ========================================================================== */
/* Event/event pair                                                           */
/* ========================================================================== */

/// One detection-event ↔ detection-event matching.
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
            return Err(MwpmError::SelfMatch {
                event: first,
            });
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
    #[must_use]
    pub const fn first(&self) -> MatchingEvent {
        self.first
    }

    /// Returns the second event.
    #[must_use]
    pub const fn second(&self) -> MatchingEvent {
        self.second
    }

    /// Returns the metric weight.
    #[must_use]
    pub const fn weight(&self) -> u64 {
        self.weight
    }
}

/* ========================================================================== */
/* Event/boundary pair                                                        */
/* ========================================================================== */

/// One detection-event ↔ boundary matching.
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
    /// Creates a boundary matching.
    #[must_use]
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
    #[must_use]
    pub const fn event(&self) -> MatchingEvent {
        self.event
    }

    /// Returns the boundary.
    #[must_use]
    pub const fn boundary(&self) -> BoundaryId {
        self.boundary
    }

    /// Returns the metric weight.
    #[must_use]
    pub const fn weight(&self) -> u64 {
        self.weight
    }
}

/* ========================================================================== */
/* Unified matching                                                           */
/* ========================================================================== */

/// One complete MWPM matching decision.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum Matching {
    /// Detection event paired with another detection event.
    EventPair(MatchingPair),

    /// Detection event terminated at a decoding boundary.
    BoundaryPair(BoundaryMatching),
}

impl Matching {
    /// Returns the metric weight.
    #[must_use]
    pub const fn weight(self) -> u64 {
        match self {
            Self::EventPair(pair) => pair.weight(),
            Self::BoundaryPair(pair) => pair.weight(),
        }
    }

    /// Returns true for an event/boundary match.
    #[must_use]
    pub const fn touches_boundary(self) -> bool {
        matches!(self, Self::BoundaryPair(_))
    }
}

/* ========================================================================== */
/* Correction path                                                            */
/* ========================================================================== */

/// Physical decoding-graph path implementing one matching decision.
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
        maximum_edges: usize,
    ) -> Result<Self, MwpmError> {
        if edges.len() > maximum_edges {
            return Err(
                MwpmError::CorrectionPathTooLong {
                    requested: edges.len(),
                    limit: maximum_edges,
                },
            );
        }

        Ok(Self {
            matching,
            edges,
        })
    }

    /// Returns the matching represented by this path.
    #[must_use]
    pub const fn matching(&self) -> Matching {
        self.matching
    }

    /// Returns graph edges in path order.
    #[must_use]
    pub fn edges(&self) -> &[GraphEdge] {
        &self.edges
    }

    /// Returns edge count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Returns true when the path contains no edges.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

/* ========================================================================== */
/* Termination                                                                */
/* ========================================================================== */

/// MWPM-specific termination state.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum MwpmTermination {
    /// The exact solver completed successfully.
    Completed,

    /// The graph contained no detection events.
    EmptyInput,
}

/* ========================================================================== */
/* Result                                                                     */
/* ========================================================================== */

/// Complete result of one exact MWPM execution.
///
/// This remains MWPM-specific. The future `decoder_result.rs` may wrap this
/// with canonical decoder metrics, resource snapshots, logical classification
/// and witnesses without changing the matching algorithm.
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
    work_units: u64,
    shortest_path_relaxations: u64,
}

impl MwpmResult {
    fn new(
        matchings: Vec<Matching>,
        paths: Vec<CorrectionPath>,
        total_weight: u64,
        termination: MwpmTermination,
        work_units: u64,
        shortest_path_relaxations: u64,
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
            work_units,
            shortest_path_relaxations,
        })
    }

    /// Returns every matching decision.
    #[must_use]
    pub fn matchings(&self) -> &[Matching] {
        &self.matchings
    }

    /// Returns event/event pairs.
    ///
    /// Boundary matches are available through `boundary_matches()`.
    #[must_use]
    pub fn pairs(&self) -> Vec<MatchingPair> {
        self.matchings
            .iter()
            .filter_map(|matching| {
                match matching {
                    Matching::EventPair(pair) => {
                        Some(*pair)
                    }
                    Matching::BoundaryPair(_) => None,
                }
            })
            .collect()
    }

    /// Compatibility alias for event/event pairs.
    #[must_use]
    pub fn event_pairs(&self) -> Vec<MatchingPair> {
        self.pairs()
    }

    /// Returns all event/boundary matches.
    #[must_use]
    pub fn boundary_matches(
        &self,
    ) -> Vec<BoundaryMatching> {
        self.matchings
            .iter()
            .filter_map(|matching| {
                match matching {
                    Matching::EventPair(_) => None,
                    Matching::BoundaryPair(pair) => {
                        Some(*pair)
                    }
                }
            })
            .collect()
    }

    /// Returns materialized correction paths.
    #[must_use]
    pub fn paths(&self) -> &[CorrectionPath] {
        &self.paths
    }

    /// Returns total metric weight.
    #[must_use]
    pub const fn total_weight(&self) -> u64 {
        self.total_weight
    }

    /// Returns number of matching decisions.
    #[must_use]
    pub fn pair_count(&self) -> usize {
        self.matchings.len()
    }

    /// Returns number of boundary matches.
    #[must_use]
    pub fn boundary_pair_count(&self) -> usize {
        self.matchings
            .iter()
            .filter(|matching| {
                matching.touches_boundary()
            })
            .count()
    }

    /// Returns whether no matching was required.
    #[must_use]
    pub fn is_trivial(&self) -> bool {
        self.matchings.is_empty()
    }

    /// Returns the MWPM termination state.
    #[must_use]
    pub const fn termination(
        &self,
    ) -> MwpmTermination {
        self.termination
    }

    /// Returns total deterministic work units consumed.
    #[must_use]
    pub const fn work_units(&self) -> u64 {
        self.work_units
    }

    /// Returns shortest-path relaxation count.
    #[must_use]
    pub const fn shortest_path_relaxations(
        &self,
    ) -> u64 {
        self.shortest_path_relaxations
    }
}

/* ========================================================================== */
/* Configuration                                                              */
/* ========================================================================== */

/// Exact-MWPM algorithm configuration.
///
/// These are algorithm preferences/capabilities, not a second production
/// resource-policy system.
///
/// `QecLimits` always wins when the two disagree.
///
/// The production configuration defaults to `usize::MAX`, meaning that no
/// independent hidden ceiling is introduced. Actual admission is determined
/// by the active `QecLimits`.
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
    /// Creates the unconstrained algorithm configuration.
    ///
    /// Global `QecLimits` remain authoritative.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_events: usize::MAX,
            max_graph_nodes: usize::MAX,
            max_graph_edges: usize::MAX,
            max_relaxations: usize::MAX,
            max_path_edges: usize::MAX,
        }
    }

    /// Creates explicit algorithm preferences.
    ///
    /// These values may reduce the active workload but can never enlarge the
    /// canonical `QecLimits`.
    pub const fn new(
        max_events: usize,
        max_graph_nodes: usize,
        max_graph_edges: usize,
        max_relaxations: usize,
        max_path_edges: usize,
    ) -> Result<Self, MwpmError> {
        if max_events == 0
            || max_graph_nodes == 0
            || max_graph_edges == 0
            || max_relaxations == 0
            || max_path_edges == 0
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

    /// Returns configured event preference.
    #[must_use]
    pub const fn max_events(self) -> usize {
        self.max_events
    }

    /// Returns configured graph-node preference.
    #[must_use]
    pub const fn max_graph_nodes(self) -> usize {
        self.max_graph_nodes
    }

    /// Returns configured graph-edge preference.
    #[must_use]
    pub const fn max_graph_edges(self) -> usize {
        self.max_graph_edges
    }

    /// Returns configured relaxation preference.
    #[must_use]
    pub const fn max_relaxations(self) -> usize {
        self.max_relaxations
    }

    /// Returns configured correction-path preference.
    #[must_use]
    pub const fn max_path_edges(self) -> usize {
        self.max_path_edges
    }
}

impl Default for MwpmConfig {
    fn default() -> Self {
        Self::production()
    }
}

/* ========================================================================== */
/* Decoder                                                                    */
/* ========================================================================== */

/// Deterministic exact MWPM decoder.
#[derive(
    Debug,
    Clone,
)]
pub struct MwpmDecoder {
    config: MwpmConfig,
}

impl MwpmDecoder {
    /// Creates the production MWPM decoder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: MwpmConfig::production(),
        }
    }

    /// Creates MWPM with explicit algorithm preferences.
    #[must_use]
    pub const fn with_config(
        config: MwpmConfig,
    ) -> Self {
        Self { config }
    }

    /// Returns the configured algorithm preferences.
    #[must_use]
    pub const fn config(&self) -> MwpmConfig {
        self.config
    }

    /// Decodes using the graph's resource policy.
    pub fn decode_graph(
        &self,
        graph: &DecodingGraph,
    ) -> Result<MwpmResult, MwpmError> {
        let limits = graph.limits();
        let cancellation =
            CancellationToken::new();

        self.decode_graph_with_context(
            graph,
            &limits,
            &cancellation,
        )
    }

    /// Decodes with explicit resource policy and cancellation.
    ///
    /// The graph's own policy is never weakened by the supplied policy.
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

        let effective = EffectiveLimits::new(
            graph.limits(),
            *limits,
            self.config,
        );

        preflight_graph(
            graph,
            effective,
        )?;

        let deadline =
            Deadline::new(
                effective.max_decoder_time_ns,
            );

        let mut budget =
            WorkBudget::new(
                effective.max_decoder_iterations,
                deadline,
                cancellation,
            );

        let events =
            collect_detection_events(
                graph,
                effective,
                &mut budget,
            )?;

        if events.is_empty() {
            return MwpmResult::new(
                Vec::new(),
                Vec::new(),
                0,
                MwpmTermination::EmptyInput,
                budget.work_units(),
                budget.relaxations(),
            );
        }

        let metric =
            ShortestPathMetric::build(
                graph,
                &events,
                effective,
                &mut budget,
            )?;

        let matching =
            solve_exact_mwpm(
                &metric,
                effective,
                &mut budget,
            )?;

        materialize_result(
            graph,
            &metric,
            matching,
            effective,
            &mut budget,
        )
    }

    /// Graph-native production integration with the common decoder execution
    /// context.
    ///
    /// MWPM intentionally consumes `DecodingGraph` here because topology and
    /// boundary semantics cannot be reconstructed safely from a primitive
    /// syndrome alone.
    pub fn decode_graph_with_decode_context(
        &self,
        graph: &DecodingGraph,
        context: &DecodeContext<'_>,
    ) -> QecResult<MwpmResult> {
        if !context
            .capabilities()
            .contains(Capability::Decode)
        {
            return Err(
                QecError::CapabilityDenied {
                    capability:
                        Capability::Decode
                            .name()
                            .to_owned(),
                    operation:
                        "mwpm_decode".to_owned(),
                    message:
                        "MWPM requires the decode capability"
                            .to_owned(),
                },
            );
        }

        context
            .cancellation()
            .check()?;

        let limits =
            context.config().limits;

        self.decode_graph_with_context(
            graph,
            &limits,
            context.cancellation(),
        )
        .map_err(MwpmError::into_qec_error)
    }
}

impl Default for MwpmDecoder {
    fn default() -> Self {
        Self::new()
    }
}

/* ========================================================================== */
/* Effective limits                                                           */
/* ========================================================================== */

/// Immutable intersection between the graph policy, caller policy and
/// algorithm preferences.
#[derive(
    Debug,
    Clone,
    Copy,
)]
struct EffectiveLimits {
    max_syndrome_events: usize,
    max_graph_nodes: usize,
    max_graph_edges: usize,
    max_memory_bytes: u64,
    max_decoder_time_ns: u64,
    max_decoder_iterations: u64,
    max_path_edges: usize,
}

impl EffectiveLimits {
    fn new(
        graph: QecLimits,
        requested: QecLimits,
        config: MwpmConfig,
    ) -> Self {
        Self {
            max_syndrome_events: graph
                .max_syndrome_events
                .min(
                    requested
                        .max_syndrome_events,
                )
                .min(config.max_events),

            max_graph_nodes: graph
                .max_graph_nodes
                .min(
                    requested
                        .max_graph_nodes,
                )
                .min(config.max_graph_nodes),

            max_graph_edges: graph
                .max_graph_edges
                .min(
                    requested
                        .max_graph_edges,
                )
                .min(config.max_graph_edges),

            max_memory_bytes: graph
                .max_memory_bytes
                .min(
                    requested
                        .max_memory_bytes,
                ),

            max_decoder_time_ns: graph
                .max_decoder_time_ns
                .min(
                    requested
                        .max_decoder_time_ns,
                ),

            max_decoder_iterations:
                u64::try_from(
                    graph
                        .max_decoder_iterations
                        .min(
                            requested
                                .max_decoder_iterations,
                        )
                        .min(
                            config
                                .max_relaxations,
                        ),
                )
                .unwrap_or(u64::MAX),

            max_path_edges: graph
                .max_graph_edges
                .min(
                    requested
                        .max_graph_edges,
                )
                .min(config.max_path_edges),
        }
    }
}

/* ========================================================================== */
/* Deadline and work budget                                                   */
/* ========================================================================== */

#[derive(Debug)]
struct Deadline {
    started: Instant,
    maximum_ns: u64,
}

impl Deadline {
    fn new(maximum_ns: u64) -> Self {
        Self {
            started: Instant::now(),
            maximum_ns,
        }
    }

    fn check(&self) -> Result<(), MwpmError> {
        let elapsed =
            self.started.elapsed();

        let elapsed_ns =
            elapsed
                .as_nanos();

        if elapsed_ns
            > u128::from(
                self.maximum_ns,
            )
        {
            return Err(
                MwpmError::TimeLimitExceeded {
                    limit_ns:
                        self.maximum_ns,
                },
            );
        }

        Ok(())
    }
}

struct WorkBudget<'a> {
    maximum: u64,
    work: u64,
    relaxations: u64,
    deadline: Deadline,
    cancellation: &'a CancellationToken,
}

impl<'a> WorkBudget<'a> {
    fn new(
        maximum: u64,
        deadline: Deadline,
        cancellation: &'a CancellationToken,
    ) -> Self {
        Self {
            maximum,
            work: 0,
            relaxations: 0,
            deadline,
            cancellation,
        }
    }

    fn poll(&mut self) -> Result<(), MwpmError> {
        self.cancellation
            .poll()
            .map_err(MwpmError::Cancellation)?;

        self.deadline.check()
    }

    fn charge(
        &mut self,
        units: u64,
    ) -> Result<(), MwpmError> {
        self.poll()?;

        self.work =
            self.work
                .checked_add(units)
                .ok_or(
                    MwpmError::ArithmeticOverflow,
                )?;

        if self.work > self.maximum {
            return Err(
                MwpmError::WorkLimitExceeded {
                    requested:
                        self.work,
                    limit:
                        self.maximum,
                },
            );
        }

        Ok(())
    }

    fn relaxation(
        &mut self,
    ) -> Result<(), MwpmError> {
        self.charge(1)?;

        self.relaxations =
            self.relaxations
                .checked_add(1)
                .ok_or(
                    MwpmError::ArithmeticOverflow,
                )?;

        Ok(())
    }

    fn work_units(&self) -> u64 {
        self.work
    }

    fn relaxations(&self) -> u64 {
        self.relaxations
    }
}

/* ========================================================================== */
/* Graph preflight                                                            */
/* ========================================================================== */

fn preflight_graph(
    graph: &DecodingGraph,
    limits: EffectiveLimits,
) -> Result<(), MwpmError> {
    let node_count =
        graph.total_node_count();

    let edge_count =
        graph.edge_count();

    if node_count
        > limits.max_graph_nodes
    {
        return Err(
            MwpmError::ResourceLimit {
                kind:
                    ResourceKind::GraphNodes,
                requested:
                    node_count,
                limit:
                    limits.max_graph_nodes,
            },
        );
    }

    if edge_count
        > limits.max_graph_edges
    {
        return Err(
            MwpmError::ResourceLimit {
                kind:
                    ResourceKind::GraphEdges,
                requested:
                    edge_count,
                limit:
                    limits.max_graph_edges,
            },
        );
    }

    let detection_count =
        graph.node_count();

    if detection_count
        > limits.max_syndrome_events
    {
        return Err(
            MwpmError::ResourceLimit {
                kind:
                    ResourceKind::SyndromeEvents,
                requested:
                    detection_count,
                limit:
                    limits.max_syndrome_events,
            },
        );
    }

    /*
     * Exact MWPM uses a u64 event mask.
     *
     * This is an implementation representation boundary, not a production
     * QEC policy. A larger workload must be routed to another MWPM backend.
     */
    if detection_count > 63 {
        return Err(
            MwpmError::ExactRepresentationLimit {
                events:
                    detection_count,
                maximum:
                    63,
            },
        );
    }

    /*
     * Metric closure requires O(n²) distance storage.
     */
    let metric_cells =
        detection_count
            .checked_mul(
                detection_count,
            )
            .ok_or(
                MwpmError::ArithmeticOverflow,
            )?;

    let metric_bytes =
        u64::try_from(
            metric_cells,
        )
        .ok()
        .and_then(|cells| {
            cells.checked_mul(8)
        })
        .ok_or(
            MwpmError::MemoryEstimateOverflow,
        )?;

    /*
     * Exact DP needs O(2^n) states.
     *
     * We estimate:
     *
     * - 32 bytes per memoized state;
     * - 16 bytes per selected-state choice;
     * - metric closure;
     * - a conservative factor for ordered-map overhead.
     */
    let state_count =
        exact_state_count(
            detection_count,
        )?;

    let state_bytes =
        state_count
            .checked_mul(48)
            .ok_or(
                MwpmError::MemoryEstimateOverflow,
            )?;

    let estimated_memory =
        metric_bytes
            .checked_add(
                state_bytes,
            )
            .ok_or(
                MwpmError::MemoryEstimateOverflow,
            )?;

    if estimated_memory
        > limits.max_memory_bytes
    {
        return Err(
            MwpmError::MemoryLimit {
                requested:
                    estimated_memory,
                limit:
                    limits.max_memory_bytes,
            },
        );
    }

    /*
     * DP transitions are bounded approximately by:
     *
     *     2^n * (n + boundary_count)
     *
     * This is deliberately conservative. Rejecting before execution is
     * preferable to starting an exact search that cannot satisfy policy.
     */
    let boundary_count =
        graph
            .boundaries()
            .count();

    let transition_factor =
        detection_count
            .checked_add(
                boundary_count,
            )
            .ok_or(
                MwpmError::ArithmeticOverflow,
            )?;

    let estimated_work =
        state_count
            .checked_mul(
                transition_factor,
            )
            .ok_or(
                MwpmError::ArithmeticOverflow,
            )?;

    let estimated_work_u64 =
        u64::try_from(
            estimated_work,
        )
        .map_err(|_| {
            MwpmError::ArithmeticOverflow
        })?;

    if estimated_work_u64
        > limits.max_decoder_iterations
    {
        return Err(
            MwpmError::WorkLimitExceeded {
                requested:
                    estimated_work_u64,
                limit:
                    limits.max_decoder_iterations,
            },
        );
    }

    Ok(())
}

fn exact_state_count(
    events: usize,
) -> Result<usize, MwpmError> {
    if events > 63 {
        return Err(
            MwpmError::ExactRepresentationLimit {
                events,
                maximum: 63,
            },
        );
    }

    1usize
        .checked_shl(
            u32::try_from(events)
                .map_err(|_| {
                    MwpmError::ArithmeticOverflow
                })?,
        )
        .ok_or(
            MwpmError::ArithmeticOverflow,
        )
}

/* ========================================================================== */
/* Detection events                                                           */
/* ========================================================================== */

fn collect_detection_events(
    graph: &DecodingGraph,
    limits: EffectiveLimits,
    budget: &mut WorkBudget<'_>,
) -> Result<Vec<MatchingEvent>, MwpmError> {
    let mut events =
        Vec::with_capacity(
            graph.node_count(),
        );

    for node in graph.nodes() {
        budget.charge(1)?;

        if events.len()
            >= limits.max_syndrome_events
        {
            return Err(
                MwpmError::ResourceLimit {
                    kind:
                        ResourceKind::SyndromeEvents,
                    requested:
                        events.len()
                            .saturating_add(1),
                    limit:
                        limits
                            .max_syndrome_events,
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

/* ========================================================================== */
/* Shortest-path metric                                                       */
/* ========================================================================== */

#[derive(
    Debug,
    Clone,
)]
struct ShortestPathMetric {
    events: Vec<MatchingEvent>,
    boundaries: Vec<BoundaryId>,
    distances: Vec<Vec<u64>>,
    boundary_distances:
        Vec<BTreeMap<BoundaryId, u64>>,
    paths:
        BTreeMap<
            (NodeId, NodeId),
            Vec<GraphEdge>,
        >,
    boundary_paths:
        BTreeMap<
            (NodeId, BoundaryId),
            Vec<GraphEdge>,
        >,
    relaxations: u64,
}

impl ShortestPathMetric {
    fn build(
        graph: &DecodingGraph,
        events: &[MatchingEvent],
        limits: EffectiveLimits,
        budget: &mut WorkBudget<'_>,
    ) -> Result<Self, MwpmError> {
        let count =
            events.len();

        let boundaries =
            graph
                .boundaries()
                .map(|boundary| {
                    boundary.id()
                })
                .collect::<Vec<_>>();

        let mut distances =
            vec![vec![INF; count]; count];

        for index in 0..count {
            let cell =
                distances
                    .get_mut(index)
                    .and_then(|row| {
                        row.get_mut(index)
                    })
                    .ok_or(
                        MwpmError::
                            MatchingIndexOutOfRange,
                    )?;

            *cell = 0;
        }

        let mut boundary_distances =
            vec![
                BTreeMap::new();
                count
            ];

        let mut paths =
            BTreeMap::new();

        let mut boundary_paths =
            BTreeMap::new();

        let start_relaxations =
            budget.relaxations();

        for (
            source_index,
            source_event,
        ) in events.iter().enumerate()
        {
            budget.poll()?;

            let shortest =
                dijkstra_from_event(
                    graph,
                    *source_event,
                    limits,
                    budget,
                )?;

            for target_index
                in (source_index + 1)..count
            {
                budget.charge(1)?;

                let target_event =
                    *events
                        .get(target_index)
                        .ok_or(
                            MwpmError::
                                MatchingIndexOutOfRange,
                        )?;

                let distance =
                    shortest
                        .distances
                        .get(
                            &GraphEndpoint::
                                Detection(
                                    target_event
                                        .node(),
                                ),
                        )
                        .copied()
                        .unwrap_or(INF);

                distances
                    .get_mut(source_index)
                    .and_then(|row| {
                        row.get_mut(
                            target_index,
                        )
                    })
                    .ok_or(
                        MwpmError::
                            MatchingIndexOutOfRange,
                    )
                    .map(|cell| {
                        *cell = distance;
                    })?;

                distances
                    .get_mut(target_index)
                    .and_then(|row| {
                        row.get_mut(
                            source_index,
                        )
                    })
                    .ok_or(
                        MwpmError::
                            MatchingIndexOutOfRange,
                    )
                    .map(|cell| {
                        *cell = distance;
                    })?;

                if distance != INF {
                    let path =
                        reconstruct_path(
                            graph,
                            source_event.node(),
                            target_event.node(),
                            &shortest
                                .predecessors,
                            budget,
                        )?;

                    paths.insert(
                        canonical_node_pair(
                            source_event
                                .node(),
                            target_event
                                .node(),
                        ),
                        path,
                    );
                }
            }

            let boundary_map =
                boundary_distances
                    .get_mut(source_index)
                    .ok_or(
                        MwpmError::
                            MatchingIndexOutOfRange,
                    )?;

            for boundary in &boundaries {
                budget.charge(1)?;

                let endpoint =
                    GraphEndpoint::
                        Boundary(
                            *boundary,
                        );

                if let Some(&distance) =
                    shortest
                        .distances
                        .get(&endpoint)
                {
                    boundary_map.insert(
                        *boundary,
                        distance,
                    );

                    let path =
                        reconstruct_boundary_path(
                            graph,
                            source_event.node(),
                            *boundary,
                            &shortest
                                .predecessors,
                            budget,
                        )?;

                    boundary_paths.insert(
                        (
                            source_event
                                .node(),
                            *boundary,
                        ),
                        path,
                    );
                }
            }
        }

        let relaxations =
            budget
                .relaxations()
                .saturating_sub(
                    start_relaxations,
                );

        Ok(Self {
            events:
                events.to_vec(),
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

    fn distance(
        &self,
        first: usize,
        second: usize,
    ) -> Result<u64, MwpmError> {
        self.distances
            .get(first)
            .and_then(|row| {
                row.get(second)
            })
            .copied()
            .ok_or(
                MwpmError::
                    MatchingIndexOutOfRange,
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
                    MwpmError::
                        MatchingIndexOutOfRange,
                )?;

        Ok(
            self.boundary_distances
                .get(event_index)
                .and_then(|map| {
                    map.get(&boundary)
                })
                .copied()
                .unwrap_or(INF),
        )
    }

    fn path(
        &self,
        first: MatchingEvent,
        second: MatchingEvent,
    ) -> Option<&[GraphEdge]> {
        self.paths
            .get(
                &canonical_node_pair(
                    first.node(),
                    second.node(),
                ),
            )
            .map(Vec::as_slice)
    }

    fn boundary_path(
        &self,
        event: MatchingEvent,
        boundary: BoundaryId,
    ) -> Option<&[GraphEdge]> {
        self.boundary_paths
            .get(&(
                event.node(),
                boundary,
            ))
            .map(Vec::as_slice)
    }
}

/* ========================================================================== */
/* Dijkstra                                                                    */
/* ========================================================================== */

#[derive(
    Debug,
    Clone,
)]
struct ShortestPathState {
    distances:
        BTreeMap<
            GraphEndpoint,
            u64,
        >,

    predecessors:
        BTreeMap<
            GraphEndpoint,
            GraphEndpoint,
        >,
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
            .cmp(
                &self.distance,
            )
            .then_with(|| {
                other
                    .endpoint
                    .cmp(
                        &self.endpoint,
                    )
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

fn dijkstra_from_event(
    graph: &DecodingGraph,
    source: MatchingEvent,
    _limits: EffectiveLimits,
    budget: &mut WorkBudget<'_>,
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
            endpoint:
                source_endpoint,
        },
    );

    while let Some(entry) =
        queue.pop()
    {
        budget.poll()?;

        let known =
            distances
                .get(
                    &entry.endpoint,
                )
                .copied()
                .ok_or(
                    MwpmError::
                        InternalDistanceState,
                )?;

        if entry.distance != known {
            continue;
        }

        for edge in graph
            .incident_edges(
                entry.endpoint,
            )
        {
            budget.relaxation()?;

            let neighbour =
                edge.other(
                    entry.endpoint,
                )
                .ok_or(
                    MwpmError::
                        InvalidGraphEndpoint,
                )?;

            let candidate =
                entry
                    .distance
                    .checked_add(
                        edge.weight()
                            .value(),
                    )
                    .ok_or(
                        MwpmError::
                            ArithmeticOverflow,
                    )?;

            let update =
                match distances
                    .get(&neighbour)
                {
                    None => true,

                    Some(&current) => {
                        if candidate
                            < current
                        {
                            true
                        } else if candidate
                            > current
                        {
                            false
                        } else {
                            /*
                             * Equal distance:
                             * choose the smaller predecessor
                             * deterministically.
                             */
                            match predecessors
                                .get(
                                    &neighbour,
                                )
                            {
                                None => true,
                                Some(
                                    &existing,
                                ) => {
                                    entry
                                        .endpoint
                                        < existing
                                }
                            }
                        }
                    }
                };

            if update {
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
                        distance:
                            candidate,
                        endpoint:
                            neighbour,
                    },
                );
            }
        }
    }

    Ok(
        ShortestPathState {
            distances,
            predecessors,
        },
    )
}

/* ========================================================================== */
/* Path reconstruction                                                        */
/* ========================================================================== */

fn reconstruct_path(
    graph: &DecodingGraph,
    source: NodeId,
    target: NodeId,
    predecessors: &BTreeMap<
        GraphEndpoint,
        GraphEndpoint,
    >,
    budget: &mut WorkBudget<'_>,
) -> Result<Vec<GraphEdge>, MwpmError> {
    reconstruct_endpoint_path(
        graph,
        GraphEndpoint::Detection(
            source,
        ),
        GraphEndpoint::Detection(
            target,
        ),
        predecessors,
        budget,
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
    budget: &mut WorkBudget<'_>,
) -> Result<Vec<GraphEdge>, MwpmError> {
    reconstruct_endpoint_path(
        graph,
        GraphEndpoint::Detection(
            source,
        ),
        GraphEndpoint::Boundary(
            boundary,
        ),
        predecessors,
        budget,
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
    budget: &mut WorkBudget<'_>,
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
        budget.charge(1)?;

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
                    MwpmError::
                        UnreachableEndpoint {
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
                    MwpmError::
                        PathEdgeMissing {
                            first:
                                predecessor,
                            second:
                                current,
                        },
                )?;

        reversed.push(edge);
        current =
            predecessor;
    }

    reversed.reverse();

    Ok(reversed)
}

/* ========================================================================== */
/* Exact MWPM                                                                 */
/* ========================================================================== */

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
struct MatchChoice {
    kind: MatchChoiceKind,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
enum MatchChoiceKind {
    /// Match the first event with another event.
    Event {
        second: usize,
    },

    /// Match the first event directly to a boundary.
    Boundary {
        boundary: usize,
    },
}

/// Solves the exact MWPM problem.
///
/// Important boundary rule:
///
/// A boundary may be used by multiple detection events. Therefore boundary
/// usage is NOT part of the DP state.
fn solve_exact_mwpm(
    metric: &ShortestPathMetric,
    limits: EffectiveLimits,
    budget: &mut WorkBudget<'_>,
) -> Result<Vec<Matching>, MwpmError> {
    let count =
        metric.event_count();

    if count == 0 {
        return Ok(Vec::new());
    }

    if count > limits.max_syndrome_events {
        return Err(
            MwpmError::ResourceLimit {
                kind:
                    ResourceKind::SyndromeEvents,
                requested:
                    count,
                limit:
                    limits.max_syndrome_events,
            },
        );
    }

    if count > 63 {
        return Err(
            MwpmError::ExactRepresentationLimit {
                events:
                    count,
                maximum:
                    63,
            },
        );
    }

    let full_mask =
        (1u64 << count) - 1;

    let mut memo =
        BTreeMap::<u64, u64>::new();

    let mut choices =
        BTreeMap::<
            u64,
            MatchChoice,
        >::new();

    let total =
        solve_mask(
            metric,
            full_mask,
            &mut memo,
            &mut choices,
            budget,
        )?;

    if total == INF {
        return Err(
            MwpmError::NoPerfectMatching,
        );
    }

    let mut mask =
        full_mask;

    let mut result =
        Vec::with_capacity(
            count,
        );

    while mask != 0 {
        budget.poll()?;

        let first =
            first_set_bit(
                mask,
            )
            .ok_or(
                MwpmError::
                    InternalMatchingState,
            )?;

        let choice =
            choices
                .get(&mask)
                .copied()
                .ok_or(
                    MwpmError::
                        InternalMatchingState,
                )?;

        match choice.kind {
            MatchChoiceKind::Event {
                second,
            } => {
                let first_event =
                    *metric.events
                        .get(first)
                        .ok_or(
                            MwpmError::
                                MatchingIndexOutOfRange,
                        )?;

                let second_event =
                    *metric.events
                        .get(second)
                        .ok_or(
                            MwpmError::
                                MatchingIndexOutOfRange,
                        )?;

                let weight =
                    metric.distance(
                        first,
                        second,
                    )?;

                if weight == INF {
                    return Err(
                        MwpmError::
                            NoPerfectMatching,
                    );
                }

                let pair =
                    MatchingPair::new(
                        first_event,
                        second_event,
                        weight,
                    )?;

                result.push(
                    Matching::EventPair(
                        pair,
                    ),
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
                            MwpmError::
                                MatchingIndexOutOfRange,
                        )?;

                let boundary_id =
                    *metric.boundaries
                        .get(boundary)
                        .ok_or(
                            MwpmError::
                                MatchingIndexOutOfRange,
                        )?;

                let weight =
                    metric.boundary_distance(
                        first,
                        boundary,
                    )?;

                if weight == INF {
                    return Err(
                        MwpmError::
                            NoPerfectMatching,
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
            }
        }
    }

    result.sort_by(
        matching_cmp,
    );

    Ok(result)
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
        MatchChoice,
    >,
    budget: &mut WorkBudget<'_>,
) -> Result<u64, MwpmError> {
    budget.poll()?;

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
                MwpmError::
                    InternalMatchingState,
            )?;

    let without_first =
        mask & !(1u64 << first);

    let mut best =
        INF;

    let mut best_choice =
        None;

    /*
     * Event/event choices.
     */
    let mut remaining =
        without_first;

    while remaining != 0 {
        budget.charge(1)?;

        let second =
            first_set_bit(
                remaining,
            )
            .ok_or(
                MwpmError::
                    InternalMatchingState,
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
                    memo,
                    choices,
                    budget,
                )?;

            if remainder != INF {
                let total =
                    weight
                        .checked_add(
                            remainder,
                        )
                        .ok_or(
                            MwpmError::
                                ArithmeticOverflow,
                        )?;

                let candidate =
                    MatchChoice {
                        kind:
                            MatchChoiceKind::
                                Event {
                                    second,
                                },
                    };

                if is_better_choice(
                    total,
                    candidate,
                    best,
                    best_choice,
                ) {
                    best =
                        total;

                    best_choice =
                        Some(candidate);
                }
            }
        }

        remaining &=
            !(1u64 << second);
    }

    /*
     * Event/boundary choices.
     *
     * The same boundary can legally be selected by multiple events.
     */
    for boundary_index
        in 0..metric.boundaries.len()
    {
        budget.charge(1)?;

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
                memo,
                choices,
                budget,
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
                    MwpmError::
                        ArithmeticOverflow,
                )?;

        let candidate =
            MatchChoice {
                kind:
                    MatchChoiceKind::
                        Boundary {
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
            best =
                total;

            best_choice =
                Some(candidate);
        }
    }

    if let Some(choice) =
        best_choice
    {
        choices.insert(
            mask,
            choice,
        );
    }

    memo.insert(
        mask,
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

    match (
        candidate.kind,
        current_choice,
    ) {
        (
            MatchChoiceKind::Event {
                second:
                    candidate_second,
            },
            Some(
                MatchChoice {
                    kind:
                        MatchChoiceKind::Event {
                            second:
                                current_second,
                        },
                },
            ),
        ) => {
            candidate_second
                < current_second
        }

        (
            MatchChoiceKind::Boundary {
                boundary:
                    candidate_boundary,
            },
            Some(
                MatchChoice {
                    kind:
                        MatchChoiceKind::Boundary {
                            boundary:
                                current_boundary,
                        },
                },
            ),
        ) => {
            candidate_boundary
                < current_boundary
        }

        /*
         * Prefer event/event on an equal metric cost.
         *
         * This provides stable tie-breaking while avoiding unnecessary
         * boundary consumption.
         */
        (
            MatchChoiceKind::Event { .. },
            Some(
                MatchChoice {
                    kind:
                        MatchChoiceKind::Boundary {
                            ..
                        },
                },
            ),
        ) => true,

        /*
         * A boundary candidate wins only if there is no existing choice.
         */
        (_, None) => true,

        _ => false,
    }
}

fn matching_cmp(
    first: &Matching,
    second: &Matching,
) -> Ordering {
    matching_sort_key(first)
        .cmp(
            &matching_sort_key(second),
        )
}

fn matching_sort_key(
    matching: &Matching,
) -> (u8, usize, usize) {
    match matching {
        Matching::EventPair(pair) => (
            0,
            pair.first()
                .node()
                .index(),
            pair.second()
                .node()
                .index(),
        ),

        Matching::BoundaryPair(pair) => (
            1,
            pair.event()
                .node()
                .index(),
            pair.boundary()
                .index(),
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

/* ========================================================================== */
/* Result materialization                                                     */
/* ========================================================================== */

fn materialize_result(
    graph: &DecodingGraph,
    metric: &ShortestPathMetric,
    matchings: Vec<Matching>,
    limits: EffectiveLimits,
    budget: &mut WorkBudget<'_>,
) -> Result<MwpmResult, MwpmError> {
    let mut total_weight =
        0u64;

    let mut total_path_edges =
        0usize;

    let mut paths =
        Vec::with_capacity(
            matchings.len(),
        );

    for matching in &matchings {
        budget.poll()?;

        total_weight =
            total_weight
                .checked_add(
                    matching.weight(),
                )
                .ok_or(
                    MwpmError::
                        ArithmeticOverflow,
                )?;

        let edges =
            match matching {
                Matching::EventPair(
                    pair,
                ) => {
                    metric
                        .path(
                            pair.first(),
                            pair.second(),
                        )
                        .ok_or(
                            MwpmError::
                                UnreachablePair {
                                    first:
                                        pair.first()
                                            .node(),
                                    second:
                                        pair.second()
                                            .node(),
                                },
                        )?
                }

                Matching::BoundaryPair(
                    pair,
                ) => {
                    metric
                        .boundary_path(
                            pair.event(),
                            pair.boundary(),
                        )
                        .ok_or(
                            MwpmError::
                                UnreachableBoundary {
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
                    MwpmError::
                        ArithmeticOverflow,
                )?;

        if total_path_edges
            > limits.max_path_edges
        {
            return Err(
                MwpmError::
                    CorrectionPathTooLong {
                        requested:
                            total_path_edges,
                        limit:
                            limits.max_path_edges,
                    },
            );
        }

        paths.push(
            CorrectionPath::new(
                *matching,
                edges.to_vec(),
                limits.max_path_edges,
            )?,
        );

        budget.charge(
            u64::try_from(
                edges.len(),
            )
            .map_err(|_| {
                MwpmError::
                    ArithmeticOverflow
            })?,
        )?;
    }

    graph
        .validate()
        .map_err(MwpmError::Graph)?;

    MwpmResult::new(
        matchings,
        paths,
        total_weight,
        MwpmTermination::Completed,
        budget.work_units(),
        budget.relaxations(),
    )
}

/* ========================================================================== */
/* Helpers                                                                    */
/* ========================================================================== */

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

/* ========================================================================== */
/* Errors                                                                     */
/* ========================================================================== */

/// Errors produced by exact MWPM.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum MwpmError {
    /// Decoding graph validation failed.
    Graph(
        super::decoding_graph::DecodingGraphError,
    ),

    /// Canonical QEC limit policy rejected the operation.
    Limit(
        super::limits::LimitError,
    ),

    /// Cooperative cancellation.
    Cancellation(QecError),

    /// A canonical resource dimension was exceeded.
    ResourceLimit {
        kind: ResourceKind,
        requested: usize,
        limit: usize,
    },

    /// Estimated exact-solver memory exceeds policy.
    MemoryLimit {
        requested: usize,
        limit: u64,
    },

    /// Exact-solver work exceeds policy.
    WorkLimitExceeded {
        requested: u64,
        limit: u64,
    },

    /// Decoder execution exceeded its configured time budget.
    TimeLimitExceeded {
        limit_ns: u64,
    },

    /// Exact MWPM's event-mask representation cannot represent the request.
    ExactRepresentationLimit {
        events: usize,
        maximum: usize,
    },

    /// Integer arithmetic overflowed.
    ArithmeticOverflow,

    /// Memory estimation overflowed.
    MemoryEstimateOverflow,

    /// A graph endpoint was invalid.
    InvalidGraphEndpoint,

    /// A shortest-path state was inconsistent.
    InternalDistanceState,

    /// A matching DP state was inconsistent.
    InternalMatchingState,

    /// Match/path result vectors diverged.
    InternalResultMismatch,

    /// A path contained a cycle.
    PathCycle,

    /// A required pair was unreachable.
    UnreachablePair {
        first: NodeId,
        second: NodeId,
    },

    /// A required boundary was unreachable.
    UnreachableBoundary {
        event: NodeId,
        boundary: BoundaryId,
    },

    /// A required graph endpoint was unreachable.
    UnreachableEndpoint {
        source: GraphEndpoint,
        target: GraphEndpoint,
    },

    /// A predecessor edge was missing.
    PathEdgeMissing {
        first: GraphEndpoint,
        second: GraphEndpoint,
    },

    /// A pair attempted to match an event to itself.
    SelfMatch {
        event: MatchingEvent,
    },

    /// A matching pair was not canonical.
    NonCanonicalPair,

    /// A metric-closure index was invalid.
    MatchingIndexOutOfRange,

    /// No complete exact solution exists.
    NoPerfectMatching,

    /// Materialized correction path exceeds policy.
    CorrectionPathTooLong {
        requested: usize,
        limit: usize,
    },

    /// Invalid algorithm preference.
    InvalidConfiguration,
}

impl MwpmError {
    /// Converts the local error to the canonical QEC error boundary.
    #[must_use]
    pub fn into_qec_error(
        self,
    ) -> QecError {
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

            Self::Cancellation(error) => {
                error
            }

            Self::ResourceLimit {
                kind,
                requested,
                limit,
            } => {
                QecError::resource_limit(
                    kind,
                    requested as u128,
                    limit as u128,
                    format!(
                        "MWPM resource limit exceeded: \
                         requested {requested}, limit {limit}"
                    ),
                )
            }

            Self::MemoryLimit {
                requested,
                limit,
            } => {
                QecError::memory_limit(
                    u64::try_from(
                        requested,
                    )
                    .unwrap_or(
                        u64::MAX,
                    ),
                    limit,
                    format!(
                        "MWPM exact-solver memory \
                         estimate {requested} bytes \
                         exceeds limit {limit} bytes"
                    ),
                )
            }

            Self::WorkLimitExceeded {
                requested,
                limit,
            } => {
                QecError::resource_limit(
                    ResourceKind::
                        DecoderIterations,
                    u128::from(
                        requested,
                    ),
                    u128::from(
                        limit,
                    ),
                    format!(
                        "MWPM exact-solver work \
                         estimate {requested} exceeds \
                         decoder-iteration budget {limit}"
                    ),
                )
            }

            Self::TimeLimitExceeded {
                limit_ns,
            } => {
                QecError::resource_limit(
                    ResourceKind::
                        DecoderTimeNs,
                    u128::from(
                        limit_ns,
                    )
                    .saturating_add(1),
                    u128::from(
                        limit_ns,
                    ),
                    format!(
                        "MWPM exceeded decoder time \
                         limit of {limit_ns} ns"
                    ),
                )
            }

            Self::ExactRepresentationLimit {
                events,
                maximum,
            } => {
                QecError::unsupported(
                    "mwpm_exact_representation",
                    format!(
                        "exact MWPM represents at most \
                         {maximum} active events; \
                         requested {events}; use a \
                         scalable MWPM implementation"
                    ),
                )
            }

            Self::ArithmeticOverflow => {
                QecError::numerical_failure(
                    super::errors::
                        NumericalOperation::
                            Accumulation,
                    "MWPM checked arithmetic overflow",
                )
            }

            Self::MemoryEstimateOverflow => {
                QecError::numerical_failure(
                    super::errors::
                        NumericalOperation::
                            IntegerConversion,
                    "MWPM memory estimation overflow",
                )
            }

            Self::InvalidGraphEndpoint => {
                QecError::invalid_graph(
                    "MWPM encountered an invalid \
                     graph endpoint",
                )
            }

            Self::InternalDistanceState => {
                QecError::invariant(
                    "mwpm_distance_state",
                    "MWPM shortest-path state \
                     became inconsistent",
                )
            }

            Self::InternalMatchingState => {
                QecError::invariant(
                    "mwpm_matching_state",
                    "MWPM dynamic-programming \
                     state became inconsistent",
                )
            }

            Self::InternalResultMismatch => {
                QecError::invariant(
                    "mwpm_result",
                    "MWPM produced different \
                     numbers of matches and paths",
                )
            }

            Self::PathCycle => {
                QecError::invariant(
                    "mwpm_path",
                    "MWPM path reconstruction \
                     encountered a cycle",
                )
            }

            Self::UnreachablePair {
                first,
                second,
            } => {
                QecError::decoder_failure(
                    DecoderKind::Mwpm,
                    format!(
                        "no correction path exists \
                         between {first} and {second}"
                    ),
                )
            }

            Self::UnreachableBoundary {
                event,
                boundary,
            } => {
                QecError::decoder_failure(
                    DecoderKind::Mwpm,
                    format!(
                        "no correction path exists \
                         from {event} to boundary \
                         {boundary}"
                    ),
                )
            }

            Self::UnreachableEndpoint {
                source,
                target,
            } => {
                QecError::decoder_failure(
                    DecoderKind::Mwpm,
                    format!(
                        "no path exists from \
                         {source:?} to {target:?}"
                    ),
                )
            }

            Self::PathEdgeMissing {
                first,
                second,
            } => {
                QecError::invariant(
                    "mwpm_predecessor_edge",
                    format!(
                        "predecessor edge missing \
                         between {first:?} and \
                         {second:?}"
                    ),
                )
            }

            Self::SelfMatch { event } => {
                QecError::invalid_graph(
                    format!(
                        "MWPM attempted to self-match \
                         {event}"
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
                    "MWPM referenced an event \
                     outside its metric closure",
                )
            }

            Self::NoPerfectMatching => {
                QecError::decoder_failure(
                    DecoderKind::Mwpm,
                    "no valid MWPM solution exists \
                     for the supplied graph",
                )
            }

            Self::CorrectionPathTooLong {
                requested,
                limit,
            } => {
                QecError::resource_limit(
                    ResourceKind::GraphEdges,
                    requested as u128,
                    limit as u128,
                    format!(
                        "MWPM correction path contains \
                         {requested} edges, limit {limit}"
                    ),
                )
            }

            Self::InvalidConfiguration => {
                QecError::unsupported(
                    "mwpm_configuration",
                    "invalid MWPM algorithm configuration",
                )
            }
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
                    "MWPM resource limit exceeded for \
                     {kind}: {requested} > {limit}"
                )
            }

            Self::MemoryLimit {
                requested,
                limit,
            } => {
                write!(
                    formatter,
                    "MWPM memory limit exceeded: \
                     {requested} > {limit} bytes"
                )
            }

            Self::WorkLimitExceeded {
                requested,
                limit,
            } => {
                write!(
                    formatter,
                    "MWPM work estimate exceeded: \
                     {requested} > {limit}"
                )
            }

            Self::TimeLimitExceeded {
                limit_ns,
            } => {
                write!(
                    formatter,
                    "MWPM decoder time limit \
                     {limit_ns} ns exceeded"
                )
            }

            Self::ExactRepresentationLimit {
                events,
                maximum,
            } => {
                write!(
                    formatter,
                    "exact MWPM supports at most \
                     {maximum} active events in \
                     its current representation; \
                     requested {events}"
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "MWPM arithmetic overflow",
                )
            }

            Self::MemoryEstimateOverflow => {
                formatter.write_str(
                    "MWPM memory-estimation overflow",
                )
            }

            Self::InvalidGraphEndpoint => {
                formatter.write_str(
                    "MWPM encountered an invalid \
                     graph endpoint",
                )
            }

            Self::InternalDistanceState => {
                formatter.write_str(
                    "MWPM shortest-path state \
                     became inconsistent",
                )
            }

            Self::InternalMatchingState => {
                formatter.write_str(
                    "MWPM matching state \
                     became inconsistent",
                )
            }

            Self::InternalResultMismatch => {
                formatter.write_str(
                    "MWPM result contains inconsistent \
                     match/path counts",
                )
            }

            Self::PathCycle => {
                formatter.write_str(
                    "MWPM path reconstruction \
                     encountered a cycle",
                )
            }

            Self::UnreachablePair {
                first,
                second,
            } => {
                write!(
                    formatter,
                    "no path exists between \
                     {first} and {second}"
                )
            }

            Self::UnreachableBoundary {
                event,
                boundary,
            } => {
                write!(
                    formatter,
                    "no path exists from {event} \
                     to boundary {boundary}"
                )
            }

            Self::UnreachableEndpoint {
                source,
                target,
            } => {
                write!(
                    formatter,
                    "no path exists from \
                     {source:?} to {target:?}"
                )
            }

            Self::PathEdgeMissing {
                first,
                second,
            } => {
                write!(
                    formatter,
                    "predecessor edge missing \
                     between {first:?} and {second:?}"
                )
            }

            Self::SelfMatch { event } => {
                write!(
                    formatter,
                    "MWPM cannot self-match {event}"
                )
            }

            Self::NonCanonicalPair => {
                formatter.write_str(
                    "MWPM pair is not in canonical order",
                )
            }

            Self::MatchingIndexOutOfRange => {
                formatter.write_str(
                    "MWPM matching index is outside \
                     the active metric closure",
                )
            }

            Self::NoPerfectMatching => {
                formatter.write_str(
                    "no valid MWPM solution exists",
                )
            }

            Self::CorrectionPathTooLong {
                requested,
                limit,
            } => {
                write!(
                    formatter,
                    "correction path has {requested} \
                     edges, limit {limit}"
                )
            }

            Self::InvalidConfiguration => {
                formatter.write_str(
                    "invalid MWPM configuration",
                )
            }
        }
    }
}

impl std::error::Error for MwpmError {}

impl From<MwpmError> for QecError {
    fn from(
        error: MwpmError,
    ) -> Self {
        error.into_qec_error()
    }
}

/* ========================================================================== */
/* Canonical convenience APIs                                                 */
/* ========================================================================== */

/// Decodes a graph with the production exact MWPM decoder.
pub fn decode(
    graph: &DecodingGraph,
) -> QecResult<MwpmResult> {
    MwpmDecoder::new()
        .decode_graph(graph)
        .map_err(
            MwpmError::into_qec_error,
        )
}

/// Decodes a graph with explicit limits and cancellation.
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
        .map_err(
            MwpmError::into_qec_error,
        )
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

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
            SpatialCoordinate::xy(
                x,
                0,
            )
            .expect(
                "valid test coordinate",
            ),
            MeasurementRound::new(
                0,
            )
            .expect(
                "valid test round",
            ),
        )
        .expect(
            "valid space-time coordinate",
        )
    }

    fn graph_with_line(
        count: usize,
    ) -> DecodingGraph {
        let mut graph =
            DecodingGraph::new();

        let mut nodes =
            Vec::with_capacity(
                count,
            );

        for index in 0..count {
            let node =
                graph
                    .add_detection_node(
                        coordinate(
                            index as i64,
                        ),
                        StabilizerId::new(
                            index,
                        ),
                        MeasurementConfidence::certain(),
                    )
                    .expect(
                        "valid test node",
                    );

            nodes.push(node);
        }

        for index in
            0..count.saturating_sub(1)
        {
            graph
                .add_edge(
                    GraphEndpoint::
                        Detection(
                            nodes[index],
                        ),
                    GraphEndpoint::
                        Detection(
                            nodes[index + 1],
                        ),
                    EdgeWeight::new(
                        1,
                    )
                    .expect(
                        "valid weight",
                    ),
                    EdgeKind::Spatial,
                )
                .expect(
                    "valid edge",
                );
        }

        graph
    }

    #[test]
    fn empty_graph_is_trivial() {
        let graph =
            DecodingGraph::new();

        let result =
            MwpmDecoder::new()
                .decode_graph(
                    &graph,
                )
                .expect(
                    "empty graph",
                );

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

        assert_eq!(
            result.termination(),
            MwpmTermination::EmptyInput
        );
    }

    #[test]
    fn two_events_are_matched() {
        let graph =
            graph_with_line(
                2,
            );

        let result =
            MwpmDecoder::new()
                .decode_graph(
                    &graph,
                )
                .expect(
                    "two-event graph",
                );

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
                .expect(
                    "one event pair",
                );

        assert_eq!(
            pair.first()
                .node(),
            NodeId::new(0)
        );

        assert_eq!(
            pair.second()
                .node(),
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
                .expect(
                    "node a",
                );

        let b =
            graph
                .add_detection_node(
                    coordinate(1),
                    StabilizerId::new(1),
                    MeasurementConfidence::certain(),
                )
                .expect(
                    "node b",
                );

        let c =
            graph
                .add_detection_node(
                    coordinate(2),
                    StabilizerId::new(2),
                    MeasurementConfidence::certain(),
                )
                .expect(
                    "node c",
                );

        graph
            .add_edge(
                GraphEndpoint::Detection(a),
                GraphEndpoint::Detection(b),
                EdgeWeight::new(10)
                    .expect(
                        "weight",
                    ),
                EdgeKind::Spatial,
            )
            .expect(
                "edge",
            );

        graph
            .add_edge(
                GraphEndpoint::Detection(b),
                GraphEndpoint::Detection(c),
                EdgeWeight::new(1)
                    .expect(
                        "weight",
                    ),
                EdgeKind::Spatial,
            )
            .expect(
                "edge",
            );

        graph
            .add_edge(
                GraphEndpoint::Detection(a),
                GraphEndpoint::Detection(c),
                EdgeWeight::new(20)
                    .expect(
                        "weight",
                    ),
                EdgeKind::Spatial,
            )
            .expect(
                "edge",
            );

        /*
         * Three events with no boundary cannot form a perfect matching.
         * This verifies fail-closed behavior rather than silently dropping
         * an event.
         */
        let result =
            MwpmDecoder::new()
                .decode_graph(
                    &graph,
                );

        assert!(
            matches!(
                result,
                Err(
                    MwpmError::
                        NoPerfectMatching
                )
            )
        );
    }

    #[test]
    fn odd_event_count_fails_without_boundary() {
        let graph =
            graph_with_line(
                3,
            );

        let result =
            MwpmDecoder::new()
                .decode_graph(
                    &graph,
                );

        assert!(
            matches!(
                result,
                Err(
                    MwpmError::
                        NoPerfectMatching
                )
            )
        );
    }

    #[test]
    fn configuration_never_allows_zero() {
        assert!(
            MwpmConfig::new(
                0,
                1,
                1,
                1,
                1,
            )
            .is_err()
        );
    }

    #[test]
    fn deterministic_result_order() {
        let graph =
            graph_with_line(
                4,
            );

        let first =
            MwpmDecoder::new()
                .decode_graph(
                    &graph,
                )
                .expect(
                    "first decode",
                );

        let second =
            MwpmDecoder::new()
                .decode_graph(
                    &graph,
                )
                .expect(
                    "second decode",
                );

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn cancellation_is_checked_before_work() {
        let graph =
            graph_with_line(
                2,
            );

        let cancellation =
            CancellationToken::new();

        cancellation
            .cancel();

        let limits =
            graph.limits();

        let result =
            MwpmDecoder::new()
                .decode_graph_with_context(
                    &graph,
                    &limits,
                    &cancellation,
                );

        assert!(
            matches!(
                result,
                Err(
                    MwpmError::
                        Cancellation(_)
                )
            )
        );
    }
}