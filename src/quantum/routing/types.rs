//! Zamani Quantum Routing — Stable Routing Types
//!
//! This module defines the foundational, hardware-independent vocabulary used
//! by the quantum routing subsystem.
//!
//! # Architectural responsibility
//!
//! `types.rs` owns:
//!
//! - logical routing identifiers;
//! - physical routing identifiers;
//! - logical/physical qubit references;
//! - qubit interaction descriptions;
//! - physical connectivity edges;
//! - edge direction;
//! - routing movement primitives;
//! - routing operation records;
//! - routing input/output-independent algorithm vocabulary;
//! - routing algorithm identifiers;
//! - routing objectives;
//! - verification levels;
//! - qubit roles;
//! - stable gate identity used by routing contracts.
//!
//! It does NOT own:
//!
//! - topology storage or validation;
//! - logical-to-physical mapping state;
//! - routing algorithms;
//! - path finding;
//! - layout algorithms;
//! - cost calculation;
//! - hardware calibration;
//! - scheduling;
//! - pulse generation;
//! - backend execution;
//! - OpenQASM parsing;
//! - quantum simulation;
//! - QEC decoding.
//!
//! Those responsibilities belong to the corresponding routing/backend
//! subsystems.
//!
//! # Dependency rule
//!
//! This file intentionally depends only on the Rust standard library.
//!
//! In particular, it does not import:
//!
//! - `topology.rs`;
//! - `mapping.rs`;
//! - `cost.rs`;
//! - `config.rs`;
//! - `router.rs`;
//! - hardware providers;
//! - compiler IR implementation details.
//!
//! This makes this file suitable as the first frozen routing contract.
//!
//! # Integration contract
//!
//! Later routing modules should use these types rather than introducing their
//! own `usize`, `String`, tuple, or ad-hoc enum representations for the same
//! concepts.
//!
//! Intended dependency direction:
//!
//! ```text
//!                 types.rs
//!              /     |      \
//!             /      |       \
//!       topology   mapping    cost
//!          |          |         |
//!          +----------+---------+
//!                     |
//!               algorithms
//!                     |
//!                  router
//!                     |
//!                transpiler
//! ```
//!
//! # Quantum IR boundary
//!
//! The canonical Quantum IR has its own logical `QubitId` and
//! `PhysicalQubitId`. Routing deliberately uses its own stable identifiers so
//! that routing remains a compiler/backend subsystem rather than becoming
//! coupled to a particular IR implementation.
//!
//! The integration adapter is responsible for converting between canonical
//! Quantum IR identifiers and routing identifiers.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No nightly features are required.
//! No external dependencies are required.

// =============================================================================
// Logical qubit identifier
// =============================================================================

/// Stable logical-qubit identifier used by routing.
///
/// A logical qubit represents a qubit in the program before physical hardware
/// placement is applied.
///
/// This type is deliberately distinct from [`PhysicalQubitId`].
///
/// # Invariants
///
/// A `LogicalQubitId`:
///
/// - is non-negative because it is represented by `usize`;
/// - is stable within a routing invocation;
/// - does not imply physical placement;
/// - does not imply that the corresponding qubit exists in a particular
///   circuit until the routing input is validated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LogicalQubitId(usize);

impl LogicalQubitId {
    /// Creates a logical-qubit identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the zero-based logical-qubit index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl From<usize> for LogicalQubitId {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

impl From<LogicalQubitId> for usize {
    fn from(id: LogicalQubitId) -> Self {
        id.index()
    }
}

impl std::fmt::Display for LogicalQubitId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "q{}", self.0)
    }
}

// =============================================================================
// Physical qubit identifier
// =============================================================================

/// Stable physical-hardware qubit identifier used by routing.
///
/// A physical qubit identifies a location/resource in the target hardware
/// topology.
///
/// This type is deliberately distinct from [`LogicalQubitId`].
///
/// A physical identifier does not by itself establish:
///
/// - that the qubit exists on a target device;
/// - that the qubit is currently available;
/// - that a particular gate is supported;
/// - that the qubit is calibrated;
/// - that the qubit is connected to another qubit.
///
/// Those properties belong to the hardware/topology layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhysicalQubitId(usize);

impl PhysicalQubitId {
    /// Creates a physical-qubit identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the physical hardware index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl From<usize> for PhysicalQubitId {
    fn from(index: usize) -> Self {
        Self::new(index)
    }
}

impl From<PhysicalQubitId> for usize {
    fn from(id: PhysicalQubitId) -> Self {
        id.index()
    }
}

impl std::fmt::Display for PhysicalQubitId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "p{}", self.0)
    }
}

// =============================================================================
// Logical / physical qubit reference
// =============================================================================

/// Identifies whether a routing reference is logical or physical.
///
/// This is useful at API boundaries where a caller may need to explicitly
/// state which namespace an operand belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QubitRef {
    /// A logical program qubit.
    Logical(LogicalQubitId),

    /// A physical hardware qubit.
    Physical(PhysicalQubitId),
}

impl QubitRef {
    /// Returns the logical identifier when this is a logical reference.
    #[must_use]
    pub const fn logical(self) -> Option<LogicalQubitId> {
        match self {
            Self::Logical(id) => Some(id),
            Self::Physical(_) => None,
        }
    }

    /// Returns the physical identifier when this is a physical reference.
    #[must_use]
    pub const fn physical(self) -> Option<PhysicalQubitId> {
        match self {
            Self::Physical(id) => Some(id),
            Self::Logical(_) => None,
        }
    }

    /// Returns `true` when this is a logical reference.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Logical(_))
    }

    /// Returns `true` when this is a physical reference.
    #[must_use]
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Physical(_))
    }
}

impl From<LogicalQubitId> for QubitRef {
    fn from(id: LogicalQubitId) -> Self {
        Self::Logical(id)
    }
}

impl From<PhysicalQubitId> for QubitRef {
    fn from(id: PhysicalQubitId) -> Self {
        Self::Physical(id)
    }
}

// =============================================================================
// Qubit role
// =============================================================================

