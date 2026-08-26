//! Zamani Quantum Benchmarking — Hardware Topology
//!
//! Production-grade, backend-independent topology representation and
//! topology analysis for the quantum benchmarking subsystem.
//!
//! # Responsibility
//!
//! This module owns the benchmarking view of hardware connectivity.
//!
//! It answers questions such as:
//!
//! - How many physical qubits/modes does the target expose?
//! - Which physical resources are directly connected?
//! - Are couplings directed or bidirectional?
//! - Is the target connected?
//! - How many connected components exist?
//! - What are the minimum/maximum/average degrees?
//! - What is the shortest routing distance between two resources?
//! - What is the topology diameter?
//! - What fraction of possible couplings exists?
//! - What stable fingerprint identifies this topology snapshot?
//!
//! It deliberately does NOT:
//!
//! - perform routing;
//! - transpile circuits;
//! - mutate the canonical hardware topology;
//! - execute quantum operations;
//! - own calibration data;
//! - own backend availability;
//! - own gate decomposition;
//! - schedule operations;
//! - perform network/device I/O;
//! - infer unsupported physical properties;
//! - silently convert directed connectivity into bidirectional connectivity.
//!
//! # Architectural position
//!
//! ```text
//! canonical hardware topology
//!          │
//!          ▼
//! benchmarking::hardware::topology
//!          │
//!          ├── topology validation
//!          ├── topology statistics
//!          ├── connectivity analysis
//!          ├── deterministic shortest paths
//!          ├── volumetric benchmark metadata
//!          ├── routing-cost measurements
//!          └── reproducibility fingerprint
//!          │
//!          ▼
//! benchmark protocols / analysis / reporting
//! ```
//!
//! The canonical hardware topology remains authoritative for hardware
//! connectivity. This module is a benchmarking projection/snapshot.
//!
//! # Important architectural rule
//!
//! `benchmarking::hardware::topology` may consume
//! `quantum::hardware::topology`, but the canonical hardware subsystem must
//! never depend on benchmarking.
//!
//! Therefore the dependency direction is:
//!
//! ```text
//! quantum::hardware::topology
//!          │
//!          ▼
//! benchmarking::hardware::topology
//! ```
//!
//! Never:
//!
//! ```text
//! benchmarking → hardware → benchmarking
//! ```
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1 / Rust 2021.
//!
//! This implementation intentionally uses only the Rust standard library.
//! No nightly features are required.
//!
//! # Determinism
//!
//! Benchmark results must be reproducible. Consequently:
//!
//! - qubit identifiers are ordered;
//! - adjacency lists are sorted;
//! - traversal order is deterministic;
//! - topology fingerprints use canonical ordering;
//! - no `HashMap` iteration order is used for observable results.
//!
//! # Scope
//!
//! The model intentionally uses generic `ResourceId` terminology internally
//! while exposing qubit-oriented aliases. This allows future benchmarking of
//! photonic modes, qudits, bosonic modes, or other discrete resources without
//! changing the fundamental graph model.
//!
//! # Integration contract
//!
//! This file is independently complete.
//!
//! Downstream files can consume it without requiring later changes:
//!
//! - `benchmarking::hardware::capabilities`
//! - `benchmarking::protocols::*`
//! - `benchmarking::volumetric::*`
//! - `benchmarking::metrics::resource`
//! - `benchmarking::analysis::*`
//! - `benchmarking::core::provenance`
//! - `benchmarking::reporting::*`
//!
//! The canonical hardware implementation can later be adapted through
//! `from_hardware_topology()` without changing this module's public
//! representation.
//!

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

// =============================================================================
// Schema/versioning
// =============================================================================

/// Stable schema version for the benchmarking topology representation.
///
/// Increment this when the meaning or serialized interpretation of topology
/// metadata changes incompatibly.
pub const TOPOLOGY_SCHEMA_VERSION: u16 = 1;

/// Stable identifier for this topology representation.
pub const TOPOLOGY_SCHEMA_ID: &str = "zamani.quantum.benchmarking.hardware.topology";

// =============================================================================
// Resource identifiers
// =============================================================================

/// Generic physical quantum resource identifier.
///
/// For ordinary qubit hardware this is the physical qubit index.
///
/// The type remains a `usize` so it can be used efficiently by graph
/// algorithms and can be converted to/from the canonical hardware topology.
pub type ResourceId = usize;

/// Physical qubit identifier.
///
/// This alias exists for API readability and compatibility with the
/// canonical hardware subsystem.
pub type QubitId = ResourceId;

// =============================================================================
// Connectivity direction
// =============================================================================

/// Directionality of a physical coupling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Connectivity {
    /// Operations may use the coupling in either direction.
    Bidirectional,

    /// The native coupling is available only from `source` to `target`.
    Directed,
}

impl Connectivity {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bidirectional => "bidirectional",
            Self::Directed => "directed",
        }
    }

    /// Returns whether this coupling permits traversal in the supplied
    /// direction.
    pub const fn permits(self, source: ResourceId, target: ResourceId) -> bool {
        match self {
            Self::Bidirectional => {
                // The caller normally invokes this on the corresponding
                // coupling. Both directions are permitted.
                source != target
            }
            Self::Directed => {
                source != target
            }
        }
    }
}

// =============================================================================
// Coupling
// =============================================================================

/// A physical connectivity edge.
///
/// A coupling contains no calibration information. Gate error, duration,
/// fidelity and other physical properties belong to calibration/hardware
/// metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coupling {
    /// Native source resource.
    pub source: ResourceId,

    /// Native target resource.
    pub target: ResourceId,

    /// Whether the connection is directional.
    pub connectivity: Connectivity,
}

impl Coupling {
    /// Creates a bidirectional coupling.
    pub const fn bidirectional(
        source: ResourceId,
        target: ResourceId,
    ) -> Self {
        Self {
            source,
            target,
            connectivity: Connectivity::Bidirectional,
        }
    }

    /// Creates a directed coupling.
    pub const fn directed(
        source: ResourceId,
        target: ResourceId,
    ) -> Self {
        Self {
            source,
            target,
            connectivity: Connectivity::Directed,
        }
    }

    /// Returns the opposite endpoint for an incident edge.
    pub const fn opposite(
        self,
        resource: ResourceId,
    ) -> Option<ResourceId> {
        if self.source == resource {
            Some(self.target)
        } else if self.target == resource {
            Some(self.source)
        } else {
            None
        }
    }

    /// Returns whether this edge represents the supplied native direction.
    pub const fn permits_native_direction(
        self,
        source: ResourceId,
        target: ResourceId,
    ) -> bool {
        match self.connectivity {
            Connectivity::Bidirectional => {
                (self.source == source && self.target == target)
                    || (self.source == target && self.target == source)
            }

            Connectivity::Directed => {
                self.source == source && self.target == target
            }
        }
    }

