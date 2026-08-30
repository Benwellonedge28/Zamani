//! Zamani Quantum Optimization — Phase Polynomial Algebra
//!
//! Production-grade, hardware-independent phase-polynomial representation and
//! extraction for the Zamani quantum optimizer.
//!
//! # Architectural role
//!
//! This module represents the diagonal phase component of circuits whose
//! computational-basis action can be expressed using affine Boolean parity
//! functions.
//!
//! The principal representation is:
//!
//! ```text
//! U |x> = exp(i * p(x)) |A(x)>
//!
//! p(x) = c + Σ θ_j * f_j(x)
//!
//! ```
//!
//! where:
//!
//! - `x` is the original computational-basis input;
//! - `A(x)` is an affine reversible transformation;
//! - `c` is a global phase;
//! - `f_j(x)` is an affine Boolean parity;
//! - `θ_j` is a Zamani [`Parameter`].
//!
//! This representation is particularly useful for:
//!
//! - CNOT + RZ optimization;
//! - CNOT + phase optimization;
//! - Clifford+T optimization;
//! - T-count reduction;
//! - phase folding;
//! - parity-table construction;
//! - CNOT synthesis;
//! - phase-gadget optimization;
//! - diagonal circuit analysis;
//! - QAOA diagonal blocks;
//! - Hamiltonian-simulation diagonal blocks;
//! - arithmetic and modular-exponentiation subcircuits.
//!
//! # Important semantic distinction
//!
//! A phase polynomial is NOT a replacement for Zamani's canonical Quantum IR.
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! optimization::algebra::phase_polynomial
//!      │
//!      ├── algebraic optimization
//!      ├── parity extraction
//!      └── phase folding
//!      │
//!      ▼
//! synthesis::phase
//!      │
//!      ▼
//! quantum::ir
//! ```
//!
//! This module therefore does not own:
//!
//! - quantum gates;
//! - quantum circuits;
//! - routing;
//! - hardware topology;
//! - scheduling;
//! - QPU execution;
//! - pulse generation;
//! - error-correction codes;
//! - frontend parsing;
//! - backend APIs;
//! - final gate synthesis.
//!
//! # Canonical representation
//!
//! A Boolean parity is represented by a packed bit vector.
//!
//! For example:
//!
//! ```text
//! x0 ⊕ x2 ⊕ x7
//! ```
//!
//! is represented by a bit mask with bits:
//!
//! ```text
//! 0, 2, 7 = 1
//! ```
//!
//! An affine parity additionally contains a constant bit:
//!
//! ```text
//! 1 ⊕ x0 ⊕ x2
//! ```
//!
//! The packed representation scales as O(n / 64), rather than allocating one
//! object per qubit.
//!
//! # Why affine rather than only linear parities?
//!
//! Supporting the affine constant explicitly allows this module to represent
//! the complete CNOT-dihedral family containing NOT/X operations:
//!
//! ```text
//! X
//! CNOT
//! RZ
//! Phase
//! S
//! S†
//! T
//! T†
//! Z
//! CZ
//! ```
//!
//! An X operation changes a tracked wire from `f(x)` to `1 ⊕ f(x)`.
//! That transformation must not be discarded.
//!
//! # Global phase
//!
//! Global phase is retained explicitly.
//!
//! This is important because:
//!
//! ```text
//! RZ(θ) = exp(-iθ/2) * diag(1, exp(iθ))
//! ```
//!
//! Therefore an RZ contributes both:
//!
//! ```text
//! global phase: -θ/2
//! parity phase: +θ * x
//! ```
//!
//! Discarding the first component would make this representation unsuitable
//! for exact unitary equivalence.
//!
//! A later optimization layer may deliberately compare circuits modulo global
//! phase, but that decision must be explicit and must not be baked into this
//! algebra.
//!
//! # CZ representation
//!
//! A CZ gate contributes:
//!
//! ```text
//! exp(iπ x y)
//! ```
//!
//! and the Boolean identity
//!
//! ```text
//! x y = (x + y - (x ⊕ y)) / 2
//! ```
//!
//! gives a representation entirely in affine parity terms.
//!
//! The implementation uses this identity exactly for CZ.
//!
//! # Symbolic parameters
//!
//! Coefficients are stored as the canonical Quantum IR [`Parameter`] type.
//!
//! Therefore this module supports:
//!
//! ```text
//! θ
//! θ + φ
//! 2θ
//! θ - φ
//! -θ
//! ```
//!
//! without requiring parameters to be numerically bound first.
//!
//! Constant coefficients are folded and normalized modulo 2π where possible.
//! Symbolic expressions are preserved exactly rather than approximated.
//!
//! # Extraction contract
//!
//! [`PhasePolynomial::from_gates`] accepts only operations that can be
//! represented exactly by this model.
//!
//! Supported:
//!
//! - I
//! - X
//! - Z
//! - S
//! - Sdg
//! - T
//! - Tdg
//! - RX/RY are rejected
//! - RZ
//! - Phase
//! - U1
//! - CNOT/CX
//! - CZ
//!
//! Measurements, reset, barriers, non-diagonal gates, and unsupported
//! controlled rotations terminate extraction with a structured error.
//!
//! This is intentional. Silently ignoring a gate would create an invalid
//! optimization representation.
//!
//! # Complexity
//!
//! Let:
//!
//! ```text
//! n = number of logical qubits
//! w = ceil(n / 64)
//! m = number of distinct parity terms
//! g = number of extracted gates
//! ```
//!
//! Packed parity operations are O(w).
//!
//! Extraction is approximately:
//!
//! ```text
//! O(g * w)
//! ```
//!
//! with hash-map aggregation depending on the number of distinct terms.
//!
//! Canonical folding is approximately:
//!
//! ```text
//! O(m * w)
//! ```
//!
//! No artificial maximum number of qubits is imposed here. Memory and runtime
//! limits belong to the optimizer's `OptimizationLimits` layer.
//!
//! # Determinism
//!
//! This module does not depend on global state, random numbers, wall-clock
//! time, thread-local state, or hash-map iteration order for semantic output.
//!
//! Canonical serialization and ordered iteration use `BTreeMap`.
//!
//! # Safety
//!
//! - No `unsafe`.
//! - No raw pointers.
//! - No global mutable state.
//! - No unchecked public indexing.
//! - No floating-point indexing.
//! - Checked allocation-size arithmetic.
//! - Checked parameter construction.
//!
//! # Rust compatibility
//!
//! Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features.
//! No external dependencies.
//!
//! # Integration
//!
//! This module directly consumes:
//!
//! ```text
//! quantum::ir::Gate
//! quantum::ir::GateKind
//! quantum::ir::Parameter
//! quantum::ir::ParameterExpression
//! ```
//!
//! It does not require later optimizer modules.
//!
//! Future integration points:
//!
//! ```text
//! algebra::clifford
//!     → may use phase-polynomial extraction
//!
//! algebra::symplectic
//!     → may consume parity masks
//!
//! fault_tolerant::t_count
//!     → may inspect normalized coefficients
//!
//! fault_tolerant::t_depth
//!     → may inspect parity dependencies
//!
//! synthesis::phase
//!     → consumes canonical terms and emits Quantum IR
//!
//! passes::optimize_fault_tolerance
//!     → can invoke folding and canonicalization
//! ```
//!
//! No changes to this file are required merely because those modules are later
//! implemented.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::quantum::ir::gate::{Gate, GateKind};
use crate::quantum::ir::parameter::{Parameter, ParameterExpression};

