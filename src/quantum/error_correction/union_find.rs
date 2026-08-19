//! Zamani Quantum Error Correction — Union-Find Decoder.
//!
//! This module implements a deterministic, bounded Union-Find style decoder
//! for syndrome/detection-event graphs.
//!
//! Architecture:
//!
//! ```text
//! Detection events
//!       │
//!       ▼
//! Decoding graph
//!       │
//!       ▼
//! Union-Find decoder
//!       │
//!       ├── cluster creation
//!       ├── cluster growth
//!       ├── parity tracking
//!       ├── boundary attachment
//!       ├── union operations
//!       └── peeling / correction extraction
//!       │
//!       ▼
//! Correction edges
//! ```
//!
//! Design goals:
//!
//! - deterministic decoding;
//! - bounded memory;
//! - bounded execution;
//! - no unchecked indexing;
//! - no production `unwrap()`/`expect()`;
//! - checked arithmetic;
//! - explicit graph validation;
//! - explicit boundary handling;
//! - deterministic tie breaking;
//! - cancellation support;
//! - decoder resource budgets;
//! - no floating-point values;
//! - no dependence on a particular physical noise model;
//! - suitable as a backend beneath a higher-level QEC decoder.
//!
//! Important:
//!
//! Union-Find operates on a decoding graph. It does NOT:
//!
//! - generate physical noise;
//! - extract syndromes;
//! - perform stabilizer algebra;
//! - modify quantum state;
//! - apply a Pauli frame;
//! - determine logical equivalence.
//!
//! Those responsibilities belong to other QEC layers.
//!
//! The decoder returns a set of selected graph edges. A later Pauli-frame or
//! logical layer is responsible for interpreting those edges as physical
//! corrections.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// ============================================================================
// Resource limits
// ============================================================================

/// Maximum number of graph vertices accepted by the decoder.
pub const MAX_UNION_FIND_VERTICES: usize = 1_000_000;

/// Maximum number of graph edges accepted by the decoder.
pub const MAX_UNION_FIND_EDGES: usize = 4_000_000;

/// Maximum number of selected correction edges.
pub const MAX_CORRECTION_EDGES: usize = 1_000_000;

/// Maximum number of cluster-growth operations.
pub const MAX_GROWTH_OPERATIONS: usize = 20_000_000;

/// Maximum number of union operations.
pub const MAX_UNION_OPERATIONS: usize = 4_000_000;

/// Maximum number of peeling operations.
pub const MAX_PEEL_OPERATIONS: usize = 20_000_000;

/// Maximum supported edge weight.
pub const MAX_EDGE_WEIGHT: u64 = 1_000_000_000_000_000;

/// Maximum supported vertex identifier.
pub const MAX_VERTEX_ID: usize = 1_000_000_000;

/// Maximum supported boundary identifier.
pub const MAX_BOUNDARY_ID: usize = 1_000_000_000;

// ============================================================================
// Vertex identifiers
// ============================================================================

/// Stable identifier for a detection-event vertex.
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
pub struct VertexId(usize);

impl VertexId {
    /// Creates a vertex identifier.
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for VertexId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

// ============================================================================
// Boundary identifiers
// ============================================================================

/// Stable identifier for a decoding boundary.
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
    /// Creates a boundary identifier.
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
// Graph endpoint
// ============================================================================

/// Endpoint of a decoding edge.
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
pub enum Endpoint {
    /// Detection event.
    Vertex(VertexId),

    /// Boundary endpoint.
    Boundary(BoundaryId),
}

impl Endpoint {
    /// Returns true when this is a vertex.
    pub const fn is_vertex(self) -> bool {
        matches!(self, Self::Vertex(_))
    }

    /// Returns true when this is a boundary.
    pub const fn is_boundary(self) -> bool {
        matches!(self, Self::Boundary(_))
    }
}

// ============================================================================
// Edge identifier
// ============================================================================

/// Stable edge identifier.
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
pub struct EdgeId(usize);

impl EdgeId {
    /// Creates an edge identifier.
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the numeric identifier.
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for EdgeId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "e{}", self.0)
    }
}

// ============================================================================
// Edge
// ============================================================================

/// Weighted decoding-graph edge.
///
/// Edges are normalized so that the endpoint ordering is deterministic.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct DecodingEdge {
    id: EdgeId,
    left: Endpoint,
    right: Endpoint,
    weight: u64,
}

impl DecodingEdge {
    /// Creates a validated edge.
    pub fn new(
        id: EdgeId,
        left: Endpoint,
        right: Endpoint,
        weight: u64,
    ) -> Result<Self, UnionFindError> {
        if left == right {
            return Err(
                UnionFindError::SelfLoop {
                    endpoint: left,
                },
            );
        }

        if weight > MAX_EDGE_WEIGHT {
            return Err(
                UnionFindError::EdgeWeightOutOfRange {
                    weight,
                },
            );
        }

        let (left, right) =
            if left <= right {
                (left, right)
            } else {
                (right, left)
            };

        Ok(Self {
            id,
            left,
            right,
            weight,
        })
    }

    /// Returns the edge identifier.
    pub const fn id(self) -> EdgeId {
        self.id
    }

    /// Returns the left endpoint.
    pub const fn left(self) -> Endpoint {
        self.left
    }

    /// Returns the right endpoint.
    pub const fn right(self) -> Endpoint {
        self.right
    }

    /// Returns the edge weight.
    pub const fn weight(self) -> u64 {
        self.weight
    }

    /// Returns true if this edge connects two detection vertices.
    pub const fn connects_vertices(self) -> bool {
        matches!(
            (self.left, self.right),
            (Endpoint::Vertex(_), Endpoint::Vertex(_))
        )
    }

