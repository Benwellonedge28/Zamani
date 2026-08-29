//! Zamani Quantum Memory — Memory Layout
//!
//! Defines the canonical mapping between logical quantum-qubit order and
//! storage/physical positions used by the quantum-memory subsystem.
//!
//! # Architectural role
//!
//! `layout.rs` owns **layout semantics**, not quantum-state semantics.
//!
//! It answers questions such as:
//!
//! - Which logical qubit corresponds to a particular storage bit position?
//! - Which storage bit represents a particular logical qubit?
//! - In what order are qubits encoded into basis-state indices?
//! - What is the stride of each logical qubit in a dense representation?
//! - Is storage contiguous, strided, blocked, or otherwise explicitly mapped?
//!
//! It does **not** own:
//!
//! - quantum amplitudes;
//! - state vectors;
//! - density matrices;
//! - tensor-network data;
//! - physical hardware topology;
//! - routing algorithms;
//! - scheduling;
//! - allocation;
//! - GPU implementation;
//! - distributed communication;
//! - measurement;
//! - gate semantics.
//!
//! Those responsibilities belong to their respective quantum subsystems.
//!
//! # Canonical dependency boundary
//!
//! ```text
//! quantum::ir
//!      │
//!      │ QubitId / PhysicalQubitId
//!      ▼
//! quantum::memory::layout
//!      │
//!      ├──────────────► state_vector
//!      ├──────────────► density_matrix
//!      ├──────────────► tensor_network
//!      ├──────────────► view
//!      ├──────────────► permutation
//!      ├──────────────► serialization
//!      ├──────────────► GPU/distributed memory
//!      └──────────────► routing integration
//! ```
//!
//! `MemoryLayout` is intentionally independent of any particular state
//! representation. A state-vector implementation may use its mapping to
//! calculate basis-state indices, while a tensor-network implementation may
//! use the same logical ordering without assuming dense state-vector storage.
//!
//! # Endianness
//!
//! Quantum software frequently disagrees about whether qubit zero is the least
//! significant or most significant basis bit. Zamani therefore does **not**
//! assume an implicit global endian convention.
//!
//! `BitOrder::LittleEndian` means logical/storage position zero contributes
//! the least-significant bit to a basis index.
//!
//! `BitOrder::BigEndian` means logical/storage position zero contributes the
//! most-significant bit to a basis index.
//!
//! A custom logical ordering is represented independently through
//! `QubitOrder`.
//!
//! # Safety
//!
//! This module contains no `unsafe` code and exposes no raw pointers.
//!
//! All index arithmetic that can overflow is checked.
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
//! Later memory modules must consume this API rather than creating their own
//! endian, stride, or logical-to-storage mapping implementations.
//!
//! In particular:
//!
//! - `state_vector.rs` uses `MemoryLayout::basis_index` and
//!   `MemoryLayout::qubit_stride`;
//! - `density_matrix.rs` uses the same basis mapping for row/column indices;
//! - `view.rs` uses layout transformations without taking ownership of state;
//! - `permutation.rs` constructs explicit `QubitOrder` values;
//! - `tensor_network.rs` uses logical order independently from tensor storage;
//! - `serialization.rs` persists the layout explicitly;
//! - `snapshot.rs` stores layout identity and order;
//! - `routing.rs` may translate its logical-to-physical mapping into a
//!   `QubitOrder`, but does not own this layout abstraction;
//! - `gpu.rs` and `distributed.rs` consume the same logical layout contract;
//! - `indexing.rs` may use this module's checked mapping rather than
//!   reimplementing endian arithmetic.
//!
//! # Important invariant
//!
//! A `MemoryLayout` is immutable after construction.
//!
//! If a program needs another ordering, it creates another layout or an
//! explicit permutation. This prevents a state representation from silently
//! changing the meaning of existing indices while it is being used.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::quantum::ir::{PhysicalQubitId, QubitId};

// =============================================================================
// Constants
// =============================================================================

/// Maximum number of qubits for which a dense basis index can be represented
/// by the `usize`-based indexing API.
///
/// This is intentionally conservative.
///
/// A basis index requires `n` bits. Therefore a platform with `usize::BITS`
/// bits cannot safely represent all `2^usize::BITS` basis states.
///
/// The maximum representable qubit count is one less than the number of bits
/// in `usize`, because `2^n - 1` must itself fit into `usize`.
const MAX_BASIS_INDEX_QUBITS: usize = usize::BITS as usize - 1;

// =============================================================================
// Result / Error
// =============================================================================

/// Result type used by the memory-layout subsystem.
pub type LayoutResult<T> = Result<T, LayoutError>;

