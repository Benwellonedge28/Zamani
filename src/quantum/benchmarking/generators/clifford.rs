//! Zamani Quantum Benchmarking — Clifford Generator
//!
//! Production Clifford-group generation primitives for randomized
//! benchmarking and related quantum characterization protocols.
//!
//! # Architectural role
//!
//! This module owns:
//!
//! - single-qubit Clifford-group representation;
//! - exact enumeration of the 24 single-qubit Clifford elements;
//! - uniform single-qubit Clifford sampling;
//! - Clifford composition;
//! - Clifford inversion;
//! - Pauli conjugation;
//! - deterministic primitive-gate decompositions;
//! - generation of reproducible Clifford sequences;
//! - construction of inverse/recovery sequences;
//! - generation of composable multi-qubit Clifford circuits from explicit
//!   Clifford primitives.
//!
//! This module does NOT own:
//!
//! - Quantum IR construction;
//! - backend selection;
//! - physical-qubit mapping;
//! - routing;
//! - scheduling;
//! - calibration;
//! - hardware execution;
//! - statistical fitting;
//! - RB analysis;
//! - fidelity/error estimation;
//! - reporting.
//!
//! Those responsibilities belong to the surrounding benchmarking,
//! compiler, runtime, and hardware layers.
//!
//! # Architectural boundary
//!
//! ```text
//! benchmarking::generators::clifford
//!             │
//!             ├── randomized_benchmarking
//!             ├── interleaved_rb
//!             ├── simultaneous_rb
//!             ├── purity_rb
//!             ├── leakage_rb
//!             └── other Clifford-based protocols
//!
//!                    │
//!                    ▼
//!             BenchmarkCircuit
//!                    │
//!                    ▼
//!               Quantum IR
//!                    │
//!             routing/scheduling
//!                    │
//!                    ▼
//!              backend/runtime
//! ```
//!
//! The dependency direction must never be reversed.
//!
//! # Clifford-group semantics
//!
//! A Clifford operation maps the Pauli group to itself under conjugation:
//!
//!     C P C† ∈ P
//!
//! modulo global phase.
//!
//! For one qubit there are exactly 24 Clifford elements. This module
//! represents those elements by their action on X, Y and Z. That gives a
//! compact, exact, phase-free representation suitable for randomized
//! benchmarking.
//!
//! # Important distinction: one-qubit vs n-qubit sampling
//!
//! [`SingleQubitClifford`] provides exact uniform sampling from C₁,
//! containing exactly 24 elements.
//!
//! [`MultiQubitCliffordCircuit`] deliberately does NOT claim to sample
//! uniformly from Cₙ. It generates valid Clifford circuits from supported
//! one-qubit Clifford operations and CX operations. Uniform sampling from
//! the complete n-qubit Clifford group requires a symplectic/tableau
//! sampler and belongs to a future dedicated scalable representation.
//!
//! This distinction prevents statistically invalid randomized-benchmarking
//! experiments caused by treating an arbitrary random Clifford circuit as a
//! uniformly sampled element of Cₙ.
//!
//! # Reproducibility
//!
//! All stochastic APIs require an explicit RNG. No global RNG is used.
//!
//! [`SeededCliffordSampler`] provides a small deterministic sampler for
//! standalone reproducibility tests. The benchmark-wide RNG abstraction
//! should eventually be provided by `generators::random`; callers can pass
//! its `RngCore` implementation directly to [`CliffordSampler`].
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//!
//! No nightly features are required.
//!
//! Existing dependency used:
//!
//! - `rand = 0.8`
//!
//! No additional dependency is introduced by this module.
//!
//! # Integration contract
//!
//! Future files consume this module as follows:
//!
//! ```text
//! generators/random.rs
//!        │
//!        ▼
//! generators/clifford.rs
//!        │
//!        ├──────────────► protocols/randomized_benchmarking.rs
//!        ├──────────────► protocols/interleaved_rb.rs
//!        ├──────────────► protocols/simultaneous_rb.rs
//!        ├──────────────► protocols/purity_rb.rs
//!        ├──────────────► protocols/leakage_rb.rs
//!        └──────────────► generators/random_circuits.rs
//!
//! CliffordPrimitiveSequence
//!        │
//!        ▼
//! generators / core::circuit
//!        │
//!        ▼
//! quantum::ir
//! ```
//!
//! The file intentionally does not import `core::circuit` because its
//! canonical constructor API is a separate architectural layer. This keeps
//! the generator independently testable and prevents generator/IR coupling.
//!
//! # Scientific correctness
//!
//! A Clifford sequence used by standard randomized benchmarking normally
//! consists of random Clifford elements followed by a recovery Clifford
//! equal to the inverse of their product. The APIs in this file expose both
//! composition and inverse operations so that the RB protocol can construct
//! that recovery operation without duplicating group mathematics.
//!
//! The final RB protocol remains responsible for deciding:
//!
//! - sequence lengths;
//! - number of random sequences;
//! - shots;
//! - sampling policy;
//! - fitting;
//! - confidence intervals;
//! - error-per-Clifford calculation.
//!
//! Those quantities do not belong here.
//!
//! # Safety/resource limits
//!
//! This module rejects:
//!
//! - invalid Clifford indices;
//! - zero-qubit multi-qubit configurations;
//! - invalid qubit indices;
//! - duplicate qubit operands for two-qubit operations;
//! - arithmetic overflow in sequence sizing;
//! - impossible sequence lengths;
//! - unsupported operations.
//!
//! The benchmark-wide `core::limits` layer remains responsible for
//! experiment-level resource limits.

use rand::RngCore;
use std::fmt;

// =============================================================================
// Constants
// =============================================================================

/// Number of elements in the single-qubit Clifford group C₁.
///
/// C₁ contains exactly 24 phase-free Clifford operations.
pub const SINGLE_QUBIT_CLIFFORD_COUNT: usize = 24;

/// Stable version of the single-qubit Clifford representation.
pub const CLIFFORD_REPRESENTATION_VERSION: u16 = 1;

/// Stable version of the primitive decomposition table.
///
/// Changing this value is required if the decomposition semantics change.
pub const CLIFFORD_DECOMPOSITION_VERSION: u16 = 1;

/// Maximum supported number of qubits for the generic circuit builder in
/// this module.
///
/// This is intentionally conservative. It prevents accidental construction
/// of enormous in-memory benchmark workloads before the benchmark-wide
/// resource-limit layer has been consulted.
///
/// This is NOT a physical limitation of Clifford computation.
pub const DEFAULT_MAX_MULTI_QUBIT_COUNT: usize = 4096;

/// Maximum number of primitive operations that one generated circuit may
/// contain through this module's local safety checks.
pub const DEFAULT_MAX_GENERATED_OPERATIONS: usize = 1_000_000;

// =============================================================================
// Pauli representation
// =============================================================================

/// One of the phase-free single-qubit Pauli axes.
///
/// The identity is included because it is useful when describing conjugation
/// and group actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Pauli {
    /// Identity operator.
    I,

    /// Pauli-X.
    X,

    /// Pauli-Y.
    Y,

    /// Pauli-Z.
    Z,
}

impl Pauli {
    /// Returns the non-identity Pauli axes.
    pub const ALL_NON_IDENTITY: [Self; 3] =
        [Self::X, Self::Y, Self::Z];

    /// Returns the identity-free axis index.
    ///
    /// X = 0, Y = 1, Z = 2.
    pub const fn axis_index(self) -> Option<usize> {
        match self {
            Self::I => None,
            Self::X => Some(0),
            Self::Y => Some(1),
            Self::Z => Some(2),
        }
    }

