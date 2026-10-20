#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(unused_must_use)]

//! # HDLIR Expressions
//!
//! Path: `src/hdl/hdlir/values/expressions.rs`
//! Edition: Rust 2021.
//! Minimum Rust version: 1.97.
//! Safety: Safe Rust only.
//! Status: Proposed implementation; repository integration and compilation
//! are required before this module can be frozen.
//!
//! ## Purpose
//!
//! Represent target-independent HDL expressions in an arena-backed graph.
//! The representation supports constants, symbolic references, unary and
//! binary operations, conditional expressions, concatenation, extraction,
//! casts, and explicitly named extension operations.
//!
//! This module represents expression structure. It does not evaluate
//! expressions, infer source-language types, resolve names, or realize
//! expressions on physical hardware.
//!
//! ## Authority
//!
//! - `grammar/DESIGN.md`.
//! - `grammar/spec/hdl.md`.
//! - `grammar/hdl/hdl.g4` and its applicable delegate grammars.
//! - `grammar/Zamani.g4`.
//! - `grammar/expressions/` and its applicable expression grammars.
//! - `src/frontend/semantic/domains/hdl.rs`.
//! - `src/hdl/api.rs`.
//! - `src/hdl/hdlir/types/bit_width.rs`.
//! - `src/hdl/hdlir/types/logic.rs`.
//! - `src/hdl/hdlir/values/constants.rs`.
//! - `src/hdl/hdlir/HDLIR-SPEC.md`, once established.
//!
//! The grammar defines syntax. The canonical frontend semantic model
//! establishes source meaning. `BitWidth` remains authoritative for HDLIR
//! bit-width expressions, and `ConstantValue` remains authoritative for
//! concrete HDLIR constants.
//!
//! ## Ownership
//!
//! This module owns:
//!
//! - Expression node identities scoped to one expression graph.
//! - Expression node kinds and their local structural invariants.
//! - Checked graph construction and structural validation.
//! - An optional operational node budget.
//! - Deterministic traversal in graph insertion order.
//!
//! It does not own:
//!
//! - Source parsing, AST nodes, name resolution, or source-level semantics.
//! - A replacement for `BitWidth`, `ConstantValue`, or the HDLIR type system.
//! - General type inference or proof of operator type compatibility.
//! - Constant folding, expression evaluation, or optimization.
//! - Hardware signals, registers, clocks, memory state, or physical resources.
//! - Source spans or a competing source-location system.
//!
//! ## Graph invariant
//!
//! Every operand must refer to a node already present in the graph.
//! Consequently, each newly inserted node refers only to lower-indexed nodes.
//! This establishes an acyclic, topologically ordered expression DAG by
//! construction and permits shared subexpressions.
//!
//! A graph is local to its owning `ExprGraph`. An `ExprId` from another graph
//! is not interchangeable merely because its numeric index is equal.
//!
//! ## Scalability
//!
//! There is no universal hardware-width, expression-count, or operand-count
//! constant. The default budget permits as many nodes as the host index type
//! can represent, subject to available memory and allocation success.
//!
//! A caller can configure a smaller operational node budget. Exceeding it
//! returns an explicit error and never returns a partially constructed node.
//! This budget limits construction work; it does not change language
//! semantics or imply a universal hardware capacity.
//!
//! This implementation uses `usize` for in-memory node indices because Rust
//! collections are indexed by `usize`. Such indices are local representation
//! details, not persistent identities, physical hardware IDs, or a promise
//! that serialized indices are portable across artifacts.
//!
//! Expression graphs are traversed iteratively. No claim is made that finite
//! software can materialize literally infinite expressions.
//!
//! ## Integration
//!
//! - `values/mod.rs` should declare `pub mod expressions;` and deliberately
//!   re-export the public API.
//! - `values/constants.rs` supplies `ConstantValue`.
//! - `types/bit_width.rs` supplies symbolic `BitWidth` expressions.
//! - `types/logic.rs` remains the owner of logic type and signedness meaning.
//! - `lowering/from_semantic.rs` constructs graph nodes from actual validated
//!   semantic inputs, preserving source meaning and provenance.
//! - `verify/types.rs` must verify operand types, operator constraints, and
//!   width compatibility using the actual registered semantics.
//! - `analysis/widths.rs` may analyze width expressions but must not silently
//!   replace unresolved symbolic expressions with guessed concrete values.
//! - `transform/` may rewrite expressions only under explicit preservation
//!   contracts.
//! - Serialization must encode node kinds, operand order, symbolic widths,
//!   extension names, and attributes without depending on host memory layout.
//!
//! This file deliberately does not import future builders, verifiers,
//! dialect registries, or unverified frontend adapter APIs.
//!
//! ## Failure behavior
//!
//! Invalid references, invalid symbolic names, invalid extension names,
//! node-budget exhaustion, and failed capacity reservation return explicit
//! errors. The module does not silently truncate operands, wrap indices,
//! evaluate symbolic expressions, or discard unknown extensions.
//!
//! ## Freeze criteria
//!
//! - The module compiles against the actual sibling modules.
//! - `values/mod.rs` exports the agreed API.
//! - Construction, graph-local identity, structural validation, budget
//!   exhaustion, and extension preservation have focused tests.
//! - Semantic lowering and type verification consume the representation
//!   without introducing a competing expression or constant model.
//! - Formatting, focused tests, full repository tests, and Clippy pass.

