//! Zamani Quantum Routing — Bridge Move
//!
//! Production-grade semantic bridge-routing primitive.
//!
//! # Purpose
//!
//! `BridgeMove` represents a topology-aware interaction between two physical
//! qubits that are not directly adjacent but are connected through exactly one
//! intermediate physical qubit:
//
//! ```text
//! control ───── bridge ───── target
//! ```
//!
//! The bridge qubit is a physical connectivity intermediary. The bridge move
//! itself does NOT define the hardware-level decomposition of the requested
//! quantum operation.
//!
//! This distinction is essential:
//
//! ```text
//! Routing layer
//!
//!     BridgeMove(p0, p1, p2, gate)
//!                 │
//!                 ▼
//!     "this interaction can use p1 as the connectivity bridge"
//!
//! Hardware lowering
//!
//!     BridgeMove
//!         │
//!         ├── provider-supported bridge primitive
//!         ├── CX-based remote interaction
//!         ├── native echoed construction
//!         ├── ISA-specific synthesis
//!         └── reject if the target cannot implement it
//! ```
//!
//! The routing layer therefore MUST NOT assume a universal decomposition.
//!
//! # Why exactly one intermediate qubit?
//!
//! A three-vertex path is the canonical bridge topology:
//
//! ```text
//! p0 ─ p1 ─ p2
//! ```
//!
//! A longer path:
//
//! ```text
//! p0 ─ p1 ─ p2 ─ p3
//! ```
//!
//! is not automatically equivalent to a single bridge operation. Its correct
//! synthesis depends on the target ISA and gate semantics. Treating arbitrary
//! paths as a universal bridge would therefore create an incorrect compiler
//! abstraction.
//!
//! Longer paths belong to path finding, decomposition, or repeated movement
//! planning.
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - the semantic `BridgeMove` representation;
//! - local endpoint/intermediate validation;
//! - topology validation;
//! - deterministic endpoint/intermediate access;
//! - bridge-path validation;
//! - canonical identity/order;
//! - bridge operation metadata;
//! - immutable bridge construction;
//! - conversion-independent integration with later hardware lowering;
//! - unit tests for bridge invariants.
//!
//! This module does NOT:
//!
//! - mutate logical-to-physical mappings;
//! - insert gates into a quantum circuit;
//! - decompose bridge operations;
//! - assume a particular gate decomposition;
//! - generate OpenQASM;
//! - generate pulses;
//! - schedule operations;
//! - access hardware;
//! - access calibration providers;
//! - choose routing algorithms;
//! - choose layouts;
//! - perform simulation;
//! - perform QEC;
//! - perform gate synthesis.
//!
//! # Integration contract
//!
//! The dependency direction is:
//
//! ```text
//!                    types.rs
//!                       │
//!                       ▼
//!                    errors.rs
//!                       │
//!                       ▼
//!                  topology.rs
//!                       │
//!                       ▼
//!                  bridge.rs
//!                       │
//!          ┌────────────┼─────────────┐
//!          ▼            ▼             ▼
//!      candidates    algorithms     router
//!          │            │             │
//!          └────────────┼─────────────┘
//!                       ▼
//!                  verification
//!                       │
//!                       ▼
//!                 hardware lowering
//! ```
//!
//! `bridge.rs` deliberately does not depend on `mapping.rs`, `router.rs`,
//! `algorithms/*`, `result.rs`, or `transpiler.rs`. This allows this file to be
//! completed and frozen before those later modules are implemented.
//!
//! # Mapping semantics
//!
//! A `BridgeMove` does NOT change logical-to-physical placement.
//
//! Unlike:
//
//! ```text
//! SwapMove
//! ```
//!
//! which exchanges physical states, a bridge is an interaction primitive. The
//! bridge qubit is used as part of the physical implementation of the
//! interaction, but this type does not declare that the logical states move.
//!
//! Consequently:
//
//! ```text
//! before mapping:
//!
//! q0 -> p0
//! q1 -> p1
//! q2 -> p2
//!
//! BridgeMove(p0, p1, p2, CX)
//!
//! after routing:
//!
//! q0 -> p0
//! q1 -> p1
//! q2 -> p2
//! ```
//!
//! The actual hardware lowering determines how the interaction is implemented.
//!
//! # Gate semantics
//!
//! A bridge operation contains a routing-level gate identity. The gate is
//! intentionally represented using `GateIdentity` rather than a hardware gate
//! enum.
//!
//! This file does NOT claim that every gate can be bridged.
//!
//! The hardware target, gate set, or lowering stage is responsible for
//! answering:
//
//! ```text
//! "Can this specific gate be implemented using this bridge construction?"
//! ```
//!
//! For this reason, `BridgeMove::validate()` validates topology structure, not
//! hardware-specific gate synthesis.
//!
//! # Directed connectivity
//!
//! Structural adjacency is checked using `Topology::is_adjacent()`.
//!
//! Whether a particular directed gate is executable on either edge is NOT
//! inferred from adjacency. Gate-specific legality remains a responsibility of
//! the topology/hardware/lowering boundary.
//!
//! This prevents the bridge primitive from making the dangerous assumption:
//
//! ```text
//! adjacent == executable
//! ```
//!
//! # Safety
//!
//! - No `unsafe` code.
//! - No raw pointers.
//! - No unchecked indexing.
//! - No mutation of caller state.
//! - No hidden global state.
//! - No hardware access.
//! - No floating-point routing decisions.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only.
//!
//! No nightly features are required.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::cmp::Ordering;
use std::fmt;

