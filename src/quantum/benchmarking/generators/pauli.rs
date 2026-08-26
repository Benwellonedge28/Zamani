//! Zamani Quantum Benchmarking — Pauli generation and algebra.
//!
//! This module is the canonical Pauli utility layer for benchmark generators.
//!
//! # Architectural role
//!
//! `pauli.rs` owns:
//!
//! - single-qubit Pauli identity (`I`), `X`, `Y`, and `Z`;
//! - phase-aware Pauli multiplication;
//! - tensor-product Pauli words;
//! - symplectic (`X/Z`) representation;
//! - Pauli weight and commutation tests;
//! - deterministic random Pauli generation through `generators::random`;
//! - Pauli twirling and random Pauli sampling helpers.
//!
//! It deliberately does **not** own:
//!
//! - Quantum IR gates or circuits;
//! - hardware topology;
//! - execution;
//! - calibration;
//! - Clifford generation;
//! - cycle-benchmarking statistics;
//! - benchmark result/reporting.
//!
//! The dependency direction is:
//!
//! ```text
//! generators/random.rs
//!        │
//!        ▼
//! generators/pauli.rs
//!        │
//!        ├──────────────► cycle_benchmarking
//!        ├──────────────► randomized_compiling
//!        ├──────────────► randomized_benchmarking
//!        └──────────────► future Pauli-based protocols
//! ```
//!
//! The Pauli implementation is intentionally independent of `quantum::ir`.
//! A future protocol can translate a `PauliWord` into IR operations at its
//! own boundary without making the mathematical generator depend on the IR.
//!
//! # Reproducibility
//!
//! All random generation accepts an explicit `RandomSource`. Callers should
//! normally pass a domain-separated `RandomStream` created from the canonical
//! benchmark RNG in `generators/random.rs`.
//!
//! No global RNG, thread-local RNG, or system time is used by this module.
//!
//! # Phase convention
//!
//! A Pauli word is represented as:
//!
//! ```text
//! i^phase × P0 ⊗ P1 ⊗ ... ⊗ P(n-1)
//! ```
//!
//! where `phase` is one of `1`, `i`, `-1`, `-i`, represented by
//! `PauliPhase`.
//!
//! The phase is retained because Pauli multiplication is not merely a
//! bitwise operation:
//!
//! ```text
//! X * Y =  i Z
//! Y * X = -i Z
//! ```
//!
//! For benchmarking protocols that care only about the physical Pauli
//! operator up to global phase, use `phase_ignored()` or the symplectic
//! representation explicitly.
//!
//! # Symplectic representation
//!
//! Each single-qubit Pauli is represented by two bits `(x, z)`:
//!
//! ```text
//! I = (0, 0)
//! X = (1, 0)
//! Z = (0, 1)
//! Y = (1, 1)
//! ```
//!
//! The phase is kept separately.
//!
//! # Integration contract
//!
//! This file is complete without requiring later modifications when these
//! downstream files are implemented:
//!
//! ```text
//! generators/random.rs
//!        │
//!        ▼
//! generators/pauli.rs
//!        │
//!        ├── generators/clifford.rs
//!        ├── protocols/cycle_benchmarking.rs
//!        ├── protocols/randomized_benchmarking.rs
//!        ├── protocols/purity_rb.rs
//!        ├── protocols/leakage_rb.rs
//!        └── future Pauli-based protocols
//! ```
//!
//! Downstream modules should depend on the public API here rather than
//! reimplementing Pauli enumeration, multiplication, symplectic commutation,
//! or random Pauli selection.
//!
//! # Resource limits
//!
//! This module intentionally does not duplicate `benchmarking::core::limits`.
//! Callers accepting untrusted benchmark configurations must validate the
//! requested qubit count against the canonical benchmark limits before
//! allocating a `PauliWord`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - edition 2021
//!
//! No nightly features are required.

use std::fmt;
use std::str::FromStr;

use super::random::{
    BenchmarkSeed,
    RandomError,
    RandomSource,
    RandomStream,
};

// =============================================================================
// Public constants
// =============================================================================

/// Stable API version for the Pauli generator/algebra contract.
pub const PAULI_API_VERSION: u32 = 1;

/// Stable identifier for the Pauli convention.
///
/// This identifier belongs in benchmark provenance when Pauli generation
/// affects a reproducible experiment.
pub const PAULI_CONVENTION_ID: &str =
    "pauli-i-x-y-z-symplectic-phase-v1";

/// Number of possible single-qubit Pauli operators.
pub const SINGLE_QUBIT_PAULI_COUNT: usize = 4;

/// Number of possible Pauli phases.
pub const PAULI_PHASE_COUNT: usize = 4;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by Pauli generation and algebra.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PauliError {
    /// A Pauli word was requested with an invalid length.
    InvalidLength {
        length: usize,
    },

    /// A Pauli index was outside the valid range.
    InvalidIndex {
        index: usize,
    },

    /// Textual Pauli syntax was malformed.
    InvalidText {
        reason: String,
    },

    /// A phase value was invalid.
    InvalidPhase {
        phase: u8,
    },

    /// Random generation failed.
    Random(RandomError),
}

