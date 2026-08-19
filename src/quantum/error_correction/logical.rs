//! Zamani Quantum Error Correction — Logical Operators.
//!
//! Production-grade logical-operator analysis for stabilizer codes.
//!
//! This module deliberately separates:
//!
//!   physical Pauli operators
//!          |
//!          v
//!   stabilizer equivalence
//!          |
//!          v
//!   logical operators
//!
//! The module does not physically modify a quantum state and does not apply
//! corrections. It provides validated representations and classification
//! utilities for logical X/Y/Z operators and decoder corrections.
//!
//! Global Pauli phase is ignored, consistently with `stabilizer.rs`.
//!
//! Important:
//! - A logical operator must commute with every stabilizer.
//! - A stabilizer itself represents the trivial logical operation.
//! - A non-stabilizer element of the normalizer represents a non-trivial
//!   logical operation.
//! - Logical X and logical Z must anticommute.
//! - Logical Y is represented by logical X * logical Z, up to global phase.
//!
//! The implementation is intentionally deterministic and avoids unchecked
//! indexing and panicking validation paths.

use std::fmt;

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

/// The three non-trivial logical Pauli operators.
///
/// `Identity` is included because classification and correction analysis need
/// an explicit representation of the trivial logical operation.
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
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::Identity)
    }

    pub const fn is_non_identity(self) -> bool {
        !self.is_identity()
    }

    /// Returns the product of two logical Paulis, ignoring global phase.
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
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let value = match self {
            Self::Identity => 'I',
            Self::X => 'X',
            Self::Y => 'Y',
            Self::Z => 'Z',
        };

        write!(f, "{value}")
    }
}

// ============================================================================
// Logical operator
// ============================================================================

/// A validated candidate logical Pauli operator.
///
/// The operator must:
///
/// 1. act on the same number of qubits as the code;
/// 2. commute with every stabilizer generator;
/// 3. have the declared logical type.
///
/// Whether it is stabilizer-equivalent to identity is recorded separately.
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
    /// Constructs a logical operator without assuming that it is valid.
    ///
    /// Use `validate` or `LogicalCode::validate_operator` before treating the
    /// value as a valid logical operator.
    pub fn new(
        logical_pauli: LogicalPauli,
        operator: PauliString,
    ) -> Self {
        Self {
            logical_pauli,
            operator,
        }
    }

    pub const fn logical_pauli(
        &self,
    ) -> LogicalPauli {
        self.logical_pauli
    }

    pub fn operator(
        &self,
    ) -> &PauliString {
        &self.operator
    }

    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.operator.num_qubits()
    }

    pub fn weight(
        &self,
    ) -> usize {
        self.operator.weight()
    }

    pub fn is_identity(
        &self,
    ) -> bool {
        self.operator.is_identity()
    }

    pub fn support(
        &self,
    ) -> Vec<QubitIndex> {
        self.operator.support()
    }
}

// ============================================================================
// Logical classification
// ============================================================================

/// Classification of a physical Pauli with respect to a stabilizer code.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum LogicalClassification {
    /// The operator is the identity.
    Identity,

    /// The operator is in the stabilizer group and therefore has trivial
    /// logical action.
    Stabilizer,

    /// The operator commutes with the stabilizers but is not a stabilizer.
    /// It therefore represents a non-trivial logical operation.
    Logical,

    /// The operator does not commute with at least one stabilizer.
    /// It is therefore not a valid logical operator.
    PhysicalError,
}

impl LogicalClassification {
    pub const fn is_logical(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Logical
        )
    }

    pub const fn is_trivial(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Identity
                | Self::Stabilizer
        )
    }

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
// Logical code
// ============================================================================

/// Validated logical-code context.
///
/// `LogicalCode` owns the stabilizer model and provides deterministic
/// validation/classification of logical operators.
///
/// The stabilizer group itself is responsible for mathematical validation of
/// generator commutation and dimensions.
#[derive(
    Debug,
    Clone,
)]
pub struct LogicalCode {
    stabilizers: StabilizerGroup,
}

impl LogicalCode {
    /// Creates a logical-code context from a validated stabilizer group.
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

