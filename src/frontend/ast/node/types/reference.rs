//! Zamani Native AST — Reference Types
//!
//! Canonical source-level representation of a borrow/reference type.
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
//! ReferenceType
//!     │
//!     ▼
//! TypeExpr::Reference
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic reference/ownership model
//!     │
//!     ▼
//! ZUIR / domain IR
//!     │
//!     ├── classical
//!     ├── quantum
//!     ├── HDL
//!     ├── accelerator
//!     ├── distributed
//!     └── future domains
//!     │
//!     ▼
//! target/backend lowering
//! ```
//!
//! # Responsibility
//!
//! This module owns the source-level representation of a reference type.
//!
//! A reference describes source-level intent to refer to another value/type
//! without owning or copying the referenced value according to the language's
//! later semantic rules.
//!
//! This module deliberately does **not**:
//!
//! - resolve lifetimes;
//! - perform borrow checking;
//! - perform ownership checking;
//! - determine aliasing legality;
//! - determine memory layout;
//! - allocate memory;
//! - assign addresses;
//! - select a CPU/GPU/QPU/FPGA/ASIC;
//! - assign physical quantum resources;
//! - perform quantum routing;
//! - perform scheduling;
//! - perform calibration;
//! - perform error correction;
//! - select a backend;
//! - lower to LLVM;
//! - lower to QIR;
//! - lower to MLIR;
//! - lower directly to ZUIR.
//!
//! Those responsibilities belong to later compiler layers.
//!
//! # Domain neutrality
//!
//! A reference is a language-level abstraction.
//!
//! The referenced type may ultimately represent:
//!
//! - a classical value;
//! - a collection;
//! - a user-defined type;
//! - a resource;
//! - a logical quantum resource;
//! - a hybrid computational value;
//! - an accelerator value;
//! - an HDL value;
//! - a future computational-domain value.
//!
//! `ReferenceType` therefore contains no hardware-specific information.
//!
//! In particular, it does not contain:
//!
//! - qubit IDs;
//! - physical addresses;
//! - device IDs;
//! - topology IDs;
//! - vendor identifiers;
//! - backend identifiers;
//! - fixed resource counts;
//! - machine-width assumptions.
//!
//! # POCO-REAF
//!
//! References must not introduce machine-size assumptions into the AST.
//!
//! There is no:
//!
//! ```text
//! MAX_REFERENCES
//! MAX_REFERENCE_DEPTH
//! MAX_QUBITS
//! MAX_REGISTER_SIZE
//! MAX_MACHINE_SIZE
//! ```
//!
//! in this module.
//!
//! A reference may therefore ultimately refer to a type whose semantic
//! representation is tiny or arbitrarily large, subject only to explicitly
//! configured compiler resource policies and actual available resources.
//!
//! # Canonical representation
//!
//! `TypeExpr` is the canonical AST type-expression representation.
//!
//! This module intentionally wraps the existing:
//!
//! ```text
//! TypeExpr::Reference {
//!     mutable,
//!     lifetime,
//!     inner,
//! }
//! ```
//!
//! rather than introducing a second reference variant.
//!
//! This gives callers a focused API while preserving one authoritative AST
//! representation.
//!
//! # Architectural distinction
//!
//! ```text
//! ReferenceType
//!     = source-level reference syntax
//!
//! TypeExpr
//!     = complete source-level type syntax
//!
//! SemanticType
//!     = resolved semantic type
//!
//! Borrow/ownership model
//!     = semantic analysis
//!
//! ZUIR
//!     = universal computational representation
//!
//! Domain IR
//!     = domain-specific implementation
//!
//! Target IR
//!     = hardware/backend representation
//! ```
//!
//! `ReferenceType` must never become a semantic borrow-checker object.
//!
//! # Lifetime neutrality
//!
//! `LifetimeName` is retained exactly as source-level information.
//!
//! This file does not determine:
//!
//! - lifetime regions;
//! - lifetime relationships;
//! - lifetime elision;
//! - lifetime inference;
//! - lifetime validity;
//! - ownership validity.
//!
//! Those are semantic-analysis concerns.
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
//! - no `unsafe`.
//!
//! # Dependencies
//!
//! This file intentionally depends only on the canonical local type-expression
//! module and the Rust standard library types required by its API.
//!
//! It must not depend on:
//!
//! - semantic analysis;
//! - compiler backends;
//! - quantum hardware;
//! - quantum scheduling;
//! - routing;
//! - QEC;
//! - ZQN;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - runtime execution.
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser may construct `ReferenceType` when it recognizes reference-type
//! syntax, then convert it to `TypeExpr`.
//!
//! ## TypeExpr
//!
//! `ReferenceType::into_type_expr()` produces the canonical
//! `TypeExpr::Reference` representation.
//!
//! ## Structural validation
//!
//! Validation delegates to the canonical `TypeExpr::validate()` machinery.
//! This prevents two competing validation implementations from evolving.
//!
//! ## Semantic analysis
//!
//! Semantic analysis consumes `TypeExpr::Reference` and resolves:
//!
//! - the referenced type;
//! - lifetime semantics;
//! - ownership/borrowing rules;
//! - mutability rules;
//! - aliasing rules;
//! - resource semantics.
//!
//! ## ZUIR
//!
//! ZUIR must consume the semantic result rather than depending on this source
//! AST wrapper for backend decisions.
//!
//! ## Quantum compilation
//!
//! A reference to a quantum/resource type remains a source-level reference.
//! Physical resource assignment, routing, scheduling and execution are all
//! downstream concerns.
//!
//! # Independent-file completion contract
//!
//! This file is complete when:
//!
//! - `ReferenceType` has one canonical representation;
//! - construction preserves all source-level information;
//! - accessors expose all owned information without exposing implementation
//!   internals;
//! - conversion to/from `TypeExpr` is deterministic;
//! - structural validation delegates to `TypeExpr`;
//! - equality/hash/debug semantics are deterministic;
//! - serialization compatibility is inherited from `TypeExpr`;
//! - malformed structures return errors rather than panic;
//! - no unsafe code exists;
//! - no hardware/domain dependency exists;
//! - no machine-size assumption exists;
//! - tests cover every public operation;
//! - downstream contracts are explicitly defined.
//!
//! # Forbidden dependencies
//!
//! This file must never depend on:
//!
//! ```text
//! crate::quantum::hardware
//! crate::quantum::scheduling
//! crate::quantum::routing
//! crate::quantum::error_correction
//! crate::quantum::zqn
//! crate::backend
//! crate::runtime
//! crate::semantic
//! crate::zuir
//! LLVM
//! QIR
//! MLIR
//! vendor SDKs
//! ```
//!
//! `ReferenceType` belongs entirely to the source AST layer.
//!
//! # Safety
//!
//! This module performs no raw memory operations, pointer arithmetic, FFI,
//! I/O, execution or unsafe operations.
//!
//! The crate-level `forbid(unsafe_code)` policy is reinforced locally so this
//! file cannot accidentally introduce unsafe implementation details.
//!
//! # Scalability
//!
//! Recursive type structure uses the already canonical `Box<TypeExpr>`
//! representation. This is required for Rust's recursive enum layout and does
//! not establish a language-level maximum reference depth.
//!
//! Any resource limit needed to protect a compiler invocation must be supplied
//! by the compiler's explicit validation/resource policy rather than hidden in
//! this type.
//!
//! # Determinism
//!
//! The wrapper contains no unordered collections and introduces no generated
//! identifiers, addresses or machine-dependent state.
//!
//! Equal source-level references therefore have deterministic equality,
//! hashing, debugging and serialization behavior through their canonical
//! `TypeExpr` representation.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::type_expr::{LifetimeName, TypeExpr, TypeValidationPolicy};