impl fmt::Display for PauliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { length } => {
                write!(f, "invalid Pauli word length: {length}")
            }

            Self::InvalidIndex { index } => {
                write!(
                    f,
                    "invalid Pauli index {index}; expected 0..4"
                )
            }

            Self::InvalidText { reason } => {
                write!(f, "invalid Pauli text: {reason}")
            }

            Self::InvalidPhase { phase } => {
                write!(
                    f,
                    "invalid Pauli phase {phase}; expected 0..4"
                )
            }

            Self::Random(error) => {
                write!(
                    f,
                    "Pauli random generation failed: {error}"
                )
            }
        }
    }
}

impl std::error::Error for PauliError {}

impl From<RandomError> for PauliError {
    fn from(error: RandomError) -> Self {
        Self::Random(error)
    }
}

/// Result type for Pauli operations.
pub type PauliResult<T> = Result<T, PauliError>;

// =============================================================================
// Single-qubit Pauli
// =============================================================================

/// The four single-qubit Pauli operators.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum Pauli {
    /// Identity.
    I,

    /// Pauli-X.
    X,

    /// Pauli-Y.
    Y,

    /// Pauli-Z.
    Z,
}

impl Pauli {
    /// Returns the canonical single-character representation.
    pub const fn symbol(self) -> char {
        match self {
            Self::I => 'I',
            Self::X => 'X',
            Self::Y => 'Y',
            Self::Z => 'Z',
        }
    }

    /// Returns the canonical integer encoding.
    ///
    /// ```text
    /// I = 0
    /// X = 1
    /// Y = 2
    /// Z = 3
    /// ```
    pub const fn index(self) -> usize {
        match self {
            Self::I => 0,
            Self::X => 1,
            Self::Y => 2,
            Self::Z => 3,
        }
    }

    /// Converts the canonical integer encoding into a Pauli.
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::I),
            1 => Some(Self::X),
            2 => Some(Self::Y),
            3 => Some(Self::Z),
            _ => None,
        }
    }

    /// Returns the `(x, z)` symplectic bits.
    ///
    /// ```text
    /// I = (0, 0)
    /// X = (1, 0)
    /// Y = (1, 1)
    /// Z = (0, 1)
    /// ```
    pub const fn symplectic_bits(self) -> (bool, bool) {
        match self {
            Self::I => (false, false),
            Self::X => (true, false),
            Self::Y => (true, true),
            Self::Z => (false, true),
        }
    }

    /// Constructs a Pauli from `(x, z)` symplectic bits.
    pub const fn from_symplectic_bits(
        x: bool,
        z: bool,
    ) -> Self {
        match (x, z) {
            (false, false) => Self::I,
            (true, false) => Self::X,
            (true, true) => Self::Y,
            (false, true) => Self::Z,
        }
    }

    /// Returns whether this Pauli is identity.
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::I)
    }

    /// Returns the single-qubit Pauli weight.
    pub const fn weight(self) -> usize {
        if self.is_identity() {
            0
        } else {
            1
        }
    }

    /// Every Pauli is Hermitian and therefore its own inverse up to phase.
    ///
    /// At the single-qubit operator level, this returns the same Pauli factor.
    pub const fn inverse(self) -> Self {
        self
    }

    /// Returns whether two single-qubit Paulis commute.
    pub const fn commutes_with(self, other: Self) -> bool {
        let (x1, z1) = self.symplectic_bits();
        let (x2, z2) = other.symplectic_bits();

        // Symplectic product:
        //
        // x1*z2 + z1*x2 (mod 2)
        //
        // A zero product means commute.
        (x1 && z2) == (z1 && x2)
    }

    /// Multiplies two single-qubit Paulis exactly.
    ///
    /// The returned phase is required because Pauli multiplication is
    /// non-commutative.
    pub const fn multiply(
        self,
        other: Self,
    ) -> (PauliPhase, Self) {
        match (self, other) {
            (Self::I, p) | (p, Self::I) => {
                (PauliPhase::One, p)
            }

            (Self::X, Self::X)
            | (Self::Y, Self::Y)
            | (Self::Z, Self::Z) => {
                (PauliPhase::One, Self::I)
            }

            (Self::X, Self::Y) => {
                (PauliPhase::PlusI, Self::Z)
            }

            (Self::Y, Self::X) => {
                (PauliPhase::MinusI, Self::Z)
            }

            (Self::Y, Self::Z) => {
                (PauliPhase::PlusI, Self::X)
            }

            (Self::Z, Self::Y) => {
                (PauliPhase::MinusI, Self::X)
            }

            (Self::Z, Self::X) => {
                (PauliPhase::PlusI, Self::Y)
            }

            (Self::X, Self::Z) => {
                (PauliPhase::MinusI, Self::Y)
            }
        }
    }

    /// Samples a uniformly distributed Pauli from `I`, `X`, `Y`, `Z`.
    pub fn random<R: RandomSource + ?Sized>(
        rng: &mut R,
    ) -> PauliResult<Self> {
        let index = rng.range_usize(
            0,
            SINGLE_QUBIT_PAULI_COUNT,
        )?;

        Self::from_index(index)
            .ok_or(PauliError::InvalidIndex { index })
    }

    /// Samples a uniformly distributed non-identity Pauli from
    /// `X`, `Y`, `Z`.
    pub fn random_non_identity<R: RandomSource + ?Sized>(
        rng: &mut R,
    ) -> PauliResult<Self> {
        let index = rng.range_usize(
            1,
            SINGLE_QUBIT_PAULI_COUNT,
        )?;

        Self::from_index(index)
            .ok_or(PauliError::InvalidIndex { index })
    }
}

