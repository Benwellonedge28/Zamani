
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]

//! # Zamani HDL Aggregate Types
//!
//! Path: `src/hdl/types/aggregate.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Provides a validated HDL-facing adapter for canonical aggregate type
//! definitions: tuples, arrays, records, sum types, and unions.
//!
//! This is an HDL integration boundary, not a second type system. The
//! canonical representation and local structural validation remain owned by
//! `crate::zuir::types`.
//!
//! ## Authority and ownership
//!
//! Authoritative contracts:
//!
//! - `grammar/spec/hdl.md`
//! - `grammar/spec/user-defined-types.md`
//! - `grammar/spec/type-system.md`
//! - `grammar/types/` and `grammar/declarations/`
//! - `src/frontend/ast/node/types/`
//! - `src/frontend/semantic/types/`
//! - `src/frontend/semantic/domains/hdl.rs`
//! - `src/zuir/types/type_def.rs`
//!
//! The grammar owns source syntax. The frontend AST owns parsed type
//! expressions. Semantic analysis resolves source-level names and types.
//! ZUIR owns the canonical `TypeKind`, `TypeRef`, `Dimension`, `FieldDef`,
//! `VariantDef`, and `TypeDef` definitions.
//!
//! This file owns only:
//!
//! - HDL aggregate classification;
//! - a validated adapter around a canonical aggregate `TypeKind`;
//! - aggregate-specific read-only accessors;
//! - error reporting for adapter construction and validation.
//!
//! ## Does not own
//!
//! - Source parsing, AST construction, or type-name resolution.
//! - A competing aggregate or semantic type representation.
//! - Type identity allocation, interning, or registry membership.
//! - Generic instantiation or symbolic expression evaluation.
//! - HDL signal values, constant evaluation, or runtime storage.
//! - Logical/physical layout, ABI, packing, or alignment.
//! - Hardware allocation, target feasibility, synthesis, or lowering.
//! - HDLIR. All HDLIR implementation remains under `src/hdl/hdlir/`.
//!
//! ## Validation contract
//!
//! Construction first checks that the supplied `TypeKind` is an aggregate,
//! then delegates structural validation to canonical `TypeDef::new()`.
//! A private validation name is used only to invoke the canonical validator;
//! it is not returned, registered, or treated as the aggregate's identity.
//!
//! Canonical validation checks local invariants, including duplicate field
//! or variant names, invalid identifiers, and malformed symbolic expressions.
//! It does not prove that named references resolve or that a complete program
//! is semantically valid. Those checks remain with the canonical resolver,
//! registry, and semantic pipeline.
//!
//! ## Scalability and POCO-REAF
//!
//! No universal maximum is imposed on tuple arity, array length, record
//! fields, sum variants, or union alternatives. Canonical collections and
//! their declaration order are preserved.
//!
//! Array dimensions remain canonical `Known(u128)`, `Symbolic`, or `Dynamic`
//! values. They are never converted here to `usize`, used to allocate an
//! array, or silently narrowed to a host-dependent value.
//!
//! A concrete Rust collection and the compiler process remain finite. Any
//! operational budget belongs to the owning configuration, validation,
//! decoding, or execution boundary; it must not silently change type meaning.
//!
//! ## Integration contract
//!
//! - `src/hdl/types/mod.rs` declares `pub mod aggregate;` and deliberately
//!   re-exports this module's public API.
//! - Frontend semantic analysis resolves source declarations before consumers
//!   rely on named-type identity or whole-program type correctness.
//! - Canonical ZUIR types remain the source of truth.
//! - Future HDL type analysis may consume this adapter without depending on
//!   HDLIR or a target backend.
//! - Layout and realization consumers must use their own explicit target
//!   inputs and must not infer physical layout from this descriptor.
//!
//! ## Freeze criteria
//!
//! Freeze after registering the module, compiling against the repository's
//! canonical APIs, passing these tests, and running the relevant workspace
//! checks. API changes must preserve canonical ownership and be reviewed for
//! downstream compatibility.

use core::fmt;

use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::{
    Dimension, FieldDef, TypeDef, TypeDefError, TypeKind, TypeRef, VariantDef,
};

