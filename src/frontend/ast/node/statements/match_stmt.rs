//! # Zamani Native AST — Match Statement
//!
//! Production-ready source-level representation of a Zamani `match`
//! statement.
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
//! MatchStatement
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
//!     └── future-domain IRs
//! ```
//!
//! ## Responsibility
//!
//! This module owns the source-level structural representation of a native
//! Zamani `match` statement.
//!
//! It represents:
//!
//! - the common AST node;
//! - the expression being matched;
//! - the ordered match-arm references;
//! - source identity;
//! - source span;
//! - source metadata.
//!
//! It does NOT represent:
//!
//! - name resolution;
//! - type checking;
//! - pattern exhaustiveness;
//! - unreachable-arm analysis;
//! - decision trees;
//! - control-flow graphs;
//! - SSA;
//! - branch prediction;
//! - jump-table generation;
//! - quantum routing;
//! - quantum scheduling;
//! - QEC;
//! - calibration;
//! - hardware topology;
//! - physical resources;
//! - backend instructions;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - runtime state.
//!
//! Those concerns belong to later compiler stages.
//!
//! ## POCO-REAF
//!
//! The representation deliberately contains no assumptions about:
//!
//! - machine size;
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - QPU architecture;
//! - qubit count;
//! - register width;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - number of match arms.
//!
//! Match-arm storage grows according to available resources and caller-owned
//! compiler policies. No artificial language-level maximum is encoded here.
//!
//! The Zamani objective is:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! This node therefore describes programmer intent, not its eventual
//! implementation.
//!
//! ## Source structure
//!
//! The canonical structure is:
//!
//! ```text
//! MatchStatement
//! ├── Node
//! ├── scrutinee: NodeId
//! └── arms: Vec<NodeId>
//! ```
//!
//! `NodeId` references are resolved by the enclosing AST graph.
//!
//! The referenced arm nodes are expected to be represented by the canonical
//! match-arm AST abstraction. This file does not duplicate that abstraction.
//!
//! ## Determinism
//!
//! Match-arm ordering is semantically significant. The original source order
//! is therefore preserved exactly in the `Vec<NodeId>`.
//!
//! No unordered collection is used to represent source order.
//!
//! ## Structural validation boundary
//!
//! This module validates only local structural invariants:
//!
//! - the enclosing node has `CoreNodeKind::MatchStatement`;
//! - the scrutinee is a non-null `NodeId`;
//! - every arm reference is a non-null `NodeId`;
//! - the enclosing node is not referenced as its own scrutinee;
//! - the enclosing node is not referenced as an arm;
//! - duplicate arm references are rejected;
//! - caller-provided resource limits are respected.
//!
//! It cannot determine whether a `NodeId` actually exists in the global AST.
//! That requires AST-graph validation.
//!
//! It cannot determine whether an arm is exhaustive or reachable.
//! That requires semantic analysis.
//!
//! ## Parser contract
//!
//! The parser is responsible for:
//!
//! 1. parsing the `match` keyword;
//! 2. parsing the scrutinee expression;
//! 3. parsing the match cases/arms;
//! 4. allocating node identities;
//! 5. preserving source spans;
//! 6. constructing the match-arm nodes;
//! 7. constructing this `MatchStatement`.
//!
//! The parser must not perform semantic exhaustiveness analysis or target
//! lowering.
//!
//! ## Semantic contract
//!
//! Semantic analysis consumes this node and resolves:
//!
//! - the scrutinee's type;
//! - pattern compatibility;
//! - pattern bindings;
//! - guards;
//! - exhaustiveness;
//! - unreachable arms;
//! - effects;
//! - capabilities;
//! - resource semantics;
//! - domain semantics.
//!
//! Semantic information must not be written into this source node.
//!
//! ## ZUIR contract
//!
//! The node is lowered through the semantic model into ZUIR.
//!
//! This module has no ZUIR dependency.
//!
//! Conceptually:
//!
//! ```text
//! MatchStatement
//!      │
//!      ▼
//! Semantic Model
//!      │
//!      ▼
//! ZUIR control flow
//! ```
//!
//! ZUIR may subsequently lower the construct into classical, quantum,
//! hybrid, HDL, distributed, accelerator, or future-domain representations.
//!
//! ## Quantum integration
//!
//! A match statement may control quantum/classical computation when its
//! scrutinee ultimately derives from a measurement or another quantum-related
//! value.
//!
//! This node does not need a quantum-specific representation for that case.
//!
//! For example, the semantic layer may eventually transform:
//!
//! ```text
//! measurement result
//!       │
//!       ▼
//! match result
//!       │
//!       ▼
//! dynamic classical/quantum control
//! ```
//!
//! No physical qubit, gate, topology, pulse, calibration or backend information
//! is stored here.
//!
//! ## External quantum formats
//!
//! OpenQASM, QIR, Quil, vendor formats and other external representations must
//! not become dependencies of this module.
//!
//! They must be imported into Zamani through their respective frontend/format
//! layers and subsequently represented using Zamani's native source or semantic
//! structures.
//!
//! ## Scalability
//!
//! There is deliberately no:
//!
//! ```text
//! MAX_MATCH_ARMS
//! MAX_MATCH_DEPTH
//! MAX_MATCHES
//! MAX_QUBITS
//! MAX_MACHINES
//! ```
//!
//! in this module.
//!
//! Resource limits used by a compiler service are supplied explicitly through
//! `MatchValidationPolicy`.
//!
//! Such limits are safety/resource policies, not language semantics.
//!
//! ## Security
//!
//! This module:
//!
//! - forbids unsafe Rust;
//! - performs no I/O;
//! - executes no source program;
//! - performs no unchecked indexing;
//! - uses no raw pointers;
//! - uses no global mutable state;
//! - does not recursively traverse the AST;
//! - does not dereference `NodeId`s;
//! - does not access hardware;
//! - does not access backend credentials;
//! - does not execute extensions.
//!
//! Malformed references are rejected locally where possible.
//!
//! ## No-reedit integration contract
//!
//! This module depends only on stable foundational AST contracts:
//!
//! - `Node`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - `NodeMetadata`;
//! - `Span`.
//!
//! Changes to:
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - QIR;
//! - routing;
//! - scheduling;
//! - QEC;
//! - resilience;
//! - calibration;
//! - hardware;
//! - runtime;
//! - backend providers
//!
//! must not require this file to be changed.
//!
//! This file should change only if the source-language structure or public AST
//! contract of a match statement changes.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;
use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::{AstNode, Node};
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Independent schema version for this source-level node contract.
///
/// This is intentionally independent from the Zamani language version,
/// compiler version and global AST serialization version.
pub const MATCH_STATEMENT_SCHEMA_VERSION: u16 = 1;