/// Semantic role of a physical or logical qubit during compilation.
///
/// Routing must not hard-code QEC semantics into its core algorithms, but the
/// mapping contract must be capable of distinguishing important resource
/// classes.
///
/// This allows future QEC-aware routing without redesigning the foundational
/// routing types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QubitRole {
    /// Ordinary computational/data qubit.
    Data,

    /// Ancillary qubit used by an algorithm or protocol.
    Ancilla,

    /// Syndrome-extraction qubit used by an error-correction protocol.
    Syndrome,

    /// Qubit reserved for magic-state preparation/use.
    MagicState,

    /// Qubit reserved for a caller-controlled purpose.
    Reserved,

    /// Qubit cannot currently participate in routing.
    Unavailable,
}

impl Default for QubitRole {
    fn default() -> Self {
        Self::Data
    }
}

impl QubitRole {
    /// Returns whether the role represents a usable routing resource.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        !matches!(self, Self::Unavailable)
    }

    /// Returns whether the qubit is explicitly reserved.
    #[must_use]
    pub const fn is_reserved(self) -> bool {
        matches!(self, Self::Reserved)
    }

    /// Returns whether the qubit is unavailable.
    #[must_use]
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }
}

// =============================================================================
// Qubit interaction
// =============================================================================

/// A logical interaction that routing must make physically executable.
///
/// This is deliberately independent of the canonical Quantum IR `Gate`.
/// The routing layer only needs the semantic information required to determine
/// connectivity and routing cost.
///
/// # Example
///
/// A logical CX:
///
/// ```text
/// q0 ──●
///      │
/// q1 ──X
/// ```
///
/// becomes an interaction containing:
///
/// ```text
/// operands = [q0, q1]
/// gate = GateIdentity::Named("cx")
/// ```
///
/// Routing does not decide how `cx` is decomposed. That remains a later
/// synthesis/hardware-lowering responsibility.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QubitInteraction {
    /// Logical operands in their semantic gate order.
    operands: Vec<LogicalQubitId>,

    /// Stable gate identity.
    gate: GateIdentity,
}

impl QubitInteraction {
    /// Creates a logical interaction.
    ///
    /// This constructor does not validate gate arity because gate semantics
    /// belong to the Quantum IR/gate layer. Routing only records the
    /// interaction.
    #[must_use]
    pub fn new(operands: Vec<LogicalQubitId>, gate: GateIdentity) -> Self {
        Self { operands, gate }
    }

    /// Creates an interaction from a slice of logical operands.
    #[must_use]
    pub fn from_slice(operands: &[LogicalQubitId], gate: GateIdentity) -> Self {
        Self {
            operands: operands.to_vec(),
            gate,
        }
    }

    /// Returns the logical operands.
    #[must_use]
    pub fn operands(&self) -> &[LogicalQubitId] {
        &self.operands
    }

    /// Returns the gate identity.
    #[must_use]
    pub const fn gate(&self) -> &GateIdentity {
        &self.gate
    }

    /// Returns the interaction arity.
    #[must_use]
    pub fn arity(&self) -> usize {
        self.operands.len()
    }

    /// Returns whether the interaction is single-qubit.
    #[must_use]
    pub fn is_single_qubit(&self) -> bool {
        self.operands.len() == 1
    }

    /// Returns whether the interaction is two-qubit.
    #[must_use]
    pub fn is_two_qubit(&self) -> bool {
        self.operands.len() == 2
    }

    /// Returns whether the interaction is multi-qubit.
    #[must_use]
    pub fn is_multi_qubit(&self) -> bool {
        self.operands.len() > 2
    }
}

// =============================================================================
// Gate identity
// =============================================================================

/// Stable routing-level identity for a quantum operation.
///
/// Routing must not depend on a particular gate implementation enum because
/// hardware providers and future Zamani IR revisions may introduce additional
/// gates.
///
/// Built-in semantic names cover the common gates while `Custom` permits
/// vendor-, research-, or future-specific operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GateIdentity {
    /// Identity operation.
    Identity,

    /// Pauli-X.
    X,

    /// Pauli-Y.
    Y,

    /// Pauli-Z.
    Z,

    /// Hadamard.
    H,

    /// S phase gate.
    S,

    /// S dagger.
    Sdg,

    /// T gate.
    T,

    /// T dagger.
    Tdg,

    /// RX rotation.
    Rx,

    /// RY rotation.
    Ry,

    /// RZ rotation.
    Rz,

    /// Generic phase operation.
    Phase,

    /// Controlled-X.
    Cx,

    /// Controlled-Y.
    Cy,

    /// Controlled-Z.
    Cz,

    /// Controlled-H.
    Ch,

    /// SWAP operation.
    Swap,

    /// iSWAP operation.
    ISwap,

    /// Echoed cross-resonance operation.
    Ecr,

    /// Controlled-RX.
    Crx,

    /// Controlled-RY.
    Cry,

    /// Controlled-RZ.
    Crz,

    /// Toffoli / controlled-controlled-X.
    Ccx,

    /// Fredkin / controlled-SWAP.
    CSwap,

    /// Measurement.
    Measure,

    /// Barrier.
    Barrier,

    /// Reset.
    Reset,

    /// A custom gate identified by a stable name.
    Custom(String),
}

