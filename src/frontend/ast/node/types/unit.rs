//! Zamani Native AST — Unit Type
//!
//! Production-ready typed façade for the canonical source-level
//! [`TypeExpr::Unit`] representation.
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
//! TypeExpr::Unit
//!     │
//!     ├── UnitType  ← this façade
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic unit type
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
//! `UnitType` is a strongly typed façade over the canonical source-level
//! [`TypeExpr::Unit`] representation.
//!
//! It does **not** introduce a second independent AST representation.
//!
//! The authoritative representation remains:
//!
//! ```rust
//! TypeExpr::Unit
//! ```
//!
//! The façade exists for compiler components that specifically need to
//! construct, inspect, validate, or consume a unit type without repeatedly
//! matching the complete `TypeExpr` enum.
//!
//! # Source-level meaning
//!
//! Unit represents the source-language type containing exactly one conceptual
//! value, commonly used for computations whose meaningful result is absence of
//! a data payload while still having a type-level result.
//!
//! The exact semantic interpretation of unit is owned by the Zamani semantic
//! type system.
//!
//! This module does not decide whether unit is used for:
//!
//! - procedures/functions with no meaningful return value;
//! - empty tuples;
//! - effect-only computations;
//! - control-flow values;
//! - resource-independent computations;
//! - interoperability conventions;
//! - a particular ABI representation;
//! - a particular machine register representation.
//!
//! Those decisions belong to later compiler phases.
//!
//! # Canonical representation
//!
//! `type_expr.rs` owns the canonical representation:
//!
//! ```rust
//! TypeExpr::Unit
//! ```
//!
//! `UnitType` stores that canonical representation directly.
//!
//! It does not create a competing structure such as:
//!
//! ```text
//! struct UnitType {
//!     ...
//! }
//! ```
//!
//! containing duplicated semantic state.
//!
//! This guarantees a single source of truth.
//!
//! # Domain neutrality
//!
//! Unit is a general source-language type.
//!
//! This module deliberately knows nothing about:
//!
//! - quantum hardware;
//! - physical qubits;
//! - logical qubits;
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
//! - backend instructions;
//! - vendor SDKs.
//!
//! A quantum, classical, hybrid, distributed, accelerator, HDL, AI/ML, or
//! future-domain computation may semantically produce unit. This façade does
//! not need to know how that computation is eventually realized.
//!
//! # POCO-REAF
//!
//! `UnitType` contains no machine-size-dependent information.
//!
//! It has no:
//!
//! - machine width;
//! - pointer width;
//! - register width;
//! - memory capacity;
//! - qubit count;
//! - register count;
//! - topology;
//! - vendor identifier;
//! - backend identifier;
//! - instruction-set identifier;
//! - hardware address;
//! - execution queue;
//! - target-specific ABI information.
//!
//! Therefore unit does not constrain the scale at which a program can execute.
//!
//! A unit-typed computation can participate in programs ranging from the
//! smallest supported target to arbitrarily large targets supported by the
//! compilation/execution environment.
//!
//! No artificial language-level limit is introduced by this file.
//!
//! # Parser integration
//!
//! The parser owns recognition of unit syntax.
//!
//! The parser must construct the canonical representation:
//!
//! ```text
//! parser → TypeExpr::Unit
//! ```
//!
//! `UnitType` is not required for parsing.
//!
//! Parser code may use this façade after construction when a strongly typed
//! unit-specific API is useful.
//!
//! Parser recovery remains a parser responsibility.
//!
//! # TypeExpr integration
//!
//! `UnitType` is a typed view over:
//!
//! ```rust
//! TypeExpr::Unit
//! ```
//!
//! Conversion into `TypeExpr` always returns the canonical variant.
//!
//! Conversion from arbitrary `TypeExpr` is fallible and verifies the variant.
//!
//! There is therefore exactly one authoritative representation.
//!
//! # Structural validation
//!
//! `UnitType` contains no children.
//!
//! Structural validation therefore delegates to the canonical `TypeExpr`
//! validation implementation.
//!
//! This is intentional: there must not be two independent validation systems
//! capable of disagreeing about whether `Unit` is structurally valid.
//!
//! # Semantic integration
//!
//! Semantic analysis consumes the canonical:
//!
//! ```text
//! TypeExpr::Unit
//! ```
//!
//! and resolves it into the semantic type representation.
//!
//! Semantic analysis is responsible for determining:
//!
//! - unit compatibility;
//! - unit coercions, if any;
//! - function return semantics;
//! - generic substitution involving unit;
//! - pattern matching involving unit;
//! - ownership/resource implications;
//! - interaction with effects;
//! - interaction with control flow;
//! - domain-specific semantic legality.
//!
//! None of these decisions are encoded here.
//!
//! # ZUIR integration
//!
//! This file deliberately has no dependency on ZUIR.
//!
//! The intended pipeline is:
//!
//! ```text
//! UnitType
//!     │
//!     ▼
//! TypeExpr::Unit
//!     │
//!     ▼
//! semantic unit type
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ▼
//! domain / target representation
//! ```
//!
//! If ZUIR represents a unit-producing operation as a value, terminator,
//! effect-only computation, or another construct, that decision belongs to
//! semantic lowering and ZUIR.
//!
//! # Quantum integration
//!
//! No quantum-specific integration is required.
//!
//! A quantum operation or hybrid program may eventually have unit as its
//! semantic result type. This file does not interpret unit as:
//!
//! - a measured qubit;
//! - an empty quantum register;
//! - a released resource;
//! - a reset state;
//! - a hardware acknowledgement;
//! - a backend completion token.
//!
//! Those meanings, if present, belong to the relevant semantic/domain layers.
//!
//! # Serialization
//!
//! Serialization uses the canonical `TypeExpr` representation.
//!
//! `UnitType` therefore introduces no competing wire representation.
//!
//! Deserialization explicitly verifies that the decoded value is exactly:
//!
//! ```text
//! TypeExpr::Unit
//! ```
//!
//! A different type-expression variant is rejected instead of being silently
//! accepted as a `UnitType`.
//!
//! This protects the façade invariant when serialized data is untrusted.
//!
//! # Determinism
//!
//! `UnitType` contains no:
//!
//! - unordered collections;
//! - generated identifiers;
//! - timestamps;
//! - random values;
//! - memory addresses;
//! - target information;
//! - mutable global state.
//!
//! Equality, hashing, debugging and serialization are deterministic for equal
//! canonical values.
//!
//! # Scalability
//!
//! Unit has constant structural representation:
//!
//! ```text
//! UnitType
//!     └── TypeExpr::Unit
//! ```
//!
//! There is no representation proportional to:
//!
//! - machine size;
//! - program data size;
//! - qubit count;
//! - processor count;
//! - backend capacity.
//!
//! Consequently this file introduces no machine-size ceiling.
//!
//! Compiler resource limits remain explicit compiler-policy concerns and must
//! not be hidden inside this AST façade.
//!
//! # Performance
//!
//! Construction requires no heap allocation beyond the `TypeExpr` value itself.
//!
//! Variant conversion is constant time.
//!
//! Access to the canonical representation is constant time.
//!
//! `into_type_expr()` moves the canonical value.
//!
//! `to_type_expr()` constructs the zero-payload `TypeExpr::Unit` value directly.
//!
//! No recursive traversal is required because unit has no children.
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
//! - performs no pointer arithmetic;
//! - contains no `unsafe` code;
//! - has no mutable global state;
//! - validates variant identity when converting from arbitrary `TypeExpr`;
//! - validates the same invariant during deserialization.
//!
//! The AST must be treated as untrusted compiler input.
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
//! - legacy `crate::ast`;
//! - lexer implementation;
//! - parser implementation;
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
//! - resilience;
//! - runtime execution;
//! - QIR;
//! - LLVM;
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
//! - `UnitType` has exactly one canonical representation;
//! - construction always produces `TypeExpr::Unit`;
//! - conversion from arbitrary `TypeExpr` checks the variant;
//! - invalid variants are rejected without panic;
//! - structural validation delegates to `TypeExpr`;
//! - source formatting delegates to `TypeExpr`;
//! - inference information delegates to `TypeExpr`;
//! - serialization has no competing schema;
//! - deserialization preserves the unit invariant;
//! - equality/hash/debug behavior is deterministic;
//! - no machine-specific information exists;
//! - no hidden scalability limit exists;
//! - no unsafe code exists;
//! - parser integration is explicitly defined;
//! - semantic integration is explicitly defined;
//! - ZUIR integration is explicitly defined;
//! - quantum integration remains domain-neutral;
//! - tests cover all public behavior.
//!
//! # Migration contract
//!
//! Existing code using:
//!
//! ```rust
//! TypeExpr::Unit
//! ```
//!
//! remains canonical.
//!
//! Introducing `UnitType` does not require unrelated AST components to migrate.
//!
//! Consumers may incrementally adopt:
//!
//! ```text
//! TypeExpr::Unit
//!     ↕
//! UnitType
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
// UnitType
// =============================================================================

