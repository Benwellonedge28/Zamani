//! # QEC Mathematical Verification
//!
//! Deterministic, resource-bounded verification primitives for the quantum
//! error-correction subsystem.
//!
//! ## Responsibilities
//!
//! This module owns verification *orchestration* and verification results.
//! It does not own the underlying stabilizer algebra. That remains in
//! [`crate::quantum::error_correction::stabilizer`].
//!
//! The central mathematical contracts are:
//!
//! ```text
//! A ≡ B (mod S)
//!     iff
//! A * B ∈ S
//!
//! P ∈ N(S)
//!     iff
//! P commutes with every generator of S
//!
//! P ∈ S
//!     => stabilizer-equivalent / trivial
//!
//! P ∈ N(S) \\ S
//!     => non-trivial logical operator
//!
//! P ∉ N(S)
//!     => detectable physical error
//! ```
//!
//! The Pauli representation used by the QEC subsystem is phase-free. Therefore
//! all equivalence decisions in this module are made modulo global Pauli phase.
//!
//! ## Architectural rules
//!
//! - No decoder implementation belongs here.
//! - No QPU/backend implementation belongs here.
//! - No random numbers are used.
//! - No global mutable state is used.
//! - Verification never silently converts resource exhaustion into a
//!   mathematical `false` result.
//! - Potentially expensive operations are checked against [`QecLimits`]
//!   before they are performed.
//! - Existing stabilizer mathematics is reused rather than duplicated.
//!
//! ## Integration
//!
//! ```text
//! arithmetic.rs
//!       │
//! errors.rs ───────────────┐
//!       │                  │
//! limits.rs ───────────────┤
//!       │                  │
//! stabilizer.rs ───────────┤
//!                          ▼
//!                  verification.rs
//!                          │
//!              ┌───────────┼───────────┐
//!              ▼           ▼           ▼
//!       logical.rs   distance.rs   logical_equivalence.rs
//!              │                       │
//!              └───────────┬───────────┘
//!                          ▼
//!                    decoder_result
//! ```
//!
//! ## Rust compatibility
//!
//! Designed for Rust 1.97.1 and does not require nightly features.

use core::fmt;

use super::errors::{QecError, QecResult};
use super::limits::QecLimits;
use super::stabilizer::{PauliString, StabilizerGroup};

// ============================================================================
// Verification kind
// ============================================================================

/// Mathematical property being verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationKind {
    /// Two Pauli operators are equivalent modulo the stabilizer group.
    StabilizerEquivalence,

    /// An operator commutes with every stabilizer generator.
    NormalizerMembership,

    /// An operator belongs to the stabilizer group.
    StabilizerMembership,

    /// An operator is a detectable physical error.
    DetectableError,

    /// An operator represents a non-trivial logical operation.
    LogicalOperator,

    /// A Pauli operator is exactly the identity in the phase-free
    /// representation.
    Identity,

    /// A complete stabilizer group satisfies its structural invariants.
    StabilizerGroup,

    /// A correction is mathematically valid for the supplied stabilizer
    /// structure.
    Correction,

    /// Two corrections/operators have the requested equivalence relation.
    OperatorPair,
}

// ============================================================================
// Verification mode
// ============================================================================

/// Controls how verification is performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationMode {
    /// Perform the strongest exact verification permitted by the resource
    /// limits.
    Exact,

    /// Perform verification while respecting the supplied resource limits.
    ///
    /// This is the normal production mode.
    Bounded,

    /// Validate only structural invariants.
    Structural,
}

impl Default for VerificationMode {
    fn default() -> Self {
        Self::Bounded
    }
}

// ============================================================================
// Verification status
// ============================================================================

/// Result status of a mathematical verification operation.
///
/// `VerifiedNotEquivalent` is fundamentally different from `ResourceLimited`
/// or `Cancelled`. The former is a mathematical counterexample; the latter
/// means the requested proof did not complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationStatus {
    /// The requested property has been mathematically established.
    Verified,

    /// The requested equivalence has been disproved.
    VerifiedNotEquivalent,

    /// The operator is a valid non-trivial logical operator.
    VerifiedLogical,

    /// The operator is a valid stabilizer-equivalent operator.
    VerifiedStabilizer,

    /// The operator is in the normalizer but not in the stabilizer group.
    VerifiedNormalizer,

    /// The operator is not in the normalizer.
    VerifiedDetectableError,

    /// Verification could not complete within the configured resource
    /// limits.
    ResourceLimited,

    /// Verification was cancelled before completion.
    Cancelled,

    /// The supplied mathematical object is invalid.
    Invalid,
}

