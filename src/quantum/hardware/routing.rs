//! Zamani Quantum — Hardware Routing Contract
//!
//! Production-grade, provider-neutral physical routing primitives for
//! `crate::quantum::hardware`.
//!
//! # Responsibility
//!
//! This module answers:
//!
//! - Can a physical resource be reached from another resource?
//! - Which physical path should be used under a selected routing policy?
//! - Does a route respect directed native connectivity?
//! - What is the deterministic cost of a physical route?
//! - What constraints must a higher-level circuit router obey?
//! - Which physical edges/resources participate in a route?
//! - What routing overhead will a route introduce?
//! - Can several independent physical routes be represented together?
//!
//! This module is the hardware-facing routing contract.
//!
//! # Explicit non-responsibilities
//!
//! This module does NOT:
//!
//! - parse Zamani source;
//! - manipulate Zamani source;
//! - own the canonical Quantum IR;
//! - perform logical-to-physical layout selection;
//! - rewrite quantum circuits;
//! - insert SWAP instructions into circuits;
//! - decompose SWAP gates;
//! - translate gates to native instructions;
//! - schedule operations;
//! - acquire calibration data;
//! - communicate with providers;
//! - authenticate providers;
//! - store credentials;
//! - execute quantum programs;
//! - perform benchmarking;
//! - perform QEC;
//! - depend on IBM/IonQ/AWS/Rigetti/IQM/Quantinuum/etc.
//!
//! The existing `crate::quantum::routing::transpiler` remains responsible for
//! circuit-level logical-to-physical transformation and SWAP insertion.
//!
//! This module supplies the physical routing information that such a higher
//! level compiler can consume.
//!
//! # Architectural position
//!
//! ```text
//!                    Zamani Quantum IR
//!                           |
//!                           v
//!                  quantum::routing
//!                           |
//!                logical/physical mapping
//!                           |
//!                           v
//!              hardware::routing   <--- this module
//!                           |
//!             +-------------+-------------+
//!             |             |             |
//!             v             v             v
//!         topology     capabilities   calibration
//!             |             |             |
//!             +-------------+-------------+
//!                           |
//!                           v
//!                     backend / adapter
//!                           |
//!                           v
//!                         QPU
//! ```
//!
//! # Dependency direction
//!
//! This module depends only on the canonical topology model:
//!
//! ```text
//! hardware::routing
//!        |
//!        v
//! hardware::topology
//! ```
//!
//! It intentionally does not depend on backend.rs, validation.rs,
//! calibration.rs, instruction_set.rs, scheduling.rs, benchmarking, or
//! provider adapters.
//!
//! Those later modules can integrate with this file through the public
//! traits defined here without requiring this file to be modified.
//!
//! This is deliberate: the routing contract must be independently complete
//! and stable before higher-level hardware modules are implemented.
//!
//! # Integration contract
//!
//! Future modules integrate as follows:
//!
//! - `topology.rs` supplies physical connectivity.
//! - `calibration.rs` may implement `EdgeCostModel`.
//! - `instruction_set.rs` may implement `EdgeConstraintModel`.
//! - `compatibility.rs` may consume `RoutingRequirements` and `RoutingPlan`.
//! - `validation.rs` may consume route feasibility.
//! - `backend.rs` may expose routing constraints as backend metadata.
//! - `quantum::routing` may consume `RoutePlan` when inserting physical
//!   routing operations.
//! - `quantum::scheduling` may consume `RouteStep`/`RouteEdge` information.
//! - benchmarking may record routing overhead and route fingerprints.
//! - provider adapters may translate physical resource IDs to provider IDs.
//!
//! No downstream module needs to modify this file merely because it begins
//! implementing one of those integrations.
//!
//! # Design principles
//!
//! 1. Physical topology is authoritative.
//! 2. Native direction is never silently ignored.
//! 3. Undirected traversal is explicitly opt-in.
//! 4. Routing is deterministic.
//! 5. Cost models are injectable.
//! 6. Calibration-aware routing can be added without changing this file.
//! 7. Instruction-aware routing can be added without changing this file.
//! 8. Routing results are immutable after construction.
//! 9. Failed routing never returns a partial successful route.
//! 10. Numeric costs are finite integer values rather than floating-point
//!     values, avoiding NaN/∞ ordering hazards.
//! 11. No provider-specific identifiers leak into the core contract.
//! 12. No routing result claims that a gate is executable merely because a
//!     physical path exists.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features are required.
//! No external crates are required.
//!
//! # Safety
//!
//! Unsafe Rust is forbidden.
//!
//! ```text
//! #![deny(unsafe_code)]
//! #![deny(unsafe_op_in_unsafe_fn)]
//! #![deny(unused_must_use)]
//! ```
//!
//! # Important semantic distinction
//!
//! A topology path means:
//!
//! > Physical resources are connected according to the selected traversal
//! > semantics.
//!
//! It does NOT mean:
//!
//! > The requested quantum instruction is executable along every edge.
//!
//! Instruction support belongs to the instruction/capability subsystem.
//!
//! Likewise, a shortest path is not necessarily the lowest-error or fastest
//! path. Those policies are represented through `EdgeCostModel`.
//!
//! Current quantum compiler ecosystems make the same conceptual separation:
//! hardware connectivity, layout, routing, instruction translation and
//! scheduling are separate compilation concerns.
//!
//! -----------------------------------------------------------------------------
//! Schema
//! -----------------------------------------------------------------------------

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};
use std::fmt;

use super::topology::{
    Connectivity,
    Coupling,
    HardwareTopology,
    ResourceId,
};

// =============================================================================
// Schema
// =============================================================================

/// Stable schema identifier for hardware routing.
pub const ROUTING_SCHEMA_ID: &str = "zamani.quantum.hardware.routing";

/// Semantic version of the routing contract.
///
/// Increment only when the meaning of serialized/public routing semantics
/// changes incompatibly.
pub const ROUTING_SCHEMA_VERSION: u16 = 1;

/// Maximum number of route edges accepted by one route request.
///
/// This is a defensive bound against accidental or malicious requests on
/// pathological inputs. It does not limit the size of the hardware topology.
pub const DEFAULT_MAX_ROUTE_EDGES: usize = 1_000_000;

/// Maximum number of route candidates returned by a multi-route request.
pub const DEFAULT_MAX_ROUTES: usize = 100_000;

// =============================================================================
// Routing direction
// =============================================================================

/// Direction semantics used while searching a physical route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RoutingDirection {
    /// Respect native hardware coupling direction.
    ///
    /// A directed coupling `A -> B` can be traversed from A to B but not
    /// from B to A.
    Native,

    /// Treat physical couplings as undirected physical adjacency.
    ///
    /// This mode is appropriate for physical-distance analysis and
    /// movement planning where the caller will later determine how an
    /// operation is implemented.
    ///
    /// It MUST NOT be interpreted as proof that a directed native gate can
    /// execute in the reverse direction.
    Physical,
}