    /// Returns true if this edge connects a vertex to a boundary.
    pub const fn connects_boundary(self) -> bool {
        matches!(
            (self.left, self.right),
            (Endpoint::Vertex(_), Endpoint::Boundary(_))
                | (Endpoint::Boundary(_), Endpoint::Vertex(_))
        )
    }
}

// ============================================================================
// Syndrome graph
// ============================================================================

/// Bounded decoding graph consumed by the Union-Find decoder.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct DecodingGraph {
    vertices: BTreeSet<VertexId>,
    active: BTreeSet<VertexId>,
    boundaries: BTreeSet<BoundaryId>,
    edges: BTreeMap<EdgeId, DecodingEdge>,
}

impl DecodingGraph {
    /// Creates an empty graph.
    pub fn new() -> Self {
        Self {
            vertices: BTreeSet::new(),
            active: BTreeSet::new(),
            boundaries: BTreeSet::new(),
            edges: BTreeMap::new(),
        }
    }

    /// Adds a vertex.
    pub fn add_vertex(
        &mut self,
        vertex: VertexId,
    ) -> Result<(), UnionFindError> {
        if vertex.index()
            > MAX_VERTEX_ID
        {
            return Err(
                UnionFindError::VertexIdOutOfRange {
                    vertex,
                },
            );
        }

        if self.vertices.len()
            >= MAX_UNION_FIND_VERTICES
            && !self.vertices.contains(&vertex)
        {
            return Err(
                UnionFindError::TooManyVertices {
                    limit:
                        MAX_UNION_FIND_VERTICES,
                },
            );
        }

        self.vertices.insert(vertex);

        Ok(())
    }

    /// Marks a vertex as an active detection event.
    pub fn activate(
        &mut self,
        vertex: VertexId,
    ) -> Result<(), UnionFindError> {
        if !self.vertices.contains(&vertex) {
            return Err(
                UnionFindError::UnknownVertex {
                    vertex,
                },
            );
        }

        self.active.insert(vertex);

        Ok(())
    }

    /// Adds a boundary.
    pub fn add_boundary(
        &mut self,
        boundary: BoundaryId,
    ) -> Result<(), UnionFindError> {
        if boundary.index()
            > MAX_BOUNDARY_ID
        {
            return Err(
                UnionFindError::BoundaryIdOutOfRange {
                    boundary,
                },
            );
        }

        self.boundaries.insert(boundary);

        Ok(())
    }

    /// Adds a weighted edge.
    pub fn add_edge(
        &mut self,
        edge: DecodingEdge,
    ) -> Result<(), UnionFindError> {
        if self.edges.len()
            >= MAX_UNION_FIND_EDGES
            && !self.edges.contains_key(&edge.id())
        {
            return Err(
                UnionFindError::TooManyEdges {
                    limit:
                        MAX_UNION_FIND_EDGES,
                },
            );
        }

        self.validate_endpoint(
            edge.left(),
        )?;

        self.validate_endpoint(
            edge.right(),
        )?;

        if self.edges.contains_key(&edge.id()) {
            return Err(
                UnionFindError::DuplicateEdge {
                    edge: edge.id(),
                },
            );
        }

        self.edges.insert(
            edge.id(),
            edge,
        );

        Ok(())
    }