impl VerificationStatus {
    /// Returns `true` if the status constitutes a mathematically proven
    /// positive result.
    #[must_use]
    pub const fn is_verified(self) -> bool {
        matches!(
            self,
            Self::Verified
                | Self::VerifiedLogical
                | Self::VerifiedStabilizer
                | Self::VerifiedNormalizer
        )
    }

    /// Returns `true` if the status is a mathematical negative result.
    #[must_use]
    pub const fn is_disproof(self) -> bool {
        matches!(
            self,
            Self::VerifiedNotEquivalent
                | Self::VerifiedDetectableError
        )
    }

    /// Returns `true` when the operation did not establish a mathematical
    /// result because execution could not complete.
    #[must_use]
    pub const fn is_inconclusive(self) -> bool {
        matches!(
            self,
            Self::ResourceLimited | Self::Cancelled
        )
    }
}

// ============================================================================
// Equivalence result
// ============================================================================

/// Result of comparing two phase-free Pauli operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquivalenceResult {
    /// The operators differ by an element of the stabilizer group.
    Equivalent,

    /// The operators are not stabilizer-equivalent.
    NotEquivalent,
}

impl EquivalenceResult {
    #[must_use]
    pub const fn are_equivalent(self) -> bool {
        matches!(self, Self::Equivalent)
    }
}

// ============================================================================
// Normalizer result
// ============================================================================

/// Result of checking whether an operator belongs to the stabilizer
/// normalizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizerResult {
    /// The operator commutes with every stabilizer generator.
    InNormalizer,

    /// The operator anticommutes with at least one stabilizer generator.
    NotInNormalizer,
}

impl NormalizerResult {
    #[must_use]
    pub const fn is_in_normalizer(self) -> bool {
        matches!(self, Self::InNormalizer)
    }
}

// ============================================================================
// Verification report
// ============================================================================

/// Structured result returned by verification operations.
///
/// The report deliberately separates:
///
/// - what was requested;
/// - what mathematical result was obtained;
/// - whether execution completed;
/// - the amount of work involved.
///
/// This prevents callers from treating a resource failure as a mathematical
/// negative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReport {
    kind: VerificationKind,
    mode: VerificationMode,
    status: VerificationStatus,
    qubits: usize,
    generators: usize,
    operations: usize,
}

impl VerificationReport {
    fn new(
        kind: VerificationKind,
        mode: VerificationMode,
        status: VerificationStatus,
        qubits: usize,
        generators: usize,
        operations: usize,
    ) -> Self {
        Self {
            kind,
            mode,
            status,
            qubits,
            generators,
            operations,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> VerificationKind {
        self.kind
    }

    #[must_use]
    pub const fn mode(&self) -> VerificationMode {
        self.mode
    }

    #[must_use]
    pub const fn status(&self) -> VerificationStatus {
        self.status
    }

    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    #[must_use]
    pub const fn generators(&self) -> usize {
        self.generators
    }

    #[must_use]
    pub const fn operations(&self) -> usize {
        self.operations
    }

    #[must_use]
    pub const fn is_verified(&self) -> bool {
        self.status.is_verified()
    }

    #[must_use]
    pub const fn is_disproof(&self) -> bool {
        self.status.is_disproof()
    }

    #[must_use]
    pub const fn is_inconclusive(&self) -> bool {
        self.status.is_inconclusive()
    }
}

impl fmt::Display for VerificationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: {:?} (qubits={}, generators={}, operations={})",
            self.kind,
            self.status,
            self.qubits,
            self.generators,
            self.operations,
        )
    }
}

// ============================================================================
// Internal validation helpers
// ============================================================================

fn validate_limits(limits: &QecLimits) -> QecResult<()> {
    limits
        .validate()
        .map_err(|error| QecError::InvalidInput {
            message: format!("invalid QEC limits: {error}"),
        })
}

