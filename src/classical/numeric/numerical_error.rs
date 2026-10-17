
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical numerical error contract.
//!
//! File: src/classical/numeric/numerical_error.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Provide a common, typed error boundary for concrete classical numerical
//! operations. Preserve the original error when wrapping failures from the
//! canonical integer, floating-point, arithmetic, and elementary-function
//! implementations.
//!
//! # Authority and ownership
//!
//! Canonical numeric representations and their errors:
//!   crate::quantum::ir::classical::integer
//!   crate::quantum::ir::classical::float
//!
//! Numerical operations:
//!   crate::classical::numeric::arithmetic
//!   crate::classical::numeric::elementary
//!   crate::classical::numeric::statistics
//!   crate::classical::numeric::reproducibility
//!
//! Normative semantic specification:
//!   grammar/spec/classical.md
//!
//! Syntax authorities:
//!   grammar/classical/numeric.g4
//!   grammar/classical/numerical.g4
//!   grammar/expressions/arithmetic.g4
//!   grammar/expressions/expressions.g4
//!
//! This module owns shared numerical error classification, stable diagnostic
//! codes, contextual numerical errors, and error-source preservation.
//!
//! It does not own numeric representations, arithmetic semantics, implicit
//! conversions, ASTs, IR, grammar, runtime scheduling, or hardware limits.
//!
//! # Error contract
//!
//! - Invalid mathematical domains are distinguished from arithmetic overflow.
//! - Division by zero is distinguishable from other numeric failures.
//! - Resource exhaustion is distinguishable from invalid program semantics.
//! - Wrapped errors retain their original source error.
//! - Stable codes are independent of human-readable error wording.
//! - Error messages must not be used as machine-readable identifiers.
//! - No error variant encodes a universal machine or data-size ceiling.
//! - No error silently converts a failed operation into a successful result.
//!
//! # POCO-REAF
//!
//! Numerical operations may fail because actual resources are exhausted or
//! because the selected numeric representation cannot express a result.
//! Such failures do not make the source program intrinsically invalid on every
//! possible target.
//!
//! The compiler/runtime must distinguish:
//!   - language-semantic invalidity;
//!   - numeric representation limitations;
//!   - target capability limitations;
//!   - operational resource exhaustion.
//!
//! # Integration contract
//!
//! 1. Declare `pub mod numerical_error;` in
//!    `src/classical/numeric/mod.rs`.
//! 2. Keep `ArithmeticError` and `ElementaryError` as their operation-specific
//!    errors. Convert them into this common error only at shared numerical API
//!    boundaries that need a unified result.
//! 3. Statistics and reproducibility code may use `NumericalError` for shared
//!    classifications while preserving lower-level source errors.
//! 4. Semantic analysis maps numerical failures to the repository's diagnostic
//!    infrastructure without parsing Display strings.
//! 5. Runtime and execution planning classify resource failures separately
//!    from deterministic semantic errors.
//! 6. The frontend grammar remains unchanged. Parsing must not create or
//!    interpret runtime numerical errors.
//! 7. Do not introduce another canonical integer or floating-point type.
//!
//! # Freeze rule
//!
//! Adding or changing stable diagnostic codes requires compatibility review.
//! Changing numeric semantics belongs to the canonical numeric owner and
//! normative specification, not to this error module.

use core::fmt;
use std::error::Error;

use crate::quantum::ir::classical::float::ClassicalFloatError;
use crate::quantum::ir::classical::integer::IntegerError;

/// Stable classification of a numerical failure.
///
/// These codes are intended for diagnostics, structured logging, testing,
/// telemetry, and tooling. Their string values are part of this module's
/// compatibility contract; do not derive them from variant names or messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NumericalErrorCode {
    /// Input is outside the mathematical domain of an operation.
    InvalidDomain,

    /// A divisor or denominator is zero where zero is forbidden.
    DivisionByZero,

    /// A result or required intermediate cannot be represented.
    Overflow,

    /// A nonzero mathematical result became unrepresentable as zero.
    Underflow,

    /// An input or result violates the selected finite-value contract.
    NonFinite,

    /// A numeric conversion cannot preserve the required value.
    ConversionFailure,

    /// An operation requires more observations than were supplied.
    InsufficientData,

    /// Input dimensions or lengths are incompatible.
    DimensionMismatch,

    /// A required mathematical result is undefined.
    UndefinedResult,

    /// An iterative method did not meet its convergence contract.
    ConvergenceFailure,

    /// An algorithm cannot meet the requested numerical guarantee.
    GuaranteeUnavailable,

    /// The selected numeric operation or representation is unsupported.
    UnsupportedOperation,

    /// A numerical algorithm encountered a singular system.
    SingularSystem,

    /// A computation failed because an operational resource was unavailable.
    ResourceExhausted,

    /// A lower-level canonical numeric operation failed.
    NumericBackendFailure,

    /// An operation failed for a reason not covered by a more specific code.
    Other,
}

