//! # Zamani Native AST — Lambda Expression
//!
//! Source-level representation of lambda/closure expressions.
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
//! LambdaExpression
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ├── scope analysis
//!     ├── name resolution
//!     ├── type inference
//!     ├── capture analysis
//!     ├── effect analysis
//!     ├── capability analysis
//!     └── resource analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical lowering
//!     ├── quantum lowering
//!     ├── hybrid lowering
//!     └── future-domain lowering
//! ```
//!
//! ## Purpose
//!
//! This module owns the **source-level syntax representation** of a lambda,
//! closure, or anonymous-function expression.
//!
//! It represents programmer intent and source structure only.
//!
//! It does NOT determine:
//!
//! - the concrete function type;
//! - inferred parameter types;
//! - return type semantics;
//! - captured variables;
//! - capture modes;
//! - ownership;
//! - borrow semantics;
//! - effects;
//! - capabilities;
//! - resource requirements;
//! - execution strategy;
//! - CPU/GPU/QPU realization;
//! - quantum-resource allocation;
//! - physical qubit assignment;
//! - scheduling;
//! - routing;
//! - optimization;
//! - QEC;
//! - resilience;
//! - calibration;
//! - backend selection;
//! - runtime representation.
//!
//! Those concerns belong to semantic analysis and later compiler layers.
//!
//! ## POCO-REAF
//!
//! A lambda must remain independent of the eventual execution resource.
//!
//! The same source lambda can ultimately be:
//!
//! - executed classically;
//! - compiled to native code;
//! - transformed into accelerator code;
//! - used as a quantum-control computation;
//! - lowered into a distributed computation;
//! - specialized for a particular target;
//! - interpreted;
//! - transformed into another computational representation.
//!
//! None of those decisions belong in this AST node.
//!
//! This is required for:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF).
//!
//! ## Child representation
//!
//! The native expression architecture uses [`NodeId`] references for child
//! nodes rather than recursively embedding concrete expression objects.
//!
//! Consequently this node stores:
//!
//! ```text
//! LambdaExpression
//! ├── Node
//! ├── parameters: Vec<NodeId>
//! ├── body: NodeId
//! └── return_type: Option<NodeId>
//! ```
//!
//! The AST graph owns the actual parameter/body/type nodes.
//!
//! This design:
//!
//! - prevents recursive Rust ownership structures;
//! - permits iterative traversal;
//! - supports large ASTs;
//! - allows shared source graph infrastructure;
//! - keeps traversal independent from this node;
//! - avoids hidden recursion limits.
//!
//! ## Grammar integration
//!
//! The repository grammar currently supports lambda/closure forms equivalent
//! to:
//!
//! ```text
//! | [Params] | (BlockExpr | Expression)
//! ```
//!
//! and anonymous function syntax equivalent to:
//!
//! ```text
//! fn ( [Params] ) [-> TypeExpr] BlockExpr
//! ```
//!
//! This node deliberately does not preserve the exact delimiter spelling as a
//! semantic field. The complete source spelling remains recoverable through
//! the source span/source document infrastructure.
//!
//! If the parser needs to distinguish syntactic lambda forms for tooling,
//! that distinction should be represented by a source-level syntax-form enum
//! or metadata extension owned by the expression grammar, rather than by
//! introducing backend/domain semantics here.
//!
//! ## Parameter representation
//!
//! Parameters are referenced by [`NodeId`] rather than duplicated here.
//!
//! The referenced node may be the canonical AST parameter/declaration node
//! containing:
//!
//! - identifier;
//! - optional mutability;
//! - optional type expression;
//! - optional default expression where the grammar permits it;
//! - source metadata.
//!
//! This prevents lambda parameters from becoming a second competing parameter
//! representation.
//!
//! ## Return type
//!
//! `return_type` is an optional **source-level type expression reference**.
//!
//! It must not contain a resolved semantic type.
//!
//! For an omitted return type:
//!
//! ```text
//! return_type = None
//! ```
//!
//! means only that the source did not explicitly provide one.
//!
//! Semantic analysis may infer, validate, or otherwise determine the return
//! type later.
//!
//! ## Body representation
//!
//! `body` is a required [`NodeId`] referring to the source-level body
//! expression or block expression.
//!
//! The body may therefore represent:
//!
//! - a simple expression;
//! - a block expression;
//! - a future source-level expression construct.
//!
//! This file does not define a separate `LambdaBody` enum because doing so
//! would duplicate the canonical expression representation.
//!
//! ## Captures
//!
//! Captured variables are intentionally **not stored** in this node.
//!
//! Capture analysis is semantic information.
//!
//! For example:
//!
//! ```text
//! |x| x + external_value
//! ```
//!
//! does not cause this AST to contain a field such as:
//!
//! ```text
//! captures: ...
//! ```
//!
//! The semantic analyzer determines that `external_value` is captured and
//! records the result in a semantic side table/model keyed by this node's
//! [`NodeId`].
//!
//! This separation is essential because capture mode may depend on:
//!
//! - type information;
//! - ownership;
//! - lifetime;
//! - mutability;
//! - closure conversion;
//! - target semantics.
//!
//! ## Quantum neutrality
//!
//! This node contains no quantum-specific fields.
//!
//! A lambda may nevertheless be used to express quantum algorithms or hybrid
//! classical/quantum computation.
//!
//! For example, a source-level lambda may eventually:
//!
//! - construct a quantum operation;
//! - control a quantum computation;
//! - calculate a parameter;
//! - transform measurement results;
//! - express a higher-order quantum algorithm;
//! - participate in generic resource manipulation.
//!
//! Those meanings are resolved later.
//!
//! No fields such as:
//!
//! ```text
//! qubits
//! quantum_parameters
//! quantum_capture
//! physical_resources
//! backend
//! ```
//!
//! are permitted here.
//!
//! ## Dependency policy
//!
//! This module may depend on:
//!
//! - `Node`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - `NodeMetadata`;
//! - `Span`;
//! - standard-library types;
//! - Serde.
//!
//! It must NOT depend on:
//!
//! - semantic analysis;
//! - symbol tables;
//! - type checker;
//! - borrow checker;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - QIR;
//! - LLVM;
//! - MLIR;
//! - OpenQASM AST;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC;
//! - resilience;
//! - calibration;
//! - runtime;
//! - backend providers.
//!
//! ## Scalability
//!
//! No language-level maximum exists for:
//!
//! - number of lambda expressions;
//! - number of parameters;
//! - parameter name size;
//! - nesting depth;
//! - body size;
//! - type-expression size.
//!
//! `Vec<NodeId>` is used for parameters because lambda arity is not a fixed
//! machine property.
//!
//! Any protection against hostile source input belongs to configurable
//! compiler/parser/validation resource policies.
//!
//! There must be no hidden:
//!
//! ```text
//! MAX_LAMBDA_PARAMETERS
//! MAX_CLOSURE_DEPTH
//! MAX_CAPTURE_COUNT
//! MAX_QUANTUM_PARAMETERS
//! ```
//!
//! constants in this file.
//!
//! ## Determinism
//!
//! Parameter order is source order.
//!
//! AST equality and serialization depend only on the stored logical fields.
//!
//! No:
//!
//! - memory address;
//! - pointer;
//! - timestamp;
//! - thread ID;
//! - random value;
//! - backend state
//!
//! is stored.
//!
//! ## Traversal
//!
//! Direct children are returned in deterministic source/structural order:
//!
//! 1. parameters, in source order;
//! 2. explicit return type, when present;
//! 3. body.
//!
//! The traversal method is non-recursive.
//!
//! The complete AST walker remains responsible for recursively visiting the
//! referenced nodes.
//!
//! ## Validation boundary
//!
//! Local validation verifies only structural invariants:
//!
//! - correct node kind;
//! - valid node identity;
//! - non-empty parameter list entries where applicable;
//! - valid child references;
//! - no duplicate parameter node IDs;
//! - required body reference;
//! - return-type reference consistency.
//!
//! It does NOT validate:
//!
//! - whether parameters are declared correctly;
//! - whether names are unique in the enclosing scope;
//! - whether captures are valid;
//! - whether the lambda is type-correct;
//! - whether the return type matches the body;
//! - whether a target supports the resulting computation.
//!
//! Those checks belong downstream.
//!
//! ## Serialization
//!
//! Serde serialization is deterministic with respect to the logical structure.
//!
//! AST-wide serialization versioning belongs to the AST serialization layer.
//! This file does not introduce a competing serialization protocol.
//!
//! ## Security
//!
//! This file:
//!
//! - forbids unsafe code;
//! - performs no I/O;
//! - executes no source code;
//! - performs no unchecked indexing;
//! - does not dereference raw pointers;
//! - does not recursively traverse the AST;
//! - does not perform target-dependent allocation.
//!
//! Constructors reject structurally invalid required references instead of
//! relying on panic-based validation.
//!
//! ## Integration
//!
//! ### Parser
//!
//! The parser:
//!
//! 1. parses parameters;
//! 2. obtains their AST `NodeId`s;
//! 3. optionally parses an explicit return type;
//! 4. parses the body expression/block;
//! 5. allocates the lambda's `NodeId`;
//! 6. computes the complete source span;
//! 7. constructs this node.
//!
//! The parser must not perform capture analysis or type inference.
//!
//! ### Structural validation
//!
//! The AST validation layer validates this node locally and then resolves
//! its child `NodeId`s through the AST graph.
//!
//! ### Semantic analysis
//!
//! Semantic analysis uses this node to determine:
//!
//! - lexical scope;
//! - parameter bindings;
//! - capture set;
//! - capture modes;
//! - parameter types;
//! - return type;
//! - callable type;
//! - effects;
//! - capabilities;
//! - resource semantics.
//!
//! Those results are side-table/semantic-model data keyed by `NodeId`.
//!
//! ### ZUIR
//!
//! The semantic/lowering layer converts the resolved lambda into the
//! appropriate ZUIR callable/closure/function representation.
//!
//! This file must never import ZUIR.
//!
//! ### Quantum
//!
//! Quantum-specific semantics are obtained downstream from the referenced
//! expressions, resources, operations, effects, capabilities, or extensions.
//!
//! The lambda node itself remains domain-neutral.
//!
//! ### Visitors
//!
//! Generic visitors should visit:
//!
//! ```text
//! LambdaExpression
//!     parameters[0..N]
//!     return_type (if present)
//!     body
//! ```
//!
//! in that deterministic order.
//!
//! ### Migration
//!
//! Legacy representations such as:
//!
//! ```text
//! Lambda(Vec<...>, Box<Expression>)
//! Closure(...)
//! AnonymousFunction(...)
//! ```
//!
//! must be mapped into this canonical representation rather than duplicated.
//!
//! ## No-re-edit guarantee
//!
//! Once this public contract is established, adding:
//!
//! - a new quantum backend;
//! - a new quantum technology;
//! - a new computational domain;
//! - a new optimizer;
//! - a new scheduler;
//! - a new ZUIR lowering;
//! - a new hardware target
//!
//! must not require modifying this file.
//!
//! Changes are required only if the **Zamani source-language contract for
//! lambdas itself** changes.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::frontend::ast::node::{
    metadata::NodeMetadata,
    node::Node,
    node_id::NodeId,
    node_kind::{CoreNodeKind, NodeKind},
};
use crate::frontend::ast::source::Span;