fn validate_dimensions(
    first: &PauliString,
    second: &PauliString,
) -> QecResult<()> {
    if first.num_qubits() != second.num_qubits() {
        return Err(QecError::InvalidInput {
            message: format!(
                "Pauli dimension mismatch: {} != {}",
                first.num_qubits(),
                second.num_qubits()
            ),
        });
    }

    Ok(())
}

fn validate_group(
    group: &StabilizerGroup,
    limits: &QecLimits,
) -> QecResult<()> {
    group
        .validate_with_limits(limits)
        .map_err(|error| QecError::InvalidInput {
            message: format!("invalid stabilizer group: {error}"),
        })
}

fn validate_operator(
    operator: &PauliString,
    limits: &QecLimits,
) -> QecResult<()> {
    if operator.num_qubits() == 0 {
        return Err(QecError::InvalidInput {
            message: "Pauli operator must contain at least one qubit"
                .to_owned(),
        });
    }

    if operator.num_qubits() > limits.max_qubits {
        return Err(QecError::ResourceLimitExceeded {
            resource: "qubits",
            requested: operator.num_qubits(),
            maximum: limits.max_qubits,
        });
    }

    Ok(())
}

fn check_operation_budget(
    operations: usize,
    limits: &QecLimits,
) -> QecResult<()> {
    if operations > limits.max_operations {
        return Err(QecError::ResourceLimitExceeded {
            resource: "verification operations",
            requested: operations,
            maximum: limits.max_operations,
        });
    }

    Ok(())
}

// ============================================================================
// Stabilizer-group verification
// ============================================================================

/// Verifies the structural invariants of a stabilizer group.
pub fn verify_stabilizer_group(
    group: &StabilizerGroup,
    limits: &QecLimits,
) -> QecResult<VerificationReport> {
    validate_limits(limits)?;

    let operations = group
        .len()
        .checked_mul(group.len())
        .ok_or(QecError::NumericalFailure {
            operation: "stabilizer verification operation count",
        })?;

    check_operation_budget(operations, limits)?;
    validate_group(group, limits)?;

    Ok(VerificationReport::new(
        VerificationKind::StabilizerGroup,
        VerificationMode::Structural,
        VerificationStatus::Verified,
        group.num_qubits(),
        group.len(),
        operations,
    ))
}

// ============================================================================
// Identity verification
// ============================================================================

/// Verifies whether a Pauli operator is the phase-free identity.
pub fn verify_identity(
    operator: &PauliString,
    limits: &QecLimits,
) -> QecResult<VerificationReport> {
    validate_limits(limits)?;
    validate_operator(operator, limits)?;

    let status = if operator.is_identity() {
        VerificationStatus::Verified
    } else {
        VerificationStatus::VerifiedNotEquivalent
    };

    Ok(VerificationReport::new(
        VerificationKind::Identity,
        VerificationMode::Exact,
        status,
        operator.num_qubits(),
        0,
        1,
    ))
}

// ============================================================================
// Normalizer verification
// ============================================================================

/// Verifies whether `operator` belongs to the normalizer N(S).
///
/// An operator is in the normalizer exactly when it commutes with every
/// stabilizer generator.
pub fn verify_normalizer_membership(
    operator: &PauliString,
    group: &StabilizerGroup,
    limits: &QecLimits,
) -> QecResult<NormalizerResult> {
    validate_limits(limits)?;
    validate_operator(operator, limits)?;
    validate_group(group, limits)?;

    if operator.num_qubits() != group.num_qubits() {
        return Err(QecError::InvalidInput {
            message: format!(
                "operator/group dimension mismatch: {} != {}",
                operator.num_qubits(),
                group.num_qubits()
            ),
        });
    }

    check_operation_budget(group.len(), limits)?;

    for generator in group.generators() {
        if operator
            .anticommutes_with(generator.operator())
            .map_err(|error| QecError::InvalidInput {
                message: format!(
                    "failed normalizer verification: {error}"
                ),
            })?
        {
            return Ok(NormalizerResult::NotInNormalizer);
        }
    }

    Ok(NormalizerResult::InNormalizer)
}