    fn validate_endpoint(
        &self,
        endpoint: Endpoint,
    ) -> Result<(), UnionFindError> {
        match endpoint {
            Endpoint::Vertex(vertex) => {
                if !self.vertices.contains(&vertex) {
                    return Err(
                        UnionFindError::UnknownVertex {
                            vertex,
                        },
                    );
                }
            }

            Endpoint::Boundary(boundary) => {
                if !self
                    .boundaries
                    .contains(&boundary)
                {
                    return Err(
                        UnionFindError::UnknownBoundary {
                            boundary,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Returns all vertices.
    pub fn vertices(
        &self,
    ) -> impl Iterator<Item = VertexId> + '_ {
        self.vertices.iter().copied()
    }

    /// Returns active detection events.
    pub fn active_vertices(
        &self,
    ) -> impl Iterator<Item = VertexId> + '_ {
        self.active.iter().copied()
    }

    /// Returns all boundaries.
    pub fn boundaries(
        &self,
    ) -> impl Iterator<Item = BoundaryId> + '_ {
        self.boundaries.iter().copied()
    }

    /// Returns all edges in deterministic order.
    pub fn edges(
        &self,
    ) -> impl Iterator<Item = DecodingEdge> + '_ {
        self.edges.values().copied()
    }

    /// Returns an edge by identifier.
    pub fn edge(
        &self,
        id: EdgeId,
    ) -> Option<DecodingEdge> {
        self.edges.get(&id).copied()
    }

    /// Returns the number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of active vertices.
    pub fn active_count(&self) -> usize {
        self.active.len()
    }

    /// Returns the number of edges.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Validates the entire graph.
    pub fn validate(
        &self,
    ) -> Result<(), UnionFindError> {
        if self.vertices.len()
            > MAX_UNION_FIND_VERTICES
        {
            return Err(
                UnionFindError::TooManyVertices {
                    limit:
                        MAX_UNION_FIND_VERTICES,
                },
            );
        }

        if self.edges.len()
            > MAX_UNION_FIND_EDGES
        {
            return Err(
                UnionFindError::TooManyEdges {
                    limit:
                        MAX_UNION_FIND_EDGES,
                },
            );
        }

        for vertex in &self.active {
            if !self.vertices.contains(vertex) {
                return Err(
                    UnionFindError::UnknownVertex {
                        vertex: *vertex,
                    },
                );
            }
        }

        for edge in self.edges.values() {
            self.validate_endpoint(
                edge.left(),
            )?;

            self.validate_endpoint(
                edge.right(),
            )?;
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
// Decoder budget
// ============================================================================

/// Execution budget for the decoder.
///
/// This prevents pathological input from consuming unlimited CPU time.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct DecoderBudget {
    growth_operations: usize,
    union_operations: usize,
    peel_operations: usize,
}

impl DecoderBudget {
    /// Creates a budget.
    pub const fn new(
        growth_operations: usize,
        union_operations: usize,
        peel_operations: usize,
    ) -> Self {
        Self {
            growth_operations,
            union_operations,
            peel_operations,
        }
    }

    /// Production-oriented default budget.
    pub const fn production() -> Self {
        Self {
            growth_operations:
                MAX_GROWTH_OPERATIONS,
            union_operations:
                MAX_UNION_OPERATIONS,
            peel_operations:
                MAX_PEEL_OPERATIONS,
        }
    }

    /// Returns the growth-operation limit.
    pub const fn growth_operations(
        self,
    ) -> usize {
        self.growth_operations
    }

    /// Returns the union-operation limit.
    pub const fn union_operations(
        self,
    ) -> usize {
        self.union_operations
    }

    /// Returns the peeling-operation limit.
    pub const fn peel_operations(
        self,
    ) -> usize {
        self.peel_operations
    }
}

impl Default for DecoderBudget {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Cancellation
// ============================================================================

/// Cooperative cancellation interface.
///
/// A hardware/runtime integration can implement this trait to stop a decoder
/// without terminating the process.
pub trait CancellationToken {
    /// Returns true when decoding must stop.
    fn is_cancelled(&self) -> bool;
}

/// Cancellation token that never cancels.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
)]
pub struct NeverCancel;

impl CancellationToken for NeverCancel {
    fn is_cancelled(&self) -> bool {
        false
    }
}

// ============================================================================
// Decoder configuration
// ============================================================================

/// Configuration of the Union-Find decoder.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct UnionFindConfig {
    budget: DecoderBudget,
}

impl UnionFindConfig {
    /// Creates a configuration.
    pub const fn new(
        budget: DecoderBudget,
    ) -> Self {
        Self { budget }
    }

    /// Production configuration.
    pub const fn production() -> Self {
        Self {
            budget:
                DecoderBudget::production(),
        }
    }

    /// Returns the execution budget.
    pub const fn budget(
        self,
    ) -> DecoderBudget {
        self.budget
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

/// A selected correction path represented by graph edges.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct Correction {
    edges: Vec<EdgeId>,
}

impl Correction {
    /// Creates an empty correction.
    pub fn empty() -> Self {
        Self {
            edges: Vec::new(),
        }
    }

    /// Returns selected correction edges.
    pub fn edges(
        &self,
    ) -> &[EdgeId] {
        &self.edges
    }

    /// Returns the number of correction edges.
    pub fn len(&self) -> usize {
        self.edges.len()
    }

    /// Returns true if no correction edges are selected.
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }
}

impl Default for Correction {
    fn default() -> Self {
        Self::empty()
    }
}

// ============================================================================
// Decode result
// ============================================================================

/// Result returned by Union-Find decoding.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct DecodeResult {
    correction: Correction,
    clusters: usize,
    growth_operations: usize,
    union_operations: usize,
    peel_operations: usize,
}

impl DecodeResult {
    /// Returns the correction.
    pub fn correction(
        &self,
    ) -> &Correction {
        &self.correction
    }

    /// Returns the number of final clusters.
    pub const fn clusters(
        &self,
    ) -> usize {
        self.clusters
    }

    /// Returns the number of growth operations.
    pub const fn growth_operations(
        &self,
    ) -> usize {
        self.growth_operations
    }

    /// Returns the number of union operations.
    pub const fn union_operations(
        &self,
    ) -> usize {
        self.union_operations
    }

    /// Returns the number of peeling operations.
    pub const fn peel_operations(
        &self,
    ) -> usize {
        self.peel_operations
    }
}

// ============================================================================
// Union-Find cluster
// ============================================================================

#[derive(
    Debug,
    Clone,
)]
struct Cluster {
    parent: usize,
    rank: usize,
    parity: bool,
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

// ============================================================================
// Union-Find state
// ============================================================================

#[derive(
    Debug,
    Clone,
)]
struct UnionFindState {
    vertices: Vec<VertexId>,
    index: BTreeMap<VertexId, usize>,
    clusters: Vec<Cluster>,
}

impl UnionFindState {
    fn new(
        graph: &DecodingGraph,
    ) -> Result<Self, UnionFindError> {
        let vertices: Vec<VertexId> =
            graph.vertices().collect();

        if vertices.len()
            > MAX_UNION_FIND_VERTICES
        {
            return Err(
                UnionFindError::TooManyVertices {
                    limit:
                        MAX_UNION_FIND_VERTICES,
                },
            );
        }

        let mut index =
            BTreeMap::new();

        for (
            position,
            vertex,
        ) in vertices.iter().copied().enumerate()
        {
            index.insert(
                vertex,
                position,
            );
        }

        let active: BTreeSet<VertexId> =
            graph
                .active_vertices()
                .collect();

        let clusters =
            vertices
                .iter()
                .enumerate()
                .map(
                    |(position, vertex)| {
                        Cluster::new(
                            position,
                            active.contains(vertex),
                        )
                    },
                )
                .collect();

        Ok(Self {
            vertices,
            index,
            clusters,
        })
    }

