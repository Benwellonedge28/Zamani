//! # Zamani Native AST — Match Expression
//!
//! Production-ready source-level representation of a match expression.
//!
//! ## Architectural position
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
//! MatchExpression
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── hybrid IR
//!     ├── HDL IR
//!     └── future domain IRs
//! ```
//!
//! ## Responsibility
//!
//! This module owns the source-level structural representation of a match
//! expression and its ordered match arms.
//!
//! It represents:
//!
//! - the expression being matched;
//! - an ordered, arbitrarily sized sequence of match arms;
//! - each arm's source-level pattern;
//! - an optional guard;
//! - the arm result/body;
//! - common AST node identity;
//! - source location;
//! - source-level metadata.
//!
//! It deliberately does **not** represent:
//!
//! - resolved symbols;
//! - resolved types;
//! - exhaustiveness results;
//! - reachability results;
//! - decision trees;
//! - jump tables;
//! - branch prediction;
//! - CFG blocks;
//! - SSA values;
//! - quantum hardware;
//! - physical qubits;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - backend instructions.
//!
//! Those concerns belong to later compiler stages.
//!
//! ## POCO-REAF
//!
//! Match expressions contain no assumptions about:
//!
//! - machine size;
//! - register width;
//! - qubit count;
//! - processor architecture;
//! - quantum technology;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - fixed number of match arms.
//!
//! A program may contain any number of arms representable by the available
//! compiler resources and configured compiler safety policies.
//!
//! ## Child representation
//!
//! Cross-node references use [`NodeId`].
//!
//! ```text
//! MatchExpression
//! ├── Node
//! ├── scrutinee: NodeId
//! └── arms: Vec<MatchArm>
//!
//! MatchArm
//! ├── Node
//! ├── pattern: NodeId
//! ├── guard: Option<NodeId>
//! └── body: NodeId
//! ```
//!
//! This avoids recursive Rust ownership structures and permits the repository's
//! AST graph/traversal layer to control traversal strategy.
//!
//! ## Determinism
//!
//! Match-arm ordering is semantically significant and is therefore preserved
//! exactly as supplied by the parser.
//!
//! No hash-map iteration participates in the representation.
//!
//! ## Domain neutrality
//!
//! A match expression can later operate on:
//!
//! - classical values;
//! - resource states;
//! - measurement-derived values;
//! - hybrid values;
//! - accelerator values;
//! - distributed values;
//! - future computational-domain values.
//!
//! This module does not need to know which.
//!
//! ## Structural validation boundary
//!
//! Local validation checks only invariants that can be established from this
//! node and its immediate children:
//!
//! - the node has the correct node kind;
//! - the scrutinee does not reference the match node itself;
//! - no arm references the match node itself;
//! - an arm's pattern, guard and body do not alias each other;
//! - arm node identities are unique within the supplied vector;
//! - the match node is not reused as an arm node.
//!
//! It does not validate:
//!
//! - whether referenced NodeIds exist in the global AST;
//! - pattern type compatibility;
//! - guard type;
//! - exhaustiveness;
//! - unreachable arms;
//! - binding correctness;
//! - semantic ownership;
//! - backend support.
//!
//! Those require later compiler context.
//!
//! ## Scalability
//!
//! The arm collection is dynamically sized.
//!
//! There is no:
//!
//! ```text
//! MAX_ARMS
//! MAX_PATTERN_SIZE
//! MAX_MATCH_DEPTH
//! MAX_QUANTUM_MATCHES
//! ```
//!
//! in this module.
//!
//! Safety/resource limits belong to configurable compiler policy rather than
//! language semantics.
//!
//! ## Security
//!
//! This module:
//!
//! - forbids unsafe Rust;
//! - performs no I/O;
//! - executes no user program;
//! - performs no unchecked indexing;
//! - does not dereference pointers;
//! - does not use global mutable state;
//! - does not recursively walk the AST;
//! - does not inspect backend state.
//!
//! ## Integration
//!
//! ### `node.rs`
//!
//! Provides [`Node`] and [`AstNode`].
//!
//! ### `node_id.rs`
//!
//! Provides [`NodeId`].
//!
//! ### `node_kind.rs`
//!
//! Provides [`CoreNodeKind::MatchExpression`].
//!
//! ### `metadata.rs`
//!
//! Provides [`NodeMetadata`].
//!
//! ### `source`
//!
//! Provides [`Span`].
//!
//! ### `expressions/mod.rs`
//!
//! Must expose:
//!
//! ```text
//! pub mod match_expr;
//! ```
//!
//! ### Parser
//!
//! The parser creates:
//!
//! 1. the scrutinee expression;
//! 2. each pattern;
//! 3. each optional guard;
//! 4. each arm body;
//! 5. the match node.
//!
//! It then stores their stable [`NodeId`]s in this representation.
//!
//! The parser must not perform semantic exhaustiveness checking or hardware
//! lowering.
//!
//! ### Semantic analysis
//!
//! Semantic analysis consumes this representation to determine:
//!
//! - pattern validity;
//! - bindings;
//! - guard validity;
//! - type compatibility;
//! - exhaustiveness;
//! - reachability;
//! - effects;
//! - resource semantics;
//! - domain semantics.
//!
//! ### ZUIR
//!
//! ZUIR lowering occurs after semantic analysis.
//!
//! This module has no ZUIR dependency.
//!
//! ### Optimization
//!
//! Decision-tree construction, branch elimination, jump-table formation,
//! predication and other optimizations belong downstream.
//!
//! ### Quantum compilation
//!
//! A match whose scrutinee ultimately derives from measurement may eventually
//! become dynamic quantum/classical control. That is downstream semantic and
//! target lowering work.
//!
//! No quantum-specific representation is stored here.
//!
//! ## No-re-edit contract
//!
//! Changes to:
//!
//! - quantum IR;
//! - QIR;
//! - ZUIR;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - hardware;
//! - runtime;
//! - backend providers
//!
//! must not require modifying this file.
//!
//! This file changes only when the source-level semantics of match expressions
//! change.
//!
//! ## Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema version of the match-expression source AST contract.
///
/// This is deliberately independent from the global AST serialization schema
/// and from the Zamani language version.
pub const MATCH_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level name of a match expression.
pub const MATCH_EXPRESSION_KIND_NAME: &str = "zamani:match-expression";

