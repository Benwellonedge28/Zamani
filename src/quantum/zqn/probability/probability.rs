//! Zamani Quantum Noise (ZQN) — Probability Primitive
//!
//! This module defines the canonical scalar probability type used by ZQN.
//!
//! # Ownership
//!
//! This file owns:
//!
//! - the mathematical probability interval `[0, 1]`;
//! - validated finite floating-point probabilities;
//! - construction and validation;
//! - exact endpoint constants `ZERO` and `ONE`;
//! - checked arithmetic whose results remain probabilities;
//! - probability complements;
//! - conversion to/from the repository's numerical scalar representation;
//! - deterministic ordering and hashing semantics;
//! - formatting;
//! - local probability errors;
//! - probability-related utility operations that do not require a
//!   distribution, quantum resource, or execution context.
//!
//! # Does not own
//!
//! This file does NOT own:
//!
//! - probability distributions;
//! - categorical distributions;
//! - continuous distributions;
//! - statistical estimates;
//! - confidence intervals;
//! - amplitudes;
//! - quantum states;
//! - density matrices;
//! - channels;
//! - Kraus operators;
//! - faults;
//! - noise models;
//! - qubit identities;
//! - physical resources;
//! - calibration;
//! - sampling engines;
//! - random-number generators;
//! - benchmarking policy;
//! - hardware capabilities.
//!
//! Those concerns belong to their respective ZQN modules or existing Zamani
//! quantum subsystems.
//!
//! # Canonical quantum identity boundary
//!
//! A probability has no inherent qubit identity.
//!
//! Therefore this module intentionally does NOT define or import another
//! `QubitId`, `PhysicalQubitId`, resource identifier, or hardware identifier.
//!
//! When a probability is associated with a quantum resource, the owning layer
//! must use the canonical identities from:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! crate::quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! For example:
//!
//! ```text
//! Probability
//!     |
//!     +--> Distribution<QubitId>
//!     |
//!     +--> NoiseLocation::Qubit(QubitId)
//!     |
//!     +--> ReadoutModel<QubitId>
//! ```
//!
//! The scalar probability primitive remains independent of that association.
//!
//! # Mathematical semantics
//!
//! A `Probability` represents a finite real number `p` satisfying:
//!
//! ```text
//! 0 <= p <= 1
//! ```
//!
//! The implementation uses `f64` as its storage representation.
//!
//! This is a representation choice, NOT a semantic machine-size limit.
//! Probability does not become less valid because a quantum system contains
//! more qubits.
//!
//! # Important numerical rule
//!
//! `NaN`, positive infinity, and negative infinity are never valid
//! probabilities.
//!
//! They are rejected rather than:
//!
//! - clamped;
//! - converted to zero;
//! - converted to one;
//! - silently normalized;
//! - silently discarded.
//!
//! This prevents numerical corruption from becoming a physically valid-looking
//! result.
//!
//! # Boundary semantics
//!
//! Both endpoints are valid:
//!
//! ```text
//! Probability::ZERO == 0
//! Probability::ONE  == 1
//! ```
//!
//! A probability of zero means an event has zero probability under the model.
//!
//! A probability of one means an event has unit probability under the model.
//!
//! This type does not assert that the model itself is physically correct.
//! Physical validity of a complete model is the responsibility of the owning
//! distribution/channel/noise subsystem.
//!
//! # Arithmetic semantics
//!
//! Arithmetic is deliberately separated into:
//!
//! - operations that are mathematically guaranteed to remain in `[0, 1]`;
//! - checked operations returning an error if the result leaves `[0, 1]`;
//! - explicitly named transformations such as complement.
//!
//! The implementation does NOT silently clamp arithmetic results.
//!
//! For example:
//!
//! ```text
//! p.checked_add(q)
//! ```
//!
//! fails when `p + q > 1`.
//!
//! This is preferable to silently changing a model.
//!
//! # Precision
//!
//! The semantic domain is the real interval `[0, 1]`; `f64` is the current
//! storage representation.
//!
//! This file therefore does not claim arbitrary exact real-number precision.
//! Future exact/rational or higher-precision representations can implement
//! the same conceptual contract without changing the ZQN semantic model.
//!
//! # Scalability
//!
//! There is no quantum-system-size limit in this type.
//!
//! A probability occupies one scalar value regardless of whether it is used
//! for:
//!
//! - one event;
//! - one qubit;
//! - one million resources;
//! - a distributed quantum system;
//! - a sparse model;
//! - a tensorized model;
//! - a streamed computation.
//!
//! Memory requirements associated with *collections* of probabilities belong
//! to the collection/distribution layer, not this scalar primitive.
//!
//! # Resource safety
//!
//! This module:
//!
//! - performs no allocation;
//! - performs no recursion;
//! - performs no I/O;
//! - performs no network operations;
//! - performs no dynamic code execution;
//! - contains no global mutable state;
//! - contains no global RNG;
//! - contains no unsafe code.
//!
//! Therefore operations on a single `Probability` have constant resource
//! requirements.
//!
//! # Determinism
//!
//! This type is deterministic.
//!
//! It does not sample and does not own an RNG.
//!
//! Given identical IEEE-754 `f64` inputs, the same operation produces the same
//! result on supported Rust targets using the same floating-point semantics.
//!
//! Reproducible stochastic sampling belongs to ZQN's sampling/reproducibility
//! subsystem.
//!
//! # Serialization
//!
//! This module deliberately does not make Rust's in-memory representation an
//! external wire format.
//!
//! Serialization belongs to:
//!
//! ```text
//! zqn::io
//! ```
//!
//! A serializer should serialize the numerical probability value through an
//! explicitly versioned schema.
//!
//! Future schema versions can therefore change representation without making
//! `Probability` itself an accidental wire protocol.
//!
//! # Integration contract
//!
//! ```text
//!                     Probability
//!                          |
//!          +---------------+----------------+
//!          |               |                |
//!          v               v                v
//!     Distribution      Channel          Fault/Noise
//!          |               |                |
//!          +---------------+----------------+
//!                          |
//!                          v
//!                    Characterization
//!                          |
//!                          v
//!                      Benchmarking
//! ```
//!
//! Probability must remain the lowest-level scalar primitive.
//!
//! Higher layers may attach:
//!
//! - resource identity;
//! - operation identity;
//! - uncertainty;
//! - provenance;
//! - calibration;
//! - confidence;
//! - temporal/spatial correlation.
//!
//! They must not redefine the meaning of `[0, 1]`.
//!
//! # Error integration
//!
//! This file provides `ProbabilityError` because it must be independently
//! compilable and testable as a foundational module.
//!
//! The eventual ZQN `core::error` layer may wrap or convert this error without
//! changing this file's mathematical API.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - standard library only.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! - every constructor validates finiteness;
//! - every stored value satisfies `[0, 1]`;
//! - no operation can construct an invalid `Probability` through safe public
//!   APIs;
//! - invalid input produces an explicit error;
//! - arithmetic never silently clamps;
//! - endpoints remain representable;
//! - equality and ordering are deterministic;
//! - no RNG or global state exists;
//! - the implementation has no quantum-machine-size assumptions;
//! - downstream modules can use it without knowing its representation details.
//!
//! # Examples
//!
//! ```
//! use probability::{Probability, ProbabilityError};
//!
//! let p = Probability::new(0.25).expect("0.25 is a valid probability");
//! assert_eq!(p.value(), 0.25);
//! assert_eq!(p.complement().value(), 0.75);
//!
//! assert!(Probability::new(-0.1).is_err());
//! assert!(Probability::new(1.1).is_err());
//! assert!(Probability::new(f64::NAN).is_err());
//! assert!(Probability::new(f64::INFINITY).is_err());
//!
//! let sum = p.checked_add(Probability::new(0.50).unwrap()).unwrap();
//! assert_eq!(sum.value(), 0.75);
//!
//! let _ = ProbabilityError::NonFinite { value: f64::NAN };
//! ```
//!
//! The example assumes this module is compiled as `probability`; within
//! Zamani it is normally reached through `crate::quantum::zqn::probability`.

