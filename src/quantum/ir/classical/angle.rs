//! Zamani Quantum IR — Classical Angle Semantics
//!
//! Path:
//!     src/quantum/ir/classical/angle.rs
//!
//! # Purpose
//!
//! This module owns the semantic representation of classical mathematical
//! angles used by the canonical Zamani Quantum IR.
//!
//! An [`Angle`] represents an angle measured in radians at the semantic level.
//! Its underlying expression is represented by the canonical
//! `quantum::ir::core::parameter::Parameter` system.
//!
//! This gives Zamani one stable angle abstraction that can represent:
//!
//! - concrete finite angles;
//! - symbolic angles;
//! - parameterized angles;
//! - mathematical constants such as `pi` and `tau`;
//! - arithmetic combinations of angles;
//! - explicit evaluation with caller-supplied parameter bindings;
//! - periodic normalization modulo `2*pi`.
//!
//! # Architectural ownership
//!
//! This module owns:
//!
//! - [`Angle`];
//! - angle construction;
//! - angle-specific validation;
//! - angle-specific evaluation;
//! - angle-specific canonical representation;
//! - angle-specific periodic normalization;
//! - angle arithmetic at the semantic layer;
//! - built-in angle constants;
//! - conversion between degrees, turns and radians.
//!
//! This module does NOT own:
//!
//! - generic parameters;
//! - generic parameter expressions;
//! - quantum-bit identities;
//! - physical-qubit identities;
//! - gate definitions;
//! - pulse definitions;
//! - hardware precision;
//! - hardware calibration;
//! - scheduling;
//! - routing;
//! - optimization policy;
//! - frontend syntax;
//! - backend execution;
//! - simulator state.
//!
//! Those responsibilities belong to other IR layers.
//!
//! # Dependency boundary
//!
//! ```text
//! quantum::ir::core::parameter
//!             │
//!             ▼
//!     classical::angle
//!             │
//!       ┌─────┼─────┐
//!       ▼     ▼     ▼
//!     gates  pulse  classical values
//! ```
//!
//! The dependency is deliberately one-way.
//!
//! ```text
//! angle.rs ─────► core/parameter.rs
//! ```
//!
//! `core/parameter.rs` must not depend on this module.
//!
//! # Quantum identity boundary
//!
//! Angles do not identify quantum resources.
//!
//! Therefore this file deliberately does NOT import or define a qubit
//! identifier.
//!
//! When an angle is attached to a quantum operation, the operation/gate layer
//! combines this semantic angle with the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! owned by `quantum::ir::qubit`.
//!
//! There must never be another `QubitId` defined here.
//!
//! # Unit semantics
//!
//! The canonical semantic unit of [`Angle`] is radians.
//!
//! This is important because the same semantic angle can later be lowered to:
//!
//! - radians;
//! - degrees;
//! - turns;
//! - fixed-width phase representations;
//! - hardware-specific phase accumulators;
//! - pulse-frame phase values;
//! - native gate parameters;
//! - another target representation.
//!
//! Those representations belong to downstream lowering.
//!
//! The canonical IR therefore stores the meaning of the angle rather than
//! the representation required by a particular machine.
//!
//! # Symbolic semantics
//!
//! An angle can contain a symbolic [`Parameter`].
//!
//! Examples:
//!
//! ```text
//! theta
//! theta / 2
//! theta + pi
//! 2 * theta
//! -theta
//! ```
//!
//! The angle module does not introduce another expression language.
//! It reuses the canonical parameter expression system.
//!
//! This is critical for keeping symbolic optimization and parameter binding
//! consistent across gates, pulses, Hamiltonians and other IR models.
//!
//! # Built-in constants
//!
//! The canonical angle environment recognizes:
//!
//! ```text
//! pi
//! tau
//! ```
//!
//! where:
//!
//! ```text
//! tau = 2*pi
//! ```
//!
//! These constants are semantic constants of the angle domain.
//!
//! They are resolved by [`Angle::evaluate`] and therefore do not require a
//! mutable global environment.
//!
//! A caller-supplied resolver may still provide every other symbol explicitly.
//!
//! # Periodicity
//!
//! Angles are mathematically periodic:
//!
//! ```text
//! theta ≡ theta + 2*pi*n
//! ```
//!
//! The canonical [`Angle`] value is NOT automatically normalized when it is
//! constructed.
//!
//! This is deliberate.
//!
//! Automatic normalization would destroy useful symbolic information such as:
//!
//! ```text
//! theta + 2*pi
//! ```
//!
//! and would make canonical symbolic optimization more difficult.
//!
//! Instead, normalization is explicit through:
//!
//! ```text
//! Angle::normalize_radians(...)
//! Angle::evaluate_normalized(...)
//! ```
//!
//! This preserves both:
//!
//! ```text
//! semantic expression
//! ```
//!
//! and:
//!
//! ```text
//! evaluated periodic representation
//! ```
//!
//! as separate concepts.
//!
//! # Numerical semantics
//!
//! Concrete values use finite IEEE-754 `f64` through the existing canonical
//! `Parameter` implementation.
//!
//! NaN and positive/negative infinity are rejected at this module's boundary.
//!
//! This does NOT make `f64` the semantic precision ceiling of Zamani.
//!
//! The canonical IR is intentionally designed so that a future parameter
//! implementation can provide arbitrary/exact numeric representations without
//! changing the ownership model of this file.
//!
//! In particular, no quantum-machine size is encoded by this module.
//!
//! # Scalability
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_ANGLE_BITS
//! MAX_PARAMETERS
//! MAX_QUBITS
//! MAX_GATES
//! MAX_PROGRAM_SIZE
//! MAX_MACHINE_SIZE
//! ```
//!
//! An angle expression may therefore grow according to available resources
//! and explicit compiler/security policies.
//!
//! Resource limits belong to:
//!
//! ```text
//! QuantumIrLimits
//! ParameterValidationPolicy
//! ```
//!
//! or equivalent downstream execution policy.
//!
//! They are not semantic limits of `Angle`.
//!
//! # Determinism
//!
//! The underlying `Parameter` implementation provides deterministic canonical
//! textual representations.
//!
//! This module exposes that representation through [`Angle::canonical_string`].
//!
//! Consequently the same semantic angle expression produces the same
//! canonical textual form when the underlying parameter expression is the
//! same.
//!
//! This is suitable for:
//!
//! - diagnostics;
//! - reproducible compilation;
//! - canonical serialization;
//! - hashing layers;
//! - provenance;
//! - compiler caching.
//!
//! # Integration contract
//!
//! `core/parameter.rs`
//!     owns generic symbolic parameter semantics and evaluation.
//!
//! `core/types.rs`
//!     owns [`AngleType`] as the semantic type descriptor.
//!
//! `classical/value.rs`
//!     currently owns concrete classical runtime values and may represent a
//!     concrete angle using its existing finite-angle representation.
//!
//! `quantum/gate.rs`
//!     may consume [`Angle`] for semantic gate parameters.
//!
//! `program/operation.rs`
//!     may carry angle-valued operands/results through the canonical value/type
//!     system.
//!
//! `pulse/frame.rs`
//!     may use [`Angle`] for phase expressions.
//!
//! `pulse/calibration.rs`
//!     may use [`Angle`] for symbolic phase/calibration parameters.
//!
//! `optimization/parameter/*`
//!     can consume `Angle::parameter()` without knowing the angle's higher-level
//!     semantic ownership.
//!
//! `validation/*`
//!     may call [`Angle::validate`] and parameter-policy validation.
//!
//! `serialization/*`
//!     should serialize the canonical parameter representation exposed by
//!     [`Angle::parameter`] or [`Angle::canonical_string`] according to the
//!     canonical IR serialization schema.
//!
//! `hashing/*`
//!     should hash the canonical semantic representation rather than any
//!     hardware-specific representation.
//!
//! `frontend/*`
//!     should lower source-language angle syntax into [`Angle`].
//!
//! `hardware/*`
//!     may lower an angle into target-specific phase representations but must
//!     never become a dependency of this module.
//!
//! # Important integration rule
//!
//! This module does not require `quantum::ir::qubit` because an angle is a
//! classical semantic quantity.
//!
//! A gate such as:
//!
//! ```text
//! rx(theta) q
//! ```
//!
//! is represented downstream by combining:
//!
//! ```text
//! Angle(theta)
//! QubitId(q)
//! ```
//!
//! The angle module must never manufacture or duplicate the qubit identity.
//!
//! # Rust compatibility
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition.
//!
//! Requirements:
//!
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - no additional external dependency.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler enforced.
//!
//! # Stability rule
//!
//! Once downstream IR modules depend on [`Angle`], existing semantic meaning
//! must not be silently changed.
//!
//! New constructors and methods may be added compatibly.
//!
//! Changes to the meaning of canonical representation, evaluation or periodic
//! semantics require an IR version/migration decision.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;
use std::ops::{Add, Neg, Sub};

