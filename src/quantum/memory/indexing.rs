//! Zamani Quantum Memory — Safe Indexing
//!
//! Production-grade, overflow-checked indexing primitives for the quantum
//! memory subsystem.
//!
//! # Purpose
//!
//! This module is the single authority for converting between:
//!
//! - logical qubit indices;
//! - basis-state indices;
//! - amplitude indices;
//! - qubit bit masks;
//! - qubit strides;
//! - multi-qubit basis coordinates;
//! - tensor coordinates;
//! - linear tensor offsets.
//!
//! It deliberately contains **no quantum-state representation** and does not
//! allocate quantum memory.
//!
//! # Architectural boundary
//!
//! ```text
//! quantum::ir::QubitId
//!          │
//!          ▼
//! quantum::memory::indexing
//!          │
//!     ┌────┼─────────┐
//!     ▼    ▼         ▼
//! state_vector   sparse   tensor_network
//!     │
//!     ▼
//! density_matrix / stabilizer / GPU / distributed
//! ```
//!
//! The module does not own:
//!
//! - quantum state storage;
//! - state-vector amplitudes;
//! - density matrices;
//! - tensor-network storage;
//! - routing;
//! - scheduling;
//! - hardware topology;
//! - GPU APIs;
//! - distributed communication;
//! - serialization.
//!
//! Those systems consume the checked indexing primitives defined here.
//!
//! # Critical safety rule
//!
//! Quantum indexing frequently involves expressions such as:
//!
//! ```text
//! 1 << qubit
//! 2^qubits
//! qubit * stride
//! row * dimension + column
//! ```
//!
//! These operations are dangerous when performed without overflow checking.
//! In particular, a state vector has `2^n` amplitudes and therefore grows
//! exponentially with the number of qubits.
//!
//! **No unchecked shift or multiplication is permitted in this module.**
//!
//! # Logical qubit convention
//!
//! This module treats a basis-state index as a bit field:
//!
//! ```text
//! basis index = b_(n-1) ... b_2 b_1 b_0
//! ```
//!
//! By default, qubit `q` corresponds to bit `q` (little-endian bit indexing).
//! Endianness transformations belong to `memory::layout` and can use these
//! primitives without changing their semantics.
//!
//! Example for three qubits:
//!
//! ```text
//! q2 q1 q0
//!  1  0  1  => basis index 5
//! ```
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No nightly features.
//! No external dependencies.
//! No unsafe code.
//!
//! # Integration contract
//!
//! Later memory modules should use this module rather than implementing their
//! own bit shifts, stride calculations, or basis-index arithmetic.
//!
//! Specifically:
//!
//! - `state_vector.rs` uses `basis_state_count`, `amplitude_index`,
//!   `qubit_mask`, `qubit_stride`, `toggle_bit`, and `replace_bit`.
//! - `density_matrix.rs` uses `matrix_dimension`, `matrix_index`, and
//!   `basis_state_count`.
//! - `stabilizer.rs` uses checked qubit-bit calculations.
//! - `sparse.rs` uses basis-state validation and bit operations.
//! - `tensor.rs` and `tensor_network.rs` use `TensorShape`,
//!   `TensorStrides`, and `tensor_offset`.
//! - `permutation.rs` uses `validate_permutation`.
//! - `layout.rs` can build higher-level endian/stride mappings on top of this
//!   module.
//! - `limits.rs` can use `required_bits_for_count` and the checked resource
//!   functions before allocation.
//!
//! The module intentionally uses `usize` at its machine-indexing boundary.
//! Conversion from the canonical `quantum::ir::QubitId` can be performed by
//! callers through `QubitId::index()` without making this low-level module
//! depend on `quantum::ir` internals.

use std::fmt;

// =============================================================================
// Constants
// =============================================================================

/// Number of bits available in a machine `usize`.
const USIZE_BITS: u32 = usize::BITS;

/// Maximum qubit index that can be represented as one bit in a `usize`.
///
/// A qubit mask requires `1 << qubit`. Therefore the highest representable
/// qubit index is `usize::BITS - 1`.
const MAX_MASK_QUBIT_INDEX: usize = (USIZE_BITS - 1) as usize;

// =============================================================================
// Errors
// =============================================================================

/// Result type used by all checked indexing operations.
pub type IndexResult<T> = Result<T, IndexingError>;

/// Errors produced by checked quantum-memory indexing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexingError {
    /// A qubit index is outside the representable machine-bit range.
    QubitIndexOutOfRange {
        qubit: usize,
        maximum: usize,
    },

    /// A requested number of qubits cannot be represented as an indexable
    /// basis-state space on the current target.
    QubitCountTooLarge {
        qubits: usize,
        maximum: usize,
    },

    /// A basis-state index is outside `[0, 2^qubits)`.
    BasisIndexOutOfRange {
        index: usize,
        qubits: usize,
        dimension: Option<usize>,
    },

    /// A requested amplitude index is invalid.
    AmplitudeIndexOutOfRange {
        index: usize,
        amplitudes: usize,
    },

    /// A stride cannot be represented.
    StrideOverflow {
        qubits: usize,
        stride_qubits: usize,
    },

    /// An arithmetic operation overflowed.
    ArithmeticOverflow {
        operation: &'static str,
    },

    /// A tensor has no dimensions when an operation requires at least one.
    EmptyTensorShape,

    /// A tensor dimension is zero.
    ZeroTensorDimension {
        axis: usize,
    },

    /// A tensor coordinate does not belong to its corresponding dimension.
    TensorCoordinateOutOfRange {
        axis: usize,
        coordinate: usize,
        dimension: usize,
    },

    /// The number of tensor dimensions and coordinates differ.
    TensorRankMismatch {
        rank: usize,
        coordinates: usize,
    },

    /// The number of dimensions and supplied strides differ.
    TensorStrideRankMismatch {
        rank: usize,
        strides: usize,
    },

    /// Tensor shape multiplication exceeds the representable index space.
    TensorSizeOverflow {
        operation: &'static str,
    },

    /// A permutation does not contain exactly one occurrence of every axis.
    InvalidPermutation {
        axis: usize,
        rank: usize,
    },

    /// A permutation contains a duplicate axis.
    DuplicatePermutationAxis {
        axis: usize,
    },

    /// A requested range is invalid.
    InvalidRange {
        start: usize,
        end: usize,
        upper_bound: usize,
    },
}

