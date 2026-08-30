//! Zamani Quantum Optimization — Clifford Algebra
//!
//! Production-grade Clifford algebra and tableau infrastructure for the
//! quantum optimizer.
//!
//! # Architectural position
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!             optimization::algebra::clifford
//!                              │
//!             ┌────────────────┼────────────────┐
//!             ▼                ▼                ▼
//!       Clifford analysis   optimization     synthesis
//!             │                │                │
//!             └────────────────┼────────────────┘
//!                              ▼
//!                         verification
//! ```
//!
//! This module owns the mathematical representation of Clifford conjugation.
//!
//! It deliberately does NOT own:
//!
//! - quantum circuit storage;
//! - optimization pass scheduling;
//! - routing;
//! - hardware topology;
//! - pulse generation;
//! - execution;
//! - QPU communication;
//! - error-correction codes;
//! - backend-specific gate costs;
//! - benchmarking orchestration.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Canonical IR rule
//!
//! The canonical quantum representation remains:
//!
//! `crate::quantum::ir`
//!
//! This module never defines another `QuantumGate`, `QuantumOperation`, or
//! circuit representation.
//!
//! # Representation
//!
//! A Pauli operator is represented as
//!
//! `i^phase X^x Z^z`
//!
//! where:
//!
//! - `x` is the X bit vector;
//! - `z` is the Z bit vector;
//! - `phase` is modulo four.
//!
//! A Clifford operator is represented by the conjugation images of the
//! canonical generators:
//!
//! ```text
//! X_0 ... X_(n-1)
//! Z_0 ... Z_(n-1)
//! ```
//!
//! This is a standard symplectic/tableau-style representation.
//!
//! The representation is exact for Clifford conjugation and does not use
//! floating-point matrices.
//!
//! # Global phase
//!
//! A tableau represents conjugation:
//!
//! `U P U†`
//!
//! and therefore does not distinguish Clifford operators that differ only by
//! a global phase.
//!
//! This is intentional. Quantum circuits are normally equivalent under
//! global phase unless a higher-level semantic contract explicitly requires
//! tracking it.
//!
//! # Complexity
//!
//! For `n` logical qubits, a dense tableau requires O(n²) bits for its
//! symplectic information and O(n) Pauli rows.
//!
//! Applying a constant-arity Clifford gate to the tableau is O(n² / W) in
//! the dense representation, where `W` is the machine word width.
//!
//! This module imposes no arbitrary qubit-count ceiling. The practical limit
//! is the available memory and address space.
//!
//! Large-scale future implementations may add sparse or block/tableau
//! representations behind a higher-level abstraction without changing the
//! semantic API established here.
//!
//! # Safety
//!
//! - no `unsafe` code;
//! - no raw pointers;
//! - no unchecked indexing in public operations;
//! - arithmetic that can overflow is checked;
//! - vector allocation is attempted fallibly where practical;
//! - malformed external IR gates are rejected;
//! - qubit indices are validated against the tableau width.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no external dependencies.
//!
//! # Integration contract
//!
//! Future optimizer files should consume this module through:
//!
//! - [`Pauli`];
//! - [`CliffordTableau`];
//! - [`CliffordError`];
//! - [`CliffordGateClass`];
//! - [`CliffordClassification`];
//! - [`CliffordCircuit`];
//!
//! `local`, `passes`, `verification`, `phase_polynomial`, `symplectic`,
//! `synthesis`, and fault-tolerant optimization code must not implement
//! independent Clifford representations.
//!
//! # Important semantic rule
//!
//! Parameterized gates are not classified as Clifford merely because some
//! parameter values might happen to be Clifford. A future exact-angle
//! subsystem may provide value-sensitive classification. Until then, this
//! module is deliberately conservative rather than risking an unsound
//! compiler transformation.

use std::fmt;

use crate::quantum::ir::{Gate, GateKind, QubitId};

// =============================================================================
// Public result and error types
// =============================================================================

/// Result type for Clifford algebra operations.
pub type CliffordResult<T> = Result<T, CliffordError>;

/// Errors produced by Clifford algebra operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliffordError {
    /// The tableau contains zero logical qubits when a non-empty operation
    /// requires at least one.
    InvalidQubitCount {
        /// Supplied number of qubits.
        qubits: usize,
    },

    /// The requested operation references a qubit outside the tableau.
    QubitOutOfRange {
        /// Referenced logical qubit.
        qubit: QubitId,

        /// Number of qubits represented by the tableau.
        qubit_count: usize,
    },

    /// A gate has an unsupported arity.
    InvalidArity {
        /// Gate kind.
        gate: GateKind,

        /// Expected number of operands.
        expected: usize,

        /// Actual number of operands.
        actual: usize,
    },

    /// A gate is not Clifford.
    NotClifford {
        /// Gate kind.
        gate: GateKind,
    },

    /// A parameterized operation cannot be classified exactly by this module.
    ParameterizedGate {
        /// Gate kind.
        gate: GateKind,
    },

    /// A non-unitary operation was supplied to the Clifford algebra.
    NonUnitaryGate {
        /// Gate kind.
        gate: GateKind,
    },

    /// Two Pauli values have different widths.
    PauliWidthMismatch {
        /// Left width.
        left: usize,

        /// Right width.
        right: usize,
    },

    /// A Pauli phase was outside the canonical modulo-four range.
    InvalidPhase {
        /// Supplied phase.
        phase: u8,
    },

    /// A bit-vector allocation failed.
    AllocationFailure {
        /// Logical resource being allocated.
        resource: &'static str,

        /// Requested number of elements.
        requested: usize,
    },

    /// An internal tableau invariant was violated.
    InvalidTableau {
        /// Static description of the invariant failure.
        message: &'static str,
    },

    /// An operation attempted to create an invalid Pauli.
    InvalidPauli {
        /// Static description.
        message: &'static str,
    },
}

