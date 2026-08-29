//! Zamani Quantum Memory — Complex Scalar Types
//!
//! Production-grade, provider-neutral complex-number primitives for
//! `crate::quantum::memory`.
//!
//! # Responsibility
//!
//! This module defines the canonical complex scalar types used by the quantum
//! memory subsystem:
//!
//! - [`Complex32`] — single-precision complex value;
//! - [`Complex64`] — double-precision complex value;
//! - [`ComplexScalar`] — common abstraction implemented by both;
//! - [`ComplexError`] — validation errors for complex values.
//!
//! The module is deliberately independent of:
//!
//! - quantum IR;
//! - state-vector storage;
//! - density matrices;
//! - tensor networks;
//! - GPU runtimes;
//! - CPU SIMD implementations;
//! - hardware providers;
//! - simulators;
//! - routing;
//! - scheduling;
//! - benchmarking;
//! - serialization;
//! - compiler/frontend code.
//!
//! Those systems consume this numerical contract rather than redefining
//! complex-number semantics.
//!
//! # Architectural position
//!
//! ```text
//!                         Zamani Quantum
//!                              |
//!                              v
//!                       quantum::memory
//!                              |
//!             +----------------+----------------+
//!             |                |                |
//!             v                v                v
//!        state_vector    density_matrix      tensor
//!             |                |                |
//!             +----------------+----------------+
//!                              |
//!                              v
//!                     memory::complex
//!                              |
//!                    +---------+---------+
//!                    |                   |
//!                    v                   v
//!               Complex32            Complex64
//! ```
//!
//! # Design goals
//!
//! The implementation provides:
//!
//! - finite-value validation;
//! - exact zero/one/imaginary-unit constants;
//! - conjugation;
//! - squared magnitude;
//! - magnitude;
//! - phase;
//! - normalization;
//! - checked division;
//! - checked reciprocal;
//! - checked arithmetic helpers;
//! - approximate equality;
//! - stable deterministic formatting;
//! - conversion between precisions;
//! - conversion from real values;
//! - arithmetic operator implementations;
//! - `Sum` and `Product` support;
//! - no panics for invalid numerical operations;
//! - no hidden allocation;
//! - no global state;
//! - no `unsafe`;
//! - no external dependency.
//!
//! # Numerical safety
//!
//! Quantum amplitudes must not silently contain:
//!
//! - NaN;
//! - positive infinity;
//! - negative infinity.
//!
//! Constructors that establish validated quantum-memory values therefore
//! reject non-finite components.
//!
//! Arithmetic operators follow normal Rust operator semantics and are
//! intentionally infallible at the type level. For operations where an
//! invalid mathematical result must be reported, callers should use the
//! corresponding checked method:
//!
//! - [`ComplexScalar::checked_div`];
//! - [`ComplexScalar::checked_recip`];
//! - [`ComplexScalar::checked_normalize`];
//! - [`ComplexScalar::try_from_polar`];
//!
//! This distinction is important because operator traits cannot return
//! `Result` without making ordinary numerical code unusable.
//!
//! # Precision policy
//!
//! `Complex64` is the canonical default precision for high-accuracy quantum
//! simulation.
//!
//! `Complex32` exists for:
//!
//! - GPU workloads;
//! - memory-constrained simulation;
//! - SIMD kernels;
//! - approximate algorithms;
//! - accelerator-native storage.
//!
//! Conversion from `Complex64` to `Complex32` is explicit and fallible so that
//! precision loss is never accidentally hidden.
//!
//! # Representation
//!
//! The structs use `#[repr(C)]` so their two scalar fields have a stable,
//! conventional field ordering suitable for future safe interoperability
//! boundaries.
//!
//! This module does not expose raw pointers or perform FFI.
//!
//! # Integration contract
//!
//! `numeric.rs`
//!     May later define broader scalar/numerical policies. It must consume
//!     these types rather than redefine them.
//!
//! `representation.rs`
//!     May identify `Complex32` and `Complex64` as supported precisions.
//!
//! `limits.rs`
//!     Uses the element size exposed through [`ComplexScalar::BYTE_SIZE`] when
//!     calculating state-memory requirements.
//!
//! `layout.rs` / `indexing.rs`
//!     Do not need to know complex arithmetic. They operate on indices.
//!
//! `allocator.rs`
//!     Allocates storage containing these values but does not own their
//!     numerical semantics.
//!
//! `state_vector.rs`
//!     Uses `Complex32` or `Complex64` as amplitude storage.
//!
//! `density_matrix.rs`
//!     Uses complex values for matrix elements.
//!
//! `tensor.rs`
//!     Uses the scalar abstraction to support generic tensor storage.
//!
//! `simd.rs`
//!     May implement optimized kernels while preserving this module's
//!     semantics.
//!
//! `gpu.rs`
//!     May transfer these values to device memory through provider-specific
//!     implementations without changing the canonical Rust representation.
//!
//! `serialization.rs`
//!     Must serialize real and imaginary components explicitly and preserve
//!     precision metadata.
//!
//! `snapshot.rs` / `checkpoint.rs`
//!     Consume the canonical scalar representation through serialization.
//!
//! # No-reedit contract
//!
//! This file owns the semantics of Zamani's basic complex values.
//!
//! Future memory modules MUST NOT:
//!
//! - define another `Complex64`;
//! - define another `Complex32`;
//! - invent different conjugation semantics;
//! - invent different magnitude semantics;
//! - silently accept NaN amplitudes;
//! - silently accept infinite amplitudes;
//! - redefine zero, one, or imaginary-unit values;
//! - introduce provider-specific complex types into the canonical memory API.
//!
//! Provider-specific or accelerator-specific types belong behind adapter
//! boundaries.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Security
//!
//! This module contains no:
//!
//! - network operations;
//! - filesystem operations;
//! - process execution;
//! - credentials;
//! - global mutable state;
//! - dynamic code execution.
//!
//! # Testing
//!
//! The unit tests at the bottom of this file validate the numerical contract
//! independently of the rest of `quantum::memory`.
//!
//! Later state representations should add differential tests against this
//! contract rather than changing this file for representation-specific needs.
//!
//! # References
//!
//! The implementation intentionally follows the conventional Cartesian
//! complex representation used by established Rust numerical ecosystems while
//! keeping Zamani's foundational memory layer dependency-independent.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};

