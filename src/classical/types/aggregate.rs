
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Zamani classical aggregate type descriptors.
//!
//! Path: src/classical/types/aggregate.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only.
//!
//! # Purpose
//!
//! Defines target-independent descriptors for aggregate types. Supported
//! forms include structures, records, tuples, enumerations, tagged unions,
//! untagged unions, optional values, and result values.
//!
//! # Authority and ownership
//!
//! This module owns:
//! - AggregateKind and AggregateTypeDescriptor.
//! - AggregateField and AggregateVariant.
//! - AggregateMemberType, the aggregate-member type reference.
//! - Local aggregate invariants and validation errors.
//!
//! This module does not own:
//! - Source syntax, source AST nodes, parsing, or name resolution.
//! - The canonical semantic type system or canonical IR.
//! - Generic parameter substitution or generic constraint solving.
//! - Array, vector, matrix, tensor, map, or set storage.
//! - Physical memory layout, allocation, ABI representation, or execution.
//! - Numeric operations or scalar value representations.
//!
//! # Architecture
//!
//! Source syntax is parsed into the canonical frontend AST. Semantic
//! analysis resolves names and generic arguments and checks these
//! descriptors against the canonical semantic type system. Lowering then
//! maps validated types to the existing canonical IR.
//!
//! These descriptors are not a second AST or executable IR.
//!
//! # POCO-REAF and scalability
//!
//! This module imposes no universal maximum on field count, tuple arity,
//! variant count, nesting depth, or hardware capacity.
//!
//! It preserves declaration order and uses checked collection operations
//! where arithmetic is required. It does not allocate storage for aggregate
//! values or promise that every finite machine can materialize every type.
//!
//! A finite Rust process remains subject to available memory, host
//! representation limits, explicit resource policies, and target
//! capabilities. Those constraints must not be confused with language
//! semantics.
//!
//! # Integration contract
//!
//! - `types/mod.rs` registers and exports this module.
//! - `types/scalar.rs` owns scalar type descriptors.
//! - `types/generic.rs` owns generic parameters and substitution.
//! - `types/constraints.rs` owns type constraints and their resolution.
//! - `types/layout.rs` owns layout decisions, if introduced.
//! - `collections/` and canonical classical IR modules own collection
//!   semantics and storage.
//! - `src/frontend/ast/node/types/type_expr.rs` remains the source-level
//!   type-expression owner.
//! - Semantic analysis resolves `Named` references and checks type identity.
//! - Lowering maps validated semantic types to existing canonical IR types.
//!
//! A `Named` member is a reference, not proof that a declaration exists.
//! Name resolution and cycle analysis must be performed by semantic analysis.
//!
//! # Determinism
//!
//! Field and variant order is preserved exactly as supplied. No unordered
//! map is used for declaration contents. Equality and hashing are structural.
//!
//! # Freeze rule
//!
//! Changes to aggregate-kind invariants or public API require a compatibility
//! review against the normative type specification and canonical semantic
//! type owner.

use core::fmt;
use crate::classical::types::scalar::ScalarTypeDescriptor;

/// The semantic form of an aggregate type.
///
/// These are language-level type categories, not physical layout strategies.
/// A backend may realize them differently when doing so preserves semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AggregateKind {
    /// A product type whose fields are identified by names.
    Struct,

    /// A named-field record type.
    Record,

    /// A positional product type.
    Tuple,

    /// A finite set of named, payload-free alternatives.
    Enum,

    /// A tagged choice between named alternatives with optional payloads.
    TaggedUnion,

    /// A union whose alternatives share a representation contract.
    ///
    /// Validity and safe access rules must be defined by semantic analysis.
    /// This descriptor does not establish that arbitrary union reads are safe.
    UntaggedUnion,

    /// An optional value represented semantically as absent or present.
    Optional,

    /// A successful value or an error value.
    Result,

    /// A dialect-defined aggregate category.
    Extension(String),
}