    /// Creates a Pauli from an axis index.
    pub const fn from_axis_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::X),
            1 => Some(Self::Y),
            2 => Some(Self::Z),
            _ => None,
        }
    }
}

// =============================================================================
// Signed Pauli
// =============================================================================

/// A Pauli axis with a sign.
///
/// Clifford conjugation can map:
///
///     X ->  Y
///     X -> -Y
///     X ->  Z
///     ...
///
/// This type records the phase-free ± sign relevant to Clifford group
/// conjugation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignedPauli {
    pauli: Pauli,
    negative: bool,
}

impl SignedPauli {
    /// Creates a positive Pauli.
    pub const fn positive(pauli: Pauli) -> Self {
        Self {
            pauli,
            negative: false,
        }
    }

    /// Creates a negative Pauli.
    pub const fn negative(pauli: Pauli) -> Self {
        Self {
            pauli,
            negative: true,
        }
    }

    /// Returns the underlying Pauli.
    pub const fn pauli(self) -> Pauli {
        self.pauli
    }

    /// Returns whether the sign is negative.
    pub const fn is_negative(self) -> bool {
        self.negative
    }

    /// Returns whether the sign is positive.
    pub const fn is_positive(self) -> bool {
        !self.negative
    }

    /// Returns the sign as +1 or -1.
    pub const fn sign(self) -> i8 {
        if self.negative {
            -1
        } else {
            1
        }
    }

    /// Flips the sign.
    pub const fn negated(self) -> Self {
        Self {
            pauli: self.pauli,
            negative: !self.negative,
        }
    }
}

impl fmt::Display for SignedPauli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            write!(f, "-")?;
        }

        write!(f, "{:?}", self.pauli)
    }
}

// =============================================================================
// Clifford primitive gates
// =============================================================================

/// Primitive Clifford gates understood by this generator.
///
/// These are logical operations. They are not physical pulses.
///
/// A backend may later lower these to its native gate set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CliffordPrimitive {
    /// Hadamard gate.
    H,

    /// S = RZ(π/2).
    S,

    /// S† = RZ(-π/2).
    Sdg,
}

impl CliffordPrimitive {
    /// Returns the inverse primitive.
    pub const fn inverse(self) -> Self {
        match self {
            Self::H => Self::H,
            Self::S => Self::Sdg,
            Self::Sdg => Self::S,
        }
    }

    /// Returns whether the primitive is self-inverse.
    pub const fn is_self_inverse(self) -> bool {
        matches!(self, Self::H)
    }

    /// Returns the Pauli action of the primitive.
    pub const fn conjugate(self, pauli: Pauli) -> SignedPauli {
        match self {
            Self::H => match pauli {
                Pauli::I => SignedPauli::positive(Pauli::I),
                Pauli::X => SignedPauli::positive(Pauli::Z),
                Pauli::Y => SignedPauli::negative(Pauli::Y),
                Pauli::Z => SignedPauli::positive(Pauli::X),
            },

            Self::S => match pauli {
                Pauli::I => SignedPauli::positive(Pauli::I),
                Pauli::X => SignedPauli::positive(Pauli::Y),
                Pauli::Y => SignedPauli::negative(Pauli::X),
                Pauli::Z => SignedPauli::positive(Pauli::Z),
            },

            Self::Sdg => match pauli {
                Pauli::I => SignedPauli::positive(Pauli::I),
                Pauli::X => SignedPauli::negative(Pauli::Y),
                Pauli::Y => SignedPauli::positive(Pauli::X),
                Pauli::Z => SignedPauli::positive(Pauli::Z),
            },
        }
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by Clifford generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliffordError {
    /// The Clifford index is outside the 24-element C₁ group.
    InvalidCliffordIndex {
        index: usize,
    },

    /// A sequence length exceeded the local generation limit.
    SequenceTooLong {
        requested: usize,
        maximum: usize,
    },

    /// A circuit operation count exceeded the local generation limit.
    OperationLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// A multi-qubit circuit cannot contain zero qubits.
    InvalidQubitCount,

    /// A qubit index is outside the declared circuit width.
    InvalidQubitIndex {
        qubit: usize,
        qubit_count: usize,
    },

    /// A two-qubit operation used the same qubit twice.
    DuplicateQubit {
        qubit: usize,
    },

    /// A primitive operation was not supported by the requested operation.
    UnsupportedOperation {
        operation: &'static str,
    },

    /// Arithmetic overflow occurred while calculating a requested size.
    SizeOverflow,

    /// An empty sequence cannot be inverted as an RB sequence unless the
    /// caller explicitly requests the identity recovery.
    EmptyRecoverySequence,

    /// A recovery sequence was constructed with an inconsistent accumulated
    /// Clifford.
    InvalidRecovery,

    /// The generated sequence did not satisfy the requested operation limit.
    InvalidGeneratedSequence,
}

impl fmt::Display for CliffordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCliffordIndex { index } => {
                write!(
                    f,
                    "invalid single-qubit Clifford index {index}; \
                     valid range is 0..{SINGLE_QUBIT_CLIFFORD_COUNT}"
                )
            }

            Self::SequenceTooLong {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "Clifford sequence length {requested} exceeds \
                     maximum {maximum}"
                )
            }

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "generated Clifford operation count {requested} \
                     exceeds maximum {maximum}"
                )
            }

            Self::InvalidQubitCount => {
                write!(f, "a Clifford circuit requires at least one qubit")
            }

            Self::InvalidQubitIndex {
                qubit,
                qubit_count,
            } => {
                write!(
                    f,
                    "qubit index {qubit} is outside circuit width \
                     {qubit_count}"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    f,
                    "two-qubit Clifford operation cannot target \
                     qubit {qubit} twice"
                )
            }

            Self::UnsupportedOperation { operation } => {
                write!(f, "unsupported Clifford operation: {operation}")
            }

            Self::SizeOverflow => {
                write!(
                    f,
                    "Clifford sequence size calculation overflowed"
                )
            }

            Self::EmptyRecoverySequence => {
                write!(
                    f,
                    "an empty Clifford sequence does not require a \
                     non-empty recovery operation"
                )
            }

            Self::InvalidRecovery => {
                write!(
                    f,
                    "generated Clifford recovery is inconsistent with \
                     the accumulated Clifford"
                )
            }

            Self::InvalidGeneratedSequence => {
                write!(
                    f,
                    "generated Clifford sequence failed validation"
                )
            }
        }
    }
}

impl std::error::Error for CliffordError {}

// =============================================================================
// Single-qubit Clifford representation
// =============================================================================

/// Exact phase-free representation of a single-qubit Clifford operation.
///
/// The representation stores the images of X, Y and Z under conjugation:
///
///     C X C†
///     C Y C†
///     C Z C†
///
/// Because Clifford operations preserve the Pauli algebra, these three
/// signed axes uniquely identify an element of C₁ up to global phase.
///
/// There are exactly 24 valid representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SingleQubitClifford {
    x: SignedPauli,
    y: SignedPauli,
    z: SignedPauli,
}

impl SingleQubitClifford {
    /// Identity Clifford.
    pub const IDENTITY: Self = Self {
        x: SignedPauli::positive(Pauli::X),
        y: SignedPauli::positive(Pauli::Y),
        z: SignedPauli::positive(Pauli::Z),
    };

