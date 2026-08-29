//! Zamani Quantum Memory — Logical Quantum Register
//!
//! Production-grade logical quantum-register abstraction.
//!
//! # Architectural responsibility
//!
//! `quantum::memory::register` owns the relationship between a memory-domain
//! register and its canonical logical `quantum::ir::QubitId` members.
//!
//! It is responsible for:
//!
//! - defining a deterministic logical-qubit collection;
//! - validating register membership;
//! - validating operand collections;
//! - preserving logical-qubit identity;
//! - providing stable logical-position lookup;
//! - providing safe register slicing by logical position;
//! - providing deterministic iteration;
//! - providing register-level resource metadata;
//! - preventing duplicate logical-qubit membership;
//! - providing a provider-neutral boundary for simulators and QPUs.
//!
//! It does NOT own:
//!
//! - physical-qubit allocation;
//! - logical-to-physical routing;
//! - hardware topology;
//! - QPU communication;
//! - gate semantics;
//! - quantum amplitudes;
//! - density matrices;
//! - stabilizer tableaux;
//! - tensor-network state;
//! - measurement collapse;
//! - quantum-state lifetime transitions;
//! - memory allocation implementation;
//! - GPU buffers;
//! - distributed communication;
//! - scheduling;
//! - optimization;
//! - benchmarking.
//!
//! Those responsibilities belong to:
//!
//! ```text
//! quantum::ir
//! quantum::memory::allocator
//! quantum::memory::lifetime
//! quantum::memory::layout
//! quantum::memory::permutation
//! quantum::memory::state
//! quantum::hardware
//! quantum::routing
//! quantum::scheduling
//! quantum::benchmarking
//! ```
//!
//! # Canonical identity rule
//!
//! The canonical logical qubit identity is:
//!
//! ```text
//! quantum::ir::QubitId
//! ```
//!
//! The canonical physical-qubit identity is:
//!
//! ```text
//! quantum::ir::PhysicalQubitId
//! ```
//!
//! This module intentionally does not redefine either type.
//!
//! A memory register therefore remains valid regardless of whether execution
//! eventually occurs on:
//!
//! - a state-vector simulator;
//! - a density-matrix simulator;
//! - a stabilizer simulator;
//! - a tensor-network simulator;
//! - a sparse simulator;
//! - a CPU backend;
//! - a SIMD backend;
//! - a GPU backend;
//! - a distributed simulator;
//! - a superconducting QPU;
//! - a trapped-ion QPU;
//! - a neutral-atom QPU;
//! - a photonic QPU;
//! - a spin/qubit QPU;
//! - an annealing backend;
//! - a remote provider;
//! - a future hardware architecture.
//!
//! # Critical architectural invariant
//!
//! A `QuantumRegister` describes *which logical qubits belong together*.
//!
//! It does not decide where those qubits physically execute.
//!
//! The intended flow is:
//!
//! ```text
//!                 Quantum IR
//!                     │
//!                     │ canonical QubitId
//!                     ▼
//!             QuantumRegister
//!                     │
//!          ┌──────────┴──────────┐
//!          │                     │
//!          ▼                     ▼
//!      Memory State          Routing
//!                                │
//!                                ▼
//!                         PhysicalQubitId
//!                                │
//!                                ▼
//!                            Hardware
//! ```
//!
//! Consequently, routing may change the physical placement without requiring
//! this register to change its logical membership.
//!
//! # Immutability of membership
//!
//! Once constructed, the logical membership of a `QuantumRegister` does not
//! change.
//!
//! This is intentional.
//!
//! Allocation/deallocation and lifetime belong to other memory layers.
//!
//! If a program needs a different logical collection, construct another
//! register or a register view/slice.
//!
//! This prevents a state representation from observing a register whose
//! logical meaning changes underneath it.
//!
//! # Safety
//!
//! This module:
//!
//! - contains no `unsafe`;
//! - exposes no raw pointers;
//! - exposes no mutable slice of register membership;
//! - performs checked arithmetic where required;
//! - performs no hidden hardware access;
//! - performs no allocation before configured cardinality validation;
//! - never uses `unwrap()` or `expect()` in production paths;
//! - never prints to stdout/stderr.
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
//! The following modules may consume this module:
//!
//! ```text
//! lifetime.rs
//! state.rs
//! view.rs
//! slice.rs
//! permutation.rs
//! measurement.rs
//! reset.rs
//! serialization.rs
//! snapshot.rs
//! checkpoint.rs
//! diagnostics.rs
//! telemetry.rs
//! routing/
//! scheduling/
//! hardware/
//! benchmarking/
//! ```
//!
//! None of those modules should create a competing logical-register type.
//!
//! In particular:
//!
//! - `state.rs` consumes the register's logical qubit set;
//! - `layout.rs` maps that logical set into storage positions;
//! - `permutation.rs` transforms ordering without changing logical identity;
//! - `routing` maps logical identities to physical identities;
//! - `hardware` consumes physical placement and backend capabilities;
//! - `lifetime.rs` owns allocation/lifetime transitions;
//! - `measurement.rs` consumes validated register subsets;
//! - `serialization.rs` persists logical IDs and register metadata;
//! - `diagnostics.rs` and `telemetry.rs` consume read-only register metadata.
//!
//! The register itself remains independent of all of them.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use std::collections::HashSet;
use std::fmt;

