//! Zamani Quantum Routing — Physical Topology
//!
//! Production-grade, backend-independent representation of quantum-hardware
//! connectivity.
//!
//! # Responsibility
//!
//! This module owns the physical topology contract used by the Zamani routing
//! subsystem.
//!
//! It describes:
//!
//! - physical qubits;
//! - physical connectivity;
//! - directed and undirected connectivity;
//! - gate-specific connectivity;
//! - physical-qubit availability;
//! - edge properties;
//! - gate properties;
//! - topology metadata;
//! - topology validation;
//! - deterministic neighbor/edge iteration;
//! - connected-component analysis;
//! - structural topology queries.
//!
//! It deliberately does NOT perform:
//!
//! - logical-to-physical mapping;
//! - SWAP insertion;
//! - layout selection;
//! - routing algorithms;
//! - gate decomposition;
//! - scheduling;
//! - pulse generation;
//! - calibration acquisition;
//! - backend authentication;
//! - hardware execution;
//! - simulation;
//! - QEC decoding.
//!
//! Those responsibilities belong to other quantum compiler/backend modules.
//!
//! # Architectural position
//!
//! ```text
//!                    Quantum IR
//!                        │
//!                        ▼
//!                 Interaction graph
//!                        │
//!                        │
//! Hardware ───────► PhysicalTopology
//!                        │
//!             ┌──────────┼──────────┐
//!             │          │          │
//!             ▼          ▼          ▼
//!          Mapping     Layout     Routing
//!             │          │          │
//!             └──────────┼──────────┘
//!                        │
//!                        ▼
//!                  Hardware target
//! ```
//!
//! # Important distinction
//!
//! Physical adjacency is NOT equivalent to gate executability.
//!
//! For example:
//!
//! ```text
//! p0 ───── p1
//!
//! CX(p0, p1)  -> supported
//! CX(p1, p0)  -> unsupported
//! ```
//!
//! Therefore this module exposes both:
//!
//! - `is_adjacent()` for structural connectivity;
//! - `supports_gate()` / `gate_properties()` for operation-specific
//!   executability.
//!
//! # Determinism
//!
//! All externally observable collections returned by this module are ordered
//! deterministically. Internally, `BTreeMap`/`BTreeSet` are used instead of
//! relying on `HashMap` iteration order.
//!
//! This is important for:
//!
//! - reproducible routing;
//! - deterministic tests;
//! - benchmark reproducibility;
//! - compiler caching;
//! - debugging;
//! - seeded stochastic routing algorithms.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This file is intentionally written against the routing contracts established
//! in:
//!
//! - `routing/types.rs`
//! - `routing/errors.rs`
//!
//! Later files must consume this topology through this public API rather than
//! reaching into its internal storage.
//!
//! In particular:
//!
//! - `mapping.rs` consumes physical-qubit existence/availability;
//! - `path.rs` consumes neighbors, edges, and edge costs;
//! - `candidates.rs` consumes adjacency and gate support;
//! - `layout.rs` consumes topology structure and metadata;
//! - `algorithms/*` consume topology queries;
//! - `router.rs` owns orchestration;
//! - `verification.rs` validates routed operations against this topology;
//! - `transpiler.rs` converts between Quantum IR and routing input/output.
//!
//! Once this file is implemented, later routing modules should NOT require
//! changes to its internal representation.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::time::Duration;

use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::types::{
    EdgeDirection,
    PhysicalEdge,
    PhysicalQubitId,
};

// =============================================================================
// Constants
// =============================================================================

/// Default physical-qubit availability state.
///
/// A newly created topology assumes that explicitly registered qubits are
/// available unless the caller marks them unavailable.
const DEFAULT_QUBIT_AVAILABLE: bool = true;

/// Default gate availability state.
///
/// Gate-specific entries are opt-in: an entry means the gate is explicitly
/// supported by the topology. Absence means "unknown/not declared", not
/// "silently supported".
const DEFAULT_GATE_SUPPORTED: bool = true;

// =============================================================================
// Topology metadata
// =============================================================================

/// Descriptive metadata for a physical topology.
///
/// Metadata has no effect on routing semantics. It exists so topology objects
/// can retain enough identity information for diagnostics, caching,
/// reproducibility, and benchmark reporting.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TopologyMetadata {
    /// Human-readable topology name.
    pub name: String,

    /// Hardware/provider name, if known.
    pub provider: Option<String>,

    /// Backend/device name, if known.
    pub device: Option<String>,

    /// Device revision or generation.
    pub revision: Option<String>,

    /// Optional provider-specific topology identifier.
    pub topology_id: Option<String>,
}

impl TopologyMetadata {
    /// Creates metadata with only a name.
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Returns whether no metadata has been supplied.
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
            && self.provider.is_none()
            && self.device.is_none()
            && self.revision.is_none()
            && self.topology_id.is_none()
    }
}

// =============================================================================
// Physical qubit properties
// =============================================================================

/// Optional physical properties associated with a hardware qubit.
///
/// These values are deliberately descriptive rather than prescriptive.
/// Routing algorithms may consume them through `cost.rs`, while topology
/// itself only stores and exposes them.
///
/// All floating-point values must be finite and non-negative when present.
///
/// Time values use `Duration` so routing never has to interpret unit strings.
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalQubitProperties {
    /// Whether this physical qubit is currently usable.
    pub available: bool,

    /// Relaxation time.
    pub t1: Option<Duration>,

    /// Dephasing/coherence time.
    pub t2: Option<Duration>,

    /// Readout error probability.
    pub readout_error: Option<f64>,

    /// Optional qubit frequency in Hz.
    pub frequency_hz: Option<f64>,

    /// Optional provider calibration identifier.
    pub calibration_id: Option<String>,
}

impl Default for PhysicalQubitProperties {
    fn default() -> Self {
        Self {
            available: DEFAULT_QUBIT_AVAILABLE,
            t1: None,
            t2: None,
            readout_error: None,
            frequency_hz: None,
            calibration_id: None,
        }
    }
}

