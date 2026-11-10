
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Standard Library — Constraint Contracts
//!
//! Path: `src/stdlib/constraint.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## PURPOSE
//!
//! Defines portable, extensible standard-library contracts for mandatory
//! constraints.
//!
//! A constraint expresses a restriction that acceptable semantic
//! interpretations or realizations must satisfy.
//!
//! Constraints are independent of:
//!
//! - processor architecture;
//! - CPU, GPU, FPGA, ASIC, and QPU availability;
//! - quantum topology;
//! - memory capacity;
//! - register width;
//! - tensor rank;
//! - distributed node count;
//! - operating system;
//! - execution provider;
//! - physical hardware;
//! - future computational domains.
//!
//! ## ARCHITECTURAL AUTHORITY
//!
//! This module owns:
//!
//! - ConstraintKey.
//! - Constraint.
//! - ConstraintOrigin.
//! - ConstraintSet.
//! - ConstraintError.
//! - ConstraintComponent.
//! - Local declaration validation.
//! - Deterministic collection inspection.
//!
//! This module does NOT own:
//!
//! - Constraint grammar.
//! - Constraint AST nodes.
//! - Source expression semantics.
//! - Name resolution.
//! - Type checking.
//! - Resource quantities.
//! - Resource expressions.
//! - Capability definitions.
//! - Requirement evaluation.
//! - Preference scoring.
//! - Hint interpretation.
//! - Constraint satisfiability.
//! - Constraint solving.
//! - Feasibility analysis.
//! - Policy authorization.
//! - Resource allocation.
//! - Hardware discovery.
//! - Provider selection.
//! - Scheduling.
//! - Routing.
//! - Execution.
//! - IR construction.
//!
//! ## CANONICAL AUTHORITIES
//!
//! Grammar:
//!     grammar/core/constraints.g4
//!
//! Resource-specific constraint syntax:
//!     grammar/resources/constraints.g4
//!
//! Policy-specific constraint binding:
//!     grammar/policies/constraints.g4
//!
//! Semantic model:
//!     grammar/specification/semantic-model.md
//!
//! Canonical semantic resource constraints:
//!     src/frontend/semantic/resources/model.rs
//!
//! Canonical semantic capabilities:
//!     src/frontend/semantic/capabilities/model.rs
//!
//! Standard-library requirements:
//!     src/stdlib/requirement.rs
//!
//! Standard-library capabilities:
//!     src/stdlib/capability.rs
//!
//! Standard-library preferences:
//!     src/stdlib/preference.rs
//!
//! Standard-library errors:
//!     src/stdlib/error.rs
//!
//! ## FUNDAMENTAL DISTINCTIONS
//!
//! Requirement:
//!     A mandatory condition that must be available or satisfied.
//!
//! Capability:
//!     An ability an environment or implementation can provide.
//!
//! Preference:
//!     A desirable, non-mandatory property.
//!
//! Constraint:
//!     A mandatory restriction on acceptable outcomes.
//!
//! Hint:
//!     Non-binding implementation guidance.
//!
//! Feasibility:
//!     An evidence-based assessment of whether conditions can be satisfied.
//!
//! These concepts must never be silently converted into one another.
//!
//! ## SEMANTIC PAYLOAD
//!
//! Constraint<T> stores its semantic predicate as T.
//!
//! T is supplied by the canonical semantic subsystem or an explicit adapter.
//!
//! This module does not parse, reinterpret, evaluate, normalize, or solve T.
//!
//! This prevents the standard library from creating a competing expression
//! language or competing semantic model.
//!
//! ## POCO-REAF
//!
//! Constraint identities are open-world.
//!
//! New domains and constraint categories do not require modifying a closed
//! enum in this module.
//!
//! No arbitrary maximum exists for:
//!
//! - constraint declarations;
//! - namespace depth;
//! - identifier length;
//! - collection size;
//! - source metadata entries;
//! - hardware resources;
//! - computational domains.
//!
//! Actual growth is subject to available memory and explicitly configured
//! operational budgets outside this module.
//!
//! This does not promise literally infinite memory or computation.
//!
//! ## DETERMINISM
//!
//! Insertion order is preserved.
//!
//! Duplicate declarations are preserved.
//!
//! Canonicalized views use deterministic identity and metadata ordering.
//!
//! No HashMap iteration order is exposed as a deterministic contract.
//!
//! ## SAFETY
//!
//! Safe Rust only.
//!
//! No unsafe blocks.
//! No global mutable state.
//! No I/O.
//! No hardware discovery.
//! No implicit logging.
//! No printing.
//! No runtime initialization.
//! No hidden evaluation.
//! No panic-based validation.

use core::fmt;
use std::collections::BTreeMap;

