//! Zamani Quantum IR — Canonical Value System
//!
//! This module defines the foundational value vocabulary used by the
//! hardware-independent Zamani Quantum IR.
//!
//! # Architectural role
//!
//! `core::value` owns semantic values.
//!
//! It does NOT own:
//!
//! - gate semantics;
//! - operation semantics;
//! - circuit construction;
//! - routing;
//! - scheduling;
//! - hardware descriptions;
//! - calibration execution;
//! - simulation state;
//! - QEC decoding;
//! - frontend syntax;
//! - backend execution.
//!
//! Those responsibilities belong to higher-level IR or downstream modules.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir::identity ───────┐
//! quantum::ir::parameter ──────┼──► core::value
//! quantum::ir::qubit ──────────┘
//!
//! core::value
//!     │
//!     ├──► program
//!     ├──► operation
//!     ├──► control
//!     ├──► pulse
//!     ├──► timing
//!     ├──► resources
//!     ├──► validation
//!     ├──► analysis
//!     └──► serialization
//! ```
//!
//! `core::value` must never depend on those downstream modules.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once and may be compiled for different:
//!
//! - qubit counts;
//! - quantum architectures;
//! - hardware topologies;
//! - simulators;
//! - pulse systems;
//! - analog systems;
//! - fault-tolerant systems;
//! - distributed quantum systems;
//! - future quantum technologies.
//!
//! Consequently this file contains no hardware-specific limits.
//!
//! There is no:
//!
//! - maximum qubit count;
//! - maximum register size;
//! - maximum operation count;
//! - fixed gate universe;
//! - fixed hardware topology.
//!
//! Concrete compiler/security limits belong to explicit policy objects such as
//! `QuantumIrLimits` and must not be encoded into semantic value types.
//!
//! # Numeric policy
//!
//! Finite floating-point values are represented by [`FiniteFloat`].
//!
//! NaN and positive/negative infinity are rejected by checked constructors.
//!
//! Integer domains use `i128` and `u128` because they are stable,
//! platform-independent semantic integer domains in this dependency-free
//! foundational layer.
//!
//! This does NOT claim that these are the maximum integers supported by every
//! future Zamani subsystem. Arbitrary-precision integer support, if required,
//! should be introduced as an explicit numeric dialect/type rather than
//! silently changing the meaning of the existing integer variants.
//!
//! # Unit policy
//!
//! Canonical exact units are:
//!
//! - duration: attoseconds;
//! - frequency: femtohertz;
//! - angle: radians;
//! - phase: radians;
//! - amplitude: finite unit-neutral scalar.
//!
//! Hardware conversion is outside this module.
//!
//! # Quantum identity boundary
//!
//! Logical and physical qubit identities are imported from the canonical
//! `quantum::ir::qubit` module:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! This module never defines duplicate qubit identifier types.
//!
//! # Parameter boundary
//!
//! Symbolic parameter semantics remain owned by:
//!
//! ```text
//! quantum::ir::parameter::Parameter
//! ```
//!
//! This module embeds `Parameter`; it does not duplicate the parameter AST.
//!
//! # Value reference boundary
//!
//! `ValueId` is owned by `identity.rs`.
//!
//! `Value::Reference(ValueId)` is only a reference to an IR value. It does not
//! define SSA dominance, use-def chains, block semantics, or ownership.
//!
//! # Rust compatibility
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
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
//! `operation.rs` may use [`Value`] for semantic operands/results.
//!
//! `gate.rs` may consume [`Angle`], [`Parameter`], and scalar values.
//!
//! `measurement.rs` may consume [`Value::Qubit`] and classical values.
//!
//! `pulse.rs` may consume [`Amplitude`], [`Duration`], [`Frequency`], and
//! [`Phase`].
//!
//! `timing.rs` may consume [`Duration`].
//!
//! `control_flow.rs` may consume [`Value::Bool`], [`Value::Parameter`], and
//! [`Value::Reference`].
//!
//! `validation.rs` may call [`Value::validate`].
//!
//! `analysis.rs` may inspect [`ValueKind`] and structural metrics.
//!
//! `serialization.rs` should encode values structurally rather than using
//! display strings as the semantic representation.
//!
//! `hashing` may use [`Value::canonical_hash`].
//!
//! No new hardware architecture should require changing this module merely
//! because the architecture exists.

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt;
use std::hash::{Hash, Hasher};

use super::identity::ValueId;
use super::parameter::Parameter;
use super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Unit constants
// =============================================================================

/// Number of attoseconds in one second.
pub const ATTOSECONDS_PER_SECOND: u128 = 1_000_000_000_000_000_000;

/// Number of femtohertz in one hertz.
pub const FEMTOHERTZ_PER_HERTZ: u128 = 1_000_000_000_000;

/// Number of femtohertz in one kilohertz.
pub const FEMTOHERTZ_PER_KHZ: u128 = 1_000_000_000_000_000;

/// Number of femtohertz in one megahertz.
pub const FEMTOHERTZ_PER_MHZ: u128 = 1_000_000_000_000_000;

/// Number of femtohertz in one gigahertz.
pub const FEMTOHERTZ_PER_GHZ: u128 = 1_000_000_000_000_000_000;

/// Number of attoseconds in one femtosecond.
pub const ATTOSECONDS_PER_FEMTOSECOND: u128 = 1_000;

/// Number of attoseconds in one picosecond.
pub const ATTOSECONDS_PER_PICOSECOND: u128 = 1_000_000;

/// Number of attoseconds in one nanosecond.
pub const ATTOSECONDS_PER_NANOSECOND: u128 = 1_000_000_000;

/// Number of attoseconds in one microsecond.
pub const ATTOSECONDS_PER_MICROSECOND: u128 = 1_000_000_000_000;

/// Number of attoseconds in one millisecond.
pub const ATTOSECONDS_PER_MILLISECOND: u128 = 1_000_000_000_000_000;

// =============================================================================
// Value kind
// =============================================================================

/// Broad semantic classification of a [`Value`].
///
/// `ValueKind` is intentionally smaller than the eventual full canonical IR
/// type system. The future `types.rs` module can provide richer types while
/// `ValueKind` remains useful for lightweight classification.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum ValueKind {
    /// Boolean.
    Bool,

    /// Signed integer.
    Integer,

    /// Unsigned integer.
    UnsignedInteger,

    /// Finite floating-point scalar.
    Float,

    /// Finite complex scalar.
    Complex,

    /// Angle in radians.
    Angle,

    /// Exact duration.
    Duration,

    /// Exact non-negative frequency.
    Frequency,

    /// Hardware-independent finite amplitude.
    Amplitude,

    /// Phase in radians.
    Phase,

    /// Logical qubit.
    Qubit,

    /// Physical-qubit namespace reference.
    PhysicalQubit,

    /// Symbolic/runtime parameter.
    Parameter,

    /// Reference to an existing IR value.
    Reference,

    /// Homogeneous array.
    Array,

    /// Heterogeneous ordered tuple.
    Tuple,

    /// Optional value.
    Optional,

    /// Unit value.
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

