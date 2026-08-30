"src/quantum/optimization/structure/conditional.rs"

//! Zamani Quantum Optimization — Conditional Control-Flow Structure
//!
//! Production-grade representation and analysis contracts for conditional
//! quantum regions.
//
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! quantum::frontend
//!      │
//!      ▼
//! quantum::ir::QuantumCircuit
//!      │
//!      ▼
//! quantum::optimization::circuit
//!      │
//!      ▼
//! quantum::optimization::structure::conditional
//!      │
//!      ├── predicate representation
//!      ├── branch structure
//!      ├── classical dependency metadata
//!      ├── semantic boundary metadata
//!      ├── branch equivalence contracts
//!      └── safe optimization eligibility
//!      │
//!      ▼
//! optimization passes
//!      │
//!      ▼
//! routing
//!      │
//!      ▼
//! scheduling
//!      │
//!      ▼
//! hardware
//! ```
//!
//! # Important architectural rule
//!
//! This module does NOT define another Quantum IR.
//!
//! The authoritative quantum representation remains:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! and the optimizer's canonical access layer remains:
//!
//! `crate::quantum::optimization::circuit::CircuitView`
//!
//! This file defines only optimizer-owned structural metadata for conditional
//! control flow.
//!
//! The current canonical Quantum IR does not yet expose a first-class
//! conditional operation. Consequently, this module deliberately does not
//! pretend that conditional execution is already encoded in `Gate`.
//!
//! Instead, it establishes the complete optimizer-side contract that a future
//! canonical conditional IR operation can map into.
//!
//! # Design goals
//!
//! This module provides:
//!
//! - deterministic conditional identity;
//! - classical predicate representation;
//! - composable Boolean predicates;
//! - equality and inequality predicates;
//! - single-bit predicates;
//! - register/value predicates;
//! - conjunction/disjunction/xor/negation;
//! - explicit branch structure;
//! - optional else branches;
//! - nested conditionals;
//! - branch ownership metadata;
//! - branch boundary semantics;
//! - classical dependency metadata;
//! - quantum dependency metadata;
//! - branch-local optimization eligibility;
//! - branch-equivalence contracts;
//! - conservative semantic-preservation rules;
//! - deterministic structural fingerprints;
//! - overflow-safe accounting;
//! - resource-limit hooks;
//! - no execution;
//! - no hardware dependency;
//! - no routing dependency;
//! - no scheduling dependency;
//! - no backend dependency;
//! - no unsafe code.
//!
//! # Semantic rule
//!
//! A conditional is not merely:
//!
//! ```text
//! if condition {
//!     gates
//! }
//! ```
//!
//! It is semantically:
//!
//! ```text
//! predicate
//!     │
//!     ├── true branch
//!     │
//!     └── false branch
//! ```
//!
//! with dependencies from the predicate's classical inputs into every
//! operation whose execution depends on the predicate.
//!
//! Therefore an optimizer MUST NOT:
//!
//! - move a predicate-dependent operation outside its conditional;
//! - merge branches merely because they contain syntactically similar gates;
//! - remove a condition because both branches currently look empty;
//! - move measurement-dependent operations across their measurement source;
//! - change branch order;
//! - discard a branch predicate;
//! - assume predicates are mutually exclusive without proving it;
//! - assume two predicates are equivalent merely because they have equal text;
//! - treat an unknown predicate as constant.
//!
//! # Current-IR compatibility
//!
//! Because the canonical IR currently does not have a conditional gate, the
//! types in this module use optimizer-local identifiers and primitive values.
//! This avoids coupling the optimizer to a not-yet-existing IR type.
//!
//! Once canonical conditional IR exists, the integration point is:
//!
//! ```text
//! canonical conditional IR
//!          │
//!          ▼
//! Conditional::from_ir(...)
//!          │
//!          ▼
//! optimizer conditional model
//! ```
//!
//! No redesign of this file should be necessary.
//!
//! # Scaling
//!
//! The representation is intentionally compact and immutable after
//! construction. Predicates use recursive `Box` nodes for nested Boolean
//! expressions, which means memory consumption is proportional to the actual
//! predicate expression rather than to the total circuit size.
//!
//! There is no artificial "maximum circuit size" in this module.
//!
//! Extremely large programs remain bounded by:
//!
//! - available memory;
//! - the optimizer's configured resource limits;
//! - the canonical IR limits;
//! - compiler process limits.
//!
//! All recursive algorithms in this file use explicit iterative traversal
//! where practical to avoid stack growth proportional to attacker-controlled
//! predicate depth.
//!
//! # Determinism
//!
//! This module is deterministic:
//!
//! - no randomness;
//! - no hash iteration in semantic ordering;
//! - branch order is explicit;
//! - predicate operands retain deterministic order;
//! - fingerprints are deterministic;
//! - IDs are invocation-local.
//!
//! # Security
//!
//! This module treats conditional metadata as potentially untrusted compiler
//! input.
//!
//! It therefore:
//!
//! - validates identifiers;
//! - rejects malformed structures;
//! - rejects excessive nesting when a caller-supplied structural budget is
//!   exceeded;
//! - detects arithmetic overflow;
//! - never evaluates an unknown predicate as true or false;
//! - never executes classical expressions;
//! - never performs backend I/O.
//!
//! # Rust compatibility
//!
//! Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features.
//! No unsafe code.
//! No external dependencies.
//!
//! -----------------------------------------------------------------------------
//! Integration contract
//! -----------------------------------------------------------------------------
//!
//! `structure/conditional.rs` is intentionally independent of:
//!
//! - `OptimizationPipeline`;
//! - `OptimizationPass`;
//! - `OptimizationContext`;
//! - `OptimizationTarget`;
//! - routing;
//! - scheduling;
//! - hardware;
//! - benchmarking;
//! - execution.
//!
//! It may be consumed by:
//!
//! - `structure/block.rs`;
//! - `structure/region.rs`;
//! - `structure/loop.rs`;
//! - `structure/control_flow.rs`;
//! - `analysis/dependency.rs`;
//! - `analysis/commutation.rs`;
//! - `analysis/liveness.rs`;
//! - `rewrite.rs`;
//! - `pipeline.rs`;
//! - `planner.rs`;
//! - `verification/semantic.rs`.
//!
//! None of those modules need to modify the fundamental predicate or branch
//! representation established here.

use std::fmt;

// ============================================================================
// Identifiers
// ============================================================================

/// Invocation-local identifier for a conditional construct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionalId(usize);

impl ConditionalId {
    /// Creates an invocation-local conditional identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the invocation-local numeric index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for ConditionalId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "conditional{}", self.0)
    }
}

