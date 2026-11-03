
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Symbolic Tensor Expressions
//!
//! Path: `src/ai/tensor/symbolic.rs`
//! Rust baseline: 1.97+
//! Edition: 2021
//!
//! This module describes symbolic tensor computation without materializing
//! tensor elements or selecting an execution target.
//!
//! ## Architectural ownership
//!
//! Canonical semantic dimensions:
//! `crate::frontend::semantic::domains::ai::DimensionExpr`
//!
//! Canonical semantic shapes:
//! `crate::frontend::semantic::domains::ai::TensorShape`
//!
//! Shape analysis:
//! `crate::ai::tensor::shapes`
//!
//! Tensor storage:
//! `crate::classical::collections::tensor`
//!
//! Universal tensor operations:
//! `crate::zuir::dialects::tensor`
//!
//! AI grammar:
//! `grammar/ai/tensors.g4`
//!
//! ## Non-goals
//!
//! This module does not:
//!
//! - define another tensor shape authority;
//! - allocate tensor elements;
//! - implement numerical tensor kernels;
//! - implement a second universal IR;
//! - perform hardware discovery;
//! - select devices;
//! - schedule operations;
//! - impose a universal tensor-rank limit;
//! - impose a universal graph-node limit.
//!
//! Symbolic expressions describe computational intent. Lowering and
//! execution are delegated to the canonical compiler and runtime.
//!
//! ## Resource behavior
//!
//! Graph nodes and edges use dynamically sized collections.
//!
//! Construction is bounded by available memory and the host's addressable
//! collection capacity. No language-level maximum is introduced.
//!
//! A symbolic expression can describe a computation whose concrete tensor
//! would be too large to materialize on the current machine.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::frontend::semantic::domains::ai::{
    AiId,
    DimensionExpr,
    TensorShape,
};

// ============================================================================
// SYMBOLIC EXPRESSION IDENTITIES
// ============================================================================

/// Identifies a symbolic tensor expression within one expression graph.
///
/// The identifier is local to its graph. It is not a globally stable model
/// identifier, semantic identifier, or serialized artifact identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SymbolicExprId(usize);

impl SymbolicExprId {
    /// Returns the zero-based graph-local identifier.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

// ============================================================================
// SYMBOLIC TENSOR OPERATIONS
// ============================================================================

/// Describes a symbolic tensor operation.
///
/// Operation semantics are intentionally open-ended through `Custom`.
///
/// The actual operation schema, type checking, effects, and lowering must
/// be supplied by the canonical AI operation registry and ZUIR dialect.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SymbolicTensorOp {
    /// An externally defined symbolic tensor input.
    Input {
        /// Stable semantic identity of the input.
        identity: AiId,
    },

    /// An externally defined constant tensor.
    ///
    /// This records the semantic identity of the constant. It does not
    /// contain or allocate the tensor's element storage.
    Constant {
        /// Stable semantic identity of the constant.
        identity: AiId,
    },

    /// Elementwise addition.
    Add,

    /// Elementwise subtraction.
    Subtract,

    /// Elementwise multiplication.
    Multiply,

    /// Elementwise division.
    Divide,

    /// Matrix multiplication.
    MatMul,

    /// Tensor contraction.
    Contract {
        /// Contracted axes on the left operand.
        left_axes: Vec<usize>,

        /// Contracted axes on the right operand.
        right_axes: Vec<usize>,
    },

    /// Transpose.
    Transpose {
        /// Output-axis to input-axis permutation.
        permutation: Vec<usize>,
    },

    /// Reshape.
    ///
    /// The target shape remains symbolic.
    Reshape {
        /// Requested output shape.
        shape: TensorShape,
    },

    /// Broadcast.
    Broadcast {
        /// Requested output shape.
        shape: TensorShape,
    },

    /// Reduction.
    Reduce {
        /// Reduced axes.
        axes: Vec<usize>,

        /// Whether reduced axes remain with extent one.
        keep_dimensions: bool,
    },

    /// An extensible operation registered outside this module.
    Custom {
        /// Canonical qualified operation name.
        operation: String,

        /// Opaque operation attributes.
        ///
        /// Attributes are not interpreted by this module.
        attributes: Vec<(String, String)>,
    },
}

// ============================================================================
// SYMBOLIC EXPRESSION NODES
// ============================================================================

/// A symbolic tensor expression node.
///
/// The node records computation intent and its inferred or declared shape.
/// It does not contain tensor elements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolicTensorNode {
    /// Graph-local node identity.
    pub id: SymbolicExprId,

    /// Symbolic operation.
    pub operation: SymbolicTensorOp,

    /// Input expression identities.
    pub inputs: Vec<SymbolicExprId>,

    /// Canonical semantic output shape.
    pub shape: TensorShape,
}

