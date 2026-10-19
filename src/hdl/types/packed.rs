
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Packed Layout
//!
//! Path: `src/hdl/types/packed.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Define portable, target-independent packed-field ordering and bit-offset
//! calculations for HDL types.
//!
//! This module describes a logical packed representation. It does not choose
//! an ABI, byte order, physical memory arrangement, device, pin, register,
//! routing resource, synthesis strategy, or backend representation.
//!
//! ## Authority and ownership
//!
//! Authoritative contracts:
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/user-defined-types.md`
//! - `grammar/spec/type-system.md`
//! - `grammar/hdl/hdl.g4`
//! - `grammar/declarations/records.g4`
//! - `grammar/types/record.g4`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/hdl/types/aggregate.rs`
//! - `src/zuir/types/type_def.rs`
//!
//! The grammar owns source syntax. The canonical semantic type system owns
//! type identity and type meaning. The HDL semantic width contract supplies
//! `HdlWidth`. Canonical aggregate declarations retain their own member order.
//!
//! This module owns only:
//!
//! - explicit packed-field ordering;
//! - packed field widths and logical offsets;
//! - checked arithmetic for concrete packed extents;
//! - symbolic offsets for parameter-dependent widths;
//! - an adapter from canonical record fields using a caller-supplied resolver.
//!
//! ## Does not own
//!
//! - A second AST, semantic type system, or canonical type identity.
//! - Name resolution or generic-parameter resolution.
//! - Physical or target-specific layout, alignment, or ABI.
//! - Value encoding, serialization formats, or runtime storage.
//! - Hardware allocation, synthesis, placement, or routing.
//! - HDLIR. All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Layout semantics
//!
//! Offsets are logical bit offsets, measured from the least-significant end
//! of the packed aggregate. Every field occupies a half-open interval:
//!
//! `offset .. offset + width`
//!
//! `LeastSignificantFirst` places the first declared field at offset zero.
//! `MostSignificantFirst` places the first declared field at the highest
//! field position, so its offset is the sum of the widths of later fields.
//!
//! These names describe field placement in a logical packed bit vector.
//! They do not specify byte order when that vector is serialized or stored.
//!
//! Declaration order is retained in the returned `fields()` slice for either
//! ordering policy. No sorting is performed.
//!
//! ## Symbolic widths and scalability
//!
//! Widths reuse the existing `HdlWidth` type. Unresolved widths remain
//! symbolic; this module does not guess parameter values.
//!
//! Concrete sums are checked. An overflow is an error, never wrapping or
//! truncation. Symbolic expressions remain symbolic until another layer
//! resolves them.
//!
//! The existing canonical `HdlWidth::Known(u128)` bounds the concrete numeric
//! values this adapter can currently represent. This module does not claim
//! arbitrary-precision concrete widths. Removing that representation boundary
//! requires a coordinated change to the canonical width contract and its
//! consumers.
//!
//! There is no arbitrary language-level limit on the number of fields.
//! Actual work and memory usage are bounded by the input and available
//! resources. Operational budgets belong to the calling compilation session.
//!
//! ## Integration contract
//!
//! - `src/hdl/types/mod.rs` registers this module as `pub mod packed;`.
//! - `PackedLayout::from_record_fields` consumes canonical `FieldDef` values.
//! - Its resolver must obtain each field's width from the canonical type and
//!   semantic-resolution services; unresolved or unsupported types must be
//!   reported rather than guessed.
//! - `src/hdl/types/aggregate.rs` remains the aggregate validation adapter.
//! - A future layout consumer can use `fields()`, `total_width()`, and each
//!   field's `offset()` without depending on HDLIR or a target backend.
//! - Target-specific layout must be calculated in a separate layer with
//!   explicit target inputs.
//!
//! ## Freeze criteria
//!
//! Freeze this API after module registration, compilation against the actual
//! workspace, and unit/integration tests. Any future layout extension must
//! preserve canonical type ownership, declaration order, checked arithmetic,
//! and the separation between logical packed layout and physical layout.

