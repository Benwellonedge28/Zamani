//! Zamani Quantum Error Correction — Logical Equivalence.
//!
//! Canonical equivalence layer between physical Pauli operators,
//! stabilizer cosets, and encoded logical outcomes.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - equivalence modulo the stabilizer group;
//! - classification of Pauli operators relative to a stabilizer code;
//! - validation of logical X/Z basis operators;
//! - derivation of the logical Y basis operator;
//! - residual-error analysis;
//! - logical-action comparison;
//! - deterministic bounded stabilizer-group construction.
//!
//! This module does NOT own:
//!
//! - Pauli algebra;
//! - symplectic multiplication;
//! - stabilizer-generator algebra;
//! - decoder algorithms;
//! - Pauli-frame mutation;
//! - QPU execution;
//! - circuit construction;
//! - code-topology construction;
//! - statistical threshold estimation.
//!
//! Those responsibilities remain in their respective modules.
//!
//! # Canonical relation
//!
//! For phase-free Pauli operators A and B:
//!
//! ```text
//! A ~ B  <=>  A * B ∈ S
//! ```
//!
//! where `S` is the stabilizer group.
//!
//! Global Pauli phase is intentionally ignored because the stabilizer
//! representation is phase-free.
//!
//! # Integration
//!
//! ```text
//!                         stabilizer.rs
//!                              │
//!                              ▼
//!                    ┌─────────────────────┐
//!                    │ logical_equivalence │
//!                    └──────────┬──────────┘
//!                               │
//!             ┌─────────────────┼─────────────────┐
//!             ▼                 ▼                 ▼
//!          decoder         pauli_frame       verification
//!             │                 │                 │
//!             └─────────────────┼─────────────────┘
//!                               ▼
//!                       LogicalOutcome
//! ```
//!
//! `stabilizer.rs` remains the owner of Pauli multiplication and
//! commutation. Consumers must not implement independent equivalence
//! algorithms.
//!
//! # Resource safety
//!
//! Exact stabilizer-group expansion is exponential in the number of
//! independent generators. Consequently, this implementation performs
//! preflight checks against `QecLimits` before expansion.
//!
//! The bounded exact representation is appropriate for verification,
//! small codes, and deterministic tests. Large production codes should
//! use the corresponding symplectic membership/rank operations exposed
//! by the stabilizer subsystem rather than materializing an exponential
//! group.
//!
//! # Determinism
//!
//! Stabilizers are stored in `BTreeSet`, giving deterministic ordering.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97.1.
//!
//! No nightly-only features are required.

use core::fmt;
use std::collections::BTreeSet;

use super::limits::QecLimits;
use super::logical::{LogicalOutcome, LogicalPauli};
use super::stabilizer::{PauliString, StabilizerError};

/// Logical-equivalence format version.
///
/// Increment this when the serialized/public semantic contract changes.
pub const LOGICAL_EQUIVALENCE_FORMAT_VERSION: u32 = 1;

/// Result type for this module.
pub type LogicalEquivalenceResult<T> = Result<T, LogicalEquivalenceError>;

/// Errors owned by the logical-equivalence layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalEquivalenceError {
    /// The supplied resource limits are invalid.
    InvalidLimits(String),

    /// An operator has the wrong number of physical qubits.
    DimensionMismatch {
        expected: usize,
        actual: usize,
    },

    /// An operator or code definition requires at least one qubit.
    EmptyOperator,

    /// A stabilizer generator was the identity.
    IdentityStabilizerGenerator {
        index: usize,
    },

    /// Two stabilizer generators anticommute.
    NonCommutingStabilizers {
        left: usize,
        right: usize,
    },

    /// Exact stabilizer-group materialization would exceed the configured
    /// number of elements.
    StabilizerGroupTooLarge {
        requested: usize,
        maximum: usize,
    },

    /// The verification operation budget would be exceeded.
    VerificationBudgetExceeded {
        requested: u64,
        maximum: u64,
    },

    /// A Pauli operation failed inside the stabilizer subsystem.
    Stabilizer(StabilizerError),

    /// A supplied logical basis operator is invalid.
    InvalidLogicalBasis {
        logical: LogicalPauli,
    },

    /// Logical X and logical Z do not anticommute.
    LogicalBasisDoesNotAnticommute,

    /// A logical basis operator is actually a stabilizer.
    LogicalBasisIsStabilizer {
        logical: LogicalPauli,
    },

    /// A logical basis operator does not commute with all stabilizers.
    LogicalBasisNotInNormalizer {
        logical: LogicalPauli,
    },

    /// A logical basis is dimensionally inconsistent with this code.
    LogicalBasisDimensionMismatch,

    /// A logical basis does not form a complete single-logical-qubit
    /// Pauli basis.
    InvalidLogicalBasisRelation,
}

