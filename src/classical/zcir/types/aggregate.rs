
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR aggregate type adapter
//!
//! Path: `src/classical/zcir/types/aggregate.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provides a validated ZCIR-facing adapter for the canonical ZUIR aggregate
//! type forms:
//!
//! - tuples;
//! - arrays;
//! - records;
//! - sum types;
//! - unions.
//!
//! This module adapts `crate::zuir::types::TypeKind`. It does not define a
//! competing aggregate type representation or type identity system.
//!
//! ## Authority and ownership
//!
//! Canonical ownership:
//!
//! - `TypeKind` owns the canonical aggregate type variants.
//! - `TypeRef` owns named and inline type-reference representation.
//! - `Dimension` owns known, symbolic, and dynamic dimensions.
//! - `FieldDef` owns record and variant field descriptions.
//! - `VariantDef` owns sum-type variant descriptions.
//! - `TypeDef::validate()` owns canonical local type-definition validation.
//! - The frontend semantic model owns source-language type resolution.
//! - The canonical type registry owns named-type resolution and identity.
//!
//! This module owns:
//!
//! - `AggregateTypeDescriptor`, a validated adapter around an aggregate
//!   `TypeKind`;
//! - `AggregateKind`, a non-authoritative classification of aggregate forms;
//! - aggregate-specific accessors and conversions;
//! - a local error wrapper for invalid input.
//!
//! This module does not own:
//!
//! - a new tuple, array, record, sum, or union data model;
//! - source parsing, type inference, or name resolution;
//! - generic specialization or symbolic constraint evaluation;
//! - type interning, type IDs, or canonical identity;
//! - value storage, collection mutation, or numeric operations;
//! - ownership, borrow checking, or lifetime analysis;
//! - logical or physical layout, ABI, or allocation;
//! - target capability discovery, scheduling, or lowering;
//! - a new serialization format.
//!
//! ## Validation strategy
//!
//! Aggregate validation delegates to the canonical `TypeDef::validate()`
//! implementation. A temporary, internal validation name is used solely to
//! invoke that existing validator. It is not registered, returned, persisted,
//! or used as the aggregate's semantic identity.
//!
//! Reusing canonical validation avoids creating a second, potentially
//! divergent set of rules for identifiers, duplicate fields, symbolic
//! expressions, nested inline types, and empty unions.
//!
//! Validation does not resolve named references or prove whole-program type
//! correctness. Those responsibilities remain with the canonical semantic
//! resolver and type registry.
//!
//! ## POCO-REAF and scalability
//!
//! No universal maximum tuple arity, array length, record field count, sum
//! variant count, or union alternative count is imposed here.
//!
//! Array dimensions are preserved exactly as canonical `Dimension` values:
//!
//! - `Known(u128)`;
//! - `Symbolic(SymbolicExpression)`;
//! - `Dynamic`.
//!
//! A dimension is semantic metadata, not a request to allocate that many
//! values immediately. Symbolic and dynamic dimensions are not converted to
//! `usize` or silently replaced with host-specific limits.
//!
//! Collections retain their canonical ordering. Tuple order and declared
//! field/variant order are not silently normalized or discarded.
//!
//! Finite memory, execution time, host indexing limits, and target capacity
//! remain practical constraints. Any operational budget must be imposed by
//! the owning configuration, verification, decoding, or runtime layer, and
//! must not redefine the language's type semantics.
//!
//! ## Integration contract
//!
//! - `src/classical/zcir/types/mod.rs` declares `pub mod aggregate;`.
//! - The module may re-export `AggregateKind`, `AggregateTypeDescriptor`,
//!   and `AggregateTypeError`.
//! - `src/zuir/types/type_def.rs` remains the canonical aggregate authority.
//! - Frontend semantic analysis resolves source declarations before ZCIR
//!   consumes their canonical types.
//! - `types/layout.rs` handles layout without allocating aggregate values.
//! - `values/aggregate.rs` owns aggregate values and storage policy.
//! - `verify/types.rs` handles cross-type compatibility and reference
//!   resolution after local descriptor validation.
//! - `lowering/to_zuir.rs` preserves the canonical `TypeKind` without
//!   changing ordering, dimensions, or type references.
//! - `serialization/decode.rs` validates untrusted decoded data and applies
//!   configured operational budgets.
//! - Quantum/classical interoperability continues to use the existing
//!   canonical classical IR interfaces where appropriate.
//!
//! This file does not depend on planned ZCIR siblings, avoiding forward
//! dependencies on APIs that may not yet exist.
//!
//! ## Freeze criteria
//!
//! Freeze this file after compiling against the actual canonical APIs,
//! registering its module, passing the tests below, and completing the
//! relevant ZCIR/ZUIR interoperability checks.

