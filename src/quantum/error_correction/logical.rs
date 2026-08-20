//! Zamani Quantum Error Correction — Logical Operators.
//!
//! Production-grade logical-equivalence and logical-outcome analysis for
//! stabilizer quantum error-correction codes.
//!
//! Architectural role:
//!
//! ```text
//! physical Pauli / decoder correction
//!             |
//!             v
//!     stabilizer equivalence
//!             |
//!             v
//!       normalizer test
//!             |
//!             v
//!     logical equivalence
//!             |
//!             v
//!      logical outcome
//! ```
//!
//! This module:
//!
//! - never modifies a quantum state;
//! - never applies a correction;
//! - never performs QPU I/O;
//! - uses the checked Pauli/stabilizer APIs;
//! - distinguishes physical errors from logical operators;
//! - distinguishes stabilizer-equivalent identity from logical failure;
//! - supports explicit single-logical-qubit X/Y/Z classification;
//! - supports residual-error analysis (`physical_error * correction`);
//! - provides logical equivalence testing;
//! - exposes logical-qubit count;
//! - supports explicit `QecLimits` validation;
//! - avoids unchecked indexing and panic-based validation;
//! - preserves deterministic behavior.
//!
//! Global Pauli phase is ignored because `stabilizer.rs` represents Pauli
//! operators modulo global phase.
//!
//! Important mathematical distinction:
//!
//! A decoder correction by itself is NOT necessarily a logical operator.
//! For a physical error E and decoder correction C, the physically relevant
//! residual is:
//!
//!     R = E * C
//!
//! A successful correction has R stabilizer-equivalent to identity.
//!
//! Therefore callers performing decoder-failure analysis should prefer:
//!
//!     LogicalCode::analyze_residual(error, correction, basis)
//!
//! over classifying the correction alone.

use core::fmt;
use std::collections::BTreeSet;

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
/// `Identity` is included because logical-outcome analysis must explicitly
/// represent successful/no-logical-error execution.
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
    Identity,
    X,
    Y,
    Z,
}

impl LogicalPauli {
    #[must_use]
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::Identity)
    }

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
            (Identity, p) | (p, Identity) => p,

            (X, X) | (Y, Y) | (Z, Z) => Identity,

            (X, Y) | (Y, X) => Z,

            (X, Z) | (Z, X) => Y,

            (Y, Z) | (Z, Y) => X,
        }
    }

    /// Returns whether two logical Pauli classes commute.
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

/// High-level logical execution outcome.
///
/// This is intentionally separate from `LogicalClassification`.
///
/// `LogicalClassification` answers:
///
/// > "What is this physical Pauli with respect to the stabilizer group?"
///
/// `LogicalOutcome` answers:
///
/// > "What logical result did the QEC operation produce?"
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
    /// No logical error occurred.
    Identity,

    /// A logical X error occurred.
    LogicalX,

    /// A logical Y error occurred.
    LogicalY,

    /// A logical Z error occurred.
    LogicalZ,

    /// The supplied information was insufficient to determine the logical
    /// outcome safely.
    Unknown,
}

impl LogicalOutcome {
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Identity)
    }

    #[must_use]
    pub const fn is_logical_failure(self) -> bool {
        matches!(
            self,
            Self::LogicalX
                | Self::LogicalY
                | Self::LogicalZ
        )
    }

    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

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
            Self::Identity => write!(formatter, "identity"),
            Self::LogicalX => write!(formatter, "logical-X"),
            Self::LogicalY => write!(formatter, "logical-Y"),
            Self::LogicalZ => write!(formatter, "logical-Z"),
            Self::Unknown => write!(formatter, "unknown"),
        }
    }
}

// ============================================================================
// Logical operator
// ============================================================================

