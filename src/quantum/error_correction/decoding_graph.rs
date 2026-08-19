//! Zamani Quantum Error Correction — Space-Time Decoding Graph.
//!
//! This module converts detection events into a bounded, deterministic
//! space-time graph suitable for fault-tolerant decoding.
//!
//! Architecture:
//!
//! ```text
//! Syndrome rounds
//!       │
//!       ▼
//! Detection events
//!       │
//!       ▼
//! DecodingGraph
//!       │
//!       ├── Detection nodes
//!       │
//!       ├── Boundary nodes
//!       │
//!       └── Weighted edges
//!              │
//!              ▼
//!          MWPM / Union-Find
//! ```
//!
//! This module deliberately does NOT implement a decoder.
//!
//! Responsibilities:
//! - represent space-time detection nodes;
//! - represent boundary nodes;
//! - represent weighted candidate error paths;
//! - validate graph invariants;
//! - prevent duplicate nodes and edges;
//! - enforce resource limits;
//! - provide deterministic ordering;
//! - provide probability-to-weight conversion;
//! - support future MWPM and Union-Find implementations;
//! - reject malformed external input without panicking.
//!
//! Non-responsibilities:
//! - syndrome extraction;
//! - stabilizer algebra;
//! - physical noise generation;
//! - correction application;
//! - matching;
//! - logical-error classification.
//!
//! The graph is therefore an intermediate representation between syndrome
//! processing and decoding.
//
// ============================================================================
// Imports
// ============================================================================

use std::collections::{
    BTreeMap,
    BTreeSet,
};
use std::fmt;

use super::syndrome::{
    DetectionEvent,
    MeasurementConfidence,
    MeasurementRound,
    MeasurementTimestamp,
    StabilizerId,
};

// ============================================================================
// Resource limits
// ============================================================================

/// Maximum number of detection nodes in one graph.
pub const MAX_DETECTION_NODES: usize = 1_000_000;

/// Maximum number of boundary nodes in one graph.
pub const MAX_BOUNDARY_NODES: usize = 100_000;

/// Maximum number of graph edges.
pub const MAX_GRAPH_EDGES: usize = 4_000_000;

/// Maximum number of spatial coordinates supported by one node.
pub const MAX_SPATIAL_DIMENSIONS: usize = 8;

/// Maximum absolute spatial coordinate.
///
/// Coordinates are signed because some lattice representations naturally use
/// offsets around an origin.
pub const MAX_COORDINATE_ABS: i64 = 1_000_000_000;

/// Maximum measurement round accepted by the graph.
pub const MAX_GRAPH_ROUND: u64 = u64::MAX - 1;

/// Maximum timestamp accepted by the graph.
pub const MAX_GRAPH_TIMESTAMP: u64 = u64::MAX - 1;

/// Probability fixed-point scale.
///
/// ```text
/// 0                  = 0
/// 1_000_000_000_000  = 1
/// ```
pub const PROBABILITY_SCALE: u64 = 1_000_000_000_000;

/// Maximum finite decoding weight.
///
/// This prevents pathological probabilities from creating infinities or
/// overflowing downstream matching algorithms.
pub const MAX_WEIGHT: u64 = 1_000_000_000_000_000;

// ============================================================================
// Graph coordinate
// ============================================================================

