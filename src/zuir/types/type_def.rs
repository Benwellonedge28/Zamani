
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # ZUIR semantic type definitions
//!
//! Path: `src/zuir/types/type_def.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! PURPOSE
//!
//! Defines target-independent semantic type definitions for ZUIR.
//!
//! OWNS
//!
//! - The semantic type-definition vocabulary.
//! - Aggregate, function, generic, nominal, opaque, and domain types.
//! - Symbolic integer widths and symbolic aggregate dimensions.
//! - Local type-definition validation.
//! - Deterministic structural traversal and validation errors.
//!
//! DOES_NOT_OWN
//!
//! - Canonical type-ID allocation or interning.
//! - Type inference, unification, or generic specialization.
//! - Physical layout, ABI, alignment, register allocation, or placement.
//! - Name resolution or symbol-table ownership.
//! - Dialect registration or operation verification.
//! - Quantum physical mapping, HDL synthesis, or backend code generation.
//!
//! INTEGRATION
//!
//! - `types/mod.rs` declares and exports this module.
//! - `types/type_id.rs` should own canonical type identity when introduced.
//! - `types/layout.rs` derives target-specific layout from validated types.
//! - `types/constraints.rs` owns general constraint solving.
//! - `types/compatibility.rs` owns cross-type compatibility queries.
//! - `dialect/registry.rs` owns extension registration and resolution.
//! - `verify/types.rs` invokes local and cross-reference validation.
//!
//! This file deliberately does not import planned sibling modules that are
//! not present in the inspected repository. This avoids circular dependencies
//! and allows the type vocabulary to be reviewed independently.
//!
//! SCALABILITY
//!
//! There are no universal limits on integer width, tensor rank, aggregate
//! size, quantum resources, signal width, or the number of declared types.
//! Symbolic quantities remain symbolic until a responsible later stage
//! resolves them. All actual compilation remains bounded by available
//! resources and the implementation's representational limits.
//!
//! HARD-CODING
//!
//! Semantic primitives are stable language concepts. Domain-specific
//! operations, vendor instructions, and future computational types must not
//! be added to a closed hardware-specific catalogue. Use `Dialect` for
//! types whose semantics are defined by an extension.
//!
//! SAFETY
//!
//! Safe Rust only. Deserialized values must be validated before being trusted.
//! Local validation does not establish that referenced types exist or that
//! a complete program is semantically valid.

use crate::zuir::schema::types::QualifiedName;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// A semantic type definition.
///
/// `name` is the qualified semantic name, not a physical representation
/// name. Canonical identity, interning, and cross-module uniqueness belong
/// to the type registry and type-ID subsystem.
///
/// The fields are public to support inspection and schema interoperability.
/// Call `validate()` after construction and after deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeDef {
    /// Fully qualified semantic name.
    pub name: QualifiedName,

    /// Definition body.
    pub kind: TypeKind,

    /// Optional documentation or tooling annotations.
    ///
    /// These annotations do not change type semantics unless an owning
    /// specification explicitly defines otherwise.
    #[serde(default)]
    pub attributes: BTreeMap<String, TypeAttribute>,

    /// Whether this definition is intended to be externally visible.
    ///
    /// This is declaration metadata; linkage and access checking belong
    /// to the module and symbol systems.
    #[serde(default)]
    pub visibility: TypeVisibility,
}

/// Visibility of a type declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TypeVisibility {
    /// Usable only within its declaring scope.
    #[default]
    Private,

    /// Exportable subject to module and policy validation.
    Public,
}

/// A typed, serializable descriptive attribute.
///
/// These values are deliberately independent of the full schema attribute
/// system. Integration may later replace this with the canonical attribute
/// value type once that module's public contract is established.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum TypeAttribute {
    String(String),
    Boolean(bool),
    Integer(i128),
    Name(QualifiedName),
}

