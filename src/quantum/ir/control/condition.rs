//! Zamani Quantum IR — Canonical Execution Conditions
//!
//! Production-grade, hardware-independent representation of conditions used
//! to control quantum, classical, pulse, analog, and hybrid operations.
//!
//! # Architectural role
//!
//! `control::condition` owns the semantic boundary between:
//!
//! ```text
//! classical predicate
//!        │
//!        ▼
//! execution condition
//!        │
//!        ▼
//! conditional operation / region
//! ```
//!
//! The canonical Boolean predicate itself is owned by:
//!
//! ```text
//! quantum::ir::classical::predicate::ClassicalPredicate
//! ```
//!
//! This module deliberately does NOT redefine `ClassicalPredicate`.
//!
//! `Condition` answers:
//!
//! > Under what semantic condition may this construct execute?
//!
//! It does NOT answer:
//!
//! - which physical qubits execute the operation;
//! - where the classical predicate is evaluated;
//! - which hardware controller evaluates it;
//! - how measurement is physically performed;
//! - how routing is performed;
//! - how scheduling is performed;
//! - how pulses are generated;
//! - how a backend implements the condition;
//! - how a simulator evaluates quantum state.
//!
//! Those responsibilities belong to the appropriate downstream IR/compiler,
//! hardware, runtime, simulator, or backend subsystem.
//!
//! # Canonical dependency boundary
//!
//! ```text
//! quantum::ir::classical::predicate
//!                 │
//!                 ▼
//!       quantum::ir::control::condition
//!                 │
//!                 ▼
//!        control-flow / operation
//!                 │
//!                 ▼
//!        quantum::ir::qubit::QubitId
//! ```
//!
//! `QubitId` is intentionally not imported here. A condition does not mean
//! "the quantum state of q is true". Quantum control belongs to the operation
//! or gate semantics. A classical execution condition normally originates from
//! measurement/classical state.
//!
//! This is important because the repository defines
//! `quantum::ir::qubit::QubitId` as the authoritative logical-qubit identity.
//!
//! # Universal-program principle
//!
//! There is no architectural maximum number of:
//!
//! - conditions;
//! - predicate terms;
//! - classical bits;
//! - IR values;
//! - nested conditions;
//! - operations controlled by conditions.
//!
//! The representation scales with the program and available resources.
//!
//! Any finite validation limit in this file is an explicit safety/resource
//! policy, not a semantic limit of the Zamani language.
//!
//! # Design principles
//!
//! This file guarantees:
//!
//! 1. no duplicated classical-predicate type;
//! 2. no fixed quantum-machine size;
//! 3. no fixed predicate width;
//! 4. no vendor-specific semantics;
//! 5. deterministic dependency extraction;
//! 6. checked arithmetic;
//! 7. bounded structural validation;
//! 8. explicit empty-condition semantics;
//! 9. explicit always/never semantics;
//! 10. no silent invalid states through constructors;
//! 11. no unsafe code;
//! 12. Rust 1.97 / 1.97.1 compatibility.
//!
//! # Integration contract
//!
//! `classical/predicate.rs`
//!     Owns the actual Boolean predicate semantics.
//!
//! `control_flow.rs`
//!     Uses `Condition` for conditional execution and should eventually
//!     remove its legacy duplicate `ClassicalPredicate` condition model.
//!
//! `operation.rs`
//!     May attach a `Condition` to a conditional operation.
//!
//! `program/operation.rs`
//!     May consume `Condition` when structured operation IR is migrated into
//!     the new program hierarchy.
//!
//! `measurement.rs`
//!     Produces classical information which can subsequently be consumed by
//!     the canonical predicate.
//!
//! `qubit.rs`
//!     Remains the canonical owner of `QubitId`; this file does not duplicate
//!     it.
//!
//! `validation.rs`
//!     Can validate condition structure and dependency references against the
//!     enclosing program's declarations.
//!
//! `analysis.rs`
//!     Can use `dependencies()` to determine classical/value dependencies.
//!
//! `serialization.rs`
//!     Can serialize the deterministic semantic representation.
//!
//! `hash.rs`
//!     Can hash the condition structurally as part of canonical IR hashing.
//!
//! `dialect/*`
//!     May extend condition semantics without changing this core contract.
//!
//! # Important migration rule
//!
//! The legacy `quantum::ir::control_flow::ClassicalPredicate` must NOT become
//! a second canonical condition type.
//!
//! The long-term direction is:
//!
//! ```text
//! classical::predicate::ClassicalPredicate
//!                     │
//!                     ▼
//! control::condition::Condition
//!                     │
//!                     ▼
//! control flow / operations
//! ```
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - no external dependencies.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]

// =============================================================================
// Imports
// =============================================================================

use std::collections::BTreeSet;
use std::fmt;

use super::super::classical::predicate::{
    ClassicalPredicate,
    PredicateOperand,
};

// =============================================================================
// Result
// =============================================================================

/// Result type used by condition construction and validation.
pub type ConditionResult<T> = Result<T, ConditionError>;