/// Stable source-level name of a match arm.
pub const MATCH_ARM_KIND_NAME: &str = "zamani:match-arm";

/// A source-level match expression.
///
/// The actual child AST nodes are owned by the surrounding AST graph and are
/// referenced through [`NodeId`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchExpression {
    /// Common source-level node information.
    node: Node,

    /// Expression whose value is matched.
    scrutinee: NodeId,

    /// Ordered match arms.
    ///
    /// A `Vec` is used intentionally: arm ordering is semantically significant.
    /// There is no fixed maximum.
    arms: Vec<MatchArm>,
}

/// One ordered arm of a [`MatchExpression`].
///
/// A match arm consists of:
///
/// ```text
/// pattern
/// optional guard
/// body
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchArm {
    /// Common source-level node information for the arm.
    node: Node,

    /// Pattern tested against the match scrutinee.
    pattern: NodeId,

    /// Optional source-level guard evaluated after the pattern matches.
    guard: Option<NodeId>,

    /// Expression produced/executed when this arm is selected.
    body: NodeId,
}

/// Stable validation errors detectable without global AST or semantic context.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MatchValidationError {
    /// The match node has the wrong node kind.
    InvalidMatchNodeKind {
        /// Actual kind found.
        actual: NodeKind,
    },

    /// An arm has the wrong node kind.
    InvalidArmNodeKind {
        /// Index of the invalid arm.
        arm_index: usize,
        /// Actual kind found.
        actual: NodeKind,
    },

    /// The scrutinee points at the match node itself.
    SelfReferentialScrutinee,

    /// An arm node points at the enclosing match node.
    SelfReferentialArm {
        /// Index of the offending arm.
        arm_index: usize,
    },

    /// An arm's pattern points at the arm node itself.
    SelfReferentialPattern {
        /// Index of the offending arm.
        arm_index: usize,
    },

    /// An arm's guard points at the arm node itself.
    SelfReferentialGuard {
        /// Index of the offending arm.
        arm_index: usize,
    },

    /// An arm's body points at the arm node itself.
    SelfReferentialBody {
        /// Index of the offending arm.
        arm_index: usize,
    },

    /// Pattern and guard refer to the same child.
    PatternGuardAlias {
        /// Index of the offending arm.
        arm_index: usize,
        /// Shared child ID.
        id: NodeId,
    },

    /// Pattern and body refer to the same child.
    PatternBodyAlias {
        /// Index of the offending arm.
        arm_index: usize,
        /// Shared child ID.
        id: NodeId,
    },

    /// Guard and body refer to the same child.
    GuardBodyAlias {
        /// Index of the offending arm.
        arm_index: usize,
        /// Shared child ID.
        id: NodeId,
    },

    /// Two distinct arms use the same arm-node identity.
    DuplicateArmId {
        /// Index of the first occurrence.
        first_index: usize,
        /// Index of the second occurrence.
        second_index: usize,
        /// Duplicated identity.
        id: NodeId,
    },

    /// A match arm uses the same node identity as the enclosing match.
    MatchNodeUsedAsArm {
        /// Index of the offending arm.
        arm_index: usize,
    },
}

