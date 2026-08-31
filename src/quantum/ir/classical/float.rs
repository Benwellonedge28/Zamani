//! Zamani Quantum IR — Classical Floating-Point Semantics
//!
//! Production-grade, hardware-independent classical floating-point
//! representation for the canonical Zamani Quantum Intermediate
//! Representation.
//!
//! # Architectural role
//!
//! This module owns the semantic representation and deterministic operations
//! for classical floating-point values.
//!
//! It answers:
//!
//! > What does a classical floating-point value mean?
//!
//! It does NOT decide:
//!
//! - where the value is stored;
//! - which CPU/GPU/accelerator evaluates it;
//! - which hardware register contains it;
//! - how a quantum device implements it;
//! - how a backend serializes it for a specific machine;
//! - how classical expressions are scheduled;
//! - how quantum operations are routed;
//! - how a frontend parses floating-point syntax;
//! - how a simulator stores runtime state.
//!
//! Those responsibilities belong to higher-level IR and downstream modules.
//!
//! # Canonical ownership
//!
//! The intended path is:
//!
//! ```text
//! src/quantum/ir/classical/float.rs
//! ```
//!
//! The parent classical module should expose this module through:
//!
//! ```text
//! quantum::ir::classical::float
//! ```
//!
//! The canonical classical value layer may then use this type:
//!
//! ```text
//! quantum::ir::classical::value
//! ```
//!
//! # Important ownership rule
//!
//! `float.rs` owns floating-point semantics.
//!
//! `classical/value.rs` owns the aggregate `ClassicalValue` representation.
//!
//! `types.rs` owns the canonical type vocabulary (`FloatType`).
//!
//! `parameter.rs` owns symbolic parameter expressions.
//!
//! `qubit.rs` owns logical and physical qubit identity.
//!
//! Therefore this file MUST NOT redefine:
//!
//! - `QubitId`;
//! - `PhysicalQubitId`;
//! - `ValueId`;
//! - `Parameter`;
//! - `FloatType`;
//! - `ClassicalValue`.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::types
//!          │
//!          │ semantic float format
//!          ▼
//! classical::float
//!          │
//!          ├────────► classical::value
//!          ├────────► classical::expression
//!          ├────────► classical::predicate
//!          ├────────► validation
//!          ├────────► serialization
//!          └────────► hashing
//! ```
//!
//! This module intentionally remains below those higher-level modules.
//!
//! # Universal-program principle
//!
//! A Zamani program must be expressible independently of the eventual
//! execution machine.
//!
//! Consequently this file contains:
//!
//! - no CPU-specific floating-point register assumptions;
//! - no GPU-specific assumptions;
//! - no FPGA-specific assumptions;
//! - no quantum-vendor assumptions;
//! - no fixed quantum-machine size;
//! - no fixed number of qubits;
//! - no fixed classical-memory size;
//! - no fixed array size;
//! - no hardware-specific precision ceiling.
//!
//! A finite `f64` value is a concrete semantic representation supported by
//! this Rust implementation. The existence of `f64` does NOT define the
//! maximum floating-point precision of the Zamani language or the complete
//! future IR.
//!
//! Future arbitrary-precision or target-specific floating-point formats can
//! be represented through explicit numeric dialects/types without changing
//! the meaning of this type.
//!
//! # Floating-point safety
//!
//! NaN and positive/negative infinity are rejected by checked constructors.
//!
//! This is deliberate because canonical semantic values need deterministic:
//!
//! - equality;
//! - ordering;
//! - hashing;
//! - serialization;
//! - validation.
//!
//! IEEE-754 NaN values do not provide ordinary mathematical equality and
//! therefore must not enter the canonical semantic representation through
//! the checked API.
//!
//! # Zero semantics
//!
//! IEEE-754 distinguishes:
//!
//! ```text
//! +0.0
//! -0.0
//! ```
//!
//! This module preserves that distinction at the bit-level representation.
//!
//! Therefore:
//!
//! ```text
//! +0.0 != -0.0
//! ```
//!
//! for structural/canonical equality.
//!
//! Numeric comparison still follows IEEE floating-point comparison semantics,
//! where both compare numerically equal to zero.
//!
//! # Determinism
//!
//! Floating-point identity is represented by IEEE-754 bits.
//!
//! This provides deterministic:
//!
//! - `Eq`;
//! - `Hash`;
//! - canonical byte representation;
//! - serialization input;
//! - content-addressable caching.
//!
//! # Arithmetic
//!
//! Arithmetic methods are provided as checked semantic operations.
//!
//! They reject:
//!
//! - non-finite inputs;
//! - non-finite results;
//! - division by zero;
//! - invalid square roots;
//! - invalid logarithms;
//! - overflow into infinity.
//!
//! Arithmetic does NOT silently convert invalid results into NaN or infinity.
//!
//! # Numerical policy
//!
//! This module deliberately does not claim that every mathematical operation
//! is exact. `f64` remains an IEEE-754 finite approximation.
//!
//! Consequently the module distinguishes:
//!
//! ```text
//! structural equality
//!     exact IEEE bit equality
//!
//! numeric comparison
//!     IEEE numeric ordering
//!
//! approximate equality
//!     explicitly supplied tolerance
//! ```
//!
//! Approximate equality is never used implicitly for `Eq` or `Hash`.
//!
//! # Serialization
//!
//! The canonical representation is the IEEE-754 `u64` bit pattern in
//! big-endian byte order.
//!
//! This avoids locale-dependent formatting and floating-point decimal
//! rendering differences.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `quantum::ir::types::FloatType`
//!     remains the owner of the semantic type/precision vocabulary.
//!
//! `quantum::ir::classical::value`
//!     may embed `ClassicalFloat` as its canonical float value.
//!
//! `quantum::ir::parameter`
//!     remains the owner of symbolic parameter expressions. This module does
//!     not create a second parameter AST.
//!
//! `quantum::ir::classical::expression`
//!     may use `ClassicalFloat` for concrete floating-point literals and
//!     arithmetic results.
//!
//! `quantum::ir::classical::predicate`
//!     may compare `ClassicalFloat` values using explicit comparison
//!     semantics.
//!
//! `quantum::ir::validation`
//!     may call `validate()` and inspect the representation.
//!
//! `quantum::ir::serialization`
//!     should serialize `bits()` rather than `Display` output.
//!
//! `quantum::ir::hash`
//!     should hash `canonical_bytes()` or `bits()`.
//!
//! `quantum::ir::qubit`
//!     remains the canonical owner of qubit identity. This module does not
//!     import or redefine qubit identity because floating-point semantics do
//!     not require it.
//!
//! # Completion contract
//!
//! Once this file is implemented, downstream modules must not need to modify
//! its internal representation merely because:
//!
//! - the number of qubits changes;
//! - a new quantum processor appears;
//! - a new vendor appears;
//! - a new topology appears;
//! - routing changes;
//! - scheduling changes;
//! - pulse control is added;
//! - QEC is added;
//! - distributed quantum computing is added.
//!
//! Those concerns remain outside this module.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};

