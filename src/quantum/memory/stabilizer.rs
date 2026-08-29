//! Zamani Quantum Memory — Stabilizer / Clifford State Representation.
//!
//! Production-grade, hardware-independent stabilizer-state representation.
//!
//! # Purpose
//!
//! This module implements the stabilizer formalism for efficient simulation
//! and representation of Clifford quantum states and circuits.
//!
//! It is deliberately independent of:
//!
//! - a particular QPU vendor;
//! - CUDA, HIP, Metal, Vulkan, SYCL, or any other accelerator API;
//! - a particular simulator;
//! - Quantum IR gate definitions;
//! - routing;
//! - scheduling;
//! - error-correction decoders;
//! - serialization formats;
//! - benchmarking;
//! - frontend/source-language syntax.
//!
//! Those layers can adapt to this module through its stable public API.
//!
//! # Supported mathematical operations
//!
//! The implementation supports:
//!
//! - |0...0> initialization;
//! - X, Y, Z;
//! - H;
//! - S;
//! - S†;
//! - CNOT;
//! - CZ;
//! - SWAP;
//! - arbitrary Pauli-string expectation values;
//! - single-qubit X/Y/Z measurements;
//! - arbitrary Pauli-string measurements;
//! - deterministic and random measurement;
//! - qubit reset to |0>;
//! - Pauli-frame tracking;
//! - tableau validation;
//! - tableau inspection;
//! - conversion to public Pauli strings.
//!
//! The representation uses the Gottesman/Aaronson stabilizer tableau:
//!
//! ```text
//!              destabilizers       stabilizers
//!             ┌──────────────┐   ┌──────────────┐
//!             │ D0           │   │ S0           │
//!             │ D1           │   │ S1           │
//!             │ ...          │   │ ...          │
//!             │ D(n-1)       │   │ S(n-1)       │
//!             └──────────────┘   └──────────────┘
//! ```
//!
//! Each row is represented by X/Z symplectic bitsets plus a phase.
//!
//! # Complexity
//!
//! Let `n` be the number of logical qubits and `W = ceil(n / 64)`.
//!
//! - tableau storage: O(n² / 64) machine words;
//! - Pauli commutation: O(n / 64);
//! - row multiplication: O(n / 64);
//! - single-qubit Clifford application: O(n);
//! - two-qubit Clifford application: O(n);
//! - measurement: O(n² / 64) worst case due to elimination;
//!
//! This representation is polynomial in `n`, unlike dense state-vector
//! simulation, which requires O(2^n) amplitudes.
//!
//! # Hardware independence
//!
//! This module does NOT claim that every QPU natively executes stabilizer
//! operations. Instead, it provides a canonical mathematical representation
//! that can be used by:
//!
//! - Clifford simulators;
//! - QEC simulators;
//! - syndrome extraction engines;
//! - Pauli-frame engines;
//! - verification engines;
//! - transpiler validation;
//! - hybrid execution systems;
//! - backend adapters that expose Clifford-compatible execution.
//!
//! Hardware adapters may implement `RandomSource` using hardware-provided
//! randomness when appropriate.
//!
//! # Safety
//!
//! - No `unsafe` code.
//! - No raw pointers.
//! - No global mutable state.
//! - No unchecked indexing in public APIs.
//! - Allocation sizes are checked before construction.
//! - Arithmetic uses checked operations where overflow can affect correctness.
//! - Measurement randomness is explicitly injected.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No nightly features are required.
//! No external dependencies are required.
//!
//! # Integration contract
//!
//! Later `quantum::memory` modules should treat this module as the concrete
//! stabilizer representation and should NOT duplicate tableau logic.
//!
//! `state.rs` should adapt this type behind its `QuantumState` abstraction.
//!
//! `measurement.rs` should adapt `MeasurementOutcome` and the measurement
//! methods rather than implementing a second stabilizer measurement engine.
//!
//! QEC should use `PauliString`, `PauliFrame`, `stabilizers()`, and
//! `measure_pauli()` rather than accessing tableau internals.
//!
//! Hardware adapters should use the public operation methods and
//! `RandomSource`; no hardware-specific type belongs in this file.

// =============================================================================
// Constants
// =============================================================================

const WORD_BITS: usize = 64;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by stabilizer-state operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StabilizerError {
    /// The requested number of qubits is invalid for this platform.
    InvalidQubitCount {
        qubits: usize,
    },

    /// The requested qubit is outside the state.
    QubitOutOfRange {
        qubit: usize,
        qubits: usize,
    },

    /// Two qubit operands are identical where distinct operands are required.
    DuplicateQubit {
        qubit: usize,
    },

    /// A Pauli string has a dimension different from the state.
    DimensionMismatch {
        expected: usize,
        actual: usize,
    },

    /// A Pauli string contains an invalid phase.
    InvalidPauliPhase {
        phase: u8,
    },

    /// A Pauli string contains no non-identity operator when one is required.
    EmptyPauliString,

    /// The supplied tableau is structurally invalid.
    InvalidTableau {
        reason: &'static str,
    },

    /// The requested operation is not a Clifford operation supported by this
    /// stabilizer representation.
    UnsupportedOperation {
        operation: &'static str,
    },

    /// A requested operation cannot be represented by the current API.
    InvalidOperation {
        operation: &'static str,
    },

    /// A required arithmetic operation overflowed.
    ArithmeticOverflow,

    /// The supplied random source rejected the requested operation.
    RandomnessUnavailable,
}

impl std::fmt::Display for StabilizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidQubitCount { qubits } => {
                write!(f, "invalid stabilizer qubit count: {qubits}")
            }
            Self::QubitOutOfRange { qubit, qubits } => {
                write!(
                    f,
                    "qubit {qubit} is outside stabilizer state with {qubits} qubits"
                )
            }
            Self::DuplicateQubit { qubit } => {
                write!(f, "control and target qubits must differ: q{qubit}")
            }
            Self::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Pauli dimension mismatch: expected {expected}, got {actual}"
                )
            }
            Self::InvalidPauliPhase { phase } => {
                write!(f, "invalid Pauli phase {phase}; stabilizer observables require 0 or 2")
            }
            Self::EmptyPauliString => {
                write!(f, "Pauli operation requires at least one non-identity qubit")
            }
            Self::InvalidTableau { reason } => {
                write!(f, "invalid stabilizer tableau: {reason}")
            }
            Self::UnsupportedOperation { operation } => {
                write!(
                    f,
                    "operation {operation} is not supported by the stabilizer representation"
                )
            }
            Self::InvalidOperation { operation } => {
                write!(f, "invalid stabilizer operation: {operation}")
            }
            Self::ArithmeticOverflow => {
                write!(f, "stabilizer arithmetic overflow")
            }
            Self::RandomnessUnavailable => {
                write!(f, "measurement randomness is unavailable")
            }
        }
    }
}