use core::convert::Infallible;
use core::fmt;
use std::collections::BTreeSet;
use std::error::Error;

use crate::frontend::semantic::domains::hdl::HdlWidth;
use crate::zuir::types::{FieldDef, TypeRef};

/// Policy for assigning fields to positions in a logical packed bit vector.
///
/// This is a source-independent layout policy. It is not a byte-order or
/// physical-memory policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PackedOrder {
    /// The first declared field starts at bit offset zero.
    ///
    /// Later fields occupy successively higher bit positions.
    LeastSignificantFirst,

    /// The first declared field occupies the most-significant field position.
    ///
    /// Its offset is the sum of the widths of all fields declared after it.
    MostSignificantFirst,
}

/// A logical extent expression that can represent zero and symbolic sums.
///
/// `HdlWidth` represents positive bit widths, so it cannot by itself represent
/// the zero offset or an empty packed aggregate. This wrapper adds those
/// cases without changing the canonical width type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackedExtent {
    expression: PackedExtentExpr,
    known: Option<u128>,
}

/// The expression underlying a packed extent.
///
/// Values are produced by this module's checked constructors. Consumers
/// should treat the expression as descriptive data, not as permission to
/// evaluate symbolic values without the canonical semantic environment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PackedExtentExpr {
    /// Zero bits. This is valid for an offset or an empty aggregate.
    Zero,

    /// A canonical HDL width expression.
    Width(HdlWidth),

    /// The sum of two extents.
    Add(Box<PackedExtentExpr>, Box<PackedExtentExpr>),
}

impl PackedExtent {
    /// Return the exact concrete value when it is known.
    ///
    /// `None` means that at least one symbolic term remains unresolved.
    #[must_use]
    pub const fn as_known(&self) -> Option<u128> {
        self.known
    }

    /// Return whether the extent contains no unresolved width terms.
    #[must_use]
    pub const fn is_known(&self) -> bool {
        self.known.is_some()
    }

    /// Return the symbolic expression for this extent.
    #[must_use]
    pub const fn expression(&self) -> &PackedExtentExpr {
        &self.expression
    }

    fn zero() -> Self {
        Self {
            expression: PackedExtentExpr::Zero,
            known: Some(0),
        }
    }

    fn from_width<E>(
        width: HdlWidth,
        field: &str,
    ) -> Result<Self, PackedLayoutError<E>> {
        validate_width::<E>(&width, field)?;
        let known = evaluate_concrete_width::<E>(&width, field)?;

        Ok(Self {
            expression: PackedExtentExpr::Width(width),
            known,
        })
    }

    fn checked_add_width<E>(
        &self,
        width: &HdlWidth,
        field: &str,
    ) -> Result<Self, PackedLayoutError<E>> {
        validate_width::<E>(width, field)?;
        let width_known = evaluate_concrete_width::<E>(width, field)?;

        let known = match (self.known, width_known) {
            (Some(left), Some(right)) => Some(
                left.checked_add(right)
                    .ok_or_else(|| PackedLayoutError::ArithmeticOverflow {
                        field: field.to_owned(),
                    })?,
            ),
            _ => None,
        };

        let expression = match (&self.expression, width) {
            (PackedExtentExpr::Zero, HdlWidth::Known(value)) => {
                PackedExtentExpr::Width(HdlWidth::Known(*value))
            }
            (PackedExtentExpr::Zero, other) => {
                PackedExtentExpr::Width(other.clone())
            }
            (current, HdlWidth::Known(0)) => current.clone(),
            (current, other) => PackedExtentExpr::Add(
                Box::new(current.clone()),
                Box::new(PackedExtentExpr::Width(other.clone())),
            ),
        };

        Ok(Self { expression, known })
    }
}

impl fmt::Display for PackedExtent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.known {
            Some(value) => write!(f, "{value}"),
            None => write!(f, "{:?}", self.expression),
        }
    }
}