/// A spatial coordinate in the decoding lattice.
///
/// The decoding graph treats spatial coordinates as opaque integer
/// coordinates. The topology owner decides what the coordinates mean.
///
/// For example, a surface-code implementation may use:
///
/// ```text
/// (x, y)
/// ```
///
/// while a more complex architecture may use:
///
/// ```text
/// (x, y, layer)
/// ```
///
/// No floating-point coordinates are permitted.
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
    /// Creates a spatial coordinate.
    pub fn new(
        values: Vec<i64>,
    ) -> Result<Self, DecodingGraphError> {
        if values.is_empty() {
            return Err(
                DecodingGraphError::EmptyCoordinate,
            );
        }

        if values.len()
            > MAX_SPATIAL_DIMENSIONS
        {
            return Err(
                DecodingGraphError::TooManyDimensions {
                    dimensions: values.len(),
                    limit:
                        MAX_SPATIAL_DIMENSIONS,
                },
            );
        }

        for &value in &values {
            if value
                .checked_abs()
                .is_none_or(
                    |absolute| {
                        absolute
                            > MAX_COORDINATE_ABS
                    },
                )
            {
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

    /// Returns the coordinate dimension.
    pub fn dimensions(
        &self,
    ) -> usize {
        self.values.len()
    }

    /// Returns a coordinate component.
    pub fn get(
        &self,
        dimension: usize,
    ) -> Option<i64> {
        self.values.get(dimension).copied()
    }

    /// Returns all coordinate components.
    pub fn values(
        &self,
    ) -> &[i64] {
        &self.values
    }

    /// Calculates Manhattan distance between coordinates.
    pub fn manhattan_distance(
        &self,
        other: &Self,
    ) -> Result<u64, DecodingGraphError> {
        if self.dimensions()
            != other.dimensions()
        {
            return Err(
                DecodingGraphError::DimensionMismatch,
            );
        }

        let mut distance = 0u64;

        for (&left, &right) in self
            .values
            .iter()
            .zip(other.values.iter())
        {
            let delta =
                left
                    .checked_sub(right)
                    .ok_or(
                        DecodingGraphError::ArithmeticOverflow,
                    )?;

            let absolute =
                delta
                    .checked_abs()
                    .ok_or(
                        DecodingGraphError::ArithmeticOverflow,
                    )?;

            distance =
                distance
                    .checked_add(
                        absolute as u64,
                    )
                    .ok_or(
                        DecodingGraphError::ArithmeticOverflow,
                    )?;
        }

        Ok(distance)
    }
}

impl fmt::Display
    for SpatialCoordinate
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "(")?;

        for (
            index,
            value,
        ) in self.values.iter().enumerate()
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

/// A node position in space-time.
///
/// ```text
/// (spatial coordinate, measurement round)
/// ```
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
    /// Creates a space-time coordinate.
    pub fn new(
        spatial: SpatialCoordinate,
        round: MeasurementRound,
    ) -> Result<Self, DecodingGraphError> {
        if round.value()
            > MAX_GRAPH_ROUND
        {
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
// Node identifier
// ============================================================================

/// Stable graph-node identifier.
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
pub struct NodeId(
    usize,
);

impl NodeId {
    /// Creates a node identifier.
    pub const fn new(
        id: usize,
    ) -> Self {
        Self(id)
    }

    /// Returns the underlying identifier.
    pub const fn index(
        self,
    ) -> usize {
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

// ============================================================================
// Boundary identifier
// ============================================================================

/// Stable identifier for a graph boundary.
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
pub struct BoundaryId(
    usize,
);

impl BoundaryId {
    /// Creates a boundary identifier.
    pub const fn new(
        id: usize,
    ) -> Self {
        Self(id)
    }

    /// Returns the underlying identifier.
    pub const fn index(
        self,
    ) -> usize {
        self.0
    }
}

impl fmt::Display
    for BoundaryId
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "b{}", self.0)
    }
}

// ============================================================================
// Graph node
// ============================================================================

/// A detection node in the decoding graph.
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

    /// Returns node identifier.
    pub const fn id(
        &self,
    ) -> NodeId {
        self.id
    }

    /// Returns node coordinate.
    pub fn coordinate(
        &self,
    ) -> &SpaceTimeCoordinate {
        &self.coordinate
    }

    /// Returns stabilizer identifier.
    pub const fn stabilizer(
        &self,
    ) -> StabilizerId {
        self.stabilizer
    }

    /// Returns measurement confidence.
    pub const fn confidence(
        &self,
    ) -> MeasurementConfidence {
        self.confidence
    }
}

// ============================================================================
// Boundary node
// ============================================================================

/// A graph boundary.
///
/// Boundary nodes represent error chains terminating at a physical or
/// logical boundary instead of another detection event.
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
    /// Creates a boundary node.
    pub fn new(
        id: BoundaryId,
        coordinate: SpaceTimeCoordinate,
    ) -> Self {
        Self {
            id,
            coordinate,
        }
    }

    /// Returns the boundary identifier.
    pub const fn id(
        &self,
    ) -> BoundaryId {
        self.id
    }

    /// Returns the boundary coordinate.
    pub fn coordinate(
        &self,
    ) -> &SpaceTimeCoordinate {
        &self.coordinate
    }
}

// ============================================================================
// Graph endpoint
// ============================================================================

/// Endpoint of a decoding-graph edge.
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
    /// Detection-event node.
    Detection(NodeId),

    /// Boundary node.
    Boundary(BoundaryId),
}

impl GraphEndpoint {
    /// Returns true if the endpoint is a detection node.
    pub const fn is_detection(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Detection(_)
        )
    }

    /// Returns true if the endpoint is a boundary.
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

/// Physical interpretation of a decoding-graph edge.
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
    /// Spatial propagation of an error.
    Spatial,

    /// Temporal propagation caused by measurement/storage faults.
    Temporal,

    /// Combined spatial and temporal propagation.
    SpaceTime,

    /// Connection to a physical/logical boundary.
    Boundary,

    /// Explicitly supplied backend-specific edge.
    Custom,
}

// ============================================================================
// Edge weight
// ============================================================================

/// Exact bounded non-negative decoding weight.
///
/// MWPM traditionally uses:
///
/// ```text
/// w = -log(p / (1-p))
/// ```
///
/// or another decoder-specific log-likelihood convention.
///
/// This module does not prescribe the matching convention. It stores a
/// validated non-negative integer weight so that the graph remains
/// deterministic and free from NaN/infinity.
///
/// A backend can construct weights using [`EdgeWeight::from_probability`].
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
pub struct EdgeWeight(
    u64,
);

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

    /// Returns the underlying weight.
    pub const fn value(
        self,
    ) -> u64 {
        self.0
    }

    /// Creates a weight from a fixed-point probability.
    ///
    /// This uses a deterministic fixed-point approximation of:
    ///
    /// ```text
    /// -ln(p)
    /// ```
    ///
    /// It is intentionally conservative and finite.
    ///
    /// For a production decoder, a calibrated backend may supply its own
    /// weights using [`EdgeWeight::new`].
    pub fn from_probability(
        probability: u64,
    ) -> Result<Self, DecodingGraphError> {
        if probability == 0
            || probability
                > PROBABILITY_SCALE
        {
            return Err(
                DecodingGraphError::InvalidProbability,
            );
        }

        if probability
            == PROBABILITY_SCALE
        {
            return Ok(Self::ZERO);
        }

        let ratio =
            PROBABILITY_SCALE
                .checked_div(
                    probability,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let ratio =
            ratio.max(1);

        let mut value =
            0u64;

        let mut current =
            ratio;

        while current > 1 {
            current /= 2;

            value =
                value
                    .checked_add(
                        693_147_180_559,
                    )
                    .ok_or(
                        DecodingGraphError::ArithmeticOverflow,
                    )?;

            if value
                >= MAX_WEIGHT
            {
                return Ok(
                    Self(MAX_WEIGHT),
                );
            }
        }

        Self::new(value)
    }

    /// Adds two weights with overflow and maximum checks.
    pub fn checked_add(
        self,
        other: Self,
    ) -> Result<Self, DecodingGraphError> {
        let value =
            self.0
                .checked_add(other.0)
                .ok_or(
                    DecodingGraphError::WeightOverflow,
                )?;

        Self::new(value)
    }
}