impl GateIdentity {
    /// Returns the canonical lowercase name for built-in gates.
    ///
    /// Custom gate names are returned unchanged.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Identity => "id",
            Self::X => "x",
            Self::Y => "y",
            Self::Z => "z",
            Self::H => "h",
            Self::S => "s",
            Self::Sdg => "sdg",
            Self::T => "t",
            Self::Tdg => "tdg",
            Self::Rx => "rx",
            Self::Ry => "ry",
            Self::Rz => "rz",
            Self::Phase => "phase",
            Self::Cx => "cx",
            Self::Cy => "cy",
            Self::Cz => "cz",
            Self::Ch => "ch",
            Self::Swap => "swap",
            Self::ISwap => "iswap",
            Self::Ecr => "ecr",
            Self::Crx => "crx",
            Self::Cry => "cry",
            Self::Crz => "crz",
            Self::Ccx => "ccx",
            Self::CSwap => "cswap",
            Self::Measure => "measure",
            Self::Barrier => "barrier",
            Self::Reset => "reset",
            Self::Custom(name) => name.as_str(),
        }
    }

    /// Returns the canonical gate name as an owned string.
    #[must_use]
    pub fn name_owned(&self) -> String {
        self.name().to_owned()
    }

    /// Returns whether this is a built-in gate.
    #[must_use]
    pub const fn is_builtin(&self) -> bool {
        !matches!(self, Self::Custom(_))
    }

    /// Returns whether the gate is non-unitary.
    #[must_use]
    pub const fn is_non_unitary(&self) -> bool {
        matches!(self, Self::Measure | Self::Barrier | Self::Reset)
    }

    /// Returns whether the gate is a measurement.
    #[must_use]
    pub const fn is_measurement(&self) -> bool {
        matches!(self, Self::Measure)
    }

    /// Returns whether the gate is a reset.
    #[must_use]
    pub const fn is_reset(&self) -> bool {
        matches!(self, Self::Reset)
    }

    /// Returns whether the gate is a barrier.
    #[must_use]
    pub const fn is_barrier(&self) -> bool {
        matches!(self, Self::Barrier)
    }

    /// Returns whether the gate is naturally symmetric with respect to its
    /// two-qubit operands.
    ///
    /// This describes operand exchange at the routing-connectivity level. It
    /// does not assert that every physical implementation of the gate is
    /// symmetric.
    #[must_use]
    pub const fn is_symmetric_two_qubit_gate(&self) -> bool {
        matches!(
            self,
            Self::Cz
                | Self::Swap
                | Self::ISwap
        )
    }

    /// Returns whether operand order is semantically directional.
    #[must_use]
    pub const fn is_directional(&self) -> bool {
        matches!(
            self,
            Self::Cx
                | Self::Cy
                | Self::Ch
                | Self::Crx
                | Self::Cry
                | Self::Crz
                | Self::Ecr
        )
    }
}

impl From<&str> for GateIdentity {
    fn from(name: &str) -> Self {
        match name {
            "id" | "i" => Self::Identity,
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            "h" => Self::H,
            "s" => Self::S,
            "sdg" | "sdag" => Self::Sdg,
            "t" => Self::T,
            "tdg" | "tdag" => Self::Tdg,
            "rx" => Self::Rx,
            "ry" => Self::Ry,
            "rz" => Self::Rz,
            "phase" | "p" => Self::Phase,
            "cx" | "cnot" => Self::Cx,
            "cy" => Self::Cy,
            "cz" => Self::Cz,
            "ch" => Self::Ch,
            "swap" => Self::Swap,
            "iswap" => Self::ISwap,
            "ecr" => Self::Ecr,
            "crx" => Self::Crx,
            "cry" => Self::Cry,
            "crz" => Self::Crz,
            "ccx" | "toffoli" => Self::Ccx,
            "cswap" | "fredkin" => Self::CSwap,
            "measure" | "measurement" | "m" => Self::Measure,
            "barrier" => Self::Barrier,
            "reset" => Self::Reset,
            custom => Self::Custom(custom.to_owned()),
        }
    }
}

impl From<String> for GateIdentity {
    fn from(name: String) -> Self {
        Self::from(name.as_str())
    }
}

// =============================================================================
// Physical edge direction
// =============================================================================

/// Directionality of a physical connectivity edge.
///
/// Topology and hardware layers use this to determine whether a gate can be
/// executed in a particular operand order.
///
/// Importantly, an undirected connectivity edge does not mean that every gate
/// is executable in both directions. Gate-specific directionality is resolved
/// by the topology/hardware capability layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EdgeDirection {
    /// Connectivity is usable in both directions.
    Undirected,

    /// Connectivity is directed from `source` to `target`.
    Forward,

    /// Connectivity is directed from `target` to `source`.
    Reverse,
}

impl Default for EdgeDirection {
    fn default() -> Self {
        Self::Undirected
    }
}

impl EdgeDirection {
    /// Returns whether this direction permits either endpoint order.
    #[must_use]
    pub const fn is_undirected(self) -> bool {
        matches!(self, Self::Undirected)
    }

    /// Returns whether this direction is explicitly directional.
    #[must_use]
    pub const fn is_directed(self) -> bool {
        !self.is_undirected()
    }
}

// =============================================================================
// Physical edge
// =============================================================================

/// Physical connectivity edge.
///
/// This is a value object. It does not itself validate that the endpoint
/// qubits exist or that the edge is supported by a hardware target.
///
/// Such validation belongs to `topology.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhysicalEdge {
    /// First endpoint.
    a: PhysicalQubitId,

    /// Second endpoint.
    b: PhysicalQubitId,

    /// Connectivity direction.
    direction: EdgeDirection,
}

impl PhysicalEdge {
    /// Creates a physical edge.
    #[must_use]
    pub const fn new(
        a: PhysicalQubitId,
        b: PhysicalQubitId,
        direction: EdgeDirection,
    ) -> Self {
        Self { a, b, direction }
    }

