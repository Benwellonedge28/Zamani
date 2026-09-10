//! Zamani Frontend AST — Tuple Type Expressions
//!
//! This module provides the tuple-type-specific API for the canonical
//! [`TypeExpr`] representation.
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
//! TypeExpr::Tuple
//!     │
//!     ▼
//! TupleType  ← this module
//!     │
//!     ▼
//! structural validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic tuple/type model
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
//! `TupleType` is a typed façade over the canonical
//! [`TypeExpr::Tuple`] representation.
//!
//! It does **not** introduce another tuple AST representation.
//!
//! The authoritative representation remains:
//!
//! ```text
//! TypeExpr::Tuple(Vec<TypeExpr>)
//! ```
//!
//! This design is intentional. It prevents the AST from accumulating
//! multiple competing representations of tuple types.
//!
//! # Why a façade?
//!
//! The native Zamani AST must have exactly one authoritative representation
//! for every source-language concept.
//!
//! A second independent structure such as:
//!
//! ```text
//! struct TupleType {
//!     elements: Vec<TypeExpr>
//! }
//! ```
//!
//! would duplicate the tuple representation already owned by `TypeExpr`.
//!
//! That duplication would eventually cause:
//!
//! - inconsistent validation;
//! - inconsistent serialization;
//! - inconsistent traversal;
//! - parser divergence;
//! - conversion bugs;
//! - semantic-lowering ambiguity;
//! - compatibility problems.
//!
//! Instead, this module wraps `TypeExpr` and guarantees that a successfully
//! constructed `TupleType` always contains `TypeExpr::Tuple`.
//!
//! # POCO-REAF and scalability
//!
//! Tuple arity is represented by the dynamically sized collection already
//! owned by `TypeExpr::Tuple`.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_TUPLE_ELEMENTS
//! MAX_TUPLE_ARITY
//! MAX_RESOURCE_COUNT
//! MAX_QUBITS
//! MAX_MACHINE_SIZE
//! ```
//!
//! Tuple arity is therefore limited only by:
//!
//! 1. available compiler memory;
//! 2. explicitly configured compiler safety/resource policies;
//! 3. the host environment's ability to represent the program.
//!
//! These are compiler-resource concerns rather than Zamani language
//! semantics.
//!
//! The tuple itself carries no information about:
//!
//! - CPU layout;
//! - GPU layout;
//! - FPGA layout;
//! - QPU layout;
//! - qubit count;
//! - physical qubit assignment;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - ABI;
//! - register width;
//! - scheduling;
//! - routing;
//! - calibration;
//! - error correction;
//! - resilience;
//! - QIR;
//! - LLVM;
//! - MLIR.
//!
//! A tuple may therefore contain types representing classical values,
//! quantum resources, symbolic resources, distributed values, accelerator
//! resources, or future computational abstractions without this module
//! needing to know their eventual physical realization.
//!
//! # Source-level semantics
//!
//! `TupleType` represents source-level structure only.
//!
//! It does not perform:
//!
//! - name resolution;
//! - type inference;
//! - unification;
//! - generic substitution;
//! - trait resolution;
//! - overload resolution;
//! - ownership analysis;
//! - borrow checking;
//! - resource allocation;
//! - quantum legality checking;
//! - topology checking;
//! - routing;
//! - scheduling;
//! - hardware selection;
//! - backend lowering.
//!
//! Those responsibilities belong to later compiler stages.
//!
//! # Empty tuples
//!
//! An empty tuple is structurally representable:
//!
//! ```text
//! ()
//! ```
//!
//! This is distinct at the AST level from `TypeExpr::Unit`, even though
//! semantic analysis may decide that the language gives them equivalent or
//! related meaning.
//!
//! The distinction must not be erased here.
//!
//! # Tuple elements
//!
//! Elements retain source order.
//!
//! For example:
//!
//! ```text
//! (A, B, C)
//! ```
//!
//! is represented in exactly that order.
//!
//! The tuple representation imposes no fixed element count.
//!
//! # Validation
//!
//! Structural validation is delegated to the canonical `TypeExpr`
//! implementation. This guarantees that there is one authoritative validation
//! implementation for tuple expressions.
//!
//! Compiler resource limits are supplied through [`TypeValidationPolicy`].
//!
//! No language-level maximum is introduced here.
//!
//! # Serialization
//!
//! Serialization remains owned by the canonical `TypeExpr` representation.
//!
//! `TupleType` therefore does not define an independent serialization schema.
//!
//! Serializing the underlying expression produces the canonical:
//!
//! ```text
//! TypeExpr::Tuple
//! ```
//!
//! representation.
//!
//! # Parser integration
//!
//! The parser should construct the canonical expression:
//!
//! ```rust
//! TypeExpr::tuple(elements)
//! ```
//!
//! and may subsequently obtain a `TupleType` view when tuple-specific
//! inspection is useful.
//!
//! This means existing parser code does not have to be rewritten merely
//! because this façade is introduced.
//!
//! # Semantic integration
//!
//! Semantic analysis consumes the canonical `TypeExpr` representation or a
//! `TupleType` view and resolves each element independently.
//!
//! Conceptually:
//!
//! ```text
//! TupleType
//!     │
//!     ▼
//! TypeExpr::Tuple
//!     │
//!     ▼
//! semantic resolver
//!     │
//!     ▼
//! semantic tuple type
//! ```
//!
//! Tuple-specific semantic rules do not belong in this module.
//!
//! # ZUIR integration
//!
//! `TupleType` does not lower directly to ZUIR.
//!
//! The intended pipeline is:
//!
//! ```text
//! TupleType
//!     │
//!     ▼
//! TypeExpr
//!     │
//!     ▼
//! semantic tuple type
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! Target-specific tuple layout is determined downstream.
//!
//! # Determinism
//!
//! Tuple elements use the canonical ordered `Vec<TypeExpr>` representation.
//!
//! No unordered data structure participates in tuple element ordering.
//!
//! Equal source-level tuple structures therefore produce equal canonical
//! representations and deterministic source formatting.
//!
//! # Large/deep programs
//!
//! `TupleType` does not recursively walk nested tuple elements itself.
//!
//! Validation delegates to `TypeExpr`, whose repository implementation uses
//! iterative validation rather than recursive Rust calls. This is important
//! for deeply nested or adversarial compiler input.
//!
//! # Security
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no execution;
//! - performs no raw pointer operations;
//! - uses no `unsafe`;
//! - does not trust tuple contents as resolved semantic objects;
//! - delegates structural validation to the canonical AST validation layer.
//!
//! # Compatibility
//!
//! Existing code using:
//!
//! ```text
//! TypeExpr::Tuple
//! ```
//!
//! remains valid.
//!
//! `TupleType` is an additional typed API, not a replacement requiring
//! unrelated AST components to change.
//!
//! # Forbidden dependencies
//!
//! This module must not depend on:
//!
//! - the legacy `crate::ast`;
//! - lexer token types;
//! - parser implementation details;
//! - semantic analysis;
//! - compiler drivers;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - quantum backends;
//! - hardware;
//! - routing;
//! - scheduling;
//! - QEC;
//! - calibration;
//! - runtime execution.
//!
//! It depends only on the canonical sibling `type_expr` module and the Rust
//! standard library.
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
//! ```text
//! TupleType
//!     = typed view
//!
//! TypeExpr::Tuple
//!     = canonical AST representation
//!
//! SemanticType
//!     = resolved meaning
//!
//! ZUIR
//!     = universal computational representation
//!
//! Domain IR
//!     = domain-specific representation
//!
//! Target IR
//!     = target/backend representation
//! ```
//!
//! This separation is mandatory for Zamani's POCO-REAF architecture.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use super::type_expr::{TypeExpr, TypeExprError, TypeValidationPolicy};

