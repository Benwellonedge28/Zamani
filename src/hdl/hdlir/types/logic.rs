
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # Zamani HDLIR Logic Types
//!
//! Path: `src/hdl/hdlir/types/logic.rs`
//! Edition: Rust 2021.
//! Minimum supported Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Proposed HDLIR implementation; repository integration and
//! conformance testing are required before it can be frozen.
//!
//! ## Purpose
//!
//! Define the HDLIR-owned type descriptor for digital logic values.
//! A descriptor preserves a symbolic or concrete width, signedness, and
//! permitted logic-state model without making assumptions about a physical
//! implementation.
//!
//! ## Normative authority
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/specification/semantic-model.md`.
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars.
//! - `src/frontend/semantic/domains/hdl.rs`.
//! - `src/hdl/types/logic.rs`.
//! - `src/hdl/values/four_state.rs`.
//! - `src/hdl/hdlir/types/bit_width.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, once established.
//!
//! The grammar owns syntax. The canonical frontend semantic model owns
//! source-level type meaning. The existing HDL type layer owns its
//! source-facing logic-type contract. HDLIR owns this IR-specific descriptor.
//!
//! ## Owns
//!
//! - HDLIR logic signedness.
//! - HDLIR two-state and four-state type classification.
//! - The HDLIR logic type descriptor and its invariants.
//! - Conservative type-compatibility queries.
//! - Checked conversion from the existing semantic HDL value-type contract.
//! - Iterative conversion of symbolic width expressions.
//!
//! ## Does not own
//!
//! - Source parsing, AST nodes, name resolution, or semantic truth.
//! - A replacement for `HdlWidth` or `BitWidth`.
//! - Runtime logic values, truth tables, or multi-driver resolution.
//! - Simulation time, process execution, clocking, or sequential behavior.
//! - Physical bit layouts, pins, wires, cells, placement, or routing.
//! - Target capability discovery or hardware allocation.
//! - HDLIR builders, graph verification, optimization, or serialization.
//!
//! Concrete logic values and their four-state operations remain owned by
//! `src/hdl/values/four_state.rs`. Symbolic HDLIR widths remain owned by
//! `src/hdl/hdlir/types/bit_width.rs`.
//!
//! ## Scalability
//!
//! There is no universal maximum logic-vector width or hardware-design size.
//! Symbolic widths remain symbolic until a caller explicitly requests
//! conversion or evaluation. Concrete widths use the existing HDLIR
//! `BitWidth` representation and its checked numeric semantics.
//!
//! Semantic-width conversion is iterative to avoid recursion proportional
//! to expression depth. Callers can provide an explicit node budget.
//! Budget exhaustion is an incomplete conversion, not successful conversion
//! and not proof that the source design is invalid.
//!
//! No finite implementation can process literally infinite input. Practical
//! limits arise from available resources, representable values, supported
//! semantics, and explicitly configured operational budgets.
//!
//! ## Integration contract
//!
//! - `types/mod.rs` must register this module and export its intended API.
//! - `types/bit_width.rs` remains the sole owner of HDLIR bit widths.
//! - `lowering/from_semantic.rs` maps validated semantic types to this API.
//! - `verify/types.rs` uses compatibility results without treating an
//!   indeterminate symbolic comparison as proven compatibility.
//! - `values/constants.rs` and `values/expressions.rs` use this descriptor
//!   when checking the type of a constant or expression.
//! - `ir/ports.rs` and `ir/operations.rs` use it for port and operation
//!   type contracts where the relevant dialect requires logic types.
//! - Serialization preserves width expressions, signedness, and state model.
//! - Physical realization remains downstream of HDLIR.
//!
//! This file does not assume that future HDLIR builders, verifiers, or
//! serialization APIs already exist.
//!
//! ## Compatibility and freeze criteria
//!
//! - Existing semantic types remain authoritative for source meaning.
//! - Width conversion preserves known, symbolic, unresolved, additive,
//!   and multiplicative width expressions.
//! - Zero widths and unsupported source types are rejected explicitly.
//! - Symbolic inequality is never treated as proof of incompatibility or
//!   proof of compatibility without sufficient evidence.
//! - No silent two-state/four-state coercion occurs.
//! - All public items are documented and covered by focused tests.
//! - Formatting, compilation, tests, and Clippy pass after integration.