/// A type that can appear as an aggregate member.
///
/// Named references allow recursive declarations without recursively
/// embedding an infinitely sized Rust value. Their resolution is delegated
/// to the canonical semantic analyzer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AggregateMemberType {
    /// A scalar type descriptor owned by `types::scalar`.
    Scalar(ScalarTypeDescriptor),

    /// A directly nested aggregate descriptor.
    Aggregate(Box<AggregateTypeDescriptor>),

    /// A reference to a declared type, possibly qualified by module.
    ///
    /// Examples include `Customer`, `network::Packet`, and `tree::Node`.
    /// The string is opaque to this module except for basic emptiness and
    /// whitespace checks. It is not parsed or resolved here.
    Named(String),
}

impl AggregateMemberType {
    /// Constructs a named type reference.
    ///
    /// This checks only that the reference is non-empty and has no
    /// surrounding or embedded whitespace. Identifier grammar and name
    /// resolution belong to the canonical frontend and semantic analyzer.
    pub fn named(name: impl Into<String>) -> Result<Self, AggregateTypeError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(AggregateTypeError::EmptyTypeReference);
        }

        if name.chars().any(char::is_whitespace) {
            return Err(AggregateTypeError::WhitespaceInTypeReference { name });
        }

        Ok(Self::Named(name))
    }

    /// Constructs a directly nested aggregate type.
    pub fn aggregate(
        aggregate: AggregateTypeDescriptor,
    ) -> Self {
        Self::Aggregate(Box::new(aggregate))
    }
}

/// A declared aggregate field or positional tuple element.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AggregateField {
    name: Option<String>,
    ty: AggregateMemberType,
}

impl AggregateField {
    /// Creates a named field.
    ///
    /// Empty names are rejected. Full identifier validation remains the
    /// responsibility of the canonical name-resolution layer.
    pub fn named(
        name: impl Into<String>,
        ty: AggregateMemberType,
    ) -> Result<Self, AggregateTypeError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(AggregateTypeError::EmptyFieldName);
        }

        if name.chars().any(char::is_whitespace) {
            return Err(AggregateTypeError::WhitespaceInFieldName { name });
        }

        Ok(Self {
            name: Some(name),
            ty,
        })
    }

    /// Creates an unnamed positional element.
    pub fn positional(ty: AggregateMemberType) -> Self {
        Self { name: None, ty }
    }

    /// Returns the field name, if this is a named field.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the member type.
    #[must_use]
    pub const fn ty(&self) -> &AggregateMemberType {
        &self.ty
    }
}

/// One named alternative in an enumeration or union.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AggregateVariant {
    name: String,
    fields: Vec<AggregateField>,
}

impl AggregateVariant {
    /// Creates an alternative without a payload.
    pub fn unit(name: impl Into<String>) -> Result<Self, AggregateTypeError> {
        Self::new(name, Vec::new())
    }

    /// Creates an alternative with zero or more payload fields.
    ///
    /// Payload fields must be consistently named or positional.
    /// Duplicate names are rejected.
    pub fn new(
        name: impl Into<String>,
        fields: Vec<AggregateField>,
    ) -> Result<Self, AggregateTypeError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(AggregateTypeError::EmptyVariantName);
        }

        if name.chars().any(char::is_whitespace) {
            return Err(AggregateTypeError::WhitespaceInVariantName { name });
        }

        validate_field_names(&fields)?;

        Ok(Self { name, fields })
    }

    /// Returns the alternative name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the payload fields in declaration order.
    #[must_use]
    pub fn fields(&self) -> &[AggregateField] {
        &self.fields
    }
}

/// A validated aggregate type description.
///
/// Construction is restricted to the provided constructors so local
/// invariants cannot be bypassed by mutating public fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AggregateTypeDescriptor {
    kind: AggregateKind,
    fields: Vec<AggregateField>,
    variants: Vec<AggregateVariant>,
}

