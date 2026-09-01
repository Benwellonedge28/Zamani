//! Zamani Quantum Noise (ZQN) — Spatial Noise Semantics
//!
//! # Ownership
//!
//! This module owns the **spatial semantics of quantum noise**.
//!
//! It answers:
//!
//! > Which quantum resources are spatially related, how are those resources
//! > connected or positioned relative to one another, and how can a spatial
//! > noise/correlation policy inspect those relationships without assuming a
//! > particular machine size, topology, vendor, or quantum technology?
//!
//! This module owns:
//!
//! - spatial resource references;
//! - spatial coordinates;
//! - topology-independent spatial relationships;
//! - deterministic spatial graphs;
//! - directed and undirected spatial edges;
//! - arbitrary edge metadata;
//! - neighborhood queries;
//! - adjacency queries;
//! - connected-component traversal;
//! - bounded graph traversal;
//! - spatial distance policies;
//! - spatial correlation kernels;
//! - spatial influence queries;
//! - deterministic canonicalization;
//! - validation of spatial models;
//! - incremental spatial-model construction;
//! - resource-independent scaling semantics.
//!
//! This module does NOT own:
//!
//! - canonical quantum identities;
//! - hardware topology ownership;
//! - routing;
//! - placement;
//! - scheduling;
//! - calibration;
//! - temporal noise;
//! - quantum channels;
//! - probability distributions;
//! - stochastic sampling;
//! - realized faults;
//! - QEC;
//! - simulator state;
//! - vendor APIs;
//! - serialization formats;
//! - global resource limits;
//! - global registries;
//! - global mutable state.
//!
//! Canonical quantum identities remain owned by:
//!
//! ```text
//! crate::quantum::ir::qubit
//! ```
//!
//! Specifically:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! # Architectural position
//!
//! ```text
//!                     quantum::ir
//!                         │
//!                         │ canonical resources
//!                         ▼
//!                 zqn::noise::spatial
//!                         │
//!             ┌───────────┼────────────┐
//!             │           │            │
//!             ▼           ▼            ▼
//!        correlation   crosstalk    noise model
//!             │           │            │
//!             └───────────┼────────────┘
//!                         ▼
//!                application/simulation
//!                         │
//!                ┌────────┼─────────┐
//!                ▼        ▼         ▼
//!               QEC     routing   scheduling
//! ```
//!
//! Spatial noise describes physical/spatial relationships.
//!
//! Routing decides where resources are placed.
//!
//! Scheduling decides when operations occur.
//!
//! Hardware owns the authoritative device topology.
//!
//! This module therefore consumes spatial information without becoming a
//! hardware or routing subsystem.
//!
//! # Spatial versus temporal semantics
//!
//! Spatial correlation answers:
//!
//! ```text
//! "How are resources related because of where they are or how they connect?"
//! ```
//!
//! Temporal correlation answers:
//!
//! ```text
//! "How is noise related because of when events occur?"
//! ```
//!
//! Temporal semantics belong to `noise::temporal`.
//!
//! Spatial semantics belong here.
//!
//! A combined physical model may consume both:
//!
//! ```text
//! spatial relation
//!        +
//! temporal relation
//!        +
//! channel/fault semantics
//!        ↓
//! complete noise model
//! ```
//!
//! # Canonical qubit identity
//!
//! This module MUST NOT define:
//!
//! ```text
//! SpatialQubitId
//! NoiseQubitId
//! SpatialPhysicalQubitId
//! ZqnQubitId
//! ```
//!
//! Logical qubits are represented by:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! Physical qubits are represented by:
//!
//! ```text
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! Logical and physical identities remain different Rust types.
//!
//! ```text
//! QubitId(7)
//! PhysicalQubitId(7)
//! ```
//!
//! are not interchangeable merely because their underlying indices happen to
//! be equal.
//!
//! # Future quantum technologies
//!
//! ZQN must not permanently assume that every quantum system is a qubit graph.
//!
//! This module therefore provides an opaque resource-reference variant for
//! spatial resources whose canonical identity is owned by another IR domain.
//!
//! That permits future integration with:
//!
//! - qudits;
//! - bosonic modes;
//! - continuous-variable modes;
//! - photonic modes;
//! - fermionic modes;
//! - analog resources;
//! - distributed quantum resources;
//! - communication links;
//! - logical resources;
//! - future quantum modalities.
//!
//! The opaque identifier is only a spatial reference.
//!
//! It does not become a competing canonical quantum-resource identity.
//!
//! # Write once, scale everywhere
//!
//! This module deliberately contains no:
//!
//! ```text
//! MAX_QUBITS
//! MAX_NODES
//! MAX_EDGES
//! MAX_NEIGHBORS
//! MAX_DISTANCE
//! MAX_CORRELATIONS
//! ```
//!
//! A spatial model can contain any finite number of resources representable by
//! the host system and permitted by the caller's resource policy.
//!
//! The architecture therefore imposes no semantic machine-size ceiling.
//!
//! "Infinity" means:
//!
//! > no artificial finite machine-size ceiling is encoded into the spatial
//! > semantics.
//!
//! It does NOT mean that a particular process can materialize an infinite
//! graph or that a physical machine has infinite resources.
//!
//! Resource limits belong to the caller/runtime/resource-policy layer.
//!
//! # Determinism
//!
//! This module is deterministic.
//!
//! It:
//!
//! - owns no RNG;
//! - reads no system time;
//! - owns no global mutable state;
//! - does not depend on hash-map iteration order;
//! - uses ordered collections;
//! - canonicalizes undirected edges;
//! - validates floating-point values;
//! - provides deterministic traversal order;
//! - provides deterministic equality and ordering.
//!
//! Given identical semantic inputs, the resulting spatial model is identical.
//!
//! # Numerical policy
//!
//! Spatial coordinates and distances use `f64` because spatial metadata is
//! generally measured rather than exact.
//!
//! Floating-point values are accepted only when finite.
//!
//! NaN and infinite values are rejected.
//!
//! Negative distances are rejected where a distance is explicitly supplied.
//!
//! Spatial correlation strength is represented separately from probability.
//!
//! A correlation coefficient is not automatically a probability.
//!
//! # Approximation
//!
//! Spatial distance and correlation may be exact, approximate, or bounded.
//!
//! This file does not silently convert one semantic class into another.
//!
//! An approximation must be explicit in the calling noise model.
//!
//! The spatial graph itself records spatial facts; it does not invent a
//! statistical confidence level.
//!
//! # Topology
//!
//! The topology is represented as data.
//!
//! No topology is hard-coded.
//!
//! The same spatial model can represent:
//!
//! - line topology;
//! - ring topology;
//! - grid topology;
//! - lattice topology;
//! - arbitrary graph topology;
//! - fully connected topology;
//! - sparse topology;
//! - disconnected systems;
//! - distributed systems;
//! - irregular physical layouts;
//! - future topologies.
//!
//! The number of nodes and edges is data, not a compile-time architectural
//! parameter.
//!
//! # Directionality
//!
//! Some spatial relationships are naturally undirected:
//!
//! ```text
//! A <-> B
//! ```
//!
//! Others are directed:
//!
//! ```text
//! A -> B
//! ```
//!
//! For example, a spatial influence may be asymmetric.
//!
//! The model therefore supports both.
//!
//! Undirected edges are canonicalized so:
//!
//! ```text
//! A -- B
//! B -- A
//! ```
//!
//! represent the same relationship.
//!
//! Directed edges remain direction-sensitive.
//!
//! # Self relationships
//!
//! A spatial edge from a resource to itself is rejected.
//!
//! Self-correlation may have legitimate meanings in statistical or temporal
//! models, but a spatial self-edge is ambiguous at this abstraction level.
//!
//! Such semantics belong to the appropriate temporal/statistical model.
//!
//! # Duplicate relationships
//!
//! Duplicate semantic edges are rejected during construction.
//!
//! The module never silently:
//!
//! - sums weights;
//! - overwrites edges;
//! - chooses one duplicate;
//! - changes direction.
//!
//! If a caller has multiple observations of the same physical relationship,
//! characterization/estimation should combine them before producing the
//! canonical spatial model.
//!
//! # Resource membership
//!
//! Every edge must reference resources that belong to the same model.
//!
//! An edge referencing an unknown resource is rejected.
//!
//! This prevents dangling topology entries and makes downstream consumers
//! deterministic.
//!
//! # Resource identity domains
//!
//! The following identities are distinct:
//!
//! ```text
//! LogicalQubit(QubitId)
//! PhysicalQubit(PhysicalQubitId)
//! External(String)
//! ```
//!
//! The `External` variant exists only as a bridge for resource domains that are
//! owned elsewhere.
//!
//! It must not be used to smuggle hardware handles, credentials, pointers, or
//! executable capabilities into ZQN.
//!
//! # Memory model
//!
//! The canonical `SpatialModel` is an owned materialized representation.
//!
//! It is suitable for ordinary compiler/runtime workloads.
//!
//! For extremely large systems, callers may:
//!
//! - construct the model incrementally;
//! - partition the model;
//! - use streaming topology sources;
//! - query only relevant neighborhoods;
//! - maintain distributed partitions;
//! - use target-specific sparse representations.
//!
//! This module does not claim that all spatial information must fit in one
//! process.
//!
//! # Resource safety
//!
//! This module does not enforce global memory limits.
//!
//! A caller accepting untrusted spatial specifications MUST impose appropriate
//! resource policies before materializing unbounded input.
//!
//! `with_capacity` is only an allocation hint.
//!
//! It is never a semantic maximum.
//!
//! # Serialization
//!
//! This module intentionally does not depend on a serialization framework.
//!
//! Serialization belongs to the ZQN IO subsystem.
//!
//! A serializer must preserve:
//!
//! - resource identity domain;
//! - resource identity;
//! - coordinate dimensionality;
//! - coordinates;
//! - edge direction;
//! - edge metadata;
//! - distance semantics;
//! - spatial kernel parameters;
//! - canonical ordering;
//! - schema/version.
//!
//! Serialization must not collapse logical and physical identifiers into a
//! common integer namespace.
//!
//! # Security
//!
//! Spatial resources are data, not capabilities.
//!
//! A spatial identifier must never grant:
//!
//! - QPU access;
//! - backend execution;
//! - network access;
//! - filesystem access;
//! - credential access;
//! - calibration write access.
//!
//! External/untrusted topology data must be subject to explicit caller-owned
//! resource limits.
//!
//! # Thread safety
//!
//! `SpatialModel` is immutable after construction.
//!
//! It contains no interior mutability or global state.
//!
//! Read-only instances can therefore be shared across threads when their
//! contained values are thread-safe.
//!
//! # Integration contract
//!
//! The intended integration direction is:
//!
//! ```text
//! quantum::ir::qubit
//!          │
//!          ▼
//! SpatialResource
//!          │
//!          ▼
//! SpatialModel
//!          │
//!      ┌───┼─────────────┐
//!      ▼   ▼             ▼
//! correlation crosstalk routing/scheduling
//!      │   │             │
//!      └───┼─────────────┘
//!          ▼
//!       NoiseModel
//! ```
//!
//! `noise::correlation` may consume the spatial model to construct a
//! correlation domain.
//!
//! `noise::crosstalk` may consume spatial adjacency and influence information.
//!
//! Routing may consume spatial information when calculating placement costs,
//! but routing remains the owner of placement decisions.
//!
//! Scheduling may consume spatial interaction information when estimating
//! crosstalk or resource contention.
//!
//! Hardware adapters may construct a `SpatialModel` from provider-neutral
//! topology descriptions.
//!
//! ZQN does not call hardware APIs.
//!
//! # File-completion invariant
//!
//! This file is complete when:
//!
//! 1. canonical IR qubit identities are reused;
//! 2. no duplicate qubit identity types exist;
//! 3. no hardware/vendor assumptions exist;
//! 4. no machine-size constant exists;
//! 5. topology is data-driven;
//! 6. directed and undirected relationships are explicit;
//! 7. duplicate/self edges are rejected;
//! 8. dangling resources are rejected;
//! 9. traversal is deterministic;
//! 10. invalid floating-point values are rejected;
//! 11. no unsafe Rust exists;
//! 12. no global mutable state exists;
//! 13. no hidden randomness exists;
//! 14. resource limits remain caller policy;
//! 15. the module compiles independently once declared by `noise/mod.rs`;
//! 16. adding downstream consumers does not require modifying this file.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - standard library only.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::ops::RangeInclusive;

