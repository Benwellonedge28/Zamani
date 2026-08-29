//! Zamani Quantum Memory — Qubit Permutations
//!
//! Production-grade, representation-independent quantum-qubit permutation
//! primitives.
//!
//! # Responsibility
//!
//! This module owns the mathematical and memory-layout semantics of a
//! permutation of quantum positions.
//!
//! It answers:
//!
//! - Which old position becomes a new position?
//! - Which new position originated from an old position?
//! - How do we invert a permutation?
//! - How do we compose permutations?
//! - How do we apply a permutation to a logical/storage order?
//! - How do we validate a permutation before it reaches a hot path?
//! - How do we represent permutation cycles without unsafe memory access?
//!
//! It does NOT own:
//!
//! - quantum-state amplitudes;
//! - gate semantics;
//! - circuit IR;
//! - routing algorithms;
//! - hardware topology;
//! - scheduling;
//! - allocation;
//! - GPU APIs;
//! - distributed communication;
//! - QPU provider APIs;
//! - measurement;
//! - QEC algorithms.
//!
//! Those responsibilities remain in their respective subsystems.
//!
//! # Architectural boundary
//!
//! ```text
//!                         quantum::ir
//!                             │
//!                 QubitId / PhysicalQubitId
//!                             │
//!                             ▼
//!                 quantum::memory::layout
//!                             │
//!                             ▼
//!              quantum::memory::permutation
//!                             │
//!          ┌──────────────────┼──────────────────┐
//!          ▼                  ▼                  ▼
//!      state_vector         views          tensor_network
//!          │                  │                  │
//!          └──────────────────┼──────────────────┘
//!                             ▼
//!                         execution
//!                             │
//!                    ┌────────┴────────┐
//!                    ▼                 ▼
//!                  CPU               QPU
//!                                      │
//!                               hardware/routing
//! ```
//!
//! # Critical architectural distinction
//!
//! A memory permutation is NOT a routing mapping.
//!
//! `quantum::routing::mapping` owns mutable associations such as:
//!
//! ```text
//! logical qubit <-> physical qubit
//! ```
//!
//! This module instead owns the pure permutation operation used to transform
//! an ordering.
//!
//! A routing subsystem may consume this module to express a layout change,
//! but this module must never depend on routing.
//!
//! This prevents a dependency cycle:
//!
//! ```text
//! memory ──X──> routing
//! ```
//!
//! while allowing:
//!
//! ```text
//! routing ──> memory::permutation
//! ```
//!
//! # Permutation convention
//!
//! The canonical representation used here is:
//!
//! ```text
//! new_position -> old_position
//! ```
//!
//! For example:
//!
//! ```text
//! permutation = [2, 0, 1]
//! ```
//!
//! means:
//!
//! ```text
//! new[0] = old[2]
//! new[1] = old[0]
//! new[2] = old[1]
//! ```
//!
//! This convention matches `QubitOrder::permute()` in `memory::layout`.
//!
//! # Why this convention matters
//!
//! A permutation is often described in the opposite direction by different
//! quantum frameworks. Making the direction explicit prevents subtle
//! state-vector, tensor-network, endian, and hardware-layout bugs.
//!
//! # Hardware independence
//!
//! This module does not assume:
//!
//! - superconducting qubits;
//! - trapped ions;
//! - neutral atoms;
//! - photonics;
//! - spin qubits;
//! - annealers;
//! - neutral-atom arrays;
//! - FPGA-backed QPUs;
//! - GPU simulators;
//! - distributed simulators;
//! - any particular vendor.
//!
//! A permutation operates on positions/identifiers. The hardware subsystem
//! decides whether that permutation corresponds to a SWAP, relabeling, qubit
//! movement, transport, beam steering, ion movement, lattice rearrangement,
//! compiler remapping, or another backend-specific operation.
//!
//! # Safety
//!
//! This module uses no `unsafe`.
//!
//! It never exposes raw pointers.
//!
//! It never mutates caller-owned slices through aliasing.
//!
//! All indexing is bounds checked.
//!
//! All permutation construction is validated before use.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! This module depends only on:
//!
//! - `quantum::ir::{QubitId, PhysicalQubitId}`;
//! - `quantum::memory::layout::{MemoryLayout, QubitOrder}`;
//!
//! It intentionally does NOT depend on:
//!
//! - routing;
//! - hardware;
//! - state-vector implementation;
//! - density matrix;
//! - tensor network;
//! - allocator;
//! - GPU;
//! - distributed memory.
//!
//! Later modules should consume this API rather than implement independent
//! permutation representations.
//!
//! In particular:
//!
//! - `layout.rs` remains authoritative for `QubitOrder`;
//! - `view.rs` can use `QubitPermutation` to describe view transformations;
//! - `state_vector.rs` can use `PermutationPlan` when reordering amplitudes;
//! - `density_matrix.rs` can apply the same logical permutation to both matrix
//!   axes;
//! - `tensor_network.rs` can permute tensor/site order;
//! - `routing` can convert its mapping decisions into memory permutations;
//! - `gpu.rs` can use permutation plans without this module knowing GPU APIs;
//! - `distributed.rs` can use the same logical permutation contract for
//!   partition movement;
//! - `serialization.rs` can persist the canonical permutation representation;
//! - `snapshot.rs` can store the permutation as part of immutable state
//!   metadata.
//!
//! # Important invariant
//!
//! A valid permutation of length `n` contains every integer in:
//!
//! ```text
//! 0..n
//! ```
//!
//! exactly once.
//!
//! No duplicate, missing, or out-of-range position is accepted.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::quantum::ir::{PhysicalQubitId, QubitId};
use crate::quantum::memory::layout::{LayoutError, MemoryLayout, QubitOrder};