/// One input field to packed-layout construction.
///
/// Field names are retained for diagnostics and downstream lookup. Widths
/// must be positive when concrete; symbolic widths are permitted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackedFieldSpec {
    name: String,
    width: HdlWidth,
}

impl PackedFieldSpec {
    /// Construct a field specification.
    ///
    /// Width validation occurs when the complete layout is built, so all
    /// construction paths share the same validation rules.
    #[must_use]
    pub fn new(name: impl Into<String>, width: HdlWidth) -> Self {
        Self {
            name: name.into(),
            width,
        }
    }

    /// Return the field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the canonical width expression.
    #[must_use]
    pub const fn width(&self) -> &HdlWidth {
        &self.width
    }
}

/// A field after its packed position has been calculated.
///
/// The field's declaration order is preserved separately from its bit offset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackedFieldLayout {
    name: String,
    width: HdlWidth,
    offset: PackedExtent,
    end_offset: PackedExtent,
}

impl PackedFieldLayout {
    /// Return the field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the field's canonical width expression.
    #[must_use]
    pub const fn width(&self) -> &HdlWidth {
        &self.width
    }

    /// Return the logical bit offset from the least-significant end.
    #[must_use]
    pub const fn offset(&self) -> &PackedExtent {
        &self.offset
    }

    /// Return the exclusive end offset, equivalent to `offset + width`.
    #[must_use]
    pub const fn end_offset(&self) -> &PackedExtent {
        &self.end_offset
    }
}

/// A validated logical packed layout.
///
/// This object does not own the source declaration or canonical type. It
/// records a checked layout derived from resolved field-width expressions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackedLayout {
    order: PackedOrder,
    fields: Vec<PackedFieldLayout>,
    total_width: PackedExtent,
}

impl PackedLayout {
    /// Build a packed layout from ordered field specifications.
    ///
    /// Duplicate names and invalid widths are rejected. Declaration order is
    /// retained. No fields are allocated as runtime values or hardware.
    ///
    /// An empty input is allowed and has a total width of zero.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty field name, duplicate field name,
    /// invalid width, or concrete arithmetic overflow.
    pub fn new(
        fields: impl IntoIterator<Item = PackedFieldSpec>,
        order: PackedOrder,
    ) -> Result<Self, PackedLayoutError<Infallible>> {
        Self::build(fields.into_iter().collect(), order)
    }

    /// Build a packed layout from canonical record fields.
    ///
    /// `width_of` is supplied by the caller's canonical type-resolution
    /// layer. It must return a width only when the field's semantic type has
    /// a valid HDL packed-bit interpretation. This function deliberately does
    /// not guess widths for named, aggregate, opaque, or extension types.
    ///
    /// The canonical field order is retained. The returned layout does not
    /// replace or mutate the canonical `FieldDef` values.
    ///
    /// # Errors
    ///
    /// Returns an error if a field name is invalid or duplicated, the width
    /// resolver fails, a width is invalid, or concrete arithmetic overflows.
    pub fn from_record_fields<E, F>(
        fields: &[FieldDef],
        order: PackedOrder,
        mut width_of: F,
    ) -> Result<Self, PackedLayoutError<E>>
    where
        F: FnMut(&TypeRef) -> Result<HdlWidth, E>,
    {
        let mut specifications = Vec::with_capacity(fields.len());

        for field in fields {
            let width = width_of(&field.ty).map_err(|source| {
                PackedLayoutError::TypeResolution {
                    field: field.name.clone(),
                    source,
                }
            })?;

            specifications.push(PackedFieldSpec::new(field.name.clone(), width));
        }

        Self::build(specifications, order)
    }

