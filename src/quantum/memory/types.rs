//! Zamani Quantum Memory — Fundamental Types
//!
//! This module defines the foundational, representation-independent types
//! used by `quantum::memory`.
//!
//! # Architectural boundary
//!
//! `quantum::memory::types` owns **memory-domain identities and scalar
//! quantities**. It does not own:
//!
//! - quantum IR semantics;
//! - gate definitions;
//! - circuit representation;
//! - physical hardware topology;
//! - routing;
//! - scheduling;
//! - simulation algorithms;
//! - allocation implementations;
//! - serialization formats;
//! - GPU APIs;
//! - distributed communication.
//!
//! Those responsibilities belong to their respective quantum subsystems.
//!
//! # Canonical identity rule
//!
//! The canonical logical and physical qubit identities are owned by
//! `quantum::ir`:
//!
//! ```text
//! quantum::ir::QubitId
//! quantum::ir::PhysicalQubitId
//! quantum::ir::ClassicalBitId
//! ```
//!
//! This module intentionally does **not** redefine those types.
//!
//! Likewise, circuit and operation identities remain owned by
//! `quantum::ir::identity`:
//!
//! ```text
//! quantum::ir::CircuitId
//! quantum::ir::OperationId
//! ```
//!
//! Memory-specific identities defined here identify memory resources rather
//! than quantum-program semantics.
//!
//! # Dependency direction
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! quantum::memory::types
//!      │
//!      ├── limits
//!      ├── layout
//!      ├── allocator
//!      ├── state
//!      ├── persistence
//!      ├── migration
//!      └── diagnostics
//! ```
//!
//! `types.rs` is deliberately kept below the canonical IR identity layer and
//! above representation-specific memory implementations.
//!
//! # Design principles
//!
//! This module follows these rules:
//!
//! 1. Strongly typed quantities are preferred over raw integers.
//! 2. Memory resource identities are opaque.
//! 3. Arithmetic that can overflow has checked variants.
//! 4. Zero-sized resources are representable.
//! 5. Negative quantities are impossible by construction.
//! 6. No allocation occurs in this module.
//! 7. No `unsafe` code is used.
//! 8. No global mutable state is used.
//! 9. No backend-specific types are exposed.
//! 10. No simulator-specific representation is assumed.
//! 11. Display formatting is deterministic.
//! 12. Serialization is explicit and stable.
//!
//! # Rust compatibility
//!
//! This module targets:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! Later memory modules must use these types rather than introducing
//! replacement aliases such as `usize` for memory-domain quantities.
//!
//! In particular:
//!
//! - `QubitCount` is used for quantum-qubit counts;
//! - `ClassicalBitCount` is used for classical-bit counts;
//! - `ByteCount` is used for memory sizes;
//! - `AmplitudeCount` is used for state-vector element counts;
//! - `MemoryId` identifies a memory resource;
//! - `AllocationId` identifies an allocation;
//! - `SnapshotId` identifies a snapshot;
//! - `CheckpointId` identifies a checkpoint;
//! - `BackendMemoryId` identifies externally owned backend memory.
//!
//! The canonical IR's `QubitId` and `PhysicalQubitId` remain canonical and
//! should be imported from `quantum::ir` wherever logical/physical quantum
//! identity is required.
//!
//! # Serialization contract
//!
//! These types derive Serde serialization because the memory subsystem will
//! eventually need to persist snapshots, checkpoints, diagnostics, and
//! configuration.
//!
//! Numeric memory quantities are serialized as their underlying unsigned
//! integer values. Opaque identities are serialized as numeric values as well.
//!
//! The enclosing snapshot/checkpoint format is responsible for versioning and
//! schema validation. These primitive types do not embed persistence-format
//! policy.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::num::NonZeroU64;

// =============================================================================
// Internal validation helpers
// =============================================================================