    /// Creates an undirected physical edge.
    #[must_use]
    pub const fn undirected(
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Self {
        Self::new(a, b, EdgeDirection::Undirected)
    }

    /// Creates a directed edge from `a` to `b`.
    #[must_use]
    pub const fn directed(
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Self {
        Self::new(a, b, EdgeDirection::Forward)
    }

    /// Returns the first endpoint.
    #[must_use]
    pub const fn a(self) -> PhysicalQubitId {
        self.a
    }

    /// Returns the second endpoint.
    #[must_use]
    pub const fn b(self) -> PhysicalQubitId {
        self.b
    }

    /// Returns the edge direction.
    #[must_use]
    pub const fn direction(self) -> EdgeDirection {
        self.direction
    }

    /// Returns whether the endpoints are identical.
    ///
    /// A self-loop is structurally representable as a value but must be
    /// rejected by topology validation.
    #[must_use]
    pub const fn is_self_loop(self) -> bool {
        self.a == self.b
    }

    /// Returns the opposite endpoint when `qubit` belongs to this edge.
    #[must_use]
    pub const fn other(self, qubit: PhysicalQubitId) -> Option<PhysicalQubitId> {
        if qubit == self.a {
            Some(self.b)
        } else if qubit == self.b {
            Some(self.a)
        } else {
            None
        }
    }
}

// =============================================================================
// Routing movement
// =============================================================================

/// A physical movement requested by the routing algorithm.
///
/// A movement is a semantic routing operation. It is intentionally not a
/// hardware gate.
///
/// For example, a `Swap` says that the logical states occupying two adjacent
/// physical locations must exchange positions. The hardware-lowering layer
/// decides whether this becomes:
///
/// - a native SWAP;
/// - three CNOTs;
/// - another decomposition;
/// - a calibrated primitive.
///
/// This separation prevents routing from becoming a hardware synthesis layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoutingMove {
    /// Exchange the logical states occupying two physical qubits.
    Swap {
        /// First physical location.
        a: PhysicalQubitId,

        /// Second physical location.
        b: PhysicalQubitId,
    },

    /// A bridge/remote interaction supported by a later lowering stage.
    ///
    /// The routing layer records the semantic movement/connection requirement;
    /// hardware lowering determines the concrete gate sequence.
    Bridge {
        /// First endpoint.
        a: PhysicalQubitId,

        /// Intermediate physical qubit.
        bridge: PhysicalQubitId,

        /// Second endpoint.
        b: PhysicalQubitId,

        /// Gate represented by the bridge operation.
        gate: GateIdentity,
    },

    /// An arbitrary physical permutation.
    ///
    /// This is useful for layout/routing engines that discover a permutation
    /// directly rather than as a sequence of SWAPs.
    Permutation {
        /// Resulting physical ordering.
        mapping: Vec<(LogicalQubitId, PhysicalQubitId)>,
    },
}

impl RoutingMove {
    /// Returns all physical qubits directly touched by the move.
    #[must_use]
    pub fn physical_qubits(&self) -> Vec<PhysicalQubitId> {
        match self {
            Self::Swap { a, b } => vec![*a, *b],

            Self::Bridge {
                a,
                bridge,
                b,
                ..
            } => vec![*a, *bridge, *b],

            Self::Permutation { mapping } => mapping
                .iter()
                .map(|(_, physical)| *physical)
                .collect(),
        }
    }

    /// Returns whether this movement is a SWAP.
    #[must_use]
    pub const fn is_swap(&self) -> bool {
        matches!(self, Self::Swap { .. })
    }

    /// Returns whether this movement is a bridge operation.
    #[must_use]
    pub const fn is_bridge(&self) -> bool {
        matches!(self, Self::Bridge { .. })
    }

    /// Returns whether this movement is a permutation.
    #[must_use]
    pub const fn is_permutation(&self) -> bool {
        matches!(self, Self::Permutation { .. })
    }
}

// =============================================================================
// Routing operation
// =============================================================================

/// Operation emitted by the routing layer.
///
/// Unlike [`RoutingMove`], this enum also permits the actual logical operation
/// to be emitted after its operands have become physically executable.
///
/// The operation stream is therefore suitable for later verification,
/// lowering, metrics collection, and compiler integration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingOperation {
    /// A semantic movement operation.
    Move(RoutingMove),

    /// A logical gate that is executable at the specified physical locations.
    Gate {
        /// Logical gate identity.
        gate: GateIdentity,

        /// Physical operands in the gate's semantic order.
        operands: Vec<PhysicalQubitId>,

        /// Original logical operands in the same semantic order.
        logical_operands: Vec<LogicalQubitId>,
    },

    /// A non-gate routing boundary marker.
    Barrier {
        /// Physical qubits affected by the barrier.
        operands: Vec<PhysicalQubitId>,
    },
}

impl RoutingOperation {
    /// Returns the physical qubits touched by this operation.
    #[must_use]
    pub fn physical_qubits(&self) -> Vec<PhysicalQubitId> {
        match self {
            Self::Move(movement) => movement.physical_qubits(),

            Self::Gate { operands, .. } => operands.clone(),

            Self::Barrier { operands } => operands.clone(),
        }
    }

    /// Returns the logical operands associated with this operation.
    ///
    /// Movement operations have no fixed logical operand sequence because their
    /// logical meaning is derived from the mapping at the time they execute.
    #[must_use]
    pub fn logical_operands(&self) -> &[LogicalQubitId] {
        match self {
            Self::Move(_) => &[],

            Self::Gate {
                logical_operands,
                ..
            } => logical_operands,

            Self::Barrier { .. } => &[],
        }
    }

    /// Returns the gate identity when this is a gate operation.
    #[must_use]
    pub fn gate(&self) -> Option<&GateIdentity> {
        match self {
            Self::Move(_) => None,

            Self::Gate { gate, .. } => Some(gate),

            Self::Barrier { .. } => None,
        }
    }

    /// Returns whether this is a movement operation.
    #[must_use]
    pub const fn is_move(&self) -> bool {
        matches!(self, Self::Move(_))
    }

    /// Returns whether this is a gate operation.
    #[must_use]
    pub const fn is_gate(&self) -> bool {
        matches!(self, Self::Gate { .. })
    }

    /// Returns whether this is a barrier operation.
    #[must_use]
    pub const fn is_barrier(&self) -> bool {
        matches!(self, Self::Barrier { .. })
    }
}

// =============================================================================
// Routing algorithm identity
// =============================================================================

/// Identifies the routing algorithm requested for a routing invocation.
///
/// The implementations themselves belong to `algorithms/`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoutingAlgorithm {
    /// Automatically select an appropriate algorithm.
    Auto,

    /// Do not perform movement routing.
    ///
    /// This is useful when the input is already physically executable or when
    /// a caller wants verification without automatic movement.
    None,

    /// Deterministic greedy routing.
    Basic,

    /// Deterministic shortest-path routing.
    ShortestPath,

    /// Lookahead routing.
    Lookahead,

    /// SABRE-style heuristic routing.
    Sabre,

    /// Hardware/noise-aware routing.
    NoiseAware,

    /// Dynamic/online routing.
    Dynamic,

    /// Caller-registered custom routing algorithm.
    Custom(String),
}

