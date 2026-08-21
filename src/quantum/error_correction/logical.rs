//! Zamani Quantum Error Correction — Logical Operators.
//!
//! Production-grade logical-operator, logical-equivalence, and
//! logical-outcome analysis for stabilizer quantum error-correction codes.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - logical Pauli classification;
//! - logical operator validation;
//! - logical X/Y/Z basis validation;
//! - stabilizer equivalence;
//! - logical equivalence;
//! - residual-error analysis;
//! - logical outcome classification;
//! - logical-code metadata;
//! - explicit logical-basis validation;
//! - logical-weight helpers;
//! - conversion of local errors into the canonical `QecError` boundary.
//!
//! This module does NOT own:
//!
//! - surface-code topology;
//! - stabilizer algebra;
//! - syndrome storage;
//! - decoder algorithms;
//! - MWPM;
//! - Union-Find;
//! - Pauli-frame mutation;
//! - QPU execution;
//! - streaming;
//! - distributed execution;
//! - checkpoint persistence;
//! - telemetry transport;
//! - capability authorization.
//!
//! Those responsibilities belong to their respective QEC modules.
//!
//! # Mathematical model
//!
//! Let `S` be the stabilizer group.
//!
//! For a physical Pauli `P`:
//!
//! ```text
//! P ∈ S                  => stabilizer-equivalent to identity
//! P ∈ N(S) \ S          => non-trivial logical operator
//! P ∉ N(S)              => detectable physical error
//! ```
//!
//! where `N(S)` is the stabilizer normalizer/centralizer.
//!
//! For a physical error `E` and decoder correction `C`, the residual error is:
//!
//! ```text
//! R = E * C
//! ```
//!
//! Successful error correction requires:
//!
//! ```text
//! R ∈ S
//! ```
//!
//! A residual in `N(S) \ S` represents a logical error.
//!
//! # Important distinction
//!
//! A decoder correction by itself is NOT necessarily a logical operator.
//! Callers evaluating decoder correctness should prefer:
//!
//! ```text
//! LogicalCode::analyze_residual(error, correction, basis)
//! ```
//!
//! rather than classifying the correction alone.
//!
//! # Multi-logical-qubit safety
//!
//! A single `LogicalBasis` represents exactly one encoded logical qubit.
//!
//! Multi-logical-qubit codes must use `LogicalBasisSet`. This prevents a
//! logical operator acting on another encoded qubit from being incorrectly
//! classified using an unrelated X/Z pair.
//!
//! # Phase convention
//!
//! `stabilizer.rs` intentionally represents Pauli operators modulo global
//! phase. This module therefore also ignores global Pauli phase.
//!
//! # Integration
//!
//! ```text
//! errors.rs
//!     ▲
//!     │
//! logical.rs
//!     ▲
//!     │
//! stabilizer.rs
//!     │
//!     ├── surface_code.rs
//!     ├── distance.rs
//!     ├── decoder.rs
//!     ├── pauli_frame.rs
//!     └── decoding_graph.rs
//! ```
//!
//! `logical.rs` consumes the checked algebra exposed by `stabilizer.rs`.
//! It does not duplicate stabilizer mathematics.
//!
//! # Rust compatibility
//!
//! Targets Rust 1.97.1 using stable standard-library facilities.

use core::fmt;
use std::collections::BTreeSet;

use super::errors::QecError;
use super::limits::QecLimits;
use super::stabilizer::{
    Pauli,
    PauliString,
    QubitIndex,
    StabilizerError,
    StabilizerGroup,
};

// ============================================================================
// Logical Pauli
// ============================================================================

/// Logical Pauli class.
///
/// Global phase is ignored.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum LogicalPauli {
    /// Logical identity.
    Identity,

    /// Logical X.
    X,

    /// Logical Y.
    Y,

    /// Logical Z.
    Z,
}

impl LogicalPauli {
    /// Returns true for identity.
    #[must_use]
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::Identity)
    }

    /// Returns true for X, Y, or Z.
    #[must_use]
    pub const fn is_non_identity(self) -> bool {
        !self.is_identity()
    }

    /// Multiplies logical Paulis modulo global phase.
    #[must_use]
    pub const fn multiply(
        self,
        other: Self,
    ) -> Self {
        use LogicalPauli::*;

        match (self, other) {
            (Identity, value)
            | (value, Identity) => value,

            (X, X)
            | (Y, Y)
            | (Z, Z) => Identity,

            (X, Y)
            | (Y, X) => Z,

            (X, Z)
            | (Z, X) => Y,

            (Y, Z)
            | (Z, Y) => X,
        }
    }

    /// Returns whether two logical Paulis commute.
    #[must_use]
    pub const fn commutes_with(
        self,
        other: Self,
    ) -> bool {
        !matches!(
            (self, other),
            (X, Y)
                | (Y, X)
                | (X, Z)
                | (Z, X)
                | (Y, Z)
                | (Z, Y)
        )
    }
}

impl fmt::Display for LogicalPauli {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let symbol = match self {
            Self::Identity => 'I',
            Self::X => 'X',
            Self::Y => 'Y',
            Self::Z => 'Z',
        };

        write!(formatter, "{symbol}")
    }
}

// ============================================================================
// Logical outcome
// ============================================================================

/// Logical execution outcome.
///
/// `Identity` and stabilizer-equivalent residuals are successful logical
/// outcomes. `Unknown` means that the available information was insufficient
/// to make a safe logical classification.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum LogicalOutcome {
    /// No logical error.
    Identity,

    /// Logical X error.
    LogicalX,

    /// Logical Y error.
    LogicalY,

    /// Logical Z error.
    LogicalZ,

    /// Safe classification was impossible.
    Unknown,
}

impl LogicalOutcome {
    /// Returns true when decoding produced no logical error.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Identity)
    }

    /// Returns true when a logical X/Y/Z error occurred.
    #[must_use]
    pub const fn is_logical_failure(self) -> bool {
        matches!(
            self,
            Self::LogicalX
                | Self::LogicalY
                | Self::LogicalZ
        )
    }

    /// Returns true when classification is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Converts the outcome into a logical Pauli.
    ///
    /// `Unknown` deliberately maps to identity only as a representation
    /// fallback. Callers must check `is_unknown()` before using this value
    /// semantically.
    #[must_use]
    pub const fn logical_pauli(self) -> LogicalPauli {
        match self {
            Self::Identity => LogicalPauli::Identity,
            Self::LogicalX => LogicalPauli::X,
            Self::LogicalY => LogicalPauli::Y,
            Self::LogicalZ => LogicalPauli::Z,
            Self::Unknown => LogicalPauli::Identity,
        }
    }
}

