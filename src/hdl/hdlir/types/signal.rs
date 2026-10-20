
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Signal Types
//!
//! Path: `src/hdl/hdlir/types/signal.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: Rust 1.97.
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Represent the target-independent type of a logical hardware signal in
//! HDLIR. A signal type describes whether the signal carries ordinary logic,
//! a clock, a reset, or a reference to a named semantic type.
//!
//! Ordinary logic types reuse the canonical HDLIR `LogicType` and therefore
//! preserve symbolic widths, signedness, and the selected logic-state model.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hdl.g4`.
//! - `grammar/hdl/signals.g4`.
//! - `grammar/Zamani.g4`.
//! - `src/frontend/semantic/domains/hdl.rs`.
//! - `src/hdl/design/signal.rs`.
//! - `src/hdl/hdlir/types/bit_width.rs`.
//! - `src/hdl/hdlir/types/logic.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, once established.
//!
//! The grammar owns source syntax. The canonical frontend semantic model
//! owns source-level type meaning and identity. `LogicType` owns HDLIR logic
//! type semantics. This module composes those contracts into a signal type
//! suitable for use by HDLIR graph structures.
//!
//! ## Owns
//!
//! - The HDLIR signal-type classification.
//! - Preservation of named semantic type identities.
//! - Explicit conversion from canonical HDL semantic value types.
//! - Conservative signal-type compatibility queries.
//!
//! ## Does not own
//!
//! - Signal declarations, signal identity allocation, or name resolution.
//! - Module ports, direction rules, or port ordering.
//! - Signal connectivity, drivers, sinks, or alias resolution.
//! - Source parsing, AST construction, or semantic validation.
//! - A competing logic type or bit-width representation.
//! - Concrete runtime logic values or truth tables.
//! - Physical pins, wires, registers, placement, routing, or allocation.
//! - Target capability discovery, synthesis, or timing closure.
//!
//! Signal declarations remain owned by the existing design layer and later
//! HDLIR graph modules. Port direction remains governed by the canonical
//! semantic contract and the HDLIR port model. Physical realization remains
//! downstream.
//!
//! ## Scalability
//!
//! This module introduces no universal maximum signal width, number of
//! signals, number of types, or hardware capacity.
//!
//! Widths remain symbolic through `LogicType` until the responsible stage
//! explicitly evaluates them. Width-conversion budgets limit compiler work,
//! not the set of legal hardware designs. Budget exhaustion is an explicit
//! error rather than a successful partial conversion.
//!
//! Named types preserve their canonical `HdlId`; this module does not
//! resolve them or infer their representation. Type resolution belongs to
//! semantic analysis or an explicitly defined later lowering stage.
//!
//! Concrete width representation follows the current canonical semantic
//! contract. This module does not claim arbitrary-precision concrete widths.
//!
//! ## Integration
//!
//! - `types/mod.rs` registers and exports this module.
//! - `types/logic.rs` remains the sole HDLIR owner of logic-type semantics.
//! - `lowering/from_semantic.rs` uses `from_semantic_type` after the
//!   appropriate source semantics and state-model policy are established.
//! - `ir/ports.rs` uses signal types when validating logical port contracts.
//! - `ir/operations.rs` and `ir/connectivity.rs` use compatibility results
//!   where their operation or connection semantics require type agreement.
//! - `verify/types.rs` must treat indeterminate compatibility as unproved,
//!   never as success.
//! - Serialization must preserve each variant and all nested type data.
//!
//! This module intentionally does not depend on future builders, IR graph
//! structures, verification modules, or serialization APIs.
//!
//! ## Compatibility and freeze criteria
//!
//! - Existing canonical semantic identities and types are reused.
//! - Clock, reset, and named types are not silently coerced into bit vectors.
//! - Different named identities are not assumed compatible.
//! - Symbolic width uncertainty is preserved in compatibility results.
//! - Width conversion errors and budget exhaustion remain explicit.
//! - Public APIs and all variants are documented and tested.
//! - Formatting, compilation, focused tests, repository tests, and Clippy
//!   pass after the module is registered in the actual checkout.

use core::fmt;

use crate::frontend::semantic::domains::hdl::{
    HdlId, HdlValueType,
};

use super::logic::{
    LogicStateModel,
    LogicType,
    LogicTypeCompatibility,
    LogicTypeError,
    WidthConversionBudget,
};