use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::topology::Topology;
use crate::quantum::routing::types::{
    GateIdentity,
    PhysicalQubitId,
};

// =============================================================================
// BridgeMove
// =============================================================================

/// A semantic topology-aware bridge interaction.
//
/// A bridge consists of exactly three distinct physical locations:
//
/// ```text
//! control ───── bridge ───── target
//!     p0           p1          p2
//! ```
//!
//! The endpoints (`control`, `target`) are the qubits whose logical interaction
//! is being routed. The middle physical qubit is the connectivity bridge.
//
//! # Important
//!
//! `BridgeMove` is a routing primitive, not a hardware gate.
//
//! It does not claim that:
//
//! ```text
//! gate(control, target)
//! ```
//!
//! can be executed directly by the hardware.
//
//! It only records that the interaction has been selected for bridge-style
//! handling and that the topology contains the required three-vertex path.
//
//! Hardware lowering must subsequently determine whether the requested gate is
//! supported and how it should be synthesized.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BridgeMove {
    control: PhysicalQubitId,
    bridge: PhysicalQubitId,
    target: PhysicalQubitId,
    gate: GateIdentity,
}

impl BridgeMove {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates a semantic bridge move.
    ///
    /// Construction validates only local invariants:
    ///
    /// - control != bridge;
    /// - bridge != target;
    /// - control != target.
    ///
    /// Topology-dependent validation is performed by [`Self::validate`].
    ///
    /// This separation allows routing algorithms to construct candidates before
    /// a topology-specific validation step.
    pub fn new(
        control: PhysicalQubitId,
        bridge: PhysicalQubitId,
        target: PhysicalQubitId,
        gate: GateIdentity,
    ) -> Result<Self, RoutingError> {
        validate_distinct(control, bridge, target)?;

        if gate.is_non_unitary() {
            return Err(RoutingError::InvalidMove {
                message: format!(
                    "bridge operation cannot represent non-unitary gate `{}`",
                    gate.name()
                ),
            });
        }

        Ok(Self {
            control,
            bridge,
            target,
            gate,
        })
    }

    /// Returns the control physical qubit.
    #[inline]
    #[must_use]
    pub const fn control(self) -> PhysicalQubitId {
        self.control
    }