/// Canonical source-level reference type.
///
/// This is a focused API over [`TypeExpr::Reference`]. The underlying
/// `TypeExpr` remains the authoritative AST representation.
///
/// A reference consists of:
///
/// - mutability;
/// - an optional source-level lifetime;
/// - the referenced type expression.
///
/// No semantic resolution is performed by this type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReferenceType(TypeExpr);

impl ReferenceType {
    /// Creates a reference type.
    ///
    /// The referenced type is preserved exactly as supplied. No semantic
    /// validation is performed during construction because parser recovery and
    /// programmatic AST construction may intentionally create temporarily
    /// incomplete structures.
    #[must_use]
    pub fn new(
        mutable: bool,
        lifetime: Option<LifetimeName>,
        inner: TypeExpr,
    ) -> Self {
        Self(TypeExpr::Reference {
            mutable,
            lifetime,
            inner: Box::new(inner),
        })
    }

    /// Creates an immutable reference.
    #[must_use]
    pub fn shared(inner: TypeExpr) -> Self {
        Self::new(false, None, inner)
    }

    /// Creates a mutable reference.
    #[must_use]
    pub fn mutable(inner: TypeExpr) -> Self {
        Self::new(true, None, inner)
    }

    /// Creates an immutable reference with a source-level lifetime.
    #[must_use]
    pub fn shared_with_lifetime(
        lifetime: LifetimeName,
        inner: TypeExpr,
    ) -> Self {
        Self::new(false, Some(lifetime), inner)
    }

