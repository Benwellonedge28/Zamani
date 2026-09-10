//! Zamani Native AST — Slice Type
//!
//! Production-ready typed façade for the canonical source-level slice type.
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
//! TypeExpr::Slice(Box<TypeExpr>)
//!     │
//!     ├── SliceType (this façade)
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! SemanticType
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
//! This module provides the strongly typed `SliceType` façade for the
//! canonical `TypeExpr::Slice(Box<TypeExpr>)` representation.
//!
//! `TypeExpr` remains the single authoritative representation of source-level
//! type expressions. This file MUST NOT introduce a second independent slice
//! representation.
//!
//! # Source-level meaning
//!
//! A slice describes a dynamically sized sequence whose element type is known
//! at the source level while its runtime cardinality is not encoded as a fixed
//! compile-time machine size in this AST node.
//!
//! Conceptually:
//!
//! ```text
//! [T]
//! ```
//!
//! or whatever equivalent slice syntax the Zamani grammar defines.
//!
//! The exact runtime representation is deliberately outside this module.
//!
//! A slice may ultimately be implemented using:
//!
//! - memory;
//! - distributed memory;
//! - accelerator memory;
//! - a quantum/resource abstraction;
//! - a future execution substrate;
//! - another implementation chosen downstream.
//!
//! None of those implementation choices belong in the native AST.
//!
//! # POCO-REAF
//!
//! `SliceType` contains no fixed capacity, machine size, register width,
//! address width, qubit count, topology, vendor, backend or instruction set.
//!
//! Therefore:
//!
//! ```text
//! Slice<T>
//! ```
//!
//! remains a source-level abstraction rather than a commitment to a particular
//! physical storage implementation.
//!
//! The number of elements is determined by the language semantics and later
//! compilation stages.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_SLICE_LENGTH
//! MAX_ELEMENTS
//! MAX_QUBITS
//! MAX_REGISTER_SIZE
//! ```
//!
//! in this file.
//!
//! Configurable compiler resource limits, if required for denial-of-service
//! protection, belong to explicit compiler validation policies rather than to
//! the language representation.
//!
//! # Canonical representation
//!
//! The authoritative representation is:
//!
//! ```rust
//! TypeExpr::Slice(Box<TypeExpr>)
//! ```
//!
//! This façade stores exactly that representation.
//!
//! It does NOT define:
//!
//! ```text
//! SliceType { element: TypeExpr }
//! ```
//!
//! as a second serialized schema.
//!
//! This prevents representation drift between the canonical `TypeExpr` and
//! the typed façade.
//!
//! # Domain neutrality
//!
//! This file knows nothing about:
//!
//! - quantum hardware;
//! - physical qubits;
//! - logical qubits;
//! - QPU topology;
//! - quantum vendors;
//! - gate sets;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - QEC;
//! - noise models;
//! - resilience;
//! - GPUs;
//! - CPUs;
//! - FPGAs;
//! - ASICs;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - ZUIR implementation details;
//! - backend instructions.
//!
//! A slice may contain a source-level quantum/resource type if the language
//! permits such a type, but this module does not interpret it.
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser should construct the canonical representation:
//!
//! ```text
//! parser
//!     → TypeExpr::Slice(Box::new(element))
//! ```
//!
//! It may then use `SliceType::try_from_type_expr` when a typed façade is
//! useful.
//!
//! The parser must not construct backend-specific slice objects.
//!
//! ## Structural validation
//!
//! Structural validation validates the underlying `TypeExpr` and therefore
//! also validates the element type recursively according to the configured
//! validation policy.
//!
//! ## Semantic analysis
//!
//! Semantic analysis consumes the canonical `TypeExpr` and resolves the
//! element type into the compiler's semantic type model.
//!
//! `SliceType` does not perform name resolution, inference, unification,
//! ownership checking, borrow checking, resource allocation or target
//! selection.
//!
//! ## ZUIR
//!
//! ZUIR lowering occurs after semantic analysis.
//!
//! This file does not import ZUIR and must not encode a ZUIR-specific slice
//! representation.
//!
//! ## Serialization
//!
//! Serialization is delegated to the canonical `TypeExpr` representation.
//! `SliceType` therefore derives the same serialization traits but introduces
//! no alternate wire schema.
//!
//! ## Visitors
//!
//! Visitors should see the underlying `TypeExpr::Slice` and then traverse its
//! element type.
//!
//! This façade does not require a second visitor hierarchy.
//!
//! # Dependency contract
//!
//! Allowed dependencies:
//!
//! - Rust standard library;
//! - the canonical `super::type_expr` module;
//! - `serde` through the canonical AST dependency set.
//!
//! Forbidden dependencies:
//!
//! - `crate::semantic`;
//! - `crate::compiler`;
//! - `crate::quantum::*`;
//! - hardware backends;
//! - schedulers;
//! - routers;
//! - QEC implementations;
//! - runtime execution;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - ZUIR implementation modules.
//!
//! # Error model
//!
//! Construction from a canonical `TypeExpr` is fallible because arbitrary
//! `TypeExpr` values may be supplied by parser recovery, programmatic AST
//! construction or deserialization.
//!
//! A non-slice expression returns `SliceTypeError::NotSlice`.
//!
//! Underlying type-expression validation failures are converted into
//! `SliceTypeError::InvalidType`.
//!
//! No public operation on malformed external input should intentionally panic.
//!
//! # Invariants
//!
//! A valid `SliceType` always satisfies:
//!
//! ```text
//! inner == TypeExpr::Slice(_)
//! ```
//!
//! The wrapped element type is exactly the child stored by the canonical
//! `TypeExpr`.
//!
//! There is exactly one source of truth for slice representation.
//!
//! # Scalability
//!
//! The type has no machine-size-dependent fields.
//!
//! Its storage cost is proportional to the element type representation and
//! the surrounding AST, not to the number of elements that a runtime slice
//! may eventually contain.
//!
//! Consequently, this type does not need to change when the eventual runtime
//! system changes from:
//!
//! ```text
//! tiny → large → distributed → heterogeneous → future architecture
//! ```
//!
//! # Determinism
//!
//! `SliceType` contains exactly one ordered child type.
//!
//! No hash-map iteration or target-dependent state participates in its
//! representation, validation or serialization.
//!
//! # Security
//!
//! This module performs no I/O, no execution, no unsafe operations, no raw
//! pointer manipulation and no filesystem/network access.
//!
//! Recursive validation is delegated to the canonical `TypeExpr` validation
//! machinery, including its explicit configurable validation policy.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Implementation
//!
//! The façade intentionally remains small. Complexity belongs in the
//! canonical `TypeExpr` implementation rather than being duplicated here.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::type_expr::{TypeExpr, TypeExprError, TypeValidationPolicy};