    /// Returns the intermediate bridge physical qubit.
    #[inline]
    #[must_use]
    pub const fn bridge(self) -> PhysicalQubitId {
        self.bridge
    }

    /// Returns the target physical qubit.
    #[inline]
    #[must_use]
    pub const fn target(self) -> PhysicalQubitId {
        self.target
    }

    /// Returns the requested routing-level gate identity.
    #[inline]
    #[must_use]
    pub const fn gate(&self) -> &GateIdentity {
        &self.gate
    }

    /// Returns the gate's canonical routing-level name.
    #[inline]
    #[must_use]
    pub fn gate_name(&self) -> &str {
        self.gate.name()
    }

    /// Returns the three physical vertices in semantic order.
    ///
    /// The order is:
    ///
    /// ```text
    /// control, bridge, target
    /// ```
    #[inline]
    #[must_use]
    pub const fn vertices(
        self,
    ) -> (
        PhysicalQubitId,
        PhysicalQubitId,
        PhysicalQubitId,
    ) {
        (self.control, self.bridge, self.target)
    }

    /// Returns the two physical edges required by the bridge.
    ///
    /// The first edge is:
    ///
    /// ```text
    /// control -> bridge
    /// ```
    ///
    /// The second edge is:
    ///
    /// ```text
    /// bridge -> target
    /// ```
    ///
    /// The returned edges represent structural topology requirements. They do
    /// not imply gate directionality.
    #[inline]
    #[must_use]
    pub const fn edges(
        self,
    ) -> (
        (PhysicalQubitId, PhysicalQubitId),
        (PhysicalQubitId, PhysicalQubitId),
    ) {
        (
            (self.control, self.bridge),
            (self.bridge, self.target),
        )
    }

    /// Returns the physical endpoints of the logical interaction.
    #[inline]
    #[must_use]
    pub const fn endpoints(
        self,
    ) -> (PhysicalQubitId, PhysicalQubitId) {
        (self.control, self.target)
    }

    /// Returns the number of intermediate physical qubits.
    ///
    /// A `BridgeMove` always has exactly one.
    #[inline]
    #[must_use]
    pub const fn intermediate_count(self) -> usize {
        1
    }

    /// Returns the physical path length in edges.
    ///
    /// A canonical bridge has exactly two topology edges.
    #[inline]
    #[must_use]
    pub const fn path_length(self) -> usize {
        2
    }

    /// Returns the number of physical vertices in the bridge path.
    #[inline]
    #[must_use]
    pub const fn vertex_count(self) -> usize {
        3
    }

    /// Returns whether the bridge has distinct endpoints.
    #[inline]
    #[must_use]
    pub const fn has_distinct_endpoints(self) -> bool {
        self.control != self.target
    }

    /// Returns whether the bridge is structurally non-trivial.
    #[inline]
    #[must_use]
    pub const fn is_non_trivial(self) -> bool {
        self.control != self.bridge
            && self.bridge != self.target
            && self.control != self.target
    }

    // =========================================================================
    // Topology validation
    // =========================================================================

    /// Validates the bridge against a physical topology.
    ///
    /// Validation checks:
    ///
    /// 1. all three physical qubits exist;
    /// 2. all three physical qubits are distinct;
    /// 3. control and bridge are adjacent;
    /// 4. bridge and target are adjacent;
    /// 5. control and target are not accidentally collapsed into one location.
    ///
    /// This method does NOT check whether the requested gate has a supported
    /// bridge decomposition. That belongs to hardware lowering.
    pub fn validate(
        &self,
        topology: &Topology,
    ) -> Result<(), RoutingError> {
        validate_distinct(
            self.control,
            self.bridge,
            self.target,
        )?;

        if !topology.contains(self.control) {
            return Err(RoutingError::InvalidPhysicalQubit {
                qubit: self.control,
            });
        }

        if !topology.contains(self.bridge) {
            return Err(RoutingError::InvalidPhysicalQubit {
                qubit: self.bridge,
            });
        }

        if !topology.contains(self.target) {
            return Err(RoutingError::InvalidPhysicalQubit {
                qubit: self.target,
            });
        }

        if !topology.is_adjacent(self.control, self.bridge) {
            return Err(RoutingError::UnsupportedMove {
                message: format!(
                    "bridge requires physical adjacency between {} and {}",
                    self.control, self.bridge
                ),
            });
        }

        if !topology.is_adjacent(self.bridge, self.target) {
            return Err(RoutingError::UnsupportedMove {
                message: format!(
                    "bridge requires physical adjacency between {} and {}",
                    self.bridge, self.target
                ),
            });
        }

        Ok(())
    }

