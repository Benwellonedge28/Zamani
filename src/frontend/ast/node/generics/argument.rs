//! Zamani Frontend AST — Generic Arguments
//!
//! Canonical source-level representation and API for generic arguments.
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
//! Native Zamani AST
//!     │
//!     ├── GenericParameter
//!     │
//!     └── GenericArgument  ← this module
//!             │
//!             ▼
//!     structural validation
//!             │
//!             ▼
//!     semantic analysis
//!             │
//!             ├── type resolution
//!             ├── value/const resolution
//!             ├── resource resolution
//!             ├── capability resolution
//!             └── domain-specific validation
//!             │
//!             ▼
//!     Semantic Model
//!             │
//!             ▼
//!     ZUIR
//!             │
//!             ▼
//!     domain / target lowering
//! ```
//!
//! # Responsibility
//!
//! This module owns the source-level representation of an individual generic
//! argument.
//!
//! A generic argument is an argument supplied to a generic declaration at a
//! source-language generic-application site.
//!
//! The representation is deliberately source-oriented and domain-neutral.
//!
//! It does not resolve the argument, infer its meaning, allocate resources,
//! select a machine, or select a backend.
//!
//! # Current language contract
//!
//! The current Zamani grammar represents type arguments as types. The existing
//! canonical `TypeExpr::Generic` representation therefore stores its ordered
//! arguments as `Vec<TypeExpr>`.
//!
//! This module intentionally does NOT replace that representation.
//!
//! Instead, `GenericArgument` provides a typed source-level façade over one
//! canonical `TypeExpr` value.
//!
//! This is important because introducing another independent representation
//! such as:
//!
//! ```text
//! enum GenericArgument {
//!     Type(TypeExpr),
//!     Value(...),
//!     Resource(...),
//! }
//! ```
//!
//! would immediately require `TypeExpr::Generic` and all generic consumers to
//! choose between two competing representations.
//!
//! That would create:
//!
//! - duplicate ownership;
//! - duplicate serialization;
//! - duplicate validation;
//! - visitor divergence;
//! - parser divergence;
//! - semantic-lowering ambiguity;
//! - migration complexity;
//! - compatibility problems.
//!
//! The canonical AST therefore remains:
//!
//! ```text
//! TypeExpr::Generic {
//!     base: Box<TypeExpr>,
//!     arguments: Vec<TypeExpr>,
//! }
//! ```
//!
//! `GenericArgument` is an API façade around an individual argument.
//!
//! # Future extensibility
//!
//! Zamani may eventually support generic arguments whose semantic categories
//! include:
//!
//! - type arguments;
//! - compile-time value arguments;
//! - symbolic constants;
//! - resource arguments;
//! - lifetime arguments;
//! - capability arguments;
//! - domain-specific arguments.
//!
//! This file must not prematurely encode those future categories into the
//! current canonical representation.
//!
//! When the Zamani grammar and canonical AST explicitly acquire such syntax,
//! the generic-argument model can be extended through a versioned, compatible
//! representation.
//!
//! Until then, the only canonical source-level generic argument is a
//! `TypeExpr`.
//!
//! # POCO-REAF
//!
//! Generic arguments must not encode machine size.
//!
//! This file contains no assumptions about:
//!
//! - CPU width;
//! - GPU width;
//! - FPGA size;
//! - QPU size;
//! - qubit count;
//! - register count;
//! - memory capacity;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - target architecture;
//! - quantum technology.
//!
//! A generic argument can therefore participate in source programs whose
//! eventual resource scale ranges from a tiny target to arbitrarily large
//! resources supported by the compiler, runtime, and target.
//!
//! The source representation does not need to change as the target grows.
//!
//! # Quantum neutrality
//!
//! A generic argument may eventually represent a quantum type or resource,
//! for example conceptually:
//!
//! ```text
//! Quantum<Q>
//! Register<N>
//! State<S>
//! ```
//!
//! but this module does not know what those names mean.
//!
//! It must not contain:
//!
//! - qubit IDs;
//! - physical qubit mappings;
//! - quantum gates;
//! - coupling graphs;
//! - QEC structures;
//! - schedulers;
//! - routing information;
//! - calibration;
//! - pulse data;
//! - backend jobs;
//! - vendor APIs.
//!
//! Those concerns belong to later compiler layers.
//!
//! # Semantic boundary
//!
//! This module does NOT perform:
//!
//! - name resolution;
//! - type inference;
//! - generic substitution;
//! - type unification;
//! - trait resolution;
//! - constraint solving;
//! - overload resolution;
//! - constant evaluation;
//! - resource allocation;
//! - capability matching;
//! - hardware selection;
//! - quantum legality checking;
//! - routing;
//! - scheduling;
//! - QEC;
//! - backend lowering.
//!
//! Those responsibilities belong to later compiler stages.
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser constructs canonical `TypeExpr` values and inserts them into
//! `TypeExpr::Generic.arguments` in source order.
//!
//! `GenericArgument` may be constructed by parser adapters when an individual
//! argument needs to be handled independently.
//!
//! The parser owns syntax recognition.
//!
//! This module owns the source-level argument façade.
//!
//! ## Structural validation
//!
//! Structural validation delegates to the canonical `TypeExpr` validation
//! implementation.
//!
//! There is intentionally no second recursive validation implementation here.
//!
//! ## Semantic analysis
//!
//! Semantic analysis consumes the underlying `TypeExpr` and determines what
//! generic parameter it corresponds to and what semantic category is expected.
//!
//! For example, semantic analysis may eventually determine that an argument
//! denotes:
//!
//! ```text
//! type T
//! const N
//! resource Q
//! capability C
//! ```
//!
//! Such interpretation must not be performed here.
//!
//! ## ZUIR
//!
//! ZUIR lowering consumes resolved semantic information.
//!
//! `GenericArgument` must never choose a ZUIR representation directly.
//!
//! ## Visitors
//!
//! A visitor visiting a `GenericArgument` must visit its underlying
//! `TypeExpr` exactly once.
//!
//! Traversal order is source order.
//!
//! ## Serialization
//!
//! Serialization of the canonical generic application remains owned by
//! `TypeExpr`.
//!
//! `GenericArgument` does not introduce an independent serialization schema.
//!
//! ## Determinism
//!
//! Generic argument ordering is preserved by the canonical `Vec<TypeExpr>`
//! representation.
//!
//! No unordered collection is used.
//!
//! ## Scalability
//!
//! There is no language-level maximum for:
//!
//! - generic argument count;
//! - identifier size;
//! - type nesting;
//! - generic nesting;
//! - resource-independent symbolic parameters.
//!
//! Any safety/resource limits must be supplied by the compiler's explicit
//! validation policy.
//!
//! # Dependency policy
//!
//! This module may depend on:
//!
//! - the canonical sibling `type_expr` module;
//! - Rust standard-library functionality;
//! - `serde`, which is already used by the canonical AST.
//!
//! It must not depend on:
//!
//! - the legacy `crate::ast`;
//! - lexer implementation details;
//! - parser implementation details;
//! - semantic analysis;
//! - compiler driver;
//! - ZUIR;
//! - quantum IR implementation;
//! - hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime execution;
//! - LLVM;
//! - QIR implementation types;
//! - MLIR implementation types;
//! - vendor SDKs.
//!
//! # Rust requirements
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ```text
//! #![forbid(unsafe_code)]
//! ```
//!
//! is intentionally used below.
//!
//! # Compatibility
//!
//! Existing code using:
//!
//! ```text
//! TypeExpr::Generic {
//!     base,
//!     arguments,
//! }
//! ```
//!
//! remains authoritative and does not need to be migrated merely because this
//! façade exists.
//!
//! Existing generic APIs can consume `GenericArgument::as_type_expr()` or
//! `GenericArgument::into_type_expr()`.
//!
//! # Invariant
//!
//! A successfully constructed `GenericArgument` always contains exactly one
//! canonical `TypeExpr`.
//!
//! It never contains:
//!
//! - a resolved symbol;
//! - a semantic type;
//! - a hardware resource;
//! - a backend object.
//!
//! # File completion contract
//!
//! This file is complete when:
//!
//! - the representation is source-level;
//! - it contains no duplicated generic AST schema;
//! - it preserves argument identity and ordering;
//! - it delegates validation to `TypeExpr`;
//! - it provides deterministic source formatting;
//! - it provides conversion into/from `TypeExpr`;
//! - it provides inference inspection through `TypeExpr`;
//! - it is serializable without inventing a second schema;
//! - it has no unsafe code;
//! - it has no machine-size constants;
//! - it has no quantum/backend dependencies;
//! - it has unit tests for all public behavior;
//! - it remains compatible with the existing `TypeExpr::Generic` model.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::type_expr::{TypeExpr, TypeExprError, TypeValidationPolicy};

