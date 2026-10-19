
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Logic Types
//!
//! Path: `src/hdl/types/logic.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the target-independent type contract for digital logic values used
//! by Zamani HDL. A logic type describes the width, signedness, and permitted
//! logic-state model of a value without describing its physical realization.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/signals.g4`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/widths.rs`
//!
//! The grammar owns source syntax. The existing semantic HDL module owns the
//! canonical `HdlWidth`, `HdlSignedness`, and `HdlValueType` definitions.
//! This module reuses those types rather than creating competing definitions.
//!
//! ## Owns
//!
//! - Logic-type invariants.
//! - The distinction between two-state and four-state logic type models.
//! - Checked construction of concrete and symbolic logic types.
//! - Compatibility checks that can be decided from the available type data.
//! - An explicit adapter from the existing semantic value-type contract.
//!
//! ## Does not own
//!
//! - Source parsing, AST construction, name resolution, or general type
//!   inference.
//! - A replacement width-expression representation.
//! - Actual logic-value storage or bit-state operations.
//! - Expression evaluation, assignment execution, or simulation.
//! - Clock/reset behavior or process semantics.
//! - Physical bit layouts, pins, wires, cells, or target allocation.
//! - HDLIR structures, builders, verification, passes, or lowering.
//!
//! Actual value states belong in `src/hdl/values/`. Symbolic width evaluation
//! belongs in `src/hdl/widths.rs`. Physical feasibility belongs downstream.
//!
//! ## Scalability
//!
//! This module defines no maximum vector width, signal count, or design size.
//! Widths reuse the existing `HdlWidth` representation, including symbolic
//! widths. The existing representation uses `u128` for concrete widths;
//! this module does not silently claim support for concrete integers beyond
//! that representation. Extending the canonical numeric representation is a
//! separate semantic-contract change, not a hidden limit introduced here.
//!
//! Symbolic widths are not evidence that a target can realize the type.
//! Target feasibility is determined separately.
//!
//! ## Integration
//!
//! `src/hdl/types/mod.rs` should declare and re-export this module. The HDL
//! semantic adapter may convert `HdlValueType::Bit` and
//! `HdlValueType::BitVector` through `LogicType::from_semantic_type`.
//! Clock, reset, and named types remain owned by their existing semantic
//! contracts and are deliberately not silently converted to ordinary logic
//! types.
//!
//! The `state_model` field describes the type's logic-state model. It does
//! not itself represent a bit value or authorize conversion of unknown or
//! high-impedance states.
//!
//! ## Freeze criteria
//!
//! - Width and signedness reuse the canonical semantic definitions.
//! - Invalid concrete zero widths are rejected.
//! - Symbolic widths remain symbolic.
//! - Compatibility results distinguish incompatible from indeterminate.
//! - No target-specific assumptions or HDLIR dependencies are introduced.
//! - Unit tests pass with the repository's declared Rust toolchain.

use std::fmt;

use crate::frontend::semantic::domains::hdl::{
    HdlSignedness, HdlValueType, HdlWidth,
};

/// The logic-state model permitted by a logic type.
///
/// This describes type-level interpretation, not a particular runtime or
/// simulated value. The language front end and import adapters must select
/// the model according to the applicable language and source semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LogicStateModel {
    /// Values have only known logical zero and logical one states.
    TwoState,

    /// Values may additionally represent unknown and high-impedance states
    /// where those states are defined by the applicable language semantics.
    FourState,
}

impl LogicStateModel {
    /// Return whether the model permits an unknown state to be represented.
    ///
    /// This is a type-model capability, not a statement that every context
    /// permits an unknown value.
    #[must_use]
    pub const fn supports_unknown(self) -> bool {
        matches!(self, Self::FourState)
    }

    /// Return whether the model permits a high-impedance state to be
    /// represented.
    ///
    /// Contextual legality and the meaning of high impedance remain the
    /// responsibility of the relevant semantic or value operation.
    #[must_use]
    pub const fn supports_high_impedance(self) -> bool {
        matches!(self, Self::FourState)
    }
}

/// A target-independent digital logic type.
///
/// `width` is the canonical semantic HDL width. A width may be concrete or
/// symbolic; it is not converted to a host-sized integer merely to construct
/// this type.
///
/// `signedness` describes the interpretation of the bit vector. It does not
/// change the number of bits.
///
/// `state_model` specifies which logic-state model the type supports.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LogicType {
    width: HdlWidth,
    signedness: HdlSignedness,
    state_model: LogicStateModel,
}

