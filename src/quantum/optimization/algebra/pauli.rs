//! Zamani Quantum Optimization — Pauli Algebra
//!
//! Production-grade, hardware-independent Pauli algebra for the quantum
//! optimization subsystem.
//!
//! # Architectural role
//!
//! This module provides the canonical Pauli algebra used by optimization
//! passes that reason about:
//!
//! - Pauli strings;
//! - Pauli products;
//! - Pauli commutation;
//! - Clifford conjugation;
//! - Pauli-frame transformations;
//! - phase-polynomial optimization;
//! - stabilizer optimization;
//! - Clifford+T optimization;
//! - Hamiltonian/operator decomposition;
//! - observable canonicalization;
//! - algebraic circuit rewriting.
//!
//! It intentionally does NOT own:
//!
//! - quantum gates;
//! - quantum circuits;
//! - Quantum IR;
//! - routing;
//! - scheduling;
//! - hardware topology;
//! - QPU execution;
//! - measurement sampling;
//! - error-correction codes;
//! - frontend parsing;
//! - backend APIs.
//!
//! The intended dependency direction is:
//!
//! ```text
//! quantum::ir
//!      │
//!      ▼
//! quantum::optimization
//!      │
//!      ├── algebra::pauli
//!      │      │
//!      │      ├── algebra::clifford
//!      │      ├── algebra::phase_polynomial
//!      │      ├── algebra::symplectic
//!      │      ├── synthesis
//!      │      └── fault_tolerant
//!      │
//!      └── local/rewrite passes
//! ```
//!
//! This module must remain independent of those downstream modules so that it
//! can be completed and stabilized without later architectural rewrites.
//!
//! # Representation
//!
//! A Pauli string over `n` qubits is represented in symplectic form:
//
//! ```text
//! X = x[0..n)
//! Z = z[0..n)
//!
//! I = (0, 0)
//! X = (1, 0)
//! Z = (0, 1)
//! Y = (1, 1)
//! ```
//!
//! The implementation stores the X and Z vectors as packed `u64` words.
//!
//! Therefore, for `n` qubits:
//
//! ```text
//! storage = O(n / 64)
//! ```
//!
//! rather than `O(n)` bytes/objects.
//!
//! Operations such as commutation and multiplication therefore operate on
//! machine words rather than allocating one object per qubit.
//!
//! # Phase representation
//!
//! A general Pauli product may contain one of:
//!
//! ```text
//! +1
//! +i
//! -1
//! -i
//! ```
//!
//! Consequently, [`PauliProduct`] stores the phase exponent modulo four:
//!
//! ```text
//! 0 → +1
//! 1 → +i
//! 2 → -1
//! 3 → -i
//! ```
//!
//! A Hermitian observable/stabilizer is restricted to phases `0` and `2` and
//! is represented by [`PauliString`].
//!
//! This distinction is important. A generic Pauli multiplication routine
//! must not incorrectly discard ±i phases.
//!
//! # Mathematical conventions
//!
//! Qubit ordering is deterministic and zero-based. The qubit index supplied
//! to [`PauliString::set`] corresponds directly to the logical qubit index
//! used by the caller.
//!
//! No endian-dependent byte representation is exposed.
//!
//! # Complexity
//!
//! Let:
//!
//! ```text
//! n = number of qubits
//! W = ceil(n / 64)
//! ```
//!
//! Then:
//!
//! - construction: O(W);
//! - Pauli lookup: O(1);
//! - Pauli update: O(1);
//! - equality: O(W);
//! - commutation: O(W);
//! - multiplication: O(W);
//! - tensor-compatible concatenation: O(W);
//! - conversion to dense Pauli values: O(n);
//! - highest non-identity qubit lookup: O(W) worst case.
//!
//! The implementation deliberately does not impose an artificial fixed
//! maximum qubit count. Resource limits belong to the optimization subsystem,
//! while this mathematical representation remains bounded by the platform's
//! available memory and address space.
//!
//! # Safety
//!
//! - No `unsafe` code.
//! - No raw pointers.
//! - No global mutable state.
//! - No unchecked public indexing.
//! - Allocation sizes are checked.
//! - Integer overflow affecting allocation or indexing is checked.
//! - Phase arithmetic is performed modulo four.
//!
//! # Rust compatibility
//!
//! Designed for Rust 1.97 and Rust 1.97.1.
//!
//! No nightly features are required.
//! No external dependencies are required.
//!
//! # Integration contract
//!
//! `algebra::clifford` should consume [`PauliString`] and [`PauliProduct`].
//!
//! `algebra::phase_polynomial` should use [`PauliString::commutes_with`],
//! [`PauliString::multiply`], and [`PauliProduct`].
//!
//! `algebra::symplectic` can use [`PauliString::x_bits`] and
//! [`PauliString::z_bits`] through the word-oriented APIs provided here.
//!
//! `fault_tolerant` should use this module for Clifford+T Pauli reasoning.
//!
//! `quantum::memory::stabilizer` already implements a related packed
//! symplectic representation; this optimizer module deliberately does not
//! depend on the memory subsystem so that optimization remains usable without
//! a simulator.
//!
//! Future adapters may convert between the two representations at their
//! integration boundary.
//!
//! `algebra::mod.rs` should eventually expose:
//!
//! ```text
//! pub mod pauli;
//! ```
//!
//! No changes to this file should be required when those future modules are
//! implemented.

use std::fmt;

// =============================================================================
// Constants
// =============================================================================

