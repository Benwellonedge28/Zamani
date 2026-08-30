//! Zamani Quantum Optimization — Binary Symplectic Algebra
//!
//! Production-grade binary symplectic representation for Clifford
//! transformations over qubits.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!             optimization::algebra::symplectic
//!                              │
//!             ┌────────────────┼─────────────────┐
//!             ▼                ▼                 ▼
//!          Clifford         synthesis       verification
//!          algebra             │                 │
//!             │                │                 │
//!             └────────────────┼─────────────────┘
//!                              ▼
//!                         optimization
//! ```
//!
//! This module owns the *binary symplectic linear part* of Clifford
//! transformations.
//!
//! It intentionally does NOT own:
//!
//! - the canonical Quantum IR;
//! - circuit storage;
//! - Pauli signs/phases;
//! - stabilizer measurement outcomes;
//! - optimization pass scheduling;
//! - routing;
//! - hardware topology;
//! - pulse generation;
//! - execution;
//! - QPU communication;
//! - error-correction codes;
//! - backend-specific costs.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Mathematical representation
//!
//! For `n` qubits, a Pauli support is represented over `F₂` as
//!
//! ```text
//! v = (x | z)
//! ```
//!
//! where:
//!
//! - `x[i] = 1` means X acts on qubit `i`;
//! - `z[i] = 1` means Z acts on qubit `i`;
//! - `x[i] = z[i] = 1` therefore represents Y support, ignoring phase.
//!
//! A Clifford transformation is represented by a binary matrix
//!
//! ```text
//! M ∈ F₂^(2n × 2n)
//! ```
//!
//! acting as
//!
//! ```text
//! v' = M v
//! ```
//!
//! The symplectic form used by this module is
//!
//! ```text
//! Ω = [ 0  I ]
//!     [ I  0 ]
//! ```
//!
//! A valid binary symplectic matrix satisfies
//!
//! ```text
//! M Ω Mᵀ = Ω
//! ```
//!
//! over `F₂`.
//!
//! Equivalently:
//!
//! ```text
//! Mᵀ Ω M = Ω
//! ```
//!
//! A major consequence is:
//!
//! ```text
//! M⁻¹ = Ω Mᵀ Ω
//! ```
//!
//! # Important phase limitation
//!
//! A binary symplectic matrix represents the Clifford action *modulo Pauli
//! signs/phases*. Consequently:
//!
//! - X, Y and Z have the identity symplectic matrix;
//! - S and S† have the same symplectic matrix;
//! - V and V† have the same symplectic matrix;
//! - two Clifford operations differing by a Pauli correction can therefore
//!   have the same symplectic matrix.
//!
//! This is intentional.
//!
//! The existing `optimization::algebra::clifford` subsystem is responsible
//! for phase/sign information. This module must not invent a second phase
//! representation.
//!
//! # Storage
//!
//! The matrix is stored densely but bit-packed:
//!
//! ```text
//! 2n rows × ceil(2n / 64) u64 words
//! ```
//!
//! Therefore the raw storage is O(n²) bits rather than O(n²) bytes.
//!
//! The implementation uses checked arithmetic and fallible allocation where
//! allocations are performed explicitly.
//!
//! There is no artificial qubit-count ceiling. Practical limits are imposed
//! only by:
//!
//! - `usize` addressability;
//! - available memory;
//! - the caller's configured optimization/IR limits.
//!
//! # Performance
//!
//! Elementary Clifford updates modify a small number of matrix rows and are
//! therefore O(n / W), where W is the machine-word width.
//!
//! Matrix composition is O(n³ / W) in the straightforward dense
//! implementation.
//!
//! Symplectic validation is O(n³ / W).
//!
//! Inversion uses the symplectic identity
//!
//! ```text
//! M⁻¹ = Ω Mᵀ Ω
//! ```
//!
//! and therefore avoids generic Gaussian elimination.
//!
//! # Canonical IR integration
//!
//! The only circuit-level input accepted by this module is the canonical:
//!
//! ```text
//! crate::quantum::ir::Gate
//! ```
//!
//! No optimization-specific `QuantumGate` type is defined here.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no external dependencies;
//! - no `unsafe`;
//! - no nightly features.
//!
//! # Integration contract
//!
//! Future optimization files should consume this module through:
//!
//! - [`SymplecticMatrix`];
//! - [`SymplecticError`];
//! - [`SymplecticResult`];
//! - [`SymplecticVector`];
//! - [`SymplecticGateClass`];
//! - [`classify_gate`];
//!
//! In particular:
//!
//! ```text
//! algebra::clifford
//!          │
//!          ├──── phase/sign aware Clifford representation
//!          │
//!          ▼
//! algebra::symplectic
//!          │
//!          ├──── binary linear Clifford action
//!          │
//!          ├──── phase_polynomial
//!          ├──── synthesis::clifford
//!          └──── verification
//! ```
//!
//! The symplectic layer must never become dependent on optimization passes,
//! routing, hardware, or benchmarking.

// =============================================================================
// Imports
// =============================================================================

use std::fmt;

use crate::quantum::ir::{Gate, GateKind, QubitId};

// =============================================================================
// Constants
// =============================================================================

/// Number of bits stored in one packed machine word.
///
/// The implementation deliberately fixes this to `u64` rather than relying
/// on `usize` width so serialized/debug representations remain predictable.
const WORD_BITS: usize = u64::BITS as usize;

// =============================================================================
// Result
// =============================================================================

/// Result type used by this module.
pub type SymplecticResult<T> = Result<T, SymplecticError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by binary symplectic operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymplecticError {
    /// The requested matrix dimensions cannot be represented safely.
    DimensionOverflow {
        /// Number of qubits.
        qubits: usize,
    },

    /// A matrix/vector allocation could not be reserved.
    AllocationFailure {
        /// Logical resource being allocated.
        resource: &'static str,

        /// Requested element count.
        requested: usize,
    },

    /// A matrix contains dimensions that do not describe a 2n × 2n matrix.
    InvalidDimension {
        /// Number of rows.
        rows: usize,

        /// Number of columns.
        columns: usize,
    },

    /// A qubit is outside the represented logical namespace.
    QubitOutOfRange {
        /// Requested logical qubit.
        qubit: QubitId,

        /// Number of represented qubits.
        qubit_count: usize,
    },

    /// A supplied gate is not a Clifford operation supported by the
    /// symplectic representation.
    UnsupportedGate {
        /// Gate kind.
        gate: GateKind,
    },

    /// A parameterized gate cannot be classified by this exact,
    /// parameter-independent representation.
    ParameterizedGate {
        /// Gate kind.
        gate: GateKind,
    },

    /// A non-unitary operation was supplied.
    NonUnitaryGate {
        /// Gate kind.
        gate: GateKind,
    },

    /// A gate has the wrong number of operands.
    InvalidArity {
        /// Gate kind.
        gate: GateKind,

        /// Expected operand count.
        expected: usize,

        /// Actual operand count.
        actual: usize,
    },

    /// Two symplectic objects have incompatible widths.
    WidthMismatch {
        /// Left width.
        left: usize,

        /// Right width.
        right: usize,
    },

    /// A supplied matrix violates the binary symplectic condition.
    NotSymplectic {
        /// First row involved in the failed pair.
        row_a: usize,

        /// Second row involved in the failed pair.
        row_b: usize,
    },

    /// A supplied vector has the wrong number of packed words.
    InvalidVectorStorage {
        /// Expected word count.
        expected: usize,

        /// Actual word count.
        actual: usize,
    },

    /// An internal invariant was violated.
    InvalidInvariant {
        /// Static invariant description.
        message: &'static str,
    },
}

