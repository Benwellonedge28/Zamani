
// src/classical/values/boolean.rs
//
// Contract status: Implementation proposed; integration and compiler
// verification are still required.
//
// Purpose:
//   Canonical, target-independent semantics for a single Boolean value.
//
// Owns:
//   Boolean, BooleanError, Boolean truth operations, and the canonical
//   one-byte encoding of a Boolean.
//
// Does not own:
//   Source tokens, AST nodes, Boolean type checking, expression trees,
//   classical storage identities, quantum measurement, runtime execution,
//   physical representation, or collections of Boolean values.
//
// Integration:
//   - classical::values::mod declares this module.
//   - classical::mod exposes the values module.
//   - quantum::ir::classical::boolean must re-export this implementation.
//   - The canonical ClassicalValue enum remains owned by its existing module.
//   - Frontend literal lowering maps the `true` / `false` AST literals
//     into this value; this file does not parse source text.
//
// Rust: 2021 edition; intended for Rust 1.97+.
// Safety: no unsafe Rust; no external dependencies.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use core::fmt;
use core::ops::{BitAnd, BitOr, BitXor, Not};

/// A Zamani Boolean value.
///
/// This represents logical truth, not a storage location or a hardware bit.
/// Its semantic domain is exactly `False` and `True`, independent of the
/// width or representation chosen by a target backend.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Boolean {
    /// Logical false. Its canonical serialized byte is `0`.
    #[default]
    False,

    /// Logical true. Its canonical serialized byte is `1`.
    True,
}

/// Failure while decoding the canonical Boolean representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BooleanError {
    /// A byte other than `0` or `1` was supplied.
    InvalidEncoding(u8),

    /// The encoded value did not contain exactly one byte.
    InvalidLength {
        /// Number of bytes required by this encoding.
        expected: usize,
        /// Number of bytes supplied by the caller.
        actual: usize,
    },
}

impl Boolean {
    /// Constructs a Boolean from Rust's primitive Boolean type.
    #[must_use]
    pub const fn new(value: bool) -> Self {
        if value {
            Self::True
        } else {
            Self::False
        }
    }

    /// Returns the equivalent Rust Boolean.
    ///
    /// This is a value conversion; it does not expose target storage.
    #[must_use]
    pub const fn get(self) -> bool {
        matches!(self, Self::True)
    }

    /// Returns whether this value is logically true.
    #[must_use]
    pub const fn is_true(self) -> bool {
        matches!(self, Self::True)
    }

    /// Returns whether this value is logically false.
    #[must_use]
    pub const fn is_false(self) -> bool {
        matches!(self, Self::False)
    }

    /// Logical negation.
    #[must_use]
    pub const fn not(self) -> Self {
        match self {
            Self::False => Self::True,
            Self::True => Self::False,
        }
    }

    /// Logical conjunction (AND).
    #[must_use]
    pub const fn and(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::True, Self::True) => Self::True,
            _ => Self::False,
        }
    }

    /// Logical disjunction (OR).
    #[must_use]
    pub const fn or(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::False, Self::False) => Self::False,
            _ => Self::True,
        }
    }

    /// Logical exclusive disjunction (XOR).
    #[must_use]
    pub const fn xor(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::False, Self::False) | (Self::True, Self::True) => Self::False,
            _ => Self::True,
        }
    }

    /// Logical NAND: NOT (self AND rhs).
    #[must_use]
    pub const fn nand(self, rhs: Self) -> Self {
        self.and(rhs).not()
    }

    /// Logical NOR: NOT (self OR rhs).
    #[must_use]
    pub const fn nor(self, rhs: Self) -> Self {
        self.or(rhs).not()
    }

    /// Logical equivalence (XNOR).
    #[must_use]
    pub const fn xnor(self, rhs: Self) -> Self {
        self.xor(rhs).not()
    }

    /// Logical implication (`self → rhs`).
    ///
    /// Implication is false only when `self` is true and `rhs` is false.
    #[must_use]
    pub const fn implies(self, rhs: Self) -> Self {
        self.not().or(rhs)
    }

    /// Reverse implication (`rhs → self`).
    #[must_use]
    pub const fn implied_by(self, rhs: Self) -> Self {
        rhs.implies(self)
    }

    /// Boolean conditional selection.
    ///
    /// Returns `when_true` when this value is true, otherwise `when_false`.
    #[must_use]
    pub const fn select<T: Copy>(self, when_true: T, when_false: T) -> T {
        match self {
            Self::True => when_true,
            Self::False => when_false,
        }
    }

    /// Returns the stable canonical byte: false = `0`, true = `1`.
    ///
    /// This is a serialization contract, not a promise about the in-memory
    /// enum layout or the representation used by any target.
    #[must_use]
    pub const fn canonical_byte(self) -> u8 {
        match self {
            Self::False => 0,
            Self::True => 1,
        }
    }

    /// Returns the canonical serialized representation.
    ///
    /// The returned array always contains exactly one byte.
    #[must_use]
    pub const fn canonical_bytes(self) -> [u8; 1] {
        [self.canonical_byte()]
    }

    /// Decodes the canonical one-byte representation.
    ///
    /// Invalid bytes and incorrect input lengths are rejected explicitly;
    /// they are never coerced to true or false.
    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, BooleanError> {
        if bytes.len() != 1 {
            return Err(BooleanError::InvalidLength {
                expected: 1,
                actual: bytes.len(),
            });
        }

        Self::try_from(bytes[0])
    }

    /// Returns the canonical lowercase source-style spelling.
    ///
    /// This is useful for diagnostics and display. It is not a parser and
    /// does not make source text the canonical binary representation.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::False => "false",
            Self::True => "true",
        }
    }
}

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