const WORD_BITS: usize = 64;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by Pauli algebra operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PauliError {
    /// The requested number of qubits cannot be represented safely.
    QubitCountOverflow {
        qubits: usize,
    },

    /// A requested qubit is outside the Pauli string.
    QubitOutOfRange {
        qubit: usize,
        qubits: usize,
    },

    /// Two Pauli operands represent different numbers of qubits.
    DimensionMismatch {
        left: usize,
        right: usize,
    },

    /// A phase exponent is outside the canonical modulo-four domain.
    InvalidPhase {
        phase: u8,
    },

    /// A Hermitian Pauli string was required but the phase was ±i.
    NonHermitianPhase {
        phase: u8,
    },

    /// A Pauli value could not be represented.
    InvalidPauliValue,

    /// An arithmetic operation overflowed.
    ArithmeticOverflow,

    /// Allocation failed because the requested collection size cannot be
    /// represented by the platform.
    AllocationSizeOverflow,
}

impl fmt::Display for PauliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QubitCountOverflow { qubits } => {
                write!(f, "Pauli qubit count cannot be represented safely: {qubits}")
            }

            Self::QubitOutOfRange { qubit, qubits } => {
                write!(
                    f,
                    "Pauli qubit index {qubit} is outside {qubits} qubits"
                )
            }

            Self::DimensionMismatch { left, right } => {
                write!(
                    f,
                    "Pauli dimension mismatch: left has {left} qubits, right has {right}"
                )
            }

            Self::InvalidPhase { phase } => {
                write!(
                    f,
                    "invalid Pauli phase exponent {phase}; expected 0..=3"
                )
            }

            Self::NonHermitianPhase { phase } => {
                write!(
                    f,
                    "Pauli string is not Hermitian because its phase exponent is {phase}"
                )
            }

            Self::InvalidPauliValue => {
                write!(f, "invalid Pauli value")
            }

            Self::ArithmeticOverflow => {
                write!(f, "Pauli arithmetic overflow")
            }

            Self::AllocationSizeOverflow => {
                write!(f, "Pauli allocation size overflow")
            }
        }
    }
}

impl std::error::Error for PauliError {}

/// Result type used by this module.
pub type PauliResult<T> = Result<T, PauliError>;

// =============================================================================
// Pauli
// =============================================================================

/// Single-qubit Pauli operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pauli {
    /// Identity.
    I,

    /// Pauli X.
    X,

    /// Pauli Y.
    Y,

    /// Pauli Z.
    Z,
}

impl Pauli {
    /// Returns true when this is the identity operator.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::I)
    }

    /// Returns the symplectic X bit.
    ///
    /// ```text
    /// I → 0
    /// X → 1
    /// Y → 1
    /// Z → 0
    /// ```
    #[must_use]
    pub const fn x_bit(self) -> bool {
        matches!(self, Self::X | Self::Y)
    }

    /// Returns the symplectic Z bit.
    ///
    /// ```text
    /// I → 0
    /// X → 0
    /// Y → 1
    /// Z → 1
    /// ```
    #[must_use]
    pub const fn z_bit(self) -> bool {
        matches!(self, Self::Y | Self::Z)
    }

    /// Creates a Pauli from its symplectic bits.
    #[must_use]
    pub const fn from_bits(x: bool, z: bool) -> Self {
        match (x, z) {
            (false, false) => Self::I,
            (true, false) => Self::X,
            (false, true) => Self::Z,
            (true, true) => Self::Y,
        }
    }

    /// Returns the conventional character representation.
    #[must_use]
    pub const fn as_char(self) -> char {
        match self {
            Self::I => 'I',
            Self::X => 'X',
            Self::Y => 'Y',
            Self::Z => 'Z',
        }
    }

    /// Returns the Pauli product of two single-qubit Paulis.
    ///
    /// The returned value includes the complete phase:
    ///
    /// ```text
    /// X X = +I
    /// Y Y = +I
    /// Z Z = +I
    ///
    /// X Y = +iZ
    /// Y X = -iZ
    ///
    /// X Z = -iY
    /// Z X = +iY
    ///
    /// Y Z = +iX
    /// Z Y = -iX
    /// ```
    #[must_use]
    pub const fn multiply(self, rhs: Self) -> SinglePauliProduct {
        use Pauli::*;

        match (self, rhs) {
            (I, p) | (p, I) => SinglePauliProduct {
                pauli: p,
                phase: 0,
            },

            (X, X) | (Y, Y) | (Z, Z) => SinglePauliProduct {
                pauli: I,
                phase: 0,
            },

            (X, Y) => SinglePauliProduct {
                pauli: Z,
                phase: 1,
            },

            (Y, X) => SinglePauliProduct {
                pauli: Z,
                phase: 3,
            },

            (X, Z) => SinglePauliProduct {
                pauli: Y,
                phase: 3,
            },

            (Z, X) => SinglePauliProduct {
                pauli: Y,
                phase: 1,
            },

            (Y, Z) => SinglePauliProduct {
                pauli: X,
                phase: 1,
            },

            (Z, Y) => SinglePauliProduct {
                pauli: X,
                phase: 3,
            },
        }
    }
}

impl fmt::Display for Pauli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::I => "I",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
        })
    }
}

/// Result of multiplying two single-qubit Pauli operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SinglePauliProduct {
    pauli: Pauli,
    phase: u8,
}

impl SinglePauliProduct {
    /// Returns the resulting Pauli.
    #[must_use]
    pub const fn pauli(self) -> Pauli {
        self.pauli
    }

    /// Returns the phase exponent modulo four.
    #[must_use]
    pub const fn phase(self) -> u8 {
        self.phase
    }
}

// =============================================================================
// Packed bitset
// =============================================================================

