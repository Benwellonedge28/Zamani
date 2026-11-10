
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Foundational Primitives
//!
//! Path: `src/stdlib/foundation/primitives.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides reusable, target-independent Rust-side primitive contracts
//! for the Zamani standard library.
//!
//! This module supplies:
//!
//! - Extensible primitive metadata.
//! - Primitive category metadata.
//! - Checked integer arithmetic contracts.
//! - Implementations for Rust's built-in integer types.
//! - Floating-point classification contracts.
//! - Implementations for Rust's built-in floating-point types.
//!
//! It is a foundational implementation utility, not a source-language
//! type system.
//!
//! ## Architectural ownership
//!
//! This module owns:
//!
//! - The `Primitive` metadata trait.
//! - The `PrimitiveInteger` checked-arithmetic trait.
//! - The `PrimitiveFloat` classification trait.
//! - Implementations of these contracts for supported Rust primitives.
//!
//! This module does NOT own:
//!
//! - Zamani source-level primitive type definitions.
//! - Semantic type resolution.
//! - Type inference or type checking.
//! - Numeric conversion policy.
//! - Arbitrary-precision arithmetic.
//! - Tensor or array storage.
//! - Quantum numeric representations.
//! - HDL signal or register representations.
//! - Hardware ABI or register widths.
//! - Device allocation or execution.
//! - Compiler optimization.
//! - Backend code generation.
//!
//! ## Existing repository authorities
//!
//! - `src/frontend/ast/node/types/primitive.rs`:
//!   Source-level primitive type representation.
//!
//! - `src/frontend/semantic/types/ty.rs`:
//!   Canonical semantic type representation.
//!
//! - `grammar/types/primitive.g4`:
//!   Source-level primitive type grammar.
//!
//! - `src/classical/collections/array.rs`:
//!   Canonical classical array storage.
//!
//! - `src/classical/collections/tensor.rs`:
//!   Tensor abstraction over canonical array storage.
//!
//! - `src/stdlib/numeric.rs`:
//!   Existing numerical API, which requires a separate correctness
//!   and migration audit.
//!
//! This module must not become a competing authority for any of them.
//!
//! ## Primitive metadata
//!
//! `Primitive::NAME` identifies the Rust-side primitive implementation.
//! It is not a Zamani source-language spelling or target ABI identifier.
//!
//! `Primitive::CATEGORY` is an extensible descriptive label.
//! It is deliberately a string rather than a closed enum so that
//! downstream implementations can introduce additional categories.
//!
//! The boolean metadata constants describe the implementation type.
//! They do not determine whether a source-language operation is legal.
//!
//! ## Scalability
//!
//! This module imposes no universal maximum on:
//!
//! - Integer width.
//! - Numeric precision.
//! - Tensor rank.
//! - Array dimensions.
//! - Device count.
//! - Processor count.
//! - Distributed participants.
//! - Computational domains.
//!
//! Built-in Rust integer types retain their actual Rust representation
//! and checked arithmetic behavior.
//!
//! Future scalar implementations may implement `Primitive` and,
//! where appropriate, the numeric extension traits.
//!
//! Arbitrary-precision and symbolic numeric implementations belong
//! in their respective numeric authorities and may implement these
//! open traits without modifying this module.
//!
//! Physical computation remains bounded by available resources.
//!
//! ## Error and overflow behavior
//!
//! Checked integer operations return `None` when the operation cannot
//! be represented by the concrete implementation type.
//!
//! They never silently wrap, saturate, or change the mathematical result.
//!
//! Callers must explicitly select another arithmetic policy if their
//! operation requires wrapping or saturation.
//!
//! Division and remainder by zero return `None`.
//!
//! Signed division overflow, such as the minimum signed value divided
//! by negative one, also returns `None`.
//!
//! ## Portability
//!
//! The traits describe concrete Rust implementation types.
//!
//! They do not promise that a Rust primitive has the same representation
//! on every target.
//!
//! Source-language types such as `usize` and `isize` remain semantic
//! types whose target representation is selected downstream.
//!
//! ## Integration
//!
//! `foundation/mod.rs` exports this module.
//!
//! `foundation/numeric.rs` may consume the numeric traits.
//!
//! `foundation/conversion.rs` may implement explicit conversion policies.
//!
//! `foundation/math.rs` may use the numeric traits for generic operations.
//!
//! Existing `core.rs` and `collections.rs` must migrate through
//! compatibility facades rather than defining competing primitive types.
//!
//! Semantic and AST adapters must perform explicit conversions between
//! source-language types and implementation types.
//!
//! ## Freeze criteria
//!
//! This file is ready to freeze when:
//!
//! - The foundation module exports it.
//! - Primitive metadata is tested.
//! - Every built-in integer implementation is tested.
//! - Every built-in floating-point implementation is tested.
//! - Overflow and division edge cases are tested.
//! - External custom primitive implementations compile.
//! - No source-language type authority is duplicated.
//! - Formatting, compilation, tests and Clippy pass.

