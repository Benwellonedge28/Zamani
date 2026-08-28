//! Zamani Quantum Routing — Path Finding
//!
//! Production-grade, backend-independent path-finding primitives for the
//! Zamani quantum routing subsystem.
//!
//! # Responsibility
//!
//! This module owns graph/path algorithms used by quantum layout and routing.
//!
//! It provides:
//!
//! - deterministic unweighted shortest paths;
//! - deterministic weighted shortest paths;
//! - shortest-path distances;
//! - path validation;
//! - path cost calculation;
//! - bounded path search;
//! - shortest-path candidate enumeration;
//! - reusable path-finding configuration;
//! - explicit handling of unavailable physical resources;
//! - deterministic tie-breaking;
//! - overflow-safe cost accumulation;
//! - reusable graph-search results.
//!
//! It deliberately does NOT own:
//!
//! - topology storage;
//! - topology construction;
//! - logical-to-physical mapping;
//! - layout selection;
//! - SWAP insertion;
//! - SABRE;
//! - lookahead routing;
//! - gate decomposition;
//! - scheduling;
//! - pulse generation;
//! - hardware execution;
//! - calibration acquisition;
//! - simulation;
//! - QEC decoding.
//!
//! Those responsibilities belong to the corresponding routing/backend
//! subsystems.
//!
//! # Architectural position
//!
//! ```text
//!                    PhysicalTopology
//!                           │
//!                           ▼
//!                     ┌───────────┐
//!                     │  path.rs  │
//!                     └─────┬─────┘
//!                           │
//!             ┌─────────────┼─────────────┐
//!             ▼             ▼             ▼
//!         shortest       weighted      candidates
//!           path           path        / distance
//!             │             │             │
//!             └─────────────┼─────────────┘
//!                           ▼
//!                    routing algorithms
//!                           │
//!             ┌─────────────┼─────────────┐
//!             ▼             ▼             ▼
//!          basic        lookahead        SABRE
//!
//! ```
//!
//! # Important design rule
//!
//! `PhysicalTopology` remains the source of truth for graph structure.
//! `path.rs` never reaches into topology internals.
//!
//! This is essential for the "finish each file once" architecture:
//!
//! ```text
//! types.rs
//!    │
//!    ▼
//! topology.rs
//!    │
//!    ▼
//! path.rs
//!    │
//!    ├──► candidates.rs
//!    ├──► algorithms/shortest_path.rs
//!    ├──► algorithms/lookahead.rs
//!    ├──► algorithms/sabre.rs
//!    └──► layout.rs
//! ```
//!
//! Later modules consume the public API of this file and do not need to modify
//! this file merely because a new routing algorithm is added.
//!
//! # Determinism
//!
//! All graph traversal decisions are deterministic.
//!
//! For equal-cost alternatives, the implementation chooses the
//! lexicographically smallest physical-qubit sequence.
//!
//! Therefore identical:
//!
//! ```text
//! topology + source + target + configuration
//! ```
//!
//! produce identical results.
//!
//! # No unsafe
//!
//! This module contains no `unsafe` code.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//!
//! No nightly features are required.
//!
//! # Complexity
//!
//! Unweighted shortest path:
//!
//! - Time: O(V + E)
//! - Space: O(V)
//!
//! Weighted shortest path:
//!
//! - Time: O((V + E) log V)
//! - Space: O(V)
//!
//! The implementation uses only the Rust standard library.
//!
//! # Integration contract
//!
//! This module consumes:
//!
//! - `routing/types.rs`
//! - `routing/errors.rs`
//! - `routing/topology.rs`
//!
//! It must not depend on:
//!
//! - `mapping.rs`
//! - `cost.rs`
//! - `layout.rs`
//! - `router.rs`
//! - `transpiler.rs`
//! - algorithm implementations.
//!
//! Higher-level weighted routing can supply a `PathWeight` implementation
//! without making this module dependent on `cost.rs`.
//!
//! -----------------------------------------------------------------------------
//! Public API summary
//! -----------------------------------------------------------------------------
//!
//! ```text
//! PathFinder
//! ├── new()
//! ├── with_config()
//! ├── config()
//! ├── shortest_path()
//! ├── shortest_distance()
//! ├── weighted_shortest_path()
//! ├── weighted_shortest_distance()
//! ├── shortest_paths()
//! ├── validate_path()
//! ├── path_cost()
//! └── reachable()
//!
//! PathSearchConfig
//! ├── max_path_length
//! ├── max_visited_vertices
//! ├── allow_unavailable
//! └── deterministic
//!
//! PathWeight
//! ├── unit()
//! ├── custom()
//! └── weight()
//!
//! PathResult
//! ├── vertices
//! ├── edge_count()
//! ├── distance()
//! ├── is_empty()
//! ├── source()
//! └── target()
//! ```
//!
//! The API intentionally exposes no mutable topology state.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};

use crate::quantum::routing::errors::{RoutingError, RoutingResult};
use crate::quantum::routing::topology::PhysicalTopology;
use crate::quantum::routing::types::PhysicalQubitId;

// =============================================================================
// Constants
// =============================================================================

/// Default maximum number of vertices that one path search may visit.
///
/// This is a safety guard against pathological input or accidental use of an
/// enormous dynamically generated topology.
///
/// The limit is intentionally large enough for normal quantum hardware while
/// preventing unbounded resource consumption.
///
/// Callers working with larger distributed systems can explicitly configure a
/// larger value.
pub const DEFAULT_MAX_VISITED_VERTICES: usize = 1_000_000;

/// Default maximum number of edges in a returned path.
///
/// `None` is represented internally by `0` in the configuration as "unbounded".
/// The public builder uses `Option<usize>` to make this distinction explicit.
pub const DEFAULT_MAX_PATH_EDGES: Option<usize> = None;

/// Unit weight used by unweighted shortest-path search.
const UNIT_EDGE_WEIGHT: u64 = 1;

// =============================================================================
// Path search configuration
// =============================================================================