/// The semantic body of a type definition.
///
/// The enum is non-exhaustive so consumers must not assume this list
/// represents every domain that Zamani may eventually support.
///
/// New domain-specific types should normally use `Dialect` instead of
/// adding vendor-specific or target-specific variants to the universal core.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[non_exhaustive]
pub enum TypeKind {
    /// A value with no inhabitants.
    Never,

    /// A unit value.
    Unit,

    /// Boolean semantics independent of physical representation.
    Boolean,

    /// Mathematical integer semantics.
    Integer {
        signedness: IntegerSignedness,
        width: IntegerWidth,
    },

    /// Floating-point semantics described by an extensible format name.
    ///
    /// A format name is a semantic format identifier, not a promise that
    /// every target natively supports that format.
    Float {
        format: QualifiedName,
    },

    /// Exact or inexact decimal semantics identified by a format.
    Decimal {
        format: QualifiedName,
    },

    /// Unicode text.
    String,

    /// Opaque bytes without an implied encoding.
    Bytes,

    /// A tuple whose elements may have distinct types.
    Tuple {
        elements: Vec<TypeRef>,
    },

    /// A sequence with an element type and symbolic or known length.
    Array {
        element: TypeRef,
        length: Dimension,
    },

    /// A record with named, typed fields.
    Record {
        fields: Vec<FieldDef>,
    },

    /// A sum type with named variants.
    Sum {
        variants: Vec<VariantDef>,
    },

    /// A union with explicitly declared alternatives.
    ///
    /// Whether the representation is tagged, untagged, or target-specific
    /// is not decided here.
    Union {
        alternatives: Vec<TypeRef>,
    },

    /// A reference to another named semantic type.
    Reference {
        target: QualifiedName,
    },

    /// A nominal type that wraps another type but retains distinct identity.
    Newtype {
        underlying: TypeRef,
    },

    /// An opaque type whose representation is intentionally hidden.
    Opaque {
        constraints: Vec<TypeConstraint>,
    },

    /// A generic parameter.
    Parameter {
        name: String,
        bounds: Vec<TypeConstraint>,
    },

    /// A generic type application.
    Applied {
        constructor: QualifiedName,
        arguments: Vec<TypeArgument>,
    },

    /// A function signature.
    Function {
        parameters: Vec<FunctionParameter>,
        results: Vec<TypeRef>,
        effects: Vec<QualifiedName>,
        calling_convention: Option<QualifiedName>,
    },

    /// A quantum-semantic type, independent of physical qubit allocation.
    Quantum {
        domain_type: QualifiedName,
        parameters: Vec<TypeArgument>,
    },

    /// A hardware-description semantic type, independent of a particular
    /// device's available width or resource count.
    Hardware {
        domain_type: QualifiedName,
        parameters: Vec<TypeArgument>,
    },

    /// A type defined by a registered extension or computational dialect.
    Dialect {
        dialect: QualifiedName,
        type_name: QualifiedName,
        arguments: Vec<TypeArgument>,
    },
}

/// A reference to a semantic type.
///
/// Named references are resolved by the canonical type registry. They are
/// intentionally not resolved inside this file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum TypeRef {
    Named(QualifiedName),
    /// An inline type is useful for anonymous structural types.
    Inline(Box<TypeKind>),
}

/// Integer signedness is a semantic property, not a register-width choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegerSignedness {
    Signed,
    Unsigned,
    Signless,
}

/// Integer width is fixed or symbolic.
///
/// `Unbounded` denotes mathematical integer semantics. It does not guarantee
/// that a concrete backend can execute every operation without a lowering
/// strategy or resource cost.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum IntegerWidth {
    Unbounded,
    Fixed(u128),
    Symbolic(SymbolicExpression),
}

/// A symbolic expression reference.
///
/// This is a reference to an expression owned by the semantic/constraint
/// system; it is not an expression evaluator or a source-language parser.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolicExpression {
    pub expression: String,
}