impl Default for RoutingAlgorithm {
    fn default() -> Self {
        Self::Auto
    }
}

impl RoutingAlgorithm {
    /// Returns a stable human-readable algorithm name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Auto => "auto",
            Self::None => "none",
            Self::Basic => "basic",
            Self::ShortestPath => "shortest_path",
            Self::Lookahead => "lookahead",
            Self::Sabre => "sabre",
            Self::NoiseAware => "noise_aware",
            Self::Dynamic => "dynamic",
            Self::Custom(name) => name.as_str(),
        }
    }

    /// Returns whether this algorithm is a built-in Zamani algorithm.
    #[must_use]
    pub const fn is_builtin(&self) -> bool {
        !matches!(self, Self::Custom(_))
    }

    /// Returns whether the algorithm is heuristic rather than purely
    /// deterministic shortest-path/greedy routing.
    #[must_use]
    pub const fn is_heuristic(&self) -> bool {
        matches!(
            self,
            Self::Lookahead
                | Self::Sabre
                | Self::NoiseAware
                | Self::Dynamic
        )
    }
}

impl From<&str> for RoutingAlgorithm {
    fn from(value: &str) -> Self {
        match value {
            "auto" => Self::Auto,
            "none" => Self::None,
            "basic" => Self::Basic,
            "shortest_path" | "shortest-path" => Self::ShortestPath,
            "lookahead" => Self::Lookahead,
            "sabre" => Self::Sabre,
            "noise_aware" | "noise-aware" => Self::NoiseAware,
            "dynamic" => Self::Dynamic,
            custom => Self::Custom(custom.to_owned()),
        }
    }
}

// =============================================================================
// Routing objective
// =============================================================================

/// Primary optimization objective for routing.
///
/// Routing must not be permanently tied to minimum SWAP count. Different
/// hardware and workloads require different objectives.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RoutingObjective {
    /// Minimize the number of inserted SWAP operations.
    SwapCount,

    /// Minimize resulting circuit depth.
    Depth,

    /// Minimize physical execution duration.
    Duration,

    /// Minimize estimated physical error.
    Error,

    /// Maximize estimated fidelity.
    Fidelity,

    /// Optimize a caller-defined weighted combination.
    Weighted,

    /// Delegate comparison to a registered custom objective.
    Custom(String),
}

impl Default for RoutingObjective {
    fn default() -> Self {
        Self::SwapCount
    }
}

impl RoutingObjective {
    /// Returns a stable objective name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::SwapCount => "swap_count",
            Self::Depth => "depth",
            Self::Duration => "duration",
            Self::Error => "error",
            Self::Fidelity => "fidelity",
            Self::Weighted => "weighted",
            Self::Custom(name) => name.as_str(),
        }
    }

    /// Returns whether the objective is hardware-quality-aware.
    #[must_use]
    pub const fn is_hardware_aware(&self) -> bool {
        matches!(
            self,
            Self::Duration
                | Self::Error
                | Self::Fidelity
                | Self::Weighted
        )
    }
}

// =============================================================================
// Routing mode
// =============================================================================

/// Behavioral strictness of a routing invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoutingMode {
    /// Reject any condition that prevents the requested routing contract from
    /// being satisfied exactly.
    Strict,

    /// Prefer a valid route but permit configured fallback behavior.
    BestEffort,

    /// Permit approximation where an explicitly configured algorithm supports
    /// it.
    Approximate,
}

impl Default for RoutingMode {
    fn default() -> Self {
        Self::Strict
    }
}

// =============================================================================
// Verification level
// =============================================================================

/// Level of post-routing correctness checking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationLevel {
    /// Do not perform post-routing verification.
    ///
    /// This should normally only be used by trusted internal performance
    /// paths. CI and production defaults should use a stronger level.
    None,

    /// Validate basic structural invariants.
    Basic,

    /// Validate routing and physical-executability invariants.
    Standard,

    /// Perform the strongest available validation, including preservation
    /// checks and strict invariants.
    Strict,
}

impl Default for VerificationLevel {
    fn default() -> Self {
        Self::Standard
    }
}

impl VerificationLevel {
    /// Returns whether any verification is requested.
    #[must_use]
    pub const fn enabled(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Returns whether this level requires strict verification.
    #[must_use]
    pub const fn is_strict(self) -> bool {
        matches!(self, Self::Strict)
    }
}

// =============================================================================
// Layout source
// =============================================================================

/// Describes how the initial logical-to-physical layout was selected.
///
/// The actual layout implementation belongs to `layout.rs`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LayoutSource {
    /// No explicit layout source was provided.
    None,

    /// Direct/trivial logical-index-to-physical-index placement.
    Trivial,

    /// Sequential placement.
    Sequential,

    /// Connectivity-driven placement.
    Connectivity,

    /// Interaction-graph-driven placement.
    InteractionGraph,

    /// Noise-aware placement.
    NoiseAware,

    /// SABRE-derived initial layout.
    Sabre,

    /// Caller-provided mapping.
    UserProvided,

    /// Custom layout strategy.
    Custom(String),
}

impl Default for LayoutSource {
    fn default() -> Self {
        Self::None
    }
}

impl LayoutSource {
    /// Returns a stable layout-source name.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::None => "none",
            Self::Trivial => "trivial",
            Self::Sequential => "sequential",
            Self::Connectivity => "connectivity",
            Self::InteractionGraph => "interaction_graph",
            Self::NoiseAware => "noise_aware",
            Self::Sabre => "sabre",
            Self::UserProvided => "user_provided",
            Self::Custom(name) => name.as_str(),
        }
    }
}

// =============================================================================
// Routing phase
// =============================================================================