// =============================================================================
// Floating-point format
// =============================================================================

/// Semantic floating-point representation supported directly by this module.
///
/// The representation is deliberately independent of hardware.
///
/// `F64` is the concrete implementation used by [`ClassicalFloat`].
/// Additional formats can be introduced through the IR type/dialect system
/// without changing the semantic meaning of existing values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FloatFormat {
    /// IEEE-754 binary64.
    F64,
}

impl FloatFormat {
    /// Returns the number of bits in the representation.
    #[must_use]
    pub const fn bit_width(self) -> u16 {
        match self {
            Self::F64 => 64,
        }
    }

    /// Returns the number of bytes in the representation.
    #[must_use]
    pub const fn byte_width(self) -> u16 {
        self.bit_width() / 8
    }

    /// Returns the canonical type name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::F64 => "f64",
        }
    }
}

impl fmt::Display for FloatFormat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

// =============================================================================
// Error
// =============================================================================

/// Errors produced by checked classical floating-point operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassicalFloatError {
    /// Input or result is NaN or infinite.
    NonFinite,

    /// Division by zero was requested.
    DivisionByZero,

    /// Square root received a negative operand.
    NegativeSquareRoot,

    /// Natural logarithm received a non-positive operand.
    InvalidLogarithmDomain,

    /// Base-10 logarithm received a non-positive operand.
    InvalidLog10Domain,

    /// Exponentiation produced a non-finite result.
    NonFiniteResult,

    /// An approximation tolerance is invalid.
    InvalidTolerance,

    /// A conversion cannot be represented by the requested integer type.
    IntegerOverflow,

    /// A conversion would discard a fractional component.
    FractionalValue,

    /// A requested conversion is not supported.
    UnsupportedConversion,
}