/// Produces a structured normalizer verification report.
pub fn verify_normalizer(
    operator: &PauliString,
    group: &StabilizerGroup,
    mode: VerificationMode,
    limits: &QecLimits,
) -> QecResult<VerificationReport> {
    let result =
        verify_normalizer_membership(operator, group, limits)?;

    let status = match result {
        NormalizerResult::InNormalizer => {
            VerificationStatus::VerifiedNormalizer
        }
        NormalizerResult::NotInNormalizer => {
            VerificationStatus::VerifiedDetectableError
        }
    };

    Ok(VerificationReport::new(
        VerificationKind::NormalizerMembership,
        mode,
        status,
        group.num_qubits(),
        group.len(),
        group.len(),
    ))
}

// ============================================================================
// Stabilizer membership
// ============================================================================

/// Verifies whether `operator` belongs to the stabilizer group.
///
/// The test uses the existing stabilizer rank implementation:
///
/// ```text
/// P ∈ S
/// iff
/// rank(S ∪ {P}) == rank(S)
/// ```
///
/// The operator must first be shown to commute with the stabilizer group.
/// A non-normalizer operator cannot be a member of the stabilizer group.
pub fn verify_stabilizer_membership(
    operator: &PauliString,
    group: &StabilizerGroup,
    limits: &QecLimits,
) -> QecResult<bool> {
    validate_limits(limits)?;
    validate_operator(operator, limits)?;
    validate_group(group, limits)?;

    if operator.num_qubits() != group.num_qubits() {
        return Err(QecError::InvalidInput {
            message: format!(
                "operator/group dimension mismatch: {} != {}",
                operator.num_qubits(),
                group.num_qubits()
            ),
        });
    }

    let normalizer =
        verify_normalizer_membership(operator, group, limits)?;

    if !normalizer.is_in_normalizer() {
        return Ok(false);
    }

    //
    // The existing StabilizerGroup rank implementation is intentionally
    // reused rather than duplicating its GF(2) Gaussian elimination here.
    //
    // A temporary generator is required to evaluate rank(S ∪ {P}).
    //
    // The public StabilizerGenerator constructor is used so that all
    // stabilizer invariants remain centralized in stabilizer.rs.
    //
    let generator =
        super::stabilizer::StabilizerGenerator::new(
            next_verification_generator_id(group)?,
            operator.clone(),
        )
        .map_err(|error| QecError::InvalidInput {
            message: format!(
                "unable to construct verification generator: {error}"
            ),
        })?;

    let base_rank =
        group
            .rank_with_limits(limits)
            .map_err(|error| QecError::InvalidInput {
                message: format!(
                    "unable to calculate stabilizer rank: {error}"
                ),
            })?;

    let mut augmented = group.clone();

    //
    // The temporary group may contain one additional generator. We therefore
    // validate against the same explicit policy before inserting it.
    //
    augmented
        .add_generator_with_limits(generator, limits)
        .map_err(|error| QecError::InvalidInput {
            message: format!(
                "unable to augment stabilizer group: {error}"
            ),
        })?;

    let augmented_rank =
        augmented
            .rank_with_limits(limits)
            .map_err(|error| QecError::InvalidInput {
                message: format!(
                    "unable to calculate augmented stabilizer rank: {error}"
                ),
            })?;

    Ok(base_rank == augmented_rank)
}

fn next_verification_generator_id(
    group: &StabilizerGroup,
) -> QecResult<usize> {
    group
        .generators()
        .iter()
        .map(|generator| generator.id())
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(QecError::NumericalFailure {
            operation: "verification generator ID + 1",
        })
}

// ============================================================================
// Stabilizer equivalence
// ============================================================================

/// Verifies whether two Pauli operators are equivalent modulo the supplied
/// stabilizer group.
///
/// For phase-free Pauli operators:
///
/// ```text
/// A ≡ B (mod S)
/// iff
/// A * B ∈ S
/// ```
///
/// Since the Pauli representation is phase-free, multiplication is also
/// phase-insensitive for the purpose of this test.
pub fn verify_stabilizer_equivalence(
    first: &PauliString,
    second: &PauliString,
    group: &StabilizerGroup,
    limits: &QecLimits,
) -> QecResult<EquivalenceResult> {
    validate_limits(limits)?;
    validate_dimensions(first, second)?;
    validate_operator(first, limits)?;
    validate_operator(second, limits)?;
    validate_group(group, limits)?;

    if first.num_qubits() != group.num_qubits() {
        return Err(QecError::InvalidInput {
            message: format!(
                "operator/group dimension mismatch: {} != {}",
                first.num_qubits(),
                group.num_qubits()
            ),
        });
    }

    check_operation_budget(2, limits)?;

    let difference =
        first
            .multiply(second)
            .map_err(|error| QecError::InvalidInput {
                message: format!(
                    "unable to calculate Pauli difference: {error}"
                ),
            })?;

    if verify_stabilizer_membership(
        &difference,
        group,
        limits,
    )? {
        Ok(EquivalenceResult::Equivalent)
    } else {
        Ok(EquivalenceResult::NotEquivalent)
    }
}