impl fmt::Display for LogicalOutcome {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Identity => formatter.write_str("identity"),
            Self::LogicalX => formatter.write_str("logical-X"),
            Self::LogicalY => formatter.write_str("logical-Y"),
            Self::LogicalZ => formatter.write_str("logical-Z"),
            Self::Unknown => formatter.write_str("unknown"),
        }
    }
}

// ============================================================================
// Logical operator
// ============================================================================

/// A candidate logical Pauli operator.
///
/// Construction alone does not prove that the operator is a valid logical
/// operator. It must be validated against a `LogicalCode`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct LogicalOperator {
    logical_pauli: LogicalPauli,
    operator: PauliString,
}

impl LogicalOperator {
    /// Creates a candidate logical operator.
    #[must_use]
    pub fn new(
        logical_pauli: LogicalPauli,
        operator: PauliString,
    ) -> Self {
        Self {
            logical_pauli,
            operator,
        }
    }

    /// Returns the declared logical type.
    #[must_use]
    pub const fn logical_pauli(
        &self,
    ) -> LogicalPauli {
        self.logical_pauli
    }

    /// Returns the underlying physical Pauli string.
    #[must_use]
    pub fn operator(
        &self,
    ) -> &PauliString {
        &self.operator
    }

    /// Returns the number of physical qubits.
    #[must_use]
    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.operator.num_qubits()
    }

    /// Returns the physical Pauli weight.
    #[must_use]
    pub fn weight(
        &self,
    ) -> usize {
        self.operator.weight()
    }

    /// Returns whether the underlying operator is identity.
    #[must_use]
    pub fn is_identity(
        &self,
    ) -> bool {
        self.operator.is_identity()
    }

    /// Returns the physical support.
    #[must_use]
    pub fn support(
        &self,
    ) -> Vec<QubitIndex> {
        self.operator.support()
    }
}

// ============================================================================
// Logical classification
// ============================================================================

/// Classification of a physical Pauli relative to a stabilizer code.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum LogicalClassification {
    /// Exact identity.
    Identity,

    /// Stabilizer-equivalent to identity.
    Stabilizer,

    /// Non-trivial element of the stabilizer normalizer.
    Logical,

    /// Does not commute with the stabilizer group.
    PhysicalError,
}

impl LogicalClassification {
    /// Returns true when the operator is a non-trivial logical operator.
    #[must_use]
    pub const fn is_logical(
        self,
    ) -> bool {
        matches!(self, Self::Logical)
    }

    /// Returns true for identity or stabilizer-equivalent identity.
    #[must_use]
    pub const fn is_trivial(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Identity | Self::Stabilizer
        )
    }

    /// Returns true when the operator belongs to the normalizer.
    #[must_use]
    pub const fn is_valid_normalizer_element(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Identity
                | Self::Stabilizer
                | Self::Logical
        )
    }
}

// ============================================================================
// Logical effect
// ============================================================================

/// Detailed logical effect.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum LogicalEffect {
    /// Exact identity.
    Identity,

    /// Non-identity stabilizer.
    Stabilizer,

    /// Logical X.
    LogicalX,

    /// Logical Y.
    LogicalY,

    /// Logical Z.
    LogicalZ,

    /// Physical error outside the stabilizer normalizer.
    UncorrectablePhysicalError,

    /// Safe classification was unavailable.
    Unknown,
}

impl LogicalEffect {
    /// Returns true for logical X/Y/Z.
    #[must_use]
    pub const fn is_logical_error(
        self,
    ) -> bool {
        matches!(
            self,
            Self::LogicalX
                | Self::LogicalY
                | Self::LogicalZ
        )
    }

    /// Returns the corresponding logical Pauli.
    #[must_use]
    pub const fn logical_pauli(
        self,
    ) -> LogicalPauli {
        match self {
            Self::Identity
            | Self::Stabilizer => {
                LogicalPauli::Identity
            }

            Self::LogicalX => LogicalPauli::X,
            Self::LogicalY => LogicalPauli::Y,
            Self::LogicalZ => LogicalPauli::Z,

            Self::UncorrectablePhysicalError
            | Self::Unknown => {
                LogicalPauli::Identity
            }
        }
    }

    /// Converts the effect to a logical outcome.
    #[must_use]
    pub const fn outcome(
        self,
    ) -> LogicalOutcome {
        match self {
            Self::Identity
            | Self::Stabilizer => {
                LogicalOutcome::Identity
            }

            Self::LogicalX => LogicalOutcome::LogicalX,
            Self::LogicalY => LogicalOutcome::LogicalY,
            Self::LogicalZ => LogicalOutcome::LogicalZ,

            Self::UncorrectablePhysicalError
            | Self::Unknown => {
                LogicalOutcome::Unknown
            }
        }
    }
}

// ============================================================================
// Logical basis
// ============================================================================

/// A complete X/Z/Y basis for exactly one encoded logical qubit.
///
/// The logical Y operator is derived from X * Z modulo global phase.
///
/// This type deliberately rejects codes encoding anything other than exactly
/// one logical qubit. Multi-logical-qubit codes must use `LogicalBasisSet`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct LogicalBasis {
    x: LogicalOperator,
    y: LogicalOperator,
    z: LogicalOperator,
}

