
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR reference type adapters
//!
//! Path: `src/classical/zcir/types/reference.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Adapts both canonical ZUIR reference concepts:
//!
//! 1. `TypeRef`, which represents either a named type reference or an inline
//!    structural type reference.
//! 2. `TypeKind::Reference`, which represents a semantic reference type
//!    targeting a qualified semantic type name.
//!
//! These concepts are deliberately separate. A `TypeRef::Named` is not
//! automatically a runtime reference, pointer, borrow, or address.
//!
//! ## Authority and ownership
//!
//! Canonical ownership:
//!
//! - `TypeRef` owns named and inline type-reference representation.
//! - `TypeKind::Reference` owns the semantic reference-type representation.
//! - `QualifiedName` owns qualified-name representation and validation.
//! - `TypeDef::validate()` owns local canonical type validation.
//! - The canonical type registry owns named-type resolution and identity.
//! - The semantic model owns the meaning of references, mutability, ownership,
//!   lifetimes, aliasing, and permitted operations.
//! - Layout and lowering own target-specific realization.
//!
//! This module owns:
//!
//! - `TypeReferenceDescriptor`, a validated adapter for `TypeRef`.
//! - `ReferenceTypeDescriptor`, a validated adapter for `TypeKind::Reference`.
//! - Explicit conversions and accessors for those canonical forms.
//! - Structured errors for invalid reference descriptors.
//!
//! This module does not own:
//!
//! - a new canonical type-reference or pointer representation;
//! - name resolution, symbol tables, type inference, or generic solving;
//! - borrow checking, lifetime inference, alias analysis, or ownership rules;
//! - raw pointers, physical addresses, register values, or memory allocation;
//! - source parsing, AST nodes, or grammar-generated parser types;
//! - ABI, data layout, target capabilities, or backend lowering;
//! - type identity allocation, interning, or serialization formats.
//!
//! ## Scalability and portability
//!
//! No arbitrary maximum number of type references or nested type members is
//! imposed here. Canonical collections and symbolic types are preserved.
//!
//! A named reference remains symbolic until the canonical type registry
//! resolves it. An inline reference retains its complete canonical TypeKind.
//! This module never substitutes a missing definition with a fallback type.
//!
//! Validation delegates recursive structure checks to the canonical
//! `TypeDef::validate()` implementation. That validator uses an explicit work
//! stack for structural traversal, avoiding a second recursive validator
//! with different depth or correctness behavior.
//!
//! Rust allocation and address-space limits remain real implementation
//! constraints. Resource policies for untrusted input belong to the enclosing
//! decoding and compilation boundaries, not to type semantics.
//!
//! ## Validation boundary
//!
//! The descriptors check locally enforceable invariants. They do not resolve
//! names, prove that named definitions exist, prove reference-type safety,
//! establish lifetimes, or guarantee that a target supports a reference.
//!
//! A temporary internal TypeDef name is used to invoke the canonical
//! validator. It is never registered, returned, serialized, or used as the
//! semantic identity of the referenced type.
//!
//! ## Integration
//!
//! - `src/classical/zcir/types/mod.rs` declares `pub mod reference;`.
//! - The types module may re-export both descriptors and their error type.
//! - Frontend semantic analysis supplies resolved or unresolved canonical
//!   values according to the existing semantic pipeline contract.
//! - The canonical type registry resolves named references.
//! - Type verification checks type relationships and reference semantics.
//! - Alias and ownership analysis check aliasing, lifetimes, and access rules.
//! - Layout and lowering determine the supported target realization.
//!
//! This file does not depend on sibling ZCIR adapter modules.
//!
//! ## Freeze criteria
//!
//! Freeze only after this file compiles against the actual canonical API,
//! module registration is complete, tests pass, and downstream consumers
//! agree on the distinction between TypeRef and TypeKind::Reference.

use core::fmt;

use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::{TypeDef, TypeDefError, TypeKind, TypeRef};