/// Identifies the phase responsible for producing a routing record.
///
/// This allows diagnostics and metrics to distinguish layout, movement,
/// routing, and verification events without coupling the metrics system to
/// concrete implementation types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoutingPhase {
    /// Input validation.
    Validation,

    /// Initial layout selection.
    Layout,

    /// Main routing search.
    Routing,

    /// Movement generation.
    Movement,

    /// Mapping update.
    Mapping,

    /// Output verification.
    Verification,

    /// Result construction/metrics finalization.
    Finalization,
}

// =============================================================================
// Routing event
// =============================================================================

/// Deterministic diagnostic/event record emitted by a routing engine.
///
/// This is intentionally lightweight and contains no logging-framework
/// dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingEvent {
    /// A routing phase started.
    PhaseStarted {
        /// Phase identifier.
        phase: RoutingPhase,
    },

    /// A routing phase completed.
    PhaseCompleted {
        /// Phase identifier.
        phase: RoutingPhase,
    },

    /// A movement was selected.
    MovementSelected {
        /// Movement selected by the algorithm.
        movement: RoutingMove,
    },

    /// A gate became physically executable.
    GateRouted {
        /// Logical gate.
        gate: GateIdentity,

        /// Physical operands.
        physical_operands: Vec<PhysicalQubitId>,
    },

    /// A candidate was rejected.
    CandidateRejected {
        /// Human-readable reason.
        reason: String,
    },
}

// =============================================================================
// Routing limits
// =============================================================================

/// Safety limits consumed by routing implementations.
///
/// These are vocabulary-level limits only. Configuration ownership belongs to
/// `config.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoutingLimits {
    /// Maximum number of logical qubits accepted by one routing invocation.
    pub max_logical_qubits: usize,

    /// Maximum number of physical qubits considered by one routing
    /// invocation.
    pub max_physical_qubits: usize,

    /// Maximum number of operations processed.
    pub max_operations: usize,

    /// Maximum number of inserted movement operations.
    pub max_inserted_moves: usize,

    /// Maximum number of routing iterations.
    pub max_iterations: usize,
}

impl RoutingLimits {
    /// Creates routing limits.
    #[must_use]
    pub const fn new(
        max_logical_qubits: usize,
        max_physical_qubits: usize,
        max_operations: usize,
        max_inserted_moves: usize,
        max_iterations: usize,
    ) -> Self {
        Self {
            max_logical_qubits,
            max_physical_qubits,
            max_operations,
            max_inserted_moves,
            max_iterations,
        }
    }
}

impl Default for RoutingLimits {
    fn default() -> Self {
        Self {
            max_logical_qubits: 1_000_000,
            max_physical_qubits: 1_000_000,
            max_operations: 10_000_000,
            max_inserted_moves: 10_000_000,
            max_iterations: 100_000_000,
        }
    }
}

// =============================================================================
// Routing seed
// =============================================================================

/// Deterministic randomization seed.
///
/// Kept as a dedicated value object so future APIs cannot accidentally confuse
/// a routing seed with a qubit index, operation index, or configuration value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RoutingSeed(u64);

impl RoutingSeed {
    /// Creates a routing seed.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the seed value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for RoutingSeed {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<RoutingSeed> for u64 {
    fn from(seed: RoutingSeed) -> Self {
        seed.value()
    }
}

// =============================================================================
// Routing identifiers
// =============================================================================

/// Stable identifier for one routing invocation.
///
/// This is useful for diagnostics, reproducibility metadata, tracing, and
/// parallel routing trials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RoutingId(u64);

impl RoutingId {
    /// Creates a routing invocation identifier.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the identifier value.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for RoutingId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<RoutingId> for u64 {
    fn from(id: RoutingId) -> Self {
        id.value()
    }
}

// =============================================================================
// Candidate score
// =============================================================================

/// Comparable score attached to a routing candidate.
///
/// `f64` itself does not implement `Ord` because NaN is not ordered. Routing
/// algorithms must therefore never use raw floating-point values as map/set
/// keys or assume they are totally ordered.
///
/// This wrapper documents the contract that a candidate score is expected to
/// be finite. Validation of the actual value belongs to candidate/cost code.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CandidateScore {
    /// Numerical score.
    value: f64,
}

impl CandidateScore {
    /// Creates a candidate score when the value is finite.
    ///
    /// Returns `None` for NaN or infinity.
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        if value.is_finite() {
            Some(Self { value })
        } else {
            None
        }
    }

    /// Returns the score.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }
}

// =============================================================================
// Routing candidate
// =============================================================================

/// A candidate movement evaluated by a routing algorithm.
///
/// The actual score is optional because candidate generation and candidate
/// evaluation are intentionally separate phases.
#[derive(Debug, Clone, PartialEq)]
pub struct RoutingCandidate {
    /// Candidate movement.
    pub movement: RoutingMove,

    /// Candidate score, if already evaluated.
    pub score: Option<CandidateScore>,

    /// Search depth at which the candidate was evaluated.
    pub depth: usize,
}

impl RoutingCandidate {
    /// Creates an unevaluated candidate.
    #[must_use]
    pub const fn new(movement: RoutingMove, depth: usize) -> Self {
        Self {
            movement,
            score: None,
            depth,
        }
    }

    /// Returns a copy with the supplied score.
    #[must_use]
    pub const fn with_score(
        mut self,
        score: CandidateScore,
    ) -> Self {
        self.score = Some(score);
        self
    }
}

// =============================================================================
// Mapping change
// =============================================================================

/// A single logical-to-physical mapping transition.
///
/// This is a value object used for transaction logs, diagnostics, verification,
/// and reproducibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MappingChange {
    /// Logical qubit whose location changed.
    pub logical: LogicalQubitId,

    /// Previous physical location.
    pub from: PhysicalQubitId,

    /// New physical location.
    pub to: PhysicalQubitId,
}

impl MappingChange {
    /// Creates a mapping transition.
    #[must_use]
    pub const fn new(
        logical: LogicalQubitId,
        from: PhysicalQubitId,
        to: PhysicalQubitId,
    ) -> Self {
        Self {
            logical,
            from,
            to,
        }
    }
}

// =============================================================================
// Routing transaction state
// =============================================================================

