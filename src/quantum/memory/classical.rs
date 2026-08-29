//! Zamani Quantum Memory — Classical Runtime Memory
//!
//! This module owns the runtime storage of classical data associated with
//! quantum execution.
//!
//! # Architectural boundary
//!
//! `quantum::ir::ClassicalBitId` remains the canonical logical classical-bit
//! identity. This module deliberately does not redefine that identity.
//!
//! ```text
//! quantum::ir::ClassicalBitId
//!             │
//!             ▼
//! quantum::memory::classical
//!             │
//!      ┌──────┼─────────┐
//!      ▼      ▼         ▼
//!    bits   registers  values
//!      │      │         │
//!      └──────┼─────────┘
//!             ▼
//!       runtime/executor
//!             │
//!       ┌─────┴─────┐
//!       ▼           ▼
//!   simulator      hardware
//! ```
//!
//! The memory layer is hardware-independent. A QPU may support only a subset
//! of the value types or widths represented here; that is a backend capability
//! question and must not narrow the language/runtime memory model.
//!
//! Hardware adapters are responsible for lowering supported values into the
//! target QPU's native classical-control/readout representation.
//!
//! # Responsibilities
//!
//! This module provides:
//!
//! - classical-bit storage;
//! - explicit uninitialized state;
//! - arbitrary-width unsigned bit vectors;
//! - fixed-width signed two's-complement integers;
//! - booleans;
//! - finite floating-point values;
//! - durations;
//! - named classical registers;
//! - measurement-result destinations;
//! - runtime classical conditions;
//! - optimistic grouped writes;
//! - thread-safe shared memory;
//! - immutable snapshots;
//! - generation tracking;
//! - explicit resource limits;
//! - deterministic register ordering;
//! - serialization-compatible data structures.
//!
//! It does NOT provide:
//!
//! - quantum measurement sampling;
//! - quantum-state collapse;
//! - QPU communication;
//! - hardware register addresses;
//! - pulse generation;
//! - routing;
//! - scheduling;
//! - classical-expression parsing;
//! - compiler frontend semantics;
//! - benchmark execution.
//!
//! # Safety
//!
//! This module contains no `unsafe` code and exposes no raw pointers.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021.
//!
//! No nightly features are required.
//!
//! # Integration contract
//!
//! Later memory modules must treat this file as the classical-memory contract.
//!
//! `measurement.rs` writes measurement results through `write_measurement`.
//!
//! `reset.rs` may invalidate or rewrite classical locations when the execution
//! model requires a new classical epoch.
//!
//! `snapshot.rs` and `checkpoint.rs` may serialize/restore
//! `ClassicalMemorySnapshot` and `ClassicalMemory`.
//!
//! `state.rs` and state representations must not depend on classical storage
//! internals; they should consume measurement destinations/results through the
//! public API.
//!
//! `runtime` may use `ClassicalMemory` directly or
//! `ClassicalSynchronizedMemory` for concurrent execution.
//!
//! `hardware` must translate between QPU-specific readout/control data and
//! `ClassicalBitId`/classical values rather than introducing hardware-specific
//! types here.
//!
//! `routing` and `scheduling` may reference classical dependencies but must not
//! own classical storage.
//!
//! `benchmarking` may snapshot and inspect classical-memory telemetry but must
//! not become a dependency of this module.

use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::quantum::ir::ClassicalBitId;

// =============================================================================
// Conservative standalone limits
// =============================================================================

/// Conservative hard ceiling used when a caller has not yet supplied the
/// complete quantum-memory limit policy.
///
/// The normal production path should construct `ClassicalMemory` with
/// `ClassicalMemoryLimits` derived from the wider quantum memory policy.
pub const DEFAULT_MAX_CLASSICAL_BITS: usize = 1 << 30;

/// Maximum arbitrary-width integer/bit-vector width accepted by this module.
///
/// The wider memory subsystem may choose a smaller limit.
pub const DEFAULT_MAX_BIT_WIDTH: usize = 1 << 20;

/// Maximum number of named runtime classical registers.
pub const DEFAULT_MAX_REGISTERS: usize = 1 << 20;

// =============================================================================
// Error model
// =============================================================================

/// Result type for classical-memory operations.
pub type ClassicalResult<T> = Result<T, ClassicalMemoryError>;

/// Complete error vocabulary for classical runtime memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalMemoryError {
    /// A classical identifier is outside the allocated namespace.
    OutOfBounds {
        index: usize,
        len: usize,
    },

    /// A non-empty bit vector/register was required.
    ZeroWidth,

    /// A configured resource limit was exceeded.
    LimitExceeded {
        resource: &'static str,
        requested: usize,
        maximum: usize,
    },

    /// Checked arithmetic overflowed.
    ArithmeticOverflow {
        operation: &'static str,
    },

    /// A classical location has not received a value.
    Uninitialized {
        bit: ClassicalBitId,
    },

    /// A register contains at least one uninitialized bit.
    RegisterUninitialized {
        first_uninitialized: usize,
    },

    /// A register name is invalid.
    InvalidName,

    /// A register name is already present.
    DuplicateName(String),

    /// A requested register does not exist.
    UnknownName(String),

    /// The number of words does not match a bit-vector width.
    InvalidWordCount {
        width: usize,
        expected_words: usize,
        actual_words: usize,
    },

    /// Unused bits in the last word are non-zero.
    NonCanonicalBits,

    /// A value cannot be represented by the requested type/width.
    ValueOutOfRange,

    /// Two classical values cannot participate in the requested operation.
    TypeMismatch {
        expected: &'static str,
        actual: &'static str,
    },

    /// An operation requires a known value.
    UnknownValue,

    /// A floating-point value is NaN or infinite.
    NonFiniteFloat,

    /// A condition is malformed or semantically incompatible.
    InvalidCondition(&'static str),

    /// A concurrent memory lock was poisoned.
    LockPoisoned,

    /// Optimistic transaction generation mismatch.
    ConcurrentModification {
        expected_generation: u64,
        actual_generation: u64,
    },

    /// Serialization/version boundary failure.
    Serialization(&'static str),
}

impl fmt::Display for ClassicalMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds { index, len } => {
                write!(f, "classical index {index} is outside 0..{len}")
            }

            Self::ZeroWidth => {
                f.write_str("classical bit-vector/register width must be non-zero")
            }

            Self::LimitExceeded {
                resource,
                requested,
                maximum,
            } => write!(
                f,
                "classical {resource} request {requested} exceeds maximum {maximum}"
            ),

            Self::ArithmeticOverflow { operation } => {
                write!(
                    f,
                    "classical-memory arithmetic overflow during {operation}"
                )
            }

            Self::Uninitialized { bit } => {
                write!(f, "classical bit {bit} is uninitialized")
            }

            Self::RegisterUninitialized {
                first_uninitialized,
            } => write!(
                f,
                "classical register contains an uninitialized bit at index \
                 {first_uninitialized}"
            ),

            Self::InvalidName => {
                f.write_str("classical register name must be non-empty")
            }

            Self::DuplicateName(name) => {
                write!(f, "classical register name already exists: {name}")
            }

            Self::UnknownName(name) => {
                write!(f, "unknown classical register: {name}")
            }

            Self::InvalidWordCount {
                width,
                expected_words,
                actual_words,
            } => write!(
                f,
                "bit-vector width {width} requires {expected_words} words, \
                 received {actual_words}"
            ),

            Self::NonCanonicalBits => {
                f.write_str(
                    "bit-vector contains non-zero unused high bits",
                )
            }

            Self::ValueOutOfRange => {
                f.write_str(
                    "classical value is outside the requested representation",
                )
            }

            Self::TypeMismatch { expected, actual } => {
                write!(
                    f,
                    "classical type mismatch: expected {expected}, got {actual}"
                )
            }

            Self::UnknownValue => {
                f.write_str("classical value is unknown")
            }

            Self::NonFiniteFloat => {
                f.write_str(
                    "classical floating-point value must be finite",
                )
            }

            Self::InvalidCondition(message) => {
                write!(f, "invalid classical condition: {message}")
            }

            Self::LockPoisoned => {
                f.write_str("classical memory lock is poisoned")
            }

            Self::ConcurrentModification {
                expected_generation,
                actual_generation,
            } => write!(
                f,
                "classical memory changed concurrently: expected generation \
                 {expected_generation}, actual {actual_generation}"
            ),

            Self::Serialization(message) => {
                write!(
                    f,
                    "classical-memory serialization error: {message}"
                )
            }
        }
    }
}

