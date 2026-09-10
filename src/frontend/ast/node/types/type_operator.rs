//! Zamani Frontend AST — Type Operators
//!
//! Production-ready source-level representation of type operators.
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
//! TypeExpr / TypeOperatorExpr
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic type system
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical
//!     ├── quantum
//!     ├── hybrid
//!     ├── distributed
//!     ├── accelerator
//!     └── future domains
//! ```
//!
//! # Purpose
//!
//! This module owns the source-level representation of operations that combine
//! or transform type expressions.
//!
//! Examples of possible type-level operators include:
//!
//! ```text
//! A | B
//! A & B
//! !A
//! A -> B
//! A × B
//! A[N]
//! ```
//!
//! The exact semantic meaning of an operator is deliberately NOT decided by
//! this module.
//!
//! Type operators may eventually participate in:
//!
//! - union types;
//! - intersection types;
//! - negation/complement types;
//! - function/arrow types;
//! - product types;
//! - dependent/resource types;
//! - capability types;
//! - symbolic cardinality expressions;
//! - future type-system extensions.
//!
//! This module represents source-level intent. Semantic analysis determines
//! whether a particular operator is legal and what it means.
//!
//! # Critical architectural rule
//!
//! `TypeOperator` is intentionally an extensible identifier rather than a
//! closed enum containing every possible type operator.
//!
//! Do NOT implement:
//!
//! ```text
//! enum TypeOperator {
//!     Union,
//!     Intersection,
//!     Function,
//!     ...
//! }
//! ```
//!
//! as the fundamental representation.
//!
//! A permanently closed enum would require modification of this core file
//! whenever Zamani adds a new type-system feature or computational domain.
//!
//! Instead, operators are represented by a stable namespaced textual identity.
//!
//! This follows the same extensibility principle used throughout the new
//! frontend AST: the AST represents syntax and intent; semantic analysis owns
//! interpretation.
//!
//! # Domain neutrality
//!
//! This module contains no knowledge of:
//!
//! - quantum gates;
//! - qubits;
//! - quantum topology;
//! - physical resources;
//! - QEC;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulses;
//! - hardware;
//! - CPU instructions;
//! - GPU instructions;
//! - FPGA instructions;
//! - ASIC instructions;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor APIs;
//! - runtime implementation.
//!
//! A type operator may be used to describe a quantum-related type, but the
//! operator itself remains a language-level construct.
//!
//! # POCO-REAF
//!
//! No operator contains:
//!
//! ```text
//! MAX_QUBITS
//! MAX_MACHINE_SIZE
//! MAX_REGISTER_SIZE
//! MAX_GENERIC_ARITY
//! MAX_TYPE_DEPTH
//! ```
//!
//! Type-expression arity and nesting are represented by dynamically sized
//! structures and are limited only by explicitly supplied compiler resource
//! policies.
//!
//! This means that source-level type expressions can remain independent of the
//! eventual computational scale.
//!
//! # Canonical TypeExpr boundary
//!
//! `type_expr.rs` is the authoritative owner of the canonical `TypeExpr`
//! representation. This module MUST NOT create a second competing definition
//! of `TypeExpr`.
//!
//! A type operator is therefore represented independently here until the
//! canonical `TypeExpr` schema explicitly gains a corresponding variant.
//!
//! The intended future integration, if the language grammar requires operators
//! to be first-class type expressions, is:
//!
//! ```text
//! TypeExpr::Operator {
//!     operator: TypeOperator,
//!     operands: Vec<TypeExpr>,
//! }
//! ```
//!
//! That change must be made deliberately in `type_expr.rs` together with the
//! parser, validation, serialization, visitors and semantic lowering.
//!
//! This file must not silently invent such a variant.
//!
//! # Semantic boundary
//!
//! This module does NOT perform:
//!
//! - type resolution;
//! - type inference;
//! - unification;
//! - constraint solving;
//! - overload resolution;
//! - trait resolution;
//! - subtype checking;
//! - generic substitution;
//! - type normalization;
//! - ownership analysis;
//! - resource allocation;
//! - quantum legality checking;
//! - hardware capability checking.
//!
//! Those operations belong to semantic analysis and later compiler stages.
//!
//! # Serialization
//!
//! `TypeOperator` is serializable as its stable source-level identity.
//!
//! Serialization must preserve:
//!
//! - namespace;
//! - operator name;
//! - source spelling;
//! - operands where represented by `TypeOperatorExpr`.
//!
//! Semantic interpretation must never be inferred merely from serialized
//! operator text.
//!
//! # Determinism
//!
//! Determinism is guaranteed by:
//!
//! - preserving operand order;
//! - preserving operator namespace/name;
//! - avoiding unordered collections;
//! - avoiding timestamps;
//! - avoiding random IDs;
//! - avoiding memory addresses.
//!
//! For example:
//!
//! ```text
//! A | B
//! ```
//!
//! and:
//!
//! ```text
//! B | A
//! ```
//!
//! remain structurally distinct at the AST level unless a later semantic phase
//! explicitly establishes commutativity.
//!
//! # Security
//!
//! Operator names are untrusted source data.
//!
//! This module performs no:
//!
//! - filesystem access;
//! - network access;
//! - code execution;
//! - dynamic library loading;
//! - backend calls;
//! - unsafe operations.
//!
//! Resource limits, if required for hostile input, must be supplied by the
//! caller through compiler validation policies.
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
//! # Dependency contract
//!
//! Allowed dependencies:
//!
//! - Rust standard library;
//! - `serde`;
//! - canonical `TypeExpr`.
//!
//! Forbidden dependencies:
//!
//! - legacy `crate::ast`;
//! - semantic implementation;
//! - compiler driver;
//! - ZUIR implementation;
//! - quantum IR;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - hardware;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - ZQN;
//! - runtime;
//! - vendor SDKs.
//!
//! # Integration contract
//!
//! ## Parser
//!
//! The parser constructs `TypeOperator` from source syntax and preserves the
//! operator's source spelling and namespace.
//!
//! Parser recovery may construct a syntactically representable operator and
//! defer structural validity to AST validation.
//!
//! ## AST validation
//!
//! Validation verifies structural properties such as:
//!
//! - non-empty operator name;
//! - valid namespace components;
//! - valid source spelling where required;
//! - operand arity when an explicit language-level arity is defined.
//!
//! Semantic legality is not checked here.
//!
//! ## Semantic analysis
//!
//! Semantic analysis resolves the operator identity and determines:
//!
//! - whether the operator exists;
//! - its arity;
//! - operand compatibility;
//! - resulting type;
//! - generic constraints;
//! - capability requirements;
//! - domain-specific semantics.
//!
//! ## ZUIR
//!
//! A type operator reaches ZUIR only through the semantic representation.
//!
//! This module must never import ZUIR merely to provide a lowering shortcut.
//!
//! ## Visitors
//!
//! `TypeOperatorExpr` exposes ordered operands so the central AST visitor can
//! visit every child deterministically.
//!
//! ## Serialization
//!
//! Serialization is structural and source-oriented.
//!
//! The canonical AST serializer should own the final schema/version envelope.
//!
//! # Independent-file completion
//!
//! This file is complete when:
//!
//! - operator identity is extensible;
//! - source spelling is preserved;
//! - namespace is preserved;
//! - no closed finite operator list is required;
//! - no machine limits are embedded;
//! - no domain-specific implementation is embedded;
//! - operand ordering is deterministic;
//! - structural validation is available;
//! - serialization is available;
//! - parser integration is defined;
//! - semantic integration is defined;
//! - ZUIR integration is explicitly downstream;
//! - no unsafe code exists;
//! - no legacy AST dependency exists.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::type_expr::TypeExpr;

