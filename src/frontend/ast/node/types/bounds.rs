//! Zamani Frontend AST — Type Bounds
//!
//! Production-ready, source-level representation of the bounds attached to
//! generic type parameters.
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
//!     └── TypeBounds
//!             │
//!             └── TypeExpr
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── name resolution
//!     ├── constraint solving
//!     ├── trait/capability interpretation
//!     └── generic satisfiability
//!     │
//!     ▼
//! semantic type/constraint model
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
//! `TypeBounds` represents the ordered source-level constraints associated
//! with a generic type parameter.
//!
//! A bound is deliberately represented by the canonical [`TypeExpr`] rather
//! than by a closed enumeration such as:
//!
//! ```text
//! enum TypeBound {
//!     Trait(...),
//!     Quantum(...),
//!     ...
//! }
//! ```
//!
//! Such an enumeration would make the native AST depend on a finite list of
//! currently known concepts and would force unrelated AST code to change every
//! time Zamani acquires a new type-system capability or computational domain.
//!
//! Instead:
//!
//! ```text
//! TypeBounds
//!     └── Vec<TypeExpr>
//! ```
//!
//! gives the language an extensible source-level constraint representation.
//!
//! Semantic analysis determines what each expression means as a bound.
//!
//! # Existing repository integration
//!
//! The current `TypeParameter` implementation already represents bounds as:
//!
//! ```text
//! Vec<TypeExpr>
//! ```
//!
//! and explicitly defines bounds as ordinary source-level type expressions.
//!
//! `TypeBounds` therefore acts as the canonical collection abstraction for
//! that concept. Migration from the raw vector should be performed at the
//! `TypeParameter` boundary rather than introducing a second incompatible
//! bound representation.
//!
//! Existing source-level expressions remain valid:
//!
//! ```text
//! T
//! T extends Numeric
//! T extends Foo + Bar
//! ```
//!
//! The parser should preserve their order.
//!
//! # Domain neutrality
//!
//! This file contains no quantum-specific bound type.
//!
//! A bound may eventually constrain:
//!
//! - classical computation;
//! - quantum computation;
//! - hybrid computation;
//! - distributed computation;
//! - accelerator computation;
//! - HDL constructs;
//! - AI/ML constructs;
//! - future computational domains.
//!
//! The AST does not decide which domain a bound belongs to.
//!
//! It also does not know whether a bound eventually describes:
//!
//! - a qubit;
//! - a logical quantum resource;
//! - a physical resource;
//! - a GPU;
//! - a CPU;
//! - memory;
//! - an accelerator;
//! - a distributed resource;
//! - a future computational resource.
//!
//! Those decisions belong to semantic analysis and later compiler stages.
//!
//! # POCO-REAF
//!
//! `TypeBounds` introduces no machine-size assumptions.
//!
//! There is no:
//!
//! ```text
//! MAX_BOUNDS
//! MAX_GENERIC_PARAMETERS
//! MAX_QUBITS
//! MAX_RESOURCES
//! MAX_MACHINE_SIZE
//! ```
//!
//! in this module.
//!
//! The number of bounds is represented by an ordinary dynamically sized
//! collection and is limited only by the resources available to the compiler
//! and any explicitly configured compiler safety policy.
//!
//! Therefore a generic declaration can conceptually contain:
//!
//! ```text
//! zero bounds
//! one bound
//! many bounds
//! arbitrarily large source-level bound sets
//! ```
//!
//! without changing the AST schema.
//!
//! # Semantic boundary
//!
//! This module does NOT perform:
//!
//! - name resolution;
//! - type inference;
//! - generic substitution;
//! - trait resolution;
//! - capability resolution;
//! - constraint solving;
//! - satisfiability checking;
//! - specialization;
//! - monomorphization;
//! - type layout;
//! - resource allocation;
//! - quantum allocation;
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
//! It represents source-level intent only.
//!
//! # Parser integration
//!
//! The parser should construct bounds in source order.
//!
//! For grammar conceptually equivalent to:
//!
//! ```text
//! typeBound     : 'extends' typeExpr;
//! typeBoundList : typeBound ('+' typeBound)*;
//! ```
//!
//! the parser should produce:
//!
//! ```text
//! TypeParameter
//!     └── TypeBounds
//!           ├── TypeExpr
//!           ├── TypeExpr
//!           └── ...
//! ```
//!
//! No parser token type is imported here.
//!
//! No parser recovery logic is implemented here.
//!
//! # Semantic integration
//!
//! Semantic analysis consumes the validated collection and determines:
//!
//! - what each bound resolves to;
//! - whether the referenced type exists;
//! - whether the bound is a valid constraint;
//! - whether bounds conflict;
//! - whether bounds imply capabilities;
//! - whether a generic instantiation satisfies the bounds;
//! - whether recursive constraints are valid;
//! - whether associated constraints can be resolved.
//!
//! None of those operations belong here.
//!
//! # ZUIR integration
//!
//! `TypeBounds` must not depend directly on ZUIR.
//!
//! The intended lowering boundary is:
//!
//! ```text
//! TypeBounds
//!     │
//!     ▼
//! semantic generic constraints
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! ZUIR decides how semantic constraints that survive lowering are represented.
//!
//! # Serialization
//!
//! Serialization is structural and deterministic.
//!
//! Bounds preserve source order.
//!
//! No unordered collection is used.
//!
//! No memory address is serialized.
//!
//! No compiler-global state is serialized.
//!
//! No hardware information is serialized.
//!
//! No backend information is serialized.
//!
//! `serde` derives are intentionally used so this collection follows the
//! serialization representation of the canonical `TypeExpr` values.
//!
//! Schema/version management remains owned by the AST serialization layer.
//!
//! # Validation
//!
//! `TypeBounds` performs only structural validation:
//!
//! - every contained `TypeExpr` must be structurally valid;
//! - configured compiler resource policies are respected;
//! - malformed structures return errors rather than panic.
//!
//! It does NOT determine whether a particular expression is semantically
//! legal as a generic bound.
//!
//! For example:
//!
//! ```text
//! T extends Foo
//! ```
//!
//! may be structurally valid even when `Foo` does not exist.
//!
//! Whether `Foo` is a legal bound is a semantic-analysis concern.
//!
//! # Determinism
//!
//! Bound order is part of the source representation.
//!
//! Therefore:
//!
//! ```text
//! A + B
//! ```
//!
//! remains structurally distinct from:
//!
//! ```text
//! B + A
//! ```
//!
//! even if a later semantic phase determines that the constraints are
//! mathematically commutative.
//!
//! This preserves source fidelity and deterministic diagnostics.
//!
//! # Scalability
//!
//! The collection uses `Vec<TypeExpr>` because:
//!
//! - it has no language-level fixed capacity;
//! - it preserves source order;
//! - it provides contiguous storage;
//! - it supports efficient iteration;
//! - it has predictable ownership semantics;
//! - it integrates naturally with the existing `TypeParameter` representation.
//!
//! This file introduces no recursive traversal of nested `TypeExpr` values.
//! Nested validation is delegated to the canonical `TypeExpr` implementation.
//!
//! Compiler safety limits remain policy-driven.
//!
//! # Performance
//!
//! The collection operations are intentionally straightforward:
//!
//! - creation: proportional to the supplied collection;
//! - append: amortized constant time;
//! - iteration: linear in the number of bounds;
//! - indexing: constant time;
//! - conversion to/from the canonical vector: ownership-preserving;
//! - structural validation: delegated to `TypeExpr`.
//!
//! No hash map is used because bound order must remain deterministic.
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
//! - contains no `unsafe`;
//! - does not access mutable global state.
//!
//! Untrusted serialized data must be structurally validated before semantic
//! consumption.
//!
//! # Thread safety
//!
//! The collection contains owned AST data and has no mutable global state.
//!
//! Its `Send`/`Sync` properties therefore follow those of `TypeExpr` and its
//! contained values.
//!
//! No synchronization primitive is required by this module.
//!
//! # Forbidden dependencies
//!
//! This module must not depend on:
//!
//! - `crate::ast`;
//! - lexer implementation;
//! - parser implementation;
//! - compiler driver;
//! - semantic analyzer;
//! - ZUIR;
//! - quantum IR;
//! - QEC;
//! - ZQN;
//! - routing;
//! - scheduling;
//! - calibration;
//! - runtime;
//! - hardware;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - vendor SDKs.
//!
//! Allowed dependencies are:
//!
//! - Rust standard library;
//! - `serde`;
//! - canonical sibling `type_expr`.
//!
//! # Migration contract
//!
//! The legacy AST has a closed `TypeBound` abstraction. The new frontend AST
//! must not import it.
//!
//! Migration is:
//!
//! ```text
//! legacy TypeBound
//!       │
//!       ▼
//! source-level TypeExpr
//!       │
//!       ▼
//! TypeBounds
//!       │
//!       ▼
//! semantic constraint representation
//! ```
//!
//! Existing `TypeExpr` remains authoritative.
//!
//! `TypeBounds` is a collection abstraction, not another type-expression
//! representation.
//!
//! # Independent-file completion contract
//!
//! This file is complete when:
//!
//! - there is one canonical bound collection representation;
//! - every bound is represented by `TypeExpr`;
//! - source order is preserved;
//! - arbitrary generic arity is supported;
//! - no fixed bound count exists;
//! - no hardware information exists;
//! - no quantum backend information exists;
//! - no closed future-domain enumeration exists;
//! - structural validation is available;
//! - validation can use compiler policy;
//! - malformed data produces errors rather than panics;
//! - serialization is deterministic;
//! - visitors can enumerate every bound;
//! - parser integration is explicit;
//! - semantic integration is explicit;
//! - ZUIR integration is explicit;
//! - no legacy AST dependency exists;
//! - no unsafe code exists.
//!
//! # Rust compatibility
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
//! # Implementation
//!
//! This module intentionally contains only source-level bound collection
//! behavior.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::type_expr::{TypeExpr, TypeExprError, TypeValidationPolicy};