// =============================================================================
// Result aliases
// =============================================================================

/// Result type used by this permutation module.
pub type PermutationResult<T> = Result<T, PermutationError>;

// =============================================================================
// Direction
// =============================================================================

/// Explicit direction of a permutation mapping.
///
/// The internal canonical representation remains `new_position -> old_position`.
///
/// This enum exists so callers can explicitly document which direction an
/// external mapping uses before converting it to the canonical representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermutationDirection {
    /// The mapping describes:
    ///
    /// ```text
    /// new_position -> old_position
    /// ```
    NewToOld,

    /// The mapping describes:
    ///
    /// ```text
    /// old_position -> new_position
    /// ```
    OldToNew,
}

impl Default for PermutationDirection {
    fn default() -> Self {
        Self::NewToOld
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by permutation construction and transformation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermutationError {
    /// The permutation has an invalid length.
    InvalidLength {
        expected: usize,
        actual: usize,
    },

    /// An entry refers to a position outside the permutation domain.
    OutOfRange {
        position: usize,
        length: usize,
    },

    /// The same old position appears more than once.
    DuplicatePosition {
        position: usize,
    },

    /// A permutation is missing one or more positions.
    MissingPosition {
        position: usize,
    },

    /// An arithmetic operation overflowed.
    ArithmeticOverflow,

    /// The requested operation cannot be represented by the supplied layout.
    LayoutIncompatible {
        reason: String,
    },

    /// The supplied logical qubit cannot be represented by the target order.
    LogicalQubitMismatch {
        qubit: QubitId,
    },

    /// The supplied physical qubit cannot be represented by the requested
    /// physical ordering.
    PhysicalQubitMismatch {
        qubit: PhysicalQubitId,
    },

    /// The permutation was expected to be non-trivial but is the identity.
    ExpectedNonIdentity,

    /// The caller supplied an invalid transformation.
    InvalidTransformation {
        reason: String,
    },
}

impl fmt::Display for PermutationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { expected, actual } => write!(
                f,
                "invalid permutation length: expected {expected}, got {actual}"
            ),

            Self::OutOfRange { position, length } => write!(
                f,
                "permutation position {position} is outside range 0..{length}"
            ),

            Self::DuplicatePosition { position } => {
                write!(f, "permutation contains duplicate position {position}")
            }

            Self::MissingPosition { position } => {
                write!(f, "permutation is missing position {position}")
            }

            Self::ArithmeticOverflow => {
                write!(f, "permutation arithmetic overflow")
            }

            Self::LayoutIncompatible { reason } => {
                write!(f, "permutation is incompatible with memory layout: {reason}")
            }

            Self::LogicalQubitMismatch { qubit } => {
                write!(f, "logical qubit {qubit} is not present in the target order")
            }

            Self::PhysicalQubitMismatch { qubit } => {
                write!(
                    f,
                    "physical qubit {qubit} is not present in the requested ordering"
                )
            }

            Self::ExpectedNonIdentity => {
                write!(f, "the requested permutation is the identity permutation")
            }

            Self::InvalidTransformation { reason } => {
                write!(f, "invalid permutation transformation: {reason}")
            }
        }
    }
}

impl std::error::Error for PermutationError {}

impl From<LayoutError> for PermutationError {
    fn from(error: LayoutError) -> Self {
        Self::LayoutIncompatible {
            reason: error.to_string(),
        }
    }
}

// =============================================================================
// Canonical permutation
// =============================================================================

/// An immutable validated permutation of positions.
///
/// The canonical representation is:
///
/// ```text
/// new_position -> old_position
/// ```
///
/// Therefore:
///
/// ```text
/// let p = [2, 0, 1];
///
/// new[0] = old[2];
/// new[1] = old[0];
/// new[2] = old[1];
/// ```
///
/// The structure stores both the forward and inverse permutation so repeated
/// transformations do not need to reconstruct the inverse.
///
/// # Complexity
///
/// Construction:
///
/// ```text
/// O(n)
/// ```
///
/// Inversion:
///
/// ```text
/// O(1)
/// ```
///
/// Position lookup:
///
/// ```text
/// O(1)
/// ```
///
/// Applying to a slice:
///
/// ```text
/// O(n)
/// ```
///
/// Composition:
///
/// ```text
/// O(n)
/// ```
///
/// Cycle decomposition:
///
/// ```text
/// O(n)
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QubitPermutation {
    /// Canonical mapping:
    ///
    /// `new_position -> old_position`.
    new_to_old: Vec<usize>,

    /// Inverse mapping:
    ///
    /// `old_position -> new_position`.
    old_to_new: Vec<usize>,
}