use crate::quantum::ir::QubitId;
use crate::quantum::memory::errors::MemoryError;
use crate::quantum::memory::types::{MemoryId, QubitCount};

// =============================================================================
// Constants
// =============================================================================

/// Stable schema identifier for logical quantum registers.
pub const QUANTUM_REGISTER_SCHEMA_ID: &str = "zamani.quantum.memory.register";

/// Semantic version of the register contract.
///
/// Increment this when the externally observable register contract changes
/// incompatibly.
pub const QUANTUM_REGISTER_SCHEMA_VERSION: u16 = 1;

/// Default diagnostic resource name used by this module.
const RESOURCE_NAME: &str = "quantum_register";

// =============================================================================
// Result
// =============================================================================

/// Result type used by the logical-register subsystem.
pub type RegisterResult<T> = Result<T, MemoryError>;

// =============================================================================
// Register errors
// =============================================================================

/// Creates the canonical memory error used for logical-register failures.
///
/// `errors.rs` deliberately owns the global error taxonomy. This helper keeps
/// all register failures inside that contract without introducing a second
/// public error enum.
fn logical_error(reason: impl Into<String>) -> MemoryError {
    MemoryError::LogicalMemoryError {
        reason: reason.into(),
    }
}

/// Creates a canonical invalid-argument error.
///
/// This is used when an argument is structurally invalid rather than when a
/// register-membership invariant itself has failed.
fn invalid_argument(argument: impl Into<String>) -> MemoryError {
    MemoryError::InvalidArgument {
        argument: argument.into(),
        context: None,
    }
}

// =============================================================================
// Register metadata
// =============================================================================

/// Immutable metadata describing a logical quantum register.
///
/// This type deliberately contains no hardware-specific information.
///
/// It can therefore be safely used by:
///
/// - compilers;
/// - simulators;
/// - QPU adapters;
/// - routing;
/// - scheduling;
/// - persistence;
/// - diagnostics;
/// - benchmarking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuantumRegisterMetadata {
    /// Memory-domain identity associated with the register.
    memory_id: MemoryId,

    /// Number of logical qubits.
    qubit_count: QubitCount,
}

impl QuantumRegisterMetadata {
    /// Creates register metadata.
    pub const fn new(memory_id: MemoryId, qubit_count: QubitCount) -> Self {
        Self {
            memory_id,
            qubit_count,
        }
    }

    /// Returns the memory-domain identity.
    pub const fn memory_id(self) -> MemoryId {
        self.memory_id
    }

    /// Returns the number of logical qubits.
    pub const fn qubit_count(self) -> QubitCount {
        self.qubit_count
    }

    /// Returns the number of logical qubits as `usize`.
    pub const fn len(self) -> usize {
        self.qubit_count.get()
    }

    /// Returns whether the register is empty.
    pub const fn is_empty(self) -> bool {
        self.qubit_count.is_zero()
    }
}