impl fmt::Display for ClassicalFloatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => {
                formatter.write_str(
                    "classical floating-point value must be finite",
                )
            }

            Self::DivisionByZero => {
                formatter.write_str(
                    "classical floating-point division by zero",
                )
            }

            Self::NegativeSquareRoot => {
                formatter.write_str(
                    "square root domain requires a non-negative value",
                )
            }

            Self::InvalidLogarithmDomain => {
                formatter.write_str(
                    "logarithm domain requires a positive value",
                )
            }

            Self::InvalidLog10Domain => {
                formatter.write_str(
                    "base-10 logarithm domain requires a positive value",
                )
            }

            Self::NonFiniteResult => {
                formatter.write_str(
                    "floating-point operation produced a non-finite result",
                )
            }

            Self::InvalidTolerance => {
                formatter.write_str(
                    "floating-point comparison tolerance must be finite and non-negative",
                )
            }

            Self::IntegerOverflow => {
                formatter.write_str(
                    "floating-point value cannot be represented by the requested integer type",
                )
            }

            Self::FractionalValue => {
                formatter.write_str(
                    "floating-point value contains a fractional component",
                )
            }

            Self::UnsupportedConversion => {
                formatter.write_str(
                    "requested floating-point conversion is unsupported",
                )
            }
        }
    }
}

impl std::error::Error for ClassicalFloatError {}

// =============================================================================
// Canonical floating-point value
// =============================================================================

/// Canonical finite classical floating-point value.
///
/// This type is the semantic floating-point value layer for the classical IR.
///
/// # Representation
///
/// The current concrete representation is IEEE-754 binary64 (`f64`).
///
/// NaN and infinities are prohibited.
///
/// The underlying bits are preserved exactly, including the distinction
/// between positive and negative zero.
///
/// # Why this is a wrapper
///
/// A raw `f64` is insufficient as a canonical IR value because it permits
/// values whose equality and ordering semantics are problematic for
/// deterministic IR infrastructure.
///
/// This wrapper establishes the invariant once:
///
/// ```text
/// ClassicalFloat
///     => finite IEEE-754 binary64
/// ```
///
/// Downstream code therefore does not need to repeatedly check for NaN or
/// infinity after receiving a valid `ClassicalFloat`.
#[derive(Clone, Copy, Debug)]
pub struct ClassicalFloat {
    bits: u64,
}

impl ClassicalFloat {
    /// Creates a canonical classical float from an `f64`.
    ///
    /// NaN and positive/negative infinity are rejected.
    pub fn new(value: f64) -> Result<Self, ClassicalFloatError> {
        if !value.is_finite() {
            return Err(ClassicalFloatError::NonFinite);
        }

        Ok(Self {
            bits: value.to_bits(),
        })
    }

    /// Creates a canonical value from IEEE-754 bits.
    ///
    /// The bits are accepted only when they encode a finite value.
    pub fn from_bits(bits: u64) -> Result<Self, ClassicalFloatError> {
        let value = f64::from_bits(bits);

        if !value.is_finite() {
            return Err(ClassicalFloatError::NonFinite);
        }

        Ok(Self { bits })
    }