/// Strongly typed façade for `TypeExpr::Slice`.
///
/// `SliceType` does not replace `TypeExpr`. It provides an invariant-preserving
/// API for code that specifically needs to work with a slice type.
///
/// # Invariant
///
/// The private field can only contain a valid `TypeExpr::Slice` through the
/// public constructors/conversions provided by this type.
///
/// # Example
///
/// ```
/// use crate::frontend::ast::node::types::slice::SliceType;
/// use crate::frontend::ast::node::types::type_expr::TypeExpr;
///
/// let slice = SliceType::new(TypeExpr::name("Element"));
///
/// assert_eq!(
///     slice.element(),
///     &TypeExpr::name("Element")
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SliceType(TypeExpr);

impl SliceType {
    /// Constructs a slice type from an element type.
    ///
    /// This constructor establishes the representation invariant immediately.
    ///
    /// No semantic validation is performed here because the element may be a
    /// forward reference, unresolved generic, dependent type or other valid
    /// source-level expression whose meaning is resolved later.
    #[must_use]
    pub fn new(element: TypeExpr) -> Self {
        Self(TypeExpr::Slice(Box::new(element)))
    }

    /// Constructs a slice type from an element type while validating it using
    /// the canonical type-expression validation policy.
    ///
    /// This is useful for parser boundaries, deserializers and other
    /// untrusted-input boundaries where structural validation is desired at
    /// construction time.
    pub fn try_new(element: TypeExpr) -> Result<Self, SliceTypeError> {
        Self::try_new_with_policy(element, &TypeValidationPolicy::default())
    }