/// A validated adapter for a canonical `TypeRef`.
///
/// This represents how a type is referenced inside another canonical type.
/// It does not imply runtime indirection, a pointer, a borrow, or an address.
///
/// Named references remain unresolved. Inline references retain their
/// canonical structural type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeReferenceDescriptor {
    reference: TypeRef,
}

impl TypeReferenceDescriptor {
    /// Constructs and validates a descriptor around a canonical type
    /// reference.
    ///
    /// Named references are validated syntactically but are not resolved.
    /// Inline types are validated by the canonical ZUIR type validator.
    ///
    /// # Errors
    ///
    /// Returns an error if the reference or an inline nested type violates
    /// canonical locally enforceable type invariants.
    pub fn new(reference: TypeRef) -> Result<Self, ReferenceTypeError> {
        validate_type_reference(&reference)?;

        Ok(Self { reference })
    }

    /// Returns the canonical reference without resolving or changing it.
    #[must_use]
    pub fn as_type_ref(&self) -> &TypeRef {
        &self.reference
    }

    /// Returns whether this is a named reference.
    #[must_use]
    pub fn is_named(&self) -> bool {
        matches!(self.reference, TypeRef::Named(_))
    }

    /// Returns whether this is an inline structural type reference.
    #[must_use]
    pub fn is_inline(&self) -> bool {
        matches!(self.reference, TypeRef::Inline(_))
    }

    /// Returns the qualified name when this is a named reference.
    ///
    /// This does not prove that the name exists in the canonical type
    /// registry.
    #[must_use]
    pub fn named_target(&self) -> Option<&QualifiedName> {
        match &self.reference {
            TypeRef::Named(name) => Some(name),
            TypeRef::Inline(_) => None,
        }
    }

    /// Returns the inline type when this is an inline reference.
    #[must_use]
    pub fn inline_type(&self) -> Option<&TypeKind> {
        match &self.reference {
            TypeRef::Named(_) => None,
            TypeRef::Inline(kind) => Some(kind.as_ref()),
        }
    }

    /// Revalidates the reference at a trust boundary.
    pub fn validate(&self) -> Result<(), ReferenceTypeError> {
        validate_type_reference(&self.reference)
    }

    /// Consumes the adapter and returns the canonical `TypeRef`.
    ///
    /// This conversion is lossless.
    #[must_use]
    pub fn into_type_ref(self) -> TypeRef {
        self.reference
    }

    /// Clones the canonical `TypeRef`.
    #[must_use]
    pub fn to_type_ref(&self) -> TypeRef {
        self.reference.clone()
    }
}

impl TryFrom<TypeRef> for TypeReferenceDescriptor {
    type Error = ReferenceTypeError;

    fn try_from(reference: TypeRef) -> Result<Self, Self::Error> {
        Self::new(reference)
    }
}

impl TryFrom<&TypeRef> for TypeReferenceDescriptor {
    type Error = ReferenceTypeError;

    fn try_from(reference: &TypeRef) -> Result<Self, Self::Error> {
        Self::new(reference.clone())
    }
}

impl From<TypeReferenceDescriptor> for TypeRef {
    fn from(descriptor: TypeReferenceDescriptor) -> Self {
        descriptor.into_type_ref()
    }
}

/// A validated adapter for canonical `TypeKind::Reference`.
///
/// Unlike `TypeReferenceDescriptor`, this represents a semantic reference
/// type whose canonical representation names a target type.
///
/// The target name is not resolved here. Whether this type represents a
/// safe reference, mutable reference, handle, or other semantic construct
/// is determined by the language's canonical type and semantic contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceTypeDescriptor {
    target: QualifiedName,
}

impl ReferenceTypeDescriptor {
    /// Constructs a semantic reference-type descriptor.
    ///
    /// # Errors
    ///
    /// Returns an error if the target name fails canonical validation.
    pub fn new(target: QualifiedName) -> Result<Self, ReferenceTypeError> {
        let descriptor = Self { target };
        descriptor.validate()?;
        Ok(descriptor)
    }

    /// Returns the qualified target type name.
    ///
    /// The name is not resolved and is not a type identity.
    #[must_use]
    pub fn target(&self) -> &QualifiedName {
        &self.target
    }

