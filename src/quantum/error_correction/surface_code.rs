//! Zamani Quantum Error Correction — Surface Code.
//!
//! Mathematically explicit surface-code representation.
//!
//! The implementation separates:
//!
//! 1. physical data-qubit topology;
//! 2. stabilizer topology;
//! 3. stabilizer Pauli operators;
//! 4. logical operators;
//! 5. code validation;
//! 6. code-distance verification.
//!
//! A surface code is valid only when:
//!
//! - every referenced data qubit exists;
//! - no stabilizer contains duplicate qubits;
//! - stabilizer support matches its topology;
//! - stabilizer weights are valid;
//! - X/Z stabilizers commute;
//! - logical operators commute with every stabilizer;
//! - logical X and logical Z anticommute;
//! - logical operators are not themselves stabilizers;
//! - the claimed code distance is consistent with the logical operators.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::stabilizer::{
    Pauli,
    PauliString,
    QubitIndex,
    StabilizerError,
    StabilizerGenerator,
    StabilizerGroup,
};

// -----------------------------------------------------------------------------
// Coordinates
// -----------------------------------------------------------------------------

/// Integer coordinate in the surface-code lattice.
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
pub struct Coordinate {
    pub row: usize,
    pub column: usize,
}

impl Coordinate {
    pub const fn new(
        row: usize,
        column: usize,
    ) -> Self {
        Self { row, column }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "({}, {})",
            self.row,
            self.column
        )
    }
}

// -----------------------------------------------------------------------------
// Data qubit
// -----------------------------------------------------------------------------

/// Physical data qubit in the surface-code lattice.
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
pub struct DataQubit {
    id: QubitIndex,
    coordinate: Coordinate,
}

impl DataQubit {
    pub const fn new(
        id: QubitIndex,
        coordinate: Coordinate,
    ) -> Self {
        Self {
            id,
            coordinate,
        }
    }

    pub const fn id(&self) -> QubitIndex {
        self.id
    }

    pub const fn coordinate(
        &self,
    ) -> Coordinate {
        self.coordinate
    }
}

// -----------------------------------------------------------------------------
// Stabilizer kind
// -----------------------------------------------------------------------------

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
pub enum StabilizerKind {
    X,
    Z,
}

impl StabilizerKind {
    pub const fn pauli(
        self,
    ) -> Pauli {
        match self {
            Self::X => Pauli::X,
            Self::Z => Pauli::Z,
        }
    }
}

// -----------------------------------------------------------------------------
// Surface-code stabilizer
// -----------------------------------------------------------------------------

/// Explicit geometric stabilizer.
///
/// `qubits` is the exact set of data qubits on which the stabilizer acts.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct SurfaceStabilizer {
    id: usize,
    kind: StabilizerKind,
    coordinate: Coordinate,
    qubits: Vec<QubitIndex>,
}

impl SurfaceStabilizer {
    pub fn new(
        id: usize,
        kind: StabilizerKind,
        coordinate: Coordinate,
        qubits: Vec<QubitIndex>,
    ) -> Result<Self, SurfaceCodeError> {
        if qubits.is_empty() {
            return Err(
                SurfaceCodeError::EmptyStabilizer {
                    id,
                },
            );
        }

        let unique =
            qubits
                .iter()
                .copied()
                .collect::<BTreeSet<_>>();

        if unique.len() != qubits.len() {
            return Err(
                SurfaceCodeError::DuplicateQubit {
                    stabilizer: id,
                },
            );
        }

        Ok(Self {
            id,
            kind,
            coordinate,
            qubits,
        })
    }

    pub const fn id(&self) -> usize {
        self.id
    }

    pub const fn kind(
        &self,
    ) -> StabilizerKind {
        self.kind
    }

    pub const fn coordinate(
        &self,
    ) -> Coordinate {
        self.coordinate
    }

    pub fn qubits(
        &self,
    ) -> &[QubitIndex] {
        &self.qubits
    }

    pub fn weight(&self) -> usize {
        self.qubits.len()
    }