impl fmt::Display for IndexingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QubitIndexOutOfRange { qubit, maximum } => write!(
                f,
                "qubit index {qubit} is outside the representable range 0..={maximum}"
            ),

            Self::QubitCountTooLarge { qubits, maximum } => write!(
                f,
                "qubit count {qubits} is too large for the current index space; maximum is {maximum}"
            ),

            Self::BasisIndexOutOfRange {
                index,
                qubits,
                dimension,
            } => {
                if let Some(dimension) = dimension {
                    write!(
                        f,
                        "basis index {index} is outside 0..{dimension} for {qubits} qubits"
                    )
                } else {
                    write!(
                        f,
                        "basis index {index} is not representable for {qubits} qubits"
                    )
                }
            }

            Self::AmplitudeIndexOutOfRange { index, amplitudes } => write!(
                f,
                "amplitude index {index} is outside 0..{amplitudes}"
            ),

            Self::StrideOverflow {
                qubits,
                stride_qubits,
            } => write!(
                f,
                "stride for qubit {stride_qubits} cannot be represented for {qubits} qubits"
            ),

            Self::ArithmeticOverflow { operation } => {
                write!(f, "integer overflow while performing {operation}")
            }

            Self::EmptyTensorShape => {
                write!(f, "tensor shape must contain at least one dimension")
            }

            Self::ZeroTensorDimension { axis } => {
                write!(f, "tensor dimension at axis {axis} must be non-zero")
            }

            Self::TensorCoordinateOutOfRange {
                axis,
                coordinate,
                dimension,
            } => write!(
                f,
                "tensor coordinate {coordinate} at axis {axis} is outside 0..{dimension}"
            ),

            Self::TensorRankMismatch { rank, coordinates } => write!(
                f,
                "tensor rank {rank} does not match coordinate count {coordinates}"
            ),

            Self::TensorStrideRankMismatch { rank, strides } => write!(
                f,
                "tensor rank {rank} does not match stride count {strides}"
            ),

            Self::TensorSizeOverflow { operation } => {
                write!(f, "tensor size overflow while performing {operation}")
            }

            Self::InvalidPermutation { axis, rank } => write!(
                f,
                "permutation axis {axis} is outside 0..{rank}"
            ),

            Self::DuplicatePermutationAxis { axis } => {
                write!(f, "permutation contains duplicate axis {axis}")
            }

            Self::InvalidRange {
                start,
                end,
                upper_bound,
            } => write!(
                f,
                "invalid range {start}..{end}; valid upper bound is {upper_bound}"
            ),
        }
    }
}

impl std::error::Error for IndexingError {}

// =============================================================================
// Fundamental representability functions
// =============================================================================

/// Returns the largest qubit index that can be represented by a `usize` bit
/// mask on this target.
///
/// This does **not** mean a state vector containing that many qubits is
/// practical. Resource limits must be enforced separately by
/// `memory::limits`.
pub const fn maximum_mask_qubit_index() -> usize {
    MAX_MASK_QUBIT_INDEX
}

/// Returns the maximum number of qubits for which `2^n` can be represented as
/// a `usize`.
///
/// Because `2^usize::BITS` cannot be represented in `usize`, the maximum
/// representable state-space exponent is `usize::BITS - 1`.
pub const fn maximum_indexable_qubit_count() -> usize {
    MAX_MASK_QUBIT_INDEX
}

/// Returns the number of bits required to represent `count - 1`.
///
/// This is useful for determining the minimum number of qubits required to
/// address `count` basis states.
///
/// # Examples
///
/// ```
/// use zamani::quantum::memory::indexing::required_bits_for_count;
///
/// assert_eq!(required_bits_for_count(1), 0);
/// assert_eq!(required_bits_for_count(2), 1);
/// assert_eq!(required_bits_for_count(4), 2);
/// assert_eq!(required_bits_for_count(5), 3);
/// ```
pub const fn required_bits_for_count(count: usize) -> usize {
    if count <= 1 {
        0
    } else {
        (usize::BITS - (count - 1).leading_zeros()) as usize
    }
}

/// Returns `2^qubits` if the result is representable as a `usize`.
///
/// This is the canonical checked basis-space dimension calculation.
pub const fn basis_state_count(qubits: usize) -> IndexResult<usize> {
    if qubits > maximum_indexable_qubit_count() {
        return Err(IndexingError::QubitCountTooLarge {
            qubits,
            maximum: maximum_indexable_qubit_count(),
        });
    }

    Ok(1usize << qubits)
}

/// Returns the number of basis states for `qubits`, validating that the
/// requested count is indexable.
///
/// This function is an explicit alias for [`basis_state_count`] intended for
/// callers whose terminology is "dimension".
pub const fn basis_dimension(qubits: usize) -> IndexResult<usize> {
    basis_state_count(qubits)
}

/// Validates a qubit index against the machine bit representation.
pub const fn validate_qubit_index(qubit: usize) -> IndexResult<()> {
    if qubit > maximum_mask_qubit_index() {
        Err(IndexingError::QubitIndexOutOfRange {
            qubit,
            maximum: maximum_mask_qubit_index(),
        })
    } else {
        Ok(())
    }
}

/// Validates a qubit index against a specific register size.
pub const fn validate_qubit_in_register(
    qubit: usize,
    qubits: usize,
) -> IndexResult<()> {
    if qubit >= qubits {
        return Err(IndexingError::QubitIndexOutOfRange {
            qubit,
            maximum: qubits.saturating_sub(1),
        });
    }

    validate_qubit_index(qubit)
}

// =============================================================================
// Bit masks
// =============================================================================

/// Returns the bit mask corresponding to a qubit.
///
/// ```text
/// qubit 0 -> 0001
/// qubit 1 -> 0010
/// qubit 2 -> 0100
/// qubit 3 -> 1000
/// ```
pub const fn qubit_mask(qubit: usize) -> IndexResult<usize> {
    match validate_qubit_index(qubit) {
        Ok(()) => Ok(1usize << qubit),
        Err(error) => Err(error),
    }
}

/// Returns the mask for a set of qubits.
///
/// Duplicate qubit indices are harmless because the resulting mask represents
/// a set, but callers that need uniqueness should use their own semantic
/// validation or `validate_unique_qubits`.
pub fn qubit_mask_set(qubits: &[usize]) -> IndexResult<usize> {
    let mut mask = 0usize;

    for &qubit in qubits {
        mask |= qubit_mask(qubit)?;
    }

    Ok(mask)
}

