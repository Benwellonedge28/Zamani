//! Zamani Quantum Error Correction — Deterministic Space-Time Decoding Graph.
//!
//! This module is the canonical intermediate representation between syndrome
//! processing and QEC decoders such as MWPM and Union-Find.
//!
//! # Architectural contract
//!
//! ```text
//! Syndrome / QPU / Replay
//!          |
//!          v
//!   DetectionEvent
//!          |
//!          v
//!   DecodingGraph
//!      |       |
//!      |       +--> Boundaries
//!      |
//!      +--> Detection Nodes
//!      |
//!      +--> Weighted Edges
//!          |
//!          v
//!     MWPM / Union-Find
//! ```
//!
//! ## This module owns
//!
//! - validated spatial coordinates;
//! - validated space-time coordinates;
//! - stable detection-node identifiers;
//! - stable boundary identifiers;
//! - detection nodes;
//! - boundary nodes;
//! - graph endpoints;
//! - graph edges;
//! - bounded deterministic edge weights;
//! - graph construction;
//! - graph indexing;
//! - duplicate detection;
//! - deterministic iteration;
//! - graph validation;
//! - allocation-free resource preflight;
//! - graph memory estimation;
//! - checked arithmetic required by graph construction.
//!
//! ## This module does NOT own
//!
//! - syndrome extraction;
//! - stabilizer algebra;
//! - surface-code topology;
//! - physical noise generation;
//! - decoder algorithms;
//! - matching;
//! - correction application;
//! - logical-equivalence classification;
//! - QPU access;
//! - scheduling;
//! - distributed execution.
//!
//! ## Integration contract
//!
//! `limits.rs`
//!     Canonical declarative resource policy.
//!
//! `syndrome.rs`
//!     Supplies validated `DetectionEvent` values.
//!
//! `surface_code.rs`
//!     Supplies physical coordinates for detection events and boundaries.
//!
//! `memory.rs`
//!     Owns actual memory reservation/enforcement. This module performs
//!     allocation-free graph preflight before graph mutation.
//!
//! `resources.rs`
//!     Owns runtime accounting. This module does not create a second runtime
//!     accounting system.
//!
//! `mwpm.rs`
//! `union_find.rs`
//!     Consume this graph through its stable public API.
//!
//! `distance.rs`
//!     May use graph resource estimation and validation but must not make the
//!     graph responsible for distance verification.
//!
//! `streaming.rs` / `partition.rs`
//!     May construct bounded graph fragments using the same limits contract.
//!
//! # Resource ordering
//!
//! Every graph mutation follows:
//!
//! ```text
//! input validation
//!       |
//!       v
//! duplicate / endpoint validation
//!       |
//!       v
//! checked capacity calculation
//!       |
//!       v
//! QecLimits validation
//!       |
//!       v
//! memory preflight
//!       |
//!       v
//! graph mutation
//! ```
//!
//! The graph never silently wraps arithmetic.
//!
//! # Determinism
//!
//! - `BTreeMap` and `BTreeSet` provide canonical ordering.
//! - Endpoint pairs are canonicalized before storage.
//! - Node identifiers are monotonic and checked.
//! - Edge weights are integer-valued.
//! - Probability conversion uses deterministic fixed-point arithmetic.
//! - No floating-point value participates in graph identity or ordering.
//!
//! Rust compatibility target: Rust 1.97.1.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use super::limits::{LimitError, LimitKind, QecLimits};
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
/// This is a representation invariant, not an execution-resource policy.
pub const MAX_SPATIAL_DIMENSIONS: usize = 8;

/// Maximum absolute coordinate component accepted by the graph.
///
/// This prevents malformed external input from producing pathological
/// coordinate arithmetic while leaving workload sizing to `QecLimits`.
pub const MAX_COORDINATE_ABS: i64 = 1_000_000_000;

/// Maximum valid graph round.
///
/// `MeasurementRound` already enforces the same invariant.
pub const MAX_GRAPH_ROUND: u64 = u64::MAX - 1;

/// Fixed-point probability scale.
///
/// `1_000_000_000_000` represents probability `1.0`.
pub const PROBABILITY_SCALE: u64 = 1_000_000_000_000;

/// Fixed-point logarithm scale used for edge weights.
pub const LOG_WEIGHT_SCALE: i128 = 1_000_000;