/// A symbolic or concrete dimension.
///
/// `Dynamic` means the value is not statically known. It does not mean
/// unlimited runtime memory or a guaranteed allocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum Dimension {
    Known(u128),
    Symbolic(SymbolicExpression),
    Dynamic,
}

/// A named field in a record type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    pub ty: TypeRef,
    #[serde(default)]
    pub attributes: BTreeMap<String, TypeAttribute>,
}

/// A variant in a sum type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariantDef {
    pub name: String,
    /// Empty payloads represent unit-like variants.
    #[serde(default)]
    pub fields: Vec<FieldDef>,
    #[serde(default)]
    pub attributes: BTreeMap<String, TypeAttribute>,
}

/// A type argument supplied to a generic or dialect-defined type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum TypeArgument {
    Type(TypeRef),
    Constant(ConstantArgument),
    Symbolic(SymbolicExpression),
}

/// A constant argument that can be represented without host-width truncation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum ConstantArgument {
    Boolean(bool),
    Integer(String),
    String(String),
    Name(QualifiedName),
}

/// Function parameter semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: Option<String>,
    pub ty: TypeRef,
    #[serde(default)]
    pub mode: ParameterMode,
}

/// Parameter passing and ownership intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ParameterMode {
    #[default]
    Value,
    Borrow,
    BorrowMut,
    Move,
}

/// Declarative constraints attached to a type parameter or opaque type.
///
/// Constraint interpretation and solving are delegated to the constraint
/// system. An unknown constraint must not be silently treated as satisfied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum TypeConstraint {
    Implements(QualifiedName),
    EqualTo(TypeRef),
    IntegerWidthAtLeast(SymbolicExpression),
    Shape(SymbolicExpression),
    Dialect {
        namespace: QualifiedName,
        constraint: QualifiedName,
    },
}

impl TypeDef {
    /// Construct a type definition and validate locally enforceable rules.
    pub fn new(name: QualifiedName, kind: TypeKind) -> Result<Self, TypeDefError> {
        let definition = Self {
            name,
            kind,
            attributes: BTreeMap::new(),
            visibility: TypeVisibility::Private,
        };
        definition.validate()?;
        Ok(definition)
    }