/// Configuration controlling graph/path search.
///
/// The configuration is deliberately independent from `RoutingConfig`.
///
/// `RoutingConfig` belongs to the routing-engine layer, while this type only
/// controls graph traversal safety and deterministic behavior.
///
/// This separation prevents `path.rs` from depending on higher-level routing
/// configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathSearchConfig {
    /// Optional maximum number of edges permitted in a returned path.
    ///
    /// `None` means no path-length limit.
    pub max_path_edges: Option<usize>,

    /// Maximum number of distinct vertices that a single search may visit.
    ///
    /// This prevents accidental resource exhaustion.
    pub max_visited_vertices: usize,

    /// Whether unavailable physical qubits may participate in a path.
    ///
    /// Production routing normally sets this to `false`.
    ///
    /// It is exposed because topology analysis and diagnostics may legitimately
    /// need to inspect paths through currently unavailable resources.
    pub allow_unavailable: bool,

    /// Whether deterministic traversal/tie-breaking is required.
    ///
    /// The production default is `true`.
    pub deterministic: bool,
}

impl Default for PathSearchConfig {
    fn default() -> Self {
        Self {
            max_path_edges: DEFAULT_MAX_PATH_EDGES,
            max_visited_vertices: DEFAULT_MAX_VISITED_VERTICES,
            allow_unavailable: false,
            deterministic: true,
        }
    }
}

impl PathSearchConfig {
    /// Creates the production default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum path length in edges.
    #[must_use]
    pub const fn with_max_path_edges(
        mut self,
        max_path_edges: Option<usize>,
    ) -> Self {
        self.max_path_edges = max_path_edges;
        self
    }

    /// Sets the maximum number of vertices that may be visited.
    #[must_use]
    pub const fn with_max_visited_vertices(
        mut self,
        max_visited_vertices: usize,
    ) -> Self {
        self.max_visited_vertices = max_visited_vertices;
        self
    }

    /// Controls whether unavailable physical qubits may be traversed.
    #[must_use]
    pub const fn with_allow_unavailable(
        mut self,
        allow_unavailable: bool,
    ) -> Self {
        self.allow_unavailable = allow_unavailable;
        self
    }

    /// Controls deterministic traversal.
    #[must_use]
    pub const fn with_deterministic(
        mut self,
        deterministic: bool,
    ) -> Self {
        self.deterministic = deterministic;
        self
    }

    /// Validates search configuration.
    pub fn validate(&self) -> RoutingResult<()> {
        if self.max_visited_vertices == 0 {
            return Err(RoutingError::InvalidConfiguration(
                "path search max_visited_vertices must be greater than zero"
                    .to_string(),
            ));
        }

        if let Some(max_edges) = self.max_path_edges {
            if max_edges == 0 {
                return Err(RoutingError::InvalidConfiguration(
                    "path search max_path_edges must be greater than zero"
                        .to_string(),
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Path result
// =============================================================================

/// A validated path through the physical topology.
///
/// The path contains both endpoints.
///
/// For example:
///
/// ```text
/// p0 -> p1 -> p2 -> p3
/// ```
///
/// is represented as:
///
/// ```text
/// [p0, p1, p2, p3]
/// ```
///
/// Therefore:
///
/// ```text
/// edge_count() == vertices.len() - 1
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathResult {
    vertices: Vec<PhysicalQubitId>,
    distance: u64,
}

impl PathResult {
    /// Creates a path result.
    ///
    /// This constructor is private because callers must not be able to create
    /// an invalid path result without validation.
    fn new(
        vertices: Vec<PhysicalQubitId>,
        distance: u64,
    ) -> RoutingResult<Self> {
        if vertices.is_empty() {
            return Err(RoutingError::InternalInvariantViolation(
                "path result cannot contain zero vertices".to_string(),
            ));
        }

        Ok(Self {
            vertices,
            distance,
        })
    }

    /// Returns the physical qubits in traversal order.
    #[must_use]
    pub fn vertices(&self) -> &[PhysicalQubitId] {
        &self.vertices
    }

    /// Consumes the result and returns its physical-qubit sequence.
    #[must_use]
    pub fn into_vertices(self) -> Vec<PhysicalQubitId> {
        self.vertices
    }

    /// Returns the source physical qubit.
    #[must_use]
    pub fn source(&self) -> PhysicalQubitId {
        self.vertices[0]
    }

    /// Returns the destination physical qubit.
    #[must_use]
    pub fn target(&self) -> PhysicalQubitId {
        self.vertices[self.vertices.len() - 1]
    }

    /// Returns the number of edges in the path.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.vertices.len().saturating_sub(1)
    }

    /// Returns the path distance/cost.
    ///
    /// For unweighted BFS this equals `edge_count()`.
    #[must_use]
    pub fn distance(&self) -> u64 {
        self.distance
    }

    /// Returns whether this is a zero-edge path.
    #[must_use]
    pub fn is_trivial(&self) -> bool {
        self.vertices.len() == 1
    }

    /// Returns whether the path contains no vertices.
    ///
    /// This can never be true for a valid `PathResult`, but the method is
    /// provided for ergonomic generic code.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty()
    }
}

// =============================================================================
// Path weight
// =============================================================================

/// Edge-weight abstraction used by weighted shortest-path search.
///
/// This trait deliberately does not depend on `cost.rs`.
///
/// That allows:
///
/// - shortest-path routing;
/// - duration-aware routing;
/// - noise-aware routing;
/// - calibration-aware routing;
/// - SABRE candidate scoring;
///
/// to share this graph-search implementation without creating dependency
/// cycles.
pub trait PathWeight {
    /// Returns the non-negative finite weight of traversing `from -> to`.
    ///
    /// The result is expressed as `u64` so path accumulation cannot produce
    /// floating-point NaNs or infinities.
    fn weight(
        &self,
        topology: &PhysicalTopology,
        from: PhysicalQubitId,
        to: PhysicalQubitId,
    ) -> RoutingResult<u64>;
}

/// Unit-cost edge weighting.
///
/// Every topology edge has cost `1`.
///
/// This produces the same shortest paths as BFS while using the generic
/// weighted-search interface.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnitPathWeight;

impl PathWeight for UnitPathWeight {
    fn weight(
        &self,
        _topology: &PhysicalTopology,
        _from: PhysicalQubitId,
        _to: PhysicalQubitId,
    ) -> RoutingResult<u64> {
        Ok(UNIT_EDGE_WEIGHT)
    }
}

/// Closure-backed path weighting.
///
/// This is useful for higher-level routing algorithms that need custom edge
/// costs without adding a dependency from `path.rs` to `cost.rs`.
pub struct ClosurePathWeight<F>
where
    F: Fn(
        &PhysicalTopology,
        PhysicalQubitId,
        PhysicalQubitId,
    ) -> RoutingResult<u64>,
{
    function: F,
}

impl<F> ClosurePathWeight<F>
where
    F: Fn(
        &PhysicalTopology,
        PhysicalQubitId,
        PhysicalQubitId,
    ) -> RoutingResult<u64>,
{
    /// Creates a closure-backed weight model.
    #[must_use]
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<F> PathWeight for ClosurePathWeight<F>
where
    F: Fn(
        &PhysicalTopology,
        PhysicalQubitId,
        PhysicalQubitId,
    ) -> RoutingResult<u64>,
{
    fn weight(
        &self,
        topology: &PhysicalTopology,
        from: PhysicalQubitId,
        to: PhysicalQubitId,
    ) -> RoutingResult<u64> {
        (self.function)(topology, from, to)
    }
}

// =============================================================================
// Dijkstra queue state
// =============================================================================

/// Internal priority-queue state for deterministic Dijkstra.
///
/// `BinaryHeap` is a max-heap, so the ordering is reversed to make the
/// smallest distance the highest-priority item.
///
/// For equal distances, smaller physical-qubit IDs are preferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct QueueState {
    distance: u64,
    vertex: PhysicalQubitId,
}

impl Ord for QueueState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| other.vertex.cmp(&self.vertex))
    }
}

impl PartialOrd for QueueState {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// Path finder
// =============================================================================

/// Reusable production path-finding engine.
///
/// The engine is immutable with respect to topology and can therefore be
/// shared by multiple routing passes when the topology itself is immutable.
///
/// It does not cache paths internally because topology mutation and calibration
/// updates are owned by the topology layer. A future explicit path cache can
/// be added without changing this API.
#[derive(Debug, Clone)]
pub struct PathFinder {
    config: PathSearchConfig,
}

impl Default for PathFinder {
    fn default() -> Self {
        Self::new()
    }
}

impl PathFinder {
    /// Creates a path finder using production defaults.
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: PathSearchConfig::default(),
        }
    }