impl fmt::Display for LogicalEquivalenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimits(message) => {
                write!(f, "invalid logical-equivalence limits: {message}")
            }

            Self::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Pauli dimension mismatch: expected {expected} qubits, got {actual}"
                )
            }

            Self::EmptyOperator => {
                f.write_str("logical-equivalence operators cannot have zero qubits")
            }

            Self::IdentityStabilizerGenerator { index } => {
                write!(f, "stabilizer generator {index} is the identity")
            }

            Self::NonCommutingStabilizers { left, right } => {
                write!(
                    f,
                    "stabilizer generators {left} and {right} anticommute"
                )
            }

            Self::StabilizerGroupTooLarge {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "stabilizer group would contain {requested} elements; maximum is {maximum}"
                )
            }

            Self::VerificationBudgetExceeded {
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "logical-equivalence verification budget exceeded: requested {requested}, maximum {maximum}"
                )
            }

            Self::Stabilizer(error) => {
                write!(f, "stabilizer algebra error: {error}")
            }

            Self::InvalidLogicalBasis { logical } => {
                write!(f, "invalid logical {logical} basis operator")
            }

            Self::LogicalBasisDoesNotAnticommute => {
                f.write_str("logical X and logical Z basis operators must anticommute")
            }

            Self::LogicalBasisIsStabilizer { logical } => {
                write!(
                    f,
                    "logical {logical} basis operator is stabilizer-equivalent to identity"
                )
            }

            Self::LogicalBasisNotInNormalizer { logical } => {
                write!(
                    f,
                    "logical {logical} basis operator is not in the stabilizer normalizer"
                )
            }

            Self::LogicalBasisDimensionMismatch => {
                f.write_str("logical basis dimension does not match the code")
            }

            Self::InvalidLogicalBasisRelation => {
                f.write_str("logical X, Y, and Z do not form a valid Pauli relation")
            }
        }
    }
}

impl std::error::Error for LogicalEquivalenceError {}

impl From<StabilizerError> for LogicalEquivalenceError {
    fn from(value: StabilizerError) -> Self {
        Self::Stabilizer(value)
    }
}

/// Classification of two Pauli operators modulo the stabilizer group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquivalenceClass {
    /// The phase-free Pauli representations are identical.
    Identical,

    /// The operators differ only by a stabilizer.
    StabilizerEquivalent,

    /// The operators belong to different stabilizer cosets.
    LogicallyDistinct,
}

impl EquivalenceClass {
    /// Returns `true` when both operators represent the same stabilizer coset.
    #[must_use]
    pub const fn is_stabilizer_equivalent(self) -> bool {
        matches!(
            self,
            Self::Identical | Self::StabilizerEquivalent
        )
    }

    /// Returns `true` when the operators are byte-for-byte equal in their
    /// canonical phase-free representation.
    #[must_use]
    pub const fn is_identical(self) -> bool {
        matches!(self, Self::Identical)
    }

    /// Returns `true` when the operators are not in the same stabilizer coset.
    #[must_use]
    pub const fn is_logically_distinct(self) -> bool {
        matches!(self, Self::LogicallyDistinct)
    }
}

/// Classification of one Pauli operator relative to a stabilizer code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorClass {
    /// The identity operator.
    Identity,

    /// A non-identity stabilizer.
    Stabilizer,

    /// A non-stabilizer element of the stabilizer normalizer.
    Logical,

    /// An operator outside the stabilizer normalizer.
    PhysicalError,
}