// ============================================================================
// CONSTRAINT IDENTITY
// ============================================================================

/// An extensible identity for a constraint declaration.
///
/// The namespace may contain multiple `::`-separated components.
///
/// The name identifies one constraint within its namespace.
///
/// Examples:
///
/// - `classical::numeric::positive`
— `quantum::logical_qubits::minimum`
/// - `hdl::timing::clock_period`
/// - `distributed::communication::latency`
/// - `resource::memory::capacity`
/// - `ai::model::precision`
/// - `future::architecture::property`
///
/// Identity strings are preserved exactly.
///
/// This type does not perform:
///
/// - semantic name resolution;
/// - case folding;
/// - alias expansion;
/// - namespace import resolution;
/// - grammar parsing;
/// - provider lookup.
///
/// There is no arbitrary maximum identifier length or namespace depth.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConstraintKey {
    namespace: Option<String>,
    name: String,
}

impl ConstraintKey {
    /// Constructs a namespaced constraint identity.
    ///
    /// # Errors
    ///
    /// Returns [`ConstraintError`] if a component is malformed.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, ConstraintError> {
        let namespace = namespace.into();
        let name = name.into();

        validate_namespace(&namespace)?;

        validate_component(
            &name,
            ConstraintComponent::Name,
        )?;

        if name.contains("::") {
            return Err(ConstraintError::InvalidName {
                name,
                reason:
                    "constraint name must be one namespace component"
                        .to_owned(),
            });
        }

        Ok(Self {
            namespace: Some(namespace),
            name,
        })
    }

    /// Constructs an unqualified constraint identity.
    ///
    /// Unqualified identities are useful where the surrounding semantic
    /// scope supplies the namespace.
    ///
    /// Whether an unqualified identity is valid in a particular program
    /// is decided by semantic analysis.
    ///
    /// # Errors
    ///
    /// Returns [`ConstraintError`] if the name is malformed.
    pub fn unqualified(
        name: impl Into<String>,
    ) -> Result<Self, ConstraintError> {
        let name = name.into();

        validate_component(
            &name,
            ConstraintComponent::Name,
        )?;

        if name.contains("::") {
            return Err(ConstraintError::InvalidName {
                name,
                reason:
                    "unqualified constraint name must not contain \
                     a namespace separator"
                        .to_owned(),
            });
        }

        Ok(Self {
            namespace: None,
            name,
        })
    }

    /// Constructs a constraint identity from a qualified textual name.
    ///
    /// The final `::` separates the namespace and name.
    ///
    /// Examples:
    ///
    /// - `latency`
    /// - `quantum::fidelity`
    /// - `future::architecture::property`
    ///
    /// This is an identity parser, not an implementation of Zamani grammar.
    ///
    /// # Errors
    ///
    /// Returns [`ConstraintError`] if the identity is malformed.
    pub fn from_qualified_name(
        value: impl AsRef<str>,
    ) -> Result<Self, ConstraintError> {
        let value = value.as_ref();

        match value.rsplit_once("::") {
            Some((namespace, name)) => {
                Self::new(namespace, name)
            }

            None => {
                Self::unqualified(value)
            }
        }
    }

    /// Returns the optional namespace.
    #[must_use]
    pub fn namespace(&self) -> Option<&str> {
        self.namespace.as_deref()
    }

    /// Returns the unqualified constraint name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the identity has an explicit namespace.
    #[must_use]
    pub fn is_namespaced(&self) -> bool {
        self.namespace.is_some()
    }

    /// Returns the qualified textual representation.
    ///
    /// Identity comparisons should use [`ConstraintKey`] directly.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        match &self.namespace {
            Some(namespace) => {
                format!("{namespace}::{}", self.name)
            }

            None => self.name.clone(),
        }
    }

    /// Checks an identity without allocating a temporary key.
    #[must_use]
    pub fn matches(
        &self,
        namespace: Option<&str>,
        name: &str,
    ) -> bool {
        self.namespace() == namespace
            && self.name == name
    }
}

impl fmt::Display for ConstraintKey {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match &self.namespace {
            Some(namespace) => {
                write!(
                    formatter,
                    "{namespace}::{}",
                    self.name
                )
            }

            None => {
                formatter.write_str(&self.name)
            }
        }
    }
}

impl TryFrom<&str> for ConstraintKey {
    type Error = ConstraintError;

    fn try_from(
        value: &str,
    ) -> Result<Self, Self::Error> {
        Self::from_qualified_name(value)
    }
}

impl TryFrom<String> for ConstraintKey {
    type Error = ConstraintError;

    fn try_from(
        value: String,
    ) -> Result<Self, Self::Error> {
        Self::from_qualified_name(value)
    }
}