/// Errors produced by checked value operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueError {
    /// Floating-point input was NaN or infinite.
    NonFiniteFloat,

    /// Arithmetic operation overflowed.
    NumericOverflow,

    /// Arithmetic operation underflowed.
    NumericUnderflow,

    /// Conversion would lose exact information.
    NumericLossOfPrecision,

    /// Division by zero.
    DivisionByZero,

    /// Invalid duration.
    InvalidDuration,

    /// Invalid frequency.
    InvalidFrequency,

    /// Invalid angle.
    InvalidAngle,

    /// Invalid amplitude.
    InvalidAmplitude,

    /// Invalid phase.
    InvalidPhase,

    /// Collection structure is invalid.
    InvalidStructure,

    /// Collection length cannot be represented.
    CollectionSizeOverflow,

    /// An array contains an element of the wrong kind.
    TypeMismatch {
        /// Expected value kind.
        expected: ValueKind,

        /// Actual value kind.
        actual: ValueKind,
    },

    /// A required semantic conversion is unavailable.
    UnsupportedConversion {
        /// Source value kind.
        from: ValueKind,

        /// Requested destination kind.
        to: ValueKind,
    },

    /// A value tree exceeded an explicitly supplied traversal budget.
    ///
    /// This is a caller/resource-policy failure, not a semantic machine-size
    /// limit.
    TraversalLimitExceeded,
}

impl fmt::Display for ValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteFloat => {
                formatter.write_str("floating-point value must be finite")
            }

            Self::NumericOverflow => {
                formatter.write_str("numeric operation overflowed")
            }

            Self::NumericUnderflow => {
                formatter.write_str("numeric operation underflowed")
            }

            Self::NumericLossOfPrecision => {
                formatter.write_str(
                    "numeric conversion would lose information",
                )
            }

            Self::DivisionByZero => {
                formatter.write_str("division by zero")
            }

            Self::InvalidDuration => {
                formatter.write_str("invalid duration")
            }

            Self::InvalidFrequency => {
                formatter.write_str("invalid frequency")
            }

            Self::InvalidAngle => {
                formatter.write_str("invalid angle")
            }

            Self::InvalidAmplitude => {
                formatter.write_str("invalid amplitude")
            }

            Self::InvalidPhase => {
                formatter.write_str("invalid phase")
            }

            Self::InvalidStructure => {
                formatter.write_str("invalid value structure")
            }

            Self::CollectionSizeOverflow => {
                formatter.write_str("collection size overflowed")
            }

            Self::TypeMismatch { expected, actual } => {
                write!(
                    formatter,
                    "value type mismatch: expected {expected}, found {actual}"
                )
            }

            Self::UnsupportedConversion { from, to } => {
                write!(
                    formatter,
                    "unsupported value conversion: {from} -> {to}"
                )
            }

            Self::TraversalLimitExceeded => {
                formatter.write_str("value traversal limit exceeded")
            }
        }
    }
}

impl std::error::Error for ValueError {}

// =============================================================================
// Structural traversal policy
// =============================================================================

/// Explicit resource policy for value-tree traversal.
///
/// This is deliberately separate from [`Value`].
///
/// A value has no architectural depth/size limit. A compiler can nevertheless
/// provide a finite traversal budget to protect itself against hostile or
/// accidentally enormous input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueTraversalPolicy {
    /// Maximum number of nodes that may be inspected.
    ///
    /// `None` means no local traversal budget.
    pub max_nodes: Option<usize>,

    /// Maximum structural depth that may be inspected.
    ///
    /// `None` means no local depth budget.
    pub max_depth: Option<usize>,
}

impl ValueTraversalPolicy {
    /// Creates an unrestricted traversal policy.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_nodes: None,
            max_depth: None,
        }
    }

    /// Creates an explicit traversal policy.
    #[must_use]
    pub const fn new(
        max_nodes: Option<usize>,
        max_depth: Option<usize>,
    ) -> Self {
        Self {
            max_nodes,
            max_depth,
        }
    }

    /// Checks whether a node is permitted.
    fn check_node(
        self,
        nodes: usize,
        depth: usize,
    ) -> Result<(), ValueError> {
        if let Some(limit) = self.max_nodes {
            if nodes > limit {
                return Err(ValueError::TraversalLimitExceeded);
            }
        }

        if let Some(limit) = self.max_depth {
            if depth > limit {
                return Err(ValueError::TraversalLimitExceeded);
            }
        }

        Ok(())
    }
}

impl Default for ValueTraversalPolicy {
    fn default() -> Self {
        Self::unlimited()
    }
}

// =============================================================================
// Finite floating point
// =============================================================================

/// Finite IEEE-754 `f64`.
///
/// The constructor rejects NaN and both infinities.
///
/// Equality and hashing use the exact IEEE-754 bit representation. Therefore
/// positive and negative zero remain distinct semantic bit patterns.
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

    /// Returns the underlying `f64`.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns the exact IEEE-754 representation.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0.to_bits()
    }

    /// Returns whether this value is either positive or negative zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns the absolute value.
    #[must_use]
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }
}

impl PartialEq for FiniteFloat {
    fn eq(&self, other: &Self) -> bool {
        self.bits() == other.bits()
    }
}

impl Eq for FiniteFloat {}

impl Hash for FiniteFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bits().hash(state);
    }
}

impl PartialOrd for FiniteFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for FiniteFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .unwrap_or(Ordering::Equal)
    }
}

impl fmt::Display for FiniteFloat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

// =============================================================================
// Complex
// =============================================================================

/// Finite complex scalar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComplexValue {
    real: FiniteFloat,
    imaginary: FiniteFloat,
}

impl ComplexValue {
    /// Creates a finite complex value.
    pub fn new(
        real: f64,
        imaginary: f64,
    ) -> Result<Self, ValueError> {
        Ok(Self {
            real: FiniteFloat::new(real)?,
            imaginary: FiniteFloat::new(imaginary)?,
        })
    }

    /// Creates a complex value from already validated components.
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
    pub const fn components(
        self,
    ) -> (FiniteFloat, FiniteFloat) {
        (self.real, self.imaginary)
    }
}

impl fmt::Display for ComplexValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imaginary.get().is_sign_negative() {
            write!(
                formatter,
                "{}{}i",
                self.real,
                self.imaginary
            )
        } else {
            write!(
                formatter,
                "{}+{}i",
                self.real,
                self.imaginary
            )
        }
    }
}

// =============================================================================
// Angle
// =============================================================================

