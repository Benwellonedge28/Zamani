//! Zamani Quantum Memory — Numerical Policy and Validation
//!
//! Production-grade, provider-neutral numerical policy for
//! `crate::quantum::memory`.
//!
//! # Responsibility
//!
//! This module owns numerical policy and validation for the quantum-memory
//! subsystem. It does NOT own complex-number representation; that responsibility
//! belongs to [`super::complex`].
//!
//! This module provides:
//!
//! - scalar precision identification;
//! - absolute/relative tolerance policy;
//! - probability tolerance;
//! - normalization tolerance;
//! - unitarity tolerance;
//! - Hermiticity tolerance;
//! - trace tolerance;
//! - finite-value validation;
//! - non-negative-value validation;
//! - probability validation;
//! - normalization validation;
//! - approximate equality;
//! - approximate zero/one checks;
//! - safe probability clamping;
//! - precision-aware conversion checks;
//! - numerical accumulation helpers;
//! - deterministic numerical policy;
//! - fidelity/error bounds;
//! - phase/amplitude validation;
//! - stable numerical contracts for CPU, SIMD, GPU and distributed execution.
//!
//! # Architectural boundary
//!
//! ```text
//! quantum::memory
//!       |
//!       +--> complex.rs
//!       |      |
//!       |      +--> Complex32
//!       |      +--> Complex64
//!       |      +--> ComplexScalar
//!       |
//!       +--> numeric.rs  <--- this module
//!              |
//!              +--> tolerance policy
//!              +--> validation
//!              +--> finite checks
//!              +--> probability checks
//!              +--> normalization checks
//!              +--> numerical invariants
//!              |
//!              +-----------------------------+
//!              |                             |
//!              v                             v
//!        state representations          accelerator layers
//!        state_vector                    CPU
//!        density_matrix                  SIMD
//!        stabilizer                      GPU
//!        sparse                          distributed
//!        tensor_network
//! ```
//!
//! # Critical ownership rule
//!
//! `complex.rs` owns the representation and arithmetic of complex values.
//!
//! `numeric.rs` owns numerical *policy*.
//!
//! Therefore this module must NOT introduce another `Complex32`,
//! `Complex64`, or competing complex arithmetic implementation.
//!
//! # Precision support
//!
//! The numerical policy supports the scalar precisions currently exposed by
//! `memory::complex`:
//!
//! - `f32` / `Complex32`;
//! - `f64` / `Complex64`.
//!
//! The API is intentionally designed so additional precision types can be
//! introduced later without changing the meaning of the existing policy.
//!
//! # Hardware neutrality
//!
//! This module contains no assumptions about:
//!
//! - CPU architecture;
//! - AVX;
//! - AVX2;
//! - AVX-512;
//! - NEON;
//! - CUDA;
//! - HIP;
//! - Metal;
//! - Vulkan;
//! - SYCL;
//! - FPGA;
//! - QPU vendor;
//! - simulator vendor;
//! - distributed-memory protocol.
//!
//! Numerical policy is a semantic contract. Hardware implementations must
//! preserve this contract regardless of where calculations execute.
//!
//! # Determinism
//!
//! A [`NumericPolicy`] is immutable and contains all numerical decisions needed
//! by the consuming operation. This avoids hidden global floating-point policy.
//!
//! Identical inputs, precision, policy, and algorithmic ordering must produce
//! semantically equivalent results within the declared tolerance.
//!
//! Hardware kernels may use different implementations, but they must not
//! silently change the configured numerical guarantees.
//!
//! # No-reedit contract
//!
//! Later modules must use this file for numerical policy instead of introducing
//! their own tolerance constants.
//!
//! In particular, later modules must NOT scatter values such as:
//!
//! ```text
//! 1e-10
//! 1e-12
//! 1e-6
//! ```
//!
//! throughout the memory subsystem.
//!
//! If a future representation requires a genuinely different tolerance model,
//! that model must be represented as an explicit [`NumericPolicy`] value or a
//! future versioned extension rather than an unrelated local constant.
//!
//! # Safety
//!
//! This module is safe Rust.
//!
//! - no `unsafe`;
//! - no raw pointers;
//! - no global mutable state;
//! - no hidden allocation;
//! - no I/O;
//! - no backend communication;
//! - no vendor dependencies.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features.
//!
//! # Integration contract
//!
//! ## `complex.rs`
//!
//! `numeric.rs` consumes [`super::complex::ComplexScalar`] and does not
//! redefine complex arithmetic.
//!
//! ## `limits.rs`
//!
//! `limits.rs` owns memory-resource limits. Numerical policy does not decide
//! whether an allocation is affordable.
//!
//! ## `state_vector.rs`
//!
//! Uses this module for normalization, probability and numerical comparison.
//!
//! ## `density_matrix.rs`
//!
//! Uses this module for trace, Hermiticity, positivity tolerances and numerical
//! comparison.
//!
//! ## `stabilizer.rs`
//!
//! Uses exact/discrete semantics where appropriate and may use this module only
//! for numerical interfaces that actually require floating-point values.
//!
//! ## `sparse.rs`
//!
//! Uses [`NumericPolicy::sparse_zero_tolerance`] when deciding whether an
//! amplitude is numerically negligible. Such removal must always be an explicit
//! policy decision.
//!
//! ## `tensor_network.rs`
//!
//! Uses truncation, normalization and fidelity tolerances from this module.
//!
//! ## `measurement.rs` / `collapse.rs`
//!
//! Uses probability and normalization tolerances. Randomness itself does NOT
//! belong here.
//!
//! ## `gpu.rs` / `simd.rs`
//!
//! Hardware kernels must preserve the policy supplied by the caller.
//!
//! ## `distributed.rs`
//!
//! Distributed reductions must use explicit accumulation policy and must not
//! silently change numerical semantics because values crossed a node boundary.
//!
//! ## `serialization.rs`
//!
//! Numerical policy metadata may be persisted when required for reproducibility.
//!
//! ## `snapshot.rs` / `checkpoint.rs`
//!
//! Restored execution must validate numerical-policy compatibility before
//! committing state.
//!
//! # Important distinction
//!
//! A tolerance is NOT permission to accept arbitrary invalid quantum states.
//!
//! In particular:
//!
//! - NaN is never accepted;
//! - infinity is never accepted;
//! - negative probability is never accepted merely because it is "small";
//! - normalization failure is not silently repaired unless the caller
//!   explicitly requests a documented normalization operation.
//!
//! Tiny floating-point artifacts may be handled by explicit policy methods such
//! as [`NumericPolicy::clamp_probability`].

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;

use super::complex::{
    Complex32,
    Complex64,
    ComplexScalar,
    DEFAULT_F32_ABS_TOLERANCE,
    DEFAULT_F32_REL_TOLERANCE,
    DEFAULT_F64_ABS_TOLERANCE,
    DEFAULT_F64_REL_TOLERANCE,
};

/// Machine-readable identifier for this numerical contract.
pub const NUMERIC_SCHEMA_ID: &str = "zamani.quantum.memory.numeric";

/// Semantic version of the numerical contract.
///
/// Increment this when the meaning of numerical-policy fields or validation
/// semantics changes.
pub const NUMERIC_SCHEMA_VERSION: u16 = 1;