impl Default for RoutingDirection {
    fn default() -> Self {
        Self::Native
    }
}

impl RoutingDirection {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Physical => "physical",
        }
    }

    /// Returns whether native directionality is enforced.
    pub const fn respects_native_direction(self) -> bool {
        matches!(self, Self::Native)
    }
}

impl fmt::Display for RoutingDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Routing objective
// =============================================================================

/// Primary objective used by a routing search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RoutingObjective {
    /// Minimize the number of physical edges.
    ///
    /// This is the default deterministic shortest-path objective.
    HopCount,

    /// Minimize a provider/calibration supplied integer cost.
    WeightedCost,

    /// Minimize weighted cost and use hop count as the first tie breaker.
    WeightedThenHops,
}

impl Default for RoutingObjective {
    fn default() -> Self {
        Self::HopCount
    }
}

impl RoutingObjective {
    /// Stable machine-readable representation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HopCount => "hop_count",
            Self::WeightedCost => "weighted_cost",
            Self::WeightedThenHops => "weighted_then_hops",
        }
    }
}

// =============================================================================
// Routing cost
// =============================================================================

/// Deterministic integer cost assigned to one physical routing edge.
///
/// Integer costs are intentional. Floating-point costs introduce NaN,
/// infinity, rounding and platform-dependent ordering hazards into a
/// deterministic compiler subsystem.
///
/// A calibration subsystem may convert physical values such as error rate,
/// duration or infidelity into a fixed-point integer before returning them
/// here.
///
/// # Ordering
///
/// Costs are lexicographically ordered:
///
/// 1. `primary`
/// 2. `secondary`
/// 3. `tertiary`
/// 4. `hops`
///
/// This makes tie-breaking deterministic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RoutingCost {
    /// Primary optimization value.
    pub primary: u64,

    /// Secondary optimization value.
    pub secondary: u64,

    /// Tertiary optimization value.
    pub tertiary: u64,

    /// Number of physical edges represented by this accumulated cost.
    pub hops: u64,
}

impl RoutingCost {
    /// Creates a zero cost.
    pub const fn zero() -> Self {
        Self {
            primary: 0,
            secondary: 0,
            tertiary: 0,
            hops: 0,
        }
    }

    /// Creates a one-hop structural cost.
    pub const fn one_hop() -> Self {
        Self {
            primary: 1,
            secondary: 0,
            tertiary: 0,
            hops: 1,
        }
    }

    /// Creates a weighted edge cost.
    pub const fn weighted(primary: u64) -> Self {
        Self {
            primary,
            secondary: 0,
            tertiary: 0,
            hops: 1,
        }
    }

    /// Adds another edge cost using checked arithmetic.
    ///
    /// Returns `None` on overflow.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        let primary = match self.primary.checked_add(rhs.primary) {
            Some(value) => value,
            None => return None,
        };

        let secondary = match self.secondary.checked_add(rhs.secondary) {
            Some(value) => value,
            None => return None,
        };

        let tertiary = match self.tertiary.checked_add(rhs.tertiary) {
            Some(value) => value,
            None => return None,
        };

        let hops = match self.hops.checked_add(rhs.hops) {
            Some(value) => value,
            None => return None,
        };

        Some(Self {
            primary,
            secondary,
            tertiary,
            hops,
        })
    }

    /// Returns a cost suitable for the hop-count objective.
    pub const fn hop_count(self) -> u64 {
        self.hops
    }
}

impl Ord for RoutingCost {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.primary,
            self.secondary,
            self.tertiary,
            self.hops,
        )
            .cmp(&(
                other.primary,
                other.secondary,
                other.tertiary,
                other.hops,
            ))
    }
}

impl PartialOrd for RoutingCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// Edge cost model
// =============================================================================

/// Supplies deterministic routing costs for physical edges.
///
/// This trait is intentionally independent of calibration and instruction
/// modules. Future modules can implement it without modifying `routing.rs`.
///
/// # Example future integrations
///
/// ```text
/// calibration.rs
///     └── implements EdgeCostModel
///
/// instruction_set.rs
///     └── implements EdgeConstraintModel
///
/// topology.rs
///     └── provides physical connectivity
/// ```
pub trait EdgeCostModel {
    /// Returns the cost of traversing the physical edge.
    ///
    /// `source` and `target` are the actual traversal direction, not merely
    /// an unordered physical pair.
    ///
    /// Returning `None` means the edge is not routable under this model.
    fn edge_cost(
        &self,
        topology: &HardwareTopology,
        source: ResourceId,
        target: ResourceId,
    ) -> Option<RoutingCost>;
}

/// Default structural cost model.
///
/// Every permitted physical edge has equal cost.
#[derive(Debug, Clone, Copy, Default)]
pub struct HopCostModel;

impl EdgeCostModel for HopCostModel {
    fn edge_cost(
        &self,
        _topology: &HardwareTopology,
        _source: ResourceId,
        _target: ResourceId,
    ) -> Option<RoutingCost> {
        Some(RoutingCost::one_hop())
    }
}

// =============================================================================
// Edge constraints
// =============================================================================

/// Optional hardware-specific constraints over route edges.
///
/// This trait allows future instruction-set and capability modules to reject
/// edges without changing this routing contract.
///
/// Returning `true` means the edge is allowed.
///
/// Returning `false` means the edge must not be used.
pub trait EdgeConstraintModel {
    /// Determines whether a route may traverse an edge.
    fn allows_edge(
        &self,
        topology: &HardwareTopology,
        source: ResourceId,
        target: ResourceId,
    ) -> bool;
}

/// Constraint model that permits every topologically valid edge.
#[derive(Debug, Clone, Copy, Default)]
pub struct AllowAllEdges;

impl EdgeConstraintModel for AllowAllEdges {
    fn allows_edge(
        &self,
        _topology: &HardwareTopology,
        _source: ResourceId,
        _target: ResourceId,
    ) -> bool {
        true
    }
}

// =============================================================================
// Routing policy
// =============================================================================

/// Immutable routing policy.
///
/// The policy is deliberately independent of circuit/IR types so it can be
/// reused by:
///
/// - circuit routing;
/// - hardware discovery;
/// - resource estimation;
/// - benchmark preparation;
/// - topology analysis;
/// - future distributed quantum routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoutingPolicy {
    /// Whether native edge direction must be respected.
    pub direction: RoutingDirection,

    /// Primary route objective.
    pub objective: RoutingObjective,

    /// Maximum permitted route edges.
    pub max_route_edges: usize,

    /// Whether a zero-length source-to-source route is allowed.
    pub allow_trivial_route: bool,
}