// =============================================================================
// Constants
// =============================================================================

const WORD_BITS: usize = 64;
const TWO_PI: f64 = std::f64::consts::PI * 2.0;
const PI: f64 = std::f64::consts::PI;
const HALF_PI: f64 = std::f64::consts::FRAC_PI_2;
const QUARTER_PI: f64 = std::f64::consts::FRAC_PI_4;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by phase-polynomial construction or extraction.
#[derive(Debug, Clone, PartialEq)]
pub enum PhasePolynomialError {
    /// The requested qubit count cannot be represented safely.
    QubitCountOverflow {
        qubits: usize,
    },

    /// A qubit index is outside the polynomial's logical namespace.
    QubitOutOfRange {
        qubit: usize,
        qubits: usize,
    },

    /// Two phase-polynomial objects have different dimensions.
    DimensionMismatch {
        left: usize,
        right: usize,
    },

    /// The supplied gate cannot be represented exactly by this phase-polynomial
    /// model.
    UnsupportedGate {
        gate: GateKind,
    },

    /// A gate has an invalid operand structure.
    InvalidGateOperands {
        gate: GateKind,
        expected: usize,
        actual: usize,
    },

    /// A gate has an invalid parameter structure.
    InvalidGateParameters {
        gate: GateKind,
        expected: usize,
        actual: usize,
    },

    /// A gate parameter could not be converted into a valid symbolic parameter.
    InvalidParameter {
        gate: GateKind,
        index: usize,
    },

    /// A gate parameter is required to be finite but is not.
    NonFiniteParameter {
        gate: GateKind,
        index: usize,
    },

    /// Internal checked arithmetic failed.
    ArithmeticOverflow,

    /// A collection size cannot be represented safely.
    AllocationSizeOverflow,

    /// A coefficient cannot be represented by the IR parameter model.
    ParameterConstructionFailure,

    /// The polynomial contains a representation that violates its invariants.
    InvalidRepresentation {
        message: &'static str,
    },
}

impl fmt::Display for PhasePolynomialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QubitCountOverflow { qubits } => {
                write!(f, "phase-polynomial qubit count overflow: {qubits}")
            }

            Self::QubitOutOfRange { qubit, qubits } => {
                write!(
                    f,
                    "phase-polynomial qubit {qubit} is outside {qubits} qubits"
                )
            }

            Self::DimensionMismatch { left, right } => {
                write!(
                    f,
                    "phase-polynomial dimension mismatch: {left} != {right}"
                )
            }

            Self::UnsupportedGate { gate } => {
                write!(
                    f,
                    "gate {gate:?} cannot be represented by a phase polynomial"
                )
            }

            Self::InvalidGateOperands {
                gate,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "gate {gate:?} requires {expected} operands, received {actual}"
                )
            }

            Self::InvalidGateParameters {
                gate,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "gate {gate:?} requires {expected} parameters, received {actual}"
                )
            }

            Self::InvalidParameter { gate, index } => {
                write!(
                    f,
                    "gate {gate:?} contains an invalid parameter at index {index}"
                )
            }

            Self::NonFiniteParameter { gate, index } => {
                write!(
                    f,
                    "gate {gate:?} contains a non-finite parameter at index {index}"
                )
            }

            Self::ArithmeticOverflow => {
                write!(f, "phase-polynomial arithmetic overflow")
            }

            Self::AllocationSizeOverflow => {
                write!(f, "phase-polynomial allocation size overflow")
            }

            Self::ParameterConstructionFailure => {
                write!(f, "failed to construct a valid IR parameter")
            }

            Self::InvalidRepresentation { message } => {
                write!(f, "invalid phase-polynomial representation: {message}")
            }
        }
    }
}

impl Error for PhasePolynomialError {}

/// Result type used by this module.
pub type PhasePolynomialResult<T> = Result<T, PhasePolynomialError>;

// =============================================================================
// Packed parity mask
// =============================================================================

/// A packed Boolean parity mask.
///
/// Bit `q` represents whether logical input qubit `q` participates in the
/// parity.
///
/// ```text
/// x0 ⊕ x3 ⊕ x9
/// ```
///
/// is stored as bits 0, 3 and 9.
///
/// The representation is immutable from the outside and therefore cannot
/// violate the logical qubit count invariant.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ParityMask {
    words: Vec<u64>,
    qubits: usize,
}

impl fmt::Debug for ParityMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ParityMask")
            .field("qubits", &self.qubits)
            .field("weight", &self.weight())
            .finish()
    }
}

impl ParityMask {
    /// Returns the number of packed words needed for `qubits`.
    fn word_count(qubits: usize) -> PhasePolynomialResult<usize> {
        qubits
            .checked_add(WORD_BITS - 1)
            .ok_or(PhasePolynomialError::QubitCountOverflow { qubits })
            .map(|value| value / WORD_BITS)
    }

    /// Creates the zero parity.
    pub fn zero(qubits: usize) -> PhasePolynomialResult<Self> {
        let words = Self::word_count(qubits)?;

        Ok(Self {
            words: vec![0; words],
            qubits,
        })
    }

    /// Creates a parity containing exactly one qubit.
    pub fn single(qubits: usize, qubit: usize) -> PhasePolynomialResult<Self> {
        let mut result = Self::zero(qubits)?;
        result.set(qubit, true)?;
        Ok(result)
    }

    /// Creates a parity from qubit indices.
    ///
    /// Duplicate indices cancel because parity is over GF(2).
    pub fn from_qubits<I>(
        qubits: usize,
        indices: I,
    ) -> PhasePolynomialResult<Self>
    where
        I: IntoIterator<Item = usize>,
    {
        let mut result = Self::zero(qubits)?;

        for index in indices {
            let current = result.get(index);
            result.set(index, !current)?;
        }

        Ok(result)
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    /// Returns whether the parity contains no variables.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.words.iter().all(|word| *word == 0)
    }