impl LogicalBasis {
    /// Creates and validates a single-logical-qubit basis.
    pub fn new(
        code: &LogicalCode,
        x: LogicalOperator,
        z: LogicalOperator,
    ) -> Result<Self, LogicalError> {
        let logical_qubits =
            code.logical_qubit_count()?;

        if logical_qubits != 1 {
            return Err(
                LogicalError::MultiLogicalQubitBasisRequired {
                    logical_qubits,
                },
            );
        }

        if x.logical_pauli()
            != LogicalPauli::X
        {
            return Err(
                LogicalError::WrongLogicalType {
                    expected: LogicalPauli::X,
                    actual: x.logical_pauli(),
                },
            );
        }

        if z.logical_pauli()
            != LogicalPauli::Z
        {
            return Err(
                LogicalError::WrongLogicalType {
                    expected: LogicalPauli::Z,
                    actual: z.logical_pauli(),
                },
            );
        }

        code.validate_operator(&x)?;
        code.validate_operator(&z)?;

        let anticommutes =
            x.operator()
                .anticommutes_with(
                    z.operator(),
                )
                .map_err(
                    LogicalError::Stabilizer,
                )?;

        if !anticommutes {
            return Err(
                LogicalError::LogicalXZHermitianMismatch,
            );
        }

        let y_operator =
            x.operator()
                .multiply(z.operator())
                .map_err(
                    LogicalError::Stabilizer,
                )?;

        let y = LogicalOperator::new(
            LogicalPauli::Y,
            y_operator,
        );

        code.validate_operator(&y)?;

        Ok(Self {
            x,
            y,
            z,
        })
    }

    /// Returns logical X.
    #[must_use]
    pub fn x(
        &self,
    ) -> &LogicalOperator {
        &self.x
    }

    /// Returns logical Y.
    #[must_use]
    pub fn y(
        &self,
    ) -> &LogicalOperator {
        &self.y
    }

    /// Returns logical Z.
    #[must_use]
    pub fn z(
        &self,
    ) -> &LogicalOperator {
        &self.z
    }
}

// ============================================================================
// Multi-logical-qubit basis
// ============================================================================

/// Logical X/Z basis pair for one encoded logical qubit.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct LogicalBasisPair {
    index: usize,
    x: LogicalOperator,
    z: LogicalOperator,
}

impl LogicalBasisPair {
    /// Creates a logical basis pair.
    pub fn new(
        index: usize,
        x: LogicalOperator,
        z: LogicalOperator,
    ) -> Result<Self, LogicalError> {
        if x.logical_pauli()
            != LogicalPauli::X
        {
            return Err(
                LogicalError::WrongLogicalType {
                    expected: LogicalPauli::X,
                    actual: x.logical_pauli(),
                },
            );
        }

        if z.logical_pauli()
            != LogicalPauli::Z
        {
            return Err(
                LogicalError::WrongLogicalType {
                    expected: LogicalPauli::Z,
                    actual: z.logical_pauli(),
                },
            );
        }

        Ok(Self {
            index,
            x,
            z,
        })
    }

    /// Returns the encoded logical-qubit index.
    #[must_use]
    pub const fn index(
        &self,
    ) -> usize {
        self.index
    }

    /// Returns logical X.
    #[must_use]
    pub fn x(
        &self,
    ) -> &LogicalOperator {
        &self.x
    }

    /// Returns logical Z.
    #[must_use]
    pub fn z(
        &self,
    ) -> &LogicalOperator {
        &self.z
    }
}

/// Complete logical basis for a stabilizer code.
///
/// Every encoded logical qubit must have one X/Z pair.
///
/// The pairs are stored in deterministic ascending logical-qubit order.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct LogicalBasisSet {
    pairs: Vec<LogicalBasisPair>,
}

impl LogicalBasisSet {
    /// Creates an empty basis set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            pairs: Vec::new(),
        }
    }

    /// Creates a basis set from all logical X/Z pairs.
    pub fn from_pairs(
        code: &LogicalCode,
        mut pairs: Vec<LogicalBasisPair>,
    ) -> Result<Self, LogicalError> {
        let expected =
            code.logical_qubit_count()?;

        if expected == 0 {
            return Err(
                LogicalError::NoEncodedLogicalQubits,
            );
        }

        pairs.sort_by_key(
            LogicalBasisPair::index,
        );

        let mut seen =
            BTreeSet::new();

        for pair in &pairs {
            if !seen.insert(pair.index()) {
                return Err(
                    LogicalError::DuplicateLogicalQubit {
                        index: pair.index(),
                    },
                );
            }

            if pair.x().num_qubits()
                != code.num_qubits()
            {
                return Err(
                    LogicalError::QubitCountMismatch {
                        expected: code.num_qubits(),
                        actual: pair.x().num_qubits(),
                    },
                );
            }

            if pair.z().num_qubits()
                != code.num_qubits()
            {
                return Err(
                    LogicalError::QubitCountMismatch {
                        expected: code.num_qubits(),
                        actual: pair.z().num_qubits(),
                    },
                );
            }

            code.validate_operator(
                pair.x(),
            )?;

            code.validate_operator(
                pair.z(),
            )?;

            let anticommutes =
                pair.x()
                    .operator()
                    .anticommutes_with(
                        pair.z().operator(),
                    )
                    .map_err(
                        LogicalError::Stabilizer,
                    )?;

            if !anticommutes {
                return Err(
                    LogicalError::LogicalXZHermitianMismatch,
                );
            }
        }

        if pairs.len() != expected {
            return Err(
                LogicalError::IncompleteLogicalBasis {
                    expected,
                    actual: pairs.len(),
                },
            );
        }

        for index in 0..expected {
            if pairs[index].index()
                != index
            {
                return Err(
                    LogicalError::IncompleteLogicalBasis {
                        expected,
                        actual: pairs.len(),
                    },
                );
            }
        }

        // Logical operators belonging to different encoded qubits must
        // commute. This validates the supplied logical coordinate system.
        for first in 0..pairs.len() {
            for second in
                (first + 1)..pairs.len()
            {
                let first_pair =
                    &pairs[first];

                let second_pair =
                    &pairs[second];

                let xx =
                    first_pair
                        .x()
                        .operator()
                        .anticommutes_with(
                            second_pair
                                .x()
                                .operator(),
                        )
                        .map_err(
                            LogicalError::Stabilizer,
                        )?;

                let xz =
                    first_pair
                        .x()
                        .operator()
                        .anticommutes_with(
                            second_pair
                                .z()
                                .operator(),
                        )
                        .map_err(
                            LogicalError::Stabilizer,
                        )?;

                let zx =
                    first_pair
                        .z()
                        .operator()
                        .anticommutes_with(
                            second_pair
                                .x()
                                .operator(),
                        )
                        .map_err(
                            LogicalError::Stabilizer,
                        )?;

                let zz =
                    first_pair
                        .z()
                        .operator()
                        .anticommutes_with(
                            second_pair
                                .z()
                                .operator(),
                        )
                        .map_err(
                            LogicalError::Stabilizer,
                        )?;

                if xx || xz || zx || zz {
                    return Err(
                        LogicalError::CrossLogicalAnticommutation {
                            first: first_pair.index(),
                            second: second_pair.index(),
                        },
                    );
                }
            }
        }

        Ok(Self { pairs })
    }

    /// Returns the number of encoded logical qubits represented.
    #[must_use]
    pub fn len(
        &self,
    ) -> usize {
        self.pairs.len()
    }

    /// Returns true when no logical basis exists.
    #[must_use]
    pub fn is_empty(
        &self,
    ) -> bool {
        self.pairs.is_empty()
    }

    /// Returns all basis pairs in deterministic order.
    #[must_use]
    pub fn pairs(
        &self,
    ) -> &[LogicalBasisPair] {
        &self.pairs
    }

    /// Returns a basis pair by logical-qubit index.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&LogicalBasisPair> {
        self.pairs.get(index)
    }
}