/// Machine-readable identifier for this numerical contract.
pub const COMPLEX_SCHEMA_ID: &str = "zamani.quantum.memory.complex";

/// Semantic version of the complex-number contract.
///
/// This version changes only when the public semantic contract changes.
pub const COMPLEX_SCHEMA_VERSION: u16 = 1;

/// Default absolute tolerance for double-precision quantum calculations.
///
/// This is deliberately exposed as a constant rather than scattering
/// numerical literals through state implementations.
pub const DEFAULT_F64_ABS_TOLERANCE: f64 = 1.0e-12;

/// Default relative tolerance for double-precision quantum calculations.
pub const DEFAULT_F64_REL_TOLERANCE: f64 = 1.0e-10;

/// Default absolute tolerance for single-precision quantum calculations.
pub const DEFAULT_F32_ABS_TOLERANCE: f32 = 1.0e-6;

/// Default relative tolerance for single-precision quantum calculations.
pub const DEFAULT_F32_REL_TOLERANCE: f32 = 1.0e-5;

/// Error type for validated complex-number operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComplexError {
    /// At least one component was NaN or infinite.
    NonFinite,

    /// A divisor has zero magnitude.
    DivisionByZero,

    /// A normalization operation was requested for the zero vector.
    CannotNormalizeZero,

    /// A conversion would overflow the destination representation.
    ConversionOverflow,

    /// A finite source value cannot be represented by the destination
    /// precision without becoming non-finite.
    ConversionNonFinite,

    /// A polar radius is negative.
    NegativeRadius,

    /// A polar angle or radius is non-finite.
    NonFinitePolarCoordinate,
}

impl fmt::Display for ComplexError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => {
                formatter.write_str("complex value contains a non-finite component")
            }
            Self::DivisionByZero => {
                formatter.write_str("division by a zero-magnitude complex value")
            }
            Self::CannotNormalizeZero => {
                formatter.write_str("cannot normalize a zero-magnitude complex value")
            }
            Self::ConversionOverflow => {
                formatter.write_str("complex conversion overflowed the destination precision")
            }
            Self::ConversionNonFinite => {
                formatter.write_str(
                    "complex conversion produced a non-finite destination component",
                )
            }
            Self::NegativeRadius => {
                formatter.write_str("polar radius cannot be negative")
            }
            Self::NonFinitePolarCoordinate => {
                formatter.write_str("polar radius and angle must be finite")
            }
        }
    }
}

impl std::error::Error for ComplexError {}

/// Common interface for Zamani complex scalar types.
///
/// This trait deliberately contains only operations that are meaningful for
/// all supported floating-point precisions.
///
/// It is the primary integration boundary for:
///
/// - state vectors;
/// - density matrices;
/// - tensors;
/// - sparse states;
/// - tensor networks;
/// - CPU kernels;
/// - GPU abstractions.
///
/// It does not depend on any future `numeric.rs` module so this file remains
/// independently complete.
pub trait ComplexScalar:
    Copy
    + Clone
    + Send
    + Sync
    + PartialEq
    + fmt::Debug
    + fmt::Display
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
    /// Underlying real component type.
    type Real: Copy
        + Clone
        + PartialEq
        + PartialOrd
        + fmt::Debug
        + fmt::Display;

    /// Number of bytes occupied by one complex scalar.
    const BYTE_SIZE: usize;

    /// Precision in bits of one real component.
    const REAL_BITS: u16;

    /// Returns zero.
    fn zero() -> Self;

    /// Returns one.
    fn one() -> Self;

    /// Returns the imaginary unit.
    fn i() -> Self;

    /// Constructs a complex value after validating finiteness.
    fn try_new(real: Self::Real, imaginary: Self::Real)
        -> Result<Self, ComplexError>;

    /// Constructs a complex value without a runtime fallibility boundary.
    ///
    /// # Panics
    ///
    /// Panics if either component is non-finite.
    ///
    /// Quantum-memory code that processes untrusted numerical data should use
    /// [`ComplexScalar::try_new`] instead.
    fn new(real: Self::Real, imaginary: Self::Real) -> Self;

    /// Returns the real component.
    fn real(self) -> Self::Real;

    /// Returns the imaginary component.
    fn imaginary(self) -> Self::Real;

    /// Returns the complex conjugate.
    fn conjugate(self) -> Self;

    /// Returns the squared magnitude.
    fn norm_squared(self) -> Self::Real;

    /// Returns the magnitude.
    fn magnitude(self) -> Self::Real;

    /// Returns the phase angle in radians.
    fn phase(self) -> Self::Real;

    /// Returns whether both components are finite.
    fn is_finite(self) -> bool;

    /// Returns whether the value contains NaN.
    fn is_nan(self) -> bool;

    /// Returns whether the magnitude is zero.
    fn is_zero(self) -> bool;

    /// Checked reciprocal.
    fn checked_recip(self) -> Result<Self, ComplexError>;

    /// Checked division.
    fn checked_div(self, rhs: Self) -> Result<Self, ComplexError>;

    /// Checked normalization.
    fn checked_normalize(self) -> Result<Self, ComplexError>;

    /// Returns true when two values are approximately equal under the supplied
    /// absolute and relative tolerances.
    fn approx_eq(
        self,
        other: Self,
        abs_tolerance: Self::Real,
        rel_tolerance: Self::Real,
    ) -> bool;

    /// Creates a complex number from polar coordinates.
    fn try_from_polar(
        radius: Self::Real,
        angle: Self::Real,
    ) -> Result<Self, ComplexError>;

    /// Returns the real scalar corresponding to the supplied value.
    fn from_real(value: Self::Real) -> Result<Self, ComplexError>;

    /// Returns a zero-initialized complex value.
    fn default_zero() -> Self {
        Self::zero()
    }
}