    /// Validates the bridge and returns the canonical physical path.
    ///
    /// The returned path is:
    ///
    /// ```text
    /// [control, bridge, target]
    /// ```
    pub fn validated_path(
        &self,
        topology: &Topology,
    ) -> Result<[PhysicalQubitId; 3], RoutingError> {
        self.validate(topology)?;

        Ok([
            self.control,
            self.bridge,
            self.target,
        ])
    }

    // =========================================================================
    // Gate semantics boundary
    // =========================================================================

    /// Returns whether the requested gate is a two-qubit operation candidate.
    ///
    /// The bridge primitive is intended for two-endpoint interactions.
    ///
    /// This method intentionally recognizes the routing-level gate identity
    /// without asserting that the target hardware can synthesize it.
    #[must_use]
    pub fn is_two_qubit_gate(&self) -> bool {
        matches!(
            self.gate,
            GateIdentity::Cx
                | GateIdentity::Cy
                | GateIdentity::Cz
                | GateIdentity::Ch
                | GateIdentity::Crx
                | GateIdentity::Cry
                | GateIdentity::Crz
                | GateIdentity::Ecr
                | GateIdentity::ISwap
                | GateIdentity::Swap
                | GateIdentity::Custom(_)
        )
    }

    /// Returns whether the operation is a custom gate.
    #[inline]
    #[must_use]
    pub fn is_custom_gate(&self) -> bool {
        matches!(self.gate, GateIdentity::Custom(_))
    }

    /// Returns whether the gate is one for which bridge-style routing is
    /// commonly meaningful at the routing boundary.
    ///
    /// This is intentionally conservative.
    ///
    /// `Custom` returns `true` because the actual gate semantics are owned by
    /// the target ISA, not this routing module.
    ///
    /// This method does NOT guarantee hardware support.
    #[must_use]
    pub fn is_bridge_candidate(&self) -> bool {
        matches!(
            self.gate,
            GateIdentity::Cx
                | GateIdentity::Cy
                | GateIdentity::Cz
                | GateIdentity::Ch
                | GateIdentity::Crx
                | GateIdentity::Cry
                | GateIdentity::Crz
                | GateIdentity::Ecr
                | GateIdentity::Custom(_)
        )
    }

    // =========================================================================
    // Canonical identity
    // =========================================================================

    /// Returns a canonical representation suitable for deterministic
    /// deduplication.
    ///
    /// The control/target ordering is preserved because controlled gates are
    /// generally directional:
    ///
    /// ```text
    /// CX(control,target)
    /// ```
    ///
    /// is not semantically equivalent to:
    ///
    /// ```text
    /// CX(target,control)
    /// ```
    ///
    /// The bridge qubit is never reordered with the endpoints.
    #[inline]
    #[must_use]
    pub const fn canonical(self) -> Self {
        self
    }

    /// Returns whether two bridge moves describe the same semantic operation.
    ///
    /// Endpoint order matters because the gate may be directional.
    #[inline]
    #[must_use]
    pub fn equivalent(self, other: Self) -> bool {
        self == other
    }

