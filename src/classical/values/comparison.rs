
 //! Zamani — Classical Value Comparison
 //!
 //! File: src/classical/values/comparison.rs
 //! Rust edition: 2021
 //! Minimum Rust version: 1.97
 //! Safety: unsafe Rust is forbidden.
 //!
 //! STATUS
 //!   Proposed implementation. Compile and integration verification are
 //!   required before this file can be marked Verified or Integrated.
 //!
 //! PURPOSE
 //!   Provide reusable, target-independent comparison semantics for canonical
 //!   classical values.
 //!
 //! AUTHORITY AND OWNERSHIP
 //!   This file owns comparison outcomes, comparison errors, and the policy
 //!   for comparing canonical classical values.
 //!
 //!   The canonical value representations remain owned by:
 //!     crate::quantum::ir::classical::value
 //!
 //!   This file does not define a second ClassicalValue, integer, float,
 //!   complex number, symbolic parameter, AST, type system, or IR.
 //!
 //! DOES NOT OWN
 //!   - lexical or grammar rules for equality and relational operators;
 //!   - source-language type checking or implicit-conversion rules;
 //!   - canonical value representation or serialization;
 //!   - expression evaluation, runtime memory, scheduling, or hardware;
 //!   - arbitrary-precision integer arithmetic;
 //!   - symbolic algebra or theorem proving.
 //!
 //! SEMANTIC CONTRACT
 //!   - Integer comparisons are exact and support arbitrary-width canonical
 //!     signed and unsigned integers.
 //!   - Signed/unsigned integer comparisons do not narrow either operand.
 //!   - Floating-point ordering uses numeric comparison; positive and negative
 //!     zero compare numerically equal.
 //!   - Complex values support equality, not less-than/greater-than ordering.
 //!   - Symbolic parameters that cannot be proven equal are indeterminate.
 //!   - Aggregates support semantic equality but have no implicit ordering.
 //!   - No numeric promotion is performed implicitly between integers and
 //!     floats. A future mixed-numeric rule must be explicitly specified.
 //!
 //! POCO-REAF
 //!   This module introduces no fixed hardware, integer-width, collection-size,
 //!   tensor-rank, or operation-count ceiling. It does not promise infinite
 //!   resources. Comparisons use the canonical value representations and
 //!   whatever memory is available to represent the input.
 //!
 //! INTEGRATION CONTRACT
 //!   1. Declare `pub mod comparison;` in `src/classical/values/mod.rs`.
 //!   2. Re-export the public API there if consistent with the finalized
 //!      `values` module visibility policy.
 //!   3. Keep `ClassicalValue`, `BigInt`, `BigUint`, `FiniteFloat`,
 //!      `ComplexValue`, `BitVector`, and `ClassicalValueKind` owned by the
 //!      canonical IR value module.
 //!   4. Route semantic relational/equality evaluation through this API only
 //!      after semantic analysis has validated operand types and operators.
 //!   5. Reconcile `src/quantum/memory/classical.rs` with this contract in a
 //!      separate integration change; do not duplicate runtime memory here.
 //!   6. Grammar files own syntax, while the normative semantic specification
 //!      owns the meaning of comparison operators.
 //!
 //! FREEZE RULE
 //!   Changing numeric promotion, symbolic equality, aggregate equality, or
 //!   ordering rules requires an explicit semantic compatibility review.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::cmp::Ordering;
use core::fmt;

use crate::quantum::ir::classical::value::{
    BigInt,
    BigUint,
    ClassicalValue,
    ClassicalValueKind,
    Sign,
};

/// Result of a relational comparison.
///
/// `Unordered` means the operands do not have a defined less/equal/greater
/// ordering under this API. It is not the same as `Equal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComparisonResult {
    /// The left operand is less than the right operand.
    Less,

    /// The operands are numerically or semantically equal under the
    /// comparison contract.
    Equal,

    /// The left operand is greater than the right operand.
    Greater,

    /// The operands do not have a defined relational ordering.
    Unordered,
}