/// Stable logical schema identifier for this node contract.
///
/// This is deliberately separate from the overall AST serialization schema
/// and from the Zamani language version.
pub const LAMBDA_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level kind identifier.
pub const LAMBDA_EXPRESSION_KIND_NAME: &str = "zamani:lambda-expression";

/// Errors that can be produced by local lambda construction/validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LambdaExpressionError {
    /// The lambda node was constructed with the wrong AST node kind.
    InvalidNodeKind {
        /// Actual node classification.
        actual: NodeKind,
    },

    /// The lambda does not contain a valid node identity.
    InvalidNodeId,

    /// A required body reference was not supplied.
    MissingBody,

    /// A parameter reference is invalid.
    InvalidParameterReference {
        /// Position of the invalid parameter.
        index: usize,

        /// Invalid node ID.
        id: NodeId,
    },

    /// The same parameter node was referenced more than once.
    DuplicateParameter {
        /// Position of the repeated parameter.
        index: usize,

        /// Repeated node ID.
        id: NodeId,
    },

    /// The explicit return-type reference is invalid.
    InvalidReturnTypeReference {
        /// Invalid node ID.
        id: NodeId,
    },

    /// The body reference is invalid.
    InvalidBodyReference {
        /// Invalid node ID.
        id: NodeId,
    },

    /// A caller supplied a collection larger than its configured safety
    /// policy permits.
    LimitExceeded {
        /// Name of the caller-defined limit.
        limit: &'static str,

        /// Observed value.
        actual: usize,

        /// Maximum allowed by the caller's policy.
        maximum: usize,
    },
}