impl NumericalErrorCode {
    /// Returns the stable machine-readable code.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidDomain => "ZAM-NUM-001",
            Self::DivisionByZero => "ZAM-NUM-002",
            Self::Overflow => "ZAM-NUM-003",
            Self::Underflow => "ZAM-NUM-004",
            Self::NonFinite => "ZAM-NUM-005",
            Self::ConversionFailure => "ZAM-NUM-006",
            Self::InsufficientData => "ZAM-NUM-007",
            Self::DimensionMismatch => "ZAM-NUM-008",
            Self::UndefinedResult => "ZAM-NUM-009",
            Self::ConvergenceFailure => "ZAM-NUM-010",
            Self::GuaranteeUnavailable => "ZAM-NUM-011",
            Self::UnsupportedOperation => "ZAM-NUM-012",
            Self::SingularSystem => "ZAM-NUM-013",
            Self::ResourceExhausted => "ZAM-NUM-014",
            Self::NumericBackendFailure => "ZAM-NUM-015",
            Self::Other => "ZAM-NUM-999",
        }
    }
}

impl fmt::Display for NumericalErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Error from a concrete classical numerical operation.
///
/// Operation-specific modules should retain their own more precise error types
/// where useful. This enum is the common boundary for APIs that combine
/// multiple numerical domains or need stable error classification.
///
/// `Arithmetic` and `Elementary` wrap their original errors instead of
/// translating them into message strings. This preserves diagnostic detail
/// and the standard error-source chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumericalError {
    /// The operand does not belong to the operation's mathematical domain.
    InvalidDomain {
        /// Stable name of the operation, not source text.
        operation: &'static str,
    },

    /// The operation requires a nonzero divisor or denominator.
    DivisionByZero {
        /// Stable name of the operation, not source text.
        operation: &'static str,
    },

    /// A result or intermediate is not representable by the selected numeric
    /// representation.
    Overflow {
        /// Stable name of the operation, not source text.
        operation: &'static str,
    },

    /// A nonzero result is too small to be represented under the selected
    /// numeric representation and rounding behavior.
    Underflow {
        /// Stable name of the operation, not source text.
        operation: &'static str,
    },

    /// An input or result violates a required finite-value contract.
    NonFinite {
        /// Stable name of the operation, not source text.
        operation: &'static str,
    },

    /// A conversion cannot satisfy the selected conversion contract.
    ConversionFailure {
        /// Stable name of the conversion, not source text.
        conversion: &'static str,
    },

    /// The algorithm requires more observations than it received.
    InsufficientData {
        /// Stable name of the operation, not source text.
        operation: &'static str,
        /// Number of observations supplied.
        actual: usize,
        /// Minimum number of observations required by this operation.
        required: usize,
    },

    /// Input dimensions or lengths are incompatible.
    DimensionMismatch {
        /// Stable name of the operation, not source text.
        operation: &'static str,
        /// Expected length or dimension.
        expected: usize,
        /// Actual length or dimension.
        actual: usize,
    },

    /// The requested mathematical result is undefined for these inputs.
    UndefinedResult {
        /// Stable name of the operation, not source text.
        operation: &'static str,
    },

    /// An iterative numerical algorithm failed to converge within its
    /// explicitly supplied stopping contract.
    ConvergenceFailure {
        /// Stable name of the algorithm, not source text.
        algorithm: &'static str,
    },

    /// The selected implementation cannot provide the requested numerical
    /// guarantee under the current declared execution contract.
    GuaranteeUnavailable {
        /// Stable name of the requested guarantee.
        guarantee: &'static str,
    },

    /// The requested concrete numerical operation is unsupported.
    UnsupportedOperation {
        /// Stable name of the operation, not source text.
        operation: &'static str,
        /// Stable name of the numeric domain, not source text.
        domain: &'static str,
    },

    /// A mathematical system is singular under the algorithm's defined
    /// singularity test.
    SingularSystem {
        /// Stable name of the algorithm, not source text.
        algorithm: &'static str,
    },

    /// An actual operational resource was unavailable.
    ///
    /// `resource` identifies the kind of resource, not a fixed capacity.
    /// This variant must not be used to represent a universal language limit.
    ResourceExhausted {
        /// Stable resource category, such as "memory" or "work budget".
        resource: &'static str,
    },

    /// Failure from the canonical arbitrary-width integer implementation.
    Integer(IntegerError),

    /// Failure from the canonical floating-point implementation.
    Float(ClassicalFloatError),

    /// Failure from the classical arithmetic API.
    Arithmetic(crate::classical::numeric::arithmetic::ArithmeticError),

    /// Failure from the classical elementary-function API.
    Elementary(crate::classical::numeric::elementary::ElementaryError),

    /// Other numerical failure with a stable operation identifier.
    Other {
        /// Stable name of the operation, not source text.
        operation: &'static str,
    },
}