impl fmt::Display for Pauli {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(&self.symbol().to_string())
    }
}

impl FromStr for Pauli {
    type Err = PauliError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "I" | "i" => Ok(Self::I),
            "X" | "x" => Ok(Self::X),
            "Y" | "y" => Ok(Self::Y),
            "Z" | "z" => Ok(Self::Z),

            _ => Err(PauliError::InvalidText {
                reason: format!(
                    "expected I, X, Y, or Z; got {value:?}"
                ),
            }),
        }
    }
}

// =============================================================================
// Pauli phase
// =============================================================================

/// A Pauli global phase.
///
/// Values represent:
///
/// ```text
/// One      =  1
/// PlusI    =  i
/// MinusOne = -1
/// MinusI   = -i
/// ```
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum PauliPhase {
    /// `1`.
    One,

    /// `i`.
    PlusI,

    /// `-1`.
    MinusOne,

    /// `-i`.
    MinusI,
}

impl PauliPhase {
    /// Returns the exponent `k` in `i^k`.
    pub const fn exponent(self) -> u8 {
        match self {
            Self::One => 0,
            Self::PlusI => 1,
            Self::MinusOne => 2,
            Self::MinusI => 3,
        }
    }

    /// Constructs a phase from an exponent modulo four.
    pub const fn from_exponent(
        exponent: u8,
    ) -> Self {
        match exponent & 3 {
            0 => Self::One,
            1 => Self::PlusI,
            2 => Self::MinusOne,
            _ => Self::MinusI,
        }
    }

    /// Returns the multiplicative inverse phase.
    pub const fn inverse(self) -> Self {
        match self {
            Self::One => Self::One,
            Self::PlusI => Self::MinusI,
            Self::MinusOne => Self::MinusOne,
            Self::MinusI => Self::PlusI,
        }
    }

    /// Multiplies two phases.
    pub const fn multiply(
        self,
        other: Self,
    ) -> Self {
        Self::from_exponent(
            self.exponent() + other.exponent(),
        )
    }

    /// Returns `(real, imaginary)`.
    pub const fn complex(self) -> (f64, f64) {
        match self {
            Self::One => (1.0, 0.0),
            Self::PlusI => (0.0, 1.0),
            Self::MinusOne => (-1.0, 0.0),
            Self::MinusI => (0.0, -1.0),
        }
    }

    /// Returns whether this is the identity phase.
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::One)
    }
}

impl fmt::Display for PauliPhase {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        f.write_str(match self {
            Self::One => "1",
            Self::PlusI => "i",
            Self::MinusOne => "-1",
            Self::MinusI => "-i",
        })
    }
}

// =============================================================================
// Symplectic representation
// =============================================================================

/// Phase-free symplectic representation of a Pauli word.
///
/// For `n` qubits:
///
/// ```text
/// I = 00
/// X = 10
/// Y = 11
/// Z = 01
/// ```
///
/// The global phase is intentionally excluded.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SymplecticPauli {
    x: Vec<bool>,
    z: Vec<bool>,
}

impl SymplecticPauli {
    /// Constructs a symplectic Pauli from explicit X/Z bit vectors.
    pub fn new(
        x: Vec<bool>,
        z: Vec<bool>,
    ) -> PauliResult<Self> {
        if x.len() != z.len() {
            return Err(PauliError::InvalidLength {
                length: x.len(),
            });
        }

        Ok(Self { x, z })
    }

    /// Returns the number of represented qubits.
    pub fn len(&self) -> usize {
        self.x.len()
    }

    /// Returns whether this representation is empty.
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }

    /// Returns X bits.
    pub fn x_bits(&self) -> &[bool] {
        &self.x
    }

    /// Returns Z bits.
    pub fn z_bits(&self) -> &[bool] {
        &self.z
    }

    /// Returns one Pauli factor.
    pub fn get(
        &self,
        index: usize,
    ) -> Option<Pauli> {
        if index >= self.len() {
            return None;
        }

        Some(Pauli::from_symplectic_bits(
            self.x[index],
            self.z[index],
        ))
    }

    /// Calculates the binary symplectic product.
    ///
    /// `false` means commute.
    /// `true` means anticommute.
    pub fn symplectic_product(
        &self,
        other: &Self,
    ) -> PauliResult<bool> {
        if self.len() != other.len() {
            return Err(PauliError::InvalidLength {
                length: other.len(),
            });
        }

        let mut parity = false;

        for index in 0..self.len() {
            parity ^= self.x[index] & other.z[index];
            parity ^= self.z[index] & other.x[index];
        }

        Ok(parity)
    }

    /// Tests whether two symplectic Paulis commute.
    pub fn commutes_with(
        &self,
        other: &Self,
    ) -> PauliResult<bool> {
        Ok(!self.symplectic_product(other)?)
    }

    /// Converts the representation into a phase-free Pauli word.
    pub fn to_pauli_word(&self) -> PauliWord {
        let mut factors =
            Vec::with_capacity(self.len());

        for index in 0..self.len() {
            factors.push(
                Pauli::from_symplectic_bits(
                    self.x[index],
                    self.z[index],
                ),
            );
        }

        // Lengths are already guaranteed equal by this type.
        PauliWord {
            factors,
            phase: PauliPhase::One,
        }
    }
}