/// Target-independent type of a logical HDLIR signal.
///
/// This describes the semantic category of a signal's value. It does not
/// describe the signal declaration, direction, driver, physical wire, or
/// storage implementation.
///
/// `Named` preserves the canonical semantic type identity. Its underlying
/// representation must be resolved by the semantic or lowering layer; this
/// enum does not assume that a named type is an ordinary logic vector.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignalType {
    /// A Boolean or vector-valued digital logic signal.
    ///
    /// Width, signedness, and two-state/four-state classification are owned
    /// by the contained `LogicType`.
    Logic(LogicType),

    /// A logical clock signal.
    ///
    /// This classification does not specify clock frequency, clock-tree
    /// implementation, timing closure, or physical clock resources.
    Clock,

    /// A logical reset signal.
    ///
    /// Reset polarity, synchronous/asynchronous behavior, and association
    /// with a clock are specified by the relevant semantic or IR contracts.
    Reset,

    /// A reference to a canonical, named semantic type.
    ///
    /// The identity is preserved without resolving or copying its definition.
    Named(HdlId),
}

/// Result of comparing two signal types using only the available type data.
///
/// `Indeterminate` means the available symbolic information is insufficient
/// to prove equality or incompatibility. It must not be treated as verified
/// compatibility.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignalTypeCompatibility {
    /// Available information proves that the types are compatible.
    Compatible,

    /// Available information proves that the types are incompatible.
    Incompatible,

    /// Available information cannot prove compatibility or incompatibility.
    Indeterminate,
}

/// Errors converting canonical semantic value types into HDLIR signal types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SignalTypeError {
    /// Conversion of an ordinary logic type failed.
    Logic(LogicTypeError),
}

impl fmt::Display for SignalTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Logic(error) => {
                write!(formatter, "cannot construct HDLIR signal type: {error}")
            }
        }
    }
}

impl std::error::Error for SignalTypeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Logic(error) => Some(error),
        }
    }
}

impl From<LogicTypeError> for SignalTypeError {
    fn from(error: LogicTypeError) -> Self {
        Self::Logic(error)
    }
}

impl SignalType {
    /// Construct a signal type for a logic value.
    ///
    /// The supplied `LogicType` retains its existing width, signedness, and
    /// logic-state semantics. No width evaluation or target selection occurs.
    #[must_use]
    pub const fn logic(logic_type: LogicType) -> Self {
        Self::Logic(logic_type)
    }

    /// Construct a clock signal type.
    ///
    /// This is a semantic category, not a clock implementation or timing
    /// guarantee.
    #[must_use]
    pub const fn clock() -> Self {
        Self::Clock
    }

    /// Construct a reset signal type.
    ///
    /// Polarity and synchronous/asynchronous behavior are modeled separately.
    #[must_use]
    pub const fn reset() -> Self {
        Self::Reset
    }

    /// Construct a reference to an existing named semantic type.
    ///
    /// This does not prove that the identifier resolves in the current scope.
    /// Resolution remains the responsibility of the canonical semantic or
    /// lowering layer.
    #[must_use]
    pub const fn named(type_id: HdlId) -> Self {
        Self::Named(type_id)
    }

    /// Convert a canonical semantic HDL value type into an HDLIR signal type.
    ///
    /// `state_model` is an explicit policy input for ordinary bit and
    /// bit-vector types. It must come from the language semantics or the
    /// applicable import/lowering policy; it must not be guessed from the
    /// target hardware.
    ///
    /// This convenience method uses an operationally unbounded width-node
    /// budget. Applications processing untrusted or very large inputs should
    /// use [`Self::from_semantic_type_with_budget`].
    ///
    /// Clock, reset, and named semantic types retain their own categories.
    /// Named types preserve their canonical identity without resolution.
    ///
    /// # Errors
    ///
    /// Returns an error if a bit or bit-vector type cannot be represented
    /// under the current HDLIR logic-type contract, including invalid widths,
    /// unsupported signedness, or a width-conversion failure.
    pub fn from_semantic_type(
        value_type: &HdlValueType,
        state_model: LogicStateModel,
    ) -> Result<Self, SignalTypeError> {
        Self::from_semantic_type_with_budget(
            value_type,
            state_model,
            WidthConversionBudget::unbounded(),
        )
    }

