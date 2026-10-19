
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Logical Layout
//!
//! Path: `src/hdl/types/layout.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provide an HDL-specific adapter for logical bit extents and structural
//! layout facts. This module connects the canonical HDL width model, packed
//! layouts, and canonical symbolic array dimensions without creating another
//! universal semantic type system.
//!
//! ## Authority
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/type-system.md`
//! - `grammar/spec/user-defined-types.md`
//! - `grammar/hdl/hdl.g4` and its delegate grammars
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/types/packed.rs`
//! - `src/hdl/types/aggregate.rs`
//! - `src/zuir/types/layout.rs`
//! - `src/zuir/types/type_def.rs`
//!
//! The grammar owns source syntax. The frontend semantic layer owns canonical
//! HDL width expressions. ZUIR owns universal semantic type definitions and
//! general logical type descriptions. `packed.rs` owns packed-field offsets.
//! This module composes those contracts for HDL-specific consumers.
//!
//! ## Owns
//!
//! - A logical extent expression that can preserve concrete, symbolic, and
//!   dynamic sizes.
//! - Checked addition and multiplication of logical extents.
//! - HDL layout facts derived from bit widths, packed records, and array
//!   dimensions.
//! - Explicit representation of layouts whose bit extent is unavailable or
//!   not applicable.
//!
//! ## Does not own
//!
//! - Source parsing, AST construction, name resolution, or type inference.
//! - A replacement for `HdlWidth`, `Dimension`, `TypeDef`, or ZUIR's
//!   `LogicalLayout`.
//! - Physical byte size, ABI, byte order, padding, alignment, or addresses.
//! - Hardware allocation, synthesis, placement, routing, or timing closure.
//! - Target capability discovery or feasibility decisions.
//! - HDLIR. All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Scalability
//!
//! There is no arbitrary language-level limit on dimensions, layout nodes, or
//! aggregate sizes. Symbolic values remain symbolic. Arithmetic involving
//! concrete `u128` values is checked and never wraps or truncates.
//!
//! The canonical `HdlWidth::Known(u128)` currently bounds concrete HDL widths
//! representable by that upstream contract. This module cannot remove that
//! boundary independently. Rust collections and the compiler process remain
//! finite; callers must apply explicit operational budgets where appropriate.
//!
//! ## Integration
//!
//! - `src/hdl/types/mod.rs` must register this module as `pub mod layout;`.
//! - `bit_vector` consumes the canonical `HdlWidth`.
//! - `packed_record` consumes the existing `PackedLayout`.
//! - `array` consumes canonical `Dimension` without converting it to `usize`.
//! - Canonical type resolution remains upstream and supplies valid resolved
//!   widths and dimensions.
//! - Target-specific layout requires a separate, explicit target-layout
//!   contract; this module does not infer physical layout.
//!
//! ## Freeze criteria
//!
//! Freeze after registering the module and compiling against the complete
//! workspace. Tests must cover symbolic values, overflow, zero-sized array
//! dimensions, dynamic dimensions, packed layouts, and non-bit-addressable
//! types. The repository's grammar/specification remains authoritative if
//! any rule conflicts with this adapter.

use core::fmt;
use std::error::Error;

use crate::frontend::semantic::domains::hdl::{HdlId, HdlWidth};
use crate::hdl::types::packed::PackedLayout;
use crate::zuir::types::Dimension;

/// The logical category of an HDL layout description.
///
/// These categories describe semantic structure, not a target's physical
/// representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum HdlLayoutKind {
    /// A bit-vector or scalar represented by a canonical HDL width.
    BitVector,

    /// A record with an explicitly defined packed representation.
    PackedRecord,

    /// An array whose extent derives from an element and a canonical
    /// dimension.
    Array,

    /// A structural aggregate whose physical representation is not implied.
    Aggregate,

    /// An opaque type whose bit representation is not exposed here.
    Opaque,

    /// A layout whose size cannot currently be established.
    Unresolved,

    /// A type for which a logical bit extent is not applicable.
    NotBitAddressable,
}

