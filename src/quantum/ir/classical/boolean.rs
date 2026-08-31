//! Zamani Quantum IR — Canonical Boolean Value
//!
//! Production-grade, hardware-independent representation of a classical
//! Boolean value used by the Zamani Quantum Intermediate Representation.
//!
//! # Architectural role
//!
//! `boolean.rs` owns the semantic representation and deterministic operations
//! of a single classical Boolean value.
//!
//! A Boolean answers:
//!
//! > "Is this classical value logically true or false?"
//!
//! It does NOT own:
//!
//! - classical-bit identity;
//! - classical-register declarations;
//! - quantum-bit identity;
//! - measurement semantics;
//! - quantum operations;
//! - control-flow graphs;
//! - symbolic parameter expressions;
//! - hardware registers;
//! - CPU registers;
//! - FPGA registers;
//! - device memory;
//! - scheduling;
//! - routing;
//! - optimization policy;
//! - backend execution;
//! - simulator state;
//! - frontend syntax;
//! - vendor-specific behavior.
//!
//! Those responsibilities belong to the corresponding IR modules.
//!
//! # Canonical location
//!
//! ```text
//! src/quantum/ir/classical/boolean.rs
//! ```
//!
//! The intended parent module is:
//!
//! ```text
//! quantum::ir::classical
//! ```
//!
//! and the intended public path is:
//!
//! ```text
//! quantum::ir::classical::boolean::Boolean
//! ```
//!
//! A future `classical/mod.rs` may additionally re-export it as:
//!
//! ```text
//! quantum::ir::classical::Boolean
//! ```
//!
//! # Semantic distinction
//!
//! This type represents a Boolean VALUE.
//!
//! It must not be confused with:
//!
//! ```text
//! ClassicalBitId
//! ```
//!
//! A `ClassicalBitId` identifies a logical storage/resource location.
//!
//! `Boolean` represents the logical value stored in, produced by, or consumed
//! by classical computation.
//!
//! Conceptually:
//!
//! ```text
//! ClassicalBitId
//!       │
//!       │ identifies
//!       ▼
//! classical resource
//!       │
//!       │ contains/produces
//!       ▼
//! Boolean
//! ```
//!
//! A Boolean therefore has no knowledge of where the value is stored.
//!
//! # Quantum boundary
//!
//! A Boolean may ultimately originate from a quantum measurement:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//!             │
//!             ▼
//!        measurement
//!             │
//!             ▼
//!       ClassicalBitId
//!             │
//!             ▼
//!          Boolean
//! ```
//!
//! However, that relationship is intentionally NOT represented in this file.
//!
//! The measurement subsystem owns the relationship between:
//!
//! ```text
//! QubitId → ClassicalBitId
//! ```
//!
//! The classical subsystem owns the interpretation of the resulting value.
//!
//! This prevents a dependency cycle and preserves the canonical ownership
//! boundary:
//!
//! ```text
//! qubit.rs
//!     │
//!     ▼
//! measurement.rs
//!     │
//!     ▼
//! classical/boolean.rs
//! ```
//!
//! `boolean.rs` therefore does not import `quantum::ir::qubit`.
//!
//! # Universal-program principle
//!
//! Zamani programs are intended to be written once and lowered to different
//! quantum machines and execution technologies.
//!
//! Boolean semantics therefore contain no assumptions about:
//!
//! - number of qubits;
//! - number of classical bits;
//! - machine width;
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - hardware vendor;
//! - quantum technology;
//! - device topology;
//! - readout architecture;
//! - classical register width.
//!
//! Boolean itself is intrinsically one logical truth value, so its semantic
//! domain is exactly:
//!
//! ```text
//! false
//! true
//! ```
//!
//! This is not a scalability restriction. It is the mathematical definition
//! of a Boolean.
//!
//! Large Boolean collections are represented by higher-level structures such
//! as classical bit vectors, arrays, registers, and IR values. This file does
//! not allocate or manage those collections.
//!
//! # No artificial limits
//!
//! This module deliberately contains no:
//!
//! ```text
//! MAX_BITS
//! MAX_VALUES
//! MAX_OPERATIONS
//! MAX_REGISTERS
//! MAX_QUBITS
//! ```
//!
//! A compiler/runtime may impose resource limits elsewhere through explicit
//! policy such as `quantum::ir::core::limits`.
//!
//! Such limits must never be introduced into this semantic Boolean type.
//!
//! # Determinism
//!
//! Boolean semantics are completely deterministic.
//!
//! The canonical serialized representation is:
//!
//! ```text
//! false → 0x00
//! true  → 0x01
//! ```
//!
//! No platform-dependent representation is used.
//!
//! This makes Boolean values suitable for:
//!
//! - canonical serialization;
//! - structural hashing;
//! - program fingerprints;
//! - reproducible compilation;
//! - deterministic testing;
//! - cache keys;
//! - distributed compilation artifacts.
//!
//! # Equality and hashing
//!
//! Boolean equality is semantic equality.
//!
//! ```text
//! false == false
//! true  == true
//! false != true
//! ```
//!
//! Hashing is derived directly from the Boolean semantic value.
//!
//! No pointer, memory address, process identifier, or machine state is part
//! of the hash.
//!
//! # Classical computation
//!
//! This file provides primitive Boolean operations:
//!
//! - NOT;
//! - AND;
//! - OR;
//! - XOR;
//! - NAND;
//! - NOR;
//! - XNOR;
//! - implication;
//! - reverse implication;
//! - conditional selection;
//! - equality;
//! - inequality.
//!
//! These are pure semantic operations.
//!
//! They do not execute instructions on a CPU or quantum device.
//!
//! Higher-level expression/control-flow modules may use these operations while
//! constructing or evaluating classical expressions.
//!
//! # Constant-time consideration
//!
//! Boolean operations themselves are implemented without data-dependent
//! loops, allocation, or branching where practical.
//!
//! This does not claim that an entire compiler/runtime is constant-time.
//! Side-channel policy belongs to the appropriate execution/security layer.
//!
//! # Integration contract
//!
//! ## `classical/value.rs`
//!
//! `ClassicalValue` may use:
//!
//! ```text
//! ClassicalValue::Bool(Boolean)
//! ```
//!
//! The current value subsystem already identifies `Bool` as a semantic
//! classical value kind. `Boolean` provides the strongly typed scalar backing
//! representation for that kind.
//!
//! ## `types.rs`
//!
//! `IrType::Bool` is the semantic type corresponding to `Boolean`.
//!
//! This file does not import `IrType`, intentionally avoiding a type/value
//! dependency cycle.
//!
//! The relationship is:
//!
//! ```text
//! IrType::Bool
//!       ↕
//! Boolean
//! ```
//!
//! ## `classical/expression.rs`
//!
//! Boolean expressions may use:
//!
//! ```text
//! Boolean
//! ```
//!
//! as literal operands/results.
//!
//! Expression trees remain owned by `expression.rs`.
//!
//! ## `classical/predicate.rs`
//!
//! Predicates may evaluate or produce Boolean results.
//!
//! Predicate structure remains owned by `predicate.rs`.
//!
//! ## `control_flow.rs`
//!
//! Conditional branches may consume Boolean values or Boolean-producing
//! predicates.
//!
//! `boolean.rs` does not own branch semantics.
//!
//! ## `operation.rs`
//!
//! Classical operations may use Boolean operands/results.
//!
//! `operation.rs` owns operation identity and operand/result relationships.
//!
//! ## `measurement.rs`
//!
//! Measurement may produce a classical bit whose interpreted scalar value is
//! represented as `Boolean`.
//!
//! This file does not own measurement semantics.
//!
//! ## `serialization.rs`
//!
//! Canonical Boolean encoding must use:
//!
//! ```text
//! false = 0
//! true  = 1
//! ```
//!
//! No textual or platform-specific Boolean representation should become the
//! canonical wire representation.
//!
//! ## `hash.rs`
//!
//! Hashing may consume:
//!
//! ```text
//! Boolean::canonical_byte()
//! ```
//!
//! or the semantic Boolean value directly using its `Hash` implementation.
//!
//! ## `validation.rs`
//!
//! Validation may use:
//!
//! ```text
//! Boolean::validate()
//! ```
//!
//! The current representation cannot contain an invalid Boolean state because
//! construction is restricted to the two valid states.
//!
//! ## `analysis.rs`
//!
//! Analysis may inspect Boolean values without mutating them.
//!
//! ## `qubit.rs`
//!
//! No dependency is required.
//!
//! Quantum identity remains exclusively owned by:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! If a quantum measurement produces a Boolean, the measurement/operation
//! layer performs that integration.
//!
//! # Ownership rule
//!
//! This file owns:
//!
//! ```text
//! Boolean
//! BooleanError
//! Boolean operations
//! Boolean canonical representation
//! ```
//!
//! It does not own any higher-level IR graph.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition.
//!
//! Requirements:
//!
//! - stable Rust;
//! - no nightly features;
//! - no external crates;
//! - no `unsafe`;
//! - deterministic behavior.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # API stability principle
//!
//! The semantic API in this file should remain stable even if:
//!
//! - the compiler changes;
//! - the runtime changes;
//! - the quantum backend changes;
//! - the number of qubits increases;
//! - the number of classical resources increases;
//! - new quantum technologies are added;
//! - new dialects are introduced.
//!
//! New hardware should not require changes to this file merely because the
//! hardware has a different Boolean implementation.
//!
//! # Example
//!
//! ```
//! use std::ops::Not;
//!
//! use boolean::{Boolean, BooleanError};
//!
//! let value = Boolean::new(true);
//!
//! assert!(value.is_true());
//! assert!(!value.is_false());
//! assert_eq!(value.not(), Boolean::False);
//! assert_eq!(value.and(Boolean::True), Boolean::True);
//! assert_eq!(value.xor(Boolean::True), Boolean::False);
//!
//! assert_eq!(value.canonical_byte(), 1);
//! assert_eq!(Boolean::False.canonical_byte(), 0);
//!
//! assert_eq!(!value, Boolean::False);
//! assert_eq!(Boolean::try_from(1_u8), Ok(Boolean::True));
//! assert_eq!(Boolean::try_from(0_u8), Ok(Boolean::False));
//!
//! let _ = BooleanError::InvalidEncoding(2);
//! ```
//!
//! The example uses the file as an isolated Rust module. In the actual Zamani
//! tree it is exposed through `quantum::ir::classical`.