// Safety contract.
//
// Probability is a pure numerical value. It must never require unsafe code.
//
// Keeping the prohibition here is intentional: if this file is ever moved,
// extracted, or compiled independently, the invariant remains compiler
// enforced.
#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;

/// Mathematical lower bound for a probability.
///
/// This is a semantic mathematical bound, not a machine-size limit.
pub const MIN_PROBABILITY: f64 = 0.0;

/// Mathematical upper bound for a probability.
///
/// This is a semantic mathematical bound, not a machine-size limit.
pub const MAX_PROBABILITY: f64 = 1.0;

/// Canonical probability scalar.
///
/// `Probability` guarantees that its contained value is:
///
/// ```text
/// finite
/// and
/// 0 <= value <= 1
/// ```
///
/// The invariant is maintained by the private field and validated
/// constructors.
///
/// # Why a newtype?
///
/// A raw `f64` can accidentally be used for:
///
/// - an amplitude;
/// - an angle;
/// - a duration;
/// - an error rate;
/// - a probability;
/// - a confidence level.
///
/// `Probability` makes the semantic domain explicit at the type level.
#[derive(Clone, Copy, Debug)]
pub struct Probability(f64);

impl Probability {
    /// Exact zero probability.
    pub const ZERO: Self = Self(0.0);