/// A symbolic or concrete logical extent, measured in bits.
///
/// `known_value()` returns `Some` only when the extent is fully known and
/// fits the current canonical `u128` representation. `None` does not mean
/// unlimited size: it means the extent is symbolic, dynamic, or unresolved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LayoutExtent {
    expression: LayoutExtentExpr,
    known: Option<u128>,
}

/// The expression underlying a logical extent.
///
/// The variants preserve source-independent semantic inputs. This expression
/// is not a replacement for the canonical source AST or symbolic constraint
/// system.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum LayoutExtentExpr {
    /// The extent is exactly zero.
    Zero,

    /// A canonical HDL width expression.
    Width(HdlWidth),

    /// A canonical array dimension.
    Dimension(Dimension),

    /// A packed layout supplied by `src/hdl/types/packed.rs`.
    Packed(Box<PackedLayout>),

    /// The sum of two extents.
    Add(Box<LayoutExtentExpr>, Box<LayoutExtentExpr>),

    /// The product of two extents.
    Multiply(Box<LayoutExtentExpr>, Box<LayoutExtentExpr>),

    /// The extent depends on a value that is dynamic at runtime.
    Dynamic,

    /// The extent cannot yet be resolved.
    Unresolved(String),
}

impl LayoutExtent {
    /// Construct a zero extent.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            expression: LayoutExtentExpr::Zero,
            known: Some(0),
        }
    }

    /// Construct an extent from the canonical HDL width model.
    ///
    /// Width expressions containing a zero-width literal are rejected,
    /// consistent with the current `HdlWidth` contract.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid zero-width literal or concrete
    /// arithmetic overflow inside the width expression.
    pub fn from_width(width: HdlWidth) -> Result<Self, HdlLayoutError> {
        let known = evaluate_width(&width)?;
        Ok(Self {
            expression: LayoutExtentExpr::Width(width),
            known,
        })
    }

    /// Construct an extent from a canonical array dimension.
    ///
    /// A known zero dimension is valid and produces a zero extent. Symbolic
    /// and dynamic dimensions remain symbolic and dynamic respectively.
    #[must_use]
    pub fn from_dimension(dimension: Dimension) -> Self {
        match dimension {
            Dimension::Known(value) => Self {
                expression: if value == 0 {
                    LayoutExtentExpr::Zero
                } else {
                    LayoutExtentExpr::Dimension(Dimension::Known(value))
                },
                known: Some(value),
            },
            symbolic @ Dimension::Symbolic(_) => Self {
                expression: LayoutExtentExpr::Dimension(symbolic),
                known: None,
            },
            Dimension::Dynamic => Self {
                expression: LayoutExtentExpr::Dynamic,
                known: None,
            },
        }
    }

    /// Construct an extent from an existing validated packed layout.
    ///
    /// The packed layout's field ordering and offsets remain owned by
    /// `PackedLayout`. This method does not reinterpret them.
    #[must_use]
    pub fn from_packed(layout: PackedLayout) -> Self {
        let known = layout.total_width().as_known();
        Self {
            expression: LayoutExtentExpr::Packed(Box::new(layout)),
            known,
        }
    }

    /// Construct an unresolved extent with an explanatory reason.
    ///
    /// This is not a successful resolution result. Callers must not treat it
    /// as zero, unlimited, or feasible for a target.
    pub fn unresolved(
        reason: impl Into<String>,
    ) -> Result<Self, HdlLayoutError> {
        let reason = reason.into();
        if reason.trim().is_empty() {
            return Err(HdlLayoutError::EmptyReason);
        }

        Ok(Self {
            expression: LayoutExtentExpr::Unresolved(reason),
            known: None,
        })
    }

    /// Return the concrete extent if it is known.
    #[must_use]
    pub const fn known_value(&self) -> Option<u128> {
        self.known
    }

    /// Return whether the extent is concretely known.
    #[must_use]
    pub const fn is_known(&self) -> bool {
        self.known.is_some()
    }

    /// Return the preserved extent expression.
    #[must_use]
    pub const fn expression(&self) -> &LayoutExtentExpr {
        &self.expression
    }

    /// Add another logical extent using checked arithmetic.
    ///
    /// If either operand is symbolic, dynamic, or unresolved, the expression
    /// is preserved and the result remains unknown. Concrete overflow is an
    /// error rather than wrapping or truncation.
    ///
    /// # Errors
    ///
    /// Returns [`HdlLayoutError::ArithmeticOverflow`] when both extents are
    /// concrete and their sum cannot be represented by `u128`.
    pub fn checked_add(&self, other: &Self) -> Result<Self, HdlLayoutError> {
        let known = match (self.known, other.known) {
            (Some(left), Some(right)) => Some(
                left.checked_add(right)
                    .ok_or(HdlLayoutError::ArithmeticOverflow)?,
            ),
            _ => None,
        };

        if self.known == Some(0) {
            return Ok(other.clone());
        }
        if other.known == Some(0) {
            return Ok(self.clone());
        }

        Ok(Self {
            expression: LayoutExtentExpr::Add(
                Box::new(self.expression.clone()),
                Box::new(other.expression.clone()),
            ),
            known,
        })
    }

    /// Multiply logical extents using checked arithmetic.
    ///
    /// This is useful for deriving an array's total bit extent from its
    /// element extent and dimension. It does not allocate an array.
    ///
    /// # Errors
    ///
    /// Returns [`HdlLayoutError::ArithmeticOverflow`] when both extents are
    /// concrete and their product cannot be represented by `u128`.
    pub fn checked_mul(&self, other: &Self) -> Result<Self, HdlLayoutError> {
        let known = match (self.known, other.known) {
            (Some(left), Some(right)) => Some(
                left.checked_mul(right)
                    .ok_or(HdlLayoutError::ArithmeticOverflow)?,
            ),
            _ => None,
        };

        if known == Some(0) {
            return Ok(Self::zero());
        }
        if self.known == Some(1) {
            return Ok(other.clone());
        }
        if other.known == Some(1) {
            return Ok(self.clone());
        }

        Ok(Self {
            expression: LayoutExtentExpr::Multiply(
                Box::new(self.expression.clone()),
                Box::new(other.expression.clone()),
            ),
            known,
        })
    }
}