/// Invocation-local identifier for a classical source.
///
/// This deliberately does not assume that the future canonical IR will use
/// any particular classical-register representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassicalSourceId(usize);

impl ClassicalSourceId {
    /// Creates a source identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the source index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for ClassicalSourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "classical{}", self.0)
    }
}

/// Invocation-local identifier for a conditional branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BranchId(usize);

impl BranchId {
    /// Creates a branch identifier.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns the branch index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for BranchId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "branch{}", self.0)
    }
}

// ============================================================================
// Errors
// ============================================================================

/// Errors produced by conditional structural validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionalError {
    /// A conditional contains no branch.
    MissingBranch,

    /// A branch identifier is duplicated.
    DuplicateBranch {
        /// Duplicated branch.
        branch: BranchId,
    },

    /// The same branch is used as both true and false branch.
    AliasedBranches {
        /// Aliased branch.
        branch: BranchId,
    },

    /// A classical source is invalid for the supplied source limit.
    ClassicalSourceOutOfRange {
        /// Invalid source.
        source: ClassicalSourceId,

        /// Number of available sources.
        available: usize,
    },

    /// A conditional identifier is invalid.
    InvalidConditionalId,

    /// A branch contains an invalid operation range.
    InvalidOperationRange {
        /// Start of range.
        start: usize,

        /// End of range.
        end: usize,
    },

    /// A nested predicate exceeded a caller-provided structural limit.
    PredicateDepthExceeded {
        /// Observed depth.
        depth: usize,

        /// Maximum permitted depth.
        maximum: usize,
    },

    /// A predicate exceeded a caller-provided node limit.
    PredicateNodeLimitExceeded {
        /// Observed number of nodes.
        nodes: usize,

        /// Maximum permitted nodes.
        maximum: usize,
    },

    /// A branch nesting depth exceeded a caller-provided structural limit.
    BranchDepthExceeded {
        /// Observed depth.
        depth: usize,

        /// Maximum permitted depth.
        maximum: usize,
    },

    /// Integer arithmetic overflow occurred.
    ArithmeticOverflow {
        /// Calculation that overflowed.
        calculation: &'static str,
    },

    /// A conditional contains semantically contradictory metadata.
    InvalidStructure {
        /// Explanation.
        message: &'static str,
    },

    /// A transformation attempted to cross a semantic boundary.
    SemanticBoundaryViolation {
        /// Explanation.
        message: &'static str,
    },
}

impl fmt::Display for ConditionalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingBranch => {
                write!(formatter, "conditional must contain at least one branch")
            }

            Self::DuplicateBranch { branch } => {
                write!(formatter, "conditional contains duplicate {branch}")
            }

            Self::AliasedBranches { branch } => {
                write!(
                    formatter,
                    "conditional true and false branches alias {branch}"
                )
            }

            Self::ClassicalSourceOutOfRange {
                source,
                available,
            } => {
                write!(
                    formatter,
                    "{source} is outside the available classical-source range \
                     of {available}"
                )
            }

            Self::InvalidConditionalId => {
                write!(formatter, "invalid conditional identifier")
            }

            Self::InvalidOperationRange { start, end } => {
                write!(
                    formatter,
                    "invalid conditional operation range {start}..{end}"
                )
            }

            Self::PredicateDepthExceeded { depth, maximum } => {
                write!(
                    formatter,
                    "predicate depth {depth} exceeds configured maximum {maximum}"
                )
            }

            Self::PredicateNodeLimitExceeded { nodes, maximum } => {
                write!(
                    formatter,
                    "predicate node count {nodes} exceeds configured maximum \
                     {maximum}"
                )
            }

            Self::BranchDepthExceeded { depth, maximum } => {
                write!(
                    formatter,
                    "conditional branch depth {depth} exceeds configured \
                     maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidStructure { message } => {
                write!(formatter, "invalid conditional structure: {message}")
            }

            Self::SemanticBoundaryViolation { message } => {
                write!(
                    formatter,
                    "conditional semantic-boundary violation: {message}"
                )
            }
        }
    }
}

impl std::error::Error for ConditionalError {}

/// Result type used throughout this module.
pub type ConditionalResult<T> = Result<T, ConditionalError>;

// ============================================================================
// Predicate source
// ============================================================================

/// Classical source used by a conditional predicate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClassicalSource {
    /// One classical bit.
    Bit(ClassicalSourceId),

    /// A classical register represented by a source ID.
    Register(ClassicalSourceId),
}

impl ClassicalSource {
    /// Returns the source identifier.
    #[must_use]
    pub const fn id(self) -> ClassicalSourceId {
        match self {
            Self::Bit(id) | Self::Register(id) => id,
        }
    }

    /// Returns true for a single classical bit.
    #[must_use]
    pub const fn is_bit(self) -> bool {
        matches!(self, Self::Bit(_))
    }

    /// Returns true for a register source.
    #[must_use]
    pub const fn is_register(self) -> bool {
        matches!(self, Self::Register(_))
    }
}

// ============================================================================
// Predicate comparison
// ============================================================================

/// Comparison applied to a classical value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PredicateComparison {
    /// Equal.
    Equal,

    /// Not equal.
    NotEqual,

    /// Unsigned less-than.
    LessThan,

    /// Unsigned less-than-or-equal.
    LessThanOrEqual,

    /// Unsigned greater-than.
    GreaterThan,

    /// Unsigned greater-than-or-equal.
    GreaterThanOrEqual,
}

impl PredicateComparison {
    /// Returns the logical inverse comparison.
    #[must_use]
    pub const fn inverse(self) -> Self {
        match self {
            Self::Equal => Self::NotEqual,
            Self::NotEqual => Self::Equal,
            Self::LessThan => Self::GreaterThanOrEqual,
            Self::LessThanOrEqual => Self::GreaterThan,
            Self::GreaterThan => Self::LessThanOrEqual,
            Self::GreaterThanOrEqual => Self::LessThan,
        }
    }
}

// ============================================================================
// Predicate expression
// ============================================================================

/// Immutable Boolean predicate used to control a conditional region.
///
/// The representation is deliberately independent of the canonical Quantum
/// IR so it can survive future changes to classical-control representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Predicate {
    /// Predicate that is always true.
    True,

    /// Predicate that is always false.
    False,

    /// Tests one classical bit.
    BitIs {
        /// Classical source.
        source: ClassicalSource,

        /// Required bit value.
        value: bool,
    },

    /// Compares a classical source with an unsigned integer.
    Compare {
        /// Classical source.
        source: ClassicalSource,

        /// Comparison operator.
        comparison: PredicateComparison,

        /// Constant value.
        value: u64,
    },

    /// Logical negation.
    Not(Box<Predicate>),

    /// Logical conjunction.
    All(Vec<Predicate>),

    /// Logical disjunction.
    Any(Vec<Predicate>),

    /// Exclusive-or of all operands.
    Xor(Vec<Predicate>),
}