/// Ordered source-level generic type bounds.
///
/// `TypeBounds` is deliberately a collection of canonical [`TypeExpr`] values
/// rather than a closed enum of bound kinds.
///
/// # Invariant
///
/// Every element is an independent source-level type expression.
///
/// Semantic validity of those expressions as bounds is checked later by
/// semantic analysis.
///
/// # Determinism
///
/// Elements retain source order.
///
/// # Scalability
///
/// There is no language-level maximum number of bounds.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TypeBounds(Vec<TypeExpr>);

impl TypeBounds {
    /// Creates an empty bound collection.
    ///
    /// An empty collection represents an unconstrained generic parameter.
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates bounds from an owned vector while preserving source order.
    #[must_use]
    pub fn from_vec(bounds: Vec<TypeExpr>) -> Self {
        Self(bounds)
    }

    /// Creates bounds from any iterator.
    ///
    /// Source order is the iterator order.
    #[must_use]
    pub fn from_iter<I>(bounds: I) -> Self
    where
        I: IntoIterator<Item = TypeExpr>,
    {
        Self(bounds.into_iter().collect())
    }

    /// Returns the ordered bounds as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[TypeExpr] {
        &self.0
    }

    /// Returns a mutable view of the ordered bounds.
    ///
    /// Mutating the collection does not perform semantic validation.
    ///
    /// Call [`Self::validate`] before treating the resulting AST as
    /// structurally validated.
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [TypeExpr] {
        &mut self.0
    }

    /// Returns the number of bounds.
    ///
    /// This is a source-level collection size, not a machine-resource count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether there are no bounds.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the first bound, if present.
    #[must_use]
    pub fn first(&self) -> Option<&TypeExpr> {
        self.0.first()
    }

    /// Returns the last bound, if present.
    #[must_use]
    pub fn last(&self) -> Option<&TypeExpr> {
        self.0.last()
    }

    /// Returns a bound by zero-based source order.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&TypeExpr> {
        self.0.get(index)
    }

    /// Appends one source-level bound.
    ///
    /// No semantic interpretation is performed.
    pub fn push(&mut self, bound: TypeExpr) {
        self.0.push(bound);
    }

    /// Appends several bounds while preserving iterator order.
    pub fn extend<I>(&mut self, bounds: I)
    where
        I: IntoIterator<Item = TypeExpr>,
    {
        self.0.extend(bounds);
    }

    /// Removes and returns the last bound.
    #[must_use]
    pub fn pop(&mut self) -> Option<TypeExpr> {
        self.0.pop()
    }

    /// Removes all bounds.
    ///
    /// This produces a structurally representable unbounded collection.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns an iterator over bounds in source order.
    pub fn iter(&self) -> std::slice::Iter<'_, TypeExpr> {
        self.0.iter()
    }

    /// Returns a mutable iterator over bounds in source order.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, TypeExpr> {
        self.0.iter_mut()
    }

    /// Returns an owned vector containing the canonical bound expressions.
    ///
    /// This consumes the collection and therefore performs no element cloning.
    #[must_use]
    pub fn into_vec(self) -> Vec<TypeExpr> {
        self.0
    }

    /// Returns the collection's canonical source-level variant name.
    #[must_use]
    pub const fn variant_name() -> &'static str {
        "type_bounds"
    }

    /// Returns whether any bound structurally requires inference.
    ///
    /// This reports only information exposed by the canonical `TypeExpr`
    /// representation. It does not perform type inference.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.0.iter().any(TypeExpr::requires_inference)
    }

    /// Validates all contained type expressions with the default policy.
    ///
    /// Semantic validity as a generic constraint is intentionally not checked.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.validate_with_policy(&TypeValidationPolicy::default())
    }

    /// Validates all contained type expressions using an explicit compiler
    /// resource/safety policy.
    ///
    /// The policy is a compiler input and is not part of Zamani language
    /// semantics.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        for bound in &self.0 {
            bound.validate_with_policy(policy)?;
        }

        Ok(())
    }

    /// Produces the deterministic source representation of the bounds.
    ///
    /// Individual bounds use the canonical `TypeExpr` source representation.
    ///
    /// The `+` separator corresponds to Zamani's source-level bound-list
    /// grammar.
    ///
    /// An empty collection produces an empty string.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        self.0
            .iter()
            .map(TypeExpr::to_source_string)
            .collect::<Vec<_>>()
            .join(" + ")
    }

    /// Returns the bounds as a borrowed iterator.
    ///
    /// This is provided as an explicit traversal boundary for visitors and
    /// generic AST tooling.
    pub fn children(&self) -> impl Iterator<Item = &TypeExpr> {
        self.0.iter()
    }

    /// Returns an owned collection of cloned child expressions.
    ///
    /// This is useful for transformations that need ownership of the bounds.
    #[must_use]
    pub fn cloned_children(&self) -> Vec<TypeExpr> {
        self.0.clone()
    }
}