/// Errors produced by layout construction and checked layout operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutError {
    /// A layout was requested with an invalid number of qubits.
    InvalidQubitCount {
        count: usize,
    },

    /// The requested qubit count cannot be represented by the basis-index API.
    QubitCountTooLarge {
        count: usize,
        maximum: usize,
    },

    /// A logical qubit is outside the layout's logical namespace.
    LogicalQubitOutOfRange {
        qubit: QubitId,
        num_qubits: usize,
    },

    /// A physical qubit is outside the layout's physical namespace.
    PhysicalQubitOutOfRange {
        qubit: PhysicalQubitId,
        num_positions: usize,
    },

    /// A supplied qubit order has the wrong length.
    InvalidOrderLength {
        expected: usize,
        actual: usize,
    },

    /// A supplied qubit order contains the same logical qubit more than once.
    DuplicateLogicalQubit {
        qubit: QubitId,
    },

    /// A supplied logical qubit is outside the permitted namespace.
    InvalidLogicalQubit {
        qubit: QubitId,
    },

    /// A supplied physical position is duplicated.
    DuplicatePhysicalPosition {
        position: usize,
    },

    /// A stride is zero.
    ZeroStride,

    /// A stride or offset calculation overflowed.
    ArithmeticOverflow,

    /// A basis-index operation was requested for a basis value containing an
    /// invalid bit.
    BasisValueOutOfRange {
        basis: usize,
        num_qubits: usize,
    },

    /// A requested storage position cannot be represented.
    StoragePositionOutOfRange {
        position: usize,
        num_positions: usize,
    },

    /// A layout transformation cannot be represented without changing the
    /// declared layout contract.
    InvalidTransformation {
        reason: String,
    },
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCount { count } => {
                write!(f, "invalid quantum-memory qubit count: {count}")
            }

            Self::QubitCountTooLarge { count, maximum } => {
                write!(
                    f,
                    "qubit count {count} exceeds maximum representable basis-index \
                     width {maximum}"
                )
            }

            Self::LogicalQubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "logical qubit {qubit} is outside layout range 0..{num_qubits}"
                )
            }

            Self::PhysicalQubitOutOfRange {
                qubit,
                num_positions,
            } => {
                write!(
                    f,
                    "physical qubit {qubit} is outside layout position range \
                     0..{num_positions}"
                )
            }

            Self::InvalidOrderLength { expected, actual } => {
                write!(
                    f,
                    "invalid qubit-order length: expected {expected}, got {actual}"
                )
            }

            Self::DuplicateLogicalQubit { qubit } => {
                write!(f, "logical qubit {qubit} occurs more than once in the order")
            }

            Self::InvalidLogicalQubit { qubit } => {
                write!(f, "logical qubit {qubit} is invalid for this layout")
            }

            Self::DuplicatePhysicalPosition { position } => {
                write!(
                    f,
                    "physical storage position {position} occurs more than once"
                )
            }

            Self::ZeroStride => {
                write!(f, "a memory-layout stride must be greater than zero")
            }

            Self::ArithmeticOverflow => {
                write!(f, "memory-layout arithmetic overflow")
            }

            Self::BasisValueOutOfRange {
                basis,
                num_qubits,
            } => {
                write!(
                    f,
                    "basis value {basis} is outside the {num_qubits}-qubit basis space"
                )
            }

            Self::StoragePositionOutOfRange {
                position,
                num_positions,
            } => {
                write!(
                    f,
                    "storage position {position} is outside range 0..{num_positions}"
                )
            }

            Self::InvalidTransformation { reason } => {
                write!(f, "invalid memory-layout transformation: {reason}")
            }
        }
    }
}

impl std::error::Error for LayoutError {}

// =============================================================================
// Bit order
// =============================================================================

/// Defines how a qubit position contributes to a basis-state integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BitOrder {
    /// Position zero contributes the least-significant bit.
    ///
    /// For qubits `[q0, q1, q2]`:
    ///
    /// ```text
    /// q0 -> bit 0
    /// q1 -> bit 1
    /// q2 -> bit 2
    /// ```
    LittleEndian,

    /// Position zero contributes the most-significant bit.
    ///
    /// For qubits `[q0, q1, q2]`:
    ///
    /// ```text
    /// q0 -> bit 2
    /// q1 -> bit 1
    /// q2 -> bit 0
    /// ```
    BigEndian,
}

impl Default for BitOrder {
    fn default() -> Self {
        Self::LittleEndian
    }
}

impl BitOrder {
    /// Returns the bit position corresponding to a zero-based storage
    /// position.
    pub const fn bit_position(self, position: usize, num_qubits: usize) -> usize {
        match self {
            Self::LittleEndian => position,
            Self::BigEndian => num_qubits - 1 - position,
        }
    }

    /// Returns whether the position is interpreted as the least-significant
    /// side of the basis index.
    pub const fn is_little_endian(self) -> bool {
        matches!(self, Self::LittleEndian)
    }

    /// Returns whether the position is interpreted as the most-significant
    /// side of the basis index.
    pub const fn is_big_endian(self) -> bool {
        matches!(self, Self::BigEndian)
    }
}

// =============================================================================
// Storage model
// =============================================================================

/// Describes the physical organization of logical positions in memory.
///
/// This enum describes the layout contract, not a concrete allocator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageLayout {
    /// Positions are contiguous with unit stride.
    Contiguous,

    /// Positions are separated by a fixed positive stride.
    Strided {
        stride: usize,
    },

    /// Positions are organized into fixed-size blocks.
    ///
    /// The final block may contain fewer logical positions.
    Blocked {
        block_size: usize,
    },

    /// Positions use an explicit permutation.
    ///
    /// The actual mapping is carried by `QubitOrder`.
    Permuted,
}

impl Default for StorageLayout {
    fn default() -> Self {
        Self::Contiguous
    }
}