impl Predicate {
    /// Returns a constant true predicate.
    #[must_use]
    pub const fn always_true() -> Self {
        Self::True
    }

    /// Returns a constant false predicate.
    #[must_use]
    pub const fn always_false() -> Self {
        Self::False
    }

    /// Creates a single-bit predicate.
    #[must_use]
    pub const fn bit_is(
        source: ClassicalSource,
        value: bool,
    ) -> Self {
        Self::BitIs { source, value }
    }

    /// Creates an equality predicate.
    #[must_use]
    pub const fn equals(
        source: ClassicalSource,
        value: u64,
    ) -> Self {
        Self::Compare {
            source,
            comparison: PredicateComparison::Equal,
            value,
        }
    }

    /// Creates an inequality predicate.
    #[must_use]
    pub const fn not_equals(
        source: ClassicalSource,
        value: u64,
    ) -> Self {
        Self::Compare {
            source,
            comparison: PredicateComparison::NotEqual,
            value,
        }
    }

    /// Creates a generic comparison predicate.
    #[must_use]
    pub const fn compare(
        source: ClassicalSource,
        comparison: PredicateComparison,
        value: u64,
    ) -> Self {
        Self::Compare {
            source,
            comparison,
            value,
        }
    }

    /// Returns a logical negation of this predicate.
    #[must_use]
    pub fn negate(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Not(inner) => *inner,
            other => Self::Not(Box::new(other)),
        }
    }

    /// Creates conjunction while applying safe constant folding.
    #[must_use]
    pub fn all(predicates: Vec<Self>) -> Self {
        let mut result = Vec::with_capacity(predicates.len());

        for predicate in predicates {
            match predicate {
                Self::False => return Self::False,
                Self::True => {}
                Self::All(children) => {
                    result.extend(children);
                }
                other => result.push(other),
            }
        }

        match result.len() {
            0 => Self::True,
            1 => result.into_iter().next().unwrap_or(Self::True),
            _ => Self::All(result),
        }
    }

    /// Creates disjunction while applying safe constant folding.
    #[must_use]
    pub fn any(predicates: Vec<Self>) -> Self {
        let mut result = Vec::with_capacity(predicates.len());

        for predicate in predicates {
            match predicate {
                Self::True => return Self::True,
                Self::False => {}
                Self::Any(children) => {
                    result.extend(children);
                }
                other => result.push(other),
            }
        }

        match result.len() {
            0 => Self::False,
            1 => result.into_iter().next().unwrap_or(Self::False),
            _ => Self::Any(result),
        }
    }

    /// Creates an XOR expression with constant folding limited to constants.
    ///
    /// General XOR algebra is intentionally not performed here because doing
    /// so can become expensive for very large expressions.
    #[must_use]
    pub fn xor(predicates: Vec<Self>) -> Self {
        let mut result = Vec::with_capacity(predicates.len());
        let mut parity = false;

        for predicate in predicates {
            match predicate {
                Self::True => parity = !parity,
                Self::False => {}
                Self::Xor(children) => {
                    for child in children {
                        match child {
                            Self::True => parity = !parity,
                            Self::False => {}
                            other => result.push(other),
                        }
                    }
                }
                other => result.push(other),
            }
        }

        if result.is_empty() {
            return if parity {
                Self::True
            } else {
                Self::False
            };
        }

        if parity {
            result.push(Self::True);
        }

        if result.len() == 1 {
            return result.into_iter().next().unwrap_or(Self::False);
        }

        Self::Xor(result)
    }

    /// Returns the number of predicate nodes.
    ///
    /// Traversal is iterative to avoid stack overflow for deeply nested
    /// predicates.
    #[must_use]
    pub fn node_count(&self) -> usize {
        let mut count = 0usize;
        let mut stack = vec![self];

        while let Some(predicate) = stack.pop() {
            count = count.saturating_add(1);

            match predicate {
                Self::True
                | Self::False
                | Self::BitIs { .. }
                | Self::Compare { .. } => {}

                Self::Not(inner) => {
                    stack.push(inner);
                }

                Self::All(children)
                | Self::Any(children)
                | Self::Xor(children) => {
                    for child in children.iter().rev() {
                        stack.push(child);
                    }
                }
            }
        }

        count
    }

    /// Returns the maximum predicate nesting depth.
    ///
    /// Traversal is iterative.
    #[must_use]
    pub fn depth(&self) -> usize {
        let mut maximum = 0usize;

        let mut stack = vec![(self, 1usize)];

        while let Some((predicate, depth)) = stack.pop() {
            maximum = maximum.max(depth);

            match predicate {
                Self::True
                | Self::False
                | Self::BitIs { .. }
                | Self::Compare { .. } => {}

                Self::Not(inner) => {
                    stack.push((inner, depth.saturating_add(1)));
                }

                Self::All(children)
                | Self::Any(children)
                | Self::Xor(children) => {
                    let next_depth = depth.saturating_add(1);

                    for child in children.iter().rev() {
                        stack.push((child, next_depth));
                    }
                }
            }
        }

        maximum
    }

    /// Returns all classical sources referenced by the predicate.
    ///
    /// The result preserves first-occurrence order and does not use a hash
    /// set, ensuring deterministic output.
    pub fn sources(&self) -> Vec<ClassicalSource> {
        let mut result = Vec::new();
        let mut stack = vec![self];

        while let Some(predicate) = stack.pop() {
            match predicate {
                Self::True | Self::False => {}

                Self::BitIs { source, .. }
                | Self::Compare { source, .. } => {
                    if !result.contains(source) {
                        result.push(*source);
                    }
                }

                Self::Not(inner) => {
                    stack.push(inner);
                }

                Self::All(children)
                | Self::Any(children)
                | Self::Xor(children) => {
                    for child in children.iter().rev() {
                        stack.push(child);
                    }
                }
            }
        }

        result
    }

    /// Returns true when the predicate is statically true.
    #[must_use]
    pub const fn is_true(&self) -> bool {
        matches!(self, Self::True)
    }

    /// Returns true when the predicate is statically false.
    #[must_use]
    pub const fn is_false(&self) -> bool {
        matches!(self, Self::False)
    }

    /// Returns whether this predicate contains runtime classical dependencies.
    #[must_use]
    pub fn is_runtime_dependent(&self) -> bool {
        !matches!(self, Self::True | Self::False)
    }

    /// Returns whether this predicate is structurally equivalent to another
    /// predicate.
    ///
    /// This intentionally uses exact structural equality. Semantic equivalence
    /// such as:
    ///
    /// `a && b == b && a`
    ///
    /// requires a separate theorem/normalization layer and must not be assumed
    /// here.
    #[must_use]
    pub fn structurally_equivalent(&self, other: &Self) -> bool {
        self == other
    }

    /// Returns whether two predicates are conservatively known to be
    /// mutually exclusive.
    ///
    /// This method only returns `true` for cases that can be proven locally.
    /// Unknown relationships return `false`.
    #[must_use]
    pub fn known_mutually_exclusive(
        &self,
        other: &Self,
    ) -> bool {
        match (self, other) {
            (Self::False, _) | (_, Self::False) => true,

            (
                Self::BitIs {
                    source: left_source,
                    value: left_value,
                },
                Self::BitIs {
                    source: right_source,
                    value: right_value,
                },
            ) => left_source == right_source && left_value != right_value,

            (
                Self::Compare {
                    source: left_source,
                    comparison: PredicateComparison::Equal,
                    value: left_value,
                },
                Self::Compare {
                    source: right_source,
                    comparison: PredicateComparison::Equal,
                    value: right_value,
                },
            ) => left_source == right_source && left_value != right_value,

            _ => false,
        }
    }

    /// Validates structural limits.
    pub fn validate(
        &self,
        limits: &PredicateLimits,
    ) -> ConditionalResult<()> {
        let nodes = self.node_count();

        if nodes > limits.max_nodes {
            return Err(
                ConditionalError::PredicateNodeLimitExceeded {
                    nodes,
                    maximum: limits.max_nodes,
                },
            );
        }

        let depth = self.depth();

        if depth > limits.max_depth {
            return Err(
                ConditionalError::PredicateDepthExceeded {
                    depth,
                    maximum: limits.max_depth,
                },
            );
        }

        Ok(())
    }
}

