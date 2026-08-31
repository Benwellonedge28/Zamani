//! Zamani Quantum IR — Execution Guards
//!
//! Production-grade, target-independent representation of conditional
//! execution guards for quantum, classical, pulse, analog, logical,
//! distributed, and hybrid programs.
//!
//! # Architectural role
//!
//! A [`Guard`] expresses:
//!
//! > Under what classical condition may a previously-defined IR operation
//! > execute?
//!
//! The guard is intentionally a reference-level abstraction:
//!
//! ```text
//! ClassicalPredicate
//!        │
//!        ▼
//!     Condition
//!        │
//!        ▼
//!      Guard ───────────────► OperationId
//!        │
//!        ▼
//! conditional execution
//! ```
//!
//! `guard.rs` does NOT own:
//!
//! - classical predicate semantics;
//! - logical qubit identity;
//! - operation semantics;
//! - operation storage;
//! - source-language ASTs;
//! - quantum state;
//! - physical qubits;
//! - routing;
//! - scheduling;
//! - hardware;
//! - calibration;
//! - pulse generation;
//! - simulation;
//! - backend execution;
//! - QEC decoding.
//!
//! Those responsibilities belong to their respective IR/compiler layers.
//!
//! # Canonical ownership
//!
//! The dependency boundary is:
//!
//! ```text
//! classical::predicate
//!          │
//!          ▼
//! control::condition
//!          │
//!          ▼
//! control::guard
//!          │
//!          ├────────► identity::OperationId
//!          │
//!          └────────► analysis / validation / program
//! ```
//!
//! The canonical logical-qubit identity remains:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! A guard does not duplicate `QubitId` because the guarded operation already
//! owns its semantic operands. This prevents the guard from becoming a second,
//! potentially inconsistent source of quantum-operand truth.
//!
//! # Why Guard is separate from Condition
//!
//! [`Condition`](super::condition::Condition) answers:
//!
//! > Is this Boolean condition satisfied?
//!
//! [`Guard`] answers:
//!
//! > Which IR operation is conditionally permitted to execute under that
//! > condition?
//!
//! Keeping these concepts separate allows the same condition to be reused by
//! different operations without making the classical predicate layer aware of
//! operations.
//!
//! # Why Guard is separate from Operation
//!
//! The operation layer deliberately uses stable [`OperationId`] references for
//! conditional execution rather than recursively embedding operations.
//!
//! Therefore:
//!
//! ```text
//! Guard {
//!     condition,
//!     target: OperationId,
//! }
//! ```
//!
//! is preferable to:
//!
//! ```text
//! Guard {
//!     operation: Operation,
//! }
//! ```
//!
//! This prevents recursive heap structures, simplifies serialization,
//! preserves stable operation identity, and permits very large IR graphs.
//!
//! # Universal-program principle
//!
//! There is no architectural maximum for:
//!
//! - the number of guards;
//! - the number of operations;
//! - the number of classical dependencies;
//! - the number of qubits affected by guarded operations;
//! - the number of nested control-flow regions;
//! - the number of operations sharing the same condition.
//!
//! Any finite limit must come from an explicit compiler/resource policy.
//!
//! The IR itself remains capable of representing any finite program permitted
//! by available resources.
//!
//! "Infinity" therefore means:
//!
//! ```text
//! no artificial fixed machine-size ceiling
//! ```
//!
//! rather than an attempt to allocate an actually infinite Rust object.
//!
//! # Determinism
//!
//! Guards contain no unordered semantic collection. Dependency information is
//! delegated to [`Condition`] and returned through deterministic ordered sets.
//!
//! # No unsafe
//!
//! This module contains no unsafe code and enforces that contract at compile
//! time.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies.
//!
//! # Integration contract
//!
//! `control/mod.rs` should expose:
//!
//! ```text
//! pub mod guard;
//! ```
//!
//! The canonical condition implementation remains:
//!
//! ```text
//! control::condition::Condition
//! ```
//!
//! The canonical operation identity remains:
//!
//! ```text
//! identity::OperationId
//! ```
//!
//! The canonical logical-qubit identity remains:
//!
//! ```text
//! qubit::QubitId
//! ```
//!
//! No new qubit identity is introduced here.
//!
//! `operation.rs` may use `Guard` when the operation representation is migrated
//! to the structured program IR. Until then, the existing conditional
//! operation representation can remain source-compatible.
//!
//! `control_flow.rs` may use `Guard` for condition-dependent operation
//! references without embedding operation implementations.
//!
//! `validation.rs` can validate the target `OperationId` against the enclosing
//! program's operation table.
//!
//! `analysis.rs` can use the guard's dependency APIs to calculate classical
//! dependencies without inspecting predicate internals.
//!
//! `serialization.rs` can serialize the stable condition + operation identity.
//!
//! `hash.rs` can include the canonical guard representation in program hashing.
//!
//! `provenance.rs` can track transformations involving guarded operations.
//!
//! `dialect/*` may introduce dialect-specific guard attributes without changing
//! this core abstraction.
//!
//! # Important architectural rule
//!
//! A guard must never contain:
//!
//! - a physical qubit;
//! - a hardware ID;
//! - a backend handle;
//! - a scheduler timestamp;
//! - a pulse channel;
//! - a calibration object;
//! - a simulator state;
//! - a raw pointer;
//! - an execution callback.
//!
//! Such information would violate the canonical IR boundary.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt;