impl Error for ClassicalMemoryError {}

// =============================================================================
// Arbitrary-width bit vector
// =============================================================================

/// Arbitrary-width unsigned bit vector.
///
/// Words are stored least-significant first.
///
/// Bit `0` is the least significant bit.
///
/// Bits above the declared width are always zero.
///
/// This representation allows Zamani to model language-level classical
/// integers without restricting the memory layer to the width of one specific
/// QPU's classical-control unit.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BitVector {
    width: usize,
    words: Vec<u64>,
}

impl BitVector {
    /// Creates an all-zero bit vector.
    pub fn zero(width: usize) -> ClassicalResult<Self> {
        Self::validate_width(width)?;

        let word_count = Self::word_count_for_width(width)?;

        Ok(Self {
            width,
            words: vec![0; word_count],
        })
    }

    /// Creates a bit vector from a `u64`.
    ///
    /// The value must fit within the requested width.
    pub fn from_u64(value: u64, width: usize) -> ClassicalResult<Self> {
        Self::validate_width(width)?;

        if width < 64 && (value >> width) != 0 {
            return Err(ClassicalMemoryError::ValueOutOfRange);
        }

        let mut result = Self::zero(width)?;

        result.words[0] = value;
        result.canonicalize();

        Ok(result)
    }

    /// Creates a bit vector from little-endian `u64` words.
    ///
    /// The caller must provide exactly the number of words required by `width`.
    ///
    /// Unused high bits in the final word must already be zero.
    pub fn from_words(
        width: usize,
        words: Vec<u64>,
    ) -> ClassicalResult<Self> {
        Self::validate_width(width)?;

        let expected = Self::word_count_for_width(width)?;

        if words.len() != expected {
            return Err(ClassicalMemoryError::InvalidWordCount {
                width,
                expected_words: expected,
                actual_words: words.len(),
            });
        }

        let result = Self { width, words };

        if !result.has_canonical_unused_bits() {
            return Err(ClassicalMemoryError::NonCanonicalBits);
        }

        Ok(result)
    }

    /// Returns the declared width.
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Returns the number of backing words.
    pub fn word_count(&self) -> usize {
        self.words.len()
    }

    /// Returns the canonical little-endian words.
    pub fn words(&self) -> &[u64] {
        &self.words
    }

    /// Reads one logical bit.
    pub fn get_bit(&self, index: usize) -> ClassicalResult<bool> {
        if index >= self.width {
            return Err(ClassicalMemoryError::OutOfBounds {
                index,
                len: self.width,
            });
        }

        let word = index / 64;
        let offset = index % 64;

        Ok((self.words[word] & (1u64 << offset)) != 0)
    }

    /// Returns a copy with one bit changed.
    pub fn with_bit(
        &self,
        index: usize,
        value: bool,
    ) -> ClassicalResult<Self> {
        if index >= self.width {
            return Err(ClassicalMemoryError::OutOfBounds {
                index,
                len: self.width,
            });
        }

        let mut result = self.clone();

        let word = index / 64;
        let offset = index % 64;
        let mask = 1u64 << offset;

        if value {
            result.words[word] |= mask;
        } else {
            result.words[word] &= !mask;
        }

        Ok(result)
    }

    /// Converts to `u64`, failing if significant bits above bit 63 exist.
    pub fn to_u64(&self) -> ClassicalResult<u64> {
        if self.width > 64
            && self.words[1..]
                .iter()
                .any(|word| *word != 0)
        {
            return Err(ClassicalMemoryError::ValueOutOfRange);
        }

        Ok(self.words[0])
    }

    /// Returns true when all bits are zero.
    pub fn is_zero(&self) -> bool {
        self.words.iter().all(|word| *word == 0)
    }

    /// Returns true when bit zero is set.
    pub fn is_odd(&self) -> bool {
        (self.words[0] & 1) != 0
    }

    /// Returns the width-preserving bitwise complement.
    pub fn bit_not(&self) -> Self {
        let mut result = self.clone();

        for word in &mut result.words {
            *word = !*word;
        }

        result.canonicalize();

        result
    }

    /// Returns a width-preserving bitwise AND.
    pub fn bit_and(&self, rhs: &Self) -> ClassicalResult<Self> {
        self.ensure_same_width(rhs)?;

        let words = self
            .words
            .iter()
            .zip(&rhs.words)
            .map(|(left, right)| left & right)
            .collect();

        Ok(Self {
            width: self.width,
            words,
        })
    }

    /// Returns a width-preserving bitwise OR.
    pub fn bit_or(&self, rhs: &Self) -> ClassicalResult<Self> {
        self.ensure_same_width(rhs)?;

        let words = self
            .words
            .iter()
            .zip(&rhs.words)
            .map(|(left, right)| left | right)
            .collect();

        Ok(Self {
            width: self.width,
            words,
        })
    }

    /// Returns a width-preserving bitwise XOR.
    pub fn bit_xor(&self, rhs: &Self) -> ClassicalResult<Self> {
        self.ensure_same_width(rhs)?;

        let words = self
            .words
            .iter()
            .zip(&rhs.words)
            .map(|(left, right)| left ^ right)
            .collect();

        Ok(Self {
            width: self.width,
            words,
        })
    }