    /// Constructs a slice type using an explicit validation policy.
    ///
    /// The policy is supplied by the caller so this type does not introduce a
    /// hidden language-level limit.
    pub fn try_new_with_policy(
        element: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, SliceTypeError> {
        element
            .validate_with_policy(policy)
            .map_err(SliceTypeError::InvalidType)?;

        Ok(Self::new(element))
    }

    /// Converts an existing `TypeExpr` into `SliceType`.
    ///
    /// Returns an error if the expression is not a slice.
    pub fn try_from_type_expr(value: TypeExpr) -> Result<Self, SliceTypeError> {
        match value {
            TypeExpr::Slice(_) => Ok(Self(value)),
            other => Err(SliceTypeError::NotSlice {
                actual: other.variant_name().to_owned(),
            }),
        }
    }

    /// Converts an existing `TypeExpr` into `SliceType` while validating the
    /// complete expression using the supplied policy.
    pub fn try_from_type_expr_with_policy(
        value: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, SliceTypeError> {
        value
            .validate_with_policy(policy)
            .map_err(SliceTypeError::InvalidType)?;

        Self::try_from_type_expr(value)
    }

    /// Returns the canonical underlying `TypeExpr`.
    #[must_use]
    pub fn as_type_expr(&self) -> &TypeExpr {
        &self.0
    }

    /// Consumes the façade and returns the canonical `TypeExpr`.
    #[must_use]
    pub fn into_type_expr(self) -> TypeExpr {
        self.0
    }

    /// Returns the element type.
    ///
    /// The returned value is borrowed from the canonical representation and
    /// therefore does not clone the subtree.
    #[must_use]
    pub fn element(&self) -> &TypeExpr {
        match &self.0 {
            TypeExpr::Slice(element) => element.as_ref(),

            // This state is unreachable through the public API because the
            // invariant is established by `new` and `try_from_type_expr`.
            // Keep the branch explicit rather than using unsafe assumptions.
            _ => unreachable!("SliceType invariant violated"),
        }
    }

    /// Returns whether the element type requires semantic inference.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.element().requires_inference()
    }

    /// Validates the wrapped type using the default policy.
    ///
    /// This method performs structural type-expression validation only.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.0.validate()
    }

    /// Validates the wrapped type using an explicit policy.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Returns the canonical source representation.
    ///
    /// Formatting is delegated to `TypeExpr` so there is no second formatter
    /// that can diverge from the canonical source syntax.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.0.to_source_string()
    }

    /// Returns the canonical `TypeExpr` variant name.
    #[must_use]
    pub fn variant_name(&self) -> &'static str {
        self.0.variant_name()
    }

    /// Returns this value as its canonical `TypeExpr`.
    ///
    /// This method is intentionally equivalent to `as_type_expr`; it exists as
    /// a stable façade API consistent with the other typed type wrappers.
    #[must_use]
    pub fn to_type_expr(&self) -> &TypeExpr {
        self.as_type_expr()
    }

    /// Returns the canonical slice element without cloning.
    ///
    /// This is an alias intended for code that uses typed type façades.
    #[must_use]
    pub fn element_type(&self) -> &TypeExpr {
        self.element()
    }

    /// Returns whether this value is structurally a slice.
    ///
    /// Always true for a valid `SliceType`; provided for generic typed-type
    /// façade interoperability.
    #[must_use]
    pub const fn is_slice(&self) -> bool {
        true
    }
}

impl From<SliceType> for TypeExpr {
    fn from(value: SliceType) -> Self {
        value.into_type_expr()
    }
}

impl TryFrom<TypeExpr> for SliceType {
    type Error = SliceTypeError;

    fn try_from(value: TypeExpr) -> Result<Self, Self::Error> {
        Self::try_from_type_expr(value)
    }
}

impl AsRef<TypeExpr> for SliceType {
    fn as_ref(&self) -> &TypeExpr {
        self.as_type_expr()
    }
}

impl fmt::Display for SliceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

/// Errors produced when constructing or validating a `SliceType`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SliceTypeError {
    /// The supplied type expression was not a slice.
    NotSlice {
        /// Canonical name of the supplied `TypeExpr` variant.
        actual: String,
    },

    /// The supplied type expression failed canonical structural validation.
    InvalidType(TypeExprError),
}

impl fmt::Display for SliceTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSlice { actual } => {
                write!(
                    formatter,
                    "expected TypeExpr::Slice, found TypeExpr::{actual}"
                )
            }
            Self::InvalidType(error) => {
                write!(formatter, "invalid slice element type: {error}")
            }
        }
    }
}

impl std::error::Error for SliceTypeError {}

