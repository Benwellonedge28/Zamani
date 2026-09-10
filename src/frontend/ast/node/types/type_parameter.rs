//! Zamani Frontend AST — Type Parameters
//!
//! Production-ready source-level representation of a generic type parameter.
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
//! TypeParameter
//!     │
//!     ├── TypeParameterName
//!     └── zero or more TypeExpr bounds
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! generic/type semantic model
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! # Purpose
//!
//! `TypeParameter` represents one source-level generic type parameter.
//!
//! Examples:
//!
//! ```text
//! T
//! T extends Numeric
//! Q extends QuantumResource
//! T extends Foo + Bar
//! ```
//!
//! The type parameter records programmer intent only. It does not resolve the
//! parameter, determine its concrete type, allocate resources, select a target,
//! or perform generic specialization.
//!
//! # Canonical representation
//!
//! A type parameter consists of:
//!
//! - a [`TypeParameterName`];
//! - an ordered collection of zero or more [`TypeExpr`] bounds.
//!
//! The canonical generic-parameter reference inside a type expression remains:
//!
//! ```text
//! TypeExpr::GenericParameter(TypeParameterName)
//! ```
//!
//! This file therefore does not create a second representation for references
//! to generic parameters.
//!
//! # Architectural separation
//!
//! This module does NOT perform:
//!
//! - name resolution;
//! - generic substitution;
//! - type inference;
//! - unification;
//! - trait resolution;
//! - overload resolution;
//! - specialization;
//! - monomorphization;
//! - type layout;
//! - ownership analysis;
//! - borrow checking;
//! - resource allocation;
//! - quantum resource allocation;
//! - hardware mapping;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - backend selection;
//! - QIR lowering;
//! - LLVM lowering;
//! - MLIR lowering.
//!
//! Those responsibilities belong to later compiler layers.
//!
//! # POCO-REAF
//!
//! A type parameter introduces no machine-size assumption.
//!
//! It does not encode:
//!
//! - machine width;
//! - memory capacity;
//! - CPU count;
//! - GPU count;
//! - FPGA capacity;
//! - qubit count;
//! - quantum topology;
//! - vendor;
//! - backend;
//! - instruction set;
//! - physical resource count.
//!
//! Consequently the representation is independent of computational scale.
//!
//! A generic algorithm can use a type parameter to describe an abstraction
//! whose eventual realization may range from a tiny system to a substantially
//! larger system, subject to compiler and target capabilities.
//!
//! # Bounds
//!
//! Bounds are represented as ordinary source-level [`TypeExpr`] values.
//!
//! This is intentional:
//!
//! ```text
//! TypeParameter
//!     └── Vec<TypeExpr>
//! ```
//!
//! rather than coupling this file to a closed enumeration of bound kinds.
//!
//! A bound may therefore evolve with the language type system without requiring
//! this file to know about every future computational domain.
//!
//! Semantic interpretation of a bound belongs to semantic analysis.
//!
//! # Domain neutrality
//!
//! A bound may eventually describe a capability relevant to:
//!
//! - classical computation;
//! - quantum computation;
//! - hybrid computation;
//! - distributed computation;
//! - accelerators;
//! - HDL;
//! - AI/ML;
//! - future computational domains.
//!
//! This type does not interpret any bound as hardware-specific.
//!
//! # Dependency contract
//!
//! Allowed dependencies:
//!
//! - Rust standard library;
//! - `serde`;
//! - sibling `type_expr` module.
//!
//! Forbidden dependencies:
//!
//! - legacy `crate::ast`;
//! - lexer implementation;
//! - parser implementation;
//! - compiler driver;
//! - semantic implementation;
//! - ZUIR implementation;
//! - quantum IR;
//! - hardware IR;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - ZQN;
//! - runtime;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor SDKs.
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
//! # Source compatibility
//!
//! The legacy AST currently contains a `TypeParameter` abstraction with a name
//! and bounds. The new frontend representation deliberately does not depend on
//! the legacy `Identifier` or `TypeBound` types.
//!
//! Migration is therefore:
//!
//! ```text
//! legacy Identifier
//!        │
//!        ▼
//! TypeParameterName
//!
//! legacy TypeBound
//!        │
//!        ▼
//! canonical source-level TypeExpr
//! ```
//!
//! The migration adapter belongs in the parser/legacy-compatibility layer,
//! rather than in this file.
//!
//! # Serialization
//!
//! Serialization is deterministic because:
//!
//! - the parameter has a single name;
//! - bounds are stored in source order using `Vec`;
//! - no hash-map iteration is involved;
//! - no target state is serialized;
//! - no memory addresses are serialized;
//! - no timestamps or random values are serialized.
//!
//! `serde` serialization is structural. Semantic validation remains a separate
//! operation so malformed serialized structures can be detected explicitly.
//!
//! # Validation
//!
//! Structural validation verifies:
//!
//! - the parameter name is non-empty;
//! - the parameter name satisfies the configured identifier policy;
//! - the number of bounds satisfies the configured collection policy;
//! - every bound is structurally valid according to the canonical `TypeExpr`
//!   validator.
//!
//! This module does NOT determine whether a bound is semantically legal.
//!
//! For example, whether:
//!
//! ```text
//! T extends Foo
//! ```
//!
//! is valid depends on whether `Foo` is a valid bound in the semantic type
//! system. That decision belongs downstream.
//!
//! # Scalability
//!
//! There is no language-level maximum number of bounds.
//!
//! The collection is represented by `Vec<TypeExpr>` and may grow according to
//! available compiler resources.
//!
//! Any protection against pathological input is supplied through the existing
//! [`TypeValidationPolicy`] rather than a hidden constant in this file.
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_TYPE_PARAMETERS
//! MAX_BOUNDS
//! MAX_GENERIC_ARITY
//! MAX_QUBITS
//! MAX_MACHINE_SIZE
//! ```
//!
//! # Determinism
//!
//! Bound order is preserved exactly.
//!
//! This matters because source order may be relevant to diagnostics and must
//! not be silently replaced by unordered storage.
//!
//! Equality and hashing are structural and deterministic.
//!
//! # Security
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no filesystem access;
//! - performs no network access;
//! - executes no source code;
//! - contains no raw pointer operations;
//! - contains no `unsafe`;
//! - does not access global mutable state.
//!
//! Validation of nested type expressions is delegated to the canonical
//! `TypeExpr` validator, which is iterative and therefore avoids recursive
//! stack growth during structural validation.
//!
//! # Parser integration
//!
//! The parser should construct:
//!
//! ```text
//! parser
//!   └── TypeParameter::new(...)
//! ```
//!
//! or:
//!
//! ```text
//! parser
//!   └── TypeParameter::with_bounds(...)
//! ```
//!
//! Parser syntax/recovery remains outside this file.
//!
//! The parser must preserve the source order of bounds.
//!
//! # Semantic integration
//!
//! Semantic analysis consumes validated `TypeParameter` values and is
//! responsible for:
//!
//! - introducing the generic parameter into the current generic scope;
//! - checking duplicate parameter names;
//! - resolving bound names;
//! - validating bound relationships;
//! - constructing semantic generic constraints;
//! - checking recursive/contradictory constraints;
//! - determining whether the parameter is satisfiable;
//! - applying substitutions;
//! - checking instantiations.
//!
//! None of these operations belong to the AST.
//!
//! # ZUIR integration
//!
//! `TypeParameter` does not depend directly on ZUIR.
//!
//! The intended boundary is:
//!
//! ```text
//! TypeParameter
//!     │
//!     ▼
//! semantic generic parameter
//!     │
//!     ▼
//! resolved semantic type/constraint
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! Generic parameters that survive into ZUIR must be represented according to
//! ZUIR's own semantic model. This file must not predict that representation.
//!
//! # Visitor integration
//!
//! A type parameter has:
//!
//! - one leaf name;
//! - zero or more type-expression bounds.
//!
//! Visitors should therefore visit every bound in source order.
//!
//! This file provides `bounds()` and `into_parts()` so visitor implementations
//! can traverse it without accessing private representation.
//!
//! # Traversal integration
//!
//! There is no recursive traversal implementation here. The central AST
//! visitor/traversal layer should handle traversal policy.
//!
//! A caller can obtain the ordered bounds through [`Self::bounds`].
//!
//! # Independent-file completion contract
//!
//! This file is complete when:
//!
//! - `TypeParameter` has exactly one canonical representation;
//! - its name uses `TypeParameterName`;
//! - bounds use canonical `TypeExpr`;
//! - no legacy AST type is imported;
//! - construction is deterministic;
//! - malformed structures can be validated without panic;
//! - validation delegates nested validation to `TypeExpr`;
//! - serialization is structural and deterministic;
//! - visitor traversal can enumerate all bounds;
//! - parser integration is defined;
//! - semantic integration is defined;
//! - ZUIR integration is defined;
//! - no machine-specific information exists;
//! - no hidden scalability limit exists;
//! - no `unsafe` code exists.
//!
//! # =============================================================================
//! # Implementation
//! # =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::type_expr::{
    TypeExpr,
    TypeExprError,
    TypeParameterName,
    TypeValidationPolicy,
};