    /// Unsigned numeric comparison.
    pub fn unsigned_cmp(
        &self,
        rhs: &Self,
    ) -> ClassicalResult<Ordering> {
        self.ensure_same_width(rhs)?;

        for (left, right) in self
            .words
            .iter()
            .zip(rhs.words.iter())
            .rev()
        {
            match left.cmp(right) {
                Ordering::Equal => {}
                ordering => return Ok(ordering),
            }
        }

        Ok(Ordering::Equal)
    }

    /// Zero-extends the vector to `new_width`.
    pub fn zero_extend(
        &self,
        new_width: usize,
    ) -> ClassicalResult<Self> {
        if new_width < self.width {
            return Err(ClassicalMemoryError::ValueOutOfRange);
        }

        Self::validate_width(new_width)?;

        let mut result = Self::zero(new_width)?;

        for (destination, source) in
            result.words.iter_mut().zip(&self.words)
        {
            *destination = *source;
        }

        result.canonicalize();

        Ok(result)
    }

    /// Truncates to the low `new_width` bits.
    ///
    /// Increasing the width is treated as zero-extension.
    pub fn truncate(
        &self,
        new_width: usize,
    ) -> ClassicalResult<Self> {
        Self::validate_width(new_width)?;

        if new_width > self.width {
            return self.zero_extend(new_width);
        }

        let mut result = Self::zero(new_width)?;

        for (destination, source) in
            result.words.iter_mut().zip(&self.words)
        {
            *destination = *source;
        }

        result.canonicalize();

        Ok(result)
    }

    /// Returns a hexadecimal representation without a `0x` prefix.
    pub fn to_hex(&self) -> String {
        if self.width <= 64 {
            return format!("{:x}", self.words[0]);
        }

        let mut output = String::new();

        for word in self.words.iter().rev() {
            if output.is_empty() {
                output.push_str(&format!("{:x}", word));
            } else {
                output.push_str(&format!("{:016x}", word));
            }
        }

        if output.is_empty() {
            output.push('0');
        }

        output
    }

    fn validate_width(width: usize) -> ClassicalResult<()> {
        if width == 0 {
            return Err(ClassicalMemoryError::ZeroWidth);
        }

        if width > DEFAULT_MAX_BIT_WIDTH {
            return Err(ClassicalMemoryError::LimitExceeded {
                resource: "bit width",
                requested: width,
                maximum: DEFAULT_MAX_BIT_WIDTH,
            });
        }

        Ok(())
    }

    fn word_count_for_width(width: usize) -> ClassicalResult<usize> {
        width
            .checked_add(63)
            .ok_or(ClassicalMemoryError::ArithmeticOverflow {
                operation: "bit-vector word-count calculation",
            })
            .map(|value| value / 64)
    }

    fn has_canonical_unused_bits(&self) -> bool {
        let remainder = self.width % 64;

        if remainder == 0 {
            return true;
        }

        let mask = (1u64 << remainder) - 1;

        self.words
            .last()
            .copied()
            .unwrap_or(0)
            & !mask
            == 0
    }

    fn canonicalize(&mut self) {
        let remainder = self.width % 64;

        if remainder != 0 {
            let mask = (1u64 << remainder) - 1;

            if let Some(last) = self.words.last_mut() {
                *last &= mask;
            }
        }
    }

    fn ensure_same_width(&self, rhs: &Self) -> ClassicalResult<()> {
        if self.width != rhs.width {
            return Err(ClassicalMemoryError::TypeMismatch {
                expected: "equal-width bit vectors",
                actual: "different-width bit vectors",
            });
        }

        Ok(())
    }
}

impl BitAnd for BitVector {
    type Output = ClassicalResult<Self>;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.bit_and(&rhs)
    }
}

impl BitOr for BitVector {
    type Output = ClassicalResult<Self>;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.bit_or(&rhs)
    }
}

impl BitXor for BitVector {
    type Output = ClassicalResult<Self>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.bit_xor(&rhs)
    }
}

impl Not for BitVector {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.bit_not()
    }
}

// =============================================================================
// Classical values
// =============================================================================

/// Runtime classical values.
///
/// Integer widths are explicit. This avoids making the memory subsystem depend
/// on the native width of the host CPU or any particular QPU.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClassicalValue {
    /// One logical classical bit.
    Bit(bool),

    /// Fixed-width unsigned integer.
    UInt(BitVector),

    /// Fixed-width signed two's-complement integer.
    Int(BitVector),

    /// Language-level Boolean.
    Bool(bool),

    /// Finite IEEE-754 binary64 value.
    Float64(f64),

    /// Non-negative duration in picoseconds.
    DurationPs(u64),
}

impl ClassicalValue {
    /// Returns the stable semantic type name.
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Bit(_) => "bit",
            Self::UInt(_) => "uint",
            Self::Int(_) => "int",
            Self::Bool(_) => "bool",
            Self::Float64(_) => "float64",
            Self::DurationPs(_) => "duration",
        }
    }

    /// Returns the underlying bit vector for integer values.
    pub fn as_bits(&self) -> Option<&BitVector> {
        match self {
            Self::UInt(bits) | Self::Int(bits) => Some(bits),
            Self::Bit(_)
            | Self::Bool(_)
            | Self::Float64(_)
            | Self::DurationPs(_) => None,
        }
    }

    /// Converts a bit/bool to `bool`.
    pub fn as_bool(&self) -> ClassicalResult<bool> {
        match self {
            Self::Bit(value) | Self::Bool(value) => Ok(*value),

            _ => Err(ClassicalMemoryError::TypeMismatch {
                expected: "bit or bool",
                actual: self.type_name(),
            }),
        }
    }

    /// Converts an integer-like value to `u64`.
    pub fn as_u64(&self) -> ClassicalResult<u64> {
        match self {
            Self::Bit(value) | Self::Bool(value) => {
                Ok(u64::from(*value))
            }

            Self::UInt(bits) | Self::Int(bits) => bits.to_u64(),

            Self::DurationPs(value) => Ok(*value),

            Self::Float64(_) => Err(
                ClassicalMemoryError::TypeMismatch {
                    expected: "integer-like classical value",
                    actual: "float64",
                },
            ),
        }
    }

    /// Converts a signed integer-like value to `i64`.
    pub fn as_i64(&self) -> ClassicalResult<i64> {
        match self {
            Self::Int(bits) => signed_to_i64(bits),

            Self::Bit(value) | Self::Bool(value) => {
                Ok(i64::from(*value))
            }

            Self::UInt(bits) => {
                let value = bits.to_u64()?;

                i64::try_from(value)
                    .map_err(|_| ClassicalMemoryError::ValueOutOfRange)
            }

            Self::DurationPs(value) => i64::try_from(*value)
                .map_err(|_| ClassicalMemoryError::ValueOutOfRange),

            Self::Float64(_) => Err(
                ClassicalMemoryError::TypeMismatch {
                    expected: "integer-like classical value",
                    actual: "float64",
                },
            ),
        }
    }

    /// Validates the value.
    pub fn validate(&self) -> ClassicalResult<()> {
        if let Self::Float64(value) = self {
            if !value.is_finite() {
                return Err(ClassicalMemoryError::NonFiniteFloat);
            }
        }

        Ok(())
    }
}