/// Strongly typed façade over the canonical [`TypeExpr::Unit`] representation.
///
/// `UnitType` is deliberately not `Copy`. Although `TypeExpr::Unit` has no
/// payload, the complete `TypeExpr` enum is not a `Copy` type because other
/// variants contain owned recursive structures. Keeping the façade aligned
/// with the canonical representation avoids introducing an incompatible
/// trait contract.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct UnitType(TypeExpr);

impl UnitType {
    /// Constructs the canonical unit type.
    ///
    /// Construction cannot fail because this constructor directly establishes
    /// the representation invariant.
    #[must_use]
    pub fn new() -> Self {
        Self(TypeExpr::Unit)
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

    /// Returns an owned canonical [`TypeExpr`].
    ///
    /// Since `TypeExpr::Unit` contains no payload, this does not clone a
    /// subtree or allocate.
    #[must_use]
    pub fn to_type_expr(&self) -> TypeExpr {
        TypeExpr::Unit
    }

    /// Returns the canonical source-level spelling.
    ///
    /// Formatting is delegated to `TypeExpr` so this façade cannot establish a
    /// competing source representation.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.0.to_source_string()
    }

    /// Performs canonical structural validation using the default policy.
    ///
    /// Unit has no child structure, so validation is delegated to the
    /// authoritative `TypeExpr` validator.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.0.validate()
    }

    /// Performs canonical structural validation using an explicit compiler
    /// validation/resource policy.
    ///
    /// The policy is supplied by the compiler and does not become part of the
    /// AST representation.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.0.validate_with_policy(policy)
    }

    /// Attempts to create a `UnitType` from an existing [`TypeExpr`].
    ///
    /// Only `TypeExpr::Unit` is accepted.
    pub fn try_from_type_expr(value: TypeExpr) -> Result<Self, UnitTypeError> {
        match value {
            TypeExpr::Unit => Ok(Self(TypeExpr::Unit)),
            other => Err(UnitTypeError::NotUnit {
                variant: other.variant_name(),
            }),
        }
    }

    /// Attempts to create a `UnitType` from an existing [`TypeExpr`] and
    /// validates it with an explicit compiler policy.
    pub fn try_from_type_expr_with_policy(
        value: TypeExpr,
        policy: &TypeValidationPolicy,
    ) -> Result<Self, UnitTypeError> {
        let unit = Self::try_from_type_expr(value)?;

        unit.validate_with_policy(policy)
            .map_err(UnitTypeError::InvalidType)?;

        Ok(unit)
    }

    /// Returns the stable source-level variant name.
    ///
    /// This describes the AST variant, not a semantic type identity.
    #[must_use]
    pub const fn variant_name(&self) -> &'static str {
        "unit"
    }

    /// Returns `true` because this façade always represents unit.
    #[must_use]
    pub const fn is_unit(&self) -> bool {
        true
    }

    /// Returns `false` because unit itself contains no unresolved inference
    /// placeholder.
    ///
    /// This is structural information only. It does not perform type
    /// inference.
    #[must_use]
    pub const fn requires_inference(&self) -> bool {
        false
    }
}