    /// Revalidates this descriptor using canonical ZUIR validation.
    pub fn validate(&self) -> Result<(), ReferenceTypeError> {
        validate_semantic_reference_type(&self.to_type_kind())
    }

    /// Converts this descriptor into the canonical semantic reference type.
    #[must_use]
    pub fn into_type_kind(self) -> TypeKind {
        TypeKind::Reference {
            target: self.target,
        }
    }

    /// Returns an owned canonical semantic reference type.
    #[must_use]
    pub fn to_type_kind(&self) -> TypeKind {
        TypeKind::Reference {
            target: self.target.clone(),
        }
    }
}

impl TryFrom<TypeKind> for ReferenceTypeDescriptor {
    type Error = ReferenceTypeError;

    fn try_from(kind: TypeKind) -> Result<Self, Self::Error> {
        Self::try_from_kind(&kind)
    }
}

impl TryFrom<&TypeKind> for ReferenceTypeDescriptor {
    type Error = ReferenceTypeError;

    fn try_from(kind: &TypeKind) -> Result<Self, Self::Error> {
        Self::try_from_kind(kind)
    }
}

impl ReferenceTypeDescriptor {
    /// Adapts a canonical semantic reference type.
    ///
    /// Returns `NotSemanticReference` for other TypeKind variants.
    pub fn try_from_kind(kind: &TypeKind) -> Result<Self, ReferenceTypeError> {
        match kind {
            TypeKind::Reference { target } => Self::new(target.clone()),
            _ => Err(ReferenceTypeError::NotSemanticReference),
        }
    }
}

impl From<ReferenceTypeDescriptor> for TypeKind {
    fn from(descriptor: ReferenceTypeDescriptor) -> Self {
        descriptor.into_type_kind()
    }
}

/// Validates a canonical `TypeRef` through the canonical type validator.
///
/// A temporary `Newtype` wrapper makes the canonical validator traverse an
/// inline type using the same rules it applies to all other nested types.
/// The wrapper is internal only and does not become part of the returned
/// descriptor or its identity.
pub fn validate_type_reference(
    reference: &TypeRef,
) -> Result<(), ReferenceTypeError> {
    let wrapped = TypeKind::Newtype {
        underlying: reference.clone(),
    };

    validate_canonical_type(&wrapped)
}

/// Validates a canonical semantic `TypeKind::Reference`.
///
/// Other TypeKind variants are rejected rather than silently accepted.
pub fn validate_semantic_reference_type(
    kind: &TypeKind,
) -> Result<(), ReferenceTypeError> {
    if !matches!(kind, TypeKind::Reference { .. }) {
        return Err(ReferenceTypeError::NotSemanticReference);
    }

    validate_canonical_type(kind)
}

fn validate_canonical_type(kind: &TypeKind) -> Result<(), ReferenceTypeError> {
    let validation_name =
        QualifiedName::parse("zcir::ReferenceValidationProbe").map_err(
            |error| ReferenceTypeError::InvalidValidationName(error.to_string()),
        )?;

    TypeDef::new(validation_name, kind.clone())
        .map(|_| ())
        .map_err(ReferenceTypeError::InvalidCanonicalType)
}

/// Errors returned by reference-type adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceTypeError {
    /// The supplied TypeKind is not the canonical semantic Reference variant.
    NotSemanticReference,

    /// The internal validation name could not be constructed.
    InvalidValidationName(String),

    /// The canonical ZUIR validator rejected the type or nested structure.
    InvalidCanonicalType(TypeDefError),
}

impl fmt::Display for ReferenceTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSemanticReference => {
                f.write_str("canonical type is not a semantic reference type")
            }
            Self::InvalidValidationName(message) => {
                write!(f, "invalid internal reference validation name: {message}")
            }
            Self::InvalidCanonicalType(error) => {
                write!(f, "invalid canonical reference type: {error}")
            }
        }
    }
}