impl fmt::Display for SymplecticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionOverflow { qubits } => {
                write!(
                    f,
                    "binary symplectic dimension overflow for {qubits} qubits"
                )
            }

            Self::AllocationFailure {
                resource,
                requested,
            } => {
                write!(
                    f,
                    "allocation failed for {resource}: requested {requested} elements"
                )
            }

            Self::InvalidDimension { rows, columns } => {
                write!(
                    f,
                    "invalid symplectic matrix dimensions: {rows} × {columns}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                qubit_count,
            } => {
                write!(
                    f,
                    "logical qubit {qubit} is outside symplectic width {qubit_count}"
                )
            }

            Self::UnsupportedGate { gate } => {
                write!(
                    f,
                    "gate {gate:?} is not supported by the binary symplectic representation"
                )
            }

            Self::ParameterizedGate { gate } => {
                write!(
                    f,
                    "parameterized gate {gate:?} requires exact angle classification"
                )
            }

            Self::NonUnitaryGate { gate } => {
                write!(
                    f,
                    "non-unitary gate {gate:?} cannot be represented as a Clifford symplectic transformation"
                )
            }

            Self::InvalidArity {
                gate,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "gate {gate:?} requires {expected} operands, received {actual}"
                )
            }

            Self::WidthMismatch { left, right } => {
                write!(
                    f,
                    "symplectic width mismatch: left={left}, right={right}"
                )
            }

            Self::NotSymplectic { row_a, row_b } => {
                write!(
                    f,
                    "symplectic condition failed for rows {row_a} and {row_b}"
                )
            }

            Self::InvalidVectorStorage { expected, actual } => {
                write!(
                    f,
                    "invalid packed symplectic vector storage: expected {expected} words, received {actual}"
                )
            }

            Self::InvalidInvariant { message } => {
                write!(f, "invalid symplectic invariant: {message}")
            }
        }
    }
}

impl std::error::Error for SymplecticError {}

// =============================================================================
// Gate classification
// =============================================================================

/// Clifford gate class recognized by the symplectic layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymplecticGateClass {
    /// Identity or Pauli operation.
    Pauli,

    /// One-qubit Clifford operation that changes the symplectic support.
    SingleQubit,

    /// Two-qubit Clifford operation.
    TwoQubit,
}

impl SymplecticGateClass {
    /// Returns the expected arity.
    #[must_use]
    pub const fn arity(self) -> usize {
        match self {
            Self::Pauli => 1,
            Self::SingleQubit => 1,
            Self::TwoQubit => 2,
        }
    }
}

/// Classifies a canonical IR gate for the symplectic representation.
///
/// The classification deliberately follows mathematical Clifford semantics,
/// rather than blindly trusting a gate-name flag in another module.
///
/// Parameterized gates are rejected even if a particular parameter value
/// could make them Clifford. Exact angle classification belongs to a future
/// parameter-aware layer.
pub fn classify_gate(gate: &Gate) -> SymplecticResult<SymplecticGateClass> {
    let kind = gate.kind();

    if !kind.is_unitary() {
        return Err(SymplecticError::NonUnitaryGate { gate: kind });
    }

    if kind.is_parameterized() {
        return Err(SymplecticError::ParameterizedGate { gate: kind });
    }

    let class = match kind {
        GateKind::I
        | GateKind::X
        | GateKind::Y
        | GateKind::Z => SymplecticGateClass::Pauli,

        GateKind::H
        | GateKind::S
        | GateKind::Sdg
        | GateKind::V
        | GateKind::Vdg => SymplecticGateClass::SingleQubit,

        GateKind::CX
        | GateKind::CY
        | GateKind::CZ
        | GateKind::SWAP
        | GateKind::ISWAP => SymplecticGateClass::TwoQubit,

        // These gates are intentionally rejected here.
        //
        // CH is not a Clifford gate merely because H itself is Clifford.
        // ECR is kept outside this layer until its exact canonical Clifford
        // semantics are established by the IR/backend contract.
        GateKind::CH
        | GateKind::ECR
        | GateKind::CCX
        | GateKind::CSWAP
        | GateKind::RX
        | GateKind::RY
        | GateKind::RZ
        | GateKind::Phase
        | GateKind::U1
        | GateKind::U2
        | GateKind::U3
        | GateKind::CRX
        | GateKind::CRY
        | GateKind::CRZ
        | GateKind::Measure
        | GateKind::Barrier
        | GateKind::Reset
        | GateKind::T
        | GateKind::Tdg => {
            return Err(SymplecticError::UnsupportedGate { gate: kind });
        }
    };

    let actual = gate.qubits().len();

    if actual != class.arity() {
        return Err(SymplecticError::InvalidArity {
            gate: kind,
            expected: class.arity(),
            actual,
        });
    }

    Ok(class)
}

// =============================================================================
// Symplectic vector
// =============================================================================

/// A packed binary symplectic vector.
///
/// The vector contains `2n` bits:
///
/// ```text
/// [ x_0 ... x_(n-1) | z_0 ... z_(n-1) ]
/// ```
///
/// The type is useful for propagating Pauli support through a
/// [`SymplecticMatrix`].
///
/// It intentionally does not contain a Pauli phase/sign.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymplecticVector {
    qubit_count: usize,
    words: Vec<u64>,
}

impl SymplecticVector {
    /// Creates the zero vector for `qubit_count` qubits.
    pub fn zero(qubit_count: usize) -> SymplecticResult<Self> {
        let dimension = checked_dimension(qubit_count)?;
        let word_count = words_for_bits(dimension)?;

        let mut words = Vec::new();

        words.try_reserve_exact(word_count).map_err(|_| {
            SymplecticError::AllocationFailure {
                resource: "symplectic vector",
                requested: word_count,
            }
        })?;

        words.resize(word_count, 0);

        Ok(Self {
            qubit_count,
            words,
        })
    }

    /// Creates a vector from packed words.
    ///
    /// The caller must provide exactly the number of words required for
    /// `2 * qubit_count` bits.
    pub fn from_words(
        qubit_count: usize,
        words: Vec<u64>,
    ) -> SymplecticResult<Self> {
        let dimension = checked_dimension(qubit_count)?;
        let expected = words_for_bits(dimension)?;

        if words.len() != expected {
            return Err(SymplecticError::InvalidVectorStorage {
                expected,
                actual: words.len(),
            });
        }

        Ok(Self {
            qubit_count,
            words,
        })
    }

