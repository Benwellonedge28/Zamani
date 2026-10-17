
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical numerical reproducibility.
//!
//! File: src/classical/numeric/reproducibility.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Define explicit reproducibility requirements and deterministic numerical
//! reduction algorithms for concrete classical floating-point computation.
//!
//! # Authority and ownership
//!
//! Canonical numeric representation:
//!   crate::quantum::ir::classical::float::ClassicalFloat
//!
//! Classical numerical algorithms:
//!   crate::classical::numeric
//!
//! Normative classical semantics:
//!   grammar/spec/classical.md
//!
//! Syntax authorities:
//!   grammar/classical/numeric.g4
//!   grammar/classical/numerical.g4
//!   grammar/expressions/arithmetic.g4
//!   grammar/expressions/expressions.g4
//!
//! This module owns reproducibility policy values, their validation, and
//! deterministic reduction order. It does not define numeric representations,
//! parse source syntax, perform type inference, choose hardware, schedule
//! distributed workers, or create another IR.
//!
//! # Reproducibility levels
//!
//! Semantic:
//!   The observable program meaning must be preserved.
//!
//! Numerical:
//!   The result must satisfy the numerical guarantees explicitly defined by
//!   the applicable type, algorithm, and caller contract.
//!
//! Bitwise:
//!   The relevant result representation must match bit for bit within the
//!   scope of the guarantee declared by the execution contract.
//!
//! These levels are not interchangeable. A deterministic reduction order
//! alone does not prove that different processors, compilers, math libraries,
//! or floating-point execution modes will produce identical bits.
//!
//! # Numerical contract
//!
//! - CanonicalPairwise uses a deterministic, input-order-preserving binary
//!   reduction tree.
//! - InputOrder evaluates values from first to last.
//! - Arithmetic uses checked ClassicalFloat operations.
//! - Non-finite results and unrepresentable intermediate results are errors.
//! - Inputs are borrowed and never modified.
//! - No randomness, wall-clock time, environment, or hardware discovery is
//!   consulted.
//! - No implicit precision conversion or tolerance is introduced.
//!
//! # Scalability contract
//!
//! This module defines no universal observation-count, vector-length, worker,
//! device, or machine-capacity limit.
//!
//! The algorithms operate on the slice supplied by the caller. Actual work is
//! limited by the representable input, execution time, and available resources.
//! CanonicalPairwise uses logarithmic recursion depth and requires no working
//! allocation.
//!
//! This module does not promise infinite physical resources, arbitrary
//! precision, or bitwise identity across all possible targets.
//!
//! # Integration contract
//!
//! 1. Register `pub mod reproducibility;` in
//!    `src/classical/numeric/mod.rs`.
//! 2. Expose `numeric` from `src/classical/mod.rs`, preserving existing module
//!    declarations and public API compatibility.
//! 3. Semantic analysis selects an explicit policy when the program or
//!    compilation configuration requests reproducibility guarantees.
//! 4. Numeric algorithms call these reductions rather than independently
//!    inventing reduction-order policies.
//! 5. The optimizer must preserve the requested contract before reordering,
//!    fusing, vectorizing, or parallelizing arithmetic.
//! 6. The execution planner verifies target support for any requested
//!    cross-target or bitwise guarantee.
//! 7. Distributed reductions must use a separately specified canonical
//!    partition and merge order if results must remain invariant across
//!    worker counts or partition layouts.
//! 8. Grammar remains unchanged: reproducibility is semantic/compiler/runtime
//!    policy, not a new arithmetic precedence rule.
//!
//! # Freeze rule
//!
//! Changes to reproducibility-level meaning, reduction ordering, arithmetic
//! error behavior, or policy validation require a compatibility review and
//! conformance tests.

use core::fmt;

use crate::quantum::ir::classical::float::{
    ClassicalFloat,
    ClassicalFloatError,
};

/// Version of the reproducibility contract implemented by this module.
///
/// Increment this when the externally observable policy or reduction
/// semantics change incompatibly.
pub const REPRODUCIBILITY_SCHEMA_VERSION: u16 = 1;

/// Stable identifier for the canonical pairwise reduction algorithm.
///
/// This identifies the algorithm contract, not a guarantee that every target
/// will produce the same result bits under every compiler configuration.
pub const CANONICAL_PAIRWISE_ALGORITHM_ID: &str =
    "zamani.classical.reduction.pairwise.v1";

/// Stable identifier for left-to-right input-order reduction.
pub const INPUT_ORDER_ALGORITHM_ID: &str =
    "zamani.classical.reduction.input-order.v1";