/// Private packed bitset used by PauliString.
///
/// The representation is intentionally opaque so that its storage strategy
/// can evolve without changing the public Pauli algebra API.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BitSet {
    words: Vec<u64>,
    bits: usize,
}

impl BitSet {
    fn word_count(bits: usize) -> PauliResult<usize> {
        bits.checked_add(WORD_BITS - 1)
            .ok_or(PauliError::QubitCountOverflow { qubits: bits })
            .map(|value| value / WORD_BITS)
    }

    fn new(bits: usize) -> PauliResult<Self> {
        let word_count = Self::word_count(bits)?;

        Ok(Self {
            words: vec![0; word_count],
            bits,
        })
    }

    #[inline]
    fn bits(&self) -> usize {
        self.bits
    }

    #[inline]
    fn get(&self, index: usize) -> bool {
        if index >= self.bits {
            return false;
        }

        let word = index / WORD_BITS;
        let bit = index % WORD_BITS;

        ((self.words[word] >> bit) & 1) != 0
    }

    fn set(&mut self, index: usize, value: bool) -> PauliResult<()> {
        if index >= self.bits {
            return Err(PauliError::QubitOutOfRange {
                qubit: index,
                qubits: self.bits,
            });
        }

        let word = index / WORD_BITS;
        let bit = index % WORD_BITS;
        let mask = 1u64 << bit;

        if value {
            self.words[word] |= mask;
        } else {
            self.words[word] &= !mask;
        }

        Ok(())
    }

    #[inline]
    fn xor_assign(&mut self, rhs: &Self) {
        for (left, right) in self.words.iter_mut().zip(rhs.words.iter()) {
            *left ^= *right;
        }
    }

    #[inline]
    fn parity_and(&self, rhs: &Self) -> bool {
        let mut parity = false;

        for (left, right) in self.words.iter().zip(rhs.words.iter()) {
            parity ^= ((left & right).count_ones() & 1) != 0;
        }

        parity
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.words.iter().all(|word| *word == 0)
    }

    fn words(&self) -> &[u64] {
        &self.words
    }
}

// =============================================================================
// Pauli product
// =============================================================================

/// General product of two Pauli operators.
///
/// Unlike [`PauliString`], this type permits ±i phases because a generic
/// product of Hermitian Pauli operators is not necessarily Hermitian.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliProduct {
    x: BitSet,
    z: BitSet,
    phase: u8,
}

impl PauliProduct {
    fn new(x: BitSet, z: BitSet, phase: u8) -> PauliResult<Self> {
        if phase > 3 {
            return Err(PauliError::InvalidPhase { phase });
        }

        if x.bits() != z.bits() {
            return Err(PauliError::DimensionMismatch {
                left: x.bits(),
                right: z.bits(),
            });
        }

        Ok(Self {
            x,
            z,
            phase,
        })
    }

    /// Returns the number of qubits.
    #[must_use]
    pub fn qubits(&self) -> usize {
        self.x.bits()
    }

    /// Returns the phase exponent modulo four.
    ///
    /// ```text
    /// 0 → +1
    /// 1 → +i
    /// 2 → -1
    /// 3 → -i
    /// ```
    #[must_use]
    pub const fn phase(&self) -> u8 {
        self.phase
    }

    /// Returns true when the product is Hermitian.
    #[must_use]
    pub const fn is_hermitian(&self) -> bool {
        self.phase == 0 || self.phase == 2
    }

    /// Returns the Pauli operator at a qubit.
    #[must_use]
    pub fn get(&self, qubit: usize) -> Pauli {
        Pauli::from_bits(self.x.get(qubit), self.z.get(qubit))
    }

    /// Converts the product into a Hermitian [`PauliString`].
    pub fn into_pauli_string(self) -> PauliResult<PauliString> {
        if !self.is_hermitian() {
            return Err(PauliError::NonHermitianPhase {
                phase: self.phase,
            });
        }

        Ok(PauliString {
            x: self.x,
            z: self.z,
            phase: self.phase,
        })
    }

    /// Borrows the X symplectic words.
    ///
    /// This is intended for high-performance algebra modules such as
    /// `symplectic.rs`. The representation remains read-only.
    #[must_use]
    pub fn x_words(&self) -> &[u64] {
        self.x.words()
    }

    /// Borrows the Z symplectic words.
    #[must_use]
    pub fn z_words(&self) -> &[u64] {
        self.z.words()
    }
}

// =============================================================================
// PauliString
// =============================================================================

/// Hermitian multi-qubit Pauli operator.
///
/// The phase is restricted to ±1:
///
/// ```text
/// phase 0 → +P
/// phase 2 → -P
/// ```
///
/// This makes the type suitable for observables, stabilizers and
/// Pauli-frame elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliString {
    x: BitSet,
    z: BitSet,
    phase: u8,
}

impl PauliString {
    /// Creates an all-identity Pauli string.
    ///
    /// The identity is a valid Pauli string and is intentionally supported.
    pub fn identity(qubits: usize) -> PauliResult<Self> {
        Ok(Self {
            x: BitSet::new(qubits)?,
            z: BitSet::new(qubits)?,
            phase: 0,
        })
    }

    /// Creates a single-qubit Pauli string.
    pub fn single(
        qubits: usize,
        qubit: usize,
        pauli: Pauli,
    ) -> PauliResult<Self> {
        let mut result = Self::identity(qubits)?;
        result.set(qubit, pauli)?;
        Ok(result)
    }

