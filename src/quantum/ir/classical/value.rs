//! Zamani Quantum IR — Classical Value System
//!
//! Production-grade, hardware-independent representation of classical values
//! used by the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! This module owns the semantic representation of classical runtime/IR
//! values. It deliberately does not own:
//!
//! - classical-bit identity;
//! - classical-register declarations;
//! - quantum-bit identity;
//! - quantum operations;
//! - measurement semantics;
//! - control-flow semantics;
//! - scheduling;
//! - hardware memory;
//! - backend execution;
//! - simulator state;
//! - frontend ASTs;
//! - optimization policy.
//!
//! Those responsibilities belong to their respective IR modules.
//!
//! # Module boundary
//!
//! This file is intended to live at:
//!
//! ```text
//! src/quantum/ir/classical/value.rs
//! ```
//!
//! and is exposed through:
//!
//! ```text
//! src/quantum/ir/classical/mod.rs
//! ```
//!
//! The parent classical module remains the owner of classical resources,
//! while this file owns only classical value semantics.
//!
//! # Design principles
//!
//! A Zamani program is written once and may ultimately execute on:
//!
//! - a tiny quantum processor;
//! - a large quantum processor;
//! - a simulator;
//! - a fault-tolerant machine;
//! - a distributed quantum system;
//! - a future quantum architecture.
//!
//! Classical values therefore contain no assumptions about:
//!
//! - number of qubits;
//! - number of classical bits;
//! - hardware topology;
//! - vendor;
//! - CPU architecture;
//! - FPGA architecture;
//! - GPU architecture;
//! - ADC/DAC width;
//! - device memory layout.
//!
//! # Integer scalability
//!
//! Rust's primitive integers have finite widths. A universal IR must not make
//! `i64` or `u64` the semantic ceiling of a classical integer.
//!
//! Therefore this module provides:
//!
//! - `BigUint` — arbitrary-width unsigned integer;
//! - `BigInt` — arbitrary-width signed integer.
//!
//! Their magnitude is represented canonically as big-endian bytes.
//!
//! The practical limit is available memory and explicit compiler/runtime
//! resource policy, not a semantic machine-size constant.
//!
//! # Floating-point safety
//!
//! Semantic floating-point values are represented by `FiniteFloat`.
//!
//! NaN and positive/negative infinity are rejected at construction time.
//!
//! Equality and hashing use IEEE-754 bit representation. Consequently,
//! `+0.0` and `-0.0` remain distinct at the canonical value layer.
//!
//! # Determinism
//!
//! This module intentionally uses deterministic structures and representations.
//!
//! In particular:
//!
//! - integer magnitudes have canonical zero-free leading bytes;
//! - signed zero has one representation;
//! - floating-point values use their IEEE bits;
//! - arrays and tuples preserve order;
//! - maps are represented with `BTreeMap`;
//! - structural hashing does not depend on randomized hash-map ordering.
//!
//! # No unsafe
//!
//! This file explicitly forbids unsafe Rust.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition.
//!
//! No nightly features are required.
//! No external crates are required.
//!
//! # Integration
//!
//! `classical/mod.rs`
//!     re-exports the public types from this module.
//!
//! `classical.rs`
//!     must be migrated to `classical/mod.rs`; its classical resource types
//!     should remain there or be split into their own future submodules.
//!
//! `quantum::ir::identity::ValueId`
//!     identifies another IR value when `ClassicalValue::Reference` is used.
//!
//! `quantum::ir::parameter::Parameter`
//!     provides symbolic parameter semantics. Parameters are intentionally
//!     embedded rather than duplicated here.
//!
//! `quantum::ir::qubit`
//!     remains the canonical owner of quantum identity. This module does not
//!     duplicate `QubitId` or `PhysicalQubitId` because they are not classical
//!     values.
//!
//! `operation.rs`
//!     may use `ClassicalValue` for typed classical operands/results.
//!
//! `control_flow.rs`
//!     may consume boolean/bit values and evaluate predicates through its own
//!     control-flow semantics.
//!
//! `measurement.rs`
//!     may produce `Bit` or `BitVector` values.
//!
//! `parameter.rs`
//!     remains the owner of parameter-expression semantics.
//!
//! `validation.rs`
//!     may call `validate()` and inspect value kinds.
//!
//! `serialization.rs`
//!     should serialize the enum structurally using canonical integer bytes.
//!
//! `hash.rs`
//!     may use `canonical_bytes()` or `canonical_hash()`.
//!
//! `analysis.rs`
//!     may inspect value kinds without executing them.
//!
//! No hardware implementation should need to modify this file merely because
//! a new quantum architecture is introduced.
//!
//! # Important ownership rule
//!
//! `ClassicalValue` describes WHAT a classical value means.
//!
//! It does not decide:
//!
//! - WHERE it is stored;
//! - WHEN it is evaluated;
//! - HOW it is executed.
//!
//! Those decisions belong to mapping, scheduling, runtime and backend layers.

#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

// =============================================================================
// Value kind
// =============================================================================

/// Semantic category of a [`ClassicalValue`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassicalValueKind {
    /// Unit/no-value result.
    Unit,

    /// Boolean.
    Bool,

    /// One classical bit.
    Bit,

    /// Ordered classical bit vector.
    BitVector,

    /// Arbitrary-width signed integer.
    Integer,

    /// Arbitrary-width unsigned integer.
    UnsignedInteger,

    /// Finite IEEE-754 floating-point value.
    Float,

    /// Complex value composed of two finite floating-point values.
    Complex,

    /// Angle measured in radians.
    Angle,

    /// UTF-8 string.
    String,

    /// Homogeneous logical array.
    Array,

    /// Ordered heterogeneous tuple.
    Tuple,

    /// Optional value.
    Optional,

    /// Reference to an existing IR value.
    Reference,

    /// Symbolic/runtime parameter.
    Parameter,
}

impl fmt::Display for ClassicalValueKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Unit => "unit",
            Self::Bool => "bool",
            Self::Bit => "bit",
            Self::BitVector => "bit_vector",
            Self::Integer => "integer",
            Self::UnsignedInteger => "unsigned_integer",
            Self::Float => "float",
            Self::Complex => "complex",
            Self::Angle => "angle",
            Self::String => "string",
            Self::Array => "array",
            Self::Tuple => "tuple",
            Self::Optional => "optional",
            Self::Reference => "reference",
            Self::Parameter => "parameter",
        };

        f.write_str(name)
    }
}

