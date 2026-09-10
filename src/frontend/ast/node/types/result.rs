//! Zamani Native AST — Result Type
//!
//! Production-ready typed façade for the canonical source-level
//! [`TypeExpr::Result`] representation.
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
//! TypeExpr::Result(ok, error)
//!     │
//!     ├── ResultType (this façade)
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic Result/error model
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
//! `ResultType` provides a strongly typed, invariant-preserving API for the
//! canonical source-level [`TypeExpr::Result`] representation.
//!
//! It does **not** define a second Result AST representation.
//!
//! The authoritative representation remains:
//!
//! ```text
//! TypeExpr::Result(Box<TypeExpr>, Box<TypeExpr>)
//! ```
//!
//! The first child is the successful-value type.
//!
//! The second child is the error type.
//!
//! This distinction is important for AST stability: parser, validation,
//! serialization, semantic analysis, and future compiler stages continue to
//! share one canonical representation.
//!
//! # Source-level meaning
//!
//! A Result type expresses an operation whose source-level outcome is one of
//! two alternatives:
//!
//! ```text
//! success(T)
//! error(E)
//! ```
//!
//! Conceptually:
//!
//! ```text
//! Result<T, E>
//! ```
//!
//! `T` and `E` are arbitrary source-level [`TypeExpr`] values.
//!
//! They may contain:
//!
//! - named types;
//! - generic types;
//! - tuples;
//! - arrays;
//! - slices;
//! - references;
//! - pointers;
//! - optional types;
//! - other Result types;
//! - user-defined types;
//! - symbolic/resource types;
//! - future extension types.
//!
//! This file does not determine how either branch is represented at runtime.
//!
//! # Domain neutrality
//!
//! `ResultType` is a general language construct.
//!
//! It is not a quantum Result type, hardware Result type, measurement-result
//! type, QEC-result type, backend-result type, or runtime execution result.
//!
//! A Result may contain a source-level type associated with:
//!
//! - classical computation;
//! - quantum computation;
//! - hybrid computation;
//! - distributed computation;
//! - accelerator computation;
//! - HDL computation;
//! - AI computation;
//! - future computational domains.
//!
//! The meaning of those types is resolved by later compiler stages.
//!
//! This file must never depend on:
//!
//! - quantum hardware;
//! - physical qubits;
//! - QPU topology;
//! - gate sets;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse generation;
//! - QEC;
//! - resilience;
//! - noise models;
//! - backend queues;
//! - CPUs;
//! - GPUs;
//! - FPGAs;
//! - ASICs;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - ZUIR implementation details;
//! - vendor SDKs.
//!
//! # POCO-REAF
//!
//! `ResultType` introduces no machine-size assumptions.
//!
//! It contains no:
//!
//! ```text
//! MAX_RESULT_SIZE
//! MAX_ERROR_SIZE
//! MAX_RESULT_DEPTH
//! MAX_QUBITS
//! MAX_REGISTER_SIZE
//! MAX_MACHINE_SIZE
//! ```
//!
//! A Result can therefore contain source-level types representing arbitrarily
//! large structures, symbolic resources, or future computational abstractions.
//!
//! Any compiler resource limit must be supplied explicitly by the compiler's
//! resource/validation policy rather than encoded here.
//!
//! # Important architectural distinction
//!
//! ```text
//! ResultType
//!     = typed source-level façade
//!
//! TypeExpr::Result
//!     = canonical source-level AST representation
//!
//! SemanticType::Result
//!     = resolved semantic meaning
//!
//! ZUIR
//!     = universal computational representation
//!
//! Domain IR
//!     = domain-specific implementation
//!
//! Target IR
//!     = target/backend representation
//! ```
//!
//! This file must never become a semantic Result implementation.
//!
//! # Error branch semantics
//!
//! The error branch is deliberately represented as a [`TypeExpr`].
//!
//! It is therefore not restricted to a particular built-in error type.
//!
//! For example, the source language may eventually permit:
//!
//! ```text
//! Result<Value, Error>
//! Result<Value, MyError>
//! Result<Value, (ParseError, IoError)>
//! Result<Value, GenericError<T>>
//! Result<QuantumState, QuantumError>
//! ```
//!
//! Whether a particular combination is semantically legal belongs to semantic
//! analysis.
//!
//! This file must not encode a closed list of permissible error types.
//!
//! # Quantum integration boundary
//!
//! Quantum measurement results must not be confused with this source-level
//! Result type.
//!
//! For example:
//!
//! ```text
//! Result<Measurement, MeasurementError>
//! ```
//!
//! is merely a source-level type expression.
//!
//! It does not mean that the AST knows how measurement is performed.
//!
//! Actual measurement semantics, classical-result representation, execution
//! failure, hardware failure, retry policy, error mitigation, or QEC behavior
//! belong to later stages.
//!
//! # Semantic boundary
//!
//! This module does not perform:
//!
//! - name resolution;
//! - type inference;
//! - type unification;
//! - generic substitution;
//! - trait resolution;
//! - overload resolution;
//! - ownership checking;
//! - borrow checking;
//! - effect checking;
//! - resource allocation;
//! - quantum legality checking;
//! - topology checking;
//! - routing;
//! - scheduling;
//! - calibration;
//! - backend selection;
//! - runtime execution.
//!
//! Semantic analysis consumes the canonical `TypeExpr::Result` and determines
//! the language-level meaning of both branches.
//!
//! # Parser integration
//!
//! The parser should construct the canonical representation:
//!
//! ```text
//! parser
//!     → TypeExpr::Result(ok, error)
//! ```
//!
//! It may use `ResultType` as a focused construction/inspection API, but the
//! parser must not create an independent Result AST schema.
//!
//! Parser recovery may temporarily contain incomplete child type expressions.
//! Complete structural validity is established by the canonical type-expression
//! validation system.
//!
//! # TypeExpr integration
//!
//! `ResultType::into_type_expr()` returns the canonical
//! `TypeExpr::Result` representation.
//!
//! `ResultType::try_from_type_expr()` accepts only a `TypeExpr::Result`.
//!
//! No information is lost during either conversion.
//!
//! # Structural validation
//!
//! Validation delegates to [`TypeExpr::validate`] and
//! [`TypeExpr::validate_with_policy`].
//!
//! This is intentional.
//!
//! There must be exactly one authoritative structural validation mechanism for
//! type expressions.
//!
//! `ResultType` must not create a second validation implementation that could
//! diverge from `TypeExpr`.
//!
//! # Semantic integration
//!
//! Semantic analysis consumes:
//!
//! ```text
//! TypeExpr::Result(ok, error)
//! ```
//!
//! and resolves both branches independently.
//!
//! Conceptually:
//!
//! ```text
//! ResultType
//!     │
//!     ▼
//! TypeExpr::Result(ok, error)
//!     │
//!     ├── resolve(ok)
//!     │
//!     └── resolve(error)
//!     │
//!     ▼
//! semantic Result type
//! ```
//!
//! Semantic analysis decides:
//!
//! - whether both branches are valid;
//! - whether the error branch satisfies language constraints;
//! - generic constraints;
//! - ownership/resource rules;
//! - conversions;
//! - control-flow semantics;
//! - pattern matching;
//! - propagation semantics;
//! - interaction with effects.
//!
//! None of those rules belong in this file.
//!
//! # ZUIR integration
//!
//! `result.rs` has no dependency on ZUIR.
//!
//! The intended pipeline is:
//!
//! ```text
//! TypeExpr::Result
//!     │
//!     ▼
//! semantic Result type
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! ZUIR determines how the semantic Result construct participates in universal
//! computation.
//!
//! This AST file must not know whether a Result becomes:
//!
//! - a tagged value;
//! - control flow;
//! - an exception-like mechanism;
//! - a sum type;
//! - a resource outcome;
//! - a distributed message;
//! - a quantum/classical boundary;
//! - some future representation.
//!
//! # Serialization
//!
//! Serialization is inherited from the canonical `TypeExpr` representation.
//!
//! `ResultType` therefore does not introduce a competing serialized schema.
//!
//! This provides:
//!
//! - deterministic representation;
//! - stable ordering;
//! - round-trip compatibility;
//! - one authoritative wire representation;
//! - easier schema versioning.
//!
//! Deserialization enforces the wrapper invariant by accepting only a
//! `TypeExpr::Result`.
//!
//! # Determinism
//!
//! A Result contains exactly two ordered child type expressions:
//!
//! ```text
//! ok
//! error
//! ```
//!
//! Their ordering is semantically significant and is never represented using
//! an unordered collection.
//!
//! Equality, hashing, debugging and canonical serialization are therefore
//! deterministic for equal source-level structures.
//!
//! # Scalability
//!
//! Result types add a constant number of immediate AST edges:
//!
//! ```text
//! Result
//! ├── success type
//! └── error type
//! ```
//!
//! No representation is allocated based on the number of possible runtime
//! outcomes.
//!
//! Nested Result types are naturally representable:
//!
//! ```text
//! Result<T, E>
//! Result<Result<T, E1>, E2>
//! Result<T, Result<E1, E2>>
//! ```
//!
//! No language-level Result nesting limit is introduced here.
//!
//! Any compiler protection against pathological source input must be provided
//! by explicit compiler resource policies.
//!
//! # Security
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no filesystem access;
//! - performs no network access;
//! - performs no code execution;
//! - performs no FFI;
//! - performs no raw memory operations;
//! - performs no pointer arithmetic;
//! - contains no `unsafe`;
//! - does not contain machine addresses;
//! - does not trust a deserialized variant without checking it.
//!
//! The AST is compiler input and must be treated as untrusted data.
//!
//! # Performance
//!
//! Construction performs only the allocation required by the canonical boxed
//! children.
//!
//! Accessors borrow the existing child expressions.
//!
//! Converting into `TypeExpr` moves the canonical representation without
//! cloning the subtree.
//!
//! No hash maps or repeated global scans are introduced.
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
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR implementation;
//! - quantum compiler implementation;
//! - quantum hardware;
//! - routing;
//! - scheduling;
//! - QEC;
//! - ZQN;
//! - resilience;
//! - calibration;
//! - backend execution;
//! - LLVM;
//! - QIR;
//! - MLIR;
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
//! - `ResultType` has exactly one canonical representation;
//! - success and error branches are preserved exactly;
//! - arbitrary `TypeExpr` values are accepted as branches;
//! - conversion from arbitrary `TypeExpr` is checked;
//! - deserialization preserves the canonical representation;
//! - structural validation delegates to `TypeExpr`;
//! - equality/hash/debug behavior is deterministic;
//! - source formatting is deterministic;
//! - no machine-specific information exists;
//! - no hidden size/depth/resource limit exists;
//! - parser integration is defined;
//! - semantic integration is defined;
//! - ZUIR integration is defined;
//! - quantum integration remains domain-neutral;
//! - public APIs have tests;
//! - malformed structures fail safely;
//! - no unsafe code exists.
//!
//! # Migration contract
//!
//! The legacy frontend/AST representation already models Result as a pair of
//! success and error types. Existing downstream semantic code resolves those
//! two branches independently. The new façade preserves that conceptual
//! contract rather than introducing a new representation. 
//!
//! Existing code using:
//!
//! ```rust
//! TypeExpr::Result(Box::new(ok), Box::new(error))
//! ```
//!
//! remains the canonical form.
//!
//! `ResultType` can therefore be adopted incrementally without forcing
//! unrelated AST files to change.
//!
//! # Future compatibility
//!
//! A future language extension may introduce additional Result-related syntax,
//! attributes, effects, capabilities, or resource semantics.
//!
//! Those must not be added as fields to this façade unless they are genuinely
//! part of the canonical `TypeExpr::Result` source representation.
//!
//! Backend metadata, runtime status, execution identifiers, hardware errors,
//! calibration failures, queue information, and retry state must never be
//! embedded into this AST type.
//!
//! # Testing strategy
//!
//! The tests below verify the invariant and public structural API.
//!
//! Larger AST-wide tests should additionally verify:
//!
//! ```text
//! parser → TypeExpr::Result
//!
//! TypeExpr::Result
//!     → structural validation
//!
//! TypeExpr::Result
//!     → semantic resolution
//!
//! semantic Result
//!     → ZUIR
//!
//! AST → serialize → deserialize → AST
//! ```
//!
//! No backend-specific output belongs in this unit's tests.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Deserializer, Serialize};