/// Default sparse-state zero tolerance for double precision.
pub const DEFAULT_F64_SPARSE_ZERO_TOLERANCE: f64 = 1.0e-14;

/// Default sparse-state zero tolerance for single precision.
pub const DEFAULT_F32_SPARSE_ZERO_TOLERANCE: f32 = 1.0e-6;

/// Default probability tolerance for double precision.
pub const DEFAULT_F64_PROBABILITY_TOLERANCE: f64 = 1.0e-12;

/// Default probability tolerance for single precision.
pub const DEFAULT_F32_PROBABILITY_TOLERANCE: f32 = 1.0e-6;

/// Default normalization tolerance for double precision.
pub const DEFAULT_F64_NORMALIZATION_TOLERANCE: f64 = 1.0e-10;

/// Default normalization tolerance for single precision.
pub const DEFAULT_F32_NORMALIZATION_TOLERANCE: f32 = 1.0e-5;

/// Default unitarity tolerance for double precision.
pub const DEFAULT_F64_UNITARITY_TOLERANCE: f64 = 1.0e-10;

/// Default unitarity tolerance for single precision.
pub const DEFAULT_F32_UNITARITY_TOLERANCE: f32 = 1.0e-5;

/// Default Hermiticity tolerance for double precision.
pub const DEFAULT_F64_HERMITICITY_TOLERANCE: f64 = 1.0e-10;

/// Default Hermiticity tolerance for single precision.
pub const DEFAULT_F32_HERMITICITY_TOLERANCE: f32 = 1.0e-5;

/// Default trace tolerance for double precision.
pub const DEFAULT_F64_TRACE_TOLERANCE: f64 = 1.0e-10;

/// Default trace tolerance for single precision.
pub const DEFAULT_F32_TRACE_TOLERANCE: f32 = 1.0e-5;

/// Numerical precision supported by the memory subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Precision {
    /// IEEE-754 binary32.
    F32,

    /// IEEE-754 binary64.
    F64,
}

impl Precision {
    /// Returns the number of bits in one real scalar.
    pub const fn real_bits(self) -> u16 {
        match self {
            Self::F32 => 32,
            Self::F64 => 64,
        }
    }

    /// Returns the number of bytes in one real scalar.
    pub const fn real_bytes(self) -> usize {
        match self {
            Self::F32 => 4,
            Self::F64 => 8,
        }
    }

    /// Returns the number of bytes in one complex scalar.
    pub const fn complex_bytes(self) -> usize {
        self.real_bytes() * 2
    }

    /// Returns whether this is single precision.
    pub const fn is_f32(self) -> bool {
        matches!(self, Self::F32)
    }

    /// Returns whether this is double precision.
    pub const fn is_f64(self) -> bool {
        matches!(self, Self::F64)
    }
}

impl fmt::Display for Precision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::F32 => formatter.write_str("f32"),
            Self::F64 => formatter.write_str("f64"),
        }
    }
}

/// Errors produced by numerical validation and policy operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericError {
    /// A supplied value was NaN or infinite.
    NonFinite,

    /// A probability was negative.
    NegativeProbability,

    /// A probability exceeded one beyond the configured tolerance.
    ProbabilityAboveOne,

    /// A collection of probabilities did not sum to one within tolerance.
    ProbabilityNormalizationFailed,

    /// A normalization value was invalid or too far from one.
    NormalizationFailed,

    /// A normalization denominator was zero.
    ZeroNorm,

    /// A supplied tolerance was invalid.
    InvalidTolerance,

    /// A relative tolerance was negative or otherwise invalid.
    InvalidRelativeTolerance,

    /// An absolute tolerance was negative or otherwise invalid.
    InvalidAbsoluteTolerance,

    /// A requested precision conversion cannot preserve finite values.
    PrecisionConversionFailed,

    /// A value could not be represented under the requested policy.
    PrecisionPolicyViolation,

    /// A squared magnitude overflowed to infinity.
    MagnitudeOverflow,

    /// A numerical comparison was requested with incompatible precision.
    PrecisionMismatch,

    /// An accumulation operation encountered an invalid value.
    AccumulationNonFinite,

    /// A matrix trace was outside the configured trace tolerance.
    TraceMismatch,

    /// A matrix failed Hermiticity validation.
    NotHermitian,

    /// A matrix failed a unitarity validation.
    NotUnitary,

    /// A value that must be non-negative violated the configured constraint.
    NegativeValue,

    /// A tolerance-based operation could not safely clamp the value.
    ClampFailed,
}

impl fmt::Display for NumericError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => {
                formatter.write_str("numerical value contains NaN or infinity")
            }
            Self::NegativeProbability => {
                formatter.write_str("probability cannot be negative")
            }
            Self::ProbabilityAboveOne => {
                formatter.write_str("probability exceeds one beyond the configured tolerance")
            }
            Self::ProbabilityNormalizationFailed => {
                formatter.write_str("probabilities do not sum to one within the configured tolerance")
            }
            Self::NormalizationFailed => {
                formatter.write_str("normalization condition failed")
            }
            Self::ZeroNorm => {
                formatter.write_str("normalization denominator is zero")
            }
            Self::InvalidTolerance => {
                formatter.write_str("numerical tolerance is invalid")
            }
            Self::InvalidRelativeTolerance => {
                formatter.write_str("relative tolerance is invalid")
            }
            Self::InvalidAbsoluteTolerance => {
                formatter.write_str("absolute tolerance is invalid")
            }
            Self::PrecisionConversionFailed => {
                formatter.write_str("precision conversion failed")
            }
            Self::PrecisionPolicyViolation => {
                formatter.write_str("value violates the selected precision policy")
            }
            Self::MagnitudeOverflow => {
                formatter.write_str("magnitude calculation overflowed")
            }
            Self::PrecisionMismatch => {
                formatter.write_str("numerical values use incompatible precision")
            }
            Self::AccumulationNonFinite => {
                formatter.write_str("numerical accumulation produced a non-finite value")
            }
            Self::TraceMismatch => {
                formatter.write_str("matrix trace is outside the configured tolerance")
            }
            Self::NotHermitian => {
                formatter.write_str("matrix is not Hermitian within the configured tolerance")
            }
            Self::NotUnitary => {
                formatter.write_str("matrix is not unitary within the configured tolerance")
            }
            Self::NegativeValue => {
                formatter.write_str("value must not be negative")
            }
            Self::ClampFailed => {
                formatter.write_str("value could not be safely clamped")
            }
        }
    }
}

impl std::error::Error for NumericError {}

