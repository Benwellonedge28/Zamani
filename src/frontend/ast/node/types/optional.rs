//! Zamani Native AST — Optional Type
//!
//! Production-ready typed façade for the canonical source-level
//! [`TypeExpr::Optional`] representation.
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
//! TypeExpr::Optional(Box<TypeExpr>)
//!     │
//!     ├── OptionalType (this façade)
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic optional/nullability model
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
//! `OptionalType` is a strongly typed façade over the canonical
//! [`TypeExpr::Optional`] representation.
//!
//! It does **not** introduce a second independent optional-type AST.
//!
//! The authoritative source-level representation remains:
//!
//! ```text
//! TypeExpr::Optional(Box<TypeExpr>)
//! ```
//!
//! This module exists to provide an invariant-preserving, ergonomic API for
//! compiler components that specifically need to inspect or construct an
//! optional type.
//!
//! # Source-level meaning
//!
//! An optional type expresses that a value of another source-level type may be
//! absent according to Zamani's language semantics.
//!
//! Conceptually:
//!
//! ```text
//! ?T
//! ```
//!
//! The exact surface spelling is owned by the canonical `TypeExpr` formatter
//! and parser. This façade deliberately does not establish a competing syntax.
//!
//! Optionality is a source-language concept. It does not imply any particular
//! representation such as:
//!
//! - null pointers;
//! - tagged machine values;
//! - sentinel values;
//! - nullable references;
//! - heap allocation;
//! - CPU registers;
//! - GPU memory;
//! - QPU storage;
//! - hardware addresses;
//! - ABI-specific layouts.
//!
//! Representation is determined by semantic and downstream compilation
//! stages.
//!
//! # POCO-REAF
//!
//! `OptionalType` contains no machine-size-dependent information.
//!
//! It does not contain:
//!
//! - machine widths;
//! - pointer widths;
//! - fixed capacities;
//! - fixed register counts;
//! - fixed qubit counts;
//! - hardware addresses;
//! - topology information;
//! - vendor identifiers;
//! - backend identifiers;
//! - instruction-set information;
//! - target-specific layouts.
//!
//! Therefore an optional value may contain a source-level type representing:
//!
//! - a small classical value;
//! - an arbitrarily structured user type;
//! - a symbolic resource;
//! - a quantum resource abstraction;
//! - a distributed value;
//! - an accelerator value;
//! - an HDL-level abstraction;
//! - a future computational-domain type.
//!
//! The eventual realization is a downstream concern.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_OPTIONAL_DEPTH
//! MAX_OPTIONAL_SIZE
//! MAX_QUBITS
//! MAX_MACHINE_SIZE
//! ```
//!
//! in this file.
//!
//! Compiler safety/resource limits are supplied explicitly through
//! [`TypeValidationPolicy`] rather than being hidden in the AST representation.
//!
//! # Canonical representation
//!
//! The single authoritative representation is:
//!
//! ```rust
//! TypeExpr::Optional(Box<TypeExpr>)
//! ```
//!
//! `OptionalType` stores that representation directly.
//!
//! It does **not** create an independent structure such as:
//!
//! ```text
//! struct OptionalType {
//!     inner: TypeExpr
//! }
//! ```
//!
//! with a separate serialized schema.
//!
//! This avoids representation drift between the canonical type-expression
//! implementation and the typed façade.
//!
//! # Domain neutrality
//!
//! This module deliberately knows nothing about:
//!
//! - quantum hardware;
//! - physical qubits;
//! - logical-qubit allocation;
//! - QPU topology;
//! - gate sets;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - QEC;
//! - noise models;
//! - resilience;
//! - CPUs;
//! - GPUs;
//! - FPGAs;
//! - ASICs;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - ZUIR implementation details;
//! - backend instructions.
//!
//! A source-level optional type may wrap a quantum/resource type if Zamani's
//! language semantics permit that combination, but this file does not
//! interpret the wrapped type.
//!
//! # Semantic boundary
//!
//! This module represents syntax/source intent only.
//!
//! It does not perform:
//!
//! - name resolution;
//! - type inference;
//! - type unification;
//! - generic substitution;
//! - trait resolution;
//! - overload resolution;
//! - ownership checking;
//! - borrow checking;
//! - nullability-flow analysis;
//! - resource allocation;
//! - quantum legality checking;
//! - topology checking;
//! - routing;
//! - scheduling;
//! - hardware selection;
//! - backend lowering.
//!
//! Those operations belong to later compiler phases.
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser should construct the canonical representation:
//!
//! ```text
//! parser
//!     → TypeExpr::optional(inner)
//! ```
//!
//! The parser may subsequently obtain an `OptionalType` façade where
//! optional-specific inspection is useful.
//!
//! Parser recovery may construct an incomplete nested `TypeExpr`; complete
//! structural validation occurs through the canonical validation API.
//!
//! ## TypeExpr
//!
//! `OptionalType` is a typed view of `TypeExpr::Optional`.
//!
//! `OptionalType::into_type_expr()` returns the canonical representation.
//!
//! `OptionalType::try_from_type_expr()` accepts only
//! `TypeExpr::Optional(...)`.
//!
//! ## Structural validation
//!
//! Validation delegates entirely to `TypeExpr::validate()` or
//! `TypeExpr::validate_with_policy()`.
//!
//! This ensures that optional types do not acquire a second, divergent
//! structural-validation implementation.
//!
//! ## Semantic analysis
//!
//! Semantic analysis consumes the canonical `TypeExpr::Optional` and resolves
//! the wrapped type into the semantic type model.
//!
//! Semantic analysis determines language-specific rules such as:
//!
//! - whether nested optional types are legal;
//! - whether a particular type may be optional;
//! - absence/nullability semantics;
//! - implicit conversions;
//! - pattern matching behavior;
//! - control-flow narrowing;
//! - interaction with ownership/resources.
//!
//! Those decisions must not be embedded in this AST façade.
//!
//! ## ZUIR
//!
//! This module does not depend on ZUIR.
//!
//! The intended pipeline is:
//!
//! ```text
//! OptionalType
//!     │
//!     ▼
//! TypeExpr::Optional
//!     │
//!     ▼
//! semantic optional type
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! ZUIR and later stages determine the appropriate representation.
//!
//! ## Quantum integration
//!
//! Optionality may eventually appear around a source-level quantum/resource
//! abstraction, for example conceptually:
//!
//! ```text
//! Quantum<Q>?
//! ```
//!
//! This file does not decide whether such a type is semantically legal.
//!
//! It does not interpret optionality as physical qubit absence, qubit
//! deallocation, measurement state, hardware reset, or resource release.
//!
//! Those meanings belong to the semantic/domain layers.
//!
//! ## Serialization
//!
//! Serialization uses the canonical `TypeExpr` representation.
//!
//! `OptionalType` therefore has no alternate wire format.
//!
//! Serialization is deterministic because the wrapped representation contains
//! one ordered child and no unordered collections.
//!
//! Deserialization additionally verifies the wrapper invariant. A serialized
//! value representing a different `TypeExpr` variant is rejected instead of
//! producing an invalid `OptionalType`.
//!
//! # Invariants
//!
//! A successfully constructed or deserialized `OptionalType` always contains:
//!
//! ```text
//! TypeExpr::Optional(_)
//! ```
//!
//! The wrapped inner type is exactly the child stored by the canonical
//! `TypeExpr`.
//!
//! No semantic resolution is stored in the wrapper.
//!
//! # Error model
//!
//! Construction from a known inner type cannot fail because the constructor
//! itself establishes the `Optional` variant.
//!
//! Conversion from arbitrary `TypeExpr` is fallible.
//!
//! Validation failures are represented by the canonical `TypeExprError`.
//!
//! Serialization/deserialization failures are delegated to Serde, with
//! deserialization explicitly rejecting a non-optional canonical expression.
//!
//! Public APIs avoid unchecked indexing and do not intentionally panic on
//! untrusted external input.
//!
//! # Scalability
//!
//! Optionality adds exactly one source-level type-expression node around its
//! child.
//!
//! The wrapper contains no representation proportional to the number of
//! possible runtime values.
//!
//! Nested types therefore scale according to the AST itself:
//!
//! ```text
//! T
//! ?T
//! ??T
//! ???T
//! ...
//! ```
//!
//! No language-level maximum optional nesting depth is introduced here.
//!
//! The canonical `TypeExpr` validation implementation uses iterative
//! traversal, so structural validation does not add a recursive Rust call
//! stack proportional to optional nesting.
//!
//! Any compiler resource limit must be supplied through an explicit
//! `TypeValidationPolicy`.
//!
//! # Determinism
//!
//! `OptionalType` contains exactly one canonical child.
//!
//! It has no unordered collections, generated identifiers, memory addresses,
//! machine-dependent state or target-dependent metadata.
//!
//! Equality, hashing, debugging and serialization are consequently
//! deterministic for equal source-level structures.
//!
//! # Security
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no network access;
//! - performs no filesystem access;
//! - performs no execution;
//! - performs no raw pointer operations;
//! - contains no `unsafe` code;
//! - does not perform pointer arithmetic;
//! - does not trust deserialized data without checking its variant;
//! - delegates nested structural validation to the canonical AST validator.
//!
//! The AST is treated as untrusted compiler input.
//!
//! # Performance
//!
//! Construction is constant-time apart from allocation required by the
//! canonical boxed child.
//!
//! Accessors borrow the existing child and do not clone the subtree.
//!
//! Conversion into `TypeExpr` moves the existing representation.
//!
//! Validation complexity is delegated to the canonical `TypeExpr` validator.
//!
//! No hash maps or repeated scans are introduced by this façade.
//!
//! # Dependency contract
//!
//! Allowed dependencies:
//!
//! - Rust standard library;
//! - `serde`;
//! - `super::type_expr`.
//!
//! Forbidden dependencies:
//!
//! - `crate::semantic`;
//! - compiler backends;
//! - quantum hardware;
//! - quantum scheduling;
//! - quantum routing;
//! - QEC;
//! - ZQN;
//! - runtime execution;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - ZUIR implementation modules;
//! - vendor SDKs.
//!
//! # Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Independent-file completion contract
//!
//! This file is complete when:
//!
//! - `OptionalType` has exactly one canonical representation;
//! - construction preserves the inner source type;
//! - arbitrary `TypeExpr` conversion is checked;
//! - deserialization enforces the optional invariant;
//! - structural validation delegates to `TypeExpr`;
//! - source formatting delegates to `TypeExpr`;
//! - inference information delegates to `TypeExpr`;
//! - equality/hash/debug behavior is deterministic;
//! - no machine-specific information exists;
//! - no hidden scalability limit exists;
//! - no unsafe code exists;
//! - all public operations have tests;
//! - parser integration is explicitly defined;
//! - semantic integration is explicitly defined;
//! - ZUIR integration is explicitly defined;
//! - quantum integration remains downstream and target-neutral.
//!
//! # Migration
//!
//! Existing code that directly constructs:
//!
//! ```rust
//! TypeExpr::Optional(Box::new(inner))
//! ```
//!
//! remains the canonical representation.
//!
//! Introducing `OptionalType` does not require unrelated AST components to
//! migrate immediately.
//!
//! The façade can be adopted incrementally at type-specific API boundaries.
//!
//! ```text
//! Existing parser / semantic code
//!         │
//!         ▼
//! TypeExpr::Optional
//!         │
//!         ├── existing consumers remain valid
//!         │
//!         └── OptionalType façade for focused consumers
//! ```
//!
//! There is therefore no second source of truth.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize};

