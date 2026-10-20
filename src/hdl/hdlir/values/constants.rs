
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Constants
//!
//! Path: `src/hdl/hdlir/values/constants.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Proposed implementation; compile and integration verification required.
//!
//! ## Purpose
//!
//! Represent concrete, target-independent constant values used by HDLIR.
//! The implementation supports:
//!
//! - Exact two-state and four-state logic bit patterns.
//! - Dynamically sized bit vectors.
//! - Nested aggregate constants.
//! - Text constants.
//! - Explicit width, state-model, numeric-range, and work-budget errors.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hdl.g4` and the applicable HDL delegate grammars.
//! - `grammar/hdl/signals.g4`.
//! - `grammar/hdl/arrays.g4`.
//! - `src/frontend/semantic/domains/hdl.rs`.
//! - `src/hdl/hdlir/types/bit_width.rs`.
//! - `src/hdl/hdlir/types/logic.rs`.
//! - `src/hdl/hdlir/types/aggregate.rs`.
//! - `src/hdl/values/four_state.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, once established.
//!
//! Grammar and canonical semantic analysis determine which constants are
//! legal in a particular source-language context. This module owns their
//! HDLIR value representation and local validity checks.
//!
//! ## Ownership
//!
//! This module owns concrete HDLIR constant representations and their
//! local invariants.
//!
//! It does not own source parsing, name resolution, semantic type inference,
//! symbolic expression evaluation, aggregate type declarations, signal
//! declarations, simulation, synthesis, or physical hardware allocation.
//!
//! ## Scalability
//!
//! No universal hardware-width or aggregate-size ceiling is defined here.
//! Bit vectors and aggregates use dynamically sized storage. Numeric
//! convenience constructors accept explicit work budgets where creating a
//! large concrete value would require substantial allocation.
//!
//! A budget limits construction work; it does not change source-language
//! semantics. Symbolic widths cannot be used to validate a concrete bit
//! vector until the responsible stage resolves them.
//!
//! Allocation availability, Rust collection representation, and concrete
//! integer ranges remain real implementation constraints. No finite
//! implementation can materialize literally infinite values.
//!
//! ## Bit ordering
//!
//! `LogicConstant::bits()` stores bits least-significant-bit first.
//! `LogicConstant::from_msb_text()` accepts text most-significant-bit first.
//! These conventions match `src/hdl/values/four_state.rs`.
//!
//! ## Integration
//!
//! - `values/mod.rs` must declare `pub mod constants;` and deliberately
//!   re-export the public API.
//! - `types/logic.rs` remains authoritative for HDLIR logic type semantics.
//! - `src/hdl/values/four_state.rs` remains authoritative for individual
//!   logic-state values.
//! - `types/aggregate.rs` remains authoritative for aggregate types.
//! - `verify/types.rs` must check a constant against the type required by
//!   its containing operation, declaration, or initializer.
//! - `lowering/from_semantic.rs` must construct constants only after
//!   preserving the source-language literal's actual semantics.
//! - Serialization must preserve exact states, bit ordering, aggregate
//!   ordering, and text contents.
//!
//! ## Freeze criteria
//!
//! This file is complete only after it compiles against the actual sibling
//! APIs, its tests pass, its public exports are registered, and the required
//! repository checks pass.

use core::fmt;

use crate::hdl::values::four_state::LogicState;

use super::super::types::logic::{
    LogicStateModel,
    LogicType,
};

/// A concrete logic bit vector with an associated HDLIR logic type.
///
/// The vector stores its least-significant bit at index zero. The associated
/// type must have a concrete width equal to the number of stored bits.
/// Symbolic widths are preserved by `LogicType`, but cannot validate a
/// concrete vector until resolved by an appropriate earlier stage.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct LogicConstant {
    ty: LogicType,
    bits: Vec<LogicState>,
}

/// A constant value represented in HDLIR.
///
/// Aggregate values preserve their element order but do not independently
/// establish the aggregate's declared type. Type verification must compare
/// an aggregate constant against the relevant `AggregateType` and reject
/// incompatible shapes or element types.
///
/// Text values preserve their exact UTF-8 contents. Whether text constants
/// are legal in a particular HDL construct is decided by semantic
/// validation and the applicable HDLIR operation or dialect.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ConstantValue {
    /// A concrete logic scalar or bit-vector constant.
    Logic(LogicConstant),

    /// An ordered aggregate of nested constant values.
    Aggregate(Vec<ConstantValue>),

    /// A UTF-8 text constant.
    Text(String),
}