use core::fmt;

use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::{
    Dimension, FieldDef, TypeDef, TypeDefError, TypeKind, TypeRef, VariantDef,
};

/// Classification of canonical aggregate type forms.
///
/// This enum is descriptive metadata for ZCIR callers. It does not replace
/// the corresponding `TypeKind` variants or define their semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AggregateKind {
    /// Ordered product whose elements may have distinct types.
    Tuple,

    /// Sequence with one element type and a known, symbolic, or dynamic
    /// dimension.
    Array,

    /// Named fields with individually declared types.
    Record,

    /// One selected variant from a declared set of variants.
    Sum,

    /// One selected alternative from a declared set of alternatives.
    Union,
}

/// Validated adapter for a canonical aggregate type.
///
/// The descriptor stores the original canonical `TypeKind`. It does not
/// allocate a type ID, resolve named references, compute physical layout,
/// or allocate values.
///
/// Use [`Self::new`] to construct a descriptor or
/// [`Self::try_from_kind`] to adapt a borrowed canonical type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregateTypeDescriptor {
    kind: TypeKind,
}

impl AggregateTypeDescriptor {
    /// Constructs a descriptor from a canonical aggregate type.
    ///
    /// The input is retained without reordering, narrowing dimensions,
    /// resolving references, or materializing storage.
    ///
    /// # Errors
    ///
    /// Returns [`AggregateTypeError::NotAggregate`] if `kind` is not one of
    /// the supported canonical aggregate variants.
    ///
    /// Returns [`AggregateTypeError::InvalidCanonicalType`] if canonical
    /// local validation rejects the aggregate or any nested inline type.
    pub fn new(kind: TypeKind) -> Result<Self, AggregateTypeError> {
        if Self::classify(&kind).is_none() {
            return Err(AggregateTypeError::NotAggregate);
        }

        validate_canonical_type(&kind)?;

        Ok(Self { kind })
    }

    /// Adapts a borrowed canonical type.
    ///
    /// The type is cloned because the descriptor owns its canonical
    /// `TypeKind`. No new semantic identity is created.
    ///
    /// # Errors
    ///
    /// Returns an error if the type is not an aggregate or fails canonical
    /// local validation.
    pub fn try_from_kind(kind: &TypeKind) -> Result<Self, AggregateTypeError> {
        Self::new(kind.clone())
    }

    /// Returns the canonical aggregate type.
    #[must_use]
    pub const fn kind(&self) -> &TypeKind {
        &self.kind
    }