// =============================================================================
// TupleType
// =============================================================================

/// A typed view of a canonical [`TypeExpr::Tuple`] expression.
///
/// `TupleType` does not introduce an independent tuple representation.
/// Internally it stores the canonical [`TypeExpr`] and guarantees through its
/// constructors that the contained value is the `Tuple` variant.
///
/// # Invariant
///
/// A successfully constructed `TupleType` always contains:
///
/// ```text
/// TypeExpr::Tuple(_)
/// ```
///
/// # Examples
///
/// ```
/// use zamani::frontend::ast::node::types::tuple::TupleType;
/// use zamani::frontend::ast::node::types::type_expr::TypeExpr;
///
/// let tuple = TupleType::new(vec![
///     TypeExpr::name("Left"),
///     TypeExpr::name("Right"),
/// ]);
///
/// assert_eq!(tuple.element_count(), 2);
/// assert_eq!(tuple.to_source_string(), "(Left, Right)");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TupleType(TypeExpr);

impl TupleType {
    /// Creates a tuple type from ordered element types.
    ///
    /// This constructor intentionally does not perform structural validation.
    /// Parser and recovery code can therefore construct an AST first and
    /// validate it later using the compiler's selected policy.
    ///
    /// Use [`Self::try_new`] when immediate validation is required.
    #[must_use]
    pub fn new(elements: Vec<TypeExpr>) -> Self {
        Self(TypeExpr::tuple(elements))
    }

