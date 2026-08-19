//! Zamani Quantum Error Correction — Stabilizer Algebra.
//!
//! Provides:
//! - Pauli operators
//! - Pauli strings
//! - Binary-symplectic representation
//! - Stabilizer generators
//! - Stabilizer groups
//! - Exact commutation checks
//! - Syndrome extraction
//! - Polynomial-time stabilizer membership using GF(2) elimination
//!
//! Global Pauli phase is intentionally ignored. This is sufficient for
//! stabilizer commutation, syndrome extraction, stabilizer membership,
//! logical-operator validation, and code-distance calculations.

use std::collections::BTreeSet;
use std::fmt;

// ============================================================================
// Qubit index
// ============================================================================

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

// ============================================================================
// Pauli
// ============================================================================

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

    /// Returns true when the two single-qubit Paulis anticommute.
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

    /// Multiplies two Paulis while ignoring global phase.
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

    /// Constructs a Pauli from its binary-symplectic X/Z bits.
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

// ============================================================================
// Pauli string
// ============================================================================

/// A multi-qubit Pauli operator represented in binary-symplectic form.
///
/// For n qubits:
///
///     P = [x | z]
///
/// where each x/z vector has n bits.
///
/// Mapping:
///
///     I = (0,0)
///     X = (1,0)
///     Y = (1,1)
///     Z = (0,1)
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
    /// Creates an n-qubit identity.
    pub fn identity(
        num_qubits: usize,
    ) -> Self {
        Self {
            num_qubits,
            x: vec![false; num_qubits],
            z: vec![false; num_qubits],
        }
    }

    /// Creates a Pauli string from explicit Pauli operators.
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

    /// Creates a Pauli string directly from X/Z binary vectors.
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

    /// Returns the Pauli acting on one qubit.
    pub fn pauli_at(
        &self,
        qubit: QubitIndex,
    ) -> Result<Pauli, StabilizerError> {
        self.check_qubit(qubit)?;

        let index =
            qubit.index();

        Ok(Pauli::from_bits(
            self.x[index],
            self.z[index],
        ))
    }

    /// Sets the Pauli acting on one qubit.
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

    /// Returns the number of non-identity qubits.
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

    /// Returns the support of this Pauli operator.
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

    /// Binary symplectic inner product.
    ///
    ///     <P,Q> = xP·zQ + zP·xQ mod 2
    ///
    /// 0 => commute
    /// 1 => anticommute
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

    /// Multiplies two Pauli strings while ignoring global phase.
    ///
    /// In the binary-symplectic representation this is simply XOR.
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

// ============================================================================
// Stabilizer generator
// ============================================================================

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

// ============================================================================
// Stabilizer group
// ============================================================================

#[derive(
    Debug,
    Clone,
)]
pub struct StabilizerGroup {
    num_qubits: usize,
    generators: Vec<StabilizerGenerator>,
}

impl StabilizerGroup {
    /// Creates an empty stabilizer group over `num_qubits`.
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

    /// Adds a stabilizer generator.
    ///
    /// The generator must:
    /// - have the same number of qubits;
    /// - have a unique ID;
    /// - not be identity;
    /// - commute with every existing generator.
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

        if self.generators
            .iter()
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

    /// Validates the entire stabilizer group.
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

