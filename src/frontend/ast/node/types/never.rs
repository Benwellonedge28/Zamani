//! Zamani Frontend AST — Never Type
//!
//! Production-ready typed façade for the canonical source-level
//! [`TypeExpr::Never`] representation.
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
//! TypeExpr::Never
//!     │
//!     ├── NeverType  ← this façade
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic never/bottom type
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
//! `NeverType` is a strongly typed façade over the canonical
//! [`TypeExpr::Never`] representation.
//!
//! It does **not** introduce a second independent AST representation.
//!
//! The authoritative source-level representation remains:
//!
//! ```text
//! TypeExpr::Never
//! ```
//!
//! The façade exists so compiler components that specifically require the
//! never/bottom type can express that requirement through a type-safe API
//! without duplicating the AST representation.
//!
//! # Source-level meaning
//!
//! The never type represents a computation/value type for which normal value
//! production does not occur according to Zamani's language semantics.
//!
//! Its canonical source spelling in `TypeExpr` is:
//!
//! ```text
//! !
//! ```
//!
//! The exact semantic consequences of `!` belong to semantic analysis.
//!
//! This file does not decide whether `Never` represents:
//!
//! - a diverging function;
//! - unreachable control flow;
//! - a non-returning operation;
//! - an impossible value;
//! - bottom in a type lattice;
//! - an uninhabited type;
//! - a control-flow terminator;
//! - a compiler optimization fact.
//!
//! Those meanings are semantic-layer responsibilities.
//!
//! # Canonical representation
//!
//! `type_expr.rs` owns the canonical representation:
//!
//! ```rust
//! TypeExpr::Never
//! ```
//!
//! `NeverType` stores that representation directly.
//!
//! It must never introduce an independent structure containing duplicated
//! semantic state.
//!
//! This prevents representation drift between the canonical type-expression
//! implementation and the specialized façade.
//!
//! # Architectural separation
//!
//! ```text
//! NeverType
//!     │
//!     ▼
//! TypeExpr::Never
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic Never/bottom representation
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! The following must remain outside this file:
//!
//! - type inference;
//! - type unification;
//! - control-flow analysis;
//! - exhaustiveness analysis;
//! - divergence analysis;
//! - dead-code analysis;
//! - optimization;
//! - backend lowering;
//! - hardware mapping;
//! - quantum scheduling;
//! - quantum routing;
//! - QEC;
//! - resilience;
//! - runtime execution.
//!
//! # POCO-REAF
//!
//! `NeverType` contains no machine-size information.
//!
//! It has:
//!
//! - no machine width;
//! - no register width;
//! - no memory size;
//! - no qubit count;
//! - no topology;
//! - no backend identifier;
//! - no vendor identifier;
//! - no instruction-set information;
//! - no runtime address;
//! - no fixed resource capacity.
//!
//! Therefore the representation is inherently independent of computational
//! scale.
//!
//! A program may contain zero, one, or arbitrarily many never-typed
//! expressions/functions/paths as allowed by the source language and available
//! compiler resources.
//!
//! No language-level maximum is introduced here.
//!
//! # Quantum and heterogeneous-computing neutrality
//!
//! `NeverType` is not quantum-specific.
//!
//! It may participate in source programs that eventually lower to:
//!
//! - classical execution;
//! - quantum execution;
//! - hybrid execution;
//! - distributed execution;
//! - accelerator execution;
//! - HDL-related computation;
//! - AI/ML computation;
//! - future computational domains.
//!
//! This module does not interpret `Never` as:
//!
//! - a failed quantum operation;
//! - a missing qubit;
//! - a released resource;
//! - a hardware fault;
//! - a backend rejection;
//! - a scheduler cancellation;
//! - a QEC failure.
//!
//! Such interpretations, if they exist, belong to later semantic/domain
//! layers.
//!
//! # Parser integration
//!
//! The parser owns recognition of the source syntax.
//!
//! When the Zamani grammar recognizes the never type, the canonical parser
//! representation must be:
//!
//! ```text
//! parser → TypeExpr::Never
//! ```
//!
//! The parser does not need `NeverType` to recognize the syntax.
//!
//! `NeverType` is available as a typed façade for code that already knows that
//! the resulting expression is the never type.
//!
//! Parser recovery and syntax diagnostics remain parser responsibilities.
//!
//! # TypeExpr integration
//!
//! `NeverType` is a view of [`TypeExpr::Never`].
//!
//! Conversion in the other direction always returns the canonical variant:
//!
//! ```text
//! NeverType → TypeExpr::Never
//! ```
//!
//! There is therefore exactly one authoritative AST representation.
//!
//! # Structural validation
//!
//! `NeverType` contains no child data and consequently has no nested structure
//! of its own to validate.
//!
//! Structural validation delegates to the canonical `TypeExpr` validator.
//!
//! This is important because validation must have one authoritative
//! implementation.
//!
//! `NeverType` must not introduce a second validation policy that can diverge
//! from `TypeExpr`.
//!
//! # Semantic integration
//!
//! Semantic analysis consumes the canonical `TypeExpr::Never` representation
//! and determines its language-level meaning.
//!
//! Semantic analysis may use `NeverType` when a specialized never-type API is
//! useful.
//!
//! Semantic analysis is responsible for decisions such as:
//!
//! - whether a function may return `Never`;
//! - whether an expression of `Never` can coerce into another type;
//! - whether control flow terminates;
//! - whether pattern branches are unreachable;
//! - how `Never` participates in type inference;
//! - how `Never` participates in type compatibility.
//!
//! None of those decisions belong to this file.
//!
//! # ZUIR integration
//!
//! This module does not depend on ZUIR.
//!
//! The intended lowering boundary is:
//!
//! ```text
//! NeverType
//!     │
//!     ▼
//! TypeExpr::Never
//!     │
//!     ▼
//! semantic never/bottom type
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! If ZUIR represents non-returning control flow using a terminator or another
//! representation, that is determined by the semantic/ZUIR layers.
//!
//! The AST must not prematurely encode that implementation.
//!
//! # Quantum integration
//!
//! `NeverType` requires no quantum-specific integration.
//!
//! A quantum function may semantically return `Never`, but the AST does not
//! need to know whether the function executes on:
//!
//! - one quantum resource;
//! - many quantum resources;
//! - logical resources;
//! - physical resources;
//! - a simulator;
//! - a distributed quantum system;
//! - a future quantum architecture.
//!
//! Consequently `NeverType` does not need modification when a new quantum
//! technology or backend is introduced.
//!
//! # Serialization
//!
//! Serialization is inherited from the canonical [`TypeExpr`] representation.
//!
//! The canonical representation is:
//!
//! ```text
//! TypeExpr::Never
//! ```
//!
//! This façade does not establish an alternative serialized schema.
//!
//! Because `NeverType` contains no payload, its serialized representation has
//! no ordering-sensitive child collection and no machine-dependent data.
//!
//! Deserialization must verify the representation invariant:
//!
//! ```text
//! serialized value → TypeExpr::Never
//! ```
//!
//! A serialized `TypeExpr` representing another variant must be rejected when
//! converting it into `NeverType`.
//!
//! # Determinism
//!
//! `NeverType` contains no:
//!
//! - hash maps;
//! - generated identifiers;
//! - timestamps;
//! - memory addresses;
//! - random values;
//! - target information;
//! - mutable global state.
//!
//! Equality, hashing, debugging and canonical source formatting are therefore
//! deterministic.
//!
//! # Scalability
//!
//! `NeverType` has constant structural size:
//!
//! ```text
//! NeverType
//!     └── TypeExpr::Never
//! ```
//!
//! There is no collection whose size depends on:
//!
//! - program size;
//! - machine size;
//! - qubit count;
//! - register count;
//! - number of processors;
//! - backend capacity.
//!
//! Consequently this file introduces no artificial scalability ceiling.
//!
//! Compiler-wide safety limits remain explicit compiler-policy concerns.
//!
//! # Security
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no filesystem access;
//! - performs no network access;
//! - executes no source code;
//! - performs no raw pointer operations;
//! - contains no `unsafe` code;
//! - performs no pointer arithmetic;
//! - does not expose mutable global state.
//!
//! Conversion from arbitrary `TypeExpr` is fallible so an incorrect variant
//! cannot silently become a `NeverType`.
//!
//! # Performance
//!
//! Construction is constant time.
//!
//! Conversion from `TypeExpr` performs one enum-variant inspection.
//!
//! Access to the underlying representation is constant time.
//!
//! No heap allocation is required by `NeverType` itself.
//!
//! # Dependency contract
//!
//! Allowed dependencies:
//!
//! - Rust standard library;
//! - `serde` where required by the canonical AST serialization contract;
//! - sibling `type_expr` module.
//!
//! Forbidden dependencies:
//!
//! - legacy `crate::ast`;
//! - parser implementation;
//! - lexer implementation;
//! - semantic implementation;
//! - compiler driver;
//! - ZUIR implementation;
//! - quantum IR;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - ZQN;
//! - runtime execution;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor SDKs.
//!
//! # Rust compatibility
//!
//! This file is designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! # Independent-file completion contract
//!
//! This file is complete when:
//!
//! - `NeverType` has exactly one canonical representation;
//! - construction always produces `TypeExpr::Never`;
//! - conversion from `TypeExpr` checks the variant;
//! - invalid variants are rejected without panic;
//! - validation delegates to `TypeExpr`;
//! - formatting delegates to `TypeExpr`;
//! - inference information delegates to `TypeExpr`;
//! - serialization has no competing schema;
//! - equality/hash/debug behavior is deterministic;
//! - no machine-specific information exists;
//! - no hidden scalability limit exists;
//! - no unsafe code exists;
//! - parser integration is explicitly defined;
//! - semantic integration is explicitly defined;
//! - ZUIR integration is explicitly defined;
//! - quantum integration remains domain-neutral.
//!
//! # Migration contract
//!
//! Existing code using:
//!
//! ```rust
//! TypeExpr::Never
//! ```
//!
//! remains valid and authoritative.
//!
//! Introducing `NeverType` does not require unrelated AST components to migrate.
//!
//! Consumers can incrementally adopt:
//!
//! ```text
//! TypeExpr::Never
//!     ↕
//! NeverType
//! ```
//!
//! without creating a second AST representation.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize};