impl Default for RoutingPolicy {
    fn default() -> Self {
        Self {
            direction: RoutingDirection::Native,
            objective: RoutingObjective::HopCount,
            max_route_edges: DEFAULT_MAX_ROUTE_EDGES,
            allow_trivial_route: true,
        }
    }
}

impl RoutingPolicy {
    /// Creates the default production policy.
    pub const fn new() -> Self {
        Self {
            direction: RoutingDirection::Native,
            objective: RoutingObjective::HopCount,
            max_route_edges: DEFAULT_MAX_ROUTE_EDGES,
            allow_trivial_route: true,
        }
    }

    /// Creates a native shortest-path policy.
    pub const fn native_shortest_path() -> Self {
        Self::new()
    }

    /// Creates a physical-connectivity shortest-path policy.
    pub const fn physical_shortest_path() -> Self {
        Self {
            direction: RoutingDirection::Physical,
            ..Self::new()
        }
    }

    /// Uses weighted routing while preserving native directionality.
    pub const fn native_weighted() -> Self {
        Self {
            direction: RoutingDirection::Native,
            objective: RoutingObjective::WeightedCost,
            ..Self::new()
        }
    }

    /// Validates policy configuration.
    pub fn validate(&self) -> Result<(), RoutingError> {
        if self.max_route_edges == 0 {
            return Err(RoutingError::InvalidPolicy {
                message: "max_route_edges must be greater than zero".to_string(),
            });
        }

        Ok(())
    }
}

// =============================================================================
// Routing request
// =============================================================================

/// Request to route between two physical resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoutingRequest {
    /// Starting physical resource.
    pub source: ResourceId,

    /// Destination physical resource.
    pub target: ResourceId,

    /// Routing policy.
    pub policy: RoutingPolicy,
}

impl RoutingRequest {
    /// Creates a request using the default native shortest-path policy.
    pub const fn new(source: ResourceId, target: ResourceId) -> Self {
        Self {
            source,
            target,
            policy: RoutingPolicy::new(),
        }
    }

    /// Creates a request with an explicit policy.
    pub const fn with_policy(
        source: ResourceId,
        target: ResourceId,
        policy: RoutingPolicy,
    ) -> Self {
        Self {
            source,
            target,
            policy,
        }
    }
}

// =============================================================================
// Route edge
// =============================================================================

/// One physical traversal in a route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RouteEdge {
    /// Physical source.
    pub source: ResourceId,

    /// Physical target.
    pub target: ResourceId,

    /// Whether the topology declares the native traversal directly.
    pub native_direction: bool,

    /// Whether this traversal uses a bidirectional coupling.
    pub bidirectional_coupling: bool,
}

impl RouteEdge {
    /// Creates a route edge from a coupling and traversal direction.
    pub const fn from_coupling(
        coupling: Coupling,
        source: ResourceId,
        target: ResourceId,
    ) -> Option<Self> {
        if !coupling.permits_native_direction(source, target)
            && coupling.connectivity == Connectivity::Directed
        {
            return None;
        }

        let native_direction =
            coupling.permits_native_direction(source, target);

        Some(Self {
            source,
            target,
            native_direction,
            bidirectional_coupling: matches!(
                coupling.connectivity,
                Connectivity::Bidirectional
            ),
        })
    }

    /// Returns the unordered physical pair.
    pub const fn physical_pair(self) -> (ResourceId, ResourceId) {
        if self.source <= self.target {
            (self.source, self.target)
        } else {
            (self.target, self.source)
        }
    }
}

// =============================================================================
// Route plan
// =============================================================================

/// Immutable physical route produced by the routing engine.
///
/// The first resource is always `source`.
/// The last resource is always `target`.
///
/// For a non-trivial route with N resources, there are N-1 route edges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutePlan {
    /// Routing schema version.
    pub schema_version: u16,

    /// Stable schema identifier.
    pub schema_id: &'static str,

    /// Starting physical resource.
    pub source: ResourceId,

    /// Destination physical resource.
    pub target: ResourceId,

    /// Resources visited in order.
    resources: Vec<ResourceId>,

    /// Physical route edges.
    edges: Vec<RouteEdge>,

    /// Accumulated deterministic routing cost.
    pub cost: RoutingCost,

    /// Direction semantics used by the search.
    pub direction: RoutingDirection,

    /// Objective used by the search.
    pub objective: RoutingObjective,
}

impl RoutePlan {
    /// Creates a route plan after checking all structural invariants.
    fn new(
        source: ResourceId,
        target: ResourceId,
        resources: Vec<ResourceId>,
        edges: Vec<RouteEdge>,
        cost: RoutingCost,
        direction: RoutingDirection,
        objective: RoutingObjective,
    ) -> Result<Self, RoutingError> {
        if resources.is_empty() {
            return Err(RoutingError::InvalidRoute {
                message: "route must contain at least one resource".to_string(),
            });
        }

        if resources[0] != source {
            return Err(RoutingError::InvalidRoute {
                message: "route does not begin at source".to_string(),
            });
        }

        if *resources.last().unwrap_or(&source) != target {
            return Err(RoutingError::InvalidRoute {
                message: "route does not terminate at target".to_string(),
            });
        }

        if resources.len() != edges.len().saturating_add(1) {
            return Err(RoutingError::InvalidRoute {
                message: "resource/edge cardinality mismatch".to_string(),
            });
        }

        for index in 0..edges.len() {
            if edges[index].source != resources[index]
                || edges[index].target != resources[index + 1]
            {
                return Err(RoutingError::InvalidRoute {
                    message: "route edge does not match resource sequence"
                        .to_string(),
                });
            }
        }

        if cost.hops != edges.len() as u64 {
            return Err(RoutingError::InvalidRoute {
                message: "route cost hop count does not match route length"
                    .to_string(),
            });
        }

        Ok(Self {
            schema_version: ROUTING_SCHEMA_VERSION,
            schema_id: ROUTING_SCHEMA_ID,
            source,
            target,
            resources,
            edges,
            cost,
            direction,
            objective,
        })
    }

    /// Returns all physical resources in traversal order.
    pub fn resources(&self) -> &[ResourceId] {
        &self.resources
    }

    /// Returns all physical edges in traversal order.
    pub fn edges(&self) -> &[RouteEdge] {
        &self.edges
    }

    /// Returns the number of physical hops.
    pub fn hop_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns whether this is a source-to-source route.
    pub fn is_trivial(&self) -> bool {
        self.source == self.target
    }

    /// Returns whether this route contains at least one physical traversal.
    pub fn is_nontrivial(&self) -> bool {
        !self.is_trivial()
    }

    /// Returns whether the route visits a physical resource.
    pub fn contains_resource(&self, resource: ResourceId) -> bool {
        self.resources.contains(&resource)
    }

