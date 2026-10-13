#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Zamani frontend semantic analysis — type constraints.
//!
//! Path: src/frontend/semantic/types/constraints.rs
//! Edition: Rust 2021
//! Minimum Rust version: Rust 1.97
//!
//! PURPOSE
//!
//! Represent, validate, collect, order, and classify semantic type
//! obligations without confusing an unresolved obligation with a proof.
//!
//! ARCHITECTURAL AUTHORITY
//!
//! - Canonical source syntax: frontend::ast::node::types::type_expr
//! - Resolved type meaning: super::ty::SemanticType
//! - Type equality and inference-variable substitution: super::unification
//! - Assignability and compatibility: super::compatibility
//! - Expression inference: super::inference
//! - Generic instantiation: super::generics
//! - Constant evaluation: super::constants
//! - Normative semantic rules: grammar/specification/semantic-model.md
//!
//! RESPONSIBILITIES
//!
//! - Stable, checked identities for constraints.
//! - Explicit representations of type and type-level obligations.
//! - Deterministic storage, lookup, and iteration.
//! - Explicit proof states: proven, disproven, or unresolved.
//! - Explicit recording of unresolved obligations for later passes.
//! - Safe, configurable work accounting.
//! - Extension-friendly constraints for future semantic domains.
//!
//! DOES NOT OWN
//!
//! - AST construction, parsing, or name resolution.
//! - Type inference or type-variable allocation.
//! - Type unification or assignability rules.
//! - Trait implementation lookup.
//! - Constant-expression evaluation.
//! - Effect inference or authorization.
//! - Capability or resource feasibility.
//! - Ownership, lifetime, or borrow checking algorithms.
//! - Quantum operation semantics, HDL elaboration, or tensor operators.
//! - Hardware selection, scheduling, routing, lowering, or runtime execution.
//!
//! INTEGRATION CONTRACT
//!
//! Inference may add constraints while analyzing expressions.
//! Unification may discharge type-equality obligations.
//! Compatibility may discharge assignability obligations.
//! Generic and trait resolution may discharge trait obligations.
//! Constant evaluation may discharge type-level predicates.
//! Domain validators may add namespaced extension obligations.
//! Final semantic validation must reject unresolved obligations that the
//! language requires to be proven before a program can be certified valid.
//!
//! This module never treats an unknown, error, symbolic, or unresolved value
//! as proof of correctness.
//!
//! SCALABILITY
//!
//! There are no fixed hardware capacities, domain counts, generic-arity
//! limits, or symbolic-dimension ceilings. Decimal values and symbolic
//! expressions are retained in the canonical type model.
//!
//! A finite process is necessarily bounded by its available memory and time.
//! Optional work budgets therefore represent compiler operating policy,
//! never a source-language restriction. Budget exhaustion is explicit.
//!
//! DETERMINISM
//!
//! Constraints are stored in BTreeMap order by stable constraint identity.
//! No result depends on hash-map iteration order.
//!
//! SAFETY
//!
//! No unsafe code, global mutable state, unchecked arithmetic, or silent
//! identity wraparound.
//!
//! FREEZE CRITERIA
//!
//! - Canonical semantic types are used without defining a second type system.
//! - Constraint identity exhaustion is reported rather than wrapped.
//! - Malformed inputs and unresolved obligations remain explicit.
//! - Constraint collection and result ordering are deterministic.
//! - Type equality and assignability are delegated to their owning modules.
//! - Integration tests prove that inference, unification, compatibility,
//!   generics, constants, and domain validators can exchange obligations.
//! - The complete workspace and semantic conformance suite pass.

use core::fmt;
use std::collections::BTreeMap;
use std::error::Error;

use super::ty::{Cardinality, QualifiedName, SemanticName, SemanticType, TypeLevelValue};

/// Stable identity of a constraint within one constraint collection.
///
/// This ID identifies an analysis obligation, not a type variable, AST node,
/// hardware resource, or globally persistent semantic entity.
///
/// IDs are local to a `ConstraintSet`. To persist constraints across analysis
/// sessions, the caller must also persist the owning program/session identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstraintId(u64);

