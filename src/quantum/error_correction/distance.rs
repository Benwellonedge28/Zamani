//! Zamani Quantum Error Correction — Code Distance.
//!
//! Computes and validates the distance of a stabilizer code.
//!
//! For a stabilizer code with stabilizer group S, the distance is
//!
//!     d = min wt(P)
//!
//! over all Pauli operators P that:
//!
//!   1. commute with every stabilizer, and
//!   2. are not themselves members of S.
//!
//! Such operators are non-trivial logical operators.
//!
//! This implementation performs an exact search by increasing Pauli weight.
//! It is intended primarily for validation and small/medium codes.
//!
//! The stabilizer membership test itself uses GF(2) linear algebra through
//! `StabilizerGroup::contains()`.

use super::stabilizer::{
    commutes_with_stabilizer_group,
    Pauli,
    PauliString,
    StabilizerError,
    StabilizerGroup,
};

// ============================================================================
// Distance result
// ============================================================================

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct CodeDistance {
    distance: usize,
    logical_operator: PauliString,
}

impl CodeDistance {
    pub fn new(
        distance: usize,
        logical_operator: PauliString,
    ) -> Result<Self, DistanceError> {
        if distance == 0 {
            return Err(
                DistanceError::InvalidDistance {
                    distance,
                },
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
                DistanceError::DistanceWeightMismatch {
                    distance,
                    weight:
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

// ============================================================================
// Distance calculation
// ============================================================================

/// Computes the exact stabilizer-code distance.
///
/// The search starts at weight 1 and stops at the first non-trivial logical
/// operator found.
///
/// Returns:
///
///     d = minimum weight of a Pauli in N(S) \ S
///
/// where N(S) is the normalizer of the stabilizer group.
pub fn compute_distance(
    stabilizers: &StabilizerGroup,
) -> Result<CodeDistance, DistanceError> {
    stabilizers.validate()?;

    if stabilizers.is_empty() {
        return Err(
            DistanceError::NoStabilizerGenerators,
        );
    }

    let num_qubits =
        stabilizers.num_qubits();

    // Search by increasing weight. The first logical operator encountered
    // therefore defines the exact code distance.
    for weight in
        1..=num_qubits
    {
        if let Some(operator) =
            find_logical_operator_of_weight(
                stabilizers,
                weight,
            )?
        {
            return CodeDistance::new(
                weight,
                operator,
            );
        }
    }

    Err(
        DistanceError::NoLogicalOperatorFound {
            num_qubits,
        },
    )
}

/// Finds a non-trivial logical operator of exactly `weight`.
///
/// A valid logical operator must:
///
///     - act on exactly `weight` qubits;
///     - commute with every stabilizer;
///     - not belong to the stabilizer group.
pub fn find_logical_operator_of_weight(
    stabilizers: &StabilizerGroup,
    weight: usize,
) -> Result<Option<PauliString>, DistanceError> {
    stabilizers.validate()?;

    let num_qubits =
        stabilizers.num_qubits();

    if weight == 0
        || weight > num_qubits
    {
        return Ok(None);
    }

    let mut selected =
        Vec::with_capacity(weight);

    search_supports(
        stabilizers,
        weight,
        0,
        &mut selected,
    )
}

// ============================================================================
// Support search
// ============================================================================

fn search_supports(
    stabilizers: &StabilizerGroup,
    remaining_weight: usize,
    start_qubit: usize,
    selected: &mut Vec<usize>,
) -> Result<Option<PauliString>, DistanceError> {
    let num_qubits =
        stabilizers.num_qubits();

    if remaining_weight == 0 {
        return search_paulis_on_support(
            stabilizers,
            selected,
        );
    }

    if num_qubits
        .saturating_sub(start_qubit)
        < remaining_weight
    {
        return Ok(None);
    }

    for qubit in
        start_qubit..num_qubits
    {
        selected.push(qubit);

        if let Some(operator) =
            search_supports(
                stabilizers,
                remaining_weight - 1,
                qubit + 1,
                selected,
            )?
        {
            return Ok(Some(operator));
        }

        selected.pop();
    }

    Ok(None)
}

// ============================================================================
// Pauli assignment search
// ============================================================================

fn search_paulis_on_support(
    stabilizers: &StabilizerGroup,
    support: &[usize],
) -> Result<Option<PauliString>, DistanceError> {
    let num_qubits =
        stabilizers.num_qubits();

    let mut paulis =
        vec![Pauli::I; num_qubits];

    search_pauli_assignments(
        stabilizers,
        support,
        0,
        &mut paulis,
    )
}

fn search_pauli_assignments(
    stabilizers: &StabilizerGroup,
    support: &[usize],
    position: usize,
    paulis: &mut [Pauli],
) -> Result<Option<PauliString>, DistanceError> {
    if position == support.len() {
        let operator =
            PauliString::from_paulis(
                paulis,
            );

        // Defensive invariant: the generated operator must have exactly
        // the requested support weight.
        if operator.weight()
            != support.len()
        {
            return Ok(None);
        }

        // A logical operator must lie in the normalizer of the stabilizer
        // group.
        if !commutes_with_stabilizer_group(
            &operator,
            stabilizers,
        )? {
            return Ok(None);
        }

        // A stabilizer itself represents the trivial logical operator.
        if stabilizers.contains(
            &operator,
        )? {
            return Ok(None);
        }

        return Ok(Some(operator));
    }

    let qubit =
        support[position];

    // Every non-identity Pauli is considered because X, Y and Z may have
    // different logical behaviour on the same support.
    for pauli in [
        Pauli::X,
        Pauli::Y,
        Pauli::Z,
    ] {
        paulis[qubit] =
            pauli;

        if let Some(operator) =
            search_pauli_assignments(
                stabilizers,
                support,
                position + 1,
                paulis,
            )?
        {
            return Ok(Some(operator));
        }
    }

    paulis[qubit] =
        Pauli::I;

    Ok(None)
}

// ============================================================================
// Distance validation
// ============================================================================

/// Validates a claimed code distance.
///
/// The supplied logical operator must:
///
///     - have the claimed weight;
///     - commute with every stabilizer;
///     - not be a stabilizer;
///     - have no lower-weight logical operator.
///
/// This provides a stronger invariant than simply trusting a stored
/// distance value.
pub fn validate_distance(
    stabilizers: &StabilizerGroup,
    claimed_distance: usize,
    witness: &PauliString,
) -> Result<(), DistanceError> {
    stabilizers.validate()?;

    if claimed_distance == 0 {
        return Err(
            DistanceError::InvalidDistance {
                distance:
                    claimed_distance,
            },
        );
    }

    if witness.num_qubits()
        != stabilizers.num_qubits()
    {
        return Err(
            DistanceError::Stabilizer(
                StabilizerError::QubitCountMismatch {
                    first:
                        stabilizers.num_qubits(),
                    second:
                        witness.num_qubits(),
                },
            ),
        );
    }

    if witness.is_identity() {
        return Err(
            DistanceError::IdentityLogicalOperator,
        );
    }

    if witness.weight()
        != claimed_distance
    {
        return Err(
            DistanceError::DistanceWeightMismatch {
                distance:
                    claimed_distance,
                weight:
                    witness.weight(),
            },
        );
    }

    if !commutes_with_stabilizer_group(
        witness,
        stabilizers,
    )? {
        return Err(
            DistanceError::WitnessDoesNotCommute,
        );
    }

    if stabilizers.contains(
        witness,
    )? {
        return Err(
            DistanceError::WitnessIsStabilizer,
        );
    }

    // Search for anything strictly smaller than the claimed distance.
    for weight in
        1..claimed_distance
    {
        if find_logical_operator_of_weight(
            stabilizers,
            weight,
        )?
        .is_some()
        {
            return Err(
                DistanceError::LowerWeightLogicalOperator {
                    weight,
                },
            );
        }
    }

    Ok(())
}

// ============================================================================
// Convenience API
// ============================================================================

/// Returns the exact distance as a plain integer.
pub fn distance(
    stabilizers: &StabilizerGroup,
) -> Result<usize, DistanceError> {
    Ok(
        compute_distance(
            stabilizers,
        )?
        .distance(),
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
pub enum DistanceError {
    Stabilizer(
        StabilizerError,
    ),

    NoStabilizerGenerators,

    InvalidDistance {
        distance: usize,
    },

    IdentityLogicalOperator,

    DistanceWeightMismatch {
        distance: usize,
        weight: usize,
    },

    WitnessDoesNotCommute,

    WitnessIsStabilizer,

    LowerWeightLogicalOperator {
        weight: usize,
    },

    NoLogicalOperatorFound {
        num_qubits: usize,
    },
}

impl fmt::Display
    for DistanceError
{
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

            Self::NoStabilizerGenerators => {
                write!(
                    f,
                    "cannot determine code distance without stabilizer generators"
                )
            }

            Self::InvalidDistance {
                distance,
            } => {
                write!(
                    f,
                    "invalid code distance: {distance}"
                )
            }

            Self::IdentityLogicalOperator => {
                write!(
                    f,
                    "identity cannot be a logical-operator witness"
                )
            }

            Self::DistanceWeightMismatch {
                distance,
                weight,
            } => {
                write!(
                    f,
                    "claimed distance {distance} does not match witness weight {weight}"
                )
            }

            Self::WitnessDoesNotCommute => {
                write!(
                    f,
                    "logical-operator witness does not commute with the stabilizer group"
                )
            }

            Self::WitnessIsStabilizer => {
                write!(
                    f,
                    "logical-operator witness is itself a stabilizer"
                )
            }

            Self::LowerWeightLogicalOperator {
                weight,
            } => {
                write!(
                    f,
                    "found a logical operator of lower weight {weight}"
                )
            }

            Self::NoLogicalOperatorFound {
                num_qubits,
            } => {
                write!(
                    f,
                    "no non-trivial logical operator found for {num_qubits}-qubit stabilizer system"
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn three_qubit_repetition_code()
        -> StabilizerGroup
    {
        let mut group =
            StabilizerGroup::new(3)
                .unwrap();

        group
            .add_generator(
                super::super::stabilizer::StabilizerGenerator::new(
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
                super::super::stabilizer::StabilizerGenerator::new(
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
    fn finds_three_qubit_code_distance() {
        let group =
            three_qubit_repetition_code();

        let result =
            compute_distance(
                &group,
            )
            .unwrap();

        assert_eq!(
            result.distance(),
            1
        );
    }

    #[test]
    fn rejects_stabilizer_as_logical_witness() {
        let group =
            three_qubit_repetition_code();

        let stabilizer =
            PauliString::from_paulis(
                &[
                    Pauli::Z,
                    Pauli::Z,
                    Pauli::I,
                ],
            );

        let result =
            validate_distance(
                &group,
                2,
                &stabilizer,
            );

        assert!(matches!(
            result,
            Err(
                DistanceError::
                    WitnessIsStabilizer
            )
        ));
    }

    #[test]
    fn rejects_non_commuting_witness() {
        let group =
            three_qubit_repetition_code();

        let witness =
            PauliString::from_paulis(
                &[
                    Pauli::X,
                    Pauli::I,
                    Pauli::I,
                ],
            );

        // X on q0 actually commutes with ZZ only if it acts on an even
        // number of overlapping X/Z locations; here it anticommutes with
        // the first stabilizer.
        let result =
            validate_distance(
                &group,
                1,
                &witness,
            );

        assert!(matches!(
            result,
            Err(
                DistanceError::
                    WitnessDoesNotCommute
            )
        ));
    }

    #[test]
    fn accepts_valid_logical_witness() {
        let group =
            three_qubit_repetition_code();

        // XXX commutes with both ZZ stabilizers and is not itself a
        // stabilizer. Its weight is 3.
        let witness =
            PauliString::from_paulis(
                &[
                    Pauli::X,
                    Pauli::X,
                    Pauli::X,
                ],
            );

        assert!(
            validate_distance(
                &group,
                3,
                &witness,
            )
            .is_ok()
        );
    }

    #[test]
    fn identity_is_not_a_logical_operator() {
        let group =
            three_qubit_repetition_code();

        let identity =
            PauliString::identity(3);

        let result =
            validate_distance(
                &group,
                0,
                &identity,
            );

        assert!(matches!(
            result,
            Err(
                DistanceError::
                    InvalidDistance {
                        distance: 0
                    }
            )
        ));
    }

    #[test]
    fn distance_convenience_function_works() {
        let group =
            three_qubit_repetition_code();

        assert_eq!(
            distance(&group)
                .unwrap(),
            1
        );
    }
}