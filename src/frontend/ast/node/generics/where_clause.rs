//! Zamani Frontend AST — Generic Where Clauses
//!
//! Canonical source-level representation of a generic `where` clause.
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
//! frontend::ast::node::generics::where_clause::WhereClause
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── generic scope
//!     ├── name resolution
//!     ├── bound resolution
//!     ├── constraint compatibility
//!     ├── substitution
//!     └── satisfiability
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR / domain IR
//! ```
//!
//! # Responsibility
//!
//! This module owns the source-level representation of a generic `where`
//! clause.
//!
//! A `WhereClause` expresses programmer-written constraints over generic
//! parameters and/or other source-level type expressions.
//!
//! It represents source intent only.
//!
//! It does NOT:
//!
//! - resolve names;
//! - resolve traits/interfaces;
//! - perform type inference;
//! - perform generic substitution;
//! - determine satisfiability;
//! - select implementations;
//! - select hardware;
//! - allocate resources;
//! - determine machine size;
//! - determine qubit count;
//! - determine topology;
//! - route quantum operations;
//! - schedule execution;
//! - perform calibration;
//! - perform quantum error correction;
//! - select a backend;
//! - lower directly to QIR;
//! - lower directly to LLVM;
//! - lower directly to MLIR;
//! - execute code.
//!
//! Those responsibilities belong to later compiler stages.
//!
//! # Architectural principle
//!
//! A `where` clause is a source-level constraint.
//!
//! It must never become a disguised hardware-capability list.
//!
//! For example, a source-level constraint such as:
//!
//! ```text
//! where Q: QuantumResource
//! ```
//!
//! says that `Q` must satisfy the named source-level constraint.
//!
//! It does NOT mean:
//!
//! - a fixed number of qubits;
//! - a specific QPU;
//! - a specific vendor;
//! - a specific coupling graph;
//! - a specific gate set;
//! - a specific physical implementation.
//!
//! The semantic/compiler layers determine whether and how that requirement can
//! be realized on a selected execution target.
//!
//! # Canonical representation
//!
//! Existing Zamani AST infrastructure already represents generic constraints
//! using canonical `TypeExpr` values.
//!
//! This module therefore deliberately uses:
//!
//! ```text
//! TypeParameterName
//! TypeExpr
//! ```
//!
//! rather than introducing a second bound/constraint AST.
//!
//! A predicate is represented as:
//!
//! ```text
//! subject : TypeExpr
//! ```
//!
//! or, where the grammar permits a source-level parameter name directly:
//!
//! ```text
//! subject : TypeParameterName
//! ```
//!
//! The normalized representation stored by this module is a `TypeExpr` subject
//! plus an ordered collection of canonical `TypeExpr` bounds.
//!
//! # Why the representation is ordered
//!
//! Source order is preserved because it is useful for:
//!
//! - deterministic diagnostics;
//! - deterministic serialization;
//! - source reconstruction;
//! - stable compiler behavior;
//! - tooling;
//! - IDE integrations;
//! - reproducible builds.
//!
//! Semantic analysis may later canonicalize or otherwise interpret constraints,
//! but that does not belong in the AST.
//!
//! # POCO-REAF
//!
//! Generic constraints must not encode the physical size of the execution
//! system.
//!
//! A generic algorithm can therefore be expressed using source-level
//! constraints while leaving resource realization to later compilation stages.
//!
//! Conceptually:
//!
//! ```text
//! source program
//!     │
//!     ▼
//! generic constraints
//!     │
//!     ▼
//! semantic requirements
//!     │
//!     ▼
//! available capabilities/resources
//!     │
//!     ▼
//! target realization
//! ```
//!
//! This is compatible with Zamani's:
//!
//! ```text
//! Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//! ```
//!
//! No fixed machine size, qubit count, topology, vendor, backend or processor
//! architecture is encoded here.
//!
//! # Scalability
//!
//! There is intentionally no:
//!
//! ```text
//! MAX_WHERE_PREDICATES
//! MAX_BOUNDS
//! MAX_GENERIC_PARAMETERS
//! MAX_QUBITS
//! MAX_MACHINE_SIZE
//! ```
//!
//! in this file.
//!
//! Collections grow according to the available compiler resources.
//!
//! If compilation needs protection against pathological input, limits must be
//! supplied through explicit compiler policies. Such limits are resource and
//! safety policies, not language semantics.
//!
//! # Dependency boundary
//!
//! Allowed dependencies:
//!
//! - Rust standard library;
//! - `serde`;
//! - canonical frontend AST type-expression definitions;
//! - canonical frontend AST source-span infrastructure when available through
//!   the surrounding node contract.
//!
//! Forbidden dependencies:
//!
//! - parser implementation;
//! - lexer implementation;
//! - semantic model implementation;
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
//! - filesystem APIs;
//! - network APIs.
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
//! This type derives `Serialize` and `Deserialize`.
//!
//! Serialization preserves:
//!
//! - predicate order;
//! - subject order;
//! - bound order;
//! - source-level names;
//! - source-level type expressions.
//!
//! It does not serialize:
//!
//! - resolved symbols;
//! - semantic types;
//! - hardware;
//! - capabilities discovered from a target;
//! - compiler caches;
//! - memory addresses;
//! - backend state.
//!
//! Serialization compatibility is governed by the surrounding AST schema
//! version. This local representation does not attempt to conflate language,
//! compiler, semantic-model, ZUIR, or backend versions.
//!
//! # Validation
//!
//! Validation performed here is structural.
//!
//! It may validate:
//!
//! - non-empty subjects;
//! - valid canonical `TypeExpr` structures;
//! - explicit collection/resource policies;
//! - structurally valid constraint collections.
//!
//! It must NOT determine whether a constraint is semantically satisfiable.
//!
//! For example:
//!
//! ```text
//! where T: Numeric
//! ```
//!
//! may be structurally valid even when `Numeric` has not yet been resolved.
//!
//! Resolution belongs to semantic analysis.
//!
//! # Semantic integration
//!
//! Semantic analysis consumes validated `WhereClause` values and is responsible
//! for:
//!
//! - resolving generic parameter names;
//! - resolving type names;
//! - resolving traits/interfaces/constraints;
//! - detecting duplicate predicates;
//! - checking constraint compatibility;
//! - detecting impossible constraints;
//! - checking generic scope;
//! - performing substitution;
//! - checking instantiated arguments;
//! - constructing the semantic generic constraint model.
//!
//! This module deliberately does none of those things.
//!
//! # ZUIR integration
//!
//! There is no direct ZUIR dependency.
//!
//! The intended path is:
//!
//! ```text
//! WhereClause
//!     │
//!     ▼
//! semantic generic constraints
//!     │
//!     ▼
//! resolved semantic requirements
//!     │
//!     ▼
//! ZUIR
//! ```
//!
//! This prevents the source AST from becoming coupled to one intermediate
//! representation.
//!
//! # Visitor integration
//!
//! Central AST visitors should traverse:
//!
//! 1. every predicate in source order;
//! 2. each predicate subject;
//! 3. each bound in source order.
//!
//! This file intentionally does not define a competing visitor framework.
//! Traversal policy belongs to the canonical AST visitor/traversal subsystem.
//!
//! # Parser integration
//!
//! The parser should construct this representation after parsing the complete
//! syntactic `where` clause.
//!
//! Conceptually:
//!
//! ```text
//! where
//!     predicate
//!     (separator predicate)*
//! ```
//!
//! The exact separator and predicate grammar must be determined by Zamani's
//! canonical grammar.
//!
//! This module intentionally does not duplicate parser grammar rules.
//!
//! Parser recovery remains a parser responsibility.
//!
//! # Generic-parameter integration
//!
//! A generic declaration may conceptually contain:
//!
//! ```text
//! <T, U>
//! where
//!     T: Constraint,
//!     U: Constraint
//! ```
//!
//! `TypeParameter` remains responsible for constraints syntactically attached
//! directly to a generic parameter when that is how the grammar represents
//! them.
//!
//! `WhereClause` represents constraints written in a separate `where`
//! section.
//!
//! The semantic layer is responsible for combining those source-level
//! constraints into the complete generic constraint environment.
//!
//! Neither representation should be silently duplicated or merged inside the
//! AST.
//!
//! # No duplicate bound model
//!
//! Existing Zamani type infrastructure already has canonical `TypeExpr`
//! structures and type-bound support.
//!
//! `WhereClause` therefore does NOT introduce:
//!
//! ```text
//! GenericBound
//! TraitBound
//! HardwareBound
//! QuantumBound
//! BackendBound
//! ```
//!
//! enums merely to represent categories that semantic analysis can resolve.
//!
//! This prevents a closed-world constraint hierarchy from becoming a future
//! scalability bottleneck.
//!
//! # Domain neutrality
//!
//! This file does not contain special cases for:
//!
//! - quantum;
//! - classical;
//! - HDL;
//! - AI;
//! - GPU;
//! - FPGA;
//! - distributed computing;
//! - photonic computing;
//! - neuromorphic computing;
//! - future computational domains.
//!
//! A domain-specific constraint is represented through the canonical
//! source-level type/path vocabulary and interpreted later.
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
//! Deserialized values remain untrusted until structural validation succeeds.
//!
//! # Definition of done
//!
//! This file is complete when:
//!
//! - `WhereClause` is the single canonical AST representation for standalone
//!   `where` clauses;
//! - it uses the canonical `TypeExpr` representation;
//! - no hardware assumptions exist;
//! - no fixed collection limits exist in the type;
//! - serialization is deterministic;
//! - structural validation is available;
//! - parser integration is explicitly defined;
//! - semantic integration is explicitly defined;
//! - visitor integration is defined;
//! - ZUIR lowering has a documented boundary;
//! - tests cover construction, ordering, validation and serialization;
//! - `generics/mod.rs` exports it;
//! - declaration/function nodes consume it without duplicating its model;
//! - semantic analysis consumes it without requiring changes to this file;
//! - no unsafe code is used.
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