// =============================================================================
// Error
// =============================================================================

/// Error produced by checked classical-value operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalValueError {
    /// A floating-point value is not finite.
    NonFiniteFloat,

    /// An integer representation is malformed.
    InvalidIntegerRepresentation,

    /// A signed integer has an invalid sign/magnitude combination.
    InvalidIntegerSign,

    /// An integer conversion cannot be represented by the requested type.
    IntegerOverflow,

    /// A conversion would lose information.
    NumericLossOfPrecision,

    /// A bit-vector width is inconsistent with its storage.
    InvalidBitVector,

    /// A bit-vector operation has an invalid index.
    BitIndexOutOfBounds,

    /// A collection operation would overflow its length.
    CollectionSizeOverflow,

    /// A recursive value has an invalid structure.
    InvalidStructure,

    /// A nested value has an unexpected type.
    TypeMismatch {
        /// Expected kind.
        expected: ClassicalValueKind,

        /// Actual kind.
        actual: ClassicalValueKind,
    },

    /// An array contains incompatible element kinds.
    HeterogeneousArray,

    /// A symbolic parameter could not be evaluated.
    ParameterEvaluationFailed,

    /// A referenced value is invalid in the current context.
    InvalidReference,
}

impl fmt::Display for ClassicalValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteFloat => {
                f.write_str("classical floating-point value must be finite")
            }

            Self::InvalidIntegerRepresentation => {
                f.write_str("integer representation is not canonical")
            }

            Self::InvalidIntegerSign => {
                f.write_str("integer sign is invalid for its magnitude")
            }

            Self::IntegerOverflow => {
                f.write_str("integer conversion overflowed")
            }

            Self::NumericLossOfPrecision => {
                f.write_str("numeric conversion would lose precision")
            }

            Self::InvalidBitVector => {
                f.write_str("bit-vector representation is invalid")
            }

            Self::BitIndexOutOfBounds => {
                f.write_str("bit-vector index is out of bounds")
            }

            Self::CollectionSizeOverflow => {
                f.write_str("classical collection size overflowed")
            }

            Self::InvalidStructure => {
                f.write_str("classical value structure is invalid")
            }

            Self::TypeMismatch { expected, actual } => {
                write!(
                    f,
                    "classical value type mismatch: expected {expected}, found {actual}"
                )
            }

            Self::HeterogeneousArray => {
                f.write_str("classical array contains incompatible value kinds")
            }

            Self::ParameterEvaluationFailed => {
                f.write_str("classical parameter evaluation failed")
            }

            Self::InvalidReference => {
                f.write_str("classical value contains an invalid IR reference")
            }
        }
    }
}

impl std::error::Error for ClassicalValueError {}

// =============================================================================
// Finite floating point
// =============================================================================

/// A finite IEEE-754 `f64`.
///
/// NaN and infinities cannot be constructed through [`Self::new`].
#[derive(Clone, Copy, Debug)]
pub struct FiniteFloat(f64);

impl FiniteFloat {
    /// Creates a finite floating-point value.
    pub fn new(value: f64) -> Result<Self, ClassicalValueError> {
        if !value.is_finite() {
            return Err(ClassicalValueError::NonFiniteFloat);
        }

        Ok(Self(value))
    }

    /// Returns the underlying `f64`.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// Returns the IEEE-754 bit representation.
    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0.to_bits()
    }

    /// Returns whether this value is zero.
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
        self.0.total_cmp(&other.0)
    }
}

impl fmt::Display for FiniteFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// =============================================================================
// Arbitrary-width unsigned integer
// =============================================================================

/// Arbitrary-width unsigned integer.
///
/// The magnitude is stored in canonical big-endian representation:
///
/// - zero is represented by an empty byte vector;
/// - non-zero values contain no leading zero bytes.
///
/// There is no fixed semantic width such as 32, 64, 128 or 256 bits.
///
/// The practical maximum is determined by available memory and explicit
/// compilation/runtime policy.
#[derive(Clone, Debug, Default)]
pub struct BigUint {
    magnitude: Vec<u8>,
}

impl BigUint {
    /// Creates zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            magnitude: Vec::new(),
        }
    }

    /// Creates a value from a `u128`.
    #[must_use]
    pub fn from_u128(value: u128) -> Self {
        if value == 0 {
            return Self::zero();
        }

        let bytes = value.to_be_bytes();
        let first = bytes
            .iter()
            .position(|byte| *byte != 0)
            .unwrap_or(bytes.len());

        Self {
            magnitude: bytes[first..].to_vec(),
        }
    }

    /// Creates a value from canonical big-endian magnitude bytes.
    ///
    /// Leading zero bytes are removed so the resulting representation is
    /// canonical.
    #[must_use]
    pub fn from_be_bytes(bytes: &[u8]) -> Self {
        let first = bytes
            .iter()
            .position(|byte| *byte != 0)
            .unwrap_or(bytes.len());

        Self {
            magnitude: bytes[first..].to_vec(),
        }
    }

    /// Returns the canonical big-endian magnitude.
    #[must_use]
    pub fn as_be_bytes(&self) -> &[u8] {
        &self.magnitude
    }

    /// Consumes this integer and returns its canonical magnitude bytes.
    #[must_use]
    pub fn into_be_bytes(self) -> Vec<u8> {
        self.magnitude
    }

    /// Returns whether the value is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.magnitude.is_empty()
    }

    /// Returns the number of significant bits.
    #[must_use]
    pub fn bit_width(&self) -> usize {
        match self.magnitude.first() {
            None => 0,

            Some(first) => {
                let leading = first.leading_zeros() as usize;

                self.magnitude
                    .len()
                    .saturating_mul(8)
                    .saturating_sub(leading)
            }
        }
    }

    /// Returns the number of significant bytes.
    #[must_use]
    pub fn byte_width(&self) -> usize {
        self.magnitude.len()
    }

    /// Converts to `u128` if representable.
    pub fn to_u128(&self) -> Result<u128, ClassicalValueError> {
        if self.magnitude.len() > 16 {
            return Err(ClassicalValueError::IntegerOverflow);
        }

        let mut bytes = [0u8; 16];
        let offset = 16 - self.magnitude.len();

        bytes[offset..].copy_from_slice(&self.magnitude);

        Ok(u128::from_be_bytes(bytes))
    }

    /// Returns a canonical zero.
    #[must_use]
    pub fn is_canonical(&self) -> bool {
        self.magnitude.is_empty()
            || self.magnitude.first().is_some_and(|byte| *byte != 0)
    }

    /// Validates the canonical representation.
    pub fn validate(&self) -> Result<(), ClassicalValueError> {
        if !self.is_canonical() {
            return Err(ClassicalValueError::InvalidIntegerRepresentation);
        }

        Ok(())
    }

    /// Compares two arbitrary-width unsigned integers.
    #[must_use]
    pub fn cmp_numeric(&self, other: &Self) -> Ordering {
        self.bit_width()
            .cmp(&other.bit_width())
            .then_with(|| self.magnitude.cmp(&other.magnitude))
    }
}