use super::type_expr::{TypeExpr, TypeExprError, TypeValidationPolicy};

// =============================================================================
// NeverType
// =============================================================================

/// Strongly typed façade over the canonical [`TypeExpr::Never`] representation.
///
/// # Invariant
///
/// Every successfully constructed or deserialized `NeverType` contains
/// exactly [`TypeExpr::Never`].
///
/// `NeverType` has no payload because the canonical never type itself has no
/// child type or source-level parameters.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct NeverType(TypeExpr);

impl NeverType {
    /// Constructs the canonical never type.
    ///
    /// Construction cannot fail because the constructor directly establishes
    /// the representation invariant.
    #[must_use]
    pub const fn new() -> Self {
        Self(TypeExpr::Never)
    }

    /// Returns the canonical underlying [`TypeExpr`].
    ///
    /// This is the primary interoperability boundary with the rest of the
    /// frontend AST.
    #[must_use]
    pub const fn as_type_expr(&self) -> &TypeExpr {
        &self.0
    }

    /// Consumes the façade and returns the canonical [`TypeExpr`].
    #[must_use]
    pub const fn into_type_expr(self) -> TypeExpr {
        self.0
    }

    /// Returns an owned copy of the canonical [`TypeExpr`].
    ///
    /// Since `TypeExpr::Never` carries no payload, this operation is cheap.
    #[must_use]
    pub const fn to_type_expr(&self) -> TypeExpr {
        TypeExpr::Never
    }