use super::condition::Condition;
use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::classical::bit::ClassicalBitId;
use crate::quantum::ir::identity::ValueId;

// =============================================================================
// Result
// =============================================================================

/// Result type used by guard construction and validation.
pub type GuardResult<T> = Result<T, GuardError>;

// =============================================================================
// Guard error
// =============================================================================

/// Errors produced by guard construction or local validation.
///
/// These errors deliberately concern only the guard abstraction itself.
/// Program-wide operation existence, qubit existence, resource availability,
/// and hardware capability validation belong to higher-level validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardError {
    /// A guard must have a target operation.
    MissingTarget,

    /// The supplied condition exceeds an explicitly supplied policy.
    ConditionLimitExceeded {
        /// Number of predicate nodes requested.
        requested: usize,

        /// Maximum permitted by the caller's validation policy.
        maximum: usize,
    },

    /// The guard exceeds an explicitly supplied validation policy.
    GuardLimitExceeded {
        /// Number of guard nodes requested.
        requested: usize,

        /// Maximum permitted by the caller's validation policy.
        maximum: usize,
    },

    /// A composition operation overflowed its explicitly supplied accounting.
    ArithmeticOverflow {
        /// Semantic calculation that overflowed.
        calculation: &'static str,
    },

    /// A structurally invalid guard was encountered.
    InvalidStructure {
        /// Static reason for the failure.
        reason: &'static str,
    },
}

impl fmt::Display for GuardError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingTarget => {
                formatter.write_str(
                    "execution guard requires a target operation",
                )
            }

            Self::ConditionLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "guard condition node limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::GuardLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "guard limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidStructure { reason } => {
                write!(
                    formatter,
                    "invalid execution guard: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for GuardError {}

// =============================================================================
// Validation policy
// =============================================================================

/// Explicit local validation policy for guards.
///
/// These values are NOT semantic limits of the Zamani language.
///
/// They are caller-selected resource/safety limits and should normally be
/// derived from the repository-wide IR compilation policy at the integration
/// boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GuardValidationPolicy {
    /// Maximum predicate nodes accepted by one guard.
    pub max_condition_nodes: usize,

    /// Maximum number of guard records accepted by a guard collection or
    /// validation pass.
    pub max_guards: usize,
}

impl GuardValidationPolicy {
    /// Creates an explicit guard validation policy.
    #[must_use]
    pub const fn new(
        max_condition_nodes: usize,
        max_guards: usize,
    ) -> Self {
        Self {
            max_condition_nodes,
            max_guards,
        }
    }

    /// Creates a policy with no finite application-level guard ceiling.
    ///
    /// This does not create infinite memory or infinite execution resources.
    /// It simply avoids imposing an architectural maximum.
    #[must_use]
    pub const fn unbounded() -> Self {
        Self {
            max_condition_nodes: usize::MAX,
            max_guards: usize::MAX,
        }
    }
}

impl Default for GuardValidationPolicy {
    fn default() -> Self {
        Self::unbounded()
    }
}

// =============================================================================
// Guard kind
// =============================================================================

/// Semantic classification of an execution guard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GuardKind {
    /// The target operation always executes.
    Always,

    /// The target operation never executes.
    Never,

    /// Execution depends on a non-constant classical predicate.
    Predicate,
}

