//! Zamani Quantum Error Correction — Code Distance.
//!
//! Computes and verifies the distance of a stabilizer code.
//!
//! For a stabilizer code, the distance is the minimum weight of a Pauli
//! operator that:
//!
//! 1. commutes with every stabilizer; and
//! 2. is not itself an element of the stabilizer group.
//!
//! In other words:
//!
//!     d = min wt(P)
//!         where P ∈ N(S) \ S
//!
//! where:
//!
//!     S = stabilizer group
//!     N(S) = normaliser of S
//!
//! This implementation provides an exact exhaustive search for small codes.
//! It is intended primarily for correctness validation and testing.
//!
//! Larger production codes should use specialised algorithms rather than
//! exhaustive enumeration.

use std::fmt;

use super::stabilizer::{
    Pauli,
    PauliString,
    StabilizerError,
    StabilizerGroup,
};

// -----------------------------------------------------------------------------
// Distance result
// -----------------------------------------------------------------------------

/// Exact result of a code-distance calculation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct DistanceResult {
    distance: usize,
    logical_operator: PauliString,
}

impl DistanceResult {
    pub fn new(
        distance: usize,
        logical_operator: PauliString,
    ) -> Result<Self, DistanceError> {
        if distance == 0 {
            return Err(
                DistanceError::InvalidDistance,
            );
        }

        if logical_operator.is_identity() {
            return Err(
                DistanceError::IdentityLogicalOperator,
            );
        }

        if logical_operator.weight()
            != distance
        {
            return Err(
                DistanceError::WeightMismatch {
                    expected: distance,
                    actual:
                        logical_operator.weight(),
                },
            );
        }

        Ok(Self {
            distance,
            logical_operator,
        })
    }

    pub const fn distance(
        &self,
    ) -> usize {
        self.distance
    }

    pub fn logical_operator(
        &self,
    ) -> &PauliString {
        &self.logical_operator
    }
}

// -----------------------------------------------------------------------------
// Distance calculator
// -----------------------------------------------------------------------------

/// Exact distance calculator.
///
/// This enumerates all non-identity Pauli operators and searches by
/// increasing weight. The first normaliser element that is not a stabiliser
/// determines the exact code distance.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
)]
pub struct ExactDistanceCalculator;

impl ExactDistanceCalculator {
    pub const fn new() -> Self {
        Self
    }

    /// Computes the exact distance of a stabilizer code.
    pub fn calculate(
        &self,
        stabilizers: &StabilizerGroup,
    ) -> Result<DistanceResult, DistanceError> {
        let n =
            stabilizers.num_qubits();

        if n == 0 {
            return Err(
                DistanceError::EmptyCode,
            );
        }

        // Search from weight 1 upwards.
        for weight in 1..=n {
            if let Some(operator) =
                self.search_weight(
                    stabilizers,
                    weight,
                )?
            {
                return DistanceResult::new(
                    weight,
                    operator,
                );
            }
        }

        Err(
            DistanceError::NoLogicalOperator,
        )
    }

    /// Verifies a claimed code distance.
    pub fn verify(
        &self,
        stabilizers: &StabilizerGroup,
        claimed_distance: usize,
    ) -> Result<DistanceResult, DistanceError> {
        let actual =
            self.calculate(stabilizers)?;

        if actual.distance()
            != claimed_distance
        {
            return Err(
                DistanceError::DistanceMismatch {
                    claimed:
                        claimed_distance,
                    actual:
                        actual.distance(),
                },
            );
        }

        Ok(actual)
    }

    fn search_weight(
        &self,
        stabilizers: &StabilizerGroup,
        weight: usize,
    ) -> Result<Option<PauliString>, DistanceError> {
        let n =
            stabilizers.num_qubits();

        let mut current =
            vec![Pauli::I; n];

        self.search_recursive(
            stabilizers,
            weight,
            0,
            &mut current,
        )
    }