// =============================================================================
// Condition
// =============================================================================

/// Canonical execution condition.
///
/// A `Condition` is deliberately a thin semantic wrapper around the canonical
/// classical predicate representation.
///
/// The wrapper exists because a control-flow operation needs a semantic
/// *execution condition*, while `ClassicalPredicate` describes the Boolean
/// expression itself.
///
/// This separation prevents control-flow semantics from becoming coupled to
/// the internal predicate AST.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Condition {
    /// The controlled construct always executes.
    Always,

    /// The controlled construct never executes.
    Never,

    /// The controlled construct executes when the supplied canonical
    /// classical predicate evaluates to true.
    Predicate(ClassicalPredicate),
}

impl Condition {
    // =========================================================================
    // Constructors
    // =========================================================================

    /// Creates an unconditional condition.
    ///
    /// This is equivalent to `Condition::Always`.
    #[must_use]
    pub const fn always() -> Self {
        Self::Always
    }

    /// Creates a condition that can never execute.
    ///
    /// This is useful for explicit dead branches after transformation and for
    /// representing a semantically valid false guard without inventing a
    /// special optimizer-only representation.
    #[must_use]
    pub const fn never() -> Self {
        Self::Never
    }

    /// Wraps a canonical classical predicate.
    ///
    /// Constant predicates are normalized to `Always` or `Never` where
    /// possible. This keeps equivalent semantic conditions canonical.
    #[must_use]
    pub fn predicate(predicate: ClassicalPredicate) -> Self {
        match predicate.constant_value() {
            Some(true) => Self::Always,
            Some(false) => Self::Never,
            None => Self::Predicate(predicate),
        }
    }

    /// Creates a condition directly from a classical predicate.
    ///
    /// This is an alias for [`Self::predicate`].
    #[must_use]
    pub fn from_predicate(predicate: ClassicalPredicate) -> Self {
        Self::predicate(predicate)
    }

    // =========================================================================
    // Semantic classification
    // =========================================================================

    /// Returns whether this condition is unconditional.
    #[must_use]
    pub const fn is_always(&self) -> bool {
        matches!(self, Self::Always)
    }

    /// Returns whether this condition can never execute.
    #[must_use]
    pub const fn is_never(&self) -> bool {
        matches!(self, Self::Never)
    }

    /// Returns whether this condition contains a non-constant predicate.
    #[must_use]
    pub const fn is_predicate(&self) -> bool {
        matches!(self, Self::Predicate(_))
    }

    /// Returns whether this condition can execute for at least one possible
    /// classical state.
    ///
    /// `Predicate` returns `true` because this module intentionally does not
    /// perform arbitrary theorem proving.
    #[must_use]
    pub const fn may_execute(&self) -> bool {
        !matches!(self, Self::Never)
    }

    /// Returns whether this condition is guaranteed to execute.
    #[must_use]
    pub const fn must_execute(&self) -> bool {
        matches!(self, Self::Always)
    }

    /// Returns the underlying predicate when this is a predicate condition.
    #[must_use]
    pub const fn as_predicate(&self) -> Option<&ClassicalPredicate> {
        match self {
            Self::Predicate(predicate) => Some(predicate),
            Self::Always | Self::Never => None,
        }
    }

    /// Consumes the condition and returns its underlying predicate.
    ///
    /// `Always` becomes the canonical `true` predicate and `Never` becomes
    /// the canonical `false` predicate.
    #[must_use]
    pub fn into_predicate(self) -> ClassicalPredicate {
        match self {
            Self::Always => ClassicalPredicate::always(),
            Self::Never => ClassicalPredicate::never(),
            Self::Predicate(predicate) => predicate,
        }
    }

    // =========================================================================
    // Logical normalization
    // =========================================================================

    /// Returns the logical negation of this condition.
    ///
    /// The operation performs constant folding:
    ///
    /// ```text
    /// !Always -> Never
    /// !Never  -> Always
    /// !P      -> Predicate(!P)
    /// ```
    #[must_use]
    pub fn not(self) -> Self {
        match self {
            Self::Always => Self::Never,
            Self::Never => Self::Always,
            Self::Predicate(predicate) => {
                Self::predicate(ClassicalPredicate::not(predicate))
            }
        }
    }

    /// Returns the conjunction of two execution conditions.
    ///
    /// Constant folding is performed without attempting arbitrary Boolean
    /// theorem proving.
    pub fn and(
        self,
        other: Self,
    ) -> ConditionResult<Self> {
        match (self, other) {
            (Self::Never, _) | (_, Self::Never) => Ok(Self::Never),

            (Self::Always, condition)
            | (condition, Self::Always) => Ok(condition),

            (Self::Predicate(left), Self::Predicate(right)) => {
                let predicate = ClassicalPredicate::and(vec![left, right])
                    .map_err(ConditionError::Predicate)?;

                Ok(Self::predicate(predicate))
            }
        }
    }