/// State of a speculative routing transaction.
///
/// The actual transaction implementation belongs to `mapping.rs`/`router.rs`;
/// this enum provides the stable vocabulary used by those modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransactionState {
    /// Transaction has been created but not committed or rolled back.
    Active,

    /// Candidate changes have been committed.
    Committed,

    /// Candidate changes have been discarded.
    RolledBack,
}

impl Default for TransactionState {
    fn default() -> Self {
        Self::Active
    }
}

impl TransactionState {
    /// Returns whether the transaction is still active.
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    /// Returns whether the transaction was committed.
    #[must_use]
    pub const fn is_committed(self) -> bool {
        matches!(self, Self::Committed)
    }

    /// Returns whether the transaction was rolled back.
    #[must_use]
    pub const fn is_rolled_back(self) -> bool {
        matches!(self, Self::RolledBack)
    }
}

// =============================================================================
// Route disposition
// =============================================================================

/// Final disposition of a routing request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RouteDisposition {
    /// A route was successfully produced.
    Routed,

    /// The input was already physically executable and required no movement.
    AlreadyExecutable,

    /// No routing was requested.
    NotRequested,

    /// Routing completed using a configured fallback strategy.
    Fallback,

    /// Routing was deliberately approximated.
    Approximate,
}

impl RouteDisposition {
    /// Returns whether the routing request produced a usable routed result.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(
            self,
            Self::Routed
                | Self::AlreadyExecutable
                | Self::NotRequested
                | Self::Fallback
                | Self::Approximate
        )
    }

    /// Returns whether routing inserted or otherwise selected movement.
    #[must_use]
    pub const fn involved_routing(self) -> bool {
        matches!(
            self,
            Self::Routed
                | Self::Fallback
                | Self::Approximate
        )
    }
}

// =============================================================================
// Stable routing contract
// =============================================================================

/// High-level immutable description of what a routing invocation is expected
/// to operate on.
///
/// This is intentionally not the complete `RoutingInput` owned by `router.rs`.
/// It is a stable contract/value object that can be embedded into that richer
/// input without creating a dependency cycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingWorkload {
    /// Logical qubits participating in the workload.
    logical_qubits: Vec<LogicalQubitId>,

    /// Logical interactions in program order.
    interactions: Vec<QubitInteraction>,
}

impl RoutingWorkload {
    /// Creates a routing workload.
    #[must_use]
    pub fn new(
        logical_qubits: Vec<LogicalQubitId>,
        interactions: Vec<QubitInteraction>,
    ) -> Self {
        Self {
            logical_qubits,
            interactions,
        }
    }

    /// Returns the logical qubits.
    #[must_use]
    pub fn logical_qubits(&self) -> &[LogicalQubitId] {
        &self.logical_qubits
    }

    /// Returns the logical interactions.
    #[must_use]
    pub fn interactions(&self) -> &[QubitInteraction] {
        &self.interactions
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub fn logical_qubit_count(&self) -> usize {
        self.logical_qubits.len()
    }

    /// Returns the number of interactions.
    #[must_use]
    pub fn interaction_count(&self) -> usize {
        self.interactions.len()
    }
}

// =============================================================================
// Unit tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_and_physical_ids_are_distinct_types() {
        let logical = LogicalQubitId::new(3);
        let physical = PhysicalQubitId::new(3);

