//! Zamani Quantum Error Correction — Stabilizer Algebra.
//!
//! Hardware-independent representation and validation of Pauli operators
//! and stabilizer generators.
//!
//! The module provides:
//!
//! - single-qubit Pauli operators;
//! - multi-qubit Pauli strings;
//! - symplectic commutation checks;
//! - stabilizer generators;
//! - stabilizer-group validation;
//! - logical-operator commutation checks;
//! - syndrome calculation.
//!
//! The implementation uses the binary symplectic representation:
//!
//!     P = X^x Z^z
//!
//! where each qubit has one X bit and one Z bit.
//!
//! Two Pauli strings commute iff:
//!
//!     x₁ · z₂ + z₁ · x₂ = 0 (mod 2)
//!
//! This representation intentionally ignores global phase. That is
//! sufficient for stabilizer commutation, syndrome extraction, logical
//! equivalence and error-correction decisions.

use std::collections::BTreeSet;
use std::fmt;

// -----------------------------------------------------------------------------
// Qubit identifier
// -----------------------------------------------------------------------------

/// Logical index of a qubit in a stabilizer Pauli string.
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
pub struct QubitIndex(pub usize);

impl QubitIndex {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for QubitIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "q{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Single-qubit Pauli
// -----------------------------------------------------------------------------

/// Single-qubit Pauli operator.
///
/// Global phase is intentionally excluded.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}

impl Pauli {
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::I)
    }

    pub const fn is_non_identity(self) -> bool {
        !self.is_identity()
    }

    pub const fn has_x_component(self) -> bool {
        matches!(self, Self::X | Self::Y)
    }

    pub const fn has_z_component(self) -> bool {
        matches!(self, Self::Z | Self::Y)
    }

    /// Returns true when the two single-qubit Paulis anticommute.
    pub const fn anticommutes_with(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::X, Self::Z)
                | (Self::Z, Self::X)
                | (Self::X, Self::Y)
                | (Self::Y, Self::X)
                | (Self::Y, Self::Z)
                | (Self::Z, Self::Y)
        )
    }

    /// Multiplication with global phase discarded.
    pub const fn multiply(self, other: Self) -> Self {
        use Pauli::*;

        match (self, other) {
            (I, p) | (p, I) => p,

            (X, X) | (Y, Y) | (Z, Z) => I,

            (X, Y) | (Y, X) => Z,
            (X, Z) | (Z, X) => Y,
            (Y, Z) | (Z, Y) => X,
        }
    }

    pub const fn from_bits(
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
}

impl fmt::Display for Pauli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Self::I => 'I',
            Self::X => 'X',
            Self::Y => 'Y',
            Self::Z => 'Z',
        };

        write!(f, "{symbol}")
    }
}

// -----------------------------------------------------------------------------
// Binary symplectic Pauli string
// -----------------------------------------------------------------------------

/// Multi-qubit Pauli operator represented in binary symplectic form.
///
/// For each qubit:
///
/// ```text
/// x = 0, z = 0  -> I
/// x = 1, z = 0  -> X
/// x = 1, z = 1  -> Y
/// x = 0, z = 1  -> Z
/// ```
///
/// The global phase is not stored.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct PauliString {
    num_qubits: usize,
    x: Vec<bool>,
    z: Vec<bool>,
}

impl PauliString {
    /// Creates the identity on `num_qubits`.
    pub fn identity(
        num_qubits: usize,
    ) -> Self {
        Self {
            num_qubits,
            x: vec![false; num_qubits],
            z: vec![false; num_qubits],
        }
    }

    /// Creates a Pauli string from single-qubit operators.
    pub fn from_paulis(
        paulis: &[Pauli],
    ) -> Self {
        let mut result =
            Self::identity(paulis.len());

        for (index, &pauli) in paulis.iter().enumerate() {
            result.x[index] =
                pauli.has_x_component();

            result.z[index] =
                pauli.has_z_component();
        }

        result
    }

    /// Creates a Pauli string from explicit X/Z bit vectors.
    pub fn from_bits(
        x: Vec<bool>,
        z: Vec<bool>,
    ) -> Result<Self, StabilizerError> {
        if x.len() != z.len() {
            return Err(
                StabilizerError::SymplecticDimensionMismatch {
                    x: x.len(),
                    z: z.len(),
                },
            );
        }

        Ok(Self {
            num_qubits: x.len(),
            x,
            z,
        })
    }

    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn x_bits(&self) -> &[bool] {
        &self.x
    }