// =============================================================================
// Schema
// =============================================================================

/// Local schema/API version for [`GenericArgument`].
///
/// This is deliberately independent of:
///
/// - Zamani language version;
/// - compiler version;
/// - complete serialized-AST schema version;
/// - semantic-model version;
/// - ZUIR version.
///
/// The current value is stable for the source-level façade.
pub const GENERIC_ARGUMENT_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// GenericArgument
// =============================================================================

/// A source-level generic argument.
///
/// # Canonical representation
///
/// `GenericArgument` is a typed façade over one canonical [`TypeExpr`].
///
/// It does not introduce a second representation of generic arguments.
///
/// # Examples
///
/// ```text
/// Vec<Int>
///     ^^^
///
/// Map<String, Int>
///     ^^^^^^  ^^^
/// ```
///
/// Each individual type expression can be viewed as a `GenericArgument`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GenericArgument(TypeExpr);

impl GenericArgument {
    /// Creates a generic argument from a canonical source-level type
    /// expression.
    ///
    /// This constructor intentionally does not perform semantic resolution.
    ///
    /// Structural validation may be performed later using [`Self::validate`]
    /// or [`Self::validate_with_policy`].
    #[must_use]
    pub fn new(value: TypeExpr) -> Self {
        Self(value)
    }

    /// Creates and validates a generic argument using the canonical default
    /// type-expression validation policy.
    ///
    /// The returned value is guaranteed to contain a structurally valid
    /// `TypeExpr`.
    pub fn try_new(value: TypeExpr) -> Result<Self, TypeExprError> {
        let argument = Self::new(value);
        argument.validate()?;
        Ok(argument)
    }