impl Default for LogicalBasisSet {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Logical code
// ============================================================================

/// Validated logical-code context.
///
/// Stabilizer algebra remains owned by `StabilizerGroup`.
#[derive(
    Debug,
    Clone,
)]
pub struct LogicalCode {
    stabilizers: StabilizerGroup,
}

impl LogicalCode {
    /// Creates a logical-code context.
    pub fn new(
        stabilizers: StabilizerGroup,
    ) -> Result<Self, LogicalError> {
        stabilizers
            .validate()
            .map_err(LogicalError::Stabilizer)?;

        Ok(Self {
            stabilizers,
        })
    }

    /// Creates a logical-code context under explicit QEC limits.
    pub fn new_with_limits(
        stabilizers: StabilizerGroup,
        limits: &QecLimits,
    ) -> Result<Self, LogicalError> {
        stabilizers
            .validate_with_limits(limits)
            .map_err(LogicalError::Stabilizer)?;

        Ok(Self {
            stabilizers,
        })
    }

    /// Returns the underlying stabilizer group.
    #[must_use]
    pub fn stabilizers(
        &self,
    ) -> &StabilizerGroup {
        &self.stabilizers
    }

    /// Returns the physical-qubit count.
    #[must_use]
    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.stabilizers.num_qubits()
    }

    /// Returns the number of stabilizer generators.
    #[must_use]
    pub fn generator_count(
        &self,
    ) -> usize {
        self.stabilizers.len()
    }

    /// Returns the stabilizer rank.
    pub fn stabilizer_rank(
        &self,
    ) -> Result<usize, LogicalError> {
        self.stabilizers
            .rank()
            .map_err(LogicalError::Stabilizer)
    }

    /// Returns the number of encoded logical qubits.
    ///
    /// ```text
    /// k = n - rank(S)
    /// ```
    pub fn logical_qubit_count(
        &self,
    ) -> Result<usize, LogicalError> {
        self.stabilizers
            .logical_qubit_count()
            .map_err(LogicalError::Stabilizer)
    }

    /// Validates an operator's physical-qubit dimension.
    pub fn validate_dimensions(
        &self,
        operator: &PauliString,
    ) -> Result<(), LogicalError> {
        if operator.num_qubits()
            != self.num_qubits()
        {
            return Err(
                LogicalError::QubitCountMismatch {
                    expected: self.num_qubits(),
                    actual: operator.num_qubits(),
                },
            );
        }

        Ok(())
    }

    /// Returns whether an operator commutes with all stabilizers.
    pub fn commutes_with_stabilizers(
        &self,
        operator: &PauliString,
    ) -> Result<bool, LogicalError> {
        self.validate_dimensions(operator)?;

        self.stabilizers
            .is_in_normalizer(operator)
            .map_err(LogicalError::Stabilizer)
    }

    /// Returns whether an operator belongs to the stabilizer group.
    pub fn is_stabilizer(
        &self,
        operator: &PauliString,
    ) -> Result<bool, LogicalError> {
        self.validate_dimensions(operator)?;

        self.stabilizers
            .contains(operator)
            .map_err(LogicalError::Stabilizer)
    }

    /// Resource-aware stabilizer membership.
    pub fn is_stabilizer_with_limits(
        &self,
        operator: &PauliString,
        limits: &QecLimits,
    ) -> Result<bool, LogicalError> {
        self.validate_dimensions(operator)?;

        self.stabilizers
            .contains_with_limits(
                operator,
                limits,
            )
            .map_err(LogicalError::Stabilizer)
    }

    /// Classifies a physical Pauli relative to the code.
    pub fn classify(
        &self,
        operator: &PauliString,
    ) -> Result<LogicalClassification, LogicalError> {
        self.validate_dimensions(operator)?;

        if operator.is_identity() {
            return Ok(
                LogicalClassification::Identity,
            );
        }

        if self.is_stabilizer(operator)? {
            return Ok(
                LogicalClassification::Stabilizer,
            );
        }

        if self
            .commutes_with_stabilizers(operator)?
        {
            return Ok(
                LogicalClassification::Logical,
            );
        }

        Ok(
            LogicalClassification::PhysicalError,
        )
    }

    /// Resource-aware classification.
    pub fn classify_with_limits(
        &self,
        operator: &PauliString,
        limits: &QecLimits,
    ) -> Result<LogicalClassification, LogicalError> {
        self.validate_dimensions(operator)?;

        self.stabilizers
            .validate_with_limits(limits)
            .map_err(LogicalError::Stabilizer)?;

        if operator.is_identity() {
            return Ok(
                LogicalClassification::Identity,
            );
        }

        if self
            .stabilizers
            .contains_with_limits(
                operator,
                limits,
            )
            .map_err(LogicalError::Stabilizer)?
        {
            return Ok(
                LogicalClassification::Stabilizer,
            );
        }

        if self
            .stabilizers
            .is_in_normalizer(operator)
            .map_err(LogicalError::Stabilizer)?
        {
            return Ok(
                LogicalClassification::Logical,
            );
        }

        Ok(
            LogicalClassification::PhysicalError,
        )
    }

    /// Validates a proposed logical operator.
    pub fn validate_operator(
        &self,
        logical: &LogicalOperator,
    ) -> Result<(), LogicalError> {
        self.validate_dimensions(
            logical.operator(),
        )?;

        if logical.logical_pauli()
            == LogicalPauli::Identity
        {
            return Err(
                LogicalError::WrongLogicalType {
                    expected: LogicalPauli::X,
                    actual: LogicalPauli::Identity,
                },
            );
        }

        match self.classify(
            logical.operator(),
        )? {
            LogicalClassification::Logical => Ok(()),

            LogicalClassification::Identity
            | LogicalClassification::Stabilizer => {
                Err(
                    LogicalError::TrivialLogicalOperator {
                        kind: logical.logical_pauli(),
                    },
                )
            }

            LogicalClassification::PhysicalError => {
                Err(
                    LogicalError::DoesNotCommuteWithStabilizers,
                )
            }
        }
    }

    /// Creates and validates a logical operator.
    pub fn logical_operator(
        &self,
        logical_pauli: LogicalPauli,
        operator: PauliString,
    ) -> Result<LogicalOperator, LogicalError> {
        let logical =
            LogicalOperator::new(
                logical_pauli,
                operator,
            );

        self.validate_operator(&logical)?;

        Ok(logical)
    }

    /// Classifies a logical operator against a validated single-qubit basis.
    pub fn logical_effect(
        &self,
        operator: &PauliString,
        basis: &LogicalBasis,
    ) -> Result<LogicalEffect, LogicalError> {
        self.validate_dimensions(operator)?;

        match self.classify(operator)? {
            LogicalClassification::Identity => {
                return Ok(
                    LogicalEffect::Identity,
                );
            }

            LogicalClassification::Stabilizer => {
                return Ok(
                    LogicalEffect::Stabilizer,
                );
            }

            LogicalClassification::PhysicalError => {
                return Ok(
                    LogicalEffect::UncorrectablePhysicalError,
                );
            }

            LogicalClassification::Logical => {}
        }

        let anti_x =
            operator
                .anticommutes_with(
                    basis.x().operator(),
                )
                .map_err(
                    LogicalError::Stabilizer,
                )?;

        let anti_z =
            operator
                .anticommutes_with(
                    basis.z().operator(),
                )
                .map_err(
                    LogicalError::Stabilizer,
                )?;

        match (anti_x, anti_z) {
            (false, true) => {
                Ok(LogicalEffect::LogicalX)
            }

            (true, false) => {
                Ok(LogicalEffect::LogicalZ)
            }

            (true, true) => {
                Ok(LogicalEffect::LogicalY)
            }

            (false, false) => Err(
                LogicalError::IncompleteLogicalBasis {
                    expected: 1,
                    actual: 0,
                },
            ),
        }
    }

    /// Converts a single-qubit logical effect to a logical outcome.
    pub fn logical_outcome(
        &self,
        operator: &PauliString,
        basis: &LogicalBasis,
    ) -> Result<LogicalOutcome, LogicalError> {
        Ok(
            self.logical_effect(
                operator,
                basis,
            )?
            .outcome(),
        )
    }

    /// Analyzes the residual left by a decoder.
    ///
    /// ```text
    /// residual = physical_error * correction
    /// ```
    pub fn analyze_residual(
        &self,
        physical_error: &PauliString,
        correction: &PauliString,
        basis: &LogicalBasis,
    ) -> Result<LogicalEffect, LogicalError> {
        self.validate_dimensions(
            physical_error,
        )?;

        self.validate_dimensions(
            correction,
        )?;

        let residual =
            physical_error
                .multiply(correction)
                .map_err(
                    LogicalError::Stabilizer,
                )?;

        self.logical_effect(
            &residual,
            basis,
        )
    }

    /// Returns the residual logical outcome.
    pub fn residual_outcome(
        &self,
        physical_error: &PauliString,
        correction: &PauliString,
        basis: &LogicalBasis,
    ) -> Result<LogicalOutcome, LogicalError> {
        Ok(
            self.analyze_residual(
                physical_error,
                correction,
                basis,
            )?
            .outcome(),
        )
    }

    /// Classifies a residual using a complete multi-logical-qubit basis.
    ///
    /// This method verifies that every encoded logical qubit is accounted for.
    ///
    /// The returned vector contains one logical Pauli per encoded logical
    /// qubit, in ascending logical-qubit order.
    pub fn logical_pauli_vector(
        &self,
        operator: &PauliString,
        basis: &LogicalBasisSet,
    ) -> Result<Vec<LogicalPauli>, LogicalError> {
        self.validate_dimensions(operator)?;

        let expected =
            self.logical_qubit_count()?;

        if basis.len() != expected {
            return Err(
                LogicalError::IncompleteLogicalBasis {
                    expected,
                    actual: basis.len(),
                },
            );
        }

        match self.classify(operator)? {
            LogicalClassification::Identity => {
                return Ok(vec![
                    LogicalPauli::Identity;
                    expected
                ]);
            }

            LogicalClassification::Stabilizer => {
                return Ok(vec![
                    LogicalPauli::Identity;
                    expected
                ]);
            }

            LogicalClassification::PhysicalError => {
                return Err(
                    LogicalError::UncorrectablePhysicalError,
                );
            }

            LogicalClassification::Logical => {}
        }

        let mut result =
            Vec::new();

        result
            .try_reserve(expected)
            .map_err(|_| {
                LogicalError::AllocationFailure
            })?;

        for pair in basis.pairs() {
            let anti_x =
                operator
                    .anticommutes_with(
                        pair.x().operator(),
                    )
                    .map_err(
                        LogicalError::Stabilizer,
                    )?;

            let anti_z =
                operator
                    .anticommutes_with(
                        pair.z().operator(),
                    )
                    .map_err(
                        LogicalError::Stabilizer,
                    )?;

            let logical = match (
                anti_x,
                anti_z,
            ) {
                (false, false) => {
                    LogicalPauli::Identity
                }

                (false, true) => {
                    LogicalPauli::X
                }

                (true, false) => {
                    LogicalPauli::Z
                }

                (true, true) => {
                    LogicalPauli::Y
                }
            };

            result.push(logical);
        }

        Ok(result)
    }

    /// Determines whether two physical Paulis are logically equivalent.
    ///
    /// ```text
    /// P ~ Q  iff  P * Q ∈ S
    /// ```
    pub fn logically_equivalent(
        &self,
        first: &PauliString,
        second: &PauliString,
    ) -> Result<bool, LogicalError> {
        self.validate_dimensions(first)?;
        self.validate_dimensions(second)?;

        let product =
            first
                .multiply(second)
                .map_err(
                    LogicalError::Stabilizer,
                )?;

        self.is_stabilizer(&product)
    }

    /// Resource-aware logical equivalence.
    pub fn logically_equivalent_with_limits(
        &self,
        first: &PauliString,
        second: &PauliString,
        limits: &QecLimits,
    ) -> Result<bool, LogicalError> {
        self.validate_dimensions(first)?;
        self.validate_dimensions(second)?;

        let product =
            first
                .multiply(second)
                .map_err(
                    LogicalError::Stabilizer,
                )?;

        self.is_stabilizer_with_limits(
            &product,
            limits,
        )
    }
}

