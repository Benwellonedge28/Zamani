//! Zamani Quantum Error Correction — Stabilizer Algebra.
//!
//! Production-grade stabilizer infrastructure.
//!
//! Guarantees:
//! - Binary-symplectic Pauli representation.
//! - Checked dimensional compatibility.
//! - No panic-based validation.
//! - Deterministic stabilizer ordering.
//! - Commutation and anti-commutation verification.
//! - GF(2) stabilizer membership.
//! - Stabilizer rank.
//! - Syndrome extraction.
//! - Logical-normalizer validation.
//! - Configurable resource limits.
//! - Unified `QecError` integration.
//! - Overflow-safe resource calculations.
//!
//! Global Pauli phase is intentionally ignored.
//!
//! This representation therefore models Pauli operators modulo global phase,
//! which is sufficient for:
//! - stabilizer commutation;
//! - syndrome extraction;
//! - stabilizer membership;
//! - logical-operator classification;
//! - code-distance calculations.
//!
//! For full phase-sensitive Clifford simulation, a separate phase-aware layer
//! should be used.

use core::fmt;
use std::collections::BTreeSet;

use super::errors::{QecError, QecResult};
use super::limits::{LimitError, QecLimits};

// ============================================================================
// Qubit index
// ============================================================================

/// Stable identifier for a physical qubit.
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
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    #[must_use]
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

/// Single-qubit Pauli operator.
///
/// The binary-symplectic mapping is:
///
/// ```text
/// I = (0,0)
/// X = (1,0)
/// Y = (1,1)
/// Z = (0,1)
/// ```
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}

impl Pauli {
    #[must_use]
    pub const fn is_identity(self) -> bool {
        matches!(self, Self::I)
    }

    #[must_use]
    pub const fn is_non_identity(self) -> bool {
        !self.is_identity()
    }

    #[must_use]
    pub const fn has_x_component(self) -> bool {
        matches!(self, Self::X | Self::Y)
    }

    #[must_use]
    pub const fn has_z_component(self) -> bool {
        matches!(self, Self::Z | Self::Y)
    }

    /// Returns whether two single-qubit Paulis anticommute.
    #[must_use]
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

    /// Multiplies two Paulis modulo global phase.
    #[must_use]
    pub const fn multiply(
        self,
        other: Self,
    ) -> Self {
        use Pauli::*;

        match (self, other) {
            (I, p) | (p, I) => p,

            (X, X) | (Y, Y) | (Z, Z) => I,

            (X, Y) | (Y, X) => Z,

            (X, Z) | (Z, X) => Y,

            (Y, Z) | (Z, Y) => X,
        }
    }

    /// Converts binary-symplectic bits into a Pauli.
    #[must_use]
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

/// Multi-qubit Pauli operator in binary-symplectic form.
///
/// ```text
/// P = [x | z]
/// ```
///
/// where both vectors contain one entry per qubit.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct PauliString {
    num_qubits: usize,
    x: Vec<bool>,
    z: Vec<bool>,
}

impl PauliString {
    /// Creates the identity on `num_qubits`.
    #[must_use]
    pub fn identity(
        num_qubits: usize,
    ) -> Self {
        Self {
            num_qubits,
            x: vec![false; num_qubits],
            z: vec![false; num_qubits],
        }
    }

    /// Creates a Pauli string from single-qubit Paulis.
    #[must_use]
    pub fn from_paulis(
        paulis: &[Pauli],
    ) -> Self {
        let mut result =
            Self::identity(paulis.len());

        for (index, &pauli) in
            paulis.iter().enumerate()
        {
            result.x[index] =
                pauli.has_x_component();

            result.z[index] =
                pauli.has_z_component();
        }

        result
    }

    /// Creates a Pauli string from binary-symplectic vectors.
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

