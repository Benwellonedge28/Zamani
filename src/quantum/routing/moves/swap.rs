//! Zamani Quantum Routing — SWAP Move
//!
//! Production-grade physical SWAP movement primitive.
//!
//! # Responsibility
//!
//! This module represents the semantic routing operation:
//!
//! ```text
//! physical qubit A <──────> physical qubit B
//! ```
//!
//! A [`SwapMove`] is a routing/mapping operation. It is deliberately NOT a
//! hardware gate implementation.
//!
//! The distinction is important:
//!
//! ```text
//! Routing layer:
//!     SwapMove(p0, p1)
//!             │
//!             ▼
//!     exchange logical states/mapping positions
//!
//! Hardware lowering:
//!     SwapMove
//!        │
//!        ├── native SWAP
//!        ├── 3 × CX
//!        ├── provider-specific decomposition
//!        └── other hardware-supported implementation
//! ```
//!
//! Therefore this module must never:
//!
//! - emit OpenQASM;
//! - construct provider-specific gates;
//! - generate pulses;
//! - perform scheduling;
//! - access a QPU;
//! - perform calibration;
//! - choose a routing algorithm;
//! - choose a layout;
//! - mutate a quantum circuit directly;
//! - assume that SWAP is a native hardware instruction.
//!
//! # Architectural dependencies
//!
//! This file depends only on the stable routing contracts:
//!
//! - `routing::types::PhysicalQubitId`
//! - `routing::errors::RoutingError`
//! - `routing::mapping::QubitMapping`
//! - `routing::topology::Topology`
//!
//! Later routing algorithms (`basic`, `shortest_path`, `lookahead`, `sabre`,
//! `noise_aware`, `dynamic`) consume this primitive rather than implementing
//! their own SWAP semantics.
//!
//! # Mapping invariant
//!
//! A physical SWAP exchanges the logical states occupying the two physical
//! locations. It does NOT mean that the logical qubit identities themselves
//! change.
//!
//! Example:
//!
//! ```text
//! before:
//!     logical q0 -> physical p0
//!     logical q1 -> physical p1
//!
//! SwapMove(p0, p1)
//!
//! after:
//!     logical q0 -> physical p1
//!     logical q1 -> physical p0
//! ```
//!
//! # Safety
//!
//! - No `unsafe` code.
//! - No raw pointer manipulation.
//! - No unchecked indexing.
//! - No implicit integer conversion between logical and physical IDs.
//! - Validation occurs before mutation.
//! - Mapping mutation is performed only through `QubitMapping`.
//! - A failed validation cannot partially mutate the mapping.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//!
//! This module intentionally uses only stable Rust facilities available to
//! that toolchain.

use crate::quantum::routing::errors::RoutingError;
use crate::quantum::routing::mapping::QubitMapping;
use crate::quantum::routing::topology::Topology;
use crate::quantum::routing::types::PhysicalQubitId;

// -----------------------------------------------------------------------------
// SwapMove
// -----------------------------------------------------------------------------

/// A semantic physical SWAP routing operation.
///
/// `SwapMove` represents an exchange of quantum states between two adjacent
/// physical locations. It is independent of the eventual hardware-level
/// decomposition of SWAP.
///
/// # Invariants
///
/// A valid `SwapMove` has:
///
/// - two distinct physical qubits;
/// - both physical qubits represented by the target topology;
/// - a physical connectivity edge between the qubits;
/// - no hardware-specific assumptions.
///
/// Construction is intentionally infallible because the move can be created
/// before a topology is available. Topology-dependent validation is performed
/// by [`SwapMove::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SwapMove {
    a: PhysicalQubitId,
    b: PhysicalQubitId,
}