// ============================================================================
// Predicate limits
// ============================================================================

/// Resource limits specifically for conditional predicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PredicateLimits {
    /// Maximum predicate nodes.
    pub max_nodes: usize,

    /// Maximum predicate nesting depth.
    pub max_depth: usize,
}

impl Default for PredicateLimits {
    fn default() -> Self {
        Self {
            max_nodes: 1_000_000,
            max_depth: 16_384,
        }
    }
}

impl PredicateLimits {
    /// Creates limits with explicit values.
    #[must_use]
    pub const fn new(
        max_nodes: usize,
        max_depth: usize,
    ) -> Self {
        Self {
            max_nodes,
            max_depth,
        }
    }
}

// ============================================================================
// Branch operation range
// ============================================================================

/// Half-open logical operation range belonging to a conditional branch.
///
/// This is deliberately an optimizer-local range. It refers to the logical
/// operation snapshot seen by the optimizer and must not be treated as a
/// persistent IR identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BranchOperationRange {
    start: usize,
    end: usize,
}

impl BranchOperationRange {
    /// Creates a validated half-open range.
    pub fn new(
        start: usize,
        end: usize,
    ) -> ConditionalResult<Self> {
        if start > end {
            return Err(ConditionalError::InvalidOperationRange {
                start,
                end,
            });
        }

        Ok(Self { start, end })
    }

    /// Returns the inclusive start boundary.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// Returns the exclusive end boundary.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }

    /// Returns the number of operations.
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }

    /// Returns true when no operations belong to this branch.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

// ============================================================================
// Branch kind
// ============================================================================

/// Semantic role of a conditional branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchKind {
    /// Predicate evaluates to true.
    Then,

    /// Predicate evaluates to false.
    Else,

    /// Explicitly named compiler-generated branch.
    Named,
}

// ============================================================================
// Branch
// ============================================================================

/// Immutable optimizer-side conditional branch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalBranch {
    id: BranchId,
    kind: BranchKind,
    operations: BranchOperationRange,
    nested_conditionals: Vec<ConditionalId>,
    optimizable: bool,
}

impl ConditionalBranch {
    /// Creates a branch.
    pub fn new(
        id: BranchId,
        kind: BranchKind,
        operations: BranchOperationRange,
    ) -> Self {
        Self {
            id,
            kind,
            operations,
            nested_conditionals: Vec::new(),
            optimizable: true,
        }
    }

    /// Returns the branch identifier.
    #[must_use]
    pub const fn id(&self) -> BranchId {
        self.id
    }

    /// Returns the branch kind.
    #[must_use]
    pub const fn kind(&self) -> BranchKind {
        self.kind
    }

    /// Returns the operation range.
    #[must_use]
    pub const fn operations(&self) -> BranchOperationRange {
        self.operations
    }

    /// Returns nested conditionals.
    #[must_use]
    pub fn nested_conditionals(&self) -> &[ConditionalId] {
        &self.nested_conditionals
    }

    /// Adds a nested conditional.
    ///
    /// Duplicate identifiers are rejected.
    pub fn add_nested_conditional(
        &mut self,
        conditional: ConditionalId,
    ) -> ConditionalResult<()> {
        if self.nested_conditionals.contains(&conditional) {
            return Err(ConditionalError::InvalidStructure {
                message: "duplicate nested conditional identifier",
            });
        }

        self.nested_conditionals.push(conditional);

        Ok(())
    }

    /// Returns whether ordinary branch-local optimization is permitted.
    #[must_use]
    pub const fn is_optimizable(&self) -> bool {
        self.optimizable
    }

    /// Sets branch-local optimization eligibility.
    ///
    /// This does not alter semantics.
    pub const fn set_optimizable(
        &mut self,
        optimizable: bool,
    ) {
        self.optimizable = optimizable;
    }
}

// ============================================================================
// Branch boundary policy
// ============================================================================

/// Policy governing transformations at a conditional boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConditionalBoundaryPolicy {
    /// Whether an operation may be moved into the conditional.
    allow_hoist_into_branch: bool,

    /// Whether an operation may be moved out of the conditional.
    allow_sink_out_of_branch: bool,

    /// Whether identical branch operations may be structurally merged.
    allow_branch_merge: bool,

    /// Whether an empty branch may be removed.
    allow_empty_branch_elimination: bool,
}