    /// Returns the ordered physical edge pairs.
    pub fn edge_pairs(&self) -> impl Iterator<Item = (ResourceId, ResourceId)> + '_ {
        self.edges
            .iter()
            .map(|edge| (edge.source, edge.target))
    }

    /// Returns a stable route fingerprint represented as a deterministic
    /// hexadecimal string.
    ///
    /// This is deliberately a local deterministic fingerprint, not a
    /// cryptographic identity or security primitive.
    pub fn fingerprint(&self) -> String {
        let mut hash: u64 = 0xcbf29ce484222325;

        fn mix(hash: &mut u64, value: u64) {
            *hash ^= value;
            *hash = hash.wrapping_mul(0x100000001b3);
        }

        mix(&mut hash, self.source as u64);
        mix(&mut hash, self.target as u64);
        mix(&mut hash, self.direction as u64);
        mix(&mut hash, self.objective as u64);

        for resource in &self.resources {
            mix(&mut hash, *resource as u64);
        }

        format!("{hash:016x}")
    }
}

// =============================================================================
// Routing errors
// =============================================================================

/// Errors produced by the hardware routing subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingError {
    /// Topology is invalid.
    InvalidTopology(String),

    /// Source resource does not exist.
    InvalidSource {
        source: ResourceId,
        resource_count: usize,
    },

    /// Target resource does not exist.
    InvalidTarget {
        target: ResourceId,
        resource_count: usize,
    },

    /// Source equals target while trivial routes are disabled.
    TrivialRouteDisabled {
        resource: ResourceId,
    },

    /// No route exists under the selected constraints.
    NoRoute {
        source: ResourceId,
        target: ResourceId,
        direction: RoutingDirection,
    },

    /// The route exceeds the configured maximum length.
    RouteTooLong {
        source: ResourceId,
        target: ResourceId,
        hops: usize,
        maximum: usize,
    },

    /// An edge cost overflowed.
    CostOverflow {
        source: ResourceId,
        target: ResourceId,
    },

    /// A cost model rejected an otherwise topologically valid edge.
    EdgeRejected {
        source: ResourceId,
        target: ResourceId,
    },

    /// Policy configuration is invalid.
    InvalidPolicy {
        message: String,
    },

    /// A route invariant was violated.
    InvalidRoute {
        message: String,
    },

    /// A multi-route request exceeded its configured bound.
    TooManyRoutes {
        requested: usize,
        maximum: usize,
    },
}

impl fmt::Display for RoutingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTopology(message) => {
                write!(formatter, "hardware routing: invalid topology: {message}")
            }

            Self::InvalidSource {
                source,
                resource_count,
            } => {
                write!(
                    formatter,
                    "hardware routing: source resource {} is outside \
                     topology containing {} resources",
                    source, resource_count
                )
            }

            Self::InvalidTarget {
                target,
                resource_count,
            } => {
                write!(
                    formatter,
                    "hardware routing: target resource {} is outside \
                     topology containing {} resources",
                    target, resource_count
                )
            }

            Self::TrivialRouteDisabled { resource } => {
                write!(
                    formatter,
                    "hardware routing: source and target are both \
                     resource {}, but trivial routes are disabled",
                    resource
                )
            }

            Self::NoRoute {
                source,
                target,
                direction,
            } => {
                write!(
                    formatter,
                    "hardware routing: no {direction} route exists from \
                     resource {} to {}",
                    source, target
                )
            }

            Self::RouteTooLong {
                source,
                target,
                hops,
                maximum,
            } => {
                write!(
                    formatter,
                    "hardware routing: route from {} to {} requires {} \
                     hops, exceeding configured maximum {}",
                    source, target, hops, maximum
                )
            }

            Self::CostOverflow { source, target } => {
                write!(
                    formatter,
                    "hardware routing: accumulated cost overflowed while \
                     traversing {} -> {}",
                    source, target
                )
            }

            Self::EdgeRejected { source, target } => {
                write!(
                    formatter,
                    "hardware routing: edge {} -> {} was rejected by \
                     the active edge-cost/constraint model",
                    source, target
                )
            }

            Self::InvalidPolicy { message } => {
                write!(
                    formatter,
                    "hardware routing: invalid routing policy: {message}"
                )
            }

            Self::InvalidRoute { message } => {
                write!(
                    formatter,
                    "hardware routing: invalid route: {message}"
                )
            }

            Self::TooManyRoutes {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "hardware routing: requested {} routes, maximum is {}",
                    requested, maximum
                )
            }
        }
    }
}

impl std::error::Error for RoutingError {}

// =============================================================================
// Routing requirements
// =============================================================================

/// Provider-neutral routing requirements supplied by a higher-level compiler.
///
/// This type deliberately does not contain quantum IR instructions.
///
/// A circuit compiler can convert its requirements into this structure without
/// making the hardware layer depend on the compiler representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingRequirements {
    /// Physical source/target pairs that must become reachable.
    interactions: Vec<(ResourceId, ResourceId)>,

    /// Whether all requested interactions must be routable.
    pub require_all: bool,

    /// Maximum permitted route length for each interaction.
    pub maximum_hops: Option<usize>,
}

impl RoutingRequirements {
    /// Creates an empty requirement set.
    pub fn new() -> Self {
        Self {
            interactions: Vec::new(),
            require_all: true,
            maximum_hops: None,
        }
    }

    /// Adds a required physical interaction.
    pub fn add_interaction(
        &mut self,
        source: ResourceId,
        target: ResourceId,
    ) {
        self.interactions.push((source, target));
    }

    /// Returns all required interactions.
    pub fn interactions(&self) -> &[(ResourceId, ResourceId)] {
        &self.interactions
    }

    /// Returns the number of interactions.
    pub fn len(&self) -> usize {
        self.interactions.len()
    }

    /// Returns whether no interactions were requested.
    pub fn is_empty(&self) -> bool {
        self.interactions.is_empty()
    }

    /// Sorts and deduplicates requirements deterministically.
    pub fn normalize(&mut self) {
        self.interactions.sort_unstable();
        self.interactions.dedup();
    }
}

impl Default for RoutingRequirements {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Routing report
// =============================================================================

/// Deterministic report for a set of routing requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingReport {
    /// Routing schema version.
    pub schema_version: u16,

    /// Routing schema identifier.
    pub schema_id: &'static str,

    /// Successful routes, sorted by source/target.
    pub routes: Vec<RoutePlan>,

    /// Interactions that could not be routed.
    pub unroutable: Vec<(ResourceId, ResourceId)>,

    /// Total physical hops across successful routes.
    pub total_hops: u64,

    /// Total primary routing cost.
    pub total_primary_cost: u64,
}