    /// Creates a Pauli string from a dense sequence.
    ///
    /// The sequence is converted once into the packed representation.
    pub fn from_paulis<I>(paulis: I) -> PauliResult<Self>
    where
        I: IntoIterator<Item = Pauli>,
    {
        let values: Vec<Pauli> = paulis.into_iter().collect();

        let mut result = Self::identity(values.len())?;

        for (index, pauli) in values.into_iter().enumerate() {
            result.set(index, pauli)?;
        }

        Ok(result)
    }

    /// Creates a Pauli string from a compact string such as `"IXYZ"`.
    ///
    /// Whitespace is ignored.
    pub fn from_str_repr(value: &str) -> PauliResult<Self> {
        let mut paulis = Vec::with_capacity(value.len());

        for character in value.chars() {
            match character {
                'I' | 'i' => paulis.push(Pauli::I),
                'X' | 'x' => paulis.push(Pauli::X),
                'Y' | 'y' => paulis.push(Pauli::Y),
                'Z' | 'z' => paulis.push(Pauli::Z),
                ' ' | '\t' | '\n' | '\r' => {}
                _ => return Err(PauliError::InvalidPauliValue),
            }
        }

        Self::from_paulis(paulis)
    }

    /// Returns the number of qubits.
    #[must_use]
    pub fn qubits(&self) -> usize {
        self.x.bits()
    }

    /// Returns the phase exponent.
    ///
    /// ```text
    /// 0 → +1
    /// 2 → -1
    /// ```
    #[must_use]
    pub const fn phase(&self) -> u8 {
        self.phase
    }

    /// Returns the numerical sign.
    ///
    /// The result is always `+1` or `-1`.
    #[must_use]
    pub const fn sign(&self) -> i8 {
        if self.phase == 0 {
            1
        } else {
            -1
        }
    }