// =============================================================================
// Runtime classical register identity
// =============================================================================

/// Runtime classical-register identity.
///
/// This is deliberately different from `quantum::ir::ClassicalRegister`.
///
/// The IR register describes logical program structure; this ID identifies a
/// runtime storage object.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct ClassicalRegisterId(usize);

impl ClassicalRegisterId {
    /// Creates a runtime register identifier.
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the zero-based runtime identifier.
    pub const fn index(self) -> usize {
        self.0
    }
}

impl From<usize> for ClassicalRegisterId {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<ClassicalRegisterId> for usize {
    fn from(value: ClassicalRegisterId) -> Self {
        value.index()
    }
}

impl fmt::Display for ClassicalRegisterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "cr{}", self.0)
    }
}

// =============================================================================
// Runtime classical register
// =============================================================================

/// Runtime classical register metadata.
///
/// The register contains canonical IR `ClassicalBitId`s in deterministic
/// order, but does not own the actual bit values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassicalRegisterMemory {
    id: ClassicalRegisterId,
    name: String,
    bits: Vec<ClassicalBitId>,
}

impl ClassicalRegisterMemory {
    /// Creates a contiguous register.
    pub fn new(
        id: ClassicalRegisterId,
        name: impl Into<String>,
        start_bit: usize,
        width: usize,
    ) -> ClassicalResult<Self> {
        let name = name.into();

        validate_name(&name)?;

        let end = start_bit.checked_add(width).ok_or(
            ClassicalMemoryError::ArithmeticOverflow {
                operation: "classical-register range",
            },
        )?;

        if width == 0 {
            return Err(ClassicalMemoryError::ZeroWidth);
        }

        let bits = (start_bit..end)
            .map(ClassicalBitId::new)
            .collect();

        Ok(Self { id, name, bits })
    }

    /// Creates a register over an explicit bit list.
    pub fn from_bits(
        id: ClassicalRegisterId,
        name: impl Into<String>,
        bits: Vec<ClassicalBitId>,
    ) -> ClassicalResult<Self> {
        let name = name.into();

        validate_name(&name)?;

        if bits.is_empty() {
            return Err(ClassicalMemoryError::ZeroWidth);
        }

        let mut seen =
            std::collections::HashSet::with_capacity(bits.len());

        for bit in &bits {
            if !seen.insert(bit.index()) {
                return Err(
                    ClassicalMemoryError::Serialization(
                        "duplicate classical bit in register",
                    ),
                );
            }
        }

        Ok(Self { id, name, bits })
    }

    /// Returns runtime register identity.
    pub const fn id(&self) -> ClassicalRegisterId {
        self.id
    }

    /// Returns register name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns canonical logical classical bits.
    pub fn bits(&self) -> &[ClassicalBitId] {
        &self.bits
    }

    /// Returns register width.
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Returns true when empty.
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }
}

// =============================================================================
// Classical bit state
// =============================================================================

/// Explicit three-state representation of a classical bit.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum ClassicalBitState {
    /// No result/value has been written.
    Uninitialized,

    /// Logical zero.
    Zero,

    /// Logical one.
    One,
}

impl ClassicalBitState {
    /// Converts into `Option<bool>`.
    pub const fn as_option(self) -> Option<bool> {
        match self {
            Self::Uninitialized => None,
            Self::Zero => Some(false),
            Self::One => Some(true),
        }
    }

    /// Returns true when initialized.
    pub const fn is_initialized(self) -> bool {
        !matches!(self, Self::Uninitialized)
    }
}

// =============================================================================
// Classical-memory limits
// =============================================================================

/// Limits owned by the classical-memory subsystem.
///
/// The wider `quantum::memory::limits` module can construct this policy and
/// pass it to `ClassicalMemory`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct ClassicalMemoryLimits {
    /// Maximum canonical logical classical bits.
    pub max_bits: usize,

    /// Maximum runtime registers.
    pub max_registers: usize,

    /// Maximum width of one arbitrary-width integer/bit-vector.
    pub max_bit_width: usize,

    /// Maximum packed bytes occupied by the classical-bit namespace.
    pub max_bit_bytes: usize,
}

impl Default for ClassicalMemoryLimits {
    fn default() -> Self {
        Self {
            max_bits: DEFAULT_MAX_CLASSICAL_BITS,
            max_registers: DEFAULT_MAX_REGISTERS,
            max_bit_width: DEFAULT_MAX_BIT_WIDTH,
            max_bit_bytes: DEFAULT_MAX_CLASSICAL_BITS / 8,
        }
    }
}

impl ClassicalMemoryLimits {
    /// Validates the policy itself.
    pub fn validate(&self) -> ClassicalResult<()> {
        if self.max_bits == 0
            || self.max_registers == 0
            || self.max_bit_width == 0
        {
            return Err(ClassicalMemoryError::ZeroWidth);
        }

        Ok(())
    }

    /// Returns packed bytes required for `bits`.
    pub fn packed_bit_bytes(
        &self,
        bits: usize,
    ) -> ClassicalResult<usize> {
        if bits > self.max_bits {
            return Err(ClassicalMemoryError::LimitExceeded {
                resource: "bits",
                requested: bits,
                maximum: self.max_bits,
            });
        }

        let bytes = bits
            .checked_add(7)
            .ok_or(ClassicalMemoryError::ArithmeticOverflow {
                operation: "packed classical-bit byte calculation",
            })?
            / 8;

        if bytes > self.max_bit_bytes {
            return Err(ClassicalMemoryError::LimitExceeded {
                resource: "classical bit bytes",
                requested: bytes,
                maximum: self.max_bit_bytes,
            });
        }

        Ok(bytes)
    }