/// Stable source-level name for the native match-statement construct.
pub const MATCH_STATEMENT_KIND_NAME: &str = "zamani:match-statement";

/// Result type for local match-statement construction and validation.
pub type MatchStatementResult<T> = Result<T, MatchStatementError>;

/// Local structural errors for a match statement.
///
/// These errors intentionally do not include semantic errors such as
/// non-exhaustive patterns or incompatible pattern types.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MatchStatementError {
    /// The common node has a node kind other than `MatchStatement`.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// The scrutinee references an invalid/null node ID.
    InvalidScrutineeReference,

    /// The scrutinee references the enclosing match node.
    SelfReferentialScrutinee,

    /// A match arm contains an invalid/null node ID.
    InvalidArmReference {
        /// Source-order index of the invalid arm.
        arm_index: usize,
    },

    /// The match node is incorrectly reused as an arm node.
    SelfReferentialArm {
        /// Source-order index of the offending arm.
        arm_index: usize,
    },

    /// The same arm node occurs more than once.
    DuplicateArmReference {
        /// First source-order occurrence.
        first_index: usize,

        /// Second source-order occurrence.
        second_index: usize,

        /// Duplicated node identity.
        id: NodeId,
    },

    /// A caller-provided structural resource policy was exceeded.
    LimitExceeded {
        /// Name of the policy.
        limit: &'static str,

        /// Observed value.
        actual: usize,

        /// Allowed value.
        maximum: usize,
    },
}