/// Maximum finite encoded edge weight.
pub const MAX_WEIGHT: u64 = 1_000_000_000_000_000;

/// Fixed-point approximation of `ln(2) * LOG_WEIGHT_SCALE`.
///
/// 0.693147 * 1_000_000 = 693147.
const LN_2_SCALED: i128 = 693_147;

/// Conservative per-detection-node memory estimate.
const ESTIMATED_NODE_BYTES: u64 = 128;

/// Conservative per-boundary-node memory estimate.
const ESTIMATED_BOUNDARY_BYTES: u64 = 96;

/// Conservative per-edge memory estimate.
const ESTIMATED_EDGE_BYTES: u64 = 96;

/// Conservative graph bookkeeping overhead.
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

    /// Returns the dimensionality.
    #[must_use]
    pub fn dimensions(&self) -> usize {
        self.values.len()
    }

    /// Returns one coordinate component.
    #[must_use]
    pub fn get(
        &self,
        dimension: usize,
    ) -> Option<i64> {
        self.values.get(dimension).copied()
    }

    /// Returns all coordinate components.
    #[must_use]
    pub fn values(&self) -> &[i64] {
        &self.values
    }

    /// Computes Manhattan distance with checked arithmetic.
    pub fn manhattan_distance(
        &self,
        other: &Self,
    ) -> Result<u64, DecodingGraphError> {
        if self.dimensions() != other.dimensions() {
            return Err(
                DecodingGraphError::DimensionMismatch,
            );
        }

        let mut distance = 0_u64;

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

            let magnitude =
                u64::try_from(absolute).map_err(|_| {
                    DecodingGraphError::ArithmeticOverflow
                })?;

            distance = distance
                .checked_add(magnitude)
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
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str("(")?;

        for (index, value) in
            self.values.iter().enumerate()
        {
            if index != 0 {
                formatter.write_str(",")?;
            }

            write!(formatter, "{value}")?;
        }

        formatter.write_str(")")
    }
}

// ============================================================================
// Space-time coordinate
// ============================================================================

/// Position in the decoding space-time lattice.
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
    #[must_use]
    pub fn spatial(&self) -> &SpatialCoordinate {
        &self.spatial
    }

    /// Returns the measurement round.
    #[must_use]
    pub const fn round(&self) -> MeasurementRound {
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
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "n{}", self.0)
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
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for BoundaryId {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(formatter, "b{}", self.0)
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
    #[must_use]
    pub const fn new(
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

    /// Returns the node identifier.
    #[must_use]
    pub const fn id(&self) -> NodeId {
        self.id
    }

    /// Returns the node coordinate.
    #[must_use]
    pub fn coordinate(
        &self,
    ) -> &SpaceTimeCoordinate {
        &self.coordinate
    }

    /// Returns the stabilizer identifier.
    #[must_use]
    pub const fn stabilizer(
        &self,
    ) -> StabilizerId {
        self.stabilizer
    }

    /// Returns measurement confidence.
    #[must_use]
    pub const fn confidence(
        &self,
    ) -> MeasurementConfidence {
        self.confidence
    }
}

// ============================================================================
// Boundary node
// ============================================================================

/// Physical or logical decoding boundary.
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
    #[must_use]
    pub const fn new(
        id: BoundaryId,
        coordinate: SpaceTimeCoordinate,
    ) -> Self {
        Self {
            id,
            coordinate,
        }
    }

    /// Returns the boundary identifier.
    #[must_use]
    pub const fn id(&self) -> BoundaryId {
        self.id
    }

    /// Returns the boundary coordinate.
    #[must_use]
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
    /// Detection event endpoint.
    Detection(NodeId),

    /// Boundary endpoint.
    Boundary(BoundaryId),
}

impl GraphEndpoint {
    /// Returns whether this is a detection endpoint.
    #[must_use]
    pub const fn is_detection(self) -> bool {
        matches!(
            self,
            Self::Detection(_)
        )
    }

    /// Returns whether this is a boundary endpoint.
    #[must_use]
    pub const fn is_boundary(self) -> bool {
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

    /// Combined space-time propagation.
    SpaceTime,

    /// Connection to a physical/logical boundary.
    Boundary,

    /// Backend-specific path semantics.
    Custom,
}

// ============================================================================
// Edge weight
// ============================================================================

/// Bounded integer edge weight.
///
/// Decoder algorithms must use this integer representation rather than
/// floating-point values.
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
    /// Zero-cost edge.
    pub const ZERO: Self = Self(0);