    /// Validates a bit-vector width.
    pub fn validate_width(
        &self,
        width: usize,
    ) -> ClassicalResult<()> {
        if width == 0 {
            return Err(ClassicalMemoryError::ZeroWidth);
        }

        if width > self.max_bit_width {
            return Err(ClassicalMemoryError::LimitExceeded {
                resource: "bit width",
                requested: width,
                maximum: self.max_bit_width,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Classical runtime memory
// =============================================================================

/// Owning hardware-independent classical runtime memory.
///
/// The storage contains logical values only. Physical readout buffers,
/// hardware control registers, detector addresses, and transport buffers
/// remain owned by `quantum::hardware`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassicalMemory {
    limits: ClassicalMemoryLimits,
    bits: Vec<ClassicalBitState>,
    registers: Vec<ClassicalRegisterMemory>,
    generation: u64,
}

impl ClassicalMemory {
    /// Creates empty classical memory with the standalone default policy.
    pub fn new() -> Self {
        Self {
            limits: ClassicalMemoryLimits::default(),
            bits: Vec::new(),
            registers: Vec::new(),
            generation: 0,
        }
    }

    /// Creates empty classical memory with an explicit policy.
    pub fn with_limits(
        limits: ClassicalMemoryLimits,
    ) -> ClassicalResult<Self> {
        limits.validate()?;

        Ok(Self {
            limits,
            bits: Vec::new(),
            registers: Vec::new(),
            generation: 0,
        })
    }

    /// Returns the configured limits.
    pub const fn limits(&self) -> ClassicalMemoryLimits {
        self.limits
    }

    /// Returns the current monotonic memory generation.
    ///
    /// The generation changes after successful mutations.
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the number of canonical classical bits.
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// Returns whether there are no canonical bits.
    pub fn is_empty(&self) -> bool {
        self.bits.is_empty()
    }

    /// Allocates canonical classical bits.
    ///
    /// New bits begin explicitly uninitialized.
    ///
    /// The operation validates the complete resulting size before changing
    /// memory.
    pub fn allocate_bits(
        &mut self,
        count: usize,
    ) -> ClassicalResult<Vec<ClassicalBitId>> {
        let new_len = self
            .len()
            .checked_add(count)
            .ok_or(ClassicalMemoryError::ArithmeticOverflow {
                operation: "classical bit allocation",
            })?;

        self.limits.packed_bit_bytes(new_len)?;

        let start = self.len();

        let ids = (start..new_len)
            .map(ClassicalBitId::new)
            .collect::<Vec<_>>();

        self.bits
            .resize(new_len, ClassicalBitState::Uninitialized);

        self.bump_generation()?;

        Ok(ids)
    }

    /// Resets every classical bit to explicit uninitialized state.
    pub fn clear(&mut self) -> ClassicalResult<()> {
        for bit in &mut self.bits {
            *bit = ClassicalBitState::Uninitialized;
        }

        self.bump_generation()
    }

    /// Reads a classical bit.
    ///
    /// An uninitialized bit is an error rather than silently becoming zero.
    pub fn read_bit(
        &self,
        id: ClassicalBitId,
    ) -> ClassicalResult<bool> {
        match self.read_bit_state(id)? {
            ClassicalBitState::Zero => Ok(false),
            ClassicalBitState::One => Ok(true),

            ClassicalBitState::Uninitialized => {
                Err(ClassicalMemoryError::Uninitialized { bit: id })
            }
        }
    }

    /// Reads the explicit three-state bit.
    pub fn read_bit_state(
        &self,
        id: ClassicalBitId,
    ) -> ClassicalResult<ClassicalBitState> {
        self.bits
            .get(id.index())
            .copied()
            .ok_or(ClassicalMemoryError::OutOfBounds {
                index: id.index(),
                len: self.len(),
            })
    }

    /// Writes one classical bit.
    pub fn write_bit(
        &mut self,
        id: ClassicalBitId,
        value: bool,
    ) -> ClassicalResult<()> {
        let slot = self
            .bits
            .get_mut(id.index())
            .ok_or(ClassicalMemoryError::OutOfBounds {
                index: id.index(),
                len: self.len(),
            })?;

        *slot = if value {
            ClassicalBitState::One
        } else {
            ClassicalBitState::Zero
        };

        self.bump_generation()
    }

    /// Writes one bit and returns its previous state.
    pub fn swap_bit(
        &mut self,
        id: ClassicalBitId,
        value: bool,
    ) -> ClassicalResult<ClassicalBitState> {
        let slot = self
            .bits
            .get_mut(id.index())
            .ok_or(ClassicalMemoryError::OutOfBounds {
                index: id.index(),
                len: self.len(),
            })?;

        let old = *slot;

        *slot = if value {
            ClassicalBitState::One
        } else {
            ClassicalBitState::Zero
        };

        self.bump_generation()?;

        Ok(old)
    }

    /// Invalidates one classical bit.
    ///
    /// This is useful for dynamic-circuit epochs, discarded readout data, or
    /// execution boundaries where a previous value must not accidentally be
    /// reused.
    pub fn invalidate_bit(
        &mut self,
        id: ClassicalBitId,
    ) -> ClassicalResult<()> {
        let slot = self
            .bits
            .get_mut(id.index())
            .ok_or(ClassicalMemoryError::OutOfBounds {
                index: id.index(),
                len: self.len(),
            })?;

        *slot = ClassicalBitState::Uninitialized;

        self.bump_generation()
    }

    /// Reads a runtime register as an unsigned bit vector.
    pub fn read_register(
        &self,
        register: &ClassicalRegisterMemory,
    ) -> ClassicalResult<BitVector> {
        if register.is_empty() {
            return Err(ClassicalMemoryError::ZeroWidth);
        }

        let mut result = BitVector::zero(register.len())?;

        for (offset, bit) in register.bits().iter().enumerate() {
            let value = self.read_bit(*bit)?;
            result = result.with_bit(offset, value)?;
        }

        Ok(result)
    }

    /// Writes a bit vector to a runtime register.
    ///
    /// The entire width must match. No truncation occurs implicitly.
    pub fn write_register(
        &mut self,
        register: &ClassicalRegisterMemory,
        value: &BitVector,
    ) -> ClassicalResult<()> {
        if register.len() != value.width() {
            return Err(ClassicalMemoryError::TypeMismatch {
                expected: "bit-vector matching register width",
                actual: "different width",
            });
        }

        // Validate all accesses before mutating anything.
        for bit in register.bits() {
            if bit.index() >= self.len() {
                return Err(ClassicalMemoryError::OutOfBounds {
                    index: bit.index(),
                    len: self.len(),
                });
            }
        }

        let mut next_values = Vec::with_capacity(register.len());

        for offset in 0..register.len() {
            next_values.push(value.get_bit(offset)?);
        }

        for (bit, value) in
            register.bits().iter().zip(next_values)
        {
            self.bits[bit.index()] = if value {
                ClassicalBitState::One
            } else {
                ClassicalBitState::Zero
            };
        }

        self.bump_generation()
    }

    /// Allocates a named runtime register over newly allocated bits.
    ///
    /// The register is only published after all validation succeeds.
    pub fn allocate_register(
        &mut self,
        name: impl Into<String>,
        width: usize,
    ) -> ClassicalResult<ClassicalRegisterMemory> {
        if self.registers.len() >= self.limits.max_registers {
            let requested = self
                .registers
                .len()
                .checked_add(1)
                .unwrap_or(usize::MAX);

            return Err(ClassicalMemoryError::LimitExceeded {
                resource: "registers",
                requested,
                maximum: self.limits.max_registers,
            });
        }

        let name = name.into();

        validate_name(&name)?;

        if self
            .registers
            .iter()
            .any(|register| register.name() == name)
        {
            return Err(ClassicalMemoryError::DuplicateName(name));
        }

        if width == 0 {
            return Err(ClassicalMemoryError::ZeroWidth);
        }

        self.limits.validate_width(width)?;

        let ids = self.allocate_bits(width)?;

        let id = ClassicalRegisterId::new(self.registers.len());

        let register =
            ClassicalRegisterMemory::from_bits(id, name, ids)?;

        self.registers.push(register.clone());

        self.bump_generation()?;

        Ok(register)
    }

    /// Returns a runtime register by ID.
    pub fn register(
        &self,
        id: ClassicalRegisterId,
    ) -> ClassicalResult<&ClassicalRegisterMemory> {
        self.registers
            .get(id.index())
            .ok_or(ClassicalMemoryError::OutOfBounds {
                index: id.index(),
                len: self.registers.len(),
            })
    }

    /// Returns a runtime register by name.
    pub fn register_by_name(
        &self,
        name: &str,
    ) -> ClassicalResult<&ClassicalRegisterMemory> {
        self.registers
            .iter()
            .find(|register| register.name() == name)
            .ok_or_else(|| {
                ClassicalMemoryError::UnknownName(name.to_owned())
            })
    }

    /// Returns runtime register metadata in deterministic order.
    pub fn registers(&self) -> &[ClassicalRegisterMemory] {
        &self.registers
    }

    /// Writes a quantum measurement result into its canonical logical
    /// classical destination.
    ///
    /// Sampling and quantum-state collapse remain outside this module.
    pub fn write_measurement(
        &mut self,
        destination: ClassicalBitId,
        result: bool,
    ) -> ClassicalResult<()> {
        self.write_bit(destination, result)
    }

    /// Evaluates a dynamic-circuit condition.
    pub fn evaluate(
        &self,
        condition: &ClassicalCondition,
    ) -> ClassicalResult<bool> {
        condition.evaluate(self)
    }

    /// Creates an immutable snapshot.
    pub fn snapshot(&self) -> ClassicalMemorySnapshot {
        ClassicalMemorySnapshot {
            generation: self.generation,
            bits: self.bits.clone(),
            registers: self.registers.clone(),
        }
    }

    /// Creates an optimistic transaction against the current generation.
    pub fn transaction(&self) -> ClassicalTransaction {
        ClassicalTransaction {
            expected_generation: self.generation,
            writes: Vec::new(),
        }
    }

    fn bump_generation(&mut self) -> ClassicalResult<()> {
        self.generation = self.generation.checked_add(1).ok_or(
            ClassicalMemoryError::ArithmeticOverflow {
                operation: "classical memory generation",
            },
        )?;

        Ok(())
    }
}

impl Default for ClassicalMemory {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Immutable snapshot
// =============================================================================

/// Immutable classical-memory snapshot.
///
/// This type is appropriate for checkpointing, branching, replay, diagnostics,
/// and dynamic-control evaluation.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct ClassicalMemorySnapshot {
    generation: u64,
    bits: Vec<ClassicalBitState>,
    registers: Vec<ClassicalRegisterMemory>,
}

impl ClassicalMemorySnapshot {
    /// Returns the captured generation.
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Reads a bit.
    pub fn read_bit(
        &self,
        id: ClassicalBitId,
    ) -> ClassicalResult<bool> {
        match self
            .bits
            .get(id.index())
            .copied()
            .ok_or(ClassicalMemoryError::OutOfBounds {
                index: id.index(),
                len: self.bits.len(),
            })?
        {
            ClassicalBitState::Zero => Ok(false),
            ClassicalBitState::One => Ok(true),

            ClassicalBitState::Uninitialized => {
                Err(ClassicalMemoryError::Uninitialized { bit: id })
            }
        }
    }

    /// Returns register metadata captured by the snapshot.
    pub fn registers(&self) -> &[ClassicalRegisterMemory] {
        &self.registers
    }
}

// =============================================================================
// Optimistic transaction
// =============================================================================

/// Optimistic grouped classical write transaction.
///
/// Transactions prevent partially applied batches when a caller needs several
/// classical updates to become visible together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassicalTransaction {
    expected_generation: u64,
    writes: Vec<(ClassicalBitId, bool)>,
}

impl ClassicalTransaction {
    /// Returns the generation against which this transaction was created.
    pub const fn expected_generation(&self) -> u64 {
        self.expected_generation
    }

    /// Queues a classical-bit write.
    pub fn write_bit(
        &mut self,
        bit: ClassicalBitId,
        value: bool,
    ) {
        self.writes.push((bit, value));
    }

    /// Returns queued write count.
    pub fn len(&self) -> usize {
        self.writes.len()
    }

    /// Returns whether no writes are queued.
    pub fn is_empty(&self) -> bool {
        self.writes.is_empty()
    }

    /// Atomically commits all queued writes after validating every destination.
    pub fn commit(
        self,
        memory: &mut ClassicalMemory,
    ) -> ClassicalResult<()> {
        if memory.generation != self.expected_generation {
            return Err(
                ClassicalMemoryError::ConcurrentModification {
                    expected_generation: self.expected_generation,
                    actual_generation: memory.generation,
                },
            );
        }

        for (bit, _) in &self.writes {
            if bit.index() >= memory.len() {
                return Err(ClassicalMemoryError::OutOfBounds {
                    index: bit.index(),
                    len: memory.len(),
                });
            }
        }

        let write_count = self.writes.len();

        for (bit, value) in self.writes {
            memory.bits[bit.index()] = if value {
                ClassicalBitState::One
            } else {
                ClassicalBitState::Zero
            };
        }

        if write_count != 0 {
            memory.bump_generation()?;
        }

        Ok(())
    }
}

// =============================================================================
// Synchronized classical memory
// =============================================================================

/// Thread-safe classical-memory owner.
///
/// This is useful for runtimes executing concurrent shots, independent circuit
/// branches, or multiple classical-control tasks.
#[derive(Clone)]
pub struct ClassicalSynchronizedMemory {
    inner: Arc<RwLock<ClassicalMemory>>,
}

impl fmt::Debug for ClassicalSynchronizedMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClassicalSynchronizedMemory")
            .finish_non_exhaustive()
    }
}

impl ClassicalSynchronizedMemory {
    /// Wraps existing classical memory.
    pub fn new(memory: ClassicalMemory) -> Self {
        Self {
            inner: Arc::new(RwLock::new(memory)),
        }
    }