/// Returns whether a particular qubit bit is set in a basis-state index.
pub const fn is_bit_set(basis_index: usize, qubit: usize) -> IndexResult<bool> {
    let mask = qubit_mask(qubit)?;
    Ok((basis_index & mask) != 0)
}

/// Returns a basis-state index with the selected qubit set to `1`.
pub const fn set_bit(basis_index: usize, qubit: usize) -> IndexResult<usize> {
    let mask = qubit_mask(qubit)?;
    Ok(basis_index | mask)
}

/// Returns a basis-state index with the selected qubit set to `0`.
pub const fn clear_bit(
    basis_index: usize,
    qubit: usize,
) -> IndexResult<usize> {
    let mask = qubit_mask(qubit)?;
    Ok(basis_index & !mask)
}

/// Returns a basis-state index with the selected qubit toggled.
pub const fn toggle_bit(
    basis_index: usize,
    qubit: usize,
) -> IndexResult<usize> {
    let mask = qubit_mask(qubit)?;
    Ok(basis_index ^ mask)
}

/// Returns a basis-state index whose selected qubit has been replaced with
/// `bit`.
///
/// `bit` must be `0` or `1`.
pub const fn replace_bit(
    basis_index: usize,
    qubit: usize,
    bit: bool,
) -> IndexResult<usize> {
    if bit {
        set_bit(basis_index, qubit)
    } else {
        clear_bit(basis_index, qubit)
    }
}

// =============================================================================
// Basis-state indexing
// =============================================================================

/// Validates a basis-state index for a given number of qubits.
pub const fn validate_basis_index(
    basis_index: usize,
    qubits: usize,
) -> IndexResult<()> {
    let dimension = match basis_state_count(qubits) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    if basis_index >= dimension {
        return Err(IndexingError::BasisIndexOutOfRange {
            index: basis_index,
            qubits,
            dimension: Some(dimension),
        });
    }

    Ok(())
}

/// Returns the amplitude index corresponding to a basis state.
///
/// For a dense state vector, the basis-state index and amplitude index are
/// numerically identical. This function exists to make that relationship
/// explicit and to provide a single validation boundary.
///
/// The function deliberately does not return a raw unchecked index.
pub const fn amplitude_index(
    basis_index: usize,
    qubits: usize,
) -> IndexResult<usize> {
    match validate_basis_index(basis_index, qubits) {
        Ok(()) => Ok(basis_index),
        Err(error) => Err(error),
    }
}

/// Validates a dense state-vector amplitude index.
///
/// `amplitudes` must be the actual allocated logical length of the amplitude
/// buffer.
pub const fn validate_amplitude_index(
    index: usize,
    amplitudes: usize,
) -> IndexResult<()> {
    if index >= amplitudes {
        Err(IndexingError::AmplitudeIndexOutOfRange {
            index,
            amplitudes,
        })
    } else {
        Ok(())
    }
}

/// Returns the stride associated with a qubit in a standard little-endian
/// dense state-vector layout.
///
/// ```text
/// stride(q) = 2^q
/// ```
///
/// For example:
///
/// ```text
/// q0 -> 1
/// q1 -> 2
/// q2 -> 4
/// q3 -> 8
/// ```
pub const fn qubit_stride(qubit: usize) -> IndexResult<usize> {
    match validate_qubit_index(qubit) {
        Ok(()) => Ok(1usize << qubit),
        Err(error) => Err(error),
    }
}

/// Returns the paired amplitude index obtained by flipping `qubit`.
///
/// This is useful for single-qubit gate kernels.
///
/// Example:
///
/// ```text
/// index = 0b0101
/// q1
///      ↓
/// index = 0b0111
/// ```
pub const fn paired_index(
    basis_index: usize,
    qubit: usize,
    qubits: usize,
) -> IndexResult<usize> {
    validate_basis_index(basis_index, qubits)?;
    validate_qubit_in_register(qubit, qubits)?;
    toggle_bit(basis_index, qubit)
}

/// Returns the lower member of a qubit pair.
///
/// For a single-qubit operation, every amplitude belongs to a pair whose
/// members differ only in the target qubit. This returns the member where the
/// target bit is zero.
pub const fn lower_pair_index(
    basis_index: usize,
    qubit: usize,
    qubits: usize,
) -> IndexResult<usize> {
    validate_basis_index(basis_index, qubits)?;
    validate_qubit_in_register(qubit, qubits)?;
    clear_bit(basis_index, qubit)
}

/// Returns the upper member of a qubit pair.
///
/// This returns the member where the target qubit bit is one.
pub const fn upper_pair_index(
    basis_index: usize,
    qubit: usize,
    qubits: usize,
) -> IndexResult<usize> {
    validate_basis_index(basis_index, qubits)?;
    validate_qubit_in_register(qubit, qubits)?;
    set_bit(basis_index, qubit)
}

/// Returns the contiguous block containing `basis_index` for a target qubit.
///
/// In a standard little-endian dense layout, a target qubit partitions the
/// state vector into alternating blocks of length `2^qubit`.
///
/// The returned tuple is:
///
/// ```text
/// (block_start, offset_inside_block)
/// ```
pub fn qubit_block_position(
    basis_index: usize,
    qubit: usize,
    qubits: usize,
) -> IndexResult<(usize, usize)> {
    validate_basis_index(basis_index, qubits)?;
    validate_qubit_in_register(qubit, qubits)?;

    let stride = qubit_stride(qubit)?;
    let block_width = stride
        .checked_mul(2)
        .ok_or(IndexingError::ArithmeticOverflow {
            operation: "qubit block width",
        })?;

    let block_start = (basis_index / block_width)
        .checked_mul(block_width)
        .ok_or(IndexingError::ArithmeticOverflow {
            operation: "qubit block start",
        })?;

    let offset = basis_index
        .checked_sub(block_start)
        .ok_or(IndexingError::ArithmeticOverflow {
            operation: "qubit block offset",
        })?;

    Ok((block_start, offset))
}

// =============================================================================
// Multi-qubit indexing
// =============================================================================

/// Validates that all qubit indices are unique and representable.
///
/// The function is allocation-free and deterministic.
///
/// It is intended for small operation arities such as one-, two-, three-, or
/// four-qubit operations. It does not impose an arbitrary arity limit.
pub fn validate_unique_qubits(qubits: &[usize]) -> IndexResult<()> {
    for (position, &qubit) in qubits.iter().enumerate() {
        validate_qubit_index(qubit)?;

        for &previous in &qubits[..position] {
            if previous == qubit {
                return Err(IndexingError::ArithmeticOverflow {
                    operation: "duplicate qubit index",
                });
            }
        }
    }

    Ok(())
}