impl NumericalError {
    /// Returns the stable machine-readable classification code.
    pub const fn code(&self) -> NumericalErrorCode {
        match self {
            Self::InvalidDomain { .. } => NumericalErrorCode::InvalidDomain,
            Self::DivisionByZero { .. } => NumericalErrorCode::DivisionByZero,
            Self::Overflow { .. } => NumericalErrorCode::Overflow,
            Self::Underflow { .. } => NumericalErrorCode::Underflow,
            Self::NonFinite { .. } => NumericalErrorCode::NonFinite,
            Self::ConversionFailure { .. } => {
                NumericalErrorCode::ConversionFailure
            }
            Self::InsufficientData { .. } => {
                NumericalErrorCode::InsufficientData
            }
            Self::DimensionMismatch { .. } => {
                NumericalErrorCode::DimensionMismatch
            }
            Self::UndefinedResult { .. } => {
                NumericalErrorCode::UndefinedResult
            }
            Self::ConvergenceFailure { .. } => {
                NumericalErrorCode::ConvergenceFailure
            }
            Self::GuaranteeUnavailable { .. } => {
                NumericalErrorCode::GuaranteeUnavailable
            }
            Self::UnsupportedOperation { .. } => {
                NumericalErrorCode::UnsupportedOperation
            }
            Self::SingularSystem { .. } => NumericalErrorCode::SingularSystem,
            Self::ResourceExhausted { .. } => {
                NumericalErrorCode::ResourceExhausted
            }
            Self::Integer(_) | Self::Float(_) | Self::Arithmetic(_) |
            Self::Elementary(_) => NumericalErrorCode::NumericBackendFailure,
            Self::Other { .. } => NumericalErrorCode::Other,
        }
    }

    /// Returns the stable code string suitable for structured diagnostics.
    pub const fn code_str(&self) -> &'static str {
        self.code().as_str()
    }

    /// Returns the stable operation identifier when the error has one.
    ///
    /// This is metadata for tooling. It is not source code and must not be
    /// treated as an expression to execute.
    pub const fn operation(&self) -> Option<&'static str> {
        match self {
            Self::InvalidDomain { operation }
            | Self::DivisionByZero { operation }
            | Self::Overflow { operation }
            | Self::Underflow { operation }
            | Self::NonFinite { operation }
            | Self::UndefinedResult { operation }
            | Self::Other { operation }
            | Self::InsufficientData { operation, .. }
            | Self::DimensionMismatch { operation, .. }
            | Self::UnsupportedOperation { operation, .. } => Some(operation),

            Self::ConversionFailure { conversion } => Some(conversion),
            Self::ConvergenceFailure { algorithm }
            | Self::SingularSystem { algorithm } => Some(algorithm),

            Self::GuaranteeUnavailable { guarantee } => Some(guarantee),

            Self::ResourceExhausted { .. }
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Arithmetic(_)
            | Self::Elementary(_) => None,
        }
    }

    /// Returns whether this error describes operational resource exhaustion
    /// rather than an intrinsically invalid numerical operation.
    pub const fn is_resource_exhaustion(&self) -> bool {
        matches!(self, Self::ResourceExhausted { .. })
    }
}

impl fmt::Display for NumericalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDomain { operation } => {
                write!(formatter, "{operation}: operand outside mathematical domain")
            }
            Self::DivisionByZero { operation } => {
                write!(formatter, "{operation}: division by zero")
            }
            Self::Overflow { operation } => {
                write!(formatter, "{operation}: numeric overflow")
            }
            Self::Underflow { operation } => {
                write!(formatter, "{operation}: numeric underflow")
            }
            Self::NonFinite { operation } => {
                write!(formatter, "{operation}: non-finite input or result")
            }
            Self::ConversionFailure { conversion } => {
                write!(formatter, "{conversion}: numeric conversion failed")
            }
            Self::InsufficientData {
                operation,
                actual,
                required,
            } => write!(
                formatter,
                "{operation}: insufficient data; received {actual}, requires {required}"
            ),
            Self::DimensionMismatch {
                operation,
                expected,
                actual,
            } => write!(
                formatter,
                "{operation}: dimension mismatch; expected {expected}, received {actual}"
            ),
            Self::UndefinedResult { operation } => {
                write!(formatter, "{operation}: result is undefined")
            }
            Self::ConvergenceFailure { algorithm } => {
                write!(formatter, "{algorithm}: convergence contract not met")
            }
            Self::GuaranteeUnavailable { guarantee } => {
                write!(formatter, "numerical guarantee unavailable: {guarantee}")
            }
            Self::UnsupportedOperation { operation, domain } => {
                write!(formatter, "{operation}: unsupported for {domain}")
            }
            Self::SingularSystem { algorithm } => {
                write!(formatter, "{algorithm}: singular system")
            }
            Self::ResourceExhausted { resource } => {
                write!(formatter, "numerical operation could not obtain resource: {resource}")
            }
            Self::Integer(error) => {
                write!(formatter, "canonical integer operation failed: {error}")
            }
            Self::Float(error) => {
                write!(formatter, "canonical floating-point operation failed: {error}")
            }
            Self::Arithmetic(error) => {
                write!(formatter, "classical arithmetic failed: {error}")
            }
            Self::Elementary(error) => {
                write!(formatter, "elementary numerical operation failed: {error}")
            }
            Self::Other { operation } => {
                write!(formatter, "{operation}: numerical operation failed")
            }
        }
    }
}