    /// Returns true when this Pauli string is the identity with positive
    /// phase.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.phase == 0 && self.x.is_zero() && self.z.is_zero()
    }

    /// Returns true when the underlying unsigned Pauli operator is identity.
    ///
    /// This differs from [`Self::is_identity`] because `-I` is not the same
    /// observable as `+I`.
    #[must_use]
    pub fn has_identity_support(&self) -> bool {
        self.x.is_zero() && self.z.is_zero()
    }

    /// Returns true when the Pauli acts non-trivially on at least one qubit.
    #[must_use]
    pub fn has_non_identity_support(&self) -> bool {
        !self.x.is_zero() || !self.z.is_zero()
    }

    /// Returns the Pauli operator at a qubit.
    ///
    /// Out-of-range indices return identity. This makes read-only analysis
    /// safe and allocation-free.
    #[must_use]
    pub fn get(&self, qubit: usize) -> Pauli {
        Pauli::from_bits(self.x.get(qubit), self.z.get(qubit))
    }

    /// Sets the Pauli operator at one qubit.
    ///
    /// The operation changes only the requested symplectic bits and does not
    /// alter the global phase.
    pub fn set(&mut self, qubit: usize, pauli: Pauli) -> PauliResult<()> {
        self.x.set(qubit, pauli.x_bit())?;
        self.z.set(qubit, pauli.z_bit())?;

        Ok(())
    }

    /// Sets the global sign to +1.
    pub const fn set_positive(&mut self) {
        self.phase = 0;
    }

    /// Sets the global sign to -1.
    pub const fn set_negative(&mut self) {
        self.phase = 2;
    }

    /// Negates the Pauli string.
    pub const fn negated(mut self) -> Self {
        self.phase ^= 2;
        self
    }

    /// Returns whether this Pauli string commutes with another.
    ///
    /// For symplectic vectors `(x,z)` and `(x',z')`, two Pauli operators
    /// commute iff:
    ///
    /// ```text
    /// x·z' + z·x' = 0 mod 2
    /// ```
    #[must_use]
    pub fn commutes_with(&self, rhs: &Self) -> bool {
        if self.qubits() != rhs.qubits() {
            return false;
        }

        let first = self.x.parity_and(&rhs.z);
        let second = self.z.parity_and(&rhs.x);

        !(first ^ second)
    }

    /// Returns whether this Pauli string anticommutes with another.
    #[must_use]
    pub fn anticommutes_with(&self, rhs: &Self) -> bool {
        !self.commutes_with(rhs)
    }

    /// Multiplies two Pauli strings while retaining the complete ±i phase.
    ///
    /// The returned type is [`PauliProduct`] because the product of two
    /// Hermitian Paulis can be anti-Hermitian.
    pub fn multiply(&self, rhs: &Self) -> PauliResult<PauliProduct> {
        if self.qubits() != rhs.qubits() {
            return Err(PauliError::DimensionMismatch {
                left: self.qubits(),
                right: rhs.qubits(),
            });
        }

        let mut x = self.x.clone();
        let mut z = self.z.clone();

        x.xor_assign(&rhs.x);
        z.xor_assign(&rhs.z);

        let phase = multiply_phase(self, rhs)?;

        PauliProduct::new(x, z, phase)
    }

    /// Multiplies two commuting Pauli strings and returns the Hermitian
    /// result directly.
    pub fn multiply_commuting(&self, rhs: &Self) -> PauliResult<Self> {
        let product = self.multiply(rhs)?;

        product.into_pauli_string()
    }

    /// Returns the X symplectic words.
    ///
    /// This is read-only and allocation-free.
    #[must_use]
    pub fn x_words(&self) -> &[u64] {
        self.x.words()
    }

    /// Returns the Z symplectic words.
    ///
    /// This is read-only and allocation-free.
    #[must_use]
    pub fn z_words(&self) -> &[u64] {
        self.z.words()
    }

    /// Returns the highest qubit on which this Pauli acts non-trivially.
    #[must_use]
    pub fn highest_non_identity_qubit(&self) -> Option<usize> {
        let x = self.x.highest_set_bit();
        let z = self.z.highest_set_bit();

        match (x, z) {
            (Some(left), Some(right)) => Some(left.max(right)),
            (Some(value), None) | (None, Some(value)) => Some(value),
            (None, None) => None,
        }
    }

    /// Returns the number of non-identity qubits.
    ///
    /// This is useful when estimating Pauli-string application cost.
    #[must_use]
    pub fn weight(&self) -> usize {
        self.x
            .words()
            .iter()
            .zip(self.z.words().iter())
            .map(|(x, z)| (*x | *z).count_ones() as usize)
            .sum()
    }

    /// Returns an iterator over all qubit/Pauli pairs.
    pub fn iter(&self) -> PauliStringIter<'_> {
        PauliStringIter {
            pauli: self,
            index: 0,
        }
    }

    /// Returns an iterator over only non-identity terms.
    pub fn non_identity_iter(&self) -> NonIdentityPauliIter<'_> {
        NonIdentityPauliIter {
            pauli: self,
            index: 0,
        }
    }

    /// Returns the dense Pauli sequence.
    ///
    /// This allocates O(n) storage. Use [`Self::iter`] for streaming access.
    pub fn to_vec(&self) -> Vec<Pauli> {
        self.iter().map(|(_, pauli)| pauli).collect()
    }

    /// Returns a compact textual representation.
    ///
    /// Examples:
    ///
    /// ```text
    /// III
    /// +XYZ
    /// -XYZ
    /// ```
    #[must_use]
    pub fn to_string_repr(&self) -> String {
        let mut result = String::with_capacity(self.qubits() + 1);

        if self.phase == 2 {
            result.push('-');
        }

        for index in 0..self.qubits() {
            result.push(self.get(index).as_char());
        }

        result
    }

    /// Returns the Hamming-distance-like symplectic support difference.
    ///
    /// This is useful for rewrite heuristics and phase-polynomial grouping.
    pub fn symplectic_distance(&self, rhs: &Self) -> PauliResult<usize> {
        if self.qubits() != rhs.qubits() {
            return Err(PauliError::DimensionMismatch {
                left: self.qubits(),
                right: rhs.qubits(),
            });
        }

        let mut distance = 0usize;

        for (left, right) in self.x.words().iter().zip(rhs.x.words().iter()) {
            distance = distance
                .checked_add((left ^ right).count_ones() as usize)
                .ok_or(PauliError::ArithmeticOverflow)?;
        }

        for (left, right) in self.z.words().iter().zip(rhs.z.words().iter()) {
            distance = distance
                .checked_add((left ^ right).count_ones() as usize)
                .ok_or(PauliError::ArithmeticOverflow)?;
        }

        Ok(distance)
    }

    /// Applies a Pauli X conjugation:
    ///
    /// ```text
    /// X P X
    /// ```
    ///
    /// The Pauli support remains unchanged; only the sign changes for Y/Z.
    pub fn conjugate_by_x(&mut self, qubit: usize) -> PauliResult<()> {
        if qubit >= self.qubits() {
            return Err(PauliError::QubitOutOfRange {
                qubit,
                qubits: self.qubits(),
            });
        }

        if self.z.get(qubit) {
            self.phase ^= 2;
        }

        Ok(())
    }

    /// Applies a Pauli Z conjugation.
    pub fn conjugate_by_z(&mut self, qubit: usize) -> PauliResult<()> {
        if qubit >= self.qubits() {
            return Err(PauliError::QubitOutOfRange {
                qubit,
                qubits: self.qubits(),
            });
        }

        if self.x.get(qubit) {
            self.phase ^= 2;
        }

        Ok(())
    }

    /// Applies a Pauli Y conjugation.
    pub fn conjugate_by_y(&mut self, qubit: usize) -> PauliResult<()> {
        if qubit >= self.qubits() {
            return Err(PauliError::QubitOutOfRange {
                qubit,
                qubits: self.qubits(),
            });
        }

        if self.x.get(qubit) ^ self.z.get(qubit) {
            self.phase ^= 2;
        }

        Ok(())
    }

    /// Returns the expectation-sign transformation caused by a Pauli frame.
    ///
    /// This helper is useful when an observable is transformed by a tracked
    /// Pauli correction.
    pub fn frame_sign(&self, frame: &Self) -> PauliResult<i8> {
        if self.qubits() != frame.qubits() {
            return Err(PauliError::DimensionMismatch {
                left: self.qubits(),
                right: frame.qubits(),
            });
        }

        if self.commutes_with(frame) {
            Ok(1)
        } else {
            Ok(-1)
        }
    }
}

impl fmt::Display for PauliString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string_repr())
    }
}

// =============================================================================
// Iterators
// =============================================================================

/// Allocation-free iterator over every qubit and Pauli.
pub struct PauliStringIter<'a> {
    pauli: &'a PauliString,
    index: usize,
}

impl<'a> Iterator for PauliStringIter<'a> {
    type Item = (usize, Pauli);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.pauli.qubits() {
            return None;
        }

        let index = self.index;
        self.index += 1;