    /// Returns whether a logical qubit participates in this parity.
    #[must_use]
    pub fn get(&self, qubit: usize) -> bool {
        if qubit >= self.qubits {
            return false;
        }

        let word = qubit / WORD_BITS;
        let bit = qubit % WORD_BITS;

        ((self.words[word] >> bit) & 1) != 0
    }

    /// Sets or clears one qubit.
    pub fn set(
        &mut self,
        qubit: usize,
        value: bool,
    ) -> PhasePolynomialResult<()> {
        if qubit >= self.qubits {
            return Err(PhasePolynomialError::QubitOutOfRange {
                qubit,
                qubits: self.qubits,
            });
        }

        let word = qubit / WORD_BITS;
        let bit = qubit % WORD_BITS;
        let mask = 1u64 << bit;

        if value {
            self.words[word] |= mask;
        } else {
            self.words[word] &= !mask;
        }

        Ok(())
    }

    /// Returns the packed words.
    ///
    /// This is read-only and allocation-free.
    #[must_use]
    pub fn words(&self) -> &[u64] {
        &self.words
    }

    /// Returns the number of variables in the parity.
    #[must_use]
    pub fn weight(&self) -> usize {
        self.words
            .iter()
            .map(|word| word.count_ones() as usize)
            .sum()
    }

    /// Returns the highest participating qubit.
    #[must_use]
    pub fn highest_qubit(&self) -> Option<usize> {
        for word_index in (0..self.words.len()).rev() {
            let word = self.words[word_index];

            if word != 0 {
                let highest = WORD_BITS - 1 - word.leading_zeros() as usize;

                return word_index
                    .checked_mul(WORD_BITS)
                    .and_then(|base| base.checked_add(highest))
                    .filter(|index| *index < self.qubits);
            }
        }

        None
    }

    /// XORs this parity with another.
    pub fn xor(
        &self,
        rhs: &Self,
    ) -> PhasePolynomialResult<Self> {
        if self.qubits != rhs.qubits {
            return Err(PhasePolynomialError::DimensionMismatch {
                left: self.qubits,
                right: rhs.qubits,
            });
        }

        let mut result = self.clone();

        for (left, right) in result.words.iter_mut().zip(rhs.words.iter()) {
            *left ^= *right;
        }

        Ok(result)
    }

    /// Returns whether this parity equals another parity.
    #[must_use]
    pub fn equals(&self, rhs: &Self) -> bool {
        self == rhs
    }

    /// Returns the parity as sorted qubit indices.
    ///
    /// This allocates O(weight).
    pub fn to_qubits(&self) -> Vec<usize> {
        let mut result = Vec::with_capacity(self.weight());

        for word_index in 0..self.words.len() {
            let mut word = self.words[word_index];

            while word != 0 {
                let offset = word.trailing_zeros() as usize;

                if let Some(index) = word_index
                    .checked_mul(WORD_BITS)
                    .and_then(|base| base.checked_add(offset))
                    .filter(|index| *index < self.qubits)
                {
                    result.push(index);
                }

                word &= word - 1;
            }
        }

        result
    }

    /// Returns a compact deterministic textual representation.
    pub fn to_string_repr(&self) -> String {
        let indices = self.to_qubits();

        if indices.is_empty() {
            return "0".to_owned();
        }

        let mut result = String::new();

        for (index, qubit) in indices.iter().enumerate() {
            if index != 0 {
                result.push('^');
            }

            result.push('x');

            result.push_str(&qubit.to_string());
        }

        result
    }
}

impl Ord for ParityMask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.qubits
            .cmp(&other.qubits)
            .then_with(|| self.words.cmp(&other.words))
    }
}

impl PartialOrd for ParityMask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for ParityMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string_repr())
    }
}

// =============================================================================
// Affine parity
// =============================================================================

/// An affine Boolean parity:
///
/// ```text
/// c ⊕ x_i ⊕ x_j ⊕ ...
/// ```
///
/// where `c` is either zero or one.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AffineParity {
    mask: ParityMask,
    constant: bool,
}

impl AffineParity {
    /// Creates a zero affine parity.
    pub fn zero(qubits: usize) -> PhasePolynomialResult<Self> {
        Ok(Self {
            mask: ParityMask::zero(qubits)?,
            constant: false,
        })
    }

    /// Creates a single input variable.
    pub fn variable(
        qubits: usize,
        qubit: usize,
    ) -> PhasePolynomialResult<Self> {
        Ok(Self {
            mask: ParityMask::single(qubits, qubit)?,
            constant: false,
        })
    }

    /// Creates an affine parity from a mask and constant.
    pub fn new(
        mask: ParityMask,
        constant: bool,
    ) -> Self {
        Self { mask, constant }
    }

    /// Returns the underlying linear parity mask.
    #[must_use]
    pub fn mask(&self) -> &ParityMask {
        &self.mask
    }

    /// Returns the affine constant bit.
    #[must_use]
    pub const fn constant(&self) -> bool {
        self.constant
    }

    /// Returns the logical-qubit count.
    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.mask.qubits()
    }

    /// Returns the affine XOR of two parities.
    pub fn xor(
        &self,
        rhs: &Self,
    ) -> PhasePolynomialResult<Self> {
        Ok(Self {
            mask: self.mask.xor(&rhs.mask)?,
            constant: self.constant ^ rhs.constant,
        })
    }

    /// Returns whether this affine function is identically zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        !self.constant && self.mask.is_zero()
    }

    /// Returns whether this affine function is identically one.
    #[must_use]
    pub fn is_one(&self) -> bool {
        self.constant && self.mask.is_zero()
    }

    /// Returns the variable indices.
    pub fn to_qubits(&self) -> Vec<usize> {
        self.mask.to_qubits()
    }

    /// Returns a deterministic textual representation.
    pub fn to_string_repr(&self) -> String {
        let mut result = String::new();

        if self.constant {
            result.push('1');

            if !self.mask.is_zero() {
                result.push('^');
            }
        }

        result.push_str(&self.mask.to_string_repr());

        result
    }
}

impl fmt::Display for AffineParity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string_repr())
    }
}

// =============================================================================
// Phase term
// =============================================================================

/// One phase-polynomial term.
///
/// Semantically:
///
/// ```text
/// coefficient * affine_parity(x)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PhaseTerm {
    parity: AffineParity,
    coefficient: Parameter,
}

impl PhaseTerm {
    /// Creates a phase term.
    pub fn new(
        parity: AffineParity,
        coefficient: Parameter,
    ) -> PhasePolynomialResult<Self> {
        coefficient
            .validate()
            .map_err(|_| PhasePolynomialError::ParameterConstructionFailure)?;

        Ok(Self {
            parity,
            coefficient,
        })
    }