impl ConstraintId {
    /// Returns the numeric value for diagnostics and serialization adapters.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Checked allocator for constraint identities.
///
/// Allocation is monotonic. Exhaustion is reported and never wraps or reuses
/// an existing identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstraintIdAllocator {
    next: u64,
}

impl Default for ConstraintIdAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstraintIdAllocator {
    /// Creates an allocator starting at identity zero.
    #[must_use]
    pub const fn new() -> Self {
        Self { next: 0 }
    }

    /// Allocates an identity or reports exhaustion.
    pub fn allocate(&mut self) -> Result<ConstraintId, ConstraintError> {
        let id = ConstraintId(self.next);
        self.next = self
            .next
            .checked_add(1)
            .ok_or(ConstraintError::IdentityExhausted)?;
        Ok(id)
    }

    /// Returns the next identity that would be allocated.
    #[must_use]
    pub const fn next_value(&self) -> u64 {
        self.next
    }
}

/// Relationship between two resolved semantic types.
///
/// These are obligations, not implementations of the corresponding rules.
/// The owning type subsystem determines whether a relationship is valid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TypeRelation {
    /// Both types must denote the same type.
    Equal,

    /// The actual type must be accepted where the expected type is required.
    AssignableTo,

    /// The actual type must satisfy the language-defined subtype relation.
    SubtypeOf,

    /// The two types must be disjoint under the language's type rules.
    Disjoint,

    /// The type must be well-formed in its declaration and use context.
    WellFormed,
}

/// Comparison between type-level values.
///
/// The constant-evaluation subsystem or a dedicated symbolic solver owns
/// the proof rules. This module merely records the requested relation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValueRelation {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

/// A type-level operand retained symbolically.
///
/// A cardinality is used for lengths and dimensions. A general value supports
/// other dependent-type expressions without forcing them into `usize` or
/// another host-sized integer.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeLevelOperand {
    Cardinality(Cardinality),
    Value(TypeLevelValue),
}

impl TypeLevelOperand {
    /// Validates the operand's canonical representation.
    pub fn validate(&self) -> Result<(), ConstraintError> {
        match self {
            Self::Cardinality(value) => value
                .validate()
                .map_err(|error| ConstraintError::InvalidOperand(error.to_string())),
            Self::Value(value) => value
                .validate()
                .map_err(|error| ConstraintError::InvalidOperand(error.to_string())),
        }
    }
}

/// A constraint that can be consumed by the appropriate semantic pass.
///
/// The variants are deliberately target-independent. Domain-specific
/// obligations use `Extension` rather than requiring a new core variant
/// whenever a new computational domain is introduced.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum TypeConstraint {
    /// Relates two semantic types.
    TypeRelation {
        relation: TypeRelation,
        left: SemanticType,
        right: SemanticType,
    },

    /// Requires a type to satisfy a declared trait or interface.
    ///
    /// Trait identity is a qualified semantic name. Existence of the trait,
    /// implementation lookup, and coherence are owned by trait resolution.
    Trait {
        ty: SemanticType,
        trait_name: QualifiedName,
        arguments: Vec<SemanticType>,
    },

    /// Relates symbolic or concrete type-level values.
    ///
    /// Values may remain symbolic until a permitted evaluation or proof pass
    /// can establish the relationship.
    ValueRelation {
        relation: ValueRelation,
        left: TypeLevelOperand,
        right: TypeLevelOperand,
    },

    /// Requires one lifetime or region to outlive another.
    ///
    /// The lifetime names must have been resolved in the appropriate scope.
    /// This record does not itself prove the outlives relationship.
    Outlives {
        longer: SemanticName,
        shorter: SemanticName,
    },

    /// Requires a type-level proposition to be established by a registered
    /// semantic extension.
    ///
    /// `namespace` and `name` identify the proposition. Arguments preserve
    /// canonical semantic types. Extension handlers must be registered and
    /// validated by the semantic pipeline; unknown extensions are unresolved,
    /// not implicitly satisfied.
    Extension {
        namespace: QualifiedName,
        name: SemanticName,
        arguments: Vec<SemanticType>,
    },
}