impl fmt::Display for LambdaExpressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "lambda expression has invalid node kind: {actual}"
                )
            }

            Self::InvalidNodeId => {
                formatter.write_str("lambda expression has invalid node id")
            }

            Self::MissingBody => {
                formatter.write_str("lambda expression requires a body")
            }

            Self::InvalidParameterReference { index, id } => {
                write!(
                    formatter,
                    "lambda parameter at index {index} has invalid node reference {id:?}"
                )
            }

            Self::DuplicateParameter { index, id } => {
                write!(
                    formatter,
                    "lambda parameter at index {index} duplicates node reference {id:?}"
                )
            }

            Self::InvalidReturnTypeReference { id } => {
                write!(
                    formatter,
                    "lambda return type has invalid node reference {id:?}"
                )
            }

            Self::InvalidBodyReference { id } => {
                write!(
                    formatter,
                    "lambda body has invalid node reference {id:?}"
                )
            }

            Self::LimitExceeded {
                limit,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "lambda validation limit `{limit}` exceeded: {actual} > {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for LambdaExpressionError {}

/// Configurable local validation policy.
///
/// These limits are **compiler resource-safety policies**, not language
/// semantics and not machine-size restrictions.
///
/// `None` means that this policy imposes no limit for that property.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LambdaValidationPolicy {
    /// Optional maximum number of parameters accepted by this validation
    /// invocation.
    pub max_parameters: Option<usize>,
}