/// A source-level generic type parameter.
///
/// # Representation invariant
///
/// A successfully validated value has:
///
/// - a non-empty parameter name;
/// - zero or more structurally valid type-expression bounds.
///
/// Duplicate parameters within the same generic declaration are intentionally
/// not rejected here because that requires declaration-scope information.
/// Semantic analysis owns duplicate-name checking.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeParameter {
    /// Source-level generic parameter name.
    name: TypeParameterName,

    /// Ordered source-level bounds.
    ///
    /// Bounds are stored as `TypeExpr` so this file remains independent from a
    /// closed set of bound kinds and from future domain-specific extensions.
    bounds: Vec<TypeExpr>,
}

impl TypeParameter {
    /// Creates an unbounded type parameter.
    ///
    /// The constructor establishes a structurally representable value.
    /// Call [`Self::validate`] before treating externally supplied data as a
    /// fully valid AST node.
    #[must_use]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: TypeParameterName::new(name),
            bounds: Vec::new(),
        }
    }

    /// Creates a type parameter with ordered bounds.
    #[must_use]
    pub fn with_bounds<S>(name: S, bounds: Vec<TypeExpr>) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: TypeParameterName::new(name),
            bounds,
        }
    }

    /// Creates a type parameter from an existing canonical parameter name.
    #[must_use]
    pub fn from_name(name: TypeParameterName) -> Self {
        Self {
            name,
            bounds: Vec::new(),
        }
    }

    /// Creates a type parameter from a canonical parameter name and bounds.
    #[must_use]
    pub fn from_parts(name: TypeParameterName, bounds: Vec<TypeExpr>) -> Self {
        Self { name, bounds }
    }

    /// Returns the source-level parameter name.
    #[must_use]
    pub fn name(&self) -> &TypeParameterName {
        &self.name
    }

    /// Returns the textual parameter name.
    #[must_use]
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the ordered source-level bounds.
    #[must_use]
    pub fn bounds(&self) -> &[TypeExpr] {
        &self.bounds
    }

    /// Returns the number of bounds.
    #[must_use]
    pub fn bound_count(&self) -> usize {
        self.bounds.len()
    }

    /// Returns whether the parameter has no bounds.
    #[must_use]
    pub fn is_unbounded(&self) -> bool {
        self.bounds.is_empty()
    }

    /// Returns whether the parameter has one or more bounds.
    #[must_use]
    pub fn is_bounded(&self) -> bool {
        !self.bounds.is_empty()
    }

    /// Adds one source-level bound.
    ///
    /// No semantic interpretation is performed.
    pub fn push_bound(&mut self, bound: TypeExpr) {
        self.bounds.push(bound);
    }

    /// Extends the parameter with ordered source-level bounds.
    pub fn extend_bounds<I>(&mut self, bounds: I)
    where
        I: IntoIterator<Item = TypeExpr>,
    {
        self.bounds.extend(bounds);
    }

    /// Returns an owned `(name, bounds)` pair.
    ///
    /// This is useful to declarations and lowering adapters that need ownership
    /// without cloning potentially large bound structures.
    #[must_use]
    pub fn into_parts(self) -> (TypeParameterName, Vec<TypeExpr>) {
        (self.name, self.bounds)
    }

    /// Converts this parameter reference into the canonical type-expression
    /// representation.
    ///
    /// Bounds are declaration constraints and therefore are intentionally not
    /// embedded in the resulting `TypeExpr`.
    ///
    /// ```text
    /// TypeParameter<T: ...>
    ///          │
    ///          ▼
    /// TypeExpr::GenericParameter(T)
    /// ```
    #[must_use]
    pub fn to_type_expr(&self) -> TypeExpr {
        TypeExpr::GenericParameter(self.name.clone())
    }

    /// Returns the canonical type-expression representation for this
    /// parameter.
    ///
    /// This is equivalent to [`Self::to_type_expr`].
    #[must_use]
    pub fn as_type_expr(&self) -> TypeExpr {
        self.to_type_expr()
    }

    /// Returns the canonical stable source-AST category name.
    #[must_use]
    pub const fn variant_name(&self) -> &'static str {
        "type_parameter"
    }

    /// Performs structural validation with the default validation policy.
    ///
    /// The default policy imposes no artificial collection or identifier
    /// limits.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.validate_with_policy(&TypeValidationPolicy::default())
    }

    /// Performs structural validation using an explicit compiler policy.
    ///
    /// Policy limits are security/resource controls and are never part of
    /// Zamani's language semantics.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        validate_parameter_name(&self.name, policy)?;

        if let Some(maximum) = policy.max_collection_items {
            if self.bounds.len() > maximum {
                return Err(TypeExprError::CollectionTooLarge {
                    actual: self.bounds.len(),
                    maximum,
                });
            }
        }

        for bound in &self.bounds {
            bound.validate_with_policy(policy)?;
        }

        Ok(())
    }

    /// Returns whether the parameter itself contains a structurally valid
    /// non-empty name.
    ///
    /// This is intentionally narrower than [`Self::validate`], because it does
    /// not validate nested bounds.
    #[must_use]
    pub fn has_valid_name(&self) -> bool {
        !self.name.as_str().is_empty()
    }

    /// Returns whether any bound contains a source-level inference placeholder.
    ///
    /// This is structural information only. It does not perform inference.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.bounds.iter().any(TypeExpr::requires_inference)
    }

    /// Returns whether any bound is itself source-level generic.
    ///
    /// This is a structural predicate and does not resolve generic scopes.
    #[must_use]
    pub fn has_generic_bound(&self) -> bool {
        self.bounds.iter().any(TypeExpr::is_generic)
    }

    /// Returns a deterministic source-level representation.
    ///
    /// Examples:
    ///
    /// ```text
    /// T
    /// T extends Numeric
    /// T extends Numeric + Ordered
    /// ```
    ///
    /// The exact semantic interpretation of `extends` belongs to the parser
    /// and semantic layers; this method is only a deterministic structural
    /// representation.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        if self.bounds.is_empty() {
            return self.name.to_string();
        }

        let bounds = self
            .bounds
            .iter()
            .map(TypeExpr::to_source_string)
            .collect::<Vec<_>>()
            .join(" + ");

        format!("{} extends {}", self.name, bounds)
    }
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

