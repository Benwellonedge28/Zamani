
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # ZCIR function type adapter
//!
//! Path: `src/classical/zcir/types/function.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! ## Purpose
//!
//! Provides a validated classical-facing adapter for canonical ZUIR function
//! types. A function type describes:
//!
//! - zero or more parameters and their canonical types;
//! - parameter passing and ownership intent;
//! - zero or more result types;
//! - declared effects;
//! - an optional, extensible calling-convention identifier.
//!
//! The adapter preserves canonical ZUIR representations. It does not define
//! another function-signature language or canonical type identity.
//!
//! ## Authority and ownership
//!
//! Canonical authority:
//!
//! - `TypeKind::Function` owns the function-type representation.
//! - `FunctionParameter` owns parameter names, types, and passing modes.
//! - `ParameterMode` owns the parameter-passing vocabulary.
//! - `TypeRef` owns named and inline type references.
//! - `QualifiedName` owns qualified-name representation and validation.
//! - `TypeDef::validate()` owns canonical local type validation.
//! - Zamani's semantic specifications own the meaning of functions, effects,
//!   parameter modes, result values, and observable behavior.
//!
//! This file owns:
//!
//! - `FunctionTypeDescriptor`, a validated adapter over `TypeKind::Function`;
//! - function-specific local checks not performed by the canonical validator;
//! - explicit, lossless conversions to and from the canonical function type;
//! - `FunctionTypeError`, describing adapter validation failures.
//!
//! This file does not own:
//!
//! - source syntax, parsing, ASTs, name resolution, or type inference;
//! - a competing parameter, result, effect, or type-reference representation;
//! - function declarations, function bodies, blocks, regions, or control flow;
//! - generic inference, specialization, or symbolic constraint solving;
//! - effect inference, effect authorization, or whole-program effect checking;
//! - overload resolution, linkage, symbol tables, or function identity;
//! - ABI selection, physical stack layout, register assignment, or mangling;
//! - target capability discovery, scheduling, code generation, or execution.
//!
//! ## Scalability and POCO-REAF
//!
//! This module imposes no arbitrary maximum number of parameters, results,
//! effects, or named types. It does not narrow canonical collections to
//! fixed-size arrays or use machine-width assumptions as semantic limits.
//!
//! Collection sizes remain subject to available memory, address space, and
//! the operational budgets imposed by the enclosing compiler. Such budgets
//! must be explicit implementation policy, not hidden function semantics.
//!
//! A valid signature does not imply that every target supports its parameter
//! modes, effects, types, or calling convention. Target feasibility and ABI
//! realization belong to later verification and lowering stages.
//!
//! ## Validation
//!
//! This adapter checks that explicitly named parameters are unique and
//! delegates canonical type validation to `TypeDef::validate()`.
//!
//! Named parameter identifiers, nested type references, effects, and calling
//! convention names are checked by the canonical validator. Inline nested
//! types are recursively validated by that same validator.
//!
//! Parameter names are optional in the canonical representation. Unnamed
//! parameters are preserved and do not participate in duplicate-name checks.
//! Parameter order, result order, effect order, and all canonical parameter
//! modes are preserved; this adapter does not sort or normalize them.
//!
//! This is local signature validation only. It does not resolve named types,
//! prove that effects are permitted, check function bodies, infer effects,
//! or establish that a target can realize the signature.
//!
//! ## Integration contract
//!
//! - `src/classical/zcir/types/mod.rs` should declare `pub mod function;`.
//! - The parent module may re-export `FunctionTypeDescriptor` and
//!   `FunctionTypeError` without duplicating canonical types.
//! - Frontend semantic analysis resolves source declarations and supplies
//!   canonical `FunctionParameter`, `TypeRef`, effect, and convention values.
//! - ZCIR verification invokes `validate()` at relevant trust boundaries.
//! - Function declarations and bodies belong to the canonical IR's function
//!   representation, not to this type descriptor.
//! - Effect verification remains a separate semantic/IR verification concern.
//! - ABI selection and target-specific calling sequences belong to lowering.
//! - Serialization consumers must validate untrusted decoded signatures.
//!
//! This file deliberately does not depend on planned ZCIR sibling modules.
//!
//! ## Freeze criteria
//!
//! Freeze this file only after the parent module is registered, the code
//! compiles against the repository's canonical API, tests pass, and the
//! resulting API is reviewed with downstream consumers.

use core::fmt;
use std::collections::BTreeSet;

use crate::zuir::schema::types::QualifiedName;
use crate::zuir::types::{
    FunctionParameter, TypeDef, TypeDefError, TypeKind, TypeRef,
};