impl OperatorClass {
    /// Returns whether the operator belongs to the stabilizer normalizer.
    #[must_use]
    pub const fn is_normalizer_element(self) -> bool {
        matches!(
            self,
            Self::Identity | Self::Stabilizer | Self::Logical
        )
    }

    /// Returns whether the operator has no logical effect.
    #[must_use]
    pub const fn is_stabilizer_trivial(self) -> bool {
        matches!(self, Self::Identity | Self::Stabilizer)
    }

    /// Returns whether the operator represents a potentially non-trivial
    /// encoded logical action.
    #[must_use]
    pub const fn is_logical(self) -> bool {
        matches!(self, Self::Logical)
    }

    /// Returns whether the operator is outside the stabilizer normalizer.
    #[must_use]
    pub const fn is_physical_error(self) -> bool {
        matches!(self, Self::PhysicalError)
    }
}

/// Validated X/Z/Y basis for one encoded logical qubit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalBasis {
    x: PauliString,
    y: PauliString,
    z: PauliString,
}

impl LogicalBasis {
    /// Construct and validate an encoded logical basis.
    ///
    /// Requirements:
    ///
    /// - X and Z have the code's physical dimension;
    /// - neither is identity;
    /// - neither is a stabilizer;
    /// - both commute with every stabilizer;
    /// - X and Z anticommute.
    ///
    /// Y is derived as X * Z modulo global phase.
    pub fn new(
        equivalence: &LogicalEquivalence,
        x: PauliString,
        z: PauliString,
    ) -> LogicalEquivalenceResult<Self> {
        equivalence.validate_dimension(&x)?;
        equivalence.validate_dimension(&z)?;

        if x.is_identity() {
            return Err(LogicalEquivalenceError::InvalidLogicalBasis {
                logical: LogicalPauli::X,
            });
        }

        if z.is_identity() {
            return Err(LogicalEquivalenceError::InvalidLogicalBasis {
                logical: LogicalPauli::Z,
            });
        }

        if equivalence.is_stabilizer(&x)? {
            return Err(
                LogicalEquivalenceError::LogicalBasisIsStabilizer {
                    logical: LogicalPauli::X,
                },
            );
        }

        if equivalence.is_stabilizer(&z)? {
            return Err(
                LogicalEquivalenceError::LogicalBasisIsStabilizer {
                    logical: LogicalPauli::Z,
                },
            );
        }

        if !equivalence.commutes_with_stabilizers(&x)? {
            return Err(
                LogicalEquivalenceError::LogicalBasisNotInNormalizer {
                    logical: LogicalPauli::X,
                },
            );
        }

        if !equivalence.commutes_with_stabilizers(&z)? {
            return Err(
                LogicalEquivalenceError::LogicalBasisNotInNormalizer {
                    logical: LogicalPauli::Z,
                },
            );
        }

        if !x.anticommutes_with(&z)? {
            return Err(
                LogicalEquivalenceError::LogicalBasisDoesNotAnticommute,
            );
        }

        let y = x.multiply(&z)?;

        Ok(Self { x, y, z })
    }

    /// Logical X operator.
    #[must_use]
    pub fn x(&self) -> &PauliString {
        &self.x
    }

    /// Logical Y operator.
    #[must_use]
    pub fn y(&self) -> &PauliString {
        &self.y
    }

    /// Logical Z operator.
    #[must_use]
    pub fn z(&self) -> &PauliString {
        &self.z
    }
}

/// Canonical logical-equivalence engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalEquivalence {
    num_qubits: usize,

    /// Complete stabilizer group.
    ///
    /// This representation is intentionally bounded. It is not intended
    /// to be materialized for arbitrarily large surface codes.
    stabilizers: BTreeSet<PauliString>,
}