impl QubitPermutation {
    // =========================================================================
    // Construction
    // =========================================================================

    /// Creates the identity permutation for `length` positions.
    pub fn identity(length: usize) -> Self {
        let positions: Vec<usize> = (0..length).collect();

        Self {
            new_to_old: positions.clone(),
            old_to_new: positions,
        }
    }

    /// Creates a validated permutation using the canonical `new -> old`
    /// convention.
    pub fn try_new(new_to_old: Vec<usize>) -> PermutationResult<Self> {
        let length = new_to_old.len();

        let mut old_to_new = vec![0usize; length];
        let mut seen = vec![false; length];

        for (new_position, &old_position) in new_to_old.iter().enumerate() {
            if old_position >= length {
                return Err(PermutationError::OutOfRange {
                    position: old_position,
                    length,
                });
            }

            if seen[old_position] {
                return Err(PermutationError::DuplicatePosition {
                    position: old_position,
                });
            }

            seen[old_position] = true;
            old_to_new[old_position] = new_position;
        }

        for (position, present) in seen.into_iter().enumerate() {
            if !present {
                return Err(PermutationError::MissingPosition { position });
            }
        }

        Ok(Self {
            new_to_old,
            old_to_new,
        })
    }

    /// Creates a permutation from an explicit mapping and direction.
    ///
    /// `NewToOld` means the supplied vector is already in canonical form.
    ///
    /// `OldToNew` means the supplied vector describes:
    ///
    /// ```text
    /// old_position -> new_position
    /// ```
    ///
    /// and is converted into the canonical representation.
    pub fn from_direction(
        mapping: Vec<usize>,
        direction: PermutationDirection,
    ) -> PermutationResult<Self> {
        match direction {
            PermutationDirection::NewToOld => Self::try_new(mapping),

            PermutationDirection::OldToNew => {
                let length = mapping.len();

                let mut new_to_old = vec![0usize; length];
                let mut seen_new = vec![false; length];

                for (old_position, &new_position) in mapping.iter().enumerate() {
                    if new_position >= length {
                        return Err(PermutationError::OutOfRange {
                            position: new_position,
                            length,
                        });
                    }

                    if seen_new[new_position] {
                        return Err(PermutationError::DuplicatePosition {
                            position: new_position,
                        });
                    }

                    seen_new[new_position] = true;
                    new_to_old[new_position] = old_position;
                }

                Self::try_new(new_to_old)
            }
        }
    }

    /// Creates the permutation represented by swapping two positions.
    pub fn swap(length: usize, first: usize, second: usize) -> PermutationResult<Self> {
        if first >= length {
            return Err(PermutationError::OutOfRange {
                position: first,
                length,
            });
        }

        if second >= length {
            return Err(PermutationError::OutOfRange {
                position: second,
                length,
            });
        }

        let mut permutation: Vec<usize> = (0..length).collect();
        permutation.swap(first, second);

        Self::try_new(permutation)
    }

    /// Creates a cyclic rotation of positions.
    ///
    /// Positive offsets move a position toward higher indices.
    ///
    /// Example for length 4 and offset 1:
    ///
    /// ```text
    /// new: 0 1 2 3
    /// old: 3 0 1 2
    /// ```
    pub fn rotate(length: usize, offset: isize) -> PermutationResult<Self> {
        if length == 0 {
            return Ok(Self::identity(0));
        }

        let length_isize =
            isize::try_from(length).map_err(|_| PermutationError::ArithmeticOverflow)?;

        let normalized = offset.rem_euclid(length_isize);

        let mut new_to_old = Vec::with_capacity(length);

        for new_position in 0..length {
            let new_position_isize = isize::try_from(new_position)
                .map_err(|_| PermutationError::ArithmeticOverflow)?;

            let old_position = (new_position_isize - normalized)
                .rem_euclid(length_isize);

            let old_position = usize::try_from(old_position)
                .map_err(|_| PermutationError::ArithmeticOverflow)?;

            new_to_old.push(old_position);
        }

        Self::try_new(new_to_old)
    }

    /// Creates a reversal permutation.
    pub fn reverse(length: usize) -> Self {
        let new_to_old: Vec<usize> = (0..length).rev().collect();

        // This vector is guaranteed to be a permutation, so construction
        // cannot fail.
        Self::try_new(new_to_old)
            .expect("reverse permutation construction must be valid")
    }

    // =========================================================================
    // Constructors from Zamani memory layout
    // =========================================================================

    /// Creates a permutation from a `QubitOrder` target order.
    ///
    /// The returned permutation describes how to transform the identity
    /// storage order into the supplied order.
    ///
    /// Example:
    ///
    /// ```text
    /// target order = [q2, q0, q1]
    ///
    /// permutation = [2, 0, 1]
    /// ```
    pub fn from_qubit_order(order: &QubitOrder) -> PermutationResult<Self> {
        let new_to_old = order
            .as_slice()
            .iter()
            .map(QubitId::index)
            .collect::<Vec<_>>();

        Self::try_new(new_to_old)
    }