impl RoutingReport {
    /// Returns true if every requested interaction was successfully routed.
    pub fn is_complete(&self) -> bool {
        self.unroutable.is_empty()
    }

    /// Returns the number of successful routes.
    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    /// Returns the number of failed interactions.
    pub fn unroutable_count(&self) -> usize {
        self.unroutable.len()
    }
}

// =============================================================================
// Routing engine
// =============================================================================

/// Production physical routing engine.
///
/// This engine performs deterministic physical-path search only.
///
/// It does not mutate the topology and does not mutate a logical-to-physical
/// mapping.
///
/// Higher-level `quantum::routing` code can consume its `RoutePlan` values to
/// perform actual circuit transformations.
#[derive(Debug, Clone, Copy, Default)]
pub struct HardwareRouter;

impl HardwareRouter {
    /// Creates a hardware router.
    pub const fn new() -> Self {
        Self
    }

    /// Routes a pair using the default hop-count cost model.
    pub fn route(
        &self,
        topology: &HardwareTopology,
        request: RoutingRequest,
    ) -> Result<RoutePlan, RoutingError> {
        self.route_with_models(
            topology,
            request,
            &HopCostModel,
            &AllowAllEdges,
        )
    }

    /// Routes a pair using caller-provided edge costs and constraints.
    ///
    /// This is the primary extensibility point for future calibration-aware
    /// and instruction-aware routing.
    pub fn route_with_models<C, X>(
        &self,
        topology: &HardwareTopology,
        request: RoutingRequest,
        cost_model: &C,
        constraint_model: &X,
    ) -> Result<RoutePlan, RoutingError>
    where
        C: EdgeCostModel,
        X: EdgeConstraintModel,
    {
        request.policy.validate()?;
        validate_topology(topology)?;

        if !topology.contains(request.source) {
            return Err(RoutingError::InvalidSource {
                source: request.source,
                resource_count: topology.resource_count(),
            });
        }

        if !topology.contains(request.target) {
            return Err(RoutingError::InvalidTarget {
                target: request.target,
                resource_count: topology.resource_count(),
            });
        }

        if request.source == request.target {
            if !request.policy.allow_trivial_route {
                return Err(RoutingError::TrivialRouteDisabled {
                    resource: request.source,
                });
            }

            return RoutePlan::new(
                request.source,
                request.target,
                vec![request.source],
                Vec::new(),
                RoutingCost::zero(),
                request.policy.direction,
                request.policy.objective,
            );
        }

        let route = self.dijkstra(
            topology,
            request,
            cost_model,
            constraint_model,
        )?;

        if route.edges().len() > request.policy.max_route_edges {
            return Err(RoutingError::RouteTooLong {
                source: request.source,
                target: request.target,
                hops: route.edges().len(),
                maximum: request.policy.max_route_edges,
            });
        }

        Ok(route)
    }

    /// Routes every interaction in a requirement set.
    ///
    /// Each interaction is solved independently. The engine does not mutate
    /// mappings between interactions.
    ///
    /// This is intentional: global token-swapping / circuit routing is the
    /// responsibility of `quantum::routing`, not this hardware abstraction.
    pub fn route_requirements<C, X>(
        &self,
        topology: &HardwareTopology,
        requirements: &RoutingRequirements,
        policy: RoutingPolicy,
        cost_model: &C,
        constraint_model: &X,
    ) -> Result<RoutingReport, RoutingError>
    where
        C: EdgeCostModel,
        X: EdgeConstraintModel,
    {
        policy.validate()?;
        validate_topology(topology)?;

        let mut normalized = requirements.clone();
        normalized.normalize();

        let mut routes = Vec::new();
        let mut unroutable = Vec::new();

        let mut total_hops = 0u64;
        let mut total_primary_cost = 0u64;

        for &(source, target) in normalized.interactions() {
            let request = RoutingRequest::with_policy(
                source,
                target,
                policy,
            );

            match self.route_with_models(
                topology,
                request,
                cost_model,
                constraint_model,
            ) {
                Ok(route) => {
                    total_hops = total_hops
                        .checked_add(route.cost.hops)
                        .ok_or(RoutingError::CostOverflow {
                            source,
                            target,
                        })?;

                    total_primary_cost = total_primary_cost
                        .checked_add(route.cost.primary)
                        .ok_or(RoutingError::CostOverflow {
                            source,
                            target,
                        })?;

                    routes.push(route);
                }

                Err(RoutingError::NoRoute { .. })
                | Err(RoutingError::EdgeRejected { .. })
                | Err(RoutingError::RouteTooLong { .. }) => {
                    unroutable.push((source, target));

                    if requirements.require_all {
                        return Err(RoutingError::NoRoute {
                            source,
                            target,
                            direction: policy.direction,
                        });
                    }
                }

                Err(error) => return Err(error),
            }
        }

        routes.sort_by_key(|route| (route.source, route.target));

        if let Some(maximum_hops) = requirements.maximum_hops {
            for route in &routes {
                if route.hop_count() > maximum_hops {
                    return Err(RoutingError::RouteTooLong {
                        source: route.source,
                        target: route.target,
                        hops: route.hop_count(),
                        maximum: maximum_hops,
                    });
                }
            }
        }

        Ok(RoutingReport {
            schema_version: ROUTING_SCHEMA_VERSION,
            schema_id: ROUTING_SCHEMA_ID,
            routes,
            unroutable,
            total_hops,
            total_primary_cost,
        })
    }

    /// Checks whether a route exists without materializing a route plan.
    pub fn is_reachable(
        &self,
        topology: &HardwareTopology,
        source: ResourceId,
        target: ResourceId,
        policy: RoutingPolicy,
    ) -> Result<bool, RoutingError> {
        match self.route(
            topology,
            RoutingRequest::with_policy(source, target, policy),
        ) {
            Ok(_) => Ok(true),
            Err(RoutingError::NoRoute { .. }) => Ok(false),
            Err(error) => Err(error),
        }
    }

    /// Returns the deterministic physical distance between two resources.
    pub fn distance(
        &self,
        topology: &HardwareTopology,
        source: ResourceId,
        target: ResourceId,
        direction: RoutingDirection,
    ) -> Result<usize, RoutingError> {
        let policy = RoutingPolicy {
            direction,
            objective: RoutingObjective::HopCount,
            ..RoutingPolicy::new()
        };

        Ok(self
            .route(
                topology,
                RoutingRequest::with_policy(source, target, policy),
            )?
            .hop_count())
    }

    // -------------------------------------------------------------------------
    // Dijkstra
    // -------------------------------------------------------------------------

