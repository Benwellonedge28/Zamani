#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Four-State Logic Values
//!
//! Path: `src/hdl/values/four_state.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent and operate on four-state digital logic values:
//!
//! - `0`: known logical zero.
//! - `1`: known logical one.
//! - `X`: unknown logic value.
//! - `Z`: high-impedance logic value.
//!
//! Provide target-independent scalar and dynamically sized vector values,
//! explicit conversions, four-state bitwise operations, reduction operations,
//! conditional merging, and multi-driver resolution.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars.
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/types/logic.rs`
//! - `src/hdl/values/mod.rs`, when introduced.
//!
//! Grammar defines source syntax. The existing semantic subsystem owns
//! canonical HDL widths and value types. `types/logic.rs` owns logic-type
//! semantics. This module owns the concrete four-state value operations.
//!
//! ## Owns
//!
//! - Four-state scalar values and their logic-operation truth tables.
//! - Dynamically sized vectors of four-state values.
//! - Explicit MSB-first textual parsing and formatting.
//! - Checked bit access and equal-width vector operations.
//! - Unknown-aware reductions and equality.
//! - Conditional merging when the selector is unknown or high impedance.
//! - Explicit resolution of multiple drivers.
//! - Explicit conversion failures for unknown and high-impedance values.
//!
//! ## Does not own
//!
//! - Source parsing, AST construction, or name resolution.
//! - Canonical HDL types, signedness, or symbolic width expressions.
//! - General arithmetic, expression parsing, or expression evaluation.
//! - Event scheduling, clocking, sequential execution, or simulation time.
//! - Synthesis interpretation or target-specific optimization.
//! - Physical wires, electrical modeling beyond the defined resolution
//!   abstraction, device allocation, or target capabilities.
//! - HDLIR structures, builders, verification, passes, or lowering.
//!
//! ## Representation and scalability
//!
//! Vector storage is dynamically allocated. There is no language-level
//! maximum vector width in this module. A vector's realizable size is limited
//! by available memory, allocation success, and the surrounding application's
//! operational budgets.
//!
//! This module does not claim that allocations of arbitrary size will
//! succeed. Allocation failure may be handled by the process's allocator
//! policy; callers requiring recoverable allocation failure must enforce
//! budgets or use an allocation strategy that provides that guarantee.
//!
//! Bit index zero is the least-significant bit (LSB). Textual vectors are
//! parsed and formatted most-significant bit (MSB) first. These conventions
//! are explicit and independent of host endianness.
//!
//! ## Semantic rules
//!
//! `X` and `Z` are never implicitly converted to `false` or `true`.
//! Ordinary bitwise operations treat `Z` as an indeterminate logic input,
//! following four-state logic semantics. Multi-driver resolution is a
//! separate operation and preserves high impedance where appropriate.
//!
//! These operations do not define every SystemVerilog expression rule.
//! In particular, logical operators, case equality, arithmetic, strengths,
//! and context-dependent sizing belong to their respective semantic
//! operations and must not be inferred from these bitwise operations.
//!
//! ## Integration
//!
//! - `src/hdl/values/mod.rs` should declare `pub mod four_state;` and expose
//!   the public types deliberately.
//! - `src/hdl/types/logic.rs` remains the owner of `LogicStateModel` and
//!   `LogicType`. A future checked adapter may verify that a value's width
//!   and states conform to a concrete `LogicType`.
//! - HDL expression evaluation should call these operations only when the
//!   source operator has matching four-state bitwise semantics.
//! - Simulation may reuse the scalar and vector truth tables, but owns event
//!   scheduling, assignment timing, and process execution.
//! - Verilog/SystemVerilog import and export adapters may use the textual
//!   representation, while preserving their own language-specific sizing,
//!   signedness, and context rules.
//!
//! No dependency on a future module is required to compile this file.
//!
//! ## Completion criteria
//!
//! The tests cover truth tables, parsing, formatting, width mismatches,
//! indexing, unknown propagation, conditional merging, resolution, and
//! explicit Boolean conversion failures. HDL conformance and integration
//! suites must additionally check agreement with the normative language
//! semantics and supported imported-HDL dialects.

use std::fmt;

/// One digital logic state.
///
/// This is a value, not a type declaration. Whether a type or language
/// context permits a particular state is checked by the owning semantic
/// layer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LogicState {
    /// Known logical zero.
    Zero,

    /// Known logical one.
    One,

    /// Unknown or indeterminate logic value.
    Unknown,

    /// High-impedance logic value.
    HighImpedance,
}

impl LogicState {
    /// Return the character used to represent this state in HDL bit-vector
    /// text.
    #[must_use]
    pub const fn as_char(self) -> char {
        match self {
            Self::Zero => '0',
            Self::One => '1',
            Self::Unknown => 'x',
            Self::HighImpedance => 'z',
        }
    }

    /// Parse one four-state character.
    ///
    /// The accepted characters are `0`, `1`, `x`, `X`, `z`, and `Z`.
    /// Whitespace and other characters are rejected.
    pub fn from_char(value: char) -> Result<Self, FourStateError> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            'x' | 'X' => Ok(Self::Unknown),
            'z' | 'Z' => Ok(Self::HighImpedance),
            other => Err(FourStateError::InvalidCharacter {
                character: other,
                byte_offset: 0,
            }),
        }
    }

    /// Construct a known state from a Boolean value.
    #[must_use]
    pub const fn from_bool(value: bool) -> Self {
        if value {
            Self::One
        } else {
            Self::Zero
        }
    }

    /// Convert this state to a Boolean only when it is known.
    ///
    /// Unknown and high-impedance states return explicit errors; neither
    /// state is silently treated as zero or one.
    pub fn to_bool(self) -> Result<bool, FourStateError> {
        match self {
            Self::Zero => Ok(false),
            Self::One => Ok(true),
            Self::Unknown => Err(FourStateError::UnknownConversion),
            Self::HighImpedance => Err(FourStateError::HighImpedanceConversion),
        }
    }

    /// Return whether this state is known.
    #[must_use]
    pub const fn is_known(self) -> bool {
        matches!(self, Self::Zero | Self::One)
    }

    /// Return whether this state is unknown.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Return whether this state is high impedance.
    #[must_use]
    pub const fn is_high_impedance(self) -> bool {
        matches!(self, Self::HighImpedance)
    }

    /// Four-state bitwise NOT.
    ///
    /// `X` and `Z` both produce `X`: logical inversion cannot determine a
    /// known result from either indeterminate or disconnected input.
    #[must_use]
    pub const fn bit_not(self) -> Self {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Zero,
            Self::Unknown | Self::HighImpedance => Self::Unknown,
        }
    }

    /// Four-state bitwise AND.
    ///
    /// A known zero determines the result even if the other input is
    /// unknown. `Z` is treated as indeterminate for this operation.
    #[must_use]
    pub const fn bit_and(self, other: Self) -> Self {
        use LogicState::{HighImpedance as Z, One, Unknown as X, Zero};

        match (self, other) {
            (Zero, _) | (_, Zero) => Zero,
            (One, One) => One,
            (One, X | Z) | (X | Z, One) | (X | Z, X | Z) => X,
        }
    }

    /// Four-state bitwise OR.
    ///
    /// A known one determines the result even if the other input is
    /// unknown. `Z` is treated as indeterminate for this operation.
    #[must_use]
    pub const fn bit_or(self, other: Self) -> Self {
        use LogicState::{HighImpedance as Z, One, Unknown as X, Zero};

        match (self, other) {
            (One, _) | (_, One) => One,
            (Zero, Zero) => Zero,
            (Zero, X | Z) | (X | Z, Zero) | (X | Z, X | Z) => X,
        }
    }

    /// Four-state bitwise XOR.
    ///
    /// XOR is known only when both operands are known.
    #[must_use]
    pub const fn bit_xor(self, other: Self) -> Self {
        match (self, other) {
            (Self::Zero, Self::Zero) | (Self::One, Self::One) => Self::Zero,
            (Self::Zero, Self::One) | (Self::One, Self::Zero) => Self::One,
            _ => Self::Unknown,
        }
    }

    /// Resolve several drivers of one logic value.
    ///
    /// This is a simple four-state, equal-strength resolution abstraction:
    ///
    /// - No drivers, or only high-impedance drivers, resolve to `Z`.
    /// - All active drivers agreeing on zero resolve to `0`.
    /// - All active drivers agreeing on one resolve to `1`.
    /// - Conflicting known drivers resolve to `X`.
    /// - Any active unknown driver makes the result `X`.
    ///
    /// Electrical drive strengths and technology-specific net resolution
    /// are outside this method's contract. A dialect requiring such behavior
    /// must use its own explicitly defined resolution model.
    #[must_use]
    pub fn resolve_drivers(
        drivers: impl IntoIterator<Item = Self>,
    ) -> Self {
        let mut resolved: Option<Self> = None;

        for driver in drivers {
            match driver {
                Self::HighImpedance => {}
                Self::Unknown => return Self::Unknown,
                Self::Zero | Self::One => match resolved {
                    None => resolved = Some(driver),
                    Some(current) if current == driver => {}
                    Some(_) => return Self::Unknown,
                },
            }
        }

        resolved.unwrap_or(Self::HighImpedance)
    }
}

impl fmt::Display for LogicState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_char().to_string())
    }
}

/// Errors produced by four-state value construction and operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FourStateError {
    /// A textual value contains a character outside the supported alphabet.
    InvalidCharacter {
        /// The offending character.
        character: char,
        /// UTF-8 byte offset of the character in the original text.
        byte_offset: usize,
    },

    /// The supplied textual vector contains no logic digits.
    EmptyVector,

    /// A vector operation requires equal widths.
    WidthMismatch {
        /// Width expected by the operation.
        expected: usize,
        /// Width actually provided.
        actual: usize,
    },

    /// A requested bit index is outside the vector.
    IndexOutOfBounds {
        /// Requested LSB-based index.
        index: usize,
        /// Number of bits in the vector.
        width: usize,
    },

    /// A multi-bit vector cannot be converted to a scalar Boolean.
    NotScalar {
        /// Actual vector width.
        width: usize,
    },

    /// An unknown value cannot be converted to a Boolean.
    UnknownConversion,

    /// A high-impedance value cannot be converted to a Boolean.
    HighImpedanceConversion,
}

impl fmt::Display for FourStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter {
                character,
                byte_offset,
            } => write!(
                f,
                "invalid four-state logic character {character:?} at byte \
                 offset {byte_offset}"
            ),
            Self::EmptyVector => {
                f.write_str("four-state vector must contain at least one bit")
            }
            Self::WidthMismatch { expected, actual } => write!(
                f,
                "four-state vector width mismatch: expected {expected} bits, \
                 found {actual} bits"
            ),
            Self::IndexOutOfBounds { index, width } => write!(
                f,
                "four-state bit index {index} is outside vector width {width}"
            ),
            Self::NotScalar { width } => write!(
                f,
                "cannot convert a {width}-bit vector to a scalar Boolean"
            ),
            Self::UnknownConversion => {
                f.write_str("cannot convert unknown logic value to Boolean")
            }
            Self::HighImpedanceConversion => {
                f.write_str("cannot convert high-impedance logic value to Boolean")
            }
        }
    }
}

impl std::error::Error for FourStateError {}

/// A dynamically sized four-state bit vector.
///
/// Internally, bits are stored least-significant bit first. Public textual
/// conversion uses most-significant bit first. The vector is non-empty:
/// zero-width hardware values must be modeled by a separate explicit
/// semantic construct if the language specification permits them.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FourStateVector {
    bits_lsb_first: Vec<LogicState>,
}

impl FourStateVector {
    /// Construct a vector from bits in least-significant-bit-first order.
    ///
    /// Empty vectors are rejected so that a constructed value always has a
    /// positive width.
    pub fn from_lsb_first(
        bits: impl Into<Vec<LogicState>>,
    ) -> Result<Self, FourStateError> {
        let bits_lsb_first = bits.into();

        if bits_lsb_first.is_empty() {
            return Err(FourStateError::EmptyVector);
        }

        Ok(Self { bits_lsb_first })
    }

    /// Parse a vector from most-significant-bit-first text.
    ///
    /// Accepted digits are `0`, `1`, `x`, `X`, `z`, and `Z`. Underscores
    /// are accepted as visual separators and ignored. Whitespace is not
    /// ignored. At least one logic digit is required.
    ///
    /// Example: `"10_xZ"` represents the bits `1`, `0`, `X`, `Z` from MSB
    /// to LSB.
    pub fn from_msb_str(text: &str) -> Result<Self, FourStateError> {
        let mut bits_lsb_first = Vec::new();
        let mut saw_logic_digit = false;

        for (byte_offset, character) in text.char_indices() {
            if character == '_' {
                continue;
            }

            let state = LogicState::from_char(character).map_err(|_| {
                FourStateError::InvalidCharacter {
                    character,
                    byte_offset,
                }
            })?;

            saw_logic_digit = true;
            bits_lsb_first.push(state);
        }

        if !saw_logic_digit {
            return Err(FourStateError::EmptyVector);
        }

        bits_lsb_first.reverse();

        Ok(Self { bits_lsb_first })
    }

    /// Construct a vector filled with known zeros.
    pub fn zeros(width: usize) -> Result<Self, FourStateError> {
        Self::filled(width, LogicState::Zero)
    }

    /// Construct a vector filled with known ones.
    pub fn ones(width: usize) -> Result<Self, FourStateError> {
        Self::filled(width, LogicState::One)
    }

    /// Construct a vector filled with unknown values.
    pub fn unknowns(width: usize) -> Result<Self, FourStateError> {
        Self::filled(width, LogicState::Unknown)
    }

    /// Construct a vector filled with high-impedance values.
    pub fn high_impedances(width: usize) -> Result<Self, FourStateError> {
        Self::filled(width, LogicState::HighImpedance)
    }

    /// Construct a vector filled with one state.
    ///
    /// A width of zero is rejected. Allocation behavior follows the
    /// standard allocator; this method does not impose a separate capacity
    /// limit.
    pub fn filled(
        width: usize,
        state: LogicState,
    ) -> Result<Self, FourStateError> {
        if width == 0 {
            return Err(FourStateError::EmptyVector);
        }

        Ok(Self {
            bits_lsb_first: vec![state; width],
        })
    }

    /// Return the vector width in bits.
    #[must_use]
    pub fn width(&self) -> usize {
        self.bits_lsb_first.len()
    }

    /// Return whether every bit is known zero or known one.
    #[must_use]
    pub fn is_fully_known(&self) -> bool {
        self.bits_lsb_first.iter().all(|state| state.is_known())
    }

    /// Return a bit by its least-significant-bit-based index.
    #[must_use]
    pub fn get_lsb(&self, index: usize) -> Option<LogicState> {
        self.bits_lsb_first.get(index).copied()
    }

    /// Set a bit by its least-significant-bit-based index.
    ///
    /// Returns an error rather than indexing outside the vector.
    pub fn set_lsb(
        &mut self,
        index: usize,
        state: LogicState,
    ) -> Result<(), FourStateError> {
        let width = self.width();
        let bit = self.bits_lsb_first.get_mut(index).ok_or(
            FourStateError::IndexOutOfBounds { index, width },
        )?;

        *bit = state;
        Ok(())
    }

    /// Return a copy of the bits in least-significant-bit-first order.
    #[must_use]
    pub fn as_lsb_first(&self) -> &[LogicState] {
        &self.bits_lsb_first
    }

    /// Format the vector most-significant bit first.
    #[must_use]
    pub fn to_msb_string(&self) -> String {
        self.bits_lsb_first
            .iter()
            .rev()
            .map(|state| state.as_char())
            .collect()
    }

    /// Apply four-state bitwise NOT to every bit.
    #[must_use]
    pub fn bit_not(&self) -> Self {
        Self {
            bits_lsb_first: self
                .bits_lsb_first
                .iter()
                .map(|state| state.bit_not())
                .collect(),
        }
    }

    /// Apply four-state bitwise AND to two equal-width vectors.
    pub fn bit_and(&self, other: &Self) -> Result<Self, FourStateError> {
        self.zip_map(other, LogicState::bit_and)
    }

    /// Apply four-state bitwise OR to two equal-width vectors.
    pub fn bit_or(&self, other: &Self) -> Result<Self, FourStateError> {
        self.zip_map(other, LogicState::bit_or)
    }

    /// Apply four-state bitwise XOR to two equal-width vectors.
    pub fn bit_xor(&self, other: &Self) -> Result<Self, FourStateError> {
        self.zip_map(other, LogicState::bit_xor)
    }

    /// Reduce all bits using four-state AND semantics.
    ///
    /// A known zero determines the result. Otherwise, any unknown or
    /// high-impedance bit makes the result unknown; all ones produce one.
    #[must_use]
    pub fn reduce_and(&self) -> LogicState {
        self.bits_lsb_first
            .iter()
            .copied()
            .fold(LogicState::One, LogicState::bit_and)
    }

    /// Reduce all bits using four-state OR semantics.
    ///
    /// A known one determines the result. Otherwise, any unknown or
    /// high-impedance bit makes the result unknown; all zeros produce zero.
    #[must_use]
    pub fn reduce_or(&self) -> LogicState {
        self.bits_lsb_first
            .iter()
            .copied()
            .fold(LogicState::Zero, LogicState::bit_or)
    }

    /// Reduce all bits using four-state XOR semantics.
    #[must_use]
    pub fn reduce_xor(&self) -> LogicState {
        self.bits_lsb_first
            .iter()
            .copied()
            .fold(LogicState::Zero, LogicState::bit_xor)
    }

    /// Compare two vectors using four-state logical equality.
    ///
    /// A known mismatch makes the result zero. If there is no known
    /// mismatch but at least one compared bit is unknown or high impedance,
    /// the result is unknown. Fully known equal vectors produce one.
    ///
    /// This is not case equality. Use [`Self::case_eq`] when the caller
    /// requires exact state-by-state comparison including `X` and `Z`.
    pub fn logical_eq(&self, other: &Self) -> Result<LogicState, FourStateError> {
        self.require_equal_width(other)?;

        let mut saw_indeterminate = false;

        for (left, right) in self
            .bits_lsb_first
            .iter()
            .zip(other.bits_lsb_first.iter())
        {
            match (*left, *right) {
                (LogicState::Zero, LogicState::One)
                | (LogicState::One, LogicState::Zero) => {
                    return Ok(LogicState::Zero);
                }
                (a, b) if a.is_known() && b.is_known() => {}
                _ => saw_indeterminate = true,
            }
        }

        if saw_indeterminate {
            Ok(LogicState::Unknown)
        } else {
            Ok(LogicState::One)
        }
    }

    /// Compare vectors exactly, including their `X` and `Z` states.
    ///
    /// This operation returns a Rust Boolean because it compares the
    /// represented values exactly; it does not model HDL logical equality.
    #[must_use]
    pub fn case_eq(&self, other: &Self) -> bool {
        self.bits_lsb_first == other.bits_lsb_first
    }

    /// Select between two equal-width vectors using a four-state selector.
    ///
    /// - Selector zero returns `when_zero`.
    /// - Selector one returns `when_one`.
    /// - Selector `X` or `Z` merges corresponding bits: equal states are
    ///   preserved, while differing states become `X`.
    ///
    /// The caller remains responsible for using this operation only where
    /// the source language's conditional operator has these semantics.
    pub fn conditional_merge(
        selector: LogicState,
        when_zero: &Self,
        when_one: &Self,
    ) -> Result<Self, FourStateError> {
        when_zero.require_equal_width(when_one)?;

        match selector {
            LogicState::Zero => Ok(when_zero.clone()),
            LogicState::One => Ok(when_one.clone()),
            LogicState::Unknown | LogicState::HighImpedance => {
                let bits_lsb_first = when_zero
                    .bits_lsb_first
                    .iter()
                    .zip(when_one.bits_lsb_first.iter())
                    .map(|(left, right)| {
                        if left == right {
                            *left
                        } else {
                            LogicState::Unknown
                        }
                    })
                    .collect();

                Ok(Self { bits_lsb_first })
            }
        }
    }

    /// Resolve one bit from each of several drivers.
    ///
    /// An empty driver collection resolves to high impedance. This method
    /// is useful when a caller already has drivers grouped per bit.
    #[must_use]
    pub fn resolve_bit_drivers(
        drivers: impl IntoIterator<Item = LogicState>,
    ) -> LogicState {
        LogicState::resolve_drivers(drivers)
    }

    /// Resolve equal-width vectors representing multiple drivers.
    ///
    /// An empty driver collection is rejected because it does not carry a
    /// width. For each bit position, the driver's states are resolved using
    /// [`LogicState::resolve_drivers`].
    pub fn resolve_vectors<'a>(
        drivers: impl IntoIterator<Item = &'a Self>,
    ) -> Result<Self, FourStateError> {
        let mut drivers = drivers.into_iter();

        let Some(first) = drivers.next() else {
            return Err(FourStateError::EmptyVector);
        };

        let width = first.width();

        for driver in drivers.clone() {
            if driver.width() != width {
                return Err(FourStateError::WidthMismatch {
                    expected: width,
                    actual: driver.width(),
                });
            }
        }

        let drivers: Vec<&Self> = drivers.collect();

        let bits_lsb_first = (0..width)
            .map(|index| {
                LogicState::resolve_drivers(
                    std::iter::once(first.bits_lsb_first[index]).chain(
                        drivers
                            .iter()
                            .map(|driver| driver.bits_lsb_first[index]),
                    ),
                )
            })
            .collect();

        Ok(Self { bits_lsb_first })
    }

    /// Convert a one-bit vector to a Boolean.
    ///
    /// Multi-bit vectors, unknown values, and high-impedance values are
    /// rejected explicitly.
    pub fn to_bool(&self) -> Result<bool, FourStateError> {
        if self.width() != 1 {
            return Err(FourStateError::NotScalar { width: self.width() });
        }

        self.bits_lsb_first[0].to_bool()
    }

    fn require_equal_width(&self, other: &Self) -> Result<(), FourStateError> {
        if self.width() == other.width() {
            Ok(())
        } else {
            Err(FourStateError::WidthMismatch {
                expected: self.width(),
                actual: other.width(),
            })
        }
    }

    fn zip_map(
        &self,
        other: &Self,
        operation: fn(LogicState, LogicState) -> LogicState,
    ) -> Result<Self, FourStateError> {
        self.require_equal_width(other)?;

        let bits_lsb_first = self
            .bits_lsb_first
            .iter()
            .copied()
            .zip(other.bits_lsb_first.iter().copied())
            .map(|(left, right)| operation(left, right))
            .collect();

        Ok(Self { bits_lsb_first })
    }
}

impl fmt::Display for FourStateVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_msb_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_not_truth_table() {
        assert_eq!(LogicState::Zero.bit_not(), LogicState::One);
        assert_eq!(LogicState::One.bit_not(), LogicState::Zero);
        assert_eq!(LogicState::Unknown.bit_not(), LogicState::Unknown);
        assert_eq!(
            LogicState::HighImpedance.bit_not(),
            LogicState::Unknown
        );
    }

    #[test]
    fn scalar_and_truth_table() {
        use LogicState::{HighImpedance as Z, One, Unknown as X, Zero};

        assert_eq!(Zero.bit_and(Zero), Zero);
        assert_eq!(Zero.bit_and(One), Zero);
        assert_eq!(One.bit_and(One), One);
        assert_eq!(Zero.bit_and(X), Zero);
        assert_eq!(One.bit_and(X), X);
        assert_eq!(One.bit_and(Z), X);
        assert_eq!(X.bit_and(Z), X);
    }

    #[test]
    fn scalar_or_truth_table() {
        use LogicState::{HighImpedance as Z, One, Unknown as X, Zero};

        assert_eq!(One.bit_or(Zero), One);
        assert_eq!(One.bit_or(One), One);
        assert_eq!(Zero.bit_or(Zero), Zero);
        assert_eq!(One.bit_or(X), One);
        assert_eq!(Zero.bit_or(X), X);
        assert_eq!(Zero.bit_or(Z), X);
    }

    #[test]
    fn scalar_xor_propagates_indeterminate_states() {
        use LogicState::{HighImpedance as Z, One, Unknown as X, Zero};

        assert_eq!(Zero.bit_xor(Zero), Zero);
        assert_eq!(Zero.bit_xor(One), One);
        assert_eq!(One.bit_xor(One), Zero);
        assert_eq!(One.bit_xor(X), X);
        assert_eq!(Zero.bit_xor(Z), X);
    }

    #[test]
    fn scalar_boolean_conversion_is_explicit() {
        assert_eq!(LogicState::Zero.to_bool(), Ok(false));
        assert_eq!(LogicState::One.to_bool(), Ok(true));
        assert_eq!(
            LogicState::Unknown.to_bool(),
            Err(FourStateError::UnknownConversion)
        );
        assert_eq!(
            LogicState::HighImpedance.to_bool(),
            Err(FourStateError::HighImpedanceConversion)
        );
    }

    #[test]
    fn parsing_and_formatting_use_msb_first_text() {
        let value = FourStateVector::from_msb_str("10_xZ")
            .expect("valid four-state vector");

        assert_eq!(value.width(), 4);
        assert_eq!(value.to_msb_string(), "10xz");
        assert_eq!(value.get_lsb(0), Some(LogicState::HighImpedance));
        assert_eq!(value.get_lsb(1), Some(LogicState::Unknown));
        assert_eq!(value.get_lsb(2), Some(LogicState::Zero));
        assert_eq!(value.get_lsb(3), Some(LogicState::One));
    }

    #[test]
    fn invalid_text_reports_byte_offset() {
        assert_eq!(
            FourStateVector::from_msb_str("10q1"),
            Err(FourStateError::InvalidCharacter {
                character: 'q',
                byte_offset: 2,
            })
        );
    }

    #[test]
    fn empty_vector_text_is_rejected() {
        assert_eq!(
            FourStateVector::from_msb_str("___"),
            Err(FourStateError::EmptyVector)
        );

        assert_eq!(
            FourStateVector::from_lsb_first(Vec::new()),
            Err(FourStateError::EmptyVector)
        );
    }

    #[test]
    fn vector_bitwise_operations_preserve_width_and_unknowns() {
        let left =
            FourStateVector::from_msb_str("10x0").expect("valid vector");
        let right =
            FourStateVector::from_msb_str("01x1").expect("valid vector");

        assert_eq!(
            left.bit_and(&right)
                .expect("equal widths")
                .to_msb_string(),
            "00x0"
        );
        assert_eq!(
            left.bit_or(&right)
                .expect("equal widths")
                .to_msb_string(),
            "11xx"
        );
        assert_eq!(
            left.bit_xor(&right)
                .expect("equal widths")
                .to_msb_string(),
            "11xx"
        );
        assert_eq!(left.bit_not().to_msb_string(), "01x1");
    }

    #[test]
    fn mismatched_widths_are_rejected() {
        let left = FourStateVector::from_msb_str("10").expect("valid vector");
        let right = FourStateVector::from_msb_str("101").expect("valid vector");

        assert_eq!(
            left.bit_and(&right),
            Err(FourStateError::WidthMismatch {
                expected: 2,
                actual: 3,
            })
        );
    }

    #[test]
    fn checked_bit_access_rejects_out_of_bounds() {
        let mut value =
            FourStateVector::from_msb_str("101").expect("valid vector");

        assert_eq!(value.get_lsb(3), None);
        assert_eq!(
            value.set_lsb(3, LogicState::One),
            Err(FourStateError::IndexOutOfBounds { index: 3, width: 3 })
        );

        value
            .set_lsb(0, LogicState::Unknown)
            .expect("index zero is valid");

        assert_eq!(value.to_msb_string(), "10x");
    }

    #[test]
    fn reductions_follow_four_state_truth_tables() {
        assert_eq!(
            FourStateVector::from_msb_str("10x0")
                .expect("valid vector")
                .reduce_and(),
            LogicState::Zero
        );

        assert_eq!(
            FourStateVector::from_msb_str("11x")
                .expect("valid vector")
                .reduce_and(),
            LogicState::Unknown
        );

        assert_eq!(
            FourStateVector::from_msb_str("01x1")
                .expect("valid vector")
                .reduce_or(),
            LogicState::One
        );

        assert_eq!(
            FourStateVector::from_msb_str("00z")
                .expect("valid vector")
                .reduce_or(),
            LogicState::Unknown
        );
    }

    #[test]
    fn logical_equality_propagates_unknown_unless_mismatch_is_known() {
        let left = FourStateVector::from_msb_str("10x").expect("valid vector");
        let same = FourStateVector::from_msb_str("10x").expect("valid vector");
        let different =
            FourStateVector::from_msb_str("11x").expect("valid vector");

        assert_eq!(
            left.logical_eq(&same),
            Ok(LogicState::Unknown)
        );
        assert_eq!(
            left.logical_eq(&different),
            Ok(LogicState::Zero)
        );

        let known_left =
            FourStateVector::from_msb_str("101").expect("valid vector");
        let known_right =
            FourStateVector::from_msb_str("101").expect("valid vector");

        assert_eq!(
            known_left.logical_eq(&known_right),
            Ok(LogicState::One)
        );
    }

    #[test]
    fn case_equality_compares_unknown_states_exactly() {
        let left = FourStateVector::from_msb_str("1xz")
            .expect("valid vector");
        let same =
            FourStateVector::from_msb_str("1XZ").expect("valid vector");
        let different =
            FourStateVector::from_msb_str("1xx").expect("valid vector");

        assert!(left.case_eq(&same));
        assert!(!left.case_eq(&different));
    }

    #[test]
    fn unknown_selector_merges_equal_bits_and_marks_differences_unknown() {
        let when_zero =
            FourStateVector::from_msb_str("10z0").expect("valid vector");
        let when_one =
            FourStateVector::from_msb_str("11z1").expect("valid vector");

        let merged = FourStateVector::conditional_merge(
            LogicState::Unknown,
            &when_zero,
            &when_one,
        )
        .expect("equal widths");

        assert_eq!(merged.to_msb_string(), "1xzX".to_lowercase());
    }

    #[test]
    fn known_selector_selects_the_corresponding_vector() {
        let when_zero =
            FourStateVector::from_msb_str("10").expect("valid vector");
        let when_one =
            FourStateVector::from_msb_str("01").expect("valid vector");

        assert_eq!(
            FourStateVector::conditional_merge(
                LogicState::Zero,
                &when_zero,
                &when_one,
            )
            .expect("equal widths"),
            when_zero
        );

        assert_eq!(
            FourStateVector::conditional_merge(
                LogicState::One,
                &when_zero,
                &when_one,
            )
            .expect("equal widths"),
            when_one
        );
    }

    #[test]
    fn driver_resolution_preserves_z_and_detects_conflicts() {
        use LogicState::{HighImpedance as Z, One, Unknown as X, Zero};

        assert_eq!(LogicState::resolve_drivers([]), Z);
        assert_eq!(LogicState::resolve_drivers([Z, Z]), Z);
        assert_eq!(LogicState::resolve_drivers([Z, Zero, Zero]), Zero);
        assert_eq!(LogicState::resolve_drivers([Z, One, One]), One);
        assert_eq!(LogicState::resolve_drivers([Zero, One]), X);
        assert_eq!(LogicState::resolve_drivers([One, X]), X);
    }

    #[test]
    fn vector_driver_resolution_is_bitwise() {
        let first =
            FourStateVector::from_msb_str("10z").expect("valid vector");
        let second =
            FourStateVector::from_msb_str("1z0").expect("valid vector");

        let resolved =
            FourStateVector::resolve_vectors([&first, &second])
                .expect("equal widths");

        assert_eq!(resolved.to_msb_string(), "1x0");
    }

    #[test]
    fn vector_driver_resolution_rejects_mismatched_widths() {
        let first =
            FourStateVector::from_msb_str("10").expect("valid vector");
        let second =
            FourStateVector::from_msb_str("101").expect("valid vector");

        assert_eq!(
            FourStateVector::resolve_vectors([&first, &second]),
            Err(FourStateError::WidthMismatch {
                expected: 2,
                actual: 3,
            })
        );
    }

    #[test]
    fn scalar_boolean_conversion_requires_one_known_bit() {
        let zero = FourStateVector::from_msb_str("0").expect("valid vector");
        let unknown = FourStateVector::from_msb_str("x").expect("valid vector");
        let high_impedance =
            FourStateVector::from_msb_str("z").expect("valid vector");
        let multiple =
            FourStateVector::from_msb_str("01").expect("valid vector");

        assert_eq!(zero.to_bool(), Ok(false));
        assert_eq!(
            unknown.to_bool(),
            Err(FourStateError::UnknownConversion)
        );
        assert_eq!(
            high_impedance.to_bool(),
            Err(FourStateError::HighImpedanceConversion)
        );
        assert_eq!(
            multiple.to_bool(),
            Err(FourStateError::NotScalar { width: 2 })
        );
    }
}