/// Error returned when a memory-domain numeric value cannot be represented
/// safely by the requested strongly typed quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuantityError {
    /// A conversion from a `u64` to `usize` failed because the value is larger
    /// than the platform can represent.
    PlatformOverflow {
        value: u64,
    },

    /// A value was required to be non-zero but zero was supplied.
    ZeroNotAllowed,
}

impl fmt::Display for QuantityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlatformOverflow { value } => {
                write!(
                    f,
                    "value {value} cannot be represented as usize on this platform"
                )
            }
            Self::ZeroNotAllowed => {
                write!(f, "zero is not permitted for this quantity")
            }
        }
    }
}

impl std::error::Error for QuantityError {}

// =============================================================================
// Quantum qubit counts
// =============================================================================

/// Number of logical or physical qubits.
///
/// `QubitCount` represents a quantity, not an individual qubit identity.
///
/// Individual logical qubits must use the canonical
/// `quantum::ir::QubitId`.
///
/// Individual physical qubits must use the canonical
/// `quantum::ir::PhysicalQubitId`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct QubitCount(usize);

impl QubitCount {
    /// Zero qubits.
    pub const ZERO: Self = Self(0);

    /// Creates a qubit count.
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    /// Returns the underlying count.
    pub const fn get(self) -> usize {
        self.0
    }

    /// Returns whether the count is zero.
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns whether the count is non-zero.
    pub const fn is_non_zero(self) -> bool {
        self.0 != 0
    }

    /// Checked addition.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked subtraction.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked multiplication.
    pub const fn checked_mul(self, rhs: usize) -> Option<Self> {
        match self.0.checked_mul(rhs) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for QubitCount {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<QubitCount> for usize {
    fn from(value: QubitCount) -> Self {
        value.get()
    }
}

impl fmt::Display for QubitCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} qubit{}", self.0, if self.0 == 1 { "" } else { "s" })
    }
}

// =============================================================================
// Classical-bit counts
// =============================================================================

/// Number of classical bits.
///
/// This is a quantity and must not be confused with the canonical
/// `quantum::ir::ClassicalBitId`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct ClassicalBitCount(usize);

impl ClassicalBitCount {
    /// Zero classical bits.
    pub const ZERO: Self = Self(0);

    /// Creates a classical-bit count.
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    /// Returns the underlying count.
    pub const fn get(self) -> usize {
        self.0
    }

    /// Returns whether the count is zero.
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns whether the count is non-zero.
    pub const fn is_non_zero(self) -> bool {
        self.0 != 0
    }

    /// Checked addition.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked subtraction.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked multiplication.
    pub const fn checked_mul(self, rhs: usize) -> Option<Self> {
        match self.0.checked_mul(rhs) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for ClassicalBitCount {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<ClassicalBitCount> for usize {
    fn from(value: ClassicalBitCount) -> Self {
        value.get()
    }
}

impl fmt::Display for ClassicalBitCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} classical bit{}",
            self.0,
            if self.0 == 1 { "" } else { "s" }
        )
    }
}

// =============================================================================
// Amplitude counts
// =============================================================================

/// Number of amplitudes/elements in a quantum state representation.
///
/// For a dense state vector containing `n` qubits, the expected amplitude
/// count is normally `2^n`.
///
/// This type deliberately represents the count only. It does not perform
/// exponential allocation or imply that the represented state is dense.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct AmplitudeCount(usize);

impl AmplitudeCount {
    /// Zero amplitudes.
    pub const ZERO: Self = Self(0);

    /// Creates an amplitude count.
    pub const fn new(value: usize) -> Self {
        Self(value)
    }

    /// Returns the underlying count.
    pub const fn get(self) -> usize {
        self.0
    }

    /// Returns whether the count is zero.
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns whether the count is non-zero.
    pub const fn is_non_zero(self) -> bool {
        self.0 != 0
    }

    /// Computes `2^qubits` when the result fits into `usize`.
    ///
    /// This function performs no allocation.
    pub fn checked_for_qubits(qubits: QubitCount) -> Option<Self> {
        if qubits.get() >= usize::BITS as usize {
            return None;
        }

        Some(Self::new(1usize << qubits.get()))
    }

