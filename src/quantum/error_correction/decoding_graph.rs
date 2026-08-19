//! Zamani Quantum Error Correction — Deterministic Space-Time Decoding Graph.
//!
//! This module is the intermediate representation between syndrome processing
//! and a decoder such as MWPM or Union-Find.
//!
//! Design:
//!
//! ```text
//! DetectionEvent
//!      │
//!      ▼
//! DecodingGraph
//!      │
//!      ├── DetectionNode
//!      ├── BoundaryNode
//!      └── GraphEdge
//!             │
//!             ▼
//!       MWPM / Union-Find
//! ```
//!
//! The graph is deliberately not a decoder.
//!
//! Responsibilities:
//! - represent space-time detection events;
//! - represent physical/logical boundaries;
//! - represent weighted candidate error paths;
//! - enforce QecLimits before allocation;
//! - perform memory preflight;
//! - validate graph invariants;
//! - reject duplicate nodes and edges;
//! - provide deterministic iteration;
//! - provide checked coordinate arithmetic;
//! - provide bounded probability-to-weight conversion;
//! - expose a stable API to decoder implementations.
//!
//! Non-responsibilities:
//! - syndrome extraction;
//! - stabilizer algebra;
//! - noise generation;
//! - matching;
//! - correction application;
//! - logical-error classification;
//! - QPU execution.
//!
//! Resource policy:
//!
//! ```text
//! QecLimits
//!     │
//!     ▼
//! DecodingGraph preflight
//!     │
//!     ▼
//! graph allocation
//! ```
//!
//! No production graph-size ceiling is defined locally. Representation
//! invariants such as coordinate dimensionality and maximum encoded weight
//! remain local because they describe the graph representation itself rather
//! than execution policy.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::limits::{LimitError, QecLimits};
use super::syndrome::{
    DetectionEvent,
    MeasurementConfidence,
    MeasurementRound,
    StabilizerId,
};

// ============================================================================
// Representation invariants
// ============================================================================

/// Maximum number of spatial dimensions represented by one coordinate.
///
/// This is a representation invariant, not an execution resource policy.
pub const MAX_SPATIAL_DIMENSIONS: usize = 8;

/// Maximum absolute coordinate component representable by the graph.
///
/// This prevents malformed external coordinates from creating pathological
/// arithmetic while leaving workload sizing to `QecLimits`.
pub const MAX_COORDINATE_ABS: i64 = 1_000_000_000;

/// Maximum valid graph round.
///
/// `MeasurementRound` already validates its own range. This constant exists
/// as a graph-level invariant for defensive validation.
pub const MAX_GRAPH_ROUND: u64 = u64::MAX - 1;

/// Maximum valid graph timestamp.
///
/// `MeasurementTimestamp` already validates its own range.
pub const MAX_GRAPH_TIMESTAMP: u64 = u64::MAX - 1;

/// Fixed-point probability scale.
///
/// ```text
/// 0                   = 0
/// 1_000_000_000_000   = 1.0
/// ```
pub const PROBABILITY_SCALE: u64 = 1_000_000_000_000;

/// Maximum finite encoded edge weight.
///
/// This prevents pathological values from reaching decoders.
pub const MAX_WEIGHT: u64 = 1_000_000_000_000_000;

/// Conservative per-node memory estimate used by preflight.
///
/// This is deliberately an estimate, not an allocator measurement.
const ESTIMATED_NODE_BYTES: u64 = 128;

/// Conservative per-boundary memory estimate used by preflight.
const ESTIMATED_BOUNDARY_BYTES: u64 = 96;

/// Conservative per-edge memory estimate used by preflight.
const ESTIMATED_EDGE_BYTES: u64 = 96;

/// Conservative graph base memory estimate.
const ESTIMATED_GRAPH_BASE_BYTES: u64 = 1024;

// ============================================================================
// Spatial coordinate
// ============================================================================

/// Spatial coordinate in the decoding lattice.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct SpatialCoordinate {
    values: Vec<i64>,
}

impl SpatialCoordinate {
    /// Creates a validated spatial coordinate.
    pub fn new(
        values: Vec<i64>,
    ) -> Result<Self, DecodingGraphError> {
        if values.is_empty() {
            return Err(
                DecodingGraphError::EmptyCoordinate,
            );
        }

        if values.len() > MAX_SPATIAL_DIMENSIONS {
            return Err(
                DecodingGraphError::TooManyDimensions {
                    dimensions: values.len(),
                    limit: MAX_SPATIAL_DIMENSIONS,
                },
            );
        }

        for &value in &values {
            let absolute = value
                .checked_abs()
                .ok_or(
                    DecodingGraphError::CoordinateOutOfRange {
                        value,
                    },
                )?;

            if absolute > MAX_COORDINATE_ABS {
                return Err(
                    DecodingGraphError::CoordinateOutOfRange {
                        value,
                    },
                );
            }
        }

        Ok(Self { values })
    }

    /// Creates a two-dimensional coordinate.
    pub fn xy(
        x: i64,
        y: i64,
    ) -> Result<Self, DecodingGraphError> {
        Self::new(vec![x, y])
    }

    /// Creates a three-dimensional coordinate.
    pub fn xyz(
        x: i64,
        y: i64,
        z: i64,
    ) -> Result<Self, DecodingGraphError> {
        Self::new(vec![x, y, z])
    }