// ============================================================================
// Logical-weight helpers
// ============================================================================

/// Returns the minimum physical weight among supplied non-trivial logical
/// candidates.
///
/// This function does NOT enumerate the full Pauli group.
pub fn minimum_logical_weight(
    code: &LogicalCode,
    candidates: &[PauliString],
) -> Result<Option<usize>, LogicalError> {
    let mut minimum =
        None;

    for candidate in candidates {
        code.validate_dimensions(
            candidate,
        )?;

        if code.classify(candidate)?
            != LogicalClassification::Logical
        {
            continue;
        }

        let weight =
            candidate.weight();

        minimum = Some(
            minimum.map_or(
                weight,
                |current: usize| {
                    current.min(weight)
                },
            ),
        );
    }

    Ok(minimum)
}

// ============================================================================
// Free-function compatibility API
// ============================================================================

/// Returns whether two physical Pauli operators implement the same logical
/// operation.
pub fn logically_equivalent(
    code: &LogicalCode,
    first: &PauliString,
    second: &PauliString,
) -> Result<bool, LogicalError> {
    code.logically_equivalent(
        first,
        second,
    )
}

/// Resource-aware logical equivalence.
pub fn logically_equivalent_with_limits(
    code: &LogicalCode,
    first: &PauliString,
    second: &PauliString,
    limits: &QecLimits,
) -> Result<bool, LogicalError> {
    code.logically_equivalent_with_limits(
        first,
        second,
        limits,
    )
}

