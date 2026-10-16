#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical type-layout model.
//!
//! Path: src/classical/types/layout.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Represents the size, alignment, stride, and field offsets of a type
//! representation without defining a competing semantic type system.
//!
//! # Authority
//!
//! Source types are owned by the canonical frontend AST.
//! Resolved type meaning is owned by
//! `crate::frontend::semantic::types::ty::SemanticType`.
//! Target-specific ABI and representation rules are owned by the relevant
//! target/backend policy. This module supplies target-independent layout
//! values and checked composition primitives.
//!
//! # Owns
//!
//! - LayoutValue and symbolic layout expressions.
//! - TypeLayout and field-offset descriptions.
//! - Checked layout composition for records and arrays.
//! - Local layout validation and errors.
//!
//! # Does not own
//!
//! - Source syntax, AST construction, name resolution, or type inference.
//! - Canonical semantic types, generic substitution, or constraint solving.
//! - Target ABI rules, pointer widths, register widths, or device capabilities.
//! - Memory allocation, physical addresses, ownership, or borrow checking.
//! - IR construction, scheduling, or runtime execution.
//!
//! # POCO-REAF
//!
//! No universal maximum type size, field count, array length, or target
//! capacity is defined here. Concrete values use `usize` because they must
//! fit the current compiler process; arithmetic overflow is preserved as a
//! symbolic expression rather than wrapped or truncated.
//!
//! Symbolic layouts are not proof that a target can realize them. A backend
//! must resolve the required expressions and verify its own ABI and resource
//! constraints before code generation.
//!
//! # Integration
//!
//! - `types/mod.rs` must declare `pub mod layout;`.
//! - The semantic resolver maps canonical `SemanticType` values into
//!   layout requests; this module deliberately does not reimplement that
//!   resolver or duplicate `SemanticType`.
//! - `types/scalar.rs` and `types/aggregate.rs` may supply classical-facing
//!   descriptors, but they do not determine target ABI layout.
//! - Lowering and target backends consume `TypeLayout` and must check that
//!   symbolic values are resolved before requiring concrete offsets.
//!
//! # Freeze criteria
//!
//! Formatting, unit tests, module registration, semantic mapping, target
//! layout tests, and workspace compilation must pass before this file is
//! marked verified or production-ready.

use core::fmt;

/// A symbolic arithmetic expression used when a layout quantity cannot
/// safely be represented as a concrete `usize`.
///
/// Constants are decimal strings so expressions can retain values larger
/// than the current compiler host can represent. This type does not evaluate
/// arbitrary-precision arithmetic; it preserves the expression for a later
/// evaluator or target-specific layout service.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LayoutExpression {
    /// A non-negative decimal integer.
    Constant(String),

    /// A symbolic name, such as a resolved type-level dimension.
    Symbol(String),

    /// Addition of two layout expressions.
    Add(Box<LayoutExpression>, Box<LayoutExpression>),

    /// Multiplication of two layout expressions.
    Multiply(Box<LayoutExpression>, Box<LayoutExpression>),

    /// Round `value` up to a multiple of `alignment`.
    AlignUp {
        value: Box<LayoutExpression>,
        alignment: Box<LayoutExpression>,
    },

    /// Maximum of one or more expressions.
    Maximum(Vec<LayoutExpression>),
}

impl LayoutExpression {
    /// Constructs a non-negative decimal constant without narrowing it.
    pub fn constant_decimal(
        decimal: impl AsRef<str>,
    ) -> Result<Self, LayoutError> {
        let input = decimal.as_ref();

        if input.is_empty() || !input.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(LayoutError::InvalidDecimalConstant);
        }

        let normalized = input.trim_start_matches('0');
        let normalized = if normalized.is_empty() { "0" } else { normalized };

        Ok(Self::Constant(normalized.to_owned()))
    }

    /// Constructs a symbolic name.
    pub fn symbol(name: impl Into<String>) -> Result<Self, LayoutError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(LayoutError::EmptySymbol);
        }

        Ok(Self::Symbol(name))
    }

    fn from_value(value: &LayoutValue) -> Option<Self> {
        match value {
            LayoutValue::Known(value) => Some(Self::Constant(value.to_string())),
            LayoutValue::Symbolic(expression) => Some(expression.clone()),
            LayoutValue::Unsized | LayoutValue::Unknown => None,
        }
    }
}

