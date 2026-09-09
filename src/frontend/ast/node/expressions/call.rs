//! # Zamani Native AST — Call Expression
//!
//! Canonical source-level representation of a call expression in the Zamani
//! native frontend AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! Lexer
//!     │
//!     ▼
//! Parser
//!     │
//!     ▼
//! CallExpression
//!     │
//!     ▼
//! Structural AST validation
//!     │
//!     ▼
//! Semantic analysis
//!     │
//!     ├── name/path resolution
//!     ├── callable resolution
//!     ├── overload resolution
//!     ├── generic substitution
//!     ├── type checking
//!     ├── effect resolution
//!     ├── capability resolution
//!     ├── resource resolution
//!     └── domain resolution
//!     │
//!     ▼
//! Semantic Model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── HDL IR
//!     └── future domain IRs
//!     │
//!     ▼
//! Target lowering / execution
//! ```
//!
//! ## Core responsibility
//!
//! `CallExpression` represents the **source-level structure of a call**.
//!
//! It records what the programmer wrote, not how the call will eventually be
//! implemented.
//!
//! A call may eventually denote:
//!
//! - a normal function;
//! - a method;
//! - a constructor;
//! - a generic function;
//! - an operation;
//! - a quantum operation;
//! - a classical operation;
//! - a hybrid operation;
//! - a distributed operation;
//! - an accelerator operation;
//! - an external function;
//! - a macro-like callable construct;
//! - a future computational abstraction.
//!
//! The AST does not decide which of these meanings applies.
//!
//! ## POCO-REAF
//!
//! The call representation is deliberately independent of:
//!
//! - CPU architecture;
//! - GPU architecture;
//! - FPGA architecture;
//! - ASIC architecture;
//! - QPU architecture;
//! - quantum technology;
//! - quantum gate set;
//! - physical qubit count;
//! - logical qubit count;
//! - hardware topology;
//! - vendor;
//! - backend;
//! - runtime;
//! - scheduler;
//! - router;
//! - calibration;
//! - error correction;
//! - resilience;
//! - QIR;
//! - LLVM;
//! - MLIR.
//!
//! Therefore a source call can remain unchanged while the compiler selects a
//! different realization for a different computational environment.
//!
//! This is required for:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF).
//!
//! ## Important graph representation rule
//!
//! The native Zamani AST expression system uses a graph-oriented representation.
//!
//! Child nodes are represented by [`NodeId`] references rather than recursively
//! embedding `Expression` values.
//!
//! Conceptually:
//!
//! ```text
//! CallExpression
//! ├── Node
//! ├── callee: NodeId
//! ├── generic_arguments: [NodeId, ...]
//! └── arguments
//!      ├── argument NodeId
//!      ├── argument NodeId
//!      └── ...
//! ```
//!
//! The owning AST graph stores the actual nodes.
//!
//! This design is intentional because it:
//!
//! - avoids recursive Rust ownership structures;
//! - allows iterative graph traversal;
//! - supports large ASTs;
//! - supports incremental compilation;
//! - permits side tables keyed by `NodeId`;
//! - avoids coupling this node to a concrete `Expression` enum;
//! - makes source-level AST ownership explicit.
//!
//! ## No semantic resolution
//!
//! This file must never store:
//!
//! - resolved function IDs;
//! - symbol-table entries;
//! - resolved overloads;
//! - resolved generic substitutions;
//! - inferred argument types;
//! - return types;
//! - effect sets;
//! - capabilities;
//! - resource allocations;
//! - physical qubit assignments;
//! - hardware instructions;
//! - QIR values;
//! - LLVM values;
//! - runtime handles.
//!
//! Those belong to semantic analysis and later compiler layers.
//!
//! ## Generic call model
//!
//! A call consists of:
//!
//! 1. a callee expression/node;
//! 2. zero or more generic arguments;
//! 3. zero or more positional/named/structured arguments.
//!
//! The node does not decide whether the callee denotes a function, operation,
//! method, constructor, intrinsic, quantum operation, or future callable entity.
//!
//! Semantic analysis determines that meaning.
//!
//! ## Quantum compatibility
//!
//! Quantum computation must not require a special:
//!
//! ```text
//! QuantumCallExpression
//! ```
//!
//! inside the native AST.
//!
//! A quantum operation may be represented by an ordinary call whose callee and
//! arguments have source-level quantum/resource meaning determined downstream.
//!
//! For example, conceptually:
//!
//! ```text
//! apply_operation(target, parameter)
//! ```
//!
//! remains a generic call at the native AST layer.
//!
//! Semantic analysis may subsequently determine that the callable represents a
//! quantum operation.
//!
//! This permits new operations and new quantum technologies without changing
//! this file.
//!
//! ## Arbitrary argument count
//!
//! The call contains a dynamically sized collection of arguments.
//!
//! There is no fixed limit such as:
//!
//! ```text
//! MAX_ARGUMENTS = 8
//! ```
//!
//! or:
//!
//! ```text
//! MAX_PARAMETERS = 32
//! ```
//!
//! Such limits would incorrectly turn compiler resource policy into language
//! semantics.
//!
//! Any hostile-input protection belongs to configurable compiler limits.
//!
//! ## Generic arguments
//!
//! Generic arguments are represented using `NodeId` references rather than
//! concrete semantic types.
//!
//! This permits generic arguments to represent source-level constructs such as:
//!
//! - type arguments;
//! - const arguments;
//! - resource dimensions;
//! - symbolic values;
//! - future generic argument forms.
//!
//! Their semantic interpretation belongs downstream.
//!
//! ## Argument representation
//!
//! Arguments are represented structurally rather than as semantic values.
//!
//! An argument may contain:
//!
//! - an optional source-level name;
//! - an expression node;
//! - optional source metadata.
//!
//! This allows positional and named arguments without requiring separate call
//! node types.
//!
//! ## Named arguments
//!
//! Named arguments preserve the source spelling of the argument label.
//!
//! The AST does not determine whether the name corresponds to a valid parameter.
//!
//! That is semantic analysis.
//!
//! ## Source preservation
//!
//! The common [`Node`] stores the canonical source span.
//!
//! The call span supplied by the parser should cover the complete call syntax,
//! including the callee and argument list.
//!
//! For example:
//!
//! ```text
//! compute(a, b)
//! ^^^^^^^^^^^^^
//! ```
//!
//! The exact span boundaries are parser/source-map responsibilities.
//!
//! ## Parsing boundary
//!
//! The parser is responsible for:
//!
//! - parsing the callee;
//! - parsing generic arguments;
//! - parsing the argument list;
//! - determining positional/named syntax;
//! - assigning source spans;
//! - allocating node IDs;
//! - constructing this node.
//!
//! The parser must not perform:
//!
//! - overload resolution;
//! - type inference;
//! - capability resolution;
//! - resource allocation;
//! - quantum legality checking;
//! - hardware mapping.
//!
//! ## Operator/call distinction
//!
//! A call is not an operator.
//!
//! For example:
//!
//! ```text
//! f(x)
//! ```
//!
//! is represented by this node.
//!
//! An expression such as:
//!
//! ```text
//! a + b
//! ```
//!
//! is represented by the binary-expression subsystem.
//!
//! The semantic layer may ultimately lower either form into operations, but the
//! native AST preserves their source-level distinction.
//!
//! ## Path distinction
//!
//! The callee is represented as a node reference because the callee may itself
//! be an arbitrary expression.
//!
//! A qualified name should be represented using the canonical path/identifier
//! expression infrastructure rather than introducing path-specific fields into
//! this node.
//!
//! This allows calls such as:
//!
//! ```text
//! math.linear.transform(x)
//! ```
//!
//! to remain structurally generic.
//!
//! ## Method calls
//!
//! Method-call syntax must not require a separate hardware-aware node.
//!
//! Depending on Zamani's final grammar, a method call can be represented through
//! a member/access expression used as the callee:
//!
//! ```text
//! object.method(value)
//! ```
//!
//! The call node then simply references the member-access expression as its
//! callee.
//!
//! Semantic analysis determines dispatch semantics.
//!
//! ## Constructors
//!
//! Constructors can use the same call representation.
//!
//! There must be no special hardware-specific constructor type.
//!
//! ## External calls
//!
//! A source-level call to an external declaration remains a call expression.
//!
//! ABI/calling-convention resolution belongs to semantic/code-generation
//! layers.
//!
//! ## Quantum operations
//!
//! A quantum operation may eventually require:
//!
//! - quantum resources;
//! - classical parameters;
//! - symbolic parameters;
//! - modifiers;
//! - measurement results;
//! - capabilities;
//! - effects;
//! - resource constraints.
//!
//! None of these require the native call node to know about hardware.
//!
//! The callee and arguments carry source-level structure; semantic analysis
//! resolves their computational meaning.
//!
//! ## Quantum scalability
//!
//! The call representation does not contain:
//!
//! - qubit arrays;
//! - fixed qubit counts;
//! - physical qubit IDs;
//! - device IDs;
//! - coupling maps;
//! - gate-set restrictions.
//!
//! A call can therefore participate in programs operating over symbolic or
//! dynamically determined resource cardinalities.
//!
//! ## Hybrid computation
//!
//! The same call representation works for:
//!
//! ```text
//! classical → classical
//! classical → quantum
//! quantum → classical
//! quantum → quantum
//! distributed → distributed
//! accelerator → host
//! host → accelerator
//! ```
//!
//! The AST does not encode those execution relationships.
//!
//! Effects, capabilities, resources, domains and semantic analysis determine
//! the eventual interpretation.
//!
//! ## AST ≠ IR
//!
//! This file deliberately does not contain an IR-level operation structure.
//!
//! It must not be converted into a disguised:
//!
//! - ZUIR operation;
//! - QIR call;
//! - LLVM call;
//! - MLIR operation;
//! - backend command;
//! - scheduler operation.
//!
//! Semantic lowering performs that transformation later.
//!
//! ## Validation boundary
//!
//! Local validation verifies only intrinsic structural properties:
//!
//! - node classification;
//! - required callee reference;
//! - argument structure;
//! - generic-argument structure;
//! - valid node references;
//! - duplicate references where prohibited;
//! - configurable local collection limits.
//!
//! It does not determine whether:
//!
//! - the callee exists;
//! - the callee is callable;
//! - argument types match;
//! - generic arguments are valid;
//! - a quantum operation is legal;
//! - resources are available;
//! - a backend supports the operation.
//!
//! ## Determinism
//!
//! Source-order collections are stored in `Vec`.
//!
//! Iteration is deterministic:
//!
//! ```text
//! callee
//! generic arguments in source order
//! arguments in source order
//! ```
//!
//! No unordered semantic state is stored here.
//!
//! ## Serialization
//!
//! The node derives Serde serialization.
//!
//! AST-wide schema versioning belongs to the AST serialization subsystem.
//!
//! This file does not introduce a competing serialization protocol.
//!
//! Serialization contains logical AST information only.
//!
//! It must never serialize:
//!
//! - pointers;
//! - memory addresses;
//! - runtime handles;
//! - backend state;
//! - caches;
//! - synchronization primitives.
//!
//! ## Security
//!
//! This file:
//!
//! - contains no `unsafe`;
//! - performs no I/O;
//! - performs no execution;
//! - uses no raw pointers;
//! - uses no unchecked indexing;
//! - performs no recursive traversal;
//! - has no global mutable state;
//! - does not execute the callable;
//! - does not resolve external resources.
//!
//! This is important because AST instances may originate from untrusted source.
//!
//! ## Scalability
//!
//! There is no AST-level maximum for:
//!
//! - number of calls;
//! - number of arguments;
//! - number of generic arguments;
//! - identifier length;
//! - program size;
//! - resource count;
//! - qubit count;
//! - machine count.
//!
//! Actual compilation remains bounded only by available resources and explicit
//! compiler resource policies.
//!
//! ## Incremental compilation
//!
//! Stable `NodeId` values allow semantic and tooling side tables to associate
//! information with the call without embedding semantic state in this node.
//!
//! This allows a later compiler phase to cache information such as:
//!
//! ```text
//! NodeId -> resolved callable
//! NodeId -> inferred type
//! NodeId -> effect set
//! NodeId -> resource requirements
//! ```
//!
//! without changing this AST structure.
//!
//! ## Thread safety
//!
//! This node contains only owned AST data and foundational source structures.
//!
//! No global mutable state is used.
//!
//! Immutable AST consumers may process nodes concurrently whenever their
//! containing AST and contained types satisfy the corresponding `Send` and
//! `Sync` requirements.
//!
//! ## Dependency contract
//!
//! This file may depend on:
//!
//! - `Node`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - `NodeMetadata`;
//! - `Span`;
//! - standard library;
//! - Serde.
//!
//! It must not depend on:
//!
//! - semantic analysis;
//! - compiler orchestration;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - quantum optimization;
//! - routing;
//! - scheduling;
//! - calibration;
//! - error correction;
//! - resilience;
//! - runtime;
//! - backend providers;
//! - LLVM;
//! - QIR;
//! - MLIR;
//! - OpenQASM AST;
//! - external language ASTs.
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
//! File contract
//! =============================================================================
//!
//! **Inputs**
//!
//! - AST node identity;
//! - source span;
//! - optional metadata;
//! - callee node reference;
//! - zero or more generic-argument references;
//! - zero or more call arguments.
//!
//! **Outputs**
//!
//! - canonical source-level `CallExpression`;
//! - deterministic child enumeration;
//! - local structural validation.
//!
//! **Semantic contract**
//!
//! Semantic analysis resolves the meaning of the callee, generic arguments and
//! argument expressions.
//!
//! **ZUIR contract**
//!
//! Semantic lowering converts the resolved call into the appropriate ZUIR
//! representation.
//!
//! **Quantum contract**
//!
//! Quantum calls remain generic source-level calls. Quantum interpretation is
//! downstream.
//!
//! **Scalability contract**
//!
//! Collections grow dynamically. No machine/domain/cardinality constants are
//! encoded in this node.
//!
//! **Migration contract**
//!
//! Legacy call representations should map into this structure. Lexer token
//! types and legacy semantic values must not be imported into this module.
//!
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