    pub fn stabilizers(
        &self,
    ) -> &StabilizerGroup {
        &self.stabilizers
    }

    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.stabilizers.num_qubits()
    }

    pub fn generator_count(
        &self,
    ) -> usize {
        self.stabilizers.len()
    }

    /// Determines whether an operator commutes with every stabilizer.
    pub fn commutes_with_stabilizers(
        &self,
        operator: &PauliString,
    ) -> Result<bool, LogicalError> {
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

        for generator
            in self.stabilizers.generators()
        {
            let commutes = operator
                .commutes_with(
                    generator.operator(),
                )
                .map_err(LogicalError::Stabilizer)?;

            if !commutes {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Determines whether an operator is generated by the stabilizer group.
    ///
    /// This delegates to the stabilizer algebra rather than duplicating its
    /// GF(2) elimination implementation.
    pub fn is_stabilizer(
        &self,
        operator: &PauliString,
    ) -> Result<bool, LogicalError> {
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

        self.stabilizers
            .contains(operator)
            .map_err(LogicalError::Stabilizer)
    }

    /// Classifies an arbitrary physical Pauli.
    pub fn classify(
        &self,
        operator: &PauliString,
    ) -> Result<LogicalClassification, LogicalError> {
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

        if self.commutes_with_stabilizers(
            operator,
        )? {
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
        if logical.num_qubits()
            != self.num_qubits()
        {
            return Err(
                LogicalError::QubitCountMismatch {
                    expected: self.num_qubits(),
                    actual: logical.num_qubits(),
                },
            );
        }

        let classification =
            self.classify(
                logical.operator(),
            )?;

        match classification {
            LogicalClassification::Identity
            | LogicalClassification::Stabilizer => {
                return Err(
                    LogicalError::TrivialLogicalOperator {
                        kind: logical.logical_pauli(),
                    },
                );
            }

            LogicalClassification::PhysicalError => {
                return Err(
                    LogicalError::DoesNotCommuteWithStabilizers,
                );
            }

            LogicalClassification::Logical => {}
        }

        Ok(())
    }

    /// Creates a validated logical operator.
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

        self.validate_operator(
            &logical,
        )?;

        Ok(logical)
    }
}

// ============================================================================
// Logical basis
// ============================================================================

/// A complete single-logical-qubit Pauli basis.
///
/// The three operators must satisfy:
///
///     X * Z = Y
///
/// and
///
///     {X, Z} = 0
///
/// while each logical operator commutes with every physical stabilizer.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct LogicalBasis {
    x: LogicalOperator,
    z: LogicalOperator,
    y: LogicalOperator,
}

impl LogicalBasis {
    /// Builds a logical basis and verifies all algebraic invariants.
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
                .multiply(
                    z.operator(),
                )
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
            z,
            y,
        })
    }

    pub fn x(
        &self,
    ) -> &LogicalOperator {
        &self.x
    }

    pub fn y(
        &self,
    ) -> &LogicalOperator {
        &self.y
    }

    pub fn z(
        &self,
    ) -> &LogicalOperator {
        &self.z
    }
}

// ============================================================================
// Logical error analysis
// ============================================================================

/// Result of comparing a physical correction/error against the logical code.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub enum LogicalEffect {
    /// No physical operation remains after stabilizer equivalence.
    Identity,

    /// The physical operation is a stabilizer and therefore has no logical
    /// effect.
    Stabilizer,

    /// The operation implements logical X.
    LogicalX,

    /// The operation implements logical Y.
    LogicalY,

    /// The operation implements logical Z.
    LogicalZ,

    /// The operation is not in the stabilizer normalizer and therefore cannot
    /// be interpreted as a logical operation.
    UncorrectablePhysicalError,
}

impl LogicalEffect {
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

    pub const fn logical_pauli(
        self,
    ) -> LogicalPauli {
        match self {
            Self::Identity
            | Self::Stabilizer => {
                LogicalPauli::Identity
            }

            Self::LogicalX => {
                LogicalPauli::X
            }

            Self::LogicalY => {
                LogicalPauli::Y
            }

            Self::LogicalZ => {
                LogicalPauli::Z
            }

            Self::UncorrectablePhysicalError => {
                LogicalPauli::Identity
            }
        }
    }
}

// ============================================================================
// Logical analysis
// ============================================================================