        Some((index, self.pauli.get(index)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.pauli.qubits().saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for PauliStringIter<'_> {}

/// Allocation-free iterator over non-identity Pauli terms.
pub struct NonIdentityPauliIter<'a> {
    pauli: &'a PauliString,
    index: usize,
}

impl<'a> Iterator for NonIdentityPauliIter<'a> {
    type Item = (usize, Pauli);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.pauli.qubits() {
            let index = self.index;
            self.index += 1;

            let value = self.pauli.get(index);

            if !value.is_identity() {
                return Some((index, value));
            }
        }

        None
    }
}

// =============================================================================
// Multiplication
// =============================================================================

/// Computes the phase of `left * right`.
///
/// The phase of each individual Pauli string is already encoded as ±1.
/// The additional phase comes from the local Pauli products.
///
/// The returned value is modulo four.
fn multiply_phase(left: &PauliString, right: &PauliString) -> PauliResult<u8> {
    if left.qubits() != right.qubits() {
        return Err(PauliError::DimensionMismatch {
            left: left.qubits(),
            right: right.qubits(),
        });
    }

    let mut phase = (left.phase + right.phase) & 3;

    for word_index in 0..left.x.words().len() {
        let lx = left.x.words()[word_index];
        let lz = left.z.words()[word_index];

        let rx = right.x.words()[word_index];
        let rz = right.z.words()[word_index];

        // For symplectic Pauli encoding P(x,z), the multiplication phase
        // contributed by a pair is determined by the overlap:
        //
        //     z_left · x_right - x_left · z_right
        //
        // modulo four.
        //
        // Counting the two directions separately avoids signed integer
        // arithmetic and therefore avoids overflow concerns.
        let left_z_right_x = (lz & rx).count_ones() as u8;
        let left_x_right_z = (lx & rz).count_ones() as u8;

        phase = (phase + left_z_right_x) & 3;
        phase = (phase + (4 - (left_x_right_z & 3))) & 3;
    }

    Ok(phase)
}

// =============================================================================
// Clifford conjugation helpers
// =============================================================================

/// Clifford conjugation helpers for individual Pauli operators.
///
/// This trait is intentionally small. `algebra::clifford` can build richer
/// gate-level transformations on top of these primitives.
pub trait PauliConjugation {
    /// Conjugates this Pauli string by H on one qubit.
    fn conjugate_h(&mut self, qubit: usize) -> PauliResult<()>;

    /// Conjugates this Pauli string by S on one qubit.
    fn conjugate_s(&mut self, qubit: usize) -> PauliResult<()>;

    /// Conjugates this Pauli string by S† on one qubit.
    fn conjugate_sdg(&mut self, qubit: usize) -> PauliResult<()>;

    /// Conjugates this Pauli string by CNOT.
    fn conjugate_cx(
        &mut self,
        control: usize,
        target: usize,
    ) -> PauliResult<()>;

    /// Conjugates this Pauli string by CZ.
    fn conjugate_cz(
        &mut self,
        control: usize,
        target: usize,
    ) -> PauliResult<()>;
}

impl PauliConjugation for PauliString {
    fn conjugate_h(&mut self, qubit: usize) -> PauliResult<()> {
        if qubit >= self.qubits() {
            return Err(PauliError::QubitOutOfRange {
                qubit,
                qubits: self.qubits(),
            });
        }

        let x = self.x.get(qubit);
        let z = self.z.get(qubit);

        self.x.set(qubit, z)?;
        self.z.set(qubit, x)?;

        // H maps Y -> -Y.
        if x && z {
            self.phase ^= 2;
        }

        Ok(())
    }

    fn conjugate_s(&mut self, qubit: usize) -> PauliResult<()> {
        if qubit >= self.qubits() {
            return Err(PauliError::QubitOutOfRange {
                qubit,
                qubits: self.qubits(),
            });
        }

        // S X S† = Y
        // S Y S† = -X
        // S Z S† = Z
        //
        // The X bit remains unchanged. Z becomes Z xor X.
        let x = self.x.get(qubit);

        if x {
            if self.z.get(qubit) {
                // Y -> -X
                self.phase ^= 2;
            }

            self.z.xor_assign(&single_bitset(self.qubits(), qubit)?);
        }

        Ok(())
    }

    fn conjugate_sdg(&mut self, qubit: usize) -> PauliResult<()> {
        if qubit >= self.qubits() {
            return Err(PauliError::QubitOutOfRange {
                qubit,
                qubits: self.qubits(),
            });
        }

        // S† X S = -Y
        // S† Y S = X
        // S† Z S = Z
        //
        // X is unchanged; Z becomes Z xor X.
        let x = self.x.get(qubit);

        if x {
            if !self.z.get(qubit) {
                // X -> -Y
                self.phase ^= 2;
            }

            self.z.xor_assign(&single_bitset(self.qubits(), qubit)?);
        }

        Ok(())
    }

    fn conjugate_cx(
        &mut self,
        control: usize,
        target: usize,
    ) -> PauliResult<()> {
        validate_two_qubits(self.qubits(), control, target)?;

        // CNOT conjugation in symplectic form:
        //
        // Xc -> Xc Xt
        // Zc -> Zc
        // Xt -> Xt
        // Zt -> Zc Zt
        //
        // The phase correction for the compact representation is determined
        // by the Y/Y interaction. The direct bit transformation below is
        // exact for the Pauli basis.
        let x_control = self.x.get(control);
        let z_target = self.z.get(target);

        if x_control {
            self.x.set(target, !self.x.get(target))?;
        }

        if z_target {
            self.z.set(control, !self.z.get(control))?;
        }

        // The transformation above is symplectic. The sign correction is
        // needed when both transformed local components produce Y parity.
        //
        // A robust way to calculate it is to apply the known Pauli basis
        // transformation through the local symplectic relation.
        if x_control && self.z.get(control) && self.x.get(target) && z_target {
            self.phase ^= 2;
        }

        Ok(())
    }