    /// Returns the coordinate dimensionality.
    pub fn dimensions(&self) -> usize {
        self.values.len()
    }

    /// Returns one coordinate component.
    pub fn get(
        &self,
        dimension: usize,
    ) -> Option<i64> {
        self.values.get(dimension).copied()
    }

    /// Returns all coordinate components.
    pub fn values(&self) -> &[i64] {
        &self.values
    }

    /// Computes Manhattan distance using checked arithmetic.
    pub fn manhattan_distance(
        &self,
        other: &Self,
    ) -> Result<u64, DecodingGraphError> {
        if self.dimensions() != other.dimensions() {
            return Err(
                DecodingGraphError::DimensionMismatch,
            );
        }

        let mut distance = 0u64;

        for (&left, &right) in
            self.values.iter().zip(other.values.iter())
        {
            let delta = left
                .checked_sub(right)
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

            let absolute = delta
                .checked_abs()
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

            distance = distance
                .checked_add(absolute as u64)
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;
        }

        Ok(distance)
    }
}

impl fmt::Display for SpatialCoordinate {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "(")?;

        for (index, value) in
            self.values.iter().enumerate()
        {
            if index != 0 {
                write!(f, ",")?;
            }

            write!(f, "{value}")?;
        }

        write!(f, ")")
    }
}

// ============================================================================
// Space-time coordinate
// ============================================================================

/// A position in the decoding space-time lattice.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct SpaceTimeCoordinate {
    spatial: SpatialCoordinate,
    round: MeasurementRound,
}

impl SpaceTimeCoordinate {
    /// Creates a validated space-time coordinate.
    pub fn new(
        spatial: SpatialCoordinate,
        round: MeasurementRound,
    ) -> Result<Self, DecodingGraphError> {
        if round.value() > MAX_GRAPH_ROUND {
            return Err(
                DecodingGraphError::InvalidRound {
                    round: round.value(),
                },
            );
        }

        Ok(Self {
            spatial,
            round,
        })
    }

    /// Returns the spatial coordinate.
    pub fn spatial(
        &self,
    ) -> &SpatialCoordinate {
        &self.spatial
    }

    /// Returns the measurement round.
    pub const fn round(
        &self,
    ) -> MeasurementRound {
        self.round
    }
}

// ============================================================================
// Node identifiers
// ============================================================================

/// Stable detection-node identifier.
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
pub struct NodeId(usize);

impl NodeId {
    /// Creates an identifier.
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "n{}", self.0)
    }
}

/// Stable boundary identifier.
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
pub struct BoundaryId(usize);

impl BoundaryId {
    /// Creates an identifier.
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for BoundaryId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "b{}", self.0)
    }
}

// ============================================================================
// Detection node
// ============================================================================

/// Detection event represented in the decoding graph.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct DetectionNode {
    id: NodeId,
    coordinate: SpaceTimeCoordinate,
    stabilizer: StabilizerId,
    confidence: MeasurementConfidence,
}

impl DetectionNode {
    /// Creates a detection node.
    pub fn new(
        id: NodeId,
        coordinate: SpaceTimeCoordinate,
        stabilizer: StabilizerId,
        confidence: MeasurementConfidence,
    ) -> Self {
        Self {
            id,
            coordinate,
            stabilizer,
            confidence,
        }
    }

    pub const fn id(
        &self,
    ) -> NodeId {
        self.id
    }

    pub fn coordinate(
        &self,
    ) -> &SpaceTimeCoordinate {
        &self.coordinate
    }

    pub const fn stabilizer(
        &self,
    ) -> StabilizerId {
        self.stabilizer
    }

    pub const fn confidence(
        &self,
    ) -> MeasurementConfidence {
        self.confidence
    }
}

// ============================================================================
// Boundary node
// ============================================================================

/// A physical or logical decoding boundary.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct BoundaryNode {
    id: BoundaryId,
    coordinate: SpaceTimeCoordinate,
}

impl BoundaryNode {
    pub fn new(
        id: BoundaryId,
        coordinate: SpaceTimeCoordinate,
    ) -> Self {
        Self {
            id,
            coordinate,
        }
    }

    pub const fn id(
        &self,
    ) -> BoundaryId {
        self.id
    }

    pub fn coordinate(
        &self,
    ) -> &SpaceTimeCoordinate {
        &self.coordinate
    }
}

// ============================================================================
// Graph endpoint
// ============================================================================

/// Endpoint of a decoding graph edge.
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
pub enum GraphEndpoint {
    /// Detection event.
    Detection(NodeId),

    /// Physical or logical boundary.
    Boundary(BoundaryId),
}

impl GraphEndpoint {
    pub const fn is_detection(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Detection(_)
        )
    }

    pub const fn is_boundary(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Boundary(_)
        )
    }
}

// ============================================================================
// Edge kind
// ============================================================================

/// Semantic type of a candidate error path.
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
pub enum EdgeKind {
    /// Spatial propagation.
    Spatial,

    /// Temporal propagation.
    Temporal,

    /// Combined spatial/temporal propagation.
    SpaceTime,

    /// Connection to a boundary.
    Boundary,

    /// Backend-specific edge.
    Custom,
}

// ============================================================================
// Edge weight
// ============================================================================