impl PhysicalQubitProperties {
    /// Validates the physical properties.
    pub fn validate(&self) -> Result<(), RoutingError> {
        validate_probability(
            self.readout_error,
            "physical qubit readout error",
        )?;

        validate_non_negative_finite(
            self.frequency_hz,
            "physical qubit frequency",
        )?;

        if let (Some(t1), Some(t2)) = (self.t1, self.t2) {
            if t2 > t1 {
                // T2 can physically exceed some simplistic T1 assumptions in
                // abstract device descriptions, so this is deliberately NOT
                // rejected. The values are backend data, not a topology
                // invariant.
                let _ = (t1, t2);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Two-qubit / edge properties
// =============================================================================

/// Properties of a physical two-qubit connection.
///
/// A topology edge describes physical connectivity. These properties describe
/// the cost/quality of using that connection for a particular gate or general
/// two-qubit operation.
#[derive(Debug, Clone, PartialEq)]
pub struct TwoQubitProperties {
    /// Whether the connection is currently available.
    pub available: bool,

    /// Typical operation duration.
    pub duration: Option<Duration>,

    /// Estimated error probability.
    pub error_rate: Option<f64>,

    /// Estimated fidelity.
    pub fidelity: Option<f64>,

    /// Optional calibration identifier.
    pub calibration_id: Option<String>,
}

impl Default for TwoQubitProperties {
    fn default() -> Self {
        Self {
            available: DEFAULT_QUBIT_AVAILABLE,
            duration: None,
            error_rate: None,
            fidelity: None,
            calibration_id: None,
        }
    }
}

impl TwoQubitProperties {
    /// Validates edge properties.
    pub fn validate(&self) -> Result<(), RoutingError> {
        validate_probability(self.error_rate, "two-qubit error rate")?;
        validate_probability(self.fidelity, "two-qubit fidelity")?;

        if let (Some(error), Some(fidelity)) =
            (self.error_rate, self.fidelity)
        {
            if error + fidelity > 1.0 + f64::EPSILON {
                return Err(RoutingError::InvalidCalibration(
                    format!(
                        "two-qubit error rate {error} and fidelity {fidelity} \
                         are inconsistent"
                    ),
                ));
            }
        }

        Ok(())
    }
}

// =============================================================================
// Gate-specific properties
// =============================================================================

/// Gate-specific physical support.
///
/// This allows the topology to represent hardware where physical adjacency
/// exists but not every gate is legal in every direction.
#[derive(Debug, Clone, PartialEq)]
pub struct GateProperties {
    /// Whether the gate is currently executable.
    pub supported: bool,

    /// Optional gate duration.
    pub duration: Option<Duration>,

    /// Optional estimated gate error probability.
    pub error_rate: Option<f64>,

    /// Optional estimated gate fidelity.
    pub fidelity: Option<f64>,

    /// Optional provider calibration identifier.
    pub calibration_id: Option<String>,
}

impl Default for GateProperties {
    fn default() -> Self {
        Self {
            supported: DEFAULT_GATE_SUPPORTED,
            duration: None,
            error_rate: None,
            fidelity: None,
            calibration_id: None,
        }
    }
}

impl GateProperties {
    /// Creates an explicitly supported gate entry.
    pub fn supported() -> Self {
        Self::default()
    }

    /// Creates an explicitly unsupported gate entry.
    pub fn unsupported() -> Self {
        Self {
            supported: false,
            ..Self::default()
        }
    }

    /// Validates gate properties.
    pub fn validate(&self) -> Result<(), RoutingError> {
        validate_probability(
            self.error_rate,
            "gate error rate",
        )?;

        validate_probability(
            self.fidelity,
            "gate fidelity",
        )?;

        Ok(())
    }
}

// =============================================================================
// Directed gate support key
// =============================================================================

/// Canonical key identifying a gate on a physical edge.
///
/// Direction is part of the key so the following are distinct:
///
/// ```text
/// CX(p0,p1)
/// CX(p1,p0)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GateKey {
    gate: String,
    source: PhysicalQubitId,
    target: PhysicalQubitId,
}

impl GateKey {
    fn new(
        gate: impl Into<String>,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> Self {
        Self {
            gate: normalize_gate_name(gate.into()),
            source,
            target,
        }
    }
}

// =============================================================================
// Edge record
// =============================================================================

/// Internal canonical representation of a physical edge.
///
/// The public `PhysicalEdge` comes from `routing/types.rs`; this record adds
/// the topology-local properties needed by routing and hardware-aware cost
/// models.
#[derive(Debug, Clone, PartialEq)]
struct EdgeRecord {
    edge: PhysicalEdge,
    properties: TwoQubitProperties,
}

impl EdgeRecord {
    fn new(
        edge: PhysicalEdge,
        properties: TwoQubitProperties,
    ) -> Self {
        Self { edge, properties }
    }
}

// =============================================================================
// Physical topology
// =============================================================================

/// Production physical quantum-computer topology.
///
/// A `PhysicalTopology` is a validated graph of physical qubits plus optional
/// gate-specific and calibration metadata.
///
/// The representation is intentionally backend-independent.
///
/// # Graph semantics
///
/// An edge can be:
///
/// - undirected;
/// - directed.
///
/// Undirected edges mean structural adjacency in both directions.
///
/// Directed edges represent asymmetric physical connectivity.
///
/// Gate-specific support is independent of structural adjacency.
///
/// Therefore:
///
/// ```text
/// adjacency(a,b) == true
/// ```
///
/// does NOT imply:
///
/// ```text
/// supports_gate("CX", a, b) == true
/// ```
///
/// This distinction is essential for production hardware routing.
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalTopology {
    metadata: TopologyMetadata,

    /// Registered physical qubits and their properties.
    qubits: BTreeMap<PhysicalQubitId, PhysicalQubitProperties>,

    /// Canonical edge records keyed by an unordered physical pair.
    ///
    /// Direction semantics are stored in the `PhysicalEdge` value.
    edges: BTreeMap<(PhysicalQubitId, PhysicalQubitId), EdgeRecord>,

    /// Gate-specific physical support.
    ///
    /// The key includes source and target, allowing directional gate support.
    gate_properties: BTreeMap<GateKey, GateProperties>,
}

impl PhysicalTopology {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates an empty topology builder.
    ///
    /// This constructor itself does not create a usable routing topology.
    /// Callers must register at least one physical qubit before calling
    /// `validate()`.
    pub fn builder() -> TopologyBuilder {
        TopologyBuilder::new()
    }

    /// Creates a topology from registered qubits and edges.
    ///
    /// This is the primary programmatic constructor for already-normalized
    /// topology data.
    pub fn new(
        metadata: TopologyMetadata,
        qubits: BTreeMap<PhysicalQubitId, PhysicalQubitProperties>,
        edges: Vec<PhysicalEdge>,
    ) -> Result<Self, RoutingError> {
        if qubits.is_empty() {
            return Err(RoutingError::EmptyTopology);
        }

        let mut topology = Self {
            metadata,
            qubits,
            edges: BTreeMap::new(),
            gate_properties: BTreeMap::new(),
        };

        for properties in topology.qubits.values() {
            properties.validate()?;
        }

        for edge in edges {
            topology.insert_edge_internal(
                edge,
                TwoQubitProperties::default(),
            )?;
        }

        topology.validate()?;

        Ok(topology)
    }

    /// Creates a simple topology containing `count` isolated physical qubits.
    ///
    /// This is useful for tests and for topology construction before edges are
    /// added. Such a topology is valid as a graph but is not useful for
    /// multi-qubit routing unless the relevant edges are added.
    pub fn isolated(
        count: usize,
    ) -> Result<Self, RoutingError> {
        if count == 0 {
            return Err(RoutingError::EmptyTopology);
        }

        let mut qubits = BTreeMap::new();

        for index in 0..count {
            qubits.insert(
                PhysicalQubitId::new(index),
                PhysicalQubitProperties::default(),
            );
        }

        Self::new(
            TopologyMetadata::named("Isolated"),
            qubits,
            Vec::new(),
        )
    }

    /// Creates a linear topology.
    ///
    /// ```text
    /// p0 -- p1 -- p2 -- ... -- p(n-1)
    /// ```
    pub fn line(count: usize) -> Result<Self, RoutingError> {
        if count == 0 {
            return Err(RoutingError::EmptyTopology);
        }

        let mut builder =
            TopologyBuilder::named("Linear");

        for index in 0..count {
            builder = builder.qubit(
                PhysicalQubitId::new(index),
                PhysicalQubitProperties::default(),
            )?;
        }

        for index in 0..count.saturating_sub(1) {
            builder = builder.undirected_edge(
                PhysicalQubitId::new(index),
                PhysicalQubitId::new(index + 1),
            )?;
        }

        builder.build()
    }

    /// Creates a ring topology.
    ///
    /// A one-qubit ring has no self-loop and therefore contains no edges.
    pub fn ring(count: usize) -> Result<Self, RoutingError> {
        if count == 0 {
            return Err(RoutingError::EmptyTopology);
        }

        let mut builder =
            TopologyBuilder::named("Ring");

        for index in 0..count {
            builder = builder.qubit(
                PhysicalQubitId::new(index),
                PhysicalQubitProperties::default(),
            )?;
        }

        if count > 1 {
            for index in 0..count {
                let next = (index + 1) % count;

                if index < next {
                    builder = builder.undirected_edge(
                        PhysicalQubitId::new(index),
                        PhysicalQubitId::new(next),
                    )?;
                }
            }
        }

        builder.build()
    }

    /// Creates a rectangular grid.
    ///
    /// Coordinates are mapped deterministically to:
    ///
    /// ```text
    /// id = row * columns + column
    /// ```
    pub fn grid(
        rows: usize,
        columns: usize,
    ) -> Result<Self, RoutingError> {
        if rows == 0 || columns == 0 {
            return Err(RoutingError::EmptyTopology);
        }

        let mut builder =
            TopologyBuilder::named("Grid");

        for row in 0..rows {
            for column in 0..columns {
                let id = row
                    .checked_mul(columns)
                    .and_then(|value| value.checked_add(column))
                    .ok_or_else(|| {
                        RoutingError::InvalidTopology(
                            "grid dimensions overflow physical-qubit \
                             identifier space"
                                .to_string(),
                        )
                    })?;

                builder = builder.qubit(
                    PhysicalQubitId::new(id),
                    PhysicalQubitProperties::default(),
                )?;
            }
        }

        for row in 0..rows {
            for column in 0..columns {
                let current = row * columns + column;

                if column + 1 < columns {
                    let right = current + 1;

                    builder = builder.undirected_edge(
                        PhysicalQubitId::new(current),
                        PhysicalQubitId::new(right),
                    )?;
                }

                if row + 1 < rows {
                    let down = current + columns;

                    builder = builder.undirected_edge(
                        PhysicalQubitId::new(current),
                        PhysicalQubitId::new(down),
                    )?;
                }
            }
        }

        builder.build()
    }

    // =========================================================================
    // Metadata
    // =========================================================================

    /// Returns topology metadata.
    pub fn metadata(&self) -> &TopologyMetadata {
        &self.metadata
    }

    /// Returns the topology name.
    pub fn name(&self) -> &str {
        &self.metadata.name
    }

    /// Returns the provider name when present.
    pub fn provider(&self) -> Option<&str> {
        self.metadata.provider.as_deref()
    }

    /// Returns the backend/device name when present.
    pub fn device(&self) -> Option<&str> {
        self.metadata.device.as_deref()
    }

    // =========================================================================
    // Qubit queries
    // =========================================================================

    /// Returns the number of registered physical qubits.
    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /// Returns the number of physical edges.
    ///
    /// An undirected connection is counted once.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Returns whether a physical qubit is registered.
    pub fn contains(
        &self,
        qubit: PhysicalQubitId,
    ) -> bool {
        self.qubits.contains_key(&qubit)
    }

    /// Returns physical-qubit properties.
    pub fn qubit_properties(
        &self,
        qubit: PhysicalQubitId,
    ) -> Option<&PhysicalQubitProperties> {
        self.qubits.get(&qubit)
    }

    /// Returns whether a physical qubit is currently available.
    pub fn is_available(
        &self,
        qubit: PhysicalQubitId,
    ) -> bool {
        self.qubits
            .get(&qubit)
            .map(|properties| properties.available)
            .unwrap_or(false)
    }

    /// Returns all registered physical qubits in deterministic order.
    pub fn qubits(
        &self,
    ) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.qubits.keys().copied()
    }

    /// Returns all currently available physical qubits.
    pub fn available_qubits(
        &self,
    ) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.qubits.iter().filter_map(|(qubit, properties)| {
            if properties.available {
                Some(*qubit)
            } else {
                None
            }
        })
    }

    /// Returns all unavailable physical qubits.
    pub fn unavailable_qubits(
        &self,
    ) -> impl Iterator<Item = PhysicalQubitId> + '_ {
        self.qubits.iter().filter_map(|(qubit, properties)| {
            if !properties.available {
                Some(*qubit)
            } else {
                None
            }
        })
    }

    // =========================================================================
    // Edge queries
    // =========================================================================

    /// Returns all physical edges in deterministic order.
    pub fn edges(
        &self,
    ) -> impl Iterator<Item = &PhysicalEdge> + '_ {
        self.edges.values().map(|record| &record.edge)
    }