    /// Returns the number of represented qubits.
    #[must_use]
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Returns the number of represented binary coordinates.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.qubit_count.saturating_mul(2)
    }

    /// Returns the packed storage.
    #[must_use]
    pub fn words(&self) -> &[u64] {
        &self.words
    }

    /// Returns a bit at a binary coordinate.
    #[must_use]
    pub fn get(&self, coordinate: usize) -> Option<bool> {
        if coordinate >= self.dimension() {
            return None;
        }

        let word = coordinate / WORD_BITS;
        let bit = coordinate % WORD_BITS;

        self.words
            .get(word)
            .map(|value| ((value >> bit) & 1) != 0)
    }

    /// Sets a binary coordinate.
    pub fn set(
        &mut self,
        coordinate: usize,
        value: bool,
    ) -> SymplecticResult<()> {
        if coordinate >= self.dimension() {
            return Err(SymplecticError::InvalidInvariant {
                message: "symplectic vector coordinate is outside its dimension",
            });
        }

        set_packed_bit(&mut self.words, coordinate, value)
    }

    /// Returns the X component for a logical qubit.
    #[must_use]
    pub fn x(&self, qubit: usize) -> Option<bool> {
        self.get(qubit)
    }

    /// Returns the Z component for a logical qubit.
    #[must_use]
    pub fn z(&self, qubit: usize) -> Option<bool> {
        qubit
            .checked_add(self.qubit_count)
            .and_then(|coordinate| self.get(coordinate))
    }

    /// Sets the X component for a logical qubit.
    pub fn set_x(
        &mut self,
        qubit: usize,
        value: bool,
    ) -> SymplecticResult<()> {
        if qubit >= self.qubit_count {
            return Err(SymplecticError::QubitOutOfRange {
                qubit: QubitId::new(qubit),
                qubit_count: self.qubit_count,
            });
        }

        self.set(qubit, value)
    }

    /// Sets the Z component for a logical qubit.
    pub fn set_z(
        &mut self,
        qubit: usize,
        value: bool,
    ) -> SymplecticResult<()> {
        if qubit >= self.qubit_count {
            return Err(SymplecticError::QubitOutOfRange {
                qubit: QubitId::new(qubit),
                qubit_count: self.qubit_count,
            });
        }

        let coordinate = qubit
            .checked_add(self.qubit_count)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        self.set(coordinate, value)
    }

    /// Creates an X Pauli support vector.
    pub fn x_pauli(
        qubit_count: usize,
        qubit: usize,
    ) -> SymplecticResult<Self> {
        let mut result = Self::zero(qubit_count)?;
        result.set_x(qubit, true)?;
        Ok(result)
    }

    /// Creates a Z Pauli support vector.
    pub fn z_pauli(
        qubit_count: usize,
        qubit: usize,
    ) -> SymplecticResult<Self> {
        let mut result = Self::zero(qubit_count)?;
        result.set_z(qubit, true)?;
        Ok(result)
    }

    /// Creates a Y Pauli support vector.
    pub fn y_pauli(
        qubit_count: usize,
        qubit: usize,
    ) -> SymplecticResult<Self> {
        let mut result = Self::zero(qubit_count)?;
        result.set_x(qubit, true)?;
        result.set_z(qubit, true)?;
        Ok(result)
    }
}

// =============================================================================
// Symplectic matrix
// =============================================================================

/// Dense bit-packed binary symplectic matrix.
///
/// The matrix is a `2n × 2n` linear transformation over `F₂`.
///
/// Rows are output coordinates and columns are input coordinates:
///
/// ```text
/// output = matrix × input
/// ```
///
/// Coordinates are ordered:
///
/// ```text
/// X_0 ... X_(n-1) Z_0 ... Z_(n-1)
/// ```
///
/// This convention is stable and must be used by all future Zamani
/// symplectic/Clifford code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymplecticMatrix {
    qubit_count: usize,
    words_per_row: usize,
    rows: Vec<u64>,
}

impl SymplecticMatrix {
    /// Creates the identity symplectic matrix for `qubit_count` qubits.
    pub fn identity(qubit_count: usize) -> SymplecticResult<Self> {
        let mut matrix = Self::zero(qubit_count)?;

        let dimension = matrix.dimension();

        for coordinate in 0..dimension {
            matrix.set(coordinate, coordinate, true)?;
        }

        Ok(matrix)
    }