impl TypeConstraint {
    /// Validates the structural integrity of a constraint.
    ///
    /// This does not establish that the obligation is true.
    pub fn validate(&self) -> Result<(), ConstraintError> {
        match self {
            Self::TypeRelation { left, right, .. } => {
                validate_type(left)?;
                validate_type(right)
            }
            Self::Trait {
                ty,
                trait_name,
                arguments,
            } => {
                validate_type(ty)?;
                trait_name
                    .validate()
                    .map_err(|error| ConstraintError::InvalidOperand(error.to_string()))?;
                for argument in arguments {
                    validate_type(argument)?;
                }
                Ok(())
            }
            Self::ValueRelation { left, right, .. } => {
                left.validate()?;
                right.validate()
            }
            Self::Outlives { longer, shorter } => {
                longer
                    .validate()
                    .map_err(|error| ConstraintError::InvalidOperand(error.to_string()))?;
                shorter
                    .validate()
                    .map_err(|error| ConstraintError::InvalidOperand(error.to_string()))
            }
            Self::Extension {
                namespace,
                name,
                arguments,
            } => {
                namespace
                    .validate()
                    .map_err(|error| ConstraintError::InvalidOperand(error.to_string()))?;
                name.validate()
                    .map_err(|error| ConstraintError::InvalidOperand(error.to_string()))?;
                for argument in arguments {
                    validate_type(argument)?;
                }
                Ok(())
            }
        }
    }
}

fn validate_type(ty: &SemanticType) -> Result<(), ConstraintError> {
    // Error types are permitted to travel through recovery so that a compiler
    // can report multiple diagnostics. They are not accepted as proof.
    //
    // SemanticType::is_incomplete() is a useful fast check, but does not
    // replace recursive structural validation of every canonical type value.
    if ty.is_error() {
        return Err(ConstraintError::ErrorTypeCannotProveConstraint);
    }

    if ty.is_incomplete() {
        return Err(ConstraintError::IncompleteType);
    }

    Ok(())
}

/// State of a constraint after an evaluation attempt.
///
/// `Unresolved` is a first-class result. It must not be converted into
/// `Proven` simply to allow compilation to continue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstraintStatus {
    Proven,

    Disproven {
        reason: String,
    },

    Unresolved {
        reason: UnresolvedReason,
    },
}

/// Why an obligation has not yet been discharged.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum UnresolvedReason {
    /// One or more type variables still require inference.
    TypeInference,

    /// A type-level value is symbolic or has not been evaluated.
    SymbolicValue,

    /// Trait or interface implementation lookup has not completed.
    TraitResolution,

    /// A lifetime or region relationship has not been established.
    LifetimeResolution,

    /// A registered domain extension must evaluate the obligation.
    ExtensionResolution,

    /// The required semantic service is unavailable in this analysis context.
    MissingResolver,

    /// An explicitly configured compiler work budget was exhausted.
    WorkBudgetExceeded,

    /// The obligation depends on information available only in a later pass.
    DeferredToLaterPass,
}

/// One stored semantic obligation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constraint {
    id: ConstraintId,
    obligation: TypeConstraint,
    status: ConstraintStatus,
    origin: Option<String>,
}

impl Constraint {
    /// Returns the constraint identity.
    #[must_use]
    pub const fn id(&self) -> ConstraintId {
        self.id
    }

    /// Returns the obligation.
    #[must_use]
    pub fn obligation(&self) -> &TypeConstraint {
        &self.obligation
    }

    /// Returns the latest evaluation status.
    #[must_use]
    pub fn status(&self) -> &ConstraintStatus {
        &self.status
    }

    /// Returns optional diagnostic/provenance text supplied by the caller.
    ///
    /// Prefer canonical AST node identities and source spans in the semantic
    /// model when those services are available. This text is not an identity.
    #[must_use]
    pub fn origin(&self) -> Option<&str> {
        self.origin.as_deref()
    }

    /// Whether this constraint has been proven.
    #[must_use]
    pub fn is_proven(&self) -> bool {
        matches!(self.status, ConstraintStatus::Proven)
    }