impl Error for NumericalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Integer(error) => Some(error),
            Self::Float(error) => Some(error),
            Self::Arithmetic(error) => Some(error),
            Self::Elementary(error) => Some(error),
            _ => None,
        }
    }
}

impl From<IntegerError> for NumericalError {
    fn from(error: IntegerError) -> Self {
        Self::Integer(error)
    }
}

impl From<ClassicalFloatError> for NumericalError {
    fn from(error: ClassicalFloatError) -> Self {
        Self::Float(error)
    }
}

impl From<crate::classical::numeric::arithmetic::ArithmeticError>
    for NumericalError
{
    fn from(
        error: crate::classical::numeric::arithmetic::ArithmeticError,
    ) -> Self {
        Self::Arithmetic(error)
    }
}

impl From<crate::classical::numeric::elementary::ElementaryError>
    for NumericalError
{
    fn from(
        error: crate::classical::numeric::elementary::ElementaryError,
    ) -> Self {
        Self::Elementary(error)
    }
}

/// Standard result alias for APIs using the common numerical error contract.
pub type NumericalResult<T> = Result<T, NumericalError>;

#[cfg(test)]
mod tests {
    use super::{
        NumericalError,
        NumericalErrorCode,
    };

    #[test]
    fn error_codes_are_stable_and_unique() {
        let codes = [
            NumericalErrorCode::InvalidDomain.as_str(),
            NumericalErrorCode::DivisionByZero.as_str(),
            NumericalErrorCode::Overflow.as_str(),
            NumericalErrorCode::Underflow.as_str(),
            NumericalErrorCode::NonFinite.as_str(),
            NumericalErrorCode::ConversionFailure.as_str(),
            NumericalErrorCode::InsufficientData.as_str(),
            NumericalErrorCode::DimensionMismatch.as_str(),
            NumericalErrorCode::UndefinedResult.as_str(),
            NumericalErrorCode::ConvergenceFailure.as_str(),
            NumericalErrorCode::GuaranteeUnavailable.as_str(),
            NumericalErrorCode::UnsupportedOperation.as_str(),
            NumericalErrorCode::SingularSystem.as_str(),
            NumericalErrorCode::ResourceExhausted.as_str(),
            NumericalErrorCode::NumericBackendFailure.as_str(),
            NumericalErrorCode::Other.as_str(),
        ];

        for (index, code) in codes.iter().enumerate() {
            assert!(
                !codes[..index].contains(code),
                "duplicate numerical error code: {code}"
            );
        }
    }

    #[test]
    fn error_code_does_not_depend_on_display_text() {
        let error = NumericalError::DivisionByZero {
            operation: "divide",
        };

        assert_eq!(error.code_str(), "ZAM-NUM-002");
        assert_eq!(error.operation(), Some("divide"));
        assert_eq!(error.to_string(), "divide: division by zero");
    }

    #[test]
    fn insufficient_data_preserves_counts() {
        let error = NumericalError::InsufficientData {
            operation: "sample_variance",
            actual: 1,
            required: 2,
        };

        assert_eq!(error.code(), NumericalErrorCode::InsufficientData);
        assert!(error.to_string().contains("received 1"));
        assert!(error.to_string().contains("requires 2"));
    }

    #[test]
    fn resource_exhaustion_is_not_a_semantic_size_limit() {
        let error = NumericalError::ResourceExhausted {
            resource: "memory",
        };

        assert!(error.is_resource_exhaustion());
        assert_eq!(error.code(), NumericalErrorCode::ResourceExhausted);
    }

    #[test]
    fn unsupported_guarantee_has_its_own_classification() {
        let error = NumericalError::GuaranteeUnavailable {
            guarantee: "cross-target bitwise identity",
        };

        assert_eq!(
            error.code(),
            NumericalErrorCode::GuaranteeUnavailable
        );
        assert_eq!(
            error.operation(),
            Some("cross-target bitwise identity")
        );
    }
}