/// Errors encountered while constructing or checking a logic type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LogicTypeError {
    /// A concrete width, or a concrete width component, is zero.
    ZeroWidth,

    /// The width expression contains a structurally invalid concrete width.
    InvalidWidthExpression,

    /// Two logic types have widths known to be different.
    WidthMismatch {
        /// Expected width.
        expected: HdlWidth,

        /// Actual width.
        actual: HdlWidth,
    },

    /// Two logic types use incompatible signedness interpretations.
    SignednessMismatch {
        /// Expected signedness.
        expected: HdlSignedness,

        /// Actual signedness.
        actual: HdlSignedness,
    },

    /// The source semantic type is not an ordinary bit or bit-vector type.
    UnsupportedSemanticType,
}

impl fmt::Display for LogicTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroWidth => {
                f.write_str("HDL logic type width must be positive")
            }
            Self::InvalidWidthExpression => {
                f.write_str("HDL logic type contains an invalid width expression")
            }
            Self::WidthMismatch { expected, actual } => {
                write!(
                    f,
                    "HDL logic width mismatch: expected {expected:?}, \
                     found {actual:?}"
                )
            }
            Self::SignednessMismatch { expected, actual } => {
                write!(
                    f,
                    "HDL logic signedness mismatch: expected {expected:?}, \
                     found {actual:?}"
                )
            }
            Self::UnsupportedSemanticType => {
                f.write_str(
                    "semantic type is not an ordinary HDL bit or bit vector",
                )
            }
        }
    }
}

impl std::error::Error for LogicTypeError {}

/// The result of checking whether two logic types are compatible.
///
/// Symbolic expressions are not assumed equal merely because their values
/// have not yet been resolved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogicTypeCompatibility {
    /// The available type information proves compatibility.
    Compatible,

    /// The available type information proves incompatibility.
    Incompatible,

    /// Compatibility cannot be determined without resolving symbolic
    /// widths or obtaining additional semantic information.
    Indeterminate,
}

impl LogicType {
    /// Construct a logic type after checking its width expression.
    ///
    /// Symbolic widths are accepted without being evaluated. Concrete zero
    /// widths are rejected, including zero-width components within a
    /// compound width expression.
    pub fn new(
        width: HdlWidth,
        signedness: HdlSignedness,
        state_model: LogicStateModel,
    ) -> Result<Self, LogicTypeError> {
        validate_width_expression(&width)?;

        Ok(Self {
            width,
            signedness,
            state_model,
        })
    }

    /// Construct an unsigned one-bit, two-state logic type.
    ///
    /// The one-bit width is a semantic definition of this constructor, not a
    /// maximum width or a target-specific assumption.
    #[must_use]
    pub fn bit() -> Self {
        Self {
            width: HdlWidth::Known(1),
            signedness: HdlSignedness::Unsigned,
            state_model: LogicStateModel::TwoState,
        }
    }

    /// Construct an unsigned one-bit logic type using the specified
    /// logic-state model.
    #[must_use]
    pub fn bit_with_state_model(state_model: LogicStateModel) -> Self {
        Self {
            width: HdlWidth::Known(1),
            signedness: HdlSignedness::Unsigned,
            state_model,
        }
    }

    /// Construct a logic vector with the supplied width and signedness.
    pub fn vector(
        width: HdlWidth,
        signedness: HdlSignedness,
        state_model: LogicStateModel,
    ) -> Result<Self, LogicTypeError> {
        Self::new(width, signedness, state_model)
    }

    /// Adapt the existing canonical semantic value type.
    ///
    /// `Bit` maps to a one-bit two-state type. `BitVector` preserves its
    /// canonical width and signedness and uses the caller-provided state
    /// model. Clock, reset, and named types are not silently reinterpreted
    /// as ordinary logic types.
    pub fn from_semantic_type(
        value_type: &HdlValueType,
        state_model: LogicStateModel,
    ) -> Result<Self, LogicTypeError> {
        match value_type {
            HdlValueType::Bit => {
                Ok(Self::bit_with_state_model(state_model))
            }
            HdlValueType::BitVector {
                width,
                signedness,
            } => Self::new(
                width.clone(),
                *signedness,
                state_model,
            ),
            HdlValueType::Clock
            | HdlValueType::Reset
            | HdlValueType::Named(_) => {
                Err(LogicTypeError::UnsupportedSemanticType)
            }
        }
    }

    /// Return the canonical width expression.
    #[must_use]
    pub fn width(&self) -> &HdlWidth {
        &self.width
    }

    /// Return the declared signedness interpretation.
    #[must_use]
    pub const fn signedness(&self) -> HdlSignedness {
        self.signedness
    }

    /// Return the permitted logic-state model.
    #[must_use]
    pub const fn state_model(&self) -> LogicStateModel {
        self.state_model
    }