/// A layout quantity.
///
/// `Known` is a value representable by the current compiler process.
/// `Symbolic` preserves unresolved or host-overflowing calculations.
/// `Unsized` represents a type with no standalone fixed size.
/// `Unknown` represents information that has not been supplied.
///
/// `Unsized` and `Unknown` are deliberately distinct: a dynamically sized
/// type is not the same as a type whose layout has not yet been determined.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LayoutValue {
    /// A non-negative, host-representable quantity.
    Known(usize),

    /// An expression that must be evaluated by a suitable later stage.
    Symbolic(LayoutExpression),

    /// No standalone fixed size or stride exists.
    Unsized,

    /// Insufficient information is currently available.
    Unknown,
}

impl LayoutValue {
    /// Returns a known layout quantity.
    #[must_use]
    pub const fn known(value: usize) -> Self {
        Self::Known(value)
    }

    /// Returns a symbolic layout quantity.
    #[must_use]
    pub fn symbolic(expression: LayoutExpression) -> Self {
        Self::Symbolic(expression)
    }

    /// Returns a concrete value, if one is available.
    #[must_use]
    pub const fn as_known(&self) -> Option<usize> {
        match self {
            Self::Known(value) => Some(*value),
            Self::Symbolic(_) | Self::Unsized | Self::Unknown => None,
        }
    }

    /// Returns whether the quantity has a concrete representation.
    #[must_use]
    pub const fn is_known(&self) -> bool {
        matches!(self, Self::Known(_))
    }

    /// Adds two quantities without wrapping.
    ///
    /// If concrete arithmetic overflows, the operation is retained
    /// symbolically. An unsized operand yields `Unsized`; an unknown operand
    /// yields `Unknown`.
    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Known(left), Self::Known(right)) => match left.checked_add(*right) {
                Some(value) => Self::Known(value),
                None => Self::symbolic(Self::Add(
                    Box::new(Self::Constant(left.to_string())),
                    Box::new(Self::Constant(right.to_string())),
                )),
            },
            (Self::Unsized, _) | (_, Self::Unsized) => Self::Unsized,
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            _ => match (
                LayoutExpression::from_value(self),
                LayoutExpression::from_value(other),
            ) {
                (Some(left), Some(right)) => {
                    Self::symbolic(LayoutExpression::Add(Box::new(left), Box::new(right)))
                }
                _ => Self::Unknown,
            },
        }
    }

    /// Multiplies two quantities without wrapping.
    #[must_use]
    pub fn multiply(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Known(left), Self::Known(right)) => match left.checked_mul(*right) {
                Some(value) => Self::Known(value),
                None => Self::symbolic(LayoutExpression::Multiply(
                    Box::new(LayoutExpression::Constant(left.to_string())),
                    Box::new(LayoutExpression::Constant(right.to_string())),
                )),
            },
            (Self::Unsized, _) | (_, Self::Unsized) => Self::Unsized,
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            _ => match (
                LayoutExpression::from_value(self),
                LayoutExpression::from_value(other),
            ) {
                (Some(left), Some(right)) => Self::symbolic(
                    LayoutExpression::Multiply(Box::new(left), Box::new(right)),
                ),
                _ => Self::Unknown,
            },
        }
    }

    /// Rounds a quantity up to a multiple of an alignment.
    ///
    /// Alignment must be positive when concrete. This implementation does
    /// not assume that every target's alignment is a power of two.
    pub fn align_up(&self, alignment: &Self) -> Result<Self, LayoutError> {
        if let Self::Known(0) = alignment {
            return Err(LayoutError::ZeroAlignment);
        }

        match (self, alignment) {
            (Self::Known(value), Self::Known(align)) => {
                let remainder = value % align;

                if remainder == 0 {
                    return Ok(Self::Known(*value));
                }

                let padding = align - remainder;

                match value.checked_add(padding) {
                    Some(aligned) => Ok(Self::Known(aligned)),
                    None => Ok(Self::symbolic(LayoutExpression::AlignUp {
                        value: Box::new(LayoutExpression::Constant(value.to_string())),
                        alignment: Box::new(LayoutExpression::Constant(align.to_string())),
                    })),
                }
            }
            (Self::Unsized, _) => Ok(Self::Unsized),
            (Self::Unknown, _) | (_, Self::Unknown) | (_, Self::Unsized) => {
                Ok(Self::Unknown)
            }
            _ => {
                let value = LayoutExpression::from_value(self);
                let alignment = LayoutExpression::from_value(alignment);

                match (value, alignment) {
                    (Some(value), Some(alignment)) => {
                        Ok(Self::symbolic(LayoutExpression::AlignUp {
                            value: Box::new(value),
                            alignment: Box::new(alignment),
                        }))
                    }
                    _ => Ok(Self::Unknown),
                }
            }
        }
    }

    fn maximum(values: &[Self]) -> Result<Self, LayoutError> {
        if values.is_empty() {
            return Err(LayoutError::EmptyAlignmentSet);
        }

        if values.iter().any(|value| matches!(value, Self::Unsized)) {
            return Err(LayoutError::UnsizedAlignment);
        }

        if values.iter().any(|value| matches!(value, Self::Unknown)) {
            return Ok(Self::Unknown);
        }

        if values.iter().all(|value| matches!(value, Self::Known(_))) {
            let maximum = values
                .iter()
                .filter_map(Self::as_known)
                .max()
                .ok_or(LayoutError::EmptyAlignmentSet)?;

            return Ok(Self::Known(maximum));
        }

        let expressions = values
            .iter()
            .map(LayoutExpression::from_value)
            .collect::<Option<Vec<_>>>()
            .ok_or(LayoutError::InvalidLayoutExpression)?;

        if expressions.len() == 1 {
            return expressions
                .into_iter()
                .next()
                .map(Self::Symbolic)
                .ok_or(LayoutError::InvalidLayoutExpression);
        }

        Ok(Self::symbolic(LayoutExpression::Maximum(expressions)))
    }
}