/// Classification of canonical aggregate type forms.
///
/// This classification is descriptive metadata. It does not replace the
/// canonical `TypeKind` variant or define independent type semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AggregateKind {
    /// An ordered product of potentially heterogeneous element types.
    Tuple,

    /// A sequence with an element type and known, symbolic, or dynamic length.
    Array,

    /// An ordered collection of named, typed fields.
    Record,

    /// A type with one selected variant from a declared set.
    Sum,

    /// A type with one selected alternative from a declared set.
    Union,
}

/// Validated adapter around a canonical aggregate type.
///
/// The descriptor preserves the canonical definition and its order. It does
/// not create a new type identity, resolve names, compute layout, or allocate
/// aggregate values.
///
/// Use [`AggregateTypeDescriptor::new`] to construct a descriptor from an
/// owned canonical type, or [`AggregateTypeDescriptor::try_from_kind`] to
/// adapt a borrowed canonical type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregateTypeDescriptor {
    kind: TypeKind,
    aggregate_kind: AggregateKind,
}

impl AggregateTypeDescriptor {
    /// Constructs a validated descriptor from a canonical aggregate type.
    ///
    /// The canonical type is retained without reordering members, resolving
    /// references, narrowing dimensions, or materializing storage.
    ///
    /// # Errors
    ///
    /// Returns [`AggregateTypeError::NotAggregate`] if `kind` is not one of
    /// the supported aggregate variants.
    ///
    /// Returns [`AggregateTypeError::InvalidCanonicalType`] if the canonical
    /// validator rejects the type or any nested inline type.
    pub fn new(kind: TypeKind) -> Result<Self, AggregateTypeError> {
        let aggregate_kind =
            Self::classify(&kind).ok_or(AggregateTypeError::NotAggregate)?;

        validate_canonical_type(&kind)?;

        Ok(Self {
            kind,
            aggregate_kind,
        })
    }

    /// Adapts a borrowed canonical type.
    ///
    /// The descriptor owns a clone of the supplied `TypeKind`; no canonical
    /// type identity is created or modified.
    ///
    /// # Errors
    ///
    /// Returns an error if the type is not an aggregate or canonical local
    /// validation rejects it.
    pub fn try_from_kind(kind: &TypeKind) -> Result<Self, AggregateTypeError> {
        Self::new(kind.clone())
    }

    /// Returns the underlying canonical aggregate type.
    #[must_use]
    pub const fn kind(&self) -> &TypeKind {
        &self.kind
    }

    /// Returns the aggregate classification established during construction.
    ///
    /// The classification is stored with the private canonical type so this
    /// accessor does not need a panic or a misleading fallback value.
    #[must_use]
    pub const fn aggregate_kind(&self) -> AggregateKind {
        self.aggregate_kind
    }

    /// Returns the tuple's element type references, if this is a tuple.
    ///
    /// Element order is preserved.
    #[must_use]
    pub fn tuple_elements(&self) -> Option<&[TypeRef]> {
        match &self.kind {
            TypeKind::Tuple { elements } => Some(elements),
            _ => None,
        }
    }

    /// Returns the array's element type and dimension, if this is an array.
    ///
    /// The dimension remains a canonical `Dimension`. This accessor does not
    /// convert it to a host index or allocate an array.
    #[must_use]
    pub fn array_parts(&self) -> Option<(&TypeRef, &Dimension)> {
        match &self.kind {
            TypeKind::Array { element, length } => Some((element, length)),
            _ => None,
        }
    }

    /// Returns the array element type reference, if this is an array.
    #[must_use]
    pub fn array_element(&self) -> Option<&TypeRef> {
        match &self.kind {
            TypeKind::Array { element, .. } => Some(element),
            _ => None,
        }
    }

    /// Returns the array dimension, if this is an array.
    ///
    /// `Known`, `Symbolic`, and `Dynamic` are preserved as defined by the
    /// canonical type model.
    #[must_use]
    pub fn array_dimension(&self) -> Option<&Dimension> {
        match &self.kind {
            TypeKind::Array { length, .. } => Some(length),
            _ => None,
        }
    }

    /// Returns record fields, if this is a record.
    ///
    /// Declaration order, field types, and field attributes are preserved.
    #[must_use]
    pub fn record_fields(&self) -> Option<&[FieldDef]> {
        match &self.kind {
            TypeKind::Record { fields } => Some(fields),
            _ => None,
        }
    }

    /// Returns sum-type variants, if this is a sum type.
    ///
    /// Variant order, field order, and attributes are preserved.
    #[must_use]
    pub fn sum_variants(&self) -> Option<&[VariantDef]> {
        match &self.kind {
            TypeKind::Sum { variants } => Some(variants),
            _ => None,
        }
    }