    /// Checked multiplication.
    pub const fn checked_mul(self, rhs: usize) -> Option<Self> {
        match self.0.checked_mul(rhs) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }
}

impl From<usize> for AmplitudeCount {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<AmplitudeCount> for usize {
    fn from(value: AmplitudeCount) -> Self {
        value.get()
    }
}

impl fmt::Display for AmplitudeCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} amplitude{}", self.0, if self.0 == 1 { "" } else { "s" })
    }
}

// =============================================================================
// Byte counts
// =============================================================================

/// Number of bytes occupied, reserved, or required by a memory resource.
///
/// `ByteCount` is deliberately independent of a particular allocator or
/// address space.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct ByteCount(u64);

impl ByteCount {
    /// Zero bytes.
    pub const ZERO: Self = Self(0);

    /// One byte.
    pub const ONE: Self = Self(1);

    /// One kibibyte.
    pub const KIB: Self = Self(1024);

    /// One mebibyte.
    pub const MIB: Self = Self(1024 * 1024);

    /// One gibibyte.
    pub const GIB: Self = Self(1024 * 1024 * 1024);

    /// One tebibyte.
    pub const TIB: Self = Self(1024 * 1024 * 1024 * 1024);

    /// Creates a byte count.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the underlying byte count.
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns whether this is zero bytes.
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns whether this is non-zero.
    pub const fn is_non_zero(self) -> bool {
        self.0 != 0
    }

    /// Checked addition.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0.checked_add(rhs.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked subtraction.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.0.checked_sub(rhs.0) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Checked multiplication.
    pub const fn checked_mul(self, rhs: u64) -> Option<Self> {
        match self.0.checked_mul(rhs) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Converts to `usize` when representable on the current platform.
    pub fn try_as_usize(self) -> Result<usize, QuantityError> {
        usize::try_from(self.0)
            .map_err(|_| QuantityError::PlatformOverflow { value: self.0 })
    }

    /// Returns the whole kibibyte quantity.
    pub const fn kibibytes(self) -> u64 {
        self.0 / 1024
    }

    /// Returns the whole mebibyte quantity.
    pub const fn mebibytes(self) -> u64 {
        self.0 / (1024 * 1024)
    }

    /// Returns the whole gibibyte quantity.
    pub const fn gibibytes(self) -> u64 {
        self.0 / (1024 * 1024 * 1024)
    }

    /// Returns the whole tebibyte quantity.
    pub const fn tebibytes(self) -> u64 {
        self.0 / (1024 * 1024 * 1024 * 1024)
    }
}

impl From<u64> for ByteCount {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<ByteCount> for u64 {
    fn from(value: ByteCount) -> Self {
        value.get()
    }
}

impl fmt::Display for ByteCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} B", self.0)
    }
}

// =============================================================================
// Memory resource identity
// =============================================================================

/// Opaque identity of a managed quantum-memory resource.
///
/// `MemoryId` identifies a memory object, arena, pool-owned resource, or other
/// logical memory domain depending on the owning subsystem.
///
/// It must never be interpreted as a raw pointer or process address.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct MemoryId(u64);

impl MemoryId {
    /// Creates a memory identity.
    ///
    /// Identity allocation belongs to the owning subsystem. This constructor
    /// does not allocate or register anything.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the opaque numeric identity.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for MemoryId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<MemoryId> for u64 {
    fn from(id: MemoryId) -> Self {
        id.value()
    }
}

impl fmt::Display for MemoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mem{}", self.0)
    }
}

// =============================================================================
// Allocation identity
// =============================================================================

/// Opaque identity of an individual memory allocation.
///
/// An `AllocationId` identifies an allocation event/resource, not a raw
/// address.
///
/// It remains valid as an identity even when an allocator migrates the
/// underlying storage between host/device/distributed memory.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct AllocationId(u64);