impl fmt::Display for CliffordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCount { qubits } => {
                write!(f, "invalid Clifford tableau qubit count: {qubits}")
            }

            Self::QubitOutOfRange {
                qubit,
                qubit_count,
            } => {
                write!(
                    f,
                    "Clifford operation references {qubit}, \
                     but tableau contains {qubit_count} qubits"
                )
            }

            Self::InvalidArity {
                gate,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Clifford gate {gate:?} requires {expected} operands, \
                     received {actual}"
                )
            }

            Self::NotClifford { gate } => {
                write!(f, "gate {gate:?} is not a supported Clifford operation")
            }

            Self::ParameterizedGate { gate } => {
                write!(
                    f,
                    "parameterized gate {gate:?} cannot be classified as \
                     Clifford without exact angle semantics"
                )
            }

            Self::NonUnitaryGate { gate } => {
                write!(
                    f,
                    "non-unitary gate {gate:?} cannot be represented by \
                     Clifford conjugation"
                )
            }

            Self::PauliWidthMismatch { left, right } => {
                write!(
                    f,
                    "Pauli width mismatch: left={left}, right={right}"
                )
            }

            Self::InvalidPhase { phase } => {
                write!(f, "invalid Pauli phase {phase}; expected modulo four")
            }

            Self::AllocationFailure {
                resource,
                requested,
            } => {
                write!(
                    f,
                    "allocation failed for {resource}: requested {requested}"
                )
            }

            Self::InvalidTableau { message } => {
                write!(f, "invalid Clifford tableau: {message}")
            }

            Self::InvalidPauli { message } => {
                write!(f, "invalid Pauli: {message}")
            }
        }
    }
}

impl std::error::Error for CliffordError {}

// =============================================================================
// Clifford classification
// =============================================================================

/// Semantic classification of an IR gate with respect to the Clifford group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CliffordGateClass {
    /// Identity.
    Identity,

    /// One-qubit Clifford.
    SingleQubit,

    /// Two-qubit Clifford.
    TwoQubit,

    /// Three-qubit Clifford.
    ThreeQubit,
}

impl CliffordGateClass {
    /// Returns the expected gate arity.
    #[must_use]
    pub const fn arity(self) -> usize {
        match self {
            Self::Identity => 0,
            Self::SingleQubit => 1,
            Self::TwoQubit => 2,
            Self::ThreeQubit => 3,
        }
    }
}

/// Result of classifying a canonical IR gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CliffordClassification {
    /// Original gate kind.
    pub gate: GateKind,

    /// Clifford class.
    pub class: CliffordGateClass,
}

/// Classifies a canonical IR gate as Clifford or rejects it.
///
/// This classification is intentionally semantic and is independent of the
/// conservative `GateKind::is_clifford()` helper in the IR.
pub fn classify_gate(gate: &Gate) -> CliffordResult<CliffordClassification> {
    let kind = gate.kind();

    if !kind.is_unitary() {
        return Err(CliffordError::NonUnitaryGate { gate: kind });
    }

    if kind.is_parameterized() {
        return Err(CliffordError::ParameterizedGate { gate: kind });
    }

    let class = match kind {
        GateKind::I => CliffordGateClass::Identity,

        GateKind::X
        | GateKind::Y
        | GateKind::Z
        | GateKind::H
        | GateKind::S
        | GateKind::Sdg
        | GateKind::V
        | GateKind::Vdg => CliffordGateClass::SingleQubit,

        GateKind::CX
        | GateKind::CY
        | GateKind::CZ
        | GateKind::CH
        | GateKind::SWAP
        | GateKind::ISWAP => CliffordGateClass::TwoQubit,

        GateKind::CCX | GateKind::CSWAP => {
            // CCX and CSWAP are not Clifford.
            return Err(CliffordError::NotClifford { gate: kind });
        }

        GateKind::ECR => {
            // The current IR deliberately does not classify ECR as Clifford.
            // Do not make an unverified assumption here.
            return Err(CliffordError::NotClifford { gate: kind });
        }

        GateKind::RX
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
        | GateKind::Reset => {
            return Err(CliffordError::NotClifford { gate: kind });
        }
    };

    let actual = gate.qubits().len();

    if class != CliffordGateClass::Identity
        && actual != class.arity()
    {
        return Err(CliffordError::InvalidArity {
            gate: kind,
            expected: class.arity(),
            actual,
        });
    }

    Ok(CliffordClassification {
        gate: kind,
        class,
    })
}

// =============================================================================
// Pauli representation
// =============================================================================

/// Exact Pauli operator.
///
/// The mathematical representation is:
///
/// `i^phase X^x Z^z`
///
/// The bit vectors use little-endian qubit indexing:
///
/// - bit 0 corresponds to logical qubit 0;
/// - bit 1 corresponds to logical qubit 1;
/// - etc.
///
/// The phase is always normalized to `0..=3`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pauli {
    qubit_count: usize,
    x: Vec<u64>,
    z: Vec<u64>,
    phase: u8,
}

impl Pauli {
    /// Creates an identity Pauli on `qubit_count` qubits.
    pub fn identity(qubit_count: usize) -> CliffordResult<Self> {
        Self::zero(qubit_count)
    }

    /// Creates a zero Pauli.
    pub fn zero(qubit_count: usize) -> CliffordResult<Self> {
        let words = words_for_qubits(qubit_count)?;

        let mut x = Vec::new();
        x.try_reserve_exact(words)
            .map_err(|_| CliffordError::AllocationFailure {
                resource: "Pauli X bit vector",
                requested: words,
            })?;
        x.resize(words, 0);

        let mut z = Vec::new();
        z.try_reserve_exact(words)
            .map_err(|_| CliffordError::AllocationFailure {
                resource: "Pauli Z bit vector",
                requested: words,
            })?;
        z.resize(words, 0);

        Ok(Self {
            qubit_count,
            x,
            z,
            phase: 0,
        })
    }

    /// Creates a single-qubit Pauli X.
    pub fn x(qubit_count: usize, qubit: usize) -> CliffordResult<Self> {
        let mut result = Self::zero(qubit_count)?;
        result.set_x(qubit)?;
        Ok(result)
    }