impl From<Vec<TypeExpr>> for TypeBounds {
    fn from(value: Vec<TypeExpr>) -> Self {
        Self::from_vec(value)
    }
}

impl From<TypeBounds> for Vec<TypeExpr> {
    fn from(value: TypeBounds) -> Self {
        value.into_vec()
    }
}

impl<'a> IntoIterator for &'a TypeBounds {
    type Item = &'a TypeExpr;
    type IntoIter = std::slice::Iter<'a, TypeExpr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut TypeBounds {
    type Item = &'a mut TypeExpr;
    type IntoIter = std::slice::IterMut<'a, TypeExpr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl IntoIterator for TypeBounds {
    type Item = TypeExpr;
    type IntoIter = std::vec::IntoIter<TypeExpr>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<TypeExpr> for TypeBounds {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = TypeExpr>,
    {
        Self::from_iter(iter)
    }
}

impl fmt::Display for TypeBounds {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(name: &str) -> TypeExpr {
        TypeExpr::name(name)
    }

    #[test]
    fn empty_bounds_are_unbounded() {
        let bounds = TypeBounds::new();

        assert!(bounds.is_empty());
        assert_eq!(bounds.len(), 0);
        assert_eq!(bounds.to_source_string(), "");
    }

    #[test]
    fn preserves_source_order() {
        let bounds = TypeBounds::from_vec(vec![
            named("Numeric"),
            named("Serializable"),
            named("QuantumResource"),
        ]);

        assert_eq!(
            bounds.to_source_string(),
            "Numeric + Serializable + QuantumResource"
        );
    }