/// Produces a structured stabilizer-equivalence report.
pub fn verify_operator_pair(
    first: &PauliString,
    second: &PauliString,
    group: &StabilizerGroup,
    mode: VerificationMode,
    limits: &QecLimits,
) -> QecResult<VerificationReport> {
    let result =
        verify_stabilizer_equivalence(
            first,
            second,
            group,
            limits,
        )?;

    let status = match result {
        EquivalenceResult::Equivalent => {
            VerificationStatus::Verified
        }
        EquivalenceResult::NotEquivalent => {
            VerificationStatus::VerifiedNotEquivalent
        }
    };

    Ok(VerificationReport::new(
        VerificationKind::OperatorPair,
        mode,
        status,
        group.num_qubits(),
        group.len(),
        2,
    ))
}

// ============================================================================
// Logical-operator verification
// ============================================================================

/// Verifies whether an operator is a non-trivial logical operator.
///
/// A non-trivial logical operator must:
///
/// 1. commute with every stabilizer;
/// 2. not itself belong to the stabilizer group.
///
/// Therefore:
///
/// ```text
/// P ∈ N(S) \\ S
/// ```
pub fn verify_logical_operator(
    operator: &PauliString,
    group: &StabilizerGroup,
    limits: &QecLimits,
) -> QecResult<bool> {
    validate_limits(limits)?;
    validate_operator(operator, limits)?;
    validate_group(group, limits)?;

    let normalizer =
        verify_normalizer_membership(
            operator,
            group,
            limits,
        )?;

    if !normalizer.is_in_normalizer() {
        return Ok(false);
    }

    Ok(!verify_stabilizer_membership(
        operator,
        group,
        limits,
    )?)
}

/// Produces a logical-operator verification report.
pub fn verify_logical_operator_report(
    operator: &PauliString,
    group: &StabilizerGroup,
    mode: VerificationMode,
    limits: &QecLimits,
) -> QecResult<VerificationReport> {
    let normalizer =
        verify_normalizer_membership(
            operator,
            group,
            limits,
        )?;

    if !normalizer.is_in_normalizer() {
        return Ok(VerificationReport::new(
            VerificationKind::LogicalOperator,
            mode,
            VerificationStatus::VerifiedDetectableError,
            group.num_qubits(),
            group.len(),
            group.len(),
        ));
    }

    let stabilizer =
        verify_stabilizer_membership(
            operator,
            group,
            limits,
        )?;

    let status = if stabilizer {
        VerificationStatus::VerifiedStabilizer
    } else {
        VerificationStatus::VerifiedLogical
    };

    Ok(VerificationReport::new(
        VerificationKind::LogicalOperator,
        mode,
        status,
        group.num_qubits(),
        group.len(),
        group.len().saturating_add(1),
    ))
}

// ============================================================================
// Correction verification
// ============================================================================

/// Verifies that a proposed correction is a valid normalizer-preserving
/// correction for a stabilizer code.
///
/// A correction is considered structurally valid when it commutes with the
/// supplied stabilizer group.
///
/// This deliberately does not decide whether the correction is the *best*
—optimality belongs to the decoder layer.
pub fn verify_correction(
    correction: &PauliString,
    group: &StabilizerGroup,
    limits: &QecLimits,
) -> QecResult<bool> {
    let normalizer =
        verify_normalizer_membership(
            correction,
            group,
            limits,
        )?;

    Ok(normalizer.is_in_normalizer())
}