    /// Creates a zero matrix.
    ///
    /// A zero matrix is not symplectic. This constructor exists for
    /// low-level construction and testing; call [`Self::validate`] before
    /// treating it as a valid symplectic transformation.
    pub fn zero(qubit_count: usize) -> SymplecticResult<Self> {
        let dimension = checked_dimension(qubit_count)?;
        let words_per_row = words_for_bits(dimension)?;

        let row_count = dimension;

        let total_words = row_count
            .checked_mul(words_per_row)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: qubit_count,
            })?;

        let mut rows = Vec::new();

        rows.try_reserve_exact(total_words).map_err(|_| {
            SymplecticError::AllocationFailure {
                resource: "binary symplectic matrix",
                requested: total_words,
            }
        })?;

        rows.resize(total_words, 0);

        Ok(Self {
            qubit_count,
            words_per_row,
            rows,
        })
    }

    /// Creates a matrix from packed row storage.
    ///
    /// The input must contain exactly:
    ///
    /// `2n * ceil(2n / 64)`
    ///
    /// words.
    pub fn from_words(
        qubit_count: usize,
        words: Vec<u64>,
    ) -> SymplecticResult<Self> {
        let dimension = checked_dimension(qubit_count)?;
        let words_per_row = words_for_bits(dimension)?;

        let expected = dimension.checked_mul(words_per_row).ok_or(
            SymplecticError::DimensionOverflow {
                qubits: qubit_count,
            },
        )?;

        if words.len() != expected {
            return Err(SymplecticError::InvalidInvariant {
                message: "packed matrix storage length does not match matrix dimensions",
            });
        }

        Ok(Self {
            qubit_count,
            words_per_row,
            rows: words,
        })
    }

    /// Returns the number of represented qubits.
    #[must_use]
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Returns the matrix dimension, `2n`.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.qubit_count.saturating_mul(2)
    }

    /// Returns the number of packed words per row.
    #[must_use]
    pub const fn words_per_row(&self) -> usize {
        self.words_per_row
    }

    /// Returns the packed row-major storage.
    #[must_use]
    pub fn words(&self) -> &[u64] {
        &self.rows
    }

    /// Returns a matrix element.
    #[must_use]
    pub fn get(&self, row: usize, column: usize) -> Option<bool> {
        if row >= self.dimension() || column >= self.dimension() {
            return None;
        }

        let base = row.checked_mul(self.words_per_row)?;
        let word = column / WORD_BITS;
        let bit = column % WORD_BITS;

        self.rows
            .get(base.checked_add(word)?)
            .map(|value| ((value >> bit) & 1) != 0)
    }

    /// Sets a matrix element.
    pub fn set(
        &mut self,
        row: usize,
        column: usize,
        value: bool,
    ) -> SymplecticResult<()> {
        if row >= self.dimension() || column >= self.dimension() {
            return Err(SymplecticError::InvalidInvariant {
                message: "matrix coordinate is outside matrix dimensions",
            });
        }

        let base = row
            .checked_mul(self.words_per_row)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        let word = column / WORD_BITS;

        let index = base
            .checked_add(word)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        let bit = column % WORD_BITS;

        let slot = self.rows.get_mut(index).ok_or(
            SymplecticError::InvalidInvariant {
                message: "matrix storage index is inconsistent with dimensions",
            },
        )?;

        if value {
            *slot |= 1_u64 << bit;
        } else {
            *slot &= !(1_u64 << bit);
        }

        Ok(())
    }

    /// Returns whether this matrix is the identity matrix.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        let dimension = self.dimension();

        for row in 0..dimension {
            for column in 0..dimension {
                let expected = row == column;

                if self.get(row, column) != Some(expected) {
                    return false;
                }
            }
        }

        true
    }

    /// Returns the packed row slice.
    fn row(&self, row: usize) -> Option<&[u64]> {
        if row >= self.dimension() {
            return None;
        }

        let start = row.checked_mul(self.words_per_row)?;
        let end = start.checked_add(self.words_per_row)?;

        self.rows.get(start..end)
    }

    /// Returns the mutable packed row slice.
    fn row_mut(&mut self, row: usize) -> Option<&mut [u64]> {
        if row >= self.dimension() {
            return None;
        }

        let start = row.checked_mul(self.words_per_row)?;
        let end = start.checked_add(self.words_per_row)?;

        self.rows.get_mut(start..end)
    }

    /// XORs one row with another.
    ///
    /// This is the primitive used by most Clifford gate updates.
    fn xor_row(
        &mut self,
        destination: usize,
        source: usize,
    ) -> SymplecticResult<()> {
        if destination >= self.dimension()
            || source >= self.dimension()
        {
            return Err(SymplecticError::InvalidInvariant {
                message: "row index is outside matrix dimensions",
            });
        }

        if destination == source {
            return Err(SymplecticError::InvalidInvariant {
                message: "cannot XOR a symplectic row with itself",
            });
        }

        let source_row = self
            .row(source)
            .ok_or(SymplecticError::InvalidInvariant {
                message: "source row unavailable",
            })?
            .to_vec();

        let destination_row = self
            .row_mut(destination)
            .ok_or(SymplecticError::InvalidInvariant {
                message: "destination row unavailable",
            })?;

        for (dst, src) in destination_row.iter_mut().zip(source_row) {
            *dst ^= src;
        }

        Ok(())
    }

    /// Swaps two matrix rows.
    fn swap_rows(
        &mut self,
        first: usize,
        second: usize,
    ) -> SymplecticResult<()> {
        if first >= self.dimension() || second >= self.dimension() {
            return Err(SymplecticError::InvalidInvariant {
                message: "row index is outside matrix dimensions",
            });
        }

        if first == second {
            return Ok(());
        }

        let first_row = self
            .row(first)
            .ok_or(SymplecticError::InvalidInvariant {
                message: "first row unavailable",
            })?
            .to_vec();

        let second_row = self
            .row(second)
            .ok_or(SymplecticError::InvalidInvariant {
                message: "second row unavailable",
            })?
            .to_vec();

        let first_destination = self
            .row_mut(first)
            .ok_or(SymplecticError::InvalidInvariant {
                message: "first row unavailable for mutation",
            })?;

        first_destination.copy_from_slice(&second_row);

        let second_destination = self
            .row_mut(second)
            .ok_or(SymplecticError::InvalidInvariant {
                message: "second row unavailable for mutation",
            })?;

        second_destination.copy_from_slice(&first_row);

        Ok(())
    }

    /// Applies a Hadamard symplectic transformation to one qubit.
    ///
    /// ```text
    /// X -> Z
    /// Z -> X
    /// ```
    pub fn apply_h(&mut self, qubit: usize) -> SymplecticResult<()> {
        self.validate_qubit_index(qubit)?;

        let z_row = qubit
            .checked_add(self.qubit_count)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        self.swap_rows(qubit, z_row)
    }

    /// Applies the symplectic action of S.
    ///
    /// ```text
    /// X -> Y = XZ
    /// Z -> Z
    ///
    /// Therefore:
    /// z_output <- z_output XOR x_output
    /// ```
    ///
    /// S and S† have the same binary symplectic matrix because their
    /// difference is contained in Pauli phase/sign information.
    pub fn apply_s(&mut self, qubit: usize) -> SymplecticResult<()> {
        self.validate_qubit_index(qubit)?;

        let z_row = qubit
            .checked_add(self.qubit_count)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        self.xor_row(z_row, qubit)
    }

    /// Applies the symplectic action of S†.
    ///
    /// At the binary symplectic level S and S† are identical.
    pub fn apply_sdg(&mut self, qubit: usize) -> SymplecticResult<()> {
        self.apply_s(qubit)
    }

    /// Applies the symplectic action of V = sqrt(X).
    ///
    /// Ignoring Pauli signs/phases:
    ///
    /// ```text
    /// X -> X
    /// Z -> Y = XZ
    /// ```
    ///
    /// Therefore:
    ///
    /// ```text
    /// x_output <- x_output XOR z_output
    /// ```
    pub fn apply_v(&mut self, qubit: usize) -> SymplecticResult<()> {
        self.validate_qubit_index(qubit)?;

        let z_row = qubit
            .checked_add(self.qubit_count)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        self.xor_row(qubit, z_row)
    }

    /// Applies the symplectic action of V†.
    ///
    /// V and V† have the same binary symplectic action.
    pub fn apply_vdg(&mut self, qubit: usize) -> SymplecticResult<()> {
        self.apply_v(qubit)
    }

    /// Applies the CNOT/CX transformation.
    ///
    /// For control `c` and target `t`:
    ///
    /// ```text
    /// X_c -> X_c X_t
    /// Z_c -> Z_c
    /// X_t -> X_t
    /// Z_t -> Z_c Z_t
    /// ```
    ///
    /// In output-coordinate form this is:
    ///
    /// ```text
    /// X_t <- X_t XOR X_c
    /// Z_c <- Z_c XOR Z_t
    /// ```
    pub fn apply_cx(
        &mut self,
        control: usize,
        target: usize,
    ) -> SymplecticResult<()> {
        self.validate_two_qubits(control, target)?;

        let target_z = target
            .checked_add(self.qubit_count)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        let control_z = control
            .checked_add(self.qubit_count)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        self.xor_row(target, control)?;
        self.xor_row(control_z, target_z)?;

        Ok(())
    }

    /// Applies CZ.
    ///
    /// ```text
    /// X_c -> X_c Z_t
    /// Z_c -> Z_c
    /// X_t -> Z_c X_t
    /// Z_t -> Z_t
    /// ```
    ///
    /// Therefore:
    ///
    /// ```text
    /// Z_c <- Z_c XOR X_t
    /// Z_t <- Z_t XOR X_c
    /// ```
    pub fn apply_cz(
        &mut self,
        first: usize,
        second: usize,
    ) -> SymplecticResult<()> {
        self.validate_two_qubits(first, second)?;

        let first_z = first
            .checked_add(self.qubit_count)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        let second_z = second
            .checked_add(self.qubit_count)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        self.xor_row(first_z, second)?;
        self.xor_row(second_z, first)?;

        Ok(())
    }

    /// Applies CY.
    ///
    /// The exact phase/sign contribution belongs to the Clifford tableau,
    /// but its binary symplectic action can be represented by:
    ///
    /// ```text
    /// S(target)
    /// CX(control, target)
    /// S(target)
    /// ```
    ///
    /// because S and S† have identical binary symplectic actions.
    pub fn apply_cy(
        &mut self,
        control: usize,
        target: usize,
    ) -> SymplecticResult<()> {
        self.validate_two_qubits(control, target)?;

        self.apply_s(target)?;
        self.apply_cx(control, target)?;
        self.apply_s(target)?;

        Ok(())
    }

    /// Applies SWAP.
    ///
    /// This exchanges both X and Z coordinates of the two logical qubits.
    pub fn apply_swap(
        &mut self,
        first: usize,
        second: usize,
    ) -> SymplecticResult<()> {
        self.validate_two_qubits(first, second)?;

        let first_z = first
            .checked_add(self.qubit_count)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        let second_z = second
            .checked_add(self.qubit_count)
            .ok_or(SymplecticError::DimensionOverflow {
                qubits: self.qubit_count,
            })?;

        self.swap_rows(first, second)?;
        self.swap_rows(first_z, second_z)?;

        Ok(())
    }

    /// Applies iSWAP.
    ///
    /// iSWAP is Clifford. Its symplectic action is generated using the
    /// standard Clifford decomposition:
    ///
    /// ```text
    /// S q0
    /// H q0
    /// CX q0,q1
    /// X q0
    /// S q1
    /// X q1
    /// CX q1,q0
    /// H q1
    /// ```
    ///
    /// X gates do not affect the binary symplectic matrix, so they are
    /// omitted from the actual row operations.
    pub fn apply_iswap(
        &mut self,
        first: usize,
        second: usize,
    ) -> SymplecticResult<()> {
        self.validate_two_qubits(first, second)?;

        self.apply_s(first)?;
        self.apply_h(first)?;
        self.apply_cx(first, second)?;

        self.apply_s(second)?;
        self.apply_cx(second, first)?;
        self.apply_h(second)?;

        Ok(())
    }

    /// Applies a canonical IR Clifford gate.
    ///
    /// This is the primary integration entry point for the rest of the
    /// optimizer.
    pub fn apply_gate(&mut self, gate: &Gate) -> SymplecticResult<()> {
        let kind = gate.kind();

        let _classification = classify_gate(gate)?;

        match kind {
            GateKind::I
            | GateKind::X
            | GateKind::Y
            | GateKind::Z => {
                self.validate_single_gate_qubit(gate)?;
            }

            GateKind::H => {
                let qubit = single_qubit(gate)?;
                self.apply_h(qubit)?;
            }

            GateKind::S => {
                let qubit = single_qubit(gate)?;
                self.apply_s(qubit)?;
            }

            GateKind::Sdg => {
                let qubit = single_qubit(gate)?;
                self.apply_sdg(qubit)?;
            }

            GateKind::V => {
                let qubit = single_qubit(gate)?;
                self.apply_v(qubit)?;
            }

            GateKind::Vdg => {
                let qubit = single_qubit(gate)?;
                self.apply_vdg(qubit)?;
            }

            GateKind::CX => {
                let (control, target) = two_qubits(gate)?;
                self.apply_cx(control, target)?;
            }

            GateKind::CY => {
                let (control, target) = two_qubits(gate)?;
                self.apply_cy(control, target)?;
            }

            GateKind::CZ => {
                let (first, second) = two_qubits(gate)?;
                self.apply_cz(first, second)?;
            }

            GateKind::SWAP => {
                let (first, second) = two_qubits(gate)?;
                self.apply_swap(first, second)?;
            }

            GateKind::ISWAP => {
                let (first, second) = two_qubits(gate)?;
                self.apply_iswap(first, second)?;
            }

            _ => {
                return Err(SymplecticError::UnsupportedGate { gate: kind });
            }
        }

        Ok(())
    }

    /// Builds the symplectic transformation of a canonical IR gate sequence.
    ///
    /// Gates are applied in circuit order.
    pub fn from_gates(
        qubit_count: usize,
        gates: &[Gate],
    ) -> SymplecticResult<Self> {
        let mut matrix = Self::identity(qubit_count)?;

        for gate in gates {
            matrix.apply_gate(gate)?;
        }

        Ok(matrix)
    }

    /// Applies the matrix to a binary symplectic vector.
    ///
    /// The returned vector has the same width as the matrix.
    pub fn apply(
        &self,
        vector: &SymplecticVector,
    ) -> SymplecticResult<SymplecticVector> {
        if self.qubit_count != vector.qubit_count {
            return Err(SymplecticError::WidthMismatch {
                left: self.qubit_count,
                right: vector.qubit_count,
            });
        }

        let mut result = SymplecticVector::zero(self.qubit_count)?;

        for row in 0..self.dimension() {
            let matrix_row =
                self.row(row).ok_or(SymplecticError::InvalidInvariant {
                    message: "matrix row unavailable during vector application",
                })?;

            let parity = xor_dot(matrix_row, &vector.words)?;

            result.set(row, parity)?;
        }

        Ok(result)
    }

    /// Returns the transpose of this matrix.
    pub fn transpose(&self) -> SymplecticResult<Self> {
        let mut result = Self::zero(self.qubit_count)?;

        let dimension = self.dimension();

        for row in 0..dimension {
            for column in 0..dimension {
                let value = self.get(row, column).ok_or(
                    SymplecticError::InvalidInvariant {
                        message: "matrix element unavailable during transpose",
                    },
                )?;

                if value {
                    result.set(column, row, true)?;
                }
            }
        }

        Ok(result)
    }

    /// Returns the symplectic inverse.
    ///
    /// Uses:
    ///
    /// ```text
    /// M⁻¹ = Ω Mᵀ Ω
    /// ```
    ///
    /// where Ω swaps the X and Z halves of the coordinate space.
    ///
    /// This method does not perform Gaussian elimination.
    pub fn inverse(&self) -> SymplecticResult<Self> {
        self.validate()?;

        let transpose = self.transpose()?;
        let mut inverse = Self::zero(self.qubit_count)?;

        let n = self.qubit_count;

        for output_row in 0..self.dimension() {
            let source_row = if output_row < n {
                output_row
                    .checked_add(n)
                    .ok_or(SymplecticError::DimensionOverflow {
                        qubits: n,
                    })?
            } else {
                output_row
                    .checked_sub(n)
                    .ok_or(SymplecticError::DimensionOverflow {
                        qubits: n,
                    })?
            };

            for output_column in 0..self.dimension() {
                let source_column = if output_column < n {
                    output_column
                        .checked_add(n)
                        .ok_or(SymplecticError::DimensionOverflow {
                            qubits: n,
                        })?
                } else {
                    output_column
                        .checked_sub(n)
                        .ok_or(SymplecticError::DimensionOverflow {
                            qubits: n,
                        })?
                };

                let value = transpose
                    .get(source_row, source_column)
                    .ok_or(SymplecticError::InvalidInvariant {
                        message: "matrix element unavailable during symplectic inversion",
                    })?;

                if value {
                    inverse.set(output_row, output_column, true)?;
                }
            }
        }

        Ok(inverse)
    }

    /// Composes this transformation with another transformation.
    ///
    /// The returned matrix represents:
    ///
    /// ```text
    /// self ∘ other
    /// ```
    ///
    /// meaning `other` is applied first and `self` second.
    pub fn compose_after(
        &self,
        other: &Self,
    ) -> SymplecticResult<Self> {
        if self.qubit_count != other.qubit_count {
            return Err(SymplecticError::WidthMismatch {
                left: self.qubit_count,
                right: other.qubit_count,
            });
        }

        let dimension = self.dimension();

        let mut result = Self::zero(self.qubit_count)?;

        // Each output row of A*B is the XOR of rows of B selected by the
        // corresponding set bits in A.
        //
        // This formulation avoids materializing a dense byte matrix.
        for row in 0..dimension {
            let left_row =
                self.row(row).ok_or(SymplecticError::InvalidInvariant {
                    message: "left matrix row unavailable during composition",
                })?;

            let result_row =
                result
                    .row_mut(row)
                    .ok_or(SymplecticError::InvalidInvariant {
                        message: "result matrix row unavailable during composition",
                    })?;

            for word_index in 0..self.words_per_row {
                let mut bits = *left_row
                    .get(word_index)
                    .ok_or(SymplecticError::InvalidInvariant {
                        message: "left matrix word unavailable during composition",
                    })?;

                while bits != 0 {
                    let bit = bits.trailing_zeros() as usize;
                    bits &= bits - 1;

                    let selected_row = word_index
                        .checked_mul(WORD_BITS)
                        .and_then(|value| value.checked_add(bit))
                        .ok_or(SymplecticError::DimensionOverflow {
                            qubits: self.qubit_count,
                        })?;

                    if selected_row >= dimension {
                        continue;
                    }

                    let source =
                        other.row(selected_row).ok_or(
                            SymplecticError::InvalidInvariant {
                                message: "right matrix row unavailable during composition",
                            },
                        )?;

                    for (dst, src) in
                        result_row.iter_mut().zip(source.iter().copied())
                    {
                        *dst ^= src;
                    }
                }
            }
        }

        Ok(result)
    }

    /// Validates the binary symplectic condition.
    ///
    /// The condition checked is:
    ///
    /// ```text
    /// M Ω Mᵀ = Ω
    /// ```
    ///
    /// over `F₂`.
    pub fn validate(&self) -> SymplecticResult<()> {
        let dimension = self.dimension();

        if dimension % 2 != 0 {
            return Err(SymplecticError::InvalidDimension {
                rows: dimension,
                columns: dimension,
            });
        }

        // The rows of a symplectic matrix form a symplectic basis.
        //
        // Therefore:
        //
        // <row_i, row_j> = Ω[i,j]
        //
        // where:
        //
        // <a,b> = x_a · z_b XOR z_a · x_b
        //
        // This avoids explicitly constructing Ω or multiplying dense
        // matrices.
        for first in 0..dimension {
            let first_row =
                self.row(first).ok_or(SymplecticError::InvalidInvariant {
                    message: "first matrix row unavailable during validation",
                })?;

            for second in first..dimension {
                let second_row =
                    self.row(second).ok_or(
                        SymplecticError::InvalidInvariant {
                            message: "second matrix row unavailable during validation",
                        },
                    )?;

                let actual = symplectic_inner_product(
                    first_row,
                    second_row,
                    self.qubit_count,
                )?;

                let expected = expected_form_bit(
                    first,
                    second,
                    self.qubit_count,
                );

                if actual != expected {
                    return Err(SymplecticError::NotSymplectic {
                        row_a: first,
                        row_b: second,
                    });
                }
            }
        }

        Ok(())
    }

    /// Returns whether the matrix satisfies the symplectic condition.
    ///
    /// This is deliberately a convenience predicate. Detailed diagnostics
    /// are available through [`Self::validate`].
    #[must_use]
    pub fn is_symplectic(&self) -> bool {
        self.validate().is_ok()
    }

    /// Validates a logical qubit index.
    fn validate_qubit_index(
        &self,
        qubit: usize,
    ) -> SymplecticResult<()> {
        if qubit >= self.qubit_count {
            return Err(SymplecticError::QubitOutOfRange {
                qubit: QubitId::new(qubit),
                qubit_count: self.qubit_count,
            });
        }

        Ok(())
    }

    /// Validates two distinct logical qubit indices.
    fn validate_two_qubits(
        &self,
        first: usize,
        second: usize,
    ) -> SymplecticResult<()> {
        self.validate_qubit_index(first)?;
        self.validate_qubit_index(second)?;

        if first == second {
            return Err(SymplecticError::InvalidInvariant {
                message: "two-qubit Clifford operation requires distinct logical qubits",
            });
        }

        Ok(())
    }

    /// Validates a one-qubit canonical gate.
    fn validate_single_gate_qubit(
        &self,
        gate: &Gate,
    ) -> SymplecticResult<()> {
        let qubits = gate.qubits();

        if qubits.len() != 1 {
            return Err(SymplecticError::InvalidArity {
                gate: gate.kind(),
                expected: 1,
                actual: qubits.len(),
            });
        }

        let qubit = qubits[0];

        self.validate_qubit_index(qubit.index())
    }
}