/// Strength of the reproducibility guarantee requested by a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReproducibilityLevel {
    /// Preserve observable semantic meaning without requiring identical
    /// floating-point bits.
    Semantic,

    /// Preserve the numerical guarantees specified by the selected numeric
    /// type, algorithm, and execution contract.
    Numerical,

    /// Require bitwise identity within the explicitly verified execution
    /// scope. This level requires downstream target/compiler verification.
    Bitwise,
}

impl ReproducibilityLevel {
    /// Returns whether the selected level requires bitwise verification by
    /// the compiler and execution environment.
    pub const fn requires_bitwise_verification(self) -> bool {
        matches!(self, Self::Bitwise)
    }
}

impl fmt::Display for ReproducibilityLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Semantic => formatter.write_str("semantic"),
            Self::Numerical => formatter.write_str("numerical"),
            Self::Bitwise => formatter.write_str("bitwise"),
        }
    }
}

/// Ordering rule used to combine floating-point values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReductionOrder {
    /// Evaluate values in their original order, from first to last.
    ///
    /// This preserves sequential accumulation order but may produce different
    /// results if a caller changes the input order.
    InputOrder,

    /// Use a deterministic binary reduction tree derived only from the input
    /// slice's order and length.
    ///
    /// The grouping does not depend on worker scheduling. Floating-point
    /// addition is not associative, so this grouping is part of the algorithm
    /// contract and must not be silently changed.
    CanonicalPairwise,
}

impl ReductionOrder {
    /// Returns the stable algorithm identifier for this reduction order.
    pub const fn algorithm_id(self) -> &'static str {
        match self {
            Self::InputOrder => INPUT_ORDER_ALGORITHM_ID,
            Self::CanonicalPairwise => CANONICAL_PAIRWISE_ALGORITHM_ID,
        }
    }
}

/// Contract governing whether an implementation may contract separate
/// floating-point operations into a fused operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContractionPolicy {
    /// The compiler or backend may use a fused operation when allowed by the
    /// selected numeric semantics.
    Permitted,

    /// Separate operations must not be contracted into a fused operation
    /// where that would alter the specified result.
    Forbidden,
}

/// A validated, immutable numerical reproducibility policy.
///
/// The policy records requirements; it does not itself configure compiler
/// flags, floating-point hardware, target math libraries, or distributed
/// execution. Those are responsibilities of downstream compiler/runtime
/// components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReproducibilityPolicy {
    level: ReproducibilityLevel,
    reduction_order: ReductionOrder,
    contraction: ContractionPolicy,
}

impl ReproducibilityPolicy {
    /// Creates a policy and validates internal consistency.
    ///
    /// A bitwise policy requires a canonical reduction order and forbids
    /// floating-point contraction. These are necessary policy constraints,
    /// not sufficient proof of cross-target bitwise identity.
    pub fn new(
        level: ReproducibilityLevel,
        reduction_order: ReductionOrder,
        contraction: ContractionPolicy,
    ) -> Result<Self, ReproducibilityError> {
        if level == ReproducibilityLevel::Bitwise {
            if reduction_order != ReductionOrder::CanonicalPairwise {
                return Err(ReproducibilityError::InvalidPolicy {
                    reason:
                        "bitwise reproducibility requires canonical pairwise reduction",
                });
            }

            if contraction != ContractionPolicy::Forbidden {
                return Err(ReproducibilityError::InvalidPolicy {
                    reason:
                        "bitwise reproducibility forbids arithmetic contraction that changes results",
                });
            }
        }

        Ok(Self {
            level,
            reduction_order,
            contraction,
        })
    }

    /// Creates a semantic-level policy with input-order reduction.
    pub const fn semantic() -> Self {
        Self {
            level: ReproducibilityLevel::Semantic,
            reduction_order: ReductionOrder::InputOrder,
            contraction: ContractionPolicy::Permitted,
        }
    }

    /// Creates a numerical policy using a canonical pairwise reduction.
    ///
    /// This gives the reduction a stable grouping for the same ordered input.
    /// It does not establish an arbitrary cross-target error bound; that bound
    /// must be supplied by the selected numeric algorithm and type contract.
    pub const fn numerical() -> Self {
        Self {
            level: ReproducibilityLevel::Numerical,
            reduction_order: ReductionOrder::CanonicalPairwise,
            contraction: ContractionPolicy::Forbidden,
        }
    }