impl Default for ConditionalBoundaryPolicy {
    fn default() -> Self {
        Self {
            allow_hoist_into_branch: false,
            allow_sink_out_of_branch: false,
            allow_branch_merge: false,
            allow_empty_branch_elimination: false,
        }
    }
}

impl ConditionalBoundaryPolicy {
    /// Creates a conservative policy.
    #[must_use]
    pub const fn conservative() -> Self {
        Self {
            allow_hoist_into_branch: false,
            allow_sink_out_of_branch: false,
            allow_branch_merge: false,
            allow_empty_branch_elimination: false,
        }
    }

    /// Creates a policy in which all transformations still require the
    /// individual pass to prove semantic legality.
    ///
    /// This is intentionally not the default.
    #[must_use]
    pub const fn permissive() -> Self {
        Self {
            allow_hoist_into_branch: true,
            allow_sink_out_of_branch: true,
            allow_branch_merge: true,
            allow_empty_branch_elimination: true,
        }
    }

    /// Returns whether hoisting is permitted.
    #[must_use]
    pub const fn allows_hoist(self) -> bool {
        self.allow_hoist_into_branch
    }

    /// Returns whether sinking is permitted.
    #[must_use]
    pub const fn allows_sink(self) -> bool {
        self.allow_sink_out_of_branch
    }

    /// Returns whether branch merging is permitted.
    #[must_use]
    pub const fn allows_branch_merge(self) -> bool {
        self.allow_branch_merge
    }

    /// Returns whether empty branch elimination is permitted.
    #[must_use]
    pub const fn allows_empty_branch_elimination(self) -> bool {
        self.allow_empty_branch_elimination
    }
}

// ============================================================================
// Conditional
// ============================================================================

/// Complete optimizer-side conditional structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conditional {
    id: ConditionalId,

    /// Classical predicate controlling execution.
    predicate: Predicate,

    /// True branch.
    then_branch: ConditionalBranch,

    /// Optional false branch.
    else_branch: Option<ConditionalBranch>,

    /// Conditional nesting depth.
    depth: usize,

    /// Parent conditional, when nested.
    parent: Option<ConditionalId>,

    /// Transformation policy.
    boundary_policy: ConditionalBoundaryPolicy,
}

impl Conditional {
    /// Creates a conditional containing a mandatory true branch.
    pub fn new(
        id: ConditionalId,
        predicate: Predicate,
        then_branch: ConditionalBranch,
    ) -> ConditionalResult<Self> {
        Self::with_else(
            id,
            predicate,
            then_branch,
            None,
        )
    }

    /// Creates a conditional with an optional else branch.
    pub fn with_else(
        id: ConditionalId,
        predicate: Predicate,
        then_branch: ConditionalBranch,
        else_branch: Option<ConditionalBranch>,
    ) -> ConditionalResult<Self> {
        if then_branch.kind() == BranchKind::Else {
            return Err(ConditionalError::InvalidStructure {
                message: "true branch cannot have Else kind",
            });
        }

        if let Some(branch) = else_branch.as_ref() {
            if branch.kind() == BranchKind::Then {
                return Err(ConditionalError::InvalidStructure {
                    message: "false branch cannot have Then kind",
                });
            }

            if then_branch.id() == branch.id() {
                return Err(ConditionalError::AliasedBranches {
                    branch: branch.id(),
                });
            }
        }

        Ok(Self {
            id,
            predicate,
            then_branch,
            else_branch,
            depth: 0,
            parent: None,
            boundary_policy: ConditionalBoundaryPolicy::conservative(),
        })
    }

    /// Returns the conditional identifier.
    #[must_use]
    pub const fn id(&self) -> ConditionalId {
        self.id
    }

    /// Returns the predicate.
    #[must_use]
    pub fn predicate(&self) -> &Predicate {
        &self.predicate
    }

    /// Returns the true branch.
    #[must_use]
    pub const fn then_branch(&self) -> &ConditionalBranch {
        &self.then_branch
    }

    /// Returns the optional false branch.
    #[must_use]
    pub const fn else_branch(&self) -> Option<&ConditionalBranch> {
        self.else_branch.as_ref()
    }

    /// Returns whether an explicit else branch exists.
    #[must_use]
    pub const fn has_else(&self) -> bool {
        self.else_branch.is_some()
    }

