//! Zamani Quantum IR — Canonical Typed Value System
//!
//! Hardware-independent, deterministic, resource-safe values for the
//! Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `value.rs` defines the canonical representation of values that can occur
//! in the Quantum IR.
//!
//! It owns:
//!
//! - scalar values;
//! - finite floating-point values;
//! - arbitrary-width signed/unsigned integer values representable by the
//!   standard Rust integer types;
//! - booleans;
//! - complex scalar values;
//! - angles;
//! - durations;
//! - frequencies;
//! - amplitudes;
//! - phases;
//! - logical qubit references;
//! - physical qubit references;
//! - symbolic parameters;
//! - IR value references;
//! - homogeneous arrays;
//! - tuples;
//! - optional values;
//! - deterministic structural value operations.
//!
//! It does NOT own:
//!
//! - gate semantics;
//! - measurement semantics;
//! - circuit construction;
//! - control-flow semantics;
//! - pulse generation;
//! - hardware calibration;
//! - physical hardware topology;
//! - routing;
//! - scheduling policy;
//! - backend execution;
//! - simulation state;
//! - frontend parsing.
//!
//! Those responsibilities belong to their corresponding IR or downstream
//! modules.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::identity
//!          │
//!          └──────────────┐
//!                         ▼
//! quantum::ir::qubit ──► value.rs ◄── quantum::ir::parameter
//!                         │
//!             ┌───────────┼───────────────┐
//!             ▼           ▼               ▼
//!          operation    pulse           timing
//!             │           │               │
//!             └───────────┼───────────────┘
//!                         ▼
//!                    program/region
//! ```
//!
//! `value.rs` therefore depends only on foundational identity, parameter and
//! qubit contracts. It must not depend on higher-level IR structures.
//!
//! # Universal-program principle
//!
//! Zamani quantum programs are hardware-independent.
//!
//! A value such as:
//!
//! ```text
//! 0.3
//! ```
//!
//! may be interpreted by a consuming semantic type as an amplitude, angle,
//! probability, coefficient, or another scalar.
//!
//! A value such as:
//!
//! ```text
//! 20ns
//! ```
//!
//! is represented here as a strongly typed duration rather than as an
//! untyped string.
//!
//! Hardware-specific interpretation occurs later.
//!
//! Therefore this module contains no assumptions about:
//!
//! - the number of qubits in a machine;
//! - physical topology;
//! - DAC resolution;
//! - ADC resolution;
//! - hardware clock rate;
//! - pulse generator limits;
//! - device-specific numeric widths.
//!
//! # Scalability
//!
//! There is NO architectural maximum number of:
//!
//! - qubits;
//! - values;
//! - operations;
//! - arrays;
//! - tuple elements;
//! - program regions;
//! - quantum machines.
//!
//! Concrete resource limits belong to an explicit policy such as
//! `QuantumIrLimits`.
//!
//! This module deliberately does not introduce artificial limits such as:
//!
//! ```text
//! 63
//! 64
//! 4096
//! 1_000_000
//! ```
//!
//! as quantum-machine boundaries.
//!
//! Collection sizes are limited only by the address space and by explicit
//! caller/compiler resource policies.
//!
//! # Numeric safety
//!
//! Floating-point values represented by semantic scalar types must be finite.
//!
//! NaN and positive/negative infinity are rejected by checked constructors.
//!
//! Integer conversions use checked conversion.
//!
//! Duration/frequency/amplitude/phase constructors reject invalid values.
//!
//! # Floating-point equality
//!
//! Semantic value equality for floating-point values is intentionally based on
//! their IEEE-754 bit representation after construction has guaranteed that
//! the value is finite.
//!
//! This gives deterministic behavior for:
//!
//! - hashing;
//! - equality;
//! - canonical caches;
//! - structural comparisons.
//!
//! In particular, `-0.0` and `+0.0` remain distinguishable at the raw value
//! layer because they have different IEEE-754 representations.
//!
//! # Units
//!
//! This module provides canonical semantic unit wrappers but does not define
//! hardware conversion rules.
//!
//! The canonical units are:
//!
//! - duration: attoseconds;
//! - frequency: femtohertz;
//! - angle: radians;
//! - amplitude: unit-neutral finite scalar;
//! - phase: radians.
//!
//! Using integer base units avoids floating-point accumulation for duration,
//! frequency and angle storage.
//!
//! # Quantum identity boundary
//!
//! Logical and physical qubits are imported directly from:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! No duplicate qubit identifier type is defined here.
//!
//! This is essential because `qubit.rs` is the canonical owner of those
//! identity domains.
//!
//! # Parameter boundary
//!
//! Symbolic and arithmetic parameter semantics remain owned by:
//!
//! ```text
//! quantum::ir::parameter::Parameter
//! ```
//!
//! `Value::Parameter` embeds that canonical parameter representation rather
//! than duplicating the parameter AST.
//!
//! # Value identity boundary
//!
//! `ValueId` remains owned by:
//!
//! ```text
//! quantum::ir::identity::ValueId
//! ```
//!
//! `Value::Reference(ValueId)` refers to an existing IR value. It does not
//! itself define SSA, dominance, use-def chains, or block semantics.
//!
//! Those belong to `region.rs`, `operation.rs`, and `program.rs`.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe` code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `parameter.rs` supplies [`Parameter`].
//!
//! `qubit.rs` supplies [`QubitId`] and [`PhysicalQubitId`].
//!
//! `identity.rs` supplies [`ValueId`].
//!
//! `operation.rs` should use [`Value`] for typed operands/results where
//! appropriate.
//!
//! `gate.rs` may consume scalar, angle and parameter values.
//!
//! `measurement.rs` may consume qubit and classical-result-related values.
//!
//! `pulse.rs` may consume [`Value::Amplitude`], [`Value::Duration`],
//! [`Value::Frequency`] and [`Value::Phase`].
//!
//! `timing.rs` may consume [`Duration`].
//!
//! `channel.rs` and `frame.rs` may use [`Value::Reference`] and parameterized
//! values without creating another generic value representation.
//!
//! `control_flow.rs` may use [`Value::Bool`], [`Value::Parameter`] and
//! [`Value::Reference`] as condition operands.
//!
//! `program.rs` and `region.rs` may store `Value` instances.
//!
//! `serialization.rs` should serialize this enum structurally.
//!
//! `hash.rs` can use [`Value::canonical_hash`].
//!
//! `validation.rs` can use [`Value::validate`].
//!
//! `analysis.rs` can inspect value kinds without interpreting hardware.
//!
//! No future hardware technology should require changing this file merely
//! because a new quantum device architecture is introduced.

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

use super::identity::ValueId;
use super::parameter::Parameter;
use super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Constants
// =============================================================================

/// Number of attoseconds in one second.
///
/// This is a unit conversion constant, not a machine-size limit.
pub const ATTOSECONDS_PER_SECOND: u128 = 1_000_000_000_000_000_000;

/// Number of femtohertz in one hertz.
///
/// This is a unit conversion constant, not a machine-size limit.
pub const FEMTOHERTZ_PER_HERTZ: u128 = 1_000_000_000_000;

/// Number of femtohertz in one megahertz.
pub const FEMTOHERTZ_PER_MHZ: u128 = 1_000_000_000_000_000;

/// Number of femtohertz in one gigahertz.
pub const FEMTOHERTZ_PER_GHZ: u128 = 1_000_000_000_000_000_000;