    /// Validate the definition's locally enforceable invariants.
    ///
    /// This function does not resolve references, instantiate generics,
    /// evaluate symbolic expressions, or prove dialect constraints.
    pub fn validate(&self) -> Result<(), TypeDefError> {
        self.name
            .validate()
            .map_err(|e| TypeDefError::InvalidName(e.to_string()))?;

        let mut pending = vec![WorkItem::Kind(&self.kind)];

        for (key, value) in &self.attributes {
            validate_identifier(key)?;
            validate_attribute(value)?;
        }

        while let Some(item) = pending.pop() {
            match item {
                WorkItem::Kind(kind) => match kind {
                    TypeKind::Never
                    | TypeKind::Unit
                    | TypeKind::Boolean
                    | TypeKind::String
                    | TypeKind::Bytes => {}

                    TypeKind::Integer { width, .. } => match width {
                        IntegerWidth::Unbounded => {}
                        IntegerWidth::Fixed(0) => {
                            return Err(TypeDefError::ZeroIntegerWidth);
                        }
                        IntegerWidth::Fixed(_) => {}
                        IntegerWidth::Symbolic(expr) => {
                            validate_expression(expr)?;
                        }
                    },

                    TypeKind::Float { format } | TypeKind::Decimal { format } => {
                        format.validate().map_err(|e| {
                            TypeDefError::InvalidName(e.to_string())
                        })?;
                    }

                    TypeKind::Tuple { elements } => {
                        for ty in elements {
                            pending.push(WorkItem::TypeRef(ty));
                        }
                    }

                    TypeKind::Array { element, length } => {
                        pending.push(WorkItem::TypeRef(element));
                        if let Dimension::Symbolic(expr) = length {
                            validate_expression(expr)?;
                        }
                    }

                    TypeKind::Record { fields } => {
                        validate_unique_names(
                            fields.iter().map(|f| f.name.as_str()),
                            "record field",
                        )?;
                        for field in fields {
                            validate_identifier(&field.name)?;
                            validate_attributes(&field.attributes)?;
                            pending.push(WorkItem::TypeRef(&field.ty));
                        }
                    }

                    TypeKind::Sum { variants } => {
                        validate_unique_names(
                            variants.iter().map(|v| v.name.as_str()),
                            "sum variant",
                        )?;
                        for variant in variants {
                            validate_identifier(&variant.name)?;
                            validate_attributes(&variant.attributes)?;
                            for field in &variant.fields {
                                validate_identifier(&field.name)?;
                                validate_attributes(&field.attributes)?;
                                pending.push(WorkItem::TypeRef(&field.ty));
                            }
                            validate_unique_names(
                                variant.fields.iter().map(|f| f.name.as_str()),
                                "variant field",
                            )?;
                        }
                    }

                    TypeKind::Union { alternatives } => {
                        if alternatives.is_empty() {
                            return Err(TypeDefError::EmptyUnion);
                        }
                        for ty in alternatives {
                            pending.push(WorkItem::TypeRef(ty));
                        }
                    }

                    TypeKind::Reference { target } => {
                        target.validate().map_err(|e| {
                            TypeDefError::InvalidName(e.to_string())
                        })?;
                    }

                    TypeKind::Newtype { underlying } => {
                        pending.push(WorkItem::TypeRef(underlying));
                    }

                    TypeKind::Opaque { constraints } => {
                        for constraint in constraints {
                            pending.push(WorkItem::Constraint(constraint));
                        }
                    }

                    TypeKind::Parameter { name, bounds } => {
                        validate_identifier(name)?;
                        for constraint in bounds {
                            pending.push(WorkItem::Constraint(constraint));
                        }
                    }

                    TypeKind::Applied {
                        constructor,
                        arguments,
                    } => {
                        constructor.validate().map_err(|e| {
                            TypeDefError::InvalidName(e.to_string())
                        })?;
                        for argument in arguments {
                            pending.push(WorkItem::Argument(argument));
                        }
                    }

                    TypeKind::Function {
                        parameters,
                        results,
                        effects,
                        calling_convention,
                    } => {
                        for parameter in parameters {
                            if let Some(name) = &parameter.name {
                                validate_identifier(name)?;
                            }
                            pending.push(WorkItem::TypeRef(&parameter.ty));
                        }
                        for result in results {
                            pending.push(WorkItem::TypeRef(result));
                        }
                        for effect in effects {
                            effect.validate().map_err(|e| {
                                TypeDefError::InvalidName(e.to_string())
                            })?;
                        }
                        if let Some(convention) = calling_convention {
                            convention.validate().map_err(|e| {
                                TypeDefError::InvalidName(e.to_string())
                            })?;
                        }
                    }

                    TypeKind::Quantum {
                        domain_type,
                        parameters,
                    }
                    | TypeKind::Hardware {
                        domain_type,
                        parameters,
                    } => {
                        domain_type.validate().map_err(|e| {
                            TypeDefError::InvalidName(e.to_string())
                        })?;
                        for argument in parameters {
                            pending.push(WorkItem::Argument(argument));
                        }
                    }

                    TypeKind::Dialect {
                        dialect,
                        type_name,
                        arguments,
                    } => {
                        dialect.validate().map_err(|e| {
                            TypeDefError::InvalidName(e.to_string())
                        })?;
                        type_name.validate().map_err(|e| {
                            TypeDefError::InvalidName(e.to_string())
                        })?;
                        for argument in arguments {
                            pending.push(WorkItem::Argument(argument));
                        }
                    }
                },

                WorkItem::TypeRef(ty) => match ty {
                    TypeRef::Named(name) => {
                        name.validate().map_err(|e| {
                            TypeDefError::InvalidName(e.to_string())
                        })?;
                    }
                    TypeRef::Inline(kind) => pending.push(WorkItem::Kind(kind)),
                },

                WorkItem::Argument(argument) => match argument {
                    TypeArgument::Type(ty) => {
                        pending.push(WorkItem::TypeRef(ty));
                    }
                    TypeArgument::Constant(value) => {
                        if let ConstantArgument::Name(name) = value {
                            name.validate().map_err(|e| {
                                TypeDefError::InvalidName(e.to_string())
                            })?;
                        }
                    }
                    TypeArgument::Symbolic(expr) => {
                        validate_expression(expr)?;
                    }
                },

                WorkItem::Constraint(constraint) => match constraint {
                    TypeConstraint::Implements(name) => {
                        name.validate().map_err(|e| {
                            TypeDefError::InvalidName(e.to_string())
                        })?;
                    }
                    TypeConstraint::EqualTo(ty) => {
                        pending.push(WorkItem::TypeRef(ty));
                    }
                    TypeConstraint::IntegerWidthAtLeast(expr)
                    | TypeConstraint::Shape(expr) => {
                        validate_expression(expr)?;
                    }
                    TypeConstraint::Dialect {
                        namespace,
                        constraint,
                    } => {
                        namespace.validate().map_err(|e| {
                            TypeDefError::InvalidName(e.to_string())
                        })?;
                        constraint.validate().map_err(|e| {
                            TypeDefError::InvalidName(e.to_string())
                        })?;
                    }
                },
            }
        }

        Ok(())
    }
}