/// The broad representation category described by a layout.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LayoutKind {
    /// A scalar or other single storage unit.
    Scalar,

    /// A fixed-layout aggregate whose fields have individual offsets.
    Record,

    /// A repeated element representation.
    Array {
        /// Length as a known or symbolic value.
        length: LayoutValue,
    },

    /// A dynamically sized representation without a standalone fixed size.
    Unsized,

    /// A representation described by a target or extension contract.
    Opaque,

    /// A dialect-defined layout category.
    Extension(String),
}

/// Layout metadata for one field of an aggregate.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FieldLayout {
    name: Option<String>,
    offset: LayoutValue,
    layout: Box<TypeLayout>,
}

impl FieldLayout {
    /// Returns the optional source/semantic field name.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the field's offset from the beginning of its containing type.
    ///
    /// The offset may remain symbolic until target layout resolution.
    #[must_use]
    pub fn offset(&self) -> &LayoutValue {
        &self.offset
    }

    /// Returns the field's own layout.
    #[must_use]
    pub fn layout(&self) -> &TypeLayout {
        &self.layout
    }
}

/// A computed or partially symbolic type layout.
///
/// This is representation metadata, not a semantic type, allocation, or
/// physical address. A `TypeLayout` does not itself prove that a target
/// supports the representation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeLayout {
    kind: LayoutKind,
    size: LayoutValue,
    alignment: LayoutValue,
    stride: LayoutValue,
    fields: Vec<FieldLayout>,
}

impl TypeLayout {
    /// Constructs a scalar layout from a size and alignment.
    pub fn scalar(
        size: LayoutValue,
        alignment: LayoutValue,
    ) -> Result<Self, LayoutError> {
        Self::validate_alignment(&alignment)?;

        let stride = size.align_up(&alignment)?;

        Ok(Self {
            kind: LayoutKind::Scalar,
            size,
            alignment,
            stride,
            fields: Vec::new(),
        })
    }

    /// Constructs opaque representation metadata supplied by a target or
    /// extension contract.
    pub fn opaque(
        size: LayoutValue,
        alignment: LayoutValue,
        stride: LayoutValue,
    ) -> Result<Self, LayoutError> {
        Self::validate_alignment(&alignment)?;

        Ok(Self {
            kind: LayoutKind::Opaque,
            size,
            alignment,
            stride,
            fields: Vec::new(),
        })
    }