use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};

/// Stable identifier for a spatial model.
///
/// This is a ZQN model identity, not a quantum-resource identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SpatialModelId(u128);

impl SpatialModelId {
    /// Creates a model identifier.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the raw model identifier.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }
}

impl fmt::Display for SpatialModelId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:032x}", self.0)
    }
}

/// Spatially addressable resource.
///
/// The canonical logical and physical qubit variants use the authoritative
/// Zamani IR identifiers.
///
/// `External` is an opaque reference for resource domains owned by another
/// subsystem.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpatialResource {
    /// Canonical logical qubit.
    LogicalQubit(QubitId),

    /// Canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Opaque externally-owned resource identity.
    ///
    /// This string is an identity reference only. It is never interpreted as
    /// an executable handle or hardware API identifier by this module.
    External(String),
}

impl SpatialResource {
    /// Creates an external resource reference.
    ///
    /// Empty external identifiers are rejected.
    pub fn external<S>(identifier: S) -> Result<Self, SpatialError>
    where
        S: Into<String>,
    {
        let identifier = identifier.into();

        if identifier.is_empty() {
            return Err(SpatialError::EmptyExternalResourceId);
        }

        Ok(Self::External(identifier))
    }

    /// Returns whether this is a logical qubit.
    #[must_use]
    pub const fn is_logical_qubit(&self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns whether this is a physical qubit.
    #[must_use]
    pub const fn is_physical_qubit(&self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns whether this is an externally-owned resource.
    #[must_use]
    pub const fn is_external(&self) -> bool {
        matches!(self, Self::External(_))
    }

    /// Returns the canonical logical qubit when applicable.
    #[must_use]
    pub const fn logical_qubit(&self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(id) => Some(*id),
            Self::PhysicalQubit(_) | Self::External(_) => None,
        }
    }

    /// Returns the canonical physical qubit when applicable.
    #[must_use]
    pub const fn physical_qubit(&self) -> Option<PhysicalQubitId> {
        match self {
            Self::LogicalQubit(_) | Self::External(_) => None,
            Self::PhysicalQubit(id) => Some(*id),
        }
    }
}

impl From<QubitId> for SpatialResource {
    fn from(value: QubitId) -> Self {
        Self::LogicalQubit(value)
    }
}

impl From<PhysicalQubitId> for SpatialResource {
    fn from(value: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(value)
    }
}

impl fmt::Display for SpatialResource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LogicalQubit(id) => write!(formatter, "logical:{id}"),
            Self::PhysicalQubit(id) => write!(formatter, "physical:{id}"),
            Self::External(id) => write!(formatter, "external:{id}"),
        }
    }
}

/// Spatial coordinate.
///
/// Coordinates are dimension-agnostic and may represent:
///
/// - physical positions;
/// - abstract embedding coordinates;
/// - lattice coordinates;
/// - frequency-space coordinates;
/// - topology coordinates;
/// - any other caller-defined spatial embedding.
///
/// The dimensionality is determined by the length of the coordinate vector.
///
/// A zero-dimensional coordinate is allowed because some topologies are
/// abstract graphs rather than geometrical embeddings.
#[derive(Debug, Clone, PartialEq)]
pub struct SpatialCoordinate {
    values: Vec<f64>,
}

impl SpatialCoordinate {
    /// Creates a coordinate from finite values.
    pub fn new<I>(values: I) -> Result<Self, SpatialError>
    where
        I: IntoIterator<Item = f64>,
    {
        let values: Vec<f64> = values.into_iter().collect();

        if values.iter().any(|value| !value.is_finite()) {
            return Err(SpatialError::NonFiniteCoordinate);
        }

        Ok(Self { values })
    }