/// Stable source-level namespace/name identity for a type operator.
///
/// Examples:
///
/// ```text
/// core::union
/// core::intersection
/// core::function
/// quantum::tensor
/// future_domain::operator
/// ```
///
/// The AST does not assign semantic meaning to these names.
///
/// Semantic analysis resolves them against the language's registered type
/// operator environment.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TypeOperator {
    namespace: Vec<String>,
    name: String,
}

impl TypeOperator {
    /// Creates an operator in the root namespace.
    ///
    /// The constructor intentionally permits an empty name so parser recovery
    /// can preserve malformed source. Structural validation rejects it.
    #[must_use]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            namespace: Vec::new(),
            name: name.into(),
        }
    }

    /// Creates a namespaced operator.
    #[must_use]
    pub fn namespaced<I, S, N>(namespace: I, name: N) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
        N: Into<String>,
    {
        Self {
            namespace: namespace.into_iter().map(Into::into).collect(),
            name: name.into(),
        }
    }

    /// Returns the ordered namespace components.
    #[must_use]
    pub fn namespace(&self) -> &[String] {
        &self.namespace
    }

    /// Returns the operator's unqualified name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the operator has no namespace.
    #[must_use]
    pub fn is_unqualified(&self) -> bool {
        self.namespace.is_empty()
    }

    /// Returns whether the operator name is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.name.is_empty()
    }

    /// Returns the number of namespace components.
    #[must_use]
    pub fn namespace_depth(&self) -> usize {
        self.namespace.len()
    }

    /// Returns the complete source-level operator identity.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        if self.namespace.is_empty() {
            return self.name.clone();
        }

        let mut result = self.namespace.join("::");
        result.push_str("::");
        result.push_str(&self.name);
        result
    }

    /// Validates the structural identity of the operator.
    ///
    /// This intentionally performs only representation-level checks.
    ///
    /// Semantic questions such as "does this operator exist?" belong to the
    /// semantic registry.
    pub fn validate(&self) -> Result<(), TypeOperatorError> {
        if self.name.is_empty() {
            return Err(TypeOperatorError::EmptyName);
        }

        for (index, component) in self.namespace.iter().enumerate() {
            if component.is_empty() {
                return Err(TypeOperatorError::EmptyNamespaceComponent { index });
            }
        }

        Ok(())
    }
}