/// Returns the combined mask for a unique set of qubits.
///
/// Duplicate qubits are rejected.
pub fn unique_qubit_mask(qubits: &[usize]) -> IndexResult<usize> {
    validate_unique_qubits(qubits)?;
    qubit_mask_set(qubits)
}

/// Extracts selected qubit bits from a basis-state index and packs them into
/// a compact sub-index.
///
/// The order of `qubits` defines the order of bits in the result.
///
/// Example:
///
/// ```text
/// basis = q3 q2 q1 q0
///          1  0  1  1
///
/// qubits = [1, 3]
///
/// result = q1 q3
///          1  1
///        = 3
/// ```
pub fn extract_bits(
    basis_index: usize,
    qubits: &[usize],
    register_qubits: usize,
) -> IndexResult<usize> {
    validate_basis_index(basis_index, register_qubits)?;
    validate_unique_qubits(qubits)?;

    let mut result = 0usize;

    for (packed_position, &qubit) in qubits.iter().enumerate() {
        validate_qubit_in_register(qubit, register_qubits)?;

        if is_bit_set(basis_index, qubit)? {
            let packed_mask = qubit_mask(packed_position)?;

            result |= packed_mask;
        }
    }

    Ok(result)
}

/// Replaces the selected qubits in a basis index with bits from a packed
/// sub-index.
///
/// The order of `qubits` defines the order of bits in `packed_bits`.
///
/// Bits outside the selected range in `packed_bits` are rejected.
pub fn insert_bits(
    basis_index: usize,
    qubits: &[usize],
    packed_bits: usize,
    register_qubits: usize,
) -> IndexResult<usize> {
    validate_basis_index(basis_index, register_qubits)?;
    validate_unique_qubits(qubits)?;

    let packed_dimension = basis_state_count(qubits.len())?;

    if packed_bits >= packed_dimension {
        return Err(IndexingError::BasisIndexOutOfRange {
            index: packed_bits,
            qubits: qubits.len(),
            dimension: Some(packed_dimension),
        });
    }

    let mut result = basis_index;

    for (packed_position, &qubit) in qubits.iter().enumerate() {
        validate_qubit_in_register(qubit, register_qubits)?;

        let bit = is_bit_set(packed_bits, packed_position)?;

        result = replace_bit(result, qubit, bit)?;
    }

    Ok(result)
}

/// Returns the basis index produced by setting selected qubits to the supplied
/// packed bit pattern.
pub fn with_selected_bits(
    basis_index: usize,
    qubits: &[usize],
    packed_bits: usize,
    register_qubits: usize,
) -> IndexResult<usize> {
    insert_bits(
        basis_index,
        qubits,
        packed_bits,
        register_qubits,
    )
}

/// Returns the complement of selected qubit bits while preserving all other
/// bits.
pub fn toggle_selected_bits(
    basis_index: usize,
    qubits: &[usize],
    register_qubits: usize,
) -> IndexResult<usize> {
    validate_basis_index(basis_index, register_qubits)?;
    validate_unique_qubits(qubits)?;

    let mask = unique_qubit_mask(qubits)?;

    Ok(basis_index ^ mask)
}

// =============================================================================
// Matrix indexing
// =============================================================================

/// Returns the dimension of a square density matrix for `qubits`.
///
/// A density matrix has dimension `2^n × 2^n`.
pub const fn matrix_dimension(qubits: usize) -> IndexResult<usize> {
    basis_state_count(qubits)
}

/// Returns the total number of scalar entries in a square density matrix.
///
/// The result is `4^n`.
///
/// This function exists specifically so callers can perform checked resource
/// estimation before allocating a density matrix.
pub const fn matrix_element_count(qubits: usize) -> IndexResult<usize> {
    let dimension = match matrix_dimension(qubits) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    match dimension.checked_mul(dimension) {
        Some(value) => Ok(value),
        None => Err(IndexingError::ArithmeticOverflow {
            operation: "density-matrix element count",
        }),
    }
}

/// Returns a row-major matrix index for `(row, column)`.
///
/// The matrix dimension is `2^qubits`.
///
/// ```text
/// index = row * dimension + column
/// ```
pub const fn matrix_index(
    row: usize,
    column: usize,
    qubits: usize,
) -> IndexResult<usize> {
    let dimension = match matrix_dimension(qubits) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    if row >= dimension {
        return Err(IndexingError::BasisIndexOutOfRange {
            index: row,
            qubits,
            dimension: Some(dimension),
        });
    }

    if column >= dimension {
        return Err(IndexingError::BasisIndexOutOfRange {
            index: column,
            qubits,
            dimension: Some(dimension),
        });
    }

    let base = match row.checked_mul(dimension) {
        Some(value) => value,
        None => {
            return Err(IndexingError::ArithmeticOverflow {
                operation: "matrix row offset",
            })
        }
    };

    match base.checked_add(column) {
        Some(value) => Ok(value),
        None => Err(IndexingError::ArithmeticOverflow {
            operation: "matrix linear index",
        }),
    }
}

/// Decomposes a row-major square matrix index into `(row, column)`.
pub const fn matrix_coordinates(
    index: usize,
    qubits: usize,
) -> IndexResult<(usize, usize)> {
    let dimension = matrix_dimension(qubits)?;

    let elements = dimension.checked_mul(dimension).ok_or(
        IndexingError::ArithmeticOverflow {
            operation: "matrix element count",
        },
    )?;

    if index >= elements {
        return Err(IndexingError::AmplitudeIndexOutOfRange {
            index,
            amplitudes: elements,
        });
    }

    Ok((index / dimension, index % dimension))
}

// =============================================================================
// Tensor shape
// =============================================================================

/// Immutable tensor shape.
///
/// A tensor shape is represented by its dimensions in logical axis order.
///
/// The type does not own tensor data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TensorShape {
    dimensions: Vec<usize>,
}

impl TensorShape {
    /// Creates and validates a tensor shape.
    ///
    /// Empty shapes are rejected because this module is intended for quantum
    /// memory tensors where rank is explicit.
    pub fn new(dimensions: Vec<usize>) -> IndexResult<Self> {
        if dimensions.is_empty() {
            return Err(IndexingError::EmptyTensorShape);
        }

        for (axis, &dimension) in dimensions.iter().enumerate() {
            if dimension == 0 {
                return Err(IndexingError::ZeroTensorDimension { axis });
            }
        }

        Self::checked_size(&dimensions)?;

        Ok(Self { dimensions })
    }