use core::fmt;

use crate::frontend::semantic::domains::hdl::{
    HdlSignedness, HdlValueType, HdlWidth,
};

use super::bit_width::{
    BitWidth, WidthEvaluationError, WidthSymbol,
};

/// The logic-state model permitted by an HDLIR logic type.
///
/// This is a type-level contract, not a runtime logic value. It describes
/// which states the represented type can express. Context-specific legality
/// remains the responsibility of semantic validation and dialect verification.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LogicStateModel {
    /// The type represents only known logical zero and logical one.
    TwoState,

    /// The type can represent logical zero, logical one, unknown, and
    /// high-impedance states where the applicable semantics permit them.
    FourState,
}

impl LogicStateModel {
    /// Return whether this model can represent an unknown logic state.
    #[must_use]
    pub const fn supports_unknown(self) -> bool {
        matches!(self, Self::FourState)
    }

    /// Return whether this model can represent a high-impedance state.
    #[must_use]
    pub const fn supports_high_impedance(self) -> bool {
        matches!(self, Self::FourState)
    }
}

/// Signedness interpretation of an HDLIR logic vector.
///
/// Signedness affects the interpretation of a bit pattern, not its width.
/// Source-level values such as `HdlSignedness::NotApplicable` must not be
/// silently reinterpreted as signed or unsigned logic vectors.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LogicSignedness {
    /// Interpret the bit pattern as an unsigned value.
    Unsigned,

    /// Interpret the bit pattern as a signed value under the applicable
    /// HDL integer interpretation rules.
    Signed,
}

/// A target-independent HDLIR logic type.
///
/// Widths use the canonical HDLIR [`BitWidth`] representation. Symbolic
/// widths are not evaluated during construction.
///
/// A `LogicType` describes a type; it does not contain the logic value
/// currently carried by a signal or computed by an operation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LogicType {
    width: BitWidth,
    signedness: LogicSignedness,
    state_model: LogicStateModel,
}

/// Reasons a semantic HDL type cannot be converted into an HDLIR logic type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LogicTypeError {
    /// A source width was zero, which is invalid for this logic type.
    ZeroWidth,

    /// The source semantic type is not an ordinary bit or bit-vector type.
    UnsupportedSemanticType,

    /// The source type has no applicable signedness for a logic vector.
    NotApplicableSignedness,

    /// Conversion of a source symbolic width failed.
    WidthConversion(WidthEvaluationError),

    /// The width conversion exceeded its caller-supplied work budget.
    ///
    /// This reports incomplete conversion, not invalid source semantics.
    WidthConversionBudgetExceeded {
        /// Maximum number of width-expression nodes permitted.
        limit: usize,

        /// Number of nodes reached when the budget was exceeded.
        visited: usize,
    },

    /// The types have different concrete widths.
    WidthMismatch {
        /// Required width expression.
        expected: BitWidth,

        /// Actual width expression.
        actual: BitWidth,
    },

    /// The types use different signedness interpretations.
    SignednessMismatch {
        /// Required signedness.
        expected: LogicSignedness,

        /// Actual signedness.
        actual: LogicSignedness,
    },

    /// The types permit different logic-state models.
    StateModelMismatch {
        /// Required logic-state model.
        expected: LogicStateModel,

        /// Actual logic-state model.
        actual: LogicStateModel,
    },

    /// Available symbolic width information cannot prove exact compatibility.
    IndeterminateWidthCompatibility {
        /// Required width expression.
        expected: BitWidth,

        /// Actual width expression.
        actual: BitWidth,
    },
}