    /// Returns the affine parity.
    #[must_use]
    pub fn parity(&self) -> &AffineParity {
        &self.parity
    }

    /// Returns the coefficient.
    #[must_use]
    pub fn coefficient(&self) -> &Parameter {
        &self.coefficient
    }

    /// Consumes the term and returns its parts.
    #[must_use]
    pub fn into_parts(self) -> (AffineParity, Parameter) {
        (self.parity, self.coefficient)
    }
}

// =============================================================================
// Extraction statistics
// =============================================================================

/// Statistics collected while extracting a phase polynomial.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PhasePolynomialStatistics {
    /// Number of gates examined.
    pub gates_examined: usize,

    /// Number of gates represented exactly.
    pub gates_encoded: usize,

    /// Number of phase-producing gates.
    pub phase_gates: usize,

    /// Number of CNOT/CX gates.
    pub cnot_gates: usize,

    /// Number of X gates.
    pub x_gates: usize,

    /// Number of CZ gates.
    pub cz_gates: usize,

    /// Number of terms before folding.
    pub raw_terms: usize,

    /// Number of distinct terms after folding.
    pub folded_terms: usize,

    /// Number of terms eliminated because their coefficient became zero.
    pub eliminated_terms: usize,
}

// =============================================================================
// Phase polynomial
// =============================================================================

/// Canonical phase polynomial.
///
/// The polynomial contains:
///
/// ```text
/// global_phase + Σ coefficient * affine_parity
/// ```
///
/// Terms with equal affine parities are always folded together.
#[derive(Debug, Clone, PartialEq)]
pub struct PhasePolynomial {
    qubits: usize,
    global_phase: Parameter,
    terms: BTreeMap<AffineParityKey, Parameter>,
}

/// Stable map key for affine parities.
///
/// Keeping the key separate from `AffineParity` makes it impossible for
/// coefficient manipulation to accidentally alter the identity used for map
/// lookup.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct AffineParityKey {
    mask: ParityMask,
    constant: bool,
}

impl AffineParityKey {
    fn from_parity(parity: &AffineParity) -> Self {
        Self {
            mask: parity.mask.clone(),
            constant: parity.constant,
        }
    }

    fn into_parity(self) -> AffineParity {
        AffineParity::new(self.mask, self.constant)
    }
}

impl PhasePolynomial {
    /// Creates an empty phase polynomial over `qubits` inputs.
    pub fn new(qubits: usize) -> PhasePolynomialResult<Self> {
        Ok(Self {
            qubits,
            global_phase: zero_parameter()?,
            terms: BTreeMap::new(),
        })
    }

    /// Creates a phase polynomial with the supplied global phase.
    pub fn with_global_phase(
        qubits: usize,
        global_phase: Parameter,
    ) -> PhasePolynomialResult<Self> {
        global_phase
            .validate()
            .map_err(|_| PhasePolynomialError::ParameterConstructionFailure)?;

        Ok(Self {
            qubits,
            global_phase,
            terms: BTreeMap::new(),
        })
    }

    /// Extracts a phase polynomial from canonical Quantum IR gates.
    ///
    /// The input is treated as an ordered logical circuit.
    ///
    /// The affine state of every qubit is tracked relative to the original
    /// computational-basis inputs.
    ///
    /// The method is intentionally strict: encountering an unsupported gate
    /// returns an error rather than silently dropping it.
    pub fn from_gates(
        gates: &[Gate],
        qubits: usize,
    ) -> PhasePolynomialResult<(Self, PhasePolynomialStatistics)> {
        let mut polynomial = Self::new(qubits)?;
        let mut wires = Vec::with_capacity(qubits);

        for qubit in 0..qubits {
            wires.push(AffineParity::variable(qubits, qubit)?);
        }

        let mut statistics = PhasePolynomialStatistics::default();

        for gate in gates {
            statistics.gates_examined =
                statistics
                    .gates_examined
                    .checked_add(1)
                    .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

            polynomial.apply_gate(
                gate,
                &mut wires,
                &mut statistics,
            )?;
        }

        statistics.raw_terms = polynomial.terms.len();

        // The representation is already folded incrementally.
        statistics.folded_terms = polynomial.terms.len();

        Ok((polynomial, statistics))
    }