impl std::error::Error for StabilizerError {}

/// Result alias for stabilizer operations.
pub type StabilizerResult<T> = Result<T, StabilizerError>;

// =============================================================================
// Randomness
// =============================================================================

/// Source of measurement randomness.
///
/// The stabilizer representation intentionally does not hide a global random
/// number generator. This makes simulation deterministic when desired and
/// allows hardware/runtime layers to inject their own entropy source.
///
/// A QPU adapter may implement this trait using backend-provided randomness,
/// while a simulator can use a seeded deterministic source.
pub trait RandomSource {
    /// Returns the next 64 bits of randomness.
    fn next_u64(&mut self) -> u64;

    /// Returns one uniformly sampled boolean.
    fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) != 0
    }
}

/// Small deterministic non-cryptographic random source.
///
/// This is suitable for reproducible simulation tests and deterministic
/// benchmarking. It is NOT intended as a cryptographic random-number
/// generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    /// Creates a reproducible generator.
    ///
    /// A zero seed is replaced by a fixed non-zero constant because the
    /// xorshift recurrence has zero as an absorbing state.
    pub const fn new(seed: u64) -> Self {
        let state = if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        };

        Self { state }
    }

    /// Returns the current internal state.
    ///
    /// This is useful for checkpoint/restart systems.
    pub const fn state(&self) -> u64 {
        self.state
    }

    /// Restores a generator from a checkpointed state.
    pub const fn from_state(state: u64) -> Self {
        Self::new(state)
    }
}

impl Default for XorShift64 {
    fn default() -> Self {
        Self::new(0xA5A5_A5A5_5A5A_5A5A)
    }
}

impl RandomSource for XorShift64 {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;

        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;

        self.state = x;
        x
    }
}

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
    /// Returns whether this operator is the identity.
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::I)
    }

    /// Returns the symplectic X bit.
    pub const fn x_bit(self) -> bool {
        matches!(self, Self::X | Self::Y)
    }

    /// Returns the symplectic Z bit.
    pub const fn z_bit(self) -> bool {
        matches!(self, Self::Y | Self::Z)
    }
}

// =============================================================================
// Bitset
// =============================================================================

/// Internal fixed-width dynamic bitset.
///
/// This type deliberately remains private. Public callers interact with
/// PauliString rather than implementation-specific storage.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BitSet {
    words: Vec<u64>,
    bits: usize,
}

impl BitSet {
    fn new(bits: usize) -> StabilizerResult<Self> {
        let words = bits
            .checked_add(WORD_BITS - 1)
            .ok_or(StabilizerError::ArithmeticOverflow)?
            / WORD_BITS;

        Ok(Self {
            words: vec![0; words],
            bits,
        })
    }

    fn bits(&self) -> usize {
        self.bits
    }

    fn get(&self, index: usize) -> bool {
        if index >= self.bits {
            return false;
        }

        let word = index / WORD_BITS;
        let bit = index % WORD_BITS;

        ((self.words[word] >> bit) & 1) != 0
    }