    /// Returns the underlying finite `f64`.
    #[must_use]
    pub fn get(self) -> f64 {
        f64::from_bits(self.bits)
    }

    /// Returns the exact IEEE-754 representation.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.bits
    }

    /// Returns the semantic floating-point format.
    #[must_use]
    pub const fn format(self) -> FloatFormat {
        FloatFormat::F64
    }

    /// Returns the canonical big-endian byte representation.
    #[must_use]
    pub fn canonical_bytes(self) -> [u8; 8] {
        self.bits.to_be_bytes()
    }

    /// Returns whether the value is positive zero.
    #[must_use]
    pub const fn is_positive_zero(self) -> bool {
        self.bits == 0
    }

    /// Returns whether the value is negative zero.
    #[must_use]
    pub const fn is_negative_zero(self) -> bool {
        self.bits == 0x8000_0000_0000_0000
    }

    /// Returns whether the value is either positive or negative zero.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.get() == 0.0
    }

    /// Returns whether the value is strictly positive.
    #[must_use]
    pub fn is_positive(self) -> bool {
        self.get() > 0.0
    }

    /// Returns whether the value is strictly negative.
    #[must_use]
    pub fn is_negative(self) -> bool {
        self.get() < 0.0
    }

    /// Returns the absolute value.
    #[must_use]
    pub fn abs(self) -> Self {
        Self::from_finite_unchecked(self.get().abs())
    }

    /// Returns the sign-preserving signum.
    ///
    /// The result is always finite.
    #[must_use]
    pub fn signum(self) -> Self {
        Self::from_finite_unchecked(self.get().signum())
    }

    /// Returns the minimum of two values using numeric comparison.
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        Self::from_finite_unchecked(self.get().min(other.get()))
    }

    /// Returns the maximum of two values using numeric comparison.
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        Self::from_finite_unchecked(self.get().max(other.get()))
    }

    /// Returns the mathematical floor.
    #[must_use]
    pub fn floor(self) -> Self {
        Self::from_finite_unchecked(self.get().floor())
    }

    /// Returns the mathematical ceiling.
    #[must_use]
    pub fn ceil(self) -> Self {
        Self::from_finite_unchecked(self.get().ceil())
    }

    /// Returns the nearest integer-valued floating-point representation.
    #[must_use]
    pub fn round(self) -> Self {
        Self::from_finite_unchecked(self.get().round())
    }

    /// Returns the truncated integer-valued floating-point representation.
    #[must_use]
    pub fn trunc(self) -> Self {
        Self::from_finite_unchecked(self.get().trunc())
    }

    /// Returns the fractional component.
    #[must_use]
    pub fn fract(self) -> Self {
        Self::from_finite_unchecked(self.get().fract())
    }

    /// Returns the square root.
    pub fn sqrt(self) -> Result<Self, ClassicalFloatError> {
        let value = self.get();

        if value < 0.0 {
            return Err(ClassicalFloatError::NegativeSquareRoot);
        }

        Self::checked_result(value.sqrt())
    }

    /// Returns the natural exponential.
    pub fn exp(self) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get().exp())
    }

    /// Returns the natural logarithm.
    pub fn ln(self) -> Result<Self, ClassicalFloatError> {
        let value = self.get();

        if value <= 0.0 {
            return Err(ClassicalFloatError::InvalidLogarithmDomain);
        }

        Self::checked_result(value.ln())
    }

    /// Returns the base-10 logarithm.
    pub fn log10(self) -> Result<Self, ClassicalFloatError> {
        let value = self.get();

        if value <= 0.0 {
            return Err(ClassicalFloatError::InvalidLog10Domain);
        }

        Self::checked_result(value.log10())
    }

    /// Returns the sine.
    pub fn sin(self) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get().sin())
    }

    /// Returns the cosine.
    pub fn cos(self) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get().cos())
    }

    /// Returns the tangent.
    pub fn tan(self) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get().tan())
    }

    /// Returns the arcsine.
    pub fn asin(self) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get().asin())
    }

    /// Returns the arccosine.
    pub fn acos(self) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get().acos())
    }

    /// Returns the arctangent.
    pub fn atan(self) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get().atan())
    }

    /// Adds two finite floating-point values.
    pub fn checked_add(
        self,
        other: Self,
    ) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get() + other.get())
    }

    /// Subtracts two finite floating-point values.
    pub fn checked_sub(
        self,
        other: Self,
    ) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get() - other.get())
    }

    /// Multiplies two finite floating-point values.
    pub fn checked_mul(
        self,
        other: Self,
    ) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get() * other.get())
    }

    /// Divides two finite floating-point values.
    ///
    /// Division by zero is rejected explicitly instead of allowing IEEE
    /// infinity or NaN to enter the semantic IR.
    pub fn checked_div(
        self,
        other: Self,
    ) -> Result<Self, ClassicalFloatError> {
        if other.is_zero() {
            return Err(ClassicalFloatError::DivisionByZero);
        }

        Self::checked_result(self.get() / other.get())
    }

    /// Computes the remainder.
    pub fn checked_rem(
        self,
        other: Self,
    ) -> Result<Self, ClassicalFloatError> {
        if other.is_zero() {
            return Err(ClassicalFloatError::DivisionByZero);
        }

        Self::checked_result(self.get() % other.get())
    }

    /// Raises this value to a floating-point exponent.
    pub fn checked_powf(
        self,
        exponent: Self,
    ) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get().powf(exponent.get()))
    }

    /// Raises this value to an integer exponent.
    pub fn checked_powi(
        self,
        exponent: i32,
    ) -> Result<Self, ClassicalFloatError> {
        Self::checked_result(self.get().powi(exponent))
    }

    /// Returns the reciprocal.
    pub fn checked_recip(self) -> Result<Self, ClassicalFloatError> {
        if self.is_zero() {
            return Err(ClassicalFloatError::DivisionByZero);
        }

        Self::checked_result(self.get().recip())
    }

    /// Performs an approximate comparison using an absolute tolerance.
    ///
    /// This operation is intentionally explicit. It never changes `Eq`,
    /// `Ord`, or `Hash`.
    pub fn approx_eq(
        self,
        other: Self,
        tolerance: f64,
    ) -> Result<bool, ClassicalFloatError> {
        if !tolerance.is_finite() || tolerance < 0.0 {
            return Err(ClassicalFloatError::InvalidTolerance);
        }

        let difference = (self.get() - other.get()).abs();

        Ok(difference <= tolerance)
    }

    /// Converts to `u64` only when the value is finite, non-negative,
    /// integral and representable.
    pub fn to_u64(self) -> Result<u64, ClassicalFloatError> {
        let value = self.get();

        if value < 0.0 || value > u64::MAX as f64 {
            return Err(ClassicalFloatError::IntegerOverflow);
        }

        if value.fract() != 0.0 {
            return Err(ClassicalFloatError::FractionalValue);
        }

        Ok(value as u64)
    }

    /// Converts to `i64` only when the value is finite, integral and
    /// representable.
    pub fn to_i64(self) -> Result<i64, ClassicalFloatError> {
        let value = self.get();

        if value < i64::MIN as f64 || value > i64::MAX as f64 {
            return Err(ClassicalFloatError::IntegerOverflow);
        }

        if value.fract() != 0.0 {
            return Err(ClassicalFloatError::FractionalValue);
        }

        Ok(value as i64)
    }

    /// Validates the internal invariant.
    ///
    /// This is cheap and may be called by IR validation passes.
    pub fn validate(self) -> Result<(), ClassicalFloatError> {
        if self.get().is_finite() {
            Ok(())
        } else {
            Err(ClassicalFloatError::NonFinite)
        }
    }

    /// Creates a value from a known finite result.
    ///
    /// This function is private because callers must not be allowed to bypass
    /// the invariant without first establishing finiteness.
    fn from_finite_unchecked(value: f64) -> Self {
        debug_assert!(value.is_finite());

        Self {
            bits: value.to_bits(),
        }
    }

    /// Converts an arithmetic result into the canonical representation.
    fn checked_result(value: f64) -> Result<Self, ClassicalFloatError> {
        if !value.is_finite() {
            return Err(ClassicalFloatError::NonFiniteResult);
        }

        Ok(Self {
            bits: value.to_bits(),
        })
    }
}