impl fmt::Display for GuardKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Always => formatter.write_str("always"),
            Self::Never => formatter.write_str("never"),
            Self::Predicate => formatter.write_str("predicate"),
        }
    }
}

// =============================================================================
// Guard
// =============================================================================

/// Canonical execution guard for an existing IR operation.
///
/// A guard does not own the operation it guards. It references the operation
/// using a stable [`OperationId`].
///
/// This makes a guard suitable for:
///
/// - dynamic circuits;
/// - measurement-driven feedback;
/// - classical control;
/// - conditional gates;
/// - conditional measurements;
/// - conditional reset;
/// - conditional pulse operations;
/// - conditional analog operations;
/// - logical/fault-tolerant operations;
/// - distributed operations;
/// - future dialect operations.
///
/// The guard is target-independent.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guard {
    condition: Condition,
    target: OperationId,
}

impl Guard {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates a guard for an operation.
    ///
    /// The condition is normalized by the existing `Condition` abstraction.
    pub fn new(
        condition: Condition,
        target: OperationId,
    ) -> GuardResult<Self> {
        let guard = Self {
            condition,
            target,
        };

        guard.validate_local()?;

        Ok(guard)
    }

    /// Creates an unconditional guard.
    ///
    /// This is useful when APIs require a guard object even though the
    /// operation itself is unconditional.
    pub fn always(
        target: OperationId,
    ) -> GuardResult<Self> {
        Self::new(Condition::always(), target)
    }

    /// Creates a guard that can never execute its target.
    ///
    /// This is a valid semantic representation and is useful after compiler
    /// transformations that prove a branch unreachable.
    pub fn never(
        target: OperationId,
    ) -> GuardResult<Self> {
        Self::new(Condition::never(), target)
    }

    /// Creates a predicate guard.
    pub fn predicate(
        predicate: crate::quantum::ir::classical::predicate::ClassicalPredicate,
        target: OperationId,
    ) -> GuardResult<Self> {
        Self::new(
            Condition::from_predicate(predicate),
            target,
        )
    }

    /// Creates a guard without performing policy validation.
    ///
    /// The constructor still preserves the local structural representation.
    /// Call [`Self::validate`] before accepting the object at a compiler
    /// boundary.
    ///
    /// This is useful when deserializing an IR object whose resource policy is
    /// intentionally supplied later.
    #[must_use]
    pub const fn from_parts_unchecked(
        condition: Condition,
        target: OperationId,
    ) -> Self {
        Self {
            condition,
            target,
        }
    }

    // =========================================================================
    // Accessors
    // =========================================================================

    /// Returns the guard's execution condition.
    #[must_use]
    pub const fn condition(&self) -> &Condition {
        &self.condition
    }

    /// Returns the operation targeted by this guard.
    #[must_use]
    pub const fn target(&self) -> OperationId {
        self.target
    }

    /// Returns the semantic guard kind.
    #[must_use]
    pub const fn kind(&self) -> GuardKind {
        match self.condition.kind() {
            super::condition::ConditionKind::Always => GuardKind::Always,
            super::condition::ConditionKind::Never => GuardKind::Never,
            super::condition::ConditionKind::Predicate => {
                GuardKind::Predicate
            }
        }
    }

    /// Returns whether the target operation is unconditional.
    #[must_use]
    pub const fn is_always(&self) -> bool {
        self.condition.is_always()
    }

    /// Returns whether the target operation can never execute.
    #[must_use]
    pub const fn is_never(&self) -> bool {
        self.condition.is_never()
    }

    /// Returns whether the target operation has a dynamic classical
    /// predicate.
    #[must_use]
    pub const fn is_predicate(&self) -> bool {
        self.condition.is_predicate()
    }