/// Numerical policy used by a quantum-memory operation.
///
/// The policy is immutable and self-contained. It can therefore safely be
/// passed through CPU, SIMD, GPU and distributed execution boundaries without
/// requiring global state.
///
/// The fields intentionally use `f64` so the policy itself has one stable
/// representation. Consumers operating in `f32` convert the policy through
/// [`NumericPolicy::for_precision`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumericPolicy {
    /// Numerical precision selected by the caller.
    precision: Precision,

    /// Absolute comparison tolerance.
    absolute_tolerance: f64,

    /// Relative comparison tolerance.
    relative_tolerance: f64,

    /// Tolerance used when validating probabilities.
    probability_tolerance: f64,

    /// Tolerance used when validating normalization.
    normalization_tolerance: f64,

    /// Tolerance used when validating unitary matrices.
    unitarity_tolerance: f64,

    /// Tolerance used when validating Hermitian matrices.
    hermiticity_tolerance: f64,

    /// Tolerance used when validating density-matrix trace.
    trace_tolerance: f64,

    /// Magnitude below which sparse amplitudes may be considered zero when
    /// explicit sparse thresholding is requested.
    sparse_zero_tolerance: f64,

    /// Whether tiny negative floating-point probability artifacts may be
    /// clamped to zero.
    allow_probability_clamping: bool,

    /// Whether values slightly above one may be clamped to one.
    allow_probability_upper_clamping: bool,
}

impl NumericPolicy {
    /// Creates the canonical double-precision policy.
    pub const fn f64_default() -> Self {
        Self {
            precision: Precision::F64,
            absolute_tolerance: DEFAULT_F64_ABS_TOLERANCE,
            relative_tolerance: DEFAULT_F64_REL_TOLERANCE,
            probability_tolerance: DEFAULT_F64_PROBABILITY_TOLERANCE,
            normalization_tolerance: DEFAULT_F64_NORMALIZATION_TOLERANCE,
            unitarity_tolerance: DEFAULT_F64_UNITARITY_TOLERANCE,
            hermiticity_tolerance: DEFAULT_F64_HERMITICITY_TOLERANCE,
            trace_tolerance: DEFAULT_F64_TRACE_TOLERANCE,
            sparse_zero_tolerance: DEFAULT_F64_SPARSE_ZERO_TOLERANCE,
            allow_probability_clamping: true,
            allow_probability_upper_clamping: true,
        }
    }

    /// Creates the canonical single-precision policy.
    pub const fn f32_default() -> Self {
        Self {
            precision: Precision::F32,
            absolute_tolerance: DEFAULT_F32_ABS_TOLERANCE as f64,
            relative_tolerance: DEFAULT_F32_REL_TOLERANCE as f64,
            probability_tolerance: DEFAULT_F32_PROBABILITY_TOLERANCE as f64,
            normalization_tolerance: DEFAULT_F32_NORMALIZATION_TOLERANCE as f64,
            unitarity_tolerance: DEFAULT_F32_UNITARITY_TOLERANCE as f64,
            hermiticity_tolerance: DEFAULT_F32_HERMITICITY_TOLERANCE as f64,
            trace_tolerance: DEFAULT_F32_TRACE_TOLERANCE as f64,
            sparse_zero_tolerance: DEFAULT_F32_SPARSE_ZERO_TOLERANCE as f64,
            allow_probability_clamping: true,
            allow_probability_upper_clamping: true,
        }
    }

    /// Creates the default policy for the requested precision.
    pub const fn for_precision(precision: Precision) -> Self {
        match precision {
            Precision::F32 => Self::f32_default(),
            Precision::F64 => Self::f64_default(),
        }
    }

    /// Returns the selected precision.
    pub const fn precision(&self) -> Precision {
        self.precision
    }

    /// Returns the absolute tolerance.
    pub const fn absolute_tolerance(&self) -> f64 {
        self.absolute_tolerance
    }

    /// Returns the relative tolerance.
    pub const fn relative_tolerance(&self) -> f64 {
        self.relative_tolerance
    }

    /// Returns the probability tolerance.
    pub const fn probability_tolerance(&self) -> f64 {
        self.probability_tolerance
    }

    /// Returns the normalization tolerance.
    pub const fn normalization_tolerance(&self) -> f64 {
        self.normalization_tolerance
    }

    /// Returns the unitarity tolerance.
    pub const fn unitarity_tolerance(&self) -> f64 {
        self.unitarity_tolerance
    }

    /// Returns the Hermiticity tolerance.
    pub const fn hermiticity_tolerance(&self) -> f64 {
        self.hermiticity_tolerance
    }

    /// Returns the trace tolerance.
    pub const fn trace_tolerance(&self) -> f64 {
        self.trace_tolerance
    }

    /// Returns the sparse-state zero tolerance.
    pub const fn sparse_zero_tolerance(&self) -> f64 {
        self.sparse_zero_tolerance
    }

    /// Returns whether lower probability clamping is enabled.
    pub const fn allows_probability_clamping(&self) -> bool {
        self.allow_probability_clamping
    }

    /// Returns whether upper probability clamping is enabled.
    pub const fn allows_probability_upper_clamping(&self) -> bool {
        self.allow_probability_upper_clamping
    }

    /// Validates the policy itself.
    pub fn validate(&self) -> Result<(), NumericError> {
        validate_tolerance(self.absolute_tolerance)?;
        validate_tolerance(self.relative_tolerance)?;
        validate_tolerance(self.probability_tolerance)?;
        validate_tolerance(self.normalization_tolerance)?;
        validate_tolerance(self.unitarity_tolerance)?;
        validate_tolerance(self.hermiticity_tolerance)?;
        validate_tolerance(self.trace_tolerance)?;
        validate_tolerance(self.sparse_zero_tolerance)?;

        Ok(())
    }

    /// Returns a policy with a different absolute tolerance.
    pub fn with_absolute_tolerance(
        mut self,
        tolerance: f64,
    ) -> Result<Self, NumericError> {
        validate_tolerance(tolerance)?;
        self.absolute_tolerance = tolerance;
        Ok(self)
    }

    /// Returns a policy with a different relative tolerance.
    pub fn with_relative_tolerance(
        mut self,
        tolerance: f64,
    ) -> Result<Self, NumericError> {
        validate_tolerance(tolerance)?;
        self.relative_tolerance = tolerance;
        Ok(self)
    }

    /// Returns a policy with a different probability tolerance.
    pub fn with_probability_tolerance(
        mut self,
        tolerance: f64,
    ) -> Result<Self, NumericError> {
        validate_tolerance(tolerance)?;
        self.probability_tolerance = tolerance;
        Ok(self)
    }

    /// Returns a policy with a different normalization tolerance.
    pub fn with_normalization_tolerance(
        mut self,
        tolerance: f64,
    ) -> Result<Self, NumericError> {
        validate_tolerance(tolerance)?;
        self.normalization_tolerance = tolerance;
        Ok(self)
    }

    /// Returns a policy with a different unitarity tolerance.
    pub fn with_unitarity_tolerance(
        mut self,
        tolerance: f64,
    ) -> Result<Self, NumericError> {
        validate_tolerance(tolerance)?;
        self.unitarity_tolerance = tolerance;
        Ok(self)
    }

    /// Returns a policy with a different Hermiticity tolerance.
    pub fn with_hermiticity_tolerance(
        mut self,
        tolerance: f64,
    ) -> Result<Self, NumericError> {
        validate_tolerance(tolerance)?;
        self.hermiticity_tolerance = tolerance;
        Ok(self)
    }

    /// Returns a policy with a different trace tolerance.
    pub fn with_trace_tolerance(
        mut self,
        tolerance: f64,
    ) -> Result<Self, NumericError> {
        validate_tolerance(tolerance)?;
        self.trace_tolerance = tolerance;
        Ok(self)
    }