impl AggregateTypeDescriptor {
    /// Constructs a structure.
    ///
    /// Every field must have a unique name. Declaration order is preserved.
    pub fn structure(
        fields: Vec<AggregateField>,
    ) -> Result<Self, AggregateTypeError> {
        Self::new(AggregateKind::Struct, fields, Vec::new())
    }

    /// Constructs a record.
    ///
    /// Every field must have a unique name. Declaration order is preserved.
    pub fn record(
        fields: Vec<AggregateField>,
    ) -> Result<Self, AggregateTypeError> {
        Self::new(AggregateKind::Record, fields, Vec::new())
    }

    /// Constructs a positional tuple.
    ///
    /// All elements must be positional. An empty tuple is valid.
    pub fn tuple(
        fields: Vec<AggregateField>,
    ) -> Result<Self, AggregateTypeError> {
        Self::new(AggregateKind::Tuple, fields, Vec::new())
    }

    /// Constructs a payload-free enumeration.
    ///
    /// Variant names must be unique. An empty enumeration is rejected
    /// because it has no inhabitants; uninhabited-type semantics, if needed,
    /// belong to the canonical type system's `Never` representation.
    pub fn enumeration(
        variants: Vec<AggregateVariant>,
    ) -> Result<Self, AggregateTypeError> {
        Self::new(AggregateKind::Enum, Vec::new(), variants)
    }

    /// Constructs a tagged union with named alternatives.
    pub fn tagged_union(
        variants: Vec<AggregateVariant>,
    ) -> Result<Self, AggregateTypeError> {
        Self::new(AggregateKind::TaggedUnion, Vec::new(), variants)
    }

    /// Constructs an untagged union with named alternatives.
    ///
    /// This records the semantic alternatives only. It does not select a
    /// C ABI layout or permit unchecked reads of inactive alternatives.
    pub fn untagged_union(
        variants: Vec<AggregateVariant>,
    ) -> Result<Self, AggregateTypeError> {
        Self::new(AggregateKind::UntaggedUnion, Vec::new(), variants)
    }

    /// Constructs an optional type containing one named `value` field.
    ///
    /// Absence/presence semantics are represented by the category itself;
    /// this is not a promise of a particular niche or memory representation.
    pub fn optional(
        value: AggregateMemberType,
    ) -> Result<Self, AggregateTypeError> {
        Self::new(
            AggregateKind::Optional,
            vec![AggregateField::named("value", value)?],
            Vec::new(),
        )
    }

    /// Constructs a result type with named `ok` and `error` fields.
    ///
    /// The two fields represent the success and error payload types,
    /// respectively. Their runtime encoding is determined downstream.
    pub fn result(
        ok: AggregateMemberType,
        error: AggregateMemberType,
    ) -> Result<Self, AggregateTypeError> {
        Self::new(
            AggregateKind::Result,
            vec![
                AggregateField::named("ok", ok)?,
                AggregateField::named("error", error)?,
            ],
            Vec::new(),
        )
    }