    /// Creates a single-qubit Pauli Y.
    pub fn y(qubit_count: usize, qubit: usize) -> CliffordResult<Self> {
        let mut result = Self::zero(qubit_count)?;
        result.set_x(qubit)?;
        result.set_z(qubit)?;
        result.phase = 1;
        Ok(result)
    }

    /// Creates a single-qubit Pauli Z.
    pub fn z(qubit_count: usize, qubit: usize) -> CliffordResult<Self> {
        let mut result = Self::zero(qubit_count)?;
        result.set_z(qubit)?;
        Ok(result)
    }

    /// Returns the number of represented qubits.
    #[must_use]
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Returns the phase modulo four.
    #[must_use]
    pub const fn phase(&self) -> u8 {
        self.phase
    }

    /// Returns whether this Pauli is the identity.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.x.iter().all(|word| *word == 0)
            && self.z.iter().all(|word| *word == 0)
            && self.phase == 0
    }

    /// Returns whether this Pauli has no X/Z support, ignoring phase.
    #[must_use]
    pub fn has_identity_support(&self) -> bool {
        self.x.iter().all(|word| *word == 0)
            && self.z.iter().all(|word| *word == 0)
    }

    /// Returns whether the Pauli is Hermitian.
    ///
    /// A physical Pauli observable must have phase 0 or 2 in this
    /// representation when its support is non-empty.
    #[must_use]
    pub fn is_hermitian(&self) -> bool {
        self.phase % 2 == 0
    }

    /// Returns whether the operator contains X support on `qubit`.
    #[must_use]
    pub fn has_x(&self, qubit: usize) -> bool {
        self.get_bit(&self.x, qubit)
    }

    /// Returns whether the operator contains Z support on `qubit`.
    #[must_use]
    pub fn has_z(&self, qubit: usize) -> bool {
        self.get_bit(&self.z, qubit)
    }

    /// Returns the X bit vector as machine words.
    #[must_use]
    pub fn x_words(&self) -> &[u64] {
        &self.x
    }

    /// Returns the Z bit vector as machine words.
    #[must_use]
    pub fn z_words(&self) -> &[u64] {
        &self.z
    }

    /// Sets X support.
    pub fn set_x(&mut self, qubit: usize) -> CliffordResult<()> {
        Self::validate_qubit(qubit, self.qubit_count)?;
        set_bit(&mut self.x, qubit);
        Ok(())
    }

    /// Sets Z support.
    pub fn set_z(&mut self, qubit: usize) -> CliffordResult<()> {
        Self::validate_qubit(qubit, self.qubit_count)?;
        set_bit(&mut self.z, qubit);
        Ok(())
    }

    /// Clears X support.
    pub fn clear_x(&mut self, qubit: usize) -> CliffordResult<()> {
        Self::validate_qubit(qubit, self.qubit_count)?;
        clear_bit(&mut self.x, qubit);
        Ok(())
    }

    /// Clears Z support.
    pub fn clear_z(&mut self, qubit: usize) -> CliffordResult<()> {
        Self::validate_qubit(qubit, self.qubit_count)?;
        clear_bit(&mut self.z, qubit);
        Ok(())
    }

    /// Changes the phase modulo four.
    pub fn set_phase(&mut self, phase: u8) -> CliffordResult<()> {
        if phase > 3 {
            return Err(CliffordError::InvalidPhase { phase });
        }

        self.phase = phase;
        Ok(())
    }

    /// Adds a phase modulo four.
    pub fn add_phase(&mut self, phase: u8) {
        self.phase = (self.phase + phase) & 3;
    }

    /// Multiplies this Pauli by another Pauli.
    ///
    /// If:
    ///
    /// `P = i^a X^x Z^z`
    ///
    /// and:
    ///
    /// `Q = i^b X^u Z^v`
    ///
    /// then:
    ///
    /// `PQ = i^(a+b+2 z·u) X^(x⊕u) Z^(z⊕v)`.
    pub fn multiply(&self, other: &Self) -> CliffordResult<Self> {
        if self.qubit_count != other.qubit_count {
            return Err(CliffordError::PauliWidthMismatch {
                left: self.qubit_count,
                right: other.qubit_count,
            });
        }

        let mut result = self.clone();
        result.multiply_assign(other)?;
        Ok(result)
    }

    /// Multiplies this Pauli by another Pauli in place.
    pub fn multiply_assign(&mut self, other: &Self) -> CliffordResult<()> {
        if self.qubit_count != other.qubit_count {
            return Err(CliffordError::PauliWidthMismatch {
                left: self.qubit_count,
                right: other.qubit_count,
            });
        }

        let parity = dot_parity(&self.z, &other.x);

        self.phase = (self.phase + other.phase + if parity { 2 } else { 0 }) & 3;

        for index in 0..self.x.len() {
            self.x[index] ^= other.x[index];
            self.z[index] ^= other.z[index];
        }

        Ok(())
    }

    /// Returns the commutation phase between two Paulis.
    ///
    /// `0` means commuting and `2` means anticommuting in modulo-four phase
    /// notation.
    pub fn commutation_phase(&self, other: &Self) -> CliffordResult<u8> {
        if self.qubit_count != other.qubit_count {
            return Err(CliffordError::PauliWidthMismatch {
                left: self.qubit_count,
                right: other.qubit_count,
            });
        }

        let left = dot_parity(&self.x, &other.z);
        let right = dot_parity(&self.z, &other.x);

        Ok(if left ^ right { 2 } else { 0 })
    }

    /// Returns true when the two Paulis commute.
    pub fn commutes_with(&self, other: &Self) -> CliffordResult<bool> {
        Ok(self.commutation_phase(other)? == 0)
    }

    /// Returns the support qubits in ascending order.
    pub fn support(&self) -> Vec<usize> {
        let mut result = Vec::new();

        for qubit in 0..self.qubit_count {
            if self.has_x(qubit) || self.has_z(qubit) {
                result.push(qubit);
            }
        }

        result
    }

    /// Returns the single-qubit Pauli character at `qubit`.
    ///
    /// The return value is:
    ///
    /// - `I` for no support;
    /// - `X` for X only;
    /// - `Z` for Z only;
    /// - `Y` for both X and Z.
    #[must_use]
    pub fn character_at(&self, qubit: usize) -> char {
        let x = self.has_x(qubit);
        let z = self.has_z(qubit);

        match (x, z) {
            (false, false) => 'I',
            (true, false) => 'X',
            (false, true) => 'Z',
            (true, true) => 'Y',
        }
    }

    fn validate_qubit(qubit: usize, qubit_count: usize) -> CliffordResult<()> {
        if qubit >= qubit_count {
            return Err(CliffordError::QubitOutOfRange {
                qubit: QubitId::new(qubit),
                qubit_count,
            });
        }

        Ok(())
    }

    fn get_bit(&self, words: &[u64], qubit: usize) -> bool {
        if qubit >= self.qubit_count {
            return false;
        }

        let word = qubit / 64;
        let bit = qubit % 64;

        ((words[word] >> bit) & 1) != 0
    }
}