use core::fmt;
use std::collections::BTreeSet;

use crate::hdl::hdlir::types::bit_width::BitWidth;
use crate::hdl::hdlir::values::constants::ConstantValue;

/// Identity of an expression node within one [`ExprGraph`].
///
/// The index is deliberately private so callers cannot manufacture an
/// identity without going through the graph API.
///
/// This identity is not globally unique and must not be serialized as a
/// persistent identity without an explicit artifact-level mapping.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExprId(usize);

impl ExprId {
    /// Return this node's insertion index.
    ///
    /// The index is meaningful only within the graph that created the ID.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

/// Unary expression operators.
///
/// Their exact typing, overflow, signedness, and four-state behavior are
/// determined by the applicable registered operation semantics and the
/// canonical HDL semantic contract. This enum does not evaluate them.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum UnaryOperator {
    /// Arithmetic negation.
    Negate,
    /// Logical negation.
    LogicalNot,
    /// Bitwise complement.
    BitwiseNot,
    /// Reduction AND.
    ReduceAnd,
    /// Reduction OR.
    ReduceOr,
    /// Reduction XOR.
    ReduceXor,
}

/// Binary expression operators.
///
/// These identify expression operations, not their evaluation rules.
/// Operator type constraints and exact behavior must be verified separately.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BinaryOperator {
    /// Addition.
    Add,
    /// Subtraction.
    Subtract,
    /// Multiplication.
    Multiply,
    /// Division.
    Divide,
    /// Remainder.
    Remainder,
    /// Left shift.
    ShiftLeft,
    /// Right shift.
    ShiftRight,
    /// Bitwise AND.
    BitwiseAnd,
    /// Bitwise OR.
    BitwiseOr,
    /// Bitwise XOR.
    BitwiseXor,
    /// Logical AND.
    LogicalAnd,
    /// Logical OR.
    LogicalOr,
    /// Equality comparison.
    Equal,
    /// Inequality comparison.
    NotEqual,
    /// Less-than comparison.
    LessThan,
    /// Less-than-or-equal comparison.
    LessThanOrEqual,
    /// Greater-than comparison.
    GreaterThan,
    /// Greater-than-or-equal comparison.
    GreaterThanOrEqual,
}

/// Signedness requested by an explicit expression cast.
///
/// This describes the requested interpretation, not a physical representation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CastSignedness {
    /// Interpret the result as unsigned.
    Unsigned,
    /// Interpret the result as signed.
    Signed,
}

