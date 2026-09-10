//! Zamani Frontend AST — Generic Type Expressions
//!
//! This module provides the generic-type-specific API for the canonical
//! Zamani [`TypeExpr`] representation.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! TypeExpr::Generic
//!     │
//!     ▼
//! GenericType  ← this module
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic generic/type model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target lowering
//! ```
//!
//! # Purpose
//!
//! Generic types are a fundamental part of Zamani's ability to express
//! resource-independent programs.
//!
//! A generic type can describe computation independently of:
//!
//! - machine size;
//! - processor architecture;
//! - quantum technology;
//! - qubit count;
//! - register width;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - execution environment.
//!
//! For example, a source program may use concepts such as:
//!
//! ```text
//! Container<T>
//! Quantum<Q>
//! Matrix<T>
//! Algorithm<Resource, Parameter>
//! ```
//!
//! without determining the eventual physical realization.
//!
//! # Canonical representation
//!
//! `type_expr.rs` is the authoritative owner of the generic type-expression
//! representation:
//!
//! ```text
//! TypeExpr::Generic {
//!     base: Box<TypeExpr>,
//!     arguments: Vec<TypeExpr>,
//! }
//! ```
//!
//! This module intentionally does **not** redefine that structure.
//!
//! [`GenericType`] is a typed façade around the existing canonical variant.
//! This prevents the AST from accumulating two representations of the same
//! concept.
//!
//! # Why a façade instead of another AST node?
//!
//! The native AST must have exactly one authoritative representation for a
//! concept. A second structure such as:
//!
//! ```text
//! struct GenericType {
//!     base: TypeExpr,
//!     arguments: Vec<TypeExpr>,
//! }
//! ```
//!
//! would duplicate the representation already owned by `TypeExpr`.
//!
//! Duplication would eventually cause:
//!
//! - inconsistent validation;
//! - inconsistent serialization;
//! - inconsistent visitor behavior;
//! - conversion bugs;
//! - parser divergence;
//! - semantic-lowering ambiguity;
//! - compatibility problems.
//!
//! Instead, this module wraps `TypeExpr` and guarantees that its contained
//! expression is specifically `TypeExpr::Generic`.
//!
//! # POCO-REAF
//!
//! Generic types are one of the mechanisms that allow Zamani programs to be
//! written independently of resource scale.
//!
//! Generic arity is represented by a dynamically sized `Vec<TypeExpr>`.
//!
//! No constant such as:
//!
//! ```text
//! MAX_GENERIC_ARGUMENTS
//! MAX_QUBITS
//! MAX_RESOURCES
//! MAX_MACHINE_SIZE
//! ```
//!
//! exists here.
//!
//! Any resource limit required for compiler security is supplied explicitly
//! through [`TypeValidationPolicy`], which is already owned by
//! `type_expr.rs`.
//!
//! # Domain neutrality
//!
//! This module does not know whether a generic parameter eventually represents:
//!
//! - a classical value;
//! - a quantum resource;
//! - a logical qubit;
//! - a physical resource;
//! - memory;
//! - an accelerator;
//! - a distributed resource;
//! - an HDL object;
//! - an AI model;
//! - a future computational resource.
//!
//! Those decisions belong to semantic analysis and later compiler phases.
//!
//! # Quantum independence
//!
//! This module deliberately does not contain:
//!
//! - quantum gates;
//! - quantum backends;
//! - qubit topology;
//! - QEC implementation;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse definitions;
//! - physical qubit allocation;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor APIs.
//!
//! A generic type may eventually participate in a quantum program, but the
//! generic-type representation itself remains domain-neutral.
//!
//! # Semantic boundary
//!
//! This module represents source-level syntax only.
//!
//! It does not perform:
//!
//! - name resolution;
//! - generic argument substitution;
//! - type inference;
//! - unification;
//! - trait resolution;
//! - constraint solving;
//! - overload resolution;
//! - resource allocation;
//! - hardware selection.
//!
//! Those responsibilities belong to later compiler stages.
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser constructs the canonical `TypeExpr::Generic` through
//! `TypeExpr::generic(...)` and may obtain a [`GenericType`] view through
//! [`GenericType::try_from_type_expr`] when generic-specific validation or
//! inspection is useful.
//!
//! ## AST validation
//!
//! `GenericType` delegates structural validation to the canonical
//! `TypeExpr::validate_with_policy` implementation.
//!
//! This guarantees that there is one authoritative validation implementation.
//!
//! ## Semantic analysis
//!
//! Semantic analysis consumes `TypeExpr`/`GenericType` and resolves:
//!
//! - the generic constructor;
//! - generic parameters;
//! - generic arguments;
//! - constraints;
//! - substitutions;
//! - associated types;
//! - resource semantics.
//!
//! This module does not perform those operations.
//!
//! ## ZUIR
//!
//! ZUIR lowering must consume the semantic representation rather than using
//! `GenericType` to make target-specific decisions.
//!
//! ## Serialization
//!
//! Serialization remains owned by the canonical `TypeExpr` representation.
//! `GenericType` therefore serializes as the same `TypeExpr::Generic` value
//! rather than inventing a second schema.
//!
//! # Determinism
//!
//! Generic arguments preserve source order through the canonical `Vec` stored
//! by `TypeExpr::Generic`.
//!
//! No unordered collection is used to define generic argument order.
//!
//! # Large programs
//!
//! The implementation performs no recursive traversal of nested type
//! expressions itself. Validation is delegated to `TypeExpr`, whose repository
//! implementation uses an iterative traversal specifically to avoid exhausting
//! the Rust call stack on deeply nested input. 
//!
//! # Rust requirements
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Forbidden dependencies
//!
//! This module must not depend on:
//!
//! - the legacy `crate::ast`;
//! - parser implementation details;
//! - lexer token types;
//! - semantic analysis;
//! - compiler drivers;
//! - ZUIR;
//! - quantum backends;
//! - hardware;
//! - runtime;
//! - QIR;
//! - LLVM;
//! - MLIR.
//!
//! It depends only on the canonical sibling `type_expr` module and the Rust
//! standard library.
//!
//! # Compatibility principle
//!
//! Existing code using:
//!
//! ```text
//! TypeExpr::Generic
//! ```
//!
//! remains valid.
//!
//! `GenericType` is an additional typed API, not a replacement that requires
//! unrelated files to be rewritten.
//!
//! This is important for the repository's incremental AST migration.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::type_expr::{TypeExpr, TypeExprError, TypeValidationPolicy};