impl PartialEq for BigUint {
    fn eq(&self, other: &Self) -> bool {
        self.magnitude == other.magnitude
    }
}

impl Eq for BigUint {}

impl Hash for BigUint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.magnitude.hash(state);
    }
}

impl PartialOrd for BigUint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_numeric(other))
    }
}

impl Ord for BigUint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_numeric(other)
    }
}

impl From<u128> for BigUint {
    fn from(value: u128) -> Self {
        Self::from_u128(value)
    }
}

impl From<u64> for BigUint {
    fn from(value: u64) -> Self {
        Self::from_u128(value as u128)
    }
}

impl From<u32> for BigUint {
    fn from(value: u32) -> Self {
        Self::from_u128(value as u128)
    }
}

impl From<usize> for BigUint {
    fn from(value: usize) -> Self {
        Self::from_u128(value as u128)
    }
}

impl fmt::Display for BigUint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return f.write_str("0");
        }

        // Decimal formatting is deliberately implemented without requiring an
        // external big-integer crate. Repeated division operates on the
        // canonical byte magnitude and therefore works for arbitrary width.
        let mut bytes = self.magnitude.clone();
        let mut digits = Vec::new();

        while !bytes.is_empty() {
            let mut remainder = 0u16;
            let mut quotient = Vec::with_capacity(bytes.len());

            for byte in bytes {
                let value = (remainder << 8) | byte as u16;
                let q = value / 10;
                remainder = value % 10;

                if !quotient.is_empty() || q != 0 {
                    quotient.push(q as u8);
                }
            }

            digits.push((b'0' + remainder as u8) as char);
            bytes = quotient;
        }

        for digit in digits.iter().rev() {
            f.write_str(&digit.to_string())?;
        }

        Ok(())
    }
}

// =============================================================================
// Arbitrary-width signed integer
// =============================================================================

/// Sign of an arbitrary-width signed integer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Sign {
    /// Negative.
    Negative,

    /// Positive or zero.
    Positive,
}

/// Arbitrary-width signed integer.
///
/// Zero always has `Positive` sign and an empty magnitude.
#[derive(Clone, Debug)]
pub struct BigInt {
    sign: Sign,
    magnitude: BigUint,
}

impl BigInt {
    /// Creates zero.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            sign: Sign::Positive,
            magnitude: BigUint::zero(),
        }
    }

    /// Creates an integer from an `i128`.
    #[must_use]
    pub fn from_i128(value: i128) -> Self {
        if value == 0 {
            return Self::zero();
        }

        if value < 0 {
            let magnitude = value.unsigned_abs();

            Self {
                sign: Sign::Negative,
                magnitude: BigUint::from_u128(magnitude),
            }
        } else {
            Self {
                sign: Sign::Positive,
                magnitude: BigUint::from_u128(value as u128),
            }
        }
    }

    /// Creates a positive integer from an unsigned magnitude.
    #[must_use]
    pub fn positive(magnitude: BigUint) -> Self {
        if magnitude.is_zero() {
            return Self::zero();
        }

        Self {
            sign: Sign::Positive,
            magnitude,
        }
    }

    /// Creates a negative integer from an unsigned magnitude.
    ///
    /// Zero is normalized to positive zero.
    #[must_use]
    pub fn negative(magnitude: BigUint) -> Self {
        if magnitude.is_zero() {
            return Self::zero();
        }

        Self {
            sign: Sign::Negative,
            magnitude,
        }
    }

    /// Returns the sign.
    #[must_use]
    pub const fn sign(&self) -> Sign {
        self.sign
    }

    /// Returns the absolute magnitude.
    #[must_use]
    pub const fn magnitude(&self) -> &BigUint {
        &self.magnitude
    }

    /// Returns whether this is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.magnitude.is_zero()
    }

    /// Returns whether this is negative.
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        matches!(self.sign, Sign::Negative)
    }

    /// Returns the significant bit width.
    #[must_use]
    pub fn bit_width(&self) -> usize {
        self.magnitude.bit_width()
    }

    /// Converts to `i128` if representable.
    pub fn to_i128(&self) -> Result<i128, ClassicalValueError> {
        let magnitude = self.magnitude.to_u128()?;

        match self.sign {
            Sign::Positive => {
                i128::try_from(magnitude)
                    .map_err(|_| ClassicalValueError::IntegerOverflow)
            }

            Sign::Negative => {
                if magnitude > (i128::MAX as u128) + 1 {
                    return Err(ClassicalValueError::IntegerOverflow);
                }

                if magnitude == (i128::MAX as u128) + 1 {
                    Ok(i128::MIN)
                } else {
                    Ok(-(magnitude as i128))
                }
            }
        }
    }

    /// Returns whether the representation is canonical.
    #[must_use]
    pub fn is_canonical(&self) -> bool {
        self.magnitude.is_canonical()
            && (!self.magnitude.is_zero()
                || self.sign == Sign::Positive)
    }

    /// Validates the representation.
    pub fn validate(&self) -> Result<(), ClassicalValueError> {
        if !self.is_canonical() {
            return Err(ClassicalValueError::InvalidIntegerSign);
        }

        self.magnitude.validate()
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.sign == other.sign && self.magnitude == other.magnitude
    }
}

impl Eq for BigInt {}

