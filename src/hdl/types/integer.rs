
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Integer Types
//!
//! Path: `src/hdl/types/integer.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define the target-independent type contract for fixed-width and
//! parameterized HDL integer values.
//!
//! An integer type consists of a positive concrete or symbolic bit width
//! and an explicit signed or unsigned interpretation. The type describes
//! meaning; it does not allocate storage or select a hardware implementation.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/hdl/signals.g4`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/types/logic.rs`
//! - `src/hdl/widths.rs`
//!
//! The grammar owns source syntax. The existing frontend semantic module
//! owns `HdlWidth`, `HdlSignedness`, and `HdlValueType`. This module reuses
//! those canonical contracts.
//!
//! ## Owns
//!
//! - Integer-type construction and structural validation.
//! - Explicit signedness and width accessors.
//! - Type compatibility and conservative width comparison.
//! - Classification of width changes for downstream conversion analysis.
//! - Adaptation from the canonical HDL semantic bit-vector type.
//!
//! ## Does not own
//!
//! - Parsing integer declarations or literals.
//! - A second AST, width-expression tree, or semantic type system.
//! - Integer constant storage or arbitrary-precision arithmetic.
//! - Evaluation of expressions or arithmetic overflow policies.
//! - Implicit conversion, truncation, extension, or saturation execution.
//! - Packed physical layouts, target ABIs, device allocation, or realization.
//! - HDLIR data structures, builders, passes, verification, or lowering.
//!
//! ## Scalability
//!
//! No universal integer-width ceiling is introduced here. Symbolic widths
//! remain symbolic until resolved by the canonical width subsystem.
//!
//! The current canonical `HdlWidth::Known` representation is `u128`.
//! This file preserves that existing contract rather than claiming that
//! larger concrete widths are representable by the current semantic model.
//! Supporting arbitrary-size concrete width values requires an explicit,
//! coordinated evolution of that canonical contract.
//!
//! Compilation and realization are necessarily constrained by available
//! memory, execution time, and configured operational budgets. Such limits
//! must not be confused with language-level integer semantics.
//!
//! ## Integration
//!
//! `src/hdl/types/mod.rs` must declare `pub mod integer;` and may explicitly
//! re-export `IntegerType`, `IntegerTypeError`, `IntegerSignedness`,
//! `IntegerTypeCompatibility`, and `IntegerWidthRelation`.
//!
//! `src/hdl/types/logic.rs` remains the owner of general digital logic
//! types. Integer types reuse its canonical width and signedness contracts;
//! they do not duplicate its logic-state representation.
//!
//! `src/hdl/widths.rs` owns symbolic-width resolution and checked arithmetic.
//! Expression, assignment, constant, layout, and interoperability modules
//! consume this type contract but must implement their own respective
//! behavior without changing its ownership boundary.
//!
//! ## Semantic policy
//!
//! This module does not prescribe overflow, truncation, rounding, or
//! extension behavior. Those are operation- and language-specific semantic
//! rules. A width change is classified here, not silently executed.
//!
//! ## Freeze criteria
//!
//! - Canonical semantic widths and signedness are reused.
//! - Non-integer signedness is rejected.
//! - Zero widths are rejected, including zero components in compound
//!   width expressions.
//! - Symbolic widths are preserved and never guessed.
//! - Incompatible and indeterminate comparisons are distinguished.
//! - No target-specific assumptions or HDLIR dependencies are introduced.
//! - Unit tests pass and the parent module registers this file.

use std::error::Error;
use std::fmt;

use crate::frontend::semantic::domains::hdl::{
    HdlSignedness, HdlValueType, HdlWidth,
};

/// The signedness of an HDL integer.
///
/// This intentionally excludes `HdlSignedness::NotApplicable`: an integer
/// type must have an explicit signed or unsigned interpretation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IntegerSignedness {
    /// Signed integer interpretation.
    ///
    /// The exact signed encoding and operation semantics are defined by the
    /// applicable language specification and operation, not by this enum.
    Signed,

    /// Unsigned integer interpretation.
    Unsigned,
}

impl IntegerSignedness {
    /// Convert the integer signedness into the canonical HDL signedness.
    #[must_use]
    pub const fn as_hdl_signedness(self) -> HdlSignedness {
        match self {
            Self::Signed => HdlSignedness::Signed,
            Self::Unsigned => HdlSignedness::Unsigned,
        }
    }