    /// Creates a tensor shape from a slice.
    pub fn from_slice(dimensions: &[usize]) -> IndexResult<Self> {
        Self::new(dimensions.to_vec())
    }

    /// Returns the tensor rank.
    pub fn rank(&self) -> usize {
        self.dimensions.len()
    }

    /// Returns the dimensions.
    pub fn dimensions(&self) -> &[usize] {
        &self.dimensions
    }

    /// Returns one dimension.
    pub fn dimension(&self, axis: usize) -> Option<usize> {
        self.dimensions.get(axis).copied()
    }

    /// Returns the total number of tensor elements.
    pub fn size(&self) -> IndexResult<usize> {
        Self::checked_size(&self.dimensions)
    }

    /// Returns whether the tensor contains exactly one element.
    pub fn is_scalar_sized(&self) -> bool {
        self.dimensions.iter().all(|&dimension| dimension == 1)
    }

    /// Validates a coordinate against this shape.
    pub fn validate_coordinates(
        &self,
        coordinates: &[usize],
    ) -> IndexResult<()> {
        if coordinates.len() != self.rank() {
            return Err(IndexingError::TensorRankMismatch {
                rank: self.rank(),
                coordinates: coordinates.len(),
            });
        }

        for (axis, (&coordinate, &dimension)) in coordinates
            .iter()
            .zip(self.dimensions.iter())
            .enumerate()
        {
            if coordinate >= dimension {
                return Err(IndexingError::TensorCoordinateOutOfRange {
                    axis,
                    coordinate,
                    dimension,
                });
            }
        }

        Ok(())
    }

    fn checked_size(dimensions: &[usize]) -> IndexResult<usize> {
        let mut size = 1usize;

        for &dimension in dimensions {
            size = size.checked_mul(dimension).ok_or(
                IndexingError::TensorSizeOverflow {
                    operation: "tensor element count",
                },
            )?;
        }

        Ok(size)
    }
}

// =============================================================================
// Tensor strides
// =============================================================================

/// Row-major tensor strides.
///
/// For shape `[d0, d1, d2]`, the strides are:
///
/// ```text
/// [d1*d2, d2, 1]
/// ```
///
/// The type owns no tensor storage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TensorStrides {
    strides: Vec<usize>,
}

impl TensorStrides {
    /// Computes row-major strides for a validated shape.
    pub fn from_shape(shape: &TensorShape) -> IndexResult<Self> {
        let rank = shape.rank();
        let mut strides = vec![0usize; rank];

        let mut stride = 1usize;

        for axis in (0..rank).rev() {
            strides[axis] = stride;

            stride = stride.checked_mul(shape.dimensions()[axis]).ok_or(
                IndexingError::TensorSizeOverflow {
                    operation: "tensor stride calculation",
                },
            )?;
        }

        Ok(Self { strides })
    }

    /// Creates validated explicit strides.
    pub fn new(
        shape: &TensorShape,
        strides: Vec<usize>,
    ) -> IndexResult<Self> {
        if strides.len() != shape.rank() {
            return Err(IndexingError::TensorStrideRankMismatch {
                rank: shape.rank(),
                strides: strides.len(),
            });
        }

        Ok(Self { strides })
    }

    /// Returns the rank.
    pub fn rank(&self) -> usize {
        self.strides.len()
    }

    /// Returns the strides.
    pub fn as_slice(&self) -> &[usize] {
        &self.strides
    }

    /// Returns one stride.
    pub fn stride(&self, axis: usize) -> Option<usize> {
        self.strides.get(axis).copied()
    }
}

/// Calculates a row-major tensor offset from coordinates and strides.
pub fn tensor_offset(
    shape: &TensorShape,
    strides: &TensorStrides,
    coordinates: &[usize],
) -> IndexResult<usize> {
    if shape.rank() != strides.rank() {
        return Err(IndexingError::TensorStrideRankMismatch {
            rank: shape.rank(),
            strides: strides.rank(),
        });
    }

    shape.validate_coordinates(coordinates)?;

    let mut offset = 0usize;

    for axis in 0..shape.rank() {
        let contribution = coordinates[axis]
            .checked_mul(strides.as_slice()[axis])
            .ok_or(IndexingError::ArithmeticOverflow {
                operation: "tensor coordinate contribution",
            })?;

        offset = offset
            .checked_add(contribution)
            .ok_or(IndexingError::ArithmeticOverflow {
                operation: "tensor linear offset",
            })?;
    }

    let size = shape.size()?;

    if offset >= size {
        return Err(IndexingError::ArithmeticOverflow {
            operation: "tensor offset validation",
        });
    }

    Ok(offset)
}

/// Calculates canonical row-major strides directly from dimensions.
pub fn tensor_strides(
    dimensions: &[usize],
) -> IndexResult<Vec<usize>> {
    let shape = TensorShape::from_slice(dimensions)?;
    Ok(TensorStrides::from_shape(&shape)?
        .as_slice()
        .to_vec())
}

// =============================================================================
// Tensor coordinate conversion
// =============================================================================

/// Converts a row-major tensor offset into coordinates.
pub fn tensor_coordinates(
    shape: &TensorShape,
    index: usize,
) -> IndexResult<Vec<usize>> {
    let size = shape.size()?;

    if index >= size {
        return Err(IndexingError::AmplitudeIndexOutOfRange {
            index,
            amplitudes: size,
        });
    }

    let strides = TensorStrides::from_shape(shape)?;

    let mut remaining = index;
    let mut coordinates = Vec::with_capacity(shape.rank());

    for (axis, &stride) in strides.as_slice().iter().enumerate() {
        let coordinate = remaining / stride;
        remaining %= stride;

        if coordinate >= shape.dimensions()[axis] {
            return Err(IndexingError::TensorCoordinateOutOfRange {
                axis,
                coordinate,
                dimension: shape.dimensions()[axis],
            });
        }

        coordinates.push(coordinate);
    }

    Ok(coordinates)
}

// =============================================================================
// Tensor permutation
// =============================================================================

/// Validates a tensor-axis permutation.
///
/// A valid permutation of rank `n` contains every integer in `0..n` exactly
/// once.
pub fn validate_permutation(permutation: &[usize]) -> IndexResult<()> {
    let rank = permutation.len();

    for &axis in permutation {
        if axis >= rank {
            return Err(IndexingError::InvalidPermutation { axis, rank });
        }
    }

    for position in 0..rank {
        let axis = permutation[position];

        for &previous in &permutation[..position] {
            if previous == axis {
                return Err(IndexingError::DuplicatePermutationAxis { axis });
            }
        }
    }

    Ok(())
}