impl fmt::Display for LayoutExtent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.known {
            Some(value) => write!(f, "{value} bits"),
            None => write!(f, "{:?}", self.expression),
        }
    }
}

/// An HDL-specific logical layout description.
///
/// `bit_extent` is absent when a bit extent is not applicable or is not
/// provided by the canonical type contract. A missing extent must never be
/// interpreted as zero.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HdlTypeLayout {
    kind: HdlLayoutKind,
    bit_extent: Option<LayoutExtent>,
}

impl HdlTypeLayout {
    /// Describe a bit-vector type using its canonical HDL width.
    ///
    /// # Errors
    ///
    /// Returns an error if the width contains an invalid zero-width literal
    /// or a concrete subexpression overflows.
    pub fn bit_vector(width: HdlWidth) -> Result<Self, HdlLayoutError> {
        Ok(Self {
            kind: HdlLayoutKind::BitVector,
            bit_extent: Some(LayoutExtent::from_width(width)?),
        })
    }

    /// Describe a packed record using its already validated packed layout.
    #[must_use]
    pub fn packed_record(layout: PackedLayout) -> Self {
        Self {
            kind: HdlLayoutKind::PackedRecord,
            bit_extent: Some(LayoutExtent::from_packed(layout)),
        }
    }

    /// Describe an array without allocating its elements.
    ///
    /// The array extent is `element_extent * dimension`. Symbolic dimensions
    /// remain symbolic; dynamic dimensions remain dynamic.
    ///
    /// # Errors
    ///
    /// Returns an error if both extents are concrete and their product
    /// overflows `u128`.
    pub fn array(
        element_extent: LayoutExtent,
        dimension: Dimension,
    ) -> Result<Self, HdlLayoutError> {
        let dimension_extent = LayoutExtent::from_dimension(dimension);
        let bit_extent = element_extent.checked_mul(&dimension_extent)?;

        Ok(Self {
            kind: HdlLayoutKind::Array,
            bit_extent: Some(bit_extent),
        })
    }