/// Schema version for the standalone generic `where` clause representation.
///
/// This version is independent of the Zamani language version, compiler
/// version, semantic-model version and ZUIR version.
pub const WHERE_CLAUSE_SCHEMA_VERSION: u16 = 1;

/// One source-level generic constraint predicate.
///
/// Conceptually represents:
///
/// ```text
/// Subject: Bound
/// ```
///
/// For example:
///
/// ```text
/// T: Numeric
/// Q: QuantumResource
/// R: Resource + Iterable
/// ```
///
/// `subject` and `bounds` remain unresolved source-level expressions.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WherePredicate {
    /// The source-level type expression being constrained.
    subject: TypeExpr,

    /// Ordered source-level bounds/constraints.
    bounds: Vec<TypeExpr>,
}

impl WherePredicate {
    /// Creates a predicate with no bounds.
    ///
    /// This is useful during parser construction and error recovery.
    /// Structural validation must be performed before accepting the completed
    /// AST.
    #[must_use]
    pub fn new(subject: TypeExpr) -> Self {
        Self {
            subject,
            bounds: Vec::new(),
        }
    }

    /// Creates a predicate with ordered bounds.
    #[must_use]
    pub fn with_bounds(
        subject: TypeExpr,
        bounds: Vec<TypeExpr>,
    ) -> Self {
        Self { subject, bounds }
    }

