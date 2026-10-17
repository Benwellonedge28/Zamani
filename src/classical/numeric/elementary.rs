
```rust
//! Zamani classical numeric elementary functions.
//!
//! File: src/classical/numeric/elementary.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Contract
//!
//! PURPOSE
//!   Expose checked elementary mathematical functions over Zamani's
//!   canonical ClassicalFloat representation.
//!
//! AUTHORITY
//!   Numeric representation and finite-value invariants:
//!     crate::quantum::ir::classical::float
//!
//!   Arithmetic syntax and precedence:
//!     grammar/expressions/arithmetic.g4
//!     grammar/expressions/expressions.g4
//!
//! OWNS
//!   - elementary-function dispatch;
//!   - explicit mathematical domain checks not already performed by
//!     ClassicalFloat;
//!   - elementary-operation error categorization;
//!   - conformance tests for this API.
//!
//! DOES NOT OWN
//!   - floating-point representation, precision, or canonical equality;
//!   - numeric types, implicit conversions, or promotion;
//!   - ASTs, source tokens, or grammar precedence;
//!   - symbolic expression representation;
//!   - canonical IR or runtime execution;
//!   - target-specific math libraries or hardware selection;
//!   - resource discovery or execution scheduling.
//!
//! SEMANTICS
//!   - Inputs and successful outputs are finite ClassicalFloat values.
//!   - Invalid mathematical domains return errors.
//!   - Overflow to infinity and invalid results such as NaN are rejected.
//!   - No implicit conversion to integers or other numeric formats occurs.
//!   - Approximate equality is not used for canonical equality.
//!
//! SCALABILITY
//!   This module defines no machine-count, array-size, or precision ceiling.
//!   The canonical ClassicalFloat implementation currently uses IEEE-754
//!   binary64. Arbitrary-precision and alternate formats require an explicit
//!   numeric type/format contract and must not be silently substituted.
//!
//! INTEGRATION
//!   1. Declare `pub mod elementary;` in numeric/mod.rs.
//!   2. Re-export the public API if desired.
//!   3. Map type-checked numeric expressions to ElementaryFunction.
//!   4. Keep symbolic expressions symbolic in their owning IR.
//!   5. Preserve canonical ClassicalFloat ownership.
//!   6. Treat resource budgets as explicit caller/compiler/runtime policy.
//!
//! STATUS
//!   Implementation draft against inspected canonical float APIs.
//!   Full repository compilation and integration remain to be verified.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use crate::quantum::ir::classical::float::{
    ClassicalFloat,
    ClassicalFloatError,
};

/// Elementary unary mathematical functions.
///
/// This enum describes semantic dispatch, not source tokens. The frontend
/// and semantic analyzer remain responsible for interpreting source syntax
/// and checking operand types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementaryFunction {
    /// Absolute value.
    Abs,

    /// Signum, preserving the canonical floating-point behavior.
    Signum,

    /// Square root.
    Sqrt,

    /// Cube root.
    Cbrt,

    /// Exponential function, e^x.
    Exp,

    /// Exponential function, 2^x.
    Exp2,

    /// Natural logarithm.
    Ln,

    /// Base-2 logarithm.
    Log2,

    /// Base-10 logarithm.
    Log10,

    /// Natural logarithm of 1 + x.
    Ln1p,

    /// Exponential of x minus one.
    Expm1,

    /// Sine.
    Sin,

    /// Cosine.
    Cos,

    /// Tangent.
    Tan,

    /// Inverse sine.
    Asin,

    /// Inverse cosine.
    Acos,

    /// Inverse tangent.
    Atan,

    /// Hyperbolic sine.
    Sinh,

    /// Hyperbolic cosine.
    Cosh,

    /// Hyperbolic tangent.
    Tanh,

    /// Inverse hyperbolic sine.
    Asinh,

    /// Inverse hyperbolic cosine.
    Acosh,

    /// Inverse hyperbolic tangent.
    Atanh,

    /// Floor.
    Floor,

    /// Ceiling.
    Ceil,

    /// Round halfway cases away from zero, as defined by Rust's f64::round.
    Round,

    /// Truncate toward zero.
    Trunc,

    /// Fractional component, as defined by Rust's f64::fract.
    Fract,
}

/// Binary elementary mathematical functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryElementaryFunction {
    /// Numeric minimum.
    Min,

    /// Numeric maximum.
    Max,

    /// Two-argument arctangent: atan2(y, x).
    Atan2,

    /// Euclidean hypotenuse: hypot(x, y).
    Hypot,

    /// Floating-point exponentiation: base^exponent.
    Pow,
}

/// Errors from elementary numeric evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementaryError {
    /// The canonical float implementation rejected an operation or result.
    Float(ClassicalFloatError),

    /// The operand is outside the mathematical domain.
    InvalidDomain {
        /// Function whose domain was violated.
        function: ElementaryFunction,
    },
}

impl fmt::Display for ElementaryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(error) => {
                write!(formatter, "elementary numeric operation failed: {error}")
            }

            Self::InvalidDomain { function } => {
                write!(
                    formatter,
                    "operand is outside the mathematical domain of {function:?}"
                )
            }
        }
    }
}

impl std::error::Error for ElementaryError {}

impl From<ClassicalFloatError> for ElementaryError {
    fn from(error: ClassicalFloatError) -> Self {
        Self::Float(error)
    }
}

/// Result type for elementary numeric operations.
pub type ElementaryResult<T> = Result<T, ElementaryError>;

/// Evaluates a unary elementary function on a canonical finite float.
///
/// Domain-specific errors are returned before evaluating functions with
/// explicit real-number domain restrictions. Non-finite results are rejected
/// and never become canonical ClassicalFloat values.
pub fn evaluate_unary(
    function: ElementaryFunction,
    value: ClassicalFloat,
) -> ElementaryResult<ClassicalFloat> {
    let raw = value.get();

    match function {
        // Delegate to canonical operations wherever the repository already
        // owns the operation and its error semantics.
        ElementaryFunction::Abs => Ok(value.abs()),

        ElementaryFunction::Signum => Ok(value.signum()),

        ElementaryFunction::Sqrt => {
            value.sqrt().map_err(ElementaryError::Float)
        }

        ElementaryFunction::Exp => {
            value.exp().map_err(ElementaryError::Float)
        }

        ElementaryFunction::Ln => {
            value.ln().map_err(ElementaryError::Float)
        }

        ElementaryFunction::Log10 => {
            value.log10().map_err(ElementaryError::Float)
        }

        ElementaryFunction::Sin => {
            value.sin().map_err(ElementaryError::Float)
        }

        ElementaryFunction::Cos => {
            value.cos().map_err(ElementaryError::Float)
        }

        ElementaryFunction::Tan => {
            value.tan().map_err(ElementaryError::Float)
        }

        ElementaryFunction::Asin => {
            if !(-1.0..=1.0).contains(&raw) {
                return Err(domain_error(function));
            }

            value.asin().map_err(ElementaryError::Float)
        }

        ElementaryFunction::Acos => {
            if !(-1.0..=1.0).contains(&raw) {
                return Err(domain_error(function));
            }

            value.acos().map_err(ElementaryError::Float)
        }

        ElementaryFunction::Atan => {
            value.atan().map_err(ElementaryError::Float)
        }

        ElementaryFunction::Floor => Ok(value.floor()),

        ElementaryFunction::Ceil => Ok(value.ceil()),

        ElementaryFunction::Round => Ok(value.round()),

        ElementaryFunction::Trunc => Ok(value.trunc()),

        ElementaryFunction::Fract => Ok(value.fract()),

        // These operations are not currently exposed as checked methods by
        // ClassicalFloat. Construct their results through the canonical
        // checked constructor so non-finite results cannot escape.
        ElementaryFunction::Cbrt => checked_result(raw.cbrt()),

        ElementaryFunction::Exp2 => checked_result(raw.exp2()),

        ElementaryFunction::Log2 => {
            if raw <= 0.0 {
                return Err(domain_error(function));
            }

            checked_result(raw.log2())
        }

        ElementaryFunction::Ln1p => {
            if raw <= -1.0 {
                return Err(domain_error(function));
            }

            checked_result(raw.ln_1p())
        }

        ElementaryFunction::Expm1 => {
            checked_result(raw.exp_m1())
        }

        ElementaryFunction::Sinh => {
            checked_result(raw.sinh())
        }

        ElementaryFunction::Cosh => {
            checked_result(raw.cosh())
        }

        ElementaryFunction::Tanh => {
            checked_result(raw.tanh())
        }

        ElementaryFunction::Asinh => {
            checked_result(raw.asinh())
        }

        ElementaryFunction::Acosh => {
            if raw < 1.0 {
                return Err(domain_error(function));
            }

            checked_result(raw.acosh())
        }

        ElementaryFunction::Atanh => {
            if !(-1.0..1.0).contains(&raw) {
                return Err(domain_error(function));
            }

            checked_result(raw.atanh())
        }
    }
}

/// Evaluates a binary elementary function on canonical finite floats.
///
/// Argument order for Atan2 is `lhs = y`, `rhs = x`, matching `atan2(y, x)`.
///
/// Min and max delegate to the canonical float implementation. Power delegates
/// to its checked power operation. Additional functions construct their
/// results through the canonical finite-value constructor.
pub fn evaluate_binary(
    function: BinaryElementaryFunction,
    lhs: ClassicalFloat,
    rhs: ClassicalFloat,
) -> ElementaryResult<ClassicalFloat> {
    let left = lhs.get();
    let right = rhs.get();

    match function {
        BinaryElementaryFunction::Min => Ok(lhs.min(rhs)),

        BinaryElementaryFunction::Max => Ok(lhs.max(rhs)),

        BinaryElementaryFunction::Pow => {
            lhs.checked_powf(rhs).map_err(ElementaryError::Float)
        }

        BinaryElementaryFunction::Atan2 => {
            checked_result(left.atan2(right))
        }

        BinaryElementaryFunction::Hypot => {
            checked_result(left.hypot(right))
        }
    }
}

/// Converts a raw calculation result to the canonical finite representation.
///
/// `ClassicalFloat::new` reports non-finite values as `NonFinite`. For an
/// operation result, this helper translates that condition to `NonFiniteResult`
/// so callers can distinguish invalid inputs from arithmetic failures.
fn checked_result(value: f64) -> ElementaryResult<ClassicalFloat> {
    match ClassicalFloat::new(value) {
        Ok(value) => Ok(value),

        Err(ClassicalFloatError::NonFinite) => {
            Err(ElementaryError::Float(
                ClassicalFloatError::NonFiniteResult,
            ))
        }

        Err(error) => Err(ElementaryError::Float(error)),
    }
}

fn domain_error(function: ElementaryFunction) -> ElementaryError {
    ElementaryError::InvalidDomain { function }
}

#[cfg(test)]
mod tests {
    use super::{
        evaluate_binary,
        evaluate_unary,
        BinaryElementaryFunction,
        ElementaryError,
        ElementaryFunction,
    };

    use crate::quantum::ir::classical::float::{
        ClassicalFloat,
        ClassicalFloatError,
    };

    fn float(value: f64) -> ClassicalFloat {
        ClassicalFloat::new(value).expect("test input must be finite")
    }

    fn result(
        function: ElementaryFunction,
        input: f64,
    ) -> f64 {
        evaluate_unary(function, float(input))
            .expect("expected finite result")
            .get()
    }

    #[test]
    fn square_root_is_checked() {
        assert_eq!(result(ElementaryFunction::Sqrt, 9.0), 3.0);

        assert_eq!(
            evaluate_unary(ElementaryFunction::Sqrt, float(-1.0)),
            Err(ElementaryError::Float(
                ClassicalFloatError::NegativeSquareRoot
            ))
        );
    }

    #[test]
    fn logarithm_domains_are_checked() {
        assert_eq!(
            evaluate_unary(ElementaryFunction::Ln, float(0.0)),
            Err(ElementaryError::Float(
                ClassicalFloatError::InvalidLogarithmDomain
            ))
        );

        assert_eq!(
            evaluate_unary(ElementaryFunction::Log10, float(-1.0)),
            Err(ElementaryError::Float(
                ClassicalFloatError::InvalidLog10Domain
            ))
        );

        assert_eq!(
            evaluate_unary(ElementaryFunction::Log2, float(0.0)),
            Err(ElementaryError::InvalidDomain {
                function: ElementaryFunction::Log2,
            })
        );
    }

    #[test]
    fn trigonometric_functions_use_radians() {
        let tolerance = 1.0e-12;

        assert!(
            (result(ElementaryFunction::Sin, 0.0)).abs() <= tolerance
        );

        assert!(
            (result(
                ElementaryFunction::Cos,
                core::f64::consts::PI
            ) + 1.0)
                .abs()
                <= tolerance
        );

        assert!(
            (result(
                ElementaryFunction::Atan,
                1.0
            ) - core::f64::consts::FRAC_PI_4)
                .abs()
                <= tolerance
        );
    }

    #[test]
    fn inverse_trigonometric_domains_are_checked() {
        assert_eq!(
            evaluate_unary(ElementaryFunction::Asin, float(2.0)),
            Err(ElementaryError::InvalidDomain {
                function: ElementaryFunction::Asin,
            })
        );

        assert_eq!(
            evaluate_unary(ElementaryFunction::Acos, float(-2.0)),
            Err(ElementaryError::InvalidDomain {
                function: ElementaryFunction::Acos,
            })
        );
    }

    #[test]
    fn hyperbolic_domains_are_checked() {
        assert_eq!(
            evaluate_unary(ElementaryFunction::Acosh, float(0.5)),
            Err(ElementaryError::InvalidDomain {
                function: ElementaryFunction::Acosh,
            })
        );

        assert_eq!(
            evaluate_unary(ElementaryFunction::Atanh, float(1.0)),
            Err(ElementaryError::InvalidDomain {
                function: ElementaryFunction::Atanh,
            })
        );

        assert_eq!(result(ElementaryFunction::Tanh, 0.0), 0.0);
    }

    #[test]
    fn exponential_overflow_is_rejected() {
        assert_eq!(
            evaluate_unary(ElementaryFunction::Exp, float(1000.0)),
            Err(ElementaryError::Float(
                ClassicalFloatError::NonFiniteResult
            ))
        );
    }

    #[test]
    fn logarithm_one_plus_x_checks_its_domain() {
        assert_eq!(
            evaluate_unary(ElementaryFunction::Ln1p, float(-1.0)),
            Err(ElementaryError::InvalidDomain {
                function: ElementaryFunction::Ln1p,
            })
        );

        assert_eq!(result(ElementaryFunction::Ln1p, 0.0), 0.0);
    }

    #[test]
    fn rounding_operations_are_explicit() {
        assert_eq!(result(ElementaryFunction::Floor, 2.9), 2.0);
        assert_eq!(result(ElementaryFunction::Ceil, 2.1), 3.0);
        assert_eq!(result(ElementaryFunction::Trunc, -2.9), -2.0);
        assert_eq!(result(ElementaryFunction::Round, -2.5), -3.0);
        assert_eq!(result(ElementaryFunction::Fract, 2.25), 0.25);
    }

    #[test]
    fn binary_functions_are_checked() {
        assert_eq!(
            evaluate_binary(
                BinaryElementaryFunction::Min,
                float(3.0),
                float(2.0),
            )
            .expect("finite result")
            .get(),
            2.0
        );

        assert_eq!(
            evaluate_binary(
                BinaryElementaryFunction::Max,
                float(3.0),
                float(2.0),
            )
            .expect("finite result")
            .get(),
            3.0
        );

        assert_eq!(
            evaluate_binary(
                BinaryElementaryFunction::Atan2,
                float(1.0),
                float(1.0),
            )
            .expect("finite result")
            .get(),
            core::f64::consts::FRAC_PI_4
        );

        assert_eq!(
            evaluate_binary(
                BinaryElementaryFunction::Hypot,
                float(3.0),
                float(4.0),
            )
            .expect("finite result")
            .get(),
            5.0
        );

        assert_eq!(
            evaluate_binary(
                BinaryElementaryFunction::Pow,
                float(2.0),
                float(10.0),
            )
            .expect("finite result")
            .get(),
            1024.0
        );
    }

    #[test]
    fn overflowing_binary_operation_is_rejected() {
        assert_eq!(
            evaluate_binary(
                BinaryElementaryFunction::Hypot,
                float(f64::MAX),
                float(f64::MAX),
            ),
            Err(ElementaryError::Float(
                ClassicalFloatError::NonFiniteResult
            ))
        );
    }

    #[test]
    fn signed_zero_is_preserved_by_canonical_operations() {
        let negative_zero = ClassicalFloat::negative_zero();

        let absolute = evaluate_unary(
            ElementaryFunction::Abs,
            negative_zero,
        )
        .expect("absolute value");

        assert!(absolute.is_positive_zero());

        let signum = evaluate_unary(
            ElementaryFunction::Signum,
            negative_zero,
        )
        .expect("signum");

        assert!(signum.is_negative_zero());
    }

    #[test]
    fn elementary_results_remain_canonical() {
        let functions = [
            ElementaryFunction::Sqrt,
            ElementaryFunction::Cbrt,
            ElementaryFunction::Exp,
            ElementaryFunction::Exp2,
            ElementaryFunction::Ln,
            ElementaryFunction::Log2,
            ElementaryFunction::Log10,
            ElementaryFunction::Sin,
            ElementaryFunction::Cos,
            ElementaryFunction::Tan,
            ElementaryFunction::Sinh,
            ElementaryFunction::Cosh,
            ElementaryFunction::Tanh,
        ];

        for function in functions {
            let output = evaluate_unary(function, float(0.5))
                .expect("operation should return a finite result");

            assert!(output.validate().is_ok());
        }
    }
}
```