    /// Constructs a dialect-defined aggregate descriptor.
    ///
    /// The extension must provide its own normative invariants and semantic
    /// validation. This constructor enforces only the common structural
    /// invariants.
    pub fn extension(
        name: impl Into<String>,
        fields: Vec<AggregateField>,
        variants: Vec<AggregateVariant>,
    ) -> Result<Self, AggregateTypeError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(AggregateTypeError::EmptyExtensionName);
        }

        Self::new(AggregateKind::Extension(name), fields, variants)
    }

    /// Constructs and validates an aggregate descriptor.
    ///
    /// This is the shared validation boundary for all aggregate categories.
    pub fn new(
        kind: AggregateKind,
        fields: Vec<AggregateField>,
        variants: Vec<AggregateVariant>,
    ) -> Result<Self, AggregateTypeError> {
        match &kind {
            AggregateKind::Struct | AggregateKind::Record => {
                require_no_variants(&kind, &variants)?;
                require_named_fields(&fields)?;
                validate_field_names(&fields)?;
            }
            AggregateKind::Tuple => {
                require_no_variants(&kind, &variants)?;
                require_positional_fields(&fields)?;
            }
            AggregateKind::Enum => {
                require_no_fields(&kind, &fields)?;
                require_variants(&variants)?;
                validate_variants(&variants)?;
                for variant in &variants {
                    if !variant.fields.is_empty() {
                        return Err(AggregateTypeError::PayloadNotAllowed {
                            kind: kind.clone(),
                            variant: variant.name.clone(),
                        });
                    }
                }
            }
            AggregateKind::TaggedUnion | AggregateKind::UntaggedUnion => {
                require_no_fields(&kind, &fields)?;
                require_variants(&variants)?;
                validate_variants(&variants)?;
            }
            AggregateKind::Optional => {
                require_no_variants(&kind, &variants)?;
                if fields.len() != 1 || fields[0].name.as_deref() != Some("value") {
                    return Err(AggregateTypeError::InvalidOptionalFields);
                }
                validate_field_names(&fields)?;
            }
            AggregateKind::Result => {
                require_no_variants(&kind, &variants)?;
                if fields.len() != 2
                    || fields[0].name.as_deref() != Some("ok")
                    || fields[1].name.as_deref() != Some("error")
                {
                    return Err(AggregateTypeError::InvalidResultFields);
                }
                validate_field_names(&fields)?;
            }
            AggregateKind::Extension(name) => {
                if name.trim().is_empty() {
                    return Err(AggregateTypeError::EmptyExtensionName);
                }
                validate_field_names(&fields)?;
                validate_variants(&variants)?;
            }
        }

        Ok(Self {
            kind,
            fields,
            variants,
        })
    }

    /// Returns the aggregate category.
    #[must_use]
    pub const fn kind(&self) -> &AggregateKind {
        &self.kind
    }

    /// Returns aggregate-level fields in declaration order.
    #[must_use]
    pub fn fields(&self) -> &[AggregateField] {
        &self.fields
    }

    /// Returns alternatives in declaration order.
    #[must_use]
    pub fn variants(&self) -> &[AggregateVariant] {
        &self.variants
    }

    /// Returns whether this descriptor is a product type.
    #[must_use]
    pub const fn is_product(&self) -> bool {
        matches!(
            self.kind,
            AggregateKind::Struct | AggregateKind::Record | AggregateKind::Tuple
        )
    }

    /// Returns whether this descriptor represents a choice between variants.
    #[must_use]
    pub const fn is_sum(&self) -> bool {
        matches!(
            self.kind,
            AggregateKind::Enum
                | AggregateKind::TaggedUnion
                | AggregateKind::UntaggedUnion
                | AggregateKind::Optional
                | AggregateKind::Result
        )
    }

    /// Returns a named field by exact name.
    ///
    /// Lookup is linear in the number of fields. The descriptor makes no
    /// assumption about a maximum field count or a particular map layout.
    #[must_use]
    pub fn field(&self, name: &str) -> Option<&AggregateField> {
        self.fields
            .iter()
            .find(|field| field.name.as_deref() == Some(name))
    }

    /// Returns a variant by exact name.
    #[must_use]
    pub fn variant(&self, name: &str) -> Option<&AggregateVariant> {
        self.variants.iter().find(|variant| variant.name == name)
    }

    /// Revalidates the local descriptor invariants.
    ///
    /// This is not a substitute for name resolution, recursive-type analysis,
    /// generic checking, or canonical semantic type checking.
    pub fn validate(&self) -> Result<(), AggregateTypeError> {
        Self::new(
            self.kind.clone(),
            self.fields.clone(),
            self.variants.clone(),
        )
        .map(|_| ())
    }
}