impl fmt::Display for MatchStatementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "match statement has invalid AST node kind: {actual}"
                )
            }

            Self::InvalidScrutineeReference => {
                formatter.write_str(
                    "match statement has an invalid scrutinee node reference",
                )
            }

            Self::SelfReferentialScrutinee => {
                formatter.write_str(
                    "match statement cannot use itself as its scrutinee",
                )
            }

            Self::InvalidArmReference { arm_index } => {
                write!(
                    formatter,
                    "match statement arm {arm_index} has an invalid node reference"
                )
            }

            Self::SelfReferentialArm { arm_index } => {
                write!(
                    formatter,
                    "match statement arm {arm_index} cannot reference the enclosing match node"
                )
            }

            Self::DuplicateArmReference {
                first_index,
                second_index,
                id,
            } => {
                write!(
                    formatter,
                    "match statement arms {first_index} and {second_index} \
                     duplicate node {id:?}"
                )
            }

            Self::LimitExceeded {
                limit,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "match statement validation limit `{limit}` exceeded: \
                     {actual} > {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for MatchStatementError {}

/// Caller-owned structural validation policy.
///
/// All limits are optional. `None` means that this module imposes no limit for
/// that dimension.
///
/// The default policy is unrestricted.
///
/// These limits are deliberately not part of the language semantics.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MatchValidationPolicy {
    /// Optional maximum number of direct arm references.
    pub max_arms: Option<usize>,
}

impl MatchValidationPolicy {
    /// Creates an unrestricted policy.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self { max_arms: None }
    }

    fn check_arms(self, count: usize) -> MatchStatementResult<()> {
        if let Some(maximum) = self.max_arms {
            if count > maximum {
                return Err(MatchStatementError::LimitExceeded {
                    limit: "max_arms",
                    actual: count,
                    maximum,
                });
            }
        }

        Ok(())
    }
}

/// Source-level representation of a Zamani `match` statement.
///
/// The actual child nodes are owned by the enclosing AST graph and referenced
/// through `NodeId`.
///
/// ```text
/// MatchStatement
/// ├── Node
/// ├── scrutinee: NodeId
/// └── arms: Vec<NodeId>
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchStatement {
    /// Common AST identity, source span, node kind and metadata.
    node: Node,

    /// Expression being matched.
    scrutinee: NodeId,

    /// Ordered match-arm node references.
    arms: Vec<NodeId>,
}

impl MatchStatement {
    /// Creates a match statement with the canonical `MatchStatement` node kind.
    ///
    /// This constructor performs local structural validation using the
    /// unrestricted policy.
    pub fn new(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        scrutinee: NodeId,
        arms: Vec<NodeId>,
    ) -> MatchStatementResult<Self> {
        let statement = Self {
            node: Node::new(
                id,
                NodeKind::Core(CoreNodeKind::MatchStatement),
                span,
                metadata,
            ),
            scrutinee,
            arms,
        };

        statement.validate_structure(MatchValidationPolicy::unrestricted())?;

        Ok(statement)
    }

    /// Creates a match statement from an existing common AST node.
    ///
    /// Unlike [`Self::new`], this constructor does not rewrite the supplied
    /// node kind. It is intended for parser/migration code that already owns
    /// a canonical `Node`.
    ///
    /// Call [`Self::validate_structure`] and
    /// [`Self::validate_node_kind`] before treating the result as valid.
    #[must_use]
    pub fn from_node(
        node: Node,
        scrutinee: NodeId,
        arms: Vec<NodeId>,
    ) -> Self {
        Self {
            node,
            scrutinee,
            arms,
        }
    }

    /// Returns the common AST node.
    #[must_use]
    #[inline]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns this statement's stable AST identity.
    #[must_use]
    #[inline]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the complete source span of the match statement.
    #[must_use]
    #[inline]
    pub fn span(&self) -> Span {
        self.node.span()
    }

    /// Returns source-level metadata.
    #[must_use]
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns the expression being matched.
    #[must_use]
    #[inline]
    pub const fn scrutinee(&self) -> NodeId {
        self.scrutinee
    }

    /// Returns the ordered match-arm node references.
    #[must_use]
    #[inline]
    pub fn arms(&self) -> &[NodeId] {
        &self.arms
    }