    /// Describe a structural aggregate with a known or symbolic extent.
    ///
    /// This method does not decide field order, padding, alignment, or ABI.
    #[must_use]
    pub fn aggregate(bit_extent: LayoutExtent) -> Self {
        Self {
            kind: HdlLayoutKind::Aggregate,
            bit_extent: Some(bit_extent),
        }
    }

    /// Describe an opaque type whose logical bit extent is not exposed.
    #[must_use]
    pub const fn opaque() -> Self {
        Self {
            kind: HdlLayoutKind::Opaque,
            bit_extent: None,
        }
    }

    /// Describe a type whose bit extent is not applicable.
    #[must_use]
    pub const fn not_bit_addressable() -> Self {
        Self {
            kind: HdlLayoutKind::NotBitAddressable,
            bit_extent: None,
        }
    }

    /// Describe a layout whose extent remains unresolved.
    ///
    /// # Errors
    ///
    /// Returns an error if `reason` is empty or whitespace-only.
    pub fn unresolved(
        reason: impl Into<String>,
    ) -> Result<Self, HdlLayoutError> {
        Ok(Self {
            kind: HdlLayoutKind::Unresolved,
            bit_extent: Some(LayoutExtent::unresolved(reason)?),
        })
    }

    /// Return the layout category.
    #[must_use]
    pub const fn kind(&self) -> HdlLayoutKind {
        self.kind
    }

    /// Return the logical bit extent, if applicable and represented.
    #[must_use]
    pub const fn bit_extent(&self) -> Option<&LayoutExtent> {
        self.bit_extent.as_ref()
    }
}

/// Errors produced while describing or combining HDL logical layouts.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HdlLayoutError {
    /// A width expression contains a zero-width literal.
    InvalidWidth,

    /// A concrete extent operation overflowed `u128`.
    ArithmeticOverflow,

    /// An unresolved layout was constructed without an explanation.
    EmptyReason,
}

impl fmt::Display for HdlLayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidWidth => {
                f.write_str("HDL layout width contains an invalid zero-width literal")
            }
            Self::ArithmeticOverflow => {
                f.write_str("HDL logical layout arithmetic overflowed u128")
            }
            Self::EmptyReason => {
                f.write_str("an unresolved HDL layout requires a non-empty reason")
            }
        }
    }
}

impl Error for HdlLayoutError {}