impl StorageLayout {
    /// Validates the structural parameters of the storage layout.
    fn validate(self) -> LayoutResult<()> {
        match self {
            Self::Contiguous | Self::Permuted => Ok(()),

            Self::Strided { stride } => {
                if stride == 0 {
                    Err(LayoutError::ZeroStride)
                } else {
                    Ok(())
                }
            }

            Self::Blocked { block_size } => {
                if block_size == 0 {
                    Err(LayoutError::InvalidTransformation {
                        reason: "block size must be greater than zero".to_owned(),
                    })
                } else {
                    Ok(())
                }
            }
        }
    }
}

// =============================================================================
// Qubit order
// =============================================================================

/// Canonical logical-to-storage ordering.
///
/// `QubitOrder` contains each logical qubit exactly once.
///
/// The vector is interpreted as:
///
/// ```text
/// storage_position -> logical_qubit
/// ```
///
/// Example:
///
/// ```text
/// [q2, q0, q1]
/// ```
///
/// means:
///
/// ```text
/// storage position 0 -> q2
/// storage position 1 -> q0
/// storage position 2 -> q1
/// ```
///
/// This representation is deliberately independent of physical hardware
/// topology. Routing can create such an order, but routing remains responsible
/// for deciding *why* the mapping exists.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QubitOrder {
    logical_by_position: Vec<QubitId>,
}

impl QubitOrder {
    /// Creates the canonical identity order:
    ///
    /// ```text
    /// q0, q1, q2, ..., q(n-1)
    /// ```
    pub fn identity(num_qubits: usize) -> LayoutResult<Self> {
        validate_qubit_count(num_qubits)?;

        let logical_by_position = (0..num_qubits)
            .map(QubitId::new)
            .collect::<Vec<_>>();

        Ok(Self {
            logical_by_position,
        })
    }

    /// Creates an explicit logical-to-storage order.
    ///
    /// The input must contain every logical qubit exactly once and must use
    /// identifiers in `0..num_qubits`.
    pub fn try_from_logical_order(
        num_qubits: usize,
        order: Vec<QubitId>,
    ) -> LayoutResult<Self> {
        validate_qubit_count(num_qubits)?;

        if order.len() != num_qubits {
            return Err(LayoutError::InvalidOrderLength {
                expected: num_qubits,
                actual: order.len(),
            });
        }

        let mut seen = vec![false; num_qubits];

        for &qubit in &order {
            let index = qubit.index();

            if index >= num_qubits {
                return Err(LayoutError::InvalidLogicalQubit { qubit });
            }

            if seen[index] {
                return Err(LayoutError::DuplicateLogicalQubit { qubit });
            }

            seen[index] = true;
        }

        Ok(Self {
            logical_by_position: order,
        })
    }

    /// Creates an order from a slice.
    pub fn try_from_slice(
        num_qubits: usize,
        order: &[QubitId],
    ) -> LayoutResult<Self> {
        Self::try_from_logical_order(num_qubits, order.to_vec())
    }

    /// Returns the number of logical qubits.
    pub fn len(&self) -> usize {
        self.logical_by_position.len()
    }

    /// Returns whether the order contains no qubits.
    pub fn is_empty(&self) -> bool {
        self.logical_by_position.is_empty()
    }

    /// Returns the logical qubit stored at a storage position.
    pub fn logical_at(&self, position: usize) -> Option<QubitId> {
        self.logical_by_position.get(position).copied()
    }

    /// Returns the storage position of a logical qubit.
    pub fn position_of(&self, qubit: QubitId) -> LayoutResult<usize> {
        if qubit.index() >= self.len() {
            return Err(LayoutError::LogicalQubitOutOfRange {
                qubit,
                num_qubits: self.len(),
            });
        }

        // The order is normally small compared with the state representation,
        // and the linear search keeps this foundational type allocation-free.
        //
        // `MemoryLayout` additionally maintains the inverse mapping used by
        // hot-path operations.
        self.logical_by_position
            .iter()
            .position(|candidate| *candidate == qubit)
            .ok_or(LayoutError::InvalidLogicalQubit { qubit })
    }

    /// Returns the complete storage-position → logical-qubit mapping.
    pub fn as_slice(&self) -> &[QubitId] {
        &self.logical_by_position
    }

    /// Returns whether this order is the canonical identity order.
    pub fn is_identity(&self) -> bool {
        self.logical_by_position
            .iter()
            .enumerate()
            .all(|(position, qubit)| qubit.index() == position)
    }

    /// Returns a reversed logical order.
    pub fn reversed(num_qubits: usize) -> LayoutResult<Self> {
        validate_qubit_count(num_qubits)?;

        let order = (0..num_qubits)
            .rev()
            .map(QubitId::new)
            .collect::<Vec<_>>();

        Self::try_from_logical_order(num_qubits, order)
    }

    /// Creates a new order by applying a permutation expressed as
    /// `new_position -> old_position`.
    ///
    /// This method does not mutate the original order.
    pub fn permute(&self, new_to_old: &[usize]) -> LayoutResult<Self> {
        if new_to_old.len() != self.len() {
            return Err(LayoutError::InvalidOrderLength {
                expected: self.len(),
                actual: new_to_old.len(),
            });
        }

        let mut seen = vec![false; self.len()];
        let mut result = Vec::with_capacity(self.len());

        for &old_position in new_to_old {
            if old_position >= self.len() {
                return Err(LayoutError::StoragePositionOutOfRange {
                    position: old_position,
                    num_positions: self.len(),
                });
            }

            if seen[old_position] {
                return Err(LayoutError::DuplicatePhysicalPosition {
                    position: old_position,
                });
            }

            seen[old_position] = true;

            result.push(self.logical_by_position[old_position]);
        }

        Self::try_from_logical_order(self.len(), result)
    }
}