// ============================================================================
// CONSTRAINT PROVENANCE
// ============================================================================

/// Provenance associated with a constraint declaration.
///
/// Provenance fields are intentionally opaque.
///
/// The frontend may preserve its own source identifiers, AST node identities,
/// source spans, semantic producers, or transformation information.
///
/// This type does not import frontend-specific types and does not replace
/// canonical source-location or semantic-provenance authorities.
///
/// A source range uses an inclusive start byte and exclusive end byte.
#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub struct ConstraintOrigin {
    source_id: Option<String>,
    node_id: Option<String>,
    start_byte: Option<u64>,
    end_byte: Option<u64>,
    producer: Option<String>,
}

impl ConstraintOrigin {
    /// Creates an empty provenance record.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_id: None,
            node_id: None,
            start_byte: None,
            end_byte: None,
            producer: None,
        }
    }

    /// Sets an opaque source identifier.
    #[must_use]
    pub fn with_source_id(
        mut self,
        source_id: impl Into<String>,
    ) -> Self {
        self.source_id = Some(source_id.into());
        self
    }

    /// Sets an opaque AST or semantic node identifier.
    #[must_use]
    pub fn with_node_id(
        mut self,
        node_id: impl Into<String>,
    ) -> Self {
        self.node_id = Some(node_id.into());
        self
    }

    /// Sets the producer of this declaration.
    ///
    /// Examples:
    ///
    /// - `frontend.semantic`
    /// - `stdlib`
    /// - `resource.semantic`
    /// - `policy.adapter`
    #[must_use]
    pub fn with_producer(
        mut self,
        producer: impl Into<String>,
    ) -> Self {
        self.producer = Some(producer.into());
        self
    }

    /// Sets the source byte range.
    ///
    /// The start is inclusive and the end is exclusive.
    ///
    /// # Errors
    ///
    /// Returns [`ConstraintError::InvalidSourceRange`] if the end precedes
    /// the start.
    pub fn with_source_range(
        mut self,
        start_byte: u64,
        end_byte: u64,
    ) -> Result<Self, ConstraintError> {
        if end_byte < start_byte {
            return Err(
                ConstraintError::InvalidSourceRange {
                    start: start_byte,
                    end: end_byte,
                },
            );
        }

        self.start_byte = Some(start_byte);
        self.end_byte = Some(end_byte);

        Ok(self)
    }

    /// Returns the source identifier.
    #[must_use]
    pub fn source_id(&self) -> Option<&str> {
        self.source_id.as_deref()
    }

    /// Returns the source node identifier.
    #[must_use]
    pub fn node_id(&self) -> Option<&str> {
        self.node_id.as_deref()
    }

    /// Returns the source byte range.
    #[must_use]
    pub const fn source_range(
        &self,
    ) -> Option<(u64, u64)> {
        match (self.start_byte, self.end_byte) {
            (Some(start), Some(end)) => {
                Some((start, end))
            }

            _ => None,
        }
    }

    /// Returns the declaration producer.
    #[must_use]
    pub fn producer(&self) -> Option<&str> {
        self.producer.as_deref()
    }

    /// Validates the local provenance invariants.
    ///
    /// # Errors
    ///
    /// Returns an error if the source range is incomplete or invalid.
    pub fn validate(&self) -> Result<(), ConstraintError> {
        match (self.start_byte, self.end_byte) {
            (Some(start), Some(end)) if end >= start => Ok(()),

            (None, None) => Ok(()),

            (Some(start), Some(end)) => {
                Err(ConstraintError::InvalidSourceRange {
                    start,
                    end,
                })
            }

            _ => {
                Err(ConstraintError::IncompleteSourceRange)
            }
        }
    }
}

// ============================================================================
// CONSTRAINT DECLARATION
// ============================================================================

/// A mandatory semantic constraint declaration.
///
/// `T` is the opaque, typed semantic predicate supplied by the canonical
/// semantic subsystem.
///
/// Examples of possible payloads include:
///
/// - a validated semantic expression;
/// - a semantic resource constraint;
/// - a type-bound predicate;
/// - a policy constraint expression;
/// - a domain-specific predicate.
///
/// This module does not interpret the payload.
///
/// A constraint does not become a preference or hint merely because its
/// evaluation is deferred.
///
/// A constraint declaration does not establish that the constraint is
/// satisfiable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Constraint<T> {
    key: Option<ConstraintKey>,
    predicate: T,
    rationale: Option<String>,
    origin: ConstraintOrigin,
}