/// Bounded deterministic edge weight.
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
pub struct EdgeWeight(u64);

impl EdgeWeight {
    /// Zero weight.
    pub const ZERO: Self = Self(0);

    /// Creates a validated weight.
    pub const fn new(
        value: u64,
    ) -> Result<Self, DecodingGraphError> {
        if value > MAX_WEIGHT {
            return Err(
                DecodingGraphError::WeightOutOfRange {
                    value,
                },
            );
        }

        Ok(Self(value))
    }

    /// Returns the numeric weight.
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Creates a bounded weight from a probability.
    ///
    /// The probability is supplied as fixed-point:
    ///
    /// `1_000_000_000_000 == 1.0`
    ///
    /// The conversion uses `-ln(p)` and stores a fixed integer scale.
    /// Decoder decisions themselves remain integer-only.
    pub fn from_probability(
        probability: u64,
    ) -> Result<Self, DecodingGraphError> {
        if probability == 0
            || probability > PROBABILITY_SCALE
        {
            return Err(
                DecodingGraphError::InvalidProbability,
            );
        }

        let p =
            probability as f64
                / PROBABILITY_SCALE as f64;

        let raw = -p.ln() * 1_000_000.0;

        if !raw.is_finite()
            || raw < 0.0
            || raw > MAX_WEIGHT as f64
        {
            return Err(
                DecodingGraphError::WeightOutOfRange {
                    value: MAX_WEIGHT,
                },
            );
        }

        Self::new(raw.round() as u64)
    }
}

// ============================================================================
// Graph edge
// ============================================================================

/// Weighted candidate error path.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct GraphEdge {
    first: GraphEndpoint,
    second: GraphEndpoint,
    weight: EdgeWeight,
    kind: EdgeKind,
}

impl GraphEdge {
    /// Creates an edge.
    ///
    /// Endpoints are canonicalized so deterministic equality and ordering are
    /// preserved regardless of insertion direction.
    pub fn new(
        first: GraphEndpoint,
        second: GraphEndpoint,
        weight: EdgeWeight,
        kind: EdgeKind,
    ) -> Result<Self, DecodingGraphError> {
        if first == second {
            return Err(
                DecodingGraphError::SelfLoop,
            );
        }

        let (first, second) =
            canonical_endpoints(first, second);

        if kind == EdgeKind::Boundary
            && !(
                first.is_boundary()
                    || second.is_boundary()
            )
        {
            return Err(
                DecodingGraphError::BoundaryEdgeRequiresBoundary,
            );
        }

        Ok(Self {
            first,
            second,
            weight,
            kind,
        })
    }

    /// First canonical endpoint.
    pub const fn first(
        &self,
    ) -> GraphEndpoint {
        self.first
    }

    /// Second canonical endpoint.
    pub const fn second(
        &self,
    ) -> GraphEndpoint {
        self.second
    }

    /// Edge weight.
    pub const fn weight(
        &self,
    ) -> EdgeWeight {
        self.weight
    }

    /// Edge semantic kind.
    pub const fn kind(
        &self,
    ) -> EdgeKind {
        self.kind
    }

    /// Returns true if this edge touches a boundary.
    pub const fn touches_boundary(
        &self,
    ) -> bool {
        self.first.is_boundary()
            || self.second.is_boundary()
    }

    /// Returns the opposite endpoint.
    pub fn other(
        &self,
        endpoint: GraphEndpoint,
    ) -> Option<GraphEndpoint> {
        if endpoint == self.first {
            Some(self.second)
        } else if endpoint == self.second {
            Some(self.first)
        } else {
            None
        }
    }
}

// ============================================================================
// Graph configuration
// ============================================================================

/// Graph construction policy.
///
/// `QecLimits` is the authoritative execution/resource policy.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct DecodingGraphConfig {
    limits: QecLimits,
}

impl DecodingGraphConfig {
    /// Creates a graph configuration from central QEC limits.
    pub fn new(
        limits: QecLimits,
    ) -> Result<Self, DecodingGraphError> {
        limits
            .validate()
            .map_err(DecodingGraphError::Limit)?;

        Ok(Self { limits })
    }

    /// Returns the resource policy.
    pub const fn limits(
        &self,
    ) -> QecLimits {
        self.limits
    }
}

impl Default for DecodingGraphConfig {
    fn default() -> Self {
        Self {
            limits: QecLimits::new(),
        }
    }
}

// ============================================================================
// Graph resource estimate
// ============================================================================

/// Resource estimate generated before graph allocation.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct GraphResourceEstimate {
    detection_nodes: usize,
    boundary_nodes: usize,
    edges: usize,
    estimated_memory_bytes: u64,
}