    /// Whether this constraint still requires evaluation.
    #[must_use]
    pub fn is_unresolved(&self) -> bool {
        matches!(self.status, ConstraintStatus::Unresolved { .. })
    }
}

/// Optional operational budget for constraint operations.
///
/// `None` means that this collection imposes no additional work limit.
/// The semantic session should normally supply a budget from its configuration
/// when processing untrusted or generated input.
///
/// A budget exhaustion result is incomplete analysis, never successful proof.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ConstraintConfig {
    pub max_operations: Option<usize>,
}

/// Deterministic collection of semantic type obligations.
///
/// This type owns constraint storage and lifecycle state only. It deliberately
/// does not own a unifier, trait solver, constant evaluator, or domain registry.
#[derive(Clone, Debug, Default)]
pub struct ConstraintSet {
    ids: ConstraintIdAllocator,
    constraints: BTreeMap<ConstraintId, Constraint>,
    config: ConstraintConfig,
    operations: usize,
}

impl ConstraintSet {
    /// Creates a collection without an additional operation budget.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a collection with an explicit operational budget.
    pub fn with_config(config: ConstraintConfig) -> Result<Self, ConstraintError> {
        if config.max_operations == Some(0) {
            return Err(ConstraintError::InvalidConfiguration);
        }

        Ok(Self {
            ids: ConstraintIdAllocator::new(),
            constraints: BTreeMap::new(),
            config,
            operations: 0,
        })
    }

    /// Adds an obligation in the unresolved state.
    ///
    /// Structural validation happens before allocation, so malformed input
    /// does not consume an identity. New obligations are never implicitly
    /// proven.
    pub fn insert(
        &mut self,
        obligation: TypeConstraint,
        origin: Option<String>,
    ) -> Result<ConstraintId, ConstraintError> {
        self.charge_operation()?;
        obligation.validate()?;

        let id = self.ids.allocate()?;
        let constraint = Constraint {
            id,
            obligation,
            status: ConstraintStatus::Unresolved {
                reason: UnresolvedReason::DeferredToLaterPass,
            },
            origin,
        };

        self.constraints.insert(id, constraint);
        Ok(id)
    }

    /// Returns a constraint by identity.
    #[must_use]
    pub fn get(&self, id: ConstraintId) -> Option<&Constraint> {
        self.constraints.get(&id)
    }

    /// Returns the number of stored obligations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    /// Returns whether no obligations are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    /// Iterates in deterministic identity order.
    pub fn iter(&self) -> impl Iterator<Item = &Constraint> {
        self.constraints.values()
    }

    /// Iterates over unresolved obligations in deterministic identity order.
    pub fn unresolved(&self) -> impl Iterator<Item = &Constraint> {
        self.constraints
            .values()
            .filter(|constraint| constraint.is_unresolved())
    }

    /// Iterates over disproven obligations in deterministic identity order.
    pub fn disproven(&self) -> impl Iterator<Item = &Constraint> {
        self.constraints.values().filter(|constraint| {
            matches!(constraint.status, ConstraintStatus::Disproven { .. })
        })
    }

    /// Returns whether every stored obligation has been proven.
    ///
    /// An empty collection is vacuously proven. Callers must separately ensure
    /// that all required analysis passes ran and that analysis did not abort.
    #[must_use]
    pub fn all_proven(&self) -> bool {
        self.constraints
            .values()
            .all(|constraint| constraint.is_proven())
    }

    /// Returns the number of unresolved obligations.
    #[must_use]
    pub fn unresolved_count(&self) -> usize {
        self.unresolved().count()
    }

    /// Returns the number of disproven obligations.
    #[must_use]
    pub fn disproven_count(&self) -> usize {
        self.disproven().count()
    }

    /// Records an evaluation result produced by the owning semantic solver.
    ///
    /// This method does not itself prove the obligation. The caller must only
    /// pass `Proven` after the authoritative solver has established the rule.
    pub fn record_status(
        &mut self,
        id: ConstraintId,
        status: ConstraintStatus,
    ) -> Result<(), ConstraintError> {
        self.charge_operation()?;

        if matches!(status, ConstraintStatus::Unresolved { .. }) {
            // Unresolved remains unresolved; it is never promoted implicitly.
        }

        let constraint = self
            .constraints
            .get_mut(&id)
            .ok_or(ConstraintError::UnknownConstraint(id))?;

        constraint.status = status;
        Ok(())
    }