    fn build<E>(
        specifications: Vec<PackedFieldSpec>,
        order: PackedOrder,
    ) -> Result<Self, PackedLayoutError<E>> {
        let mut names = BTreeSet::new();

        for field in &specifications {
            if field.name.trim().is_empty() {
                return Err(PackedLayoutError::EmptyFieldName);
            }

            if !names.insert(field.name.clone()) {
                return Err(PackedLayoutError::DuplicateFieldName {
                    name: field.name.clone(),
                });
            }

            validate_width::<E>(&field.width, &field.name)?;
        }

        let mut offsets = vec![PackedExtent::zero(); specifications.len()];
        let mut total_width = PackedExtent::zero();

        for field in &specifications {
            total_width =
                total_width.checked_add_width::<E>(&field.width, &field.name)?;
        }

        match order {
            PackedOrder::LeastSignificantFirst => {
                let mut cursor = PackedExtent::zero();

                for (index, field) in specifications.iter().enumerate() {
                    offsets[index] = cursor.clone();
                    cursor =
                        cursor.checked_add_width::<E>(&field.width, &field.name)?;
                }
            }
            PackedOrder::MostSignificantFirst => {
                let mut cursor = PackedExtent::zero();

                for (index, field) in specifications.iter().enumerate().rev() {
                    offsets[index] = cursor.clone();
                    cursor =
                        cursor.checked_add_width::<E>(&field.width, &field.name)?;
                }
            }
        }

        let mut layouts = Vec::with_capacity(specifications.len());

        for (field, offset) in specifications.into_iter().zip(offsets) {
            let end_offset =
                offset.checked_add_width::<E>(&field.width, &field.name)?;

            layouts.push(PackedFieldLayout {
                name: field.name,
                width: field.width,
                offset,
                end_offset,
            });
        }

        Ok(Self {
            order,
            fields: layouts,
            total_width,
        })
    }

    /// Return the selected field-ordering policy.
    #[must_use]
    pub const fn order(&self) -> PackedOrder {
        self.order
    }

    /// Return fields in their original declaration order.
    #[must_use]
    pub fn fields(&self) -> &[PackedFieldLayout] {
        &self.fields
    }

    /// Find a field by its exact declared name.
    ///
    /// Names are not normalized or case-folded by this module.
    #[must_use]
    pub fn field(&self, name: &str) -> Option<&PackedFieldLayout> {
        self.fields.iter().find(|field| field.name == name)
    }

    /// Return the total packed width as a concrete or symbolic extent.
    ///
    /// An empty layout has a total width of zero.
    #[must_use]
    pub const fn total_width(&self) -> &PackedExtent {
        &self.total_width
    }
}

/// Errors encountered while constructing a packed layout.
///
/// `E` preserves the caller's original type-resolution error instead of
/// reducing it to a diagnostic string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PackedLayoutError<E = Infallible> {
    /// A field name was empty or contained only whitespace.
    EmptyFieldName,

    /// A field name appeared more than once.
    DuplicateFieldName {
        /// Repeated field name.
        name: String,
    },

    /// A width expression contained a zero-width literal.
    InvalidWidth {
        /// Field whose width was rejected.
        field: String,
    },

    /// Concrete width or offset arithmetic exceeded `u128`.
    ArithmeticOverflow {
        /// Field whose position or extent overflowed.
        field: String,
    },

    /// The canonical type-width resolver could not establish a field width.
    TypeResolution {
        /// Field whose type could not be resolved to a packed width.
        field: String,
        /// Original error from the caller's resolver.
        source: E,
    },
}

impl<E: fmt::Display> fmt::Display for PackedLayoutError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyFieldName => {
                f.write_str("packed field name cannot be empty")
            }
            Self::DuplicateFieldName { name } => {
                write!(f, "duplicate packed field name `{name}`")
            }
            Self::InvalidWidth { field } => {
                write!(f, "packed field `{field}` has an invalid width")
            }
            Self::ArithmeticOverflow { field } => {
                write!(
                    f,
                    "packed layout arithmetic overflow while processing `{field}`"
                )
            }
            Self::TypeResolution { field, source } => {
                write!(
                    f,
                    "cannot resolve packed width for field `{field}`: {source}"
                )
            }
        }
    }
}