    /// Returns a policy with a different sparse threshold.
    pub fn with_sparse_zero_tolerance(
        mut self,
        tolerance: f64,
    ) -> Result<Self, NumericError> {
        validate_tolerance(tolerance)?;
        self.sparse_zero_tolerance = tolerance;
        Ok(self)
    }

    /// Enables or disables lower probability clamping.
    pub const fn with_probability_clamping(mut self, enabled: bool) -> Self {
        self.allow_probability_clamping = enabled;
        self
    }

    /// Enables or disables upper probability clamping.
    pub const fn with_probability_upper_clamping(mut self, enabled: bool) -> Self {
        self.allow_probability_upper_clamping = enabled;
        self
    }

    /// Tests approximate equality using the policy's general tolerances.
    pub fn approx_eq_f64(&self, lhs: f64, rhs: f64) -> Result<bool, NumericError> {
        validate_finite(lhs)?;
        validate_finite(rhs)?;

        Ok(approx_eq(
            lhs,
            rhs,
            self.absolute_tolerance,
            self.relative_tolerance,
        ))
    }

    /// Tests whether an f64 value is approximately zero.
    pub fn approx_zero_f64(&self, value: f64) -> Result<bool, NumericError> {
        validate_finite(value)?;

        Ok(value.abs() <= self.absolute_tolerance)
    }

    /// Tests whether an f64 value is approximately one.
    pub fn approx_one_f64(&self, value: f64) -> Result<bool, NumericError> {
        validate_finite(value)?;

        Ok(approx_eq(
            value,
            1.0,
            self.normalization_tolerance,
            self.relative_tolerance,
        ))
    }

    /// Validates a real scalar under the configured policy.
    pub fn validate_f64(&self, value: f64) -> Result<(), NumericError> {
        validate_finite(value)
    }

    /// Validates an f32 scalar under the configured policy.
    pub fn validate_f32(&self, value: f32) -> Result<(), NumericError> {
        validate_finite_f32(value)
    }

    /// Validates a complex scalar using the canonical complex contract.
    pub fn validate_complex64(&self, value: Complex64) -> Result<(), NumericError> {
        if value.is_finite() {
            Ok(())
        } else {
            Err(NumericError::NonFinite)
        }
    }

    /// Validates a single-precision complex scalar.
    pub fn validate_complex32(&self, value: Complex32) -> Result<(), NumericError> {
        if value.is_finite() {
            Ok(())
        } else {
            Err(NumericError::NonFinite)
        }
    }

    /// Compares two complex64 values using this policy.
    pub fn approx_eq_complex64(
        &self,
        lhs: Complex64,
        rhs: Complex64,
    ) -> Result<bool, NumericError> {
        self.validate_complex64(lhs)?;
        self.validate_complex64(rhs)?;

        Ok(lhs.approx_eq(
            rhs,
            self.absolute_tolerance,
            self.relative_tolerance,
        ))
    }

    /// Compares two complex32 values using this policy.
    pub fn approx_eq_complex32(
        &self,
        lhs: Complex32,
        rhs: Complex32,
    ) -> Result<bool, NumericError> {
        self.validate_complex32(lhs)?;
        self.validate_complex32(rhs)?;

        let abs_tolerance = self.absolute_tolerance as f32;
        let rel_tolerance = self.relative_tolerance as f32;

        Ok(lhs.approx_eq(rhs, abs_tolerance, rel_tolerance))
    }

    /// Validates a probability without modifying it.
    pub fn validate_probability_f64(
        &self,
        probability: f64,
    ) -> Result<(), NumericError> {
        validate_probability(
            probability,
            self.probability_tolerance,
        )
    }

    /// Validates an f32 probability.
    pub fn validate_probability_f32(
        &self,
        probability: f32,
    ) -> Result<(), NumericError> {
        validate_probability(
            probability as f64,
            self.probability_tolerance,
        )
    }

    /// Validates and optionally clamps a probability.
    ///
    /// Only tiny floating-point excursions within the configured probability
    /// tolerance may be corrected.
    ///
    /// Values such as `-0.1` are never silently converted to zero.
    pub fn clamp_probability_f64(
        &self,
        probability: f64,
    ) -> Result<f64, NumericError> {
        validate_finite(probability)?;

        if probability < 0.0 {
            if self.allow_probability_clamping
                && probability >= -self.probability_tolerance
            {
                return Ok(0.0);
            }

            return Err(NumericError::NegativeProbability);
        }

        if probability > 1.0 {
            if self.allow_probability_upper_clamping
                && probability <= 1.0 + self.probability_tolerance
            {
                return Ok(1.0);
            }

            return Err(NumericError::ProbabilityAboveOne);
        }

        Ok(probability)
    }

    /// Validates and optionally clamps an f32 probability.
    pub fn clamp_probability_f32(
        &self,
        probability: f32,
    ) -> Result<f32, NumericError> {
        Ok(self.clamp_probability_f64(probability as f64)? as f32)
    }

    /// Validates that a sequence of probabilities is valid and normalized.
    ///
    /// The input is consumed only for iteration and is not modified.
    pub fn validate_probabilities<I>(
        &self,
        probabilities: I,
    ) -> Result<f64, NumericError>
    where
        I: IntoIterator<Item = f64>,
    {
        let mut sum = 0.0_f64;

        for probability in probabilities {
            let probability = self.clamp_probability_f64(probability)?;

            sum += probability;

            if !sum.is_finite() {
                return Err(NumericError::AccumulationNonFinite);
            }
        }

        if !approx_eq(
            sum,
            1.0,
            self.probability_tolerance.max(self.normalization_tolerance),
            self.relative_tolerance,
        ) {
            return Err(NumericError::ProbabilityNormalizationFailed);
        }

        Ok(sum)
    }

    /// Validates that a value is normalized to one within the configured
    /// normalization tolerance.
    pub fn validate_normalized_f64(
        &self,
        norm: f64,
    ) -> Result<(), NumericError> {
        validate_finite(norm)?;

        if approx_eq(
            norm,
            1.0,
            self.normalization_tolerance,
            self.relative_tolerance,
        ) {
            Ok(())
        } else {
            Err(NumericError::NormalizationFailed)
        }
    }

    /// Validates an f32 normalization value.
    pub fn validate_normalized_f32(
        &self,
        norm: f32,
    ) -> Result<(), NumericError> {
        self.validate_normalized_f64(norm as f64)
    }

    /// Validates that a norm is usable for normalization.
    pub fn validate_nonzero_norm_f64(
        &self,
        norm: f64,
    ) -> Result<(), NumericError> {
        validate_finite(norm)?;

        if norm <= self.absolute_tolerance {
            Err(NumericError::ZeroNorm)
        } else {
            Ok(())
        }
    }

    /// Validates that a trace is approximately one.
    pub fn validate_trace(&self, trace: f64) -> Result<(), NumericError> {
        validate_finite(trace)?;

        if approx_eq(
            trace,
            1.0,
            self.trace_tolerance,
            self.relative_tolerance,
        ) {
            Ok(())
        } else {
            Err(NumericError::TraceMismatch)
        }
    }