    /// Returns whether this coupling touches a resource.
    pub const fn contains(self, resource: ResourceId) -> bool {
        self.source == resource || self.target == resource
    }
}

// =============================================================================
// Topology errors
// =============================================================================

/// Errors produced while constructing or analysing a benchmark topology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyError {
    /// A topology cannot contain zero physical resources.
    ZeroResources,

    /// A resource identifier is outside the topology.
    InvalidResource {
        resource: ResourceId,
        resource_count: usize,
    },

    /// A coupling connects a resource to itself.
    SelfCoupling {
        resource: ResourceId,
    },

    /// A coupling already exists.
    DuplicateCoupling {
        source: ResourceId,
        target: ResourceId,
    },

    /// A requested native connection does not exist.
    MissingCoupling {
        source: ResourceId,
        target: ResourceId,
    },

    /// No path exists under the requested traversal semantics.
    NoPath {
        source: ResourceId,
        target: ResourceId,
    },

    /// A topology invariant was violated.
    InvalidTopology {
        message: String,
    },

    /// A conversion from the canonical hardware topology failed.
    ConversionError {
        message: String,
    },

    /// A numeric calculation could not be represented safely.
    NumericOverflow {
        operation: &'static str,
    },
}

impl fmt::Display for TopologyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroResources => {
                write!(
                    f,
                    "benchmark topology must contain at least one resource"
                )
            }

            Self::InvalidResource {
                resource,
                resource_count,
            } => {
                write!(
                    f,
                    "resource {} is outside topology containing {} resources",
                    resource,
                    resource_count
                )
            }

            Self::SelfCoupling { resource } => {
                write!(
                    f,
                    "resource {} cannot be coupled to itself",
                    resource
                )
            }

            Self::DuplicateCoupling { source, target } => {
                write!(
                    f,
                    "coupling between resources {} and {} already exists",
                    source,
                    target
                )
            }

            Self::MissingCoupling { source, target } => {
                write!(
                    f,
                    "no native coupling exists from resource {} to {}",
                    source,
                    target
                )
            }

            Self::NoPath { source, target } => {
                write!(
                    f,
                    "no topology path exists from resource {} to {}",
                    source,
                    target
                )
            }

            Self::InvalidTopology { message } => {
                write!(f, "invalid benchmark topology: {}", message)
            }

            Self::ConversionError { message } => {
                write!(
                    f,
                    "failed to convert canonical hardware topology: {}",
                    message
                )
            }

            Self::NumericOverflow { operation } => {
                write!(
                    f,
                    "numeric overflow while calculating topology {}",
                    operation
                )
            }
        }
    }
}

impl std::error::Error for TopologyError {}

// =============================================================================
// Path semantics
// =============================================================================

/// Controls how graph traversal interprets directed couplings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PathMode {
    /// Respect native coupling direction.
    Directed,

    /// Treat every physical coupling as an undirected physical adjacency.
    ///
    /// This is useful for physical-distance and connectivity analysis where
    /// direction of the native gate is not itself the quantity being measured.
    Undirected,
}

impl PathMode {
    /// Stable machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Directed => "directed",
            Self::Undirected => "undirected",
        }
    }
}

// =============================================================================
// Topology statistics
// =============================================================================

/// Deterministic topology statistics.
///
/// These values are calculated from the topology snapshot only. They do not
/// claim anything about gate fidelity, error rates, calibration quality or
/// actual execution performance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TopologyStatistics {
    /// Number of physical resources.
    pub resource_count: usize,

    /// Number of explicitly represented coupling edges.
    pub coupling_count: usize,

    /// Number of directed edges.
    pub directed_coupling_count: usize,

    /// Number of bidirectional couplings.
    pub bidirectional_coupling_count: usize,

    /// Number of resources with at least one incident coupling.
    pub connected_resource_count: usize,

    /// Number of weakly connected components.
    pub connected_components: usize,

    /// Minimum undirected degree.
    pub minimum_degree: usize,

    /// Maximum undirected degree.
    pub maximum_degree: usize,

    /// Average undirected degree.
    pub average_degree: f64,

    /// Density over unordered resource pairs.
    ///
    /// Range: `[0, 1]`.
    pub undirected_density: f64,

    /// Whether all resources belong to one weakly connected component.
    pub is_connected: bool,

    /// Undirected topology diameter.
    ///
    /// `None` when the topology is disconnected.
    pub diameter: Option<usize>,

    /// Average finite pairwise undirected distance.
    ///
    /// `None` when there are no distinct resource pairs.
    pub average_shortest_path: Option<f64>,
}

impl TopologyStatistics {
    /// Returns whether this topology has a single connected component.
    pub const fn is_fully_connected(&self) -> bool {
        self.is_connected
    }
}

// =============================================================================
// HardwareTopology
// =============================================================================

/// Immutable-after-construction benchmarking topology snapshot.
///
/// This is intentionally separate from the canonical hardware topology.
///
/// The snapshot is suitable for:
///
/// - benchmark provenance;
/// - reproducibility;
/// - topology metrics;
/// - topology-aware benchmark generation;
/// - comparison between hardware versions;
/// - regression analysis;
/// - reporting.
///
/// Routing/transpilation must continue to use the canonical hardware/routing
/// subsystem.
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareTopology {
    /// Number of physical resources.
    resource_count: usize,

    /// Canonically ordered physical couplings.
    couplings: Vec<Coupling>,

    /// Outgoing native adjacency.
    outgoing: BTreeMap<ResourceId, Vec<ResourceId>>,

    /// Incoming native adjacency.
    incoming: BTreeMap<ResourceId, Vec<ResourceId>>,

    /// Undirected physical adjacency.
    undirected: BTreeMap<ResourceId, Vec<ResourceId>>,
}

impl HardwareTopology {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates an empty-coupling topology with `resource_count` resources.
    pub fn new(resource_count: usize) -> Result<Self, TopologyError> {
        if resource_count == 0 {
            return Err(TopologyError::ZeroResources);
        }

        let mut outgoing = BTreeMap::new();
        let mut incoming = BTreeMap::new();
        let mut undirected = BTreeMap::new();

        for resource in 0..resource_count {
            outgoing.insert(resource, Vec::new());
            incoming.insert(resource, Vec::new());
            undirected.insert(resource, Vec::new());
        }

        Ok(Self {
            resource_count,
            couplings: Vec::new(),
            outgoing,
            incoming,
            undirected,
        })
    }

    /// Creates a topology from a coupling list.
    ///
    /// All invariants are checked before the resulting topology is returned.
    pub fn from_couplings<I>(
        resource_count: usize,
        couplings: I,
    ) -> Result<Self, TopologyError>
    where
        I: IntoIterator<Item = Coupling>,
    {
        let mut topology = Self::new(resource_count)?;

        for coupling in couplings {
            topology.add_coupling(coupling)?;
        }

        Ok(topology)
    }