/// A node in an HDLIR expression graph.
///
/// Operands refer to existing nodes in the same graph. All nodes are
/// immutable after insertion, so a graph can safely share a node among
/// multiple expressions.
///
/// `Extension` preserves explicitly named operations that this core module
/// does not interpret. Their semantics must be supplied by a registered
/// dialect or another authoritative extension contract.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ExprKind {
    /// A concrete HDLIR constant.
    Constant(ConstantValue),

    /// A symbolic reference whose name has not been resolved here.
    ///
    /// Name resolution belongs to the canonical semantic/lowering layer.
    Symbol(String),

    /// A unary operation.
    Unary {
        /// Operation being applied.
        operator: UnaryOperator,
        /// Operand node.
        operand: ExprId,
    },

    /// A binary operation.
    Binary {
        /// Operation being applied.
        operator: BinaryOperator,
        /// Left operand node.
        lhs: ExprId,
        /// Right operand node.
        rhs: ExprId,
    },

    /// A conditional expression.
    ///
    /// The condition and both alternatives remain explicit. This node does
    /// not assert that the condition is two-state or that both alternatives
    /// have compatible types.
    Conditional {
        /// Condition expression.
        condition: ExprId,
        /// Expression selected when the condition is true.
        then_value: ExprId,
        /// Expression selected when the condition is false.
        else_value: ExprId,
    },

    /// Concatenate operands in their declared order.
    ///
    /// The meaning of operand order is preserved. Width calculation and
    /// context-specific legality are handled by the relevant verifier.
    Concat {
        /// Ordered concatenation operands.
        operands: Vec<ExprId>,
    },

    /// Extract a bit range from an expression.
    ///
    /// The offset is itself an expression, allowing symbolic and computed
    /// offsets. The result width uses the canonical symbolic width model.
    Extract {
        /// Source expression.
        value: ExprId,
        /// Offset expression.
        offset: ExprId,
        /// Requested result width.
        width: BitWidth,
    },

    /// Explicitly request a cast to a bit width and signedness.
    ///
    /// State-model conversion, truncation, extension, and representability
    /// rules must be defined by the applicable semantic contract. This node
    /// does not silently perform or approve such conversions.
    Cast {
        /// Source expression.
        value: ExprId,
        /// Requested destination width.
        width: BitWidth,
        /// Requested signedness.
        signedness: CastSignedness,
    },

    /// An operation defined by an explicitly named extension.
    ///
    /// The extension name and operation name are retained verbatim. Attribute
    /// pairs are retained in caller-provided order. Canonical serialization
    /// may define a canonical ordering, but must not change their meaning.
    Extension {
        /// Dialect or extension namespace.
        dialect: String,
        /// Operation name within the namespace.
        operation: String,
        /// Ordered operands.
        operands: Vec<ExprId>,
        /// Opaque string attributes in insertion order.
        attributes: Vec<(String, String)>,
    },
}

/// One immutable expression node and its identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExprNode {
    id: ExprId,
    kind: ExprKind,
}

impl ExprNode {
    /// Return this node's graph-local identity.
    #[must_use]
    pub const fn id(&self) -> ExprId {
        self.id
    }

    /// Return the node's expression kind.
    #[must_use]
    pub const fn kind(&self) -> &ExprKind {
        &self.kind
    }
}

/// Operational limits for expression graph construction.
///
/// This is a resource-safety control, not a language-level expression-count
/// or hardware-capacity limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExprGraphBudget {
    max_nodes: usize,
}

impl ExprGraphBudget {
    /// Create a budget that permits up to `max_nodes` graph nodes.
    ///
    /// A zero budget is valid and permits an empty graph but no inserted node.
    #[must_use]
    pub const fn new(max_nodes: usize) -> Self {
        Self { max_nodes }
    }

    /// Return the configured maximum node count.
    #[must_use]
    pub const fn max_nodes(self) -> usize {
        self.max_nodes
    }
}

impl Default for ExprGraphBudget {
    /// Return a budget limited only by the representable host index range.
    ///
    /// Real construction remains constrained by allocation availability.
    fn default() -> Self {
        Self::new(usize::MAX)
    }
}

/// Errors returned by expression graph construction and validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExprError {
    /// An operand ID does not refer to an existing earlier node in this graph.
    InvalidOperand {
        /// The invalid operand ID.
        operand: ExprId,
        /// Number of nodes present when validation occurred.
        node_count: usize,
    },

    /// A name used for a symbolic reference is empty or whitespace-only.
    EmptySymbol,

    /// A dialect or extension operation name is empty or whitespace-only.
    InvalidExtensionName {
        /// Which name failed validation.
        field: &'static str,
    },

    /// The node budget would be exceeded by this insertion.
    NodeBudgetExceeded {
        /// Configured node budget.
        limit: usize,
        /// Number of nodes already present.
        existing: usize,
    },

    /// The host index space cannot represent another node.
    NodeIndexExhausted,

    /// The requested vector capacity could not be reserved.
    AllocationFailed,

    /// An expression node violates a graph invariant.
    InvalidGraph {
        /// Node whose invariant failed, when available.
        node: Option<ExprId>,
        /// Explanation suitable for a diagnostic.
        reason: &'static str,
    },

    /// The same expression node was visited more than once in a traversal
    /// that requires unique IDs.
    DuplicateNode(ExprId),
}