    fn dijkstra<C, X>(
        &self,
        topology: &HardwareTopology,
        request: RoutingRequest,
        cost_model: &C,
        constraint_model: &X,
    ) -> Result<RoutePlan, RoutingError>
    where
        C: EdgeCostModel,
        X: EdgeConstraintModel,
    {
        let node_count = topology.resource_count();

        let mut best_cost: Vec<Option<RoutingCost>> =
            vec![None; node_count];

        let mut best_path: Vec<Option<Vec<ResourceId>>> =
            vec![None; node_count];

        let mut heap = BinaryHeap::new();

        let start_cost = RoutingCost::zero();
        let start_path = vec![request.source];

        best_cost[request.source] = Some(start_cost);
        best_path[request.source] = Some(start_path.clone());

        heap.push(SearchState {
            cost: start_cost,
            node: request.source,
            path: start_path,
        });

        while let Some(state) = heap.pop() {
            let current_best_cost =
                match best_cost[state.node] {
                    Some(value) => value,
                    None => continue,
                };

            let current_best_path =
                match &best_path[state.node] {
                    Some(value) => value,
                    None => continue,
                };

            if state.cost != current_best_cost
                || &state.path != current_best_path
            {
                continue;
            }

            if state.node == request.target {
                return self.build_route_from_path(
                    topology,
                    request,
                    &state.path,
                    state.cost,
                );
            }

            let mut neighbours =
                neighbours_for_direction(
                    topology,
                    state.node,
                    request.policy.direction,
                );

            neighbours.sort_unstable();
            neighbours.dedup();

            for neighbour in neighbours {
                if !constraint_model.allows_edge(
                    topology,
                    state.node,
                    neighbour,
                ) {
                    continue;
                }

                let edge_cost = match cost_model.edge_cost(
                    topology,
                    state.node,
                    neighbour,
                ) {
                    Some(cost) => cost,
                    None => continue,
                };

                let next_cost = state
                    .cost
                    .checked_add(edge_cost)
                    .ok_or(RoutingError::CostOverflow {
                        source: state.node,
                        target: neighbour,
                    })?;

                let next_hops = match usize::try_from(next_cost.hops) {
                    Ok(value) => value,
                    Err(_) => {
                        return Err(RoutingError::RouteTooLong {
                            source: request.source,
                            target: request.target,
                            hops: usize::MAX,
                            maximum: request.policy.max_route_edges,
                        });
                    }
                };

                if next_hops > request.policy.max_route_edges {
                    continue;
                }

                if state.path.contains(&neighbour) {
                    // With non-negative edge costs, cycles cannot improve a
                    // shortest path. Rejecting them also keeps route plans
                    // simple and bounded.
                    continue;
                }

                let mut next_path = state.path.clone();
                next_path.push(neighbour);

                let replace = match best_cost[neighbour] {
                    None => true,
                    Some(existing_cost) => {
                        is_better_state(
                            next_cost,
                            &next_path,
                            existing_cost,
                            best_path[neighbour]
                                .as_ref()
                                .expect("best path exists with best cost"),
                            request.policy.objective,
                        )
                    }
                };

                if replace {
                    best_cost[neighbour] = Some(next_cost);
                    best_path[neighbour] = Some(next_path.clone());

                    heap.push(SearchState {
                        cost: next_cost,
                        node: neighbour,
                        path: next_path,
                    });
                }
            }
        }

        Err(RoutingError::NoRoute {
            source: request.source,
            target: request.target,
            direction: request.policy.direction,
        })
    }

    fn build_route_from_path(
        &self,
        topology: &HardwareTopology,
        request: RoutingRequest,
        path: &[ResourceId],
        cost: RoutingCost,
    ) -> Result<RoutePlan, RoutingError> {
        let mut edges = Vec::with_capacity(path.len().saturating_sub(1));

        for window in path.windows(2) {
            let source = window[0];
            let target = window[1];

            let coupling =
                find_coupling(topology, source, target, request.policy.direction)
                    .ok_or(RoutingError::NoRoute {
                        source: request.source,
                        target: request.target,
                        direction: request.policy.direction,
                    })?;

            let edge =
                RouteEdge::from_coupling(coupling, source, target)
                    .ok_or(RoutingError::NoRoute {
                        source: request.source,
                        target: request.target,
                        direction: request.policy.direction,
                    })?;

            edges.push(edge);
        }

        RoutePlan::new(
            request.source,
            request.target,
            path.to_vec(),
            edges,
            cost,
            request.policy.direction,
            request.policy.objective,
        )
    }
}

// =============================================================================
// Search state
// =============================================================================

/// Internal Dijkstra state.
///
/// Ordering is reversed so `BinaryHeap` behaves as a min-priority queue.
///
/// Path ordering is included in the comparison to guarantee deterministic
/// tie-breaking between equal-cost routes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SearchState {
    cost: RoutingCost,
    node: ResourceId,
    path: Vec<ResourceId>,
}

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            other.cost,
            &other.path,
            other.node,
        )
            .cmp(&(
                self.cost,
                &self.path,
                self.node,
            ))
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Validates the authoritative topology before routing.
///
/// Routing deliberately does not reproduce topology invariants; topology.rs
/// remains the single owner of them.
fn validate_topology(
    topology: &HardwareTopology,
) -> Result<(), RoutingError> {
    topology
        .validate()
        .map_err(|error| RoutingError::InvalidTopology(error.to_string()))
}

/// Returns physical neighbours according to routing direction.
///
/// Directed mode uses native outgoing couplings.
///
/// Physical mode treats every coupling as an undirected physical connection.
fn neighbours_for_direction(
    topology: &HardwareTopology,
    source: ResourceId,
    direction: RoutingDirection,
) -> Vec<ResourceId> {
    let mut result = Vec::new();

    for coupling in topology.couplings() {
        match direction {
            RoutingDirection::Native => {
                if coupling.source == source {
                    result.push(coupling.target);
                }

                if matches!(coupling.connectivity, Connectivity::Bidirectional)
                    && coupling.target == source
                {
                    result.push(coupling.source);
                }
            }

            RoutingDirection::Physical => {
                if coupling.source == source {
                    result.push(coupling.target);
                } else if coupling.target == source {
                    result.push(coupling.source);
                }
            }
        }
    }

    result
}

/// Finds the canonical coupling corresponding to a traversal.
fn find_coupling(
    topology: &HardwareTopology,
    source: ResourceId,
    target: ResourceId,
    direction: RoutingDirection,
) -> Option<Coupling> {
    topology.couplings().iter().copied().find(|coupling| {
        match direction {
            RoutingDirection::Native => {
                coupling.permits_native_direction(source, target)
            }

            RoutingDirection::Physical => {
                coupling.contains(source)
                    && coupling.contains(target)
                    && source != target
            }
        }
    })
}