    /// Returns the allowed sparse threshold in the selected precision.
    pub fn sparse_zero_tolerance_for_precision(&self) -> f64 {
        self.sparse_zero_tolerance
    }

    /// Returns the policy adjusted to the requested precision.
    ///
    /// This does not reinterpret the policy arbitrarily. It replaces the
    /// tolerance family with the canonical defaults for the requested
    /// precision.
    pub const fn canonical_for_precision(precision: Precision) -> Self {
        Self::for_precision(precision)
    }
}

/// Trait implemented by supported real scalar types.
///
/// This trait is intentionally small. It exists to allow numerical algorithms
/// to express their requirements without coupling themselves to a particular
/// accelerator or backend.
///
/// Implementations are provided only for the scalar types currently supported
/// by Zamani's canonical complex layer.
pub trait RealScalar:
    Copy
    + Clone
    + Send
    + Sync
    + PartialEq
    + PartialOrd
    + fmt::Debug
    + fmt::Display
{
    /// Numerical precision represented by this scalar.
    const PRECISION: Precision;

    /// Returns zero.
    fn zero() -> Self;

    /// Returns one.
    fn one() -> Self;

    /// Returns the absolute value.
    fn abs(self) -> Self;

    /// Returns whether the value is finite.
    fn is_finite(self) -> bool;

    /// Returns whether the value is NaN.
    fn is_nan(self) -> bool;

    /// Converts to f64 for policy calculations.
    fn to_f64(self) -> f64;

    /// Constructs the scalar from f64.
    ///
    /// The conversion must reject values that cannot be represented as finite
    /// values in the target precision.
    fn try_from_f64(value: f64) -> Result<Self, NumericError>;

    /// Returns true if values are approximately equal under the supplied
    /// tolerances.
    fn approx_eq(self, other: Self, abs_tolerance: Self, rel_tolerance: Self)
        -> bool;
}

impl RealScalar for f32 {
    const PRECISION: Precision = Precision::F32;

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn is_finite(self) -> bool {
        self.is_finite()
    }

    fn is_nan(self) -> bool {
        self.is_nan()
    }

    fn to_f64(self) -> f64 {
        self as f64
    }

    fn try_from_f64(value: f64) -> Result<Self, NumericError> {
        if !value.is_finite() {
            return Err(NumericError::PrecisionConversionFailed);
        }

        let converted = value as f32;

        if !converted.is_finite() {
            return Err(NumericError::PrecisionConversionFailed);
        }

        Ok(converted)
    }

    fn approx_eq(
        self,
        other: Self,
        abs_tolerance: Self,
        rel_tolerance: Self,
    ) -> bool {
        approx_eq(
            self as f64,
            other as f64,
            abs_tolerance as f64,
            rel_tolerance as f64,
        )
    }
}

impl RealScalar for f64 {
    const PRECISION: Precision = Precision::F64;

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn is_finite(self) -> bool {
        self.is_finite()
    }

    fn is_nan(self) -> bool {
        self.is_nan()
    }

    fn to_f64(self) -> f64 {
        self
    }

    fn try_from_f64(value: f64) -> Result<Self, NumericError> {
        if value.is_finite() {
            Ok(value)
        } else {
            Err(NumericError::PrecisionConversionFailed)
        }
    }

    fn approx_eq(
        self,
        other: Self,
        abs_tolerance: Self,
        rel_tolerance: Self,
    ) -> bool {
        approx_eq(self, other, abs_tolerance, rel_tolerance)
    }
}

/// Validates a tolerance.
///
/// A tolerance must be finite and non-negative.
pub fn validate_tolerance(value: f64) -> Result<(), NumericError> {
    if !value.is_finite() {
        return Err(NumericError::InvalidTolerance);
    }

    if value < 0.0 {
        return Err(NumericError::InvalidTolerance);
    }

    Ok(())
}

/// Validates an f64 value for quantum numerical use.
pub fn validate_finite(value: f64) -> Result<(), NumericError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(NumericError::NonFinite)
    }
}

/// Validates an f32 value.
pub fn validate_finite_f32(value: f32) -> Result<(), NumericError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(NumericError::NonFinite)
    }
}

/// Validates a probability in the mathematical interval [0, 1].
///
/// Unlike [`NumericPolicy::clamp_probability_f64`], this function never
/// modifies the supplied value.
pub fn validate_probability(
    probability: f64,
    tolerance: f64,
) -> Result<(), NumericError> {
    validate_finite(probability)?;
    validate_tolerance(tolerance)?;

    if probability < 0.0 {
        if probability >= -tolerance {
            return Ok(());
        }

        return Err(NumericError::NegativeProbability);
    }

    if probability > 1.0 + tolerance {
        return Err(NumericError::ProbabilityAboveOne);
    }

    Ok(())
}

/// Compares two finite f64 values using absolute and relative tolerance.
///
/// The comparison uses:
///
/// ```text
/// |a - b| <= max(abs_tolerance,
///                rel_tolerance * max(|a|, |b|))
/// ```
///
/// This is the canonical scalar comparison rule for this module.
pub fn approx_eq(
    lhs: f64,
    rhs: f64,
    abs_tolerance: f64,
    rel_tolerance: f64,
) -> bool {
    if !lhs.is_finite()
        || !rhs.is_finite()
        || !abs_tolerance.is_finite()
        || !rel_tolerance.is_finite()
        || abs_tolerance < 0.0
        || rel_tolerance < 0.0
    {
        return false;
    }

    let difference = (lhs - rhs).abs();

    if difference <= abs_tolerance {
        return true;
    }

    let scale = lhs.abs().max(rhs.abs());

    difference <= rel_tolerance * scale
}

/// Returns whether a finite value is approximately zero.
pub fn approx_zero(
    value: f64,
    tolerance: f64,
) -> Result<bool, NumericError> {
    validate_finite(value)?;
    validate_tolerance(tolerance)?;

    Ok(value.abs() <= tolerance)
}

/// Returns whether a finite value is approximately one.
pub fn approx_one(
    value: f64,
    tolerance: f64,
) -> Result<bool, NumericError> {
    validate_finite(value)?;
    validate_tolerance(tolerance)?;

    Ok((value - 1.0).abs() <= tolerance)
}

/// Computes a stable approximate relative error.
///
/// The denominator is the maximum magnitude of the two operands. If both
/// operands are zero, the relative error is zero.
pub fn relative_error(lhs: f64, rhs: f64) -> Result<f64, NumericError> {
    validate_finite(lhs)?;
    validate_finite(rhs)?;

    let denominator = lhs.abs().max(rhs.abs());

    if denominator == 0.0 {
        return Ok(0.0);
    }

    let result = (lhs - rhs).abs() / denominator;

    if result.is_finite() {
        Ok(result)
    } else {
        Err(NumericError::NonFinite)
    }
}

/// Computes absolute error.
pub fn absolute_error(lhs: f64, rhs: f64) -> Result<f64, NumericError> {
    validate_finite(lhs)?;
    validate_finite(rhs)?;

    let result = (lhs - rhs).abs();

    if result.is_finite() {
        Ok(result)
    } else {
        Err(NumericError::NonFinite)
    }
}