    pub fn z_bits(&self) -> &[bool] {
        &self.z
    }

    pub fn pauli_at(
        &self,
        qubit: QubitIndex,
    ) -> Result<Pauli, StabilizerError> {
        self.check_qubit(qubit)?;

        Ok(Pauli::from_bits(
            self.x[qubit.index()],
            self.z[qubit.index()],
        ))
    }

    pub fn set_pauli(
        &mut self,
        qubit: QubitIndex,
        pauli: Pauli,
    ) -> Result<(), StabilizerError> {
        self.check_qubit(qubit)?;

        let index = qubit.index();

        self.x[index] =
            pauli.has_x_component();

        self.z[index] =
            pauli.has_z_component();

        Ok(())
    }

    pub fn weight(&self) -> usize {
        (0..self.num_qubits)
            .filter(|&index| {
                self.x[index] || self.z[index]
            })
            .count()
    }

    pub fn is_identity(&self) -> bool {
        self.weight() == 0
    }

    /// Returns the qubits on which this Pauli is non-identity.
    pub fn support(&self) -> Vec<QubitIndex> {
        (0..self.num_qubits)
            .filter_map(|index| {
                if self.x[index] || self.z[index] {
                    Some(QubitIndex(index))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Symplectic commutation test.
    ///
    /// Two Pauli strings commute iff:
    ///
    /// ```text
    /// x₁·z₂ + z₁·x₂ = 0 mod 2
    /// ```
    pub fn commutes_with(
        &self,
        other: &Self,
    ) -> Result<bool, StabilizerError> {
        self.check_compatible(other)?;

        Ok(self.symplectic_product(other) == 0)
    }

    pub fn anticommutes_with(
        &self,
        other: &Self,
    ) -> Result<bool, StabilizerError> {
        self.check_compatible(other)?;

        Ok(self.symplectic_product(other) == 1)
    }

    /// Returns the binary symplectic product.
    pub fn symplectic_product(
        &self,
        other: &Self,
    ) -> u8 {
        let mut parity = false;

        for index in 0..self.num_qubits {
            parity ^=
                self.x[index] && other.z[index];

            parity ^=
                self.z[index] && other.x[index];
        }

        parity as u8
    }

    /// Multiplies two Pauli strings, ignoring global phase.
    pub fn multiply(
        &self,
        other: &Self,
    ) -> Result<Self, StabilizerError> {
        self.check_compatible(other)?;

        let mut result =
            Self::identity(self.num_qubits);

        for index in 0..self.num_qubits {
            result.x[index] =
                self.x[index] ^ other.x[index];

            result.z[index] =
                self.z[index] ^ other.z[index];
        }

        Ok(result)
    }

    fn check_qubit(
        &self,
        qubit: QubitIndex,
    ) -> Result<(), StabilizerError> {
        if qubit.index() >= self.num_qubits {
            return Err(
                StabilizerError::QubitOutOfRange {
                    qubit,
                    num_qubits: self.num_qubits,
                },
            );
        }

        Ok(())
    }

    fn check_compatible(
        &self,
        other: &Self,
    ) -> Result<(), StabilizerError> {
        if self.num_qubits != other.num_qubits {
            return Err(
                StabilizerError::QubitCountMismatch {
                    first: self.num_qubits,
                    second: other.num_qubits,
                },
            );
        }

        Ok(())
    }
}

impl fmt::Display for PauliString {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        for index in 0..self.num_qubits {
            write!(
                f,
                "{}",
                Pauli::from_bits(
                    self.x[index],
                    self.z[index],
                )
            )?;
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Stabilizer generator
// -----------------------------------------------------------------------------

/// A single stabilizer generator.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct StabilizerGenerator {
    id: usize,
    operator: PauliString,
}

impl StabilizerGenerator {
    pub fn new(
        id: usize,
        operator: PauliString,
    ) -> Result<Self, StabilizerError> {
        if operator.is_identity() {
            return Err(
                StabilizerError::IdentityGenerator {
                    id,
                },
            );
        }

        Ok(Self {
            id,
            operator,
        })
    }

    pub const fn id(&self) -> usize {
        self.id
    }

    pub fn operator(&self) -> &PauliString {
        &self.operator
    }

    pub fn weight(&self) -> usize {
        self.operator.weight()
    }
}

// -----------------------------------------------------------------------------
// Stabilizer group
// -----------------------------------------------------------------------------

/// Validated collection of stabilizer generators.
#[derive(
    Debug,
    Clone,
)]
pub struct StabilizerGroup {
    num_qubits: usize,
    generators: Vec<StabilizerGenerator>,
}

impl StabilizerGroup {
    pub fn new(
        num_qubits: usize,
    ) -> Result<Self, StabilizerError> {
        if num_qubits == 0 {
            return Err(
                StabilizerError::ZeroQubits
            );
        }

        Ok(Self {
            num_qubits,
            generators: Vec::new(),
        })
    }

    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn generators(
        &self,
    ) -> &[StabilizerGenerator] {
        &self.generators
    }

    pub fn len(&self) -> usize {
        self.generators.len()
    }

    pub fn is_empty(&self) -> bool {
        self.generators.is_empty()
    }

    pub fn add_generator(
        &mut self,
        generator: StabilizerGenerator,
    ) -> Result<(), StabilizerError> {
        if generator
            .operator()
            .num_qubits()
            != self.num_qubits
        {
            return Err(
                StabilizerError::QubitCountMismatch {
                    first: self.num_qubits,
                    second: generator
                        .operator()
                        .num_qubits(),
                },
            );
        }

        if self
            .generators
            .iter()
            .any(|existing| {
                existing.id() == generator.id()
            })
        {
            return Err(
                StabilizerError::DuplicateGenerator {
                    id: generator.id(),
                },
            );
        }

        for existing in &self.generators {
            if generator
                .operator()
                .anticommutes_with(
                    existing.operator(),
                )?
            {
                return Err(
                    StabilizerError::NonCommutingGenerators {
                        first: existing.id(),
                        second: generator.id(),
                    },
                );
            }
        }

        self.generators.push(generator);

        Ok(())
    }

    pub fn validate(
        &self,
    ) -> Result<(), StabilizerError> {
        let mut ids =
            BTreeSet::new();

        for generator in &self.generators {
            if !ids.insert(generator.id()) {
                return Err(
                    StabilizerError::DuplicateGenerator {
                        id: generator.id(),
                    },
                );
            }

            if generator
                .operator()
                .num_qubits()
                != self.num_qubits
            {
                return Err(
                    StabilizerError::QubitCountMismatch {
                        first: self.num_qubits,
                        second: generator
                            .operator()
                            .num_qubits(),
                    },
                );
            }
        }

        for i in 0..self.generators.len() {
            for j in (i + 1)..self.generators.len() {
                if self.generators[i]
                    .operator()
                    .anticommutes_with(
                        self.generators[j]
                            .operator(),
                    )?
                {
                    return Err(
                        StabilizerError::NonCommutingGenerators {
                            first: self.generators[i].id(),
                            second: self.generators[j].id(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Returns the syndrome produced by a Pauli error.
    ///
    /// A syndrome bit is `1` exactly when the error anticommutes with the
    /// corresponding stabilizer generator.
    pub fn syndrome(
        &self,
        error: &PauliString,
    ) -> Result<Syndrome, StabilizerError> {
        if error.num_qubits()
            != self.num_qubits
        {
            return Err(
                StabilizerError::QubitCountMismatch {
                    first: self.num_qubits,
                    second: error.num_qubits(),
                },
            );
        }

        let mut bits =
            Vec::with_capacity(
                self.generators.len()
            );

        for generator in &self.generators {
            bits.push(
                generator
                    .operator()
                    .anticommutes_with(error)?,
            );
        }

        Ok(Syndrome {
            bits,
        })
    }

    /// Multiplies a collection of generators, ignoring global phase.
    pub fn product(
        &self,
        indices: &[usize],
    ) -> Result<PauliString, StabilizerError> {
        let mut result =
            PauliString::identity(
                self.num_qubits
            );

        for &index in indices {
            let generator =
                self.generators
                    .iter()
                    .find(|g| g.id() == index)
                    .ok_or(
                        StabilizerError::UnknownGenerator {
                            id: index,
                        },
                    )?;

            result =
                result.multiply(
                    generator.operator(),
                )?;
        }

        Ok(result)
    }
}

// -----------------------------------------------------------------------------
// Syndrome
// -----------------------------------------------------------------------------

/// Syndrome produced by measuring stabilizer generators.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct Syndrome {
    bits: Vec<bool>,
}

impl Syndrome {
    pub fn new(
        bits: Vec<bool>,
    ) -> Self {
        Self { bits }
    }

    pub fn bits(&self) -> &[bool] {
        &self.bits
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    pub fn triggered_count(&self) -> usize {
        self.bits
            .iter()
            .filter(|&&bit| bit)
            .count()
    }

    pub fn triggered(
        &self,
    ) -> impl Iterator<Item = usize> + '_ {
        self.bits
            .iter()
            .enumerate()
            .filter_map(
                |(index, &triggered)| {
                    triggered.then_some(index)
                },
            )
    }

    pub fn is_trivial(&self) -> bool {
        self.triggered_count() == 0
    }
}

// -----------------------------------------------------------------------------
// Logical operator validation
// -----------------------------------------------------------------------------

/// Validates that a Pauli operator commutes with every stabilizer generator.
pub fn commutes_with_stabilizer_group(
    operator: &PauliString,
    group: &StabilizerGroup,
) -> Result<bool, StabilizerError> {
    for generator in group.generators() {
        if operator
            .anticommutes_with(
                generator.operator(),
            )?
        {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Returns true when two operators anticommute.
pub fn logical_operators_anticommute(
    first: &PauliString,
    second: &PauliString,
) -> Result<bool, StabilizerError> {
    first.anticommutes_with(second)
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
pub enum StabilizerError {
    ZeroQubits,

    QubitOutOfRange {
        qubit: QubitIndex,
        num_qubits: usize,
    },

    QubitCountMismatch {
        first: usize,
        second: usize,
    },

    SymplecticDimensionMismatch {
        x: usize,
        z: usize,
    },

    IdentityGenerator {
        id: usize,
    },

    DuplicateGenerator {
        id: usize,
    },

    UnknownGenerator {
        id: usize,
    },

    NonCommutingGenerators {
        first: usize,
        second: usize,
    },
}

impl fmt::Display for StabilizerError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroQubits => {
                write!(
                    f,
                    "stabilizer system must contain at least one qubit"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "qubit {qubit} is outside a {num_qubits}-qubit system"
                )
            }

            Self::QubitCountMismatch {
                first,
                second,
            } => {
                write!(
                    f,
                    "Pauli/stabilizer qubit-count mismatch: {first} != {second}"
                )
            }

            Self::SymplecticDimensionMismatch {
                x,
                z,
            } => {
                write!(
                    f,
                    "symplectic X/Z dimensions differ: {x} != {z}"
                )
            }

            Self::IdentityGenerator { id } => {
                write!(
                    f,
                    "stabilizer generator {id} cannot be identity"
                )
            }

            Self::DuplicateGenerator { id } => {
                write!(
                    f,
                    "stabilizer generator {id} already exists"
                )
            }

            Self::UnknownGenerator { id } => {
                write!(
                    f,
                    "unknown stabilizer generator {id}"
                )
            }

            Self::NonCommutingGenerators {
                first,
                second,
            } => {
                write!(
                    f,
                    "stabilizer generators {first} and {second} do not commute"
                )
            }
        }
    }
}

impl std::error::Error for StabilizerError {}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pauli_multiplication_ignores_global_phase() {
        assert_eq!(
            Pauli::X.multiply(Pauli::Z),
            Pauli::Y
        );

        assert_eq!(
            Pauli::Y.multiply(Pauli::Y),
            Pauli::I
        );
    }

    #[test]
    fn single_qubit_x_and_z_anticommute() {
        assert!(
            Pauli::X
                .anticommutes_with(Pauli::Z)
        );

        assert!(
            !Pauli::X
                .anticommutes_with(Pauli::X)
        );
    }

    #[test]
    fn pauli_string_commutation_is_symplectic() {
        let x =
            PauliString::from_paulis(&[
                Pauli::X,
            ]);

        let z =
            PauliString::from_paulis(&[
                Pauli::Z,
            ]);

        assert!(
            x.anticommutes_with(&z)
                .unwrap()
        );
    }

    #[test]
    fn disjoint_paulis_commute() {
        let x =
            PauliString::from_paulis(&[
                Pauli::X,
                Pauli::I,
            ]);

        let z =
            PauliString::from_paulis(&[
                Pauli::I,
                Pauli::Z,
            ]);

        assert!(
            x.commutes_with(&z)
                .unwrap()
        );
    }

    #[test]
    fn overlapping_even_parity_commutes() {
        let x =
            PauliString::from_paulis(&[
                Pauli::X,
                Pauli::X,
            ]);

        let z =
            PauliString::from_paulis(&[
                Pauli::Z,
                Pauli::Z,
            ]);

        assert!(
            x.commutes_with(&z)
                .unwrap()
        );
    }

    #[test]
    fn pauli_string_multiplication_works() {
        let x =
            PauliString::from_paulis(&[
                Pauli::X,
                Pauli::I,
            ]);

        let z =
            PauliString::from_paulis(&[
                Pauli::Z,
                Pauli::I,
            ]);

        let result =
            x.multiply(&z)
                .unwrap();

        assert_eq!(
            result.pauli_at(
                QubitIndex::new(0)
            )
            .unwrap(),
            Pauli::Y
        );
    }

    #[test]
    fn stabilizer_group_rejects_non_commuting_generators() {
        let mut group =
            StabilizerGroup::new(1)
                .unwrap();

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(
                        &[Pauli::X],
                    ),
                )
                .unwrap(),
            )
            .unwrap();

        let result =
            group.add_generator(
                StabilizerGenerator::new(
                    1,
                    PauliString::from_paulis(
                        &[Pauli::Z],
                    ),
                )
                .unwrap(),
            );

        assert!(matches!(
            result,
            Err(
                StabilizerError::NonCommutingGenerators {
                    first: 0,
                    second: 1
                }
            )
        ));
    }

    #[test]
    fn syndrome_detects_anticommutation() {
        let mut group =
            StabilizerGroup::new(1)
                .unwrap();

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(
                        &[Pauli::Z],
                    ),
                )
                .unwrap(),
            )
            .unwrap();

        let error =
            PauliString::from_paulis(
                &[Pauli::X],
            );

        let syndrome =
            group.syndrome(&error)
                .unwrap();

        assert_eq!(
            syndrome.bits(),
            &[true]
        );

        assert_eq!(
            syndrome.triggered_count(),
            1
        );
    }

    #[test]
    fn trivial_syndrome_is_detected() {
        let mut group =
            StabilizerGroup::new(1)
                .unwrap();

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(
                        &[Pauli::Z],
                    ),
                )
                .unwrap(),
            )
            .unwrap();

        let error =
            PauliString::from_paulis(
                &[Pauli::Z],
            );

        let syndrome =
            group.syndrome(&error)
                .unwrap();

        assert!(
            syndrome.is_trivial()
        );
    }

    #[test]
    fn logical_operator_must_commute_with_group() {
        let mut group =
            StabilizerGroup::new(2)
                .unwrap();

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(
                        &[
                            Pauli::Z,
                            Pauli::Z,
                        ],
                    ),
                )
                .unwrap(),
            )
            .unwrap();

        let logical =
            PauliString::from_paulis(
                &[
                    Pauli::X,
                    Pauli::X,
                ],
            );

        assert!(
            commutes_with_stabilizer_group(
                &logical,
                &group,
            )
            .unwrap()
        );
    }

    #[test]
    fn logical_x_and_z_can_anticommute() {
        let logical_x =
            PauliString::from_paulis(
                &[
                    Pauli::X,
                    Pauli::I,
                ],
            );

        let logical_z =
            PauliString::from_paulis(
                &[
                    Pauli::Z,
                    Pauli::I,
                ],
            );

        assert!(
            logical_operators_anticommute(
                &logical_x,
                &logical_z,
            )
            .unwrap()
        );
    }

    #[test]
    fn identity_generator_is_rejected() {
        let result =
            StabilizerGenerator::new(
                0,
                PauliString::identity(3),
            );

        assert!(matches!(
            result,
            Err(
                StabilizerError::IdentityGenerator {
                    id: 0
                }
            )
        ));
    }
}