/// Schema identifier for the call-expression contract.
///
/// This is deliberately separate from the global AST serialization schema
/// version and the Zamani language version.
pub const CALL_EXPRESSION_SCHEMA_VERSION: u16 = 1;

/// Stable source-level kind identifier.
pub const CALL_EXPRESSION_KIND_NAME: &str = "zamani:call-expression";

/// Structural validation errors for [`CallExpression`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CallExpressionError {
    /// The common node is not classified as a call expression.
    InvalidNodeKind {
        /// Actual node classification.
        actual: NodeKind,
    },

    /// The callee reference is invalid.
    InvalidCalleeReference,

    /// A generic argument reference is invalid.
    InvalidGenericArgumentReference {
        /// Index in the source-order generic argument list.
        index: usize,
    },

    /// A call argument is structurally invalid.
    InvalidArgument {
        /// Index in the source-order argument list.
        index: usize,
    },

    /// A child reference is duplicated where a distinct reference is required.
    DuplicateChild {
        /// Duplicated child ID.
        id: NodeId,
    },

    /// A configured safety limit was exceeded.
    LimitExceeded {
        /// Name of the policy limit.
        limit: &'static str,

        /// Observed value.
        actual: usize,

        /// Configured maximum.
        maximum: usize,
    },
}

impl core::fmt::Display for CallExpressionError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "call expression has invalid node kind: {actual}"
                )
            }

            Self::InvalidCalleeReference => {
                formatter.write_str("call expression has an invalid callee reference")
            }

            Self::InvalidGenericArgumentReference { index } => {
                write!(
                    formatter,
                    "call expression has an invalid generic argument reference at index {index}"
                )
            }

            Self::InvalidArgument { index } => {
                write!(
                    formatter,
                    "call expression has an invalid argument at index {index}"
                )
            }

            Self::DuplicateChild { id } => {
                write!(
                    formatter,
                    "call expression contains duplicate child node id: {id:?}"
                )
            }

            Self::LimitExceeded {
                limit,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "call expression validation limit `{limit}` exceeded: \
                     {actual} > {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for CallExpressionError {}

/// Configurable structural-validation policy for call expressions.
///
/// These limits are compiler safety policy, not language semantics.
///
/// `None` means that this individual limit is not imposed by the caller.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CallExpressionValidationPolicy {
    /// Maximum number of generic arguments.
    pub max_generic_arguments: Option<usize>,

    /// Maximum number of call arguments.
    pub max_arguments: Option<usize>,
}