impl<T> Constraint<T> {
    /// Creates an anonymous constraint.
    ///
    /// Anonymous constraints are useful for ordinary constraint expressions
    /// that do not have an independently declared name.
    #[must_use]
    pub fn new(predicate: T) -> Self {
        Self {
            key: None,
            predicate,
            rationale: None,
            origin: ConstraintOrigin::new(),
        }
    }

    /// Creates a named constraint.
    #[must_use]
    pub fn named(
        key: ConstraintKey,
        predicate: T,
    ) -> Self {
        Self {
            key: Some(key),
            predicate,
            rationale: None,
            origin: ConstraintOrigin::new(),
        }
    }

    /// Creates a named constraint from namespace and name components.
    ///
    /// # Errors
    ///
    /// Returns [`ConstraintError`] if the identity is malformed.
    pub fn from_parts(
        namespace: impl Into<String>,
        name: impl Into<String>,
        predicate: T,
    ) -> Result<Self, ConstraintError> {
        Ok(Self::named(
            ConstraintKey::new(namespace, name)?,
            predicate,
        ))
    }

    /// Returns the optional constraint identity.
    #[must_use]
    pub fn key(&self) -> Option<&ConstraintKey> {
        self.key.as_ref()
    }

    /// Returns the semantic predicate.
    ///
    /// No interpretation or evaluation is performed.
    #[must_use]
    pub fn predicate(&self) -> &T {
        &self.predicate
    }

    /// Returns mutable access to the semantic payload.
    ///
    /// Mutation is the responsibility of the semantic payload's owner.
    pub fn predicate_mut(&mut self) -> &mut T {
        &mut self.predicate
    }

    /// Replaces the semantic payload.
    pub fn set_predicate(
        &mut self,
        predicate: T,
    ) {
        self.predicate = predicate;
    }

    /// Consumes the declaration and returns its semantic payload.
    #[must_use]
    pub fn into_predicate(self) -> T {
        self.predicate
    }

    /// Returns the optional explanatory rationale.
    #[must_use]
    pub fn rationale(&self) -> Option<&str> {
        self.rationale.as_deref()
    }

    /// Adds explanatory rationale.
    ///
    /// Rationale is metadata. It cannot change the meaning of the predicate.
    ///
    /// # Errors
    ///
    /// Returns [`ConstraintError::EmptyRationale`] for empty or whitespace-only
    /// rationale.
    pub fn with_rationale(
        mut self,
        rationale: impl Into<String>,
    ) -> Result<Self, ConstraintError> {
        let rationale = rationale.into();

        validate_rationale(&rationale)?;

        self.rationale = Some(rationale);

        Ok(self)
    }

    /// Returns the provenance of this declaration.
    #[must_use]
    pub fn origin(&self) -> &ConstraintOrigin {
        &self.origin
    }

    /// Replaces the provenance.
    ///
    /// # Errors
    ///
    /// Returns an error if the provenance is invalid.
    pub fn with_origin(
        mut self,
        origin: ConstraintOrigin,
    ) -> Result<Self, ConstraintError> {
        origin.validate()?;

        self.origin = origin;

        Ok(self)
    }

    /// Returns the stable identity, if the declaration is named.
    ///
    /// Anonymous declarations have no independent declaration identity.
    #[must_use]
    pub fn canonical_key(&self) -> Option<&ConstraintKey> {
        self.key.as_ref()
    }

    /// Returns whether this declaration has an explicit identity.
    #[must_use]
    pub fn is_named(&self) -> bool {
        self.key.is_some()
    }

    /// Returns the semantic payload and metadata without interpreting them.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        Option<ConstraintKey>,
        T,
        Option<String>,
        ConstraintOrigin,
    ) {
        (
            self.key,
            self.predicate,
            self.rationale,
            self.origin,
        )
    }
}

impl<T> fmt::Display for Constraint<T> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match &self.key {
            Some(key) => key.fmt(formatter),

            None => formatter.write_str("<anonymous constraint>"),
        }
    }
}

// ============================================================================
// CONSTRAINT COLLECTION
// ============================================================================

/// An ordered collection of constraint declarations.
///
/// The collection preserves:
///
/// - insertion order;
/// - anonymous declarations;
/// - named declarations;
/// - duplicate identities;
/// - potentially contradictory predicates;
/// - independently sourced declarations.
///
/// It does not:
///
/// - deduplicate constraints;
/// - merge expressions;
/// - normalize predicates;
/// - infer logical relationships;
/// - solve constraints;
/// - decide satisfiability;
/// - decide feasibility.
///
/// The surrounding semantic subsystem determines the meaning of combining
/// declarations.
///
/// There is no fixed collection capacity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstraintSet<T> {
    constraints: Vec<Constraint<T>>,
}