    fn search_recursive(
        &self,
        stabilizers: &StabilizerGroup,
        target_weight: usize,
        position: usize,
        current: &mut [Pauli],
    ) -> Result<Option<PauliString>, DistanceError> {
        let n =
            current.len();

        if position == n {
            let actual_weight =
                current
                    .iter()
                    .filter(|p| **p != Pauli::I)
                    .count();

            if actual_weight
                != target_weight
            {
                return Ok(None);
            }

            let candidate =
                PauliString::from_paulis(
                    current,
                );

            if candidate.is_identity() {
                return Ok(None);
            }

            // Candidate must commute with every stabilizer.
            let syndrome =
                stabilizers
                    .syndrome(&candidate)
                    .map_err(
                        DistanceError::Stabilizer,
                    )?;

            if !syndrome.is_trivial() {
                return Ok(None);
            }

            // Candidate must not be a stabilizer.
            if stabilizers
                .contains(&candidate)
                .map_err(
                    DistanceError::Stabilizer,
                )?
            {
                return Ok(None);
            }

            return Ok(Some(candidate));
        }

        // Prune if there are not enough remaining positions to reach the
        // target weight.
        let remaining =
            n - position;

        let current_weight =
            current
                .iter()
                .take(position)
                .filter(|p| **p != Pauli::I)
                .count();

        if current_weight
            > target_weight
        {
            return Ok(None);
        }

        if current_weight
            + remaining
            < target_weight
        {
            return Ok(None);
        }

        // Identity branch.
        current[position] =
            Pauli::I;

        if let Some(result) =
            self.search_recursive(
                stabilizers,
                target_weight,
                position + 1,
                current,
            )?
        {
            return Ok(Some(result));
        }

        // X branch.
        current[position] =
            Pauli::X;

        if let Some(result) =
            self.search_recursive(
                stabilizers,
                target_weight,
                position + 1,
                current,
            )?
        {
            return Ok(Some(result));
        }

        // Y branch.
        current[position] =
            Pauli::Y;

        if let Some(result) =
            self.search_recursive(
                stabilizers,
                target_weight,
                position + 1,
                current,
            )?
        {
            return Ok(Some(result));
        }

        // Z branch.
        current[position] =
            Pauli::Z;

        if let Some(result) =
            self.search_recursive(
                stabilizers,
                target_weight,
                position + 1,
                current,
            )?
        {
            return Ok(Some(result));
        }

        current[position] =
            Pauli::I;

        Ok(None)
    }
}

// -----------------------------------------------------------------------------
// Convenience API
// -----------------------------------------------------------------------------

/// Calculates the exact distance of a stabilizer code.
pub fn calculate_distance(
    stabilizers: &StabilizerGroup,
) -> Result<DistanceResult, DistanceError> {
    ExactDistanceCalculator::new()
        .calculate(stabilizers)
}

/// Verifies the exact distance against a claimed value.
pub fn verify_distance(
    stabilizers: &StabilizerGroup,
    claimed_distance: usize,
) -> Result<DistanceResult, DistanceError> {
    ExactDistanceCalculator::new()
        .verify(
            stabilizers,
            claimed_distance,
        )
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum DistanceError {
    EmptyCode,

    InvalidDistance,

    IdentityLogicalOperator,

    WeightMismatch {
        expected: usize,
        actual: usize,
    },

    DistanceMismatch {
        claimed: usize,
        actual: usize,
    },

    NoLogicalOperator,

    Stabilizer(
        StabilizerError,
    ),
}

impl fmt::Display
    for DistanceError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyCode => {
                write!(
                    f,
                    "cannot calculate distance of an empty code"
                )
            }

            Self::InvalidDistance => {
                write!(
                    f,
                    "distance must be greater than zero"
                )
            }

            Self::IdentityLogicalOperator => {
                write!(
                    f,
                    "identity cannot be a logical operator"
                )
            }

            Self::WeightMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "distance/operator weight mismatch: expected {expected}, got {actual}"
                )
            }

            Self::DistanceMismatch {
                claimed,
                actual,
            } => {
                write!(
                    f,
                    "claimed code distance {claimed} does not match exact distance {actual}"
                )
            }

            Self::NoLogicalOperator => {
                write!(
                    f,
                    "no non-stabilizer logical operator was found"
                )
            }

            Self::Stabilizer(error) => {
                write!(
                    f,
                    "stabilizer error: {error}"
                )
            }
        }
    }
}

impl std::error::Error
    for DistanceError
{
}

impl From<StabilizerError>
    for DistanceError
{
    fn from(
        error: StabilizerError,
    ) -> Self {
        Self::Stabilizer(error)
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn repetition_code() -> StabilizerGroup {
        let mut group =
            StabilizerGroup::new(3)
                .unwrap();

        group
            .add_generator(
                super::super::stabilizer::
                    StabilizerGenerator::new(
                        0,
                        PauliString::from_paulis(
                            &[
                                Pauli::Z,
                                Pauli::Z,
                                Pauli::I,
                            ],
                        ),
                    )
                    .unwrap(),
            )
            .unwrap();

        group
            .add_generator(
                super::super::stabilizer::
                    StabilizerGenerator::new(
                        1,
                        PauliString::from_paulis(
                            &[
                                Pauli::I,
                                Pauli::Z,
                                Pauli::Z,
                            ],
                        ),
                    )
                    .unwrap(),
            )
            .unwrap();

        group
    }

    #[test]
    fn finds_repetition_code_distance() {
        let group =
            repetition_code();

        let result =
            calculate_distance(
                &group,
            )
            .unwrap();

        assert_eq!(
            result.distance(),
            1
        );
    }

    #[test]
    fn rejects_wrong_claimed_distance() {
        let group =
            repetition_code();

        let result =
            verify_distance(
                &group,
                3,
            );

        assert!(matches!(
            result,
            Err(
                DistanceError::DistanceMismatch {
                    claimed: 3,
                    actual: 1
                }
            )
        ));
    }

    #[test]
    fn found_operator_has_minimum_weight() {
        let group =
            repetition_code();

        let result =
            calculate_distance(
                &group,
            )
            .unwrap();

        assert_eq!(
            result.logical_operator()
                .weight(),
            result.distance()
        );

        assert!(
            !result
                .logical_operator()
                .is_identity()
        );
    }
}