impl fmt::Display for ExprError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperand {
                operand,
                node_count,
            } => write!(
                formatter,
                "expression operand {} is invalid for a graph containing {} nodes",
                operand.index(),
                node_count
            ),
            Self::EmptySymbol => {
                formatter.write_str("expression symbol must not be empty")
            }
            Self::InvalidExtensionName { field } => {
                write!(formatter, "expression extension {field} must not be empty")
            }
            Self::NodeBudgetExceeded { limit, existing } => write!(
                formatter,
                "expression graph node budget of {limit} exceeded with {existing} existing nodes"
            ),
            Self::NodeIndexExhausted => {
                formatter.write_str("expression graph node index space exhausted")
            }
            Self::AllocationFailed => {
                formatter.write_str("unable to reserve expression graph storage")
            }
            Self::InvalidGraph { node, reason } => match node {
                Some(id) => write!(
                    formatter,
                    "invalid expression graph at node {}: {reason}",
                    id.index()
                ),
                None => write!(formatter, "invalid expression graph: {reason}"),
            },
            Self::DuplicateNode(id) => write!(
                formatter,
                "expression traversal encountered duplicate node ID {}",
                id.index()
            ),
        }
    }
}

impl std::error::Error for ExprError {}

/// An arena-backed, topologically ordered HDLIR expression graph.
///
/// Nodes are immutable once inserted. To construct a compound expression,
/// insert its operands first and then insert the compound node. The graph
/// therefore cannot acquire a cycle through the public insertion API.
///
/// A graph can contain multiple roots and shared subexpressions. The graph
/// itself does not assume that every node is reachable from one root.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExprGraph {
    nodes: Vec<ExprNode>,
    budget: ExprGraphBudget,
}

impl ExprGraph {
    /// Create an empty expression graph with the default operational budget.
    #[must_use]
    pub fn new() -> Self {
        Self::with_budget(ExprGraphBudget::default())
    }

    /// Create an empty graph with an explicit node budget.
    #[must_use]
    pub const fn with_budget(budget: ExprGraphBudget) -> Self {
        Self {
            nodes: Vec::new(),
            budget,
        }
    }

    /// Return the configured construction budget.
    #[must_use]
    pub const fn budget(&self) -> ExprGraphBudget {
        self.budget
    }

    /// Return the number of inserted nodes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Return whether the graph contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Return all nodes in topological insertion order.
    #[must_use]
    pub fn nodes(&self) -> &[ExprNode] {
        &self.nodes
    }

    /// Look up a node by graph-local ID.
    #[must_use]
    pub fn get(&self, id: ExprId) -> Option<&ExprNode> {
        self.nodes.get(id.0).filter(|node| node.id == id)
    }

    /// Insert a constant expression.
    ///
    /// # Errors
    ///
    /// Returns an error if the configured node budget is exhausted or
    /// storage cannot be reserved.
    pub fn constant(
        &mut self,
        value: ConstantValue,
    ) -> Result<ExprId, ExprError> {
        self.insert(ExprKind::Constant(value))
    }