    /// Returns a mutable view of the arm references.
    ///
    /// Structural validation should be performed after mutation.
    #[must_use]
    #[inline]
    pub fn arms_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.arms
    }

    /// Returns the number of match arms.
    #[must_use]
    #[inline]
    pub fn arm_count(&self) -> usize {
        self.arms.len()
    }

    /// Returns an arm reference by source-order index.
    #[must_use]
    #[inline]
    pub fn arm(&self, index: usize) -> Option<NodeId> {
        self.arms.get(index).copied()
    }

    /// Returns direct child node IDs in deterministic source order.
    ///
    /// The ordering is:
    ///
    /// 1. scrutinee;
    /// 2. first arm;
    /// 3. second arm;
    /// 4. remaining arms in source order.
    ///
    /// This method returns a caller-owned snapshot. Traversal implementations
    /// that already operate over the AST graph may use [`Self::children`] to
    /// avoid materializing an additional vector.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        let mut children = Vec::with_capacity(self.child_count());

        children.push(self.scrutinee);
        children.extend(self.arms.iter().copied());

        children
    }

    /// Returns the number of direct child references.
    ///
    /// This is O(1).
    #[must_use]
    #[inline]
    pub fn child_count(&self) -> usize {
        1 + self.arms.len()
    }

    /// Returns an iterator over direct children without allocating a vector.
    ///
    /// The iterator preserves source/structural order.
    pub fn children(
        &self,
    ) -> impl Iterator<Item = NodeId> + '_ {
        core::iter::once(self.scrutinee).chain(self.arms.iter().copied())
    }

    /// Validates the local structural invariants of the statement.
    ///
    /// This does not verify that referenced IDs exist in the enclosing AST
    /// graph. That is the responsibility of graph-level validation.
    ///
    /// It also does not perform semantic analysis.
    pub fn validate_structure(
        &self,
        policy: MatchValidationPolicy,
    ) -> MatchStatementResult<()> {
        if self.node.kind()
            != NodeKind::Core(CoreNodeKind::MatchStatement)
        {
            return Err(MatchStatementError::InvalidNodeKind {
                actual: self.node.kind(),
            });
        }

        policy.check_arms(self.arms.len())?;

        if self.scrutinee.get() == 0 {
            return Err(MatchStatementError::InvalidScrutineeReference);
        }

        if self.scrutinee == self.id() {
            return Err(MatchStatementError::SelfReferentialScrutinee);
        }

        let mut seen = HashSet::with_capacity(self.arms.len());

        for (arm_index, arm) in self.arms.iter().copied().enumerate() {
            if arm.get() == 0 {
                return Err(MatchStatementError::InvalidArmReference {
                    arm_index,
                });
            }

            if arm == self.id() {
                return Err(MatchStatementError::SelfReferentialArm {
                    arm_index,
                });
            }

            if !seen.insert(arm) {
                let first_index = self
                    .arms
                    .iter()
                    .take(arm_index)
                    .position(|candidate| *candidate == arm)
                    .unwrap_or(arm_index);

                return Err(MatchStatementError::DuplicateArmReference {
                    first_index,
                    second_index: arm_index,
                    id: arm,
                });
            }
        }

        Ok(())
    }

    /// Validates this node using the unrestricted local policy.
    pub fn validate(&self) -> MatchStatementResult<()> {
        self.validate_structure(MatchValidationPolicy::unrestricted())?;
        self.validate_node_kind()
    }

    /// Returns the canonical source-level node kind.
    #[must_use]
    #[inline]
    pub const fn expected_node_kind() -> NodeKind {
        NodeKind::Core(CoreNodeKind::MatchStatement)
    }

    /// Consumes this object and returns its common node and child references.
    ///
    /// This is useful for AST graph insertion/migration.
    #[must_use]
    pub fn into_parts(self) -> (Node, NodeId, Vec<NodeId>) {
        (self.node, self.scrutinee, self.arms)
    }
}

impl AstNode for MatchStatement {
    /// Returns the common AST node.
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn metadata() -> NodeMetadata {
        NodeMetadata::default()
    }

    #[test]
    fn canonical_kind_is_match_statement() {
        assert_eq!(
            MatchStatement::expected_node_kind(),
            NodeKind::Core(CoreNodeKind::MatchStatement)
        );
    }

    #[test]
    fn constructor_preserves_identity_and_children() {
        let statement = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(2),
            vec![node_id(3), node_id(4)],
        )
        .expect("valid match statement");