impl fmt::Display for Pauli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let phase = match self.phase {
            0 => "",
            1 => "i",
            2 => "-",
            3 => "-i",
            _ => unreachable!(),
        };

        write!(f, "{phase}")?;

        for qubit in 0..self.qubit_count {
            write!(f, "{}", self.character_at(qubit))?;
        }

        Ok(())
    }
}

// =============================================================================
// Clifford tableau
// =============================================================================

/// Exact Clifford conjugation tableau.
///
/// The tableau stores:
///
/// ```text
/// image(X_0), ..., image(X_(n-1)),
/// image(Z_0), ..., image(Z_(n-1))
/// ```
///
/// Each image is an exact [`Pauli`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliffordTableau {
    qubit_count: usize,
    x_images: Vec<Pauli>,
    z_images: Vec<Pauli>,
}

impl CliffordTableau {
    /// Creates the identity Clifford on `qubit_count` qubits.
    pub fn identity(qubit_count: usize) -> CliffordResult<Self> {
        let mut x_images = Vec::new();
        x_images
            .try_reserve_exact(qubit_count)
            .map_err(|_| CliffordError::AllocationFailure {
                resource: "Clifford X image table",
                requested: qubit_count,
            })?;

        let mut z_images = Vec::new();
        z_images
            .try_reserve_exact(qubit_count)
            .map_err(|_| CliffordError::AllocationFailure {
                resource: "Clifford Z image table",
                requested: qubit_count,
            })?;

        for qubit in 0..qubit_count {
            x_images.push(Pauli::x(qubit_count, qubit)?);
            z_images.push(Pauli::z(qubit_count, qubit)?);
        }

        Ok(Self {
            qubit_count,
            x_images,
            z_images,
        })
    }