impl<E: Error + 'static> Error for PackedLayoutError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::TypeResolution { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Validate a width without recursively walking arbitrarily nested expressions.
///
/// Concrete subexpressions are checked for arithmetic overflow. Symbolic
/// subexpressions are retained as unresolved expressions.
fn validate_width<E>(
    width: &HdlWidth,
    field: &str,
) -> Result<(), PackedLayoutError<E>> {
    let _ = evaluate_concrete_width::<E>(width, field)?;
    Ok(())
}

/// Evaluate a width if all its terms are concrete.
///
/// An explicit stack avoids recursive evaluation of nested width expressions.
/// `None` means at least one symbol remains unresolved. Invalid zero literals
/// and overflow in wholly concrete subexpressions are reported immediately.
fn evaluate_concrete_width<E>(
    width: &HdlWidth,
    field: &str,
) -> Result<Option<u128>, PackedLayoutError<E>> {
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
                        return Err(PackedLayoutError::InvalidWidth {
                            field: field.to_owned(),
                        });
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
                let right = values.pop().ok_or_else(|| {
                    PackedLayoutError::InvalidWidth {
                        field: field.to_owned(),
                    }
                })?;
                let left = values.pop().ok_or_else(|| {
                    PackedLayoutError::InvalidWidth {
                        field: field.to_owned(),
                    }
                })?;

                let result = match (left, right, frame) {
                    (Some(left), Some(right), Frame::Add) => Some(
                        left.checked_add(right).ok_or_else(|| {
                            PackedLayoutError::ArithmeticOverflow {
                                field: field.to_owned(),
                            }
                        })?,
                    ),
                    (Some(left), Some(right), Frame::Multiply) => Some(
                        left.checked_mul(right).ok_or_else(|| {
                            PackedLayoutError::ArithmeticOverflow {
                                field: field.to_owned(),
                            }
                        })?,
                    ),
                    (_, _, _) => None,
                };

                if result == Some(0) {
                    return Err(PackedLayoutError::InvalidWidth {
                        field: field.to_owned(),
                    });
                }

                values.push(result);
            }
        }
    }

    if values.len() != 1 {
        return Err(PackedLayoutError::InvalidWidth {
            field: field.to_owned(),
        });
    }

    values.pop().ok_or_else(|| PackedLayoutError::InvalidWidth {
        field: field.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::semantic::domains::hdl::HdlId;

    fn known(value: u128) -> HdlWidth {
        HdlWidth::Known(value)
    }

    fn symbol(name: &str) -> HdlWidth {
        HdlWidth::Symbol(
            HdlId::new(name).expect("test symbol should be valid"),
        )
    }

    fn field(name: &str, width: HdlWidth) -> PackedFieldSpec {
        PackedFieldSpec::new(name, width)
    }

    #[test]
    fn least_significant_first_preserves_declaration_order() {
        let layout = PackedLayout::new(
            [
                field("opcode", known(4)),
                field("flags", known(3)),
                field("payload", known(9)),
            ],
            PackedOrder::LeastSignificantFirst,
        )
        .expect("layout should be valid");

        assert_eq!(layout.fields()[0].name(), "opcode");
        assert_eq!(layout.fields()[0].offset().as_known(), Some(0));
        assert_eq!(layout.fields()[1].offset().as_known(), Some(4));
        assert_eq!(layout.fields()[2].offset().as_known(), Some(7));
        assert_eq!(layout.total_width().as_known(), Some(16));
    }

    #[test]
    fn most_significant_first_uses_suffix_widths() {
        let layout = PackedLayout::new(
            [
                field("opcode", known(4)),
                field("flags", known(3)),
                field("payload", known(9)),
            ],
            PackedOrder::MostSignificantFirst,
        )
        .expect("layout should be valid");

        assert_eq!(layout.fields()[0].name(), "opcode");
        assert_eq!(layout.fields()[0].offset().as_known(), Some(12));
        assert_eq!(layout.fields()[1].offset().as_known(), Some(9));
        assert_eq!(layout.fields()[2].offset().as_known(), Some(0));
        assert_eq!(layout.total_width().as_known(), Some(16));
    }

    #[test]
    fn end_offsets_are_exclusive() {
        let layout = PackedLayout::new(
            [field("a", known(5)), field("b", known(7))],
            PackedOrder::LeastSignificantFirst,
        )
        .expect("layout should be valid");

        assert_eq!(layout.fields()[0].offset().as_known(), Some(0));
        assert_eq!(layout.fields()[0].end_offset().as_known(), Some(5));
        assert_eq!(layout.fields()[1].offset().as_known(), Some(5));
        assert_eq!(layout.fields()[1].end_offset().as_known(), Some(12));
    }

    #[test]
    fn symbolic_widths_produce_symbolic_offsets() {
        let layout = PackedLayout::new(
            [
                field("prefix", symbol("PREFIX_WIDTH")),
                field("payload", known(8)),
                field("suffix", symbol("SUFFIX_WIDTH")),
            ],
            PackedOrder::LeastSignificantFirst,
        )
        .expect("symbolic layout should be valid");

        assert_eq!(layout.fields()[0].offset().as_known(), Some(0));
        assert_eq!(layout.fields()[1].offset().as_known(), None);
        assert_eq!(layout.fields()[2].offset().as_known(), None);
        assert_eq!(layout.total_width().as_known(), None);
    }

    #[test]
    fn empty_layout_has_zero_width() {
        let layout = PackedLayout::new(
            std::iter::empty(),
            PackedOrder::LeastSignificantFirst,
        )
        .expect("empty layout is representable");

        assert!(layout.fields().is_empty());
        assert_eq!(layout.total_width().as_known(), Some(0));
    }

    #[test]
    fn duplicate_names_are_rejected() {
        let result = PackedLayout::new(
            [field("value", known(4)), field("value", known(8))],
            PackedOrder::LeastSignificantFirst,
        );

        assert_eq!(
            result,
            Err(PackedLayoutError::DuplicateFieldName {
                name: "value".to_owned(),
            })
        );
    }

    #[test]
    fn zero_width_is_rejected() {
        let result = PackedLayout::new(
            [field("invalid", known(0))],
            PackedOrder::LeastSignificantFirst,
        );

        assert_eq!(
            result,
            Err(PackedLayoutError::InvalidWidth {
                field: "invalid".to_owned(),
            })
        );
    }

    #[test]
    fn concrete_total_width_overflow_is_rejected() {
        let result = PackedLayout::new(
            [
                field("large", known(u128::MAX)),
                field("extra", known(1)),
            ],
            PackedOrder::LeastSignificantFirst,
        );

        assert_eq!(
            result,
            Err(PackedLayoutError::ArithmeticOverflow {
                field: "extra".to_owned(),
            })
        );
    }

    #[test]
    fn nested_concrete_width_overflow_is_rejected() {
        let width = HdlWidth::Add(
            Box::new(known(u128::MAX)),
            Box::new(known(1)),
        );

        let result = PackedLayout::new(
            [field("overflow", width)],
            PackedOrder::LeastSignificantFirst,
        );

        assert_eq!(
            result,
            Err(PackedLayoutError::ArithmeticOverflow {
                field: "overflow".to_owned(),
            })
        );
    }

    #[test]
    fn symbolic_width_expression_is_preserved() {
        let width = HdlWidth::Add(
            Box::new(symbol("LANES")),
            Box::new(known(2)),
        );

        let layout = PackedLayout::new(
            [field("data", width.clone())],
            PackedOrder::LeastSignificantFirst,
        )
        .expect("symbolic width is valid");

        assert_eq!(layout.fields()[0].width(), &width);
        assert_eq!(layout.fields()[0].offset().as_known(), Some(0));
        assert_eq!(layout.fields()[0].end_offset().as_known(), None);
    }

    #[test]
    fn field_lookup_uses_exact_name() {
        let layout = PackedLayout::new(
            [field("Data", known(8))],
            PackedOrder::LeastSignificantFirst,
        )
        .expect("layout should be valid");

        assert!(layout.field("Data").is_some());
        assert!(layout.field("data").is_none());
    }
}
