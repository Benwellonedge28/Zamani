//! Zamani Quantum — Canonical Hardware Topology
//!
//! Production-grade, deterministic representation of physical quantum
//! hardware connectivity.
//!
//! # Responsibility
//!
//! This module is the authoritative topology model for
//! `crate::quantum::hardware`.
//!
//! It answers:
//!
//! - How many physical quantum resources exist?
//! - Which resources are coupled?
//! - Which couplings are directed?
//! - Which operations may traverse a coupling natively?
//! - Are two resources directly connected?
//! - Is a route possible?
//! - What is the deterministic shortest route?
//! - How many connected components exist?
//! - What are the topology statistics?
//! - What stable fingerprint identifies this topology snapshot?
//!
//! # Explicit non-responsibilities
//!
//! This module does NOT:
//!
//! - execute quantum operations;
//! - perform network/device I/O;
//! - own provider credentials;
//! - own backend availability;
//! - own calibration values;
//! - perform gate decomposition;
//! - perform transpilation;
//! - schedule instructions;
//! - perform benchmark analysis;
//! - depend on `quantum::benchmarking`;
//! - depend on a provider such as IBM, IonQ, AWS, Rigetti, IQM, etc.
//!
//! # Architectural position
//!
//! ```text
//! Zamani Quantum IR
//!        |
//!        v
//! routing / transpilation
//!        |
//!        v
//! quantum::hardware::topology   <-- authoritative physical topology
//!        |
//!        +---- capabilities
//!        +---- instruction set
//!        +---- calibration
//!        +---- timing
//!        |
//!        v
//! backend / provider adapters
//! ```
//!
//! Benchmarking consumes this topology. This module never consumes
//! benchmarking.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1, Rust 2021.
//!
//! This implementation intentionally uses only the Rust standard library.
//!
//! # Determinism
//!
//! Production quantum compilation and benchmarking require deterministic
//! topology behaviour. Therefore:
//!
//! - resources are represented by stable integer identifiers;
//! - all observable adjacency lists are sorted;
//! - couplings are stored in canonical order;
//! - traversal order is deterministic;
//! - shortest-path tie-breaking is deterministic;
//! - statistics are independent of `HashMap` iteration order;
//! - topology fingerprints use canonical serialization.
//!
//! # Integration contract
//!
//! This file is independently complete.
//!
//! Downstream consumers may use:
//!
//! - `crate::quantum::hardware::backend`;
//! - `crate::quantum::routing`;
//! - `crate::quantum::scheduling`;
//! - `crate::quantum::benchmarking`;
//! - provider adapters;
//! - hardware discovery;
//! - compatibility validation.
//!
//! None of those modules are required to compile this topology model.
//!
//! The canonical hardware layer must remain lower-level than benchmarking,
//! provider adapters and execution.
//!
//! -----------------------------------------------------------------------------
//! Schema
//! -----------------------------------------------------------------------------
//
// A schema identifier is included so serialized/provenance systems can
// identify the semantic model even when the Rust type itself changes.

/// Stable schema version for the canonical hardware topology.
pub const TOPOLOGY_SCHEMA_VERSION: u16 = 1;

/// Stable schema identifier for the canonical hardware topology.
pub const TOPOLOGY_SCHEMA_ID: &str = "zamani.quantum.hardware.topology";

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};

// =============================================================================
// Resource identifiers
// =============================================================================

/// Generic physical quantum resource identifier.
///
/// For ordinary gate-model qubit hardware this is the physical qubit index.
///
/// The identifier is deliberately independent of a provider. A provider
/// adapter may translate its own device identifiers into this canonical
/// representation.
pub type ResourceId = usize;

/// Physical qubit identifier.
///
/// Kept as an alias for compatibility with the existing Zamani hardware API.
pub type QubitId = ResourceId;

// =============================================================================
// Connectivity
// =============================================================================

/// Directionality of a physical coupling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Connectivity {
    /// Native operations may traverse the coupling in either direction.
    Bidirectional,

    /// Native operations are available only from `source` to `target`.
    Directed,
}

impl Connectivity {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bidirectional => "bidirectional",
            Self::Directed => "directed",
        }
    }
}

// =============================================================================
// Coupling
// =============================================================================

/// A physical coupling between two quantum resources.
///
/// Calibration properties such as fidelity, error rate, duration, crosstalk
/// and frequency belong in the calibration subsystem and are deliberately not
/// embedded here.
///
/// A coupling is purely structural.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coupling {
    /// Native source resource.
    pub source: ResourceId,

    /// Native target resource.
    pub target: ResourceId,

    /// Directionality of the native connection.
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

    /// Returns whether the coupling touches a resource.
    pub const fn contains(self, resource: ResourceId) -> bool {
        self.source == resource || self.target == resource
    }

    /// Returns the opposite endpoint of an incident coupling.
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

    /// Returns whether this coupling permits the supplied native operation
    /// direction.
    pub const fn permits_native_direction(
        self,
        source: ResourceId,
        target: ResourceId,
    ) -> bool {
        match self.connectivity {
            Self::Bidirectional => {
                (self.source == source && self.target == target)
                    || (self.source == target && self.target == source)
            }

            Self::Directed => {
                self.source == source && self.target == target
            }
        }
    }

    /// Returns a canonical endpoint pair.
    ///
    /// The result is useful when comparing topology connectivity without
    /// considering direction.
    pub const fn canonical_pair(self) -> (ResourceId, ResourceId) {
        if self.source <= self.target {
            (self.source, self.target)
        } else {
            (self.target, self.source)
        }
    }
}

// =============================================================================
// Traversal semantics
// =============================================================================

/// Controls how topology traversal interprets directed couplings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PathMode {
    /// Respect native hardware direction.
    Directed,

    /// Treat physical couplings as undirected adjacency.
    ///
    /// This is appropriate for physical connectivity analysis and distance
    /// measurements. It must not be used as evidence that a directed gate is
    /// natively executable in the reverse direction.
    Undirected,
}

impl Default for PathMode {
    fn default() -> Self {
        Self::Directed
    }
}