use super::super::core::parameter::{
    Parameter,
    ParameterBindings,
    ParameterError,
    ParameterValidationPolicy,
};

// =============================================================================
// Mathematical constants
// =============================================================================

/// Mathematical π in the canonical `f64` evaluation domain.
///
/// This constant is used only when a concrete symbolic angle is evaluated
/// through the current `Parameter` implementation.
///
/// It does not define the semantic precision of the Zamani IR.
pub const PI_RADIANS: f64 = std::f64::consts::PI;

/// Mathematical τ = 2π in the canonical `f64` evaluation domain.
///
/// This constant is used only when a concrete symbolic angle is evaluated
/// through the current `Parameter` implementation.
pub const TAU_RADIANS: f64 = std::f64::consts::TAU;

/// Degrees in one complete revolution.
pub const DEGREES_PER_TURN: f64 = 360.0;

/// Degrees in a half revolution.
pub const DEGREES_PER_HALF_TURN: f64 = 180.0;

// =============================================================================
// Errors
// =============================================================================

/// Error returned by checked angle operations.
#[derive(Debug, Clone, PartialEq)]
pub enum AngleError {
    /// A concrete radian value was not finite.
    NonFiniteRadians,

    /// A concrete degree value was not finite.
    NonFiniteDegrees,

    /// A turn value was not finite.
    NonFiniteTurns,