    /// Creates empty synchronized classical memory.
    pub fn empty() -> Self {
        Self::new(ClassicalMemory::new())
    }

    /// Executes a read-only operation.
    pub fn read<T, F>(
        &self,
        operation: F,
    ) -> ClassicalResult<T>
    where
        F: FnOnce(&ClassicalMemory) -> ClassicalResult<T>,
    {
        let guard = self
            .inner
            .read()
            .map_err(|_| ClassicalMemoryError::LockPoisoned)?;

        operation(&guard)
    }

    /// Executes an exclusive mutation.
    pub fn write<T, F>(
        &self,
        operation: F,
    ) -> ClassicalResult<T>
    where
        F: FnOnce(&mut ClassicalMemory) -> ClassicalResult<T>,
    {
        let mut guard = self
            .inner
            .write()
            .map_err(|_| ClassicalMemoryError::LockPoisoned)?;

        operation(&mut guard)
    }

    /// Takes a consistent snapshot.
    pub fn snapshot(
        &self,
    ) -> ClassicalResult<ClassicalMemorySnapshot> {
        self.read(|memory| Ok(memory.snapshot()))
    }
}

// =============================================================================
// Dynamic-circuit conditions
// =============================================================================

/// Hardware-independent classical condition.
///
/// This deliberately models execution conditions rather than becoming a
/// complete source-language expression AST. Full classical expression parsing
/// belongs to the frontend/IR layer.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum ClassicalCondition {
    /// Test one classical bit.
    Bit(ClassicalBitId),