// =============================================================================
// Memory layout
// =============================================================================

/// Immutable canonical mapping between logical qubits and storage positions.
///
/// The layout is representation-independent but provides the operations
/// needed by dense state-vector and matrix implementations.
///
/// # Mapping
///
/// ```text
/// logical qubit
///      │
///      ▼
/// logical_to_position
///      │
///      ▼
/// storage position
///      │
///      ▼
/// bit order
///      │
///      ▼
/// basis-index bit
/// ```
///
/// The inverse mapping is maintained explicitly so hot-path operations do not
/// need to scan the logical order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryLayout {
    num_qubits: usize,
    bit_order: BitOrder,
    storage_layout: StorageLayout,
    order: QubitOrder,
    logical_to_position: Vec<usize>,
}

impl MemoryLayout {
    /// Creates the canonical contiguous little-endian layout.
    pub fn new(num_qubits: usize) -> LayoutResult<Self> {
        Self::try_new(
            num_qubits,
            BitOrder::LittleEndian,
            StorageLayout::Contiguous,
            QubitOrder::identity(num_qubits)?,
        )
    }

    /// Creates a layout with explicit bit order.
    pub fn with_bit_order(
        num_qubits: usize,
        bit_order: BitOrder,
    ) -> LayoutResult<Self> {
        Self::try_new(
            num_qubits,
            bit_order,
            StorageLayout::Contiguous,
            QubitOrder::identity(num_qubits)?,
        )
    }

    /// Creates a fully specified layout.
    ///
    /// This constructor validates all structural invariants before returning.
    pub fn try_new(
        num_qubits: usize,
        bit_order: BitOrder,
        storage_layout: StorageLayout,
        order: QubitOrder,
    ) -> LayoutResult<Self> {
        validate_qubit_count(num_qubits)?;
        storage_layout.validate()?;

        if order.len() != num_qubits {
            return Err(LayoutError::InvalidOrderLength {
                expected: num_qubits,
                actual: order.len(),
            });
        }

        let mut logical_to_position = vec![0usize; num_qubits];

        for position in 0..num_qubits {
            let qubit = order
                .logical_at(position)
                .ok_or(LayoutError::StoragePositionOutOfRange {
                    position,
                    num_positions: num_qubits,
                })?;

            logical_to_position[qubit.index()] = position;
        }

        Ok(Self {
            num_qubits,
            bit_order,
            storage_layout,
            order,
            logical_to_position,
        })
    }

    /// Creates an explicitly permuted layout.
    pub fn from_order(
        num_qubits: usize,
        bit_order: BitOrder,
        order: QubitOrder,
    ) -> LayoutResult<Self> {
        Self::try_new(
            num_qubits,
            bit_order,
            StorageLayout::Permuted,
            order,
        )
    }

    /// Returns the number of logical qubits.
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the configured bit order.
    pub const fn bit_order(&self) -> BitOrder {
        self.bit_order
    }

    /// Returns the storage organization.
    pub const fn storage_layout(&self) -> StorageLayout {
        self.storage_layout
    }

    /// Returns the complete logical ordering.
    pub fn qubit_order(&self) -> &QubitOrder {
        &self.order
    }

    /// Returns whether the layout is the canonical identity ordering.
    pub fn is_identity(&self) -> bool {
        self.order.is_identity()
            && matches!(self.storage_layout, StorageLayout::Contiguous)
    }

    /// Returns the storage position of a logical qubit.
    #[inline]
    pub fn position_of(&self, qubit: QubitId) -> LayoutResult<usize> {
        self.validate_logical_qubit(qubit)?;
        Ok(self.logical_to_position[qubit.index()])
    }

    /// Returns the logical qubit at a storage position.
    #[inline]
    pub fn logical_at(&self, position: usize) -> LayoutResult<QubitId> {
        self.order
            .logical_at(position)
            .ok_or(LayoutError::StoragePositionOutOfRange {
                position,
                num_positions: self.num_qubits,
            })
    }

    /// Returns the basis-index bit associated with a logical qubit.
    #[inline]
    pub fn bit_position(&self, qubit: QubitId) -> LayoutResult<usize> {
        let storage_position = self.position_of(qubit)?;

        Ok(self
            .bit_order
            .bit_position(storage_position, self.num_qubits))
    }

    /// Returns the bit mask associated with a logical qubit.
    ///
    /// Returns an error instead of shifting when the shift would not be
    /// representable.
    #[inline]
    pub fn bit_mask(&self, qubit: QubitId) -> LayoutResult<usize> {
        let bit = self.bit_position(qubit)?;

        checked_bit_mask(bit)
    }