    /// Returns the reverse-endpoint bridge.
    ///
    /// This reverses control and target but retains the same physical bridge
    /// location.
    ///
    /// This is NOT guaranteed to be semantically equivalent for directional
    /// gates. Callers must use it only when the gate semantics permit endpoint
    /// reversal.
    #[must_use]
    pub fn reversed(self) -> Self {
        Self {
            control: self.target,
            bridge: self.bridge,
            target: self.control,
            gate: self.gate,
        }
    }

    // =========================================================================
    // Cost/metric boundary
    // =========================================================================

    /// Returns the number of semantic movement operations represented by the
    /// bridge.
    ///
    /// This is one routing-level bridge operation.
    ///
    /// It deliberately does not return the number of low-level gates because
    /// that depends on hardware lowering.
    #[inline]
    #[must_use]
    pub const fn operation_count(self) -> usize {
        1
    }

    /// Returns the number of topology edges traversed by the bridge path.
    #[inline]
    #[must_use]
    pub const fn physical_distance(self) -> usize {
        2
    }

    /// Returns whether this operation changes logical-to-physical placement.
    ///
    /// Bridge operations do not.
    #[inline]
    #[must_use]
    pub const fn changes_mapping(self) -> bool {
        false
    }

    /// Returns whether the bridge requires the intermediate physical qubit.
    ///
    /// This is always true for a valid bridge.
    #[inline]
    #[must_use]
    pub const fn consumes_intermediate(self) -> bool {
        true
    }

    // =========================================================================
    // Formatting
    // =========================================================================

    /// Returns a deterministic textual representation.
    #[must_use]
    pub fn display_name(&self) -> String {
        format!(
            "BRIDGE({}, {}, {}; {})",
            self.control,
            self.bridge,
            self.target,
            self.gate_name()
        )
    }
}

impl fmt::Display for BridgeMove {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "BRIDGE({}, {}, {}; {})",
            self.control,
            self.bridge,
            self.target,
            self.gate_name()
        )
    }
}

// =============================================================================
// Deterministic ordering
// =============================================================================

impl Ord for BridgeMove {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.control
            .cmp(&other.control)
            .then_with(|| self.bridge.cmp(&other.bridge))
            .then_with(|| self.target.cmp(&other.target))
            .then_with(|| self.gate_name().cmp(other.gate_name()))
    }
}

impl PartialOrd for BridgeMove {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// =============================================================================
// Bridge path
// =============================================================================

/// Immutable validated three-vertex bridge path.
///
/// This type separates path validity from the gate-specific bridge operation.
///
/// A `BridgePath` represents only:
//
//! ```text
//! p0 ─ p1 ─ p2
//! ```
//
//! It does not contain a gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BridgePath {
    control: PhysicalQubitId,
    bridge: PhysicalQubitId,
    target: PhysicalQubitId,
}