impl ComparisonResult {
    /// Converts an `Ordering` into a comparison result.
    #[must_use]
    pub const fn from_ordering(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Less => Self::Less,
            Ordering::Equal => Self::Equal,
            Ordering::Greater => Self::Greater,
        }
    }

    /// Returns `true` only for a defined equality result.
    #[must_use]
    pub const fn is_equal(self) -> bool {
        matches!(self, Self::Equal)
    }

    /// Returns `true` when the comparison has no relational ordering.
    #[must_use]
    pub const fn is_unordered(self) -> bool {
        matches!(self, Self::Unordered)
    }

    /// Returns `Some(Ordering)` for an ordered result.
    #[must_use]
    pub const fn as_ordering(self) -> Option<Ordering> {
        match self {
            Self::Less => Some(Ordering::Less),
            Self::Equal => Some(Ordering::Equal),
            Self::Greater => Some(Ordering::Greater),
            Self::Unordered => None,
        }
    }
}

/// Result of a semantic equality query.
///
/// This is separate from relational ordering because complex values and
/// symbolic values can have meaningful equality semantics without having a
/// meaningful less-than/greater-than relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EqualityResult {
    /// The values are semantically equal.
    Equal,

    /// The values are known not to be semantically equal.
    NotEqual,

    /// Equality cannot be established from the values alone.
    ///
    /// For example, distinct symbolic parameters may happen to have equal
    /// runtime values, but this module does not evaluate their definitions.
    Indeterminate,
}

impl EqualityResult {
    /// Returns whether equality was established.
    #[must_use]
    pub const fn is_equal(self) -> bool {
        matches!(self, Self::Equal)
    }

    /// Returns whether inequality was established.
    #[must_use]
    pub const fn is_not_equal(self) -> bool {
        matches!(self, Self::NotEqual)
    }
}

/// Errors raised when a requested comparison is invalid or unsupported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComparisonError {
    /// The requested relational comparison is not defined for this value kind.
    NotOrderable {
        /// Kind of the left operand.
        kind: ClassicalValueKind,
    },

    /// The operands cannot be ordered together without an explicit
    /// language-defined conversion or promotion.
    IncompatibleKinds {
        /// Kind of the left operand.
        left: ClassicalValueKind,

        /// Kind of the right operand.
        right: ClassicalValueKind,
    },

    /// A malformed canonical value was supplied.
    InvalidValue {
        /// Kind of the invalid value.
        kind: ClassicalValueKind,
    },
}

impl fmt::Display for ComparisonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotOrderable { kind } => {
                write!(formatter, "classical value kind '{kind}' is not orderable")
            }
            Self::IncompatibleKinds { left, right } => {
                write!(
                    formatter,
                    "cannot order classical values of kinds '{left}' and '{right}' \
                     without an explicit compatible conversion"
                )
            }
            Self::InvalidValue { kind } => {
                write!(formatter, "invalid canonical value of kind '{kind}'")
            }
        }
    }
}

impl std::error::Error for ComparisonError {}