    /// Return whether the width is a concrete one-bit width.
    #[must_use]
    pub const fn is_concrete_bit(&self) -> bool {
        matches!(self.width, HdlWidth::Known(1))
    }

    /// Return whether the width is concrete.
    #[must_use]
    pub const fn has_concrete_width(&self) -> bool {
        matches!(self.width, HdlWidth::Known(_))
    }

    /// Check compatibility with another logic type.
    ///
    /// Widths that are both concrete can be compared directly. Identical
    /// symbolic expressions can be recognized as structurally equal, but
    /// differently written symbolic expressions are not assumed equivalent.
    ///
    /// State-model compatibility is conservative: a four-state type can
    /// represent more states than a two-state type, but this fact alone does
    /// not authorize a conversion that loses observable information.
    #[must_use]
    pub fn compatibility_with(
        &self,
        other: &Self,
    ) -> LogicTypeCompatibility {
        if self.signedness != other.signedness {
            return LogicTypeCompatibility::Incompatible;
        }

        if self.state_model != other.state_model {
            return LogicTypeCompatibility::Incompatible;
        }

        match (&self.width, &other.width) {
            (HdlWidth::Known(left), HdlWidth::Known(right)) => {
                if left == right {
                    LogicTypeCompatibility::Compatible
                } else {
                    LogicTypeCompatibility::Incompatible
                }
            }
            (left, right) if left == right => {
                LogicTypeCompatibility::Compatible
            }
            _ => LogicTypeCompatibility::Indeterminate,
        }
    }

    /// Check exact type compatibility and return a structured error when
    /// the available type information proves incompatibility.
    ///
    /// An unresolved symbolic comparison is reported as indeterminate rather
    /// than incorrectly accepted as proven compatible.
    pub fn require_compatible_with(
        &self,
        other: &Self,
    ) -> Result<(), LogicTypeError> {
        if self.signedness != other.signedness {
            return Err(LogicTypeError::SignednessMismatch {
                expected: self.signedness,
                actual: other.signedness,
            });
        }

        match (&self.width, &other.width) {
            (HdlWidth::Known(left), HdlWidth::Known(right))
                if left != right =>
            {
                return Err(LogicTypeError::WidthMismatch {
                    expected: self.width.clone(),
                    actual: other.width.clone(),
                });
            }
            (left, right) if left != right => {
                return Err(LogicTypeError::InvalidWidthExpression);
            }
            _ => {}
        }

        if self.state_model != other.state_model {
            return Err(LogicTypeError::InvalidWidthExpression);
        }

        Ok(())
    }
}