impl fmt::Display for LogicTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroWidth => {
                formatter.write_str("HDLIR logic width must be positive")
            }
            Self::UnsupportedSemanticType => formatter.write_str(
                "semantic type is not an ordinary HDL bit or bit vector",
            ),
            Self::NotApplicableSignedness => formatter.write_str(
                "source type has no applicable logic-vector signedness",
            ),
            Self::WidthConversion(error) => {
                write!(formatter, "cannot convert HDLIR logic width: {error}")
            }
            Self::WidthConversionBudgetExceeded { limit, visited } => {
                write!(
                    formatter,
                    "logic-width conversion budget exceeded: \
                     limit {limit} nodes, visited {visited}"
                )
            }
            Self::WidthMismatch { expected, actual } => {
                write!(
                    formatter,
                    "HDLIR logic width mismatch: expected {expected:?}, \
                     found {actual:?}"
                )
            }
            Self::SignednessMismatch { expected, actual } => {
                write!(
                    formatter,
                    "HDLIR logic signedness mismatch: expected {expected:?}, \
                     found {actual:?}"
                )
            }
            Self::StateModelMismatch { expected, actual } => {
                write!(
                    formatter,
                    "HDLIR logic-state model mismatch: expected {expected:?}, \
                     found {actual:?}"
                )
            }
            Self::IndeterminateWidthCompatibility { expected, actual } => {
                write!(
                    formatter,
                    "HDLIR logic width compatibility is indeterminate: \
                     expected {expected:?}, found {actual:?}"
                )
            }
        }
    }
}

impl std::error::Error for LogicTypeError {}

/// Result of conservatively comparing two HDLIR logic types.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogicTypeCompatibility {
    /// Available type information proves exact compatibility.
    Compatible,

    /// Available type information proves incompatibility.
    Incompatible,

    /// Symbolic widths differ structurally and cannot be proven equivalent
    /// or unequal from the information currently available.
    Indeterminate,
}

/// Operational budget for converting a source semantic width expression.
///
/// This budget counts source-width expression nodes visited during conversion.
/// It limits compiler work, not the width of a legal hardware design.
///
/// A budget of `usize::MAX` imposes no practical node-count limit beyond
/// representability and available resources.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WidthConversionBudget {
    /// Maximum number of source-width expression nodes to visit.
    pub max_nodes: usize,
}

impl WidthConversionBudget {
    /// Create a caller-selected width-conversion work budget.
    #[must_use]
    pub const fn new(max_nodes: usize) -> Self {
        Self { max_nodes }
    }

    /// Create a budget limited by the representable node count and available
    /// resources rather than an arbitrary configured node ceiling.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_nodes: usize::MAX,
        }
    }
}

impl Default for WidthConversionBudget {
    fn default() -> Self {
        Self::unbounded()
    }
}

impl LogicType {
    /// Construct a logic type after checking its width representation.
    ///
    /// The width is retained exactly as supplied. No symbolic evaluation,
    /// target selection, or physical allocation occurs.
    pub fn new(
        width: BitWidth,
        signedness: LogicSignedness,
        state_model: LogicStateModel,
    ) -> Result<Self, LogicTypeError> {
        validate_width(&width)?;

        Ok(Self {
            width,
            signedness,
            state_model,
        })
    }

    /// Construct an unsigned, one-bit, two-state logic type.
    ///
    /// The one-bit width is part of the meaning of this constructor, not a
    /// maximum width or a target-specific assumption.
    #[must_use]
    pub fn bit() -> Self {
        Self {
            width: BitWidth::Known(
                core::num::NonZeroU128::new(1)
                    .expect("one is non-zero"),
            ),
            signedness: LogicSignedness::Unsigned,
            state_model: LogicStateModel::TwoState,
        }
    }

    /// Construct an unsigned one-bit logic type with an explicit state model.
    #[must_use]
    pub fn bit_with_state_model(state_model: LogicStateModel) -> Self {
        Self {
            state_model,
            ..Self::bit()
        }
    }

    /// Construct a logic vector using a symbolic or concrete width.
    pub fn vector(
        width: BitWidth,
        signedness: LogicSignedness,
        state_model: LogicStateModel,
    ) -> Result<Self, LogicTypeError> {
        Self::new(width, signedness, state_model)
    }

    /// Convert an existing validated semantic HDL value type into HDLIR.
    ///
    /// This convenience method uses an unbounded operational node budget.
    /// Applications processing untrusted or extremely large expressions
    /// should use [`Self::from_semantic_type_with_budget`].
    ///
    /// The selected state model must come from the applicable source-language
    /// semantics or an explicit import policy. It is never inferred from
    /// a target device.
    pub fn from_semantic_type(
        value_type: &HdlValueType,
        state_model: LogicStateModel,
    ) -> Result<Self, LogicTypeError> {
        Self::from_semantic_type_with_budget(
            value_type,
            state_model,
            WidthConversionBudget::unbounded(),
        )
    }