    /// Returns the basis-state index for a logical bit assignment.
    ///
    /// `bits` contains logical qubit values in the same namespace as the
    /// layout. A `true` value means that qubit is in logical state `|1>`.
    ///
    /// Example for identity/little-endian layout:
    ///
    /// ```text
    /// q0 = 1
    /// q1 = 0
    /// q2 = 1
    ///
    /// basis = 0b101 = 5
    /// ```
    pub fn basis_index_from_bits(
        &self,
        bits: &[bool],
    ) -> LayoutResult<usize> {
        if bits.len() != self.num_qubits {
            return Err(LayoutError::InvalidOrderLength {
                expected: self.num_qubits,
                actual: bits.len(),
            });
        }

        let mut basis = 0usize;

        for (index, &value) in bits.iter().enumerate() {
            if value {
                let qubit = QubitId::new(index);
                basis |= self.bit_mask(qubit)?;
            }
        }

        Ok(basis)
    }

    /// Returns the logical bit value for a qubit in a basis-state index.
    #[inline]
    pub fn bit_value(
        &self,
        basis: usize,
        qubit: QubitId,
    ) -> LayoutResult<bool> {
        self.validate_basis(basis)?;
        let mask = self.bit_mask(qubit)?;

        Ok((basis & mask) != 0)
    }

    /// Returns all logical qubit bits represented by a basis-state index.
    ///
    /// The returned vector is indexed by logical qubit identifier:
    ///
    /// ```text
    /// result[q0] -> q0 bit
    /// result[q1] -> q1 bit
    /// ...
    /// ```
    pub fn bits_from_basis(&self, basis: usize) -> LayoutResult<Vec<bool>> {
        self.validate_basis(basis)?;

        let mut bits = Vec::with_capacity(self.num_qubits);

        for index in 0..self.num_qubits {
            bits.push(self.bit_value(basis, QubitId::new(index))?);
        }

        Ok(bits)
    }

    /// Returns the basis index obtained by changing one logical qubit to the
    /// requested bit value.
    #[inline]
    pub fn with_bit(
        &self,
        basis: usize,
        qubit: QubitId,
        value: bool,
    ) -> LayoutResult<usize> {
        self.validate_basis(basis)?;
        let mask = self.bit_mask(qubit)?;

        if value {
            Ok(basis | mask)
        } else {
            Ok(basis & !mask)
        }
    }

    /// Flips one logical qubit in a basis-state index.
    #[inline]
    pub fn flip_bit(
        &self,
        basis: usize,
        qubit: QubitId,
    ) -> LayoutResult<usize> {
        self.validate_basis(basis)?;
        let mask = self.bit_mask(qubit)?;

        Ok(basis ^ mask)
    }

    /// Returns the dense basis-space dimension `2^n`.
    ///
    /// This method checks the exponent before shifting.
    pub fn basis_dimension(&self) -> LayoutResult<usize> {
        checked_basis_dimension(self.num_qubits)
    }

    /// Returns the number of basis states minus one.
    ///
    /// This is useful for validation without calculating a potentially
    /// overflowing `2^n`.
    pub fn maximum_basis_index(&self) -> LayoutResult<usize> {
        let dimension = self.basis_dimension()?;
        dimension
            .checked_sub(1)
            .ok_or(LayoutError::ArithmeticOverflow)
    }

    /// Returns the stride of a logical qubit for a conventional dense basis
    /// representation.
    ///
    /// For little-endian layout:
    ///
    /// ```text
    /// q0 -> 1
    /// q1 -> 2
    /// q2 -> 4
    /// ```
    ///
    /// For big-endian layout with three qubits:
    ///
    /// ```text
    /// q0 -> 4
    /// q1 -> 2
    /// q2 -> 1
    /// ```
    ///
    /// This describes basis-index stride, not necessarily byte stride.
    #[inline]
    pub fn qubit_stride(&self, qubit: QubitId) -> LayoutResult<usize> {
        let bit = self.bit_position(qubit)?;
        checked_bit_mask(bit)
    }

    /// Returns the byte stride for a dense element array when the element size
    /// is known.
    pub fn byte_stride(
        &self,
        qubit: QubitId,
        element_size: usize,
    ) -> LayoutResult<usize> {
        let basis_stride = self.qubit_stride(qubit)?;

        basis_stride
            .checked_mul(element_size)
            .ok_or(LayoutError::ArithmeticOverflow)
    }

    /// Validates a logical qubit against this layout.
    #[inline]
    pub fn validate_logical_qubit(
        &self,
        qubit: QubitId,
    ) -> LayoutResult<()> {
        if qubit.index() >= self.num_qubits {
            return Err(LayoutError::LogicalQubitOutOfRange {
                qubit,
                num_qubits: self.num_qubits,
            });
        }

        Ok(())
    }

    /// Validates a storage position against this layout.
    #[inline]
    pub fn validate_storage_position(
        &self,
        position: usize,
    ) -> LayoutResult<()> {
        if position >= self.num_qubits {
            return Err(LayoutError::StoragePositionOutOfRange {
                position,
                num_positions: self.num_qubits,
            });
        }

        Ok(())
    }

    /// Validates a basis-state index against this layout.
    #[inline]
    pub fn validate_basis(&self, basis: usize) -> LayoutResult<()> {
        let maximum = self.maximum_basis_index()?;

        if basis > maximum {
            return Err(LayoutError::BasisValueOutOfRange {
                basis,
                num_qubits: self.num_qubits,
            });
        }

        Ok(())
    }

