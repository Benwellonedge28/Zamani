//! Zamani Quantum Error Correction — Stabilizer Algebra.
//!
//! Production-grade binary-symplectic stabilizer infrastructure.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - physical-qubit identifiers;
//! - single-qubit Pauli algebra;
//! - phase-free multi-qubit Pauli strings;
//! - stabilizer generators;
//! - commuting stabilizer-generator groups;
//! - GF(2) rank and membership;
//! - stabilizer syndrome generation;
//! - normalizer/centralizer checks;
//! - logical-operator pair validation;
//! - deterministic algebraic ordering;
//! - stabilizer-specific resource preflight;
//! - conversion into the canonical `QecError` boundary.
//!
//! This module does NOT own:
//!
//! - surface-code topology;
//! - decoding algorithms;
//! - MWPM;
//! - Union-Find;
//! - QPU execution;
//! - streaming;
//! - distributed execution;
//! - checkpoint persistence;
//! - telemetry transport;
//! - capability authorization.
//!
//! Those responsibilities belong to their respective QEC modules.
//!
//! # Representation
//!
//! A Pauli string uses the binary-symplectic representation
//!
//! ```text
//! P = [x | z]
//! ```
//!
//! with one X bit and one Z bit per physical qubit:
//!
//! ```text
//! I = (0,0)
//! X = (1,0)
//! Y = (1,1)
//! Z = (0,1)
//! ```
//!
//! Global Pauli phase is intentionally discarded. Therefore multiplication
//! is represented by XOR of the binary-symplectic vectors.
//!
//! This is sufficient for:
//!
//! - commutation;
//! - anti-commutation;
//! - stabilizer membership;
//! - syndrome extraction;
//! - stabilizer rank;
//! - normalizer checks;
//! - logical-equivalence foundations;
//! - surface-code distance calculations.
//!
//! A phase-sensitive Clifford simulator must use a separate representation.
//!
//! # Integration contract
//!
//! ```text
//! limits.rs
//!      │
//!      ▼
//! stabilizer.rs
//!      │
//!      ├── surface_code.rs
//!      ├── distance.rs
//!      ├── decoding_graph.rs
//!      ├── decoder.rs
//!      ├── pauli_frame.rs
//!      └── logical-equivalence layer
//! ```
//!
//! `limits.rs` remains the source of declarative resource policy.
//!
//! `errors.rs` remains the canonical public error boundary.
//!
//! `surface_code.rs` owns topology and constructs stabilizers using this
//! module.
//!
//! `decoder.rs` consumes `PauliString` and `Syndrome` but does not own their
//! algebra.
//!
//! The future `syndrome.rs` layer must consume/re-export the canonical
//! `Syndrome` representation from this module rather than introducing a
//! second incompatible syndrome type.
//!
//! # Determinism
//!
//! Generator IDs are unique and stored in ascending order. All elimination
//! uses deterministic left-to-right pivot selection and stable row ordering.
//!
//! # Resource safety
//!
//! Public operations that may allocate or perform large GF(2) work accept
//! `QecLimits` variants. Allocation sizes are checked before allocation.
//!
//! # Rust compatibility
//!
//! This implementation targets Rust 1.97.1 and uses stable standard-library
//! facilities only.

use core::fmt;
use std::collections::BTreeSet;

use super::errors::{QecError, QecResult, ResourceKind};
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "q{}", self.0)
    }
}

// ============================================================================
// Pauli
// ============================================================================