#![forbid(unsafe_code)]

use std::convert::TryFrom;
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

// =============================================================================
// Boolean
// =============================================================================

/// Canonical semantic Boolean value used by the Zamani Quantum IR.
///
/// `Boolean` contains exactly one logical truth value:
///
/// ```text
/// false
/// true
/// ```
///
/// It is deliberately distinct from:
///
/// - `bool` as a host-language implementation detail;
/// - `ClassicalBitId` as a logical storage identity;
/// - a classical register;
/// - a quantum measurement operation;
/// - a predicate expression.
///
/// The type is `Copy`, allocation-free, deterministic, and independent of
/// hardware.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Boolean {
    /// Logical false.
    False,

    /// Logical true.
    True,
}

impl Boolean {
    /// Constructs a Boolean from a host-language `bool`.
    ///
    /// This conversion is semantic and does not expose any hardware detail.
    #[must_use]
    pub const fn new(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }

    /// Returns the semantic host-language Boolean representation.
    #[must_use]
    pub const fn get(self) -> bool {
        match self {
            Self::False => false,
            Self::True => true,
        }
    }

    /// Returns `true` when this value is logically true.
    #[must_use]
    pub const fn is_true(self) -> bool {
        matches!(self, Self::True)
    }

    /// Returns `true` when this value is logically false.
    #[must_use]
    pub const fn is_false(self) -> bool {
        matches!(self, Self::False)
    }