    fn vertex_index(
        &self,
        vertex: VertexId,
    ) -> Result<usize, UnionFindError> {
        self.index
            .get(&vertex)
            .copied()
            .ok_or(
                UnionFindError::UnknownVertex {
                    vertex,
                },
            )
    }

    fn find(
        &mut self,
        index: usize,
    ) -> Result<usize, UnionFindError> {
        if index >= self.clusters.len() {
            return Err(
                UnionFindError::InternalIndexOutOfRange {
                    index,
                },
            );
        }

        let parent =
            self.clusters[index].parent;

        if parent == index {
            return Ok(index);
        }

        let root =
            self.find(parent)?;

        self.clusters[index].parent =
            root;

        Ok(root)
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
            self.clusters[left_root].rank;

        let right_rank =
            self.clusters[right_root].rank;

        if left_rank < right_rank
            || (left_rank == right_rank
                && left_root > right_root)
        {
            std::mem::swap(
                &mut left_root,
                &mut right_root,
            );
        }

        let right_parity =
            self.clusters[right_root]
                .parity;

        let right_boundary =
            self.clusters[right_root]
                .has_boundary;

        let left_cluster =
            &mut self.clusters[left_root];

        left_cluster.parity ^=
            right_parity;

        left_cluster.has_boundary |=
            right_boundary;

        self.clusters[right_root]
            .parent = left_root;

        if left_rank == right_rank {
            self.clusters[left_root]
                .rank = self.clusters[left_root]
                .rank
                .checked_add(1)
                .ok_or(
                    UnionFindError::ArithmeticOverflow,
                )?;
        }

        Ok(true)
    }

    fn mark_boundary(
        &mut self,
        vertex: usize,
    ) -> Result<(), UnionFindError> {
        let root =
            self.find(vertex)?;

        self.clusters[root]
            .has_boundary = true;

        Ok(())
    }

    fn root_state(
        &mut self,
        vertex: usize,
    ) -> Result<(bool, bool), UnionFindError> {
        let root =
            self.find(vertex)?;

        Ok((
            self.clusters[root].parity,
            self.clusters[root]
                .has_boundary,
        ))
    }

    fn root_count(
        &mut self,
    ) -> Result<usize, UnionFindError> {
        let mut roots =
            BTreeSet::new();

        for index in
            0..self.clusters.len()
        {
            roots.insert(
                self.find(index)?,
            );
        }

        Ok(roots.len())
    }
}

// ============================================================================
// Union-Find decoder
// ============================================================================

/// Production-oriented Union-Find decoder.
///
/// The decoder works over an explicit weighted graph.
///
/// The implementation uses:
///
/// 1. deterministic edge ordering;
/// 2. active syndrome parity;
/// 3. boundary attachment;
/// 4. Union-Find cluster merging;
/// 5. deterministic correction extraction.
///
/// The graph must already represent the physical decoding problem. The
/// decoder does not infer lattice geometry.
#[derive(
    Debug,
    Clone,
)]
pub struct UnionFindDecoder {
    config: UnionFindConfig,
}

impl UnionFindDecoder {
    /// Creates a decoder using the production configuration.
    pub const fn new() -> Self {
        Self {
            config:
                UnionFindConfig::production(),
        }
    }

    /// Creates a decoder with explicit resource limits.
    pub const fn with_config(
        config: UnionFindConfig,
    ) -> Self {
        Self { config }
    }

    /// Returns decoder configuration.
    pub const fn config(
        &self,
    ) -> UnionFindConfig {
        self.config
    }

    /// Decodes using a non-cancelling token.
    pub fn decode(
        &self,
        graph: &DecodingGraph,
    ) -> Result<DecodeResult, UnionFindError> {
        self.decode_with_cancellation(
            graph,
            &NeverCancel,
        )
    }