/// Returns the shape obtained by applying an axis permutation.
///
/// The permutation uses the convention:
///
/// ```text
/// output_axis[i] = input_axis[permutation[i]]
/// ```
pub fn permuted_shape(
    shape: &TensorShape,
    permutation: &[usize],
) -> IndexResult<TensorShape> {
    validate_permutation(permutation)?;

    if permutation.len() != shape.rank() {
        return Err(IndexingError::TensorRankMismatch {
            rank: shape.rank(),
            coordinates: permutation.len(),
        });
    }

    let dimensions = permutation
        .iter()
        .map(|&axis| shape.dimensions()[axis])
        .collect::<Vec<_>>();

    TensorShape::new(dimensions)
}

/// Maps output coordinates through a permutation into input coordinates.
///
/// The convention is:
///
/// ```text
/// output_axis[i] = input_axis[permutation[i]]
/// ```
///
/// Therefore, given output coordinates, the returned vector is indexed in
/// input-axis order.
pub fn inverse_permuted_coordinates(
    output_coordinates: &[usize],
    permutation: &[usize],
) -> IndexResult<Vec<usize>> {
    validate_permutation(permutation)?;

    if output_coordinates.len() != permutation.len() {
        return Err(IndexingError::TensorRankMismatch {
            rank: permutation.len(),
            coordinates: output_coordinates.len(),
        });
    }

    let rank = permutation.len();
    let mut input_coordinates = vec![0usize; rank];

    for output_axis in 0..rank {
        let input_axis = permutation[output_axis];
        input_coordinates[input_axis] = output_coordinates[output_axis];
    }

    Ok(input_coordinates)
}

// =============================================================================
// Range indexing
// =============================================================================

/// Validates a half-open range `[start, end)` against an upper bound.
pub const fn validate_range(
    start: usize,
    end: usize,
    upper_bound: usize,
) -> IndexResult<()> {
    if start > end || end > upper_bound {
        return Err(IndexingError::InvalidRange {
            start,
            end,
            upper_bound,
        });
    }

    Ok(())
}

/// Returns the number of elements in a checked half-open range.
pub const fn checked_range_len(
    start: usize,
    end: usize,
    upper_bound: usize,
) -> IndexResult<usize> {
    validate_range(start, end, upper_bound)?;

    Ok(end - start)
}

// =============================================================================
// Flattened register indexing
// =============================================================================

/// Returns the linear offset of a logical qubit inside a contiguous logical
/// register.
///
/// This function intentionally does not perform routing or physical mapping.
/// A contiguous logical register has:
///
/// ```text
/// q0 -> 0
/// q1 -> 1
/// q2 -> 2
/// ...
/// ```
pub const fn register_index(
    qubit: usize,
    register_qubits: usize,
) -> IndexResult<usize> {
    validate_qubit_in_register(qubit, register_qubits)?;
    Ok(qubit)
}

/// Converts a register index into a logical qubit index after validating it.
pub const fn register_qubit(
    index: usize,
    register_qubits: usize,
) -> IndexResult<usize> {
    if index >= register_qubits {
        return Err(IndexingError::QubitIndexOutOfRange {
            qubit: index,
            maximum: register_qubits.saturating_sub(1),
        });
    }

    validate_qubit_index(index)?;
    Ok(index)
}

// =============================================================================
// Batch basis indexing
// =============================================================================

/// Validates every basis index in a slice.
pub fn validate_basis_indices(
    indices: &[usize],
    qubits: usize,
) -> IndexResult<()> {
    for &index in indices {
        validate_basis_index(index, qubits)?;
    }

    Ok(())
}

/// Converts a slice of basis indices into validated amplitude indices.
///
/// This performs no allocation if validation fails.
pub fn amplitude_indices(
    indices: &[usize],
    qubits: usize,
) -> IndexResult<Vec<usize>> {
    validate_basis_indices(indices, qubits)?;

    Ok(indices.to_vec())
}

// =============================================================================
// Utility functions for state-vector kernels
// =============================================================================

/// Returns the number of independent qubit pairs for a single-qubit
/// operation.
///
/// A state vector with `n` qubits contains `2^(n-1)` pairs for any selected
/// qubit.
pub const fn single_qubit_pair_count(
    qubits: usize,
) -> IndexResult<usize> {
    if qubits == 0 {
        return Err(IndexingError::QubitCountTooLarge {
            qubits,
            maximum: maximum_indexable_qubit_count(),
        });
    }

    basis_state_count(qubits - 1)
}

/// Returns the first index of the `pair`-th single-qubit pair for `qubit`.
///
/// The pair ordering follows the standard little-endian dense layout.
pub fn single_qubit_pair_start(
    pair: usize,
    qubit: usize,
    qubits: usize,
) -> IndexResult<usize> {
    if qubits == 0 {
        return Err(IndexingError::QubitCountTooLarge {
            qubits,
            maximum: maximum_indexable_qubit_count(),
        });
    }

    validate_qubit_in_register(qubit, qubits)?;

    let pair_count = single_qubit_pair_count(qubits)?;

    if pair >= pair_count {
        return Err(IndexingError::AmplitudeIndexOutOfRange {
            index: pair,
            amplitudes: pair_count,
        });
    }

    let stride = qubit_stride(qubit)?;
    let block_width = stride
        .checked_mul(2)
        .ok_or(IndexingError::ArithmeticOverflow {
            operation: "single-qubit block width",
        })?;

    let block = pair / stride;
    let offset = pair % stride;

    let block_start = block
        .checked_mul(block_width)
        .ok_or(IndexingError::ArithmeticOverflow {
            operation: "single-qubit pair block",
        })?;

    block_start
        .checked_add(offset)
        .ok_or(IndexingError::ArithmeticOverflow {
            operation: "single-qubit pair start",
        })
}

// =============================================================================
// Two-dimensional qubit-subspace indexing
// =============================================================================