/// Validated adapter for a canonical ZUIR function type.
///
/// This descriptor owns no canonical type identity. It preserves the
/// parameter list, result list, effects, and optional calling convention
/// represented by `TypeKind::Function`.
///
/// Use [`FunctionTypeDescriptor::new`] to construct a descriptor from its
/// constituent canonical values, or [`FunctionTypeDescriptor::try_from_kind`]
/// to adapt an existing canonical type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionTypeDescriptor {
    parameters: Vec<FunctionParameter>,
    results: Vec<TypeRef>,
    effects: Vec<QualifiedName>,
    calling_convention: Option<QualifiedName>,
}

impl FunctionTypeDescriptor {
    /// Constructs and validates a function-type descriptor.
    ///
    /// Parameter and result ordering is preserved. Empty parameter, result,
    /// and effect lists are permitted by this adapter; their interpretation
    /// follows the canonical type and language semantic contracts.
    ///
    /// An absent calling convention remains absent. No platform-specific
    /// default is inferred.
    ///
    /// # Errors
    ///
    /// Returns an error if explicitly named parameters are duplicated or
    /// canonical validation rejects the signature or one of its nested types.
    pub fn new(
        parameters: Vec<FunctionParameter>,
        results: Vec<TypeRef>,
        effects: Vec<QualifiedName>,
        calling_convention: Option<QualifiedName>,
    ) -> Result<Self, FunctionTypeError> {
        let descriptor = Self {
            parameters,
            results,
            effects,
            calling_convention,
        };

        descriptor.validate()?;
        Ok(descriptor)
    }

    /// Adapts a canonical ZUIR type into a function-type descriptor.
    ///
    /// All fields are cloned without reordering, narrowing, name resolution,
    /// or target-specific interpretation.
    ///
    /// # Errors
    ///
    /// Returns [`FunctionTypeError::NotFunction`] when `kind` is not a
    /// function type, or a validation error when the signature is invalid.
    pub fn try_from_kind(kind: &TypeKind) -> Result<Self, FunctionTypeError> {
        match kind {
            TypeKind::Function {
                parameters,
                results,
                effects,
                calling_convention,
            } => Self::new(
                parameters.clone(),
                results.clone(),
                effects.clone(),
                calling_convention.clone(),
            ),
            _ => Err(FunctionTypeError::NotFunction),
        }
    }

    /// Returns the parameters in their declared order.
    #[must_use]
    pub fn parameters(&self) -> &[FunctionParameter] {
        &self.parameters
    }

    /// Returns the result types in their declared order.
    ///
    /// The canonical representation permits multiple results. This adapter
    /// does not collapse them into a tuple or infer an implicit unit result.
    #[must_use]
    pub fn results(&self) -> &[TypeRef] {
        &self.results
    }

    /// Returns the declared effects in their canonical order.
    ///
    /// This is a declaration view, not proof that the effects are authorized
    /// or that a function body obeys them.
    #[must_use]
    pub fn effects(&self) -> &[QualifiedName] {
        &self.effects
    }

    /// Returns the optional calling-convention identifier.
    ///
    /// The identifier is semantic metadata in the canonical signature. Its
    /// target-specific interpretation and realization belong to downstream
    /// linkage and lowering contracts.
    #[must_use]
    pub fn calling_convention(&self) -> Option<&QualifiedName> {
        self.calling_convention.as_ref()
    }

    /// Returns the number of declared parameters.
    ///
    /// This is the length of the in-memory collection, not a language-level
    /// parameter limit. The return type follows Rust's collection API.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Returns the number of declared result types.
    #[must_use]
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Returns whether the signature has no declared parameters.
    ///
    /// This does not imply that the function is pure, has no effects, or
    /// requires no runtime resources.
    #[must_use]
    pub fn has_no_parameters(&self) -> bool {
        self.parameters.is_empty()
    }

    /// Returns whether the signature declares no result types.
    ///
    /// The meaning of a function with no result types is defined by the
    /// language semantics. This method does not synthesize a unit type.
    #[must_use]
    pub fn has_no_results(&self) -> bool {
        self.results.is_empty()
    }

    /// Revalidates the descriptor at a trust boundary.
    ///
    /// This checks local parameter-name uniqueness and delegates recursive
    /// canonical type checks to `TypeDef::validate()`.
    ///
    /// It does not resolve named references, infer effects, validate a
    /// function body, or establish target feasibility.
    pub fn validate(&self) -> Result<(), FunctionTypeError> {
        self.validate_parameter_names()?;

        let kind = self.to_type_kind();
        validate_canonical_function_type(&kind)
    }

    /// Reconstructs the canonical ZUIR function type without loss.
    ///
    /// This does not allocate a type ID, register a type, select an ABI,
    /// or lower the signature to a target.
    #[must_use]
    pub fn into_type_kind(self) -> TypeKind {
        TypeKind::Function {
            parameters: self.parameters,
            results: self.results,
            effects: self.effects,
            calling_convention: self.calling_convention,
        }
    }