// =============================================================================
// Pauli word
// =============================================================================

/// A tensor-product Pauli operator with explicit global phase.
///
/// Mathematical representation:
///
/// ```text
/// i^phase × P[0] ⊗ P[1] ⊗ ... ⊗ P[n-1]
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PauliWord {
    factors: Vec<Pauli>,
    phase: PauliPhase,
}

impl PauliWord {
    /// Creates a phase-free Pauli word.
    pub fn new(
        factors: Vec<Pauli>,
    ) -> PauliResult<Self> {
        Self::with_phase(
            factors,
            PauliPhase::One,
        )
    }

    /// Creates a Pauli word with an explicit phase.
    pub fn with_phase(
        factors: Vec<Pauli>,
        phase: PauliPhase,
    ) -> PauliResult<Self> {
        Ok(Self { factors, phase })
    }

    /// Creates an identity operator on `qubits` qubits.
    pub fn identity(
        qubits: usize,
    ) -> PauliResult<Self> {
        Ok(Self {
            factors: vec![Pauli::I; qubits],
            phase: PauliPhase::One,
        })
    }

    /// Returns the number of qubits.
    pub fn len(&self) -> usize {
        self.factors.len()
    }

    /// Returns whether this word has zero factors.
    pub fn is_empty(&self) -> bool {
        self.factors.is_empty()
    }

    /// Returns the global phase.
    pub const fn phase(&self) -> PauliPhase {
        self.phase
    }

    /// Returns the phase exponent.
    pub const fn phase_exponent(&self) -> u8 {
        self.phase.exponent()
    }

    /// Returns all Pauli factors.
    pub fn factors(&self) -> &[Pauli] {
        &self.factors
    }

    /// Returns one factor.
    pub fn get(
        &self,
        index: usize,
    ) -> Option<Pauli> {
        self.factors.get(index).copied()
    }

    /// Returns the number of non-identity factors.
    pub fn weight(&self) -> usize {
        self.factors
            .iter()
            .map(|pauli| pauli.weight())
            .sum()
    }

    /// Returns whether the word is exactly the identity.
    pub fn is_identity(&self) -> bool {
        self.phase.is_identity() && self.weight() == 0
    }

    /// Returns whether the Pauli factor is identity regardless of global
    /// phase.
    pub fn is_identity_up_to_phase(&self) -> bool {
        self.weight() == 0
    }

    /// Returns a copy with global phase removed.
    pub fn phase_ignored(&self) -> Self {
        Self {
            factors: self.factors.clone(),
            phase: PauliPhase::One,
        }
    }

    /// Returns the exact inverse.
    pub fn inverse(&self) -> Self {
        Self {
            factors: self.factors.clone(),
            phase: self.phase.inverse(),
        }
    }

    /// Converts to phase-free symplectic representation.
    pub fn symplectic(&self) -> SymplecticPauli {
        let mut x =
            Vec::with_capacity(self.len());

        let mut z =
            Vec::with_capacity(self.len());

        for factor in &self.factors {
            let (x_bit, z_bit) =
                factor.symplectic_bits();

            x.push(x_bit);
            z.push(z_bit);
        }

        SymplecticPauli { x, z }
    }

    /// Tests whether two Pauli words commute.
    pub fn commutes_with(
        &self,
        other: &Self,
    ) -> PauliResult<bool> {
        self.symplectic()
            .commutes_with(&other.symplectic())
    }

    /// Multiplies two Pauli words exactly, including phase.
    pub fn multiply(
        &self,
        other: &Self,
    ) -> PauliResult<Self> {
        if self.len() != other.len() {
            return Err(PauliError::InvalidLength {
                length: other.len(),
            });
        }

        let mut factors =
            Vec::with_capacity(self.len());

        let mut phase =
            self.phase.multiply(other.phase);

        for index in 0..self.len() {
            let (local_phase, local_pauli) =
                self.factors[index]
                    .multiply(other.factors[index]);

            phase = phase.multiply(local_phase);
            factors.push(local_pauli);
        }

        Ok(Self { factors, phase })
    }

    /// Forms the tensor product.
    ///
    /// This is concatenation of tensor factors, not matrix multiplication.
    pub fn tensor(
        &self,
        other: &Self,
    ) -> Self {
        let mut factors =
            Vec::with_capacity(self.len() + other.len());

        factors.extend_from_slice(&self.factors);
        factors.extend_from_slice(&other.factors);

        Self {
            factors,
            phase: self.phase.multiply(other.phase),
        }
    }