    /// Applies one gate to the phase-polynomial state.
    fn apply_gate(
        &mut self,
        gate: &Gate,
        wires: &mut [AffineParity],
        statistics: &mut PhasePolynomialStatistics,
    ) -> PhasePolynomialResult<()> {
        match gate.kind() {
            GateKind::I => {
                require_operands(gate, 1)?;

                statistics.gates_encoded =
                    statistics
                        .gates_encoded
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;
            }

            GateKind::X => {
                require_operands(gate, 1)?;

                let qubit = gate.qubits()[0].index();

                ensure_qubit(qubit, self.qubits)?;

                wires[qubit].constant ^= true;

                statistics.x_gates =
                    statistics
                        .x_gates
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

                statistics.gates_encoded =
                    statistics
                        .gates_encoded
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;
            }

            GateKind::Z => {
                require_operands(gate, 1)?;

                let qubit = gate.qubits()[0].index();
                ensure_qubit(qubit, self.qubits)?;

                let parity = wires[qubit].clone();

                self.add_phase(parity, constant_parameter(PI)?)?;

                statistics.phase_gates =
                    statistics
                        .phase_gates
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

                statistics.gates_encoded =
                    statistics
                        .gates_encoded
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;
            }

            GateKind::S => {
                require_operands(gate, 1)?;

                let qubit = gate.qubits()[0].index();
                ensure_qubit(qubit, self.qubits)?;

                self.add_phase(
                    wires[qubit].clone(),
                    constant_parameter(HALF_PI)?,
                )?;

                statistics.phase_gates =
                    statistics
                        .phase_gates
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

                statistics.gates_encoded =
                    statistics
                        .gates_encoded
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;
            }

            GateKind::Sdg => {
                require_operands(gate, 1)?;

                let qubit = gate.qubits()[0].index();
                ensure_qubit(qubit, self.qubits)?;

                self.add_phase(
                    wires[qubit].clone(),
                    constant_parameter(-HALF_PI)?,
                )?;

                statistics.phase_gates =
                    statistics
                        .phase_gates
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

                statistics.gates_encoded =
                    statistics
                        .gates_encoded
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;
            }

            GateKind::T => {
                require_operands(gate, 1)?;

                let qubit = gate.qubits()[0].index();
                ensure_qubit(qubit, self.qubits)?;

                self.add_phase(
                    wires[qubit].clone(),
                    constant_parameter(QUARTER_PI)?,
                )?;

                statistics.phase_gates =
                    statistics
                        .phase_gates
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

                statistics.gates_encoded =
                    statistics
                        .gates_encoded
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;
            }

            GateKind::Tdg => {
                require_operands(gate, 1)?;

                let qubit = gate.qubits()[0].index();
                ensure_qubit(qubit, self.qubits)?;

                self.add_phase(
                    wires[qubit].clone(),
                    constant_parameter(-QUARTER_PI)?,
                )?;

                statistics.phase_gates =
                    statistics
                        .phase_gates
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

                statistics.gates_encoded =
                    statistics
                        .gates_encoded
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;
            }

            GateKind::RZ => {
                require_operands(gate, 1)?;
                require_parameters(gate, 1)?;

                let qubit = gate.qubits()[0].index();
                ensure_qubit(qubit, self.qubits)?;

                let theta = parameter_from_gate(gate, 0)?;

                // RZ(theta) = exp(-i theta / 2)
                //            * diag(1, exp(i theta)).
                self.add_global(negate_half(theta.clone())?)?;

                self.add_phase(wires[qubit].clone(), theta)?;

                statistics.phase_gates =
                    statistics
                        .phase_gates
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

                statistics.gates_encoded =
                    statistics
                        .gates_encoded
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;
            }

            GateKind::Phase | GateKind::U1 => {
                require_operands(gate, 1)?;
                require_parameters(gate, 1)?;

                let qubit = gate.qubits()[0].index();
                ensure_qubit(qubit, self.qubits)?;

                let theta = parameter_from_gate(gate, 0)?;

                self.add_phase(wires[qubit].clone(), theta)?;

                statistics.phase_gates =
                    statistics
                        .phase_gates
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

                statistics.gates_encoded =
                    statistics
                        .gates_encoded
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;
            }

            GateKind::CX => {
                require_operands(gate, 2)?;

                let control = gate.qubits()[0].index();
                let target = gate.qubits()[1].index();

                ensure_qubit(control, self.qubits)?;
                ensure_qubit(target, self.qubits)?;

                let updated =
                    wires[control].xor(&wires[target])?;

                wires[target] = updated;

                statistics.cnot_gates =
                    statistics
                        .cnot_gates
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

                statistics.gates_encoded =
                    statistics
                        .gates_encoded
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;
            }

            GateKind::CZ => {
                require_operands(gate, 2)?;

                let left = gate.qubits()[0].index();
                let right = gate.qubits()[1].index();

                ensure_qubit(left, self.qubits)?;
                ensure_qubit(right, self.qubits)?;

                /*
                 * CZ contributes π * a*b.
                 *
                 * For Boolean a,b:
                 *
                 *   a*b = (a + b - (a⊕b)) / 2.
                 *
                 * Therefore:
                 *
                 *   π*a*b =
                 *       π/2*a
                 *     + π/2*b
                 *     - π/2*(a⊕b).
                 */
                let a = wires[left].clone();
                let b = wires[right].clone();
                let xor = a.xor(&b)?;

                self.add_phase(a, constant_parameter(HALF_PI)?)?;
                self.add_phase(b, constant_parameter(HALF_PI)?)?;
                self.add_phase(
                    xor,
                    constant_parameter(-HALF_PI)?,
                )?;

                statistics.cz_gates =
                    statistics
                        .cz_gates
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

                statistics.phase_gates =
                    statistics
                        .phase_gates
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;

                statistics.gates_encoded =
                    statistics
                        .gates_encoded
                        .checked_add(1)
                        .ok_or(PhasePolynomialError::ArithmeticOverflow)?;
            }

            _ => {
                return Err(PhasePolynomialError::UnsupportedGate {
                    gate: gate.kind(),
                });
            }
        }

        Ok(())
    }

    /// Returns the number of logical input qubits.
    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    /// Returns the global phase.
    #[must_use]
    pub fn global_phase(&self) -> &Parameter {
        &self.global_phase
    }

    /// Returns the number of distinct parity terms.
    #[must_use]
    pub fn term_count(&self) -> usize {
        self.terms.len()
    }

    /// Returns whether the polynomial has no non-global phase terms.
    #[must_use]
    pub fn is_diagonal_constant(&self) -> bool {
        self.terms.is_empty()
    }