impl BridgePath {
    /// Creates a bridge path after checking local vertex distinctness.
    pub fn new(
        control: PhysicalQubitId,
        bridge: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> Result<Self, RoutingError> {
        validate_distinct(control, bridge, target)?;

        Ok(Self {
            control,
            bridge,
            target,
        })
    }

    /// Creates and topology-validates a bridge path.
    pub fn validated(
        topology: &Topology,
        control: PhysicalQubitId,
        bridge: PhysicalQubitId,
        target: PhysicalQubitId,
    ) -> Result<Self, RoutingError> {
        let path = Self::new(
            control,
            bridge,
            target,
        )?;

        path.validate(topology)?;

        Ok(path)
    }

    /// Returns the first endpoint.
    #[inline]
    #[must_use]
    pub const fn control(self) -> PhysicalQubitId {
        self.control
    }

    /// Returns the intermediate vertex.
    #[inline]
    #[must_use]
    pub const fn bridge(self) -> PhysicalQubitId {
        self.bridge
    }

    /// Returns the second endpoint.
    #[inline]
    #[must_use]
    pub const fn target(self) -> PhysicalQubitId {
        self.target
    }

    /// Returns the complete path as a fixed-size array.
    #[inline]
    #[must_use]
    pub const fn vertices(
        self,
    ) -> [PhysicalQubitId; 3] {
        [
            self.control,
            self.bridge,
            self.target,
        ]
    }

    /// Returns the two required topology edges.
    #[inline]
    #[must_use]
    pub const fn edges(
        self,
    ) -> [
        (PhysicalQubitId, PhysicalQubitId);
        2
    ] {
        [
            (self.control, self.bridge),
            (self.bridge, self.target),
        ]
    }

    /// Validates this path against the physical topology.
    pub fn validate(
        &self,
        topology: &Topology,
    ) -> Result<(), RoutingError> {
        if !topology.contains(self.control) {
            return Err(RoutingError::InvalidPhysicalQubit {
                qubit: self.control,
            });
        }

        if !topology.contains(self.bridge) {
            return Err(RoutingError::InvalidPhysicalQubit {
                qubit: self.bridge,
            });
        }

        if !topology.contains(self.target) {
            return Err(RoutingError::InvalidPhysicalQubit {
                qubit: self.target,
            });
        }

        if !topology.is_adjacent(self.control, self.bridge) {
            return Err(RoutingError::UnsupportedMove {
                message: format!(
                    "bridge path requires adjacency between {} and {}",
                    self.control,
                    self.bridge
                ),
            });
        }

        if !topology.is_adjacent(self.bridge, self.target) {
            return Err(RoutingError::UnsupportedMove {
                message: format!(
                    "bridge path requires adjacency between {} and {}",
                    self.bridge,
                    self.target
                ),
            });
        }

        Ok(())
    }

    /// Returns the number of physical edges in the path.
    #[inline]
    #[must_use]
    pub const fn edge_count(self) -> usize {
        2
    }

    /// Returns the number of physical vertices in the path.
    #[inline]
    #[must_use]
    pub const fn vertex_count(self) -> usize {
        3
    }

    /// Returns the endpoint pair.
    #[inline]
    #[must_use]
    pub const fn endpoints(
        self,
    ) -> (PhysicalQubitId, PhysicalQubitId) {
        (self.control, self.target)
    }

    /// Returns whether this path is a valid non-trivial bridge path without
    /// consulting a topology.
    #[inline]
    #[must_use]
    pub const fn is_non_trivial(self) -> bool {
        self.control != self.bridge
            && self.bridge != self.target
            && self.control != self.target
    }
}

// =============================================================================
// Construction helpers
// =============================================================================

/// Constructs a bridge move after local validation.
///
/// Topology validation must be performed by [`BridgeMove::validate`].
pub fn candidate(
    control: PhysicalQubitId,
    bridge: PhysicalQubitId,
    target: PhysicalQubitId,
    gate: GateIdentity,
) -> Result<BridgeMove, RoutingError> {
    BridgeMove::new(
        control,
        bridge,
        target,
        gate,
    )
}

/// Constructs a bridge move only when the physical topology contains the
/// required two-edge path.
pub fn candidate_if_valid(
    topology: &Topology,
    control: PhysicalQubitId,
    bridge: PhysicalQubitId,
    target: PhysicalQubitId,
    gate: GateIdentity,
) -> Result<Option<BridgeMove>, RoutingError> {
    let move_ = BridgeMove::new(
        control,
        bridge,
        target,
        gate,
    )?;

    if !topology.contains(control)
        || !topology.contains(bridge)
        || !topology.contains(target)
    {
        return Ok(None);
    }

    if !topology.is_adjacent(control, bridge) {
        return Ok(None);
    }

    if !topology.is_adjacent(bridge, target) {
        return Ok(None);
    }

    Ok(Some(move_))
}

// =============================================================================
// Internal validation
// =============================================================================

/// Validates the local bridge geometry.
fn validate_distinct(
    control: PhysicalQubitId,
    bridge: PhysicalQubitId,
    target: PhysicalQubitId,
) -> Result<(), RoutingError> {
    if control == bridge {
        return Err(RoutingError::InvalidMove {
            message: format!(
                "bridge control {} and intermediate {} must be distinct",
                control, bridge
            ),
        });
    }

    if bridge == target {
        return Err(RoutingError::InvalidMove {
            message: format!(
                "bridge intermediate {} and target {} must be distinct",
                bridge, target
            ),
        });
    }

    if control == target {
        return Err(RoutingError::InvalidMove {
            message: format!(
                "bridge endpoints {} and {} must be distinct",
                control, target
            ),
        });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn topology() -> Topology {
        Topology::line(3)
            .expect("three-qubit line topology must be valid")
    }

    fn cx_bridge() -> BridgeMove {
        BridgeMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
            GateIdentity::Cx,
        )
        .expect("valid CX bridge must construct")
    }

    #[test]
    fn constructs_valid_bridge() {
        let bridge = cx_bridge();

        assert_eq!(
            bridge.control(),
            PhysicalQubitId::new(0)
        );
        assert_eq!(
            bridge.bridge(),
            PhysicalQubitId::new(1)
        );
        assert_eq!(
            bridge.target(),
            PhysicalQubitId::new(2)
        );
        assert_eq!(
            bridge.gate(),
            &GateIdentity::Cx
        );
    }

    #[test]
    fn rejects_control_equal_to_bridge() {
        let result = BridgeMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(2),
            GateIdentity::Cx,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_bridge_equal_to_target() {
        let result = BridgeMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(1),
            GateIdentity::Cx,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_control_equal_to_target() {
        let result = BridgeMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(0),
            GateIdentity::Cx,
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_unitary_gate() {
        let result = BridgeMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
            GateIdentity::Measure,
        );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_custom_gate() {
        let result = BridgeMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
            GateIdentity::Custom(
                "zamani_remote_entangle".to_string(),
            ),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn validates_line_topology() {
        let bridge = cx_bridge();

        bridge
            .validate(&topology())
            .expect("line topology must support 0-1-2 bridge");
    }

    #[test]
    fn rejects_missing_control_qubit() {
        let topology = Topology::line(2)
            .expect("two-qubit topology must be valid");

        let bridge = BridgeMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
            GateIdentity::Cx,
        )
        .expect("local bridge construction should succeed");

        assert!(bridge.validate(&topology).is_err());
    }

    #[test]
    fn rejects_missing_bridge_qubit() {
        let topology = Topology::isolated(3)
            .expect("isolated topology must be valid");

        let bridge = cx_bridge();

        assert!(bridge.validate(&topology).is_err());
    }

    #[test]
    fn rejects_missing_target_qubit() {
        let topology = Topology::line(2)
            .expect("two-qubit topology must be valid");

        let bridge = cx_bridge();

        assert!(bridge.validate(&topology).is_err());
    }

    #[test]
    fn rejects_missing_first_edge() {
        let topology = Topology::isolated(3)
            .expect("isolated topology must be valid");

        let bridge = cx_bridge();

        assert!(bridge.validate(&topology).is_err());
    }

    #[test]
    fn rejects_missing_second_edge() {
        let topology = Topology::builder()
            .add_qubit(PhysicalQubitId::new(0))
            .add_qubit(PhysicalQubitId::new(1))
            .add_qubit(PhysicalQubitId::new(2))
            .add_undirected_edge(
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
            )
            .build()
            .expect("partial topology must be structurally valid");

        let bridge = cx_bridge();

        assert!(bridge.validate(&topology).is_err());
    }

    #[test]
    fn validated_path_returns_exact_three_vertices() {
        let bridge = cx_bridge();

        let path = bridge
            .validated_path(&topology())
            .expect("bridge path must validate");

        assert_eq!(
            path,
            [
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1),
                PhysicalQubitId::new(2),
            ]
        );
    }

    #[test]
    fn exposes_correct_edges() {
        let bridge = cx_bridge();

        assert_eq!(
            bridge.edges(),
            [
                (
                    PhysicalQubitId::new(0),
                    PhysicalQubitId::new(1),
                ),
                (
                    PhysicalQubitId::new(1),
                    PhysicalQubitId::new(2),
                ),
            ]
        );
    }

    #[test]
    fn does_not_change_mapping() {
        let bridge = cx_bridge();

        assert!(!bridge.changes_mapping());
    }

    #[test]
    fn has_one_intermediate() {
        let bridge = cx_bridge();

        assert_eq!(
            bridge.intermediate_count(),
            1
        );
    }

    #[test]
    fn has_two_physical_edges() {
        let bridge = cx_bridge();

        assert_eq!(
            bridge.path_length(),
            2
        );
    }

    #[test]
    fn has_three_physical_vertices() {
        let bridge = cx_bridge();

        assert_eq!(
            bridge.vertex_count(),
            3
        );
    }

    #[test]
    fn reports_two_qubit_gate() {
        let bridge = cx_bridge();

        assert!(bridge.is_two_qubit_gate());
    }

    #[test]
    fn reports_bridge_candidate() {
        let bridge = cx_bridge();

        assert!(bridge.is_bridge_candidate());
    }

    #[test]
    fn custom_gate_is_bridge_candidate() {
        let bridge = BridgeMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
            GateIdentity::Custom(
                "remote_gate".to_string(),
            ),
        )
        .expect("custom bridge should construct");

        assert!(bridge.is_bridge_candidate());
        assert!(bridge.is_custom_gate());
    }

    #[test]
    fn reversed_bridge_preserves_intermediate() {
        let bridge = cx_bridge();
        let reversed = bridge.reversed();

        assert_eq!(
            reversed.control(),
            PhysicalQubitId::new(2)
        );
        assert_eq!(
            reversed.bridge(),
            PhysicalQubitId::new(1)
        );
        assert_eq!(
            reversed.target(),
            PhysicalQubitId::new(0)
        );
    }

    #[test]
    fn display_is_deterministic() {
        let bridge = cx_bridge();

        assert_eq!(
            bridge.to_string(),
            "BRIDGE(p0, p1, p2; cx)"
        );
    }

    #[test]
    fn ordering_is_deterministic() {
        let a = BridgeMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
            GateIdentity::Cx,
        )
        .expect("bridge A must construct");

        let b = BridgeMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(2),
            PhysicalQubitId::new(3),
            GateIdentity::Cx,
        )
        .expect("bridge B must construct");

        assert!(a < b);
    }