impl fmt::Display for QuantumRegisterMetadata {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            self.memory_id,
            self.qubit_count
        )
    }
}

// =============================================================================
// Quantum register
// =============================================================================

/// Immutable logical quantum register.
///
/// A `QuantumRegister` is an ordered collection of canonical logical
/// `QubitId`s.
///
/// The order is significant for memory consumers that need deterministic
/// positional semantics, but it is *not* a physical-hardware mapping.
///
/// Example:
///
/// ```text
/// [q3, q7, q2]
/// ```
///
/// means that this register contains exactly those three logical qubits in
/// that declared logical order.
///
/// It does NOT mean:
///
/// ```text
/// q3 -> physical 0
/// q7 -> physical 1
/// q2 -> physical 2
/// ```
///
/// Physical placement remains the responsibility of routing/backend layers.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct QuantumRegister {
    metadata: QuantumRegisterMetadata,
    qubits: Vec<QubitId>,
}

impl QuantumRegister {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates an empty logical register.
    ///
    /// Empty registers are valid because some compiler and dynamic-circuit
    /// workflows construct register structures before allocation.
    pub fn empty(memory_id: MemoryId) -> Self {
        Self {
            metadata: QuantumRegisterMetadata::new(
                memory_id,
                QubitCount::ZERO,
            ),
            qubits: Vec::new(),
        }
    }

    /// Creates a register containing contiguous logical qubits:
    ///
    /// ```text
    /// q0, q1, q2, ..., q(n-1)
    /// ```
    ///
    /// `maximum` is an explicit safety limit.
    ///
    /// No register allocation occurs when `count > maximum`.
    pub fn try_contiguous(
        memory_id: MemoryId,
        count: QubitCount,
        maximum: QubitCount,
    ) -> RegisterResult<Self> {
        let count_value = count.get();
        let maximum_value = maximum.get();

        if count_value > maximum_value {
            return Err(logical_error(format!(
                "register contains {count_value} logical qubits, \
                 exceeding configured maximum {maximum_value}"
            )));
        }

        // `Vec<QubitId>` is a thin collection of strongly typed IDs.
        //
        // The checked upper bound prevents pathological requests from reaching
        // the allocation step. The actual memory allocator/state allocator
        // remains responsible for quantum-state storage limits.
        let mut qubits = Vec::with_capacity(count_value);

        for index in 0..count_value {
            qubits.push(QubitId::new(index));
        }

        Self::from_owned_qubits_with_validated_limit(
            memory_id,
            qubits,
            maximum_value,
        )
    }

    /// Creates a register from an owned logical-qubit collection.
    ///
    /// The collection:
    ///
    /// - may contain arbitrary canonical logical IDs;
    /// - must not contain duplicates;
    /// - must not exceed `maximum`;
    /// - retains the supplied deterministic order.
    ///
    /// This constructor is the primary integration point for Quantum IR.
    ///
    /// It is also suitable for dynamic circuits where the logical IDs are
    /// generated by a compiler rather than being contiguous.
    pub fn try_from_qubits(
        memory_id: MemoryId,
        qubits: Vec<QubitId>,
        maximum: QubitCount,
    ) -> RegisterResult<Self> {
        Self::from_owned_qubits_with_validated_limit(
            memory_id,
            qubits,
            maximum.get(),
        )
    }

    /// Creates a register from a borrowed logical-qubit slice.
    ///
    /// This method copies only the logical identifiers. It does not copy any
    /// quantum state.
    pub fn try_from_slice(
        memory_id: MemoryId,
        qubits: &[QubitId],
        maximum: QubitCount,
    ) -> RegisterResult<Self> {
        if qubits.len() > maximum.get() {
            return Err(logical_error(format!(
                "register contains {} logical qubits, \
                 exceeding configured maximum {}",
                qubits.len(),
                maximum.get()
            )));
        }

        let owned = qubits.to_vec();

        Self::from_owned_qubits_with_validated_limit(
            memory_id,
            owned,
            maximum.get(),
        )
    }