impl TryFrom<u8> for Boolean {
    type Error = BooleanError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::False),
            1 => Ok(Self::True),
            other => Err(BooleanError::InvalidEncoding(other)),
        }
    }
}

impl TryFrom<&[u8]> for Boolean {
    type Error = BooleanError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_canonical_bytes(bytes)
    }
}

impl Not for Boolean {
    type Output = Self;

    fn not(self) -> Self::Output {
        Boolean::not(self)
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

impl fmt::Display for Boolean {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for BooleanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEncoding(value) => write!(
                formatter,
                "invalid Boolean encoding {value}; expected byte 0 or 1"
            ),
            Self::InvalidLength { expected, actual } => write!(
                formatter,
                "invalid Boolean encoding length {actual}; expected {expected} byte"
            ),
        }
    }
}

impl std::error::Error for BooleanError {}

#[cfg(test)]
mod tests {
    use super::{Boolean, BooleanError};

    #[test]
    fn primitive_conversion_is_bidirectional() {
        assert_eq!(Boolean::from(false), Boolean::False);
        assert_eq!(Boolean::from(true), Boolean::True);
        assert!(!bool::from(Boolean::False));
        assert!(bool::from(Boolean::True));
    }

    #[test]
    fn boolean_has_exactly_two_values() {
        assert!(Boolean::False.is_false());
        assert!(!Boolean::False.is_true());
        assert!(Boolean::True.is_true());
        assert!(!Boolean::True.is_false());
    }

    #[test]
    fn truth_tables_are_deterministic() {
        let cases = [
            (Boolean::False, Boolean::False),
            (Boolean::False, Boolean::True),
            (Boolean::True, Boolean::False),
            (Boolean::True, Boolean::True),
        ];

        for (left, right) in cases {
            let a = left.get();
            let b = right.get();

            assert_eq!(left.and(right).get(), a && b);
            assert_eq!(left.or(right).get(), a || b);
            assert_eq!(left.xor(right).get(), a ^ b);
            assert_eq!(left.nand(right).get(), !(a && b));
            assert_eq!(left.nor(right).get(), !(a || b));
            assert_eq!(left.xnor(right).get(), a == b);
            assert_eq!(left.implies(right).get(), !a || b);
            assert_eq!(left.implied_by(right).get(), !b || a);
        }
    }

    #[test]
    fn canonical_encoding_is_exact_and_stable() {
        assert_eq!(Boolean::False.canonical_byte(), 0);
        assert_eq!(Boolean::True.canonical_byte(), 1);
        assert_eq!(Boolean::False.canonical_bytes(), [0]);
        assert_eq!(Boolean::True.canonical_bytes(), [1]);

        assert_eq!(
            Boolean::from_canonical_bytes(&[0]),
            Ok(Boolean::False)
        );
        assert_eq!(
            Boolean::from_canonical_bytes(&[1]),
            Ok(Boolean::True)
        );
    }

    #[test]
    fn invalid_encodings_are_rejected_without_coercion() {
        assert_eq!(
            Boolean::try_from(2_u8),
            Err(BooleanError::InvalidEncoding(2))
        );
        assert_eq!(
            Boolean::try_from(u8::MAX),
            Err(BooleanError::InvalidEncoding(u8::MAX))
        );
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
    fn operator_traits_match_named_operations() {
        assert_eq!(!Boolean::True, Boolean::False);
        assert_eq!(Boolean::True & Boolean::False, Boolean::False);
        assert_eq!(Boolean::True | Boolean::False, Boolean::True);
        assert_eq!(Boolean::True ^ Boolean::True, Boolean::False);
    }

    #[test]
    fn selection_uses_only_the_condition() {
        assert_eq!(
            Boolean::True.select(17_u64, 29_u64),
            17
        );
        assert_eq!(
            Boolean::False.select(17_u64, 29_u64),
            29
        );
    }

    #[test]
    fn display_uses_canonical_lowercase_spellings() {
        assert_eq!(Boolean::False.to_string(), "false");
        assert_eq!(Boolean::True.to_string(), "true");
    }

    #[test]
    fn default_is_false() {
        assert_eq!(Boolean::default(), Boolean::False);
    }
}