    /// Returns the disjunction of two execution conditions.
    ///
    /// Constant folding is performed without attempting arbitrary Boolean
    /// theorem proving.
    pub fn or(
        self,
        other: Self,
    ) -> ConditionResult<Self> {
        match (self, other) {
            (Self::Always, _) | (_, Self::Always) => Ok(Self::Always),

            (Self::Never, condition)
            | (condition, Self::Never) => Ok(condition),

            (Self::Predicate(left), Self::Predicate(right)) => {
                let predicate = ClassicalPredicate::or(vec![left, right])
                    .map_err(ConditionError::Predicate)?;

                Ok(Self::predicate(predicate))
            }
        }
    }

    /// Returns the exclusive disjunction of two execution conditions.
    ///
    /// This operation is intentionally delegated to the canonical predicate
    /// layer rather than implementing a second XOR AST here.
    pub fn xor(
        self,
        other: Self,
    ) -> ConditionResult<Self> {
        match (self, other) {
            (Self::Always, Self::Always)
            | (Self::Never, Self::Never) => Ok(Self::Never),

            (Self::Always, Self::Never)
            | (Self::Never, Self::Always) => Ok(Self::Always),

            (Self::Predicate(left), Self::Predicate(right)) => {
                let predicate = ClassicalPredicate::xor(vec![left, right])
                    .map_err(ConditionError::Predicate)?;

                Ok(Self::predicate(predicate))
            }
        }
    }

    /// Returns logical implication.
    ///
    /// The predicate layer remains the canonical owner of implication
    /// semantics.
    #[must_use]
    pub fn implies(
        self,
        other: Self,
    ) -> Self {
        match (self, other) {
            (Self::Never, _) => Self::Always,
            (Self::Always, condition) => condition,
            (condition, Self::Always) => Self::Always,
            (condition, Self::Never) => condition.not(),

            (Self::Predicate(left), Self::Predicate(right)) => {
                Self::predicate(ClassicalPredicate::implies(left, right))
            }
        }
    }

    /// Returns logical equivalence.
    #[must_use]
    pub fn equivalent(
        self,
        other: Self,
    ) -> Self {
        match (self, other) {
            (Self::Always, Self::Always)
            | (Self::Never, Self::Never) => Self::Always,

            (Self::Always, Self::Never)
            | (Self::Never, Self::Always) => Self::Never,

            (Self::Always, condition)
            | (condition, Self::Always) => condition,

            (Self::Never, condition)
            | (condition, Self::Never) => condition.not(),

            (Self::Predicate(left), Self::Predicate(right)) => {
                Self::predicate(
                    ClassicalPredicate::equivalent(left, right),
                )
            }
        }
    }

    // =========================================================================
    // Structural analysis
    // =========================================================================

    /// Returns the root semantic kind.
    #[must_use]
    pub const fn kind(&self) -> ConditionKind {
        match self {
            Self::Always => ConditionKind::Always,
            Self::Never => ConditionKind::Never,
            Self::Predicate(_) => ConditionKind::Predicate,
        }
    }

    /// Returns the predicate depth.
    ///
    /// `Always` and `Never` have depth zero.
    #[must_use]
    pub fn depth(&self) -> usize {
        match self {
            Self::Always | Self::Never => 0,
            Self::Predicate(predicate) => predicate.depth(),
        }
    }

    /// Returns the total number of predicate nodes.
    ///
    /// `Always` and `Never` contain zero predicate nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        match self {
            Self::Always | Self::Never => 0,
            Self::Predicate(predicate) => predicate.node_count(),
        }
    }

    /// Returns the number of logical predicate terms.
    ///
    /// This is intentionally derived from the canonical predicate structure
    /// rather than stored as a second counter that could become inconsistent.
    #[must_use]
    pub fn term_count(&self) -> usize {
        match self {
            Self::Always | Self::Never => 0,
            Self::Predicate(predicate) => predicate.term_count(),
        }
    }

    /// Returns all classical-bit dependencies in deterministic order.
    ///
    /// The result is a `BTreeSet`, not a `HashSet`, so iteration order is
    /// stable across runs.
    #[must_use]
    pub fn classical_dependencies(&self) -> BTreeSet<super::super::classical::bit::ClassicalBitId> {
        let mut dependencies = BTreeSet::new();

        if let Self::Predicate(predicate) = self {
            collect_classical_dependencies(predicate, &mut dependencies);
        }

        dependencies
    }

    /// Returns all referenced IR value IDs in deterministic order.
    ///
    /// This allows analysis and validation layers to determine which SSA/IR
    /// values must dominate the conditional use without coupling this file to
    /// the operation/value implementation.
    #[must_use]
    pub fn value_dependencies(
        &self,
    ) -> BTreeSet<super::super::identity::ValueId> {
        let mut dependencies = BTreeSet::new();

        if let Self::Predicate(predicate) = self {
            collect_value_dependencies(predicate, &mut dependencies);
        }

        dependencies
    }

    /// Returns the number of classical-bit dependencies.
    #[must_use]
    pub fn classical_dependency_count(&self) -> usize {
        self.classical_dependencies().len()
    }

    /// Returns the number of IR-value dependencies.
    #[must_use]
    pub fn value_dependency_count(&self) -> usize {
        self.value_dependencies().len()
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates the condition using the supplied explicit policy.
    ///
    /// This validation is structural. It does not inspect hardware.
    pub fn validate(
        &self,
        limits: &ConditionValidationLimits,
    ) -> ConditionResult<()> {
        match self {
            Self::Always | Self::Never => Ok(()),

            Self::Predicate(predicate) => {
                validate_predicate(predicate, limits)
            }
        }
    }

    /// Validates the condition using production-oriented defaults.
    pub fn validate_production(&self) -> ConditionResult<()> {
        self.validate(&ConditionValidationLimits::production())
    }

    // =========================================================================
    // Canonical text representation
    // =========================================================================

    /// Returns a deterministic semantic representation suitable for
    /// diagnostics, debugging and canonical textual serialization.
    ///
    /// This representation is deliberately independent of Rust debug output.
    #[must_use]
    pub fn canonical_string(&self) -> String {
        match self {
            Self::Always => "always".to_owned(),
            Self::Never => "never".to_owned(),
            Self::Predicate(predicate) => {
                canonical_predicate_string(predicate)
            }
        }
    }
}