    /// Returns whether the target may execute.
    #[must_use]
    pub const fn may_execute(&self) -> bool {
        self.condition.may_execute()
    }

    /// Returns whether the target is guaranteed to execute.
    #[must_use]
    pub const fn must_execute(&self) -> bool {
        self.condition.must_execute()
    }

    /// Returns the underlying predicate, if the guard is predicate-controlled.
    #[must_use]
    pub const fn as_predicate(
        &self,
    ) -> Option<
        &crate::quantum::ir::classical::predicate::ClassicalPredicate,
    > {
        self.condition.as_predicate()
    }

    // =========================================================================
    // Structural information
    // =========================================================================

    /// Returns the predicate depth of this guard.
    #[must_use]
    pub fn depth(&self) -> usize {
        self.condition.depth()
    }

    /// Returns the number of predicate nodes in this guard.
    #[must_use]
    pub fn condition_node_count(&self) -> usize {
        self.condition.node_count()
    }

    /// Returns the number of predicate terms in this guard.
    #[must_use]
    pub fn condition_term_count(&self) -> usize {
        self.condition.term_count()
    }

    /// Returns the classical-bit dependencies of the guard.
    ///
    /// Dependencies are returned in deterministic order.
    #[must_use]
    pub fn classical_dependencies(
        &self,
    ) -> BTreeSet<ClassicalBitId> {
        self.condition.classical_dependencies()
    }