    /// Creates a layout with a different bit order but identical logical
    /// ordering.
    ///
    /// This is a semantic transformation, not a state transformation.
    pub fn with_bit_ordered_copy(
        &self,
        bit_order: BitOrder,
    ) -> LayoutResult<Self> {
        Self::try_new(
            self.num_qubits,
            bit_order,
            self.storage_layout,
            self.order.clone(),
        )
    }

    /// Creates a layout with a different logical/storage ordering.
    pub fn with_order(
        &self,
        order: QubitOrder,
    ) -> LayoutResult<Self> {
        Self::try_new(
            self.num_qubits,
            self.bit_order,
            StorageLayout::Permuted,
            order,
        )
    }

    /// Returns the inverse mapping:
    ///
    /// ```text
    /// logical qubit -> storage position
    /// ```
    ///
    /// The returned slice is read-only so the layout's invariants cannot be
    /// broken after construction.
    pub fn logical_to_storage(&self) -> &[usize] {
        &self.logical_to_position
    }

    /// Returns the storage-to-logical mapping.
    pub fn storage_to_logical(&self) -> &[QubitId] {
        self.order.as_slice()
    }
}

// =============================================================================
// Dense indexing helpers
// =============================================================================

/// Calculates `2^qubits` with overflow checking.
///
/// This function is intentionally public because `limits.rs`, state-vector
/// memory estimation, and other memory modules need exactly the same checked
/// definition.
pub fn checked_basis_dimension(qubits: usize) -> LayoutResult<usize> {
    validate_qubit_count(qubits)?;

    1usize
        .checked_shl(qubits as u32)
        .ok_or(LayoutError::ArithmeticOverflow)
}

/// Calculates the maximum valid basis index for a qubit count.
pub fn checked_maximum_basis_index(qubits: usize) -> LayoutResult<usize> {
    let dimension = checked_basis_dimension(qubits)?;

    dimension
        .checked_sub(1)
        .ok_or(LayoutError::ArithmeticOverflow)
}

/// Calculates `1 << bit` safely.
pub fn checked_bit_mask(bit: usize) -> LayoutResult<usize> {
    if bit >= usize::BITS as usize {
        return Err(LayoutError::ArithmeticOverflow);
    }

    1usize
        .checked_shl(bit as u32)
        .ok_or(LayoutError::ArithmeticOverflow)
}

/// Validates a qubit count for operations whose basis index is represented by
/// `usize`.
pub fn validate_qubit_count(count: usize) -> LayoutResult<()> {
    if count == 0 {
        // Zero-qubit quantum states are mathematically meaningful in some
        // tensor/composition contexts, but a concrete memory layout with zero
        // qubits has no logical positions. We therefore permit it as a valid
        // empty layout rather than conflating it with an invalid request.
        return Ok(());
    }

    if count > MAX_BASIS_INDEX_QUBITS {
        return Err(LayoutError::QubitCountTooLarge {
            count,
            maximum: MAX_BASIS_INDEX_QUBITS,
        });
    }

    Ok(())
}

/// Returns the maximum number of qubits representable by this layout's
/// `usize` basis-index API.
pub const fn maximum_basis_index_qubits() -> usize {
    MAX_BASIS_INDEX_QUBITS
}

// =============================================================================
// Common constructors
// =============================================================================

/// Creates the canonical little-endian contiguous layout.
pub fn identity_layout(num_qubits: usize) -> LayoutResult<MemoryLayout> {
    MemoryLayout::new(num_qubits)
}

/// Creates the canonical big-endian contiguous layout.
pub fn big_endian_layout(num_qubits: usize) -> LayoutResult<MemoryLayout> {
    MemoryLayout::with_bit_order(num_qubits, BitOrder::BigEndian)
}