    fn set(&mut self, index: usize, value: bool) -> StabilizerResult<()> {
        if index >= self.bits {
            return Err(StabilizerError::QubitOutOfRange {
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

    fn xor_assign(&mut self, other: &Self) {
        for (left, right) in self.words.iter_mut().zip(other.words.iter()) {
            *left ^= *right;
        }
    }

    fn xor_word_bit(&mut self, index: usize) -> StabilizerResult<()> {
        if index >= self.bits {
            return Err(StabilizerError::QubitOutOfRange {
                qubit: index,
                qubits: self.bits,
            });
        }

        let word = index / WORD_BITS;
        let bit = index % WORD_BITS;
        self.words[word] ^= 1u64 << bit;

        Ok(())
    }

    fn parity_and(&self, other: &Self) -> bool {
        self.words
            .iter()
            .zip(other.words.iter())
            .fold(false, |parity, (a, b)| {
                parity ^ ((a & b).count_ones() & 1 != 0)
            })
    }

    fn is_zero(&self) -> bool {
        self.words.iter().all(|word| *word == 0)
    }

    fn highest_set_bit(&self) -> Option<usize> {
        for (word_index, word) in self.words.iter().enumerate().rev() {
            if *word != 0 {
                let bit = WORD_BITS - 1 - word.leading_zeros() as usize;
                let index = word_index * WORD_BITS + bit;

                if index < self.bits {
                    return Some(index);
                }
            }
        }

        None
    }
}

// =============================================================================
// PauliString
// =============================================================================

/// Multi-qubit Pauli observable.
///
/// The operator is represented as:
///
/// ```text
/// phase × P0 ⊗ P1 ⊗ ... ⊗ P(n-1)
/// ```
///
/// The phase is restricted to `+1` (`0`) or `-1` (`2`) because stabilizer
/// observables are Hermitian.
///
/// `PauliString` is intentionally independent from any QPU representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliString {
    x: BitSet,
    z: BitSet,
    phase: u8,
}

impl PauliString {
    /// Creates an all-identity Pauli string.
    pub fn identity(qubits: usize) -> StabilizerResult<Self> {
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
    ) -> StabilizerResult<Self> {
        let mut result = Self::identity(qubits)?;
        result.set(qubit, pauli)?;
        Ok(result)
    }

    /// Returns the number of qubits represented.
    pub fn qubits(&self) -> usize {
        self.x.bits()
    }

    /// Returns the sign as `+1` or `-1`.
    pub const fn sign(&self) -> i8 {
        if self.phase == 0 {
            1
        } else {
            -1
        }
    }

    /// Returns the internal Hermitian phase exponent.
    ///
    /// `0` means +1 and `2` means -1.
    pub const fn phase(&self) -> u8 {
        self.phase
    }

    /// Sets the sign.
    pub fn set_sign(&mut self, sign: i8) -> StabilizerResult<()> {
        match sign {
            1 => {
                self.phase = 0;
                Ok(())
            }
            -1 => {
                self.phase = 2;
                Ok(())
            }
            _ => Err(StabilizerError::InvalidPauliPhase {
                phase: sign as u8,
            }),
        }
    }

    /// Returns the Pauli acting on one qubit.
    pub fn get(&self, qubit: usize) -> StabilizerResult<Pauli> {
        if qubit >= self.qubits() {
            return Err(StabilizerError::QubitOutOfRange {
                qubit,
                qubits: self.qubits(),
            });
        }

        let x = self.x.get(qubit);
        let z = self.z.get(qubit);

        Ok(match (x, z) {
            (false, false) => Pauli::I,
            (true, false) => Pauli::X,
            (true, true) => Pauli::Y,
            (false, true) => Pauli::Z,
        })
    }

    /// Sets the Pauli acting on one qubit.
    pub fn set(&mut self, qubit: usize, pauli: Pauli) -> StabilizerResult<()> {
        if qubit >= self.qubits() {
            return Err(StabilizerError::QubitOutOfRange {
                qubit,
                qubits: self.qubits(),
            });
        }

        self.x.set(qubit, pauli.x_bit())?;
        self.z.set(qubit, pauli.z_bit())?;

        Ok(())
    }

    /// Returns whether the operator is the identity.
    pub fn is_identity(&self) -> bool {
        self.x.is_zero() && self.z.is_zero()
    }

    /// Returns whether this Pauli anticommutes with another Pauli.
    pub fn anticommutes_with(&self, other: &Self) -> StabilizerResult<bool> {
        self.ensure_same_dimension(other)?;

        Ok(self.x.parity_and(&other.z) ^ self.z.parity_and(&other.x))
    }

    /// Returns whether this Pauli commutes with another Pauli.
    pub fn commutes_with(&self, other: &Self) -> StabilizerResult<bool> {
        Ok(!self.anticommutes_with(other)?)
    }

    fn ensure_same_dimension(&self, other: &Self) -> StabilizerResult<()> {
        if self.qubits() != other.qubits() {
            return Err(StabilizerError::DimensionMismatch {
                expected: self.qubits(),
                actual: other.qubits(),
            });
        }

        Ok(())
    }

    fn xor_with_row(&mut self, row: &PauliRow) {
        self.x.xor_assign(&row.x);
        self.z.xor_assign(&row.z);
    }

    fn phase_mul(&mut self, row: &PauliRow) {
        self.phase = (self.phase + row.phase) & 3;
        let extra = if self.z.parity_and(&row.x) {
            2
        } else {
            0
        };
        self.phase = (self.phase + extra) & 3;
    }

    /// Returns the list of non-identity operations.
    pub fn terms(&self) -> Vec<(usize, Pauli)> {
        let mut result = Vec::new();

        for qubit in 0..self.qubits() {
            let pauli = match self.get(qubit) {
                Ok(value) => value,
                Err(_) => continue,
            };

            if !pauli.is_identity() {
                result.push((qubit, pauli));
            }
        }

        result
    }
}

// =============================================================================
// Internal tableau row
// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
struct PauliRow {
    x: BitSet,
    z: BitSet,
    phase: u8,
}

impl PauliRow {
    fn identity(qubits: usize) -> StabilizerResult<Self> {
        Ok(Self {
            x: BitSet::new(qubits)?,
            z: BitSet::new(qubits)?,
            phase: 0,
        })
    }

    fn single(
        qubits: usize,
        qubit: usize,
        pauli: Pauli,
    ) -> StabilizerResult<Self> {
        let mut row = Self::identity(qubits)?;
        row.x.set(qubit, pauli.x_bit())?;
        row.z.set(qubit, pauli.z_bit())?;
        Ok(row)
    }

    fn from_pauli(pauli: &PauliString) -> Self {
        Self {
            x: pauli.x.clone(),
            z: pauli.z.clone(),
            phase: pauli.phase,
        }
    }

    fn to_pauli(&self) -> PauliString {
        PauliString {
            x: self.x.clone(),
            z: self.z.clone(),
            phase: self.phase,
        }
    }

    fn multiply(left: &Self, right: &Self) -> Self {
        let mut result = Self {
            x: left.x.clone(),
            z: left.z.clone(),
            phase: left.phase,
        };

        result.x.xor_assign(&right.x);
        result.z.xor_assign(&right.z);

        let phase = (left.phase + right.phase) & 3;
        let commutation_phase = if left.z.parity_and(&right.x) {
            2
        } else {
            0
        };

        result.phase = (phase + commutation_phase) & 3;
        result
    }

    fn anticommutes_with(&self, other: &Self) -> bool {
        self.x.parity_and(&other.z) ^ self.z.parity_and(&other.x)
    }

    fn is_hermitian(&self) -> bool {
        self.phase == 0 || self.phase == 2
    }
}

// =============================================================================
// Measurement
// =============================================================================

/// Measurement basis for a single qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeasurementBasis {
    /// Computational/Z basis.
    Z,

    /// X basis.
    X,

    /// Y basis.
    Y,
}

/// Result of a stabilizer measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasurementOutcome {
    /// `0` corresponds to eigenvalue +1.
    /// `1` corresponds to eigenvalue -1.
    pub bit: u8,

    /// Eigenvalue of the measured observable.
    pub eigenvalue: i8,

    /// True when the result was determined by the pre-measurement
    /// stabilizer group.
    pub deterministic: bool,
}

impl MeasurementOutcome {
    fn new(bit: u8, deterministic: bool) -> Self {
        Self {
            bit,
            eigenvalue: if bit == 0 { 1 } else { -1 },
            deterministic,
        }
    }
}

// =============================================================================
// Clifford operations
// =============================================================================

/// Clifford operations supported by the stabilizer representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CliffordGate {
    /// Hadamard.
    H {
        qubit: usize,
    },

    /// Phase gate S.
    S {
        qubit: usize,
    },