impl GraphResourceEstimate {
    /// Calculates an estimate without allocating graph storage.
    pub fn calculate(
        detection_nodes: usize,
        boundary_nodes: usize,
        edges: usize,
    ) -> Result<Self, DecodingGraphError> {
        let detection_bytes =
            (detection_nodes as u64)
                .checked_mul(
                    ESTIMATED_NODE_BYTES,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let boundary_bytes =
            (boundary_nodes as u64)
                .checked_mul(
                    ESTIMATED_BOUNDARY_BYTES,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let edge_bytes =
            (edges as u64)
                .checked_mul(
                    ESTIMATED_EDGE_BYTES,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let total =
            ESTIMATED_GRAPH_BASE_BYTES
                .checked_add(
                    detection_bytes,
                )
                .and_then(|v| {
                    v.checked_add(
                        boundary_bytes,
                    )
                })
                .and_then(|v| {
                    v.checked_add(edge_bytes)
                })
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        Ok(Self {
            detection_nodes,
            boundary_nodes,
            edges,
            estimated_memory_bytes: total,
        })
    }

    pub const fn detection_nodes(
        self,
    ) -> usize {
        self.detection_nodes
    }

    pub const fn boundary_nodes(
        self,
    ) -> usize {
        self.boundary_nodes
    }

    pub const fn edges(
        self,
    ) -> usize {
        self.edges
    }

    pub const fn estimated_memory_bytes(
        self,
    ) -> u64 {
        self.estimated_memory_bytes
    }

    /// Validates the estimate against central QEC limits.
    pub fn validate_against(
        &self,
        limits: &QecLimits,
    ) -> Result<(), DecodingGraphError> {
        limits
            .validate()
            .map_err(DecodingGraphError::Limit)?;

        let nodes = self
            .detection_nodes
            .checked_add(
                self.boundary_nodes,
            )
            .ok_or(
                DecodingGraphError::ArithmeticOverflow,
            )?;

        if nodes > limits.max_graph_nodes {
            return Err(
                DecodingGraphError::Limit(
                    LimitError::GraphNodes {
                        requested: nodes,
                        maximum:
                            limits.max_graph_nodes,
                    },
                ),
            );
        }

        if self.edges > limits.max_graph_edges {
            return Err(
                DecodingGraphError::Limit(
                    LimitError::GraphEdges {
                        requested: self.edges,
                        maximum:
                            limits.max_graph_edges,
                    },
                ),
            );
        }

        if self.estimated_memory_bytes
            > limits.max_memory_bytes
        {
            return Err(
                DecodingGraphError::Limit(
                    LimitError::MemoryBytes {
                        requested:
                            self.estimated_memory_bytes,
                        maximum:
                            limits.max_memory_bytes,
                    },
                ),
            );
        }

        Ok(())
    }
}

// ============================================================================
// Decoding graph
// ============================================================================

/// Deterministic bounded decoding graph.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct DecodingGraph {
    config: DecodingGraphConfig,

    nodes:
        BTreeMap<
            NodeId,
            DetectionNode,
        >,

    boundaries:
        BTreeMap<
            BoundaryId,
            BoundaryNode,
        >,

    edges:
        BTreeMap<
            (GraphEndpoint, GraphEndpoint),
            GraphEdge,
        >,

    detection_index:
        BTreeMap<
            DetectionKey,
            NodeId,
        >,

    boundary_index:
        BTreeMap<
            SpaceTimeCoordinate,
            BoundaryId,
        >,

    next_node_id: usize,
    next_boundary_id: usize,
}

impl DecodingGraph {
    /// Creates an empty graph using the default QEC resource policy.
    pub fn new() -> Self {
        Self::with_config(
            DecodingGraphConfig::default(),
        )
        .expect(
            "default QEC limits must be valid",
        )
    }

    /// Creates an empty graph using explicit QEC limits.
    pub fn new_with_limits(
        limits: &QecLimits,
    ) -> Result<Self, DecodingGraphError> {
        let config =
            DecodingGraphConfig::new(
                *limits,
            )?;

        Self::with_config(config)
    }

    /// Creates an empty graph from an explicit configuration.
    pub fn with_config(
        config: DecodingGraphConfig,
    ) -> Result<Self, DecodingGraphError> {
        config
            .limits
            .validate()
            .map_err(DecodingGraphError::Limit)?;

        Ok(Self {
            config,

            nodes:
                BTreeMap::new(),

            boundaries:
                BTreeMap::new(),

            edges:
                BTreeMap::new(),

            detection_index:
                BTreeMap::new(),

            boundary_index:
                BTreeMap::new(),

            next_node_id: 0,
            next_boundary_id: 0,
        })
    }

    /// Returns the graph resource policy.
    pub const fn limits(
        &self,
    ) -> QecLimits {
        self.config.limits()
    }

    /// Returns the graph configuration.
    pub const fn config(
        &self,
    ) -> DecodingGraphConfig {
        self.config
    }

    /// Performs allocation-free resource preflight.
    pub fn preflight(
        limits: &QecLimits,
        detection_nodes: usize,
        boundary_nodes: usize,
        edges: usize,
    ) -> Result<
        GraphResourceEstimate,
        DecodingGraphError,
    > {
        let estimate =
            GraphResourceEstimate::calculate(
                detection_nodes,
                boundary_nodes,
                edges,
            )?;

        estimate
            .validate_against(limits)?;

        Ok(estimate)
    }

    /// Returns the number of detection nodes.
    pub fn node_count(
        &self,
    ) -> usize {
        self.nodes.len()
    }

    /// Returns the number of boundary nodes.
    pub fn boundary_count(
        &self,
    ) -> usize {
        self.boundaries.len()
    }

    /// Returns the total graph-node count.
    pub fn total_node_count(
        &self,
    ) -> usize {
        self.node_count()
            .saturating_add(
                self.boundary_count(),
            )
    }

    /// Returns the edge count.
    pub fn edge_count(
        &self,
    ) -> usize {
        self.edges.len()
    }

    /// Returns true when no detection nodes exist.
    pub fn is_empty(
        &self,
    ) -> bool {
        self.nodes.is_empty()
    }

    /// Returns detection nodes in deterministic ID order.
    pub fn nodes(
        &self,
    ) -> impl Iterator<
        Item = &DetectionNode,
    > {
        self.nodes.values()
    }

    /// Returns detection nodes.
    pub fn detection_nodes(
        &self,
    ) -> impl Iterator<
        Item = &DetectionNode,
    > {
        self.nodes.values()
    }

    /// Returns boundaries in deterministic ID order.
    pub fn boundaries(
        &self,
    ) -> impl Iterator<
        Item = &BoundaryNode,
    > {
        self.boundaries.values()
    }

    /// Returns all edges in deterministic endpoint order.
    pub fn edges(
        &self,
    ) -> impl Iterator<
        Item = &GraphEdge,
    > {
        self.edges.values()
    }

    /// Gets a detection node.
    pub fn node(
        &self,
        id: NodeId,
    ) -> Option<&DetectionNode> {
        self.nodes.get(&id)
    }

    /// Gets a boundary node.
    pub fn boundary(
        &self,
        id: BoundaryId,
    ) -> Option<&BoundaryNode> {
        self.boundaries.get(&id)
    }

    /// Adds a detection event using an explicit spatial coordinate.
    ///
    /// The graph owns the space-time coordinate; the syndrome layer owns the
    /// event semantics.
    pub fn add_detection_event(
        &mut self,
        event: DetectionEvent,
        spatial: SpatialCoordinate,
    ) -> Result<NodeId, DecodingGraphError> {
        if !event.value() {
            return Err(
                DecodingGraphError::InactiveDetectionEvent,
            );
        }

        let coordinate =
            SpaceTimeCoordinate::new(
                spatial,
                event.round(),
            )?;

        self.add_detection_node(
            coordinate,
            event.stabilizer(),
            event.confidence(),
        )
    }

    /// Adds a detection node.
    pub fn add_detection_node(
        &mut self,
        coordinate: SpaceTimeCoordinate,
        stabilizer: StabilizerId,
        confidence: MeasurementConfidence,
    ) -> Result<NodeId, DecodingGraphError> {
        self.check_node_capacity(1)?;

        let key =
            DetectionKey {
                coordinate:
                    coordinate.clone(),
                stabilizer,
            };

        if self
            .detection_index
            .contains_key(&key)
        {
            return Err(
                DecodingGraphError::DuplicateDetectionNode,
            );
        }

        let id =
            NodeId::new(
                self.next_node_id,
            );

        self.next_node_id =
            self.next_node_id
                .checked_add(1)
                .ok_or(
                    DecodingGraphError::IdentifierOverflow,
                )?;

        let node =
            DetectionNode::new(
                id,
                coordinate,
                stabilizer,
                confidence,
            );

        self.nodes.insert(
            id,
            node,
        );

        self.detection_index
            .insert(key, id);

        Ok(id)
    }

    /// Adds a physical/logical boundary.
    pub fn add_boundary(
        &mut self,
        coordinate: SpaceTimeCoordinate,
    ) -> Result<
        BoundaryId,
        DecodingGraphError,
    > {
        self.check_node_capacity(1)?;

        if self
            .boundary_index
            .contains_key(&coordinate)
        {
            return Err(
                DecodingGraphError::DuplicateBoundary,
            );
        }

        let id =
            BoundaryId::new(
                self.next_boundary_id,
            );

        self.next_boundary_id =
            self.next_boundary_id
                .checked_add(1)
                .ok_or(
                    DecodingGraphError::IdentifierOverflow,
                )?;

        let boundary =
            BoundaryNode::new(
                id,
                coordinate.clone(),
            );

        self.boundaries.insert(
            id,
            boundary,
        );

        self.boundary_index
            .insert(coordinate, id);

        Ok(id)
    }

    /// Adds a weighted graph edge.
    pub fn add_edge(
        &mut self,
        first: GraphEndpoint,
        second: GraphEndpoint,
        weight: EdgeWeight,
        kind: EdgeKind,
    ) -> Result<(), DecodingGraphError> {
        self.ensure_endpoint_exists(first)?;
        self.ensure_endpoint_exists(second)?;

        let edge =
            GraphEdge::new(
                first,
                second,
                weight,
                kind,
            )?;

        let key =
            canonical_endpoints(
                edge.first(),
                edge.second(),
            );

        if self.edges.contains_key(&key) {
            return Err(
                DecodingGraphError::DuplicateEdge,
            );
        }

        self.check_edge_capacity(1)?;

        self.edges.insert(
            key,
            edge,
        );

        Ok(())
    }

    /// Returns an edge between two endpoints.
    pub fn edge(
        &self,
        first: GraphEndpoint,
        second: GraphEndpoint,
    ) -> Option<&GraphEdge> {
        let key =
            canonical_endpoints(
                first,
                second,
            );

        self.edges.get(&key)
    }

    /// Returns edges incident to an endpoint.
    ///
    /// Results are deterministic.
    pub fn incident_edges(
        &self,
        endpoint: GraphEndpoint,
    ) -> Vec<&GraphEdge> {
        self.edges
            .values()
            .filter(
                |edge| {
                    edge.first() == endpoint
                        || edge.second()
                            == endpoint
                },
            )
            .collect()
    }

    /// Returns graph neighbours for an endpoint.
    ///
    /// The result is deterministic and contains no duplicates.
    pub fn neighbours(
        &self,
        endpoint: GraphEndpoint,
    ) -> Vec<GraphEndpoint> {
        let mut result =
            BTreeSet::new();

        for edge in self.edges.values() {
            if edge.first() == endpoint {
                result.insert(
                    edge.second(),
                );
            } else if edge.second()
                == endpoint
            {
                result.insert(
                    edge.first(),
                );
            }
        }

        result.into_iter().collect()
    }

    /// US spelling compatibility alias.
    pub fn neighbors(
        &self,
        endpoint: GraphEndpoint,
    ) -> Vec<GraphEndpoint> {
        self.neighbours(endpoint)
    }

    /// Validates the complete graph.
    pub fn validate(
        &self,
    ) -> Result<(), DecodingGraphError> {
        self.limits()
            .validate()
            .map_err(
                DecodingGraphError::Limit,
            )?;

        Self::preflight(
            &self.limits(),
            self.node_count(),
            self.boundary_count(),
            self.edge_count(),
        )?;

        if self.nodes.len()
            != self.detection_index.len()
        {
            return Err(
                DecodingGraphError::IndexMismatch,
            );
        }

        if self.boundaries.len()
            != self.boundary_index.len()
        {
            return Err(
                DecodingGraphError::IndexMismatch,
            );
        }

        for (id, node) in &self.nodes {
            if *id != node.id() {
                return Err(
                    DecodingGraphError::NodeIdMismatch,
                );
            }

            if node.coordinate()
                .round()
                .value()
                > MAX_GRAPH_ROUND
            {
                return Err(
                    DecodingGraphError::InvalidRound {
                        round: node
                            .coordinate()
                            .round()
                            .value(),
                    },
                );
            }
        }

        for (id, boundary)
            in &self.boundaries
        {
            if *id != boundary.id() {
                return Err(
                    DecodingGraphError::BoundaryIdMismatch,
                );
            }
        }

        let mut seen_edges =
            BTreeSet::new();

        for edge in self.edges.values() {
            let first =
                edge.first();
            let second =
                edge.second();

            self.ensure_endpoint_exists(
                first,
            )?;

            self.ensure_endpoint_exists(
                second,
            )?;

            if first == second {
                return Err(
                    DecodingGraphError::SelfLoop,
                );
            }

            let key =
                canonical_endpoints(
                    first,
                    second,
                );

            if !seen_edges.insert(key) {
                return Err(
                    DecodingGraphError::DuplicateEdge,
                );
            }

            if edge.weight().value()
                > MAX_WEIGHT
            {
                return Err(
                    DecodingGraphError::WeightOutOfRange {
                        value: edge
                            .weight()
                            .value(),
                    },
                );
            }
        }

        Ok(())
    }

    /// Returns a deterministic graph resource estimate.
    pub fn resource_estimate(
        &self,
    ) -> Result<
        GraphResourceEstimate,
        DecodingGraphError,
    > {
        GraphResourceEstimate::calculate(
            self.node_count(),
            self.boundary_count(),
            self.edge_count(),
        )
    }

    // ------------------------------------------------------------------------
    // Internal capacity checks
    // ------------------------------------------------------------------------

    fn check_node_capacity(
        &self,
        additional: usize,
    ) -> Result<(), DecodingGraphError> {
        let requested_detection =
            self.node_count()
                .checked_add(
                    additional,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let total =
            requested_detection
                .checked_add(
                    self.boundary_count(),
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        if total
            > self
                .limits()
                .max_graph_nodes
        {
            return Err(
                DecodingGraphError::Limit(
                    LimitError::GraphNodes {
                        requested: total,
                        maximum:
                            self.limits()
                                .max_graph_nodes,
                    },
                ),
            );
        }

        let estimate =
            GraphResourceEstimate::calculate(
                requested_detection,
                self.boundary_count(),
                self.edge_count(),
            )?;

        if estimate
            .estimated_memory_bytes()
            > self
                .limits()
                .max_memory_bytes
        {
            return Err(
                DecodingGraphError::Limit(
                    LimitError::MemoryBytes {
                        requested:
                            estimate
                                .estimated_memory_bytes(),
                        maximum:
                            self.limits()
                                .max_memory_bytes,
                    },
                ),
            );
        }

        Ok(())
    }

    fn check_edge_capacity(
        &self,
        additional: usize,
    ) -> Result<(), DecodingGraphError> {
        let requested =
            self.edge_count()
                .checked_add(
                    additional,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        if requested
            > self
                .limits()
                .max_graph_edges
        {
            return Err(
                DecodingGraphError::Limit(
                    LimitError::GraphEdges {
                        requested,
                        maximum:
                            self.limits()
                                .max_graph_edges,
                    },
                ),
            );
        }

        let estimate =
            GraphResourceEstimate::calculate(
                self.node_count(),
                self.boundary_count(),
                requested,
            )?;

        if estimate
            .estimated_memory_bytes()
            > self
                .limits()
                .max_memory_bytes
        {
            return Err(
                DecodingGraphError::Limit(
                    LimitError::MemoryBytes {
                        requested:
                            estimate
                                .estimated_memory_bytes(),
                        maximum:
                            self.limits()
                                .max_memory_bytes,
                    },
                ),
            );
        }

        Ok(())
    }

    fn ensure_endpoint_exists(
        &self,
        endpoint: GraphEndpoint,
    ) -> Result<(), DecodingGraphError> {
        match endpoint {
            GraphEndpoint::Detection(id) => {
                if !self.nodes.contains_key(&id) {
                    return Err(
                        DecodingGraphError::UnknownNode {
                            id,
                        },
                    );
                }
            }

            GraphEndpoint::Boundary(id) => {
                if !self
                    .boundaries
                    .contains_key(&id)
                {
                    return Err(
                        DecodingGraphError::UnknownBoundary {
                            id,
                        },
                    );
                }
            }
        }

        Ok(())
    }
}

impl Default for DecodingGraph {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Internal indexing
// ============================================================================

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
struct DetectionKey {
    coordinate: SpaceTimeCoordinate,
    stabilizer: StabilizerId,
}

// ============================================================================
// Canonical ordering
// ============================================================================

fn canonical_endpoints(
    first: GraphEndpoint,
    second: GraphEndpoint,
) -> (
    GraphEndpoint,
    GraphEndpoint,
) {
    if first <= second {
        (first, second)
    } else {
        (second, first)
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by decoding graph construction and validation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum DecodingGraphError {
    EmptyCoordinate,

    TooManyDimensions {
        dimensions: usize,
        limit: usize,
    },

    CoordinateOutOfRange {
        value: i64,
    },

    DimensionMismatch,

    ArithmeticOverflow,

    InvalidRound {
        round: u64,
    },

    InvalidTimestamp {
        timestamp: u64,
    },

    InvalidProbability,

    WeightOutOfRange {
        value: u64,
    },

    SelfLoop,

    BoundaryEdgeRequiresBoundary,

    DuplicateDetectionNode,

    DuplicateBoundary,

    DuplicateEdge,

    InactiveDetectionEvent,

    IdentifierOverflow,

    UnknownNode {
        id: NodeId,
    },

    UnknownBoundary {
        id: BoundaryId,
    },

    IndexMismatch,

    NodeIdMismatch,

    BoundaryIdMismatch,

    Limit(LimitError),
}

impl fmt::Display for DecodingGraphError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyCoordinate => {
                write!(
                    f,
                    "spatial coordinate cannot be empty"
                )
            }

            Self::TooManyDimensions {
                dimensions,
                limit,
            } => {
                write!(
                    f,
                    "coordinate has {dimensions} dimensions; maximum is {limit}"
                )
            }

            Self::CoordinateOutOfRange {
                value,
            } => {
                write!(
                    f,
                    "coordinate component {value} is outside the supported range"
                )
            }

            Self::DimensionMismatch => {
                write!(
                    f,
                    "coordinate dimensions do not match"
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    f,
                    "arithmetic overflow in decoding graph"
                )
            }

            Self::InvalidRound {
                round,
            } => {
                write!(
                    f,
                    "invalid graph measurement round: {round}"
                )
            }

            Self::InvalidTimestamp {
                timestamp,
            } => {
                write!(
                    f,
                    "invalid graph timestamp: {timestamp}"
                )
            }

            Self::InvalidProbability => {
                write!(
                    f,
                    "probability must be in the range (0, 1]"
                )
            }

            Self::WeightOutOfRange {
                value,
            } => {
                write!(
                    f,
                    "edge weight {value} exceeds the supported maximum"
                )
            }

            Self::SelfLoop => {
                write!(
                    f,
                    "self-loop edges are not permitted"
                )
            }

            Self::BoundaryEdgeRequiresBoundary => {
                write!(
                    f,
                    "a Boundary edge must touch a boundary endpoint"
                )
            }

            Self::DuplicateDetectionNode => {
                write!(
                    f,
                    "duplicate detection node"
                )
            }

            Self::DuplicateBoundary => {
                write!(
                    f,
                    "duplicate boundary"
                )
            }

            Self::DuplicateEdge => {
                write!(
                    f,
                    "duplicate graph edge"
                )
            }

            Self::InactiveDetectionEvent => {
                write!(
                    f,
                    "inactive detection event cannot become a graph node"
                )
            }

            Self::IdentifierOverflow => {
                write!(
                    f,
                    "graph identifier overflow"
                )
            }

            Self::UnknownNode {
                id,
            } => {
                write!(
                    f,
                    "unknown graph node {id}"
                )
            }

            Self::UnknownBoundary {
                id,
            } => {
                write!(
                    f,
                    "unknown graph boundary {id}"
                )
            }

            Self::IndexMismatch => {
                write!(
                    f,
                    "graph index does not match graph storage"
                )
            }

            Self::NodeIdMismatch => {
                write!(
                    f,
                    "graph node identifier mismatch"
                )
            }

            Self::BoundaryIdMismatch => {
                write!(
                    f,
                    "graph boundary identifier mismatch"
                )
            }

            Self::Limit(error) => {
                write!(
                    f,
                    "{error}"
                )
            }
        }
    }
}

impl std::error::Error
    for DecodingGraphError
{}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> QecLimits {
        let mut limits =
            QecLimits::new();

        limits.max_graph_nodes = 32;
        limits.max_graph_edges = 64;
        limits.max_memory_bytes = 64 * 1024;

        limits
    }

    fn round(
        value: u64,
    ) -> MeasurementRound {
        MeasurementRound::new(value)
            .expect(
                "test round must be valid",
            )
    }

    fn coordinate(
        x: i64,
        y: i64,
    ) -> SpatialCoordinate {
        SpatialCoordinate::xy(x, y)
            .expect(
                "test coordinate must be valid",
            )
    }

    #[test]
    fn graph_starts_empty() {
        let graph =
            DecodingGraph::new_with_limits(
                &limits(),
            )
            .expect(
                "graph creation should succeed",
            );

        assert_eq!(
            graph.node_count(),
            0
        );

        assert_eq!(
            graph.boundary_count(),
            0
        );

        assert_eq!(
            graph.edge_count(),
            0
        );

        assert!(
            graph.is_empty()
        );
    }

    #[test]
    fn coordinate_distance_is_checked() {
        let a =
            coordinate(0, 0);

        let b =
            coordinate(3, 4);

        assert_eq!(
            a.manhattan_distance(&b)
                .expect(
                    "distance should succeed",
                ),
            7
        );
    }

    #[test]
    fn duplicate_detection_nodes_are_rejected() {
        let mut graph =
            DecodingGraph::new_with_limits(
                &limits(),
            )
            .unwrap();

        let coordinate =
            SpaceTimeCoordinate::new(
                coordinate(1, 2),
                round(0),
            )
            .unwrap();

        graph
            .add_detection_node(
                coordinate.clone(),
                StabilizerId::new(0),
                MeasurementConfidence::FULL,
            )
            .unwrap();

        let result =
            graph.add_detection_node(
                coordinate,
                StabilizerId::new(0),
                MeasurementConfidence::FULL,
            );

        assert_eq!(
            result,
            Err(
                DecodingGraphError::
                    DuplicateDetectionNode
            )
        );
    }

    #[test]
    fn duplicate_edges_are_rejected() {
        let mut graph =
            DecodingGraph::new_with_limits(
                &limits(),
            )
            .unwrap();

        let a =
            graph
                .add_detection_node(
                    SpaceTimeCoordinate::new(
                        coordinate(0, 0),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(0),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let b =
            graph
                .add_detection_node(
                    SpaceTimeCoordinate::new(
                        coordinate(1, 0),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(1),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let first =
            GraphEndpoint::Detection(a);

        let second =
            GraphEndpoint::Detection(b);

        graph
            .add_edge(
                first,
                second,
                EdgeWeight::new(1).unwrap(),
                EdgeKind::Spatial,
            )
            .unwrap();

        let result =
            graph.add_edge(
                second,
                first,
                EdgeWeight::new(1).unwrap(),
                EdgeKind::Spatial,
            );

        assert_eq!(
            result,
            Err(
                DecodingGraphError::
                    DuplicateEdge
            )
        );
    }

    #[test]
    fn boundaries_are_real_graph_endpoints() {
        let mut graph =
            DecodingGraph::new_with_limits(
                &limits(),
            )
            .unwrap();

        let node =
            graph
                .add_detection_node(
                    SpaceTimeCoordinate::new(
                        coordinate(0, 0),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(0),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let boundary =
            graph
                .add_boundary(
                    SpaceTimeCoordinate::new(
                        coordinate(0, -1),
                        round(0),
                    )
                    .unwrap(),
                )
                .unwrap();

        graph
            .add_edge(
                GraphEndpoint::Detection(node),
                GraphEndpoint::Boundary(
                    boundary,
                ),
                EdgeWeight::new(5).unwrap(),
                EdgeKind::Boundary,
            )
            .unwrap();

        assert_eq!(
            graph.boundary_count(),
            1
        );

        assert_eq!(
            graph.edge_count(),
            1
        );

        assert!(
            graph.validate().is_ok()
        );
    }

    #[test]
    fn resource_preflight_happens_before_graph_allocation() {
        let mut limits =
            limits();

        limits.max_graph_nodes = 2;
        limits.max_graph_edges = 1;

        let result =
            DecodingGraph::preflight(
                &limits,
                3,
                0,
                0,
            );

        assert!(matches!(
            result,
            Err(
                DecodingGraphError::Limit(
                    LimitError::GraphNodes {
                        ..
                    }
                )
            )
        ));
    }

    #[test]
    fn graph_iteration_is_deterministic() {
        let mut graph =
            DecodingGraph::new_with_limits(
                &limits(),
            )
            .unwrap();

        let first =
            graph
                .add_detection_node(
                    SpaceTimeCoordinate::new(
                        coordinate(5, 5),
                        round(1),
                    )
                    .unwrap(),
                    StabilizerId::new(5),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let second =
            graph
                .add_detection_node(
                    SpaceTimeCoordinate::new(
                        coordinate(1, 1),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(1),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let ids: Vec<NodeId> =
            graph
                .nodes()
                .map(DetectionNode::id)
                .collect();

        assert_eq!(
            ids,
            vec![first, second]
        );
    }
}