    /// Compare a runtime register with a fixed-width value.
    RegisterEquals {
        register: ClassicalRegisterId,
        value: BitVector,
    },

    /// Compare two classical operands.
    Compare {
        left: ClassicalOperand,
        operator: ComparisonOperator,
        right: ClassicalOperand,
    },

    /// Logical AND.
    And(
        Box<ClassicalCondition>,
        Box<ClassicalCondition>,
    ),

    /// Logical OR.
    Or(
        Box<ClassicalCondition>,
        Box<ClassicalCondition>,
    ),

    /// Logical NOT.
    Not(Box<ClassicalCondition>),
}

impl ClassicalCondition {
    /// Evaluates this condition against classical runtime memory.
    pub fn evaluate(
        &self,
        memory: &ClassicalMemory,
    ) -> ClassicalResult<bool> {
        match self {
            Self::Bit(bit) => memory.read_bit(*bit),

            Self::RegisterEquals { register, value } => {
                let runtime_register = memory.register(*register)?;

                let actual =
                    memory.read_register(runtime_register)?;

                if actual.width() != value.width() {
                    return Err(
                        ClassicalMemoryError::InvalidCondition(
                            "register/value widths differ",
                        ),
                    );
                }

                Ok(actual == *value)
            }

            Self::Compare {
                left,
                operator,
                right,
            } => operator.evaluate(
                left.resolve(memory)?,
                right.resolve(memory)?,
            ),

            Self::And(left, right) => {
                Ok(left.evaluate(memory)?
                    && right.evaluate(memory)?)
            }

            Self::Or(left, right) => {
                Ok(left.evaluate(memory)?
                    || right.evaluate(memory)?)
            }

            Self::Not(condition) => {
                Ok(!condition.evaluate(memory)?)
            }
        }
    }
}

/// Operand of a runtime classical condition.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
)]
pub enum ClassicalOperand {
    /// One canonical IR classical bit.
    Bit(ClassicalBitId),

    /// One runtime classical register.
    Register(ClassicalRegisterId),

    /// Literal classical value.
    Constant(ClassicalValue),
}

impl ClassicalOperand {
    fn resolve(
        &self,
        memory: &ClassicalMemory,
    ) -> ClassicalResult<ClassicalValue> {
        match self {
            Self::Bit(bit) => {
                Ok(ClassicalValue::Bit(
                    memory.read_bit(*bit)?,
                ))
            }

            Self::Register(register) => {
                let runtime_register =
                    memory.register(*register)?;

                Ok(ClassicalValue::UInt(
                    memory.read_register(runtime_register)?,
                ))
            }

            Self::Constant(value) => {
                value.validate()?;
                Ok(value.clone())
            }
        }
    }
}

/// Comparison operators for dynamic quantum control.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

impl ComparisonOperator {
    /// Evaluates two resolved classical values.
    pub fn evaluate(
        &self,
        left: ClassicalValue,
        right: ClassicalValue,
    ) -> ClassicalResult<bool> {
        match (&left, &right) {
            (ClassicalValue::Bit(a), ClassicalValue::Bit(b))
            | (ClassicalValue::Bool(a), ClassicalValue::Bool(b)) => {
                compare_ordering(self, a.cmp(b))
            }

            (
                ClassicalValue::UInt(left),
                ClassicalValue::UInt(right),
            ) => compare_ordering(
                self,
                left.unsigned_cmp(right)?,
            ),

            (
                ClassicalValue::Int(left),
                ClassicalValue::Int(right),
            ) => compare_ordering(
                self,
                signed_cmp(left, right)?,
            ),

            (
                ClassicalValue::Float64(left),
                ClassicalValue::Float64(right),
            ) => {
                if !left.is_finite() || !right.is_finite() {
                    return Err(
                        ClassicalMemoryError::NonFiniteFloat,
                    );
                }

                compare_ordering(
                    self,
                    left.partial_cmp(right).ok_or(
                        ClassicalMemoryError::NonFiniteFloat,
                    )?,
                )
            }

            (
                ClassicalValue::DurationPs(left),
                ClassicalValue::DurationPs(right),
            ) => compare_ordering(self, left.cmp(right)),

            _ => Err(
                ClassicalMemoryError::InvalidCondition(
                    "incompatible operand types",
                ),
            ),
        }
    }
}

fn compare_ordering(
    operator: &ComparisonOperator,
    ordering: Ordering,
) -> ClassicalResult<bool> {
    Ok(match operator {
        ComparisonOperator::Equal => {
            ordering == Ordering::Equal
        }

        ComparisonOperator::NotEqual => {
            ordering != Ordering::Equal
        }

        ComparisonOperator::Less => {
            ordering == Ordering::Less
        }

        ComparisonOperator::LessOrEqual => {
            ordering != Ordering::Greater
        }

        ComparisonOperator::Greater => {
            ordering == Ordering::Greater
        }

        ComparisonOperator::GreaterOrEqual => {
            ordering != Ordering::Less
        }
    })
}

// =============================================================================
// Signed integer helpers
// =============================================================================

/// Interprets a two's-complement bit vector as `i64`.
///
/// Values wider than 64 bits cannot be represented by this convenience
/// conversion. The underlying `BitVector` remains fully arbitrary-width.
fn signed_to_i64(bits: &BitVector) -> ClassicalResult<i64> {
    if bits.width() == 0 || bits.width() > 64 {
        return Err(ClassicalMemoryError::ValueOutOfRange);
    }

    let raw = bits.to_u64()?;

    if bits.width() == 64 {
        return Ok(raw as i64);
    }

    let sign_bit = 1u64 << (bits.width() - 1);

    if raw & sign_bit == 0 {
        return i64::try_from(raw)
            .map_err(|_| ClassicalMemoryError::ValueOutOfRange);
    }

    let modulus = 1u64 << bits.width();
    let magnitude = modulus - raw;

    let magnitude = i64::try_from(magnitude)
        .map_err(|_| ClassicalMemoryError::ValueOutOfRange)?;

    Ok(-magnitude)
}