/// Finite angle represented in radians.
///
/// No normalization is performed. In particular, `2π` and `0` remain
/// distinguishable until a separate semantic canonicalization pass chooses to
/// normalize them.
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

    /// Returns the finite radian value.
    #[must_use]
    pub const fn radians_value(self) -> FiniteFloat {
        self.0
    }

    /// Returns the angle as `f64`.
    #[must_use]
    pub const fn as_f64(self) -> f64 {
        self.0.get()
    }

    /// Returns zero radians.
    #[must_use]
    pub const fn zero() -> Self {
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

/// Finite phase represented in radians.
///
/// Phase and angle intentionally remain separate semantic types even though
/// their storage unit is identical.
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

    /// Returns the finite radian value.
    #[must_use]
    pub const fn radians_value(self) -> FiniteFloat {
        self.0
    }

    /// Returns the phase as `f64`.
    #[must_use]
    pub const fn as_f64(self) -> f64 {
        self.0.get()
    }

    /// Returns zero phase.
    #[must_use]
    pub const fn zero() -> Self {
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

/// Finite hardware-independent amplitude.
///
/// The IR intentionally does not constrain amplitudes to `[0, 1]`.
///
/// Some quantum technologies use signed, complex, calibrated, normalized, or
/// otherwise device-specific amplitude domains. The target backend owns that
/// interpretation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Amplitude(FiniteFloat);

impl Amplitude {
    /// Creates a finite amplitude.
    pub fn new(value: f64) -> Result<Self, ValueError> {
        Ok(Self(FiniteFloat::new(value)?))
    }

    /// Creates an amplitude from a validated finite scalar.
    #[must_use]
    pub const fn from_finite(value: FiniteFloat) -> Self {
        Self(value)
    }

    /// Returns the amplitude as `f64`.
    #[must_use]
    pub const fn as_f64(self) -> f64 {
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

/// Exact non-negative duration represented in attoseconds.
///
/// The type is a semantic duration, not a hardware clock tick.
///
/// Backend-specific units such as `dt` must be resolved by the timing/hardware
/// layers rather than being encoded into this type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration(u128);

impl Duration {
    /// Creates an exact duration from attoseconds.
    #[must_use]
    pub const fn attoseconds(value: u128) -> Self {
        Self(value)
    }

    /// Creates a duration from femtoseconds.
    pub fn femtoseconds(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(ATTOSECONDS_PER_FEMTOSECOND)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Creates a duration from picoseconds.
    pub fn picoseconds(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(ATTOSECONDS_PER_PICOSECOND)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Creates a duration from nanoseconds.
    pub fn nanoseconds(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(ATTOSECONDS_PER_NANOSECOND)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Creates a duration from microseconds.
    pub fn microseconds(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(ATTOSECONDS_PER_MICROSECOND)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Creates a duration from milliseconds.
    pub fn milliseconds(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(ATTOSECONDS_PER_MILLISECOND)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Creates a duration from whole seconds.
    pub fn seconds(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(ATTOSECONDS_PER_SECOND)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Returns exact attoseconds.
    #[must_use]
    pub const fn as_attoseconds(self) -> u128 {
        self.0
    }

    /// Returns whether the duration is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Adds durations with overflow checking.
    pub fn checked_add(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Subtracts durations with underflow checking.
    pub fn checked_sub(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(ValueError::NumericUnderflow)
    }

    /// Multiplies by an unsigned scalar.
    pub fn checked_mul(
        self,
        multiplier: u128,
    ) -> Result<Self, ValueError> {
        self.0
            .checked_mul(multiplier)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Returns whole nanoseconds and remaining attoseconds.
    #[must_use]
    pub fn split_nanoseconds(self) -> (u128, u128) {
        (
            self.0 / ATTOSECONDS_PER_NANOSECOND,
            self.0 % ATTOSECONDS_PER_NANOSECOND,
        )
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 % ATTOSECONDS_PER_NANOSECOND == 0 {
            write!(
                formatter,
                "{}ns",
                self.0 / ATTOSECONDS_PER_NANOSECOND
            )
        } else if self.0 % ATTOSECONDS_PER_PICOSECOND == 0 {
            write!(
                formatter,
                "{}ps",
                self.0 / ATTOSECONDS_PER_PICOSECOND
            )
        } else if self.0 % ATTOSECONDS_PER_FEMTOSECOND == 0 {
            write!(
                formatter,
                "{}fs",
                self.0 / ATTOSECONDS_PER_FEMTOSECOND
            )
        } else {
            write!(formatter, "{}as", self.0)
        }
    }
}

// =============================================================================
// Frequency
// =============================================================================

/// Exact non-negative frequency represented in femtohertz.
///
/// Negative frequency shifts should remain symbolic or use an explicitly
/// signed frequency type introduced by a future type/dialect layer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Frequency(u128);

impl Frequency {
    /// Creates a frequency from exact femtohertz.
    #[must_use]
    pub const fn femtohertz(value: u128) -> Self {
        Self(value)
    }

    /// Creates a frequency from whole hertz.
    pub fn hertz(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(FEMTOHERTZ_PER_HERTZ)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Creates a frequency from whole kilohertz.
    pub fn kilohertz(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(FEMTOHERTZ_PER_KHZ)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Creates a frequency from whole megahertz.
    pub fn megahertz(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(FEMTOHERTZ_PER_MHZ)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Creates a frequency from whole gigahertz.
    pub fn gigahertz(value: u128) -> Result<Self, ValueError> {
        value
            .checked_mul(FEMTOHERTZ_PER_GHZ)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Returns exact femtohertz.
    #[must_use]
    pub const fn as_femtohertz(self) -> u128 {
        self.0
    }

    /// Returns whether the frequency is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Adds frequencies with overflow checking.
    pub fn checked_add(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Subtracts frequencies with underflow checking.
    pub fn checked_sub(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(ValueError::NumericUnderflow)
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
        } else if self.0 % FEMTOHERTZ_PER_KHZ == 0 {
            write!(
                formatter,
                "{}kHz",
                self.0 / FEMTOHERTZ_PER_KHZ
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
// Signed integer
// =============================================================================

/// Platform-independent signed semantic integer.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
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
    pub fn checked_add(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked subtraction.
    pub fn checked_sub(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked multiplication.
    pub fn checked_mul(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        self.0
            .checked_mul(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked division.
    pub fn checked_div(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        if other.0 == 0 {
            return Err(ValueError::DivisionByZero);
        }

        self.0
            .checked_div(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked remainder.
    pub fn checked_rem(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        if other.0 == 0 {
            return Err(ValueError::DivisionByZero);
        }

        self.0
            .checked_rem(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked negation.
    pub fn checked_neg(self) -> Result<Self, ValueError> {
        self.0
            .checked_neg()
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
// Unsigned integer
// =============================================================================

/// Platform-independent unsigned semantic integer.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
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
    pub fn checked_add(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked subtraction.
    pub fn checked_sub(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        self.0
            .checked_sub(other.0)
            .map(Self)
            .ok_or(ValueError::NumericUnderflow)
    }

    /// Checked multiplication.
    pub fn checked_mul(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        self.0
            .checked_mul(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked division.
    pub fn checked_div(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        if other.0 == 0 {
            return Err(ValueError::DivisionByZero);
        }

        self.0
            .checked_div(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }

    /// Checked remainder.
    pub fn checked_rem(
        self,
        other: Self,
    ) -> Result<Self, ValueError> {
        if other.0 == 0 {
            return Err(ValueError::DivisionByZero);
        }

        self.0
            .checked_rem(other.0)
            .map(Self)
            .ok_or(ValueError::NumericOverflow)
    }
}

impl fmt::Display for UnsignedInteger {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}u", self.0)
    }
}

// =============================================================================
// Array
// =============================================================================

/// Canonical homogeneous value array.
///
/// The element kind is explicit, including for empty arrays.
///
/// This is important because an empty array has no element from which its
/// semantic kind can be inferred.
#[derive(Clone, Debug, PartialEq)]
pub struct ValueArray {
    element_kind: ValueKind,
    elements: Vec<Value>,
}

impl ValueArray {
    /// Creates an empty typed array.
    #[must_use]
    pub fn empty(element_kind: ValueKind) -> Self {
        Self {
            element_kind,
            elements: Vec::new(),
        }
    }

    /// Creates a homogeneous array by inferring its element kind.
    ///
    /// An empty input is represented as a `Unit`-typed empty array. Callers
    /// that require a different empty-array type should use [`Self::with_kind`].
    pub fn new(
        elements: Vec<Value>,
    ) -> Result<Self, ValueError> {
        let element_kind = match elements.first() {
            Some(value) => value.kind(),
            None => ValueKind::Unit,
        };

        Self::with_kind(element_kind, elements)
    }

    /// Creates a homogeneous array with an explicit element kind.
    pub fn with_kind(
        element_kind: ValueKind,
        elements: Vec<Value>,
    ) -> Result<Self, ValueError> {
        for element in &elements {
            if element.kind() != element_kind {
                return Err(ValueError::TypeMismatch {
                    expected: element_kind,
                    actual: element.kind(),
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

    /// Returns an element.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.elements.get(index)
    }

    /// Returns an iterator.
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.elements.iter()
    }

    /// Returns the owned elements.
    #[must_use]
    pub fn into_elements(self) -> Vec<Value> {
        self.elements
    }

    /// Appends an element after checking its semantic kind.
    pub fn push(
        &mut self,
        value: Value,
    ) -> Result<(), ValueError> {
        if value.kind() != self.element_kind {
            return Err(ValueError::TypeMismatch {
                expected: self.element_kind,
                actual: value.kind(),
            });
        }

        self.elements.push(value);

        Ok(())
    }

    /// Reserves capacity for additional elements.
    ///
    /// Allocation failures are handled by Rust's normal allocation behavior;
    /// this API does not turn memory availability into a semantic IR limit.
    pub fn reserve(&mut self, additional: usize) {
        self.elements.reserve(additional);
    }
}

// =============================================================================
// Tuple
// =============================================================================

/// Canonical heterogeneous ordered tuple.
#[derive(Clone, Debug, PartialEq)]
pub struct ValueTuple {
    elements: Vec<Value>,
}

impl ValueTuple {
    /// Creates a tuple.
    #[must_use]
    pub fn new(elements: Vec<Value>) -> Self {
        Self { elements }
    }

    /// Returns the number of elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// Returns whether the tuple is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns an element.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.elements.get(index)
    }

    /// Returns an iterator.
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.elements.iter()
    }

    /// Returns owned elements.
    #[must_use]
    pub fn into_elements(self) -> Vec<Value> {
        self.elements
    }
}

// =============================================================================
// Optional
// =============================================================================

/// Canonical optional semantic value.
#[derive(Clone, Debug, PartialEq)]
pub enum OptionalValue {
    /// No value.
    None,

    /// A value is present.
    Some(Box<Value>),
}

impl OptionalValue {
    /// Creates `None`.
    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }

    /// Creates `Some(value)`.
    #[must_use]
    pub fn some(value: Value) -> Self {
        Self::Some(Box::new(value))
    }

    /// Returns whether a value exists.
    #[must_use]
    pub const fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    /// Returns whether no value exists.
    #[must_use]
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns the contained value by reference.
    #[must_use]
    pub fn as_ref(&self) -> Option<&Value> {
        match self {
            Self::None => None,
            Self::Some(value) => Some(value.as_ref()),
        }
    }

    /// Consumes the optional and returns its contained value.
    #[must_use]
    pub fn into_option(self) -> Option<Value> {
        match self {
            Self::None => None,
            Self::Some(value) => Some(*value),
        }
    }
}

// =============================================================================
// Canonical value
// =============================================================================

/// Canonical typed value used throughout Zamani Quantum IR.
///
/// `Value` deliberately represents semantic data rather than quantum state.
///
/// It can represent values used by:
///
/// - gate parameters;
/// - measurements;
/// - dynamic classical control;
/// - pulse descriptions;
/// - timing;
/// - resource requirements;
/// - mappings;
/// - runtime parameters;
/// - structured program data.
///
/// It does not represent:
///
/// - state vectors;
/// - density matrices;
/// - simulator memory;
/// - hardware calibration databases;
/// - device topology.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Boolean.
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

    /// Exact duration.
    Duration(Duration),

    /// Exact non-negative frequency.
    Frequency(Frequency),

    /// Hardware-independent finite amplitude.
    Amplitude(Amplitude),

    /// Phase in radians.
    Phase(Phase),

    /// Logical qubit reference.
    Qubit(QubitId),

    /// Physical-qubit reference.
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

    /// Unit.
    Unit,
}

impl Value {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates a boolean.
    #[must_use]
    pub const fn bool(value: bool) -> Self {
        Self::Bool(value)
    }

    /// Creates a signed integer.
    #[must_use]
    pub const fn integer(value: i128) -> Self {
        Self::Integer(Integer::new(value))
    }

    /// Creates an unsigned integer.
    #[must_use]
    pub const fn unsigned_integer(value: u128) -> Self {
        Self::UnsignedInteger(UnsignedInteger::new(value))
    }

    /// Creates a finite floating-point value.
    pub fn float(value: f64) -> Result<Self, ValueError> {
        Ok(Self::Float(FiniteFloat::new(value)?))
    }

    /// Creates a finite complex value.
    pub fn complex(
        real: f64,
        imaginary: f64,
    ) -> Result<Self, ValueError> {
        Ok(Self::Complex(ComplexValue::new(
            real,
            imaginary,
        )?))
    }

    /// Creates an angle in radians.
    pub fn angle_radians(
        value: f64,
    ) -> Result<Self, ValueError> {
        Ok(Self::Angle(Angle::radians(value)?))
    }

    /// Creates an exact duration from attoseconds.
    #[must_use]
    pub const fn duration_attoseconds(
        value: u128,
    ) -> Self {
        Self::Duration(Duration::attoseconds(value))
    }

    /// Creates a duration from femtoseconds.
    pub fn duration_femtoseconds(
        value: u128,
    ) -> Result<Self, ValueError> {
        Ok(Self::Duration(Duration::femtoseconds(
            value,
        )?))
    }

    /// Creates a duration from picoseconds.
    pub fn duration_picoseconds(
        value: u128,
    ) -> Result<Self, ValueError> {
        Ok(Self::Duration(Duration::picoseconds(
            value,
        )?))
    }

    /// Creates a duration from nanoseconds.
    pub fn duration_nanoseconds(
        value: u128,
    ) -> Result<Self, ValueError> {
        Ok(Self::Duration(Duration::nanoseconds(
            value,
        )?))
    }

    /// Creates a duration from microseconds.
    pub fn duration_microseconds(
        value: u128,
    ) -> Result<Self, ValueError> {
        Ok(Self::Duration(Duration::microseconds(
            value,
        )?))
    }

    /// Creates a duration from milliseconds.
    pub fn duration_milliseconds(
        value: u128,
    ) -> Result<Self, ValueError> {
        Ok(Self::Duration(Duration::milliseconds(
            value,
        )?))
    }

    /// Creates a duration from seconds.
    pub fn duration_seconds(
        value: u128,
    ) -> Result<Self, ValueError> {
        Ok(Self::Duration(Duration::seconds(value)?))
    }

    /// Creates a frequency from femtohertz.
    #[must_use]
    pub const fn frequency_femtohertz(
        value: u128,
    ) -> Self {
        Self::Frequency(Frequency::femtohertz(value))
    }

    /// Creates a frequency from hertz.
    pub fn frequency_hertz(
        value: u128,
    ) -> Result<Self, ValueError> {
        Ok(Self::Frequency(Frequency::hertz(value)?))
    }

    /// Creates a frequency from kilohertz.
    pub fn frequency_kilohertz(
        value: u128,
    ) -> Result<Self, ValueError> {
        Ok(Self::Frequency(Frequency::kilohertz(
            value,
        )?))
    }

    /// Creates a frequency from megahertz.
    pub fn frequency_megahertz(
        value: u128,
    ) -> Result<Self, ValueError> {
        Ok(Self::Frequency(Frequency::megahertz(
            value,
        )?))
    }

    /// Creates a frequency from gigahertz.
    pub fn frequency_gigahertz(
        value: u128,
    ) -> Result<Self, ValueError> {
        Ok(Self::Frequency(Frequency::gigahertz(
            value,
        )?))
    }

    /// Creates an amplitude.
    pub fn amplitude(
        value: f64,
    ) -> Result<Self, ValueError> {
        Ok(Self::Amplitude(Amplitude::new(value)?))
    }

    /// Creates a phase in radians.
    pub fn phase_radians(
        value: f64,
    ) -> Result<Self, ValueError> {
        Ok(Self::Phase(Phase::radians(value)?))
    }

    /// Creates a logical-qubit value.
    #[must_use]
    pub const fn qubit(qubit: QubitId) -> Self {
        Self::Qubit(qubit)
    }

    /// Creates a physical-qubit value.
    #[must_use]
    pub const fn physical_qubit(
        qubit: PhysicalQubitId,
    ) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Creates a parameter value.
    #[must_use]
    pub fn parameter(parameter: Parameter) -> Self {
        Self::Parameter(parameter)
    }

    /// Creates a reference to another IR value.
    #[must_use]
    pub const fn reference(value: ValueId) -> Self {
        Self::Reference(value)
    }

    /// Creates an empty typed array.
    #[must_use]
    pub fn empty_array(kind: ValueKind) -> Self {
        Self::Array(ValueArray::empty(kind))
    }

    /// Creates a homogeneous array.
    pub fn array(
        elements: Vec<Value>,
    ) -> Result<Self, ValueError> {
        Ok(Self::Array(ValueArray::new(elements)?))
    }

    /// Creates a homogeneous array with an explicit kind.
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

    /// Creates unit.
    #[must_use]
    pub const fn unit() -> Self {
        Self::Unit
    }

    // =========================================================================
    // Classification
    // =========================================================================

    /// Returns the broad semantic kind.
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

    /// Returns whether this is a scalar semantic value.
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

    /// Returns whether this is a quantum-resource reference.
    #[must_use]
    pub const fn is_qubit_reference(&self) -> bool {
        matches!(
            self,
            Self::Qubit(_) | Self::PhysicalQubit(_)
        )
    }

    /// Returns whether the value is symbolic.
    ///
    /// Traversal is iterative to avoid making ordinary Rust call-stack depth a
    /// semantic IR limit.
    pub fn is_symbolic(&self) -> bool {
        let mut stack = vec![self];

        while let Some(value) = stack.pop() {
            match value {
                Self::Parameter(parameter) => {
                    if parameter.is_symbolic() {
                        return true;
                    }
                }

                Self::Array(array) => {
                    for child in array.iter() {
                        stack.push(child);
                    }
                }

                Self::Tuple(tuple) => {
                    for child in tuple.iter() {
                        stack.push(child);
                    }
                }

                Self::Optional(optional) => {
                    if let Some(child) = optional.as_ref() {
                        stack.push(child);
                    }
                }

                _ => {}
            }
        }

        false
    }

    /// Returns whether this value is concrete.
    ///
    /// A reference is not concrete because it depends on another IR value.
    pub fn is_concrete(&self) -> bool {
        !self.is_symbolic()
            && !matches!(self, Self::Reference(_))
    }

    // =========================================================================
    // Accessors
    // =========================================================================

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
    pub const fn as_unsigned_integer(
        &self,
    ) -> Option<u128> {
        match self {
            Self::UnsignedInteger(value) => Some(value.value()),
            _ => None,
        }
    }

    /// Returns a finite floating-point value.
    #[must_use]
    pub const fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(value.get()),
            _ => None,
        }
    }

    /// Returns a complex value.
    #[must_use]
    pub const fn as_complex(
        &self,
    ) -> Option<ComplexValue> {
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
    pub const fn as_duration(
        &self,
    ) -> Option<Duration> {
        match self {
            Self::Duration(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a frequency.
    #[must_use]
    pub const fn as_frequency(
        &self,
    ) -> Option<Frequency> {
        match self {
            Self::Frequency(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns an amplitude.
    #[must_use]
    pub const fn as_amplitude(
        &self,
    ) -> Option<Amplitude> {
        match self {
            Self::Amplitude(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a phase.
    #[must_use]
    pub const fn as_phase(
        &self,
    ) -> Option<Phase> {
        match self {
            Self::Phase(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a logical qubit.
    #[must_use]
    pub const fn as_qubit(
        &self,
    ) -> Option<QubitId> {
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

    /// Returns the embedded parameter.
    #[must_use]
    pub fn as_parameter(
        &self,
    ) -> Option<&Parameter> {
        match self {
            Self::Parameter(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the referenced value identity.
    #[must_use]
    pub const fn as_reference(
        &self,
    ) -> Option<ValueId> {
        match self {
            Self::Reference(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the array.
    #[must_use]
    pub fn as_array(
        &self,
    ) -> Option<&ValueArray> {
        match self {
            Self::Array(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the tuple.
    #[must_use]
    pub fn as_tuple(
        &self,
    ) -> Option<&ValueTuple> {
        match self {
            Self::Tuple(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the optional value.
    #[must_use]
    pub fn as_optional(
        &self,
    ) -> Option<&OptionalValue> {
        match self {
            Self::Optional(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the referenced value ID.
    #[must_use]
    pub const fn referenced_value_id(
        &self,
    ) -> Option<ValueId> {
        self.as_reference()
    }

    /// Returns the logical qubit ID.
    #[must_use]
    pub const fn logical_qubit_id(
        &self,
    ) -> Option<QubitId> {
        self.as_qubit()
    }

    /// Returns the physical qubit ID.
    #[must_use]
    pub const fn physical_qubit_id(
        &self,
    ) -> Option<PhysicalQubitId> {
        self.as_physical_qubit()
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates the complete value tree with no local traversal budget.
    ///
    /// For compiler-facing security validation, prefer
    /// [`Self::validate_with_policy`].
    pub fn validate(&self) -> Result<(), ValueError> {
        self.validate_with_policy(ValueTraversalPolicy::unlimited())
    }

    /// Validates the complete value tree under an explicit resource policy.
    ///
    /// Traversal itself is iterative.
    pub fn validate_with_policy(
        &self,
        policy: ValueTraversalPolicy,
    ) -> Result<(), ValueError> {
        let mut stack: Vec<(&Value, usize)> = Vec::new();
        stack.push((self, 0));

        let mut visited = 0usize;

        while let Some((value, depth)) = stack.pop() {
            visited = visited
                .checked_add(1)
                .ok_or(ValueError::CollectionSizeOverflow)?;

            policy.check_node(visited, depth)?;

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
                    parameter
                        .validate()
                        .map_err(|_| ValueError::InvalidStructure)?;
                }

                Self::Array(array) => {
                    for child in array.iter() {
                        if child.kind() != array.element_kind() {
                            return Err(ValueError::TypeMismatch {
                                expected: array.element_kind(),
                                actual: child.kind(),
                            });
                        }

                        stack.push((
                            child,
                            depth.checked_add(1).ok_or(
                                ValueError::CollectionSizeOverflow,
                            )?,
                        ));
                    }
                }

                Self::Tuple(tuple) => {
                    for child in tuple.iter() {
                        stack.push((
                            child,
                            depth.checked_add(1).ok_or(
                                ValueError::CollectionSizeOverflow,
                            )?,
                        ));
                    }
                }

                Self::Optional(optional) => {
                    if let Some(child) = optional.as_ref() {
                        stack.push((
                            child,
                            depth.checked_add(1).ok_or(
                                ValueError::CollectionSizeOverflow,
                            )?,
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    // =========================================================================
    // Structural metrics
    // =========================================================================

    /// Counts value nodes iteratively.
    ///
    /// `usize::MAX` is returned if the count itself cannot be represented.
    pub fn node_count(&self) -> usize {
        let mut count = 0usize;
        let mut stack = vec![self];

        while let Some(value) = stack.pop() {
            count = match count.checked_add(1) {
                Some(next) => next,
                None => return usize::MAX,
            };

            match value {
                Self::Array(array) => {
                    for child in array.iter() {
                        stack.push(child);
                    }
                }

                Self::Tuple(tuple) => {
                    for child in tuple.iter() {
                        stack.push(child);
                    }
                }

                Self::Optional(optional) => {
                    if let Some(child) = optional.as_ref() {
                        stack.push(child);
                    }
                }

                _ => {}
            }
        }

        count
    }

    /// Returns maximum structural nesting depth.
    ///
    /// The root has depth zero.
    pub fn depth(&self) -> usize {
        let mut maximum = 0usize;
        let mut stack = vec![(self, 0usize)];

        while let Some((value, depth)) = stack.pop() {
            maximum = maximum.max(depth);

            let child_depth = depth.saturating_add(1);

            match value {
                Self::Array(array) => {
                    for child in array.iter() {
                        stack.push((child, child_depth));
                    }
                }

                Self::Tuple(tuple) => {
                    for child in tuple.iter() {
                        stack.push((child, child_depth));
                    }
                }

                Self::Optional(optional) => {
                    if let Some(child) = optional.as_ref() {
                        stack.push((child, child_depth));
                    }
                }

                _ => {}
            }
        }

        maximum
    }

    // =========================================================================
    // Symbol collection
    // =========================================================================

    /// Collects all parameter symbols in deterministic lexical order.
    ///
    /// Duplicate symbols are removed.
    pub fn collect_symbols(&self) -> Vec<String> {
        let mut symbols = BTreeSet::<String>::new();
        let mut stack = vec![self];

        while let Some(value) = stack.pop() {
            match value {
                Self::Parameter(parameter) => {
                    for symbol in parameter.collect_symbols() {
                        symbols.insert(symbol);
                    }
                }

                Self::Array(array) => {
                    for child in array.iter() {
                        stack.push(child);
                    }
                }

                Self::Tuple(tuple) => {
                    for child in tuple.iter() {
                        stack.push(child);
                    }
                }

                Self::Optional(optional) => {
                    if let Some(child) = optional.as_ref() {
                        stack.push(child);
                    }
                }

                _ => {}
            }
        }

        symbols.into_iter().collect()
    }

    // =========================================================================
    // Type checking
    // =========================================================================

    /// Returns whether the value has a particular kind.
    #[must_use]
    pub const fn is_kind(&self, kind: ValueKind) -> bool {
        self.kind() == kind
    }

    /// Requires a particular kind.
    pub fn require_kind(
        &self,
        expected: ValueKind,
    ) -> Result<(), ValueError> {
        let actual = self.kind();

        if actual == expected {
            Ok(())
        } else {
            Err(ValueError::TypeMismatch {
                expected,
                actual,
            })
        }
    }

    // =========================================================================
    // Numeric conversion
    // =========================================================================

    /// Converts an integer value exactly to `i128`.
    pub fn to_i128(&self) -> Result<i128, ValueError> {
        match self {
            Self::Integer(value) => Ok(value.value()),

            Self::UnsignedInteger(value) => {
                i128::try_from(value.value())
                    .map_err(|_| ValueError::NumericOverflow)
            }

            _ => Err(ValueError::UnsupportedConversion {
                from: self.kind(),
                to: ValueKind::Integer,
            }),
        }
    }

    /// Converts an integer value exactly to `u128`.
    pub fn to_u128(&self) -> Result<u128, ValueError> {
        match self {
            Self::UnsignedInteger(value) => Ok(value.value()),

            Self::Integer(value) => {
                u128::try_from(value.value())
                    .map_err(|_| ValueError::NumericOverflow)
            }

            _ => Err(ValueError::UnsupportedConversion {
                from: self.kind(),
                to: ValueKind::UnsignedInteger,
            }),
        }
    }

    /// Converts a finite numeric scalar exactly to `f64`.
    ///
    /// Integer conversion is accepted only if converting to `f64` and back
    /// reproduces the original integer exactly.
    pub fn to_f64_exact(&self) -> Result<f64, ValueError> {
        match self {
            Self::Float(value) => Ok(value.get()),

            Self::Integer(value) => {
                let integer = value.value();
                let converted = integer as f64;

                if !converted.is_finite() {
                    return Err(ValueError::NumericOverflow);
                }

                if converted as i128 != integer {
                    return Err(ValueError::NumericLossOfPrecision);
                }

                Ok(converted)
            }

            Self::UnsignedInteger(value) => {
                let integer = value.value();
                let converted = integer as f64;

                if !converted.is_finite() {
                    return Err(ValueError::NumericOverflow);
                }

                if converted as u128 != integer {
                    return Err(ValueError::NumericLossOfPrecision);
                }

                Ok(converted)
            }

            Self::Angle(value) => Ok(value.as_f64()),

            Self::Amplitude(value) => Ok(value.as_f64()),

            Self::Phase(value) => Ok(value.as_f64()),

            _ => Err(ValueError::UnsupportedConversion {
                from: self.kind(),
                to: ValueKind::Float,
            }),
        }
    }

    // =========================================================================
    // Canonical hashing
    // =========================================================================

    /// Feeds a deterministic structural representation into a caller-provided
    /// hasher.
    ///
    /// This method intentionally does not select a cryptographic hash
    /// algorithm. The hashing subsystem owns algorithm selection.
    ///
    /// `Parameter` currently does not expose `Hash`, so parameters are encoded
    /// through their canonical textual representation at this boundary.
    /// Serialization infrastructure should eventually replace that textual
    /// fallback with the canonical parameter encoder once the parameter schema
    /// is frozen.
    pub fn canonical_hash<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        let mut stack = vec![HashFrame::Value(self)];

        while let Some(frame) = stack.pop() {
            match frame {
                HashFrame::Value(value) => {
                    value.kind().hash(state);

                    match value {
                        Self::Bool(value) => {
                            value.hash(state);
                        }

                        Self::Integer(value) => {
                            value.value().hash(state);
                        }

                        Self::UnsignedInteger(value) => {
                            value.value().hash(state);
                        }

                        Self::Float(value) => {
                            value.bits().hash(state);
                        }

                        Self::Complex(value) => {
                            value.real().bits().hash(state);
                            value.imaginary().bits().hash(state);
                        }

                        Self::Angle(value) => {
                            value.radians_value().bits().hash(state);
                        }

                        Self::Duration(value) => {
                            value.as_attoseconds().hash(state);
                        }

                        Self::Frequency(value) => {
                            value.as_femtohertz().hash(state);
                        }

                        Self::Amplitude(value) => {
                            value.value().bits().hash(state);
                        }

                        Self::Phase(value) => {
                            value.radians_value().bits().hash(state);
                        }

                        Self::Qubit(value) => {
                            value.hash(state);
                        }

                        Self::PhysicalQubit(value) => {
                            value.hash(state);
                        }

                        Self::Parameter(value) => {
                            //
                            // Parameter's canonical semantic representation
                            // is owned by parameter.rs. Until that module
                            // exposes a formal Hash contract, use Display as
                            // a deterministic boundary representation.
                            //
                            value.to_string().hash(state);
                        }

                        Self::Reference(value) => {
                            value.hash(state);
                        }

                        Self::Array(array) => {
                            array.element_kind().hash(state);
                            array.len().hash(state);

                            stack.push(HashFrame::EndCollection);

                            for child in array.iter().rev() {
                                stack.push(HashFrame::Value(child));
                            }
                        }

                        Self::Tuple(tuple) => {
                            tuple.len().hash(state);

                            stack.push(HashFrame::EndCollection);

                            for child in tuple.iter().rev() {
                                stack.push(HashFrame::Value(child));
                            }
                        }

                        Self::Optional(optional) => {
                            match optional {
                                OptionalValue::None => {
                                    0u8.hash(state);
                                }

                                OptionalValue::Some(child) => {
                                    1u8.hash(state);
                                    stack.push(
                                        HashFrame::Value(child.as_ref()),
                                    );
                                }
                            }
                        }

                        Self::Unit => {}
                    }
                }

                HashFrame::EndCollection => {
                    0xFFu8.hash(state);
                }
            }
        }
    }
}

// =============================================================================
// Canonical hash traversal frame
// =============================================================================

enum HashFrame<'a> {
    Value(&'a Value),
    EndCollection,
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for Value {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Bool(value) => {
                write!(formatter, "{value}")
            }

            Self::Integer(value) => {
                write!(formatter, "{value}")
            }

            Self::UnsignedInteger(value) => {
                write!(formatter, "{value}")
            }

            Self::Float(value) => {
                write!(formatter, "{value}")
            }

            Self::Complex(value) => {
                write!(formatter, "{value}")
            }

            Self::Angle(value) => {
                write!(formatter, "{value}")
            }

            Self::Duration(value) => {
                write!(formatter, "{value}")
            }

            Self::Frequency(value) => {
                write!(formatter, "{value}")
            }

            Self::Amplitude(value) => {
                write!(formatter, "{value}")
            }

            Self::Phase(value) => {
                write!(formatter, "{value}")
            }

            Self::Qubit(value) => {
                write!(formatter, "{value}")
            }

            Self::PhysicalQubit(value) => {
                write!(formatter, "{value}")
            }

            Self::Parameter(value) => {
                write!(formatter, "{value}")
            }

            Self::Reference(value) => {
                write!(formatter, "%{value}")
            }

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

            Self::Unit => {
                formatter.write_str("()")
            }
        }
    }
}

// =============================================================================
// Primitive conversions
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
    fn finite_float_preserves_signed_zero() {
        let positive =
            FiniteFloat::new(0.0).expect("valid finite float");

        let negative =
            FiniteFloat::new(-0.0).expect("valid finite float");

        assert_ne!(positive, negative);
        assert_ne!(positive.bits(), negative.bits());
    }

    #[test]
    fn duration_nanoseconds_are_exact() {
        let duration =
            Duration::nanoseconds(20).expect("valid duration");

        assert_eq!(
            duration.as_attoseconds(),
            20_000_000_000
        );
    }

    #[test]
    fn duration_conversions_are_checked() {
        assert_eq!(
            Duration::seconds(u128::MAX),
            Err(ValueError::NumericOverflow)
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
    fn duration_subtraction_is_checked() {
        let left = Duration::attoseconds(1);
        let right = Duration::attoseconds(2);

        assert_eq!(
            left.checked_sub(right),
            Err(ValueError::NumericUnderflow)
        );
    }

    #[test]
    fn frequency_conversion_is_exact() {
        let frequency =
            Frequency::gigahertz(5).expect("valid frequency");

        assert_eq!(
            frequency.as_femtohertz(),
            5_000_000_000_000_000_000
        );
    }

    #[test]
    fn frequency_kilohertz_is_exact() {
        let frequency =
            Frequency::kilohertz(1).expect("valid frequency");

        assert_eq!(
            frequency.as_femtohertz(),
            FEMTOHERTZ_PER_KHZ
        );
    }

    #[test]
    fn integer_checked_operations_work() {
        let value = Integer::new(10);

        assert_eq!(
            value
                .checked_add(Integer::new(5))
                .expect("valid addition")
                .value(),
            15
        );

        assert_eq!(
            value
                .checked_sub(Integer::new(5))
                .expect("valid subtraction")
                .value(),
            5
        );

        assert_eq!(
            value
                .checked_mul(Integer::new(2))
                .expect("valid multiplication")
                .value(),
            20
        );
    }

    #[test]
    fn integer_division_by_zero_is_rejected() {
        assert_eq!(
            Integer::new(10).checked_div(Integer::new(0)),
            Err(ValueError::DivisionByZero)
        );
    }

    #[test]
    fn unsigned_subtraction_underflow_is_rejected() {
        assert_eq!(
            UnsignedInteger::new(1)
                .checked_sub(UnsignedInteger::new(2)),
            Err(ValueError::NumericUnderflow)
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
        let value =
            Value::amplitude(0.3).expect("valid amplitude");

        assert_eq!(
            value
                .as_amplitude()
                .expect("amplitude")
                .as_f64(),
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
    fn angle_accepts_finite_values() {
        let value =
            Value::angle_radians(3.141592653589793)
                .expect("valid angle");

        assert!(value.as_angle().is_some());
    }

    #[test]
    fn logical_qubit_uses_canonical_qubit_id() {
        let qubit = QubitId::new(123_456);

        let value = Value::qubit(qubit);

        assert_eq!(value.as_qubit(), Some(qubit));
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
        .expect("homogeneous array");

        assert_eq!(
            array
                .as_array()
                .expect("array")
                .len(),
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
    fn explicitly_typed_empty_arrays_are_supported() {
        let value =
            Value::empty_array(ValueKind::Qubit);

        assert_eq!(
            value
                .as_array()
                .expect("array")
                .element_kind(),
            ValueKind::Qubit
        );

        assert!(
            value
                .as_array()
                .expect("array")
                .is_empty()
        );
    }

    #[test]
    fn tuple_allows_mixed_types() {
        let tuple = Value::tuple(vec![
            Value::integer(1),
            Value::bool(true),
            Value::unit(),
        ]);

        assert_eq!(
            tuple
                .as_tuple()
                .expect("tuple")
                .len(),
            3
        );
    }

    #[test]
    fn optional_values_work() {
        let some = Value::some(Value::integer(7));
        let none = Value::none();

        assert!(
            some.as_optional()
                .expect("optional")
                .is_some()
        );

        assert!(
            none.as_optional()
                .expect("optional")
                .is_none()
        );
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
    fn traversal_policy_is_explicit() {
        let mut value = Value::integer(1);

        for _ in 0..16 {
            value = Value::tuple(vec![value]);
        }

        let policy = ValueTraversalPolicy::new(
            None,
            Some(8),
        );

        assert_eq!(
            value.validate_with_policy(policy),
            Err(ValueError::TraversalLimitExceeded)
        );
    }

    #[test]
    fn structural_metrics_are_iterative() {
        let value = Value::tuple(vec![
            Value::integer(1),
            Value::tuple(vec![
                Value::integer(2),
                Value::integer(3),
            ]),
        ]);

        assert_eq!(value.node_count(), 5);
        assert_eq!(value.depth(), 2);
    }

    #[test]
    fn symbols_are_deterministic_and_unique() {
        let first =
            Parameter::symbol("z").expect("valid symbol");
        let second =
            Parameter::symbol("a").expect("valid symbol");

        let value = Value::tuple(vec![
            Value::parameter(first),
            Value::parameter(second),
        ]);

        assert_eq!(
            value.collect_symbols(),
            vec![
                "a".to_string(),
                "z".to_string(),
            ]
        );
    }

    #[test]
    fn exact_integer_to_float_conversion_works() {
        let value = Value::integer(42);

        assert_eq!(
            value.to_f64_exact().expect("exact conversion"),
            42.0
        );
    }

    #[test]
    fn inexact_integer_to_float_conversion_is_rejected() {
        let value =
            Value::unsigned_integer((1u128 << 53) + 1);

        assert_eq!(
            value.to_f64_exact(),
            Err(ValueError::NumericLossOfPrecision)
        );
    }

    #[test]
    fn negative_integer_cannot_become_unsigned() {
        let value = Value::integer(-1);

        assert_eq!(
            value.to_u128(),
            Err(ValueError::NumericOverflow)
        );
    }

    #[test]
    fn canonical_hash_is_deterministic() {
        use std::collections::hash_map::DefaultHasher;

        let value = Value::tuple(vec![
            Value::integer(42),
            Value::bool(true),
            Value::duration_nanoseconds(20)
                .expect("valid duration"),
        ]);

        let mut first = DefaultHasher::new();
        value.canonical_hash(&mut first);

        let mut second = DefaultHasher::new();
        value.canonical_hash(&mut second);

        assert_eq!(
            first.finish(),
            second.finish()
        );
    }

    #[test]
    fn value_equality_is_structural() {
        let left = Value::tuple(vec![
            Value::integer(1),
            Value::bool(false),
        ]);

        let right = Value::tuple(vec![
            Value::integer(1),
            Value::bool(false),
        ]);

        assert_eq!(left, right);
    }

    #[test]
    fn unit_is_not_scalar() {
        assert!(!Value::unit().is_scalar());
    }

    #[test]
    fn qubit_references_are_distinguished_from_physical_qubits() {
        let logical =
            Value::qubit(QubitId::new(7));

        let physical =
            Value::physical_qubit(
                PhysicalQubitId::new(7),
            );

        assert_ne!(logical.kind(), physical.kind());
        assert_ne!(logical, physical);
    }
}