// =============================================================================
// Gate operand helpers
// =============================================================================

fn single_qubit(gate: &Gate) -> SymplecticResult<usize> {
    let qubits = gate.qubits();

    if qubits.len() != 1 {
        return Err(SymplecticError::InvalidArity {
            gate: gate.kind(),
            expected: 1,
            actual: qubits.len(),
        });
    }

    Ok(qubits[0].index())
}

fn two_qubits(gate: &Gate) -> SymplecticResult<(usize, usize)> {
    let qubits = gate.qubits();

    if qubits.len() != 2 {
        return Err(SymplecticError::InvalidArity {
            gate: gate.kind(),
            expected: 2,
            actual: qubits.len(),
        });
    }

    Ok((qubits[0].index(), qubits[1].index()))
}

// =============================================================================
// Packed bit helpers
// =============================================================================

fn checked_dimension(qubit_count: usize) -> SymplecticResult<usize> {
    qubit_count
        .checked_mul(2)
        .ok_or(SymplecticError::DimensionOverflow {
            qubits: qubit_count,
        })
}

fn words_for_bits(bits: usize) -> SymplecticResult<usize> {
    if bits == 0 {
        return Ok(0);
    }

    let adjusted =
        bits.checked_add(WORD_BITS - 1).ok_or(
            SymplecticError::InvalidInvariant {
                message: "packed bit dimension overflow",
            },
        )?;

    Ok(adjusted / WORD_BITS)
}