use super::type_expr::{TypeExpr, TypeExprError, TypeValidationPolicy};

/// Strongly typed façade for [`TypeExpr::Optional`].
///
/// `OptionalType` does not replace [`TypeExpr`]. The canonical representation
/// remains `TypeExpr::Optional(Box<TypeExpr>)`.
///
/// # Invariant
///
/// A successfully constructed or deserialized value always contains:
///
/// ```text
/// TypeExpr::Optional(_)
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct OptionalType(TypeExpr);

impl OptionalType {
    /// Constructs an optional type from an inner source-level type.
    ///
    /// The constructor establishes the representation invariant immediately.
    ///
    /// No semantic validation is performed because the inner type may contain
    /// unresolved names, generic parameters, inference placeholders,
    /// dependent types or future extension types.
    #[must_use]
    pub fn new(inner: TypeExpr) -> Self {
        Self(TypeExpr::optional(inner))
    }

    /// Constructs and structurally validates an optional type using the
    /// default compiler validation policy.
    ///
    /// The default policy imposes no artificial optional-type depth or size
    /// limit.
    pub fn try_new(inner: TypeExpr) -> Result<Self, TypeExprError> {
        Self::try_new_with_policy(inner, &TypeValidationPolicy::default())
    }

    /// Constructs and structurally validates an optional type using an
    /// explicit compiler resource policy.
    ///
    /// The policy belongs to the compiler invocation rather than the language
    /// representation.
    pub fn try_new_with_policy(
        inner: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, TypeExprError> {
        inner.validate_with_policy(policy)?;
        Ok(Self::new(inner))
    }

    /// Attempts to create an optional façade from an existing type expression.
    ///
    /// Returns [`OptionalTypeError::NotOptional`] when the supplied expression
    /// is another type-expression variant.
    pub fn try_from_type_expr(value: TypeExpr) -> Result<Self, OptionalTypeError> {
        match value {
            TypeExpr::Optional(_) => Ok(Self(value)),
            other => Err(OptionalTypeError::NotOptional {
                variant: other.variant_name(),
            }),
        }
    }

    /// Attempts to create an optional façade while also validating the entire
    /// supplied type expression using an explicit compiler policy.
    pub fn try_from_type_expr_with_policy(
        value: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, OptionalTypeError> {
        let optional = Self::try_from_type_expr(value)?;

        optional
            .validate_with_policy(policy)
            .map_err(OptionalTypeError::InvalidType)?;

        Ok(optional)
    }

    /// Returns the canonical underlying [`TypeExpr`].
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

    /// Returns the wrapped source-level inner type.
    ///
    /// The returned value is borrowed directly from the canonical AST
    /// representation; the subtree is not cloned.
    #[must_use]
    pub fn inner(&self) -> &TypeExpr {
        match &self.0 {
            TypeExpr::Optional(inner) => inner.as_ref(),

            // This branch is unreachable through the safe public API because
            // construction and deserialization both enforce the invariant.
            //
            // Keeping this explicit avoids unsafe assumptions and matches the
            // invariant-preserving typed-façade design used elsewhere in the
            // frontend AST.
            _ => unreachable!("OptionalType invariant violated"),
        }
    }

    /// Alias for [`Self::inner`] intended for generic typed-type façade APIs.
    #[must_use]
    pub fn inner_type(&self) -> &TypeExpr {
        self.inner()
    }

    /// Returns whether this value is structurally an optional type.
    ///
    /// Always `true` for a valid `OptionalType`.
    #[must_use]
    pub const fn is_optional(&self) -> bool {
        true
    }

    /// Returns whether the wrapped type requires semantic inference.
    ///
    /// This delegates to the canonical `TypeExpr` implementation.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.inner().requires_inference()
    }

    /// Performs structural validation using the default compiler policy.
    ///
    /// This does not perform semantic nullability or type checking.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.0.validate()
    }