    /// Adapt an existing canonical HDL signedness.
    ///
    /// Returns `None` for `NotApplicable`, because that value cannot
    /// describe an integer type.
    #[must_use]
    pub const fn from_hdl_signedness(
        signedness: HdlSignedness,
    ) -> Option<Self> {
        match signedness {
            HdlSignedness::Signed => Some(Self::Signed),
            HdlSignedness::Unsigned => Some(Self::Unsigned),
            HdlSignedness::NotApplicable => None,
        }
    }
}

/// A target-independent HDL integer type.
///
/// The width is the canonical semantic `HdlWidth`; it may be concrete or
/// symbolic. Signedness is always explicit.
///
/// This structure does not contain an integer value and does not define
/// arithmetic overflow, conversion, or physical layout behavior.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IntegerType {
    width: HdlWidth,
    signedness: IntegerSignedness,
}

/// Errors produced while constructing or adapting an integer type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntegerTypeError {
    /// A concrete width is zero.
    ZeroWidth,

    /// A width expression contains a zero-width component.
    ZeroWidthComponent,

    /// The source type does not describe an integer bit vector.
    NotAnIntegerType,

    /// The canonical signedness is not applicable to integer values.
    SignednessNotApplicable,

    /// The available information proves that two widths differ.
    WidthMismatch {
        /// Expected width.
        expected: HdlWidth,

        /// Actual width.
        actual: HdlWidth,
    },

    /// Signedness differs between two types.
    SignednessMismatch {
        /// Expected signedness.
        expected: IntegerSignedness,

        /// Actual signedness.
        actual: IntegerSignedness,
    },

    /// Width compatibility cannot be proven without resolving symbolic
    /// expressions.
    WidthIndeterminate {
        /// Expected width.
        expected: HdlWidth,

        /// Actual width.
        actual: HdlWidth,
    },
}

impl fmt::Display for IntegerTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroWidth => {
                f.write_str("HDL integer width must be positive")
            }
            Self::ZeroWidthComponent => {
                f.write_str(
                    "HDL integer width expression contains a zero-width component",
                )
            }
            Self::NotAnIntegerType => {
                f.write_str(
                    "semantic type is not an HDL integer bit-vector type",
                )
            }
            Self::SignednessNotApplicable => {
                f.write_str(
                    "HDL integer type requires signed or unsigned interpretation",
                )
            }
            Self::WidthMismatch { expected, actual } => {
                write!(
                    f,
                    "HDL integer width mismatch: expected {expected:?}, \
                     found {actual:?}"
                )
            }
            Self::SignednessMismatch { expected, actual } => {
                write!(
                    f,
                    "HDL integer signedness mismatch: expected \
                     {expected:?}, found {actual:?}"
                )
            }
            Self::WidthIndeterminate { expected, actual } => {
                write!(
                    f,
                    "HDL integer width compatibility is indeterminate: \
                     expected {expected:?}, found {actual:?}"
                )
            }
        }
    }
}

impl Error for IntegerTypeError {}

/// Result of comparing two integer types using the available information.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegerTypeCompatibility {
    /// The types have provably equal widths and signedness.
    Compatible,

    /// The types are provably incompatible.
    Incompatible,

    /// Symbolic width information prevents a definitive comparison.
    Indeterminate,
}

/// Relationship between two integer widths.
///
/// This classification does not execute or authorize a conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegerWidthRelation {
    /// The widths are known to be equal.
    Equal,

    /// The destination width is known to be greater than the source width.
    DestinationWider,

    /// The destination width is known to be less than the source width.
    DestinationNarrower,

    /// The relationship cannot be determined from the current expressions.
    Indeterminate,
}

impl IntegerType {
    /// Construct an integer type after validating its width expression.
    ///
    /// Symbolic expressions are preserved and are not evaluated. A concrete
    /// zero width, or a zero-width component in a compound expression, is
    /// rejected.
    pub fn new(
        width: HdlWidth,
        signedness: IntegerSignedness,
    ) -> Result<Self, IntegerTypeError> {
        validate_width_expression(&width)?;

        Ok(Self { width, signedness })
    }

    /// Construct an unsigned integer with a concrete positive width.
    ///
    /// The supplied width is part of this type's declaration, not a
    /// universal width limit.
    pub fn unsigned(width: u128) -> Result<Self, IntegerTypeError> {
        Self::new(HdlWidth::Known(width), IntegerSignedness::Unsigned)
    }

    /// Construct a signed integer with a concrete positive width.
    pub fn signed(width: u128) -> Result<Self, IntegerTypeError> {
        Self::new(HdlWidth::Known(width), IntegerSignedness::Signed)
    }

    /// Construct an unsigned integer with a symbolic width.
    pub fn symbolic_unsigned(
        width: HdlWidth,
    ) -> Result<Self, IntegerTypeError> {
        Self::new(width, IntegerSignedness::Unsigned)
    }