    /// Convert a semantic HDL value type with an explicit width-node budget.
    ///
    /// `Bit` becomes an unsigned one-bit logic type. `BitVector` preserves
    /// its width expression and signedness. Clock, reset, and named types
    /// are rejected instead of being silently reinterpreted as data vectors.
    pub fn from_semantic_type_with_budget(
        value_type: &HdlValueType,
        state_model: LogicStateModel,
        budget: WidthConversionBudget,
    ) -> Result<Self, LogicTypeError> {
        match value_type {
            HdlValueType::Bit => Ok(Self::bit_with_state_model(state_model)),

            HdlValueType::BitVector {
                width,
                signedness,
            } => {
                let width = convert_semantic_width(width, budget)?;
                let signedness = match signedness {
                    HdlSignedness::Unsigned => LogicSignedness::Unsigned,
                    HdlSignedness::Signed => LogicSignedness::Signed,
                    HdlSignedness::NotApplicable => {
                        return Err(LogicTypeError::NotApplicableSignedness);
                    }
                };

                Self::new(width, signedness, state_model)
            }

            HdlValueType::Clock
            | HdlValueType::Reset
            | HdlValueType::Named(_) => {
                Err(LogicTypeError::UnsupportedSemanticType)
            }
        }
    }

    /// Return the width expression without evaluating it.
    #[must_use]
    pub const fn width(&self) -> &BitWidth {
        &self.width
    }

    /// Return the signedness interpretation.
    #[must_use]
    pub const fn signedness(&self) -> LogicSignedness {
        self.signedness
    }

    /// Return the permitted logic-state model.
    #[must_use]
    pub const fn state_model(&self) -> LogicStateModel {
        self.state_model
    }

    /// Return whether this is a concrete one-bit type.
    #[must_use]
    pub fn is_concrete_bit(&self) -> bool {
        self.width.as_known() == Some(1)
    }

    /// Return whether the width contains no symbolic or unresolved nodes.
    ///
    /// This is a structural query. It does not guarantee that evaluating
    /// a concrete arithmetic expression will fit in `u128`.
    #[must_use]
    pub fn has_fully_concrete_width(&self) -> bool {
        self.width.is_fully_concrete()
    }

    /// Return whether values of this type may represent an unknown state.
    #[must_use]
    pub const fn supports_unknown(&self) -> bool {
        self.state_model.supports_unknown()
    }

    /// Return whether values of this type may represent high impedance.
    #[must_use]
    pub const fn supports_high_impedance(&self) -> bool {
        self.state_model.supports_high_impedance()
    }

    /// Compare this type with another type using only available type facts.
    ///
    /// Different signedness or state models prove incompatibility.
    /// Equal symbolic width expressions are compatible by structural
    /// identity. Different symbolic expressions are indeterminate unless
    /// a separate analysis resolves or proves their relationship.
    #[must_use]
    pub fn compatibility_with(
        &self,
        other: &Self,
    ) -> LogicTypeCompatibility {
        if self.signedness != other.signedness
            || self.state_model != other.state_model
        {
            return LogicTypeCompatibility::Incompatible;
        }

        match (&self.width, &other.width) {
            (left, right) if left == right => {
                LogicTypeCompatibility::Compatible
            }
            (left, right) => match (left.as_known(), right.as_known()) {
                (Some(left), Some(right)) if left != right => {
                    LogicTypeCompatibility::Incompatible
                }
                _ => LogicTypeCompatibility::Indeterminate,
            },
        }
    }

    /// Require proven exact type compatibility.
    ///
    /// Indeterminate symbolic width compatibility is returned as an error,
    /// not treated as successful verification.
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

        if self.state_model != other.state_model {
            return Err(LogicTypeError::StateModelMismatch {
                expected: self.state_model,
                actual: other.state_model,
            });
        }