    /// Performs structural validation using an explicit compiler policy.
    ///
    /// The policy may impose resource/security limits for a particular
    /// compilation invocation without becoming part of Zamani's language
    /// semantics.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Returns the deterministic canonical source-level representation.
    ///
    /// Formatting is delegated to `TypeExpr`, preventing a second optional-type
    /// formatter from drifting away from the canonical AST syntax.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.0.to_source_string()
    }

    /// Returns the canonical stable `TypeExpr` variant identifier.
    #[must_use]
    pub fn variant_name(&self) -> &'static str {
        self.0.variant_name()
    }

    /// Returns the canonical type expression by reference.
    ///
    /// This is an alias of [`Self::as_type_expr`] for typed-façade
    /// interoperability.
    #[must_use]
    pub fn to_type_expr(&self) -> &TypeExpr {
        self.as_type_expr()
    }

    /// Returns a new optional type with the supplied inner type.
    ///
    /// This is a structural AST operation. It intentionally does not perform
    /// semantic compatibility checking between the old and new inner types.
    #[must_use]
    pub fn with_inner(&self, inner: TypeExpr) -> Self {
        Self::new(inner)
    }
}

impl From<OptionalType> for TypeExpr {
    fn from(value: OptionalType) -> Self {
        value.into_type_expr()
    }
}