impl SwapMove {
    /// Creates a semantic SWAP move.
    ///
    /// This constructor performs only local validation.
    ///
    /// It rejects:
    ///
    /// - `a == b`
    ///
    /// Topology-dependent validation must be performed with
    /// [`SwapMove::validate`].
    ///
    /// # Errors
    ///
    /// Returns [`RoutingError`] when the two endpoints are identical.
    pub fn new(
        a: PhysicalQubitId,
        b: PhysicalQubitId,
    ) -> Result<Self, RoutingError> {
        if a == b {
            return Err(RoutingError::InvalidMove {
                message: "SWAP endpoints must be distinct".to_string(),
            });
        }

        Ok(Self { a, b })
    }

    /// Returns the first physical endpoint.
    #[inline]
    pub const fn a(self) -> PhysicalQubitId {
        self.a
    }

    /// Returns the second physical endpoint.
    #[inline]
    pub const fn b(self) -> PhysicalQubitId {
        self.b
    }

    /// Returns both physical endpoints as an ordered pair.
    ///
    /// The ordering is stable and deterministic.
    #[inline]
    pub const fn endpoints(self) -> (PhysicalQubitId, PhysicalQubitId) {
        (self.a, self.b)
    }

    /// Returns the endpoints in canonical order.
    ///
    /// This is useful when:
    ///
    /// - deduplicating candidate SWAPs;
    /// - hashing routing decisions;
    /// - producing deterministic reports;
    /// - comparing candidate moves.
    #[inline]
    pub fn canonical(self) -> Self {
        if self.a <= self.b {
            self
        } else {
            Self {
                a: self.b,
                b: self.a,
            }
        }
    }

    /// Returns whether two SWAP moves represent the same physical exchange.
    ///
    /// Endpoint order does not matter because SWAP is symmetric.
    #[inline]
    pub fn equivalent(self, other: Self) -> bool {
        self.canonical() == other.canonical()
    }

    /// Validates this SWAP against a physical topology.
    ///
    /// Validation is intentionally stricter than merely checking that both
    /// physical qubits exist.
    ///
    /// A routing SWAP requires a direct physical connectivity edge.
    pub fn validate(
        &self,
        topology: &Topology,
    ) -> Result<(), RoutingError> {
        if self.a == self.b {
            return Err(RoutingError::InvalidMove {
                message: "SWAP endpoints must be distinct".to_string(),
            });
        }

        if !topology.contains(self.a) {
            return Err(RoutingError::InvalidPhysicalQubit {
                qubit: self.a,
            });
        }

        if !topology.contains(self.b) {
            return Err(RoutingError::InvalidPhysicalQubit {
                qubit: self.b,
            });
        }

        if !topology.is_adjacent(self.a, self.b) {
            return Err(RoutingError::UnsupportedMove {
                message: format!(
                    "SWAP between {} and {} is not supported by the target \
                     physical topology",
                    self.a, self.b
                ),
            });
        }

        Ok(())
    }

    /// Validates the move and the mapping before applying it.
    ///
    /// This is the preferred entry point for routing algorithms.
    ///
    /// Validation occurs completely before the mapping is changed.
    pub fn validate_mapping(
        &self,
        topology: &Topology,
        mapping: &QubitMapping,
    ) -> Result<(), RoutingError> {
        self.validate(topology)?;

        mapping.validate()?;

        // A routing SWAP exchanges physical locations. The mapping must know
        // about both physical locations before the operation is committed.
        //
        // This requirement prevents a router from accidentally treating an
        // unallocated physical location as though it contained a valid logical
        // quantum state.
        if mapping.logical_of(self.a).is_none() {
            return Err(RoutingError::InvalidMove {
                message: format!(
                    "cannot apply SWAP to unoccupied physical qubit {}",
                    self.a
                ),
            });
        }

        if mapping.logical_of(self.b).is_none() {
            return Err(RoutingError::InvalidMove {
                message: format!(
                    "cannot apply SWAP to unoccupied physical qubit {}",
                    self.b
                ),
            });
        }

        Ok(())
    }