    /// Returns all terms in deterministic canonical order.
    ///
    /// The iterator performs no sorting because terms are already stored in a
    /// BTreeMap.
    pub fn terms(&self) -> impl Iterator<Item = PhaseTerm> + '_ {
        self.terms.iter().map(|(key, coefficient)| {
            PhaseTerm {
                parity: key.clone().into_parity(),
                coefficient: coefficient.clone(),
            }
        })
    }

    /// Adds a global phase.
    ///
    /// Equal global phases are folded immediately.
    pub fn add_global(
        &mut self,
        phase: Parameter,
    ) -> PhasePolynomialResult<()> {
        phase
            .validate()
            .map_err(|_| PhasePolynomialError::ParameterConstructionFailure)?;

        self.global_phase =
            add_parameters(self.global_phase.clone(), phase)?;

        self.global_phase =
            normalize_parameter(self.global_phase)?;

        Ok(())
    }

    /// Adds one phase term.
    ///
    /// Terms with equal affine parities are folded immediately.
    ///
    /// Zero coefficients are eliminated.
    pub fn add_phase(
        &mut self,
        parity: AffineParity,
        coefficient: Parameter,
    ) -> PhasePolynomialResult<()> {
        if parity.qubits() != self.qubits {
            return Err(PhasePolynomialError::DimensionMismatch {
                left: self.qubits,
                right: parity.qubits(),
            });
        }

        coefficient
            .validate()
            .map_err(|_| PhasePolynomialError::ParameterConstructionFailure)?;

        let coefficient = normalize_parameter(coefficient)?;

        if parameter_is_zero(&coefficient) {
            return Ok(());
        }

        // A constant affine parity is itself a global phase.
        if parity.mask().is_zero() {
            if parity.constant() {
                self.add_global(coefficient)?;
            }

            return Ok(());
        }

        let key = AffineParityKey::from_parity(&parity);

        match self.terms.remove(&key) {
            Some(existing) => {
                let merged =
                    add_parameters(existing, coefficient)?;

                let merged =
                    normalize_parameter(merged)?;

                if !parameter_is_zero(&merged) {
                    self.terms.insert(key, merged);
                }
            }

            None => {
                self.terms.insert(key, coefficient);
            }
        }

        Ok(())
    }

    /// Adds a linear input-variable phase.
    pub fn add_variable_phase(
        &mut self,
        qubit: usize,
        coefficient: Parameter,
    ) -> PhasePolynomialResult<()> {
        let parity =
            AffineParity::variable(self.qubits, qubit)?;

        self.add_phase(parity, coefficient)
    }

    /// Adds an affine phase.
    pub fn add_affine_phase(
        &mut self,
        parity: AffineParity,
        coefficient: Parameter,
    ) -> PhasePolynomialResult<()> {
        self.add_phase(parity, coefficient)
    }

    /// Returns the coefficient for a parity, if one exists.
    #[must_use]
    pub fn coefficient(
        &self,
        parity: &AffineParity,
    ) -> Option<&Parameter> {
        self.terms
            .get(&AffineParityKey::from_parity(parity))
    }

    /// Removes a parity term.
    ///
    /// Returns the previous coefficient.
    pub fn remove(
        &mut self,
        parity: &AffineParity,
    ) -> Option<Parameter> {
        self.terms
            .remove(&AffineParityKey::from_parity(parity))
    }

    /// Returns the total number of variable references across all terms.
    ///
    /// This is a useful sparsity metric.
    #[must_use]
    pub fn total_parity_weight(&self) -> usize {
        self.terms
            .keys()
            .map(|key| key.mask.weight())
            .sum()
    }

    /// Returns the maximum parity weight.
    #[must_use]
    pub fn max_parity_weight(&self) -> usize {
        self.terms
            .keys()
            .map(|key| key.mask.weight())
            .max()
            .unwrap_or(0)
    }

    /// Returns whether every coefficient is a concrete numerical constant.
    #[must_use]
    pub fn is_fully_bound(&self) -> bool {
        if !self.global_phase.is_constant() {
            return false;
        }

        self.terms
            .values()
            .all(Parameter::is_constant)
    }

    /// Returns all constant coefficients as f64 values.
    ///
    /// Returns `None` when at least one coefficient is symbolic.
    pub fn constant_coefficients(
        &self,
    ) -> Option<(f64, Vec<(AffineParity, f64)>)> {
        let global = self.global_phase.as_constant()?;

        let mut result =
            Vec::with_capacity(self.terms.len());

        for (key, coefficient) in &self.terms {
            let value = coefficient.as_constant()?;

            result.push((
                key.clone().into_parity(),
                value,
            ));
        }

        Some((global, result))
    }

    /// Normalizes all constant coefficients modulo 2π.
    ///
    /// Symbolic coefficients remain untouched.
    pub fn normalize(&mut self) -> PhasePolynomialResult<()> {
        self.global_phase =
            normalize_parameter(self.global_phase.clone())?;

        let entries: Vec<(AffineParityKey, Parameter)> =
            self.terms.clone().into_iter().collect();

        self.terms.clear();

        for (key, coefficient) in entries {
            let coefficient =
                normalize_parameter(coefficient)?;

            if !parameter_is_zero(&coefficient) {
                self.terms.insert(key, coefficient);
            }
        }

        Ok(())
    }

    /// Returns the number of non-zero T-like terms.
    ///
    /// A T-like term is a concrete coefficient equal to an odd multiple of
    /// π/4 modulo 2π.
    ///
    /// This is a resource-estimation helper, not a complete T-count
    /// synthesis algorithm.
    #[must_use]
    pub fn t_like_term_count(&self) -> usize {
        self.terms
            .values()
            .filter(|parameter| {
                parameter
                    .as_constant()
                    .map(is_odd_quarter_pi)
                    .unwrap_or(false)
            })
            .count()
    }

    /// Returns the number of Clifford-compatible phase terms.
    ///
    /// This recognizes coefficients that are integer multiples of π/2
    /// modulo 2π.
    #[must_use]
    pub fn clifford_phase_term_count(&self) -> usize {
        self.terms
            .values()
            .filter(|parameter| {
                parameter
                    .as_constant()
                    .map(is_multiple_of_half_pi)
                    .unwrap_or(false)
            })
            .count()
    }

    /// Returns the polynomial's maximum support dimension.
    #[must_use]
    pub fn support_qubits(&self) -> usize {
        self.terms
            .keys()
            .filter_map(|key| key.mask.highest_qubit())
            .max()
            .map(|index| index + 1)
            .unwrap_or(0)
    }

    /// Returns the parity masks only.
    ///
    /// This is useful to `synthesis::phase` and parity-matrix algorithms.
    pub fn parity_masks(&self) -> impl Iterator<Item = &ParityMask> {
        self.terms.keys().map(|key| &key.mask)
    }

    /// Returns a stable parity matrix representation.
    ///
    /// The returned vector contains one row per logical qubit and one column
    /// per phase term.
    ///
    /// This allocates O(n*m) bytes and is therefore intended for synthesis
    /// boundaries, not for inner-loop analysis.
    pub fn parity_matrix(&self) -> Vec<Vec<bool>> {
        let masks: Vec<&ParityMask> =
            self.parity_masks().collect();

        let mut matrix =
            Vec::with_capacity(self.qubits);

        for qubit in 0..self.qubits {
            let row = masks
                .iter()
                .map(|mask| mask.get(qubit))
                .collect();

            matrix.push(row);
        }

        matrix
    }

    /// Returns a sparse parity matrix.
    ///
    /// Each element is `(qubit, term_index)`.
    pub fn sparse_parity_matrix(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        for (term_index, key) in self.terms.keys().enumerate() {
            for qubit in key.mask.to_qubits() {
                result.push((qubit, term_index));
            }
        }

        result
    }

    /// Returns the phase polynomial in deterministic human-readable form.
    pub fn to_string_repr(&self) -> String {
        let mut result = String::new();

        if !parameter_is_zero(&self.global_phase) {
            result.push_str("global(");
            result.push_str(&self.global_phase.to_string());
            result.push(')');
        }

        for term in self.terms() {
            if !result.is_empty() {
                result.push_str(" + ");
            }

            result.push('(');
            result.push_str(&term.coefficient.to_string());
            result.push_str(")*");
            result.push_str(&term.parity.to_string());
        }

        if result.is_empty() {
            "0".to_owned()
        } else {
            result
        }
    }

    /// Returns whether two polynomials have exactly equal canonical
    /// representations.
    #[must_use]
    pub fn structurally_equal(&self, rhs: &Self) -> bool {
        self == rhs
    }

    /// Returns the number of terms that could potentially be synthesized as
    /// T gates from concrete coefficients.
    #[must_use]
    pub fn nonzero_term_count(&self) -> usize {
        self.terms.len()
    }

    /// Merges another phase polynomial into this one.
    ///
    /// This operation represents multiplication of diagonal phase operators,
    /// so phase functions add.
    pub fn merge(
        &mut self,
        rhs: &Self,
    ) -> PhasePolynomialResult<()> {
        if self.qubits != rhs.qubits {
            return Err(PhasePolynomialError::DimensionMismatch {
                left: self.qubits,
                right: rhs.qubits,
            });
        }

        self.add_global(rhs.global_phase.clone())?;

        for (key, coefficient) in &rhs.terms {
            self.add_phase(
                key.clone().into_parity(),
                coefficient.clone(),
            )?;
        }

        Ok(())
    }

    /// Returns the polynomial with all coefficients normalized.
    pub fn normalized(
        mut self,
    ) -> PhasePolynomialResult<Self> {
        self.normalize()?;
        Ok(self)
    }
}