    /// Constructs a dynamically sized layout.
    pub fn unsized(alignment: LayoutValue) -> Result<Self, LayoutError> {
        Self::validate_alignment(&alignment)?;

        Ok(Self {
            kind: LayoutKind::Unsized,
            size: LayoutValue::Unsized,
            alignment,
            stride: LayoutValue::Unsized,
            fields: Vec::new(),
        })
    }

    /// Returns the representation category.
    #[must_use]
    pub fn kind(&self) -> &LayoutKind {
        &self.kind
    }

    /// Returns the complete size, or its symbolic/unresolved state.
    #[must_use]
    pub fn size(&self) -> &LayoutValue {
        &self.size
    }

    /// Returns the required alignment, or its symbolic/unresolved state.
    #[must_use]
    pub fn alignment(&self) -> &LayoutValue {
        &self.alignment
    }

    /// Returns the stride used when this representation is repeated.
    #[must_use]
    pub fn stride(&self) -> &LayoutValue {
        &self.stride
    }

    /// Returns aggregate fields in the order used by the layout policy.
    #[must_use]
    pub fn fields(&self) -> &[FieldLayout] {
        &self.fields
    }

    /// Returns true only when size, alignment, stride, and field offsets
    /// are all concrete.
    #[must_use]
    pub fn is_fully_concrete(&self) -> bool {
        self.size.is_known()
            && self.alignment.is_known()
            && self.stride.is_known()
            && self.fields.iter().all(|field| {
                field.offset.is_known() && field.layout.is_fully_concrete()
            })
    }

    fn validate_alignment(alignment: &LayoutValue) -> Result<(), LayoutError> {
        match alignment {
            LayoutValue::Known(0) => Err(LayoutError::ZeroAlignment),
            LayoutValue::Unsized => Err(LayoutError::UnsizedAlignment),
            LayoutValue::Known(_) | LayoutValue::Symbolic(_) | LayoutValue::Unknown => Ok(()),
        }
    }
}

/// Target-independent layout composition helpers.
///
/// This helper composes layouts supplied by semantic mapping or a target
/// policy. It does not choose pointer widths, field reordering, packing,
/// bit-field rules, enum encodings, or an ABI.
#[derive(Clone, Copy, Debug, Default)]
pub struct LayoutEngine;

impl LayoutEngine {
    /// Lays out a record in the supplied field order.
    ///
    /// `fields` contains `(optional_name, field_layout)` pairs. The caller
    /// is responsible for supplying fields in the order required by the
    /// applicable language and target policy.
    ///
    /// `minimum_alignment`, when supplied, is combined with the field
    /// alignments. It is a policy input, not a universal constant.
    ///
    /// If any field's size is unsized, this method returns an error rather
    /// than guessing flexible-array or dynamically sized aggregate rules.
    /// Such rules must be explicitly supplied by the owning target policy.
    pub fn record<I>(
        &self,
        fields: I,
        minimum_alignment: Option<LayoutValue>,
    ) -> Result<TypeLayout, LayoutError>
    where
        I: IntoIterator<Item = (Option<String>, TypeLayout)>,
    {
        let fields = fields.into_iter().collect::<Vec<_>>();
        let mut alignments = Vec::with_capacity(fields.len() + usize::from(minimum_alignment.is_some()));

        for (_, field) in &fields {
            TypeLayout::validate_alignment(&field.alignment)?;

            if matches!(field.size, LayoutValue::Unsized) {
                return Err(LayoutError::UnsizedAggregateMember);
            }

            alignments.push(field.alignment.clone());
        }

        if let Some(alignment) = minimum_alignment {
            TypeLayout::validate_alignment(&alignment)?;
            alignments.push(alignment);
        }

        // An empty record still needs an explicit target/language policy.
        // Do not silently assume that its size or alignment is universally 1.
        if alignments.is_empty() {
            return Err(LayoutError::EmptyRecordNeedsPolicy);
        }

        let aggregate_alignment = LayoutValue::maximum(&alignments)?;
        let mut cursor = LayoutValue::Known(0);
        let mut field_layouts = Vec::with_capacity(fields.len());

        for (name, field) in fields {
            let offset = cursor.align_up(&field.alignment)?;
            cursor = offset.add(&field.size);

            field_layouts.push(FieldLayout {
                name,
                offset,
                layout: Box::new(field),
            });
        }

        let size = cursor.align_up(&aggregate_alignment)?;
        let stride = size.align_up(&aggregate_alignment)?;

        Ok(TypeLayout {
            kind: LayoutKind::Record,
            size,
            alignment: aggregate_alignment,
            stride,
            fields: field_layouts,
        })
    }