    /// A scaling factor was not finite.
    NonFiniteScale,

    /// Division by zero was requested.
    DivisionByZero,

    /// A parameter expression failed validation.
    Parameter(ParameterError),

    /// Evaluation produced a non-finite result.
    EvaluationNonFinite,
}

impl fmt::Display for AngleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteRadians => {
                formatter.write_str("angle in radians must be finite")
            }

            Self::NonFiniteDegrees => {
                formatter.write_str("angle in degrees must be finite")
            }

            Self::NonFiniteTurns => {
                formatter.write_str("angle in turns must be finite")
            }

            Self::NonFiniteScale => {
                formatter.write_str("angle scale must be finite")
            }

            Self::DivisionByZero => {
                formatter.write_str("angle division by zero is invalid")
            }

            Self::Parameter(error) => {
                write!(formatter, "angle parameter error: {error}")
            }

            Self::EvaluationNonFinite => {
                formatter.write_str(
                    "angle evaluation produced a non-finite value",
                )
            }
        }
    }
}

impl std::error::Error for AngleError {
    fn source(
        &self,
    ) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Parameter(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ParameterError> for AngleError {
    fn from(error: ParameterError) -> Self {
        Self::Parameter(error)
    }
}

/// Result type for checked angle operations.
pub type AngleResult<T> = Result<T, AngleError>;

// =============================================================================
// Angle
// =============================================================================

/// Canonical semantic quantum angle.
///
/// `Angle` is a unit-aware wrapper around the canonical Zamani
/// [`Parameter`] abstraction.
///
/// The expression is semantically measured in radians.
///
/// # Examples
///
/// Concrete angle:
///
/// ```
/// # use crate::quantum::ir::classical::angle::Angle;
/// let theta = Angle::radians(1.25)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// Symbolic angle:
///
/// ```text
/// theta
/// theta / 2
/// theta + pi
/// ```
///
/// The expression remains symbolic until an explicit binding environment is
/// supplied.
#[derive(Debug, Clone, PartialEq)]
pub struct Angle {
    parameter: Parameter,
}

impl Angle {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates an angle from an already validated canonical parameter.
    ///
    /// The parameter remains owned by the canonical parameter subsystem.
    ///
    /// This constructor does not normalize the angle modulo `2*pi`.
    pub fn from_parameter(
        parameter: Parameter,
    ) -> AngleResult<Self> {
        parameter.validate()?;

        Ok(Self { parameter })
    }

    /// Creates a concrete angle from radians.
    ///
    /// The value must be finite.
    pub fn radians(value: f64) -> AngleResult<Self> {
        if !value.is_finite() {
            return Err(AngleError::NonFiniteRadians);
        }

        Ok(Self {
            parameter: Parameter::from(value),
        })
    }

    /// Creates a concrete angle from degrees.
    ///
    /// The stored semantic value is converted to radians.
    pub fn degrees(value: f64) -> AngleResult<Self> {
        if !value.is_finite() {
            return Err(AngleError::NonFiniteDegrees);
        }

        let radians = value * PI_RADIANS / DEGREES_PER_HALF_TURN;

        if !radians.is_finite() {
            return Err(AngleError::NonFiniteRadians);
        }

        Self::radians(radians)
    }

    /// Creates a concrete angle from turns.
    ///
    /// One turn is exactly one complete revolution, represented as `2*pi`
    /// radians at the concrete evaluation layer.
    pub fn turns(value: f64) -> AngleResult<Self> {
        if !value.is_finite() {
            return Err(AngleError::NonFiniteTurns);
        }

        let radians = value * TAU_RADIANS;

        if !radians.is_finite() {
            return Err(AngleError::NonFiniteRadians);
        }

        Self::radians(radians)
    }

    /// Creates a symbolic angle from a parameter name.
    ///
    /// The name is interpreted as an angle-valued symbol by the consumer.
    pub fn symbol<S: Into<String>>(
        name: S,
    ) -> AngleResult<Self> {
        Ok(Self {
            parameter: Parameter::symbol(name)?,
        })
    }

    /// Returns the zero angle.
    #[must_use]
    pub fn zero() -> Self {
        Self {
            parameter: Parameter::zero(),
        }
    }

    /// Returns the mathematical π angle.
    ///
    /// The representation remains symbolic until evaluated.
    #[must_use]
    pub fn pi() -> Self {
        Self {
            parameter: Parameter::Symbol("pi".to_owned()),
        }
    }

    /// Returns the mathematical τ = 2π angle.
    ///
    /// The representation remains symbolic until evaluated.
    #[must_use]
    pub fn tau() -> Self {
        Self {
            parameter: Parameter::Symbol("tau".to_owned()),
        }
    }

    /// Returns π/2.
    #[must_use]
    pub fn half_pi() -> Self {
        Self::pi().divided_by_unchecked(2.0)
    }

    /// Returns π/4.
    #[must_use]
    pub fn quarter_pi() -> Self {
        Self::pi().divided_by_unchecked(4.0)
    }

    /// Returns 3π/2.
    #[must_use]
    pub fn three_half_pi() -> Self {
        Self::pi().scaled_unchecked(1.5)
    }

    // -------------------------------------------------------------------------
    // Parameter access
    // -------------------------------------------------------------------------

    /// Returns the underlying canonical parameter expression.
    ///
    /// This does not transfer ownership.
    #[must_use]
    pub fn parameter(&self) -> &Parameter {
        &self.parameter
    }

    /// Consumes the angle and returns its canonical parameter expression.
    #[must_use]
    pub fn into_parameter(self) -> Parameter {
        self.parameter
    }

    /// Returns the deterministic canonical parameter representation.
    ///
    /// This intentionally excludes the textual `angle(...)` wrapper used by
    /// [`canonical_string`].
    #[must_use]
    pub fn parameter_canonical_string(&self) -> String {
        self.parameter.canonical_string()
    }

    /// Returns a deterministic canonical textual representation.
    ///
    /// Example:
    ///
    /// ```text
    /// angle(theta)
    /// angle((theta + pi))
    /// angle(1.5707963267948966)
    /// ```
    #[must_use]
    pub fn canonical_string(&self) -> String {
        format!(
            "angle({})",
            self.parameter.canonical_string()
        )
    }

    /// Returns all distinct symbolic parameters in deterministic order.
    ///
    /// Built-in constants such as `pi` and `tau` are returned as symbols
    /// because they are represented symbolically in the canonical expression.
    #[must_use]
    pub fn symbols(&self) -> Vec<String> {
        self.parameter.symbols()
    }

    /// Validates the underlying parameter expression using the canonical
    /// unrestricted parameter policy.
    pub fn validate(&self) -> AngleResult<()> {
        self.parameter.validate()?;
        Ok(())
    }

    /// Validates the underlying parameter expression against an explicit
    /// resource/security policy.
    ///
    /// This policy is not an architectural limit on Zamani.
    pub fn validate_with_policy(
        &self,
        policy: ParameterValidationPolicy,
    ) -> AngleResult<()> {
        self.parameter.validate_with_policy(policy)?;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Concrete inspection
    // -------------------------------------------------------------------------

    /// Returns the concrete radian value when the angle is directly constant.
    ///
    /// A symbolic expression such as `pi / 2` returns `None`, even though it
    /// can be evaluated, because it remains semantically symbolic.
    #[must_use]
    pub fn as_radians(&self) -> Option<f64> {
        self.parameter.as_constant()
    }

    /// Returns whether this angle is represented by a direct concrete
    /// constant.
    #[must_use]
    pub fn is_concrete(&self) -> bool {
        self.as_radians().is_some()
    }

    /// Returns whether this angle contains symbolic/expression semantics.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        !self.is_concrete()
    }

    // -------------------------------------------------------------------------
    // Evaluation
    // -------------------------------------------------------------------------

    /// Evaluates an angle using explicit parameter bindings.
    ///
    /// The evaluation environment recognizes the built-in constants:
    ///
    /// ```text
    /// pi
    /// tau
    /// ```
    ///
    /// No global mutable state is consulted.
    pub fn evaluate(
        &self,
        bindings: &ParameterBindings,
    ) -> AngleResult<f64> {
        self.evaluate_with_resolver(&|name| bindings.get(name))
    }

    /// Evaluates an angle using an explicit symbol resolver.
    ///
    /// `pi` and `tau` are reserved semantic angle constants and are resolved
    /// by this method before consulting the caller's resolver.
    pub fn evaluate_with_resolver<F>(
        &self,
        resolver: &F,
    ) -> AngleResult<f64>
    where
        F: Fn(&str) -> Option<f64>,
    {
        let value = self
            .parameter
            .evaluate_with_resolver(&|name| match name {
                "pi" => Some(PI_RADIANS),
                "tau" => Some(TAU_RADIANS),
                _ => resolver(name),
            })?;

        if !value.is_finite() {
            return Err(AngleError::EvaluationNonFinite);
        }

        Ok(value)
    }

    /// Evaluates an angle and normalizes the resulting concrete value into
    /// `[0, 2*pi)`.
    pub fn evaluate_normalized(
        &self,
        bindings: &ParameterBindings,
    ) -> AngleResult<f64> {
        let value = self.evaluate(bindings)?;
        Self::normalize_radians(value)
    }

    /// Evaluates an angle using an explicit resolver and normalizes it into
    /// `[0, 2*pi)`.
    pub fn evaluate_normalized_with_resolver<F>(
        &self,
        resolver: &F,
    ) -> AngleResult<f64>
    where
        F: Fn(&str) -> Option<f64>,
    {
        let value = self.evaluate_with_resolver(resolver)?;
        Self::normalize_radians(value)
    }

    // -------------------------------------------------------------------------
    // Periodic semantics
    // -------------------------------------------------------------------------

    /// Normalizes a concrete radian value into `[0, 2*pi)`.
    ///
    /// This is an explicit operation. Construction of an [`Angle`] never
    /// performs normalization automatically.
    ///
    /// Rust's stable `f64::rem_euclid` is available on the Rust 1.97 target,
    /// but the implementation below intentionally uses `%` plus a correction
    /// to keep the operation straightforward and compatible with the
    /// canonical floating-point semantics.
    pub fn normalize_radians(
        value: f64,
    ) -> AngleResult<f64> {
        if !value.is_finite() {
            return Err(AngleError::NonFiniteRadians);
        }

        let remainder = value % TAU_RADIANS;

        let normalized = if remainder < 0.0 {
            remainder + TAU_RADIANS
        } else {
            remainder
        };

        if !normalized.is_finite() {
            return Err(AngleError::EvaluationNonFinite);
        }

        // Floating-point roundoff can theoretically produce the upper
        // endpoint. The canonical half-open interval requires it to map to
        // zero.
        if normalized >= TAU_RADIANS {
            return Ok(0.0);
        }

        Ok(normalized)
    }

    /// Returns whether two concrete radian values are exactly equal under
    /// the canonical modulo-`2*pi` representation.
    ///
    /// This method deliberately does not use an arbitrary tolerance.
    /// Approximate numerical equality is a policy decision and must not be
    /// silently introduced into semantic IR equality.
    pub fn equivalent_modulo_tau(
        lhs: f64,
        rhs: f64,
    ) -> AngleResult<bool> {
        let lhs = Self::normalize_radians(lhs)?;
        let rhs = Self::normalize_radians(rhs)?;

        Ok(lhs == rhs)
    }

    // -------------------------------------------------------------------------
    // Arithmetic
    // -------------------------------------------------------------------------

    /// Adds another angle expression.
    ///
    /// The result remains symbolic if either operand is symbolic.
    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        Self {
            parameter: self.parameter.clone() + other.parameter.clone(),
        }
    }

    /// Subtracts another angle expression.
    ///
    /// The result remains symbolic if either operand is symbolic.
    #[must_use]
    pub fn sub(&self, other: &Self) -> Self {
        Self {
            parameter: self.parameter.clone() - other.parameter.clone(),
        }
    }

    /// Negates an angle expression.
    #[must_use]
    pub fn negated(&self) -> Self {
        Self {
            parameter: -self.parameter.clone(),
        }
    }

    /// Multiplies an angle by a finite scalar.
    pub fn scaled(
        &self,
        factor: f64,
    ) -> AngleResult<Self> {
        if !factor.is_finite() {
            return Err(AngleError::NonFiniteScale);
        }

        Ok(Self {
            parameter: self.parameter.clone() * Parameter::from(factor),
        })
    }

    /// Divides an angle by a finite, non-zero scalar.
    pub fn divided_by(
        &self,
        divisor: f64,
    ) -> AngleResult<Self> {
        if !divisor.is_finite() {
            return Err(AngleError::NonFiniteScale);
        }

        if divisor == 0.0 {
            return Err(AngleError::DivisionByZero);
        }

        Ok(Self {
            parameter: self.parameter.clone()
                / Parameter::from(divisor),
        })
    }

    /// Internal infallible scalar multiplication used only for fixed,
    /// statically valid constants such as `pi / 2`.
    fn scaled_unchecked(&self, factor: f64) -> Self {
        debug_assert!(factor.is_finite());

        Self {
            parameter: self.parameter.clone() * Parameter::from(factor),
        }
    }

    /// Internal infallible scalar division used only for fixed,
    /// statically valid constants such as `pi / 2`.
    fn divided_by_unchecked(&self, divisor: f64) -> Self {
        debug_assert!(divisor.is_finite());
        debug_assert!(divisor != 0.0);

        Self {
            parameter: self.parameter.clone()
                / Parameter::from(divisor),
        }
    }
}

// =============================================================================
// Standard operator implementations
// =============================================================================

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            parameter: self.parameter + rhs.parameter,
        }
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            parameter: self.parameter - rhs.parameter,
        }
    }
}

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            parameter: -self.parameter,
        }
    }
}