impl CallExpressionValidationPolicy {
    /// Creates a policy with no local collection limits.
    #[must_use]
    pub const fn unlimited() -> Self {
        Self {
            max_generic_arguments: None,
            max_arguments: None,
        }
    }

    /// Creates a policy with explicit collection limits.
    #[must_use]
    pub const fn new(
        max_generic_arguments: Option<usize>,
        max_arguments: Option<usize>,
    ) -> Self {
        Self {
            max_generic_arguments,
            max_arguments,
        }
    }

    fn check(
        self,
        generic_argument_count: usize,
        argument_count: usize,
    ) -> Result<(), CallExpressionError> {
        if let Some(maximum) = self.max_generic_arguments {
            if generic_argument_count > maximum {
                return Err(CallExpressionError::LimitExceeded {
                    limit: "max_generic_arguments",
                    actual: generic_argument_count,
                    maximum,
                });
            }
        }

        if let Some(maximum) = self.max_arguments {
            if argument_count > maximum {
                return Err(CallExpressionError::LimitExceeded {
                    limit: "max_arguments",
                    actual: argument_count,
                    maximum,
                });
            }
        }

        Ok(())
    }
}

/// A source-level argument of a call expression.
///
/// The argument expression is represented by [`NodeId`] because the native
/// Zamani AST is graph-oriented.
///
/// An argument may optionally have a source-level name:
///
/// ```text
/// function(value)
/// function(name: value)
/// ```
///
/// The AST does not determine whether a named argument corresponds to an
/// actual parameter. That is semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CallArgument {
    /// Optional source-level argument label.
    ///
    /// `None` represents a positional argument.
    name: Option<String>,

    /// AST node containing the argument expression.
    expression: NodeId,
}