    /// Inverse phase gate S†.
    Sdg {
        qubit: usize,
    },

    /// Pauli X.
    X {
        qubit: usize,
    },

    /// Pauli Y.
    Y {
        qubit: usize,
    },

    /// Pauli Z.
    Z {
        qubit: usize,
    },

    /// Controlled-NOT.
    Cnot {
        control: usize,
        target: usize,
    },

    /// Controlled-Z.
    Cz {
        control: usize,
        target: usize,
    },

    /// SWAP.
    Swap {
        first: usize,
        second: usize,
    },
}

// =============================================================================
// Pauli frame
// =============================================================================

/// Classical Pauli-frame correction.
///
/// A Pauli frame records corrections logically rather than necessarily
/// applying physical X/Z operations. This is particularly useful for QEC,
/// fault-tolerant execution, and hardware backends that support frame updates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliFrame {
    x: BitSet,
    z: BitSet,
}

impl PauliFrame {
    /// Creates an empty Pauli frame.
    pub fn new(qubits: usize) -> StabilizerResult<Self> {
        Ok(Self {
            x: BitSet::new(qubits)?,
            z: BitSet::new(qubits)?,
        })
    }

    /// Returns the number of tracked qubits.
    pub fn qubits(&self) -> usize {
        self.x.bits()
    }

    /// Returns the correction on a qubit.
    pub fn get(&self, qubit: usize) -> StabilizerResult<Pauli> {
        if qubit >= self.qubits() {
            return Err(StabilizerError::QubitOutOfRange {
                qubit,
                qubits: self.qubits(),
            });
        }

        Ok(match (self.x.get(qubit), self.z.get(qubit)) {
            (false, false) => Pauli::I,
            (true, false) => Pauli::X,
            (true, true) => Pauli::Y,
            (false, true) => Pauli::Z,
        })
    }

    /// Sets the correction on a qubit.
    pub fn set(&mut self, qubit: usize, pauli: Pauli) -> StabilizerResult<()> {
        if qubit >= self.qubits() {
            return Err(StabilizerError::QubitOutOfRange {
                qubit,
                qubits: self.qubits(),
            });
        }

        self.x.set(qubit, pauli.x_bit())?;
        self.z.set(qubit, pauli.z_bit())?;

        Ok(())
    }

    /// Clears the correction on a qubit.
    pub fn clear(&mut self, qubit: usize) -> StabilizerResult<()> {
        self.set(qubit, Pauli::I)
    }

    /// Applies a Pauli-frame correction algebraically.
    pub fn compose(&mut self, other: &Self) -> StabilizerResult<()> {
        if self.qubits() != other.qubits() {
            return Err(StabilizerError::DimensionMismatch {
                expected: self.qubits(),
                actual: other.qubits(),
            });
        }

        self.x.xor_assign(&other.x);
        self.z.xor_assign(&other.z);

        Ok(())
    }

    /// Returns whether the frame is empty.
    pub fn is_identity(&self) -> bool {
        self.x.is_zero() && self.z.is_zero()
    }

    /// Returns a Pauli string representation of this frame.
    pub fn as_pauli_string(&self) -> PauliString {
        PauliString {
            x: self.x.clone(),
            z: self.z.clone(),
            phase: 0,
        }
    }
}

// =============================================================================
// Stabilizer state
// =============================================================================

/// Production stabilizer-state tableau.
///
/// The first `n` rows are destabilizers and the final `n` rows are stabilizer
/// generators.
///
/// The public API deliberately hides raw row mutation so that canonical
/// tableau invariants cannot be violated accidentally.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilizerState {
    qubits: usize,
    rows: Vec<PauliRow>,
}

impl StabilizerState {
    /// Creates the computational-basis state |0...0>.
    ///
    /// The initial stabilizer generators are:
    ///
    /// ```text
    /// Z0, Z1, ..., Z(n-1)
    /// ```
    ///
    /// and destabilizers:
    ///
    /// ```text
    /// X0, X1, ..., X(n-1)
    /// ```
    pub fn new(qubits: usize) -> StabilizerResult<Self> {
        let words = qubits
            .checked_add(WORD_BITS - 1)
            .ok_or(StabilizerError::ArithmeticOverflow)?
            / WORD_BITS;

        let max_rows = qubits
            .checked_mul(2)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        let _ = qubits
            .checked_mul(words)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        let mut rows = Vec::with_capacity(max_rows);

        for qubit in 0..qubits {
            rows.push(PauliRow::single(qubits, qubit, Pauli::X)?);
        }

        for qubit in 0..qubits {
            rows.push(PauliRow::single(qubits, qubit, Pauli::Z)?);
        }

        let result = Self { qubits, rows };

        result.validate()?;

        Ok(result)
    }

    /// Creates a one-qubit |0> stabilizer state.
    pub fn zero() -> StabilizerResult<Self> {
        Self::new(1)
    }

    /// Returns the number of logical qubits.
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    /// Returns the number of tableau rows.
    pub fn tableau_rows(&self) -> usize {
        self.rows.len()
    }

    /// Returns the destabilizer generators.
    pub fn destabilizers(&self) -> Vec<PauliString> {
        self.rows[..self.qubits]
            .iter()
            .map(PauliRow::to_pauli)
            .collect()
    }

    /// Returns the stabilizer generators.
    pub fn stabilizers(&self) -> Vec<PauliString> {
        self.rows[self.qubits..]
            .iter()
            .map(PauliRow::to_pauli)
            .collect()
    }

    /// Returns one stabilizer generator by index.
    pub fn stabilizer(&self, index: usize) -> StabilizerResult<PauliString> {
        if index >= self.qubits {
            return Err(StabilizerError::QubitOutOfRange {
                qubit: index,
                qubits: self.qubits,
            });
        }

        Ok(self.rows[self.qubits + index].to_pauli())
    }