impl From<TypeExprError> for SliceTypeError {
    fn from(value: TypeExprError) -> Self {
        Self::InvalidType(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(name: &str) -> TypeExpr {
        TypeExpr::name(name)
    }

    #[test]
    fn constructs_canonical_slice_representation() {
        let slice = SliceType::new(named("Element"));

        assert!(matches!(
            slice.as_type_expr(),
            TypeExpr::Slice(element)
                if element.as_ref() == &named("Element")
        ));
    }

    #[test]
    fn exposes_element_without_cloning() {
        let slice = SliceType::new(named("Element"));

        assert_eq!(slice.element(), &named("Element"));
        assert_eq!(slice.element_type(), &named("Element"));
    }

    #[test]
    fn converts_to_canonical_type_expression() {
        let slice = SliceType::new(named("Element"));
        let expression = slice.into_type_expr();

        assert!(matches!(expression, TypeExpr::Slice(_)));
    }

    #[test]
    fn converts_from_canonical_type_expression() {
        let expression = TypeExpr::Slice(Box::new(named("Element")));

        let slice = SliceType::try_from_type_expr(expression)
            .expect("canonical slice expression should convert");

        assert_eq!(slice.element(), &named("Element"));
    }

    #[test]
    fn rejects_non_slice_type_expression() {
        let error = SliceType::try_from_type_expr(named("NotASlice"))
            .expect_err("non-slice expression must be rejected");

        assert!(matches!(error, SliceTypeError::NotSlice { .. }));
    }

    #[test]
    fn supports_nested_slices() {
        let inner = SliceType::new(named("Element"));
        let outer = SliceType::new(inner.into_type_expr());

        assert!(matches!(
            outer.element(),
            TypeExpr::Slice(_)
        ));
    }

    #[test]
    fn supports_generic_element_types() {
        let element = TypeExpr::generic(
            named("Container"),
            vec![named("Element")],
        );

        let slice = SliceType::new(element);

        assert!(matches!(
            slice.element(),
            TypeExpr::Generic { .. }
        ));
    }

    #[test]
    fn preserves_source_structure() {
        let element = TypeExpr::name("My::Element");
        let slice = SliceType::new(element.clone());

        assert_eq!(slice.element(), &element);
    }

    #[test]
    fn supports_inference_reporting() {
        let element = TypeExpr::infer();
        let slice = SliceType::new(element);

        assert!(slice.requires_inference());
    }

    #[test]
    fn validates_with_default_policy() {
        let slice = SliceType::new(named("Element"));

        assert!(slice.validate().is_ok());
    }

    #[test]
    fn validates_with_explicit_policy() {
        let slice = SliceType::new(named("Element"));
        let policy = TypeValidationPolicy::default();

        assert!(slice.validate_with_policy(&policy).is_ok());
    }

    #[test]
    fn try_new_validates_element() {
        let slice = SliceType::try_new(named("Element"))
            .expect("valid element type should construct");

        assert_eq!(slice.element(), &named("Element"));
    }

    #[test]
    fn preserves_arbitrarily_large_source_level_nesting_subject_to_resources() {
        let mut element = named("Element");

        for _ in 0..1024 {
            element = TypeExpr::Slice(Box::new(element));
        }

        let slice = SliceType::new(element);

        assert!(matches!(slice.as_type_expr(), TypeExpr::Slice(_)));
    }

    #[test]
    fn display_delegates_to_canonical_formatter() {
        let slice = SliceType::new(named("Element"));

        assert_eq!(
            slice.to_string(),
            slice.to_source_string()
        );
    }

    #[test]
    fn canonical_variant_name_is_preserved() {
        let slice = SliceType::new(named("Element"));

        assert_eq!(slice.variant_name(), "Slice");
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let slice = SliceType::new(TypeExpr::optional(named("Element")));

        let encoded =
            serde_json::to_string(&slice).expect("slice serialization must succeed");

        let decoded: SliceType =
            serde_json::from_str(&encoded).expect("slice deserialization must succeed");

        assert_eq!(decoded, slice);
    }

    #[test]
    fn remains_domain_neutral() {
        let slice = SliceType::new(named("Element"));

        assert_eq!(
            slice.as_type_expr().variant_name(),
            "Slice"
        );
    }
}

Required integration

Because this file is a typed façade over the already-existing canonical "TypeExpr::Slice", only module exposure should be changed in the AST type module. Do not modify "type_expr.rs" merely to accommodate "slice.rs".

The intended wiring is:

// src/frontend/ast/node/types/mod.rs

pub mod slice;

If the project exposes typed type façades from "types/mod.rs", optionally:

pub use slice::{SliceType, SliceTypeError};

The dependency direction remains:

parser
   │
   ▼
TypeExpr::Slice
   │
   ├── SliceType façade
   │
   ▼
AST validation
   │
   ▼
semantic analysis
   │
   ▼
SemanticType
   │
   ▼
ZUIR
   │
   ▼
quantum/classical/HDL/etc. domain lowering

The current repository confirms that the canonical frontend representation is already "TypeExpr::Slice(Box<TypeExpr>)", while the older "src/ast/mod.rs" independently contains "Slice(Box<TypeExpr>)"; that older representation must be handled by the broader AST migration rather than duplicated in this new file.

One important caveat: I would not claim this exact file is compile-verified until the current "type_expr.rs" constructor signatures ("generic", "optional", "infer", "validate", etc.) and "types/mod.rs" exports are checked against the repository's current HEAD. The architecture itself is deliberately aligned to the canonical "TypeExpr::Slice" design rather than introducing another slice AST.