    /// Creates a zero-dimensional coordinate.
    #[must_use]
    pub const fn zero_dimensional() -> Self {
        Self { values: Vec::new() }
    }

    /// Returns the coordinate dimensionality.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.values.len()
    }

    /// Returns the coordinate values.
    #[must_use]
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Returns whether this coordinate has no dimensions.
    #[must_use]
    pub fn is_zero_dimensional(&self) -> bool {
        self.values.is_empty()
    }

    /// Computes Euclidean distance to another coordinate.
    ///
    /// Returns an error when the dimensionalities differ or when the resulting
    /// distance cannot be represented as a finite `f64`.
    pub fn euclidean_distance(
        &self,
        other: &Self,
    ) -> Result<f64, SpatialError> {
        if self.dimension() != other.dimension() {
            return Err(SpatialError::DimensionMismatch {
                left: self.dimension(),
                right: other.dimension(),
            });
        }

        let mut squared = 0.0_f64;

        for (&left, &right) in self.values.iter().zip(other.values.iter()) {
            let delta = left - right;

            squared = squared
                .checked_add(delta * delta)
                .ok_or(SpatialError::NumericalOverflow)?;
        }

        let distance = squared.sqrt();

        if !distance.is_finite() {
            return Err(SpatialError::NumericalOverflow);
        }

        Ok(distance)
    }
}

/// Direction of a spatial relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpatialDirection {
    /// Relationship has no direction.
    Undirected,

    /// Relationship is directed from source to target.
    Directed,
}

/// Spatial edge.
///
/// Edges are immutable semantic relationships.
///
/// For undirected edges, the constructor canonicalizes endpoint ordering.
/// Directed edges preserve source and target.
#[derive(Debug, Clone, PartialEq)]
pub struct SpatialEdge {
    source: SpatialResource,
    target: SpatialResource,
    direction: SpatialDirection,
    distance: Option<f64>,
    influence: Option<f64>,
}

impl SpatialEdge {
    /// Creates an undirected edge.
    pub fn undirected(
        first: SpatialResource,
        second: SpatialResource,
    ) -> Result<Self, SpatialError> {
        Self::new(
            first,
            second,
            SpatialDirection::Undirected,
            None,
            None,
        )
    }

    /// Creates a directed edge.
    pub fn directed(
        source: SpatialResource,
        target: SpatialResource,
    ) -> Result<Self, SpatialError> {
        Self::new(
            source,
            target,
            SpatialDirection::Directed,
            None,
            None,
        )
    }

    /// Creates an edge with optional spatial distance and influence.
    pub fn new(
        first: SpatialResource,
        second: SpatialResource,
        direction: SpatialDirection,
        distance: Option<f64>,
        influence: Option<f64>,
    ) -> Result<Self, SpatialError> {
        if first == second {
            return Err(SpatialError::SelfRelationship {
                resource: first,
            });
        }

        if let Some(value) = distance {
            validate_non_negative_finite(value, "distance")?;
        }

        if let Some(value) = influence {
            validate_finite(value, "influence")?;
        }

        let (source, target) = match direction {
            SpatialDirection::Directed => (first, second),
            SpatialDirection::Undirected => {
                if first <= second {
                    (first, second)
                } else {
                    (second, first)
                }
            }
        };

        Ok(Self {
            source,
            target,
            direction,
            distance,
            influence,
        })
    }

    /// Returns the source endpoint.
    #[must_use]
    pub fn source(&self) -> &SpatialResource {
        &self.source
    }

    /// Returns the target endpoint.
    #[must_use]
    pub fn target(&self) -> &SpatialResource {
        &self.target
    }

    /// Returns the direction.
    #[must_use]
    pub const fn direction(&self) -> SpatialDirection {
        self.direction
    }

    /// Returns whether the edge is directed.
    #[must_use]
    pub const fn is_directed(&self) -> bool {
        matches!(self.direction, SpatialDirection::Directed)
    }

    /// Returns whether the edge is undirected.
    #[must_use]
    pub const fn is_undirected(&self) -> bool {
        matches!(self.direction, SpatialDirection::Undirected)
    }

    /// Returns the explicitly supplied distance, if any.
    #[must_use]
    pub const fn distance(&self) -> Option<f64> {
        self.distance
    }

    /// Returns the explicitly supplied influence value, if any.
    #[must_use]
    pub const fn influence(&self) -> Option<f64> {
        self.influence
    }

    /// Returns the endpoint opposite `resource`.
    ///
    /// For a directed edge, this method returns the other endpoint regardless
    /// of direction. Direction-sensitive traversal should use
    /// `SpatialModel::outgoing_neighbors` or
    /// `SpatialModel::incoming_neighbors`.
    #[must_use]
    pub fn opposite(
        &self,
        resource: &SpatialResource,
    ) -> Option<&SpatialResource> {
        if resource == &self.source {
            Some(&self.target)
        } else if resource == &self.target {
            Some(&self.source)
        } else {
            None
        }
    }
}

/// Spatial edge key used for deterministic duplicate detection.
///
/// The direction is part of the key, so:
///
/// ```text
/// A -> B
/// B -> A
/// ```
///
/// remain different directed relationships.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct SpatialEdgeKey {
    source: SpatialResource,
    target: SpatialResource,
    direction: SpatialDirection,
}

impl From<&SpatialEdge> for SpatialEdgeKey {
    fn from(edge: &SpatialEdge) -> Self {
        Self {
            source: edge.source.clone(),
            target: edge.target.clone(),
            direction: edge.direction,
        }
    }
}

/// Optional coordinate associated with a spatial resource.
#[derive(Debug, Clone, PartialEq)]
pub struct SpatialPlacement {
    resource: SpatialResource,
    coordinate: SpatialCoordinate,
}

impl SpatialPlacement {
    /// Creates a resource placement.
    #[must_use]
    pub fn new(
        resource: SpatialResource,
        coordinate: SpatialCoordinate,
    ) -> Self {
        Self {
            resource,
            coordinate,
        }
    }

    /// Returns the resource.
    #[must_use]
    pub fn resource(&self) -> &SpatialResource {
        &self.resource
    }

    /// Returns the coordinate.
    #[must_use]
    pub fn coordinate(&self) -> &SpatialCoordinate {
        &self.coordinate
    }
}

/// Spatial distance metric.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SpatialMetric {
    /// Euclidean distance between coordinates.
    Euclidean,

    /// Manhattan/L1 distance.
    Manhattan,

    /// Chebyshev/L-infinity distance.
    Chebyshev,

    /// Graph hop distance.
    Hop,
}

/// Spatial correlation kernel.
///
/// A kernel maps a non-negative spatial distance to an influence value.
///
/// The result is not automatically a probability.
///
/// A kernel is a deterministic semantic function.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpatialKernel {
    /// Influence decreases as:
    ///
    /// `exp(-distance / length_scale)`
    ///
    /// where `length_scale > 0`.
    Exponential {
        length_scale: f64,
    },

    /// Influence decreases as:
    ///
    /// `exp(-(distance / length_scale)^2)`
    ///
    /// where `length_scale > 0`.
    Gaussian {
        length_scale: f64,
    },

    /// Influence follows:
    ///
    /// `1 / (1 + distance / length_scale)^power`
    ///
    /// where `length_scale > 0` and `power > 0`.
    PowerLaw {
        length_scale: f64,
        power: f64,
    },

    /// Constant influence independent of distance.
    Constant {
        value: f64,
    },
}

impl SpatialKernel {
    /// Creates and validates an exponential kernel.
    pub fn exponential(length_scale: f64) -> Result<Self, SpatialError> {
        validate_positive_finite(length_scale, "length_scale")?;

        Ok(Self::Exponential { length_scale })
    }