/// Single-precision complex number.
///
/// The fields are named `real` and `imaginary` rather than `re` and `im` to
/// make the public Zamani API explicit and language-facing.
///
/// `#[repr(C)]` provides a conventional stable field ordering without exposing
/// pointers or requiring unsafe code.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex32 {
    /// Real component.
    pub real: f32,

    /// Imaginary component.
    pub imaginary: f32,
}

impl Complex32 {
    /// Creates a validated single-precision complex number.
    pub fn try_new(real: f32, imaginary: f32) -> Result<Self, ComplexError> {
        <Self as ComplexScalar>::try_new(real, imaginary)
    }

    /// Creates a single-precision complex number.
    ///
    /// # Panics
    ///
    /// Panics if either component is non-finite.
    pub fn new(real: f32, imaginary: f32) -> Self {
        <Self as ComplexScalar>::new(real, imaginary)
    }

    /// Zero.
    pub const ZERO: Self = Self {
        real: 0.0,
        imaginary: 0.0,
    };

    /// One.
    pub const ONE: Self = Self {
        real: 1.0,
        imaginary: 0.0,
    };

    /// Imaginary unit.
    pub const I: Self = Self {
        real: 0.0,
        imaginary: 1.0,
    };

    /// Returns the real component.
    pub const fn real(self) -> f32 {
        self.real
    }

    /// Returns the imaginary component.
    pub const fn imaginary(self) -> f32 {
        self.imaginary
    }

    /// Returns the conjugate.
    pub const fn conjugate(self) -> Self {
        Self {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }

    /// Returns squared magnitude.
    pub fn norm_squared(self) -> f32 {
        self.real.mul_add(self.real, self.imaginary * self.imaginary)
    }

    /// Returns magnitude.
    pub fn magnitude(self) -> f32 {
        self.norm_squared().sqrt()
    }

    /// Returns phase in radians.
    pub fn phase(self) -> f32 {
        self.imaginary.atan2(self.real)
    }

    /// Returns whether all components are finite.
    pub fn is_finite(self) -> bool {
        self.real.is_finite() && self.imaginary.is_finite()
    }

    /// Returns whether either component is NaN.
    pub fn is_nan(self) -> bool {
        self.real.is_nan() || self.imaginary.is_nan()
    }

    /// Returns whether the complex value is exactly zero.
    pub fn is_zero(self) -> bool {
        self.real == 0.0 && self.imaginary == 0.0
    }

    /// Checked reciprocal.
    pub fn checked_recip(self) -> Result<Self, ComplexError> {
        <Self as ComplexScalar>::checked_recip(self)
    }

    /// Checked division.
    pub fn checked_div(self, rhs: Self) -> Result<Self, ComplexError> {
        <Self as ComplexScalar>::checked_div(self, rhs)
    }

    /// Checked normalization.
    pub fn checked_normalize(self) -> Result<Self, ComplexError> {
        <Self as ComplexScalar>::checked_normalize(self)
    }

    /// Approximate equality.
    pub fn approx_eq(
        self,
        other: Self,
        abs_tolerance: f32,
        rel_tolerance: f32,
    ) -> bool {
        <Self as ComplexScalar>::approx_eq(
            self,
            other,
            abs_tolerance,
            rel_tolerance,
        )
    }

    /// Constructs a value from polar coordinates.
    pub fn try_from_polar(
        radius: f32,
        angle: f32,
    ) -> Result<Self, ComplexError> {
        <Self as ComplexScalar>::try_from_polar(radius, angle)
    }

    /// Converts this value to double precision without loss of the represented
    /// `f32` value.
    pub fn to_complex64(self) -> Complex64 {
        Complex64 {
            real: self.real as f64,
            imaginary: self.imaginary as f64,
        }
    }
}

impl fmt::Display for Complex32 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imaginary.is_sign_negative() {
            write!(
                formatter,
                "{} - {}i",
                self.real,
                self.imaginary.abs()
            )
        } else {
            write!(formatter, "{} + {}i", self.real, self.imaginary)
        }
    }
}

/// Double-precision complex number.
///
/// This is Zamani's default high-accuracy complex scalar.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex64 {
    /// Real component.
    pub real: f64,

    /// Imaginary component.
    pub imaginary: f64,
}

impl Complex64 {
    /// Creates a validated double-precision complex number.
    pub fn try_new(real: f64, imaginary: f64) -> Result<Self, ComplexError> {
        <Self as ComplexScalar>::try_new(real, imaginary)
    }

    /// Creates a double-precision complex number.
    ///
    /// # Panics
    ///
    /// Panics if either component is non-finite.
    pub fn new(real: f64, imaginary: f64) -> Self {
        <Self as ComplexScalar>::new(real, imaginary)
    }

    /// Zero.
    pub const ZERO: Self = Self {
        real: 0.0,
        imaginary: 0.0,
    };

    /// One.
    pub const ONE: Self = Self {
        real: 1.0,
        imaginary: 0.0,
    };

    /// Imaginary unit.
    pub const I: Self = Self {
        real: 0.0,
        imaginary: 1.0,
    };

    /// Returns the real component.
    pub const fn real(self) -> f64 {
        self.real
    }