    /// Creates a predicate from canonical parts.
    #[must_use]
    pub fn from_parts(
        subject: TypeExpr,
        bounds: Vec<TypeExpr>,
    ) -> Self {
        Self { subject, bounds }
    }

    /// Creates a predicate whose subject is a generic parameter name.
    #[must_use]
    pub fn for_parameter(name: TypeParameterName) -> Self {
        Self::new(TypeExpr::GenericParameter(name))
    }

    /// Returns the constrained source-level type expression.
    #[must_use]
    pub fn subject(&self) -> &TypeExpr {
        &self.subject
    }

    /// Returns the ordered source-level bounds.
    #[must_use]
    pub fn bounds(&self) -> &[TypeExpr] {
        &self.bounds
    }

    /// Returns a bound by index.
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

    /// Returns whether this predicate has no explicit bounds.
    #[must_use]
    pub fn is_unbounded(&self) -> bool {
        self.bounds.is_empty()
    }

    /// Returns whether this predicate has at least one bound.
    #[must_use]
    pub fn is_bounded(&self) -> bool {
        !self.bounds.is_empty()
    }

    /// Appends a bound while preserving source order.
    pub fn push_bound(&mut self, bound: TypeExpr) {
        self.bounds.push(bound);
    }

    /// Extends the bounds using iterator order.
    pub fn extend_bounds<I>(&mut self, bounds: I)
    where
        I: IntoIterator<Item = TypeExpr>,
    {
        self.bounds.extend(bounds);
    }