    /// Exact unit probability.
    pub const ONE: Self = Self(1.0);

    /// Creates a probability from an `f64`.
    ///
    /// # Errors
    ///
    /// Returns:
    ///
    /// - `ProbabilityError::NonFinite` for `NaN` or infinities;
    /// - `ProbabilityError::OutOfRange` for values outside `[0, 1]`.
    ///
    /// No clamping is performed.
    pub fn new(value: f64) -> Result<Self, ProbabilityError> {
        if !value.is_finite() {
            return Err(ProbabilityError::NonFinite { value });
        }

        if !(MIN_PROBABILITY..=MAX_PROBABILITY).contains(&value) {
            return Err(ProbabilityError::OutOfRange { value });
        }

        Ok(Self(value))
    }

    /// Creates a probability from an `f64` known by the caller to be valid.
    ///
    /// This function still validates the value.
    ///
    /// The `const` endpoint constructors should be used when a compile-time
    /// constant is desired.
    pub const fn from_f64_const(value: f64) -> Option<Self> {
        if value.is_nan() {
            return None;
        }

        if value < MIN_PROBABILITY || value > MAX_PROBABILITY {
            return None;
        }

        Some(Self(value))
    }

    /// Returns the underlying `f64`.
    ///
    /// This conversion is lossless with respect to the current representation.
    ///
    /// The returned value is guaranteed to satisfy the `Probability` invariant
    /// because instances can only be constructed through validated safe APIs.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    /// Returns the underlying `f64`.
    ///
    /// This is an explicit alias for APIs where the conversion reads more
    /// naturally as a numerical conversion.
    #[must_use]
    pub const fn as_f64(self) -> f64 {
        self.0
    }

    /// Returns whether this is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns whether this is exactly one.
    #[must_use]
    pub const fn is_one(self) -> bool {
        self.0 == 1.0
    }

    /// Returns whether this probability is strictly between zero and one.
    #[must_use]
    pub const fn is_strictly_between_zero_and_one(self) -> bool {
        self.0 > MIN_PROBABILITY && self.0 < MAX_PROBABILITY
    }

    /// Returns the mathematical complement:
    ///
    /// ```text
    /// 1 - p
    /// ```
    ///
    /// For every valid `p`, the result is also a valid probability.
    #[must_use]
    pub fn complement(self) -> Self {
        // Because self.0 is finite and in [0, 1], 1 - self.0 is necessarily
        // finite and in [0, 1]. The endpoints are also represented exactly.
        Self(MAX_PROBABILITY - self.0)
    }