/// Computes a fidelity-like scalar from an overlap magnitude.
///
/// The value must already represent the appropriate physical overlap for the
/// caller's state representation.
///
/// This function only performs numerical validation and squaring.
pub fn overlap_fidelity(overlap_magnitude: f64) -> Result<f64, NumericError> {
    validate_finite(overlap_magnitude)?;

    if overlap_magnitude < 0.0 {
        return Err(NumericError::NegativeValue);
    }

    let fidelity = overlap_magnitude * overlap_magnitude;

    if !fidelity.is_finite() {
        return Err(NumericError::MagnitudeOverflow);
    }

    Ok(fidelity)
}

/// Validates a complex scalar generically.
pub fn validate_complex<S: ComplexScalar>(
    value: S,
) -> Result<(), NumericError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(NumericError::NonFinite)
    }
}

/// Returns the squared magnitude of a complex scalar with validation.
pub fn checked_norm_squared<S: ComplexScalar>(
    value: S,
) -> Result<S::Real, NumericError> {
    validate_complex(value)?;

    let result = value.norm_squared();

    if result.partial_cmp(&result).is_none() {
        return Err(NumericError::NonFinite);
    }

    Ok(result)
}

/// Returns the magnitude of a complex scalar with validation.
pub fn checked_magnitude<S: ComplexScalar>(
    value: S,
) -> Result<S::Real, NumericError> {
    validate_complex(value)?;

    let result = value.magnitude();

    if result.partial_cmp(&result).is_none() {
        return Err(NumericError::NonFinite);
    }

    Ok(result)
}

/// Validates that a generic complex value has finite magnitude.
pub fn validate_complex_magnitude<S: ComplexScalar>(
    value: S,
) -> Result<(), NumericError> {
    let magnitude = checked_magnitude(value)?;

    if magnitude.partial_cmp(&magnitude).is_none() {
        Err(NumericError::NonFinite)
    } else {
        Ok(())
    }
}

/// Safely converts an f64 value to f32.
pub fn try_f64_to_f32(value: f64) -> Result<f32, NumericError> {
    <f32 as RealScalar>::try_from_f64(value)
}

/// Converts an f32 value to f64.
///
/// Every finite f32 value is exactly representable as f64.
pub fn f32_to_f64(value: f32) -> Result<f64, NumericError> {
    validate_finite_f32(value)?;
    Ok(value as f64)
}

/// Safely converts Complex64 to Complex32.
///
/// This operation explicitly checks the destination components instead of
/// allowing a finite f64 value to become infinity after conversion.
pub fn try_complex64_to_complex32(
    value: Complex64,
) -> Result<Complex32, NumericError> {
    validate_complex(value)?;

    let real = try_f64_to_f32(value.real())?;
    let imaginary = try_f64_to_f32(value.imaginary())?;

    Complex32::try_new(real, imaginary)
        .map_err(|_| NumericError::PrecisionConversionFailed)
}

/// Converts Complex32 to Complex64.
///
/// This conversion is lossless for finite IEEE-754 binary32 values.
pub fn complex32_to_complex64(
    value: Complex32,
) -> Result<Complex64, NumericError> {
    validate_complex(value)?;

    Complex64::try_new(
        value.real() as f64,
        value.imaginary() as f64,
    )
    .map_err(|_| NumericError::PrecisionConversionFailed)
}

/// Validates the normalization of a collection of real amplitudes.
///
/// This function computes the sum of squared magnitudes and verifies that it is
/// approximately one.
pub fn validate_real_amplitude_norm<I>(
    amplitudes: I,
    policy: &NumericPolicy,
) -> Result<f64, NumericError>
where
    I: IntoIterator<Item = f64>,
{
    let mut norm_squared = 0.0_f64;

    for amplitude in amplitudes {
        validate_finite(amplitude)?;

        let contribution = amplitude * amplitude;

        if !contribution.is_finite() {
            return Err(NumericError::AccumulationNonFinite);
        }

        norm_squared += contribution;

        if !norm_squared.is_finite() {
            return Err(NumericError::AccumulationNonFinite);
        }
    }

    policy.validate_normalized_f64(norm_squared)?;

    Ok(norm_squared)
}

/// Validates the normalization of complex amplitudes.
///
/// This is representation-neutral and can therefore be used by dense, sparse
/// and tensor-network state implementations.
pub fn validate_complex_amplitude_norm<S, I>(
    amplitudes: I,
    policy: &NumericPolicy,
) -> Result<S::Real, NumericError>
where
    S: ComplexScalar,
    I: IntoIterator<Item = S>,
{
    let mut norm_squared = S::Real::zero();

    for amplitude in amplitudes {
        validate_complex(amplitude)?;

        let contribution = amplitude.norm_squared();

        if contribution.partial_cmp(&contribution).is_none() {
            return Err(NumericError::AccumulationNonFinite);
        }

        norm_squared = norm_squared + contribution;

        if norm_squared.partial_cmp(&norm_squared).is_none() {
            return Err(NumericError::AccumulationNonFinite);
        }
    }

    let norm = norm_squared
        .partial_cmp(&S::Real::one())
        .ok_or(NumericError::NonFinite)?;

    let _ = norm;

    let norm_as_f64 = real_to_f64(norm_squared)?;

    policy.validate_normalized_f64(norm_as_f64)?;

    Ok(norm_squared)
}

/// Converts a supported real scalar into f64.
pub fn real_to_f64<R: RealScalar>(
    value: R,
) -> Result<f64, NumericError> {
    if !value.is_finite() {
        return Err(NumericError::NonFinite);
    }

    let result = value.to_f64();

    validate_finite(result)?;

    Ok(result)
}

/// Checks whether a scalar is non-negative.
pub fn validate_non_negative(
    value: f64,
) -> Result<(), NumericError> {
    validate_finite(value)?;

    if value < 0.0 {
        Err(NumericError::NegativeValue)
    } else {
        Ok(())
    }
}

/// Checks whether a scalar is approximately non-negative.
///
/// This is useful for floating-point artifacts generated by mathematically
/// non-negative quantities such as probabilities or eigenvalue calculations.
pub fn validate_non_negative_with_tolerance(
    value: f64,
    tolerance: f64,
) -> Result<(), NumericError> {
    validate_finite(value)?;
    validate_tolerance(tolerance)?;

    if value >= 0.0 {
        return Ok(());
    }

    if value >= -tolerance {
        return Ok(());
    }

    Err(NumericError::NegativeValue)
}

/// Returns zero when a value is within the supplied tolerance.
///
/// This function is deliberately explicit; it does not silently modify values
/// elsewhere in the memory subsystem.
pub fn zero_if_small(
    value: f64,
    tolerance: f64,
) -> Result<f64, NumericError> {
    validate_finite(value)?;
    validate_tolerance(tolerance)?;

    if value.abs() <= tolerance {
        Ok(0.0)
    } else {
        Ok(value)
    }
}

/// Validates a unitary error measure.
///
/// `error` should be supplied by the caller after calculating its matrix
/// unitarity residual.
pub fn validate_unitarity_error(
    error: f64,
    policy: &NumericPolicy,
) -> Result<(), NumericError> {
    validate_finite(error)?;

    if error < 0.0 {
        return Err(NumericError::NegativeValue);
    }

    if error <= policy.unitarity_tolerance() {
        Ok(())
    } else {
        Err(NumericError::NotUnitary)
    }
}