// =============================================================================
// GenericType
// =============================================================================

/// A typed view of a canonical [`TypeExpr::Generic`] expression.
///
/// `GenericType` does not introduce a second generic-type representation.
/// Internally it stores the canonical [`TypeExpr`] and guarantees through its
/// constructors that the contained value is the `Generic` variant.
///
/// # Invariant
///
/// A successfully constructed `GenericType` always contains:
///
/// ```text
/// TypeExpr::Generic { .. }
/// ```
///
/// This invariant makes it safe for consumers to inspect generic-specific
/// information without repeatedly matching arbitrary `TypeExpr` values.
///
/// # Example
///
/// ```
/// use zamani::frontend::ast::node::types::generic::GenericType;
/// use zamani::frontend::ast::node::types::type_expr::TypeExpr;
///
/// let generic = GenericType::new(
///     TypeExpr::name("Container"),
///     vec![TypeExpr::name("Element")],
/// );
///
/// assert_eq!(generic.argument_count(), 1);
/// assert_eq!(generic.to_source_string(), "Container<Element>");
/// ```
///
/// The exact crate import path in documentation examples may differ depending
/// on the final crate root/module exports.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GenericType(TypeExpr);

impl GenericType {
    /// Creates a generic type from a base type and ordered type arguments.
    ///
    /// This constructor intentionally does not perform structural validation.
    /// That allows parser and recovery code to construct AST values first and
    /// validate them later using the compiler's selected policy.
    ///
    /// Use [`Self::try_new`] when immediate structural validation is required.
    #[must_use]
    pub fn new(base: TypeExpr, arguments: Vec<TypeExpr>) -> Self {
        Self(TypeExpr::generic(base, arguments))
    }