    /// Returns a copy with one factor replaced.
    pub fn with_factor(
        &self,
        index: usize,
        factor: Pauli,
    ) -> PauliResult<Self> {
        if index >= self.len() {
            return Err(PauliError::InvalidIndex {
                index,
            });
        }

        let mut factors =
            self.factors.clone();

        factors[index] = factor;

        Ok(Self {
            factors,
            phase: self.phase,
        })
    }

    /// Generates an independent uniformly random Pauli factor on every
    /// qubit.
    ///
    /// Each factor is independently sampled from `I/X/Y/Z`.
    pub fn random<R: RandomSource + ?Sized>(
        qubits: usize,
        rng: &mut R,
    ) -> PauliResult<Self> {
        let mut factors =
            Vec::with_capacity(qubits);

        for _ in 0..qubits {
            factors.push(Pauli::random(rng)?);
        }

        Self::new(factors)
    }

    /// Generates a Pauli word where each factor is identity with the supplied
    /// probability and otherwise uniformly sampled from X/Y/Z.
    ///
    /// `identity_probability` must be finite and in `[0, 1]`.
    pub fn random_with_identity_probability<
        R: RandomSource + ?Sized,
    >(
        qubits: usize,
        identity_probability: f64,
        rng: &mut R,
    ) -> PauliResult<Self> {
        validate_probability(
            identity_probability,
        )?;

        let mut factors =
            Vec::with_capacity(qubits);

        for _ in 0..qubits {
            let selector = rng.next_f64()?;

            if selector < identity_probability {
                factors.push(Pauli::I);
            } else {
                factors.push(
                    Pauli::random_non_identity(rng)?,
                );
            }
        }

        Self::new(factors)
    }

    /// Generates a guaranteed non-identity Pauli word.
    ///
    /// The result is guaranteed to contain at least one X/Y/Z factor.
    ///
    /// This method deliberately does **not** claim uniformity over all
    /// non-identity Pauli words. For an exactly uniform distribution over
    /// non-identity words, use `random_uniform_non_identity`.
    pub fn random_non_identity<R: RandomSource + ?Sized>(
        qubits: usize,
        rng: &mut R,
    ) -> PauliResult<Self> {
        if qubits == 0 {
            return Err(PauliError::InvalidLength {
                length: 0,
            });
        }

        let mandatory =
            rng.range_usize(0, qubits)?;

        let mut factors =
            Vec::with_capacity(qubits);

        for index in 0..qubits {
            if index == mandatory {
                factors.push(
                    Pauli::random_non_identity(rng)?,
                );
            } else {
                factors.push(Pauli::random(rng)?);
            }
        }

        Self::new(factors)
    }

    /// Generates a uniformly random non-identity Pauli word.
    ///
    /// Every one of the `4^n - 1` non-identity Pauli words has equal
    /// probability.
    ///
    /// Rejection sampling is used. For ordinary benchmark widths the
    /// expected number of attempts is very close to one.
    pub fn random_uniform_non_identity<
        R: RandomSource + ?Sized,
    >(
        qubits: usize,
        rng: &mut R,
    ) -> PauliResult<Self> {
        if qubits == 0 {
            return Err(PauliError::InvalidLength {
                length: 0,
            });
        }

        loop {
            let word = Self::random(
                qubits,
                rng,
            )?;

            if !word.is_identity_up_to_phase() {
                return Ok(word);
            }
        }
    }

    /// Generates a Pauli word with exactly `weight` non-identity factors.
    ///
    /// Every subset of positions of the requested weight is sampled
    /// uniformly, and each selected position receives an independent
    /// uniformly sampled X/Y/Z factor.
    pub fn random_exact_weight<
        R: RandomSource + ?Sized,
    >(
        qubits: usize,
        weight: usize,
        rng: &mut R,
    ) -> PauliResult<Self> {
        if weight > qubits {
            return Err(PauliError::InvalidLength {
                length: weight,
            });
        }

        let mut factors =
            vec![Pauli::I; qubits];

        if weight == 0 {
            return Self::new(factors);
        }

        let mut positions: Vec<usize> =
            (0..qubits).collect();

        for index in 0..weight {
            let selected =
                rng.range_usize(index, qubits)?;

            positions.swap(index, selected);

            factors[positions[index]] =
                Pauli::random_non_identity(rng)?;
        }

        Self::new(factors)
    }

    /// Generates a random Pauli acting non-trivially on exactly one random
    /// qubit.
    pub fn random_single_site<
        R: RandomSource + ?Sized,
    >(
        qubits: usize,
        rng: &mut R,
    ) -> PauliResult<Self> {
        if qubits == 0 {
            return Err(PauliError::InvalidLength {
                length: 0,
            });
        }

        let index =
            rng.range_usize(0, qubits)?;

        let factor =
            Pauli::random_non_identity(rng)?;

        let mut word =
            Self::identity(qubits)?;

        word.factors[index] = factor;

        Ok(word)
    }

    /// Deterministic convenience constructor for tests and fixtures.
    ///
    /// This uses the same canonical RNG as all other benchmark generation.
    pub fn random_with_seed(
        qubits: usize,
        seed: u64,
    ) -> PauliResult<Self> {
        let benchmark_seed =
            BenchmarkSeed::from_u64(seed);

        let mut stream =
            RandomStream::from_seed(
                benchmark_seed,
            );

        Self::random(
            qubits,
            &mut stream,
        )
    }
}