impl LambdaValidationPolicy {
    /// Create a policy without resource limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_parameters: None,
        }
    }

    /// Create a policy with a caller-defined parameter limit.
    #[must_use]
    pub const fn with_max_parameters(max_parameters: usize) -> Self {
        Self {
            max_parameters: Some(max_parameters),
        }
    }
}

/// Source-level lambda/closure/anonymous-function expression.
///
/// The node contains only source structure and references to other AST nodes.
///
/// It deliberately does not contain resolved semantic information.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LambdaExpression {
    /// Common source-level AST identity, classification, span, and metadata.
    node: Node,

    /// Parameter/declaration node references in source order.
    parameters: Vec<NodeId>,

    /// Optional explicit source-level return type.
    ///
    /// This is a reference to a type-expression AST node, not a resolved
    /// semantic type.
    return_type: Option<NodeId>,

    /// Body expression/block node reference.
    body: NodeId,
}

impl LambdaExpression {
    /// Construct a lambda expression.
    ///
    /// The constructor performs only intrinsic structural validation.
    ///
    /// It does not:
    ///
    /// - resolve names;
    /// - infer types;
    /// - analyze captures;
    /// - resolve effects;
    /// - resolve capabilities;
    /// - allocate resources;
    /// - select hardware.
    pub fn try_new(
        id: NodeId,
        span: Span,
        parameters: Vec<NodeId>,
        return_type: Option<NodeId>,
        body: NodeId,
    ) -> Result<Self, LambdaExpressionError> {
        Self::try_new_with_metadata(
            id,
            span,
            NodeMetadata::default(),
            parameters,
            return_type,
            body,
        )
    }