// =============================================================================
// Primitive metadata
// =============================================================================

/// Descriptive metadata for a concrete primitive implementation.
///
/// This trait is intentionally open: external crates and future Zamani
/// subsystems may implement it for their own primitive representations.
///
/// The metadata describes the Rust-side implementation type. It does not
/// establish source-language type meaning, ABI layout, hardware support,
/// or semantic compatibility.
///
/// # Extensibility
///
/// `CATEGORY` is a descriptive string rather than a closed enumeration.
/// Implementors may introduce additional category names.
///
/// The built-in category labels used by this module are:
///
/// - `"unit"`
/// - `"boolean"`
/// - `"character"`
/// - `"signed-integer"`
/// - `"unsigned-integer"`
/// - `"floating-point"`
///
/// These labels are conventions, not an exhaustive list.
pub trait Primitive {
    /// Stable descriptive name of the implementation type.
    ///
    /// This is not necessarily the spelling used in Zamani source code.
    const NAME: &'static str;

    /// Extensible descriptive category of the implementation type.
    const CATEGORY: &'static str;

    /// Whether this implementation represents numeric data.
    const IS_NUMERIC: bool = false;

    /// Whether this implementation represents integer data.
    const IS_INTEGER: bool = false;

    /// Whether this implementation represents signed integer data.
    const IS_SIGNED: bool = false;

    /// Whether this implementation represents unsigned integer data.
    const IS_UNSIGNED: bool = false;

    /// Whether this implementation represents floating-point data.
    const IS_FLOAT: bool = false;
}

// =============================================================================
// Built-in non-numeric primitives
// =============================================================================

impl Primitive for () {
    const NAME: &'static str = "()";
    const CATEGORY: &'static str = "unit";
}

impl Primitive for bool {
    const NAME: &'static str = "bool";
    const CATEGORY: &'static str = "boolean";
}

impl Primitive for char {
    const NAME: &'static str = "char";
    const CATEGORY: &'static str = "character";
}

// =============================================================================
// Checked integer arithmetic
// =============================================================================

/// Checked arithmetic operations for a concrete integer implementation.
///
/// Implementations must return `None` when the mathematical result cannot
/// be represented by the implementation type.
///
/// They must not silently wrap, saturate, or truncate.
///
/// This trait describes concrete implementation arithmetic. It does not
/// define Zamani's source-language overflow policy.
pub trait PrimitiveInteger:
    Primitive
    + Copy
    + PartialEq
    + PartialOrd
    + Send
    + Sync
    + 'static
{
    /// Returns the checked sum.
    ///
    /// Returns `None` if the result overflows the implementation type.
    fn checked_add(self, rhs: Self) -> Option<Self>;

    /// Returns the checked difference.
    ///
    /// Returns `None` if the result underflows or overflows the
    /// implementation type.
    fn checked_sub(self, rhs: Self) -> Option<Self>;

    /// Returns the checked product.
    ///
    /// Returns `None` if the result overflows the implementation type.
    fn checked_mul(self, rhs: Self) -> Option<Self>;

    /// Returns the checked quotient.
    ///
    /// Returns `None` when:
    ///
    /// - The divisor is zero.
    /// - The quotient cannot be represented by the implementation type.
    fn checked_div(self, rhs: Self) -> Option<Self>;

    /// Returns the checked remainder.
    ///
    /// Returns `None` when:
    ///
    /// - The divisor is zero.
    /// - The remainder operation cannot be represented by the type.
    fn checked_rem(self, rhs: Self) -> Option<Self>;
}