    /// Removes all bounds.
    pub fn clear_bounds(&mut self) {
        self.bounds.clear();
    }

    /// Consumes the predicate into its canonical parts.
    #[must_use]
    pub fn into_parts(self) -> (TypeExpr, Vec<TypeExpr>) {
        (self.subject, self.bounds)
    }

    /// Returns whether the subject is a generic parameter reference.
    #[must_use]
    pub fn subject_is_generic_parameter(&self) -> bool {
        matches!(self.subject, TypeExpr::GenericParameter(_))
    }

    /// Returns the generic parameter name when the subject is a generic
    /// parameter reference.
    #[must_use]
    pub fn parameter_name(&self) -> Option<&TypeParameterName> {
        match &self.subject {
            TypeExpr::GenericParameter(name) => Some(name),
            _ => None,
        }
    }

    /// Returns whether this predicate contains an inference placeholder.
    ///
    /// This performs structural inspection only.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.subject.requires_inference()
            || self.bounds.iter().any(TypeExpr::requires_inference)
    }

    /// Returns whether the predicate contains generic expressions.
    ///
    /// This does not resolve generic scopes.
    #[must_use]
    pub fn contains_generic_types(&self) -> bool {
        self.subject.is_generic()
            || self.bounds.iter().any(TypeExpr::is_generic)
    }

    /// Structurally validates the predicate using the canonical default policy.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.validate_with_policy(&TypeValidationPolicy::default())
    }

    /// Structurally validates the predicate using an explicit compiler policy.
    ///
    /// Policy limits are compiler safety/resource controls and do not impose
    /// language-level machine or quantum-resource limits.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        self.subject.validate_with_policy(policy)?;

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

    /// Produces deterministic source-level text for the predicate.
    ///
    /// This is intended for diagnostics, tooling and source-oriented
    /// serialization helpers. It does not attempt semantic canonicalization.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        let mut output = self.subject.to_string();

        if !self.bounds.is_empty() {
            output.push_str(\": \");

            for (index, bound) in self.bounds.iter().enumerate() {
                if index != 0 {
                    output.push_str(\" + \");
                }

                output.push_str(&bound.to_string());
            }
        }

        output
    }
}

impl fmt::Display for WherePredicate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

/// Canonical source-level generic `where` clause.
///
/// A clause is an ordered collection of [`WherePredicate`] values.
///
/// The collection is intentionally unbounded by the AST representation itself.
/// Compiler resource policies may impose configurable limits when parsing,
/// validating or deserializing untrusted input.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WhereClause {
    /// Predicates in source order.
    predicates: Vec<WherePredicate>,
}

impl WhereClause {
    /// Creates an empty `where` clause.
    ///
    /// An empty clause is structurally representable so parser construction
    /// and transformations can be performed incrementally. Whether an empty
    /// clause is syntactically legal is a parser/grammar concern.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a clause from ordered predicates.
    #[must_use]
    pub fn from_predicates(predicates: Vec<WherePredicate>) -> Self {
        Self { predicates }
    }

    /// Returns all predicates in source order.
    #[must_use]
    pub fn predicates(&self) -> &[WherePredicate] {
        &self.predicates
    }