    /// Internal constructor used after cardinality validation.
    fn from_owned_qubits_with_validated_limit(
        memory_id: MemoryId,
        qubits: Vec<QubitId>,
        maximum: usize,
    ) -> RegisterResult<Self> {
        if qubits.len() > maximum {
            return Err(logical_error(format!(
                "register contains {} logical qubits, \
                 exceeding configured maximum {maximum}",
                qubits.len()
            )));
        }

        validate_unique_qubits(&qubits)?;

        let count = QubitCount::new(qubits.len());

        Ok(Self {
            metadata: QuantumRegisterMetadata::new(memory_id, count),
            qubits,
        })
    }

    // -------------------------------------------------------------------------
    // Metadata
    // -------------------------------------------------------------------------

    /// Returns immutable register metadata.
    pub const fn metadata(&self) -> QuantumRegisterMetadata {
        self.metadata
    }

    /// Returns the memory-domain identity.
    pub const fn memory_id(&self) -> MemoryId {
        self.metadata.memory_id()
    }

    /// Returns the number of logical qubits.
    pub const fn len(&self) -> usize {
        self.metadata.len()
    }

    /// Returns whether the register contains no logical qubits.
    pub const fn is_empty(&self) -> bool {
        self.metadata.is_empty()
    }

    /// Returns the strongly typed qubit count.
    pub const fn qubit_count(&self) -> QubitCount {
        self.metadata.qubit_count()
    }

    // -------------------------------------------------------------------------
    // Logical membership
    // -------------------------------------------------------------------------

    /// Returns whether a logical qubit belongs to this register.
    ///
    /// This operation is representation-independent and has no hardware
    /// implications.
    pub fn contains(&self, qubit: QubitId) -> bool {
        self.qubits.contains(&qubit)
    }

    /// Returns the logical qubit at a register position.
    ///
    /// Position is the register's declared logical ordering, not a physical
    /// hardware position.
    pub fn get(&self, position: usize) -> RegisterResult<QubitId> {
        self.qubits
            .get(position)
            .copied()
            .ok_or_else(|| MemoryError::OutOfBounds {
                index: position as u64,
                length: self.qubits.len() as u64,
                resource: RESOURCE_NAME.to_owned(),
            })
    }

    /// Returns a logical qubit at a register position without constructing an
    /// error.
    pub fn get_opt(&self, position: usize) -> Option<QubitId> {
        self.qubits.get(position).copied()
    }

    /// Returns the register position of a logical qubit.
    ///
    /// The returned position is a logical-register position. It is not a
    /// physical qubit identifier and must never be interpreted as one.
    pub fn position_of(&self, qubit: QubitId) -> RegisterResult<usize> {
        self.qubits
            .iter()
            .position(|candidate| *candidate == qubit)
            .ok_or_else(|| logical_error(format!(
                "logical qubit {qubit} is not a member of register {}",
                self.memory_id()
            )))
    }

    /// Returns the first logical qubit, if any.
    pub fn first(&self) -> Option<QubitId> {
        self.qubits.first().copied()
    }

    /// Returns the last logical qubit, if any.
    pub fn last(&self) -> Option<QubitId> {
        self.qubits.last().copied()
    }

    /// Returns an immutable slice of canonical logical qubit IDs.
    ///
    /// The caller cannot mutate register membership through this API.
    pub fn as_slice(&self) -> &[QubitId] {
        &self.qubits
    }