    /// Performs canonical structural validation using the default policy.
    ///
    /// `NeverType` itself has no child structure, so this delegates to the
    /// authoritative `TypeExpr` validator.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.0.validate()
    }

    /// Performs canonical structural validation using an explicit compiler
    /// validation/resource policy.
    ///
    /// No language-level limit is introduced by this type.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Returns the canonical source-level spelling.
    ///
    /// For the current canonical `TypeExpr` representation this is `!`.
    ///
    /// Formatting remains delegated to `TypeExpr` so that this façade cannot
    /// drift from the canonical source representation.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.0.to_source_string()
    }

    /// Returns the canonical AST variant name.
    ///
    /// This is always `never` for a valid `NeverType`.
    #[must_use]
    pub const fn variant_name(&self) -> &'static str {
        "never"
    }

    /// Returns whether this value is a never type.
    ///
    /// This method is intentionally constant and does not perform semantic
    /// analysis.
    #[must_use]
    pub const fn is_never(&self) -> bool {
        true
    }

    /// Returns whether this type expression requires inference.
    ///
    /// `Never` itself carries no unresolved type parameters, so this is always
    /// false.
    ///
    /// This method reports structural information only; it does not perform
    /// type inference.
    #[must_use]
    pub const fn requires_inference(&self) -> bool {
        false
    }

    /// Attempts to create a `NeverType` from an existing canonical
    /// [`TypeExpr`].
    ///
    /// Any variant other than `TypeExpr::Never` is rejected.
    pub fn try_from_type_expr(value: TypeExpr) -> Result<Self, NeverTypeError> {
        match value {
            TypeExpr::Never => Ok(Self::new()),
            other => Err(NeverTypeError::NotNever {
                variant: other.variant_name(),
            }),
        }
    }

    /// Attempts to create a validated `NeverType` from an existing
    /// [`TypeExpr`].
    ///
    /// The supplied policy remains a compiler resource policy and does not
    /// change the language representation.
    pub fn try_from_type_expr_with_policy(
        value: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, NeverTypeError> {
        let never = Self::try_from_type_expr(value)?;

        never.validate_with_policy(policy)
            .map_err(NeverTypeError::InvalidType)?;

        Ok(never)
    }

    /// Creates a validated `NeverType` using the canonical validation path.
    ///
    /// Since the representation is intrinsic and contains no child nodes,
    /// validation is expected to succeed unless the canonical validator itself
    /// changes its global structural rules.
    pub fn try_new() -> Result<Self, TypeExprError> {
        let never = Self::new();
        never.validate()?;
        Ok(never)
    }

    /// Creates a validated `NeverType` using an explicit validation policy.
    pub fn try_new_with_policy(
        policy: &TypeValidationPolicy,
    ) -> Result<Self, TypeExprError> {
        let never = Self::new();
        never.validate_with_policy(policy)?;
        Ok(never)
    }
}