impl<T> Default for ConstraintSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ConstraintSet<T> {
    /// Creates an empty collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    /// Creates a collection from an iterator.
    ///
    /// Insertion order and duplicates are preserved.
    #[must_use]
    pub fn from_constraints(
        constraints: impl IntoIterator<Item = Constraint<T>>,
    ) -> Self {
        Self {
            constraints: constraints.into_iter().collect(),
        }
    }

    /// Adds a constraint declaration.
    pub fn push(
        &mut self,
        constraint: Constraint<T>,
    ) {
        self.constraints.push(constraint);
    }

    /// Adds a declaration and returns the updated collection.
    #[must_use]
    pub fn with(
        mut self,
        constraint: Constraint<T>,
    ) -> Self {
        self.push(constraint);
        self
    }

    /// Appends another collection.
    ///
    /// Existing order and duplicate declarations are preserved.
    pub fn extend(
        &mut self,
        other: Self,
    ) {
        self.constraints.extend(other.constraints);
    }

    /// Appends declarations from an iterator.
    pub fn extend_from(
        &mut self,
        constraints: impl IntoIterator<Item = Constraint<T>>,
    ) {
        self.constraints.extend(constraints);
    }

    /// Returns the number of declarations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Returns whether the collection is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    /// Returns declarations in insertion order.
    pub fn iter(
        &self,
    ) -> core::slice::Iter<'_, Constraint<T>> {
        self.constraints.iter()
    }

    /// Returns a declaration by insertion-order index.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Constraint<T>> {
        self.constraints.get(index)
    }

    /// Returns mutable access to a declaration by index.
    pub fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut Constraint<T>> {
        self.constraints.get_mut(index)
    }

    /// Returns named declarations belonging to a namespace.
    ///
    /// Anonymous declarations are excluded.
    ///
    /// Insertion order is preserved.
    pub fn for_namespace<'a>(
        &'a self,
        namespace: &'a str,
    ) -> impl Iterator<Item = &'a Constraint<T>> + 'a {
        self.constraints
            .iter()
            .filter(move |constraint| {
                constraint
                    .key()
                    .and_then(ConstraintKey::namespace)
                    == Some(namespace)
            })
    }

    /// Returns declarations matching an identity.
    ///
    /// All duplicates are returned in insertion order.
    pub fn matching<'a>(
        &'a self,
        key: &'a ConstraintKey,
    ) -> impl Iterator<Item = &'a Constraint<T>> + 'a {
        self.constraints
            .iter()
            .filter(move |constraint| {
                constraint.key() == Some(key)
            })
    }

    /// Returns whether a named declaration exists.
    ///
    /// Presence does not establish that its predicate is satisfied.
    #[must_use]
    pub fn contains(
        &self,
        key: &ConstraintKey,
    ) -> bool {
        self.constraints.iter().any(
            |constraint| constraint.key() == Some(key),
        )
    }

    /// Returns the number of named declarations.
    #[must_use]
    pub fn named_len(&self) -> usize {
        self.constraints
            .iter()
            .filter(|constraint| constraint.is_named())
            .count()
    }

    /// Returns the number of anonymous declarations.
    #[must_use]
    pub fn anonymous_len(&self) -> usize {
        self.constraints.len() - self.named_len()
    }

    /// Removes all declarations.
    ///
    /// This changes only this collection, not the semantic model or source.
    pub fn clear(&mut self) {
        self.constraints.clear();
    }

    /// Consumes the collection and returns its declarations.
    #[must_use]
    pub fn into_vec(self) -> Vec<Constraint<T>> {
        self.constraints
    }

    /// Returns a deterministically ordered view.
    ///
    /// Named constraints are ordered by identity and metadata.
    /// Anonymous constraints follow named constraints in their original
    /// relative order.
    ///
    /// The payload is intentionally excluded from sorting because this
    /// module cannot assume that arbitrary semantic payloads have a
    /// canonical serialization or ordering.
    #[must_use]
    pub fn canonicalized(
        &self,
    ) -> Vec<&Constraint<T>> {
        let mut named = Vec::new();
        let mut anonymous = Vec::new();

        for constraint in &self.constraints {
            if constraint.is_named() {
                named.push(constraint);
            } else {
                anonymous.push(constraint);
            }
        }

        named.sort_by(|left, right| {
            left.key()
                .cmp(&right.key())
                .then_with(|| {
                    left.rationale().cmp(&right.rationale())
                })
                .then_with(|| {
                    left.origin().cmp(right.origin())
                })
        });

        named.extend(anonymous);

        named
    }

    /// Reports groups of duplicate named identities.
    ///
    /// Returned indices refer to the original insertion order.
    ///
    /// Anonymous declarations are excluded.
    ///
    /// Duplicate reporting does not establish semantic equivalence,
    /// redundancy, or contradiction.
    #[must_use]
    pub fn duplicate_groups(
        &self,
    ) -> Vec<(ConstraintKey, Vec<usize>)> {
        let mut groups: BTreeMap<
            ConstraintKey,
            Vec<usize>,
        > = BTreeMap::new();

        for (index, constraint) in
            self.constraints.iter().enumerate()
        {
            if let Some(key) = constraint.key() {
                groups
                    .entry(key.clone())
                    .or_default()
                    .push(index);
            }
        }

        groups
            .into_iter()
            .filter(|(_, indices)| indices.len() > 1)
            .collect()
    }

    /// Retains declarations satisfying a caller-supplied selection predicate.
    ///
    /// This is structural filtering only. It does not evaluate semantic
    /// constraint predicates.
    #[must_use]
    pub fn filter(
        &self,
        mut predicate: impl FnMut(&Constraint<T>) -> bool,
    ) -> Vec<&Constraint<T>> {
        self.constraints
            .iter()
            .filter(|constraint| predicate(constraint))
            .collect()
    }
}