/// Analyzes a decoder correction.
///
/// For decoder correctness, prefer `analyze_decoder_result` when the original
/// physical error is available.
pub fn analyze_correction(
    code: &LogicalCode,
    correction: &PauliString,
    basis: &LogicalBasis,
) -> Result<LogicalEffect, LogicalError> {
    code.logical_effect(
        correction,
        basis,
    )
}

/// Analyzes the residual produced by a decoder.
pub fn analyze_decoder_result(
    code: &LogicalCode,
    physical_error: &PauliString,
    correction: &PauliString,
    basis: &LogicalBasis,
) -> Result<LogicalOutcome, LogicalError> {
    code.residual_outcome(
        physical_error,
        correction,
        basis,
    )
}

// ============================================================================
// Constructors
// ============================================================================

/// Constructs an n-qubit identity operator.
#[must_use]
pub fn identity_operator(
    num_qubits: usize,
) -> PauliString {
    PauliString::identity(
        num_qubits,
    )
}

/// Constructs a Pauli string from `(qubit, Pauli)` assignments.
///
/// Duplicate assignments are rejected rather than silently overwriting an
/// earlier assignment.
pub fn pauli_operator(
    num_qubits: usize,
    assignments: &[(usize, Pauli)],
) -> Result<PauliString, LogicalError> {
    let mut operator =
        PauliString::identity(
            num_qubits,
        );

    let mut seen =
        BTreeSet::new();

    for &(qubit, pauli) in assignments {
        if qubit >= num_qubits {
            return Err(
                LogicalError::QubitIndexOutOfRange {
                    qubit,
                    num_qubits,
                },
            );
        }

        if !seen.insert(qubit) {
            return Err(
                LogicalError::DuplicateQubitAssignment {
                    qubit,
                },
            );
        }

        operator
            .set_pauli(
                QubitIndex::new(qubit),
                pauli,
            )
            .map_err(
                LogicalError::Stabilizer,
            )?;
    }

    Ok(operator)
}

// ============================================================================
// Canonical QecError integration
// ============================================================================