    /// Creates a fully connected bidirectional topology.
    pub fn fully_connected(
        resource_count: usize,
    ) -> Result<Self, TopologyError> {
        let mut topology = Self::new(resource_count)?;

        for source in 0..resource_count {
            for target in (source + 1)..resource_count {
                topology.add_coupling(
                    Coupling::bidirectional(source, target),
                )?;
            }
        }

        Ok(topology)
    }

    /// Creates a bidirectional linear nearest-neighbour topology.
    pub fn linear(
        resource_count: usize,
    ) -> Result<Self, TopologyError> {
        let mut topology = Self::new(resource_count)?;

        for source in 0..resource_count.saturating_sub(1) {
            topology.add_coupling(
                Coupling::bidirectional(source, source + 1),
            )?;
        }

        Ok(topology)
    }

    /// Creates a bidirectional ring topology.
    ///
    /// A one-resource topology contains no edge.
    /// A two-resource topology contains exactly one edge; a second edge would
    /// represent the same physical adjacency.
    pub fn ring(
        resource_count: usize,
    ) -> Result<Self, TopologyError> {
        let mut topology = Self::linear(resource_count)?;

        if resource_count > 2 {
            topology.add_coupling(
                Coupling::bidirectional(
                    resource_count - 1,
                    0,
                ),
            )?;
        }

        Ok(topology)
    }

    /// Creates a two-dimensional rectangular lattice.
    ///
    /// `rows * columns` must fit in `usize`.
    pub fn grid(
        rows: usize,
        columns: usize,
    ) -> Result<Self, TopologyError> {
        if rows == 0 || columns == 0 {
            return Err(TopologyError::ZeroResources);
        }

        let resource_count = rows.checked_mul(columns).ok_or(
            TopologyError::NumericOverflow {
                operation: "grid resource count",
            },
        )?;

        let mut topology = Self::new(resource_count)?;

        for row in 0..rows {
            for column in 0..columns {
                let current = row * columns + column;

                if column + 1 < columns {
                    let right = current + 1;

                    topology.add_coupling(
                        Coupling::bidirectional(current, right),
                    )?;
                }

                if row + 1 < rows {
                    let below = current + columns;

                    topology.add_coupling(
                        Coupling::bidirectional(current, below),
                    )?;
                }
            }
        }

        Ok(topology)
    }

    // -------------------------------------------------------------------------
    // Mutation during construction
    // -------------------------------------------------------------------------

    /// Adds one coupling.
    ///
    /// This method is intentionally public because topology builders and
    /// adapters may construct a snapshot incrementally.
    ///
    /// Once handed to a benchmark, the topology should normally be treated as
    /// immutable by convention.
    pub fn add_coupling(
        &mut self,
        coupling: Coupling,
    ) -> Result<(), TopologyError> {
        self.validate_resource(coupling.source)?;
        self.validate_resource(coupling.target)?;

        if coupling.source == coupling.target {
            return Err(TopologyError::SelfCoupling {
                resource: coupling.source,
            });
        }

        if self.has_native_connection(
            coupling.source,
            coupling.target,
        ) {
            return Err(TopologyError::DuplicateCoupling {
                source: coupling.source,
                target: coupling.target,
            });
        }

        // A bidirectional edge represents both physical directions.
        // Therefore it cannot coexist with either directed representation.
        if coupling.connectivity == Connectivity::Bidirectional
            && (self.has_native_connection(
                coupling.target,
                coupling.source,
            ) || self.has_native_connection(
                coupling.source,
                coupling.target,
            ))
        {
            return Err(TopologyError::DuplicateCoupling {
                source: coupling.source,
                target: coupling.target,
            });
        }

        // If a directed edge already exists in the reverse direction and this
        // edge is bidirectional, it is also a duplicate physical adjacency.
        if coupling.connectivity == Connectivity::Directed
            && self.has_bidirectional_connection(
                coupling.source,
                coupling.target,
            )
        {
            return Err(TopologyError::DuplicateCoupling {
                source: coupling.source,
                target: coupling.target,
            });
        }

        self.couplings.push(coupling);

        self.couplings.sort_unstable();

        self.rebuild_indexes();

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Basic accessors
    // -------------------------------------------------------------------------

    /// Number of physical resources.
    pub const fn resource_count(&self) -> usize {
        self.resource_count
    }

    /// Number of represented couplings.
    pub const fn coupling_count(&self) -> usize {
        self.couplings.len()
    }

    /// Returns all couplings in canonical deterministic order.
    pub fn couplings(&self) -> &[Coupling] {
        &self.couplings
    }

    /// Returns all physical resource identifiers in ascending order.
    pub fn resources(&self) -> impl Iterator<Item = ResourceId> {
        0..self.resource_count
    }

    /// Returns whether the resource exists.
    pub const fn contains_resource(
        &self,
        resource: ResourceId,
    ) -> bool {
        resource < self.resource_count
    }

    /// Returns the topology schema version.
    pub const fn schema_version(&self) -> u16 {
        TOPOLOGY_SCHEMA_VERSION
    }

    /// Returns the topology schema identifier.
    pub const fn schema_id(&self) -> &'static str {
        TOPOLOGY_SCHEMA_ID
    }

    // -------------------------------------------------------------------------
    // Connectivity queries
    // -------------------------------------------------------------------------

    /// Returns native outgoing neighbours.
    ///
    /// Directed edges are respected.
    pub fn outgoing_neighbours(
        &self,
        resource: ResourceId,
    ) -> Result<&[ResourceId], TopologyError> {
        self.validate_resource(resource)?;

        Ok(self
            .outgoing
            .get(&resource)
            .map(Vec::as_slice)
            .unwrap_or(&[]))
    }

    /// Returns native incoming neighbours.
    pub fn incoming_neighbours(
        &self,
        resource: ResourceId,
    ) -> Result<&[ResourceId], TopologyError> {
        self.validate_resource(resource)?;

        Ok(self
            .incoming
            .get(&resource)
            .map(Vec::as_slice)
            .unwrap_or(&[]))
    }

    /// Returns physical neighbours ignoring coupling direction.
    pub fn neighbours(
        &self,
        resource: ResourceId,
    ) -> Result<&[ResourceId], TopologyError> {
        self.validate_resource(resource)?;

        Ok(self
            .undirected
            .get(&resource)
            .map(Vec::as_slice)
            .unwrap_or(&[]))
    }

    /// Returns whether a native operation is supported from `source` to
    /// `target`.
    pub fn is_connected(
        &self,
        source: ResourceId,
        target: ResourceId,
    ) -> Result<bool, TopologyError> {
        self.validate_resource(source)?;
        self.validate_resource(target)?;

        Ok(self.has_native_connection(source, target))
    }