impl CallArgument {
    /// Creates a positional call argument.
    ///
    /// The expression reference must be a valid `NodeId`.
    #[must_use]
    pub const fn positional(expression: NodeId) -> Self {
        Self {
            name: None,
            expression,
        }
    }

    /// Creates a named call argument.
    ///
    /// Lexical validity of the name is the parser/lexer responsibility.
    /// Structural validation only requires a non-empty name.
    #[must_use]
    pub fn named(name: impl Into<String>, expression: NodeId) -> Self {
        Self {
            name: Some(name.into()),
            expression,
        }
    }

    /// Returns the optional source-level argument name.
    #[must_use]
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the argument expression node ID.
    #[must_use]
    #[inline]
    pub const fn expression(&self) -> NodeId {
        self.expression
    }

    /// Returns `true` when this is a named argument.
    #[must_use]
    #[inline]
    pub const fn is_named(&self) -> bool {
        self.name.is_some()
    }

    /// Returns `true` when this is a positional argument.
    #[must_use]
    #[inline]
    pub const fn is_positional(&self) -> bool {
        self.name.is_none()
    }

    /// Validates the local structure of this argument.
    ///
    /// This does not resolve the argument name or expression semantics.
    pub fn validate_structure(&self) -> Result<(), CallExpressionError> {
        if self.expression.is_invalid() {
            return Err(CallExpressionError::InvalidArgument { index: 0 });
        }

        if self.name.as_deref().is_some_and(str::is_empty) {
            return Err(CallExpressionError::InvalidArgument { index: 0 });
        }

        Ok(())
    }