impl SymbolicTensorNode {
    /// Returns the node's graph-local identity.
    #[must_use]
    pub const fn id(&self) -> SymbolicExprId {
        self.id
    }

    /// Returns the node's symbolic output shape.
    #[must_use]
    pub fn shape(&self) -> &TensorShape {
        &self.shape
    }

    /// Returns the node's input identities.
    #[must_use]
    pub fn inputs(&self) -> &[SymbolicExprId] {
        &self.inputs
    }
}

// ============================================================================
// SYMBOLIC EXPRESSION GRAPH
// ============================================================================

/// A symbolic tensor expression graph.
///
/// The graph owns symbolic operation nodes, not tensor storage.
///
/// Nodes are stored in deterministic insertion order. Node identities are
/// assigned monotonically within the graph.
///
/// This representation supports shared subexpressions. It does not
/// automatically deduplicate expressions because doing so requires
/// operation-specific semantic equivalence rules.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SymbolicTensorGraph {
    nodes: Vec<SymbolicTensorNode>,
}

impl SymbolicTensorGraph {
    /// Creates an empty symbolic tensor graph.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }

    /// Returns the number of graph nodes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true when the graph contains no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns all nodes in deterministic insertion order.
    #[must_use]
    pub fn nodes(&self) -> &[SymbolicTensorNode] {
        &self.nodes
    }

    /// Returns a node by its graph-local identity.
    #[must_use]
    pub fn get(&self, id: SymbolicExprId) -> Option<&SymbolicTensorNode> {
        self.nodes.get(id.0)
    }

    /// Inserts a symbolic operation node.
    ///
    /// Every input must refer to an existing node.
    ///
    /// The shape is supplied by the caller or by a preceding semantic
    /// inference stage. This function does not invent shape information.
    pub fn insert(
        &mut self,
        operation: SymbolicTensorOp,
        inputs: Vec<SymbolicExprId>,
        shape: TensorShape,
    ) -> Result<SymbolicExprId, SymbolicTensorError> {
        for input in &inputs {
            if self.get(*input).is_none() {
                return Err(SymbolicTensorError::UnknownInput {
                    input: *input,
                });
            }
        }

        let id = SymbolicExprId(self.nodes.len());

        self.nodes.push(SymbolicTensorNode {
            id,
            operation,
            inputs,
            shape,
        });

        Ok(id)
    }

    /// Inserts an input node.
    pub fn input(
        &mut self,
        identity: AiId,
        shape: TensorShape,
    ) -> Result<SymbolicExprId, SymbolicTensorError> {
        self.insert(
            SymbolicTensorOp::Input { identity },
            Vec::new(),
            shape,
        )
    }

    /// Inserts a symbolic constant reference.
    pub fn constant(
        &mut self,
        identity: AiId,
        shape: TensorShape,
    ) -> Result<SymbolicExprId, SymbolicTensorError> {
        self.insert(
            SymbolicTensorOp::Constant { identity },
            Vec::new(),
            shape,
        )
    }

    /// Returns a deterministic topological traversal.
    ///
    /// The current graph representation only permits references to nodes
    /// inserted earlier. Therefore, insertion order is already topological.
    #[must_use]
    pub fn topological_order(&self) -> Vec<SymbolicExprId> {
        self.nodes.iter().map(SymbolicTensorNode::id).collect()
    }

    /// Returns the symbolic graph's output nodes.
    ///
    /// An output is a node that is not consumed by another node.
    #[must_use]
    pub fn outputs(&self) -> Vec<SymbolicExprId> {
        let mut consumed = BTreeSet::new();

        for node in &self.nodes {
            consumed.extend(node.inputs.iter().copied());
        }

        self.nodes
            .iter()
            .map(SymbolicTensorNode::id)
            .filter(|id| !consumed.contains(id))
            .collect()
    }