    /// Creates and structurally validates a generic type using the default
    /// validation policy.
    ///
    /// The default policy imposes no artificial collection or identifier
    /// limits.
    pub fn try_new(
        base: TypeExpr,
        arguments: Vec<TypeExpr>,
    ) -> Result<Self, TypeExprError> {
        let generic = Self::new(base, arguments);
        generic.validate()?;
        Ok(generic)
    }

    /// Creates and structurally validates a generic type using an explicit
    /// compiler security/resource policy.
    ///
    /// The policy belongs to the compilation invocation rather than to the
    /// language representation.
    pub fn try_new_with_policy(
        base: TypeExpr,
        arguments: Vec<TypeExpr>,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, TypeExprError> {
        let generic = Self::new(base, arguments);
        generic.validate_with_policy(policy)?;
        Ok(generic)
    }

    /// Attempts to create a [`GenericType`] from an existing type expression.
    ///
    /// Returns [`GenericTypeError::NotGeneric`] when the expression is not a
    /// generic type application.
    pub fn try_from_type_expr(value: TypeExpr) -> Result<Self, GenericTypeError> {
        match value {
            TypeExpr::Generic { .. } => Ok(Self(value)),
            other => Err(GenericTypeError::NotGeneric {
                variant: other.variant_name(),
            }),
        }
    }

    /// Creates a generic type from an existing expression after validation.
    ///
    /// This is useful at parser/AST boundaries where a generic expression has
    /// already been structurally validated.
    pub fn try_from_type_expr_with_policy(
        value: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, GenericTypeError> {
        let generic = Self::try_from_type_expr(value)?;

        generic
            .validate_with_policy(policy)
            .map_err(GenericTypeError::InvalidType)?;

        Ok(generic)
    }

    /// Returns the canonical underlying [`TypeExpr`].
    ///
    /// This is the primary interoperability boundary with existing AST code.
    #[must_use]
    pub fn as_type_expr(&self) -> &TypeExpr {
        &self.0
    }

    /// Consumes the façade and returns the canonical [`TypeExpr`].
    #[must_use]
    pub fn into_type_expr(self) -> TypeExpr {
        self.0
    }

    /// Returns the generic base/constructor type.
    ///
    /// For:
    ///
    /// ```text
    /// Container<Element>
    /// ```
    ///
    /// this returns:
    ///
    /// ```text
    /// Container
    /// ```
    #[must_use]
    pub fn base(&self) -> &TypeExpr {
        match &self.0 {
            TypeExpr::Generic { base, .. } => base.as_ref(),
            // The constructor invariant makes this branch unreachable through
            // safe public construction. Keeping the match exhaustive makes
            // the implementation robust if the enum changes in the future.
            _ => unreachable!("GenericType invariant violated"),
        }
    }

    /// Returns the ordered generic arguments.
    ///
    /// The returned slice is the canonical argument collection owned by
    /// `TypeExpr::Generic`.
    #[must_use]
    pub fn arguments(&self) -> &[TypeExpr] {
        match &self.0 {
            TypeExpr::Generic { arguments, .. } => arguments,
            _ => unreachable!("GenericType invariant violated"),
        }
    }

    /// Returns the number of generic arguments.
    ///
    /// This is a source-level count and is not a machine-resource count.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments().len()
    }

    /// Returns whether this generic type has no arguments.
    ///
    /// A zero-argument generic application is structurally representable.
    /// Whether it is semantically meaningful is decided later.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.arguments().is_empty()
    }

    /// Returns whether the generic base itself is another generic expression.
    ///
    /// This permits arbitrarily nested source-level generic applications
    /// without imposing a fixed nesting limit.
    #[must_use]
    pub fn has_generic_base(&self) -> bool {
        matches!(self.base(), TypeExpr::Generic { .. })
    }