    #[must_use]
    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.num_qubits
    }

    #[must_use]
    pub fn x_bits(
        &self,
    ) -> &[bool] {
        &self.x
    }

    #[must_use]
    pub fn z_bits(
        &self,
    ) -> &[bool] {
        &self.z
    }

    /// Returns the Pauli acting on a particular qubit.
    pub fn pauli_at(
        &self,
        qubit: QubitIndex,
    ) -> Result<Pauli, StabilizerError> {
        self.check_qubit(qubit)?;

        let index = qubit.index();

        Ok(Pauli::from_bits(
            self.x[index],
            self.z[index],
        ))
    }

    /// Sets the Pauli acting on a particular qubit.
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

    /// Returns the operator weight.
    #[must_use]
    pub fn weight(
        &self,
    ) -> usize {
        self.x
            .iter()
            .zip(self.z.iter())
            .filter(|(x, z)| **x || **z)
            .count()
    }

    #[must_use]
    pub fn is_identity(
        &self,
    ) -> bool {
        self.x
            .iter()
            .zip(self.z.iter())
            .all(|(x, z)| !*x && !*z)
    }

    /// Returns the non-identity support.
    #[must_use]
    pub fn support(
        &self,
    ) -> Vec<QubitIndex> {
        self.x
            .iter()
            .zip(self.z.iter())
            .enumerate()
            .filter_map(
                |(index, (x, z))| {
                    if *x || *z {
                        Some(QubitIndex(index))
                    } else {
                        None
                    }
                },
            )
            .collect()
    }

    /// Checked binary-symplectic inner product.
    ///
    /// ```text
    /// <P,Q> =
    /// xP · zQ + zP · xQ mod 2
    /// ```
    ///
    /// Returns:
    ///
    /// * `0` — commute
    /// * `1` — anticommute
    pub fn try_symplectic_product(
        &self,
        other: &Self,
    ) -> Result<u8, StabilizerError> {
        self.check_compatible(other)?;

        let mut parity = false;

        for index in 0..self.num_qubits {
            parity ^=
                self.x[index] && other.z[index];

            parity ^=
                self.z[index] && other.x[index];
        }

        Ok(u8::from(parity))
    }

    /// Backward-compatible checked symplectic operation.
    ///
    /// This deliberately returns `Result` instead of relying on
    /// `debug_assert!` or unchecked indexing.
    pub fn symplectic_product(
        &self,
        other: &Self,
    ) -> Result<u8, StabilizerError> {
        self.try_symplectic_product(other)
    }

    pub fn commutes_with(
        &self,
        other: &Self,
    ) -> Result<bool, StabilizerError> {
        Ok(
            self.try_symplectic_product(other)? == 0
        )
    }

    pub fn anticommutes_with(
        &self,
        other: &Self,
    ) -> Result<bool, StabilizerError> {
        Ok(
            self.try_symplectic_product(other)? == 1
        )
    }

    /// Multiplies two Pauli strings modulo global phase.
    ///
    /// Binary-symplectic multiplication is XOR.
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

    /// Converts the operator into a deterministic Pauli vector.
    #[must_use]
    pub fn to_paulis(
        &self,
    ) -> Vec<Pauli> {
        self.x
            .iter()
            .zip(self.z.iter())
            .map(|(x, z)| {
                Pauli::from_bits(*x, *z)
            })
            .collect()
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

// ============================================================================
// Stabilizer generator
// ============================================================================

/// A named stabilizer generator.
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
                StabilizerError::IdentityGenerator { id },
            );
        }

        Ok(Self {
            id,
            operator,
        })
    }

    #[must_use]
    pub const fn id(
        &self,
    ) -> usize {
        self.id
    }

    #[must_use]
    pub fn operator(
        &self,
    ) -> &PauliString {
        &self.operator
    }

    #[must_use]
    pub fn weight(
        &self,
    ) -> usize {
        self.operator.weight()
    }
}

// ============================================================================
// Stabilizer group
// ============================================================================

/// A commuting stabilizer-generator set.
#[derive(
    Debug,
    Clone,
)]
pub struct StabilizerGroup {
    num_qubits: usize,
    generators: Vec<StabilizerGenerator>,
}

impl StabilizerGroup {
    /// Creates an empty stabilizer group.
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

    /// Creates an empty group after validating its resource policy.
    pub fn new_with_limits(
        num_qubits: usize,
        limits: &QecLimits,
    ) -> Result<Self, StabilizerError> {
        limits
            .validate()
            .map_err(StabilizerError::InvalidLimits)?;

        if num_qubits == 0 {
            return Err(
                StabilizerError::ZeroQubits,
            );
        }

        if num_qubits > limits.max_qubits {
            return Err(
                StabilizerError::QubitLimitExceeded {
                    requested: num_qubits,
                    maximum: limits.max_qubits,
                },
            );
        }

        Ok(Self {
            num_qubits,
            generators: Vec::new(),
        })
    }

    #[must_use]
    pub const fn num_qubits(
        &self,
    ) -> usize {
        self.num_qubits
    }

    #[must_use]
    pub fn generators(
        &self,
    ) -> &[StabilizerGenerator] {
        &self.generators
    }

    #[must_use]
    pub fn len(
        &self,
    ) -> usize {
        self.generators.len()
    }

    #[must_use]
    pub fn is_empty(
        &self,
    ) -> bool {
        self.generators.is_empty()
    }

    /// Adds a generator without a separate resource policy.
    pub fn add_generator(
        &mut self,
        generator: StabilizerGenerator,
    ) -> Result<(), StabilizerError> {
        self.add_generator_with_limits(
            generator,
            &QecLimits {
                max_qubits: self.num_qubits,
                max_stabilizers: usize::MAX,
                max_stabilizer_weight: usize::MAX,
                ..QecLimits::new()
            },
        )
    }