/// Returns the four basis indices corresponding to two selected qubits while
/// preserving all other bits.
///
/// The returned array is ordered by the selected-qubit pattern:
///
/// ```text
/// [00, 01, 10, 11]
/// ```
///
/// The selected qubits are interpreted in the order supplied by `qubits`.
pub fn two_qubit_subspace(
    base_index: usize,
    qubits: [usize; 2],
    register_qubits: usize,
) -> IndexResult<[usize; 4]> {
    validate_basis_index(base_index, register_qubits)?;
    validate_unique_qubits(&qubits)?;

    validate_qubit_in_register(qubits[0], register_qubits)?;
    validate_qubit_in_register(qubits[1], register_qubits)?;

    let cleared = clear_bit(
        clear_bit(base_index, qubits[0])?,
        qubits[1],
    )?;

    Ok([
        cleared,
        set_bit(cleared, qubits[0])?,
        set_bit(cleared, qubits[1])?,
        set_bit(
            set_bit(cleared, qubits[0])?,
            qubits[1],
        )?,
    ])
}

// =============================================================================
// Documentation-level invariants
// =============================================================================

/// Returns whether a basis index belongs to a valid `qubits`-wide state
/// space.
///
/// This is a convenience predicate for hot-path validation decisions.
///
/// It never panics.
pub const fn is_valid_basis_index(
    basis_index: usize,
    qubits: usize,
) -> bool {
    match validate_basis_index(basis_index, qubits) {
        Ok(()) => true,
        Err(_) => false,
    }
}