    /// Creates a Clifford tableau by applying an IR gate sequence.
    pub fn from_gates<'a, I>(
        qubit_count: usize,
        gates: I,
    ) -> CliffordResult<Self>
    where
        I: IntoIterator<Item = &'a Gate>,
    {
        let mut tableau = Self::identity(qubit_count)?;

        for gate in gates {
            tableau.apply_gate(gate)?;
        }

        Ok(tableau)
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Returns the image of `X_qubit`.
    pub fn x_image(&self, qubit: usize) -> CliffordResult<&Pauli> {
        if qubit >= self.qubit_count {
            return Err(CliffordError::QubitOutOfRange {
                qubit: QubitId::new(qubit),
                qubit_count: self.qubit_count,
            });
        }

        Ok(&self.x_images[qubit])
    }

    /// Returns the image of `Z_qubit`.
    pub fn z_image(&self, qubit: usize) -> CliffordResult<&Pauli> {
        if qubit >= self.qubit_count {
            return Err(CliffordError::QubitOutOfRange {
                qubit: QubitId::new(qubit),
                qubit_count: self.qubit_count,
            });
        }

        Ok(&self.z_images[qubit])
    }

    /// Returns the image of an arbitrary Pauli under Clifford conjugation.
    pub fn conjugate_pauli(&self, pauli: &Pauli) -> CliffordResult<Pauli> {
        if pauli.qubit_count != self.qubit_count {
            return Err(CliffordError::PauliWidthMismatch {
                left: self.qubit_count,
                right: pauli.qubit_count,
            });
        }

        let mut result = Pauli::identity(self.qubit_count)?;
        result.phase = pauli.phase;

        for qubit in set_bits(&pauli.x) {
            result.multiply_assign(&self.x_images[qubit])?;
        }

        for qubit in set_bits(&pauli.z) {
            result.multiply_assign(&self.z_images[qubit])?;
        }

        Ok(result)
    }

    /// Applies a canonical IR Clifford gate.
    pub fn apply_gate(&mut self, gate: &Gate) -> CliffordResult<()> {
        let classification = classify_gate(gate)?;

        if classification.class == CliffordGateClass::Identity {
            return Ok(());
        }

        let qubits = gate.qubits();

        for &qubit in qubits {
            self.validate_qubit(qubit.index())?;
        }

        let kind = gate.kind();

        for image in &mut self.x_images {
            let transformed = transform_pauli_by_gate(
                image,
                kind,
                qubits,
            )?;
            *image = transformed;
        }

        for image in &mut self.z_images {
            let transformed = transform_pauli_by_gate(
                image,
                kind,
                qubits,
            )?;
            *image = transformed;
        }

        Ok(())
    }

    /// Applies a sequence of canonical IR Clifford gates.
    pub fn apply_gates<'a, I>(
        &mut self,
        gates: I,
    ) -> CliffordResult<()>
    where
        I: IntoIterator<Item = &'a Gate>,
    {
        for gate in gates {
            self.apply_gate(gate)?;
        }

        Ok(())
    }

    /// Returns true if this tableau is the identity Clifford.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        for qubit in 0..self.qubit_count {
            if self.x_images[qubit]
                != Pauli::x(self.qubit_count, qubit)
                    .expect("valid tableau width")
            {
                return false;
            }

            if self.z_images[qubit]
                != Pauli::z(self.qubit_count, qubit)
                    .expect("valid tableau width")
            {
                return false;
            }
        }

        true
    }

    /// Compares two Cliffords up to global phase.
    ///
    /// Global phase is intentionally invisible to conjugation.
    pub fn equivalent_up_to_global_phase(
        &self,
        other: &Self,
    ) -> CliffordResult<bool> {
        if self.qubit_count != other.qubit_count {
            return Ok(false);
        }

        Ok(self == other)
    }

    /// Validates the tableau's symplectic commutation invariants.
    ///
    /// For every pair of canonical generators, the images must preserve their
    /// original commutation relation.
    pub fn validate(&self) -> CliffordResult<()> {
        if self.x_images.len() != self.qubit_count {
            return Err(CliffordError::InvalidTableau {
                message: "X image count does not match qubit count",
            });
        }

        if self.z_images.len() != self.qubit_count {
            return Err(CliffordError::InvalidTableau {
                message: "Z image count does not match qubit count",
            });
        }

        for qubit in 0..self.qubit_count {
            if self.x_images[qubit].qubit_count != self.qubit_count
                || self.z_images[qubit].qubit_count != self.qubit_count
            {
                return Err(CliffordError::InvalidTableau {
                    message: "Pauli image width does not match tableau width",
                });
            }
        }

        for left in 0..self.qubit_count {
            for right in 0..self.qubit_count {
                let xx = self.x_images[left]
                    .commutes_with(&self.x_images[right])?;

                let zz = self.z_images[left]
                    .commutes_with(&self.z_images[right])?;

                let xz = self.x_images[left]
                    .commutes_with(&self.z_images[right])?;

                let expected_xx = true;
                let expected_zz = true;
                let expected_xz = left != right;

                if xx != expected_xx
                    || zz != expected_zz
                    || xz != expected_xz
                {
                    return Err(CliffordError::InvalidTableau {
                        message: "symplectic commutation invariant violated",
                    });
                }

                let zx = self.z_images[left]
                    .commutes_with(&self.x_images[right])?;

                let expected_zx = left != right;

                if zx != expected_zx {
                    return Err(CliffordError::InvalidTableau {
                        message: "X/Z symplectic commutation invariant violated",
                    });
                }
            }
        }

        Ok(())
    }

    fn validate_qubit(&self, qubit: usize) -> CliffordResult<()> {
        if qubit >= self.qubit_count {
            return Err(CliffordError::QubitOutOfRange {
                qubit: QubitId::new(qubit),
                qubit_count: self.qubit_count,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Circuit-level Clifford wrapper
// =============================================================================

/// A validated Clifford circuit view.
///
/// This type does not own or replace the canonical IR circuit. It is an
/// algebraic view over a sequence of already existing IR gates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliffordCircuit {
    tableau: CliffordTableau,
    gate_count: usize,
}

impl CliffordCircuit {
    /// Creates an empty Clifford circuit view.
    pub fn new(qubit_count: usize) -> CliffordResult<Self> {
        Ok(Self {
            tableau: CliffordTableau::identity(qubit_count)?,
            gate_count: 0,
        })
    }

    /// Creates a Clifford view from canonical IR gates.
    pub fn from_gates<'a, I>(
        qubit_count: usize,
        gates: I,
    ) -> CliffordResult<Self>
    where
        I: IntoIterator<Item = &'a Gate>,
    {
        let mut result = Self::new(qubit_count)?;

        for gate in gates {
            result.apply_gate(gate)?;
        }

        Ok(result)
    }

    /// Applies one canonical IR Clifford gate.
    pub fn apply_gate(&mut self, gate: &Gate) -> CliffordResult<()> {
        self.tableau.apply_gate(gate)?;

        self.gate_count = self
            .gate_count
            .checked_add(1)
            .ok_or(CliffordError::InvalidTableau {
                message: "Clifford gate count overflowed usize",
            })?;

        Ok(())
    }

    /// Returns the underlying tableau.
    #[must_use]
    pub const fn tableau(&self) -> &CliffordTableau {
        &self.tableau
    }

    /// Returns the number of gates represented by this view.
    #[must_use]
    pub const fn gate_count(&self) -> usize {
        self.gate_count
    }

    /// Returns whether the complete Clifford circuit is semantically identity
    /// up to global phase.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.tableau.is_identity()
    }

    /// Validates the underlying tableau.
    pub fn validate(&self) -> CliffordResult<()> {
        self.tableau.validate()
    }
}

// =============================================================================
// Internal transformation engine
// =============================================================================

/// A primitive Clifford operation used internally to express compound
/// Clifford gates.
///
/// This is NOT a second public quantum IR. It exists only as an internal
/// implementation detail for exact conjugation formulas.
#[derive(Debug, Clone, Copy)]
enum Primitive {
    I,
    X(usize),
    Y(usize),
    Z(usize),
    H(usize),
    S(usize),
    Sdg(usize),
    CX(usize, usize),
    CZ(usize, usize),
    SWAP(usize, usize),
}

/// Applies a canonical gate's conjugation action to one Pauli.
///
/// The implementation is expressed in terms of exact generator images rather
/// than floating-point matrices.
fn transform_pauli_by_gate(
    pauli: &Pauli,
    kind: GateKind,
    qubits: &[QubitId],
) -> CliffordResult<Pauli> {
    let indices: Vec<usize> =
        qubits.iter().map(|qubit| qubit.index()).collect();

    match kind {
        GateKind::I => Ok(pauli.clone()),

        GateKind::X => {
            transform_by_primitive(pauli, Primitive::X(indices[0]))
        }

        GateKind::Y => {
            transform_by_primitive(pauli, Primitive::Y(indices[0]))
        }

        GateKind::Z => {
            transform_by_primitive(pauli, Primitive::Z(indices[0]))
        }

        GateKind::H => {
            transform_by_primitive(pauli, Primitive::H(indices[0]))
        }

        GateKind::S => {
            transform_by_primitive(pauli, Primitive::S(indices[0]))
        }

        GateKind::Sdg => {
            transform_by_primitive(pauli, Primitive::Sdg(indices[0]))
        }

        GateKind::V => {
            // V is Clifford-equivalent to H S H up to global phase.
            transform_sequence(
                pauli,
                &[
                    Primitive::H(indices[0]),
                    Primitive::S(indices[0]),
                    Primitive::H(indices[0]),
                ],
            )
        }

        GateKind::Vdg => {
            // V† is Clifford-equivalent to H S† H up to global phase.
            transform_sequence(
                pauli,
                &[
                    Primitive::H(indices[0]),
                    Primitive::Sdg(indices[0]),
                    Primitive::H(indices[0]),
                ],
            )
        }

        GateKind::CX => {
            transform_by_primitive(
                pauli,
                Primitive::CX(indices[0], indices[1]),
            )
        }

        GateKind::CZ => {
            transform_by_primitive(
                pauli,
                Primitive::CZ(indices[0], indices[1]),
            )
        }

        GateKind::SWAP => {
            transform_by_primitive(
                pauli,
                Primitive::SWAP(indices[0], indices[1]),
            )
        }

        GateKind::CY => {
            // CY = (I ⊗ S) CX (I ⊗ S†).
            //
            // For conjugation, operators are encountered from right to left.
            transform_sequence(
                pauli,
                &[
                    Primitive::Sdg(indices[1]),
                    Primitive::CX(indices[0], indices[1]),
                    Primitive::S(indices[1]),
                ],
            )
        }

        GateKind::CH => {
            // CH = (I ⊗ H) CZ (I ⊗ H).
            transform_sequence(
                pauli,
                &[
                    Primitive::H(indices[1]),
                    Primitive::CZ(indices[0], indices[1]),
                    Primitive::H(indices[1]),
                ],
            )
        }

        GateKind::ISWAP => {
            // iSWAP is Clifford. Up to global phase it can be expressed as:
            //
            // (S ⊗ S) SWAP CZ
            //
            // The global phase is irrelevant to conjugation.
            transform_sequence(
                pauli,
                &[
                    Primitive::CZ(indices[0], indices[1]),
                    Primitive::SWAP(indices[0], indices[1]),
                    Primitive::S(indices[0]),
                    Primitive::S(indices[1]),
                ],
            )
        }

        GateKind::CCX
        | GateKind::CSWAP
        | GateKind::ECR
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
        | GateKind::Reset => Err(CliffordError::NotClifford { gate: kind }),
    }
}

/// Applies a sequence of primitive Clifford conjugations.
fn transform_sequence(
    pauli: &Pauli,
    sequence: &[Primitive],
) -> CliffordResult<Pauli> {
    let mut result = pauli.clone();

    for primitive in sequence {
        result = transform_by_primitive(&result, *primitive)?;
    }

    Ok(result)
}

/// Applies one primitive Clifford conjugation.
fn transform_by_primitive(
    pauli: &Pauli,
    primitive: Primitive,
) -> CliffordResult<Pauli> {
    match primitive {
        Primitive::I => Ok(pauli.clone()),

        Primitive::X(qubit) => {
            let x_image = Pauli::x(pauli.qubit_count, qubit)?;
            let mut z_image =
                Pauli::z(pauli.qubit_count, qubit)?;
            z_image.phase = 2;

            transform_using_generators(
                pauli,
                qubit,
                x_image,
                z_image,
            )
        }

        Primitive::Y(qubit) => {
            let mut x_image =
                Pauli::x(pauli.qubit_count, qubit)?;
            let mut z_image =
                Pauli::z(pauli.qubit_count, qubit)?;

            x_image.phase = 2;
            z_image.phase = 2;

            transform_using_generators(
                pauli,
                qubit,
                x_image,
                z_image,
            )
        }

        Primitive::Z(qubit) => {
            let mut x_image =
                Pauli::x(pauli.qubit_count, qubit)?;
            let z_image =
                Pauli::z(pauli.qubit_count, qubit)?;

            x_image.phase = 2;

            transform_using_generators(
                pauli,
                qubit,
                x_image,
                z_image,
            )
        }

        Primitive::H(qubit) => {
            let x_image =
                Pauli::z(pauli.qubit_count, qubit)?;
            let z_image =
                Pauli::x(pauli.qubit_count, qubit)?;

            transform_using_generators(
                pauli,
                qubit,
                x_image,
                z_image,
            )
        }

        Primitive::S(qubit) => {
            let x_image =
                Pauli::y(pauli.qubit_count, qubit)?;
            let z_image =
                Pauli::z(pauli.qubit_count, qubit)?;

            transform_using_generators(
                pauli,
                qubit,
                x_image,
                z_image,
            )
        }

        Primitive::Sdg(qubit) => {
            let mut x_image =
                Pauli::y(pauli.qubit_count, qubit)?;
            let z_image =
                Pauli::z(pauli.qubit_count, qubit)?;

            x_image.phase = 3;

            transform_using_generators(
                pauli,
                qubit,
                x_image,
                z_image,
            )
        }

        Primitive::CX(control, target) => {
            transform_two_qubit_generators(
                pauli,
                control,
                target,
                TwoQubitMap::CX,
            )
        }

        Primitive::CZ(control, target) => {
            transform_two_qubit_generators(
                pauli,
                control,
                target,
                TwoQubitMap::CZ,
            )
        }

        Primitive::SWAP(left, right) => {
            transform_two_qubit_generators(
                pauli,
                left,
                right,
                TwoQubitMap::SWAP,
            )
        }
    }
}

/// Applies a single-qubit generator transformation.
fn transform_using_generators(
    pauli: &Pauli,
    qubit: usize,
    x_image: Pauli,
    z_image: Pauli,
) -> CliffordResult<Pauli> {
    if qubit >= pauli.qubit_count {
        return Err(CliffordError::QubitOutOfRange {
            qubit: QubitId::new(qubit),
            qubit_count: pauli.qubit_count,
        });
    }

    let mut result = Pauli::identity(pauli.qubit_count)?;
    result.phase = pauli.phase;

    // Every unaffected generator is mapped to itself.
    //
    // We only need to replace the X_q and Z_q factors.
    for index in set_bits(&pauli.x) {
        if index == qubit {
            result.multiply_assign(&x_image)?;
        } else {
            result.multiply_assign(
                &Pauli::x(pauli.qubit_count, index)?,
            )?;
        }
    }

    for index in set_bits(&pauli.z) {
        if index == qubit {
            result.multiply_assign(&z_image)?;
        } else {
            result.multiply_assign(
                &Pauli::z(pauli.qubit_count, index)?,
            )?;
        }
    }

    Ok(result)
}

// =============================================================================
// Two-qubit generator mappings
// =============================================================================

#[derive(Debug, Clone, Copy)]
enum TwoQubitMap {
    CX,
    CZ,
    SWAP,
}

fn transform_two_qubit_generators(
    pauli: &Pauli,
    first: usize,
    second: usize,
    map: TwoQubitMap,
) -> CliffordResult<Pauli> {
    if first >= pauli.qubit_count {
        return Err(CliffordError::QubitOutOfRange {
            qubit: QubitId::new(first),
            qubit_count: pauli.qubit_count,
        });
    }

    if second >= pauli.qubit_count {
        return Err(CliffordError::QubitOutOfRange {
            qubit: QubitId::new(second),
            qubit_count: pauli.qubit_count,
        });
    }

    if first == second {
        return Err(CliffordError::InvalidTableau {
            message: "two-qubit Clifford operation requires distinct qubits",
        });
    }

    let mut result = Pauli::identity(pauli.qubit_count)?;
    result.phase = pauli.phase;

    for index in set_bits(&pauli.x) {
        let generator =
            two_qubit_x_image(
                pauli.qubit_count,
                index,
                first,
                second,
                map,
            )?;

        result.multiply_assign(&generator)?;
    }

    for index in set_bits(&pauli.z) {
        let generator =
            two_qubit_z_image(
                pauli.qubit_count,
                index,
                first,
                second,
                map,
            )?;

        result.multiply_assign(&generator)?;
    }

    Ok(result)
}

fn two_qubit_x_image(
    qubit_count: usize,
    index: usize,
    first: usize,
    second: usize,
    map: TwoQubitMap,
) -> CliffordResult<Pauli> {
    let mut result =
        Pauli::x(qubit_count, index)?;

    match map {
        TwoQubitMap::CX => {
            if index == first {
                result.set_x(second)?;
            }
        }

        TwoQubitMap::CZ => {
            if index == first {
                result.set_z(second)?;
            } else if index == second {
                result.set_z(first)?;
            }
        }

        TwoQubitMap::SWAP => {
            if index == first || index == second {
                result.clear_x(index)?;

                let replacement =
                    if index == first {
                        second
                    } else {
                        first
                    };

                result.set_x(replacement)?;
            }
        }
    }

    Ok(result)
}

fn two_qubit_z_image(
    qubit_count: usize,
    index: usize,
    first: usize,
    second: usize,
    map: TwoQubitMap,
) -> CliffordResult<Pauli> {
    let mut result =
        Pauli::z(qubit_count, index)?;

    match map {
        TwoQubitMap::CX => {
            if index == second {
                result.set_z(first)?;
            }
        }

        TwoQubitMap::CZ => {
            if index == first {
                // Z_first -> Z_first.
            } else if index == second {
                // Z_second -> Z_second.
            }
        }

        TwoQubitMap::SWAP => {
            if index == first || index == second {
                result.clear_z(index)?;

                let replacement =
                    if index == first {
                        second
                    } else {
                        first
                    };

                result.set_z(replacement)?;
            }
        }
    }

    Ok(result)
}

// =============================================================================
// Utility functions
// =============================================================================

fn words_for_qubits(qubit_count: usize) -> CliffordResult<usize> {
    if qubit_count == 0 {
        return Ok(0);
    }

    qubit_count
        .checked_add(63)
        .map(|value| value / 64)
        .ok_or(CliffordError::InvalidQubitCount {
            qubits: qubit_count,
        })
}

fn set_bit(words: &mut [u64], bit: usize) {
    let word = bit / 64;
    let offset = bit % 64;

    words[word] |= 1u64 << offset;
}

fn clear_bit(words: &mut [u64], bit: usize) {
    let word = bit / 64;
    let offset = bit % 64;

    words[word] &= !(1u64 << offset);
}

fn dot_parity(left: &[u64], right: &[u64]) -> bool {
    let mut parity = false;

    for (a, b) in left.iter().zip(right.iter()) {
        parity ^= (a & b).count_ones() % 2 != 0;
    }

    parity
}

fn set_bits(words: &[u64]) -> Vec<usize> {
    let mut result = Vec::new();

    for (word_index, word) in words.iter().copied().enumerate() {
        let mut value = word;

        while value != 0 {
            let bit = value.trailing_zeros() as usize;

            result.push(
                word_index
                    .saturating_mul(64)
                    .saturating_add(bit),
            );

            value &= value - 1;
        }
    }

    result
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::Gate;

    fn gate(kind: GateKind, qubits: &[usize]) -> Gate {
        Gate::new(
            kind,
            qubits
                .iter()
                .copied()
                .map(QubitId::new)
                .collect(),
            Vec::new(),
            None,
            None,
        )
        .expect("test gate must be valid")
    }

    #[test]
    fn identity_pauli_is_identity() {
        let pauli =
            Pauli::identity(8).expect("identity allocation");

        assert!(pauli.is_identity());
        assert_eq!(pauli.phase(), 0);
    }

    #[test]
    fn pauli_multiplication_is_exact() {
        let x =
            Pauli::x(1, 0).expect("X");
        let z =
            Pauli::z(1, 0).expect("Z");

        let y =
            x.multiply(&z).expect("XZ");

        assert_eq!(y.character_at(0), 'Y');
        assert_eq!(y.phase(), 2);
    }

    #[test]
    fn h_conjugates_x_to_z() {
        let mut tableau =
            CliffordTableau::identity(1)
                .expect("identity");

        tableau
            .apply_gate(&gate(GateKind::H, &[0]))
            .expect("H");

        assert_eq!(
            tableau
                .x_image(0)
                .expect("X image")
                .character_at(0),
            'Z'
        );

        assert_eq!(
            tableau
                .z_image(0)
                .expect("Z image")
                .character_at(0),
            'X'
        );
    }

    #[test]
    fn s_conjugates_x_to_y() {
        let mut tableau =
            CliffordTableau::identity(1)
                .expect("identity");

        tableau
            .apply_gate(&gate(GateKind::S, &[0]))
            .expect("S");

        let image =
            tableau.x_image(0).expect("X image");

        assert_eq!(image.character_at(0), 'Y');
        assert_eq!(image.phase(), 1);
    }

    #[test]
    fn sdg_conjugates_x_to_minus_y() {
        let mut tableau =
            CliffordTableau::identity(1)
                .expect("identity");

        tableau
            .apply_gate(&gate(GateKind::Sdg, &[0]))
            .expect("Sdg");

        let image =
            tableau.x_image(0).expect("X image");

        assert_eq!(image.character_at(0), 'Y');
        assert_eq!(image.phase(), 3);
    }

    #[test]
    fn cx_has_correct_generator_images() {
        let mut tableau =
            CliffordTableau::identity(2)
                .expect("identity");

        tableau
            .apply_gate(&gate(GateKind::CX, &[0, 1]))
            .expect("CX");

        let x0 =
            tableau.x_image(0).expect("X0");

        assert!(x0.has_x(0));
        assert!(x0.has_x(1));

        let z1 =
            tableau.z_image(1).expect("Z1");

        assert!(z1.has_z(0));
        assert!(z1.has_z(1));
    }

    #[test]
    fn swap_exchanges_generator_images() {
        let mut tableau =
            CliffordTableau::identity(2)
                .expect("identity");

        tableau
            .apply_gate(&gate(GateKind::SWAP, &[0, 1]))
            .expect("SWAP");

        assert!(tableau
            .x_image(0)
            .expect("X0")
            .has_x(1));

        assert!(tableau
            .x_image(1)
            .expect("X1")
            .has_x(0));

        assert!(tableau
            .z_image(0)
            .expect("Z0")
            .has_z(1));

        assert!(tableau
            .z_image(1)
            .expect("Z1")
            .has_z(0));
    }

    #[test]
    fn double_h_is_identity() {
        let mut tableau =
            CliffordTableau::identity(1)
                .expect("identity");

        let h =
            gate(GateKind::H, &[0]);

        tableau.apply_gate(&h).expect("H");
        tableau.apply_gate(&h).expect("H");

        assert!(tableau.is_identity());
    }

    #[test]
    fn double_s_is_z() {
        let mut tableau =
            CliffordTableau::identity(1)
                .expect("identity");

        let s =
            gate(GateKind::S, &[0]);

        tableau.apply_gate(&s).expect("S");
        tableau.apply_gate(&s).expect("S");

        let x =
            tableau.x_image(0).expect("X");

        assert_eq!(x.character_at(0), 'X');
        assert_eq!(x.phase(), 2);

        let z =
            tableau.z_image(0).expect("Z");

        assert_eq!(z.character_at(0), 'Z');
        assert_eq!(z.phase(), 0);
    }

    #[test]
    fn v_is_clifford() {
        let classification =
            classify_gate(&gate(GateKind::V, &[0]))
                .expect("V");

        assert_eq!(
            classification.class,
            CliffordGateClass::SingleQubit
        );
    }

    #[test]
    fn cy_is_clifford() {
        let classification =
            classify_gate(&gate(GateKind::CY, &[0, 1]))
                .expect("CY");

        assert_eq!(
            classification.class,
            CliffordGateClass::TwoQubit
        );
    }

    #[test]
    fn ch_is_clifford() {
        let classification =
            classify_gate(&gate(GateKind::CH, &[0, 1]))
                .expect("CH");

        assert_eq!(
            classification.class,
            CliffordGateClass::TwoQubit
        );
    }

    #[test]
    fn iswap_is_clifford() {
        let classification =
            classify_gate(&gate(GateKind::ISWAP, &[0, 1]))
                .expect("iSWAP");

        assert_eq!(
            classification.class,
            CliffordGateClass::TwoQubit
        );
    }

    #[test]
    fn parameterized_rotation_is_not_guessed_as_clifford() {
        let parameter = crate::quantum::ir::Parameter::constant(0.0)
            .expect("finite parameter");

        let rotation = Gate::new(
            GateKind::RZ,
            vec![QubitId::new(0)],
            vec![parameter],
            None,
            None,
        )
        .expect("RZ");

        assert!(matches!(
            classify_gate(&rotation),
            Err(CliffordError::ParameterizedGate {
                gate: GateKind::RZ
            })
        ));
    }

    #[test]
    fn measurement_is_rejected() {
        let measurement = Gate::new(
            GateKind::Measure,
            vec![QubitId::new(0)],
            Vec::new(),
            Some(0),
            None,
        )
        .expect("measurement");

        assert!(matches!(
            classify_gate(&measurement),
            Err(CliffordError::NonUnitaryGate {
                gate: GateKind::Measure
            })
        ));
    }

    #[test]
    fn tableau_preserves_symplectic_relations() {
        let mut tableau =
            CliffordTableau::identity(3)
                .expect("identity");

        let gates = [
            gate(GateKind::H, &[0]),
            gate(GateKind::S, &[1]),
            gate(GateKind::CX, &[0, 1]),
            gate(GateKind::CZ, &[1, 2]),
            gate(GateKind::SWAP, &[0, 2]),
        ];

        for operation in &gates {
            tableau
                .apply_gate(operation)
                .expect("Clifford operation");
        }

        tableau.validate().expect("valid tableau");
    }

    #[test]
    fn same_clifford_is_equivalent_up_to_global_phase() {
        let mut first =
            CliffordTableau::identity(2)
                .expect("identity");

        let mut second =
            CliffordTableau::identity(2)
                .expect("identity");

        first
            .apply_gate(&gate(GateKind::H, &[0]))
            .expect("H");

        second
            .apply_gate(&gate(GateKind::H, &[0]))
            .expect("H");

        assert!(first
            .equivalent_up_to_global_phase(&second)
            .expect("comparison"));
    }
}