/// A candidate logical Pauli operator.
///
/// Construction does not imply mathematical validity. The operator must be
/// validated against a `LogicalCode` before being treated as a logical
/// operator.
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

    #[must_use]
    pub const fn logical_pauli(
        &self,
    ) -> LogicalPauli {
        self.logical_pauli
    }

    #[must_use]
    pub fn operator(
        &self,
    ) -> &PauliString {
        &self.operator
    }

    #[must_use]
    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.operator.num_qubits()
    }

    #[must_use]
    pub fn weight(
        &self,
    ) -> usize {
        self.operator.weight()
    }

    #[must_use]
    pub fn is_identity(
        &self,
    ) -> bool {
        self.operator.is_identity()
    }

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
    ///
    /// This is a logical operator, but its X/Y/Z class requires a logical
    /// basis or equivalent logical information.
    Logical,

    /// Does not commute with the stabilizer group.
    PhysicalError,
}

impl LogicalClassification {
    #[must_use]
    pub const fn is_logical(
        self,
    ) -> bool {
        matches!(self, Self::Logical)
    }

    #[must_use]
    pub const fn is_trivial(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Identity | Self::Stabilizer
        )
    }

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

/// Detailed logical effect for a single encoded logical qubit.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum LogicalEffect {
    Identity,
    Stabilizer,
    LogicalX,
    LogicalY,
    LogicalZ,
    UncorrectablePhysicalError,
    Unknown,
}

impl LogicalEffect {
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

    #[must_use]
    pub const fn logical_pauli(
        self,
    ) -> LogicalPauli {
        match self {
            Self::Identity
            | Self::Stabilizer => LogicalPauli::Identity,

            Self::LogicalX => LogicalPauli::X,
            Self::LogicalY => LogicalPauli::Y,
            Self::LogicalZ => LogicalPauli::Z,

            Self::UncorrectablePhysicalError
            | Self::Unknown => LogicalPauli::Identity,
        }
    }

    #[must_use]
    pub const fn outcome(
        self,
    ) -> LogicalOutcome {
        match self {
            Self::Identity
            | Self::Stabilizer => LogicalOutcome::Identity,

            Self::LogicalX => LogicalOutcome::LogicalX,
            Self::LogicalY => LogicalOutcome::LogicalY,
            Self::LogicalZ => LogicalOutcome::LogicalZ,

            Self::UncorrectablePhysicalError
            | Self::Unknown => LogicalOutcome::Unknown,
        }
    }
}

// ============================================================================
// Logical code
// ============================================================================

/// Validated logical-code context.
///
/// This object is deliberately lightweight: the stabilizer group remains the
/// owner of stabilizer algebra while this module owns logical semantics.
#[derive(
    Debug,
    Clone,
)]
pub struct LogicalCode {
    stabilizers: StabilizerGroup,
}

impl LogicalCode {
    /// Creates a logical-code context after validating the stabilizer group.
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

    /// Creates a logical-code context under an explicit QEC resource policy.
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

    #[must_use]
    pub fn stabilizers(
        &self,
    ) -> &StabilizerGroup {
        &self.stabilizers
    }