        assert_eq!(logical.index(), 3);
        assert_eq!(physical.index(), 3);
        assert_ne!(
            QubitRef::Logical(logical),
            QubitRef::Physical(physical)
        );
    }

    #[test]
    fn identifiers_have_stable_display_format() {
        assert_eq!(LogicalQubitId::new(7).to_string(), "q7");
        assert_eq!(PhysicalQubitId::new(11).to_string(), "p11");
    }

    #[test]
    fn qubit_ref_namespace_queries_are_correct() {
        let logical = QubitRef::Logical(LogicalQubitId::new(0));
        let physical = QubitRef::Physical(PhysicalQubitId::new(0));

        assert!(logical.is_logical());
        assert!(!logical.is_physical());
        assert_eq!(
            logical.logical(),
            Some(LogicalQubitId::new(0))
        );
        assert_eq!(logical.physical(), None);

        assert!(physical.is_physical());
        assert!(!physical.is_logical());
        assert_eq!(
            physical.physical(),
            Some(PhysicalQubitId::new(0))
        );
        assert_eq!(physical.logical(), None);
    }

    #[test]
    fn gate_identity_parses_common_gates() {
        assert_eq!(GateIdentity::from("cx"), GateIdentity::Cx);
        assert_eq!(GateIdentity::from("cnot"), GateIdentity::Cx);
        assert_eq!(GateIdentity::from("swap"), GateIdentity::Swap);
        assert_eq!(GateIdentity::from("ccx"), GateIdentity::Ccx);
        assert_eq!(GateIdentity::from("measure"), GateIdentity::Measure);
    }

    #[test]
    fn unknown_gate_is_preserved_as_custom() {
        let gate = GateIdentity::from("my_vendor_gate");

        assert_eq!(
            gate,
            GateIdentity::Custom("my_vendor_gate".to_owned())
        );
        assert!(!gate.is_builtin());
        assert_eq!(gate.name(), "my_vendor_gate");
    }

    #[test]
    fn directional_gate_classification_is_correct() {
        assert!(GateIdentity::Cx.is_directional());
        assert!(GateIdentity::Crx.is_directional());
        assert!(!GateIdentity::Cz.is_directional());
        assert!(!GateIdentity::Swap.is_directional());
    }

    #[test]
    fn physical_edge_preserves_direction() {
        let edge = PhysicalEdge::directed(
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(4),
        );

        assert_eq!(edge.a(), PhysicalQubitId::new(1));
        assert_eq!(edge.b(), PhysicalQubitId::new(4));
        assert_eq!(edge.direction(), EdgeDirection::Forward);
        assert!(!edge.is_self_loop());
        assert_eq!(
            edge.other(PhysicalQubitId::new(1)),
            Some(PhysicalQubitId::new(4))
        );
        assert_eq!(edge.other(PhysicalQubitId::new(99)), None);
    }

    #[test]
    fn routing_move_reports_touched_physical_qubits() {
        let movement = RoutingMove::Swap {
            a: PhysicalQubitId::new(1),
            b: PhysicalQubitId::new(2),
        };

        assert!(movement.is_swap());
        assert!(!movement.is_bridge());
        assert_eq!(
            movement.physical_qubits(),
            vec![
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(2)
            ]
        );
    }

    #[test]
    fn bridge_move_reports_all_endpoints() {
        let movement = RoutingMove::Bridge {
            a: PhysicalQubitId::new(0),
            bridge: PhysicalQubitId::new(1),
            b: PhysicalQubitId::new(2),
            gate: GateIdentity::Cx,
        };

        assert!(movement.is_bridge());
        assert_eq!(
            movement.physical_qubits(),
            vec![
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(2)
            ]
        );
    }

    #[test]
    fn routing_operation_preserves_logical_and_physical_operands() {
        let operation = RoutingOperation::Gate {
            gate: GateIdentity::Cx,
            operands: vec![
                PhysicalQubitId::new(4),
                PhysicalQubitId::new(7),
            ],
            logical_operands: vec![
                LogicalQubitId::new(0),
                LogicalQubitId::new(1),
            ],
        };

        assert!(operation.is_gate());
        assert_eq!(
            operation.physical_qubits(),
            vec![
                PhysicalQubitId::new(4),
                PhysicalQubitId::new(7)
            ]
        );
        assert_eq!(
            operation.logical_operands(),
            &[
                LogicalQubitId::new(0),
                LogicalQubitId::new(1)
            ]
        );
        assert_eq!(operation.gate(), Some(&GateIdentity::Cx));
    }

    #[test]
    fn routing_algorithm_names_are_stable() {
        assert_eq!(RoutingAlgorithm::Auto.name(), "auto");
        assert_eq!(RoutingAlgorithm::Basic.name(), "basic");
        assert_eq!(
            RoutingAlgorithm::ShortestPath.name(),
            "shortest_path"
        );
        assert_eq!(RoutingAlgorithm::Sabre.name(), "sabre");
        assert_eq!(
            RoutingAlgorithm::NoiseAware.name(),
            "noise_aware"
        );
    }

    #[test]
    fn routing_objective_names_are_stable() {
        assert_eq!(
            RoutingObjective::SwapCount.name(),
            "swap_count"
        );
        assert_eq!(
            RoutingObjective::Depth.name(),
            "depth"
        );
        assert_eq!(
            RoutingObjective::Duration.name(),
            "duration"
        );
        assert_eq!(
            RoutingObjective::Error.name(),
            "error"
        );
        assert_eq!(
            RoutingObjective::Fidelity.name(),
            "fidelity"
        );
    }

    #[test]
    fn candidate_score_rejects_non_finite_values() {
        assert!(CandidateScore::new(1.0).is_some());
        assert!(CandidateScore::new(f64::NAN).is_none());
        assert!(CandidateScore::new(f64::INFINITY).is_none());
        assert!(CandidateScore::new(f64::NEG_INFINITY).is_none());
    }

    #[test]
    fn routing_limits_have_safe_defaults() {
        let limits = RoutingLimits::default();

        assert!(limits.max_logical_qubits > 0);
        assert!(limits.max_physical_qubits > 0);
        assert!(limits.max_operations > 0);
        assert!(limits.max_inserted_moves > 0);
        assert!(limits.max_iterations > 0);
    }

    #[test]
    fn routing_seed_round_trips() {
        let seed = RoutingSeed::new(42);

        assert_eq!(seed.value(), 42);
        assert_eq!(u64::from(seed), 42);
        assert_eq!(
            RoutingSeed::from(42_u64),
            RoutingSeed::new(42)
        );
    }

    #[test]
    fn routing_transaction_state_is_explicit() {
        assert!(TransactionState::Active.is_active());
        assert!(TransactionState::Committed.is_committed());
        assert!(TransactionState::RolledBack.is_rolled_back());
    }

    #[test]
    fn routing_workload_preserves_program_order() {
        let q0 = LogicalQubitId::new(0);
        let q1 = LogicalQubitId::new(1);

        let first = QubitInteraction::new(
            vec![q0, q1],
            GateIdentity::Cx,
        );

        let second = QubitInteraction::new(
            vec![q1, q0],
            GateIdentity::Cz,
        );

        let workload = RoutingWorkload::new(
            vec![q0, q1],
            vec![first.clone(), second.clone()],
        );

        assert_eq!(workload.logical_qubit_count(), 2);
        assert_eq!(workload.interaction_count(), 2);
        assert_eq!(workload.interactions()[0], first);
        assert_eq!(workload.interactions()[1], second);
    }

    #[test]
    fn interaction_reports_arity() {
        let interaction = QubitInteraction::new(
            vec![
                LogicalQubitId::new(0),
                LogicalQubitId::new(1),
                LogicalQubitId::new(2),
            ],
            GateIdentity::Ccx,
        );

        assert_eq!(interaction.arity(), 3);
        assert!(interaction.is_multi_qubit());
        assert!(!interaction.is_two_qubit());
        assert!(!interaction.is_single_qubit());
    }

    #[test]
    fn qubit_roles_have_expected_availability() {
        assert!(QubitRole::Data.is_usable());
        assert!(QubitRole::Ancilla.is_usable());
        assert!(QubitRole::Syndrome.is_usable());
        assert!(QubitRole::Reserved.is_usable());
        assert!(!QubitRole::Unavailable.is_usable());
        assert!(QubitRole::Unavailable.is_unavailable());
    }

    #[test]
    fn verification_levels_have_expected_behavior() {
        assert!(!VerificationLevel::None.enabled());
        assert!(VerificationLevel::Basic.enabled());
        assert!(VerificationLevel::Standard.enabled());
        assert!(VerificationLevel::Strict.enabled());
        assert!(VerificationLevel::Strict.is_strict());
        assert!(!VerificationLevel::Standard.is_strict());
    }
}