        assert_eq!(statement.id(), node_id(1));
        assert_eq!(statement.scrutinee(), node_id(2));
        assert_eq!(
            statement.arms(),
            &[node_id(3), node_id(4)]
        );
    }

    #[test]
    fn children_are_deterministic_and_source_ordered() {
        let statement = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(10),
            vec![
                node_id(20),
                node_id(30),
                node_id(40),
            ],
        )
        .expect("valid match statement");

        let children: Vec<NodeId> =
            statement.children().collect();

        assert_eq!(
            children,
            vec![
                node_id(10),
                node_id(20),
                node_id(30),
                node_id(40),
            ]
        );
    }

    #[test]
    fn child_count_is_constant_time_and_correct() {
        let statement = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(2),
            vec![node_id(3), node_id(4), node_id(5)],
        )
        .expect("valid match statement");

        assert_eq!(statement.child_count(), 4);
        assert_eq!(statement.arm_count(), 3);
    }

    #[test]
    fn rejects_null_scrutinee() {
        let result = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(0),
            vec![node_id(2)],
        );

        assert_eq!(
            result,
            Err(MatchStatementError::InvalidScrutineeReference)
        );
    }

    #[test]
    fn rejects_null_arm_reference() {
        let result = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(2),
            vec![node_id(0)],
        );

        assert_eq!(
            result,
            Err(MatchStatementError::InvalidArmReference {
                arm_index: 0,
            })
        );
    }

    #[test]
    fn rejects_self_referential_scrutinee() {
        let result = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(1),
            vec![node_id(2)],
        );

        assert_eq!(
            result,
            Err(MatchStatementError::SelfReferentialScrutinee)
        );
    }

    #[test]
    fn rejects_self_referential_arm() {
        let result = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(2),
            vec![node_id(1)],
        );

        assert_eq!(
            result,
            Err(MatchStatementError::SelfReferentialArm {
                arm_index: 0,
            })
        );
    }

    #[test]
    fn rejects_duplicate_arm_references() {
        let result = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(2),
            vec![node_id(3), node_id(3)],
        );

        assert_eq!(
            result,
            Err(MatchStatementError::DuplicateArmReference {
                first_index: 0,
                second_index: 1,
                id: node_id(3),
            })
        );
    }

    #[test]
    fn accepts_empty_arm_vector_structurally() {
        // The grammar currently requires at least one match case. Whether
        // zero arms are syntactically legal is therefore a parser/grammar
        // concern, not a reason to encode a hidden language invariant here.
        //
        // Keeping this constructor structurally permissive makes the node
        // reusable for programmatic AST construction and allows grammar-level
        // validation to remain in the parser/AST graph layer.
        let statement = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(2),
            Vec::new(),
        )
        .expect("empty arm collection is locally well formed");

        assert_eq!(statement.arm_count(), 0);
    }

    #[test]
    fn validation_policy_is_explicit() {
        let statement = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(2),
            vec![node_id(3), node_id(4)],
        )
        .expect("valid match statement");

        let policy = MatchValidationPolicy {
            max_arms: Some(1),
        };

        assert_eq!(
            statement.validate_structure(policy),
            Err(MatchStatementError::LimitExceeded {
                limit: "max_arms",
                actual: 2,
                maximum: 1,
            })
        );
    }

    #[test]
    fn unrestricted_policy_has_no_artificial_arm_limit() {
        let arms: Vec<NodeId> =
            (1_u64..=128).map(node_id).collect();

        let statement = MatchStatement::new(
            node_id(1_000),
            Span::default(),
            metadata(),
            node_id(10_000),
            arms,
        )
        .expect("large structurally valid match statement");

        assert_eq!(statement.arm_count(), 128);
    }

    #[test]
    fn ast_node_trait_exposes_common_node() {
        let statement = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(2),
            vec![node_id(3)],
        )
        .expect("valid match statement");

        assert_eq!(
            AstNode::node(&statement).kind(),
            NodeKind::Core(CoreNodeKind::MatchStatement)
        );
    }

    #[test]
    fn serde_round_trip_preserves_structure() {
        let statement = MatchStatement::new(
            node_id(1),
            Span::default(),
            metadata(),
            node_id(2),
            vec![node_id(3), node_id(4)],
        )
        .expect("valid match statement");

        let encoded =
            serde_json::to_string(&statement)
                .expect("serialize match statement");

        let decoded: MatchStatement =
            serde_json::from_str(&encoded)
                .expect("deserialize match statement");

        assert_eq!(decoded, statement);
    }
}