    /// Returns a predicate by index.
    #[must_use]
    pub fn predicate(&self, index: usize) -> Option<&WherePredicate> {
        self.predicates.get(index)
    }

    /// Returns an iterator over predicates in source order.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &WherePredicate> {
        self.predicates.iter()
    }

    /// Returns the number of predicates.
    #[must_use]
    pub fn len(&self) -> usize {
        self.predicates.len()
    }

    /// Returns whether the clause contains no predicates.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.predicates.is_empty()
    }

    /// Appends a predicate while preserving source order.
    pub fn push(&mut self, predicate: WherePredicate) {
        self.predicates.push(predicate);
    }

    /// Extends the clause using iterator order.
    pub fn extend<I>(&mut self, predicates: I)
    where
        I: IntoIterator<Item = WherePredicate>,
    {
        self.predicates.extend(predicates);
    }

    /// Removes all predicates.
    pub fn clear(&mut self) {
        self.predicates.clear();
    }

    /// Consumes the clause and returns its predicates.
    #[must_use]
    pub fn into_predicates(self) -> Vec<WherePredicate> {
        self.predicates
    }

    /// Returns the schema version of this representation.
    #[must_use]
    pub const fn schema_version() -> u16 {
        WHERE_CLAUSE_SCHEMA_VERSION
    }

    /// Returns the stable source-level AST construct name.
    #[must_use]
    pub const fn kind_name() -> &'static str {
        "where_clause"
    }

    /// Validates the complete clause using the canonical default type policy.
    ///
    /// Validation is structural only.
    pub fn validate(&self) -> Result<(), TypeExprError> {
        self.validate_with_policy(&TypeValidationPolicy::default())
    }

    /// Validates the complete clause using an explicit compiler policy.
    ///
    /// No semantic constraint resolution is performed.
    pub fn validate_with_policy(
        &self,
        policy: &TypeValidationPolicy,
    ) -> Result<(), TypeExprError> {
        if let Some(maximum) = policy.max_collection_items {
            if self.predicates.len() > maximum {
                return Err(TypeExprError::CollectionTooLarge {
                    actual: self.predicates.len(),
                    maximum,
                });
            }
        }

        for predicate in &self.predicates {
            predicate.validate_with_policy(policy)?;
        }

        Ok(())
    }

    /// Returns whether any predicate contains an inference placeholder.
    #[must_use]
    pub fn requires_inference(&self) -> bool {
        self.predicates
            .iter()
            .any(WherePredicate::requires_inference)
    }

    /// Returns whether any predicate contains generic type expressions.
    #[must_use]
    pub fn contains_generic_types(&self) -> bool {
        self.predicates
            .iter()
            .any(WherePredicate::contains_generic_types)
    }

    /// Returns the number of source-level bounds across all predicates.
    ///
    /// This performs a linear traversal and deliberately does not cache a
    /// mutable derived count inside the AST.
    #[must_use]
    pub fn total_bound_count(&self) -> usize {
        self.predicates
            .iter()
            .map(WherePredicate::bound_count)
            .sum()
    }

    /// Produces deterministic source-level text.
    ///
    /// Semantic normalization is intentionally excluded.
    #[must_use]
    pub fn to_source_string(&self) -> String {
        if self.predicates.is_empty() {
            return String::new();
        }

        let mut output = String::from("where ");

        for (index, predicate) in self.predicates.iter().enumerate() {
            if index != 0 {
                output.push_str(\", \");
            }

            output.push_str(&predicate.to_source_string());
        }

        output
    }
}