/// Errors encountered while constructing or validating HDLIR constants.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstantError {
    /// A concrete logic value requires a known width.
    SymbolicWidth,

    /// The concrete type width and supplied number of bits differ.
    WidthMismatch {
        /// Required concrete width.
        expected: u128,

        /// Actual number of supplied bits.
        actual: usize,
    },

    /// The type does not permit unknown logic states.
    UnknownStateNotAllowed,

    /// The type does not permit high-impedance logic states.
    HighImpedanceNotAllowed,

    /// A logic-text character is not a valid four-state bit.
    InvalidLogicCharacter {
        /// The invalid character.
        character: char,

        /// Zero-based character position in the input text.
        position: usize,
    },

    /// The requested numeric value cannot be represented at the selected
    /// width without losing significant bits.
    NumericValueDoesNotFit {
        /// Selected bit width.
        width: u128,
    },

    /// The requested concrete width exceeds the caller's work budget.
    ///
    /// This is a construction-work limit, not a language-level width limit.
    WidthBudgetExceeded {
        /// Requested width.
        requested: u128,

        /// Maximum number of bits permitted for this construction.
        budget: usize,
    },

    /// The requested width cannot be represented by the host collection
    /// length type.
    WidthNotRepresentableOnHost {
        /// Requested width.
        width: u128,
    },

    /// A requested collection allocation could not be reserved.
    AllocationFailed,
}

impl fmt::Display for ConstantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SymbolicWidth => {
                formatter.write_str(
                    "cannot validate a concrete logic constant against a symbolic width",
                )
            }
            Self::WidthMismatch { expected, actual } => {
                write!(
                    formatter,
                    "logic constant width mismatch: expected {expected} bits, got {actual}"
                )
            }
            Self::UnknownStateNotAllowed => {
                formatter.write_str(
                    "the logic type does not permit the unknown state",
                )
            }
            Self::HighImpedanceNotAllowed => {
                formatter.write_str(
                    "the logic type does not permit the high-impedance state",
                )
            }
            Self::InvalidLogicCharacter {
                character,
                position,
            } => {
                write!(
                    formatter,
                    "invalid logic character {character:?} at character position {position}"
                )
            }
            Self::NumericValueDoesNotFit { width } => {
                write!(
                    formatter,
                    "numeric value does not fit in {width} bits"
                )
            }
            Self::WidthBudgetExceeded { requested, budget } => {
                write!(
                    formatter,
                    "logic constant construction requires {requested} bits, exceeding the work budget of {budget} bits"
                )
            }
            Self::WidthNotRepresentableOnHost { width } => {
                write!(
                    formatter,
                    "logic width {width} cannot be represented as a host collection length"
                )
            }
            Self::AllocationFailed => {
                formatter.write_str(
                    "unable to reserve storage for the requested logic constant",
                )
            }
        }
    }
}

impl std::error::Error for ConstantError {}

impl LogicConstant {
    /// Construct a concrete logic constant from least-significant-bit-first
    /// storage.
    ///
    /// The number of supplied bits must equal the type's concrete width.
    /// Unknown and high-impedance states are rejected unless the associated
    /// logic type explicitly supports them.
    ///
    /// This constructor does not evaluate symbolic widths, infer types,
    /// resize values, or silently coerce logic states.
    ///
    /// # Errors
    ///
    /// Returns an error for symbolic widths, width mismatches, or states
    /// that the selected type cannot represent.
    pub fn new(
        ty: LogicType,
        bits: Vec<LogicState>,
    ) -> Result<Self, ConstantError> {
        let width = ty
            .width()
            .as_known()
            .ok_or(ConstantError::SymbolicWidth)?;

        let actual = u128::try_from(bits.len()).map_err(|_| {
            ConstantError::WidthNotRepresentableOnHost { width }
        })?;

        if width != actual {
            return Err(ConstantError::WidthMismatch {
                expected: width,
                actual: bits.len(),
            });
        }

        for state in &bits {
            match state {
                LogicState::Zero | LogicState::One => {}
                LogicState::Unknown if !ty.state_model().supports_unknown() => {
                    return Err(ConstantError::UnknownStateNotAllowed);
                }
                LogicState::HighImpedance
                    if !ty.state_model().supports_high_impedance() =>
                {
                    return Err(ConstantError::HighImpedanceNotAllowed);
                }
                LogicState::Unknown | LogicState::HighImpedance => {}
            }
        }

        Ok(Self { ty, bits })
    }