impl<T> IntoIterator for ConstraintSet<T> {
    type Item = Constraint<T>;
    type IntoIter = std::vec::IntoIter<Constraint<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.constraints.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a ConstraintSet<T> {
    type Item = &'a Constraint<T>;
    type IntoIter = core::slice::Iter<'a, Constraint<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.constraints.iter()
    }
}

// ============================================================================
// VALIDATION ERRORS
// ============================================================================

/// Identifies the component that failed constraint validation.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ConstraintComponent {
    /// Constraint namespace.
    Namespace,

    /// Constraint name.
    Name,

    /// Human-readable rationale.
    Rationale,

    /// Provenance producer.
    Producer,

    /// Source identifier.
    SourceId,

    /// Source node identifier.
    NodeId,
}

impl fmt::Display for ConstraintComponent {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Namespace => formatter.write_str("namespace"),
            Self::Name => formatter.write_str("name"),
            Self::Rationale => formatter.write_str("rationale"),
            Self::Producer => formatter.write_str("producer"),
            Self::SourceId => formatter.write_str("source identifier"),
            Self::NodeId => formatter.write_str("node identifier"),
        }
    }
}

/// Local validation errors for constraint declarations.
///
/// These errors indicate malformed local declaration data.
///
/// They do not indicate:
///
/// - an unsatisfied constraint;
/// - an infeasible program;
/// - a missing capability;
/// - an unavailable resource;
/// - an unauthorized operation;
/// - a target incompatibility.
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConstraintError {
    /// A required component was empty or whitespace-only.
    EmptyComponent {
        component: ConstraintComponent,
    },

    /// A component contained leading or trailing whitespace.
    SurroundingWhitespace {
        component: ConstraintComponent,
    },

    /// A component contained a control character.
    ControlCharacter {
        component: ConstraintComponent,
    },

    /// A namespace contained an empty segment.
    EmptyNamespaceSegment,

    /// A constraint name contained invalid namespace syntax.
    InvalidName {
        name: String,
        reason: String,
    },

    /// Explanatory rationale was empty.
    EmptyRationale,

    /// A source range ended before it started.
    InvalidSourceRange {
        start: u64,
        end: u64,
    },

    /// Only one endpoint of a source range was supplied.
    IncompleteSourceRange,
}

impl fmt::Display for ConstraintError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EmptyComponent { component } => {
                write!(
                    formatter,
                    "constraint {component} must not be empty"
                )
            }

            Self::SurroundingWhitespace { component } => {
                write!(
                    formatter,
                    "constraint {component} must not have \
                     leading or trailing whitespace"
                )
            }

            Self::ControlCharacter { component } => {
                write!(
                    formatter,
                    "constraint {component} must not contain \
                     control characters"
                )
            }

            Self::EmptyNamespaceSegment => {
                formatter.write_str(
                    "constraint namespace contains an empty segment",
                )
            }

            Self::InvalidName { name, reason } => {
                write!(
                    formatter,
                    "invalid constraint name '{name}': {reason}"
                )
            }

            Self::EmptyRationale => {
                formatter.write_str(
                    "constraint rationale must not be empty",
                )
            }

            Self::InvalidSourceRange { start, end } => {
                write!(
                    formatter,
                    "invalid constraint source range: \
                     end {end} precedes start {start}"
                )
            }

            Self::IncompleteSourceRange => {
                formatter.write_str(
                    "constraint source range must have \
                     both start and end positions",
                )
            }
        }
    }
}

impl std::error::Error for ConstraintError {}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