impl fmt::Display for WhereClause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_source_string())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn generic(name: &str) -> TypeExpr {
        TypeExpr::GenericParameter(TypeParameterName::new(name))
    }

    fn named(name: &str) -> TypeExpr {
        TypeExpr::Identifier(
            super::super::super::types::type_expr::TypePath::single(name),
        )
    }

    #[test]
    fn empty_clause_is_constructible() {
        let clause = WhereClause::new();

        assert!(clause.is_empty());
        assert_eq!(clause.len(), 0);
        assert_eq!(clause.total_bound_count(), 0);
    }

    #[test]
    fn predicate_preserves_bound_order() {
        let predicate = WherePredicate::with_bounds(
            generic("T"),
            vec![named("First"), named("Second"), named("Third")],
        );

        assert_eq!(predicate.bound_count(), 3);
        assert_eq!(
            predicate.bound(0),
            Some(&named("First"))
        );
        assert_eq!(
            predicate.bound(1),
            Some(&named("Second"))
        );
        assert_eq!(
            predicate.bound(2),
            Some(&named("Third"))
        );
    }

    #[test]
    fn clause_preserves_predicate_order() {
        let first = WherePredicate::with_bounds(
            generic("T"),
            vec![named("Numeric")],
        );

        let second = WherePredicate::with_bounds(
            generic("Q"),
            vec![named("QuantumResource")],
        );

        let clause = WhereClause::from_predicates(vec![first, second]);

        assert_eq!(clause.len(), 2);
        assert_eq!(
            clause.predicate(0).and_then(WherePredicate::parameter_name)
                .map(TypeParameterName::as_str),
            Some("T")
        );
        assert_eq!(
            clause.predicate(1).and_then(WherePredicate::parameter_name)
                .map(TypeParameterName::as_str),
            Some("Q")
        );
    }

    #[test]
    fn parameter_constructor_creates_generic_subject() {
        let predicate =
            WherePredicate::for_parameter(TypeParameterName::new("T"));

        assert!(predicate.subject_is_generic_parameter());
        assert_eq!(
            predicate.parameter_name().map(TypeParameterName::as_str),
            Some("T")
        );
    }

    #[test]
    fn empty_predicate_is_structurally_constructible() {
        let predicate = WherePredicate::new(generic("T"));

        assert!(predicate.is_unbounded());
        assert_eq!(predicate.bound_count(), 0);
    }

    #[test]
    fn source_formatting_is_deterministic() {
        let clause = WhereClause::from_predicates(vec![
            WherePredicate::with_bounds(
                generic("T"),
                vec![named("Numeric"), named("Ordered")],
            ),
            WherePredicate::with_bounds(
                generic("Q"),
                vec![named("QuantumResource")],
            ),
        ]);

        assert_eq!(
            clause.to_source_string(),
            "where T: Numeric + Ordered, Q: QuantumResource"
        );
    }

    #[test]
    fn validation_accepts_valid_structure() {
        let clause = WhereClause::from_predicates(vec![
            WherePredicate::with_bounds(
                generic("T"),
                vec![named("Numeric")],
            ),
        ]);

        assert!(clause.validate().is_ok());
    }

    #[test]
    fn validation_uses_explicit_collection_policy() {
        let clause = WhereClause::from_predicates(vec![
            WherePredicate::new(generic("T")),
            WherePredicate::new(generic("U")),
        ]);

        let policy = TypeValidationPolicy {
            max_collection_items: Some(1),
            ..TypeValidationPolicy::default()
        };

        let result = clause.validate_with_policy(&policy);

        assert!(matches!(
            result,
            Err(TypeExprError::CollectionTooLarge {
                actual: 2,
                maximum: 1
            })
        ));
    }

    #[test]
    fn no_hardware_information_is_stored() {
        let predicate = WherePredicate::with_bounds(
            generic("Q"),
            vec![named("QuantumResource")],
        );

        let clause = WhereClause::from_predicates(vec![predicate]);

        assert_eq!(clause.len(), 1);
        assert_eq!(clause.total_bound_count(), 1);
    }

    #[test]
    fn round_trip_serialization_preserves_structure() {
        let clause = WhereClause::from_predicates(vec![
            WherePredicate::with_bounds(
                generic("T"),
                vec![named("Numeric"), named("Ordered")],
            ),
        ]);

        let encoded =
            serde_json::to_string(&clause).expect("serialization must succeed");

        let decoded: WhereClause =
            serde_json::from_str(&encoded).expect("deserialization must succeed");

        assert_eq!(decoded, clause);
    }
}