    #[test]
    fn supports_arbitrary_generic_bound_arity() {
        let bounds = TypeBounds::from_iter(
            [
                "A", "B", "C", "D", "E", "F", "G", "H",
            ]
            .into_iter()
            .map(named),
        );

        assert_eq!(bounds.len(), 8);
        assert_eq!(bounds.get(0), Some(&named("A")));
        assert_eq!(bounds.get(7), Some(&named("H")));
    }

    #[test]
    fn push_and_extend_preserve_order() {
        let mut bounds = TypeBounds::new();

        bounds.push(named("A"));
        bounds.extend(vec![named("B"), named("C")]);

        assert_eq!(bounds.to_source_string(), "A + B + C");
    }

    #[test]
    fn pop_and_clear_are_structural_operations() {
        let mut bounds =
            TypeBounds::from_vec(vec![named("A"), named("B")]);

        assert_eq!(bounds.pop(), Some(named("B")));
        assert_eq!(bounds.to_source_string(), "A");

        bounds.clear();

        assert!(bounds.is_empty());
    }

    #[test]
    fn validates_canonical_type_expressions() {
        let bounds = TypeBounds::from_vec(vec![
            named("Numeric"),
            named("Serializable"),
        ]);

        assert!(bounds.validate().is_ok());
    }