    /// Returns the logical negation.
    #[must_use]
    pub const fn not(self) -> Self {
        match self {
            Self::False => Self::True,
            Self::True => Self::False,
        }
    }

    /// Logical conjunction.
    ///
    /// Equivalent to Boolean `AND`.
    #[must_use]
    pub const fn and(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::True, Self::True) => Self::True,
            _ => Self::False,
        }
    }

    /// Logical disjunction.
    ///
    /// Equivalent to Boolean `OR`.
    #[must_use]
    pub const fn or(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::False, Self::False) => Self::False,
            _ => Self::True,
        }
    }

    /// Logical exclusive disjunction.
    ///
    /// Equivalent to Boolean `XOR`.
    #[must_use]
    pub const fn xor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::False, Self::False) => Self::False,
            (Self::True, Self::True) => Self::False,
            _ => Self::True,
        }
    }

    /// Logical NAND.
    ///
    /// Equivalent to:
    ///
    /// ```text
    /// NOT (self AND rhs)
    /// ```
    #[must_use]
    pub const fn nand(self, rhs: Self) -> Self {
        self.and(rhs).not()
    }

    /// Logical NOR.
    ///
    /// Equivalent to:
    ///
    /// ```text
    /// NOT (self OR rhs)
    /// ```
    #[must_use]
    pub const fn nor(self, rhs: Self) -> Self {
        self.or(rhs).not()
    }

    /// Logical equivalence.
    ///
    /// Returns true when both operands have the same truth value.
    ///
    /// Equivalent to `XNOR`.
    #[must_use]
    pub const fn xnor(self, rhs: Self) -> Self {
        self.xor(rhs).not()
    }

    /// Logical implication.
    ///
    /// Returns:
    ///
    /// ```text
    /// self → rhs
    /// ```
    ///
    /// It is false only when `self` is true and `rhs` is false.
    #[must_use]
    pub const fn implies(self, rhs: Self) -> Self {
        self.not().or(rhs)
    }

    /// Reverse logical implication.
    ///
    /// Returns:
    ///
    /// ```text
    /// rhs → self
    /// ```
    #[must_use]
    pub const fn implied_by(self, rhs: Self) -> Self {
        rhs.implies(self)
    }

    /// Selects `when_true` when `self` is true, otherwise `when_false`.
    ///
    /// This is the semantic conditional-selection primitive.
    #[must_use]
    pub const fn select(
        self,
        when_true: Self,
        when_false: Self,
    ) -> Self {
        if self.is_true() {
            when_true
        } else {
            when_false
        }
    }

    /// Returns whether both values are logically equal.
    #[must_use]
    pub const fn equals(self, rhs: Self) -> Self {
        self.xnor(rhs)
    }

    /// Returns whether both values are logically different.
    #[must_use]
    pub const fn not_equals(self, rhs: Self) -> Self {
        self.xor(rhs)
    }

    /// Returns the canonical serialization byte.
    ///
    /// Canonical representation:
    ///
    /// ```text
    /// false = 0x00
    /// true  = 0x01
    /// ```
    #[must_use]
    pub const fn canonical_byte(self) -> u8 {
        match self {
            Self::False => 0,
            Self::True => 1,
        }
    }

    /// Returns the canonical one-byte serialization.
    ///
    /// The returned array has a fixed size because a Boolean has exactly one
    /// bit of semantic information.
    #[must_use]
    pub const fn canonical_bytes(self) -> [u8; 1] {
        [self.canonical_byte()]
    }

    /// Creates a Boolean from its canonical serialization byte.
    ///
    /// Only `0` and `1` are valid canonical encodings.
    pub const fn from_canonical_byte(
        byte: u8,
    ) -> Result<Self, BooleanError> {
        match byte {
            0 => Ok(Self::False),
            1 => Ok(Self::True),
            other => Err(BooleanError::InvalidEncoding(other)),
        }
    }

    /// Creates a Boolean from a single-byte canonical representation.
    ///
    /// The slice must contain exactly one byte.
    pub fn from_canonical_bytes(
        bytes: &[u8],
    ) -> Result<Self, BooleanError> {
        if bytes.len() != 1 {
            return Err(BooleanError::InvalidLength {
                expected: 1,
                actual: bytes.len(),
            });
        }

        Self::from_canonical_byte(bytes[0])
    }

    /// Returns whether a byte is a valid canonical Boolean encoding.
    #[must_use]
    pub const fn is_valid_canonical_byte(byte: u8) -> bool {
        matches!(byte, 0 | 1)
    }

    /// Validates this Boolean.
    ///
    /// All values constructible by this type are valid. The method exists as a
    /// uniform validation hook for the wider IR validation architecture.
    pub const fn validate(self) -> Result<(), BooleanError> {
        Ok(())
    }

    /// Returns a canonical numeric representation suitable for deterministic
    /// low-level encoding.
    ///
    /// This is intentionally `u8`, not `usize`, so serialization is independent
    /// of host architecture.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.canonical_byte()
    }

    /// Creates a Boolean from a canonical numeric representation.
    ///
    /// Only `0` and `1` are accepted.
    pub const fn from_u8(value: u8) -> Result<Self, BooleanError> {
        Self::from_canonical_byte(value)
    }
}