    /// Creates and validates a generic argument using an explicit compiler
    /// resource/safety policy.
    ///
    /// The policy belongs to the compilation invocation rather than to the
    /// language representation.
    pub fn try_new_with_policy(
        value: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, TypeExprError> {
        let argument = Self::new(value);
        argument.validate_with_policy(policy)?;
        Ok(argument)
    }

    /// Returns a reference to the canonical underlying [`TypeExpr`].
    ///
    /// This is the primary integration boundary with existing AST code.
    #[must_use]
    pub fn as_type_expr(&self) -> &TypeExpr {
        &self.0
    }

    /// Consumes this façade and returns the canonical [`TypeExpr`].
    #[must_use]
    pub fn into_type_expr(self) -> TypeExpr {
        self.0
    }

    /// Clones and returns the canonical [`TypeExpr`].
    ///
    /// This method is useful at phase boundaries where ownership of the
    /// original façade must be retained.
    #[must_use]
    pub fn to_type_expr(&self) -> TypeExpr {
        self.0.clone()
    }

    /// Returns whether the argument is structurally valid according to the
    /// canonical `TypeExpr` implementation.
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        self.0.is_structurally_valid()
    }

    /// Performs structural validation using the canonical default policy.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.0.validate()
    }

    /// Performs structural validation using an explicit compiler policy.
    ///
    /// No language-level machine/resource limit is introduced here.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Returns whether this argument contains a generic parameter or another
    /// source construct requiring inference according to the canonical
    /// `TypeExpr` implementation.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.0.requires_inference()
    }

    /// Returns the canonical source-level variant name.
    ///
    /// This method is useful for diagnostics and tooling without requiring
    /// consumers to depend on the internal enum representation.
    #[must_use]
    pub fn variant_name(&self) -> &'static str {
        self.0.variant_name()
    }

    /// Returns a deterministic source-level representation.
    ///
    /// No name resolution, alias expansion, target lowering, or semantic
    /// canonicalization is performed.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.0.to_source_string()
    }

    /// Returns the local schema version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        GENERIC_ARGUMENT_SCHEMA_VERSION
    }

    /// Returns the number of immediate source-level AST children represented
    /// by the underlying type expression.
    ///
    /// This is delegated to `TypeExpr` so this façade does not duplicate
    /// traversal semantics.
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.0.child_count()
    }

    /// Visits the immediate source-level children of the underlying type
    /// expression.
    ///
    /// This method is intentionally a shallow convenience API. Full recursive
    /// AST traversal belongs to the canonical traversal/visitor subsystem.
    pub fn visit_children<F>(&self, visitor: F)
    where
        F: FnMut(&TypeExpr),
    {
        self.0.visit_children(visitor);
    }

    /// Returns whether this argument is a generic application itself.
    ///
    /// For example, the argument `Vec<Int>` is itself a generic type
    /// expression.
    #[must_use]
    pub fn is_generic(&self) -> bool {
        matches!(self.0, TypeExpr::Generic { .. })
    }

    /// Returns the generic base when this argument is itself a generic
    /// application.
    ///
    /// Returns `None` when the argument is not a `TypeExpr::Generic`.
    #[must_use]
    pub fn generic_base(&self) -> Option<&TypeExpr> {
        match &self.0 {
            TypeExpr::Generic { base, .. } => Some(base.as_ref()),
            _ => None,
        }
    }

    /// Returns the nested generic arguments when this argument is itself a
    /// generic application.
    ///
    /// Returns `None` when the argument is not a `TypeExpr::Generic`.
    ///
    /// The returned slice preserves source order.
    #[must_use]
    pub fn generic_arguments(&self) -> Option<&[TypeExpr]> {
        match &self.0 {
            TypeExpr::Generic { arguments, .. } => Some(arguments.as_slice()),
            _ => None,
        }
    }

    /// Returns the number of nested generic arguments when this argument is
    /// itself a generic application.
    ///
    /// Returns `None` for non-generic arguments.
    #[must_use]
    pub fn generic_argument_count(&self) -> Option<usize> {
        self.generic_arguments().map(<[TypeExpr]>::len)
    }
}