impl std::error::Error for ReferenceTypeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidCanonicalType(error) => Some(error),
            Self::NotSemanticReference | Self::InvalidValidationName(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("valid test qualified name")
    }

    fn named(value: &str) -> TypeRef {
        TypeRef::Named(name(value))
    }

    #[test]
    fn preserves_named_type_reference() {
        let reference = named("core::Integer");
        let descriptor = TypeReferenceDescriptor::new(reference.clone())
            .expect("valid named type reference");

        assert!(descriptor.is_named());
        assert!(!descriptor.is_inline());
        assert_eq!(descriptor.named_target(), Some(&name("core::Integer")));
        assert_eq!(descriptor.as_type_ref(), &reference);
        assert_eq!(descriptor.to_type_ref(), reference);
    }

    #[test]
    fn preserves_inline_type_reference() {
        let inline = TypeRef::Inline(Box::new(TypeKind::Boolean));
        let descriptor = TypeReferenceDescriptor::new(inline.clone())
            .expect("valid inline type reference");

        assert!(!descriptor.is_named());
        assert!(descriptor.is_inline());
        assert_eq!(descriptor.inline_type(), Some(&TypeKind::Boolean));
        assert_eq!(descriptor.to_type_ref(), inline);
    }

    #[test]
    fn accepts_named_reference_without_resolving_it() {
        // The type registry owns name resolution. Local validation must not
        // require a declaration to exist in this module.
        let descriptor =
            TypeReferenceDescriptor::new(named("future_module::FutureType"))
                .expect("valid qualified name should remain representable");

        assert_eq!(
            descriptor.named_target(),
            Some(&name("future_module::FutureType"))
        );
    }

    #[test]
    fn rejects_invalid_nested_inline_type() {
        let reference = TypeRef::Inline(Box::new(TypeKind::Union {
            alternatives: Vec::new(),
        }));

        let result = TypeReferenceDescriptor::new(reference);

        assert!(matches!(
            result,
            Err(ReferenceTypeError::InvalidCanonicalType(
                TypeDefError::EmptyUnion
            ))
        ));
    }

    #[test]
    fn rejects_non_reference_semantic_types() {
        assert!(matches!(
            ReferenceTypeDescriptor::try_from_kind(&TypeKind::Boolean),
            Err(ReferenceTypeError::NotSemanticReference)
        ));
    }

    #[test]
    fn preserves_semantic_reference_target() {
        let canonical = TypeKind::Reference {
            target: name("memory::Region"),
        };

        let descriptor = ReferenceTypeDescriptor::try_from_kind(&canonical)
            .expect("canonical semantic reference type");

        assert_eq!(descriptor.target(), &name("memory::Region"));
        assert_eq!(descriptor.to_type_kind(), canonical);
        assert_eq!(descriptor.into_type_kind(), canonical);
    }

    #[test]
    fn semantic_reference_target_is_validated() {
        let descriptor = ReferenceTypeDescriptor::new(name("memory::Region"))
            .expect("valid target name");

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn conversion_traits_preserve_named_reference() {
        let original = named("module::Value");
        let descriptor = TypeReferenceDescriptor::try_from(&original)
            .expect("reference conversion");

        let reconstructed: TypeRef = descriptor.into();

        assert_eq!(reconstructed, original);
    }

    #[test]
    fn conversion_traits_preserve_semantic_reference() {
        let original = TypeKind::Reference {
            target: name("module::Value"),
        };

        let descriptor = ReferenceTypeDescriptor::try_from(&original)
            .expect("semantic reference conversion");

        let reconstructed: TypeKind = descriptor.into();

        assert_eq!(reconstructed, original);
    }

    #[test]
    fn validation_is_repeatable() {
        let descriptor = TypeReferenceDescriptor::new(TypeRef::Inline(
            Box::new(TypeKind::Tuple {
                elements: vec![
                    named("core::Boolean"),
                    TypeRef::Inline(Box::new(TypeKind::Integer {
                        signedness: crate::zuir::types::IntegerSignedness::Signless,
                        width: crate::zuir::types::IntegerWidth::Unbounded,
                    })),
                ],
            }),
        ))
        .expect("valid nested inline type");

        assert_eq!(descriptor.validate(), descriptor.validate());
    }
}