impl fmt::Display for PauliWord {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        if !self.phase.is_identity() {
            write!(f, "{} ", self.phase)?;
        }

        for factor in &self.factors {
            write!(f, "{}", factor.symbol())?;
        }

        Ok(())
    }
}

impl FromStr for PauliWord {
    type Err = PauliError;

    fn from_str(
        value: &str,
    ) -> Result<Self, Self::Err> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(PauliError::InvalidText {
                reason:
                    "Pauli word cannot be empty"
                        .to_string(),
            });
        }

        let (phase, body) =
            if let Some(rest) =
                trimmed.strip_prefix("-i ")
            {
                (PauliPhase::MinusI, rest)
            } else if let Some(rest) =
                trimmed.strip_prefix("i ")
            {
                (PauliPhase::PlusI, rest)
            } else if let Some(rest) =
                trimmed.strip_prefix("-1 ")
            {
                (PauliPhase::MinusOne, rest)
            } else if let Some(rest) =
                trimmed.strip_prefix("1 ")
            {
                (PauliPhase::One, rest)
            } else {
                (PauliPhase::One, trimmed)
            };

        if body.is_empty() {
            return Err(PauliError::InvalidText {
                reason:
                    "Pauli word has no factors"
                        .to_string(),
            });
        }

        let mut factors =
            Vec::with_capacity(body.len());

        for character in body.chars() {
            let factor = match character {
                'I' | 'i' => Pauli::I,
                'X' | 'x' => Pauli::X,
                'Y' | 'y' => Pauli::Y,
                'Z' | 'z' => Pauli::Z,

                _ => {
                    return Err(
                        PauliError::InvalidText {
                            reason: format!(
                                "invalid Pauli factor {character:?}"
                            ),
                        },
                    );
                }
            };

            factors.push(factor);
        }

        Self::with_phase(
            factors,
            phase,
        )
    }
}

// =============================================================================
// Canonical bases
// =============================================================================

/// Returns the canonical single-qubit Pauli basis.
///
/// Ordering is stable and must not be changed:
///
/// ```text
/// I, X, Y, Z
/// ```
pub const fn single_qubit_basis()
    -> [Pauli; SINGLE_QUBIT_PAULI_COUNT]
{
    [
        Pauli::I,
        Pauli::X,
        Pauli::Y,
        Pauli::Z,
    ]
}

/// Returns the canonical non-identity Pauli basis.
///
/// Ordering is stable:
///
/// ```text
/// X, Y, Z
/// ```
pub const fn non_identity_single_qubit_basis()
    -> [Pauli; 3]
{
    [
        Pauli::X,
        Pauli::Y,
        Pauli::Z,
    ]
}

// =============================================================================
// Twirling helpers
// =============================================================================

/// Samples a uniformly random single-qubit Pauli twirl.
pub fn random_single_qubit_twirl<
    R: RandomSource + ?Sized,
>(
    rng: &mut R,
) -> PauliResult<Pauli> {
    Pauli::random(rng)
}

/// Samples an independent random Pauli from I/X/Y/Z on every qubit.
pub fn random_pauli_twirl<
    R: RandomSource + ?Sized,
>(
    qubits: usize,
    rng: &mut R,
) -> PauliResult<PauliWord> {
    PauliWord::random(
        qubits,
        rng,
    )
}

/// Samples a Pauli word with exactly one non-identity factor per qubit.
///
/// Therefore every factor is X/Y/Z.
pub fn random_non_identity_twirl<
    R: RandomSource + ?Sized,
>(
    qubits: usize,
    rng: &mut R,
) -> PauliResult<PauliWord> {
    PauliWord::random_exact_weight(
        qubits,
        qubits,
        rng,
    )
}

/// Samples a uniformly random non-identity Pauli word.
pub fn random_uniform_non_identity_twirl<
    R: RandomSource + ?Sized,