    #[test]
    fn equivalent_preserves_direction() {
        let forward = cx_bridge();

        let reverse = forward.reversed();

        assert!(!forward.equivalent(reverse));
    }

    #[test]
    fn candidate_if_valid_accepts_valid_path() {
        let result = candidate_if_valid(
            &topology(),
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
            GateIdentity::Cx,
        )
        .expect("candidate generation must not fail");

        assert!(result.is_some());
    }

    #[test]
    fn candidate_if_valid_rejects_nonexistent_path() {
        let result = candidate_if_valid(
            &Topology::isolated(3)
                .expect("topology must construct"),
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
            GateIdentity::Cx,
        )
        .expect("candidate generation must not fail");

        assert!(result.is_none());
    }

    #[test]
    fn bridge_path_validates() {
        let path = BridgePath::validated(
            &topology(),
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
        )
        .expect("bridge path must validate");

        assert_eq!(
            path.edge_count(),
            2
        );
        assert_eq!(
            path.vertex_count(),
            3
        );
    }

    #[test]
    fn bridge_path_rejects_repeated_vertex() {
        let result = BridgePath::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(1),
        );

        assert!(result.is_err());
    }

    #[test]
    fn bridge_path_reports_endpoints() {
        let path = BridgePath::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
        )
        .expect("path must construct");

        assert_eq!(
            path.endpoints(),
            (
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(2),
            )
        );
    }
}