    /// Returns union alternatives, if this is a union.
    ///
    /// Alternative order is preserved. This method does not determine whether
    /// a target uses a tagged, untagged, or other representation.
    #[must_use]
    pub fn union_alternatives(&self) -> Option<&[TypeRef]> {
        match &self.kind {
            TypeKind::Union { alternatives } => Some(alternatives),
            _ => None,
        }
    }

    /// Revalidates the descriptor through canonical type validation.
    ///
    /// Use this at a trust boundary if canonical values may have been
    /// deserialized or otherwise constructed outside validated APIs.
    ///
    /// # Errors
    ///
    /// Returns an error if canonical validation rejects the aggregate.
    pub fn validate(&self) -> Result<(), AggregateTypeError> {
        let actual_kind =
            Self::classify(&self.kind).ok_or(AggregateTypeError::NotAggregate)?;

        // This mismatch should be impossible through the public API because
        // both fields are private and initialized together in `new()`.
        // Still report it rather than silently accepting inconsistent state.
        if actual_kind != self.aggregate_kind {
            return Err(AggregateTypeError::InconsistentClassification {
                expected: self.aggregate_kind,
                actual: actual_kind,
            });
        }

        validate_canonical_type(&self.kind)
    }

    /// Converts the descriptor into its canonical `TypeKind`.
    ///
    /// This conversion is lossless and does not create a type identity.
    #[must_use]
    pub fn into_type_kind(self) -> TypeKind {
        self.kind
    }

    /// Returns an owned copy of the canonical `TypeKind`.
    #[must_use]
    pub fn to_type_kind(&self) -> TypeKind {
        self.kind.clone()
    }

    /// Classifies a canonical type if it is an aggregate.
    ///
    /// This operation is structural only; it does not validate the supplied
    /// type or resolve any named references.
    #[must_use]
    pub fn classify(kind: &TypeKind) -> Option<AggregateKind> {
        match kind {
            TypeKind::Tuple { .. } => Some(AggregateKind::Tuple),
            TypeKind::Array { .. } => Some(AggregateKind::Array),
            TypeKind::Record { .. } => Some(AggregateKind::Record),
            TypeKind::Sum { .. } => Some(AggregateKind::Sum),
            TypeKind::Union { .. } => Some(AggregateKind::Union),
            _ => None,
        }
    }
}

impl TryFrom<&TypeKind> for AggregateTypeDescriptor {
    type Error = AggregateTypeError;

    fn try_from(kind: &TypeKind) -> Result<Self, Self::Error> {
        Self::try_from_kind(kind)
    }
}

/// Validates a canonical aggregate by delegating to the existing ZUIR
/// validator.
///
/// The temporary name exists only to satisfy `TypeDef::new()`'s requirement
/// for a named definition. It is never exposed as the aggregate's identity.
fn validate_canonical_type(kind: &TypeKind) -> Result<(), AggregateTypeError> {
    let validation_name = QualifiedName::parse("hdl::AggregateValidationProbe")
        .map_err(|error| {
            AggregateTypeError::InvalidValidationName(error.to_string())
        })?;

    TypeDef::new(validation_name, kind.clone())
        .map(|_| ())
        .map_err(AggregateTypeError::InvalidCanonicalType)
}

/// Errors raised when adapting or validating canonical aggregate types.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AggregateTypeError {
    /// The supplied canonical type is not a supported aggregate.
    NotAggregate,

    /// The internal validation name could not be constructed.
    ///
    /// This indicates an unexpected failure in the canonical qualified-name
    /// implementation, rather than a malformed aggregate declaration.
    InvalidValidationName(String),

    /// The canonical ZUIR validator rejected the type.
    InvalidCanonicalType(TypeDefError),

    /// The descriptor's stored classification disagrees with its type.
    ///
    /// This is defensive validation for an internal invariant. The public API
    /// does not expose mutable access to either field.
    InconsistentClassification {
        /// Classification stored by the descriptor.
        expected: AggregateKind,

        /// Classification derived from the canonical type.
        actual: AggregateKind,
    },
}