use super::type_expr::{TypeExpr, TypeExprError, TypeValidationPolicy};

/// Strongly typed façade for [`TypeExpr::Result`].
///
/// The canonical AST representation remains:
///
/// ```text
/// TypeExpr::Result(Box<TypeExpr>, Box<TypeExpr>)
/// ```
///
/// `ResultType` provides a focused API without introducing a second AST
/// representation.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct ResultType(TypeExpr);

impl ResultType {
    /// Constructs a source-level Result type.
    ///
    /// `ok` is the successful-value type.
    ///
    /// `error` is the error type.
    ///
    /// Construction establishes the representation invariant immediately.
    ///
    /// No semantic validation is performed here.
    #[must_use]
    pub fn new(ok: TypeExpr, error: TypeExpr) -> Self {
        Self(TypeExpr::Result(Box::new(ok), Box::new(error)))
    }

    /// Constructs a Result and validates its complete type structure using
    /// the default compiler validation policy.
    ///
    /// The default policy belongs to the AST validation infrastructure and does
    /// not introduce a Result-specific language limit.
    pub fn try_new(ok: TypeExpr, error: TypeExpr) -> Result<Self, TypeExprError> {
        Self::try_new_with_policy(ok, error, &TypeValidationPolicy::default())
    }

    /// Constructs a Result and validates it using an explicit compiler
    /// resource/validation policy.
    ///
    /// Resource policies are compilation-safety controls, not part of the
    /// source-language representation.
    pub fn try_new_with_policy(
        ok: TypeExpr,
        error: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, TypeExprError> {
        let result = Self::new(ok, error);
        result.validate_with_policy(policy)?;
        Ok(result)
    }

    /// Attempts to construct a `ResultType` from an arbitrary type expression.
    ///
    /// The supplied expression must already be the canonical
    /// `TypeExpr::Result` variant.
    pub fn try_from_type_expr(value: TypeExpr) -> Result<Self, ResultTypeError> {
        match value {
            TypeExpr::Result(_, _) => Ok(Self(value)),
            other => Err(ResultTypeError::NotResult {
                variant: other.variant_name(),
            }),
        }
    }

    /// Attempts to construct a `ResultType` from an arbitrary type expression
    /// and validate the complete structure under an explicit policy.
    pub fn try_from_type_expr_with_policy(
        value: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, ResultTypeError> {
        let result = Self::try_from_type_expr(value)?;

        result
            .validate_with_policy(policy)
            .map_err(ResultTypeError::InvalidType)?;

        Ok(result)
    }

    /// Returns the successful-value type.
    ///
    /// The returned value is unresolved source-level syntax.
    #[must_use]
    pub fn ok(&self) -> &TypeExpr {
        match &self.0 {
            TypeExpr::Result(ok, _) => ok.as_ref(),
            _ => unreachable!("ResultType invariant violated"),
        }
    }

    /// Returns the error type.
    ///
    /// The returned value is unresolved source-level syntax.
    #[must_use]
    pub fn error(&self) -> &TypeExpr {
        match &self.0 {
            TypeExpr::Result(_, error) => error.as_ref(),
            _ => unreachable!("ResultType invariant violated"),
        }
    }

    /// Returns the canonical underlying type expression.
    #[must_use]
    pub fn as_type_expr(&self) -> &TypeExpr {
        &self.0
    }

    /// Consumes the façade and returns the canonical type expression.
    #[must_use]
    pub fn into_type_expr(self) -> TypeExpr {
        self.0
    }

    /// Replaces the successful-value type while preserving the error type.
    ///
    /// This is a structural AST operation. Semantic compatibility is not
    /// checked here.
    #[must_use]
    pub fn with_ok(&self, ok: TypeExpr) -> Self {
        Self::new(ok, self.error().clone())
    }

    /// Replaces the error type while preserving the successful-value type.
    ///
    /// This is a structural AST operation. Semantic compatibility is not
    /// checked here.
    #[must_use]
    pub fn with_error(&self, error: TypeExpr) -> Self {
        Self::new(self.ok().clone(), error)
    }

    /// Returns both branches as an ordered pair of borrowed type expressions.
    ///
    /// The ordering is always `(success, error)`.
    #[must_use]
    pub fn branches(&self) -> (&TypeExpr, &TypeExpr) {
        (self.ok(), self.error())
    }

    /// Returns whether the success branch is structurally equal to the error
    /// branch.
    ///
    /// This is purely structural. It does not imply that a Result is semantically
    /// redundant.
    #[must_use]
    pub fn branches_equal(&self) -> bool {
        self.ok() == self.error()
    }

    /// Returns the canonical frontend type-expression schema version.
    ///
    /// Result does not maintain an independent schema version because doing so
    /// would create competing version sources.
    #[must_use]
    pub const fn schema_version() -> u16 {
        super::type_expr::TYPE_EXPR_SCHEMA_VERSION
    }

    /// Validates the complete Result structure using the canonical type
    /// expression validator.
    ///
    /// Semantic Result rules are intentionally excluded.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.0.validate()
    }

    /// Validates the complete Result structure using an explicit compiler
    /// validation/resource policy.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Returns whether this Result is structurally valid under the default
    /// validation policy.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    /// Produces the canonical source-level textual representation.
    ///
    /// The actual spelling of the child expressions is delegated to
    /// `TypeExpr`'s display implementation.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        format!("Result<{}, {}>", self.ok(), self.error())
    }
}