    #[must_use]
    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.stabilizers.num_qubits()
    }

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

    /// Returns the number of encoded logical qubits:
    ///
    ///     k = n - rank(S)
    pub fn logical_qubit_count(
        &self,
    ) -> Result<usize, LogicalError> {
        self.stabilizers
            .logical_qubit_count()
            .map_err(LogicalError::Stabilizer)
    }

    /// Returns whether an operator commutes with every stabilizer.
    pub fn commutes_with_stabilizers(
        &self,
        operator: &PauliString,
    ) -> Result<bool, LogicalError> {
        self.check_qubit_count(operator)?;

        for generator in self.stabilizers.generators() {
            if !operator
                .commutes_with(generator.operator())
                .map_err(LogicalError::Stabilizer)?
            {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Returns whether an operator is generated by the stabilizer group.
    pub fn is_stabilizer(
        &self,
        operator: &PauliString,
    ) -> Result<bool, LogicalError> {
        self.check_qubit_count(operator)?;

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
        self.check_qubit_count(operator)?;

        self.stabilizers
            .contains_with_limits(
                operator,
                limits,
            )
            .map_err(LogicalError::Stabilizer)
    }

    /// Classifies an arbitrary physical Pauli.
    pub fn classify(
        &self,
        operator: &PauliString,
    ) -> Result<LogicalClassification, LogicalError> {
        self.check_qubit_count(operator)?;

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

        if self.commutes_with_stabilizers(operator)? {
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
        self.check_qubit_count(operator)?;

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

        for generator in self.stabilizers.generators() {
            if !operator
                .commutes_with(generator.operator())
                .map_err(LogicalError::Stabilizer)?
            {
                return Ok(
                    LogicalClassification::PhysicalError,
                );
            }
        }

        Ok(
            LogicalClassification::Logical,
        )
    }

    /// Validates a proposed logical operator.
    pub fn validate_operator(
        &self,
        logical: &LogicalOperator,
    ) -> Result<(), LogicalError> {
        self.check_qubit_count(
            logical.operator(),
        )?;

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
        let logical = LogicalOperator::new(
            logical_pauli,
            operator,
        );

        self.validate_operator(&logical)?;

        Ok(logical)
    }

    fn check_qubit_count(
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
}

// ============================================================================
// Logical basis
// ============================================================================

/// A complete X/Y/Z basis for one encoded logical qubit.
///
/// X and Z must anticommute. Y is derived as X * Z modulo global phase.
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
    pub fn new(
        code: &LogicalCode,
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

        code.validate_operator(&x)?;
        code.validate_operator(&z)?;

        let anticommutes = x
            .operator()
            .anticommutes_with(
                z.operator(),
            )
            .map_err(LogicalError::Stabilizer)?;

        if !anticommutes {
            return Err(
                LogicalError::LogicalXZHermitianMismatch,
            );
        }

        let y_operator = x
            .operator()
            .multiply(z.operator())
            .map_err(LogicalError::Stabilizer)?;

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

    #[must_use]
    pub fn x(
        &self,
    ) -> &LogicalOperator {
        &self.x
    }

    #[must_use]
    pub fn y(
        &self,
    ) -> &LogicalOperator {
        &self.y
    }

    #[must_use]
    pub fn z(
        &self,
    ) -> &LogicalOperator {
        &self.z
    }
}

// ============================================================================
// Logical analysis
// ============================================================================

impl LogicalCode {
    /// Classifies a normalizer element against a single logical-qubit basis.
    ///
    /// The supplied basis must describe one encoded logical qubit. For
    /// multi-logical-qubit codes, callers should use a complete logical basis
    /// per encoded qubit rather than assuming one X/Z pair is sufficient.
    pub fn logical_effect(
        &self,
        operator: &PauliString,
        basis: &LogicalBasis,
    ) -> Result<LogicalEffect, LogicalError> {
        self.check_qubit_count(operator)?;

        match self.classify(operator)? {
            LogicalClassification::Identity => {
                return Ok(LogicalEffect::Identity);
            }

            LogicalClassification::Stabilizer => {
                return Ok(LogicalEffect::Stabilizer);
            }

            LogicalClassification::PhysicalError => {
                return Ok(
                    LogicalEffect::UncorrectablePhysicalError,
                );
            }

            LogicalClassification::Logical => {}
        }

        let anti_x = operator
            .anticommutes_with(
                basis.x().operator(),
            )
            .map_err(LogicalError::Stabilizer)?;

        let anti_z = operator
            .anticommutes_with(
                basis.z().operator(),
            )
            .map_err(LogicalError::Stabilizer)?;

        match (anti_x, anti_z) {
            (false, false) => {
                Err(
                    LogicalError::IncompleteLogicalBasis,
                )
            }

            (false, true) => {
                Ok(LogicalEffect::LogicalX)
            }

            (true, false) => {
                Ok(LogicalEffect::LogicalZ)
            }

            (true, true) => {
                Ok(LogicalEffect::LogicalY)
            }
        }
    }

    /// Converts a logical effect into the standardized logical outcome.
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

    /// Analyzes a decoder correction against the actual physical error.
    ///
    /// The residual is:
    ///
    ///     R = E * C
    ///
    /// where E is the physical error and C is the decoder correction.
    ///
    /// This is the preferred API for determining whether decoding succeeded.
    pub fn analyze_residual(
        &self,
        physical_error: &PauliString,
        correction: &PauliString,
        basis: &LogicalBasis,
    ) -> Result<LogicalEffect, LogicalError> {
        self.check_qubit_count(
            physical_error,
        )?;

        self.check_qubit_count(
            correction,
        )?;

        let residual = physical_error
            .multiply(correction)
            .map_err(LogicalError::Stabilizer)?;

        self.logical_effect(
            &residual,
            basis,
        )
    }

    /// Returns the logical outcome of physical error + decoder correction.
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
}

// ============================================================================
// Logical distance helpers
// ============================================================================

/// Returns the minimum weight among explicitly supplied non-trivial logical
/// candidates.
///
/// This does not perform exponential Pauli-group enumeration.
pub fn minimum_logical_weight(
    code: &LogicalCode,
    candidates: &[PauliString],
) -> Result<Option<usize>, LogicalError> {
    let mut minimum = None;

    for candidate in candidates {
        code.check_candidate_dimensions(
            candidate,
        )?;

        if code.classify(candidate)?
            != LogicalClassification::Logical
        {
            continue;
        }

        let weight = candidate.weight();

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

impl LogicalCode {
    fn check_candidate_dimensions(
        &self,
        candidate: &PauliString,
    ) -> Result<(), LogicalError> {
        self.check_qubit_count(candidate)
    }
}

// ============================================================================
// Logical equivalence
// ============================================================================

/// Returns whether two physical Pauli operators implement the same logical
/// operation.
///
/// For Pauli operators modulo phase:
///
///     P ~ Q  iff  P * Q ∈ S
///
/// where S is the stabilizer group.
pub fn logically_equivalent(
    code: &LogicalCode,
    first: &PauliString,
    second: &PauliString,
) -> Result<bool, LogicalError> {
    code.check_candidate_dimensions(first)?;
    code.check_candidate_dimensions(second)?;

    let product = first
        .multiply(second)
        .map_err(LogicalError::Stabilizer)?;

    code.is_stabilizer(&product)
}

/// Resource-aware logical equivalence.
pub fn logically_equivalent_with_limits(
    code: &LogicalCode,
    first: &PauliString,
    second: &PauliString,
    limits: &QecLimits,
) -> Result<bool, LogicalError> {
    code.check_candidate_dimensions(first)?;
    code.check_candidate_dimensions(second)?;

    let product = first
        .multiply(second)
        .map_err(LogicalError::Stabilizer)?;

    code.is_stabilizer_with_limits(
        &product,
        limits,
    )
}

// ============================================================================
// Logical correction analysis
// ============================================================================

/// Analyzes a decoder correction.
///
/// Important:
///
/// A correction alone is not necessarily a logical operator. If the caller
/// has the original physical error, use `LogicalCode::analyze_residual`.
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

/// Analyzes the residual left after decoder correction.
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
// Errors
// ============================================================================

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum LogicalError {
    /// Underlying stabilizer algebra failure.
    Stabilizer(StabilizerError),

    /// Operator/code dimensional mismatch.
    QubitCountMismatch {
        expected: usize,
        actual: usize,
    },

    /// Candidate is not in the stabilizer normalizer.
    DoesNotCommuteWithStabilizers,

    /// Candidate is stabilizer-equivalent to identity.
    TrivialLogicalOperator {
        kind: LogicalPauli,
    },

    /// Logical operator was labelled incorrectly.
    WrongLogicalType {
        expected: LogicalPauli,
        actual: LogicalPauli,
    },

    /// Logical X and Z failed the required anti-commutation relation.
    LogicalXZHermitianMismatch,

    /// The supplied basis cannot classify the operator.
    IncompleteLogicalBasis,

    /// Invalid logical basis.
    InvalidLogicalBasis,

    /// Empty logical basis.
    EmptyLogicalBasis,

    /// The operation requires a complete multi-logical-qubit basis.
    MultiLogicalQubitBasisRequired {
        logical_qubits: usize,
    },
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
                write!(
                    formatter,
                    "operator does not commute with the stabilizer group"
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
                write!(
                    formatter,
                    "logical X and logical Z must anticommute"
                )
            }

            Self::IncompleteLogicalBasis => {
                write!(
                    formatter,
                    "the supplied logical basis cannot classify this logical operator"
                )
            }

            Self::InvalidLogicalBasis => {
                write!(
                    formatter,
                    "invalid logical operator basis"
                )
            }

            Self::EmptyLogicalBasis => {
                write!(
                    formatter,
                    "logical operator basis is empty"
                )
            }

            Self::MultiLogicalQubitBasisRequired {
                logical_qubits,
            } => {
                write!(
                    formatter,
                    "a complete logical basis is required for {logical_qubits} encoded logical qubits"
                )
            }
        }
    }
}

impl std::error::Error for LogicalError {}

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
/// Duplicate assignments are rejected so malformed external input cannot
/// silently overwrite an earlier operation.
pub fn pauli_operator(
    num_qubits: usize,
    assignments: &[(usize, Pauli)],
) -> Result<PauliString, LogicalError> {
    let mut operator =
        PauliString::identity(num_qubits);

    let mut seen =
        BTreeSet::new();

    for &(qubit, pauli) in assignments {
        if qubit >= num_qubits {
            return Err(
                LogicalError::QubitCountMismatch {
                    expected: num_qubits,
                    actual: qubit.saturating_add(1),
                },
            );
        }

        if !seen.insert(qubit) {
            return Err(
                LogicalError::InvalidLogicalBasis,
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
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn simple_code() -> LogicalCode {
        let mut group =
            StabilizerGroup::new(2)
                .expect("valid stabilizer group");

        group
            .add_generator(
                super::super::stabilizer::StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(&[
                        Pauli::Z,
                        Pauli::Z,
                    ]),
                )
                .expect("valid generator"),
            )
            .expect("generator accepted");

        LogicalCode::new(group)
            .expect("valid logical code")
    }

    #[test]
    fn logical_pauli_multiplication_is_closed() {
        assert_eq!(
            LogicalPauli::X.multiply(
                LogicalPauli::Z
            ),
            LogicalPauli::Y
        );

        assert_eq!(
            LogicalPauli::Y.multiply(
                LogicalPauli::Y
            ),
            LogicalPauli::Identity
        );
    }

    #[test]
    fn logical_pauli_commutation_is_correct() {
        assert!(
            LogicalPauli::X
                .commutes_with(
                    LogicalPauli::X
                )
        );

        assert!(
            !LogicalPauli::X
                .commutes_with(
                    LogicalPauli::Z
                )
        );

        assert!(
            !LogicalPauli::Y
                .commutes_with(
                    LogicalPauli::Z
                )
        );
    }

    #[test]
    fn identity_is_trivial() {
        let code = simple_code();

        let identity =
            identity_operator(2);

        assert_eq!(
            code.classify(&identity)
                .expect("classification"),
            LogicalClassification::Identity
        );
    }

    #[test]
    fn stabilizer_is_trivial_logically() {
        let code = simple_code();

        let stabilizer =
            PauliString::from_paulis(&[
                Pauli::Z,
                Pauli::Z,
            ]);

        assert_eq!(
            code.classify(&stabilizer)
                .expect("classification"),
            LogicalClassification::Stabilizer
        );
    }

    #[test]
    fn logical_equivalence_is_stabilizer_based() {
        let code = simple_code();

        let first =
            PauliString::from_paulis(&[
                Pauli::X,
                Pauli::I,
            ]);

        let second =
            PauliString::from_paulis(&[
                Pauli::I,
                Pauli::X,
            ]);

        assert!(
            logically_equivalent(
                &code,
                &first,
                &second,
            )
            .expect("equivalence")
        );
    }

    #[test]
    fn pauli_constructor_rejects_duplicate_assignments() {
        let result =
            pauli_operator(
                3,
                &[
                    (0, Pauli::X),
                    (0, Pauli::Z),
                ],
            );

        assert!(result.is_err());
    }

    #[test]
    fn pauli_constructor_rejects_out_of_range_qubits() {
        let result =
            pauli_operator(
                3,
                &[(3, Pauli::X)],
            );

        assert!(result.is_err());
    }

    #[test]
    fn dimension_mismatch_is_structured() {
        let code = simple_code();

        let operator =
            PauliString::identity(3);

        let result =
            code.classify(&operator);

        assert!(matches!(
            result,
            Err(
                LogicalError::QubitCountMismatch {
                    expected: 2,
                    actual: 3
                }
            )
        ));
    }
}