    /// Construct a signed integer with a symbolic width.
    pub fn symbolic_signed(
        width: HdlWidth,
    ) -> Result<Self, IntegerTypeError> {
        Self::new(width, IntegerSignedness::Signed)
    }

    /// Adapt a canonical HDL semantic bit-vector type.
    ///
    /// The width and signedness are preserved. `Bit`, `Clock`, `Reset`, and
    /// `Named` types are not silently reinterpreted as integer types.
    pub fn from_semantic_type(
        value_type: &HdlValueType,
    ) -> Result<Self, IntegerTypeError> {
        match value_type {
            HdlValueType::BitVector { width, signedness } => {
                let signedness =
                    IntegerSignedness::from_hdl_signedness(*signedness)
                        .ok_or(IntegerTypeError::SignednessNotApplicable)?;

                Self::new(width.clone(), signedness)
            }
            HdlValueType::Bit
            | HdlValueType::Clock
            | HdlValueType::Reset
            | HdlValueType::Named(_) => {
                Err(IntegerTypeError::NotAnIntegerType)
            }
        }
    }

    /// Return the canonical width expression.
    #[must_use]
    pub fn width(&self) -> &HdlWidth {
        &self.width
    }

    /// Return the integer signedness.
    #[must_use]
    pub const fn signedness(&self) -> IntegerSignedness {
        self.signedness
    }

    /// Return the canonical HDL signedness.
    #[must_use]
    pub const fn hdl_signedness(&self) -> HdlSignedness {
        self.signedness.as_hdl_signedness()
    }

    /// Return the concrete width when the width is a single known literal.
    ///
    /// Compound or symbolic expressions are not evaluated here.
    #[must_use]
    pub const fn known_width(&self) -> Option<u128> {
        match self.width {
            HdlWidth::Known(width) => Some(width),
            _ => None,
        }
    }

    /// Return whether this type has a directly known concrete width.
    #[must_use]
    pub const fn has_known_width(&self) -> bool {
        matches!(self.width, HdlWidth::Known(_))
    }

    /// Return whether the type uses signed interpretation.
    #[must_use]
    pub const fn is_signed(&self) -> bool {
        matches!(self.signedness, IntegerSignedness::Signed)
    }

    /// Return whether the type uses unsigned interpretation.
    #[must_use]
    pub const fn is_unsigned(&self) -> bool {
        matches!(self.signedness, IntegerSignedness::Unsigned)
    }

    /// Compare this type with another type conservatively.
    ///
    /// Different signedness is incompatible. Known widths are compared
    /// directly. Identical symbolic width expressions are structurally
    /// compatible, but different symbolic expressions are not assumed
    /// equivalent or different in their eventual numeric values.
    #[must_use]
    pub fn compatibility_with(
        &self,
        other: &Self,
    ) -> IntegerTypeCompatibility {
        if self.signedness != other.signedness {
            return IntegerTypeCompatibility::Incompatible;
        }

        match (&self.width, &other.width) {
            (HdlWidth::Known(left), HdlWidth::Known(right)) => {
                if left == right {
                    IntegerTypeCompatibility::Compatible
                } else {
                    IntegerTypeCompatibility::Incompatible
                }
            }
            (left, right) if left == right => {
                IntegerTypeCompatibility::Compatible
            }
            _ => IntegerTypeCompatibility::Indeterminate,
        }
    }

    /// Require provable exact type compatibility.
    ///
    /// Returns a width or signedness mismatch when incompatibility is known.
    /// Returns `WidthIndeterminate` when symbolic widths differ structurally
    /// and cannot be compared without resolving them.
    pub fn require_compatible_with(
        &self,
        other: &Self,
    ) -> Result<(), IntegerTypeError> {
        if self.signedness != other.signedness {
            return Err(IntegerTypeError::SignednessMismatch {
                expected: self.signedness,
                actual: other.signedness,
            });
        }

        match (&self.width, &other.width) {
            (HdlWidth::Known(left), HdlWidth::Known(right))
                if left != right =>
            {
                Err(IntegerTypeError::WidthMismatch {
                    expected: self.width.clone(),
                    actual: other.width.clone(),
                })
            }
            (left, right) if left != right => {
                Err(IntegerTypeError::WidthIndeterminate {
                    expected: self.width.clone(),
                    actual: other.width.clone(),
                })
            }
            _ => Ok(()),
        }
    }

