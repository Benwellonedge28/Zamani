//! Zamani Quantum Error Correction — Stabilizer Algebra.
//!
//! Hardware-independent representation and validation of Pauli operators
//! and stabilizer generators.
//!
//! The implementation uses the binary symplectic representation:
//!
//!     P = X^x Z^z
//!
//! Two Pauli strings commute iff:
//!
//!     x₁ · z₂ + z₁ · x₂ = 0 (mod 2)
//!
//! Global phase is intentionally ignored because it is not required for
//! stabilizer commutation, syndrome extraction, logical equivalence, or
//! error-correction decisions.

use std::collections::BTreeSet;
use std::fmt;

// -----------------------------------------------------------------------------
// Qubit identifier
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
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "q{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Pauli
// -----------------------------------------------------------------------------

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
    pub const fn is_identity(
        self,
    ) -> bool {
        matches!(self, Self::I)
    }

    pub const fn is_non_identity(
        self,
    ) -> bool {
        !self.is_identity()
    }

    pub const fn has_x_component(
        self,
    ) -> bool {
        matches!(
            self,
            Self::X | Self::Y
        )
    }

    pub const fn has_z_component(
        self,
    ) -> bool {
        matches!(
            self,
            Self::Z | Self::Y
        )
    }

    pub const fn anticommutes_with(
        self,
        other: Self,
    ) -> bool {
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

    /// Multiplies two Paulis while discarding global phase.
    pub const fn multiply(
        self,
        other: Self,
    ) -> Self {
        use Pauli::*;

        match (self, other) {
            (I, p) | (p, I) => p,

            (X, X)
            | (Y, Y)
            | (Z, Z) => I,

            (X, Y)
            | (Y, X) => Z,

            (X, Z)
            | (Z, X) => Y,

            (Y, Z)
            | (Z, Y) => X,
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
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
// Pauli string
// -----------------------------------------------------------------------------

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
    pub fn identity(
        num_qubits: usize,
    ) -> Self {
        Self {
            num_qubits,
            x: vec![false; num_qubits],
            z: vec![false; num_qubits],
        }
    }

    pub fn from_paulis(
        paulis: &[Pauli],
    ) -> Self {
        let mut result =
            Self::identity(
                paulis.len(),
            );

        for (index, &pauli)
            in paulis.iter().enumerate()
        {
            result.x[index] =
                pauli.has_x_component();

            result.z[index] =
                pauli.has_z_component();
        }

        result
    }

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

    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.num_qubits
    }

    pub fn x_bits(
        &self,
    ) -> &[bool] {
        &self.x
    }

    pub fn z_bits(
        &self,
    ) -> &[bool] {
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

        let index =
            qubit.index();

        self.x[index] =
            pauli.has_x_component();

        self.z[index] =
            pauli.has_z_component();

        Ok(())
    }

    pub fn weight(
        &self,
    ) -> usize {
        (0..self.num_qubits)
            .filter(|&index| {
                self.x[index]
                    || self.z[index]
            })
            .count()
    }

    pub fn is_identity(
        &self,
    ) -> bool {
        self.weight() == 0
    }

    pub fn support(
        &self,
    ) -> Vec<QubitIndex> {
        (0..self.num_qubits)
            .filter_map(|index| {
                if self.x[index]
                    || self.z[index]
                {
                    Some(
                        QubitIndex(index),
                    )
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn commutes_with(
        &self,
        other: &Self,
    ) -> Result<bool, StabilizerError> {
        self.check_compatible(other)?;

        Ok(
            self.symplectic_product(
                other,
            ) == 0
        )
    }

    pub fn anticommutes_with(
        &self,
        other: &Self,
    ) -> Result<bool, StabilizerError> {
        self.check_compatible(other)?;

        Ok(
            self.symplectic_product(
                other,
            ) == 1
        )
    }

    pub fn symplectic_product(
        &self,
        other: &Self,
    ) -> u8 {
        debug_assert_eq!(
            self.num_qubits,
            other.num_qubits
        );

        let mut parity =
            false;

        for index in
            0..self.num_qubits
        {
            parity ^=
                self.x[index]
                    && other.z[index];

            parity ^=
                self.z[index]
                    && other.x[index];
        }

        parity as u8
    }

    /// Multiplies two Pauli strings while ignoring global phase.
    pub fn multiply(
        &self,
        other: &Self,
    ) -> Result<Self, StabilizerError> {
        self.check_compatible(other)?;

        let mut result =
            Self::identity(
                self.num_qubits,
            );

        for index in
            0..self.num_qubits
        {
            result.x[index] =
                self.x[index]
                    ^ other.x[index];

            result.z[index] =
                self.z[index]
                    ^ other.z[index];
        }

        Ok(result)
    }

    fn check_qubit(
        &self,
        qubit: QubitIndex,
    ) -> Result<(), StabilizerError> {
        if qubit.index()
            >= self.num_qubits
        {
            return Err(
                StabilizerError::QubitOutOfRange {
                    qubit,
                    num_qubits:
                        self.num_qubits,
                },
            );
        }

        Ok(())
    }

    fn check_compatible(
        &self,
        other: &Self,
    ) -> Result<(), StabilizerError> {
        if self.num_qubits
            != other.num_qubits
        {
            return Err(
                StabilizerError::QubitCountMismatch {
                    first:
                        self.num_qubits,
                    second:
                        other.num_qubits,
                },
            );
        }

        Ok(())
    }
}

impl fmt::Display
    for PauliString
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        for index in
            0..self.num_qubits
        {
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

    pub const fn id(
        &self,
    ) -> usize {
        self.id
    }

    pub fn operator(
        &self,
    ) -> &PauliString {
        &self.operator
    }

    pub fn weight(
        &self,
    ) -> usize {
        self.operator.weight()
    }
}

// -----------------------------------------------------------------------------
// Stabilizer group
// -----------------------------------------------------------------------------

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
                StabilizerError::ZeroQubits,
            );
        }

        Ok(Self {
            num_qubits,
            generators: Vec::new(),
        })
    }

    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.num_qubits
    }

    pub fn generators(
        &self,
    ) -> &[StabilizerGenerator] {
        &self.generators
    }

    pub fn len(
        &self,
    ) -> usize {
        self.generators.len()
    }

    pub fn is_empty(
        &self,
    ) -> bool {
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
                    first:
                        self.num_qubits,
                    second:
                        generator
                            .operator()
                            .num_qubits(),
                },
            );
        }

        if self.generators.iter()
            .any(|existing| {
                existing.id()
                    == generator.id()
            })
        {
            return Err(
                StabilizerError::DuplicateGenerator {
                    id:
                        generator.id(),
                },
            );
        }

        for existing
            in &self.generators
        {
            if generator
                .operator()
                .anticommutes_with(
                    existing.operator(),
                )?
            {
                return Err(
                    StabilizerError::NonCommutingGenerators {
                        first:
                            existing.id(),
                        second:
                            generator.id(),
                    },
                );
            }
        }

        self.generators
            .push(generator);

        Ok(())
    }

    pub fn validate(
        &self,
    ) -> Result<(), StabilizerError> {
        let mut ids =
            BTreeSet::new();

        for generator
            in &self.generators
        {
            if !ids.insert(
                generator.id(),
            ) {
                return Err(
                    StabilizerError::DuplicateGenerator {
                        id:
                            generator.id(),
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
                        first:
                            self.num_qubits,
                        second:
                            generator
                                .operator()
                                .num_qubits(),
                    },
                );
            }
        }

        for i in
            0..self.generators.len()
        {
            for j in
                (i + 1)..self.generators.len()
            {
                if self.generators[i]
                    .operator()
                    .anticommutes_with(
                        self.generators[j]
                            .operator(),
                    )?
                {
                    return Err(
                        StabilizerError::NonCommutingGenerators {
                            first:
                                self.generators[i]
                                    .id(),
                            second:
                                self.generators[j]
                                    .id(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Exact stabilizer membership
    // -------------------------------------------------------------------------

    /// Returns whether `operator` belongs to the stabilizer group.
    ///
    /// The stabilizer group generated by `r` independent generators contains
    /// up to 2^r distinct phase-free Pauli operators.
    ///
    /// This implementation enumerates generator products exactly. It is
    /// intentionally simple and deterministic and is suitable for validation
    /// and small-to-medium codes.
    ///
    /// Larger codes should use binary-symplectic Gaussian elimination.
    pub fn contains(
        &self,
        operator: &PauliString,
    ) -> Result<bool, StabilizerError> {
        if operator.num_qubits()
            != self.num_qubits
        {
            return Err(
                StabilizerError::QubitCountMismatch {
                    first:
                        self.num_qubits,
                    second:
                        operator
                            .num_qubits(),
                },
            );
        }

        // The identity is always in the stabilizer group.
        if operator.is_identity() {
            return Ok(true);
        }

        let generator_count =
            self.generators.len();

        // Enumerate every subset of generators.
        //
        // The group generated by commuting Pauli generators consists of
        // products of arbitrary subsets of those generators.
        if generator_count
            >= usize::BITS as usize
        {
            return Err(
                StabilizerError::MembershipSearchTooLarge {
                    generators:
                        generator_count,
                },
            );
        }

        let combinations =
            1usize
                << generator_count;

        for mask in 0..combinations
        {
            let mut product =
                PauliString::identity(
                    self.num_qubits,
                );

            for index in
                0..generator_count
            {
                if (mask
                    & (1usize << index))
                    != 0
                {
                    product =
                        product.multiply(
                            self.generators
                                [index]
                                .operator(),
                        )?;
                }
            }

            if product == *operator {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Returns the stabilizer element generated by the selected generator
    /// IDs.
    pub fn product(
        &self,
        indices: &[usize],
    ) -> Result<PauliString, StabilizerError> {
        let mut result =
            PauliString::identity(
                self.num_qubits,
            );

        for &index in indices {
            let generator =
                self.generators
                    .iter()
                    .find(
                        |generator| {
                            generator.id()
                                == index
                        },
                    )
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

    // -------------------------------------------------------------------------
    // Syndrome
    // -------------------------------------------------------------------------

    pub fn syndrome(
        &self,
        error: &PauliString,
    ) -> Result<Syndrome, StabilizerError> {
        if error.num_qubits()
            != self.num_qubits
        {
            return Err(
                StabilizerError::QubitCountMismatch {
                    first:
                        self.num_qubits,
                    second:
                        error.num_qubits(),
                },
            );
        }

        let mut bits =
            Vec::with_capacity(
                self.generators.len(),
            );

        for generator
            in &self.generators
        {
            bits.push(
                generator
                    .operator()
                    .anticommutes_with(
                        error,
                    )?,
            );
        }

        Ok(Syndrome {
            bits,
        })
    }
}

// -----------------------------------------------------------------------------
// Syndrome
// -----------------------------------------------------------------------------

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

    pub fn bits(
        &self,
    ) -> &[bool] {
        &self.bits
    }

    pub fn len(
        &self,
    ) -> usize {
        self.bits.len()
    }

    pub fn is_empty(
        &self,
    ) -> bool {
        self.bits.is_empty()
    }

    pub fn triggered_count(
        &self,
    ) -> usize {
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

    pub fn is_trivial(
        &self,
    ) -> bool {
        self.triggered_count() == 0
    }
}

// -----------------------------------------------------------------------------
// Logical operator helpers
// -----------------------------------------------------------------------------

pub fn commutes_with_stabilizer_group(
    operator: &PauliString,
    group: &StabilizerGroup,
) -> Result<bool, StabilizerError> {
    if operator.num_qubits()
        != group.num_qubits()
    {
        return Err(
            StabilizerError::QubitCountMismatch {
                first:
                    group.num_qubits(),
                second:
                    operator.num_qubits(),
            },
        );
    }

    for generator
        in group.generators()
    {
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

pub fn logical_operators_anticommute(
    first: &PauliString,
    second: &PauliString,
) -> Result<bool, StabilizerError> {
    first.anticommutes_with(
        second,
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

    MembershipSearchTooLarge {
        generators: usize,
    },
}

impl fmt::Display
    for StabilizerError
{
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

            Self::IdentityGenerator {
                id,
            } => {
                write!(
                    f,
                    "stabilizer generator {id} cannot be identity"
                )
            }

            Self::DuplicateGenerator {
                id,
            } => {
                write!(
                    f,
                    "stabilizer generator {id} already exists"
                )
            }

            Self::UnknownGenerator {
                id,
            } => {
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

            Self::MembershipSearchTooLarge {
                generators,
            } => {
                write!(
                    f,
                    "exact stabilizer membership search is too large for {generators} generators"
                )
            }
        }
    }
}

impl std::error::Error
    for StabilizerError
{
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn three_qubit_group()
        -> StabilizerGroup
    {
        let mut group =
            StabilizerGroup::new(3)
                .unwrap();

        group
            .add_generator(
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
    fn identity_is_in_stabilizer_group()
    {
        let group =
            three_qubit_group();

        assert!(
            group
                .contains(
                    &PauliString::identity(3)
                )
                .unwrap()
        );
    }

    #[test]
    fn generator_is_in_group()
    {
        let group =
            three_qubit_group();

        let operator =
            PauliString::from_paulis(
                &[
                    Pauli::Z,
                    Pauli::Z,
                    Pauli::I,
                ],
            );

        assert!(
            group
                .contains(&operator)
                .unwrap()
        );
    }

    #[test]
    fn product_of_generators_is_in_group()
    {
        let group =
            three_qubit_group();

        let operator =
            PauliString::from_paulis(
                &[
                    Pauli::Z,
                    Pauli::I,
                    Pauli::Z,
                ],
            );

        assert!(
            group
                .contains(&operator)
                .unwrap()
        );
    }

    #[test]
    fn unrelated_operator_is_not_in_group()
    {
        let group =
            three_qubit_group();

        let operator =
            PauliString::from_paulis(
                &[
                    Pauli::X,
                    Pauli::I,
                    Pauli::I,
                ],
            );

        assert!(
            !group
                .contains(&operator)
                .unwrap()
        );
    }

    #[test]
    fn syndrome_detects_anticommutation()
    {
        let group =
            three_qubit_group();

        let error =
            PauliString::from_paulis(
                &[
                    Pauli::X,
                    Pauli::I,
                    Pauli::I,
                ],
            );

        let syndrome =
            group
                .syndrome(&error)
                .unwrap();

        assert_eq!(
            syndrome.bits(),
            &[true, false]
        );
    }

    #[test]
    fn incompatible_membership_is_rejected()
    {
        let group =
            three_qubit_group();

        let operator =
            PauliString::identity(2);

        assert!(matches!(
            group.contains(
                &operator
            ),
            Err(
                StabilizerError::
                    QubitCountMismatch {
                        first: 3,
                        second: 2
                    }
            )
        ));
    }

    #[test]
    fn generators_commute()
    {
        let group =
            three_qubit_group();

        assert!(
            group.validate().is_ok()
        );
    }
}