    /// Construct a logic constant from a textual bit pattern.
    ///
    /// The text is interpreted most-significant-bit first. Accepted logic
    /// characters are `0`, `1`, `x`, `X`, `z`, and `Z`.
    ///
    /// No implicit resizing or truncation is performed.
    ///
    /// # Errors
    ///
    /// Returns an error if the text contains an invalid character, its
    /// length differs from the concrete type width, or the selected type
    /// cannot represent one of its logic states.
    pub fn from_msb_text(
        ty: LogicType,
        text: &str,
    ) -> Result<Self, ConstantError> {
        let mut bits = Vec::new();

        bits.try_reserve_exact(text.chars().count())
            .map_err(|_| ConstantError::AllocationFailed)?;

        for (position, character) in text.chars().enumerate() {
            let state = LogicState::from_char(character).map_err(|_| {
                ConstantError::InvalidLogicCharacter {
                    character,
                    position,
                }
            })?;

            bits.push(state);
        }

        bits.reverse();
        Self::new(ty, bits)
    }

    /// Construct an unsigned numeric constant with an explicit bit budget.
    ///
    /// The numeric input is a `u128`; this convenience method therefore
    /// represents only values supplied through that integer type. Larger
    /// or arbitrary bit patterns can be supplied through `new()` or
    /// `from_msb_text()` instead.
    ///
    /// Bits beyond the numeric value's significant bits are zero-filled.
    /// The selected type must be unsigned and two-state or four-state;
    /// because this constructor emits only `Zero` and `One`, either state
    /// model is valid.
    ///
    /// `max_bits` is an operational construction budget, not a maximum
    /// legal HDL width. A caller can choose a larger budget when resources
    /// permit.
    ///
    /// # Errors
    ///
    /// Returns an error for a symbolic width, a signed type, an insufficient
    /// width, an exceeded budget, an unrepresentable collection length, or
    /// a failed allocation reservation.
    pub fn from_unsigned_with_budget(
        ty: LogicType,
        value: u128,
        max_bits: usize,
    ) -> Result<Self, ConstantError> {
        use super::super::types::logic::LogicSignedness;

        if ty.signedness() != LogicSignedness::Unsigned {
            return Err(ConstantError::NumericValueDoesNotFit {
                width: ty.width().as_known().unwrap_or(0),
            });
        }

        let width = ty
            .width()
            .as_known()
            .ok_or(ConstantError::SymbolicWidth)?;

        if width > max_bits as u128 {
            return Err(ConstantError::WidthBudgetExceeded {
                requested: width,
                budget: max_bits,
            });
        }

        let width_usize = usize::try_from(width)
            .map_err(|_| ConstantError::WidthNotRepresentableOnHost {
                width,
            })?;

        if width < u128::BITS as u128 && (value >> width) != 0 {
            return Err(ConstantError::NumericValueDoesNotFit { width });
        }

        let mut bits = Vec::new();

        bits.try_reserve_exact(width_usize)
            .map_err(|_| ConstantError::AllocationFailed)?;

        for index in 0..width_usize {
            let state = if index < u128::BITS as usize
                && ((value >> index) & 1) != 0
            {
                LogicState::One
            } else {
                LogicState::Zero
            };

            bits.push(state);
        }

        Self::new(ty, bits)
    }

    /// Return the associated HDLIR logic type.
    #[must_use]
    pub const fn ty(&self) -> &LogicType {
        &self.ty
    }

    /// Return the exact bit pattern, least-significant bit first.
    #[must_use]
    pub fn bits(&self) -> &[LogicState] {
        &self.bits
    }

    /// Return the concrete bit width.
    ///
    /// This is the length of the stored bit pattern, not an estimate of
    /// physical storage or a target resource allocation.
    #[must_use]
    pub fn width(&self) -> usize {
        self.bits.len()
    }

    /// Format the bit pattern most-significant-bit first.
    #[must_use]
    pub fn to_msb_text(&self) -> String {
        self.bits
            .iter()
            .rev()
            .map(|state| state.as_char())
            .collect()
    }
}