    /// Returns the imaginary component.
    pub const fn imaginary(self) -> f64 {
        self.imaginary
    }

    /// Returns the conjugate.
    pub const fn conjugate(self) -> Self {
        Self {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }

    /// Returns squared magnitude.
    pub fn norm_squared(self) -> f64 {
        self.real.mul_add(self.real, self.imaginary * self.imaginary)
    }

    /// Returns magnitude.
    pub fn magnitude(self) -> f64 {
        self.norm_squared().sqrt()
    }

    /// Returns phase in radians.
    pub fn phase(self) -> f64 {
        self.imaginary.atan2(self.real)
    }

    /// Returns whether all components are finite.
    pub fn is_finite(self) -> bool {
        self.real.is_finite() && self.imaginary.is_finite()
    }

    /// Returns whether either component is NaN.
    pub fn is_nan(self) -> bool {
        self.real.is_nan() || self.imaginary.is_nan()
    }

    /// Returns whether the complex value is exactly zero.
    pub fn is_zero(self) -> bool {
        self.real == 0.0 && self.imaginary == 0.0
    }

    /// Checked reciprocal.
    pub fn checked_recip(self) -> Result<Self, ComplexError> {
        <Self as ComplexScalar>::checked_recip(self)
    }

    /// Checked division.
    pub fn checked_div(self, rhs: Self) -> Result<Self, ComplexError> {
        <Self as ComplexScalar>::checked_div(self, rhs)
    }

    /// Checked normalization.
    pub fn checked_normalize(self) -> Result<Self, ComplexError> {
        <Self as ComplexScalar>::checked_normalize(self)
    }

    /// Approximate equality.
    pub fn approx_eq(
        self,
        other: Self,
        abs_tolerance: f64,
        rel_tolerance: f64,
    ) -> bool {
        <Self as ComplexScalar>::approx_eq(
            self,
            other,
            abs_tolerance,
            rel_tolerance,
        )
    }

    /// Constructs a value from polar coordinates.
    pub fn try_from_polar(
        radius: f64,
        angle: f64,
    ) -> Result<Self, ComplexError> {
        <Self as ComplexScalar>::try_from_polar(radius, angle)
    }

    /// Converts this value to single precision after explicit validation.
    pub fn try_to_complex32(self) -> Result<Complex32, ComplexError> {
        if !self.is_finite() {
            return Err(ComplexError::NonFinite);
        }

        let real = self.real as f32;
        let imaginary = self.imaginary as f32;

        if !real.is_finite() || !imaginary.is_finite() {
            return Err(ComplexError::ConversionNonFinite);
        }

        Ok(Complex32 { real, imaginary })
    }
}

impl fmt::Display for Complex64 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imaginary.is_sign_negative() {
            write!(
                formatter,
                "{} - {}i",
                self.real,
                self.imaginary.abs()
            )
        } else {
            write!(formatter, "{} + {}i", self.real, self.imaginary)
        }
    }
}

// =============================================================================
// Shared implementation helpers
// =============================================================================

fn approx_scalar_f32(
    lhs: f32,
    rhs: f32,
    abs_tolerance: f32,
    rel_tolerance: f32,
) -> bool {
    if !abs_tolerance.is_finite()
        || !rel_tolerance.is_finite()
        || abs_tolerance < 0.0
        || rel_tolerance < 0.0
    {
        return false;
    }

    if !lhs.is_finite() || !rhs.is_finite() {
        return lhs == rhs;
    }

    let difference = (lhs - rhs).abs();

    if difference <= abs_tolerance {
        return true;
    }

    let scale = lhs.abs().max(rhs.abs());

    difference <= rel_tolerance * scale
}

fn approx_scalar_f64(
    lhs: f64,
    rhs: f64,
    abs_tolerance: f64,
    rel_tolerance: f64,
) -> bool {
    if !abs_tolerance.is_finite()
        || !rel_tolerance.is_finite()
        || abs_tolerance < 0.0
        || rel_tolerance < 0.0
    {
        return false;
    }

    if !lhs.is_finite() || !rhs.is_finite() {
        return lhs == rhs;
    }

    let difference = (lhs - rhs).abs();

    if difference <= abs_tolerance {
        return true;
    }

    let scale = lhs.abs().max(rhs.abs());

    difference <= rel_tolerance * scale
}

// =============================================================================
// Complex32 implementation
// =============================================================================

impl ComplexScalar for Complex32 {
    type Real = f32;

    const BYTE_SIZE: usize = core::mem::size_of::<Self>();
    const REAL_BITS: u16 = 32;

    fn zero() -> Self {
        Self::ZERO
    }

    fn one() -> Self {
        Self::ONE
    }

    fn i() -> Self {
        Self::I
    }

    fn try_new(
        real: Self::Real,
        imaginary: Self::Real,
    ) -> Result<Self, ComplexError> {
        if !real.is_finite() || !imaginary.is_finite() {
            return Err(ComplexError::NonFinite);
        }

        Ok(Self { real, imaginary })
    }

    fn new(real: Self::Real, imaginary: Self::Real) -> Self {
        match Self::try_new(real, imaginary) {
            Ok(value) => value,
            Err(error) => panic!("invalid Complex32 construction: {error}"),
        }
    }

    fn real(self) -> Self::Real {
        self.real
    }

    fn imaginary(self) -> Self::Real {
        self.imaginary
    }

    fn conjugate(self) -> Self {
        self.conjugate()
    }

    fn norm_squared(self) -> Self::Real {
        self.norm_squared()
    }

    fn magnitude(self) -> Self::Real {
        self.magnitude()
    }

    fn phase(self) -> Self::Real {
        self.phase()
    }

    fn is_finite(self) -> bool {
        self.is_finite()
    }

    fn is_nan(self) -> bool {
        self.is_nan()
    }