    /// Creates a bitwise policy for downstream verification.
    ///
    /// Successful construction means the policy is internally consistent.
    /// The compiler/runtime must still verify that the requested target,
    /// compiler options, numeric type, and execution mode can meet the
    /// bitwise guarantee within its declared scope.
    pub const fn bitwise() -> Self {
        Self {
            level: ReproducibilityLevel::Bitwise,
            reduction_order: ReductionOrder::CanonicalPairwise,
            contraction: ContractionPolicy::Forbidden,
        }
    }

    /// Returns the requested guarantee level.
    pub const fn level(self) -> ReproducibilityLevel {
        self.level
    }

    /// Returns the reduction-order requirement.
    pub const fn reduction_order(self) -> ReductionOrder {
        self.reduction_order
    }

    /// Returns the floating-point contraction requirement.
    pub const fn contraction(self) -> ContractionPolicy {
        self.contraction
    }

    /// Returns the stable identifier of the selected reduction algorithm.
    pub const fn algorithm_id(self) -> &'static str {
        self.reduction_order.algorithm_id()
    }

    /// Indicates whether downstream target/compiler verification is required
    /// before claiming the requested guarantee has been met.
    pub const fn requires_target_verification(self) -> bool {
        self.level.requires_bitwise_verification()
    }
}

impl Default for ReproducibilityPolicy {
    /// Defaults to semantic reproducibility rather than falsely promising
    /// numerical or bitwise identity across arbitrary targets.
    fn default() -> Self {
        Self::semantic()
    }
}

/// Errors raised by reproducibility policies and deterministic reductions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReproducibilityError {
    /// The policy combines requirements that cannot be satisfied together.
    InvalidPolicy {
        /// Stable explanation suitable for diagnostics and tests.
        reason: &'static str,
    },

    /// Two paired inputs must have the same length.
    LengthMismatch {
        /// Length of the left input.
        left: usize,
        /// Length of the right input.
        right: usize,
    },

    /// An operation requires at least one observation.
    EmptyInput,

    /// A canonical numeric operation failed.
    Numeric(ClassicalFloatError),
}

impl fmt::Display for ReproducibilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPolicy { reason } => {
                write!(formatter, "invalid reproducibility policy: {reason}")
            }
            Self::LengthMismatch { left, right } => write!(
                formatter,
                "paired reduction lengths differ: left has {left}, right has {right}"
            ),
            Self::EmptyInput => {
                formatter.write_str("operation requires non-empty input")
            }
            Self::Numeric(error) => {
                write!(formatter, "reproducible numeric operation failed: {error}")
            }
        }
    }
}

impl std::error::Error for ReproducibilityError {}

impl From<ClassicalFloatError> for ReproducibilityError {
    fn from(error: ClassicalFloatError) -> Self {
        Self::Numeric(error)
    }
}

/// Result type for reproducibility policy and reduction operations.
pub type ReproducibilityResult<T> = Result<T, ReproducibilityError>;

/// Sums values in their original input order using checked arithmetic.
///
/// An empty slice produces canonical positive zero. The input is not modified.
///
/// This function provides stable sequential ordering for the same ordered
/// input. It does not promise that permutations of the input produce the same
/// floating-point result.
pub fn ordered_sum(
    values: &[ClassicalFloat],
) -> ReproducibilityResult<ClassicalFloat> {
    let mut total = ClassicalFloat::zero();

    for value in values.iter().copied() {
        total = total.checked_add(value)?;
    }

    Ok(total)
}

/// Sums values using a deterministic binary pairwise reduction.
///
/// The reduction tree is determined by the input slice's order and length.
/// No allocation is required, and the recursive depth grows logarithmically
/// with the number of observations.
///
/// An empty slice produces canonical positive zero. The input is not modified.
///
/// Checked arithmetic rejects unrepresentable intermediate results. This is
/// not arbitrary-precision summation: a mathematically representable final
/// result can still fail if an intermediate result is not representable.
pub fn canonical_sum(
    values: &[ClassicalFloat],
) -> ReproducibilityResult<ClassicalFloat> {
    match values.len() {
        0 => Ok(ClassicalFloat::zero()),
        1 => Ok(values[0]),
        length => {
            let midpoint = length / 2;
            let left = canonical_sum(&values[..midpoint])?;
            let right = canonical_sum(&values[midpoint..])?;
            Ok(left.checked_add(right)?)
        }
    }
}