    /// Adds a generator while enforcing `QecLimits`.
    pub fn add_generator_with_limits(
        &mut self,
        generator: StabilizerGenerator,
        limits: &QecLimits,
    ) -> Result<(), StabilizerError> {
        limits
            .validate()
            .map_err(StabilizerError::InvalidLimits)?;

        let generator_qubits =
            generator.operator().num_qubits();

        if generator_qubits != self.num_qubits {
            return Err(
                StabilizerError::QubitCountMismatch {
                    first: self.num_qubits,
                    second: generator_qubits,
                },
            );
        }

        if self.generators.len()
            >= limits.max_stabilizers
        {
            return Err(
                StabilizerError::StabilizerLimitExceeded {
                    requested: self.generators.len() + 1,
                    maximum: limits.max_stabilizers,
                },
            );
        }

        let weight =
            generator.weight();

        if weight > limits.max_stabilizer_weight {
            return Err(
                StabilizerError::StabilizerWeightLimitExceeded {
                    id: generator.id(),
                    requested: weight,
                    maximum: limits.max_stabilizer_weight,
                },
            );
        }

        if self.generators.iter().any(
            |existing| {
                existing.id() == generator.id()
            },
        ) {
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

        // Keep generator order deterministic even when callers add IDs
        // in arbitrary order.
        self.generators.sort_by_key(
            StabilizerGenerator::id,
        );

        Ok(())
    }

    /// Validates the complete stabilizer system.
    pub fn validate(
        &self,
    ) -> Result<(), StabilizerError> {
        self.validate_with_limits(
            &QecLimits {
                max_qubits: self.num_qubits,
                max_stabilizers: usize::MAX,
                max_stabilizer_weight: usize::MAX,
                ..QecLimits::new()
            },
        )
    }

    /// Validates the complete stabilizer system against a resource policy.
    pub fn validate_with_limits(
        &self,
        limits: &QecLimits,
    ) -> Result<(), StabilizerError> {
        limits
            .validate()
            .map_err(StabilizerError::InvalidLimits)?;

        if self.num_qubits == 0 {
            return Err(
                StabilizerError::ZeroQubits,
            );
        }

        if self.num_qubits > limits.max_qubits {
            return Err(
                StabilizerError::QubitLimitExceeded {
                    requested: self.num_qubits,
                    maximum: limits.max_qubits,
                },
            );
        }

        if self.generators.len()
            > limits.max_stabilizers
        {
            return Err(
                StabilizerError::StabilizerLimitExceeded {
                    requested: self.generators.len(),
                    maximum: limits.max_stabilizers,
                },
            );
        }

        let mut ids = BTreeSet::new();

        for generator in &self.generators {
            if !ids.insert(generator.id()) {
                return Err(
                    StabilizerError::DuplicateGenerator {
                        id: generator.id(),
                    },
                );
            }

            let operator =
                generator.operator();

            if operator.num_qubits()
                != self.num_qubits
            {
                return Err(
                    StabilizerError::QubitCountMismatch {
                        first: self.num_qubits,
                        second: operator.num_qubits(),
                    },
                );
            }

            if operator.is_identity() {
                return Err(
                    StabilizerError::IdentityGenerator {
                        id: generator.id(),
                    },
                );
            }

            if operator.weight()
                > limits.max_stabilizer_weight
            {
                return Err(
                    StabilizerError::StabilizerWeightLimitExceeded {
                        id: generator.id(),
                        requested: operator.weight(),
                        maximum: limits.max_stabilizer_weight,
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

    // ========================================================================
    // Rank
    // ========================================================================

    /// Returns the GF(2) rank of the stabilizer generators.
    ///
    /// This is not necessarily equal to the number of supplied generators:
    /// redundant generators can exist.
    pub fn rank(
        &self,
    ) -> Result<usize, StabilizerError> {
        self.rank_with_limits(&QecLimits {
            max_qubits: self.num_qubits,
            max_stabilizers: usize::MAX,
            max_stabilizer_weight: usize::MAX,
            ..QecLimits::new()
        })
    }

    /// Returns the GF(2) rank while enforcing resource limits.
    pub fn rank_with_limits(
        &self,
        limits: &QecLimits,
    ) -> Result<usize, StabilizerError> {
        self.validate_with_limits(limits)?;

        if self.generators.is_empty() {
            return Ok(0);
        }

        let width = self
            .num_qubits
            .checked_mul(2)
            .ok_or(
                StabilizerError::ArithmeticOverflow {
                    operation:
                        "2 * num_qubits",
                },
            )?;

        let mut rows =
            self.generator_rows_packed(width)?;

        Ok(gf2_rank(
            &mut rows,
            width,
        ))
    }

    /// Number of encoded logical qubits for an independent stabilizer set.
    ///
    /// For a valid stabilizer code:
    ///
    /// ```text
    /// k = n - rank(S)
    /// ```
    pub fn logical_qubit_count(
        &self,
    ) -> Result<usize, StabilizerError> {
        let rank = self.rank()?;

        self.num_qubits
            .checked_sub(rank)
            .ok_or(
                StabilizerError::InvalidRank {
                    rank,
                    num_qubits: self.num_qubits,
                },
            )
    }

    // ========================================================================
    // Membership
    // ========================================================================

    /// Determines whether an operator belongs to the stabilizer group.
    pub fn contains(
        &self,
        operator: &PauliString,
    ) -> Result<bool, StabilizerError> {
        self.contains_with_limits(
            operator,
            &QecLimits {
                max_qubits: self.num_qubits,
                max_stabilizers: usize::MAX,
                max_stabilizer_weight: usize::MAX,
                ..QecLimits::new()
            },
        )
    }

    /// Determines stabilizer membership with resource enforcement.
    pub fn contains_with_limits(
        &self,
        operator: &PauliString,
        limits: &QecLimits,
    ) -> Result<bool, StabilizerError> {
        self.validate_with_limits(limits)?;

        if operator.num_qubits()
            != self.num_qubits
        {
            return Err(
                StabilizerError::QubitCountMismatch {
                    first: self.num_qubits,
                    second: operator.num_qubits(),
                },
            );
        }

        if operator.is_identity() {
            return Ok(true);
        }

        if self.generators.is_empty() {
            return Ok(false);
        }

        let width = self
            .num_qubits
            .checked_mul(2)
            .ok_or(
                StabilizerError::ArithmeticOverflow {
                    operation:
                        "2 * num_qubits",
                },
            )?;

        let mut rows =
            self.generator_rows_packed(width)?;

        let rank =
            gf2_reduce_rows(
                &mut rows,
                width,
            );

        let mut target =
            pack_bits(
                operator.x_bits(),
                operator.z_bits(),
            );

        // Reduce the target against the same deterministic basis.
        for row_index in 0..rank {
            let pivot =
                first_set_bit(
                    &rows[row_index],
                    width,
                );

            let Some(pivot) = pivot else {
                continue;
            };

            if get_bit(&target, pivot) {
                xor_packed(
                    &mut target,
                    &rows[row_index],
                );
            }
        }

        Ok(
            target
                .iter()
                .all(|word| *word == 0),
        )
    }

    // ========================================================================
    // Products
    // ========================================================================

    /// Returns the product of generators identified by ID.
    pub fn product(
        &self,
        indices: &[usize],
    ) -> Result<PauliString, StabilizerError> {
        self.validate()?;

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
                            generator.id() == id
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

    // ========================================================================
    // Syndrome
    // ========================================================================

    /// Computes the syndrome produced by a Pauli error.
    pub fn syndrome(
        &self,
        error: &PauliString,
    ) -> Result<Syndrome, StabilizerError> {
        self.syndrome_with_limits(
            error,
            &QecLimits {
                max_qubits: self.num_qubits,
                max_stabilizers: usize::MAX,
                max_stabilizer_weight: usize::MAX,
                ..QecLimits::new()
            },
        )
    }

    /// Computes a syndrome with explicit resource validation.
    pub fn syndrome_with_limits(
        &self,
        error: &PauliString,
        limits: &QecLimits,
    ) -> Result<Syndrome, StabilizerError> {
        self.validate_with_limits(limits)?;

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

        if self.generators.len()
            > limits.max_syndrome_events
        {
            return Err(
                StabilizerError::SyndromeLimitExceeded {
                    requested: self.generators.len(),
                    maximum: limits.max_syndrome_events,
                },
            );
        }

        let mut bits =
            Vec::with_capacity(
                self.generators.len(),
            );

        for generator in &self.generators {
            bits.push(
                generator
                    .operator()
                    .anticommutes_with(error)?,
            );
        }

        Ok(Syndrome { bits })
    }

    // ========================================================================
    // Logical-normalizer checks
    // ========================================================================

    /// Returns whether an operator belongs to the normalizer/centralizer
    /// of the stabilizer group.
    pub fn is_in_normalizer(
        &self,
        operator: &PauliString,
    ) -> Result<bool, StabilizerError> {
        if operator.num_qubits()
            != self.num_qubits
        {
            return Err(
                StabilizerError::QubitCountMismatch {
                    first: self.num_qubits,
                    second: operator.num_qubits(),
                },
            );
        }

        for generator in &self.generators {
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

    /// Returns whether an operator is a non-trivial logical Pauli:
    ///
    /// ```text
    /// normalizer(operator)
    /// AND
    /// NOT stabilizer(operator)
    /// ```
    pub fn is_nontrivial_logical(
        &self,
        operator: &PauliString,
    ) -> Result<bool, StabilizerError> {
        if !self.is_in_normalizer(operator)? {
            return Ok(false);
        }

        Ok(!self.contains(operator)?)
    }

    /// Validates a pair of logical operators.
    ///
    /// They must:
    /// - have compatible dimensions;
    /// - commute with all stabilizers;
    /// - anticommute with each other when requested.
    pub fn validate_logical_pair(
        &self,
        logical_x: &PauliString,
        logical_z: &PauliString,
    ) -> Result<(), StabilizerError> {
        if !self.is_in_normalizer(logical_x)? {
            return Err(
                StabilizerError::LogicalNotInNormalizer {
                    operator: logical_x.clone(),
                },
            );
        }

        if !self.is_in_normalizer(logical_z)? {
            return Err(
                StabilizerError::LogicalNotInNormalizer {
                    operator: logical_z.clone(),
                },
            );
        }

        if self.contains(logical_x)?
            || self.contains(logical_z)?
        {
            return Err(
                StabilizerError::LogicalOperatorIsStabilizer,
            );
        }

        if !logical_x.anticommutes_with(
            logical_z,
        )? {
            return Err(
                StabilizerError::LogicalOperatorsDoNotAnticommute,
            );
        }

        Ok(())
    }

    // ========================================================================
    // Internal packed GF(2) representation
    // ========================================================================

    fn generator_rows_packed(
        &self,
        width: usize,
    ) -> Result<Vec<Vec<u64>>, StabilizerError> {
        let words =
            width
                .checked_add(63)
                .ok_or(
                    StabilizerError::ArithmeticOverflow {
                        operation:
                            "width + 63",
                    },
                )?
                / 64;

        let row_bytes =
            words
                .checked_mul(
                    std::mem::size_of::<u64>(),
                )
                .ok_or(
                    StabilizerError::ArithmeticOverflow {
                        operation:
                            "packed GF(2) row bytes",
                    },
                )?;

        let total_bytes =
            row_bytes
                .checked_mul(
                    self.generators.len(),
                )
                .ok_or(
                    StabilizerError::ArithmeticOverflow {
                        operation:
                            "packed GF(2) matrix bytes",
                    },
                )?;

        // Prevent accidental host allocation overflow.
        if total_bytes
            > isize::MAX as usize
        {
            return Err(
                StabilizerError::AllocationTooLarge {
                    bytes: total_bytes,
                },
            );
        }

        let mut rows =
            Vec::with_capacity(
                self.generators.len(),
            );

        for generator in &self.generators {
            rows.push(
                pack_bits(
                    generator.operator().x_bits(),
                    generator.operator().z_bits(),
                ),
            );
        }

        Ok(rows)
    }
}

// ============================================================================
// Syndrome
// ============================================================================

/// Syndrome bits corresponding to stabilizer-generator order.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Syndrome {
    bits: Vec<bool>,
}

impl Syndrome {
    #[must_use]
    pub fn new(
        bits: Vec<bool>,
    ) -> Self {
        Self { bits }
    }

    #[must_use]
    pub fn bits(
        &self,
    ) -> &[bool] {
        &self.bits
    }

    #[must_use]
    pub fn len(
        &self,
    ) -> usize {
        self.bits.len()
    }

    #[must_use]
    pub fn is_empty(
        &self,
    ) -> bool {
        self.bits.is_empty()
    }

    #[must_use]
    pub fn triggered_count(
        &self,
    ) -> usize {
        self.bits
            .iter()
            .filter(|bit| **bit)
            .count()
    }

    pub fn triggered(
        &self,
    ) -> impl Iterator<Item = usize> + '_ {
        self.bits
            .iter()
            .enumerate()
            .filter_map(
                |(index, triggered)| {
                    triggered.then_some(index)
                },
            )
    }

    #[must_use]
    pub fn is_trivial(
        &self,
    ) -> bool {
        self.triggered_count() == 0
    }

    /// Returns a deterministic hash-independent compact bit representation.
    #[must_use]
    pub fn as_bytes(&self) -> Vec<u8> {
        let byte_count =
            self.bits.len()
                .saturating_add(7)
                / 8;

        let mut bytes =
            vec![0u8; byte_count];

        for (index, bit)
            in self.bits.iter().enumerate()
        {
            if *bit {
                bytes[index / 8] |=
                    1u8 << (index % 8);
            }
        }

        bytes
    }
}

// ============================================================================
// Logical helpers
// ============================================================================

/// Returns whether an operator commutes with every stabilizer.
pub fn commutes_with_stabilizer_group(
    operator: &PauliString,
    group: &StabilizerGroup,
) -> Result<bool, StabilizerError> {
    group.is_in_normalizer(operator)
}

/// Returns whether two Pauli operators anticommute.
pub fn logical_operators_anticommute(
    first: &PauliString,
    second: &PauliString,
) -> Result<bool, StabilizerError> {
    first.anticommutes_with(second)
}

// ============================================================================
// Packed GF(2) helpers
// ============================================================================

fn pack_bits(
    x: &[bool],
    z: &[bool],
) -> Vec<u64> {
    let width = x.len() + z.len();
    let words =
        (width.saturating_add(63)) / 64;

    let mut result =
        vec![0u64; words];

    for (index, bit) in
        x.iter().enumerate()
    {
        if *bit {
            set_bit(
                &mut result,
                index,
            );
        }
    }

    for (index, bit) in
        z.iter().enumerate()
    {
        if *bit {
            set_bit(
                &mut result,
                x.len() + index,
            );
        }
    }

    result
}

fn set_bit(
    words: &mut [u64],
    index: usize,
) {
    let word =
        index / 64;
    let bit =
        index % 64;

    if let Some(value) =
        words.get_mut(word)
    {
        *value |=
            1u64 << bit;
    }
}

fn get_bit(
    words: &[u64],
    index: usize,
) -> bool {
    let word =
        index / 64;
    let bit =
        index % 64;

    words
        .get(word)
        .map_or(
            false,
            |value| {
                (*value
                    & (1u64 << bit))
                    != 0
            },
        )
}

fn xor_packed(
    destination: &mut [u64],
    source: &[u64],
) {
    for (lhs, rhs) in
        destination
            .iter_mut()
            .zip(source.iter())
    {
        *lhs ^= *rhs;
    }
}

fn first_set_bit(
    words: &[u64],
    width: usize,
) -> Option<usize> {
    for (word_index, &word)
        in words.iter().enumerate()
    {
        if word == 0 {
            continue;
        }

        let bit =
            word.trailing_zeros()
                as usize;

        let index =
            word_index
                .checked_mul(64)?
                .checked_add(bit)?;

        if index < width {
            return Some(index);
        }

        return None;
    }

    None
}

/// Performs deterministic Gauss-Jordan elimination over GF(2).
///
/// Returns the rank.
fn gf2_reduce_rows(
    rows: &mut [Vec<u64>],
    width: usize,
) -> usize {
    let mut pivot_row = 0usize;

    for column in 0..width {
        let Some(pivot) =
            (pivot_row..rows.len())
                .find(
                    |row| {
                        get_bit(
                            &rows[*row],
                            column,
                        )
                    },
                )
        else {
            continue;
        };

        rows.swap(
            pivot_row,
            pivot,
        );

        for row in 0..rows.len() {
            if row == pivot_row {
                continue;
            }

            if get_bit(
                &rows[row],
                column,
            ) {
                // Clone only the packed row.
                // This is bounded by O(n/64), rather than O(n).
                let pivot_data =
                    rows[pivot_row].clone();

                xor_packed(
                    &mut rows[row],
                    &pivot_data,
                );
            }
        }

        pivot_row += 1;

        if pivot_row == rows.len() {
            break;
        }
    }

    pivot_row
}

fn gf2_rank(
    rows: &mut [Vec<u64>],
    width: usize,
) -> usize {
    gf2_reduce_rows(
        rows,
        width,
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

    StabilizerLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    QubitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    SyndromeLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    StabilizerWeightLimitExceeded {
        id: usize,
        requested: usize,
        maximum: usize,
    },

    InvalidLimits(
        LimitError,
    ),

    ArithmeticOverflow {
        operation: &'static str,
    },

    AllocationTooLarge {
        bytes: usize,
    },

    InvalidRank {
        rank: usize,
        num_qubits: usize,
    },

    LogicalNotInNormalizer {
        operator: PauliString,
    },

    LogicalOperatorIsStabilizer,

    LogicalOperatorsDoNotAnticommute,
}

impl fmt::Display
    for StabilizerError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroQubits =>
                write!(
                    f,
                    "stabilizer system must contain at least one qubit"
                ),

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } =>
                write!(
                    f,
                    "qubit {qubit} is outside a {num_qubits}-qubit system"
                ),

            Self::QubitCountMismatch {
                first,
                second,
            } =>
                write!(
                    f,
                    "qubit-count mismatch: {first} != {second}"
                ),

            Self::SymplecticDimensionMismatch {
                x,
                z,
            } =>
                write!(
                    f,
                    "symplectic X/Z dimensions differ: {x} != {z}"
                ),

            Self::IdentityGenerator { id } =>
                write!(
                    f,
                    "stabilizer generator {id} cannot be identity"
                ),

            Self::DuplicateGenerator { id } =>
                write!(
                    f,
                    "stabilizer generator {id} already exists"
                ),

            Self::UnknownGenerator { id } =>
                write!(
                    f,
                    "unknown stabilizer generator {id}"
                ),

            Self::NonCommutingGenerators {
                first,
                second,
            } =>
                write!(
                    f,
                    "stabilizer generators {first} and {second} do not commute"
                ),

            Self::StabilizerLimitExceeded {
                requested,
                maximum,
            } =>
                write!(
                    f,
                    "stabilizer count {requested} exceeds configured maximum {maximum}"
                ),

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } =>
                write!(
                    f,
                    "qubit count {requested} exceeds configured maximum {maximum}"
                ),

            Self::SyndromeLimitExceeded {
                requested,
                maximum,
            } =>
                write!(
                    f,
                    "syndrome count {requested} exceeds configured maximum {maximum}"
                ),

            Self::StabilizerWeightLimitExceeded {
                id,
                requested,
                maximum,
            } =>
                write!(
                    f,
                    "stabilizer {id} has weight {requested}, exceeding maximum {maximum}"
                ),

            Self::InvalidLimits(error) =>
                write!(
                    f,
                    "invalid QEC limits: {error}"
                ),

            Self::ArithmeticOverflow {
                operation,
            } =>
                write!(
                    f,
                    "arithmetic overflow while calculating {operation}"
                ),

            Self::AllocationTooLarge {
                bytes,
            } =>
                write!(
                    f,
                    "requested packed GF(2) allocation of {bytes} bytes is too large"
                ),

            Self::InvalidRank {
                rank,
                num_qubits,
            } =>
                write!(
                    f,
                    "invalid stabilizer rank {rank} for {num_qubits} qubits"
                ),

            Self::LogicalNotInNormalizer { .. } =>
                write!(
                    f,
                    "logical operator does not commute with the stabilizer group"
                ),

            Self::LogicalOperatorIsStabilizer =>
                write!(
                    f,
                    "logical operator is contained in the stabilizer group"
                ),

            Self::LogicalOperatorsDoNotAnticommute =>
                write!(
                    f,
                    "logical X and logical Z must anticommute"
                ),
        }
    }
}

impl std::error::Error
    for StabilizerError
{
}

// ============================================================================
// Unified QEC error integration
// ============================================================================

impl From<StabilizerError>
    for QecError
{
    fn from(
        error: StabilizerError,
    ) -> Self {
        match error {
            StabilizerError::ZeroQubits
            | StabilizerError::QubitOutOfRange { .. }
            | StabilizerError::QubitCountMismatch { .. }
            | StabilizerError::SymplecticDimensionMismatch { .. }
            | StabilizerError::IdentityGenerator { .. }
            | StabilizerError::DuplicateGenerator { .. }
            | StabilizerError::UnknownGenerator { .. }
            | StabilizerError::NonCommutingGenerators { .. }
            | StabilizerError::LogicalNotInNormalizer { .. }
            | StabilizerError::LogicalOperatorIsStabilizer
            | StabilizerError::LogicalOperatorsDoNotAnticommute => {
                QecError::invalid_stabilizer(
                    error.to_string(),
                )
            }

            StabilizerError::StabilizerLimitExceeded {
                requested,
                maximum,
            } =>
                QecError::resource_limit(
                    super::errors::ResourceKind::Stabilizers,
                    requested as u128,
                    maximum as u128,
                    error.to_string(),
                ),

            StabilizerError::QubitLimitExceeded {
                requested,
                maximum,
            } =>
                QecError::resource_limit(
                    super::errors::ResourceKind::Qubits,
                    requested as u128,
                    maximum as u128,
                    error.to_string(),
                ),

            StabilizerError::SyndromeLimitExceeded {
                requested,
                maximum,
            } =>
                QecError::resource_limit(
                    super::errors::ResourceKind::SyndromeEvents,
                    requested as u128,
                    maximum as u128,
                    error.to_string(),
                ),

            StabilizerError::StabilizerWeightLimitExceeded {
                requested,
                maximum,
                ..
            } =>
                QecError::resource_limit(
                    super::errors::ResourceKind::Stabilizers,
                    requested as u128,
                    maximum as u128,
                    error.to_string(),
                ),

            StabilizerError::InvalidLimits(_) =>
                QecError::invalid_input(
                    error.to_string(),
                ),

            StabilizerError::ArithmeticOverflow { .. } =>
                QecError::numerical_failure(
                    super::errors::NumericalOperation::IntegerConversion,
                    error.to_string(),
                ),

            StabilizerError::AllocationTooLarge { bytes } =>
                QecError::memory_limit(
                    bytes as u64,
                    u64::MAX,
                    error.to_string(),
                ),

            StabilizerError::InvalidRank { .. } =>
                QecError::invariant(
                    "stabilizer_rank <= num_qubits",
                    error.to_string(),
                ),
        }
    }
}

// ============================================================================
// High-level QEC helpers
// ============================================================================

/// Checked high-level commutation operation returning the canonical QEC
/// error type.
pub fn try_commutes(
    first: &PauliString,
    second: &PauliString,
) -> QecResult<bool> {
    first
        .commutes_with(second)
        .map_err(QecError::from)
}

/// Checked high-level anti-commutation operation.
pub fn try_anticommutes(
    first: &PauliString,
    second: &PauliString,
) -> QecResult<bool> {
    first
        .anticommutes_with(second)
        .map_err(QecError::from)
}

/// Validates a stabilizer group through the canonical QEC error boundary.
pub fn validate_stabilizer_group(
    group: &StabilizerGroup,
    limits: &QecLimits,
) -> QecResult<()> {
    group
        .validate_with_limits(limits)
        .map_err(QecError::from)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn three_qubit_group() -> StabilizerGroup {
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
                    &PauliString::identity(3),
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
                        second: 1,
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
                &operator,
            ),
            Err(
                StabilizerError::
                    QubitCountMismatch {
                        first: 3,
                        second: 2,
                    }
            )
        ));
    }