/// Validate the structure of a canonical HDL width expression.
///
/// This function deliberately does not evaluate symbolic expressions.
/// Resolution and checked arithmetic belong to `src/hdl/widths.rs`.
fn validate_width_expression(width: &HdlWidth) -> Result<(), LogicTypeError> {
    match width {
        HdlWidth::Known(0) => Err(LogicTypeError::ZeroWidth),
        HdlWidth::Known(_) => Ok(()),
        HdlWidth::Symbol(_) | HdlWidth::Unresolved(_) => Ok(()),
        HdlWidth::Add(left, right) | HdlWidth::Multiply(left, right) => {
            validate_width_expression(left)?;
            validate_width_expression(right)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::HdlId;

    fn symbol(name: &str) -> HdlId {
        HdlId::new(name).expect("test symbol should be valid")
    }

    #[test]
    fn bit_is_one_bit_wide() {
        let ty = LogicType::bit();

        assert!(ty.is_concrete_bit());
        assert_eq!(ty.width(), &HdlWidth::Known(1));
        assert_eq!(ty.signedness(), HdlSignedness::Unsigned);
        assert_eq!(ty.state_model(), LogicStateModel::TwoState);
    }

    #[test]
    fn four_state_bit_preserves_state_model() {
        let ty = LogicType::bit_with_state_model(LogicStateModel::FourState);

        assert!(ty.is_concrete_bit());
        assert!(ty.state_model().supports_unknown());
        assert!(ty.state_model().supports_high_impedance());
    }

    #[test]
    fn concrete_zero_width_is_rejected() {
        let result = LogicType::new(
            HdlWidth::Known(0),
            HdlSignedness::Unsigned,
            LogicStateModel::TwoState,
        );

        assert_eq!(result, Err(LogicTypeError::ZeroWidth));
    }

    #[test]
    fn zero_width_inside_expression_is_rejected() {
        let width = HdlWidth::Add(
            Box::new(HdlWidth::Known(8)),
            Box::new(HdlWidth::Known(0)),
        );

        let result = LogicType::new(
            width,
            HdlSignedness::Unsigned,
            LogicStateModel::TwoState,
        );

        assert_eq!(result, Err(LogicTypeError::ZeroWidth));
    }

    #[test]
    fn symbolic_width_is_preserved() {
        let width = HdlWidth::Symbol(symbol("DATA_WIDTH"));
        let ty = LogicType::vector(
            width.clone(),
            HdlSignedness::Unsigned,
            LogicStateModel::TwoState,
        )
        .expect("symbolic width should be accepted");

        assert_eq!(ty.width(), &width);
        assert!(!ty.has_concrete_width());
    }

    #[test]
    fn equal_concrete_types_are_compatible() {
        let left = LogicType::vector(
            HdlWidth::Known(128),
            HdlSignedness::Unsigned,
            LogicStateModel::TwoState,
        )
        .expect("valid type");

        let right = left.clone();

        assert_eq!(
            left.compatibility_with(&right),
            LogicTypeCompatibility::Compatible
        );
        assert!(left.require_compatible_with(&right).is_ok());
    }

    #[test]
    fn different_concrete_widths_are_incompatible() {
        let left = LogicType::vector(
            HdlWidth::Known(8),
            HdlSignedness::Unsigned,
            LogicStateModel::TwoState,
        )
        .expect("valid type");

        let right = LogicType::vector(
            HdlWidth::Known(16),
            HdlSignedness::Unsigned,
            LogicStateModel::TwoState,
        )
        .expect("valid type");

        assert_eq!(
            left.compatibility_with(&right),
            LogicTypeCompatibility::Incompatible
        );

        assert!(matches!(
            left.require_compatible_with(&right),
            Err(LogicTypeError::WidthMismatch { .. })
        ));
    }

    #[test]
    fn unresolved_different_widths_are_indeterminate() {
        let left = LogicType::vector(
            HdlWidth::Symbol(symbol("WIDTH_A")),
            HdlSignedness::Unsigned,
            LogicStateModel::TwoState,
        )
        .expect("valid symbolic type");

        let right = LogicType::vector(
            HdlWidth::Symbol(symbol("WIDTH_B")),
            HdlSignedness::Unsigned,
            LogicStateModel::TwoState,
        )
        .expect("valid symbolic type");

        assert_eq!(
            left.compatibility_with(&right),
            LogicTypeCompatibility::Indeterminate
        );
    }

    #[test]
    fn signedness_mismatch_is_incompatible() {
        let unsigned = LogicType::vector(
            HdlWidth::Known(16),
            HdlSignedness::Unsigned,
            LogicStateModel::TwoState,
        )
        .expect("valid type");

        let signed = LogicType::vector(
            HdlWidth::Known(16),
            HdlSignedness::Signed,
            LogicStateModel::TwoState,
        )
        .expect("valid type");

        assert_eq!(
            unsigned.compatibility_with(&signed),
            LogicTypeCompatibility::Incompatible
        );

        assert!(matches!(
            unsigned.require_compatible_with(&signed),
            Err(LogicTypeError::SignednessMismatch { .. })
        ));
    }

    #[test]
    fn semantic_bit_type_can_be_adapted() {
        let ty = LogicType::from_semantic_type(
            &HdlValueType::Bit,
            LogicStateModel::FourState,
        )
        .expect("bit type should be adaptable");

        assert!(ty.is_concrete_bit());
        assert_eq!(ty.state_model(), LogicStateModel::FourState);
    }

    #[test]
    fn semantic_bit_vector_preserves_width_and_signedness() {
        let width = HdlWidth::Symbol(symbol("BUS_WIDTH"));
        let source = HdlValueType::BitVector {
            width: width.clone(),
            signedness: HdlSignedness::Signed,
        };

        let ty = LogicType::from_semantic_type(
            &source,
            LogicStateModel::TwoState,
        )
        .expect("bit vector should be adaptable");

        assert_eq!(ty.width(), &width);
        assert_eq!(ty.signedness(), HdlSignedness::Signed);
    }

    #[test]
    fn clock_type_is_not_silently_converted_to_logic() {
        assert_eq!(
            LogicType::from_semantic_type(
                &HdlValueType::Clock,
                LogicStateModel::TwoState,
            ),
            Err(LogicTypeError::UnsupportedSemanticType)
        );
    }

    #[test]
    fn reset_type_is_not_silently_converted_to_logic() {
        assert_eq!(
            LogicType::from_semantic_type(
                &HdlValueType::Reset,
                LogicStateModel::TwoState,
            ),
            Err(LogicTypeError::UnsupportedSemanticType)
        );
    }
}