            if generator
                .operator()
                .is_identity()
            {
                return Err(
                    StabilizerError::IdentityGenerator {
                        id:
                            generator.id(),
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

    // ------------------------------------------------------------------------
    // Stabilizer membership
    // ------------------------------------------------------------------------

    /// Determines whether an operator belongs to the stabilizer group.
    ///
    /// The calculation is performed over GF(2).
    ///
    /// Each Pauli is represented as:
    ///
    ///     [x | z]
    ///
    /// Multiplication corresponds to XOR, so membership is equivalent to
    /// asking whether the target vector lies in the binary span of the
    /// generator vectors.
    ///
    /// This avoids the exponential 2^r enumeration previously used here.
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

        // Identity is always a member of the stabilizer group.
        if operator.is_identity() {
            return Ok(true);
        }

        if self.generators.is_empty() {
            return Ok(false);
        }

        let width =
            self.num_qubits * 2;

        // Generator matrix.
        let mut rows: Vec<Vec<bool>> =
            self.generators
                .iter()
                .map(|generator| {
                    let mut row =
                        Vec::with_capacity(
                            width,
                        );

                    row.extend_from_slice(
                        generator
                            .operator()
                            .x_bits(),
                    );

                    row.extend_from_slice(
                        generator
                            .operator()
                            .z_bits(),
                    );

                    row
                })
                .collect();

        let mut pivot_columns =
            Vec::new();

        let mut pivot_row =
            0usize;

        // Gaussian elimination over GF(2).
        for column in 0..width {
            let pivot =
                (pivot_row..rows.len())
                    .find(
                        |&row| {
                            rows[row][column]
                        },
                    );

            let Some(pivot) = pivot
            else {
                continue;
            };

            rows.swap(
                pivot_row,
                pivot,
            );

            for row in
                0..rows.len()
            {
                if row == pivot_row {
                    continue;
                }

                if rows[row][column] {
                    let pivot_data =
                        rows[pivot_row]
                            .clone();

                    xor_rows(
                        &mut rows[row],
                        &pivot_data,
                    );
                }
            }

            pivot_columns.push(
                column,
            );

            pivot_row += 1;

            if pivot_row
                == rows.len()
            {
                break;
            }
        }

        // Target vector.
        let mut target =
            Vec::with_capacity(
                width,
            );

        target.extend_from_slice(
            operator.x_bits(),
        );

        target.extend_from_slice(
            operator.z_bits(),
        );

        // Reduce target against the row-echelon basis.
        for (row_index, &column)
            in pivot_columns
                .iter()
                .enumerate()
        {
            if target[column] {
                let row =
                    rows[row_index]
                        .clone();

                xor_rows(
                    &mut target,
                    &row,
                );
            }
        }

        // Target belongs to the span iff the complete vector is zero.
        Ok(
            !target
                .iter()
                .any(|&bit| bit)
        )
    }

    // ------------------------------------------------------------------------
    // Stabilizer products
    // ------------------------------------------------------------------------

    /// Returns the product of the stabilizer generators identified by ID.
    pub fn product(
        &self,
        indices: &[usize],
    ) -> Result<PauliString, StabilizerError> {
        let mut result =
            PauliString::identity(
                self.num_qubits,
            );

        for &id in indices {
            let generator =
                self.generators
                    .iter()
                    .find(
                        |generator| {
                            generator.id()
                                == id
                        },
                    )
                    .ok_or(
                        StabilizerError::UnknownGenerator {
                            id,
                        },
                    )?;

            result =
                result.multiply(
                    generator.operator(),
                )?;
        }

        Ok(result)
    }

    // ------------------------------------------------------------------------
    // Syndrome
    // ------------------------------------------------------------------------

    /// Calculates the syndrome produced by a Pauli error.
    ///
    /// A syndrome bit is:
    ///
    ///     0 -> error commutes with stabilizer
    ///     1 -> error anticommutes with stabilizer
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

// ============================================================================
// Syndrome
// ============================================================================

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

// ============================================================================
// Logical-operator helpers
// ============================================================================

/// Returns true if an operator commutes with every stabilizer generator.
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

/// Returns true when two Pauli operators anticommute.
pub fn logical_operators_anticommute(
    first: &PauliString,
    second: &PauliString,
) -> Result<bool, StabilizerError> {
    first.anticommutes_with(
        second,
    )
}

// ============================================================================
// GF(2) helpers
// ============================================================================

/// XORs two GF(2) rows.
fn xor_rows(
    destination: &mut [bool],
    source: &[bool],
) {
    debug_assert_eq!(
        destination.len(),
        source.len()
    );

    for (lhs, rhs) in
        destination
            .iter_mut()
            .zip(source.iter())
    {
        *lhs ^= *rhs;
    }
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
                    "qubit-count mismatch: {first} != {second}"
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
        }
    }
}

impl std::error::Error
    for StabilizerError
{
}

// ============================================================================
// Tests
// ============================================================================

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
    fn identity_is_in_group() {
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
    fn generator_is_in_group() {
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
    fn generator_product_is_in_group() {
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
    fn unrelated_operator_is_not_in_group() {
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
    fn syndrome_detects_error() {
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
    fn commuting_generators_are_accepted() {
        let group =
            three_qubit_group();

        assert!(
            group.validate().is_ok()
        );
    }

    #[test]
    fn non_commuting_generator_is_rejected() {
        let mut group =
            StabilizerGroup::new(2)
                .unwrap();

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(
                        &[
                            Pauli::X,
                            Pauli::I,
                        ],
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
                        &[
                            Pauli::Z,
                            Pauli::I,
                        ],
                    ),
                )
                .unwrap(),
            );

        assert!(matches!(
            result,
            Err(
                StabilizerError::
                    NonCommutingGenerators {
                        first: 0,
                        second: 1
                    }
            )
        ));
    }

    #[test]
    fn wrong_qubit_count_is_rejected() {
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
    fn pauli_multiplication_works() {
        let x =
            PauliString::from_paulis(
                &[Pauli::X],
            );

        let z =
            PauliString::from_paulis(
                &[Pauli::Z],
            );

        let result =
            x.multiply(&z)
                .unwrap();

        assert_eq!(
            result,
            PauliString::from_paulis(
                &[Pauli::Y],
            )
        );
    }

    #[test]
    fn symplectic_commutation_works() {
        let x =
            PauliString::from_paulis(
                &[Pauli::X],
            );

        let z =
            PauliString::from_paulis(
                &[Pauli::Z],
            );

        assert!(
            x.anticommutes_with(&z)
                .unwrap()
        );

        assert!(
            !x.commutes_with(&z)
                .unwrap()
        );
    }

    #[test]
    fn support_and_weight_are_correct() {
        let operator =
            PauliString::from_paulis(
                &[
                    Pauli::I,
                    Pauli::X,
                    Pauli::I,
                    Pauli::Z,
                    Pauli::Y,
                ],
            );

        assert_eq!(
            operator.weight(),
            3
        );

        assert_eq!(
            operator.support(),
            vec![
                QubitIndex(1),
                QubitIndex(3),
                QubitIndex(4),
            ]
        );
    }
}