impl Hash for BigInt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sign.hash(state);
        self.magnitude.hash(state);
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.sign, other.sign) {
            (Sign::Negative, Sign::Positive) => Ordering::Less,
            (Sign::Positive, Sign::Negative) => Ordering::Greater,

            (Sign::Positive, Sign::Positive) => {
                self.magnitude.cmp_numeric(&other.magnitude)
            }

            (Sign::Negative, Sign::Negative) => {
                other.magnitude.cmp_numeric(&self.magnitude)
            }
        }
    }
}

impl From<i128> for BigInt {
    fn from(value: i128) -> Self {
        Self::from_i128(value)
    }
}

impl From<i64> for BigInt {
    fn from(value: i64) -> Self {
        Self::from_i128(value as i128)
    }
}

impl From<i32> for BigInt {
    fn from(value: i32) -> Self {
        Self::from_i128(value as i128)
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_negative() {
            write!(f, "-")?;
        }

        self.magnitude.fmt(f)
    }
}

// =============================================================================
// Bit vector
// =============================================================================

/// Arbitrary-width classical bit vector.
///
/// Bits are stored in logical order:
///
/// ```text
/// index 0 -> first logical bit
/// index 1 -> second logical bit
/// ...
/// ```
///
/// Storage is compacted into bytes, with the unused high bits of the final
/// byte always zero.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BitVector {
    width: usize,
    bytes: Vec<u8>,
}

impl BitVector {
    /// Creates an all-zero bit vector.
    pub fn zeros(width: usize) -> Result<Self, ClassicalValueError> {
        let byte_count = width
            .checked_add(7)
            .ok_or(ClassicalValueError::CollectionSizeOverflow)?
            / 8;

        Ok(Self {
            width,
            bytes: vec![0; byte_count],
        })
    }

    /// Creates a bit vector from logical bits.
    ///
    /// The first element is logical bit zero.
    pub fn from_bits<I>(
        bits: I,
    ) -> Result<Self, ClassicalValueError>
    where
        I: IntoIterator<Item = bool>,
    {
        let mut vector = Self::zeros(0)?;

        for bit in bits {
            vector.push(bit)?;
        }

        Ok(vector)
    }

    /// Creates a vector from packed bytes.
    ///
    /// Bit zero is the least significant bit of byte zero.
    pub fn from_bytes(
        width: usize,
        mut bytes: Vec<u8>,
    ) -> Result<Self, ClassicalValueError> {
        let expected = width
            .checked_add(7)
            .ok_or(ClassicalValueError::CollectionSizeOverflow)?
            / 8;

        if bytes.len() != expected {
            return Err(ClassicalValueError::InvalidBitVector);
        }

        if width != 0 {
            let remainder = width % 8;

            if remainder != 0 {
                let mask = (1u8 << remainder) - 1;
                let last = bytes
                    .last_mut()
                    .ok_or(ClassicalValueError::InvalidBitVector)?;

                if *last & !mask != 0 {
                    return Err(ClassicalValueError::InvalidBitVector);
                }
            }
        }

        Ok(Self { width, bytes })
    }

    /// Returns the number of logical bits.
    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Returns whether the vector is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.width == 0
    }

    /// Returns the packed bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns whether the specified bit is set.
    pub fn get(
        &self,
        index: usize,
    ) -> Result<bool, ClassicalValueError> {
        if index >= self.width {
            return Err(ClassicalValueError::BitIndexOutOfBounds);
        }

        let byte = index / 8;
        let offset = index % 8;

        Ok((self.bytes[byte] & (1u8 << offset)) != 0)
    }

    /// Sets the specified bit.
    pub fn set(
        &mut self,
        index: usize,
        value: bool,
    ) -> Result<(), ClassicalValueError> {
        if index >= self.width {
            return Err(ClassicalValueError::BitIndexOutOfBounds);
        }

        let byte = index / 8;
        let offset = index % 8;
        let mask = 1u8 << offset;

        if value {
            self.bytes[byte] |= mask;
        } else {
            self.bytes[byte] &= !mask;
        }

        Ok(())
    }

    /// Appends one logical bit.
    pub fn push(
        &mut self,
        value: bool,
    ) -> Result<(), ClassicalValueError> {
        let new_width = self
            .width
            .checked_add(1)
            .ok_or(ClassicalValueError::CollectionSizeOverflow)?;

        let required_bytes = new_width
            .checked_add(7)
            .ok_or(ClassicalValueError::CollectionSizeOverflow)?
            / 8;

        if required_bytes > self.bytes.len() {
            self.bytes.push(0);
        }

        self.width = new_width;

        self.set(new_width - 1, value)
    }

    /// Returns an iterator over logical bits.
    pub fn iter(&self) -> BitVectorIter<'_> {
        BitVectorIter {
            vector: self,
            index: 0,
        }
    }

    /// Validates the representation.
    pub fn validate(&self) -> Result<(), ClassicalValueError> {
        let expected = self
            .width
            .checked_add(7)
            .ok_or(ClassicalValueError::CollectionSizeOverflow)?
            / 8;

        if self.bytes.len() != expected {
            return Err(ClassicalValueError::InvalidBitVector);
        }

        if self.width != 0 {
            let remainder = self.width % 8;

            if remainder != 0 {
                let mask = (1u8 << remainder) - 1;
                let last = self
                    .bytes
                    .last()
                    .ok_or(ClassicalValueError::InvalidBitVector)?;

                if *last & !mask != 0 {
                    return Err(ClassicalValueError::InvalidBitVector);
                }
            }
        }

        Ok(())
    }
}

/// Lazy logical-bit iterator.
pub struct BitVectorIter<'a> {
    vector: &'a BitVector,
    index: usize,
}

impl Iterator for BitVectorIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vector.width {
            return None;
        }

        let result = self.vector.get(self.index).ok();
        self.index = self.index.saturating_add(1);
        result
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.vector.width.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for BitVectorIter<'_> {}

impl std::iter::FusedIterator for BitVectorIter<'_> {}

// =============================================================================
// Complex
// =============================================================================

/// Finite complex scalar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComplexValue {
    /// Real component.
    pub real: FiniteFloat,

    /// Imaginary component.
    pub imaginary: FiniteFloat,
}

impl ComplexValue {
    /// Creates a finite complex scalar.
    pub fn new(
        real: f64,
        imaginary: f64,
    ) -> Result<Self, ClassicalValueError> {
        Ok(Self {
            real: FiniteFloat::new(real)?,
            imaginary: FiniteFloat::new(imaginary)?,
        })
    }