    /// Returns the minimum of two probabilities.
    #[must_use]
    pub const fn min(self, other: Self) -> Self {
        if self.0 <= other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the maximum of two probabilities.
    #[must_use]
    pub const fn max(self, other: Self) -> Self {
        if self.0 >= other.0 {
            self
        } else {
            other
        }
    }

    /// Returns the absolute numerical difference between two probabilities.
    ///
    /// The result is itself a valid probability because both operands are in
    /// `[0, 1]`.
    #[must_use]
    pub fn abs_difference(self, other: Self) -> Self {
        let difference = (self.0 - other.0).abs();

        // The mathematical result is guaranteed to be in [0, 1].
        // Defensive handling is retained in the implementation so future
        // representation changes cannot silently violate the invariant.
        debug_assert!(
            difference.is_finite()
                && difference >= MIN_PROBABILITY
                && difference <= MAX_PROBABILITY
        );

        Self(difference)
    }

    /// Returns the product of two probabilities.
    ///
    /// Since both operands are in `[0, 1]`, multiplication remains in
    /// `[0, 1]`.
    #[must_use]
    pub fn multiply(self, other: Self) -> Self {
        let product = self.0 * other.0;

        debug_assert!(
            product.is_finite()
                && product >= MIN_PROBABILITY
                && product <= MAX_PROBABILITY
        );

        Self(product)
    }

    /// Checked addition.
    ///
    /// Returns an error if the mathematical result exceeds one.
    ///
    /// No clamping is performed.
    pub fn checked_add(self, other: Self) -> Result<Self, ProbabilityError> {
        let result = self.0 + other.0;

        Self::new(result)
    }

    /// Checked subtraction.
    ///
    /// Returns an error if the mathematical result is negative.
    ///
    /// No clamping is performed.
    pub fn checked_sub(self, other: Self) -> Result<Self, ProbabilityError> {
        let result = self.0 - other.0;

        Self::new(result)
    }

    /// Checked division.
    ///
    /// Returns an error when the divisor is zero or when the mathematical
    /// result is outside `[0, 1]`.
    pub fn checked_div(self, other: Self) -> Result<Self, ProbabilityError> {
        if other.is_zero() {
            return Err(ProbabilityError::DivisionByZero);
        }

        Self::new(self.0 / other.0)
    }

    /// Raises this probability to a non-negative integer power.
    ///
    /// `p^0 = 1`.
    ///
    /// Integer exponentiation is used instead of a floating-point logarithm
    /// path so that common endpoint cases remain exact.
    #[must_use]
    pub fn powi(self, exponent: u32) -> Self {
        if exponent == 0 {
            return Self::ONE;
        }

        let mut base = self;
        let mut exponent = exponent;
        let mut result = Self::ONE;

        while exponent != 0 {
            if exponent & 1 == 1 {
                result = result.multiply(base);
            }

            exponent >>= 1;

            if exponent != 0 {
                base = base.multiply(base);
            }
        }

        result
    }

    /// Returns whether the supplied scalar is a valid probability.
    #[must_use]
    pub fn is_valid(value: f64) -> bool {
        value.is_finite() && (MIN_PROBABILITY..=MAX_PROBABILITY).contains(&value)
    }

    /// Validates a raw `f64` without constructing a `Probability`.
    ///
    /// This is useful at boundaries where a caller wants to validate data
    /// before choosing an allocation or representation.
    pub fn validate(value: f64) -> Result<(), ProbabilityError> {
        Self::new(value).map(|_| ())
    }

    /// Attempts to create a probability from a ratio.
    ///
    /// The numerator and denominator must both be non-negative and the
    /// denominator must be non-zero.
    ///
    /// The resulting ratio must be in `[0, 1]`.
    pub fn from_ratio(
        numerator: u64,
        denominator: u64,
    ) -> Result<Self, ProbabilityError> {
        if denominator == 0 {
            return Err(ProbabilityError::ZeroDenominator);
        }

        if numerator > denominator {
            return Err(ProbabilityError::RatioOutOfRange {
                numerator,
                denominator,
            });
        }

        // Conversion of u64 to f64 is finite for all u64 values. Precision
        // may be rounded by IEEE-754 representation, which is an explicit
        // property of this f64-backed scalar.
        let value = numerator as f64 / denominator as f64;

        Self::new(value)
    }

    /// Attempts to create a probability from a signed integer ratio.
    ///
    /// Negative values are rejected rather than interpreted as magnitudes.
    pub fn from_signed_ratio(
        numerator: i128,
        denominator: i128,
    ) -> Result<Self, ProbabilityError> {
        if denominator == 0 {
            return Err(ProbabilityError::ZeroDenominator);
        }

        if numerator < 0 || denominator < 0 {
            return Err(ProbabilityError::NegativeRatio {
                numerator,
                denominator,
            });
        }

        if numerator > denominator {
            return Err(ProbabilityError::RatioOutOfRangeSigned {
                numerator,
                denominator,
            });
        }

        let value = numerator as f64 / denominator as f64;

        Self::new(value)
    }

    /// Returns the probability as a percentage in `[0, 100]`.
    ///
    /// This method is presentation-oriented.
    ///
    /// It does not change the semantic representation of the probability.
    #[must_use]
    pub fn as_percentage(self) -> f64 {
        self.0 * 100.0
    }

    /// Returns whether this probability is approximately equal to another
    /// probability using an absolute tolerance.
    ///
    /// The tolerance must be finite and non-negative.
    ///
    /// This method is intended for numerical comparisons, not semantic
    /// equality. `PartialEq` remains exact equality.
    pub fn approx_eq(
        self,
        other: Self,
        tolerance: f64,
    ) -> Result<bool, ProbabilityError> {
        validate_tolerance(tolerance)?;

        Ok((self.0 - other.0).abs() <= tolerance)
    }

    /// Returns the numerical distance from zero.
    ///
    /// Since probabilities are non-negative, this equals the probability
    /// itself and is provided primarily for generic numerical code.
    #[must_use]
    pub const fn distance_from_zero(self) -> f64 {
        self.0
    }

    /// Returns the numerical distance from one.
    #[must_use]
    pub fn distance_from_one(self) -> f64 {
        (MAX_PROBABILITY - self.0).abs()
    }
}

// =============================================================================
// Trait implementations
// =============================================================================

impl Default for Probability {
    /// Defaults to zero.
    ///
    /// Zero is the only default that carries no implicit positive event
    /// likelihood.
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<Probability> for f64 {
    fn from(value: Probability) -> Self {
        value.0
    }
}

impl TryFrom<f64> for Probability {
    type Error = ProbabilityError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for Probability {
    type Err = ProbabilityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = value
            .trim()
            .parse::<f64>()
            .map_err(ProbabilityError::ParseFloat)?;

        Self::new(parsed)
    }
}

impl fmt::Display for Probability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl PartialEq for Probability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Probability {}

impl PartialOrd for Probability {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Probability {
    fn cmp(&self, other: &Self) -> Ordering {
        // Both values are guaranteed finite by construction.
        //
        // `total_cmp` gives deterministic ordering even for values that might
        // become unusual through future representation changes.
        self.0.total_cmp(&other.0)
    }
}

// =============================================================================
// Arithmetic trait implementations
// =============================================================================
//
// Intentionally absent:
//
// - Add
// - Sub
// - Div
//
// A raw `p + q` returning another probability can be mathematically invalid.
// Requiring `checked_add` / `checked_sub` / `checked_div` makes invalid model
// construction explicit instead of silently changing semantics.
//
// Multiplication is safe because [0,1] is closed under multiplication, but it
// is still exposed as a named method rather than introducing surprising
// implicit arithmetic semantics into the foundational primitive.

// =============================================================================
// Error model
// =============================================================================

/// Error returned when a value cannot be represented as a valid probability.
#[derive(Clone, Debug, PartialEq)]
pub enum ProbabilityError {
    /// The supplied floating-point value is NaN or infinite.
    NonFinite {
        /// Invalid numerical input.
        value: f64,
    },