    /// Returns the nesting depth.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.depth
    }

    /// Returns the parent conditional.
    #[must_use]
    pub const fn parent(&self) -> Option<ConditionalId> {
        self.parent
    }

    /// Sets nesting metadata.
    ///
    /// This changes only optimizer structural metadata, not circuit semantics.
    pub const fn set_nesting(
        &mut self,
        parent: Option<ConditionalId>,
        depth: usize,
    ) {
        self.parent = parent;
        self.depth = depth;
    }

    /// Returns the boundary transformation policy.
    #[must_use]
    pub const fn boundary_policy(
        &self,
    ) -> ConditionalBoundaryPolicy {
        self.boundary_policy
    }

    /// Sets the boundary transformation policy.
    pub const fn set_boundary_policy(
        &mut self,
        policy: ConditionalBoundaryPolicy,
    ) {
        self.boundary_policy = policy;
    }

    /// Returns all classical sources referenced by the condition.
    #[must_use]
    pub fn classical_sources(&self) -> Vec<ClassicalSource> {
        self.predicate.sources()
    }

    /// Returns the total number of branch operations.
    ///
    /// This is structural metadata only; it does not inspect the canonical
    /// circuit.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        let then_count = self.then_branch.operations().len();

        let else_count = self
            .else_branch
            .as_ref()
            .map(|branch| branch.operations().len())
            .unwrap_or(0);

        then_count.saturating_add(else_count)
    }

    /// Returns true when the conditional has no branch operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.then_branch.operations().is_empty()
            && self
                .else_branch
                .as_ref()
                .map(|branch| branch.operations().is_empty())
                .unwrap_or(true)
    }

    /// Returns true when the predicate is statically true.
    #[must_use]
    pub fn is_unconditional(&self) -> bool {
        self.predicate.is_true()
    }

    /// Returns true when the predicate is statically false.
    #[must_use]
    pub fn is_dead(&self) -> bool {
        self.predicate.is_false()
    }

    /// Returns whether the conditional contains nested control flow.
    #[must_use]
    pub fn contains_nested_control_flow(&self) -> bool {
        !self.then_branch.nested_conditionals().is_empty()
            || self
                .else_branch
                .as_ref()
                .map(|branch| !branch.nested_conditionals().is_empty())
                .unwrap_or(false)
    }

    /// Validates the conditional and its predicate.
    pub fn validate(
        &self,
        predicate_limits: &PredicateLimits,
        max_branch_depth: usize,
    ) -> ConditionalResult<()> {
        self.predicate.validate(predicate_limits)?;

        if self.depth > max_branch_depth {
            return Err(ConditionalError::BranchDepthExceeded {
                depth: self.depth,
                maximum: max_branch_depth,
            });
        }

        validate_branch(&self.then_branch)?;

        if let Some(branch) = self.else_branch.as_ref() {
            validate_branch(branch)?;

            if branch.id() == self.then_branch.id() {
                return Err(ConditionalError::AliasedBranches {
                    branch: branch.id(),
                });
            }
        }

        Ok(())
    }

    /// Returns true if branch-local optimization is safe by default.
    ///
    /// This means only that optimization is restricted to the interior of a
    /// branch. It does NOT authorize movement across the condition boundary.
    #[must_use]
    pub const fn permits_branch_local_optimization(&self) -> bool {
        true
    }

    /// Returns whether an operation can conservatively be moved from the
    /// branch into the surrounding unconditional region.
    ///
    /// The default is false.
    #[must_use]
    pub fn permits_hoisting(
        &self,
        operation_depends_on_predicate: bool,
    ) -> bool {
        self.boundary_policy.allows_hoist()
            && !operation_depends_on_predicate
            && !self.predicate.is_runtime_dependent()
    }

    /// Returns whether an operation can conservatively be sunk into a branch.
    ///
    /// The default is false.
    #[must_use]
    pub fn permits_sinking(
        &self,
        operation_depends_on_predicate: bool,
    ) -> bool {
        self.boundary_policy.allows_sink()
            && !operation_depends_on_predicate
    }

    /// Returns whether the two branches have predicates that can be merged
    /// without additional logical reasoning.
    ///
    /// Exact structural equality is required.
    #[must_use]
    pub fn branches_have_same_condition(&self) -> bool {
        self.predicate.structurally_equivalent(&self.predicate)
    }

    /// Returns a deterministic structural fingerprint.
    ///
    /// This is NOT a cryptographic hash and must not be used for security.
    /// It is intended for compiler-local equality/caching decisions.
    #[must_use]
    pub fn structural_fingerprint(&self) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;

        hash = mix_u64(hash, self.id.index() as u64);
        hash = mix_predicate(hash, &self.predicate);
        hash = mix_u64(hash, self.then_branch.id().index() as u64);
        hash = mix_u64(
            hash,
            self.then_branch.operations().start() as u64,
        );
        hash = mix_u64(
            hash,
            self.then_branch.operations().end() as u64,
        );

        if let Some(branch) = self.else_branch.as_ref() {
            hash = mix_u64(hash, branch.id().index() as u64);
            hash = mix_u64(hash, branch.operations().start() as u64);
            hash = mix_u64(hash, branch.operations().end() as u64);
        }

        hash
    }
}

// ============================================================================
// Dependency metadata
// ============================================================================

/// Classical dependency of a conditional region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClassicalDependency {
    source: ClassicalSource,
    predicate: ConditionalId,
}

impl ClassicalDependency {
    /// Creates a dependency.
    #[must_use]
    pub const fn new(
        predicate: ConditionalId,
        source: ClassicalSource,
    ) -> Self {
        Self { source, predicate }
    }

    /// Returns the classical source.
    #[must_use]
    pub const fn source(self) -> ClassicalSource {
        self.source
    }

    /// Returns the dependent conditional.
    #[must_use]
    pub const fn conditional(self) -> ConditionalId {
        self.predicate
    }
}

// ============================================================================
// Optimization classification
// ============================================================================

/// Conservative classification of a transformation relative to a
/// conditional boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConditionalTransformation {
    /// Transformation occurs entirely within one branch.
    BranchLocal,

    /// Transformation moves an operation into a branch.
    HoistIntoBranch,

    /// Transformation moves an operation out of a branch.
    SinkOutOfBranch,

    /// Transformation transforms both branches together.
    JointBranchRewrite,

    /// Transformation changes or simplifies the predicate.
    PredicateRewrite,

    /// Transformation removes the conditional.
    ConditionalElimination,

    /// Transformation duplicates work into branches.
    BranchDuplication,

    /// Transformation changes branch structure.
    BranchRestructure,
}

impl ConditionalTransformation {
    /// Returns whether the transformation crosses the conditional boundary.
    #[must_use]
    pub const fn crosses_boundary(self) -> bool {
        !matches!(self, Self::BranchLocal)
    }
}

// ============================================================================
// Rewrite authorization
// ============================================================================

/// Result of checking whether a transformation is authorized by the
/// conditional's structural policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConditionalAuthorization {
    /// Transformation is allowed by structural policy.
    Allowed,

    /// Transformation is forbidden by conservative policy.
    Denied,

    /// Transformation requires a semantic proof outside this module.
    RequiresProof,
}

/// Checks a transformation against conditional boundaries.
///
/// This function intentionally does not prove quantum equivalence. It only
/// enforces structural safety.
#[must_use]
pub fn authorize_transformation(
    conditional: &Conditional,
    transformation: ConditionalTransformation,
) -> ConditionalAuthorization {
    match transformation {
        ConditionalTransformation::BranchLocal => {
            ConditionalAuthorization::Allowed
        }

        ConditionalTransformation::HoistIntoBranch => {
            if conditional.boundary_policy.allows_hoist() {
                ConditionalAuthorization::RequiresProof
            } else {
                ConditionalAuthorization::Denied
            }
        }

        ConditionalTransformation::SinkOutOfBranch => {
            if conditional.boundary_policy.allows_sink() {
                ConditionalAuthorization::RequiresProof
            } else {
                ConditionalAuthorization::Denied
            }
        }

        ConditionalTransformation::JointBranchRewrite
        | ConditionalTransformation::PredicateRewrite
        | ConditionalTransformation::ConditionalElimination
        | ConditionalTransformation::BranchDuplication
        | ConditionalTransformation::BranchRestructure => {
            ConditionalAuthorization::RequiresProof
        }
    }
}

// ============================================================================
// Conditional collection
// ============================================================================

/// Deterministic collection of conditional structures for one optimizer
/// invocation.
///
/// This container provides stable lookup without requiring a global mutable
/// registry.
#[derive(Debug, Default, Clone)]
pub struct ConditionalSet {
    conditionals: Vec<Conditional>,
}