impl fmt::Display for AggregateTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAggregate => {
                f.write_str("canonical type is not an aggregate type")
            }
            Self::InvalidValidationName(message) => {
                write!(f, "cannot construct aggregate validation name: {message}")
            }
            Self::InvalidCanonicalType(error) => {
                write!(f, "invalid canonical aggregate type: {error}")
            }
            Self::InconsistentClassification { expected, actual } => {
                write!(
                    f,
                    "inconsistent aggregate classification: stored {expected:?}, \
                     canonical type is {actual:?}"
                )
            }
        }
    }
}

impl std::error::Error for AggregateTypeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidCanonicalType(error) => Some(error),
            Self::NotAggregate
            | Self::InvalidValidationName(_)
            | Self::InconsistentClassification { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::types::{
        FieldDef, IntegerSignedness, IntegerWidth, SymbolicExpression, TypeKind,
        TypeRef, VariantDef,
    };
    use std::collections::BTreeMap;

    fn qualified_name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test qualified name should be valid")
    }

    fn named_type(value: &str) -> TypeRef {
        TypeRef::Named(qualified_name(value))
    }

    fn empty_attributes() -> BTreeMap<String, crate::zuir::types::TypeAttribute> {
        BTreeMap::new()
    }

    #[test]
    fn classifies_all_canonical_aggregate_variants() {
        let cases = [
            (
                TypeKind::Tuple {
                    elements: vec![named_type("core::Integer")],
                },
                AggregateKind::Tuple,
            ),
            (
                TypeKind::Array {
                    element: named_type("core::Integer"),
                    length: Dimension::Known(3),
                },
                AggregateKind::Array,
            ),
            (
                TypeKind::Record {
                    fields: vec![FieldDef {
                        name: "value".to_owned(),
                        ty: named_type("core::Integer"),
                        attributes: empty_attributes(),
                    }],
                },
                AggregateKind::Record,
            ),
            (
                TypeKind::Sum {
                    variants: vec![VariantDef {
                        name: "Some".to_owned(),
                        fields: vec![],
                        attributes: empty_attributes(),
                    }],
                },
                AggregateKind::Sum,
            ),
            (
                TypeKind::Union {
                    alternatives: vec![named_type("core::Integer")],
                },
                AggregateKind::Union,
            ),
        ];

        for (kind, expected) in cases {
            assert_eq!(AggregateTypeDescriptor::classify(&kind), Some(expected));

            let descriptor =
                AggregateTypeDescriptor::new(kind).expect("valid aggregate");
            assert_eq!(descriptor.aggregate_kind(), expected);
            descriptor.validate().expect("descriptor should validate");
        }
    }

    #[test]
    fn rejects_non_aggregate_types() {
        let error = AggregateTypeDescriptor::new(TypeKind::Boolean)
            .expect_err("Boolean is not an aggregate");

        assert_eq!(error, AggregateTypeError::NotAggregate);
    }

    #[test]
    fn preserves_tuple_order() {
        let elements = vec![
            named_type("core::Integer"),
            named_type("core::Boolean"),
            named_type("core::String"),
        ];
        let descriptor = AggregateTypeDescriptor::new(TypeKind::Tuple {
            elements: elements.clone(),
        })
        .expect("tuple should validate");

        assert_eq!(descriptor.tuple_elements(), Some(elements.as_slice()));
        assert_eq!(descriptor.to_type_kind(), TypeKind::Tuple { elements });
    }

    #[test]
    fn preserves_known_symbolic_and_dynamic_array_dimensions() {
        let dimensions = [
            Dimension::Known(17),
            Dimension::Symbolic(SymbolicExpression {
                expression: "element_count".to_owned(),
            }),
            Dimension::Dynamic,
        ];

        for dimension in dimensions {
            let descriptor = AggregateTypeDescriptor::new(TypeKind::Array {
                element: named_type("core::Integer"),
                length: dimension.clone(),
            })
            .expect("array dimension should be preserved");

            assert_eq!(
                descriptor.array_dimension(),
                Some(&dimension),
            );
        }
    }

    #[test]
    fn preserves_record_field_order() {
        let fields = vec![
            FieldDef {
                name: "first".to_owned(),
                ty: named_type("core::Integer"),
                attributes: empty_attributes(),
            },
            FieldDef {
                name: "second".to_owned(),
                ty: named_type("core::Boolean"),
                attributes: empty_attributes(),
            },
        ];
        let descriptor = AggregateTypeDescriptor::new(TypeKind::Record {
            fields: fields.clone(),
        })
        .expect("record should validate");

        assert_eq!(descriptor.record_fields(), Some(fields.as_slice()));
    }

    #[test]
    fn preserves_sum_variant_and_payload_order() {
        let variants = vec![
            VariantDef {
                name: "Empty".to_owned(),
                fields: vec![],
                attributes: empty_attributes(),
            },
            VariantDef {
                name: "Value".to_owned(),
                fields: vec![
                    FieldDef {
                        name: "left".to_owned(),
                        ty: named_type("core::Integer"),
                        attributes: empty_attributes(),
                    },
                    FieldDef {
                        name: "right".to_owned(),
                        ty: named_type("core::Boolean"),
                        attributes: empty_attributes(),
                    },
                ],
                attributes: empty_attributes(),
            },
        ];

        let descriptor = AggregateTypeDescriptor::new(TypeKind::Sum {
            variants: variants.clone(),
        })
        .expect("sum should validate");

        assert_eq!(descriptor.sum_variants(), Some(variants.as_slice()));
    }

    #[test]
    fn rejects_duplicate_record_fields_through_canonical_validation() {
        let duplicate_fields = vec![
            FieldDef {
                name: "value".to_owned(),
                ty: named_type("core::Integer"),
                attributes: empty_attributes(),
            },
            FieldDef {
                name: "value".to_owned(),
                ty: named_type("core::Boolean"),
                attributes: empty_attributes(),
            },
        ];

        let error = AggregateTypeDescriptor::new(TypeKind::Record {
            fields: duplicate_fields,
        })
        .expect_err("duplicate field names must be rejected");

        assert!(matches!(
            error,
            AggregateTypeError::InvalidCanonicalType(
                TypeDefError::DuplicateName {
                    category: "record field",
                    ..
                }
            )
        ));
    }

    #[test]
    fn rejects_empty_union_through_canonical_validation() {
        let error = AggregateTypeDescriptor::new(TypeKind::Union {
            alternatives: vec![],
        })
        .expect_err("empty union must be rejected");

        assert!(matches!(
            error,
            AggregateTypeError::InvalidCanonicalType(TypeDefError::EmptyUnion)
        ));
    }

    #[test]
    fn rejects_malformed_symbolic_dimension() {
        let error = AggregateTypeDescriptor::new(TypeKind::Array {
            element: named_type("core::Integer"),
            length: Dimension::Symbolic(SymbolicExpression {
                expression: "   ".to_owned(),
            }),
        })
        .expect_err("empty symbolic expression must be rejected");

        assert!(matches!(
            error,
            AggregateTypeError::InvalidCanonicalType(
                TypeDefError::EmptySymbolicExpression
            )
        ));
    }

    #[test]
    fn preserves_canonical_integer_types_inside_aggregates() {
        let inline_integer = TypeRef::Inline(Box::new(TypeKind::Integer {
            signedness: IntegerSignedness::Unsigned,
            width: IntegerWidth::Fixed(257),
        }));

        let descriptor = AggregateTypeDescriptor::new(TypeKind::Tuple {
            elements: vec![inline_integer.clone()],
        })
        .expect("wide integer tuple should validate");

        assert_eq!(
            descriptor.tuple_elements(),
            Some(std::slice::from_ref(&inline_integer)),
        );
    }

    #[test]
    fn borrowed_conversion_preserves_the_canonical_type() {
        let kind = TypeKind::Array {
            element: named_type("core::Integer"),
            length: Dimension::Symbolic(SymbolicExpression {
                expression: "N * M".to_owned(),
            }),
        };

        let descriptor =
            AggregateTypeDescriptor::try_from_kind(&kind).expect("valid array");

        assert_eq!(descriptor.to_type_kind(), kind);
        assert_eq!(descriptor.clone().into_type_kind(), kind);
    }

    #[test]
    fn accessors_return_none_for_other_aggregate_categories() {
        let descriptor = AggregateTypeDescriptor::new(TypeKind::Tuple {
            elements: vec![],
        })
        .expect("empty tuple should be accepted by canonical validation");

        assert!(descriptor.array_parts().is_none());
        assert!(descriptor.array_element().is_none());
        assert!(descriptor.array_dimension().is_none());
        assert!(descriptor.record_fields().is_none());
        assert!(descriptor.sum_variants().is_none());
        assert!(descriptor.union_alternatives().is_none());
    }
}