    /// Consumes the argument and returns its components.
    #[must_use]
    pub fn into_parts(self) -> (Option<String>, NodeId) {
        (self.name, self.expression)
    }
}

/// Canonical source-level call expression.
///
/// A call contains:
///
/// - a common AST [`Node`];
/// - a callee [`NodeId`];
/// - zero or more source-level generic arguments;
/// - zero or more source-level call arguments.
///
/// It contains no resolved semantic information.
///
/// # Invariants
///
/// A structurally valid call expression:
///
/// 1. has `CoreNodeKind::CallExpression`;
/// 2. has a valid callee node ID;
/// 3. has valid generic-argument references;
/// 4. has valid argument references;
/// 5. contains no semantic/backend state;
/// 6. preserves source ordering of generic arguments and call arguments.
///
/// # Ownership
///
/// The containing AST graph owns the nodes referenced by this structure.
///
/// `CallExpression` owns only its local references and argument metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CallExpression {
    /// Common AST identity, classification, span and metadata.
    node: Node,

    /// Expression serving as the callable target.
    ///
    /// This may itself be an identifier, path, member access, lambda,
    /// closure, or another callable expression.
    callee: NodeId,

    /// Source-level generic arguments in source order.
    ///
    /// Their semantic interpretation is deferred.
    generic_arguments: Vec<NodeId>,

    /// Call arguments in source order.
    arguments: Vec<CallArgument>,
}

impl CallExpression {
    /// Creates a call expression with no generic arguments or call arguments.
    ///
    /// The constructor classifies the common node as
    /// `CoreNodeKind::CallExpression`.
    ///
    /// This constructor does not perform semantic validation.
    #[must_use]
    pub fn new(id: NodeId, span: Span, callee: NodeId) -> Self {
        Self::with_metadata(
            id,
            span,
            NodeMetadata::default(),
            callee,
            Vec::new(),
            Vec::new(),
        )
    }