    /// Resets an obligation to unresolved when a dependency changes.
    ///
    /// Incremental invalidation should call this for affected obligations
    /// rather than preserving a proof derived from stale dependencies.
    pub fn invalidate(
        &mut self,
        id: ConstraintId,
        reason: UnresolvedReason,
    ) -> Result<(), ConstraintError> {
        self.record_status(id, ConstraintStatus::Unresolved { reason })
    }

    /// Returns the configured operation budget.
    #[must_use]
    pub const fn config(&self) -> ConstraintConfig {
        self.config
    }

    /// Returns the number of charged collection operations.
    #[must_use]
    pub const fn operations(&self) -> usize {
        self.operations
    }

    fn charge_operation(&mut self) -> Result<(), ConstraintError> {
        if let Some(maximum) = self.config.max_operations {
            if self.operations >= maximum {
                return Err(ConstraintError::WorkBudgetExceeded { maximum });
            }
        }

        self.operations = self
            .operations
            .checked_add(1)
            .ok_or(ConstraintError::OperationCounterExhausted)?;

        Ok(())
    }
}

/// Failures raised by constraint representation and collection.
///
/// Semantic contradictions are represented as `ConstraintStatus::Disproven`.
/// This error type is for malformed input, collection lifecycle errors, and
/// operational failures.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConstraintError {
    IdentityExhausted,
    OperationCounterExhausted,
    InvalidConfiguration,
    InvalidOperand(String),
    IncompleteType,
    ErrorTypeCannotProveConstraint,
    UnknownConstraint(ConstraintId),
    WorkBudgetExceeded { maximum: usize },
}

impl fmt::Display for ConstraintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdentityExhausted => {
                formatter.write_str("constraint identity space exhausted")
            }
            Self::OperationCounterExhausted => {
                formatter.write_str("constraint operation counter exhausted")
            }
            Self::InvalidConfiguration => {
                formatter.write_str("invalid constraint configuration")
            }
            Self::InvalidOperand(message) => {
                write!(formatter, "invalid constraint operand: {message}")
            }
            Self::IncompleteType => {
                formatter.write_str(
                    "an incomplete semantic type cannot establish a constraint",
                )
            }
            Self::ErrorTypeCannotProveConstraint => {
                formatter.write_str(
                    "an error-recovery type cannot establish a constraint",
                )
            }
            Self::UnknownConstraint(id) => {
                write!(formatter, "unknown constraint identity {}", id.get())
            }
            Self::WorkBudgetExceeded { maximum } => {
                write!(
                    formatter,
                    "constraint operation budget exhausted ({maximum})"
                )
            }
        }
    }
}