    /// Construct a lambda expression with source metadata.
    pub fn try_new_with_metadata(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        parameters: Vec<NodeId>,
        return_type: Option<NodeId>,
        body: NodeId,
    ) -> Result<Self, LambdaExpressionError> {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::LambdaExpression),
            span,
            metadata,
        );

        Self::from_node(node, parameters, return_type, body)
    }

    /// Construct a lambda from an existing AST node.
    ///
    /// This constructor does not rewrite the supplied node's classification.
    /// Structural validation therefore remains responsible for detecting a
    /// mismatched node kind.
    #[must_use]
    pub fn from_node(
        node: Node,
        parameters: Vec<NodeId>,
        return_type: Option<NodeId>,
        body: NodeId,
    ) -> Result<Self, LambdaExpressionError> {
        let expression = Self {
            node,
            parameters,
            return_type,
            body,
        };

        expression.validate_structure()?;

        Ok(expression)
    }

    /// Return the common AST node header.
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Return mutable access to the common AST node header.
    ///
    /// Only source-level node information should be changed through this
    /// interface. Semantic information must remain outside the AST.
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Return this node's stable AST identity.
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Return the complete source span of the lambda.
    #[must_use]
    pub fn span(&self) -> Span {
        self.node.span()
    }

    /// Return the node metadata, if present.
    #[must_use]
    pub fn metadata(&self) -> Option<&NodeMetadata> {
        self.node.metadata()
    }

    /// Attach source-level metadata.
    pub fn set_metadata(&mut self, metadata: NodeMetadata) {
        self.node.set_metadata(metadata);
    }

    /// Remove and return node metadata.
    pub fn take_metadata(&mut self) -> Option<NodeMetadata> {
        self.node.take_metadata()
    }

    /// Return the source-level AST classification.
    #[must_use]
    pub const fn node_kind(&self) -> NodeKind {
        NodeKind::core(CoreNodeKind::LambdaExpression)
    }

    /// Return all parameter node references in source order.
    #[must_use]
    pub fn parameters(&self) -> &[NodeId] {
        &self.parameters
    }

    /// Return the number of parameters.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }

    /// Return a parameter node reference by source-order position.
    #[must_use]
    pub fn parameter(&self, index: usize) -> Option<NodeId> {
        self.parameters.get(index).copied()
    }

    /// Return the optional explicit source-level return-type node reference.
    #[must_use]
    pub const fn return_type(&self) -> Option<NodeId> {
        self.return_type
    }

    /// Return whether the source explicitly supplied a return type.
    #[must_use]
    pub const fn has_explicit_return_type(&self) -> bool {
        self.return_type.is_some()
    }

    /// Return the body node reference.
    #[must_use]
    pub const fn body(&self) -> NodeId {
        self.body
    }

    /// Replace the complete parameter reference sequence.
    ///
    /// The new sequence is validated before it replaces the existing one.
    pub fn set_parameters(
        &mut self,
        parameters: Vec<NodeId>,
    ) -> Result<(), LambdaExpressionError> {
        Self::validate_parameters(&parameters)?;
        self.parameters = parameters;
        Ok(())
    }

    /// Replace the optional explicit return-type reference.
    pub fn set_return_type(
        &mut self,
        return_type: Option<NodeId>,
    ) -> Result<(), LambdaExpressionError> {
        if let Some(id) = return_type {
            if id.is_invalid() {
                return Err(LambdaExpressionError::InvalidReturnTypeReference {
                    id,
                });
            }
        }

        self.return_type = return_type;
        Ok(())
    }

    /// Replace the body reference.
    pub fn set_body(&mut self, body: NodeId) -> Result<(), LambdaExpressionError> {
        if body.is_invalid() {
            return Err(LambdaExpressionError::InvalidBodyReference {
                id: body,
            });
        }

        self.body = body;
        Ok(())
    }

    /// Append one parameter reference.
    ///
    /// The caller remains responsible for ensuring that the referenced node
    /// is a valid parameter/declaration node in the AST graph.
    pub fn push_parameter(
        &mut self,
        parameter: NodeId,
    ) -> Result<(), LambdaExpressionError> {
        if parameter.is_invalid() {
            return Err(LambdaExpressionError::InvalidParameterReference {
                index: self.parameters.len(),
                id: parameter,
            });
        }

        if self.parameters.contains(&parameter) {
            return Err(LambdaExpressionError::DuplicateParameter {
                index: self.parameters.len(),
                id: parameter,
            });
        }

        self.parameters.push(parameter);
        Ok(())
    }

    /// Return all direct child node references in deterministic traversal order.
    ///
    /// Order:
    ///
    /// 1. parameters;
    /// 2. explicit return type;
    /// 3. body.
    ///
    /// The returned vector is newly allocated so callers may freely modify it
    /// without modifying the AST node.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        let additional = usize::from(self.return_type.is_some());

        let mut children = Vec::with_capacity(self.parameters.len() + additional + 1);

        children.extend(self.parameters.iter().copied());

        if let Some(return_type) = self.return_type {
            children.push(return_type);
        }

        children.push(self.body);

        children
    }

    /// Return the number of direct AST children.
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.parameters.len()
            + usize::from(self.return_type.is_some())
            + 1
    }

    /// Returns whether this lambda has no parameters.
    #[must_use]
    pub fn is_parameterless(&self) -> bool {
        self.parameters.is_empty()
    }

    /// Returns whether this lambda has an explicit return type.
    #[must_use]
    pub const fn is_explicitly_typed(&self) -> bool {
        self.return_type.is_some()
    }

    /// Return the complete structural parts of the lambda.
    ///
    /// The returned parameter vector remains in source order.
    #[must_use]
    pub fn parts(&self) -> (&[NodeId], Option<NodeId>, NodeId) {
        (&self.parameters, self.return_type, self.body)
    }

    /// Consume the node and return its parameter/body references.
    #[must_use]
    pub fn into_parts(self) -> (Node, Vec<NodeId>, Option<NodeId>, NodeId) {
        (self.node, self.parameters, self.return_type, self.body)
    }

    /// Consume the node and return its parameter references.
    #[must_use]
    pub fn into_parameters(self) -> Vec<NodeId> {
        self.parameters
    }

    /// Consume the node and return the body reference.
    #[must_use]
    pub const fn into_body(self) -> NodeId {
        self.body
    }

    /// Consume the node and return the optional return-type reference.
    #[must_use]
    pub const fn into_return_type(self) -> Option<NodeId> {
        self.return_type
    }

    /// Validate intrinsic structural invariants without applying a resource
    /// limit.
    pub fn validate_structure(&self) -> Result<(), LambdaExpressionError> {
        self.validate_structure_with_policy(LambdaValidationPolicy::unlimited())
    }

    /// Validate intrinsic structural invariants under a caller-supplied policy.
    ///
    /// This method does not inspect the referenced nodes because that requires
    /// access to the owning AST graph. Graph-level validation belongs to the
    /// aggregate AST validation subsystem.
    pub fn validate_structure_with_policy(
        &self,
        policy: LambdaValidationPolicy,
    ) -> Result<(), LambdaExpressionError> {
        if self.id().is_invalid() {
            return Err(LambdaExpressionError::InvalidNodeId);
        }

        if self.node_kind() != NodeKind::core(CoreNodeKind::LambdaExpression) {
            return Err(LambdaExpressionError::InvalidNodeKind {
                actual: self.node_kind(),
            });
        }

        if let Some(maximum) = policy.max_parameters {
            if self.parameters.len() > maximum {
                return Err(LambdaExpressionError::LimitExceeded {
                    limit: "lambda_parameters",
                    actual: self.parameters.len(),
                    maximum,
                });
            }
        }

        Self::validate_parameters(&self.parameters)?;

        if let Some(return_type) = self.return_type {
            if return_type.is_invalid() {
                return Err(
                    LambdaExpressionError::InvalidReturnTypeReference {
                        id: return_type,
                    },
                );
            }
        }

        if self.body.is_invalid() {
            return Err(LambdaExpressionError::InvalidBodyReference {
                id: self.body,
            });
        }

        Ok(())
    }

    /// Validate parameter references.
    ///
    /// This verifies only reference-level structure:
    ///
    /// - IDs must be valid;
    /// - the same parameter node cannot appear twice.
    ///
    /// It does not verify that the referenced node is actually a parameter.
    /// That requires access to the owning AST graph.
    fn validate_parameters(
        parameters: &[NodeId],
    ) -> Result<(), LambdaExpressionError> {
        for (index, parameter) in parameters.iter().copied().enumerate() {
            if parameter.is_invalid() {
                return Err(LambdaExpressionError::InvalidParameterReference {
                    index,
                    id: parameter,
                });
            }

            if parameters[..index].contains(&parameter) {
                return Err(LambdaExpressionError::DuplicateParameter {
                    index,
                    id: parameter,
                });
            }
        }

        Ok(())
    }

    /// Return a compact source-level description useful for diagnostics.
    ///
    /// This does not include semantic type information or backend details.
    #[must_use]
    pub fn source_summary(&self) -> LambdaSourceSummary {
        LambdaSourceSummary {
            parameter_count: self.parameters.len(),
            has_explicit_return_type: self.return_type.is_some(),
            body: self.body,
        }
    }
}