// =============================================================================
// Equality / ordering / hashing
// =============================================================================

impl PartialEq for ClassicalFloat {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

impl Eq for ClassicalFloat {}

impl Hash for ClassicalFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bits.hash(state);
    }
}

impl PartialOrd for ClassicalFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

impl Ord for ClassicalFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().total_cmp(&other.get())
    }
}

impl fmt::Display for ClassicalFloat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

// =============================================================================
// Conversions
// =============================================================================

impl TryFrom<f64> for ClassicalFloat {
    type Error = ClassicalFloatError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ClassicalFloat> for f64 {
    fn from(value: ClassicalFloat) -> Self {
        value.get()
    }
}

impl From<ClassicalFloat> for u64 {
    fn from(value: ClassicalFloat) -> Self {
        value.bits()
    }
}

// =============================================================================
// Constants
// =============================================================================

impl ClassicalFloat {
    /// Positive zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self { bits: 0 }
    }

    /// Negative zero.
    #[must_use]
    pub const fn negative_zero() -> Self {
        Self {
            bits: 0x8000_0000_0000_0000,
        }
    }

    /// Positive one.
    #[must_use]
    pub const fn one() -> Self {
        Self {
            bits: 0x3ff0_0000_0000_0000,
        }
    }

    /// Negative one.
    #[must_use]
    pub const fn negative_one() -> Self {
        Self {
            bits: 0xbff0_0000_0000_0000,
        }
    }