    /// Lays out a fixed-length or symbolically sized array.
    ///
    /// Array size is `element_stride * length`; element stride is the
    /// element size rounded up to its alignment. Symbolic dimensions remain
    /// symbolic. No conversion to a fixed host-sized length is performed.
    pub fn array(
        &self,
        element: TypeLayout,
        length: LayoutValue,
    ) -> Result<TypeLayout, LayoutError> {
        TypeLayout::validate_alignment(&element.alignment)?;

        if matches!(length, LayoutValue::Unsized) {
            return Err(LayoutError::InvalidArrayLength);
        }

        if matches!(element.size, LayoutValue::Unsized) {
            return Err(LayoutError::UnsizedArrayElement);
        }

        let stride = element.size.align_up(&element.alignment)?;
        let size = stride.multiply(&length);

        Ok(TypeLayout {
            kind: LayoutKind::Array { length },
            size,
            alignment: element.alignment,
            stride,
            fields: Vec::new(),
        })
    }
}

/// Errors produced by local layout validation or composition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LayoutError {
    /// A decimal constant contains no digits or contains non-decimal bytes.
    InvalidDecimalConstant,

    /// A symbolic identifier is empty or whitespace-only.
    EmptySymbol,

    /// A concrete alignment was zero.
    ZeroAlignment,

    /// An alignment is unsized or cannot be used as an alignment value.
    UnsizedAlignment,

    /// A record has no field alignment and no explicit target policy.
    EmptyRecordNeedsPolicy,

    /// A dynamically sized field requires an explicit target-specific rule.
    UnsizedAggregateMember,

    /// A dynamically sized element cannot be represented by this fixed-array
    /// composition rule.
    UnsizedArrayElement,

    /// Array length has an invalid state.
    InvalidArrayLength,

    /// A symbolic expression could not be constructed from its inputs.
    InvalidLayoutExpression,

    /// A maximum-alignment operation was requested with no values.
    EmptyAlignmentSet,
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDecimalConstant => {
                f.write_str("layout constant must be a non-negative decimal integer")
            }
            Self::EmptySymbol => f.write_str("layout symbol must not be empty"),
            Self::ZeroAlignment => f.write_str("alignment must be greater than zero"),
            Self::UnsizedAlignment => f.write_str("alignment must have a usable size"),
            Self::EmptyRecordNeedsPolicy => {
                f.write_str("empty record layout requires an explicit target policy")
            }
            Self::UnsizedAggregateMember => {
                f.write_str("unsized record member requires an explicit layout policy")
            }
            Self::UnsizedArrayElement => {
                f.write_str("unsized array element requires an explicit layout policy")
            }
            Self::InvalidArrayLength => {
                f.write_str("array length must be known or symbolically represented")
            }
            Self::InvalidLayoutExpression => {
                f.write_str("layout expression could not be represented")
            }
            Self::EmptyAlignmentSet => {
                f.write_str("cannot compute maximum alignment from an empty set")
            }
        }
    }
}

