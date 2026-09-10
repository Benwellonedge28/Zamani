//! Zamani Frontend AST — Generic Parameters
//!
//! Canonical source-level representation of a generic type parameter.
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
//! frontend::ast::node::generics::parameter::TypeParameter
//!     │
//!     ├── TypeParameterName
//!     └── ordered TypeExpr bounds
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── generic scope
//!     ├── name resolution
//!     ├── constraint checking
//!     └── substitution
//!     │
//!     ▼
//! semantic type model
//!     │
//!     ▼
//! ZUIR / domain IR
//! ```
//!
//! # Responsibility
//!
//! This module owns the canonical source-level representation of one generic
//! type parameter.
//!
//! It represents programmer intent. It does not resolve names, perform type
//! inference, select hardware, allocate resources, specialize generics,
//! monomorphize code, route quantum resources, schedule execution, perform
//! error correction, select a backend, or lower directly to QIR/LLVM/MLIR.
//!
//! # Grammar integration
//!
//! The current Zamani grammar defines:
//!
//! ```text
//! genericParameters:
//!     '<' genericParameter (',' genericParameter)* '>'
//!
//! genericParameter:
//!     IDENTIFIER ('extends' typeConstraint)?
//!
//! typeConstraint:
//!     typeExpr ('+' typeExpr)*
//! ```
//!
//! Therefore the canonical representation is:
//!
//! ```text
//! TypeParameter
//!     ├── name: TypeParameterName
//!     └── bounds: Vec<TypeExpr>
//! ```
//!
//! Bound order is preserved exactly as written by the programmer.
//!
//! # Architectural invariants
//!
//! 1. There is exactly one canonical generic-parameter representation.
//! 2. A generic parameter is source-level and hardware-independent.
//! 3. Bounds are canonical `TypeExpr` values.
//! 4. No closed list of quantum/classical/hardware constraints exists here.
//! 5. Generic arity is not fixed.
//! 6. Bound count is not fixed by the language.
//! 7. No machine size is represented.
//! 8. No qubit count is represented.
//! 9. No topology is represented.
//! 10. No vendor or backend is represented.
//! 11. Safety/resource limits are explicit policies, never hidden constants.
//! 12. Semantic validity belongs to semantic analysis.
//!
//! # POCO-REAF
//!
//! A generic parameter does not encode the size or technology of the eventual
//! execution system.
//!
//! For example:
//!
//! ```text
//! fn solve<Q extends QuantumResource>(problem: Problem<Q>) ...
//! ```
//!
//! expresses an abstraction. It does not mean a fixed number of qubits,
//! registers, processors, devices, or any particular quantum technology.
//!
//! The eventual realization is determined downstream from the program's
//! semantic requirements and the capabilities/resources of the selected
//! execution environment.
//!
//! # Scalability
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_GENERIC_PARAMETERS
//! MAX_BOUNDS
//! MAX_QUBITS
//! MAX_MACHINE_SIZE
//! ```
//!
//! in this file.
//!
//! `Vec<TypeExpr>` grows according to available compiler resources.
//!
//! If a compiler invocation needs protection against pathological input, it
//! supplies a `TypeValidationPolicy`. Such a limit is a compiler resource
//! policy, not a Zamani language limitation.
//!
//! Thus this representation is valid from tiny programs through arbitrarily
//! large programs that the available compiler resources can represent.
//!
//! # Dependency boundary
//!
//! Allowed dependencies:
//!
//! - Rust standard library;
//! - `serde`;
//! - canonical frontend AST type-expression definitions.
//!
//! Forbidden dependencies:
//!
//! - parser implementation;
//! - lexer implementation;
//! - semantic implementation;
//! - compiler driver;
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
//! - vendor SDKs;
//! - filesystem/network APIs.
//!
//! # Rust requirements
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
//! # Serialization
//!
//! The representation derives `Serialize` and `Deserialize` structurally.
//!
//! Serialization is deterministic because:
//!
//! - the name is a single value;
//! - bounds use ordered `Vec` storage;
//! - no hash-map ordering is involved;
//! - no pointers or memory addresses are serialized;
//! - no target/backend state is serialized.
//!
//! Serialized AST compatibility is governed by the surrounding AST schema
//! version. This type deliberately does not conflate language, compiler,
//! semantic, or ZUIR versions.
//!
//! # Validation
//!
//! This module performs structural validation only.
//!
//! It validates:
//!
//! - parameter-name structure;
//! - configured identifier limits;
//! - configured collection limits;
//! - every bound through canonical `TypeExpr` validation.
//!
//! It does not determine whether a bound is semantically legal.
//!
//! For example, whether `QuantumResource` is a valid bound is a semantic
//! question, not an AST question.
//!
//! # Semantic integration
//!
//! Semantic analysis consumes validated values and performs:
//!
//! - generic-scope introduction;
//! - duplicate-name detection;
//! - name resolution;
//! - constraint resolution;
//! - constraint compatibility;
//! - recursive-constraint analysis;
//! - satisfiability analysis;
//! - substitution;
//! - instantiation checking.
//!
//! None of those responsibilities belong here.
//!
//! # ZUIR integration
//!
//! This file has no direct ZUIR dependency.
//!
//! The intended lowering boundary is:
//!
//! ```text
//! TypeParameter
//!     │
//!     ▼
//! semantic generic parameter
//!     │
//!     ▼
//! resolved semantic constraints
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! This prevents the source AST from becoming coupled to one downstream IR.
//!
//! # Visitor integration
//!
//! A visitor should:
//!
//! 1. visit the parameter name as a leaf/source identifier if its visitor
//!    contract exposes names;
//! 2. visit each bound in source order.
//!
//! This type intentionally does not implement a visitor itself. Traversal
//! policy belongs to the central AST traversal subsystem.
//!
//! # Parser integration
//!
//! The parser should construct this type after parsing:
//!
//! ```text
//! IDENTIFIER
//! optional "extends"
//! typeExpr
//! ("+" typeExpr)*
//! ```
//!
//! The parser must preserve bound order.
//!
//! Parser recovery remains outside this module.
//!
//! # Compatibility integration
//!
//! The repository currently contains a `types::type_parameter` implementation.
//! That implementation represents the same source-level concept. It must not
//! remain a second authoritative implementation.
//!
//! Migration target:
//!
//! ```text
//! old:
//! frontend::ast::node::types::type_parameter::TypeParameter
//!
//! new canonical:
//! frontend::ast::node::generics::parameter::TypeParameter
//! ```
//!
//! The old module should become a compatibility re-export once this module is
//! registered by `generics/mod.rs`.
//!
//! # Security
//!
//! This module:
//!
//! - performs no I/O;
//! - performs no network access;
//! - executes no source code;
//! - performs no raw pointer operations;
//! - uses no global mutable state;
//! - contains no `unsafe`.
//!
//! Invalid externally deserialized structures are rejected through explicit
//! validation rather than being treated as semantically valid.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::types::type_expr::{
    TypeExpr,
    TypeExprError,
    TypeParameterName,
    TypeValidationPolicy,
};