fn require_no_fields(
    kind: &AggregateKind,
    fields: &[AggregateField],
) -> Result<(), AggregateTypeError> {
    if fields.is_empty() {
        Ok(())
    } else {
        Err(AggregateTypeError::UnexpectedFields {
            kind: kind.clone(),
        })
    }
}

fn require_no_variants(
    kind: &AggregateKind,
    variants: &[AggregateVariant],
) -> Result<(), AggregateTypeError> {
    if variants.is_empty() {
        Ok(())
    } else {
        Err(AggregateTypeError::UnexpectedVariants {
            kind: kind.clone(),
        })
    }
}

fn require_named_fields(fields: &[AggregateField]) -> Result<(), AggregateTypeError> {
    if fields.iter().all(|field| field.name.is_some()) {
        Ok(())
    } else {
        Err(AggregateTypeError::NamedFieldsRequired)
    }
}

fn require_positional_fields(fields: &[AggregateField]) -> Result<(), AggregateTypeError> {
    if fields.iter().all(|field| field.name.is_none()) {
        Ok(())
    } else {
        Err(AggregateTypeError::PositionalFieldsRequired)
    }
}

fn require_variants(variants: &[AggregateVariant]) -> Result<(), AggregateTypeError> {
    if variants.is_empty() {
        Err(AggregateTypeError::EmptyVariants)
    } else {
        Ok(())
    }
}

fn validate_field_names(fields: &[AggregateField]) -> Result<(), AggregateTypeError> {
    for (index, field) in fields.iter().enumerate() {
        let Some(name) = field.name.as_deref() else {
            continue;
        };

        if fields[..index]
            .iter()
            .any(|previous| previous.name.as_deref() == Some(name))
        {
            return Err(AggregateTypeError::DuplicateField {
                name: name.to_owned(),
            });
        }
    }

    Ok(())
}

fn validate_variants(variants: &[AggregateVariant]) -> Result<(), AggregateTypeError> {
    for (index, variant) in variants.iter().enumerate() {
        if variants[..index]
            .iter()
            .any(|previous| previous.name == variant.name)
        {
            return Err(AggregateTypeError::DuplicateVariant {
                name: variant.name.clone(),
            });
        }

        validate_field_names(&variant.fields)?;
    }

    Ok(())
}

/// Aggregate descriptor validation failures.
///
/// Errors are deterministic and contain no hardware-specific assumptions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregateTypeError {
    /// A named type reference was empty.
    EmptyTypeReference,

    /// A named type reference contains whitespace.
    WhitespaceInTypeReference { name: String },

    /// A field name was empty.
    EmptyFieldName,

    /// A field name contains whitespace.
    WhitespaceInFieldName { name: String },

    /// A variant name was empty.
    EmptyVariantName,

    /// A variant name contains whitespace.
    WhitespaceInVariantName { name: String },

    /// An extension category name was empty.
    EmptyExtensionName,

    /// A field name was declared more than once.
    DuplicateField { name: String },

    /// A variant name was declared more than once.
    DuplicateVariant { name: String },

    /// A named-field aggregate contains an unnamed field.
    NamedFieldsRequired,

    /// A tuple contains a named field.
    PositionalFieldsRequired,

    /// An enum or union has no alternatives.
    EmptyVariants,

    /// A payload is not allowed for this category.
    PayloadNotAllowed {
        kind: AggregateKind,
        variant: String,
    },

    /// Fields were supplied to a category that does not accept them.
    UnexpectedFields { kind: AggregateKind },

    /// Variants were supplied to a category that does not accept them.
    UnexpectedVariants { kind: AggregateKind },

    /// Optional must contain exactly one field named `value`.
    InvalidOptionalFields,

    /// Result must contain `ok` followed by `error`.
    InvalidResultFields,
}