// =============================================================================
// Error
// =============================================================================

/// Errors produced by checked Boolean conversions and canonical decoding.
///
/// Boolean operations themselves cannot fail because their domain is closed:
/// every Boolean operation maps valid Boolean values to another valid Boolean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BooleanError {
    /// The serialized Boolean contained a value other than `0` or `1`.
    InvalidEncoding(u8),

    /// The serialized representation did not contain exactly one byte.
    InvalidLength {
        /// Required canonical byte length.
        expected: usize,

        /// Actual byte length.
        actual: usize,
    },
}

impl fmt::Display for BooleanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEncoding(value) => {
                write!(
                    formatter,
                    "invalid Boolean canonical encoding: {value}; expected 0 or 1"
                )
            }

            Self::InvalidLength { expected, actual } => {
                write!(
                    formatter,
                    "invalid Boolean encoding length: expected {expected} byte, found {actual}"
                )
            }
        }
    }
}

impl std::error::Error for BooleanError {}

// =============================================================================
// Standard conversions
// =============================================================================

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Self::new(value)
    }
}

impl From<Boolean> for bool {
    fn from(value: Boolean) -> Self {
        value.get()
    }
}

impl From<Boolean> for u8 {
    fn from(value: Boolean) -> Self {
        value.canonical_byte()
    }
}