    /// Returns one destabilizer generator by index.
    pub fn destabilizer(&self, index: usize) -> StabilizerResult<PauliString> {
        if index >= self.qubits {
            return Err(StabilizerError::QubitOutOfRange {
                qubit: index,
                qubits: self.qubits,
            });
        }

        Ok(self.rows[index].to_pauli())
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates all stabilizer-tableau invariants.
    ///
    /// The validation checks:
    ///
    /// 1. exactly 2n rows;
    /// 2. every row has n qubits;
    /// 3. every row is Hermitian;
    /// 4. stabilizers mutually commute;
    /// 5. destabilizers mutually commute;
    /// 6. each destabilizer anticommutes with its paired stabilizer;
    /// 7. each destabilizer commutes with all other stabilizers.
    pub fn validate(&self) -> StabilizerResult<()> {
        let expected_rows = self
            .qubits
            .checked_mul(2)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        if self.rows.len() != expected_rows {
            return Err(StabilizerError::InvalidTableau {
                reason: "tableau must contain exactly 2n rows",
            });
        }

        for row in &self.rows {
            if row.x.bits() != self.qubits || row.z.bits() != self.qubits {
                return Err(StabilizerError::InvalidTableau {
                    reason: "row dimension does not match tableau dimension",
                });
            }

            if !row.is_hermitian() {
                return Err(StabilizerError::InvalidTableau {
                    reason: "tableau row is not Hermitian",
                });
            }
        }

        let destabilizers = &self.rows[..self.qubits];
        let stabilizers = &self.rows[self.qubits..];

        for i in 0..self.qubits {
            for j in (i + 1)..self.qubits {
                if destabilizers[i].anticommutes_with(&destabilizers[j]) {
                    return Err(StabilizerError::InvalidTableau {
                        reason: "destabilizers do not mutually commute",
                    });
                }

                if stabilizers[i].anticommutes_with(&stabilizers[j]) {
                    return Err(StabilizerError::InvalidTableau {
                        reason: "stabilizers do not mutually commute",
                    });
                }
            }
        }

        for i in 0..self.qubits {
            for j in 0..self.qubits {
                let anticommutes =
                    destabilizers[i].anticommutes_with(&stabilizers[j]);

                if i == j {
                    if !anticommutes {
                        return Err(StabilizerError::InvalidTableau {
                            reason: "destabilizer/stabilizer pair must anticommute",
                        });
                    }
                } else if anticommutes {
                    return Err(StabilizerError::InvalidTableau {
                        reason: "destabilizer anticommutes with unrelated stabilizer",
                    });
                }
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Clifford operations
    // -------------------------------------------------------------------------

    /// Applies a supported Clifford operation.
    pub fn apply(&mut self, gate: CliffordGate) -> StabilizerResult<()> {
        match gate {
            CliffordGate::H { qubit } => self.h(qubit),
            CliffordGate::S { qubit } => self.s(qubit),
            CliffordGate::Sdg { qubit } => self.sdg(qubit),
            CliffordGate::X { qubit } => self.x(qubit),
            CliffordGate::Y { qubit } => self.y(qubit),
            CliffordGate::Z { qubit } => self.z(qubit),
            CliffordGate::Cnot { control, target } => {
                self.cnot(control, target)
            }
            CliffordGate::Cz { control, target } => {
                self.cz(control, target)
            }
            CliffordGate::Swap { first, second } => {
                self.swap(first, second)
            }
        }
    }

    /// Applies H to one qubit.
    pub fn h(&mut self, qubit: usize) -> StabilizerResult<()> {
        self.validate_qubit(qubit)?;

        let bit = 1u64
            .checked_shl((qubit % WORD_BITS) as u32)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        let word = qubit / WORD_BITS;

        for row in &mut self.rows {
            let x = (row.x.words[word] & bit) != 0;
            let z = (row.z.words[word] & bit) != 0;

            if x && z {
                row.phase = (row.phase + 2) & 3;
            }

            if x != z {
                row.x.words[word] ^= bit;
                row.z.words[word] ^= bit;
            }
        }

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Applies S to one qubit.
    pub fn s(&mut self, qubit: usize) -> StabilizerResult<()> {
        self.validate_qubit(qubit)?;

        let bit = 1u64
            .checked_shl((qubit % WORD_BITS) as u32)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        let word = qubit / WORD_BITS;

        for row in &mut self.rows {
            if (row.x.words[word] & bit) != 0 {
                row.z.words[word] ^= bit;
                row.phase = (row.phase + 1) & 3;
            }
        }

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Applies S† to one qubit.
    pub fn sdg(&mut self, qubit: usize) -> StabilizerResult<()> {
        self.validate_qubit(qubit)?;

        let bit = 1u64
            .checked_shl((qubit % WORD_BITS) as u32)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        let word = qubit / WORD_BITS;

        for row in &mut self.rows {
            if (row.x.words[word] & bit) != 0 {
                row.z.words[word] ^= bit;
                row.phase = (row.phase + 3) & 3;
            }
        }

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Applies Pauli X.
    pub fn x(&mut self, qubit: usize) -> StabilizerResult<()> {
        self.validate_qubit(qubit)?;

        // Conjugation by X changes the sign of Y and Z.
        let bit = 1u64
            .checked_shl((qubit % WORD_BITS) as u32)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        let word = qubit / WORD_BITS;

        for row in &mut self.rows {
            if (row.z.words[word] & bit) != 0 {
                row.phase = (row.phase + 2) & 3;
            }
        }

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Applies Pauli Y.
    pub fn y(&mut self, qubit: usize) -> StabilizerResult<()> {
        self.validate_qubit(qubit)?;

        let bit = 1u64
            .checked_shl((qubit % WORD_BITS) as u32)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        let word = qubit / WORD_BITS;

        for row in &mut self.rows {
            let x = (row.x.words[word] & bit) != 0;
            let z = (row.z.words[word] & bit) != 0;

            if x != z {
                row.phase = (row.phase + 2) & 3;
            }
        }

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Applies Pauli Z.
    pub fn z(&mut self, qubit: usize) -> StabilizerResult<()> {
        self.validate_qubit(qubit)?;

        let bit = 1u64
            .checked_shl((qubit % WORD_BITS) as u32)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        let word = qubit / WORD_BITS;

        for row in &mut self.rows {
            if (row.x.words[word] & bit) != 0 {
                row.phase = (row.phase + 2) & 3;
            }
        }

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Applies CNOT.
    pub fn cnot(
        &mut self,
        control: usize,
        target: usize,
    ) -> StabilizerResult<()> {
        self.validate_distinct_qubits(control, target)?;

        let control_word = control / WORD_BITS;
        let target_word = target / WORD_BITS;

        let control_bit = 1u64
            .checked_shl((control % WORD_BITS) as u32)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        let target_bit = 1u64
            .checked_shl((target % WORD_BITS) as u32)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        for row in &mut self.rows {
            let control_x =
                (row.x.words[control_word] & control_bit) != 0;

            let target_z =
                (row.z.words[target_word] & target_bit) != 0;

            if control_x {
                row.x.words[target_word] ^= target_bit;
            }

            if target_z {
                row.z.words[control_word] ^= control_bit;
            }
        }

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Applies controlled-Z.
    pub fn cz(
        &mut self,
        control: usize,
        target: usize,
    ) -> StabilizerResult<()> {
        self.validate_distinct_qubits(control, target)?;

        let control_word = control / WORD_BITS;
        let target_word = target / WORD_BITS;

        let control_bit = 1u64
            .checked_shl((control % WORD_BITS) as u32)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        let target_bit = 1u64
            .checked_shl((target % WORD_BITS) as u32)
            .ok_or(StabilizerError::ArithmeticOverflow)?;

        for row in &mut self.rows {
            let control_x =
                (row.x.words[control_word] & control_bit) != 0;

            let target_x =
                (row.x.words[target_word] & target_bit) != 0;

            if control_x {
                row.z.words[target_word] ^= target_bit;
            }

            if target_x {
                row.z.words[control_word] ^= control_bit;
            }

            if control_x && target_x {
                row.phase = (row.phase + 2) & 3;
            }
        }

        debug_assert!(self.validate().is_ok());

        Ok(())
    }

    /// Applies SWAP.
    ///
    /// SWAP is decomposed into three CNOT operations. This keeps the
    /// stabilizer implementation mathematically canonical and avoids adding
    /// another independent tableau transformation formula.
    pub fn swap(
        &mut self,
        first: usize,
        second: usize,
    ) -> StabilizerResult<()> {
        self.validate_distinct_qubits(first, second)?;

        self.cnot(first, second)?;
        self.cnot(second, first)?;
        self.cnot(first, second)?;

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Pauli expectation
    // -------------------------------------------------------------------------

    /// Returns the expectation value of a Pauli observable.
    ///
    /// The result is:
    ///
    /// - `+1` if the state is a +1 eigenstate;
    /// - `-1` if the state is a -1 eigenstate;
    /// - `0` if the observable is not in the stabilizer group.
    pub fn expectation(
        &self,
        observable: &PauliString,
    ) -> StabilizerResult<i8> {
        self.ensure_dimension(observable.qubits())?;

        if observable.is_identity() {
            return Ok(observable.sign());
        }

        let (member, sign) = self.stabilizer_membership(observable)?;

        if !member {
            return Ok(0);
        }

        Ok(sign * observable.sign())
    }

    /// Returns whether a Pauli observable belongs to the stabilizer group.
    pub fn contains_stabilizer(
        &self,
        observable: &PauliString,
    ) -> StabilizerResult<bool> {
        self.ensure_dimension(observable.qubits())?;

        let (member, _) = self.stabilizer_membership(observable)?;

        Ok(member)
    }

    /// Determines whether a Pauli is in the stabilizer group and returns the
    /// sign of the corresponding generated operator.
    fn stabilizer_membership(
        &self,
        observable: &PauliString,
    ) -> StabilizerResult<(bool, i8)> {
        let mut basis: Vec<Option<PauliRow>> = vec![None; self.qubits * 2];

        for row in &self.rows[self.qubits..] {
            let mut candidate = row.clone();

            loop {
                let pivot = match highest_vector_bit(&candidate.x, &candidate.z) {
                    Some(value) => value,
                    None => break,
                };

                if let Some(existing) = &basis[pivot] {
                    candidate = PauliRow::multiply(&candidate, existing);
                } else {
                    basis[pivot] = Some(candidate);
                    break;
                }
            }
        }

        let mut target = PauliRow::from_pauli(observable);

        loop {
            let pivot = match highest_vector_bit(&target.x, &target.z) {
                Some(value) => value,
                None => break,
            };

            match &basis[pivot] {
                Some(row) => {
                    target = PauliRow::multiply(&target, row);
                }
                None => {
                    return Ok((false, 0));
                }
            }
        }

        let sign = if target.phase == 0 {
            1
        } else if target.phase == 2 {
            -1
        } else {
            return Err(StabilizerError::InvalidTableau {
                reason: "stabilizer reduction produced a non-Hermitian phase",
            });
        };

        Ok((true, sign))
    }

    // -------------------------------------------------------------------------
    // Measurement
    // -------------------------------------------------------------------------

    /// Measures a single qubit in the specified basis.
    pub fn measure(
        &mut self,
        qubit: usize,
        basis: MeasurementBasis,
        rng: &mut impl RandomSource,
    ) -> StabilizerResult<MeasurementOutcome> {
        self.validate_qubit(qubit)?;

        let observable =
            PauliString::single(self.qubits, qubit, match basis {
                MeasurementBasis::X => Pauli::X,
                MeasurementBasis::Y => Pauli::Y,
                MeasurementBasis::Z => Pauli::Z,
            })?;

        self.measure_pauli(&observable, rng)
    }

    /// Measures a Z-basis qubit.
    pub fn measure_z(
        &mut self,
        qubit: usize,
        rng: &mut impl RandomSource,
    ) -> StabilizerResult<MeasurementOutcome> {
        self.measure(qubit, MeasurementBasis::Z, rng)
    }

    /// Measures an X-basis qubit.
    pub fn measure_x(
        &mut self,
        qubit: usize,
        rng: &mut impl RandomSource,
    ) -> StabilizerResult<MeasurementOutcome> {
        self.measure(qubit, MeasurementBasis::X, rng)
    }

    /// Measures a Y-basis qubit.
    pub fn measure_y(
        &mut self,
        qubit: usize,
        rng: &mut impl RandomSource,
    ) -> StabilizerResult<MeasurementOutcome> {
        self.measure(qubit, MeasurementBasis::Y, rng)
    }

    /// Measures an arbitrary Hermitian Pauli observable.
    ///
    /// If the observable is already determined by the stabilizer group,
    /// measurement is deterministic and the RNG is not consumed.
    ///
    /// If the observable anticommutes with one or more stabilizer generators,
    /// the result is sampled uniformly and the tableau is collapsed to the
    /// selected eigenstate.
    pub fn measure_pauli(
        &mut self,
        observable: &PauliString,
        rng: &mut impl RandomSource,
    ) -> StabilizerResult<MeasurementOutcome> {
        self.ensure_dimension(observable.qubits())?;

        if observable.is_identity() {
            return Err(StabilizerError::EmptyPauliString);
        }

        if observable.phase != 0 && observable.phase != 2 {
            return Err(StabilizerError::InvalidPauliPhase {
                phase: observable.phase,
            });
        }

        let target = PauliRow::from_pauli(observable);

        let random_row = self.find_anticommuting_stabilizer(&target);

        match random_row {
            Some(index) => {
                let old_row = self.rows[index].clone();

                // Before replacing the selected stabilizer, multiply every
                // tableau row that anticommutes with it by the old row.
                //
                // This is the standard stabilizer-measurement update and
                // preserves the destabilizer/stabilizer symplectic pairing.
                for row_index in 0..self.rows.len() {
                    if row_index == index {
                        continue;
                    }

                    if self.rows[row_index].anticommutes_with(&old_row) {
                        let updated =
                            PauliRow::multiply(&self.rows[row_index], &old_row);

                        self.rows[row_index] = updated;
                    }
                }

                let sampled_bit = if rng.next_bool() { 1 } else { 0 };

                let mut measured = target;
                measured.phase =
                    (measured.phase + if sampled_bit == 0 { 0 } else { 2 }) & 3;

                self.rows[index] = measured;

                self.validate()?;

                Ok(MeasurementOutcome::new(sampled_bit, false))
            }

            None => {
                let (member, sign) =
                    self.stabilizer_membership(observable)?;

                if !member {
                    return Err(StabilizerError::InvalidTableau {
                        reason:
                            "measurement observable is neither anticommuting nor a stabilizer",
                    });
                }

                let observable_sign = observable.sign();
                let eigenvalue = sign * observable_sign;

                let bit = if eigenvalue == 1 { 0 } else { 1 };

                Ok(MeasurementOutcome::new(bit, true))
            }
        }
    }

    fn find_anticommuting_stabilizer(
        &self,
        observable: &PauliRow,
    ) -> Option<usize> {
        self.rows[self.qubits..]
            .iter()
            .position(|row| row.anticommutes_with(observable))
            .map(|index| index + self.qubits)
    }

    // -------------------------------------------------------------------------
    // Reset
    // -------------------------------------------------------------------------

    /// Resets one qubit to |0>.
    ///
    /// If the qubit is not already a deterministic +Z eigenstate, reset first
    /// measures Z and applies X when the result is |1>.
    pub fn reset_zero(
        &mut self,
        qubit: usize,
        rng: &mut impl RandomSource,
    ) -> StabilizerResult<()> {
        self.validate_qubit(qubit)?;

        let result = self.measure_z(qubit, rng)?;

        if result.bit == 1 {
            self.x(qubit)?;
        }

        Ok(())
    }

    /// Resets a list of qubits to |0>.
    ///
    /// The operation is deterministic in its ordering: qubits are processed
    /// in the order supplied by the caller.
    pub fn reset_zero_many(
        &mut self,
        qubits: &[usize],
        rng: &mut impl RandomSource,
    ) -> StabilizerResult<()> {
        for &qubit in qubits {
            self.reset_zero(qubit, rng)?;
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Pauli frame
    // -------------------------------------------------------------------------

    /// Creates an empty Pauli frame for this state.
    pub fn pauli_frame(&self) -> StabilizerResult<PauliFrame> {
        PauliFrame::new(self.qubits)
    }

    // -------------------------------------------------------------------------
    // Utilities
    // -------------------------------------------------------------------------

    fn validate_qubit(&self, qubit: usize) -> StabilizerResult<()> {
        if qubit >= self.qubits {
            return Err(StabilizerError::QubitOutOfRange {
                qubit,
                qubits: self.qubits,
            });
        }

        Ok(())
    }

    fn validate_distinct_qubits(
        &self,
        first: usize,
        second: usize,
    ) -> StabilizerResult<()> {
        self.validate_qubit(first)?;
        self.validate_qubit(second)?;

        if first == second {
            return Err(StabilizerError::DuplicateQubit { qubit: first });
        }

        Ok(())
    }

    fn ensure_dimension(&self, actual: usize) -> StabilizerResult<()> {
        if actual != self.qubits {
            return Err(StabilizerError::DimensionMismatch {
                expected: self.qubits,
                actual,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Helpers
// =============================================================================

fn highest_vector_bit(x: &BitSet, z: &BitSet) -> Option<usize> {
    match (x.highest_set_bit(), z.highest_set_bit()) {
        (Some(left), Some(right)) => Some(left.max(right)),
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_state_has_expected_generators() {
        let state = StabilizerState::new(3).expect("state");

        let stabilizers = state.stabilizers();

        assert_eq!(stabilizers.len(), 3);

        assert_eq!(stabilizers[0].get(0).expect("pauli"), Pauli::Z);
        assert_eq!(stabilizers[1].get(1).expect("pauli"), Pauli::Z);
        assert_eq!(stabilizers[2].get(2).expect("pauli"), Pauli::Z);

        state.validate().expect("valid tableau");
    }

    #[test]
    fn hadamard_creates_plus_state() {
        let mut state = StabilizerState::new(1).expect("state");

        state.h(0).expect("H");

        let observable =
            PauliString::single(1, 0, Pauli::X).expect("observable");

        assert_eq!(
            state.expectation(&observable).expect("expectation"),
            1
        );

        let z =
            PauliString::single(1, 0, Pauli::Z).expect("observable");

        assert_eq!(
            state.expectation(&z).expect("expectation"),
            0
        );
    }

    #[test]
    fn bell_state_has_xx_and_zz_stabilizers() {
        let mut state = StabilizerState::new(2).expect("state");

        state.h(0).expect("H");
        state.cnot(0, 1).expect("CNOT");

        let xx = {
            let mut p = PauliString::identity(2).expect("pauli");
            p.set(0, Pauli::X).expect("set");
            p.set(1, Pauli::X).expect("set");
            p
        };

        let zz = {
            let mut p = PauliString::identity(2).expect("pauli");
            p.set(0, Pauli::Z).expect("set");
            p.set(1, Pauli::Z).expect("set");
            p
        };

        assert_eq!(state.expectation(&xx).expect("XX"), 1);
        assert_eq!(state.expectation(&zz).expect("ZZ"), 1);

        state.validate().expect("valid tableau");
    }

    #[test]
    fn bell_z_measurements_are_correlated() {
        let mut state = StabilizerState::new(2).expect("state");

        state.h(0).expect("H");
        state.cnot(0, 1).expect("CNOT");

        let mut rng = XorShift64::new(42);

        let first = state.measure_z(0, &mut rng).expect("measurement");
        let second = state.measure_z(1, &mut rng).expect("measurement");

        assert_eq!(first.bit, second.bit);
    }

    #[test]
    fn deterministic_measurement_does_not_consume_randomness() {
        let mut state = StabilizerState::new(1).expect("state");
        let mut rng = XorShift64::new(123);

        let before = rng.state();

        let result = state.measure_z(0, &mut rng).expect("measurement");

        assert_eq!(result.bit, 0);
        assert!(result.deterministic);
        assert_eq!(before, rng.state());
    }

    #[test]
    fn random_measurement_is_reproducible() {
        let mut first = StabilizerState::new(1).expect("state");
        let mut second = StabilizerState::new(1).expect("state");

        first.h(0).expect("H");
        second.h(0).expect("H");

        let mut rng_a = XorShift64::new(999);
        let mut rng_b = XorShift64::new(999);

        let a = first.measure_z(0, &mut rng_a).expect("measurement");
        let b = second.measure_z(0, &mut rng_b).expect("measurement");

        assert_eq!(a, b);
        assert_eq!(first, second);
    }

    #[test]
    fn reset_zero_produces_zero_eigenstate() {
        let mut state = StabilizerState::new(1).expect("state");

        state.x(0).expect("X");

        let mut rng = XorShift64::new(7);

        state.reset_zero(0, &mut rng).expect("reset");

        let z =
            PauliString::single(1, 0, Pauli::Z).expect("observable");

        assert_eq!(state.expectation(&z).expect("expectation"), 1);
    }

    #[test]
    fn swap_exchanges_single_qubit_information() {
        let mut state = StabilizerState::new(2).expect("state");

        state.x(0).expect("X");
        state.swap(0, 1).expect("SWAP");

        let z0 =
            PauliString::single(2, 0, Pauli::Z).expect("Z0");

        let z1 =
            PauliString::single(2, 1, Pauli::Z).expect("Z1");

        assert_eq!(state.expectation(&z0).expect("Z0"), 1);
        assert_eq!(state.expectation(&z1).expect("Z1"), -1);
    }

    #[test]
    fn pauli_frame_composes() {
        let mut first = PauliFrame::new(2).expect("frame");
        let mut second = PauliFrame::new(2).expect("frame");

        first.set(0, Pauli::X).expect("set");
        second.set(0, Pauli::Z).expect("set");

        first.compose(&second).expect("compose");

        assert_eq!(first.get(0).expect("get"), Pauli::Y);
    }

    #[test]
    fn arbitrary_pauli_measurement_works() {
        let mut state = StabilizerState::new(2).expect("state");

        state.h(0).expect("H");
        state.cnot(0, 1).expect("CNOT");

        let mut zz = PauliString::identity(2).expect("Pauli");
        zz.set(0, Pauli::Z).expect("set");
        zz.set(1, Pauli::Z).expect("set");

        let mut rng = XorShift64::new(1234);

        let result =
            state.measure_pauli(&zz, &mut rng).expect("measurement");

        assert_eq!(result.bit, 0);
        assert!(result.deterministic);
    }

    #[test]
    fn arbitrary_negative_pauli_measurement_flips_result() {
        let mut state = StabilizerState::new(1).expect("state");

        let mut negative_z =
            PauliString::single(1, 0, Pauli::Z).expect("Pauli");

        negative_z.set_sign(-1).expect("sign");

        let mut rng = XorShift64::new(1);

        let result = state
            .measure_pauli(&negative_z, &mut rng)
            .expect("measurement");

        assert_eq!(result.bit, 1);
        assert!(result.deterministic);
    }

    #[test]
    fn clifford_sequence_preserves_tableau_invariants() {
        let mut state = StabilizerState::new(8).expect("state");

        state.h(0).expect("H");
        state.s(1).expect("S");
        state.sdg(2).expect("Sdg");
        state.x(3).expect("X");
        state.y(4).expect("Y");
        state.z(5).expect("Z");
        state.cnot(0, 1).expect("CNOT");
        state.cz(2, 3).expect("CZ");
        state.swap(6, 7).expect("SWAP");

        state.validate().expect("valid tableau");
    }

    #[test]
    fn invalid_two_qubit_operation_is_rejected() {
        let mut state = StabilizerState::new(2).expect("state");

        let error = state.cnot(0, 0).expect_err("must reject");

        assert_eq!(
            error,
            StabilizerError::DuplicateQubit { qubit: 0 }
        );
    }

    #[test]
    fn dimension_mismatch_is_rejected() {
        let state = StabilizerState::new(2).expect("state");
        let observable =
            PauliString::identity(3).expect("observable");

        let error =
            state.expectation(&observable).expect_err("must reject");

        assert_eq!(
            error,
            StabilizerError::DimensionMismatch {
                expected: 2,
                actual: 3,
            }
        );
    }

    #[test]
    fn pauli_commutation_is_correct() {
        let x =
            PauliString::single(1, 0, Pauli::X).expect("X");

        let z =
            PauliString::single(1, 0, Pauli::Z).expect("Z");

        assert!(x.anticommutes_with(&z).expect("commutation"));

        let y =
            PauliString::single(1, 0, Pauli::Y).expect("Y");

        assert!(x.anticommutes_with(&y).expect("commutation"));
        assert!(z.anticommutes_with(&y).expect("commutation"));
    }
}