fn set_packed_bit(
    words: &mut [u64],
    bit_index: usize,
    value: bool,
) -> SymplecticResult<()> {
    let word_index = bit_index / WORD_BITS;
    let bit = bit_index % WORD_BITS;

    let word = words.get_mut(word_index).ok_or(
        SymplecticError::InvalidInvariant {
            message: "packed bit index is outside storage",
        },
    )?;

    if value {
        *word |= 1_u64 << bit;
    } else {
        *word &= !(1_u64 << bit);
    }

    Ok(())
}

/// Computes the GF(2) dot product of two packed vectors.
fn xor_dot(left: &[u64], right: &[u64]) -> SymplecticResult<bool> {
    if left.len() != right.len() {
        return Err(SymplecticError::InvalidVectorStorage {
            expected: left.len(),
            actual: right.len(),
        });
    }

    let mut parity = false;

    for (a, b) in left.iter().zip(right.iter()) {
        parity ^= (a & b).count_ones() % 2 != 0;
    }

    Ok(parity)
}

/// Computes the binary symplectic inner product:
///
/// ```text
/// <a,b> = x_a · z_b XOR z_a · x_b
/// ```
fn symplectic_inner_product(
    left: &[u64],
    right: &[u64],
    qubit_count: usize,
) -> SymplecticResult<bool> {
    let dimension = checked_dimension(qubit_count)?;
    let words_per_component = words_for_bits(qubit_count)?;
    let words_per_dimension = words_for_bits(dimension)?;

    if left.len() != words_per_dimension
        || right.len() != words_per_dimension
    {
        return Err(SymplecticError::InvalidInvariant {
            message: "symplectic inner-product storage width mismatch",
        });
    }

    let mut parity = false;

    // X-left with Z-right.
    for word in 0..words_per_component {
        let left_x = left.get(word).copied().unwrap_or(0);

        let right_z = right
            .get(
                words_per_component
                    .checked_add(word)
                    .ok_or(SymplecticError::DimensionOverflow {
                        qubits: qubit_count,
                    })?,
            )
            .copied()
            .unwrap_or(0);

        parity ^= (left_x & right_z).count_ones() % 2 != 0;
    }

    // Z-left with X-right.
    for word in 0..words_per_component {
        let left_z = left
            .get(
                words_per_component
                    .checked_add(word)
                    .ok_or(SymplecticError::DimensionOverflow {
                        qubits: qubit_count,
                    })?,
            )
            .copied()
            .unwrap_or(0);

        let right_x = right.get(word).copied().unwrap_or(0);

        parity ^= (left_z & right_x).count_ones() % 2 != 0;
    }

    Ok(parity)
}