impl TryFrom<TypeExpr> for OptionalType {
    type Error = OptionalTypeError;

    fn try_from(value: TypeExpr) -> Result<Self, Self::Error> {
        Self::try_from_type_expr(value)
    }
}

impl AsRef<TypeExpr> for OptionalType {
    fn as_ref(&self) -> &TypeExpr {
        self.as_type_expr()
    }
}

impl fmt::Display for OptionalType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

/// Errors produced when constructing or validating an [`OptionalType`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OptionalTypeError {
    /// The supplied type expression was not an optional type.
    NotOptional {
        /// Canonical name of the supplied `TypeExpr` variant.
        variant: &'static str,
    },

    /// The supplied type expression failed canonical structural validation.
    InvalidType(TypeExprError),
}

impl fmt::Display for OptionalTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotOptional { variant } => {
                write!(
                    formatter,
                    "expected TypeExpr::Optional, found TypeExpr::{variant}"
                )
            }

            Self::InvalidType(error) => {
                write!(formatter, "invalid optional inner type: {error}")
            }
        }
    }
}

impl std::error::Error for OptionalTypeError {}

impl From<TypeExprError> for OptionalTypeError {
    fn from(value: TypeExprError) -> Self {
        Self::InvalidType(value)
    }
}

/// Enforces the `OptionalType` representation invariant during deserialization.
///
/// A transparent derived deserializer alone would accept any `TypeExpr`.
/// This implementation deliberately converts through the canonical
/// `TypeExpr` and then checks that the value is actually
/// `TypeExpr::Optional`.
impl<'de> Deserialize<'de> for OptionalType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = TypeExpr::deserialize(deserializer)?;

        Self::try_from_type_expr(value).map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(name: &str) -> TypeExpr {
        TypeExpr::name(name)
    }

    #[test]
    fn construction_preserves_inner_type() {
        let optional = OptionalType::new(named("Value"));

        assert_eq!(optional.inner(), &named("Value"));
        assert_eq!(optional.as_type_expr(), &TypeExpr::optional(named("Value")));
    }

    #[test]
    fn constructor_establishes_optional_variant() {
        let optional = OptionalType::new(named("Value"));

        assert!(optional.is_optional());
        assert_eq!(optional.variant_name(), "optional");
    }

    #[test]
    fn conversion_accepts_only_optional_type() {
        let expression = TypeExpr::optional(named("Value"));

        let optional =
            OptionalType::try_from_type_expr(expression).expect("optional must be accepted");

        assert_eq!(optional.inner(), &named("Value"));
    }

    #[test]
    fn conversion_rejects_non_optional_type() {
        let expression = named("Value");

        let error = OptionalType::try_from_type_expr(expression)
            .expect_err("non-optional type must be rejected");

        assert_eq!(
            error,
            OptionalTypeError::NotOptional {
                variant: "identifier",
            }
        );
    }

    #[test]
    fn from_trait_uses_checked_conversion() {
        let optional =
            OptionalType::try_from(TypeExpr::optional(named("Value")))
                .expect("conversion must succeed");

        assert_eq!(optional.inner(), &named("Value"));
    }

    #[test]
    fn into_type_expr_round_trips() {
        let original = TypeExpr::optional(TypeExpr::tuple(vec![
            named("A"),
            named("B"),
        ]));

        let optional =
            OptionalType::try_from_type_expr(original.clone())
                .expect("optional conversion must succeed");

        assert_eq!(optional.into_type_expr(), original);
    }

    #[test]
    fn source_formatting_is_delegated_to_type_expr() {
        let optional = OptionalType::new(named("Value"));

        assert_eq!(optional.to_source_string(), "?Value");
        assert_eq!(optional.to_string(), "?Value");
    }

    #[test]
    fn nested_optionals_are_supported_without_fixed_depth() {
        let mut ty = named("Value");

        for _ in 0..4096 {
            ty = TypeExpr::optional(ty);
        }

        let optional =
            OptionalType::try_from_type_expr(ty).expect("nested optional must be accepted");

        assert!(optional.validate().is_ok());
    }

    #[test]
    fn nested_optionals_require_inference_when_inner_type_does() {
        let optional = OptionalType::new(TypeExpr::optional(TypeExpr::Infer));

        assert!(optional.requires_inference());
    }

    #[test]
    fn concrete_optional_does_not_require_inference() {
        let optional = OptionalType::new(named("Value"));

        assert!(!optional.requires_inference());
    }

    #[test]
    fn validation_delegates_to_canonical_type_expr() {
        let optional = OptionalType::new(named("Value"));

        assert!(optional.validate().is_ok());
    }

    #[test]
    fn explicit_validation_policy_is_supported() {
        let optional = OptionalType::new(named("Value"));

        let policy = TypeValidationPolicy {
            max_collection_items: None,
            max_identifier_bytes: Some(64),
            max_extension_namespace_bytes: None,
            max_extension_name_bytes: None,
            max_attribute_value_bytes: None,
        };

        assert!(optional.validate_with_policy(&policy).is_ok());
    }

    #[test]
    fn validation_policy_is_not_embedded_in_type() {
        let optional = OptionalType::new(named("Value"));

        let unrestricted = TypeValidationPolicy::default();

        assert!(optional.validate_with_policy(&unrestricted).is_ok());
    }

    #[test]
    fn with_inner_replaces_only_the_source_level_child() {
        let optional = OptionalType::new(named("Old"));

        let replaced = optional.with_inner(named("New"));

        assert_eq!(replaced.inner(), &named("New"));
        assert_eq!(optional.inner(), &named("Old"));
    }

    #[test]
    fn as_ref_exposes_canonical_type_expr() {
        let optional = OptionalType::new(named("Value"));

        let expression: &TypeExpr = optional.as_ref();

        assert_eq!(expression, &TypeExpr::optional(named("Value")));
    }

    #[test]
    fn display_matches_canonical_source_format() {
        let optional = OptionalType::new(TypeExpr::tuple(vec![
            named("A"),
            named("B"),
        ]));

        assert_eq!(format!("{optional}"), "?(A, B)");
    }

    #[test]
    fn equality_and_hash_are_structural() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let left = OptionalType::new(named("Value"));
        let right = OptionalType::new(named("Value"));

        assert_eq!(left, right);

        let mut left_hasher = DefaultHasher::new();
        left.hash(&mut left_hasher);

        let mut right_hasher = DefaultHasher::new();
        right.hash(&mut right_hasher);

        assert_eq!(left_hasher.finish(), right_hasher.finish());
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let optional = OptionalType::new(TypeExpr::optional(TypeExpr::tuple(vec![
            named("Left"),
            named("Right"),
        ])));

        let encoded =
            serde_json::to_string(&optional).expect("optional serialization must succeed");

        let decoded: OptionalType =
            serde_json::from_str(&encoded).expect("optional deserialization must succeed");

        assert_eq!(decoded, optional);
        assert_eq!(decoded.to_source_string(), "??(Left, Right)");
    }

    #[test]
    fn deserialization_rejects_non_optional_type_expr() {
        let encoded =
            serde_json::to_string(&TypeExpr::name("NotOptional"))
                .expect("type expression serialization must succeed");

        let result = serde_json::from_str::<OptionalType>(&encoded);

        assert!(result.is_err());
    }

    #[test]
    fn serialization_is_deterministic() {
        let optional = OptionalType::new(TypeExpr::optional(named("Value")));

        let first =
            serde_json::to_string(&optional).expect("first serialization must succeed");

        let second =
            serde_json::to_string(&optional).expect("second serialization must succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn arbitrary_nested_source_types_are_supported() {
        let inner = TypeExpr::generic(
            TypeExpr::name("Container"),
            vec![
                TypeExpr::array(named("Element"), None),
                TypeExpr::slice(TypeExpr::optional(named("Nested"))),
                TypeExpr::quantum(TypeExpr::name("State")),
            ],
        );

        let optional = OptionalType::new(inner);

        assert!(optional.validate().is_ok());
        assert!(optional.to_source_string().starts_with("?"));
    }

    #[test]
    fn generic_and_quantum_types_remain_domain_neutral() {
        let quantum = TypeExpr::quantum(TypeExpr::name("Q"));

        let optional = OptionalType::new(quantum.clone());

        assert_eq!(optional.inner(), &quantum);
        assert!(optional.validate().is_ok());
    }

    #[test]
    fn try_new_with_policy_validates_before_returning() {
        let inner = TypeExpr::name("Value");

        let policy = TypeValidationPolicy {
            max_collection_items: None,
            max_identifier_bytes: Some(5),
            max_extension_namespace_bytes: None,
            max_extension_name_bytes: None,
            max_attribute_value_bytes: None,
        };

        let result = OptionalType::try_new_with_policy(inner, &policy);

        assert!(result.is_err());
    }

    #[test]
    fn try_from_type_expr_with_policy_checks_validation() {
        let value = TypeExpr::optional(TypeExpr::name("Value"));

        let policy = TypeValidationPolicy {
            max_collection_items: None,
            max_identifier_bytes: Some(5),
            max_extension_namespace_bytes: None,
            max_extension_name_bytes: None,
            max_attribute_value_bytes: None,
        };

        let result =
            OptionalType::try_from_type_expr_with_policy(value, &policy);

        assert!(result.is_ok());
    }

    #[test]
    fn optional_type_does_not_change_inner_type_identity() {
        let inner = TypeExpr::GenericParameter(
            super::super::type_expr::TypeParameterName::from("T"),
        );

        let optional = OptionalType::new(inner.clone());

        assert_eq!(optional.inner(), &inner);
    }
}