    /// Converts the geometric stabilizer into the common stabilizer algebra.
    pub fn to_pauli_string(
        &self,
        num_qubits: usize,
    ) -> Result<PauliString, SurfaceCodeError> {
        let mut operator =
            PauliString::identity(
                num_qubits,
            );

        for &qubit in &self.qubits {
            if qubit.index() >= num_qubits {
                return Err(
                    SurfaceCodeError::NonexistentQubit {
                        stabilizer: self.id,
                        qubit,
                    },
                );
            }

            operator
                .set_pauli(
                    qubit,
                    self.kind.pauli(),
                )
                .map_err(
                    SurfaceCodeError::Stabilizer,
                )?;
        }

        Ok(operator)
    }
}

// -----------------------------------------------------------------------------
// Logical operator
// -----------------------------------------------------------------------------

/// Logical Pauli operator represented explicitly on physical data qubits.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct LogicalOperator {
    name: String,
    pauli: PauliString,
}

impl LogicalOperator {
    pub fn new(
        name: impl Into<String>,
        pauli: PauliString,
    ) -> Result<Self, SurfaceCodeError> {
        if pauli.is_identity() {
            return Err(
                SurfaceCodeError::IdentityLogicalOperator,
            );
        }

        Ok(Self {
            name: name.into(),
            pauli,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn operator(&self) -> &PauliString {
        &self.pauli
    }

    pub fn weight(&self) -> usize {
        self.pauli.weight()
    }
}

// -----------------------------------------------------------------------------
// Surface code
// -----------------------------------------------------------------------------

/// Explicit surface-code model.
#[derive(
    Debug,
    Clone,
)]
pub struct SurfaceCode {
    distance: usize,
    data_qubits: BTreeMap<QubitIndex, DataQubit>,
    stabilizers: Vec<SurfaceStabilizer>,
    logical_x: LogicalOperator,
    logical_z: LogicalOperator,
}

impl SurfaceCode {
    /// Creates an empty surface-code model.
    ///
    /// Topology is added explicitly using `add_data_qubit` and
    /// `add_stabilizer`.
    pub fn new(
        distance: usize,
        logical_x: LogicalOperator,
        logical_z: LogicalOperator,
    ) -> Result<Self, SurfaceCodeError> {
        if distance < 2 {
            return Err(
                SurfaceCodeError::InvalidDistance {
                    distance,
                },
            );
        }

        if logical_x
            .operator()
            .num_qubits()
            != logical_z
                .operator()
                .num_qubits()
        {
            return Err(
                SurfaceCodeError::LogicalQubitCountMismatch,
            );
        }

        Ok(Self {
            distance,
            data_qubits: BTreeMap::new(),
            stabilizers: Vec::new(),
            logical_x,
            logical_z,
        })
    }

    pub const fn distance(
        &self,
    ) -> usize {
        self.distance
    }

    pub fn data_qubits(
        &self,
    ) -> impl Iterator<Item = &DataQubit> {
        self.data_qubits.values()
    }

    pub fn stabilizers(
        &self,
    ) -> &[SurfaceStabilizer] {
        &self.stabilizers
    }

    pub fn logical_x(
        &self,
    ) -> &LogicalOperator {
        &self.logical_x
    }

    pub fn logical_z(
        &self,
    ) -> &LogicalOperator {
        &self.logical_z
    }

    pub fn num_data_qubits(&self) -> usize {
        self.data_qubits.len()
    }

    pub fn num_stabilizers(&self) -> usize {
        self.stabilizers.len()
    }

    /// Adds one physical data qubit.
    pub fn add_data_qubit(
        &mut self,
        qubit: DataQubit,
    ) -> Result<(), SurfaceCodeError> {
        if self
            .data_qubits
            .contains_key(&qubit.id())
        {
            return Err(
                SurfaceCodeError::DuplicateDataQubit {
                    qubit: qubit.id(),
                },
            );
        }

        if self
            .data_qubits
            .values()
            .any(|existing| {
                existing.coordinate()
                    == qubit.coordinate()
            })
        {
            return Err(
                SurfaceCodeError::DuplicateCoordinate {
                    coordinate: qubit.coordinate(),
                },
            );
        }

        self.data_qubits
            .insert(qubit.id(), qubit);

        Ok(())
    }