impl ConditionalSet {
    /// Creates an empty conditional set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            conditionals: Vec::new(),
        }
    }

    /// Returns the number of conditionals.
    #[must_use]
    pub fn len(&self) -> usize {
        self.conditionals.len()
    }

    /// Returns whether no conditionals are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.conditionals.is_empty()
    }

    /// Adds a conditional.
    ///
    /// IDs must be unique within this invocation.
    pub fn insert(
        &mut self,
        conditional: Conditional,
    ) -> ConditionalResult<()> {
        if self
            .conditionals
            .iter()
            .any(|existing| existing.id() == conditional.id())
        {
            return Err(ConditionalError::InvalidStructure {
                message: "duplicate conditional identifier",
            });
        }

        self.conditionals.push(conditional);

        Ok(())
    }

    /// Returns a conditional by ID.
    #[must_use]
    pub fn get(
        &self,
        id: ConditionalId,
    ) -> Option<&Conditional> {
        self.conditionals
            .iter()
            .find(|conditional| conditional.id() == id)
    }

    /// Returns all conditionals in deterministic insertion order.
    #[must_use]
    pub fn as_slice(&self) -> &[Conditional] {
        &self.conditionals
    }

    /// Validates every conditional.
    pub fn validate(
        &self,
        predicate_limits: &PredicateLimits,
        max_branch_depth: usize,
    ) -> ConditionalResult<()> {
        for conditional in &self.conditionals {
            conditional.validate(
                predicate_limits,
                max_branch_depth,
            )?;
        }

        Ok(())
    }
}

// ============================================================================
// Internal validation
// ============================================================================

fn validate_branch(
    branch: &ConditionalBranch,
) -> ConditionalResult<()> {
    let range = branch.operations();

    if range.start() > range.end() {
        return Err(ConditionalError::InvalidOperationRange {
            start: range.start(),
            end: range.end(),
        });
    }

    Ok(())
}

// ============================================================================
// Deterministic fingerprinting
// ============================================================================

fn mix_u64(
    current: u64,
    value: u64,
) -> u64 {
    let mut result = current ^ value;
    result = result.wrapping_mul(0x100000001b3);
    result
}

fn mix_predicate(
    mut hash: u64,
    predicate: &Predicate,
) -> u64 {
    let mut stack = vec![predicate];

    while let Some(node) = stack.pop() {
        match node {
            Predicate::True => {
                hash = mix_u64(hash, 1);
            }

            Predicate::False => {
                hash = mix_u64(hash, 2);
            }

            Predicate::BitIs {
                source,
                value,
            } => {
                hash = mix_u64(hash, 3);
                hash = mix_u64(hash, source.id().index() as u64);
                hash = mix_u64(hash, u64::from(*value));
            }

            Predicate::Compare {
                source,
                comparison,
                value,
            } => {
                hash = mix_u64(hash, 4);
                hash = mix_u64(hash, source.id().index() as u64);
                hash = mix_u64(
                    hash,
                    comparison_code(*comparison),
                );
                hash = mix_u64(hash, *value);
            }

            Predicate::Not(inner) => {
                hash = mix_u64(hash, 5);
                stack.push(inner);
            }

            Predicate::All(children) => {
                hash = mix_u64(hash, 6);

                for child in children.iter().rev() {
                    stack.push(child);
                }
            }

            Predicate::Any(children) => {
                hash = mix_u64(hash, 7);

                for child in children.iter().rev() {
                    stack.push(child);
                }
            }

            Predicate::Xor(children) => {
                hash = mix_u64(hash, 8);

                for child in children.iter().rev() {
                    stack.push(child);
                }
            }
        }
    }

    hash
}