enum WorkItem<'a> {
    Kind(&'a TypeKind),
    TypeRef(&'a TypeRef),
    Argument(&'a TypeArgument),
    Constraint(&'a TypeConstraint),
}

fn validate_expression(expr: &SymbolicExpression) -> Result<(), TypeDefError> {
    if expr.expression.trim().is_empty() {
        return Err(TypeDefError::EmptySymbolicExpression);
    }
    Ok(())
}

fn validate_attributes(
    attributes: &BTreeMap<String, TypeAttribute>,
) -> Result<(), TypeDefError> {
    for (key, value) in attributes {
        validate_identifier(key)?;
        validate_attribute(value)?;
    }
    Ok(())
}

fn validate_attribute(value: &TypeAttribute) -> Result<(), TypeDefError> {
    if let TypeAttribute::Name(name) = value {
        name.validate()
            .map_err(|e| TypeDefError::InvalidName(e.to_string()))?;
    }
    Ok(())
}

fn validate_unique_names<'a, I>(
    names: I,
    category: &'static str,
) -> Result<(), TypeDefError>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut seen = std::collections::BTreeSet::new();
    for name in names {
        if !seen.insert(name) {
            return Err(TypeDefError::DuplicateName {
                category,
                name: name.to_owned(),
            });
        }
    }
    Ok(())
}

/// A conservative identifier check for names owned by this module.
///
/// Source-language identifier rules remain owned by the language/frontend
/// specification. This helper checks the representation contract only.
fn validate_identifier(value: &str) -> Result<(), TypeDefError> {
    let mut chars = value.chars();

    let Some(first) = chars.next() else {
        return Err(TypeDefError::EmptyIdentifier);
    };

    if !(first == '_' || first.is_ascii_alphabetic())
        || !chars.all(|c| c == '_' || c.is_ascii_alphanumeric())
    {
        return Err(TypeDefError::InvalidIdentifier(value.to_owned()));
    }

    Ok(())
}

/// Errors from local type-definition construction and validation.
///
/// The verifier may translate these into the repository's canonical
/// structured diagnostic format.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TypeDefError {
    InvalidName(String),
    EmptyIdentifier,
    InvalidIdentifier(String),
    ZeroIntegerWidth,
    EmptyUnion,
    EmptySymbolicExpression,
    DuplicateName {
        category: &'static str,
        name: String,
    },
}