    /// Returns a deterministic iterator over logical qubits.
    pub fn iter(&self) -> std::slice::Iter<'_, QubitId> {
        self.qubits.iter()
    }

    // -------------------------------------------------------------------------
    // Operand validation
    // -------------------------------------------------------------------------

    /// Validates that one logical qubit belongs to this register.
    pub fn validate_qubit(&self, qubit: QubitId) -> RegisterResult<()> {
        if self.contains(qubit) {
            Ok(())
        } else {
            Err(logical_error(format!(
                "logical qubit {qubit} does not belong to register {}",
                self.memory_id()
            )))
        }
    }

    /// Validates that all supplied logical qubits belong to this register.
    ///
    /// Duplicates are rejected because an operand collection such as:
    ///
    /// ```text
    /// [q0, q0]
    /// ```
    ///
    /// is not equivalent to:
    ///
    /// ```text
    /// [q0, q1]
    /// ```
    ///
    /// This is especially important for gate application, measurement,
    /// reset, routing metadata, QEC syndrome operations, and backend lowering.
    pub fn validate_operands(
        &self,
        operands: &[QubitId],
    ) -> RegisterResult<()> {
        validate_unique_qubits(operands)?;

        for &qubit in operands {
            self.validate_qubit(qubit)?;
        }

        Ok(())
    }

    /// Validates that the supplied operands are a non-empty subset of the
    /// register.
    ///
    /// This is useful for operations such as measurement, reset, partial
    /// tracing, tensor contraction, and QEC operations.
    pub fn validate_non_empty_operands(
        &self,
        operands: &[QubitId],
    ) -> RegisterResult<()> {
        if operands.is_empty() {
            return Err(invalid_argument(
                "operands must contain at least one logical qubit",
            ));
        }

        self.validate_operands(operands)
    }

    /// Validates that the register contains every supplied logical qubit and
    /// that the supplied collection has exactly the same cardinality as the
    /// register.
    ///
    /// This does not require the same order.
    pub fn validate_full_membership(
        &self,
        qubits: &[QubitId],
    ) -> RegisterResult<()> {
        if qubits.len() != self.len() {
            return Err(logical_error(format!(
                "full register membership requires {} logical qubits, \
                 received {}",
                self.len(),
                qubits.len()
            )));
        }

        self.validate_operands(qubits)?;

        for &qubit in &self.qubits {
            if !qubits.contains(&qubit) {
                return Err(logical_error(format!(
                    "logical qubit {qubit} is missing from full-register membership"
                )));
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Sub-register creation
    // -------------------------------------------------------------------------

    /// Creates a new register containing selected logical qubits by their
    /// positions in this register.
    ///
    /// This operation creates a new logical namespace view/container. It does
    /// not copy quantum-state amplitudes or any other representation data.
    ///
    /// The resulting register retains the same `MemoryId` because the logical
    /// subset originates from this memory domain. State/view layers decide
    /// whether the subset is an alias, projection, or independent state.
    pub fn subset_by_positions(
        &self,
        positions: &[usize],
    ) -> RegisterResult<Self> {
        if positions.is_empty() {
            return Ok(Self::empty(self.memory_id()));
        }

        let mut selected = Vec::with_capacity(positions.len());

        for &position in positions {
            let qubit = self.get(position)?;
            selected.push(qubit);
        }

        Self::try_from_qubits(
            self.memory_id(),
            selected,
            self.qubit_count(),
        )
    }

    /// Creates a new register from explicitly supplied logical members.
    ///
    /// This is useful when a caller already has canonical `QubitId`s and wants
    /// to construct a logically related register without depending on the
    /// source register's ordering.
    pub fn subset_by_qubits(
        &self,
        qubits: &[QubitId],
    ) -> RegisterResult<Self> {
        self.validate_operands(qubits)?;

        Self::try_from_slice(
            self.memory_id(),
            qubits,
            self.qubit_count(),
        )
    }

    // -------------------------------------------------------------------------
    // Equality and deterministic identity
    // -------------------------------------------------------------------------

    /// Returns whether two registers contain the same logical members in the
    /// same order.
    ///
    /// This is equivalent to `PartialEq`, but is exposed explicitly because
    /// register ordering is semantically relevant to memory consumers.
    pub fn same_ordered_membership(
        &self,
        other: &Self,
    ) -> bool {
        self.qubits == other.qubits
    }

    /// Returns whether two registers contain the same logical members
    /// regardless of ordering.
    ///
    /// This method does not mutate either register.
    pub fn same_membership_unordered(
        &self,
        other: &Self,
    ) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let other_members: HashSet<QubitId> =
            other.qubits.iter().copied().collect();

        self.qubits
            .iter()
            .all(|qubit| other_members.contains(qubit))
    }

    // -------------------------------------------------------------------------
    // Register validation
    // -------------------------------------------------------------------------

    /// Performs a complete structural validation of this register.
    ///
    /// This is intentionally inexpensive relative to quantum-state validation.
    /// It verifies:
    ///
    /// - metadata cardinality;
    /// - non-duplicated logical IDs;
    /// - deterministic membership consistency.
    pub fn validate(&self) -> RegisterResult<()> {
        if self.metadata.qubit_count().get() != self.qubits.len() {
            return Err(logical_error(
                "register metadata cardinality does not match logical membership",
            ));
        }

        validate_unique_qubits(&self.qubits)?;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Interoperability helpers
    // -------------------------------------------------------------------------

    /// Returns the canonical logical IDs needed by a routing layer.
    ///
    /// This method intentionally returns only logical IDs. A routing subsystem
    /// must perform the logical-to-physical mapping using its own topology and
    /// policy.
    pub fn logical_qubits(&self) -> &[QubitId] {
        self.as_slice()
    }

    /// Returns the logical qubit IDs in deterministic order as an owned vector.
    ///
    /// This is useful at FFI, backend, serialization, and asynchronous task
    /// boundaries where ownership of the ID collection is required.
    ///
    /// No quantum state is copied.
    pub fn to_qubit_vec(&self) -> Vec<QubitId> {
        self.qubits.clone()
    }
}

impl fmt::Debug for QuantumRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("QuantumRegister")
            .field("memory_id", &self.memory_id())
            .field("qubit_count", &self.qubit_count())
            .field("qubits", &self.qubits)
            .finish()
    }
}