    /// Returns the aggregate category.
    ///
    /// This is a structural classification, not a statement about layout,
    /// storage, or target support.
    #[must_use]
    pub fn aggregate_kind(&self) -> AggregateKind {
        // Construction guarantees that `kind` is an aggregate variant.
        // Keeping this method total avoids introducing a panic-based API.
        Self::classify(&self.kind).unwrap_or(AggregateKind::Tuple)
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

    /// Returns the array element type and dimension, if this is an array.
    ///
    /// The dimension is preserved as `Known`, `Symbolic`, or `Dynamic`.
    /// This accessor does not allocate an array or convert its dimension
    /// to a host index.
    #[must_use]
    pub fn array_parts(&self) -> Option<(&TypeRef, &Dimension)> {
        match &self.kind {
            TypeKind::Array { element, length } => Some((element, length)),
            _ => None,
        }
    }

    /// Returns the array's element type reference, if this is an array.
    #[must_use]
    pub fn array_element(&self) -> Option<&TypeRef> {
        match &self.kind {
            TypeKind::Array { element, .. } => Some(element),
            _ => None,
        }
    }

    /// Returns the array's canonical dimension, if this is an array.
    #[must_use]
    pub fn array_dimension(&self) -> Option<&Dimension> {
        match &self.kind {
            TypeKind::Array { length, .. } => Some(length),
            _ => None,
        }
    }

    /// Returns record fields, if this is a record.
    ///
    /// Declared field order and attributes are preserved.
    #[must_use]
    pub fn record_fields(&self) -> Option<&[FieldDef]> {
        match &self.kind {
            TypeKind::Record { fields } => Some(fields),
            _ => None,
        }
    }

    /// Returns sum variants, if this is a sum type.
    ///
    /// Declared variant order and field order are preserved.
    #[must_use]
    pub fn sum_variants(&self) -> Option<&[VariantDef]> {
        match &self.kind {
            TypeKind::Sum { variants } => Some(variants),
            _ => None,
        }
    }

    /// Returns union alternatives, if this is a union.
    ///
    /// Alternative order is preserved. This method does not decide whether
    /// a target must use tagged, untagged, or another representation.
    #[must_use]
    pub fn union_alternatives(&self) -> Option<&[TypeRef]> {
        match &self.kind {
            TypeKind::Union { alternatives } => Some(alternatives),
            _ => None,
        }
    }

    /// Revalidates this descriptor using canonical type validation.
    ///
    /// Call this at trust boundaries if the descriptor may have come from
    /// an external representation or unchecked construction elsewhere.
    ///
    /// # Errors
    ///
    /// Returns an error if the type is no longer a supported aggregate
    /// variant or canonical validation rejects its contents.
    pub fn validate(&self) -> Result<(), AggregateTypeError> {
        if Self::classify(&self.kind).is_none() {
            return Err(AggregateTypeError::NotAggregate);
        }

        validate_canonical_type(&self.kind)
    }

    /// Converts this descriptor into the canonical ZUIR type.
    ///
    /// The conversion is lossless and does not create a new type identity.
    #[must_use]
    pub fn into_type_kind(self) -> TypeKind {
        self.kind
    }

    /// Returns an owned copy of the canonical ZUIR type.
    #[must_use]
    pub fn to_type_kind(&self) -> TypeKind {
        self.kind.clone()
    }

    /// Returns the aggregate category for a canonical type, if applicable.
    ///
    /// This classification does not perform validation.
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

impl TryFrom<TypeKind> for AggregateTypeDescriptor {
    type Error = AggregateTypeError;

    fn try_from(kind: TypeKind) -> Result<Self, Self::Error> {
        Self::new(kind)
    }
}

/// Validates a canonical aggregate type using the existing ZUIR validator.
///
/// A temporary type-definition name is required by the current canonical
/// `TypeDef::new` API. The temporary definition is never registered or
/// returned, and its name is not used as an identity for the aggregate.
///
/// This deliberate delegation ensures ZCIR does not reimplement rules for
/// nested inline types, duplicate field/variant names, symbolic expressions,
/// or empty unions.
fn validate_canonical_type(kind: &TypeKind) -> Result<(), AggregateTypeError> {
    let validation_name = QualifiedName::parse("zcir::AggregateValidationProbe")
        .map_err(|error| {
            AggregateTypeError::InvalidValidationName(error.to_string())
        })?;

    TypeDef::new(validation_name, kind.clone())
        .map(|_| ())
        .map_err(AggregateTypeError::InvalidCanonicalType)
}

/// Errors raised when adapting or validating aggregate types.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AggregateTypeError {
    /// The supplied type is not a tuple, array, record, sum, or union.
    NotAggregate,

    /// The internal validation name could not be constructed.
    ///
    /// This indicates an unexpected failure in the canonical qualified-name
    /// implementation, not an error in the user's aggregate declaration.
    InvalidValidationName(String),

    /// The canonical ZUIR validator rejected the aggregate type.
    InvalidCanonicalType(TypeDefError),
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
        }
    }
}