impl From<ResultType> for TypeExpr {
    fn from(value: ResultType) -> Self {
        value.into_type_expr()
    }
}

impl TryFrom<TypeExpr> for ResultType {
    type Error = ResultTypeError;

    fn try_from(value: TypeExpr) -> Result<Self, Self::Error> {
        Self::try_from_type_expr(value)
    }
}

impl AsRef<TypeExpr> for ResultType {
    fn as_ref(&self) -> &TypeExpr {
        self.as_type_expr()
    }
}

impl fmt::Display for ResultType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

/// Error returned when an arbitrary [`TypeExpr`] cannot be viewed as a
/// [`ResultType`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResultTypeError {
    /// The supplied type expression was not a Result expression.
    NotResult {
        /// Stable source-level variant name.
        variant: &'static str,
    },

    /// The supplied Result expression failed structural validation.
    InvalidType(TypeExprError),
}

impl fmt::Display for ResultTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotResult { variant } => {
                write!(
                    formatter,
                    "expected Result type expression, found {variant}"
                )
            }
            Self::InvalidType(error) => {
                write!(formatter, "invalid Result type expression: {error}")
            }
        }
    }
}

impl std::error::Error for ResultTypeError {}

/// Serde deserialization implementation that enforces the `ResultType`
/// representation invariant.
///
/// The serialized form is the canonical `TypeExpr` form. This prevents a
/// second Result-specific wire schema from diverging from `TypeExpr`.
impl<'de> Deserialize<'de> for ResultType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = TypeExpr::deserialize(deserializer)?;

        Self::try_from_type_expr(value)
            .map_err(|error| serde::de::Error::custom(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(name: &str) -> TypeExpr {
        TypeExpr::Identifier(super::super::type_expr::TypePath::single(name))
    }

    #[test]
    fn constructor_preserves_success_and_error_types() {
        let ok = named("Value");
        let error = named("Error");

        let result = ResultType::new(ok.clone(), error.clone());

        assert_eq!(result.ok(), &ok);
        assert_eq!(result.error(), &error);
    }

    #[test]
    fn canonical_conversion_round_trips() {
        let value = TypeExpr::Result(
            Box::new(named("Value")),
            Box::new(named("Error")),
        );

        let result =
            ResultType::try_from_type_expr(value.clone()).expect("Result must convert");

        assert_eq!(result.as_type_expr(), &value);
        assert_eq!(result.into_type_expr(), value);
    }

    #[test]
    fn non_result_type_is_rejected() {
        let value = named("NotResult");

        let error =
            ResultType::try_from_type_expr(value).expect_err("non-Result must fail");

        assert!(matches!(
            error,
            ResultTypeError::NotResult { .. }
        ));
    }

    #[test]
    fn accessors_borrow_without_cloning() {
        let result = ResultType::new(named("Value"), named("Error"));

        let (ok, error) = result.branches();

        assert_eq!(ok, result.ok());
        assert_eq!(error, result.error());
    }

    #[test]
    fn branches_are_ordered_success_then_error() {
        let result = ResultType::new(named("Success"), named("Failure"));

        assert_eq!(result.ok().to_string(), "Success");
        assert_eq!(result.error().to_string(), "Failure");
    }

    #[test]
    fn branch_equality_is_structural() {
        let result = ResultType::new(named("Same"), named("Same"));

        assert!(result.branches_equal());
    }

    #[test]
    fn branch_equality_detects_difference() {
        let result = ResultType::new(named("Value"), named("Error"));

        assert!(!result.branches_equal());
    }

    #[test]
    fn with_ok_preserves_error() {
        let result = ResultType::new(named("OldValue"), named("Error"));
        let updated = result.with_ok(named("NewValue"));

        assert_eq!(updated.ok().to_string(), "NewValue");
        assert_eq!(updated.error().to_string(), "Error");
    }

    #[test]
    fn with_error_preserves_success() {
        let result = ResultType::new(named("Value"), named("OldError"));
        let updated = result.with_error(named("NewError"));

        assert_eq!(updated.ok().to_string(), "Value");
        assert_eq!(updated.error().to_string(), "NewError");
    }

    #[test]
    fn display_is_source_oriented() {
        let result = ResultType::new(named("Value"), named("Error"));

        assert_eq!(result.to_string(), "Result<Value, Error>");
    }

    #[test]
    fn schema_version_comes_from_canonical_type_expr() {
        assert_eq!(
            ResultType::schema_version(),
            super::super::type_expr::TYPE_EXPR_SCHEMA_VERSION
        );
    }

    #[test]
    fn serde_round_trip_preserves_canonical_structure() {
        let result = ResultType::new(
            named("Value"),
            TypeExpr::Generic {
                base: Box::new(named("Error")),
                arguments: vec![named("Code")],
            },
        );

        let encoded =
            serde_json::to_string(&result).expect("Result serialization must succeed");

        let decoded: ResultType =
            serde_json::from_str(&encoded).expect("Result deserialization must succeed");

        assert_eq!(decoded, result);
    }

    #[test]
    fn nested_results_are_supported() {
        let inner = ResultType::new(named("Value"), named("InnerError"));

        let outer = ResultType::new(
            inner.clone().into_type_expr(),
            named("OuterError"),
        );

        assert_eq!(outer.ok(), inner.as_type_expr());
        assert_eq!(outer.error().to_string(), "OuterError");
    }

    #[test]
    fn result_can_contain_generic_types() {
        let result = ResultType::new(
            TypeExpr::Generic {
                base: Box::new(named("Value")),
                arguments: vec![named("T")],
            },
            TypeExpr::Generic {
                base: Box::new(named("Error")),
                arguments: vec![named("E")],
            },
        );

        assert_eq!(result.ok().to_string(), "Value<T>");
        assert_eq!(result.error().to_string(), "Error<E>");
    }

    #[test]
    fn no_machine_specific_information_is_encoded() {
        let result = ResultType::new(
            named("QuantumState"),
            named("ComputationError"),
        );

        let text = result.to_source_string();

        assert!(text.contains("QuantumState"));
        assert!(text.contains("ComputationError"));
        assert!(!text.contains("IBM"));
        assert!(!text.contains("IonQ"));
        assert!(!text.contains("LLVM"));
        assert!(!text.contains("QIR"));
    }
}