    /// Creates and validates a Gaussian kernel.
    pub fn gaussian(length_scale: f64) -> Result<Self, SpatialError> {
        validate_positive_finite(length_scale, "length_scale")?;

        Ok(Self::Gaussian { length_scale })
    }

    /// Creates and validates a power-law kernel.
    pub fn power_law(
        length_scale: f64,
        power: f64,
    ) -> Result<Self, SpatialError> {
        validate_positive_finite(length_scale, "length_scale")?;
        validate_positive_finite(power, "power")?;

        Ok(Self::PowerLaw {
            length_scale,
            power,
        })
    }

    /// Creates and validates a constant kernel.
    pub fn constant(value: f64) -> Result<Self, SpatialError> {
        validate_finite(value, "value")?;

        Ok(Self::Constant { value })
    }

    /// Evaluates the kernel at a non-negative finite distance.
    pub fn evaluate(&self, distance: f64) -> Result<f64, SpatialError> {
        validate_non_negative_finite(distance, "distance")?;

        let value = match self {
            Self::Exponential { length_scale } => {
                (-distance / *length_scale).exp()
            }

            Self::Gaussian { length_scale } => {
                let normalized = distance / *length_scale;
                (-(normalized * normalized)).exp()
            }

            Self::PowerLaw {
                length_scale,
                power,
            } => {
                let base = 1.0 + distance / *length_scale;
                base.powf(-*power)
            }

            Self::Constant { value } => *value,
        };

        if !value.is_finite() {
            return Err(SpatialError::NumericalOverflow);
        }

        Ok(value)
    }
}

/// Immutable spatial-noise model.
///
/// The model contains:
///
/// - a stable ZQN model identifier;
/// - resources;
/// - optional coordinates;
/// - deterministic spatial edges;
/// - optional spatial kernel.
///
/// It does not own hardware state or execution state.
#[derive(Debug, Clone, PartialEq)]
pub struct SpatialModel {
    id: SpatialModelId,
    resources: BTreeSet<SpatialResource>,
    placements: BTreeMap<SpatialResource, SpatialCoordinate>,
    edges: BTreeMap<SpatialEdgeKey, SpatialEdge>,
    kernel: Option<SpatialKernel>,
}

impl SpatialModel {
    /// Returns the model identifier.
    #[must_use]
    pub const fn id(&self) -> SpatialModelId {
        self.id
    }

    /// Returns all resources in canonical order.
    #[must_use]
    pub fn resources(&self) -> &BTreeSet<SpatialResource> {
        &self.resources
    }

    /// Returns all spatial placements in canonical order.
    #[must_use]
    pub fn placements(
        &self,
    ) -> &BTreeMap<SpatialResource, SpatialCoordinate> {
        &self.placements
    }

    /// Returns all edges in canonical order.
    #[must_use]
    pub fn edges(&self) -> impl Iterator<Item = &SpatialEdge> {
        self.edges.values()
    }

    /// Returns the number of resources.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Returns the number of edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns the configured spatial kernel.
    #[must_use]
    pub const fn kernel(&self) -> Option<SpatialKernel> {
        self.kernel
    }

    /// Returns whether a resource belongs to the model.
    #[must_use]
    pub fn contains_resource(&self, resource: &SpatialResource) -> bool {
        self.resources.contains(resource)
    }

    /// Returns a resource's coordinate when one is available.
    #[must_use]
    pub fn coordinate(
        &self,
        resource: &SpatialResource,
    ) -> Option<&SpatialCoordinate> {
        self.placements.get(resource)
    }

    /// Returns the explicitly represented distance between two resources.
    ///
    /// If the edge does not have an explicit distance but both resources have
    /// coordinates, Euclidean distance is derived.
    pub fn distance(
        &self,
        first: &SpatialResource,
        second: &SpatialResource,
    ) -> Result<Option<f64>, SpatialError> {
        if !self.contains_resource(first) {
            return Err(SpatialError::UnknownResource {
                resource: first.clone(),
            });
        }

        if !self.contains_resource(second) {
            return Err(SpatialError::UnknownResource {
                resource: second.clone(),
            });
        }

        if first == second {
            return Ok(Some(0.0));
        }

        if let Some(edge) = self.find_edge(first, second) {
            if let Some(distance) = edge.distance() {
                return Ok(Some(distance));
            }
        }

        match (self.coordinate(first), self.coordinate(second)) {
            (Some(first), Some(second)) => {
                Ok(Some(first.euclidean_distance(second)?))
            }
            _ => Ok(None),
        }
    }

    /// Returns all undirected and direction-independent neighbors.
    ///
    /// Results are returned in deterministic canonical order.
    #[must_use]
    pub fn neighbors(
        &self,
        resource: &SpatialResource,
    ) -> Vec<SpatialResource> {
        let mut result = BTreeSet::new();

        for edge in self.edges.values() {
            if let Some(other) = edge.opposite(resource) {
                result.insert(other.clone());
            }
        }

        result.into_iter().collect()
    }

    /// Returns outgoing neighbors for directed traversal.
    ///
    /// Undirected edges are considered outgoing in both directions.
    #[must_use]
    pub fn outgoing_neighbors(
        &self,
        resource: &SpatialResource,
    ) -> Vec<SpatialResource> {
        let mut result = BTreeSet::new();

        for edge in self.edges.values() {
            match edge.direction() {
                SpatialDirection::Undirected => {
                    if let Some(other) = edge.opposite(resource) {
                        result.insert(other.clone());
                    }
                }

                SpatialDirection::Directed => {
                    if edge.source() == resource {
                        result.insert(edge.target().clone());
                    }
                }
            }
        }

        result.into_iter().collect()
    }

    /// Returns incoming neighbors for directed traversal.
    ///
    /// Undirected edges are considered incoming in both directions.
    #[must_use]
    pub fn incoming_neighbors(
        &self,
        resource: &SpatialResource,
    ) -> Vec<SpatialResource> {
        let mut result = BTreeSet::new();

        for edge in self.edges.values() {
            match edge.direction() {
                SpatialDirection::Undirected => {
                    if let Some(other) = edge.opposite(resource) {
                        result.insert(other.clone());
                    }
                }

                SpatialDirection::Directed => {
                    if edge.target() == resource {
                        result.insert(edge.source().clone());
                    }
                }
            }
        }

        result.into_iter().collect()
    }

    /// Returns whether a relationship exists between two resources.
    ///
    /// For directed relationships, both directions are checked independently.
    #[must_use]
    pub fn is_connected(
        &self,
        first: &SpatialResource,
        second: &SpatialResource,
    ) -> bool {
        self.find_edge(first, second).is_some()
            || self.find_edge(second, first).is_some()
    }

    /// Returns the direct edge from `first` to `second`, if present.
    #[must_use]
    pub fn edge(
        &self,
        first: &SpatialResource,
        second: &SpatialResource,
    ) -> Option<&SpatialEdge> {
        self.find_edge(first, second)
    }

    /// Returns all resources within a direct spatial neighborhood.
    ///
    /// This operation does not allocate or traverse beyond direct edges.
    #[must_use]
    pub fn direct_neighborhood(
        &self,
        resource: &SpatialResource,
    ) -> Vec<SpatialResource> {
        self.neighbors(resource)
    }

    /// Traverses the spatial graph up to `max_hops`.
    ///
    /// The traversal is deterministic and breadth-first.
    ///
    /// `max_hops == 0` returns only the starting resource when it exists.
    pub fn reachable_within_hops(
        &self,
        start: &SpatialResource,
        max_hops: usize,
    ) -> Result<Vec<SpatialResource>, SpatialError> {
        if !self.contains_resource(start) {
            return Err(SpatialError::UnknownResource {
                resource: start.clone(),
            });
        }

        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start.clone());
        queue.push_back((start.clone(), 0usize));