impl ConstantValue {
    /// Construct a logic constant.
    #[must_use]
    pub const fn logic(value: LogicConstant) -> Self {
        Self::Logic(value)
    }

    /// Construct an ordered aggregate constant.
    ///
    /// Element count and element types are checked against the declared
    /// aggregate type by the relevant verifier, not by this constructor.
    #[must_use]
    pub fn aggregate(elements: Vec<ConstantValue>) -> Self {
        Self::Aggregate(elements)
    }

    /// Construct a UTF-8 text constant.
    #[must_use]
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    /// Return the contained logic constant, if this is a logic value.
    #[must_use]
    pub const fn as_logic(&self) -> Option<&LogicConstant> {
        match self {
            Self::Logic(value) => Some(value),
            Self::Aggregate(_) | Self::Text(_) => None,
        }
    }

    /// Return the aggregate elements, if this is an aggregate value.
    #[must_use]
    pub fn as_aggregate(&self) -> Option<&[ConstantValue]> {
        match self {
            Self::Aggregate(elements) => Some(elements),
            Self::Logic(_) | Self::Text(_) => None,
        }
    }

    /// Return the text contents, if this is a text value.
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(value) => Some(value),
            Self::Logic(_) | Self::Aggregate(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdlir::types::bit_width::BitWidth;
    use crate::hdl::hdlir::types::logic::{
        LogicSignedness,
    };

    fn four_state_vector(width: u128) -> LogicType {
        LogicType::vector(
            BitWidth::known(width).expect("width must be positive"),
            LogicSignedness::Unsigned,
            LogicStateModel::FourState,
        )
        .expect("test logic type must be valid")
    }

    #[test]
    fn parses_and_formats_msb_first_without_losing_states() {
        let value = LogicConstant::from_msb_text(
            four_state_vector(4),
            "10xz",
        )
        .expect("four-state vector should be valid");

        assert_eq!(value.to_msb_text(), "10xz");
        assert_eq!(value.width(), 4);
        assert_eq!(
            value.bits(),
            &[
                LogicState::HighImpedance,
                LogicState::Unknown,
                LogicState::Zero,
                LogicState::One,
            ]
        );
    }

    #[test]
    fn rejects_width_mismatch_instead_of_truncating() {
        let result =
            LogicConstant::from_msb_text(four_state_vector(4), "101");

        assert!(matches!(
            result,
            Err(ConstantError::WidthMismatch {
                expected: 4,
                actual: 3
            })
        ));
    }

    #[test]
    fn rejects_unknown_state_for_two_state_type() {
        let result =
            LogicConstant::from_msb_text(LogicType::bit(), "x");

        assert_eq!(
            result,
            Err(ConstantError::UnknownStateNotAllowed)
        );
    }

    #[test]
    fn rejects_invalid_logic_character() {
        let result =
            LogicConstant::from_msb_text(four_state_vector(1), "2");

        assert!(matches!(
            result,
            Err(ConstantError::InvalidLogicCharacter {
                character: '2',
                position: 0
            })
        ));
    }

    #[test]
    fn numeric_constructor_preserves_exact_unsigned_value() {
        let value = LogicConstant::from_unsigned_with_budget(
            four_state_vector(8),
            0b1010_0101,
            8,
        )
        .expect("value fits in eight bits");

        assert_eq!(value.to_msb_text(), "10100101");
    }

    #[test]
    fn numeric_constructor_reports_budget_exhaustion() {
        let result = LogicConstant::from_unsigned_with_budget(
            four_state_vector(8),
            1,
            4,
        );

        assert!(matches!(
            result,
            Err(ConstantError::WidthBudgetExceeded {
                requested: 8,
                budget: 4
            })
        ));
    }

    #[test]
    fn aggregate_preserves_nested_order() {
        let first = ConstantValue::logic(
            LogicConstant::from_msb_text(LogicType::bit(), "1")
                .expect("one-bit constant should be valid"),
        );
        let second = ConstantValue::text("data");
        let value = ConstantValue::aggregate(vec![
            first.clone(),
            ConstantValue::aggregate(vec![second.clone()]),
        ]);

        let elements = value
            .as_aggregate()
            .expect("outer value should be an aggregate");

        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0], first);

        let nested = elements[1]
            .as_aggregate()
            .expect("second element should be nested");

        assert_eq!(nested[0], second);
    }
}