    /// Positive infinity cannot be represented by this semantic type.
    ///
    /// This method intentionally does not exist as a constructor. Keeping
    /// infinity outside the canonical value domain is part of the invariant.
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::{
        ClassicalFloat,
        ClassicalFloatError,
        FloatFormat,
    };

    #[test]
    fn finite_values_are_accepted() {
        let value = ClassicalFloat::new(0.25).expect("finite value");

        assert_eq!(value.get(), 0.25);
        assert_eq!(value.format(), FloatFormat::F64);
        assert_eq!(value.bits(), 0x3fd0_0000_0000_0000);
    }

    #[test]
    fn_nan_is_rejected() {
        let result = ClassicalFloat::new(f64::NAN);

        assert_eq!(
            result,
            Err(ClassicalFloatError::NonFinite)
        );
    }

    #[test]
    fn_positive_infinity_is_rejected() {
        let result = ClassicalFloat::new(f64::INFINITY);

        assert_eq!(
            result,
            Err(ClassicalFloatError::NonFinite)
        );
    }

    #[test]
    fn_negative_infinity_is_rejected() {
        let result = ClassicalFloat::new(f64::NEG_INFINITY);

        assert_eq!(
            result,
            Err(ClassicalFloatError::NonFinite)
        );
    }

    #[test]
    fn_positive_and_negative_zero_have_distinct_canonical_bits() {
        let positive = ClassicalFloat::zero();
        let negative = ClassicalFloat::negative_zero();

        assert_ne!(positive, negative);
        assert_eq!(positive.get(), 0.0);
        assert_eq!(negative.get(), -0.0);
        assert!(positive.is_positive_zero());
        assert!(negative.is_negative_zero());
    }

    #[test]
    fn_one_constants_are_correct() {
        assert_eq!(ClassicalFloat::one().get(), 1.0);
        assert_eq!(ClassicalFloat::negative_one().get(), -1.0);
    }