    /// Returns whether the value is exactly zero in both components.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imaginary.is_zero()
    }

    /// Validates both components.
    pub fn validate(&self) -> Result<(), ClassicalValueError> {
        let _ = FiniteFloat::new(self.real.get())?;
        let _ = FiniteFloat::new(self.imaginary.get())?;
        Ok(())
    }
}

// =============================================================================
// Classical value
// =============================================================================

/// Canonical classical value used by the Zamani Quantum IR.
///
/// The enum is intentionally semantic rather than hardware-specific.
///
/// It does not represent CPU registers, memory addresses, FPGA registers or
/// vendor-specific runtime buffers.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClassicalValue {
    /// Unit/no-value.
    Unit,

    /// Boolean.
    Bool(bool),

    /// One classical bit.
    Bit(bool),

    /// Arbitrary-width classical bit vector.
    BitVector(BitVector),

    /// Arbitrary-width signed integer.
    Integer(BigInt),

    /// Arbitrary-width unsigned integer.
    UnsignedInteger(BigUint),

    /// Finite floating-point value.
    Float(FiniteFloat),

    /// Finite complex value.
    Complex(ComplexValue),

    /// Angle in radians.
    Angle(FiniteFloat),

    /// UTF-8 string.
    String(String),

    /// Homogeneous logical array.
    Array(Vec<ClassicalValue>),

    /// Ordered heterogeneous tuple.
    Tuple(Vec<ClassicalValue>),

    /// Optional value.
    Optional(Option<Box<ClassicalValue>>),

    /// Reference to another IR value.
    Reference(super::super::identity::ValueId),

    /// Canonical symbolic parameter.
    Parameter(super::super::parameter::Parameter),
}

impl ClassicalValue {
    /// Returns the semantic kind.
    #[must_use]
    pub const fn kind(&self) -> ClassicalValueKind {
        match self {
            Self::Unit => ClassicalValueKind::Unit,
            Self::Bool(_) => ClassicalValueKind::Bool,
            Self::Bit(_) => ClassicalValueKind::Bit,
            Self::BitVector(_) => ClassicalValueKind::BitVector,
            Self::Integer(_) => ClassicalValueKind::Integer,
            Self::UnsignedInteger(_) => ClassicalValueKind::UnsignedInteger,
            Self::Float(_) => ClassicalValueKind::Float,
            Self::Complex(_) => ClassicalValueKind::Complex,
            Self::Angle(_) => ClassicalValueKind::Angle,
            Self::String(_) => ClassicalValueKind::String,
            Self::Array(_) => ClassicalValueKind::Array,
            Self::Tuple(_) => ClassicalValueKind::Tuple,
            Self::Optional(_) => ClassicalValueKind::Optional,
            Self::Reference(_) => ClassicalValueKind::Reference,
            Self::Parameter(_) => ClassicalValueKind::Parameter,
        }
    }

    /// Returns whether the value is scalar.
    #[must_use]
    pub const fn is_scalar(&self) -> bool {
        matches!(
            self,
            Self::Unit
                | Self::Bool(_)
                | Self::Bit(_)
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Complex(_)
                | Self::Angle(_)
        )
    }

    /// Creates a finite float.
    pub fn float(value: f64) -> Result<Self, ClassicalValueError> {
        Ok(Self::Float(FiniteFloat::new(value)?))
    }

    /// Creates a finite angle.
    pub fn angle(value: f64) -> Result<Self, ClassicalValueError> {
        Ok(Self::Angle(FiniteFloat::new(value)?))
    }

    /// Creates a finite complex value.
    pub fn complex(
        real: f64,
        imaginary: f64,
    ) -> Result<Self, ClassicalValueError> {
        Ok(Self::Complex(ComplexValue::new(
            real,
            imaginary,
        )?))
    }

    /// Creates an arbitrary-width signed integer.
    #[must_use]
    pub fn integer<T: Into<BigInt>>(value: T) -> Self {
        Self::Integer(value.into())
    }

    /// Creates an arbitrary-width unsigned integer.
    #[must_use]
    pub fn unsigned_integer<T: Into<BigUint>>(value: T) -> Self {
        Self::UnsignedInteger(value.into())
    }

    /// Creates a bit vector.
    #[must_use]
    pub fn bit_vector(vector: BitVector) -> Self {
        Self::BitVector(vector)
    }

    /// Creates a string value.
    #[must_use]
    pub fn string<S: Into<String>>(value: S) -> Self {
        Self::String(value.into())
    }

    /// Creates an optional value.
    #[must_use]
    pub fn optional(value: Option<Self>) -> Self {
        Self::Optional(value.map(Box::new))
    }

    /// Creates a tuple.
    #[must_use]
    pub fn tuple(values: Vec<Self>) -> Self {
        Self::Tuple(values)
    }

    /// Creates an array after requiring homogeneous element kinds.
    ///
    /// An empty array is valid because its element type is supplied by the
    /// surrounding IR type system rather than inferred from runtime values.
    pub fn array(
        values: Vec<Self>,
    ) -> Result<Self, ClassicalValueError> {
        if let Some(first) = values.first() {
            let expected = first.kind();

            if values
                .iter()
                .skip(1)
                .any(|value| value.kind() != expected)
            {
                return Err(ClassicalValueError::HeterogeneousArray);
            }
        }

        Ok(Self::Array(values))
    }