impl fmt::Display for Angle {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.canonical_string())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    #[test]
    fn zero_is_zero_radians() {
        let angle = Angle::zero();

        assert_eq!(angle.as_radians(), Some(0.0));
        assert!(angle.is_concrete());
        assert!(!angle.is_symbolic());
    }

    #[test]
    fn radians_rejects_nan() {
        assert!(matches!(
            Angle::radians(f64::NAN),
            Err(AngleError::NonFiniteRadians)
        ));
    }

    #[test]
    fn radians_rejects_positive_infinity() {
        assert!(matches!(
            Angle::radians(f64::INFINITY),
            Err(AngleError::NonFiniteRadians)
        ));
    }

    #[test]
    fn radians_rejects_negative_infinity() {
        assert!(matches!(
            Angle::radians(f64::NEG_INFINITY),
            Err(AngleError::NonFiniteRadians)
        ));
    }

    #[test]
    fn degrees_convert_to_radians() {
        let angle = Angle::degrees(180.0).expect("180 degrees is valid");

        let radians = angle
            .as_radians()
            .expect("degree conversion is concrete");

        assert!((radians - PI_RADIANS).abs() <= f64::EPSILON);
    }

    #[test]
    fn turns_convert_to_radians() {
        let angle = Angle::turns(1.0).expect("one turn is valid");

        let radians = angle
            .as_radians()
            .expect("turn conversion is concrete");

        assert!((radians - TAU_RADIANS).abs() <= f64::EPSILON);
    }

    #[test]
    fn degree_conversion_rejects_non_finite_input() {
        assert!(matches!(
            Angle::degrees(f64::NAN),
            Err(AngleError::NonFiniteDegrees)
        ));

        assert!(matches!(
            Angle::degrees(f64::INFINITY),
            Err(AngleError::NonFiniteDegrees)
        ));
    }

    #[test]
    fn turn_conversion_rejects_non_finite_input() {
        assert!(matches!(
            Angle::turns(f64::NAN),
            Err(AngleError::NonFiniteTurns)
        ));

        assert!(matches!(
            Angle::turns(f64::INFINITY),
            Err(AngleError::NonFiniteTurns)
        ));
    }

    // -------------------------------------------------------------------------
    // Built-in constants
    // -------------------------------------------------------------------------

    #[test]
    fn pi_is_symbolic_before_evaluation() {
        let angle = Angle::pi();

        assert!(angle.is_symbolic());
        assert_eq!(
            angle.parameter_canonical_string(),
            "pi"
        );
    }

    #[test]
    fn tau_is_symbolic_before_evaluation() {
        let angle = Angle::tau();

        assert!(angle.is_symbolic());
        assert_eq!(
            angle.parameter_canonical_string(),
            "tau"
        );
    }

    #[test]
    fn pi_evaluates_to_pi() {
        let angle = Angle::pi();
        let bindings = ParameterBindings::new();

        let value = angle
            .evaluate(&bindings)
            .expect("pi must evaluate");

        assert_eq!(value, PI_RADIANS);
    }

    #[test]
    fn tau_evaluates_to_tau() {
        let angle = Angle::tau();
        let bindings = ParameterBindings::new();

        let value = angle
            .evaluate(&bindings)
            .expect("tau must evaluate");

        assert_eq!(value, TAU_RADIANS);
    }

    #[test]
    fn half_pi_evaluates_correctly() {
        let angle = Angle::half_pi();
        let bindings = ParameterBindings::new();

        let value = angle
            .evaluate(&bindings)
            .expect("pi/2 must evaluate");

        assert!((value - PI_RADIANS / 2.0).abs() <= f64::EPSILON);
    }

    #[test]
    fn quarter_pi_evaluates_correctly() {
        let angle = Angle::quarter_pi();
        let bindings = ParameterBindings::new();

        let value = angle
            .evaluate(&bindings)
            .expect("pi/4 must evaluate");

        assert!((value - PI_RADIANS / 4.0).abs() <= f64::EPSILON);
    }

    // -------------------------------------------------------------------------
    // Symbolic parameters
    // -------------------------------------------------------------------------

    #[test]
    fn symbolic_angle_is_preserved() {
        let angle =
            Angle::symbol("theta").expect("valid symbol");

        assert!(angle.is_symbolic());
        assert_eq!(
            angle.parameter_canonical_string(),
            "theta"
        );
        assert_eq!(
            angle.symbols(),
            vec!["theta".to_owned()]
        );
    }

    #[test]
    fn symbolic_angle_evaluates_with_binding() {
        let angle =
            Angle::symbol("theta").expect("valid symbol");

        let mut bindings = ParameterBindings::new();

        bindings
            .insert("theta", PI_RADIANS / 2.0)
            .expect("finite binding");

        let value = angle
            .evaluate(&bindings)
            .expect("theta must be bound");

        assert!((value - PI_RADIANS / 2.0).abs() <= f64::EPSILON);
    }

    #[test]
    fn missing_symbol_is_reported() {
        let angle =
            Angle::symbol("theta").expect("valid symbol");

        let bindings = ParameterBindings::new();

        assert!(angle.evaluate(&bindings).is_err());
    }

    // -------------------------------------------------------------------------
    // Arithmetic
    // -------------------------------------------------------------------------

    #[test]
    fn angle_addition_preserves_concrete_values() {
        let lhs =
            Angle::radians(1.0).expect("finite angle");

        let rhs =
            Angle::radians(2.0).expect("finite angle");

        let result = lhs + rhs;

        assert_eq!(result.as_radians(), Some(3.0));
    }

    #[test]
    fn angle_subtraction_preserves_concrete_values() {
        let lhs =
            Angle::radians(1.0).expect("finite angle");

        let rhs =
            Angle::radians(2.0).expect("finite angle");

        let result = lhs - rhs;

        assert_eq!(result.as_radians(), Some(-1.0));
    }

    #[test]
    fn angle_negation_preserves_semantics() {
        let angle =
            Angle::radians(1.5).expect("finite angle");

        let result = -angle;

        assert_eq!(result.as_radians(), Some(-1.5));
    }

    #[test]
    fn symbolic_addition_remains_symbolic() {
        let theta =
            Angle::symbol("theta").expect("valid symbol");

        let result = theta.add(&Angle::pi());

        assert!(result.is_symbolic());
    }

    #[test]
    fn scaling_rejects_non_finite_factor() {
        let angle = Angle::pi();

        assert!(matches!(
            angle.scaled(f64::NAN),
            Err(AngleError::NonFiniteScale)
        ));

        assert!(matches!(
            angle.scaled(f64::INFINITY),
            Err(AngleError::NonFiniteScale)
        ));
    }

    #[test]
    fn division_rejects_zero() {
        let angle = Angle::pi();

        assert!(matches!(
            angle.divided_by(0.0),
            Err(AngleError::DivisionByZero)
        ));
    }

    #[test]
    fn division_rejects_non_finite_divisor() {
        let angle = Angle::pi();

        assert!(matches!(
            angle.divided_by(f64::NAN),
            Err(AngleError::NonFiniteScale)
        ));

        assert!(matches!(
            angle.divided_by(f64::INFINITY),
            Err(AngleError::NonFiniteScale)
        ));
    }

    #[test]
    fn symbolic_scaling_remains_symbolic() {
        let theta =
            Angle::symbol("theta").expect("valid symbol");

        let result =
            theta.scaled(2.0).expect("finite scale");

        assert!(result.is_symbolic());
    }

    // -------------------------------------------------------------------------
    // Periodic semantics
    // -------------------------------------------------------------------------

    #[test]
    fn normalize_zero() {
        assert_eq!(
            Angle::normalize_radians(0.0)
                .expect("zero is valid"),
            0.0
        );
    }

    #[test]
    fn normalize_tau_to_zero() {
        assert_eq!(
            Angle::normalize_radians(TAU_RADIANS)
                .expect("tau is valid"),
            0.0
        );
    }

    #[test]
    fn normalize_negative_pi_to_pi() {
        let value =
            Angle::normalize_radians(-PI_RADIANS)
                .expect("negative pi is valid");

        assert!((value - PI_RADIANS).abs() <= f64::EPSILON);
    }

    #[test]
    fn normalize_three_pi_to_pi() {
        let value =
            Angle::normalize_radians(3.0 * PI_RADIANS)
                .expect("3pi is valid");

        assert!((value - PI_RADIANS).abs() <= f64::EPSILON);
    }

    #[test]
    fn normalize_rejects_non_finite_values() {
        assert!(Angle::normalize_radians(f64::NAN).is_err());
        assert!(Angle::normalize_radians(f64::INFINITY).is_err());
        assert!(
            Angle::normalize_radians(f64::NEG_INFINITY)
                .is_err()
        );
    }

    #[test]
    fn equivalent_modulo_tau_recognizes_zero_and_tau() {
        assert!(
            Angle::equivalent_modulo_tau(
                0.0,
                TAU_RADIANS
            )
            .expect("both values are finite")
        );
    }

    #[test]
    fn equivalent_modulo_tau_recognizes_pi_and_three_pi() {
        assert!(
            Angle::equivalent_modulo_tau(
                PI_RADIANS,
                3.0 * PI_RADIANS
            )
            .expect("both values are finite")
        );
    }

    #[test]
    fn equivalent_modulo_tau_does_not_use_hidden_tolerance() {
        let lhs = 1.0;
        let rhs = 1.0 + 4.0 * f64::EPSILON;

        assert!(
            !Angle::equivalent_modulo_tau(lhs, rhs)
                .expect("both values are finite")
        );
    }

    // -------------------------------------------------------------------------
    // Canonical representation
    // -------------------------------------------------------------------------

    #[test]
    fn canonical_string_is_deterministic() {
        let angle =
            Angle::symbol("theta").expect("valid symbol");

        assert_eq!(
            angle.canonical_string(),
            "angle(theta)"
        );
    }

    #[test]
    fn pi_canonical_string_is_deterministic() {
        assert_eq!(
            Angle::pi().canonical_string(),
            "angle(pi)"
        );
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    #[test]
    fn concrete_angle_validates() {
        let angle =
            Angle::radians(1.0).expect("finite angle");

        angle.validate().expect("valid angle");
    }

    #[test]
    fn symbolic_angle_validates() {
        let angle =
            Angle::symbol("theta").expect("valid symbol");

        angle.validate().expect("valid symbolic angle");
    }

    // -------------------------------------------------------------------------
    // Resolver isolation
    // -------------------------------------------------------------------------

    #[test]
    fn evaluation_has_no_global_mutable_state() {
        let angle =
            Angle::symbol("theta").expect("valid symbol");

        let resolver =
            |name: &str| {
                if name == "theta" {
                    Some(0.75)
                } else {
                    None
                }
            };

        let value = angle
            .evaluate_with_resolver(&resolver)
            .expect("resolver supplies theta");

        assert_eq!(value, 0.75);
    }

    #[test]
    fn built_in_pi_is_resolved_without_binding() {
        let angle =
            Angle::symbol("pi").expect("valid symbol");

        let bindings = ParameterBindings::new();

        let value = angle
            .evaluate(&bindings)
            .expect("pi is built in");

        assert_eq!(value, PI_RADIANS);
    }

    #[test]
    fn built_in_tau_is_resolved_without_binding() {
        let angle =
            Angle::symbol("tau").expect("valid symbol");

        let bindings = ParameterBindings::new();

        let value = angle
            .evaluate(&bindings)
            .expect("tau is built in");

        assert_eq!(value, TAU_RADIANS);
    }

    // -------------------------------------------------------------------------
    // Normalized evaluation
    // -------------------------------------------------------------------------

    #[test]
    fn symbolic_pi_normalized_is_pi() {
        let angle = Angle::pi();
        let bindings = ParameterBindings::new();

        let value = angle
            .evaluate_normalized(&bindings)
            .expect("pi must normalize");

        assert!((value - PI_RADIANS).abs() <= f64::EPSILON);
    }

    #[test]
    fn symbolic_tau_normalized_is_zero() {
        let angle = Angle::tau();
        let bindings = ParameterBindings::new();

        let value = angle
            .evaluate_normalized(&bindings)
            .expect("tau must normalize");

        assert_eq!(value, 0.0);
    }
}