impl fmt::Display for MatchValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMatchNodeKind { actual } => {
                write!(
                    formatter,
                    "match expression has invalid AST node kind: {actual}"
                )
            }

            Self::InvalidArmNodeKind { arm_index, actual } => {
                write!(
                    formatter,
                    "match arm {arm_index} has invalid AST node kind: {actual}"
                )
            }

            Self::SelfReferentialScrutinee => {
                formatter.write_str(
                    "match expression cannot use itself as its scrutinee",
                )
            }

            Self::SelfReferentialArm { arm_index } => {
                write!(
                    formatter,
                    "match arm {arm_index} cannot use the enclosing match node as its identity"
                )
            }

            Self::SelfReferentialPattern { arm_index } => {
                write!(
                    formatter,
                    "match arm {arm_index} cannot use itself as its pattern"
                )
            }

            Self::SelfReferentialGuard { arm_index } => {
                write!(
                    formatter,
                    "match arm {arm_index} cannot use itself as its guard"
                )
            }

            Self::SelfReferentialBody { arm_index } => {
                write!(
                    formatter,
                    "match arm {arm_index} cannot use itself as its body"
                )
            }

            Self::PatternGuardAlias { arm_index, id } => {
                write!(
                    formatter,
                    "match arm {arm_index} pattern and guard alias node {id:?}"
                )
            }

            Self::PatternBodyAlias { arm_index, id } => {
                write!(
                    formatter,
                    "match arm {arm_index} pattern and body alias node {id:?}"
                )
            }

            Self::GuardBodyAlias { arm_index, id } => {
                write!(
                    formatter,
                    "match arm {arm_index} guard and body alias node {id:?}"
                )
            }

            Self::DuplicateArmId {
                first_index,
                second_index,
                id,
            } => {
                write!(
                    formatter,
                    "match arms {first_index} and {second_index} use duplicate node id {id:?}"
                )
            }

            Self::MatchNodeUsedAsArm { arm_index } => {
                write!(
                    formatter,
                    "match arm {arm_index} uses the enclosing match node identity"
                )
            }
        }
    }
}

impl std::error::Error for MatchValidationError {}

impl MatchExpression {
    /// Creates a match expression from an already-created common [`Node`].
    ///
    /// This constructor does not rewrite the supplied node kind.
    /// [`Self::validate_structure`] can therefore be used to validate it.
    #[must_use]
    pub fn from_node(
        node: Node,
        scrutinee: NodeId,
        arms: Vec<MatchArm>,
    ) -> Self {
        Self {
            node,
            scrutinee,
            arms,
        }
    }