/// Compares two canonical values for relational ordering.
///
/// Supported ordered values:
/// - signed integers;
/// - unsigned integers;
/// - signed integers against unsigned integers, exactly;
/// - finite floats of the same semantic kind;
/// - angles against angles;
/// - strings using Rust's deterministic UTF-8 string ordering;
/// - bit vectors of equal width, using their canonical byte representation.
///
/// Booleans, complex values, aggregates, references, and symbolic parameters
/// are not relationally ordered by this API.
///
/// Integer/float mixed comparisons are rejected instead of silently
/// converting arbitrary-width integers to finite-precision floats.
pub fn compare(
    left: &ClassicalValue,
    right: &ClassicalValue,
) -> Result<ComparisonResult, ComparisonError> {
    use ClassicalValue as V;

    let ordering = match (left, right) {
        (V::Integer(a), V::Integer(b)) => {
            validate_integer(a, left.kind())?;
            validate_integer(b, right.kind())?;
            Some(a.cmp(b))
        }

        (V::UnsignedInteger(a), V::UnsignedInteger(b)) => {
            validate_unsigned(a, left.kind())?;
            validate_unsigned(b, right.kind())?;
            Some(a.cmp_numeric(b))
        }

        (V::Integer(a), V::UnsignedInteger(b)) => {
            validate_integer(a, left.kind())?;
            validate_unsigned(b, right.kind())?;
            Some(compare_signed_unsigned(a, b))
        }

        (V::UnsignedInteger(a), V::Integer(b)) => {
            validate_unsigned(a, left.kind())?;
            validate_integer(b, right.kind())?;
            Some(compare_signed_unsigned(b, a).reverse())
        }

        (V::Float(a), V::Float(b)) => {
            if !a.get().is_finite() || !b.get().is_finite() {
                return Err(ComparisonError::InvalidValue {
                    kind: ClassicalValueKind::Float,
                });
            }

            a.get().partial_cmp(&b.get())
        }

        (V::Angle(a), V::Angle(b)) => {
            if !a.get().is_finite() || !b.get().is_finite() {
                return Err(ComparisonError::InvalidValue {
                    kind: ClassicalValueKind::Angle,
                });
            }

            a.get().partial_cmp(&b.get())
        }

        (V::String(a), V::String(b)) => Some(a.cmp(b)),

        (V::BitVector(a), V::BitVector(b)) => {
            if a.validate().is_err() || b.validate().is_err() {
                return Err(ComparisonError::InvalidValue {
                    kind: ClassicalValueKind::BitVector,
                });
            }

            if a.width() != b.width() {
                return Err(ComparisonError::IncompatibleKinds {
                    left: ClassicalValueKind::BitVector,
                    right: ClassicalValueKind::BitVector,
                });
            }

            // The canonical representation stores logical bit zero in the
            // least-significant bit of byte zero. Compare from the most
            // significant stored byte toward the least significant byte.
            Some(
                a.as_bytes()
                    .iter()
                    .rev()
                    .cmp(b.as_bytes().iter().rev()),
            )
        }

        (V::Bool(_), V::Bool(_))
        | (V::Bit(_), V::Bit(_))
        | (V::Complex(_), V::Complex(_))
        | (V::Unit, V::Unit)
        | (V::Array(_), V::Array(_))
        | (V::Tuple(_), V::Tuple(_))
        | (V::Optional(_), V::Optional(_))
        | (V::Reference(_), V::Reference(_))
        | (V::Parameter(_), V::Parameter(_)) => {
            return Err(ComparisonError::NotOrderable {
                kind: left.kind(),
            });
        }

        _ => {
            return Err(ComparisonError::IncompatibleKinds {
                left: left.kind(),
                right: right.kind(),
            });
        }
    };

    Ok(match ordering {
        Some(ordering) => ComparisonResult::from_ordering(ordering),
        None => ComparisonResult::Unordered,
    })
}

/// Determines semantic equality between canonical values.
///
/// Unlike derived `PartialEq` on the canonical `ClassicalValue`, this function
/// uses numeric float equality, so positive and negative floating-point zero
/// are semantically equal.
///
/// Signed and unsigned integers are compared exactly without narrowing.
/// Complex values support equality by comparing both components numerically.
/// Arrays, tuples, and optional values are compared iteratively to avoid
/// recursion proportional to aggregate nesting depth.
///
/// Distinct symbolic parameters produce `Indeterminate`: this function does
/// not invoke a symbolic solver or evaluate runtime bindings.
pub fn semantic_equal(
    left: &ClassicalValue,
    right: &ClassicalValue,
) -> Result<EqualityResult, ComparisonError> {
    use ClassicalValue as V;

    // Explicit work stack: deeply nested values do not recursively call this
    // function, avoiding call-stack growth proportional to aggregate depth.
    let mut pending: Vec<(&ClassicalValue, &ClassicalValue)> =
        vec![(left, right)];
    let mut indeterminate = false;

    while let Some((a, b)) = pending.pop() {
        match (a, b) {
            (V::Unit, V::Unit) => {}

            (V::Bool(x), V::Bool(y)) | (V::Bit(x), V::Bit(y)) => {
                if x != y {
                    return Ok(EqualityResult::NotEqual);
                }
            }

            (V::BitVector(x), V::BitVector(y)) => {
                if x.validate().is_err() || y.validate().is_err() {
                    return Err(ComparisonError::InvalidValue {
                        kind: ClassicalValueKind::BitVector,
                    });
                }

                if x != y {
                    return Ok(EqualityResult::NotEqual);
                }
            }

            (V::Integer(x), V::Integer(y)) => {
                validate_integer(x, a.kind())?;
                validate_integer(y, b.kind())?;

                if x != y {
                    return Ok(EqualityResult::NotEqual);
                }
            }

            (V::UnsignedInteger(x), V::UnsignedInteger(y)) => {
                validate_unsigned(x, a.kind())?;
                validate_unsigned(y, b.kind())?;

                if x != y {
                    return Ok(EqualityResult::NotEqual);
                }
            }

            (V::Integer(x), V::UnsignedInteger(y)) => {
                validate_integer(x, a.kind())?;
                validate_unsigned(y, b.kind())?;

                if compare_signed_unsigned(x, y) != Ordering::Equal {
                    return Ok(EqualityResult::NotEqual);
                }
            }

            (V::UnsignedInteger(x), V::Integer(y)) => {
                validate_unsigned(x, a.kind())?;
                validate_integer(y, b.kind())?;

                if compare_signed_unsigned(y, x) != Ordering::Equal {
                    return Ok(EqualityResult::NotEqual);
                }
            }

            (V::Float(x), V::Float(y)) | (V::Angle(x), V::Angle(y)) => {
                if !x.get().is_finite() || !y.get().is_finite() {
                    return Err(ComparisonError::InvalidValue {
                        kind: a.kind(),
                    });
                }

                if x.get() != y.get() {
                    return Ok(EqualityResult::NotEqual);
                }
            }

            (V::Complex(x), V::Complex(y)) => {
                if x.validate().is_err() || y.validate().is_err() {
                    return Err(ComparisonError::InvalidValue {
                        kind: ClassicalValueKind::Complex,
                    });
                }

                if x.real.get() != y.real.get()
                    || x.imaginary.get() != y.imaginary.get()
                {
                    return Ok(EqualityResult::NotEqual);
                }
            }

            (V::String(x), V::String(y)) => {
                if x != y {
                    return Ok(EqualityResult::NotEqual);
                }
            }

            (V::Array(x), V::Array(y)) | (V::Tuple(x), V::Tuple(y)) => {
                if x.len() != y.len() {
                    return Ok(EqualityResult::NotEqual);
                }

                pending.extend(x.iter().zip(y.iter()));
            }

            (V::Optional(x), V::Optional(y)) => match (x, y) {
                (None, None) => {}
                (Some(x), Some(y)) => pending.push((x, y)),
                _ => return Ok(EqualityResult::NotEqual),
            },

            (V::Reference(x), V::Reference(y)) => {
                if x != y {
                    return Ok(EqualityResult::NotEqual);
                }
            }

            (V::Parameter(x), V::Parameter(y)) => {
                if x != y {
                    indeterminate = true;
                }
            }

            _ => return Ok(EqualityResult::NotEqual),
        }
    }

    if indeterminate {
        Ok(EqualityResult::Indeterminate)
    } else {
        Ok(EqualityResult::Equal)
    }
}