    /// Insert a symbolic reference.
    ///
    /// This checks only that the name is non-empty. It does not resolve the
    /// name or assert that a declaration with that name exists.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty name, budget exhaustion, or allocation
    /// failure.
    pub fn symbol(
        &mut self,
        name: impl Into<String>,
    ) -> Result<ExprId, ExprError> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(ExprError::EmptySymbol);
        }

        self.insert(ExprKind::Symbol(name))
    }

    /// Insert an arbitrary expression kind after checking its invariants.
    ///
    /// All operands must refer to existing nodes in this graph. This
    /// topological construction rule prevents cycles and accidental use of
    /// IDs from a graph with a different insertion history.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid operands, malformed names, budget
    /// exhaustion, exhausted node indices, or allocation failure.
    pub fn insert(&mut self, kind: ExprKind) -> Result<ExprId, ExprError> {
        self.validate_kind(&kind)?;

        let existing = self.nodes.len();

        if existing >= self.budget.max_nodes {
            return Err(ExprError::NodeBudgetExceeded {
                limit: self.budget.max_nodes,
                existing,
            });
        }

        let next_index = existing;
        let next_index_plus_one = next_index
            .checked_add(1)
            .ok_or(ExprError::NodeIndexExhausted)?;

        // Ensure the next identity is representable before mutating the graph.
        let _ = next_index_plus_one;

        self.nodes
            .try_reserve(1)
            .map_err(|_| ExprError::AllocationFailed)?;

        let id = ExprId(next_index);
        self.nodes.push(ExprNode { id, kind });

        Ok(id)
    }

    /// Validate the graph's structural invariants.
    ///
    /// Because public insertion requires operands to precede their users,
    /// cycles and forward references cannot be introduced through this API.
    /// This method nevertheless checks the full graph so that the invariant
    /// remains explicit for future deserializers and internal integrations.
    ///
    /// This method does not verify operator types, source name resolution,
    /// width compatibility, or extension semantics.
    ///
    /// # Errors
    ///
    /// Returns an error if any identity, operand, symbol, or extension name
    /// violates the graph contract.
    pub fn validate(&self) -> Result<(), ExprError> {
        if self.nodes.len() > self.budget.max_nodes {
            return Err(ExprError::InvalidGraph {
                node: None,
                reason: "graph exceeds its configured node budget",
            });
        }

        for (index, node) in self.nodes.iter().enumerate() {
            if node.id.0 != index {
                return Err(ExprError::InvalidGraph {
                    node: Some(node.id),
                    reason: "node identity does not match its insertion index",
                });
            }

            self.validate_kind(&node.kind).map_err(|error| match error {
                ExprError::InvalidOperand { .. } => ExprError::InvalidGraph {
                    node: Some(node.id),
                    reason: "node contains an invalid or forward operand reference",
                },
                ExprError::EmptySymbol => ExprError::InvalidGraph {
                    node: Some(node.id),
                    reason: "node contains an empty symbolic name",
                },
                ExprError::InvalidExtensionName { .. } => ExprError::InvalidGraph {
                    node: Some(node.id),
                    reason: "node contains an invalid extension name",
                },
                other => other,
            })?;
        }

        Ok(())
    }

    /// Collect the transitive operands reachable from `root` in deterministic
    /// depth-first postorder.
    ///
    /// Shared subexpressions are returned once. Every returned ID belongs to
    /// this graph. The root is included as the final element.
    ///
    /// The traversal is iterative, so expression depth does not directly
    /// consume the Rust call stack.
    ///
    /// # Errors
    ///
    /// Returns an error if the root does not belong to this graph or if an
    /// invalid reference is encountered.
    pub fn postorder(&self, root: ExprId) -> Result<Vec<ExprId>, ExprError> {
        if self.get(root).is_none() {
            return Err(ExprError::InvalidOperand {
                operand: root,
                node_count: self.nodes.len(),
            });
        }

        let mut visited = BTreeSet::new();
        let mut result = Vec::new();
        let mut pending: Vec<(ExprId, bool)> = Vec::new();

        pending
            .try_reserve(1)
            .map_err(|_| ExprError::AllocationFailed)?;
        pending.push((root, false));

        while let Some((id, expanded)) = pending.pop() {
            if expanded {
                if visited.insert(id) {
                    result
                        .try_reserve(1)
                        .map_err(|_| ExprError::AllocationFailed)?;
                    result.push(id);
                }
                continue;
            }

            if visited.contains(&id) {
                continue;
            }

            let node = self.get(id).ok_or(ExprError::InvalidOperand {
                operand: id,
                node_count: self.nodes.len(),
            })?;

            pending
                .try_reserve(1)
                .map_err(|_| ExprError::AllocationFailed)?;
            pending.push((id, true));

            let operands = operands_of(&node.kind);

            // Push in reverse order so that the original operand order is
            // preserved by the depth-first traversal.
            for operand in operands.into_iter().rev() {
                if self.get(operand).is_none() {
                    return Err(ExprError::InvalidOperand {
                        operand,
                        node_count: self.nodes.len(),
                    });
                }

                if !visited.contains(&operand) {
                    pending
                        .try_reserve(1)
                        .map_err(|_| ExprError::AllocationFailed)?;
                    pending.push((operand, false));
                }
            }
        }

        Ok(result)
    }

    fn validate_kind(&self, kind: &ExprKind) -> Result<(), ExprError> {
        let mut check_operand = |operand: ExprId| {
            if self.get(operand).is_none() {
                Err(ExprError::InvalidOperand {
                    operand,
                    node_count: self.nodes.len(),
                })
            } else {
                Ok(())
            }
        };

        match kind {
            ExprKind::Constant(_) => Ok(()),

            ExprKind::Symbol(name) => {
                if name.trim().is_empty() {
                    Err(ExprError::EmptySymbol)
                } else {
                    Ok(())
                }
            }

            ExprKind::Unary { operand, .. } => check_operand(*operand),

            ExprKind::Binary { lhs, rhs, .. } => {
                check_operand(*lhs)?;
                check_operand(*rhs)
            }

            ExprKind::Conditional {
                condition,
                then_value,
                else_value,
            } => {
                check_operand(*condition)?;
                check_operand(*then_value)?;
                check_operand(*else_value)
            }

            ExprKind::Concat { operands } => {
                for operand in operands {
                    check_operand(*operand)?;
                }
                Ok(())
            }

            ExprKind::Extract {
                value,
                offset,
                ..
            } => {
                check_operand(*value)?;
                check_operand(*offset)
            }

            ExprKind::Cast { value, .. } => check_operand(*value),

            ExprKind::Extension {
                dialect,
                operation,
                operands,
                attributes,
            } => {
                if dialect.trim().is_empty() {
                    return Err(ExprError::InvalidExtensionName {
                        field: "dialect",
                    });
                }

                if operation.trim().is_empty() {
                    return Err(ExprError::InvalidExtensionName {
                        field: "operation",
                    });
                }

                for operand in operands {
                    check_operand(*operand)?;
                }

                for (key, _) in attributes {
                    if key.trim().is_empty() {
                        return Err(ExprError::InvalidExtensionName {
                            field: "attribute key",
                        });
                    }
                }

                Ok(())
            }
        }
    }
}