    fn is_zero(self) -> bool {
        self.is_zero()
    }

    fn checked_recip(self) -> Result<Self, ComplexError> {
        if !self.is_finite() {
            return Err(ComplexError::NonFinite);
        }

        let norm_squared = self.norm_squared();

        if norm_squared == 0.0 {
            return Err(ComplexError::DivisionByZero);
        }

        let result = Self {
            real: self.real / norm_squared,
            imaginary: -self.imaginary / norm_squared,
        };

        if !result.is_finite() {
            return Err(ComplexError::ConversionOverflow);
        }

        Ok(result)
    }

    fn checked_div(self, rhs: Self) -> Result<Self, ComplexError> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(ComplexError::NonFinite);
        }

        let denominator = rhs.norm_squared();

        if denominator == 0.0 {
            return Err(ComplexError::DivisionByZero);
        }

        let numerator_real =
            self.real * rhs.real + self.imaginary * rhs.imaginary;

        let numerator_imaginary =
            self.imaginary * rhs.real - self.real * rhs.imaginary;

        let result = Self {
            real: numerator_real / denominator,
            imaginary: numerator_imaginary / denominator,
        };

        if !result.is_finite() {
            return Err(ComplexError::ConversionOverflow);
        }

        Ok(result)
    }

    fn checked_normalize(self) -> Result<Self, ComplexError> {
        if !self.is_finite() {
            return Err(ComplexError::NonFinite);
        }

        let magnitude = self.magnitude();

        if magnitude == 0.0 {
            return Err(ComplexError::CannotNormalizeZero);
        }

        let result = Self {
            real: self.real / magnitude,
            imaginary: self.imaginary / magnitude,
        };

        if !result.is_finite() {
            return Err(ComplexError::ConversionOverflow);
        }

        Ok(result)
    }

    fn approx_eq(
        self,
        other: Self,
        abs_tolerance: Self::Real,
        rel_tolerance: Self::Real,
    ) -> bool {
        approx_scalar_f32(
            self.real,
            other.real,
            abs_tolerance,
            rel_tolerance,
        ) && approx_scalar_f32(
            self.imaginary,
            other.imaginary,
            abs_tolerance,
            rel_tolerance,
        )
    }

    fn try_from_polar(
        radius: Self::Real,
        angle: Self::Real,
    ) -> Result<Self, ComplexError> {
        if !radius.is_finite() || !angle.is_finite() {
            return Err(ComplexError::NonFinitePolarCoordinate);
        }

        if radius < 0.0 {
            return Err(ComplexError::NegativeRadius);
        }

        let result = Self {
            real: radius * angle.cos(),
            imaginary: radius * angle.sin(),
        };

        if !result.is_finite() {
            return Err(ComplexError::ConversionOverflow);
        }

        Ok(result)
    }

    fn from_real(value: Self::Real) -> Result<Self, ComplexError> {
        Self::try_new(value, 0.0)
    }
}

// =============================================================================
// Complex64 implementation
// =============================================================================

impl ComplexScalar for Complex64 {
    type Real = f64;

    const BYTE_SIZE: usize = core::mem::size_of::<Self>();
    const REAL_BITS: u16 = 64;

    fn zero() -> Self {
        Self::ZERO
    }

    fn one() -> Self {
        Self::ONE
    }

    fn i() -> Self {
        Self::I
    }

    fn try_new(
        real: Self::Real,
        imaginary: Self::Real,
    ) -> Result<Self, ComplexError> {
        if !real.is_finite() || !imaginary.is_finite() {
            return Err(ComplexError::NonFinite);
        }

        Ok(Self { real, imaginary })
    }

    fn new(real: Self::Real, imaginary: Self::Real) -> Self {
        match Self::try_new(real, imaginary) {
            Ok(value) => value,
            Err(error) => panic!("invalid Complex64 construction: {error}"),
        }
    }

    fn real(self) -> Self::Real {
        self.real
    }

    fn imaginary(self) -> Self::Real {
        self.imaginary
    }

    fn conjugate(self) -> Self {
        self.conjugate()
    }

    fn norm_squared(self) -> Self::Real {
        self.norm_squared()
    }

    fn magnitude(self) -> Self::Real {
        self.magnitude()
    }

    fn phase(self) -> Self::Real {
        self.phase()
    }

    fn is_finite(self) -> bool {
        self.is_finite()
    }

    fn is_nan(self) -> bool {
        self.is_nan()
    }

    fn is_zero(self) -> bool {
        self.is_zero()
    }

    fn checked_recip(self) -> Result<Self, ComplexError> {
        if !self.is_finite() {
            return Err(ComplexError::NonFinite);
        }

        let norm_squared = self.norm_squared();

        if norm_squared == 0.0 {
            return Err(ComplexError::DivisionByZero);
        }

        let result = Self {
            real: self.real / norm_squared,
            imaginary: -self.imaginary / norm_squared,
        };

        if !result.is_finite() {
            return Err(ComplexError::ConversionOverflow);
        }

        Ok(result)
    }

    fn checked_div(self, rhs: Self) -> Result<Self, ComplexError> {
        if !self.is_finite() || !rhs.is_finite() {
            return Err(ComplexError::NonFinite);
        }

        let denominator = rhs.norm_squared();

        if denominator == 0.0 {
            return Err(ComplexError::DivisionByZero);
        }

        let numerator_real =
            self.real * rhs.real + self.imaginary * rhs.imaginary;

        let numerator_imaginary =
            self.imaginary * rhs.real - self.real * rhs.imaginary;

        let result = Self {
            real: numerator_real / denominator,
            imaginary: numerator_imaginary / denominator,
        };

        if !result.is_finite() {
            return Err(ComplexError::ConversionOverflow);
        }

        Ok(result)
    }