impl LogicalError {
    /// Converts this local error into the canonical QEC error boundary.
    ///
    /// This preserves the architectural rule that `logical.rs` owns detailed
    /// diagnostics while `errors.rs` owns the public subsystem error model.
    #[must_use]
    pub fn into_qec_error(
        self,
    ) -> QecError {
        match self {
            Self::Stabilizer(error) => {
                QecError::invalid_stabilizer(
                    error.to_string(),
                )
            }

            Self::QubitCountMismatch {
                expected,
                actual,
            } => QecError::invalid_input(
                format!(
                    "logical operator acts on {actual} qubits; expected {expected}"
                ),
            ),

            Self::DoesNotCommuteWithStabilizers => {
                QecError::invalid_stabilizer(
                    "operator does not commute with the stabilizer group",
                )
            }

            Self::TrivialLogicalOperator {
                kind,
            } => QecError::invalid_stabilizer(
                format!(
                    "logical {kind} operator is stabilizer-equivalent to identity"
                ),
            ),

            Self::WrongLogicalType {
                expected,
                actual,
            } => QecError::invalid_input(
                format!(
                    "wrong logical operator type: expected {expected}, got {actual}"
                ),
            ),

            Self::LogicalXZHermitianMismatch => {
                QecError::invalid_stabilizer(
                    "logical X and logical Z must anticommute",
                )
            }

            Self::IncompleteLogicalBasis {
                expected,
                actual,
            } => QecError::invalid_input(
                format!(
                    "incomplete logical basis: expected {expected} pairs, got {actual}"
                ),
            ),

            Self::MultiLogicalQubitBasisRequired {
                logical_qubits,
            } => QecError::unsupported(
                "single-logical-qubit basis",
                format!(
                    "the code contains {logical_qubits} encoded logical qubits; use LogicalBasisSet"
                ),
            ),

            Self::InvalidLogicalBasis => {
                QecError::invalid_input(
                    "invalid logical operator basis",
                )
            }

            Self::NoEncodedLogicalQubits => {
                QecError::invalid_stabilizer(
                    "stabilizer code contains no encoded logical qubits",
                )
            }

            Self::DuplicateLogicalQubit {
                index,
            } => QecError::invalid_input(
                format!(
                    "duplicate logical-qubit basis index {index}"
                ),
            ),

            Self::CrossLogicalAnticommutation {
                first,
                second,
            } => QecError::invalid_stabilizer(
                format!(
                    "logical basis pairs {first} and {second} must commute across encoded qubits"
                ),
            ),

            Self::AllocationFailure => {
                QecError::resource_limit(
                    super::errors::ResourceKind::Allocations,
                    1,
                    0,
                    0,
                    "logical basis allocation failed",
                )
            }

            Self::QubitIndexOutOfRange {
                qubit,
                num_qubits,
            } => QecError::invalid_input(
                format!(
                    "qubit index {qubit} is outside {num_qubits} physical qubits"
                ),
            ),

            Self::DuplicateQubitAssignment {
                qubit,
            } => QecError::invalid_input(
                format!(
                    "qubit {qubit} was assigned more than once"
                ),
            },

            Self::LogicalBasisNotAssociatedWithCode => {
                QecError::invalid_stabilizer(
                    "logical basis is not associated with the supplied code",
                )
            }
        }
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Detailed logical-module error.
///
/// Higher-level APIs can convert this to `QecError` using
/// `LogicalError::into_qec_error()`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum LogicalError {
    /// Underlying stabilizer algebra error.
    Stabilizer(StabilizerError),

    /// Physical-qubit dimension mismatch.
    QubitCountMismatch {
        expected: usize,
        actual: usize,
    },

    /// Operator does not commute with the stabilizer group.
    DoesNotCommuteWithStabilizers,

    /// Candidate is stabilizer-equivalent to identity.
    TrivialLogicalOperator {
        kind: LogicalPauli,
    },

    /// Logical operator was declared with the wrong type.
    WrongLogicalType {
        expected: LogicalPauli,
        actual: LogicalPauli,
    },

    /// Logical X and Z do not anticommute.
    LogicalXZHermitianMismatch,

    /// Logical basis is incomplete.
    IncompleteLogicalBasis {
        expected: usize,
        actual: usize,
    },

    /// A single-logical-qubit basis was requested for a multi-logical code.
    MultiLogicalQubitBasisRequired {
        logical_qubits: usize,
    },

    /// Invalid logical basis.
    InvalidLogicalBasis,

    /// Code contains no encoded logical qubits.
    NoEncodedLogicalQubits,

    /// Duplicate logical-qubit basis index.
    DuplicateLogicalQubit {
        index: usize,
    },

    /// Distinct logical-qubit basis pairs anticommute.
    CrossLogicalAnticommutation {
        first: usize,
        second: usize,
    },

    /// A bounded allocation failed.
    AllocationFailure,

    /// Pauli assignment references an invalid qubit.
    QubitIndexOutOfRange {
        qubit: usize,
        num_qubits: usize,
    },

    /// A physical qubit was assigned more than once.
    DuplicateQubitAssignment {
        qubit: usize,
    },

    /// Reserved for future basis/code association checks.
    LogicalBasisNotAssociatedWithCode,
}

impl fmt::Display for LogicalError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Stabilizer(error) => {
                write!(
                    formatter,
                    "stabilizer error: {error}"
                )
            }

            Self::QubitCountMismatch {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "logical operator acts on {actual} qubits; expected {expected}"
                )
            }

            Self::DoesNotCommuteWithStabilizers => {
                formatter.write_str(
                    "operator does not commute with the stabilizer group",
                )
            }

            Self::TrivialLogicalOperator {
                kind,
            } => {
                write!(
                    formatter,
                    "logical {kind} operator is stabilizer-equivalent to identity"
                )
            }

            Self::WrongLogicalType {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "wrong logical operator type: expected {expected}, got {actual}"
                )
            }

            Self::LogicalXZHermitianMismatch => {
                formatter.write_str(
                    "logical X and logical Z must anticommute",
                )
            }

            Self::IncompleteLogicalBasis {
                expected,
                actual,
            } => {
                write!(
                    formatter,
                    "incomplete logical basis: expected {expected} pairs, got {actual}"
                )
            }

            Self::MultiLogicalQubitBasisRequired {
                logical_qubits,
            } => {
                write!(
                    formatter,
                    "a single-logical-qubit basis is invalid for a code with {logical_qubits} encoded logical qubits"
                )
            }

            Self::InvalidLogicalBasis => {
                formatter.write_str(
                    "invalid logical operator basis",
                )
            }

            Self::NoEncodedLogicalQubits => {
                formatter.write_str(
                    "the stabilizer code contains no encoded logical qubits",
                )
            }

            Self::DuplicateLogicalQubit {
                index,
            } => {
                write!(
                    formatter,
                    "duplicate logical-qubit basis index {index}"
                )
            }

            Self::CrossLogicalAnticommutation {
                first,
                second,
            } => {
                write!(
                    formatter,
                    "logical basis pairs {first} and {second} must commute across encoded logical qubits"
                )
            }

            Self::AllocationFailure => {
                formatter.write_str(
                    "logical basis allocation failed",
                )
            }

            Self::QubitIndexOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    formatter,
                    "qubit index {qubit} is outside {num_qubits} physical qubits"
                )
            }

            Self::DuplicateQubitAssignment {
                qubit,
            } => {
                write!(
                    formatter,
                    "qubit {qubit} was assigned more than once"
                )
            }

            Self::LogicalBasisNotAssociatedWithCode => {
                formatter.write_str(
                    "logical basis is not associated with the supplied code",
                )
            }
        }
    }
}