impl fmt::Display for TypeOperator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

impl From<&str> for TypeOperator {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for TypeOperator {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

/// A source-level application of a type operator.
///
/// This is deliberately separate from `TypeExpr` until the canonical
/// `TypeExpr` schema explicitly adopts a type-operator variant.
///
/// Example conceptual forms:
///
/// ```text
/// Union<A, B>
/// Intersection<A, B>
/// Function<A, B>
/// ```
///
/// or, after parsing symbolic syntax:
///
/// ```text
/// A | B
/// A & B
/// A -> B
/// ```
///
/// The parser may normalize surface syntax into the same operator identity
/// while preserving source information elsewhere in the AST.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeOperatorExpr {
    operator: TypeOperator,
    operands: Vec<TypeExpr>,
}

impl TypeOperatorExpr {
    /// Creates an operator expression from an operator and ordered operands.
    ///
    /// Structural validation is intentionally deferred.
    #[must_use]
    pub fn new(operator: TypeOperator, operands: Vec<TypeExpr>) -> Self {
        Self { operator, operands }
    }

    /// Returns the operator identity.
    #[must_use]
    pub fn operator(&self) -> &TypeOperator {
        &self.operator
    }

    /// Returns the ordered operands.
    #[must_use]
    pub fn operands(&self) -> &[TypeExpr] {
        &self.operands
    }

    /// Returns the number of operands.
    #[must_use]
    pub fn operand_count(&self) -> usize {
        self.operands.len()
    }

    /// Returns whether there are no operands.
    #[must_use]
    pub fn is_nullary(&self) -> bool {
        self.operands.is_empty()
    }

    /// Returns the first operand, if present.
    #[must_use]
    pub fn first_operand(&self) -> Option<&TypeExpr> {
        self.operands.first()
    }

    /// Returns the final operand, if present.
    #[must_use]
    pub fn last_operand(&self) -> Option<&TypeExpr> {
        self.operands.last()
    }

    /// Returns an operand by source-order index.
    #[must_use]
    pub fn operand(&self, index: usize) -> Option<&TypeExpr> {
        self.operands.get(index)
    }

    /// Returns an iterator over operands.
    pub fn iter_operands(&self) -> impl ExactSizeIterator<Item = &TypeExpr> {
        self.operands.iter()
    }

    /// Consumes the expression and returns its operator and operands.
    #[must_use]
    pub fn into_parts(self) -> (TypeOperator, Vec<TypeExpr>) {
        (self.operator, self.operands)
    }

    /// Consumes the expression and returns its operands.
    #[must_use]
    pub fn into_operands(self) -> Vec<TypeExpr> {
        self.operands
    }

    /// Returns a source-oriented textual representation.
    ///
    /// This method deliberately uses a generic function-like representation
    /// rather than guessing symbolic syntax such as `|` or `&`.
    ///
    /// The semantic/operator registry may later define canonical surface
    /// syntax.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        let mut result = self.operator.to_source_string();
        result.push('<');

        for (index, operand) in self.operands.iter().enumerate() {
            if index != 0 {
                result.push_str(", ");
            }

            result.push_str(&operand.to_source_string());
        }

        result.push('>');
        result
    }

    /// Returns whether any immediate operand requires inference.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.operands.iter().any(TypeExpr::requires_inference)
    }

    /// Performs structural validation using the canonical `TypeExpr`
    /// validation implementation.
    pub fn validate(&self) -> Result<(), TypeOperatorError> {
        self.operator.validate()?;

        for operand in &self.operands {
            operand
                .validate()
                .map_err(TypeOperatorError::InvalidOperand)?;
        }

        Ok(())
    }

    /// Returns the stable source-AST category name.
    #[must_use]
    pub const fn variant_name(&self) -> &'static str {
        "type_operator"
    }
}