    /// Returns whether any immediate generic argument requires inference.
    ///
    /// This delegates to the canonical `TypeExpr` representation.
    #[must_use]
    pub fn arguments_require_inference(&self) -> bool {
        self.arguments()
            .iter()
            .any(TypeExpr::requires_inference)
    }

    /// Returns whether the base type requires inference.
    #[must_use]
    pub fn base_requires_inference(&self) -> bool {
        self.base().requires_inference()
    }

    /// Returns whether this generic type requires semantic inference
    /// somewhere in its immediate generic structure.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.0.requires_inference()
    }

    /// Performs structural validation using the default policy.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.0.validate()
    }

    /// Performs structural validation using an explicit compiler policy.
    ///
    /// No language-level maximum is introduced by this method.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Returns the deterministic source-level spelling.
    ///
    /// This does not perform name resolution, alias expansion or semantic
    /// canonicalization.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.0.to_source_string()
    }

    /// Returns the canonical AST variant identifier.
    ///
    /// Always returns `"generic"` for a valid `GenericType`.
    #[must_use]
    pub fn variant_name(&self) -> &'static str {
        self.0.variant_name()
    }

    /// Returns the schema-level representation of this value.
    ///
    /// This is intentionally an owned clone of the canonical `TypeExpr`.
    /// Serialization should normally be performed through `TypeExpr` itself
    /// so the AST has exactly one serialization schema.
    #[must_use]
    pub fn to_type_expr(&self) -> TypeExpr {
        self.0.clone()
    }
}

impl From<GenericType> for TypeExpr {
    fn from(value: GenericType) -> Self {
        value.into_type_expr()
    }
}

impl TryFrom<TypeExpr> for GenericType {
    type Error = GenericTypeError;

    fn try_from(value: TypeExpr) -> Result<Self, Self::Error> {
        Self::try_from_type_expr(value)
    }
}

impl AsRef<TypeExpr> for GenericType {
    fn as_ref(&self) -> &TypeExpr {
        self.as_type_expr()
    }
}

impl fmt::Display for GenericType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors specific to the typed [`GenericType`] façade.
///
/// Structural errors inside a generic expression are delegated to the
/// canonical [`TypeExprError`] implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum GenericTypeError {
    /// The supplied type expression was not a generic application.
    NotGeneric {
        /// Canonical source-AST variant identifier.
        variant: &'static str,
    },

    /// The generic expression failed canonical structural validation.
    InvalidType(TypeExprError),
}

impl fmt::Display for GenericTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotGeneric { variant } => {
                write!(
                    formatter,
                    "expected generic type expression, found `{variant}`"
                )
            }

            Self::InvalidType(error) => {
                write!(formatter, "invalid generic type expression: {error}")
            }
        }
    }
}

impl std::error::Error for GenericTypeError {}