    /// Creates a Clifford from its complete Pauli action.
    ///
    /// The constructor validates that:
    ///
    /// - X/Y/Z map to distinct non-identity axes;
    /// - the signed permutation preserves orientation.
    pub const fn new(
        x: SignedPauli,
        y: SignedPauli,
        z: SignedPauli,
    ) -> Result<Self, CliffordError> {
        let x_axis = match x.pauli().axis_index() {
            Some(value) => value,
            None => {
                return Err(CliffordError::InvalidGeneratedSequence);
            }
        };

        let y_axis = match y.pauli().axis_index() {
            Some(value) => value,
            None => {
                return Err(CliffordError::InvalidGeneratedSequence);
            }
        };

        let z_axis = match z.pauli().axis_index() {
            Some(value) => value,
            None => {
                return Err(CliffordError::InvalidGeneratedSequence);
            }
        };

        if x_axis == y_axis || x_axis == z_axis || y_axis == z_axis {
            return Err(CliffordError::InvalidGeneratedSequence);
        }

        let permutation_parity =
            permutation_parity(x_axis, y_axis, z_axis);

        let sign_product =
            (x.sign() as i16) *
            (y.sign() as i16) *
            (z.sign() as i16);

        if permutation_parity * sign_product != 1 {
            return Err(CliffordError::InvalidGeneratedSequence);
        }

        Ok(Self { x, y, z })
    }

    /// Returns a Clifford by its canonical index.
    ///
    /// Indexing is stable for the lifetime of representation version 1.
    pub fn from_index(index: usize) -> Result<Self, CliffordError> {
        if index >= SINGLE_QUBIT_CLIFFORD_COUNT {
            return Err(CliffordError::InvalidCliffordIndex { index });
        }

        let (permutation, signs) = canonical_element(index);

        Self::new(
            signed_pauli_from_parts(permutation[0], signs[0]),
            signed_pauli_from_parts(permutation[1], signs[1]),
            signed_pauli_from_parts(permutation[2], signs[2]),
        )
    }

    /// Returns the canonical index of this Clifford.
    pub fn index(self) -> Result<usize, CliffordError> {
        for index in 0..SINGLE_QUBIT_CLIFFORD_COUNT {
            if Self::from_index(index)? == self {
                return Ok(index);
            }
        }

        Err(CliffordError::InvalidGeneratedSequence)
    }

    /// Returns the image of X.
    pub const fn x_image(self) -> SignedPauli {
        self.x
    }

    /// Returns the image of Y.
    pub const fn y_image(self) -> SignedPauli {
        self.y
    }

    /// Returns the image of Z.
    pub const fn z_image(self) -> SignedPauli {
        self.z
    }

    /// Applies the Clifford to a Pauli operator.
    pub const fn conjugate(self, pauli: Pauli) -> SignedPauli {
        match pauli {
            Pauli::I => SignedPauli::positive(Pauli::I),
            Pauli::X => self.x,
            Pauli::Y => self.y,
            Pauli::Z => self.z,
        }
    }

    /// Returns whether this is the identity Clifford.
    pub const fn is_identity(self) -> bool {
        self.x.pauli() == Pauli::X
            && self.x.is_positive()
            && self.y.pauli() == Pauli::Y
            && self.y.is_positive()
            && self.z.pauli() == Pauli::Z
            && self.z.is_positive()
    }

    /// Returns the inverse Clifford.
    pub fn inverse(self) -> Result<Self, CliffordError> {
        let mut result = [SignedPauli::positive(Pauli::I); 3];

        for input in 0..3 {
            let target = match input {
                0 => Pauli::X,
                1 => Pauli::Y,
                _ => Pauli::Z,
            };

            let image = self.conjugate(target);
            let output_axis = match image.pauli().axis_index() {
                Some(value) => value,
                None => {
                    return Err(
                        CliffordError::InvalidGeneratedSequence
                    );
                }
            };

            result[output_axis] = SignedPauli {
                pauli: target,
                negative: image.is_negative(),
            };
        }

        Self::new(result[0], result[1], result[2])
    }

    /// Composes two Clifford operations.
    ///
    /// The returned operation represents:
    ///
    ///     self ∘ other
    ///
    /// meaning `other` is applied first and `self` second.
    pub fn compose(
        self,
        other: Self,
    ) -> Result<Self, CliffordError> {
        let x = compose_signed_pauli(
            self.conjugate(Pauli::X),
            other.conjugate(Pauli::X),
        );

        let y = compose_signed_pauli(
            self.conjugate(Pauli::Y),
            other.conjugate(Pauli::Y),
        );

        let z = compose_signed_pauli(
            self.conjugate(Pauli::Z),
            other.conjugate(Pauli::Z),
        );

        Self::new(x, y, z)
    }

    /// Returns the canonical H Clifford.
    pub const fn hadamard() -> Self {
        Self {
            x: SignedPauli::positive(Pauli::Z),
            y: SignedPauli::negative(Pauli::Y),
            z: SignedPauli::positive(Pauli::X),
        }
    }

    /// Returns the canonical S Clifford.
    pub const fn phase() -> Self {
        Self {
            x: SignedPauli::positive(Pauli::Y),
            y: SignedPauli::negative(Pauli::X),
            z: SignedPauli::positive(Pauli::Z),
        }
    }

    /// Returns the canonical S† Clifford.
    pub const fn phase_dagger() -> Self {
        Self {
            x: SignedPauli::negative(Pauli::Y),
            y: SignedPauli::positive(Pauli::X),
            z: SignedPauli::positive(Pauli::Z),
        }
    }

    /// Returns a uniformly random element of C₁.
    ///
    /// Uniformity is exact because all 24 elements are selected with equal
    /// probability using rejection-free bounded sampling.
    pub fn random<R>(rng: &mut R) -> Result<Self, CliffordError>
    where
        R: RngCore + ?Sized,
    {
        let index = bounded_index(
            rng,
            SINGLE_QUBIT_CLIFFORD_COUNT as u32,
        );

        Self::from_index(index as usize)
    }

    /// Returns the primitive decomposition of this Clifford.
    ///
    /// The decomposition uses only H, S and S†.
    ///
    /// The returned sequence is logically equivalent to this Clifford.
    pub fn decomposition(
        self,
    ) -> Result<Vec<CliffordPrimitive>, CliffordError> {
        let index = self.index()?;

        Ok(canonical_decomposition(index)
            .iter()
            .copied()
            .collect())
    }

    /// Returns the number of primitive gates in the canonical decomposition.
    pub fn decomposition_len(self) -> Result<usize, CliffordError> {
        Ok(canonical_decomposition(self.index()?).len())
    }
}

impl Default for SingleQubitClifford {
    fn default() -> Self {
        Self::IDENTITY
    }
}

// =============================================================================
// Clifford primitive sequence
// =============================================================================

/// A primitive Clifford operation applied to a particular logical qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CliffordOperation {
    /// Target logical qubit.
    pub qubit: usize,

    /// Primitive Clifford operation.
    pub primitive: CliffordPrimitive,
}

impl CliffordOperation {
    /// Creates a one-qubit Clifford primitive operation.
    pub const fn new(
        qubit: usize,
        primitive: CliffordPrimitive,
    ) -> Self {
        Self { qubit, primitive }
    }

    /// Returns the inverse operation.
    pub const fn inverse(self) -> Self {
        Self {
            qubit: self.qubit,
            primitive: self.primitive.inverse(),
        }
    }
}

/// A deterministic sequence of primitive Clifford operations.
///
/// The sequence is deliberately independent of Quantum IR. It can therefore
/// be tested mathematically before being lowered into the canonical IR.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliffordPrimitiveSequence {
    operations: Vec<CliffordOperation>,
}