    /// Applies the SWAP to a validated logical-to-physical mapping.
    ///
    /// The topology is validated first and the mapping is validated before
    /// mutation.
    ///
    /// This operation is atomic from the caller's perspective:
    ///
    /// - if validation fails, the mapping is unchanged;
    /// - the mapping mutation is delegated to `QubitMapping`;
    /// - this module never directly manipulates mapping internals.
    pub fn apply(
        &self,
        topology: &Topology,
        mapping: &mut QubitMapping,
    ) -> Result<(), RoutingError> {
        self.validate_mapping(topology, mapping)?;

        mapping.swap_physical(self.a, self.b)?;

        debug_assert!(
            mapping.validate().is_ok(),
            "QubitMapping invariant violated after SwapMove::apply"
        );

        Ok(())
    }

    /// Returns the inverse of this SWAP.
    ///
    /// SWAP is self-inverse, so this method returns the same semantic move.
    #[inline]
    pub const fn inverse(self) -> Self {
        self
    }

    /// Returns the number of physical movement operations represented by this
    /// move.
    ///
    /// This is always one and exists primarily to make cost/metrics code
    /// generic over movement primitives.
    #[inline]
    pub const fn operation_count(self) -> usize {
        1
    }

    /// Returns the physical distance represented by this move.
    ///
    /// A valid SWAP always connects adjacent physical qubits, therefore the
    /// graph distance is exactly one.
    #[inline]
    pub const fn physical_distance(self) -> usize {
        1
    }
}

// -----------------------------------------------------------------------------
// Ordering
// -----------------------------------------------------------------------------

impl Ord for SwapMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.canonical()
            .endpoints()
            .cmp(&other.canonical().endpoints())
    }
}

impl PartialOrd for SwapMove {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// -----------------------------------------------------------------------------
// Display
// -----------------------------------------------------------------------------

impl std::fmt::Display for SwapMove {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "SWAP({}, {})", self.a, self.b)
    }
}

// -----------------------------------------------------------------------------
// Conversion into the canonical routing operation
// -----------------------------------------------------------------------------

impl From<SwapMove> for crate::quantum::routing::types::RoutingOperation {
    fn from(value: SwapMove) -> Self {
        Self::Swap {
            a: value.a,
            b: value.b,
        }
    }
}

// -----------------------------------------------------------------------------
// Swap application result
// -----------------------------------------------------------------------------

/// Information returned after successfully applying a SWAP.
///
/// This structure is intentionally small and deterministic.
///
/// Routing algorithms can use it for metrics without having to inspect
/// `QubitMapping` internals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwapApplication {
    /// The physical locations exchanged.
    pub move_: SwapMove,

    /// Number of inserted semantic SWAP operations.
    pub operation_count: usize,
}

impl SwapApplication {
    /// Creates a successful SWAP application result.
    #[inline]
    pub const fn new(move_: SwapMove) -> Self {
        Self {
            move_,
            operation_count: 1,
        }
    }

    /// Returns the applied SWAP move.
    #[inline]
    pub const fn swap(self) -> SwapMove {
        self.move_
    }
}

// -----------------------------------------------------------------------------
// Convenience operation
// -----------------------------------------------------------------------------

/// Validates and applies a SWAP, returning an operation record on success.
///
/// This helper is intentionally thin. The authoritative semantics remain on
/// [`SwapMove`].
pub fn apply_swap(
    swap: SwapMove,
    topology: &Topology,
    mapping: &mut QubitMapping,
) -> Result<SwapApplication, RoutingError> {
    swap.apply(topology, mapping)?;

    Ok(SwapApplication::new(swap))
}

// -----------------------------------------------------------------------------
// Candidate construction helpers
// -----------------------------------------------------------------------------

/// Constructs a validated SWAP from two physical qubits.
///
/// This helper is useful to routing algorithms that generate candidate moves.
///
/// It deliberately does not apply the move.
pub fn candidate(
    a: PhysicalQubitId,
    b: PhysicalQubitId,
) -> Result<SwapMove, RoutingError> {
    SwapMove::new(a, b)
}