impl fmt::Display for TypeDefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidName(message) => write!(f, "invalid qualified type name: {message}"),
            Self::EmptyIdentifier => f.write_str("identifier must not be empty"),
            Self::InvalidIdentifier(name) => {
                write!(f, "invalid type-definition identifier: {name:?}")
            }
            Self::ZeroIntegerWidth => {
                f.write_str("fixed integer width must be greater than zero")
            }
            Self::EmptyUnion => {
                f.write_str("union type must contain at least one alternative")
            }
            Self::EmptySymbolicExpression => {
                f.write_str("symbolic expression reference must not be empty")
            }
            Self::DuplicateName { category, name } => {
                write!(f, "duplicate {category} name: {name:?}")
            }
        }
    }
}

impl std::error::Error for TypeDefError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("valid qualified name")
    }

    #[test]
    fn accepts_unbounded_integer_semantics() {
        let ty = TypeDef::new(
            name("core::Integer"),
            TypeKind::Integer {
                signedness: IntegerSignedness::Signless,
                width: IntegerWidth::Unbounded,
            },
        );
        assert!(ty.is_ok());
    }

    #[test]
    fn rejects_zero_width() {
        let ty = TypeDef::new(
            name("core::InvalidInteger"),
            TypeKind::Integer {
                signedness: IntegerSignedness::Unsigned,
                width: IntegerWidth::Fixed(0),
            },
        );
        assert_eq!(ty.unwrap_err(), TypeDefError::ZeroIntegerWidth);
    }

    #[test]
    fn accepts_symbolic_array_dimensions() {
        let ty = TypeDef::new(
            name("tensor::Vector"),
            TypeKind::Array {
                element: TypeRef::Named(name("core::Number")),
                length: Dimension::Symbolic(SymbolicExpression {
                    expression: "vector_length".to_owned(),
                }),
            },
        );
        assert!(ty.is_ok());
    }

    #[test]
    fn rejects_empty_symbolic_dimensions() {
        let ty = TypeDef::new(
            name("tensor::InvalidVector"),
            TypeKind::Array {
                element: TypeRef::Named(name("core::Number")),
                length: Dimension::Symbolic(SymbolicExpression {
                    expression: "  ".to_owned(),
                }),
            },
        );
        assert_eq!(ty.unwrap_err(), TypeDefError::EmptySymbolicExpression);
    }

    #[test]
    fn rejects_duplicate_record_fields() {
        let ty = TypeDef::new(
            name("core::DuplicateRecord"),
            TypeKind::Record {
                fields: vec![
                    FieldDef {
                        name: "value".to_owned(),
                        ty: TypeRef::Named(name("core::Integer")),
                        attributes: BTreeMap::new(),
                    },
                    FieldDef {
                        name: "value".to_owned(),
                        ty: TypeRef::Named(name("core::Boolean")),
                        attributes: BTreeMap::new(),
                    },
                ],
            },
        );
        assert!(matches!(
            ty,
            Err(TypeDefError::DuplicateName {
                category: "record field",
                ..
            })
        ));
    }

    #[test]
    fn accepts_dialect_defined_types_without_core_changes() {
        let ty = TypeDef::new(
            name("vendor::SpecialTensor"),
            TypeKind::Dialect {
                dialect: name("vendor::tensor"),
                type_name: name("tensor::Special"),
                arguments: vec![TypeArgument::Symbolic(SymbolicExpression {
                    expression: "rank".to_owned(),
                })],
            },
        );
        assert!(ty.is_ok());
    }

    #[test]
    fn validates_after_deserialization_or_direct_mutation() {
        let mut ty = TypeDef::new(name("core::Value"), TypeKind::Unit)
            .expect("valid initial type");
        ty.kind = TypeKind::Union {
            alternatives: Vec::new(),
        };
        assert_eq!(ty.validate(), Err(TypeDefError::EmptyUnion));
    }
}