    /// Creates a mutable reference with a source-level lifetime.
    #[must_use]
    pub fn mutable_with_lifetime(
        lifetime: LifetimeName,
        inner: TypeExpr,
    ) -> Self {
        Self::new(true, Some(lifetime), inner)
    }

    /// Returns whether the reference is mutable.
    #[must_use]
    pub fn is_mutable(&self) -> bool {
        match &self.0 {
            TypeExpr::Reference { mutable, .. } => *mutable,
            // The invariant of this wrapper guarantees this branch is
            // unreachable for valid instances. Keeping it total avoids a
            // panic if a future internal representation is introduced.
            _ => false,
        }
    }

    /// Returns whether the reference is immutable.
    #[must_use]
    pub fn is_shared(&self) -> bool {
        !self.is_mutable()
    }

    /// Returns the source-level lifetime, if one was explicitly supplied.
    #[must_use]
    pub fn lifetime(&self) -> Option<&LifetimeName> {
        match &self.0 {
            TypeExpr::Reference { lifetime, .. } => lifetime.as_ref(),
            _ => None,
        }
    }

    /// Returns the referenced source-level type.
    ///
    /// The returned value remains unresolved source syntax.
    #[must_use]
    pub fn inner(&self) -> &TypeExpr {
        match &self.0 {
            TypeExpr::Reference { inner, .. } => inner.as_ref(),
            _ => unreachable!("ReferenceType invariant violated"),
        }
    }

    /// Returns the canonical underlying [`TypeExpr`].
    #[must_use]
    pub fn as_type_expr(&self) -> &TypeExpr {
        &self.0
    }

    /// Consumes the wrapper and returns the canonical [`TypeExpr`].
    #[must_use]
    pub fn into_type_expr(self) -> TypeExpr {
        self.0
    }

    /// Returns the configured AST type-expression schema version.
    ///
    /// This method intentionally derives the version from the canonical
    /// `TypeExpr` module rather than maintaining a second local schema number.
    #[must_use]
    pub const fn schema_version() -> u16 {
        super::type_expr::TYPE_EXPR_SCHEMA_VERSION
    }

    /// Performs structural validation with the default policy.
    ///
    /// Semantic lifetime/ownership/borrow validation is deliberately not
    /// performed here.
    pub fn validate(&self) -> Result<(), super::type_expr::TypeExprError> {
        self.0.validate()
    }

    /// Performs structural validation using an explicit compiler policy.
    ///
    /// Policy limits are safety/resource limits, not language semantics.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), super::type_expr::TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Returns whether the referenced type is structurally valid under the
    /// default policy.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Returns the source-level lifetime name as text.
    #[must_use]
    pub fn lifetime_name(&self) -> Option<&str> {
        self.lifetime().map(LifetimeName::as_str)
    }

    /// Returns whether this reference has an explicit lifetime.
    #[must_use]
    pub fn has_explicit_lifetime(&self) -> bool {
        self.lifetime().is_some()
    }

    /// Returns a copy of this reference with a different mutability marker.
    ///
    /// The referenced type and lifetime are preserved exactly.
    #[must_use]
    pub fn with_mutability(&self, mutable: bool) -> Self {
        Self::new(mutable, self.lifetime().cloned(), self.inner().clone())
    }

    /// Returns a copy of this reference with the supplied lifetime.
    ///
    /// This changes source-level syntax only. It does not perform semantic
    /// lifetime checking.
    #[must_use]
    pub fn with_lifetime(&self, lifetime: Option<LifetimeName>) -> Self {
        Self::new(self.is_mutable(), lifetime, self.inner().clone())
    }

    /// Returns a copy of this reference with a different referenced type.
    ///
    /// This is a structural AST operation and intentionally performs no
    /// semantic compatibility checking.
    #[must_use]
    pub fn with_inner(&self, inner: TypeExpr) -> Self {
        Self::new(self.is_mutable(), self.lifetime().cloned(), inner)
    }

    /// Returns a stable source-level representation.
    ///
    /// This method does not attempt to pretty-print every possible future
    /// `TypeExpr` extension. It provides the canonical reference decoration
    /// around the existing `TypeExpr` display implementation.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        let mut result = String::new();

        result.push('&');