    /// Returns IR value dependencies of the guard.
    ///
    /// This is useful for SSA/data-flow analysis.
    #[must_use]
    pub fn value_dependencies(&self) -> BTreeSet<ValueId> {
        self.condition.value_dependencies()
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Performs local structural validation with an unbounded policy.
    pub fn validate_local(&self) -> GuardResult<()> {
        self.validate(&GuardValidationPolicy::unbounded())
    }

    /// Validates this guard against an explicit resource policy.
    pub fn validate(
        &self,
        policy: &GuardValidationPolicy,
    ) -> GuardResult<()> {
        if self.condition.node_count() > policy.max_condition_nodes {
            return Err(
                GuardError::ConditionLimitExceeded {
                    requested: self.condition.node_count(),
                    maximum: policy.max_condition_nodes,
                },
            );
        }

        Ok(())
    }

    /// Validates a guard count against an explicit policy.
    ///
    /// This helper belongs here because guard collections may be represented
    /// by different higher-level program structures.
    pub fn validate_count(
        count: usize,
        policy: &GuardValidationPolicy,
    ) -> GuardResult<()> {
        if count > policy.max_guards {
            return Err(
                GuardError::GuardLimitExceeded {
                    requested: count,
                    maximum: policy.max_guards,
                },
            );
        }

        Ok(())
    }

    // =========================================================================
    // Condition transformations
    // =========================================================================

    /// Returns a copy of this guard with its condition logically negated.
    pub fn negate(self) -> Self {
        Self {
            condition: self.condition.not(),
            target: self.target,
        }
    }

    /// Returns a copy of this guard with its condition ANDed with another
    /// condition.
    pub fn with_condition_and(
        self,
        condition: Condition,
    ) -> GuardResult<Self> {
        let combined = self.condition.and(condition)?;

        Ok(Self {
            condition: combined,
            target: self.target,
        })
    }

    /// Returns a copy of this guard with its condition ORed with another
    /// condition.
    pub fn with_condition_or(
        self,
        condition: Condition,
    ) -> GuardResult<Self> {
        let combined = self.condition.or(condition)?;

        Ok(Self {
            condition: combined,
            target: self.target,
        })
    }

    /// Returns a copy of this guard with its condition XORed with another
    /// condition.
    pub fn with_condition_xor(
        self,
        condition: Condition,
    ) -> GuardResult<Self> {
        let combined = self.condition.xor(condition)?;

        Ok(Self {
            condition: combined,
            target: self.target,
        })
    }

    /// Returns a copy of this guard with an implication condition.
    ///
    /// Semantically:
    ///
    /// ```text
    /// old_condition -> new_condition
    /// ```
    pub fn implies_condition(
        self,
        condition: Condition,
    ) -> Self {
        Self {
            condition: self.condition.implies(condition),
            target: self.target,
        }
    }

    /// Returns a copy of this guard whose condition is equivalent to another
    /// condition.
    pub fn equivalent_condition(
        self,
        condition: Condition,
    ) -> Self {
        Self {
            condition: self.condition.equivalent(condition),
            target: self.target,
        }
    }

    // =========================================================================
    // Target transformation
    // =========================================================================

    /// Returns a copy of this guard targeting another operation.
    ///
    /// This does not mutate the original guard.
    pub const fn with_target(
        self,
        target: OperationId,
    ) -> Self {
        Self {
            condition: self.condition,
            target,
        }
    }

    // =========================================================================
    // Semantic comparison
    // =========================================================================

    /// Returns whether two guards target the same operation.
    #[must_use]
    pub const fn targets_same_operation(
        &self,
        other: &Self,
    ) -> bool {
        self.target == other.target
    }

    /// Returns whether two guards have exactly the same semantic condition.
    ///
    /// The comparison is structural. This function deliberately does not
    /// perform arbitrary Boolean theorem proving.
    #[must_use]
    pub fn same_condition(
        &self,
        other: &Self,
    ) -> bool {
        self.condition == other.condition
    }

    /// Returns whether two guards have exactly the same target and condition.
    #[must_use]
    pub fn semantically_equal(
        &self,
        other: &Self,
    ) -> bool {
        self.target == other.target
            && self.condition == other.condition
    }

    // =========================================================================
    // Composition
    // =========================================================================

    /// Combines two guards targeting the same operation using logical AND.
    ///
    /// The resulting guard is:
    ///
    /// ```text
    /// target executes when A AND B
    /// ```
    ///
    /// Guards targeting different operations cannot be combined because doing
    /// so would silently change the operation association.
    pub fn and(
        self,
        other: Self,
    ) -> GuardResult<Self> {
        if self.target != other.target {
            return Err(
                GuardError::InvalidStructure {
                    reason: "cannot AND guards targeting different operations",
                },
            );
        }

        let condition = self.condition.and(other.condition)?;

        Ok(Self {
            condition,
            target: self.target,
        })
    }

    /// Combines two guards targeting the same operation using logical OR.
    ///
    /// The resulting guard is:
    ///
    /// ```text
    /// target executes when A OR B
    /// ```
    pub fn or(
        self,
        other: Self,
    ) -> GuardResult<Self> {
        if self.target != other.target {
            return Err(
                GuardError::InvalidStructure {
                    reason: "cannot OR guards targeting different operations",
                },
            );
        }

        let condition = self.condition.or(other.condition)?;

        Ok(Self {
            condition,
            target: self.target,
        })
    }

    /// Combines two guards targeting the same operation using logical XOR.
    pub fn xor(
        self,
        other: Self,
    ) -> GuardResult<Self> {
        if self.target != other.target {
            return Err(
                GuardError::InvalidStructure {
                    reason: "cannot XOR guards targeting different operations",
                },
            );
        }

        let condition = self.condition.xor(other.condition)?;

        Ok(Self {
            condition,
            target: self.target,
        })
    }
}

// =============================================================================
// Guard collection
// =============================================================================

/// Deterministic collection of execution guards.
///
/// This type is intentionally backed by a `Vec` rather than a hash map:
///
/// - guard order can be meaningful to transformations;
/// - operation identity remains independent of position;
/// - serialization can preserve explicit order;
/// - no fixed capacity is introduced.
///
/// Higher-level program structures may instead index guards by `OperationId`
/// when that is appropriate for their own representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct GuardSet {
    guards: Vec<Guard>,
}