/// Small deterministic source-level summary of a lambda.
///
/// This type intentionally contains no semantic information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LambdaSourceSummary {
    /// Number of syntactic parameters.
    pub parameter_count: usize,

    /// Whether the source supplied an explicit return type.
    pub has_explicit_return_type: bool,

    /// Body node reference.
    pub body: NodeId,
}

impl fmt::Display for LambdaExpression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("|")?;

        for (index, parameter) in self.parameters.iter().enumerate() {
            if index != 0 {
                formatter.write_str(", ")?;
            }

            write!(formatter, "{parameter:?}")?;
        }

        formatter.write_str("|")?;

        if let Some(return_type) = self.return_type {
            write!(formatter, " -> {return_type:?}")?;
        }

        write!(formatter, " {:?}", self.body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::metadata::NodeMetadata;
    use crate::frontend::ast::node::node_id::NodeId;
    use crate::frontend::ast::source::source_id::SourceId;

    fn span() -> Span {
        Span::new(SourceId::new(1), 0, 10)
    }

    fn id(value: u64) -> NodeId {
        NodeId::new(value)
    }

    #[test]
    fn constructs_parameterless_lambda() {
        let expression =
            LambdaExpression::try_new(id(1), span(), Vec::new(), None, id(2))
                .expect("valid lambda");

        assert_eq!(expression.id(), id(1));
        assert_eq!(expression.parameter_count(), 0);
        assert!(expression.is_parameterless());
        assert!(!expression.has_explicit_return_type());
        assert_eq!(expression.body(), id(2));
        assert_eq!(
            expression.node_kind(),
            NodeKind::core(CoreNodeKind::LambdaExpression)
        );
    }

    #[test]
    fn constructs_lambda_with_parameters() {
        let expression = LambdaExpression::try_new(
            id(1),
            span(),
            vec![id(2), id(3), id(4)],
            None,
            id(5),
        )
        .expect("valid lambda");

        assert_eq!(expression.parameter_count(), 3);
        assert_eq!(expression.parameters(), &[id(2), id(3), id(4)]);
        assert_eq!(expression.parameter(0), Some(id(2)));
        assert_eq!(expression.parameter(2), Some(id(4)));
        assert_eq!(expression.parameter(3), None);
    }

    #[test]
    fn preserves_parameter_order() {
        let expression = LambdaExpression::try_new(
            id(1),
            span(),
            vec![id(9), id(4), id(7)],
            None,
            id(10),
        )
        .expect("valid lambda");

        assert_eq!(
            expression.child_node_ids(),
            vec![id(9), id(4), id(7), id(10)]
        );
    }

    #[test]
    fn preserves_explicit_return_type_order() {
        let expression = LambdaExpression::try_new(
            id(1),
            span(),
            vec![id(2), id(3)],
            Some(id(4)),
            id(5),
        )
        .expect("valid lambda");

        assert_eq!(
            expression.child_node_ids(),
            vec![id(2), id(3), id(4), id(5)]
        );
    }

    #[test]
    fn counts_children_correctly() {
        let without_return_type = LambdaExpression::try_new(
            id(1),
            span(),
            vec![id(2), id(3)],
            None,
            id(4),
        )
        .expect("valid lambda");

        assert_eq!(without_return_type.child_count(), 3);

        let with_return_type = LambdaExpression::try_new(
            id(5),
            span(),
            vec![id(6), id(7)],
            Some(id(8)),
            id(9),
        )
        .expect("valid lambda");

        assert_eq!(with_return_type.child_count(), 4);
    }

    #[test]
    fn rejects_invalid_body() {
        let result =
            LambdaExpression::try_new(id(1), span(), Vec::new(), None, NodeId::INVALID);

        assert_eq!(
            result,
            Err(LambdaExpressionError::InvalidBodyReference {
                id: NodeId::INVALID
            })
        );
    }

    #[test]
    fn rejects_invalid_parameter() {
        let result =
            LambdaExpression::try_new(id(1), span(), vec![NodeId::INVALID], None, id(2));

        assert_eq!(
            result,
            Err(LambdaExpressionError::InvalidParameterReference {
                index: 0,
                id: NodeId::INVALID
            })
        );
    }

    #[test]
    fn rejects_duplicate_parameters() {
        let result =
            LambdaExpression::try_new(id(1), span(), vec![id(2), id(2)], None, id(3));

        assert_eq!(
            result,
            Err(LambdaExpressionError::DuplicateParameter {
                index: 1,
                id: id(2)
            })
        );
    }

    #[test]
    fn rejects_invalid_return_type_reference() {
        let result = LambdaExpression::try_new(
            id(1),
            span(),
            Vec::new(),
            Some(NodeId::INVALID),
            id(2),
        );

        assert_eq!(
            result,
            Err(LambdaExpressionError::InvalidReturnTypeReference {
                id: NodeId::INVALID
            })
        );
    }

    #[test]
    fn rejects_invalid_node_id() {
        let result =
            LambdaExpression::try_new(NodeId::INVALID, span(), Vec::new(), None, id(2));

        assert_eq!(
            result,
            Err(LambdaExpressionError::InvalidNodeId)
        );
    }

    #[test]
    fn validates_unlimited_policy() {
        let expression = LambdaExpression::try_new(
            id(1),
            span(),
            vec![id(2), id(3), id(4)],
            None,
            id(5),
        )
        .expect("valid lambda");

        assert_eq!(
            expression.validate_structure_with_policy(
                LambdaValidationPolicy::unlimited()
            ),
            Ok(())
        );
    }

    #[test]
    fn validates_caller_supplied_parameter_limit() {
        let expression = LambdaExpression::try_new(
            id(1),
            span(),
            vec![id(2), id(3), id(4)],
            None,
            id(5),
        )
        .expect("valid lambda");

        assert_eq!(
            expression.validate_structure_with_policy(
                LambdaValidationPolicy::with_max_parameters(2)
            ),
            Err(LambdaExpressionError::LimitExceeded {
                limit: "lambda_parameters",
                actual: 3,
                maximum: 2
            })
        );
    }

    #[test]
    fn metadata_is_preserved() {
        let mut metadata = NodeMetadata::new();

        metadata.insert(
            "test",
            crate::frontend::ast::node::metadata::MetadataValue::Boolean(true),
        );

        let expression = LambdaExpression::try_new_with_metadata(
            id(1),
            span(),
            metadata.clone(),
            Vec::new(),
            None,
            id(2),
        )
        .expect("valid lambda");

        assert_eq!(expression.metadata(), Some(&metadata));
    }

    #[test]
    fn setter_preserves_valid_structure() {
        let mut expression =
            LambdaExpression::try_new(id(1), span(), Vec::new(), None, id(2))
                .expect("valid lambda");

        expression
            .set_parameters(vec![id(3), id(4)])
            .expect("valid parameters");

        expression
            .set_return_type(Some(id(5)))
            .expect("valid return type");

        expression.set_body(id(6)).expect("valid body");

        assert_eq!(expression.parameters(), &[id(3), id(4)]);
        assert_eq!(expression.return_type(), Some(id(5)));
        assert_eq!(expression.body(), id(6));
    }

    #[test]
    fn push_parameter_preserves_order() {
        let mut expression =
            LambdaExpression::try_new(id(1), span(), Vec::new(), None, id(2))
                .expect("valid lambda");

        expression.push_parameter(id(3)).expect("valid parameter");
        expression.push_parameter(id(4)).expect("valid parameter");

        assert_eq!(expression.parameters(), &[id(3), id(4)]);
    }

    #[test]
    fn push_parameter_rejects_duplicate() {
        let mut expression =
            LambdaExpression::try_new(id(1), span(), vec![id(3)], None, id(2))
                .expect("valid lambda");

        assert_eq!(
            expression.push_parameter(id(3)),
            Err(LambdaExpressionError::DuplicateParameter {
                index: 1,
                id: id(3)
            })
        );
    }

    #[test]
    fn source_summary_is_deterministic() {
        let expression = LambdaExpression::try_new(
            id(1),
            span(),
            vec![id(2), id(3)],
            Some(id(4)),
            id(5),
        )
        .expect("valid lambda");

        assert_eq!(
            expression.source_summary(),
            LambdaSourceSummary {
                parameter_count: 2,
                has_explicit_return_type: true,
                body: id(5)
            }
        );
    }

    #[test]
    fn serde_round_trip_preserves_lambda() {
        let expression = LambdaExpression::try_new(
            id(1),
            span(),
            vec![id(2), id(3)],
            Some(id(4)),
            id(5),
        )
        .expect("valid lambda");

        let encoded =
            serde_json::to_string(&expression).expect("serialize lambda");

        let decoded: LambdaExpression =
            serde_json::from_str(&encoded).expect("deserialize lambda");

        assert_eq!(expression, decoded);
    }

    #[test]
    fn supports_large_parameter_vectors_without_fixed_ast_capacity() {
        let parameters: Vec<NodeId> =
            (1..=10_000).map(NodeId::new).collect();

        let expression = LambdaExpression::try_new(
            id(20_000),
            span(),
            parameters.clone(),
            None,
            id(20_001),
        )
        .expect("large lambda");

        assert_eq!(expression.parameter_count(), 10_000);
        assert_eq!(expression.parameters(), parameters.as_slice());
    }
}