/// Maximum finite `f64` value.
pub const MAX_FINITE_F64: f64 = f64::MAX;

// =============================================================================
// Value kind
// =============================================================================

/// Broad semantic category of a [`Value`].
///
/// This is intentionally independent of `type.rs`.
///
/// `type.rs` will eventually own the canonical IR type system, while
/// `ValueKind` remains a lightweight classification useful for diagnostics,
/// validation, pattern matching and serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ValueKind {
    /// Boolean value.
    Bool,

    /// Signed integer.
    Integer,

    /// Unsigned integer.
    UnsignedInteger,

    /// Finite floating-point scalar.
    Float,

    /// Complex scalar.
    Complex,

    /// Angle measured in radians.
    Angle,

    /// Time duration.
    Duration,

    /// Frequency.
    Frequency,

    /// Hardware-independent amplitude scalar.
    Amplitude,

    /// Phase measured in radians.
    Phase,

    /// Logical qubit.
    Qubit,

    /// Physical qubit.
    PhysicalQubit,

    /// Symbolic/runtime parameter.
    Parameter,

    /// Reference to another IR value.
    Reference,

    /// Homogeneous array.
    Array,

    /// Heterogeneous ordered tuple.
    Tuple,

    /// Optional value.
    Optional,

    /// Explicit unitless null value.
    Unit,
}

impl fmt::Display for ValueKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Bool => "bool",
            Self::Integer => "integer",
            Self::UnsignedInteger => "unsigned_integer",
            Self::Float => "float",
            Self::Complex => "complex",
            Self::Angle => "angle",
            Self::Duration => "duration",
            Self::Frequency => "frequency",
            Self::Amplitude => "amplitude",
            Self::Phase => "phase",
            Self::Qubit => "qubit",
            Self::PhysicalQubit => "physical_qubit",
            Self::Parameter => "parameter",
            Self::Reference => "reference",
            Self::Array => "array",
            Self::Tuple => "tuple",
            Self::Optional => "optional",
            Self::Unit => "unit",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Value error
// =============================================================================

/// Error returned by checked value construction and conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueError {
    /// A floating-point value was NaN or infinite.
    NonFiniteFloat,

    /// A numeric conversion cannot represent the requested value.
    NumericOverflow,

    /// A numeric conversion would lose information.
    NumericLossOfPrecision,

    /// A duration was invalid.
    InvalidDuration,

    /// A frequency was invalid.
    InvalidFrequency,

    /// An angle was invalid.
    InvalidAngle,

    /// An amplitude was invalid.
    InvalidAmplitude,

    /// A phase was invalid.
    InvalidPhase,

    /// A collection operation would overflow its length.
    CollectionSizeOverflow,

    /// A value has an invalid recursive structure.
    InvalidStructure,

    /// A value contains an unsupported nested value.
    UnsupportedNestedValue,

    /// A value is not the expected kind.
    TypeMismatch {
        /// Expected kind.
        expected: ValueKind,

        /// Actual kind.
        actual: ValueKind,
    },
}

impl fmt::Display for ValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteFloat => {
                formatter.write_str("floating-point value must be finite")
            }

            Self::NumericOverflow => {
                formatter.write_str("numeric conversion overflowed")
            }

            Self::NumericLossOfPrecision => {
                formatter.write_str("numeric conversion would lose precision")
            }

            Self::InvalidDuration => {
                formatter.write_str("duration is invalid")
            }

            Self::InvalidFrequency => {
                formatter.write_str("frequency is invalid")
            }

            Self::InvalidAngle => {
                formatter.write_str("angle is invalid")
            }

            Self::InvalidAmplitude => {
                formatter.write_str("amplitude is invalid")
            }

            Self::InvalidPhase => {
                formatter.write_str("phase is invalid")
            }

            Self::CollectionSizeOverflow => {
                formatter.write_str("collection size overflowed")
            }

            Self::InvalidStructure => {
                formatter.write_str("value structure is invalid")
            }

            Self::UnsupportedNestedValue => {
                formatter.write_str("nested value is unsupported")
            }

            Self::TypeMismatch { expected, actual } => {
                write!(
                    formatter,
                    "value type mismatch: expected {expected}, found {actual}"
                )
            }
        }
    }
}

impl std::error::Error for ValueError {}

// =============================================================================
// Finite float
// =============================================================================

/// A finite IEEE-754 `f64`.
///
/// This wrapper exists so semantic values cannot accidentally contain NaN or
/// infinity after construction.
///
/// Equality and hashing are based on IEEE-754 bit representation.
#[derive(Clone, Copy, Debug)]
pub struct FiniteFloat(f64);

impl FiniteFloat {
    /// Creates a finite floating-point value.
    pub fn new(value: f64) -> Result<Self, ValueError> {
        if !value.is_finite() {
            return Err(ValueError::NonFiniteFloat);
        }

        Ok(Self(value))
    }

    /// Creates a finite value in a constant context.
    ///
    /// This is intentionally not `const` because Rust's stable const-float
    /// support is not required by the IR contract.
    #[must_use]
    pub fn get(self) -> f64 {
        self.0
    }

    /// Returns the IEEE-754 bit representation.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0.to_bits()
    }

    /// Returns whether this value is positive or negative zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns the absolute value.
    ///
    /// The result remains finite because the input is finite.
    #[must_use]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Returns the underlying value as `f64`.
    #[must_use]
    pub fn as_f64(self) -> f64 {
        self.0
    }
}

impl PartialEq for FiniteFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for FiniteFloat {}

impl Hash for FiniteFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

impl PartialOrd for FiniteFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for FiniteFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl fmt::Display for FiniteFloat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Complex scalar
// =============================================================================

/// Finite complex scalar.
///
/// Both real and imaginary components must be finite.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComplexValue {
    real: FiniteFloat,
    imaginary: FiniteFloat,
}

impl ComplexValue {
    /// Creates a finite complex scalar.
    pub fn new(real: f64, imaginary: f64) -> Result<Self, ValueError> {
        Ok(Self {
            real: FiniteFloat::new(real)?,
            imaginary: FiniteFloat::new(imaginary)?,
        })
    }

    /// Creates a complex value from finite components.
    #[must_use]
    pub const fn from_finite(
        real: FiniteFloat,
        imaginary: FiniteFloat,
    ) -> Self {
        Self { real, imaginary }
    }

    /// Returns the real component.
    #[must_use]
    pub const fn real(self) -> FiniteFloat {
        self.real
    }

    /// Returns the imaginary component.
    #[must_use]
    pub const fn imaginary(self) -> FiniteFloat {
        self.imaginary
    }

    /// Returns `(real, imaginary)`.
    #[must_use]
    pub const fn components(self) -> (FiniteFloat, FiniteFloat) {
        (self.real, self.imaginary)
    }
}

impl fmt::Display for ComplexValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}{}{}i",
            self.real,
            if self.imaginary.get() < 0.0 {
                ""
            } else {
                "+"
            },
            self.imaginary
        )
    }
}

// =============================================================================
// Angle
// =============================================================================

/// Angle stored canonically in radians.
///
/// The value must be finite.
///
/// No normalization to `[0, 2π)` is performed because normalization can change
/// symbolic/semantic information and should be an explicit optimization or
/// canonicalization decision.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Angle(FiniteFloat);