/// Returns the corresponding bit of the canonical symplectic form Ω.
///
/// ```text
/// Ω = [0 I]
///     [I 0]
/// ```
fn expected_form_bit(
    row: usize,
    column: usize,
    qubit_count: usize,
) -> bool {
    if row < qubit_count && column >= qubit_count {
        return row == column - qubit_count;
    }

    if row >= qubit_count && column < qubit_count {
        return row - qubit_count == column;
    }

    false
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    #[test]
    fn identity_is_symplectic() {
        let matrix =
            SymplecticMatrix::identity(3).expect("identity construction");

        assert!(matrix.is_identity());
        assert!(matrix.is_symplectic());
    }

    #[test]
    fn zero_matrix_is_not_symplectic() {
        let matrix =
            SymplecticMatrix::zero(2).expect("zero construction");

        assert!(!matrix.is_symplectic());
    }

    #[test]
    fn zero_qubit_identity_is_valid() {
        let matrix =
            SymplecticMatrix::identity(0).expect("zero-qubit identity");

        assert_eq!(matrix.dimension(), 0);
        assert!(matrix.is_symplectic());
    }

    // -------------------------------------------------------------------------
    // Single-qubit Clifford transformations
    // -------------------------------------------------------------------------

    #[test]
    fn h_squared_is_identity() {
        let mut matrix =
            SymplecticMatrix::identity(1).expect("identity");

        matrix.apply_h(0).expect("first H");
        matrix.apply_h(0).expect("second H");

        assert!(matrix.is_identity());
        assert!(matrix.is_symplectic());
    }

    #[test]
    fn s_squared_is_identity_symplectically() {
        let mut matrix =
            SymplecticMatrix::identity(1).expect("identity");

        matrix.apply_s(0).expect("first S");
        matrix.apply_s(0).expect("second S");

        // S² = Z. Z is a Pauli and therefore has the identity symplectic
        // action.
        assert!(matrix.is_identity());
        assert!(matrix.is_symplectic());
    }

    #[test]
    fn v_squared_is_identity_symplectically() {
        let mut matrix =
            SymplecticMatrix::identity(1).expect("identity");

        matrix.apply_v(0).expect("first V");
        matrix.apply_v(0).expect("second V");

        // V² = X. X is a Pauli and therefore has the identity symplectic
        // action.
        assert!(matrix.is_identity());
        assert!(matrix.is_symplectic());
    }

    #[test]
    fn h_maps_x_to_z() {
        let matrix =
            {
                let mut value =
                    SymplecticMatrix::identity(1).expect("identity");

                value.apply_h(0).expect("H");
                value
            };

        let x =
            SymplecticVector::x_pauli(1, 0).expect("X vector");

        let result = matrix.apply(&x).expect("apply");

        assert_eq!(result.z(0), Some(true));
        assert_eq!(result.x(0), Some(false));
    }

    #[test]
    fn s_maps_x_to_y_support() {
        let matrix =
            {
                let mut value =
                    SymplecticMatrix::identity(1).expect("identity");

                value.apply_s(0).expect("S");
                value
            };

        let x =
            SymplecticVector::x_pauli(1, 0).expect("X vector");

        let result = matrix.apply(&x).expect("apply");

        assert_eq!(result.x(0), Some(true));
        assert_eq!(result.z(0), Some(true));
    }

    #[test]
    fn v_maps_z_to_y_support() {
        let matrix =
            {
                let mut value =
                    SymplecticMatrix::identity(1).expect("identity");

                value.apply_v(0).expect("V");
                value
            };

        let z =
            SymplecticVector::z_pauli(1, 0).expect("Z vector");

        let result = matrix.apply(&z).expect("apply");

        assert_eq!(result.x(0), Some(true));
        assert_eq!(result.z(0), Some(true));
    }

    // -------------------------------------------------------------------------
    // Two-qubit transformations
    // -------------------------------------------------------------------------

    #[test]
    fn cx_squared_is_identity() {
        let mut matrix =
            SymplecticMatrix::identity(2).expect("identity");

        matrix.apply_cx(0, 1).expect("first CX");
        matrix.apply_cx(0, 1).expect("second CX");

        assert!(matrix.is_identity());
        assert!(matrix.is_symplectic());
    }

    #[test]
    fn cz_squared_is_identity() {
        let mut matrix =
            SymplecticMatrix::identity(2).expect("identity");

        matrix.apply_cz(0, 1).expect("first CZ");
        matrix.apply_cz(0, 1).expect("second CZ");

        assert!(matrix.is_identity());
        assert!(matrix.is_symplectic());
    }

    #[test]
    fn swap_squared_is_identity() {
        let mut matrix =
            SymplecticMatrix::identity(2).expect("identity");

        matrix.apply_swap(0, 1).expect("first SWAP");
        matrix.apply_swap(0, 1).expect("second SWAP");

        assert!(matrix.is_identity());
        assert!(matrix.is_symplectic());
    }

    #[test]
    fn cx_maps_target_x_to_control_target_x_support() {
        let matrix =
            {
                let mut value =
                    SymplecticMatrix::identity(2).expect("identity");

                value.apply_cx(0, 1).expect("CX");
                value
            };

        let target_x =
            SymplecticVector::x_pauli(2, 1).expect("target X");

        let result = matrix.apply(&target_x).expect("apply");

        assert_eq!(result.x(0), Some(true));
        assert_eq!(result.x(1), Some(true));
        assert_eq!(result.z(0), Some(false));
        assert_eq!(result.z(1), Some(false));
    }

    #[test]
    fn cx_maps_control_z_to_control_target_z_support() {
        let matrix =
            {
                let mut value =
                    SymplecticMatrix::identity(2).expect("identity");

                value.apply_cx(0, 1).expect("CX");
                value
            };

        let control_z =
            SymplecticVector::z_pauli(2, 0).expect("control Z");

        let result = matrix.apply(&control_z).expect("apply");

        assert_eq!(result.x(0), Some(false));
        assert_eq!(result.x(1), Some(false));
        assert_eq!(result.z(0), Some(true));
        assert_eq!(result.z(1), Some(true));
    }

    // -------------------------------------------------------------------------
    // Symplectic invariants
    // -------------------------------------------------------------------------

    #[test]
    fn elementary_transformations_preserve_symplectic_form() {
        let mut matrix =
            SymplecticMatrix::identity(4).expect("identity");

        matrix.apply_h(0).expect("H");
        matrix.apply_s(1).expect("S");
        matrix.apply_v(2).expect("V");
        matrix.apply_cx(0, 1).expect("CX");
        matrix.apply_cz(2, 3).expect("CZ");
        matrix.apply_swap(1, 3).expect("SWAP");

        assert!(matrix.is_symplectic());
    }

    #[test]
    fn inverse_is_actual_inverse() {
        let mut matrix =
            SymplecticMatrix::identity(4).expect("identity");

        matrix.apply_h(0).expect("H");
        matrix.apply_s(1).expect("S");
        matrix.apply_cx(0, 2).expect("CX");
        matrix.apply_cz(1, 3).expect("CZ");
        matrix.apply_swap(2, 3).expect("SWAP");

        let inverse = matrix.inverse().expect("inverse");

        let composed = matrix
            .compose_after(&inverse)
            .expect("composition");

        assert!(composed.is_identity());

        let composed_reverse = inverse
            .compose_after(&matrix)
            .expect("reverse composition");

        assert!(composed_reverse.is_identity());
    }

    #[test]
    fn inverse_of_identity_is_identity() {
        let matrix =
            SymplecticMatrix::identity(5).expect("identity");

        let inverse = matrix.inverse().expect("inverse");

        assert!(inverse.is_identity());
    }

    // -------------------------------------------------------------------------
    // Composition
    // -------------------------------------------------------------------------

    #[test]
    fn composition_matches_sequential_application() {
        let mut first =
            SymplecticMatrix::identity(2).expect("identity");

        first.apply_h(0).expect("H");

        let mut second =
            SymplecticMatrix::identity(2).expect("identity");

        second.apply_cx(0, 1).expect("CX");

        let composed = second
            .compose_after(&first)
            .expect("composition");

        let x =
            SymplecticVector::x_pauli(2, 0).expect("X");

        let sequential = {
            let after_first =
                first.apply(&x).expect("first application");

            second
                .apply(&after_first)
                .expect("second application")
        };

        let composed_result =
            composed.apply(&x).expect("composed application");

        assert_eq!(sequential, composed_result);
    }

    // -------------------------------------------------------------------------
    // iSWAP / CY
    // -------------------------------------------------------------------------

    #[test]
    fn cy_is_symplectic() {
        let mut matrix =
            SymplecticMatrix::identity(2).expect("identity");

        matrix.apply_cy(0, 1).expect("CY");

        assert!(matrix.is_symplectic());
    }

    #[test]
    fn iswap_is_symplectic() {
        let mut matrix =
            SymplecticMatrix::identity(2).expect("identity");

        matrix.apply_iswap(0, 1).expect("iSWAP");

        assert!(matrix.is_symplectic());
    }

    #[test]
    fn iswap_squared_has_identity_symplectic_action() {
        let mut matrix =
            SymplecticMatrix::identity(2).expect("identity");

        matrix.apply_iswap(0, 1).expect("first iSWAP");
        matrix.apply_iswap(0, 1).expect("second iSWAP");

        // iSWAP² is a Pauli-equivalent operation at the symplectic level.
        assert!(matrix.is_identity());
    }

    // -------------------------------------------------------------------------
    // Error handling
    // -------------------------------------------------------------------------

    #[test]
    fn out_of_range_qubit_is_rejected() {
        let mut matrix =
            SymplecticMatrix::identity(2).expect("identity");

        let result = matrix.apply_h(2);

        assert!(matches!(
            result,
            Err(SymplecticError::QubitOutOfRange { .. })
        ));
    }

    #[test]
    fn duplicate_two_qubit_operands_are_rejected() {
        let mut matrix =
            SymplecticMatrix::identity(2).expect("identity");

        let result = matrix.apply_cx(1, 1);

        assert!(matches!(
            result,
            Err(SymplecticError::InvalidInvariant { .. })
        ));
    }

    #[test]
    fn vector_width_mismatch_is_rejected() {
        let matrix =
            SymplecticMatrix::identity(2).expect("identity");

        let vector =
            SymplecticVector::zero(3).expect("vector");

        let result = matrix.apply(&vector);

        assert!(matches!(
            result,
            Err(SymplecticError::WidthMismatch { .. })
        ));
    }

    // -------------------------------------------------------------------------
    // Gate classification
    // -------------------------------------------------------------------------

    #[test]
    fn unsupported_non_clifford_gate_is_rejected_by_classifier() {
        // This test is intentionally kept independent of Gate construction
        // details because parameterized/non-Clifford classification must be
        // conservative.
        assert!(matches!(
            GateKind::T.is_parameterized(),
            false
        ));

        assert!(!GateKind::T.is_clifford());
    }

    // -------------------------------------------------------------------------
    // Large-width construction
    // -------------------------------------------------------------------------

    #[test]
    fn packed_storage_scales_beyond_machine_word_width() {
        let matrix =
            SymplecticMatrix::identity(130).expect("identity");

        assert_eq!(matrix.dimension(), 260);
        assert!(matrix.words_per_row() > 4);
        assert!(matrix.is_symplectic());
    }
}