    /// Reconstructs an owned canonical type while retaining this descriptor.
    #[must_use]
    pub fn to_type_kind(&self) -> TypeKind {
        TypeKind::Function {
            parameters: self.parameters.clone(),
            results: self.results.clone(),
            effects: self.effects.clone(),
            calling_convention: self.calling_convention.clone(),
        }
    }

    fn validate_parameter_names(&self) -> Result<(), FunctionTypeError> {
        let mut names = BTreeSet::new();

        for parameter in &self.parameters {
            if let Some(name) = parameter.name.as_deref() {
                if !names.insert(name) {
                    return Err(FunctionTypeError::DuplicateParameterName(
                        name.to_owned(),
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Validates a canonical function type using the canonical ZUIR validator.
///
/// This helper exists so other ZCIR code can reuse the same checks without
/// implementing an independent function-signature validator.
///
/// A temporary internal definition name is used only to invoke
/// `TypeDef::validate()`. It is not registered, returned, serialized, or
/// used as the function's identity.
///
/// # Errors
///
/// Returns [`FunctionTypeError::NotFunction`] for a non-function type, or
/// [`FunctionTypeError::InvalidCanonicalType`] when canonical validation
/// rejects the signature.
pub fn validate_function_type(
    kind: &TypeKind,
) -> Result<(), FunctionTypeError> {
    validate_canonical_function_type(kind)
}

fn validate_canonical_function_type(
    kind: &TypeKind,
) -> Result<(), FunctionTypeError> {
    if !matches!(kind, TypeKind::Function { .. }) {
        return Err(FunctionTypeError::NotFunction);
    }

    let validation_name = QualifiedName::parse("zcir::FunctionValidationProbe")
        .map_err(|error| {
            FunctionTypeError::InvalidValidationName(error.to_string())
        })?;

    TypeDef::new(validation_name, kind.clone())
        .map(|_| ())
        .map_err(FunctionTypeError::InvalidCanonicalType)
}

/// Errors raised while adapting or validating a function type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionTypeError {
    /// The supplied canonical type is not a function type.
    NotFunction,

    /// Two or more explicitly named parameters use the same identifier.
    DuplicateParameterName(String),

    /// The internal validation name could not be constructed.
    ///
    /// This is defensive error handling for the fixed internal name used
    /// to invoke the canonical validator. It is not a source-level name.
    InvalidValidationName(String),

    /// The canonical ZUIR type validator rejected the function signature.
    InvalidCanonicalType(TypeDefError),
}

impl fmt::Display for FunctionTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFunction => {
                f.write_str("canonical type is not a function type")
            }
            Self::DuplicateParameterName(name) => {
                write!(f, "duplicate function parameter name: {name:?}")
            }
            Self::InvalidValidationName(message) => {
                write!(f, "could not construct internal validation name: {message}")
            }
            Self::InvalidCanonicalType(error) => {
                write!(f, "invalid canonical function type: {error}")
            }
        }
    }
}

impl std::error::Error for FunctionTypeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidCanonicalType(error) => Some(error),
            Self::NotFunction
            | Self::DuplicateParameterName(_)
            | Self::InvalidValidationName(_) => None,
        }
    }
}

impl TryFrom<&TypeKind> for FunctionTypeDescriptor {
    type Error = FunctionTypeError;

    fn try_from(kind: &TypeKind) -> Result<Self, Self::Error> {
        Self::try_from_kind(kind)
    }
}

impl TryFrom<TypeKind> for FunctionTypeDescriptor {
    type Error = FunctionTypeError;

    fn try_from(kind: TypeKind) -> Result<Self, Self::Error> {
        Self::try_from_kind(&kind)
    }
}