/// Validates a Hermiticity residual.
pub fn validate_hermiticity_error(
    error: f64,
    policy: &NumericPolicy,
) -> Result<(), NumericError> {
    validate_finite(error)?;

    if error < 0.0 {
        return Err(NumericError::NegativeValue);
    }

    if error <= policy.hermiticity_tolerance() {
        Ok(())
    } else {
        Err(NumericError::NotHermitian)
    }
}

/// Validates a trace residual.
pub fn validate_trace_error(
    error: f64,
    policy: &NumericPolicy,
) -> Result<(), NumericError> {
    validate_finite(error)?;

    if error < 0.0 {
        return Err(NumericError::NegativeValue);
    }

    if error <= policy.trace_tolerance() {
        Ok(())
    } else {
        Err(NumericError::TraceMismatch)
    }
}

/// Accumulates finite f64 values.
///
/// This is intentionally simple and deterministic with respect to iteration
/// order. Callers requiring compensated summation should provide a deterministic
/// ordering and use [`CompensatedSum`].
pub fn checked_sum<I>(
    values: I,
) -> Result<f64, NumericError>
where
    I: IntoIterator<Item = f64>,
{
    let mut sum = 0.0_f64;

    for value in values {
        validate_finite(value)?;

        sum += value;

        if !sum.is_finite() {
            return Err(NumericError::AccumulationNonFinite);
        }
    }

    Ok(sum)
}

/// Deterministic compensated floating-point accumulator.
///
/// This implementation uses Neumaier-style compensation and does not require
/// unsafe code or external numerical crates.
///
/// It is useful for:
///
/// - probabilities;
/// - expectation values;
/// - norms;
/// - distributed reduction stages after deterministic ordering;
/// - benchmark statistics consumed by the memory layer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompensatedSum {
    sum: f64,
    compensation: f64,
}

impl CompensatedSum {
    /// Creates an empty accumulator.
    pub const fn new() -> Self {
        Self {
            sum: 0.0,
            compensation: 0.0,
        }
    }

    /// Adds a finite value.
    pub fn add(&mut self, value: f64) -> Result<(), NumericError> {
        validate_finite(value)?;

        let temporary = self.sum + value;

        if !temporary.is_finite() {
            return Err(NumericError::AccumulationNonFinite);
        }

        if self.sum.abs() >= value.abs() {
            self.compensation +=
                (self.sum - temporary) + value;
        } else {
            self.compensation +=
                (value - temporary) + self.sum;
        }

        self.sum = temporary;

        if !self.compensation.is_finite() {
            return Err(NumericError::AccumulationNonFinite);
        }

        Ok(())
    }

    /// Returns the compensated result.
    pub fn total(&self) -> Result<f64, NumericError> {
        let result = self.sum + self.compensation;

        if result.is_finite() {
            Ok(result)
        } else {
            Err(NumericError::AccumulationNonFinite)
        }
    }

    /// Returns the raw primary sum.
    pub const fn primary_sum(&self) -> f64 {
        self.sum
    }

    /// Returns the compensation term.
    pub const fn compensation(&self) -> f64 {
        self.compensation
    }

    /// Returns whether no values have materially been accumulated.
    pub fn is_zero(&self, tolerance: f64) -> Result<bool, NumericError> {
        validate_tolerance(tolerance)?;

        Ok(self.total()?.abs() <= tolerance)
    }
}

impl Default for CompensatedSum {
    fn default() -> Self {
        Self::new()
    }
}

/// Numerically stable sum of finite f64 values.
pub fn compensated_sum<I>(
    values: I,
) -> Result<f64, NumericError>
where
    I: IntoIterator<Item = f64>,
{
    let mut accumulator = CompensatedSum::new();

    for value in values {
        accumulator.add(value)?;
    }

    accumulator.total()
}

/// Returns a numerical-policy signature.
///
/// The signature is deterministic and can be stored by higher-level snapshot
/// or checkpoint code as part of reproducibility metadata.
///
/// This is intentionally not a cryptographic hash.
pub fn policy_signature(policy: &NumericPolicy) -> String {
    format!(
        "{}:{}:{:.17e}:{:.17e}:{:.17e}:{:.17e}:{:.17e}:{:.17e}:{:.17e}:{}:{}",
        NUMERIC_SCHEMA_ID,
        policy.precision(),
        policy.absolute_tolerance(),
        policy.relative_tolerance(),
        policy.probability_tolerance(),
        policy.normalization_tolerance(),
        policy.unitarity_tolerance(),
        policy.hermiticity_tolerance(),
        policy.trace_tolerance(),
        policy.allow_probability_clamping,
        policy.allow_probability_upper_clamping,
    )
}

/// Returns the canonical default policy for double-precision quantum
/// simulation.
pub const fn default_policy() -> NumericPolicy {
    NumericPolicy::f64_default()
}