impl From<TypeParameterName> for TypeParameter {
    fn from(name: TypeParameterName) -> Self {
        Self::from_name(name)
    }
}

impl From<TypeParameter> for TypeExpr {
    fn from(parameter: TypeParameter) -> Self {
        TypeExpr::GenericParameter(parameter.name)
    }
}

impl From<&TypeParameter> for TypeExpr {
    fn from(parameter: &TypeParameter) -> Self {
        parameter.to_type_expr()
    }
}

/// Validates the parameter name using the canonical `TypeExpr` validation
/// policy.
///
/// `TypeParameterName` intentionally exposes its textual representation rather
/// than its internal storage, keeping the validation contract independent of
/// its memory representation.
fn validate_parameter_name(
    name: &TypeParameterName,
    policy: &TypeValidationPolicy,
) -> Result<(), TypeExprError> {
    let value = name.as_str();

    if value.is_empty() {
        return Err(TypeExprError::EmptyName);
    }

    if let Some(maximum) = policy.max_identifier_bytes {
        if value.len() > maximum {
            return Err(TypeExprError::IdentifierTooLarge {
                actual: value.len(),
                maximum,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parameter(name: &str) -> TypeParameter {
        TypeParameter::new(name)
    }

    #[test]
    fn constructs_unbounded_parameter() {
        let parameter = parameter("T");

        assert_eq!(parameter.name_str(), "T");
        assert_eq!(parameter.bound_count(), 0);
        assert!(parameter.is_unbounded());
        assert!(!parameter.is_bounded());
    }

    #[test]
    fn constructs_bounded_parameter() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![
                TypeExpr::name("Numeric"),
                TypeExpr::name("Ordered"),
            ],
        );

        assert_eq!(parameter.name_str(), "T");
        assert_eq!(parameter.bound_count(), 2);
        assert!(parameter.is_bounded());
        assert_eq!(parameter.bounds()[0].to_source_string(), "Numeric");
        assert_eq!(parameter.bounds()[1].to_source_string(), "Ordered");
    }

    #[test]
    fn preserves_bound_order() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![
                TypeExpr::name("First"),
                TypeExpr::name("Second"),
                TypeExpr::name("Third"),
            ],
        );

        let names = parameter
            .bounds()
            .iter()
            .map(TypeExpr::to_source_string)
            .collect::<Vec<_>>();

        assert_eq!(names, vec!["First", "Second", "Third"]);
    }

    #[test]
    fn converts_to_canonical_generic_parameter_expression() {
        let parameter = parameter("T");

        assert_eq!(
            parameter.to_type_expr(),
            TypeExpr::GenericParameter(TypeParameterName::new("T"))
        );
    }

    #[test]
    fn conversion_discards_declaration_bounds_from_type_reference() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::name("Numeric")],
        );

        assert_eq!(
            TypeExpr::from(&parameter),
            TypeExpr::GenericParameter(TypeParameterName::new("T"))
        );
    }

    #[test]
    fn validates_valid_parameter() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::name("Numeric")],
        );

        assert!(parameter.validate().is_ok());
    }

    #[test]
    fn rejects_empty_parameter_name() {
        let parameter = TypeParameter::new("");

        assert_eq!(
            parameter.validate(),
            Err(TypeExprError::EmptyName)
        );
    }

    #[test]
    fn rejects_empty_bound_name() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::name("")],
        );

        assert_eq!(
            parameter.validate(),
            Err(TypeExprError::EmptyName)
        );
    }

    #[test]
    fn respects_identifier_policy() {
        let parameter = TypeParameter::new("LongParameterName");

        let policy = TypeValidationPolicy {
            max_identifier_bytes: Some(4),
            ..TypeValidationPolicy::default()
        };

        assert_eq!(
            parameter.validate_with_policy(&policy),
            Err(TypeExprError::IdentifierTooLarge {
                actual: "LongParameterName".len(),
                maximum: 4,
            })
        );
    }

    #[test]
    fn respects_bound_collection_policy() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![
                TypeExpr::name("A"),
                TypeExpr::name("B"),
            ],
        );

        let policy = TypeValidationPolicy {
            max_collection_items: Some(1),
            ..TypeValidationPolicy::default()
        };

        assert_eq!(
            parameter.validate_with_policy(&policy),
            Err(TypeExprError::CollectionTooLarge {
                actual: 2,
                maximum: 1,
            })
        );
    }

    #[test]
    fn nested_bounds_are_validated() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::generic(
                TypeExpr::name("Container"),
                vec![TypeExpr::name("")],
            )],
        );

        assert_eq!(
            parameter.validate(),
            Err(TypeExprError::EmptyName)
        );
    }

    #[test]
    fn inference_is_detected_only_from_bounds() {
        let unbounded = TypeParameter::new("T");
        assert!(!unbounded.requires_inference());

        let bounded = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::Infer],
        );

        assert!(bounded.requires_inference());
    }

    #[test]
    fn generic_bound_is_detected_structurally() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::generic(
                TypeExpr::name("Container"),
                vec![TypeExpr::name("U")],
            )],
        );

        assert!(parameter.has_generic_bound());
    }

    #[test]
    fn source_formatting_is_deterministic() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![
                TypeExpr::name("Numeric"),
                TypeExpr::name("Ordered"),
            ],
        );

        assert_eq!(
            parameter.to_source_string(),
            "T extends Numeric + Ordered"
        );
        assert_eq!(
            parameter.to_string(),
            "T extends Numeric + Ordered"
        );
    }

    #[test]
    fn into_parts_preserves_structure() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::name("Numeric")],
        );

        let (name, bounds) = parameter.into_parts();

        assert_eq!(name.as_str(), "T");
        assert_eq!(bounds.len(), 1);
    }

    #[test]
    fn equality_and_hashing_are_structural() {
        use std::collections::HashSet;

        let first = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::name("Numeric")],
        );

        let second = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::name("Numeric")],
        );

        assert_eq!(first, second);

        let mut set = HashSet::new();
        set.insert(first);

        assert!(set.contains(&second));
    }

    #[test]
    fn no_machine_specific_information_is_represented() {
        let parameter = TypeParameter::with_bounds(
            "Q",
            vec![TypeExpr::name("QuantumResource")],
        );

        assert_eq!(parameter.name_str(), "Q");
        assert_eq!(
            parameter.to_type_expr(),
            TypeExpr::GenericParameter(TypeParameterName::new("Q"))
        );
    }

    #[test]
    fn deeply_nested_bounds_delegate_to_type_expr_validation() {
        let mut current = TypeExpr::name("Leaf");

        for _ in 0..256 {
            current = TypeExpr::optional(current);
        }

        let parameter = TypeParameter::with_bounds("T", vec![current]);

        assert!(parameter.validate().is_ok());
    }
}