impl Default for NeverType {
    fn default() -> Self {
        Self::new()
    }
}

impl From<NeverType> for TypeExpr {
    fn from(value: NeverType) -> Self {
        value.into_type_expr()
    }
}

impl From<&NeverType> for TypeExpr {
    fn from(_value: &NeverType) -> Self {
        TypeExpr::Never
    }
}

impl TryFrom<TypeExpr> for NeverType {
    type Error = NeverTypeError;

    fn try_from(value: TypeExpr) -> Result<Self, Self::Error> {
        Self::try_from_type_expr(value)
    }
}

impl fmt::Display for NeverType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

// =============================================================================
// Deserialization
// =============================================================================

impl<'de> Deserialize<'de> for NeverType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = TypeExpr::deserialize(deserializer)?;

        Self::try_from_type_expr(value).map_err(D::Error::custom)
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Error returned when a [`TypeExpr`] cannot be represented as [`NeverType`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NeverTypeError {
    /// The supplied expression was not `TypeExpr::Never`.
    NotNever {
        /// Canonical name of the supplied type-expression variant.
        variant: &'static str,
    },

    /// The supplied canonical never expression failed structural validation.
    InvalidType(TypeExprError),
}

impl fmt::Display for NeverTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotNever { variant } => {
                write!(
                    formatter,
                    "expected TypeExpr::Never, found TypeExpr::{variant}"
                )
            }
            Self::InvalidType(error) => {
                write!(formatter, "invalid never type expression: {error}")
            }
        }
    }
}

impl std::error::Error for NeverTypeError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_constructs_canonical_never() {
        let never = NeverType::new();