        while let Some((resource, depth)) = queue.pop_front() {
            if depth >= max_hops {
                continue;
            }

            for neighbor in self.outgoing_neighbors(&resource) {
                if visited.insert(neighbor.clone()) {
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }

        Ok(visited.into_iter().collect())
    }

    /// Computes the shortest directed hop distance.
    ///
    /// Undirected edges can be traversed in both directions.
    pub fn hop_distance(
        &self,
        start: &SpatialResource,
        goal: &SpatialResource,
    ) -> Result<Option<usize>, SpatialError> {
        if !self.contains_resource(start) {
            return Err(SpatialError::UnknownResource {
                resource: start.clone(),
            });
        }

        if !self.contains_resource(goal) {
            return Err(SpatialError::UnknownResource {
                resource: goal.clone(),
            });
        }

        if start == goal {
            return Ok(Some(0));
        }

        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start.clone());
        queue.push_back((start.clone(), 0usize));

        while let Some((resource, distance)) = queue.pop_front() {
            for neighbor in self.outgoing_neighbors(&resource) {
                if &neighbor == goal {
                    return Ok(Some(distance + 1));
                }

                if visited.insert(neighbor.clone()) {
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }

        Ok(None)
    }

    /// Computes connected components using undirected connectivity.
    ///
    /// Direction is ignored for component discovery because a spatial
    /// relationship still connects the two resources physically.
    pub fn connected_components(
        &self,
    ) -> Vec<Vec<SpatialResource>> {
        let mut remaining = self.resources.clone();
        let mut components = Vec::new();

        while let Some(start) = remaining.iter().next().cloned() {
            let mut component = BTreeSet::new();
            let mut queue = VecDeque::new();

            remaining.remove(&start);
            component.insert(start.clone());
            queue.push_back(start);

            while let Some(resource) = queue.pop_front() {
                for neighbor in self.neighbors(&resource) {
                    if remaining.remove(&neighbor) {
                        component.insert(neighbor.clone());
                        queue.push_back(neighbor);
                    }
                }
            }

            components.push(component.into_iter().collect());
        }

        components
    }

    /// Evaluates the configured spatial kernel at the relationship distance.
    ///
    /// Returns `None` when there is no kernel or no determinable distance.
    pub fn spatial_influence(
        &self,
        first: &SpatialResource,
        second: &SpatialResource,
    ) -> Result<Option<f64>, SpatialError> {
        let Some(kernel) = self.kernel else {
            return Ok(None);
        };

        let Some(distance) = self.distance(first, second)? else {
            return Ok(None);
        };

        Ok(Some(kernel.evaluate(distance)?))
    }

    /// Validates all internal invariants.
    pub fn validate(&self) -> Result<(), SpatialError> {
        for edge in self.edges.values() {
            if !self.resources.contains(edge.source()) {
                return Err(SpatialError::UnknownResource {
                    resource: edge.source().clone(),
                });
            }

            if !self.resources.contains(edge.target()) {
                return Err(SpatialError::UnknownResource {
                    resource: edge.target().clone(),
                });
            }

            if edge.source() == edge.target() {
                return Err(SpatialError::SelfRelationship {
                    resource: edge.source().clone(),
                });
            }

            if let Some(distance) = edge.distance() {
                validate_non_negative_finite(distance, "distance")?;
            }

            if let Some(influence) = edge.influence() {
                validate_finite(influence, "influence")?;
            }
        }

        for coordinate in self.placements.values() {
            if coordinate.values().iter().any(|value| !value.is_finite()) {
                return Err(SpatialError::NonFiniteCoordinate);
            }
        }

        if let Some(kernel) = self.kernel {
            validate_kernel(kernel)?;
        }

        Ok(())
    }

    fn find_edge(
        &self,
        first: &SpatialResource,
        second: &SpatialResource,
    ) -> Option<&SpatialEdge> {
        let undirected_key = SpatialEdgeKey {
            source: first.clone(),
            target: second.clone(),
            direction: SpatialDirection::Undirected,
        };

        if let Some(edge) = self.edges.get(&undirected_key) {
            return Some(edge);
        }

        let directed_key = SpatialEdgeKey {
            source: first.clone(),
            target: second.clone(),
            direction: SpatialDirection::Directed,
        };

        self.edges.get(&directed_key)
    }
}

/// Incremental builder for [`SpatialModel`].
///
/// The builder is intentionally independent of hardware APIs and can consume
/// generated or streamed topology information.
///
/// `with_capacity` is only an allocation hint.
///
/// It is never a semantic maximum.
#[derive(Debug, Clone)]
pub struct SpatialModelBuilder {
    id: SpatialModelId,
    resources: BTreeSet<SpatialResource>,
    placements: BTreeMap<SpatialResource, SpatialCoordinate>,
    edges: BTreeMap<SpatialEdgeKey, SpatialEdge>,
    kernel: Option<SpatialKernel>,
}

impl SpatialModelBuilder {
    /// Creates an empty spatial-model builder.
    #[must_use]
    pub fn new(id: SpatialModelId) -> Self {
        Self {
            id,
            resources: BTreeSet::new(),
            placements: BTreeMap::new(),
            edges: BTreeMap::new(),
            kernel: None,
        }
    }

    /// Creates a builder with allocation hints.
    ///
    /// The hints do not impose semantic limits.
    #[must_use]
    pub fn with_capacity(
        id: SpatialModelId,
        resource_capacity: usize,
        edge_capacity: usize,
    ) -> Self {
        let _ = resource_capacity;
        let _ = edge_capacity;

        Self::new(id)
    }

    /// Adds a resource.
    ///
    /// Duplicate resources are rejected.
    pub fn add_resource(
        &mut self,
        resource: SpatialResource,
    ) -> Result<(), SpatialError> {
        if !self.resources.insert(resource.clone()) {
            return Err(SpatialError::DuplicateResource { resource });
        }

        Ok(())
    }

    /// Adds many resources incrementally.
    ///
    /// The first validation failure stops the operation.
    pub fn add_resources<I>(
        &mut self,
        resources: I,
    ) -> Result<(), SpatialError>
    where
        I: IntoIterator<Item = SpatialResource>,
    {
        for resource in resources {
            self.add_resource(resource)?;
        }

        Ok(())
    }

    /// Adds or replaces a coordinate for an existing resource.
    ///
    /// Replacing a coordinate is explicit and therefore does not silently
    /// alter topology semantics.
    pub fn set_coordinate(
        &mut self,
        resource: SpatialResource,
        coordinate: SpatialCoordinate,
    ) -> Result<(), SpatialError> {
        if !self.resources.contains(&resource) {
            return Err(SpatialError::UnknownResource { resource });
        }

        self.placements.insert(resource, coordinate);

        Ok(())
    }

    /// Adds an undirected edge.
    pub fn add_undirected_edge(
        &mut self,
        first: SpatialResource,
        second: SpatialResource,
    ) -> Result<(), SpatialError> {
        self.add_edge(SpatialEdge::undirected(first, second)?)
    }

    /// Adds a directed edge.
    pub fn add_directed_edge(
        &mut self,
        source: SpatialResource,
        target: SpatialResource,
    ) -> Result<(), SpatialError> {
        self.add_edge(SpatialEdge::directed(source, target)?)
    }

    /// Adds an arbitrary validated edge.
    pub fn add_edge(
        &mut self,
        edge: SpatialEdge,
    ) -> Result<(), SpatialError> {
        if !self.resources.contains(edge.source()) {
            return Err(SpatialError::UnknownResource {
                resource: edge.source().clone(),
            });
        }

        if !self.resources.contains(edge.target()) {
            return Err(SpatialError::UnknownResource {
                resource: edge.target().clone(),
            });
        }

        let key = SpatialEdgeKey::from(&edge);

        if self.edges.contains_key(&key) {
            return Err(SpatialError::DuplicateEdge {
                source: edge.source().clone(),
                target: edge.target().clone(),
                direction: edge.direction(),
            });
        }

        self.edges.insert(key, edge);

        Ok(())
    }

    /// Sets the spatial kernel.
    pub fn set_kernel(
        &mut self,
        kernel: SpatialKernel,
    ) -> Result<(), SpatialError> {
        validate_kernel(kernel)?;
        self.kernel = Some(kernel);
        Ok(())
    }

    /// Removes the configured kernel.
    pub fn clear_kernel(&mut self) {
        self.kernel = None;
    }

    /// Returns the number of resources currently registered.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Returns the number of edges currently registered.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Validates the builder without consuming it.
    pub fn validate(&self) -> Result<(), SpatialError> {
        let model = self.clone().build()?;
        model.validate()
    }

    /// Consumes the builder and creates an immutable spatial model.
    pub fn build(self) -> Result<SpatialModel, SpatialError> {
        let model = SpatialModel {
            id: self.id,
            resources: self.resources,
            placements: self.placements,
            edges: self.edges,
            kernel: self.kernel,
        };

        model.validate()?;

        Ok(model)
    }
}

/// Spatial error vocabulary.
///
/// This module deliberately keeps spatial-specific errors local so that it can
/// be completed independently. Higher ZQN layers may convert these into their
/// canonical error taxonomy at integration boundaries.
#[derive(Debug, Clone, PartialEq)]
pub enum SpatialError {
    /// Resource identifier is empty.
    EmptyExternalResourceId,

    /// A resource was inserted more than once.
    DuplicateResource {
        resource: SpatialResource,
    },

    /// An edge was inserted more than once.
    DuplicateEdge {
        source: SpatialResource,
        target: SpatialResource,
        direction: SpatialDirection,
    },

    /// An edge connects a resource to itself.
    SelfRelationship {
        resource: SpatialResource,
    },

    /// An edge references a resource absent from the model.
    UnknownResource {
        resource: SpatialResource,
    },

    /// Coordinates have different dimensionalities.
    DimensionMismatch {
        left: usize,
        right: usize,
    },

    /// Coordinate contains NaN or infinity.
    NonFiniteCoordinate,

    /// Numerical calculation overflowed or became non-finite.
    NumericalOverflow,

    /// A numerical parameter was not finite.
    NonFiniteValue {
        field: &'static str,
    },

    /// A numerical parameter was negative.
    NegativeValue {
        field: &'static str,
        value: f64,
    },

    /// A strictly positive parameter was zero or negative.
    NonPositiveValue {
        field: &'static str,
        value: f64,
    },
}

impl fmt::Display for SpatialError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyExternalResourceId => {
                write!(formatter, "external spatial resource ID is empty")
            }

            Self::DuplicateResource { resource } => {
                write!(formatter, "duplicate spatial resource: {resource}")
            }

            Self::DuplicateEdge {
                source,
                target,
                direction,
            } => {
                write!(
                    formatter,
                    "duplicate spatial edge: {source} -> {target} ({direction:?})"
                )
            }

            Self::SelfRelationship { resource } => {
                write!(
                    formatter,
                    "spatial self-relationship is not permitted: {resource}"
                )
            }

            Self::UnknownResource { resource } => {
                write!(formatter, "unknown spatial resource: {resource}")
            }

            Self::DimensionMismatch { left, right } => {
                write!(
                    formatter,
                    "spatial coordinate dimension mismatch: {left} != {right}"
                )
            }

            Self::NonFiniteCoordinate => {
                write!(formatter, "spatial coordinate contains a non-finite value")
            }

            Self::NumericalOverflow => {
                write!(formatter, "spatial numerical operation overflowed")
            }

            Self::NonFiniteValue { field } => {
                write!(formatter, "{field} must be finite")
            }

            Self::NegativeValue { field, value } => {
                write!(formatter, "{field} must be non-negative; got {value}")
            }

            Self::NonPositiveValue { field, value } => {
                write!(formatter, "{field} must be positive; got {value}")
            }
        }
    }
}