impl fmt::Display for PhasePolynomial {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.to_string_repr())
    }
}

// =============================================================================
// Parameter algebra
// =============================================================================

fn zero_parameter() -> PhasePolynomialResult<Parameter> {
    constant_parameter(0.0)
}

fn constant_parameter(
    value: f64,
) -> PhasePolynomialResult<Parameter> {
    Parameter::constant(value)
        .map_err(|_| PhasePolynomialError::ParameterConstructionFailure)
}

fn parameter_is_zero(
    parameter: &Parameter,
) -> bool {
    parameter
        .as_constant()
        .map(|value| value == 0.0)
        .unwrap_or(false)
}

fn normalize_angle(value: f64) -> f64 {
    if !value.is_finite() {
        return value;
    }

    let mut result = value % TWO_PI;

    if result <= -PI {
        result += TWO_PI;
    } else if result > PI {
        result -= TWO_PI;
    }

    if result == -0.0 {
        0.0
    } else {
        result
    }
}

fn normalize_parameter(
    parameter: Parameter,
) -> PhasePolynomialResult<Parameter> {
    match parameter {
        Parameter::Constant(value) => {
            constant_parameter(normalize_angle(value))
        }

        Parameter::Symbol(_) => Ok(parameter),

        Parameter::Expression(expression) => {
            Ok(Parameter::Expression(expression))
        }
    }
}

fn negate_parameter(
    parameter: Parameter,
) -> PhasePolynomialResult<Parameter> {
    match parameter {
        Parameter::Constant(value) => {
            constant_parameter(-value)
        }

        other => {
            Parameter::expression(
                ParameterExpression::Multiply(
                    Box::new(other),
                    Box::new(
                        constant_parameter(-1.0)?,
                    ),
                ),
            )
            .map_err(|_| {
                PhasePolynomialError::ParameterConstructionFailure
            })
        }
    }
}

fn negate_half(
    parameter: Parameter,
) -> PhasePolynomialResult<Parameter> {
    match parameter {
        Parameter::Constant(value) => {
            constant_parameter(-value * 0.5)
        }

        other => {
            Parameter::expression(
                ParameterExpression::Multiply(
                    Box::new(other),
                    Box::new(
                        constant_parameter(-0.5)?,
                    ),
                ),
            )
            .map_err(|_| {
                PhasePolynomialError::ParameterConstructionFailure
            })
        }
    }
}

fn add_parameters(
    left: Parameter,
    right: Parameter,
) -> PhasePolynomialResult<Parameter> {
    match (left, right) {
        (
            Parameter::Constant(left),
            Parameter::Constant(right),
        ) => constant_parameter(left + right),

        (left, right) => {
            Parameter::expression(
                ParameterExpression::Add(
                    Box::new(left),
                    Box::new(right),
                ),
            )
            .map_err(|_| {
                PhasePolynomialError::ParameterConstructionFailure
            })
        }
    }
}

// =============================================================================
// Gate validation helpers
// =============================================================================

fn ensure_qubit(
    qubit: usize,
    qubits: usize,
) -> PhasePolynomialResult<()> {
    if qubit >= qubits {
        return Err(PhasePolynomialError::QubitOutOfRange {
            qubit,
            qubits,
        });
    }

    Ok(())
}

fn require_operands(
    gate: &Gate,
    expected: usize,
) -> PhasePolynomialResult<()> {
    let actual = gate.qubits().len();

    if actual != expected {
        return Err(
            PhasePolynomialError::InvalidGateOperands {
                gate: gate.kind(),
                expected,
                actual,
            },
        );
    }

    Ok(())
}

fn require_parameters(
    gate: &Gate,
    expected: usize,
) -> PhasePolynomialResult<()> {
    let actual = gate.parameters().len();

    if actual != expected {
        return Err(
            PhasePolynomialError::InvalidGateParameters {
                gate: gate.kind(),
                expected,
                actual,
            },
        );
    }

    Ok(())
}

fn parameter_from_gate(
    gate: &Gate,
    index: usize,
) -> PhasePolynomialResult<Parameter> {
    gate.parameters()
        .get(index)
        .cloned()
        .ok_or(
            PhasePolynomialError::InvalidParameter {
                gate: gate.kind(),
                index,
            },
        )
}

// =============================================================================
// Coefficient classification
// =============================================================================

fn is_odd_quarter_pi(
    value: f64,
) -> bool {
    let units = value / QUARTER_PI;
    let rounded = units.round();

    if (units - rounded).abs() > 1.0e-10 {
        return false;
    }

    let integer = rounded as i64;

    integer.rem_euclid(2) != 0
}