    fn checked_normalize(self) -> Result<Self, ComplexError> {
        if !self.is_finite() {
            return Err(ComplexError::NonFinite);
        }

        let magnitude = self.magnitude();

        if magnitude == 0.0 {
            return Err(ComplexError::CannotNormalizeZero);
        }

        let result = Self {
            real: self.real / magnitude,
            imaginary: self.imaginary / magnitude,
        };

        if !result.is_finite() {
            return Err(ComplexError::ConversionOverflow);
        }

        Ok(result)
    }

    fn approx_eq(
        self,
        other: Self,
        abs_tolerance: Self::Real,
        rel_tolerance: Self::Real,
    ) -> bool {
        approx_scalar_f64(
            self.real,
            other.real,
            abs_tolerance,
            rel_tolerance,
        ) && approx_scalar_f64(
            self.imaginary,
            other.imaginary,
            abs_tolerance,
            rel_tolerance,
        )
    }

    fn try_from_polar(
        radius: Self::Real,
        angle: Self::Real,
    ) -> Result<Self, ComplexError> {
        if !radius.is_finite() || !angle.is_finite() {
            return Err(ComplexError::NonFinitePolarCoordinate);
        }

        if radius < 0.0 {
            return Err(ComplexError::NegativeRadius);
        }

        let result = Self {
            real: radius * angle.cos(),
            imaginary: radius * angle.sin(),
        };

        if !result.is_finite() {
            return Err(ComplexError::ConversionOverflow);
        }

        Ok(result)
    }

    fn from_real(value: Self::Real) -> Result<Self, ComplexError> {
        Self::try_new(value, 0.0)
    }
}

// =============================================================================
// Arithmetic
// =============================================================================

impl Add for Complex32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl AddAssign for Complex32 {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imaginary += rhs.imaginary;
    }
}

impl Sub for Complex32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl SubAssign for Complex32 {
    fn sub_assign(&mut self, rhs: Self) {
        self.real -= rhs.real;
        self.imaginary -= rhs.imaginary;
    }
}

impl Mul for Complex32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real
                - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary
                + self.imaginary * rhs.real,
        }
    }
}

impl MulAssign for Complex32 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Complex32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denominator = rhs.norm_squared();

        Self {
            real: (self.real * rhs.real
                + self.imaginary * rhs.imaginary)
                / denominator,
            imaginary: (self.imaginary * rhs.real
                - self.real * rhs.imaginary)
                / denominator,
        }
    }
}

impl DivAssign for Complex32 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for Complex32 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl Add for Complex64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl AddAssign for Complex64 {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imaginary += rhs.imaginary;
    }
}

impl Sub for Complex64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl SubAssign for Complex64 {
    fn sub_assign(&mut self, rhs: Self) {
        self.real -= rhs.real;
        self.imaginary -= rhs.imaginary;
    }
}

impl Mul for Complex64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real
                - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary
                + self.imaginary * rhs.real,
        }
    }
}

impl MulAssign for Complex64 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Complex64 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denominator = rhs.norm_squared();

        Self {
            real: (self.real * rhs.real
                + self.imaginary * rhs.imaginary)
                / denominator,
            imaginary: (self.imaginary * rhs.real
                - self.real * rhs.imaginary)
                / denominator,
        }
    }
}

impl DivAssign for Complex64 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for Complex64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

// =============================================================================
// Scalar multiplication
// =============================================================================

impl Mul<f32> for Complex32 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            real: self.real * rhs,
            imaginary: self.imaginary * rhs,
        }
    }
}

impl MulAssign<f32> for Complex32 {
    fn mul_assign(&mut self, rhs: f32) {
        self.real *= rhs;
        self.imaginary *= rhs;
    }
}

impl Div<f32> for Complex32 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            real: self.real / rhs,
            imaginary: self.imaginary / rhs,
        }
    }
}

impl DivAssign<f32> for Complex32 {
    fn div_assign(&mut self, rhs: f32) {
        self.real /= rhs;
        self.imaginary /= rhs;
    }
}

impl Mul<f64> for Complex64 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            real: self.real * rhs,
            imaginary: self.imaginary * rhs,
        }
    }
}

impl MulAssign<f64> for Complex64 {
    fn mul_assign(&mut self, rhs: f64) {
        self.real *= rhs;
        self.imaginary *= rhs;
    }
}

impl Div<f64> for Complex64 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            real: self.real / rhs,
            imaginary: self.imaginary / rhs,
        }
    }
}

impl DivAssign<f64> for Complex64 {
    fn div_assign(&mut self, rhs: f64) {
        self.real /= rhs;
        self.imaginary /= rhs;
    }
}

// =============================================================================
// Iterator support
// =============================================================================

impl Sum for Complex32 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |accumulator, value| accumulator + value)
    }
}

impl<'a> Sum<&'a Complex32> for Complex32 {
    fn sum<I: Iterator<Item = &'a Complex32>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |accumulator, value| accumulator + *value)
    }
}

impl Product for Complex32 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |accumulator, value| accumulator * value)
    }
}

impl<'a> Product<&'a Complex32> for Complex32 {
    fn product<I: Iterator<Item = &'a Complex32>>(iter: I) -> Self {
        iter.fold(Self::ONE, |accumulator, value| accumulator * *value)
    }
}

impl Sum for Complex64 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |accumulator, value| accumulator + value)
    }
}

impl<'a> Sum<&'a Complex64> for Complex64 {
    fn sum<I: Iterator<Item = &'a Complex64>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |accumulator, value| accumulator + *value)
    }
}

impl Product for Complex64 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |accumulator, value| accumulator * value)
    }
}

impl<'a> Product<&'a Complex64> for Complex64 {
    fn product<I: Iterator<Item = &'a Complex64>>(iter: I) -> Self {
        iter.fold(Self::ONE, |accumulator, value| accumulator * *value)
    }
}