    /// Creates a validated edge weight.
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
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }

    /// Converts fixed-point probability to deterministic fixed-point
    /// negative log-likelihood weight.
    ///
    /// The conversion is deliberately integer-only.
    ///
    /// `probability == PROBABILITY_SCALE` represents probability `1.0`.
    ///
    /// The result is approximately:
    ///
    /// `-ln(p) * LOG_WEIGHT_SCALE`
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

        let scaled_probability =
            probability as i128;

        let mut normalized =
            scaled_probability;

        let mut exponent: i32 = 0;

        /*
         * Normalize p into approximately [0.5, 1].
         *
         * This keeps the atanh-series convergence fast and deterministic.
         */
        while normalized < PROBABILITY_SCALE as i128 / 2 {
            normalized = normalized
                .checked_mul(2)
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

            exponent = exponent
                .checked_add(1)
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;
        }

        while normalized
            > PROBABILITY_SCALE as i128
        {
            normalized /= 2;

            exponent = exponent
                .checked_sub(1)
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;
        }

        /*
         * ln(x) =
         *
         *     2 * (z + z^3/3 + z^5/5 + ...)
         *
         * where
         *
         *     z = (x - 1) / (x + 1).
         *
         * All values are represented at LOG_WEIGHT_SCALE precision.
         */
        let scale =
            LOG_WEIGHT_SCALE;

        let numerator =
            normalized
                .checked_sub(
                    PROBABILITY_SCALE as i128,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let denominator =
            normalized
                .checked_add(
                    PROBABILITY_SCALE as i128,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let mut z = numerator
            .checked_mul(scale)
            .ok_or(
                DecodingGraphError::ArithmeticOverflow,
            )?
            / denominator;

        let mut z_power = z;
        let mut series = z;

        /*
         * With normalization to [0.5, 1], |z| <= 1/3.
         * Thirty-one terms provide ample deterministic precision for graph
         * ordering while keeping the calculation bounded.
         */
        for denominator_term in
            (3_i128..=31_i128).step_by(2)
        {
            z_power = z_power
                .checked_mul(z)
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?
                / scale;

            let term = z_power
                / denominator_term;

            series = series
                .checked_add(term)
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;
        }

        let normalized_ln = series
            .checked_mul(2)
            .ok_or(
                DecodingGraphError::ArithmeticOverflow,
            )?;

        /*
         * ln(p) = ln(normalized) - exponent * ln(2)
         */
        let exponent_component =
            (exponent as i128)
                .checked_mul(LN_2_SCALED)
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let ln_probability =
            normalized_ln
                .checked_sub(
                    exponent_component,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let negative_log =
            ln_probability
                .checked_neg()
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let bounded =
            if negative_log < 0 {
                0
            } else {
                u64::try_from(
                    negative_log,
                )
                .map_err(|_| {
                    DecodingGraphError::WeightOutOfRange {
                        value: u64::MAX,
                    }
                })?
            };

        Self::new(bounded)
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
    /// Creates an edge and canonicalizes its endpoints.
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

        let (first, second) =
            canonical_endpoints(
                first,
                second,
            );

        Ok(Self {
            first,
            second,
            weight,
            kind,
        })
    }

    /// Returns first canonical endpoint.
    #[must_use]
    pub const fn first(
        &self,
    ) -> GraphEndpoint {
        self.first
    }

    /// Returns second canonical endpoint.
    #[must_use]
    pub const fn second(
        &self,
    ) -> GraphEndpoint {
        self.second
    }

    /// Returns edge weight.
    #[must_use]
    pub const fn weight(
        &self,
    ) -> EdgeWeight {
        self.weight
    }

    /// Returns semantic edge kind.
    #[must_use]
    pub const fn kind(
        &self,
    ) -> EdgeKind {
        self.kind
    }

    /// Returns whether the edge touches a boundary.
    #[must_use]
    pub const fn touches_boundary(
        &self,
    ) -> bool {
        self.first.is_boundary()
            || self.second.is_boundary()
    }

    /// Returns the endpoint opposite `endpoint`.
    #[must_use]
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

/// Decoding-graph construction configuration.
///
/// `QecLimits` remains the only production resource-policy source.
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
    /// Creates a graph configuration.
    pub fn new(
        limits: QecLimits,
    ) -> Result<Self, DecodingGraphError> {
        limits
            .validate()
            .map_err(
                DecodingGraphError::Limit,
            )?;

        Ok(Self { limits })
    }

    /// Returns the authoritative resource policy.
    #[must_use]
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

/// Allocation-free graph resource estimate.
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
    /// Calculates a graph resource estimate without allocating graph storage.
    pub fn calculate(
        detection_nodes: usize,
        boundary_nodes: usize,
        edges: usize,
    ) -> Result<Self, DecodingGraphError> {
        let detection_nodes_u64 =
            u64::try_from(
                detection_nodes,
            )
            .map_err(|_| {
                DecodingGraphError::ArithmeticOverflow
            })?;

        let boundary_nodes_u64 =
            u64::try_from(
                boundary_nodes,
            )
            .map_err(|_| {
                DecodingGraphError::ArithmeticOverflow
            })?;

        let edges_u64 =
            u64::try_from(edges)
                .map_err(|_| {
                    DecodingGraphError::ArithmeticOverflow
                })?;

        let detection_bytes =
            detection_nodes_u64
                .checked_mul(
                    ESTIMATED_NODE_BYTES,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let boundary_bytes =
            boundary_nodes_u64
                .checked_mul(
                    ESTIMATED_BOUNDARY_BYTES,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let edge_bytes =
            edges_u64
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
                .and_then(|value| {
                    value.checked_add(
                        boundary_bytes,
                    )
                })
                .and_then(|value| {
                    value.checked_add(
                        edge_bytes,
                    )
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

    /// Number of detection nodes.
    #[must_use]
    pub const fn detection_nodes(
        self,
    ) -> usize {
        self.detection_nodes
    }

    /// Number of boundary nodes.
    #[must_use]
    pub const fn boundary_nodes(
        self,
    ) -> usize {
        self.boundary_nodes
    }

    /// Number of edges.
    #[must_use]
    pub const fn edges(
        self,
    ) -> usize {
        self.edges
    }

    /// Estimated memory consumption.
    #[must_use]
    pub const fn estimated_memory_bytes(
        self,
    ) -> u64 {
        self.estimated_memory_bytes
    }

    /// Validates the estimate against canonical QEC limits.
    pub fn validate_against(
        &self,
        limits: &QecLimits,
    ) -> Result<(), DecodingGraphError> {
        limits
            .validate()
            .map_err(
                DecodingGraphError::Limit,
            )?;

        let total_nodes =
            self.detection_nodes
                .checked_add(
                    self.boundary_nodes,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        limits
            .validate_graph(
                total_nodes,
                self.edges,
            )
            .map_err(
                DecodingGraphError::Limit,
            )?;

        limits
            .validate_memory(
                self.estimated_memory_bytes,
            )
            .map_err(
                DecodingGraphError::Limit,
            )?;

        Ok(())
    }
}

// ============================================================================
// Decoding graph
// ============================================================================

/// Deterministic resource-bounded decoding graph.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct DecodingGraph {
    config: DecodingGraphConfig,

    nodes: BTreeMap<
        NodeId,
        DetectionNode,
    >,

    boundaries: BTreeMap<
        BoundaryId,
        BoundaryNode,
    >,

    edges: BTreeMap<
        (GraphEndpoint, GraphEndpoint),
        GraphEdge,
    >,

    detection_index: BTreeMap<
        DetectionKey,
        NodeId,
    >,

    boundary_index: BTreeMap<
        SpaceTimeCoordinate,
        BoundaryId,
    >,

    next_node_id: usize,
    next_boundary_id: usize,
}

impl DecodingGraph {
    /// Creates an empty graph with canonical default limits.
    ///
    /// The default policy is a repository invariant and therefore this
    /// constructor cannot fail under normal operation.
    pub fn new() -> Self {
        Self::with_config(
            DecodingGraphConfig::default(),
        )
        .expect(
            "canonical default QEC limits must be valid",
        )
    }

    /// Creates an empty graph with explicit limits.
    pub fn new_with_limits(
        limits: &QecLimits,
    ) -> Result<Self, DecodingGraphError> {
        let config =
            DecodingGraphConfig::new(
                *limits,
            )?;

        Self::with_config(config)
    }

    /// Creates an empty graph from explicit configuration.
    pub fn with_config(
        config: DecodingGraphConfig,
    ) -> Result<Self, DecodingGraphError> {
        config
            .limits()
            .validate()
            .map_err(
                DecodingGraphError::Limit,
            )?;

        Ok(Self {
            config,
            nodes: BTreeMap::new(),
            boundaries: BTreeMap::new(),
            edges: BTreeMap::new(),
            detection_index: BTreeMap::new(),
            boundary_index: BTreeMap::new(),
            next_node_id: 0,
            next_boundary_id: 0,
        })
    }

    /// Returns graph configuration.
    #[must_use]
    pub const fn config(
        &self,
    ) -> DecodingGraphConfig {
        self.config
    }

    /// Returns the authoritative QEC limits.
    #[must_use]
    pub const fn limits(
        &self,
    ) -> QecLimits {
        self.config.limits()
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

        estimate.validate_against(
            limits,
        )?;

        Ok(estimate)
    }

    /// Returns the number of detection nodes.
    #[must_use]
    pub fn node_count(
        &self,
    ) -> usize {
        self.nodes.len()
    }

    /// Returns the number of boundaries.
    #[must_use]
    pub fn boundary_count(
        &self,
    ) -> usize {
        self.boundaries.len()
    }

    /// Returns total graph-node count, including boundaries.
    #[must_use]
    pub fn total_node_count(
        &self,
    ) -> usize {
        self.node_count()
            .saturating_add(
                self.boundary_count(),
            )
    }

    /// Returns the number of graph edges.
    #[must_use]
    pub fn edge_count(
        &self,
    ) -> usize {
        self.edges.len()
    }

    /// Returns whether there are no detection nodes.
    #[must_use]
    pub fn is_empty(
        &self,
    ) -> bool {
        self.nodes.is_empty()
    }

    /// Iterates over detection nodes in deterministic ID order.
    pub fn nodes(
        &self,
    ) -> impl Iterator<
        Item = &DetectionNode,
    > {
        self.nodes.values()
    }

    /// Alias for `nodes()`.
    pub fn detection_nodes(
        &self,
    ) -> impl Iterator<
        Item = &DetectionNode,
    > {
        self.nodes.values()
    }

    /// Iterates over boundaries in deterministic ID order.
    pub fn boundaries(
        &self,
    ) -> impl Iterator<
        Item = &BoundaryNode,
    > {
        self.boundaries.values()
    }

    /// Iterates over edges in canonical endpoint order.
    pub fn edges(
        &self,
    ) -> impl Iterator<
        Item = &GraphEdge,
    > {
        self.edges.values()
    }

    /// Gets a detection node.
    #[must_use]
    pub fn node(
        &self,
        id: NodeId,
    ) -> Option<&DetectionNode> {
        self.nodes.get(&id)
    }

    /// Gets a boundary node.
    #[must_use]
    pub fn boundary(
        &self,
        id: BoundaryId,
    ) -> Option<&BoundaryNode> {
        self.boundaries.get(&id)
    }

    /// Adds a validated active detection event with a supplied spatial
    /// coordinate.
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

    /// Adds a detection node after complete preflight.
    pub fn add_detection_node(
        &mut self,
        coordinate: SpaceTimeCoordinate,
        stabilizer: StabilizerId,
        confidence: MeasurementConfidence,
    ) -> Result<NodeId, DecodingGraphError> {
        let key = DetectionKey {
            coordinate: coordinate.clone(),
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

        /*
         * Capacity and memory checks happen before any mutation.
         */
        self.check_node_capacity(1)?;

        let id =
            NodeId::new(
                self.next_node_id,
            );

        let next_id =
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
            .insert(
                key,
                id,
            );

        self.next_node_id = next_id;

        Ok(id)
    }

    /// Adds a physical or logical boundary.
    pub fn add_boundary(
        &mut self,
        coordinate: SpaceTimeCoordinate,
    ) -> Result<
        BoundaryId,
        DecodingGraphError,
    > {
        if self
            .boundary_index
            .contains_key(&coordinate)
        {
            return Err(
                DecodingGraphError::DuplicateBoundary,
            );
        }

        self.check_node_capacity(1)?;

        let id =
            BoundaryId::new(
                self.next_boundary_id,
            );

        let next_id =
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
            .insert(
                coordinate,
                id,
            );

        self.next_boundary_id =
            next_id;

        Ok(id)
    }

    /// Adds a graph edge.
    ///
    /// Endpoint existence is checked before graph mutation.
    pub fn add_edge(
        &mut self,
        first: GraphEndpoint,
        second: GraphEndpoint,
        weight: EdgeWeight,
        kind: EdgeKind,
    ) -> Result<(), DecodingGraphError> {
        self.ensure_endpoint_exists(
            first,
        )?;

        self.ensure_endpoint_exists(
            second,
        )?;

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
    #[must_use]
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

    /// Returns all edges incident to an endpoint.
    ///
    /// The returned vector is deterministic.
    #[must_use]
    pub fn incident_edges(
        &self,
        endpoint: GraphEndpoint,
    ) -> Vec<&GraphEdge> {
        self.edges
            .values()
            .filter(
                |edge| {
                    edge.first()
                        == endpoint
                        || edge.second()
                            == endpoint
                },
            )
            .collect()
    }

    /// Returns neighbouring endpoints in deterministic order.
    #[must_use]
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
    #[must_use]
    pub fn neighbors(
        &self,
        endpoint: GraphEndpoint,
    ) -> Vec<GraphEndpoint> {
        self.neighbours(endpoint)
    }

    /// Validates the entire graph.
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

        for (id, node) in
            &self.nodes
        {
            if *id != node.id() {
                return Err(
                    DecodingGraphError::NodeIdMismatch,
                );
            }

            if node
                .coordinate()
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

            let key =
                DetectionKey {
                    coordinate:
                        node.coordinate()
                            .clone(),
                    stabilizer:
                        node.stabilizer(),
                };

            if self
                .detection_index
                .get(&key)
                != Some(id)
            {
                return Err(
                    DecodingGraphError::IndexMismatch,
                );
            }
        }

        for (id, boundary) in
            &self.boundaries
        {
            if *id != boundary.id() {
                return Err(
                    DecodingGraphError::BoundaryIdMismatch,
                );
            }

            if self
                .boundary_index
                .get(
                    boundary.coordinate(),
                )
                != Some(id)
            {
                return Err(
                    DecodingGraphError::IndexMismatch,
                );
            }
        }

        let mut seen_edges =
            BTreeSet::new();

        for edge in
            self.edges.values()
        {
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

            if !seen_edges.insert(
                key,
            ) {
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

            if edge.kind()
                == EdgeKind::Boundary
                && !edge
                    .touches_boundary()
            {
                return Err(
                    DecodingGraphError::BoundaryEdgeRequiresBoundary,
                );
            }
        }

        Ok(())
    }

    /// Returns the current graph resource estimate.
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
    // Capacity checks
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

        let total_nodes =
            requested_detection
                .checked_add(
                    self.boundary_count(),
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        self.limits()
            .validate_graph(
                total_nodes,
                self.edge_count(),
            )
            .map_err(
                DecodingGraphError::Limit,
            )?;

        let estimate =
            GraphResourceEstimate::calculate(
                requested_detection,
                self.boundary_count(),
                self.edge_count(),
            )?;

        self.limits()
            .validate_memory(
                estimate
                    .estimated_memory_bytes(),
            )
            .map_err(
                DecodingGraphError::Limit,
            )?;

        Ok(())
    }

    fn check_edge_capacity(
        &self,
        additional: usize,
    ) -> Result<(), DecodingGraphError> {
        let requested_edges =
            self.edge_count()
                .checked_add(
                    additional,
                )
                .ok_or(
                    DecodingGraphError::ArithmeticOverflow,
                )?;

        let total_nodes =
            self.total_node_count();

        self.limits()
            .validate_graph(
                total_nodes,
                requested_edges,
            )
            .map_err(
                DecodingGraphError::Limit,
            )?;

        let estimate =
            GraphResourceEstimate::calculate(
                self.node_count(),
                self.boundary_count(),
                requested_edges,
            )?;

        self.limits()
            .validate_memory(
                estimate
                    .estimated_memory_bytes(),
            )
            .map_err(
                DecodingGraphError::Limit,
            )?;

        Ok(())
    }

    fn ensure_endpoint_exists(
        &self,
        endpoint: GraphEndpoint,
    ) -> Result<(), DecodingGraphError> {
        match endpoint {
            GraphEndpoint::Detection(id) => {
                if !self.nodes.contains_key(
                    &id,
                ) {
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
// Canonical endpoint ordering
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

/// Errors produced by graph construction and validation.
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
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyCoordinate => {
                formatter.write_str(
                    "spatial coordinate cannot be empty",
                )
            }

            Self::TooManyDimensions {
                dimensions,
                limit,
            } => {
                write!(
                    formatter,
                    "coordinate has {dimensions} dimensions; maximum is {limit}"
                )
            }

            Self::CoordinateOutOfRange {
                value,
            } => {
                write!(
                    formatter,
                    "coordinate component {value} is outside the supported range"
                )
            }

            Self::DimensionMismatch => {
                formatter.write_str(
                    "coordinate dimensions do not match",
                )
            }

            Self::ArithmeticOverflow => {
                formatter.write_str(
                    "arithmetic overflow in decoding graph",
                )
            }

            Self::InvalidRound {
                round,
            } => {
                write!(
                    formatter,
                    "invalid graph measurement round: {round}"
                )
            }

            Self::InvalidProbability => {
                formatter.write_str(
                    "probability must be in the range (0, 1]",
                )
            }

            Self::WeightOutOfRange {
                value,
            } => {
                write!(
                    formatter,
                    "edge weight {value} exceeds the supported maximum"
                )
            }

            Self::SelfLoop => {
                formatter.write_str(
                    "self-loop edges are not permitted",
                )
            }

            Self::BoundaryEdgeRequiresBoundary => {
                formatter.write_str(
                    "a Boundary edge must touch a boundary endpoint",
                )
            }

            Self::DuplicateDetectionNode => {
                formatter.write_str(
                    "duplicate detection node",
                )
            }

            Self::DuplicateBoundary => {
                formatter.write_str(
                    "duplicate boundary",
                )
            }

            Self::DuplicateEdge => {
                formatter.write_str(
                    "duplicate graph edge",
                )
            }

            Self::InactiveDetectionEvent => {
                formatter.write_str(
                    "inactive detection event cannot become a graph node",
                )
            }

            Self::IdentifierOverflow => {
                formatter.write_str(
                    "graph identifier overflow",
                )
            }

            Self::UnknownNode {
                id,
            } => {
                write!(
                    formatter,
                    "unknown graph node {id}"
                )
            }

            Self::UnknownBoundary {
                id,
            } => {
                write!(
                    formatter,
                    "unknown graph boundary {id}"
                )
            }

            Self::IndexMismatch => {
                formatter.write_str(
                    "graph index does not match graph storage",
                )
            }

            Self::NodeIdMismatch => {
                formatter.write_str(
                    "graph node identifier mismatch",
                )
            }

            Self::BoundaryIdMismatch => {
                formatter.write_str(
                    "graph boundary identifier mismatch",
                )
            }

            Self::Limit(error) => {
                write!(
                    formatter,
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
        limits.max_memory_bytes =
            64 * 1024;

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
        SpatialCoordinate::xy(
            x,
            y,
        )
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
            coordinate(
                0,
                0,
            );

        let b =
            coordinate(
                3,
                4,
            );

        assert_eq!(
            a.manhattan_distance(
                &b,
            )
            .expect(
                "distance should succeed",
            ),
            7
        );
    }

    #[test]
    fn coordinate_overflow_is_rejected() {
        let result =
            SpatialCoordinate::new(
                vec![
                    i64::MIN,
                ],
            );

        assert!(matches!(
            result,
            Err(
                DecodingGraphError::
                    CoordinateOutOfRange {
                        ..
                    }
            )
        ));
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
                coordinate(
                    1,
                    2,
                ),
                round(0),
            )
            .unwrap();

        graph
            .add_detection_node(
                coordinate.clone(),
                StabilizerId::new(
                    0,
                ),
                MeasurementConfidence::FULL,
            )
            .unwrap();

        let result =
            graph.add_detection_node(
                coordinate,
                StabilizerId::new(
                    0,
                ),
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
                        coordinate(
                            0,
                            0,
                        ),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(
                        0,
                    ),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let b =
            graph
                .add_detection_node(
                    SpaceTimeCoordinate::new(
                        coordinate(
                            1,
                            0,
                        ),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(
                        1,
                    ),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let first =
            GraphEndpoint::Detection(
                a,
            );

        let second =
            GraphEndpoint::Detection(
                b,
            );

        graph
            .add_edge(
                first,
                second,
                EdgeWeight::new(
                    1,
                )
                .unwrap(),
                EdgeKind::Spatial,
            )
            .unwrap();

        let result =
            graph.add_edge(
                second,
                first,
                EdgeWeight::new(
                    1,
                )
                .unwrap(),
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
                        coordinate(
                            0,
                            0,
                        ),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(
                        0,
                    ),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let boundary =
            graph
                .add_boundary(
                    SpaceTimeCoordinate::new(
                        coordinate(
                            0,
                            -1,
                        ),
                        round(0),
                    )
                    .unwrap(),
                )
                .unwrap();

        graph
            .add_edge(
                GraphEndpoint::Detection(
                    node,
                ),
                GraphEndpoint::Boundary(
                    boundary,
                ),
                EdgeWeight::new(
                    5,
                )
                .unwrap(),
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
    fn invalid_boundary_edge_is_rejected() {
        let mut graph =
            DecodingGraph::new_with_limits(
                &limits(),
            )
            .unwrap();

        let a =
            graph
                .add_detection_node(
                    SpaceTimeCoordinate::new(
                        coordinate(
                            0,
                            0,
                        ),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(
                        0,
                    ),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let b =
            graph
                .add_detection_node(
                    SpaceTimeCoordinate::new(
                        coordinate(
                            1,
                            0,
                        ),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(
                        1,
                    ),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let result =
            graph.add_edge(
                GraphEndpoint::Detection(
                    a,
                ),
                GraphEndpoint::Detection(
                    b,
                ),
                EdgeWeight::new(
                    1,
                )
                .unwrap(),
                EdgeKind::Boundary,
            );

        assert_eq!(
            result,
            Err(
                DecodingGraphError::
                    BoundaryEdgeRequiresBoundary
            )
        );
    }

    #[test]
    fn resource_preflight_happens_before_allocation() {
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
                    LimitError::Exceeded {
                        resource:
                            LimitKind::GraphNodes,
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
                        coordinate(
                            5,
                            5,
                        ),
                        round(1),
                    )
                    .unwrap(),
                    StabilizerId::new(
                        5,
                    ),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let second =
            graph
                .add_detection_node(
                    SpaceTimeCoordinate::new(
                        coordinate(
                            1,
                            1,
                        ),
                        round(0),
                    )
                    .unwrap(),
                    StabilizerId::new(
                        1,
                    ),
                    MeasurementConfidence::FULL,
                )
                .unwrap();

        let ids: Vec<NodeId> =
            graph
                .nodes()
                .map(
                    DetectionNode::id,
                )
                .collect();

        assert_eq!(
            ids,
            vec![
                first,
                second
            ]
        );
    }

    #[test]
    fn probability_weight_is_deterministic() {
        let weight =
            EdgeWeight::from_probability(
                PROBABILITY_SCALE,
            )
            .expect(
                "probability 1.0 must be valid",
            );

        assert_eq!(
            weight,
            EdgeWeight::ZERO
        );
    }

    #[test]
    fn invalid_probability_is_rejected() {
        assert!(matches!(
            EdgeWeight::from_probability(
                0
            ),
            Err(
                DecodingGraphError::
                    InvalidProbability
            )
        ));

        assert!(matches!(
            EdgeWeight::from_probability(
                PROBABILITY_SCALE
                    .saturating_add(
                        1,
                    )
            ),
            Err(
                DecodingGraphError::
                    InvalidProbability
            )
        ));
    }

    #[test]
    fn graph_rejects_unknown_endpoints() {
        let mut graph =
            DecodingGraph::new_with_limits(
                &limits(),
            )
            .unwrap();

        let result =
            graph.add_edge(
                GraphEndpoint::Detection(
                    NodeId::new(
                        999,
                    ),
                ),
                GraphEndpoint::Detection(
                    NodeId::new(
                        1000,
                    ),
                ),
                EdgeWeight::new(
                    1,
                )
                .unwrap(),
                EdgeKind::Spatial,
            );

        assert!(matches!(
            result,
            Err(
                DecodingGraphError::
                    UnknownNode {
                        ..
                    }
            )
        ));
    }

    #[test]
    fn graph_validation_checks_indices() {
        let graph =
            DecodingGraph::new_with_limits(
                &limits(),
            )
            .unwrap();

        assert!(
            graph.validate().is_ok()
        );
    }
}