impl fmt::Display for AggregateTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTypeReference => {
                f.write_str("aggregate member type reference must not be empty")
            }
            Self::WhitespaceInTypeReference { name } => {
                write!(f, "type reference contains whitespace: {name:?}")
            }
            Self::EmptyFieldName => f.write_str("aggregate field name must not be empty"),
            Self::WhitespaceInFieldName { name } => {
                write!(f, "aggregate field name contains whitespace: {name:?}")
            }
            Self::EmptyVariantName => {
                f.write_str("aggregate variant name must not be empty")
            }
            Self::WhitespaceInVariantName { name } => {
                write!(f, "aggregate variant name contains whitespace: {name:?}")
            }
            Self::EmptyExtensionName => {
                f.write_str("aggregate extension name must not be empty")
            }
            Self::DuplicateField { name } => {
                write!(f, "duplicate aggregate field name: {name}")
            }
            Self::DuplicateVariant { name } => {
                write!(f, "duplicate aggregate variant name: {name}")
            }
            Self::NamedFieldsRequired => {
                f.write_str("this aggregate requires named fields")
            }
            Self::PositionalFieldsRequired => {
                f.write_str("tuple elements must be positional")
            }
            Self::EmptyVariants => {
                f.write_str("this aggregate requires at least one variant")
            }
            Self::PayloadNotAllowed { kind, variant } => {
                write!(f, "variant {variant} cannot carry a payload in {kind:?}")
            }
            Self::UnexpectedFields { kind } => {
                write!(f, "{kind:?} does not accept aggregate-level fields")
            }
            Self::UnexpectedVariants { kind } => {
                write!(f, "{kind:?} does not accept variants")
            }
            Self::InvalidOptionalFields => {
                f.write_str("optional type requires exactly one field named `value`")
            }
            Self::InvalidResultFields => {
                f.write_str("result type requires `ok` followed by `error`")
            }
        }
    }
}

impl std::error::Error for AggregateTypeError {}

#[cfg(test)]
mod tests {
    use super::{
        AggregateField, AggregateKind, AggregateMemberType, AggregateTypeDescriptor,
        AggregateTypeError, AggregateVariant,
    };
    use crate::classical::types::scalar::{
        ScalarKind, ScalarTypeDescriptor,
    };

    fn scalar() -> AggregateMemberType {
        AggregateMemberType::Scalar(
            ScalarTypeDescriptor::unspecified(ScalarKind::SignedInteger)
                .expect("scalar descriptor should be valid"),
        )
    }

    fn named_field(name: &str) -> AggregateField {
        AggregateField::named(name, scalar()).expect("field should be valid")
    }

    #[test]
    fn structure_preserves_field_order() {
        let ty = AggregateTypeDescriptor::structure(vec![
            named_field("first"),
            named_field("second"),
        ])
        .expect("structure should be valid");

        assert_eq!(ty.fields()[0].name(), Some("first"));
        assert_eq!(ty.fields()[1].name(), Some("second"));
        assert!(ty.is_product());
    }

    #[test]
    fn duplicate_structure_fields_are_rejected() {
        assert_eq!(
            AggregateTypeDescriptor::structure(vec![
                named_field("value"),
                named_field("value"),
            ]),
            Err(AggregateTypeError::DuplicateField {
                name: "value".to_owned(),
            })
        );
    }

    #[test]
    fn tuple_requires_positional_elements() {
        assert_eq!(
            AggregateTypeDescriptor::tuple(vec![named_field("value")]),
            Err(AggregateTypeError::PositionalFieldsRequired)
        );
    }

    #[test]
    fn empty_tuple_is_valid() {
        let ty = AggregateTypeDescriptor::tuple(Vec::new())
            .expect("unit tuple should be valid");

        assert!(ty.is_product());
        assert!(ty.fields().is_empty());
    }