    /// The supplied value is finite but outside `[0, 1]`.
    OutOfRange {
        /// Invalid numerical input.
        value: f64,
    },

    /// Division by zero was requested.
    DivisionByZero,

    /// A ratio used a zero denominator.
    ZeroDenominator,

    /// An unsigned ratio has a numerator larger than its denominator.
    RatioOutOfRange {
        /// Ratio numerator.
        numerator: u64,

        /// Ratio denominator.
        denominator: u64,
    },

    /// A signed ratio contains a negative component.
    NegativeRatio {
        /// Ratio numerator.
        numerator: i128,

        /// Ratio denominator.
        denominator: i128,
    },

    /// A signed ratio has a numerator larger than its denominator.
    RatioOutOfRangeSigned {
        /// Ratio numerator.
        numerator: i128,

        /// Ratio denominator.
        denominator: i128,
    },

    /// A numerical tolerance was invalid.
    InvalidTolerance {
        /// Invalid tolerance.
        tolerance: f64,
    },

    /// String parsing failed.
    ParseFloat(ParseFloatError),
}

impl fmt::Display for ProbabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite { value } => {
                write!(
                    formatter,
                    "probability must be finite; received {value:?}"
                )
            }

            Self::OutOfRange { value } => {
                write!(
                    formatter,
                    "probability must be in [{MIN_PROBABILITY}, \
                     {MAX_PROBABILITY}]; received {value}"
                )
            }

            Self::DivisionByZero => {
                write!(formatter, "probability division by zero")
            }

            Self::ZeroDenominator => {
                write!(formatter, "probability ratio denominator must not be zero")
            }

            Self::RatioOutOfRange {
                numerator,
                denominator,
            } => {
                write!(
                    formatter,
                    "probability ratio numerator {numerator} exceeds \
                     denominator {denominator}"
                )
            }

            Self::NegativeRatio {
                numerator,
                denominator,
            } => {
                write!(
                    formatter,
                    "probability ratio must be non-negative; \
                     received {numerator}/{denominator}"
                )
            }

            Self::RatioOutOfRangeSigned {
                numerator,
                denominator,
            } => {
                write!(
                    formatter,
                    "probability ratio numerator {numerator} exceeds \
                     denominator {denominator}"
                )
            }

            Self::InvalidTolerance { tolerance } => {
                write!(
                    formatter,
                    "probability comparison tolerance must be finite and \
                     non-negative; received {tolerance:?}"
                )
            }

            Self::ParseFloat(error) => {
                write!(formatter, "invalid probability number: {error}")
            }
        }
    }
}

impl Error for ProbabilityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseFloat(error) => Some(error),
            _ => None,
        }
    }
}

// =============================================================================
// Internal validation
// =============================================================================