    /// Returns whether two resources share a physical coupling, ignoring
    /// direction.
    pub fn is_physically_adjacent(
        &self,
        source: ResourceId,
        target: ResourceId,
    ) -> Result<bool, TopologyError> {
        self.validate_resource(source)?;
        self.validate_resource(target)?;

        Ok(self
            .undirected
            .get(&source)
            .map(|items| items.binary_search(&target).is_ok())
            .unwrap_or(false))
    }

    /// Returns the exact coupling for a native direction if one exists.
    pub fn coupling(
        &self,
        source: ResourceId,
        target: ResourceId,
    ) -> Result<Coupling, TopologyError> {
        self.validate_resource(source)?;
        self.validate_resource(target)?;

        self.couplings
            .iter()
            .copied()
            .find(|coupling| {
                coupling.permits_native_direction(source, target)
            })
            .ok_or(TopologyError::MissingCoupling {
                source,
                target,
            })
    }

    // -------------------------------------------------------------------------
    // Path finding
    // -------------------------------------------------------------------------

    /// Finds a deterministic shortest path.
    ///
    /// `PathMode::Directed` respects native gate direction.
    ///
    /// `PathMode::Undirected` treats every physical coupling as an adjacency.
    pub fn shortest_path(
        &self,
        source: ResourceId,
        target: ResourceId,
        mode: PathMode,
    ) -> Result<Vec<ResourceId>, TopologyError> {
        self.validate_resource(source)?;
        self.validate_resource(target)?;

        if source == target {
            return Ok(vec![source]);
        }

        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        let mut predecessor = BTreeMap::<ResourceId, ResourceId>::new();

        queue.push_back(source);
        visited.insert(source);

        while let Some(current) = queue.pop_front() {
            let neighbours = match mode {
                PathMode::Directed => self
                    .outgoing
                    .get(&current)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),

                PathMode::Undirected => self
                    .undirected
                    .get(&current)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),
            };