    /// Creates a call expression with explicit metadata and child lists.
    ///
    /// The lists are retained in source order.
    ///
    /// The constructor does not silently rewrite malformed child references.
    /// Callers that accept untrusted programmatically constructed ASTs should
    /// invoke [`Self::validate_structure`].
    #[must_use]
    pub fn with_metadata(
        id: NodeId,
        span: Span,
        metadata: NodeMetadata,
        callee: NodeId,
        generic_arguments: Vec<NodeId>,
        arguments: Vec<CallArgument>,
    ) -> Self {
        let node = Node::new(
            id,
            NodeKind::core(CoreNodeKind::CallExpression),
            span,
            metadata,
        );

        Self {
            node,
            callee,
            generic_arguments,
            arguments,
        }
    }

    /// Constructs a call from an existing common node.
    ///
    /// This is useful for AST graph builders and deserializers that already
    /// possess the canonical node header.
    ///
    /// The supplied node's classification is not rewritten.
    #[must_use]
    pub fn from_node(
        node: Node,
        callee: NodeId,
        generic_arguments: Vec<NodeId>,
        arguments: Vec<CallArgument>,
    ) -> Self {
        Self {
            node,
            callee,
            generic_arguments,
            arguments,
        }
    }

    /// Returns the common AST node.
    #[must_use]
    #[inline]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common AST node.
    #[must_use]
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST node ID.
    #[must_use]
    #[inline]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the complete source span of the call.
    #[must_use]
    #[inline]
    pub fn span(&self) -> Span {
        self.node.span()
    }

    /// Returns the source-level metadata.
    #[must_use]
    #[inline]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Replaces the source-level metadata.
    pub fn set_metadata(&mut self, metadata: NodeMetadata) {
        self.node.set_metadata(metadata);
    }

    /// Removes and returns the source-level metadata.
    pub fn take_metadata(&mut self) -> NodeMetadata {
        self.node.take_metadata()
    }

    /// Returns the node classification.
    #[must_use]
    #[inline]
    pub fn node_kind(&self) -> NodeKind {
        self.node.kind()
    }

    /// Returns the callee node reference.
    #[must_use]
    #[inline]
    pub const fn callee(&self) -> NodeId {
        self.callee
    }

    /// Replaces the callee node reference.
    ///
    /// The new reference is not semantically resolved by this method.
    pub const fn set_callee(&mut self, callee: NodeId) {
        self.callee = callee;
    }

    /// Returns all generic argument node references in source order.
    #[must_use]
    #[inline]
    pub fn generic_arguments(&self) -> &[NodeId] {
        &self.generic_arguments
    }