/// Single-qubit Pauli operator.
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
        matches!(self, Self::Y | Self::Z)
    }

    /// Returns true when two single-qubit Paulis anticommute.
    #[must_use]
    pub const fn anticommutes_with(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::X, Self::Y)
                | (Self::Y, Self::X)
                | (Self::X, Self::Z)
                | (Self::Z, Self::X)
                | (Self::Y, Self::Z)
                | (Self::Z, Self::Y)
        )
    }

    /// Multiplies two Pauli operators modulo global phase.
    #[must_use]
    pub const fn multiply(self, other: Self) -> Self {
        use Pauli::*;

        match (self, other) {
            (I, value) | (value, I) => value,
            (X, X) | (Y, Y) | (Z, Z) => I,
            (X, Y) | (Y, X) => Z,
            (X, Z) | (Z, X) => Y,
            (Y, Z) | (Z, Y) => X,
        }
    }

    /// Constructs a Pauli from binary-symplectic bits.
    #[must_use]
    pub const fn from_bits(x: bool, z: bool) -> Self {
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

// ============================================================================
// Pauli string
// ============================================================================

/// Multi-qubit Pauli operator in binary-symplectic form.
///
/// Global phase is discarded.
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
    /// Creates the identity operator on `num_qubits`.
    #[must_use]
    pub fn identity(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            x: vec![false; num_qubits],
            z: vec![false; num_qubits],
        }
    }

    /// Creates a Pauli string from individual Pauli operators.
    #[must_use]
    pub fn from_paulis(paulis: &[Pauli]) -> Self {
        let mut result = Self::identity(paulis.len());

        for (index, &pauli) in paulis.iter().enumerate() {
            result.x[index] = pauli.has_x_component();
            result.z[index] = pauli.has_z_component();
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
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    #[must_use]
    pub fn x_bits(&self) -> &[bool] {
        &self.x
    }

    #[must_use]
    pub fn z_bits(&self) -> &[bool] {
        &self.z
    }

    /// Returns the Pauli acting on `qubit`.
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

    /// Sets the Pauli acting on `qubit`.
    pub fn set_pauli(
        &mut self,
        qubit: QubitIndex,
        pauli: Pauli,
    ) -> Result<(), StabilizerError> {
        self.check_qubit(qubit)?;

        let index = qubit.index();

        self.x[index] = pauli.has_x_component();
        self.z[index] = pauli.has_z_component();

        Ok(())
    }

    /// Returns the operator weight.
    #[must_use]
    pub fn weight(&self) -> usize {
        self.x
            .iter()
            .zip(self.z.iter())
            .filter(|(x, z)| **x || **z)
            .count()
    }

    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.x
            .iter()
            .zip(self.z.iter())
            .all(|(x, z)| !*x && !*z)
    }

    /// Returns the support of the operator.
    #[must_use]
    pub fn support(&self) -> Vec<QubitIndex> {
        self.x
            .iter()
            .zip(self.z.iter())
            .enumerate()
            .filter_map(|(index, (x, z))| {
                if *x || *z {
                    Some(QubitIndex(index))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Computes the binary-symplectic product.
    ///
    /// Returns:
    ///
    /// - `0` when the operators commute;
    /// - `1` when the operators anticommute.
    pub fn try_symplectic_product(
        &self,
        other: &Self,
    ) -> Result<u8, StabilizerError> {
        self.check_compatible(other)?;

        let mut parity = false;

        for index in 0..self.num_qubits {
            parity ^= self.x[index] && other.z[index];
            parity ^= self.z[index] && other.x[index];
        }

        Ok(u8::from(parity))
    }

    /// Compatibility alias for the checked symplectic operation.
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
        Ok(self.try_symplectic_product(other)? == 0)
    }

    pub fn anticommutes_with(
        &self,
        other: &Self,
    ) -> Result<bool, StabilizerError> {
        Ok(self.try_symplectic_product(other)? == 1)
    }

    /// Multiplies two Pauli strings modulo global phase.
    pub fn multiply(
        &self,
        other: &Self,
    ) -> Result<Self, StabilizerError> {
        self.check_compatible(other)?;

        let mut result = Self::identity(self.num_qubits);

        for index in 0..self.num_qubits {
            result.x[index] = self.x[index] ^ other.x[index];
            result.z[index] = self.z[index] ^ other.z[index];
        }

        Ok(result)
    }

    #[must_use]
    pub fn to_paulis(&self) -> Vec<Pauli> {
        self.x
            .iter()
            .zip(self.z.iter())
            .map(|(x, z)| Pauli::from_bits(*x, *z))
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// Named stabilizer generator.
///
/// Generator IDs are unique within a `StabilizerGroup`.
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
    pub const fn id(&self) -> usize {
        self.id
    }

    #[must_use]
    pub fn operator(&self) -> &PauliString {
        &self.operator
    }

    #[must_use]
    pub fn weight(&self) -> usize {
        self.operator.weight()
    }
}

// ============================================================================
// Stabilizer group
// ============================================================================

/// A deterministic commuting set of stabilizer generators.
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

    /// Creates an empty stabilizer group under QEC resource policy.
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
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    #[must_use]
    pub fn generators(
        &self,
    ) -> &[StabilizerGenerator] {
        &self.generators
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.generators.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.generators.is_empty()
    }

    /// Adds a generator without imposing a stricter caller limit.
    ///
    /// This retains the historical API while still enforcing:
    ///
    /// - dimension compatibility;
    /// - non-identity invariant;
    /// - unique ID;
    /// - mutual commutation.
    pub fn add_generator(
        &mut self,
        generator: StabilizerGenerator,
    ) -> Result<(), StabilizerError> {
        let limits = unrestricted_limits(self.num_qubits)?;
        self.add_generator_with_limits(generator, &limits)
    }

    /// Adds a generator under explicit QEC limits.
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

        let requested_count = self
            .generators
            .len()
            .checked_add(1)
            .ok_or(
                StabilizerError::ArithmeticOverflow {
                    operation:
                        "stabilizer count + 1",
                },
            )?;

        if requested_count
            > limits.max_stabilizers
        {
            return Err(
                StabilizerError::StabilizerLimitExceeded {
                    requested: requested_count,
                    maximum: limits.max_stabilizers,
                },
            );
        }

        let weight = generator.weight();

        if weight
            > limits.max_stabilizer_weight
        {
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
                existing.id()
                    == generator.id()
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

        self.generators
            .sort_by_key(StabilizerGenerator::id);

        Ok(())
    }

    /// Validates the complete group using unconstrained dimensions.
    pub fn validate(
        &self,
    ) -> Result<(), StabilizerError> {
        let limits =
            unrestricted_limits(self.num_qubits)?;

        self.validate_with_limits(&limits)
    }

    /// Validates the complete group under explicit QEC limits.
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

        if self.num_qubits
            > limits.max_qubits
        {
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

            let weight =
                operator.weight();

            if weight
                > limits.max_stabilizer_weight
            {
                return Err(
                    StabilizerError::StabilizerWeightLimitExceeded {
                        id: generator.id(),
                        requested: weight,
                        maximum: limits.max_stabilizer_weight,
                    },
                );
            }
        }

        for first in 0..self.generators.len() {
            for second in
                (first + 1)..self.generators.len()
            {
                if self.generators[first]
                    .operator()
                    .anticommutes_with(
                        self.generators[second]
                            .operator(),
                    )?
                {
                    return Err(
                        StabilizerError::NonCommutingGenerators {
                            first: self.generators[first].id(),
                            second: self.generators[second].id(),
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
    pub fn rank(
        &self,
    ) -> Result<usize, StabilizerError> {
        let limits =
            unrestricted_limits(self.num_qubits)?;

        self.rank_with_limits(&limits)
    }

    /// Returns GF(2) rank under explicit resource policy.
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

    /// Returns the number of encoded logical qubits.
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
        let limits =
            unrestricted_limits(self.num_qubits)?;

        self.contains_with_limits(
            operator,
            &limits,
        )
    }

    /// Determines stabilizer membership under resource policy.
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

        let mut target = pack_bits_checked(
            operator.x_bits(),
            operator.z_bits(),
        )?;

        for row_index in 0..rank {
            let Some(pivot) =
                first_set_bit(
                    &rows[row_index],
                    width,
                )
            else {
                continue;
            };

            if get_bit(&target, pivot) {
                xor_packed(
                    &mut target,
                    &rows[row_index],
                );
            }
        }

        Ok(target
            .iter()
            .all(|word| *word == 0))
    }

    // ========================================================================
    // Generator products
    // ========================================================================

    /// Returns the product of generators identified by ID.
    pub fn product(
        &self,
        ids: &[usize],
    ) -> Result<PauliString, StabilizerError> {
        self.validate()?;

        let mut result =
            PauliString::identity(
                self.num_qubits,
            );

        for &id in ids {
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

            result = result.multiply(
                generator.operator(),
            )?;
        }

        Ok(result)
    }

    // ========================================================================
    // Syndrome
    // ========================================================================

    /// Computes the syndrome induced by a Pauli error.
    ///
    /// Syndrome bit `i` is one exactly when the error anticommutes with
    /// generator `i`.
    pub fn syndrome(
        &self,
        error: &PauliString,
    ) -> Result<Syndrome, StabilizerError> {
        let limits =
            unrestricted_limits(self.num_qubits)?;

        self.syndrome_with_limits(
            error,
            &limits,
        )
    }

    /// Computes a syndrome under explicit resource policy.
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
            Vec::new();

        bits.try_reserve(
            self.generators.len(),
        )
        .map_err(|_| {
            StabilizerError::AllocationTooLarge {
                bytes: self.generators.len(),
            }
        })?;

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
    // Normalizer / logical algebra
    // ========================================================================

    /// Returns whether `operator` commutes with every stabilizer generator.
    pub fn is_in_normalizer(
        &self,
        operator: &PauliString,
    ) -> Result<bool, StabilizerError> {
        self.validate()?;

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

    /// Returns true when the operator is in the normalizer but not in the
    /// stabilizer group.
    pub fn is_nontrivial_logical(
        &self,
        operator: &PauliString,
    ) -> Result<bool, StabilizerError> {
        if !self.is_in_normalizer(operator)? {
            return Ok(false);
        }

        Ok(!self.contains(operator)?)
    }

    /// Validates a logical-X/logical-Z pair.
    ///
    /// Both operators must:
    ///
    /// - have the correct number of qubits;
    /// - commute with every stabilizer;
    /// - not themselves be stabilizers;
    /// - anticommute with each other.
    pub fn validate_logical_pair(
        &self,
        logical_x: &PauliString,
        logical_z: &PauliString,
    ) -> Result<(), StabilizerError> {
        if !self.is_in_normalizer(logical_x)? {
            return Err(
                StabilizerError::LogicalNotInNormalizer {
                    operator:
                        logical_x.clone(),
                },
            );
        }

        if !self.is_in_normalizer(logical_z)? {
            return Err(
                StabilizerError::LogicalNotInNormalizer {
                    operator:
                        logical_z.clone(),
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

        if !logical_x
            .anticommutes_with(logical_z)?
        {
            return Err(
                StabilizerError::LogicalOperatorsDoNotAnticommute,
            );
        }

        Ok(())
    }

    // ========================================================================
    // Packed GF(2) representation
    // ========================================================================

    fn generator_rows_packed(
        &self,
        width: usize,
    ) -> Result<Vec<Vec<u64>>, StabilizerError> {
        let words = checked_word_count(width)?;

        let row_bytes =
            words
                .checked_mul(
                    core::mem::size_of::<u64>(),
                )
                .ok_or(
                    StabilizerError::ArithmeticOverflow {
                        operation:
                            "GF(2) row byte count",
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
                            "GF(2) matrix byte count",
                    },
                )?;

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
            Vec::new();

        rows.try_reserve(
            self.generators.len(),
        )
        .map_err(|_| {
            StabilizerError::AllocationTooLarge {
                bytes: total_bytes,
            }
        })?;

        for generator in &self.generators {
            rows.push(
                pack_bits_checked(
                    generator
                        .operator()
                        .x_bits(),
                    generator
                        .operator()
                        .z_bits(),
                )?,
            );
        }

        Ok(rows)
    }
}

// ============================================================================
// Syndrome
// ============================================================================

/// Syndrome bits in deterministic stabilizer-generator order.
///
/// This representation is retained here as the compatibility contract used by
/// `surface_code.rs` and `decoder.rs`. The later `syndrome.rs` layer should
/// build streaming/timestamped/detection-event representations around this
/// primitive instead of replacing it with a second incompatible algebraic
/// syndrome type.
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
    pub fn new(bits: Vec<bool>) -> Self {
        Self { bits }
    }

    #[must_use]
    pub fn bits(&self) -> &[bool] {
        &self.bits
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    #[must_use]
    pub fn triggered_count(&self) -> usize {
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
    pub fn is_trivial(&self) -> bool {
        self.triggered_count() == 0
    }

    /// Returns a deterministic little-endian packed bit representation.
    #[must_use]
    pub fn as_bytes(&self) -> Vec<u8> {
        let byte_count =
            self.bits
                .len()
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
// Free algebra helpers
// ============================================================================

/// Returns whether two Pauli operators commute.
pub fn commutes(
    first: &PauliString,
    second: &PauliString,
) -> Result<bool, StabilizerError> {
    first.commutes_with(second)
}

/// Returns whether two Pauli operators anticommute.
pub fn anticommutes(
    first: &PauliString,
    second: &PauliString,
) -> Result<bool, StabilizerError> {
    first.anticommutes_with(second)
}

/// Returns whether an operator commutes with an entire stabilizer group.
pub fn commutes_with_stabilizer_group(
    operator: &PauliString,
    group: &StabilizerGroup,
) -> Result<bool, StabilizerError> {
    group.is_in_normalizer(operator)
}

/// Returns whether two logical candidates anticommute.
pub fn logical_operators_anticommute(
    first: &PauliString,
    second: &PauliString,
) -> Result<bool, StabilizerError> {
    first.anticommutes_with(second)
}

// ============================================================================
// Checked high-level QEC boundary
// ============================================================================

/// Checked commutation operation returning the canonical QEC error type.
pub fn try_commutes(
    first: &PauliString,
    second: &PauliString,
) -> QecResult<bool> {
    first
        .commutes_with(second)
        .map_err(QecError::from)
}

/// Checked anti-commutation operation returning the canonical QEC error type.
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
// Internal resource helpers
// ============================================================================

fn unrestricted_limits(
    num_qubits: usize,
) -> Result<QecLimits, StabilizerError> {
    if num_qubits == 0 {
        return Err(
            StabilizerError::ZeroQubits,
        );
    }

    let mut limits = QecLimits::new();

    limits.max_qubits = num_qubits;
    limits.max_stabilizers = usize::MAX;
    limits.max_stabilizer_weight = usize::MAX;

    Ok(limits)
}

fn checked_word_count(
    width: usize,
) -> Result<usize, StabilizerError> {
    width
        .checked_add(63)
        .ok_or(
            StabilizerError::ArithmeticOverflow {
                operation:
                    "GF(2) width + 63",
            },
        )
        .map(|value| value / 64)
}

fn pack_bits_checked(
    x: &[bool],
    z: &[bool],
) -> Result<Vec<u64>, StabilizerError> {
    if x.len() != z.len() {
        return Err(
            StabilizerError::SymplecticDimensionMismatch {
                x: x.len(),
                z: z.len(),
            },
        );
    }

    let width =
        x.len()
            .checked_add(z.len())
            .ok_or(
                StabilizerError::ArithmeticOverflow {
                    operation:
                        "symplectic vector width",
                },
            )?;

    let words =
        checked_word_count(width)?;

    let bytes =
        words
            .checked_mul(
                core::mem::size_of::<u64>(),
            )
            .ok_or(
                StabilizerError::ArithmeticOverflow {
                    operation:
                        "packed symplectic allocation",
                },
            )?;

    if bytes
        > isize::MAX as usize
    {
        return Err(
            StabilizerError::AllocationTooLarge {
                bytes,
            },
        );
    }

    let mut result =
        Vec::new();

    result
        .try_reserve(words)
        .map_err(|_| {
            StabilizerError::AllocationTooLarge {
                bytes,
            }
        })?;

    result.resize(words, 0);

    for (index, bit)
        in x.iter().enumerate()
    {
        if *bit {
            set_bit(
                &mut result,
                index,
            );
        }
    }

    for (index, bit)
        in z.iter().enumerate()
    {
        if *bit {
            let destination =
                x.len()
                    .checked_add(index)
                    .ok_or(
                        StabilizerError::ArithmeticOverflow {
                            operation:
                                "X/Z packed bit index",
                        },
                    )?;

            if *bit {
                set_bit(
                    &mut result,
                    destination,
                );
            }
        }
    }

    Ok(result)
}

fn set_bit(
    words: &mut [u64],
    index: usize,
) {
    let word = index / 64;
    let bit = index % 64;

    if let Some(value) =
        words.get_mut(word)
    {
        *value |= 1u64 << bit;
    }
}

fn get_bit(
    words: &[u64],
    index: usize,
) -> bool {
    let word = index / 64;
    let bit = index % 64;

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
    for (left, right) in destination
        .iter_mut()
        .zip(source.iter())
    {
        *left ^= *right;
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

        let local =
            word.trailing_zeros()
                as usize;

        let index =
            word_index
                .checked_mul(64)?
                .checked_add(local)?;

        if index < width {
            return Some(index);
        }

        return None;
    }

    None
}

/// Deterministic Gauss-Jordan elimination over GF(2).
///
/// Rows are reduced into reduced-echelon form using the lowest available
/// pivot in each column. The returned value is the rank.
///
/// The pivot row is cloned once per pivot, rather than once for every row
/// operation.
fn gf2_reduce_rows(
    rows: &mut [Vec<u64>],
    width: usize,
) -> usize {
    let mut pivot_row = 0usize;

    for column in 0..width {
        if pivot_row >= rows.len() {
            break;
        }

        let Some(pivot) =
            (pivot_row..rows.len())
                .find(|row| {
                    get_bit(
                        &rows[*row],
                        column,
                    )
                })
        else {
            continue;
        };

        rows.swap(
            pivot_row,
            pivot,
        );

        let pivot_data =
            rows[pivot_row].clone();

        for row in 0..rows.len() {
            if row == pivot_row {
                continue;
            }

            if get_bit(
                &rows[row],
                column,
            ) {
                xor_packed(
                    &mut rows[row],
                    &pivot_data,
                );
            }
        }

        pivot_row += 1;
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

    InvalidLimits(LimitError),

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

impl fmt::Display for StabilizerError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroQubits => write!(
                f,
                "stabilizer system must contain at least one qubit"
            ),

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => write!(
                f,
                "qubit {qubit} is outside a {num_qubits}-qubit system"
            ),

            Self::QubitCountMismatch {
                first,
                second,
            } => write!(
                f,
                "qubit-count mismatch: {first} != {second}"
            ),

            Self::SymplecticDimensionMismatch {
                x,
                z,
            } => write!(
                f,
                "symplectic X/Z dimensions differ: {x} != {z}"
            ),

            Self::IdentityGenerator { id } => write!(
                f,
                "stabilizer generator {id} cannot be identity"
            ),

            Self::DuplicateGenerator { id } => write!(
                f,
                "stabilizer generator {id} already exists"
            ),

            Self::UnknownGenerator { id } => write!(
                f,
                "unknown stabilizer generator {id}"
            ),

            Self::NonCommutingGenerators {
                first,
                second,
            } => write!(
                f,
                "stabilizer generators {first} and {second} do not commute"
            ),

            Self::StabilizerLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "stabilizer count {requested} exceeds configured maximum {maximum}"
            ),

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "qubit count {requested} exceeds configured maximum {maximum}"
            ),

            Self::SyndromeLimitExceeded {
                requested,
                maximum,
            } => write!(
                f,
                "syndrome count {requested} exceeds configured maximum {maximum}"
            ),

            Self::StabilizerWeightLimitExceeded {
                id,
                requested,
                maximum,
            } => write!(
                f,
                "stabilizer {id} has weight {requested}, exceeding maximum {maximum}"
            ),

            Self::InvalidLimits(error) => write!(
                f,
                "invalid QEC limits: {error}"
            ),

            Self::ArithmeticOverflow {
                operation,
            } => write!(
                f,
                "arithmetic overflow while calculating {operation}"
            ),

            Self::AllocationTooLarge {
                bytes,
            } => write!(
                f,
                "requested stabilizer allocation of {bytes} bytes is too large"
            ),

            Self::InvalidRank {
                rank,
                num_qubits,
            } => write!(
                f,
                "invalid stabilizer rank {rank} for {num_qubits} qubits"
            ),

            Self::LogicalNotInNormalizer { .. } => write!(
                f,
                "logical operator does not commute with the stabilizer group"
            ),

            Self::LogicalOperatorIsStabilizer => write!(
                f,
                "logical operator is contained in the stabilizer group"
            ),

            Self::LogicalOperatorsDoNotAnticommute => write!(
                f,
                "logical X and logical Z must anticommute"
            ),
        }
    }
}

impl std::error::Error for StabilizerError {}

// ============================================================================
// Canonical QEC error integration
// ============================================================================

impl From<StabilizerError> for QecError {
    fn from(error: StabilizerError) -> Self {
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
            } => QecError::resource_limit(
                ResourceKind::Stabilizers,
                requested as u128,
                maximum as u128,
                error.to_string(),
            ),

            StabilizerError::QubitLimitExceeded {
                requested,
                maximum,
            } => QecError::resource_limit(
                ResourceKind::Qubits,
                requested as u128,
                maximum as u128,
                error.to_string(),
            ),

            StabilizerError::SyndromeLimitExceeded {
                requested,
                maximum,
            } => QecError::resource_limit(
                ResourceKind::SyndromeEvents,
                requested as u128,
                maximum as u128,
                error.to_string(),
            ),

            StabilizerError::StabilizerWeightLimitExceeded {
                requested,
                maximum,
                ..
            } => QecError::resource_limit(
                ResourceKind::StabilizerWeight,
                requested as u128,
                maximum as u128,
                error.to_string(),
            ),

            StabilizerError::InvalidLimits(_) => {
                QecError::invalid_input(
                    error.to_string(),
                )
            }

            StabilizerError::ArithmeticOverflow { .. } => {
                QecError::numerical_failure(
                    super::errors::NumericalOperation::IntegerConversion,
                    error.to_string(),
                )
            }

            StabilizerError::AllocationTooLarge {
                bytes,
            } => QecError::memory_limit(
                bytes as u64,
                u64::MAX,
                error.to_string(),
            ),

            StabilizerError::InvalidRank { .. } => {
                QecError::invariant(
                    "stabilizer_rank <= num_qubits",
                    error.to_string(),
                )
            }
        }
    }
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
                .expect("valid group");

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
                .expect("valid generator"),
            )
            .expect("generator accepted");

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
                .expect("valid generator"),
            )
            .expect("generator accepted");

        group
    }

    #[test]
    fn pauli_mapping_is_correct() {
        assert_eq!(
            Pauli::from_bits(false, false),
            Pauli::I
        );

        assert_eq!(
            Pauli::from_bits(true, false),
            Pauli::X
        );

        assert_eq!(
            Pauli::from_bits(true, true),
            Pauli::Y
        );

        assert_eq!(
            Pauli::from_bits(false, true),
            Pauli::Z
        );
    }

    #[test]
    fn single_qubit_anticommutation_is_correct() {
        assert!(
            Pauli::X
                .anticommutes_with(Pauli::Z)
        );

        assert!(
            Pauli::Y
                .anticommutes_with(Pauli::Z)
        );

        assert!(
            Pauli::X
                .anticommutes_with(Pauli::Y)
        );

        assert!(
            !Pauli::X
                .anticommutes_with(Pauli::X)
        );
    }

    #[test]
    fn symplectic_product_is_correct() {
        let x = PauliString::from_paulis(
            &[Pauli::X],
        );

        let z = PauliString::from_paulis(
            &[Pauli::Z],
        );

        assert_eq!(
            x.symplectic_product(&z)
                .expect("compatible"),
            1
        );

        assert!(
            x.anticommutes_with(&z)
                .expect("compatible")
        );
    }

    #[test]
    fn multiplication_is_xor_based() {
        let x = PauliString::from_paulis(
            &[Pauli::X],
        );

        let z = PauliString::from_paulis(
            &[Pauli::Z],
        );

        let y =
            x.multiply(&z)
                .expect("compatible");

        assert_eq!(
            y,
            PauliString::from_paulis(
                &[Pauli::Y],
            )
        );
    }

    #[test]
    fn mismatched_dimensions_are_rejected() {
        let one =
            PauliString::identity(1);

        let two =
            PauliString::identity(2);

        assert!(
            one.multiply(&two).is_err()
        );

        assert!(
            one.symplectic_product(&two)
                .is_err()
        );
    }

    #[test]
    fn generators_are_sorted() {
        let mut group =
            StabilizerGroup::new(2)
                .expect("valid group");

        group
            .add_generator(
                StabilizerGenerator::new(
                    7,
                    PauliString::from_paulis(
                        &[Pauli::Z, Pauli::I],
                    ),
                )
                .expect("valid"),
            )
            .expect("accepted");

        group
            .add_generator(
                StabilizerGenerator::new(
                    2,
                    PauliString::from_paulis(
                        &[Pauli::I, Pauli::Z],
                    ),
                )
                .expect("valid"),
            )
            .expect("accepted");

        assert_eq!(
            group.generators()[0].id(),
            2
        );

        assert_eq!(
            group.generators()[1].id(),
            7
        );
    }

    #[test]
    fn non_commuting_generators_are_rejected() {
        let mut group =
            StabilizerGroup::new(1)
                .expect("valid group");

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(
                        &[Pauli::X],
                    ),
                )
                .expect("valid"),
            )
            .expect("first accepted");

        let result =
            group.add_generator(
                StabilizerGenerator::new(
                    1,
                    PauliString::from_paulis(
                        &[Pauli::Z],
                    ),
                )
                .expect("valid"),
            );

        assert!(matches!(
            result,
            Err(
                StabilizerError::NonCommutingGenerators {
                    ..
                }
            )
        ));
    }

    #[test]
    fn rank_handles_redundant_generators() {
        let mut group =
            StabilizerGroup::new(2)
                .expect("valid group");

        let z1 =
            PauliString::from_paulis(
                &[Pauli::Z, Pauli::I],
            );

        let z2 =
            PauliString::from_paulis(
                &[Pauli::I, Pauli::Z],
            );

        let zz =
            PauliString::from_paulis(
                &[Pauli::Z, Pauli::Z],
            );

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    z1,
                )
                .expect("valid"),
            )
            .expect("accepted");

        group
            .add_generator(
                StabilizerGenerator::new(
                    1,
                    z2,
                )
                .expect("valid"),
            )
            .expect("accepted");

        group
            .add_generator(
                StabilizerGenerator::new(
                    2,
                    zz,
                )
                .expect("valid"),
            )
            .expect("accepted");

        assert_eq!(
            group.rank().expect("rank"),
            2
        );
    }

    #[test]
    fn membership_detects_identity_and_products() {
        let group =
            three_qubit_group();

        let identity =
            PauliString::identity(3);

        assert!(
            group
                .contains(&identity)
                .expect("membership")
        );

        let product =
            group.product(&[0, 1])
                .expect("product");

        assert!(
            group
                .contains(&product)
                .expect("membership")
        );
    }

    #[test]
    fn membership_rejects_non_member() {
        let group =
            three_qubit_group();

        let x =
            PauliString::from_paulis(
                &[
                    Pauli::X,
                    Pauli::I,
                    Pauli::I,
                ],
            );

        assert!(
            !group
                .contains(&x)
                .expect("membership")
        );
    }

    #[test]
    fn syndrome_matches_anticommutation() {
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
                .expect("syndrome");

        assert_eq!(
            syndrome.bits(),
            &[true, false]
        );
    }

    #[test]
    fn normalizer_and_logical_pair_are_validated() {
        let mut group =
            StabilizerGroup::new(1)
                .expect("valid group");

        group
            .add_generator(
                StabilizerGenerator::new(
                    0,
                    PauliString::from_paulis(
                        &[Pauli::Z],
                    ),
                )
                .expect("valid"),
            )
            .expect("accepted");

        let x =
            PauliString::from_paulis(
                &[Pauli::X],
            );

        assert!(
            !group
                .is_in_normalizer(&x)
                .expect("normalizer")
        );
    }

    #[test]
    fn syndrome_bytes_are_deterministic() {
        let syndrome =
            Syndrome::new(vec![
                true,
                false,
                true,
                false,
                true,
                false,
                false,
                true,
                true,
            ]);

        assert_eq!(
            syndrome.as_bytes(),
            vec![0b10010101, 0b00000001]
        );
    }

    #[test]
    fn qec_error_conversion_is_available() {
        let error =
            StabilizerError::ZeroQubits;

        let qec: QecError =
            error.into();

        assert!(
            matches!(
                qec,
                QecError::InvalidStabilizer { .. }
            )
        );
    }
}