/// Evaluate a canonical width expression if it is concrete.
///
/// The explicit traversal stack avoids recursive evaluation of deeply nested
/// `HdlWidth` expressions. Concrete subexpressions are checked even when
/// another branch is symbolic.
fn evaluate_width(width: &HdlWidth) -> Result<Option<u128>, HdlLayoutError> {
    enum Frame<'a> {
        Visit(&'a HdlWidth),
        Add,
        Multiply,
    }

    let mut frames = vec![Frame::Visit(width)];
    let mut values: Vec<Option<u128>> = Vec::new();

    while let Some(frame) = frames.pop() {
        match frame {
            Frame::Visit(current) => match current {
                HdlWidth::Known(value) => {
                    if *value == 0 {
                        return Err(HdlLayoutError::InvalidWidth);
                    }
                    values.push(Some(*value));
                }
                HdlWidth::Symbol(_) | HdlWidth::Unresolved(_) => {
                    values.push(None);
                }
                HdlWidth::Add(left, right) => {
                    frames.push(Frame::Add);
                    frames.push(Frame::Visit(right));
                    frames.push(Frame::Visit(left));
                }
                HdlWidth::Multiply(left, right) => {
                    frames.push(Frame::Multiply);
                    frames.push(Frame::Visit(right));
                    frames.push(Frame::Visit(left));
                }
            },
            Frame::Add | Frame::Multiply => {
                let right = values.pop().ok_or(HdlLayoutError::InvalidWidth)?;
                let left = values.pop().ok_or(HdlLayoutError::InvalidWidth)?;

                let value = match (left, right) {
                    (Some(left), Some(right)) => {
                        if matches!(frame, Frame::Add) {
                            Some(
                                left.checked_add(right)
                                    .ok_or(HdlLayoutError::ArithmeticOverflow)?,
                            )
                        } else {
                            Some(
                                left.checked_mul(right)
                                    .ok_or(HdlLayoutError::ArithmeticOverflow)?,
                            )
                        }
                    }
                    _ => None,
                };

                values.push(value);
            }
        }
    }

    if values.len() != 1 {
        return Err(HdlLayoutError::InvalidWidth);
    }

    values.pop().ok_or(HdlLayoutError::InvalidWidth)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::HdlId;
    use crate::hdl::types::packed::{PackedFieldSpec, PackedOrder};

    #[test]
    fn concrete_width_is_preserved() {
        let layout = HdlTypeLayout::bit_vector(HdlWidth::Known(17))
            .expect("valid width");

        assert_eq!(
            layout.bit_extent().and_then(LayoutExtent::known_value),
            Some(17)
        );
    }

    #[test]
    fn symbolic_width_remains_symbolic() {
        let id = HdlId::new("WIDTH").expect("valid identifier");
        let layout = HdlTypeLayout::bit_vector(HdlWidth::Symbol(id))
            .expect("symbolic width is valid");

        assert_eq!(layout.kind(), HdlLayoutKind::BitVector);
        assert_eq!(
            layout.bit_extent().and_then(LayoutExtent::known_value),
            None
        );
    }

    #[test]
    fn zero_width_literal_is_rejected() {
        assert_eq!(
            HdlTypeLayout::bit_vector(HdlWidth::Known(0)),
            Err(HdlLayoutError::InvalidWidth)
        );
    }

    #[test]
    fn concrete_addition_checks_overflow() {
        let left = LayoutExtent::from_width(HdlWidth::Known(u128::MAX))
            .expect("valid width");
        let right = LayoutExtent::from_width(HdlWidth::Known(1))
            .expect("valid width");

        assert_eq!(
            left.checked_add(&right),
            Err(HdlLayoutError::ArithmeticOverflow)
        );
    }

    #[test]
    fn concrete_multiplication_checks_overflow() {
        let left = LayoutExtent::from_width(HdlWidth::Known(u128::MAX))
            .expect("valid width");
        let right = LayoutExtent::from_dimension(Dimension::Known(2));

        assert_eq!(
            left.checked_mul(&right),
            Err(HdlLayoutError::ArithmeticOverflow)
        );
    }

    #[test]
    fn zero_length_array_has_zero_extent() {
        let element =
            LayoutExtent::from_width(HdlWidth::Known(32)).expect("valid width");
        let layout =
            HdlTypeLayout::array(element, Dimension::Known(0)).expect("valid array");

        assert_eq!(
            layout.bit_extent().and_then(LayoutExtent::known_value),
            Some(0)
        );
    }

    #[test]
    fn dynamic_array_dimension_stays_dynamic() {
        let element =
            LayoutExtent::from_width(HdlWidth::Known(8)).expect("valid width");
        let layout =
            HdlTypeLayout::array(element, Dimension::Dynamic).expect("valid array");

        assert_eq!(
            layout.bit_extent().and_then(LayoutExtent::known_value),
            None
        );
    }

    #[test]
    fn packed_record_reuses_canonical_packed_layout() {
        let packed = PackedLayout::new(
            [
                PackedFieldSpec::new("tag", HdlWidth::Known(4)),
                PackedFieldSpec::new("payload", HdlWidth::Known(12)),
            ],
            PackedOrder::LeastSignificantFirst,
        )
        .expect("valid packed layout");

        let layout = HdlTypeLayout::packed_record(packed);

        assert_eq!(layout.kind(), HdlLayoutKind::PackedRecord);
        assert_eq!(
            layout.bit_extent().and_then(LayoutExtent::known_value),
            Some(16)
        );
    }

    #[test]
    fn non_bit_addressable_layout_does_not_mean_zero_bits() {
        let layout = HdlTypeLayout::not_bit_addressable();

        assert_eq!(layout.kind(), HdlLayoutKind::NotBitAddressable);
        assert!(layout.bit_extent().is_none());
    }

    #[test]
    fn unresolved_layout_requires_a_reason() {
        assert_eq!(
            HdlTypeLayout::unresolved("   "),
            Err(HdlLayoutError::EmptyReason)
        );

        assert!(
            HdlTypeLayout::unresolved("type parameter is not resolved").is_ok()
        );
    }
}