fn validate_component(
    value: &str,
    component: ConstraintComponent,
) -> Result<(), ConstraintError> {
    if value.trim().is_empty() {
        return Err(
            ConstraintError::EmptyComponent {
                component,
            },
        );
    }

    if value.trim() != value {
        return Err(
            ConstraintError::SurroundingWhitespace {
                component,
            },
        );
    }

    if value.chars().any(char::is_control) {
        return Err(
            ConstraintError::ControlCharacter {
                component,
            },
        );
    }

    Ok(())
}

fn validate_namespace(
    namespace: &str,
) -> Result<(), ConstraintError> {
    validate_component(
        namespace,
        ConstraintComponent::Namespace,
    )?;

    if namespace
        .split("::")
        .any(str::is_empty)
    {
        return Err(
            ConstraintError::EmptyNamespaceSegment,
        );
    }

    Ok(())
}

fn validate_rationale(
    rationale: &str,
) -> Result<(), ConstraintError> {
    validate_component(
        rationale,
        ConstraintComponent::Rationale,
    )
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_namespaced_constraint_key() {
        let key = ConstraintKey::new(
            "quantum::logical_qubits",
            "minimum",
        )
        .expect("valid identity");

        assert_eq!(
            key.qualified_name(),
            "quantum::logical_qubits::minimum"
        );

        assert_eq!(
            key.namespace(),
            Some("quantum::logical_qubits")
        );

        assert_eq!(key.name(), "minimum");
    }

    #[test]
    fn creates_unqualified_constraint_key() {
        let key = ConstraintKey::unqualified("latency")
            .expect("valid identity");

        assert_eq!(key.namespace(), None);
        assert_eq!(key.name(), "latency");
        assert_eq!(key.to_string(), "latency");
    }

    #[test]
    fn parses_qualified_identity() {
        let key = ConstraintKey::from_qualified_name(
            "future::architecture::property",
        )
        .expect("valid identity");

        assert_eq!(
            key.namespace(),
            Some("future::architecture")
        );

        assert_eq!(key.name(), "property");
    }

    #[test]
    fn rejects_empty_namespace() {
        assert!(matches!(
            ConstraintKey::new("", "minimum"),
            Err(ConstraintError::EmptyComponent {
                component: ConstraintComponent::Namespace
            })
        ));
    }

    #[test]
    fn rejects_empty_namespace_segments() {
        assert!(matches!(
            ConstraintKey::new(
                "quantum::::hardware",
                "minimum"
            ),
            Err(ConstraintError::EmptyNamespaceSegment)
        ));
    }

    #[test]
    fn rejects_surrounding_whitespace() {
        assert!(matches!(
            ConstraintKey::unqualified(" latency "),
            Err(ConstraintError::SurroundingWhitespace {
                component: ConstraintComponent::Name
            })
        ));
    }

    #[test]
    fn rejects_control_characters() {
        assert!(matches!(
            ConstraintKey::unqualified("latency\n"),
            Err(ConstraintError::ControlCharacter {
                component: ConstraintComponent::Name
            })
        ));
    }

    #[test]
    fn rejects_qualified_name_as_unqualified_name() {
        assert!(matches!(
            ConstraintKey::unqualified(
                "quantum::fidelity"
            ),
            Err(ConstraintError::InvalidName { .. })
        ));
    }

    #[test]
    fn creates_anonymous_constraint() {
        let constraint = Constraint::new(
            "value >= minimum",
        );

        assert!(!constraint.is_named());
        assert_eq!(constraint.predicate(), &"value >= minimum");
    }

    #[test]
    fn creates_named_constraint() {
        let key = ConstraintKey::new(
            "classical::numeric",
            "positive",
        )
        .expect("valid identity");

        let constraint = Constraint::named(
            key.clone(),
            "value > 0",
        );

        assert!(constraint.is_named());
        assert_eq!(constraint.key(), Some(&key));
    }

    #[test]
    fn rationale_is_validated() {
        let constraint = Constraint::new("value > 0")
            .with_rationale(
                "The value must remain positive",
            )
            .expect("valid rationale");

        assert_eq!(
            constraint.rationale(),
            Some("The value must remain positive")
        );

        assert!(matches!(
            Constraint::new("value > 0")
                .with_rationale("   "),
            Err(ConstraintError::EmptyRationale)
        ));
    }

    #[test]
    fn provenance_validates_source_range() {
        let origin = ConstraintOrigin::new()
            .with_source_id("module.zm")
            .with_node_id("node-1")
            .with_producer("frontend.semantic")
            .with_source_range(10, 25)
            .expect("valid range");

        assert_eq!(
            origin.source_range(),
            Some((10, 25))
        );

        assert!(matches!(
            ConstraintOrigin::new()
                .with_source_range(25, 10),
            Err(ConstraintError::InvalidSourceRange {
                start: 25,
                end: 10
            })
        ));
    }

    #[test]
    fn provenance_rejects_incomplete_range() {
        let origin = ConstraintOrigin {
            start_byte: Some(10),
            ..ConstraintOrigin::new()
        };

        assert_eq!(
            origin.validate(),
            Err(ConstraintError::IncompleteSourceRange)
        );
    }

    #[test]
    fn set_preserves_insertion_order_and_duplicates() {
        let key = ConstraintKey::unqualified("minimum")
            .expect("valid identity");

        let first = Constraint::named(
            key.clone(),
            "value >= 10",
        );

        let second = Constraint::named(
            key,
            "value >= 20",
        );

        let set = ConstraintSet::from_constraints([
            first,
            Constraint::new("other condition"),
            second,
        ]);

        assert_eq!(set.len(), 3);
        assert_eq!(set.named_len(), 2);
        assert_eq!(set.anonymous_len(), 1);

        assert_eq!(
            set.get(1).expect("entry").predicate(),
            &"other condition"
        );
    }

    #[test]
    fn duplicate_groups_preserve_original_indices() {
        let key = ConstraintKey::unqualified("capacity")
            .expect("valid identity");

        let set = ConstraintSet::from_constraints([
            Constraint::named(
                key.clone(),
                "memory >= required",
            ),
            Constraint::new("independent condition"),
            Constraint::named(
                key.clone(),
                "memory <= budget",
            ),
        ]);

        let duplicates = set.duplicate_groups();

        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].0, key);
        assert_eq!(duplicates[0].1, vec![0, 2]);
    }

    #[test]
    fn namespace_filter_preserves_order() {
        let set = ConstraintSet::from_constraints([
            Constraint::named(
                ConstraintKey::new(
                    "quantum",
                    "fidelity",
                )
                .expect("valid key"),
                "fidelity >= minimum",
            ),
            Constraint::named(
                ConstraintKey::new(
                    "classical",
                    "precision",
                )
                .expect("valid key"),
                "precision >= required",
            ),
            Constraint::named(
                ConstraintKey::new(
                    "quantum",
                    "logical_qubits",
                )
                .expect("valid key"),
                "logical_qubits >= required",
            ),
        ]);

        let quantum: Vec<_> = set
            .for_namespace("quantum")
            .collect();

        assert_eq!(quantum.len(), 2);

        assert_eq!(
            quantum[0].key().expect("named").name(),
            "fidelity"
        );

        assert_eq!(
            quantum[1].key().expect("named").name(),
            "logical_qubits"
        );
    }

    #[test]
    fn canonicalized_view_is_deterministic() {
        let set = ConstraintSet::from_constraints([
            Constraint::named(
                ConstraintKey::unqualified("zeta")
                    .expect("valid key"),
                "z",
            ),
            Constraint::named(
                ConstraintKey::unqualified("alpha")
                    .expect("valid key"),
                "a",
            ),
            Constraint::new("anonymous"),
        ]);

        let ordered = set.canonicalized();

        assert_eq!(
            ordered[0].key().expect("named").name(),
            "alpha"
        );

        assert_eq!(
            ordered[1].key().expect("named").name(),
            "zeta"
        );

        assert!(!ordered[2].is_named());
    }

    #[test]
    fn contains_checks_identity_not_satisfaction() {
        let key = ConstraintKey::new(
            "resource::memory",
            "capacity",
        )
        .expect("valid key");

        let set = ConstraintSet::from_constraints([
            Constraint::named(
                key.clone(),
                "memory >= required",
            ),
        ]);

        assert!(set.contains(&key));
    }

    #[test]
    fn semantic_payload_is_generic() {
        #[derive(Clone, Debug, Eq, PartialEq)]
        struct SemanticPredicate {
            expression_id: String,
            resolved: bool,
        }

        let predicate = SemanticPredicate {
            expression_id: "node-42".to_owned(),
            resolved: false,
        };

        let constraint = Constraint::new(predicate.clone());

        assert_eq!(
            constraint.predicate(),
            &predicate
        );
    }

    #[test]
    fn empty_set_is_valid() {
        let set: ConstraintSet<String> =
            ConstraintSet::new();

        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
        assert!(set.duplicate_groups().is_empty());
    }

    #[test]
    fn extension_preserves_declarations() {
        let mut first = ConstraintSet::new();

        first.push(Constraint::new("first"));

        let second = ConstraintSet::from_constraints([
            Constraint::new("second"),
            Constraint::new("third"),
        ]);

        first.extend(second);

        assert_eq!(first.len(), 3);
        assert_eq!(
            first.get(2).expect("third").predicate(),
            &"third"
        );
    }
}