    /// Creates a new match expression with the canonical match node kind.
    ///
    /// No semantic validation is performed here.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        scrutinee: NodeId,
        arms: Vec<MatchArm>,
    ) -> Self {
        Self {
            node: Node::new(
                id,
                NodeKind::Core(CoreNodeKind::MatchExpression),
                span,
                metadata,
            ),
            scrutinee,
            arms,
        }
    }

    /// Returns the common AST node.
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns the match expression's stable [`NodeId`].
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the source span of the complete match expression.
    #[must_use]
    pub fn span(&self) -> Span {
        self.node.span()
    }

    /// Returns source-level metadata.
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns the matched expression.
    #[must_use]
    pub fn scrutinee(&self) -> NodeId {
        self.scrutinee
    }

    /// Returns the ordered match arms.
    #[must_use]
    pub fn arms(&self) -> &[MatchArm] {
        &self.arms
    }

    /// Returns a mutable view of the ordered arms.
    ///
    /// This intentionally exposes only the arm collection. Structural
    /// validation remains available through [`Self::validate_structure`].
    #[must_use]
    pub fn arms_mut(&mut self) -> &mut Vec<MatchArm> {
        &mut self.arms
    }

    /// Returns the number of match arms.
    #[must_use]
    pub fn arm_count(&self) -> usize {
        self.arms.len()
    }

    /// Returns the arm at `index`, if it exists.
    #[must_use]
    pub fn arm(&self, index: usize) -> Option<&MatchArm> {
        self.arms.get(index)
    }

    /// Returns the mutable arm at `index`, if it exists.
    #[must_use]
    pub fn arm_mut(&mut self, index: usize) -> Option<&mut MatchArm> {
        self.arms.get_mut(index)
    }

    /// Returns deterministic direct child IDs.
    ///
    /// Ordering is:
    ///
    /// 1. scrutinee;
    /// 2. pattern;
    /// 3. guard, when present;
    /// 4. body;
    /// 5. next arm;
    /// 6. repeat.
    ///
    /// No AST graph lookup is performed.
    pub fn child_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.arms.iter().flat_map(|arm| {
            core::iter::once(arm.pattern())
                .chain(arm.guard())
                .chain(core::iter::once(arm.body()))
        })
        .chain(core::iter::once(self.scrutinee))
    }

    /// Validates invariants that can be checked without access to the complete
    /// AST graph or semantic environment.
    pub fn validate_structure(&self) -> Result<(), MatchValidationError> {
        if self.node.kind()
            != NodeKind::Core(CoreNodeKind::MatchExpression)
        {
            return Err(MatchValidationError::InvalidMatchNodeKind {
                actual: self.node.kind().clone(),
            });
        }

        if self.scrutinee == self.id() {
            return Err(MatchValidationError::SelfReferentialScrutinee);
        }

        for (arm_index, arm) in self.arms.iter().enumerate() {
            if arm.id() == self.id() {
                return Err(MatchValidationError::MatchNodeUsedAsArm {
                    arm_index,
                });
            }

            if arm.node.kind() != NodeKind::Core(CoreNodeKind::MatchArm) {
                return Err(MatchValidationError::InvalidArmNodeKind {
                    arm_index,
                    actual: arm.node.kind().clone(),
                });
            }

            if arm.pattern == arm.id() {
                return Err(MatchValidationError::SelfReferentialPattern {
                    arm_index,
                });
            }

            if let Some(guard) = arm.guard {
                if guard == arm.id() {
                    return Err(MatchValidationError::SelfReferentialGuard {
                        arm_index,
                    });
                }
            }

            if arm.body == arm.id() {
                return Err(MatchValidationError::SelfReferentialBody {
                    arm_index,
                });
            }

            if let Some(guard) = arm.guard {
                if arm.pattern == guard {
                    return Err(MatchValidationError::PatternGuardAlias {
                        arm_index,
                        id: arm.pattern,
                    });
                }

                if guard == arm.body {
                    return Err(MatchValidationError::GuardBodyAlias {
                        arm_index,
                        id: guard,
                    });
                }
            }

            if arm.pattern == arm.body {
                return Err(MatchValidationError::PatternBodyAlias {
                    arm_index,
                    id: arm.pattern,
                });
            }
        }

        for (first_index, first) in self.arms.iter().enumerate() {
            for (second_index, second) in self
                .arms
                .iter()
                .enumerate()
                .skip(first_index + 1)
            {
                if first.id() == second.id() {
                    return Err(MatchValidationError::DuplicateArmId {
                        first_index,
                        second_index,
                        id: first.id(),
                    });
                }
            }
        }

        Ok(())
    }
}