impl Default for UnitType {
    fn default() -> Self {
        Self::new()
    }
}

impl From<UnitType> for TypeExpr {
    fn from(value: UnitType) -> Self {
        value.into_type_expr()
    }
}

impl TryFrom<TypeExpr> for UnitType {
    type Error = UnitTypeError;

    fn try_from(value: TypeExpr) -> Result<Self, Self::Error> {
        Self::try_from_type_expr(value)
    }
}

impl fmt::Display for UnitType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

// =============================================================================
// Error
// =============================================================================

/// Errors produced when a canonical [`TypeExpr`] cannot be represented as a
/// [`UnitType`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnitTypeError {
    /// The supplied expression was a different type-expression variant.
    NotUnit {
        /// Canonical name of the supplied AST variant.
        variant: &'static str,
    },

    /// The supplied expression was `Unit` but failed canonical structural
    /// validation under the requested compiler policy.
    InvalidType(TypeExprError),
}

impl fmt::Display for UnitTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotUnit { variant } => {
                write!(
                    formatter,
                    "expected unit type expression, found `{variant}`"
                )
            }
            Self::InvalidType(error) => {
                write!(formatter, "invalid unit type expression: {error}")
            }
        }
    }
}

impl std::error::Error for UnitTypeError {}

// =============================================================================
// Serde
// =============================================================================