        if self.is_mutable() {
            result.push_str("mut ");
        }

        if let Some(lifetime) = self.lifetime() {
            result.push_str(&lifetime.to_string());
            result.push(' ');
        }

        result.push_str(&self.inner().to_string());
        result
    }
}

impl From<ReferenceType> for TypeExpr {
    fn from(value: ReferenceType) -> Self {
        value.into_type_expr()
    }
}

impl TryFrom<TypeExpr> for ReferenceType {
    type Error = TypeExpr;

    fn try_from(value: TypeExpr) -> Result<Self, Self::Error> {
        match value {
            TypeExpr::Reference { .. } => Ok(Self(value)),
            other => Err(other),
        }
    }
}

impl AsRef<TypeExpr> for ReferenceType {
    fn as_ref(&self) -> &TypeExpr {
        self.as_type_expr()
    }
}

impl fmt::Display for ReferenceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

/// Error returned when converting a non-reference type expression into a
/// [`ReferenceType`].
///
/// The original `TypeExpr` is returned by `TryFrom` so callers never lose the
/// AST value they attempted to convert.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotReferenceType;

impl fmt::Display for NotReferenceType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("type expression is not a reference type")
    }
}

/// Explicit conversion helper that exposes a named error instead of using
/// `TypeExpr` as the conversion error.
///
/// This is useful at parser/AST adapter boundaries where diagnostics need to
/// distinguish an invalid reference conversion from the original AST value.
impl ReferenceType {
    /// Attempts to interpret a type expression as a reference.
    pub fn try_from_type_expr(
        value: TypeExpr,
    ) -> Result<Self, (NotReferenceType, TypeExpr)> {
        match value {
            TypeExpr::Reference { .. } => Ok(Self(value)),
            other => Err((NotReferenceType, other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::ast::node::types::type_expr::{
        TypeName, TypePath,
    };

    #[test]
    fn shared_reference_preserves_inner_type() {
        let inner = TypeExpr::Identifier(TypePath::single("Int"));
        let reference = ReferenceType::shared(inner.clone());

        assert!(reference.is_shared());
        assert!(!reference.is_mutable());
        assert_eq!(reference.inner(), &inner);
        assert!(!reference.has_explicit_lifetime());
    }

    #[test]
    fn mutable_reference_preserves_mutability() {
        let inner = TypeExpr::Identifier(TypePath::single("Value"));
        let reference = ReferenceType::mutable(inner);

        assert!(reference.is_mutable());
        assert!(!reference.is_shared());
    }

    #[test]
    fn lifetime_is_source_level_only() {
        let inner = TypeExpr::Identifier(TypePath::single("Value"));
        let lifetime = LifetimeName::from("data");

        let reference =
            ReferenceType::shared_with_lifetime(lifetime.clone(), inner);

        assert_eq!(reference.lifetime(), Some(&lifetime));
        assert_eq!(reference.lifetime_name(), Some("data"));
        assert_eq!(reference.to_source_string(), "&'data Value");
    }

    #[test]
    fn mutable_lifetime_reference_is_deterministic() {
        let inner = TypeExpr::Identifier(TypePath::single("Value"));
        let lifetime = LifetimeName::from("scope");

        let reference =
            ReferenceType::mutable_with_lifetime(lifetime, inner);

        assert_eq!(reference.to_source_string(), "&mut 'scope Value");
    }

    #[test]
    fn converts_to_canonical_type_expr() {
        let inner = TypeExpr::Identifier(TypePath::single("Value"));
        let reference = ReferenceType::shared(inner.clone());

        let type_expr = reference.into_type_expr();

        assert!(matches!(
            type_expr,
            TypeExpr::Reference {
                mutable: false,
                lifetime: None,
                ..
            }
        ));
    }

    #[test]
    fn converts_reference_type_expr_back() {
        let type_expr = TypeExpr::Reference {
            mutable: true,
            lifetime: None,
            inner: Box::new(TypeExpr::Identifier(TypePath::single("Value"))),
        };

        let reference =
            ReferenceType::try_from(type_expr).expect("reference expected");

        assert!(reference.is_mutable());
        assert_eq!(reference.inner().to_string(), "Value");
    }

    #[test]
    fn rejects_non_reference_type_expr() {
        let type_expr = TypeExpr::Identifier(TypePath::single("Value"));

        let result = ReferenceType::try_from(type_expr);

        assert!(result.is_err());
    }

    #[test]
    fn named_type_paths_are_supported_without_domain_assumptions() {
        let inner = TypeExpr::Identifier(TypePath::new(vec![
            TypeName::from("quantum"),
            TypeName::from("State"),
        ]));

        let reference = ReferenceType::shared(inner);

        assert_eq!(
            reference.to_source_string(),
            "&quantum::State"
        );
    }

    #[test]
    fn generic_inner_types_are_preserved() {
        let inner = TypeExpr::Generic {
            base: Box::new(TypeExpr::Identifier(TypePath::single("Register"))),
            arguments: vec![TypeExpr::Identifier(TypePath::single("Q"))],
        };

        let reference = ReferenceType::shared(inner.clone());

        assert_eq!(reference.inner(), &inner);
    }

    #[test]
    fn quantum_types_remain_source_level() {
        let inner = TypeExpr::Quantum(Box::new(
            TypeExpr::Identifier(TypePath::single("Q")),
        ));

        let reference = ReferenceType::shared(inner.clone());

        assert_eq!(reference.inner(), &inner);
    }

    #[test]
    fn symbolic_generic_reference_has_no_machine_limit() {
        let inner = TypeExpr::Generic {
            base: Box::new(TypeExpr::Identifier(TypePath::single("Register"))),
            arguments: vec![TypeExpr::GenericParameter(
                super::super::type_expr::TypeParameterName::from("N"),
            )],
        };

        let reference = ReferenceType::shared(inner);

        assert!(reference.validate().is_ok());
    }

    #[test]
    fn policy_validation_is_delegated_to_canonical_type_expr() {
        let inner = TypeExpr::Tuple(vec![
            TypeExpr::Unit,
            TypeExpr::Unit,
            TypeExpr::Unit,
        ]);

        let reference = ReferenceType::shared(inner);

        let policy = TypeValidationPolicy {
            max_collection_items: Some(2),
            ..TypeValidationPolicy::default()
        };

        assert!(reference.validate_with_policy(&policy).is_err());
    }

    #[test]
    fn mutability_transformation_preserves_inner_and_lifetime() {
        let inner = TypeExpr::Identifier(TypePath::single("Value"));
        let lifetime = LifetimeName::from("scope");

        let reference =
            ReferenceType::shared_with_lifetime(lifetime.clone(), inner.clone());

        let mutable = reference.with_mutability(true);

        assert!(mutable.is_mutable());
        assert_eq!(mutable.lifetime(), Some(&lifetime));
        assert_eq!(mutable.inner(), &inner);
    }

    #[test]
    fn lifetime_transformation_preserves_other_structure() {
        let inner = TypeExpr::Identifier(TypePath::single("Value"));

        let reference = ReferenceType::shared(inner.clone());
        let changed =
            reference.with_lifetime(Some(LifetimeName::from("scope")));

        assert_eq!(changed.inner(), &inner);
        assert_eq!(changed.lifetime_name(), Some("scope"));
    }

    #[test]
    fn inner_transformation_preserves_reference_metadata() {
        let original_inner = TypeExpr::Identifier(TypePath::single("A"));
        let replacement_inner = TypeExpr::Identifier(TypePath::single("B"));
        let lifetime = LifetimeName::from("scope");

        let reference =
            ReferenceType::mutable_with_lifetime(lifetime.clone(), original_inner);

        let changed = reference.with_inner(replacement_inner.clone());

        assert!(changed.is_mutable());
        assert_eq!(changed.lifetime(), Some(&lifetime));
        assert_eq!(changed.inner(), &replacement_inner);
    }

    #[test]
    fn schema_version_matches_canonical_type_expr_schema() {
        assert_eq!(
            ReferenceType::schema_version(),
            super::super::type_expr::TYPE_EXPR_SCHEMA_VERSION
        );
    }

    #[test]
    fn serialization_round_trip_preserves_reference() {
        let inner = TypeExpr::Identifier(TypePath::single("Value"));
        let reference =
            ReferenceType::mutable_with_lifetime(
                LifetimeName::from("scope"),
                inner,
            );

        let encoded =
            serde_json::to_string(&reference).expect("serialization failed");

        let decoded: ReferenceType =
            serde_json::from_str(&encoded).expect("deserialization failed");

        assert_eq!(decoded, reference);
    }

    #[test]
    fn no_semantic_resolution_is_embedded() {
        let inner =
            TypeExpr::Identifier(TypePath::single("FutureResource"));

        let reference = ReferenceType::shared(inner);

        // The AST contains only source-level information.
        assert_eq!(
            reference.inner().to_string(),
            "FutureResource"
        );
    }
}