    /// Creates a tuple type from an iterator of ordered element types.
    ///
    /// The iterator is consumed exactly once and its order becomes the
    /// canonical tuple element order.
    #[must_use]
    pub fn from_elements<I>(elements: I) -> Self
    where
        I: IntoIterator<Item = TypeExpr>,
    {
        Self::new(elements.into_iter().collect())
    }

    /// Creates and validates a tuple using the default validation policy.
    ///
    /// The default policy does not impose an artificial tuple-arity limit.
    pub fn try_new(elements: Vec<TypeExpr>) -> Result<Self, TypeExprError> {
        let tuple = Self::new(elements);
        tuple.validate()?;
        Ok(tuple)
    }

    /// Creates and validates a tuple using an explicit compiler resource
    /// policy.
    ///
    /// The policy is a compiler invocation concern and does not become part
    /// of the language's tuple semantics.
    pub fn try_new_with_policy(
        elements: Vec<TypeExpr>,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, TypeExprError> {
        let tuple = Self::new(elements);
        tuple.validate_with_policy(policy)?;
        Ok(tuple)
    }

    /// Attempts to create a [`TupleType`] from an existing type expression.
    ///
    /// Returns [`TupleTypeError::NotTuple`] when the expression is not a
    /// tuple.
    pub fn try_from_type_expr(value: TypeExpr) -> Result<Self, TupleTypeError> {
        match value {
            TypeExpr::Tuple(_) => Ok(Self(value)),
            other => Err(TupleTypeError::NotTuple {
                variant: other.variant_name(),
            }),
        }
    }

    /// Attempts to create a tuple façade and validate the expression using
    /// the supplied compiler policy.
    pub fn try_from_type_expr_with_policy(
        value: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, TupleTypeError> {
        let tuple = Self::try_from_type_expr(value)?;

        tuple
            .validate_with_policy(policy)
            .map_err(TupleTypeError::InvalidType)?;

        Ok(tuple)
    }

    /// Returns a reference to the canonical underlying [`TypeExpr`].
    ///
    /// This is the primary interoperability boundary with the rest of the
    /// frontend AST.
    #[must_use]
    pub fn as_type_expr(&self) -> &TypeExpr {
        &self.0
    }

    /// Consumes the façade and returns the canonical [`TypeExpr`].
    #[must_use]
    pub fn into_type_expr(self) -> TypeExpr {
        self.0
    }

    /// Returns the ordered tuple elements.
    ///
    /// The returned slice is the canonical element collection owned by
    /// `TypeExpr::Tuple`.
    #[must_use]
    pub fn elements(&self) -> &[TypeExpr] {
        match &self.0 {
            TypeExpr::Tuple(elements) => elements,
            _ => unreachable!("TupleType invariant violated"),
        }
    }

    /// Returns the number of tuple elements.
    ///
    /// This is a source-level tuple arity, not a hardware-resource count.
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.elements().len()
    }

    /// Returns whether this tuple has no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.elements().is_empty()
    }

    /// Returns whether this tuple contains exactly one element.
    #[must_use]
    pub fn is_singleton(&self) -> bool {
        self.element_count() == 1
    }

    /// Returns the first tuple element, if present.
    #[must_use]
    pub fn first(&self) -> Option<&TypeExpr> {
        self.elements().first()
    }

    /// Returns the final tuple element, if present.
    #[must_use]
    pub fn last(&self) -> Option<&TypeExpr> {
        self.elements().last()
    }