impl Angle {
    /// Creates an angle in radians.
    pub fn radians(value: f64) -> Result<Self, ValueError> {
        Ok(Self(FiniteFloat::new(value)?))
    }

    /// Creates an angle from a finite scalar.
    #[must_use]
    pub const fn from_finite(value: FiniteFloat) -> Self {
        Self(value)
    }

    /// Returns the angle in radians.
    #[must_use]
    pub const fn radians_value(self) -> FiniteFloat {
        self.0
    }

    /// Returns the angle as `f64`.
    #[must_use]
    pub fn as_f64(self) -> f64 {
        self.0.get()
    }

    /// Returns zero radians.
    #[must_use]
    pub fn zero() -> Self {
        Self(FiniteFloat(0.0))
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}rad", self.0)
    }
}

// =============================================================================
// Phase
// =============================================================================

/// Phase stored canonically in radians.
///
/// Unlike `Angle`, `Phase` identifies the semantic role of the scalar in
/// control/pulse contexts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Phase(FiniteFloat);

impl Phase {
    /// Creates a phase in radians.
    pub fn radians(value: f64) -> Result<Self, ValueError> {
        Ok(Self(FiniteFloat::new(value)?))
    }

    /// Creates a phase from a finite scalar.
    #[must_use]
    pub const fn from_finite(value: FiniteFloat) -> Self {
        Self(value)
    }

    /// Returns the phase in radians.
    #[must_use]
    pub const fn radians_value(self) -> FiniteFloat {
        self.0
    }

    /// Returns the phase as `f64`.
    #[must_use]
    pub fn as_f64(self) -> f64 {
        self.0.get()
    }

    /// Returns zero phase.
    #[must_use]
    pub fn zero() -> Self {
        Self(FiniteFloat(0.0))
    }
}

impl fmt::Display for Phase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}rad", self.0)
    }
}

// =============================================================================
// Amplitude
// =============================================================================

/// Hardware-independent finite amplitude scalar.
///
/// The IR does not assume a device-specific amplitude unit.
///
/// A hardware backend may later map this value into its own calibrated
/// amplitude domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Amplitude(FiniteFloat);

impl Amplitude {
    /// Creates an amplitude.
    ///
    /// Any finite scalar is representable at the semantic IR layer.
    pub fn new(value: f64) -> Result<Self, ValueError> {
        Ok(Self(FiniteFloat::new(value)?))
    }

    /// Creates an amplitude from a finite scalar.
    #[must_use]
    pub const fn from_finite(value: FiniteFloat) -> Self {
        Self(value)
    }

    /// Returns the amplitude as `f64`.
    #[must_use]
    pub fn as_f64(self) -> f64 {
        self.0.get()
    }

    /// Returns the underlying finite scalar.
    #[must_use]
    pub const fn value(self) -> FiniteFloat {
        self.0
    }
}

impl fmt::Display for Amplitude {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Duration
// =============================================================================

/// Non-negative duration represented exactly in attoseconds.
///
/// Integer storage avoids floating-point accumulation and gives deterministic
/// serialization and comparison.
///
/// A duration of zero is valid.
///
/// The representation can cover a very large finite interval without being
/// tied to a particular machine's clock width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration(u128);

impl Duration {
    /// Creates a duration from attoseconds.
    #[must_use]
    pub const fn attoseconds(value: u128) -> Self {
        Self(value)
    }

    /// Creates a duration from femtoseconds.
    pub fn femtoseconds(value: u128) -> Result<Self, ValueError> {
        let attoseconds = value
            .checked_mul(1_000)
            .ok_or(ValueError::NumericOverflow)?;

        Ok(Self(attoseconds))
    }

    /// Creates a duration from picoseconds.
    pub fn picoseconds(value: u128) -> Result<Self, ValueError> {
        let attoseconds = value
            .checked_mul(1_000_000)
            .ok_or(ValueError::NumericOverflow)?;

        Ok(Self(attoseconds))
    }

    /// Creates a duration from nanoseconds.
    pub fn nanoseconds(value: u128) -> Result<Self, ValueError> {
        let attoseconds = value
            .checked_mul(1_000_000_000)
            .ok_or(ValueError::NumericOverflow)?;

        Ok(Self(attoseconds))
    }

    /// Creates a duration from microseconds.
    pub fn microseconds(value: u128) -> Result<Self, ValueError> {
        let attoseconds = value
            .checked_mul(1_000_000_000_000)
            .ok_or(ValueError::NumericOverflow)?;

        Ok(Self(attoseconds))
    }

    /// Creates a duration from milliseconds.
    pub fn milliseconds(value: u128) -> Result<Self, ValueError> {
        let attoseconds = value
            .checked_mul(1_000_000_000_000_000)
            .ok_or(ValueError::NumericOverflow)?;

        Ok(Self(attoseconds))
    }

    /// Creates a duration from seconds.
    pub fn seconds(value: u128) -> Result<Self, ValueError> {
        let attoseconds = value
            .checked_mul(ATTOSECONDS_PER_SECOND)
            .ok_or(ValueError::NumericOverflow)?;

        Ok(Self(attoseconds))
    }

    /// Returns the exact duration in attoseconds.
    #[must_use]
    pub const fn as_attoseconds(self) -> u128 {
        self.0
    }

    /// Returns whether the duration is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Adds two durations with overflow checking.
    pub fn checked_add(self, other: Self) -> Result<Self, ValueError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Subtracts two durations with underflow checking.
    pub fn checked_sub(self, other: Self) -> Result<Self, ValueError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(ValueError::InvalidDuration)
    }

    /// Multiplies a duration by an unsigned scalar.
    pub fn checked_mul(self, multiplier: u128) -> Result<Self, ValueError> {
        self.0
            .checked_mul(multiplier)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Returns the duration as whole nanoseconds and remaining attoseconds.
    #[must_use]
    pub fn split_nanoseconds(self) -> (u128, u128) {
        (self.0 / 1_000_000_000, self.0 % 1_000_000_000)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 % 1_000_000_000 == 0 {
            write!(formatter, "{}ns", self.0 / 1_000_000_000)
        } else {
            write!(formatter, "{}as", self.0)
        }
    }
}

// =============================================================================
// Frequency
// =============================================================================

/// Frequency represented exactly in femtohertz.
///
/// Frequency is required to be non-negative at the canonical value layer.
///
/// Signed frequency shifts should be represented by a parameter/expression or
/// by a future explicitly signed frequency type rather than by violating the
/// invariant of this type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Frequency(u128);

impl Frequency {
    /// Creates a frequency in femtohertz.
    #[must_use]
    pub const fn femtohertz(value: u128) -> Self {
        Self(value)
    }