// ============================================================================
// Graph edge
// ============================================================================

/// Weighted candidate physical-error path.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct GraphEdge {
    from: GraphEndpoint,
    to: GraphEndpoint,
    kind: EdgeKind,
    weight: EdgeWeight,
}

impl GraphEdge {
    /// Creates a graph edge.
    pub fn new(
        from: GraphEndpoint,
        to: GraphEndpoint,
        kind: EdgeKind,
        weight: EdgeWeight,
    ) -> Result<Self, DecodingGraphError> {
        if from == to {
            return Err(
                DecodingGraphError::SelfLoop,
            );
        }

        Ok(Self {
            from,
            to,
            kind,
            weight,
        })
    }

    /// Returns the source endpoint.
    pub const fn from(
        &self,
    ) -> GraphEndpoint {
        self.from
    }

    /// Returns the destination endpoint.
    pub const fn to(
        &self,
    ) -> GraphEndpoint {
        self.to
    }

    /// Returns the edge kind.
    pub const fn kind(
        &self,
    ) -> EdgeKind {
        self.kind
    }

    /// Returns the decoding weight.
    pub const fn weight(
        &self,
    ) -> EdgeWeight {
        self.weight
    }

    /// Returns true when this edge touches a boundary.
    pub const fn touches_boundary(
        &self,
    ) -> bool {
        self.from.is_boundary()
            || self.to.is_boundary()
    }

    /// Returns true when this is an ordinary detection-to-detection edge.
    pub const fn connects_detections(
        &self,
    ) -> bool {
        self.from.is_detection()
            && self.to.is_detection()
    }
}

// ============================================================================
// Graph
// ============================================================================

/// Deterministic bounded space-time decoding graph.
///
/// The graph uses ordered maps/sets so that:
///
/// - insertion order does not affect iteration;
/// - serialization can be deterministic;
/// - decoder behavior can be reproducible;
/// - malformed duplicate topology is rejected.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct DecodingGraph {
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
            GraphEdgeKey,
            GraphEdge,
        >,

    coordinate_index:
        BTreeMap<
            SpaceTimeCoordinate,
            NodeId,
        >,

    next_node_id: usize,
    next_boundary_id: usize,
}