fn validate_tolerance(tolerance: f64) -> Result<(), ProbabilityError> {
    if !tolerance.is_finite() || tolerance < 0.0 {
        return Err(ProbabilityError::InvalidTolerance { tolerance });
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_valid() {
        let probability = Probability::new(0.0).expect("zero is valid");

        assert_eq!(probability, Probability::ZERO);
        assert!(probability.is_zero());
        assert!(!probability.is_one());
        assert!(!probability.is_strictly_between_zero_and_one());
    }

    #[test]
    fn one_is_valid() {
        let probability = Probability::new(1.0).expect("one is valid");

        assert_eq!(probability, Probability::ONE);
        assert!(probability.is_one());
        assert!(!probability.is_zero());
        assert!(!probability.is_strictly_between_zero_and_one());
    }

    #[test]
    fn interior_value_is_valid() {
        let probability = Probability::new(0.25).expect("0.25 is valid");

        assert_eq!(probability.value(), 0.25);
        assert!(probability.is_strictly_between_zero_and_one());
    }

    #[test]
    fn negative_values_are_rejected() {
        assert_eq!(
            Probability::new(-f64::EPSILON),
            Err(ProbabilityError::OutOfRange {
                value: -f64::EPSILON
            })
        );

        assert!(Probability::new(-1.0).is_err());
    }

    #[test]
    fn_values_above_one_are_rejected() {
        assert!(Probability::new(1.0 + f64::EPSILON).is_err());
        assert!(Probability::new(2.0).is_err());
    }

    #[test]
    fn nan_is_rejected() {
        assert_eq!(
            Probability::new(f64::NAN),
            Err(ProbabilityError::NonFinite {
                value: f64::NAN
            })
        );
    }

    #[test]
    fn_positive_infinity_is_rejected() {
        assert_eq!(
            Probability::new(f64::INFINITY),
            Err(ProbabilityError::NonFinite {
                value: f64::INFINITY
            })
        );
    }

    #[test]
    fn negative_infinity_is_rejected() {
        assert_eq!(
            Probability::new(f64::NEG_INFINITY),
            Err(ProbabilityError::NonFinite {
                value: f64::NEG_INFINITY
            })
        );
    }

    #[test]
    fn const_constructor_accepts_valid_values() {
        const VALUE: Option<Probability> = Probability::from_f64_const(0.5);

        assert!(VALUE.is_some());
        assert_eq!(VALUE.unwrap().value(), 0.5);
    }

    #[test]
    fn const_constructor_rejects_nan() {
        const VALUE: Option<Probability> = Probability::from_f64_const(f64::NAN);

        assert!(VALUE.is_none());
    }

    #[test]
    fn const_constructor_rejects_out_of_range_values() {
        const LOW: Option<Probability> = Probability::from_f64_const(-0.1);
        const HIGH: Option<Probability> = Probability::from_f64_const(1.1);

        assert!(LOW.is_none());
        assert!(HIGH.is_none());
    }

    #[test]
    fn complement_is_closed_over_probability_domain() {
        let zero = Probability::ZERO;
        let one = Probability::ONE;
        let quarter = Probability::new(0.25).unwrap();

        assert_eq!(zero.complement(), one);
        assert_eq!(one.complement(), zero);
        assert_eq!(quarter.complement().value(), 0.75);
    }

    #[test]
    fn complement_is_involution() {
        let values = [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0];

        for value in values {
            let probability = Probability::new(value).unwrap();

            assert_eq!(probability.complement().complement(), probability);
        }
    }

    #[test]
    fn multiplication_is_closed() {
        let values = [
            Probability::ZERO,
            Probability::new(0.25).unwrap(),
            Probability::new(0.5).unwrap(),
            Probability::new(0.75).unwrap(),
            Probability::ONE,
        ];

        for left in values {
            for right in values {
                let product = left.multiply(right);

                assert!(Probability::is_valid(product.value()));
            }
        }
    }

    #[test]
    fn checked_add_accepts_valid_sum() {
        let left = Probability::new(0.25).unwrap();
        let right = Probability::new(0.50).unwrap();

        let result = left.checked_add(right).unwrap();

        assert_eq!(result.value(), 0.75);
    }

    #[test]
    fn checked_add_accepts_exact_one() {
        let left = Probability::new(0.25).unwrap();
        let right = Probability::new(0.75).unwrap();

        assert_eq!(left.checked_add(right).unwrap(), Probability::ONE);
    }

    #[test]
    fn checked_add_rejects_sum_above_one() {
        let left = Probability::new(0.75).unwrap();
        let right = Probability::new(0.50).unwrap();

        assert!(matches!(
            left.checked_add(right),
            Err(ProbabilityError::OutOfRange { .. })
        ));
    }

    #[test]
    fn checked_sub_accepts_non_negative_difference() {
        let left = Probability::new(0.75).unwrap();
        let right = Probability::new(0.25).unwrap();

        let result = left.checked_sub(right).unwrap();

        assert_eq!(result.value(), 0.50);
    }

    #[test]
    fn checked_sub_accepts_exact_zero() {
        let probability = Probability::new(0.5).unwrap();

        assert_eq!(
            probability.checked_sub(probability).unwrap(),
            Probability::ZERO
        );
    }

    #[test]
    fn checked_sub_rejects_negative_result() {
        let left = Probability::new(0.25).unwrap();
        let right = Probability::new(0.75).unwrap();

        assert!(matches!(
            left.checked_sub(right),
            Err(ProbabilityError::OutOfRange { .. })
        ));
    }

    #[test]
    fn checked_division_is_correct() {
        let numerator = Probability::new(0.25).unwrap();
        let denominator = Probability::new(0.50).unwrap();

        assert_eq!(
            numerator.checked_div(denominator).unwrap().value(),
            0.5
        );
    }

    #[test]
    fn checked_division_by_zero_is_rejected() {
        let numerator = Probability::new(0.25).unwrap();

        assert_eq!(
            numerator.checked_div(Probability::ZERO),
            Err(ProbabilityError::DivisionByZero)
        );
    }

    #[test]
    fn checked_division_above_one_is_rejected() {
        let numerator = Probability::new(0.75).unwrap();
        let denominator = Probability::new(0.25).unwrap();

        assert!(matches!(
            numerator.checked_div(denominator),
            Err(ProbabilityError::OutOfRange { .. })
        ));
    }

    #[test]
    fn power_zero_is_one() {
        let probability = Probability::new(0.25).unwrap();

        assert_eq!(probability.powi(0), Probability::ONE);
    }

    #[test]
    fn power_one_is_identity() {
        let probability = Probability::new(0.25).unwrap();

        assert_eq!(probability.powi(1), probability);
    }

    #[test]
    fn powers_remain_in_probability_domain() {
        let probability = Probability::new(0.75).unwrap();

        for exponent in 0..=100 {
            let result = probability.powi(exponent);

            assert!(Probability::is_valid(result.value()));
        }
    }

    #[test]
    fn from_ratio_accepts_valid_ratio() {
        assert_eq!(
            Probability::from_ratio(1, 4).unwrap().value(),
            0.25
        );

        assert_eq!(
            Probability::from_ratio(3, 4).unwrap().value(),
            0.75
        );

        assert_eq!(
            Probability::from_ratio(1, 1).unwrap(),
            Probability::ONE
        );
    }

    #[test]
    fn from_ratio_rejects_zero_denominator() {
        assert_eq!(
            Probability::from_ratio(0, 0),
            Err(ProbabilityError::ZeroDenominator)
        );
    }

    #[test]
    fn from_ratio_rejects_numerator_above_denominator() {
        assert_eq!(
            Probability::from_ratio(2, 1),
            Err(ProbabilityError::RatioOutOfRange {
                numerator: 2,
                denominator: 1
            })
        );
    }

    #[test]
    fn signed_ratio_rejects_negative_numerator() {
        assert!(matches!(
            Probability::from_signed_ratio(-1, 2),
            Err(ProbabilityError::NegativeRatio { .. })
        ));
    }

    #[test]
    fn signed_ratio_rejects_negative_denominator() {
        assert!(matches!(
            Probability::from_signed_ratio(1, -2),
            Err(ProbabilityError::NegativeRatio { .. })
        ));
    }

    #[test]
    fn signed_ratio_accepts_valid_ratio() {
        assert_eq!(
            Probability::from_signed_ratio(1, 4).unwrap().value(),
            0.25
        );
    }

    #[test]
    fn percentage_conversion_is_correct() {
        let probability = Probability::new(0.25).unwrap();

        assert_eq!(probability.as_percentage(), 25.0);
    }

    #[test]
    fn absolute_difference_is_valid() {
        let left = Probability::new(0.25).unwrap();
        let right = Probability::new(0.75).unwrap();

        assert_eq!(left.abs_difference(right).value(), 0.5);
    }

    #[test]
    fn min_and_max_are_correct() {
        let left = Probability::new(0.25).unwrap();
        let right = Probability::new(0.75).unwrap();

        assert_eq!(left.min(right), left);
        assert_eq!(left.max(right), right);
    }

    #[test]
    fn approximate_equality_accepts_within_tolerance() {
        let left = Probability::new(0.5).unwrap();
        let right = Probability::new(0.500_000_000_1).unwrap();

        assert!(left.approx_eq(right, 1e-9).unwrap());
    }

    #[test]
    fn approximate_equality_rejects_outside_tolerance() {
        let left = Probability::new(0.5).unwrap();
        let right = Probability::new(0.51).unwrap();

        assert!(!left.approx_eq(right, 1e-3).unwrap());
    }

    #[test]
    fn invalid_tolerance_is_rejected() {
        let probability = Probability::new(0.5).unwrap();

        assert!(matches!(
            probability.approx_eq(
                Probability::new(0.5).unwrap(),
                f64::NAN
            ),
            Err(ProbabilityError::InvalidTolerance { .. })
        ));

        assert!(matches!(
            probability.approx_eq(
                Probability::new(0.5).unwrap(),
                -1.0
            ),
            Err(ProbabilityError::InvalidTolerance { .. })
        ));

        assert!(matches!(
            probability.approx_eq(
                Probability::new(0.5).unwrap(),
                f64::INFINITY
            ),
            Err(ProbabilityError::InvalidTolerance { .. })
        ));
    }

    #[test]
    fn display_is_stable_for_common_values() {
        let probability = Probability::new(0.25).unwrap();

        assert_eq!(probability.to_string(), "0.25");
    }

    #[test]
    fn parsing_valid_values_works() {
        let probability: Probability = "0.25".parse().unwrap();

        assert_eq!(probability.value(), 0.25);
    }

    #[test]
    fn parsing_whitespace_is_supported() {
        let probability: Probability = " 0.25 ".parse().unwrap();

        assert_eq!(probability.value(), 0.25);
    }

    #[test]
    fn parsing_nan_is_rejected() {
        let result = "NaN".parse::<Probability>();

        assert!(matches!(
            result,
            Err(ProbabilityError::NonFinite { .. })
        ));
    }

    #[test]
    fn parsing_infinity_is_rejected() {
        let result = "inf".parse::<Probability>();

        assert!(matches!(
            result,
            Err(ProbabilityError::NonFinite { .. })
        ));
    }

    #[test]
    fn ordering_is_deterministic() {
        let zero = Probability::ZERO;
        let quarter = Probability::new(0.25).unwrap();
        let one = Probability::ONE;

        assert!(zero < quarter);
        assert!(quarter < one);
        assert!(zero < one);
    }

    #[test]
    fn default_is_zero() {
        assert_eq!(Probability::default(), Probability::ZERO);
    }

    #[test]
    fn f64_conversion_is_lossless_for_stored_value() {
        let original = Probability::new(0.123_456_789).unwrap();

        let converted: f64 = original.into();

        assert_eq!(converted, original.value());
    }

    #[test]
    fn validate_matches_constructor() {
        assert!(Probability::validate(0.5).is_ok());
        assert!(Probability::validate(-0.1).is_err());
        assert!(Probability::validate(1.1).is_err());
        assert!(Probability::validate(f64::NAN).is_err());
        assert!(Probability::validate(f64::INFINITY).is_err());
    }

    #[test]
    fn all_publicly_constructible_values_preserve_invariant() {
        let values = [
            Probability::ZERO,
            Probability::ONE,
            Probability::new(0.1).unwrap(),
            Probability::new(0.5).unwrap(),
            Probability::new(0.9).unwrap(),
            Probability::from_ratio(1, 3).unwrap(),
            Probability::from_ratio(2, 3).unwrap(),
        ];

        for probability in values {
            assert!(probability.value().is_finite());
            assert!(probability.value() >= MIN_PROBABILITY);
            assert!(probability.value() <= MAX_PROBABILITY);
        }
    }

    #[test]
    fn multiplication_endpoints_are_exact() {
        assert_eq!(
            Probability::ZERO.multiply(Probability::ONE),
            Probability::ZERO
        );

        assert_eq!(
            Probability::ONE.multiply(Probability::ONE),
            Probability::ONE
        );
    }

    #[test]
    fn complement_endpoints_are_exact() {
        assert_eq!(Probability::ZERO.complement(), Probability::ONE);
        assert_eq!(Probability::ONE.complement(), Probability::ZERO);
    }

    #[test]
    fn distance_methods_are_correct() {
        let probability = Probability::new(0.25).unwrap();

        assert_eq!(probability.distance_from_zero(), 0.25);
        assert_eq!(probability.distance_from_one(), 0.75);
    }

    #[test]
    fn no_operation_can_create_an_invalid_probability() {
        let values = [
            Probability::ZERO,
            Probability::ONE,
            Probability::new(0.01).unwrap(),
            Probability::new(0.5).unwrap(),
            Probability::new(0.99).unwrap(),
        ];

        for probability in values {
            assert!(Probability::is_valid(probability.value()));
            assert!(Probability::is_valid(probability.complement().value()));
            assert!(Probability::is_valid(
                probability.multiply(probability).value()
            ));

            for exponent in 0..=32 {
                assert!(Probability::is_valid(
                    probability.powi(exponent).value()
                ));
            }
        }
    }
}