impl LogicalCode {
    /// Determines the logical effect of a physical operator when a logical
    /// basis is available.
    ///
    /// This uses commutation with logical X/Z:
    ///
    ///                 Xc  Zc
    /// identity         +   +
    /// logical X        +   -
    /// logical Z        -   +
    /// logical Y        -   -
    ///
    /// where `+` means commute and `-` means anticommute.
    pub fn logical_effect(
        &self,
        operator: &PauliString,
        basis: &LogicalBasis,
    ) -> Result<LogicalEffect, LogicalError> {
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

        let classification =
            self.classify(operator)?;

        match classification {
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
            (false, false) => {
                // A non-stabilizer normalizer element that commutes with
                // both logical generators cannot be represented by the
                // supplied single-qubit logical basis.
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
}

// ============================================================================
// Logical operator distance
// ============================================================================

/// Searches a bounded set of explicitly supplied operators and returns the
/// minimum weight non-trivial logical operator.
///
/// This function intentionally does NOT brute-force the entire Pauli group.
/// Exhaustive enumeration scales as 4^n and is unsuitable for production
/// code-distance measurement.
///
/// A production benchmark should provide candidate logical operators generated
/// by the surface-code geometry or a dedicated distance algorithm.
pub fn minimum_logical_weight(
    code: &LogicalCode,
    candidates: &[PauliString],
) -> Result<Option<usize>, LogicalError> {
    let mut minimum: Option<usize> = None;

    for candidate in candidates {
        if candidate.num_qubits()
            != code.num_qubits()
        {
            return Err(
                LogicalError::QubitCountMismatch {
                    expected: code.num_qubits(),
                    actual: candidate.num_qubits(),
                },
            );
        }

        if code.classify(candidate)?
            != LogicalClassification::Logical
        {
            continue;
        }

        let weight =
            candidate.weight();

        minimum = Some(
            minimum
                .map_or(
                    weight,
                    |current| current.min(weight),
                ),
        );
    }

    Ok(minimum)
}

// ============================================================================
// Logical equivalence
// ============================================================================

/// Returns whether two physical Pauli operators implement the same logical
/// operation.
///
/// Two operators are logically equivalent iff their product is a stabilizer.
pub fn logically_equivalent(
    code: &LogicalCode,
    first: &PauliString,
    second: &PauliString,
) -> Result<bool, LogicalError> {
    if first.num_qubits()
        != code.num_qubits()
    {
        return Err(
            LogicalError::QubitCountMismatch {
                expected: code.num_qubits(),
                actual: first.num_qubits(),
            },
        );
    }

    if second.num_qubits()
        != code.num_qubits()
    {
        return Err(
            LogicalError::QubitCountMismatch {
                expected: code.num_qubits(),
                actual: second.num_qubits(),
            },
        );
    }

    let product =
        first
            .multiply(second)
            .map_err(
                LogicalError::Stabilizer,
            )?;

    code.is_stabilizer(
        &product,
    )
}

// ============================================================================
// Logical correction analysis
// ============================================================================

/// Analyzes a correction produced by a decoder.
///
/// This is deliberately independent of the decoder implementation. MWPM,
/// Union-Find, or another decoder can all pass their resulting Pauli frame
/// operator through this function.
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
    Stabilizer(
        StabilizerError,
    ),

    QubitCountMismatch {
        expected: usize,
        actual: usize,
    },

    DoesNotCommuteWithStabilizers,

    TrivialLogicalOperator {
        kind: LogicalPauli,
    },

    WrongLogicalType {
        expected: LogicalPauli,
        actual: LogicalPauli,
    },

    LogicalXZHermitianMismatch,

    IncompleteLogicalBasis,

    InvalidLogicalBasis,

    EmptyLogicalBasis,
}

impl fmt::Display for LogicalError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Stabilizer(error) => {
                write!(
                    f,
                    "stabilizer error: {error}"
                )
            }

            Self::QubitCountMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "logical operator acts on {actual} qubits, expected {expected}"
                )
            }

            Self::DoesNotCommuteWithStabilizers => {
                write!(
                    f,
                    "operator does not commute with the stabilizer group"
                )
            }

            Self::TrivialLogicalOperator {
                kind,
            } => {
                write!(
                    f,
                    "logical {kind} operator is stabilizer-equivalent to identity"
                )
            }

            Self::WrongLogicalType {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "wrong logical operator type: expected {expected}, got {actual}"
                )
            }

            Self::LogicalXZHermitianMismatch => {
                write!(
                    f,
                    "logical X and logical Z must anticommute"
                )
            }

            Self::IncompleteLogicalBasis => {
                write!(
                    f,
                    "the supplied logical basis cannot classify this logical operator"
                )
            }

            Self::InvalidLogicalBasis => {
                write!(
                    f,
                    "invalid logical operator basis"
                )
            }

            Self::EmptyLogicalBasis => {
                write!(
                    f,
                    "logical operator basis is empty"
                )
            }
        }
    }
}

impl std::error::Error for LogicalError {}

// ============================================================================
// Constructors for explicit single-qubit logical candidates
// ============================================================================

/// Constructs an n-qubit identity candidate.
pub fn identity_operator(
    num_qubits: usize,
) -> PauliString {
    PauliString::identity(
        num_qubits,
    )
}

/// Constructs a Pauli string from `(qubit, Pauli)` assignments.
///
/// Duplicate qubit assignments are rejected instead of silently overwriting
/// an earlier assignment. This makes malformed external input deterministic.
pub fn pauli_operator(
    num_qubits: usize,
    assignments: &[(usize, Pauli)],
) -> Result<PauliString, LogicalError> {
    let mut operator =
        PauliString::identity(
            num_qubits,
        );

    let mut seen =
        std::collections::BTreeSet::new();

    for &(qubit, pauli)
        in assignments
    {
        if qubit >= num_qubits {
            return Err(
                LogicalError::QubitCountMismatch {
                    expected: num_qubits,
                    actual: qubit
                        .saturating_add(1),
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

    #[test]
    fn logical_pauli_multiplication_is_closed() {
        assert_eq!(
            LogicalPauli::X
                .multiply(
                    LogicalPauli::Z
                ),
            LogicalPauli::Y
        );

        assert_eq!(
            LogicalPauli::Y
                .multiply(
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
    fn identity_is_not_a_nontrivial_logical_operator() {
        let operator =
            identity_operator(3);

        assert!(
            operator.is_identity()
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

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn pauli_constructor_rejects_out_of_range_qubits() {
        let result =
            pauli_operator(
                3,
                &[
                    (3, Pauli::X),
                ],
            );

        assert!(
            result.is_err()
        );
    }
}