/// Produces a structured correction-verification report.
pub fn verify_correction_report(
    correction: &PauliString,
    group: &StabilizerGroup,
    mode: VerificationMode,
    limits: &QecLimits,
) -> QecResult<VerificationReport> {
    let result =
        verify_correction(correction, group, limits)?;

    let status = if result {
        VerificationStatus::Verified
    } else {
        VerificationStatus::VerifiedNotEquivalent
    };

    Ok(VerificationReport::new(
        VerificationKind::Correction,
        mode,
        status,
        group.num_qubits(),
        group.len(),
        group.len(),
    ))
}

// ============================================================================
// Complete verification
// ============================================================================

/// Performs the complete structural verification sequence for an operator.
///
/// The sequence is deliberately ordered:
///
/// ```text
/// validate group
///       ↓
/// validate operator
///       ↓
/// normalizer test
///       ↓
/// stabilizer membership
///       ↓
/// classify
/// ```
///
/// Classification:
///
/// ```text
/// stabilizer       -> VerifiedStabilizer
/// normalizer \\ S   -> VerifiedLogical
/// outside normalizer
///                  -> VerifiedDetectableError
/// ```
pub fn verify_all(
    operator: &PauliString,
    group: &StabilizerGroup,
    mode: VerificationMode,
    limits: &QecLimits,
) -> QecResult<VerificationReport> {
    validate_limits(limits)?;
    validate_operator(operator, limits)?;
    validate_group(group, limits)?;

    if operator.num_qubits() != group.num_qubits() {
        return Err(QecError::InvalidInput {
            message: format!(
                "operator/group dimension mismatch: {} != {}",
                operator.num_qubits(),
                group.num_qubits()
            ),
        });
    }

    if operator.is_identity() {
        return Ok(VerificationReport::new(
            VerificationKind::Identity,
            mode,
            VerificationStatus::Verified,
            group.num_qubits(),
            group.len(),
            1,
        ));
    }

    let normalizer =
        verify_normalizer_membership(
            operator,
            group,
            limits,
        )?;

    if !normalizer.is_in_normalizer() {
        return Ok(VerificationReport::new(
            VerificationKind::DetectableError,
            mode,
            VerificationStatus::VerifiedDetectableError,
            group.num_qubits(),
            group.len(),
            group.len(),
        ));
    }

    let stabilizer =
        verify_stabilizer_membership(
            operator,
            group,
            limits,
        )?;

    let status = if stabilizer {
        VerificationStatus::VerifiedStabilizer
    } else {
        VerificationStatus::VerifiedLogical
    };

    Ok(VerificationReport::new(
        VerificationKind::LogicalOperator,
        mode,
        status,
        group.num_qubits(),
        group.len(),
        group.len().saturating_add(1),
    ))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> QecLimits {
        QecLimits::default()
    }

    fn pauli(value: &str) -> PauliString {
        PauliString::from_str(value)
            .expect("test Pauli string must be valid")
    }

    fn stabilizer_group() -> StabilizerGroup {
        let mut group =
            StabilizerGroup::new(2)
                .expect("two-qubit group");

        let generator =
            super::super::stabilizer::StabilizerGenerator::new(
                0,
                pauli("ZZ"),
            )
            .expect("valid generator");

        group
            .add_generator_with_limits(
                generator,
                &limits(),
            )
            .expect("valid stabilizer");

        group
    }

    #[test]
    fn identity_is_verified() {
        let identity = pauli("II");

        let report =
            verify_identity(
                &identity,
                &limits(),
            )
            .expect("identity verification");

        assert_eq!(
            report.status(),
            VerificationStatus::Verified
        );
    }

    #[test]
    fn stabilizer_group_is_verified() {
        let group = stabilizer_group();

        let report =
            verify_stabilizer_group(
                &group,
                &limits(),
            )
            .expect("group verification");

        assert_eq!(
            report.status(),
            VerificationStatus::Verified
        );
    }

    #[test]
    fn_stabilizer_is_in_normalizer() {
        let group = stabilizer_group();

        let result =
            verify_normalizer_membership(
                &pauli("ZZ"),
                &group,
                &limits(),
            )
            .expect("normalizer verification");

        assert_eq!(
            result,
            NormalizerResult::InNormalizer
        );
    }

    #[test]
    fn anticommting_operator_is_not_in_normalizer() {
        let group = stabilizer_group();

        let result =
            verify_normalizer_membership(
                &pauli("XI"),
                &group,
                &limits(),
            )
            .expect("normalizer verification");

        assert_eq!(
            result,
            NormalizerResult::NotInNormalizer
        );
    }

    #[test]
    fn stabilizer_is_member() {
        let group = stabilizer_group();

        assert!(
            verify_stabilizer_membership(
                &pauli("ZZ"),
                &group,
                &limits(),
            )
            .expect("membership verification")
        );
    }

    #[test]
    fn identity_is_member_of_stabilizer_span() {
        let group = stabilizer_group();

        assert!(
            verify_stabilizer_membership(
                &pauli("II"),
                &group,
                &limits(),
            )
            .expect("membership verification")
        );
    }

    #[test]
    fn stabilizer_equivalence_is_reflexive() {
        let group = stabilizer_group();
        let operator = pauli("XX");

        assert_eq!(
            verify_stabilizer_equivalence(
                &operator,
                &operator,
                &group,
                &limits(),
            )
            .expect("equivalence verification"),
            EquivalenceResult::Equivalent
        );
    }

    #[test]
    fn stabilizer_multiplication_preserves_equivalence() {
        let group = stabilizer_group();

        let first = pauli("XX");
        let second = pauli("YY");

        assert_eq!(
            verify_stabilizer_equivalence(
                &first,
                &second,
                &group,
                &limits(),
            )
            .expect("equivalence verification"),
            EquivalenceResult::Equivalent
        );
    }

    #[test]
    fn non_equivalent_operators_are_rejected() {
        let group = stabilizer_group();

        assert_eq!(
            verify_stabilizer_equivalence(
                &pauli("XI"),
                &pauli("IX"),
                &group,
                &limits(),
            )
            .expect("equivalence verification"),
            EquivalenceResult::NotEquivalent
        );
    }

    #[test]
    fn non_trivial_normalizer_operator_is_logical() {
        let group = stabilizer_group();

        //
        // XX commutes with ZZ but is not generated by ZZ.
        //
        assert!(
            verify_logical_operator(
                &pauli("XX"),
                &group,
                &limits(),
            )
            .expect("logical verification")
        );
    }

    #[test]
    fn detectable_error_is_not_logical() {
        let group = stabilizer_group();

        assert!(
            !verify_logical_operator(
                &pauli("XI"),
                &group,
                &limits(),
            )
            .expect("logical verification")
        );
    }

    #[test]
    fn complete_verification_classifies_stabilizer() {
        let group = stabilizer_group();

        let report =
            verify_all(
                &pauli("ZZ"),
                &group,
                VerificationMode::Bounded,
                &limits(),
            )
            .expect("complete verification");

        assert_eq!(
            report.status(),
            VerificationStatus::VerifiedStabilizer
        );
    }

    #[test]
    fn complete_verification_classifies_logical() {
        let group = stabilizer_group();

        let report =
            verify_all(
                &pauli("XX"),
                &group,
                VerificationMode::Bounded,
                &limits(),
            )
            .expect("complete verification");

        assert_eq!(
            report.status(),
            VerificationStatus::VerifiedLogical
        );
    }

    #[test]
    fn complete_verification_classifies_detectable_error() {
        let group = stabilizer_group();

        let report =
            verify_all(
                &pauli("XI"),
                &group,
                VerificationMode::Bounded,
                &limits(),
            )
            .expect("complete verification");

        assert_eq!(
            report.status(),
            VerificationStatus::VerifiedDetectableError
        );
    }

    #[test]
    fn mismatched_dimensions_are_rejected() {
        let group = stabilizer_group();

        let result =
            verify_stabilizer_equivalence(
                &pauli("XX"),
                &pauli("XXX"),
                &group,
                &limits(),
            );

        assert!(result.is_err());
    }

    #[test]
    fn verification_is_deterministic() {
        let group = stabilizer_group();
        let operator = pauli("XX");

        let first =
            verify_all(
                &operator,
                &group,
                VerificationMode::Bounded,
                &limits(),
            )
            .expect("first verification");

        let second =
            verify_all(
                &operator,
                &group,
                VerificationMode::Bounded,
                &limits(),
            )
            .expect("second verification");

        assert_eq!(first, second);
    }
}