const fn comparison_code(
    comparison: PredicateComparison,
) -> u64 {
    match comparison {
        PredicateComparison::Equal => 1,
        PredicateComparison::NotEqual => 2,
        PredicateComparison::LessThan => 3,
        PredicateComparison::LessThanOrEqual => 4,
        PredicateComparison::GreaterThan => 5,
        PredicateComparison::GreaterThanOrEqual => 6,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn bit(index: usize) -> ClassicalSource {
        ClassicalSource::Bit(ClassicalSourceId::new(index))
    }

    fn branch(
        index: usize,
        kind: BranchKind,
        start: usize,
        end: usize,
    ) -> ConditionalBranch {
        ConditionalBranch::new(
            BranchId::new(index),
            kind,
            BranchOperationRange::new(start, end)
                .expect("test range must be valid"),
        )
    }

    #[test]
    fn true_predicate_is_constant() {
        let predicate = Predicate::always_true();

        assert!(predicate.is_true());
        assert!(!predicate.is_false());
        assert!(!predicate.is_runtime_dependent());
        assert_eq!(predicate.node_count(), 1);
        assert_eq!(predicate.depth(), 1);
    }

    #[test]
    fn false_predicate_is_constant() {
        let predicate = Predicate::always_false();

        assert!(predicate.is_false());
        assert!(!predicate.is_true());
        assert!(!predicate.is_runtime_dependent());
    }

    #[test]
    fn negation_folds_constants() {
        assert_eq!(
            Predicate::always_true().negate(),
            Predicate::False
        );

        assert_eq!(
            Predicate::always_false().negate(),
            Predicate::True
        );
    }

    #[test]
    fn double_negation_folds() {
        let predicate =
            Predicate::bit_is(bit(0), true);

        assert_eq!(
            predicate.clone().negate().negate(),
            predicate
        );
    }

    #[test]
    fn conjunction_constant_folds() {
        let predicate = Predicate::all(vec![
            Predicate::True,
            Predicate::bit_is(bit(0), true),
            Predicate::True,
        ]);

        assert_eq!(
            predicate,
            Predicate::bit_is(bit(0), true)
        );
    }

    #[test]
    fn conjunction_false_dominates() {
        let predicate = Predicate::all(vec![
            Predicate::True,
            Predicate::False,
            Predicate::bit_is(bit(0), true),
        ]);

        assert_eq!(predicate, Predicate::False);
    }

    #[test]
    fn disjunction_true_dominates() {
        let predicate = Predicate::any(vec![
            Predicate::False,
            Predicate::True,
            Predicate::bit_is(bit(0), true),
        ]);

        assert_eq!(predicate, Predicate::True);
    }

    #[test]
    fn predicate_sources_are_deterministic() {
        let predicate = Predicate::all(vec![
            Predicate::bit_is(bit(3), true),
            Predicate::bit_is(bit(1), false),
            Predicate::bit_is(bit(3), false),
        ]);

        let sources = predicate.sources();

        assert_eq!(
            sources,
            vec![
                bit(3),
                bit(1),
            ]
        );
    }

    #[test]
    fn mutually_exclusive_bits_are_detected() {
        let left =
            Predicate::bit_is(bit(0), true);

        let right =
            Predicate::bit_is(bit(0), false);

        assert!(left.known_mutually_exclusive(&right));
    }

    #[test]
    fn unrelated_predicates_are_not_assumed_exclusive() {
        let left =
            Predicate::bit_is(bit(0), true);

        let right =
            Predicate::bit_is(bit(1), false);

        assert!(!left.known_mutually_exclusive(&right));
    }

    #[test]
    fn valid_branch_range() {
        let range =
            BranchOperationRange::new(2, 5)
                .expect("range should be valid");

        assert_eq!(range.start(), 2);
        assert_eq!(range.end(), 5);
        assert_eq!(range.len(), 3);
        assert!(!range.is_empty());
    }

    #[test]
    fn empty_branch_range_is_valid() {
        let range =
            BranchOperationRange::new(4, 4)
                .expect("empty range should be valid");

        assert!(range.is_empty());
        assert_eq!(range.len(), 0);
    }

    #[test]
    fn invalid_branch_range_is_rejected() {
        let result =
            BranchOperationRange::new(5, 4);

        assert!(matches!(
            result,
            Err(
                ConditionalError::InvalidOperationRange {
                    start: 5,
                    end: 4
                }
            )
        ));
    }

    #[test]
    fn conditional_requires_valid_true_branch() {
        let result = Conditional::new(
            ConditionalId::new(0),
            Predicate::bit_is(bit(0), true),
            branch(
                0,
                BranchKind::Then,
                0,
                2,
            ),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn conditional_rejects_aliased_branches() {
        let then_branch =
            branch(0, BranchKind::Then, 0, 2);

        let else_branch =
            branch(0, BranchKind::Else, 2, 4);

        let result = Conditional::with_else(
            ConditionalId::new(0),
            Predicate::bit_is(bit(0), true),
            then_branch,
            Some(else_branch),
        );

        assert!(matches!(
            result,
            Err(ConditionalError::AliasedBranches {
                branch: BranchId(0)
            })
        ));
    }

    #[test]
    fn conditional_accepts_else_branch() {
        let result = Conditional::with_else(
            ConditionalId::new(0),
            Predicate::bit_is(bit(0), true),
            branch(
                0,
                BranchKind::Then,
                0,
                2,
            ),
            Some(branch(
                1,
                BranchKind::Else,
                2,
                4,
            )),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn default_boundary_policy_is_conservative() {
        let policy =
            ConditionalBoundaryPolicy::default();

        assert!(!policy.allows_hoist());
        assert!(!policy.allows_sink());
        assert!(!policy.allows_branch_merge());
        assert!(!policy.allows_empty_branch_elimination());
    }

    #[test]
    fn branch_local_transform_is_allowed() {
        let conditional = Conditional::new(
            ConditionalId::new(0),
            Predicate::bit_is(bit(0), true),
            branch(
                0,
                BranchKind::Then,
                0,
                2,
            ),
        )
        .expect("conditional should be valid");

        assert_eq!(
            authorize_transformation(
                &conditional,
                ConditionalTransformation::BranchLocal,
            ),
            ConditionalAuthorization::Allowed
        );
    }

    #[test]
    fn boundary_transform_requires_proof_or_is_denied() {
        let conditional = Conditional::new(
            ConditionalId::new(0),
            Predicate::bit_is(bit(0), true),
            branch(
                0,
                BranchKind::Then,
                0,
                2,
            ),
        )
        .expect("conditional should be valid");

        assert_eq!(
            authorize_transformation(
                &conditional,
                ConditionalTransformation::HoistIntoBranch,
            ),
            ConditionalAuthorization::Denied
        );

        assert_eq!(
            authorize_transformation(
                &conditional,
                ConditionalTransformation::PredicateRewrite,
            ),
            ConditionalAuthorization::RequiresProof
        );
    }

    #[test]
    fn conditional_validation_succeeds() {
        let conditional = Conditional::new(
            ConditionalId::new(0),
            Predicate::bit_is(bit(0), true),
            branch(
                0,
                BranchKind::Then,
                0,
                2,
            ),
        )
        .expect("conditional should be valid");

        conditional
            .validate(
                &PredicateLimits::default(),
                1024,
            )
            .expect("conditional should validate");
    }

    #[test]
    fn predicate_limits_are_enforced() {
        let predicate =
            Predicate::all(vec![
                Predicate::bit_is(bit(0), true),
                Predicate::bit_is(bit(1), true),
                Predicate::bit_is(bit(2), true),
            ]);

        let limits =
            PredicateLimits::new(2, 32);

        assert!(matches!(
            predicate.validate(&limits),
            Err(
                ConditionalError::PredicateNodeLimitExceeded {
                    ..
                }
            )
        ));
    }

    #[test]
    fn deeply_nested_predicate_is_iteratively_counted() {
        let mut predicate =
            Predicate::bit_is(bit(0), true);

        for _ in 0..1_000 {
            predicate =
                Predicate::Not(Box::new(predicate));
        }

        assert_eq!(
            predicate.node_count(),
            1_001
        );

        assert_eq!(
            predicate.depth(),
            1_001
        );
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let first = Conditional::new(
            ConditionalId::new(0),
            Predicate::bit_is(bit(2), true),
            branch(
                0,
                BranchKind::Then,
                10,
                20,
            ),
        )
        .expect("conditional should be valid");

        let second = Conditional::new(
            ConditionalId::new(0),
            Predicate::bit_is(bit(2), true),
            branch(
                0,
                BranchKind::Then,
                10,
                20,
            ),
        )
        .expect("conditional should be valid");

        assert_eq!(
            first.structural_fingerprint(),
            second.structural_fingerprint()
        );
    }

    #[test]
    fn conditional_set_rejects_duplicate_ids() {
        let mut set =
            ConditionalSet::new();

        let first = Conditional::new(
            ConditionalId::new(0),
            Predicate::bit_is(bit(0), true),
            branch(
                0,
                BranchKind::Then,
                0,
                1,
            ),
        )
        .expect("conditional should be valid");

        let second = Conditional::new(
            ConditionalId::new(0),
            Predicate::bit_is(bit(1), true),
            branch(
                1,
                BranchKind::Then,
                1,
                2,
            ),
        )
        .expect("conditional should be valid");

        set.insert(first)
            .expect("first insertion should succeed");

        assert!(matches!(
            set.insert(second),
            Err(ConditionalError::InvalidStructure {
                message: "duplicate conditional identifier"
            })
        ));
    }
}


  