/// Returns whether a qubit index can be represented by a machine bit mask.
pub const fn is_valid_qubit_index(qubit: usize) -> bool {
    qubit <= maximum_mask_qubit_index()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_bits_is_correct() {
        assert_eq!(required_bits_for_count(0), 0);
        assert_eq!(required_bits_for_count(1), 0);
        assert_eq!(required_bits_for_count(2), 1);
        assert_eq!(required_bits_for_count(3), 2);
        assert_eq!(required_bits_for_count(4), 2);
        assert_eq!(required_bits_for_count(5), 3);
        assert_eq!(required_bits_for_count(8), 3);
        assert_eq!(required_bits_for_count(9), 4);
    }

    #[test]
    fn basis_state_count_is_checked() {
        assert_eq!(basis_state_count(0).unwrap(), 1);
        assert_eq!(basis_state_count(1).unwrap(), 2);
        assert_eq!(basis_state_count(2).unwrap(), 4);
        assert_eq!(basis_state_count(3).unwrap(), 8);
    }

    #[test]
    fn basis_state_count_rejects_unrepresentable_space() {
        let result = basis_state_count(maximum_indexable_qubit_count() + 1);

        assert!(matches!(
            result,
            Err(IndexingError::QubitCountTooLarge { .. })
        ));
    }

    #[test]
    fn qubit_masks_are_correct() {
        assert_eq!(qubit_mask(0).unwrap(), 0b0001);
        assert_eq!(qubit_mask(1).unwrap(), 0b0010);
        assert_eq!(qubit_mask(2).unwrap(), 0b0100);
        assert_eq!(qubit_mask(3).unwrap(), 0b1000);
    }

    #[test]
    fn highest_mask_is_representable() {
        let mask = qubit_mask(maximum_mask_qubit_index()).unwrap();

        assert_eq!(mask, 1usize << maximum_mask_qubit_index());
    }

    #[test]
    fn qubit_index_above_mask_range_is_rejected() {
        let result = qubit_mask(maximum_mask_qubit_index() + 1);

        assert!(matches!(
            result,
            Err(IndexingError::QubitIndexOutOfRange { .. })
        ));
    }

    #[test]
    fn basis_index_validation_is_correct() {
        assert!(validate_basis_index(0, 3).is_ok());
        assert!(validate_basis_index(7, 3).is_ok());
        assert!(validate_basis_index(8, 3).is_err());
    }

    #[test]
    fn bit_operations_are_correct() {
        let index = 0b0101usize;

        assert!(is_bit_set(index, 0).unwrap());
        assert!(!is_bit_set(index, 1).unwrap());
        assert!(is_bit_set(index, 2).unwrap());

        assert_eq!(set_bit(index, 1).unwrap(), 0b0111);
        assert_eq!(clear_bit(index, 2).unwrap(), 0b0001);
        assert_eq!(toggle_bit(index, 1).unwrap(), 0b0111);
        assert_eq!(toggle_bit(index, 0).unwrap(), 0b0100);
    }

    #[test]
    fn replace_bit_is_correct() {
        assert_eq!(replace_bit(0b0000, 2, true).unwrap(), 0b0100);
        assert_eq!(replace_bit(0b1111, 2, false).unwrap(), 0b1011);
    }

    #[test]
    fn qubit_stride_is_correct() {
        assert_eq!(qubit_stride(0).unwrap(), 1);
        assert_eq!(qubit_stride(1).unwrap(), 2);
        assert_eq!(qubit_stride(2).unwrap(), 4);
        assert_eq!(qubit_stride(3).unwrap(), 8);
    }

    #[test]
    fn paired_indices_are_correct() {
        let basis = 0b0101usize;

        assert_eq!(
            paired_index(basis, 1, 3).unwrap(),
            0b0111
        );

        assert_eq!(
            lower_pair_index(basis, 2, 3).unwrap(),
            0b0001
        );

        assert_eq!(
            upper_pair_index(basis, 1, 3).unwrap(),
            0b0111
        );
    }

    #[test]
    fn unique_qubit_validation_is_correct() {
        assert!(validate_unique_qubits(&[0, 1, 2]).is_ok());
        assert!(validate_unique_qubits(&[0, 2, 5]).is_ok());

        assert!(validate_unique_qubits(&[0, 1, 0]).is_err());
    }

    #[test]
    fn extract_and_insert_bits_round_trip() {
        let original = 0b101101usize;

        let selected = [0usize, 2, 5];

        let packed =
            extract_bits(original, &selected, 6).unwrap();

        let cleared =
            insert_bits(0, &selected, packed, 6).unwrap();

        assert_eq!(extract_bits(cleared, &selected, 6).unwrap(), packed);
        assert_eq!(
            is_bit_set(cleared, 0).unwrap(),
            is_bit_set(original, 0).unwrap()
        );
        assert_eq!(
            is_bit_set(cleared, 2).unwrap(),
            is_bit_set(original, 2).unwrap()
        );
        assert_eq!(
            is_bit_set(cleared, 5).unwrap(),
            is_bit_set(original, 5).unwrap()
        );
    }

    #[test]
    fn insert_bits_rejects_too_large_packed_value() {
        let result = insert_bits(
            0,
            &[0, 1],
            4,
            4,
        );

        assert!(matches!(
            result,
            Err(IndexingError::BasisIndexOutOfRange { .. })
        ));
    }

    #[test]
    fn matrix_indexing_is_correct() {
        assert_eq!(matrix_dimension(2).unwrap(), 4);
        assert_eq!(matrix_element_count(2).unwrap(), 16);

        assert_eq!(matrix_index(0, 0, 2).unwrap(), 0);
        assert_eq!(matrix_index(0, 3, 2).unwrap(), 3);
        assert_eq!(matrix_index(1, 0, 2).unwrap(), 4);
        assert_eq!(matrix_index(3, 3, 2).unwrap(), 15);
    }

    #[test]
    fn matrix_coordinates_round_trip() {
        for index in 0..16 {
            let (row, column) =
                matrix_coordinates(index, 2).unwrap();

            assert_eq!(
                matrix_index(row, column, 2).unwrap(),
                index
            );
        }
    }

    #[test]
    fn tensor_shape_is_validated() {
        let shape =
            TensorShape::new(vec![2, 3, 4]).unwrap();

        assert_eq!(shape.rank(), 3);
        assert_eq!(shape.size().unwrap(), 24);
        assert_eq!(shape.dimension(0), Some(2));
        assert_eq!(shape.dimension(1), Some(3));
        assert_eq!(shape.dimension(2), Some(4));
    }

    #[test]
    fn tensor_shape_rejects_empty() {
        let result = TensorShape::new(Vec::new());

        assert!(matches!(
            result,
            Err(IndexingError::EmptyTensorShape)
        ));
    }

    #[test]
    fn tensor_shape_rejects_zero_dimension() {
        let result =
            TensorShape::new(vec![2, 0, 4]);

        assert!(matches!(
            result,
            Err(IndexingError::ZeroTensorDimension { axis: 1 })
        ));
    }

    #[test]
    fn tensor_strides_are_row_major() {
        let shape =
            TensorShape::new(vec![2, 3, 4]).unwrap();

        let strides =
            TensorStrides::from_shape(&shape).unwrap();

        assert_eq!(
            strides.as_slice(),
            &[12, 4, 1]
        );
    }

    #[test]
    fn tensor_offset_is_correct() {
        let shape =
            TensorShape::new(vec![2, 3, 4]).unwrap();

        let strides =
            TensorStrides::from_shape(&shape).unwrap();

        assert_eq!(
            tensor_offset(
                &shape,
                &strides,
                &[0, 0, 0]
            )
            .unwrap(),
            0
        );

        assert_eq!(
            tensor_offset(
                &shape,
                &strides,
                &[1, 2, 3]
            )
            .unwrap(),
            23
        );

        assert_eq!(
            tensor_offset(
                &shape,
                &strides,
                &[1, 0, 0]
            )
            .unwrap(),
            12
        );
    }

    #[test]
    fn tensor_coordinate_round_trip() {
        let shape =
            TensorShape::new(vec![2, 3, 4]).unwrap();

        let strides =
            TensorStrides::from_shape(&shape).unwrap();

        for index in 0..24 {
            let coordinates =
                tensor_coordinates(&shape, index).unwrap();

            let reconstructed =
                tensor_offset(
                    &shape,
                    &strides,
                    &coordinates,
                )
                .unwrap();

            assert_eq!(reconstructed, index);
        }
    }

    #[test]
    fn tensor_coordinate_validation_is_correct() {
        let shape =
            TensorShape::new(vec![2, 3, 4]).unwrap();

        assert!(
            shape.validate_coordinates(&[1, 2, 3]).is_ok()
        );

        assert!(
            shape.validate_coordinates(&[2, 0, 0]).is_err()
        );

        assert!(
            shape.validate_coordinates(&[1, 2]).is_err()
        );
    }

    #[test]
    fn permutation_validation_is_correct() {
        assert!(
            validate_permutation(&[0, 1, 2]).is_ok()
        );

        assert!(
            validate_permutation(&[2, 0, 1]).is_ok()
        );

        assert!(
            validate_permutation(&[0, 0, 1]).is_err()
        );

        assert!(
            validate_permutation(&[0, 3, 1]).is_err()
        );
    }

    #[test]
    fn permuted_shape_is_correct() {
        let shape =
            TensorShape::new(vec![2, 3, 4]).unwrap();

        let result =
            permuted_shape(&shape, &[2, 0, 1]).unwrap();

        assert_eq!(
            result.dimensions(),
            &[4, 2, 3]
        );
    }

    #[test]
    fn inverse_permuted_coordinates_is_correct() {
        let output = [30usize, 10, 20];
        let permutation = [2usize, 0, 1];

        let input =
            inverse_permuted_coordinates(
                &output,
                &permutation,
            )
            .unwrap();

        assert_eq!(input, vec![10, 20, 30]);
    }

    #[test]
    fn checked_ranges_are_correct() {
        assert!(
            validate_range(0, 0, 10).is_ok()
        );

        assert!(
            validate_range(2, 5, 10).is_ok()
        );

        assert_eq!(
            checked_range_len(2, 5, 10).unwrap(),
            3
        );

        assert!(
            validate_range(5, 2, 10).is_err()
        );

        assert!(
            validate_range(2, 11, 10).is_err()
        );
    }

    #[test]
    fn single_qubit_pair_count_is_correct() {
        assert_eq!(
            single_qubit_pair_count(1).unwrap(),
            1
        );

        assert_eq!(
            single_qubit_pair_count(2).unwrap(),
            2
        );

        assert_eq!(
            single_qubit_pair_count(3).unwrap(),
            4
        );
    }

    #[test]
    fn two_qubit_subspace_is_correct() {
        let result =
            two_qubit_subspace(
                0b1000,
                [0, 2],
                4,
            )
            .unwrap();

        assert_eq!(
            result,
            [
                0b1000,
                0b1001,
                0b1100,
                0b1101
            ]
        );
    }

    #[test]
    fn basis_indices_are_validated() {
        let indices = [0usize, 1, 2, 7];

        assert!(
            validate_basis_indices(
                &indices,
                3
            )
            .is_ok()
        );

        let invalid = [0usize, 8];

        assert!(
            validate_basis_indices(
                &invalid,
                3
            )
            .is_err()
        );
    }

    #[test]
    fn convenience_predicates_never_panic() {
        assert!(is_valid_qubit_index(0));
        assert!(is_valid_basis_index(0, 0));
        assert!(!is_valid_basis_index(1, 0));
    }
}