impl<'de> Deserialize<'de> for UnitType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = TypeExpr::deserialize(deserializer)?;

        Self::try_from_type_expr(value).map_err(D::Error::custom)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_canonical_unit() {
        let unit = UnitType::new();

        assert!(matches!(unit.as_type_expr(), TypeExpr::Unit));
        assert!(unit.is_unit());
        assert!(!unit.requires_inference());
    }

    #[test]
    fn default_creates_canonical_unit() {
        let unit = UnitType::default();

        assert!(matches!(unit.as_type_expr(), TypeExpr::Unit));
    }

    #[test]
    fn into_type_expr_returns_canonical_unit() {
        let expression = UnitType::new().into_type_expr();

        assert!(matches!(expression, TypeExpr::Unit));
    }

    #[test]
    fn to_type_expr_returns_canonical_unit() {
        let unit = UnitType::new();

        assert!(matches!(unit.to_type_expr(), TypeExpr::Unit));
    }

    #[test]
    fn try_from_type_expr_accepts_unit() {
        let result = UnitType::try_from_type_expr(TypeExpr::Unit);

        assert!(result.is_ok());
    }

    #[test]
    fn try_from_type_expr_rejects_non_unit() {
        let result = UnitType::try_from_type_expr(TypeExpr::Never);

        assert!(matches!(
            result,
            Err(UnitTypeError::NotUnit { variant: "never" })
        ));
    }

    #[test]
    fn conversion_trait_accepts_unit() {
        let result = UnitType::try_from(TypeExpr::Unit);

        assert!(result.is_ok());
    }

    #[test]
    fn conversion_trait_rejects_non_unit() {
        let result = UnitType::try_from(TypeExpr::Never);

        assert!(result.is_err());
    }

    #[test]
    fn source_formatting_delegates_to_type_expr() {
        let unit = UnitType::new();

        assert_eq!(unit.to_source_string(), unit.as_type_expr().to_source_string());
        assert_eq!(unit.to_string(), unit.to_source_string());
    }

    #[test]
    fn variant_name_is_stable() {
        assert_eq!(UnitType::new().variant_name(), "unit");
    }

    #[test]
    fn validation_delegates_to_type_expr() {
        let unit = UnitType::new();

        assert!(unit.validate().is_ok());
    }

    #[test]
    fn validation_with_policy_delegates_to_type_expr() {
        let unit = UnitType::new();
        let policy = TypeValidationPolicy::default();

        assert!(unit.validate_with_policy(&policy).is_ok());
    }

    #[test]
    fn unit_has_no_inference_placeholder() {
        assert!(!UnitType::new().requires_inference());
    }

    #[test]
    fn unit_is_deterministic() {
        let first = UnitType::new();
        let second = UnitType::new();

        assert_eq!(first, second);
        assert_eq!(first.to_source_string(), second.to_source_string());
        assert_eq!(first.as_type_expr(), second.as_type_expr());
    }

    #[test]
    fn clone_preserves_canonical_representation() {
        let original = UnitType::new();
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert!(matches!(cloned.as_type_expr(), TypeExpr::Unit));
    }

    #[test]
    fn from_unit_type_returns_canonical_variant() {
        let expression: TypeExpr = UnitType::new().into();

        assert!(matches!(expression, TypeExpr::Unit));
    }

    #[test]
    fn unit_type_error_is_displayable() {
        let error = UnitTypeError::NotUnit {
            variant: "never",
        };

        assert_eq!(
            error.to_string(),
            "expected unit type expression, found `never`"
        );
    }

    #[cfg(feature = "serde_json")]
    #[test]
    fn serde_round_trip_preserves_unit() {
        let original = UnitType::new();

        let encoded = serde_json::to_string(&original)
            .expect("serializing UnitType should succeed");

        let decoded: UnitType = serde_json::from_str(&encoded)
            .expect("deserializing UnitType should succeed");

        assert_eq!(original, decoded);
        assert!(matches!(decoded.as_type_expr(), TypeExpr::Unit));
    }
}