impl fmt::Display for QuantumRegister {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}[",
            self.memory_id()
        )?;

        for (index, qubit) in self.qubits.iter().enumerate() {
            if index != 0 {
                formatter.write_str(", ")?;
            }

            write!(formatter, "{qubit}")?;
        }

        formatter.write_str("]")
    }
}

impl<'a> IntoIterator for &'a QuantumRegister {
    type Item = &'a QubitId;
    type IntoIter = std::slice::Iter<'a, QubitId>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates that a collection contains no duplicate logical qubit IDs.
///
/// This helper is public because `state`, `measurement`, `reset`, `QEC`, and
/// routing-adjacent memory code frequently need to validate logical operands
/// without constructing a full register first.
///
/// The canonical identity type remains `quantum::ir::QubitId`.
pub fn validate_unique_qubits(
    qubits: &[QubitId],
) -> RegisterResult<()> {
    let mut seen = HashSet::with_capacity(qubits.len());

    for &qubit in qubits {
        if !seen.insert(qubit) {
            return Err(logical_error(format!(
                "logical qubit {qubit} occurs more than once"
            )));
        }
    }

    Ok(())
}

/// Validates that every logical qubit in `qubits` belongs to the supplied
/// register.
///
/// This is a convenience wrapper for callers that already have a register
/// and want a free function rather than a method call.
pub fn validate_register_membership(
    register: &QuantumRegister,
    qubits: &[QubitId],
) -> RegisterResult<()> {
    register.validate_operands(qubits)
}

/// Validates that a requested register cardinality does not exceed the
/// configured maximum.
///
/// This function performs no allocation.
pub fn validate_register_count(
    count: QubitCount,
    maximum: QubitCount,
) -> RegisterResult<()> {
    if count.get() > maximum.get() {
        return Err(logical_error(format!(
            "logical register cardinality {} exceeds configured maximum {}",
            count.get(),
            maximum.get()
        )));
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn memory_id() -> MemoryId {
        MemoryId::new(1)
    }

    fn max_qubits() -> QubitCount {
        QubitCount::new(1024)
    }

    #[test]
    fn empty_register_is_valid() {
        let register = QuantumRegister::empty(memory_id());

        assert!(register.is_empty());
        assert_eq!(register.len(), 0);
        assert_eq!(register.qubit_count(), QubitCount::ZERO);
        assert!(register.validate().is_ok());
    }

    #[test]
    fn contiguous_register_has_canonical_logical_ids() {
        let register = QuantumRegister::try_contiguous(
            memory_id(),
            QubitCount::new(4),
            max_qubits(),
        )
        .expect("bounded register construction should succeed");

        assert_eq!(register.len(), 4);
        assert_eq!(
            register.as_slice(),
            &[
                QubitId::new(0),
                QubitId::new(1),
                QubitId::new(2),
                QubitId::new(3),
            ]
        );
    }

    #[test]
    fn register_rejects_excessive_cardinality_before_allocation() {
        let result = QuantumRegister::try_contiguous(
            memory_id(),
            QubitCount::new(5),
            QubitCount::new(4),
        );

        assert!(result.is_err());
    }

    #[test]
    fn register_accepts_non_contiguous_logical_ids() {
        let register = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(17),
                QubitId::new(3),
                QubitId::new(91),
            ],
            max_qubits(),
        )
        .expect("unique logical IDs should be accepted");

        assert_eq!(register.get(0).unwrap(), QubitId::new(17));
        assert_eq!(register.get(1).unwrap(), QubitId::new(3));
        assert_eq!(register.get(2).unwrap(), QubitId::new(91));
    }