        match (&self.width, &other.width) {
            (expected, actual) if expected == actual => Ok(()),

            (expected, actual) => match (
                expected.as_known(),
                actual.as_known(),
            ) {
                (Some(_), Some(_)) => Err(LogicTypeError::WidthMismatch {
                    expected: expected.clone(),
                    actual: actual.clone(),
                }),

                _ => Err(LogicTypeError::IndeterminateWidthCompatibility {
                    expected: expected.clone(),
                    actual: actual.clone(),
                }),
            },
        }
    }
}

/// Validate a constructed HDLIR width expression.
///
/// The `BitWidth` constructors enforce positive concrete widths. This
/// iterative walk additionally ensures the invariant holds for compound
/// expressions without recursively consuming the Rust call stack.
fn validate_width(width: &BitWidth) -> Result<(), LogicTypeError> {
    let mut pending = vec![width];

    while let Some(current) = pending.pop() {
        match current {
            BitWidth::Known(value) if value.get() == 0 => {
                return Err(LogicTypeError::ZeroWidth);
            }

            BitWidth::Known(_) | BitWidth::Symbol(_) | BitWidth::Unresolved(_) => {}

            BitWidth::Add(left, right)
            | BitWidth::Multiply(left, right) => {
                pending.push(right);
                pending.push(left);
            }
        }
    }

    Ok(())
}