impl AstNode for MatchExpression {
    fn node(&self) -> &Node {
        &self.node
    }
}

impl MatchArm {
    /// Creates a match arm from an existing common AST node.
    #[must_use]
    pub fn from_node(
        node: Node,
        pattern: NodeId,
        guard: Option<NodeId>,
        body: NodeId,
    ) -> Self {
        Self {
            node,
            pattern,
            guard,
            body,
        }
    }

    /// Creates a canonical match arm.
    #[must_use]
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        pattern: NodeId,
        guard: Option<NodeId>,
        body: NodeId,
    ) -> Self {
        Self {
            node: Node::new(
                id,
                NodeKind::Core(CoreNodeKind::MatchArm),
                span,
                metadata,
            ),
            pattern,
            guard,
            body,
        }
    }

    /// Returns the common AST node.
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns the arm's stable node identity.
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the arm's source span.
    #[must_use]
    pub fn span(&self) -> Span {
        self.node.span()
    }

    /// Returns the arm's source-level metadata.
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns the pattern node.
    #[must_use]
    pub fn pattern(&self) -> NodeId {
        self.pattern
    }

    /// Returns the optional guard node.
    #[must_use]
    pub fn guard(&self) -> Option<NodeId> {
        self.guard
    }

    /// Returns the arm body/result node.
    #[must_use]
    pub fn body(&self) -> NodeId {
        self.body
    }

    /// Returns deterministic direct child IDs.
    ///
    /// Ordering:
    ///
    /// 1. pattern;
    /// 2. guard, when present;
    /// 3. body.
    pub fn child_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        core::iter::once(self.pattern)
            .chain(self.guard)
            .chain(core::iter::once(self.body))
    }

    /// Validates only invariants local to this arm.
    pub fn validate_structure(&self) -> Result<(), MatchValidationError> {
        if self.node.kind() != NodeKind::Core(CoreNodeKind::MatchArm) {
            return Err(MatchValidationError::InvalidArmNodeKind {
                arm_index: 0,
                actual: self.node.kind().clone(),
            });
        }

        if self.pattern == self.id() {
            return Err(MatchValidationError::SelfReferentialPattern {
                arm_index: 0,
            });
        }

        if let Some(guard) = self.guard {
            if guard == self.id() {
                return Err(MatchValidationError::SelfReferentialGuard {
                    arm_index: 0,
                });
            }

            if guard == self.pattern {
                return Err(MatchValidationError::PatternGuardAlias {
                    arm_index: 0,
                    id: guard,
                });
            }

            if guard == self.body {
                return Err(MatchValidationError::GuardBodyAlias {
                    arm_index: 0,
                    id: guard,
                });
            }
        }

        if self.body == self.id() {
            return Err(MatchValidationError::SelfReferentialBody {
                arm_index: 0,
            });
        }

        if self.pattern == self.body {
            return Err(MatchValidationError::PatternBodyAlias {
                arm_index: 0,
                id: self.pattern,
            });
        }

        Ok(())
    }
}

impl AstNode for MatchArm {
    fn node(&self) -> &Node {
        &self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::from_raw(value)
    }

    fn metadata() -> NodeMetadata {
        NodeMetadata::default()
    }

    fn span() -> Span {
        Span::default()
    }

    #[test]
    fn match_expression_uses_match_node_kind() {
        let expression = MatchExpression::new(
            node_id(1),
            span(),
            metadata(),
            node_id(2),
            Vec::new(),
        );

        assert_eq!(
            expression.node().kind(),
            NodeKind::Core(CoreNodeKind::MatchExpression)
        );
    }

    #[test]
    fn match_arm_uses_match_arm_node_kind() {
        let arm = MatchArm::new(
            node_id(2),
            span(),
            metadata(),
            node_id(3),
            None,
            node_id(4),
        );

        assert_eq!(
            arm.node().kind(),
            NodeKind::Core(CoreNodeKind::MatchArm)
        );
    }