    /// Returns the tuple element at `index`, if present.
    ///
    /// This method deliberately returns `Option` instead of panicking on an
    /// invalid index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&TypeExpr> {
        self.elements().get(index)
    }

    /// Returns an iterator over tuple elements in source order.
    pub fn iter(&self) -> std::slice::Iter<'_, TypeExpr> {
        self.elements().iter()
    }

    /// Returns whether any element requires semantic type inference.
    ///
    /// The canonical `TypeExpr` implementation determines inference
    /// requirements for each nested element.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.0.requires_inference()
    }

    /// Performs structural validation using the default policy.
    ///
    /// No language-level tuple-arity limit is introduced.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.0.validate()
    }

    /// Performs structural validation using an explicit compiler policy.
    ///
    /// Any collection limit is supplied by the caller and is therefore not
    /// embedded in the Zamani language semantics.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Returns the deterministic source-level representation.
    ///
    /// Semantic resolution and canonicalization are intentionally not
    /// performed.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.0.to_source_string()
    }

    /// Returns the canonical AST variant identifier.
    ///
    /// Always returns `"tuple"` for a valid `TupleType`.
    #[must_use]
    pub fn variant_name(&self) -> &'static str {
        self.0.variant_name()
    }

    /// Returns an owned canonical [`TypeExpr`] representation.
    ///
    /// This is useful when passing a tuple façade into APIs that consume the
    /// general type-expression representation.
    #[must_use]
    pub fn to_type_expr(&self) -> TypeExpr {
        self.0.clone()
    }
}

// =============================================================================
// Standard conversions
// =============================================================================

impl From<TupleType> for TypeExpr {
    fn from(value: TupleType) -> Self {
        value.into_type_expr()
    }
}

impl TryFrom<TypeExpr> for TupleType {
    type Error = TupleTypeError;

    fn try_from(value: TypeExpr) -> Result<Self, Self::Error> {
        Self::try_from_type_expr(value)
    }
}

impl AsRef<TypeExpr> for TupleType {
    fn as_ref(&self) -> &TypeExpr {
        self.as_type_expr()
    }
}

impl fmt::Display for TupleType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors specific to the typed [`TupleType`] façade.
///
/// Structural errors inside tuple elements are delegated to the canonical
/// [`TypeExprError`] implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum TupleTypeError {
    /// The supplied type expression was not a tuple.
    NotTuple {
        /// Canonical source-AST variant identifier.
        variant: &'static str,
    },

    /// The tuple expression failed canonical structural validation.
    InvalidType(TypeExprError),
}

impl fmt::Display for TupleTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotTuple { variant } => {
                write!(
                    formatter,
                    "expected tuple type expression, found `{variant}`"
                )
            }

            Self::InvalidType(error) => {
                write!(formatter, "invalid tuple type expression: {error}")
            }
        }
    }
}

impl std::error::Error for TupleTypeError {}

impl From<TypeExprError> for TupleTypeError {
    fn from(value: TypeExprError) -> Self {
        Self::InvalidType(value)
    }
}

// =============================================================================
// Iterator integration
// =============================================================================

impl<'a> IntoIterator for &'a TupleType {
    type Item = &'a TypeExpr;
    type IntoIter = std::slice::Iter<'a, TypeExpr>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
    fn constructs_empty_tuple() {
        let tuple = TupleType::new(Vec::new());

        assert!(tuple.is_empty());
        assert_eq!(tuple.element_count(), 0);
        assert_eq!(tuple.to_source_string(), "()");
    }

    #[test]
    fn constructs_tuple_without_hard_coded_arity() {
        let tuple = TupleType::new(vec![
            TypeExpr::name("A"),
            TypeExpr::name("B"),
            TypeExpr::name("C"),
            TypeExpr::name("D"),
            TypeExpr::name("E"),
        ]);

        assert_eq!(tuple.element_count(), 5);
        assert_eq!(tuple.to_source_string(), "(A, B, C, D, E)");
    }

    #[test]
    fn preserves_source_order() {
        let tuple = TupleType::new(vec![
            TypeExpr::name("First"),
            TypeExpr::name("Second"),
            TypeExpr::name("Third"),
        ]);

        assert_eq!(tuple.first().and_then(TypeExpr::name), Some("First"));
        assert_eq!(tuple.last().and_then(TypeExpr::name), Some("Third"));

        let names: Vec<&str> = tuple
            .iter()
            .filter_map(TypeExpr::name)
            .collect();

        assert_eq!(names, vec!["First", "Second", "Third"]);
    }

    #[test]
    fn indexed_access_is_non_panicking() {
        let tuple = TupleType::new(vec![
            TypeExpr::name("A"),
            TypeExpr::name("B"),
        ]);

        assert_eq!(tuple.get(0).and_then(TypeExpr::name), Some("A"));
        assert_eq!(tuple.get(1).and_then(TypeExpr::name), Some("B"));
        assert!(tuple.get(2).is_none());
    }