impl From<FunctionTypeDescriptor> for TypeKind {
    fn from(descriptor: FunctionTypeDescriptor) -> Self {
        descriptor.into_type_kind()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zuir::types::ParameterMode;

    fn name(value: &str) -> QualifiedName {
        QualifiedName::parse(value).expect("test name should be valid")
    }

    fn named_type(value: &str) -> TypeRef {
        TypeRef::Named(name(value))
    }

    fn parameter(name: Option<&str>, ty: TypeRef) -> FunctionParameter {
        FunctionParameter {
            name: name.map(str::to_owned),
            ty,
            mode: ParameterMode::Value,
        }
    }

    #[test]
    fn accepts_empty_parameter_and_result_lists() {
        let descriptor = FunctionTypeDescriptor::new(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
        )
        .expect("empty lists are representable");

        assert!(descriptor.has_no_parameters());
        assert!(descriptor.has_no_results());
        assert_eq!(descriptor.parameter_count(), 0);
        assert_eq!(descriptor.result_count(), 0);
    }

    #[test]
    fn preserves_parameter_modes_and_order() {
        let parameters = vec![
            FunctionParameter {
                name: Some("input".to_owned()),
                ty: named_type("core::Input"),
                mode: ParameterMode::Borrow,
            },
            FunctionParameter {
                name: Some("state".to_owned()),
                ty: named_type("core::State"),
                mode: ParameterMode::BorrowMut,
            },
            FunctionParameter {
                name: Some("owned".to_owned()),
                ty: named_type("core::Owned"),
                mode: ParameterMode::Move,
            },
        ];

        let descriptor = FunctionTypeDescriptor::new(
            parameters.clone(),
            vec![named_type("core::Output")],
            Vec::new(),
            None,
        )
        .expect("signature should be valid");

        assert_eq!(descriptor.parameters(), parameters.as_slice());
        assert_eq!(descriptor.parameter_count(), 3);
        assert_eq!(descriptor.results(), &[named_type("core::Output")]);
    }

    #[test]
    fn preserves_multiple_results_and_effects() {
        let effects = vec![name("effects::IO"), name("effects::Network")];
        let results = vec![
            named_type("core::Response"),
            named_type("core::Status"),
        ];

        let descriptor = FunctionTypeDescriptor::new(
            vec![parameter(Some("request"), named_type("core::Request"))],
            results.clone(),
            effects.clone(),
            Some(name("calling::Portable")),
        )
        .expect("signature should be valid");

        assert_eq!(descriptor.results(), results.as_slice());
        assert_eq!(descriptor.effects(), effects.as_slice());
        assert_eq!(
            descriptor.calling_convention(),
            Some(&name("calling::Portable"))
        );
    }

    #[test]
    fn preserves_unnamed_parameters() {
        let descriptor = FunctionTypeDescriptor::new(
            vec![
                parameter(None, named_type("core::A")),
                parameter(None, named_type("core::B")),
            ],
            Vec::new(),
            Vec::new(),
            None,
        )
        .expect("unnamed parameters are representable");

        assert_eq!(descriptor.parameter_count(), 2);
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_explicit_parameter_names() {
        let result = FunctionTypeDescriptor::new(
            vec![
                parameter(Some("value"), named_type("core::A")),
                parameter(Some("value"), named_type("core::B")),
            ],
            Vec::new(),
            Vec::new(),
            None,
        );

        assert!(matches!(
            result,
            Err(FunctionTypeError::DuplicateParameterName(name))
                if name == "value"
        ));
    }

    #[test]
    fn rejects_invalid_nested_inline_types() {
        let invalid_union = TypeRef::Inline(Box::new(TypeKind::Union {
            alternatives: Vec::new(),
        }));

        let result = FunctionTypeDescriptor::new(
            vec![parameter(Some("input"), invalid_union)],
            Vec::new(),
            Vec::new(),
            None,
        );

        assert!(matches!(
            result,
            Err(FunctionTypeError::InvalidCanonicalType(
                TypeDefError::EmptyUnion
            ))
        ));
    }

    #[test]
    fn rejects_non_function_types() {
        assert!(matches!(
            FunctionTypeDescriptor::try_from_kind(&TypeKind::Boolean),
            Err(FunctionTypeError::NotFunction)
        ));
    }

    #[test]
    fn round_trip_preserves_canonical_signature() {
        let canonical = TypeKind::Function {
            parameters: vec![
                FunctionParameter {
                    name: Some("items".to_owned()),
                    ty: named_type("collections::Items"),
                    mode: ParameterMode::Borrow,
                },
                parameter(None, named_type("core::Options")),
            ],
            results: vec![
                named_type("core::Result"),
                named_type("core::Metadata"),
            ],
            effects: vec![name("effects::IO")],
            calling_convention: Some(name("calling::Portable")),
        };

        let descriptor =
            FunctionTypeDescriptor::try_from_kind(&canonical)
                .expect("canonical function type should be valid");

        assert_eq!(descriptor.to_type_kind(), canonical);
        assert_eq!(descriptor.into_type_kind(), canonical);
    }

    #[test]
    fn conversion_trait_uses_the_same_validation() {
        let canonical = TypeKind::Function {
            parameters: Vec::new(),
            results: Vec::new(),
            effects: Vec::new(),
            calling_convention: None,
        };

        let descriptor = FunctionTypeDescriptor::try_from(&canonical)
            .expect("function conversion should succeed");

        let reconstructed: TypeKind = descriptor.into();
        assert_eq!(reconstructed, canonical);
    }

    #[test]
    fn validates_canonical_function_types_directly() {
        let canonical = TypeKind::Function {
            parameters: vec![parameter(
                Some("input"),
                named_type("core::Input"),
            )],
            results: vec![named_type("core::Output")],
            effects: Vec::new(),
            calling_convention: None,
        };

        assert!(validate_function_type(&canonical).is_ok());
        assert!(validate_function_type(&TypeKind::Boolean).is_err());
    }
}