/// Determines whether a candidate route should replace the current route.
///
/// The objective is applied first. The complete path is then used as the
/// deterministic final tie breaker.
fn is_better_state(
    candidate_cost: RoutingCost,
    candidate_path: &[ResourceId],
    existing_cost: RoutingCost,
    existing_path: &[ResourceId],
    objective: RoutingObjective,
) -> bool {
    let candidate_key = objective_key(candidate_cost, objective);
    let existing_key = objective_key(existing_cost, objective);

    if candidate_key != existing_key {
        return candidate_key < existing_key;
    }

    candidate_path < existing_path
}

/// Produces a lexicographic objective key.
///
/// `hops` remains part of every key so equal weighted costs prefer shorter
/// physical routes.
fn objective_key(
    cost: RoutingCost,
    objective: RoutingObjective,
) -> (u64, u64, u64, u64) {
    match objective {
        RoutingObjective::HopCount => (
            cost.hops,
            cost.primary,
            cost.secondary,
            cost.tertiary,
        ),

        RoutingObjective::WeightedCost => (
            cost.primary,
            cost.secondary,
            cost.tertiary,
            cost.hops,
        ),

        RoutingObjective::WeightedThenHops => (
            cost.primary,
            cost.hops,
            cost.secondary,
            cost.tertiary,
        ),
    }
}

// =============================================================================
// Built-in deterministic weighted models
// =============================================================================

/// Fixed structural weighting model.
///
/// Every edge receives:
///
/// - primary = `primary_per_hop`;
/// - secondary = `secondary_per_hop`;
/// - tertiary = `tertiary_per_hop`.
///
/// This is useful when an application wants integer fixed-point weighting
/// without defining a custom type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedEdgeCostModel {
    /// Primary cost added per edge.
    pub primary_per_hop: u64,

    /// Secondary cost added per edge.
    pub secondary_per_hop: u64,

    /// Tertiary cost added per edge.
    pub tertiary_per_hop: u64,
}

impl FixedEdgeCostModel {
    /// Creates a fixed cost model.
    pub const fn new(
        primary_per_hop: u64,
        secondary_per_hop: u64,
        tertiary_per_hop: u64,
    ) -> Self {
        Self {
            primary_per_hop,
            secondary_per_hop,
            tertiary_per_hop,
        }
    }
}

impl EdgeCostModel for FixedEdgeCostModel {
    fn edge_cost(
        &self,
        _topology: &HardwareTopology,
        _source: ResourceId,
        _target: ResourceId,
    ) -> Option<RoutingCost> {
        Some(RoutingCost {
            primary: self.primary_per_hop,
            secondary: self.secondary_per_hop,
            tertiary: self.tertiary_per_hop,
            hops: 1,
        })
    }
}

// =============================================================================
// Route utilities
// =============================================================================

/// Returns the set of physical resources occupied by one or more routes.
///
/// The result is deterministic.
pub fn route_resources(routes: &[RoutePlan]) -> BTreeSet<ResourceId> {
    let mut resources = BTreeSet::new();

    for route in routes {
        resources.extend(route.resources.iter().copied());
    }

    resources
}

/// Returns the set of physical edges used by one or more routes.
///
/// Direction is preserved in the tuple.
pub fn route_edges(
    routes: &[RoutePlan],
) -> BTreeSet<(ResourceId, ResourceId)> {
    let mut edges = BTreeSet::new();

    for route in routes {
        edges.extend(route.edge_pairs());
    }

    edges
}