fn is_multiple_of_half_pi(
    value: f64,
) -> bool {
    let units = value / HALF_PI;
    let rounded = units.round();

    (units - rounded).abs() <= 1.0e-10
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::quantum::ir::gate::Gate;
    use crate::quantum::ir::qubits::QubitId;

    fn q(index: usize) -> QubitId {
        QubitId::new(index)
    }

    #[test]
    fn zero_parity_is_empty() {
        let parity =
            ParityMask::zero(130).expect("valid");

        assert!(parity.is_zero());
        assert_eq!(parity.weight(), 0);
        assert_eq!(parity.qubits(), 130);
    }

    #[test]
    fn packed_parity_handles_more_than_one_word() {
        let mut parity =
            ParityMask::zero(130).expect("valid");

        parity.set(0, true).expect("valid");
        parity.set(64, true).expect("valid");
        parity.set(129, true).expect("valid");

        assert_eq!(parity.weight(), 3);
        assert_eq!(
            parity.to_qubits(),
            vec![0, 64, 129]
        );
    }

    #[test]
    fn duplicate_qubits_cancel_in_parity() {
        let parity =
            ParityMask::from_qubits(
                8,
                [1usize, 2, 1],
            )
            .expect("valid");

        assert_eq!(
            parity.to_qubits(),
            vec![2]
        );
    }

    #[test]
    fn affine_xor_is_correct() {
        let left =
            AffineParity::variable(8, 1)
                .expect("valid");

        let right =
            AffineParity::variable(8, 3)
                .expect("valid");

        let result =
            left.xor(&right)
                .expect("valid");

        assert!(!result.constant());
        assert_eq!(
            result.to_qubits(),
            vec![1, 3]
        );
    }

    #[test]
    fn x_gate_updates_affine_wire() {
        let gates = vec![
            Gate::x(q(0)).expect("valid"),
        ];

        let (polynomial, _) =
            PhasePolynomial::from_gates(
                &gates,
                1,
            )
            .expect("valid");

        assert_eq!(
            polynomial.term_count(),
            0
        );
    }

    #[test]
    fn rz_contains_global_and_local_phase() {
        let gates = vec![
            Gate::rz(q(0), PI)
                .expect("valid"),
        ];

        let (polynomial, _) =
            PhasePolynomial::from_gates(
                &gates,
                1,
            )
            .expect("valid");

        assert_eq!(
            polynomial
                .global_phase()
                .as_constant(),
            Some(-PI / 2.0)
        );

        assert_eq!(
            polynomial.term_count(),
            1
        );

        let parity =
            AffineParity::variable(1, 0)
                .expect("valid");

        assert_eq!(
            polynomial
                .coefficient(&parity)
                .and_then(Parameter::as_constant),
            Some(PI)
        );
    }

    #[test]
    fn t_and_tdg_cancel() {
        let gates = vec![
            Gate::t(q(0)).expect("valid"),
            Gate::tdg(q(0)).expect("valid"),
        ];

        let (polynomial, _) =
            PhasePolynomial::from_gates(
                &gates,
                1,
            )
            .expect("valid");

        assert_eq!(
            polynomial.term_count(),
            0
        );
    }

    #[test]
    fn repeated_same_parity_is_folded() {
        let gates = vec![
            Gate::t(q(0)).expect("valid"),
            Gate::t(q(0)).expect("valid"),
            Gate::t(q(0)).expect("valid"),
            Gate::t(q(0)).expect("valid"),
        ];

        let (polynomial, _) =
            PhasePolynomial::from_gates(
                &gates,
                1,
            )
            .expect("valid");

        assert_eq!(
            polynomial.term_count(),
            0
        );
    }

    #[test]
    fn cnot_changes_tracked_parity() {
        let gates = vec![
            Gate::cx(q(0), q(1))
                .expect("valid"),
            Gate::rz(q(1), PI)
                .expect("valid"),
        ];

        let (polynomial, _) =
            PhasePolynomial::from_gates(
                &gates,
                2,
            )
            .expect("valid");

        let parity =
            AffineParity::new(
                ParityMask::from_qubits(
                    2,
                    [0, 1],
                )
                .expect("valid"),
                false,
            );

        assert_eq!(
            polynomial.coefficient(&parity)
                .and_then(Parameter::as_constant),
            Some(PI)
        );
    }

    #[test]
    fn cz_is_represented_without_quadratic_storage() {
        let gates = vec![
            Gate::cz(q(0), q(1))
                .expect("valid"),
        ];

        let (polynomial, _) =
            PhasePolynomial::from_gates(
                &gates,
                2,
            )
            .expect("valid");

        assert_eq!(
            polynomial.term_count(),
            3
        );

        let x0 =
            AffineParity::variable(2, 0)
                .expect("valid");

        let x1 =
            AffineParity::variable(2, 1)
                .expect("valid");

        let x01 =
            AffineParity::new(
                ParityMask::from_qubits(
                    2,
                    [0, 1],
                )
                .expect("valid"),
                false,
            );

        assert_eq!(
            polynomial
                .coefficient(&x0)
                .and_then(Parameter::as_constant),
            Some(HALF_PI)
        );

        assert_eq!(
            polynomial
                .coefficient(&x1)
                .and_then(Parameter::as_constant),
            Some(HALF_PI)
        );

        assert_eq!(
            polynomial
                .coefficient(&x01)
                .and_then(Parameter::as_constant),
            Some(-HALF_PI)
        );
    }

    #[test]
    fn unsupported_h_gate_is_rejected() {
        let gates = vec![
            Gate::h(q(0)).expect("valid"),
        ];

        let result =
            PhasePolynomial::from_gates(
                &gates,
                1,
            );

        assert!(matches!(
            result,
            Err(
                PhasePolynomialError::UnsupportedGate {
                    gate: GateKind::H
                }
            )
        ));
    }

    #[test]
    fn symbolic_rz_is_preserved() {
        let parameter =
            Parameter::symbol("theta")
                .expect("valid");

        let gate =
            Gate::parameterized(
                GateKind::RZ,
                vec![q(0)],
                vec![parameter],
            )
            .expect("valid");

        let (polynomial, _) =
            PhasePolynomial::from_gates(
                &[gate],
                1,
            )
            .expect("valid");

        assert!(!polynomial
            .global_phase()
            .is_constant());

        assert_eq!(
            polynomial.term_count(),
            1
        );
    }

    #[test]
    fn t_like_count_is_detected() {
        let gates = vec![
            Gate::t(q(0)).expect("valid"),
            Gate::t(q(1)).expect("valid"),
        ];

        let (polynomial, _) =
            PhasePolynomial::from_gates(
                &gates,
                2,
            )
            .expect("valid");

        assert_eq!(
            polynomial.t_like_term_count(),
            2
        );
    }

    #[test]
    fn deterministic_order_is_preserved() {
        let mut polynomial =
            PhasePolynomial::new(4)
                .expect("valid");

        polynomial
            .add_variable_phase(
                3,
                constant_parameter(PI)
                    .expect("valid"),
            )
            .expect("valid");

        polynomial
            .add_variable_phase(
                0,
                constant_parameter(PI)
                    .expect("valid"),
            )
            .expect("valid");

        let terms: Vec<_> =
            polynomial
                .terms()
                .map(|term| term.parity().to_qubits())
                .collect();

        assert_eq!(
            terms,
            vec![vec![0], vec![3]]
        );
    }

    #[test]
    fn large_qubit_namespace_is_supported() {
        let polynomial =
            PhasePolynomial::new(1_000_000)
                .expect("valid");

        assert_eq!(
            polynomial.qubits(),
            1_000_000
        );
    }

    #[test]
    fn phase_polynomial_can_merge() {
        let mut left =
            PhasePolynomial::new(2)
                .expect("valid");

        let mut right =
            PhasePolynomial::new(2)
                .expect("valid");

        left.add_variable_phase(
            0,
            constant_parameter(QUARTER_PI)
                .expect("valid"),
        )
        .expect("valid");

        right.add_variable_phase(
            0,
            constant_parameter(QUARTER_PI)
                .expect("valid"),
        )
        .expect("valid");

        left.merge(&right)
            .expect("valid");

        let parity =
            AffineParity::variable(2, 0)
                .expect("valid");

        assert_eq!(
            left.coefficient(&parity)
                .and_then(Parameter::as_constant),
            Some(HALF_PI)
        );
    }
}