impl TryFrom<u8> for Boolean {
    type Error = BooleanError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value)
    }
}

impl TryFrom<&[u8]> for Boolean {
    type Error = BooleanError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_canonical_bytes(value)
    }
}

// =============================================================================
// Standard operators
// =============================================================================

impl Not for Boolean {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}

impl BitAnd for Boolean {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.and(rhs)
    }
}

impl BitOr for Boolean {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.or(rhs)
    }
}

impl BitXor for Boolean {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.xor(rhs)
    }
}

// =============================================================================
// Display
// =============================================================================

impl fmt::Display for Boolean {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::False => formatter.write_str("false"),
            Self::True => formatter.write_str("true"),
        }
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
    fn construction_from_bool() {
        assert_eq!(Boolean::new(false), Boolean::False);
        assert_eq!(Boolean::new(true), Boolean::True);
    }

    #[test]
    fn conversion_to_bool() {
        assert!(!Boolean::False.get());
        assert!(Boolean::True.get());

        assert!(!bool::from(Boolean::False));
        assert!(bool::from(Boolean::True));
    }

    #[test]
    fn predicates_are_correct() {
        assert!(Boolean::True.is_true());
        assert!(!Boolean::True.is_false());

        assert!(Boolean::False.is_false());
        assert!(!Boolean::False.is_true());
    }

    // -------------------------------------------------------------------------
    // NOT
    // -------------------------------------------------------------------------

    #[test]
    fn not_is_correct() {
        assert_eq!(Boolean::False.not(), Boolean::True);
        assert_eq!(Boolean::True.not(), Boolean::False);
    }

    #[test]
    fn not_operator_is_correct() {
        assert_eq!(!Boolean::False, Boolean::True);
        assert_eq!(!Boolean::True, Boolean::False);
    }

    // -------------------------------------------------------------------------
    // AND
    // -------------------------------------------------------------------------

    #[test]
    fn and_truth_table() {
        assert_eq!(
            Boolean::False.and(Boolean::False),
            Boolean::False
        );

        assert_eq!(
            Boolean::False.and(Boolean::True),
            Boolean::False
        );

        assert_eq!(
            Boolean::True.and(Boolean::False),
            Boolean::False
        );

        assert_eq!(
            Boolean::True.and(Boolean::True),
            Boolean::True
        );
    }

    #[test]
    fn bitand_operator_is_correct() {
        assert_eq!(
            Boolean::True & Boolean::True,
            Boolean::True
        );

        assert_eq!(
            Boolean::True & Boolean::False,
            Boolean::False
        );
    }

    // -------------------------------------------------------------------------
    // OR
    // -------------------------------------------------------------------------

    #[test]
    fn or_truth_table() {
        assert_eq!(
            Boolean::False.or(Boolean::False),
            Boolean::False
        );

        assert_eq!(
            Boolean::False.or(Boolean::True),
            Boolean::True
        );

        assert_eq!(
            Boolean::True.or(Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::True.or(Boolean::True),
            Boolean::True
        );
    }

    #[test]
    fn bitor_operator_is_correct() {
        assert_eq!(
            Boolean::False | Boolean::False,
            Boolean::False
        );

        assert_eq!(
            Boolean::False | Boolean::True,
            Boolean::True
        );
    }

    // -------------------------------------------------------------------------
    // XOR
    // -------------------------------------------------------------------------

    #[test]
    fn xor_truth_table() {
        assert_eq!(
            Boolean::False.xor(Boolean::False),
            Boolean::False
        );

        assert_eq!(
            Boolean::False.xor(Boolean::True),
            Boolean::True
        );

        assert_eq!(
            Boolean::True.xor(Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::True.xor(Boolean::True),
            Boolean::False
        );
    }

    #[test]
    fn bitxor_operator_is_correct() {
        assert_eq!(
            Boolean::False ^ Boolean::True,
            Boolean::True
        );

        assert_eq!(
            Boolean::True ^ Boolean::True,
            Boolean::False
        );
    }

    // -------------------------------------------------------------------------
    // Derived operations
    // -------------------------------------------------------------------------

    #[test]
    fn nand_is_correct() {
        assert_eq!(
            Boolean::False.nand(Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::False.nand(Boolean::True),
            Boolean::True
        );

        assert_eq!(
            Boolean::True.nand(Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::True.nand(Boolean::True),
            Boolean::False
        );
    }

    #[test]
    fn nor_is_correct() {
        assert_eq!(
            Boolean::False.nor(Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::False.nor(Boolean::True),
            Boolean::False
        );

        assert_eq!(
            Boolean::True.nor(Boolean::False),
            Boolean::False
        );

        assert_eq!(
            Boolean::True.nor(Boolean::True),
            Boolean::False
        );
    }

    #[test]
    fn xnor_is_correct() {
        assert_eq!(
            Boolean::False.xnor(Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::False.xnor(Boolean::True),
            Boolean::False
        );

        assert_eq!(
            Boolean::True.xnor(Boolean::False),
            Boolean::False
        );

        assert_eq!(
            Boolean::True.xnor(Boolean::True),
            Boolean::True
        );
    }

    #[test]
    fn implication_is_correct() {
        assert_eq!(
            Boolean::False.implies(Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::False.implies(Boolean::True),
            Boolean::True
        );

        assert_eq!(
            Boolean::True.implies(Boolean::False),
            Boolean::False
        );

        assert_eq!(
            Boolean::True.implies(Boolean::True),
            Boolean::True
        );
    }

    #[test]
    fn reverse_implication_is_correct() {
        assert_eq!(
            Boolean::False.implied_by(Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::False.implied_by(Boolean::True),
            Boolean::False
        );

        assert_eq!(
            Boolean::True.implied_by(Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::True.implied_by(Boolean::True),
            Boolean::True
        );
    }

    // -------------------------------------------------------------------------
    // Selection
    // -------------------------------------------------------------------------

    #[test]
    fn select_is_correct() {
        assert_eq!(
            Boolean::True.select(Boolean::True, Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::True.select(Boolean::False, Boolean::True),
            Boolean::False
        );

        assert_eq!(
            Boolean::False.select(Boolean::True, Boolean::False),
            Boolean::False
        );

        assert_eq!(
            Boolean::False.select(Boolean::False, Boolean::True),
            Boolean::True
        );
    }

    // -------------------------------------------------------------------------
    // Equality
    // -------------------------------------------------------------------------

    #[test]
    fn semantic_equality_is_correct() {
        assert_eq!(
            Boolean::False.equals(Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::False.equals(Boolean::True),
            Boolean::False
        );

        assert_eq!(
            Boolean::True.equals(Boolean::False),
            Boolean::False
        );

        assert_eq!(
            Boolean::True.equals(Boolean::True),
            Boolean::True
        );
    }

    #[test]
    fn semantic_inequality_is_correct() {
        assert_eq!(
            Boolean::False.not_equals(Boolean::False),
            Boolean::False
        );

        assert_eq!(
            Boolean::False.not_equals(Boolean::True),
            Boolean::True
        );

        assert_eq!(
            Boolean::True.not_equals(Boolean::False),
            Boolean::True
        );

        assert_eq!(
            Boolean::True.not_equals(Boolean::True),
            Boolean::False
        );
    }

    // -------------------------------------------------------------------------
    // Canonical serialization
    // -------------------------------------------------------------------------

    #[test]
    fn canonical_byte_is_stable() {
        assert_eq!(Boolean::False.canonical_byte(), 0);
        assert_eq!(Boolean::True.canonical_byte(), 1);
    }

    #[test]
    fn canonical_bytes_are_stable() {
        assert_eq!(Boolean::False.canonical_bytes(), [0]);
        assert_eq!(Boolean::True.canonical_bytes(), [1]);
    }

    #[test]
    fn canonical_byte_round_trip() {
        for value in [Boolean::False, Boolean::True] {
            let encoded = value.canonical_byte();
            let decoded = Boolean::from_canonical_byte(encoded);

            assert_eq!(decoded, Ok(value));
        }
    }

    #[test]
    fn canonical_bytes_round_trip() {
        for value in [Boolean::False, Boolean::True] {
            let encoded = value.canonical_bytes();
            let decoded = Boolean::from_canonical_bytes(&encoded);

            assert_eq!(decoded, Ok(value));
        }
    }

    #[test]
    fn invalid_canonical_byte_is_rejected() {
        for value in 2_u8..=u8::MAX {
            assert_eq!(
                Boolean::from_canonical_byte(value),
                Err(BooleanError::InvalidEncoding(value))
            );
        }
    }

    #[test]
    fn invalid_canonical_length_is_rejected() {
        assert_eq!(
            Boolean::from_canonical_bytes(&[]),
            Err(BooleanError::InvalidLength {
                expected: 1,
                actual: 0,
            })
        );

        assert_eq!(
            Boolean::from_canonical_bytes(&[0, 1]),
            Err(BooleanError::InvalidLength {
                expected: 1,
                actual: 2,
            })
        );
    }

    #[test]
    fn canonical_byte_validation_is_correct() {
        assert!(Boolean::is_valid_canonical_byte(0));
        assert!(Boolean::is_valid_canonical_byte(1));

        assert!(!Boolean::is_valid_canonical_byte(2));
        assert!(!Boolean::is_valid_canonical_byte(u8::MAX));
    }

    // -------------------------------------------------------------------------
    // Conversions
    // -------------------------------------------------------------------------

    #[test]
    fn u8_conversion_is_checked() {
        assert_eq!(
            Boolean::try_from(0_u8),
            Ok(Boolean::False)
        );

        assert_eq!(
            Boolean::try_from(1_u8),
            Ok(Boolean::True)
        );

        assert_eq!(
            Boolean::try_from(2_u8),
            Err(BooleanError::InvalidEncoding(2))
        );
    }

    #[test]
    fn byte_slice_conversion_is_checked() {
        assert_eq!(
            Boolean::try_from(&[0_u8][..]),
            Ok(Boolean::False)
        );

        assert_eq!(
            Boolean::try_from(&[1_u8][..]),
            Ok(Boolean::True)
        );

        assert_eq!(
            Boolean::try_from(&[2_u8][..]),
            Err(BooleanError::InvalidEncoding(2))
        );
    }

    #[test]
    fn u8_output_is_canonical() {
        assert_eq!(u8::from(Boolean::False), 0);
        assert_eq!(u8::from(Boolean::True), 1);
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    #[test]
    fn every_boolean_is_valid() {
        assert_eq!(Boolean::False.validate(), Ok(()));
        assert_eq!(Boolean::True.validate(), Ok(()));
    }

    // -------------------------------------------------------------------------
    // Display
    // -------------------------------------------------------------------------

    #[test]
    fn display_is_source_compatible() {
        assert_eq!(Boolean::False.to_string(), "false");
        assert_eq!(Boolean::True.to_string(), "true");
    }

    // -------------------------------------------------------------------------
    // Algebraic identities
    // -------------------------------------------------------------------------

    #[test]
    fn double_negation() {
        for value in [Boolean::False, Boolean::True] {
            assert_eq!(value.not().not(), value);
        }
    }

    #[test]
    fn xor_with_false_is_identity() {
        for value in [Boolean::False, Boolean::True] {
            assert_eq!(
                value.xor(Boolean::False),
                value
            );
        }
    }

    #[test]
    fn xor_with_true_is_negation() {
        for value in [Boolean::False, Boolean::True] {
            assert_eq!(
                value.xor(Boolean::True),
                value.not()
            );
        }
    }

    #[test]
    fn and_with_true_is_identity() {
        for value in [Boolean::False, Boolean::True] {
            assert_eq!(
                value.and(Boolean::True),
                value
            );
        }
    }

    #[test]
    fn and_with_false_is_zero() {
        for value in [Boolean::False, Boolean::True] {
            assert_eq!(
                value.and(Boolean::False),
                Boolean::False
            );
        }
    }

    #[test]
    fn or_with_false_is_identity() {
        for value in [Boolean::False, Boolean::True] {
            assert_eq!(
                value.or(Boolean::False),
                value
            );
        }
    }

    #[test]
    fn or_with_true_is_one() {
        for value in [Boolean::False, Boolean::True] {
            assert_eq!(
                value.or(Boolean::True),
                Boolean::True
            );
        }
    }

    #[test]
    fn nand_is_not_and() {
        for lhs in [Boolean::False, Boolean::True] {
            for rhs in [Boolean::False, Boolean::True] {
                assert_eq!(
                    lhs.nand(rhs),
                    lhs.and(rhs).not()
                );
            }
        }
    }

    #[test]
    fn nor_is_not_or() {
        for lhs in [Boolean::False, Boolean::True] {
            for rhs in [Boolean::False, Boolean::True] {
                assert_eq!(
                    lhs.nor(rhs),
                    lhs.or(rhs).not()
                );
            }
        }
    }

    #[test]
    fn xnor_is_not_xor() {
        for lhs in [Boolean::False, Boolean::True] {
            for rhs in [Boolean::False, Boolean::True] {
                assert_eq!(
                    lhs.xnor(rhs),
                    lhs.xor(rhs).not()
                );
            }
        }
    }
}