/// Convert a source semantic width into the canonical HDLIR width type.
///
/// The conversion is iterative and preserves every existing `HdlWidth`
/// variant. The node budget is an operational safeguard, not a hardware
/// capacity limit. No arithmetic is evaluated here.
fn convert_semantic_width(
    width: &HdlWidth,
    budget: WidthConversionBudget,
) -> Result<BitWidth, LogicTypeError> {
    let mut pending: Vec<(&HdlWidth, bool)> = vec![(width, false)];
    let mut converted: Vec<BitWidth> = Vec::new();
    let mut visited = 0_usize;

    while let Some((node, operands_ready)) = pending.pop() {
        if !operands_ready {
            visited = visited.checked_add(1).ok_or(
                LogicTypeError::WidthConversionBudgetExceeded {
                    limit: budget.max_nodes,
                    visited: usize::MAX,
                },
            )?;

            if visited > budget.max_nodes {
                return Err(
                    LogicTypeError::WidthConversionBudgetExceeded {
                        limit: budget.max_nodes,
                        visited,
                    },
                );
            }

            match node {
                HdlWidth::Known(value) => {
                    if *value == 0 {
                        return Err(LogicTypeError::ZeroWidth);
                    }

                    converted.push(
                        BitWidth::known(*value)
                            .map_err(LogicTypeError::WidthConversion)?,
                    );
                }

                HdlWidth::Symbol(symbol) => {
                    let symbol = WidthSymbol::new(symbol.as_str())
                        .map_err(LogicTypeError::WidthConversion)?;

                    converted.push(BitWidth::Symbol(symbol));
                }

                HdlWidth::Unresolved(symbol) => {
                    let symbol = WidthSymbol::new(symbol.as_str())
                        .map_err(LogicTypeError::WidthConversion)?;

                    converted.push(BitWidth::Unresolved(symbol));
                }

                HdlWidth::Add(left, right)
                | HdlWidth::Multiply(left, right) => {
                    pending.push((node, true));
                    pending.push((right, false));
                    pending.push((left, false));
                }
            }
        } else {
            let right = converted.pop().ok_or_else(|| {
                LogicTypeError::WidthConversion(
                    WidthEvaluationError::InvalidEvaluationState,
                )
            })?;

            let left = converted.pop().ok_or_else(|| {
                LogicTypeError::WidthConversion(
                    WidthEvaluationError::InvalidEvaluationState,
                )
            })?;

            let result = match node {
                HdlWidth::Add(_, _) => BitWidth::add(left, right),
                HdlWidth::Multiply(_, _) => BitWidth::multiply(left, right),

                HdlWidth::Known(_)
                | HdlWidth::Symbol(_)
                | HdlWidth::Unresolved(_) => {
                    return Err(LogicTypeError::WidthConversion(
                        WidthEvaluationError::InvalidEvaluationState,
                    ));
                }
            };

            converted.push(result);
        }
    }

    if converted.len() != 1 {
        return Err(LogicTypeError::WidthConversion(
            WidthEvaluationError::InvalidEvaluationState,
        ));
    }

    converted.pop().ok_or_else(|| {
        LogicTypeError::WidthConversion(
            WidthEvaluationError::InvalidEvaluationState,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::HdlId;

    fn source_id(name: &str) -> HdlId {
        HdlId::new(name).expect("test identifier must be valid")
    }

    fn vector(
        width: BitWidth,
        signedness: LogicSignedness,
        state_model: LogicStateModel,
    ) -> LogicType {
        LogicType::vector(width, signedness, state_model)
            .expect("test logic type must be valid")
    }

    #[test]
    fn bit_has_one_bit_unsigned_two_state_type() {
        let ty = LogicType::bit();

        assert!(ty.is_concrete_bit());
        assert_eq!(ty.width().as_known(), Some(1));
        assert_eq!(ty.signedness(), LogicSignedness::Unsigned);
        assert_eq!(ty.state_model(), LogicStateModel::TwoState);
        assert!(!ty.supports_unknown());
        assert!(!ty.supports_high_impedance());
    }

    #[test]
    fn four_state_bit_preserves_its_state_model() {
        let ty = LogicType::bit_with_state_model(
            LogicStateModel::FourState,
        );

        assert!(ty.is_concrete_bit());
        assert!(ty.supports_unknown());
        assert!(ty.supports_high_impedance());
    }

    #[test]
    fn symbolic_width_is_preserved() {
        let width = BitWidth::symbol("BUS_WIDTH")
            .expect("symbol must be valid");

        let ty = vector(
            width.clone(),
            LogicSignedness::Unsigned,
            LogicStateModel::TwoState,
        );

        assert_eq!(ty.width(), &width);
        assert!(!ty.has_fully_concrete_width());
    }

    #[test]
    fn zero_width_is_rejected_during_semantic_conversion() {
        let source = HdlValueType::BitVector {
            width: HdlWidth::Known(0),
            signedness: HdlSignedness::Unsigned,
        };

        assert_eq!(
            LogicType::from_semantic_type(
                &source,
                LogicStateModel::TwoState,
            ),
            Err(LogicTypeError::ZeroWidth)
        );
    }

    #[test]
    fn semantic_bit_maps_to_unsigned_one_bit() {
        let ty = LogicType::from_semantic_type(
            &HdlValueType::Bit,
            LogicStateModel::FourState,
        )
        .expect("bit type should convert");

        assert!(ty.is_concrete_bit());
        assert_eq!(ty.signedness(), LogicSignedness::Unsigned);
        assert_eq!(ty.state_model(), LogicStateModel::FourState);
    }

    #[test]
    fn semantic_vector_preserves_signedness_and_symbolic_width() {
        let width = HdlWidth::Multiply(
            Box::new(HdlWidth::Symbol(source_id("LANES"))),
            Box::new(HdlWidth::Known(8)),
        );

        let source = HdlValueType::BitVector {
            width,
            signedness: HdlSignedness::Signed,
        };

        let ty = LogicType::from_semantic_type(
            &source,
            LogicStateModel::TwoState,
        )
        .expect("symbolic vector should convert");

        assert_eq!(ty.signedness(), LogicSignedness::Signed);
        assert_eq!(
            ty.width(),
            &BitWidth::multiply(
                BitWidth::symbol("LANES").expect("valid symbol"),
                BitWidth::known(8).expect("positive width"),
            )
        );
    }

    #[test]
    fn unresolved_width_remains_unresolved() {
        let source = HdlValueType::BitVector {
            width: HdlWidth::Unresolved(source_id("UNRESOLVED_WIDTH")),
            signedness: HdlSignedness::Unsigned,
        };

        let ty = LogicType::from_semantic_type(
            &source,
            LogicStateModel::TwoState,
        )
        .expect("unresolved width should be preserved");

        assert!(ty.width().is_unresolved());
    }

    #[test]
    fn clock_reset_and_named_types_are_not_coerced_to_logic_vectors() {
        for source in [
            HdlValueType::Clock,
            HdlValueType::Reset,
            HdlValueType::Named(source_id("CustomType")),
        ] {
            assert_eq!(
                LogicType::from_semantic_type(
                    &source,
                    LogicStateModel::TwoState,
                ),
                Err(LogicTypeError::UnsupportedSemanticType)
            );
        }
    }

    #[test]
    fn not_applicable_signedness_is_rejected() {
        let source = HdlValueType::BitVector {
            width: HdlWidth::Known(8),
            signedness: HdlSignedness::NotApplicable,
        };

        assert_eq!(
            LogicType::from_semantic_type(
                &source,
                LogicStateModel::TwoState,
            ),
            Err(LogicTypeError::NotApplicableSignedness)
        );
    }

    #[test]
    fn identical_types_are_compatible() {
        let left = vector(
            BitWidth::known(128).expect("positive width"),
            LogicSignedness::Unsigned,
            LogicStateModel::TwoState,
        );

        assert_eq!(
            left.compatibility_with(&left),
            LogicTypeCompatibility::Compatible
        );
        assert!(left.require_compatible_with(&left).is_ok());
    }

    #[test]
    fn different_concrete_widths_are_incompatible() {
        let left = vector(
            BitWidth::known(8).expect("positive width"),
            LogicSignedness::Unsigned,
            LogicStateModel::TwoState,
        );

        let right = vector(
            BitWidth::known(16).expect("positive width"),
            LogicSignedness::Unsigned,
            LogicStateModel::TwoState,
        );

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
    fn different_symbolic_widths_are_indeterminate() {
        let left = vector(
            BitWidth::symbol("WIDTH_A").expect("valid symbol"),
            LogicSignedness::Unsigned,
            LogicStateModel::TwoState,
        );

        let right = vector(
            BitWidth::symbol("WIDTH_B").expect("valid symbol"),
            LogicSignedness::Unsigned,
            LogicStateModel::TwoState,
        );

        assert_eq!(
            left.compatibility_with(&right),
            LogicTypeCompatibility::Indeterminate
        );

        assert!(matches!(
            left.require_compatible_with(&right),
            Err(LogicTypeError::IndeterminateWidthCompatibility { .. })
        ));
    }

    #[test]
    fn signedness_mismatch_is_incompatible() {
        let unsigned = vector(
            BitWidth::known(16).expect("positive width"),
            LogicSignedness::Unsigned,
            LogicStateModel::TwoState,
        );

        let signed = vector(
            BitWidth::known(16).expect("positive width"),
            LogicSignedness::Signed,
            LogicStateModel::TwoState,
        );

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
    fn state_model_mismatch_is_not_silently_coerced() {
        let two_state = LogicType::bit();
        let four_state = LogicType::bit_with_state_model(
            LogicStateModel::FourState,
        );

        assert_eq!(
            two_state.compatibility_with(&four_state),
            LogicTypeCompatibility::Incompatible
        );

        assert!(matches!(
            two_state.require_compatible_with(&four_state),
            Err(LogicTypeError::StateModelMismatch { .. })
        ));
    }

    #[test]
    fn width_conversion_budget_exhaustion_is_explicit() {
        let source = HdlValueType::BitVector {
            width: HdlWidth::Add(
                Box::new(HdlWidth::Known(4)),
                Box::new(HdlWidth::Known(4)),
            ),
            signedness: HdlSignedness::Unsigned,
        };

        let result = LogicType::from_semantic_type_with_budget(
            &source,
            LogicStateModel::TwoState,
            WidthConversionBudget::new(1),
        );

        assert!(matches!(
            result,
            Err(LogicTypeError::WidthConversionBudgetExceeded {
                limit: 1,
                ..
            })
        ));
    }

    #[test]
    fn exact_budget_succeeds_for_single_node_width() {
        let source = HdlValueType::BitVector {
            width: HdlWidth::Known(8),
            signedness: HdlSignedness::Unsigned,
        };

        let result = LogicType::from_semantic_type_with_budget(
            &source,
            LogicStateModel::TwoState,
            WidthConversionBudget::new(1),
        );

        assert!(result.is_ok());
    }
}