// =============================================================================
// Built-in integer implementations
// =============================================================================

macro_rules! impl_signed_integer {
    ($($type:ty),+ $(,)?) => {
        $(
            impl Primitive for $type {
                const NAME: &'static str = stringify!($type);
                const CATEGORY: &'static str = "signed-integer";
                const IS_NUMERIC: bool = true;
                const IS_INTEGER: bool = true;
                const IS_SIGNED: bool = true;
            }

            impl PrimitiveInteger for $type {
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    <$type>::checked_add(self, rhs)
                }

                fn checked_sub(self, rhs: Self) -> Option<Self> {
                    <$type>::checked_sub(self, rhs)
                }

                fn checked_mul(self, rhs: Self) -> Option<Self> {
                    <$type>::checked_mul(self, rhs)
                }

                fn checked_div(self, rhs: Self) -> Option<Self> {
                    <$type>::checked_div(self, rhs)
                }

                fn checked_rem(self, rhs: Self) -> Option<Self> {
                    <$type>::checked_rem(self, rhs)
                }
            }
        )+
    };
}

macro_rules! impl_unsigned_integer {
    ($($type:ty),+ $(,)?) => {
        $(
            impl Primitive for $type {
                const NAME: &'static str = stringify!($type);
                const CATEGORY: &'static str = "unsigned-integer";
                const IS_NUMERIC: bool = true;
                const IS_INTEGER: bool = true;
                const IS_UNSIGNED: bool = true;
            }

            impl PrimitiveInteger for $type {
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    <$type>::checked_add(self, rhs)
                }

                fn checked_sub(self, rhs: Self) -> Option<Self> {
                    <$type>::checked_sub(self, rhs)
                }

                fn checked_mul(self, rhs: Self) -> Option<Self> {
                    <$type>::checked_mul(self, rhs)
                }

                fn checked_div(self, rhs: Self) -> Option<Self> {
                    <$type>::checked_div(self, rhs)
                }

                fn checked_rem(self, rhs: Self) -> Option<Self> {
                    <$type>::checked_rem(self, rhs)
                }
            }
        )+
    };
}

impl_signed_integer!(
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
);

impl_unsigned_integer!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
);

// =============================================================================
// Floating-point classification
// =============================================================================

/// Floating-point classification operations.
///
/// This trait provides basic classification without defining a numerical
/// algorithm, precision policy, approximation policy, or target ABI.
///
/// Implementations must preserve the classification semantics of their
/// underlying floating-point representation.
pub trait PrimitiveFloat:
    Primitive
    + Copy
    + PartialEq
    + PartialOrd
    + Send
    + Sync
    + 'static
{
    /// Returns whether the value is finite.
    fn is_finite(self) -> bool;

    /// Returns whether the value is infinite.
    fn is_infinite(self) -> bool;

    /// Returns whether the value is NaN.
    fn is_nan(self) -> bool;
}

// =============================================================================
// Built-in floating-point implementations
// =============================================================================