impl AllocationId {
    /// Creates an allocation identity.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the opaque numeric identity.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for AllocationId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<AllocationId> for u64 {
    fn from(id: AllocationId) -> Self {
        id.value()
    }
}

impl fmt::Display for AllocationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "alloc{}", self.0)
    }
}

// =============================================================================
// Snapshot identity
// =============================================================================

/// Opaque identity for an immutable quantum-memory snapshot.
///
/// Snapshot identity is independent of the snapshot payload and must not be
/// treated as a cryptographic content hash.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct SnapshotId(u64);

impl SnapshotId {
    /// Creates a snapshot identity.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the opaque numeric identity.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for SnapshotId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<SnapshotId> for u64 {
    fn from(id: SnapshotId) -> Self {
        id.value()
    }
}

impl fmt::Display for SnapshotId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "snap{}", self.0)
    }
}

// =============================================================================
// Checkpoint identity
// =============================================================================

/// Opaque identity for a restartable quantum-memory checkpoint.
///
/// A checkpoint may later contain quantum state, classical state, execution
/// position, RNG state, backend metadata, and other restart information.
/// Those semantics belong to `checkpoint.rs`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct CheckpointId(u64);

impl CheckpointId {
    /// Creates a checkpoint identity.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the opaque numeric identity.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for CheckpointId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<CheckpointId> for u64 {
    fn from(id: CheckpointId) -> Self {
        id.value()
    }
}

impl fmt::Display for CheckpointId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "chk{}", self.0)
    }
}

// =============================================================================
// Backend memory identity
// =============================================================================

/// Opaque identity for memory owned by an external execution backend.
///
/// This may represent, for example:
///
/// - a simulator-owned state handle;
/// - a GPU allocation;
/// - a distributed state partition;
/// - a remote QPU session's memory object.
///
/// It deliberately carries no vendor-specific semantics.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
pub struct BackendMemoryId(u64);

impl BackendMemoryId {
    /// Creates a backend-memory identity.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the opaque numeric identity.
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl From<u64> for BackendMemoryId {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<BackendMemoryId> for u64 {
    fn from(id: BackendMemoryId) -> Self {
        id.value()
    }
}

impl fmt::Display for BackendMemoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "backend-mem{}", self.0)
    }
}

// =============================================================================
// Memory size arithmetic
// =============================================================================

/// Computes the number of bytes required for `elements` elements of
/// `element_size` bytes each.
///
/// This function performs checked arithmetic and therefore cannot wrap on
/// overflow.
///
/// It is intentionally located in `types.rs` because multiple later memory
/// representations need the same primitive calculation.
pub const fn checked_byte_size(
    elements: usize,
    element_size: usize,
) -> Option<ByteCount> {
    match elements.checked_mul(element_size) {
        Some(bytes) => Some(ByteCount::new(bytes as u64)),
        None => None,
    }
}

/// Computes the number of bytes required for a quantum state containing
/// `amplitudes` elements with the specified element size.
///
/// This is representation-neutral. The caller determines whether an element
/// is a real scalar, complex scalar, tensor element, or other representation.
pub fn checked_amplitude_bytes(
    amplitudes: AmplitudeCount,
    element_size: usize,
) -> Option<ByteCount> {
    checked_byte_size(amplitudes.get(), element_size)
}

// =============================================================================
// Dense-state dimension helpers
// =============================================================================

/// Computes the dense state-vector amplitude count for a qubit count.
///
/// The calculation is:
///
/// ```text
/// amplitudes = 2^n
/// ```
///
/// No memory is allocated.
///
/// The operation returns `None` when `2^n` cannot be represented by `usize`.
pub fn checked_state_vector_amplitudes(
    qubits: QubitCount,
) -> Option<AmplitudeCount> {
    AmplitudeCount::checked_for_qubits(qubits)
}