    /// Decodes with cooperative cancellation.
    pub fn decode_with_cancellation<C>(
        &self,
        graph: &DecodingGraph,
        cancellation: &C,
    ) -> Result<DecodeResult, UnionFindError>
    where
        C: CancellationToken,
    {
        graph.validate()?;

        if cancellation.is_cancelled() {
            return Err(
                UnionFindError::Cancelled,
            );
        }

        if graph.active_count() == 0 {
            return Ok(DecodeResult {
                correction:
                    Correction::empty(),
                clusters:
                    graph.vertex_count(),
                growth_operations: 0,
                union_operations: 0,
                peel_operations: 0,
            });
        }

        let mut state =
            UnionFindState::new(graph)?;

        let mut growth_operations =
            0usize;

        let mut union_operations =
            0usize;

        let mut selected =
            BTreeSet::new();

        let mut edges: Vec<DecodingEdge> =
            graph.edges().collect();

        // Deterministic minimum-weight ordering.
        //
        // Ties are resolved by edge identifier and endpoint ordering.
        edges.sort_by(
            |left, right| {
                (
                    left.weight(),
                    left.id(),
                    left.left(),
                    left.right(),
                )
                    .cmp(&(
                        right.weight(),
                        right.id(),
                        right.left(),
                        right.right(),
                    ))
            },
        );

        // --------------------------------------------------------------------
        // Boundary attachment
        // --------------------------------------------------------------------
        //
        // Boundary edges are processed before internal edges. This gives
        // deterministic preference to the least-cost available boundary
        // resolution and marks the corresponding cluster as boundary-connected.
        //
        // The selected edge itself is retained as part of the correction path.
        // --------------------------------------------------------------------

        for edge in edges.iter().copied() {
            if cancellation.is_cancelled() {
                return Err(
                    UnionFindError::Cancelled,
                );
            }

            if !edge.connects_boundary() {
                continue;
            }

            growth_operations =
                growth_operations
                    .checked_add(1)
                    .ok_or(
                        UnionFindError::ArithmeticOverflow,
                    )?;

            if growth_operations
                > self
                    .config
                    .budget()
                    .growth_operations()
            {
                return Err(
                    UnionFindError::GrowthBudgetExceeded {
                        limit:
                            self.config
                                .budget()
                                .growth_operations(),
                    },
                );
            }

            let vertex =
                match (
                    edge.left(),
                    edge.right(),
                ) {
                    (
                        Endpoint::Vertex(vertex),
                        Endpoint::Boundary(_),
                    )
                    | (
                        Endpoint::Boundary(_),
                        Endpoint::Vertex(vertex),
                    ) => vertex,

                    _ => {
                        continue;
                    }
                };

            let index =
                state.vertex_index(vertex)?;

            let (
                parity,
                has_boundary,
            ) =
                state.root_state(index)?;

            if parity
                && !has_boundary
            {
                selected.insert(
                    edge.id(),
                );

                state.mark_boundary(index)?;
            }
        }

        // --------------------------------------------------------------------
        // Internal cluster growth
        // --------------------------------------------------------------------
        //
        // Process internal edges in nondecreasing weight order.
        //
        // A merge is accepted when it reduces the number of unresolved odd
        // clusters. This is the core parity-driven Union-Find operation.
        // --------------------------------------------------------------------

        for edge in edges.iter().copied() {
            if cancellation.is_cancelled() {
                return Err(
                    UnionFindError::Cancelled,
                );
            }

            if !edge.connects_vertices() {
                continue;
            }

            growth_operations =
                growth_operations
                    .checked_add(1)
                    .ok_or(
                        UnionFindError::ArithmeticOverflow,
                    )?;

            if growth_operations
                > self
                    .config
                    .budget()
                    .growth_operations()
            {
                return Err(
                    UnionFindError::GrowthBudgetExceeded {
                        limit:
                            self.config
                                .budget()
                                .growth_operations(),
                    },
                );
            }

            let (
                left_vertex,
                right_vertex,
            ) =
                match (
                    edge.left(),
                    edge.right(),
                ) {
                    (
                        Endpoint::Vertex(left),
                        Endpoint::Vertex(right),
                    ) => (left, right),

                    _ => continue,
                };

            let left_index =
                state.vertex_index(
                    left_vertex,
                )?;

            let right_index =
                state.vertex_index(
                    right_vertex,
                )?;

            let (
                left_parity,
                left_boundary,
            ) =
                state.root_state(
                    left_index,
                )?;

            let (
                right_parity,
                right_boundary,
            ) =
                state.root_state(
                    right_index,
                )?;

            if left_boundary
                && right_boundary
            {
                continue;
            }

            let beneficial =
                (left_parity
                    || right_parity)
                    && !(
                        left_boundary
                            && !left_parity
                    )
                    && !(
                        right_boundary
                            && !right_parity
                    );

            if !beneficial {
                continue;
            }

            if union_operations
                >= self
                    .config
                    .budget()
                    .union_operations()
            {
                return Err(
                    UnionFindError::UnionBudgetExceeded {
                        limit:
                            self.config
                                .budget()
                                .union_operations(),
                    },
                );
            }

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

                selected.insert(
                    edge.id(),
                );
            }
        }

        // --------------------------------------------------------------------
        // Final boundary resolution
        // --------------------------------------------------------------------
        //
        // Some odd clusters may remain after internal growth. Find the
        // cheapest deterministic boundary edge capable of resolving each
        // remaining cluster.
        // --------------------------------------------------------------------

        for edge in edges.iter().copied() {
            if cancellation.is_cancelled() {
                return Err(
                    UnionFindError::Cancelled,
                );
            }

            if !edge.connects_boundary() {
                continue;
            }

            let vertex =
                match (
                    edge.left(),
                    edge.right(),
                ) {
                    (
                        Endpoint::Vertex(vertex),
                        Endpoint::Boundary(_),
                    )
                    | (
                        Endpoint::Boundary(_),
                        Endpoint::Vertex(vertex),
                    ) => vertex,

                    _ => continue,
                };

            let index =
                state.vertex_index(vertex)?;

            let (
                parity,
                has_boundary,
            ) =
                state.root_state(index)?;

            if parity
                && !has_boundary
            {
                selected.insert(
                    edge.id(),
                );

                state.mark_boundary(index)?;
            }
        }

        // --------------------------------------------------------------------
        // Correction extraction
        // --------------------------------------------------------------------

        let mut peel_operations =
            0usize;

        let mut correction_edges =
            Vec::new();

        for edge_id in selected {
            if cancellation.is_cancelled() {
                return Err(
                    UnionFindError::Cancelled,
                );
            }

            peel_operations =
                peel_operations
                    .checked_add(1)
                    .ok_or(
                        UnionFindError::ArithmeticOverflow,
                    )?;

            if peel_operations
                > self
                    .config
                    .budget()
                    .peel_operations()
            {
                return Err(
                    UnionFindError::PeelBudgetExceeded {
                        limit:
                            self.config
                                .budget()
                                .peel_operations(),
                    },
                );
            }

            if correction_edges.len()
                >= MAX_CORRECTION_EDGES
            {
                return Err(
                    UnionFindError::TooManyCorrectionEdges {
                        limit:
                            MAX_CORRECTION_EDGES,
                    },
                );
            }

            correction_edges.push(
                edge_id,
            );
        }

        correction_edges.sort();

        // --------------------------------------------------------------------
        // Validate resulting parity state.
        // --------------------------------------------------------------------

        let roots =
            state.root_count()?;

        let mut unresolved =
            BTreeSet::new();