impl fmt::Display for TypeOperatorExpr {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

/// Structural errors produced by this module.
///
/// These errors intentionally do not include semantic failures such as:
///
/// - unknown operator;
/// - invalid operator arity;
/// - incompatible operands;
/// - unsatisfied generic constraints.
///
/// Those belong to semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeOperatorError {
    /// Operator name is empty.
    EmptyName,

    /// A namespace component is empty.
    EmptyNamespaceComponent {
        /// Source-order namespace component index.
        index: usize,
    },

    /// An operand contains an invalid canonical `TypeExpr`.
    InvalidOperand(super::type_expr::TypeExprError),
}

impl fmt::Display for TypeOperatorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => formatter.write_str("type operator name must not be empty"),

            Self::EmptyNamespaceComponent { index } => {
                write!(formatter, "type operator namespace component {index} is empty")
            }

            Self::InvalidOperand(error) => {
                write!(formatter, "invalid type-operator operand: {error}")
            }
        }
    }
}

impl std::error::Error for TypeOperatorError {}

/// Convenient conversion from a structural `TypeExpr` validation failure.
impl From<super::type_expr::TypeExprError> for TypeOperatorError {
    fn from(value: super::type_expr::TypeExprError) -> Self {
        Self::InvalidOperand(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(name: &str) -> TypeExpr {
        TypeExpr::name(name)
    }

    #[test]
    fn unqualified_operator_round_trips_source_name() {
        let operator = TypeOperator::new("union");

        assert_eq!(operator.name(), "union");
        assert!(operator.is_unqualified());
        assert_eq!(operator.to_source_string(), "union");
    }

    #[test]
    fn namespaced_operator_preserves_namespace_order() {
        let operator = TypeOperator::namespaced(
            ["core", "types"],
            "union",
        );

        assert_eq!(operator.namespace(), ["core", "types"]);
        assert_eq!(operator.namespace_depth(), 2);
        assert_eq!(operator.to_source_string(), "core::types::union");
    }

    #[test]
    fn empty_operator_name_is_structurally_invalid() {
        let operator = TypeOperator::new("");

        assert_eq!(operator.validate(), Err(TypeOperatorError::EmptyName));
    }

    #[test]
    fn empty_namespace_component_is_structurally_invalid() {
        let operator = TypeOperator::namespaced(["core", ""], "union");

        assert_eq!(
            operator.validate(),
            Err(TypeOperatorError::EmptyNamespaceComponent { index: 1 })
        );
    }

    #[test]
    fn operator_expression_preserves_operand_order() {
        let expression = TypeOperatorExpr::new(
            TypeOperator::new("union"),
            vec![named("A"), named("B")],
        );

        assert_eq!(expression.operand_count(), 2);
        assert_eq!(expression.first_operand(), Some(&named("A")));
        assert_eq!(expression.last_operand(), Some(&named("B")));
    }

    #[test]
    fn operator_expression_formats_deterministically() {
        let expression = TypeOperatorExpr::new(
            TypeOperator::new("union"),
            vec![named("A"), named("B")],
        );

        assert_eq!(expression.to_source_string(), "union<A, B>");
    }

    #[test]
    fn operator_expression_supports_arbitrary_arity() {
        let operands = vec![
            named("A"),
            named("B"),
            named("C"),
            named("D"),
            named("E"),
        ];

        let expression =
            TypeOperatorExpr::new(TypeOperator::new("product"), operands);

        assert_eq!(expression.operand_count(), 5);
    }

    #[test]
    fn operator_expression_supports_nested_type_expressions() {
        let inner = TypeExpr::generic(
            named("Container"),
            vec![named("T")],
        );

        let expression = TypeOperatorExpr::new(
            TypeOperator::new("transform"),
            vec![inner],
        );

        assert!(expression.validate().is_ok());
    }

    #[test]
    fn inference_delegates_to_canonical_type_expr() {
        let expression = TypeOperatorExpr::new(
            TypeOperator::new("transform"),
            vec![TypeExpr::GenericParameter(
                super::super::type_expr::TypeParameterName::from("T"),
            )],
        );

        assert!(expression.requires_inference());
    }

    #[test]
    fn validation_delegates_to_type_expr() {
        let expression = TypeOperatorExpr::new(
            TypeOperator::new("union"),
            vec![named("A"), named("B")],
        );

        assert!(expression.validate().is_ok());
    }

    #[test]
    fn into_parts_preserves_structure() {
        let expression = TypeOperatorExpr::new(
            TypeOperator::new("union"),
            vec![named("A"), named("B")],
        );

        let (operator, operands) = expression.into_parts();

        assert_eq!(operator.name(), "union");
        assert_eq!(operands.len(), 2);
    }
}