    #[test]
    fn mismatched_symplectic_product_never_panics() {
        let first =
            PauliString::identity(3);

        let second =
            PauliString::identity(2);

        assert!(matches!(
            first.symplectic_product(
                &second,
            ),
            Err(
                StabilizerError::
                    QubitCountMismatch {
                        first: 3,
                        second: 2,
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

    #[test]
    fn rank_is_correct() {
        let group =
            three_qubit_group();

        assert_eq!(
            group.rank().unwrap(),
            2
        );
    }

    #[test]
    fn logical_qubit_count_is_correct() {
        let group =
            three_qubit_group();

        assert_eq!(
            group
                .logical_qubit_count()
                .unwrap(),
            1
        );
    }

    #[test]
    fn redundant_generators_do_not_increase_rank() {
        let mut group =
            StabilizerGroup::new(3)
                .unwrap();

        let a =
            PauliString::from_paulis(
                &[
                    Pauli::Z,
                    Pauli::Z,
                    Pauli::I,
                ],
            );

        let b =
            PauliString::from_paulis(
                &[
                    Pauli::I,
                    Pauli::Z,
                    Pauli::Z,
                ],
            );

        let c =
            PauliString::from_paulis(
                &[
                    Pauli::Z,
                    Pauli::I,
                    Pauli::Z,
                ],
            );

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    a,
                )
                .unwrap(),
            )
            .unwrap();

        group
            .add_generator(
                StabilizerGenerator::new(
                    1,
                    b,
                )
                .unwrap(),
            )
            .unwrap();

        group
            .add_generator(
                StabilizerGenerator::new(
                    2,
                    c,
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            group.rank().unwrap(),
            2
        );
    }

    #[test]
    fn normalizer_detection_works() {
        let group =
            three_qubit_group();

        let logical =
            PauliString::from_paulis(
                &[
                    Pauli::X,
                    Pauli::X,
                    Pauli::X,
                ],
            );

        assert!(
            group
                .is_in_normalizer(
                    &logical,
                )
                .unwrap()
        );
    }

    #[test]
    fn nontrivial_logical_detection_works() {
        let group =
            three_qubit_group();

        let logical =
            PauliString::from_paulis(
                &[
                    Pauli::X,
                    Pauli::X,
                    Pauli::X,
                ],
            );

        assert!(
            group
                .is_nontrivial_logical(
                    &logical,
                )
                .unwrap()
        );
    }

    #[test]
    fn deterministic_generator_order_is_preserved() {
        let mut group =
            StabilizerGroup::new(2)
                .unwrap();

        group
            .add_generator(
                StabilizerGenerator::new(
                    10,
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

        group
            .add_generator(
                StabilizerGenerator::new(
                    2,
                    PauliString::from_paulis(
                        &[
                            Pauli::X,
                            Pauli::X,
                        ],
                    ),
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            group
                .generators()
                .iter()
                .map(StabilizerGenerator::id)
                .collect::<Vec<_>>(),
            vec![2, 10]
        );
    }

    #[test]
    fn resource_limits_are_enforced() {
        let limits =
            QecLimits {
                max_qubits: 2,
                max_stabilizers: 1,
                ..QecLimits::new()
            };

        assert!(
            StabilizerGroup::new_with_limits(
                3,
                &limits,
            )
            .is_err()
        );

        let mut group =
            StabilizerGroup::new_with_limits(
                2,
                &limits,
            )
            .unwrap();

        group
            .add_generator_with_limits(
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
                &limits,
            )
            .unwrap();

        assert!(
            group
                .add_generator_with_limits(
                    StabilizerGenerator::new(
                        1,
                        PauliString::from_paulis(
                            &[
                                Pauli::X,
                                Pauli::X,
                            ],
                        ),
                    )
                    .unwrap(),
                    &limits,
                )
                .is_err()
        );
    }

    #[test]
    fn syndrome_serialization_is_deterministic() {
        let syndrome =
            Syndrome::new(
                vec![
                    true,
                    false,
                    true,
                    false,
                    true,
                    false,
                    false,
                    true,
                    true,
                ],
            );

        assert_eq!(
            syndrome.as_bytes(),
            vec![0b10010101, 0b00000001]
        );
    }

    #[test]
    fn unified_qec_error_conversion_works() {
        let error =
            StabilizerError::
                ZeroQubits;

        let qec_error:
            QecError =
            error.into();

        assert_eq!(
            qec_error.kind(),
            super::super::errors::
                QecErrorKind::
                    InvalidStabilizer
        );
    }
}