impl std::error::Error for AggregateTypeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidCanonicalType(error) => Some(error),
            Self::NotAggregate | Self::InvalidValidationName(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::zuir::schema::types::QualifiedName;
    use crate::zuir::types::{
        Dimension, FieldDef, SymbolicExpression, TypeAttribute, TypeDefError,
        TypeKind, TypeRef, VariantDef,
    };

    use super::{
        AggregateKind, AggregateTypeDescriptor, AggregateTypeError,
    };

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test qualified name should be valid")
    }

    fn named_type(value: &str) -> TypeRef {
        TypeRef::Named(name(value))
    }

    fn empty_attributes() -> std::collections::BTreeMap<String, TypeAttribute> {
        std::collections::BTreeMap::new()
    }

    #[test]
    fn accepts_tuple_and_preserves_element_order() {
        let kind = TypeKind::Tuple {
            elements: vec![
                named_type("core::Integer"),
                named_type("core::Boolean"),
                named_type("core::String"),
            ],
        };

        let descriptor = AggregateTypeDescriptor::new(kind.clone())
            .expect("valid tuple should be accepted");

        assert_eq!(descriptor.aggregate_kind(), AggregateKind::Tuple);
        assert_eq!(descriptor.tuple_elements().map(<[_]>::len), Some(3));
        assert_eq!(descriptor.to_type_kind(), kind);
    }

    #[test]
    fn accepts_empty_tuple_when_canonical_semantics_allow_it() {
        let kind = TypeKind::Tuple {
            elements: Vec::new(),
        };

        let descriptor = AggregateTypeDescriptor::new(kind)
            .expect("canonical validator permits an empty tuple");

        assert_eq!(descriptor.tuple_elements(), Some([].as_slice()));
    }

    #[test]
    fn preserves_known_array_dimension() {
        let kind = TypeKind::Array {
            element: Box::new(named_type("core::Boolean")).as_ref().clone(),
            length: Dimension::Known(u128::MAX),
        };

        let descriptor = AggregateTypeDescriptor::new(kind.clone())
            .expect("known array dimension should be preserved");

        assert_eq!(
            descriptor.array_dimension(),
            Some(&Dimension::Known(u128::MAX))
        );
        assert_eq!(descriptor.to_type_kind(), kind);
    }

    #[test]
    fn preserves_symbolic_array_dimension_without_evaluation() {
        let dimension = Dimension::Symbolic(SymbolicExpression {
            expression: "batch_size * sequence_length".to_owned(),
        });

        let kind = TypeKind::Array {
            element: named_type("core::Float"),
            length: dimension.clone(),
        };

        let descriptor = AggregateTypeDescriptor::new(kind)
            .expect("non-empty symbolic dimension should be accepted");

        assert_eq!(descriptor.array_dimension(), Some(&dimension));
    }

    #[test]
    fn preserves_dynamic_array_dimension() {
        let kind = TypeKind::Array {
            element: named_type("core::Integer"),
            length: Dimension::Dynamic,
        };

        let descriptor = AggregateTypeDescriptor::new(kind)
            .expect("dynamic dimensions should be accepted");

        assert_eq!(
            descriptor.array_dimension(),
            Some(&Dimension::Dynamic)
        );
    }

    #[test]
    fn rejects_empty_symbolic_array_dimension_using_canonical_validation() {
        let kind = TypeKind::Array {
            element: named_type("core::Integer"),
            length: Dimension::Symbolic(SymbolicExpression {
                expression: "   ".to_owned(),
            }),
        };

        assert!(matches!(
            AggregateTypeDescriptor::new(kind),
            Err(AggregateTypeError::InvalidCanonicalType(
                TypeDefError::EmptySymbolicExpression
            ))
        ));
    }

    #[test]
    fn accepts_record_and_preserves_field_metadata() {
        let mut attributes = empty_attributes();
        attributes.insert(
            "documentation".to_owned(),
            TypeAttribute::String("Primary value".to_owned()),
        );

        let fields = vec![
            FieldDef {
                name: "value".to_owned(),
                ty: named_type("core::Integer"),
                attributes,
            },
            FieldDef {
                name: "valid".to_owned(),
                ty: named_type("core::Boolean"),
                attributes: empty_attributes(),
            },
        ];

        let kind = TypeKind::Record {
            fields: fields.clone(),
        };

        let descriptor = AggregateTypeDescriptor::new(kind)
            .expect("valid record should be accepted");

        assert_eq!(descriptor.record_fields(), Some(fields.as_slice()));
    }

    #[test]
    fn rejects_duplicate_record_field_names() {
        let fields = vec![
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

        assert!(matches!(
            AggregateTypeDescriptor::new(TypeKind::Record { fields }),
            Err(AggregateTypeError::InvalidCanonicalType(
                TypeDefError::DuplicateName {
                    category: "record field",
                    ..
                }
            ))
        ));
    }

    #[test]
    fn accepts_sum_type_and_preserves_variants() {
        let variants = vec![
            VariantDef {
                name: "Success".to_owned(),
                fields: vec![FieldDef {
                    name: "value".to_owned(),
                    ty: named_type("core::Integer"),
                    attributes: empty_attributes(),
                }],
                attributes: empty_attributes(),
            },
            VariantDef {
                name: "Failure".to_owned(),
                fields: Vec::new(),
                attributes: empty_attributes(),
            },
        ];

        let descriptor = AggregateTypeDescriptor::new(TypeKind::Sum {
            variants: variants.clone(),
        })
        .expect("valid sum type should be accepted");

        assert_eq!(descriptor.aggregate_kind(), AggregateKind::Sum);
        assert_eq!(descriptor.sum_variants(), Some(variants.as_slice()));
    }

    #[test]
    fn rejects_duplicate_sum_variant_names() {
        let variants = vec![
            VariantDef {
                name: "Result".to_owned(),
                fields: Vec::new(),
                attributes: empty_attributes(),
            },
            VariantDef {
                name: "Result".to_owned(),
                fields: Vec::new(),
                attributes: empty_attributes(),
            },
        ];

        assert!(matches!(
            AggregateTypeDescriptor::new(TypeKind::Sum { variants }),
            Err(AggregateTypeError::InvalidCanonicalType(
                TypeDefError::DuplicateName {
                    category: "sum variant",
                    ..
                }
            ))
        ));
    }

    #[test]
    fn accepts_union_and_preserves_alternative_order() {
        let alternatives = vec![
            named_type("core::Integer"),
            named_type("core::String"),
        ];

        let descriptor = AggregateTypeDescriptor::new(TypeKind::Union {
            alternatives: alternatives.clone(),
        })
        .expect("non-empty union should be accepted");

        assert_eq!(
            descriptor.union_alternatives(),
            Some(alternatives.as_slice())
        );
    }

    #[test]
    fn rejects_empty_union_using_canonical_validation() {
        assert!(matches!(
            AggregateTypeDescriptor::new(TypeKind::Union {
                alternatives: Vec::new(),
            }),
            Err(AggregateTypeError::InvalidCanonicalType(
                TypeDefError::EmptyUnion
            ))
        ));
    }

    #[test]
    fn validates_nested_inline_aggregate_types() {
        let nested = TypeKind::Record {
            fields: vec![FieldDef {
                name: "items".to_owned(),
                ty: TypeRef::Inline(Box::new(TypeKind::Array {
                    element: named_type("core::Integer"),
                    length: Dimension::Symbolic(SymbolicExpression {
                        expression: "item_count".to_owned(),
                    }),
                })),
                attributes: empty_attributes(),
            }],
        };

        let descriptor = AggregateTypeDescriptor::new(nested)
            .expect("valid nested inline aggregates should be accepted");

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn rejects_invalid_nested_inline_aggregate_types() {
        let nested = TypeKind::Record {
            fields: vec![FieldDef {
                name: "items".to_owned(),
                ty: TypeRef::Inline(Box::new(TypeKind::Union {
                    alternatives: Vec::new(),
                })),
                attributes: empty_attributes(),
            }],
        };

        assert!(matches!(
            AggregateTypeDescriptor::new(nested),
            Err(AggregateTypeError::InvalidCanonicalType(
                TypeDefError::EmptyUnion
            ))
        ));
    }

    #[test]
    fn rejects_non_aggregate_type_without_coercion() {
        assert_eq!(
            AggregateTypeDescriptor::new(TypeKind::Boolean),
            Err(AggregateTypeError::NotAggregate)
        );
    }

    #[test]
    fn classification_does_not_consume_the_canonical_type() {
        let kind = TypeKind::Tuple {
            elements: vec![named_type("core::Integer")],
        };

        assert_eq!(
            AggregateTypeDescriptor::classify(&kind),
            Some(AggregateKind::Tuple)
        );

        assert!(matches!(kind, TypeKind::Tuple { .. }));
    }

    #[test]
    fn canonical_round_trip_preserves_the_entire_aggregate() {
        let original = TypeKind::Record {
            fields: vec![
                FieldDef {
                    name: "left".to_owned(),
                    ty: named_type("core::Integer"),
                    attributes: empty_attributes(),
                },
                FieldDef {
                    name: "right".to_owned(),
                    ty: named_type("core::Float"),
                    attributes: empty_attributes(),
                },
            ],
        };

        let descriptor = AggregateTypeDescriptor::try_from(&original)
            .expect("valid canonical aggregate should adapt");

        assert_eq!(descriptor.to_type_kind(), original);
        assert_eq!(descriptor.into_type_kind(), original);
    }

    #[test]
    fn repeated_validation_is_deterministic() {
        let descriptor = AggregateTypeDescriptor::new(TypeKind::Tuple {
            elements: vec![
                named_type("core::Integer"),
                named_type("core::Boolean"),
            ],
        })
        .expect("valid tuple should construct");

        assert_eq!(descriptor.validate(), descriptor.validate());
    }
}