/// Computes the number of scalar elements in a dense density matrix for
/// `qubits` qubits.
///
/// The calculation is:
///
/// ```text
/// matrix elements = 2^n × 2^n = 4^n
/// ```
///
/// No memory is allocated.
///
/// The operation returns `None` when the result cannot be represented by
/// `usize`.
pub fn checked_density_matrix_elements(
    qubits: QubitCount,
) -> Option<AmplitudeCount> {
    let amplitudes = checked_state_vector_amplitudes(qubits)?;

    amplitudes.checked_mul(amplitudes.get())
}

// =============================================================================
// Non-zero identity helper
// =============================================================================

/// Converts a `u64` into a non-zero identity component.
///
/// This helper is intentionally small and does not allocate.
pub fn non_zero_identity(value: u64) -> Result<NonZeroU64, QuantityError> {
    NonZeroU64::new(value).ok_or(QuantityError::ZeroNotAllowed)
}

// =============================================================================
// Canonical IR integration helpers
// =============================================================================

/// Converts a canonical IR logical qubit identifier into its memory index.
///
/// This function deliberately accepts the canonical IR type instead of
/// defining another memory-local logical-qubit identity.
///
/// The memory index is only an index into a memory representation; it does not
/// change the identity semantics of the IR qubit.
#[inline]
pub const fn logical_qubit_index(
    qubit: crate::quantum::ir::QubitId,
) -> usize {
    qubit.index()
}