    /// Convert a canonical semantic HDL value type with an explicit work
    /// budget for converting symbolic width expressions.
    ///
    /// The budget limits conversion work only; it is not a maximum signal
    /// width or a hardware-capacity limit.
    ///
    /// # Errors
    ///
    /// Returns a structured error if logic-type conversion fails. Exceeding
    /// the width-conversion budget is an incomplete conversion, not proof
    /// that the source design is semantically invalid.
    pub fn from_semantic_type_with_budget(
        value_type: &HdlValueType,
        state_model: LogicStateModel,
        budget: WidthConversionBudget,
    ) -> Result<Self, SignalTypeError> {
        match value_type {
            HdlValueType::Bit | HdlValueType::BitVector { .. } => {
                LogicType::from_semantic_type_with_budget(
                    value_type,
                    state_model,
                    budget,
                )
                .map(Self::Logic)
                .map_err(SignalTypeError::Logic)
            }
            HdlValueType::Clock => Ok(Self::Clock),
            HdlValueType::Reset => Ok(Self::Reset),
            HdlValueType::Named(type_id) => Ok(Self::Named(type_id.clone())),
        }
    }

    /// Return the contained logic type, if this is a logic signal.
    #[must_use]
    pub const fn as_logic(&self) -> Option<&LogicType> {
        match self {
            Self::Logic(logic_type) => Some(logic_type),
            Self::Clock | Self::Reset | Self::Named(_) => None,
        }
    }

    /// Compare this signal type with another type conservatively.
    ///
    /// Logic types delegate to `LogicType::compatibility_with`, preserving
    /// its distinctions between proven compatibility, proven incompatibility,
    /// and indeterminate symbolic-width relationships.
    ///
    /// Clock and reset types are compatible only with their own category.
    /// Named types are compatible when their canonical identities are equal.
    #[must_use]
    pub fn compatibility_with(
        &self,
        other: &Self,
    ) -> SignalTypeCompatibility {
        match (self, other) {
            (Self::Logic(left), Self::Logic(right)) => {
                match left.compatibility_with(right) {
                    LogicTypeCompatibility::Compatible => {
                        SignalTypeCompatibility::Compatible
                    }
                    LogicTypeCompatibility::Incompatible => {
                        SignalTypeCompatibility::Incompatible
                    }
                    LogicTypeCompatibility::Indeterminate => {
                        SignalTypeCompatibility::Indeterminate
                    }
                }
            }
            (Self::Clock, Self::Clock) | (Self::Reset, Self::Reset) => {
                SignalTypeCompatibility::Compatible
            }
            (Self::Named(left), Self::Named(right)) if left == right => {
                SignalTypeCompatibility::Compatible
            }
            (Self::Logic(_), Self::Clock | Self::Reset | Self::Named(_))
            | (Self::Clock | Self::Reset | Self::Named(_), Self::Logic(_))
            | (Self::Clock, Self::Reset)
            | (Self::Reset, Self::Clock)
            | (Self::Clock | Self::Reset, Self::Named(_))
            | (Self::Named(_), Self::Clock | Self::Reset)
            | (Self::Named(_), Self::Named(_)) => {
                SignalTypeCompatibility::Incompatible
            }
        }
    }

    /// Return whether the available type information proves compatibility.
    ///
    /// An indeterminate symbolic comparison returns `false`. Callers that
    /// need to distinguish indeterminate from incompatible should use
    /// [`Self::compatibility_with`] directly.
    #[must_use]
    pub fn is_proven_compatible_with(&self, other: &Self) -> bool {
        self.compatibility_with(other) == SignalTypeCompatibility::Compatible
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SignalType,
        SignalTypeCompatibility,
        SignalTypeError,
    };
    use crate::frontend::semantic::domains::hdl::{
        HdlId,
        HdlSignedness,
        HdlValueType,
        HdlWidth,
    };
    use crate::hdl::hdlir::types::logic::{
        LogicSignedness,
        LogicStateModel,
        LogicType,
        WidthConversionBudget,
    };
    use crate::hdl::hdlir::types::bit_width::BitWidth;

    #[test]
    fn converts_one_bit_semantic_type() {
        let result = SignalType::from_semantic_type(
            &HdlValueType::Bit,
            LogicStateModel::TwoState,
        );

        assert!(matches!(
            result,
            Ok(SignalType::Logic(_))
        ));

        let logic = result.expect("valid semantic bit must convert");
        let logic = logic.as_logic().expect("bit must be a logic type");

        assert_eq!(logic.width().as_known(), Some(1));
        assert_eq!(logic.signedness(), LogicSignedness::Unsigned);
        assert_eq!(logic.state_model(), LogicStateModel::TwoState);
    }