    /// Validates graph identity and dependency invariants.
    pub fn validate(&self) -> Result<(), SymbolicTensorError> {
        for (index, node) in self.nodes.iter().enumerate() {
            if node.id.0 != index {
                return Err(SymbolicTensorError::InvalidNodeIdentity {
                    expected: index,
                    actual: node.id.0,
                });
            }

            for input in &node.inputs {
                if input.0 >= index {
                    return Err(SymbolicTensorError::InvalidDependency {
                        node: node.id,
                        input: *input,
                    });
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// SYMBOLIC DIMENSION SUBSTITUTION
// ============================================================================

/// A mapping from canonical semantic dimension identities to expressions.
///
/// This mapping is an input to symbolic transformation. It does not become
/// another semantic symbol table.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DimensionSubstitution {
    substitutions: BTreeMap<AiId, DimensionExpr>,
}

impl DimensionSubstitution {
    /// Creates an empty substitution.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces a symbolic dimension substitution.
    pub fn insert(
        &mut self,
        identity: AiId,
        expression: DimensionExpr,
    ) -> Option<DimensionExpr> {
        self.substitutions.insert(identity, expression)
    }

    /// Returns a substitution, if present.
    #[must_use]
    pub fn get(&self, identity: &AiId) -> Option<&DimensionExpr> {
        self.substitutions.get(identity)
    }

    /// Returns all substitutions in deterministic identity order.
    #[must_use]
    pub fn entries(&self) -> &BTreeMap<AiId, DimensionExpr> {
        &self.substitutions
    }

    /// Applies substitutions recursively to a canonical dimension expression.
    ///
    /// A substitution cycle is reported rather than recursed indefinitely.
    pub fn apply(
        &self,
        expression: &DimensionExpr,
    ) -> Result<DimensionExpr, SymbolicTensorError> {
        self.apply_inner(expression, &mut BTreeSet::new())
    }

    fn apply_inner(
        &self,
        expression: &DimensionExpr,
        active: &mut BTreeSet<AiId>,
    ) -> Result<DimensionExpr, SymbolicTensorError> {
        match expression {
            DimensionExpr::Constant(value) => {
                Ok(DimensionExpr::Constant(*value))
            }

            DimensionExpr::Symbol(identity) => {
                let Some(replacement) = self.get(identity) else {
                    return Ok(expression.clone());
                };

                if !active.insert(identity.clone()) {
                    return Err(SymbolicTensorError::SubstitutionCycle {
                        identity: identity.clone(),
                    });
                }

                let result = self.apply_inner(replacement, active);

                active.remove(identity);

                result
            }

            DimensionExpr::Constraint(_) => {
                // Constraint identifiers are resolved by the canonical
                // semantic constraint system. They are not ordinary
                // symbolic variable substitutions.
                Ok(expression.clone())
            }

            DimensionExpr::Add(left, right) => {
                Ok(DimensionExpr::Add(
                    Box::new(self.apply_inner(left, active)?),
                    Box::new(self.apply_inner(right, active)?),
                ))
            }

            DimensionExpr::Multiply(left, right) => {
                Ok(DimensionExpr::Multiply(
                    Box::new(self.apply_inner(left, active)?),
                    Box::new(self.apply_inner(right, active)?),
                ))
            }

            DimensionExpr::Subtract(left, right) => {
                Ok(DimensionExpr::Subtract(
                    Box::new(self.apply_inner(left, active)?),
                    Box::new(self.apply_inner(right, active)?),
                ))
            }

            DimensionExpr::Divide(left, right) => {
                Ok(DimensionExpr::Divide(
                    Box::new(self.apply_inner(left, active)?),
                    Box::new(self.apply_inner(right, active)?),
                ))
            }
        }
    }

    /// Applies substitutions to a canonical tensor shape.
    pub fn apply_shape(
        &self,
        shape: &TensorShape,
    ) -> Result<TensorShape, SymbolicTensorError> {
        let dimensions = shape
            .dimensions
            .iter()
            .map(|dimension| self.apply(dimension))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TensorShape::new(dimensions))
    }
}

// ============================================================================
// SYMBOLIC TENSOR ERRORS
// ============================================================================

/// Errors produced by symbolic tensor graph construction and transformation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SymbolicTensorError {
    /// An input node does not exist.
    UnknownInput {
        input: SymbolicExprId,
    },

    /// A graph node identity does not match its position.
    InvalidNodeIdentity {
        expected: usize,
        actual: usize,
    },

    /// A node refers to an invalid dependency.
    InvalidDependency {
        node: SymbolicExprId,
        input: SymbolicExprId,
    },

    /// A dimension substitution contains a cycle.
    SubstitutionCycle {
        identity: AiId,
    },
}

impl fmt::Display for SymbolicTensorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownInput { input } => {
                write!(
                    f,
                    "symbolic tensor input {} does not exist",
                    input.index()
                )
            }

            Self::InvalidNodeIdentity { expected, actual } => {
                write!(
                    f,
                    "invalid symbolic tensor node identity: \
                     expected {expected}, found {actual}"
                )
            }

            Self::InvalidDependency { node, input } => {
                write!(
                    f,
                    "symbolic tensor node {} has invalid dependency {}",
                    node.index(),
                    input.index()
                )
            }

            Self::SubstitutionCycle { identity } => {
                write!(
                    f,
                    "cyclic symbolic dimension substitution for {identity:?}"
                )
            }
        }
    }
}