/// Schema version for the generic-parameter source representation.
///
/// This is independent from the Zamani language version, compiler version,
/// semantic-type version, and ZUIR version.
pub const GENERIC_PARAMETER_SCHEMA_VERSION: u16 = 1;

/// Canonical source-level generic type parameter.
///
/// A `TypeParameter` represents one entry in a generic parameter list such as:
///
/// ```text
/// <T>
/// <T extends Numeric>
/// <T extends Numeric + Ordered>
/// <Q extends QuantumResource>
/// ```
///
/// The type itself contains no resolved symbol identity. Resolution belongs to
/// semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeParameter {
    /// Source-level generic parameter name.
    name: TypeParameterName,

    /// Ordered source-level type constraints.
    ///
    /// The order is deliberately preserved because source order is useful for
    /// diagnostics, deterministic serialization, and source reconstruction.
    bounds: Vec<TypeExpr>,
}

impl TypeParameter {
    /// Creates an unbounded generic type parameter.
    ///
    /// Construction itself does not imply structural validity. Call
    /// [`Self::validate`] before accepting externally supplied data as a
    /// validated AST node.
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

    /// Creates a generic parameter with ordered type constraints.
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

    /// Creates an unbounded parameter from the canonical parameter-name type.
    #[must_use]
    pub fn from_name(name: TypeParameterName) -> Self {
        Self {
            name,
            bounds: Vec::new(),
        }
    }

    /// Creates a parameter from its canonical parts.
    #[must_use]
    pub fn from_parts(
        name: TypeParameterName,
        bounds: Vec<TypeExpr>,
    ) -> Self {
        Self { name, bounds }
    }

    /// Returns the canonical source-level parameter name.
    #[must_use]
    pub fn name(&self) -> &TypeParameterName {
        &self.name
    }