/// Converts a canonical IR physical qubit identifier into a memory index.
///
/// Routing and hardware remain responsible for determining whether that
/// physical identifier is valid for a particular device.
#[inline]
pub const fn physical_qubit_index(
    qubit: crate::quantum::ir::PhysicalQubitId,
) -> usize {
    qubit.index()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::{PhysicalQubitId, QubitId};

    #[test]
    fn qubit_count_is_strongly_typed() {
        let count = QubitCount::new(128);

        assert_eq!(count.get(), 128);
        assert!(!count.is_zero());
        assert!(count.is_non_zero());
        assert_eq!(count.to_string(), "128 qubits");
    }

    #[test]
    fn classical_bit_count_is_strongly_typed() {
        let count = ClassicalBitCount::new(1);

        assert_eq!(count.get(), 1);
        assert_eq!(count.to_string(), "1 classical bit");
    }

    #[test]
    fn amplitude_count_for_zero_qubits_is_one() {
        let amplitudes =
            checked_state_vector_amplitudes(QubitCount::ZERO)
                .expect("2^0 must be representable");

        assert_eq!(amplitudes.get(), 1);
    }

    #[test]
    fn amplitude_count_is_exponential() {
        let amplitudes =
            checked_state_vector_amplitudes(QubitCount::new(10))
                .expect("2^10 must be representable");

        assert_eq!(amplitudes.get(), 1024);
    }

    #[test]
    fn density_matrix_dimension_is_four_to_the_n() {
        let elements =
            checked_density_matrix_elements(QubitCount::new(3))
                .expect("4^3 must be representable");

        assert_eq!(elements.get(), 64);
    }

    #[test]
    fn amplitude_count_rejects_unrepresentable_shift() {
        let count = QubitCount::new(usize::BITS as usize);

        assert!(
            checked_state_vector_amplitudes(count).is_none()
        );
    }

    #[test]
    fn byte_arithmetic_is_checked() {
        let size =
            checked_byte_size(1024, 16)
                .expect("1024 × 16 must fit");

        assert_eq!(size.get(), 16_384);
    }

    #[test]
    fn byte_arithmetic_rejects_overflow() {
        let size =
            checked_byte_size(usize::MAX, 2);

        assert!(size.is_none());
    }

    #[test]
    fn byte_count_units_are_binary() {
        assert_eq!(ByteCount::KIB.get(), 1024);
        assert_eq!(ByteCount::MIB.get(), 1024 * 1024);
        assert_eq!(
            ByteCount::GIB.get(),
            1024 * 1024 * 1024
        );
    }

    #[test]
    fn byte_count_converts_to_usize_when_possible() {
        let value = ByteCount::new(4096);

        assert_eq!(
            value
                .try_as_usize()
                .expect("4096 fits"),
            4096
        );
    }

    #[test]
    fn memory_identity_is_opaque() {
        let id = MemoryId::new(42);

        assert_eq!(id.value(), 42);
        assert_eq!(u64::from(id), 42);
        assert_eq!(id.to_string(), "mem42");
    }

    #[test]
    fn allocation_identity_is_distinct_from_memory_identity() {
        let memory = MemoryId::new(7);
        let allocation = AllocationId::new(7);

        // They intentionally have the same underlying numeric value while
        // remaining different Rust types.
        assert_eq!(memory.value(), allocation.value());
    }

    #[test]
    fn snapshot_and_checkpoint_identities_are_distinct() {
        let snapshot = SnapshotId::new(1);
        let checkpoint = CheckpointId::new(1);

        assert_eq!(snapshot.value(), checkpoint.value());
        assert_eq!(snapshot.to_string(), "snap1");
        assert_eq!(checkpoint.to_string(), "chk1");
    }

    #[test]
    fn backend_memory_identity_is_opaque() {
        let id = BackendMemoryId::new(99);

        assert_eq!(id.value(), 99);
        assert_eq!(id.to_string(), "backend-mem99");
    }

    #[test]
    fn checked_quantity_addition_is_safe() {
        let a = QubitCount::new(10);
        let b = QubitCount::new(20);

        assert_eq!(
            a.checked_add(b)
                .expect("10 + 20 must fit")
                .get(),
            30
        );
    }

    #[test]
    fn checked_quantity_subtraction_rejects_underflow() {
        let a = QubitCount::new(10);
        let b = QubitCount::new(20);

        assert!(a.checked_sub(b).is_none());
    }

    #[test]
    fn checked_byte_addition_rejects_overflow() {
        let a = ByteCount::new(u64::MAX);
        let b = ByteCount::ONE;

        assert!(a.checked_add(b).is_none());
    }

    #[test]
    fn canonical_ir_qubit_ids_are_used_without_duplication() {
        let logical = QubitId::new(7);
        let physical = PhysicalQubitId::new(11);

        assert_eq!(logical_qubit_index(logical), 7);
        assert_eq!(physical_qubit_index(physical), 11);
    }

    #[test]
    fn zero_identity_is_rejected_when_non_zero_is_required() {
        assert_eq!(
            non_zero_identity(0),
            Err(QuantityError::ZeroNotAllowed)
        );
    }

    #[test]
    fn non_zero_identity_is_constructed() {
        let identity =
            non_zero_identity(42)
                .expect("42 is non-zero");

        assert_eq!(identity.get(), 42);
    }

    #[test]
    fn quantity_conversions_round_trip() {
        let qubits = QubitCount::new(123);
        let classical = ClassicalBitCount::new(456);
        let amplitudes = AmplitudeCount::new(789);
        let bytes = ByteCount::new(1000);

        assert_eq!(
            QubitCount::from(usize::from(qubits)),
            qubits
        );

        assert_eq!(
            ClassicalBitCount::from(usize::from(classical)),
            classical
        );

        assert_eq!(
            AmplitudeCount::from(usize::from(amplitudes)),
            amplitudes
        );

        assert_eq!(
            ByteCount::from(u64::from(bytes)),
            bytes
        );
    }

    #[test]
    fn serde_round_trip_preserves_fundamental_types() {
        let memory = MemoryId::new(123);
        let bytes = ByteCount::new(4096);
        let qubits = QubitCount::new(32);

        let encoded =
            serde_json::to_string(&(memory, bytes, qubits))
                .expect("serialization must succeed");

        let decoded:
            (MemoryId, ByteCount, QubitCount) =
            serde_json::from_str(&encoded)
                .expect("deserialization must succeed");

        assert_eq!(decoded.0, memory);
        assert_eq!(decoded.1, bytes);
        assert_eq!(decoded.2, qubits);
    }
}