>(
    qubits: usize,
    rng: &mut R,
) -> PauliResult<PauliWord> {
    PauliWord::random_uniform_non_identity(
        qubits,
        rng,
    )
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates a probability used by randomized benchmark generation.
fn validate_probability(
    value: f64,
) -> PauliResult<()> {
    if !value.is_finite()
        || !(0.0..=1.0).contains(&value)
    {
        return Err(PauliError::InvalidText {
            reason: format!(
                "probability must be finite and in [0, 1], got {value}"
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
    use super::super::random::{
        BenchmarkSeed,
        RandomStream,
    };

    fn stream() -> RandomStream {
        RandomStream::from_seed(
            BenchmarkSeed::from_u64(
                0x5A4D_4E49,
            ),
        )
    }

    #[test]
    fn single_qubit_index_round_trip() {
        for index in 0..SINGLE_QUBIT_PAULI_COUNT {
            let pauli =
                Pauli::from_index(index)
                    .expect("valid Pauli index");

            assert_eq!(
                pauli.index(),
                index
            );
        }
    }

    #[test]
    fn single_qubit_multiplication_is_phase_aware() {
        assert_eq!(
            Pauli::X.multiply(Pauli::Y),
            (
                PauliPhase::PlusI,
                Pauli::Z
            )
        );

        assert_eq!(
            Pauli::Y.multiply(Pauli::X),
            (
                PauliPhase::MinusI,
                Pauli::Z
            )
        );

        assert_eq!(
            Pauli::X.multiply(Pauli::X),
            (
                PauliPhase::One,
                Pauli::I
            )
        );
    }

    #[test]
    fn phase_multiplication_is_mod_four() {
        assert_eq!(
            PauliPhase::PlusI
                .multiply(PauliPhase::PlusI),
            PauliPhase::MinusOne
        );

        assert_eq!(
            PauliPhase::MinusI
                .multiply(PauliPhase::MinusI),
            PauliPhase::MinusOne
        );
    }

    #[test]
    fn phase_inverse_is_correct() {
        assert_eq!(
            PauliPhase::PlusI.inverse(),
            PauliPhase::MinusI
        );

        assert_eq!(
            PauliPhase::MinusI.inverse(),
            PauliPhase::PlusI
        );

        assert_eq!(
            PauliPhase::MinusOne.inverse(),
            PauliPhase::MinusOne
        );
    }

    #[test]
    fn symplectic_bits_round_trip() {
        for pauli in single_qubit_basis() {
            let (x, z) =
                pauli.symplectic_bits();

            assert_eq!(
                Pauli::from_symplectic_bits(x, z),
                pauli
            );
        }
    }

    #[test]
    fn pauli_word_has_correct_weight() {
        let word =
            PauliWord::new(vec![
                Pauli::I,
                Pauli::X,
                Pauli::Y,
                Pauli::Z,
            ])
            .expect("valid word");

        assert_eq!(
            word.weight(),
            3
        );

        assert!(!word.is_identity());
        assert!(
            !word.is_identity_up_to_phase()
        );
    }

    #[test]
    fn identity_word_is_detected() {
        let word =
            PauliWord::identity(8)
                .expect("valid identity");

        assert!(word.is_identity());
        assert!(
            word.is_identity_up_to_phase()
        );
        assert_eq!(
            word.weight(),
            0
        );
    }

    #[test]
    fn commuting_and_anticommuting_words_are_detected() {
        let x =
            PauliWord::new(vec![Pauli::X])
                .expect("valid");

        let z =
            PauliWord::new(vec![Pauli::Z])
                .expect("valid");

        assert!(
            !x.commutes_with(&z)
                .expect("same width")
        );

        let xx =
            PauliWord::new(vec![
                Pauli::X,
                Pauli::X,
            ])
            .expect("valid");

        let zz =
            PauliWord::new(vec![
                Pauli::Z,
                Pauli::Z,
            ])
            .expect("valid");

        assert!(
            xx.commutes_with(&zz)
                .expect("same width")
        );
    }

    #[test]
    fn word_multiplication_tracks_global_phase() {
        let x =
            PauliWord::new(vec![Pauli::X])
                .expect("valid");

        let y =
            PauliWord::new(vec![Pauli::Y])
                .expect("valid");

        let result =
            x.multiply(&y)
                .expect("same width");

        assert_eq!(
            result.get(0),
            Some(Pauli::Z)
        );

        assert_eq!(
            result.phase(),
            PauliPhase::PlusI
        );
    }

    #[test]
    fn word_multiplication_is_non_commutative_when_phase_is_retained() {
        let x =
            PauliWord::new(vec![Pauli::X])
                .expect("valid");

        let y =
            PauliWord::new(vec![Pauli::Y])
                .expect("valid");

        let xy =
            x.multiply(&y)
                .expect("same width");

        let yx =
            y.multiply(&x)
                .expect("same width");

        assert_ne!(
            xy.phase(),
            yx.phase()
        );

        assert_eq!(
            xy.get(0),
            yx.get(0)
        );
    }

    #[test]
    fn inverse_multiplies_to_identity() {
        let word =
            PauliWord::with_phase(
                vec![
                    Pauli::X,
                    Pauli::Y,
                    Pauli::Z,
                ],
                PauliPhase::PlusI,
            )
            .expect("valid");

        let result =
            word.multiply(
                &word.inverse()
            )
            .expect("same width");

        assert!(result.is_identity());
    }

    #[test]
    fn tensor_product_preserves_order() {
        let x =
            PauliWord::new(vec![Pauli::X])
                .expect("valid");

        let y =
            PauliWord::new(vec![Pauli::Y])
                .expect("valid");

        let result =
            x.tensor(&y);

        assert_eq!(
            result.factors(),
            &[Pauli::X, Pauli::Y]
        );

        assert_eq!(
            result.phase(),
            PauliPhase::One
        );
    }

    #[test]
    fn exact_weight_generation_is_exact() {
        let mut rng = stream();

        for weight in 0..=8 {
            let word =
                PauliWord::random_exact_weight(
                    8,
                    weight,
                    &mut rng,
                )
                .expect(
                    "valid exact-weight generation"
                );

            assert_eq!(
                word.weight(),
                weight
            );
        }
    }

    #[test]
    fn guaranteed_non_identity_generation_is_non_identity() {
        let mut rng = stream();

        for _ in 0..128 {
            let word =
                PauliWord::random_non_identity(
                    12,
                    &mut rng,
                )
                .expect(
                    "valid non-identity generation"
                );

            assert!(
                !word.is_identity_up_to_phase()
            );
        }
    }

    #[test]
    fn uniform_non_identity_generation_is_non_identity() {
        let mut rng = stream();

        for _ in 0..128 {
            let word =
                PauliWord::random_uniform_non_identity(
                    12,
                    &mut rng,
                )
                .expect(
                    "valid uniform non-identity generation"
                );

            assert!(
                !word.is_identity_up_to_phase()
            );
        }
    }

    #[test]
    fn deterministic_generation_is_reproducible() {
        let mut left = stream();
        let mut right = stream();

        for _ in 0..32 {
            let first =
                PauliWord::random(
                    16,
                    &mut left,
                )
                .expect("left generation");

            let second =
                PauliWord::random(
                    16,
                    &mut right,
                )
                .expect("right generation");

            assert_eq!(
                first,
                second
            );
        }
    }

    #[test]
    fn deterministic_seed_convenience_is_reproducible() {
        let first =
            PauliWord::random_with_seed(
                10,
                42,
            )
            .expect("first");

        let second =
            PauliWord::random_with_seed(
                10,
                42,
            )
            .expect("second");

        assert_eq!(
            first,
            second
        );
    }

    #[test]
    fn random_single_site_has_exactly_one_non_identity_factor() {
        let mut rng = stream();

        let word =
            PauliWord::random_single_site(
                20,
                &mut rng,
            )
            .expect("valid");

        assert_eq!(
            word.weight(),
            1
        );
    }

    #[test]
    fn random_identity_probability_zero_has_no_identity_factors() {
        let mut rng = stream();

        let word =
            PauliWord::random_with_identity_probability(
                32,
                0.0,
                &mut rng,
            )
            .expect("valid");

        assert_eq!(
            word.weight(),
            32
        );
    }

    #[test]
    fn random_identity_probability_one_is_identity() {
        let mut rng = stream();

        let word =
            PauliWord::random_with_identity_probability(
                32,
                1.0,
                &mut rng,
            )
            .expect("valid");

        assert_eq!(
            word.weight(),
            0
        );
    }

    #[test]
    fn probability_validation_rejects_invalid_values() {
        let mut rng = stream();

        assert!(
            PauliWord::random_with_identity_probability(
                4,
                f64::NAN,
                &mut rng,
            )
            .is_err()
        );

        assert!(
            PauliWord::random_with_identity_probability(
                4,
                f64::INFINITY,
                &mut rng,
            )
            .is_err()
        );

        assert!(
            PauliWord::random_with_identity_probability(
                4,
                -0.1,
                &mut rng,
            )
            .is_err()
        );

        assert!(
            PauliWord::random_with_identity_probability(
                4,
                1.1,
                &mut rng,
            )
            .is_err()
        );
    }

    #[test]
    fn mismatched_widths_are_rejected() {
        let x =
            PauliWord::new(vec![Pauli::X])
                .expect("valid");

        let xx =
            PauliWord::new(vec![
                Pauli::X,
                Pauli::X,
            ])
            .expect("valid");

        assert!(
            x.multiply(&xx).is_err()
        );

        assert!(
            x.commutes_with(&xx).is_err()
        );
    }

    #[test]
    fn parser_round_trip() {
        let word =
            PauliWord::with_phase(
                vec![
                    Pauli::X,
                    Pauli::Y,
                    Pauli::Z,
                ],
                PauliPhase::MinusI,
            )
            .expect("valid");

        let text =
            word.to_string();

        let parsed: PauliWord =
            text.parse()
                .expect(
                    "valid Pauli representation"
                );

        assert_eq!(
            parsed,
            word
        );
    }

    #[test]
    fn parser_rejects_empty_word() {
        let result =
            "".parse::<PauliWord>();

        assert!(result.is_err());
    }

    #[test]
    fn parser_rejects_invalid_factor() {
        let result =
            "A".parse::<PauliWord>();

        assert!(result.is_err());
    }

    #[test]
    fn symplectic_conversion_preserves_factors() {
        let original =
            PauliWord::new(vec![
                Pauli::I,
                Pauli::X,
                Pauli::Y,
                Pauli::Z,
            ])
            .expect("valid");

        let symplectic =
            original.symplectic();

        let recovered =
            symplectic.to_pauli_word();

        assert_eq!(
            recovered,
            original.phase_ignored()
        );
    }

    #[test]
    fn symplectic_commutation_matches_pauli_commutation() {
        let left =
            PauliWord::new(vec![
                Pauli::X,
                Pauli::Y,
                Pauli::I,
            ])
            .expect("valid");

        let right =
            PauliWord::new(vec![
                Pauli::Z,
                Pauli::Y,
                Pauli::X,
            ])
            .expect("valid");

        assert_eq!(
            left.commutes_with(&right)
                .expect("same width"),
            left.symplectic()
                .commutes_with(
                    &right.symplectic()
                )
                .expect("same width")
        );
    }
}