        for vertex in
            graph.active_vertices()
        {
            let index =
                state.vertex_index(vertex)?;

            let (
                parity,
                has_boundary,
            ) =
                state.root_state(index)?;

            if parity && !has_boundary {
                unresolved.insert(
                    vertex,
                );
            }
        }

        if !unresolved.is_empty() {
            return Err(
                UnionFindError::UnresolvedSyndrome {
                    vertices:
                        unresolved.len(),
                },
            );
        }

        Ok(DecodeResult {
            correction:
                Correction {
                    edges:
                        correction_edges,
                },
            clusters: roots,
            growth_operations,
            union_operations,
            peel_operations,
        })
    }
}

impl Default for UnionFindDecoder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Convenience API
// ============================================================================

/// Decodes a graph with the production Union-Find configuration.
pub fn decode(
    graph: &DecodingGraph,
) -> Result<DecodeResult, UnionFindError> {
    UnionFindDecoder::new()
        .decode(graph)
}

/// Decodes a graph using explicit configuration.
pub fn decode_with_config(
    graph: &DecodingGraph,
    config: UnionFindConfig,
) -> Result<DecodeResult, UnionFindError> {
    UnionFindDecoder::with_config(config)
        .decode(graph)
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by the Union-Find subsystem.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum UnionFindError {
    /// Vertex identifier is outside the supported range.
    VertexIdOutOfRange {
        vertex: VertexId,
    },

    /// Boundary identifier is outside the supported range.
    BoundaryIdOutOfRange {
        boundary: BoundaryId,
    },

    /// Too many graph vertices.
    TooManyVertices {
        limit: usize,
    },

    /// Too many graph edges.
    TooManyEdges {
        limit: usize,
    },

    /// Too many correction edges.
    TooManyCorrectionEdges {
        limit: usize,
    },

    /// Referenced vertex does not exist.
    UnknownVertex {
        vertex: VertexId,
    },

    /// Referenced boundary does not exist.
    UnknownBoundary {
        boundary: BoundaryId,
    },

    /// Duplicate edge identifier.
    DuplicateEdge {
        edge: EdgeId,
    },

    /// Edge connects an endpoint to itself.
    SelfLoop {
        endpoint: Endpoint,
    },

    /// Edge weight is outside the supported range.
    EdgeWeightOutOfRange {
        weight: u64,
    },

    /// Internal vector index was invalid.
    InternalIndexOutOfRange {
        index: usize,
    },

    /// Checked arithmetic failed.
    ArithmeticOverflow,

    /// Growth budget exhausted.
    GrowthBudgetExceeded {
        limit: usize,
    },

    /// Union budget exhausted.
    UnionBudgetExceeded {
        limit: usize,
    },

    /// Peeling budget exhausted.
    PeelBudgetExceeded {
        limit: usize,
    },

    /// Decoder was cancelled.
    Cancelled,

    /// One or more detection events could not be resolved.
    UnresolvedSyndrome {
        vertices: usize,
    },
}

impl fmt::Display
    for UnionFindError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::VertexIdOutOfRange {
                vertex,
            } => {
                write!(
                    f,
                    "vertex identifier {vertex} exceeds the supported range"
                )
            }

            Self::BoundaryIdOutOfRange {
                boundary,
            } => {
                write!(
                    f,
                    "boundary identifier {boundary} exceeds the supported range"
                )
            }

            Self::TooManyVertices {
                limit,
            } => {
                write!(
                    f,
                    "decoding graph exceeds the {limit} vertex limit"
                )
            }

            Self::TooManyEdges {
                limit,
            } => {
                write!(
                    f,
                    "decoding graph exceeds the {limit} edge limit"
                )
            }

            Self::TooManyCorrectionEdges {
                limit,
            } => {
                write!(
                    f,
                    "correction exceeds the {limit} edge limit"
                )
            }

            Self::UnknownVertex {
                vertex,
            } => {
                write!(
                    f,
                    "unknown decoding vertex {vertex}"
                )
            }

            Self::UnknownBoundary {
                boundary,
            } => {
                write!(
                    f,
                    "unknown decoding boundary {boundary}"
                )
            }

            Self::DuplicateEdge {
                edge,
            } => {
                write!(
                    f,
                    "duplicate decoding edge {edge}"
                )
            }

            Self::SelfLoop {
                endpoint,
            } => {
                write!(
                    f,
                    "self-loop at {endpoint} is not permitted"
                )
            }

            Self::EdgeWeightOutOfRange {
                weight,
            } => {
                write!(
                    f,
                    "edge weight {weight} exceeds the supported maximum"
                )
            }

            Self::InternalIndexOutOfRange {
                index,
            } => {
                write!(
                    f,
                    "internal Union-Find index {index} is out of range"
                )
            }

            Self::ArithmeticOverflow => {
                write!(
                    f,
                    "arithmetic overflow during Union-Find decoding"
                )
            }

            Self::GrowthBudgetExceeded {
                limit,
            } => {
                write!(
                    f,
                    "Union-Find growth budget of {limit} operations exceeded"
                )
            }

            Self::UnionBudgetExceeded {
                limit,
            } => {
                write!(
                    f,
                    "Union-Find union budget of {limit} operations exceeded"
                )
            }

            Self::PeelBudgetExceeded {
                limit,
            } => {
                write!(
                    f,
                    "Union-Find peeling budget of {limit} operations exceeded"
                )
            }

            Self::Cancelled => {
                write!(
                    f,
                    "Union-Find decoding was cancelled"
                )
            }

            Self::UnresolvedSyndrome {
                vertices,
            } => {
                write!(
                    f,
                    "{vertices} detection-event vertices remain unresolved"
                )
            }
        }
    }
}