/// Sums values according to an explicit reduction policy.
///
/// The policy selects ordering only. Its guarantee level remains a requirement
/// for downstream semantic/compiler/runtime verification.
pub fn sum_with_policy(
    values: &[ClassicalFloat],
    policy: ReproducibilityPolicy,
) -> ReproducibilityResult<ClassicalFloat> {
    match policy.reduction_order() {
        ReductionOrder::InputOrder => ordered_sum(values),
        ReductionOrder::CanonicalPairwise => canonical_sum(values),
    }
}

/// Computes a dot product using input-order accumulation.
///
/// Both slices must have equal lengths. Empty inputs produce canonical zero.
/// Each multiplication and addition uses checked canonical arithmetic.
pub fn ordered_dot_product(
    left: &[ClassicalFloat],
    right: &[ClassicalFloat],
) -> ReproducibilityResult<ClassicalFloat> {
    validate_pair_lengths(left, right)?;

    let mut total = ClassicalFloat::zero();

    for (lhs, rhs) in left.iter().copied().zip(right.iter().copied()) {
        let product = lhs.checked_mul(rhs)?;
        total = total.checked_add(product)?;
    }

    Ok(total)
}

/// Computes a dot product using a canonical pairwise reduction tree.
///
/// Products are computed at the leaves and combined according to a fixed
/// binary tree determined by the input order and length. No intermediate
/// vector or allocation is needed.
///
/// The two inputs must have equal lengths. Empty inputs produce canonical
/// zero. Intermediate overflow is reported as an error.
pub fn canonical_dot_product(
    left: &[ClassicalFloat],
    right: &[ClassicalFloat],
) -> ReproducibilityResult<ClassicalFloat> {
    validate_pair_lengths(left, right)?;
    pairwise_dot_product(left, right)
}

/// Computes a dot product according to an explicit reduction policy.
pub fn dot_product_with_policy(
    left: &[ClassicalFloat],
    right: &[ClassicalFloat],
    policy: ReproducibilityPolicy,
) -> ReproducibilityResult<ClassicalFloat> {
    match policy.reduction_order() {
        ReductionOrder::InputOrder => ordered_dot_product(left, right),
        ReductionOrder::CanonicalPairwise => {
            canonical_dot_product(left, right)
        }
    }
}

fn validate_pair_lengths(
    left: &[ClassicalFloat],
    right: &[ClassicalFloat],
) -> ReproducibilityResult<()> {
    if left.len() != right.len() {
        return Err(ReproducibilityError::LengthMismatch {
            left: left.len(),
            right: right.len(),
        });
    }

    Ok(())
}