impl Default for Condition {
    fn default() -> Self {
        Self::Always
    }
}

impl From<ClassicalPredicate> for Condition {
    fn from(predicate: ClassicalPredicate) -> Self {
        Self::predicate(predicate)
    }
}

impl fmt::Display for Condition {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.canonical_string())
    }
}

// =============================================================================
// Condition kind
// =============================================================================

/// Root semantic kind of an execution condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConditionKind {
    /// Unconditional execution.
    Always,

    /// Never executes.
    Never,

    /// Execution depends on a classical predicate.
    Predicate,
}

impl fmt::Display for ConditionKind {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let name = match self {
            Self::Always => "always",
            Self::Never => "never",
            Self::Predicate => "predicate",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Validation limits
// =============================================================================

/// Explicit structural-safety policy for conditions.
///
/// These values are *resource/security limits for one IR operation*, not
/// architectural limits of Zamani.
///
/// A caller compiling a larger program may provide a larger policy when the
/// available resources justify it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConditionValidationLimits {
    /// Maximum permitted predicate depth.
    pub max_depth: usize,

    /// Maximum permitted number of predicate nodes.
    pub max_nodes: usize,

    /// Maximum permitted number of logical terms across n-ary operators.
    pub max_terms: usize,

    /// Maximum permitted number of distinct classical-bit dependencies.
    pub max_classical_dependencies: usize,

    /// Maximum permitted number of distinct IR-value dependencies.
    pub max_value_dependencies: usize,
}

impl ConditionValidationLimits {
    /// Creates an explicit validation policy.
    #[must_use]
    pub const fn new(
        max_depth: usize,
        max_nodes: usize,
        max_terms: usize,
        max_classical_dependencies: usize,
        max_value_dependencies: usize,
    ) -> Self {
        Self {
            max_depth,
            max_nodes,
            max_terms,
            max_classical_dependencies,
            max_value_dependencies,
        }
    }

    /// Production-oriented default policy.
    ///
    /// These values are deliberately validation-policy defaults rather than
    /// language or hardware limits.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_depth: 1_024,
            max_nodes: 1_000_000,
            max_terms: 1_000_000,
            max_classical_dependencies: 1_000_000,
            max_value_dependencies: 1_000_000,
        }
    }

    /// Completely unbounded structural policy within the host's representable
    /// address space.
    ///
    /// This does not make memory allocation infinite. It simply removes the
    /// explicit condition-specific policy ceilings.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_depth: usize::MAX,
            max_nodes: usize::MAX,
            max_terms: usize::MAX,
            max_classical_dependencies: usize::MAX,
            max_value_dependencies: usize::MAX,
        }
    }

    /// Returns whether a depth is permitted.
    #[must_use]
    pub const fn allows_depth(&self, depth: usize) -> bool {
        depth <= self.max_depth
    }

    /// Returns whether a node count is permitted.
    #[must_use]
    pub const fn allows_nodes(&self, nodes: usize) -> bool {
        nodes <= self.max_nodes
    }

    /// Returns whether a term count is permitted.
    #[must_use]
    pub const fn allows_terms(&self, terms: usize) -> bool {
        terms <= self.max_terms
    }

    /// Returns whether a classical dependency count is permitted.
    #[must_use]
    pub const fn allows_classical_dependencies(
        &self,
        count: usize,
    ) -> bool {
        count <= self.max_classical_dependencies
    }

    /// Returns whether a value dependency count is permitted.
    #[must_use]
    pub const fn allows_value_dependencies(
        &self,
        count: usize,
    ) -> bool {
        count <= self.max_value_dependencies
    }
}