    /// Creates a frequency in hertz.
    pub fn hertz(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(FEMTOHERTZ_PER_HERTZ)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Creates a frequency in megahertz.
    pub fn megahertz(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(FEMTOHERTZ_PER_MHZ)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Creates a frequency in gigahertz.
    pub fn gigahertz(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(FEMTOHERTZ_PER_GHZ)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Returns femtohertz.
    #[must_use]
    pub const fn as_femtohertz(self) -> u128 {
        self.0
    }

    /// Returns whether this frequency is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Adds frequencies with overflow checking.
    pub fn checked_add(self, other: Self) -> Result<Self, ValueError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Subtracts frequencies with underflow checking.
    pub fn checked_sub(self, other: Self) -> Result<Self, ValueError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(ValueError::InvalidFrequency)
    }
}

impl fmt::Display for Frequency {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 % FEMTOHERTZ_PER_GHZ == 0 {
            write!(
                formatter,
                "{}GHz",
                self.0 / FEMTOHERTZ_PER_GHZ
            )
        } else if self.0 % FEMTOHERTZ_PER_MHZ == 0 {
            write!(
                formatter,
                "{}MHz",
                self.0 / FEMTOHERTZ_PER_MHZ
            )
        } else if self.0 % FEMTOHERTZ_PER_HERTZ == 0 {
            write!(
                formatter,
                "{}Hz",
                self.0 / FEMTOHERTZ_PER_HERTZ
            )
        } else {
            write!(formatter, "{}fHz", self.0)
        }
    }
}

// =============================================================================
// Integer representation
// =============================================================================

/// Canonical signed integer value.
///
/// `i128` provides a stable, fixed-width semantic integer domain while keeping
/// the IR independent of the host pointer width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Integer(i128);

impl Integer {
    /// Creates an integer.
    #[must_use]
    pub const fn new(value: i128) -> Self {
        Self(value)
    }

    /// Returns the integer.
    #[must_use]
    pub const fn value(self) -> i128 {
        self.0
    }

    /// Checked addition.
    pub fn checked_add(self, other: Self) -> Result<Self, ValueError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked subtraction.
    pub fn checked_sub(self, other: Self) -> Result<Self, ValueError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked multiplication.
    pub fn checked_mul(self, other: Self) -> Result<Self, ValueError> {
        self.0
            .checked_mul(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked division.
    pub fn checked_div(self, other: Self) -> Result<Self, ValueError> {
        if other.0 == 0 {
            return Err(ValueError::InvalidStructure);
        }

        self.0
            .checked_div(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Unsigned integer representation
// =============================================================================

/// Canonical unsigned integer value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnsignedInteger(u128);

impl UnsignedInteger {
    /// Creates an unsigned integer.
    #[must_use]
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the integer.
    #[must_use]
    pub const fn value(self) -> u128 {
        self.0
    }

    /// Checked addition.
    pub fn checked_add(self, other: Self) -> Result<Self, ValueError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked subtraction.
    pub fn checked_sub(self, other: Self) -> Result<Self, ValueError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked multiplication.
    pub fn checked_mul(self, other: Self) -> Result<Self, ValueError> {
        self.0
            .checked_mul(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked division.
    pub fn checked_div(self, other: Self) -> Result<Self, ValueError> {
        if other.0 == 0 {
            return Err(ValueError::InvalidStructure);
        }

        self.0
            .checked_div(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }
}

impl fmt::Display for UnsignedInteger {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Array
// =============================================================================

/// Canonical homogeneous array value.
///
/// The element kind is recorded explicitly so validation and serialization do
/// not need to infer it from the first element.
///
/// Arrays are ordinary owned IR data. They do not imply allocation on a QPU.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ValueArray {
    element_kind: ValueKind,
    elements: Vec<Value>,
}

impl ValueArray {
    /// Creates an empty array for a known element kind.
    #[must_use]
    pub const fn empty(element_kind: ValueKind) -> Self {
        Self {
            element_kind,
            elements: Vec::new(),
        }
    }

    /// Creates an array after checking that every element has the same kind.
    pub fn new(elements: Vec<Value>) -> Result<Self, ValueError> {
        let element_kind = match elements.first() {
            Some(value) => value.kind(),
            None => ValueKind::Unit,
        };

        for value in &elements {
            if value.kind() != element_kind {
                return Err(ValueError::TypeMismatch {
                    expected: element_kind,
                    actual: value.kind(),
                });
            }
        }

        Ok(Self {
            element_kind,
            elements,
        })
    }

    /// Creates an array with an explicitly declared element kind.
    ///
    /// This is useful for empty arrays where no element exists from which the
    /// kind can be inferred.
    pub fn with_kind(
        element_kind: ValueKind,
        elements: Vec<Value>,
    ) -> Result<Self, ValueError> {
        for value in &elements {
            if value.kind() != element_kind {
                return Err(ValueError::TypeMismatch {
                    expected: element_kind,
                    actual: value.kind(),
                });
            }
        }

        Ok(Self {
            element_kind,
            elements,
        })
    }

    /// Returns the element kind.
    #[must_use]
    pub const fn element_kind(&self) -> ValueKind {
        self.element_kind
    }

    /// Returns the number of elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the array is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns an element by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.elements.get(index)
    }

    /// Returns an iterator over elements.
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.elements.iter()
    }

    /// Returns the owned elements.
    #[must_use]
    pub fn into_elements(self) -> Vec<Value> {
        self.elements
    }

    /// Appends an element after checking its kind.
    pub fn push(&mut self, value: Value) -> Result<(), ValueError> {
        if value.kind() != self.element_kind {
            return Err(ValueError::TypeMismatch {
                expected: self.element_kind,
                actual: value.kind(),
            });
        }

        self.elements.push(value);
        Ok(())
    }
}

// =============================================================================
// Tuple
// =============================================================================

/// Canonical heterogeneous ordered tuple.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ValueTuple(Vec<Value>);

impl ValueTuple {
    /// Creates a tuple.
    #[must_use]
    pub fn new(elements: Vec<Value>) -> Self {
        Self(elements)
    }

    /// Returns the number of tuple elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the tuple is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an element.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.0.get(index)
    }

    /// Returns an iterator.
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }

    /// Returns owned elements.
    #[must_use]
    pub fn into_elements(self) -> Vec<Value> {
        self.0
    }
}

// =============================================================================
// Optional
// =============================================================================

/// Canonical optional value.
///
/// `None` has no contained value and therefore carries no runtime payload.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OptionalValue {
    /// No value is present.
    None,

    /// A value is present.
    Some(Box<Value>),
}

impl OptionalValue {
    /// Creates an empty optional.
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }

    /// Creates a populated optional.
    #[must_use]
    pub fn some(value: Value) -> Self {
        Self::Some(Box::new(value))
    }

    /// Returns whether a value is present.
    #[must_use]
    pub const fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    /// Returns whether no value is present.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns the contained value.
    #[must_use]
    pub fn as_ref(&self) -> Option<&Value> {
        match self {
            Self::None => None,
            Self::Some(value) => Some(value.as_ref()),
        }
    }
}

// =============================================================================
// Canonical Value
// =============================================================================

/// Canonical typed value in the Zamani Quantum IR.
///
/// This is the central value vocabulary shared by IR components.
///
/// It is deliberately broader than gate parameters because a universal
/// quantum-program IR must represent values used by:
///
/// - gates;
/// - measurements;
/// - dynamic control;
/// - pulse programs;
/// - timing;
/// - resource declarations;
/// - mappings;
/// - runtime parameters;
/// - classical expressions;
/// - structured program regions.
///
/// It does not represent a quantum state vector or density matrix.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    /// Boolean value.
    Bool(bool),

    /// Signed integer.
    Integer(Integer),

    /// Unsigned integer.
    UnsignedInteger(UnsignedInteger),

    /// Finite floating-point scalar.
    Float(FiniteFloat),

    /// Finite complex scalar.
    Complex(ComplexValue),

    /// Angle in radians.
    Angle(Angle),

    /// Duration.
    Duration(Duration),

    /// Frequency.
    Frequency(Frequency),

    /// Hardware-independent amplitude.
    Amplitude(Amplitude),

    /// Phase in radians.
    Phase(Phase),

    /// Logical qubit reference.
    ///
    /// Uses the canonical type from `quantum::ir::qubit`.
    Qubit(QubitId),

    /// Physical qubit reference.
    ///
    /// This identifies a physical-qubit namespace value, not a statement
    /// that the referenced hardware exists or is available.
    PhysicalQubit(PhysicalQubitId),

    /// Symbolic/runtime parameter.
    Parameter(Parameter),

    /// Reference to another IR value.
    Reference(ValueId),

    /// Homogeneous array.
    Array(ValueArray),

    /// Heterogeneous tuple.
    Tuple(ValueTuple),

    /// Optional value.
    Optional(OptionalValue),

    /// Unit value.
    Unit,
}

impl Value {
    // -------------------------------------------------------------------------
    // Constructors
    // -------------------------------------------------------------------------