    #[test]
    fn converts_vector_without_evaluating_symbolic_width() {
        let semantic_type = HdlValueType::BitVector {
            width: HdlWidth::Symbol(
                HdlId::new("WORD_WIDTH").expect("non-empty test identity"),
            ),
            signedness: HdlSignedness::Unsigned,
        };

        let converted = SignalType::from_semantic_type(
            &semantic_type,
            LogicStateModel::FourState,
        )
        .expect("symbolic vector type must be preserved");

        let logic = converted.as_logic().expect("vector is logic");

        assert_eq!(logic.width(), &BitWidth::symbol("WORD_WIDTH")
            .expect("non-empty test symbol"));
        assert_eq!(logic.state_model(), LogicStateModel::FourState);
    }

    #[test]
    fn preserves_clock_reset_and_named_type_categories() {
        assert_eq!(
            SignalType::from_semantic_type(
                &HdlValueType::Clock,
                LogicStateModel::TwoState,
            )
            .expect("clock conversion"),
            SignalType::Clock,
        );

        assert_eq!(
            SignalType::from_semantic_type(
                &HdlValueType::Reset,
                LogicStateModel::TwoState,
            )
            .expect("reset conversion"),
            SignalType::Reset,
        );

        let id = HdlId::new("custom_bus").expect("non-empty test identity");

        assert_eq!(
            SignalType::from_semantic_type(
                &HdlValueType::Named(id.clone()),
                LogicStateModel::TwoState,
            )
            .expect("named type conversion"),
            SignalType::Named(id),
        );
    }

    #[test]
    fn distinguishes_type_compatibility() {
        let clock = SignalType::Clock;
        let reset = SignalType::Reset;

        assert_eq!(
            clock.compatibility_with(&clock),
            SignalTypeCompatibility::Compatible,
        );
        assert_eq!(
            clock.compatibility_with(&reset),
            SignalTypeCompatibility::Incompatible,
        );

        let left = SignalType::Logic(
            LogicType::vector(
                BitWidth::known(8).expect("positive width"),
                LogicSignedness::Unsigned,
                LogicStateModel::TwoState,
            )
            .expect("valid logic type"),
        );

        let right = SignalType::Logic(
            LogicType::vector(
                BitWidth::known(16).expect("positive width"),
                LogicSignedness::Unsigned,
                LogicStateModel::TwoState,
            )
            .expect("valid logic type"),
        );

        assert_eq!(
            left.compatibility_with(&right),
            SignalTypeCompatibility::Incompatible,
        );
    }

    #[test]
    fn does_not_assume_different_named_types_are_compatible() {
        let left = SignalType::Named(
            HdlId::new("type_a").expect("non-empty test identity"),
        );
        let right = SignalType::Named(
            HdlId::new("type_b").expect("non-empty test identity"),
        );

        assert_eq!(
            left.compatibility_with(&right),
            SignalTypeCompatibility::Incompatible,
        );
    }

    #[test]
    fn invalid_semantic_width_is_reported() {
        let semantic_type = HdlValueType::BitVector {
            width: HdlWidth::Known(0),
            signedness: HdlSignedness::Unsigned,
        };

        let result = SignalType::from_semantic_type(
            &semantic_type,
            LogicStateModel::TwoState,
        );

        assert!(matches!(result, Err(SignalTypeError::Logic(_))));
    }

    #[test]
    fn exposes_logic_type_without_copying_it() {
        let logic_type = LogicType::vector(
            BitWidth::known(3).expect("positive width"),
            LogicSignedness::Signed,
            LogicStateModel::FourState,
        )
        .expect("valid logic type");

        let signal_type = SignalType::logic(logic_type.clone());

        assert_eq!(signal_type.as_logic(), Some(&logic_type));
        assert!(signal_type.is_proven_compatible_with(&signal_type));
    }

    #[test]
    fn explicit_conversion_budget_is_forwarded() {
        let semantic_type = HdlValueType::BitVector {
            width: HdlWidth::Known(32),
            signedness: HdlSignedness::Signed,
        };

        let converted = SignalType::from_semantic_type_with_budget(
            &semantic_type,
            LogicStateModel::FourState,
            WidthConversionBudget::unbounded(),
        );

        assert!(converted.is_ok());
    }
}