    /// Returns a boolean if this is a boolean value.
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a single bit if this is a bit value.
    #[must_use]
    pub const fn as_bit(&self) -> Option<bool> {
        match self {
            Self::Bit(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns a bit vector.
    #[must_use]
    pub fn as_bit_vector(&self) -> Option<&BitVector> {
        match self {
            Self::BitVector(value) => Some(value),
            _ => None,
        }
    }

    /// Returns a signed integer.
    #[must_use]
    pub fn as_integer(&self) -> Option<&BigInt> {
        match self {
            Self::Integer(value) => Some(value),
            _ => None,
        }
    }

    /// Returns an unsigned integer.
    #[must_use]
    pub fn as_unsigned_integer(&self) -> Option<&BigUint> {
        match self {
            Self::UnsignedInteger(value) => Some(value),
            _ => None,
        }
    }

    /// Returns a finite float.
    #[must_use]
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(value.get()),
            _ => None,
        }
    }

    /// Returns a complex value.
    #[must_use]
    pub fn as_complex(&self) -> Option<ComplexValue> {
        match self {
            Self::Complex(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns an angle in radians.
    #[must_use]
    pub fn as_angle(&self) -> Option<f64> {
        match self {
            Self::Angle(value) => Some(value.get()),
            _ => None,
        }
    }

    /// Returns the string.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value.as_str()),
            _ => None,
        }
    }

    /// Returns an array.
    #[must_use]
    pub fn as_array(&self) -> Option<&[Self]> {
        match self {
            Self::Array(values) => Some(values.as_slice()),
            _ => None,
        }
    }

    /// Returns a tuple.
    #[must_use]
    pub fn as_tuple(&self) -> Option<&[Self]> {
        match self {
            Self::Tuple(values) => Some(values.as_slice()),
            _ => None,
        }
    }

    /// Returns an optional value.
    #[must_use]
    pub fn as_optional(&self) -> Option<Option<&Self>> {
        match self {
            Self::Optional(value) => Some(value.as_deref()),
            _ => None,
        }
    }

    /// Returns the referenced IR value.
    #[must_use]
    pub fn as_reference(
        &self,
    ) -> Option<super::super::identity::ValueId> {
        match self {
            Self::Reference(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the symbolic parameter.
    #[must_use]
    pub fn as_parameter(
        &self,
    ) -> Option<&super::super::parameter::Parameter> {
        match self {
            Self::Parameter(value) => Some(value),
            _ => None,
        }
    }

    /// Validates the complete structural value.
    ///
    /// Validation is iterative for nested containers so validation does not
    /// intentionally consume Rust call-stack depth proportional to value
    /// nesting.
    pub fn validate(&self) -> Result<(), ClassicalValueError> {
        let mut stack = vec![self];

        while let Some(value) = stack.pop() {
            match value {
                Self::Unit
                | Self::Bool(_)
                | Self::Bit(_)
                | Self::Reference(_) => {}

                Self::BitVector(vector) => {
                    vector.validate()?;
                }

                Self::Integer(integer) => {
                    integer.validate()?;
                }

                Self::UnsignedInteger(integer) => {
                    integer.validate()?;
                }

                Self::Float(float) => {
                    let _ = FiniteFloat::new(float.get())?;
                }

                Self::Complex(complex) => {
                    complex.validate()?;
                }

                Self::Angle(angle) => {
                    let _ = FiniteFloat::new(angle.get())?;
                }

                Self::String(_) => {}

                Self::Array(values) => {
                    if let Some(first) = values.first() {
                        let kind = first.kind();

                        if values
                            .iter()
                            .skip(1)
                            .any(|item| item.kind() != kind)
                        {
                            return Err(
                                ClassicalValueError::HeterogeneousArray,
                            );
                        }
                    }

                    for child in values {
                        stack.push(child);
                    }
                }

                Self::Tuple(values) => {
                    for child in values {
                        stack.push(child);
                    }
                }

                Self::Optional(value) => {
                    if let Some(value) = value {
                        stack.push(value);
                    }
                }

                Self::Parameter(parameter) => {
                    parameter
                        .validate()
                        .map_err(|_| {
                            ClassicalValueError::ParameterEvaluationFailed
                        })?;
                }
            }
        }

        Ok(())
    }

    /// Returns the number of recursively contained value nodes.
    ///
    /// The traversal is iterative and therefore does not use recursive Rust
    /// calls.
    pub fn node_count(&self) -> usize {
        let mut count = 0usize;
        let mut stack = vec![self];

        while let Some(value) = stack.pop() {
            count = count.saturating_add(1);

            match value {
                Self::Array(values) | Self::Tuple(values) => {
                    for child in values {
                        stack.push(child);
                    }
                }

                Self::Optional(Some(value)) => {
                    stack.push(value);
                }

                _ => {}
            }
        }

        count
    }

    /// Returns the maximum nesting depth.
    ///
    /// A scalar has depth zero.
    pub fn depth(&self) -> usize {
        let mut maximum = 0usize;
        let mut stack = vec![(self, 0usize)];

        while let Some((value, depth)) = stack.pop() {
            maximum = maximum.max(depth);

            match value {
                Self::Array(values) | Self::Tuple(values) => {
                    for child in values {
                        stack.push((child, depth.saturating_add(1)));
                    }
                }

                Self::Optional(Some(value)) => {
                    stack.push((value, depth.saturating_add(1)));
                }

                _ => {}
            }
        }

        maximum
    }

    /// Produces deterministic canonical bytes for structural hashing.
    ///
    /// The representation is internal to the IR contract and deliberately
    /// avoids Rust's `Hash` implementation as a serialization format.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut output = Vec::new();
        self.write_canonical(&mut output);
        output
    }

    /// Writes canonical bytes into a caller-owned buffer.
    ///
    /// This allows callers to reuse allocation while constructing larger
    /// canonical IR representations.
    pub fn write_canonical(&self, output: &mut Vec<u8>) {
        match self {
            Self::Unit => {
                output.push(0);
            }

            Self::Bool(value) => {
                output.push(1);
                output.push(u8::from(*value));
            }

            Self::Bit(value) => {
                output.push(2);
                output.push(u8::from(*value));
            }

            Self::BitVector(vector) => {
                output.push(3);
                write_u64(output, vector.width() as u64);
                write_bytes(output, vector.as_bytes());
            }

            Self::Integer(value) => {
                output.push(4);
                output.push(match value.sign() {
                    Sign::Negative => 0,
                    Sign::Positive => 1,
                });
                write_bytes(output, value.magnitude().as_be_bytes());
            }

            Self::UnsignedInteger(value) => {
                output.push(5);
                write_bytes(output, value.as_be_bytes());
            }

            Self::Float(value) => {
                output.push(6);
                output.extend_from_slice(&value.bits().to_be_bytes());
            }

            Self::Complex(value) => {
                output.push(7);
                output.extend_from_slice(
                    &value.real.bits().to_be_bytes(),
                );
                output.extend_from_slice(
                    &value.imaginary.bits().to_be_bytes(),
                );
            }

            Self::Angle(value) => {
                output.push(8);
                output.extend_from_slice(&value.bits().to_be_bytes());
            }

            Self::String(value) => {
                output.push(9);
                write_bytes(output, value.as_bytes());
            }

            Self::Array(values) => {
                output.push(10);
                write_u64(output, values.len() as u64);

                for value in values {
                    value.write_canonical(output);
                }
            }

            Self::Tuple(values) => {
                output.push(11);
                write_u64(output, values.len() as u64);

                for value in values {
                    value.write_canonical(output);
                }
            }

            Self::Optional(None) => {
                output.push(12);
                output.push(0);
            }

            Self::Optional(Some(value)) => {
                output.push(12);
                output.push(1);
                value.write_canonical(output);
            }

            Self::Reference(value) => {
                output.push(13);
                let raw = value.value();
                output.extend_from_slice(&raw.to_be_bytes());
            }

            Self::Parameter(parameter) => {
                output.push(14);
                write_parameter_canonical(parameter, output);
            }
        }
    }

    /// Computes a deterministic structural hash using the supplied hasher.
    ///
    /// The caller owns the actual hash algorithm. This keeps the IR independent
    /// of any particular cryptographic implementation.
    pub fn hash_canonical<H: Hasher>(
        &self,
        state: &mut H,
    ) {
        self.write_hash(state);
    }

    fn write_hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Unit => {
                0u8.hash(state);
            }

            Self::Bool(value) => {
                1u8.hash(state);
                value.hash(state);
            }

            Self::Bit(value) => {
                2u8.hash(state);
                value.hash(state);
            }

            Self::BitVector(vector) => {
                3u8.hash(state);
                vector.width().hash(state);
                vector.as_bytes().hash(state);
            }

            Self::Integer(value) => {
                4u8.hash(state);
                value.hash(state);
            }

            Self::UnsignedInteger(value) => {
                5u8.hash(state);
                value.hash(state);
            }

            Self::Float(value) => {
                6u8.hash(state);
                value.hash(state);
            }

            Self::Complex(value) => {
                7u8.hash(state);
                value.hash(state);
            }

            Self::Angle(value) => {
                8u8.hash(state);
                value.hash(state);
            }

            Self::String(value) => {
                9u8.hash(state);
                value.hash(state);
            }

            Self::Array(values) => {
                10u8.hash(state);
                values.len().hash(state);

                for value in values {
                    value.write_hash(state);
                }
            }

            Self::Tuple(values) => {
                11u8.hash(state);
                values.len().hash(state);

                for value in values {
                    value.write_hash(state);
                }
            }

            Self::Optional(None) => {
                12u8.hash(state);
                0u8.hash(state);
            }

            Self::Optional(Some(value)) => {
                12u8.hash(state);
                1u8.hash(state);
                value.write_hash(state);
            }

            Self::Reference(value) => {
                13u8.hash(state);
                value.value().hash(state);
            }

            Self::Parameter(parameter) => {
                14u8.hash(state);
                write_parameter_hash(parameter, state);
            }
        }
    }
}

// =============================================================================
// Parameter canonicalization
// =============================================================================

fn write_parameter_canonical(
    parameter: &super::super::parameter::Parameter,
    output: &mut Vec<u8>,
) {
    match parameter {
        super::super::parameter::Parameter::Constant(value) => {
            output.push(0);
            output.extend_from_slice(&value.to_bits().to_be_bytes());
        }

        super::super::parameter::Parameter::Symbol(name) => {
            output.push(1);
            write_bytes(output, name.as_bytes());
        }

        super::super::parameter::Parameter::Expression(expression) => {
            output.push(2);
            write_parameter_expression_canonical(
                expression,
                output,
            );
        }
    }
}

fn write_parameter_expression_canonical(
    expression: &super::super::parameter::ParameterExpression,
    output: &mut Vec<u8>,
) {
    use super::super::parameter::ParameterExpression;

    match expression {
        ParameterExpression::Add(left, right) => {
            output.push(0);
            write_parameter_canonical(left, output);
            write_parameter_canonical(right, output);
        }

        ParameterExpression::Subtract(left, right) => {
            output.push(1);
            write_parameter_canonical(left, output);
            write_parameter_canonical(right, output);
        }

        ParameterExpression::Multiply(left, right) => {
            output.push(2);
            write_parameter_canonical(left, output);
            write_parameter_canonical(right, output);
        }

        ParameterExpression::Divide(left, right) => {
            output.push(3);
            write_parameter_canonical(left, output);
            write_parameter_canonical(right, output);
        }

        ParameterExpression::Negate(value) => {
            output.push(4);
            write_parameter_canonical(value, output);
        }
    }
}

fn write_parameter_hash<H: Hasher>(
    parameter: &super::super::parameter::Parameter,
    state: &mut H,
) {
    match parameter {
        super::super::parameter::Parameter::Constant(value) => {
            0u8.hash(state);
            value.to_bits().hash(state);
        }

        super::super::parameter::Parameter::Symbol(name) => {
            1u8.hash(state);
            name.hash(state);
        }

        super::super::parameter::Parameter::Expression(expression) => {
            2u8.hash(state);
            write_parameter_expression_hash(expression, state);
        }
    }
}

fn write_parameter_expression_hash<H: Hasher>(
    expression: &super::super::parameter::ParameterExpression,
    state: &mut H,
) {
    use super::super::parameter::ParameterExpression;

    match expression {
        ParameterExpression::Add(left, right) => {
            0u8.hash(state);
            write_parameter_hash(left, state);
            write_parameter_hash(right, state);
        }

        ParameterExpression::Subtract(left, right) => {
            1u8.hash(state);
            write_parameter_hash(left, state);
            write_parameter_hash(right, state);
        }

        ParameterExpression::Multiply(left, right) => {
            2u8.hash(state);
            write_parameter_hash(left, state);
            write_parameter_hash(right, state);
        }

        ParameterExpression::Divide(left, right) => {
            3u8.hash(state);
            write_parameter_hash(left, state);
            write_parameter_hash(right, state);
        }

        ParameterExpression::Negate(value) => {
            4u8.hash(state);
            write_parameter_hash(value, state);
        }
    }
}

// =============================================================================
// Canonical byte helpers
// =============================================================================

fn write_u64(output: &mut Vec<u8>, value: u64) {
    output.extend_from_slice(&value.to_be_bytes());
}

fn write_bytes(output: &mut Vec<u8>, bytes: &[u8]) {
    write_u64(output, bytes.len() as u64);
    output.extend_from_slice(bytes);
}

// =============================================================================
// Primitive conversions
// =============================================================================

impl From<bool> for ClassicalValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<BigInt> for ClassicalValue {
    fn from(value: BigInt) -> Self {
        Self::Integer(value)
    }
}

impl From<BigUint> for ClassicalValue {
    fn from(value: BigUint) -> Self {
        Self::UnsignedInteger(value)
    }
}

impl From<i128> for ClassicalValue {
    fn from(value: i128) -> Self {
        Self::Integer(BigInt::from_i128(value))
    }
}

impl From<i64> for ClassicalValue {
    fn from(value: i64) -> Self {
        Self::Integer(BigInt::from_i128(value as i128))
    }
}

impl From<i32> for ClassicalValue {
    fn from(value: i32) -> Self {
        Self::Integer(BigInt::from_i128(value as i128))
    }
}

impl From<u128> for ClassicalValue {
    fn from(value: u128) -> Self {
        Self::UnsignedInteger(BigUint::from_u128(value))
    }
}

impl From<u64> for ClassicalValue {
    fn from(value: u64) -> Self {
        Self::UnsignedInteger(BigUint::from_u128(value as u128))
    }
}

impl From<u32> for ClassicalValue {
    fn from(value: u32) -> Self {
        Self::UnsignedInteger(BigUint::from_u128(value as u128))
    }
}

impl From<BitVector> for ClassicalValue {
    fn from(value: BitVector) -> Self {
        Self::BitVector(value)
    }
}

impl From<String> for ClassicalValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<super::super::identity::ValueId> for ClassicalValue {
    fn from(value: super::super::identity::ValueId) -> Self {
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
    fn zero_unsigned_integer_is_canonical() {
        let value = BigUint::from_u128(0);

        assert!(value.is_zero());
        assert!(value.as_be_bytes().is_empty());
        assert!(value.is_canonical());
    }

    #[test]
    fn leading_zero_integer_bytes_are_normalized() {
        let value = BigUint::from_be_bytes(&[0, 0, 1, 2, 3]);

        assert_eq!(value.as_be_bytes(), &[1, 2, 3]);
        assert!(value.is_canonical());
    }

    #[test]
    fn signed_zero_is_normalized() {
        let value = BigInt::negative(BigUint::zero());

        assert!(!value.is_negative());
        assert!(value.is_zero());
        assert!(value.is_canonical());
    }

    #[test]
    fn arbitrary_width_unsigned_integer_does_not_use_fixed_width_semantics() {
        let bytes = vec![0x01; 128];
        let value = BigUint::from_be_bytes(&bytes);

        assert_eq!(value.byte_width(), 128);
        assert!(value.bit_width() > 1_000);
        assert!(value.to_u128().is_err());
    }

    #[test]
    fn arbitrary_width_signed_integer_is_ordered_correctly() {
        let negative = BigInt::negative(BigUint::from_u128(100));
        let positive = BigInt::positive(BigUint::from_u128(1));

        assert!(negative < positive);
    }

    #[test]
    fn finite_float_rejects_nan() {
        assert_eq!(
            FiniteFloat::new(f64::NAN),
            Err(ClassicalValueError::NonFiniteFloat)
        );
    }

    #[test]
    fn finite_float_rejects_infinity() {
        assert_eq!(
            FiniteFloat::new(f64::INFINITY),
            Err(ClassicalValueError::NonFiniteFloat)
        );
    }

    #[test]
    fn bit_vector_round_trip() {
        let mut vector = BitVector::zeros(10).unwrap();

        vector.set(0, true).unwrap();
        vector.set(9, true).unwrap();

        assert!(vector.get(0).unwrap());
        assert!(vector.get(9).unwrap());
        assert!(!vector.get(1).unwrap());
        assert!(vector.validate().is_ok());
    }

    #[test]
    fn bit_vector_rejects_out_of_range_access() {
        let vector = BitVector::zeros(8).unwrap();

        assert_eq!(
            vector.get(8),
            Err(ClassicalValueError::BitIndexOutOfBounds)
        );
    }

    #[test]
    fn homogeneous_arrays_are_accepted() {
        let array = ClassicalValue::array(vec![
            ClassicalValue::Bool(true),
            ClassicalValue::Bool(false),
        ]);

        assert!(array.is_ok());
    }

    #[test]
    fn heterogeneous_arrays_are_rejected() {
        let array = ClassicalValue::array(vec![
            ClassicalValue::Bool(true),
            ClassicalValue::Bit(false),
        ]);

        assert_eq!(
            array,
            Err(ClassicalValueError::HeterogeneousArray)
        );
    }

    #[test]
    fn nested_value_validation_is_iterative() {
        let value = ClassicalValue::Tuple(vec![
            ClassicalValue::Bool(true),
            ClassicalValue::Array(vec![
                ClassicalValue::Integer(BigInt::from_i128(42)),
                ClassicalValue::Integer(BigInt::from_i128(-7)),
            ]),
        ]);

        assert!(value.validate().is_ok());
        assert_eq!(value.node_count(), 5);
        assert_eq!(value.depth(), 2);
    }

    #[test]
    fn canonical_bytes_are_deterministic() {
        let value = ClassicalValue::Tuple(vec![
            ClassicalValue::Bool(true),
            ClassicalValue::Integer(BigInt::from_i128(-123)),
            ClassicalValue::String("zamani".to_owned()),
        ]);

        assert_eq!(
            value.canonical_bytes(),
            value.canonical_bytes()
        );
    }

    #[test]
    fn different_value_kinds_have_distinct_canonical_tags() {
        let bool_value = ClassicalValue::Bool(true);
        let bit_value = ClassicalValue::Bit(true);

        assert_ne!(
            bool_value.canonical_bytes(),
            bit_value.canonical_bytes()
        );
    }

    #[test]
    fn optional_none_is_distinct_from_unit() {
        assert_ne!(
            ClassicalValue::Unit,
            ClassicalValue::Optional(None)
        );
    }

    #[test]
    fn complex_value_requires_finite_components() {
        assert!(
            ClassicalValue::complex(1.0, 2.0).is_ok()
        );

        assert!(
            ClassicalValue::complex(
                f64::NAN,
                0.0
            )
            .is_err()
        );
    }
}