/// Constructs a SWAP only when the target topology permits it.
///
/// This is useful when generating candidate moves directly from a topology.
pub fn candidate_if_adjacent(
    topology: &Topology,
    a: PhysicalQubitId,
    b: PhysicalQubitId,
) -> Result<Option<SwapMove>, RoutingError> {
    if a == b {
        return Ok(None);
    }

    if !topology.contains(a) {
        return Err(RoutingError::InvalidPhysicalQubit {
            qubit: a,
        });
    }

    if !topology.contains(b) {
        return Err(RoutingError::InvalidPhysicalQubit {
            qubit: b,
        });
    }

    if !topology.is_adjacent(a, b) {
        return Ok(None);
    }

    Ok(Some(SwapMove::new(a, b)?))
}

// -----------------------------------------------------------------------------
// Unit tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn line_topology() -> Topology {
        Topology::line(3).expect("three-qubit line topology must be valid")
    }

    fn mapped_three_qubits(
        topology: &Topology,
    ) -> QubitMapping {
        let mut mapping = QubitMapping::new();

        mapping
            .assign(
                crate::quantum::routing::types::LogicalQubitId::new(0),
                PhysicalQubitId::new(0),
            )
            .expect("q0 -> p0 assignment must succeed");

        mapping
            .assign(
                crate::quantum::routing::types::LogicalQubitId::new(1),
                PhysicalQubitId::new(1),
            )
            .expect("q1 -> p1 assignment must succeed");

        mapping
            .assign(
                crate::quantum::routing::types::LogicalQubitId::new(2),
                PhysicalQubitId::new(2),
            )
            .expect("q2 -> p2 assignment must succeed");

        mapping
            .validate_against(topology)
            .expect("test mapping must be valid");

        mapping
    }

    #[test]
    fn rejects_self_swap() {
        let result = SwapMove::new(
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(1),
        );

        assert!(result.is_err());
    }

    #[test]
    fn accepts_distinct_endpoints() {
        let swap = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        )
        .expect("distinct endpoints must be accepted");

        assert_eq!(
            swap.endpoints(),
            (
                PhysicalQubitId::new(0),
                PhysicalQubitId::new(1)
            )
        );
    }

    #[test]
    fn canonicalizes_endpoint_order() {
        let forward = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        )
        .expect("valid swap");

        let reverse = SwapMove::new(
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(0),
        )
        .expect("valid swap");

        assert_eq!(forward.canonical(), reverse.canonical());
        assert!(forward.equivalent(reverse));
    }

    #[test]
    fn swap_is_self_inverse() {
        let swap = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        )
        .expect("valid swap");

        assert_eq!(swap.inverse(), swap);
    }

    #[test]
    fn adjacent_swap_validates() {
        let topology = line_topology();

        let swap = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        )
        .expect("valid swap");

        assert!(swap.validate(&topology).is_ok());
    }

    #[test]
    fn non_adjacent_swap_is_rejected() {
        let topology = line_topology();

        let swap = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(2),
        )
        .expect("construction itself does not require topology");

        assert!(swap.validate(&topology).is_err());
    }

    #[test]
    fn unknown_physical_qubit_is_rejected() {
        let topology = line_topology();

        let swap = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(99),
        )
        .expect("construction itself does not require topology");

        assert!(swap.validate(&topology).is_err());
    }

    #[test]
    fn candidate_if_adjacent_returns_move() {
        let topology = line_topology();

        let candidate = candidate_if_adjacent(
            &topology,
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        )
        .expect("candidate construction must succeed");

        assert!(candidate.is_some());
    }

    #[test]
    fn candidate_if_adjacent_rejects_non_edge_without_error() {
        let topology = line_topology();

        let candidate = candidate_if_adjacent(
            &topology,
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(2),
        )
        .expect("non-edge candidate should be a normal negative result");

        assert!(candidate.is_none());
    }

    #[test]
    fn application_swaps_mapping_positions() {
        let topology = line_topology();
        let mut mapping = mapped_three_qubits(&topology);

        let swap = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        )
        .expect("valid swap");

        swap.apply(&topology, &mut mapping)
            .expect("valid swap must apply");

        assert_eq!(
            mapping.physical_of(
                crate::quantum::routing::types::LogicalQubitId::new(0)
            ),
            Some(PhysicalQubitId::new(1))
        );

        assert_eq!(
            mapping.physical_of(
                crate::quantum::routing::types::LogicalQubitId::new(1)
            ),
            Some(PhysicalQubitId::new(0))
        );

        assert_eq!(
            mapping.logical_of(PhysicalQubitId::new(0)),
            Some(crate::quantum::routing::types::LogicalQubitId::new(1))
        );

        assert_eq!(
            mapping.logical_of(PhysicalQubitId::new(1)),
            Some(crate::quantum::routing::types::LogicalQubitId::new(0))
        );
    }

    #[test]
    fn applying_same_swap_twice_restores_mapping() {
        let topology = line_topology();
        let mut mapping = mapped_three_qubits(&topology);

        let before = mapping.clone();

        let swap = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        )
        .expect("valid swap");

        swap.apply(&topology, &mut mapping)
            .expect("first swap must apply");

        swap.apply(&topology, &mut mapping)
            .expect("second swap must apply");

        assert_eq!(mapping, before);
    }

    #[test]
    fn failed_non_adjacent_swap_does_not_mutate_mapping() {
        let topology = line_topology();
        let mut mapping = mapped_three_qubits(&topology);
        let before = mapping.clone();

        let swap = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(2),
        )
        .expect("construction itself is valid");

        assert!(swap.apply(&topology, &mut mapping).is_err());
        assert_eq!(mapping, before);
    }

    #[test]
    fn failed_unoccupied_swap_does_not_mutate_mapping() {
        let topology = line_topology();

        let mut mapping = QubitMapping::new();

        mapping
            .assign(
                crate::quantum::routing::types::LogicalQubitId::new(0),
                PhysicalQubitId::new(0),
            )
            .expect("assignment must succeed");

        let before = mapping.clone();

        let swap = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        )
        .expect("valid swap");

        assert!(swap.apply(&topology, &mut mapping).is_err());
        assert_eq!(mapping, before);
    }

    #[test]
    fn display_is_deterministic() {
        let swap = SwapMove::new(
            PhysicalQubitId::new(2),
            PhysicalQubitId::new(5),
        )
        .expect("valid swap");

        assert_eq!(swap.to_string(), "SWAP(p2, p5)");
    }

    #[test]
    fn operation_count_is_one() {
        let swap = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        )
        .expect("valid swap");

        assert_eq!(swap.operation_count(), 1);
    }

    #[test]
    fn physical_distance_is_one() {
        let swap = SwapMove::new(
            PhysicalQubitId::new(0),
            PhysicalQubitId::new(1),
        )
        .expect("valid swap");

        assert_eq!(swap.physical_distance(), 1);
    }

    #[test]
    fn ordering_is_canonical() {
        let a = SwapMove::new(
            PhysicalQubitId::new(2),
            PhysicalQubitId::new(5),
        )
        .expect("valid swap");

        let b = SwapMove::new(
            PhysicalQubitId::new(5),
            PhysicalQubitId::new(2),
        )
        .expect("valid swap");

        assert_eq!(a, b.canonical());
    }

    #[test]
    fn conversion_to_routing_operation_preserves_endpoints() {
        let swap = SwapMove::new(
            PhysicalQubitId::new(2),
            PhysicalQubitId::new(5),
        )
        .expect("valid swap");

        let operation =
            crate::quantum::routing::types::RoutingOperation::from(swap);

        match operation {
            crate::quantum::routing::types::RoutingOperation::Swap {
                a,
                b,
            } => {
                assert_eq!(a, PhysicalQubitId::new(2));
                assert_eq!(b, PhysicalQubitId::new(5));
            }

            _ => panic!("SwapMove must convert to RoutingOperation::Swap"),
        }
    }
}