impl Default for ExprGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Return operands in their semantic declaration order.
fn operands_of(kind: &ExprKind) -> Vec<ExprId> {
    match kind {
        ExprKind::Constant(_) | ExprKind::Symbol(_) => Vec::new(),

        ExprKind::Unary { operand, .. } => vec![*operand],

        ExprKind::Binary { lhs, rhs, .. } => vec![*lhs, *rhs],

        ExprKind::Conditional {
            condition,
            then_value,
            else_value,
        } => vec![*condition, *then_value, *else_value],

        ExprKind::Concat { operands } => operands.clone(),

        ExprKind::Extract { value, offset, .. } => vec![*value, *offset],

        ExprKind::Cast { value, .. } => vec![*value],

        ExprKind::Extension { operands, .. } => operands.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::hdlir::types::bit_width::BitWidth;
    use crate::hdl::hdlir::types::logic::{
        LogicSignedness,
        LogicStateModel,
        LogicType,
    };
    use crate::hdl::values::four_state::LogicState;
    use crate::hdl::hdlir::values::constants::LogicConstant;

    fn one_bit(value: char) -> ConstantValue {
        let logic_type = LogicType::bit();
        let state = LogicState::from_char(value)
            .expect("test logic character should be valid");

        ConstantValue::logic(
            LogicConstant::new(logic_type, vec![state])
                .expect("one-bit test constant should be valid"),
        )
    }

    #[test]
    fn inserts_and_retrieves_a_constant() {
        let mut graph = ExprGraph::new();
        let id = graph
            .constant(one_bit('1'))
            .expect("constant insertion should succeed");

        assert_eq!(id.index(), 0);
        assert_eq!(graph.len(), 1);
        assert!(matches!(
            graph.get(id).map(ExprNode::kind),
            Some(ExprKind::Constant(_))
        ));
        assert!(graph.validate().is_ok());
    }

    #[test]
    fn builds_a_binary_expression_from_existing_operands() {
        let mut graph = ExprGraph::new();
        let lhs = graph
            .symbol("input_a")
            .expect("symbol insertion should succeed");
        let rhs = graph
            .symbol("input_b")
            .expect("symbol insertion should succeed");

        let root = graph
            .insert(ExprKind::Binary {
                operator: BinaryOperator::BitwiseAnd,
                lhs,
                rhs,
            })
            .expect("binary insertion should succeed");

        assert_eq!(root.index(), 2);
        assert_eq!(
            graph.postorder(root).expect("traversal should succeed"),
            vec![lhs, rhs, root]
        );
    }

    #[test]
    fn rejects_an_operand_not_present_in_the_graph() {
        let mut graph = ExprGraph::new();

        let result = graph.insert(ExprKind::Unary {
            operator: UnaryOperator::BitwiseNot,
            operand: ExprId(0),
        });

        assert!(matches!(
            result,
            Err(ExprError::InvalidOperand { .. })
        ));
        assert!(graph.is_empty());
    }

    #[test]
    fn rejects_empty_symbol_names() {
        let mut graph = ExprGraph::new();

        assert_eq!(graph.symbol("  "), Err(ExprError::EmptySymbol));
        assert!(graph.is_empty());
    }

    #[test]
    fn rejects_empty_extension_names() {
        let mut graph = ExprGraph::new();
        let operand = graph
            .symbol("signal")
            .expect("symbol insertion should succeed");

        let result = graph.insert(ExprKind::Extension {
            dialect: " ".to_owned(),
            operation: "custom".to_owned(),
            operands: vec![operand],
            attributes: Vec::new(),
        });

        assert!(matches!(
            result,
            Err(ExprError::InvalidExtensionName {
                field: "dialect"
            })
        ));
    }

    #[test]
    fn enforces_an_explicit_operational_node_budget() {
        let mut graph = ExprGraph::with_budget(ExprGraphBudget::new(1));

        graph
            .symbol("first")
            .expect("first node should fit in the budget");

        let result = graph.symbol("second");

        assert_eq!(
            result,
            Err(ExprError::NodeBudgetExceeded {
                limit: 1,
                existing: 1,
            })
        );
        assert_eq!(graph.len(), 1);
    }

    #[test]
    fn preserves_symbolic_widths_in_cast_nodes() {
        let mut graph = ExprGraph::new();
        let source = graph
            .symbol("data")
            .expect("symbol insertion should succeed");
        let width = BitWidth::symbol("DATA_WIDTH")
            .expect("symbolic width should be valid");

        let cast = graph
            .insert(ExprKind::Cast {
                value: source,
                width: width.clone(),
                signedness: CastSignedness::Unsigned,
            })
            .expect("cast insertion should succeed");

        assert!(matches!(
            graph.get(cast).map(ExprNode::kind),
            Some(ExprKind::Cast { width: stored, .. }) if stored == &width
        ));
    }

    #[test]
    fn preserves_extension_operands_and_attributes() {
        let mut graph = ExprGraph::new();
        let input = graph
            .symbol("input")
            .expect("symbol insertion should succeed");

        let expression = graph
            .insert(ExprKind::Extension {
                dialect: "vendor.example".to_owned(),
                operation: "special_logic".to_owned(),
                operands: vec![input],
                attributes: vec![("mode".to_owned(), "portable-intent".to_owned())],
            })
            .expect("extension insertion should succeed");

        assert!(matches!(
            graph.get(expression).map(ExprNode::kind),
            Some(ExprKind::Extension {
                dialect,
                operation,
                attributes,
                ..
            }) if dialect == "vendor.example"
                && operation == "special_logic"
                && attributes == &vec![("mode".to_owned(), "portable-intent".to_owned())]
        ));
    }

    #[test]
    fn supports_shared_subexpressions_without_duplicate_traversal() {
        let mut graph = ExprGraph::new();
        let input = graph
            .symbol("input")
            .expect("symbol insertion should succeed");

        let inverted = graph
            .insert(ExprKind::Unary {
                operator: UnaryOperator::BitwiseNot,
                operand: input,
            })
            .expect("unary insertion should succeed");

        let root = graph
            .insert(ExprKind::Binary {
                operator: BinaryOperator::BitwiseXor,
                lhs: inverted,
                rhs: inverted,
            })
            .expect("binary insertion should succeed");

        assert_eq!(
            graph.postorder(root).expect("traversal should succeed"),
            vec![input, inverted, root]
        );
    }

    #[test]
    fn rejects_a_zero_width_only_when_the_width_owner_rejects_it() {
        // Width legality remains owned by BitWidth. This expression module
        // retains the canonical type rather than implementing a second width
        // system.
        assert!(BitWidth::known(0).is_err());

        let _ = LogicSignedness::Unsigned;
        let _ = LogicStateModel::TwoState;
    }
}