    #[test]
    fn register_rejects_duplicate_logical_ids() {
        let result = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(0),
                QubitId::new(1),
                QubitId::new(0),
            ],
            max_qubits(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn contains_uses_logical_identity_not_position() {
        let register = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(7),
                QubitId::new(2),
                QubitId::new(9),
            ],
            max_qubits(),
        )
        .unwrap();

        assert!(register.contains(QubitId::new(7)));
        assert!(register.contains(QubitId::new(2)));
        assert!(register.contains(QubitId::new(9)));
        assert!(!register.contains(QubitId::new(0)));
    }

    #[test]
    fn position_lookup_is_deterministic() {
        let register = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(7),
                QubitId::new(2),
                QubitId::new(9),
            ],
            max_qubits(),
        )
        .unwrap();

        assert_eq!(
            register.position_of(QubitId::new(7)).unwrap(),
            0
        );

        assert_eq!(
            register.position_of(QubitId::new(2)).unwrap(),
            1
        );

        assert_eq!(
            register.position_of(QubitId::new(9)).unwrap(),
            2
        );
    }

    #[test]
    fn get_rejects_out_of_bounds_positions() {
        let register = QuantumRegister::try_contiguous(
            memory_id(),
            QubitCount::new(2),
            max_qubits(),
        )
        .unwrap();

        assert!(register.get(2).is_err());
    }

    #[test]
    fn operand_validation_rejects_duplicates() {
        let register = QuantumRegister::try_contiguous(
            memory_id(),
            QubitCount::new(4),
            max_qubits(),
        )
        .unwrap();

        let result = register.validate_operands(&[
            QubitId::new(0),
            QubitId::new(0),
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn operand_validation_rejects_non_members() {
        let register = QuantumRegister::try_contiguous(
            memory_id(),
            QubitCount::new(4),
            max_qubits(),
        )
        .unwrap();

        let result = register.validate_operands(&[
            QubitId::new(0),
            QubitId::new(9),
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn operand_validation_accepts_unique_members() {
        let register = QuantumRegister::try_contiguous(
            memory_id(),
            QubitCount::new(4),
            max_qubits(),
        )
        .unwrap();

        assert!(
            register
                .validate_operands(&[
                    QubitId::new(0),
                    QubitId::new(2),
                ])
                .is_ok()
        );
    }

    #[test]
    fn non_empty_operand_validation_rejects_empty_input() {
        let register = QuantumRegister::empty(memory_id());

        assert!(
            register
                .validate_non_empty_operands(&[])
                .is_err()
        );
    }

    #[test]
    fn full_membership_accepts_same_members_in_different_order() {
        let register = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(0),
                QubitId::new(1),
                QubitId::new(2),
            ],
            max_qubits(),
        )
        .unwrap();

        assert!(
            register
                .validate_full_membership(&[
                    QubitId::new(2),
                    QubitId::new(0),
                    QubitId::new(1),
                ])
                .is_ok()
        );
    }

    #[test]
    fn full_membership_rejects_missing_member() {
        let register = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(0),
                QubitId::new(1),
                QubitId::new(2),
            ],
            max_qubits(),
        )
        .unwrap();

        assert!(
            register
                .validate_full_membership(&[
                    QubitId::new(0),
                    QubitId::new(1),
                    QubitId::new(3),
                ])
                .is_err()
        );
    }

    #[test]
    fn subset_by_positions_preserves_logical_identity() {
        let register = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(7),
                QubitId::new(2),
                QubitId::new(9),
            ],
            max_qubits(),
        )
        .unwrap();

        let subset = register
            .subset_by_positions(&[2, 0])
            .unwrap();

        assert_eq!(
            subset.as_slice(),
            &[
                QubitId::new(9),
                QubitId::new(7),
            ]
        );
    }

    #[test]
    fn subset_by_qubits_preserves_requested_order() {
        let register = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(0),
                QubitId::new(1),
                QubitId::new(2),
                QubitId::new(3),
            ],
            max_qubits(),
        )
        .unwrap();

        let subset = register
            .subset_by_qubits(&[
                QubitId::new(3),
                QubitId::new(1),
            ])
            .unwrap();

        assert_eq!(
            subset.as_slice(),
            &[
                QubitId::new(3),
                QubitId::new(1),
            ]
        );
    }

    #[test]
    fn subset_rejects_non_members() {
        let register = QuantumRegister::try_contiguous(
            memory_id(),
            QubitCount::new(2),
            max_qubits(),
        )
        .unwrap();

        assert!(
            register
                .subset_by_qubits(&[QubitId::new(99)])
                .is_err()
        );
    }

    #[test]
    fn ordered_membership_differs_from_unordered_membership() {
        let first = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(0),
                QubitId::new(1),
            ],
            max_qubits(),
        )
        .unwrap();

        let second = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(1),
                QubitId::new(0),
            ],
            max_qubits(),
        )
        .unwrap();

        assert!(!first.same_ordered_membership(&second));
        assert!(first.same_membership_unordered(&second));
    }

    #[test]
    fn register_validation_detects_metadata_corruption() {
        let register = QuantumRegister {
            metadata: QuantumRegisterMetadata::new(
                memory_id(),
                QubitCount::new(99),
            ),
            qubits: vec![QubitId::new(0)],
        };

        assert!(register.validate().is_err());
    }

    #[test]
    fn free_validation_helper_rejects_duplicates() {
        assert!(
            validate_unique_qubits(&[
                QubitId::new(1),
                QubitId::new(1),
            ])
            .is_err()
        );
    }

    #[test]
    fn free_membership_helper_works() {
        let register = QuantumRegister::try_contiguous(
            memory_id(),
            QubitCount::new(3),
            max_qubits(),
        )
        .unwrap();

        assert!(
            validate_register_membership(
                &register,
                &[
                    QubitId::new(0),
                    QubitId::new(2),
                ]
            )
            .is_ok()
        );
    }

    #[test]
    fn cardinality_validation_does_not_allocate() {
        let result = validate_register_count(
            QubitCount::new(10),
            QubitCount::new(5),
        );

        assert!(result.is_err());
    }

    #[test]
    fn iteration_is_deterministic() {
        let register = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(8),
                QubitId::new(2),
                QubitId::new(5),
            ],
            max_qubits(),
        )
        .unwrap();

        let collected: Vec<QubitId> =
            register.iter().copied().collect();

        assert_eq!(
            collected,
            vec![
                QubitId::new(8),
                QubitId::new(2),
                QubitId::new(5),
            ]
        );
    }

    #[test]
    fn register_has_no_physical_hardware_assumption() {
        let register = QuantumRegister::try_from_qubits(
            memory_id(),
            vec![
                QubitId::new(100),
                QubitId::new(7),
                QubitId::new(42),
            ],
            max_qubits(),
        )
        .unwrap();

        // The logical IDs themselves are the only hardware-independent
        // information this module owns.
        assert_eq!(
            register.logical_qubits(),
            &[
                QubitId::new(100),
                QubitId::new(7),
                QubitId::new(42),
            ]
        );
    }
}