    /// Adds a geometrically explicit stabilizer.
    pub fn add_stabilizer(
        &mut self,
        stabilizer: SurfaceStabilizer,
    ) -> Result<(), SurfaceCodeError> {
        if self
            .stabilizers
            .iter()
            .any(|existing| {
                existing.id()
                    == stabilizer.id()
            })
        {
            return Err(
                SurfaceCodeError::DuplicateStabilizer {
                    id: stabilizer.id(),
                },
            );
        }

        for &qubit in stabilizer.qubits() {
            if !self
                .data_qubits
                .contains_key(&qubit)
            {
                return Err(
                    SurfaceCodeError::NonexistentQubit {
                        stabilizer:
                            stabilizer.id(),
                        qubit,
                    },
                );
            }
        }

        self.stabilizers
            .push(stabilizer);

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Performs all mathematical and topology checks.
    pub fn validate(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        self.validate_data_qubits()?;
        self.validate_stabilizers()?;
        self.validate_commutation()?;
        self.validate_logical_operators()?;
        self.validate_distance()?;

        Ok(())
    }

    fn validate_data_qubits(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        if self.data_qubits.is_empty() {
            return Err(
                SurfaceCodeError::NoDataQubits,
            );
        }

        let mut coordinates =
            BTreeSet::new();

        for qubit in self.data_qubits.values() {
            if !coordinates.insert(
                qubit.coordinate(),
            ) {
                return Err(
                    SurfaceCodeError::DuplicateCoordinate {
                        coordinate:
                            qubit.coordinate(),
                    },
                );
            }
        }

        Ok(())
    }

    fn validate_stabilizers(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        for stabilizer in
            &self.stabilizers
        {
            if stabilizer.weight()
                > 4
            {
                return Err(
                    SurfaceCodeError::InvalidStabilizerWeight {
                        id: stabilizer.id(),
                        weight: stabilizer.weight(),
                    },
                );
            }

            if stabilizer.weight()
                == 0
            {
                return Err(
                    SurfaceCodeError::EmptyStabilizer {
                        id: stabilizer.id(),
                    },
                );
            }

            let mut seen =
                BTreeSet::new();

            for &qubit in
                stabilizer.qubits()
            {
                if !seen.insert(qubit) {
                    return Err(
                        SurfaceCodeError::DuplicateQubit {
                            stabilizer:
                                stabilizer.id(),
                        },
                    );
                }

                if !self
                    .data_qubits
                    .contains_key(&qubit)
                {
                    return Err(
                        SurfaceCodeError::NonexistentQubit {
                            stabilizer:
                                stabilizer.id(),
                            qubit,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Verifies pairwise stabilizer commutation using the common
    /// symplectic representation.
    fn validate_commutation(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let num_qubits =
            self.num_data_qubits();

        let mut operators =
            Vec::with_capacity(
                self.stabilizers.len(),
            );

        for stabilizer in
            &self.stabilizers
        {
            operators.push((
                stabilizer,
                stabilizer
                    .to_pauli_string(
                        num_qubits,
                    )?,
            ));
        }

        for i in 0..operators.len() {
            for j in (i + 1)..operators.len()
            {
                if operators[i]
                    .1
                    .anticommutes_with(
                        &operators[j].1,
                    )
                    .map_err(
                        SurfaceCodeError::Stabilizer,
                    )?
                {
                    return Err(
                        SurfaceCodeError::NonCommutingStabilizers {
                            first:
                                operators[i]
                                    .0
                                    .id(),
                            second:
                                operators[j]
                                    .0
                                    .id(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    fn validate_logical_operators(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let num_qubits =
            self.num_data_qubits();

        if self.logical_x
            .operator()
            .num_qubits()
            != num_qubits
        {
            return Err(
                SurfaceCodeError::LogicalQubitCountMismatch,
            );
        }

        if self.logical_z
            .operator()
            .num_qubits()
            != num_qubits
        {
            return Err(
                SurfaceCodeError::LogicalQubitCountMismatch,
            );
        }

        let stabilizer_group =
            self.stabilizer_group()?;

        if !super::stabilizer::
            commutes_with_stabilizer_group(
                self.logical_x.operator(),
                &stabilizer_group,
            )
            .map_err(
                SurfaceCodeError::Stabilizer,
            )?
        {
            return Err(
                SurfaceCodeError::LogicalDoesNotCommute {
                    logical: self
                        .logical_x
                        .name()
                        .to_owned(),
                },
            );
        }

        if !super::stabilizer::
            commutes_with_stabilizer_group(
                self.logical_z.operator(),
                &stabilizer_group,
            )
            .map_err(
                SurfaceCodeError::Stabilizer,
            )?
        {
            return Err(
                SurfaceCodeError::LogicalDoesNotCommute {
                    logical: self
                        .logical_z
                        .name()
                        .to_owned(),
                },
            );
        }

        if !self
            .logical_x
            .operator()
            .anticommutes_with(
                self.logical_z.operator(),
            )
            .map_err(
                SurfaceCodeError::Stabilizer,
            )?
        {
            return Err(
                SurfaceCodeError::LogicalOperatorsMustAnticommute,
            );
        }

        Ok(())
    }

    /// Verifies that the explicitly supplied logical operators have the
    /// requested distance.
    ///
    /// For a concrete logical representative, its weight cannot be smaller
    /// than the code distance. A complete distance proof additionally
    /// requires searching the normalizer for the minimum non-stabilizer
    /// logical operator.
    fn validate_distance(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        if self.logical_x.weight()
            < self.distance
        {
            return Err(
                SurfaceCodeError::LogicalOperatorBelowDistance {
                    logical: self
                        .logical_x
                        .name()
                        .to_owned(),
                    weight:
                        self.logical_x
                            .weight(),
                    distance:
                        self.distance,
                },
            );
        }

        if self.logical_z.weight()
            < self.distance
        {
            return Err(
                SurfaceCodeError::LogicalOperatorBelowDistance {
                    logical: self
                        .logical_z
                        .name()
                        .to_owned(),
                    weight:
                        self.logical_z
                            .weight(),
                    distance:
                        self.distance,
                },
            );
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Stabilizer conversion
    // -------------------------------------------------------------------------

    /// Converts the geometric stabilizers into the common stabilizer group.
    pub fn stabilizer_group(
        &self,
    ) -> Result<StabilizerGroup, SurfaceCodeError> {
        let num_qubits =
            self.num_data_qubits();

        let mut group =
            StabilizerGroup::new(
                num_qubits,
            )
            .map_err(
                SurfaceCodeError::Stabilizer,
            )?;

        for stabilizer in
            &self.stabilizers
        {
            let operator =
                stabilizer
                    .to_pauli_string(
                        num_qubits,
                    )?;

            let generator =
                StabilizerGenerator::new(
                    stabilizer.id(),
                    operator,
                )
                .map_err(
                    SurfaceCodeError::Stabilizer,
                )?;

            group
                .add_generator(generator)
                .map_err(
                    SurfaceCodeError::Stabilizer,
                )?;
        }

        Ok(group)
    }

    /// Computes the syndrome for a physical Pauli error.
    pub fn syndrome(
        &self,
        error: &PauliString,
    ) -> Result<
        super::stabilizer::Syndrome,
        SurfaceCodeError,
    > {
        let group =
            self.stabilizer_group()?;

        group
            .syndrome(error)
            .map_err(
                SurfaceCodeError::Stabilizer,
            )
    }

    // -------------------------------------------------------------------------
    // Topology helpers
    // -------------------------------------------------------------------------

    /// Returns all data qubits directly referenced by a stabilizer.
    pub fn stabilizer_support(
        &self,
        id: usize,
    ) -> Result<&[QubitIndex], SurfaceCodeError> {
        self.stabilizers
            .iter()
            .find(|stabilizer| {
                stabilizer.id() == id
            })
            .map(
                SurfaceStabilizer::qubits
            )
            .ok_or(
                SurfaceCodeError::UnknownStabilizer {
                    id,
                },
            )
    }

    /// Returns the data qubits at a lattice coordinate.
    pub fn qubit_at(
        &self,
        coordinate: Coordinate,
    ) -> Option<&DataQubit> {
        self.data_qubits
            .values()
            .find(|qubit| {
                qubit.coordinate()
                    == coordinate
            })
    }
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
pub enum SurfaceCodeError {
    InvalidDistance {
        distance: usize,
    },

    NoDataQubits,

    DuplicateDataQubit {
        qubit: QubitIndex,
    },

    DuplicateCoordinate {
        coordinate: Coordinate,
    },

    DuplicateStabilizer {
        id: usize,
    },

    EmptyStabilizer {
        id: usize,
    },

    DuplicateQubit {
        stabilizer: usize,
    },

    NonexistentQubit {
        stabilizer: usize,
        qubit: QubitIndex,
    },

    InvalidStabilizerWeight {
        id: usize,
        weight: usize,
    },

    NonCommutingStabilizers {
        first: usize,
        second: usize,
    },

    LogicalQubitCountMismatch,

    IdentityLogicalOperator,

    LogicalDoesNotCommute {
        logical: String,
    },

    LogicalOperatorsMustAnticommute,

    LogicalOperatorBelowDistance {
        logical: String,
        weight: usize,
        distance: usize,
    },

    UnknownStabilizer {
        id: usize,
    },

    Stabilizer(
        StabilizerError,
    ),
}

impl fmt::Display
    for SurfaceCodeError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidDistance {
                distance,
            } => write!(
                f,
                "surface-code distance must be >= 2, got {distance}"
            ),

            Self::NoDataQubits => {
                write!(
                    f,
                    "surface code contains no data qubits"
                )
            }

            Self::DuplicateDataQubit {
                qubit,
            } => {
                write!(
                    f,
                    "data qubit {qubit} is already defined"
                )
            }

            Self::DuplicateCoordinate {
                coordinate,
            } => {
                write!(
                    f,
                    "data-qubit coordinate {coordinate} is already occupied"
                )
            }

            Self::DuplicateStabilizer {
                id,
            } => {
                write!(
                    f,
                    "stabilizer {id} is already defined"
                )
            }

            Self::EmptyStabilizer {
                id,
            } => {
                write!(
                    f,
                    "stabilizer {id} has no data-qubit support"
                )
            }

            Self::DuplicateQubit {
                stabilizer,
            } => {
                write!(
                    f,
                    "stabilizer {stabilizer} contains a duplicate data qubit"
                )
            }

            Self::NonexistentQubit {
                stabilizer,
                qubit,
            } => {
                write!(
                    f,
                    "stabilizer {stabilizer} references nonexistent qubit {qubit}"
                )
            }

            Self::InvalidStabilizerWeight {
                id,
                weight,
            } => {
                write!(
                    f,
                    "stabilizer {id} has invalid weight {weight}; surface-code stabilizers have weight 2–4"
                )
            }

            Self::NonCommutingStabilizers {
                first,
                second,
            } => {
                write!(
                    f,
                    "stabilizers {first} and {second} do not commute"
                )
            }

            Self::LogicalQubitCountMismatch => {
                write!(
                    f,
                    "logical operators do not match the data-qubit count"
                )
            }

            Self::IdentityLogicalOperator => {
                write!(
                    f,
                    "logical operator cannot be identity"
                )
            }

            Self::LogicalDoesNotCommute {
                logical,
            } => {
                write!(
                    f,
                    "logical operator '{logical}' does not commute with the stabilizer group"
                )
            }

            Self::LogicalOperatorsMustAnticommute => {
                write!(
                    f,
                    "logical X and logical Z must anticommute"
                )
            }

            Self::LogicalOperatorBelowDistance {
                logical,
                weight,
                distance,
            } => {
                write!(
                    f,
                    "logical operator '{logical}' has weight {weight}, below claimed distance {distance}"
                )
            }

            Self::UnknownStabilizer {
                id,
            } => {
                write!(
                    f,
                    "unknown stabilizer {id}"
                )
            }

            Self::Stabilizer(error) => {
                write!(
                    f,
                    "stabilizer algebra error: {error}"
                )
            }
        }
    }
}

impl std::error::Error
    for SurfaceCodeError
{
}

impl From<StabilizerError>
    for SurfaceCodeError
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

    fn logical_operator(
        name: &str,
        paulis: &[Pauli],
    ) -> LogicalOperator {
        LogicalOperator::new(
            name,
            PauliString::from_paulis(
                paulis,
            ),
        )
        .unwrap()
    }

    #[test]
    fn duplicate_qubits_inside_stabilizer_are_rejected() {
        let result =
            SurfaceStabilizer::new(
                0,
                StabilizerKind::X,
                Coordinate::new(0, 0),
                vec![
                    QubitIndex::new(0),
                    QubitIndex::new(0),
                ],
            );

        assert!(matches!(
            result,
            Err(
                SurfaceCodeError::DuplicateQubit {
                    stabilizer: 0
                }
            )
        ));
    }

    #[test]
    fn nonexistent_stabilizer_qubit_is_rejected() {
        let logical_x =
            logical_operator(
                "logical_x",
                &[
                    Pauli::X,
                    Pauli::X,
                ],
            );

        let logical_z =
            logical_operator(
                "logical_z",
                &[
                    Pauli::Z,
                    Pauli::Z,
                ],
            );

        let mut code =
            SurfaceCode::new(
                2,
                logical_x,
                logical_z,
            )
            .unwrap();

        code.add_data_qubit(
            DataQubit::new(
                QubitIndex::new(0),
                Coordinate::new(0, 0),
            ),
        )
        .unwrap();

        let result =
            code.add_stabilizer(
                SurfaceStabilizer::new(
                    0,
                    StabilizerKind::X,
                    Coordinate::new(0, 1),
                    vec![
                        QubitIndex::new(0),
                        QubitIndex::new(1),
                    ],
                )
                .unwrap(),
            );

        assert!(matches!(
            result,
            Err(
                SurfaceCodeError::NonexistentQubit {
                    stabilizer: 0,
                    qubit: QubitIndex(1)
                }
            )
        ));
    }

    #[test]
    fn stabilizer_support_is_explicit() {
        let stabilizer =
            SurfaceStabilizer::new(
                7,
                StabilizerKind::Z,
                Coordinate::new(1, 1),
                vec![
                    QubitIndex::new(0),
                    QubitIndex::new(1),
                    QubitIndex::new(3),
                ],
            )
            .unwrap();

        assert_eq!(
            stabilizer.weight(),
            3
        );

        assert_eq!(
            stabilizer.qubits(),
            &[
                QubitIndex::new(0),
                QubitIndex::new(1),
                QubitIndex::new(3),
            ]
        );
    }

    #[test]
    fn logical_x_and_z_must_anticommute() {
        let logical_x =
            LogicalOperator::new(
                "X",
                PauliString::from_paulis(
                    &[
                        Pauli::X,
                        Pauli::I,
                    ],
                ),
            )
            .unwrap();

        let logical_z =
            LogicalOperator::new(
                "Z",
                PauliString::from_paulis(
                    &[
                        Pauli::Z,
                        Pauli::I,
                    ],
                ),
            )
            .unwrap();

        let code =
            SurfaceCode::new(
                2,
                logical_x,
                logical_z,
            )
            .unwrap();

        assert!(
            code.logical_x()
                .operator()
                .anticommutes_with(
                    code.logical_z()
                        .operator(),
                )
                .unwrap()
        );
    }

    #[test]
    fn empty_code_fails_validation() {
        let logical_x =
            logical_operator(
                "X",
                &[
                    Pauli::X,
                    Pauli::X,
                ],
            );

        let logical_z =
            logical_operator(
                "Z",
                &[
                    Pauli::Z,
                    Pauli::Z,
                ],
            );

        let code =
            SurfaceCode::new(
                2,
                logical_x,
                logical_z,
            )
            .unwrap();

        assert!(matches!(
            code.validate(),
            Err(
                SurfaceCodeError::NoDataQubits
            )
        ));
    }

    #[test]
    fn stabilizer_converts_to_common_pauli_model() {
        let stabilizer =
            SurfaceStabilizer::new(
                0,
                StabilizerKind::X,
                Coordinate::new(0, 0),
                vec![
                    QubitIndex::new(0),
                    QubitIndex::new(1),
                ],
            )
            .unwrap();

        let operator =
            stabilizer
                .to_pauli_string(2)
                .unwrap();

        assert_eq!(
            operator
                .pauli_at(
                    QubitIndex::new(0)
                )
                .unwrap(),
            Pauli::X
        );

        assert_eq!(
            operator
                .pauli_at(
                    QubitIndex::new(1)
                )
                .unwrap(),
            Pauli::X
        );
    }
}