impl std::error::Error for LayoutError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn scalar(size: usize, alignment: usize) -> TypeLayout {
        TypeLayout::scalar(
            LayoutValue::Known(size),
            LayoutValue::Known(alignment),
        )
        .expect("valid test scalar layout")
    }

    #[test]
    fn scalar_layout_preserves_size_alignment_and_stride() {
        let layout = scalar(3, 2);

        assert_eq!(layout.size(), &LayoutValue::Known(3));
        assert_eq!(layout.alignment(), &LayoutValue::Known(2));
        assert_eq!(layout.stride(), &LayoutValue::Known(4));
        assert!(layout.is_fully_concrete());
    }

    #[test]
    fn zero_alignment_is_rejected() {
        assert_eq!(
            TypeLayout::scalar(LayoutValue::Known(4), LayoutValue::Known(0)),
            Err(LayoutError::ZeroAlignment)
        );
    }

    #[test]
    fn record_fields_receive_checked_offsets_and_tail_padding() {
        let engine = LayoutEngine;
        let record = engine
            .record(
                vec![
                    (Some("a".to_owned()), scalar(1, 1)),
                    (Some("b".to_owned()), scalar(4, 4)),
                    (Some("c".to_owned()), scalar(2, 2)),
                ],
                None,
            )
            .expect("record layout should be valid");

        assert_eq!(record.fields()[0].offset(), &LayoutValue::Known(0));
        assert_eq!(record.fields()[1].offset(), &LayoutValue::Known(4));
        assert_eq!(record.fields()[2].offset(), &LayoutValue::Known(8));
        assert_eq!(record.alignment(), &LayoutValue::Known(4));
        assert_eq!(record.size(), &LayoutValue::Known(12));
    }

    #[test]
    fn symbolic_field_size_produces_symbolic_offsets_and_size() {
        let symbolic_size = LayoutValue::symbolic(
            LayoutExpression::symbol("N").expect("non-empty symbol"),
        );
        let symbolic_field =
            TypeLayout::scalar(symbolic_size, LayoutValue::Known(1))
                .expect("symbolic scalar layout");

        let record = LayoutEngine
            .record(
                vec![
                    (Some("first".to_owned()), scalar(2, 2)),
                    (Some("second".to_owned()), symbolic_field),
                ],
                None,
            )
            .expect("symbolic layouts should be retained");

        assert_eq!(record.fields()[1].offset(), &LayoutValue::Known(2));
        assert!(matches!(record.size(), LayoutValue::Symbolic(_)));
        assert!(!record.is_fully_concrete());
    }

    #[test]
    fn host_overflow_becomes_symbolic_instead_of_wrapping() {
        let value = LayoutValue::Known(usize::MAX).add(&LayoutValue::Known(1));

        assert!(matches!(value, LayoutValue::Symbolic(_)));
    }

    #[test]
    fn host_multiplication_overflow_becomes_symbolic() {
        let value =
            LayoutValue::Known(usize::MAX).multiply(&LayoutValue::Known(2));

        assert!(matches!(value, LayoutValue::Symbolic(_)));
    }

    #[test]
    fn symbolic_array_length_is_not_narrowed_to_usize() {
        let length = LayoutValue::symbolic(
            LayoutExpression::symbol("N").expect("non-empty symbol"),
        );

        let array = LayoutEngine
            .array(scalar(4, 4), length.clone())
            .expect("symbolic array layout");

        assert_eq!(
            array.kind(),
            &LayoutKind::Array { length }
        );
        assert!(matches!(array.size(), LayoutValue::Symbolic(_)));
        assert_eq!(array.stride(), &LayoutValue::Known(4));
    }

    #[test]
    fn unsized_array_length_is_rejected() {
        assert_eq!(
            LayoutEngine.array(scalar(4, 4), LayoutValue::Unsized),
            Err(LayoutError::InvalidArrayLength)
        );
    }

    #[test]
    fn unsized_record_member_is_not_guessed() {
        let unsized = TypeLayout::unsized(LayoutValue::Known(1))
            .expect("valid unsized layout");

        assert_eq!(
            LayoutEngine.record(vec![(None, unsized)], None),
            Err(LayoutError::UnsizedAggregateMember)
        );
    }

    #[test]
    fn decimal_constants_are_normalized_without_narrowing() {
        let expression =
            LayoutExpression::constant_decimal("000123456789012345678901234567890")
                .expect("decimal constant");

        assert_eq!(
            expression,
            LayoutExpression::Constant("123456789012345678901234567890".to_owned())
        );
    }

    #[test]
    fn invalid_decimal_constants_are_rejected() {
        assert_eq!(
            LayoutExpression::constant_decimal("12x"),
            Err(LayoutError::InvalidDecimalConstant)
        );
    }

    #[test]
    fn alignment_does_not_assume_power_of_two() {
        let aligned = LayoutValue::Known(10)
            .align_up(&LayoutValue::Known(3))
            .expect("positive alignment");

        assert_eq!(aligned, LayoutValue::Known(12));
    }
}