    #[test]
    fn enum_rejects_payloads() {
        let payload = AggregateField::positional(scalar());
        let variant = AggregateVariant::new("Ready", vec![payload])
            .expect("variant payload itself is valid");

        assert_eq!(
            AggregateTypeDescriptor::enumeration(vec![variant]),
            Err(AggregateTypeError::PayloadNotAllowed {
                kind: AggregateKind::Enum,
                variant: "Ready".to_owned(),
            })
        );
    }

    #[test]
    fn tagged_union_supports_payloads() {
        let variant = AggregateVariant::new(
            "Value",
            vec![AggregateField::positional(scalar())],
        )
        .expect("variant should be valid");

        let ty = AggregateTypeDescriptor::tagged_union(vec![
            AggregateVariant::unit("Empty").expect("unit variant"),
            variant,
        ])
        .expect("tagged union should be valid");

        assert!(ty.is_sum());
        assert!(ty.variant("Value").is_some());
    }

    #[test]
    fn duplicate_variants_are_rejected() {
        assert_eq!(
            AggregateTypeDescriptor::enumeration(vec![
                AggregateVariant::unit("Ready").expect("variant"),
                AggregateVariant::unit("Ready").expect("variant"),
            ]),
            Err(AggregateTypeError::DuplicateVariant {
                name: "Ready".to_owned(),
            })
        );
    }

    #[test]
    fn optional_uses_one_value_member() {
        let ty = AggregateTypeDescriptor::optional(scalar())
            .expect("optional should be valid");

        assert_eq!(ty.kind(), &AggregateKind::Optional);
        assert!(ty.field("value").is_some());
    }

    #[test]
    fn result_has_success_and_error_members() {
        let ty = AggregateTypeDescriptor::result(scalar(), scalar())
            .expect("result should be valid");

        assert!(ty.field("ok").is_some());
        assert!(ty.field("error").is_some());
    }

    #[test]
    fn named_reference_is_not_resolved_here() {
        let reference = AggregateMemberType::named("tree::Node")
            .expect("qualified reference should be accepted");

        assert_eq!(
            reference,
            AggregateMemberType::Named("tree::Node".to_owned())
        );
    }

    #[test]
    fn empty_named_reference_is_rejected() {
        assert_eq!(
            AggregateMemberType::named("  "),
            Err(AggregateTypeError::EmptyTypeReference)
        );
    }

    #[test]
    fn direct_nested_aggregate_is_supported() {
        let nested = AggregateTypeDescriptor::tuple(vec![
            AggregateField::positional(scalar()),
        ])
        .expect("nested tuple should be valid");

        let outer = AggregateTypeDescriptor::structure(vec![
            AggregateField::named(
                "coordinates",
                AggregateMemberType::aggregate(nested),
            )
            .expect("field should be valid"),
        ])
        .expect("outer structure should be valid");

        assert!(outer.field("coordinates").is_some());
    }

    #[test]
    fn extension_categories_are_open_ended() {
        let ty = AggregateTypeDescriptor::extension(
            "dialect.example.aggregate",
            vec![named_field("payload")],
            Vec::new(),
        )
        .expect("extension descriptor should be accepted");

        assert_eq!(
            ty.kind(),
            &AggregateKind::Extension(
                "dialect.example.aggregate".to_owned()
            )
        );
    }

    #[test]
    fn validation_is_repeatable() {
        let ty = AggregateTypeDescriptor::record(vec![
            named_field("id"),
            named_field("payload"),
        ])
        .expect("record should be valid");

        assert_eq!(ty.validate(), Ok(()));
        assert_eq!(ty.validate(), Ok(()));
    }

    #[test]
    fn structurally_equal_descriptors_compare_equal() {
        let left = AggregateTypeDescriptor::structure(vec![
            named_field("id"),
            named_field("payload"),
        ])
        .expect("left descriptor");

        let right = AggregateTypeDescriptor::structure(vec![
            named_field("id"),
            named_field("payload"),
        ])
        .expect("right descriptor");

        assert_eq!(left, right);
    }
}