    #[test]
    fn empty_match_is_structurally_valid() {
        let expression = MatchExpression::new(
            node_id(1),
            span(),
            metadata(),
            node_id(2),
            Vec::new(),
        );

        assert!(expression.validate_structure().is_ok());
    }

    #[test]
    fn match_with_guard_is_structurally_valid() {
        let arm = MatchArm::new(
            node_id(2),
            span(),
            metadata(),
            node_id(3),
            Some(node_id(4)),
            node_id(5),
        );

        let expression = MatchExpression::new(
            node_id(1),
            span(),
            metadata(),
            node_id(6),
            vec![arm],
        );

        assert!(expression.validate_structure().is_ok());
    }

    #[test]
    fn self_referential_scrutinee_is_rejected() {
        let expression = MatchExpression::new(
            node_id(1),
            span(),
            metadata(),
            node_id(1),
            Vec::new(),
        );

        assert_eq!(
            expression.validate_structure(),
            Err(MatchValidationError::SelfReferentialScrutinee)
        );
    }

    #[test]
    fn self_referential_arm_is_rejected() {
        let arm = MatchArm::new(
            node_id(1),
            span(),
            metadata(),
            node_id(2),
            None,
            node_id(3),
        );

        let expression = MatchExpression::new(
            node_id(4),
            span(),
            metadata(),
            node_id(5),
            vec![arm],
        );

        let result = expression.validate_structure();

        assert_eq!(
            result,
            Err(MatchValidationError::MatchNodeUsedAsArm {
                arm_index: 0
            })
        );
    }

    #[test]
    fn pattern_guard_alias_is_rejected() {
        let arm = MatchArm::new(
            node_id(2),
            span(),
            metadata(),
            node_id(3),
            Some(node_id(3)),
            node_id(4),
        );

        let expression = MatchExpression::new(
            node_id(1),
            span(),
            metadata(),
            node_id(5),
            vec![arm],
        );

        assert_eq!(
            expression.validate_structure(),
            Err(MatchValidationError::PatternGuardAlias {
                arm_index: 0,
                id: node_id(3)
            })
        );
    }

    #[test]
    fn pattern_body_alias_is_rejected() {
        let arm = MatchArm::new(
            node_id(2),
            span(),
            metadata(),
            node_id(3),
            None,
            node_id(3),
        );

        let expression = MatchExpression::new(
            node_id(1),
            span(),
            metadata(),
            node_id(5),
            vec![arm],
        );

        assert_eq!(
            expression.validate_structure(),
            Err(MatchValidationError::PatternBodyAlias {
                arm_index: 0,
                id: node_id(3)
            })
        );
    }

    #[test]
    fn child_order_is_deterministic() {
        let first = MatchArm::new(
            node_id(2),
            span(),
            metadata(),
            node_id(3),
            Some(node_id(4)),
            node_id(5),
        );

        let second = MatchArm::new(
            node_id(6),
            span(),
            metadata(),
            node_id(7),
            None,
            node_id(8),
        );

        let expression = MatchExpression::new(
            node_id(1),
            span(),
            metadata(),
            node_id(9),
            vec![first, second],
        );

        let children: Vec<_> = expression.child_ids().collect();

        assert_eq!(
            children,
            vec![
                node_id(3),
                node_id(4),
                node_id(5),
                node_id(7),
                node_id(8),
                node_id(9),
            ]
        );
    }

    #[test]
    fn serialization_round_trip_preserves_structure() {
        let arm = MatchArm::new(
            node_id(2),
            span(),
            metadata(),
            node_id(3),
            Some(node_id(4)),
            node_id(5),
        );

        let original = MatchExpression::new(
            node_id(1),
            span(),
            metadata(),
            node_id(6),
            vec![arm],
        );

        let encoded =
            serde_json::to_string(&original).expect("serialization must succeed");

        let decoded: MatchExpression =
            serde_json::from_str(&encoded)
                .expect("deserialization must succeed");

        assert_eq!(original, decoded);
    }
}