/// Performs exact equality for two unsigned arbitrary-width integers.
///
/// This helper is public so callers that already hold canonical integer
/// representations can reuse the same cross-representation contract.
pub fn compare_unsigned(
    left: &BigUint,
    right: &BigUint,
) -> Result<ComparisonResult, ComparisonError> {
    validate_unsigned(left, ClassicalValueKind::UnsignedInteger)?;
    validate_unsigned(right, ClassicalValueKind::UnsignedInteger)?;

    Ok(ComparisonResult::from_ordering(left.cmp_numeric(right)))
}

/// Performs exact ordering for two signed arbitrary-width integers.
pub fn compare_signed(
    left: &BigInt,
    right: &BigInt,
) -> Result<ComparisonResult, ComparisonError> {
    validate_integer(left, ClassicalValueKind::Integer)?;
    validate_integer(right, ClassicalValueKind::Integer)?;

    Ok(ComparisonResult::from_ordering(left.cmp(right)))
}

fn compare_signed_unsigned(
    signed: &BigInt,
    unsigned: &BigUint,
) -> Ordering {
    if signed.is_negative() {
        Ordering::Less
    } else {
        signed.magnitude().cmp_numeric(unsigned)
    }
}

fn validate_integer(
    value: &BigInt,
    kind: ClassicalValueKind,
) -> Result<(), ComparisonError> {
    value.validate().map_err(|_| ComparisonError::InvalidValue { kind })
}

fn validate_unsigned(
    value: &BigUint,
    kind: ClassicalValueKind,
) -> Result<(), ComparisonError> {
    value.validate().map_err(|_| ComparisonError::InvalidValue { kind })
}

#[cfg(test)]
mod tests {
    use super::{
        compare,
        compare_signed,
        compare_unsigned,
        semantic_equal,
        ComparisonError,
        ComparisonResult,
        EqualityResult,
    };
    use crate::quantum::ir::classical::value::{
        BigInt,
        BigUint,
        ClassicalValue,
    };

    #[test]
    fn signed_arbitrary_width_comparison_is_exact() {
        let lower = BigInt::from_i128(-900);
        let higher = BigInt::from_i128(-12);

        assert_eq!(
            compare_signed(&lower, &higher),
            Ok(ComparisonResult::Less)
        );
        assert_eq!(
            compare_signed(&higher, &lower),
            Ok(ComparisonResult::Greater)
        );
    }