        assert_eq!(never.as_type_expr(), &TypeExpr::Never);
        assert_eq!(never.variant_name(), "never");
        assert!(never.is_never());
    }

    #[test]
    fn default_constructs_canonical_never() {
        let never = NeverType::default();

        assert_eq!(never.as_type_expr(), &TypeExpr::Never);
        assert!(never.is_never());
    }

    #[test]
    fn canonical_source_spelling_is_preserved() {
        let never = NeverType::new();

        assert_eq!(never.to_source_string(), "!");
        assert_eq!(never.to_string(), "!");
    }

    #[test]
    fn canonical_type_expression_round_trips() {
        let never = NeverType::new();
        let expression = never.clone().into_type_expr();

        let restored = NeverType::try_from_type_expr(expression)
            .expect("TypeExpr::Never must convert to NeverType");

        assert_eq!(restored, never);
    }

    #[test]
    fn non_never_expression_is_rejected() {
        let expression = TypeExpr::Unit;

        let result = NeverType::try_from_type_expr(expression);

        assert!(matches!(
            result,
            Err(NeverTypeError::NotNever { variant: "unit" })
        ));
    }

    #[test]
    fn identifier_expression_is_rejected() {
        let expression = TypeExpr::name("Int");

        let result = NeverType::try_from_type_expr(expression);

        assert!(matches!(
            result,
            Err(NeverTypeError::NotNever {
                variant: "identifier"
            })
        ));
    }

    #[test]
    fn never_does_not_require_inference() {
        assert!(!NeverType::new().requires_inference());
    }

    #[test]
    fn validation_delegates_to_canonical_type_expr() {
        assert!(NeverType::new().validate().is_ok());
    }

    #[test]
    fn validation_with_policy_delegates_to_canonical_type_expr() {
        let policy = TypeValidationPolicy::default();

        assert!(NeverType::new()
            .validate_with_policy(&policy)
            .is_ok());
    }

    #[test]
    fn validated_constructor_succeeds() {
        let never = NeverType::try_new()
            .expect("canonical TypeExpr::Never should validate");

        assert_eq!(never.as_type_expr(), &TypeExpr::Never);
    }

    #[test]
    fn validated_constructor_with_policy_succeeds() {
        let policy = TypeValidationPolicy::default();

        let never = NeverType::try_new_with_policy(&policy)
            .expect("canonical TypeExpr::Never should validate");

        assert_eq!(never.as_type_expr(), &TypeExpr::Never);
    }

    #[test]
    fn conversion_from_never_type_is_canonical() {
        let never = NeverType::new();
        let expression: TypeExpr = never.into();

        assert_eq!(expression, TypeExpr::Never);
    }

    #[test]
    fn conversion_from_reference_is_canonical() {
        let never = NeverType::new();
        let expression: TypeExpr = (&never).into();

        assert_eq!(expression, TypeExpr::Never);
    }

    #[test]
    fn display_is_deterministic() {
        let never = NeverType::new();

        assert_eq!(format!("{never}"), "!");
        assert_eq!(format!("{never}"), never.to_source_string());
    }

    #[test]
    fn equality_and_hashing_are_structural() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let first = NeverType::new();
        let second = NeverType::new();

        assert_eq!(first, second);

        let mut first_hasher = DefaultHasher::new();
        first.hash(&mut first_hasher);

        let mut second_hasher = DefaultHasher::new();
        second.hash(&mut second_hasher);

        assert_eq!(first_hasher.finish(), second_hasher.finish());
    }

    #[test]
    fn serde_round_trip_preserves_canonical_value() {
        let never = NeverType::new();

        let encoded =
            serde_json::to_value(&never).expect("NeverType must serialize");

        let decoded: NeverType =
            serde_json::from_value(encoded).expect("NeverType must deserialize");

        assert_eq!(decoded, never);
        assert_eq!(decoded.as_type_expr(), &TypeExpr::Never);
    }

    #[test]
    fn serde_rejects_non_never_type() {
        let expression = TypeExpr::Unit;

        let encoded =
            serde_json::to_value(expression).expect("TypeExpr must serialize");

        let result = serde_json::from_value::<NeverType>(encoded);

        assert!(result.is_err());
    }

    #[test]
    fn never_has_no_machine_specific_state() {
        let never = NeverType::new();

        assert_eq!(never.as_type_expr(), &TypeExpr::Never);
        assert_eq!(never.variant_name(), "never");
        assert!(!never.requires_inference());
    }

    #[test]
    fn repeated_construction_is_deterministic() {
        let first = NeverType::new();
        let second = NeverType::new();

        assert_eq!(first, second);
        assert_eq!(first.to_source_string(), second.to_source_string());
        assert_eq!(first.variant_name(), second.variant_name());
    }
}