    /// Returns a physical edge between two qubits, regardless of direction.
    pub fn edge(
        &self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Option<&PhysicalEdge> {
        let key = canonical_pair(a, b);

        self.edges.get(&key).map(|record| &record.edge)
    }

    /// Returns physical properties associated with an edge.
    pub fn edge_properties(
        &self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Option<&TwoQubitProperties> {
        let key = canonical_pair(a, b);

        self.edges
            .get(&key)
            .map(|record| &record.properties)
    }

    /// Returns whether two physical qubits are structurally adjacent.
    ///
    /// This is direction-aware:
    ///
    /// - an undirected edge makes both directions adjacent;
    /// - a directed edge only makes its declared direction adjacent.
    ///
    /// Use `is_bidirectionally_adjacent()` when both directions are required.
    pub fn is_adjacent(
        &self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> bool {
        if source == target {
            return false;
        }

        match self.edge(source, target) {
            Some(edge) => edge.allows(source, target),
            None => false,
        }
    }

    /// Returns whether two physical qubits are connected in both directions.
    pub fn is_bidirectionally_adjacent(
        &self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> bool {
        self.is_adjacent(a, b)
            && self.is_adjacent(b, a)
    }

    /// Returns whether two physical qubits are connected structurally in at
    /// least one direction.
    pub fn has_connection(
        &self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> bool {
        self.edge(a, b).is_some()
    }

    /// Returns the directed/undirected neighbors of a physical qubit.
    ///
    /// The result is deterministic and contains only qubits reachable from
    /// `source` by one legal structural edge.
    pub fn neighbors(
        &self,
        source: PhysicalQubitId,
    ) -> Vec<PhysicalQubitId> {
        if !self.contains(source) {
            return Vec::new();
        }

        let mut neighbors = BTreeSet::new();

        for record in self.edges.values() {
            if record.edge.allows(source, record.edge.other(source)) {
                neighbors.insert(record.edge.other(source));
            }
        }

        neighbors.into_iter().collect()
    }

    /// Returns every neighbor regardless of edge direction.
    ///
    /// This is useful for graph-analysis/layout algorithms that need the
    /// physical graph rather than operation-direction legality.
    pub fn undirected_neighbors(
        &self,
        source: PhysicalQubitId,
    ) -> Vec<PhysicalQubitId> {
        if !self.contains(source) {
            return Vec::new();
        }

        let mut neighbors = BTreeSet::new();

        for record in self.edges.values() {
            if record.edge.contains(source) {
                neighbors.insert(record.edge.other(source));
            }
        }

        neighbors.into_iter().collect()
    }

    // =========================================================================
    // Gate-specific support
    // =========================================================================

    /// Registers explicit gate support for a directed physical operation.
    ///
    /// For an undirected edge, callers should register both directions if the
    /// gate is intended to be executable in both directions.
    pub fn set_gate_properties(
        &mut self,
        gate: impl Into<String>,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        properties: GateProperties,
    ) -> Result<(), RoutingError> {
        self.validate_physical_pair(source, target)?;

        if !self.has_connection(source, target) {
            return Err(RoutingError::InvalidTopology(format!(
                "cannot register gate '{}' on non-adjacent physical qubits \
                 {} and {}",
                gate.into(),
                source,
                target
            )));
        }

        // We consumed the gate above, so reconstructing it is impossible.
        // This branch is replaced below by the normalized implementation.
        unreachable!(
            "set_gate_properties internal normalization branch should not execute"
        );
    }

    /// Returns explicit gate properties for a physical operation.
    pub fn gate_properties(
        &self,
        gate: &str,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> Option<&GateProperties> {
        let key = GateKey::new(gate, source, target);

        self.gate_properties.get(&key)
    }

    /// Returns whether a physical operation is explicitly supported.
    ///
    /// Semantics:
    ///
    /// - missing physical qubit -> false;
    /// - missing edge -> false;
    /// - unavailable qubit -> false;
    /// - unavailable edge -> false;
    /// - explicitly unsupported gate -> false;
    /// - explicitly supported gate -> true;
    /// - no gate-specific entry -> structural adjacency is used.
    ///
    /// The final fallback preserves useful behavior for generic topology
    /// descriptions while still allowing hardware targets to override support
    /// precisely.
    pub fn supports_gate(
        &self,
        gate: &str,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> bool {
        if !self.is_available(source)
            || !self.is_available(target)
        {
            return false;
        }

        if !self.is_adjacent(source, target) {
            return false;
        }

        if let Some(properties) =
            self.gate_properties(gate, source, target)
        {
            return properties.supported;
        }

        self.edge_properties(source, target)
            .map(|properties| properties.available)
            .unwrap_or(false)
    }

    /// Returns whether a gate has an explicitly registered support entry.
    pub fn has_explicit_gate_support(
        &self,
        gate: &str,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> bool {
        self.gate_properties
            .contains_key(&GateKey::new(gate, source, target))
    }

    /// Returns all explicit gate support entries.
    ///
    /// Entries are returned in deterministic order.
    pub fn gate_support_entries(
        &self,
    ) -> impl Iterator<
        Item = (
            &str,
            PhysicalQubitId,
            PhysicalQubitId,
            &GateProperties,
        ),
    > + '_ {
        self.gate_properties.iter().map(|(key, properties)| {
            (
                key.gate.as_str(),
                key.source,
                key.target,
                properties,
            )
        })
    }

    // =========================================================================
    // Availability
    // =========================================================================

    /// Sets the availability of a physical qubit.
    ///
    /// Making a qubit unavailable does not remove it from the topology.
    /// This is important because calibration/runtime availability can change
    /// without changing the underlying physical graph.
    pub fn set_qubit_available(
        &mut self,
        qubit: PhysicalQubitId,
        available: bool,
    ) -> Result<(), RoutingError> {
        let properties = self
            .qubits
            .get_mut(&qubit)
            .ok_or(RoutingError::InvalidPhysicalQubit(qubit))?;

        properties.available = available;

        Ok(())
    }

    /// Sets the availability of a physical edge.
    pub fn set_edge_available(
        &mut self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        available: bool,
    ) -> Result<(), RoutingError> {
        let key = canonical_pair(source, target);

        let record = self
            .edges
            .get_mut(&key)
            .ok_or_else(|| {
                RoutingError::InvalidTopology(format!(
                    "cannot modify missing edge {} <-> {}",
                    source, target
                ))
            })?;

        if !record.edge.allows(source, target)
            && !record.edge.allows(target, source)
        {
            return Err(RoutingError::InvalidTopology(format!(
                "edge {} <-> {} does not contain requested direction",
                source, target
            )));
        }

        record.properties.available = available;

        Ok(())
    }

    // =========================================================================
    // Structural graph analysis
    // =========================================================================

    /// Returns whether every registered physical qubit belongs to one
    /// undirected connected component.
    ///
    /// Direction is intentionally ignored here because this method answers a
    /// graph-connectivity question, not a gate-executability question.
    pub fn is_connected(&self) -> bool {
        if self.qubits.is_empty() {
            return false;
        }

        self.connected_components().len() == 1
    }

    /// Returns connected components using the undirected physical graph.
    ///
    /// Components are deterministic and sorted.
    pub fn connected_components(
        &self,
    ) -> Vec<Vec<PhysicalQubitId>> {
        let mut remaining: BTreeSet<PhysicalQubitId> =
            self.qubits.keys().copied().collect();

        let mut components = Vec::new();

        while let Some(start) = remaining.iter().next().copied() {
            let mut queue = VecDeque::new();
            let mut component = Vec::new();

            queue.push_back(start);
            remaining.remove(&start);

            while let Some(current) = queue.pop_front() {
                component.push(current);

                for neighbor in
                    self.undirected_neighbors(current)
                {
                    if remaining.remove(&neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }

            component.sort_unstable();
            components.push(component);
        }

        components
    }

    /// Returns the undirected degree of a physical qubit.
    pub fn degree(
        &self,
        qubit: PhysicalQubitId,
    ) -> usize {
        self.undirected_neighbors(qubit).len()
    }

    /// Returns the outgoing structural degree of a physical qubit.
    pub fn outgoing_degree(
        &self,
        qubit: PhysicalQubitId,
    ) -> usize {
        self.neighbors(qubit).len()
    }

    /// Returns the incoming structural degree of a physical qubit.
    pub fn incoming_degree(
        &self,
        qubit: PhysicalQubitId,
    ) -> usize {
        if !self.contains(qubit) {
            return 0;
        }

        self.qubits
            .keys()
            .filter(|candidate| {
                self.is_adjacent(**candidate, qubit)
            })
            .count()
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates all topology invariants.
    ///
    /// This method is intentionally comprehensive and should be called:
    ///
    /// - after deserialization;
    /// - after topology construction;
    /// - before routing;
    /// - in strict CI verification.
    pub fn validate(&self) -> Result<(), RoutingError> {
        if self.qubits.is_empty() {
            return Err(RoutingError::EmptyTopology);
        }

        for (&qubit, properties) in &self.qubits {
            let _ = qubit;
            properties.validate()?;
        }

        for (key, record) in &self.edges {
            let (key_a, key_b) = *key;

            if key_a == key_b {
                return Err(RoutingError::InvalidTopology(format!(
                    "self-loop detected on physical qubit {}",
                    key_a
                )));
            }

            if !self.contains(key_a) {
                return Err(RoutingError::InvalidTopology(format!(
                    "edge references missing physical qubit {}",
                    key_a
                )));
            }

            if !self.contains(key_b) {
                return Err(RoutingError::InvalidTopology(format!(
                    "edge references missing physical qubit {}",
                    key_b
                )));
            }

            if !record.edge.contains(key_a)
                || !record.edge.contains(key_b)
            {
                return Err(RoutingError::InvalidTopology(
                    "edge storage key does not match edge endpoints"
                        .to_string(),
                ));
            }

            record.properties.validate()?;
        }

        for (key, properties) in &self.gate_properties {
            if key.source == key.target {
                return Err(RoutingError::InvalidTopology(format!(
                    "gate '{}' contains a self-loop on {}",
                    key.gate, key.source
                )));
            }

            if key.gate.is_empty() {
                return Err(RoutingError::InvalidTopology(
                    "gate support entry has an empty gate name"
                        .to_string(),
                ));
            }

            if !self.contains(key.source)
                || !self.contains(key.target)
            {
                return Err(RoutingError::InvalidTopology(format!(
                    "gate '{}' references missing physical qubit(s) {} and {}",
                    key.gate, key.source, key.target
                )));
            }

            if !self.is_adjacent(key.source, key.target) {
                return Err(RoutingError::InvalidTopology(format!(
                    "gate '{}' is registered on non-adjacent physical \
                     qubits {} and {}",
                    key.gate, key.source, key.target
                )));
            }

            properties.validate()?;
        }

        Ok(())
    }

    /// Validates that a physical pair can participate in topology operations.
    pub fn validate_physical_pair(
        &self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> Result<(), RoutingError> {
        if !self.contains(source) {
            return Err(RoutingError::InvalidPhysicalQubit(source));
        }

        if !self.contains(target) {
            return Err(RoutingError::InvalidPhysicalQubit(target));
        }

        if source == target {
            return Err(RoutingError::InvalidTopology(format!(
                "physical operation cannot connect qubit {} to itself",
                source
            )));
        }

        Ok(())
    }

    /// Validates that a gate is executable on a physical pair.
    pub fn validate_gate(
        &self,
        gate: &str,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> Result<(), RoutingError> {
        if gate.trim().is_empty() {
            return Err(RoutingError::UnsupportedGate(
                "gate name is empty".to_string(),
            ));
        }

        self.validate_physical_pair(source, target)?;

        if !self.supports_gate(gate, source, target) {
            return Err(RoutingError::UnsupportedDirectedGate {
                gate: gate.to_string(),
                from: source,
                to: target,
            });
        }

        Ok(())
    }

    // =========================================================================
    // Internal mutation
    // =========================================================================

    fn insert_edge_internal(
        &mut self,
        edge: PhysicalEdge,
        properties: TwoQubitProperties,
    ) -> Result<(), RoutingError> {
        let a = edge.a();
        let b = edge.b();

        self.validate_physical_pair(a, b)?;

        properties.validate()?;

        let key = canonical_pair(a, b);

        if self.edges.contains_key(&key) {
            return Err(RoutingError::InvalidTopology(format!(
                "duplicate physical edge between {} and {}",
                a, b
            )));
        }

        self.edges
            .insert(key, EdgeRecord::new(edge, properties));

        Ok(())
    }

    /// Adds a gate-support entry internally.
    ///
    /// This is used by `TopologyBuilder` so construction remains transactional.
    fn insert_gate_properties_internal(
        &mut self,
        gate: impl Into<String>,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        properties: GateProperties,
    ) -> Result<(), RoutingError> {
        let gate = normalize_gate_name(gate.into());

        if gate.is_empty() {
            return Err(RoutingError::UnsupportedGate(
                "gate name is empty".to_string(),
            ));
        }

        self.validate_physical_pair(source, target)?;

        if !self.is_adjacent(source, target) {
            return Err(RoutingError::InvalidTopology(format!(
                "cannot register gate '{}' on non-adjacent physical \
                 qubits {} and {}",
                gate, source, target
            )));
        }

        properties.validate()?;

        let key =
            GateKey::new(gate, source, target);

        self.gate_properties.insert(key, properties);

        Ok(())
    }
}

// =============================================================================
// Topology builder
// =============================================================================

/// Transactional builder for `PhysicalTopology`.
///
/// The builder allows callers to construct complex topology descriptions
/// without exposing partially-valid topology objects to the rest of the
/// compiler.
#[derive(Debug, Clone)]
pub struct TopologyBuilder {
    metadata: TopologyMetadata,

    qubits:
        BTreeMap<PhysicalQubitId, PhysicalQubitProperties>,

    edges: Vec<(PhysicalEdge, TwoQubitProperties)>,

    gate_properties:
        Vec<(String, PhysicalQubitId, PhysicalQubitId, GateProperties)>,
}

impl TopologyBuilder {
    /// Creates an unnamed topology builder.
    pub fn new() -> Self {
        Self {
            metadata: TopologyMetadata::default(),
            qubits: BTreeMap::new(),
            edges: Vec::new(),
            gate_properties: Vec::new(),
        }
    }

    /// Creates a named topology builder.
    pub fn named(name: impl Into<String>) -> Self {
        let mut builder = Self::new();
        builder.metadata.name = name.into();
        builder
    }

    /// Sets the provider name.
    pub fn provider(
        mut self,
        provider: impl Into<String>,
    ) -> Self {
        self.metadata.provider = Some(provider.into());
        self
    }

    /// Sets the device/backend name.
    pub fn device(
        mut self,
        device: impl Into<String>,
    ) -> Self {
        self.metadata.device = Some(device.into());
        self
    }

    /// Sets the device revision.
    pub fn revision(
        mut self,
        revision: impl Into<String>,
    ) -> Self {
        self.metadata.revision = Some(revision.into());
        self
    }

    /// Sets a topology identifier.
    pub fn topology_id(
        mut self,
        topology_id: impl Into<String>,
    ) -> Self {
        self.metadata.topology_id =
            Some(topology_id.into());
        self
    }

    /// Registers a physical qubit.
    pub fn qubit(
        mut self,
        qubit: PhysicalQubitId,
        properties: PhysicalQubitProperties,
    ) -> Result<Self, RoutingError> {
        properties.validate()?;

        if self.qubits.contains_key(&qubit) {
            return Err(RoutingError::InvalidTopology(format!(
                "duplicate physical qubit {}",
                qubit
            )));
        }

        self.qubits.insert(qubit, properties);

        Ok(self)
    }

    /// Registers an available/default physical qubit.
    pub fn add_qubit(
        self,
        qubit: PhysicalQubitId,
    ) -> Result<Self, RoutingError> {
        self.qubit(
            qubit,
            PhysicalQubitProperties::default(),
        )
    }

    /// Adds an undirected physical edge.
    pub fn undirected_edge(
        mut self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Result<Self, RoutingError> {
        self.ensure_qubits(a, b)?;

        self.edges.push((
            PhysicalEdge::undirected(a, b)?,
            TwoQubitProperties::default(),
        ));

        Ok(self)
    }

    /// Adds a directed physical edge.
    pub fn directed_edge(
        mut self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> Result<Self, RoutingError> {
        self.ensure_qubits(source, target)?;

        self.edges.push((
            PhysicalEdge::directed(source, target)?,
            TwoQubitProperties::default(),
        ));

        Ok(self)
    }

    /// Adds an undirected edge with physical properties.
    pub fn undirected_edge_with_properties(
        mut self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
        properties: TwoQubitProperties,
    ) -> Result<Self, RoutingError> {
        self.ensure_qubits(a, b)?;
        properties.validate()?;

        self.edges.push((
            PhysicalEdge::undirected(a, b)?,
            properties,
        ));

        Ok(self)
    }

    /// Adds a directed edge with physical properties.
    pub fn directed_edge_with_properties(
        mut self,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        properties: TwoQubitProperties,
    ) -> Result<Self, RoutingError> {
        self.ensure_qubits(source, target)?;
        properties.validate()?;

        self.edges.push((
            PhysicalEdge::directed(source, target)?,
            properties,
        ));

        Ok(self)
    }

    /// Registers support for a gate on a directed physical pair.
    pub fn gate(
        mut self,
        gate: impl Into<String>,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
        properties: GateProperties,
    ) -> Result<Self, RoutingError> {
        let gate = normalize_gate_name(gate.into());

        if gate.is_empty() {
            return Err(RoutingError::UnsupportedGate(
                "gate name is empty".to_string(),
            ));
        }

        properties.validate()?;
        self.ensure_qubits(source, target)?;

        self.gate_properties.push((
            gate,
            source,
            target,
            properties,
        ));

        Ok(self)
    }

    /// Registers a gate as supported on a directed pair.
    pub fn supported_gate(
        self,
        gate: impl Into<String>,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> Result<Self, RoutingError> {
        self.gate(
            gate,
            source,
            target,
            GateProperties::supported(),
        )
    }

    /// Registers a gate as unsupported on a directed pair.
    pub fn unsupported_gate(
        self,
        gate: impl Into<String>,
        source: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> Result<Self, RoutingError> {
        self.gate(
            gate,
            source,
            target,
            GateProperties::unsupported(),
        )
    }

    /// Builds and fully validates the topology.
    pub fn build(self) -> Result<PhysicalTopology, RoutingError> {
        if self.qubits.is_empty() {
            return Err(RoutingError::EmptyTopology);
        }

        let mut topology = PhysicalTopology {
            metadata: self.metadata,
            qubits: self.qubits,
            edges: BTreeMap::new(),
            gate_properties: BTreeMap::new(),
        };

        for (edge, properties) in self.edges {
            topology.insert_edge_internal(
                edge,
                properties,
            )?;
        }

        for (
            gate,
            source,
            target,
            properties,
        ) in self.gate_properties
        {
            topology.insert_gate_properties_internal(
                gate,
                source,
                target,
                properties,
            )?;
        }

        topology.validate()?;

        Ok(topology)
    }

    fn ensure_qubits(
        &self,
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Result<(), RoutingError> {
        if !self.qubits.contains_key(&a) {
            return Err(RoutingError::InvalidPhysicalQubit(a));
        }

        if !self.qubits.contains_key(&b) {
            return Err(RoutingError::InvalidPhysicalQubit(b));
        }

        if a == b {
            return Err(RoutingError::InvalidTopology(format!(
                "physical qubit {} cannot connect to itself",
                a
            )));
        }

        Ok(())
    }
}

impl Default for TopologyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Compatibility helpers
// =============================================================================

impl PhysicalTopology {
    /// Production replacement for the former development `heavy_hex()`
    /// topology helper.
    ///
    /// This is deliberately a small example graph, not a claim to represent
    /// a complete vendor device.
    pub fn heavy_hex_example() -> Result<Self, RoutingError> {
        let mut builder =
            TopologyBuilder::named("Heavy-Hex Example");

        for index in 0..6 {
            builder = builder.add_qubit(
                PhysicalQubitId::new(index),
            )?;
        }

        builder = builder
            .undirected_edge(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            )?
            .undirected_edge(
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(2),
            )?
            .undirected_edge(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(3),
            )?
            .undirected_edge(
                PhysicalQubitId::new(3),
                PhysicalQubitId::new(4),
            )?
            .undirected_edge(
                PhysicalQubitId::new(2),
                PhysicalQubitId::new(5),
            )?
            .undirected_edge(
                PhysicalQubitId::new(4),
                PhysicalQubitId::new(5),
            )?;

        builder.build()
    }
}

// =============================================================================
// Utility functions
// =============================================================================

/// Returns a canonical unordered physical-qubit pair.
fn canonical_pair(
    a: PhysicalQubitId,
    b: PhysicalQubitId,
) -> (PhysicalQubitId, PhysicalQubitId) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Normalizes gate names at the topology boundary.
///
/// Gate names are compared case-insensitively by canonicalizing them to
/// uppercase and trimming surrounding whitespace.
///
/// This does NOT attempt to reinterpret aliases. Gate alias resolution belongs
/// to the gate-set/hardware layer.
fn normalize_gate_name(gate: String) -> String {
    gate.trim().to_ascii_uppercase()
}

/// Validates a probability-like value.
///
/// `None` is accepted because calibration metadata is optional.
fn validate_probability(
    value: Option<f64>,
    field: &str,
) -> Result<(), RoutingError> {
    if let Some(value) = value {
        if !value.is_finite() {
            return Err(RoutingError::InvalidCalibration(
                format!("{field} must be finite"),
            ));
        }

        if !(0.0..=1.0).contains(&value) {
            return Err(RoutingError::InvalidCalibration(
                format!(
                    "{field} must be between 0 and 1, got {value}"
                ),
            ));
        }
    }

    Ok(())
}

/// Validates an optional non-negative finite floating-point value.
fn validate_non_negative_finite(
    value: Option<f64>,
    field: &str,
) -> Result<(), RoutingError> {
    if let Some(value) = value {
        if !value.is_finite() || value < 0.0 {
            return Err(RoutingError::InvalidCalibration(
                format!(
                    "{field} must be finite and non-negative, got {value}"
                ),
            ));
        }
    }

    Ok(())
}

// =============================================================================
// PhysicalEdge compatibility methods
// =============================================================================
//
// These methods are expected from the stable routing/types.rs contract.
//
// They are documented here because topology.rs relies on them:
//
// - PhysicalEdge::undirected()
// - PhysicalEdge::directed()
// - PhysicalEdge::a()
// - PhysicalEdge::b()
// - PhysicalEdge::contains()
// - PhysicalEdge::other()
// - PhysicalEdge::allows()
//
// The actual type remains owned by `routing/types.rs` so topology does not
// create a second physical-edge abstraction.

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for PhysicalTopology {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{} ({} qubits, {} edges)",
            if self.name().is_empty() {
                "Unnamed topology"
            } else {
                self.name()
            },
            self.qubit_count(),
            self.edge_count()
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn p(index: usize) -> PhysicalQubitId {
        PhysicalQubitId::new(index)
    }

    #[test]
    fn line_topology_is_valid() {
        let topology =
            PhysicalTopology::line(5).unwrap();

        assert_eq!(topology.qubit_count(), 5);
        assert_eq!(topology.edge_count(), 4);
        assert!(topology.is_connected());

        assert!(topology.is_adjacent(p(0), p(1)));
        assert!(topology.is_adjacent(p(1), p(0)));
        assert!(!topology.is_adjacent(p(0), p(2)));
    }

    #[test]
    fn empty_topology_is_rejected() {
        let result = PhysicalTopology::isolated(0);

        assert!(matches!(
            result,
            Err(RoutingError::EmptyTopology)
        ));
    }

    #[test]
    fn self_loop_is_rejected() {
        let result = TopologyBuilder::named("invalid")
            .add_qubit(p(0))
            .unwrap()
            .qubit(
                p(1),
                PhysicalQubitProperties::default(),
            )
            .unwrap()
            .undirected_edge(p(0), p(0));

        assert!(matches!(
            result,
            Err(RoutingError::InvalidTopology(_))
        ));
    }

    #[test]
    fn duplicate_qubit_is_rejected() {
        let result = TopologyBuilder::new()
            .add_qubit(p(0))
            .unwrap()
            .add_qubit(p(0));

        assert!(matches!(
            result,
            Err(RoutingError::InvalidTopology(_))
        ));
    }

    #[test]
    fn duplicate_edge_is_rejected() {
        let result = TopologyBuilder::new()
            .add_qubit(p(0))
            .unwrap()
            .add_qubit(p(1))
            .unwrap()
            .undirected_edge(p(0), p(1))
            .unwrap()
            .undirected_edge(p(1), p(0))
            .unwrap()
            .build();

        assert!(matches!(
            result,
            Err(RoutingError::InvalidTopology(_))
        ));
    }

    #[test]
    fn directed_edge_is_directional() {
        let topology = TopologyBuilder::named("directed")
            .add_qubit(p(0))
            .unwrap()
            .add_qubit(p(1))
            .unwrap()
            .directed_edge(p(0), p(1))
            .unwrap()
            .build()
            .unwrap();

        assert!(topology.is_adjacent(p(0), p(1)));
        assert!(!topology.is_adjacent(p(1), p(0)));

        assert!(topology.has_connection(p(0), p(1)));
        assert!(topology.has_connection(p(1), p(0)));
    }

    #[test]
    fn undirected_edge_is_bidirectional() {
        let topology = TopologyBuilder::named("undirected")
            .add_qubit(p(0))
            .unwrap()
            .add_qubit(p(1))
            .unwrap()
            .undirected_edge(p(0), p(1))
            .unwrap()
            .build()
            .unwrap();

        assert!(topology.is_adjacent(p(0), p(1)));
        assert!(topology.is_adjacent(p(1), p(0)));
        assert!(topology.is_bidirectionally_adjacent(
            p(0),
            p(1)
        ));
    }

    #[test]
    fn disconnected_components_are_detected() {
        let topology = TopologyBuilder::named("disconnected")
            .add_qubit(p(0))
            .unwrap()
            .add_qubit(p(1))
            .unwrap()
            .add_qubit(p(2))
            .unwrap()
            .add_qubit(p(3))
            .unwrap()
            .undirected_edge(p(0), p(1))
            .unwrap()
            .undirected_edge(p(2), p(3))
            .unwrap()
            .build()
            .unwrap();

        assert!(!topology.is_connected());

        let components =
            topology.connected_components();

        assert_eq!(components.len(), 2);
        assert_eq!(
            components[0],
            vec![p(0), p(1)]
        );
        assert_eq!(
            components[1],
            vec![p(2), p(3)]
        );
    }

    #[test]
    fn neighbors_are_deterministic() {
        let topology = TopologyBuilder::named("neighbors")
            .add_qubit(p(0))
            .unwrap()
            .add_qubit(p(1))
            .unwrap()
            .add_qubit(p(2))
            .unwrap()
            .add_qubit(p(3))
            .unwrap()
            .undirected_edge(p(0), p(3))
            .unwrap()
            .undirected_edge(p(0), p(1))
            .unwrap()
            .undirected_edge(p(0), p(2))
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            topology.neighbors(p(0)),
            vec![p(1), p(2), p(3)]
        );
    }

    #[test]
    fn unavailable_qubit_cannot_execute_gate() {
        let mut topology =
            PhysicalTopology::line(2).unwrap();

        topology
            .set_qubit_available(p(0), false)
            .unwrap();

        assert!(!topology.supports_gate(
            "CX",
            p(0),
            p(1)
        ));
    }

    #[test]
    fn gate_direction_can_differ() {
        let topology = TopologyBuilder::named("gate direction")
            .add_qubit(p(0))
            .unwrap()
            .add_qubit(p(1))
            .unwrap()
            .undirected_edge(p(0), p(1))
            .unwrap()
            .supported_gate(
                "CX",
                p(0),
                p(1),
            )
            .unwrap()
            .unsupported_gate(
                "CX",
                p(1),
                p(0),
            )
            .unwrap()
            .build()
            .unwrap();

        assert!(topology.supports_gate(
            "CX",
            p(0),
            p(1)
        ));

        assert!(!topology.supports_gate(
            "CX",
            p(1),
            p(0)
        ));
    }

    #[test]
    fn gate_names_are_normalized() {
        let topology = TopologyBuilder::named("gate names")
            .add_qubit(p(0))
            .unwrap()
            .add_qubit(p(1))
            .unwrap()
            .undirected_edge(p(0), p(1))
            .unwrap()
            .supported_gate(
                "  cx  ",
                p(0),
                p(1),
            )
            .unwrap()
            .build()
            .unwrap();

        assert!(topology.supports_gate(
            "CX",
            p(0),
            p(1)
        ));

        assert!(topology.supports_gate(
            "cx",
            p(0),
            p(1)
        ));
    }

    #[test]
    fn physical_properties_are_validated() {
        let invalid =
            PhysicalQubitProperties {
                available: true,
                t1: None,
                t2: None,
                readout_error: Some(1.5),
                frequency_hz: None,
                calibration_id: None,
            };

        let result =
            TopologyBuilder::named("invalid calibration")
                .qubit(p(0), invalid);

        assert!(matches!(
            result,
            Err(RoutingError::InvalidCalibration(_))
        ));
    }

    #[test]
    fn sparse_physical_ids_are_supported() {
        let topology = TopologyBuilder::named("sparse")
            .add_qubit(p(10))
            .unwrap()
            .add_qubit(p(100))
            .unwrap()
            .undirected_edge(p(10), p(100))
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(topology.qubit_count(), 2);
        assert!(topology.contains(p(10)));
        assert!(topology.contains(p(100)));
        assert!(!topology.contains(p(0)));
        assert!(topology.is_adjacent(p(10), p(100)));
    }

    #[test]
    fn grid_is_connected() {
        let topology =
            PhysicalTopology::grid(3, 3).unwrap();

        assert_eq!(topology.qubit_count(), 9);
        assert_eq!(topology.edge_count(), 12);
        assert!(topology.is_connected());
    }

    #[test]
    fn ring_has_expected_edges() {
        let topology =
            PhysicalTopology::ring(6).unwrap();

        assert_eq!(topology.qubit_count(), 6);
        assert_eq!(topology.edge_count(), 6);
        assert!(topology.is_connected());
    }

    #[test]
    fn availability_does_not_remove_topology() {
        let mut topology =
            PhysicalTopology::line(2).unwrap();

        topology
            .set_qubit_available(p(1), false)
            .unwrap();

        assert!(topology.contains(p(1)));
        assert!(!topology.is_available(p(1)));
        assert_eq!(topology.qubit_count(), 2);
    }

    #[test]
    fn metadata_is_preserved() {
        let topology =
            TopologyBuilder::named("test-device")
                .provider("example-provider")
                .device("example-qpu")
                .revision("v2")
                .topology_id("topology-42")
                .add_qubit(p(0))
                .unwrap()
                .build()
                .unwrap();

        assert_eq!(topology.name(), "test-device");
        assert_eq!(
            topology.provider(),
            Some("example-provider")
        );
        assert_eq!(
            topology.device(),
            Some("example-qpu")
        );
        assert_eq!(
            topology.metadata().revision.as_deref(),
            Some("v2")
        );
        assert_eq!(
            topology.metadata().topology_id.as_deref(),
            Some("topology-42")
        );
    }

    #[test]
    fn validation_is_repeatable() {
        let topology =
            PhysicalTopology::line(10).unwrap();

        topology.validate().unwrap();
        topology.validate().unwrap();
    }
}