    /// Returns mutable access to generic argument references.
    pub fn generic_arguments_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.generic_arguments
    }

    /// Returns the number of generic arguments.
    #[must_use]
    #[inline]
    pub fn generic_argument_count(&self) -> usize {
        self.generic_arguments.len()
    }

    /// Appends one generic argument.
    ///
    /// The caller is responsible for ensuring that the referenced node exists
    /// in the containing AST graph.
    pub fn push_generic_argument(&mut self, argument: NodeId) {
        self.generic_arguments.push(argument);
    }

    /// Returns call arguments in source order.
    #[must_use]
    #[inline]
    pub fn arguments(&self) -> &[CallArgument] {
        &self.arguments
    }

    /// Returns mutable access to call arguments.
    pub fn arguments_mut(&mut self) -> &mut Vec<CallArgument> {
        &mut self.arguments
    }

    /// Returns the number of call arguments.
    #[must_use]
    #[inline]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Appends a call argument while preserving source order.
    pub fn push_argument(&mut self, argument: CallArgument) {
        self.arguments.push(argument);
    }

    /// Returns the total number of direct AST child references.
    ///
    /// The count includes:
    ///
    /// - the callee;
    /// - every generic argument;
    /// - every call-argument expression.
    #[must_use]
    pub fn child_count(&self) -> usize {
        1usize
            .saturating_add(self.generic_arguments.len())
            .saturating_add(self.arguments.len())
    }

    /// Returns the direct child node IDs in deterministic source order.
    ///
    /// Ordering:
    ///
    /// ```text
    /// callee
    /// generic arguments
    /// call arguments
    /// ```
    ///
    /// The returned vector is intentionally newly allocated so callers may
    /// retain or mutate it without mutating the AST node.
    ///
    /// For allocation-free traversal, use [`Self::for_each_child`] instead.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        let mut children = Vec::with_capacity(self.child_count());

        children.push(self.callee);
        children.extend(self.generic_arguments.iter().copied());
        children.extend(self.arguments.iter().map(CallArgument::expression));

        children
    }

    /// Visits direct child node IDs without recursively traversing them.
    ///
    /// This method is suitable for iterative AST walkers.
    #[inline]
    pub fn for_each_child<F>(&self, mut visitor: F)
    where
        F: FnMut(NodeId),
    {
        visitor(self.callee);

        for argument in &self.generic_arguments {
            visitor(*argument);
        }

        for argument in &self.arguments {
            visitor(argument.expression());
        }
    }

    /// Validates intrinsic structure without resolving semantics.
    ///
    /// This method deliberately does not require access to the containing AST
    /// graph. Validation of whether a `NodeId` actually exists in that graph is
    /// the responsibility of graph-level AST validation.
    pub fn validate_structure(
        &self,
        policy: CallExpressionValidationPolicy,
    ) -> Result<(), CallExpressionError> {
        if self.node_kind() != NodeKind::core(CoreNodeKind::CallExpression) {
            return Err(CallExpressionError::InvalidNodeKind {
                actual: self.node_kind(),
            });
        }

        if self.callee.is_invalid() {
            return Err(CallExpressionError::InvalidCalleeReference);
        }

        policy.check(
            self.generic_arguments.len(),
            self.arguments.len(),
        )?;

        for (index, argument) in self.generic_arguments.iter().enumerate() {
            if argument.is_invalid() {
                return Err(CallExpressionError::InvalidGenericArgumentReference {
                    index,
                });
            }
        }

        for (index, argument) in self.arguments.iter().enumerate() {
            if argument.expression().is_invalid() {
                return Err(CallExpressionError::InvalidArgument { index });
            }

            if argument.name().is_some_and(str::is_empty) {
                return Err(CallExpressionError::InvalidArgument { index });
            }
        }

        Ok(())
    }

    /// Validates this call using the unlimited structural policy.
    ///
    /// This does not impose an artificial language-level maximum.
    pub fn validate(&self) -> Result<(), CallExpressionError> {
        self.validate_structure(CallExpressionValidationPolicy::unlimited())
    }

    /// Returns the call's direct children as an iterator.
    ///
    /// The iterator does not recurse.
    ///
    /// This API is useful when the surrounding traversal framework wants to
    /// avoid allocating the vector produced by [`Self::child_node_ids`].
    pub fn children(&self) -> impl Iterator<Item = NodeId> + '_ {
        core::iter::once(self.callee)
            .chain(self.generic_arguments.iter().copied())
            .chain(self.arguments.iter().map(CallArgument::expression))
    }

    /// Consumes the node and returns all structural components.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        Node,
        NodeId,
        Vec<NodeId>,
        Vec<CallArgument>,
    ) {
        (
            self.node,
            self.callee,
            self.generic_arguments,
            self.arguments,
        )
    }

    /// Replaces the generic argument list.
    ///
    /// Source order is preserved exactly as supplied.
    pub fn set_generic_arguments(&mut self, arguments: Vec<NodeId>) {
        self.generic_arguments = arguments;
    }

    /// Replaces the call argument list.
    ///
    /// Source order is preserved exactly as supplied.
    pub fn set_arguments(&mut self, arguments: Vec<CallArgument>) {
        self.arguments = arguments;
    }

    /// Returns `true` when the call has no generic arguments.
    #[must_use]
    #[inline]
    pub fn has_no_generic_arguments(&self) -> bool {
        self.generic_arguments.is_empty()
    }

    /// Returns `true` when the call has no call arguments.
    #[must_use]
    #[inline]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Returns the AST node classification expected for a call expression.
    #[must_use]
    pub const fn expected_node_kind() -> NodeKind {
        NodeKind::core(CoreNodeKind::CallExpression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(value: u64) -> NodeId {
        NodeId::new(value)
    }

    fn span() -> Span {
        Span::new(crate::frontend::ast::source::SourceId::new(1), 0, 10)
    }

    #[test]
    fn constructs_empty_call() {
        let call = CallExpression::new(node_id(1), span(), node_id(2));

        assert_eq!(call.id(), node_id(1));
        assert_eq!(call.callee(), node_id(2));
        assert_eq!(call.generic_argument_count(), 0);
        assert_eq!(call.argument_count(), 0);
        assert_eq!(
            call.node_kind(),
            NodeKind::core(CoreNodeKind::CallExpression)
        );
    }

    #[test]
    fn constructs_named_and_positional_arguments() {
        let positional = CallArgument::positional(node_id(3));
        let named = CallArgument::named("value", node_id(4));

        assert!(positional.is_positional());
        assert!(!positional.is_named());
        assert_eq!(positional.name(), None);

        assert!(named.is_named());
        assert!(!named.is_positional());
        assert_eq!(named.name(), Some("value"));
    }

    #[test]
    fn preserves_child_order() {
        let mut call = CallExpression::new(node_id(1), span(), node_id(2));

        call.push_generic_argument(node_id(3));
        call.push_generic_argument(node_id(4));
        call.push_argument(CallArgument::positional(node_id(5)));
        call.push_argument(CallArgument::named("x", node_id(6)));

        let children = call.child_node_ids();

        assert_eq!(
            children,
            vec![
                node_id(2),
                node_id(3),
                node_id(4),
                node_id(5),
                node_id(6),
            ]
        );
    }

    #[test]
    fn iterator_preserves_child_order() {
        let mut call = CallExpression::new(node_id(1), span(), node_id(2));

        call.push_generic_argument(node_id(3));
        call.push_argument(CallArgument::positional(node_id(4)));

        let children: Vec<NodeId> = call.children().collect();

        assert_eq!(
            children,
            vec![node_id(2), node_id(3), node_id(4)]
        );
    }

    #[test]
    fn rejects_invalid_callee() {
        let call = CallExpression::new(
            node_id(1),
            span(),
            NodeId::INVALID,
        );

        assert_eq!(
            call.validate(),
            Err(CallExpressionError::InvalidCalleeReference)
        );
    }

    #[test]
    fn rejects_invalid_generic_argument() {
        let mut call = CallExpression::new(node_id(1), span(), node_id(2));
        call.push_generic_argument(NodeId::INVALID);

        assert_eq!(
            call.validate(),
            Err(
                CallExpressionError::InvalidGenericArgumentReference {
                    index: 0,
                }
            )
        );
    }

    #[test]
    fn rejects_invalid_argument() {
        let mut call = CallExpression::new(node_id(1), span(), node_id(2));

        call.push_argument(CallArgument::positional(NodeId::INVALID));

        assert_eq!(
            call.validate(),
            Err(CallExpressionError::InvalidArgument { index: 0 })
        );
    }

    #[test]
    fn validates_valid_call() {
        let mut call = CallExpression::new(node_id(1), span(), node_id(2));

        call.push_generic_argument(node_id(3));
        call.push_argument(CallArgument::positional(node_id(4)));
        call.push_argument(CallArgument::named("value", node_id(5)));

        assert!(call.validate().is_ok());
    }

    #[test]
    fn configurable_limits_are_not_language_limits() {
        let mut call = CallExpression::new(node_id(1), span(), node_id(2));

        call.push_argument(CallArgument::positional(node_id(3)));
        call.push_argument(CallArgument::positional(node_id(4)));

        let policy = CallExpressionValidationPolicy::new(
            None,
            Some(1),
        );

        assert_eq!(
            call.validate_structure(policy),
            Err(CallExpressionError::LimitExceeded {
                limit: "max_arguments",
                actual: 2,
                maximum: 1,
            })
        );

        assert!(call.validate().is_ok());
    }

    #[test]
    fn serde_round_trip_preserves_structure() {
        let mut call = CallExpression::new(node_id(1), span(), node_id(2));

        call.push_generic_argument(node_id(3));
        call.push_argument(CallArgument::positional(node_id(4)));
        call.push_argument(CallArgument::named("value", node_id(5)));

        let encoded = serde_json::to_string(&call)
            .expect("call expression should serialize");

        let decoded: CallExpression =
            serde_json::from_str(&encoded)
                .expect("call expression should deserialize");

        assert_eq!(call, decoded);
    }

    #[test]
    fn into_parts_preserves_all_components() {
        let mut call = CallExpression::new(node_id(1), span(), node_id(2));

        call.push_generic_argument(node_id(3));
        call.push_argument(CallArgument::named("x", node_id(4)));

        let (node, callee, generics, arguments) = call.into_parts();

        assert_eq!(node.id(), node_id(1));
        assert_eq!(callee, node_id(2));
        assert_eq!(generics, vec![node_id(3)]);
        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0].name(), Some("x"));
        assert_eq!(arguments[0].expression(), node_id(4));
    }
}