    fn conjugate_cz(
        &mut self,
        control: usize,
        target: usize,
    ) -> PauliResult<()> {
        validate_two_qubits(self.qubits(), control, target)?;

        // CZ conjugation:
        //
        // Xc -> Xc Zt
        // Zc -> Zc
        // Xt -> Zc Xt
        // Zt -> Zt
        //
        // The X/Z support transformation is symplectic.
        let x_control = self.x.get(control);
        let x_target = self.x.get(target);

        if x_control {
            self.z.set(target, !self.z.get(target))?;
        }

        if x_target {
            self.z.set(control, !self.z.get(control))?;
        }

        // CZ introduces a sign for the appropriate Y/Y sector.
        if x_control && x_target {
            self.phase ^= 2;
        }

        Ok(())
    }
}

// =============================================================================
// Utility functions
// =============================================================================

fn single_bitset(bits: usize, index: usize) -> PauliResult<BitSet> {
    let mut result = BitSet::new(bits)?;
    result.set(index, true)?;
    Ok(result)
}

fn validate_two_qubits(
    qubits: usize,
    first: usize,
    second: usize,
) -> PauliResult<()> {
    if first >= qubits {
        return Err(PauliError::QubitOutOfRange {
            qubit: first,
            qubits,
        });
    }

    if second >= qubits {
        return Err(PauliError::QubitOutOfRange {
            qubit: second,
            qubits,
        });
    }

    if first == second {
        return Err(PauliError::QubitOutOfRange {
            qubit: second,
            qubits,
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

    #[test]
    fn single_pauli_multiplication_is_correct() {
        assert_eq!(
            Pauli::X.multiply(Pauli::X),
            SinglePauliProduct {
                pauli: Pauli::I,
                phase: 0,
            }
        );

        assert_eq!(
            Pauli::X.multiply(Pauli::Y),
            SinglePauliProduct {
                pauli: Pauli::Z,
                phase: 1,
            }
        );

        assert_eq!(
            Pauli::Y.multiply(Pauli::X),
            SinglePauliProduct {
                pauli: Pauli::Z,
                phase: 3,
            }
        );

        assert_eq!(
            Pauli::Y.multiply(Pauli::Z),
            SinglePauliProduct {
                pauli: Pauli::X,
                phase: 1,
            }
        );

        assert_eq!(
            Pauli::Z.multiply(Pauli::Y),
            SinglePauliProduct {
                pauli: Pauli::X,
                phase: 3,
            }
        );
    }

    #[test]
    fn identity_is_correct() {
        let identity = PauliString::identity(128).unwrap();

        assert_eq!(identity.qubits(), 128);
        assert!(identity.is_identity());
        assert_eq!(identity.weight(), 0);
        assert_eq!(identity.sign(), 1);
    }

    #[test]
    fn dense_construction_is_correct() {
        let pauli = PauliString::from_str_repr("IXYZ").unwrap();

        assert_eq!(pauli.qubits(), 4);
        assert_eq!(pauli.get(0), Pauli::I);
        assert_eq!(pauli.get(1), Pauli::X);
        assert_eq!(pauli.get(2), Pauli::Y);
        assert_eq!(pauli.get(3), Pauli::Z);
        assert_eq!(pauli.weight(), 3);
    }

    #[test]
    fn textual_representation_is_correct() {
        let positive = PauliString::from_str_repr("XYZ").unwrap();

        assert_eq!(positive.to_string_repr(), "XYZ");

        let negative = positive.negated();

        assert_eq!(negative.to_string_repr(), "-XYZ");
    }

    #[test]
    fn packed_storage_crosses_word_boundary() {
        let mut pauli = PauliString::identity(130).unwrap();

        pauli.set(0, Pauli::X).unwrap();
        pauli.set(63, Pauli::Y).unwrap();
        pauli.set(64, Pauli::Z).unwrap();
        pauli.set(129, Pauli::X).unwrap();

        assert_eq!(pauli.get(0), Pauli::X);
        assert_eq!(pauli.get(63), Pauli::Y);
        assert_eq!(pauli.get(64), Pauli::Z);
        assert_eq!(pauli.get(129), Pauli::X);
        assert_eq!(pauli.weight(), 4);
    }

    #[test]
    fn commutation_is_correct() {
        let x = PauliString::from_str_repr("X").unwrap();
        let z = PauliString::from_str_repr("Z").unwrap();

        assert!(!x.commutes_with(&z));
        assert!(x.anticommutes_with(&z));

        let xx = PauliString::from_str_repr("XX").unwrap();
        let zz = PauliString::from_str_repr("ZZ").unwrap();

        assert!(xx.commutes_with(&zz));
    }

    #[test]
    fn multiplication_preserves_i_phase() {
        let x = PauliString::from_str_repr("X").unwrap();
        let y = PauliString::from_str_repr("Y").unwrap();

        let product = x.multiply(&y).unwrap();

        assert_eq!(product.get(0), Pauli::Z);
        assert_eq!(product.phase(), 1);
        assert!(!product.is_hermitian());
    }

    #[test]
    fn commuting_product_is_hermitian() {
        let x = PauliString::from_str_repr("X").unwrap();
        let x2 = PauliString::from_str_repr("X").unwrap();

        let product = x.multiply_commuting(&x2).unwrap();

        assert!(product.is_identity());
        assert_eq!(product.phase(), 0);
    }

    #[test]
    fn negative_identity_is_not_positive_identity() {
        let identity = PauliString::identity(4).unwrap();
        let negative = identity.negated();

        assert!(!negative.is_identity());
        assert!(negative.has_identity_support());
        assert_eq!(negative.sign(), -1);
    }

    #[test]
    fn h_conjugation_is_correct() {
        let mut x = PauliString::from_str_repr("X").unwrap();

        x.conjugate_h(0).unwrap();

        assert_eq!(x.get(0), Pauli::Z);
        assert_eq!(x.sign(), 1);

        let mut y = PauliString::from_str_repr("Y").unwrap();

        y.conjugate_h(0).unwrap();

        assert_eq!(y.get(0), Pauli::Y);
        assert_eq!(y.sign(), -1);
    }

    #[test]
    fn s_conjugation_is_correct() {
        let mut x = PauliString::from_str_repr("X").unwrap();

        x.conjugate_s(0).unwrap();

        assert_eq!(x.get(0), Pauli::Y);
        assert_eq!(x.sign(), 1);

        let mut y = PauliString::from_str_repr("Y").unwrap();

        y.conjugate_s(0).unwrap();

        assert_eq!(y.get(0), Pauli::X);
        assert_eq!(y.sign(), -1);
    }

    #[test]
    fn sdg_conjugation_is_correct() {
        let mut x = PauliString::from_str_repr("X").unwrap();

        x.conjugate_sdg(0).unwrap();

        assert_eq!(x.get(0), Pauli::Y);
        assert_eq!(x.sign(), -1);

        let mut y = PauliString::from_str_repr("Y").unwrap();

        y.conjugate_sdg(0).unwrap();

        assert_eq!(y.get(0), Pauli::X);
        assert_eq!(y.sign(), 1);
    }

    #[test]
    fn cx_conjugation_basic_cases() {
        let mut control_x =
            PauliString::from_str_repr("XI").unwrap();

        control_x.conjugate_cx(0, 1).unwrap();

        assert_eq!(
            control_x.to_string_repr(),
            "XX"
        );

        let mut target_z =
            PauliString::from_str_repr("IZ").unwrap();

        target_z.conjugate_cx(0, 1).unwrap();

        assert_eq!(
            target_z.to_string_repr(),
            "ZZ"
        );
    }

    #[test]
    fn cz_conjugation_basic_cases() {
        let mut control_x =
            PauliString::from_str_repr("XI").unwrap();

        control_x.conjugate_cz(0, 1).unwrap();

        assert_eq!(
            control_x.to_string_repr(),
            "XZ"
        );

        let mut target_x =
            PauliString::from_str_repr("IX").unwrap();

        target_x.conjugate_cz(0, 1).unwrap();

        assert_eq!(
            target_x.to_string_repr(),
            "ZX"
        );
    }

    #[test]
    fn iterator_is_allocation_free_at_use_site() {
        let pauli =
            PauliString::from_str_repr("IXYZ").unwrap();

        let values: Vec<_> = pauli.iter().collect();

        assert_eq!(
            values,
            vec![
                (0, Pauli::I),
                (1, Pauli::X),
                (2, Pauli::Y),
                (3, Pauli::Z),
            ]
        );
    }

    #[test]
    fn non_identity_iterator_is_correct() {
        let pauli =
            PauliString::from_str_repr("IXYZ").unwrap();

        let values: Vec<_> =
            pauli.non_identity_iter().collect();

        assert_eq!(
            values,
            vec![
                (1, Pauli::X),
                (2, Pauli::Y),
                (3, Pauli::Z),
            ]
        );
    }

    #[test]
    fn symplectic_distance_is_correct() {
        let left =
            PauliString::from_str_repr("XYZ").unwrap();

        let right =
            PauliString::from_str_repr("XII").unwrap();

        assert_eq!(
            left.symplectic_distance(&right).unwrap(),
            3
        );
    }

    #[test]
    fn highest_non_identity_qubit_is_correct() {
        let pauli =
            PauliString::from_str_repr("IIXIIIZ").unwrap();

        assert_eq!(
            pauli.highest_non_identity_qubit(),
            Some(6)
        );
    }

    #[test]
    fn pauli_frame_sign_is_correct() {
        let observable =
            PauliString::from_str_repr("Z").unwrap();

        let commuting_frame =
            PauliString::from_str_repr("Z").unwrap();

        let anticommuting_frame =
            PauliString::from_str_repr("X").unwrap();

        assert_eq!(
            observable.frame_sign(&commuting_frame).unwrap(),
            1
        );

        assert_eq!(
            observable.frame_sign(&anticommuting_frame).unwrap(),
            -1
        );
    }

    #[test]
    fn very_wide_strings_use_word_packing() {
        let pauli =
            PauliString::identity(1_000_000).unwrap();

        assert_eq!(pauli.qubits(), 1_000_000);
        assert_eq!(pauli.weight(), 0);
        assert!(pauli.x_words().len() < 20_000);
    }

    #[test]
    fn invalid_qubit_is_rejected() {
        let mut pauli =
            PauliString::identity(2).unwrap();

        assert_eq!(
            pauli.set(2, Pauli::X),
            Err(PauliError::QubitOutOfRange {
                qubit: 2,
                qubits: 2,
            })
        );
    }

    #[test]
    fn dimension_mismatch_is_rejected() {
        let left =
            PauliString::identity(2).unwrap();

        let right =
            PauliString::identity(3).unwrap();

        assert_eq!(
            left.multiply(&right),
            Err(PauliError::DimensionMismatch {
                left: 2,
                right: 3,
            })
        );
    }

    #[test]
    fn zero_and_nonzero_phase_are_distinguished() {
        let positive =
            PauliString::from_str_repr("III").unwrap();

        let negative =
            positive.clone().negated();

        assert_eq!(positive.phase(), 0);
        assert_eq!(negative.phase(), 2);
        assert_ne!(positive, negative);
    }
}