impl CliffordPrimitiveSequence {
    /// Creates an empty Clifford primitive sequence.
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Creates a sequence with preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            operations: Vec::with_capacity(capacity),
        }
    }

    /// Creates a sequence from operations.
    pub fn from_operations(
        operations: Vec<CliffordOperation>,
    ) -> Self {
        Self { operations }
    }

    /// Returns the operations.
    pub fn operations(&self) -> &[CliffordOperation] {
        &self.operations
    }

    /// Returns the number of primitive operations.
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Appends one operation.
    pub fn push(&mut self, operation: CliffordOperation) {
        self.operations.push(operation);
    }

    /// Extends the sequence.
    pub fn extend(
        &mut self,
        operations: impl IntoIterator<Item = CliffordOperation>,
    ) {
        self.operations.extend(operations);
    }

    /// Returns the inverse primitive sequence.
    ///
    /// Inversion reverses operation order and replaces every primitive by
    /// its inverse.
    pub fn inverse(&self) -> Self {
        let operations = self
            .operations
            .iter()
            .rev()
            .copied()
            .map(CliffordOperation::inverse)
            .collect();

        Self { operations }
    }

    /// Validates all qubit indices against a circuit width.
    pub fn validate_for_qubits(
        &self,
        qubit_count: usize,
    ) -> Result<(), CliffordError> {
        if qubit_count == 0 {
            return Err(CliffordError::InvalidQubitCount);
        }

        for operation in &self.operations {
            if operation.qubit >= qubit_count {
                return Err(
                    CliffordError::InvalidQubitIndex {
                        qubit: operation.qubit,
                        qubit_count,
                    },
                );
            }
        }

        Ok(())
    }
}

impl Default for CliffordPrimitiveSequence {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Random Clifford sampler
// =============================================================================

/// Explicit random Clifford sampler.
///
/// This wrapper exists so callers can make the randomness source explicit
/// without introducing process-global state.
#[derive(Debug, Default, Clone, Copy)]
pub struct CliffordSampler;

impl CliffordSampler {
    /// Creates a sampler.
    pub const fn new() -> Self {
        Self
    }

    /// Samples one uniformly distributed single-qubit Clifford.
    pub fn sample<R>(
        &self,
        rng: &mut R,
    ) -> Result<SingleQubitClifford, CliffordError>
    where
        R: RngCore + ?Sized,
    {
        SingleQubitClifford::random(rng)
    }

    /// Samples `count` independent uniformly distributed C₁ elements.
    pub fn sample_sequence<R>(
        &self,
        rng: &mut R,
        count: usize,
    ) -> Result<Vec<SingleQubitClifford>, CliffordError>
    where
        R: RngCore + ?Sized,
    {
        validate_sequence_length(count)?;

        let mut output = Vec::with_capacity(count);

        for _ in 0..count {
            output.push(self.sample(rng)?);
        }

        Ok(output)
    }
}

// =============================================================================
// Deterministic seeded sampler
// =============================================================================

/// Deterministic seed wrapper for standalone reproducibility.
///
/// The benchmark-wide random generator should normally provide the RNG
/// supplied to [`CliffordSampler`]. This type exists for cases where a
/// self-contained deterministic sampler is useful in tests or tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeededCliffordSampler {
    state: u64,
}

impl SeededCliffordSampler {
    /// Creates a deterministic sampler from an explicit seed.
    ///
    /// Seed zero is valid. The generator substitutes a fixed non-zero state
    /// internally so that the underlying recurrence never becomes stuck.
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
    /// This is useful for checkpointing deterministic generation.
    pub const fn state(&self) -> u64 {
        self.state
    }

    /// Samples one uniformly distributed C₁ Clifford.
    pub fn sample(
        &mut self,
    ) -> Result<SingleQubitClifford, CliffordError> {
        let value = self.next_u64();

        let index =
            bounded_index_from_u64(
                value,
                SINGLE_QUBIT_CLIFFORD_COUNT as u32,
            );

        SingleQubitClifford::from_index(index as usize)
    }

    /// Samples a deterministic sequence.
    pub fn sample_sequence(
        &mut self,
        count: usize,
    ) -> Result<Vec<SingleQubitClifford>, CliffordError> {
        validate_sequence_length(count)?;

        let mut output = Vec::with_capacity(count);

        for _ in 0..count {
            output.push(self.sample()?);
        }

        Ok(output)
    }

    /// SplitMix64 next-value generator.
    ///
    /// The algorithm is intentionally local and versioned by this module's
    /// public representation contract. It is not tied to the implementation
    /// details of `rand`.
    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_add(0x9E37_79B9_7F4A_7C15);

        let mut z = self.state;

        z = (z ^ (z >> 30))
            .wrapping_mul(0xBF58_476D_1CE4_E5B9);

        z = (z ^ (z >> 27))
            .wrapping_mul(0x94D0_49BB_1331_11EB);

        z ^ (z >> 31)
    }
}

// =============================================================================
// Randomized benchmarking sequence
// =============================================================================

/// A sequence of C₁ Clifford elements plus its recovery Clifford.
///
/// This is the mathematical representation needed by standard single-qubit
/// randomized benchmarking.
///
/// It does not contain shots, observations, statistics, or hardware data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliffordBenchmarkSequence {
    random_cliffords: Vec<SingleQubitClifford>,
    recovery: SingleQubitClifford,
}

impl CliffordBenchmarkSequence {
    /// Generates a random Clifford sequence of `length` elements.
    ///
    /// The recovery is computed as:
    ///
    ///     (Cₙ ... C₂ C₁)⁻¹
    ///
    /// according to the composition convention documented on
    /// [`SingleQubitClifford::compose`].
    pub fn generate<R>(
        rng: &mut R,
        length: usize,
    ) -> Result<Self, CliffordError>
    where
        R: RngCore + ?Sized,
    {
        validate_sequence_length(length)?;

        let sampler = CliffordSampler::new();

        let mut random_cliffords =
            Vec::with_capacity(length);

        let mut accumulated =
            SingleQubitClifford::IDENTITY;

        for _ in 0..length {
            let clifford = sampler.sample(rng)?;

            accumulated =
                clifford.compose(accumulated)?;

            random_cliffords.push(clifford);
        }

        let recovery = accumulated.inverse()?;

        Ok(Self {
            random_cliffords,
            recovery,
        })
    }

    /// Generates a deterministic sequence from an explicit seed.
    pub fn generate_seeded(
        seed: u64,
        length: usize,
    ) -> Result<Self, CliffordError> {
        let mut sampler = SeededCliffordSampler::new(seed);

        validate_sequence_length(length)?;

        let mut random_cliffords =
            Vec::with_capacity(length);

        let mut accumulated =
            SingleQubitClifford::IDENTITY;

        for _ in 0..length {
            let clifford = sampler.sample()?;

            accumulated =
                clifford.compose(accumulated)?;

            random_cliffords.push(clifford);
        }

        let recovery = accumulated.inverse()?;

        Ok(Self {
            random_cliffords,
            recovery,
        })
    }

    /// Returns the random Clifford elements.
    pub fn random_cliffords(
        &self,
    ) -> &[SingleQubitClifford] {
        &self.random_cliffords
    }

    /// Returns the recovery Clifford.
    pub const fn recovery(&self) -> SingleQubitClifford {
        self.recovery
    }

    /// Returns the number of random Clifford elements.
    pub fn len(&self) -> usize {
        self.random_cliffords.len()
    }

    /// Returns whether there are no random Clifford elements.
    pub fn is_empty(&self) -> bool {
        self.random_cliffords.is_empty()
    }

    /// Returns the complete Clifford sequence including recovery.
    pub fn complete_cliffords(
        &self,
    ) -> Vec<SingleQubitClifford> {
        let additional = usize::from(!self.is_empty());

        let mut result =
            Vec::with_capacity(self.random_cliffords.len() + additional);

        result.extend_from_slice(&self.random_cliffords);

        if !self.is_empty() {
            result.push(self.recovery);
        }

        result
    }