impl GuardSet {
    /// Creates an empty guard set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            guards: Vec::new(),
        }
    }

    /// Creates a guard set from an existing vector.
    ///
    /// No duplicate-elimination is performed because multiple guards for the
    /// same operation can represent distinct source-level or transformation
    /// information. Semantic combination is explicit through [`Guard::and`],
    /// [`Guard::or`], or [`Guard::xor`].
    #[must_use]
    pub fn from_vec(guards: Vec<Guard>) -> Self {
        Self { guards }
    }

    /// Returns the number of guards.
    #[must_use]
    pub fn len(&self) -> usize {
        self.guards.len()
    }

    /// Returns whether the collection contains no guards.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.guards.is_empty()
    }

    /// Returns a guard by positional index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Guard> {
        self.guards.get(index)
    }

    /// Returns the guard collection as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[Guard] {
        &self.guards
    }

    /// Adds one guard.
    ///
    /// No implicit architectural capacity is used.
    pub fn push(
        &mut self,
        guard: Guard,
    ) {
        self.guards.push(guard);
    }

    /// Attempts to add a guard under an explicit validation policy.
    pub fn try_push(
        &mut self,
        guard: Guard,
        policy: &GuardValidationPolicy,
    ) -> GuardResult<()> {
        let next_len = self
            .guards
            .len()
            .checked_add(1)
            .ok_or(GuardError::ArithmeticOverflow {
                calculation: "guard collection length",
            })?;

        if next_len > policy.max_guards {
            return Err(
                GuardError::GuardLimitExceeded {
                    requested: next_len,
                    maximum: policy.max_guards,
                },
            );
        }

        guard.validate(policy)?;
        self.guards.push(guard);

        Ok(())
    }

    /// Returns all classical dependencies used by all guards.
    ///
    /// Dependencies are deduplicated and returned in deterministic order.
    #[must_use]
    pub fn classical_dependencies(
        &self,
    ) -> BTreeSet<ClassicalBitId> {
        let mut dependencies = BTreeSet::new();

        for guard in &self.guards {
            dependencies.extend(
                guard.classical_dependencies(),
            );
        }

        dependencies
    }

    /// Returns all IR value dependencies used by all guards.
    ///
    /// Dependencies are deduplicated and returned in deterministic order.
    #[must_use]
    pub fn value_dependencies(
        &self,
    ) -> BTreeSet<ValueId> {
        let mut dependencies = BTreeSet::new();

        for guard in &self.guards {
            dependencies.extend(
                guard.value_dependencies(),
            );
        }

        dependencies
    }

    /// Returns all target operation IDs in deterministic order.
    #[must_use]
    pub fn target_operations(
        &self,
    ) -> BTreeSet<OperationId> {
        let mut operations = BTreeSet::new();

        for guard in &self.guards {
            operations.insert(guard.target());
        }

        operations
    }

    /// Validates every guard against an explicit policy.
    pub fn validate(
        &self,
        policy: &GuardValidationPolicy,
    ) -> GuardResult<()> {
        Self::validate_count(self.len(), policy)?;

        for guard in &self.guards {
            guard.validate(policy)?;
        }

        Ok(())
    }

    /// Returns an iterator over guards.
    pub fn iter(
        &self,
    ) -> std::slice::Iter<'_, Guard> {
        self.guards.iter()
    }

    /// Consumes the collection and returns the underlying vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<Guard> {
        self.guards
    }
}

impl<'a> IntoIterator for &'a GuardSet {
    type Item = &'a Guard;
    type IntoIter = std::slice::Iter<'a, Guard>;

    fn into_iter(self) -> Self::IntoIter {
        self.guards.iter()
    }
}

impl IntoIterator for GuardSet {
    type Item = Guard;
    type IntoIter = std::vec::IntoIter<Guard>;