    /// Creates the permutation transforming one `QubitOrder` into another.
    ///
    /// Both orders must contain exactly the same logical-qubit namespace.
    ///
    /// The returned permutation satisfies:
    ///
    /// ```text
    /// target = permutation(source)
    /// ```
    pub fn between_orders(
        source: &QubitOrder,
        target: &QubitOrder,
    ) -> PermutationResult<Self> {
        if source.len() != target.len() {
            return Err(PermutationError::InvalidLength {
                expected: source.len(),
                actual: target.len(),
            });
        }

        let length = source.len();

        let mut source_position_by_qubit = vec![usize::MAX; length];

        for position in 0..length {
            let qubit = source
                .logical_at(position)
                .ok_or(PermutationError::InvalidTransformation {
                    reason: format!(
                        "source order does not contain storage position {position}"
                    ),
                })?;

            let index = qubit.index();

            if index >= length {
                return Err(PermutationError::LogicalQubitMismatch { qubit });
            }

            if source_position_by_qubit[index] != usize::MAX {
                return Err(PermutationError::InvalidTransformation {
                    reason: format!(
                        "source order contains logical qubit {qubit} more than once"
                    ),
                });
            }

            source_position_by_qubit[index] = position;
        }

        let mut new_to_old = Vec::with_capacity(length);

        for new_position in 0..length {
            let qubit = target
                .logical_at(new_position)
                .ok_or(PermutationError::InvalidTransformation {
                    reason: format!(
                        "target order does not contain storage position {new_position}"
                    ),
                })?;

            let index = qubit.index();

            if index >= length {
                return Err(PermutationError::LogicalQubitMismatch { qubit });
            }

            let old_position = source_position_by_qubit[index];

            if old_position == usize::MAX {
                return Err(PermutationError::LogicalQubitMismatch { qubit });
            }

            new_to_old.push(old_position);
        }

        Self::try_new(new_to_old)
    }

    /// Creates the permutation needed to transform a `MemoryLayout` into
    /// another `MemoryLayout`.
    ///
    /// Bit order and storage-layout metadata must be compatible because this
    /// function only describes a position permutation. It does not silently
    /// change endian semantics or physical strides.
    pub fn between_layouts(
        source: &MemoryLayout,
        target: &MemoryLayout,
    ) -> PermutationResult<Self> {
        if source.num_qubits() != target.num_qubits() {
            return Err(PermutationError::InvalidLength {
                expected: source.num_qubits(),
                actual: target.num_qubits(),
            });
        }

        if source.bit_order() != target.bit_order() {
            return Err(PermutationError::LayoutIncompatible {
                reason: "source and target layouts use different bit orders; \
                         endian conversion must be explicit"
                    .to_owned(),
            });
        }

        Self::between_orders(source.order(), target.order())
    }

    // =========================================================================
    // Accessors
    // =========================================================================