impl Default for ConditionValidationLimits {
    fn default() -> Self {
        Self::production()
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or validating conditions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionError {
    /// The wrapped canonical predicate rejected the requested construction.
    Predicate(String),

    /// Predicate depth exceeds the explicit policy.
    DepthLimitExceeded {
        /// Requested depth.
        requested: usize,

        /// Permitted depth.
        maximum: usize,
    },

    /// Predicate node count exceeds the explicit policy.
    NodeLimitExceeded {
        /// Requested node count.
        requested: usize,

        /// Permitted node count.
        maximum: usize,
    },

    /// Predicate term count exceeds the explicit policy.
    TermLimitExceeded {
        /// Requested term count.
        requested: usize,

        /// Permitted term count.
        maximum: usize,
    },

    /// Classical dependency count exceeds the explicit policy.
    ClassicalDependencyLimitExceeded {
        /// Requested dependency count.
        requested: usize,

        /// Permitted dependency count.
        maximum: usize,
    },

    /// IR value dependency count exceeds the explicit policy.
    ValueDependencyLimitExceeded {
        /// Requested dependency count.
        requested: usize,

        /// Permitted dependency count.
        maximum: usize,
    },

    /// Structural arithmetic overflow occurred while calculating a validation
    /// quantity.
    ArithmeticOverflow {
        /// Semantic calculation that overflowed.
        calculation: &'static str,
    },

    /// A predicate was structurally malformed.
    InvalidPredicate {
        /// Static semantic reason.
        reason: &'static str,
    },
}

impl fmt::Display for ConditionError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Predicate(error) => {
                write!(formatter, "predicate construction error: {error}")
            }

            Self::DepthLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "condition depth limit exceeded: requested \
                     {requested}, maximum {maximum}"
                )
            }

            Self::NodeLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "condition node limit exceeded: requested \
                     {requested}, maximum {maximum}"
                )
            }

            Self::TermLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "condition term limit exceeded: requested \
                     {requested}, maximum {maximum}"
                )
            }

            Self::ClassicalDependencyLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "condition classical-dependency limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ValueDependencyLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "condition value-dependency limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "condition arithmetic overflow while calculating \
                     {calculation}"
                )
            }

            Self::InvalidPredicate { reason } => {
                write!(
                    formatter,
                    "invalid condition predicate: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for ConditionError {}

// =============================================================================
// Predicate validation
// =============================================================================

/// Performs bounded structural validation without relying on recursive
/// validation supplied by another module.
///
/// This function deliberately uses an explicit stack instead of recursive
/// descent. Very deeply nested predicates therefore cannot consume the Rust
/// call stack merely because the input IR is deeply nested.
fn validate_predicate(
    root: &ClassicalPredicate,
    limits: &ConditionValidationLimits,
) -> ConditionResult<()> {
    let mut stack = Vec::new();
    stack.push((root, 1usize));

    let mut nodes = 0usize;
    let mut terms = 0usize;
    let mut classical_dependencies = BTreeSet::new();
    let mut value_dependencies = BTreeSet::new();

    while let Some((predicate, depth)) = stack.pop() {
        nodes = nodes.checked_add(1).ok_or(
            ConditionError::ArithmeticOverflow {
                calculation: "predicate node count",
            },
        )?;

        if !limits.allows_nodes(nodes) {
            return Err(ConditionError::NodeLimitExceeded {
                requested: nodes,
                maximum: limits.max_nodes,
            });
        }

        if !limits.allows_depth(depth) {
            return Err(ConditionError::DepthLimitExceeded {
                requested: depth,
                maximum: limits.max_depth,
            });
        }

        match predicate {
            ClassicalPredicate::Constant(_) => {}

            ClassicalPredicate::Bit(bit) => {
                classical_dependencies.insert(*bit);
            }

            ClassicalPredicate::Compare {
                left,
                operator: _,
                right,
            } => {
                collect_operand_dependencies(
                    left,
                    &mut classical_dependencies,
                    &mut value_dependencies,
                );

                collect_operand_dependencies(
                    right,
                    &mut classical_dependencies,
                    &mut value_dependencies,
                );
            }

            ClassicalPredicate::Not(child) => {
                let child_depth = depth.checked_add(1).ok_or(
                    ConditionError::ArithmeticOverflow {
                        calculation: "predicate depth",
                    },
                )?;

                stack.push((child, child_depth));
            }

            ClassicalPredicate::And(children)
            | ClassicalPredicate::Or(children)
            | ClassicalPredicate::Xor(children) => {
                if children.is_empty() {
                    return Err(ConditionError::InvalidPredicate {
                        reason:
                            "logical predicate contains no child terms",
                    });
                }

                terms = terms.checked_add(children.len()).ok_or(
                    ConditionError::ArithmeticOverflow {
                        calculation: "predicate term count",
                    },
                )?;

                if !limits.allows_terms(terms) {
                    return Err(ConditionError::TermLimitExceeded {
                        requested: terms,
                        maximum: limits.max_terms,
                    });
                }

                let child_depth = depth.checked_add(1).ok_or(
                    ConditionError::ArithmeticOverflow {
                        calculation: "predicate depth",
                    },
                )?;

                for child in children.iter().rev() {
                    stack.push((child, child_depth));
                }
            }

            ClassicalPredicate::Implies {
                antecedent,
                consequent,
            } => {
                let child_depth = depth.checked_add(1).ok_or(
                    ConditionError::ArithmeticOverflow {
                        calculation: "predicate depth",
                    },
                )?;

                stack.push((consequent, child_depth));
                stack.push((antecedent, child_depth));
            }

            ClassicalPredicate::Equivalent { left, right } => {
                let child_depth = depth.checked_add(1).ok_or(
                    ConditionError::ArithmeticOverflow {
                        calculation: "predicate depth",
                    },
                )?;

                stack.push((right, child_depth));
                stack.push((left, child_depth));
            }

            ClassicalPredicate::InSet { value, candidates } => {
                if candidates.is_empty() {
                    return Err(ConditionError::InvalidPredicate {
                        reason:
                            "membership predicate contains no candidates",
                    });
                }

                collect_operand_dependencies(
                    value,
                    &mut classical_dependencies,
                    &mut value_dependencies,
                );

                terms = terms.checked_add(candidates.len()).ok_or(
                    ConditionError::ArithmeticOverflow {
                        calculation: "membership candidate count",
                    },
                )?;

                if !limits.allows_terms(terms) {
                    return Err(ConditionError::TermLimitExceeded {
                        requested: terms,
                        maximum: limits.max_terms,
                    });
                }

                for candidate in candidates {
                    collect_operand_dependencies(
                        candidate,
                        &mut classical_dependencies,
                        &mut value_dependencies,
                    );
                }
            }
        }

        if !limits.allows_classical_dependencies(
            classical_dependencies.len(),
        ) {
            return Err(
                ConditionError::ClassicalDependencyLimitExceeded {
                    requested: classical_dependencies.len(),
                    maximum: limits.max_classical_dependencies,
                },
            );
        }

        if !limits.allows_value_dependencies(value_dependencies.len()) {
            return Err(ConditionError::ValueDependencyLimitExceeded {
                requested: value_dependencies.len(),
                maximum: limits.max_value_dependencies,
            });
        }
    }

    Ok(())
}

// =============================================================================
// Dependency extraction
// =============================================================================

/// Collects classical-bit and IR-value dependencies from a predicate
/// iteratively.
///
/// This function is intentionally recursive only over the predicate shape
/// through a local explicit stack, preventing stack growth proportional to
/// maliciously deep input.
fn collect_classical_dependencies(
    root: &ClassicalPredicate,
    output: &mut BTreeSet<
        super::super::classical::bit::ClassicalBitId,
    >,
) {
    let mut stack = Vec::new();
    stack.push(root);

    while let Some(predicate) = stack.pop() {
        match predicate {
            ClassicalPredicate::Constant(_) => {}

            ClassicalPredicate::Bit(bit) => {
                output.insert(*bit);
            }

            ClassicalPredicate::Compare { left, right, .. } => {
                collect_operand_classical_dependencies(left, output);
                collect_operand_classical_dependencies(right, output);
            }

            ClassicalPredicate::Not(child) => {
                stack.push(child);
            }

            ClassicalPredicate::And(children)
            | ClassicalPredicate::Or(children)
            | ClassicalPredicate::Xor(children) => {
                for child in children {
                    stack.push(child);
                }
            }

            ClassicalPredicate::Implies {
                antecedent,
                consequent,
            } => {
                stack.push(antecedent);
                stack.push(consequent);
            }

            ClassicalPredicate::Equivalent { left, right } => {
                stack.push(left);
                stack.push(right);
            }

            ClassicalPredicate::InSet { value, candidates } => {
                collect_operand_classical_dependencies(value, output);

                for candidate in candidates {
                    collect_operand_classical_dependencies(
                        candidate,
                        output,
                    );
                }
            }
        }
    }
}

/// Collects IR value dependencies from a predicate.
fn collect_value_dependencies(
    root: &ClassicalPredicate,
    output: &mut BTreeSet<super::super::identity::ValueId>,
) {
    let mut stack = Vec::new();
    stack.push(root);

    while let Some(predicate) = stack.pop() {
        match predicate {
            ClassicalPredicate::Constant(_) => {}

            ClassicalPredicate::Bit(_) => {}

            ClassicalPredicate::Compare { left, right, .. } => {
                collect_operand_value_dependencies(left, output);
                collect_operand_value_dependencies(right, output);
            }

            ClassicalPredicate::Not(child) => {
                stack.push(child);
            }

            ClassicalPredicate::And(children)
            | ClassicalPredicate::Or(children)
            | ClassicalPredicate::Xor(children) => {
                for child in children {
                    stack.push(child);
                }
            }

            ClassicalPredicate::Implies {
                antecedent,
                consequent,
            } => {
                stack.push(antecedent);
                stack.push(consequent);
            }

            ClassicalPredicate::Equivalent { left, right } => {
                stack.push(left);
                stack.push(right);
            }

            ClassicalPredicate::InSet { value, candidates } => {
                collect_operand_value_dependencies(value, output);

                for candidate in candidates {
                    collect_operand_value_dependencies(
                        candidate,
                        output,
                    );
                }
            }
        }
    }
}

/// Collects classical dependencies from one predicate operand.
fn collect_operand_classical_dependencies(
    operand: &PredicateOperand,
    output: &mut BTreeSet<
        super::super::classical::bit::ClassicalBitId,
    >,
) {
    if let Some(bit) = operand.classical_bit_id() {
        output.insert(bit);
    }
}

/// Collects IR-value dependencies from one predicate operand.
fn collect_operand_value_dependencies(
    operand: &PredicateOperand,
    output: &mut BTreeSet<super::super::identity::ValueId>,
) {
    if let Some(value) = operand.value_id() {
        output.insert(value);
    }
}

/// Collects both classes of dependencies.
fn collect_operand_dependencies(
    operand: &PredicateOperand,
    classical_dependencies: &mut BTreeSet<
        super::super::classical::bit::ClassicalBitId,
    >,
    value_dependencies: &mut BTreeSet<
        super::super::identity::ValueId,
    >,
) {
    collect_operand_classical_dependencies(
        operand,
        classical_dependencies,
    );

    collect_operand_value_dependencies(
        operand,
        value_dependencies,
    );
}

// =============================================================================
// Canonical formatting
// =============================================================================

/// Produces a deterministic textual representation.
///
/// This deliberately does not use `Debug`, because debug formatting is an
/// implementation detail and must not become a serialization/hash contract.
fn canonical_predicate_string(
    predicate: &ClassicalPredicate,
) -> String {
    match predicate {
        ClassicalPredicate::Constant(value) => {
            if value.as_bool() {
                "true".to_owned()
            } else {
                "false".to_owned()
            }
        }

        ClassicalPredicate::Bit(bit) => {
            bit.to_string()
        }

        ClassicalPredicate::Compare {
            left,
            operator,
            right,
        } => {
            format!(
                "{} {} {}",
                canonical_operand_string(left),
                operator,
                canonical_operand_string(right)
            )
        }

        ClassicalPredicate::Not(child) => {
            format!(
                "not({})",
                canonical_predicate_string(child)
            )
        }

        ClassicalPredicate::And(children) => {
            canonical_nary_predicate("and", children)
        }

        ClassicalPredicate::Or(children) => {
            canonical_nary_predicate("or", children)
        }

        ClassicalPredicate::Xor(children) => {
            canonical_nary_predicate("xor", children)
        }

        ClassicalPredicate::Implies {
            antecedent,
            consequent,
        } => {
            format!(
                "implies({}, {})",
                canonical_predicate_string(antecedent),
                canonical_predicate_string(consequent)
            )
        }

        ClassicalPredicate::Equivalent { left, right } => {
            format!(
                "equivalent({}, {})",
                canonical_predicate_string(left),
                canonical_predicate_string(right)
            )
        }

        ClassicalPredicate::InSet { value, candidates } => {
            let mut result = String::new();

            result.push_str("in(");
            result.push_str(&canonical_operand_string(value));
            result.push_str(",[");

            for (index, candidate) in candidates.iter().enumerate() {
                if index != 0 {
                    result.push(',');
                }

                result.push_str(&canonical_operand_string(candidate));
            }

            result.push_str("])");

            result
        }
    }
}

/// Produces canonical text for an n-ary predicate.
fn canonical_nary_predicate(
    operator: &str,
    children: &[ClassicalPredicate],
) -> String {
    let mut result = String::new();

    result.push_str(operator);
    result.push('(');

    for (index, child) in children.iter().enumerate() {
        if index != 0 {
            result.push(',');
        }

        result.push_str(&canonical_predicate_string(child));
    }

    result.push(')');

    result
}

/// Produces deterministic text for a predicate operand.
fn canonical_operand_string(
    operand: &PredicateOperand,
) -> String {
    match operand {
        PredicateOperand::Bool(value) => {
            if value.as_bool() {
                "true".to_owned()
            } else {
                "false".to_owned()
            }
        }

        PredicateOperand::SignedInteger(value) => {
            value.to_string()
        }

        PredicateOperand::UnsignedInteger(value) => {
            value.to_string()
        }

        PredicateOperand::Float(value) => {
            format!("float:{}", value.bits())
        }

        PredicateOperand::ClassicalBit(bit) => {
            bit.to_string()
        }

        PredicateOperand::BitVector(bits) => {
            let mut result = String::new();

            result.push_str("bits:");

            for bit in bits.iter() {
                result.push(if *bit { '1' } else { '0' });
            }

            result
        }

        PredicateOperand::Value(value) => {
            format!("value:{value}")
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::classical::bit::ClassicalBitId;
    use super::super::super::classical::predicate::{
        ComparisonOperator,
        PredicateOperand,
    };

    #[test]
    fn always_is_unconditional() {
        let condition = Condition::always();

        assert!(condition.is_always());
        assert!(!condition.is_never());
        assert!(condition.may_execute());
        assert!(condition.must_execute());
        assert_eq!(condition.depth(), 0);
        assert_eq!(condition.node_count(), 0);
        assert_eq!(condition.canonical_string(), "always");
    }

    #[test]
    fn never_is_unreachable() {
        let condition = Condition::never();

        assert!(!condition.is_always());
        assert!(condition.is_never());
        assert!(!condition.may_execute());
        assert!(!condition.must_execute());
        assert_eq!(condition.depth(), 0);
        assert_eq!(condition.node_count(), 0);
        assert_eq!(condition.canonical_string(), "never");
    }

    #[test]
    fn constant_predicates_are_normalized() {
        let true_condition =
            Condition::predicate(ClassicalPredicate::always());

        let false_condition =
            Condition::predicate(ClassicalPredicate::never());

        assert!(true_condition.is_always());
        assert!(false_condition.is_never());
    }

    #[test]
    fn bit_predicate_is_preserved() {
        let predicate =
            ClassicalPredicate::bit(ClassicalBitId::new(7));

        let condition = Condition::predicate(predicate);

        assert!(condition.is_predicate());

        let dependencies = condition.classical_dependencies();

        assert_eq!(
            dependencies.len(),
            1
        );

        assert!(
            dependencies.contains(&ClassicalBitId::new(7))
        );
    }

    #[test]
    fn and_performs_constant_folding() {
        let condition =
            Condition::always()
                .and(Condition::never())
                .expect("AND construction must succeed");

        assert!(condition.is_never());
    }

    #[test]
    fn or_performs_constant_folding() {
        let condition =
            Condition::always()
                .or(Condition::never())
                .expect("OR construction must succeed");

        assert!(condition.is_always());
    }

    #[test]
    fn not_performs_constant_folding() {
        assert!(Condition::always().not().is_never());
        assert!(Condition::never().not().is_always());
    }

    #[test]
    fn xor_constant_folding_is_correct() {
        assert!(
            Condition::always()
                .xor(Condition::never())
                .expect("XOR construction must succeed")
                .is_always()
        );

        assert!(
            Condition::always()
                .xor(Condition::always())
                .expect("XOR construction must succeed")
                .is_never()
        );
    }

    #[test]
    fn dependencies_are_deterministic() {
        let predicate =
            ClassicalPredicate::compare(
                PredicateOperand::classical_bit(
                    ClassicalBitId::new(9),
                ),
                ComparisonOperator::Equal,
                PredicateOperand::classical_bit(
                    ClassicalBitId::new(2),
                ),
            )
            .expect("comparison must be valid");

        let condition = Condition::predicate(predicate);

        let dependencies =
            condition.classical_dependencies();

        let collected: Vec<_> =
            dependencies.iter().copied().collect();

        assert_eq!(
            collected,
            vec![
                ClassicalBitId::new(2),
                ClassicalBitId::new(9),
            ]
        );
    }

    #[test]
    fn validation_accepts_valid_condition() {
        let predicate =
            ClassicalPredicate::bit(ClassicalBitId::new(0));

        let condition = Condition::predicate(predicate);

        assert!(
            condition
                .validate_production()
                .is_ok()
        );
    }

    #[test]
    fn validation_rejects_excessive_depth() {
        let mut predicate =
            ClassicalPredicate::bit(ClassicalBitId::new(0));

        for _ in 0..8 {
            predicate = ClassicalPredicate::not(predicate);
        }

        let condition = Condition::predicate(predicate);

        let limits =
            ConditionValidationLimits::new(
                4,
                100,
                100,
                100,
                100,
            );

        let result = condition.validate(&limits);

        assert!(matches!(
            result,
            Err(ConditionError::DepthLimitExceeded { .. })
        ));
    }

    #[test]
    fn value_dependencies_are_collected() {
        let left =
            PredicateOperand::value(
                super::super::super::identity::ValueId::new(1),
            );

        let right =
            PredicateOperand::value(
                super::super::super::identity::ValueId::new(2),
            );

        let predicate =
            ClassicalPredicate::equal(left, right)
                .expect("comparison must be valid");

        let condition =
            Condition::predicate(predicate);

        let dependencies =
            condition.value_dependencies();

        assert_eq!(dependencies.len(), 2);
    }

    #[test]
    fn canonical_format_is_stable() {
        let predicate =
            ClassicalPredicate::equal(
                PredicateOperand::classical_bit(
                    ClassicalBitId::new(0),
                ),
                PredicateOperand::bool(true),
            )
            .expect("comparison must be valid");

        let condition =
            Condition::predicate(predicate);

        assert_eq!(
            condition.canonical_string(),
            "c0 == true"
        );
    }

    #[test]
    fn default_is_always() {
        assert!(Condition::default().is_always());
    }
}