    #[test]
    fn_bits_round_trip() {
        let original = ClassicalFloat::new(-12.5).expect("finite");

        let reconstructed =
            ClassicalFloat::from_bits(original.bits()).expect("finite bits");

        assert_eq!(original, reconstructed);
        assert_eq!(original.canonical_bytes(), reconstructed.canonical_bytes());
    }

    #[test]
    fn arithmetic_preserves_finite_invariant() {
        let left = ClassicalFloat::new(2.0).expect("finite");
        let right = ClassicalFloat::new(3.0).expect("finite");

        let result = left.checked_mul(right).expect("finite result");

        assert_eq!(result.get(), 6.0);
    }

    #[test]
    fn division_by_zero_is_rejected() {
        let left = ClassicalFloat::one();
        let zero = ClassicalFloat::zero();

        assert_eq!(
            left.checked_div(zero),
            Err(ClassicalFloatError::DivisionByZero)
        );
    }

    #[test]
    fn overflowing_division_is_rejected() {
        let numerator =
            ClassicalFloat::new(f64::MAX).expect("finite");
        let denominator =
            ClassicalFloat::new(0.5).expect("finite");

        assert_eq!(
            numerator.checked_div(denominator),
            Err(ClassicalFloatError::NonFiniteResult)
        );
    }

    #[test]
    fn negative_square_root_is_rejected() {
        let value =
            ClassicalFloat::new(-1.0).expect("finite");

        assert_eq!(
            value.sqrt(),
            Err(ClassicalFloatError::NegativeSquareRoot)
        );
    }

    #[test]
    fn logarithm_domain_is_checked() {
        let zero = ClassicalFloat::zero();

        assert_eq!(
            zero.ln(),
            Err(ClassicalFloatError::InvalidLogarithmDomain)
        );
    }

    #[test]
    fn approximate_equality_is_explicit() {
        let left =
            ClassicalFloat::new(1.0).expect("finite");
        let right =
            ClassicalFloat::new(1.000_000_1).expect("finite");

        assert!(
            left.approx_eq(right, 0.000_001)
                .expect("valid tolerance")
        );
    }

    #[test]
    fn invalid_tolerance_is_rejected() {
        let value = ClassicalFloat::one();

        assert_eq!(
            value.approx_eq(value, -1.0),
            Err(ClassicalFloatError::InvalidTolerance)
        );

        assert_eq!(
            value.approx_eq(value, f64::NAN),
            Err(ClassicalFloatError::InvalidTolerance)
        );
    }

    #[test]
    fn integral_conversion_is_checked() {
        let value =
            ClassicalFloat::new(42.0).expect("finite");

        assert_eq!(value.to_u64().expect("integral"), 42);
        assert_eq!(value.to_i64().expect("integral"), 42);
    }

    #[test]
    fn fractional_conversion_is_rejected() {
        let value =
            ClassicalFloat::new(42.5).expect("finite");

        assert_eq!(
            value.to_u64(),
            Err(ClassicalFloatError::FractionalValue)
        );
    }

    #[test]
    fn numeric_ordering_differs_from_structural_zero_identity() {
        let positive = ClassicalFloat::zero();
        let negative = ClassicalFloat::negative_zero();

        assert_ne!(positive, negative);
        assert_eq!(
            positive.partial_cmp(&negative),
            Some(Ordering::Equal)
        );
        assert_eq!(
            positive.cmp(&negative),
            Ordering::Greater
        );
    }

    #[test]
    fn canonical_bytes_are_big_endian() {
        let value = ClassicalFloat::one();

        assert_eq!(
            value.canonical_bytes(),
            [0x3f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn validation_accepts_valid_values() {
        let value =
            ClassicalFloat::new(123.456).expect("finite");

        assert_eq!(value.validate(), Ok(()));
    }

    #[test]
    fn format_metadata_is_stable() {
        assert_eq!(FloatFormat::F64.bit_width(), 64);
        assert_eq!(FloatFormat::F64.byte_width(), 8);
        assert_eq!(FloatFormat::F64.name(), "f64");
    }
}