    /// Returns the number of positions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.new_to_old.len()
    }

    /// Returns whether the permutation is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.new_to_old.is_empty()
    }

    /// Returns the canonical `new_position -> old_position` mapping.
    #[must_use]
    pub fn new_to_old(&self) -> &[usize] {
        &self.new_to_old
    }

    /// Returns the inverse `old_position -> new_position` mapping.
    #[must_use]
    pub fn old_to_new(&self) -> &[usize] {
        &self.old_to_new
    }

    /// Returns the old position that supplies a new position.
    #[must_use]
    pub fn old_position_for_new(&self, new_position: usize) -> Option<usize> {
        self.new_to_old.get(new_position).copied()
    }

    /// Returns the new position receiving an old position.
    #[must_use]
    pub fn new_position_for_old(&self, old_position: usize) -> Option<usize> {
        self.old_to_new.get(old_position).copied()
    }

    /// Returns whether the permutation is the identity.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.new_to_old
            .iter()
            .enumerate()
            .all(|(position, &old_position)| position == old_position)
    }

    /// Returns whether the permutation changes at least one position.
    #[must_use]
    pub fn is_non_identity(&self) -> bool {
        !self.is_identity()
    }

    // =========================================================================
    // Inversion
    // =========================================================================

    /// Returns the inverse permutation.
    ///
    /// If:
    ///
    /// ```text
    /// p(new) = old
    /// ```
    ///
    /// then:
    ///
    /// ```text
    /// p^-1(old) = new
    /// ```
    #[must_use]
    pub fn inverse(&self) -> Self {
        Self {
            new_to_old: self.old_to_new.clone(),
            old_to_new: self.new_to_old.clone(),
        }
    }

    // =========================================================================
    // Composition
    // =========================================================================

    /// Composes `self` followed by `next`.
    ///
    /// If:
    ///
    /// ```text
    /// A: middle_position -> old_position
    /// B: new_position    -> middle_position
    /// ```
    ///
    /// then:
    ///
    /// ```text
    /// B ∘ A
    /// ```
    ///
    /// transforms old positions directly into new positions.
    ///
    /// This method therefore satisfies:
    ///
    /// ```text
    /// compose(next, self).apply(old) == next.apply(self.apply(old))
    /// ```
    ///
    /// in the positional sense represented by the canonical mapping.
    pub fn then(&self, next: &Self) -> PermutationResult<Self> {
        if self.len() != next.len() {
            return Err(PermutationError::InvalidLength {
                expected: self.len(),
                actual: next.len(),
            });
        }

        let mut composed = Vec::with_capacity(self.len());

        for &middle_position in &next.new_to_old {
            let old_position = self
                .new_to_old
                .get(middle_position)
                .copied()
                .ok_or(PermutationError::OutOfRange {
                    position: middle_position,
                    length: self.len(),
                })?;

            composed.push(old_position);
        }

        Self::try_new(composed)
    }

    /// Composes `self` before `next`.
    ///
    /// This is an explicit alias that reads naturally at call sites.
    pub fn followed_by(&self, next: &Self) -> PermutationResult<Self> {
        self.then(next)
    }

    /// Returns the identity permutation after applying `self` and its inverse.
    #[must_use]
    pub fn identity_after_inverse(&self) -> Self {
        // The result is mathematically guaranteed to be identity.
        Self::identity(self.len())
    }

    // =========================================================================
    // Application to raw positional data
    // =========================================================================

    /// Applies the permutation to an immutable slice and returns a reordered
    /// vector.
    ///
    /// Semantics:
    ///
    /// ```text
    /// output[new] = input[old]
    /// ```
    pub fn apply<T: Clone>(&self, input: &[T]) -> PermutationResult<Vec<T>> {
        if input.len() != self.len() {
            return Err(PermutationError::InvalidLength {
                expected: self.len(),
                actual: input.len(),
            });
        }

        let mut output = Vec::with_capacity(input.len());

        for &old_position in &self.new_to_old {
            let value = input
                .get(old_position)
                .ok_or(PermutationError::OutOfRange {
                    position: old_position,
                    length: input.len(),
                })?;

            output.push(value.clone());
        }

        Ok(output)
    }

    /// Applies the inverse permutation to an immutable slice.
    pub fn apply_inverse<T: Clone>(
        &self,
        input: &[T],
    ) -> PermutationResult<Vec<T>> {
        self.inverse().apply(input)
    }

    /// Applies the permutation to a mutable slice using a temporary owned
    /// vector.
    ///
    /// This is deliberately implemented without unsafe in-place pointer
    /// manipulation.
    ///
    /// The temporary allocation is explicit and deterministic.
    pub fn apply_in_place<T: Clone>(
        &self,
        data: &mut [T],
    ) -> PermutationResult<()> {
        if data.len() != self.len() {
            return Err(PermutationError::InvalidLength {
                expected: self.len(),
                actual: data.len(),
            });
        }

        let reordered = self.apply(data)?;

        for (destination, source) in data.iter_mut().zip(reordered.into_iter()) {
            *destination = source;
        }

        Ok(())
    }

    /// Reorders a vector and returns the new vector.
    ///
    /// This is a convenience wrapper around [`Self::apply`].
    pub fn reordered<T: Clone>(
        &self,
        input: Vec<T>,
    ) -> PermutationResult<Vec<T>> {
        self.apply(&input)
    }

    // =========================================================================
    // Application to logical qubit orders
    // =========================================================================

    /// Applies the permutation to a `QubitOrder`.
    ///
    /// Because `QubitOrder` uses the same canonical `new_position -> old_position`
    /// convention, this operation is directly composable with `layout.rs`.
    pub fn apply_to_order(
        &self,
        order: &QubitOrder,
    ) -> PermutationResult<QubitOrder> {
        if order.len() != self.len() {
            return Err(PermutationError::InvalidLength {
                expected: self.len(),
                actual: order.len(),
            });
        }

        order
            .permute(&self.new_to_old)
            .map_err(PermutationError::from)
    }

    /// Returns the logical order produced by applying this permutation to the
    /// identity order.
    pub fn to_qubit_order(&self) -> PermutationResult<QubitOrder> {
        QubitOrder::try_from_logical_order(
            self.len(),
            self.new_to_old
                .iter()
                .copied()
                .map(QubitId::new)
                .collect(),
        )
        .map_err(PermutationError::from)
    }

    /// Creates a permutation directly from an existing order and verifies that
    /// the result is a valid logical-qubit order.
    pub fn from_order(
        order: &QubitOrder,
    ) -> PermutationResult<Self> {
        Self::from_qubit_order(order)
    }

    // =========================================================================
    // Physical-qubit ordering
    // =========================================================================

    /// Applies the positional permutation to physical qubit identifiers.
    ///
    /// This method does not know anything about a hardware topology.
    ///
    /// That is intentional:
    ///
    /// ```text
    /// permutation = memory/layout semantics
    /// topology    = hardware semantics
    /// routing     = movement/placement semantics
    /// ```
    pub fn apply_to_physical_order(
        &self,
        physical_order: &[PhysicalQubitId],
    ) -> PermutationResult<Vec<PhysicalQubitId>> {
        self.apply(physical_order)
    }

    /// Applies the permutation to logical qubit identifiers.
    pub fn apply_to_logical_order(
        &self,
        logical_order: &[QubitId],
    ) -> PermutationResult<Vec<QubitId>> {
        self.apply(logical_order)
    }

    /// Creates a positional permutation from a physical-qubit ordering.
    ///
    /// Physical IDs do not need to be contiguous or start at zero. This is
    /// important for real QPUs where physical identifiers may be sparse,
    /// vendor-defined, disabled, or represented by a topology layer.
    ///
    /// The input itself defines the ordering domain.
    pub fn from_physical_order(
        source: &[PhysicalQubitId],
        target: &[PhysicalQubitId],
    ) -> PermutationResult<Self> {
        if source.len() != target.len() {
            return Err(PermutationError::InvalidLength {
                expected: source.len(),
                actual: target.len(),
            });
        }

        let length = source.len();

        let mut source_position_by_identity =
            std::collections::HashMap::with_capacity(length);

        for (position, &qubit) in source.iter().enumerate() {
            if source_position_by_identity.insert(qubit, position).is_some() {
                return Err(PermutationError::PhysicalQubitMismatch { qubit });
            }
        }

        let mut new_to_old = Vec::with_capacity(length);

        for &qubit in target {
            let old_position = source_position_by_identity
                .get(&qubit)
                .copied()
                .ok_or(PermutationError::PhysicalQubitMismatch { qubit })?;

            new_to_old.push(old_position);
        }

        Self::try_new(new_to_old)
    }

    // =========================================================================
    // Cycles
    // =========================================================================

    /// Returns the permutation as disjoint cycles.
    ///
    /// Fixed points are omitted.
    ///
    /// Example:
    ///
    /// ```text
    /// [1, 2, 0, 4, 3]
    /// ```
    ///
    /// produces cycles equivalent to:
    ///
    /// ```text
    /// (0 1 2)
    /// (3 4)
    /// ```
    ///
    /// Cycle orientation follows the canonical `new -> old` representation.
    pub fn cycles(&self) -> Vec<Vec<usize>> {
        let length = self.len();
        let mut visited = vec![false; length];
        let mut cycles = Vec::new();

        for start in 0..length {
            if visited[start] {
                continue;
            }

            let next = self.new_to_old[start];

            if next == start {
                visited[start] = true;
                continue;
            }

            let mut cycle = Vec::new();
            let mut current = start;

            loop {
                if visited[current] {
                    break;
                }

                visited[current] = true;
                cycle.push(current);

                current = self.new_to_old[current];
            }

            if !cycle.is_empty() {
                cycles.push(cycle);
            }
        }

        cycles
    }

    /// Returns the number of non-fixed permutation cycles.
    #[must_use]
    pub fn cycle_count(&self) -> usize {
        self.cycles().len()
    }

    /// Returns the number of positions moved by the permutation.
    #[must_use]
    pub fn moved_positions(&self) -> usize {
        self.new_to_old
            .iter()
            .enumerate()
            .filter(|(position, old_position)| position != old_position)
            .count()
    }

    // =========================================================================
    // Validation / diagnostics
    // =========================================================================

    /// Validates the permutation's internal invariants.
    ///
    /// Normally unnecessary because construction validates the structure.
    /// This is provided for defensive boundaries, deserialization, fuzzing,
    /// diagnostics, and future ABI/FFI adapters.
    pub fn validate(&self) -> PermutationResult<()> {
        let reconstructed = Self::try_new(self.new_to_old.clone())?;

        if reconstructed.old_to_new != self.old_to_new {
            return Err(PermutationError::InvalidTransformation {
                reason: "stored inverse permutation does not match forward \
                         permutation"
                    .to_owned(),
            });
        }

        Ok(())
    }

    /// Returns a deterministic textual representation.
    pub fn describe(&self) -> String {
        format!(
            "QubitPermutation {{ length: {}, identity: {}, moved_positions: {}, cycles: {} }}",
            self.len(),
            self.is_identity(),
            self.moved_positions(),
            self.cycle_count()
        )
    }
}