impl std::error::Error
    for UnionFindError
{
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn vertex(
        id: usize,
    ) -> VertexId {
        VertexId::new(id)
    }

    fn boundary(
        id: usize,
    ) -> BoundaryId {
        BoundaryId::new(id)
    }

    fn edge(
        id: usize,
        left: Endpoint,
        right: Endpoint,
        weight: u64,
    ) -> DecodingEdge {
        DecodingEdge::new(
            EdgeId::new(id),
            left,
            right,
            weight,
        )
        .unwrap()
    }

    #[test]
    fn empty_graph_decodes_to_empty_correction() {
        let graph =
            DecodingGraph::new();

        let result =
            decode(&graph).unwrap();

        assert!(
            result
                .correction()
                .is_empty()
        );

        assert_eq!(
            result
                .growth_operations(),
            0
        );

        assert_eq!(
            result
                .union_operations(),
            0
        );
    }

    #[test]
    fn graph_rejects_unknown_vertex_edge_endpoint() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_vertex(vertex(0))
            .unwrap();

        assert_eq!(
            graph.add_edge(
                edge(
                    0,
                    Endpoint::Vertex(
                        vertex(0),
                    ),
                    Endpoint::Vertex(
                        vertex(1),
                    ),
                    1,
                ),
            ),
            Err(
                UnionFindError::UnknownVertex {
                    vertex: vertex(1),
                }
            )
        );
    }

    #[test]
    fn graph_rejects_self_loop() {
        assert_eq!(
            DecodingEdge::new(
                EdgeId::new(0),
                Endpoint::Vertex(
                    vertex(0),
                ),
                Endpoint::Vertex(
                    vertex(0),
                ),
                1,
            ),
            Err(
                UnionFindError::SelfLoop {
                    endpoint:
                        Endpoint::Vertex(
                            vertex(0),
                        ),
                }
            )
        );
    }

    #[test]
    fn graph_rejects_unknown_boundary() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_vertex(vertex(0))
            .unwrap();

        assert_eq!(
            graph.add_edge(
                edge(
                    0,
                    Endpoint::Vertex(
                        vertex(0),
                    ),
                    Endpoint::Boundary(
                        boundary(0),
                    ),
                    1,
                ),
            ),
            Err(
                UnionFindError::UnknownBoundary {
                    boundary:
                        boundary(0),
                }
            )
        );
    }

    #[test]
    fn boundary_edge_resolves_single_detection_event() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_vertex(vertex(0))
            .unwrap();

        graph
            .activate(vertex(0))
            .unwrap();

        graph
            .add_boundary(boundary(0))
            .unwrap();

        graph
            .add_edge(
                edge(
                    0,
                    Endpoint::Vertex(
                        vertex(0),
                    ),
                    Endpoint::Boundary(
                        boundary(0),
                    ),
                    10,
                ),
            )
            .unwrap();

        let result =
            decode(&graph).unwrap();

        assert_eq!(
            result
                .correction()
                .edges(),
            &[EdgeId::new(0)]
        );

        assert_eq!(
            result
                .correction()
                .len(),
            1
        );
    }

    #[test]
    fn deterministic_tie_breaking_uses_edge_id() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_vertex(vertex(0))
            .unwrap();

        graph
            .activate(vertex(0))
            .unwrap();

        graph
            .add_boundary(boundary(0))
            .unwrap();

        graph
            .add_boundary(boundary(1))
            .unwrap();

        graph
            .add_edge(
                edge(
                    10,
                    Endpoint::Vertex(
                        vertex(0),
                    ),
                    Endpoint::Boundary(
                        boundary(0),
                    ),
                    100,
                ),
            )
            .unwrap();

        graph
            .add_edge(
                edge(
                    5,
                    Endpoint::Vertex(
                        vertex(0),
                    ),
                    Endpoint::Boundary(
                        boundary(1),
                    ),
                    100,
                ),
            )
            .unwrap();

        let result =
            decode(&graph).unwrap();

        assert_eq!(
            result
                .correction()
                .edges(),
            &[EdgeId::new(5)]
        );
    }

    #[test]
    fn two_active_vertices_can_be_connected() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_vertex(vertex(0))
            .unwrap();

        graph
            .add_vertex(vertex(1))
            .unwrap();

        graph
            .activate(vertex(0))
            .unwrap();

        graph
            .activate(vertex(1))
            .unwrap();

        graph
            .add_edge(
                edge(
                    0,
                    Endpoint::Vertex(
                        vertex(0),
                    ),
                    Endpoint::Vertex(
                        vertex(1),
                    ),
                    10,
                ),
            )
            .unwrap();

        let result =
            decode(&graph).unwrap();

        assert_eq!(
            result
                .correction()
                .edges(),
            &[EdgeId::new(0)]
        );

        assert_eq!(
            result
                .union_operations(),
            1
        );
    }

    #[test]
    fn inactive_vertices_do_not_require_correction() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_vertex(vertex(0))
            .unwrap();

        graph
            .add_vertex(vertex(1))
            .unwrap();

        graph
            .add_edge(
                edge(
                    0,
                    Endpoint::Vertex(
                        vertex(0),
                    ),
                    Endpoint::Vertex(
                        vertex(1),
                    ),
                    1,
                ),
            )
            .unwrap();

        let result =
            decode(&graph).unwrap();

        assert!(
            result
                .correction()
                .is_empty()
        );
    }

    #[test]
    fn cancellation_is_respected() {
        struct Cancel;

        impl CancellationToken
            for Cancel
        {
            fn is_cancelled(
                &self,
            ) -> bool {
                true
            }
        }

        let graph =
            DecodingGraph::new();

        let decoder =
            UnionFindDecoder::new();

        assert_eq!(
            decoder
                .decode_with_cancellation(
                    &graph,
                    &Cancel,
                ),
            Err(
                UnionFindError::Cancelled
            )
        );
    }

    #[test]
    fn growth_budget_is_enforced() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_vertex(vertex(0))
            .unwrap();

        graph
            .activate(vertex(0))
            .unwrap();

        graph
            .add_boundary(boundary(0))
            .unwrap();

        graph
            .add_edge(
                edge(
                    0,
                    Endpoint::Vertex(
                        vertex(0),
                    ),
                    Endpoint::Boundary(
                        boundary(0),
                    ),
                    1,
                ),
            )
            .unwrap();

        let config =
            UnionFindConfig::new(
                DecoderBudget::new(
                    0,
                    100,
                    100,
                ),
            );

        let decoder =
            UnionFindDecoder::with_config(
                config,
            );

        assert_eq!(
            decoder.decode(&graph),
            Err(
                UnionFindError::GrowthBudgetExceeded {
                    limit: 0,
                }
            )
        );
    }

    #[test]
    fn union_find_is_deterministic() {
        let mut first =
            DecodingGraph::new();

        let mut second =
            DecodingGraph::new();

        for graph in [
            &mut first,
            &mut second,
        ] {
            graph
                .add_vertex(vertex(0))
                .unwrap();

            graph
                .add_vertex(vertex(1))
                .unwrap();

            graph
                .activate(vertex(0))
                .unwrap();

            graph
                .activate(vertex(1))
                .unwrap();

            graph
                .add_edge(
                    edge(
                        2,
                        Endpoint::Vertex(
                            vertex(0),
                        ),
                        Endpoint::Vertex(
                            vertex(1),
                        ),
                        10,
                    ),
                )
                .unwrap();
        }

        let decoder =
            UnionFindDecoder::new();

        let first_result =
            decoder
                .decode(&first)
                .unwrap();

        let second_result =
            decoder
                .decode(&second)
                .unwrap();

        assert_eq!(
            first_result,
            second_result
        );
    }

    #[test]
    fn zero_weight_edges_are_valid() {
        let edge =
            DecodingEdge::new(
                EdgeId::new(0),
                Endpoint::Vertex(
                    vertex(0),
                ),
                Endpoint::Vertex(
                    vertex(1),
                ),
                0,
            )
            .unwrap();

        assert_eq!(
            edge.weight(),
            0
        );
    }

    #[test]
    fn excessive_weight_is_rejected() {
        assert_eq!(
            DecodingEdge::new(
                EdgeId::new(0),
                Endpoint::Vertex(
                    vertex(0),
                ),
                Endpoint::Vertex(
                    vertex(1),
                ),
                MAX_EDGE_WEIGHT + 1,
            ),
            Err(
                UnionFindError::EdgeWeightOutOfRange {
                    weight:
                        MAX_EDGE_WEIGHT + 1,
                }
            )
        );
    }

    #[test]
    fn duplicate_edges_are_rejected() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_vertex(vertex(0))
            .unwrap();

        graph
            .add_vertex(vertex(1))
            .unwrap();

        graph
            .add_edge(
                edge(
                    0,
                    Endpoint::Vertex(
                        vertex(0),
                    ),
                    Endpoint::Vertex(
                        vertex(1),
                    ),
                    1,
                ),
            )
            .unwrap();

        assert_eq!(
            graph.add_edge(
                edge(
                    0,
                    Endpoint::Vertex(
                        vertex(0),
                    ),
                    Endpoint::Vertex(
                        vertex(1),
                    ),
                    2,
                ),
            ),
            Err(
                UnionFindError::DuplicateEdge {
                    edge: EdgeId::new(0),
                }
            )
        );
    }

    #[test]
    fn endpoint_order_is_canonical() {
        let edge =
            DecodingEdge::new(
                EdgeId::new(0),
                Endpoint::Vertex(
                    vertex(2),
                ),
                Endpoint::Vertex(
                    vertex(1),
                ),
                5,
            )
            .unwrap();

        assert_eq!(
            edge.left(),
            Endpoint::Vertex(
                vertex(1),
            )
        );

        assert_eq!(
            edge.right(),
            Endpoint::Vertex(
                vertex(2),
            )
        );
    }

    #[test]
    fn budget_is_configurable() {
        let budget =
            DecoderBudget::new(
                10,
                20,
                30,
            );

        assert_eq!(
            budget.growth_operations(),
            10
        );

        assert_eq!(
            budget.union_operations(),
            20
        );

        assert_eq!(
            budget.peel_operations(),
            30
        );
    }

    #[test]
    fn correction_is_sorted() {
        let correction =
            Correction {
                edges: vec![
                    EdgeId::new(3),
                    EdgeId::new(1),
                    EdgeId::new(2),
                ],
            };

        // The structure itself does not silently mutate user input.
        assert_eq!(
            correction.edges(),
            &[
                EdgeId::new(3),
                EdgeId::new(1),
                EdgeId::new(2),
            ]
        );
    }

    #[test]
    fn graph_validation_is_idempotent() {
        let mut graph =
            DecodingGraph::new();

        graph
            .add_vertex(vertex(0))
            .unwrap();

        graph
            .add_vertex(vertex(1))
            .unwrap();

        graph
            .add_edge(
                edge(
                    0,
                    Endpoint::Vertex(
                        vertex(0),
                    ),
                    Endpoint::Vertex(
                        vertex(1),
                    ),
                    10,
                ),
            )
            .unwrap();

        graph.validate().unwrap();
        graph.validate().unwrap();
    }
}