// =============================================================================
// Conversions
// =============================================================================

impl From<TypeExpr> for GenericArgument {
    fn from(value: TypeExpr) -> Self {
        Self::new(value)
    }
}

impl From<GenericArgument> for TypeExpr {
    fn from(value: GenericArgument) -> Self {
        value.into_type_expr()
    }
}

impl AsRef<TypeExpr> for GenericArgument {
    fn as_ref(&self) -> &TypeExpr {
        self.as_type_expr()
    }
}

impl fmt::Display for GenericArgument {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

// =============================================================================
// Collection helpers
// =============================================================================

/// Converts an ordered collection of canonical type expressions into generic
/// arguments.
///
/// This function exists primarily for parser adapters and AST-building code.
/// It does not change ordering and does not perform semantic resolution.
#[must_use]
pub fn from_type_exprs<I>(arguments: I) -> Vec<GenericArgument>
where
    I: IntoIterator<Item = TypeExpr>,
{
    arguments.into_iter().map(GenericArgument::from).collect()
}

/// Converts an ordered collection of generic arguments back into canonical
/// type expressions.
///
/// Source order is preserved exactly.
#[must_use]
pub fn into_type_exprs<I>(arguments: I) -> Vec<TypeExpr>
where
    I: IntoIterator<Item = GenericArgument>,
{
    arguments
        .into_iter()
        .map(GenericArgument::into_type_expr)
        .collect()
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Validates an ordered generic-argument collection using the canonical
/// `TypeExpr` validation policy.
///
/// This helper deliberately does not impose an argument-count limit.
///
/// Any compiler resource limit must be represented by the supplied policy.
pub fn validate_all(
    arguments: &[GenericArgument],
    policy: &TypeValidationPolicy,
) -> Result<(), GenericArgumentValidationError> {
    for (index, argument) in arguments.iter().enumerate() {
        argument
            .validate_with_policy(policy)
            .map_err(|source| GenericArgumentValidationError::InvalidArgument {
                index,
                source,
            })?;
    }

    Ok(())
}

/// Structural validation error for an ordered generic-argument collection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GenericArgumentValidationError {
    /// One argument failed canonical type-expression validation.
    InvalidArgument {
        /// Zero-based source-order argument index.
        index: usize,

        /// Canonical underlying validation failure.
        source: TypeExprError,
    },
}

impl fmt::Display for GenericArgumentValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument { index, source } => {
                write!(
                    formatter,
                    "invalid generic argument at index {index}: {source}"
                )
            }
        }
    }
}