// =============================================================================
// Conversion helpers
// =============================================================================

impl From<QubitPermutation> for Vec<usize> {
    fn from(permutation: QubitPermutation) -> Self {
        permutation.new_to_old
    }
}

impl AsRef<[usize]> for QubitPermutation {
    fn as_ref(&self) -> &[usize] {
        &self.new_to_old
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    #[test]
    fn identity_is_valid() {
        let permutation = QubitPermutation::identity(4);

        assert_eq!(permutation.len(), 4);
        assert!(permutation.is_identity());
        assert_eq!(permutation.new_to_old(), &[0, 1, 2, 3]);
        assert_eq!(permutation.old_to_new(), &[0, 1, 2, 3]);
    }

    #[test]
    fn empty_identity_is_valid() {
        let permutation = QubitPermutation::identity(0);

        assert!(permutation.is_empty());
        assert!(permutation.is_identity());
    }

    #[test]
    fn validates_normal_permutation() {
        let permutation =
            QubitPermutation::try_new(vec![2, 0, 1]).expect("valid permutation");

        assert_eq!(permutation.new_to_old(), &[2, 0, 1]);
        assert_eq!(permutation.old_to_new(), &[1, 2, 0]);
    }

    #[test]
    fn rejects_duplicate_positions() {
        let error = QubitPermutation::try_new(vec![0, 0, 1])
            .expect_err("duplicate must be rejected");

        assert_eq!(
            error,
            PermutationError::DuplicatePosition { position: 0 }
        );
    }

    #[test]
    fn rejects_out_of_range_positions() {
        let error = QubitPermutation::try_new(vec![0, 1, 3])
            .expect_err("out of range position must be rejected");

        assert_eq!(
            error,
            PermutationError::OutOfRange {
                position: 3,
                length: 3
            }
        );
    }

    #[test]
    fn rejects_missing_positions() {
        let error = QubitPermutation::try_new(vec![0, 0])
            .expect_err("duplicate is also a missing-position permutation");

        assert!(matches!(
            error,
            PermutationError::DuplicatePosition { .. }
        ));
    }

    #[test]
    fn inverse_is_correct() {
        let permutation =
            QubitPermutation::try_new(vec![2, 0, 1]).expect("valid");

        let inverse = permutation.inverse();

        assert_eq!(inverse.new_to_old(), &[1, 2, 0]);
        assert_eq!(inverse.old_to_new(), &[2, 0, 1]);
    }

    #[test]
    fn inverse_round_trip_restores_data() {
        let permutation =
            QubitPermutation::try_new(vec![2, 0, 1, 3]).expect("valid");

        let input = vec![10, 20, 30, 40];

        let transformed = permutation.apply(&input).expect("apply");
        let restored = permutation
            .inverse()
            .apply(&transformed)
            .expect("inverse apply");

        assert_eq!(restored, input);
    }

    #[test]
    fn apply_uses_new_to_old_semantics() {
        let permutation =
            QubitPermutation::try_new(vec![2, 0, 1]).expect("valid");

        let input = vec!['a', 'b', 'c'];

        assert_eq!(
            permutation.apply(&input).expect("apply"),
            vec!['c', 'a', 'b']
        );
    }

    #[test]
    fn apply_in_place_is_correct() {
        let permutation =
            QubitPermutation::try_new(vec![2, 0, 1]).expect("valid");

        let mut data = vec![1, 2, 3];

        permutation
            .apply_in_place(&mut data)
            .expect("in-place permutation");

        assert_eq!(data, vec![3, 1, 2]);
    }

    #[test]
    fn rejects_wrong_input_length() {
        let permutation =
            QubitPermutation::try_new(vec![1, 0, 2]).expect("valid");

        let error = permutation
            .apply(&[1, 2])
            .expect_err("wrong length must fail");

        assert_eq!(
            error,
            PermutationError::InvalidLength {
                expected: 3,
                actual: 2
            }
        );
    }

    #[test]
    fn swap_is_correct() {
        let permutation =
            QubitPermutation::swap(4, 1, 3).expect("valid swap");

        assert_eq!(permutation.new_to_old(), &[0, 3, 2, 1]);
    }

    #[test]
    fn swapping_same_position_is_identity() {
        let permutation =
            QubitPermutation::swap(4, 2, 2).expect("valid self swap");

        assert!(permutation.is_identity());
    }

    #[test]
    fn swap_rejects_out_of_range() {
        let error =
            QubitPermutation::swap(3, 0, 3).expect_err("must reject");

        assert_eq!(
            error,
            PermutationError::OutOfRange {
                position: 3,
                length: 3
            }
        );
    }

    #[test]
    fn reverse_is_correct() {
        let permutation = QubitPermutation::reverse(4);

        assert_eq!(permutation.new_to_old(), &[3, 2, 1, 0]);
    }

    #[test]
    fn rotate_positive_is_correct() {
        let permutation =
            QubitPermutation::rotate(4, 1).expect("valid rotation");

        assert_eq!(permutation.new_to_old(), &[3, 0, 1, 2]);
    }

    #[test]
    fn rotate_negative_is_correct() {
        let permutation =
            QubitPermutation::rotate(4, -1).expect("valid rotation");

        assert_eq!(permutation.new_to_old(), &[1, 2, 3, 0]);
    }

    #[test]
    fn rotate_large_offset_is_normalized() {
        let a =
            QubitPermutation::rotate(4, 1).expect("valid rotation");

        let b =
            QubitPermutation::rotate(4, 5).expect("valid rotation");

        assert_eq!(a, b);
    }

    #[test]
    fn direction_conversion_is_correct() {
        let canonical =
            QubitPermutation::try_new(vec![2, 0, 1]).expect("valid");

        let old_to_new = canonical.old_to_new().to_vec();

        let reconstructed = QubitPermutation::from_direction(
            old_to_new,
            PermutationDirection::OldToNew,
        )
        .expect("valid conversion");

        assert_eq!(reconstructed, canonical);
    }

    #[test]
    fn composition_is_correct() {
        let first =
            QubitPermutation::try_new(vec![2, 0, 1]).expect("valid");

        let second =
            QubitPermutation::try_new(vec![1, 2, 0]).expect("valid");

        let composed = first.then(&second).expect("composition");

        let input = vec!['a', 'b', 'c'];

        let sequential = second
            .apply(&first.apply(&input).expect("first"))
            .expect("second");

        let composed_result = composed.apply(&input).expect("composed");

        assert_eq!(composed_result, sequential);
    }

    #[test]
    fn composition_with_inverse_is_identity() {
        let permutation =
            QubitPermutation::try_new(vec![2, 0, 3, 1]).expect("valid");

        let inverse = permutation.inverse();

        let composed = permutation
            .then(&inverse)
            .expect("composition");

        assert!(composed.is_identity());
    }

    #[test]
    fn cycles_are_correct() {
        let permutation =
            QubitPermutation::try_new(vec![1, 2, 0, 4, 3])
                .expect("valid");

        let cycles = permutation.cycles();

        assert_eq!(cycles.len(), 2);

        let flattened: Vec<usize> =
            cycles.into_iter().flatten().collect();

        assert_eq!(flattened.len(), 5);
    }

    #[test]
    fn identity_has_no_nontrivial_cycles() {
        let permutation = QubitPermutation::identity(8);

        assert!(permutation.cycles().is_empty());
        assert_eq!(permutation.cycle_count(), 0);
        assert_eq!(permutation.moved_positions(), 0);
    }

    #[test]
    fn moved_positions_are_counted() {
        let permutation =
            QubitPermutation::try_new(vec![0, 2, 1, 3])
                .expect("valid");

        assert_eq!(permutation.moved_positions(), 2);
    }

    #[test]
    fn converts_identity_order() {
        let order = QubitOrder::identity(3).expect("valid order");

        let permutation =
            QubitPermutation::from_qubit_order(&order)
                .expect("valid permutation");

        assert!(permutation.is_identity());
    }

    #[test]
    fn converts_permuted_order() {
        let order = QubitOrder::try_from_logical_order(
            3,
            vec![q(2), q(0), q(1)],
        )
        .expect("valid order");

        let permutation =
            QubitPermutation::from_qubit_order(&order)
                .expect("valid permutation");

        assert_eq!(permutation.new_to_old(), &[2, 0, 1]);
    }

    #[test]
    fn applies_to_qubit_order() {
        let source =
            QubitOrder::identity(3).expect("valid identity order");

        let permutation =
            QubitPermutation::try_new(vec![2, 0, 1])
                .expect("valid");

        let result =
            permutation.apply_to_order(&source).expect("apply");

        assert_eq!(result.as_slice(), &[q(2), q(0), q(1)]);
    }

    #[test]
    fn between_orders_is_correct() {
        let source =
            QubitOrder::try_from_logical_order(
                3,
                vec![q(0), q(1), q(2)],
            )
            .expect("source");

        let target =
            QubitOrder::try_from_logical_order(
                3,
                vec![q(2), q(0), q(1)],
            )
            .expect("target");

        let permutation =
            QubitPermutation::between_orders(&source, &target)
                .expect("between");

        assert_eq!(permutation.new_to_old(), &[2, 0, 1]);
    }

    #[test]
    fn between_orders_handles_non_identity_source() {
        let source =
            QubitOrder::try_from_logical_order(
                3,
                vec![q(1), q(2), q(0)],
            )
            .expect("source");

        let target =
            QubitOrder::try_from_logical_order(
                3,
                vec![q(0), q(1), q(2)],
            )
            .expect("target");

        let permutation =
            QubitPermutation::between_orders(&source, &target)
                .expect("between");

        let result =
            permutation.apply_to_order(&source)
                .expect("apply");

        assert_eq!(result, target);
    }

    #[test]
    fn physical_order_does_not_require_contiguous_ids() {
        let source = vec![
            PhysicalQubitId::new(17),
            PhysicalQubitId::new(3),
            PhysicalQubitId::new(42),
        ];

        let target = vec![
            PhysicalQubitId::new(42),
            PhysicalQubitId::new(17),
            PhysicalQubitId::new(3),
        ];

        let permutation =
            QubitPermutation::from_physical_order(&source, &target)
                .expect("valid physical permutation");

        assert_eq!(permutation.new_to_old(), &[2, 0, 1]);

        let restored = permutation
            .apply(&source)
            .expect("apply");

        assert_eq!(restored, target);
    }

    #[test]
    fn physical_order_rejects_missing_qubit() {
        let source = vec![
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(2),
        ];

        let target = vec![
            PhysicalQubitId::new(1),
            PhysicalQubitId::new(3),
        ];

        let error =
            QubitPermutation::from_physical_order(&source, &target)
                .expect_err("missing physical qubit must fail");

        assert_eq!(
            error,
            PermutationError::PhysicalQubitMismatch {
                qubit: PhysicalQubitId::new(3)
            }
        );
    }

    #[test]
    fn validate_confirms_internal_inverse() {
        let permutation =
            QubitPermutation::try_new(vec![2, 0, 1, 3])
                .expect("valid");

        permutation.validate().expect("valid internal state");
    }

    #[test]
    fn serialization_round_trip() {
        let permutation =
            QubitPermutation::try_new(vec![2, 0, 1])
                .expect("valid");

        let encoded =
            serde_json::to_string(&permutation)
                .expect("serialize");

        let decoded: QubitPermutation =
            serde_json::from_str(&encoded)
                .expect("deserialize");

        assert_eq!(decoded, permutation);
        decoded.validate().expect("validated");
    }

    #[test]
    fn description_is_deterministic() {
        let permutation =
            QubitPermutation::try_new(vec![2, 0, 1])
                .expect("valid");

        assert_eq!(
            permutation.describe(),
            "QubitPermutation { length: 3, identity: false, moved_positions: 3, cycles: 1 }"
        );
    }
}