impl From<TypeExprError> for GenericTypeError {
    fn from(value: TypeExprError) -> Self {
        Self::InvalidType(value)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::types::type_expr::TypeExpr;

    #[test]
    fn constructs_generic_type_without_hard_coded_arity() {
        let arguments = vec![
            TypeExpr::name("A"),
            TypeExpr::name("B"),
            TypeExpr::name("C"),
            TypeExpr::name("D"),
        ];

        let generic = GenericType::new(TypeExpr::name("Container"), arguments);

        assert_eq!(generic.argument_count(), 4);
        assert_eq!(generic.to_source_string(), "Container<A, B, C, D>");
    }

    #[test]
    fn preserves_canonical_type_expr_representation() {
        let generic = GenericType::new(
            TypeExpr::name("Container"),
            vec![TypeExpr::name("Element")],
        );

        assert!(matches!(
            generic.as_type_expr(),
            TypeExpr::Generic { .. }
        ));
    }

    #[test]
    fn conversion_to_type_expr_is_lossless() {
        let generic = GenericType::new(
            TypeExpr::name("Map"),
            vec![TypeExpr::name("Key"), TypeExpr::name("Value")],
        );

        let expression = generic.clone().into_type_expr();

        assert_eq!(expression, generic.to_type_expr());
        assert_eq!(expression.to_source_string(), "Map<Key, Value>");
    }

    #[test]
    fn accepts_nested_generic_types() {
        let inner = TypeExpr::generic(
            TypeExpr::name("Inner"),
            vec![TypeExpr::name("Element")],
        );

        let outer = GenericType::new(
            TypeExpr::name("Outer"),
            vec![inner],
        );

        assert_eq!(
            outer.to_source_string(),
            "Outer<Inner<Element>>"
        );
    }

    #[test]
    fn accepts_arbitrarily_structured_arguments() {
        let argument = TypeExpr::tuple(vec![
            TypeExpr::name("Left"),
            TypeExpr::optional(TypeExpr::name("Right")),
            TypeExpr::array(
                TypeExpr::name("Element"),
                None,
            ),
        ]);

        let generic = GenericType::new(
            TypeExpr::name("Container"),
            vec![argument],
        );

        assert_eq!(
            generic.to_source_string(),
            "Container<(Left, ?Right, [Element])>"
        );
    }

    #[test]
    fn rejects_non_generic_expression() {
        let result = GenericType::try_from_type_expr(TypeExpr::name("NotGeneric"));

        assert!(matches!(
            result,
            Err(GenericTypeError::NotGeneric {
                variant: "identifier"
            })
        ));
    }

    #[test]
    fn try_new_validates_using_canonical_validation() {
        let result = GenericType::try_new(
            TypeExpr::name("Container"),
            vec![TypeExpr::name("Element")],
        );

        assert!(result.is_ok());
    }

    #[test]
    fn validation_policy_is_explicit_and_configurable() {
        let policy = TypeValidationPolicy {
            max_collection_items: Some(1),
            ..TypeValidationPolicy::default()
        };

        let result = GenericType::try_new_with_policy(
            TypeExpr::name("Container"),
            vec![
                TypeExpr::name("A"),
                TypeExpr::name("B"),
            ],
            &policy,
        );

        assert!(matches!(
            result,
            Err(TypeExprError::CollectionTooLarge {
                actual: 2,
                maximum: 1
            })
        ));
    }

    #[test]
    fn no_default_generic_limit_is_imposed() {
        let arguments = (0..128)
            .map(|index| TypeExpr::name(format!("T{index}")))
            .collect::<Vec<_>>();

        let generic = GenericType::new(
            TypeExpr::name("Many"),
            arguments,
        );

        assert_eq!(generic.argument_count(), 128);
        assert!(generic.validate().is_ok());
    }

    #[test]
    fn inference_detection_delegates_to_type_expr() {
        let generic = GenericType::new(
            TypeExpr::name("Container"),
            vec![TypeExpr::Infer],
        );

        assert!(generic.arguments_require_inference());
        assert!(generic.requires_inference());
    }

    #[test]
    fn generic_base_inference_is_detected() {
        let generic = GenericType::new(
            TypeExpr::generic(
                TypeExpr::name("Outer"),
                vec![TypeExpr::Infer],
            ),
            vec![TypeExpr::name("Value")],
        );

        assert!(generic.base_requires_inference());
        assert!(generic.requires_inference());
    }

    #[test]
    fn display_matches_source_formatting() {
        let generic = GenericType::new(
            TypeExpr::name("Quantum"),
            vec![TypeExpr::name("Q")],
        );

        assert_eq!(generic.to_string(), "Quantum<Q>");
    }

    #[test]
    fn variant_name_is_canonical() {
        let generic = GenericType::new(
            TypeExpr::name("Container"),
            vec![TypeExpr::name("Element")],
        );

        assert_eq!(generic.variant_name(), "generic");
    }

    #[test]
    fn zero_argument_generic_is_representable() {
        let generic = GenericType::new(
            TypeExpr::name("Marker"),
            Vec::new(),
        );

        assert!(generic.is_empty());
        assert_eq!(generic.to_source_string(), "Marker<>");
    }
}