impl std::error::Error for SpatialError {}

fn validate_finite(value: f64, field: &'static str) -> Result<(), SpatialError> {
    if !value.is_finite() {
        return Err(SpatialError::NonFiniteValue { field });
    }

    Ok(())
}

fn validate_non_negative_finite(
    value: f64,
    field: &'static str,
) -> Result<(), SpatialError> {
    validate_finite(value, field)?;

    if value < 0.0 {
        return Err(SpatialError::NegativeValue { field, value });
    }

    Ok(())
}

fn validate_positive_finite(
    value: f64,
    field: &'static str,
) -> Result<(), SpatialError> {
    validate_finite(value, field)?;

    if value <= 0.0 {
        return Err(SpatialError::NonPositiveValue { field, value });
    }

    Ok(())
}

fn validate_kernel(kernel: SpatialKernel) -> Result<(), SpatialError> {
    match kernel {
        SpatialKernel::Exponential { length_scale }
        | SpatialKernel::Gaussian { length_scale } => {
            validate_positive_finite(length_scale, "length_scale")
        }

        SpatialKernel::PowerLaw {
            length_scale,
            power,
        } => {
            validate_positive_finite(length_scale, "length_scale")?;
            validate_positive_finite(power, "power")
        }

        SpatialKernel::Constant { value } => {
            validate_finite(value, "value")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn logical(index: usize) -> SpatialResource {
        SpatialResource::LogicalQubit(QubitId::new(index))
    }

    fn physical(index: usize) -> SpatialResource {
        SpatialResource::PhysicalQubit(PhysicalQubitId::new(index))
    }

    #[test]
    fn canonical_qubit_identity_is_used() {
        let logical = QubitId::new(7);
        let physical = PhysicalQubitId::new(11);

        let logical_resource = SpatialResource::from(logical);
        let physical_resource = SpatialResource::from(physical);

        assert_eq!(logical_resource.logical_qubit(), Some(logical));
        assert_eq!(physical_resource.physical_qubit(), Some(physical));
        assert_ne!(logical_resource, physical_resource);
    }

    #[test]
    fn logical_and_physical_same_index_remain_distinct() {
        let logical = logical(5);
        let physical = physical(5);

        assert_ne!(logical, physical);
    }

    #[test]
    fn external_resource_requires_non_empty_identifier() {
        assert!(SpatialResource::external("").is_err());
        assert!(SpatialResource::external("mode-7").is_ok());
    }

    #[test]
    fn coordinates_reject_non_finite_values() {
        assert!(SpatialCoordinate::new([0.0, 1.0, f64::NAN]).is_err());
        assert!(SpatialCoordinate::new([0.0, f64::INFINITY]).is_err());
        assert!(SpatialCoordinate::new([0.0, 1.0]).is_ok());
    }

    #[test]
    fn coordinates_compute_euclidean_distance() {
        let a = SpatialCoordinate::new([0.0, 0.0]).expect("valid");
        let b = SpatialCoordinate::new([3.0, 4.0]).expect("valid");

        let distance = a.euclidean_distance(&b).expect("valid distance");

        assert_eq!(distance, 5.0);
    }

    #[test]
    fn coordinate_dimensions_must_match() {
        let a = SpatialCoordinate::new([0.0]).expect("valid");
        let b = SpatialCoordinate::new([0.0, 1.0]).expect("valid");

        assert!(matches!(
            a.euclidean_distance(&b),
            Err(SpatialError::DimensionMismatch {
                left: 1,
                right: 2
            })
        ));
    }

    #[test]
    fn self_edges_are_rejected() {
        let resource = logical(0);

        assert!(SpatialEdge::undirected(resource.clone(), resource).is_err());
    }

    #[test]
    fn undirected_edges_are_canonicalized() {
        let a = logical(1);
        let b = logical(2);

        let first =
            SpatialEdge::undirected(a.clone(), b.clone()).expect("valid");
        let second =
            SpatialEdge::undirected(b, a).expect("valid");

        assert_eq!(first.source(), second.source());
        assert_eq!(first.target(), second.target());
    }

    #[test]
    fn directed_edges_preserve_direction() {
        let a = logical(1);
        let b = logical(2);

        let forward =
            SpatialEdge::directed(a.clone(), b.clone()).expect("valid");
        let reverse =
            SpatialEdge::directed(b, a).expect("valid");

        assert_ne!(forward.source(), reverse.source());
        assert_ne!(forward.target(), reverse.target());
    }

    #[test]
    fn model_requires_registered_resources() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder.add_resource(a.clone()).expect("resource");

        assert!(builder.add_undirected_edge(a, b).is_err());
    }

    #[test]
    fn duplicate_edges_are_rejected() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        builder
            .add_undirected_edge(a.clone(), b.clone())
            .expect("edge");

        assert!(builder.add_undirected_edge(a, b).is_err());
    }

    #[test]
    fn opposite_undirected_edges_are_the_same_relationship() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        builder
            .add_undirected_edge(a.clone(), b.clone())
            .expect("edge");

        assert!(builder.add_undirected_edge(b, a).is_err());
    }

    #[test]
    fn opposite_directed_edges_can_both_exist() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        builder
            .add_directed_edge(a.clone(), b.clone())
            .expect("forward edge");

        builder
            .add_directed_edge(b, a)
            .expect("reverse edge");
    }

    #[test]
    fn neighbors_are_deterministic() {
        let a = logical(0);
        let b = logical(1);
        let c = logical(2);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone(), c.clone()])
            .expect("resources");

        builder
            .add_undirected_edge(a.clone(), c.clone())
            .expect("edge");

        builder
            .add_undirected_edge(a.clone(), b.clone())
            .expect("edge");

        let model = builder.build().expect("model");

        assert_eq!(model.neighbors(&a), vec![b, c]);
    }

    #[test]
    fn outgoing_and_incoming_direction_are_distinct() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        builder
            .add_directed_edge(a.clone(), b.clone())
            .expect("edge");

        let model = builder.build().expect("model");

        assert_eq!(model.outgoing_neighbors(&a), vec![b.clone()]);
        assert!(model.outgoing_neighbors(&b).is_empty());

        assert_eq!(model.incoming_neighbors(&b), vec![a.clone()]);
        assert!(model.incoming_neighbors(&a).is_empty());
    }

    #[test]
    fn hop_distance_is_deterministic() {
        let a = logical(0);
        let b = logical(1);
        let c = logical(2);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone(), c.clone()])
            .expect("resources");

        builder
            .add_undirected_edge(a.clone(), b.clone())
            .expect("edge");

        builder
            .add_undirected_edge(b.clone(), c.clone())
            .expect("edge");

        let model = builder.build().expect("model");

        assert_eq!(
            model.hop_distance(&a, &c).expect("distance"),
            Some(2)
        );
    }

    #[test]
    fn unreachable_resources_return_none() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        let model = builder.build().expect("model");

        assert_eq!(
            model.hop_distance(&a, &b).expect("distance"),
            None
        );
    }

    #[test]
    fn reachable_within_hops_is_bounded_by_hops() {
        let a = logical(0);
        let b = logical(1);
        let c = logical(2);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone(), c.clone()])
            .expect("resources");

        builder
            .add_undirected_edge(a.clone(), b.clone())
            .expect("edge");

        builder
            .add_undirected_edge(b.clone(), c.clone())
            .expect("edge");

        let model = builder.build().expect("model");

        let one_hop = model
            .reachable_within_hops(&a, 1)
            .expect("traversal");

        assert_eq!(one_hop, vec![a.clone(), b.clone()]);

        let two_hop = model
            .reachable_within_hops(&a, 2)
            .expect("traversal");

        assert_eq!(two_hop, vec![a, b, c]);
    }

    #[test]
    fn connected_components_are_deterministic() {
        let a = logical(0);
        let b = logical(1);
        let c = logical(2);
        let d = logical(3);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone(), c.clone(), d.clone()])
            .expect("resources");

        builder
            .add_undirected_edge(a.clone(), b.clone())
            .expect("edge");

        builder
            .add_undirected_edge(c.clone(), d.clone())
            .expect("edge");

        let model = builder.build().expect("model");

        let components = model.connected_components();

        assert_eq!(
            components,
            vec![vec![a, b], vec![c, d]]
        );
    }

    #[test]
    fn explicit_distance_has_precedence_over_coordinates() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        builder
            .set_coordinate(
                a.clone(),
                SpatialCoordinate::new([0.0, 0.0]).expect("coordinate"),
            )
            .expect("placement");

        builder
            .set_coordinate(
                b.clone(),
                SpatialCoordinate::new([3.0, 4.0]).expect("coordinate"),
            )
            .expect("placement");

        let edge = SpatialEdge::new(
            a.clone(),
            b.clone(),
            SpatialDirection::Undirected,
            Some(10.0),
            None,
        )
        .expect("edge");

        builder.add_edge(edge).expect("edge");

        let model = builder.build().expect("model");

        assert_eq!(
            model.distance(&a, &b).expect("distance"),
            Some(10.0)
        );
    }

    #[test]
    fn coordinates_can_supply_distance_when_edge_has_none() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        builder
            .set_coordinate(
                a.clone(),
                SpatialCoordinate::new([0.0, 0.0]).expect("coordinate"),
            )
            .expect("placement");

        builder
            .set_coordinate(
                b.clone(),
                SpatialCoordinate::new([3.0, 4.0]).expect("coordinate"),
            )
            .expect("placement");

        builder
            .add_undirected_edge(a.clone(), b.clone())
            .expect("edge");

        let model = builder.build().expect("model");

        assert_eq!(
            model.distance(&a, &b).expect("distance"),
            Some(5.0)
        );
    }

    #[test]
    fn exponential_kernel_is_valid() {
        let kernel =
            SpatialKernel::exponential(2.0).expect("valid kernel");

        assert_eq!(kernel.evaluate(0.0).expect("value"), 1.0);
        assert!(kernel.evaluate(2.0).expect("value") < 1.0);
        assert!(kernel.evaluate(4.0).expect("value") > 0.0);
    }

    #[test]
    fn gaussian_kernel_is_valid() {
        let kernel =
            SpatialKernel::gaussian(2.0).expect("valid kernel");

        assert_eq!(kernel.evaluate(0.0).expect("value"), 1.0);
        assert!(kernel.evaluate(2.0).expect("value") < 1.0);
    }

    #[test]
    fn power_law_kernel_is_valid() {
        let kernel =
            SpatialKernel::power_law(2.0, 2.0).expect("valid kernel");

        assert_eq!(kernel.evaluate(0.0).expect("value"), 1.0);
        assert!(kernel.evaluate(2.0).expect("value") < 1.0);
    }

    #[test]
    fn invalid_kernel_parameters_are_rejected() {
        assert!(SpatialKernel::exponential(0.0).is_err());
        assert!(SpatialKernel::exponential(-1.0).is_err());
        assert!(SpatialKernel::gaussian(f64::NAN).is_err());
        assert!(SpatialKernel::power_law(1.0, 0.0).is_err());
        assert!(SpatialKernel::constant(f64::INFINITY).is_err());
    }

    #[test]
    fn spatial_kernel_can_be_integrated_with_coordinate_distance() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        builder
            .set_coordinate(
                a.clone(),
                SpatialCoordinate::new([0.0]).expect("coordinate"),
            )
            .expect("placement");

        builder
            .set_coordinate(
                b.clone(),
                SpatialCoordinate::new([2.0]).expect("coordinate"),
            )
            .expect("placement");

        builder
            .add_undirected_edge(a.clone(), b.clone())
            .expect("edge");

        builder
            .set_kernel(
                SpatialKernel::exponential(1.0)
                    .expect("kernel"),
            )
            .expect("kernel");

        let model = builder.build().expect("model");

        let influence = model
            .spatial_influence(&a, &b)
            .expect("influence")
            .expect("kernel configured");

        assert!(influence > 0.0);
        assert!(influence < 1.0);
    }

    #[test]
    fn model_validation_succeeds_for_valid_model() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(42));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        builder
            .add_undirected_edge(a, b)
            .expect("edge");

        let model = builder.build().expect("model");

        assert!(model.validate().is_ok());
    }

    #[test]
    fn zero_dimensional_coordinates_are_supported() {
        let coordinate = SpatialCoordinate::zero_dimensional();

        assert_eq!(coordinate.dimension(), 0);
        assert!(coordinate.is_zero_dimensional());
        assert_eq!(
            coordinate
                .euclidean_distance(&SpatialCoordinate::zero_dimensional())
                .expect("distance"),
            0.0
        );
    }

    #[test]
    fn allocation_hints_do_not_change_semantics() {
        let id = SpatialModelId::new(7);

        let normal = SpatialModelBuilder::new(id);
        let hinted = SpatialModelBuilder::with_capacity(id, 10_000, 50_000);

        assert_eq!(normal.resource_count(), hinted.resource_count());
        assert_eq!(normal.edge_count(), hinted.edge_count());
    }

    #[test]
    fn model_supports_large_identifier_values_without_machine_limits() {
        let logical_resource =
            SpatialResource::LogicalQubit(QubitId::new(usize::MAX));

        let physical_resource = SpatialResource::PhysicalQubit(
            PhysicalQubitId::new(usize::MAX),
        );

        assert!(logical_resource.is_logical_qubit());
        assert!(physical_resource.is_physical_qubit());
    }

    #[test]
    fn resource_domains_are_type_safe() {
        let logical_resource = logical(7);
        let physical_resource = physical(7);

        assert_ne!(logical_resource, physical_resource);
        assert_eq!(
            logical_resource.logical_qubit(),
            Some(QubitId::new(7))
        );
        assert_eq!(
            physical_resource.physical_qubit(),
            Some(PhysicalQubitId::new(7))
        );
    }

    #[test]
    fn external_resources_are_ordered_deterministically() {
        let a =
            SpatialResource::external("b").expect("resource");
        let b =
            SpatialResource::external("a").expect("resource");

        let mut resources = BTreeSet::new();
        resources.insert(a);
        resources.insert(b);

        let ordered: Vec<_> = resources.into_iter().collect();

        assert_eq!(
            ordered[0],
            SpatialResource::External("a".to_owned())
        );
        assert_eq!(
            ordered[1],
            SpatialResource::External("b".to_owned())
        );
    }

    #[test]
    fn external_and_canonical_domains_do_not_collide() {
        let logical_resource = logical(1);
        let external_resource =
            SpatialResource::external("logical:1").expect("resource");

        assert_ne!(logical_resource, external_resource);
    }

    #[test]
    fn directed_edges_are_not_collapsed_into_undirected_edges() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        builder
            .add_directed_edge(a.clone(), b.clone())
            .expect("directed");

        builder
            .add_undirected_edge(a, b)
            .expect("undirected");

        assert_eq!(builder.edge_count(), 2);
    }

    #[test]
    fn unknown_resource_queries_are_errors_where_semantically_required() {
        let known = logical(0);
        let unknown = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resource(known.clone())
            .expect("resource");

        let model = builder.build().expect("model");

        assert!(matches!(
            model.distance(&known, &unknown),
            Err(SpatialError::UnknownResource { .. })
        ));

        assert!(matches!(
            model.hop_distance(&known, &unknown),
            Err(SpatialError::UnknownResource { .. })
        ));

        assert!(matches!(
            model.reachable_within_hops(&unknown, 1),
            Err(SpatialError::UnknownResource { .. })
        ));
    }

    #[test]
    fn empty_graph_is_valid() {
        let model = SpatialModelBuilder::new(SpatialModelId::new(1))
            .build()
            .expect("empty spatial model");

        assert_eq!(model.resource_count(), 0);
        assert_eq!(model.edge_count(), 0);
        assert!(model.connected_components().is_empty());
    }

    #[test]
    fn disconnected_resources_are_valid() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        let model = builder.build().expect("model");

        assert_eq!(
            model.connected_components(),
            vec![vec![a], vec![b]]
        );
    }

    #[test]
    fn model_is_immutable_after_build() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        builder
            .add_undirected_edge(a, b)
            .expect("edge");

        let model = builder.build().expect("model");

        assert_eq!(model.resource_count(), 2);
        assert_eq!(model.edge_count(), 1);
    }

    #[test]
    fn spatial_influence_without_kernel_is_none() {
        let a = logical(0);
        let b = logical(1);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder
            .add_resources([a.clone(), b.clone()])
            .expect("resources");

        builder
            .set_coordinate(
                a.clone(),
                SpatialCoordinate::new([0.0]).expect("coordinate"),
            )
            .expect("placement");

        builder
            .set_coordinate(
                b.clone(),
                SpatialCoordinate::new([1.0]).expect("coordinate"),
            )
            .expect("placement");

        let model = builder.build().expect("model");

        assert_eq!(
            model.spatial_influence(&a, &b).expect("query"),
            None
        );
    }

    #[test]
    fn distance_between_same_resource_is_zero() {
        let a = logical(0);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder.add_resource(a.clone()).expect("resource");

        let model = builder.build().expect("model");

        assert_eq!(
            model.distance(&a, &a).expect("distance"),
            Some(0.0)
        );
    }

    #[test]
    fn finite_negative_distance_is_rejected() {
        let a = logical(0);
        let b = logical(1);

        assert!(SpatialEdge::new(
            a,
            b,
            SpatialDirection::Undirected,
            Some(-1.0),
            None,
        )
        .is_err());
    }

    #[test]
    fn non_finite_influence_is_rejected() {
        let a = logical(0);
        let b = logical(1);

        assert!(SpatialEdge::new(
            a,
            b,
            SpatialDirection::Undirected,
            None,
            Some(f64::NAN),
        )
        .is_err());
    }

    #[test]
    fn model_id_is_not_resource_identity() {
        let id = SpatialModelId::new(123);

        let resource = logical(123);

        assert_eq!(id.value(), 123);
        assert_eq!(
            resource.logical_qubit(),
            Some(QubitId::new(123))
        );
    }

    #[test]
    fn traversal_is_stable_under_insertion_order() {
        let a = logical(0);
        let b = logical(1);
        let c = logical(2);
        let d = logical(3);

        let mut first = SpatialModelBuilder::new(SpatialModelId::new(1));

        first
            .add_resources([a.clone(), b.clone(), c.clone(), d.clone()])
            .expect("resources");

        first
            .add_undirected_edge(a.clone(), d.clone())
            .expect("edge");

        first
            .add_undirected_edge(a.clone(), b.clone())
            .expect("edge");

        first
            .add_undirected_edge(b.clone(), c.clone())
            .expect("edge");

        let mut second = SpatialModelBuilder::new(SpatialModelId::new(1));

        second
            .add_resources([d.clone(), c.clone(), b.clone(), a.clone()])
            .expect("resources");

        second
            .add_undirected_edge(b.clone(), c.clone())
            .expect("edge");

        second
            .add_undirected_edge(a.clone(), b.clone())
            .expect("edge");

        second
            .add_undirected_edge(a.clone(), d.clone())
            .expect("edge");

        let first_model = first.build().expect("model");
        let second_model = second.build().expect("model");

        assert_eq!(
            first_model.connected_components(),
            second_model.connected_components()
        );

        assert_eq!(
            first_model.neighbors(&a),
            second_model.neighbors(&a)
        );
    }

    #[test]
    fn zero_hop_reachability_returns_start_only() {
        let a = logical(0);

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(1));

        builder.add_resource(a.clone()).expect("resource");

        let model = builder.build().expect("model");

        assert_eq!(
            model.reachable_within_hops(&a, 0).expect("traversal"),
            vec![a]
        );
    }

    #[test]
    fn model_supports_external_future_resources() {
        let mode_a =
            SpatialResource::external("photonic.mode.0")
                .expect("resource");

        let mode_b =
            SpatialResource::external("photonic.mode.1")
                .expect("resource");

        let mut builder = SpatialModelBuilder::new(SpatialModelId::new(9));

        builder
            .add_resources([mode_a.clone(), mode_b.clone()])
            .expect("resources");

        builder
            .add_undirected_edge(mode_a.clone(), mode_b.clone())
            .expect("edge");

        let model = builder.build().expect("model");

        assert_eq!(model.neighbors(&mode_a), vec![mode_b]);
    }
}