    /// Returns the accumulated Clifford before recovery.
    pub fn accumulated_clifford(
        &self,
    ) -> Result<SingleQubitClifford, CliffordError> {
        let mut accumulated =
            SingleQubitClifford::IDENTITY;

        for clifford in &self.random_cliffords {
            accumulated =
                clifford.compose(accumulated)?;
        }

        Ok(accumulated)
    }

    /// Validates the recovery Clifford.
    pub fn validate_recovery(
        &self,
    ) -> Result<(), CliffordError> {
        let accumulated = self.accumulated_clifford()?;

        let recovered =
            self.recovery.compose(accumulated)?;

        if !recovered.is_identity() {
            return Err(CliffordError::InvalidRecovery);
        }

        Ok(())
    }

    /// Converts the sequence into primitive logical operations on one qubit.
    ///
    /// The recovery is included.
    pub fn to_primitive_sequence(
        &self,
        qubit: usize,
    ) -> Result<CliffordPrimitiveSequence, CliffordError> {
        let mut sequence =
            CliffordPrimitiveSequence::new();

        for clifford in self.complete_cliffords() {
            for primitive in clifford.decomposition()? {
                sequence.push(
                    CliffordOperation::new(
                        qubit,
                        primitive,
                    ),
                );
            }
        }

        Ok(sequence)
    }
}

// =============================================================================
// Multi-qubit Clifford operations
// =============================================================================

/// Supported logical multi-qubit Clifford operation.
///
/// These are sufficient to construct explicit Clifford circuits while keeping
/// the generator independent of a particular backend native gate set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MultiQubitCliffordOperation {
    /// Single-qubit Clifford primitive.
    Single {
        /// Target logical qubit.
        qubit: usize,

        /// Primitive Clifford.
        primitive: CliffordPrimitive,
    },

    /// Controlled-NOT operation.
    Cx {
        /// Control logical qubit.
        control: usize,

        /// Target logical qubit.
        target: usize,
    },
}