    #[test]
    fn unsigned_arbitrary_width_comparison_is_exact() {
        let lower = BigUint::from_be_bytes(&[0x01, 0x00]);
        let higher = BigUint::from_be_bytes(&[0x01, 0x01]);

        assert_eq!(
            compare_unsigned(&lower, &higher),
            Ok(ComparisonResult::Less)
        );
    }

    #[test]
    fn signed_unsigned_comparison_does_not_narrow() {
        let negative = ClassicalValue::integer(-1_i128);
        let zero = ClassicalValue::unsigned_integer(0_u128);

        assert_eq!(compare(&negative, &zero), Ok(ComparisonResult::Less));
        assert_eq!(compare(&zero, &negative), Ok(ComparisonResult::Greater));
    }

    #[test]
    fn floats_compare_numerically_including_signed_zero() {
        let positive_zero = ClassicalValue::float(0.0).expect("finite");
        let negative_zero = ClassicalValue::float(-0.0).expect("finite");

        assert_eq!(
            compare(&positive_zero, &negative_zero),
            Ok(ComparisonResult::Equal)
        );
        assert_eq!(
            semantic_equal(&positive_zero, &negative_zero),
            Ok(EqualityResult::Equal)
        );
    }

    #[test]
    fn complex_values_have_equality_but_no_ordering() {
        let first = ClassicalValue::complex(2.0, 3.0).expect("finite");
        let same = ClassicalValue::complex(2.0, 3.0).expect("finite");
        let different = ClassicalValue::complex(2.0, 4.0).expect("finite");

        assert_eq!(
            compare(&first, &same),
            Err(ComparisonError::NotOrderable {
                kind: first.kind(),
            })
        );
        assert_eq!(
            semantic_equal(&first, &same),
            Ok(EqualityResult::Equal)
        );
        assert_eq!(
            semantic_equal(&first, &different),
            Ok(EqualityResult::NotEqual)
        );
    }

    #[test]
    fn mixed_integer_float_ordering_requires_an_explicit_rule() {
        let integer = ClassicalValue::integer(1_i128);
        let float = ClassicalValue::float(1.0).expect("finite");

        assert!(matches!(
            compare(&integer, &float),
            Err(ComparisonError::IncompatibleKinds { .. })
        ));
        assert_eq!(
            semantic_equal(&integer, &float),
            Ok(EqualityResult::NotEqual)
        );
    }

    #[test]
    fn symbolic_parameter_equality_is_not_guessed() {
        let value = ClassicalValue::Parameter(
            crate::quantum::ir::parameter::Parameter::new("x")
                .expect("valid parameter"),
        );
        let other = ClassicalValue::Parameter(
            crate::quantum::ir::parameter::Parameter::new("y")
                .expect("valid parameter"),
        );

        assert_eq!(
            semantic_equal(&value, &value),
            Ok(EqualityResult::Equal)
        );
        assert_eq!(
            semantic_equal(&value, &other),
            Ok(EqualityResult::Indeterminate)
        );
    }

    #[test]
    fn strings_are_ordered_lexicographically() {
        let a = ClassicalValue::string("alpha");
        let b = ClassicalValue::string("beta");

        assert_eq!(compare(&a, &b), Ok(ComparisonResult::Less));
    }

    #[test]
    fn aggregate_equality_is_structural_and_iterative() {
        let left = ClassicalValue::tuple(vec![
            ClassicalValue::integer(1_i128),
            ClassicalValue::float(2.0).expect("finite"),
        ]);
        let right = ClassicalValue::tuple(vec![
            ClassicalValue::integer(1_i128),
            ClassicalValue::float(2.0).expect("finite"),
        ]);

        assert_eq!(
            semantic_equal(&left, &right),
            Ok(EqualityResult::Equal)
        );
        assert!(matches!(
            compare(&left, &right),
            Err(ComparisonError::NotOrderable { .. })
        ));
    }

    #[test]
    fn cross_signed_unsigned_zero_is_equal() {
        let signed = ClassicalValue::integer(0_i128);
        let unsigned = ClassicalValue::unsigned_integer(0_u128);

        assert_eq!(
            semantic_equal(&signed, &unsigned),
            Ok(EqualityResult::Equal)
        );
    }
}