// =============================================================================
// Conversions
// =============================================================================

impl From<Complex32> for Complex64 {
    fn from(value: Complex32) -> Self {
        value.to_complex64()
    }
}

impl TryFrom<Complex64> for Complex32 {
    type Error = ComplexError;

    fn try_from(value: Complex64) -> Result<Self, Self::Error> {
        value.try_to_complex32()
    }
}

impl TryFrom<f32> for Complex32 {
    type Error = ComplexError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_new(value, 0.0)
    }
}

impl TryFrom<f64> for Complex64 {
    type Error = ComplexError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::try_new(value, 0.0)
    }
}

impl From<Complex32> for [f32; 2] {
    fn from(value: Complex32) -> Self {
        [value.real, value.imaginary]
    }
}

impl From<Complex64> for [f64; 2] {
    fn from(value: Complex64) -> Self {
        [value.real, value.imaginary]
    }
}

impl TryFrom<[f32; 2]> for Complex32 {
    type Error = ComplexError;

    fn try_from(value: [f32; 2]) -> Result<Self, Self::Error> {
        Self::try_new(value[0], value[1])
    }
}

impl TryFrom<[f64; 2]> for Complex64 {
    type Error = ComplexError;

    fn try_from(value: [f64; 2]) -> Result<Self, Self::Error> {
        Self::try_new(value[0], value[1])
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_are_correct() {
        assert_eq!(Complex32::ZERO, Complex32::new(0.0, 0.0));
        assert_eq!(Complex32::ONE, Complex32::new(1.0, 0.0));
        assert_eq!(Complex32::I, Complex32::new(0.0, 1.0));

        assert_eq!(Complex64::ZERO, Complex64::new(0.0, 0.0));
        assert_eq!(Complex64::ONE, Complex64::new(1.0, 0.0));
        assert_eq!(Complex64::I, Complex64::new(0.0, 1.0));
    }

    #[test]
    fn rejects_nan_and_infinity() {
        assert_eq!(
            Complex32::try_new(f32::NAN, 0.0),
            Err(ComplexError::NonFinite)
        );

        assert_eq!(
            Complex32::try_new(0.0, f32::INFINITY),
            Err(ComplexError::NonFinite)
        );

        assert_eq!(
            Complex64::try_new(f64::NAN, 0.0),
            Err(ComplexError::NonFinite)
        );

        assert_eq!(
            Complex64::try_new(0.0, f64::NEG_INFINITY),
            Err(ComplexError::NonFinite)
        );
    }

    #[test]
    fn arithmetic_is_correct_for_complex32() {
        let a = Complex32::new(1.0, 2.0);
        let b = Complex32::new(3.0, 4.0);

        assert_eq!(a + b, Complex32::new(4.0, 6.0));
        assert_eq!(b - a, Complex32::new(2.0, 2.0));
        assert_eq!(
            a * b,
            Complex32::new(-5.0, 10.0)
        );
    }

    #[test]
    fn arithmetic_is_correct_for_complex64() {
        let a = Complex64::new(1.0, 2.0);
        let b = Complex64::new(3.0, 4.0);

        assert_eq!(a + b, Complex64::new(4.0, 6.0));
        assert_eq!(b - a, Complex64::new(2.0, 2.0));
        assert_eq!(
            a * b,
            Complex64::new(-5.0, 10.0)
        );
    }

    #[test]
    fn conjugation_is_correct() {
        let value = Complex64::new(3.0, -4.0);

        assert_eq!(
            value.conjugate(),
            Complex64::new(3.0, 4.0)
        );
    }

    #[test]
    fn norm_squared_is_correct() {
        let value = Complex64::new(3.0, 4.0);

        assert_eq!(value.norm_squared(), 25.0);
        assert_eq!(value.magnitude(), 5.0);
    }

    #[test]
    fn normalization_is_correct() {
        let value = Complex64::new(3.0, 4.0);

        let normalized = value
            .checked_normalize()
            .expect("3 + 4i must normalize");

        assert!(
            normalized.approx_eq(
                Complex64::new(0.6, 0.8),
                DEFAULT_F64_ABS_TOLERANCE,
                DEFAULT_F64_REL_TOLERANCE,
            )
        );

        assert!(
            (normalized.magnitude() - 1.0).abs()
                <= DEFAULT_F64_ABS_TOLERANCE
        );
    }

    #[test]
    fn zero_cannot_be_normalized() {
        assert_eq!(
            Complex64::ZERO.checked_normalize(),
            Err(ComplexError::CannotNormalizeZero)
        );
    }

    #[test]
    fn reciprocal_is_correct() {
        let value = Complex64::new(3.0, 4.0);

        let reciprocal = value
            .checked_recip()
            .expect("non-zero complex value must have reciprocal");

        assert!(
            reciprocal.approx_eq(
                Complex64::new(0.12, -0.16),
                DEFAULT_F64_ABS_TOLERANCE,
                DEFAULT_F64_REL_TOLERANCE,
            )
        );
    }

    #[test]
    fn division_is_correct() {
        let numerator = Complex64::new(1.0, 2.0);
        let denominator = Complex64::new(3.0, 4.0);

        let result = numerator
            .checked_div(denominator)
            .expect("non-zero denominator must divide");

        assert!(
            result.approx_eq(
                Complex64::new(0.44, 0.08),
                DEFAULT_F64_ABS_TOLERANCE,
                DEFAULT_F64_REL_TOLERANCE,
            )
        );
    }

    #[test]
    fn division_by_zero_is_rejected() {
        assert_eq!(
            Complex64::ONE.checked_div(Complex64::ZERO),
            Err(ComplexError::DivisionByZero)
        );
    }

    #[test]
    fn reciprocal_of_zero_is_rejected() {
        assert_eq!(
            Complex64::ZERO.checked_recip(),
            Err(ComplexError::DivisionByZero)
        );
    }

    #[test]
    fn polar_conversion_is_correct() {
        let value = Complex64::try_from_polar(
            1.0,
            core::f64::consts::FRAC_PI_2,
        )
        .expect("unit polar coordinate must be valid");

        assert!(
            value.approx_eq(
                Complex64::I,
                1.0e-12,
                1.0e-10,
            )
        );
    }

    #[test]
    fn negative_polar_radius_is_rejected() {
        assert_eq!(
            Complex64::try_from_polar(-1.0, 0.0),
            Err(ComplexError::NegativeRadius)
        );
    }

    #[test]
    fn non_finite_polar_coordinates_are_rejected() {
        assert_eq!(
            Complex64::try_from_polar(f64::NAN, 0.0),
            Err(ComplexError::NonFinitePolarCoordinate)
        );

        assert_eq!(
            Complex64::try_from_polar(1.0, f64::INFINITY),
            Err(ComplexError::NonFinitePolarCoordinate)
        );
    }

    #[test]
    fn approximate_equality_works() {
        let a = Complex64::new(1.0, 2.0);
        let b = Complex64::new(
            1.0 + 1.0e-13,
            2.0 - 1.0e-13,
        );

        assert!(a.approx_eq(b, 1.0e-12, 1.0e-10));
        assert!(!a.approx_eq(b, 1.0e-15, 1.0e-15));
    }

    #[test]
    fn precision_conversion_is_explicit() {
        let value64 = Complex64::new(1.5, -2.5);

        let value32 = value64
            .try_to_complex32()
            .expect("representable f64 value must convert");

        assert_eq!(value32, Complex32::new(1.5, -2.5));

        let round_trip = value32.to_complex64();

        assert_eq!(round_trip, value64);
    }

    #[test]
    fn precision_conversion_rejects_overflow() {
        let value = Complex64::new(f64::MAX, 0.0);

        assert_eq!(
            value.try_to_complex32(),
            Err(ComplexError::ConversionNonFinite)
        );
    }

    #[test]
    fn array_conversion_is_deterministic() {
        let value = Complex64::new(2.0, -3.0);

        let array: [f64; 2] = value.into();

        assert_eq!(array, [2.0, -3.0]);

        let restored =
            Complex64::try_from(array).expect("finite array must convert");

        assert_eq!(restored, value);
    }

    #[test]
    fn iterator_sum_is_correct() {
        let values = [
            Complex64::new(1.0, 2.0),
            Complex64::new(3.0, 4.0),
            Complex64::new(-2.0, 1.0),
        ];

        let sum: Complex64 = values.into_iter().sum();

        assert_eq!(sum, Complex64::new(2.0, 7.0));
    }

    #[test]
    fn iterator_product_is_correct() {
        let values = [
            Complex64::new(1.0, 2.0),
            Complex64::new(3.0, 4.0),
        ];

        let product: Complex64 = values.into_iter().product();

        assert_eq!(
            product,
            Complex64::new(-5.0, 10.0)
        );
    }

    #[test]
    fn scalar_multiplication_is_correct() {
        let value = Complex64::new(2.0, -3.0);

        assert_eq!(
            value * 2.0,
            Complex64::new(4.0, -6.0)
        );

        assert_eq!(
            value / 2.0,
            Complex64::new(1.0, -1.5)
        );
    }

    #[test]
    fn display_is_stable() {
        assert_eq!(
            Complex64::new(2.0, 3.0).to_string(),
            "2 + 3i"
        );

        assert_eq!(
            Complex64::new(2.0, -3.0).to_string(),
            "2 - 3i"
        );
    }

    #[test]
    fn trait_constants_match_concrete_constants() {
        assert_eq!(
            <Complex32 as ComplexScalar>::zero(),
            Complex32::ZERO
        );

        assert_eq!(
            <Complex32 as ComplexScalar>::one(),
            Complex32::ONE
        );

        assert_eq!(
            <Complex32 as ComplexScalar>::i(),
            Complex32::I
        );

        assert_eq!(
            <Complex64 as ComplexScalar>::zero(),
            Complex64::ZERO
        );

        assert_eq!(
            <Complex64 as ComplexScalar>::one(),
            Complex64::ONE
        );

        assert_eq!(
            <Complex64 as ComplexScalar>::i(),
            Complex64::I
        );
    }

    #[test]
    fn byte_sizes_are_correct() {
        assert_eq!(
            <Complex32 as ComplexScalar>::BYTE_SIZE,
            8
        );

        assert_eq!(
            <Complex64 as ComplexScalar>::BYTE_SIZE,
            16
        );
    }

    #[test]
    fn precision_metadata_is_correct() {
        assert_eq!(
            <Complex32 as ComplexScalar>::REAL_BITS,
            32
        );

        assert_eq!(
            <Complex64 as ComplexScalar>::REAL_BITS,
            64
        );
    }

    #[test]
    fn zero_detection_is_exact() {
        assert!(Complex64::ZERO.is_zero());
        assert!(!Complex64::ONE.is_zero());
        assert!(!Complex64::I.is_zero());
    }

    #[test]
    fn finite_detection_is_correct() {
        assert!(Complex64::new(1.0, -2.0).is_finite());
        assert!(!Complex64 {
            real: f64::NAN,
            imaginary: 0.0,
        }
        .is_finite());
    }

    #[test]
    fn nan_detection_is_correct() {
        assert!(!Complex64::new(1.0, 2.0).is_nan());

        assert!(
            Complex64 {
                real: f64::NAN,
                imaginary: 0.0,
            }
            .is_nan()
        );
    }
}