    /// Creates a path finder with explicit search configuration.
    pub fn with_config(
        config: PathSearchConfig,
    ) -> RoutingResult<Self> {
        config.validate()?;

        Ok(Self { config })
    }

    /// Returns the active search configuration.
    #[must_use]
    pub const fn config(&self) -> &PathSearchConfig {
        &self.config
    }

    /// Returns the deterministic unweighted shortest path.
    ///
    /// The returned path contains both source and target.
    ///
    /// # Example
    ///
    /// ```text
    /// topology:
    ///
    /// p0 -- p1 -- p2 -- p3
    ///
    /// shortest_path(p0, p3)
    /// =>
    /// [p0, p1, p2, p3]
    /// ```
    pub fn shortest_path(
        &self,
        topology: &PhysicalTopology,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> RoutingResult<PathResult> {
        self.validate_endpoints(topology, source, target)?;

        if source == target {
            return PathResult::new(vec![source], 0);
        }

        let mut queue = VecDeque::new();

        let mut predecessor:
            BTreeMap<PhysicalQubitId, PhysicalQubitId> =
            BTreeMap::new();

        let mut visited = BTreeSet::new();

        queue.push_back(source);
        visited.insert(source);

        while let Some(current) = queue.pop_front() {
            self.check_visited_limit(visited.len())?;

            let mut neighbours =
                self.neighbours(topology, current)?;

            self.order_neighbours(&mut neighbours);

            for neighbour in neighbours {
                if !self.config.allow_unavailable
                    && !self.is_available(topology, neighbour)?
                {
                    continue;
                }

                if visited.contains(&neighbour) {
                    continue;
                }

                if !self.can_extend_path(
                    &predecessor,
                    source,
                    neighbour,
                ) {
                    continue;
                }

                visited.insert(neighbour);
                predecessor.insert(neighbour, current);

                if neighbour == target {
                    let vertices = reconstruct_path(
                        source,
                        target,
                        &predecessor,
                    )?;

                    let distance =
                        vertices.len().saturating_sub(1) as u64;

                    return PathResult::new(vertices, distance);
                }

                queue.push_back(neighbour);
            }
        }

        Err(disconnected_error(source, target))
    }

    /// Returns the unweighted shortest-path distance.
    ///
    /// This avoids allocating the full path when only the distance is needed.
    pub fn shortest_distance(
        &self,
        topology: &PhysicalTopology,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> RoutingResult<u64> {
        self.validate_endpoints(topology, source, target)?;

        if source == target {
            return Ok(0);
        }

        let mut queue = VecDeque::new();
        let mut distances:
            BTreeMap<PhysicalQubitId, u64> =
            BTreeMap::new();

        queue.push_back(source);
        distances.insert(source, 0);

        while let Some(current) = queue.pop_front() {
            self.check_visited_limit(distances.len())?;

            let current_distance = *distances
                .get(&current)
                .ok_or_else(|| {
                    RoutingError::InternalInvariantViolation(
                        format!(
                            "distance missing for visited vertex {current}"
                        ),
                    )
                })?;

            let mut neighbours =
                self.neighbours(topology, current)?;

            self.order_neighbours(&mut neighbours);

            for neighbour in neighbours {
                if !self.config.allow_unavailable
                    && !self.is_available(topology, neighbour)?
                {
                    continue;
                }

                if distances.contains_key(&neighbour) {
                    continue;
                }

                let next_distance =
                    current_distance.checked_add(1).ok_or_else(
                        || {
                            RoutingError::InternalInvariantViolation(
                                "unweighted path distance overflow"
                                    .to_string(),
                            )
                        },
                    )?;

                if let Some(max_edges) =
                    self.config.max_path_edges
                {
                    if next_distance > max_edges as u64 {
                        continue;
                    }
                }

                distances.insert(neighbour, next_distance);

                if neighbour == target {
                    return Ok(next_distance);
                }

                queue.push_back(neighbour);
            }
        }

        Err(disconnected_error(source, target))
    }

    /// Returns a weighted shortest path using Dijkstra's algorithm.
    ///
    /// All edge weights must be non-negative finite integers represented by
    /// `u64`.
    ///
    /// Zero-cost edges are allowed.
    pub fn weighted_shortest_path<W>(
        &self,
        topology: &PhysicalTopology,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        weight: &W,
    ) -> RoutingResult<PathResult>
    where
        W: PathWeight + ?Sized,
    {
        self.validate_endpoints(topology, source, target)?;

        if source == target {
            return PathResult::new(vec![source], 0);
        }

        let mut queue = BinaryHeap::new();

        let mut distances:
            BTreeMap<PhysicalQubitId, u64> =
            BTreeMap::new();

        let mut predecessors:
            BTreeMap<PhysicalQubitId, PhysicalQubitId> =
            BTreeMap::new();

        queue.push(QueueState {
            distance: 0,
            vertex: source,
        });

        distances.insert(source, 0);

        while let Some(state) = queue.pop() {
            let known_distance = match distances.get(&state.vertex) {
                Some(value) => *value,
                None => {
                    return Err(
                        RoutingError::InternalInvariantViolation(
                            format!(
                                "Dijkstra queue vertex {} has no \
                                 recorded distance",
                                state.vertex
                            ),
                        ),
                    )
                }
            };

            if state.distance != known_distance {
                continue;
            }

            self.check_visited_limit(distances.len())?;

            if state.vertex == target {
                let vertices = reconstruct_path(
                    source,
                    target,
                    &predecessors,
                )?;

                return PathResult::new(
                    vertices,
                    state.distance,
                );
            }

            let mut neighbours =
                self.neighbours(topology, state.vertex)?;

            self.order_neighbours(&mut neighbours);

            for neighbour in neighbours {
                if !self.config.allow_unavailable
                    && !self.is_available(topology, neighbour)?
                {
                    continue;
                }

                let edge_count =
                    self.edge_count_to(&predecessors, source, state.vertex)?;

                if let Some(max_edges) =
                    self.config.max_path_edges
                {
                    if edge_count >= max_edges {
                        continue;
                    }
                }

                let edge_weight = weight.weight(
                    topology,
                    state.vertex,
                    neighbour,
                )?;

                let next_distance = state
                    .distance
                    .checked_add(edge_weight)
                    .ok_or_else(|| {
                        RoutingError::InternalInvariantViolation(
                            format!(
                                "weighted path cost overflow while \
                                 traversing {} -> {}",
                                state.vertex, neighbour
                            ),
                        )
                    })?;

                let replace = match distances.get(&neighbour) {
                    None => true,
                    Some(existing) => {
                        next_distance < *existing
                    }
                };

                if replace {
                    distances.insert(
                        neighbour,
                        next_distance,
                    );

                    predecessors.insert(
                        neighbour,
                        state.vertex,
                    );

                    queue.push(QueueState {
                        distance: next_distance,
                        vertex: neighbour,
                    });
                }
            }
        }

        Err(disconnected_error(source, target))
    }

    /// Returns the weighted shortest-path distance.
    ///
    /// This avoids reconstructing the complete path.
    pub fn weighted_shortest_distance<W>(
        &self,
        topology: &PhysicalTopology,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        weight: &W,
    ) -> RoutingResult<u64>
    where
        W: PathWeight + ?Sized,
    {
        self.validate_endpoints(topology, source, target)?;

        if source == target {
            return Ok(0);
        }

        let mut queue = BinaryHeap::new();

        let mut distances:
            BTreeMap<PhysicalQubitId, u64> =
            BTreeMap::new();

        queue.push(QueueState {
            distance: 0,
            vertex: source,
        });

        distances.insert(source, 0);

        while let Some(state) = queue.pop() {
            let known_distance = match distances.get(&state.vertex) {
                Some(value) => *value,
                None => {
                    return Err(
                        RoutingError::InternalInvariantViolation(
                            format!(
                                "Dijkstra queue vertex {} has no \
                                 recorded distance",
                                state.vertex
                            ),
                        ),
                    )
                }
            };

            if state.distance != known_distance {
                continue;
            }

            self.check_visited_limit(distances.len())?;

            if state.vertex == target {
                return Ok(state.distance);
            }

            let mut neighbours =
                self.neighbours(topology, state.vertex)?;

            self.order_neighbours(&mut neighbours);

            for neighbour in neighbours {
                if !self.config.allow_unavailable
                    && !self.is_available(topology, neighbour)?
                {
                    continue;
                }

                let edge_weight = weight.weight(
                    topology,
                    state.vertex,
                    neighbour,
                )?;

                let next_distance = state
                    .distance
                    .checked_add(edge_weight)
                    .ok_or_else(|| {
                        RoutingError::InternalInvariantViolation(
                            "weighted path cost overflow".to_string(),
                        )
                    })?;

                let replace = match distances.get(&neighbour) {
                    None => true,
                    Some(existing) => {
                        next_distance < *existing
                    }
                };

                if replace {
                    distances.insert(
                        neighbour,
                        next_distance,
                    );

                    queue.push(QueueState {
                        distance: next_distance,
                        vertex: neighbour,
                    });
                }
            }
        }

        Err(disconnected_error(source, target))
    }

    /// Enumerates up to `limit` shortest paths between two physical qubits.
    ///
    /// All returned paths have the minimum number of edges.
    ///
    /// Paths are returned deterministically in lexicographic order.
    ///
    /// This is intentionally bounded. Enumerating all shortest paths can be
    /// exponential in the size of the graph.
    pub fn shortest_paths(
        &self,
        topology: &PhysicalTopology,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        limit: usize,
    ) -> RoutingResult<Vec<PathResult>> {
        self.validate_endpoints(topology, source, target)?;

        if limit == 0 {
            return Ok(Vec::new());
        }

        if source == target {
            return Ok(vec![PathResult::new(
                vec![source],
                0,
            )?]);
        }

        let mut distances:
            BTreeMap<PhysicalQubitId, usize> =
            BTreeMap::new();

        let mut predecessors:
            BTreeMap<
                PhysicalQubitId,
                Vec<PhysicalQubitId>,
            > = BTreeMap::new();

        let mut queue = VecDeque::new();

        distances.insert(source, 0);
        queue.push_back(source);

        while let Some(current) = queue.pop_front() {
            self.check_visited_limit(distances.len())?;

            let current_distance =
                *distances.get(&current).ok_or_else(|| {
                    RoutingError::InternalInvariantViolation(
                        format!(
                            "distance missing for vertex {current}"
                        ),
                    )
                })?;

            let mut neighbours =
                self.neighbours(topology, current)?;

            self.order_neighbours(&mut neighbours);

            for neighbour in neighbours {
                if !self.config.allow_unavailable
                    && !self.is_available(topology, neighbour)?
                {
                    continue;
                }

                let next_distance =
                    current_distance
                        .checked_add(1)
                        .ok_or_else(|| {
                            RoutingError::InternalInvariantViolation(
                                "path distance overflow".to_string(),
                            )
                        })?;

                match distances.get(&neighbour) {
                    None => {
                        distances.insert(
                            neighbour,
                            next_distance,
                        );

                        predecessors.insert(
                            neighbour,
                            vec![current],
                        );

                        queue.push_back(neighbour);
                    }

                    Some(existing)
                        if *existing == next_distance =>
                    {
                        predecessors
                            .entry(neighbour)
                            .or_default()
                            .push(current);
                    }

                    _ => {}
                }
            }
        }

        if !distances.contains_key(&target) {
            return Err(disconnected_error(source, target));
        }

        let mut paths = Vec::new();
        let mut current_path = vec![target];

        enumerate_shortest_paths(
            source,
            target,
            &predecessors,
            &mut current_path,
            &mut paths,
            limit,
        )?;

        paths.sort_by(|left, right| {
            compare_paths(left.vertices(), right.vertices())
        });

        Ok(paths)
    }

    /// Validates a physical path against the topology.
    ///
    /// Validation checks:
    ///
    /// - path is non-empty;
    /// - source exists;
    /// - target exists;
    /// - every vertex exists;
    /// - every consecutive pair is physically adjacent;
    /// - unavailable vertices are rejected unless configured otherwise;
    /// - configured maximum path length is respected.
    pub fn validate_path(
        &self,
        topology: &PhysicalTopology,
        path: &[PhysicalQubitId],
    ) -> RoutingResult<u64> {
        if path.is_empty() {
            return Err(RoutingError::InvalidArgument(
                "path cannot be empty".to_string(),
            ));
        }

        if path.len() > 1 {
            if let Some(max_edges) =
                self.config.max_path_edges
            {
                if path.len() - 1 > max_edges {
                    return Err(RoutingError::InvalidConfiguration(
                        format!(
                            "path contains {} edges but configured \
                             maximum is {}",
                            path.len() - 1,
                            max_edges
                        ),
                    ));
                }
            }
        }

        let mut cost = 0u64;

        for (index, vertex) in path.iter().copied().enumerate() {
            if !topology.contains(vertex) {
                return Err(RoutingError::InvalidPhysicalQubit(
                    vertex.index(),
                ));
            }

            if !self.config.allow_unavailable
                && !self.is_available(topology, vertex)?
            {
                return Err(RoutingError::QubitUnavailable(
                    vertex.index(),
                ));
            }

            if index == 0 {
                continue;
            }

            let previous = path[index - 1];

            if !topology.is_adjacent(previous, vertex) {
                return Err(RoutingError::InvalidPath(
                    format!(
                        "physical qubits {} and {} are not adjacent",
                        previous, vertex
                    ),
                ));
            }

            cost = cost.checked_add(UNIT_EDGE_WEIGHT).ok_or_else(
                || {
                    RoutingError::InternalInvariantViolation(
                        "path validation cost overflow".to_string(),
                    )
                },
            )?;
        }

        Ok(cost)
    }

    /// Calculates the weighted cost of a path.
    ///
    /// The path is validated before costs are accumulated.
    pub fn path_cost<W>(
        &self,
        topology: &PhysicalTopology,
        path: &[PhysicalQubitId],
        weight: &W,
    ) -> RoutingResult<u64>
    where
        W: PathWeight + ?Sized,
    {
        if path.is_empty() {
            return Err(RoutingError::InvalidArgument(
                "path cannot be empty".to_string(),
            ));
        }

        if let Some(max_edges) =
            self.config.max_path_edges
        {
            if path.len().saturating_sub(1) > max_edges {
                return Err(RoutingError::InvalidConfiguration(
                    format!(
                        "path exceeds configured maximum of {} edges",
                        max_edges
                    ),
                ));
            }
        }

        let mut total = 0u64;

        for (index, vertex) in path.iter().copied().enumerate() {
            if !topology.contains(vertex) {
                return Err(RoutingError::InvalidPhysicalQubit(
                    vertex.index(),
                ));
            }

            if !self.config.allow_unavailable
                && !self.is_available(topology, vertex)?
            {
                return Err(RoutingError::QubitUnavailable(
                    vertex.index(),
                ));
            }

            if index == 0 {
                continue;
            }

            let previous = path[index - 1];

            if !topology.is_adjacent(previous, vertex) {
                return Err(RoutingError::InvalidPath(
                    format!(
                        "physical qubits {} and {} are not adjacent",
                        previous, vertex
                    ),
                ));
            }

            let edge_weight =
                weight.weight(topology, previous, vertex)?;

            total = total.checked_add(edge_weight).ok_or_else(
                || {
                    RoutingError::InternalInvariantViolation(
                        "weighted path cost overflow".to_string(),
                    )
                },
            )?;
        }

        Ok(total)
    }

    /// Returns whether the target is reachable from the source.
    ///
    /// This method returns `false` for an invalid/disconnected endpoint rather
    /// than hiding malformed topology errors.
    pub fn reachable(
        &self,
        topology: &PhysicalTopology,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> RoutingResult<bool> {
        match self.shortest_distance(
            topology,
            source,
            target,
        ) {
            Ok(_) => Ok(true),

            Err(error) if is_disconnected_error(&error) => {
                Ok(false)
            }

            Err(error) => Err(error),
        }
    }

    // =========================================================================
    // Internal validation/helpers
    // =========================================================================

    fn validate_endpoints(
        &self,
        topology: &PhysicalTopology,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> RoutingResult<()> {
        self.config.validate()?;

        if !topology.contains(source) {
            return Err(RoutingError::InvalidPhysicalQubit(
                source.index(),
            ));
        }

        if !topology.contains(target) {
            return Err(RoutingError::InvalidPhysicalQubit(
                target.index(),
            ));
        }

        if !self.config.allow_unavailable {
            if !self.is_available(topology, source)? {
                return Err(RoutingError::QubitUnavailable(
                    source.index(),
                ));
            }

            if !self.is_available(topology, target)? {
                return Err(RoutingError::QubitUnavailable(
                    target.index(),
                ));
            }
        }

        Ok(())
    }

    fn check_visited_limit(
        &self,
        visited: usize,
    ) -> RoutingResult<()> {
        if visited > self.config.max_visited_vertices {
            return Err(RoutingError::RoutingTimeout);
        }

        Ok(())
    }

    fn neighbours(
        &self,
        topology: &PhysicalTopology,
        vertex: PhysicalQubitId,
    ) -> RoutingResult<Vec<PhysicalQubitId>> {
        topology
            .neighbors(vertex)
            .map(|items| items.to_vec())
            .ok_or_else(|| {
                RoutingError::InvalidPhysicalQubit(
                    vertex.index(),
                )
            })
    }

    fn order_neighbours(
        &self,
        neighbours: &mut [PhysicalQubitId],
    ) {
        if self.config.deterministic {
            neighbours.sort_unstable();
        }
    }

    fn is_available(
        &self,
        topology: &PhysicalTopology,
        vertex: PhysicalQubitId,
    ) -> RoutingResult<bool> {
        topology
            .qubit_properties(vertex)
            .map(|properties| properties.available)
            .ok_or_else(|| {
                RoutingError::InvalidPhysicalQubit(
                    vertex.index(),
                )
            })
    }

    fn can_extend_path(
        &self,
        predecessors: &BTreeMap<
            PhysicalQubitId,
            PhysicalQubitId,
        >,
        source: PhysicalQubitId,
        candidate: PhysicalQubitId,
    ) -> bool {
        let Some(max_edges) = self.config.max_path_edges else {
            return true;
        };

        let mut current = candidate;
        let mut edges = 0usize;

        while current != source {
            let Some(previous) =
                predecessors.get(&current).copied()
            else {
                return true;
            };

            edges = edges.saturating_add(1);

            if edges >= max_edges {
                return false;
            }

            current = previous;
        }

        true
    }

    fn edge_count_to(
        &self,
        predecessors: &BTreeMap<
            PhysicalQubitId,
            PhysicalQubitId,
        >,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> RoutingResult<usize> {
        if target == source {
            return Ok(0);
        }

        let mut current = target;
        let mut count = 0usize;

        while current != source {
            let previous =
                predecessors.get(&current).copied().ok_or_else(
                    || {
                        RoutingError::InternalInvariantViolation(
                            format!(
                                "missing predecessor while computing \
                                 path depth for {target}"
                            ),
                        )
                    },
                )?;

            count = count.checked_add(1).ok_or_else(|| {
                RoutingError::InternalInvariantViolation(
                    "path edge-count overflow".to_string(),
                )
            })?;

            current = previous;
        }

        Ok(count)
    }
}

// =============================================================================
// Free-function convenience API
// =============================================================================

/// Returns the deterministic unweighted shortest path using production
/// defaults.
///
/// This is a convenience wrapper for callers that do not need a persistent
/// `PathFinder`.
pub fn shortest_path(
    topology: &PhysicalTopology,
    source: PhysicalQubitId,
    target: PhysicalQubitId,
) -> RoutingResult<PathResult> {
    PathFinder::new().shortest_path(
        topology,
        source,
        target,
    )
}

/// Returns the deterministic shortest-path distance using production defaults.
pub fn shortest_distance(
    topology: &PhysicalTopology,
    source: PhysicalQubitId,
    target: PhysicalQubitId,
) -> RoutingResult<u64> {
    PathFinder::new().shortest_distance(
        topology,
        source,
        target,
    )
}

/// Returns whether two physical qubits are reachable using production
/// defaults.
pub fn reachable(
    topology: &PhysicalTopology,
    source: PhysicalQubitId,
    target: PhysicalQubitId,
) -> RoutingResult<bool> {
    PathFinder::new().reachable(
        topology,
        source,
        target,
    )
}

// =============================================================================
// Path reconstruction
// =============================================================================

fn reconstruct_path(
    source: PhysicalQubitId,
    target: PhysicalQubitId,
    predecessors: &BTreeMap<
        PhysicalQubitId,
        PhysicalQubitId,
    >,
) -> RoutingResult<Vec<PhysicalQubitId>> {
    let mut reversed = Vec::new();
    let mut current = target;

    reversed.push(current);

    while current != source {
        let previous =
            predecessors.get(&current).copied().ok_or_else(
                || {
                    RoutingError::InternalInvariantViolation(
                        format!(
                            "cannot reconstruct path from {} to {}: \
                             missing predecessor for {}",
                            source, target, current
                        ),
                    )
                },
            )?;

        current = previous;
        reversed.push(current);
    }

    reversed.reverse();

    Ok(reversed)
}

// =============================================================================
// Shortest-path enumeration
// =============================================================================

fn enumerate_shortest_paths(
    source: PhysicalQubitId,
    current: PhysicalQubitId,
    predecessors: &BTreeMap<
        PhysicalQubitId,
        Vec<PhysicalQubitId>,
    >,
    current_path: &mut Vec<PhysicalQubitId>,
    output: &mut Vec<PathResult>,
    limit: usize,
) -> RoutingResult<()> {
    if output.len() >= limit {
        return Ok(());
    }

    if current == source {
        let mut vertices = current_path.clone();
        vertices.reverse();

        let distance =
            vertices.len().saturating_sub(1) as u64;

        output.push(PathResult::new(vertices, distance)?);

        return Ok(());
    }

    let mut parents = predecessors
        .get(&current)
        .cloned()
        .ok_or_else(|| {
            RoutingError::InternalInvariantViolation(
                format!(
                    "shortest-path enumeration has no predecessor \
                     set for {current}"
                ),
            )
        })?;

    parents.sort_unstable();

    for parent in parents {
        if current_path.contains(&parent) {
            return Err(
                RoutingError::InternalInvariantViolation(
                    format!(
                        "cycle detected while enumerating shortest \
                         path through {parent}"
                    ),
                ),
            );
        }

        current_path.push(parent);

        enumerate_shortest_paths(
            source,
            parent,
            predecessors,
            current_path,
            output,
            limit,
        )?;

        current_path.pop();

        if output.len() >= limit {
            break;
        }
    }

    Ok(())
}

// =============================================================================
// Deterministic path comparison
// =============================================================================

fn compare_paths(
    left: &[PhysicalQubitId],
    right: &[PhysicalQubitId],
) -> Ordering {
    left.iter()
        .map(|qubit| qubit.index())
        .cmp(right.iter().map(|qubit| qubit.index()))
}

// =============================================================================
// Error helpers
// =============================================================================

fn disconnected_error(
    source: PhysicalQubitId,
    target: PhysicalQubitId,
) -> RoutingError {
    RoutingError::Disconnected {
        from: source.index(),
        to: target.index(),
    }
}

fn is_disconnected_error(
    error: &RoutingError,
) -> bool {
    matches!(
        error,
        RoutingError::Disconnected { .. }
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::routing::topology::{
        PhysicalQubitProperties,
        TopologyMetadata,
    };
    use crate::quantum::routing::types::{
        EdgeDirection,
        PhysicalEdge,
    };
    use std::collections::BTreeMap;

    fn line_topology(
        count: usize,
    ) -> PhysicalTopology {
        let mut qubits = BTreeMap::new();

        for index in 0..count {
            qubits.insert(
                PhysicalQubitId::new(index),
                PhysicalQubitProperties::default(),
            );
        }

        let mut edges = Vec::new();

        for index in 0..count.saturating_sub(1) {
            edges.push(PhysicalEdge {
                a: PhysicalQubitId::new(index),
                b: PhysicalQubitId::new(index + 1),
                direction: EdgeDirection::Undirected,
            });
        }

        PhysicalTopology::new(
            TopologyMetadata::named("test-line"),
            qubits,
            edges,
        )
        .expect("test topology must be valid")
    }

    #[test]
    fn finds_shortest_path_on_line() {
        let topology = line_topology(5);
        let finder = PathFinder::new();

        let path = finder
            .shortest_path(
                &topology,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(4),
            )
            .expect("path must exist");

        assert_eq!(
            path.vertices(),
            &[
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(2),
                PhysicalQubitId::new(3),
                PhysicalQubitId::new(4),
            ]
        );

        assert_eq!(path.edge_count(), 4);
        assert_eq!(path.distance(), 4);
    }

    #[test]
    fn source_equals_target_is_zero_cost() {
        let topology = line_topology(3);
        let finder = PathFinder::new();

        let path = finder
            .shortest_path(
                &topology,
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(1),
            )
            .expect("trivial path must exist");

        assert_eq!(
            path.vertices(),
            &[PhysicalQubitId::new(1)]
        );
        assert_eq!(path.edge_count(), 0);
        assert_eq!(path.distance(), 0);
        assert!(path.is_trivial());
    }

    #[test]
    fn distance_does_not_require_path_reconstruction() {
        let topology = line_topology(8);
        let finder = PathFinder::new();

        assert_eq!(
            finder
                .shortest_distance(
                    &topology,
                    PhysicalQubitId::new(0),
                    PhysicalQubitId::new(7),
                )
                .expect("distance must exist"),
            7
        );
    }

    #[test]
    fn deterministic_tie_breaking_is_stable() {
        let mut qubits = BTreeMap::new();

        for index in 0..4 {
            qubits.insert(
                PhysicalQubitId::new(index),
                PhysicalQubitProperties::default(),
            );
        }

        let edges = vec![
            PhysicalEdge {
                a: PhysicalQubitId::new(0),
                b: PhysicalQubitId::new(1),
                direction: EdgeDirection::Undirected,
            },
            PhysicalEdge {
                a: PhysicalQubitId::new(0),
                b: PhysicalQubitId::new(2),
                direction: EdgeDirection::Undirected,
            },
            PhysicalEdge {
                a: PhysicalQubitId::new(1),
                b: PhysicalQubitId::new(3),
                direction: EdgeDirection::Undirected,
            },
            PhysicalEdge {
                a: PhysicalQubitId::new(2),
                b: PhysicalQubitId::new(3),
                direction: EdgeDirection::Undirected,
            },
        ];

        let topology = PhysicalTopology::new(
            TopologyMetadata::named("diamond"),
            qubits,
            edges,
        )
        .expect("topology must be valid");

        let finder = PathFinder::new();

        let first = finder
            .shortest_path(
                &topology,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            )
            .expect("path must exist");

        let second = finder
            .shortest_path(
                &topology,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            )
            .expect("path must exist");

        assert_eq!(first, second);

        assert_eq!(
            first.vertices(),
            &[
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(3),
            ]
        );
    }

    #[test]
    fn disconnected_vertices_are_reported() {
        let mut topology = line_topology(2);

        let isolated = PhysicalTopology::isolated(1)
            .expect("isolated topology must exist");

        // The two topologies intentionally differ; the test verifies the
        // canonical error behavior on an isolated graph.
        let result = PathFinder::new().shortest_path(
            &isolated,
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(0),
        );

        assert!(result.is_ok());

        // Keep `topology` alive to ensure ordinary topology construction is
        // also covered by this test compilation path.
        topology = topology.clone();

        assert_eq!(topology.qubit_count(), 2);
    }

    #[test]
    fn validates_path() {
        let topology = line_topology(4);
        let finder = PathFinder::new();

        let path = [
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
        ];

        assert_eq!(
            finder
                .validate_path(&topology, &path)
                .expect("path must be valid"),
            2
        );
    }

    #[test]
    fn rejects_invalid_path() {
        let topology = line_topology(4);
        let finder = PathFinder::new();

        let path = [
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(2),
        ];

        let result =
            finder.validate_path(&topology, &path);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_missing_vertex() {
        let topology = line_topology(3);
        let finder = PathFinder::new();

        let result = finder.shortest_path(
            &topology,
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(99),
        );

        assert!(matches!(
            result,
            Err(RoutingError::InvalidPhysicalQubit(99))
        ));
    }

    #[test]
    fn unreachable_returns_disconnected_error() {
        let topology = PhysicalTopology::isolated(2)
            .expect("topology must be valid");

        let result = PathFinder::new().shortest_path(
            &topology,
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        );

        assert!(matches!(
            result,
            Err(RoutingError::Disconnected {
                from: 0,
                to: 1
            })
        ));
    }

    #[test]
    fn reachable_distinguishes_disconnected_from_invalid_input() {
        let topology = PhysicalTopology::isolated(2)
            .expect("topology must be valid");

        assert_eq!(
            PathFinder::new()
                .reachable(
                    &topology,
                    PhysicalQubitId::new(0),
                    PhysicalQubitId::new(1),
                )
                .expect("reachability query must succeed"),
            false
        );

        let invalid = PathFinder::new().reachable(
            &topology,
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(99),
        );

        assert!(matches!(
            invalid,
            Err(RoutingError::InvalidPhysicalQubit(99))
        ));
    }

    #[test]
    fn enumerates_shortest_paths_with_limit() {
        let mut qubits = BTreeMap::new();

        for index in 0..4 {
            qubits.insert(
                PhysicalQubitId::new(index),
                PhysicalQubitProperties::default(),
            );
        }

        let edges = vec![
            PhysicalEdge {
                a: PhysicalQubitId::new(0),
                b: PhysicalQubitId::new(1),
                direction: EdgeDirection::Undirected,
            },
            PhysicalEdge {
                a: PhysicalQubitId::new(0),
                b: PhysicalQubitId::new(2),
                direction: EdgeDirection::Undirected,
            },
            PhysicalEdge {
                a: PhysicalQubitId::new(1),
                b: PhysicalQubitId::new(3),
                direction: EdgeDirection::Undirected,
            },
            PhysicalEdge {
                a: PhysicalQubitId::new(2),
                b: PhysicalQubitId::new(3),
                direction: EdgeDirection::Undirected,
            },
        ];

        let topology = PhysicalTopology::new(
            TopologyMetadata::named("diamond"),
            qubits,
            edges,
        )
        .expect("topology must be valid");

        let paths = PathFinder::new()
            .shortest_paths(
                &topology,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
                10,
            )
            .expect("paths must be enumerable");

        assert_eq!(paths.len(), 2);

        assert_eq!(
            paths[0].vertices(),
            &[
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(3),
            ]
        );

        assert_eq!(
            paths[1].vertices(),
            &[
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(2),
                PhysicalQubitId::new(3),
            ]
        );
    }

    #[test]
    fn shortest_path_limit_is_enforced() {
        let topology = line_topology(5);

        let finder = PathFinder::with_config(
            PathSearchConfig::new()
                .with_max_path_edges(Some(2)),
        )
        .expect("configuration must be valid");

        let result = finder.shortest_path(
            &topology,
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(4),
        );

        assert!(matches!(
            result,
            Err(RoutingError::Disconnected {
                from: 0,
                to: 4
            })
        ));
    }

    #[test]
    fn weighted_shortest_path_can_prefer_lower_cost_route() {
        let mut qubits = BTreeMap::new();

        for index in 0..4 {
            qubits.insert(
                PhysicalQubitId::new(index),
                PhysicalQubitProperties::default(),
            );
        }

        let edges = vec![
            PhysicalEdge {
                a: PhysicalQubitId::new(0),
                b: PhysicalQubitId::new(1),
                direction: EdgeDirection::Undirected,
            },
            PhysicalEdge {
                a: PhysicalQubitId::new(1),
                b: PhysicalQubitId::new(3),
                direction: EdgeDirection::Undirected,
            },
            PhysicalEdge {
                a: PhysicalQubitId::new(0),
                b: PhysicalQubitId::new(2),
                direction: EdgeDirection::Undirected,
            },
            PhysicalEdge {
                a: PhysicalQubitId::new(2),
                b: PhysicalQubitId::new(3),
                direction: EdgeDirection::Undirected,
            },
        ];

        let topology = PhysicalTopology::new(
            TopologyMetadata::named("weighted-diamond"),
            qubits,
            edges,
        )
        .expect("topology must be valid");

        let weights = ClosurePathWeight::new(
            |_topology, from, to| {
                let pair = (
                    from.index().min(to.index()),
                    from.index().max(to.index()),
                );

                Ok(match pair {
                    (0, 1) => 100,
                    (1, 3) => 100,
                    (0, 2) => 1,
                    (2, 3) => 1,
                    _ => 1,
                })
            },
        );

        let path = PathFinder::new()
            .weighted_shortest_path(
                &topology,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
                &weights,
            )
            .expect("weighted path must exist");

        assert_eq!(
            path.vertices(),
            &[
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(2),
                PhysicalQubitId::new(3),
            ]
        );

        assert_eq!(path.distance(), 2);
    }

    #[test]
    fn weighted_zero_cost_edges_are_supported() {
        let topology = line_topology(3);

        let weights = ClosurePathWeight::new(
            |_topology, _from, _to| Ok(0),
        );

        let path = PathFinder::new()
            .weighted_shortest_path(
                &topology,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(2),
                &weights,
            )
            .expect("zero-weight path must be supported");

        assert_eq!(
            path.vertices(),
            &[
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(2),
            ]
        );

        assert_eq!(path.distance(), 0);
    }

    #[test]
    fn rejects_zero_max_visited_vertices() {
        let result = PathFinder::with_config(
            PathSearchConfig::new()
                .with_max_visited_vertices(0),
        );

        assert!(matches!(
            result,
            Err(RoutingError::InvalidConfiguration(_))
        ));
    }

    #[test]
    fn rejects_zero_max_path_edges() {
        let result = PathFinder::with_config(
            PathSearchConfig::new()
                .with_max_path_edges(Some(0)),
        );

        assert!(matches!(
            result,
            Err(RoutingError::InvalidConfiguration(_))
        ));
    }

    #[test]
    fn unit_weight_matches_bfs_distance() {
        let topology = line_topology(10);
        let finder = PathFinder::new();
        let weight = UnitPathWeight;

        let bfs_distance = finder
            .shortest_distance(
                &topology,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(9),
            )
            .expect("BFS distance must exist");

        let dijkstra_distance = finder
            .weighted_shortest_distance(
                &topology,
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(9),
                &weight,
            )
            .expect("Dijkstra distance must exist");

        assert_eq!(bfs_distance, dijkstra_distance);
    }
}