macro_rules! impl_float {
    ($($type:ty),+ $(,)?) => {
        $(
            impl Primitive for $type {
                const NAME: &'static str = stringify!($type);
                const CATEGORY: &'static str = "floating-point";
                const IS_NUMERIC: bool = true;
                const IS_FLOAT: bool = true;
            }

            impl PrimitiveFloat for $type {
                fn is_finite(self) -> bool {
                    <$type>::is_finite(self)
                }

                fn is_infinite(self) -> bool {
                    <$type>::is_infinite(self)
                }

                fn is_nan(self) -> bool {
                    <$type>::is_nan(self)
                }
            }
        )+
    };
}

impl_float!(
    f32,
    f64,
);

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::{
        Primitive,
        PrimitiveFloat,
        PrimitiveInteger,
    };

    // ---------------------------------------------------------------------
    // Primitive metadata
    // ---------------------------------------------------------------------

    #[test]
    fn unit_metadata_is_available() {
        assert_eq!(<()>::NAME, "()");
        assert_eq!(<()>::CATEGORY, "unit");
        assert!(!<()>::IS_NUMERIC);
    }

    #[test]
    fn boolean_metadata_is_available() {
        assert_eq!(bool::NAME, "bool");
        assert_eq!(bool::CATEGORY, "boolean");
        assert!(!bool::IS_NUMERIC);
    }

    #[test]
    fn character_metadata_is_available() {
        assert_eq!(char::NAME, "char");
        assert_eq!(char::CATEGORY, "character");
        assert!(!char::IS_NUMERIC);
    }

    #[test]
    fn signed_integer_metadata_is_available() {
        assert_eq!(i32::NAME, "i32");
        assert_eq!(i32::CATEGORY, "signed-integer");
        assert!(i32::IS_NUMERIC);
        assert!(i32::IS_INTEGER);
        assert!(i32::IS_SIGNED);
        assert!(!i32::IS_UNSIGNED);
        assert!(!i32::IS_FLOAT);
    }

    #[test]
    fn unsigned_integer_metadata_is_available() {
        assert_eq!(u64::NAME, "u64");
        assert_eq!(u64::CATEGORY, "unsigned-integer");
        assert!(u64::IS_NUMERIC);
        assert!(u64::IS_INTEGER);
        assert!(u64::IS_UNSIGNED);
        assert!(!u64::IS_SIGNED);
        assert!(!u64::IS_FLOAT);
    }

    #[test]
    fn floating_point_metadata_is_available() {
        assert_eq!(f64::NAME, "f64");
        assert_eq!(f64::CATEGORY, "floating-point");
        assert!(f64::IS_NUMERIC);
        assert!(f64::IS_FLOAT);
        assert!(!f64::IS_INTEGER);
    }

    // ---------------------------------------------------------------------
    // Checked integer arithmetic
    // ---------------------------------------------------------------------

    #[test]
    fn checked_integer_addition_returns_exact_result() {
        assert_eq!(10_i32.checked_add(20), Some(30));
        assert_eq!(10_u64.checked_add(20), Some(30));
    }

    #[test]
    fn checked_integer_subtraction_returns_exact_result() {
        assert_eq!(20_i32.checked_sub(10), Some(10));
        assert_eq!(20_u64.checked_sub(10), Some(10));
    }

    #[test]
    fn checked_integer_multiplication_returns_exact_result() {
        assert_eq!(7_i32.checked_mul(6), Some(42));
        assert_eq!(7_u64.checked_mul(6), Some(42));
    }

    #[test]
    fn checked_integer_division_returns_exact_result() {
        assert_eq!(42_i32.checked_div(6), Some(7));
        assert_eq!(42_u64.checked_div(6), Some(7));
    }

    #[test]
    fn checked_integer_remainder_returns_exact_result() {
        assert_eq!(43_i32.checked_rem(6), Some(1));
        assert_eq!(43_u64.checked_rem(6), Some(1));
    }

    #[test]
    fn checked_signed_addition_detects_overflow() {
        assert_eq!(i8::MAX.checked_add(1), None);
        assert_eq!(i16::MAX.checked_add(1), None);
        assert_eq!(i32::MAX.checked_add(1), None);
        assert_eq!(i64::MAX.checked_add(1), None);
        assert_eq!(i128::MAX.checked_add(1), None);
        assert_eq!(isize::MAX.checked_add(1), None);
    }

    #[test]
    fn checked_unsigned_addition_detects_overflow() {
        assert_eq!(u8::MAX.checked_add(1), None);
        assert_eq!(u16::MAX.checked_add(1), None);
        assert_eq!(u32::MAX.checked_add(1), None);
        assert_eq!(u64::MAX.checked_add(1), None);
        assert_eq!(u128::MAX.checked_add(1), None);
        assert_eq!(usize::MAX.checked_add(1), None);
    }

    #[test]
    fn checked_unsigned_subtraction_detects_underflow() {
        assert_eq!(0_u8.checked_sub(1), None);
        assert_eq!(0_u32.checked_sub(1), None);
        assert_eq!(0_usize.checked_sub(1), None);
    }

    #[test]
    fn checked_integer_multiplication_detects_overflow() {
        assert_eq!(i8::MAX.checked_mul(2), None);
        assert_eq!(u8::MAX.checked_mul(2), None);
    }

    #[test]
    fn checked_integer_division_rejects_zero() {
        assert_eq!(10_i32.checked_div(0), None);
        assert_eq!(10_u32.checked_div(0), None);
    }

    #[test]
    fn checked_integer_remainder_rejects_zero() {
        assert_eq!(10_i32.checked_rem(0), None);
        assert_eq!(10_u32.checked_rem(0), None);
    }

    #[test]
    fn checked_signed_division_detects_minimum_overflow() {
        assert_eq!(i8::MIN.checked_div(-1), None);
        assert_eq!(i16::MIN.checked_div(-1), None);
        assert_eq!(i32::MIN.checked_div(-1), None);
        assert_eq!(i64::MIN.checked_div(-1), None);
        assert_eq!(i128::MIN.checked_div(-1), None);
        assert_eq!(isize::MIN.checked_div(-1), None);
    }

    // ---------------------------------------------------------------------
    // Floating-point classification
    // ---------------------------------------------------------------------

    #[test]
    fn finite_floating_point_values_are_classified() {
        assert!(1.5_f32.is_finite());
        assert!(1.5_f64.is_finite());

        assert!(!1.5_f32.is_infinite());
        assert!(!1.5_f64.is_infinite());

        assert!(!1.5_f32.is_nan());
        assert!(!1.5_f64.is_nan());
    }

    #[test]
    fn infinite_floating_point_values_are_classified() {
        assert!(f32::INFINITY.is_infinite());
        assert!(f64::INFINITY.is_infinite());

        assert!(!f32::INFINITY.is_finite());
        assert!(!f64::INFINITY.is_finite());
    }

    #[test]
    fn nan_floating_point_values_are_classified() {
        assert!(f32::NAN.is_nan());
        assert!(f64::NAN.is_nan());

        assert!(!f32::NAN.is_finite());
        assert!(!f64::NAN.is_finite());
    }

    // ---------------------------------------------------------------------
    // External extensibility
    // ---------------------------------------------------------------------

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct ApplicationScalar(i64);

    impl Primitive for ApplicationScalar {
        const NAME: &'static str = "ApplicationScalar";
        const CATEGORY: &'static str = "application-defined-scalar";
        const IS_NUMERIC: bool = true;
        const IS_INTEGER: bool = true;
        const IS_SIGNED: bool = true;
    }

    #[test]
    fn external_primitive_metadata_can_be_added_without_editing_this_file() {
        assert_eq!(
            ApplicationScalar::NAME,
            "ApplicationScalar"
        );

        assert_eq!(
            ApplicationScalar::CATEGORY,
            "application-defined-scalar"
        );

        assert!(ApplicationScalar::IS_NUMERIC);
    }
}