fn pairwise_dot_product(
    left: &[ClassicalFloat],
    right: &[ClassicalFloat],
) -> ReproducibilityResult<ClassicalFloat> {
    match left.len() {
        0 => Ok(ClassicalFloat::zero()),
        1 => Ok(left[0].checked_mul(right[0])?),
        length => {
            let midpoint = length / 2;
            let left_result =
                pairwise_dot_product(&left[..midpoint], &right[..midpoint])?;
            let right_result =
                pairwise_dot_product(&left[midpoint..], &right[midpoint..])?;

            Ok(left_result.checked_add(right_result)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        canonical_dot_product,
        canonical_sum,
        dot_product_with_policy,
        ordered_dot_product,
        ordered_sum,
        sum_with_policy,
        ContractionPolicy,
        ReductionOrder,
        ReproducibilityError,
        ReproducibilityLevel,
        ReproducibilityPolicy,
    };
    use crate::quantum::ir::classical::float::ClassicalFloat;

    fn f(value: f64) -> ClassicalFloat {
        ClassicalFloat::new(value).expect("test input must be finite")
    }

    fn floats(values: &[f64]) -> Vec<ClassicalFloat> {
        values.iter().copied().map(f).collect()
    }

    #[test]
    fn empty_sum_is_canonical_zero() {
        assert_eq!(canonical_sum(&[]).unwrap(), ClassicalFloat::zero());
        assert_eq!(ordered_sum(&[]).unwrap(), ClassicalFloat::zero());
    }

    #[test]
    fn canonical_sum_is_repeatable_for_identical_ordered_input() {
        let values = floats(&[1.25, 2.5, 3.75, -1.0, 8.0]);

        let first = canonical_sum(&values).unwrap();
        let second = canonical_sum(&values).unwrap();

        assert_eq!(first, second);
    }

    #[test]
    fn input_order_sum_preserves_sequential_accumulation() {
        let values = floats(&[1.0, 2.0, 3.0]);

        assert_eq!(ordered_sum(&values).unwrap().get(), 6.0);
    }

    #[test]
    fn policy_selects_the_reduction_order() {
        let values = floats(&[1.0, 2.0, 3.0]);

        let ordered = ReproducibilityPolicy::new(
            ReproducibilityLevel::Numerical,
            ReductionOrder::InputOrder,
            ContractionPolicy::Forbidden,
        )
        .unwrap();

        let pairwise = ReproducibilityPolicy::numerical();

        assert_eq!(
            sum_with_policy(&values, ordered).unwrap().get(),
            6.0
        );
        assert_eq!(
            sum_with_policy(&values, pairwise).unwrap().get(),
            6.0
        );
    }

    #[test]
    fn bitwise_policy_rejects_input_order_reduction() {
        let result = ReproducibilityPolicy::new(
            ReproducibilityLevel::Bitwise,
            ReductionOrder::InputOrder,
            ContractionPolicy::Forbidden,
        );

        assert!(matches!(
            result,
            Err(ReproducibilityError::InvalidPolicy { .. })
        ));
    }

    #[test]
    fn bitwise_policy_rejects_permitted_contraction() {
        let result = ReproducibilityPolicy::new(
            ReproducibilityLevel::Bitwise,
            ReductionOrder::CanonicalPairwise,
            ContractionPolicy::Permitted,
        );

        assert!(matches!(
            result,
            Err(ReproducibilityError::InvalidPolicy { .. })
        ));
    }

    #[test]
    fn bitwise_policy_requires_downstream_verification() {
        let policy = ReproducibilityPolicy::bitwise();

        assert!(policy.requires_target_verification());
        assert_eq!(
            policy.reduction_order(),
            ReductionOrder::CanonicalPairwise
        );
        assert_eq!(
            policy.contraction(),
            ContractionPolicy::Forbidden
        );
    }

    #[test]
    fn default_policy_does_not_claim_bitwise_identity() {
        let policy = ReproducibilityPolicy::default();

        assert_eq!(policy.level(), ReproducibilityLevel::Semantic);
        assert!(!policy.requires_target_verification());
    }

    #[test]
    fn canonical_dot_product_is_repeatable() {
        let left = floats(&[1.0, 2.0, 3.0, 4.0]);
        let right = floats(&[5.0, 6.0, 7.0, 8.0]);

        let first = canonical_dot_product(&left, &right).unwrap();
        let second = canonical_dot_product(&left, &right).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.get(), 70.0);
    }

    #[test]
    fn ordered_dot_product_computes_expected_value() {
        let left = floats(&[1.0, 2.0, 3.0]);
        let right = floats(&[4.0, 5.0, 6.0]);

        assert_eq!(ordered_dot_product(&left, &right).unwrap().get(), 32.0);
    }

    #[test]
    fn empty_dot_product_is_zero() {
        assert_eq!(
            canonical_dot_product(&[], &[]).unwrap(),
            ClassicalFloat::zero()
        );
    }

    #[test]
    fn dot_product_rejects_length_mismatch() {
        let left = floats(&[1.0, 2.0]);
        let right = floats(&[3.0]);

        assert_eq!(
            canonical_dot_product(&left, &right),
            Err(ReproducibilityError::LengthMismatch {
                left: 2,
                right: 1,
            })
        );
    }

    #[test]
    fn policy_driven_dot_product_uses_selected_algorithm() {
        let left = floats(&[1.0, 2.0, 3.0]);
        let right = floats(&[4.0, 5.0, 6.0]);

        let policy = ReproducibilityPolicy::numerical();

        assert_eq!(
            dot_product_with_policy(&left, &right, policy)
                .unwrap()
                .get(),
            32.0
        );
    }

    #[test]
    fn arithmetic_overflow_is_reported() {
        let values = floats(&[f64::MAX, f64::MAX]);

        assert!(matches!(
            canonical_sum(&values),
            Err(ReproducibilityError::Numeric(_))
        ));
    }

    #[test]
    fn canonical_sum_does_not_mutate_input() {
        let values = floats(&[3.0, 1.0, 4.0, 1.0, 5.0]);
        let original = values.clone();

        let _ = canonical_sum(&values).unwrap();

        assert_eq!(values, original);
    }

    #[test]
    fn policy_exposes_stable_algorithm_identity() {
        assert_eq!(
            ReproducibilityPolicy::numerical().algorithm_id(),
            super::CANONICAL_PAIRWISE_ALGORITHM_ID
        );

        assert_eq!(
            ReproducibilityPolicy::semantic().algorithm_id(),
            super::INPUT_ORDER_ALGORITHM_ID
        );
    }
}