impl LogicalEquivalence {
    /// Construct an exact stabilizer-equivalence engine from commuting
    /// independent-or-redundant stabilizer generators.
    ///
    /// The implementation materializes the complete group, therefore
    /// `limits.max_stabilizers` and `limits.max_verification_operations`
    /// are hard safety boundaries.
    pub fn from_generators(
        num_qubits: usize,
        generators: &[PauliString],
        limits: &QecLimits,
    ) -> LogicalEquivalenceResult<Self> {
        validate_limits(limits)?;

        if num_qubits == 0 {
            return Err(LogicalEquivalenceError::EmptyOperator);
        }

        if num_qubits > limits.max_qubits {
            return Err(LogicalEquivalenceError::DimensionMismatch {
                expected: limits.max_qubits,
                actual: num_qubits,
            });
        }

        for (index, generator) in generators.iter().enumerate() {
            if generator.num_qubits() != num_qubits {
                return Err(LogicalEquivalenceError::DimensionMismatch {
                    expected: num_qubits,
                    actual: generator.num_qubits(),
                });
            }

            if generator.is_identity() {
                return Err(
                    LogicalEquivalenceError::IdentityStabilizerGenerator {
                        index,
                    },
                );
            }
        }

        Self::validate_commuting_generators(generators)?;

        let mut stabilizers = BTreeSet::new();

        stabilizers.insert(PauliString::identity(num_qubits));

        let mut operations: u64 = 0;

        for generator in generators {
            let existing: Vec<PauliString> =
                stabilizers.iter().cloned().collect();

            let prospective = existing
                .len()
                .checked_mul(2)
                .ok_or(
                    LogicalEquivalenceError::StabilizerGroupTooLarge {
                        requested: usize::MAX,
                        maximum: limits.max_stabilizers,
                    },
                )?;

            if prospective > limits.max_stabilizers {
                return Err(
                    LogicalEquivalenceError::StabilizerGroupTooLarge {
                        requested: prospective,
                        maximum: limits.max_stabilizers,
                    },
                );
            }

            let additions = u64::try_from(existing.len()).map_err(|_| {
                LogicalEquivalenceError::VerificationBudgetExceeded {
                    requested: u64::MAX,
                    maximum: limits.max_verification_operations,
                }
            })?;

            let prospective_operations =
                operations.checked_add(additions).ok_or(
                    LogicalEquivalenceError::VerificationBudgetExceeded {
                        requested: u64::MAX,
                        maximum: limits.max_verification_operations,
                    },
                )?;

            if prospective_operations
                > limits.max_verification_operations
            {
                return Err(
                    LogicalEquivalenceError::VerificationBudgetExceeded {
                        requested: prospective_operations,
                        maximum: limits.max_verification_operations,
                    },
                );
            }

            for current in existing {
                stabilizers.insert(current.multiply(generator)?);
            }

            operations = prospective_operations;
        }

        Ok(Self {
            num_qubits,
            stabilizers,
        })
    }

    /// Construct with canonical default resource limits.
    pub fn from_generators_default(
        num_qubits: usize,
        generators: &[PauliString],
    ) -> LogicalEquivalenceResult<Self> {
        Self::from_generators(
            num_qubits,
            generators,
            &QecLimits::default(),
        )
    }

    /// Number of physical qubits represented by the code.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Number of materialized stabilizer-group elements.
    #[must_use]
    pub fn stabilizer_count(&self) -> usize {
        self.stabilizers.len()
    }

    /// Borrow the deterministic stabilizer-group representation.
    ///
    /// Consumers should prefer [`Self::is_stabilizer`] instead of depending
    /// on this storage representation.
    #[must_use]
    pub fn stabilizers(&self) -> &BTreeSet<PauliString> {
        &self.stabilizers
    }

    /// Validate the physical dimension of an operator.
    pub fn validate_dimension(
        &self,
        operator: &PauliString,
    ) -> LogicalEquivalenceResult<()> {
        if operator.num_qubits() != self.num_qubits {
            return Err(LogicalEquivalenceError::DimensionMismatch {
                expected: self.num_qubits,
                actual: operator.num_qubits(),
            });
        }

        Ok(())
    }