    #[test]
    fn requires_inference_delegates_to_type_expr() {
        let bounds = TypeBounds::from_vec(vec![named("T")]);

        // Named types are structurally concrete in the canonical AST.
        assert!(!bounds.requires_inference());
    }

    #[test]
    fn conversion_to_vec_is_lossless() {
        let original = vec![named("A"), named("B")];
        let bounds = TypeBounds::from_vec(original.clone());

        let recovered: Vec<TypeExpr> = bounds.into();

        assert_eq!(recovered, original);
    }

    #[test]
    fn borrowed_iteration_preserves_order() {
        let bounds =
            TypeBounds::from_vec(vec![named("A"), named("B"), named("C")]);

        let names: Vec<String> =
            bounds.iter().map(TypeExpr::to_source_string).collect();

        assert_eq!(names, vec!["A", "B", "C"]);
    }

    #[test]
    fn display_is_deterministic() {
        let bounds =
            TypeBounds::from_vec(vec![named("A"), named("B")]);

        assert_eq!(bounds.to_string(), "A + B");
    }

    #[test]
    fn serde_round_trip_preserves_structure() {
        let bounds =
            TypeBounds::from_vec(vec![named("A"), named("B")]);

        let encoded = serde_json::to_string(&bounds)
            .expect("TypeBounds should serialize");

        let decoded: TypeBounds = serde_json::from_str(&encoded)
            .expect("TypeBounds should deserialize");

        assert_eq!(decoded, bounds);
    }
}