impl DecodingGraph {
    /// Creates an empty decoding graph.
    pub fn new() -> Self {
        Self {
            nodes:
                BTreeMap::new(),

            boundaries:
                BTreeMap::new(),

            edges:
                BTreeMap::new(),

            coordinate_index:
                BTreeMap::new(),

            next_node_id: 0,
            next_boundary_id: 0,
        }
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

    /// Returns the number of edges.
    pub fn edge_count(
        &self,
    ) -> usize {
        self.edges.len()
    }

    /// Returns true when the graph contains no detection nodes.
    pub fn is_empty(
        &self,
    ) -> bool {
        self.nodes.is_empty()
    }

    /// Returns an iterator over detection nodes in deterministic order.
    pub fn nodes(
        &self,
    ) -> impl Iterator<
        Item = &DetectionNode,
    > {
        self.nodes.values()
    }

    /// Returns an iterator over boundary nodes.
    pub fn boundaries(
        &self,
    ) -> impl Iterator<
        Item = &BoundaryNode,
    > {
        self.boundaries.values()
    }

    /// Returns an iterator over edges in deterministic order.
    pub fn edges(
        &self,
    ) -> impl Iterator<
        Item = &GraphEdge,
    > {
        self.edges.values()
    }

    /// Looks up a detection node.
    pub fn node(
        &self,
        id: NodeId,
    ) -> Option<&DetectionNode> {
        self.nodes.get(&id)
    }

    /// Looks up a boundary node.
    pub fn boundary(
        &self,
        id: BoundaryId,
    ) -> Option<&BoundaryNode> {
        self.boundaries.get(&id)
    }

    /// Finds a detection node by space-time coordinate.
    pub fn node_at(
        &self,
        coordinate: &SpaceTimeCoordinate,
    ) -> Option<NodeId> {
        self.coordinate_index
            .get(coordinate)
            .copied()
    }

    /// Adds a detection node.
    ///
    /// The same space-time coordinate may not identify two different
    /// detection nodes.
    pub fn add_detection(
        &mut self,
        coordinate: SpaceTimeCoordinate,
        stabilizer: StabilizerId,
        confidence: MeasurementConfidence,
    ) -> Result<NodeId, DecodingGraphError> {
        if self.nodes.len()
            >= MAX_DETECTION_NODES
        {
            return Err(
                DecodingGraphError::TooManyNodes {
                    limit:
                        MAX_DETECTION_NODES,
                },
            );
        }

        if self
            .coordinate_index
            .contains_key(&coordinate)
        {
            return Err(
                DecodingGraphError::DuplicateNodeCoordinate,
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
                    DecodingGraphError::NodeIdOverflow,
                )?;

        let node =
            DetectionNode::new(
                id,
                coordinate.clone(),
                stabilizer,
                confidence,
            );

        self.nodes
            .insert(id, node);

        self.coordinate_index
            .insert(
                coordinate,
                id,
            );

        Ok(id)
    }

    /// Adds a boundary node.
    pub fn add_boundary(
        &mut self,
        coordinate: SpaceTimeCoordinate,
    ) -> Result<
        BoundaryId,
        DecodingGraphError,
    > {
        if self.boundaries.len()
            >= MAX_BOUNDARY_NODES
        {
            return Err(
                DecodingGraphError::TooManyBoundaries {
                    limit:
                        MAX_BOUNDARY_NODES,
                },
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
                    DecodingGraphError::BoundaryIdOverflow,
                )?;

        self.boundaries.insert(
            id,
            BoundaryNode::new(
                id,
                coordinate,
            ),
        );

        Ok(id)
    }

    /// Adds an edge.
    ///
    /// The graph is undirected. Therefore `(A,B)` and `(B,A)` are considered
    /// the same edge.
    pub fn add_edge(
        &mut self,
        edge: GraphEdge,
    ) -> Result<(), DecodingGraphError> {
        self.validate_endpoint(
            edge.from(),
        )?;

        self.validate_endpoint(
            edge.to(),
        )?;

        if self.edges.len()
            >= MAX_GRAPH_EDGES
        {
            return Err(
                DecodingGraphError::TooManyEdges {
                    limit:
                        MAX_GRAPH_EDGES,
                },
            );
        }

        let key =
            GraphEdgeKey::new(
                edge.from(),
                edge.to(),
            );

        if self.edges.contains_key(&key) {
            return Err(
                DecodingGraphError::DuplicateEdge,
            );
        }

        self.edges.insert(
            key,
            edge,
        );

        Ok(())
    }

    /// Adds an edge from two endpoints.
    pub fn connect(
        &mut self,
        from: GraphEndpoint,
        to: GraphEndpoint,
        kind: EdgeKind,
        weight: EdgeWeight,
    ) -> Result<(), DecodingGraphError> {
        self.add_edge(
            GraphEdge::new(
                from,
                to,
                kind,
                weight,
            )?,
        )
    }

    /// Adds an edge using a physical probability.
    pub fn connect_with_probability(
        &mut self,
        from: GraphEndpoint,
        to: GraphEndpoint,
        kind: EdgeKind,
        probability: u64,
    ) -> Result<(), DecodingGraphError> {
        let weight =
            EdgeWeight::from_probability(
                probability,
            )?;

        self.connect(
            from,
            to,
            kind,
            weight,
        )
    }

    /// Returns edges incident to an endpoint.
    pub fn incident_edges(
        &self,
        endpoint: GraphEndpoint,
    ) -> Vec<&GraphEdge> {
        self.edges
            .values()
            .filter(
                |edge| {
                    edge.from() == endpoint
                        || edge.to()
                            == endpoint
                },
            )
            .collect()
    }

    /// Validates all graph invariants.
    pub fn validate(
        &self,
    ) -> Result<(), DecodingGraphError> {
        if self.nodes.len()
            > MAX_DETECTION_NODES
        {
            return Err(
                DecodingGraphError::TooManyNodes {
                    limit:
                        MAX_DETECTION_NODES,
                },
            );
        }

        if self.boundaries.len()
            > MAX_BOUNDARY_NODES
        {
            return Err(
                DecodingGraphError::TooManyBoundaries {
                    limit:
                        MAX_BOUNDARY_NODES,
                },
            );
        }

        if self.edges.len()
            > MAX_GRAPH_EDGES
        {
            return Err(
                DecodingGraphError::TooManyEdges {
                    limit:
                        MAX_GRAPH_EDGES,
                },
            );
        }

        for (
            id,
            node,
        ) in &self.nodes
        {
            if *id != node.id() {
                return Err(
                    DecodingGraphError::NodeIdentityMismatch,
                );
            }

            match self
                .coordinate_index
                .get(node.coordinate())
            {
                Some(indexed_id)
                    if indexed_id == id => {}

                _ => {
                    return Err(
                        DecodingGraphError::CoordinateIndexCorruption,
                    );
                }
            }
        }

        let mut coordinates =
            BTreeSet::new();

        for node in self.nodes.values() {
            if !coordinates
                .insert(
                    node.coordinate()
                        .clone(),
                )
            {
                return Err(
                    DecodingGraphError::DuplicateNodeCoordinate,
                );
            }
        }

        for edge in self.edges.values() {
            self.validate_endpoint(
                edge.from(),
            )?;

            self.validate_endpoint(
                edge.to(),
            )?;
        }

        Ok(())
    }

    /// Builds a graph directly from detection events.
    ///
    /// Every detection event becomes one graph node.
    ///
    /// This method deliberately does not infer physical adjacency. Topology
    /// must be supplied explicitly through [`DecodingGraph::connect`].
    ///
    /// The stabilizer ID is used as the first spatial coordinate and the
    /// measurement round as time. This provides a deterministic generic
    /// representation suitable for initial graph construction.
    pub fn from_detection_events(
        events: &[DetectionEvent],
    ) -> Result<Self, DecodingGraphError> {
        let mut graph =
            Self::new();

        for event in events {
            let stabilizer =
                event
                    .stabilizer()
                    .index();

            if stabilizer
                > MAX_COORDINATE_ABS as usize
            {
                return Err(
                    DecodingGraphError::CoordinateOutOfRange {
                        value:
                            stabilizer as i64,
                    },
                );
            }

            let spatial =
                SpatialCoordinate::new(
                    vec![
                        stabilizer as i64,
                    ],
                )?;

            let coordinate =
                SpaceTimeCoordinate::new(
                    spatial,
                    event.round(),
                )?;

            graph.add_detection(
                coordinate,
                event.stabilizer(),
                event.confidence(),
            )?;
        }

        graph.validate()?;

        Ok(graph)
    }

    /// Connects two detection nodes with an explicitly supplied weight.
    pub fn connect_detections(
        &mut self,
        from: NodeId,
        to: NodeId,
        kind: EdgeKind,
        weight: EdgeWeight,
    ) -> Result<(), DecodingGraphError> {
        if !self.nodes.contains_key(&from)
        {
            return Err(
                DecodingGraphError::UnknownNode {
                    node: from,
                },
            );
        }

        if !self.nodes.contains_key(&to)
        {
            return Err(
                DecodingGraphError::UnknownNode {
                    node: to,
                },
            );
        }

        self.connect(
            GraphEndpoint::Detection(from),
            GraphEndpoint::Detection(to),
            kind,
            weight,
        )
    }

    /// Connects a detection node to a boundary.
    pub fn connect_boundary(
        &mut self,
        node: NodeId,
        boundary: BoundaryId,
        weight: EdgeWeight,
    ) -> Result<(), DecodingGraphError> {
        if !self.nodes.contains_key(&node)
        {
            return Err(
                DecodingGraphError::UnknownNode {
                    node,
                },
            );
        }

        if !self
            .boundaries
            .contains_key(&boundary)
        {
            return Err(
                DecodingGraphError::UnknownBoundary {
                    boundary,
                },
            );
        }

        self.connect(
            GraphEndpoint::Detection(node),
            GraphEndpoint::Boundary(
                boundary,
            ),
            EdgeKind::Boundary,
            weight,
        )
    }

    /// Returns all graph nodes at a measurement round.
    pub fn nodes_at_round(
        &self,
        round: MeasurementRound,
    ) -> Vec<&DetectionNode> {
        self.nodes
            .values()
            .filter(
                |node| {
                    node.coordinate()
                        .round()
                        == round
                },
            )
            .collect()
    }

    /// Returns all detection nodes belonging to a stabilizer.
    pub fn nodes_for_stabilizer(
        &self,
        stabilizer: StabilizerId,
    ) -> Vec<&DetectionNode> {
        self.nodes
            .values()
            .filter(
                |node| {
                    node.stabilizer()
                        == stabilizer
                },
            )
            .collect()
    }

    /// Returns the highest measurement round represented by the graph.
    pub fn maximum_round(
        &self,
    ) -> Option<MeasurementRound> {
        self.nodes
            .values()
            .map(
                |node| {
                    node.coordinate()
                        .round()
                },
            )
            .max()
    }

    /// Returns the earliest measurement round represented by the graph.
    pub fn minimum_round(
        &self,
    ) -> Option<MeasurementRound> {
        self.nodes
            .values()
            .map(
                |node| {
                    node.coordinate()
                        .round()
                },
            )
            .min()
    }

    /// Removes all graph contents.
    pub fn clear(
        &mut self,
    ) {
        self.nodes.clear();
        self.boundaries.clear();
        self.edges.clear();
        self.coordinate_index.clear();
        self.next_node_id = 0;
        self.next_boundary_id = 0;
    }

    fn validate_endpoint(
        &self,
        endpoint: GraphEndpoint,
    ) -> Result<(), DecodingGraphError> {
        match endpoint {
            GraphEndpoint::Detection(id) => {
                if !self.nodes.contains_key(&id)
                {
                    return Err(
                        DecodingGraphError::UnknownNode {
                            node: id,
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
                            boundary: id,
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
// Edge key
// ============================================================================

/// Canonical key for an undirected edge.
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
struct GraphEdgeKey {
    first: GraphEndpoint,
    second: GraphEndpoint,
}

impl GraphEdgeKey {
    fn new(
        first: GraphEndpoint,
        second: GraphEndpoint,
    ) -> Self {
        if first <= second {
            Self {
                first,
                second,
            }
        } else {
            Self {
                first: second,
                second: first,
            }
        }
    }
}

// ============================================================================
// Error type
// ============================================================================

/// Errors returned by decoding-graph construction and validation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum DecodingGraphError {
    /// Coordinate contains no dimensions.
    EmptyCoordinate,

    /// Coordinate has more dimensions than supported.
    TooManyDimensions {
        dimensions: usize,
        limit: usize,
    },

    /// Coordinate component exceeds the supported range.
    CoordinateOutOfRange {
        value: i64,
    },

    /// Two coordinates use different dimensions.
    DimensionMismatch,

    /// Measurement round exceeds the graph limit.
    InvalidRound {
        round: u64,
    },

    /// Timestamp exceeds the graph limit.
    InvalidTimestamp {
        timestamp: u64,
    },

    /// Probability is zero or greater than one.
    InvalidProbability,

    /// A graph weight exceeds the supported maximum.
    WeightOutOfRange {
        value: u64,
    },

    /// Weight addition overflowed.
    WeightOverflow,

    /// Generic arithmetic overflow.
    ArithmeticOverflow,

    /// Too many detection nodes.
    TooManyNodes {
        limit: usize,
    },

    /// Too many boundary nodes.
    TooManyBoundaries {
        limit: usize,
    },

    /// Too many graph edges.
    TooManyEdges {
        limit: usize,
    },

    /// Node identifier counter overflowed.
    NodeIdOverflow,

    /// Boundary identifier counter overflowed.
    BoundaryIdOverflow,

    /// Two detection nodes have the same space-time coordinate.
    DuplicateNodeCoordinate,

    /// The same edge was inserted twice.
    DuplicateEdge,

    /// An edge connects an endpoint that does not exist.
    UnknownNode {
        node: NodeId,
    },

    /// An edge references a boundary that does not exist.
    UnknownBoundary {
        boundary: BoundaryId,
    },

    /// An edge connects an endpoint to itself.
    SelfLoop,

    /// Node's stored identity disagrees with its map key.
    NodeIdentityMismatch,

    /// Coordinate index does not point to its node.
    CoordinateIndexCorruption,
}

impl fmt::Display
    for DecodingGraphError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyCoordinate => {
                write!(
                    f,
                    "space coordinate cannot be empty"
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

            Self::InvalidRound {
                round,
            } => {
                write!(
                    f,
                    "invalid measurement round: {round}"
                )
            }

            Self::InvalidTimestamp {
                timestamp,
            } => {
                write!(
                    f,
                    "invalid measurement timestamp: {timestamp}"
                )
            }

            Self::InvalidProbability => {
                write!(
                    f,
                    "probability must be greater than zero and no greater than one"
                )
            }

            Self::WeightOutOfRange {
                value,
            } => {
                write!(
                    f,
                    "graph weight {value} exceeds the maximum supported weight"
                )
            }

            Self::WeightOverflow => {
                write!(
                    f,
                    "graph-weight arithmetic overflow"
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    f,
                    "arithmetic overflow"
                )
            }

            Self::TooManyNodes {
                limit,
            } => {
                write!(
                    f,
                    "decoding graph exceeds node limit {limit}"
                )
            }

            Self::TooManyBoundaries {
                limit,
            } => {
                write!(
                    f,
                    "decoding graph exceeds boundary limit {limit}"
                )
            }

            Self::TooManyEdges {
                limit,
            } => {
                write!(
                    f,
                    "decoding graph exceeds edge limit {limit}"
                )
            }

            Self::NodeIdOverflow => {
                write!(
                    f,
                    "decoding graph node identifier overflow"
                )
            }

            Self::BoundaryIdOverflow => {
                write!(
                    f,
                    "decoding graph boundary identifier overflow"
                )
            }

            Self::DuplicateNodeCoordinate => {
                write!(
                    f,
                    "two detection nodes cannot occupy the same space-time coordinate"
                )
            }

            Self::DuplicateEdge => {
                write!(
                    f,
                    "decoding graph edge already exists"
                )
            }

            Self::UnknownNode {
                node,
            } => {
                write!(
                    f,
                    "unknown detection node {node}"
                )
            }

            Self::UnknownBoundary {
                boundary,
            } => {
                write!(
                    f,
                    "unknown boundary {boundary}"
                )
            }

            Self::SelfLoop => {
                write!(
                    f,
                    "self-loop edges are not permitted"
                )
            }

            Self::NodeIdentityMismatch => {
                write!(
                    f,
                    "detection-node identity invariant violated"
                )
            }

            Self::CoordinateIndexCorruption => {
                write!(
                    f,
                    "space-time coordinate index invariant violated"
                )
            }
        }
    }
}

impl std::error::Error
    for DecodingGraphError
{
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn round(
        value: u64,
    ) -> MeasurementRound {
        MeasurementRound::new(
            value,
        )
        .unwrap()
    }

    fn timestamp(
        value: u64,
    ) -> MeasurementTimestamp {
        MeasurementTimestamp::new(
            value,
        )
        .unwrap()
    }

    fn confidence()
        -> MeasurementConfidence
    {
        MeasurementConfidence::FULL
    }

    #[test]
    fn coordinate_creation_is_deterministic() {
        let coordinate =
            SpatialCoordinate::xy(
                2,
                3,
            )
            .unwrap();

        assert_eq!(
            coordinate.values(),
            &[2, 3]
        );

        assert_eq!(
            coordinate.to_string(),
            "(2,3)"
        );
    }

    #[test]
    fn empty_coordinate_is_rejected() {
        assert_eq!(
            SpatialCoordinate::new(
                Vec::new(),
            ),
            Err(
                DecodingGraphError::EmptyCoordinate
            )
        );
    }

    #[test]
    fn excessive_coordinate_dimensions_are_rejected() {
        assert_eq!(
            SpatialCoordinate::new(
                vec![
                    0;
                    MAX_SPATIAL_DIMENSIONS
                        + 1
                ],
            ),
            Err(
                DecodingGraphError::TooManyDimensions {
                    dimensions:
                        MAX_SPATIAL_DIMENSIONS
                            + 1,
                    limit:
                        MAX_SPATIAL_DIMENSIONS,
                }
            )
        );
    }

    #[test]
    fn manhattan_distance_is_correct() {
        let a =
            SpatialCoordinate::xy(
                1,
                2,
            )
            .unwrap();

        let b =
            SpatialCoordinate::xy(
                4,
                6,
            )
            .unwrap();

        assert_eq!(
            a.manhattan_distance(&b)
                .unwrap(),
            7
        );
    }

    #[test]
    fn dimension_mismatch_is_rejected() {
        let a =
            SpatialCoordinate::xy(
                1,
                2,
            )
            .unwrap();

        let b =
            SpatialCoordinate::xyz(
                1,
                2,
                3,
            )
            .unwrap();

        assert_eq!(
            a.manhattan_distance(&b),
            Err(
                DecodingGraphError::DimensionMismatch
            )
        );
    }

    #[test]
    fn graph_starts_empty() {
        let graph =
            DecodingGraph::new();

        assert!(
            graph.is_empty()
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
    }

    #[test]
    fn detection_node_can_be_added() {
        let mut graph =
            DecodingGraph::new();

        let coordinate =
            SpaceTimeCoordinate::new(
                SpatialCoordinate::xy(
                    1,
                    2,
                )
                .unwrap(),
                round(3),
            )
            .unwrap();

        let id =
            graph
                .add_detection(
                    coordinate,
                    StabilizerId::new(4),
                    confidence(),
                )
                .unwrap();

        assert_eq!(
            id.index(),
            0
        );

        assert_eq!(
            graph.node_count(),
            1
        );

        assert_eq!(
            graph.node(id)
                .unwrap()
                .stabilizer(),
            StabilizerId::new(4)
        );
    }

    #[test]
    fn duplicate_coordinate_is_rejected() {
        let mut graph =
            DecodingGraph::new();

        let coordinate =
            SpaceTimeCoordinate::new(
                SpatialCoordinate::xy(
                    1,
                    2,
                )
                .unwrap(),
                round(3),
            )
            .unwrap();

        graph
            .add_detection(
                coordinate.clone(),
                StabilizerId::new(0),
                confidence(),
            )
            .unwrap();

        assert_eq!(
            graph.add_detection(
                coordinate,
                StabilizerId::new(1),
                confidence(),
            ),
            Err(
                DecodingGraphError::DuplicateNodeCoordinate
            )
        );
    }

    #[test]
    fn coordinate_index_returns_node() {
        let mut graph =
            DecodingGraph::new();

        let coordinate =
            SpaceTimeCoordinate::new(
                SpatialCoordinate::xy(
                    7,
                    9,
                )
                .unwrap(),
                round(4),
            )
            .unwrap();

        let id =
            graph
                .add_detection(
                    coordinate.clone(),
                    StabilizerId::new(2),
                    confidence(),
                )
                .unwrap();

        assert_eq!(
            graph.node_at(&coordinate),
            Some(id)
        );
    }

    #[test]
    fn boundary_can_be_added() {
        let mut graph =
            DecodingGraph::new();

        let coordinate =
            SpaceTimeCoordinate::new(
                SpatialCoordinate::xy(
                    0,
                    0,
                )
                .unwrap(),
                round(0),
            )
            .unwrap();

        let boundary =
            graph
                .add_boundary(
                    coordinate,
                )
                .unwrap();

        assert_eq!(
            boundary.index(),
            0
        );

        assert_eq!(
            graph.boundary_count(),
            1
        );
    }

    #[test]
    fn edge_can_connect_detection_nodes() {
        let mut graph =
            DecodingGraph::new();

        let first =
            graph
                .add_detection(
                    SpaceTimeCoordinate::new(
                        SpatialCoordinate::xy(
                            0,
                            0,
                        )
                        .unwrap(),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(0),
                    confidence(),
                )
                .unwrap();

        let second =
            graph
                .add_detection(
                    SpaceTimeCoordinate::new(
                        SpatialCoordinate::xy(
                            1,
                            0,
                        )
                        .unwrap(),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(1),
                    confidence(),
                )
                .unwrap();

        graph
            .connect_detections(
                first,
                second,
                EdgeKind::Spatial,
                EdgeWeight::new(10)
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(
            graph.edge_count(),
            1
        );
    }

    #[test]
    fn duplicate_undirected_edge_is_rejected() {
        let mut graph =
            DecodingGraph::new();

        let first =
            graph
                .add_detection(
                    SpaceTimeCoordinate::new(
                        SpatialCoordinate::xy(
                            0,
                            0,
                        )
                        .unwrap(),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(0),
                    confidence(),
                )
                .unwrap();

        let second =
            graph
                .add_detection(
                    SpaceTimeCoordinate::new(
                        SpatialCoordinate::xy(
                            1,
                            0,
                        )
                        .unwrap(),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(1),
                    confidence(),
                )
                .unwrap();

        graph
            .connect_detections(
                first,
                second,
                EdgeKind::Spatial,
                EdgeWeight::new(10)
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(
            graph.connect_detections(
                second,
                first,
                EdgeKind::Spatial,
                EdgeWeight::new(10)
                    .unwrap(),
            ),
            Err(
                DecodingGraphError::DuplicateEdge
            )
        );
    }

    #[test]
    fn self_loop_is_rejected() {
        let endpoint =
            GraphEndpoint::Detection(
                NodeId::new(0),
            );

        assert_eq!(
            GraphEdge::new(
                endpoint,
                endpoint,
                EdgeKind::Spatial,
                EdgeWeight::ZERO,
            ),
            Err(
                DecodingGraphError::SelfLoop
            )
        );
    }

    #[test]
    fn unknown_detection_node_is_rejected() {
        let mut graph =
            DecodingGraph::new();

        assert_eq!(
            graph.connect(
                GraphEndpoint::Detection(
                    NodeId::new(999),
                ),
                GraphEndpoint::Detection(
                    NodeId::new(1000),
                ),
                EdgeKind::Spatial,
                EdgeWeight::ZERO,
            ),
            Err(
                DecodingGraphError::UnknownNode {
                    node:
                        NodeId::new(999),
                }
            )
        );
    }

    #[test]
    fn boundary_connection_is_supported() {
        let mut graph =
            DecodingGraph::new();

        let node =
            graph
                .add_detection(
                    SpaceTimeCoordinate::new(
                        SpatialCoordinate::xy(
                            2,
                            2,
                        )
                        .unwrap(),
                        round(1),
                    )
                    .unwrap(),
                    StabilizerId::new(2),
                    confidence(),
                )
                .unwrap();

        let boundary =
            graph
                .add_boundary(
                    SpaceTimeCoordinate::new(
                        SpatialCoordinate::xy(
                            0,
                            2,
                        )
                        .unwrap(),
                        round(1),
                    )
                    .unwrap(),
                )
                .unwrap();

        graph
            .connect_boundary(
                node,
                boundary,
                EdgeWeight::new(5)
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(
            graph.edge_count(),
            1
        );

        assert!(
            graph
                .edges()
                .next()
                .unwrap()
                .touches_boundary()
        );
    }

    #[test]
    fn graph_validation_succeeds_for_valid_graph() {
        let mut graph =
            DecodingGraph::new();

        let first =
            graph
                .add_detection(
                    SpaceTimeCoordinate::new(
                        SpatialCoordinate::xy(
                            0,
                            0,
                        )
                        .unwrap(),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(0),
                    confidence(),
                )
                .unwrap();

        let second =
            graph
                .add_detection(
                    SpaceTimeCoordinate::xy(
                        1,
                        0,
                    )
                    .unwrap()
                    .into_space_time(
                        round(0),
                    ),
                    StabilizerId::new(1),
                    confidence(),
                )
                .unwrap();

        graph
            .connect_detections(
                first,
                second,
                EdgeKind::Spatial,
                EdgeWeight::new(10)
                    .unwrap(),
            )
            .unwrap();

        assert!(
            graph.validate().is_ok()
        );
    }

    #[test]
    fn graph_can_be_created_from_detection_events() {
        let mut syndrome =
            super::super::syndrome::Syndrome::new(
                round(1),
                timestamp(100),
            );

        syndrome
            .insert(
                super::super::syndrome::SyndromeMeasurement::new(
                    StabilizerId::new(0),
                    true,
                    confidence(),
                ),
            )
            .unwrap();

        let previous =
            super::super::syndrome::Syndrome::new(
                round(0),
                timestamp(90),
            );

        let mut previous =
            previous;

        previous
            .insert(
                super::super::syndrome::SyndromeMeasurement::new(
                    StabilizerId::new(0),
                    false,
                    confidence(),
                ),
            )
            .unwrap();

        let events =
            syndrome
                .detection_events_against(
                    &previous,
                )
                .unwrap();

        let graph =
            DecodingGraph::from_detection_events(
                &events,
            )
            .unwrap();

        assert_eq!(
            graph.node_count(),
            1
        );

        assert_eq!(
            graph.edge_count(),
            0
        );

        assert!(
            graph.validate().is_ok()
        );
    }

    #[test]
    fn nodes_can_be_filtered_by_round() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_detection(
                SpaceTimeCoordinate::new(
                    SpatialCoordinate::xy(
                        0,
                        0,
                    )
                    .unwrap(),
                    round(0),
                )
                .unwrap(),
                StabilizerId::new(0),
                confidence(),
            )
            .unwrap();

        graph
            .add_detection(
                SpaceTimeCoordinate::new(
                    SpatialCoordinate::xy(
                        1,
                        0,
                    )
                    .unwrap(),
                    round(1),
                )
                .unwrap(),
                StabilizerId::new(1),
                confidence(),
            )
            .unwrap();

        assert_eq!(
            graph
                .nodes_at_round(
                    round(0),
                )
                .len(),
            1
        );

        assert_eq!(
            graph
                .nodes_at_round(
                    round(1),
                )
                .len(),
            1
        );
    }

    #[test]
    fn nodes_can_be_filtered_by_stabilizer() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_detection(
                SpaceTimeCoordinate::new(
                    SpatialCoordinate::xy(
                        0,
                        0,
                    )
                    .unwrap(),
                    round(0),
                )
                .unwrap(),
                StabilizerId::new(5),
                confidence(),
            )
            .unwrap();

        graph
            .add_detection(
                SpaceTimeCoordinate::new(
                    SpatialCoordinate::xy(
                        0,
                        1,
                    )
                    .unwrap(),
                    round(1),
                )
                .unwrap(),
                StabilizerId::new(5),
                confidence(),
            )
            .unwrap();

        assert_eq!(
            graph
                .nodes_for_stabilizer(
                    StabilizerId::new(5),
                )
                .len(),
            2
        );
    }

    #[test]
    fn minimum_and_maximum_rounds_are_correct() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_detection(
                SpaceTimeCoordinate::new(
                    SpatialCoordinate::xy(
                        0,
                        0,
                    )
                    .unwrap(),
                    round(2),
                )
                .unwrap(),
                StabilizerId::new(0),
                confidence(),
            )
            .unwrap();

        graph
            .add_detection(
                SpaceTimeCoordinate::new(
                    SpatialCoordinate::xy(
                        1,
                        0,
                    )
                    .unwrap(),
                    round(7),
                )
                .unwrap(),
                StabilizerId::new(1),
                confidence(),
            )
            .unwrap();

        assert_eq!(
            graph.minimum_round(),
            Some(round(2))
        );

        assert_eq!(
            graph.maximum_round(),
            Some(round(7))
        );
    }

    #[test]
    fn probability_weight_is_finite_and_bounded() {
        let weight =
            EdgeWeight::from_probability(
                PROBABILITY_SCALE / 2,
            )
            .unwrap();

        assert!(
            weight.value()
                <= MAX_WEIGHT
        );
    }

    #[test]
    fn invalid_probability_is_rejected() {
        assert_eq!(
            EdgeWeight::from_probability(
                0,
            ),
            Err(
                DecodingGraphError::InvalidProbability
            )
        );

        assert_eq!(
            EdgeWeight::from_probability(
                PROBABILITY_SCALE + 1,
            ),
            Err(
                DecodingGraphError::InvalidProbability
            )
        );
    }

    #[test]
    fn weight_addition_is_checked() {
        let a =
            EdgeWeight::new(
                MAX_WEIGHT / 2,
            )
            .unwrap();

        let b =
            EdgeWeight::new(
                MAX_WEIGHT / 2,
            )
            .unwrap();

        assert_eq!(
            a.checked_add(b)
                .unwrap()
                .value(),
            MAX_WEIGHT
        );
    }

    #[test]
    fn graph_clear_is_complete() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_detection(
                SpaceTimeCoordinate::new(
                    SpatialCoordinate::xy(
                        0,
                        0,
                    )
                    .unwrap(),
                    round(0),
                )
                .unwrap(),
                StabilizerId::new(0),
                confidence(),
            )
            .unwrap();

        graph.clear();

        assert!(
            graph.is_empty()
        );

        assert_eq!(
            graph.node_count(),
            0
        );

        assert_eq!(
            graph.edge_count(),
            0
        );
    }
}

// ============================================================================
// Convenience conversion
// ============================================================================

impl SpatialCoordinate {
    /// Converts a spatial coordinate into a space-time coordinate.
    pub fn into_space_time(
        self,
        round: MeasurementRound,
    ) -> SpaceTimeCoordinate {
        SpaceTimeCoordinate {
            spatial: self,
            round,
        }
    }
}