    /// Creates a boolean value.
    #[must_use]
    pub const fn bool(value: bool) -> Self {
        Self::Bool(value)
    }

    /// Creates a signed integer value.
    #[must_use]
    pub const fn integer(value: i128) -> Self {
        Self::Integer(Integer::new(value))
    }

    /// Creates an unsigned integer value.
    #[must_use]
    pub const fn unsigned_integer(value: u128) -> Self {
        Self::UnsignedInteger(UnsignedInteger::new(value))
    }

    /// Creates a finite floating-point value.
    pub fn float(value: f64) -> Result<Self, ValueError> {
        Ok(Self::Float(FiniteFloat::new(value)?))
    }

    /// Creates a complex value.
    pub fn complex(real: f64, imaginary: f64) -> Result<Self, ValueError> {
        Ok(Self::Complex(ComplexValue::new(real, imaginary)?))
    }

    /// Creates an angle.
    pub fn angle_radians(value: f64) -> Result<Self, ValueError> {
        Ok(Self::Angle(Angle::radians(value)?))
    }

    /// Creates an exact duration in attoseconds.
    #[must_use]
    pub const fn duration_attoseconds(value: u128) -> Self {
        Self::Duration(Duration::attoseconds(value))
    }

    /// Creates a duration in nanoseconds.
    pub fn duration_nanoseconds(value: u128) -> Result<Self, ValueError> {
        Ok(Self::Duration(Duration::nanoseconds(value)?))
    }

    /// Creates a duration in microseconds.
    pub fn duration_microseconds(value: u128) -> Result<Self, ValueError> {
        Ok(Self::Duration(Duration::microseconds(value)?))
    }

    /// Creates a duration in milliseconds.
    pub fn duration_milliseconds(value: u128) -> Result<Self, ValueError> {
        Ok(Self::Duration(Duration::milliseconds(value)?))
    }

    /// Creates a duration in seconds.
    pub fn duration_seconds(value: u128) -> Result<Self, ValueError> {
        Ok(Self::Duration(Duration::seconds(value)?))
    }

    /// Creates a frequency in hertz.
    pub fn frequency_hertz(value: u128) -> Result<Self, ValueError> {
        Ok(Self::Frequency(Frequency::hertz(value)?))
    }

    /// Creates a frequency in megahertz.
    pub fn frequency_megahertz(value: u128) -> Result<Self, ValueError> {
        Ok(Self::Frequency(Frequency::megahertz(value)?))
    }

    /// Creates a frequency in gigahertz(value: u128) -> Result<Self, ValueError> {
        Ok(Self::Frequency(Frequency::gigahertz(value)?))
    }

    /// Creates an amplitude.
    pub fn amplitude(value: f64) -> Result<Self, ValueError> {
        Ok(Self::Amplitude(Amplitude::new(value)?))
    }

    /// Creates a phase in radians.
    pub fn phase_radians(value: f64) -> Result<Self, ValueError> {
        Ok(Self::Phase(Phase::radians(value)?))
    }

    /// Creates a logical qubit value.
    #[must_use]
    pub const fn qubit(qubit: QubitId) -> Self {
        Self::Qubit(qubit)
    }

    /// Creates a physical-qubit value.
    #[must_use]
    pub const fn physical_qubit(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Creates a symbolic/runtime parameter value.
    #[must_use]
    pub fn parameter(parameter: Parameter) -> Self {
        Self::Parameter(parameter)
    }

    /// Creates a reference to another IR value.
    #[must_use]
    pub const fn reference(value: ValueId) -> Self {
        Self::Reference(value)
    }

    /// Creates a unit value.
    #[must_use]
    pub const fn unit() -> Self {
        Self::Unit
    }

    /// Creates a homogeneous array.
    pub fn array(elements: Vec<Value>) -> Result<Self, ValueError> {
        Ok(Self::Array(ValueArray::new(elements)?))
    }

    /// Creates a homogeneous array with an explicit element kind.
    pub fn array_with_kind(
        kind: ValueKind,
        elements: Vec<Value>,
    ) -> Result<Self, ValueError> {
        Ok(Self::Array(ValueArray::with_kind(
            kind,
            elements,
        )?))
    }

    /// Creates a tuple.
    #[must_use]
    pub fn tuple(elements: Vec<Value>) -> Self {
        Self::Tuple(ValueTuple::new(elements))
    }

    /// Creates `Some(value)`.
    #[must_use]
    pub fn some(value: Value) -> Self {
        Self::Optional(OptionalValue::some(value))
    }

    /// Creates `None`.
    #[must_use]
    pub const fn none() -> Self {
        Self::Optional(OptionalValue::None)
    }

    // -------------------------------------------------------------------------
    // Classification
    // -------------------------------------------------------------------------

    /// Returns the canonical value kind.
    #[must_use]
    pub const fn kind(&self) -> ValueKind {
        match self {
            Self::Bool(_) => ValueKind::Bool,
            Self::Integer(_) => ValueKind::Integer,
            Self::UnsignedInteger(_) => ValueKind::UnsignedInteger,
            Self::Float(_) => ValueKind::Float,
            Self::Complex(_) => ValueKind::Complex,
            Self::Angle(_) => ValueKind::Angle,
            Self::Duration(_) => ValueKind::Duration,
            Self::Frequency(_) => ValueKind::Frequency,
            Self::Amplitude(_) => ValueKind::Amplitude,
            Self::Phase(_) => ValueKind::Phase,
            Self::Qubit(_) => ValueKind::Qubit,
            Self::PhysicalQubit(_) => ValueKind::PhysicalQubit,
            Self::Parameter(_) => ValueKind::Parameter,
            Self::Reference(_) => ValueKind::Reference,
            Self::Array(_) => ValueKind::Array,
            Self::Tuple(_) => ValueKind::Tuple,
            Self::Optional(_) => ValueKind::Optional,
            Self::Unit => ValueKind::Unit,
        }
    }

    /// Returns whether this is a scalar value.
    #[must_use]
    pub const fn is_scalar(&self) -> bool {
        matches!(
            self,
            Self::Bool(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Complex(_)
                | Self::Angle(_)
                | Self::Duration(_)
                | Self::Frequency(_)
                | Self::Amplitude(_)
                | Self::Phase(_)
        )
    }

    /// Returns whether this value is symbolic.
    #[must_use]
    pub fn is_symbolic(&self) -> bool {
        match self {
            Self::Parameter(parameter) => parameter.is_symbolic(),

            Self::Array(array) => {
                array.iter().any(Value::is_symbolic)
            }

            Self::Tuple(tuple) => {
                tuple.iter().any(Value::is_symbolic)
            }

            Self::Optional(optional) => {
                optional
                    .as_ref()
                    .map(Value::is_symbolic)
                    .unwrap_or(false)
            }

            _ => false,
        }
    }

    /// Returns whether this is a compile-time concrete value.
    ///
    /// References are not considered concrete because their value depends on
    /// another IR definition.
    #[must_use]
    pub fn is_concrete(&self) -> bool {
        !self.is_symbolic()
            && !matches!(self, Self::Reference(_))
    }

    // -------------------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------------------

    /// Returns a boolean.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a signed integer.
    #[must_use]
    pub const fn as_integer(&self) -> Option<i128> {
        match self {
            Self::Integer(value) => Some(value.value()),
            _ => None,
        }
    }

    /// Returns an unsigned integer.
    #[must_use]
    pub const fn as_unsigned_integer(&self) -> Option<u128> {
        match self {
            Self::UnsignedInteger(value) => Some(value.value()),
            _ => None,
        }
    }

    /// Returns a finite floating-point scalar.
    #[must_use]
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(value.get()),
            _ => None,
        }
    }