    #[test]
    fn singleton_tuple_is_structurally_supported() {
        let tuple = TupleType::new(vec![TypeExpr::name("Element")]);

        assert!(tuple.is_singleton());
        assert_eq!(tuple.element_count(), 1);
        assert_eq!(tuple.first().and_then(TypeExpr::name), Some("Element"));
    }

    #[test]
    fn preserves_canonical_type_expr_representation() {
        let tuple = TupleType::new(vec![
            TypeExpr::name("Left"),
            TypeExpr::name("Right"),
        ]);

        assert!(matches!(
            tuple.as_type_expr(),
            TypeExpr::Tuple(_)
        ));
    }

    #[test]
    fn conversion_to_type_expr_is_lossless() {
        let tuple = TupleType::new(vec![
            TypeExpr::name("Left"),
            TypeExpr::name("Right"),
        ]);

        let expression = tuple.clone().into_type_expr();

        assert_eq!(expression, tuple.to_type_expr());
        assert!(matches!(expression, TypeExpr::Tuple(_)));
    }

    #[test]
    fn rejects_non_tuple_type_expressions() {
        let result = TupleType::try_from_type_expr(TypeExpr::name("NotATuple"));

        assert!(matches!(
            result,
            Err(TupleTypeError::NotTuple {
                variant: "identifier"
            })
        ));
    }

    #[test]
    fn accepts_nested_tuple_types() {
        let inner = TypeExpr::tuple(vec![
            TypeExpr::name("InnerA"),
            TypeExpr::name("InnerB"),
        ]);

        let outer = TupleType::new(vec![
            TypeExpr::name("Outer"),
            inner,
        ]);

        assert_eq!(
            outer.to_source_string(),
            "(Outer, (InnerA, InnerB))"
        );
    }

    #[test]
    fn accepts_mixed_source_level_types() {
        let tuple = TupleType::new(vec![
            TypeExpr::name("Classical"),
            TypeExpr::quantum(TypeExpr::name("Q")),
            TypeExpr::optional(TypeExpr::name("Maybe")),
            TypeExpr::array(TypeExpr::name("Element"), None),
            TypeExpr::function(
                vec![TypeExpr::name("Input")],
                TypeExpr::name("Output"),
            ),
        ]);

        assert_eq!(tuple.element_count(), 5);
        assert!(tuple.validate().is_ok());
    }

    #[test]
    fn nested_inference_is_detected() {
        let tuple = TupleType::new(vec![
            TypeExpr::name("Concrete"),
            TypeExpr::Infer,
            TypeExpr::optional(TypeExpr::Infer),
        ]);

        assert!(tuple.requires_inference());
    }

    #[test]
    fn explicit_policy_is_forwarded_to_canonical_validation() {
        let tuple = TupleType::new(vec![
            TypeExpr::name("A"),
            TypeExpr::name("B"),
            TypeExpr::name("C"),
        ]);

        let policy = TypeValidationPolicy {
            max_collection_items: Some(2),
            ..TypeValidationPolicy::default()
        };

        let result = tuple.validate_with_policy(&policy);

        assert!(matches!(
            result,
            Err(TypeExprError::CollectionTooLarge {
                actual: 3,
                maximum: 2
            })
        ));
    }

    #[test]
    fn default_validation_has_no_artificial_arity_limit() {
        let tuple = TupleType::new(
            (0..1_024)
                .map(|index| TypeExpr::name(format!("T{index}")))
                .collect(),
        );

        assert_eq!(tuple.element_count(), 1_024);
        assert!(tuple.validate().is_ok());
    }

    #[test]
    fn reference_iterator_preserves_order() {
        let tuple = TupleType::new(vec![
            TypeExpr::name("A"),
            TypeExpr::name("B"),
            TypeExpr::name("C"),
        ]);

        let names: Vec<&str> = (&tuple)
            .into_iter()
            .filter_map(TypeExpr::name)
            .collect();

        assert_eq!(names, vec!["A", "B", "C"]);
    }

    #[test]
    fn display_uses_canonical_type_expression_formatting() {
        let tuple = TupleType::new(vec![
            TypeExpr::name("A"),
            TypeExpr::name("B"),
        ]);

        assert_eq!(tuple.to_string(), "(A, B)");
    }

    #[test]
    fn tuple_is_domain_neutral() {
        let tuple = TupleType::new(vec![
            TypeExpr::quantum(TypeExpr::name("Q")),
            TypeExpr::name("Classical"),
            TypeExpr::name("FutureResource"),
        ]);

        assert!(tuple.validate().is_ok());
    }
}