impl std::error::Error for SymbolicTensorError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(value: &str) -> AiId {
        AiId::new(value).expect("valid test identity")
    }

    fn shape(dimensions: Vec<DimensionExpr>) -> TensorShape {
        TensorShape::new(dimensions)
    }

    #[test]
    fn graph_starts_empty() {
        let graph = SymbolicTensorGraph::new();

        assert!(graph.is_empty());
        assert_eq!(graph.len(), 0);
        assert!(graph.outputs().is_empty());
    }

    #[test]
    fn creates_symbolic_input() {
        let mut graph = SymbolicTensorGraph::new();

        let input = graph
            .input(
                identity("input"),
                shape(vec![DimensionExpr::Constant(4)]),
            )
            .expect("input construction");

        assert_eq!(input.index(), 0);
        assert_eq!(graph.len(), 1);
        assert!(graph.get(input).is_some());
    }

    #[test]
    fn rejects_unknown_dependencies() {
        let mut graph = SymbolicTensorGraph::new();

        let result = graph.insert(
            SymbolicTensorOp::Add,
            vec![SymbolicExprId(100)],
            shape(vec![]),
        );

        assert!(matches!(
            result,
            Err(SymbolicTensorError::UnknownInput { .. })
        ));
    }

    #[test]
    fn graph_outputs_are_deterministic() {
        let mut graph = SymbolicTensorGraph::new();

        let first = graph
            .input(identity("first"), shape(vec![]))
            .expect("first input");

        let second = graph
            .input(identity("second"), shape(vec![]))
            .expect("second input");

        graph
            .insert(
                SymbolicTensorOp::Add,
                vec![first, second],
                shape(vec![]),
            )
            .expect("addition");

        assert_eq!(
            graph.outputs(),
            vec![SymbolicExprId(2)]
        );
    }

    #[test]
    fn graph_is_topologically_ordered() {
        let mut graph = SymbolicTensorGraph::new();

        let first = graph
            .input(identity("first"), shape(vec![]))
            .expect("first input");

        let second = graph
            .insert(
                SymbolicTensorOp::Add,
                vec![first],
                shape(vec![]),
            )
            .expect("second node");

        assert_eq!(
            graph.topological_order(),
            vec![first, second]
        );

        assert!(graph.validate().is_ok());
    }

    #[test]
    fn substitutes_symbolic_dimensions() {
        let symbol = identity("batch");

        let mut substitution = DimensionSubstitution::new();

        substitution.insert(
            symbol.clone(),
            DimensionExpr::Constant(32),
        );

        let original = shape(vec![
            DimensionExpr::Constant(8),
            DimensionExpr::Symbol(symbol),
        ]);

        let result = substitution
            .apply_shape(&original)
            .expect("shape substitution");

        assert_eq!(
            result,
            shape(vec![
                DimensionExpr::Constant(8),
                DimensionExpr::Constant(32),
            ])
        );
    }

    #[test]
    fn preserves_unresolved_constraint_identity() {
        let constraint = identity("constraint");

        let substitution = DimensionSubstitution::new();

        let expression = DimensionExpr::Constraint(
            constraint.clone(),
        );

        assert_eq!(
            substitution.apply(&expression),
            Ok(DimensionExpr::Constraint(constraint))
        );
    }

    #[test]
    fn detects_substitution_cycles() {
        let first = identity("first");
        let second = identity("second");

        let mut substitution = DimensionSubstitution::new();

        substitution.insert(
            first.clone(),
            DimensionExpr::Symbol(second.clone()),
        );

        substitution.insert(
            second.clone(),
            DimensionExpr::Symbol(first.clone()),
        );

        let result = substitution.apply(
            &DimensionExpr::Symbol(first),
        );

        assert!(matches!(
            result,
            Err(SymbolicTensorError::SubstitutionCycle { .. })
        ));
    }

    #[test]
    fn preserves_arbitrary_shape_rank() {
        let dimensions = (0..512)
            .map(|value| DimensionExpr::Constant(value))
            .collect();

        let tensor_shape = TensorShape::new(dimensions);

        assert_eq!(tensor_shape.rank(), 512);
    }

    #[test]
    fn symbolic_graph_does_not_materialize_tensor_elements() {
        let mut graph = SymbolicTensorGraph::new();

        let input = graph
            .input(
                identity("large_input"),
                shape(vec![
                    DimensionExpr::Constant(u128::MAX),
                    DimensionExpr::Symbol(identity("dynamic")),
                ]),
            )
            .expect("symbolic input");

        assert_eq!(graph.len(), 1);
        assert!(graph.get(input).is_some());
    }
}