    /// Classify the width relationship when this type is the destination.
    ///
    /// This method does not decide whether a narrowing conversion is legal,
    /// whether extension preserves a value, or which overflow policy applies.
    /// Those decisions belong to the language's conversion semantics.
    #[must_use]
    pub fn width_relation_from(
        &self,
        source: &Self,
    ) -> IntegerWidthRelation {
        match (&self.width, &source.width) {
            (HdlWidth::Known(destination), HdlWidth::Known(source)) => {
                match destination.cmp(source) {
                    std::cmp::Ordering::Less => {
                        IntegerWidthRelation::DestinationNarrower
                    }
                    std::cmp::Ordering::Equal => IntegerWidthRelation::Equal,
                    std::cmp::Ordering::Greater => {
                        IntegerWidthRelation::DestinationWider
                    }
                }
            }
            (destination, source) if destination == source => {
                IntegerWidthRelation::Equal
            }
            _ => IntegerWidthRelation::Indeterminate,
        }
    }
}

/// Validate a canonical width expression without recursively traversing it.
///
/// The explicit stack avoids consuming the Rust call stack merely because a
/// width expression is deeply nested. Allocation and execution remain subject
/// to available resources and any caller-configured operational budget.
fn validate_width_expression(
    width: &HdlWidth,
) -> Result<(), IntegerTypeError> {
    let mut pending = vec![width];

    while let Some(current) = pending.pop() {
        match current {
            HdlWidth::Known(0) => {
                return Err(IntegerTypeError::ZeroWidth);
            }
            HdlWidth::Known(_) | HdlWidth::Symbol(_) | HdlWidth::Unresolved(_) => {}
            HdlWidth::Add(left, right)
            | HdlWidth::Multiply(left, right) => {
                if matches!(left.as_ref(), HdlWidth::Known(0))
                    || matches!(right.as_ref(), HdlWidth::Known(0))
                {
                    return Err(IntegerTypeError::ZeroWidthComponent);
                }

                pending.push(right);
                pending.push(left);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::HdlId;

    fn id(name: &str) -> HdlId {
        HdlId::new(name).expect("test identifier must be non-empty")
    }

    fn symbolic(name: &str) -> HdlWidth {
        HdlWidth::Symbol(id(name))
    }

    #[test]
    fn constructs_signed_and_unsigned_integer_types() {
        let signed = IntegerType::signed(32).expect("positive width");
        let unsigned = IntegerType::unsigned(32).expect("positive width");

        assert!(signed.is_signed());
        assert!(!signed.is_unsigned());
        assert!(unsigned.is_unsigned());
        assert!(!unsigned.is_signed());
        assert_eq!(signed.known_width(), Some(32));
        assert_eq!(unsigned.known_width(), Some(32));
    }

    #[test]
    fn rejects_zero_width() {
        assert_eq!(
            IntegerType::unsigned(0),
            Err(IntegerTypeError::ZeroWidth)
        );
    }

    #[test]
    fn rejects_zero_width_in_compound_expression() {
        let width = HdlWidth::Add(
            Box::new(HdlWidth::Known(8)),
            Box::new(HdlWidth::Known(0)),
        );

        assert_eq!(
            IntegerType::new(width, IntegerSignedness::Unsigned),
            Err(IntegerTypeError::ZeroWidthComponent)
        );
    }

    #[test]
    fn preserves_symbolic_widths() {
        let width = symbolic("DATA_WIDTH");
        let ty = IntegerType::symbolic_unsigned(width.clone())
            .expect("symbolic width is valid");

        assert_eq!(ty.width(), &width);
        assert_eq!(ty.known_width(), None);
        assert!(!ty.has_known_width());
    }

    #[test]
    fn supports_symbolic_compound_widths() {
        let width = HdlWidth::Multiply(
            Box::new(symbolic("LANES")),
            Box::new(HdlWidth::Known(8)),
        );

        let ty = IntegerType::symbolic_signed(width.clone())
            .expect("symbolic compound width is valid");

        assert_eq!(ty.width(), &width);
        assert!(ty.is_signed());
    }

    #[test]
    fn rejects_not_applicable_signedness() {
        assert_eq!(
            IntegerType::from_semantic_type(
                &HdlValueType::BitVector {
                    width: HdlWidth::Known(8),
                    signedness: HdlSignedness::NotApplicable,
                }
            ),
            Err(IntegerTypeError::SignednessNotApplicable)
        );
    }

    #[test]
    fn adapts_canonical_signed_bit_vector() {
        let ty = IntegerType::from_semantic_type(
            &HdlValueType::BitVector {
                width: HdlWidth::Known(128),
                signedness: HdlSignedness::Signed,
            },
        )
        .expect("valid signed bit vector");

        assert_eq!(ty.known_width(), Some(128));
        assert_eq!(ty.signedness(), IntegerSignedness::Signed);
    }

    #[test]
    fn rejects_non_integer_semantic_types() {
        assert_eq!(
            IntegerType::from_semantic_type(&HdlValueType::Bit),
            Err(IntegerTypeError::NotAnIntegerType)
        );

        assert_eq!(
            IntegerType::from_semantic_type(&HdlValueType::Clock),
            Err(IntegerTypeError::NotAnIntegerType)
        );
    }

    #[test]
    fn identical_types_are_compatible() {
        let left = IntegerType::symbolic_unsigned(symbol("WIDTH"))
            .expect("valid type");
        let right = left.clone();

        assert_eq!(
            left.compatibility_with(&right),
            IntegerTypeCompatibility::Compatible
        );
        assert!(left.require_compatible_with(&right).is_ok());
    }

    #[test]
    fn different_known_widths_are_incompatible() {
        let left = IntegerType::unsigned(8).expect("valid type");
        let right = IntegerType::unsigned(16).expect("valid type");

        assert_eq!(
            left.compatibility_with(&right),
            IntegerTypeCompatibility::Incompatible
        );

        assert!(matches!(
            left.require_compatible_with(&right),
            Err(IntegerTypeError::WidthMismatch { .. })
        ));
    }

    #[test]
    fn symbolic_width_comparison_is_indeterminate() {
        let left = IntegerType::symbolic_unsigned(symbol("WIDTH_A"))
            .expect("valid type");
        let right = IntegerType::symbolic_unsigned(symbol("WIDTH_B"))
            .expect("valid type");

        assert_eq!(
            left.compatibility_with(&right),
            IntegerTypeCompatibility::Indeterminate
        );

        assert!(matches!(
            left.require_compatible_with(&right),
            Err(IntegerTypeError::WidthIndeterminate { .. })
        ));
    }

    #[test]
    fn signedness_mismatch_is_incompatible() {
        let signed = IntegerType::signed(16).expect("valid type");
        let unsigned = IntegerType::unsigned(16).expect("valid type");

        assert_eq!(
            signed.compatibility_with(&unsigned),
            IntegerTypeCompatibility::Incompatible
        );

        assert!(matches!(
            signed.require_compatible_with(&unsigned),
            Err(IntegerTypeError::SignednessMismatch { .. })
        ));
    }

    #[test]
    fn classifies_concrete_width_relationships() {
        let narrow = IntegerType::unsigned(8).expect("valid type");
        let wide = IntegerType::unsigned(64).expect("valid type");

        assert_eq!(
            wide.width_relation_from(&narrow),
            IntegerWidthRelation::DestinationWider
        );
        assert_eq!(
            narrow.width_relation_from(&wide),
            IntegerWidthRelation::DestinationNarrower
        );
        assert_eq!(
            narrow.width_relation_from(&narrow),
            IntegerWidthRelation::Equal
        );
    }

    #[test]
    fn classifies_symbolic_width_relationships_conservatively() {
        let destination = IntegerType::symbolic_unsigned(symbol("OUT_WIDTH"))
            .expect("valid type");
        let source = IntegerType::symbolic_unsigned(symbol("IN_WIDTH"))
            .expect("valid type");

        assert_eq!(
            destination.width_relation_from(&source),
            IntegerWidthRelation::Indeterminate
        );
    }

    #[test]
    fn width_relation_does_not_conflate_signedness_with_width() {
        let signed = IntegerType::signed(32).expect("valid type");
        let unsigned = IntegerType::unsigned(32).expect("valid type");

        assert_eq!(
            signed.width_relation_from(&unsigned),
            IntegerWidthRelation::Equal
        );

        assert_eq!(
            signed.compatibility_with(&unsigned),
            IntegerTypeCompatibility::Incompatible
        );
    }

    #[test]
    fn concrete_width_boundaries_are_preserved() {
        let ty = IntegerType::unsigned(u128::MAX)
            .expect("canonical width accepts u128::MAX");

        assert_eq!(ty.known_width(), Some(u128::MAX));
    }

    #[test]
    fn width_expression_validation_is_deterministic() {
        let width = HdlWidth::Add(
            Box::new(symbol("A")),
            Box::new(HdlWidth::Multiply(
                Box::new(symbol("B")),
                Box::new(HdlWidth::Known(8)),
            )),
        );

        let first = IntegerType::new(width.clone(), IntegerSignedness::Signed);
        let second = IntegerType::new(width, IntegerSignedness::Signed);

        assert_eq!(first, second);
    }
}