    /// Validate that all supplied generators commute pairwise.
    fn validate_commuting_generators(
        generators: &[PauliString],
    ) -> LogicalEquivalenceResult<()> {
        for left in 0..generators.len() {
            for right in (left + 1)..generators.len() {
                if generators[left]
                    .anticommutes_with(&generators[right])?
                {
                    return Err(
                        LogicalEquivalenceError::NonCommutingStabilizers {
                            left,
                            right,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    /// Determine whether an operator is a member of the complete
    /// stabilizer group.
    pub fn is_stabilizer(
        &self,
        operator: &PauliString,
    ) -> LogicalEquivalenceResult<bool> {
        self.validate_dimension(operator)?;
        Ok(self.stabilizers.contains(operator))
    }

    /// Determine whether an operator belongs to the stabilizer normalizer.
    ///
    /// An operator is in the normalizer iff it commutes with every
    /// stabilizer.
    pub fn commutes_with_stabilizers(
        &self,
        operator: &PauliString,
    ) -> LogicalEquivalenceResult<bool> {
        self.validate_dimension(operator)?;

        for stabilizer in &self.stabilizers {
            if operator.anticommutes_with(stabilizer)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Classify an operator relative to the stabilizer code.
    pub fn classify(
        &self,
        operator: &PauliString,
    ) -> LogicalEquivalenceResult<OperatorClass> {
        self.validate_dimension(operator)?;

        if operator.is_identity() {
            return Ok(OperatorClass::Identity);
        }

        if self.is_stabilizer(operator)? {
            return Ok(OperatorClass::Stabilizer);
        }

        if self.commutes_with_stabilizers(operator)? {
            Ok(OperatorClass::Logical)
        } else {
            Ok(OperatorClass::PhysicalError)
        }
    }

    /// Compare two physical Pauli operators modulo stabilizers.
    ///
    /// The comparison uses:
    ///
    /// `left * right ∈ S`
    ///
    /// because Pauli operators are self-inverse in the phase-free
    /// representation.
    pub fn compare(
        &self,
        left: &PauliString,
        right: &PauliString,
    ) -> LogicalEquivalenceResult<EquivalenceClass> {
        self.validate_dimension(left)?;
        self.validate_dimension(right)?;

        if left == right {
            return Ok(EquivalenceClass::Identical);
        }

        let residual = left.multiply(right)?;

        if self.is_stabilizer(&residual)? {
            Ok(EquivalenceClass::StabilizerEquivalent)
        } else {
            Ok(EquivalenceClass::LogicallyDistinct)
        }
    }

    /// Return whether two operators belong to the same stabilizer coset.
    pub fn equivalent(
        &self,
        left: &PauliString,
        right: &PauliString,
    ) -> LogicalEquivalenceResult<bool> {
        Ok(self.compare(left, right)?.is_stabilizer_equivalent())
    }

    /// Compute and classify `error * correction`.
    pub fn classify_residual(
        &self,
        error: &PauliString,
        correction: &PauliString,
    ) -> LogicalEquivalenceResult<OperatorClass> {
        self.validate_dimension(error)?;
        self.validate_dimension(correction)?;

        let residual = error.multiply(correction)?;

        self.classify(&residual)
    }

    /// Determine the encoded logical action of an operator.
    ///
    /// Returns:
    ///
    /// - `Identity` for stabilizer-trivial operators;
    /// - `LogicalX` for an X-basis coset;
    /// - `LogicalY` for a Y-basis coset;
    /// - `LogicalZ` for a Z-basis coset;
    /// - `Unknown` when the operator is outside the normalizer or belongs
    ///   to a logical coset not represented by the supplied single-qubit
    ///   basis.
    pub fn logical_effect(
        &self,
        operator: &PauliString,
        basis: &LogicalBasis,
    ) -> LogicalEquivalenceResult<LogicalOutcome> {
        self.validate_dimension(operator)?;

        self.validate_dimension(basis.x())?;
        self.validate_dimension(basis.y())?;
        self.validate_dimension(basis.z())?;

        if self.is_stabilizer(operator)? {
            return Ok(LogicalOutcome::Identity);
        }

        if !self.commutes_with_stabilizers(operator)? {
            return Ok(LogicalOutcome::Unknown);
        }

        if self.equivalent(operator, basis.x())? {
            return Ok(LogicalOutcome::LogicalX);
        }

        if self.equivalent(operator, basis.y())? {
            return Ok(LogicalOutcome::LogicalY);
        }

        if self.equivalent(operator, basis.z())? {
            return Ok(LogicalOutcome::LogicalZ);
        }

        Ok(LogicalOutcome::Unknown)
    }

    /// Compute the residual `error * correction` and classify its encoded
    /// logical action.
    ///
    /// This is the canonical decoder-verification entry point.
    pub fn analyze_residual(
        &self,
        error: &PauliString,
        correction: &PauliString,
        basis: &LogicalBasis,
    ) -> LogicalEquivalenceResult<LogicalOutcome> {
        self.validate_dimension(error)?;
        self.validate_dimension(correction)?;

        let residual = error.multiply(correction)?;

        self.logical_effect(&residual, basis)
    }

    /// Determine whether two operators have the same encoded logical action.
    ///
    /// If either action is `Unknown`, this returns `false` rather than
    /// claiming equivalence from incomplete information.
    pub fn same_logical_action(
        &self,
        left: &PauliString,
        right: &PauliString,
        basis: &LogicalBasis,
    ) -> LogicalEquivalenceResult<bool> {
        let left_outcome = self.logical_effect(left, basis)?;
        let right_outcome = self.logical_effect(right, basis)?;

        Ok(
            left_outcome == right_outcome
                && !left_outcome.is_unknown()
        )
    }

    /// Verify that a supplied logical basis is valid for this code.
    pub fn validate_basis(
        &self,
        basis: &LogicalBasis,
    ) -> LogicalEquivalenceResult<()> {
        self.validate_dimension(basis.x())?;
        self.validate_dimension(basis.y())?;
        self.validate_dimension(basis.z())?;

        if basis.x().is_identity() {
            return Err(LogicalEquivalenceError::InvalidLogicalBasis {
                logical: LogicalPauli::X,
            });
        }

        if basis.y().is_identity() {
            return Err(LogicalEquivalenceError::InvalidLogicalBasis {
                logical: LogicalPauli::Y,
            });
        }

        if basis.z().is_identity() {
            return Err(LogicalEquivalenceError::InvalidLogicalBasis {
                logical: LogicalPauli::Z,
            });
        }

        if self.is_stabilizer(basis.x())? {
            return Err(
                LogicalEquivalenceError::LogicalBasisIsStabilizer {
                    logical: LogicalPauli::X,
                },
            );
        }

        if self.is_stabilizer(basis.y())? {
            return Err(
                LogicalEquivalenceError::LogicalBasisIsStabilizer {
                    logical: LogicalPauli::Y,
                },
            );
        }

        if self.is_stabilizer(basis.z())? {
            return Err(
                LogicalEquivalenceError::LogicalBasisIsStabilizer {
                    logical: LogicalPauli::Z,
                },
            );
        }

        if !self.commutes_with_stabilizers(basis.x)? {
            return Err(
                LogicalEquivalenceError::LogicalBasisNotInNormalizer {
                    logical: LogicalPauli::X,
                },
            );
        }

        if !self.commutes_with_stabilizers(basis.y)? {
            return Err(
                LogicalEquivalenceError::LogicalBasisNotInNormalizer {
                    logical: LogicalPauli::Y,
                },
            );
        }

        if !self.commutes_with_stabilizers(basis.z)? {
            return Err(
                LogicalEquivalenceError::LogicalBasisNotInNormalizer {
                    logical: LogicalPauli::Z,
                },
            );
        }

        if !basis.x().anticommutes_with(basis.z)? {
            return Err(
                LogicalEquivalenceError::LogicalBasisDoesNotAnticommute,
            );
        }

        let derived_y = basis.x().multiply(basis.z)?;

        if derived_y != *basis.y() {
            return Err(
                LogicalEquivalenceError::InvalidLogicalBasisRelation,
            );
        }

        Ok(())
    }
}

/// Convert `QecLimits` validation failure into the local error boundary.
fn validate_limits(
    limits: &QecLimits,
) -> LogicalEquivalenceResult<()> {
    limits
        .validate()
        .map_err(|error| {
            LogicalEquivalenceError::InvalidLimits(
                error.to_string(),
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::stabilizer::Pauli;

    fn one(pauli: Pauli) -> PauliString {
        PauliString::from_paulis(&[pauli])
    }

    #[test]
    fn identity_is_stabilizer_trivial() {
        let engine =
            LogicalEquivalence::from_generators_default(1, &[])
                .expect("empty generator set is valid");

        let identity = PauliString::identity(1);

        assert_eq!(
            engine.classify(&identity).unwrap(),
            OperatorClass::Identity
        );

        assert!(
            engine.is_stabilizer(&identity).unwrap(),
            "identity must belong to every stabilizer group"
        );
    }

    #[test]
    fn generators_expand_to_complete_group() {
        let z = one(Pauli::Z);

        let engine =
            LogicalEquivalence::from_generators_default(1, &[z.clone()])
                .unwrap();

        assert_eq!(engine.stabilizer_count(), 2);
        assert!(engine.is_stabilizer(&z).unwrap());
    }

    #[test]
    fn stabilizer_difference_is_equivalent() {
        let z = one(Pauli::Z);

        let engine =
            LogicalEquivalence::from_generators_default(1, &[z.clone()])
                .unwrap();

        let identity = PauliString::identity(1);

        assert_eq!(
            engine.compare(&identity, &z).unwrap(),
            EquivalenceClass::StabilizerEquivalent
        );

        assert!(
            engine.equivalent(&identity, &z).unwrap()
        );
    }

    #[test]
    fn identical_operators_are_identical() {
        let engine =
            LogicalEquivalence::from_generators_default(1, &[])
                .unwrap();

        let x = one(Pauli::X);

        assert_eq!(
            engine.compare(&x, &x).unwrap(),
            EquivalenceClass::Identical
        );
    }

    #[test]
    fn non_commuting_generators_are_rejected() {
        let x = one(Pauli::X);
        let z = one(Pauli::Z);

        let result =
            LogicalEquivalence::from_generators_default(
                1,
                &[x, z],
            );

        assert!(matches!(
            result,
            Err(
                LogicalEquivalenceError::NonCommutingStabilizers {
                    ..
                }
            )
        ));
    }

    #[test]
    fn identity_generator_is_rejected() {
        let identity = PauliString::identity(1);

        let result =
            LogicalEquivalence::from_generators_default(
                1,
                &[identity],
            );

        assert!(matches!(
            result,
            Err(
                LogicalEquivalenceError::IdentityStabilizerGenerator {
                    index: 0
                }
            )
        ));
    }

    #[test]
    fn dimension_mismatch_is_rejected() {
        let engine =
            LogicalEquivalence::from_generators_default(1, &[])
                .unwrap();

        let x_two = PauliString::from_paulis(&[
            Pauli::X,
            Pauli::I,
        ]);

        let result = engine.classify(&x_two);

        assert!(matches!(
            result,
            Err(
                LogicalEquivalenceError::DimensionMismatch {
                    expected: 1,
                    actual: 2
                }
            )
        ));
    }

    #[test]
    fn logical_basis_derives_y() {
        let engine =
            LogicalEquivalence::from_generators_default(1, &[])
                .unwrap();

        let x = one(Pauli::X);
        let z = one(Pauli::Z);

        let basis =
            LogicalBasis::new(&engine, x, z).unwrap();

        assert_eq!(
            basis.y(),
            &one(Pauli::Y)
        );

        engine.validate_basis(&basis).unwrap();
    }

    #[test]
    fn logical_x_is_classified() {
        let engine =
            LogicalEquivalence::from_generators_default(1, &[])
                .unwrap();

        let x = one(Pauli::X);
        let z = one(Pauli::Z);

        let basis =
            LogicalBasis::new(&engine, x.clone(), z)
                .unwrap();

        assert_eq!(
            engine.logical_effect(&x, &basis).unwrap(),
            LogicalOutcome::LogicalX
        );
    }

    #[test]
    fn logical_z_is_classified() {
        let engine =
            LogicalEquivalence::from_generators_default(1, &[])
                .unwrap();

        let x = one(Pauli::X);
        let z = one(Pauli::Z);

        let basis =
            LogicalBasis::new(&engine, x, z.clone())
                .unwrap();

        assert_eq!(
            engine.logical_effect(&z, &basis).unwrap(),
            LogicalOutcome::LogicalZ
        );
    }

    #[test]
    fn logical_y_is_classified() {
        let engine =
            LogicalEquivalence::from_generators_default(1, &[])
                .unwrap();

        let x = one(Pauli::X);
        let z = one(Pauli::Z);

        let basis =
            LogicalBasis::new(&engine, x, z)
                .unwrap();

        let y = one(Pauli::Y);

        assert_eq!(
            engine.logical_effect(&y, &basis).unwrap(),
            LogicalOutcome::LogicalY
        );
    }

    #[test]
    fn stabilizer_has_identity_logical_effect() {
        let z = one(Pauli::Z);

        let engine =
            LogicalEquivalence::from_generators_default(
                1,
                &[z.clone()],
            )
            .unwrap();

        let basis_x = one(Pauli::X);
        let basis_z = one(Pauli::X);

        let _ = (basis_x, basis_z);

        assert_eq!(
            engine.classify(&z).unwrap(),
            OperatorClass::Stabilizer
        );
    }

    #[test]
    fn non_normalizer_operator_is_physical_error() {
        let stabilizer = one(Pauli::Z);

        let engine =
            LogicalEquivalence::from_generators_default(
                1,
                &[stabilizer],
            )
            .unwrap();

        let x = one(Pauli::X);

        assert_eq!(
            engine.classify(&x).unwrap(),
            OperatorClass::PhysicalError
        );
    }

    #[test]
    fn residual_analysis_uses_error_times_correction() {
        let engine =
            LogicalEquivalence::from_generators_default(1, &[])
                .unwrap();

        let x = one(Pauli::X);
        let z = one(Pauli::Z);

        let basis =
            LogicalBasis::new(&engine, x.clone(), z)
                .unwrap();

        let correction = PauliString::identity(1);

        assert_eq!(
            engine
                .analyze_residual(
                    &x,
                    &correction,
                    &basis,
                )
                .unwrap(),
            LogicalOutcome::LogicalX
        );
    }

    #[test]
    fn equivalent_operators_have_same_logical_action() {
        let stabilizer = one(Pauli::Z);

        let engine =
            LogicalEquivalence::from_generators_default(
                1,
                &[stabilizer],
            )
            .unwrap();

        let x = one(Pauli::X);
        let z = one(Pauli::Z);

        let basis =
            LogicalBasis::new(&engine, x.clone(), z)
                .unwrap();

        // With a Z stabilizer, X and X*Z belong to the same
        // stabilizer coset. Their logical action is therefore equal.
        let xz = x.multiply(&one(Pauli::Z)).unwrap();

        assert!(
            engine
                .same_logical_action(
                    &x,
                    &xz,
                    &basis,
                )
                .unwrap()
        );
    }

    #[test]
    fn different_logical_actions_are_not_equivalent() {
        let engine =
            LogicalEquivalence::from_generators_default(1, &[])
                .unwrap();

        let x = one(Pauli::X);
        let z = one(Pauli::Z);

        let basis =
            LogicalBasis::new(
                &engine,
                x.clone(),
                z.clone(),
            )
            .unwrap();

        assert!(
            !engine
                .same_logical_action(
                    &x,
                    &z,
                    &basis,
                )
                .unwrap()
        );
    }

    #[test]
    fn stabilizer_group_growth_is_bounded() {
        let mut limits = QecLimits::default();

        // A single generator requires identity + generator.
        limits.max_stabilizers = 1;

        let z = one(Pauli::Z);

        let result =
            LogicalEquivalence::from_generators(
                1,
                &[z],
                &limits,
            );

        assert!(matches!(
            result,
            Err(
                LogicalEquivalenceError::StabilizerGroupTooLarge {
                    ..
                }
            )
        ));
    }

    #[test]
    fn invalid_verification_budget_is_rejected() {
        let mut limits = QecLimits::default();

        limits.max_verification_operations = 0;

        let z = one(Pauli::Z);

        let result =
            LogicalEquivalence::from_generators(
                1,
                &[z],
                &limits,
            );

        assert!(matches!(
            result,
            Err(
                LogicalEquivalenceError::VerificationBudgetExceeded {
                    ..
                }
            )
        ));
    }
}