/// Creates a layout from an explicit logical ordering.
pub fn permuted_layout(
    num_qubits: usize,
    bit_order: BitOrder,
    order: &[QubitId],
) -> LayoutResult<MemoryLayout> {
    let qubit_order = QubitOrder::try_from_slice(num_qubits, order)?;

    MemoryLayout::from_order(num_qubits, bit_order, qubit_order)
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
    fn identity_layout_is_little_endian_by_default() {
        let layout = MemoryLayout::new(3).expect("valid layout");

        assert_eq!(layout.num_qubits(), 3);
        assert_eq!(layout.bit_order(), BitOrder::LittleEndian);
        assert!(layout.is_identity());

        assert_eq!(layout.bit_position(q(0)).unwrap(), 0);
        assert_eq!(layout.bit_position(q(1)).unwrap(), 1);
        assert_eq!(layout.bit_position(q(2)).unwrap(), 2);

        assert_eq!(layout.qubit_stride(q(0)).unwrap(), 1);
        assert_eq!(layout.qubit_stride(q(1)).unwrap(), 2);
        assert_eq!(layout.qubit_stride(q(2)).unwrap(), 4);
    }

    #[test]
    fn big_endian_reverses_bit_positions() {
        let layout =
            MemoryLayout::with_bit_order(3, BitOrder::BigEndian)
                .expect("valid layout");

        assert_eq!(layout.bit_position(q(0)).unwrap(), 2);
        assert_eq!(layout.bit_position(q(1)).unwrap(), 1);
        assert_eq!(layout.bit_position(q(2)).unwrap(), 0);

        assert_eq!(layout.qubit_stride(q(0)).unwrap(), 4);
        assert_eq!(layout.qubit_stride(q(1)).unwrap(), 2);
        assert_eq!(layout.qubit_stride(q(2)).unwrap(), 1);
    }

    #[test]
    fn logical_order_is_distinct_from_bit_order() {
        let order =
            QubitOrder::try_from_slice(3, &[q(2), q(0), q(1)])
                .expect("valid order");

        let layout =
            MemoryLayout::from_order(3, BitOrder::LittleEndian, order)
                .expect("valid layout");

        assert_eq!(layout.logical_at(0).unwrap(), q(2));
        assert_eq!(layout.logical_at(1).unwrap(), q(0));
        assert_eq!(layout.logical_at(2).unwrap(), q(1));

        assert_eq!(layout.position_of(q(2)).unwrap(), 0);
        assert_eq!(layout.position_of(q(0)).unwrap(), 1);
        assert_eq!(layout.position_of(q(1)).unwrap(), 2);

        // q2 occupies bit position 0 because it is storage position 0.
        assert_eq!(layout.bit_position(q(2)).unwrap(), 0);

        // q0 occupies bit position 1 because it is storage position 1.
        assert_eq!(layout.bit_position(q(0)).unwrap(), 1);

        // q1 occupies bit position 2 because it is storage position 2.
        assert_eq!(layout.bit_position(q(1)).unwrap(), 2);
    }

    #[test]
    fn basis_dimension_is_checked() {
        assert_eq!(checked_basis_dimension(0).unwrap(), 1);
        assert_eq!(checked_basis_dimension(1).unwrap(), 2);
        assert_eq!(checked_basis_dimension(3).unwrap(), 8);
    }

    #[test]
    fn basis_indices_are_encoded_using_logical_qubits() {
        let layout = MemoryLayout::new(3).expect("valid layout");

        let basis = layout
            .basis_index_from_bits(&[true, false, true])
            .expect("valid bits");

        assert_eq!(basis, 5);
    }

    #[test]
    fn basis_indices_respect_custom_order() {
        let layout = permuted_layout(
            3,
            BitOrder::LittleEndian,
            &[q(2), q(0), q(1)],
        )
        .expect("valid layout");

        let basis = layout
            .basis_index_from_bits(&[true, false, false])
            .expect("valid bits");

        // q0 is at storage position 1, therefore bit 1 is set.
        assert_eq!(basis, 2);
    }

    #[test]
    fn basis_indices_respect_big_endian_order() {
        let layout =
            MemoryLayout::with_bit_order(3, BitOrder::BigEndian)
                .expect("valid layout");

        let basis = layout
            .basis_index_from_bits(&[true, false, false])
            .expect("valid bits");

        // q0 is the most-significant bit.
        assert_eq!(basis, 4);
    }

    #[test]
    fn bit_round_trip_is_correct() {
        let layout = MemoryLayout::new(4).expect("valid layout");

        for basis in 0..16 {
            for index in 0..4 {
                let qubit = q(index);

                let bit = layout.bit_value(basis, qubit).unwrap();
                let rebuilt = layout.with_bit(basis, qubit, bit).unwrap();

                assert_eq!(rebuilt, basis);
            }
        }
    }

    #[test]
    fn flip_bit_is_involutive() {
        let layout = MemoryLayout::new(5).expect("valid layout");

        for basis in 0..32 {
            for index in 0..5 {
                let qubit = q(index);

                let flipped = layout.flip_bit(basis, qubit).unwrap();
                let restored = layout.flip_bit(flipped, qubit).unwrap();

                assert_eq!(restored, basis);
            }
        }
    }

    #[test]
    fn bits_from_basis_round_trip() {
        let layout = MemoryLayout::new(5).expect("valid layout");

        for basis in 0..32 {
            let bits = layout.bits_from_basis(basis).unwrap();
            let rebuilt = layout.basis_index_from_bits(&bits).unwrap();

            assert_eq!(rebuilt, basis);
        }
    }

    #[test]
    fn invalid_logical_qubit_is_rejected() {
        let layout = MemoryLayout::new(3).expect("valid layout");

        let error = layout.position_of(q(3)).unwrap_err();

        assert_eq!(
            error,
            LayoutError::LogicalQubitOutOfRange {
                qubit: q(3),
                num_qubits: 3,
            }
        );
    }

    #[test]
    fn invalid_storage_position_is_rejected() {
        let layout = MemoryLayout::new(3).expect("valid layout");

        let error = layout.logical_at(3).unwrap_err();

        assert_eq!(
            error,
            LayoutError::StoragePositionOutOfRange {
                position: 3,
                num_positions: 3,
            }
        );
    }

    #[test]
    fn invalid_basis_is_rejected() {
        let layout = MemoryLayout::new(3).expect("valid layout");

        let error = layout.validate_basis(8).unwrap_err();

        assert_eq!(
            error,
            LayoutError::BasisValueOutOfRange {
                basis: 8,
                num_qubits: 3,
            }
        );
    }

    #[test]
    fn duplicate_qubits_are_rejected() {
        let error =
            QubitOrder::try_from_slice(3, &[q(0), q(1), q(1)])
                .unwrap_err();

        assert_eq!(
            error,
            LayoutError::DuplicateLogicalQubit { qubit: q(1) }
        );
    }

    #[test]
    fn out_of_range_qubits_are_rejected() {
        let error =
            QubitOrder::try_from_slice(3, &[q(0), q(1), q(3)])
                .unwrap_err();

        assert_eq!(
            error,
            LayoutError::InvalidLogicalQubit { qubit: q(3) }
        );
    }

    #[test]
    fn wrong_order_length_is_rejected() {
        let error =
            QubitOrder::try_from_slice(3, &[q(0), q(1)])
                .unwrap_err();

        assert_eq!(
            error,
            LayoutError::InvalidOrderLength {
                expected: 3,
                actual: 2,
            }
        );
    }

    #[test]
    fn reversed_order_is_valid() {
        let order = QubitOrder::reversed(4).expect("valid order");

        assert_eq!(order.as_slice(), &[q(3), q(2), q(1), q(0)]);
    }

    #[test]
    fn order_permutation_is_checked() {
        let order =
            QubitOrder::try_from_slice(3, &[q(0), q(1), q(2)])
                .expect("valid order");

        let permuted = order.permute(&[2, 0, 1]).expect("valid permutation");

        assert_eq!(permuted.as_slice(), &[q(2), q(0), q(1)]);
    }

    #[test]
    fn invalid_permutation_is_rejected() {
        let order =
            QubitOrder::try_from_slice(3, &[q(0), q(1), q(2)])
                .expect("valid order");

        let error = order.permute(&[0, 0, 1]).unwrap_err();

        assert_eq!(
            error,
            LayoutError::DuplicatePhysicalPosition { position: 0 }
        );
    }

    #[test]
    fn byte_stride_is_checked() {
        let layout = MemoryLayout::new(4).expect("valid layout");

        assert_eq!(layout.byte_stride(q(0), 16).unwrap(), 16);
        assert_eq!(layout.byte_stride(q(1), 16).unwrap(), 32);
        assert_eq!(layout.byte_stride(q(2), 16).unwrap(), 64);
        assert_eq!(layout.byte_stride(q(3), 16).unwrap(), 128);
    }

    #[test]
    fn zero_qubit_layout_is_valid_and_empty() {
        let layout = MemoryLayout::new(0).expect("valid empty layout");

        assert_eq!(layout.num_qubits(), 0);
        assert!(layout.is_identity());
        assert_eq!(layout.basis_dimension().unwrap(), 1);
        assert_eq!(layout.maximum_basis_index().unwrap(), 0);
        assert!(layout.qubit_order().is_empty());
    }

    #[test]
    fn zero_qubit_order_is_valid() {
        let order = QubitOrder::identity(0).expect("valid empty order");

        assert!(order.is_empty());
        assert!(order.is_identity());
    }

    #[test]
    fn serde_round_trip_preserves_layout() {
        let layout = permuted_layout(
            4,
            BitOrder::BigEndian,
            &[q(2), q(0), q(3), q(1)],
        )
        .expect("valid layout");

        let encoded =
            serde_json::to_string(&layout).expect("serialization succeeds");

        let decoded: MemoryLayout =
            serde_json::from_str(&encoded).expect("deserialization succeeds");

        assert_eq!(decoded, layout);
    }

    #[test]
    fn bit_mask_is_checked() {
        assert_eq!(checked_bit_mask(0).unwrap(), 1);
        assert_eq!(checked_bit_mask(1).unwrap(), 2);
        assert_eq!(checked_bit_mask(3).unwrap(), 8);

        assert_eq!(
            checked_bit_mask(usize::BITS as usize).unwrap_err(),
            LayoutError::ArithmeticOverflow
        );
    }

    #[test]
    fn basis_dimension_boundary_is_checked() {
        let maximum = maximum_basis_index_qubits();

        assert!(checked_basis_dimension(maximum).is_ok());

        if maximum < usize::MAX {
            assert_eq!(
                checked_basis_dimension(maximum + 1).unwrap_err(),
                LayoutError::QubitCountTooLarge {
                    count: maximum + 1,
                    maximum,
                }
            );
        }
    }

    #[test]
    fn changed_bit_value_preserves_other_bits() {
        let layout = MemoryLayout::new(6).expect("valid layout");

        let original = 0b101011usize;

        let cleared = layout.with_bit(original, q(3), false).unwrap();

        assert_eq!(cleared, 0b100011);

        let set = layout.with_bit(cleared, q(4), true).unwrap();

        assert_eq!(set, 0b110011);
    }

    #[test]
    fn inverse_mapping_is_consistent() {
        let layout = permuted_layout(
            5,
            BitOrder::LittleEndian,
            &[q(3), q(0), q(4), q(1), q(2)],
        )
        .expect("valid layout");

        for position in 0..5 {
            let logical = layout.logical_at(position).unwrap();
            assert_eq!(layout.position_of(logical).unwrap(), position);
        }
    }

    #[test]
    fn changing_bit_order_does_not_change_logical_order() {
        let original = permuted_layout(
            4,
            BitOrder::LittleEndian,
            &[q(2), q(0), q(3), q(1)],
        )
        .expect("valid layout");

        let changed = original
            .with_bit_ordered_copy(BitOrder::BigEndian)
            .expect("valid layout");

        assert_eq!(
            original.storage_to_logical(),
            changed.storage_to_logical()
        );
    }
}