            for &next in neighbours {
                if !visited.insert(next) {
                    continue;
                }

                predecessor.insert(next, current);

                if next == target {
                    return reconstruct_path(
                        source,
                        target,
                        &predecessor,
                    );
                }

                queue.push_back(next);
            }
        }

        Err(TopologyError::NoPath { source, target })
    }

    /// Returns the shortest path length in physical couplings.
    pub fn distance(
        &self,
        source: ResourceId,
        target: ResourceId,
        mode: PathMode,
    ) -> Result<usize, TopologyError> {
        let path = self.shortest_path(source, target, mode)?;

        Ok(path.len().saturating_sub(1))
    }

    /// Returns whether a path exists.
    pub fn has_path(
        &self,
        source: ResourceId,
        target: ResourceId,
        mode: PathMode,
    ) -> Result<bool, TopologyError> {
        match self.shortest_path(source, target, mode) {
            Ok(_) => Ok(true),
            Err(TopologyError::NoPath { .. }) => Ok(false),
            Err(error) => Err(error),
        }
    }

    // -------------------------------------------------------------------------
    // Connectivity analysis
    // -------------------------------------------------------------------------

    /// Returns the weakly connected component containing `resource`.
    ///
    /// Coupling direction is ignored.
    pub fn component(
        &self,
        resource: ResourceId,
    ) -> Result<Vec<ResourceId>, TopologyError> {
        self.validate_resource(resource)?;

        let mut component = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();

        queue.push_back(resource);
        visited.insert(resource);

        while let Some(current) = queue.pop_front() {
            component.push(current);

            let neighbours = self
                .undirected
                .get(&current)
                .map(Vec::as_slice)
                .unwrap_or(&[]);

            for &next in neighbours {
                if visited.insert(next) {
                    queue.push_back(next);
                }
            }
        }

        component.sort_unstable();

        Ok(component)
    }

    /// Returns all weakly connected components.
    ///
    /// Components and their resource IDs are deterministic.
    pub fn connected_components(
        &self,
    ) -> Vec<Vec<ResourceId>> {
        let mut result = Vec::new();
        let mut visited = BTreeSet::new();

        for resource in self.resources() {
            if visited.contains(&resource) {
                continue;
            }

            let mut component = Vec::new();
            let mut queue = VecDeque::new();

            queue.push_back(resource);
            visited.insert(resource);

            while let Some(current) = queue.pop_front() {
                component.push(current);

                let neighbours = self
                    .undirected
                    .get(&current)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]);

                for &next in neighbours {
                    if visited.insert(next) {
                        queue.push_back(next);
                    }
                }
            }

            component.sort_unstable();
            result.push(component);
        }

        result
    }

    /// Returns the number of weakly connected components.
    pub fn connected_component_count(&self) -> usize {
        self.connected_components().len()
    }

    /// Returns whether all physical resources belong to one weakly connected
    /// component.
    pub fn is_connected(&self) -> bool {
        self.resource_count > 0
            && self.connected_component_count() == 1
    }

    // -------------------------------------------------------------------------
    // Degree analysis
    // -------------------------------------------------------------------------

    /// Returns the undirected degree of a resource.
    pub fn degree(
        &self,
        resource: ResourceId,
    ) -> Result<usize, TopologyError> {
        self.validate_resource(resource)?;

        Ok(self
            .undirected
            .get(&resource)
            .map(Vec::len)
            .unwrap_or(0))
    }

    /// Returns the native outgoing degree.
    pub fn out_degree(
        &self,
        resource: ResourceId,
    ) -> Result<usize, TopologyError> {
        self.validate_resource(resource)?;

        Ok(self
            .outgoing
            .get(&resource)
            .map(Vec::len)
            .unwrap_or(0))
    }

    /// Returns the native incoming degree.
    pub fn in_degree(
        &self,
        resource: ResourceId,
    ) -> Result<usize, TopologyError> {
        self.validate_resource(resource)?;

        Ok(self
            .incoming
            .get(&resource)
            .map(Vec::len)
            .unwrap_or(0))
    }

    /// Returns the minimum undirected degree.
    pub fn minimum_degree(&self) -> usize {
        self.resources()
            .map(|resource| {
                self.undirected
                    .get(&resource)
                    .map(Vec::len)
                    .unwrap_or(0)
            })
            .min()
            .unwrap_or(0)
    }

    /// Returns the maximum undirected degree.
    pub fn maximum_degree(&self) -> usize {
        self.resources()
            .map(|resource| {
                self.undirected
                    .get(&resource)
                    .map(Vec::len)
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
    }

    /// Returns the average undirected degree.
    pub fn average_degree(&self) -> f64 {
        if self.resource_count == 0 {
            return 0.0;
        }

        let total: usize = self
            .resources()
            .map(|resource| {
                self.undirected
                    .get(&resource)
                    .map(Vec::len)
                    .unwrap_or(0)
            })
            .sum();

        total as f64 / self.resource_count as f64
    }

    // -------------------------------------------------------------------------
    // Density
    // -------------------------------------------------------------------------

    /// Returns the density of physical adjacency.
    ///
    /// The denominator is the number of unordered resource pairs:
    ///
    /// `n * (n - 1) / 2`
    ///
    /// The result is in `[0, 1]`.
    pub fn undirected_density(&self) -> f64 {
        if self.resource_count < 2 {
            return 0.0;
        }

        let possible = match self
            .resource_count
            .checked_mul(self.resource_count - 1)
            .and_then(|value| value.checked_div(2))
        {
            Some(value) => value,
            None => return 0.0,
        };

        if possible == 0 {
            return 0.0;
        }

        let actual = self
            .undirected
            .values()
            .map(Vec::len)
            .sum::<usize>()
            / 2;

        actual as f64 / possible as f64
    }

    // -------------------------------------------------------------------------
    // Diameter / distance analysis
    // -------------------------------------------------------------------------

    /// Returns the undirected topology diameter.
    ///
    /// Returns `None` when the topology is disconnected.
    pub fn diameter(&self) -> Option<usize> {
        if !self.is_connected() {
            return None;
        }

        if self.resource_count <= 1 {
            return Some(0);
        }

        let mut maximum = 0usize;

        for source in self.resources() {
            let distances = self
                .distances_from(source, PathMode::Undirected);

            for target in self.resources() {
                if let Some(distance) = distances.get(&target) {
                    maximum = maximum.max(*distance);
                }
            }
        }

        Some(maximum)
    }

    /// Returns the average finite undirected shortest-path distance.
    ///
    /// Each unordered pair is counted exactly once.
    pub fn average_shortest_path(&self) -> Option<f64> {
        if self.resource_count < 2 {
            return None;
        }

        let mut total = 0usize;
        let mut pair_count = 0usize;

        for source in 0..self.resource_count {
            let distances = self
                .distances_from(
                    source,
                    PathMode::Undirected,
                );

            for target in (source + 1)..self.resource_count {
                if let Some(distance) = distances.get(&target) {
                    total = total.checked_add(*distance)?;
                    pair_count = pair_count.checked_add(1)?;
                }
            }
        }

        if pair_count == 0 {
            None
        } else {
            Some(total as f64 / pair_count as f64)
        }
    }

    /// Returns all shortest-path distances from a source.
    ///
    /// The map contains the source with distance zero and every reachable
    /// resource.
    pub fn distances_from(
        &self,
        source: ResourceId,
        mode: PathMode,
    ) -> BTreeMap<ResourceId, usize> {
        let mut distances = BTreeMap::new();

        if !self.contains_resource(source) {
            return distances;
        }

        let mut queue = VecDeque::new();

        distances.insert(source, 0);
        queue.push_back(source);

        while let Some(current) = queue.pop_front() {
            let current_distance =
                distances.get(&current).copied().unwrap_or(0);

            let neighbours = match mode {
                PathMode::Directed => self
                    .outgoing
                    .get(&current)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),

                PathMode::Undirected => self
                    .undirected
                    .get(&current)
                    .map(Vec::as_slice)
                    .unwrap_or(&[]),
            };

            for &next in neighbours {
                if distances.contains_key(&next) {
                    continue;
                }

                distances.insert(
                    next,
                    current_distance.saturating_add(1),
                );

                queue.push_back(next);
            }
        }

        distances
    }

    // -------------------------------------------------------------------------
    // Resource classification
    // -------------------------------------------------------------------------

    /// Returns resources that have no physical neighbours.
    pub fn isolated_resources(&self) -> Vec<ResourceId> {
        self.resources()
            .filter(|resource| {
                self.undirected
                    .get(resource)
                    .map(Vec::is_empty)
                    .unwrap_or(true)
            })
            .collect()
    }

    /// Returns resources having the maximum undirected degree.
    pub fn highest_degree_resources(&self) -> Vec<ResourceId> {
        let maximum = self.maximum_degree();

        self.resources()
            .filter(|resource| {
                self.undirected
                    .get(resource)
                    .map(Vec::len)
                    .unwrap_or(0)
                    == maximum
            })
            .collect()
    }

    /// Returns resources having the minimum undirected degree.
    pub fn lowest_degree_resources(&self) -> Vec<ResourceId> {
        let minimum = self.minimum_degree();

        self.resources()
            .filter(|resource| {
                self.undirected
                    .get(resource)
                    .map(Vec::len)
                    .unwrap_or(0)
                    == minimum
            })
            .collect()
    }

    /// Returns whether the topology contains at least one directed coupling.
    pub fn has_directed_couplings(&self) -> bool {
        self.couplings
            .iter()
            .any(|coupling| {
                coupling.connectivity == Connectivity::Directed
            })
    }

    /// Returns whether the topology contains at least one bidirectional
    /// coupling.
    pub fn has_bidirectional_couplings(&self) -> bool {
        self.couplings
            .iter()
            .any(|coupling| {
                coupling.connectivity
                    == Connectivity::Bidirectional
            })
    }

    /// Counts directed couplings.
    pub fn directed_coupling_count(&self) -> usize {
        self.couplings
            .iter()
            .filter(|coupling| {
                coupling.connectivity == Connectivity::Directed
            })
            .count()
    }

    /// Counts bidirectional couplings.
    pub fn bidirectional_coupling_count(&self) -> usize {
        self.couplings
            .iter()
            .filter(|coupling| {
                coupling.connectivity
                    == Connectivity::Bidirectional
            })
            .count()
    }

    /// Counts resources that have at least one physical neighbour.
    pub fn connected_resource_count(&self) -> usize {
        self.resources()
            .filter(|resource| {
                self.undirected
                    .get(resource)
                    .map(|items| !items.is_empty())
                    .unwrap_or(false)
            })
            .count()
    }

    // -------------------------------------------------------------------------
    // Statistics
    // -------------------------------------------------------------------------

    /// Calculates the complete topology statistics snapshot.
    pub fn statistics(&self) -> TopologyStatistics {
        TopologyStatistics {
            resource_count: self.resource_count,
            coupling_count: self.coupling_count(),
            directed_coupling_count:
                self.directed_coupling_count(),
            bidirectional_coupling_count:
                self.bidirectional_coupling_count(),
            connected_resource_count:
                self.connected_resource_count(),
            connected_components:
                self.connected_component_count(),
            minimum_degree: self.minimum_degree(),
            maximum_degree: self.maximum_degree(),
            average_degree: self.average_degree(),
            undirected_density: self.undirected_density(),
            is_connected: self.is_connected(),
            diameter: self.diameter(),
            average_shortest_path:
                self.average_shortest_path(),
        }
    }

    // -------------------------------------------------------------------------
    // Reproducibility / fingerprinting
    // -------------------------------------------------------------------------

    /// Returns a deterministic canonical textual representation.
    ///
    /// This representation is intended for hashing/fingerprinting, not as the
    /// primary user-facing serialization format.
    pub fn canonical_representation(&self) -> String {
        let mut representation = String::new();

        representation.push_str(TOPOLOGY_SCHEMA_ID);
        representation.push(':');
        representation.push_str(
            &TOPOLOGY_SCHEMA_VERSION.to_string(),
        );

        representation.push('|');
        representation.push_str("resources=");
        representation.push_str(
            &self.resource_count.to_string(),
        );

        representation.push('|');
        representation.push_str("couplings=");

        for (index, coupling) in
            self.couplings.iter().enumerate()
        {
            if index != 0 {
                representation.push(';');
            }

            representation.push_str(
                &coupling.source.to_string(),
            );
            representation.push('-');
            representation.push_str(
                &coupling.target.to_string(),
            );
            representation.push(':');
            representation.push_str(
                coupling.connectivity.as_str(),
            );
        }

        representation
    }

    /// Returns a deterministic 64-bit topology fingerprint.
    ///
    /// This uses a small standard-library-only FNV-1a implementation so the
    /// benchmarking subsystem does not require a hashing dependency merely to
    /// identify a topology snapshot.
    ///
    /// This is an identity/fingerprint mechanism, not a cryptographic hash.
    pub fn fingerprint(&self) -> u64 {
        fnv1a_64(
            self.canonical_representation().as_bytes(),
        )
    }

    /// Returns the fingerprint formatted as lowercase hexadecimal.
    pub fn fingerprint_hex(&self) -> String {
        format!("{:016x}", self.fingerprint())
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates all topology invariants.
    pub fn validate(&self) -> Result<(), TopologyError> {
        if self.resource_count == 0 {
            return Err(TopologyError::ZeroResources);
        }

        if self.couplings.iter().any(|coupling| {
            coupling.source >= self.resource_count
                || coupling.target >= self.resource_count
        }) {
            return Err(TopologyError::InvalidTopology {
                message:
                    "coupling references a resource outside the topology"
                        .to_string(),
            });
        }

        for coupling in &self.couplings {
            if coupling.source == coupling.target {
                return Err(TopologyError::SelfCoupling {
                    resource: coupling.source,
                });
            }
        }

        for window in self.couplings.windows(2) {
            if window[0] == window[1] {
                return Err(
                    TopologyError::DuplicateCoupling {
                        source: window[0].source,
                        target: window[0].target,
                    },
                );
            }
        }

        self.validate_indexes()?;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Conversion from canonical hardware topology
    // -------------------------------------------------------------------------

    /// Creates a benchmarking snapshot from Zamani's canonical hardware
    /// topology.
    ///
    /// The canonical topology remains authoritative. This method copies only
    /// topology semantics into the benchmarking representation.
    ///
    /// This function is deliberately the only place in this file that knows
    /// about the canonical hardware topology module.
    pub fn from_hardware_topology(
        source: &crate::quantum::hardware::topology::HardwareTopology,
    ) -> Result<Self, TopologyError> {
        let resource_count = source.qubit_count();

        let couplings = source
            .couplings()
            .iter()
            .map(|coupling| {
                match coupling.connectivity {
                    crate::quantum::hardware::topology::Connectivity::Bidirectional => {
                        Coupling::bidirectional(
                            coupling.source,
                            coupling.target,
                        )
                    }

                    crate::quantum::hardware::topology::Connectivity::Directed => {
                        Coupling::directed(
                            coupling.source,
                            coupling.target,
                        )
                    }
                }
            })
            .collect::<Vec<_>>();

        Self::from_couplings(resource_count, couplings)
    }

    // -------------------------------------------------------------------------
    // Internal helpers
    // -------------------------------------------------------------------------

    fn validate_resource(
        &self,
        resource: ResourceId,
    ) -> Result<(), TopologyError> {
        if !self.contains_resource(resource) {
            return Err(TopologyError::InvalidResource {
                resource,
                resource_count: self.resource_count,
            });
        }

        Ok(())
    }

    fn has_native_connection(
        &self,
        source: ResourceId,
        target: ResourceId,
    ) -> bool {
        self.couplings.iter().any(|coupling| {
            coupling.permits_native_direction(
                source,
                target,
            )
        })
    }

    fn has_bidirectional_connection(
        &self,
        source: ResourceId,
        target: ResourceId,
    ) -> bool {
        self.couplings.iter().any(|coupling| {
            coupling.connectivity == Connectivity::Bidirectional
                && ((coupling.source == source
                    && coupling.target == target)
                    || (coupling.source == target
                        && coupling.target == source))
        })
    }

    fn rebuild_indexes(&mut self) {
        self.outgoing.clear();
        self.incoming.clear();
        self.undirected.clear();

        for resource in 0..self.resource_count {
            self.outgoing.insert(resource, Vec::new());
            self.incoming.insert(resource, Vec::new());
            self.undirected.insert(resource, Vec::new());
        }

        for coupling in &self.couplings {
            match coupling.connectivity {
                Connectivity::Bidirectional => {
                    self.outgoing
                        .get_mut(&coupling.source)
                        .expect(
                            "validated source must exist",
                        )
                        .push(coupling.target);

                    self.outgoing
                        .get_mut(&coupling.target)
                        .expect(
                            "validated target must exist",
                        )
                        .push(coupling.source);

                    self.incoming
                        .get_mut(&coupling.source)
                        .expect(
                            "validated source must exist",
                        )
                        .push(coupling.target);

                    self.incoming
                        .get_mut(&coupling.target)
                        .expect(
                            "validated target must exist",
                        )
                        .push(coupling.source);
                }

                Connectivity::Directed => {
                    self.outgoing
                        .get_mut(&coupling.source)
                        .expect(
                            "validated source must exist",
                        )
                        .push(coupling.target);

                    self.incoming
                        .get_mut(&coupling.target)
                        .expect(
                            "validated target must exist",
                        )
                        .push(coupling.source);
                }
            }

            self.undirected
                .get_mut(&coupling.source)
                .expect(
                    "validated source must exist",
                )
                .push(coupling.target);

            self.undirected
                .get_mut(&coupling.target)
                .expect(
                    "validated target must exist",
                )
                .push(coupling.source);
        }

        for neighbours in self.outgoing.values_mut() {
            neighbours.sort_unstable();
            neighbours.dedup();
        }

        for neighbours in self.incoming.values_mut() {
            neighbours.sort_unstable();
            neighbours.dedup();
        }

        for neighbours in self.undirected.values_mut() {
            neighbours.sort_unstable();
            neighbours.dedup();
        }
    }

    fn validate_indexes(&self) -> Result<(), TopologyError> {
        for resource in 0..self.resource_count {
            if !self.outgoing.contains_key(&resource) {
                return Err(
                    TopologyError::InvalidTopology {
                        message: format!(
                            "missing outgoing adjacency for resource {}",
                            resource
                        ),
                    },
                );
            }

            if !self.incoming.contains_key(&resource) {
                return Err(
                    TopologyError::InvalidTopology {
                        message: format!(
                            "missing incoming adjacency for resource {}",
                            resource
                        ),
                    },
                );
            }

            if !self.undirected.contains_key(&resource) {
                return Err(
                    TopologyError::InvalidTopology {
                        message: format!(
                            "missing undirected adjacency for resource {}",
                            resource
                        ),
                    },
                );
            }
        }

        Ok(())
    }
}

// =============================================================================
// Helper functions
// =============================================================================

fn reconstruct_path(
    source: ResourceId,
    target: ResourceId,
    predecessor: &BTreeMap<ResourceId, ResourceId>,
) -> Result<Vec<ResourceId>, TopologyError> {
    let mut path = vec![target];
    let mut cursor = target;

    while cursor != source {
        let previous = predecessor
            .get(&cursor)
            .copied()
            .ok_or(TopologyError::NoPath {
                source,
                target,
            })?;

        path.push(previous);
        cursor = previous;
    }

    path.reverse();

    Ok(path)
}

/// FNV-1a 64-bit hash.
///
/// This is intentionally implemented locally to keep the topology module
/// dependency-free. It is not intended for cryptographic security.
fn fnv1a_64(bytes: &[u8]) -> u64 {
    const OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x00000100000001b3;

    let mut hash = OFFSET_BASIS;

    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }

    hash
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_resources() {
        assert_eq!(
            HardwareTopology::new(0),
            Err(TopologyError::ZeroResources)
        );
    }

    #[test]
    fn creates_linear_topology() {
        let topology =
            HardwareTopology::linear(5).unwrap();

        assert_eq!(topology.resource_count(), 5);
        assert_eq!(topology.coupling_count(), 4);

        assert_eq!(
            topology.neighbours(0).unwrap(),
            &[1]
        );

        assert_eq!(
            topology.neighbours(2).unwrap(),
            &[1, 3]
        );
    }

    #[test]
    fn creates_ring_topology() {
        let topology =
            HardwareTopology::ring(5).unwrap();

        assert_eq!(topology.coupling_count(), 5);

        assert_eq!(
            topology.neighbours(0).unwrap(),
            &[1, 4]
        );
    }

    #[test]
    fn two_resource_ring_has_one_edge() {
        let topology =
            HardwareTopology::ring(2).unwrap();

        assert_eq!(topology.coupling_count(), 1);
    }

    #[test]
    fn creates_grid() {
        let topology =
            HardwareTopology::grid(2, 3).unwrap();

        assert_eq!(topology.resource_count(), 6);
        assert_eq!(topology.coupling_count(), 7);
    }

    #[test]
    fn fully_connected_has_expected_edge_count() {
        let topology =
            HardwareTopology::fully_connected(4)
                .unwrap();

        assert_eq!(topology.coupling_count(), 6);
        assert!(topology.is_connected());
        assert_eq!(topology.maximum_degree(), 3);
    }

    #[test]
    fn bidirectional_connection_works_both_directions() {
        let topology =
            HardwareTopology::from_couplings(
                2,
                [
                    Coupling::bidirectional(0, 1),
                ],
            )
            .unwrap();

        assert!(
            topology.is_connected(0, 1).unwrap()
        );

        assert!(
            topology.is_connected(1, 0).unwrap()
        );
    }

    #[test]
    fn directed_connection_only_works_forward() {
        let topology =
            HardwareTopology::from_couplings(
                2,
                [
                    Coupling::directed(0, 1),
                ],
            )
            .unwrap();

        assert!(
            topology.is_connected(0, 1).unwrap()
        );

        assert!(
            !topology.is_connected(1, 0).unwrap()
        );

        assert!(
            topology
                .is_physically_adjacent(1, 0)
                .unwrap()
        );
    }

    #[test]
    fn directed_shortest_path_respects_direction() {
        let topology =
            HardwareTopology::from_couplings(
                3,
                [
                    Coupling::directed(0, 1),
                    Coupling::directed(1, 2),
                ],
            )
            .unwrap();

        assert_eq!(
            topology
                .shortest_path(
                    0,
                    2,
                    PathMode::Directed,
                )
                .unwrap(),
            vec![0, 1, 2]
        );

        assert_eq!(
            topology.shortest_path(
                2,
                0,
                PathMode::Directed,
            ),
            Err(TopologyError::NoPath {
                source: 2,
                target: 0,
            })
        );
    }

    #[test]
    fn undirected_shortest_path_ignores_direction() {
        let topology =
            HardwareTopology::from_couplings(
                3,
                [
                    Coupling::directed(0, 1),
                    Coupling::directed(1, 2),
                ],
            )
            .unwrap();

        assert_eq!(
            topology
                .shortest_path(
                    2,
                    0,
                    PathMode::Undirected,
                )
                .unwrap(),
            vec![2, 1, 0]
        );
    }

    #[test]
    fn shortest_path_is_deterministic() {
        let topology =
            HardwareTopology::from_couplings(
                4,
                [
                    Coupling::bidirectional(0, 2),
                    Coupling::bidirectional(0, 1),
                    Coupling::bidirectional(1, 3),
                    Coupling::bidirectional(2, 3),
                ],
            )
            .unwrap();

        let path = topology
            .shortest_path(
                0,
                3,
                PathMode::Undirected,
            )
            .unwrap();

        // Both paths have equal length. Canonical adjacency ordering selects
        // the lexicographically deterministic path through resource 1.
        assert_eq!(path, vec![0, 1, 3]);
    }

    #[test]
    fn duplicate_coupling_is_rejected() {
        let result =
            HardwareTopology::from_couplings(
                2,
                [
                    Coupling::bidirectional(0, 1),
                    Coupling::bidirectional(0, 1),
                ],
            );

        assert_eq!(
            result,
            Err(TopologyError::DuplicateCoupling {
                source: 0,
                target: 1,
            })
        );
    }

    #[test]
    fn reverse_bidirectional_coupling_is_rejected() {
        let result =
            HardwareTopology::from_couplings(
                2,
                [
                    Coupling::bidirectional(0, 1),
                    Coupling::bidirectional(1, 0),
                ],
            );

        assert!(matches!(
            result,
            Err(TopologyError::DuplicateCoupling {
                ..
            })
        ));
    }

    #[test]
    fn self_coupling_is_rejected() {
        let result =
            HardwareTopology::from_couplings(
                2,
                [Coupling::bidirectional(0, 0)],
            );

        assert_eq!(
            result,
            Err(TopologyError::SelfCoupling {
                resource: 0,
            })
        );
    }

    #[test]
    fn invalid_resource_is_reported() {
        let topology =
            HardwareTopology::new(2).unwrap();

        assert_eq!(
            topology.degree(2),
            Err(TopologyError::InvalidResource {
                resource: 2,
                resource_count: 2,
            })
        );
    }

    #[test]
    fn disconnected_topology_reports_components() {
        let topology =
            HardwareTopology::from_couplings(
                4,
                [
                    Coupling::bidirectional(0, 1),
                    Coupling::bidirectional(2, 3),
                ],
            )
            .unwrap();

        assert_eq!(
            topology.connected_component_count(),
            2
        );

        assert!(!topology.is_connected());

        assert_eq!(
            topology.connected_components(),
            vec![
                vec![0, 1],
                vec![2, 3],
            ]
        );
    }

    #[test]
    fn isolated_resources_are_detected() {
        let topology =
            HardwareTopology::from_couplings(
                4,
                [
                    Coupling::bidirectional(0, 1),
                ],
            )
            .unwrap();

        assert_eq!(
            topology.isolated_resources(),
            vec![2, 3]
        );
    }

    #[test]
    fn degree_statistics_are_correct() {
        let topology =
            HardwareTopology::linear(5).unwrap();

        assert_eq!(topology.minimum_degree(), 1);
        assert_eq!(topology.maximum_degree(), 2);
        assert_eq!(
            topology.average_degree(),
            1.6
        );
    }

    #[test]
    fn density_is_correct() {
        let topology =
            HardwareTopology::linear(4).unwrap();

        // 3 actual edges / 6 possible edges.
        assert!(
            (topology.undirected_density() - 0.5)
                .abs()
                < f64::EPSILON
        );
    }

    #[test]
    fn diameter_is_correct() {
        let topology =
            HardwareTopology::linear(5).unwrap();

        assert_eq!(topology.diameter(), Some(4));
    }

    #[test]
    fn disconnected_topology_has_no_diameter() {
        let topology =
            HardwareTopology::from_couplings(
                3,
                [
                    Coupling::bidirectional(0, 1),
                ],
            )
            .unwrap();

        assert_eq!(topology.diameter(), None);
    }

    #[test]
    fn average_shortest_path_is_correct() {
        let topology =
            HardwareTopology::linear(3).unwrap();

        // Distances:
        // 0-1 = 1
        // 0-2 = 2
        // 1-2 = 1
        //
        // Average = 4/3.
        let average =
            topology.average_shortest_path().unwrap();

        assert!(
            (average - (4.0 / 3.0)).abs()
                < 1e-12
        );
    }

    #[test]
    fn statistics_are_consistent() {
        let topology =
            HardwareTopology::linear(4).unwrap();

        let statistics =
            topology.statistics();

        assert_eq!(
            statistics.resource_count,
            4
        );

        assert_eq!(
            statistics.coupling_count,
            3
        );

        assert_eq!(
            statistics.bidirectional_coupling_count,
            3
        );

        assert_eq!(
            statistics.directed_coupling_count,
            0
        );

        assert_eq!(
            statistics.connected_components,
            1
        );

        assert!(statistics.is_connected);
        assert_eq!(statistics.diameter, Some(3));
    }

    #[test]
    fn canonical_representation_is_deterministic() {
        let first =
            HardwareTopology::from_couplings(
                3,
                [
                    Coupling::bidirectional(1, 2),
                    Coupling::bidirectional(0, 1),
                ],
            )
            .unwrap();

        let second =
            HardwareTopology::from_couplings(
                3,
                [
                    Coupling::bidirectional(0, 1),
                    Coupling::bidirectional(1, 2),
                ],
            )
            .unwrap();

        assert_eq!(
            first.canonical_representation(),
            second.canonical_representation()
        );

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );

        assert_eq!(
            first.fingerprint_hex(),
            second.fingerprint_hex()
        );
    }

    #[test]
    fn different_topologies_have_different_fingerprints_in_fixture() {
        let line =
            HardwareTopology::linear(4).unwrap();

        let complete =
            HardwareTopology::fully_connected(4)
                .unwrap();

        assert_ne!(
            line.fingerprint(),
            complete.fingerprint()
        );
    }

    #[test]
    fn outgoing_and_incoming_indexes_are_correct() {
        let topology =
            HardwareTopology::from_couplings(
                3,
                [
                    Coupling::directed(0, 1),
                    Coupling::directed(2, 1),
                ],
            )
            .unwrap();

        assert_eq!(
            topology.outgoing_neighbours(0).unwrap(),
            &[1]
        );

        assert_eq!(
            topology.incoming_neighbours(1).unwrap(),
            &[0, 2]
        );
    }

    #[test]
    fn component_is_deterministic() {
        let topology =
            HardwareTopology::from_couplings(
                5,
                [
                    Coupling::bidirectional(3, 4),
                    Coupling::bidirectional(1, 2),
                    Coupling::bidirectional(0, 1),
                ],
            )
            .unwrap();

        assert_eq!(
            topology.component(2).unwrap(),
            vec![0, 1, 2]
        );

        assert_eq!(
            topology.component(4).unwrap(),
            vec![3, 4]
        );
    }

    #[test]
    fn highest_and_lowest_degree_resources_are_correct() {
        let topology =
            HardwareTopology::linear(5).unwrap();

        assert_eq!(
            topology.highest_degree_resources(),
            vec![1, 2, 3]
        );

        assert_eq!(
            topology.lowest_degree_resources(),
            vec![0, 4]
        );
    }

    #[test]
    fn grid_statistics_are_reasonable() {
        let topology =
            HardwareTopology::grid(3, 3).unwrap();

        assert_eq!(topology.resource_count(), 9);
        assert_eq!(topology.coupling_count(), 12);
        assert_eq!(topology.maximum_degree(), 4);
        assert!(topology.is_connected());
        assert_eq!(topology.diameter(), Some(4));
    }

    #[test]
    fn validation_succeeds_for_valid_topology() {
        let topology =
            HardwareTopology::ring(8).unwrap();

        assert!(topology.validate().is_ok());
    }

    #[test]
    fn schema_metadata_is_stable() {
        let topology =
            HardwareTopology::linear(2).unwrap();

        assert_eq!(
            topology.schema_version(),
            TOPOLOGY_SCHEMA_VERSION
        );

        assert_eq!(
            topology.schema_id(),
            TOPOLOGY_SCHEMA_ID
        );
    }
}