impl PathMode {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Directed => "directed",
            Self::Undirected => "undirected",
        }
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by topology construction and analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyError {
    /// A topology cannot contain zero resources.
    ZeroQubits,

    /// Alias used by generic resource-oriented callers.
    ZeroResources,

    /// Resource does not exist in the topology.
    InvalidQubit {
        qubit: QubitId,
        qubit_count: usize,
    },

    /// Generic-resource equivalent of `InvalidQubit`.
    InvalidResource {
        resource: ResourceId,
        resource_count: usize,
    },

    /// A resource cannot be coupled to itself.
    SelfCoupling {
        qubit: QubitId,
    },

    /// A coupling already exists in a conflicting or duplicate form.
    DuplicateCoupling {
        source: QubitId,
        target: QubitId,
    },

    /// A requested coupling does not exist.
    MissingCoupling {
        source: QubitId,
        target: QubitId,
    },

    /// No path exists under the selected path semantics.
    NoPath {
        source: QubitId,
        target: QubitId,
    },

    /// The topology violates an internal invariant.
    InvalidTopology {
        message: String,
    },

    /// Numeric calculation cannot be represented safely.
    NumericOverflow {
        operation: &'static str,
    },
}

impl fmt::Display for TopologyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroQubits | Self::ZeroResources => {
                write!(
                    formatter,
                    "quantum hardware topology must contain at least one resource"
                )
            }

            Self::InvalidQubit {
                qubit,
                qubit_count,
            } => {
                write!(
                    formatter,
                    "qubit {} is outside topology containing {} qubits",
                    qubit,
                    qubit_count
                )
            }

            Self::InvalidResource {
                resource,
                resource_count,
            } => {
                write!(
                    formatter,
                    "resource {} is outside topology containing {} resources",
                    resource,
                    resource_count
                )
            }

            Self::SelfCoupling { qubit } => {
                write!(
                    formatter,
                    "qubit {} cannot be coupled to itself",
                    qubit
                )
            }

            Self::DuplicateCoupling { source, target } => {
                write!(
                    formatter,
                    "coupling between {} and {} already exists",
                    source,
                    target
                )
            }

            Self::MissingCoupling { source, target } => {
                write!(
                    formatter,
                    "no native coupling exists from {} to {}",
                    source,
                    target
                )
            }

            Self::NoPath { source, target } => {
                write!(
                    formatter,
                    "no topology path exists from {} to {}",
                    source,
                    target
                )
            }

            Self::InvalidTopology { message } => {
                write!(
                    formatter,
                    "invalid quantum hardware topology: {}",
                    message
                )
            }

            Self::NumericOverflow { operation } => {
                write!(
                    formatter,
                    "numeric overflow while calculating topology {}",
                    operation
                )
            }
        }
    }
}

impl std::error::Error for TopologyError {}

// =============================================================================
// Topology statistics
// =============================================================================

/// Deterministic structural statistics for a topology.
///
/// These statistics describe graph structure only. They do not represent
/// physical fidelity, error rate, calibration quality or execution speed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TopologyStatistics {
    /// Number of physical resources.
    pub resource_count: usize,

    /// Number of explicitly represented coupling records.
    pub coupling_count: usize,

    /// Number of directed coupling records.
    pub directed_coupling_count: usize,

    /// Number of bidirectional coupling records.
    pub bidirectional_coupling_count: usize,

    /// Number of resources with at least one physical neighbour.
    pub connected_resource_count: usize,

    /// Number of weakly connected components.
    pub connected_components: usize,

    /// Minimum undirected degree.
    pub minimum_degree: usize,

    /// Maximum undirected degree.
    pub maximum_degree: usize,

    /// Average undirected degree.
    pub average_degree: f64,

    /// Fraction of possible unordered resource pairs that are physically
    /// connected.
    ///
    /// Range: `0.0..=1.0`.
    pub undirected_density: f64,

    /// Whether every resource belongs to the same weakly connected component.
    pub is_connected: bool,

    /// Undirected graph diameter.
    ///
    /// `None` means the topology is disconnected.
    pub diameter: Option<usize>,

    /// Average shortest-path distance across all unordered reachable pairs.
    pub average_shortest_path: Option<f64>,
}

impl TopologyStatistics {
    /// Returns whether the topology is fully connected.
    pub const fn is_fully_connected(self) -> bool {
        self.is_connected
    }
}

// =============================================================================
// Hardware topology
// =============================================================================

/// Canonical physical quantum hardware topology.
///
/// # Invariants
///
/// A valid topology guarantees:
///
/// 1. `resource_count > 0`.
/// 2. Every coupling endpoint is in `0..resource_count`.
/// 3. No self-couplings exist.
/// 4. Exact duplicate coupling records do not exist.
/// 5. A bidirectional coupling cannot coexist with a directed coupling for
///    the same physical pair.
/// 6. Adjacency maps contain every resource.
/// 7. Adjacency lists are sorted and duplicate-free.
/// 8. Directed adjacency represents native outgoing connectivity.
/// 9. Undirected adjacency represents physical adjacency independent of native
///    gate direction.
/// 10. `couplings` is stored in canonical deterministic order.
///
/// # Mutation
///
/// The topology is mutable during construction/discovery. Provider adapters
/// should normally construct a complete topology and then expose it as an
/// immutable snapshot to downstream compilation/execution code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareTopology {
    resource_count: usize,

    /// Canonically ordered coupling records.
    couplings: Vec<Coupling>,

    /// Native outgoing adjacency.
    outgoing: BTreeMap<ResourceId, Vec<ResourceId>>,

    /// Native incoming adjacency.
    incoming: BTreeMap<ResourceId, Vec<ResourceId>>,

    /// Physical adjacency independent of native gate direction.
    undirected: BTreeMap<ResourceId, Vec<ResourceId>>,
}