impl std::error::Error for LogicalError {}

impl From<StabilizerError> for LogicalError {
    fn from(
        error: StabilizerError,
    ) -> Self {
        Self::Stabilizer(error)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::stabilizer::{
        StabilizerGenerator,
    };

    fn simple_code() -> LogicalCode {
        let mut group =
            StabilizerGroup::new(2)
                .expect(
                    "valid stabilizer group",
                );

        let generator =
            StabilizerGenerator::new(
                0,
                PauliString::from_paulis(
                    &[
                        Pauli::Z,
                        Pauli::Z,
                    ],
                ),
            )
            .expect(
                "valid stabilizer generator",
            );

        group
            .add_generator(generator)
            .expect(
                "generator accepted",
            );

        LogicalCode::new(group)
            .expect(
                "valid logical code",
            )
    }

    fn logical_x() -> PauliString {
        PauliString::from_paulis(
            &[
                Pauli::X,
                Pauli::I,
            ],
        )
    }

    fn logical_z() -> PauliString {
        PauliString::from_paulis(
            &[
                Pauli::Z,
                Pauli::I,
            ],
        )
    }

    #[test]
    fn logical_pauli_multiplication_is_closed() {
        assert_eq!(
            LogicalPauli::X.multiply(
                LogicalPauli::Z,
            ),
            LogicalPauli::Y,
        );

        assert_eq!(
            LogicalPauli::Y.multiply(
                LogicalPauli::Y,
            ),
            LogicalPauli::Identity,
        );
    }

    #[test]
    fn logical_pauli_commutation_is_correct() {
        assert!(
            LogicalPauli::X
                .commutes_with(
                    LogicalPauli::X,
                )
        );

        assert!(
            !LogicalPauli::X
                .commutes_with(
                    LogicalPauli::Z,
                )
        );
    }

    #[test]
    fn identity_is_trivial() {
        let code =
            simple_code();

        let identity =
            identity_operator(2);

        assert_eq!(
            code.classify(&identity)
                .expect(
                    "classification",
                ),
            LogicalClassification::Identity,
        );
    }

    #[test]
    fn stabilizer_is_trivial_logically() {
        let code =
            simple_code();

        let stabilizer =
            PauliString::from_paulis(
                &[
                    Pauli::Z,
                    Pauli::Z,
                ],
            );

        assert_eq!(
            code.classify(&stabilizer)
                .expect(
                    "classification",
                ),
            LogicalClassification::Stabilizer,
        );
    }

    #[test]
    fn_non_stabilizer_normalizer_is_logical() {
        let code =
            simple_code();

        assert_eq!(
            code.classify(
                &logical_x(),
            )
            .expect(
                "classification",
            ),
            LogicalClassification::Logical,
        );
    }

    #[test]
    fn logical_basis_validates() {
        let code =
            simple_code();

        let x =
            code.logical_operator(
                LogicalPauli::X,
                logical_x(),
            )
            .expect(
                "logical X",
            );

        let z =
            code.logical_operator(
                LogicalPauli::Z,
                logical_z(),
            )
            .expect(
                "logical Z",
            );

        let basis =
            LogicalBasis::new(
                &code,
                x,
                z,
            )
            .expect(
                "valid logical basis",
            );

        assert_eq!(
            basis.y()
                .logical_pauli(),
            LogicalPauli::Y,
        );
    }

    #[test]
    fn logical_x_is_classified_correctly() {
        let code =
            simple_code();

        let x =
            code.logical_operator(
                LogicalPauli::X,
                logical_x(),
            )
            .expect(
                "logical X",
            );

        let z =
            code.logical_operator(
                LogicalPauli::Z,
                logical_z(),
            )
            .expect(
                "logical Z",
            );

        let basis =
            LogicalBasis::new(
                &code,
                x,
                z,
            )
            .expect(
                "basis",
            );

        assert_eq!(
            code.logical_effect(
                &logical_x(),
                &basis,
            )
            .expect(
                "logical effect",
            ),
            LogicalEffect::LogicalX,
        );
    }

    #[test]
    fn residual_identity_is_success() {
        let code =
            simple_code();

        let x =
            logical_x();

        let correction =
            logical_x();

        let logical_x_operator =
            code.logical_operator(
                LogicalPauli::X,
                logical_x(),
            )
            .expect(
                "logical X",
            );

        let logical_z_operator =
            code.logical_operator(
                LogicalPauli::Z,
                logical_z(),
            )
            .expect(
                "logical Z",
            );

        let basis =
            LogicalBasis::new(
                &code,
                logical_x_operator,
                logical_z_operator,
            )
            .expect(
                "basis",
            );

        assert_eq!(
            code.residual_outcome(
                &x,
                &correction,
                &basis,
            )
            .expect(
                "residual outcome",
            ),
            LogicalOutcome::Identity,
        );
    }

    #[test]
    fn logical_equivalence_uses_stabilizer_cosets() {
        let code =
            simple_code();

        let identity =
            identity_operator(2);

        let stabilizer =
            PauliString::from_paulis(
                &[
                    Pauli::Z,
                    Pauli::Z,
                ],
            );

        assert!(
            code.logically_equivalent(
                &identity,
                &stabilizer,
            )
            .expect(
                "logical equivalence",
            )
        );
    }

    #[test]
    fn malformed_pauli_assignment_is_rejected() {
        let result =
            pauli_operator(
                2,
                &[
                    (0, Pauli::X),
                    (0, Pauli::Z),
                ],
            );

        assert!(
            matches!(
                result,
                Err(
                    LogicalError::DuplicateQubitAssignment {
                        qubit: 0,
                    },
                )
            )
        );
    }

    #[test]
    fn out_of_range_assignment_is_rejected() {
        let result =
            pauli_operator(
                2,
                &[
                    (2, Pauli::X),
                ],
            );

        assert!(
            matches!(
                result,
                Err(
                    LogicalError::QubitIndexOutOfRange {
                        qubit: 2,
                        num_qubits: 2,
                    },
                )
            )
        );
    }
}