    /// Returns a complex value.
    #[must_use]
    pub const fn as_complex(&self) -> Option<ComplexValue> {
        match self {
            Self::Complex(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns an angle.
    #[must_use]
    pub const fn as_angle(&self) -> Option<Angle> {
        match self {
            Self::Angle(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a duration.
    #[must_use]
    pub const fn as_duration(&self) -> Option<Duration> {
        match self {
            Self::Duration(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a frequency.
    #[must_use]
    pub const fn as_frequency(&self) -> Option<Frequency> {
        match self {
            Self::Frequency(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns an amplitude.
    #[must_use]
    pub const fn as_amplitude(&self) -> Option<Amplitude> {
        match self {
            Self::Amplitude(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a phase.
    #[must_use]
    pub const fn as_phase(&self) -> Option<Phase> {
        match self {
            Self::Phase(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a logical qubit.
    #[must_use]
    pub const fn as_qubit(&self) -> Option<QubitId> {
        match self {
            Self::Qubit(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a physical qubit.
    #[must_use]
    pub const fn as_physical_qubit(
        &self,
    ) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a parameter.
    #[must_use]
    pub fn as_parameter(&self) -> Option<&Parameter> {
        match self {
            Self::Parameter(value) => Some(value),
            _ => None,
        }
    }

    /// Returns a referenced value ID.
    #[must_use]
    pub const fn as_reference(&self) -> Option<ValueId> {
        match self {
            Self::Reference(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns an array.
    #[must_use]
    pub fn as_array(&self) -> Option<&ValueArray> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }

    /// Returns a tuple.
    #[must_use]
    pub fn as_tuple(&self) -> Option<&ValueTuple> {
        match self {
            Self::Tuple(value) => Some(value),
            _ => None,
        }
    }

    /// Returns an optional value.
    #[must_use]
    pub fn as_optional(&self) -> Option<&OptionalValue> {
        match self {
            Self::Optional(value) => Some(value),
            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Validates the complete value tree.
    ///
    /// Validation is iterative rather than recursively calling through the
    /// entire user-provided tree. This avoids making Rust call-stack depth a
    /// hidden limit on large IR values.
    pub fn validate(&self) -> Result<(), ValueError> {
        let mut stack = vec![self];

        while let Some(value) = stack.pop() {
            match value {
                Self::Bool(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Qubit(_)
                | Self::PhysicalQubit(_)
                | Self::Reference(_)
                | Self::Unit => {}

                Self::Float(value) => {
                    if !value.get().is_finite() {
                        return Err(ValueError::NonFiniteFloat);
                    }
                }

                Self::Complex(value) => {
                    if !value.real().get().is_finite()
                        || !value.imaginary().get().is_finite()
                    {
                        return Err(ValueError::NonFiniteFloat);
                    }
                }

                Self::Angle(value) => {
                    if !value.as_f64().is_finite() {
                        return Err(ValueError::InvalidAngle);
                    }
                }

                Self::Duration(_) => {}

                Self::Frequency(_) => {}

                Self::Amplitude(value) => {
                    if !value.as_f64().is_finite() {
                        return Err(ValueError::InvalidAmplitude);
                    }
                }

                Self::Phase(value) => {
                    if !value.as_f64().is_finite() {
                        return Err(ValueError::InvalidPhase);
                    }
                }

                Self::Parameter(parameter) => {
                    parameter.validate().map_err(|_| {
                        ValueError::InvalidStructure
                    })?;
                }

                Self::Array(array) => {
                    for element in array.iter() {
                        if element.kind() != array.element_kind() {
                            return Err(ValueError::TypeMismatch {
                                expected: array.element_kind(),
                                actual: element.kind(),
                            });
                        }

                        stack.push(element);
                    }
                }

                Self::Tuple(tuple) => {
                    for element in tuple.iter() {
                        stack.push(element);
                    }
                }

                Self::Optional(optional) => {
                    if let Some(value) = optional.as_ref() {
                        stack.push(value);
                    }
                }
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Structural metrics
    // -------------------------------------------------------------------------

    /// Returns the number of value nodes in the complete value tree.
    ///
    /// The traversal is iterative.
    pub fn node_count(&self) -> usize {
        let mut count = 0usize;
        let mut stack = vec![self];

        while let Some(value) = stack.pop() {
            count = match count.checked_add(1) {
                Some(value) => value,
                None => return usize::MAX,
            };

            match value {
                Self::Array(array) => {
                    for element in array.iter() {
                        stack.push(element);
                    }
                }

                Self::Tuple(tuple) => {
                    for element in tuple.iter() {
                        stack.push(element);
                    }
                }

                Self::Optional(optional) => {
                    if let Some(value) = optional.as_ref() {
                        stack.push(value);
                    }
                }

                _ => {}
            }
        }

        count
    }

    /// Returns the maximum structural nesting depth.
    ///
    /// The traversal is iterative.
    pub fn depth(&self) -> usize {
        let mut maximum = 0usize;
        let mut stack = vec![(self, 0usize)];

        while let Some((value, depth)) = stack.pop() {
            maximum = maximum.max(depth);

            match value {
                Self::Array(array) => {
                    for element in array.iter() {
                        stack.push((element, depth.saturating_add(1)));
                    }
                }

                Self::Tuple(tuple) => {
                    for element in tuple.iter() {
                        stack.push((element, depth.saturating_add(1)));
                    }
                }

                Self::Optional(optional) => {
                    if let Some(value) = optional.as_ref() {
                        stack.push((value, depth.saturating_add(1)));
                    }
                }

                _ => {}
            }
        }

        maximum
    }

    // -------------------------------------------------------------------------
    // Symbol collection
    // -------------------------------------------------------------------------

    /// Collects all parameter symbols contained by this value.
    ///
    /// Symbols are returned in deterministic lexical order and deduplicated.
    pub fn collect_symbols(&self) -> Vec<String> {
        let mut symbols = BTreeMap::<String, ()>::new();
        let mut stack = vec![self];

        while let Some(value) = stack.pop() {
            match value {
                Self::Parameter(parameter) => {
                    for symbol in parameter.collect_symbols() {
                        symbols.insert(symbol, ());
                    }
                }

                Self::Array(array) => {
                    for element in array.iter() {
                        stack.push(element);
                    }
                }

                Self::Tuple(tuple) => {
                    for element in tuple.iter() {
                        stack.push(element);
                    }
                }

                Self::Optional(optional) => {
                    if let Some(value) = optional.as_ref() {
                        stack.push(value);
                    }
                }

                _ => {}
            }
        }

        symbols.into_keys().collect()
    }

    // -------------------------------------------------------------------------
    // Canonical structural hash
    // -------------------------------------------------------------------------

    /// Feeds a deterministic structural representation into a hasher.
    ///
    /// This does not claim to be a cryptographic hash. The purpose is to give
    /// higher layers a deterministic structural hashing primitive.
    pub fn canonical_hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state);
    }

    // -------------------------------------------------------------------------
    // Numeric conversion
    // -------------------------------------------------------------------------

    /// Converts a scalar integer value to `i128`.
    pub fn to_i128(&self) -> Result<i128, ValueError> {
        match self {
            Self::Integer(value) => Ok(value.value()),

            Self::UnsignedInteger(value) => i128::try_from(value.value())
                .map_err(|_| ValueError::NumericOverflow),

            _ => Err(ValueError::TypeMismatch {
                expected: ValueKind::Integer,
                actual: self.kind(),
            }),
        }
    }

    /// Converts an integer value to `u128`.
    pub fn to_u128(&self) -> Result<u128, ValueError> {
        match self {
            Self::UnsignedInteger(value) => Ok(value.value()),

            Self::Integer(value) => u128::try_from(value.value())
                .map_err(|_| ValueError::NumericOverflow),

            _ => Err(ValueError::TypeMismatch {
                expected: ValueKind::UnsignedInteger,
                actual: self.kind(),
            }),
        }
    }

    /// Converts a finite numeric scalar to `f64`.
    ///
    /// Integer conversion is rejected if it cannot be represented exactly.
    pub fn to_f64_exact(&self) -> Result<f64, ValueError> {
        match self {
            Self::Float(value) => Ok(value.get()),

            Self::Integer(value) => {
                let converted = value.value() as f64;

                if !converted.is_finite() {
                    return Err(ValueError::NumericOverflow);
                }

                if (converted as i128) != value.value() {
                    return Err(ValueError::NumericLossOfPrecision);
                }

                Ok(converted)
            }

            Self::UnsignedInteger(value) => {
                let converted = value.value() as f64;

                if !converted.is_finite() {
                    return Err(ValueError::NumericOverflow);
                }

                if (converted as u128) != value.value() {
                    return Err(ValueError::NumericLossOfPrecision);
                }

                Ok(converted)
            }

            Self::Angle(value) => Ok(value.as_f64()),
            Self::Amplitude(value) => Ok(value.as_f64()),
            Self::Phase(value) => Ok(value.as_f64()),

            _ => Err(ValueError::TypeMismatch {
                expected: ValueKind::Float,
                actual: self.kind(),
            }),
        }
    }

    // -------------------------------------------------------------------------
    // Type matching
    // -------------------------------------------------------------------------

    /// Returns whether the value has the requested kind.
    #[must_use]
    pub const fn is_kind(&self, kind: ValueKind) -> bool {
        self.kind() == kind
    }

    /// Requires a specific kind.
    pub fn require_kind(
        &self,
        expected: ValueKind,
    ) -> Result<(), ValueError> {
        if self.kind() == expected {
            Ok(())
        } else {
            Err(ValueError::TypeMismatch {
                expected,
                actual: self.kind(),
            })
        }
    }

    // -------------------------------------------------------------------------
    // Reference helpers
    // -------------------------------------------------------------------------

    /// Returns the referenced value ID if this value is a reference.
    #[must_use]
    pub const fn referenced_value_id(&self) -> Option<ValueId> {
        match self {
            Self::Reference(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the logical qubit ID if this value is a logical qubit.
    #[must_use]
    pub const fn logical_qubit_id(&self) -> Option<QubitId> {
        match self {
            Self::Qubit(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns the physical qubit ID if this value is a physical qubit.
    #[must_use]
    pub const fn physical_qubit_id(
        &self,
    ) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(id) => Some(*id),
            _ => None,
        }
    }
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(value) => write!(formatter, "{value}"),

            Self::Integer(value) => write!(formatter, "{value}"),

            Self::UnsignedInteger(value) => {
                write!(formatter, "{value}u")
            }

            Self::Float(value) => write!(formatter, "{value}"),

            Self::Complex(value) => write!(formatter, "{value}"),

            Self::Angle(value) => write!(formatter, "{value}"),

            Self::Duration(value) => write!(formatter, "{value}"),

            Self::Frequency(value) => write!(formatter, "{value}"),

            Self::Amplitude(value) => write!(formatter, "{value}"),

            Self::Phase(value) => write!(formatter, "{value}"),

            Self::Qubit(value) => write!(formatter, "{value}"),

            Self::PhysicalQubit(value) => write!(formatter, "{value}"),

            Self::Parameter(value) => write!(formatter, "{value}"),

            Self::Reference(value) => write!(formatter, "%{value}"),

            Self::Array(array) => {
                formatter.write_str("[")?;

                for (index, value) in array.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }

                    write!(formatter, "{value}")?;
                }

                formatter.write_str("]")
            }

            Self::Tuple(tuple) => {
                formatter.write_str("(")?;

                for (index, value) in tuple.iter().enumerate() {
                    if index != 0 {
                        formatter.write_str(", ")?;
                    }

                    write!(formatter, "{value}")?;
                }

                formatter.write_str(")")
            }

            Self::Optional(OptionalValue::None) => {
                formatter.write_str("None")
            }

            Self::Optional(OptionalValue::Some(value)) => {
                write!(formatter, "Some({value})")
            }

            Self::Unit => formatter.write_str("()"),
        }
    }
}

// =============================================================================
// Conversions
// =============================================================================

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Self {
        Self::integer(i128::from(value))
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Self {
        Self::integer(i128::from(value))
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::integer(i128::from(value))
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::integer(i128::from(value))
    }
}

impl From<i128> for Value {
    fn from(value: i128) -> Self {
        Self::integer(value)
    }
}

impl From<isize> for Value {
    fn from(value: isize) -> Self {
        Self::integer(value as i128)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::unsigned_integer(u128::from(value))
    }
}

impl From<u16> for Value {
    fn from(value: u16) -> Self {
        Self::unsigned_integer(u128::from(value))
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::unsigned_integer(u128::from(value))
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::unsigned_integer(u128::from(value))
    }
}

impl From<u128> for Value {
    fn from(value: u128) -> Self {
        Self::unsigned_integer(value)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Self::unsigned_integer(value as u128)
    }
}

impl From<QubitId> for Value {
    fn from(value: QubitId) -> Self {
        Self::Qubit(value)
    }
}

impl From<PhysicalQubitId> for Value {
    fn from(value: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(value)
    }
}

impl From<Parameter> for Value {
    fn from(value: Parameter) -> Self {
        Self::Parameter(value)
    }
}

impl From<ValueId> for Value {
    fn from(value: ValueId) -> Self {
        Self::Reference(value)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_float_rejects_nan() {
        assert_eq!(
            FiniteFloat::new(f64::NAN),
            Err(ValueError::NonFiniteFloat)
        );
    }

    #[test]
    fn finite_float_rejects_positive_infinity() {
        assert_eq!(
            FiniteFloat::new(f64::INFINITY),
            Err(ValueError::NonFiniteFloat)
        );
    }

    #[test]
    fn finite_float_rejects_negative_infinity() {
        assert_eq!(
            FiniteFloat::new(f64::NEG_INFINITY),
            Err(ValueError::NonFiniteFloat)
        );
    }

    #[test]
    fn finite_float_accepts_zero() {
        assert!(FiniteFloat::new(0.0).is_ok());
    }

    #[test]
    fn duration_nanoseconds_are_exact() {
        let duration = Duration::nanoseconds(20).unwrap();

        assert_eq!(
            duration.as_attoseconds(),
            20_000_000_000
        );
    }

    #[test]
    fn duration_addition_is_checked() {
        let left = Duration::attoseconds(u128::MAX);
        let right = Duration::attoseconds(1);

        assert_eq!(
            left.checked_add(right),
            Err(ValueError::NumericOverflow)
        );
    }

    #[test]
    fn duration_subtraction_rejects_underflow() {
        let left = Duration::attoseconds(1);
        let right = Duration::attoseconds(2);

        assert_eq!(
            left.checked_sub(right),
            Err(ValueError::InvalidDuration)
        );
    }

    #[test]
    fn frequency_conversion_is_exact() {
        let frequency = Frequency::gigahertz(5).unwrap();

        assert_eq!(
            frequency.as_femtohertz(),
            5_000_000_000_000_000_000
        );
    }

    #[test]
    fn integer_checked_operations_work() {
        let value = Integer::new(10);

        assert_eq!(
            value.checked_add(Integer::new(5)).unwrap().value(),
            15
        );

        assert_eq!(
            value.checked_sub(Integer::new(5)).unwrap().value(),
            5
        );

        assert_eq!(
            value.checked_mul(Integer::new(2)).unwrap().value(),
            20
        );
    }

    #[test]
    fn integer_division_by_zero_is_rejected() {
        assert_eq!(
            Integer::new(10).checked_div(Integer::new(0)),
            Err(ValueError::InvalidStructure)
        );
    }

    #[test]
    fn value_kind_is_stable() {
        assert_eq!(
            Value::integer(42).kind(),
            ValueKind::Integer
        );

        assert_eq!(
            Value::unsigned_integer(42).kind(),
            ValueKind::UnsignedInteger
        );

        assert_eq!(
            Value::bool(true).kind(),
            ValueKind::Bool
        );

        assert_eq!(
            Value::unit().kind(),
            ValueKind::Unit
        );
    }

    #[test]
    fn amplitude_accepts_finite_values() {
        let amplitude = Value::amplitude(0.3).unwrap();

        assert_eq!(
            amplitude.as_amplitude().unwrap().as_f64(),
            0.3
        );
    }

    #[test]
    fn amplitude_rejects_non_finite_values() {
        assert_eq!(
            Value::amplitude(f64::INFINITY),
            Err(ValueError::NonFiniteFloat)
        );
    }

    #[test]
    fn logical_qubit_uses_canonical_qubit_id() {
        let qubit = QubitId::new(123_456);

        let value = Value::qubit(qubit);

        assert_eq!(
            value.as_qubit(),
            Some(qubit)
        );
    }

    #[test]
    fn physical_qubit_uses_canonical_physical_id() {
        let qubit = PhysicalQubitId::new(987_654);

        let value = Value::physical_qubit(qubit);

        assert_eq!(
            value.as_physical_qubit(),
            Some(qubit)
        );
    }

    #[test]
    fn arrays_are_homogeneous() {
        let array = Value::array(vec![
            Value::integer(1),
            Value::integer(2),
            Value::integer(3),
        ])
        .unwrap();

        assert_eq!(
            array.as_array().unwrap().len(),
            3
        );
    }

    #[test]
    fn arrays_reject_mixed_types() {
        let result = Value::array(vec![
            Value::integer(1),
            Value::bool(true),
        ]);

        assert!(matches!(
            result,
            Err(ValueError::TypeMismatch {
                expected: ValueKind::Integer,
                actual: ValueKind::Bool,
            })
        ));
    }

    #[test]
    fn tuple_allows_mixed_types() {
        let tuple = Value::tuple(vec![
            Value::integer(1),
            Value::bool(true),
            Value::unit(),
        ]);

        assert_eq!(tuple.as_tuple().unwrap().len(), 3);
    }

    #[test]
    fn optional_values_work() {
        let some = Value::some(Value::integer(7));
        let none = Value::none();

        assert!(some.as_optional().unwrap().is_some());
        assert!(none.as_optional().unwrap().is_none());
    }

    #[test]
    fn nested_validation_is_iterative() {
        let mut value = Value::integer(1);

        for _ in 0..1_000 {
            value = Value::tuple(vec![value]);
        }

        assert!(value.validate().is_ok());
        assert!(value.depth() >= 1_000);
    }

    #[test]
    fn node_count_is_structural() {
        let value = Value::tuple(vec![
            Value::integer(1),
            Value::tuple(vec![
                Value::integer(2),
                Value::integer(3),
            ]),
        ]);

        assert_eq!(value.node_count(), 5);
    }

    #[test]
    fn symbol_collection_is_deterministic() {
        let first =
            Parameter::symbol("z").unwrap();

        let second =
            Parameter::symbol("a").unwrap();

        let value = Value::tuple(vec![
            Value::parameter(first),
            Value::parameter(second),
        ]);

        assert_eq!(
            value.collect_symbols(),
            vec!["a".to_owned(), "z".to_owned()]
        );
    }

    #[test]
    fn value_reference_uses_canonical_value_id() {
        let id = ValueId::new(77);

        let value = Value::reference(id);

        assert_eq!(
            value.referenced_value_id(),
            Some(id)
        );
    }

    #[test]
    fn signed_integer_conversion_works() {
        let value = Value::integer(-42);

        assert_eq!(value.to_i128().unwrap(), -42);
    }

    #[test]
    fn unsigned_integer_conversion_works() {
        let value = Value::unsigned_integer(42);

        assert_eq!(value.to_u128().unwrap(), 42);
    }

    #[test]
    fn negative_integer_cannot_be_unsigned() {
        let value = Value::integer(-1);

        assert_eq!(
            value.to_u128(),
            Err(ValueError::NumericOverflow)
        );
    }

    #[test]
    fn value_display_is_deterministic() {
        assert_eq!(
            Value::integer(42).to_string(),
            "42"
        );

        assert_eq!(
            Value::unsigned_integer(42).to_string(),
            "42u"
        );

        assert_eq!(
            Value::duration_nanoseconds(20)
                .unwrap()
                .to_string(),
            "20ns"
        );
    }

    #[test]
    fn value_validation_accepts_unit_values() {
        assert!(Value::unit().validate().is_ok());
    }

    #[test]
    fn complex_values_require_finite_components() {
        assert!(
            Value::complex(1.0, 2.0).is_ok()
        );

        assert_eq!(
            Value::complex(
                f64::NAN,
                2.0
            ),
            Err(ValueError::NonFiniteFloat)
        );
    }
}