    fn into_iter(self) -> Self::IntoIter {
        self.guards.into_iter()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quantum::ir::identity::OperationId;

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    #[test]
    fn always_guard_is_unconditional() {
        let guard = Guard::always(operation(1))
            .expect("always guard should be valid");

        assert!(guard.is_always());
        assert!(!guard.is_never());
        assert!(guard.may_execute());
        assert!(guard.must_execute());
        assert_eq!(guard.kind(), GuardKind::Always);
        assert_eq!(guard.target(), operation(1));
    }

    #[test]
    fn never_guard_is_unreachable() {
        let guard = Guard::never(operation(1))
            .expect("never guard should be valid");

        assert!(!guard.is_always());
        assert!(guard.is_never());
        assert!(!guard.may_execute());
        assert!(!guard.must_execute());
        assert_eq!(guard.kind(), GuardKind::Never);
    }

    #[test]
    fn guard_negation_is_target_preserving() {
        let original = Guard::always(operation(7))
            .expect("guard should be valid");

        let negated = original.negate();

        assert_eq!(negated.target(), operation(7));
        assert!(negated.is_never());
    }

    #[test]
    fn guard_and_requires_same_target() {
        let left = Guard::always(operation(1))
            .expect("guard should be valid");

        let right = Guard::never(operation(2))
            .expect("guard should be valid");

        let result = left.and(right);

        assert!(matches!(
            result,
            Err(GuardError::InvalidStructure { .. })
        ));
    }

    #[test]
    fn guard_and_same_target_constant_folds() {
        let left = Guard::always(operation(1))
            .expect("guard should be valid");

        let right = Guard::never(operation(1))
            .expect("guard should be valid");

        let combined = left
            .and(right)
            .expect("guards have the same target");

        assert!(combined.is_never());
        assert_eq!(combined.target(), operation(1));
    }

    #[test]
    fn guard_or_same_target_constant_folds() {
        let left = Guard::always(operation(1))
            .expect("guard should be valid");

        let right = Guard::never(operation(1))
            .expect("guard should be valid");

        let combined = left
            .or(right)
            .expect("guards have the same target");

        assert!(combined.is_always());
        assert_eq!(combined.target(), operation(1));
    }

    #[test]
    fn guard_set_preserves_order() {
        let first = Guard::always(operation(10))
            .expect("guard should be valid");

        let second = Guard::never(operation(20))
            .expect("guard should be valid");

        let mut set = GuardSet::new();
        set.push(first);
        set.push(second);

        assert_eq!(set.len(), 2);
        assert_eq!(
            set.get(0).map(Guard::target),
            Some(operation(10))
        );
        assert_eq!(
            set.get(1).map(Guard::target),
            Some(operation(20))
        );
    }

    #[test]
    fn guard_set_target_operations_are_deterministic() {
        let mut set = GuardSet::new();

        set.push(
            Guard::always(operation(30))
                .expect("guard should be valid"),
        );

        set.push(
            Guard::never(operation(10))
                .expect("guard should be valid"),
        );

        set.push(
            Guard::always(operation(30))
                .expect("guard should be valid"),
        );

        let targets = set.target_operations();

        let collected: Vec<_> = targets.into_iter().collect();

        assert_eq!(
            collected,
            vec![operation(10), operation(30)]
        );
    }

    #[test]
    fn unbounded_policy_has_no_artificial_ceiling() {
        let policy = GuardValidationPolicy::unbounded();

        assert_eq!(
            policy.max_condition_nodes,
            usize::MAX
        );

        assert_eq!(
            policy.max_guards,
            usize::MAX
        );
    }

    #[test]
    fn guard_validation_accepts_normal_guard() {
        let guard = Guard::always(operation(100))
            .expect("guard should be valid");

        let policy = GuardValidationPolicy::new(
            0,
            1,
        );

        assert!(guard.validate(&policy).is_ok());
    }

    #[test]
    fn guard_set_validation_accepts_within_policy() {
        let guard = Guard::always(operation(100))
            .expect("guard should be valid");

        let mut set = GuardSet::new();
        set.push(guard);

        let policy = GuardValidationPolicy::new(
            0,
            1,
        );

        assert!(set.validate(&policy).is_ok());
    }

    #[test]
    fn guard_set_validation_rejects_excess_count() {
        let mut set = GuardSet::new();

        set.push(
            Guard::always(operation(1))
                .expect("guard should be valid"),
        );

        set.push(
            Guard::always(operation(2))
                .expect("guard should be valid"),
        );

        let policy = GuardValidationPolicy::new(
            0,
            1,
        );

        assert!(matches!(
            set.validate(&policy),
            Err(GuardError::GuardLimitExceeded {
                requested: 2,
                maximum: 1
            })
        ));
    }

    #[test]
    fn guard_target_can_be_changed_without_mutating_original() {
        let original = Guard::always(operation(1))
            .expect("guard should be valid");

        let replacement = original.clone()
            .with_target(operation(2));

        assert_eq!(
            original.target(),
            operation(1)
        );

        assert_eq!(
            replacement.target(),
            operation(2)
        );
    }
}