/// Counts how many route edges are shared by multiple routes.
///
/// This is a structural utility for future scheduling/crosstalk analysis.
/// It does not claim that shared physical edges can or cannot execute in
/// parallel.
pub fn shared_edge_counts(
    routes: &[RoutePlan],
) -> BTreeMap<(ResourceId, ResourceId), usize> {
    let mut counts = BTreeMap::new();

    for route in routes {
        for edge in route.edge_pairs() {
            let entry = counts.entry(edge).or_insert(0);
            *entry = entry.saturating_add(1);
        }
    }

    counts
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::hardware::topology::{
        Coupling,
        HardwareTopology,
    };

    fn line(size: usize) -> HardwareTopology {
        HardwareTopology::linear(size)
            .expect("test topology must be valid")
    }

    #[test]
    fn default_router_finds_shortest_native_path() {
        let topology = line(5);
        let router = HardwareRouter::new();

        let route = router
            .route(&topology, RoutingRequest::new(0, 4))
            .expect("route must exist");

        assert_eq!(route.resources(), &[0, 1, 2, 3, 4]);
        assert_eq!(route.hop_count(), 4);
        assert_eq!(route.cost.hops, 4);
    }

    #[test]
    fn trivial_route_is_supported() {
        let topology = line(3);
        let router = HardwareRouter::new();

        let route = router
            .route(&topology, RoutingRequest::new(1, 1))
            .expect("trivial route must exist");

        assert_eq!(route.resources(), &[1]);
        assert_eq!(route.hop_count(), 0);
        assert!(route.is_trivial());
    }

    #[test]
    fn trivial_route_can_be_disabled() {
        let topology = line(3);
        let router = HardwareRouter::new();

        let policy = RoutingPolicy {
            allow_trivial_route: false,
            ..RoutingPolicy::new()
        };

        let error = router
            .route(
                &topology,
                RoutingRequest::with_policy(1, 1, policy),
            )
            .expect_err("trivial route must be rejected");

        assert!(matches!(
            error,
            RoutingError::TrivialRouteDisabled { resource: 1 }
        ));
    }

    #[test]
    fn invalid_source_is_rejected() {
        let topology = line(3);
        let router = HardwareRouter::new();

        let error = router
            .route(&topology, RoutingRequest::new(3, 1))
            .expect_err("invalid source must fail");

        assert!(matches!(
            error,
            RoutingError::InvalidSource {
                source: 3,
                resource_count: 3
            }
        ));
    }

    #[test]
    fn invalid_target_is_rejected() {
        let topology = line(3);
        let router = HardwareRouter::new();

        let error = router
            .route(&topology, RoutingRequest::new(1, 3))
            .expect_err("invalid target must fail");

        assert!(matches!(
            error,
            RoutingError::InvalidTarget {
                target: 3,
                resource_count: 3
            }
        ));
    }

    #[test]
    fn disconnected_resources_are_rejected() {
        let topology = HardwareTopology::from_couplings(
            4,
            [Coupling::bidirectional(0, 1)],
        )
        .expect("topology must be valid");

        let router = HardwareRouter::new();

        let error = router
            .route(&topology, RoutingRequest::new(0, 3))
            .expect_err("no path must fail");

        assert!(matches!(error, RoutingError::NoRoute { .. }));
    }

    #[test]
    fn directed_routing_respects_native_direction() {
        let topology = HardwareTopology::from_couplings(
            3,
            [
                Coupling::directed(0, 1),
                Coupling::directed(1, 2),
            ],
        )
        .expect("topology must be valid");

        let router = HardwareRouter::new();

        let forward = router
            .route(&topology, RoutingRequest::new(0, 2))
            .expect("forward route must exist");

        assert_eq!(forward.resources(), &[0, 1, 2]);

        let reverse = router.route(
            &topology,
            RoutingRequest::new(2, 0),
        );

        assert!(matches!(reverse, Err(RoutingError::NoRoute { .. })));
    }

    #[test]
    fn physical_routing_can_ignore_native_direction() {
        let topology = HardwareTopology::from_couplings(
            3,
            [
                Coupling::directed(0, 1),
                Coupling::directed(1, 2),
            ],
        )
        .expect("topology must be valid");

        let router = HardwareRouter::new();

        let policy = RoutingPolicy::physical_shortest_path();

        let route = router
            .route(
                &topology,
                RoutingRequest::with_policy(2, 0, policy),
            )
            .expect("physical path must exist");

        assert_eq!(route.resources(), &[2, 1, 0]);
        assert_eq!(route.hop_count(), 2);

        // Important: physical traversal does not claim native execution.
        assert!(!route.edges()[0].native_direction);
    }

    #[test]
    fn maximum_route_length_is_enforced() {
        let topology = line(6);
        let router = HardwareRouter::new();

        let policy = RoutingPolicy {
            max_route_edges: 2,
            ..RoutingPolicy::new()
        };

        let error = router
            .route(
                &topology,
                RoutingRequest::with_policy(0, 5, policy),
            )
            .expect_err("route should exceed maximum");

        assert!(matches!(
            error,
            RoutingError::RouteTooLong {
                source: 0,
                target: 5,
                ..
            }
        ));
    }

    #[test]
    fn deterministic_tie_breaking_is_stable() {
        let topology = HardwareTopology::from_couplings(
            4,
            [
                Coupling::bidirectional(0, 1),
                Coupling::bidirectional(1, 3),
                Coupling::bidirectional(0, 2),
                Coupling::bidirectional(2, 3),
            ],
        )
        .expect("topology must be valid");

        let router = HardwareRouter::new();

        let first = router
            .route(&topology, RoutingRequest::new(0, 3))
            .expect("route must exist");

        let second = router
            .route(&topology, RoutingRequest::new(0, 3))
            .expect("route must exist");

        assert_eq!(first, second);
        assert_eq!(first.resources(), &[0, 1, 3]);
    }

    #[test]
    fn weighted_model_can_change_route_selection() {
        let topology = HardwareTopology::from_couplings(
            4,
            [
                Coupling::bidirectional(0, 1),
                Coupling::bidirectional(1, 3),
                Coupling::bidirectional(0, 2),
                Coupling::bidirectional(2, 3),
            ],
        )
        .expect("topology must be valid");

        struct Model;

        impl EdgeCostModel for Model {
            fn edge_cost(
                &self,
                _topology: &HardwareTopology,
                source: ResourceId,
                target: ResourceId,
            ) -> Option<RoutingCost> {
                if (source == 0 && target == 1)
                    || (source == 1 && target == 3)
                {
                    Some(RoutingCost::weighted(100))
                } else {
                    Some(RoutingCost::weighted(1))
                }
            }
        }

        let router = HardwareRouter::new();

        let policy = RoutingPolicy::native_weighted();

        let route = router
            .route_with_models(
                &topology,
                RoutingRequest::with_policy(0, 3, policy),
                &Model,
                &AllowAllEdges,
            )
            .expect("route must exist");

        assert_eq!(route.resources(), &[0, 2, 3]);
    }

    #[test]
    fn edge_constraint_can_reject_edges() {
        struct RejectOne;

        impl EdgeConstraintModel for RejectOne {
            fn allows_edge(
                &self,
                _topology: &HardwareTopology,
                source: ResourceId,
                target: ResourceId,
            ) -> bool {
                !(source == 1 && target == 2)
            }
        }

        let topology = line(4);
        let router = HardwareRouter::new();

        let result = router.route_with_models(
            &topology,
            RoutingRequest::new(0, 3),
            &HopCostModel,
            &RejectOne,
        );

        assert!(matches!(result, Err(RoutingError::NoRoute { .. })));
    }

    #[test]
    fn routing_requirements_are_normalized() {
        let mut requirements = RoutingRequirements::new();

        requirements.add_interaction(2, 3);
        requirements.add_interaction(0, 1);
        requirements.add_interaction(2, 3);

        requirements.normalize();

        assert_eq!(
            requirements.interactions(),
            &[(0, 1), (2, 3)]
        );
    }

    #[test]
    fn requirement_report_is_deterministic() {
        let topology = line(5);
        let router = HardwareRouter::new();

        let mut requirements = RoutingRequirements::new();
        requirements.add_interaction(3, 4);
        requirements.add_interaction(0, 2);

        let report = router
            .route_requirements(
                &topology,
                &requirements,
                RoutingPolicy::new(),
                &HopCostModel,
                &AllowAllEdges,
            )
            .expect("requirements must be routable");

        assert_eq!(report.routes.len(), 2);
        assert_eq!(
            (report.routes[0].source, report.routes[0].target),
            (0, 2)
        );
        assert_eq!(
            (report.routes[1].source, report.routes[1].target),
            (3, 4)
        );
        assert_eq!(report.total_hops, 3);
    }

    #[test]
    fn route_fingerprint_is_stable() {
        let topology = line(4);
        let router = HardwareRouter::new();

        let route = router
            .route(&topology, RoutingRequest::new(0, 3))
            .expect("route must exist");

        assert_eq!(route.fingerprint(), route.fingerprint());
        assert_eq!(route.fingerprint().len(), 16);
    }

    #[test]
    fn route_resource_utility_is_deterministic() {
        let topology = line(4);
        let router = HardwareRouter::new();

        let a = router
            .route(&topology, RoutingRequest::new(0, 2))
            .expect("route must exist");

        let b = router
            .route(&topology, RoutingRequest::new(1, 3))
            .expect("route must exist");

        let resources = route_resources(&[a, b]);

        assert_eq!(
            resources.iter().copied().collect::<Vec<_>>(),
            vec![0, 1, 2, 3]
        );
    }

    #[test]
    fn shared_edge_count_is_deterministic() {
        let topology = line(4);
        let router = HardwareRouter::new();

        let a = router
            .route(&topology, RoutingRequest::new(0, 3))
            .expect("route must exist");

        let b = router
            .route(&topology, RoutingRequest::new(0, 2))
            .expect("route must exist");

        let counts = shared_edge_counts(&[a, b]);

        assert_eq!(counts.get(&(0, 1)), Some(&2));
        assert_eq!(counts.get(&(1, 2)), Some(&2));
        assert_eq!(counts.get(&(2, 3)), Some(&1));
    }
}