impl Error for ConstraintError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn named_type(name: &str) -> SemanticType {
        SemanticType::named(QualifiedName::single(name))
    }

    #[test]
    fn identities_are_monotonic() {
        let mut allocator = ConstraintIdAllocator::new();

        let first = allocator.allocate().expect("first ID");
        let second = allocator.allocate().expect("second ID");

        assert!(first < second);
        assert_eq!(first.get(), 0);
        assert_eq!(second.get(), 1);
    }

    #[test]
    fn constraints_start_unresolved() {
        let mut constraints = ConstraintSet::new();

        let id = constraints
            .insert(
                TypeConstraint::TypeRelation {
                    relation: TypeRelation::Equal,
                    left: named_type("example::Left"),
                    right: named_type("example::Right"),
                },
                None,
            )
            .expect("valid obligation");

        let constraint = constraints.get(id).expect("stored obligation");
        assert!(constraint.is_unresolved());
        assert!(!constraint.is_proven());
        assert!(!constraints.all_proven());
    }

    #[test]
    fn unresolved_obligations_cannot_be_reported_as_proven() {
        let mut constraints = ConstraintSet::new();

        let id = constraints
            .insert(
                TypeConstraint::TypeRelation {
                    relation: TypeRelation::Equal,
                    left: named_type("example::Left"),
                    right: named_type("example::Right"),
                },
                None,
            )
            .expect("valid obligation");

        constraints
            .record_status(
                id,
                ConstraintStatus::Unresolved {
                    reason: UnresolvedReason::SymbolicValue,
                },
            )
            .expect("status update");

        assert!(!constraints.all_proven());
        assert_eq!(constraints.unresolved_count(), 1);
    }

    #[test]
    fn proven_status_requires_an_explicit_solver_result() {
        let mut constraints = ConstraintSet::new();

        let id = constraints
            .insert(
                TypeConstraint::TypeRelation {
                    relation: TypeRelation::Equal,
                    left: named_type("example::Value"),
                    right: named_type("example::Value"),
                },
                None,
            )
            .expect("valid obligation");

        constraints
            .record_status(id, ConstraintStatus::Proven)
            .expect("authoritative solver result");

        assert!(constraints.all_proven());
    }

    #[test]
    fn invalidation_removes_a_previous_proof() {
        let mut constraints = ConstraintSet::new();

        let id = constraints
            .insert(
                TypeConstraint::TypeRelation {
                    relation: TypeRelation::Equal,
                    left: named_type("example::Value"),
                    right: named_type("example::Value"),
                },
                None,
            )
            .expect("valid obligation");

        constraints
            .record_status(id, ConstraintStatus::Proven)
            .expect("proof");
        constraints
            .invalidate(id, UnresolvedReason::TypeInference)
            .expect("invalidation");

        assert!(!constraints.all_proven());
        assert_eq!(constraints.unresolved_count(), 1);
    }

    #[test]
    fn operation_budget_is_explicit() {
        let mut constraints = ConstraintSet::with_config(ConstraintConfig {
            max_operations: Some(1),
        })
        .expect("valid budget");

        constraints
            .insert(
                TypeConstraint::TypeRelation {
                    relation: TypeRelation::Equal,
                    left: named_type("example::A"),
                    right: named_type("example::A"),
                },
                None,
            )
            .expect("first operation");

        let result = constraints.insert(
            TypeConstraint::TypeRelation {
                relation: TypeRelation::Equal,
                left: named_type("example::B"),
                right: named_type("example::B"),
            },
            None,
        );

        assert!(matches!(
            result,
            Err(ConstraintError::WorkBudgetExceeded { maximum: 1 })
        ));
    }

    #[test]
    fn iteration_is_deterministic() {
        let mut constraints = ConstraintSet::new();

        for name in ["third", "first", "second"] {
            constraints
                .insert(
                    TypeConstraint::TypeRelation {
                        relation: TypeRelation::WellFormed,
                        left: named_type(name),
                        right: named_type(name),
                    },
                    None,
                )
                .expect("valid obligation");
        }

        let ids: Vec<_> = constraints.iter().map(Constraint::id).collect();
        assert!(ids.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn error_types_do_not_prove_constraints() {
        let mut constraints = ConstraintSet::new();

        let result = constraints.insert(
            TypeConstraint::TypeRelation {
                relation: TypeRelation::Equal,
                left: SemanticType::Error,
                right: named_type("example::Value"),
            },
            None,
        );

        assert!(matches!(
            result,
            Err(ConstraintError::ErrorTypeCannotProveConstraint)
        ));
    }

    #[test]
    fn symbolic_values_are_preserved_without_machine_integer_conversion() {
        let symbolic = TypeLevelValue::integer(
            "999999999999999999999999999999999999999999999999999999",
        )
        .expect("arbitrary-length decimal");

        let operand = TypeLevelOperand::Value(symbolic.clone());
        operand.validate().expect("valid symbolic-scale value");

        assert_eq!(operand, TypeLevelOperand::Value(symbolic));
    }

    #[test]
    fn unknown_constraint_ids_are_reported() {
        let mut constraints = ConstraintSet::new();

        let result = constraints.record_status(
            ConstraintId(123),
            ConstraintStatus::Proven,
        );

        assert!(matches!(
            result,
            Err(ConstraintError::UnknownConstraint(_))
        ));
    }
}