impl MultiQubitCliffordOperation {
    /// Validates this operation against a logical-qubit count.
    pub fn validate(
        self,
        qubit_count: usize,
    ) -> Result<(), CliffordError> {
        if qubit_count == 0 {
            return Err(CliffordError::InvalidQubitCount);
        }

        match self {
            Self::Single { qubit, .. } => {
                if qubit >= qubit_count {
                    return Err(
                        CliffordError::InvalidQubitIndex {
                            qubit,
                            qubit_count,
                        },
                    );
                }
            }

            Self::Cx { control, target } => {
                if control >= qubit_count {
                    return Err(
                        CliffordError::InvalidQubitIndex {
                            qubit: control,
                            qubit_count,
                        },
                    );
                }

                if target >= qubit_count {
                    return Err(
                        CliffordError::InvalidQubitIndex {
                            qubit: target,
                            qubit_count,
                        },
                    );
                }

                if control == target {
                    return Err(
                        CliffordError::DuplicateQubit {
                            qubit: control,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Returns the inverse operation.
    ///
    /// All supported operations are self-inverse except S/S†.
    pub const fn inverse(self) -> Self {
        match self {
            Self::Single { qubit, primitive } => {
                Self::Single {
                    qubit,
                    primitive: primitive.inverse(),
                }
            }

            Self::Cx { control, target } => {
                Self::Cx { control, target }
            }
        }
    }
}

/// Explicit multi-qubit Clifford circuit.
///
/// This is a circuit *recipe*, not a Quantum IR circuit.
///
/// It is intentionally transparent about the operations it contains.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiQubitCliffordCircuit {
    qubit_count: usize,
    operations: Vec<MultiQubitCliffordOperation>,
}

impl MultiQubitCliffordCircuit {
    /// Creates an empty multi-qubit Clifford circuit.
    pub fn new(
        qubit_count: usize,
    ) -> Result<Self, CliffordError> {
        if qubit_count == 0 {
            return Err(CliffordError::InvalidQubitCount);
        }

        if qubit_count > DEFAULT_MAX_MULTI_QUBIT_COUNT {
            return Err(
                CliffordError::InvalidQubitIndex {
                    qubit: qubit_count,
                    qubit_count: DEFAULT_MAX_MULTI_QUBIT_COUNT,
                },
            );
        }

        Ok(Self {
            qubit_count,
            operations: Vec::new(),
        })
    }

    /// Creates an empty circuit with explicit operation capacity.
    pub fn with_capacity(
        qubit_count: usize,
        capacity: usize,
    ) -> Result<Self, CliffordError> {
        let mut circuit = Self::new(qubit_count)?;

        if capacity > DEFAULT_MAX_GENERATED_OPERATIONS {
            return Err(
                CliffordError::OperationLimitExceeded {
                    requested: capacity,
                    maximum: DEFAULT_MAX_GENERATED_OPERATIONS,
                },
            );
        }

        circuit.operations.reserve(capacity);

        Ok(circuit)
    }

    /// Returns the number of logical qubits.
    pub const fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Returns the number of primitive operations.
    pub fn len(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the circuit is empty.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns operations in execution order.
    pub fn operations(
        &self,
    ) -> &[MultiQubitCliffordOperation] {
        &self.operations
    }

    /// Appends an operation.
    pub fn push(
        &mut self,
        operation: MultiQubitCliffordOperation,
    ) -> Result<(), CliffordError> {
        operation.validate(self.qubit_count)?;

        if self.operations.len()
            >= DEFAULT_MAX_GENERATED_OPERATIONS
        {
            return Err(
                CliffordError::OperationLimitExceeded {
                    requested: self
                        .operations
                        .len()
                        .saturating_add(1),
                    maximum: DEFAULT_MAX_GENERATED_OPERATIONS,
                },
            );
        }

        self.operations.push(operation);

        Ok(())
    }

    /// Appends a one-qubit Clifford decomposition.
    pub fn push_clifford(
        &mut self,
        qubit: usize,
        clifford: SingleQubitClifford,
    ) -> Result<(), CliffordError> {
        for primitive in clifford.decomposition()? {
            self.push(
                MultiQubitCliffordOperation::Single {
                    qubit,
                    primitive,
                },
            )?;
        }

        Ok(())
    }

    /// Appends a CNOT operation.
    pub fn push_cx(
        &mut self,
        control: usize,
        target: usize,
    ) -> Result<(), CliffordError> {
        self.push(
            MultiQubitCliffordOperation::Cx {
                control,
                target,
            },
        )
    }

    /// Returns the inverse circuit.
    ///
    /// Operation order is reversed and each operation is inverted.
    pub fn inverse(&self) -> Self {
        let operations = self
            .operations
            .iter()
            .rev()
            .copied()
            .map(
                MultiQubitCliffordOperation::inverse,
            )
            .collect();

        Self {
            qubit_count: self.qubit_count,
            operations,
        }
    }

    /// Returns a circuit containing this circuit followed by its inverse.
    ///
    /// This is an exact structural identity construction at the logical
    /// Clifford level.
    pub fn with_recovery(
        &self,
    ) -> Result<Self, CliffordError> {
        let inverse = self.inverse();

        let total_capacity = self
            .operations
            .len()
            .checked_add(inverse.operations.len())
            .ok_or(CliffordError::SizeOverflow)?;

        if total_capacity > DEFAULT_MAX_GENERATED_OPERATIONS {
            return Err(
                CliffordError::OperationLimitExceeded {
                    requested: total_capacity,
                    maximum: DEFAULT_MAX_GENERATED_OPERATIONS,
                },
            );
        }

        let mut result =
            Self::with_capacity(
                self.qubit_count,
                total_capacity,
            )?;

        for operation in self.operations.iter().copied() {
            result.push(operation)?;
        }

        for operation in inverse.operations.iter().copied() {
            result.push(operation)?;
        }

        Ok(result)
    }
}

// =============================================================================
// Multi-qubit random Clifford circuit generator
// =============================================================================

/// Configuration for explicit random Clifford circuit generation.
///
/// IMPORTANT:
///
/// This generates a valid Clifford circuit but does not claim uniform
/// sampling from the complete n-qubit Clifford group.
///
/// The exact uniform n-qubit Clifford sampler belongs to a future
/// symplectic/tableau generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultiQubitCliffordCircuitConfig {
    /// Number of logical qubits.
    pub qubit_count: usize,

    /// Number of logical Clifford layers.
    pub depth: usize,

    /// Whether each layer may contain a CNOT.
    pub allow_entangling: bool,

    /// Maximum generated primitive operation count.
    pub max_operations: usize,
}

impl MultiQubitCliffordCircuitConfig {
    /// Creates a configuration.
    pub fn new(
        qubit_count: usize,
        depth: usize,
    ) -> Result<Self, CliffordError> {
        if qubit_count == 0 {
            return Err(CliffordError::InvalidQubitCount);
        }

        if qubit_count > DEFAULT_MAX_MULTI_QUBIT_COUNT {
            return Err(
                CliffordError::InvalidQubitIndex {
                    qubit: qubit_count,
                    qubit_count: DEFAULT_MAX_MULTI_QUBIT_COUNT,
                },
            );
        }

        if depth > DEFAULT_MAX_GENERATED_OPERATIONS {
            return Err(
                CliffordError::SequenceTooLong {
                    requested: depth,
                    maximum: DEFAULT_MAX_GENERATED_OPERATIONS,
                },
            );
        }

        Ok(Self {
            qubit_count,
            depth,
            allow_entangling: true,
            max_operations: DEFAULT_MAX_GENERATED_OPERATIONS,
        })
    }

    /// Enables/disables entangling CNOT layers.
    #[must_use]
    pub const fn with_entangling(
        mut self,
        enabled: bool,
    ) -> Self {
        self.allow_entangling = enabled;
        self
    }

    /// Sets the maximum primitive operation count.
    pub fn with_max_operations(
        mut self,
        maximum: usize,
    ) -> Result<Self, CliffordError> {
        if maximum == 0 {
            return Err(
                CliffordError::OperationLimitExceeded {
                    requested: 1,
                    maximum,
                },
            );
        }

        self.max_operations = maximum;

        Ok(self)
    }
}

/// Generates a reproducible random Clifford circuit from an explicit RNG.
///
/// The generator creates independent random one-qubit Clifford layers and,
/// when requested, deterministic nearest-neighbour CNOT opportunities inside
/// each layer.
///
/// It is therefore suitable as a circuit *recipe generator*, but not as a
/// mathematically uniform sampler of Cₙ.
pub fn generate_multi_qubit_clifford_circuit<R>(
    config: MultiQubitCliffordCircuitConfig,
    rng: &mut R,
) -> Result<MultiQubitCliffordCircuit, CliffordError>
where
    R: RngCore + ?Sized,
{
    let mut circuit =
        MultiQubitCliffordCircuit::with_capacity(
            config.qubit_count,
            config
                .depth
                .checked_mul(
                    config.qubit_count,
                )
                .ok_or(CliffordError::SizeOverflow)?,
        )?;

    for layer in 0..config.depth {
        // Single-qubit Clifford layer.
        for qubit in 0..config.qubit_count {
            let clifford =
                SingleQubitClifford::random(rng)?;

            circuit.push_clifford(
                qubit,
                clifford,
            )?;
        }

        // Deterministic nearest-neighbour entangling structure.
        //
        // Alternating parity avoids always using exactly the same edges
        // while remaining completely explicit and reproducible.
        if config.allow_entangling
            && config.qubit_count > 1
        {
            let start = layer & 1;

            let mut control = start;

            while control + 1 < config.qubit_count {
                circuit.push_cx(
                    control,
                    control + 1,
                )?;

                control = control
                    .checked_add(2)
                    .ok_or(
                        CliffordError::SizeOverflow
                    )?;
            }
        }
    }

    if circuit.len() > config.max_operations {
        return Err(
            CliffordError::OperationLimitExceeded {
                requested: circuit.len(),
                maximum: config.max_operations,
            },
        );
    }

    Ok(circuit)
}

// =============================================================================
// Canonical C₁ representation
// =============================================================================

/// Returns the canonical signed-permutation representation for one of the
/// 24 C₁ elements.
///
/// Each tuple contains:
///
///     permutation of X/Y/Z
///     signs for the three images
///
/// The ordering is stable and therefore forms part of the representation
/// contract.
fn canonical_element(
    index: usize,
) -> ([usize; 3], [i8; 3]) {
    // The 24 orientation-preserving signed permutations.
    //
    // Ordering:
    //
    // 1. positive/negative sign combinations in deterministic permutation
    //    order;
    // 2. only orientation-preserving combinations are included.
    //
    // The explicit table avoids depending on hash-map iteration or runtime
    // enumeration order.

    const ELEMENTS: [([usize; 3], [i8; 3]); 24] = [
        ([0, 1, 2], [1, 1, 1]),
        ([0, 1, 2], [-1, -1, 1]),
        ([0, 1, 2], [-1, 1, -1]),
        ([0, 1, 2], [1, -1, -1]),

        ([0, 2, 1], [1, 1, -1]),
        ([0, 2, 1], [-1, 1, 1]),
        ([0, 2, 1], [1, -1, 1]),
        ([0, 2, 1], [-1, -1, -1]),

        ([1, 0, 2], [1, -1, 1]),
        ([1, 0, 2], [-1, 1, 1]),
        ([1, 0, 2], [1, 1, -1]),
        ([1, 0, 2], [-1, -1, -1]),

        ([1, 2, 0], [1, 1, 1]),
        ([1, 2, 0], [-1, -1, 1]),
        ([1, 2, 0], [-1, 1, -1]),
        ([1, 2, 0], [1, -1, -1]),

        ([2, 0, 1], [1, 1, 1]),
        ([2, 0, 1], [-1, -1, 1]),
        ([2, 0, 1], [-1, 1, -1]),
        ([2, 0, 1], [1, -1, -1]),

        ([2, 1, 0], [1, 1, -1]),
        ([2, 1, 0], [-1, 1, 1]),
        ([2, 1, 0], [1, -1, 1]),
        ([2, 1, 0], [-1, -1, -1]),
    ];

    ELEMENTS[index]
}

/// Returns the canonical H/S decomposition for a C₁ element.
///
/// The words were generated from H and S using deterministic breadth-first
/// enumeration. The words are intentionally kept stable as part of the
/// decomposition version.
///
/// The operation order is left-to-right execution order:
///
///     [H, S]
///
/// means H followed by S.
fn canonical_decomposition(
    index: usize,
) -> &'static [CliffordPrimitive] {
    const I: &[CliffordPrimitive] = &[];

    const H: &[CliffordPrimitive] =
        &[CliffordPrimitive::H];

    const S: &[CliffordPrimitive] =
        &[CliffordPrimitive::S];

    const HS: &[CliffordPrimitive] = &[
        CliffordPrimitive::H,
        CliffordPrimitive::S,
    ];

    const SH: &[CliffordPrimitive] = &[
        CliffordPrimitive::S,
        CliffordPrimitive::H,
    ];

    const SS: &[CliffordPrimitive] = &[
        CliffordPrimitive::S,
        CliffordPrimitive::S,
    ];

    const HSH: &[CliffordPrimitive] = &[
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
    ];

    const HSS: &[CliffordPrimitive] = &[
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
    ];

    const SHS: &[CliffordPrimitive] = &[
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
    ];

    const SSH: &[CliffordPrimitive] = &[
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
    ];

    const SSS: &[CliffordPrimitive] = &[
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
    ];

    const HSHS: &[CliffordPrimitive] = &[
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
    ];

    const HSSH: &[CliffordPrimitive] = &[
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
    ];

    const HSSS: &[CliffordPrimitive] = &[
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
    ];

    const SHSS: &[CliffordPrimitive] = &[
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
    ];

    const SSHS: &[CliffordPrimitive] = &[
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
    ];

    const HSHSS: &[CliffordPrimitive] = &[
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
    ];

    const HSSHS: &[CliffordPrimitive] = &[
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
    ];

    const SHSSH: &[CliffordPrimitive] = &[
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
    ];

    const SHSSS: &[CliffordPrimitive] = &[
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
    ];

    const SSHSS: &[CliffordPrimitive] = &[
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
    ];

    const HSHSSH: &[CliffordPrimitive] = &[
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
    ];

    const HSHSSS: &[CliffordPrimitive] = &[
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
    ];

    const HSSHSS: &[CliffordPrimitive] = &[
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
        CliffordPrimitive::H,
        CliffordPrimitive::S,
        CliffordPrimitive::S,
    ];

    const ELEMENTS: [&[CliffordPrimitive]; 24] = [
        I,
        H,
        S,
        HS,
        SH,
        SS,
        HSH,
        HSS,
        SHS,
        SSH,
        SSS,
        HSHS,
        HSSH,
        HSSS,
        SHSS,
        SSHS,
        HSHSS,
        HSSHS,
        SHSSH,
        SHSSS,
        SSHSS,
        HSHSSH,
        HSHSSS,
        HSSHSS,
    ];

    ELEMENTS[index]
}

// =============================================================================
// Mathematical helpers
// =============================================================================

/// Converts a permutation/sign representation into a signed Pauli.
const fn signed_pauli_from_parts(
    axis: usize,
    sign: i8,
) -> SignedPauli {
    let pauli = match axis {
        0 => Pauli::X,
        1 => Pauli::Y,
        _ => Pauli::Z,
    };

    if sign < 0 {
        SignedPauli::negative(pauli)
    } else {
        SignedPauli::positive(pauli)
    }
}

/// Returns the parity of a three-element permutation.
///
/// +1 = even
/// -1 = odd
const fn permutation_parity(
    a: usize,
    b: usize,
    c: usize,
) -> i16 {
    let inversions =
        (if a > b { 1 } else { 0 })
            + (if a > c { 1 } else { 0 })
            + (if b > c { 1 } else { 0 });

    if inversions % 2 == 0 {
        1
    } else {
        -1
    }
}

/// Composes signed Pauli transformations.
///
/// `outer` is applied after `inner`.
const fn compose_signed_pauli(
    outer: SignedPauli,
    inner: SignedPauli,
) -> SignedPauli {
    let outer_image =
        match outer.pauli() {
            Pauli::I => SignedPauli::positive(Pauli::I),
            Pauli::X => outer,
            Pauli::Y => outer,
            Pauli::Z => outer,
        };

    let negative =
        outer_image.is_negative()
            ^ inner.is_negative();

    let pauli = outer_image.pauli();

    if negative {
        SignedPauli::negative(pauli)
    } else {
        SignedPauli::positive(pauli)
    }
}

/// Validates a requested sequence length.
fn validate_sequence_length(
    length: usize,
) -> Result<(), CliffordError> {
    if length > DEFAULT_MAX_GENERATED_OPERATIONS {
        return Err(
            CliffordError::SequenceTooLong {
                requested: length,
                maximum: DEFAULT_MAX_GENERATED_OPERATIONS,
            },
        );
    }

    Ok(())
}

/// Generates a uniformly bounded integer without modulo bias.
///
/// The implementation uses rejection sampling over the u32 range.
fn bounded_index<R>(
    rng: &mut R,
    upper: u32,
) -> u32
where
    R: RngCore + ?Sized,
{
    debug_assert!(upper > 0);

    let zone =
        u32::MAX
            - (u32::MAX % upper);

    loop {
        let value = rng.next_u32();

        if value < zone {
            return value % upper;
        }
    }
}

/// Bounded integer helper for the deterministic seeded sampler.
fn bounded_index_from_u64(
    value: u64,
    upper: u32,
) -> u32 {
    debug_assert!(upper > 0);

    let upper64 = upper as u64;

    let zone =
        u64::MAX
            - (u64::MAX % upper64);

    let candidate = value;

    if candidate < zone {
        (candidate % upper64) as u32
    } else {
        // SplitMix64 is uniformly distributed over its 64-bit output.
        // For this standalone helper we deterministically fold the rejected
        // value instead of introducing another state transition.
        let mixed =
            candidate
                .wrapping_mul(
                    0x9E37_79B9_7F4A_7C15,
                )
                .rotate_left(17);

        (mixed % upper64) as u32
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_qubit_clifford_group_has_24_elements() {
        let mut elements = Vec::new();

        for index in 0..SINGLE_QUBIT_CLIFFORD_COUNT {
            let clifford =
                SingleQubitClifford::from_index(index)
                    .expect("valid Clifford index");

            assert_eq!(
                clifford.index().expect("canonical index"),
                index
            );

            assert!(
                !elements.contains(&clifford),
                "duplicate Clifford at index {index}"
            );

            elements.push(clifford);
        }

        assert_eq!(
            elements.len(),
            SINGLE_QUBIT_CLIFFORD_COUNT
        );
    }

    #[test]
    fn identity_is_identity() {
        let identity =
            SingleQubitClifford::IDENTITY;

        assert!(identity.is_identity());

        assert_eq!(
            identity.conjugate(Pauli::X),
            SignedPauli::positive(Pauli::X)
        );

        assert_eq!(
            identity.conjugate(Pauli::Y),
            SignedPauli::positive(Pauli::Y)
        );

        assert_eq!(
            identity.conjugate(Pauli::Z),
            SignedPauli::positive(Pauli::Z)
        );
    }

    #[test]
    fn hadamard_has_correct_pauli_action() {
        let h =
            SingleQubitClifford::hadamard();

        assert_eq!(
            h.conjugate(Pauli::X),
            SignedPauli::positive(Pauli::Z)
        );

        assert_eq!(
            h.conjugate(Pauli::Y),
            SignedPauli::negative(Pauli::Y)
        );

        assert_eq!(
            h.conjugate(Pauli::Z),
            SignedPauli::positive(Pauli::X)
        );
    }

    #[test]
    fn phase_has_correct_pauli_action() {
        let s =
            SingleQubitClifford::phase();

        assert_eq!(
            s.conjugate(Pauli::X),
            SignedPauli::positive(Pauli::Y)
        );

        assert_eq!(
            s.conjugate(Pauli::Y),
            SignedPauli::negative(Pauli::X)
        );

        assert_eq!(
            s.conjugate(Pauli::Z),
            SignedPauli::positive(Pauli::Z)
        );
    }

    #[test]
    fn every_clifford_has_valid_inverse() {
        for index in 0..SINGLE_QUBIT_CLIFFORD_COUNT {
            let clifford =
                SingleQubitClifford::from_index(index)
                    .expect("valid Clifford");

            let inverse =
                clifford.inverse()
                    .expect("inverse");

            let left =
                clifford
                    .compose(inverse)
                    .expect("composition");

            let right =
                inverse
                    .compose(clifford)
                    .expect("composition");

            assert!(left.is_identity());
            assert!(right.is_identity());
        }
    }

    #[test]
    fn every_clifford_has_valid_decomposition() {
        for index in 0..SINGLE_QUBIT_CLIFFORD_COUNT {
            let clifford =
                SingleQubitClifford::from_index(index)
                    .expect("valid Clifford");

            let mut accumulated =
                SingleQubitClifford::IDENTITY;

            for primitive in clifford
                .decomposition()
                .expect("decomposition")
            {
                let primitive_clifford =
                    match primitive {
                        CliffordPrimitive::H =>
                            SingleQubitClifford::hadamard(),

                        CliffordPrimitive::S =>
                            SingleQubitClifford::phase(),

                        CliffordPrimitive::Sdg =>
                            SingleQubitClifford::phase_dagger(),
                    };

                accumulated =
                    primitive_clifford
                        .compose(accumulated)
                        .expect("composition");
            }

            assert_eq!(
                accumulated,
                clifford,
                "decomposition mismatch for Clifford {index}"
            );
        }
    }

    #[test]
    fn primitive_inverse_is_correct() {
        assert_eq!(
            CliffordPrimitive::H.inverse(),
            CliffordPrimitive::H
        );

        assert_eq!(
            CliffordPrimitive::S.inverse(),
            CliffordPrimitive::Sdg
        );

        assert_eq!(
            CliffordPrimitive::Sdg.inverse(),
            CliffordPrimitive::S
        );
    }

    #[test]
    fn primitive_sequence_inverse_reverses_order() {
        let mut sequence =
            CliffordPrimitiveSequence::new();

        sequence.push(
            CliffordOperation::new(
                0,
                CliffordPrimitive::H,
            )
        );

        sequence.push(
            CliffordOperation::new(
                1,
                CliffordPrimitive::S,
            )
        );

        let inverse =
            sequence.inverse();

        assert_eq!(
            inverse.operations(),
            &[
                CliffordOperation::new(
                    1,
                    CliffordPrimitive::Sdg,
                ),
                CliffordOperation::new(
                    0,
                    CliffordPrimitive::H,
                ),
            ]
        );
    }

    #[test]
    fn seeded_sampling_is_reproducible() {
        let mut a =
            SeededCliffordSampler::new(42);

        let mut b =
            SeededCliffordSampler::new(42);

        let sequence_a =
            a.sample_sequence(100)
                .expect("sequence A");

        let sequence_b =
            b.sample_sequence(100)
                .expect("sequence B");

        assert_eq!(
            sequence_a,
            sequence_b
        );
    }

    #[test]
    fn different_seeds_are_not_forced_equal() {
        let mut a =
            SeededCliffordSampler::new(1);

        let mut b =
            SeededCliffordSampler::new(2);

        let sequence_a =
            a.sample_sequence(32)
                .expect("sequence A");

        let sequence_b =
            b.sample_sequence(32)
                .expect("sequence B");

        assert_ne!(
            sequence_a,
            sequence_b
        );
    }

    #[test]
    fn rb_sequence_recovery_returns_identity() {
        let sequence =
            CliffordBenchmarkSequence::generate_seeded(
                12345,
                100,
            )
            .expect("RB sequence");

        sequence
            .validate_recovery()
            .expect("valid recovery");

        let accumulated =
            sequence
                .accumulated_clifford()
                .expect("accumulated");

        let recovered =
            sequence
                .recovery()
                .compose(accumulated)
                .expect("recovery composition");

        assert!(
            recovered.is_identity()
        );
    }

    #[test]
    fn empty_rb_sequence_uses_identity_recovery() {
        let sequence =
            CliffordBenchmarkSequence::generate_seeded(
                123,
                0,
            )
            .expect("empty sequence");

        assert!(sequence.is_empty());

        assert!(
            sequence.recovery().is_identity()
        );

        sequence
            .validate_recovery()
            .expect("empty sequence recovery");
    }

    #[test]
    fn primitive_rb_conversion_contains_recovery() {
        let sequence =
            CliffordBenchmarkSequence::generate_seeded(
                99,
                10,
            )
            .expect("RB sequence");

        let primitive =
            sequence
                .to_primitive_sequence(0)
                .expect("primitive sequence");

        assert!(
            !primitive.is_empty()
        );

        primitive
            .validate_for_qubits(1)
            .expect("valid qubit");
    }

    #[test]
    fn invalid_qubit_is_rejected() {
        let operation =
            MultiQubitCliffordOperation::Single {
                qubit: 4,
                primitive: CliffordPrimitive::H,
            };

        assert_eq!(
            operation.validate(4),
            Err(
                CliffordError::InvalidQubitIndex {
                    qubit: 4,
                    qubit_count: 4,
                }
            )
        );
    }

    #[test]
    fn duplicate_cx_qubit_is_rejected() {
        let operation =
            MultiQubitCliffordOperation::Cx {
                control: 2,
                target: 2,
            };

        assert_eq!(
            operation.validate(4),
            Err(
                CliffordError::DuplicateQubit {
                    qubit: 2,
                }
            )
        );
    }

    #[test]
    fn multi_qubit_inverse_is_structural_reverse_inverse() {
        let mut circuit =
            MultiQubitCliffordCircuit::new(3)
                .expect("circuit");

        circuit
            .push_clifford(
                0,
                SingleQubitClifford::hadamard(),
            )
            .expect("H");

        circuit
            .push_cx(0, 1)
            .expect("CX");

        circuit
            .push_clifford(
                2,
                SingleQubitClifford::phase(),
            )
            .expect("S");

        let inverse =
            circuit.inverse();

        assert_eq!(
            inverse.len(),
            circuit.len()
        );

        assert_eq!(
            inverse.operations()[0],
            MultiQubitCliffordOperation::Single {
                qubit: 2,
                primitive: CliffordPrimitive::Sdg,
            }
        );

        assert_eq!(
            inverse.operations()[1],
            MultiQubitCliffordOperation::Cx {
                control: 0,
                target: 1,
            }
        );
    }

    #[test]
    fn circuit_with_recovery_doubles_operations() {
        let mut circuit =
            MultiQubitCliffordCircuit::new(2)
                .expect("circuit");

        circuit
            .push_clifford(
                0,
                SingleQubitClifford::hadamard(),
            )
            .expect("H");

        circuit
            .push_cx(0, 1)
            .expect("CX");

        let recovered =
            circuit
                .with_recovery()
                .expect("recovery");

        assert_eq!(
            recovered.len(),
            circuit.len() * 2
        );
    }

    #[test]
    fn random_multi_qubit_circuit_is_reproducible() {
        let config =
            MultiQubitCliffordCircuitConfig::new(
                4,
                8,
            )
            .expect("config");

        let mut a =
            SeededCliffordSampler::new(777);

        let mut b =
            SeededCliffordSampler::new(777);

        let circuit_a =
            generate_multi_qubit_clifford_circuit(
                config,
                &mut a,
            )
            .expect("circuit A");

        let circuit_b =
            generate_multi_qubit_clifford_circuit(
                config,
                &mut b,
            )
            .expect("circuit B");

        assert_eq!(
            circuit_a,
            circuit_b
        );
    }

    #[test]
    fn multi_qubit_generator_produces_only_clifford_operations() {
        let config =
            MultiQubitCliffordCircuitConfig::new(
                4,
                4,
            )
            .expect("config");

        let mut rng =
            SeededCliffordSampler::new(123);

        let circuit =
            generate_multi_qubit_clifford_circuit(
                config,
                &mut rng,
            )
            .expect("circuit");

        for operation in circuit.operations() {
            operation
                .validate(circuit.qubit_count())
                .expect("valid operation");
        }
    }
}