    /// Returns the parameter name as source text.
    #[must_use]
    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }

    /// Returns all bounds in their original source order.
    #[must_use]
    pub fn bounds(&self) -> &[TypeExpr] {
        &self.bounds
    }

    /// Returns the bound at `index`, if present.
    #[must_use]
    pub fn bound(&self, index: usize) -> Option<&TypeExpr> {
        self.bounds.get(index)
    }

    /// Returns an iterator over bounds in source order.
    #[must_use]
    pub fn iter_bounds(&self) -> impl ExactSizeIterator<Item = &TypeExpr> {
        self.bounds.iter()
    }

    /// Returns the number of bounds.
    #[must_use]
    pub fn bound_count(&self) -> usize {
        self.bounds.len()
    }

    /// Returns whether the parameter has no explicit bounds.
    #[must_use]
    pub fn is_unbounded(&self) -> bool {
        self.bounds.is_empty()
    }

    /// Returns whether the parameter has at least one explicit bound.
    #[must_use]
    pub fn is_bounded(&self) -> bool {
        !self.bounds.is_empty()
    }

    /// Appends one bound while preserving source order.
    ///
    /// No semantic interpretation is performed.
    pub fn push_bound(&mut self, bound: TypeExpr) {
        self.bounds.push(bound);
    }

    /// Appends multiple bounds while preserving iterator order.
    pub fn extend_bounds<I>(&mut self, bounds: I)
    where
        I: IntoIterator<Item = TypeExpr>,
    {
        self.bounds.extend(bounds);
    }

    /// Removes all bounds.
    ///
    /// This is a structural AST operation. It does not alter semantic state
    /// because semantic state does not belong to this type.
    pub fn clear_bounds(&mut self) {
        self.bounds.clear();
    }

    /// Consumes the parameter and returns its canonical parts.
    ///
    /// This avoids cloning potentially large nested bound structures.
    #[must_use]
    pub fn into_parts(self) -> (TypeParameterName, Vec<TypeExpr>) {
        (self.name, self.bounds)
    }

    /// Converts this declaration-level parameter into the canonical type
    /// expression used when the parameter is referenced.
    ///
    /// Bounds are constraints on the declaration and therefore are not copied
    /// into the reference expression.
    #[must_use]
    pub fn to_type_expr(&self) -> TypeExpr {
        TypeExpr::GenericParameter(self.name.clone())
    }

    /// Alias for [`Self::to_type_expr`].
    #[must_use]
    pub fn as_type_expr(&self) -> TypeExpr {
        self.to_type_expr()
    }

    /// Returns the stable AST construct identifier.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        "generic_parameter"
    }

    /// Returns this module's source-schema version.
    #[must_use]
    pub const fn schema_version() -> u16 {
        GENERIC_PARAMETER_SCHEMA_VERSION
    }

    /// Validates the parameter using the canonical default type policy.
    ///
    /// This validates structure, not semantic generic constraints.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.validate_with_policy(&TypeValidationPolicy::default())
    }

    /// Validates the parameter using an explicit compiler resource policy.
    ///
    /// Explicit policy limits are safety/resource controls and are not
    /// language-level limits.
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

    /// Returns whether the parameter name is structurally non-empty.
    ///
    /// This intentionally does not validate nested bounds.
    #[must_use]
    pub fn has_valid_name(&self) -> bool {
        !self.name.as_str().is_empty()
    }

    /// Returns whether any bound contains an inference placeholder.
    ///
    /// This is structural inspection only. It does not perform inference.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.bounds.iter().any(TypeExpr::requires_inference)
    }

    /// Returns whether any bound is itself a generic type expression.
    ///
    /// This does not resolve the referenced generic scope.
    #[must_use]
    pub fn has_generic_bound(&self) -> bool {
        self.bounds.iter().any(TypeExpr::is_generic)
    }

    /// Returns a deterministic Zamani source representation.
    ///
    /// Examples:
    ///
    /// ```text
    /// T
    /// T extends Numeric
    /// T extends Numeric + Ordered
    /// ```
    #[must_use]
    pub fn to_source_string(&self) -> String {
        if self.bounds.is_empty() {
            return self.name.to_string();
        }

        let mut output = String::new();

        output.push_str(self.name.as_str());
        output.push_str(" extends ");

        for (index, bound) in self.bounds.iter().enumerate() {
            if index != 0 {
                output.push_str(" + ");
            }

            output.push_str(&bound.to_source_string());
        }

        output
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

/// Validates a generic parameter name using the canonical type validation
/// policy.
///
/// Name resolution and language-specific identifier semantics remain outside
/// this file.
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

    #[test]
    fn constructs_unbounded_parameter() {
        let parameter = TypeParameter::new("T");

        assert_eq!(parameter.name_str(), "T");
        assert_eq!(parameter.bound_count(), 0);
        assert!(parameter.is_unbounded());
        assert!(!parameter.is_bounded());
        assert!(parameter.has_valid_name());
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
        assert_eq!(
            parameter.bound(0).map(TypeExpr::to_source_string),
            Some("Numeric".to_owned())
        );
        assert_eq!(
            parameter.bound(1).map(TypeExpr::to_source_string),
            Some("Ordered".to_owned())
        );
    }

    #[test]
    fn preserves_source_order() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![
                TypeExpr::name("First"),
                TypeExpr::name("Second"),
                TypeExpr::name("Third"),
            ],
        );

        let names: Vec<String> = parameter
            .iter_bounds()
            .map(TypeExpr::to_source_string)
            .collect();

        assert_eq!(
            names,
            vec![
                "First".to_owned(),
                "Second".to_owned(),
                "Third".to_owned(),
            ]
        );
    }

    #[test]
    fn converts_to_canonical_generic_parameter_reference() {
        let parameter = TypeParameter::new("T");

        assert_eq!(
            parameter.to_type_expr(),
            TypeExpr::GenericParameter(
                TypeParameterName::new("T")
            )
        );
    }

    #[test]
    fn declaration_bounds_do_not_leak_into_reference_type() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::name("Numeric")],
        );

        assert_eq!(
            TypeExpr::from(&parameter),
            TypeExpr::GenericParameter(
                TypeParameterName::new("T")
            )
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
    fn validates_nested_bounds_using_canonical_type_validation() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![
                TypeExpr::generic(
                    TypeExpr::name("Container"),
                    vec![TypeExpr::name("")],
                ),
            ],
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
    fn respects_collection_policy() {
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
    fn inference_detection_is_structural() {
        let unbounded = TypeParameter::new("T");
        assert!(!unbounded.requires_inference());

        let bounded = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::Infer],
        );

        assert!(bounded.requires_inference());
    }

    #[test]
    fn generic_bound_detection_is_structural() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![
                TypeExpr::generic(
                    TypeExpr::name("Container"),
                    vec![TypeExpr::name("U")],
                ),
            ],
        );

        assert!(parameter.has_generic_bound());
    }

    #[test]
    fn deterministic_source_formatting() {
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
    fn into_parts_preserves_all_structure() {
        let parameter = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::name("Numeric")],
        );

        let (name, bounds) = parameter.into_parts();

        assert_eq!(name.as_str(), "T");
        assert_eq!(bounds.len(), 1);
        assert_eq!(
            bounds[0].to_source_string(),
            "Numeric"
        );
    }

    #[test]
    fn structural_equality_is_deterministic() {
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

        let mut values = HashSet::new();
        values.insert(first);

        assert!(values.contains(&second));
    }

    #[test]
    fn quantum_constraints_remain_hardware_neutral() {
        let parameter = TypeParameter::with_bounds(
            "Q",
            vec![TypeExpr::name("QuantumResource")],
        );

        assert_eq!(parameter.name_str(), "Q");
        assert_eq!(
            parameter.to_type_expr(),
            TypeExpr::GenericParameter(
                TypeParameterName::new("Q")
            )
        );
    }

    #[test]
    fn supports_large_numbers_of_bounds_without_language_level_limit() {
        let bounds = (0..4096)
            .map(|index| {
                TypeExpr::name(format!("Constraint{index}"))
            })
            .collect();

        let parameter = TypeParameter::with_bounds("T", bounds);

        assert_eq!(parameter.bound_count(), 4096);
        assert!(parameter.validate().is_ok());
    }

    #[test]
    fn supports_deep_type_expression_structure() {
        let mut current = TypeExpr::name("Leaf");

        for _ in 0..256 {
            current = TypeExpr::optional(current);
        }

        let parameter =
            TypeParameter::with_bounds("T", vec![current]);

        assert!(parameter.validate().is_ok());
    }

    #[test]
    fn clear_bounds_is_structural_only() {
        let mut parameter = TypeParameter::with_bounds(
            "T",
            vec![TypeExpr::name("Numeric")],
        );

        parameter.clear_bounds();

        assert!(parameter.is_unbounded());
        assert_eq!(parameter.name_str(), "T");
    }

    #[test]
    fn schema_version_is_stable_and_nonzero() {
        assert_eq!(
            TypeParameter::schema_version(),
            GENERIC_PARAMETER_SCHEMA_VERSION
        );
        assert!(GENERIC_PARAMETER_SCHEMA_VERSION > 0);
    }
}