/// Returns the canonical policy for the requested precision.
pub const fn policy_for_precision(
    precision: Precision,
) -> NumericPolicy {
    NumericPolicy::for_precision(precision)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_f64_policy_is_valid() {
        assert!(NumericPolicy::f64_default().validate().is_ok());
    }

    #[test]
    fn default_f32_policy_is_valid() {
        assert!(NumericPolicy::f32_default().validate().is_ok());
    }

    #[test]
    fn precision_sizes_are_correct() {
        assert_eq!(Precision::F32.real_bytes(), 4);
        assert_eq!(Precision::F32.complex_bytes(), 8);

        assert_eq!(Precision::F64.real_bytes(), 8);
        assert_eq!(Precision::F64.complex_bytes(), 16);
    }

    #[test]
    fn finite_validation_rejects_nan() {
        assert_eq!(
            validate_finite(f64::NAN),
            Err(NumericError::NonFinite)
        );
    }

    #[test]
    fn finite_validation_rejects_infinity() {
        assert_eq!(
            validate_finite(f64::INFINITY),
            Err(NumericError::NonFinite)
        );
    }

    #[test]
    fn finite_validation_accepts_zero() {
        assert!(validate_finite(0.0).is_ok());
    }

    #[test]
    fn approximate_equality_uses_absolute_tolerance() {
        assert!(approx_eq(1.0, 1.000000000001, 1.0e-9, 0.0));
    }

    #[test]
    fn approximate_equality_uses_relative_tolerance() {
        assert!(approx_eq(1.0e10, 1.0000000001e10, 0.0, 1.0e-9));
    }

    #[test]
    fn approximate_equality_rejects_large_difference() {
        assert!(!approx_eq(1.0, 1.1, 1.0e-12, 1.0e-12));
    }

    #[test]
    fn probability_validation_accepts_zero() {
        assert!(validate_probability(0.0, 1.0e-12).is_ok());
    }

    #[test]
    fn probability_validation_accepts_one() {
        assert!(validate_probability(1.0, 1.0e-12).is_ok());
    }

    #[test]
    fn probability_validation_rejects_large_negative() {
        assert_eq!(
            validate_probability(-0.1, 1.0e-12),
            Err(NumericError::NegativeProbability)
        );
    }

    #[test]
    fn probability_validation_rejects_large_positive() {
        assert_eq!(
            validate_probability(1.1, 1.0e-12),
            Err(NumericError::ProbabilityAboveOne)
        );
    }

    #[test]
    fn tiny_negative_probability_can_be_clamped() {
        let policy = NumericPolicy::f64_default();

        let result = policy.clamp_probability_f64(-1.0e-13);

        assert_eq!(result, Ok(0.0));
    }

    #[test]
    fn significant_negative_probability_is_rejected() {
        let policy = NumericPolicy::f64_default();

        let result = policy.clamp_probability_f64(-0.1);

        assert_eq!(
            result,
            Err(NumericError::NegativeProbability)
        );
    }

    #[test]
    fn tiny_probability_above_one_can_be_clamped() {
        let policy = NumericPolicy::f64_default();

        let result = policy.clamp_probability_f64(1.0 + 1.0e-13);

        assert_eq!(result, Ok(1.0));
    }

    #[test]
    fn probability_vector_must_normalize() {
        let policy = NumericPolicy::f64_default();

        assert!(
            policy
                .validate_probabilities([0.25, 0.25, 0.25, 0.25])
                .is_ok()
        );
    }

    #[test]
    fn invalid_probability_vector_is_rejected() {
        let policy = NumericPolicy::f64_default();

        assert_eq!(
            policy.validate_probabilities([0.25, 0.25]),
            Err(NumericError::ProbabilityNormalizationFailed)
        );
    }

    #[test]
    fn normalization_accepts_one() {
        let policy = NumericPolicy::f64_default();

        assert!(policy.validate_normalized_f64(1.0).is_ok());
    }

    #[test]
    fn normalization_rejects_zero() {
        let policy = NumericPolicy::f64_default();

        assert_eq!(
            policy.validate_normalized_f64(0.0),
            Err(NumericError::NormalizationFailed)
        );
    }

    #[test]
    fn trace_accepts_one() {
        let policy = NumericPolicy::f64_default();

        assert!(policy.validate_trace(1.0).is_ok());
    }

    #[test]
    fn complex_values_are_validated() {
        let policy = NumericPolicy::f64_default();
        let value = Complex64::new(1.0, 2.0);

        assert!(policy.validate_complex64(value).is_ok());
    }

    #[test]
    fn complex_nan_is_rejected() {
        let value = Complex64 {
            real: f64::NAN,
            imaginary: 0.0,
        };

        assert_eq!(
            validate_complex(value),
            Err(NumericError::NonFinite)
        );
    }

    #[test]
    fn complex_conversion_is_supported() {
        let value = Complex64::new(1.5, -2.5);

        let converted = try_complex64_to_complex32(value)
            .expect("finite binary64 values in this range fit binary32");

        assert_eq!(converted.real(), 1.5_f32);
        assert_eq!(converted.imaginary(), -2.5_f32);
    }

    #[test]
    fn complex_conversion_rejects_infinity() {
        let value = Complex64 {
            real: f64::INFINITY,
            imaginary: 0.0,
        };

        assert_eq!(
            try_complex64_to_complex32(value),
            Err(NumericError::NonFinite)
        );
    }

    #[test]
    fn complex32_to_complex64_is_lossless_for_finite_values() {
        let value = Complex32::new(1.25, -3.5);

        let converted = complex32_to_complex64(value)
            .expect("finite binary32 values are exactly representable in binary64");

        assert_eq!(converted.real(), 1.25_f64);
        assert_eq!(converted.imaginary(), -3.5_f64);
    }

    #[test]
    fn real_amplitudes_can_be_normalized() {
        let policy = NumericPolicy::f64_default();

        let amplitudes = [
            1.0 / 2.0_f64.sqrt(),
            1.0 / 2.0_f64.sqrt(),
        ];

        assert!(
            validate_real_amplitude_norm(amplitudes, &policy)
                .is_ok()
        );
    }

    #[test]
    fn complex_amplitudes_can_be_normalized() {
        let policy = NumericPolicy::f64_default();

        let value = 1.0 / 2.0_f64.sqrt();

        let amplitudes = [
            Complex64::new(value, 0.0),
            Complex64::new(value, 0.0),
        ];

        assert!(
            validate_complex_amplitude_norm::<Complex64, _>(
                amplitudes,
                &policy,
            )
            .is_ok()
        );
    }

    #[test]
    fn checked_sum_rejects_non_finite_values() {
        assert_eq!(
            checked_sum([1.0, f64::NAN]),
            Err(NumericError::NonFinite)
        );
    }

    #[test]
    fn compensated_sum_is_stable_for_small_terms() {
        let result = compensated_sum([
            1.0,
            1.0e-16,
            -1.0,
        ])
        .expect("finite values");

        assert!(result.abs() <= 1.0e-15);
    }

    #[test]
    fn policy_signature_is_deterministic() {
        let first = policy_signature(&NumericPolicy::f64_default());
        let second = policy_signature(&NumericPolicy::f64_default());

        assert_eq!(first, second);
    }

    #[test]
    fn tolerance_cannot_be_negative() {
        assert_eq!(
            NumericPolicy::f64_default()
                .with_absolute_tolerance(-1.0),
            Err(NumericError::InvalidTolerance)
        );
    }

    #[test]
    fn tolerance_cannot_be_nan() {
        assert_eq!(
            NumericPolicy::f64_default()
                .with_absolute_tolerance(f64::NAN),
            Err(NumericError::InvalidTolerance)
        );
    }

    #[test]
    fn unitary_error_must_be_small() {
        let policy = NumericPolicy::f64_default();

        assert!(
            validate_unitarity_error(1.0e-12, &policy)
                .is_ok()
        );

        assert_eq!(
            validate_unitarity_error(1.0, &policy),
            Err(NumericError::NotUnitary)
        );
    }

    #[test]
    fn hermiticity_error_must_be_small() {
        let policy = NumericPolicy::f64_default();

        assert!(
            validate_hermiticity_error(1.0e-12, &policy)
                .is_ok()
        );

        assert_eq!(
            validate_hermiticity_error(1.0, &policy),
            Err(NumericError::NotHermitian)
        );
    }

    #[test]
    fn sparse_threshold_is_explicit() {
        let policy = NumericPolicy::f64_default();

        assert_eq!(
            zero_if_small(
                1.0e-15,
                policy.sparse_zero_tolerance(),
            ),
            Ok(0.0)
        );
    }

    #[test]
    fn significant_sparse_value_is_preserved() {
        let policy = NumericPolicy::f64_default();

        assert_eq!(
            zero_if_small(
                1.0e-5,
                policy.sparse_zero_tolerance(),
            ),
            Ok(1.0e-5)
        );
    }

    #[test]
    fn fidelity_is_square_of_overlap() {
        let fidelity = overlap_fidelity(0.5)
            .expect("finite non-negative overlap");

        assert_eq!(fidelity, 0.25);
    }

    #[test]
    fn negative_overlap_is_rejected() {
        assert_eq!(
            overlap_fidelity(-0.5),
            Err(NumericError::NegativeValue)
        );
    }
}