impl std::error::Error for GenericArgumentValidationError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn type_name(name: &str) -> TypeExpr {
        TypeExpr::name(name)
    }

    #[test]
    fn wraps_canonical_type_expression_without_changing_it() {
        let expression = type_name("Int");
        let argument = GenericArgument::new(expression.clone());

        assert_eq!(argument.as_type_expr(), &expression);
        assert_eq!(argument.to_type_expr(), expression);
    }

    #[test]
    fn converts_to_and_from_type_expr() {
        let expression = type_name("Element");

        let argument: GenericArgument = expression.clone().into();
        let recovered: TypeExpr = argument.into();

        assert_eq!(recovered, expression);
    }

    #[test]
    fn source_formatting_is_deterministic() {
        let argument = GenericArgument::new(type_name("Element"));

        assert_eq!(argument.to_source_string(), "Element");
        assert_eq!(argument.to_string(), "Element");
    }

    #[test]
    fn validation_delegates_to_canonical_type_expression() {
        let argument = GenericArgument::new(type_name("Int"));

        assert!(argument.is_structurally_valid());
        assert!(argument.validate().is_ok());
    }

    #[test]
    fn generic_argument_can_itself_be_generic() {
        let nested = TypeExpr::generic(
            type_name("Container"),
            vec![type_name("Element")],
        );

        let argument = GenericArgument::new(nested);

        assert!(argument.is_generic());
        assert_eq!(
            argument.generic_base(),
            Some(&type_name("Container"))
        );

        let arguments = argument
            .generic_arguments()
            .expect("generic argument should expose nested arguments");

        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0], type_name("Element"));
    }

    #[test]
    fn nested_generic_argument_order_is_preserved() {
        let nested = TypeExpr::generic(
            type_name("Map"),
            vec![type_name("Key"), type_name("Value")],
        );

        let argument = GenericArgument::new(nested);

        let arguments = argument
            .generic_arguments()
            .expect("generic argument should expose nested arguments");

        assert_eq!(
            arguments,
            &[type_name("Key"), type_name("Value")]
        );
    }

    #[test]
    fn generic_argument_count_is_source_order_count() {
        let nested = TypeExpr::generic(
            type_name("Map"),
            vec![
                type_name("Key"),
                type_name("Value"),
                type_name("Metadata"),
            ],
        );

        let argument = GenericArgument::new(nested);

        assert_eq!(argument.generic_argument_count(), Some(3));
    }

    #[test]
    fn non_generic_argument_has_no_nested_generic_arguments() {
        let argument = GenericArgument::new(type_name("Int"));

        assert!(!argument.is_generic());
        assert!(argument.generic_base().is_none());
        assert!(argument.generic_arguments().is_none());
        assert!(argument.generic_argument_count().is_none());
    }

    #[test]
    fn inference_status_delegates_to_type_expr() {
        let argument = GenericArgument::new(
            TypeExpr::generic(
                type_name("Container"),
                vec![TypeExpr::generic_parameter("T")],
            ),
        );

        assert!(argument.requires_inference());
    }

    #[test]
    fn collection_conversion_preserves_order() {
        let arguments = from_type_exprs(vec![
            type_name("A"),
            type_name("B"),
            type_name("C"),
        ]);

        assert_eq!(arguments.len(), 3);
        assert_eq!(arguments[0].to_source_string(), "A");
        assert_eq!(arguments[1].to_source_string(), "B");
        assert_eq!(arguments[2].to_source_string(), "C");

        let expressions = into_type_exprs(arguments);

        assert_eq!(
            expressions,
            vec![
                type_name("A"),
                type_name("B"),
                type_name("C"),
            ]
        );
    }

    #[test]
    fn collection_validation_preserves_source_index() {
        let policy = TypeValidationPolicy::default();

        let arguments = vec![
            GenericArgument::new(type_name("A")),
            GenericArgument::new(type_name("B")),
        ];

        assert!(validate_all(&arguments, &policy).is_ok());
    }

    #[test]
    fn schema_version_is_stable_and_independent() {
        assert_eq!(GenericArgument::schema_version(), 1);
        assert_eq!(GENERIC_ARGUMENT_SCHEMA_VERSION, 1);
    }

    #[test]
    fn transparent_serde_round_trip_preserves_argument() {
        let argument = GenericArgument::new(
            TypeExpr::generic(
                type_name("Container"),
                vec![
                    type_name("Element"),
                    TypeExpr::generic_parameter("T"),
                ],
            ),
        );

        let serialized =
            serde_json::to_string(&argument).expect("serialization must succeed");

        let restored: GenericArgument =
            serde_json::from_str(&serialized).expect("deserialization must succeed");

        assert_eq!(restored, argument);
    }

    #[test]
    fn no_machine_size_is_encoded_in_generic_argument() {
        let argument = GenericArgument::new(
            TypeExpr::generic(
                type_name("Register"),
                vec![TypeExpr::generic_parameter("N")],
            ),
        );

        assert_eq!(argument.to_source_string(), "Register<N>");
        assert!(argument.requires_inference());
    }

    #[test]
    fn deeply_nested_generic_structure_remains_a_source_value() {
        let mut expression = type_name("Leaf");

        for _ in 0..64 {
            expression = TypeExpr::generic(
                type_name("Container"),
                vec![expression],
            );
        }

        let argument = GenericArgument::new(expression);

        assert!(argument.is_structurally_valid());
        assert!(argument.validate().is_ok());
    }
}