impl HardwareTopology {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates a topology containing `resource_count` isolated resources.
    pub fn new(resource_count: usize) -> Result<Self, TopologyError> {
        if resource_count == 0 {
            return Err(TopologyError::ZeroQubits);
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

    /// Creates a topology from a complete coupling collection.
    ///
    /// Every coupling is validated before the topology is returned.
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

        topology.validate()?;

        Ok(topology)
    }

    /// Creates a fully connected bidirectional topology.
    pub fn fully_connected(
        resource_count: usize,
    ) -> Result<Self, TopologyError> {
        let mut topology = Self::new(resource_count)?;

        for source in 0..resource_count {
            for target in (source + 1)..resource_count {
                topology.add_bidirectional_coupling(source, target)?;
            }
        }

        Ok(topology)
    }

    /// Creates a bidirectional linear nearest-neighbour topology.
    pub fn linear(resource_count: usize) -> Result<Self, TopologyError> {
        let mut topology = Self::new(resource_count)?;

        if resource_count > 1 {
            for source in 0..(resource_count - 1) {
                topology.add_bidirectional_coupling(
                    source,
                    source + 1,
                )?;
            }
        }

        Ok(topology)
    }

    /// Creates a bidirectional ring topology.
    ///
    /// For one resource the topology contains no coupling.
    ///
    /// For two resources there is exactly one physical pair rather than two
    /// duplicate edges.
    pub fn ring(resource_count: usize) -> Result<Self, TopologyError> {
        let mut topology = Self::linear(resource_count)?;

        if resource_count > 2 {
            topology.add_bidirectional_coupling(
                resource_count - 1,
                0,
            )?;
        }

        Ok(topology)
    }

    // =========================================================================
    // Basic properties
    // =========================================================================

    /// Returns the number of physical resources.
    pub const fn qubit_count(&self) -> usize {
        self.resource_count
    }

    /// Generic alias for `qubit_count`.
    pub const fn resource_count(&self) -> usize {
        self.resource_count
    }

    /// Returns the number of coupling records.
    pub const fn coupling_count(&self) -> usize {
        self.couplings.len()
    }

    /// Returns true when there are no coupling records.
    pub fn is_empty(&self) -> bool {
        self.couplings.is_empty()
    }

    /// Returns all coupling records in canonical deterministic order.
    pub fn couplings(&self) -> &[Coupling] {
        &self.couplings
    }

    /// Returns whether a physical resource exists.
    pub const fn contains(&self, resource: ResourceId) -> bool {
        resource < self.resource_count
    }

    /// Returns the complete ordered set of physical resources.
    pub fn resources(&self) -> impl Iterator<Item = ResourceId> {
        0..self.resource_count
    }

    // =========================================================================
    // Coupling construction
    // =========================================================================

    /// Adds a coupling while enforcing all topology invariants.
    pub fn add_coupling(
        &mut self,
        coupling: Coupling,
    ) -> Result<(), TopologyError> {
        self.validate_qubit(coupling.source)?;
        self.validate_qubit(coupling.target)?;

        if coupling.source == coupling.target {
            return Err(TopologyError::SelfCoupling {
                qubit: coupling.source,
            });
        }

        if self.coupling_conflicts(coupling) {
            return Err(TopologyError::DuplicateCoupling {
                source: coupling.source,
                target: coupling.target,
            });
        }

        self.couplings.push(coupling);

        self.couplings.sort_unstable();

        match coupling.connectivity {
            Connectivity::Directed => {
                self.outgoing
                    .get_mut(&coupling.source)
                    .expect("validated source must exist")
                    .push(coupling.target);

                self.incoming
                    .get_mut(&coupling.target)
                    .expect("validated target must exist")
                    .push(coupling.source);

                self.undirected
                    .get_mut(&coupling.source)
                    .expect("validated source must exist")
                    .push(coupling.target);

                self.undirected
                    .get_mut(&coupling.target)
                    .expect("validated target must exist")
                    .push(coupling.source);
            }

            Connectivity::Bidirectional => {
                self.outgoing
                    .get_mut(&coupling.source)
                    .expect("validated source must exist")
                    .push(coupling.target);

                self.outgoing
                    .get_mut(&coupling.target)
                    .expect("validated target must exist")
                    .push(coupling.source);

                self.incoming
                    .get_mut(&coupling.source)
                    .expect("validated source must exist")
                    .push(coupling.target);

                self.incoming
                    .get_mut(&coupling.target)
                    .expect("validated target must exist")
                    .push(coupling.source);

                self.undirected
                    .get_mut(&coupling.source)
                    .expect("validated source must exist")
                    .push(coupling.target);

                self.undirected
                    .get_mut(&coupling.target)
                    .expect("validated target must exist")
                    .push(coupling.source);
            }
        }

        self.sort_adjacency();

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Adds bidirectional connectivity.
    pub fn add_bidirectional_coupling(
        &mut self,
        source: QubitId,
        target: QubitId,
    ) -> Result<(), TopologyError> {
        self.add_coupling(Coupling::bidirectional(source, target))
    }

    /// Adds directed native connectivity.
    pub fn add_directed_coupling(
        &mut self,
        source: QubitId,
        target: QubitId,
    ) -> Result<(), TopologyError> {
        self.add_coupling(Coupling::directed(source, target))
    }

    // =========================================================================
    // Connectivity queries
    // =========================================================================

    /// Returns native neighbours reachable from `qubit`.
    ///
    /// Directed couplings are respected.
    pub fn neighbours(
        &self,
        qubit: QubitId,
    ) -> Result<&[QubitId], TopologyError> {
        self.validate_qubit(qubit)?;

        Ok(self
            .outgoing
            .get(&qubit)
            .map(Vec::as_slice)
            .unwrap_or(&[]))
    }

    /// Returns incoming native neighbours.
    pub fn incoming_neighbours(
        &self,
        qubit: QubitId,
    ) -> Result<&[QubitId], TopologyError> {
        self.validate_qubit(qubit)?;

        Ok(self
            .incoming
            .get(&qubit)
            .map(Vec::as_slice)
            .unwrap_or(&[]))
    }

    /// Returns physical neighbours regardless of native gate direction.
    pub fn physical_neighbours(
        &self,
        qubit: QubitId,
    ) -> Result<&[QubitId], TopologyError> {
        self.validate_qubit(qubit)?;

        Ok(self
            .undirected
            .get(&qubit)
            .map(Vec::as_slice)
            .unwrap_or(&[]))
    }

    /// Returns whether a native operation can execute from `source` to
    /// `target`.
    ///
    /// This method respects directed couplings.
    pub fn is_connected(
        &self,
        source: QubitId,
        target: QubitId,
    ) -> Result<bool, TopologyError> {
        self.validate_qubit(source)?;
        self.validate_qubit(target)?;

        Ok(self
            .outgoing
            .get(&source)
            .map(|neighbours| {
                neighbours.binary_search(&target).is_ok()
            })
            .unwrap_or(false))
    }

    /// Returns whether the two resources are physically adjacent, ignoring
    /// native operation direction.
    pub fn is_physically_adjacent(
        &self,
        source: QubitId,
        target: QubitId,
    ) -> Result<bool, TopologyError> {
        self.validate_qubit(source)?;
        self.validate_qubit(target)?;

        Ok(self
            .undirected
            .get(&source)
            .map(|neighbours| {
                neighbours.binary_search(&target).is_ok()
            })
            .unwrap_or(false))
    }

    /// Returns the coupling that permits the supplied native direction.
    pub fn coupling(
        &self,
        source: QubitId,
        target: QubitId,
    ) -> Result<Option<Coupling>, TopologyError> {
        self.validate_qubit(source)?;
        self.validate_qubit(target)?;

        Ok(self
            .couplings
            .iter()
            .copied()
            .find(|coupling| {
                coupling.permits_native_direction(source, target)
            }))
    }

    /// Returns whether a physical pair has any coupling.
    pub fn has_physical_connection(
        &self,
        source: QubitId,
        target: QubitId,
    ) -> Result<bool, TopologyError> {
        self.is_physically_adjacent(source, target)
    }

    // =========================================================================
    // Paths
    // =========================================================================

    /// Finds the deterministic shortest path using native hardware
    /// directionality.
    pub fn shortest_path(
        &self,
        source: QubitId,
        target: QubitId,
    ) -> Result<Vec<QubitId>, TopologyError> {
        self.shortest_path_with_mode(
            source,
            target,
            PathMode::Directed,
        )
    }

    /// Finds the deterministic shortest path under an explicit traversal mode.
    pub fn shortest_path_with_mode(
        &self,
        source: QubitId,
        target: QubitId,
        mode: PathMode,
    ) -> Result<Vec<QubitId>, TopologyError> {
        self.validate_qubit(source)?;
        self.validate_qubit(target)?;

        if source == target {
            return Ok(vec![source]);
        }

        let mut queue = VecDeque::new();
        let mut visited = BTreeSet::new();
        let mut predecessor = BTreeMap::<QubitId, QubitId>::new();

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

    /// Returns the shortest native routing distance.
    pub fn distance(
        &self,
        source: QubitId,
        target: QubitId,
    ) -> Result<usize, TopologyError> {
        self.distance_with_mode(
            source,
            target,
            PathMode::Directed,
        )
    }

    /// Returns shortest-path distance under explicit traversal semantics.
    pub fn distance_with_mode(
        &self,
        source: QubitId,
        target: QubitId,
        mode: PathMode,
    ) -> Result<usize, TopologyError> {
        let path =
            self.shortest_path_with_mode(source, target, mode)?;

        path.len()
            .checked_sub(1)
            .ok_or(TopologyError::NumericOverflow {
                operation: "path distance",
            })
    }

    /// Returns true when every resource can reach every other resource under
    /// native directed traversal.
    pub fn is_strongly_connected(&self) -> bool {
        if self.resource_count == 0 {
            return false;
        }

        for source in self.resources() {
            for target in self.resources() {
                if source == target {
                    continue;
                }

                if self
                    .shortest_path_with_mode(
                        source,
                        target,
                        PathMode::Directed,
                    )
                    .is_err()
                {
                    return false;
                }
            }
        }

        true
    }

    /// Returns true when the physical topology is connected when native
    /// direction is ignored.
    pub fn is_fully_connected(&self) -> bool {
        if self.resource_count == 0 {
            return false;
        }

        for source in self.resources() {
            for target in self.resources() {
                if source == target {
                    continue;
                }

                if self
                    .shortest_path_with_mode(
                        source,
                        target,
                        PathMode::Undirected,
                    )
                    .is_err()
                {
                    return false;
                }
            }
        }

        true
    }

    // =========================================================================
    // Degree / graph analysis
    // =========================================================================

    /// Returns the undirected degree of one resource.
    pub fn degree(
        &self,
        qubit: QubitId,
    ) -> Result<usize, TopologyError> {
        self.validate_qubit(qubit)?;

        Ok(self
            .undirected
            .get(&qubit)
            .map(Vec::len)
            .unwrap_or(0))
    }

    /// Returns the native outgoing degree.
    pub fn out_degree(
        &self,
        qubit: QubitId,
    ) -> Result<usize, TopologyError> {
        self.validate_qubit(qubit)?;

        Ok(self
            .outgoing
            .get(&qubit)
            .map(Vec::len)
            .unwrap_or(0))
    }

    /// Returns the native incoming degree.
    pub fn in_degree(
        &self,
        qubit: QubitId,
    ) -> Result<usize, TopologyError> {
        self.validate_qubit(qubit)?;

        Ok(self
            .incoming
            .get(&qubit)
            .map(Vec::len)
            .unwrap_or(0))
    }

    /// Returns the maximum undirected degree.
    pub fn maximum_degree(&self) -> usize {
        self.undirected
            .values()
            .map(Vec::len)
            .max()
            .unwrap_or(0)
    }

    /// Returns the minimum undirected degree.
    pub fn minimum_degree(&self) -> usize {
        self.undirected
            .values()
            .map(Vec::len)
            .min()
            .unwrap_or(0)
    }

    /// Returns average undirected degree.
    pub fn average_degree(&self) -> f64 {
        if self.resource_count == 0 {
            return 0.0;
        }

        let total: usize =
            self.undirected.values().map(Vec::len).sum();

        total as f64 / self.resource_count as f64
    }

    /// Counts weakly connected components.
    pub fn connected_components(&self) -> usize {
        if self.resource_count == 0 {
            return 0;
        }

        let mut visited = BTreeSet::new();
        let mut components = 0usize;

        for start in self.resources() {
            if visited.contains(&start) {
                continue;
            }

            components += 1;
            let mut queue = VecDeque::new();

            queue.push_back(start);
            visited.insert(start);

            while let Some(current) = queue.pop_front() {
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
        }

        components
    }

    /// Returns all structural statistics.
    pub fn statistics(&self) -> TopologyStatistics {
        let coupling_count = self.couplings.len();

        let directed_coupling_count = self
            .couplings
            .iter()
            .filter(|coupling| {
                coupling.connectivity == Connectivity::Directed
            })
            .count();

        let bidirectional_coupling_count = coupling_count
            .saturating_sub(directed_coupling_count);

        let connected_resource_count = self
            .undirected
            .values()
            .filter(|neighbours| !neighbours.is_empty())
            .count();

        let connected_components = self.connected_components();

        let minimum_degree = self.minimum_degree();
        let maximum_degree = self.maximum_degree();
        let average_degree = self.average_degree();

        let possible_pairs =
            self.resource_count
                .saturating_mul(self.resource_count.saturating_sub(1))
                / 2;

        let undirected_pairs = self
            .couplings
            .iter()
            .map(|coupling| coupling.canonical_pair())
            .collect::<BTreeSet<_>>()
            .len();

        let undirected_density = if possible_pairs == 0 {
            0.0
        } else {
            undirected_pairs as f64 / possible_pairs as f64
        };

        let is_connected =
            self.resource_count > 0 && connected_components == 1;

        let (diameter, average_shortest_path) =
            self.undirected_distance_statistics();

        TopologyStatistics {
            resource_count: self.resource_count,
            coupling_count,
            directed_coupling_count,
            bidirectional_coupling_count,
            connected_resource_count,
            connected_components,
            minimum_degree,
            maximum_degree,
            average_degree,
            undirected_density,
            is_connected,
            diameter,
            average_shortest_path,
        }
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates all topology invariants.
    ///
    /// This method is intentionally public so backend discovery and provider
    /// adapters can validate externally supplied topology snapshots before
    /// accepting them.
    pub fn validate(&self) -> Result<(), TopologyError> {
        if self.resource_count == 0 {
            return Err(TopologyError::ZeroQubits);
        }

        if self.outgoing.len() != self.resource_count
            || self.incoming.len() != self.resource_count
            || self.undirected.len() != self.resource_count
        {
            return Err(TopologyError::InvalidTopology {
                message:
                    "adjacency maps do not contain every physical resource"
                        .to_string(),
            });
        }

        for resource in self.resources() {
            if !self.outgoing.contains_key(&resource)
                || !self.incoming.contains_key(&resource)
                || !self.undirected.contains_key(&resource)
            {
                return Err(TopologyError::InvalidTopology {
                    message: format!(
                        "resource {} is missing from an adjacency map",
                        resource
                    ),
                });
            }
        }

        for window in self.couplings.windows(2) {
            if window[0] >= window[1] {
                return Err(TopologyError::InvalidTopology {
                    message:
                        "coupling collection is not strictly ordered"
                            .to_string(),
                });
            }
        }

        for coupling in &self.couplings {
            self.validate_qubit(coupling.source)?;
            self.validate_qubit(coupling.target)?;

            if coupling.source == coupling.target {
                return Err(TopologyError::SelfCoupling {
                    qubit: coupling.source,
                });
            }

            if self.coupling_conflicts(*coupling)
                && !self.couplings.contains(coupling)
            {
                return Err(TopologyError::InvalidTopology {
                    message: format!(
                        "coupling conflict involving {} -> {}",
                        coupling.source,
                        coupling.target
                    ),
                });
            }
        }

        self.validate_adjacency_consistency()?;

        Ok(())
    }

    // =========================================================================
    // Deterministic identity
    // =========================================================================

    /// Returns a deterministic topology fingerprint.
    ///
    /// The fingerprint is based only on:
    ///
    /// - schema identifier;
    /// - schema version;
    /// - resource count;
    /// - canonical coupling list.
    ///
    /// Calibration, provider state, backend availability and execution
    /// metadata are intentionally excluded.
    ///
    /// This is suitable for provenance, caching and reproducibility.
    pub fn fingerprint(&self) -> String {
        let mut hasher = StableHasher::default();

        TOPOLOGY_SCHEMA_ID.hash(&mut hasher);
        TOPOLOGY_SCHEMA_VERSION.hash(&mut hasher);
        self.resource_count.hash(&mut hasher);

        for coupling in &self.couplings {
            coupling.source.hash(&mut hasher);
            coupling.target.hash(&mut hasher);
            coupling.connectivity.hash(&mut hasher);
        }

        format!("{:016x}", hasher.finish())
    }

    // =========================================================================
    // Internal implementation
    // =========================================================================

    fn validate_qubit(
        &self,
        qubit: QubitId,
    ) -> Result<(), TopologyError> {
        if qubit >= self.resource_count {
            return Err(TopologyError::InvalidQubit {
                qubit,
                qubit_count: self.resource_count,
            });
        }

        Ok(())
    }

    fn coupling_conflicts(&self, coupling: Coupling) -> bool {
        self.couplings.iter().any(|existing| {
            if existing == &coupling {
                return true;
            }

            let same_direction =
                existing.source == coupling.source
                    && existing.target == coupling.target;

            let reverse_direction =
                existing.source == coupling.target
                    && existing.target == coupling.source;

            match (existing.connectivity, coupling.connectivity) {
                (Connectivity::Bidirectional, _) => {
                    same_direction || reverse_direction
                }

                (_, Connectivity::Bidirectional) => {
                    same_direction || reverse_direction
                }

                (Connectivity::Directed, Connectivity::Directed) => {
                    same_direction
                }
            }
        })
    }

    fn sort_adjacency(&mut self) {
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

    fn validate_adjacency_consistency(
        &self,
    ) -> Result<(), TopologyError> {
        for resource in self.resources() {
            let outgoing = self
                .outgoing
                .get(&resource)
                .expect("resource must have outgoing adjacency");

            let incoming = self
                .incoming
                .get(&resource)
                .expect("resource must have incoming adjacency");

            let undirected = self
                .undirected
                .get(&resource)
                .expect("resource must have undirected adjacency");

            if !is_sorted_unique(outgoing)
                || !is_sorted_unique(incoming)
                || !is_sorted_unique(undirected)
            {
                return Err(TopologyError::InvalidTopology {
                    message: format!(
                        "adjacency for resource {} is not sorted and unique",
                        resource
                    ),
                });
            }

            for &target in outgoing {
                if !self.contains(target) {
                    return Err(TopologyError::InvalidTopology {
                        message: format!(
                            "outgoing adjacency {} -> {} references a \
                             missing resource",
                            resource, target
                        ),
                    });
                }

                let reverse_present = incoming
                    .get(&target)
                    .map(|items| {
                        items.binary_search(&resource).is_ok()
                    })
                    .unwrap_or(false);

                if !reverse_present {
                    return Err(TopologyError::InvalidTopology {
                        message: format!(
                            "outgoing/incoming adjacency mismatch for \
                             {} -> {}",
                            resource, target
                        ),
                    });
                }
            }

            for &source in incoming {
                if !self.contains(source) {
                    return Err(TopologyError::InvalidTopology {
                        message: format!(
                            "incoming adjacency {} <- {} references a \
                             missing resource",
                            resource, source
                        ),
                    });
                }

                let reverse_present = outgoing
                    .get(&source)
                    .map(|items| {
                        items.binary_search(&resource).is_ok()
                    })
                    .unwrap_or(false);

                if !reverse_present {
                    return Err(TopologyError::InvalidTopology {
                        message: format!(
                            "incoming/outgoing adjacency mismatch for \
                             {} <- {}",
                            resource, source
                        ),
                    });
                }
            }

            for &neighbour in undirected {
                if !self.contains(neighbour) {
                    return Err(TopologyError::InvalidTopology {
                        message: format!(
                            "undirected adjacency {} -- {} references a \
                             missing resource",
                            resource, neighbour
                        ),
                    });
                }

                let reverse_present = self
                    .undirected
                    .get(&neighbour)
                    .map(|items| {
                        items.binary_search(&resource).is_ok()
                    })
                    .unwrap_or(false);

                if !reverse_present {
                    return Err(TopologyError::InvalidTopology {
                        message: format!(
                            "undirected adjacency mismatch for {} -- {}",
                            resource, neighbour
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    fn undirected_distance_statistics(
        &self,
    ) -> (Option<usize>, Option<f64>) {
        if self.resource_count < 2 {
            return (Some(0), None);
        }

        let mut diameter = 0usize;
        let mut distance_sum = 0usize;
        let mut pair_count = 0usize;

        for source in self.resources() {
            let distances =
                self.bfs_distances(source, PathMode::Undirected);

            for target in (source + 1)..self.resource_count {
                let Some(distance) = distances.get(&target).copied()
                else {
                    return (None, None);
                };

                diameter = diameter.max(distance);
                distance_sum =
                    distance_sum.saturating_add(distance);
                pair_count = pair_count.saturating_add(1);
            }
        }

        if pair_count == 0 {
            return (Some(diameter), None);
        }

        (
            Some(diameter),
            Some(distance_sum as f64 / pair_count as f64),
        )
    }

    fn bfs_distances(
        &self,
        source: QubitId,
        mode: PathMode,
    ) -> BTreeMap<QubitId, usize> {
        let mut distances = BTreeMap::new();
        let mut queue = VecDeque::new();

        distances.insert(source, 0);
        queue.push_back(source);

        while let Some(current) = queue.pop_front() {
            let current_distance =
                *distances.get(&current).unwrap_or(&0);

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
}

// =============================================================================
// Default
// =============================================================================

impl Default for HardwareTopology {
    fn default() -> Self {
        Self::new(1)
            .expect("one-resource hardware topology is always valid")
    }
}

// =============================================================================
// Stable hasher
// =============================================================================

/// Small deterministic FNV-1a hasher used only for topology fingerprints.
///
/// This is NOT intended as a cryptographic hash.
///
/// It exists to provide a dependency-free, stable identifier for topology
/// provenance and cache keys.
#[derive(Debug, Clone)]
struct StableHasher {
    state: u64,
}

impl Default for StableHasher {
    fn default() -> Self {
        Self {
            state: 0xcbf29ce484222325,
        }
    }
}

impl Hasher for StableHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        const FNV_PRIME: u64 = 0x00000100000001B3;

        for byte in bytes {
            self.state ^= u64::from(*byte);
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }
}

// =============================================================================
// Helpers
// =============================================================================

fn reconstruct_path(
    source: QubitId,
    target: QubitId,
    predecessor: &BTreeMap<QubitId, QubitId>,
) -> Result<Vec<QubitId>, TopologyError> {
    let mut path = vec![target];
    let mut cursor = target;

    while cursor != source {
        let Some(previous) = predecessor.get(&cursor).copied()
        else {
            return Err(TopologyError::NoPath { source, target });
        };

        cursor = previous;
        path.push(cursor);
    }

    path.reverse();

    Ok(path)
}

fn is_sorted_unique(values: &[QubitId]) -> bool {
    values
        .windows(2)
        .all(|window| window[0] < window[1])
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    #[test]
    fn rejects_zero_resources() {
        assert_eq!(
            HardwareTopology::new(0),
            Err(TopologyError::ZeroQubits)
        );
    }

    #[test]
    fn creates_single_resource_topology() {
        let topology = HardwareTopology::new(1).unwrap();

        assert_eq!(topology.qubit_count(), 1);
        assert_eq!(topology.resource_count(), 1);
        assert_eq!(topology.coupling_count(), 0);
        assert!(topology.validate().is_ok());
    }

    #[test]
    fn linear_topology_has_expected_edges() {
        let topology = HardwareTopology::linear(5).unwrap();

        assert_eq!(topology.qubit_count(), 5);
        assert_eq!(topology.coupling_count(), 4);

        assert_eq!(
            topology.neighbours(0).unwrap(),
            &[1]
        );

        assert_eq!(
            topology.neighbours(4).unwrap(),
            &[3]
        );
    }

    #[test]
    fn ring_of_two_does_not_create_duplicate_edge() {
        let topology = HardwareTopology::ring(2).unwrap();

        assert_eq!(topology.qubit_count(), 2);
        assert_eq!(topology.coupling_count(), 1);
        assert!(topology.validate().is_ok());
    }

    #[test]
    fn ring_of_three_closes_the_cycle() {
        let topology = HardwareTopology::ring(3).unwrap();

        assert_eq!(topology.coupling_count(), 3);

        assert_eq!(
            topology.physical_neighbours(0).unwrap(),
            &[1, 2]
        );
    }

    #[test]
    fn fully_connected_topology_has_expected_edge_count() {
        let topology =
            HardwareTopology::fully_connected(5).unwrap();

        assert_eq!(topology.coupling_count(), 10);
        assert!(topology.is_fully_connected());
    }

    // -------------------------------------------------------------------------
    // Coupling validation
    // -------------------------------------------------------------------------

    #[test]
    fn rejects_self_coupling() {
        let mut topology = HardwareTopology::new(2).unwrap();

        assert_eq!(
            topology.add_bidirectional_coupling(0, 0),
            Err(TopologyError::SelfCoupling { qubit: 0 })
        );
    }

    #[test]
    fn rejects_duplicate_bidirectional_coupling() {
        let mut topology = HardwareTopology::new(2).unwrap();

        topology
            .add_bidirectional_coupling(0, 1)
            .unwrap();

        assert_eq!(
            topology.add_bidirectional_coupling(0, 1),
            Err(TopologyError::DuplicateCoupling {
                source: 0,
                target: 1
            })
        );
    }

    #[test]
    fn rejects_reverse_duplicate_of_bidirectional_coupling() {
        let mut topology = HardwareTopology::new(2).unwrap();

        topology
            .add_bidirectional_coupling(0, 1)
            .unwrap();

        assert_eq!(
            topology.add_bidirectional_coupling(1, 0),
            Err(TopologyError::DuplicateCoupling {
                source: 1,
                target: 0
            })
        );
    }

    #[test]
    fn rejects_bidirectional_after_directed_coupling() {
        let mut topology = HardwareTopology::new(2).unwrap();

        topology
            .add_directed_coupling(0, 1)
            .unwrap();

        assert_eq!(
            topology.add_bidirectional_coupling(0, 1),
            Err(TopologyError::DuplicateCoupling {
                source: 0,
                target: 1
            })
        );
    }

    #[test]
    fn rejects_bidirectional_after_reverse_directed_coupling() {
        let mut topology = HardwareTopology::new(2).unwrap();

        topology
            .add_directed_coupling(1, 0)
            .unwrap();

        assert_eq!(
            topology.add_bidirectional_coupling(0, 1),
            Err(TopologyError::DuplicateCoupling {
                source: 0,
                target: 1
            })
        );
    }

    #[test]
    fn allows_both_directions_as_separate_directed_couplings() {
        let mut topology = HardwareTopology::new(2).unwrap();

        topology
            .add_directed_coupling(0, 1)
            .unwrap();

        topology
            .add_directed_coupling(1, 0)
            .unwrap();

        assert_eq!(topology.coupling_count(), 2);
        assert!(topology.is_connected(0, 1).unwrap());
        assert!(topology.is_connected(1, 0).unwrap());
    }

    // -------------------------------------------------------------------------
    // Directed semantics
    // -------------------------------------------------------------------------

    #[test]
    fn bidirectional_connection_works_both_ways() {
        let mut topology = HardwareTopology::new(2).unwrap();

        topology
            .add_bidirectional_coupling(0, 1)
            .unwrap();

        assert!(topology.is_connected(0, 1).unwrap());
        assert!(topology.is_connected(1, 0).unwrap());
    }

    #[test]
    fn directed_connection_only_works_forward() {
        let mut topology = HardwareTopology::new(2).unwrap();

        topology
            .add_directed_coupling(0, 1)
            .unwrap();

        assert!(topology.is_connected(0, 1).unwrap());
        assert!(!topology.is_connected(1, 0).unwrap());

        assert_eq!(
            topology.neighbours(0).unwrap(),
            &[1]
        );

        assert_eq!(
            topology.neighbours(1).unwrap(),
            &[]
        );
    }

    #[test]
    fn directed_reverse_is_still_physical_adjacency() {
        let mut topology = HardwareTopology::new(2).unwrap();

        topology
            .add_directed_coupling(0, 1)
            .unwrap();

        assert!(
            topology
                .is_physically_adjacent(1, 0)
                .unwrap()
        );

        assert_eq!(
            topology.physical_neighbours(1).unwrap(),
            &[0]
        );
    }

    // -------------------------------------------------------------------------
    // Paths
    // -------------------------------------------------------------------------

    #[test]
    fn shortest_path_on_linear_topology_is_deterministic() {
        let topology = HardwareTopology::linear(5).unwrap();

        assert_eq!(
            topology.shortest_path(0, 4).unwrap(),
            vec![0, 1, 2, 3, 4]
        );

        assert_eq!(topology.distance(0, 4).unwrap(), 4);
    }

    #[test]
    fn source_equals_target_has_zero_distance() {
        let topology = HardwareTopology::linear(5).unwrap();

        assert_eq!(
            topology.shortest_path(2, 2).unwrap(),
            vec![2]
        );

        assert_eq!(topology.distance(2, 2).unwrap(), 0);
    }

    #[test]
    fn disconnected_topology_reports_no_path() {
        let topology = HardwareTopology::new(3).unwrap();

        assert_eq!(
            topology.shortest_path(0, 2),
            Err(TopologyError::NoPath {
                source: 0,
                target: 2
            })
        );
    }

    #[test]
    fn directed_path_respects_direction() {
        let topology = HardwareTopology::from_couplings(
            3,
            [
                Coupling::directed(0, 1),
                Coupling::directed(1, 2),
            ],
        )
        .unwrap();

        assert_eq!(
            topology
                .shortest_path_with_mode(
                    0,
                    2,
                    PathMode::Directed
                )
                .unwrap(),
            vec![0, 1, 2]
        );

        assert_eq!(
            topology.shortest_path_with_mode(
                2,
                0,
                PathMode::Directed
            ),
            Err(TopologyError::NoPath {
                source: 2,
                target: 0
            })
        );
    }

    #[test]
    fn undirected_path_ignores_native_direction_for_physical_distance() {
        let topology = HardwareTopology::from_couplings(
            3,
            [
                Coupling::directed(0, 1),
                Coupling::directed(1, 2),
            ],
        )
        .unwrap();

        assert_eq!(
            topology
                .shortest_path_with_mode(
                    2,
                    0,
                    PathMode::Undirected
                )
                .unwrap(),
            vec![2, 1, 0]
        );
    }

    // -------------------------------------------------------------------------
    // Connectivity
    // -------------------------------------------------------------------------

    #[test]
    fn strongly_connected_requires_directed_reachability() {
        let topology = HardwareTopology::from_couplings(
            3,
            [
                Coupling::directed(0, 1),
                Coupling::directed(1, 2),
            ],
        )
        .unwrap();

        assert!(!topology.is_strongly_connected());
    }

    #[test]
    fn bidirectional_ring_is_strongly_connected() {
        let topology = HardwareTopology::ring(5).unwrap();

        assert!(topology.is_strongly_connected());
    }

    #[test]
    fn physical_connectivity_can_differ_from_native_connectivity() {
        let topology = HardwareTopology::from_couplings(
            2,
            [Coupling::directed(0, 1)],
        )
        .unwrap();

        assert!(!topology.is_strongly_connected());
        assert!(topology.is_fully_connected());
    }

    // -------------------------------------------------------------------------
    // Degrees
    // -------------------------------------------------------------------------

    #[test]
    fn degrees_are_correct() {
        let topology = HardwareTopology::linear(5).unwrap();

        assert_eq!(topology.degree(0).unwrap(), 1);
        assert_eq!(topology.degree(2).unwrap(), 2);
        assert_eq!(topology.degree(4).unwrap(), 1);

        assert_eq!(topology.maximum_degree(), 2);
        assert_eq!(topology.minimum_degree(), 1);
        assert_eq!(topology.average_degree(), 1.6);
    }

    // -------------------------------------------------------------------------
    // Statistics
    // -------------------------------------------------------------------------

    #[test]
    fn statistics_are_deterministic() {
        let topology = HardwareTopology::linear(4).unwrap();

        let statistics = topology.statistics();

        assert_eq!(statistics.resource_count, 4);
        assert_eq!(statistics.coupling_count, 3);
        assert_eq!(
            statistics.bidirectional_coupling_count,
            3
        );
        assert_eq!(
            statistics.directed_coupling_count,
            0
        );
        assert_eq!(statistics.connected_components, 1);
        assert!(statistics.is_connected);
        assert_eq!(statistics.diameter, Some(3));
        assert_eq!(
            statistics.average_shortest_path,
            Some(5.0 / 3.0)
        );
    }

    #[test]
    fn disconnected_statistics_report_no_diameter() {
        let topology =
            HardwareTopology::from_couplings(
                4,
                [Coupling::bidirectional(0, 1)],
            )
            .unwrap();

        let statistics = topology.statistics();

        assert_eq!(statistics.connected_components, 3);
        assert!(!statistics.is_connected);
        assert_eq!(statistics.diameter, None);
        assert_eq!(statistics.average_shortest_path, None);
    }

    #[test]
    fn density_is_correct() {
        let topology = HardwareTopology::linear(4).unwrap();

        let statistics = topology.statistics();

        // Three physical pairs out of six possible pairs.
        assert_eq!(
            statistics.undirected_density,
            0.5
        );
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    #[test]
    fn constructed_topologies_validate() {
        let topologies = [
            HardwareTopology::linear(1).unwrap(),
            HardwareTopology::linear(5).unwrap(),
            HardwareTopology::ring(5).unwrap(),
            HardwareTopology::fully_connected(5).unwrap(),
        ];

        for topology in &topologies {
            assert!(topology.validate().is_ok());
        }
    }

    #[test]
    fn invalid_resource_is_rejected() {
        let topology = HardwareTopology::new(2).unwrap();

        assert_eq!(
            topology.neighbours(2),
            Err(TopologyError::InvalidQubit {
                qubit: 2,
                qubit_count: 2
            })
        );
    }

    // -------------------------------------------------------------------------
    // Coupling lookup
    // -------------------------------------------------------------------------

    #[test]
    fn coupling_lookup_respects_direction() {
        let topology = HardwareTopology::from_couplings(
            2,
            [Coupling::directed(0, 1)],
        )
        .unwrap();

        assert_eq!(
            topology.coupling(0, 1).unwrap(),
            Some(Coupling::directed(0, 1))
        );

        assert_eq!(
            topology.coupling(1, 0).unwrap(),
            None
        );
    }

    #[test]
    fn bidirectional_coupling_lookup_works_both_ways() {
        let topology = HardwareTopology::from_couplings(
            2,
            [Coupling::bidirectional(0, 1)],
        )
        .unwrap();

        assert_eq!(
            topology.coupling(0, 1).unwrap(),
            Some(Coupling::bidirectional(0, 1))
        );

        assert_eq!(
            topology.coupling(1, 0).unwrap(),
            Some(Coupling::bidirectional(0, 1))
        );
    }

    // -------------------------------------------------------------------------
    // Fingerprint
    // -------------------------------------------------------------------------

    #[test]
    fn fingerprint_is_stable() {
        let first = HardwareTopology::linear(8).unwrap();
        let second = HardwareTopology::from_couplings(
            8,
            [
                Coupling::bidirectional(3, 4),
                Coupling::bidirectional(1, 2),
                Coupling::bidirectional(6, 7),
                Coupling::bidirectional(0, 1),
                Coupling::bidirectional(4, 5),
                Coupling::bidirectional(2, 3),
                Coupling::bidirectional(5, 6),
            ],
        )
        .unwrap();

        assert_eq!(
            first.fingerprint(),
            second.fingerprint()
        );
    }

    #[test]
    fn different_topologies_have_different_fingerprints() {
        let linear = HardwareTopology::linear(4).unwrap();
        let ring = HardwareTopology::ring(4).unwrap();

        assert_ne!(
            linear.fingerprint(),
            ring.fingerprint()
        );
    }

    // -------------------------------------------------------------------------
    // Resource iteration
    // -------------------------------------------------------------------------

    #[test]
    fn resources_are_returned_in_deterministic_order() {
        let topology = HardwareTopology::new(5).unwrap();

        let resources: Vec<_> =
            topology.resources().collect();

        assert_eq!(
            resources,
            vec![0, 1, 2, 3, 4]
        );
    }

    // -------------------------------------------------------------------------
    // Default
    // -------------------------------------------------------------------------

    #[test]
    fn default_is_valid() {
        let topology = HardwareTopology::default();

        assert_eq!(topology.qubit_count(), 1);
        assert!(topology.validate().is_ok());
    }
}