/// Signed two's-complement comparison without narrowing to `i64`.
fn signed_cmp(
    left: &BitVector,
    right: &BitVector,
) -> ClassicalResult<Ordering> {
    if left.width() != right.width() {
        return Err(
            ClassicalMemoryError::InvalidCondition(
                "signed integer widths differ",
            ),
        );
    }

    let left_negative =
        left.get_bit(left.width() - 1)?;

    let right_negative =
        right.get_bit(right.width() - 1)?;

    match (left_negative, right_negative) {
        (true, false) => Ok(Ordering::Less),

        (false, true) => Ok(Ordering::Greater),

        (false, false) => left.unsigned_cmp(right),

        (true, true) => left
            .unsigned_cmp(right)
            .map(|ordering| ordering.reverse()),
    }
}

fn validate_name(name: &str) -> ClassicalResult<()> {
    if name.trim().is_empty() {
        return Err(ClassicalMemoryError::InvalidName);
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
    fn bit_vector_preserves_width_and_value() {
        let value =
            BitVector::from_u64(0b1011, 4)
                .expect("valid bit vector");

        assert_eq!(value.width(), 4);
        assert_eq!(
            value.to_u64().expect("value fits"),
            0b1011
        );

        assert!(
            value.get_bit(0).expect("in range")
        );

        assert!(
            value.get_bit(1).expect("in range")
        );

        assert!(
            !value.get_bit(2).expect("in range")
        );

        assert!(
            value.get_bit(3).expect("in range")
        );
    }

    #[test]
    fn bit_vector_rejects_overflowing_u64() {
        assert!(
            BitVector::from_u64(16, 4).is_err()
        );
    }

    #[test]
    fn bit_vector_rejects_noncanonical_high_bits() {
        assert!(
            BitVector::from_words(
                65,
                vec![0, u64::MAX],
            )
            .is_err()
        );
    }

    #[test]
    fn bit_vector_bitwise_operations_preserve_width() {
        let left =
            BitVector::from_u64(0b1100, 4)
                .expect("valid");

        let right =
            BitVector::from_u64(0b1010, 4)
                .expect("valid");

        assert_eq!(
            left.bit_and(&right)
                .expect("and")
                .to_u64()
                .expect("fits"),
            0b1000
        );

        assert_eq!(
            left.bit_or(&right)
                .expect("or")
                .to_u64()
                .expect("fits"),
            0b1110
        );

        assert_eq!(
            left.bit_xor(&right)
                .expect("xor")
                .to_u64()
                .expect("fits"),
            0b0110
        );
    }

    #[test]
    fn classical_memory_starts_uninitialized() {
        let mut memory =
            ClassicalMemory::new();

        let bits =
            memory.allocate_bits(2)
                .expect("allocation");

        assert_eq!(
            memory
                .read_bit_state(bits[0])
                .expect("bit exists"),
            ClassicalBitState::Uninitialized
        );

        assert!(
            memory.read_bit(bits[0]).is_err()
        );
    }

    #[test]
    fn measurement_write_is_hardware_independent() {
        let mut memory =
            ClassicalMemory::new();

        let bit =
            memory.allocate_bits(1)
                .expect("allocation")[0];

        memory
            .write_measurement(bit, true)
            .expect("measurement write");

        assert!(
            memory.read_bit(bit)
                .expect("initialized")
        );
    }

    #[test]
    fn register_round_trip_is_deterministic() {
        let mut memory =
            ClassicalMemory::new();

        let register =
            memory
                .allocate_register("result", 4)
                .expect("register");

        let value =
            BitVector::from_u64(0b1010, 4)
                .expect("value");

        memory
            .write_register(&register, &value)
            .expect("write");

        assert_eq!(
            memory
                .read_register(&register)
                .expect("read"),
            value
        );
    }

    #[test]
    fn register_condition_evaluates() {
        let mut memory =
            ClassicalMemory::new();

        let register =
            memory
                .allocate_register("result", 2)
                .expect("register");

        let value =
            BitVector::from_u64(3, 2)
                .expect("value");

        memory
            .write_register(&register, &value)
            .expect("write");

        let condition =
            ClassicalCondition::RegisterEquals {
                register: register.id(),
                value,
            };

        assert!(
            memory
                .evaluate(&condition)
                .expect("condition")
        );
    }

    #[test]
    fn signed_two_complement_comparison_works() {
        let negative =
            BitVector::from_u64(0b1111, 4)
                .expect("value");

        let positive =
            BitVector::from_u64(0b0001, 4)
                .expect("value");

        assert_eq!(
            signed_cmp(&negative, &positive)
                .expect("comparison"),
            Ordering::Less
        );

        assert_eq!(
            signed_to_i64(&negative)
                .expect("conversion"),
            -1
        );
    }

    #[test]
    fn transaction_rejects_stale_generation() {
        let mut memory =
            ClassicalMemory::new();

        let bit =
            memory.allocate_bits(1)
                .expect("allocation")[0];

        let mut transaction =
            memory.transaction();

        transaction.write_bit(bit, true);

        memory
            .write_bit(bit, false)
            .expect("write");

        assert!(
            transaction
                .commit(&mut memory)
                .is_err()
        );

        assert!(
            !memory
                .read_bit(bit)
                .expect("initialized")
        );
    }

    #[test]
    fn transaction_validates_all_indexes_before_mutation() {
        let mut memory =
            ClassicalMemory::new();

        let bit =
            memory.allocate_bits(1)
                .expect("allocation")[0];

        let mut transaction =
            memory.transaction();

        transaction.write_bit(bit, true);

        transaction.write_bit(
            ClassicalBitId::new(99),
            true,
        );

        assert!(
            transaction
                .commit(&mut memory)
                .is_err()
        );

        assert!(
            memory.read_bit(bit).is_err()
        );
    }

    #[test]
    fn synchronized_memory_has_no_unsafe_requirement() {
        let memory =
            ClassicalSynchronizedMemory::empty();

        memory
            .write(|inner| {
                let bit =
                    inner
                        .allocate_bits(1)?
                        [0];

                inner.write_bit(bit, true)
            })
            .expect("write");

        assert!(
            memory
                .read(|inner| {
                    let bit =
                        ClassicalBitId::new(0);

                    inner.read_bit(bit)
                })
                .expect("read")
        );
    }

    #[test]
    fn snapshot_is_stable_after_mutation() {
        let mut memory =
            ClassicalMemory::new();

        let bit =
            memory.allocate_bits(1)
                .expect("allocation")[0];

        memory
            .write_bit(bit, true)
            .expect("write");

        let snapshot =
            memory.snapshot();

        memory
            .write_bit(bit, false)
            .expect("write");

        assert!(
            snapshot
                .read_bit(bit)
                .expect("snapshot read")
        );

        assert!(
            !memory
                .read_bit(bit)
                .expect("memory read")
        );
    }

    #[test]
    fn duplicate_register_names_are_rejected() {
        let mut memory =
            ClassicalMemory::new();

        memory
            .allocate_register("results", 4)
            .expect("first register");

        assert!(
            memory
                .allocate_register("results", 4)
                .is_err()
        );
    }

    #[test]
    fn invalid_register_width_is_rejected() {
        let mut memory =
            ClassicalMemory::new();

        assert!(
            memory
                .allocate_register("empty", 0)
                .is_err()
        );
    }
}